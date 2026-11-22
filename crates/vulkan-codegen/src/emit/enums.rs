use super::variants::{associated_consts, enum_variants, name_lookup};
use super::{header, type_aliases, w};
use crate::model::Model;
use crate::naming::type_name;

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    for e in &model.ir.enums {
        assert_eq!(e.bit_width, 32, "{} is not a 32-bit enum", e.name);
        let cfg = model.cfg(e.protect.as_deref());
        let name = type_name(&e.name);
        let variants = enum_variants(model, e);
        w!(
            out,
            r#"{cfg}#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct {name}(pub i32);

{cfg}impl {name} {{
    pub const fn from_raw(value: i32) -> Self {{ Self(value) }}
    pub const fn as_raw(self) -> i32 {{ self.0 }}
    {consts}{name_lookup}pub fn is_known(self) -> bool {{ self.name().is_some() }}
}}

{cfg}impl core::fmt::Debug for {name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        debug_enum(f, "{name}", self.name(), self.0)
    }}
}}
"#,
            consts = associated_consts(&variants),
            name_lookup = name_lookup(&variants),
        );
        out.push_str(&type_aliases(&cfg, &e.aliases, &name, ""));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::super::squash;
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::{Enum, EnumField};

    #[test]
    fn enum_aliases_become_constants_but_not_debug_names() {
        let mut ir = registry();
        ir.enums.push(Enum {
            name: "VkPresentModeKHR".to_string(),
            bit_width: 32,
            fields: vec![EnumField {
                name: "VK_PRESENT_MODE_FIFO_LATEST_READY_KHR".to_string(),
                aliases: vec!["VK_PRESENT_MODE_FIFO_LATEST_READY_EXT".to_string()],
                value: "1000361000".to_string(),
                protect: None,
            }],
            ..Default::default()
        });
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub const FIFO_LATEST_READY: Self = Self(1000361000);"));
        assert!(out.contains("pub const FIFO_LATEST_READY_EXT: Self = Self(1000361000);"));
        assert!(out.contains(
            "pub const fn variants() -> &'static [(Self, &'static str)] { &[ (Self(1000361000), \"FIFO_LATEST_READY\"), ] }"
        ));
        assert!(out.contains(
            "const fn aliases() -> &'static [(Self, &'static str)] { &[ (Self(1000361000), \"FIFO_LATEST_READY_EXT\"), ] }"
        ));
    }

    #[test]
    #[should_panic(expected = "already has value")]
    fn conflicting_constant_names_abort_generation() {
        let mut ir = registry();
        ir.enums.push(Enum {
            name: "VkThingKHR".to_string(),
            bit_width: 32,
            fields: vec![
                EnumField {
                    name: "VK_THING_A".to_string(),
                    value: "0".to_string(),
                    ..Default::default()
                },
                EnumField {
                    name: "VK_THING_A_KHR".to_string(),
                    value: "1".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        emit(&Model::new(&ir));
    }
}
