mod cfg;
mod ctype;
mod depends;
mod derives;
mod dispatch;
mod platform;
mod rust_types;

pub(crate) use cfg::{BETA_PROTECT, Cfg};
pub(crate) use ctype::{CType, null_ptr};
pub(crate) use depends::{Depends, parse_version};
pub(crate) use dispatch::Dispatch;
pub(crate) use platform::{is_opaque, is_platform_type};
pub(crate) use rust_types::Scalar;

use crate::docs::{Links, Table, read_table};
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
    pub(crate) command_aliases: BTreeMap<&'a str, &'a str>,
    pub(crate) links: Links,
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
        Self::with_docs(ir, NO_DOCS.get_or_init(Docs::default))
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
            command_aliases: command_aliases.collect(),
            links: Links::default(),
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
