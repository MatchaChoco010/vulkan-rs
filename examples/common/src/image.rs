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

/// `DRM_FORMAT_MOD_LINEAR` of `drm_fourcc.h`: rows of texels one after the other.
pub const DRM_FORMAT_MOD_LINEAR: u64 = 0;

/// Whether an image is a linear resource, as the Vulkan glossary defines it, which is
/// what [`vulkan_alloc::AllocationCreateDesc::linear`] asks: an image in `LINEAR`
/// tiling, or in `DRM_FORMAT_MODIFIER_EXT` tiling with `DRM_FORMAT_MOD_LINEAR` as
/// `drm_format_modifier`, the modifier that the image has. The modifier does not matter
/// for the other tilings.
pub fn is_linear_image(tiling: vk::ImageTiling, drm_format_modifier: u64) -> bool {
    match tiling {
        vk::ImageTiling::LINEAR => true,
        vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT => drm_format_modifier == DRM_FORMAT_MOD_LINEAR,
        _ => false,
    }
}

/// The DRM format modifier that `image`, created in `DRM_FORMAT_MODIFIER_EXT` tiling,
/// was given: the one requested explicitly, or the one the implementation chose from a
/// list of candidates when it created the image.
pub unsafe fn drm_format_modifier(device: &Device, image: vk::Image) -> Result<u64> {
    let mut properties = vk::ImageDrmFormatModifierPropertiesEXT::default();
    unsafe { device.get_image_drm_format_modifier_properties_ext(image, &mut properties) }?;
    Ok(properties.drm_format_modifier)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_image_is_linear_exactly_as_the_glossary_says() {
        let modifiers = [
            DRM_FORMAT_MOD_LINEAR,
            1,
            0x0100_0000_0000_0001,
            0x0200_0000_0000_0001,
            0x00ff_ffff_ffff_ffff,
            u64::MAX,
        ];
        for tiling in [
            vk::ImageTiling::OPTIMAL,
            vk::ImageTiling::LINEAR,
            vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT,
        ] {
            for modifier in modifiers {
                let expected = tiling == vk::ImageTiling::LINEAR
                    || (tiling == vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT
                        && modifier == DRM_FORMAT_MOD_LINEAR);
                assert_eq!(
                    is_linear_image(tiling, modifier),
                    expected,
                    "{tiling:?} with modifier {modifier:#018x}"
                );
            }
        }
    }
}
