//! SPIR-V words must outlive the create info.
use vulkan::vk;

fn main() {
    let info;
    {
        let code = [0x0723_0203_u32, 0, 0, 0];
        info = vk::ShaderModuleCreateInfo::default().code(&code);
    }
    let _ = info;
}
