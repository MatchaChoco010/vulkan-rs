use vulkan::vk;

fn use_info(_: vk::ShaderModuleCreateInfo<'_>) {}

fn main() {
    let code = [0x0723_0203_u32, 0, 0, 0];
    let mut validation = vk::ValidationFeaturesEXT::default();
    let info = vk::ShaderModuleCreateInfo::default()
        .code(&code)
        .push_next(&mut validation);
    use_info(info);
}
