use crate::model::{Cfg, Model};
use std::collections::BTreeMap;
use vulkan_registry::{
    EnumField, LegacyAttribute, LegacyElement, LegacyKind, LegacyReason, Registry,
};

const NO_REASON: &str = "Legacy functionality, for which the registry gives no reason.";
const NOT_USED: &str = "Legacy functionality that is not used.";
const DESCRIBED_AS_NOT_USED: &str = "is legacy and not used";

const ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT: &[(&str, &str)] = &[(
    "StdVideoAV1ColorPrimaries",
    "STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED",
)];

pub(crate) type LegacyAttributes<'a> =
    BTreeMap<(LegacyElement, &'a str, &'a str), &'a LegacyAttribute>;

fn described(attribute: &LegacyAttribute) -> String {
    let name = &attribute.name;
    match (attribute.element, &attribute.owner) {
        (LegacyElement::Enum, Some(owner)) => format!("the value `{name}` of `{owner}`"),
        (LegacyElement::Enum, None) => format!("the constant `{name}`"),
        (LegacyElement::Member, Some(owner)) => format!("the member `{name}` of `{owner}`"),
        (LegacyElement::Member, None) => format!("the member `{name}`"),
    }
}

pub(crate) fn legacy_attributes(ir: &Registry) -> LegacyAttributes<'_> {
    let mut attributes = LegacyAttributes::new();
    for attribute in &ir.legacy_attributes {
        let owner = attribute.owner.as_deref().unwrap_or_default();
        let key = (attribute.element, owner, attribute.name.as_str());
        assert!(
            attributes.insert(key, attribute).is_none(),
            "the registry data gives {} the attribute `deprecated` twice",
            described(attribute)
        );
    }
    attributes
}

impl<'a> Model<'a> {
    fn legacy_reason(
        &self,
        element: LegacyElement,
        owner: &str,
        name: &str,
    ) -> Option<LegacyReason> {
        let attribute = self.legacy_attributes.get(&(element, owner, name))?;
        Some(attribute.reason)
    }

    pub(crate) fn is_legacy_alias(&self, owner: &str, c_name: &str) -> bool {
        self.legacy_reason(LegacyElement::Enum, owner, c_name) == Some(LegacyReason::Aliased)
    }

    pub(crate) fn legacy_value(&self, owner: &str, c_name: &str) -> String {
        match self.legacy_reason(LegacyElement::Enum, owner, c_name) {
            Some(LegacyReason::True) => NO_REASON.to_string(),
            _ => String::new(),
        }
    }

    fn unused_member(&self, structure: &str, member: &str) -> String {
        match self.legacy_reason(LegacyElement::Member, structure, member) {
            Some(LegacyReason::Unused) => NOT_USED.to_string(),
            _ => String::new(),
        }
    }

    pub(crate) fn unused_member_unless_described(
        &self,
        structure: &str,
        member: &str,
        description: &str,
    ) -> String {
        let description = description.split_whitespace().collect::<Vec<_>>().join(" ");
        match description.contains(DESCRIBED_AS_NOT_USED) {
            true => String::new(),
            false => self.unused_member(structure, member),
        }
    }

    pub(crate) fn legacy_setter(&self, from: &Cfg, structure: &str, member: &str) -> String {
        let marked = self.legacy_member(from, structure, member);
        match marked.is_empty() {
            true => self.unused_member(structure, member),
            false => marked,
        }
    }

    fn values(&self, owner: &str) -> Option<&'a [EnumField]> {
        let ir = self.ir;
        let enumeration = ir.enums.iter().find(|e| e.name == owner);
        let bitmask = ir.bitmasks.iter().find(|b| b.name == owner);
        enumeration
            .map(|e| e.fields.as_slice())
            .or(bitmask.map(|b| b.fields.as_slice()))
    }

    fn is_an_extension_macro(&self, c_name: &str) -> bool {
        let mut extensions = self.ir.extensions.iter();
        extensions.any(|ext| ext.name_macro == c_name || ext.spec_version_macro == c_name)
    }
}

fn is_left_out(owner: &str, name: &str) -> bool {
    ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT.contains(&(owner, name))
}

pub(crate) fn assert_every_legacy_attribute_is_stated(model: &Model<'_>) {
    for attribute in &model.ir.legacy_attributes {
        let what = described(attribute);
        let name = attribute.name.as_str();
        let values = |owner: &str| {
            model.values(owner).unwrap_or_else(|| {
                panic!(
                    "the registry gives {what} the attribute `deprecated`, and the registry \
                     data has no enumeration `{owner}`"
                )
            })
        };
        match (
            attribute.element,
            attribute.owner.as_deref(),
            attribute.reason,
            attribute.alias.as_deref(),
        ) {
            (LegacyElement::Enum, Some(owner), LegacyReason::Aliased, Some(alias)) => {
                let mut values = values(owner).iter();
                let value = values.find(|value| value.aliases.iter().any(|a| a == name));
                if is_left_out(owner, name) {
                    assert!(
                        value.is_none(),
                        "{what} is in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT, but the \
                         registry data has it"
                    );
                    continue;
                }
                let value = value.unwrap_or_else(|| {
                    panic!(
                        "the registry calls {what} a legacy alias, and the registry data has \
                         no value with this alias"
                    )
                });
                assert!(
                    value.name == alias || value.aliases.iter().any(|a| a == alias),
                    "the registry calls {what} a legacy alias of `{alias}`, and the registry \
                     data has it as an alias of `{}`, which `{alias}` does not name",
                    value.name
                );
            }
            (LegacyElement::Enum, Some(owner), LegacyReason::True, None) => {
                assert!(
                    values(owner).iter().any(|value| value.name == name),
                    "the registry calls {what} legacy, and the registry data has no such value"
                );
            }
            (LegacyElement::Enum, None, LegacyReason::Aliased, Some(alias)) => {
                assert!(
                    model.is_an_extension_macro(alias) && model.links.get(name).is_none(),
                    "the registry calls {what} a legacy alias of `{alias}`: the bindings leave \
                     out the legacy aliases of the name and of the version macro of an \
                     extension, and nothing else"
                );
            }
            (LegacyElement::Member, Some(structure), LegacyReason::Unused, None) => {
                assert!(
                    model.documents_the_member(structure, name),
                    "the registry calls {what} legacy and not used, and the bindings have no \
                     such member with a setter"
                );
                assert!(
                    !model
                        .legacy
                        .contains_key(&(LegacyKind::Feature, structure, name)),
                    "the registry calls {what} legacy and not used, and marks it as a legacy \
                     feature as well"
                );
            }
            (element, _, reason, alias) => panic!(
                "the registry gives {what} the attribute `deprecated` as {reason:?} on {element:?} \
                 with the alias {alias:?}, which the generator does not know how to state"
            ),
        }
    }
}

pub(crate) fn assert_listed_legacy_aliases_are_left_out(ir: &Registry) {
    for (owner, name) in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT {
        let mut attributes = ir.legacy_attributes.iter();
        assert!(
            attributes.any(|attribute| attribute.element == LegacyElement::Enum
                && attribute.reason == LegacyReason::Aliased
                && attribute.owner.as_deref() == Some(owner)
                && attribute.name == *name),
            "`{name}` of `{owner}` is in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT, but the \
             registry does not call it a legacy alias"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        catch_quietly, field, registry, run_with_the_tool, structure, tagged,
    };
    use vulkan_registry::{Bitmask, Constant, Docs, Enum, Extension, Flags, Legacy, Struct};

    fn value(name: &str, value: &str, aliases: &[&str]) -> EnumField {
        EnumField {
            name: name.to_string(),
            value: value.to_string(),
            aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
            ..Default::default()
        }
    }

    fn ir() -> Registry {
        let mut ir = registry();
        ir.enums.push(Enum {
            name: "VkFormat".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.enums.push(Enum {
            name: "VkColor".to_string(),
            bit_width: 32,
            fields: vec![
                value(
                    "VK_COLOR_RED",
                    "0",
                    &["VK_COLOR_CRIMSON", "VK_COLOR_SCARLET"],
                ),
                value("VK_COLOR_BLUE", "1", &["VK_COLOR_AZURE"]),
            ],
            ..Default::default()
        });
        ir.bitmasks.push(Bitmask {
            name: "VkThingFlagBits".to_string(),
            flags_name: "VkThingFlags".to_string(),
            bit_width: 32,
            fields: vec![value(
                "VK_THING_FAST_BIT",
                "1",
                &["VK_THING_FAST", "VK_THING_QUICK_BIT"],
            )],
            ..Default::default()
        });
        ir.flags.push(Flags {
            name: "VkThingFlags".to_string(),
            bitmask: Some("VkThingFlagBits".to_string()),
            bit_width: 32,
            ..Default::default()
        });
        let mut bits = field("bits", "uint32_t");
        bits.bit_field_width = Some(32);
        ir.structs.push(tagged(
            "VkThing",
            "VK_STRUCTURE_TYPE_THING",
            vec![
                field("count", "uint32_t"),
                field("enabled", "VkBool32"),
                bits,
            ],
        ));
        ir.structs.push(Struct {
            union: true,
            ..structure("VkThingUnion", vec![field("word", "uint32_t")])
        });
        ir.extensions.push(Extension {
            name: "VK_KHR_things_2".to_string(),
            name_macro: "VK_KHR_THINGS_2_EXTENSION_NAME".to_string(),
            spec_version_macro: "VK_KHR_THINGS_2_SPEC_VERSION".to_string(),
            device: true,
            ..Default::default()
        });
        ir.constants.push(Constant {
            name: "VK_MAX_THINGS".to_string(),
            ty: "uint32_t".to_string(),
            value: "8".to_string(),
            video_std_header: None,
        });
        ir
    }

    fn attribute(
        element: LegacyElement,
        owner: Option<&str>,
        name: &str,
        reason: LegacyReason,
        alias: Option<&str>,
    ) -> LegacyAttribute {
        LegacyAttribute {
            element,
            name: name.to_string(),
            owner: owner.map(str::to_string),
            reason,
            alias: alias.map(str::to_string),
        }
    }

    use LegacyElement::{Enum as Value, Member};
    use LegacyReason::{Aliased, True, Unused};

    fn accepted() -> Vec<LegacyAttribute> {
        vec![
            attribute(
                Value,
                Some("VkColor"),
                "VK_COLOR_CRIMSON",
                Aliased,
                Some("VK_COLOR_RED"),
            ),
            attribute(
                Value,
                Some("VkColor"),
                "VK_COLOR_SCARLET",
                Aliased,
                Some("VK_COLOR_CRIMSON"),
            ),
            attribute(Value, Some("VkColor"), "VK_COLOR_BLUE", True, None),
            attribute(
                Value,
                Some("VkThingFlagBits"),
                "VK_THING_FAST",
                Aliased,
                Some("VK_THING_FAST_BIT"),
            ),
            attribute(
                Value,
                None,
                "VK_KHR_THINGS2_EXTENSION_NAME",
                Aliased,
                Some("VK_KHR_THINGS_2_EXTENSION_NAME"),
            ),
            attribute(
                Value,
                None,
                "VK_KHR_THINGS2_SPEC_VERSION",
                Aliased,
                Some("VK_KHR_THINGS_2_SPEC_VERSION"),
            ),
            attribute(Member, Some("VkThing"), "count", Unused, None),
            attribute(Member, Some("VkThing"), "enabled", Unused, None),
        ]
    }

    #[test]
    fn the_statements_follow_the_attribute_of_the_exact_name() {
        let mut ir = ir();
        ir.legacy_attributes = accepted();
        let model = Model::new(&ir);
        assert_every_legacy_attribute_is_stated(&model);
        for (owner, name, is_legacy_alias) in [
            ("VkColor", "VK_COLOR_CRIMSON", true),
            ("VkColor", "VK_COLOR_SCARLET", true),
            ("VkColor", "VK_COLOR_AZURE", false),
            ("VkColor", "VK_COLOR_BLUE", false),
            ("VkColor", "VK_COLOR_RED", false),
            ("VkThingFlagBits", "VK_THING_FAST", true),
            ("VkThingFlagBits", "VK_THING_QUICK_BIT", false),
            ("VkThingFlags", "VK_THING_FAST", false),
            ("VkThingFlagBits", "VK_COLOR_CRIMSON", false),
            ("", "VK_COLOR_CRIMSON", false),
            ("VkThing", "count", false),
        ] {
            assert_eq!(
                model.is_legacy_alias(owner, name),
                is_legacy_alias,
                "{name}"
            );
        }
        for (owner, name, statement) in [
            ("VkColor", "VK_COLOR_BLUE", NO_REASON),
            ("VkColor", "VK_COLOR_RED", ""),
            ("VkColor", "VK_COLOR_CRIMSON", ""),
            ("VkThingFlagBits", "VK_COLOR_BLUE", ""),
            ("VkThing", "count", ""),
        ] {
            assert_eq!(model.legacy_value(owner, name), statement, "{name}");
        }
        for (structure, member, statement) in [
            ("VkThing", "count", NOT_USED),
            ("VkThing", "enabled", NOT_USED),
            ("VkThing", "bits", ""),
            ("VkThingUnion", "count", ""),
            ("VkColor", "VK_COLOR_BLUE", ""),
        ] {
            let from = Cfg::default();
            assert_eq!(
                model.legacy_setter(&from, structure, member),
                statement,
                "{member}"
            );
            for (description, described) in [
                ("", false),
                ("`count` is the number of things.", false),
                ("`count` is legacy.", false),
                (
                    "`count` is legacy and not used. Implementations ignore it.",
                    true,
                ),
                ("`count` is legacy and\nnot  used.", true),
            ] {
                assert_eq!(
                    model.unused_member_unless_described(structure, member, description),
                    if described { "" } else { statement },
                    "{member}: {description}"
                );
            }
        }
    }

    #[test]
    fn a_setter_of_a_legacy_feature_states_the_mark_and_one_of_an_unused_member_that_it_is_not_used()
     {
        let mut ir = ir();
        ir.legacy_attributes = accepted();
        ir.legacy = vec![Legacy {
            kind: LegacyKind::Feature,
            name: "bits".to_string(),
            structure: Some("VkThing".to_string()),
            link: "legacy-things".to_string(),
            marked_by: vec!["VK_VERSION_1_1".to_string()],
            superseded_by: None,
        }];
        let docs = Docs {
            anchors: [(
                "legacy-things".to_string(),
                "appendices/legacy.adoc".to_string(),
            )]
            .into(),
            ..Default::default()
        };
        let model = Model::with_docs(&ir, &docs);
        let from = Cfg::default();
        assert_eq!(
            model.legacy_setter(&from, "VkThing", "bits"),
            model.legacy_member(&from, "VkThing", "bits")
        );
        assert!(
            model
                .legacy_setter(&from, "VkThing", "bits")
                .starts_with("Legacy functionality as of")
        );
        assert_eq!(model.legacy_setter(&from, "VkThing", "count"), NOT_USED);
        assert_eq!(model.legacy_member(&from, "VkThing", "count"), "");
    }

    #[test]
    fn an_attribute_that_the_bindings_cannot_state_aborts_generation() {
        let odd = |attribute: &LegacyAttribute| {
            format!(
                "the registry gives {} the attribute `deprecated` as {:?} on {:?} with the alias {:?}, which the generator does not know how to state",
                described(attribute),
                attribute.reason,
                attribute.element,
                attribute.alias
            )
        };
        let mut cases = vec![
            (
                attribute(Value, Some("VkShade"), "VK_SHADE_DARK", Aliased, Some("VK_SHADE_DIM")),
                "the registry gives the value `VK_SHADE_DARK` of `VkShade` the attribute `deprecated`, and the registry data has no enumeration `VkShade`".to_string(),
            ),
            (
                attribute(Value, Some("VkThing"), "VK_THING_FAST", True, None),
                "the registry gives the value `VK_THING_FAST` of `VkThing` the attribute `deprecated`, and the registry data has no enumeration `VkThing`".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_COLOR_RED", Aliased, Some("VK_COLOR_CRIMSON")),
                "the registry calls the value `VK_COLOR_RED` of `VkColor` a legacy alias, and the registry data has no value with this alias".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_THING_FAST", Aliased, Some("VK_THING_FAST_BIT")),
                "the registry calls the value `VK_THING_FAST` of `VkColor` a legacy alias, and the registry data has no value with this alias".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_COLOR_CRIMSON", Aliased, Some("VK_COLOR_BLUE")),
                "the registry calls the value `VK_COLOR_CRIMSON` of `VkColor` a legacy alias of `VK_COLOR_BLUE`, and the registry data has it as an alias of `VK_COLOR_RED`, which `VK_COLOR_BLUE` does not name".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_COLOR_AZURE", Aliased, Some("VK_COLOR_CRIMSON")),
                "the registry calls the value `VK_COLOR_AZURE` of `VkColor` a legacy alias of `VK_COLOR_CRIMSON`, and the registry data has it as an alias of `VK_COLOR_BLUE`, which `VK_COLOR_CRIMSON` does not name".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_COLOR_CRIMSON", True, None),
                "the registry calls the value `VK_COLOR_CRIMSON` of `VkColor` legacy, and the registry data has no such value".to_string(),
            ),
            (
                attribute(Value, Some("VkColor"), "VK_THING_FAST_BIT", True, None),
                "the registry calls the value `VK_THING_FAST_BIT` of `VkColor` legacy, and the registry data has no such value".to_string(),
            ),
            (
                attribute(Value, None, "VK_MAX_THING", Aliased, Some("VK_MAX_THINGS")),
                "the registry calls the constant `VK_MAX_THING` a legacy alias of `VK_MAX_THINGS`: the bindings leave out the legacy aliases of the name and of the version macro of an extension, and nothing else".to_string(),
            ),
            (
                attribute(Value, None, "VK_MAX_THINGS", Aliased, Some("VK_KHR_THINGS_2_SPEC_VERSION")),
                "the registry calls the constant `VK_MAX_THINGS` a legacy alias of `VK_KHR_THINGS_2_SPEC_VERSION`: the bindings leave out the legacy aliases of the name and of the version macro of an extension, and nothing else".to_string(),
            ),
            (
                attribute(Member, Some("VkThing"), "missing", Unused, None),
                "the registry calls the member `missing` of `VkThing` legacy and not used, and the bindings have no such member with a setter".to_string(),
            ),
            (
                attribute(Member, Some("VkMissing"), "count", Unused, None),
                "the registry calls the member `count` of `VkMissing` legacy and not used, and the bindings have no such member with a setter".to_string(),
            ),
            (
                attribute(Member, Some("VkThing"), "bits", Unused, None),
                "the registry calls the member `bits` of `VkThing` legacy and not used, and the bindings have no such member with a setter".to_string(),
            ),
            (
                attribute(Member, Some("VkThing"), "sType", Unused, None),
                "the registry calls the member `sType` of `VkThing` legacy and not used, and the bindings have no such member with a setter".to_string(),
            ),
            (
                attribute(Member, Some("VkThingUnion"), "word", Unused, None),
                "the registry calls the member `word` of `VkThingUnion` legacy and not used, and the bindings have no such member with a setter".to_string(),
            ),
        ];
        for unknown in [
            attribute(Value, Some("VkColor"), "VK_COLOR_CRIMSON", Aliased, None),
            attribute(
                Value,
                Some("VkColor"),
                "VK_COLOR_BLUE",
                True,
                Some("VK_COLOR_RED"),
            ),
            attribute(Value, Some("VkColor"), "VK_COLOR_BLUE", Unused, None),
            attribute(Value, None, "VK_KHR_THINGS2_EXTENSION_NAME", Aliased, None),
            attribute(Value, None, "VK_KHR_THINGS_2_EXTENSION_NAME", True, None),
            attribute(Value, None, "VK_KHR_THINGS_2_EXTENSION_NAME", Unused, None),
            attribute(Member, Some("VkThing"), "count", True, None),
            attribute(Member, Some("VkThing"), "count", Aliased, Some("enabled")),
            attribute(Member, Some("VkThing"), "count", Unused, Some("enabled")),
            attribute(Member, None, "count", Unused, None),
        ] {
            let message = odd(&unknown);
            cases.push((unknown, message));
        }
        assert!(cases[15].1.ends_with("the attribute `deprecated` as Aliased on Enum with the alias None, which the generator does not know how to state"));
        assert!(
            cases[24]
                .1
                .starts_with("the registry gives the member `count` the attribute")
        );
        for (attribute, message) in cases {
            let mut ir = ir();
            ir.legacy_attributes = vec![attribute];
            assert_eq!(
                catch_quietly(|| crate::generate(&ir, &Docs::default())).map(|_| ()),
                Err(message)
            );
        }
    }

    #[test]
    fn a_member_that_is_a_legacy_feature_and_not_used_aborts_generation() {
        let mut ir = ir();
        ir.legacy_attributes = vec![attribute(Member, Some("VkThing"), "count", Unused, None)];
        let feature = |structure: &str, name: &str| Legacy {
            kind: LegacyKind::Feature,
            name: name.to_string(),
            structure: Some(structure.to_string()),
            link: "legacy-things".to_string(),
            marked_by: vec!["VK_VERSION_1_1".to_string()],
            superseded_by: None,
        };
        let docs = Docs {
            anchors: [(
                "legacy-things".to_string(),
                "appendices/legacy.adoc".to_string(),
            )]
            .into(),
            ..Default::default()
        };
        ir.legacy = vec![feature("VkThing", "enabled")];
        assert_every_legacy_attribute_is_stated(&Model::with_docs(&ir, &docs));
        ir.legacy = vec![feature("VkThing", "count")];
        assert_eq!(
            catch_quietly(|| assert_every_legacy_attribute_is_stated(&Model::with_docs(&ir, &docs))),
            Err("the registry calls the member `count` of `VkThing` legacy and not used, and marks it as a legacy feature as well".to_string())
        );
    }

    #[test]
    fn an_attribute_that_the_registry_data_has_twice_aborts_generation() {
        let mut ir = ir();
        ir.legacy_attributes = vec![
            attribute(Value, Some("VkColor"), "VK_COLOR_BLUE", True, None),
            attribute(Value, Some("VkThingFlagBits"), "VK_COLOR_BLUE", True, None),
            attribute(Value, None, "VK_COLOR_BLUE", True, None),
            attribute(Member, Some("VkColor"), "VK_COLOR_BLUE", True, None),
        ];
        assert_eq!(legacy_attributes(&ir).len(), 4);
        ir.legacy_attributes.push(attribute(
            Member,
            Some("VkColor"),
            "VK_COLOR_BLUE",
            Unused,
            None,
        ));
        assert_eq!(
            catch_quietly(|| legacy_attributes(&ir).len()),
            Err("the registry data gives the member `VK_COLOR_BLUE` of `VkColor` the attribute `deprecated` twice".to_string())
        );
        ir.legacy_attributes[4] = attribute(Value, None, "VK_COLOR_BLUE", Aliased, None);
        assert_eq!(
            catch_quietly(|| legacy_attributes(&ir).len()),
            Err("the registry data gives the constant `VK_COLOR_BLUE` the attribute `deprecated` twice".to_string())
        );
    }

    #[test]
    fn a_listed_alias_is_passed_over_only_while_the_registry_data_leaves_it_out() {
        let (owner, name) = ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT[0];
        let mut ir = ir();
        ir.enums.push(Enum {
            name: owner.to_string(),
            bit_width: 32,
            fields: vec![value("STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED", "2", &[])],
            ..Default::default()
        });
        let listed = attribute(
            Value,
            Some(owner),
            name,
            Aliased,
            Some("STD_VIDEO_AV1_COLOR_PRIMARIES_UNSPECIFIED"),
        );
        ir.legacy_attributes = vec![listed.clone()];
        assert_every_legacy_attribute_is_stated(&Model::new(&ir));
        assert_listed_legacy_aliases_are_left_out(&ir);

        let mut with_the_alias = ir.clone();
        with_the_alias
            .enums
            .last_mut()
            .expect("the enumeration")
            .fields[0]
            .aliases = vec![name.to_string()];
        assert_eq!(
            catch_quietly(|| assert_every_legacy_attribute_is_stated(&Model::new(&with_the_alias))),
            Err(format!(
                "the value `{name}` of `{owner}` is in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT, but the registry data has it"
            ))
        );
        let stale = format!(
            "`{name}` of `{owner}` is in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT, but the registry does not call it a legacy alias"
        );
        for other in [
            attribute(Value, Some("VkColor"), name, Aliased, None),
            attribute(Value, Some(owner), "VK_COLOR_CRIMSON", Aliased, None),
            attribute(Member, Some(owner), name, Unused, None),
            attribute(Value, Some(owner), name, True, None),
        ] {
            let mut ir = ir.clone();
            ir.legacy_attributes = vec![other];
            assert_eq!(
                catch_quietly(|| assert_listed_legacy_aliases_are_left_out(&ir)),
                Err(stale.clone())
            );
        }
        let mut in_another_enumeration = ir.clone();
        in_another_enumeration.legacy_attributes = vec![attribute(
            Value,
            Some("VkColor"),
            name,
            Aliased,
            Some("VK_COLOR_RED"),
        )];
        assert_eq!(
            catch_quietly(|| assert_every_legacy_attribute_is_stated(&Model::new(
                &in_another_enumeration
            ))),
            Err(format!(
                "the registry calls the value `{name}` of `VkColor` a legacy alias, and the registry data has no value with this alias"
            ))
        );
    }

    #[test]
    fn every_legacy_attribute_of_the_registry_can_be_stated() {
        let ir = Registry::get();
        assert_every_legacy_attribute_is_stated(&Model::new(ir));
        assert_listed_legacy_aliases_are_left_out(ir);
        let count = |element, reason, owned: bool| {
            let attributes = ir.legacy_attributes.iter();
            attributes
                .filter(|attribute| {
                    (
                        attribute.element,
                        attribute.reason,
                        attribute.owner.is_some(),
                    ) == (element, reason, owned)
                })
                .count()
        };
        assert_eq!(ir.legacy_attributes.len(), 46);
        assert_eq!(
            [
                count(Value, Aliased, true),
                count(Value, Aliased, false),
                count(Value, True, true),
                count(Member, Unused, true)
            ],
            [33, 10, 1, 2],
            "legacy aliases of values, legacy aliases of extension macros, a legacy value and unused members"
        );
    }

    fn dumped(
        registry: &[&str],
        supported: &[&str],
    ) -> Option<Result<Vec<LegacyAttribute>, String>> {
        let steps = [
            "with tempfile.TemporaryDirectory() as directory:",
            "    xml = pathlib.Path(directory) / 'vk.xml'",
            "    xml.write_text(sys.stdin.read())",
            "    loaded, _ = tool.load_registry(xml)",
            "attributes = tool.legacy_attributes(loaded.reg)",
            "attributes = tool.legacy_attributes_ir(set(sys.argv[2:]), attributes)",
            "lists = ('vendor_tags', 'video_headers', 'versions', 'constants', 'handles', 'enums', 'bitmasks')",
            "lists += ('flags', 'func_pointers', 'structs', 'commands', 'extensions', 'formats', 'legacy')",
            "ir = {name: [] for name in lists}",
            "ir.update(header_version=0, header_version_complete='', headers_tag='', headers_commit='')",
            "ir.update(platforms={}, legacy_attributes=attributes)",
            "print(json.dumps(ir))",
        ];
        let json = match run_with_the_tool(&steps, supported, &registry.join("\n"))? {
            Ok(json) => json,
            Err(error) => return Some(Err(error)),
        };
        let ir = Registry::from_json(&json).unwrap_or_else(|error| panic!("{error}: {json}"));
        Some(Ok(ir.legacy_attributes))
    }

    const REGISTRY: &[&str] = &[
        "<registry>",
        "  <types>",
        "    <type category='struct' name='VkThing'>",
        "      <member deprecated='unused'><type>uint32_t</type> <name>count</name></member>",
        "      <member><type>uint32_t</type> <name>used</name></member>",
        "    </type>",
        "  </types>",
        "  <enums name='API Constants' type='constants'>",
        "    <enum name='VK_OLD_MAX' alias='VK_MAX' deprecated='aliased'/>",
        "  </enums>",
        "  <enums name='VkColor' type='enum'>",
        "    <enum value='0' name='VK_COLOR_RED'/>",
        "    <enum name='VK_COLOR_CRIMSON' alias='VK_COLOR_RED' deprecated='aliased'/>",
        "  </enums>",
        "  <enums name='VkThingFlagBits' type='bitmask'>",
        "    <enum bitpos='0' name='VK_THING_FAST_BIT' deprecated='true'/>",
        "  </enums>",
        "  <feature api='vulkan' name='VK_VERSION_1_1' number='1.1'>",
        "    <require>",
        "      <enum extends='VkColor' name='VK_COLOR_SCARLET' alias='VK_COLOR_RED' deprecated='aliased'/>",
        "      <enum extends='VkColor' name='VK_COLOR_CRIMSON' alias='VK_COLOR_RED' deprecated='aliased'/>",
        "    </require>",
        "  </feature>",
        "  <extensions>",
        "    <extension name='VK_KHR_unsupported' number='1' supported='disabled'>",
        "      <require>",
        "        <enum extends='VkColor' name='VK_COLOR_CRIMSON' alias='VK_COLOR_BLUE' deprecated='aliased'/>",
        "        <enum extends='VkColor' name='VK_COLOR_NAVY' alias='VK_COLOR_BLUE' deprecated='aliased'/>",
        "      </require>",
        "    </extension>",
        "    <extension name='VK_KHR_things_2' number='2' supported='vulkan'>",
        "      <require>",
        "        <enum name='VK_KHR_THINGS2_EXTENSION_NAME' alias='VK_KHR_THINGS_2_EXTENSION_NAME' deprecated='aliased'/>",
        "        <enum extends='VkColor' name='VK_COLOR_SCARLET' alias='VK_COLOR_RED' deprecated='aliased'/>",
        "        <enum api='vulkansc' extends='VkColor' name='VK_COLOR_SAFE' alias='VK_COLOR_RED' deprecated='aliased'/>",
        "        <enum extends='VkThingFlagBits' name='VK_THING_QUICK' alias='VK_THING_FAST_BIT' deprecated='aliased'/>",
        "        <enum extends='VkColor' name='VK_COLOR_GREEN' offset='1'/>",
        "      </require>",
        "    </extension>",
        "  </extensions>",
        "</registry>",
    ];

    type Dumped<'a> = (
        LegacyElement,
        Option<&'a str>,
        &'a str,
        LegacyReason,
        Option<&'a str>,
    );

    fn fields(attributes: &[LegacyAttribute]) -> Vec<Dumped<'_>> {
        let fields = attributes.iter().map(|attribute| {
            (
                attribute.element,
                attribute.owner.as_deref(),
                attribute.name.as_str(),
                attribute.reason,
                attribute.alias.as_deref(),
            )
        });
        fields.collect()
    }

    #[test]
    fn the_dump_has_each_name_with_the_attribute_once_with_its_owner_reason_and_alias() {
        let Some(attributes) = dumped(REGISTRY, &["VK_KHR_things_2", "VK_KHR_other"]) else {
            return;
        };
        let attributes = attributes.unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            fields(&attributes),
            [
                (
                    Value,
                    None,
                    "VK_KHR_THINGS2_EXTENSION_NAME",
                    Aliased,
                    Some("VK_KHR_THINGS_2_EXTENSION_NAME")
                ),
                (Value, None, "VK_OLD_MAX", Aliased, Some("VK_MAX")),
                (
                    Value,
                    Some("VkColor"),
                    "VK_COLOR_CRIMSON",
                    Aliased,
                    Some("VK_COLOR_RED")
                ),
                (
                    Value,
                    Some("VkColor"),
                    "VK_COLOR_SCARLET",
                    Aliased,
                    Some("VK_COLOR_RED")
                ),
                (
                    Value,
                    Some("VkThingFlagBits"),
                    "VK_THING_FAST_BIT",
                    True,
                    None
                ),
                (
                    Value,
                    Some("VkThingFlagBits"),
                    "VK_THING_QUICK",
                    Aliased,
                    Some("VK_THING_FAST_BIT")
                ),
                (Member, Some("VkThing"), "count", Unused, None),
            ]
        );
    }

    #[test]
    fn the_dump_leaves_out_what_only_an_extension_that_is_not_supported_has() {
        let navy = (
            Value,
            Some("VkColor"),
            "VK_COLOR_NAVY",
            Aliased,
            Some("VK_COLOR_BLUE"),
        );
        let crimson = "<enum extends='VkColor' name='VK_COLOR_CRIMSON' alias='VK_COLOR_BLUE' deprecated='aliased'/>";
        let registry = REGISTRY.join("\n");
        assert_eq!(registry.matches(crimson).count(), 1);
        let without_the_conflict = registry.replacen(crimson, "", 1);
        for (supported, has_navy) in [
            (&["VK_KHR_things_2"][..], false),
            (&["VK_KHR_things_2", "VK_KHR_unsupported"][..], true),
        ] {
            let Some(attributes) = dumped(&[&without_the_conflict], supported) else {
                return;
            };
            let attributes = attributes.unwrap_or_else(|error| panic!("{error}"));
            assert_eq!(
                fields(&attributes).contains(&navy),
                has_navy,
                "{supported:?}"
            );
            assert_eq!(attributes.len(), if has_navy { 8 } else { 7 });
        }

        let Some(attributes) = dumped(REGISTRY, &["VK_KHR_things_2", "VK_KHR_unsupported"]) else {
            return;
        };
        let error = attributes.expect_err("an alias with two targets is dumped");
        assert!(
            error.trim_end().ends_with(
                "RuntimeError: VK_COLOR_CRIMSON has the attribute `deprecated` twice, as {'element': 'enum', 'name': 'VK_COLOR_CRIMSON', 'owner': 'VkColor', 'reason': 'aliased', 'alias': 'VK_COLOR_RED'} and as {'element': 'enum', 'name': 'VK_COLOR_CRIMSON', 'owner': 'VkColor', 'reason': 'aliased', 'alias': 'VK_COLOR_BLUE'}"
            ),
            "{error}"
        );
    }

    #[test]
    fn the_dump_stops_at_an_attribute_that_it_does_not_know_and_at_a_name_with_two() {
        for (replace, with, message) in [
            (
                "<enum bitpos='0' name='VK_THING_FAST_BIT' deprecated='true'/>",
                "<enum bitpos='0' name='VK_THING_FAST_BIT' deprecated='ignored'/>",
                "VK_THING_FAST_BIT has the attribute deprecated=\"ignored\", a value that this script does not know",
            ),
            (
                "<type category='struct' name='VkThing'>",
                "<type category='struct' name='VkThing' deprecated='true'>",
                "a <type> (VkThing) has the attribute deprecated=\"true\", which this script knows on <enum> and <member> only",
            ),
            (
                "<feature api='vulkan' name='VK_VERSION_1_1' number='1.1'>",
                "<commands><command deprecated='true'><proto><type>void</type> <name>vkOld</name></proto></command></commands><feature api='vulkan' name='VK_VERSION_1_1' number='1.1'>",
                "a <command> (vkOld) has the attribute deprecated=\"true\", which this script knows on <enum> and <member> only",
            ),
            (
                "<enum value='0' name='VK_COLOR_RED'/>",
                "<enum value='0' deprecated='true'/>",
                "a <enum> with the attribute deprecated=\"true\" has no name, or is a member of no type: {'value': '0', 'deprecated': 'true'}",
            ),
            (
                "<member><type>uint32_t</type> <name>used</name></member>",
                "<member deprecated='unused'><type>uint32_t</type></member>",
                "a <member> with the attribute deprecated=\"unused\" has no name, or is a member of no type: {'deprecated': 'unused'}",
            ),
            (
                "<enums name='VkThingFlagBits' type='bitmask'>",
                "<member deprecated='unused'><name>stray</name></member><enums name='VkThingFlagBits' type='bitmask'>",
                "a <member> with the attribute deprecated=\"unused\" has no name, or is a member of no type: {'deprecated': 'unused'}",
            ),
            (
                "<enum extends='VkColor' name='VK_COLOR_GREEN' offset='1'/>",
                "<enum extends='VkColor' name='VK_COLOR_CRIMSON' alias='VK_COLOR_GREEN' deprecated='aliased'/>",
                "VK_COLOR_CRIMSON has the attribute `deprecated` twice, as {'element': 'enum', 'name': 'VK_COLOR_CRIMSON', 'owner': 'VkColor', 'reason': 'aliased', 'alias': 'VK_COLOR_RED'} and as {'element': 'enum', 'name': 'VK_COLOR_CRIMSON', 'owner': 'VkColor', 'reason': 'aliased', 'alias': 'VK_COLOR_GREEN'}",
            ),
            (
                "<enum extends='VkColor' name='VK_COLOR_GREEN' offset='1'/>",
                "<enum extends='VkColor' name='VK_COLOR_SCARLET' deprecated='true'/>",
                "VK_COLOR_SCARLET has the attribute `deprecated` twice, as {'element': 'enum', 'name': 'VK_COLOR_SCARLET', 'owner': 'VkColor', 'reason': 'aliased', 'alias': 'VK_COLOR_RED'} and as {'element': 'enum', 'name': 'VK_COLOR_SCARLET', 'owner': 'VkColor', 'reason': 'true', 'alias': None}",
            ),
        ] {
            let registry = REGISTRY.join("\n");
            assert_eq!(registry.matches(replace).count(), 1, "{replace}");
            let registry = registry.replacen(replace, with, 1);
            let Some(attributes) = dumped(&[&registry], &["VK_KHR_things_2"]) else {
                return;
            };
            let error = attributes.expect_err(&format!("`{with}` is dumped"));
            assert!(
                error
                    .trim_end()
                    .ends_with(&format!("RuntimeError: {message}")),
                "{error}"
            );
        }
    }
}
