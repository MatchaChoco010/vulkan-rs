use crate::model::{Cfg, Model};
use crate::naming::{
    command_name, const_name, enum_variant_name, extension_name_const, flag_bit_name,
    func_pointer_name, type_name,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Target {
    pub(crate) path: String,
    pub(crate) cfg: Cfg,
}

#[derive(Default)]
pub(crate) struct Links {
    targets: BTreeMap<String, Target>,
}

const HAND_WRITTEN: &[(&str, &str)] = &[
    ("VkBool32", "Bool32"),
    ("VkFlags", "Flags"),
    ("VkFlags64", "Flags64"),
    ("VkDeviceSize", "DeviceSize"),
    ("VkDeviceAddress", "DeviceAddress"),
    ("VkSampleMask", "SampleMask"),
    ("VkRemoteAddressNV", "RemoteAddressNV"),
    ("VK_MAKE_API_VERSION", "make_api_version"),
    ("VK_API_VERSION_VARIANT", "api_version_variant"),
    ("VK_API_VERSION_MAJOR", "api_version_major"),
    ("VK_API_VERSION_MINOR", "api_version_minor"),
    ("VK_API_VERSION_PATCH", "api_version_patch"),
    ("VK_HEADER_VERSION", "HEADER_VERSION"),
    ("VK_HEADER_VERSION_COMPLETE", "HEADER_VERSION_COMPLETE"),
    ("VK_API_VERSION_1_0", "API_VERSION_1_0"),
];

impl Links {
    pub(crate) fn build(model: &Model<'_>) -> Self {
        let ir = model.ir;
        let mut links = Links::default();
        for (c_name, path) in HAND_WRITTEN {
            links.add(c_name, path.to_string(), Cfg::default());
        }
        for version in &ir.versions {
            links.add(
                &version.name_api,
                const_name(&version.name_api),
                Cfg::default(),
            );
        }
        for constant in &ir.constants {
            links.add(&constant.name, const_name(&constant.name), Cfg::default());
        }
        for s in &ir.structs {
            links.add_type(&s.name, &s.aliases, model.cfg(s.protect.as_deref()));
        }
        for h in &ir.handles {
            links.add_type(&h.name, &h.aliases, model.cfg(h.protect.as_deref()));
        }
        for f in &ir.flags {
            links.add_type(&f.name, &f.aliases, model.cfg(f.protect.as_deref()));
        }
        for fp in &ir.func_pointers {
            links.add(
                &fp.name,
                func_pointer_name(&fp.name),
                model.cfg(fp.protect.as_deref()),
            );
        }
        for e in &ir.enums {
            let cfg = model.cfg(e.protect.as_deref());
            links.add_type(&e.name, &e.aliases, cfg.clone());
            for field in &e.fields {
                for c_name in std::iter::once(&field.name).chain(&field.aliases) {
                    let variant = enum_variant_name(&e.name, c_name, &ir.vendor_tags);
                    links.add(
                        c_name,
                        format!("{}::{variant}", type_name(&e.name)),
                        cfg.clone().and(model.cfg(field.protect.as_deref())),
                    );
                }
            }
        }
        for b in &ir.bitmasks {
            let cfg = model.cfg(b.protect.as_deref());
            links.add_type(&b.name, &b.aliases, cfg.clone());
            for field in &b.fields {
                for c_name in std::iter::once(&field.name).chain(&field.aliases) {
                    let bit = flag_bit_name(&b.name, c_name, &ir.vendor_tags);
                    links.add(
                        c_name,
                        format!("{}::{bit}", type_name(&b.name)),
                        cfg.clone().and(model.cfg(field.protect.as_deref())),
                    );
                }
            }
        }
        for c in &ir.commands {
            let owner = model.dispatch(c).wrapper();
            links.add(
                &c.name,
                format!("{owner}::{}", command_name(&c.name)),
                model.cfg(c.protect.as_deref()),
            );
        }
        for ext in &ir.extensions {
            links.add(
                &ext.name,
                extension_name_const(&ext.name_macro),
                model.cfg(ext.protect.as_deref()),
            );
        }
        links
    }

    fn add(&mut self, c_name: &str, path: String, cfg: Cfg) {
        self.targets
            .entry(c_name.to_string())
            .or_insert(Target { path, cfg });
    }

    fn add_type(&mut self, name: &str, aliases: &[String], cfg: Cfg) {
        for c_name in std::iter::once(name).chain(aliases.iter().map(String::as_str)) {
            self.add(c_name, type_name(c_name), cfg.clone());
        }
    }

    pub(crate) fn get(&self, c_name: &str) -> Option<&Target> {
        self.targets.get(c_name)
    }
}
