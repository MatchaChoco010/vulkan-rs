use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use vulkan_codegen::{GENERATED_SRC, GENERATED_TESTS, GeneratedFile, generate, required_features};
use vulkan_registry::{Docs, Registry};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let root = match args.as_slice() {
        [_program, root] => PathBuf::from(root),
        _ => {
            eprintln!("usage: vulkan-codegen <workspace-root>");
            std::process::exit(2);
        }
    };

    let ir = Registry::get();
    assert_manifest_declares_features(&root, ir);

    let files = generate(ir, Docs::get());
    remove_files_no_longer_generated(&root, &files);
    for GeneratedFile { path, contents } in &files {
        let path = root.join(path);
        fs::write(&path, contents).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
    eprintln!(
        "generated {} files from {} ({})",
        files.len(),
        ir.headers_tag,
        ir.headers_commit
    );
}

fn remove_files_no_longer_generated(root: &Path, files: &[GeneratedFile]) {
    let generated = files
        .iter()
        .map(|file| root.join(&file.path))
        .collect::<BTreeSet<_>>();
    for dir in [GENERATED_SRC, GENERATED_TESTS] {
        let dir = root.join(dir);
        fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("create {}: {e}", dir.display()));
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
            let path = entry
                .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
                .path();
            if path.is_file() && !generated.contains(&path) {
                fs::remove_file(&path).unwrap_or_else(|e| panic!("remove {}: {e}", path.display()));
            }
        }
    }
}

fn assert_manifest_declares_features(root: &Path, ir: &Registry) {
    let manifest_path = root.join("crates/vulkan/Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", manifest_path.display()));
    let missing = required_features(ir)
        .into_iter()
        .filter(|feature| {
            !manifest
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("{feature} =")))
        })
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "add these features to {}: {}",
        manifest_path.display(),
        missing.join(", ")
    );
}
