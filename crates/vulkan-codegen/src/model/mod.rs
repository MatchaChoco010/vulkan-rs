mod alignment;
mod cfg;
mod ctype;
mod depends;
mod derives;
mod dispatch;
mod platform;
mod rust_types;
mod void_arrays;

pub(crate) use alignment::{
    AlignedOutput, Alignment, aligned_input, assert_listed_aligned_memory_exists,
};
pub(crate) use cfg::{BETA_PROTECT, Cfg};
pub(crate) use ctype::{CType, null_ptr};
pub(crate) use depends::{Depends, named_version, parse_version};
pub(crate) use dispatch::Dispatch;
pub(crate) use platform::{is_opaque, is_platform_type};
pub(crate) use rust_types::Scalar;
pub(crate) use void_arrays::{TypedValues, VoidArrayLength, void_array_len, void_array_length};

use crate::docs::{
    LegacyAttributes, LegacyMarks, Links, Table, ValueDescriptions, legacy_attributes,
    legacy_marks, read_table, value_descriptions,
};
use crate::naming::enum_variant_name;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;
use vulkan_registry::{DocPage, DocTable, Docs, Enum, Registry, Struct};

const POINTER_TYPEDEFS: &[&str] = &["VkRemoteAddressNV"];

pub(crate) struct Model<'a> {
    pub(crate) ir: &'a Registry,
    pub(crate) docs: &'a Docs,
    pub(crate) pages: BTreeMap<&'a str, &'a DocPage>,
    pub(crate) tables: Vec<(&'a DocTable, Table)>,
    pub(crate) value_descriptions: ValueDescriptions<'a>,
    pub(crate) command_aliases: BTreeMap<&'a str, &'a str>,
    pub(crate) links: Links,
    pub(crate) legacy: LegacyMarks<'a>,
    pub(crate) legacy_attributes: LegacyAttributes<'a>,
    structs: BTreeMap<&'a str, &'a Struct>,
    handles: BTreeSet<&'a str>,
    enums: BTreeSet<&'a str>,
    bitmasks: BTreeSet<&'a str>,
    flags: BTreeSet<&'a str>,
    func_pointers: BTreeSet<&'a str>,
    constants: BTreeSet<&'a str>,
    aliases: BTreeMap<&'a str, &'a str>,
    lifetimes: BTreeSet<&'a str>,
    plain_eq: BTreeSet<&'a str>,
    plain_hash: BTreeSet<&'a str>,
}

macro_rules! names {
    ($items:expr) => {
        $items.iter().map(|item| item.name.as_str()).collect()
    };
}

macro_rules! alias_targets {
    ($($items:expr),+) => {{
        let mut targets = BTreeMap::new();
        $(for item in $items {
            for alias in &item.aliases {
                targets.insert(alias.as_str(), item.name.as_str());
            }
        })+
        targets
    }};
}

impl<'a> Model<'a> {
    pub(crate) fn new(ir: &'a Registry) -> Self {
        static NO_DOCS: OnceLock<Docs> = OnceLock::new();
        let mut model = Self::with_docs(ir, NO_DOCS.get_or_init(Docs::default));
        model.legacy.clear();
        model
    }

    pub(crate) fn with_docs(ir: &'a Registry, docs: &'a Docs) -> Self {
        let pages = docs.pages.iter().flat_map(|page| {
            let names = std::iter::once(&page.name).chain(&page.aliases);
            names.map(move |name| (name.as_str(), page))
        });
        let command_aliases = ir.commands.iter().filter_map(|command| {
            let alias = command.alias.as_deref()?;
            Some((command.name.as_str(), alias))
        });
        let mut model = Self {
            ir,
            docs,
            pages: pages.collect(),
            tables: docs
                .tables
                .iter()
                .filter_map(|table| Some((table, read_table(&table.text)?)))
                .collect(),
            value_descriptions: value_descriptions(docs),
            command_aliases: command_aliases.collect(),
            links: Links::default(),
            legacy: legacy_marks(ir),
            legacy_attributes: legacy_attributes(ir),
            structs: ir.structs.iter().map(|s| (s.name.as_str(), s)).collect(),
            handles: names!(ir.handles),
            enums: names!(ir.enums),
            bitmasks: names!(ir.bitmasks),
            flags: names!(ir.flags),
            func_pointers: names!(ir.func_pointers),
            constants: names!(ir.constants),
            aliases: alias_targets!(&ir.handles, &ir.enums, &ir.bitmasks, &ir.flags, &ir.structs),
            lifetimes: BTreeSet::new(),
            plain_eq: BTreeSet::new(),
            plain_hash: BTreeSet::new(),
        };
        model.lifetimes = model.structs_that_borrow();
        model.plain_eq = model.structs_of_plain_values(derives::Floats::Allowed);
        model.plain_hash = model.structs_of_plain_values(derives::Floats::Forbidden);
        model.links = Links::build(&model);
        model
    }

    pub(crate) fn canonical<'b>(&'b self, name: &'b str) -> &'b str {
        self.aliases.get(name).copied().unwrap_or(name)
    }

    pub(crate) fn get_struct(&self, name: &str) -> Option<&'a Struct> {
        self.structs.get(self.canonical(name)).copied()
    }

    pub(crate) fn get_enum(&self, name: &str) -> &'a Enum {
        self.ir
            .enums
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| panic!("the registry has no enum {name}"))
    }

    pub(crate) fn result_code<'b>(&'b self, code: &'b str) -> &'b str {
        self.get_enum("VkResult")
            .fields
            .iter()
            .find(|f| f.name == code || f.aliases.iter().any(|alias| alias == code))
            .map_or(code, |f| f.name.as_str())
    }

    pub(crate) fn result_value(&self, code: &str) -> i64 {
        let field = self
            .get_enum("VkResult")
            .fields
            .iter()
            .find(|f| f.name == code || f.aliases.iter().any(|alias| alias == code))
            .unwrap_or_else(|| panic!("unknown result code {code}"));
        field.value.parse().unwrap_or_else(|_| {
            panic!(
                "the result code {code} has the value {}, which is not an integer",
                field.value
            )
        })
    }

    pub(crate) fn is_handle(&self, name: &str) -> bool {
        self.handles.contains(self.canonical(name))
    }

    pub(crate) fn is_flags(&self, name: &str) -> bool {
        self.flags.contains(self.canonical(name))
    }

    pub(crate) fn is_func_pointer(&self, name: &str) -> bool {
        self.func_pointers.contains(name)
    }

    pub(crate) fn has_opaque_representation(&self, name: &str) -> bool {
        is_platform_type(name) || POINTER_TYPEDEFS.contains(&name)
    }

    pub(crate) fn has_lifetime(&self, name: &str) -> bool {
        self.lifetimes.contains(self.canonical(name))
    }

    pub(crate) fn derives_partial_eq(&self, name: &str) -> bool {
        self.plain_eq.contains(self.canonical(name))
    }

    pub(crate) fn derives_eq_hash(&self, name: &str) -> bool {
        self.plain_hash.contains(self.canonical(name))
    }

    pub(crate) fn enum_variant(&self, enum_name: &str, value: &str) -> String {
        enum_variant_name(enum_name, value, &self.ir.vendor_tags)
    }

    pub(crate) fn structure_type(&self, s_type: &str) -> String {
        format!(
            "StructureType::{}",
            self.enum_variant("VkStructureType", s_type)
        )
    }
}

pub(crate) fn has_p_next(s: &Struct) -> bool {
    s.members.iter().any(|m| m.name == "pNext")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{catch_quietly, field, registry, structure, tagged};
    use vulkan_registry::EnumField;

    fn results() -> Registry {
        let mut ir = registry();
        let results = ir
            .enums
            .iter_mut()
            .find(|e| e.name == "VkResult")
            .expect("the registry of the tests has VkResult");
        for (name, value, aliases) in [
            ("VK_SUCCESS", "0", &[][..]),
            ("VK_INCOMPLETE", "5", &[]),
            (
                "VK_PIPELINE_COMPILE_REQUIRED",
                "1000297000",
                &[
                    "VK_PIPELINE_COMPILE_REQUIRED_EXT",
                    "VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT",
                ],
            ),
            ("VK_ERROR_NOT_A_NUMBER", "0x10", &[]),
        ] {
            results.fields.push(EnumField {
                name: name.to_string(),
                aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
                value: value.to_string(),
                ..Default::default()
            });
        }
        ir
    }

    #[test]
    fn a_result_code_and_its_aliases_name_the_same_code_and_value() {
        let ir = results();
        let model = Model::new(&ir);
        for (code, name, value) in [
            ("VK_SUCCESS", "VK_SUCCESS", 0),
            ("VK_INCOMPLETE", "VK_INCOMPLETE", 5),
            (
                "VK_PIPELINE_COMPILE_REQUIRED",
                "VK_PIPELINE_COMPILE_REQUIRED",
                1000297000,
            ),
            (
                "VK_PIPELINE_COMPILE_REQUIRED_EXT",
                "VK_PIPELINE_COMPILE_REQUIRED",
                1000297000,
            ),
            (
                "VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT",
                "VK_PIPELINE_COMPILE_REQUIRED",
                1000297000,
            ),
        ] {
            assert_eq!(model.result_code(code), name);
            assert_eq!(model.result_value(code), value, "{code}");
        }
        assert_eq!(model.result_code("VK_ERROR_UNLISTED"), "VK_ERROR_UNLISTED");
    }

    #[test]
    fn the_value_of_an_unknown_or_malformed_result_code_aborts_generation_with_its_name() {
        let ir = results();
        let model = Model::new(&ir);
        for (code, message) in [
            ("VK_ERROR_UNLISTED", "unknown result code VK_ERROR_UNLISTED"),
            (
                "VK_ERROR_NOT_A_NUMBER",
                "the result code VK_ERROR_NOT_A_NUMBER has the value 0x10, which is not an integer",
            ),
        ] {
            assert_eq!(
                catch_quietly(|| model.result_value(code)),
                Err(message.to_string())
            );
        }
        assert_eq!(
            catch_quietly(|| model.get_enum("VkMissing").name.clone()),
            Err("the registry has no enum VkMissing".to_string())
        );
    }

    #[test]
    fn only_a_structure_with_a_member_named_p_next_has_a_chain() {
        assert!(has_p_next(&tagged(
            "VkTagged",
            "VK_STRUCTURE_TYPE_TAGGED",
            vec![]
        )));
        assert!(has_p_next(&structure(
            "VkUntagged",
            vec![field("a", "uint32_t"), field("pNext", "void*")]
        )));
        assert!(!has_p_next(&structure("VkEmpty", vec![])));
        assert!(!has_p_next(&structure(
            "VkPlain",
            vec![field("next", "void*"), field("pNextSibling", "void*")]
        )));
    }
}
