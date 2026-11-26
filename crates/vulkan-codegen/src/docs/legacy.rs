use super::join_and;
use crate::emit::version::MACROS_OF_EVERY_HEADER;
use crate::model::{Cfg, Model, named_version};
use std::collections::BTreeMap;
use vulkan_registry::{Legacy, LegacyKind, Registry};

const SPECIFICATION: &str = "https://docs.vulkan.org/spec/latest/";

const MACROS_THAT_THE_BINDINGS_LEAVE_OUT: &[&str] = &[
    "VK_API_VERSION",
    "VK_MAKE_VERSION",
    "VK_VERSION_MAJOR",
    "VK_VERSION_MINOR",
    "VK_VERSION_PATCH",
];

pub(crate) type LegacyMarks<'a> = BTreeMap<(LegacyKind, &'a str, &'a str), &'a Legacy>;

fn described(mark: &Legacy) -> String {
    match (mark.kind, &mark.structure) {
        (LegacyKind::Type, _) => format!("the type `{}`", mark.name),
        (LegacyKind::Command, _) => format!("the command `{}`", mark.name),
        (LegacyKind::Feature, Some(structure)) => {
            format!("the feature `{}` of `{structure}`", mark.name)
        }
        (LegacyKind::Feature, None) => format!("the feature `{}`", mark.name),
    }
}

pub(crate) fn legacy_marks(ir: &Registry) -> LegacyMarks<'_> {
    let mut marks = LegacyMarks::new();
    for mark in &ir.legacy {
        let structure = mark.structure.as_deref().unwrap_or_default();
        assert!(
            marks
                .insert((mark.kind, structure, &mark.name), mark)
                .is_none(),
            "the registry data marks {} as legacy twice",
            described(mark)
        );
    }
    marks
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Promotion {
    Core,
    Khr,
    Ext,
    Vendor,
}

impl Promotion {
    fn of_author(author: Option<&str>) -> Self {
        match author {
            None => Promotion::Core,
            Some("KHR") => Promotion::Khr,
            Some("EXT") => Promotion::Ext,
            Some(_) => Promotion::Vendor,
        }
    }

    fn of_extension(name: &str) -> Self {
        Self::of_author(name.split('_').nth(1))
    }

    fn of_name(vendor_tags: &[String], c_name: &str) -> Self {
        let mut tags = vendor_tags
            .iter()
            .filter(|tag| c_name.ends_with(tag.as_str()));
        let tag = tags.next();
        if let (Some(tag), Some(other)) = (tag, tags.next()) {
            panic!("`{c_name}` ends with the two vendor tags `{tag}` and `{other}`");
        }
        Self::of_author(tag.map(String::as_str))
    }
}

impl Model<'_> {
    pub(crate) fn specification_url(&self, anchor: &str) -> String {
        let source = self.docs.anchors.get(anchor).unwrap_or_else(|| {
            panic!("the descriptions name no page of the specification for the anchor `{anchor}`")
        });
        let page = source.strip_suffix(".adoc").unwrap_or_else(|| {
            panic!(
                "the descriptions name `{source}` as the page of the anchor `{anchor}`, which \
                 is not an AsciiDoc source of the specification"
            )
        });
        format!("{SPECIFICATION}{page}.html#{anchor}")
    }

    fn legacy_statement(&self, from: &Cfg, mark: &Legacy) -> String {
        let successor = mark.superseded_by.as_deref().map(|successor| {
            assert!(
                self.links.get(successor).is_some(),
                "the registry marks {} as legacy and superseded by `{successor}`, which the \
                 bindings have no item for",
                described(mark)
            );
            self.link(from, successor)
        });
        assert!(
            !mark.marked_by.is_empty(),
            "the registry data marks {} as legacy without the version or the extension that \
             does so",
            described(mark)
        );
        let section = self.specification_url(&mark.link);
        let marked = Promotion::of_name(
            &self.ir.vendor_tags,
            mark.structure.as_deref().unwrap_or(&mark.name),
        );
        let shown = mark
            .marked_by
            .iter()
            .filter_map(|by| self.legacy_marker(from, mark, by, marked))
            .collect::<Vec<_>>();
        let is_a_type_alias =
            mark.kind == LegacyKind::Type && self.canonical(&mark.name) != mark.name;
        if shown.is_empty() || is_a_type_alias {
            return String::new();
        }
        let what = match successor {
            Some(successor) => format!(", superseded by {successor}"),
            None => format!(" as of {}", join_and(shown)),
        };
        format!(
            "Legacy functionality{what}: see [Legacy Functionality]({section}) in the Vulkan specification."
        )
    }

    fn legacy_marker(
        &self,
        from: &Cfg,
        mark: &Legacy,
        by: &str,
        marked: Promotion,
    ) -> Option<String> {
        let used_in = || format!("the legacy mark of {}", described(mark));
        if let Some((major, minor)) = named_version(by, used_in) {
            return Some(format!("Vulkan {major}.{minor}"));
        }
        assert!(
            self.ir.extensions.iter().any(|ext| ext.name == by),
            "`{by}` in {} is neither a version nor an extension of the registry",
            used_in()
        );
        (Promotion::of_extension(by) <= marked).then(|| self.link(from, by))
    }

    fn legacy(&self, from: &Cfg, kind: LegacyKind, structure: &str, name: &str) -> String {
        self.legacy
            .get(&(kind, structure, name))
            .map(|mark| self.legacy_statement(from, mark))
            .unwrap_or_default()
    }

    pub(crate) fn legacy_type(&self, from: &Cfg, c_name: &str) -> String {
        self.legacy(from, LegacyKind::Type, "", c_name)
    }

    pub(crate) fn legacy_command(&self, from: &Cfg, c_name: &str) -> String {
        self.legacy(from, LegacyKind::Command, "", c_name)
    }

    pub(crate) fn legacy_member(&self, from: &Cfg, structure: &str, member: &str) -> String {
        self.legacy(from, LegacyKind::Feature, structure, member)
    }

    fn documents_the_type(&self, c_name: &str) -> bool {
        let ir = self.ir;
        ir.structs.iter().any(|s| s.name == c_name)
            || ir.handles.iter().any(|h| h.name == c_name)
            || ir.enums.iter().any(|e| e.name == c_name)
            || ir.bitmasks.iter().any(|b| b.name == c_name)
            || ir.flags.iter().any(|f| f.name == c_name)
            || ir.func_pointers.iter().any(|fp| fp.name == c_name)
            || self.canonical(c_name) != c_name
            || MACROS_OF_EVERY_HEADER.contains(&c_name)
            || ir.versions.iter().any(|v| v.name_api == c_name)
    }

    fn documents_the_member(&self, structure: &str, member: &str) -> bool {
        let members = self
            .ir
            .structs
            .iter()
            .filter(|s| s.name == structure && !s.union)
            .flat_map(|s| &s.members);
        members
            .filter(|m| m.bit_field_width.is_none() && m.name != "sType" && m.name != "pNext")
            .any(|m| m.name == member)
    }
}

pub(crate) fn assert_every_legacy_mark_has_an_item(model: &Model<'_>) {
    for mark in &model.ir.legacy {
        let documented = match (mark.kind, &mark.structure) {
            (LegacyKind::Type, None) => model.documents_the_type(&mark.name),
            (LegacyKind::Command, None) => model.ir.commands.iter().any(|c| c.name == mark.name),
            (LegacyKind::Feature, Some(structure)) => {
                model.documents_the_member(structure, &mark.name)
            }
            (LegacyKind::Type | LegacyKind::Command, Some(structure)) => panic!(
                "the registry data marks {} as legacy in the structure `{structure}`, which only a feature has",
                described(mark)
            ),
            (LegacyKind::Feature, None) => panic!(
                "the registry data marks {} as legacy without its structure",
                described(mark)
            ),
        };
        model.legacy_statement(&Cfg::default(), mark);
        if mark.kind == LegacyKind::Type
            && MACROS_THAT_THE_BINDINGS_LEAVE_OUT.contains(&mark.name.as_str())
        {
            assert!(
                model.links.get(&mark.name).is_none(),
                "`{}` is in MACROS_THAT_THE_BINDINGS_LEAVE_OUT, but the bindings have an item for it",
                mark.name
            );
            continue;
        }
        assert!(
            documented,
            "the registry marks {} as legacy, and the bindings have no item that the generator \
             documents for it",
            described(mark)
        );
    }
}

pub(crate) fn assert_listed_legacy_macros_are_marked(ir: &Registry) {
    for name in MACROS_THAT_THE_BINDINGS_LEAVE_OUT {
        assert!(
            ir.legacy
                .iter()
                .any(|mark| mark.kind == LegacyKind::Type && mark.name == *name),
            "`{name}` is in MACROS_THAT_THE_BINDINGS_LEAVE_OUT, but the registry does not mark \
             it as legacy"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        catch_quietly, command, field, registry, run_with_the_tool, structure,
    };
    use std::collections::BTreeSet;
    use vulkan_registry::{
        Bitmask, Constant, Docs, Enum, Extension, Flags, FuncPointer, Handle, Struct, Version,
    };

    const WAYLAND: &str = "VK_USE_PLATFORM_WAYLAND_KHR";
    const SECTION: &str =
        "https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things";

    fn extension(name: &str, name_macro: &str) -> Extension {
        Extension {
            name: name.to_string(),
            name_macro: name_macro.to_string(),
            spec_version_macro: name_macro.replace("_EXTENSION_NAME", "_SPEC_VERSION"),
            device: true,
            ..Default::default()
        }
    }

    fn ir() -> Registry {
        let mut ir = registry();
        ir.platforms
            .insert("wayland".to_string(), WAYLAND.to_string());
        ir.enums.push(Enum {
            name: "VkFormat".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.handles.push(Handle {
            name: "VkDevice".to_string(),
            object_type: "VK_OBJECT_TYPE_DEVICE".to_string(),
            dispatchable: true,
            ..Default::default()
        });
        ir.bitmasks.push(Bitmask {
            name: "VkThingFlagBits".to_string(),
            flags_name: "VkThingFlags".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.flags.push(Flags {
            name: "VkThingFlags".to_string(),
            bitmask: Some("VkThingFlagBits".to_string()),
            bit_width: 32,
            ..Default::default()
        });
        ir.func_pointers.push(FuncPointer {
            name: "PFN_vkThingFunction".to_string(),
            return_ty: "void".to_string(),
            ..Default::default()
        });
        ir.versions.push(Version {
            name: "VK_VERSION_1_1".to_string(),
            name_api: "VK_API_VERSION_1_1".to_string(),
        });
        let mut bits = field("bits", "uint32_t");
        bits.bit_field_width = Some(32);
        ir.structs.push(structure(
            "VkThing",
            vec![
                field("sType", "VkStructureType"),
                field("pNext", "void*"),
                field("enabled", "VkBool32"),
                bits,
            ],
        ));
        ir.structs.push(structure(
            "VkThingEXT",
            vec![field("enabled", "VkBool32"), field("enabledNV", "VkBool32")],
        ));
        ir.structs.push(structure("VkThingNV", vec![]));
        ir.structs.push(structure("VkThingKHR", vec![]));
        let mut thing2 = structure("VkThing2", vec![]);
        thing2.aliases = vec!["VkThing2KHR".to_string()];
        ir.structs.push(thing2);
        let mut gated = structure("VkGatedThing", vec![]);
        gated.protect = Some(WAYLAND.to_string());
        ir.structs.push(gated);
        ir.structs.push(Struct {
            union: true,
            ..structure("VkThingUnion", vec![field("word", "uint32_t")])
        });
        for name in ["vkUseThing", "vkUseThing2", "vkUseThingKHR"] {
            let mut c = command(name, vec![field("device", "VkDevice")]);
            c.device = true;
            ir.commands.push(c);
        }
        ir.extensions = vec![
            extension("VK_KHR_things", "VK_KHR_THINGS_EXTENSION_NAME"),
            extension("VK_KHR_more", "VK_KHR_MORE_EXTENSION_NAME"),
            extension("VK_EXT_things", "VK_EXT_THINGS_EXTENSION_NAME"),
            extension("VK_NV_things", "VK_NV_THINGS_EXTENSION_NAME"),
        ];
        ir
    }

    fn docs() -> Docs {
        Docs {
            anchors: [
                ("legacy-things", "appendices/legacy.adoc"),
                ("things-chapter", "chapters/things.adoc"),
                ("things-nested", "chapters/things/more.adoc"),
                ("things-text", "chapters/things.txt"),
            ]
            .into_iter()
            .map(|(anchor, file)| (anchor.to_string(), file.to_string()))
            .collect(),
            ..Default::default()
        }
    }

    fn mark(kind: LegacyKind, name: &str) -> Legacy {
        Legacy {
            kind,
            name: name.to_string(),
            structure: None,
            link: "legacy-things".to_string(),
            marked_by: vec!["VK_VERSION_1_1".to_string()],
            superseded_by: None,
        }
    }

    fn feature(structure: &str, name: &str) -> Legacy {
        Legacy {
            structure: Some(structure.to_string()),
            ..mark(LegacyKind::Feature, name)
        }
    }

    fn superseded(kind: LegacyKind, name: &str, by: &str) -> Legacy {
        Legacy {
            superseded_by: Some(by.to_string()),
            ..mark(kind, name)
        }
    }

    fn marked_by(kind: LegacyKind, name: &str, by: &[&str]) -> Legacy {
        Legacy {
            marked_by: by.iter().map(|by| by.to_string()).collect(),
            ..mark(kind, name)
        }
    }

    fn statement(successor: &str, url: &str) -> String {
        format!(
            "Legacy functionality, superseded by {successor}: see [Legacy Functionality]({url}) in the Vulkan specification."
        )
    }

    fn as_of(marked_by: &str, url: &str) -> String {
        format!(
            "Legacy functionality as of {marked_by}: see [Legacy Functionality]({url}) in the Vulkan specification."
        )
    }

    #[test]
    fn the_statement_names_the_successor_or_else_the_versions_and_extensions_that_mark_the_name() {
        let mut ir = ir();
        ir.legacy = vec![
            superseded(LegacyKind::Type, "VkThing", "VkThing2"),
            superseded(LegacyKind::Command, "vkUseThing", "vkUseThing2"),
            marked_by(
                LegacyKind::Type,
                "VkThingKHR",
                &["VK_VERSION_1_1", "VK_KHR_things", "VK_KHR_more"],
            ),
            marked_by(
                LegacyKind::Type,
                "VkThing2KHR",
                &["VK_VERSION_1_1", "VK_KHR_things"],
            ),
            superseded(LegacyKind::Command, "vkUseThingKHR", "VkThing2KHR"),
            marked_by(
                LegacyKind::Type,
                "VkDevice",
                &["VK_VERSION_1_0", "VK_VERSION_1_1"],
            ),
            feature("VkThing", "enabled"),
        ];
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        let from = Cfg::default();
        assert_eq!(
            model.legacy_type(&from, "VkThing"),
            statement("[`VkThing2`](Thing2)", SECTION)
        );
        assert_eq!(
            model.legacy_command(&from, "vkUseThing"),
            statement("[`vkUseThing2`](crate::Device::use_thing2)", SECTION)
        );
        assert_eq!(
            model.legacy_type(&from, "VkThingKHR"),
            as_of(
                "Vulkan 1.1, [`VK_KHR_things`](KHR_THINGS_NAME) and [`VK_KHR_more`](KHR_MORE_NAME)",
                SECTION
            )
        );
        assert_eq!(
            model.legacy_type(&from, "VkThing2KHR"),
            "",
            "the specification shows no mark of a type alias"
        );
        assert_eq!(
            model.legacy_command(&from, "vkUseThingKHR"),
            statement("[`VkThing2KHR`](Thing2KHR)", SECTION),
            "an alias can be the successor"
        );
        assert_eq!(
            model.legacy_type(&from, "VkDevice"),
            as_of("Vulkan 1.0 and Vulkan 1.1", SECTION)
        );
        assert_eq!(
            model.legacy_member(&from, "VkThing", "enabled"),
            as_of("Vulkan 1.1", SECTION)
        );
        for unmarked in [
            model.legacy_type(&from, "VkThing2"),
            model.legacy_type(&from, "vkUseThing"),
            model.legacy_type(&from, "enabled"),
            model.legacy_command(&from, "VkThing"),
            model.legacy_command(&from, "vkUseThing2"),
            model.legacy_command(&from, "enabled"),
            model.legacy_member(&from, "VkThing", "bits"),
            model.legacy_member(&from, "VkThing2", "enabled"),
            model.legacy_member(&from, "", "VkThing"),
        ] {
            assert_eq!(unmarked, "");
        }
        assert_every_legacy_mark_has_an_item(&model);
    }

    const EXTENSIONS: [&str; 3] = ["VK_KHR_things", "VK_EXT_things", "VK_NV_things"];
    const EXTENSION_LINKS: [&str; 3] = [
        "[`VK_KHR_things`](KHR_THINGS_NAME)",
        "[`VK_EXT_things`](EXT_THINGS_NAME)",
        "[`VK_NV_things`](NV_THINGS_NAME)",
    ];

    fn stated(mark: Legacy) -> String {
        let mut ir = ir();
        let (kind, structure) = (mark.kind, mark.structure.clone().unwrap_or_default());
        let name = mark.name.clone();
        ir.legacy = vec![mark];
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        assert_every_legacy_mark_has_an_item(&model);
        model.legacy(&Cfg::default(), kind, &structure, &name)
    }

    #[test]
    fn an_extension_marks_only_what_is_not_more_promoted_than_the_extension_is() {
        for (kind, name, structure, counted) in [
            (LegacyKind::Type, "VkThing", None, 0),
            (LegacyKind::Type, "VkThingKHR", None, 1),
            (LegacyKind::Type, "VkThingEXT", None, 2),
            (LegacyKind::Type, "VkThingNV", None, 3),
            (LegacyKind::Command, "vkUseThing", None, 0),
            (LegacyKind::Command, "vkUseThingKHR", None, 1),
            (LegacyKind::Feature, "enabled", Some("VkThing"), 0),
            (LegacyKind::Feature, "enabled", Some("VkThingEXT"), 2),
            (LegacyKind::Feature, "enabledNV", Some("VkThingEXT"), 2),
        ] {
            let mark = |by: &[&str], superseded_by: Option<&str>| Legacy {
                structure: structure.map(str::to_string),
                superseded_by: superseded_by.map(str::to_string),
                ..marked_by(kind, name, by)
            };
            let links = EXTENSION_LINKS[..counted].to_vec();
            let listed = |first: Option<&str>| {
                let names = first.into_iter().chain(links.iter().copied());
                join_and(names.map(str::to_string).collect())
            };
            let what = format!("{name} {structure:?}");
            let by_extensions = match counted {
                0 => String::new(),
                _ => as_of(&listed(None), SECTION),
            };
            assert_eq!(stated(mark(&EXTENSIONS, None)), by_extensions, "{what}");
            let reversed = [EXTENSIONS[2], EXTENSIONS[1], EXTENSIONS[0]];
            assert_eq!(
                stated(mark(&reversed, None)).is_empty(),
                counted == 0,
                "{what}"
            );
            let with_a_version = [
                "VK_VERSION_1_1",
                EXTENSIONS[0],
                EXTENSIONS[1],
                EXTENSIONS[2],
            ];
            assert_eq!(
                stated(mark(&with_a_version, None)),
                as_of(&listed(Some("Vulkan 1.1")), SECTION),
                "{what}"
            );
            let superseded = match counted {
                0 => String::new(),
                _ => statement("[`VkThing2`](Thing2)", SECTION),
            };
            assert_eq!(
                stated(mark(&EXTENSIONS, Some("VkThing2"))),
                superseded,
                "{what}"
            );
            assert_eq!(
                stated(mark(&["VK_VERSION_1_0"], Some("VkThing2"))),
                statement("[`VkThing2`](Thing2)", SECTION),
                "{what}"
            );
        }
    }

    #[test]
    fn promotion_is_read_from_the_author_of_an_extension_and_the_vendor_tag_of_a_name() {
        let tags = registry().vendor_tags;
        assert_eq!(tags, ["KHR", "EXT", "NV"]);
        for (name, promotion) in [
            ("VkThing", Promotion::Core),
            ("VkThingKhr", Promotion::Core),
            ("VkKHRThing", Promotion::Core),
            ("VkThingKHR", Promotion::Khr),
            ("vkUseThingEXT", Promotion::Ext),
            ("VkThingNV", Promotion::Vendor),
        ] {
            assert_eq!(Promotion::of_name(&tags, name), promotion, "{name}");
        }
        for (name, promotion) in [
            ("VK_KHR_things", Promotion::Khr),
            ("VK_EXT_things", Promotion::Ext),
            ("VK_NV_things", Promotion::Vendor),
            ("VK_KHX_things", Promotion::Vendor),
            ("VK_NV_things_KHR", Promotion::Vendor),
        ] {
            assert_eq!(Promotion::of_extension(name), promotion, "{name}");
        }
        assert!(Promotion::Core < Promotion::Khr);
        assert!(Promotion::Khr < Promotion::Ext);
        assert!(Promotion::Ext < Promotion::Vendor);
        let ambiguous = ["KHR".to_string(), "HR".to_string()];
        assert_eq!(
            catch_quietly(|| Promotion::of_name(&ambiguous, "VkThingKHR")),
            Err("`VkThingKHR` ends with the two vendor tags `KHR` and `HR`".to_string())
        );
        assert_eq!(
            catch_quietly(|| Promotion::of_name(&ambiguous, "VkThingHR")),
            Ok(Promotion::Vendor)
        );
    }

    #[test]
    fn a_successor_behind_a_feature_is_linked_only_from_an_item_behind_the_same_feature() {
        let mut ir = ir();
        ir.legacy = vec![superseded(LegacyKind::Type, "VkThing", "VkGatedThing")];
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.legacy_type(&Cfg::default(), "VkThing"),
            statement("`VkGatedThing`", SECTION)
        );
        assert_eq!(
            model.legacy_type(&model.cfg(Some(WAYLAND)), "VkThing"),
            statement("[`VkGatedThing`](GatedThing)", SECTION)
        );
    }

    #[test]
    fn an_anchor_is_linked_on_the_page_that_the_specification_publishes_for_its_source_file() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(model.specification_url("legacy-things"), SECTION);
        assert_eq!(
            model.specification_url("things-chapter"),
            "https://docs.vulkan.org/spec/latest/chapters/things.html#things-chapter"
        );
        assert_eq!(
            model.specification_url("things-nested"),
            "https://docs.vulkan.org/spec/latest/chapters/things/more.html#things-nested"
        );
        assert_eq!(
            catch_quietly(|| model.specification_url("things-text")),
            Err(
                "the descriptions name `chapters/things.txt` as the page of the anchor `things-text`, which is not an AsciiDoc source of the specification"
                    .to_string()
            )
        );
        assert_eq!(
            catch_quietly(|| model.specification_url("things-missing")),
            Err(
                "the descriptions name no page of the specification for the anchor `things-missing`"
                    .to_string()
            )
        );
    }

    #[test]
    fn a_legacy_mark_that_the_bindings_cannot_state_aborts_generation() {
        let with_structure = |mut mark: Legacy, structure: Option<&str>| {
            mark.structure = structure.map(str::to_string);
            mark
        };
        let with_link = |mut mark: Legacy, link: &str| {
            mark.link = link.to_string();
            mark
        };
        let no_item = |what: &str| {
            format!(
                "the registry marks {what} as legacy, and the bindings have no item that the generator documents for it"
            )
        };
        for (mark, message) in [
            (
                mark(LegacyKind::Type, "VkMissing"),
                no_item("the type `VkMissing`"),
            ),
            (
                mark(LegacyKind::Type, "vkUseThing"),
                no_item("the type `vkUseThing`"),
            ),
            (
                mark(LegacyKind::Type, "VK_MAX_THINGS"),
                no_item("the type `VK_MAX_THINGS`"),
            ),
            (
                mark(LegacyKind::Command, "vkMissing"),
                no_item("the command `vkMissing`"),
            ),
            (
                mark(LegacyKind::Command, "VkThing"),
                no_item("the command `VkThing`"),
            ),
            (
                feature("VkThing", "missing"),
                no_item("the feature `missing` of `VkThing`"),
            ),
            (
                feature("VkMissing", "enabled"),
                no_item("the feature `enabled` of `VkMissing`"),
            ),
            (
                feature("VkThing2KHR", "enabled"),
                no_item("the feature `enabled` of `VkThing2KHR`"),
            ),
            (
                feature("VkThingUnion", "word"),
                no_item("the feature `word` of `VkThingUnion`"),
            ),
            (
                feature("VkThing", "bits"),
                no_item("the feature `bits` of `VkThing`"),
            ),
            (
                feature("VkThing", "sType"),
                no_item("the feature `sType` of `VkThing`"),
            ),
            (
                feature("VkThing", "pNext"),
                no_item("the feature `pNext` of `VkThing`"),
            ),
            (
                with_structure(mark(LegacyKind::Type, "VkThing"), Some("VkThing2")),
                "the registry data marks the type `VkThing` as legacy in the structure `VkThing2`, which only a feature has".to_string(),
            ),
            (
                with_structure(mark(LegacyKind::Command, "vkUseThing"), Some("VkThing")),
                "the registry data marks the command `vkUseThing` as legacy in the structure `VkThing`, which only a feature has".to_string(),
            ),
            (
                with_structure(feature("VkThing", "enabled"), None),
                "the registry data marks the feature `enabled` as legacy without its structure"
                    .to_string(),
            ),
            (
                superseded(LegacyKind::Type, "VkThing", "VkMissing"),
                "the registry marks the type `VkThing` as legacy and superseded by `VkMissing`, which the bindings have no item for".to_string(),
            ),
            (
                marked_by(
                    LegacyKind::Command,
                    "vkUseThing",
                    &["VK_VERSION_1_1", "VK_KHR_missing"],
                ),
                "`VK_KHR_missing` in the legacy mark of the command `vkUseThing` is neither a version nor an extension of the registry".to_string(),
            ),
            (
                marked_by(LegacyKind::Type, "VkThing", &["VK_BASE_VERSION_1_3"]),
                "`VK_BASE_VERSION_1_3` in the legacy mark of the type `VkThing` is neither a version nor an extension of the registry".to_string(),
            ),
            (
                marked_by(LegacyKind::Type, "VkThing", &["VK_VERSION_1_x"]),
                "`VK_VERSION_1_x` in the legacy mark of the type `VkThing` is not a version name of the form `VK_VERSION_<major>_<minor>` with a major version below 128 and a minor version below 1024".to_string(),
            ),
            (
                marked_by(LegacyKind::Type, "VkThing", &[]),
                "the registry data marks the type `VkThing` as legacy without the version or the extension that does so".to_string(),
            ),
            (
                with_link(feature("VkThing", "enabled"), "things-missing"),
                "the descriptions name no page of the specification for the anchor `things-missing`".to_string(),
            ),
            (
                with_link(mark(LegacyKind::Command, "vkUseThing"), "things-text"),
                "the descriptions name `chapters/things.txt` as the page of the anchor `things-text`, which is not an AsciiDoc source of the specification".to_string(),
            ),
        ] {
            let mut ir = ir();
            ir.constants.push(Constant {
                name: "VK_MAX_THINGS".to_string(),
                ty: "uint32_t".to_string(),
                value: "8".to_string(),
                video_std_header: None,
            });
            ir.legacy = vec![mark];
            let docs = docs();
            assert_eq!(
                catch_quietly(|| crate::generate(&ir, &docs)).map(|_| ()),
                Err(message)
            );
        }
    }

    #[test]
    fn a_name_that_the_registry_data_marks_twice_aborts_generation() {
        let mut ir = ir();
        ir.legacy = vec![
            feature("VkThing", "enabled"),
            mark(LegacyKind::Type, "enabled"),
            mark(LegacyKind::Command, "enabled"),
            feature("VkThing2", "enabled"),
        ];
        let docs = docs();
        assert_eq!(Model::with_docs(&ir, &docs).legacy.len(), 4);
        ir.legacy.push(Legacy {
            superseded_by: Some("VkThing2".to_string()),
            ..feature("VkThing", "enabled")
        });
        assert_eq!(
            catch_quietly(|| Model::with_docs(&ir, &docs).legacy.len()),
            Err(
                "the registry data marks the feature `enabled` of `VkThing` as legacy twice"
                    .to_string()
            )
        );
    }

    fn generated_statements(ir: &Registry, docs: &Docs) -> usize {
        crate::generate(ir, docs)
            .iter()
            .filter(|file| file.path.ends_with(".rs"))
            .flat_map(|file| file.contents.lines())
            .filter(|line| line.contains("Legacy functionality"))
            .count()
    }

    #[test]
    fn a_mark_is_stated_once_on_each_kind_of_item_twice_for_a_command_and_never_on_an_alias() {
        let docs = docs();
        for (kind, name, statements) in [
            (LegacyKind::Type, "VkThing", 1),
            (LegacyKind::Type, "VkThingUnion", 1),
            (LegacyKind::Type, "VkThing2", 1),
            (LegacyKind::Type, "VkThing2KHR", 0),
            (LegacyKind::Type, "VkThingKHR", 1),
            (LegacyKind::Type, "VkDevice", 1),
            (LegacyKind::Type, "VkFormat", 1),
            (LegacyKind::Type, "VkThingFlagBits", 1),
            (LegacyKind::Type, "VkThingFlags", 1),
            (LegacyKind::Type, "PFN_vkThingFunction", 1),
            (LegacyKind::Type, "VK_HEADER_VERSION", 1),
            (LegacyKind::Type, "VK_HEADER_VERSION_COMPLETE", 1),
            (LegacyKind::Type, "VK_API_VERSION_1_0", 1),
            (LegacyKind::Type, "VK_API_VERSION_1_1", 1),
            (LegacyKind::Command, "vkUseThing", 2),
        ] {
            let mut ir = ir();
            ir.legacy = vec![mark(kind, name)];
            assert_eq!(generated_statements(&ir, &docs), statements, "{name}");
        }
        let mut ir = ir();
        ir.legacy = vec![feature("VkThing", "enabled")];
        assert_eq!(generated_statements(&ir, &docs), 2);
        ir.legacy.clear();
        assert_eq!(generated_statements(&ir, &docs), 0);
    }

    #[test]
    fn generation_states_the_marks_and_passes_over_a_legacy_macro_while_the_bindings_leave_it_out()
    {
        let mut ir = ir();
        ir.legacy = vec![
            mark(LegacyKind::Type, "VK_MAKE_VERSION"),
            superseded(LegacyKind::Type, "VkThing", "VkThing2"),
        ];
        let docs = docs();
        assert_eq!(
            generated_statements(&ir, &docs),
            1,
            "only the structure has an item"
        );

        for name in ["VK_MAKE_VERSION", "VK_VERSION_PATCH"] {
            let mut ir = ir.clone();
            ir.legacy = vec![mark(LegacyKind::Type, name)];
            ir.constants.push(Constant {
                name: name.to_string(),
                ty: "uint32_t".to_string(),
                value: "8".to_string(),
                video_std_header: None,
            });
            assert_eq!(
                catch_quietly(|| crate::generate(&ir, &docs)).map(|_| ()),
                Err(format!(
                    "`{name}` is in MACROS_THAT_THE_BINDINGS_LEAVE_OUT, but the bindings have an item for it"
                ))
            );
        }
        let mut ir = ir.clone();
        ir.legacy = vec![mark(LegacyKind::Command, "VK_MAKE_VERSION")];
        assert_eq!(
            catch_quietly(|| crate::generate(&ir, &docs)).map(|_| ()),
            Err("the registry marks the command `VK_MAKE_VERSION` as legacy, and the bindings have no item that the generator documents for it".to_string()),
            "the macro is left out, not a command of the same name"
        );
    }

    #[test]
    fn the_macros_that_the_bindings_leave_out_are_legacy_in_the_registry() {
        assert_listed_legacy_macros_are_marked(Registry::get());
        let mut ir = ir();
        ir.legacy = MACROS_THAT_THE_BINDINGS_LEAVE_OUT
            .iter()
            .map(|name| mark(LegacyKind::Type, name))
            .collect();
        assert_listed_legacy_macros_are_marked(&ir);
        ir.legacy[4].kind = LegacyKind::Command;
        ir.legacy.push(mark(LegacyKind::Type, "VkThing"));
        assert_eq!(
            catch_quietly(|| assert_listed_legacy_macros_are_marked(&ir)),
            Err("`VK_VERSION_PATCH` is in MACROS_THAT_THE_BINDINGS_LEAVE_OUT, but the registry does not mark it as legacy".to_string())
        );
    }

    #[test]
    fn every_legacy_mark_of_the_registry_has_an_item_and_a_section_of_the_specification() {
        let (ir, docs) = (Registry::get(), Docs::get());
        let model = Model::with_docs(ir, docs);
        assert_every_legacy_mark_has_an_item(&model);
        let kinds = [LegacyKind::Type, LegacyKind::Command, LegacyKind::Feature];
        let of_kind = |kind| ir.legacy.iter().filter(move |mark| mark.kind == kind);
        assert_eq!(
            kinds.map(|kind| of_kind(kind).count()),
            [128, 91, 2],
            "the names that the registry marks as legacy"
        );
        assert_eq!(
            kinds.map(|kind| of_kind(kind)
                .map(|mark| mark.marked_by.len())
                .sum::<usize>()),
            [185, 116, 2],
            "the entries of the <deprecate> blocks: several versions and extensions mark the same name"
        );
        let links = ir
            .legacy
            .iter()
            .map(|mark| mark.link.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            links,
            docs.anchors.keys().map(String::as_str).collect(),
            "the descriptions locate exactly the anchors that the registry links to"
        );
        assert_eq!(links.len(), 12);
        for (anchor, file) in &docs.anchors {
            assert_eq!(file, "appendices/legacy.adoc", "{anchor}");
        }
        let left_out = of_kind(LegacyKind::Type)
            .filter(|mark| model.links.get(&mark.name).is_none())
            .map(|mark| mark.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(left_out, MACROS_THAT_THE_BINDINGS_LEAVE_OUT);

        let stated = |mark: &&Legacy| !model.legacy_statement(&Cfg::default(), mark).is_empty();
        assert_eq!(
            kinds.map(|kind| of_kind(kind).filter(stated).count()),
            [83, 61, 2],
            "the marks that the specification shows, with the five macros"
        );
        let is_an_alias = |mark: &&Legacy| model.canonical(&mark.name) != mark.name;
        let aliases = of_kind(LegacyKind::Type).filter(is_an_alias);
        assert_eq!(
            aliases.map(|mark| mark.name.as_str()).collect::<Vec<_>>(),
            [
                "VkPhysicalDeviceDescriptorIndexingFeaturesEXT",
                "VkPhysicalDeviceDescriptorIndexingPropertiesEXT",
                "VkPhysicalDeviceInlineUniformBlockFeaturesEXT",
                "VkPhysicalDeviceInlineUniformBlockPropertiesEXT",
            ],
            "the marked type aliases, which the specification lists without a warning"
        );
        let stated_aliases = of_kind(LegacyKind::Type).filter(is_an_alias).filter(stated);
        assert_eq!(stated_aliases.count(), 0);
        let mut not_stated = BTreeMap::<&[String], usize>::new();
        let no_alias = |mark: &&Legacy| mark.kind != LegacyKind::Type || !is_an_alias(mark);
        let not_aliases = ir.legacy.iter().filter(no_alias);
        for mark in not_aliases.filter(|mark| !stated(mark)) {
            *not_stated.entry(&mark.marked_by).or_default() += 1;
        }
        let only = |extension: &str| vec![extension.to_string()];
        assert_eq!(
            not_stated.into_iter().collect::<Vec<_>>(),
            [
                (only("VK_EXT_descriptor_heap").as_slice(), 49),
                (only("VK_KHR_device_address_commands").as_slice(), 18),
                (only("VK_KHR_extended_flags").as_slice(), 4),
            ],
            "the marks of extensions that are less promoted than what they mark"
        );
    }

    fn dumped_marks(registry: &[&str], supported: &[&str]) -> Option<Result<Vec<Legacy>, String>> {
        let script = [
            "import json, types",
            "from xml.etree import ElementTree",
            "vk = types.SimpleNamespace(extensions=dict.fromkeys(sys.argv[2:]))",
            "legacy = tool.legacy_ir(vk, ElementTree.fromstring(sys.stdin.read()))",
            "lists = ('vendor_tags', 'video_headers', 'versions', 'constants', 'handles', 'enums', 'bitmasks')",
            "lists += ('flags', 'func_pointers', 'structs', 'commands', 'extensions', 'formats')",
            "ir = {name: [] for name in lists}",
            "ir.update(header_version=0, header_version_complete='', headers_tag='', headers_commit='')",
            "ir.update(platforms={}, legacy=legacy)",
            "print(json.dumps(ir))",
        ];
        let json = match run_with_the_tool(&script, supported, &registry.join("\n"))? {
            Ok(json) => json,
            Err(error) => return Some(Err(error)),
        };
        let ir = Registry::from_json(&json).unwrap_or_else(|error| panic!("{error}: {json}"));
        Some(Ok(ir.legacy))
    }

    type Dumped<'a> = (
        LegacyKind,
        &'a str,
        Option<&'a str>,
        &'a str,
        Vec<&'a str>,
        Option<&'a str>,
    );

    fn dumped(mark: &Legacy) -> Dumped<'_> {
        (
            mark.kind,
            mark.name.as_str(),
            mark.structure.as_deref(),
            mark.link.as_str(),
            mark.marked_by.iter().map(String::as_str).collect(),
            mark.superseded_by.as_deref(),
        )
    }

    #[test]
    fn the_dump_has_each_marked_name_once_with_every_version_and_extension_that_marks_it() {
        let registry = [
            "<registry>",
            "  <feature api='vulkan' name='VK_VERSION_1_1'>",
            "    <require><type name='VkNew'/><feature name='old' struct='VkRequired'/></require>",
            "    <deprecate explanationlink='legacy-one'>",
            "      <type name='VkOld' supersededby='VkNew'/>",
            "      <command name='vkOld'/>",
            "    </deprecate>",
            "    <deprecate explanationlink='legacy-two'>",
            "      <type name='vkOld'/>",
            "    </deprecate>",
            "  </feature>",
            "  <feature api='vulkan' name='VK_VERSION_1_2'>",
            "    <deprecate explanationlink='legacy-one'>",
            "      <command name='vkOld'/>",
            "    </deprecate>",
            "    <deprecate explanationlink='legacy-one'>",
            "      <command name='vkOld'/>",
            "    </deprecate>",
            "  </feature>",
            "  <extensions>",
            "    <extension name='VK_KHR_unsupported' supported='vulkan'>",
            "      <deprecate explanationlink='legacy-three'>",
            "        <type name='VkOld'/>",
            "        <type name='VkOther'/>",
            "        <unknown name='VkOther'/>",
            "      </deprecate>",
            "    </extension>",
            "    <extension name='VK_KHR_new' supported='vulkan'>",
            "      <deprecate explanationlink='legacy-one'>",
            "        <feature name='old' struct='VkFeatures'/>",
            "        <feature name='old' struct='VkOld'/>",
            "        <type name='VkOld' supersededby='VkNew'/>",
            "        <command name='vkOld'/>",
            "      </deprecate>",
            "    </extension>",
            "  </extensions>",
            "</registry>",
        ];
        let Some(marks) = dumped_marks(&registry, &["VK_KHR_new", "VK_KHR_other"]) else {
            return;
        };
        let marks = marks.unwrap_or_else(|error| panic!("{error}"));
        assert_eq!(
            marks.iter().map(dumped).collect::<Vec<_>>(),
            [
                (
                    LegacyKind::Command,
                    "vkOld",
                    None,
                    "legacy-one",
                    vec!["VK_VERSION_1_1", "VK_VERSION_1_2", "VK_KHR_new"],
                    None
                ),
                (
                    LegacyKind::Feature,
                    "old",
                    Some("VkFeatures"),
                    "legacy-one",
                    vec!["VK_KHR_new"],
                    None
                ),
                (
                    LegacyKind::Feature,
                    "old",
                    Some("VkOld"),
                    "legacy-one",
                    vec!["VK_KHR_new"],
                    None
                ),
                (
                    LegacyKind::Type,
                    "VkOld",
                    None,
                    "legacy-one",
                    vec!["VK_VERSION_1_1", "VK_KHR_new"],
                    Some("VkNew")
                ),
                (
                    LegacyKind::Type,
                    "vkOld",
                    None,
                    "legacy-two",
                    vec!["VK_VERSION_1_1"],
                    None
                ),
            ]
        );
    }

    #[test]
    fn the_dump_stops_at_a_block_that_it_cannot_read_and_at_a_name_marked_in_two_ways() {
        let old = "<type name='VkOld' supersededby='VkNew'/>";
        for (first, second, message) in [
            (
                ("legacy-one", old),
                ("legacy-one", "<enum name='VK_OLD'/>"),
                "VK_KHR_new: a <deprecate> block lists a <enum> (VK_OLD), which this script does not know",
            ),
            (
                ("legacy-one", old),
                ("legacy-one", "<type name='VkOld' supersededby='VkNewer'/>"),
                "VkOld is marked as legacy by VK_VERSION_1_1 with the superseded_by VkNew and by VK_KHR_new with VkNewer",
            ),
            (
                ("legacy-one", old),
                ("legacy-one", "<type name='VkOld'/>"),
                "VkOld is marked as legacy by VK_VERSION_1_1 with the superseded_by VkNew and by VK_KHR_new with None",
            ),
            (
                ("legacy-one", "<command name='vkOld'/>"),
                ("legacy-one", "<command name='vkOld' supersededby='vkNew'/>"),
                "vkOld is marked as legacy by VK_VERSION_1_1 with the superseded_by None and by VK_KHR_new with vkNew",
            ),
            (
                ("legacy-one", old),
                ("legacy-two", old),
                "VkOld is marked as legacy by VK_VERSION_1_1 with the link legacy-one and by VK_KHR_new with legacy-two",
            ),
            (
                ("legacy-one", old),
                ("", old),
                "VK_KHR_new: a <deprecate> block without an explanationlink",
            ),
            (
                ("legacy-one", old),
                ("legacy-one", "<feature name='old'/>"),
                "VK_KHR_new: a <deprecate> block lists a <feature> with the attributes {'name': 'old'}: it needs a name, and a struct if and only if it is a feature",
            ),
            (
                ("legacy-one", "<type name='VkOld' struct='VkFeatures'/>"),
                ("legacy-one", old),
                "VK_VERSION_1_1: a <deprecate> block lists a <type> with the attributes {'name': 'VkOld', 'struct': 'VkFeatures'}: it needs a name, and a struct if and only if it is a feature",
            ),
            (
                ("legacy-one", old),
                ("legacy-one", "<command supersededby='vkNew'/>"),
                "VK_KHR_new: a <deprecate> block lists a <command> with the attributes {'supersededby': 'vkNew'}: it needs a name, and a struct if and only if it is a feature",
            ),
        ] {
            let block = |(link, marked): (&str, &str)| {
                let link = if link.is_empty() {
                    String::new()
                } else {
                    format!(" explanationlink='{link}'")
                };
                format!("<deprecate{link}>{marked}</deprecate>")
            };
            let registry = format!(
                "<registry><feature name='VK_VERSION_1_1'>{}</feature><extensions><extension name='VK_KHR_new'>{}</extension></extensions></registry>",
                block(first),
                block(second)
            );
            let Some(marks) = dumped_marks(&[&registry], &["VK_KHR_new"]) else {
                return;
            };
            let error = marks.expect_err(&format!("`{registry}` is dumped"));
            assert!(
                error
                    .trim_end()
                    .ends_with(&format!("RuntimeError: {message}")),
                "{error}"
            );
        }
    }

    fn located(linked: &[&str], sources: &[&str]) -> Option<Result<Vec<(String, String)>, String>> {
        let script = [
            "import json",
            "files = {}",
            "for line in sys.stdin.read().splitlines():",
            "    if line.startswith('FILE '):",
            "        lines = files.setdefault(line[5:], [])",
            "    else:",
            "        lines.append(line)",
            "defined = {file: tool.spec_defined_anchors(lines) for file, lines in files.items()}",
            "name = lambda file: file.rsplit('/', 1)[-1]",
            "text = lambda lines: ''.join(line + chr(10) for line in lines)",
            "pages = {file for file, lines in files.items() if tool.spec_is_page(name(file), text(lines))}",
            "anchors = tool.spec_anchor_pages(set(sys.argv[2:]), defined, pages)",
            "docs = {'commit': '', 'anchors': anchors, 'attributes': {}, 'titles': {}, 'pages': [], 'tables': [], 'items_outside_pages': []}",
            "print(json.dumps(docs))",
        ];
        let json = match run_with_the_tool(&script, linked, &sources.join("\n"))? {
            Ok(json) => json,
            Err(error) => return Some(Err(error)),
        };
        let docs = Docs::from_json(&json).unwrap_or_else(|error| panic!("{error}: {json}"));
        Some(Ok(docs.anchors.into_iter().collect()))
    }

    const TWENTY_EIGHT_LINES: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";

    const SOURCES: &[&str] = &[
        "FILE appendices/legacy.adoc",
        "// Copyright",
        "",
        "[appendix]",
        "[[legacy-appendix]]",
        "= Legacy Functionality",
        "[[legacy-one]]",
        "=== One",
        "=== [[legacy-two]]Two",
        "// [[legacy-commented]]",
        "See <<legacy-referenced>> and [[legacy-inline]]this.",
        "[[legacy-twice]]",
        "FILE chapters/things/more.adoc",
        "# Things",
        "  * [[VUID-vkThing-00001]] [[legacy-three]]",
        "[[legacy-twice]]",
        "FILE appendices/VK_KHR_partial.adoc",
        "include::{generated}/meta/VK_KHR_partial.adoc[]",
        "",
        "=== Other Extension Metadata",
        "",
        "= A Title After the First One",
        "[[partial-subtitle]]",
        "FILE chapters/untitled.adoc",
        "[[partial-untitled]]",
        "FILE chapters/lowercase.adoc",
        "= lowercase is not a title here",
        "[[partial-lowercase]]",
        "FILE chapters/_private.adoc",
        "= Title",
        "[[partial-name]]",
        "FILE chapters/vulkanscdeviations.adoc",
        "= Title",
        "[[partial-deviations]]",
        "FILE chapters/late.adoc",
        TWENTY_EIGHT_LINES,
        "",
        "",
        "= Title on Line 31",
        "[[partial-late]]",
        "FILE chapters/in_time.adoc",
        TWENTY_EIGHT_LINES,
        "[[in-time]]",
        "= Title on Line 30",
    ];

    #[test]
    fn the_dump_names_the_one_source_file_that_defines_each_linked_anchor() {
        let linked = [
            "legacy-three",
            "legacy-two",
            "legacy-one",
            "legacy-inline",
            "in-time",
        ];
        let Some(located) = located(&linked, SOURCES) else {
            return;
        };
        let located = located.unwrap_or_else(|error| panic!("{error}"));
        let expected = [
            ("in-time", "chapters/in_time.adoc"),
            ("legacy-inline", "appendices/legacy.adoc"),
            ("legacy-one", "appendices/legacy.adoc"),
            ("legacy-three", "chapters/things/more.adoc"),
            ("legacy-two", "appendices/legacy.adoc"),
        ];
        assert_eq!(
            located,
            expected.map(|(anchor, file)| (anchor.to_string(), file.to_string()))
        );
    }

    #[test]
    fn the_dump_stops_at_a_linked_anchor_that_no_source_file_or_more_than_one_defines() {
        for (anchor, defined_by) in [
            ("legacy-missing", "no source file"),
            ("legacy-commented", "no source file"),
            ("legacy-referenced", "no source file"),
            ("legacy", "no source file"),
            (
                "legacy-twice",
                "appendices/legacy.adoc and chapters/things/more.adoc",
            ),
        ] {
            let Some(located) = located(&["legacy-one", anchor], SOURCES) else {
                return;
            };
            let error = located.expect_err(&format!("`{anchor}` is located"));
            let message = format!(
                "RuntimeError: the registry links to the anchor `{anchor}` of the specification, which {defined_by} defines: exactly one source file has to define it"
            );
            assert!(error.trim_end().ends_with(&message), "{error}");
        }
    }

    #[test]
    fn the_dump_stops_at_a_linked_anchor_in_a_file_that_is_not_published_as_a_page() {
        for (anchor, file) in [
            ("partial-subtitle", "appendices/VK_KHR_partial.adoc"),
            ("partial-untitled", "chapters/untitled.adoc"),
            ("partial-lowercase", "chapters/lowercase.adoc"),
            ("partial-name", "chapters/_private.adoc"),
            ("partial-deviations", "chapters/vulkanscdeviations.adoc"),
            ("partial-late", "chapters/late.adoc"),
        ] {
            let Some(located) = located(&["legacy-one", anchor], SOURCES) else {
                return;
            };
            let error = located.expect_err(&format!("`{anchor}` is located"));
            let message = format!(
                "RuntimeError: the registry links to the anchor `{anchor}` of the specification, which {file} defines, and that file is not published as a page of its own: it has no top-level title in its first 30 lines, or not the name of a page"
            );
            assert!(error.trim_end().ends_with(&message), "{error}");
        }
    }
}
