use std::ffi::CStr;
use vulkan::vk;

/// Whether `name` is in a list that `vkEnumerateInstanceExtensionProperties` or
/// `vkEnumerateDeviceExtensionProperties` returned.
pub fn has_extension(available: &[vk::ExtensionProperties], name: &CStr) -> bool {
    available
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}
