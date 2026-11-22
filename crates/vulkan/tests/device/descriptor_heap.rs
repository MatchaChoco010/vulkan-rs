use crate::gpu::{Gpu, color_layers, color_range};
use vulkan::vk;

fn align_up(value: u64, alignment: u64) -> u64 {
    value.div_ceil(alignment) * alignment
}

fn spirv(bytes: &[u8]) -> Vec<u32> {
    bytes
        .chunks_exact(4)
        .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect()
}

#[test]
fn descriptor_heap_draws_through_resource_and_sampler_heaps() {
    let Some(gpu) = Gpu::new(&[
        vk::KHR_MAINTENANCE_5_NAME,
        vk::EXT_DESCRIPTOR_HEAP_NAME,
        vk::KHR_SHADER_UNTYPED_POINTERS_NAME,
    ]) else {
        return;
    };
    if !gpu.descriptor_heap {
        eprintln!("skipping: VK_EXT_descriptor_heap is not supported");
        return;
    }
    const SIZE: u32 = 16;
    const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
    const TEXEL: [u8; 4] = [255, 128, 0, 255];
    const TINT: [f32; 4] = [0.5, 1.0, 1.0, 1.0];

    unsafe {
        let device = &gpu.device;
        let host = vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;

        let mut heap_properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut properties =
            vk::PhysicalDeviceProperties2::default().push_next(&mut heap_properties);
        gpu.instance
            .get_physical_device_properties2(gpu.physical_device, &mut properties);
        let heap = heap_properties;

        let (texture, texture_memory) = gpu.image(
            FORMAT,
            1,
            1,
            vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
        );
        let (texel_buffer, texel_memory) =
            gpu.host_buffer(&TEXEL, vk::BufferUsageFlags::TRANSFER_SRC, false);
        let tint_bytes = TINT
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect::<Vec<_>>();
        let (params, params_memory) = gpu.host_buffer(
            &tint_bytes,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            true,
        );

        let buffer_size = heap.buffer_descriptor_size;
        let image_size = heap.image_descriptor_size;
        let sampler_size = heap.sampler_descriptor_size;
        let params_index = 0u32;
        let texture_index = 1u32.max(buffer_size.div_ceil(image_size) as u32);
        let texture_offset = u64::from(texture_index) * image_size;
        assert_eq!(texture_offset % heap.image_descriptor_alignment.max(1), 0);
        let resource_reserved = align_up(
            texture_offset + image_size,
            heap.resource_heap_alignment.max(1),
        );
        let resource_heap_size = align_up(
            resource_reserved + heap.min_resource_heap_reserved_range,
            heap.resource_heap_alignment.max(1),
        );
        let sampler_index = 0u32;
        let sampler_reserved = align_up(sampler_size, heap.sampler_heap_alignment.max(1));
        let sampler_heap_size = align_up(
            sampler_reserved + heap.min_sampler_heap_reserved_range,
            heap.sampler_heap_alignment.max(1),
        );

        let heap_usage =
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS;
        let (resource_heap, resource_heap_memory) =
            gpu.buffer(resource_heap_size, heap_usage, host, true);
        let (sampler_heap, sampler_heap_memory) =
            gpu.buffer(sampler_heap_size, heap_usage, host, true);
        let address = |buffer| {
            device.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer))
        };

        let resource_bytes = std::slice::from_raw_parts_mut(
            device
                .map_memory(
                    resource_heap_memory,
                    0,
                    resource_heap_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap()
                .cast::<u8>(),
            resource_heap_size as usize,
        );
        let params_range = vk::DeviceAddressRangeKHR::default()
            .address(address(params))
            .size(tint_bytes.len() as u64);
        let texture_view = vk::ImageViewCreateInfo::default()
            .image(texture)
            .view_type(vk::ImageViewType::_2D)
            .format(FORMAT)
            .subresource_range(color_range());
        let texture_info = vk::ImageDescriptorInfoEXT::default()
            .view(&texture_view)
            .layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
        let resources = [
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::STORAGE_BUFFER)
                .data(vk::ResourceDescriptorDataEXT::address_range(&params_range)),
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::SAMPLED_IMAGE)
                .data(vk::ResourceDescriptorDataEXT::image(&texture_info)),
        ];
        let (buffer_slots, image_slots) = resource_bytes.split_at_mut(texture_offset as usize);
        let destinations = [
            vk::HostAddressRangeEXT::default().address(&mut buffer_slots[..buffer_size as usize]),
            vk::HostAddressRangeEXT::default().address(&mut image_slots[..image_size as usize]),
        ];
        device
            .write_resource_descriptors_ext(&resources, &destinations)
            .unwrap();
        device.unmap_memory(resource_heap_memory);

        let sampler_bytes = std::slice::from_raw_parts_mut(
            device
                .map_memory(
                    sampler_heap_memory,
                    0,
                    sampler_heap_size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap()
                .cast::<u8>(),
            sampler_heap_size as usize,
        );
        let samplers = [vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::NEAREST)
            .min_filter(vk::Filter::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)];
        let destinations =
            [vk::HostAddressRangeEXT::default()
                .address(&mut sampler_bytes[..sampler_size as usize])];
        device
            .write_sampler_descriptors_ext(&samplers, &destinations)
            .unwrap();
        device.unmap_memory(sampler_heap_memory);

        let vertex_code = spirv(include_bytes!("../shaders/descriptor_heap.vert.spv"));
        let fragment_code = spirv(include_bytes!("../shaders/descriptor_heap.frag.spv"));
        let vertex_module = device
            .create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(&vertex_code),
                None,
            )
            .unwrap();
        let fragment_module = device
            .create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(&fragment_code),
                None,
            )
            .unwrap();
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::VERTEX)
                .module(vertex_module)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .module(fragment_module)
                .name(c"main"),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let extent = vk::Extent2D {
            width: SIZE,
            height: SIZE,
        };
        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: SIZE as f32,
            height: SIZE as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [vk::Rect2D {
            offset: vk::Offset2D::default(),
            extent,
        }];
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default();
        let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                vk::ColorComponentFlagBits::R
                    | vk::ColorComponentFlagBits::G
                    | vk::ColorComponentFlagBits::B
                    | vk::ColorComponentFlagBits::A,
            )];
        let blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let color_formats = [FORMAT];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&color_formats);
        let mut flags2 = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&blend)
            .push_next(&mut flags2)
            .push_next(&mut rendering);
        let pipeline = device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .map_err(|(_, error)| error)
            .unwrap()[0];
        device.destroy_shader_module(vertex_module, None);
        device.destroy_shader_module(fragment_module, None);

        let (target, target_memory) = gpu.image(
            FORMAT,
            SIZE,
            SIZE,
            vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        );
        let target_view = device
            .create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(target)
                    .view_type(vk::ImageViewType::_2D)
                    .format(FORMAT)
                    .subresource_range(color_range()),
                None,
            )
            .unwrap();
        let byte_count = u64::from(SIZE * SIZE * 4);
        let (readback, readback_memory) =
            gpu.buffer(byte_count, vk::BufferUsageFlags::TRANSFER_DST, host, false);

        let push = [texture_index, 0, sampler_index, 0, params_index, 0]
            .iter()
            .flat_map(|value| value.to_ne_bytes())
            .collect::<Vec<u8>>();

        gpu.submit(|command_buffer| {
            let to_transfer_dst = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .image(texture)
                .subresource_range(color_range())];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_transfer_dst),
            );
            device.cmd_copy_buffer_to_image(
                command_buffer,
                texel_buffer,
                texture,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[vk::BufferImageCopy::default()
                    .image_subresource(color_layers())
                    .image_extent(vk::Extent3D {
                        width: 1,
                        height: 1,
                        depth: 1,
                    })],
            );

            let images = [
                vk::ImageMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::COPY)
                    .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                    .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
                    .dst_access_mask(vk::AccessFlags2::SHADER_SAMPLED_READ)
                    .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                    .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .image(texture)
                    .subresource_range(color_range()),
                vk::ImageMemoryBarrier2::default()
                    .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                    .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                    .old_layout(vk::ImageLayout::UNDEFINED)
                    .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                    .image(target)
                    .subresource_range(color_range()),
            ];
            let heaps = [
                vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::HOST)
                    .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                    .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
                    .dst_access_mask(vk::AccessFlags2::RESOURCE_HEAP_READ_EXT)
                    .buffer(resource_heap)
                    .size(resource_heap_size),
                vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::HOST)
                    .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                    .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
                    .dst_access_mask(vk::AccessFlags2::SAMPLER_HEAP_READ_EXT)
                    .buffer(sampler_heap)
                    .size(sampler_heap_size),
            ];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default()
                    .image_memory_barriers(&images)
                    .buffer_memory_barriers(&heaps),
            );

            let attachments = [vk::RenderingAttachmentInfo::default()
                .image_view(target_view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)];
            device.cmd_begin_rendering(
                command_buffer,
                &vk::RenderingInfo::default()
                    .render_area(scissors[0])
                    .layer_count(1)
                    .color_attachments(&attachments),
            );
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_bind_resource_heap_ext(
                command_buffer,
                &vk::BindHeapInfoEXT::default()
                    .heap_range(
                        vk::DeviceAddressRangeKHR::default()
                            .address(address(resource_heap))
                            .size(resource_heap_size),
                    )
                    .reserved_range_offset(resource_reserved)
                    .reserved_range_size(heap.min_resource_heap_reserved_range),
            );
            device.cmd_bind_sampler_heap_ext(
                command_buffer,
                &vk::BindHeapInfoEXT::default()
                    .heap_range(
                        vk::DeviceAddressRangeKHR::default()
                            .address(address(sampler_heap))
                            .size(sampler_heap_size),
                    )
                    .reserved_range_offset(sampler_reserved)
                    .reserved_range_size(heap.min_sampler_heap_reserved_range),
            );
            device.cmd_push_data_ext(
                command_buffer,
                &vk::PushDataInfoEXT::default()
                    .data(vk::HostAddressRangeConstEXT::default().address(&push)),
            );
            device.cmd_draw(command_buffer, 3, 1, 0, 0);
            device.cmd_end_rendering(command_buffer);

            let to_transfer_src = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(target)
                .subresource_range(color_range())];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_transfer_src),
            );
            device.cmd_copy_image_to_buffer(
                command_buffer,
                target,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback,
                &[vk::BufferImageCopy::default()
                    .image_subresource(color_layers())
                    .image_extent(vk::Extent3D {
                        width: SIZE,
                        height: SIZE,
                        depth: 1,
                    })],
            );
        });

        let expected = [
            (f32::from(TEXEL[0]) * TINT[0]).round() as u8,
            (f32::from(TEXEL[1]) * TINT[1]).round() as u8,
            (f32::from(TEXEL[2]) * TINT[2]).round() as u8,
            (f32::from(TEXEL[3]) * TINT[3]).round() as u8,
        ];
        let mapped = device
            .map_memory(readback_memory, 0, byte_count, vk::MemoryMapFlags::empty())
            .unwrap();
        let pixels = std::slice::from_raw_parts(mapped.cast::<u8>(), byte_count as usize);
        for pixel in pixels.chunks_exact(4) {
            assert!(
                pixel.iter().zip(expected).all(|(a, b)| a.abs_diff(b) <= 1),
                "pixel is {pixel:?}, expected {expected:?}"
            );
        }
        device.unmap_memory(readback_memory);

        device.destroy_pipeline(pipeline, None);
        device.destroy_image_view(target_view, None);
        for image in [texture, target] {
            device.destroy_image(image, None);
        }
        for buffer in [texel_buffer, params, resource_heap, sampler_heap, readback] {
            device.destroy_buffer(buffer, None);
        }
        for memory in [
            texture_memory,
            target_memory,
            texel_memory,
            params_memory,
            resource_heap_memory,
            sampler_heap_memory,
            readback_memory,
        ] {
            device.free_memory(memory, None);
        }
    }
}
