use std::fmt;

use vulkan::vk;

use crate::memory_type::MemoryLocation;

/// What [`Allocator::allocate`](crate::Allocator::allocate) is asked to allocate.
///
/// Create it with [`AllocationCreateDesc::new`], which takes the fields every
/// allocation states and gives the others the values most allocations use, then change
/// only the fields an allocation needs, with struct update syntax or by assigning to
/// them. Every field is public.
///
/// ```
/// use vulkan::vk;
/// use vulkan_alloc::{AllocationCreateDesc, MemoryLocation};
///
/// fn vertex_buffer(requirements: vk::MemoryRequirements) -> AllocationCreateDesc<'static> {
///     AllocationCreateDesc {
///         name: Some("vertices"),
///         ..AllocationCreateDesc::new(requirements, MemoryLocation::GpuOnly, true)
///     }
/// }
/// ```
#[derive(Copy, Clone)]
pub struct AllocationCreateDesc<'a> {
    /// A name for diagnostics, reported by
    /// [`Allocator::live_allocations`](crate::Allocator::live_allocations) and in the
    /// error log of an allocator dropped with live allocations. The allocator stores a
    /// copy, so the string only has to live for the call.
    pub name: Option<&'a str>,
    /// The memory requirements of the resource, as returned by
    /// `vkGetBufferMemoryRequirements` or `vkGetImageMemoryRequirements`.
    ///
    /// Only memory types whose bit is set in `memory_type_bits` are considered. A `size`
    /// of 0 is rejected with
    /// [`AllocationError::InvalidAllocationCreateDesc`](crate::AllocationError::InvalidAllocationCreateDesc),
    /// and a `size` larger than the memory heap of every memory type that may be chosen
    /// with [`AllocationError::OutOfMemory`](crate::AllocationError::OutOfMemory) (see
    /// [`Allocator::allocate`](crate::Allocator::allocate) for when the heap is known).
    ///
    /// The allocation's offset is a multiple of `alignment`, and an `alignment` of 0 is
    /// treated as 1. A caller may raise `alignment` to a larger power of two, for
    /// example to align a device address. An `alignment` that is neither 0 nor a power
    /// of two is rejected with
    /// [`AllocationError::InvalidAllocationCreateDesc`](crate::AllocationError::InvalidAllocationCreateDesc).
    /// The allocator combines `alignment` with other alignments (see
    /// [`AllocatorCreateDesc::non_coherent_atom_size`](crate::AllocatorCreateDesc::non_coherent_atom_size))
    /// by taking the largest, which is a multiple of each of them.
    pub requirements: vk::MemoryRequirements,
    /// The memory property flags the memory type must have and should have.
    pub location: MemoryLocation,
    /// Whether the resource is a linear resource, as the Vulkan specification defines
    /// the term.
    ///
    /// `true` for:
    ///
    /// - buffers, including those that hold a `VkAccelerationStructureKHR`;
    /// - images created with [`vk::ImageTiling::LINEAR`];
    /// - images created with [`vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT`] whose modifier
    ///   is `DRM_FORMAT_MOD_LINEAR`;
    /// - `VkAccelerationStructureNV` objects;
    /// - tensors created with [`vk::TensorTilingARM::LINEAR`].
    ///
    /// `false` for the non-linear resources: images created with
    /// [`vk::ImageTiling::OPTIMAL`], images with any other DRM format modifier, and
    /// tensors with another tiling.
    ///
    /// Linear and non-linear allocations are never in the same `VkDeviceMemory` at the
    /// same time: a block that holds allocations takes only allocations of their kind.
    /// An empty block that the allocator keeps (see
    /// [`Allocator::free`](crate::Allocator::free)) takes either kind, so one block can
    /// hold linear allocations at one time and non-linear ones later. Resources of a
    /// freed allocation are not used any more, so this keeps the resources in use out of
    /// each other's `bufferImageGranularity` pages. Binding a resource of the other kind
    /// to the allocation breaks that; see the safety section of
    /// [`Allocator::allocate`](crate::Allocator::allocate).
    pub linear: bool,
    /// Flags for `VkMemoryAllocateFlagsInfo`, chained to the `vkAllocateMemory` call
    /// when not empty. A buffer created with
    /// [`vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS`] needs
    /// [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`] here.
    ///
    /// The `deviceMask` of the chained structure is always 0, so
    /// [`vk::MemoryAllocateFlags::DEVICE_MASK`] is refused with
    /// [`AllocationError::InvalidAllocationCreateDesc`](crate::AllocationError::InvalidAllocationCreateDesc):
    /// Vulkan requires a `deviceMask` other than 0 when it is set. Memory on chosen
    /// physical devices of a logical device that represents more than one physical
    /// device is allocated with `vkAllocateMemory` directly (see the
    /// [crate documentation](crate#what-the-allocator-does-and-does-not-do)).
    ///
    /// Allocations with different flags never share a block. With
    /// [`vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`], the allocation gets a
    /// `VkDeviceMemory` of its own of exactly `requirements.size` bytes, at offset 0,
    /// because the implementation zeroes memory only when `vkAllocateMemory` creates it,
    /// and a range of a shared block may hold the bytes of an earlier allocation.
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    /// Handle types the memory can be exported as (`VkExportMemoryAllocateInfo`), such
    /// as `DMA_BUF_EXT`. When not empty, the allocation gets a `VkDeviceMemory` of its
    /// own of exactly `requirements.size` bytes, at offset 0, because an exported handle
    /// refers to the whole memory object.
    pub export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    /// Whether the allocation is suballocated or gets dedicated memory.
    pub allocation_scheme: AllocationScheme,
}

impl<'a> AllocationCreateDesc<'a> {
    /// Describes an allocation of `requirements` in `location`, for a linear resource if
    /// `linear` is `true` (see [`AllocationCreateDesc::linear`]).
    ///
    /// The other fields are set to: [`AllocationCreateDesc::name`] `None`, empty
    /// [`AllocationCreateDesc::memory_allocate_flags`], empty
    /// [`AllocationCreateDesc::export_handle_types`], and
    /// [`AllocationCreateDesc::allocation_scheme`]
    /// [`AllocationScheme::GpuAllocatorManaged`]. Such an allocation is placed in a shared
    /// block.
    pub const fn new(
        requirements: vk::MemoryRequirements,
        location: MemoryLocation,
        linear: bool,
    ) -> Self {
        Self {
            name: None,
            requirements,
            location,
            linear,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        }
    }
}

impl fmt::Debug for AllocationCreateDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocationCreateDesc")
            .field("name", &self.name)
            .field("requirements.size", &self.requirements.size)
            .field("requirements.alignment", &self.requirements.alignment)
            .field(
                "requirements.memory_type_bits",
                &self.requirements.memory_type_bits,
            )
            .field("location", &self.location)
            .field("linear", &self.linear)
            .field("memory_allocate_flags", &self.memory_allocate_flags)
            .field("export_handle_types", &self.export_handle_types)
            .field("allocation_scheme", &self.allocation_scheme)
            .finish()
    }
}

/// Whether an allocation is cut out of a shared block or gets a `VkDeviceMemory` of its
/// own.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocationScheme {
    /// The allocation is placed in a block shared with other allocations, unless
    /// [`AllocationCreateDesc::export_handle_types`] is not empty or
    /// [`AllocationCreateDesc::memory_allocate_flags`] contains
    /// [`vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`].
    GpuAllocatorManaged,
    /// The allocation gets a `VkDeviceMemory` of its own of exactly
    /// `requirements.size` bytes, at offset 0, allocated with a
    /// `VkMemoryDedicatedAllocateInfo` that names this buffer. Use it when
    /// `VkMemoryDedicatedRequirements` asks for a dedicated allocation, and bind only
    /// this buffer to the allocation.
    DedicatedBuffer(vk::Buffer),
    /// Like [`AllocationScheme::DedicatedBuffer`], for this image.
    DedicatedImage(vk::Image),
}

/// A range of device memory returned by
/// [`Allocator::allocate`](crate::Allocator::allocate).
///
/// Bind a resource to [`Allocation::memory`] at [`Allocation::offset`]. An allocation
/// is not [`Clone`]; pass it to [`Allocator::free`](crate::Allocator::free) once the
/// device no longer uses it. It has no destructor, so dropping it without freeing it
/// leaks its range (see the [crate documentation](crate#leaks)).
#[derive(Debug, PartialEq, Eq)]
pub struct Allocation {
    pub(crate) memory_type_index: u32,
    pub(crate) memory_properties: vk::MemoryPropertyFlags,
    pub(crate) dedicated: bool,
    pub(crate) block_index: usize,
    pub(crate) allocation_id: u64,
    pub(crate) resource_class: AllocationResourceClass,
    pub(crate) memory: vk::DeviceMemory,
    pub(crate) offset: u64,
    pub(crate) size: u64,
}

impl Allocation {
    /// The `VkDeviceMemory` that holds the range. Other allocations may share it unless
    /// this one is dedicated, exportable or zero-initialized
    /// ([`vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`]), which
    /// [`Allocation::is_dedicated`] tells.
    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    /// The byte offset of the range in [`Allocation::memory`]. It is a multiple of the
    /// alignment of the requirements the allocation was made with.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// The size of the range in bytes, at least the requested size
    /// (`requirements.size`).
    ///
    /// In a shared block, when fewer than 256 free bytes would be left right after the
    /// requested size, those bytes are added to the range, so the size is at most the
    /// requested size plus 255. In a shared block of 256 bytes or less, for example, a
    /// request of 1 byte takes the whole block. An allocation with a
    /// `VkDeviceMemory` of its own (dedicated, exportable or zero-initialized) has
    /// exactly the requested size. Padding before the range that aligns
    /// [`Allocation::offset`] is not part of the range, however small it is: it stays a
    /// free gap left by alignment, which other allocations can use (see
    /// [`AllocatorStats::free_bytes`](crate::AllocatorStats::free_bytes)).
    pub fn size(&self) -> u64 {
        self.size
    }

    /// The index of the memory type of [`Allocation::memory`] in
    /// [`vk::PhysicalDeviceMemoryProperties::memory_types`].
    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }

    /// The property flags of the memory type of [`Allocation::memory`], the
    /// `property_flags` of the entry [`Allocation::memory_type_index`] of
    /// [`AllocatorCreateDesc::memory_properties`](crate::AllocatorCreateDesc::memory_properties).
    ///
    /// They tell what the location gave, since a location that only prefers a flag can
    /// give memory without it. `HOST_VISIBLE` tells whether the allocation can be mapped.
    /// `HOST_COHERENT` tells whether mapped writes and reads need
    /// [`Allocator::flush_allocation`](crate::Allocator::flush_allocation) and
    /// [`Allocator::invalidate_allocation`](crate::Allocator::invalidate_allocation); both
    /// do nothing for memory with it, so they can be called without checking.
    /// `HOST_COHERENT` is always set for allocations made with [`MemoryLocation::CpuToGpu`]
    /// and [`MemoryLocation::GpuToCpu`]; [`MemoryLocation::GpuOnly`] and a
    /// [`MemoryLocation::Custom`] location that does not require `HOST_COHERENT` can give
    /// host-visible memory without it.
    pub fn memory_properties(&self) -> vk::MemoryPropertyFlags {
        self.memory_properties
    }

    /// Whether the allocation was given a `VkDeviceMemory` of its own because of how it
    /// was requested: it was made with [`AllocationScheme::DedicatedBuffer`] or
    /// [`AllocationScheme::DedicatedImage`], with
    /// [`AllocationCreateDesc::export_handle_types`] that are not empty, or with
    /// [`AllocationCreateDesc::memory_allocate_flags`] that contain
    /// [`vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`]. Such an allocation has offset 0
    /// and exactly the requested size, and no other allocation is placed in its memory.
    /// This is the value of
    /// [`MemoryBlockSnapshot::dedicated`](crate::MemoryBlockSnapshot::dedicated) for the
    /// block that holds the allocation.
    ///
    /// `false` for every other allocation, also for one larger than a shared block (see
    /// [`AllocatorCreateDesc::block_size`](crate::AllocatorCreateDesc::block_size)) that
    /// got a block of its own size: that block is a shared block, which holds no other
    /// allocation only because the allocation fills it.
    pub fn is_dedicated(&self) -> bool {
        self.dedicated
    }
}

/// The kind of resource a block holds, as reported by
/// [`MemoryBlockSnapshot::resource_class`](crate::MemoryBlockSnapshot::resource_class).
///
/// Allocations of different classes are never in the same `VkDeviceMemory` at the same
/// time: while a block holds allocations, it takes only allocations of their class. An
/// empty block that the allocator keeps takes the class of the next allocation placed
/// in it. So the `bufferImageGranularity` limit, which applies between a linear and a
/// non-linear resource in the same memory object, never applies between two live
/// allocations whose [`AllocationCreateDesc::linear`] was stated correctly.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllocationResourceClass {
    /// Allocations made with [`AllocationCreateDesc::linear`] set to `true`.
    Linear,
    /// Allocations made with [`AllocationCreateDesc::linear`] set to `false`.
    Optimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_description_states_its_arguments_and_asks_for_shared_memory_without_extras() {
        let requirements = vk::MemoryRequirements {
            size: 4096,
            alignment: 256,
            memory_type_bits: 0b1010,
        };
        let location = MemoryLocation::custom(
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_CACHED,
        );
        for linear in [true, false] {
            let desc = AllocationCreateDesc::new(requirements, location, linear);
            assert_eq!(desc.name, None, "name");
            assert_eq!(desc.requirements.size, 4096, "requirements.size");
            assert_eq!(desc.requirements.alignment, 256, "requirements.alignment");
            assert_eq!(
                desc.requirements.memory_type_bits, 0b1010,
                "requirements.memory_type_bits"
            );
            assert_eq!(desc.location, location, "location");
            assert_eq!(desc.linear, linear, "linear");
            assert_eq!(
                desc.memory_allocate_flags,
                vk::MemoryAllocateFlags::empty(),
                "memory_allocate_flags"
            );
            assert_eq!(
                desc.export_handle_types,
                vk::ExternalMemoryHandleTypeFlags::empty(),
                "export_handle_types"
            );
            assert_eq!(
                desc.allocation_scheme,
                AllocationScheme::GpuAllocatorManaged,
                "allocation_scheme"
            );
        }
    }

    #[test]
    fn a_new_description_can_be_made_in_a_constant() {
        const DESC: AllocationCreateDesc<'static> = AllocationCreateDesc::new(
            vk::MemoryRequirements {
                size: 64,
                alignment: 16,
                memory_type_bits: 1,
            },
            MemoryLocation::GpuOnly,
            true,
        );
        assert_eq!(DESC.requirements.size, 64);
        assert_eq!(DESC.location, MemoryLocation::GpuOnly);
    }
}
