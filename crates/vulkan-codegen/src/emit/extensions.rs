use super::{header, w};
use crate::docs::{Doc, join_and, join_or};
use crate::model::{Cfg, Depends, Model, named_version, parse_version};
use crate::naming::{const_name, extension_name_const};
use vulkan_registry::Extension;

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    out.push_str("use core::ffi::CStr;\n\n");
    for ext in &model.ir.extensions {
        let cfg = model.cfg(ext.protect.as_deref());
        w!(
            out,
            "{name_doc}{cfg}pub const {name_const}: &CStr = c\"{name}\";
{version_doc}{cfg}pub const {version_const}: u32 = {version};\n",
            name_doc = name_doc(model, &cfg, ext).render(),
            version_doc = Doc::new()
                .text(format!(
                    "The revision of the {} extension that these bindings were generated from, the C macro `{}`.",
                    model.extension_reference(&ext.name),
                    ext.spec_version_macro
                ))
                .alias(&ext.spec_version_macro)
                .render(),
            name_const = extension_name_const(&ext.name_macro),
            name = ext.name,
            version_const = const_name(&ext.spec_version_macro),
            version = ext.spec_version,
        );
    }
    out.push_str(&lookup_table(model));
    out
}

enum Successor<'a> {
    Version(u32, u32),
    Extension(&'a str),
    Unnamed,
}

fn successor<'a>(model: &Model<'_>, ext: &Extension, field: &str, name: &'a str) -> Successor<'a> {
    let used_in = || format!("the `{field}` of `{}`", ext.name);
    if name.is_empty() {
        assert!(
            field != "promotedto",
            "{} is empty, but the registry allows an empty value only for `deprecatedby` and \
             `obsoletedby`",
            used_in()
        );
        return Successor::Unnamed;
    }
    if let Some((major, minor)) = named_version(name, used_in) {
        return Successor::Version(major, minor);
    }
    assert!(
        is_registry_extension(model, name),
        "`{name}` in {} is not an extension of the registry",
        used_in()
    );
    Successor::Extension(name)
}

fn is_registry_extension(model: &Model<'_>, name: &str) -> bool {
    model.ir.extensions.iter().any(|ext| ext.name == name)
}

fn dependencies(model: &Model<'_>, ext: &Extension) -> Option<Depends> {
    let text = ext.depends.as_deref()?;
    let depends = Depends::parse(text);
    for name in depends.extensions() {
        assert!(
            is_registry_extension(model, name),
            "{}: `{name}` in dependency `{text}` is not an extension of the registry",
            ext.name
        );
    }
    Some(depends)
}

fn requirement(model: &Model<'_>, from: &Cfg, depends: &Depends, nested: bool) -> String {
    let list = |items: &[Depends], join: fn(Vec<String>) -> String| {
        let joined = join(
            items
                .iter()
                .map(|item| requirement(model, from, item, true))
                .collect(),
        );
        if nested {
            format!("({joined})")
        } else {
            joined
        }
    };
    match depends {
        Depends::Version(major, minor) => format!("Vulkan {major}.{minor}"),
        Depends::Extension(name) => model.link(from, name),
        Depends::All(all) => list(all, join_and),
        Depends::Any(any) => list(any, join_or),
    }
}

fn name_doc(model: &Model<'_>, cfg: &Cfg, ext: &Extension) -> Doc {
    let level = match level(ext) {
        "Instance" => "instance",
        _ => "device",
    };
    let provisional = if ext.provisional { "provisional " } else { "" };
    let mut facts = Vec::new();
    if let Some(depends) = dependencies(model, ext) {
        facts.push(format!(
            "Requires {}.",
            requirement(model, cfg, &depends, false)
        ));
    }
    for (verb, field, by) in [
        ("Promoted to", "promotedto", &ext.promoted_to),
        ("Deprecated by", "deprecatedby", &ext.deprecated_by),
        ("Obsoleted by", "obsoletedby", &ext.obsoleted_by),
    ] {
        let Some(name) = by.as_deref() else {
            continue;
        };
        facts.push(match successor(model, ext, field, name) {
            Successor::Version(major, minor) => format!("{verb} Vulkan {major}.{minor}."),
            Successor::Extension(name) => format!("{verb} {}.", model.link(cfg, name)),
            Successor::Unnamed => format!("{}.", verb.trim_end_matches(" by")),
        });
    }
    Doc::new()
        .text(format!(
            "The name of the {provisional}{level} extension {}, the C macro `{}`.",
            model.extension_reference(&ext.name),
            ext.name_macro
        ))
        .text(facts.join(" "))
        .alias(&ext.name)
        .alias(&ext.name_macro)
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
    w!(
        out,
        "/// Every extension of the registry, sorted by name as bytes. [`ExtensionInfo::get`] searches it."
    );
    w!(out, "pub const EXTENSIONS: &[ExtensionInfo] = &[");
    for ext in extensions {
        let depends = dependencies(model, ext).map(|depends| depends_expr(&depends));
        let promoted_to = ext
            .promoted_to
            .as_deref()
            .map(|name| successor(model, ext, "promotedto", name));
        let (promoted_to_version, promoted_to_extension) = match promoted_to {
            Some(Successor::Version(major, minor)) => (
                Some(format!("make_api_version(0, {major}, {minor}, 0)")),
                None,
            ),
            Some(Successor::Extension(name)) => (None, Some(format!("c\"{name}\""))),
            Some(Successor::Unnamed) | None => (None, None),
        };
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

    fn extension(
        name: &str,
        device: bool,
        depends: Option<&str>,
        promoted: Option<&str>,
    ) -> Extension {
        Extension {
            name: name.to_string(),
            name_macro: format!("{}_EXTENSION_NAME", name.to_uppercase()),
            spec_version_macro: format!("{}_SPEC_VERSION", name.to_uppercase()),
            spec_version: 1,
            instance: !device,
            device,
            depends: depends.map(str::to_string),
            promoted_to: promoted.map(str::to_string),
            ..Default::default()
        }
    }

    fn emitted(extensions: Vec<Extension>) -> String {
        let mut ir = registry();
        ir.extensions = extensions;
        squash(&emit(&Model::new(&ir)))
    }

    #[test]
    #[should_panic(
        expected = "`VK_VERSION_1_2_3` in the `promotedto` of `VK_KHR_a` is not a version name"
    )]
    fn a_promotion_to_a_version_name_that_is_not_a_version_aborts_generation() {
        emitted(vec![extension(
            "VK_KHR_a",
            true,
            None,
            Some("VK_VERSION_1_2_3"),
        )]);
    }

    #[test]
    #[should_panic(
        expected = "`VK_VERSION_01_2` in the `deprecatedby` of `VK_KHR_a` is not a version name"
    )]
    fn a_deprecation_by_a_version_name_that_is_not_a_version_aborts_generation() {
        emitted(vec![Extension {
            deprecated_by: Some("VK_VERSION_01_2".to_string()),
            ..extension("VK_KHR_a", true, None, None)
        }]);
    }

    #[test]
    #[should_panic(
        expected = "`VK_VERSION_1_1024` in the `obsoletedby` of `VK_KHR_a` is not a version name"
    )]
    fn an_obsoletion_by_a_version_name_that_is_not_a_version_aborts_generation() {
        emitted(vec![Extension {
            obsoleted_by: Some("VK_VERSION_1_1024".to_string()),
            ..extension("VK_KHR_a", true, None, None)
        }]);
    }

    #[test]
    #[should_panic(
        expected = "VK_KHR_a: `VK_KHR_missing` in dependency `VK_VERSION_1_1+VK_KHR_missing` is not an extension of the registry"
    )]
    fn a_dependency_on_an_extension_that_the_registry_does_not_have_aborts_generation() {
        emitted(vec![extension(
            "VK_KHR_a",
            true,
            Some("VK_VERSION_1_1+VK_KHR_missing"),
            None,
        )]);
    }

    #[test]
    #[should_panic(
        expected = "the `promotedto` of `VK_KHR_a` is empty, but the registry allows an empty value only for `deprecatedby` and `obsoletedby`"
    )]
    fn an_empty_promotion_aborts_generation() {
        emitted(vec![extension("VK_KHR_a", true, None, Some(""))]);
    }

    #[test]
    fn an_empty_deprecation_or_obsoletion_names_no_successor() {
        let mut ir = registry();
        ir.extensions = vec![
            Extension {
                deprecated_by: Some(String::new()),
                ..extension("VK_KHR_deprecated", true, None, None)
            },
            Extension {
                obsoleted_by: Some(String::new()),
                ..extension("VK_KHR_obsoleted", true, None, None)
            },
        ];
        let out = emit(&Model::new(&ir));
        let doc = |item: &str| {
            let end = out.find(item).expect(item);
            out[..end]
                .lines()
                .rev()
                .take(6)
                .collect::<Vec<_>>()
                .join(" ")
        };
        assert!(
            doc("pub const KHR_DEPRECATED_NAME").contains("Deprecated."),
            "{out}"
        );
        assert!(
            doc("pub const KHR_OBSOLETED_NAME").contains("Obsoleted."),
            "{out}"
        );
        let table = squash(&out);
        assert!(table.contains(
            "name: c\"VK_KHR_deprecated\", spec_version: 1, level: ExtensionLevel::Device, depends: None, promoted_to_version: None, promoted_to_extension: None,"
        ));
    }

    #[test]
    #[should_panic(
        expected = "`VK_KHR_missing` in the `promotedto` of `VK_KHR_a` is not an extension of the registry"
    )]
    fn a_promotion_to_an_extension_that_the_registry_does_not_have_aborts_generation() {
        emitted(vec![extension(
            "VK_KHR_a",
            true,
            None,
            Some("VK_KHR_missing"),
        )]);
    }

    #[test]
    fn extension_table_is_sorted_and_carries_dependencies() {
        let mut ir = registry();
        ir.extensions = vec![
            extension("VK_KHR_surface", false, None, None),
            extension("VK_KHR_dynamic_rendering", true, None, None),
            extension("VK_EXT_debug_utils", false, None, None),
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
