//! A single struct stored by pointer (`Option<&T>` in spirit) must outlive the struct.
use vulkan::vk;

fn main() {
    let info;
    {
        let features = vk::PhysicalDeviceFeatures::default();
        info = vk::DeviceCreateInfo::default().enabled_features(&features);
    }
    let _ = info;
}
