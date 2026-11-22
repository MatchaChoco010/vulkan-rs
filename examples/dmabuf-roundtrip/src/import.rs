//! The compositor side: turn a client's dma-buf into an image, and read it.

use anyhow::{Context, Result, bail};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};
use vulkan::vk;
use vulkan_alloc::MemoryLocation;
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::image::{copy_image_to_buffer, subresource_range};
use vulkan_example_common::memory::{create_buffer, read_buffer};

use crate::{DMA_BUF, Dmabuf};

/// An image whose memory is a dma-buf that the device imported. The fields drop in
/// order, the image before its memory.
pub struct ImportedImage {
    pub image: Owned<vk::Image>,
    _memory: Owned<vk::DeviceMemory>,
}

/// Checks that the dma-buf can be imported, creates an image with exactly the layout it
/// describes, and imports the fd as the image's memory.
pub unsafe fn import_image(gpu: &Gpu, dmabuf: Dmabuf) -> Result<ImportedImage> {
    unsafe {
        let device = &gpu.device;
        let usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::SAMPLED;
        if dmabuf.planes.len() != 1 {
            bail!("this example imports single-plane buffers only");
        }

        // 1. Can this format, modifier and usage be imported at this size?
        let mut modifier_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
            .drm_format_modifier(dmabuf.modifier)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let mut external_info =
            vk::PhysicalDeviceExternalImageFormatInfo::default().handle_type(DMA_BUF);
        let format_info = vk::PhysicalDeviceImageFormatInfo2::default()
            .format(dmabuf.format)
            .ty(vk::ImageType::_2D)
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(usage)
            .push_next(&mut modifier_info)
            .push_next(&mut external_info);
        let mut external_properties = vk::ExternalImageFormatProperties::default();
        let mut format_properties =
            vk::ImageFormatProperties2::default().push_next(&mut external_properties);
        gpu.instance
            .get_physical_device_image_format_properties2(
                gpu.physical_device,
                &format_info,
                &mut format_properties,
            )
            .context("the device cannot import this format and modifier")?;
        let limits = format_properties.image_format_properties.max_extent;
        if dmabuf.width > limits.width || dmabuf.height > limits.height {
            bail!("the buffer is larger than the device can import");
        }
        if !external_properties
            .external_memory_properties
            .external_memory_features
            .contains(vk::ExternalMemoryFeatureFlags::IMPORTABLE)
        {
            bail!("dma-bufs with this format and modifier are not importable");
        }

        // 2. An image with exactly the layout the client described. Only the offset and
        //    the row pitch are given; Vulkan requires the other members to be zero.
        let plane_layouts = dmabuf
            .planes
            .iter()
            .map(|plane| {
                vk::SubresourceLayout::default()
                    .offset(plane.offset)
                    .row_pitch(plane.stride)
            })
            .collect::<Vec<_>>();
        let mut explicit = vk::ImageDrmFormatModifierExplicitCreateInfoEXT::default()
            .drm_format_modifier(dmabuf.modifier)
            .plane_layouts(&plane_layouts);
        let mut external =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF.into());
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(dmabuf.format)
            .extent(vk::Extent3D {
                width: dmabuf.width,
                height: dmabuf.height,
                depth: 1,
            })
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(usage)
            .push_next(&mut explicit)
            .push_next(&mut external);
        let image = gpu.own(device.create_image(&image_info, None)?);

        // 3. Import the fd as the image's memory. The memory type must be one that both
        //    the image and the dma-buf allow.
        let requirements = device.get_image_memory_requirements(*image);
        let mut fd_properties = vk::MemoryFdPropertiesKHR::default();
        device.get_memory_fd_properties_khr(DMA_BUF, dmabuf.fd.as_raw_fd(), &mut fd_properties)?;
        let type_bits = requirements.memory_type_bits & fd_properties.memory_type_bits;
        let Some(memory_type) = (0..u32::BITS).find(|index| type_bits & (1 << index) != 0) else {
            bail!("no memory type fits both the image and the dma-buf");
        };

        // A successful import takes ownership of the fd; on failure it is still ours.
        let raw_fd = dmabuf.fd.into_raw_fd();
        let mut import = vk::ImportMemoryFdInfoKHR::default()
            .handle_type(DMA_BUF)
            .fd(raw_fd);
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(*image);
        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type)
            .push_next(&mut import)
            .push_next(&mut dedicated);
        let memory = match device.allocate_memory(&allocate_info, None) {
            Ok(memory) => gpu.own(memory),
            Err(error) => {
                drop(OwnedFd::from_raw_fd(raw_fd));
                return Err(error).context("importing the dma-buf");
            }
        };
        device.bind_image_memory(*image, *memory, 0)?;
        Ok(ImportedImage {
            image,
            _memory: memory,
        })
    }
}

/// Copies an imported image into host memory as tightly packed BGRA.
pub unsafe fn read_back(gpu: &Gpu, image: &ImportedImage, extent: vk::Extent2D) -> Result<Vec<u8>> {
    unsafe {
        let device = &gpu.device;
        let buffer = create_buffer(
            gpu,
            "readback",
            u64::from(extent.width) * u64::from(extent.height) * 4,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
        )?;

        submit_and_wait(gpu, |command_buffer| {
            // Acquire ownership from the foreign queue family. The contents of an
            // imported image are only defined when it arrives this way: a transition
            // from UNDEFINED would allow the driver to discard them.
            let acquire = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::GENERAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .dst_queue_family_index(gpu.queue_family)
                .image(*image.image)
                .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&acquire),
            );
            copy_image_to_buffer(device, command_buffer, *image.image, extent, buffer.buffer);
            let to_host = [vk::BufferMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COPY)
                .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::HOST)
                .dst_access_mask(vk::AccessFlags2::HOST_READ)
                .buffer(buffer.buffer)
                .size(buffer.size)];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().buffer_memory_barriers(&to_host),
            );
            Ok(())
        })?;

        read_buffer(gpu, &buffer)
    }
}
