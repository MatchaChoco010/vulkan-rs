use vulkan::vk;

fn use_info_later(_: vk::ShaderModuleCreateInfo<'_>) {}

fn main() {
    let info;
    {
        let code = [0x0723_0203_u32, 0, 0, 0];
        info = vk::ShaderModuleCreateInfo::default().code(&code);
    }
    use_info_later(info);
}
