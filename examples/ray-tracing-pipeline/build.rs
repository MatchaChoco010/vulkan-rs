use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/shaders/raygen.slang");
    println!("cargo:rerun-if-changed=src/shaders/miss.slang");
    println!("cargo:rerun-if-changed=src/shaders/closesthit.slang");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").context("OUT_DIR is not set")?);
    compile_shader(
        "src/shaders/raygen.slang",
        "raygenMain",
        "raygeneration",
        &out_dir.join("raygen.spv"),
    )?;
    compile_shader(
        "src/shaders/miss.slang",
        "missMain",
        "miss",
        &out_dir.join("miss.spv"),
    )?;
    compile_shader(
        "src/shaders/closesthit.slang",
        "closestHitMain",
        "closesthit",
        &out_dir.join("closesthit.spv"),
    )?;
    Ok(())
}

fn compile_shader(path: &str, entry: &str, stage: &str, output: &std::path::Path) -> Result<()> {
    let status = Command::new("slangc")
        .args([
            path,
            "-entry",
            entry,
            "-stage",
            stage,
            "-target",
            "spirv",
            "-profile",
            "glsl_460",
            "-capability",
            "SPV_EXT_physical_storage_buffer",
            "-fvk-use-c-layout",
            "-emit-spirv-directly",
            "-fvk-use-entrypoint-name",
            "-o",
        ])
        .arg(output)
        .status()
        .context("failed to execute slangc")?;
    if !status.success() {
        return Err(anyhow!("slangc failed for {path}"));
    }
    Ok(())
}
