use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::image::{
    copy_image_to_buffer, create_image_view, image_info_2d, subresource_range,
};
use vulkan_example_common::memory::{Buffer, Image, create_image};

use crate::{COLOR_FORMAT, DEPTH_FORMAT, EXTENT};

/// The color and depth images that the spheres are drawn into. The fields drop in
/// order, each view before its image.
pub struct RenderTargets {
    depth_view: Owned<vk::ImageView>,
    depth: Owned<Image>,
    color_view: Owned<vk::ImageView>,
    color: Owned<Image>,
}

impl RenderTargets {
    pub unsafe fn new(gpu: &Gpu) -> Result<Self> {
        unsafe {
            let color = create_image(
                gpu,
                "color target",
                &image_info_2d(
                    COLOR_FORMAT,
                    EXTENT,
                    vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
                ),
            )?;
            let color_view =
                create_image_view(gpu, color.image, COLOR_FORMAT, vk::ImageAspectFlags::COLOR)?;
            let depth = create_image(
                gpu,
                "depth target",
                &image_info_2d(
                    DEPTH_FORMAT,
                    EXTENT,
                    vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
                ),
            )?;
            let depth_view =
                create_image_view(gpu, depth.image, DEPTH_FORMAT, vk::ImageAspectFlags::DEPTH)?;
            Ok(Self {
                depth_view,
                depth,
                color_view,
                color,
            })
        }
    }

    /// Records the transitions to the attachment layouts and begins rendering into
    /// both images, cleared.
    pub unsafe fn begin_rendering(&self, device: &Device, command_buffer: vk::CommandBuffer) {
        let to_attachment = [
            vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(self.color.image)
                .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR)),
            vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(
                    vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                        | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                )
                .dst_access_mask(
                    vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ
                        | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
                )
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
                .image(self.depth.image)
                .subresource_range(subresource_range(vk::ImageAspectFlags::DEPTH)),
        ];
        let color_attachments = [vk::RenderingAttachmentInfo::default()
            .image_view(*self.color_view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                0.02, 0.03, 0.05, 1.0,
            ])))];
        let depth_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(*self.depth_view)
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .clear_value(vk::ClearValue::depth_stencil(
                vk::ClearDepthStencilValue::default().depth(1.0),
            ));
        unsafe {
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_attachment),
            );
            device.cmd_begin_rendering(
                command_buffer,
                &vk::RenderingInfo::default()
                    .render_area(vk::Rect2D::default().extent(EXTENT))
                    .layer_count(1)
                    .color_attachments(&color_attachments)
                    .depth_attachment(&depth_attachment),
            );
        }
    }

    /// Records, after rendering has ended, a copy of the color image to `readback`,
    /// where the host can read it once the submission is waited for.
    pub unsafe fn copy_color_to(
        &self,
        device: &Device,
        command_buffer: vk::CommandBuffer,
        readback: &Buffer,
    ) {
        let to_transfer = [vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COPY)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .image(self.color.image)
            .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
        let to_host = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COPY)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ)
            .buffer(readback.buffer)
            .size(readback.size)];
        unsafe {
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_transfer),
            );
            copy_image_to_buffer(
                device,
                command_buffer,
                self.color.image,
                EXTENT,
                readback.buffer,
            );
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().buffer_memory_barriers(&to_host),
            );
        }
    }
}
