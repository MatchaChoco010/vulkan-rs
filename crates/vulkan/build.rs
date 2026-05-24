fn main() {
    if std::env::var_os("CARGO_FEATURE_LINKED").is_none() {
        return;
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let lib = match target_os.as_str() {
        "windows" => "vulkan-1",
        _ => "vulkan",
    };
    println!("cargo:rustc-link-lib={lib}");
}
