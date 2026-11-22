use crate::gpu::Gpu;
use vulkan::vk;

#[test]
fn dynamic_rendering_clear_reads_back() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    const SIZE: u32 = 16;
    unsafe {
        let device = &gpu.device;
        let format = vk::Format::R8G8B8A8_UNORM;
        let extent = vk::Extent3D {
            width: SIZE,
            height: SIZE,
            depth: 1,
        };

        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(format)
            .extent(extent)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
        let image = device.create_image(&image_info, None).unwrap();
        let requirements = device.get_image_memory_requirements(image);
        let image_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_image_memory(image, image_memory, 0).unwrap();

        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let view = device
            .create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::_2D)
                    .format(format)
                    .subresource_range(range),
                None,
            )
            .unwrap();

        let byte_count = u64::from(SIZE * SIZE * 4);
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(byte_count)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();
        let requirements = device.get_buffer_memory_requirements(buffer);
        let buffer_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::HOST_VISIBLE
                                | vk::MemoryPropertyFlags::HOST_COHERENT,
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

        gpu.submit(|command_buffer| {
            let to_attachment = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(range)];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_attachment),
            );

            let attachments = [vk::RenderingAttachmentInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                    1.0, 0.0, 0.0, 1.0,
                ])))];
            let rendering = vk::RenderingInfo::default()
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: vk::Extent2D {
                        width: SIZE,
                        height: SIZE,
                    },
                })
                .layer_count(1)
                .color_attachments(&attachments);
            device.cmd_begin_rendering(command_buffer, &rendering);
            device.cmd_end_rendering(command_buffer);

            let to_transfer = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(image)
                .subresource_range(range)];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_transfer),
            );

            let regions = [vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(extent)];
            device.cmd_copy_image_to_buffer(
                command_buffer,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer,
                &regions,
            );
        });

        let mapped = device
            .map_memory(buffer_memory, 0, byte_count, vk::MemoryMapFlags::empty())
            .unwrap();
        let pixels = std::slice::from_raw_parts(mapped.cast::<u8>(), byte_count as usize);
        assert!(
            pixels
                .chunks_exact(4)
                .all(|pixel| pixel == [255, 0, 0, 255]),
            "first pixel is {:?}",
            &pixels[..4]
        );
        device.unmap_memory(buffer_memory);

        device.destroy_buffer(buffer, None);
        device.free_memory(buffer_memory, None);
        device.destroy_image_view(view, None);
        device.destroy_image(image, None);
        device.free_memory(image_memory, None);
    }
}
