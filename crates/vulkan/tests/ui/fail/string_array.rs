//! The array of string pointers must outlive the struct.
use vulkan::vk;

fn main() {
    let info;
    {
        let names = [vk::CStrPtr::new(vk::KHR_SWAPCHAIN_NAME)];
        info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
    }
    let _ = info;
}
