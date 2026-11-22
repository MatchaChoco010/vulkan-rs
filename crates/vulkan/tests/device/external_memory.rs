use crate::gpu::Gpu;
use vulkan::vk;

#[test]
fn memory_round_trips_through_a_file_descriptor() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_MEMORY_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_MEMORY_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_memory_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;

        let external_info = vk::PhysicalDeviceExternalBufferInfo::default()
            .usage(usage)
            .handle_type(handle_type);
        let mut external_properties = vk::ExternalBufferProperties::default();
        gpu.instance.get_physical_device_external_buffer_properties(
            gpu.physical_device,
            &external_info,
            &mut external_properties,
        );
        let features = external_properties
            .external_memory_properties
            .external_memory_features;
        if !features.contains(
            vk::ExternalMemoryFeatureFlags::EXPORTABLE | vk::ExternalMemoryFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: opaque fd buffers are not exportable and importable");
            return;
        }

        let create_buffer = || {
            let mut external =
                vk::ExternalMemoryBufferCreateInfo::default().handle_types(handle_type.into());
            let info = vk::BufferCreateInfo::default()
                .size(4096)
                .usage(usage)
                .push_next(&mut external);
            device.create_buffer(&info, None).unwrap()
        };

        let exported_buffer = create_buffer();
        let requirements = device.get_buffer_memory_requirements(exported_buffer);
        let memory_type = gpu
            .memory_type(
                requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
            .unwrap();
        let mut export = vk::ExportMemoryAllocateInfo::default().handle_types(handle_type.into());
        let exported_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type)
                    .push_next(&mut export),
                None,
            )
            .unwrap();
        device
            .bind_buffer_memory(exported_buffer, exported_memory, 0)
            .unwrap();

        let fd = device
            .get_memory_fd_khr(
                &vk::MemoryGetFdInfoKHR::default()
                    .memory(exported_memory)
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= 0);

        let imported_buffer = create_buffer();
        let mut import = vk::ImportMemoryFdInfoKHR::default()
            .handle_type(handle_type)
            .fd(fd);
        let imported_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type)
                    .push_next(&mut import),
                None,
            )
            .unwrap();
        device
            .bind_buffer_memory(imported_buffer, imported_memory, 0)
            .unwrap();

        let written = device
            .map_memory(exported_memory, 0, 4096, vk::MemoryMapFlags::empty())
            .unwrap()
            .cast::<u8>();
        let read = device
            .map_memory(imported_memory, 0, 4096, vk::MemoryMapFlags::empty())
            .unwrap()
            .cast::<u8>();
        for i in 0..4096 {
            written.add(i).write((i % 251) as u8);
        }
        for i in 0..4096 {
            assert_eq!(read.add(i).read(), (i % 251) as u8, "byte {i}");
        }
        device.unmap_memory(exported_memory);
        device.unmap_memory(imported_memory);

        device.destroy_buffer(imported_buffer, None);
        device.free_memory(imported_memory, None);
        device.destroy_buffer(exported_buffer, None);
        device.free_memory(exported_memory, None);
    }
}

#[test]
fn drm_format_modifier_list_is_filled_through_p_next() {
    let Some(gpu) = Gpu::new(&[vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME]) else {
        return;
    };
    if !gpu.has(vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME) {
        eprintln!("skipping: VK_EXT_image_drm_format_modifier is not supported");
        return;
    }
    unsafe {
        let format = vk::Format::B8G8R8A8_UNORM;

        let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let count = list.drm_format_modifier_count as usize;
        assert!(count > 0, "no DRM format modifier for {format:?}");

        let mut modifiers = vec![vk::DrmFormatModifierPropertiesEXT::default(); count];
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default()
            .drm_format_modifier_properties(&mut modifiers);
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let written = list.drm_format_modifier_count as usize;
        assert_eq!(written, count);
        for modifier in &modifiers[..written] {
            assert!(modifier.drm_format_modifier_plane_count >= 1);
            assert!(!modifier.drm_format_modifier_tiling_features.is_empty());
        }
    }
}
