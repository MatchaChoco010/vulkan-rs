use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use crate::trace::CAPTURE_LAYER;

pub fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn workspace_root() -> PathBuf {
    manifest_dir()
        .ancestors()
        .nth(2)
        .expect("tests/render-compare has a workspace root two levels up")
        .to_path_buf()
}

pub fn shader_dir() -> PathBuf {
    manifest_dir().join("shaders")
}

pub fn required() -> bool {
    std::env::var_os("VULKAN_RS_REQUIRE_RENDER_COMPARE").is_some_and(|value| value != "0")
}

pub fn rust_program() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_render-rust"))
}

pub fn c_program() -> Result<PathBuf, String> {
    static PROGRAM: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    PROGRAM.get_or_init(build_c_program).clone()
}

fn build_c_program() -> Result<PathBuf, String> {
    let include = workspace_root().join("third_party/vulkan/include");
    if !include.join("vulkan/vulkan_core.h").exists() {
        return Err(format!(
            "{} has no Vulkan headers; run `git submodule update --init`",
            include.display()
        ));
    }
    let mut sources = std::fs::read_dir(manifest_dir().join("c"))
        .expect("the C sources")
        .map(|entry| entry.expect("a directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "c"))
        .collect::<Vec<_>>();
    sources.sort();
    let program = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("render-c");
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let output = Command::new(&cc)
        .args(["-std=c11", "-O1", "-Wall", "-Wextra", "-Werror"])
        .arg("-I")
        .arg(&include)
        .args(&sources)
        .arg("-o")
        .arg(&program)
        .arg("-lvulkan")
        .output()
        .map_err(|e| format!("cannot run the C compiler `{cc}`: {e}"))?;
    assert!(
        output.status.success(),
        "{cc} failed on the C reference:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(program)
}

pub enum Run {
    Done,
    Unsupported(String),
}

fn command(program: &Path, scenario: &str, output: &Path) -> Command {
    let mut command = Command::new(program);
    command
        .arg(scenario)
        .arg(shader_dir())
        .arg(output)
        .env_remove("RENDER_COMPARE_CAPTURE_LAYER")
        .env(
            "MALLOC_PERTURB_",
            if program == rust_program() {
                "170"
            } else {
                "85"
            },
        );
    command
}

pub fn run(program: &Path, scenario: &str, output: &Path, capture: Option<&Path>) -> Run {
    let mut command = command(program, scenario, output);
    if let Some(capture) = capture {
        command
            .env("RENDER_COMPARE_CAPTURE_LAYER", CAPTURE_LAYER)
            .env("GFXRECON_CAPTURE_FILE", capture)
            .env("GFXRECON_CAPTURE_FILE_TIMESTAMP", "false")
            .env("GFXRECON_LOG_LEVEL", "warning");
    }
    let result = command
        .output()
        .unwrap_or_else(|e| panic!("cannot run {}: {e}", program.display()));
    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    if let Some(reason) = stderr
        .lines()
        .find_map(|line| line.strip_prefix("unsupported: "))
    {
        return Run::Unsupported(reason.to_string());
    }
    assert!(
        result.status.success(),
        "{} {scenario} failed ({}):\n{stdout}{stderr}",
        program.display(),
        result.status
    );
    if required() {
        assert!(
            stdout.contains("validation layer: enabled"),
            "{} {scenario} ran without the validation layer",
            program.display()
        );
    }
    Run::Done
}

pub const INVALID_CALL: &str = "invalid_call";
pub const INVALID_CALL_MESSAGE: &str = "validation message: [VUID-VkBufferCreateInfo-size-00912]";
pub const VALIDATION_FAILURE_STATUS: i32 = 3;

pub fn run_invalid_call(program: &Path, output: &Path) -> Run {
    let result = command(program, INVALID_CALL, output)
        .output()
        .unwrap_or_else(|e| panic!("cannot run {}: {e}", program.display()));
    let stdout = String::from_utf8_lossy(&result.stdout);
    let stderr = String::from_utf8_lossy(&result.stderr);
    if let Some(reason) = stderr
        .lines()
        .find_map(|line| line.strip_prefix("unsupported: "))
    {
        return Run::Unsupported(reason.to_string());
    }
    if !stdout.contains("validation layer: enabled") {
        return Run::Unsupported(format!(
            "{} ran without the validation layer",
            program.display()
        ));
    }
    assert!(
        stderr.contains(INVALID_CALL_MESSAGE),
        "{} made an invalid call, but the validation layer did not report {INVALID_CALL_MESSAGE}:\n{stdout}{stderr}",
        program.display()
    );
    assert_eq!(
        result.status.code(),
        Some(VALIDATION_FAILURE_STATUS),
        "{} made an invalid call that the validation layer reported, but did not fail:\n{stdout}{stderr}",
        program.display()
    );
    Run::Done
}

pub fn list(program: &Path) -> Vec<String> {
    let output = Command::new(program)
        .arg("--list")
        .output()
        .unwrap_or_else(|e| panic!("cannot run {}: {e}", program.display()));
    assert!(
        output.status.success(),
        "{} --list failed",
        program.display()
    );
    String::from_utf8(output.stdout)
        .expect("UTF-8")
        .lines()
        .map(str::to_string)
        .collect()
}
