use std::ffi::{CStr, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicUsize, Ordering};
use vulkan::Instance;
use vulkan::vk::{self, Handle, VkResult};

const CREATED_DEVICE: u64 = 0xDE71CE;

type CreateDevice = unsafe extern "system" fn(
    vk::PhysicalDevice,
    *const vk::DeviceCreateInfo<'_>,
    *const vk::AllocationCallbacks<'_>,
    *mut vk::Device,
) -> VkResult;

type GetDeviceProcAddr =
    unsafe extern "system" fn(vk::Device, *const c_char) -> vk::PFN_vkVoidFunction;

unsafe extern "system" fn create_device(
    physical_device: vk::PhysicalDevice,
    _create_info: *const vk::DeviceCreateInfo<'_>,
    _allocator: *const vk::AllocationCallbacks<'_>,
    device: *mut vk::Device,
) -> VkResult {
    unsafe { &*physical_device.0.cast::<AtomicUsize>() }.fetch_add(1, Ordering::SeqCst);
    unsafe { device.write(vk::Device::from_raw_u64(CREATED_DEVICE)) };
    VkResult::SUCCESS
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    _name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    None
}

fn create_device_pointer(name: &[u8]) -> vk::PFN_vkVoidFunction {
    match name {
        b"vkCreateDevice" => Some(unsafe {
            std::mem::transmute::<CreateDevice, unsafe extern "system" fn()>(create_device)
        }),
        _ => None,
    }
}

unsafe extern "system" fn get_instance_proc_addr_without_the_device_loader(
    _instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    create_device_pointer(unsafe { CStr::from_ptr(name) }.to_bytes())
}

unsafe extern "system" fn get_instance_proc_addr(
    _instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetDeviceProcAddr" => Some(unsafe {
            std::mem::transmute::<GetDeviceProcAddr, unsafe extern "system" fn()>(
                get_device_proc_addr,
            )
        }),
        name => create_device_pointer(name),
    }
}

fn instance(get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr) -> Instance {
    let handle = vk::Instance::from_raw_u64(0x1A57);
    let fp = unsafe { vk::InstanceFn::load(get_instance_proc_addr, handle) };
    Instance::from_raw(handle, fp)
}

fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|message| message.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default()
}

#[test]
fn create_device_without_vk_get_device_proc_addr_panics_before_it_creates_a_device() {
    let instance = instance(Some(get_instance_proc_addr_without_the_device_loader));
    let created = AtomicUsize::new(0);
    let physical_device = vk::PhysicalDevice(std::ptr::from_ref(&created).cast_mut().cast());
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
    }));
    let payload = result.err().expect("create_device panics");
    assert_eq!(
        created.load(Ordering::SeqCst),
        0,
        "vkCreateDevice is not called, so no device is lost"
    );
    assert_eq!(
        panic_message(payload.as_ref()),
        "vkGetDeviceProcAddr is not loaded"
    );
}

#[test]
fn create_device_with_vk_get_device_proc_addr_creates_one_device() {
    let instance = instance(Some(get_instance_proc_addr));
    let created = AtomicUsize::new(0);
    let physical_device = vk::PhysicalDevice(std::ptr::from_ref(&created).cast_mut().cast());
    let device =
        unsafe { instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None) }
            .expect("vkCreateDevice succeeds");
    assert_eq!(device.handle(), vk::Device::from_raw_u64(CREATED_DEVICE));
    assert_eq!(created.load(Ordering::SeqCst), 1);
}
