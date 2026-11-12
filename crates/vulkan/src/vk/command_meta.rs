//! Registry facts about each command: where it is loaded from, what it can return and
//! what provides it.
//!
//! [`command`](super::command) has one constant per command, named after the wrapper
//! method: `vk::command::QUEUE_SUBMIT` describes `Device::queue_submit`.

use super::VkResult;
use core::ffi::CStr;

/// Which function pointer table, and so which wrapper type, a command belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandLevel {
    /// `vulkan::Entry`: loaded without an instance.
    Entry,
    /// `vulkan::Instance`: loaded with `vkGetInstanceProcAddr`.
    Instance,
    /// `vulkan::Device`: loaded with `vkGetDeviceProcAddr`.
    Device,
}

/// What the registry says about one command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandInfo {
    /// The C name, e.g. `"vkQueueSubmit"`.
    pub name: &'static str,
    pub level: CommandLevel,
    /// The success codes the command can return. Empty if it does not return `VkResult`.
    pub success_codes: &'static [VkResult],
    /// The error codes the registry lists for the command. Empty if it cannot fail or
    /// does not return `VkResult`.
    pub error_codes: &'static [VkResult],
    /// The core version (`vk::API_VERSION_1_x`) that provides the command, if one does.
    /// `None` for Vulkan 1.0 commands and for commands only extensions provide.
    pub version: Option<u32>,
    /// The extensions that provide the command.
    pub extensions: &'static [&'static CStr],
}

impl CommandInfo {
    /// Looks a command up by its C name, e.g. `CommandInfo::get("vkQueueSubmit")`.
    /// Commands of every platform are included, whatever Cargo features are enabled.
    pub fn get(name: &str) -> Option<&'static CommandInfo> {
        // `COMMANDS` is sorted by name.
        super::COMMANDS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::COMMANDS[index])
    }

    /// Whether the registry lists `code` among the results of this command, as a success
    /// or as an error.
    ///
    /// `vk::command::QUEUE_SUBMIT.can_return(vk::VkResult::ERROR_DEVICE_LOST)` tells
    /// whether a failure of that command may mean the device is gone.
    pub fn can_return(&self, code: VkResult) -> bool {
        self.success_codes.contains(&code) || self.error_codes.contains(&code)
    }

    /// `true` if the command returns `VkResult`.
    pub const fn returns_result(&self) -> bool {
        !self.success_codes.is_empty()
    }
}
