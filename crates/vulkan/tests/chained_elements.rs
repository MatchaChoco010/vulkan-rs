use proptest::prelude::*;
use std::cell::RefCell;
use std::ffi::{CStr, c_char, c_void};
use vulkan::Instance;
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
    available: u32,
    seen: RefCell<Vec<Seen>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Seen {
    capacity: u32,
    elements: usize,
    chains: Vec<usize>,
}

impl Driver {
    fn written(&self, capacity: u32) -> u32 {
        capacity.min(self.available)
    }
}

unsafe extern "system" fn queue_family_properties2(
    physical_device: vk::PhysicalDevice,
    count: *mut u32,
    properties: *mut vk::QueueFamilyProperties2<'_>,
) {
    let driver = unsafe { &*physical_device.0.cast::<Driver>() };
    let capacity = unsafe { count.read() };
    if properties.is_null() {
        unsafe { count.write(driver.available) };
        return;
    }
    let written = driver.written(capacity);
    let mut chains = Vec::new();
    for index in 0..written {
        let element = unsafe { &mut *properties.add(index as usize) };
        chains.push(element.p_next as usize);
        element.queue_family_properties.queue_count = index + 1;
        let video = element
            .p_next
            .cast::<vk::QueueFamilyVideoPropertiesKHR<'_>>();
        if !video.is_null() {
            unsafe {
                (*video).video_codec_operations = vk::VideoCodecOperationFlagsKHR::DECODE_H264;
            }
        }
    }
    driver.seen.borrow_mut().push(Seen {
        capacity,
        elements: properties as usize,
        chains,
    });
    unsafe { count.write(written) };
}

unsafe extern "system" fn surface_formats2(
    physical_device: vk::PhysicalDevice,
    _surface_info: *const vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    count: *mut u32,
    formats: *mut vk::SurfaceFormat2KHR<'_>,
) -> VkResult {
    let driver = unsafe { &*physical_device.0.cast::<Driver>() };
    let capacity = unsafe { count.read() };
    if formats.is_null() {
        unsafe { count.write(driver.available) };
        return VkResult::SUCCESS;
    }
    let written = driver.written(capacity);
    let mut chains = Vec::new();
    for index in 0..written {
        let element = unsafe { &mut *formats.add(index as usize) };
        chains.push(element.p_next as usize);
        element.surface_format.format = vk::Format::from_raw(index as i32 + 1);
    }
    driver.seen.borrow_mut().push(Seen {
        capacity,
        elements: formats as usize,
        chains,
    });
    unsafe { count.write(written) };
    if written < driver.available {
        VkResult::INCOMPLETE
    } else {
        VkResult::SUCCESS
    }
}

type QueueFamilyProperties2 =
    unsafe extern "system" fn(vk::PhysicalDevice, *mut u32, *mut vk::QueueFamilyProperties2<'_>);
type SurfaceFormats2 = unsafe extern "system" fn(
    vk::PhysicalDevice,
    *const vk::PhysicalDeviceSurfaceInfo2KHR<'_>,
    *mut u32,
    *mut vk::SurfaceFormat2KHR<'_>,
) -> VkResult;

unsafe extern "system" fn get_instance_proc_addr(
    _instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetPhysicalDeviceQueueFamilyProperties2" => Some(unsafe {
            std::mem::transmute::<QueueFamilyProperties2, unsafe extern "system" fn()>(
                queue_family_properties2,
            )
        }),
        b"vkGetPhysicalDeviceSurfaceFormats2KHR" => Some(unsafe {
            std::mem::transmute::<SurfaceFormats2, unsafe extern "system" fn()>(surface_formats2)
        }),
        _ => None,
    }
}

fn instance() -> Instance {
    let handle = vk::Instance(std::ptr::dangling_mut::<c_void>().cast());
    let fp = unsafe { vk::InstanceFn::load(Some(get_instance_proc_addr), handle) };
    Instance::from_raw(handle, fp)
}

fn physical_device(driver: &Driver) -> vk::PhysicalDevice {
    vk::PhysicalDevice(std::ptr::from_ref(driver).cast_mut().cast())
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn the_command_fills_the_elements_and_the_chains_that_the_caller_provides(
        available in 0..12u32,
        provided in 0..12usize,
    ) {
        let instance = instance();
        let driver = Driver {
            available,
            ..Driver::default()
        };
        let physical_device = physical_device(&driver);
        let mut video = vec![vk::QueueFamilyVideoPropertiesKHR::default(); provided];
        let chained = video
            .iter()
            .map(|video| std::ptr::from_ref(video) as usize)
            .collect::<Vec<_>>();
        let mut properties = video
            .iter_mut()
            .map(|video| vk::QueueFamilyProperties2::default().push_next(video))
            .collect::<Vec<_>>();
        let elements = properties.as_ptr() as usize;
        let written = unsafe {
            instance
                .get_physical_device_queue_family_properties2_into(physical_device, &mut properties)
        };
        let expected = provided.min(available as usize);
        prop_assert_eq!(written, expected);
        let seen = driver.seen.take();
        prop_assert_eq!(
            seen,
            vec![Seen {
                capacity: provided as u32,
                elements,
                chains: chained[..expected].to_vec(),
            }],
            "the command sees the caller's elements with their chains"
        );
        for (index, element) in properties.iter().enumerate() {
            let count = element.queue_family_properties.queue_count;
            prop_assert_eq!(count, if index < expected { index as u32 + 1 } else { 0 });
        }
        drop(properties);
        for (index, video) in video.iter().enumerate() {
            let operations = video.video_codec_operations;
            prop_assert_eq!(
                operations == vk::VideoCodecOperationFlagsKHR::DECODE_H264,
                index < expected,
                "the command wrote the chained structure of element {}",
                index
            );
        }

        let unchained = unsafe {
            instance.get_physical_device_queue_family_properties2(physical_device)
        };
        prop_assert_eq!(unchained.len(), available as usize);
        prop_assert!(unchained.iter().all(|element| element.p_next.is_null()));
    }

    #[test]
    fn a_command_with_a_result_reports_that_more_elements_exist(
        available in 0..12u32,
        provided in 0..12usize,
    ) {
        let instance = instance();
        let driver = Driver {
            available,
            ..Driver::default()
        };
        let physical_device = physical_device(&driver);
        let mut compression = vec![vk::ImageCompressionPropertiesEXT::default(); provided];
        let chained = compression
            .iter()
            .map(|properties| std::ptr::from_ref(properties) as usize)
            .collect::<Vec<_>>();
        let mut formats = compression
            .iter_mut()
            .map(|properties| vk::SurfaceFormat2KHR::default().push_next(properties))
            .collect::<Vec<_>>();
        let elements = formats.as_ptr() as usize;
        let info = vk::PhysicalDeviceSurfaceInfo2KHR::default();
        let returned = unsafe {
            instance.get_physical_device_surface_formats2_khr_into(
                physical_device,
                &info,
                &mut formats,
            )
        };
        let expected = provided.min(available as usize);
        let result = if expected < available as usize {
            VkResult::INCOMPLETE
        } else {
            VkResult::SUCCESS
        };
        prop_assert_eq!(returned, Ok((expected, result)));
        let seen = driver.seen.take();
        prop_assert_eq!(
            seen,
            vec![Seen {
                capacity: provided as u32,
                elements,
                chains: chained[..expected].to_vec(),
            }]
        );
        for (index, element) in formats.iter().enumerate() {
            let format = element.surface_format.format.as_raw();
            prop_assert_eq!(format, if index < expected { index as i32 + 1 } else { 0 });
        }
    }
}
