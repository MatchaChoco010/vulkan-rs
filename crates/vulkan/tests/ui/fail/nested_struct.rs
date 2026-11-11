//! A borrow inside a nested struct is carried by the outer struct.
use vulkan::vk;

fn main() {
    let info;
    let queues;
    {
        let priorities = [1.0];
        queues = [vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities)];
    }
    info = vk::DeviceCreateInfo::default().queue_create_infos(&queues);
    let _ = info;
}
