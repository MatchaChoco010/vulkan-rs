use anyhow::Result;
use std::ffi::{CStr, c_void};
use std::ops::Deref;
use std::rc::Rc;
use vulkan::{Entry, Instance, vk};

use crate::extensions::has_extension;

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

/// An instance and the messenger that prints what the validation layer reports, which
/// is null without the layer. The clones share them, and the last one to go destroys the
/// messenger and then the instance. Whatever is created from the instance keeps a clone,
/// so the instance outlives it.
#[derive(Clone)]
pub struct VulkanInstance(Rc<InstanceOwner>);

struct InstanceOwner {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl Deref for VulkanInstance {
    type Target = Instance;

    fn deref(&self) -> &Instance {
        &self.0.instance
    }
}

impl Drop for InstanceOwner {
    fn drop(&mut self) {
        // SAFETY: every object created from the instance holds a clone of the
        // , so all of them are gone.
        unsafe {
            if !self.messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

/// Creates an instance with `extensions` enabled.
///
/// When `VK_LAYER_KHRONOS_validation` and `VK_EXT_debug_utils` are both installed, the
/// layer is enabled too and its warnings and errors go to stderr. Otherwise the instance
/// runs without them, so that the examples also run where the layer is not installed.
pub unsafe fn create_instance(
    entry: &Entry,
    application_name: &CStr,
    api_version: u32,
    extensions: &[&CStr],
) -> Result<VulkanInstance> {
    unsafe {
        let has_validation = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .any(|layer| layer.layer_name_as_c_str() == Ok(VALIDATION_LAYER));
        let debug = has_validation
            && has_extension(
                &entry.enumerate_instance_extension_properties(None)?,
                vk::EXT_DEBUG_UTILS_NAME,
            );

        let mut layer_names = Vec::new();
        let mut extension_names = extensions
            .iter()
            .map(|name| vk::CStrPtr::new(name))
            .collect::<Vec<_>>();
        if debug {
            layer_names.push(vk::CStrPtr::new(VALIDATION_LAYER));
            extension_names.push(vk::CStrPtr::new(vk::EXT_DEBUG_UTILS_NAME));
        }
        let application = vk::ApplicationInfo::default()
            .application_name(application_name)
            .api_version(api_version);
        let mut owner = InstanceOwner {
            instance: entry.create_instance(
                &vk::InstanceCreateInfo::default()
                    .application_info(&application)
                    .enabled_layer_names(&layer_names)
                    .enabled_extension_names(&extension_names),
                None,
            )?,
            messenger: vk::DebugUtilsMessengerEXT::null(),
        };
        if debug {
            owner.messenger = owner.instance.create_debug_utils_messenger_ext(
                &vk::DebugUtilsMessengerCreateInfoEXT::default()
                    .message_severity(
                        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                    )
                    .message_type(
                        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                    )
                    .pfn_user_callback(Some(print_message)),
                None,
            )?;
        }
        Ok(VulkanInstance(Rc::new(owner)))
    }
}

unsafe extern "system" fn print_message(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    // SAFETY: Vulkan passes a valid callback data pointer with a NUL-terminated message.
    let message = unsafe { CStr::from_ptr((*data).p_message) };
    eprintln!("validation: {}", message.to_string_lossy());
    vk::FALSE
}
