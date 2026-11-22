use anyhow::Result;
use vulkan::{Device, vk};

use crate::device::{Gpu, Owned};

/// A 2D image with one mip level, one array layer and one sample, in optimal tiling.
pub fn image_info_2d(
    format: vk::Format,
    extent: vk::Extent2D,
    usage: vk::ImageUsageFlags,
) -> vk::ImageCreateInfo<'static> {
    vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(format)
        .extent(
            vk::Extent3D::default()
                .width(extent.width)
                .height(extent.height)
                .depth(1),
        )
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
}

/// The first mip level and array layer, which is all of an image from [`image_info_2d`].
pub fn subresource_range(aspect: vk::ImageAspectFlags) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .level_count(1)
        .layer_count(1)
}

/// A 2D view of the first mip level and array layer.
pub unsafe fn create_image_view(
    gpu: &Gpu,
    image: vk::Image,
    format: vk::Format,
    aspect: vk::ImageAspectFlags,
) -> Result<Owned<vk::ImageView>> {
    let info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::_2D)
        .format(format)
        .subresource_range(subresource_range(aspect));
    unsafe { Ok(gpu.own(gpu.device.create_image_view(&info, None)?)) }
}

/// Records a copy of the first mip level and array layer of a color image, which must be
/// in `TRANSFER_SRC_OPTIMAL`, to the start of `buffer`, rows tightly packed.
pub unsafe fn copy_image_to_buffer(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    extent: vk::Extent2D,
    buffer: vk::Buffer,
) {
    let region = vk::BufferImageCopy::default()
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
        );
    unsafe {
        device.cmd_copy_image_to_buffer(
            command_buffer,
            image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            buffer,
            &[region],
        );
    }
}
