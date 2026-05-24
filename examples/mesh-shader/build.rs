use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/shaders/mesh.slang");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").context("OUT_DIR is not set")?);
    compile("taskMain", "task", &out_dir.join("task.spv"))?;
    compile("meshMain", "mesh", &out_dir.join("mesh.spv"))?;
    compile("fragmentMain", "fragment", &out_dir.join("fragment.spv"))?;
    Ok(())
}

fn compile(entry: &str, stage: &str, output: &std::path::Path) -> Result<()> {
    let status = Command::new("slangc")
        .args([
            "src/shaders/mesh.slang",
            "-entry",
            entry,
            "-stage",
            stage,
            "-target",
            "spirv",
            "-profile",
            "glsl_460+spirv_1_4",
            "-o",
        ])
        .arg(output)
        .status()
        .context("failed to execute slangc")?;
    if !status.success() {
        return Err(anyhow!("slangc failed for {entry}"));
    }
    Ok(())
}
