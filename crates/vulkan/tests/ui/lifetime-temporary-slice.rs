use vulkan::vk;

fn use_info_later(_: vk::DeviceQueueCreateInfo<'_>) {}

fn main() {
    let info;
    {
        let priorities = [1.0];
        info = vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities);
    }
    use_info_later(info);
}
