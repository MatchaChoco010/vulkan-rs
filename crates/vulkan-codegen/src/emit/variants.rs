use super::w;
use crate::docs::{Doc, join_and};
use crate::model::{Cfg, Model};
use crate::naming::{enum_variant_name, flag_bit_name};
use std::collections::{BTreeMap, BTreeSet};
use vulkan_registry::{Bitmask, Enum, EnumField};

pub(crate) struct Variant<'a> {
    pub(crate) name: String,
    pub(crate) c_name: &'a str,
    value_name: &'a str,
    pub(crate) value: &'a str,
    pub(crate) protect: Option<&'a str>,
    pub(crate) cfg: Cfg,
    alias_of: Option<(&'a str, String)>,
    legacy_names: Vec<&'a str>,
}

impl Variant<'_> {
    fn numeric_value(&self) -> i128 {
        self.value
            .parse()
            .unwrap_or_else(|_| panic!("{} = {} is not an integer", self.c_name, self.value))
    }

    fn table_row(&self) -> String {
        format!("{}(Self({}), \"{}\"),", self.cfg, self.value, self.name)
    }
}

fn variants<'a>(
    model: &Model<'_>,
    owner: &str,
    fields: &'a [EnumField],
    rust_name: impl Fn(&str) -> String,
) -> Vec<Variant<'a>> {
    let names = fields
        .iter()
        .map(|field| (field.name.as_str(), field, false));
    let aliases = fields.iter().flat_map(|field| {
        field
            .aliases
            .iter()
            .map(move |alias| (alias.as_str(), field, true))
    });
    let rust_name = &rust_name;

    let mut variants = Vec::<Variant<'a>>::new();
    let mut variant_of_name = BTreeMap::<String, usize>::new();
    for (c_name, field, is_alias) in names.chain(aliases) {
        let name = rust_name(c_name);
        let Some(&index) = variant_of_name.get(&name) else {
            variant_of_name.insert(name.clone(), variants.len());
            variants.push(Variant {
                name,
                c_name,
                value_name: &field.name,
                value: &field.value,
                protect: field.protect.as_deref(),
                cfg: model.cfg(field.protect.as_deref()),
                alias_of: is_alias.then(|| (field.name.as_str(), rust_name(&field.name))),
                legacy_names: Vec::new(),
            });
            continue;
        };
        let shared = &mut variants[index];
        assert!(
            shared.value == field.value,
            "{owner}: `{c_name}` maps to `{name}` which already has value {}, not {}",
            shared.value,
            field.value
        );
        assert!(
            shared.value_name == field.name,
            "{owner}: `{c_name}`, a name of `{}`, maps to `{name}`, which stands for `{}`",
            field.name,
            shared.value_name
        );
        if model.is_legacy_alias(owner, c_name) {
            shared.legacy_names.push(c_name);
            continue;
        }
        assert!(
            model.is_legacy_alias(owner, shared.c_name),
            "{owner}: `{}` and `{c_name}` both map to `{name}`, and neither is a legacy alias",
            shared.c_name
        );
        shared.legacy_names.push(shared.c_name);
        shared.c_name = c_name;
    }
    for variant in &mut variants {
        variant.legacy_names.sort_unstable();
    }
    variants
}

pub(crate) fn enum_variants<'a>(model: &Model<'_>, e: &'a Enum) -> Vec<Variant<'a>> {
    variants(model, &e.name, &e.fields, |c_name| {
        enum_variant_name(&e.name, c_name, &model.ir.vendor_tags)
    })
}

pub(crate) fn bitmask_variants<'a>(model: &Model<'_>, b: &'a Bitmask) -> Vec<Variant<'a>> {
    variants(model, &b.name, &b.fields, |c_name| {
        flag_bit_name(&b.name, c_name, &model.ir.vendor_tags)
    })
}

fn value_doc(model: &Model<'_>, owner: &str, owner_cfg: &Cfg, v: &Variant<'_>) -> Doc {
    let from = owner_cfg.clone().and(v.cfg.clone());
    let c_name = v.c_name;
    let doc = if let Some((canonical, constant)) = &v.alias_of {
        let alias = match model.is_legacy_alias(owner, c_name) {
            true => "a legacy alias",
            false => "an alias",
        };
        Doc::new().text(format!(
            "`{c_name}` is {alias} of [`{canonical}`](Self::{constant})."
        ))
    } else if let Some(described) = model.described(&from, owner, c_name) {
        Doc::new().blocks(described)
    } else if let Some(s) = model
        .ir
        .structs
        .iter()
        .find(|s| owner == "VkStructureType" && s.s_type.as_deref() == Some(c_name))
    {
        Doc::new().text(format!(
            "`{c_name}`, the structure type of {}.",
            model.link(&from, &s.name)
        ))
    } else if let Some(h) = model
        .ir
        .handles
        .iter()
        .find(|h| owner == "VkObjectType" && h.object_type == c_name)
    {
        Doc::new().text(format!(
            "`{c_name}`, the object type of {}.",
            model.link(&from, &h.name)
        ))
    } else if let Some(tabulated) = model.tabulated_on_the_page(&from, owner, c_name) {
        Doc::new().blocks(tabulated)
    } else if let Some(described) = model.described_outside_the_page(&from, owner, c_name) {
        Doc::new().blocks(described)
    } else if let Some(tabulated) = model.tabulated_outside_the_pages(&from, owner, c_name) {
        Doc::new().blocks(tabulated)
    } else if let Some(described) = model.described_under_an_alias(&from, owner, c_name) {
        Doc::new().blocks(described)
    } else {
        Doc::new().text(format!(
            "`{c_name}`, a value of {}.",
            model.reference(owner)
        ))
    };
    let value = match &v.alias_of {
        Some((canonical, constant)) => format!("[`{canonical}`](Self::{constant})"),
        None => format!("`{c_name}`"),
    };
    let legacy_names = join_and(
        v.legacy_names
            .iter()
            .map(|name| format!("`{name}`"))
            .collect(),
    );
    let also = match v.legacy_names.len() {
        0 => String::new(),
        1 => format!(
            "The C header also calls this value {legacy_names}, which is a legacy alias of {value}."
        ),
        _ => format!(
            "The C header also calls this value {legacy_names}, which are legacy aliases of {value}."
        ),
    };
    let doc = doc
        .text(model.legacy_value(owner, c_name))
        .text(also)
        .alias(c_name);
    let doc = v.legacy_names.iter().fold(doc, |doc, name| doc.alias(name));
    doc.of_item(&v.name)
}

pub(crate) fn associated_consts(
    model: &Model<'_>,
    owner: &str,
    owner_cfg: &Cfg,
    variants: &[Variant<'_>],
) -> String {
    let mut out = String::new();
    for v in variants {
        w!(
            out,
            "{}{}pub const {}: Self = Self({});",
            value_doc(model, owner, owner_cfg, v).render(),
            v.cfg,
            v.name,
            v.value
        );
    }
    out
}

pub(crate) fn name_lookup(variants: &[Variant<'_>]) -> String {
    let mut seen_values = BTreeSet::new();
    let (mut by_value, other_names): (Vec<&Variant<'_>>, Vec<&Variant<'_>>) = variants
        .iter()
        .partition(|v| v.alias_of.is_none() && seen_values.insert(v.numeric_value()));
    by_value.sort_by_key(|v| v.numeric_value());

    let rows = |variants: &[&Variant<'_>]| {
        variants
            .iter()
            .map(|v| v.table_row())
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "/// The values of this type that have a constant, each with the name of its constant,
        /// sorted by value. A value that several constants share is listed once.
        pub const fn variants() -> &'static [(Self, &'static str)] {{
            &[
                {by_value}
            ]
        }}
        const fn aliases() -> &'static [(Self, &'static str)] {{
            &[
                {other_names}
            ]
        }}
        /// The name of the constant that has this value, as listed by [`Self::variants`], or
        /// `None` if no constant has it.
        pub fn name(self) -> Option<&'static str> {{ name_of(Self::variants(), self) }}
        /// The value of the constant called `name`, or `None` if this type has no such
        /// constant. Every constant of this type is found, including the ones that
        /// [`Self::variants`] leaves out.
        pub fn from_name(name: &str) -> Option<Self> {{ value_of(Self::variants(), Self::aliases(), name) }}
",
        by_value = rows(&by_value),
        other_names = rows(&other_names),
    )
}
