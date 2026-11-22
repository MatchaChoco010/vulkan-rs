use super::w;
use crate::model::{Cfg, Model};
use crate::naming::{enum_variant_name, flag_bit_name};
use std::collections::{BTreeMap, BTreeSet};
use vulkan_registry::{Bitmask, Enum, EnumField};

pub(crate) struct Variant<'a> {
    pub(crate) name: String,
    pub(crate) c_name: &'a str,
    pub(crate) value: &'a str,
    pub(crate) protect: Option<&'a str>,
    pub(crate) cfg: Cfg,
    is_alias: bool,
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

    let mut variants = Vec::new();
    let mut value_of_name = BTreeMap::<String, &str>::new();
    for (c_name, field, is_alias) in names.chain(aliases) {
        let name = rust_name(c_name);
        match value_of_name.get(&name) {
            Some(existing) if *existing == field.value => continue,
            Some(existing) => panic!(
                "{owner}: `{c_name}` maps to `{name}` which already has value {existing}, not {}",
                field.value
            ),
            None => {}
        }
        value_of_name.insert(name.clone(), &field.value);
        variants.push(Variant {
            name,
            c_name,
            value: &field.value,
            protect: field.protect.as_deref(),
            cfg: model.cfg(field.protect.as_deref()),
            is_alias,
        });
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

pub(crate) fn associated_consts(variants: &[Variant<'_>]) -> String {
    let mut out = String::new();
    for v in variants {
        w!(
            out,
            "{}pub const {}: Self = Self({});",
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
        .partition(|v| !v.is_alias && seen_values.insert(v.numeric_value()));
    by_value.sort_by_key(|v| v.numeric_value());

    let rows = |variants: &[&Variant<'_>]| {
        variants
            .iter()
            .map(|v| v.table_row())
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "pub const fn variants() -> &'static [(Self, &'static str)] {{
            &[
                {by_value}
            ]
        }}
        const fn aliases() -> &'static [(Self, &'static str)] {{
            &[
                {other_names}
            ]
        }}
        pub fn name(self) -> Option<&'static str> {{ name_of(Self::variants(), self) }}
        pub fn from_name(name: &str) -> Option<Self> {{ value_of(Self::variants(), Self::aliases(), name) }}
",
        by_value = rows(&by_value),
        other_names = rows(&other_names),
    )
}
