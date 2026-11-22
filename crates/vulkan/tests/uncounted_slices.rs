use std::cell::Cell;
use std::ffi::{CStr, c_char};
use vulkan::Device;
use vulkan::vk;

fn check_setter<T: Default + Clone>(name: &str, pointer_set_by: impl Fn(&[T]) -> *const T) {
    assert!(
        pointer_set_by(&[]).is_null(),
        "{name}: an empty slice sets a null pointer"
    );
    let values = vec![T::default(); 3];
    for len in 1..=values.len() {
        assert_eq!(
            pointer_set_by(&values[..len]),
            values.as_ptr(),
            "{name}: a slice of {len} elements sets a pointer to its first element"
        );
    }
}

fn check_mut_setter<T: Default + Clone>(name: &str, pointer_set_by: impl Fn(&mut [T]) -> *mut T) {
    assert!(
        pointer_set_by(&mut []).is_null(),
        "{name}: an empty slice sets a null pointer"
    );
    let mut values = vec![T::default(); 3];
    let start = values.as_mut_ptr();
    for len in 1..=values.len() {
        assert_eq!(
            pointer_set_by(&mut values[..len]),
            start,
            "{name}: a slice of {len} elements sets a pointer to its first element"
        );
    }
}

#[test]
fn every_slice_setter_without_a_count_member_sets_null_for_an_empty_slice() {
    check_setter(
        "StdVideoEncodeAV1PictureInfo::buffer_removal_times",
        |value| {
            vk::StdVideoEncodeAV1PictureInfo::default()
                .buffer_removal_times(value)
                .p_buffer_removal_times
        },
    );
    check_mut_setter("DeviceFaultInfoEXT::address_infos", |value| {
        vk::DeviceFaultInfoEXT::default()
            .address_infos(value)
            .p_address_infos
    });
    check_mut_setter("DeviceFaultInfoEXT::vendor_infos", |value| {
        vk::DeviceFaultInfoEXT::default()
            .vendor_infos(value)
            .p_vendor_infos
    });
    check_setter("PipelineMultisampleStateCreateInfo::sample_mask", |value| {
        vk::PipelineMultisampleStateCreateInfo::default()
            .sample_mask(value)
            .p_sample_mask
    });
    check_setter(
        "StdVideoH265HrdParameters::sub_layer_hrd_parameters_nal",
        |value| {
            vk::StdVideoH265HrdParameters::default()
                .sub_layer_hrd_parameters_nal(value)
                .p_sub_layer_hrd_parameters_nal
        },
    );
    check_setter(
        "StdVideoH265HrdParameters::sub_layer_hrd_parameters_vcl",
        |value| {
            vk::StdVideoH265HrdParameters::default()
                .sub_layer_hrd_parameters_vcl(value)
                .p_sub_layer_hrd_parameters_vcl
        },
    );
}

type CmdSetSampleMask =
    unsafe extern "system" fn(vk::CommandBuffer, vk::SampleCountFlagBits, *const vk::SampleMask);

unsafe extern "system" fn cmd_set_sample_mask(
    command_buffer: vk::CommandBuffer,
    _samples: vk::SampleCountFlagBits,
    sample_mask: *const vk::SampleMask,
) {
    unsafe { &*command_buffer.0.cast::<Cell<*const vk::SampleMask>>() }.set(sample_mask);
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

fn passed_sample_mask(sample_mask: Option<&[vk::SampleMask]>) -> *const vk::SampleMask {
    let handle = vk::Device(std::ptr::null_mut());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    let device = Device::from_raw(handle, fp);
    let passed = Cell::new(std::ptr::dangling::<vk::SampleMask>());
    let command_buffer = vk::CommandBuffer(std::ptr::from_ref(&passed).cast_mut().cast());
    unsafe {
        device.cmd_set_sample_mask_ext(command_buffer, vk::SampleCountFlagBits::_1, sample_mask);
    }
    passed.get()
}

#[test]
fn the_wrapper_passes_null_for_an_empty_slice_without_a_count_parameter() {
    assert!(
        passed_sample_mask(None).is_null(),
        "None is passed as a null pointer"
    );
    assert!(
        passed_sample_mask(Some(&[])).is_null(),
        "an empty slice is passed as a null pointer"
    );
    let mask = [u32::MAX, 0];
    for len in 1..=mask.len() {
        assert_eq!(
            passed_sample_mask(Some(&mask[..len])),
            mask.as_ptr(),
            "a slice of {len} elements is passed as a pointer to its first element"
        );
    }
}
