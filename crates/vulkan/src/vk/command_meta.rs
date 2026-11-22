use super::VkResult;
use core::ffi::CStr;

/// Which function pointer table, and so which wrapper type, a command belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandLevel {
    /// A global command, loaded without an instance: a method of
    /// [`Entry`](crate::Entry), in [`EntryFn`](super::EntryFn).
    Entry,
    /// Loaded with `vkGetInstanceProcAddr` for an instance: a method of
    /// [`Instance`](crate::Instance), in [`InstanceFn`](super::InstanceFn).
    Instance,
    /// Loaded with `vkGetDeviceProcAddr` for a device: a method of
    /// [`Device`](crate::Device), in [`DeviceFn`](super::DeviceFn).
    Device,
}

/// What the registry says about one command.
///
/// [`command`](super::command) has one constant per command, named after the C name in
/// upper snake case without the `vk` prefix (`vk::command::QUEUE_SUBMIT` describes
/// `vkQueueSubmit`), and [`COMMANDS`](super::COMMANDS) lists them all, sorted by name. Both
/// cover the commands of every platform and provisional extension, whatever Cargo features
/// are enabled.
///
/// ```
/// use vulkan::vk;
///
/// let submit = vk::command::QUEUE_SUBMIT;
/// assert_eq!(submit.name, "vkQueueSubmit");
/// assert_eq!(submit.level, vk::CommandLevel::Device);
/// assert!(submit.can_return(vk::VkResult::ERROR_DEVICE_LOST));
/// assert_eq!(vk::command::CMD_BEGIN_RENDERING.version, Some(vk::API_VERSION_1_3));
/// assert_eq!(
///     vk::command::ACQUIRE_NEXT_IMAGE_KHR.extensions,
///     [vk::KHR_SWAPCHAIN_NAME]
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandInfo {
    /// The C name, for example `"vkQueueSubmit"`.
    pub name: &'static str,
    /// The table the command is loaded into.
    pub level: CommandLevel,
    /// The success codes the command can return. Empty if it does not return `VkResult`.
    pub success_codes: &'static [VkResult],
    /// The error codes the registry lists for the command. Empty if it does not return
    /// `VkResult`.
    pub error_codes: &'static [VkResult],
    /// The core version (`vk::API_VERSION_1_x`) that provides the command, if one does.
    /// `None` for Vulkan 1.0 commands and for commands that only extensions provide.
    pub version: Option<u32>,
    /// The names of the extensions that provide the command. Empty for commands that only
    /// a core version provides.
    pub extensions: &'static [&'static CStr],
}

impl CommandInfo {
    /// Looks a command up by its C name, for example `CommandInfo::get("vkQueueSubmit")`.
    /// Returns `None` for a name that is not a command of the registry.
    ///
    /// ```
    /// use vulkan::vk::{self, CommandInfo};
    ///
    /// assert_eq!(CommandInfo::get("vkCmdDraw"), Some(&vk::command::CMD_DRAW));
    /// assert!(CommandInfo::get("vkCreateWin32SurfaceKHR").is_some());
    /// assert!(CommandInfo::get("vkNoSuchCommand").is_none());
    /// ```
    pub fn get(name: &str) -> Option<&'static CommandInfo> {
        super::COMMANDS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::COMMANDS[index])
    }

    /// Whether the registry lists `code` among the results of this command, as a success
    /// or as an error code.
    ///
    /// For example, `vk::command::QUEUE_SUBMIT.can_return(vk::VkResult::ERROR_DEVICE_LOST)`
    /// tells whether a failure of that command may mean the device was lost.
    pub fn can_return(&self, code: VkResult) -> bool {
        self.success_codes.contains(&code) || self.error_codes.contains(&code)
    }

    /// Whether the command returns `VkResult`.
    pub const fn returns_result(&self) -> bool {
        !self.success_codes.is_empty()
    }
}
