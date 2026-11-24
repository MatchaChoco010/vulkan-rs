use std::fmt;

use vulkan::vk;

use crate::allocation::Allocation;

/// Why an [`Allocator`](crate::Allocator) operation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    /// No memory type allowed by [`vk::MemoryRequirements::memory_type_bits`] has all the
    /// flags that the [`MemoryLocation`](crate::MemoryLocation) requires, such as no
    /// allowed type being both `HOST_VISIBLE` and `HOST_COHERENT` for
    /// [`MemoryLocation::CpuToGpu`](crate::MemoryLocation::CpuToGpu) or
    /// [`MemoryLocation::GpuToCpu`](crate::MemoryLocation::GpuToCpu).
    NoCompatibleMemoryType,
    /// The request cannot be met inside the allocator, for example because
    /// [`vk::MemoryRequirements::size`] is 0 or larger than the memory heap of the chosen
    /// memory type, or because [`vk::MemoryRequirements::alignment`] is neither 0 nor a
    /// power of two (see [`Allocator::allocate`](crate::Allocator::allocate)).
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

/// The error of [`Allocator::free`](crate::Allocator::free): the reason, and the
/// allocation that was not freed.
///
/// The allocator did not change, so [`FreeError::allocation`] is as usable as it was
/// before the call. Remove the cause and pass it to
/// [`Allocator::free`](crate::Allocator::free) again:
///
/// ```no_run
/// use vulkan_alloc::{Allocation, Allocator, FreeError};
///
/// /// # Safety
/// ///
/// /// As for `Allocator::free`, and no pointer returned by `Allocator::map_memory`
/// /// for the allocation may be used afterwards.
/// unsafe fn unmap_and_free(
///     allocator: &mut Allocator,
///     allocation: Allocation,
/// ) -> Result<(), FreeError> {
///     unsafe {
///         match allocator.free(allocation) {
///             Ok(()) => Ok(()),
///             Err(FreeError { allocation, .. }) => {
///                 while allocator.unmap_memory(&allocation).is_ok() {}
///                 allocator.free(allocation)
///             }
///         }
///     }
/// }
/// ```
///
/// It converts into [`AllocationError`] with `?`. The conversion drops the allocation,
/// which leaves its range allocated and mapped as it was until
/// [`Allocator::free_all`](crate::Allocator::free_all) (see the
/// [crate documentation](crate#leaks)).
#[derive(Debug, PartialEq, Eq)]
pub struct FreeError {
    /// The allocation that was passed to [`Allocator::free`](crate::Allocator::free).
    pub allocation: Allocation,
    /// Why it was not freed.
    pub error: AllocationError,
}

impl fmt::Display for FreeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the allocation was not freed")
    }
}

impl std::error::Error for FreeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

impl From<FreeError> for AllocationError {
    fn from(value: FreeError) -> Self {
        value.error
    }
}
