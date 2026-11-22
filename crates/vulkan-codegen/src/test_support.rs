use proptest::test_runner::Config as ProptestConfig;
use std::cell::Cell;
use std::sync::Once;
use vulkan_registry::{Command, Enum, Field, Registry, Struct};

pub(crate) const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen", "_",
];

pub(crate) fn is_rust_identifier(name: &str) -> bool {
    let unraw = name.strip_prefix("r#");
    let reserved = match unraw {
        Some(plain) => ["self", "Self", "super", "crate", "_"].contains(&plain),
        None => RUST_KEYWORDS.contains(&name),
    };
    !reserved
        && syn::parse_str::<syn::Ident>(name).is_ok()
        && syn::parse_str::<syn::ItemFn>(&format!("fn f({name}: u32) {{}}")).is_ok()
}

pub(crate) fn proptest_config(default_cases: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default_cases);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

thread_local! {
    static QUIET_PANICS: Cell<bool> = const { Cell::new(false) };
}

pub(crate) fn catch_quietly<T>(f: impl FnOnce() -> T) -> Result<T, String> {
    static HOOK: Once = Once::new();
    HOOK.call_once(|| {
        let default = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            if !QUIET_PANICS.with(Cell::get) {
                default(info);
            }
        }));
    });
    QUIET_PANICS.with(|quiet| quiet.set(true));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    QUIET_PANICS.with(|quiet| quiet.set(false));
    result.map_err(|payload| {
        payload
            .downcast_ref::<String>()
            .cloned()
            .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
            .unwrap_or_default()
    })
}

pub(crate) fn registry() -> Registry {
    Registry {
        header_version: 1,
        header_version_complete: "1.4.1".to_string(),
        headers_tag: "vulkan-sdk-1.4.1.0".to_string(),
        headers_commit: "000000000000".to_string(),
        vendor_tags: ["KHR", "EXT", "NV"].iter().map(|s| s.to_string()).collect(),
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

pub(crate) fn field(name: &str, full_ty: &str) -> Field {
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
        ..Default::default()
    }
}

pub(crate) fn counted(name: &str, full_ty: &str, len: &str) -> Field {
    Field {
        len: Some(len.to_string()),
        ..field(name, full_ty)
    }
}

pub(crate) fn optional(mut field: Field) -> Field {
    field.optional = true;
    field
}

pub(crate) fn structure(name: &str, members: Vec<Field>) -> Struct {
    Struct {
        name: name.to_string(),
        members,
        ..Default::default()
    }
}

pub(crate) fn tagged(name: &str, s_type: &str, mut members: Vec<Field>) -> Struct {
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

pub(crate) fn command(name: &str, params: Vec<Field>) -> Command {
    Command {
        name: name.to_string(),
        return_ty: "void".to_string(),
        params,
        ..Default::default()
    }
}

pub(crate) fn result_command(name: &str, success_codes: &[&str], params: Vec<Field>) -> Command {
    Command {
        return_ty: "VkResult".to_string(),
        success_codes: success_codes.iter().map(|s| s.to_string()).collect(),
        ..command(name, params)
    }
}
