use anyhow::{Result, bail};
use vulkan::{Device, vk};

/// SPIR-V as the 32-bit words Vulkan takes. `include_bytes!` gives no alignment
/// guarantee, so the bytes are copied rather than reinterpreted.
pub fn spirv_words(bytes: &[u8]) -> Result<Vec<u32>> {
    if !bytes.len().is_multiple_of(4) {
        bail!("SPIR-V size is not a multiple of four bytes");
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect())
}

pub unsafe fn create_shader_module(device: &Device, spirv: &[u8]) -> Result<vk::ShaderModule> {
    let code = spirv_words(spirv)?;
    let info = vk::ShaderModuleCreateInfo::default().code(&code);
    Ok(unsafe { device.create_shader_module(&info, None) }?)
}
