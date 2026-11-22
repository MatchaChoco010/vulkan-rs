use anyhow::Result;
use vulkan::{Entry, vk};
use vulkan_example_common::device::{
    Gpu, find_queue_family, pick_physical_device, supports_extensions,
};
use vulkan_example_common::instance::create_instance;

/// What the example needs to know about the device to lay out its buffers.
pub struct Properties {
    /// The size and alignment of the shader group handles in the shader binding table.
    pub ray_tracing: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'static>,
    /// What the scratch address of an acceleration structure build has to be a
    /// multiple of.
    pub scratch_alignment: u64,
    pub heap: vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>,
    /// What the address in a storage buffer descriptor has to be a multiple of.
    pub storage_buffer_alignment: u64,
}

/// A Vulkan 1.4 device with a compute queue, ray tracing pipelines, acceleration
/// structures, descriptor heaps and `shaderInt64`. `None` when there is no such device.
pub unsafe fn create_gpu() -> Result<Option<(Gpu, Properties)>> {
    unsafe {
        let entry = Entry::load()?;
        // With Vulkan 1.4 in use, VK_EXT_descriptor_heap needs no other extension.
        let api_version = vk::API_VERSION_1_4;
        let instance = create_instance(&entry, c"ray-tracing-pipeline", api_version, &[])?;

        // VK_KHR_acceleration_structure needs VK_KHR_deferred_host_operations even
        // though the example defers no operation.
        let device_extensions = [
            vk::KHR_DEFERRED_HOST_OPERATIONS_NAME,
            vk::KHR_ACCELERATION_STRUCTURE_NAME,
            vk::KHR_RAY_TRACING_PIPELINE_NAME,
            vk::EXT_DESCRIPTOR_HEAP_NAME,
        ];
        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < api_version
                || !supports_extensions(&instance, physical_device, &device_extensions)?
            {
                return Ok(None);
            }
            let mut acceleration_structure =
                vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
            let mut ray_tracing = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
            let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
            let mut features = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut acceleration_structure)
                .push_next(&mut ray_tracing)
                .push_next(&mut heap);
            instance.get_physical_device_features2(physical_device, &mut features);
            if features.features.shader_int64 == vk::FALSE
                || acceleration_structure.acceleration_structure == vk::FALSE
                || ray_tracing.ray_tracing_pipeline == vk::FALSE
                || heap.descriptor_heap == vk::FALSE
            {
                return Ok(None);
            }
            Ok(find_queue_family(
                &instance,
                physical_device,
                vk::QueueFlags::COMPUTE,
            ))
        })?;
        let Some((physical_device, queue_family)) = selected else {
            return Ok(None);
        };
        let properties = instance.get_physical_device_properties(physical_device);

        let mut ray_tracing_properties =
            vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default();
        let mut acceleration_structure_properties =
            vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
        let mut heap_properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut properties2 = vk::PhysicalDeviceProperties2::default()
            .push_next(&mut ray_tracing_properties)
            .push_next(&mut acceleration_structure_properties)
            .push_next(&mut heap_properties);
        instance.get_physical_device_properties2(physical_device, &mut properties2);
        // Detach the copies from the chain they were filled through.
        ray_tracing_properties.p_next = std::ptr::null_mut();
        heap_properties.p_next = std::ptr::null_mut();

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = device_extensions.map(vk::CStrPtr::new);
        // The ray generation shader reads the vertices through 64-bit addresses.
        let features = vk::PhysicalDeviceFeatures::default().shader_int64(true);
        let mut acceleration_structure_features =
            vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default()
                .acceleration_structure(true);
        let mut ray_tracing_features =
            vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default().ray_tracing_pipeline(true);
        let mut heap_features =
            vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
        // Core since 1.2; a Vulkan 1.4 device has it.
        let mut vulkan12 =
            vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
        let gpu = Gpu::create(
            instance,
            physical_device,
            queue_family,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .enabled_features(&features)
                .push_next(&mut acceleration_structure_features)
                .push_next(&mut ray_tracing_features)
                .push_next(&mut heap_features)
                .push_next(&mut vulkan12),
        )?;
        Ok(Some((
            gpu,
            Properties {
                ray_tracing: ray_tracing_properties,
                scratch_alignment: acceleration_structure_properties
                    .min_acceleration_structure_scratch_offset_alignment
                    .into(),
                heap: heap_properties,
                storage_buffer_alignment: properties.limits.min_storage_buffer_offset_alignment,
            },
        )))
    }
}
