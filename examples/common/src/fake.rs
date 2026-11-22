//! A fake Vulkan implementation for the tests of the ownership of objects: it records
//! every object that is created and destroyed, checks the order in which they are
//! destroyed, and can make the n-th call that returns a `VkResult` fail.

use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_void};
use vulkan::{Entry, vk};

/// What the fake knows about a live object.
#[derive(Clone, Debug)]
struct Object {
    kind: vk::StructureType,
    /// The instance or the device it was created from.
    parent: u64,
    /// The image of an image view, the surface of a swapchain.
    target: u64,
}

#[derive(Default)]
struct State {
    next_handle: u64,
    live: HashMap<u64, Object>,
    created: usize,
    /// The images of each swapchain.
    swapchain_images: HashMap<u64, Vec<u64>>,
    mapped: HashMap<u64, Box<[u8]>>,
    calls: u32,
    fail_at: Option<u32>,
    failed: bool,
    misuse: Vec<String>,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::default();
}

fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.with(|state| f(&mut state.borrow_mut()))
}

/// What a run through the fake left behind.
#[derive(Debug)]
pub struct Report {
    /// Whether the call that was told to fail was reached.
    pub failed: bool,
    /// How many objects were created.
    pub created: usize,
    /// The objects that were not destroyed.
    pub leaked: Vec<String>,
    /// Destructions in a wrong order, of unknown handles, or twice.
    pub misuse: Vec<String>,
}

/// Starts a run: forgets every object, and makes the `fail_at`-th call that returns a
/// `VkResult`, counted from 0, fail.
pub fn reset(fail_at: Option<u32>) {
    with(|state| {
        *state = State {
            fail_at,
            next_handle: 0x1000,
            ..State::default()
        }
    });
}

/// What happened since [`reset`].
pub fn report() -> Report {
    with(|state| Report {
        failed: state.failed,
        created: state.created,
        leaked: state
            .live
            .iter()
            .map(|(handle, object)| format!("{:?} {handle:#x}", object.kind))
            .collect(),
        misuse: state.misuse.clone(),
    })
}

/// An entry whose commands are those of the fake.
pub fn entry() -> Entry {
    // SAFETY: `get_instance_proc_addr` behaves as `vkGetInstanceProcAddr` does, and the
    // functions it returns live as long as the program.
    unsafe { Entry::from_get_instance_proc_addr(Some(get_instance_proc_addr)) }
}

/// Counts a call that can fail, and tells whether it fails.
fn fails() -> bool {
    with(|state| {
        let call = state.calls;
        state.calls += 1;
        if state.fail_at == Some(call) {
            state.failed = true;
            true
        } else {
            false
        }
    })
}

fn create_object(kind: vk::StructureType, parent: u64, target: u64) -> u64 {
    with(|state| {
        state.next_handle += 1;
        let handle = state.next_handle;
        state.live.insert(
            handle,
            Object {
                kind,
                parent,
                target,
            },
        );
        state.created += 1;
        handle
    })
}

fn destroy_object(handle: u64, what: &str) {
    with(|state| {
        let Some(object) = state.live.remove(&handle) else {
            state
                .misuse
                .push(format!("{what} of {handle:#x}, which is not alive"));
            return;
        };
        let mut children = state
            .live
            .iter()
            .filter(|(_, child)| child.parent == handle || child.target == handle)
            .map(|(child, info)| format!("{:?} {child:#x}", info.kind))
            .collect::<Vec<_>>();
        if let Some(images) = state.swapchain_images.remove(&handle) {
            children.extend(
                state
                    .live
                    .iter()
                    .filter(|(_, child)| images.contains(&child.target))
                    .map(|(child, info)| format!("{:?} {child:#x}", info.kind)),
            );
        }
        if !children.is_empty() {
            state.misuse.push(format!(
                "{what} of {:?} {handle:#x} while {} is alive",
                object.kind,
                children.join(", ")
            ));
        }
        state.mapped.remove(&handle);
    });
}

type Void = unsafe extern "system" fn();

macro_rules! commands {
    ($name:expr, $($command:literal => $function:expr,)*) => {
        match $name {
            $($command => Some(unsafe { std::mem::transmute::<*const (), Void>($function as *const ()) }),)*
            _ => None,
        }
    };
}

unsafe extern "system" fn get_instance_proc_addr(
    _instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    let name = unsafe { CStr::from_ptr(name) }.to_bytes();
    commands!(name,
        b"vkEnumerateInstanceLayerProperties" => enumerate_instance_layer_properties,
        b"vkEnumerateInstanceExtensionProperties" => enumerate_instance_extension_properties,
        b"vkCreateInstance" => create_instance,
        b"vkDestroyInstance" => destroy_dispatchable,
        b"vkCreateDebugUtilsMessengerEXT" => create,
        b"vkDestroyDebugUtilsMessengerEXT" => destroy,
        b"vkCreateWaylandSurfaceKHR" => create,
        b"vkDestroySurfaceKHR" => destroy,
        b"vkEnumeratePhysicalDevices" => enumerate_physical_devices,
        b"vkGetPhysicalDeviceProperties" => get_physical_device_properties,
        b"vkGetPhysicalDeviceMemoryProperties" => get_physical_device_memory_properties,
        b"vkGetPhysicalDeviceQueueFamilyProperties" => get_physical_device_queue_family_properties,
        b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR" => get_physical_device_surface_capabilities,
        b"vkCreateDevice" => create,
        b"vkGetDeviceProcAddr" => get_device_proc_addr,
    )
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    let name = unsafe { CStr::from_ptr(name) }.to_bytes();
    commands!(name,
        b"vkDestroyDevice" => destroy_dispatchable,
        b"vkGetDeviceQueue" => get_device_queue,
        b"vkDeviceWaitIdle" => succeed,
        b"vkCreateBuffer" => create,
        b"vkDestroyBuffer" => destroy,
        b"vkCreateImage" => create,
        b"vkDestroyImage" => destroy,
        b"vkCreateImageView" => create,
        b"vkDestroyImageView" => destroy,
        b"vkAllocateMemory" => create,
        b"vkFreeMemory" => destroy,
        b"vkCreateShaderModule" => create,
        b"vkDestroyShaderModule" => destroy,
        b"vkCreatePipelineLayout" => create,
        b"vkDestroyPipelineLayout" => destroy,
        b"vkCreateCommandPool" => create,
        b"vkDestroyCommandPool" => destroy,
        b"vkCreateFence" => create,
        b"vkDestroyFence" => destroy,
        b"vkCreateSemaphore" => create,
        b"vkDestroySemaphore" => destroy,
        b"vkCreateSwapchainKHR" => create,
        b"vkDestroySwapchainKHR" => destroy,
        b"vkDestroyPipeline" => destroy,
        b"vkCreateGraphicsPipelines" => create_graphics_pipelines,
        b"vkGetSwapchainImagesKHR" => get_swapchain_images,
        b"vkGetBufferMemoryRequirements" => get_memory_requirements,
        b"vkGetImageMemoryRequirements" => get_memory_requirements,
        b"vkBindBufferMemory" => bind_memory,
        b"vkBindImageMemory" => bind_memory,
        b"vkMapMemory" => map_memory,
        b"vkUnmapMemory" => unmap_memory,
        b"vkAllocateCommandBuffers" => allocate_command_buffers,
        b"vkBeginCommandBuffer" => fallible_with_pointer,
        b"vkEndCommandBuffer" => fallible,
        b"vkQueueSubmit" => queue_submit,
        b"vkWaitForFences" => wait_for_fences,
        b"vkCmdCopyBuffer" => cmd_copy_buffer,
        b"vkCmdPipelineBarrier" => cmd_pipeline_barrier,
    )
}

fn result(fails: bool) -> vk::VkResult {
    if fails {
        vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY
    } else {
        vk::VkResult::SUCCESS
    }
}

unsafe extern "system" fn enumerate_instance_layer_properties(
    count: *mut u32,
    properties: *mut vk::LayerProperties,
) -> vk::VkResult {
    unsafe { enumerate(count, properties, &[layer(c"VK_LAYER_KHRONOS_validation")]) }
}

unsafe extern "system" fn enumerate_instance_extension_properties(
    _layer: *const c_char,
    count: *mut u32,
    properties: *mut vk::ExtensionProperties,
) -> vk::VkResult {
    unsafe { enumerate(count, properties, &[extension(vk::EXT_DEBUG_UTILS_NAME)]) }
}

fn layer(name: &CStr) -> vk::LayerProperties {
    let mut layer = vk::LayerProperties::default();
    for (to, &from) in layer.layer_name.iter_mut().zip(name.to_bytes()) {
        *to = from as c_char;
    }
    layer
}

fn extension(name: &CStr) -> vk::ExtensionProperties {
    let mut extension = vk::ExtensionProperties::default();
    for (to, &from) in extension.extension_name.iter_mut().zip(name.to_bytes()) {
        *to = from as c_char;
    }
    extension
}

/// The two-call idiom of the `vkEnumerate*` and `vkGet*` commands that return arrays.
unsafe fn enumerate<T: Copy>(count: *mut u32, out: *mut T, values: &[T]) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    unsafe {
        if out.is_null() {
            *count = values.len() as u32;
            return vk::VkResult::SUCCESS;
        }
        let written = (*count as usize).min(values.len());
        std::ptr::copy_nonoverlapping(values.as_ptr(), out, written);
        *count = written as u32;
        if written < values.len() {
            vk::VkResult::INCOMPLETE
        } else {
            vk::VkResult::SUCCESS
        }
    }
}

unsafe extern "system" fn create_instance(
    _info: *const c_void,
    _allocator: *const c_void,
    instance: *mut u64,
) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    let handle = create_object(vk::StructureType::INSTANCE_CREATE_INFO, 0, 0);
    unsafe { *instance = handle };
    vk::VkResult::SUCCESS
}

/// `vkDestroyInstance` and `vkDestroyDevice`.
unsafe extern "system" fn destroy_dispatchable(handle: u64, _allocator: *const c_void) {
    destroy_object(handle, "destroying the instance or device");
}

/// Every `vkCreate*` and `vkAllocateMemory`: (parent, info, allocator, handle).
unsafe extern "system" fn create(
    parent: u64,
    info: *const vk::BaseInStructure<'_>,
    _allocator: *const c_void,
    handle: *mut u64,
) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    let kind = unsafe { (*info).s_type };
    let target = match kind {
        vk::StructureType::IMAGE_VIEW_CREATE_INFO => unsafe {
            (*info.cast::<vk::ImageViewCreateInfo<'_>>()).image.as_raw()
        },
        vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR => unsafe {
            (*info.cast::<vk::SwapchainCreateInfoKHR<'_>>())
                .surface
                .as_raw()
        },
        _ => 0,
    };
    // A device is a child of the instance, not of the physical device it is given.
    let parent = if kind == vk::StructureType::DEVICE_CREATE_INFO {
        with(|state| {
            state
                .live
                .iter()
                .find(|(_, object)| object.kind == vk::StructureType::INSTANCE_CREATE_INFO)
                .map_or(0, |(&instance, _)| instance)
        })
    } else {
        parent
    };
    let created = create_object(kind, parent, target);
    if kind == vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR {
        let images = (0..3).map(|index| 0x10_0000 * created + index).collect();
        with(|state| state.swapchain_images.insert(created, images));
    }
    unsafe { *handle = created };
    vk::VkResult::SUCCESS
}

/// Every `vkDestroy*` and `vkFreeMemory`: (parent, handle, allocator).
unsafe extern "system" fn destroy(_parent: u64, handle: u64, _allocator: *const c_void) {
    if handle != 0 {
        destroy_object(handle, "destroying");
    }
}

unsafe extern "system" fn enumerate_physical_devices(
    _instance: u64,
    count: *mut u32,
    devices: *mut u64,
) -> vk::VkResult {
    unsafe { enumerate(count, devices, &[0x7]) }
}

unsafe extern "system" fn get_physical_device_properties(
    _physical_device: u64,
    properties: *mut vk::PhysicalDeviceProperties,
) {
    let mut value = vk::PhysicalDeviceProperties {
        api_version: vk::API_VERSION_1_4,
        device_type: vk::PhysicalDeviceType::INTEGRATED_GPU,
        ..Default::default()
    };
    value.limits.non_coherent_atom_size = 64;
    value.limits.buffer_image_granularity = 1;
    unsafe { *properties = value };
}

unsafe extern "system" fn get_physical_device_memory_properties(
    _physical_device: u64,
    properties: *mut vk::PhysicalDeviceMemoryProperties,
) {
    let mut value = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 2,
        memory_heap_count: 1,
        ..Default::default()
    };
    value.memory_heaps[0].size = 1 << 30;
    value.memory_types[0].property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
    value.memory_types[1].property_flags =
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
    unsafe { *properties = value };
}

unsafe extern "system" fn get_physical_device_queue_family_properties(
    _physical_device: u64,
    count: *mut u32,
    properties: *mut vk::QueueFamilyProperties,
) {
    let family = vk::QueueFamilyProperties {
        queue_flags: vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE,
        queue_count: 1,
        ..Default::default()
    };
    unsafe {
        if properties.is_null() {
            *count = 1;
        } else if *count >= 1 {
            *properties = family;
            *count = 1;
        }
    }
}

unsafe extern "system" fn get_physical_device_surface_capabilities(
    _physical_device: u64,
    _surface: u64,
    capabilities: *mut vk::SurfaceCapabilitiesKHR,
) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    let value = vk::SurfaceCapabilitiesKHR {
        min_image_count: 2,
        max_image_count: 3,
        current_extent: vk::Extent2D {
            width: 64,
            height: 64,
        },
        supported_composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        ..Default::default()
    };
    unsafe { *capabilities = value };
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn get_device_queue(
    _device: u64,
    _family: u32,
    _index: u32,
    queue: *mut u64,
) {
    unsafe { *queue = 0x9 };
}

unsafe extern "system" fn succeed(_handle: u64) -> vk::VkResult {
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn fallible(_handle: u64) -> vk::VkResult {
    result(fails())
}

unsafe extern "system" fn fallible_with_pointer(
    _handle: u64,
    _info: *const c_void,
) -> vk::VkResult {
    result(fails())
}

unsafe extern "system" fn cmd_copy_buffer(
    _command_buffer: u64,
    _source: u64,
    _destination: u64,
    _region_count: u32,
    _regions: *const c_void,
) {
}

#[allow(clippy::too_many_arguments)]
unsafe extern "system" fn cmd_pipeline_barrier(
    _command_buffer: u64,
    _source_stages: u32,
    _destination_stages: u32,
    _dependency_flags: u32,
    _memory_barrier_count: u32,
    _memory_barriers: *const c_void,
    _buffer_barrier_count: u32,
    _buffer_barriers: *const c_void,
    _image_barrier_count: u32,
    _image_barriers: *const c_void,
) {
}

unsafe extern "system" fn create_graphics_pipelines(
    device: u64,
    _cache: u64,
    count: u32,
    _infos: *const c_void,
    _allocator: *const c_void,
    pipelines: *mut u64,
) -> vk::VkResult {
    let failed = fails();
    for index in 0..count as usize {
        let handle = if failed {
            0
        } else {
            create_object(vk::StructureType::GRAPHICS_PIPELINE_CREATE_INFO, device, 0)
        };
        unsafe { *pipelines.add(index) = handle };
    }
    result(failed)
}

unsafe extern "system" fn get_swapchain_images(
    _device: u64,
    swapchain: u64,
    count: *mut u32,
    images: *mut u64,
) -> vk::VkResult {
    let values = with(|state| state.swapchain_images.get(&swapchain).cloned()).unwrap_or_default();
    unsafe { enumerate(count, images, &values) }
}

unsafe extern "system" fn get_memory_requirements(
    _device: u64,
    _resource: u64,
    requirements: *mut vk::MemoryRequirements,
) {
    unsafe {
        *requirements = vk::MemoryRequirements {
            size: 256,
            alignment: 16,
            memory_type_bits: 0b11,
        };
    }
}

unsafe extern "system" fn bind_memory(
    _device: u64,
    _resource: u64,
    _memory: u64,
    _offset: u64,
) -> vk::VkResult {
    result(fails())
}

unsafe extern "system" fn map_memory(
    _device: u64,
    memory: u64,
    _offset: u64,
    size: u64,
    _flags: u32,
    data: *mut *mut c_void,
) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    with(|state| {
        let bytes = state
            .mapped
            .entry(memory)
            .or_insert_with(|| vec![0; size as usize].into_boxed_slice());
        unsafe { *data = bytes.as_mut_ptr().cast() };
    });
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn unmap_memory(_device: u64, memory: u64) {
    with(|state| state.mapped.remove(&memory));
}

unsafe extern "system" fn allocate_command_buffers(
    _device: u64,
    info: *const vk::CommandBufferAllocateInfo<'_>,
    command_buffers: *mut u64,
) -> vk::VkResult {
    if fails() {
        return result(true);
    }
    for index in 0..unsafe { (*info).command_buffer_count } as usize {
        unsafe { *command_buffers.add(index) = 0xc0 + index as u64 };
    }
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn queue_submit(
    _queue: u64,
    _count: u32,
    _submits: *const c_void,
    _fence: u64,
) -> vk::VkResult {
    result(fails())
}

unsafe extern "system" fn wait_for_fences(
    _device: u64,
    _count: u32,
    _fences: *const u64,
    _wait_all: u32,
    _timeout: u64,
) -> vk::VkResult {
    result(fails())
}
