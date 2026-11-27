use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use vulkan_codegen::{
    GENERATED_SRC, GENERATED_TESTS, assert_aligned_memory_exists,
    assert_listed_legacy_aliases_are_left_out, assert_listed_legacy_macros_are_marked,
    assert_listed_members_exist, generate, required_features,
};
use vulkan_registry::{Docs, Registry};

const MANIFEST: &str = "crates/vulkan/Cargo.toml";

fn committed_manifest() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the crate is two directories below the workspace root")
        .join(MANIFEST);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn workspace_with_manifest(name: &str, manifest: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove the workspace of an earlier run");
    }
    write(&root.join(MANIFEST), manifest);
    root
}

fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("a file in a directory"))
        .unwrap_or_else(|error| panic!("create the directory of {}: {error}", path.display()));
    fs::write(path, contents).unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
}

fn run_generator(arguments: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_vulkan-codegen"))
        .args(arguments)
        .output()
        .expect("run vulkan-codegen")
}

fn names_in(directory: &Path) -> BTreeSet<String> {
    fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()))
        .map(|entry| {
            entry
                .expect("a directory entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

#[test]
fn the_binary_writes_every_generated_file_and_removes_the_other_files_next_to_them() {
    let root = workspace_with_manifest("writes-the-generated-files", &committed_manifest());
    let source = root.join(GENERATED_SRC);
    write(&source.join("no_longer_generated.rs"), "pub struct Old;\n");
    write(&source.join("structs.rs"), "pub struct Outdated;\n");
    write(&source.join("directory/kept.rs"), "pub struct Kept;\n");
    write(
        &source
            .parent()
            .expect("the generated directory is in a directory")
            .join("handwritten.rs"),
        "pub struct Handwritten;\n",
    );

    let output = run_generator(&[&root]);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "vulkan-codegen failed: {stderr}");

    let ir = Registry::get();
    let files = generate(ir, Docs::get());
    assert_eq!(
        stderr.trim(),
        format!(
            "generated {} files from {} ({})",
            files.len(),
            ir.headers_tag,
            ir.headers_commit
        ),
        "the summary names the number of files and the revision of the registry"
    );
    for file in &files {
        let written = fs::read_to_string(root.join(&file.path))
            .unwrap_or_else(|error| panic!("{} was not written: {error}", file.path));
        assert!(
            written == file.contents,
            "{} does not hold what `generate` returns for it",
            file.path
        );
    }
    for directory in [GENERATED_SRC, GENERATED_TESTS] {
        let mut expected = files
            .iter()
            .filter_map(|file| file.path.strip_prefix(&format!("{directory}/")))
            .map(str::to_string)
            .collect::<BTreeSet<_>>();
        if directory == GENERATED_SRC {
            expected.insert("directory".to_string());
        }
        assert_eq!(
            names_in(&root.join(directory)),
            expected,
            "{directory} holds the generated files and the directory that was there"
        );
    }
    assert!(
        source.join("directory/kept.rs").is_file(),
        "a file in a directory below the generated files is removed"
    );
    assert!(
        root.join("crates/vulkan/src/vk/handwritten.rs").is_file(),
        "a file outside of the generated directories is removed"
    );
    fs::remove_dir_all(&root).expect("remove the workspace");
}

#[test]
fn the_binary_names_the_features_the_manifest_lacks_and_writes_nothing() {
    let manifest = committed_manifest();
    let required = required_features(Registry::get());
    assert!(
        required.len() > 10,
        "the registry requires only {} features",
        required.len()
    );
    let lacking = [
        &required[..1],
        &required[required.len() - 1..],
        &required[1..4],
    ];
    for (case, lacking) in lacking.into_iter().enumerate() {
        let without = manifest
            .lines()
            .filter(|line| {
                !lacking
                    .iter()
                    .any(|feature| line.trim_start().starts_with(&format!("{feature} =")))
            })
            .map(|line| format!("{line}\n"))
            .collect::<String>();
        assert_ne!(without, manifest, "the manifest declares {lacking:?}");
        let root = workspace_with_manifest(&format!("lacks-features-{case}"), &without);

        let output = run_generator(&[&root]);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !output.status.success(),
            "vulkan-codegen accepts a manifest without {lacking:?}"
        );
        let message = stderr
            .lines()
            .find(|line| line.contains("add these features to"))
            .unwrap_or_else(|| panic!("no line of `{stderr}` asks for the features"));
        assert!(
            message.contains(MANIFEST),
            "`{message}` does not name the manifest"
        );
        let named = message
            .rsplit_once(": ")
            .map(|(_, features)| features.split(", ").collect::<Vec<_>>());
        assert_eq!(
            named,
            Some(lacking.iter().map(String::as_str).collect()),
            "`{message}` names other features than the manifest lacks"
        );
        assert!(
            !root.join(GENERATED_SRC).exists() && !root.join(GENERATED_TESTS).exists(),
            "vulkan-codegen wrote files although the manifest lacks features"
        );
        fs::remove_dir_all(&root).expect("remove the workspace");
    }
}

#[test]
fn the_binary_asks_for_exactly_one_workspace_root() {
    let root = Path::new("a-workspace-that-does-not-exist");
    for arguments in [&[][..], &[root, root]] {
        let output = run_generator(arguments);
        assert_eq!(
            output.status.code(),
            Some(2),
            "{} arguments are accepted",
            arguments.len()
        );
        assert_eq!(
            String::from_utf8_lossy(&output.stderr).trim(),
            "usage: vulkan-codegen <workspace-root>"
        );
    }
    assert!(!root.exists(), "{} was created", root.display());
}

#[test]
#[should_panic(expected = "the registry has no structure ")]
fn a_registry_without_the_listed_structures_fails_the_check_of_the_unmarked_arrays() {
    assert_listed_members_exist(&Registry::default());
}

#[test]
#[should_panic(expected = "the registry has no command ")]
fn a_registry_without_the_listed_commands_fails_the_check_of_the_aligned_memory() {
    assert_aligned_memory_exists(&Registry::default());
}

#[test]
#[should_panic(
    expected = "`VK_API_VERSION` is in MACROS_THAT_THE_BINDINGS_LEAVE_OUT, but the registry does not mark it as legacy"
)]
fn a_registry_without_the_listed_legacy_macros_fails_the_check_of_the_macros_left_out() {
    assert_listed_legacy_macros_are_marked(&Registry::default());
}

#[test]
#[should_panic(
    expected = "`STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED` of `StdVideoAV1ColorPrimaries` is in ALIASES_THAT_THE_REGISTRY_DATA_LEAVES_OUT, but the registry does not call it a legacy alias"
)]
fn a_registry_without_the_listed_legacy_aliases_fails_the_check_of_the_aliases_left_out() {
    assert_listed_legacy_aliases_are_left_out(&Registry::default());
}
