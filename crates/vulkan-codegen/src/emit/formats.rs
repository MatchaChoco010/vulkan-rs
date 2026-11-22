use super::{header, w};
use crate::model::Model;
use std::collections::BTreeMap;
use vulkan_registry::{Format, FormatComponent};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let values = FormatValues::new(model);
    let mut out = header(&[]);
    w!(
        out,
        "impl Format {{
            pub const fn info(self) -> Option<&'static FormatInfo> {{
                match self.0 {{"
    );
    for f in &model.ir.formats {
        w!(
            out,
            "{} => Some(&{}),",
            values.of(&f.name),
            info(f, &values)
        );
    }
    w!(
        out,
        "_ => None,
                }}
            }}
        }}"
    );
    out
}

struct FormatValues<'a> {
    by_name: BTreeMap<&'a str, &'a str>,
}

impl<'a> FormatValues<'a> {
    fn new(model: &Model<'a>) -> Self {
        let fields = &model.get_enum("VkFormat").fields;
        for field in fields {
            assert!(
                field.name == "VK_FORMAT_UNDEFINED"
                    || model.ir.formats.iter().any(|f| f.name == field.name),
                "{} has no <format> entry in the registry",
                field.name
            );
        }
        let by_name = fields
            .iter()
            .flat_map(|field| {
                let names = std::iter::once(&field.name).chain(&field.aliases);
                names.map(|name| (name.as_str(), field.value.as_str()))
            })
            .collect();
        Self { by_name }
    }

    fn of(&self, name: &str) -> &'a str {
        self.by_name
            .get(name)
            .unwrap_or_else(|| panic!("format info for unknown format {name}"))
    }
}

fn info(f: &Format, values: &FormatValues<'_>) -> String {
    let block_extent = match f.block_extent.as_slice() {
        [] => [1, 1, 1],
        [x, y, z] => [*x, *y, *z],
        other => panic!("{}: block extent {other:?}", f.name),
    };
    let chroma = match f.chroma.as_deref() {
        None => "None".to_string(),
        Some(chroma @ ("420" | "422" | "444")) => format!("Some(ChromaSubsampling::_{chroma})"),
        Some(other) => panic!("{}: unknown chroma subsampling {other}", f.name),
    };
    let components = f
        .components
        .iter()
        .map(|c| component(f, c))
        .collect::<String>();
    let planes = f
        .planes
        .iter()
        .enumerate()
        .map(|(index, plane)| {
            assert_eq!(plane.index as usize, index, "{}: plane order", f.name);
            format!(
                "FormatPlane {{ width_divisor: {}, height_divisor: {}, compatible: Format({}) }},\n",
                plane.width_divisor,
                plane.height_divisor,
                values.of(&plane.compatible)
            )
        })
        .collect::<String>();
    format!(
        "FormatInfo {{
            class: \"{class}\",
            block_size: {block_size},
            texels_per_block: {texels_per_block},
            block_extent: {block_extent:?},
            packed: {packed:?},
            chroma: {chroma},
            compression: {compression:?},
            components: &[
                {components}
            ],
            planes: &[
                {planes}
            ],
        }}",
        class = f.class,
        block_size = f.block_size,
        texels_per_block = f.texels_per_block,
        packed = f.packed,
        compression = f.compressed,
    )
}

fn component(f: &Format, c: &FormatComponent) -> String {
    assert!(
        matches!(c.name.as_str(), "R" | "G" | "B" | "A" | "D" | "S"),
        "{}: unknown component {}",
        f.name,
        c.name
    );
    let numeric_format = match c.numeric_format.as_str() {
        "UNORM" => "Unorm",
        "SNORM" => "Snorm",
        "USCALED" => "Uscaled",
        "SSCALED" => "Sscaled",
        "UINT" => "Uint",
        "SINT" => "Sint",
        "UFLOAT" => "Ufloat",
        "SFLOAT" => "Sfloat",
        "SRGB" => "Srgb",
        "SFIXED5" => "Sfixed5",
        "BOOL" => "Bool",
        other => panic!(
            "{}: unknown numeric format {other}; add it to vk::NumericFormat",
            f.name
        ),
    };
    format!(
        "FormatComponent {{ name: ComponentName::{}, bits: {:?}, numeric_format: NumericFormat::{numeric_format}, plane: {:?} }},\n",
        c.name, c.bits, c.plane_index
    )
}

#[cfg(test)]
mod tests {
    use super::super::squash;
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::{Enum, EnumField, FormatPlane, Registry};

    fn formats() -> Registry {
        let mut ir = registry();
        let field = |name: &str, value: &str| EnumField {
            name: name.to_string(),
            value: value.to_string(),
            ..Default::default()
        };
        ir.enums.push(Enum {
            name: "VkFormat".to_string(),
            bit_width: 32,
            fields: vec![
                field("VK_FORMAT_UNDEFINED", "0"),
                field("VK_FORMAT_R8_UNORM", "9"),
                field("VK_FORMAT_R8G8_UNORM", "16"),
                field("VK_FORMAT_G8_B8R8_2PLANE_420_UNORM", "1000156003"),
            ],
            ..Default::default()
        });
        let component = |name: &str, plane: Option<u32>| FormatComponent {
            name: name.to_string(),
            bits: Some(8),
            numeric_format: "UNORM".to_string(),
            plane_index: plane,
        };
        ir.formats = vec![
            Format {
                name: "VK_FORMAT_G8_B8R8_2PLANE_420_UNORM".to_string(),
                class: "8-bit 2-plane 420".to_string(),
                block_size: 3,
                texels_per_block: 1,
                chroma: Some("420".to_string()),
                components: vec![
                    component("G", Some(0)),
                    component("B", Some(1)),
                    component("R", Some(1)),
                ],
                planes: vec![
                    FormatPlane {
                        index: 0,
                        width_divisor: 1,
                        height_divisor: 1,
                        compatible: "VK_FORMAT_R8_UNORM".to_string(),
                    },
                    FormatPlane {
                        index: 1,
                        width_divisor: 2,
                        height_divisor: 2,
                        compatible: "VK_FORMAT_R8G8_UNORM".to_string(),
                    },
                ],
                ..Default::default()
            },
            Format {
                name: "VK_FORMAT_R8G8_UNORM".to_string(),
                class: "16-bit".to_string(),
                block_size: 2,
                texels_per_block: 1,
                components: vec![component("R", None), component("G", None)],
                ..Default::default()
            },
            Format {
                name: "VK_FORMAT_R8_UNORM".to_string(),
                class: "8-bit".to_string(),
                block_size: 1,
                texels_per_block: 1,
                components: vec![component("R", None)],
                ..Default::default()
            },
        ];
        ir
    }

    #[test]
    fn format_table_is_keyed_by_value_and_links_plane_formats() {
        let out = squash(&emit(&Model::new(&formats())));
        assert!(out.contains("pub const fn info(self) -> Option<&'static FormatInfo> {"));
        assert!(out.contains(
            "1000156003 => Some(&FormatInfo { class: \"8-bit 2-plane 420\", block_size: 3, texels_per_block: 1, block_extent: [1, 1, 1], packed: None, chroma: Some(ChromaSubsampling::_420), compression: None,"
        ));
        assert!(out.contains(
            "FormatPlane { width_divisor: 2, height_divisor: 2, compatible: Format(16) },"
        ));
        assert!(out.contains(
            "FormatComponent { name: ComponentName::B, bits: Some(8), numeric_format: NumericFormat::Unorm, plane: Some(1) },"
        ));
        assert!(out.contains("_ => None,"));
    }

    #[test]
    #[should_panic(expected = "has no <format> entry")]
    fn a_format_without_layout_aborts_generation() {
        let mut ir = formats();
        ir.formats.pop();
        emit(&Model::new(&ir));
    }

    #[test]
    #[should_panic(expected = "unknown numeric format")]
    fn an_unknown_numeric_format_aborts_generation() {
        let mut ir = formats();
        ir.formats[2].components[0].numeric_format = "UFIXED".to_string();
        emit(&Model::new(&ir));
    }
}
