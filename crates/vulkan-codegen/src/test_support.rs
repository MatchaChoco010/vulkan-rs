//! Builders for small synthetic registries used by unit tests.

use vulkan_registry::{Command, Enum, Field, Registry, Struct};

pub fn registry() -> Registry {
    Registry {
        header_version: 1,
        header_version_complete: "1.4.1".to_string(),
        headers_tag: "vulkan-sdk-1.4.1.0".to_string(),
        headers_commit: "000000000000".to_string(),
        vendor_tags: ["KHR", "EXT", "NV"].iter().map(|s| s.to_string()).collect(),
        // Enums that the emitters refer to by name.
        enums: ["VkStructureType", "VkResult", "VkObjectType"]
            .iter()
            .map(|name| Enum {
                name: name.to_string(),
                bit_width: 32,
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    }
}

/// A field from a C declaration type such as `const VkFoo*`.
pub fn field(name: &str, full_ty: &str) -> Field {
    let ty = full_ty
        .replace('*', " ")
        .split_whitespace()
        .find(|token| *token != "const" && *token != "struct")
        .expect("base type")
        .to_string();
    Field {
        name: name.to_string(),
        ty,
        full_ty: full_ty.to_string(),
        is_const: full_ty.contains("const"),
        pointer: full_ty.contains('*'),
        ..Default::default()
    }
}

pub fn counted(name: &str, full_ty: &str, len: &str) -> Field {
    Field {
        len: Some(len.to_string()),
        ..field(name, full_ty)
    }
}

pub fn optional(mut field: Field) -> Field {
    field.optional = true;
    field
}

pub fn structure(name: &str, members: Vec<Field>) -> Struct {
    Struct {
        name: name.to_string(),
        members,
        ..Default::default()
    }
}

/// A struct that starts with `sType` and `pNext`.
pub fn tagged(name: &str, s_type: &str, mut members: Vec<Field>) -> Struct {
    let mut all = vec![
        field("sType", "VkStructureType"),
        optional(field("pNext", "const void*")),
    ];
    all.append(&mut members);
    Struct {
        s_type: Some(s_type.to_string()),
        ..structure(name, all)
    }
}

pub fn command(name: &str, params: Vec<Field>) -> Command {
    Command {
        name: name.to_string(),
        return_ty: "void".to_string(),
        params,
        ..Default::default()
    }
}

pub fn result_command(name: &str, success_codes: &[&str], params: Vec<Field>) -> Command {
    Command {
        return_ty: "VkResult".to_string(),
        success_codes: success_codes.iter().map(|s| s.to_string()).collect(),
        ..command(name, params)
    }
}
