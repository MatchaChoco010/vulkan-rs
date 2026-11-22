use super::variants::{associated_consts, bitmask_variants, name_lookup};
use super::{header, type_aliases, w};
use crate::model::{Cfg, Model};
use crate::naming::type_name;
use std::collections::BTreeMap;
use vulkan_registry::{Bitmask, Flags};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    for b in &model.ir.bitmasks {
        out.push_str(&single_bit_type(model, b));
    }
    let bits_of_flags = model
        .ir
        .bitmasks
        .iter()
        .map(|b| (b.name.as_str(), b))
        .collect::<BTreeMap<_, _>>();
    for f in &model.ir.flags {
        let bits = f
            .bitmask
            .as_deref()
            .and_then(|name| bits_of_flags.get(name).copied());
        out.push_str(&flags_type(model, f, bits));
    }
    out
}

fn repr(bit_width: u32) -> &'static str {
    match bit_width {
        32 => "u32",
        64 => "u64",
        other => panic!("unsupported flags width {other}"),
    }
}

fn single_bit_type(model: &Model<'_>, b: &Bitmask) -> String {
    let repr = repr(b.bit_width);
    let cfg = model.cfg(b.protect.as_deref());
    let name = type_name(&b.name);
    let flags = type_name(&b.flags_name);
    let variants = bitmask_variants(model, b);
    let doc = model
        .type_doc(&cfg, &b.name, "C enumeration of flag bits", "")
        .text(format!(
            "A value of this type is one of the constants. Combining values with `|` gives a [`{flags}`], which the API takes wherever several bits can be set."
        ))
        .render();
    let mut out = String::new();
    w!(
        out,
        r#"{doc}{cfg}#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct {name}(pub {repr});

{cfg}impl {name} {{
    /// Wraps a value of the C enumeration. Values without a constant are allowed.
    pub const fn from_raw(value: {repr}) -> Self {{ Self(value) }}
    /// The value as the C API passes it.
    pub const fn as_raw(self) -> {repr} {{ self.0 }}
    {consts}{name_lookup}/// Whether a constant of this type has this value.
    pub fn is_known(self) -> bool {{ self.name().is_some() }}
}}

{debug}
{cfg}impl From<{name}> for {flags} {{
    fn from(value: {name}) -> Self {{ Self(value.0) }}
}}

{cfg}impl core::ops::BitOr for {name} {{
    type Output = {flags};
    fn bitor(self, rhs: Self) -> {flags} {{ {flags}(self.0 | rhs.0) }}
}}

{cfg}impl core::ops::BitOr<{flags}> for {name} {{
    type Output = {flags};
    fn bitor(self, rhs: {flags}) -> {flags} {{ {flags}(self.0 | rhs.0) }}
}}

{cfg}impl core::ops::BitOr<{name}> for {flags} {{
    type Output = Self;
    fn bitor(self, rhs: {name}) -> Self {{ Self(self.0 | rhs.0) }}
}}

{cfg}impl core::ops::BitOrAssign<{name}> for {flags} {{
    fn bitor_assign(&mut self, rhs: {name}) {{ self.0 |= rhs.0; }}
}}

{cfg}impl core::ops::BitAnd<{name}> for {flags} {{
    type Output = Self;
    fn bitand(self, rhs: {name}) -> Self {{ Self(self.0 & rhs.0) }}
}}
"#,
        consts = associated_consts(model, &b.name, &cfg, &variants),
        name_lookup = name_lookup(&variants),
        debug = debug_impl(&cfg, &name, repr),
    );
    out.push_str(&type_aliases(&cfg, &b.aliases, &name, ""));
    out
}

fn flags_type(model: &Model<'_>, f: &Flags, bits: Option<&Bitmask>) -> String {
    let repr = repr(f.bit_width);
    let cfg = model.cfg(f.protect.as_deref());
    let name = type_name(&f.name);
    let variants = bits
        .map(|bits| bitmask_variants(model, bits))
        .unwrap_or_default();
    let bits_doc = match bits {
        Some(bits) => format!(
            "The bits are the constants of {}, which this type has as well. Values combine with `|`, `&`, `^` and `!`.",
            model.link(&cfg, &bits.name)
        ),
        None => "The registry defines no bits for this type.".to_string(),
    };
    let doc = model
        .type_doc(&cfg, &f.name, "C bitmask type", "")
        .text(bits_doc)
        .render();
    let bits_owner = bits.map_or(f.name.as_str(), |bits| bits.name.as_str());
    let mut out = String::new();
    w!(
        out,
        r#"{doc}{cfg}#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct {name}(pub {repr});

{cfg}impl {name} {{
    /// The value with no bit set.
    pub const fn empty() -> Self {{ Self(0) }}
    /// Wraps a value of the C bitmask. Bits without a constant are allowed.
    pub const fn from_raw(value: {repr}) -> Self {{ Self(value) }}
    /// The value as the C API passes it.
    pub const fn as_raw(self) -> {repr} {{ self.0 }}
    /// Whether no bit is set.
    pub const fn is_empty(self) -> bool {{ self.0 == 0 }}
    /// Whether every bit of `other` is set in `self`.
    pub const fn contains(self, other: Self) -> bool {{ (self.0 & other.0) == other.0 }}
    /// Whether at least one bit of `other` is set in `self`.
    pub const fn intersects(self, other: Self) -> bool {{ (self.0 & other.0) != 0 }}
    /// The bits that are set in `self` or in `other`.
    pub const fn union(self, other: Self) -> Self {{ Self(self.0 | other.0) }}
    {consts}{name_lookup}/// Every bit that a constant of this type sets.
    pub const fn all() -> Self {{
        let variants = Self::variants();
        let mut bits = 0;
        let mut index = 0;
        while index < variants.len() {{
            bits |= variants[index].0.0;
            index += 1;
        }}
        Self(bits)
    }}
    /// Whether every bit that is set is a bit of [`Self::all`].
    pub const fn is_known(self) -> bool {{ (self.0 & !Self::all().0) == 0 }}
}}

{debug}"#,
        consts = associated_consts(model, bits_owner, &cfg, &variants),
        name_lookup = name_lookup(&variants),
        debug = debug_impl(&cfg, &name, repr),
    );
    for (op, method, symbol) in [
        ("BitOr", "bitor", "|"),
        ("BitAnd", "bitand", "&"),
        ("BitXor", "bitxor", "^"),
    ] {
        w!(
            out,
            r#"{cfg}impl core::ops::{op} for {name} {{
    type Output = Self;
    fn {method}(self, rhs: Self) -> Self {{ Self(self.0 {symbol} rhs.0) }}
}}

{cfg}impl core::ops::{op}Assign for {name} {{
    fn {method}_assign(&mut self, rhs: Self) {{ self.0 {symbol}= rhs.0; }}
}}
"#
        );
    }
    w!(
        out,
        r#"{cfg}impl core::ops::Not for {name} {{
    type Output = Self;
    fn not(self) -> Self {{ Self(!self.0) }}
}}
"#
    );
    out.push_str(&type_aliases(&cfg, &f.aliases, &name, ""));
    out
}

fn debug_impl(cfg: &Cfg, name: &str, repr: &str) -> String {
    let widened = |bits: &str| match repr {
        "u64" => bits.to_string(),
        _ => format!("u64::from({bits})"),
    };
    format!(
        r#"{cfg}impl core::fmt::Debug for {name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        debug_flags(f, {value}, Self::variants().iter().map(|&(flag, name)| ({flag}, name)))
    }}
}}
"#,
        value = widened("self.0"),
        flag = widened("flag.0"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::EnumField;

    #[test]
    fn flags_get_bits_operators_and_named_debug() {
        let mut ir = registry();
        ir.bitmasks.push(Bitmask {
            name: "VkImageUsageFlagBits".to_string(),
            flags_name: "VkImageUsageFlags".to_string(),
            bit_width: 32,
            fields: vec![
                EnumField {
                    name: "VK_IMAGE_USAGE_SAMPLED_BIT".to_string(),
                    value: "4".to_string(),
                    ..Default::default()
                },
                EnumField {
                    name: "VK_IMAGE_USAGE_ALL".to_string(),
                    value: "7".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        ir.flags.push(Flags {
            name: "VkImageUsageFlags".to_string(),
            bitmask: Some("VkImageUsageFlagBits".to_string()),
            bit_width: 32,
            ..Default::default()
        });
        let out = emit(&Model::new(&ir));
        assert!(out.contains("pub struct ImageUsageFlags(pub u32);"));
        assert!(out.contains("pub const SAMPLED: Self = Self(4);"));
        assert!(out.contains("pub const ALL: Self = Self(7);"));
        assert!(out.contains("(Self(4), \"SAMPLED\"),"));
        assert!(out.contains("(Self(7), \"ALL\"),"));
        assert!(out.contains("pub const fn all() -> Self {"));
        assert!(out.contains("pub fn from_name(name: &str) -> Option<Self>"));
        assert!(out.contains("impl core::ops::Not for ImageUsageFlags"));
        assert!(out.contains("impl core::ops::BitAndAssign for ImageUsageFlags"));
        assert!(out.contains("impl From<ImageUsageFlagBits> for ImageUsageFlags"));
    }
}
