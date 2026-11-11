//! One struct cannot be linked into two chains, or twice into one.
use vulkan::vk;

fn main() {
    let mut features = vk::PhysicalDeviceVulkan12Features::default();
    let info = vk::DeviceCreateInfo::default()
        .push_next(&mut features)
        .push_next(&mut features);
    let _ = info;
}
