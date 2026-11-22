use anyhow::{Result, bail};
use std::ffi::CStr;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_example_common::device::{Gpu, pick_physical_device, supports_extensions};
use vulkan_example_common::extensions::has_extension;
use vulkan_example_common::instance::{create_instance, destroy_instance};
use vulkan_example_common::window::WaylandWindow;
use vulkan_example_common::window::surface::{
    WAYLAND_SURFACE_EXTENSIONS, create_wayland_surface, find_present_queue_family,
};

/// The optional extensions that were found and enabled.
pub struct Extensions {
    /// `VK_KHR_get_surface_capabilities2` and `VK_EXT_surface_maintenance1` on the
    /// instance.
    pub surface_maintenance1: bool,
    /// `VK_EXT_hdr_metadata` on the device.
    pub hdr_metadata: bool,
}

/// A Vulkan 1.3 device with dynamic rendering and a queue that draws and presents to a
/// surface of `window`, and that surface.
pub unsafe fn create_gpu(window: WaylandWindow) -> Result<(Gpu, vk::SurfaceKHR, Extensions)> {
    unsafe {
        let entry = Entry::load()?;
        let available = entry.enumerate_instance_extension_properties(None)?;
        let mut instance_extensions = WAYLAND_SURFACE_EXTENSIONS.to_vec();
        // Color spaces beyond sRGB, and the queries that tell which present modes a
        // swapchain can switch between.
        for optional in [
            vk::EXT_SWAPCHAIN_COLOR_SPACE_NAME,
            vk::KHR_GET_SURFACE_CAPABILITIES_2_NAME,
            vk::EXT_SURFACE_MAINTENANCE_1_NAME,
        ] {
            if has_extension(&available, optional) {
                instance_extensions.push(optional);
            }
        }
        let surface_maintenance1 = instance_extensions
            .contains(&vk::KHR_GET_SURFACE_CAPABILITIES_2_NAME)
            && instance_extensions.contains(&vk::EXT_SURFACE_MAINTENANCE_1_NAME);
        // Dynamic rendering is core in Vulkan 1.3; the device is used without
        // VK_KHR_dynamic_rendering, so it has to be a 1.3 device too.
        let api_version = vk::API_VERSION_1_3;
        let (instance, messenger) = create_instance(
            &entry,
            c"vulkan-wayland-dynamic-rendering",
            api_version,
            &instance_extensions,
        )?;
        let surface = create_wayland_surface(&instance, window)?;

        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < api_version
                || !supports_extensions(&instance, physical_device, &[vk::KHR_SWAPCHAIN_NAME])?
            {
                return Ok(None);
            }
            let Some(queue_family) =
                find_present_queue_family(&instance, physical_device, surface)?
            else {
                return Ok(None);
            };
            if !supports_dynamic_rendering(&instance, physical_device) {
                return Ok(None);
            }
            Ok(Some(queue_family))
        })?;
        let Some((physical_device, queue_family)) = selected else {
            instance.destroy_surface_khr(surface, None);
            destroy_instance(instance, messenger);
            bail!(
                "no physical device supports graphics, present, swapchain, and required features"
            );
        };

        let hdr_metadata =
            supports_extensions(&instance, physical_device, &[vk::EXT_HDR_METADATA_NAME])?;
        let mut device_extensions = vec![vk::KHR_SWAPCHAIN_NAME];
        if hdr_metadata {
            device_extensions.push(vk::EXT_HDR_METADATA_NAME);
        }
        let device = create_device(&instance, physical_device, queue_family, &device_extensions)?;
        let gpu = Gpu::new(instance, messenger, physical_device, device, queue_family);
        let extensions = Extensions {
            surface_maintenance1,
            hdr_metadata,
        };
        Ok((gpu, surface, extensions))
    }
}

unsafe fn supports_dynamic_rendering(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let mut dynamic_rendering = vk::PhysicalDeviceDynamicRenderingFeatures::default();
    let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut dynamic_rendering);
    unsafe { instance.get_physical_device_features2(physical_device, &mut features) };
    dynamic_rendering.dynamic_rendering != vk::FALSE
}

unsafe fn create_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    extensions: &[&CStr],
) -> Result<Device> {
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let extension_names = extensions
        .iter()
        .map(|name| vk::CStrPtr::new(name))
        .collect::<Vec<_>>();
    let mut dynamic_rendering =
        vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extension_names)
        .push_next(&mut dynamic_rendering);
    Ok(unsafe { instance.create_device(physical_device, &info, None) }?)
}
