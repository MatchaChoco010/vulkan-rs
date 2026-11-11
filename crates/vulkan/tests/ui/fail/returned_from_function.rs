//! A create info cannot be returned past the locals it points to.
use vulkan::vk;

fn make<'a>() -> vk::DeviceQueueCreateInfo<'a> {
    let priorities = [1.0];
    vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities)
}

fn main() {
    let _ = make();
}
