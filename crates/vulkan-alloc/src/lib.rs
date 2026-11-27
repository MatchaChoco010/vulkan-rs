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
//!    physical device it was created from. If the device was created with the
//!    `bufferDeviceAddress` feature, add `DEVICE_ADDRESS` to
//!    [`AllocatorCreateDesc::memory_allocate_flags`], so that buffers with and without a
//!    device address share blocks, and so that any allocation can back a buffer whose
//!    device address is taken.
//! 2. Create a buffer with [`Allocator::create_buffer`] or an image with
//!    [`Allocator::create_image`]. They take the create info of the resource and a
//!    [`ResourceAllocationDesc`], made with [`ResourceAllocationDesc::new`], which says
//!    where the memory goes. The [`MemoryLocation`] chooses the memory type from how the
//!    memory is used: [`MemoryLocation::GpuOnly`] for memory only the device accesses,
//!    [`MemoryLocation::CpuToGpu`] for memory the host writes and the device reads, such
//!    as the uniform buffer below and staging buffers, and [`MemoryLocation::GpuToCpu`]
//!    for memory the device writes and the host reads. The memory of the last two is
//!    host-visible and host-coherent, and [`ResourceAllocationDesc::new`] asks for it to
//!    be mapped, so that the host can reach it through [`Allocation::mapped_slice_mut`]
//!    and [`Allocation::mapped_slice`]. The function creates the resource, queries its
//!    memory requirements, allocates memory for it, and binds the resource to the
//!    memory. What the allocation needs besides the location, such as whether the
//!    resource is linear, whether it needs a dedicated allocation and whether its usage
//!    needs a device address, follows from the create info and the requirements.
//! 3. Access host-visible memory through the allocation, mapped as
//!    [`ResourceAllocationDesc::mapped`] says, or through one of the mapping functions
//!    (see [Mapping](#mapping)).
//! 4. When the device no longer uses the resource, pass it and its allocation to
//!    [`Allocator::destroy_buffer`] or [`Allocator::destroy_image`].
//! 5. Drop the allocator before the device is destroyed (see
//!    [Order of destruction](#order-of-destruction)).
//!
//! ```no_run
//! use vulkan::{Device, Instance, vk};
//! use vulkan_alloc::{Allocator, AllocatorCreateDesc, MemoryLocation, ResourceAllocationDesc};
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
//!         let (buffer, mut allocation) = allocator.create_buffer(
//!             &vk::BufferCreateInfo::default()
//!                 .size(data.len() as u64)
//!                 .usage(vk::BufferUsageFlags::UNIFORM_BUFFER),
//!             ResourceAllocationDesc {
//!                 name: Some("uniforms"),
//!                 ..ResourceAllocationDesc::new(MemoryLocation::CpuToGpu)
//!             },
//!         )?;
//!
//!         allocation
//!             .mapped_slice_mut()
//!             .ok_or("the allocation is not mapped")?
//!             .get_mut(..data.len())
//!             .ok_or("the allocation is smaller than the data")?
//!             .copy_from_slice(data);
//!         allocator.flush_allocation(&allocation)?;
//!
//!         allocator.destroy_buffer(buffer, allocation)?;
//!         drop(allocator);
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Resources that the caller creates
//!
//! [`Allocator::allocate`] allocates memory without creating anything, for a resource
//! that the caller creates and binds itself. Use it for the resources that
//! [`Allocator::create_buffer`] and [`Allocator::create_image`] refuse, such as sparse
//! buffers and images and disjoint images, for a resource bound with `vkBindBufferMemory2`
//! or `vkBindImageMemory2` and a structure in their chain, for memory bound to other
//! objects, such as acceleration structures of `VK_NV_ray_tracing`, video sessions and
//! tensors, and on a device whose Vulkan version is below 1.1:
//!
//! 1. Create the resource and query its memory requirements.
//! 2. Describe the memory in an [`AllocationCreateDesc`], made with
//!    [`AllocationCreateDesc::new`], and pass it to [`Allocator::allocate`]. The fields
//!    that [`Allocator::create_buffer`] and [`Allocator::create_image`] fill from the
//!    resource are the caller's to fill: [`AllocationCreateDesc::linear`],
//!    [`AllocationCreateDesc::allocation_scheme`] for a resource that needs or prefers a
//!    dedicated allocation, [`AllocationCreateDesc::export_handle_types`] and the
//!    `DEVICE_ADDRESS` in [`AllocationCreateDesc::memory_allocate_flags`] that some
//!    buffer usages need.
//! 3. Bind the resource to [`Allocation::memory`] at [`Allocation::offset`].
//! 4. When the device no longer uses the resource, destroy it and pass the allocation to
//!    [`Allocator::free`].
//!
//! ```no_run
//! use vulkan::{Device, vk};
//! use vulkan_alloc::{AllocationCreateDesc, Allocator, MemoryLocation};
//!
//! unsafe fn storage_buffer(
//!     allocator: &mut Allocator,
//!     device: &Device,
//! ) -> Result<(), Box<dyn std::error::Error>> {
//!     unsafe {
//!         let buffer = device.create_buffer(
//!             &vk::BufferCreateInfo::default()
//!                 .size(4096)
//!                 .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
//!             None,
//!         )?;
//!         let allocation = allocator.allocate(AllocationCreateDesc::new(
//!             device.get_buffer_memory_requirements(buffer),
//!             MemoryLocation::GpuOnly,
//!             true,
//!         ))?;
//!         device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;
//!
//!         device.destroy_buffer(buffer, None);
//!         allocator.free(allocation)?;
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
//! (or of the allocation's size, if that is larger). A memory type is out of memory for
//! the allocation if the allocation is larger than the memory heap of the type, which is
//! found without a `vkAllocateMemory` call whenever the memory properties give the size
//! of that heap, or if `vkAllocateMemory` returns `VK_ERROR_OUT_OF_DEVICE_MEMORY`. In the
//! second case the allocator first frees the empty blocks it keeps in the memory types of
//! that heap, if there are any, and calls `vkAllocateMemory` for the type once more. When
//! a memory type is out of memory, the allocator picks the next one among the remaining
//! types by the same rule and tries it, and returns [`AllocationError::OutOfMemory`] only
//! when every memory type the allocation may use is out of memory. So an allocation can
//! end up in a memory type that its [`MemoryLocation`] ranks lower than the first one
//! picked. A description the allocator does not accept, such as one of size 0, is
//! refused with [`AllocationError::InvalidAllocationCreateDesc`] before anything else is
//! checked.
//! Allocations that differ in the flags of the allocation (the union of
//! [`AllocationCreateDesc::memory_allocate_flags`] and
//! [`AllocatorCreateDesc::memory_allocate_flags`]) never share a block, and allocations
//! that differ in [`AllocationCreateDesc::linear`] are never in a block at the same time.
//! Dedicated allocations ([`AllocationScheme::DedicatedBuffer`],
//! [`AllocationScheme::DedicatedImage`]), exportable allocations
//! ([`AllocationCreateDesc::export_handle_types`]) and zero-initialized allocations
//! ([`vulkan::vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`] in those flags) get a
//! `VkDeviceMemory` of their own.
//!
//! When the last allocation of a shared block is freed, the block is kept empty if it
//! has the size of a new shared block of its memory type and the type has no other
//! empty block with the same flags of the allocation. An
//! allocation of that type with those flags that fits in no block holding allocations
//! of its kind is placed in the empty block, whatever its
//! [`AllocationCreateDesc::linear`]. This spares a `vkAllocateMemory` call each time a
//! program allocates and frees one allocation, such as a staging buffer, in a memory
//! type that holds nothing else. Other blocks are freed when their last allocation is
//! freed (see [`Allocator::free`]). A kept block still takes room in its memory heap,
//! which other allocations from that heap may need. When `vkAllocateMemory` runs out
//! of memory in a heap, the empty blocks of that heap are freed and the allocation is
//! tried again, as described above, so the room is given back when it is needed.
//! [`Allocator::free_empty_blocks`] frees the empty blocks at once, and dropping the
//! allocator frees all of its memory.
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
//! - create, bind or destroy resources other than buffers and images, nor buffers and
//!   images that [`Allocator::create_buffer`] and [`Allocator::create_image`] refuse;
//! - know when the device has finished with memory: [`Allocator::free`] releases memory
//!   immediately, so the caller waits for the device first;
//! - check memory heap budgets or `maxMemoryAllocationCount`;
//! - move allocations to reduce fragmentation;
//! - import memory. Memory whose size and layout are decided elsewhere, such as a
//!   dma-buf received from another process, is allocated with `vkAllocateMemory`
//!   directly. Memory to be exported can come from the allocator.
//! - allocate tile memory (`VK_QCOM_tile_memory_heap`), which has requirements of its
//!   own and is also allocated with `vkAllocateMemory` directly;
//! - allocate memory on chosen physical devices of a logical device that represents more
//!   than one physical device. The allocator cannot pass the `deviceMask` of
//!   `VkMemoryAllocateFlagsInfo`, so [`vulkan::vk::MemoryAllocateFlags::DEVICE_MASK`]
//!   is refused with [`AllocationError::InvalidAllocationCreateDesc`], and memory on
//!   chosen physical devices is allocated with `vkAllocateMemory` directly. Other
//!   allocations on such a device are made as on any device, but memory from a heap
//!   with `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` cannot be mapped (see [Mapping](#mapping)).
//!
//! # Mapping
//!
//! Memory that the host writes or reads, such as uniform, staging and readback buffers,
//! usually stays mapped from its allocation to its free, and [`Allocator::allocate`] maps
//! an allocation before it returns it when [`AllocationCreateDesc::mapped`] is `true`.
//! [`Allocation::mapped_slice_mut`] and [`Allocation::mapped_slice`] then give its bytes,
//! and [`Allocation::mapped_ptr`] a pointer to them, without borrowing the allocator, so
//! several allocations can be accessed at once. Flush after host writes and invalidate
//! before host reads as described below. [`Allocator::free`] releases the mapping. The
//! memory of [`MemoryLocation::CpuToGpu`] and [`MemoryLocation::GpuToCpu`] is always
//! host-visible, so it is always mapped; for other locations, memory that is not
//! host-visible is returned unmapped.
//!
//! The other ways to map an allocation of host-visible memory:
//!
//! - [`Allocator::map_persistent`] maps an allocation that was made without
//!   [`AllocationCreateDesc::mapped`] until it is freed, and leaves it in the same state
//!   as if it had been made with it.
//! - [`Allocator::map`] returns a [`MappedAllocation`] guard that unmaps when dropped.
//!   The guard borrows the allocator mutably, so it suits short accesses to memory that
//!   does not stay mapped, one allocation at a time.
//! - [`Allocator::map_memory`] returns a raw pointer; each successful call is paired with
//!   one [`Allocator::unmap_memory`]. It suits mappings whose lifetime the caller
//!   manages itself, without a borrow of the allocator.
//!
//! All of them map the whole `VkDeviceMemory` block once and count mapping references
//! per allocation; the block is unmapped when the last reference in it is released. A
//! persistent mapping and the others can be used together. Do not call `vkMapMemory` or
//! `vkUnmapMemory` on [`Allocation::memory`] yourself.
//!
//! Each error names its cause. The mapping functions return
//! [`AllocationError::NotHostVisible`] for memory that is not host-visible,
//! [`Allocator::unmap_memory`] and the flush and invalidate functions return
//! [`AllocationError::NotMapped`] for an allocation without the mapping they need, and
//! [`Allocator::free`] returns [`AllocationError::StillMapped`] for an allocation that
//! still holds references taken with [`Allocator::map_memory`]. Every function that takes
//! an allocation returns [`AllocationError::InvalidAllocation`] for one that the allocator
//! does not hold, before it checks anything else.
//!
//! On a logical device that represents more than one physical device, memory from a
//! heap with `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` has an instance on every physical
//! device, and Vulkan does not allow memory with more than one instance to be mapped.
//! Such allocations can be made and bound to resources, but must not be made with
//! [`AllocationCreateDesc::mapped`] nor passed to any of the mapping functions. The
//! allocator does not check this. The device-local heaps of such a device usually have
//! that flag. [`MemoryLocation::CpuToGpu`] prefers device-local memory, and
//! [`MemoryLocation::GpuToCpu`] can give it too, for example when the memory it ranks
//! higher is out of memory (see [`Allocator::allocate`]). For an allocation that will be mapped on
//! such a device, allow in
//! [`vk::MemoryRequirements::memory_type_bits`](vulkan::vk::MemoryRequirements::memory_type_bits)
//! only the memory types of heaps without `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`.
//!
//! Memory of [`MemoryLocation::CpuToGpu`] and [`MemoryLocation::GpuToCpu`] is always
//! `HOST_COHERENT`, so mapped writes and reads need neither a flush nor an invalidate
//! (see [`MemoryLocation`]).
//! [`MemoryLocation::GpuOnly`] and a [`MemoryLocation::Custom`] location that does not
//! require `HOST_COHERENT` can give host-visible memory without it. Such memory needs
//! [`Allocator::flush_allocation`] after host writes and
//! [`Allocator::invalidate_allocation`] before host reads (or
//! [`MappedAllocation::flush`] and [`MappedAllocation::invalidate`]). These functions do
//! nothing for `HOST_COHERENT` memory, so the memory type need not be checked: call the
//! flush after every host write and the invalidate before every host read.
//! [`Allocator::flush_allocation_range`] and [`Allocator::invalidate_allocation_range`]
//! (or [`MappedAllocation::flush_range`] and [`MappedAllocation::invalidate_range`]) do
//! the same for a range of bytes of the allocation. Allocations in memory that is not
//! `HOST_COHERENT` are aligned to `nonCoherentAtomSize`, so the atom-rounded range that
//! is flushed or invalidated never reaches into another allocation. This holds because
//! `nonCoherentAtomSize` and the alignments of memory requirements are powers of two:
//! [`AllocatorCreateDesc::non_coherent_atom_size`] requires it of the limit, and
//! [`Allocator::allocate`] rejects any other alignment than 0, which is treated as 1
//! (see [`AllocationCreateDesc::requirements`]).
//!
//! # Order of destruction
//!
//! The allocator keeps a clone of the [`vulkan::Device`]. A `Device` is a handle and a
//! function table, not an owner: nothing stops the device from being destroyed while the
//! allocator is alive, so the caller keeps this order:
//!
//! 1. Wait until the device no longer uses the buffers and images.
//! 2. Pass them with their allocations to [`Allocator::destroy_buffer`] and
//!    [`Allocator::destroy_image`], or destroy them and pass their allocations to
//!    [`Allocator::free`] or call [`Allocator::free_all`].
//! 3. Drop the allocator.
//! 4. Destroy the device.
//!
//! Dropping the allocator frees all of its `VkDeviceMemory` with `vkFreeMemory`: the
//! empty blocks it keeps (see [`Allocator::free`]) and the memory of allocations that
//! were not freed. So it is dropped before the device is destroyed, once every
//! submitted command that refers to that memory has completed; the safety section of
//! [`Allocator::allocate`] requires this. An allocator that holds no block
//! ([`Allocator::stats`] gives a `block_count` of 0), for example because it never
//! allocated or because [`Allocator::free_all`] was called after its last allocation,
//! makes no Vulkan call when dropped, and may be dropped after the device was
//! destroyed. Allocating, freeing, mapping, unmapping, flushing or invalidating after
//! that is not allowed.
//!
//! This holds for a drop during unwinding too. If code panics between submitting
//! commands that refer to the allocator's memory and waiting for them, unwinding drops
//! the allocator, and with it frees memory that pending commands refer to. Keep the
//! allocator from being dropped first in that case, for example:
//!
//! - wait for the device to become idle in the `Drop` of a guard that is dropped before
//!   the allocator: a local variable declared after the allocator, or a struct field
//!   declared before it;
//! - or keep the allocator in a [`ManuallyDrop`](std::mem::ManuallyDrop) and drop it
//!   yourself once the device has finished.
//!
//! # Leaks
//!
//! [`Allocation`] has no destructor: dropping one without passing it to
//! [`Allocator::free`] leaves its range allocated, and the block that holds it stays
//! allocated until [`Allocator::free_all`] or until the allocator is dropped.
//!
//! Dropping an [`Allocator`] that still has live allocations does not panic. It logs an
//! error through the [`log`] crate that lists each live allocation with its
//! [`AllocationCreateDesc::name`], then frees their `VkDeviceMemory` with the rest of
//! its memory. Every submitted command that refers to that memory must have completed,
//! and the resources bound to it must not be used afterwards (see the safety section of
//! [`Allocator::allocate`]).
//!
//! # Threads
//!
//! [`Allocator`] is [`Send`] but not [`Sync`]. Allocating, freeing and mapping take
//! `&mut self`; to share one allocator between threads, put it in a
//! [`Mutex`](std::sync::Mutex). [`Allocation`] is [`Send`] and [`Sync`]: besides plain
//! data it holds only the pointer of [`Allocation::mapped_ptr`], which means the same on
//! every thread and gives access to the bytes only through `unsafe` code.
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
mod resource;
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
pub use resource::ResourceAllocationDesc;
