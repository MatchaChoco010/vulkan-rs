use vulkan::vk;

fn query(_: &mut vk::PhysicalDeviceFeatures2<'_>) {}

fn main() {
    let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan12);
    let timeline = vulkan12.timeline_semaphore;
    query(&mut features);
    let _ = timeline;
}
