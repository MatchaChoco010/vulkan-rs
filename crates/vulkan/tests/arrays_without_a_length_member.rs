use std::cell::Cell;
use std::ffi::{CStr, c_char};
use vulkan::Device;
use vulkan::vk::{self, VkResult};

#[derive(Default)]
struct Driver {
    available_addresses: u32,
    available_vendors: u32,
    address_infos_seen: Cell<usize>,
    vendor_infos_seen: Cell<usize>,
}

type GetDeviceFaultInfo = unsafe extern "system" fn(
    vk::Device,
    *mut vk::DeviceFaultCountsEXT<'_>,
    *mut vk::DeviceFaultInfoEXT<'_>,
) -> VkResult;

fn reported_address(index: u32) -> vk::DeviceAddress {
    0x1000 + u64::from(index) * 0x10
}

fn vendor_fault_code(index: u32) -> u64 {
    0xf00 + u64::from(index)
}

unsafe extern "system" fn get_device_fault_info(
    device: vk::Device,
    counts: *mut vk::DeviceFaultCountsEXT<'_>,
    info: *mut vk::DeviceFaultInfoEXT<'_>,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    let counts = unsafe { &mut *counts };
    if info.is_null() {
        counts.address_info_count = driver.available_addresses;
        counts.vendor_info_count = driver.available_vendors;
        counts.vendor_binary_size = 0;
        return VkResult::SUCCESS;
    }
    let info = unsafe { &*info };
    driver.address_infos_seen.set(info.p_address_infos as usize);
    driver.vendor_infos_seen.set(info.p_vendor_infos as usize);
    let addresses = counts.address_info_count.min(driver.available_addresses);
    let vendors = counts.vendor_info_count.min(driver.available_vendors);
    for index in 0..addresses {
        unsafe {
            (*info.p_address_infos.add(index as usize)).reported_address = reported_address(index);
        }
    }
    for index in 0..vendors {
        unsafe {
            (*info.p_vendor_infos.add(index as usize)).vendor_fault_code = vendor_fault_code(index);
        }
    }
    let complete = addresses == driver.available_addresses && vendors == driver.available_vendors;
    counts.address_info_count = addresses;
    counts.vendor_info_count = vendors;
    if complete {
        VkResult::SUCCESS
    } else {
        VkResult::INCOMPLETE
    }
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetDeviceFaultInfoEXT" => Some(unsafe {
            std::mem::transmute::<GetDeviceFaultInfo, unsafe extern "system" fn()>(
                get_device_fault_info,
            )
        }),
        _ => None,
    }
}

fn device_of(driver: &Driver) -> Device {
    let handle = vk::Device(std::ptr::from_ref(driver).cast_mut().cast());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    Device::from_raw(handle, fp)
}

#[test]
fn the_driver_fills_every_element_of_the_fault_arrays_that_the_setters_take() {
    for (available_addresses, available_vendors) in [(0, 0), (1, 1), (3, 2), (2, 5), (0, 4)] {
        let driver = Driver {
            available_addresses,
            available_vendors,
            ..Default::default()
        };
        let device = device_of(&driver);
        let mut counts = vk::DeviceFaultCountsEXT::default();
        let result = unsafe { device.get_device_fault_info_ext(&mut counts, None) };
        assert_eq!(result, Ok(VkResult::SUCCESS));
        assert_eq!(
            (counts.address_info_count, counts.vendor_info_count),
            (available_addresses, available_vendors)
        );

        let mut addresses =
            vec![vk::DeviceFaultAddressInfoEXT::default(); counts.address_info_count as usize];
        let mut vendors =
            vec![vk::DeviceFaultVendorInfoEXT::default(); counts.vendor_info_count as usize];
        let addresses_start = addresses.as_ptr() as usize;
        let vendors_start = vendors.as_ptr() as usize;
        let mut info = vk::DeviceFaultInfoEXT::default()
            .address_infos(&mut addresses)
            .vendor_infos(&mut vendors);
        let result = unsafe { device.get_device_fault_info_ext(&mut counts, Some(&mut info)) };
        assert_eq!(result, Ok(VkResult::SUCCESS));

        let expected_start = |start: usize, len: u32| if len == 0 { 0 } else { start };
        assert_eq!(
            driver.address_infos_seen.get(),
            expected_start(addresses_start, available_addresses),
            "pAddressInfos points to the first of {available_addresses} elements, or is null for none"
        );
        assert_eq!(
            driver.vendor_infos_seen.get(),
            expected_start(vendors_start, available_vendors),
            "pVendorInfos points to the first of {available_vendors} elements, or is null for none"
        );
        assert_eq!(
            addresses
                .iter()
                .map(|address| address.reported_address)
                .collect::<Vec<_>>(),
            (0..available_addresses)
                .map(reported_address)
                .collect::<Vec<_>>(),
            "every element that the driver wrote to pAddressInfos is in the slice"
        );
        assert_eq!(
            vendors
                .iter()
                .map(|vendor| vendor.vendor_fault_code)
                .collect::<Vec<_>>(),
            (0..available_vendors)
                .map(vendor_fault_code)
                .collect::<Vec<_>>(),
            "every element that the driver wrote to pVendorInfos is in the slice"
        );
    }
}

#[test]
fn viewport_depths_takes_a_slice_and_sets_its_count() {
    let depths = [
        vk::Viewport::default().min_depth(0.0).max_depth(1.0),
        vk::Viewport::default().min_depth(0.25).max_depth(0.75),
        vk::Viewport::default().min_depth(0.5).max_depth(0.5),
    ];
    for len in 0..=depths.len() {
        let info = vk::CommandBufferInheritanceViewportScissorInfoNV::default()
            .viewport_scissor2_d(true)
            .viewport_depths(&depths[..len]);
        assert_eq!(
            info.viewport_depth_count as usize, len,
            "viewportDepthCount is the length of the slice"
        );
        if len > 0 {
            assert_eq!(
                info.p_viewport_depths,
                depths.as_ptr(),
                "pViewportDepths points to the first of {len} elements"
            );
            let passed = unsafe { std::slice::from_raw_parts(info.p_viewport_depths, len) };
            assert!(
                passed == &depths[..len],
                "the {len} elements behind pViewportDepths are the elements of the slice"
            );
        }
    }
}

#[test]
fn the_string_member_of_a_union_takes_a_c_string() {
    for text in [c"", c"a", c"frequency", c"a longer value string"] {
        let data = vk::PerformanceValueDataINTEL::value_string(text);
        let pointer = unsafe { data.value_string };
        assert_eq!(
            pointer,
            text.as_ptr(),
            "valueString points to the first character of the string"
        );
        assert_eq!(unsafe { CStr::from_ptr(pointer) }, text);
    }
}
