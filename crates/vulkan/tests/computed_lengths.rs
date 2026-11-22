use std::cell::Cell;
use std::ffi::{CStr, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind};
use vulkan::Device;
use vulkan::vk;

#[derive(Default)]
struct Driver {
    calls: Cell<usize>,
    passed: Cell<usize>,
}

type CmdSetSampleMask =
    unsafe extern "system" fn(vk::CommandBuffer, vk::SampleCountFlagBits, *const vk::SampleMask);

unsafe extern "system" fn cmd_set_sample_mask(
    command_buffer: vk::CommandBuffer,
    _samples: vk::SampleCountFlagBits,
    sample_mask: *const vk::SampleMask,
) {
    let driver = unsafe { &*command_buffer.0.cast::<Driver>() };
    driver.calls.set(driver.calls.get() + 1);
    driver.passed.set(sample_mask as usize);
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkCmdSetSampleMaskEXT" => Some(unsafe {
            std::mem::transmute::<CmdSetSampleMask, unsafe extern "system" fn()>(
                cmd_set_sample_mask,
            )
        }),
        _ => None,
    }
}

fn device() -> Device {
    let handle = vk::Device(std::ptr::null_mut());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    Device::from_raw(handle, fp)
}

enum Outcome {
    Passed(usize),
    PanickedBeforeTheCall,
}

fn set_sample_mask(samples: vk::SampleCountFlagBits, sample_mask: Option<&[u32]>) -> Outcome {
    let device = device();
    let driver = Driver::default();
    let command_buffer = vk::CommandBuffer(std::ptr::from_ref(&driver).cast_mut().cast());
    let returned = catch_unwind(AssertUnwindSafe(|| unsafe {
        device.cmd_set_sample_mask_ext(command_buffer, samples, sample_mask);
    }));
    match returned {
        Ok(()) => {
            assert_eq!(driver.calls.get(), 1, "the driver is called once");
            Outcome::Passed(driver.passed.get())
        }
        Err(_) => {
            assert_eq!(
                driver.calls.get(),
                0,
                "the wrapper panics before it calls the driver"
            );
            Outcome::PanickedBeforeTheCall
        }
    }
}

fn check(samples: vk::SampleCountFlagBits, words_read: usize) {
    let mask = [u32::MAX; 3];
    let raw = samples.as_raw();
    assert!(
        matches!(set_sample_mask(samples, None), Outcome::Passed(0)),
        "samples = {raw}: None is passed as a null pointer"
    );
    for len in 0..=mask.len() {
        let outcome = set_sample_mask(samples, Some(&mask[..len]));
        let case = format!(
            "samples = {raw}, of which the command reads {words_read} words, and a mask of {len} words"
        );
        if len == 0 {
            assert!(
                matches!(outcome, Outcome::Passed(0)),
                "{case}: an empty mask is passed as a null pointer"
            );
        } else if len < words_read {
            assert!(
                matches!(outcome, Outcome::PanickedBeforeTheCall),
                "{case}: the driver would read past the mask, so the wrapper panics"
            );
        } else {
            assert!(
                matches!(outcome, Outcome::Passed(passed) if passed == mask.as_ptr() as usize),
                "{case}: the mask is passed"
            );
        }
    }
}

#[test]
fn a_sample_mask_shorter_than_the_sample_count_needs_is_not_passed_to_the_driver() {
    let counts = vk::SampleCountFlagBits::variants();
    assert_eq!(
        counts.iter().map(|(count, _)| count.as_raw()).max(),
        Some(64),
        "the largest sample count needs two words"
    );
    for &(samples, _) in counts {
        check(samples, samples.as_raw().div_ceil(32) as usize);
    }
}

#[test]
fn a_value_that_is_not_one_sample_count_is_taken_as_the_number_the_c_api_passes() {
    for (raw, words_read) in [
        (0, 0),
        (3, 1),
        (33, 2),
        (65, 3),
        (96, 3),
        (97, 4),
        (u32::MAX, 134_217_728),
    ] {
        check(vk::SampleCountFlagBits::from_raw(raw), words_read);
    }
}

#[test]
fn version_data_takes_an_array_of_twice_the_uuid_size() {
    let version: [u8; 2 * vk::UUID_SIZE] = std::array::from_fn(|index| index as u8);
    let acceleration_structure =
        vk::AccelerationStructureVersionInfoKHR::default().version_data(&version);
    assert_eq!(acceleration_structure.p_version_data, version.as_ptr());
    let micromap = vk::MicromapVersionInfoEXT::default().version_data(&version);
    assert_eq!(micromap.p_version_data, version.as_ptr());
    let read = unsafe { std::slice::from_raw_parts(micromap.p_version_data, 2 * vk::UUID_SIZE) };
    assert_eq!(read, version);
}
