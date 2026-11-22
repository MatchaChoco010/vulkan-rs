use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_alloc::{Allocator, MemoryLocation};
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::device::Gpu;
use vulkan_example_common::image::{image_info_2d, subresource_range};
use vulkan_example_common::memory::{Image, create_buffer, create_image, write_buffer};

pub const TEXTURE_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;

/// A texture holding `texels`, row by row from the top, copied there through a staging
/// buffer and left in `SHADER_READ_ONLY_OPTIMAL` for the fragment shader.
pub unsafe fn create_texture(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &str,
    extent: vk::Extent2D,
    texels: &[[u8; 4]],
) -> Result<Image> {
    unsafe {
        let device = &gpu.device;
        let bytes: &[u8] = bytemuck::cast_slice(texels);
        let staging = create_buffer(
            device,
            allocator,
            "texture staging buffer",
            bytes.len() as u64,
            vk::BufferUsageFlags::TRANSFER_SRC,
            MemoryLocation::CpuToGpu,
        )?;
        write_buffer(allocator, &staging, bytes)?;
        let image = create_image(
            device,
            allocator,
            name,
            &image_info_2d(
                TEXTURE_FORMAT,
                extent,
                vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
            ),
        )?;
        submit_and_wait(gpu, |command_buffer| {
            let to_transfer = vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL);
            image_barrier(device, command_buffer, image.image, to_transfer);
            copy_buffer_to_image(device, command_buffer, staging.buffer, image.image, extent);
            // The second scope reaches into the submission that samples the texture.
            let to_shader = vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COPY)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::FRAGMENT_SHADER)
                .dst_access_mask(vk::AccessFlags2::SHADER_SAMPLED_READ)
                .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL);
            image_barrier(device, command_buffer, image.image, to_shader);
            Ok(())
        })?;
        staging.destroy(device, allocator)?;
        Ok(image)
    }
}

unsafe fn image_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    barrier: vk::ImageMemoryBarrier2<'_>,
) {
    let barriers = [barrier
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().image_memory_barriers(&barriers),
        );
    }
}

unsafe fn copy_buffer_to_image(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    buffer: vk::Buffer,
    image: vk::Image,
    extent: vk::Extent2D,
) {
    let regions = [vk::BufferImageCopy2::default()
        .image_subresource(
            vk::ImageSubresourceLayers::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .layer_count(1),
        )
        .image_extent(
            vk::Extent3D::default()
                .width(extent.width)
                .height(extent.height)
                .depth(1),
        )];
    unsafe {
        device.cmd_copy_buffer_to_image2(
            command_buffer,
            &vk::CopyBufferToImageInfo2::default()
                .src_buffer(buffer)
                .dst_image(image)
                .dst_image_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .regions(&regions),
        );
    }
}
