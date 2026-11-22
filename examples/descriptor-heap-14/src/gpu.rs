use anyhow::Result;
use vulkan::{Entry, vk};
use vulkan_example_common::device::{
    Gpu, find_queue_family, pick_physical_device, supports_extensions,
};
use vulkan_example_common::instance::create_instance;

/// What the example needs to know about the device to lay out its buffers.
pub struct Properties {
    pub heap: vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>,
    /// What the address in a storage buffer descriptor has to be a multiple of.
    pub storage_buffer_alignment: u64,
}

/// A Vulkan 1.4 device with a graphics queue, descriptor heaps and untyped pointers.
/// `None` when there is no such device.
pub unsafe fn create_gpu() -> Result<Option<(Gpu, Properties)>> {
    unsafe {
        let entry = Entry::load()?;
        let api_version = vk::API_VERSION_1_4;
        let instance = create_instance(&entry, c"descriptor-heap-14", api_version, &[])?;

        // In SPIR-V a heap is an untyped variable, so a shader that indexes the heaps
        // itself, as the fragment shader does, needs untyped pointers.
        let device_extensions = [
            vk::EXT_DESCRIPTOR_HEAP_NAME,
            vk::KHR_SHADER_UNTYPED_POINTERS_NAME,
        ];
        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < api_version
                || !supports_extensions(&instance, physical_device, &device_extensions)?
            {
                return Ok(None);
            }
            let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
            let mut untyped_pointers =
                vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default();
            let mut features = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut heap)
                .push_next(&mut untyped_pointers);
            instance.get_physical_device_features2(physical_device, &mut features);
            if heap.descriptor_heap == vk::FALSE
                || untyped_pointers.shader_untyped_pointers == vk::FALSE
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
            return Ok(None);
        };
        let properties = instance.get_physical_device_properties(physical_device);

        let mut heap_properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut properties2 =
            vk::PhysicalDeviceProperties2::default().push_next(&mut heap_properties);
        instance.get_physical_device_properties2(physical_device, &mut properties2);
        // Detach the copy from the chain it was filled through.
        heap_properties.p_next = std::ptr::null_mut();

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = device_extensions.map(vk::CStrPtr::new);
        let mut heap_features =
            vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
        let mut untyped_pointer_features =
            vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default()
                .shader_untyped_pointers(true);
        // Core since 1.2 and 1.3; a Vulkan 1.4 device has all three.
        let mut vulkan12 =
            vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .synchronization2(true)
            .dynamic_rendering(true);
        let gpu = Gpu::create(
            instance,
            physical_device,
            queue_family,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut heap_features)
                .push_next(&mut untyped_pointer_features)
                .push_next(&mut vulkan12)
                .push_next(&mut vulkan13),
        )?;
        Ok(Some((
            gpu,
            Properties {
                heap: heap_properties,
                storage_buffer_alignment: properties.limits.min_storage_buffer_offset_alignment,
            },
        )))
    }
}
