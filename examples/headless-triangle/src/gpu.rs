use anyhow::{Result, bail};
use vulkan::{Entry, Instance, vk};
use vulkan_example_common::device::{Gpu, find_queue_family, pick_physical_device};
use vulkan_example_common::instance::{create_instance, destroy_instance};

/// A Vulkan 1.1 device with a graphics queue and `shaderDrawParameters`.
pub unsafe fn create_gpu() -> Result<Gpu> {
    unsafe {
        let entry = Entry::load()?;
        // The shaders are SPIR-V 1.3, which needs Vulkan 1.1.
        let (instance, messenger) =
            create_instance(&entry, c"headless-triangle", vk::API_VERSION_1_1, &[])?;

        let selected = pick_physical_device(&instance, |physical_device, _| {
            if !supports_shader_draw_parameters(&instance, physical_device) {
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
            bail!("no physical device supports graphics and shaderDrawParameters");
        };

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let mut draw_parameters =
            vk::PhysicalDeviceShaderDrawParametersFeatures::default().shader_draw_parameters(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .push_next(&mut draw_parameters),
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

unsafe fn supports_shader_draw_parameters(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    // `VkPhysicalDeviceVulkan11Features` holds the same bit but was only added in
    // Vulkan 1.2, so with a 1.1 instance this struct is the one to use.
    let mut draw_parameters = vk::PhysicalDeviceShaderDrawParametersFeatures::default();
    let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut draw_parameters);
    unsafe { instance.get_physical_device_features2(physical_device, &mut features) };
    draw_parameters.shader_draw_parameters == vk::TRUE
}
