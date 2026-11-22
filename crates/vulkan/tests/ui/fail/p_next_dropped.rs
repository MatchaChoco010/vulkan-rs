use vulkan::vk;

fn main() {
    let info;
    {
        let mut features = vk::PhysicalDeviceVulkan12Features::default();
        info = vk::DeviceCreateInfo::default().push_next(&mut features);
    }
    let _ = info;
}
