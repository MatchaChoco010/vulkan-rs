use std::collections::BTreeSet;
use std::path::PathBuf;
use vulkan::vk::{self, ImageAspectFlags, NumericFormat, ShaderNumericType};

struct Row {
    numeric_format: &'static str,
    type_declaration: &'static str,
    numeric_type: &'static str,
    rust: NumericFormat,
}

const fn row(
    numeric_format: &'static str,
    type_declaration: &'static str,
    numeric_type: &'static str,
    rust: NumericFormat,
) -> Row {
    Row {
        numeric_format,
        type_declaration,
        numeric_type,
        rust,
    }
}

const INTERPRETATION_OF_NUMERIC_FORMAT: [Row; 11] = [
    row(
        "UNORM",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Unorm,
    ),
    row(
        "SNORM",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Snorm,
    ),
    row(
        "USCALED",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Uscaled,
    ),
    row(
        "SSCALED",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Sscaled,
    ),
    row("UINT", "OpTypeInt", "unsigned integer", NumericFormat::Uint),
    row("SINT", "OpTypeInt", "signed integer", NumericFormat::Sint),
    row(
        "UFLOAT",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Ufloat,
    ),
    row(
        "SFLOAT",
        "OpTypeFloat",
        "floating-point",
        NumericFormat::Sfloat,
    ),
    row("SRGB", "OpTypeFloat", "floating-point", NumericFormat::Srgb),
    row(
        "SFIXED5",
        "OpTypeInt",
        "scaled signed integer",
        NumericFormat::Sfixed5,
    ),
    row("BOOL", "OpTypeBool", "boolean", NumericFormat::Bool),
];

fn shader_type_of(row: &Row) -> Option<ShaderNumericType> {
    match (row.type_declaration, row.numeric_type) {
        ("OpTypeFloat", "floating-point") => Some(ShaderNumericType::Float),
        ("OpTypeInt", "unsigned integer") => Some(ShaderNumericType::Uint),
        ("OpTypeInt", "signed integer" | "scaled signed integer") => Some(ShaderNumericType::Sint),
        ("OpTypeBool", "boolean") => None,
        other => panic!(
            "{}: the table has no Rust type for {other:?}",
            row.numeric_format
        ),
    }
}

#[test]
fn every_numeric_format_is_read_as_the_type_that_its_row_of_the_specification_declares() {
    for row in &INTERPRETATION_OF_NUMERIC_FORMAT {
        assert_eq!(
            row.rust.shader_type(),
            shader_type_of(row),
            "{}: the specification gives the type-declaration instruction {} and the numeric type `{}`",
            row.numeric_format,
            row.type_declaration,
            row.numeric_type
        );
    }
    let listed = INTERPRETATION_OF_NUMERIC_FORMAT
        .iter()
        .map(|row| format!("{:?}", row.rust))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        listed.len(),
        INTERPRETATION_OF_NUMERIC_FORMAT.len(),
        "every row has a numeric format of its own"
    );
}

#[test]
fn sfixed5_is_read_as_a_signed_integer() {
    assert_eq!(
        NumericFormat::Sfixed5.shader_type(),
        Some(ShaderNumericType::Sint),
        "SFIXED5 has the type-declaration instruction OpTypeInt and the numeric type `scaled signed integer`"
    );
    for format in [vk::Format::R16G16_SFIXED5_NV, vk::Format::R16G16_S10_5_NV] {
        let info = format.info().expect("the registry describes the format");
        assert_eq!(info.numeric_format(), Some(NumericFormat::Sfixed5));
        assert_eq!(
            info.shader_type(ImageAspectFlags::COLOR),
            Some(ShaderNumericType::Sint),
            "{format:?}"
        );
    }
}

fn formats_of_the_registry() -> Vec<vk::Format> {
    let core = 1..1000;
    let of_extensions = (0..1000).flat_map(|extension| {
        let first = 1_000_000_000 + extension * 1000;
        first..first + 1000
    });
    core.chain(of_extensions)
        .map(vk::Format::from_raw)
        .filter(|format| format.info().is_some())
        .collect()
}

#[test]
fn every_component_of_every_format_is_read_as_the_type_of_its_row() {
    let formats = formats_of_the_registry();
    assert!(formats.len() >= 250, "only {} formats", formats.len());
    let mut seen = BTreeSet::new();
    for format in formats {
        let info = format.info().expect("a format of the registry");
        for component in info.components {
            let row = INTERPRETATION_OF_NUMERIC_FORMAT
                .iter()
                .find(|row| row.rust == component.numeric_format)
                .unwrap_or_else(|| {
                    panic!(
                        "{format:?}: the numeric format {:?} has no row",
                        component.numeric_format
                    )
                });
            assert_eq!(
                component.numeric_format.shader_type(),
                shader_type_of(row),
                "{format:?}"
            );
            seen.insert(row.numeric_format);
        }
        let aspects = [
            ImageAspectFlags::COLOR,
            ImageAspectFlags::DEPTH,
            ImageAspectFlags::STENCIL,
            ImageAspectFlags::PLANE_0,
            ImageAspectFlags::PLANE_1,
            ImageAspectFlags::PLANE_2,
        ];
        for aspect in aspects {
            let Some(shader_type) = info.shader_type(aspect) else {
                continue;
            };
            assert!(
                info.components
                    .iter()
                    .any(|component| component.numeric_format.shader_type() == Some(shader_type)),
                "{format:?}: {aspect:?} is read as {shader_type:?}, the type of none of its components"
            );
        }
    }
    let rows = INTERPRETATION_OF_NUMERIC_FORMAT
        .iter()
        .map(|row| row.numeric_format)
        .collect::<BTreeSet<_>>();
    assert_eq!(seen, rows, "a format of the registry uses every row");
}

#[test]
fn the_rows_are_the_rows_of_the_table_in_the_specification() {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../third_party/vulkan-docs/chapters/formats.adoc");
    let Ok(text) = std::fs::read_to_string(&source) else {
        assert!(
            std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
            "the documentation test is required but third_party/vulkan-docs is not checked out"
        );
        eprintln!("skipping: third_party/vulkan-docs is not checked out");
        return;
    };
    let table = text
        .split_once("[[formats-numericformat]]")
        .expect("the table `Interpretation of Numeric Format`")
        .1;
    let (_, table) = table.split_once("|====").expect("the start of the table");
    let (table, _) = table.split_once("|====").expect("the end of the table");
    let rows = table
        .lines()
        .filter_map(|line| line.strip_prefix("| etext:"))
        .map(|line| {
            let cells = line.split('|').map(str::trim).collect::<Vec<_>>();
            assert_eq!(cells.len(), 4, "{line}");
            (cells[0], cells[1], cells[2])
        })
        .collect::<Vec<_>>();
    let expected = INTERPRETATION_OF_NUMERIC_FORMAT
        .iter()
        .map(|row| (row.numeric_format, row.type_declaration, row.numeric_type))
        .collect::<Vec<_>>();
    assert_eq!(rows, expected);
}
