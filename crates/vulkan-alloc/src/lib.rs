//! A device-memory allocator for the [`vulkan`] crate.
//!
//! Vulkan limits how many `VkDeviceMemory` objects may exist at once
//! (`maxMemoryAllocationCount`, which the specification only guarantees to be at least
//! 4096), so buffers and images are normally bound to ranges of larger memory objects.
//! [`Allocator`] allocates `VkDeviceMemory` in blocks, keeps the blocks of each memory
//! type apart, and cuts allocations out of them with TLSF (two-level segregated fit)
//! free lists, which it searches further than TLSF does when that is needed to place an
//! allocation (see
//! [What the allocator does and does not do](#what-the-allocator-does-and-does-not-do)).
//!
//! # Use
//!
//! 1. Create an [`Allocator`] from an [`AllocatorCreateDesc`]. [`AllocatorCreateDesc::new`]
//!    takes the [`vulkan::Device`] and the memory properties and properties of the
//!    physical device it was created from.
//! 2. Create a buffer or an image and query its memory requirements.
//! 3. Describe the memory in an [`AllocationCreateDesc`] and pass it to
//!    [`Allocator::allocate`]. The [`MemoryLocation`] chooses the memory type.
//! 4. Bind the resource to [`Allocation::memory`] at [`Allocation::offset`]. The
//!    allocator does not bind anything itself.
//! 5. Access host-visible memory through one of the mapping functions (see
//!    [Mapping](#mapping)).
//! 6. When the device no longer uses the resource, destroy it and pass the allocation to
//!    [`Allocator::free`].
//! 7. Drop the allocator before the device is destroyed (see
//!    [Order of destruction](#order-of-destruction)).
//!
//! ```no_run
//! use vulkan::{Device, Instance, vk};
//! use vulkan_alloc::{
//!     AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc, MemoryLocation,
//! };
//!
//! unsafe fn upload(
//!     instance: &Instance,
//!     physical_device: vk::PhysicalDevice,
//!     device: &Device,
//!     data: &[u8],
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     unsafe {
//!         let mut allocator = Allocator::new(AllocatorCreateDesc::new(
//!             device,
//!             instance.get_physical_device_memory_properties(physical_device),
//!             instance.get_physical_device_properties(physical_device),
//!         ));
//!
//!         let buffer = device.create_buffer(
//!             &vk::BufferCreateInfo::default()
//!                 .size(data.len() as u64)
//!                 .usage(vk::BufferUsageFlags::UNIFORM_BUFFER),
//!             None,
//!         )?;
//!         let allocation = allocator.allocate(AllocationCreateDesc {
//!             name: Some("uniforms"),
//!             requirements: device.get_buffer_memory_requirements(buffer),
//!             location: MemoryLocation::CpuToGpu,
//!             linear: true,
//!             memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
//!             export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
//!             allocation_scheme: AllocationScheme::GpuAllocatorManaged,
//!         })?;
//!         device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;
//!
//!         let coherent = allocator
//!             .allocation_memory_flags(&allocation)?
//!             .contains(vk::MemoryPropertyFlags::HOST_COHERENT);
//!         {
//!             let mut mapped = allocator.map(&allocation)?;
//!             mapped
//!                 .byte_range_mut(0, data.len())
//!                 .ok_or("the allocation is smaller than the data")?
//!                 .copy_from_slice(data);
//!             if !coherent {
//!                 mapped.flush()?;
//!             }
//!         }
//!
//!         device.destroy_buffer(buffer, None);
//!         allocator.free(allocation)?;
//!         drop(allocator);
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # What the allocator does and does not do
//!
//! [`Allocator::allocate`] picks a memory type, then places the allocation in an existing
//! block of that type that has room and is compatible, or allocates a new block of the
//! smaller of [`AllocatorCreateDesc::block_size`] and the size of the type's memory heap
//! (or of the allocation's size, if that is larger). Allocations that differ in
//! [`AllocationCreateDesc::linear`] or [`AllocationCreateDesc::memory_allocate_flags`]
//! never share a block. Dedicated allocations ([`AllocationScheme::DedicatedBuffer`],
//! [`AllocationScheme::DedicatedImage`]), exportable allocations
//! ([`AllocationCreateDesc::export_handle_types`]) and zero-initialized allocations
//! ([`vulkan::vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`] in
//! [`AllocationCreateDesc::memory_allocate_flags`]) get a `VkDeviceMemory` of their own.
//! When the last allocation of a block is freed, the block's `VkDeviceMemory` is freed
//! at once.
//!
//! The free ranges of a block are kept in lists by size class. A request first looks
//! only at the first free range of each list, from the class of the requested size
//! upward, which takes at most one step for each size class, however many free ranges
//! the block has. If none of these ranges can hold the allocation at its alignment, the
//! request walks the rest of those lists. So, unlike plain TLSF, which can fail while a
//! large enough free range exists, a request is placed in a block it may share whenever
//! one of the block's free ranges can hold it at its alignment.
//!
//! The price is that a request for which no first range of a list will do, in
//! particular one that does not fit in the block at all, can take time proportional to
//! the number of free ranges of the block in the request's size class or a higher one.
//! [`Allocator::allocate`] tries the blocks that the allocation may share in turn before
//! it allocates a new one, so such a request pays this for each of them. The padding
//! left before an allocation to align it stays a free range however small it is. Small
//! ranges are in low size classes, so they add to this time only for requests whose
//! size class is not above theirs.
//!
//! The allocator does not:
//!
//! - create, bind or destroy buffers and images;
//! - know when the device has finished with memory: [`Allocator::free`] releases memory
//!   immediately, so the caller waits for the device first;
//! - try another memory type when `vkAllocateMemory` fails for the chosen one;
//! - check memory heap budgets or `maxMemoryAllocationCount`;
//! - move allocations to reduce fragmentation;
//! - import memory. Memory whose size and layout are decided elsewhere, such as a
//!   dma-buf received from another process, is allocated with `vkAllocateMemory`
//!   directly. Memory to be exported can come from the allocator.
//! - allocate tile memory (`VK_QCOM_tile_memory_heap`), which has requirements of its
//!   own and is also allocated with `vkAllocateMemory` directly.
//!
//! # Mapping
//!
//! There are three ways to map an allocation of host-visible memory:
//!
//! - [`Allocator::map`] returns a [`MappedAllocation`] guard that unmaps when dropped.
//!   The guard borrows the allocator mutably, so it suits short accesses.
//! - [`Allocator::map_persistent`] maps the allocation until it is freed and returns a
//!   pointer that borrows nothing, for memory that stays mapped for its whole life.
//!   [`Allocator::free`] releases this mapping.
//! - [`Allocator::map_memory`] returns a raw pointer; each successful call is paired with
//!   one [`Allocator::unmap_memory`].
//!
//! All three map the whole `VkDeviceMemory` block once and count mapping references per
//! allocation; the block is unmapped when the last reference in it is released. Do not
//! call `vkMapMemory` or `vkUnmapMemory` on [`Allocation::memory`] yourself.
//!
//! Memory without `HOST_COHERENT` needs [`Allocator::flush_allocation`] after host
//! writes and [`Allocator::invalidate_allocation`] before host reads (or
//! [`MappedAllocation::flush`] and [`MappedAllocation::invalidate`]). Allocations in
//! such memory are aligned to `nonCoherentAtomSize`, so the atom-rounded range that is
//! flushed or invalidated never reaches into another allocation. This holds because
//! `nonCoherentAtomSize` and the alignments of memory requirements are powers of two,
//! which [`AllocatorCreateDesc::non_coherent_atom_size`] and
//! [`AllocationCreateDesc::requirements`] require.
//!
//! # Order of destruction
//!
//! The allocator keeps a clone of the [`vulkan::Device`]. A `Device` is a handle and a
//! function table, not an owner: nothing stops the device from being destroyed while the
//! allocator is alive, so the caller keeps this order:
//!
//! 1. Wait until the device no longer uses the buffers and images, then destroy them.
//! 2. Pass their allocations to [`Allocator::free`], or call [`Allocator::free_all`].
//! 3. Drop the allocator.
//! 4. Destroy the device.
//!
//! Dropping the allocator makes no Vulkan calls, so dropping it after the device was
//! destroyed is harmless. Allocating, freeing, mapping, unmapping, flushing or
//! invalidating after that is not.
//!
//! # Leaks
//!
//! [`Allocation`] has no destructor: dropping one without passing it to
//! [`Allocator::free`] leaves its range allocated, and the block that holds it stays
//! allocated until [`Allocator::free_all`].
//!
//! Dropping an [`Allocator`] that still has live allocations does not panic. It logs an
//! error through the [`log`] crate that lists each live allocation with its
//! [`AllocationCreateDesc::name`], and does not free the `VkDeviceMemory` blocks, which
//! may still back resources in use. Vulkan requires every `VkDeviceMemory` to be freed
//! before the device is destroyed: call [`Allocator::free_all`] once the resources are no
//! longer in use, or free the distinct [`LiveAllocation::memory`] handles reported by
//! [`Allocator::live_allocations`] yourself.
//!
//! # Threads
//!
//! [`Allocator`] is [`Send`] but not [`Sync`]. Allocating, freeing and mapping take
//! `&mut self`; to share one allocator between threads, put it in a
//! [`Mutex`](std::sync::Mutex). [`Allocation`] holds only plain data and is [`Send`] and
//! [`Sync`].
//!
//! # Descriptor heaps
//!
//! [`descriptor_heap`] computes the byte layout of `VK_EXT_descriptor_heap` heaps: where
//! each descriptor goes and where the range reserved for the implementation sits.

#![warn(missing_docs)]

mod align;
mod allocation;
mod allocator;
mod block;
pub mod descriptor_heap;
mod error;
mod mapping;
mod memory_type;
mod report;
mod suballocator;

pub use allocation::{Allocation, AllocationCreateDesc, AllocationResourceClass, AllocationScheme};
pub use allocator::{Allocator, AllocatorCreateDesc};
pub use error::{AllocationError, FreeError};
pub use mapping::MappedAllocation;
pub use memory_type::MemoryLocation;
pub use report::{
    AllocatorSnapshot, AllocatorStats, LiveAllocation, MemoryBlockSnapshot, MemoryRegionSnapshot,
    MemoryRegionState, MemoryTypeSnapshot, MemoryTypeStats,
};
