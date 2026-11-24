use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use std::collections::BTreeMap;
use vulkan::vk::{
    self, ChromaSubsampling, ComponentName, Extent2D, FormatInfo, ImageAspectFlags, NumericFormat,
    ShaderNumericType,
};

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Named {
    name: ComponentName,
    bits: Option<u32>,
    numeric_format: NumericFormat,
    plane: Option<u32>,
}

#[derive(Debug, Default)]
struct ParsedName {
    groups: Vec<Vec<(char, u32)>>,
    numeric_formats: Vec<NumericFormat>,
    group_numeric_formats: Vec<NumericFormat>,
    planes: Option<u32>,
    chroma: Option<ChromaSubsampling>,
    words: Option<(u32, u32)>,
    block: bool,
}

fn numeric_format_of(token: &str) -> Option<NumericFormat> {
    Some(match token {
        "UNORM" => NumericFormat::Unorm,
        "SNORM" => NumericFormat::Snorm,
        "USCALED" => NumericFormat::Uscaled,
        "SSCALED" => NumericFormat::Sscaled,
        "UINT" => NumericFormat::Uint,
        "SINT" => NumericFormat::Sint,
        "UFLOAT" => NumericFormat::Ufloat,
        "SFLOAT" => NumericFormat::Sfloat,
        "SRGB" => NumericFormat::Srgb,
        "SFIXED5" => NumericFormat::Sfixed5,
        "BOOL" => NumericFormat::Bool,
        _ => return None,
    })
}

fn component_group(token: &str) -> Option<Vec<(char, u32)>> {
    let mut group = Vec::new();
    let mut rest = token;
    while let Some(letter) = rest.chars().next() {
        if !"RGBADSXE".contains(letter) {
            return None;
        }
        let digits = rest[1..]
            .find(|c: char| !c.is_ascii_digit())
            .map_or(rest.len(), |end| end + 1);
        group.push((letter, rest[1..digits].parse().ok()?));
        rest = &rest[digits..];
    }
    (!group.is_empty()).then_some(group)
}

fn words(token: &str) -> Option<(u32, u32)> {
    let (count, bits) = token.split_once("PACK")?;
    let count = if count.is_empty() {
        1
    } else {
        count.parse().ok()?
    };
    Some((count, bits.parse().ok()?))
}

fn parse(name: &str) -> ParsedName {
    let mut parsed = ParsedName::default();
    let mut pending = 0;
    for token in name.split('_') {
        if let Some(group) = component_group(token) {
            parsed.groups.push(group);
            pending += 1;
        } else if let Some(numeric_format) = numeric_format_of(token) {
            parsed.numeric_formats.push(numeric_format);
            for _ in 0..pending {
                parsed.group_numeric_formats.push(numeric_format);
            }
            pending = 0;
        } else if let Some(planes) = token.strip_suffix("PLANE") {
            parsed.planes = Some(planes.parse().unwrap());
        } else if let Some(pair) = words(token) {
            parsed.words = Some(pair);
        } else if token == "BLOCK" {
            parsed.block = true;
        } else {
            match token {
                "420" => parsed.chroma = Some(ChromaSubsampling::_420),
                "422" => parsed.chroma = Some(ChromaSubsampling::_422),
                "444" => parsed.chroma = Some(ChromaSubsampling::_444),
                _ => {}
            }
        }
    }
    parsed
}

fn component_name(letter: char) -> ComponentName {
    match letter {
        'R' => ComponentName::R,
        'G' => ComponentName::G,
        'B' => ComponentName::B,
        'A' => ComponentName::A,
        'D' => ComponentName::D,
        'S' => ComponentName::S,
        _ => panic!("{letter} is not a component"),
    }
}

fn stored_numeric_format(letter: char, numeric_format: NumericFormat) -> NumericFormat {
    if letter == 'A' && numeric_format == NumericFormat::Srgb {
        NumericFormat::Unorm
    } else {
        numeric_format
    }
}

fn is_3d_astc(name: &str) -> bool {
    name.starts_with("ASTC_") && name.split('_').nth(1).unwrap().matches('x').count() == 2
}

fn compressed_letters(name: &str, parsed: &ParsedName) -> Vec<(char, Option<u32>)> {
    let family = name.split('_').next().unwrap();
    let letters = match family {
        "BC1" if name.starts_with("BC1_RGBA_") => "RGBA",
        "BC1" | "BC6H" => "RGB",
        "BC2" | "BC3" | "BC7" | "ASTC" => "RGBA",
        "BC4" => "R",
        "BC5" => "RG",
        family if family.starts_with("PVRTC") => "RGBA",
        "ETC2" | "EAC" => {
            return parsed.groups[0]
                .iter()
                .map(|&(letter, bits)| (letter, (family == "EAC").then_some(bits)))
                .collect();
        }
        _ => panic!("{name}: unknown compression family"),
    };
    letters.chars().map(|letter| (letter, None)).collect()
}

fn expected_components(name: &str) -> Vec<Named> {
    let parsed = parse(name);
    if parsed.block {
        let numeric_format = parsed.numeric_formats[0];
        return compressed_letters(name, &parsed)
            .into_iter()
            .map(|(letter, bits)| Named {
                name: component_name(letter),
                bits,
                numeric_format: if is_3d_astc(name) {
                    numeric_format
                } else {
                    stored_numeric_format(letter, numeric_format)
                },
                plane: None,
            })
            .collect();
    }
    let mut components = Vec::new();
    for (index, group) in parsed.groups.iter().enumerate() {
        for &(letter, bits) in group {
            if matches!(letter, 'X' | 'E') {
                continue;
            }
            components.push(Named {
                name: component_name(letter),
                bits: Some(bits),
                numeric_format: stored_numeric_format(letter, parsed.group_numeric_formats[index]),
                plane: parsed.planes.map(|_| u32::try_from(index).unwrap()),
            });
        }
    }
    components
}

fn named_bits(group: &[(char, u32)]) -> u32 {
    group.iter().map(|&(_, bits)| bits).sum()
}

fn formats() -> Vec<(vk::Format, &'static str, &'static FormatInfo)> {
    vk::Format::variants()
        .iter()
        .filter_map(|&(format, name)| Some((format, name, format.info()?)))
        .collect()
}

fn actual_components(info: &FormatInfo) -> Vec<Named> {
    info.components
        .iter()
        .map(|c| Named {
            name: c.name,
            bits: c.bits,
            numeric_format: c.numeric_format,
            plane: c.plane,
        })
        .collect()
}

#[test]
fn every_format_but_undefined_has_info() {
    let all = formats();
    assert!(vk::Format::UNDEFINED.info().is_none());
    assert_eq!(all.len() + 1, vk::Format::variants().len());
    assert!(all.len() >= 297, "{}", all.len());
}

proptest! {
    #![proptest_config(cases(2048))]

    #[test]
    fn only_formats_of_the_registry_have_info(
        raw in prop_oneof![
            prop::sample::select(vk::Format::variants().iter().map(|(f, _)| f.as_raw()).collect::<Vec<_>>()),
            prop::sample::select(vk::Format::variants().iter().map(|(f, _)| f.as_raw().saturating_add(1)).collect::<Vec<_>>()),
            any::<i32>(),
        ]
    ) {
        let format = vk::Format::from_raw(raw);
        prop_assert_eq!(
            format.info().is_some(),
            format.name().is_some() && format != vk::Format::UNDEFINED,
            "{:?}", format
        );
    }
}

#[test]
fn components_are_those_that_the_format_name_spells_out() {
    for (_, name, info) in formats() {
        assert_eq!(
            actual_components(info),
            expected_components(name),
            "{name}: the components are named in order with their width and numeric format; the alpha of an sRGB format is UNORM except in the 3D ASTC formats, whose alpha the registry lists as sRGB; a block-compressed format gives no width except EAC"
        );
    }
}

fn compressed_block(name: &str, parsed: &ParsedName) -> (&'static str, [u32; 3], u32) {
    let tokens: Vec<&str> = name.split('_').collect();
    let family = tokens[0];
    if family == "ASTC" {
        let extent: Vec<u32> = tokens[1].split('x').map(|n| n.parse().unwrap()).collect();
        let scheme = if parsed.numeric_formats[0] == NumericFormat::Sfloat {
            "ASTC HDR"
        } else {
            "ASTC LDR"
        };
        return (
            scheme,
            [extent[0], extent[1], extent.get(2).copied().unwrap_or(1)],
            128,
        );
    }
    if family.starts_with("PVRTC") {
        let extent = if tokens[1] == "2BPP" {
            [8, 4, 1]
        } else {
            [4, 4, 1]
        };
        return ("PVRTC", extent, 64);
    }
    let bits = match family {
        "BC1" | "BC4" => 64,
        "ETC2" if matches!(tokens[1], "R8G8B8" | "R8G8B8A1") => 64,
        "EAC" if tokens[1] == "R11" => 64,
        _ => 128,
    };
    let scheme = match family {
        "ETC2" => "ETC2",
        "EAC" => "EAC",
        _ => "BC",
    };
    (scheme, [4, 4, 1], bits)
}

#[test]
fn texel_blocks_follow_the_format_name() {
    for (_, name, info) in formats() {
        let parsed = parse(name);
        assert_eq!(info.is_compressed(), parsed.block, "{name}: _BLOCK");
        assert_eq!(
            info.packed,
            parsed.words.map(|(_, bits)| bits),
            "{name}: _PACK"
        );
        if parsed.block {
            let (scheme, extent, bits) = compressed_block(name, &parsed);
            assert_eq!(info.compression, Some(scheme), "{name}");
            assert_eq!(info.block_extent, extent, "{name}");
            assert_eq!(
                info.block_size * 8,
                bits,
                "{name}: compressed texel block size"
            );
            let texels = if scheme == "PVRTC" {
                1
            } else {
                extent.iter().product()
            };
            assert_eq!(
                info.texels_per_block, texels,
                "{name}: texels per block, which the registry gives as 1 for PVRTC"
            );
            continue;
        }
        assert_eq!(info.compression, None, "{name}");
        assert_eq!(info.texels_per_block, 1, "{name}");
        let single_plane_422 =
            parsed.chroma == Some(ChromaSubsampling::_422) && parsed.planes.is_none();
        assert_eq!(
            info.block_extent,
            if single_plane_422 {
                [2, 1, 1]
            } else {
                [1, 1, 1]
            },
            "{name}"
        );
        let bits: u32 = parsed.groups.iter().map(|group| named_bits(group)).sum();
        assert_eq!(
            info.block_size * 8,
            bits,
            "{name}: a texel block holds the bits the name lists, padding and shared exponent included"
        );
        if let Some((count, bits)) = parsed.words {
            assert_eq!(
                info.block_size * 8,
                count * bits,
                "{name}: the words of _mPACKnn"
            );
            if count > 1 {
                assert_eq!(
                    info.components.len(),
                    usize::try_from(count).unwrap(),
                    "{name}: one component per word"
                );
            }
        }
    }
}

fn divisors(chroma: Option<ChromaSubsampling>, plane: u32) -> (u32, u32) {
    match (plane, chroma) {
        (0, _) | (_, Some(ChromaSubsampling::_444)) => (1, 1),
        (_, Some(ChromaSubsampling::_422)) => (2, 1),
        (_, Some(ChromaSubsampling::_420)) => (2, 2),
        (_, None) => panic!("a multi-planar format without subsampling"),
    }
}

#[test]
fn planes_follow_the_format_name() {
    let mut multi_planar = 0;
    for (_, name, info) in formats() {
        let parsed = parse(name);
        if let Some(chroma) = parsed.chroma {
            assert_eq!(info.chroma, Some(chroma), "{name}");
        }
        let Some(planes) = parsed.planes else {
            assert!(info.planes.is_empty(), "{name}");
            assert_eq!(info.plane_count(), 1, "{name}");
            assert!(!info.is_multiplanar(), "{name}");
            continue;
        };
        multi_planar += 1;
        assert_eq!(info.plane_count(), planes, "{name}");
        assert_eq!(
            info.planes.len(),
            usize::try_from(planes).unwrap(),
            "{name}"
        );
        assert!(info.is_multiplanar(), "{name}");
        let mut block_size = 0;
        for (index, plane) in (0..).zip(info.planes) {
            assert_eq!(
                (plane.width_divisor, plane.height_divisor),
                divisors(parsed.chroma, index),
                "{name}: plane {index}"
            );
            let compatible = plane.compatible.info().unwrap();
            assert!(
                !compatible.is_multiplanar() && !compatible.is_compressed(),
                "{name}: plane {index}"
            );
            let in_plane: Vec<_> = expected_components(name)
                .into_iter()
                .filter(|c| c.plane == Some(index))
                .map(|c| (c.bits, c.numeric_format))
                .collect();
            let of_compatible: Vec<_> = compatible
                .components
                .iter()
                .map(|c| (c.bits, c.numeric_format))
                .collect();
            assert_eq!(
                of_compatible, in_plane,
                "{name}: plane {index} has the layout of {:?}",
                plane.compatible
            );
            let group = &parsed.groups[usize::try_from(index).unwrap()];
            assert_eq!(
                compatible.block_size * 8,
                named_bits(group),
                "{name}: plane {index}"
            );
            block_size += compatible.block_size;
        }
        assert_eq!(
            info.block_size, block_size,
            "{name}: the block size is the sum of one texel of each plane"
        );
    }
    assert!(multi_planar >= 20, "{multi_planar}");
}

#[test]
fn aspects_and_component_queries_follow_the_format_name() {
    for (_, name, info) in formats() {
        let components = expected_components(name);
        let has = |letter: ComponentName| components.iter().any(|c| c.name == letter);
        assert_eq!(info.has_depth(), has(ComponentName::D), "{name}");
        assert_eq!(info.has_stencil(), has(ComponentName::S), "{name}");
        assert_eq!(info.has_alpha(), has(ComponentName::A), "{name}");
        let mut aspects = ImageAspectFlags::empty();
        if has(ComponentName::D) {
            aspects |= ImageAspectFlags::DEPTH;
        }
        if has(ComponentName::S) {
            aspects |= ImageAspectFlags::STENCIL;
        }
        if aspects.is_empty() {
            aspects = ImageAspectFlags::COLOR;
        }
        assert_eq!(info.aspects(), aspects, "{name}");
        assert_eq!(
            info.is_srgb(),
            parse(name).numeric_formats.contains(&NumericFormat::Srgb),
            "{name}"
        );
        let first = components[0].numeric_format;
        let shared = components
            .iter()
            .all(|c| c.numeric_format == first)
            .then_some(first);
        assert_eq!(info.numeric_format(), shared, "{name}");
    }
}

fn spec_shader_type(numeric_format: NumericFormat) -> Option<ShaderNumericType> {
    match numeric_format {
        NumericFormat::Unorm
        | NumericFormat::Snorm
        | NumericFormat::Uscaled
        | NumericFormat::Sscaled
        | NumericFormat::Ufloat
        | NumericFormat::Sfloat
        | NumericFormat::Srgb => Some(ShaderNumericType::Float),
        NumericFormat::Uint => Some(ShaderNumericType::Uint),
        NumericFormat::Sint | NumericFormat::Sfixed5 => Some(ShaderNumericType::Sint),
        NumericFormat::Bool => None,
        other => panic!("{other:?} has no numeric type in the table of the specification"),
    }
}

#[test]
fn numeric_formats_are_normalized_exactly_when_the_specification_normalizes_them() {
    for numeric_format in [
        NumericFormat::Unorm,
        NumericFormat::Snorm,
        NumericFormat::Uscaled,
        NumericFormat::Sscaled,
        NumericFormat::Uint,
        NumericFormat::Sint,
        NumericFormat::Ufloat,
        NumericFormat::Sfloat,
        NumericFormat::Srgb,
        NumericFormat::Sfixed5,
        NumericFormat::Bool,
    ] {
        assert_eq!(
            numeric_format.is_normalized(),
            matches!(
                numeric_format,
                NumericFormat::Unorm | NumericFormat::Snorm | NumericFormat::Srgb
            ),
            "{numeric_format:?}"
        );
    }
}

const SINGLE_ASPECTS: [ImageAspectFlags; 6] = [
    ImageAspectFlags::COLOR,
    ImageAspectFlags::DEPTH,
    ImageAspectFlags::STENCIL,
    ImageAspectFlags::PLANE_0,
    ImageAspectFlags::PLANE_1,
    ImageAspectFlags::PLANE_2,
];

fn in_aspect(component: &Named, aspect: ImageAspectFlags) -> bool {
    let depth_or_stencil = matches!(component.name, ComponentName::D | ComponentName::S);
    [
        (ImageAspectFlags::COLOR, !depth_or_stencil),
        (ImageAspectFlags::DEPTH, component.name == ComponentName::D),
        (
            ImageAspectFlags::STENCIL,
            component.name == ComponentName::S,
        ),
        (ImageAspectFlags::PLANE_0, component.plane == Some(0)),
        (ImageAspectFlags::PLANE_1, component.plane == Some(1)),
        (ImageAspectFlags::PLANE_2, component.plane == Some(2)),
    ]
    .contains(&(aspect, true))
}

#[test]
fn shader_type_is_the_numeric_type_of_the_components_of_the_aspect() {
    let mut formats_with_aspect = [0; 6];
    let other_aspects = [
        ImageAspectFlags::empty(),
        ImageAspectFlags::METADATA,
        ImageAspectFlags::COLOR | ImageAspectFlags::DEPTH,
        ImageAspectFlags::DEPTH | ImageAspectFlags::STENCIL,
        ImageAspectFlags::PLANE_0 | ImageAspectFlags::PLANE_1,
        ImageAspectFlags::MEMORY_PLANE_0_EXT,
    ];
    for (_, name, info) in formats() {
        let components = expected_components(name);
        for (index, aspect) in SINGLE_ASPECTS.into_iter().enumerate() {
            let types: Vec<_> = components
                .iter()
                .filter(|c| in_aspect(c, aspect))
                .map(|c| spec_shader_type(c.numeric_format))
                .collect();
            assert!(
                types.windows(2).all(|pair| pair[0] == pair[1]),
                "{name}: the components of {aspect:?} have one numeric type"
            );
            let expected = types.first().copied().flatten();
            assert_eq!(info.shader_type(aspect), expected, "{name}: {aspect:?}");
            if expected.is_some() {
                formats_with_aspect[index] += 1;
            }
        }
        for aspect in other_aspects {
            assert_eq!(info.shader_type(aspect), None, "{name}: {aspect:?}");
        }
    }
    assert!(
        formats_with_aspect.iter().all(|&count| count > 0),
        "formats with a shader type for [COLOR, DEPTH, STENCIL, PLANE_0, PLANE_1, PLANE_2] = {formats_with_aspect:?}"
    );
}

#[test]
fn plane_aspect_selects_each_plane_of_a_multi_planar_format() {
    let plane_aspects = [
        ImageAspectFlags::PLANE_0,
        ImageAspectFlags::PLANE_1,
        ImageAspectFlags::PLANE_2,
    ];
    for (_, name, info) in formats() {
        let planes = parse(name).planes.unwrap_or(0);
        for plane in [0, 1, 2, 3, u32::MAX] {
            let expected = (plane < planes).then(|| plane_aspects[usize::try_from(plane).unwrap()]);
            assert_eq!(info.plane_aspect(plane), expected, "{name}: plane {plane}");
        }
    }
}

fn extent_part() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0),
        Just(1),
        Just(2),
        Just(3),
        Just(u32::MAX - 1),
        Just(u32::MAX),
        any::<u32>().prop_map(|n| n | 1),
        any::<u32>(),
    ]
}

fn plane_index() -> impl Strategy<Value = u32> {
    prop_oneof![
        4 => 0..4u32,
        1 => Just(u32::MAX),
        1 => any::<u32>(),
    ]
}

fn any_format() -> impl Strategy<Value = vk::Format> {
    let all: Vec<vk::Format> = formats().into_iter().map(|(format, _, _)| format).collect();
    let multi_planar: Vec<vk::Format> = formats()
        .into_iter()
        .filter(|(_, _, info)| info.is_multiplanar())
        .map(|(format, _, _)| format)
        .collect();
    prop_oneof![
        prop::sample::select(all),
        prop::sample::select(multi_planar),
    ]
}

fn expected_plane_extent(format: vk::Format, plane: u32, extent: Extent2D) -> Option<Extent2D> {
    let parsed = parse(format.name().unwrap());
    match parsed.planes {
        None => (plane == 0).then_some(extent),
        Some(planes) if plane < planes => {
            let (width, height) = divisors(parsed.chroma, plane);
            Some(Extent2D {
                width: extent.width / width,
                height: extent.height / height,
            })
        }
        Some(_) => None,
    }
}

proptest! {
    #![proptest_config(cases(2048))]

    #[test]
    fn plane_extent_divides_the_extent_by_the_subsampling_of_the_plane_rounding_down(
        format in any_format(),
        plane in plane_index(),
        width in extent_part(),
        height in extent_part(),
    ) {
        let extent = Extent2D { width, height };
        let info = format.info().unwrap();
        prop_assert_eq!(
            info.plane_extent(plane, extent).map(|e| (e.width, e.height)),
            expected_plane_extent(format, plane, extent).map(|e| (e.width, e.height)),
            "{:?} plane {}", format, plane
        );
    }
}

#[test]
fn generated_plane_extents_cover_every_case() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = (any_format(), plane_index(), extent_part(), extent_part());
    let mut counts = [0u32; 6];
    for _ in 0..2000 {
        let (format, plane, width, height) = strategy.new_tree(&mut runner).unwrap().current();
        let planes = format.info().unwrap().plane_count();
        let multi_planar = format.info().unwrap().is_multiplanar();
        let kind = match (multi_planar, plane) {
            (false, 0) => 0,
            (false, _) => 1,
            (true, 0) => 2,
            (true, plane) if plane < planes && (width % 2 == 1 || height % 2 == 1) => 3,
            (true, plane) if plane < planes => 4,
            (true, _) => 5,
        };
        counts[kind] += 1;
    }
    assert!(
        counts.iter().all(|&count| count >= 40),
        "[single plane 0, single plane other, multi plane 0, multi chroma odd extent, multi chroma even extent, multi out of range] = {counts:?}"
    );
}

#[test]
fn formats_of_one_class_have_one_block_size_and_extent() {
    let mut classes = BTreeMap::<&str, Vec<(&str, u32, [u32; 3])>>::new();
    for (_, name, info) in formats() {
        classes
            .entry(info.class)
            .or_default()
            .push((name, info.block_size, info.block_extent));
    }
    for (class, members) in &classes {
        assert!(
            members
                .iter()
                .all(|&(_, size, extent)| (size, extent) == (members[0].1, members[0].2)),
            "class {class}: {members:?}"
        );
    }
    assert!(classes.len() >= 90, "{}", classes.len());
}

#[test]
fn format_info_can_be_read_in_a_constant() {
    const BLOCK: u32 = match vk::Format::R16G16B16A16_SFLOAT.info() {
        Some(info) => info.block_size,
        None => 0,
    };
    assert_eq!(BLOCK, 8);
}
