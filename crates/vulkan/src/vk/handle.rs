use super::ObjectType;

/// Implemented by every Vulkan handle type, dispatchable (such as `vk::Device`) and
/// non-dispatchable (such as `vk::Buffer`).
///
/// It gives the `VkObjectType` of the handle and converts it to and from the `uint64_t`
/// form that Vulkan uses where a member can name an object of any type, such as
/// `VkDebugUtilsObjectNameInfoEXT::objectHandle`.
///
/// ```
/// use vulkan::vk::{self, Handle};
///
/// let buffer = vk::Buffer::from_raw_u64(0x10);
/// assert_eq!(buffer.as_raw_u64(), 0x10);
/// assert_eq!(vk::Buffer::TYPE, vk::ObjectType::BUFFER);
/// ```
pub trait Handle: Copy {
    /// The `VkObjectType` of this handle type, for example `ObjectType::BUFFER` for
    /// `vk::Buffer`.
    const TYPE: ObjectType;
    /// The handle as a `uint64_t`: its value for non-dispatchable handles, its pointer
    /// address for dispatchable handles.
    fn as_raw_u64(self) -> u64;
    /// The handle with the given `uint64_t` value, the inverse of
    /// [`as_raw_u64`](Handle::as_raw_u64). For a dispatchable handle the value is
    /// converted to a pointer, truncated to the pointer width.
    fn from_raw_u64(value: u64) -> Self;
}
