use anyhow::{Result, bail};
use std::ffi::CStr;
use vulkan::{Entry, vk};
use vulkan_example_common::device::{Gpu, pick_physical_device, supports_extensions};
use vulkan_example_common::instance::{VulkanInstance, create_instance};
use vulkan_example_common::window::WaylandWindow;
use vulkan_example_common::window::surface::{
    Surface, WAYLAND_SURFACE_EXTENSIONS, create_wayland_surface, find_present_queue_family,
};

const DEVICE_EXTENSIONS: [&CStr; 1] = [vk::KHR_SWAPCHAIN_NAME];

/// A Vulkan 1.1 device with a queue that draws and presents to a surface of `window`,
/// and that surface.
pub unsafe fn create_gpu(window: WaylandWindow) -> Result<(Gpu, Surface)> {
    unsafe {
        let entry = Entry::load()?;
        // The shaders are SPIR-V 1.3, which needs Vulkan 1.1.
        let instance = create_instance(
            &entry,
            c"vulkan-wayland-triangle",
            vk::API_VERSION_1_1,
            &WAYLAND_SURFACE_EXTENSIONS,
        )?;
        let surface = create_wayland_surface(&instance, window)?;

        let selected = pick_physical_device(&instance, |physical_device, _| {
            if !supports_extensions(&instance, physical_device, &DEVICE_EXTENSIONS)? {
                return Ok(None);
            }
            find_present_queue_family(&instance, physical_device, surface.handle)
        })?;
        let Some((physical_device, queue_family)) = selected else {
            bail!(
                "no physical device supports graphics, present, swapchain, and required features"
            );
        };

        let gpu = create_device(instance, physical_device, queue_family)?;
        Ok((gpu, surface))
    }
}

unsafe fn create_device(
    instance: VulkanInstance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
) -> Result<Gpu> {
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let extension_names = DEVICE_EXTENSIONS.map(vk::CStrPtr::new);
    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extension_names);
    unsafe { Gpu::create(instance, physical_device, queue_family, &info) }
}
