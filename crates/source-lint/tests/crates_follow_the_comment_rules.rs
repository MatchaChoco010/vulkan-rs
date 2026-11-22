use std::path::{Path, PathBuf};

use source_lint::{USER_FACING_CRATES, check_crates};

#[test]
fn crates_follow_the_comment_rules() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/source-lint has a workspace root two levels up");
    let report = check_crates(workspace_root);

    for must_be_scanned in [
        file!(),
        "crates/vulkan/src/lib.rs",
        "crates/vulkan/src/vk/generated/structs.rs",
        "crates/vulkan/tests/api.rs",
        "crates/vulkan-alloc/src/allocator.rs",
        "crates/vulkan-codegen/src/lib.rs",
    ] {
        assert!(
            report.scanned.contains(Path::new(must_be_scanned)),
            "{must_be_scanned} was not scanned"
        );
    }
    for user_facing in USER_FACING_CRATES {
        let root_module = PathBuf::from(user_facing).join("src/lib.rs");
        assert!(
            report.public_modules.contains(&root_module),
            "{} is not a public module",
            root_module.display()
        );
    }
    assert!(
        report
            .public_modules
            .contains(Path::new("crates/vulkan/src/vk/mod.rs")),
        "the module tree of the vulkan crate was not followed"
    );

    assert!(
        report.violations.is_empty(),
        "the Rust sources under crates/ break the comment rules; found {}:\n{}",
        report.violations.len(),
        report.violations.join("\n")
    );
}
