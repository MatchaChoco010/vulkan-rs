//! A C string must outlive the struct that points to it.
use std::ffi::CString;
use vulkan::vk;

fn main() {
    let info;
    {
        let name = CString::new("app").unwrap();
        info = vk::ApplicationInfo::default().application_name(&name);
    }
    let _ = info;
}
