use std::fmt;

use vulkan::vk;

/// Why an [`Allocator`](crate::Allocator) operation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    /// No memory type allowed by [`vk::MemoryRequirements::memory_type_bits`] has all the
    /// flags that the [`MemoryLocation`](crate::MemoryLocation) requires.
    NoCompatibleMemoryType,
    /// The request cannot be met inside the allocator, for example because
    /// [`vk::MemoryRequirements::size`] is 0.
    ///
    /// The device running out of memory is reported as [`AllocationError::Vulkan`] with
    /// the result of `vkAllocateMemory`, such as
    /// [`vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY`].
    OutOfMemory,
    /// A Vulkan call (`vkAllocateMemory`, `vkMapMemory`, `vkFlushMappedMemoryRanges` or
    /// `vkInvalidateMappedMemoryRanges`) failed with this result.
    Vulkan(vk::VkResult),
    /// The allocator does not hold the allocation, for example because it comes from
    /// another allocator, or the operation does not apply to the allocation in its
    /// current state, such as mapping memory that is not host-visible or unmapping an
    /// allocation that is not mapped. Each function lists its cases under `# Errors`.
    InvalidAllocation,
}

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCompatibleMemoryType => write!(f, "no compatible Vulkan memory type"),
            Self::OutOfMemory => write!(f, "Vulkan allocator is out of memory"),
            Self::Vulkan(err) => write!(f, "{err}"),
            Self::InvalidAllocation => write!(f, "invalid or already freed allocation"),
        }
    }
}

impl std::error::Error for AllocationError {}

impl From<vk::VkResult> for AllocationError {
    fn from(value: vk::VkResult) -> Self {
        Self::Vulkan(value)
    }
}
