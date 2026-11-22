use anyhow::{Result, anyhow};
use std::ffi::CStr;
use vulkan::{Entry, vk};
use vulkan_example_common::device::{
    Gpu, find_queue_family, pick_physical_device, supports_extensions,
};
use vulkan_example_common::extensions::has_extension;
use vulkan_example_common::instance::{create_instance, destroy_instance};

const DEVICE_EXTENSIONS: [&CStr; 4] = [
    vk::KHR_EXTERNAL_MEMORY_FD_NAME,
    vk::EXT_EXTERNAL_MEMORY_DMA_BUF_NAME,
    vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME,
    vk::EXT_QUEUE_FAMILY_FOREIGN_NAME,
];

/// A Vulkan 1.3 device with a graphics queue and every extension the round trip needs.
pub unsafe fn create_gpu() -> Result<Gpu> {
    unsafe {
        let entry = Entry::load()?;
        let (instance, messenger) =
            create_instance(&entry, c"dmabuf-roundtrip", vk::API_VERSION_1_3, &[])?;

        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < vk::API_VERSION_1_3
                || !supports_extensions(&instance, physical_device, &DEVICE_EXTENSIONS)?
            {
                return Ok(None);
            }
            Ok(find_queue_family(
                &instance,
                physical_device,
                vk::QueueFlags::GRAPHICS,
            ))
        })?;
        let Some((physical_device, queue_family)) = selected else {
            destroy_instance(instance, messenger);
            return Err(anyhow!(
                "no Vulkan 1.3 device supports dma-buf import and export with modifiers"
            ));
        };
        let properties = instance.get_physical_device_properties(physical_device);
        println!(
            "using {}",
            properties.device_name_as_c_str()?.to_string_lossy()
        );

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = DEVICE_EXTENSIONS.map(vk::CStrPtr::new);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut vulkan13),
            None,
        )?;
        Ok(Gpu::new(
            instance,
            messenger,
            physical_device,
            device,
            queue_family,
        ))
    }
}

/// Prints the DRM device nodes of the physical device, which is how a compositor matches
/// a Vulkan device to the KMS device it drives.
pub unsafe fn print_drm_nodes(gpu: &Gpu) -> Result<()> {
    unsafe {
        let extensions = gpu
            .instance
            .enumerate_device_extension_properties(gpu.physical_device, None)?;
        if !has_extension(&extensions, vk::EXT_PHYSICAL_DEVICE_DRM_NAME) {
            println!("VK_EXT_physical_device_drm is not supported");
            return Ok(());
        }
        let mut drm = vk::PhysicalDeviceDrmPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut drm);
        gpu.instance
            .get_physical_device_properties2(gpu.physical_device, &mut properties);
        if drm.has_primary != vk::FALSE {
            println!(
                "DRM primary node {}:{}",
                drm.primary_major, drm.primary_minor
            );
        }
        if drm.has_render != vk::FALSE {
            println!("DRM render node {}:{}", drm.render_major, drm.render_minor);
        }
        Ok(())
    }
}
