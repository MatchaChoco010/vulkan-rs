use std::process::Command;

#[test]
fn generated_vulkan_crate_compiles_with_all_features() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");
    let status = Command::new("cargo")
        .arg("check")
        .arg("-p")
        .arg("vulkan")
        .arg("--all-features")
        .current_dir(root)
        .status()
        .expect("run cargo check -p vulkan --all-features");
    assert!(
        status.success(),
        "generated Vulkan bindings do not compile with all features"
    );
}

#[test]
fn examples_compile_against_generated_api() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .expect("workspace root");
    for manifest in [
        "examples/headless-triangle/Cargo.toml",
        "examples/compute-buffer/Cargo.toml",
        "examples/descriptor-heap-14/Cargo.toml",
        "examples/ray-tracing-pipeline/Cargo.toml",
        "examples/mesh-shader/Cargo.toml",
        "examples/wayland-triangle/Cargo.toml",
        "examples/wayland-dynamic-rendering/Cargo.toml",
    ] {
        let status = Command::new("cargo")
            .arg("check")
            .arg("--manifest-path")
            .arg(manifest)
            .current_dir(root)
            .status()
            .unwrap_or_else(|err| panic!("run cargo check --manifest-path {manifest}: {err}"));
        assert!(status.success(), "{manifest} does not compile");
    }
}
