use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::{Cell, RefCell};
use std::ffi::{CStr, c_char, c_void};
use vulkan::Device;
use vulkan::vk::{self, VkResult};

const REQUIRED_ALIGNMENT: usize = 16;

struct OddAddressesForBytes;

fn padded(layout: Layout) -> Layout {
    Layout::from_size_align(layout.size() + REQUIRED_ALIGNMENT, REQUIRED_ALIGNMENT)
        .expect("a layout of a few bytes more")
}

unsafe impl GlobalAlloc for OddAddressesForBytes {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() != 1 {
            return unsafe { System.alloc(layout) };
        }
        let base = unsafe { System.alloc(padded(layout)) };
        if base.is_null() {
            base
        } else {
            unsafe { base.add(1) }
        }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if layout.align() != 1 {
            unsafe { System.dealloc(pointer, layout) }
        } else {
            unsafe { System.dealloc(pointer.sub(1), padded(layout)) }
        }
    }
}

#[global_allocator]
static ALLOCATOR: OddAddressesForBytes = OddAddressesForBytes;

struct Driver {
    binary: RefCell<Vec<u8>>,
    grows_by_on_the_first_fill: Cell<usize>,
    buffers: RefCell<Vec<usize>>,
}

fn binary_of(len: usize) -> Vec<u8> {
    (0..len).map(|index| (index * 7 + 3) as u8).collect()
}

type GetShaderBinaryData =
    unsafe extern "system" fn(vk::Device, vk::ShaderEXT, *mut usize, *mut c_void) -> VkResult;

unsafe extern "system" fn get_shader_binary_data(
    device: vk::Device,
    _shader: vk::ShaderEXT,
    data_size: *mut usize,
    data: *mut c_void,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    if data.is_null() {
        unsafe { *data_size = driver.binary.borrow().len() };
        return VkResult::SUCCESS;
    }
    driver.buffers.borrow_mut().push(data as usize);
    let grows_by = driver.grows_by_on_the_first_fill.replace(0);
    if grows_by > 0 {
        let grown = binary_of(driver.binary.borrow().len() + grows_by);
        *driver.binary.borrow_mut() = grown;
    }
    let binary = driver.binary.borrow();
    if unsafe { *data_size } < binary.len() {
        return VkResult::INCOMPLETE;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(binary.as_ptr(), data.cast::<u8>(), binary.len());
        *data_size = binary.len();
    }
    VkResult::SUCCESS
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetShaderBinaryDataEXT" => Some(unsafe {
            std::mem::transmute::<GetShaderBinaryData, unsafe extern "system" fn()>(
                get_shader_binary_data,
            )
        }),
        _ => None,
    }
}

fn check(len: usize, grows_by: usize) {
    let driver = Driver {
        binary: RefCell::new(binary_of(len)),
        grows_by_on_the_first_fill: Cell::new(grows_by),
        buffers: RefCell::new(Vec::new()),
    };
    let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    let device = Device::from_raw(handle, fp);

    let returned = unsafe { device.get_shader_binary_data_ext(vk::ShaderEXT::default()) };
    let case = format!("a binary of {len} bytes that grows by {grows_by}");

    let buffers = driver.buffers.borrow();
    let expected_fills = if grows_by > 0 { 2 } else { 1 };
    assert_eq!(
        buffers.len(),
        expected_fills,
        "{case}: the driver is asked to fill a buffer once for each size it reported"
    );
    for buffer in buffers.iter() {
        assert_eq!(
            buffer % REQUIRED_ALIGNMENT,
            0,
            "{case}: VUID-vkGetShaderBinaryDataEXT-None-08499: pData {buffer:#x} is aligned to 16 bytes"
        );
    }
    assert_eq!(
        returned,
        Ok(binary_of(len + grows_by)),
        "{case}: the wrapper returns the bytes that the driver wrote, from the first one on"
    );
}

#[test]
fn the_allocator_of_this_test_returns_odd_addresses_for_bytes() {
    for len in [1, 15, 16, 17, 48, 4096] {
        let bytes = vec![0_u8; len];
        assert_eq!(
            bytes.as_ptr() as usize % 2,
            1,
            "a Vec<u8> of {len} bytes is not aligned to 16 bytes here"
        );
    }
}

#[test]
fn shader_binary_data_is_written_to_a_buffer_aligned_to_16_bytes() {
    let mut lens = vec![0, 1, 2, 15, 16, 17, 31, 32, 33, 48, 255, 256, 257, 4096];
    let mut state = 0x2545_f491_u32;
    for _ in 0..64 {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        lens.push((state >> 20) as usize);
    }
    for len in lens {
        check(len, 0);
    }
}

#[test]
fn shader_binary_data_that_grows_between_the_calls_is_asked_for_again_in_an_aligned_buffer() {
    for len in [0, 1, 15, 16, 17, 100] {
        for grows_by in [1, 15, 16, 17, 1000] {
            check(len, grows_by);
        }
    }
}
