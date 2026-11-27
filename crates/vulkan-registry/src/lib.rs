use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

const VULKAN_IR_JSON: &str = include_str!("../vulkan-ir.json");
const VULKAN_DOCS_JSON: &str = include_str!("../vulkan-docs.json");

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Registry {
    pub header_version: u32,
    pub header_version_complete: String,
    pub headers_tag: String,
    pub headers_commit: String,
    pub vendor_tags: Vec<String>,
    pub platforms: BTreeMap<String, String>,
    pub video_headers: Vec<VideoHeader>,
    pub versions: Vec<Version>,
    pub constants: Vec<Constant>,
    pub handles: Vec<Handle>,
    pub enums: Vec<Enum>,
    pub bitmasks: Vec<Bitmask>,
    pub flags: Vec<Flags>,
    pub func_pointers: Vec<FuncPointer>,
    pub structs: Vec<Struct>,
    pub commands: Vec<Command>,
    pub extensions: Vec<Extension>,
    pub legacy: Vec<Legacy>,
    pub legacy_attributes: Vec<LegacyAttribute>,
    pub formats: Vec<Format>,
}

impl Registry {
    pub fn get() -> &'static Self {
        static REGISTRY: OnceLock<Registry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            Self::from_json(VULKAN_IR_JSON).expect("embedded Vulkan registry is valid")
        })
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Docs {
    pub commit: String,
    pub anchors: BTreeMap<String, String>,
    pub attributes: BTreeMap<String, String>,
    pub titles: BTreeMap<String, String>,
    pub pages: Vec<DocPage>,
    pub tables: Vec<DocTable>,
    pub items_outside_pages: Vec<DocItemOutsidePages>,
}

impl Docs {
    pub fn get() -> &'static Self {
        static DOCS: OnceLock<Docs> = OnceLock::new();
        DOCS.get_or_init(|| {
            Self::from_json(VULKAN_DOCS_JSON).expect("embedded Vulkan descriptions are valid")
        })
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DocPage {
    pub name: String,
    pub kind: String,
    pub desc: String,
    pub aliases: Vec<String>,
    pub file: String,
    pub items: Vec<DocItem>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DocItem {
    pub name: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DocTable {
    pub file: String,
    pub page: Option<String>,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct DocItemOutsidePages {
    pub file: String,
    pub name: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct VideoHeader {
    pub name: String,
    pub header_file: String,
    pub depends: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Version {
    pub name: String,
    pub name_api: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Constant {
    pub name: String,
    pub ty: String,
    pub value: String,
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Handle {
    pub name: String,
    pub aliases: Vec<String>,
    pub object_type: String,
    pub parent: Option<String>,
    pub dispatchable: bool,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Enum {
    pub name: String,
    pub aliases: Vec<String>,
    pub bit_width: u32,
    pub fields: Vec<EnumField>,
    pub protect: Option<String>,
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct EnumField {
    pub name: String,
    pub aliases: Vec<String>,
    pub value: String,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Bitmask {
    pub name: String,
    pub aliases: Vec<String>,
    pub flags_name: String,
    pub bit_width: u32,
    pub fields: Vec<EnumField>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Flags {
    pub name: String,
    pub aliases: Vec<String>,
    pub bitmask: Option<String>,
    pub bit_width: u32,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FuncPointer {
    pub name: String,
    pub return_ty: String,
    pub params: Vec<FuncPointerParam>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FuncPointerParam {
    pub name: String,
    pub full_ty: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Struct {
    pub name: String,
    pub aliases: Vec<String>,
    pub union: bool,
    pub s_type: Option<String>,
    pub extends: Vec<String>,
    pub members: Vec<Field>,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
    pub version: Option<String>,
    pub video_std_header: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Field {
    pub name: String,
    pub ty: String,
    pub full_ty: String,
    pub optional: bool,
    pub optional_pointer: bool,
    pub len: Option<String>,
    pub null_terminated: bool,
    pub fixed_size_array: Vec<String>,
    #[serde(default)]
    pub bit_field_width: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Command {
    pub name: String,
    pub alias: Option<String>,
    pub return_ty: String,
    pub success_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub instance: bool,
    pub device: bool,
    pub params: Vec<Field>,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Extension {
    pub name: String,
    pub name_macro: String,
    pub spec_version_macro: String,
    pub spec_version: u32,
    pub instance: bool,
    pub device: bool,
    pub protect: Option<String>,
    pub depends: Option<String>,
    pub promoted_to: Option<String>,
    pub deprecated_by: Option<String>,
    pub obsoleted_by: Option<String>,
    pub provisional: bool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LegacyKind {
    #[default]
    Type,
    Command,
    Feature,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Legacy {
    pub kind: LegacyKind,
    pub name: String,
    pub structure: Option<String>,
    pub link: String,
    pub marked_by: Vec<String>,
    pub superseded_by: Option<String>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LegacyElement {
    #[default]
    Enum,
    Member,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LegacyReason {
    #[default]
    Aliased,
    True,
    Unused,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct LegacyAttribute {
    pub element: LegacyElement,
    pub name: String,
    pub owner: Option<String>,
    pub reason: LegacyReason,
    pub alias: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Format {
    pub name: String,
    pub class: String,
    pub block_size: u32,
    pub texels_per_block: u32,
    pub block_extent: Vec<u32>,
    pub packed: Option<u32>,
    pub chroma: Option<String>,
    pub compressed: Option<String>,
    pub components: Vec<FormatComponent>,
    pub planes: Vec<FormatPlane>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FormatComponent {
    pub name: String,
    pub bits: Option<u32>,
    pub numeric_format: String,
    pub plane_index: Option<u32>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FormatPlane {
    pub index: u32,
    pub width_divisor: u32,
    pub height_divisor: u32,
    pub compatible: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_parses_and_is_from_an_sdk_tag() {
        let registry = Registry::get();
        let prefix = format!("vulkan-sdk-{}.", registry.header_version_complete);
        assert!(
            registry.headers_tag.starts_with(&prefix),
            "{} does not start with {prefix}",
            registry.headers_tag
        );
        assert!(!registry.structs.is_empty());
        assert!(!registry.commands.is_empty());
    }

    #[test]
    fn embedded_legacy_marks_have_a_kind_a_link_and_what_marks_them() {
        let registry = Registry::get();
        for kind in [LegacyKind::Type, LegacyKind::Command, LegacyKind::Feature] {
            let mut marks = registry.legacy.iter().filter(|mark| mark.kind == kind);
            assert!(
                marks.all(|mark| mark.structure.is_some() == (kind == LegacyKind::Feature)),
                "{kind:?}"
            );
            assert!(registry.legacy.iter().any(|mark| mark.kind == kind));
        }
        for mark in &registry.legacy {
            assert!(!mark.name.is_empty() && !mark.link.is_empty());
            assert!(!mark.marked_by.is_empty(), "{}", mark.name);
        }
        let submit = registry
            .legacy
            .iter()
            .find(|mark| mark.name == "vkQueueSubmit")
            .expect("vkQueueSubmit is legacy");
        assert_eq!(submit.kind, LegacyKind::Command);
        assert_eq!(submit.link, "deprecation-sync2");
        assert_eq!(
            submit.marked_by,
            ["VK_VERSION_1_3", "VK_KHR_synchronization2"]
        );
        assert_eq!(submit.superseded_by.as_deref(), Some("vkQueueSubmit2"));
        let docs = Docs::get();
        assert_eq!(
            docs.anchors.get("deprecation-sync2").map(String::as_str),
            Some("appendices/legacy.adoc")
        );
    }

    #[test]
    fn embedded_descriptions_parse_and_cover_the_registry() {
        let docs = Docs::get();
        let registry = Registry::get();
        assert!(!docs.commit.is_empty());
        let page = |name: &str| docs.pages.iter().find(|page| page.name == name);
        let create_info = page("VkDeviceQueueCreateInfo").expect("a page for a core struct");
        assert_eq!(create_info.kind, "structs");
        assert_eq!(
            create_info.desc,
            "Structure specifying parameters of a newly created device queue"
        );
        assert!(
            create_info
                .items
                .iter()
                .any(|item| item.name == "queueFamilyIndex"
                    && item.text.starts_with("pname:queueFamilyIndex is "))
        );
        let described_handles = registry
            .handles
            .iter()
            .filter(|handle| page(&handle.name).is_some())
            .count();
        assert_eq!(described_handles, registry.handles.len());
    }

    #[test]
    fn crate_version_metadata_matches_headers_tag() {
        let version = env!("CARGO_PKG_VERSION");
        let (_, metadata) = version
            .split_once('+')
            .expect("crate version carries the Vulkan-Headers tag as build metadata");
        assert_eq!(metadata, Registry::get().headers_tag);
    }
}
