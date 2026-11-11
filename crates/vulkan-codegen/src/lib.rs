//! Code generator for the `vulkan` crate.
//!
//! Input: the registry dumped by `tools/generate-vulkan` (see `vulkan-registry`).
//! Output: the files listed by [`generate`], relative to the workspace root.
//!
//! - [`naming`]: C identifier to Rust identifier mapping.
//! - [`model`]: registry analysis (type lookup, lifetimes, dispatch levels).
//! - [`emit`]: one emitter per generated file.

pub mod emit;
pub mod model;
pub mod naming;

#[cfg(test)]
mod test_support;

use model::Model;
use vulkan_registry::Registry;

/// Directory of the generated modules of the `vulkan` crate.
pub const GENERATED_SRC: &str = "crates/vulkan/src/vk/generated";
/// Directory of the generated ABI cross-check inputs.
pub const GENERATED_TESTS: &str = "crates/vulkan/tests/generated";

/// Generates every file as `(path relative to the workspace root, contents)`.
///
/// Rust output is not formatted; run rustfmt on it.
pub fn generate(ir: &Registry) -> Vec<(String, String)> {
    let model = Model::new(ir);
    let src = |name: &str, body: String| (format!("{GENERATED_SRC}/{name}"), body);
    let test = |name: &str, body: String| (format!("{GENERATED_TESTS}/{name}"), body);
    vec![
        src("mod.rs", emit::basic::emit_mod()),
        src("version.rs", emit::basic::emit_version(&model)),
        src("constants.rs", emit::basic::emit_constants(&model)),
        src("handles.rs", emit::basic::emit_handles(&model)),
        src("enums.rs", emit::basic::emit_enums(&model)),
        src("bitmasks.rs", emit::basic::emit_bitmasks(&model)),
        src("func_pointers.rs", emit::basic::emit_func_pointers(&model)),
        src(
            "extensions.rs",
            emit::basic::emit_extensions(&model) + &emit::tables::emit_extension_table(&model),
        ),
        src("formats.rs", emit::tables::emit_formats(&model)),
        src("structs.rs", emit::structs::emit_structs(&model)),
        src("commands.rs", emit::commands::emit_commands(&model)),
        test("layout_table.rs", emit::layout::emit_layout_table(&model)),
        test("layout.c", emit::layout::emit_layout_c(&model)),
    ]
}

/// Cargo features of the `vulkan` crate that gate generated items.
pub fn required_features(ir: &Registry) -> Vec<String> {
    Model::new(ir).features()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The embedded registry must always generate: this is the check that a
    /// Vulkan-Headers update did not introduce a shape the generator rejects.
    #[test]
    fn embedded_registry_generates() {
        let files = generate(Registry::get());
        assert_eq!(files.len(), 13);
        for (path, body) in &files {
            assert!(!body.is_empty(), "{path} is empty");
        }
    }

    #[test]
    fn generation_is_deterministic() {
        assert_eq!(generate(Registry::get()), generate(Registry::get()));
    }
}
