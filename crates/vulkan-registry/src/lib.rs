//! Vulkan registry data dumped from Khronos Vulkan-Headers.
//!
//! `vulkan-ir.json` is written by `tools/generate-vulkan` from Khronos' `VulkanObject`
//! (the registry abstraction shipped in `Vulkan-Headers/registry`). It keeps the C names
//! and registry attributes as they are; every Rust-side naming or shaping decision is made
//! in `vulkan-codegen`.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::sync::OnceLock;

const VULKAN_IR_JSON: &str = include_str!("../vulkan-ir.json");

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Registry {
    pub header_version: u32,
    pub header_version_complete: String,
    pub headers_tag: String,
    pub headers_commit: String,
    pub vendor_tags: Vec<String>,
    /// Platform name to protect macro, e.g. `wayland` to `VK_USE_PLATFORM_WAYLAND_KHR`.
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
}

impl Registry {
    /// The registry embedded in this crate.
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
pub struct VideoHeader {
    pub name: String,
    pub header_file: String,
    pub depends: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Version {
    /// e.g. `VK_VERSION_1_1`
    pub name: String,
    /// e.g. `VK_API_VERSION_1_1`
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
    /// e.g. `VK_OBJECT_TYPE_BUFFER`
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
    /// The `FlagBits` type, e.g. `VkImageUsageFlagBits`.
    pub name: String,
    pub aliases: Vec<String>,
    /// The matching `Flags` type, e.g. `VkImageUsageFlags`.
    pub flags_name: String,
    pub bit_width: u32,
    pub fields: Vec<FlagField>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct FlagField {
    pub name: String,
    pub aliases: Vec<String>,
    pub value: String,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Flags {
    pub name: String,
    pub aliases: Vec<String>,
    /// The `FlagBits` type that defines the bits, if any bit is defined.
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
    pub ty: String,
    pub full_ty: String,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Struct {
    pub name: String,
    pub aliases: Vec<String>,
    pub union: bool,
    pub s_type: Option<String>,
    /// Structs whose `pNext` chain may contain this struct.
    pub extends: Vec<String>,
    pub members: Vec<Field>,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
    pub version: Option<String>,
    pub video_std_header: Option<String>,
}

/// A struct member or a command parameter.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Field {
    pub name: String,
    /// Base type without `const` or pointers, e.g. `VkBuffer`.
    pub ty: String,
    /// Type as written in C without the array part, e.g. `const char* const*`.
    pub full_ty: String,
    pub is_const: bool,
    pub pointer: bool,
    /// The value itself (for a pointer: the pointer) may be zero / null.
    pub optional: bool,
    /// For a pointer: the pointed-to value may be zero / null.
    pub optional_pointer: bool,
    pub len: Option<String>,
    pub null_terminated: bool,
    pub fixed_size_array: Vec<String>,
    /// Width of a C bit-field member. Always `None` for command parameters.
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
    /// Instance-level command according to the registry.
    pub instance: bool,
    /// Device-level command according to the registry.
    pub device: bool,
    pub params: Vec<Field>,
    pub protect: Option<String>,
    pub extensions: Vec<String>,
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Extension {
    pub name: String,
    /// e.g. `VK_KHR_SWAPCHAIN_EXTENSION_NAME`
    pub name_macro: String,
    /// e.g. `VK_KHR_SWAPCHAIN_SPEC_VERSION`
    pub spec_version_macro: String,
    pub spec_version: u32,
    pub instance: bool,
    pub device: bool,
    pub protect: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_parses_and_is_from_an_sdk_tag() {
        let registry = Registry::get();
        // SDK tags look like `vulkan-sdk-1.4.363.0`: the header version plus an SDK revision.
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
    fn crate_version_metadata_matches_headers_tag() {
        let version = env!("CARGO_PKG_VERSION");
        let (_, metadata) = version
            .split_once('+')
            .expect("crate version carries the Vulkan-Headers tag as build metadata");
        assert_eq!(metadata, Registry::get().headers_tag);
    }
}
