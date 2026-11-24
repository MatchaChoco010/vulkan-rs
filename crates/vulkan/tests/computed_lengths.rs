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

#[test]
fn a_sample_mask_of_a_pipeline_has_a_word_for_every_sample_of_every_sample_count() {
    let _: fn(
        vk::PipelineMultisampleStateCreateInfo<'static>,
        &'static [vk::SampleMask; 2],
    ) -> vk::PipelineMultisampleStateCreateInfo<'static> =
        vk::PipelineMultisampleStateCreateInfo::sample_mask;
    let mask = [u32::MAX, 1];
    let words = mask.len();
    for (samples, name) in vk::SampleCountFlagBits::variants() {
        let info = vk::PipelineMultisampleStateCreateInfo::default()
            .sample_mask(&mask)
            .rasterization_samples(*samples);
        assert_eq!(info.p_sample_mask, mask.as_ptr());
        let read = (info.rasterization_samples.as_raw() as usize).div_ceil(32);
        assert!(
            read <= words,
            "{name}: the C API reads {read} words of the {words} that the setter borrows"
        );
    }
    assert!(
        vk::PipelineMultisampleStateCreateInfo::default()
            .p_sample_mask
            .is_null(),
        "a structure whose setter was not called has no sample mask"
    );
}

#[test]
fn the_hrd_parameters_of_h265_have_an_element_for_every_sub_layer() {
    type Setter = fn(
        vk::StdVideoH265HrdParameters<'static>,
        &'static [vk::StdVideoH265SubLayerHrdParameters; 7],
    ) -> vk::StdVideoH265HrdParameters<'static>;
    let _: [Setter; 2] = [
        vk::StdVideoH265HrdParameters::sub_layer_hrd_parameters_nal,
        vk::StdVideoH265HrdParameters::sub_layer_hrd_parameters_vcl,
    ];
    let parameters =
        [vk::StdVideoH265SubLayerHrdParameters::default(); vk::STD_VIDEO_H265_SUBLAYERS_LIST_SIZE];
    let hrd = vk::StdVideoH265HrdParameters::default()
        .sub_layer_hrd_parameters_nal(&parameters)
        .sub_layer_hrd_parameters_vcl(&parameters);
    assert_eq!(hrd.p_sub_layer_hrd_parameters_nal, parameters.as_ptr());
    assert_eq!(hrd.p_sub_layer_hrd_parameters_vcl, parameters.as_ptr());
    let largest_max_sub_layers_minus1 = 6;
    assert_eq!(
        largest_max_sub_layers_minus1 + 1,
        parameters.len(),
        "the C API reads *_max_sub_layers_minus1 + 1 elements"
    );
    assert_eq!(hrd.cpb_cnt_minus1.len(), parameters.len());
    let unset = vk::StdVideoH265HrdParameters::default();
    assert!(
        unset.p_sub_layer_hrd_parameters_nal.is_null()
            && unset.p_sub_layer_hrd_parameters_vcl.is_null()
    );
}

#[test]
fn the_buffer_removal_times_of_av1_have_an_element_for_every_operating_point() {
    let _: fn(
        vk::StdVideoEncodeAV1PictureInfo<'static>,
        &'static [u32; 32],
    ) -> vk::StdVideoEncodeAV1PictureInfo<'static> =
        vk::StdVideoEncodeAV1PictureInfo::buffer_removal_times;
    let times = [7_u32; 32];
    let picture = vk::StdVideoEncodeAV1PictureInfo::default().buffer_removal_times(&times);
    assert_eq!(picture.p_buffer_removal_times, times.as_ptr());
    let largest_operating_points_cnt_minus_1 = 0b1_1111;
    assert_eq!(largest_operating_points_cnt_minus_1 + 1, times.len());
    assert!(
        vk::StdVideoEncodeAV1PictureInfo::default()
            .p_buffer_removal_times
            .is_null()
    );
}
