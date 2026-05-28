use serde::Deserialize;
use std::sync::OnceLock;

const VULKAN_IR_JSON: &str = include_str!("../vulkan-ir.json");

#[derive(Clone, Debug, Deserialize)]
pub struct Registry {
    pub header_version: u32,
    pub header_version_complete: String,
    pub headers_tag: String,
    pub headers_commit: String,
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
    pub fn get() -> &'static Self {
        static REGISTRY: OnceLock<Registry> = OnceLock::new();
        REGISTRY.get_or_init(|| {
            serde_json::from_str(VULKAN_IR_JSON).expect("embedded Vulkan registry is valid")
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Constant {
    pub name: String,
    pub rust_name: String,
    pub value: String,
    pub ty: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Handle {
    pub name: String,
    pub rust_name: String,
    pub aliases: Vec<String>,
    pub dispatchable: bool,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Enum {
    pub name: String,
    pub rust_name: String,
    pub aliases: Vec<String>,
    pub bit_width: u32,
    pub fields: Vec<EnumField>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EnumField {
    pub name: String,
    pub rust_name: String,
    pub value: String,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bitmask {
    pub name: String,
    pub rust_name: String,
    pub aliases: Vec<String>,
    pub bit_width: u32,
    pub fields: Vec<FlagField>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FlagField {
    pub name: String,
    pub rust_name: String,
    pub value: String,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Flags {
    pub name: String,
    pub rust_name: String,
    pub aliases: Vec<String>,
    pub bitmask: Option<String>,
    pub bit_width: u32,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FuncPointer {
    pub name: String,
    pub return_ty: String,
    pub params: Vec<Param>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Struct {
    pub name: String,
    pub rust_name: String,
    pub aliases: Vec<String>,
    pub union: bool,
    pub s_type: Option<String>,
    pub extends: Vec<String>,
    pub members: Vec<Member>,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Member {
    pub name: String,
    pub rust_name: String,
    pub ty: String,
    pub full_ty: String,
    pub is_const: bool,
    pub pointer: bool,
    pub optional: bool,
    pub optional_pointer: bool,
    pub len: Option<String>,
    pub null_terminated: bool,
    pub fixed_size_array: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Command {
    pub name: String,
    pub rust_name: String,
    pub return_ty: String,
    pub success_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub params: Vec<Param>,
    pub dispatch: String,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Extension {
    pub name: String,
    pub name_const: String,
    pub spec_version_const: String,
    pub spec_version: u32,
    pub protect: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Param {
    pub name: String,
    pub rust_name: String,
    pub ty: String,
    pub full_ty: String,
    pub is_const: bool,
    pub pointer: bool,
    pub optional: bool,
    pub optional_pointer: bool,
    pub len: Option<String>,
    pub null_terminated: bool,
    pub fixed_size_array: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_registry_is_embedded_vulkan_headers_data() {
        let registry = Registry::get();
        assert_eq!(registry.headers_tag, "vulkan-sdk-1.4.350.0");
        assert_eq!(registry.header_version_complete, "1.4.350");
        assert!(!registry.structs.is_empty());
        assert!(!registry.commands.is_empty());
    }
}
