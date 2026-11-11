//! There is no way to turn a borrowing struct into a `'static` one.
use vulkan::vk;

fn keep<T: 'static>(_: T) {}

fn main() {
    let priorities = [1.0];
    let info = vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities);
    keep(info);
}
