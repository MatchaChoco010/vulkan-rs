use std::fmt;

use vulkan::vk;

use crate::allocation::Allocation;

/// Why an [`Allocator`](crate::Allocator) operation failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    /// No memory type allowed by [`vk::MemoryRequirements::memory_type_bits`] has all the
    /// flags that the [`MemoryLocation`](crate::MemoryLocation) requires, such as no
    /// allowed type being both `HOST_VISIBLE` and `HOST_COHERENT` for
    /// [`MemoryLocation::CpuToGpu`](crate::MemoryLocation::CpuToGpu),
    /// [`MemoryLocation::CpuOnly`](crate::MemoryLocation::CpuOnly) or
    /// [`MemoryLocation::GpuToCpu`](crate::MemoryLocation::GpuToCpu).
    NoCompatibleMemoryType,
    /// The device does not have the memory: every memory type the request may use is out
    /// of memory, because the request is larger than the memory heap of the type, or
    /// because `vkAllocateMemory` returned [`vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY`] for
    /// the type, also after the empty blocks of that heap were freed (see
    /// [`Allocator::allocate`](crate::Allocator::allocate)).
    OutOfMemory,
    /// The [`AllocationCreateDesc`](crate::AllocationCreateDesc) asks for something the
    /// allocator does not accept: a [`vk::MemoryRequirements::size`] of 0, a
    /// [`vk::MemoryRequirements::alignment`] that is neither 0 nor a power of two, or
    /// [`vk::MemoryAllocateFlags::DEVICE_MASK`] in
    /// [`AllocationCreateDesc::memory_allocate_flags`](crate::AllocationCreateDesc::memory_allocate_flags)
    /// or in [`AllocatorCreateDesc::memory_allocate_flags`][allocator] (see
    /// [`Allocator::allocate`](crate::Allocator::allocate)).
    ///
    /// It depends on the description and the flags of the allocator alone, not on the
    /// device or the state of the allocator, and is returned before any Vulkan call.
    ///
    /// [allocator]: crate::AllocatorCreateDesc::memory_allocate_flags
    InvalidAllocationCreateDesc,
    /// A Vulkan call (`vkAllocateMemory`, `vkMapMemory`, `vkFlushMappedMemoryRanges` or
    /// `vkInvalidateMappedMemoryRanges`) failed with this result.
    ///
    /// [`vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY`] from `vkAllocateMemory` is not reported
    /// here: [`Allocator::allocate`](crate::Allocator::allocate) tries another memory type
    /// then, and returns [`AllocationError::OutOfMemory`] when none is left. From the other
    /// calls it is reported here.
    ///
    /// A `vkMapMemory` call that fails is reported here, also by
    /// [`Allocator::allocate`](crate::Allocator::allocate) when it maps the allocation for
    /// [`AllocationCreateDesc::mapped`](crate::AllocationCreateDesc::mapped). A
    /// `vkMapMemory` call that succeeds without giving a pointer to the allocation is
    /// reported as [`AllocationError::MappingFailed`].
    Vulkan(vk::VkResult),
    /// The allocator does not hold the allocation: it was made by another allocator, it
    /// has been freed, or [`Allocator::free_all`](crate::Allocator::free_all) was called
    /// after it was made.
    ///
    /// Every function that takes an allocation checks this before anything else, so it is
    /// returned whatever the memory type, the mapping state and the other arguments.
    InvalidAllocation,
    /// The memory type of the allocation is not `HOST_VISIBLE`
    /// ([`Allocation::memory_properties`](crate::Allocation::memory_properties) tells), so
    /// it cannot be mapped.
    ///
    /// Returned by [`Allocator::map_memory`](crate::Allocator::map_memory),
    /// [`Allocator::map`](crate::Allocator::map) and
    /// [`Allocator::map_persistent`](crate::Allocator::map_persistent).
    /// [`Allocator::allocate`](crate::Allocator::allocate) returns such an allocation
    /// unmapped instead, also with
    /// [`AllocationCreateDesc::mapped`](crate::AllocationCreateDesc::mapped).
    NotHostVisible,
    /// The operation needs a mapping that the allocation does not have:
    ///
    /// - [`Allocator::unmap_memory`](crate::Allocator::unmap_memory) of an allocation that
    ///   holds no reference taken with
    ///   [`Allocator::map_memory`](crate::Allocator::map_memory), also when it is
    ///   persistently mapped, since that mapping is released only by
    ///   [`Allocator::free`](crate::Allocator::free);
    /// - a flush or invalidate through the allocator
    ///   ([`Allocator::flush_allocation`](crate::Allocator::flush_allocation),
    ///   [`Allocator::invalidate_allocation`](crate::Allocator::invalidate_allocation) and
    ///   their range forms) of an allocation that is not mapped in any way, whether its
    ///   memory type has `HOST_COHERENT` or not.
    NotMapped,
    /// [`Allocator::free`](crate::Allocator::free) was given an allocation that still holds
    /// references taken with [`Allocator::map_memory`](crate::Allocator::map_memory).
    ///
    /// Release them with [`Allocator::unmap_memory`](crate::Allocator::unmap_memory), which
    /// returns [`AllocationError::NotMapped`] once none is left, and free the allocation
    /// again (see [`FreeError`]).
    StillMapped,
    /// Mapping cannot give a pointer to the allocation:
    ///
    /// - `vkMapMemory` succeeded but returned a null pointer;
    /// - the address of the allocation (the address of the mapped block plus
    ///   [`Allocation::offset`](crate::Allocation::offset)) cannot be represented as a
    ///   non-null pointer;
    /// - for [`Allocator::map`](crate::Allocator::map), the allocation is larger than
    ///   [`isize::MAX`] bytes, which no slice can cover;
    /// - the allocation already holds [`usize::MAX`] mapping references.
    ///
    /// The first two also apply to [`Allocator::allocate`](crate::Allocator::allocate) when
    /// it maps the allocation for
    /// [`AllocationCreateDesc::mapped`](crate::AllocationCreateDesc::mapped), which then
    /// frees it. A `vkMapMemory` call that fails is reported as
    /// [`AllocationError::Vulkan`] instead.
    MappingFailed,
    /// A range given to
    /// [`Allocator::flush_allocation_range`](crate::Allocator::flush_allocation_range),
    /// [`Allocator::invalidate_allocation_range`](crate::Allocator::invalidate_allocation_range),
    /// [`MappedAllocation::flush_range`](crate::MappedAllocation::flush_range) or
    /// [`MappedAllocation::invalidate_range`](crate::MappedAllocation::invalidate_range)
    /// does not lie inside the [`Allocation::size`](crate::Allocation::size) bytes of the
    /// allocation, including when `offset + size` overflows.
    RangeOutOfBounds,
}

impl AllocationError {
    pub(crate) fn of_allocate_memory(result: vk::VkResult) -> Self {
        match result {
            vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY => Self::OutOfMemory,
            result => Self::Vulkan(result),
        }
    }
}

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoCompatibleMemoryType => write!(f, "no compatible Vulkan memory type"),
            Self::OutOfMemory => write!(f, "out of Vulkan device memory"),
            Self::InvalidAllocationCreateDesc => {
                write!(f, "the allocation description is not accepted")
            }
            Self::Vulkan(err) => write!(f, "{err}"),
            Self::InvalidAllocation => write!(f, "the allocator does not hold the allocation"),
            Self::NotHostVisible => write!(f, "the memory of the allocation is not host-visible"),
            Self::NotMapped => write!(f, "the allocation is not mapped"),
            Self::StillMapped => {
                write!(
                    f,
                    "the allocation still holds mappings made with map_memory"
                )
            }
            Self::MappingFailed => write!(f, "mapping gave no pointer to the allocation"),
            Self::RangeOutOfBounds => write!(f, "the range lies outside the allocation"),
        }
    }
}

impl std::error::Error for AllocationError {}

/// The error of [`Allocator::free`](crate::Allocator::free): the reason, and the
/// allocation that was not freed.
///
/// [`FreeError::error`] is [`AllocationError::InvalidAllocation`] if the allocator does
/// not hold the allocation, and [`AllocationError::StillMapped`] if it does but the
/// allocation still holds references taken with
/// [`Allocator::map_memory`](crate::Allocator::map_memory).
///
/// The allocator did not change, so [`FreeError::allocation`] is as usable as it was
/// before the call. For [`AllocationError::StillMapped`], release the references and
/// pass it to [`Allocator::free`](crate::Allocator::free) again; for
/// [`AllocationError::InvalidAllocation`], pass it to the allocator that made it, if
/// that one still holds it:
///
/// ```no_run
/// use vulkan_alloc::{Allocation, AllocationError, Allocator, FreeError};
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
///             Err(FreeError {
///                 allocation,
///                 error: AllocationError::StillMapped,
///             }) => {
///                 loop {
///                     match allocator.unmap_memory(&allocation) {
///                         Ok(()) => {}
///                         Err(AllocationError::NotMapped) => break,
///                         Err(error) => return Err(FreeError { allocation, error }),
///                     }
///                 }
///                 allocator.free(allocation)
///             }
///             result => result,
///         }
///     }
/// }
/// ```
///
/// It converts into [`AllocationError`] with `?`. The conversion drops the allocation,
/// which leaves its range allocated and mapped as it was until
/// [`Allocator::free_all`](crate::Allocator::free_all) or until the allocator is dropped
/// (see the [crate documentation](crate#leaks)).
#[derive(Debug, PartialEq, Eq)]
pub struct FreeError {
    /// The allocation that was passed to [`Allocator::free`](crate::Allocator::free).
    pub allocation: Allocation,
    /// Why it was not freed: [`AllocationError::InvalidAllocation`] or
    /// [`AllocationError::StillMapped`].
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
