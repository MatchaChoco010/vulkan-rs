use proptest::prelude::*;
use std::cell::Cell;
use std::ffi::{CStr, c_char};
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

struct Driver {
    next_index: u32,
    refuses_requests: bool,
    seen: Cell<Option<(vk::Bool32, u32)>>,
}

unsafe extern "system" fn register_custom_border_color(
    device: vk::Device,
    _border_color: *const vk::SamplerCustomBorderColorCreateInfoEXT<'_>,
    request_index: vk::Bool32,
    index: *mut u32,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    driver
        .seen
        .set(Some((request_index, unsafe { index.read() })));
    if request_index == vk::TRUE {
        if driver.refuses_requests {
            return VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
        }
    } else {
        unsafe { index.write(driver.next_index) };
    }
    VkResult::SUCCESS
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkRegisterCustomBorderColorEXT" => Some(unsafe {
            std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    *const vk::SamplerCustomBorderColorCreateInfoEXT<'_>,
                    vk::Bool32,
                    *mut u32,
                ) -> VkResult,
                unsafe extern "system" fn(),
            >(register_custom_border_color)
        }),
        _ => None,
    }
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn the_command_receives_the_requested_index_and_the_wrapper_returns_the_registered_one(
        request in any::<bool>(),
        index in any::<u32>(),
        next_index in any::<u32>(),
        refuses_requests in any::<bool>(),
    ) {
        let driver = Driver {
            next_index,
            refuses_requests,
            seen: Cell::new(None),
        };
        let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
        let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
        let device = Device::from_raw(handle, fp);
        let border_color = vk::SamplerCustomBorderColorCreateInfoEXT::default();
        let registered =
            unsafe { device.register_custom_border_color_ext(&border_color, request, index) };
        prop_assert_eq!(
            driver.seen.get(),
            Some((vk::Bool32::from(request), index)),
            "the command reads the index that the caller passed"
        );
        let expected = match (request, refuses_requests) {
            (true, true) => Err(VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS),
            (true, false) => Ok(index),
            (false, _) => Ok(next_index),
        };
        prop_assert_eq!(registered, expected);
    }
}
