//! Pointer to pointer: the strings must outlive the array of pointers to them.
use std::ffi::CString;
use vulkan::vk;

fn main() {
    let names;
    {
        let owned = CString::new("VK_KHR_swapchain").unwrap();
        names = [vk::CStrPtr::new(&owned)];
    }
    let info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
    let _ = info;
}
