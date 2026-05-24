use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    compile_shader(
        "src/shaders/compute.slang",
        "computeMain",
        "compute",
        &out.join("compute.spv"),
    );
}

fn compile_shader(input: &str, entry: &str, stage: &str, output: &PathBuf) {
    println!("cargo:rerun-if-changed={input}");
    let status = Command::new("slangc")
        .arg(input)
        .arg("-entry")
        .arg(entry)
        .arg("-stage")
        .arg(stage)
        .arg("-target")
        .arg("spirv")
        .arg("-profile")
        .arg("glsl_450")
        .arg("-o")
        .arg(output)
        .status()
        .expect("failed to run slangc; install Slang and put slangc in PATH");
    assert!(status.success(), "slangc failed for {input}:{entry}");
}
