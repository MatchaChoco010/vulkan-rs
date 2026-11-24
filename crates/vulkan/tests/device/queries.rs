use crate::gpu::{Gpu, exclusive_access_to_the_loaders_device_list};
use vulkan::vk::{self, Handle};

#[test]
fn queries_fill_p_next_chains() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    unsafe {
        let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default();
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut vulkan12)
            .push_next(&mut vulkan13);
        gpu.instance
            .get_physical_device_features2(gpu.physical_device, &mut features);
        assert_eq!(vulkan12.timeline_semaphore, vk::TRUE);
        assert_eq!(vulkan13.dynamic_rendering, vk::TRUE);
        assert_eq!(vulkan13.synchronization2, vk::TRUE);

        let mut id = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
        gpu.instance
            .get_physical_device_properties2(gpu.physical_device, &mut properties);
        let core = properties.properties;
        assert!(core.api_version >= vk::API_VERSION_1_3);
        assert!(!core.device_name_as_c_str().unwrap().is_empty());
        assert_ne!(id.device_uuid, [0; vk::UUID_SIZE]);

        let families = gpu
            .instance
            .get_physical_device_queue_family_properties(gpu.physical_device);
        assert!(families[gpu.queue_family as usize].queue_count >= 1);
        assert!(!gpu.queue.is_null());
    }
}

#[test]
fn object_names_use_handle_object_types() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    if !gpu.has_debug_utils() {
        eprintln!("skipping: VK_EXT_debug_utils is not enabled");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(64)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();
        let name = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_type(vk::Buffer::TYPE)
            .object_handle(buffer.as_raw_u64())
            .object_name(c"named buffer");
        let serialized = exclusive_access_to_the_loaders_device_list();
        let named = device.set_debug_utils_object_name_ext(&name);
        drop(serialized);
        named.unwrap();
        device.destroy_buffer(buffer, None);
    }
}
