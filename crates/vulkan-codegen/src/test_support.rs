use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::{Config as ProptestConfig, RngAlgorithm, TestRng, TestRunner};
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

pub(crate) fn deterministic_values<S: Strategy>(strategy: S, count: usize) -> Vec<S::Value> {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    (0..count)
        .map(|_| {
            strategy
                .new_tree(&mut runner)
                .expect("the strategy generates a value")
                .current()
        })
        .collect()
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

pub(crate) fn chapters_of_the_specification() -> Option<String> {
    let chapters = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../third_party/vulkan-docs/chapters");
    if !chapters.is_dir() {
        assert!(
            std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
            "a test that reads the specification is required but third_party/vulkan-docs is not checked out"
        );
        eprintln!(
            "skipping a test that reads the specification: third_party/vulkan-docs is not checked out"
        );
        return None;
    }
    let mut text = String::new();
    let mut directories = vec![chapters];
    while let Some(directory) = directories.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
            .map(|entry| entry.expect("a directory entry").path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                directories.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "adoc")
            {
                let chapter = std::fs::read_to_string(&path)
                    .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
                text.push_str(&chapter);
                text.push('\n');
            }
        }
    }
    Some(text)
}

pub(crate) fn description_in<'c>(chapters: &'c str, name: &str) -> &'c str {
    let Some(at) = chapters.find(&format!("refpage='{name}'")) else {
        return "";
    };
    let opened = &chapters[at..];
    let start = opened
        .find("\n--\n")
        .map_or(opened.len(), |start| start + 4);
    let body = &opened[start..];
    &body[..body.find("\n--\n").unwrap_or(body.len())]
}
