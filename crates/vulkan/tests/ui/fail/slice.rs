//! A slice stored through a setter must outlive the struct.
use vulkan::vk;

fn main() {
    let info;
    {
        let priorities = [1.0];
        info = vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities);
    }
    let _ = info;
}
