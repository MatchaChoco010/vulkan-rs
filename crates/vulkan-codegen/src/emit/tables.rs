//! Lookup tables generated from registry data that the C headers do not carry: the
//! texel layout of formats and the dependencies of extensions.

use super::{header, w};
use crate::model::{Depends, Model, parse_version};
use std::collections::BTreeMap;
use vulkan_registry::Extension;

pub fn emit_formats(model: &Model<'_>) -> String {
    let format_enum = model
        .ir
        .enums
        .iter()
        .find(|e| e.name == "VkFormat")
        .expect("VkFormat");
    let mut values = BTreeMap::new();
    for field in &format_enum.fields {
        values.insert(field.name.as_str(), field.value.as_str());
        for alias in &field.aliases {
            values.insert(alias.as_str(), field.value.as_str());
        }
    }
    let value = |name: &str| {
        *values
            .get(name)
            .unwrap_or_else(|| panic!("format info for unknown format {name}"))
    };
    // Every format has to come with its layout, or `Format::info` silently has a hole.
    for field in &format_enum.fields {
        assert!(
            field.name == "VK_FORMAT_UNDEFINED"
                || model.ir.formats.iter().any(|f| f.name == field.name),
            "{} has no <format> entry in the registry",
            field.name
        );
    }

    let mut out = header(&[]);
    w!(out, "impl Format {{");
    w!(
        out,
        "    /// Texel layout of this format, from the registry."
    );
    w!(out, "    ///");
    w!(
        out,
        "    /// `None` for `UNDEFINED` and for values that these bindings do not know."
    );
    w!(
        out,
        "    pub const fn info(self) -> Option<&'static FormatInfo> {{"
    );
    w!(out, "        match self.0 {{");
    for f in &model.ir.formats {
        let extent = match f.block_extent.as_slice() {
            [] => [1, 1, 1],
            [x, y, z] => [*x, *y, *z],
            other => panic!("{}: block extent {other:?}", f.name),
        };
        let chroma = match f.chroma.as_deref() {
            None => "None".to_string(),
            Some(chroma @ ("420" | "422" | "444")) => {
                format!("Some(ChromaSubsampling::_{chroma})")
            }
            Some(other) => panic!("{}: unknown chroma subsampling {other}", f.name),
        };
        w!(out, "            // {}", f.name);
        w!(out, "            {} => Some(&FormatInfo {{", value(&f.name));
        w!(out, "                class: \"{}\",", f.class);
        w!(out, "                block_size: {},", f.block_size);
        w!(
            out,
            "                texels_per_block: {},",
            f.texels_per_block
        );
        w!(out, "                block_extent: {extent:?},");
        w!(out, "                packed: {:?},", f.packed);
        w!(out, "                chroma: {chroma},");
        w!(out, "                compression: {:?},", f.compressed);
        w!(out, "                components: &[");
        for c in &f.components {
            assert!(
                matches!(c.name.as_str(), "R" | "G" | "B" | "A" | "D" | "S"),
                "{}: unknown component {}",
                f.name,
                c.name
            );
            let numeric = match c.numeric_format.as_str() {
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
            w!(
                out,
                "                    FormatComponent {{ name: ComponentName::{}, bits: {:?}, numeric_format: NumericFormat::{numeric}, plane: {:?} }},",
                c.name,
                c.bits,
                c.plane_index
            );
        }
        w!(out, "                ],");
        w!(out, "                planes: &[");
        for (index, plane) in f.planes.iter().enumerate() {
            assert_eq!(plane.index as usize, index, "{}: plane order", f.name);
            w!(
                out,
                "                    FormatPlane {{ width_divisor: {}, height_divisor: {}, compatible: Format({}) }},",
                plane.width_divisor,
                plane.height_divisor,
                value(&plane.compatible)
            );
        }
        w!(out, "                ],");
        w!(out, "            }}),");
    }
    w!(out, "            _ => None,");
    w!(out, "        }}\n    }}\n}}");
    out
}

fn version_expr(name: &str) -> Option<String> {
    let (major, minor) = parse_version(name)?;
    Some(format!("make_api_version(0, {major}, {minor}, 0)"))
}

fn depends_expr(depends: &Depends) -> String {
    let list = |items: &[Depends]| {
        items
            .iter()
            .map(depends_expr)
            .collect::<Vec<_>>()
            .join(", ")
    };
    match depends {
        Depends::Version(major, minor) => {
            format!("Dependency::Version(make_api_version(0, {major}, {minor}, 0))")
        }
        Depends::Extension(name) => format!("Dependency::Extension(c\"{name}\")"),
        Depends::All(all) => format!("Dependency::All(&[{}])", list(all)),
        Depends::Any(any) => format!("Dependency::Any(&[{}])", list(any)),
    }
}

/// The `EXTENSIONS` table. It is not gated on platform features: a checking layer wants
/// to know about every extension a driver may report.
pub fn emit_extension_table(model: &Model<'_>) -> String {
    let mut extensions = model.ir.extensions.iter().collect::<Vec<&Extension>>();
    // `ExtensionInfo::get` does a binary search with `CStr`'s ordering, which is bytewise.
    extensions.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

    let mut out = String::new();
    w!(
        out,
        "/// Every extension of the registry, sorted by name. See [`ExtensionInfo::get`]."
    );
    w!(out, "pub const EXTENSIONS: &[ExtensionInfo] = &[");
    for ext in extensions {
        let level = match (ext.instance, ext.device) {
            (true, false) => "Instance",
            (false, true) => "Device",
            other => panic!("{}: instance/device is {other:?}", ext.name),
        };
        let depends = match ext.depends.as_deref() {
            Some(text) => format!("Some({})", depends_expr(&Depends::parse(text))),
            None => "None".to_string(),
        };
        let (promoted_version, promoted_extension) = match ext.promoted_to.as_deref() {
            None => ("None".to_string(), "None".to_string()),
            Some(name) => match version_expr(name) {
                Some(version) => (format!("Some({version})"), "None".to_string()),
                None => ("None".to_string(), format!("Some(c\"{name}\")")),
            },
        };
        w!(out, "    ExtensionInfo {{");
        w!(out, "        name: c\"{}\",", ext.name);
        w!(out, "        spec_version: {},", ext.spec_version);
        w!(out, "        level: ExtensionLevel::{level},");
        w!(out, "        depends: {depends},");
        w!(out, "        promoted_to_version: {promoted_version},");
        w!(out, "        promoted_to_extension: {promoted_extension},");
        w!(out, "    }},");
    }
    w!(out, "];");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::{Enum, EnumField, Format, FormatComponent, FormatPlane};

    fn squash(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn formats() -> vulkan_registry::Registry {
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
        let out = squash(&emit_formats(&Model::new(&formats())));
        assert!(out.contains("pub const fn info(self) -> Option<&'static FormatInfo> {"));
        assert!(out.contains(
            "// VK_FORMAT_G8_B8R8_2PLANE_420_UNORM 1000156003 => Some(&FormatInfo { class: \"8-bit 2-plane 420\", block_size: 3, texels_per_block: 1, block_extent: [1, 1, 1], packed: None, chroma: Some(ChromaSubsampling::_420), compression: None,"
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
        emit_formats(&Model::new(&ir));
    }

    #[test]
    #[should_panic(expected = "unknown numeric format")]
    fn an_unknown_numeric_format_aborts_generation() {
        let mut ir = formats();
        ir.formats[2].components[0].numeric_format = "UFIXED".to_string();
        emit_formats(&Model::new(&ir));
    }

    #[test]
    fn extension_table_is_sorted_and_carries_dependencies() {
        let mut ir = registry();
        let extension =
            |name: &str, device: bool, depends: Option<&str>, promoted: Option<&str>| Extension {
                name: name.to_string(),
                spec_version: 1,
                instance: !device,
                device,
                depends: depends.map(str::to_string),
                promoted_to: promoted.map(str::to_string),
                ..Default::default()
            };
        ir.extensions = vec![
            extension("VK_KHR_swapchain", true, Some("VK_KHR_surface"), None),
            extension(
                "VK_KHR_maintenance5",
                true,
                Some("(VK_VERSION_1_1+VK_KHR_dynamic_rendering),VK_VERSION_1_3"),
                Some("VK_VERSION_1_4"),
            ),
            extension(
                "VK_EXT_debug_report",
                false,
                None,
                Some("VK_EXT_debug_utils"),
            ),
        ];
        let out = squash(&emit_extension_table(&Model::new(&ir)));
        let position = |name: &str| out.find(name).expect(name);
        assert!(position("c\"VK_EXT_debug_report\"") < position("name: c\"VK_KHR_maintenance5\""));
        assert!(
            position("name: c\"VK_KHR_maintenance5\"") < position("name: c\"VK_KHR_swapchain\"")
        );
        assert!(out.contains(
            "name: c\"VK_KHR_swapchain\", spec_version: 1, level: ExtensionLevel::Device, depends: Some(Dependency::Extension(c\"VK_KHR_surface\")), promoted_to_version: None, promoted_to_extension: None,"
        ));
        assert!(out.contains(
            "depends: Some(Dependency::Any(&[Dependency::All(&[Dependency::Version(make_api_version(0, 1, 1, 0)), Dependency::Extension(c\"VK_KHR_dynamic_rendering\")]), Dependency::Version(make_api_version(0, 1, 3, 0))])), promoted_to_version: Some(make_api_version(0, 1, 4, 0)),"
        ));
        assert!(out.contains(
            "level: ExtensionLevel::Instance, depends: None, promoted_to_version: None, promoted_to_extension: Some(c\"VK_EXT_debug_utils\"),"
        ));
    }
}
