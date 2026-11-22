use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_example_common::device::Gpu;
use vulkan_example_common::env::{one_of, parsed_var};

/// What `VULKAN_EXAMPLE_COLOR_SPACE` asks for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ColorSpace {
    Srgb,
    Hdr10,
    Scrgb,
}

/// The values of `VULKAN_EXAMPLE_COLOR_SPACE`.
const COLOR_SPACES: [(&str, ColorSpace); 3] = [
    ("srgb", ColorSpace::Srgb),
    ("hdr10", ColorSpace::Hdr10),
    ("scrgb", ColorSpace::Scrgb),
];

/// The color space that `VULKAN_EXAMPLE_COLOR_SPACE` asks for, sRGB when it is not set.
/// An error for a value that is not one of [`COLOR_SPACES`].
fn requested_color_space() -> Result<ColorSpace> {
    let requested = parsed_var("VULKAN_EXAMPLE_COLOR_SPACE", |value| {
        one_of(value, &COLOR_SPACES)
    })?;
    Ok(requested.unwrap_or(ColorSpace::Srgb))
}

/// The format and color space for the swapchain, following `VULKAN_EXAMPLE_COLOR_SPACE`.
/// Falls back to 8-bit sRGB when the surface does not offer the requested one.
pub unsafe fn choose_surface_format(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceFormatKHR> {
    let formats = unsafe {
        gpu.instance
            .get_physical_device_surface_formats_khr(gpu.physical_device, surface)
    }?;
    let wanted = requested_color_space()?;
    let find = |color_space: vk::ColorSpaceKHR, candidates: &[vk::Format]| {
        candidates.iter().find_map(|&format| {
            formats
                .iter()
                .copied()
                .find(|f| f.format == format && f.color_space == color_space)
        })
    };
    let chosen = match wanted {
        // BT.2020 primaries with the PQ transfer function; needs 10 bits or more.
        ColorSpace::Hdr10 => find(
            vk::ColorSpaceKHR::HDR10_ST2084_EXT,
            &[
                vk::Format::A2B10G10R10_UNORM_PACK32,
                vk::Format::A2R10G10B10_UNORM_PACK32,
                vk::Format::R16G16B16A16_SFLOAT,
            ],
        ),
        // sRGB primaries, linear, values outside [0, 1] allowed.
        ColorSpace::Scrgb => find(
            vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT,
            &[vk::Format::R16G16B16A16_SFLOAT],
        ),
        ColorSpace::Srgb => None,
    };
    if chosen.is_none() && wanted != ColorSpace::Srgb {
        println!("color space {wanted:?} is not offered by the surface; using sRGB");
    }
    Ok(chosen
        .or_else(|| {
            find(
                vk::ColorSpaceKHR::SRGB_NONLINEAR,
                &[vk::Format::B8G8R8A8_UNORM, vk::Format::R8G8B8A8_UNORM],
            )
        })
        .unwrap_or(formats[0]))
}

/// Describes the mastering display to the compositor when the swapchain is HDR10.
/// `VK_EXT_hdr_metadata` must be enabled on the device.
pub unsafe fn set_hdr_metadata(
    device: &Device,
    swapchain: vk::SwapchainKHR,
    color_space: vk::ColorSpaceKHR,
) {
    if color_space != vk::ColorSpaceKHR::HDR10_ST2084_EXT {
        return;
    }
    let xy = |x: f32, y: f32| vk::XYColorEXT { x, y };
    // BT.2020 primaries, D65 white, a 1000 cd/m2 mastering display.
    let metadata = vk::HdrMetadataEXT::default()
        .display_primary_red(xy(0.708, 0.292))
        .display_primary_green(xy(0.170, 0.797))
        .display_primary_blue(xy(0.131, 0.046))
        .white_point(xy(0.3127, 0.3290))
        .max_luminance(1000.0)
        .min_luminance(0.001)
        .max_content_light_level(1000.0)
        .max_frame_average_light_level(400.0);
    unsafe { device.set_hdr_metadata_ext(&[swapchain], &[metadata]) };
    println!("set HDR10 metadata");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_color_space_is_named_once_and_other_values_are_errors() {
        for (index, (name, color_space)) in COLOR_SPACES.iter().enumerate() {
            assert_eq!(one_of(name, &COLOR_SPACES).unwrap(), *color_space, "{name}");
            assert!(
                COLOR_SPACES[index + 1..]
                    .iter()
                    .all(|(other_name, other)| other_name != name && other != color_space),
                "{name} is listed twice"
            );
        }
        for value in ["", "sRGB", "HDR10", "hdr", "pq", "scRGB", "linear"] {
            assert!(
                one_of(value, &COLOR_SPACES).is_err(),
                "{value:?} was accepted"
            );
        }
    }
}
