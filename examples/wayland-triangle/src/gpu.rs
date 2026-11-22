use anyhow::{Result, bail};
use std::ffi::CStr;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_example_common::device::{Gpu, meets_requirements, pick_physical_device};
use vulkan_example_common::instance::{create_instance, destroy_instance};
use vulkan_example_common::window::WaylandWindow;
use vulkan_example_common::window::surface::{
    WAYLAND_SURFACE_EXTENSIONS, create_wayland_surface, find_present_queue_family,
};

/// The shaders are SPIR-V 1.3, which needs Vulkan 1.1.
const API_VERSION: u32 = vk::API_VERSION_1_1;
const DEVICE_EXTENSIONS: [&CStr; 1] = [vk::KHR_SWAPCHAIN_NAME];

/// A Vulkan 1.1 device with a queue that draws and presents to a surface of `window`,
/// and that surface.
pub unsafe fn create_gpu(window: WaylandWindow) -> Result<(Gpu, vk::SurfaceKHR)> {
    unsafe {
        let entry = Entry::load()?;
        let (instance, messenger) = create_instance(
            &entry,
            c"vulkan-wayland-triangle",
            API_VERSION,
            &WAYLAND_SURFACE_EXTENSIONS,
        )?;
        let surface = create_wayland_surface(&instance, window)?;

        let selected = pick_physical_device(&instance, |physical_device, properties| {
            let extensions =
                instance.enumerate_device_extension_properties(physical_device, None)?;
            let present_queue_family =
                find_present_queue_family(&instance, physical_device, surface)?;
            Ok(queue_family_for(
                properties.api_version,
                &extensions,
                present_queue_family,
            ))
        })?;
        let Some((physical_device, queue_family)) = selected else {
            instance.destroy_surface_khr(surface, None);
            destroy_instance(instance, messenger);
            bail!(
                "no Vulkan 1.1 physical device draws and presents to the surface with VK_KHR_swapchain"
            );
        };

        let device = create_device(&instance, physical_device, queue_family)?;
        let gpu = Gpu::new(instance, messenger, physical_device, device, queue_family);
        Ok((gpu, surface))
    }
}

/// The queue family to draw and present with, on a physical device that reports
/// `api_version` and the device extensions `extensions` and whose first queue family that
/// draws and presents to the surface is `present_queue_family`. `None` when the device
/// cannot run the example.
fn queue_family_for(
    api_version: u32,
    extensions: &[vk::ExtensionProperties],
    present_queue_family: Option<u32>,
) -> Option<u32> {
    present_queue_family
        .filter(|_| meets_requirements(api_version, extensions, API_VERSION, &DEVICE_EXTENSIONS))
}

unsafe fn create_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
) -> Result<Device> {
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let extension_names = DEVICE_EXTENSIONS.map(vk::CStrPtr::new);
    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extension_names);
    Ok(unsafe { instance.create_device(physical_device, &info, None) }?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_char;

    fn extension(name: &CStr) -> vk::ExtensionProperties {
        let mut extension = vk::ExtensionProperties::default();
        for (to, &from) in extension.extension_name.iter_mut().zip(name.to_bytes()) {
            *to = from as c_char;
        }
        extension
    }

    #[test]
    fn a_device_is_chosen_exactly_when_it_has_vulkan_1_1_the_swapchain_and_a_present_queue() {
        let versions = [
            vk::API_VERSION_1_0,
            vk::make_api_version(0, 1, 0, 300),
            vk::API_VERSION_1_1,
            vk::API_VERSION_1_3,
        ];
        let extension_lists = [
            vec![],
            vec![extension(vk::KHR_SWAPCHAIN_NAME)],
            vec![extension(vk::EXT_HDR_METADATA_NAME)],
            vec![
                extension(vk::EXT_HDR_METADATA_NAME),
                extension(vk::KHR_SWAPCHAIN_NAME),
            ],
        ];
        for api_version in versions {
            for extensions in &extension_lists {
                for present_queue_family in [None, Some(0), Some(2)] {
                    let has_swapchain = extensions.iter().any(|extension| {
                        extension.extension_name_as_c_str() == Ok(vk::KHR_SWAPCHAIN_NAME)
                    });
                    let expected = present_queue_family
                        .filter(|_| api_version >= vk::API_VERSION_1_1 && has_swapchain);
                    assert_eq!(
                        queue_family_for(api_version, extensions, present_queue_family),
                        expected,
                        "version {}.{}.{}, swapchain: {has_swapchain}, present: {present_queue_family:?}",
                        vk::api_version_major(api_version),
                        vk::api_version_minor(api_version),
                        vk::api_version_patch(api_version),
                    );
                }
            }
        }
    }
}
