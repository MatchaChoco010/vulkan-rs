//! `void*` with a size member is borrowed as bytes.
use vulkan::vk;

fn main() {
    let info;
    {
        let data = [0u8; 16];
        info = vk::SpecializationInfo::default().data(&data);
    }
    let _ = info;
}
