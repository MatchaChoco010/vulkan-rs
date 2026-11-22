use vulkan::vk;

fn main() {
    let levels = [c"error", c"warn"];
    let _ = vk::LayerSettingEXT::default()
        .ty(vk::LayerSettingTypeEXT::STRING)
        .values(&levels);
}
