use anyhow::{Context, Result};
use vulkan::{Entry, vk};
use vulkan_example_common::device::{
    Gpu, find_queue_family, pick_physical_device, supports_extensions,
};
use vulkan_example_common::instance::create_instance;

pub type HeapProperties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>;

/// A Vulkan 1.4 device with a compute queue, descriptor heaps and buffer device
/// addresses, and the properties that lay out its descriptor heaps. `None` when there is
/// no such device.
pub unsafe fn create_gpu() -> Result<Option<(Gpu, HeapProperties)>> {
    unsafe {
        let entry = Entry::load()?;

        // With Vulkan 1.4 in use, VK_EXT_descriptor_heap needs no other extension. The
        // registry data in the bindings can say so.
        let api_version = vk::API_VERSION_1_4;
        let heap_extension = vk::ExtensionInfo::get(vk::EXT_DESCRIPTOR_HEAP_NAME)
            .context("VK_EXT_descriptor_heap is in the registry")?;
        debug_assert!(heap_extension.dependencies_met(api_version, |_| false));

        let instance = create_instance(&entry, c"compute-buffer", api_version, &[])?;
        let selected = pick_physical_device(&instance, |physical_device, properties| {
            if properties.api_version < api_version
                || !supports_extensions(
                    &instance,
                    physical_device,
                    &[vk::EXT_DESCRIPTOR_HEAP_NAME],
                )?
            {
                return Ok(None);
            }
            let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
            let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut heap)
                .push_next(&mut vulkan12);
            instance.get_physical_device_features2(physical_device, &mut features);
            if heap.descriptor_heap == vk::FALSE || vulkan12.buffer_device_address == vk::FALSE {
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
        println!(
            "using {}",
            properties.device_name_as_c_str()?.to_string_lossy()
        );

        let mut heap_properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut properties =
            vk::PhysicalDeviceProperties2::default().push_next(&mut heap_properties);
        instance.get_physical_device_properties2(physical_device, &mut properties);
        // Detach the copy from the chain it was filled through.
        heap_properties.p_next = std::ptr::null_mut();

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = [vk::CStrPtr::new(vk::EXT_DESCRIPTOR_HEAP_NAME)];
        let mut heap_features =
            vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
        let mut vulkan12 =
            vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default().synchronization2(true);
        let gpu = Gpu::create(
            instance,
            physical_device,
            queue_family,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut heap_features)
                .push_next(&mut vulkan12)
                .push_next(&mut vulkan13),
        )?;
        Ok(Some((gpu, heap_properties)))
    }
}
