use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use vulkan_codegen::{GENERATED_SRC, GENERATED_TESTS, generate, required_features};
use vulkan_registry::Registry;

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
    check_features(&root, ir);

    let files = generate(ir);
    let expected = files
        .iter()
        .map(|(path, _)| root.join(path))
        .collect::<BTreeSet<_>>();
    for dir in [GENERATED_SRC, GENERATED_TESTS] {
        let dir = root.join(dir);
        fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("create {}: {e}", dir.display()));
        // Remove files that an older generator version wrote and this one no longer does.
        for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display())) {
            let path = entry.expect("directory entry").path();
            if path.is_file() && !expected.contains(&path) {
                fs::remove_file(&path).unwrap_or_else(|e| panic!("remove {}: {e}", path.display()));
            }
        }
    }
    for (path, body) in &files {
        let path = root.join(path);
        fs::write(&path, body).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    }
    eprintln!(
        "generated {} files from {} ({})",
        files.len(),
        ir.headers_tag,
        ir.headers_commit
    );
}

/// Generated items are gated on Cargo features named after registry platforms. A new
/// platform in the registry must get a feature before the output can compile.
fn check_features(root: &Path, ir: &Registry) {
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
