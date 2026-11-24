use proptest::prelude::*;
use std::cell::RefCell;
use std::ffi::{CStr, c_char, c_void};
use std::sync::Once;
use vulkan::Device;
use vulkan::vk::{self, VkResult};

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

#[derive(Default)]
struct Driver {
    calls: RefCell<Vec<(usize, usize)>>,
}

impl Driver {
    fn record(device: vk::Device, data: *mut c_void, data_size: usize) -> VkResult {
        let driver = unsafe { &*device.0.cast::<Driver>() };
        driver.calls.borrow_mut().push((data as usize, data_size));
        VkResult::SUCCESS
    }
}

type GetQueryPoolResults = unsafe extern "system" fn(
    vk::Device,
    vk::QueryPool,
    u32,
    u32,
    usize,
    *mut c_void,
    vk::DeviceSize,
    vk::QueryResultFlags,
) -> VkResult;

#[allow(clippy::too_many_arguments)]
unsafe extern "system" fn get_query_pool_results(
    device: vk::Device,
    _query_pool: vk::QueryPool,
    _first_query: u32,
    _query_count: u32,
    data_size: usize,
    data: *mut c_void,
    _stride: vk::DeviceSize,
    _flags: vk::QueryResultFlags,
) -> VkResult {
    Driver::record(device, data, data_size)
}

type WriteProperties<T> = unsafe extern "system" fn(
    vk::Device,
    u32,
    *const T,
    vk::QueryType,
    usize,
    *mut c_void,
    usize,
) -> VkResult;

unsafe extern "system" fn write_properties<T>(
    device: vk::Device,
    _count: u32,
    _objects: *const T,
    _query_type: vk::QueryType,
    data_size: usize,
    data: *mut c_void,
    _stride: usize,
) -> VkResult {
    Driver::record(device, data, data_size)
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetQueryPoolResults" => Some(unsafe {
            std::mem::transmute::<GetQueryPoolResults, unsafe extern "system" fn()>(
                get_query_pool_results,
            )
        }),
        b"vkWriteAccelerationStructuresPropertiesKHR" => Some(unsafe {
            std::mem::transmute::<
                WriteProperties<vk::AccelerationStructureKHR>,
                unsafe extern "system" fn(),
            >(write_properties::<vk::AccelerationStructureKHR>)
        }),
        b"vkWriteMicromapsPropertiesEXT" => Some(unsafe {
            std::mem::transmute::<WriteProperties<vk::MicromapEXT>, unsafe extern "system" fn()>(
                write_properties::<vk::MicromapEXT>,
            )
        }),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug)]
enum Call {
    QueryPoolResults(vk::QueryResultFlags),
    AccelerationStructuresProperties,
    MicromapsProperties,
}

impl Call {
    fn required(self) -> usize {
        match self {
            Call::QueryPoolResults(flags) if flags.contains(vk::QueryResultFlags::_64) => 8,
            Call::QueryPoolResults(_) => 4,
            _ => std::mem::align_of::<vk::DeviceSize>(),
        }
    }

    fn message(self) -> &'static str {
        match self {
            Call::QueryPoolResults(_) => {
                "vkGetQueryPoolResults: `data` has to start at an address aligned to 4 bytes, or to 8 bytes if `flags` contains `VK_QUERY_RESULT_64_BIT`"
            }
            Call::AccelerationStructuresProperties => {
                "vkWriteAccelerationStructuresPropertiesKHR: `data` has to start at an address aligned to the alignment of `DeviceSize`"
            }
            Call::MicromapsProperties => {
                "vkWriteMicromapsPropertiesEXT: `data` has to start at an address aligned to the alignment of `DeviceSize`"
            }
        }
    }

    fn run(self, device: &Device, data: &mut [u8]) -> vk::Result<VkResult> {
        unsafe {
            match self {
                Call::QueryPoolResults(flags) => {
                    device.get_query_pool_results(vk::QueryPool::default(), 0, 1, data, 8, flags)
                }
                Call::AccelerationStructuresProperties => device
                    .write_acceleration_structures_properties_khr(
                        &[],
                        vk::QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR,
                        data,
                        8,
                    )
                    .map(|()| VkResult::SUCCESS),
                Call::MicromapsProperties => device
                    .write_micromaps_properties_ext(
                        &[],
                        vk::QueryType::MICROMAP_COMPACTED_SIZE_EXT,
                        data,
                        8,
                    )
                    .map(|()| VkResult::SUCCESS),
            }
        }
    }
}

fn flags() -> impl Strategy<Value = vk::QueryResultFlags> {
    (any::<bool>(), any::<bool>(), any::<bool>(), any::<bool>()).prop_map(
        |(wide, wait, availability, partial)| {
            let mut flags = vk::QueryResultFlags::empty();
            for (set, bit) in [
                (wide, vk::QueryResultFlags::_64),
                (wait, vk::QueryResultFlags::WAIT),
                (availability, vk::QueryResultFlags::WITH_AVAILABILITY),
                (partial, vk::QueryResultFlags::PARTIAL),
            ] {
                if set {
                    flags |= bit;
                }
            }
            flags
        },
    )
}

fn call() -> impl Strategy<Value = Call> {
    prop_oneof![
        flags().prop_map(Call::QueryPoolResults),
        Just(Call::AccelerationStructuresProperties),
        Just(Call::MicromapsProperties),
    ]
}

fn quiet_panics() {
    static QUIET: Once = Once::new();
    QUIET.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let message = info
                .payload()
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| info.payload().downcast_ref::<&str>().copied())
                .unwrap_or_default();
            if !message.contains("` has to start at an address aligned to ") {
                default(info);
            }
        }));
    });
}

proptest! {
    #![proptest_config(cases(512))]

    #[test]
    fn results_are_written_only_to_bytes_with_the_alignment_of_the_results(
        offset in 0..16usize,
        len in 0..64usize,
        call in call(),
    ) {
        quiet_panics();
        let driver = Driver::default();
        let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
        let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
        let device = Device::from_raw(handle, fp);
        let mut words = [0u64; 12];
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>().add(offset), len)
        };
        let address = bytes.as_ptr() as usize;
        let returned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            call.run(&device, bytes)
        }));
        if address.is_multiple_of(call.required()) {
            prop_assert_eq!(returned.ok(), Some(Ok(VkResult::SUCCESS)));
            prop_assert_eq!(driver.calls.into_inner(), vec![(address, len)]);
        } else {
            let message = returned.expect_err("the wrapper panics");
            let message = message
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| message.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            prop_assert!(message.starts_with(call.message()), "{}", message);
            prop_assert!(driver.calls.into_inner().is_empty());
        }
    }
}
