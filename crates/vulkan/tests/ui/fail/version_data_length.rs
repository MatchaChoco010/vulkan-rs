use vulkan::vk;

fn main() {
    let one_uuid = [0u8; vk::UUID_SIZE];
    let _ = vk::AccelerationStructureVersionInfoKHR::default().version_data(&one_uuid);
    let bytes: &[u8] = &[0u8; 2 * vk::UUID_SIZE];
    let _ = vk::MicromapVersionInfoEXT::default().version_data(bytes);
}
