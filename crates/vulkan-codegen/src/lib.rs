mod docs;
mod emit;
mod model;
mod naming;

#[cfg(test)]
mod test_support;

use model::Model;
use vulkan_registry::{Docs, Registry};

pub const GENERATED_SRC: &str = "crates/vulkan/src/vk/generated";
pub const GENERATED_TESTS: &str = "crates/vulkan/tests/generated";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedFile {
    pub path: String,
    pub contents: String,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Items {
    Reexported,
    OnlyImpls,
}

struct BindingsModule {
    name: &'static str,
    items: Items,
    emit: fn(&Model<'_>) -> String,
}

const BINDINGS_MODULES: &[BindingsModule] = &[
    BindingsModule {
        name: "bitmasks",
        items: Items::Reexported,
        emit: emit::bitmasks::emit,
    },
    BindingsModule {
        name: "command_info",
        items: Items::Reexported,
        emit: emit::command_info::emit,
    },
    BindingsModule {
        name: "commands",
        items: Items::Reexported,
        emit: emit::commands::emit,
    },
    BindingsModule {
        name: "constants",
        items: Items::Reexported,
        emit: emit::constants::emit,
    },
    BindingsModule {
        name: "enums",
        items: Items::Reexported,
        emit: emit::enums::emit,
    },
    BindingsModule {
        name: "extensions",
        items: Items::Reexported,
        emit: emit::extensions::emit,
    },
    BindingsModule {
        name: "formats",
        items: Items::OnlyImpls,
        emit: emit::formats::emit,
    },
    BindingsModule {
        name: "func_pointers",
        items: Items::Reexported,
        emit: emit::func_pointers::emit,
    },
    BindingsModule {
        name: "handles",
        items: Items::Reexported,
        emit: emit::handles::emit,
    },
    BindingsModule {
        name: "structs",
        items: Items::Reexported,
        emit: emit::structs::emit,
    },
    BindingsModule {
        name: "version",
        items: Items::Reexported,
        emit: emit::version::emit,
    },
];

fn bindings_root() -> String {
    let declarations = BINDINGS_MODULES
        .iter()
        .map(|module| format!("mod {};\n", module.name));
    let reexports = BINDINGS_MODULES
        .iter()
        .filter(|module| module.items == Items::Reexported)
        .map(|module| format!("pub use {}::*;\n", module.name));
    declarations
        .chain(std::iter::once("\n".to_string()))
        .chain(reexports)
        .collect()
}

pub fn assert_aligned_memory_exists(ir: &Registry) {
    model::assert_listed_aligned_memory_exists(ir);
}

pub fn generate(ir: &Registry, docs: &Docs) -> Vec<GeneratedFile> {
    let model = Model::with_docs(ir, docs);
    if !docs.pages.is_empty() {
        docs::assert_every_other_name_is_in_use(&model);
    }
    let file = |directory: &str, name: &str, contents: String| GeneratedFile {
        path: format!("{directory}/{name}"),
        contents,
    };
    let rust = |directory: &str, name: &str, source: String| {
        file(directory, name, emit::indent_by_nesting(&source))
    };
    let mut files = vec![rust(GENERATED_SRC, "mod.rs", bindings_root())];
    files.extend(BINDINGS_MODULES.iter().map(|module| {
        rust(
            GENERATED_SRC,
            &format!("{}.rs", module.name),
            (module.emit)(&model),
        )
    }));
    files.push(rust(
        GENERATED_TESTS,
        "layout_table.rs",
        emit::layout::emit_table(&model),
    ));
    files.push(file(
        GENERATED_TESTS,
        "layout.c",
        emit::layout::emit_c_program(&model),
    ));
    files
}

pub fn required_features(ir: &Registry) -> Vec<String> {
    Model::new(ir).features()
}

pub fn assert_listed_members_exist(ir: &Registry) {
    emit::structs::assert_listed_structures_exist(ir);
    emit::structs::assert_listed_read_only_members_exist(ir);
    emit::commands::assert_listed_uncounted_arrays_exist(ir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generation_is_deterministic() {
        assert_eq!(
            generate(Registry::get(), Docs::get()),
            generate(Registry::get(), Docs::get())
        );
    }

    #[test]
    fn keyword_named_members_parameters_and_commands_generate_rust_that_parses() {
        use crate::test_support::{RUST_KEYWORDS, command, field, registry, structure};
        use std::collections::BTreeSet;
        use vulkan_registry::{Enum, Handle};

        let mut ir = registry();
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
        let mut command_names = BTreeSet::new();
        for (i, keyword) in RUST_KEYWORDS.iter().enumerate() {
            ir.structs.push(structure(
                &format!("VkKeywordMember{i}"),
                vec![field(keyword, "uint32_t")],
            ));
            let mut c = command(
                &format!("vkKeywordParameter{i}"),
                vec![field("device", "VkDevice"), field(keyword, "uint32_t")],
            );
            c.device = true;
            ir.commands.push(c);
            let mut chars = keyword.chars();
            let first = chars.next().expect("keywords are not empty");
            command_names.insert(format!(
                "vk{}{}",
                first.to_ascii_uppercase(),
                chars.as_str()
            ));
        }
        for name in command_names {
            let mut c = command(&name, vec![field("device", "VkDevice")]);
            c.device = true;
            ir.commands.push(c);
        }
        for file in generate(&ir, &Docs::default()) {
            if file.path.ends_with(".rs") {
                syn::parse_file(&file.contents)
                    .unwrap_or_else(|error| panic!("{} does not parse: {error}", file.path));
            }
        }
    }

    #[test]
    fn the_bindings_root_declares_every_module_and_reexports_those_with_items() {
        let root = bindings_root();
        assert!(root.starts_with("mod bitmasks;\nmod command_info;\n"));
        assert!(root.contains("mod formats;\n"));
        assert!(!root.contains("pub use formats::*;"));
        assert!(root.ends_with("pub use structs::*;\npub use version::*;\n"));
    }
}
