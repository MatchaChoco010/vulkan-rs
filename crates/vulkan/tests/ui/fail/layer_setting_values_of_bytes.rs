use vulkan::vk;

fn main() {
    let enabled = vk::TRUE.to_ne_bytes();
    let _ = vk::LayerSettingEXT::default()
        .ty(vk::LayerSettingTypeEXT::BOOL32)
        .values(&enabled);
}
