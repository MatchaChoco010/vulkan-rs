mod emit;
mod model;
mod naming;

#[cfg(test)]
mod test_support;

use model::Model;
use vulkan_registry::Registry;

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

pub fn generate(ir: &Registry) -> Vec<GeneratedFile> {
    let model = Model::new(ir);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_registry_generates() {
        let files = generate(Registry::get());
        assert_eq!(files.len(), 14);
        for file in &files {
            assert!(!file.contents.is_empty(), "{} is empty", file.path);
        }
    }

    #[test]
    fn generation_is_deterministic() {
        assert_eq!(generate(Registry::get()), generate(Registry::get()));
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
