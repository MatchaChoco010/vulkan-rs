use super::{header, w};
use crate::model::{Depends, Model, parse_version};
use crate::naming::{const_name, extension_name_const};
use vulkan_registry::Extension;

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    out.push_str("use core::ffi::CStr;\n\n");
    for ext in &model.ir.extensions {
        let cfg = model.cfg(ext.protect.as_deref());
        w!(
            out,
            "{cfg}pub const {name_const}: &CStr = c\"{name}\";
{cfg}pub const {version_const}: u32 = {version};\n",
            name_const = extension_name_const(&ext.name_macro),
            name = ext.name,
            version_const = const_name(&ext.spec_version_macro),
            version = ext.spec_version,
        );
    }
    out.push_str(&lookup_table(model));
    out
}

pub(crate) fn api_version_expr(version_name: &str) -> Option<String> {
    let (major, minor) = parse_version(version_name)?;
    Some(format!("make_api_version(0, {major}, {minor}, 0)"))
}

fn option(value: Option<String>) -> String {
    match value {
        Some(value) => format!("Some({value})"),
        None => "None".to_string(),
    }
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

fn level(ext: &Extension) -> &'static str {
    match (ext.instance, ext.device) {
        (true, false) => "Instance",
        (false, true) => "Device",
        other => panic!("{}: instance/device is {other:?}", ext.name),
    }
}

fn lookup_table(model: &Model<'_>) -> String {
    let mut extensions = model.ir.extensions.iter().collect::<Vec<_>>();
    extensions.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

    let mut out = String::new();
    w!(out, "pub const EXTENSIONS: &[ExtensionInfo] = &[");
    for ext in extensions {
        let depends = ext
            .depends
            .as_deref()
            .map(|text| depends_expr(&Depends::parse(text)));
        let promoted_to_version = ext.promoted_to.as_deref().and_then(api_version_expr);
        let promoted_to_extension = ext
            .promoted_to
            .as_deref()
            .filter(|_| promoted_to_version.is_none())
            .map(|name| format!("c\"{name}\""));
        w!(
            out,
            "ExtensionInfo {{
                name: c\"{name}\",
                spec_version: {spec_version},
                level: ExtensionLevel::{level},
                depends: {depends},
                promoted_to_version: {promoted_to_version},
                promoted_to_extension: {promoted_to_extension},
            }},",
            name = ext.name,
            spec_version = ext.spec_version,
            level = level(ext),
            depends = option(depends),
            promoted_to_version = option(promoted_to_version),
            promoted_to_extension = option(promoted_to_extension),
        );
    }
    w!(out, "];");
    out
}

#[cfg(test)]
mod tests {
    use super::super::squash;
    use super::*;
    use crate::test_support::registry;

    #[test]
    fn extension_table_is_sorted_and_carries_dependencies() {
        let mut ir = registry();
        let extension =
            |name: &str, device: bool, depends: Option<&str>, promoted: Option<&str>| Extension {
                name: name.to_string(),
                name_macro: format!("{}_EXTENSION_NAME", name.to_uppercase()),
                spec_version_macro: format!("{}_SPEC_VERSION", name.to_uppercase()),
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
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub const KHR_SWAPCHAIN_NAME: &CStr = c\"VK_KHR_swapchain\";"));
        assert!(out.contains("pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 1;"));
        let table = &out[out.find("pub const EXTENSIONS").expect("lookup table")..];
        let position = |name: &str| table.find(name).expect(name);
        assert!(
            position("name: c\"VK_EXT_debug_report\"") < position("name: c\"VK_KHR_maintenance5\"")
        );
        assert!(
            position("name: c\"VK_KHR_maintenance5\"") < position("name: c\"VK_KHR_swapchain\"")
        );
        assert!(table.contains(
            "name: c\"VK_KHR_swapchain\", spec_version: 1, level: ExtensionLevel::Device, depends: Some(Dependency::Extension(c\"VK_KHR_surface\")), promoted_to_version: None, promoted_to_extension: None,"
        ));
        assert!(table.contains(
            "depends: Some(Dependency::Any(&[Dependency::All(&[Dependency::Version(make_api_version(0, 1, 1, 0)), Dependency::Extension(c\"VK_KHR_dynamic_rendering\")]), Dependency::Version(make_api_version(0, 1, 3, 0))])), promoted_to_version: Some(make_api_version(0, 1, 4, 0)),"
        ));
        assert!(table.contains(
            "level: ExtensionLevel::Instance, depends: None, promoted_to_version: None, promoted_to_extension: Some(c\"VK_EXT_debug_utils\"),"
        ));
    }
}
