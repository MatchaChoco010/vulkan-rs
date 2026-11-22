use super::VkResult;
use core::fmt;

pub type Result<T> = core::result::Result<T, VkResult>;

impl VkResult {
    pub const fn into_result(self) -> Result<()> {
        if self.0 >= 0 { Ok(()) } else { Err(self) }
    }

    pub const fn is_error(self) -> bool {
        self.0 < 0
    }
}

impl fmt::Display for VkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for VkResult {}
