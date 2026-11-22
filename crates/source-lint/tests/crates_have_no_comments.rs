use std::fs;
use std::path::{Path, PathBuf};

use source_lint::find_comments;

fn collect_rust_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

#[test]
fn crates_contain_no_comments() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/source-lint has a workspace root two levels up");
    let mut files = Vec::new();
    collect_rust_files(&workspace_root.join("crates"), &mut files);
    files.sort();
    for must_be_scanned in [
        file!(),
        "crates/vulkan/src/lib.rs",
        "crates/vulkan/src/vk/generated/structs.rs",
    ] {
        assert!(
            files.contains(&workspace_root.join(must_be_scanned)),
            "{must_be_scanned} was not scanned"
        );
    }

    let mut violations = Vec::new();
    for path in &files {
        let source =
            fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        let relative = path.strip_prefix(workspace_root).unwrap_or(path);
        for comment in find_comments(&source) {
            violations.push(format!(
                "{}:{}: {}",
                relative.display(),
                comment.line,
                comment.kind.describe()
            ));
        }
    }
    assert!(
        violations.is_empty(),
        "comments are not allowed in the Rust sources under crates/; found {}:\n{}",
        violations.len(),
        violations.join("\n")
    );
}
