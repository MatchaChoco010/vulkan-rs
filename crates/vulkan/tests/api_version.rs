use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use std::path::PathBuf;
use vulkan::vk;

const VARIANT_BITS: u32 = 3;
const MAJOR_BITS: u32 = 7;
const MINOR_BITS: u32 = 10;
const PATCH_BITS: u32 = 12;

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

fn spec_version(variant: u64, major: u64, minor: u64, patch: u64) -> u64 {
    variant * (1 << 29) + major * (1 << 22) + minor * (1 << 12) + patch
}

fn largest(bits: u32) -> u32 {
    (1 << bits) - 1
}

fn fitting(bits: u32) -> impl Strategy<Value = u32> {
    let max = largest(bits);
    prop_oneof![Just(0), Just(1), Just(max - 1), Just(max), 0..=max,]
}

fn any_field(bits: u32) -> impl Strategy<Value = u32> {
    let max = largest(bits);
    prop_oneof![
        2 => fitting(bits),
        1 => Just(max + 1),
        1 => Just(2 * max + 1),
        1 => Just(u32::MAX),
        1 => (max + 1)..=u32::MAX,
    ]
}

fn fitting_fields() -> impl Strategy<Value = (u32, u32, u32, u32)> {
    (
        fitting(VARIANT_BITS),
        fitting(MAJOR_BITS),
        fitting(MINOR_BITS),
        fitting(PATCH_BITS),
    )
}

fn any_fields() -> impl Strategy<Value = (u32, u32, u32, u32)> {
    (
        any_field(VARIANT_BITS),
        any_field(MAJOR_BITS),
        any_field(MINOR_BITS),
        any_field(PATCH_BITS),
    )
}

fn any_version() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0),
        Just(u32::MAX),
        (0..32u32).prop_map(|bit| 1 << bit),
        (0..32u32).prop_map(|bit| (1u32 << bit).wrapping_sub(1)),
        any::<u32>(),
    ]
}

fn close_fields() -> impl Strategy<Value = (u32, u32, u32)> {
    (
        prop::sample::select(vec![0, 1, largest(MAJOR_BITS)]),
        prop::sample::select(vec![0, largest(MINOR_BITS)]),
        prop::sample::select(vec![0, largest(PATCH_BITS)]),
    )
}

fn placed(field: u32, shift: u32) -> u32 {
    ((u64::from(field) * (1 << shift)) % (1 << 32)) as u32
}

proptest! {
    #![proptest_config(cases(2048))]

    #[test]
    fn fields_that_fit_pack_to_the_specified_value_and_unpack_to_themselves(
        (variant, major, minor, patch) in fitting_fields()
    ) {
        let version = vk::make_api_version(variant, major, minor, patch);
        prop_assert_eq!(
            u64::from(version),
            spec_version(variant.into(), major.into(), minor.into(), patch.into()),
            "VK_MAKE_API_VERSION({}, {}, {}, {})", variant, major, minor, patch
        );
        prop_assert_eq!(vk::api_version_variant(version), variant);
        prop_assert_eq!(vk::api_version_major(version), major);
        prop_assert_eq!(vk::api_version_minor(version), minor);
        prop_assert_eq!(vk::api_version_patch(version), patch);
    }

    #[test]
    fn every_version_unpacks_to_fields_that_pack_back_to_it(version in any_version()) {
        let fields = (
            vk::api_version_variant(version),
            vk::api_version_major(version),
            vk::api_version_minor(version),
            vk::api_version_patch(version),
        );
        prop_assert!(fields.0 <= largest(VARIANT_BITS), "{:?}", fields);
        prop_assert!(fields.1 <= largest(MAJOR_BITS), "{:?}", fields);
        prop_assert!(fields.2 <= largest(MINOR_BITS), "{:?}", fields);
        prop_assert!(fields.3 <= largest(PATCH_BITS), "{:?}", fields);
        prop_assert_eq!(
            vk::make_api_version(fields.0, fields.1, fields.2, fields.3),
            version,
            "{:#x} unpacks to {:?}", version, fields
        );
    }

    #[test]
    fn versions_of_one_variant_compare_like_their_major_minor_and_patch(
        variant in fitting(VARIANT_BITS),
        a in close_fields(),
        b in close_fields(),
    ) {
        let left = vk::make_api_version(variant, a.0, a.1, a.2);
        let right = vk::make_api_version(variant, b.0, b.1, b.2);
        prop_assert_eq!(left.cmp(&right), a.cmp(&b), "{:?} and {:?}", a, b);
    }

    #[test]
    fn fields_are_not_masked_but_placed_at_their_shift_as_in_c(
        (variant, major, minor, patch) in any_fields()
    ) {
        prop_assert_eq!(
            vk::make_api_version(variant, major, minor, patch),
            placed(variant, 29) | placed(major, 22) | placed(minor, 12) | patch,
            "VK_MAKE_API_VERSION({}, {}, {}, {})", variant, major, minor, patch
        );
    }
}

#[test]
fn fields_that_do_not_fit_spill_into_the_next_field_or_out_of_the_version() {
    let major_too_large = vk::make_api_version(0, 128, 0, 0);
    assert_eq!(
        (
            vk::api_version_variant(major_too_large),
            vk::api_version_major(major_too_large)
        ),
        (1, 0)
    );
    assert_eq!(
        vk::make_api_version(0, 1, 1024, 0),
        vk::make_api_version(0, 1, 0, 0),
        "minor 1024 sets the lowest bit of major, which 1 already sets"
    );
    assert_eq!(
        vk::make_api_version(0, 1, 0, 4096),
        vk::make_api_version(0, 1, 1, 0)
    );
    assert_eq!(
        vk::make_api_version(8, 1, 0, 0),
        vk::make_api_version(0, 1, 0, 0),
        "variant 8 is shifted out of the 32 bits"
    );
}

fn header() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../third_party/vulkan/include/vulkan/vulkan_core.h");
    std::fs::read_to_string(&path).unwrap_or_else(|e| {
        panic!(
            "cannot read {}: {e}; run `git submodule update --init`",
            path.display()
        )
    })
}

fn header_define<'a>(header: &'a str, name: &str) -> Option<&'a str> {
    header.lines().find_map(|line| {
        let rest = line.strip_prefix("#define ")?.strip_prefix(name)?;
        rest.starts_with(' ').then(|| rest.trim())
    })
}

fn header_version(header: &str, definition: &str) -> u64 {
    let arguments = definition
        .strip_prefix("VK_MAKE_API_VERSION(")
        .and_then(|rest| rest.split_once(')'))
        .map(|(arguments, _)| arguments)
        .unwrap_or_else(|| panic!("not a VK_MAKE_API_VERSION: {definition}"));
    let fields: Vec<u64> = arguments
        .split(',')
        .map(|argument| {
            let argument = argument.trim();
            argument.parse().unwrap_or_else(|_| {
                header_define(header, argument)
                    .and_then(|value| value.parse().ok())
                    .unwrap_or_else(|| panic!("{argument} is not a number in the header"))
            })
        })
        .collect();
    assert_eq!(fields.len(), 4, "{definition}");
    spec_version(fields[0], fields[1], fields[2], fields[3])
}

#[test]
fn version_constants_have_the_values_of_the_c_header() {
    let header = header();
    let constants = [
        ("VK_API_VERSION_1_0", vk::API_VERSION_1_0),
        ("VK_API_VERSION_1_1", vk::API_VERSION_1_1),
        ("VK_API_VERSION_1_2", vk::API_VERSION_1_2),
        ("VK_API_VERSION_1_3", vk::API_VERSION_1_3),
        ("VK_API_VERSION_1_4", vk::API_VERSION_1_4),
        ("VK_HEADER_VERSION_COMPLETE", vk::HEADER_VERSION_COMPLETE),
    ];
    for (name, value) in constants {
        let definition = header_define(&header, name)
            .unwrap_or_else(|| panic!("the header does not define {name}"));
        assert_eq!(
            u64::from(value),
            header_version(&header, definition),
            "{name} is {definition}"
        );
    }
    let header_versions: Vec<&str> = header
        .lines()
        .filter_map(|line| line.strip_prefix("#define VK_API_VERSION_1_"))
        .filter_map(|rest| rest.split_whitespace().next())
        .collect();
    assert_eq!(
        header_versions,
        ["0", "1", "2", "3", "4"],
        "every VK_API_VERSION_* of the header has a constant here"
    );
    assert_eq!(
        Some(vk::HEADER_VERSION.to_string().as_str()),
        header_define(&header, "VK_HEADER_VERSION")
    );
}

#[test]
fn generated_fields_reach_the_boundaries_and_overflow() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let widths = [VARIANT_BITS, MAJOR_BITS, MINOR_BITS, PATCH_BITS];
    let mut fitting_counts = [[0u32; 3]; 4];
    let fitting = fitting_fields();
    for _ in 0..1000 {
        let fields = fitting.new_tree(&mut runner).unwrap().current();
        for (index, (value, bits)) in [fields.0, fields.1, fields.2, fields.3]
            .into_iter()
            .zip(widths)
            .enumerate()
        {
            let kind = if value == 0 {
                0
            } else if value == largest(bits) {
                1
            } else {
                2
            };
            fitting_counts[index][kind] += 1;
        }
    }
    let mut overflow_counts = [[0u32; 3]; 4];
    let any = any_fields();
    for _ in 0..1000 {
        let fields = any.new_tree(&mut runner).unwrap().current();
        for (index, (value, bits)) in [fields.0, fields.1, fields.2, fields.3]
            .into_iter()
            .zip(widths)
            .enumerate()
        {
            let kind = if value <= largest(bits) {
                0
            } else if value == largest(bits) + 1 {
                1
            } else {
                2
            };
            overflow_counts[index][kind] += 1;
        }
    }
    let mut ordered = [0u32; 3];
    let close = close_fields();
    for _ in 0..1000 {
        let a = close.new_tree(&mut runner).unwrap().current();
        let b = close.new_tree(&mut runner).unwrap().current();
        ordered[match a.cmp(&b) {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 1,
            std::cmp::Ordering::Greater => 2,
        }] += 1;
    }
    for counts in fitting_counts {
        assert!(
            counts.iter().all(|&count| count >= 100),
            "fields that fit: [zero, largest, between] = {counts:?}"
        );
    }
    for counts in overflow_counts {
        assert!(
            counts.iter().all(|&count| count >= 50),
            "any fields: [fits, largest + 1, larger] = {counts:?}"
        );
    }
    assert!(
        ordered.iter().all(|&count| count >= 50),
        "pairs: [less, equal, greater] = {ordered:?}"
    );
}
