use std::path::Path;

use source_lint::check_versions;

#[test]
fn markdown_names_the_workspace_version() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/source-lint has a workspace root two levels up");
    let report = check_versions(workspace_root, env!("CARGO_PKG_VERSION"));

    for must_be_scanned in [
        "README.md",
        "crates/vulkan/README.md",
        "crates/vulkan-registry/README.md",
    ] {
        assert!(
            report
                .scanned
                .iter()
                .any(|file| file == Path::new(must_be_scanned)),
            "{must_be_scanned} was not scanned"
        );
    }
    assert!(
        report
            .mentions
            .iter()
            .any(|(file, mention)| file == Path::new("README.md") && mention.has_semver),
        "README.md has no version example to check"
    );

    assert!(
        report.violations.is_empty(),
        "the Markdown files name a version other than the one in Cargo.toml; found {}:\n{}",
        report.violations.len(),
        report.violations.join("\n")
    );
}
