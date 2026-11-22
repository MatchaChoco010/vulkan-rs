use anyhow::{Context, Result};
use vulkan::{Device, vk};
use vulkan_example_common::device::Gpu;
use vulkan_example_common::env::{one_of, parsed_var};

/// What `VULKAN_EXAMPLE_COLOR_SPACE` asks for.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorSpace {
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
pub fn requested_color_space() -> Result<ColorSpace> {
    let requested = parsed_var("VULKAN_EXAMPLE_COLOR_SPACE", |value| {
        one_of(value, &COLOR_SPACES)
    })?;
    Ok(requested.unwrap_or(ColorSpace::Srgb))
}

/// How `VULKAN_EXAMPLE_COLOR_SPACE` spells `color_space`.
fn name(color_space: ColorSpace) -> &'static str {
    COLOR_SPACES
        .iter()
        .find(|(_, listed)| *listed == color_space)
        .map_or("srgb", |(name, _)| name)
}

/// The format and color space for the swapchain in the color space `wanted`. Falls back
/// to 8-bit sRGB when the surface does not offer it.
pub unsafe fn choose_surface_format(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
    wanted: ColorSpace,
) -> Result<vk::SurfaceFormatKHR> {
    let formats = unsafe {
        gpu.instance
            .get_physical_device_surface_formats_khr(gpu.physical_device, surface)
    }?;
    let (chosen, offered) = pick_surface_format(&formats, wanted)?;
    if !offered && wanted != ColorSpace::Srgb {
        println!(
            "color space `{}` is not offered by the surface; using sRGB",
            name(wanted)
        );
    }
    Ok(chosen)
}

/// 8-bit sRGB, which every example can draw to and which is the fallback.
const SRGB: (vk::ColorSpaceKHR, &[vk::Format]) = (
    vk::ColorSpaceKHR::SRGB_NONLINEAR,
    &[vk::Format::B8G8R8A8_UNORM, vk::Format::R8G8B8A8_UNORM],
);

/// The color space that `wanted` asks for and the formats that go with it, in order of
/// preference. `None` for sRGB, which is the fallback anyway.
fn wanted_candidates(wanted: ColorSpace) -> Option<(vk::ColorSpaceKHR, &'static [vk::Format])> {
    match wanted {
        // BT.2020 primaries with the PQ transfer function; needs 10 bits or more.
        ColorSpace::Hdr10 => Some((
            vk::ColorSpaceKHR::HDR10_ST2084_EXT,
            &[
                vk::Format::A2B10G10R10_UNORM_PACK32,
                vk::Format::A2R10G10B10_UNORM_PACK32,
                vk::Format::R16G16B16A16_SFLOAT,
            ],
        )),
        // sRGB primaries, linear, values outside [0, 1] allowed.
        ColorSpace::Scrgb => Some((
            vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT,
            &[vk::Format::R16G16B16A16_SFLOAT],
        )),
        ColorSpace::Srgb => None,
    }
}

/// From the `formats` that a surface offers: the first candidate of `wanted` that is
/// among them, else the first 8-bit sRGB one, else the first of them. The flag tells
/// whether a candidate of `wanted` was found. An error when the surface offers no format.
fn pick_surface_format(
    formats: &[vk::SurfaceFormatKHR],
    wanted: ColorSpace,
) -> Result<(vk::SurfaceFormatKHR, bool)> {
    let find = |(color_space, candidates): (vk::ColorSpaceKHR, &[vk::Format])| {
        candidates.iter().find_map(|&format| {
            formats
                .iter()
                .copied()
                .find(|f| f.format == format && f.color_space == color_space)
        })
    };
    if let Some(chosen) = wanted_candidates(wanted).and_then(find) {
        return Ok((chosen, true));
    }
    let chosen = find(SRGB)
        .or(formats.first().copied())
        .context("the surface offers no format")?;
    Ok((chosen, false))
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

    const FORMATS: [vk::Format; 5] = [
        vk::Format::B8G8R8A8_UNORM,
        vk::Format::R8G8B8A8_UNORM,
        vk::Format::A2B10G10R10_UNORM_PACK32,
        vk::Format::R16G16B16A16_SFLOAT,
        vk::Format::B8G8R8A8_SRGB,
    ];
    const SURFACE_COLOR_SPACES: [vk::ColorSpaceKHR; 3] = [
        vk::ColorSpaceKHR::SRGB_NONLINEAR,
        vk::ColorSpaceKHR::HDR10_ST2084_EXT,
        vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT,
    ];

    fn pair(format: vk::SurfaceFormatKHR) -> (vk::Format, vk::ColorSpaceKHR) {
        (format.format, format.color_space)
    }

    /// Pseudo-random lists of up to six formats, the empty list among them.
    fn format_lists() -> impl Iterator<Item = Vec<vk::SurfaceFormatKHR>> {
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = move |bound: usize| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state as usize % bound
        };
        (0..500).map(move |_| {
            (0..next(7))
                .map(|_| vk::SurfaceFormatKHR {
                    format: FORMATS[next(FORMATS.len())],
                    color_space: SURFACE_COLOR_SPACES[next(SURFACE_COLOR_SPACES.len())],
                })
                .collect()
        })
    }

    /// The documented preference as one ranked list: the candidates of `wanted`, then
    /// those of 8-bit sRGB.
    fn preference(wanted: ColorSpace) -> Vec<(vk::Format, vk::ColorSpaceKHR)> {
        wanted_candidates(wanted)
            .into_iter()
            .chain([SRGB])
            .flat_map(|(color_space, formats)| {
                formats.iter().map(move |&format| (format, color_space))
            })
            .collect()
    }

    #[test]
    fn picks_the_most_preferred_offered_format_and_fails_only_on_an_empty_list() {
        for (_, wanted) in COLOR_SPACES {
            let ranked = preference(wanted);
            let wanted_count = wanted_candidates(wanted).map_or(0, |(_, formats)| formats.len());
            for formats in format_lists() {
                let listed = formats.iter().copied().map(pair).collect::<Vec<_>>();
                let picked = pick_surface_format(&formats, wanted);
                if formats.is_empty() {
                    assert!(picked.is_err(), "{wanted:?}: an empty list gave a format");
                    continue;
                }
                let (picked, offered) = picked.unwrap();
                let picked = pair(picked);
                assert!(
                    listed.contains(&picked),
                    "{wanted:?}: {picked:?} is not in {listed:?}"
                );
                let best = ranked
                    .iter()
                    .position(|candidate| listed.contains(candidate));
                match best {
                    Some(rank) => {
                        assert_eq!(picked, ranked[rank], "{wanted:?} from {listed:?}");
                        assert_eq!(offered, rank < wanted_count, "{wanted:?} from {listed:?}");
                    }
                    None => {
                        assert_eq!(picked, listed[0], "{wanted:?} from {listed:?}");
                        assert!(!offered, "{wanted:?} from {listed:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn every_color_space_is_named_once_and_other_values_are_errors() {
        for (index, (name, color_space)) in COLOR_SPACES.iter().enumerate() {
            assert_eq!(one_of(name, &COLOR_SPACES).unwrap(), *color_space, "{name}");
            assert_eq!(super::name(*color_space), *name);
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
