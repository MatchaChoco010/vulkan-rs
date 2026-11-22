use super::VkResult;
use core::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandLevel {
    Entry,
    Instance,
    Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandInfo {
    pub name: &'static str,
    pub level: CommandLevel,
    pub success_codes: &'static [VkResult],
    pub error_codes: &'static [VkResult],
    pub version: Option<u32>,
    pub extensions: &'static [&'static CStr],
}

impl CommandInfo {
    pub fn get(name: &str) -> Option<&'static CommandInfo> {
        super::COMMANDS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::COMMANDS[index])
    }

    pub fn can_return(&self, code: VkResult) -> bool {
        self.success_codes.contains(&code) || self.error_codes.contains(&code)
    }

    pub const fn returns_result(&self) -> bool {
        !self.success_codes.is_empty()
    }
}
