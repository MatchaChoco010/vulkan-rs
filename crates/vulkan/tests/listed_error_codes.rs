use std::ffi::{CStr, c_char};
use vulkan::Device;
use vulkan::vk::{self, VkResult};

type AcquireProfilingLock =
    unsafe extern "system" fn(vk::Device, *const vk::AcquireProfilingLockInfoKHR<'_>) -> VkResult;

unsafe extern "system" fn acquire_profiling_lock(
    device: vk::Device,
    _info: *const vk::AcquireProfilingLockInfoKHR<'_>,
) -> VkResult {
    unsafe { *device.0.cast::<VkResult>() }
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkAcquireProfilingLockKHR" => Some(unsafe {
            std::mem::transmute::<AcquireProfilingLock, unsafe extern "system" fn()>(
                acquire_profiling_lock,
            )
        }),
        _ => None,
    }
}

fn acquire_profiling_lock_with_the_driver_returning(code: VkResult) -> vk::Result<()> {
    let mut returned = code;
    let handle = vk::Device(std::ptr::from_mut(&mut returned).cast());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    let device = Device::from_raw(handle, fp);
    let info = vk::AcquireProfilingLockInfoKHR::default().timeout(0);
    unsafe { device.acquire_profiling_lock_khr(&info) }
}

#[test]
fn acquiring_the_profiling_lock_returns_ok_for_its_success_code() {
    assert_eq!(
        acquire_profiling_lock_with_the_driver_returning(VkResult::SUCCESS),
        Ok(())
    );
}

#[test]
fn a_profiling_lock_that_times_out_is_returned_as_an_error() {
    assert_eq!(
        acquire_profiling_lock_with_the_driver_returning(VkResult::TIMEOUT),
        Err(VkResult::TIMEOUT),
        "the lock was not acquired, so the wrapper must not return Ok"
    );
}

#[test]
fn acquiring_the_profiling_lock_returns_every_listed_error_code_as_an_error() {
    let info = vk::command::ACQUIRE_PROFILING_LOCK_KHR;
    assert!(
        info.error_codes.iter().any(|code| !code.is_error()),
        "the registry lists a non-negative error code for vkAcquireProfilingLockKHR"
    );
    for &code in info.error_codes {
        assert_eq!(
            acquire_profiling_lock_with_the_driver_returning(code),
            Err(code),
            "{code} is an error code of vkAcquireProfilingLockKHR"
        );
    }
    for &code in info.success_codes {
        assert_eq!(
            acquire_profiling_lock_with_the_driver_returning(code),
            Ok(()),
            "{code} is a success code of vkAcquireProfilingLockKHR"
        );
    }
}

#[test]
fn acquiring_the_profiling_lock_returns_negative_codes_as_errors() {
    for code in [
        VkResult::ERROR_OUT_OF_HOST_MEMORY,
        VkResult::ERROR_DEVICE_LOST,
        VkResult::from_raw(-12345),
    ] {
        assert_eq!(
            acquire_profiling_lock_with_the_driver_returning(code),
            Err(code),
            "{code} is negative"
        );
    }
}
