use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{ChildModule, DocComments, USER_FACING_CRATES, check_source};

#[derive(Debug, Default)]
pub struct Report {
    pub scanned: BTreeSet<PathBuf>,
    pub public_modules: BTreeSet<PathBuf>,
    pub violations: Vec<String>,
}

pub fn check_crates(workspace_root: &Path) -> Report {
    let mut files = Vec::new();
    collect_rust_files(&workspace_root.join("crates"), &mut files);
    files.sort();

    let mut report = Report::default();
    let mut module_is_public = BTreeMap::new();
    for user_facing in USER_FACING_CRATES {
        let sources = workspace_root.join(user_facing).join("src");
        for file in files.iter().filter(|file| file.starts_with(&sources)) {
            module_is_public.insert(file.clone(), false);
        }
        let root_module = sources.join("lib.rs");
        assert!(
            module_is_public.contains_key(&root_module),
            "{} does not exist",
            root_module.display()
        );
        let mut pending = vec![(
            root_module,
            DocComments::OnPublicApi {
                module_is_public: true,
            },
        )];
        while let Some((file, doc_comments)) = pending.pop() {
            let public = doc_comments
                == DocComments::OnPublicApi {
                    module_is_public: true,
                };
            module_is_public.insert(file.clone(), public);
            if public {
                report
                    .public_modules
                    .insert(relative(workspace_root, &file));
            }
            for child in check_file(workspace_root, &file, doc_comments, &mut report) {
                let child_doc_comments = if child.test_only {
                    DocComments::InTestModule
                } else {
                    DocComments::OnPublicApi {
                        module_is_public: child.public,
                    }
                };
                match child_module_file(&file, &child) {
                    Some(child_file) => pending.push((child_file, child_doc_comments)),
                    None => report.violations.push(format!(
                        "{}: no source file for the module `{}`",
                        relative(workspace_root, &file).display(),
                        child.name
                    )),
                }
            }
        }
    }

    for file in &files {
        if report.scanned.contains(&relative(workspace_root, file)) {
            continue;
        }
        let doc_comments = match module_is_public.get(file) {
            Some(_) => DocComments::OnPublicApi {
                module_is_public: false,
            },
            None => DocComments::Forbidden,
        };
        check_file(workspace_root, file, doc_comments, &mut report);
    }
    report.violations.sort();
    report
}

fn relative(workspace_root: &Path, file: &Path) -> PathBuf {
    file.strip_prefix(workspace_root)
        .unwrap_or(file)
        .to_path_buf()
}

fn check_file(
    workspace_root: &Path,
    file: &Path,
    doc_comments: DocComments,
    report: &mut Report,
) -> Vec<ChildModule> {
    let name = relative(workspace_root, file);
    if !report.scanned.insert(name.clone()) {
        return Vec::new();
    }
    let source =
        fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
    match check_source(&source, doc_comments) {
        Ok(checked) => {
            report
                .violations
                .extend(checked.violations.into_iter().map(|violation| {
                    format!(
                        "{}:{}: {}: {}",
                        name.display(),
                        violation.line,
                        violation.kind.describe(),
                        violation.rule.describe()
                    )
                }));
            checked.child_modules
        }
        Err(error) => {
            let start = error.span().start();
            report.violations.push(format!(
                "{}:{}: cannot parse: {error}",
                name.display(),
                start.line
            ));
            Vec::new()
        }
    }
}

fn child_module_file(parent: &Path, child: &ChildModule) -> Option<PathBuf> {
    let parent_directory = parent.parent()?;
    let owns_its_directory = matches!(
        parent.file_name()?.to_str()?,
        "lib.rs" | "main.rs" | "mod.rs"
    );
    let mut directory = if owns_its_directory || child.path_attribute.is_some() {
        parent_directory.to_path_buf()
    } else {
        parent_directory.join(parent.file_stem()?)
    };
    if child.path_attribute.is_none() || owns_its_directory {
        directory.extend(&child.inline_parents);
    }
    let candidates = match &child.path_attribute {
        Some(path) => vec![directory.join(path)],
        None => vec![
            directory.join(format!("{}.rs", child.name)),
            directory.join(&child.name).join("mod.rs"),
        ],
    };
    candidates.into_iter().find(|candidate| candidate.is_file())
}

fn collect_rust_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let entries =
        fs::read_dir(directory).unwrap_or_else(|e| panic!("read {}: {e}", directory.display()));
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Workspace {
        root: PathBuf,
    }

    impl Workspace {
        fn new(name: &str, files: &[(&str, &str)]) -> Self {
            let root =
                std::env::temp_dir().join(format!("source-lint-{}-{name}", std::process::id()));
            if root.exists() {
                fs::remove_dir_all(&root).expect("remove the old workspace");
            }
            for user_facing in USER_FACING_CRATES {
                let lib = root.join(user_facing).join("src/lib.rs");
                fs::create_dir_all(lib.parent().expect("parent")).expect("create the crate");
                fs::write(lib, "").expect("write lib.rs");
            }
            for (path, contents) in files {
                let path = root.join(path);
                fs::create_dir_all(path.parent().expect("parent")).expect("create a directory");
                fs::write(path, contents).expect("write a source file");
            }
            Self { root }
        }

        fn violations(&self) -> Vec<String> {
            check_crates(&self.root).violations
        }
    }

    impl Drop for Workspace {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }

    const DOCUMENTED: &str = "//! Module.\n/// Item.\npub fn f() {}\n";

    #[test]
    fn doc_comments_are_allowed_only_in_the_sources_of_user_facing_crates() {
        let workspace = Workspace::new(
            "only-user-facing-sources",
            &[
                ("crates/vulkan/src/lib.rs", DOCUMENTED),
                ("crates/vulkan-alloc/src/lib.rs", DOCUMENTED),
                ("crates/vulkan/tests/api.rs", DOCUMENTED),
                ("crates/vulkan/build.rs", DOCUMENTED),
                ("crates/vulkan-codegen/src/lib.rs", DOCUMENTED),
                ("examples/demo/src/main.rs", "// Outside crates/.\n"),
            ],
        );
        let rule = "doc comments are allowed only in the sources of the user-facing crates";
        assert_eq!(
            workspace.violations(),
            [
                format!("crates/vulkan-codegen/src/lib.rs:1: inner doc comment: {rule}"),
                format!("crates/vulkan-codegen/src/lib.rs:2: outer doc comment: {rule}"),
                format!("crates/vulkan/build.rs:1: inner doc comment: {rule}"),
                format!("crates/vulkan/build.rs:2: outer doc comment: {rule}"),
                format!("crates/vulkan/tests/api.rs:1: inner doc comment: {rule}"),
                format!("crates/vulkan/tests/api.rs:2: outer doc comment: {rule}"),
            ]
        );
    }

    #[test]
    fn module_documentation_follows_the_visibility_of_the_module_declarations() {
        let workspace = Workspace::new(
            "module-visibility",
            &[
                (
                    "crates/vulkan/src/lib.rs",
                    "pub mod open;\nmod closed;\npub mod inline { pub mod deep; }\n",
                ),
                (
                    "crates/vulkan/src/open.rs",
                    "//! Public.\npub mod nested;\nmod hidden;\n",
                ),
                ("crates/vulkan/src/open/nested.rs", "//! Public.\n"),
                ("crates/vulkan/src/open/hidden.rs", "//! Private.\n"),
                (
                    "crates/vulkan/src/closed/mod.rs",
                    "//! Private.\npub mod inner;\n",
                ),
                (
                    "crates/vulkan/src/closed/inner.rs",
                    "//! Inside a private module.\n",
                ),
                ("crates/vulkan/src/inline/deep.rs", "//! Public.\n"),
                (
                    "crates/vulkan/src/orphan.rs",
                    "//! Not declared.\n/// Item.\npub fn f() {}\n",
                ),
            ],
        );
        let rule = "inner doc comment: doc comments are allowed only on the public API";
        assert_eq!(
            workspace.violations(),
            [
                format!("crates/vulkan/src/closed/inner.rs:1: {rule}"),
                format!("crates/vulkan/src/closed/mod.rs:1: {rule}"),
                format!("crates/vulkan/src/open/hidden.rs:1: {rule}"),
                format!("crates/vulkan/src/orphan.rs:1: {rule}"),
            ]
        );
    }

    #[test]
    fn the_files_of_test_modules_are_not_public_api() {
        let workspace = Workspace::new(
            "test-modules",
            &[
                (
                    "crates/vulkan/src/lib.rs",
                    "#[cfg(test)]\npub mod tests;\npub mod checked;\n",
                ),
                (
                    "crates/vulkan/src/tests.rs",
                    "//! Tests.\npub mod deeper;\n/// Helper.\npub fn helper() {}\n",
                ),
                ("crates/vulkan/src/tests/deeper.rs", DOCUMENTED),
                (
                    "crates/vulkan/src/checked.rs",
                    "#![cfg(all(test, unix))]\n//! Checked.\n/// Item.\npub fn f() {}\n",
                ),
            ],
        );
        let rule = "doc comments are allowed only on the public API";
        assert_eq!(
            workspace.violations(),
            [
                format!("crates/vulkan/src/checked.rs:2: inner doc comment: {rule}"),
                format!("crates/vulkan/src/checked.rs:3: outer doc comment: {rule}"),
                format!("crates/vulkan/src/tests.rs:1: inner doc comment: {rule}"),
                format!("crates/vulkan/src/tests.rs:3: outer doc comment: {rule}"),
                format!("crates/vulkan/src/tests/deeper.rs:1: inner doc comment: {rule}"),
                format!("crates/vulkan/src/tests/deeper.rs:2: outer doc comment: {rule}"),
            ]
        );
        assert!(
            !check_crates(&workspace.root)
                .public_modules
                .contains(Path::new("crates/vulkan/src/tests.rs")),
            "a test module was reported as a public module"
        );
    }

    #[test]
    fn a_declared_module_without_a_source_file_is_reported() {
        let workspace = Workspace::new(
            "missing-module",
            &[("crates/vulkan/src/lib.rs", "mod missing;\n")],
        );
        assert_eq!(
            workspace.violations(),
            ["crates/vulkan/src/lib.rs: no source file for the module `missing`"]
        );
    }

    #[test]
    fn a_user_facing_source_that_does_not_parse_is_reported() {
        let workspace = Workspace::new(
            "unparsable",
            &[("crates/vulkan-alloc/src/lib.rs", "\n\npub fn broken( {\n")],
        );
        let violations = workspace.violations();
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(
            violations[0].starts_with("crates/vulkan-alloc/src/lib.rs:3: cannot parse: "),
            "{violations:?}"
        );
    }
}
