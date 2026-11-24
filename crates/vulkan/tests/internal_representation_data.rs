use proptest::collection::vec;
use proptest::prelude::*;
use std::cell::RefCell;
use std::ffi::{CStr, c_char};
use vulkan::Device;
use vulkan::vk::{self, VkResult};

const MOST_REPRESENTATIONS: usize = 6;
const MOST_BYTES: usize = 300;

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

#[derive(Clone, Debug)]
enum Change {
    Add(Vec<u8>),
    RemoveLast,
    RemoveFirst,
    Grow(usize, Vec<u8>),
    Shrink(usize, usize),
}

#[derive(Clone, Copy, Debug)]
enum Fill {
    AsReported,
    OnlyTheFirst(usize),
    ReportingMoreBy(usize),
}

#[derive(Default)]
struct Calls {
    filling: usize,
    elements_without_data: usize,
}

struct Driver {
    representations: RefCell<Vec<Vec<u8>>>,
    changes: RefCell<Vec<(usize, Change)>>,
    fill: Fill,
    calls: RefCell<Calls>,
}

fn name_of(index: usize) -> [c_char; vk::MAX_DESCRIPTION_SIZE] {
    let mut name = [0; vk::MAX_DESCRIPTION_SIZE];
    for (to, from) in name
        .iter_mut()
        .zip(format!("representation {index}").bytes())
    {
        *to = from as c_char;
    }
    name
}

impl Driver {
    fn apply_changes(&self, call: usize) {
        let mut changes = self.changes.borrow_mut();
        let mut representations = self.representations.borrow_mut();
        changes.retain(|(at, change)| {
            if *at != call {
                return true;
            }
            match change {
                Change::Add(bytes) => representations.push(bytes.clone()),
                Change::RemoveLast => {
                    representations.pop();
                }
                Change::RemoveFirst => {
                    if !representations.is_empty() {
                        representations.remove(0);
                    }
                }
                Change::Grow(index, more) => {
                    if let Some(len) = representations.len().checked_sub(1) {
                        representations[(*index).min(len)].extend_from_slice(more);
                    }
                }
                Change::Shrink(index, less) => {
                    if let Some(len) = representations.len().checked_sub(1) {
                        let bytes = &mut representations[(*index).min(len)];
                        bytes.truncate(bytes.len().saturating_sub(*less));
                    }
                }
            }
            false
        });
    }
}

type GetInternalRepresentations = unsafe extern "system" fn(
    vk::Device,
    *const vk::PipelineExecutableInfoKHR<'_>,
    *mut u32,
    *mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
) -> VkResult;

unsafe extern "system" fn get_internal_representations(
    device: vk::Device,
    _info: *const vk::PipelineExecutableInfoKHR<'_>,
    count: *mut u32,
    elements: *mut vk::PipelineExecutableInternalRepresentationKHR<'_>,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    let call = {
        let mut calls = driver.calls.borrow_mut();
        calls.filling += usize::from(!elements.is_null());
        calls.filling
    };
    driver.apply_changes(call);
    let representations = driver.representations.borrow();
    if elements.is_null() {
        unsafe { *count = representations.len() as u32 };
        return VkResult::SUCCESS;
    }
    let mut result = VkResult::SUCCESS;
    let asked = unsafe { *count } as usize;
    let written = asked.min(representations.len());
    if written < representations.len() {
        result = VkResult::INCOMPLETE;
    }
    for (index, bytes) in representations.iter().take(written).enumerate() {
        let element = unsafe { &mut *elements.add(index) };
        assert_eq!(
            element.s_type,
            vk::StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
            "the wrapper passes elements with their structure type"
        );
        element.name = name_of(index);
        element.is_text = vk::FALSE;
        if element.p_data.is_null() {
            driver.calls.borrow_mut().elements_without_data += 1;
            element.data_size = bytes.len();
            continue;
        }
        let room = element.data_size;
        if room < bytes.len() {
            result = VkResult::INCOMPLETE;
        }
        let copied = room.min(bytes.len());
        let (written, reported) = match driver.fill {
            Fill::AsReported => (copied, copied),
            Fill::OnlyTheFirst(len) => (len.min(copied), len.min(copied)),
            Fill::ReportingMoreBy(more) => (copied, copied + more),
        };
        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), element.p_data.cast::<u8>(), written)
        };
        element.data_size = reported;
    }
    unsafe { *count = written as u32 };
    result
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetPipelineExecutableInternalRepresentationsKHR" => Some(unsafe {
            std::mem::transmute::<GetInternalRepresentations, unsafe extern "system" fn()>(
                get_internal_representations,
            )
        }),
        _ => None,
    }
}

type Returned = (Vec<Representation>, Vec<Vec<u8>>, Calls);

type Representation = (String, Vec<u8>, bool);

fn returned(representations: Vec<Vec<u8>>, changes: Vec<(usize, Change)>, fill: Fill) -> Returned {
    let driver = Driver {
        representations: RefCell::new(representations),
        changes: RefCell::new(changes),
        fill,
        calls: RefCell::new(Calls::default()),
    };
    let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    let device = Device::from_raw(handle, fp);
    let info = vk::PipelineExecutableInfoKHR::default();
    let returned = unsafe { device.get_pipeline_executable_internal_representations_khr(&info) }
        .expect("the driver returns no error");
    let returned = returned
        .into_iter()
        .map(|(element, bytes)| {
            let name = unsafe { CStr::from_ptr(element.name.as_ptr()) };
            assert_eq!(
                element.data_size,
                match fill {
                    Fill::ReportingMoreBy(more) => bytes.len() + more,
                    _ => bytes.len(),
                },
                "the element keeps the size that the driver reported"
            );
            (
                name.to_string_lossy().into_owned(),
                bytes,
                element.p_data.is_null(),
            )
        })
        .collect();
    let representations = driver.representations.borrow().clone();
    (returned, representations, driver.calls.into_inner())
}

fn representations() -> impl Strategy<Value = Vec<Vec<u8>>> {
    vec(vec(any::<u8>(), 0..MOST_BYTES), 0..MOST_REPRESENTATIONS)
}

fn change() -> impl Strategy<Value = Change> {
    prop_oneof![
        vec(any::<u8>(), 0..MOST_BYTES).prop_map(Change::Add),
        Just(Change::RemoveLast),
        Just(Change::RemoveFirst),
        (0..MOST_REPRESENTATIONS, vec(any::<u8>(), 1..MOST_BYTES))
            .prop_map(|(index, more)| Change::Grow(index, more)),
        (0..MOST_REPRESENTATIONS, 1..MOST_BYTES)
            .prop_map(|(index, less)| Change::Shrink(index, less)),
    ]
}

fn expected(representations: &[Vec<u8>], fill: Fill) -> Vec<Representation> {
    representations
        .iter()
        .enumerate()
        .map(|(index, bytes)| {
            let mut bytes = bytes.clone();
            if let Fill::OnlyTheFirst(len) = fill {
                bytes.truncate(len);
            }
            (format!("representation {index}"), bytes, true)
        })
        .collect()
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn the_data_of_each_internal_representation_is_returned_with_it(
        representations in representations(),
    ) {
        let (returned, now, calls) = returned(representations.clone(), Vec::new(), Fill::AsReported);
        prop_assert_eq!(&now, &representations);
        prop_assert_eq!(returned, expected(&representations, Fill::AsReported));
        prop_assert_eq!(calls.filling, 2);
        prop_assert_eq!(calls.elements_without_data, representations.len());
    }

    #[test]
    fn representations_that_change_between_the_calls_are_asked_for_again(
        representations in representations(),
        changes in vec((1..=2usize, change()), 1..4),
    ) {
        let (returned, now, _) = returned(representations, changes, Fill::AsReported);
        prop_assert_eq!(returned, expected(&now, Fill::AsReported));
    }

    #[test]
    fn no_more_bytes_than_the_driver_writes_or_the_buffer_holds_are_returned(
        representations in representations(),
        fill in prop_oneof![
            (0..MOST_BYTES).prop_map(Fill::OnlyTheFirst),
            (1..MOST_BYTES).prop_map(Fill::ReportingMoreBy),
        ],
    ) {
        let (returned, now, _) = returned(representations, Vec::new(), fill);
        prop_assert_eq!(returned, expected(&now, fill));
    }
}
