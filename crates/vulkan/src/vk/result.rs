use super::VkResult;
use core::fmt;

/// The result of a command wrapper: `Ok` with the outputs, or `Err` with the
/// [`VkResult`] the command failed with.
///
/// A command with more than one success code returns the code, or a value derived from
/// it, inside `Ok`, except the commands that create several objects at once and return
/// them, which use `core::result::Result<Vec<T>, (Vec<T>, VkResult)>` instead and return
/// every code other than `SUCCESS` as the error, together with the objects.
///
/// `VkResult` implements `Display` (its name) and `core::error::Error`, so `?` converts
/// it into other error types such as `Box<dyn Error>`.
pub type Result<T> = core::result::Result<T, VkResult>;

impl VkResult {
    /// Converts the code into a [`Result`]: `Ok(())` for every non-negative code (the
    /// success codes, including `INCOMPLETE` and `SUBOPTIMAL_KHR`), `Err(self)` for the
    /// negative error codes.
    ///
    /// # Errors
    ///
    /// Returns `Err(self)` if the code is negative.
    ///
    /// ```
    /// use vulkan::vk::VkResult;
    ///
    /// assert_eq!(VkResult::SUBOPTIMAL_KHR.into_result(), Ok(()));
    /// assert_eq!(
    ///     VkResult::ERROR_DEVICE_LOST.into_result(),
    ///     Err(VkResult::ERROR_DEVICE_LOST)
    /// );
    /// ```
    pub const fn into_result(self) -> Result<()> {
        if self.0 >= 0 { Ok(()) } else { Err(self) }
    }

    /// Whether the code is an error code, that is, negative.
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
