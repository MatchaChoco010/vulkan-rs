use anyhow::{Result, bail};
use vulkan::{Entry, vk};
use vulkan_example_common::device::{Gpu, find_queue_family, pick_physical_device};
use vulkan_example_common::instance::{create_instance, destroy_instance};

/// A Vulkan 1.1 device with a graphics queue.
pub unsafe fn create_gpu() -> Result<Gpu> {
    unsafe {
        let entry = Entry::load()?;
        // The shaders are SPIR-V 1.3, which needs Vulkan 1.1.
        let api_version = vk::API_VERSION_1_1;
        let (instance, messenger) =
            create_instance(&entry, c"headless-triangle", api_version, &[])?;

        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < api_version {
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
            bail!("no Vulkan 1.1 physical device supports graphics");
        };

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let device = instance.create_device(physical_device, &device_create_info(&queues), None)?;
        Ok(Gpu::new(
            instance,
            messenger,
            physical_device,
            device,
            queue_family,
        ))
    }
}

/// The device has `queues` and no optional features: the shaders declare no SPIR-V
/// capability other than `Shader`.
fn device_create_info<'a>(queues: &'a [vk::DeviceQueueCreateInfo<'a>]) -> vk::DeviceCreateInfo<'a> {
    vk::DeviceCreateInfo::default().queue_create_infos(queues)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::{FRAGMENT_SHADER, VERTEX_SHADER};
    use std::collections::BTreeSet;
    use vulkan_example_common::device::enables_shader_draw_parameters;
    use vulkan_example_common::shader::{
        CAPABILITY_DRAW_PARAMETERS, CAPABILITY_SHADER, spirv_capabilities,
    };

    #[test]
    fn the_device_enables_what_the_shaders_declare_and_nothing_more() {
        let queues = [vk::DeviceQueueCreateInfo::default()];
        let info = device_create_info(&queues);
        let draw_parameters_enabled = unsafe { enables_shader_draw_parameters(&info) };
        let mut supported = BTreeSet::from([CAPABILITY_SHADER]);
        if draw_parameters_enabled {
            supported.insert(CAPABILITY_DRAW_PARAMETERS);
        }

        let mut declared = BTreeSet::new();
        for (name, code) in [("vertex", VERTEX_SHADER), ("fragment", FRAGMENT_SHADER)] {
            let capabilities = spirv_capabilities(code).unwrap();
            assert!(
                capabilities.is_subset(&supported),
                "the {name} shader declares {capabilities:?}, the device supports {supported:?}"
            );
            declared.extend(capabilities);
        }
        assert_eq!(
            draw_parameters_enabled,
            declared.contains(&CAPABILITY_DRAW_PARAMETERS),
            "shaderDrawParameters is enabled exactly when a shader declares DrawParameters"
        );
    }
}
