use proptest::prelude::*;
use std::cell::RefCell;
use std::ffi::{CStr, c_char, c_void};
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
    calls: RefCell<Vec<(usize, usize, vk::QueryResultFlags)>>,
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
    flags: vk::QueryResultFlags,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    driver
        .calls
        .borrow_mut()
        .push((data as usize, data_size, flags));
    VkResult::SUCCESS
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
        _ => None,
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

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn query_results_are_written_only_to_bytes_with_the_alignment_of_the_results(
        offset in 0..16usize,
        len in 0..64usize,
        flags in flags(),
    ) {
        let driver = Driver::default();
        let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
        let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
        let device = Device::from_raw(handle, fp);
        let mut words = [0u64; 12];
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(words.as_mut_ptr().cast::<u8>().add(offset), len)
        };
        let address = bytes.as_ptr() as usize;
        let required = if flags.contains(vk::QueryResultFlags::_64) { 8 } else { 4 };
        let returned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| unsafe {
            device.get_query_pool_results(vk::QueryPool::default(), 0, 1, bytes, 8, flags)
        }));
        if address.is_multiple_of(required) {
            prop_assert_eq!(returned.ok(), Some(Ok(VkResult::SUCCESS)));
            prop_assert_eq!(driver.calls.into_inner(), vec![(address, len, flags)]);
        } else {
            let message = returned.expect_err("the wrapper panics");
            let message = message
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| message.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_default();
            prop_assert!(
                message.starts_with("vkGetQueryPoolResults: `data` has to start at an address aligned to 4 bytes, or to 8 bytes if `flags` contains `VK_QUERY_RESULT_64_BIT`"),
                "{}",
                message
            );
            prop_assert!(driver.calls.into_inner().is_empty());
        }
    }
}
