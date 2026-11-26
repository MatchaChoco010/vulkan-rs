//! The client side: render into an image whose memory can be exported, and describe it.

use anyhow::{Result, bail};
use std::os::fd::{FromRawFd, OwnedFd};
use vulkan::{Device, vk};
use vulkan_alloc::{AllocationCreateDesc, AllocationScheme, MemoryLocation};
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::image::{
    create_image_view, drm_format_modifier, is_linear_image, subresource_range,
};
use vulkan_example_common::memory::Image;

use crate::{BACKGROUND, DMA_BUF, Dmabuf, FORMAT, HEIGHT, Plane, RECTANGLE, WIDTH};

/// Renders into a new image with a DRM format modifier and exports its memory. The image
/// stays the client's: it must outlive every use of the dma-buf.
pub unsafe fn render_and_export(gpu: &Gpu) -> Result<(Owned<Image>, Dmabuf)> {
    unsafe {
        let device = &gpu.device;
        let image = create_exportable_image(gpu)?;
        let view = create_image_view(gpu, image.image, FORMAT, vk::ImageAspectFlags::COLOR)?;
        submit_and_wait(gpu, |command_buffer| {
            draw(device, command_buffer, image.image, *view);

            // Hand the image to whoever imports it: release ownership to the foreign
            // queue family. The importer acquires it with the mirror image of this.
            let release = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::GENERAL)
                .src_queue_family_index(gpu.queue_family)
                .dst_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .image(image.image)
                .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&release),
            );
            Ok(())
        })?;
        drop(view);

        let dmabuf = describe(device, &image)?;
        Ok((image, dmabuf))
    }
}

/// An image that the driver lays out with one of the DRM format modifiers that can do
/// what we need and that use one memory plane, in memory of its own that can be exported
/// as a dma-buf. A real client would intersect the modifiers with the ones the
/// compositor advertises.
unsafe fn create_exportable_image(gpu: &Gpu) -> Result<Owned<Image>> {
    unsafe {
        let device = &gpu.device;
        let needed =
            vk::FormatFeatureFlags::COLOR_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
        let candidates = format_modifiers(gpu, FORMAT)?
            .into_iter()
            .filter(|m| {
                m.drm_format_modifier_plane_count == 1
                    && m.drm_format_modifier_tiling_features.contains(needed)
            })
            .map(|m| m.drm_format_modifier)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            bail!("no single-plane DRM format modifier supports rendering to {FORMAT:?}");
        }

        let mut modifier_list = vk::ImageDrmFormatModifierListCreateInfoEXT::default()
            .drm_format_modifiers(&candidates);
        let mut external =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF.into());
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(FORMAT)
            .extent(vk::Extent3D {
                width: WIDTH,
                height: HEIGHT,
                depth: 1,
            })
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
            .push_next(&mut modifier_list)
            .push_next(&mut external);
        let image = gpu.own(device.create_image(&image_info, None)?);
        // The driver chose one of the candidates, which may be DRM_FORMAT_MOD_LINEAR;
        // that makes the image a linear resource.
        let modifier = drm_format_modifier(device, *image)?;

        // vulkan-alloc gives exportable memory its own VkDeviceMemory, so the exported
        // fd refers to this image only.
        let requirements = device.get_image_memory_requirements(*image);
        let allocation = gpu.with_allocator(|allocator| {
            allocator.allocate(AllocationCreateDesc {
                name: Some("exported image"),
                export_handle_types: DMA_BUF.into(),
                allocation_scheme: AllocationScheme::DedicatedImage(*image),
                ..AllocationCreateDesc::new(
                    requirements,
                    MemoryLocation::GpuOnly,
                    is_linear_image(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT, modifier),
                )
            })
        })?;
        // From here the image and its memory go together.
        let image = gpu.own(Image {
            image: image.into_inner(),
            allocation,
        });
        device.bind_image_memory(
            image.image,
            image.allocation.memory(),
            image.allocation.offset(),
        )?;
        Ok(image)
    }
}

/// Clears are enough to draw a recognizable picture without shaders: an orange
/// background with a blue rectangle in the lower right quadrant.
unsafe fn draw(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    view: vk::ImageView,
) {
    let to_attachment = [vk::ImageMemoryBarrier2::default()
        .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    let attachments = [vk::RenderingAttachmentInfo::default()
        .image_view(view)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(clear_color(BACKGROUND))];
    let full = vk::Rect2D {
        offset: vk::Offset2D::default(),
        extent: vk::Extent2D {
            width: WIDTH,
            height: HEIGHT,
        },
    };
    let lower_right = vk::Rect2D {
        offset: vk::Offset2D {
            x: (WIDTH / 2) as i32,
            y: (HEIGHT / 2) as i32,
        },
        extent: vk::Extent2D {
            width: WIDTH / 2,
            height: HEIGHT / 2,
        },
    };
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().image_memory_barriers(&to_attachment),
        );
        device.cmd_begin_rendering(
            command_buffer,
            &vk::RenderingInfo::default()
                .render_area(full)
                .layer_count(1)
                .color_attachments(&attachments),
        );
        device.cmd_clear_attachments(
            command_buffer,
            &[vk::ClearAttachment::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .color_attachment(0)
                .clear_value(clear_color(RECTANGLE))],
            &[vk::ClearRect::default().rect(lower_right).layer_count(1)],
        );
        device.cmd_end_rendering(command_buffer);
    }
}

fn clear_color(rgba: [f32; 4]) -> vk::ClearValue {
    vk::ClearValue::color(vk::ClearColorValue::float32(rgba))
}

/// What the driver chose for the image, and a new fd for its memory, which the
/// [`Dmabuf`] owns.
unsafe fn describe(device: &Device, image: &Image) -> Result<Dmabuf> {
    unsafe {
        let mut chosen = vk::ImageDrmFormatModifierPropertiesEXT::default();
        device.get_image_drm_format_modifier_properties_ext(image.image, &mut chosen)?;
        let plane = device.get_image_subresource_layout(
            image.image,
            &vk::ImageSubresource::default().aspect_mask(vk::ImageAspectFlags::MEMORY_PLANE_0_EXT),
        );
        let fd = device.get_memory_fd_khr(
            &vk::MemoryGetFdInfoKHR::default()
                .memory(image.allocation.memory())
                .handle_type(DMA_BUF),
        )?;
        Ok(Dmabuf {
            fd: OwnedFd::from_raw_fd(fd),
            width: WIDTH,
            height: HEIGHT,
            format: FORMAT,
            modifier: chosen.drm_format_modifier,
            planes: vec![Plane {
                offset: plane.offset,
                stride: plane.row_pitch,
            }],
        })
    }
}

/// Every DRM format modifier the device supports for `format`.
unsafe fn format_modifiers(
    gpu: &Gpu,
    format: vk::Format,
) -> Result<Vec<vk::DrmFormatModifierPropertiesEXT>> {
    unsafe {
        // First call: how many?
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let count = list.drm_format_modifier_count as usize;

        // Second call: fill our array.
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
        modifiers.truncate(written);
        Ok(modifiers)
    }
}
