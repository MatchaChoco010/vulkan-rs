#![allow(dead_code)]

use proptest::test_runner::Config;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::{CStr, c_char, c_void};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::OnceLock;
use vulkan::vk::{self, Handle, VkResult};
use vulkan::{Device, Entry, Instance};

pub const INSTANCE: u64 = 0x1A57;
pub const DEVICE: u64 = 0xDE71CE;
pub const READ_LIMIT: u64 = 128;
pub const INSTANCE_VERSION: u32 = vk::make_api_version(0, 1, 3, 250);

pub fn cases(default: u32) -> Config {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    Config {
        cases,
        failure_persistence: None,
        ..Config::default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reply {
    Conforming(u32),
    Fail(VkResult),
    OverReport(u32),
    IncompleteWithoutWriting,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enumeration {
    pub reply: Option<Reply>,
    pub array: bool,
    pub capacity: u64,
    pub count: u64,
    pub written: Vec<u64>,
    pub result: VkResult,
    pub structures: Vec<(i32, bool)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Call {
    pub command: &'static str,
    pub handles: Vec<u64>,
    pub scalars: Vec<u64>,
    pub arrays: Vec<Option<Vec<u64>>>,
    pub strings: Vec<Option<Vec<u8>>>,
    pub enumeration: Option<Enumeration>,
}

#[derive(Clone, Debug, Default)]
pub struct Script {
    pub replies: Vec<Reply>,
    pub results: Vec<VkResult>,
    pub outputs: Vec<u64>,
}

#[derive(Default)]
struct Driver {
    replies: VecDeque<Reply>,
    results: VecDeque<VkResult>,
    outputs: VecDeque<u64>,
    calls: Vec<Call>,
    requests: Vec<(u64, String)>,
    enumerations: u64,
}

thread_local! {
    static DRIVER: RefCell<Driver> = RefCell::new(Driver::default());
}

fn with_driver<R>(f: impl FnOnce(&mut Driver) -> R) -> Option<R> {
    DRIVER
        .try_with(|driver| {
            driver
                .try_borrow_mut()
                .ok()
                .map(|mut driver| f(&mut driver))
        })
        .ok()
        .flatten()
}

pub fn reset() {
    with_driver(|driver| *driver = Driver::default());
}

pub fn take_requests() -> Vec<(u64, String)> {
    with_driver(|driver| std::mem::take(&mut driver.requests)).unwrap_or_default()
}

pub fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|message| message.to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_default()
}

pub fn run<R>(script: Script, f: impl FnOnce() -> R) -> (Result<R, String>, Vec<Call>) {
    with_driver(|driver| {
        *driver = Driver {
            replies: script.replies.into(),
            results: script.results.into(),
            outputs: script.outputs.into(),
            ..Driver::default()
        }
    });
    let result = catch_unwind(AssertUnwindSafe(f)).map_err(|payload| panic_message(&*payload));
    let calls = with_driver(|driver| std::mem::take(&mut driver.calls)).unwrap_or_default();
    (result, calls)
}

pub fn element_id(call: u64, index: u64) -> u64 {
    (call << 8) | index
}

pub fn byte(id: u64) -> u8 {
    (id ^ (id >> 8).wrapping_mul(0x9D)) as u8
}

pub fn extension_name(id: u64) -> [c_char; vk::MAX_EXTENSION_NAME_SIZE] {
    let mut name = [0; vk::MAX_EXTENSION_NAME_SIZE];
    let text = format!("VK_FAKE_extension_{id}");
    for (slot, byte) in name.iter_mut().zip(text.bytes()) {
        *slot = byte as c_char;
    }
    name
}

fn record(call: Call) {
    with_driver(|driver| driver.calls.push(call));
}

fn next_result() -> VkResult {
    with_driver(|driver| driver.results.pop_front())
        .flatten()
        .unwrap_or(VkResult::ERROR_UNKNOWN)
}

fn next_output() -> Option<u64> {
    with_driver(|driver| driver.outputs.pop_front()).flatten()
}

unsafe fn read<T>(pointer: *const T, count: u64, id: impl Fn(&T) -> u64) -> Option<Vec<u64>> {
    if pointer.is_null() {
        return None;
    }
    if count > READ_LIMIT {
        return Some(Vec::new());
    }
    Some(
        (0..count as usize)
            .map(|index| id(unsafe { &*pointer.add(index) }))
            .collect(),
    )
}

unsafe fn read_string(pointer: *const c_char) -> Option<Vec<u8>> {
    if pointer.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(pointer) }.to_bytes().to_vec())
    }
}

struct Answer {
    count: Option<u64>,
    written: Vec<u64>,
    result: VkResult,
}

fn enumerate(
    command: &'static str,
    handles: Vec<u64>,
    strings: Vec<Option<Vec<u8>>>,
    array: bool,
    capacity: u64,
    structures: Vec<(i32, bool)>,
) -> Answer {
    let (reply, call) = with_driver(|driver| {
        driver.enumerations += 1;
        (driver.replies.pop_front(), driver.enumerations)
    })
    .unwrap_or((None, 0));
    let (count, written, result) = match reply {
        _ if array && capacity > READ_LIMIT => (None, 0, VkResult::ERROR_UNKNOWN),
        None => (None, 0, VkResult::ERROR_UNKNOWN),
        Some(Reply::Fail(code)) => (None, 0, code),
        Some(Reply::IncompleteWithoutWriting) => (None, 0, VkResult::INCOMPLETE),
        Some(Reply::Conforming(available)) if !array => {
            (Some(u64::from(available)), 0, VkResult::SUCCESS)
        }
        Some(Reply::Conforming(available)) => {
            let written = capacity.min(u64::from(available));
            let result = if written < u64::from(available) {
                VkResult::INCOMPLETE
            } else {
                VkResult::SUCCESS
            };
            (Some(written), written, result)
        }
        Some(Reply::OverReport(extra)) if !array => (Some(u64::from(extra)), 0, VkResult::SUCCESS),
        Some(Reply::OverReport(extra)) => (
            Some(capacity + u64::from(extra)),
            capacity,
            VkResult::SUCCESS,
        ),
    };
    let written: Vec<u64> = (0..written).map(|index| element_id(call, index)).collect();
    record(Call {
        command,
        handles,
        strings,
        enumeration: Some(Enumeration {
            reply,
            array,
            capacity,
            count: count.unwrap_or(capacity),
            written: written.clone(),
            result,
            structures,
        }),
        ..Call::default()
    });
    Answer {
        count,
        written,
        result,
    }
}

unsafe fn write_u32_count(count: *mut u32, value: Option<u64>) {
    if let Some(value) = value {
        unsafe { count.write(u32::try_from(value).unwrap_or(u32::MAX)) };
    }
}

unsafe extern "system" fn enumerate_physical_devices(
    instance: vk::Instance,
    count: *mut u32,
    devices: *mut vk::PhysicalDevice,
) -> VkResult {
    if count.is_null() {
        return VkResult::ERROR_UNKNOWN;
    }
    let answer = enumerate(
        "vkEnumeratePhysicalDevices",
        vec![instance.as_raw_u64()],
        Vec::new(),
        !devices.is_null(),
        u64::from(unsafe { count.read() }),
        Vec::new(),
    );
    unsafe {
        write_u32_count(count, answer.count);
        for (index, &id) in answer.written.iter().enumerate() {
            devices
                .add(index)
                .write(vk::PhysicalDevice::from_raw_u64(id));
        }
    }
    answer.result
}

unsafe extern "system" fn get_pipeline_cache_data(
    device: vk::Device,
    pipeline_cache: vk::PipelineCache,
    size: *mut usize,
    data: *mut c_void,
) -> VkResult {
    if size.is_null() {
        return VkResult::ERROR_UNKNOWN;
    }
    let answer = enumerate(
        "vkGetPipelineCacheData",
        vec![device.as_raw_u64(), pipeline_cache.as_raw_u64()],
        Vec::new(),
        !data.is_null(),
        unsafe { size.read() } as u64,
        Vec::new(),
    );
    unsafe {
        if let Some(value) = answer.count {
            size.write(usize::try_from(value).unwrap_or(usize::MAX));
        }
        for (index, &id) in answer.written.iter().enumerate() {
            data.cast::<u8>().add(index).write(byte(id));
        }
    }
    answer.result
}

unsafe fn write_extension_properties(
    properties: *mut vk::ExtensionProperties,
    count: *mut u32,
    answer: &Answer,
) {
    unsafe {
        write_u32_count(count, answer.count);
        for (index, &id) in answer.written.iter().enumerate() {
            properties.add(index).write(vk::ExtensionProperties {
                extension_name: extension_name(id),
                spec_version: id as u32,
            });
        }
    }
}

unsafe extern "system" fn enumerate_device_extension_properties(
    physical_device: vk::PhysicalDevice,
    layer_name: *const c_char,
    count: *mut u32,
    properties: *mut vk::ExtensionProperties,
) -> VkResult {
    if count.is_null() {
        return VkResult::ERROR_UNKNOWN;
    }
    let answer = enumerate(
        "vkEnumerateDeviceExtensionProperties",
        vec![physical_device.as_raw_u64()],
        vec![unsafe { read_string(layer_name) }],
        !properties.is_null(),
        u64::from(unsafe { count.read() }),
        Vec::new(),
    );
    unsafe { write_extension_properties(properties, count, &answer) };
    answer.result
}

unsafe extern "system" fn enumerate_instance_version(api_version: *mut u32) -> VkResult {
    if api_version.is_null() {
        return VkResult::ERROR_UNKNOWN;
    }
    unsafe { api_version.write(INSTANCE_VERSION) };
    VkResult::SUCCESS
}

unsafe extern "system" fn enumerate_instance_extension_properties(
    layer_name: *const c_char,
    count: *mut u32,
    properties: *mut vk::ExtensionProperties,
) -> VkResult {
    if count.is_null() {
        return VkResult::ERROR_UNKNOWN;
    }
    let answer = enumerate(
        "vkEnumerateInstanceExtensionProperties",
        Vec::new(),
        vec![unsafe { read_string(layer_name) }],
        !properties.is_null(),
        u64::from(unsafe { count.read() }),
        Vec::new(),
    );
    unsafe { write_extension_properties(properties, count, &answer) };
    answer.result
}

unsafe extern "system" fn get_physical_device_queue_family_properties(
    physical_device: vk::PhysicalDevice,
    count: *mut u32,
    properties: *mut vk::QueueFamilyProperties,
) {
    if count.is_null() {
        return;
    }
    let answer = enumerate(
        "vkGetPhysicalDeviceQueueFamilyProperties",
        vec![physical_device.as_raw_u64()],
        Vec::new(),
        !properties.is_null(),
        u64::from(unsafe { count.read() }),
        Vec::new(),
    );
    unsafe {
        write_u32_count(count, answer.count);
        for (index, &id) in answer.written.iter().enumerate() {
            properties.add(index).write(vk::QueueFamilyProperties {
                queue_count: id as u32,
                ..Default::default()
            });
        }
    }
}

unsafe extern "system" fn get_physical_device_queue_family_properties2(
    physical_device: vk::PhysicalDevice,
    count: *mut u32,
    properties: *mut vk::QueueFamilyProperties2<'_>,
) {
    if count.is_null() {
        return;
    }
    let capacity = u64::from(unsafe { count.read() });
    let structures = if properties.is_null() || capacity > READ_LIMIT {
        Vec::new()
    } else {
        (0..capacity as usize)
            .map(|index| {
                let element = unsafe { &*properties.add(index) };
                (element.s_type.as_raw(), element.p_next.is_null())
            })
            .collect()
    };
    let answer = enumerate(
        "vkGetPhysicalDeviceQueueFamilyProperties2",
        vec![physical_device.as_raw_u64()],
        Vec::new(),
        !properties.is_null(),
        capacity,
        structures,
    );
    unsafe {
        write_u32_count(count, answer.count);
        for (index, &id) in answer.written.iter().enumerate() {
            (*properties.add(index)).queue_family_properties.queue_count = id as u32;
        }
    }
}

unsafe extern "system" fn get_physical_device_surface_support_khr(
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    surface: vk::SurfaceKHR,
    supported: *mut vk::Bool32,
) -> VkResult {
    record(Call {
        command: "vkGetPhysicalDeviceSurfaceSupportKHR",
        handles: vec![physical_device.as_raw_u64(), surface.as_raw_u64()],
        scalars: vec![u64::from(queue_family_index)],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !supported.is_null()
    {
        unsafe { supported.write(value as vk::Bool32) };
    }
    next_result()
}

unsafe extern "system" fn create_instance(
    create_info: *const vk::InstanceCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    instance: *mut vk::Instance,
) -> VkResult {
    record(Call {
        command: "vkCreateInstance",
        arrays: vec![
            unsafe { read(create_info, 1, |info| u64::from(info.enabled_layer_count)) },
            unsafe {
                read(allocator, 1, |callbacks| {
                    callbacks.p_user_data as usize as u64
                })
            },
        ],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !instance.is_null()
    {
        unsafe { instance.write(vk::Instance::from_raw_u64(value)) };
    }
    next_result()
}

unsafe extern "system" fn create_device(
    physical_device: vk::PhysicalDevice,
    create_info: *const vk::DeviceCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    device: *mut vk::Device,
) -> VkResult {
    record(Call {
        command: "vkCreateDevice",
        handles: vec![physical_device.as_raw_u64()],
        arrays: vec![
            unsafe {
                read(create_info, 1, |info| {
                    u64::from(info.queue_create_info_count)
                })
            },
            unsafe {
                read(allocator, 1, |callbacks| {
                    callbacks.p_user_data as usize as u64
                })
            },
        ],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !device.is_null()
    {
        unsafe { device.write(vk::Device::from_raw_u64(value)) };
    }
    next_result()
}

unsafe extern "system" fn device_wait_idle(device: vk::Device) -> VkResult {
    record(Call {
        command: "vkDeviceWaitIdle",
        handles: vec![device.as_raw_u64()],
        ..Call::default()
    });
    next_result()
}

unsafe extern "system" fn create_buffer(
    device: vk::Device,
    create_info: *const vk::BufferCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    buffer: *mut vk::Buffer,
) -> VkResult {
    record(Call {
        command: "vkCreateBuffer",
        handles: vec![device.as_raw_u64()],
        arrays: vec![unsafe { read(create_info, 1, |info| info.size) }, unsafe {
            read(allocator, 1, |callbacks| {
                callbacks.p_user_data as usize as u64
            })
        }],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !buffer.is_null()
    {
        unsafe { buffer.write(vk::Buffer::from_raw_u64(value)) };
    }
    next_result()
}

unsafe extern "system" fn queue_present_khr(
    queue: vk::Queue,
    present_info: *const vk::PresentInfoKHR<'_>,
) -> VkResult {
    record(Call {
        command: "vkQueuePresentKHR",
        handles: vec![queue.as_raw_u64()],
        arrays: vec![unsafe { read(present_info, 1, |info| u64::from(info.swapchain_count)) }],
        ..Call::default()
    });
    next_result()
}

unsafe extern "system" fn acquire_next_image_khr(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    timeout: u64,
    semaphore: vk::Semaphore,
    fence: vk::Fence,
    image_index: *mut u32,
) -> VkResult {
    record(Call {
        command: "vkAcquireNextImageKHR",
        handles: vec![
            device.as_raw_u64(),
            swapchain.as_raw_u64(),
            semaphore.as_raw_u64(),
            fence.as_raw_u64(),
        ],
        scalars: vec![timeout],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !image_index.is_null()
    {
        unsafe { image_index.write(value as u32) };
    }
    next_result()
}

unsafe extern "system" fn wait_for_fences(
    device: vk::Device,
    fence_count: u32,
    fences: *const vk::Fence,
    wait_all: vk::Bool32,
    timeout: u64,
) -> VkResult {
    record(Call {
        command: "vkWaitForFences",
        handles: vec![device.as_raw_u64()],
        scalars: vec![u64::from(fence_count), u64::from(wait_all), timeout],
        arrays: vec![unsafe { read(fences, u64::from(fence_count), |fence| fence.as_raw_u64()) }],
        ..Call::default()
    });
    next_result()
}

unsafe extern "system" fn get_fence_status(device: vk::Device, fence: vk::Fence) -> VkResult {
    record(Call {
        command: "vkGetFenceStatus",
        handles: vec![device.as_raw_u64(), fence.as_raw_u64()],
        ..Call::default()
    });
    next_result()
}

unsafe extern "system" fn get_swapchain_time_domain_properties_ext(
    device: vk::Device,
    swapchain: vk::SwapchainKHR,
    properties: *mut vk::SwapchainTimeDomainPropertiesEXT<'_>,
    counter: *mut u64,
) -> VkResult {
    record(Call {
        command: "vkGetSwapchainTimeDomainPropertiesEXT",
        handles: vec![device.as_raw_u64(), swapchain.as_raw_u64()],
        arrays: vec![unsafe {
            read(properties.cast_const(), 1, |properties| {
                u64::from(properties.time_domain_count)
            })
        }],
        ..Call::default()
    });
    if let Some(value) = next_output()
        && !counter.is_null()
    {
        unsafe { counter.write(value) };
    }
    next_result()
}

unsafe extern "system" fn create_graphics_pipelines(
    device: vk::Device,
    pipeline_cache: vk::PipelineCache,
    create_info_count: u32,
    create_infos: *const vk::GraphicsPipelineCreateInfo<'_>,
    allocator: *const vk::AllocationCallbacks<'_>,
    pipelines: *mut vk::Pipeline,
) -> VkResult {
    let count = u64::from(create_info_count);
    record(Call {
        command: "vkCreateGraphicsPipelines",
        handles: vec![device.as_raw_u64(), pipeline_cache.as_raw_u64()],
        scalars: vec![count],
        arrays: vec![
            unsafe { read(create_infos, count, |info| u64::from(info.stage_count)) },
            unsafe {
                read(allocator, 1, |callbacks| {
                    callbacks.p_user_data as usize as u64
                })
            },
        ],
        ..Call::default()
    });
    if !pipelines.is_null() && count <= READ_LIMIT {
        for index in 0..count as usize {
            if let Some(value) = next_output() {
                unsafe {
                    pipelines
                        .add(index)
                        .write(vk::Pipeline::from_raw_u64(value))
                };
            }
        }
    }
    next_result()
}

unsafe extern "system" fn cmd_bind_vertex_buffers(
    command_buffer: vk::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const vk::Buffer,
    offsets: *const vk::DeviceSize,
) {
    let count = u64::from(binding_count);
    record(Call {
        command: "vkCmdBindVertexBuffers",
        handles: vec![command_buffer.as_raw_u64()],
        scalars: vec![u64::from(first_binding), count],
        arrays: vec![
            unsafe { read(buffers, count, |buffer| buffer.as_raw_u64()) },
            unsafe { read(offsets, count, |offset| *offset) },
        ],
        ..Call::default()
    });
}

unsafe extern "system" fn cmd_bind_vertex_buffers2(
    command_buffer: vk::CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const vk::Buffer,
    offsets: *const vk::DeviceSize,
    sizes: *const vk::DeviceSize,
    strides: *const vk::DeviceSize,
) {
    let count = u64::from(binding_count);
    record(Call {
        command: "vkCmdBindVertexBuffers2",
        handles: vec![command_buffer.as_raw_u64()],
        scalars: vec![u64::from(first_binding), count],
        arrays: vec![
            unsafe { read(buffers, count, |buffer| buffer.as_raw_u64()) },
            unsafe { read(offsets, count, |offset| *offset) },
            unsafe { read(sizes, count, |size| *size) },
            unsafe { read(strides, count, |stride| *stride) },
        ],
        ..Call::default()
    });
}

unsafe extern "system" fn cmd_build_acceleration_structures_khr(
    command_buffer: vk::CommandBuffer,
    info_count: u32,
    infos: *const vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
    build_range_infos: *const *const vk::AccelerationStructureBuildRangeInfoKHR,
) {
    let count = u64::from(info_count);
    let geometry_counts = unsafe { read(infos, count, |info| u64::from(info.geometry_count)) };
    let mut arrays = vec![geometry_counts.clone()];
    if let Some(geometry_counts) = geometry_counts
        && !build_range_infos.is_null()
    {
        for (index, &geometry_count) in geometry_counts.iter().enumerate() {
            let entry = unsafe { build_range_infos.add(index).read() };
            arrays.push(unsafe {
                read(entry, geometry_count, |range| {
                    u64::from(range.primitive_count)
                })
            });
        }
    }
    record(Call {
        command: "vkCmdBuildAccelerationStructuresKHR",
        handles: vec![command_buffer.as_raw_u64()],
        scalars: vec![count],
        arrays,
        ..Call::default()
    });
}

macro_rules! pfn {
    ($function:expr, $pfn:ty) => {{
        let typed: $pfn = Some($function);
        unsafe { std::mem::transmute::<$pfn, vk::PFN_vkVoidFunction>(typed) }
    }};
}

fn record_request(handle: u64, name: &CStr) {
    with_driver(|driver| {
        driver
            .requests
            .push((handle, name.to_string_lossy().into_owned()))
    });
}

unsafe extern "system" fn get_device_proc_addr(
    device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    let name = unsafe { CStr::from_ptr(name) };
    record_request(device.as_raw_u64(), name);
    match name.to_bytes() {
        b"vkGetPipelineCacheData" => pfn!(get_pipeline_cache_data, vk::PFN_vkGetPipelineCacheData),
        b"vkDeviceWaitIdle" => pfn!(device_wait_idle, vk::PFN_vkDeviceWaitIdle),
        b"vkCreateBuffer" => pfn!(create_buffer, vk::PFN_vkCreateBuffer),
        b"vkQueuePresentKHR" => pfn!(queue_present_khr, vk::PFN_vkQueuePresentKHR),
        b"vkAcquireNextImageKHR" => pfn!(acquire_next_image_khr, vk::PFN_vkAcquireNextImageKHR),
        b"vkWaitForFences" => pfn!(wait_for_fences, vk::PFN_vkWaitForFences),
        b"vkGetFenceStatus" => pfn!(get_fence_status, vk::PFN_vkGetFenceStatus),
        b"vkGetSwapchainTimeDomainPropertiesEXT" => pfn!(
            get_swapchain_time_domain_properties_ext,
            vk::PFN_vkGetSwapchainTimeDomainPropertiesEXT
        ),
        b"vkCreateGraphicsPipelines" => {
            pfn!(create_graphics_pipelines, vk::PFN_vkCreateGraphicsPipelines)
        }
        b"vkCmdBindVertexBuffers" => pfn!(cmd_bind_vertex_buffers, vk::PFN_vkCmdBindVertexBuffers),
        b"vkCmdBindVertexBuffers2" => {
            pfn!(cmd_bind_vertex_buffers2, vk::PFN_vkCmdBindVertexBuffers2)
        }
        b"vkCmdBuildAccelerationStructuresKHR" => pfn!(
            cmd_build_acceleration_structures_khr,
            vk::PFN_vkCmdBuildAccelerationStructuresKHR
        ),
        _ => None,
    }
}

unsafe extern "system" fn get_instance_proc_addr(
    instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    let name = unsafe { CStr::from_ptr(name) };
    record_request(instance.as_raw_u64(), name);
    match name.to_bytes() {
        b"vkCreateInstance" => pfn!(create_instance, vk::PFN_vkCreateInstance),
        b"vkEnumerateInstanceVersion" => pfn!(
            enumerate_instance_version,
            vk::PFN_vkEnumerateInstanceVersion
        ),
        b"vkEnumerateInstanceExtensionProperties" => pfn!(
            enumerate_instance_extension_properties,
            vk::PFN_vkEnumerateInstanceExtensionProperties
        ),
        b"vkCreateDevice" => pfn!(create_device, vk::PFN_vkCreateDevice),
        b"vkGetDeviceProcAddr" => pfn!(get_device_proc_addr, vk::PFN_vkGetDeviceProcAddr),
        b"vkEnumeratePhysicalDevices" => pfn!(
            enumerate_physical_devices,
            vk::PFN_vkEnumeratePhysicalDevices
        ),
        b"vkEnumerateDeviceExtensionProperties" => pfn!(
            enumerate_device_extension_properties,
            vk::PFN_vkEnumerateDeviceExtensionProperties
        ),
        b"vkGetPhysicalDeviceQueueFamilyProperties" => pfn!(
            get_physical_device_queue_family_properties,
            vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties
        ),
        b"vkGetPhysicalDeviceQueueFamilyProperties2" => pfn!(
            get_physical_device_queue_family_properties2,
            vk::PFN_vkGetPhysicalDeviceQueueFamilyProperties2
        ),
        b"vkGetPhysicalDeviceSurfaceSupportKHR" => pfn!(
            get_physical_device_surface_support_khr,
            vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR
        ),
        _ => None,
    }
}

unsafe extern "system" fn get_no_device_proc_addr(
    device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if !name.is_null() {
        record_request(device.as_raw_u64(), unsafe { CStr::from_ptr(name) });
    }
    None
}

unsafe extern "system" fn get_no_instance_proc_addr(
    instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    if name.is_null() {
        return None;
    }
    let name = unsafe { CStr::from_ptr(name) };
    record_request(instance.as_raw_u64(), name);
    match name.to_bytes() {
        b"vkGetDeviceProcAddr" => pfn!(get_no_device_proc_addr, vk::PFN_vkGetDeviceProcAddr),
        _ => None,
    }
}

pub fn instance_proc_addr() -> vk::PFN_vkGetInstanceProcAddr {
    Some(get_instance_proc_addr)
}

pub fn device_proc_addr() -> vk::PFN_vkGetDeviceProcAddr {
    Some(get_device_proc_addr)
}

pub struct Fakes {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
}

pub fn fakes() -> &'static Fakes {
    static FAKES: OnceLock<Fakes> = OnceLock::new();
    FAKES.get_or_init(|| Fakes {
        entry: entry(),
        instance: instance(),
        device: device(),
    })
}

pub fn entry() -> Entry {
    unsafe { Entry::from_get_instance_proc_addr(instance_proc_addr()) }
}

pub fn instance() -> Instance {
    unsafe { Instance::load(&entry(), vk::Instance::from_raw_u64(INSTANCE)) }
}

pub fn device() -> Device {
    unsafe { Device::load(&instance(), vk::Device::from_raw_u64(DEVICE)) }
}

pub fn entry_without_commands() -> Entry {
    unsafe { Entry::from_get_instance_proc_addr(Some(get_no_instance_proc_addr)) }
}

pub fn instance_without_commands() -> Instance {
    unsafe {
        Instance::load(
            &entry_without_commands(),
            vk::Instance::from_raw_u64(INSTANCE),
        )
    }
}

pub fn device_without_commands() -> Device {
    unsafe {
        Device::load(
            &instance_without_commands(),
            vk::Device::from_raw_u64(DEVICE),
        )
    }
}
