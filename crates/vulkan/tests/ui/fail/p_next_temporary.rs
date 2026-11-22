use vulkan::vk;

fn main() {
    let info = vk::DeviceCreateInfo::default()
        .push_next(&mut vk::PhysicalDeviceVulkan12Features::default());
    let _ = info;
}
