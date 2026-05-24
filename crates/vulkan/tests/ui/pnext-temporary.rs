use vulkan::vk;

fn use_info_later(_: vk::ShaderModuleCreateInfo<'_>) {}

fn main() {
    let info;
    {
        let mut validation = vk::ValidationFeaturesEXT::default();
        info = vk::ShaderModuleCreateInfo::default().push_next(&mut validation);
    }
    use_info_later(info);
}
