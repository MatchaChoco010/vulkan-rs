use std::fmt;

use vulkan::vk;

use crate::memory_type::MemoryLocation;

/// What [`Allocator::allocate`](crate::Allocator::allocate) is asked to allocate.
///
/// Every field is public and there is no constructor; struct update syntax
/// (`..desc`) helps to vary one field of a common description.
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
    /// [`AllocationError::OutOfMemory`](crate::AllocationError::OutOfMemory).
    ///
    /// The allocation's offset is a multiple of `alignment`. A caller may raise
    /// `alignment`, for example to align a device address, but must keep it a power of
    /// two, as Vulkan reports it: the allocator combines it with other alignments (see
    /// [`AllocatorCreateDesc::non_coherent_atom_size`](crate::AllocatorCreateDesc::non_coherent_atom_size))
    /// by taking the largest.
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
    /// Linear and non-linear allocations never share a `VkDeviceMemory`, which keeps
    /// them out of each other's `bufferImageGranularity` pages. Binding a resource of
    /// the other kind to the allocation breaks that; see the safety section of
    /// [`Allocator::allocate`](crate::Allocator::allocate).
    pub linear: bool,
    /// Flags for `VkMemoryAllocateFlagsInfo`, chained to the `vkAllocateMemory` call
    /// when not empty. A buffer created with
    /// [`vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS`] needs
    /// [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`] here.
    ///
    /// Allocations with different flags never share a block.
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    /// Handle types the memory can be exported as (`VkExportMemoryAllocateInfo`), such
    /// as `DMA_BUF_EXT`. When not empty, the allocation gets a `VkDeviceMemory` of its
    /// own of exactly `requirements.size` bytes, at offset 0, because an exported handle
    /// refers to the whole memory object.
    pub export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    /// Whether the allocation is suballocated or gets dedicated memory.
    pub allocation_scheme: AllocationScheme,
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
    /// [`AllocationCreateDesc::export_handle_types`] is not empty.
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
    pub(crate) block_index: usize,
    pub(crate) allocation_id: u64,
    pub(crate) resource_class: AllocationResourceClass,
    pub(crate) memory: vk::DeviceMemory,
    pub(crate) offset: u64,
    pub(crate) size: u64,
}

impl Allocation {
    /// The `VkDeviceMemory` that holds the range. Other allocations may share it unless
    /// this one is dedicated or exportable.
    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    /// The byte offset of the range in [`Allocation::memory`]. It is a multiple of the
    /// alignment of the requirements the allocation was made with.
    pub fn offset(&self) -> u64 {
        self.offset
    }

    /// The size of the range in bytes. It is at least the requested size and can be
    /// larger, when a remainder too small to be used for anything else was added to the
    /// range.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// The index of the memory type of [`Allocation::memory`] in
    /// [`vk::PhysicalDeviceMemoryProperties::memory_types`].
    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }
}

/// The kind of resource a block holds, as reported by
/// [`MemoryBlockSnapshot::resource_class`](crate::MemoryBlockSnapshot::resource_class).
///
/// Allocations of different classes never share a `VkDeviceMemory`, so the
/// `bufferImageGranularity` limit, which applies between a linear and a non-linear
/// resource in the same memory object, never applies between two allocations whose
/// [`AllocationCreateDesc::linear`] was stated correctly.
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
    fn raw_allocation_desc_can_request_dedicated_resources() {
        let requirements = vk::MemoryRequirements {
            size: 1024,
            alignment: 256,
            memory_type_bits: 1,
        };

        let buffer_desc = AllocationCreateDesc {
            name: Some("buffer"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedBuffer(vk::Buffer(7)),
        };
        assert_eq!(
            buffer_desc.allocation_scheme,
            AllocationScheme::DedicatedBuffer(vk::Buffer(7))
        );
        assert!(buffer_desc.linear);

        let image_desc = AllocationCreateDesc {
            name: Some("image"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: false,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedImage(vk::Image(9)),
        };
        assert_eq!(
            image_desc.allocation_scheme,
            AllocationScheme::DedicatedImage(vk::Image(9))
        );
        assert!(!image_desc.linear);
    }
}
