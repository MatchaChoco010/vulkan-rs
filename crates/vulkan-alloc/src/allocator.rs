use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::{Device, vk};

use crate::allocation::{Allocation, AllocationCreateDesc, AllocationResourceClass};
use crate::block::{BlockRequest, MemoryBlock, MemoryTypeAllocator};
use crate::error::{AllocationError, FreeError};
use crate::mapping::MappedAllocation;
use crate::memory_type::{
    effective_alignment, find_memory_type_index, memory_heap, memory_type_flags,
};
use crate::report::{
    AllocatorSnapshot, AllocatorStats, LiveAllocation, snapshot_from_memory_types,
    stats_from_memory_types,
};

const DEFAULT_BLOCK_SIZE: u64 = 64 * 1024 * 1024;

/// The parameters of [`Allocator::new`].
///
/// [`AllocatorCreateDesc::new`] takes the memory properties and the
/// `nonCoherentAtomSize` limit that are passed to it and sets
/// [`AllocatorCreateDesc::block_size`] to 64 MiB. The fields are public and can be
/// changed afterwards.
pub struct AllocatorCreateDesc<'a> {
    /// The device to allocate from. The allocator keeps a clone of it.
    pub device: &'a Device,
    /// The memory properties of the physical device that [`AllocatorCreateDesc::device`]
    /// was created from. Memory types are chosen from them. A `memory_type_count` above
    /// [`vk::MAX_MEMORY_TYPES`] is treated as [`vk::MAX_MEMORY_TYPES`]. Only the heaps
    /// below `memory_heap_count` (and [`vk::MAX_MEMORY_HEAPS`]) are read, both to leave
    /// out the memory types of tile memory heaps and to limit the size of shared blocks
    /// (see [`AllocatorCreateDesc::block_size`]): a memory type whose `heap_index` is not
    /// below it has no heap.
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    /// The `nonCoherentAtomSize` limit of the physical device.
    ///
    /// Allocations in memory types that are `HOST_VISIBLE` but not `HOST_COHERENT` are
    /// aligned to it, and flushed and invalidated ranges are rounded to it. 0 is treated
    /// as 1.
    ///
    /// It must be a power of two, as Vulkan requires of this limit. Such an allocation is
    /// aligned to the larger of this value and the alignment of its requirements, which
    /// is a multiple of both when both are powers of two. With a value that is not a power
    /// of two, the offset of such an allocation need not be a multiple of the alignment
    /// its requirements ask for, nor of this value, and a flushed or invalidated range can
    /// reach into another allocation.
    pub non_coherent_atom_size: u64,
    /// The size in bytes of each shared `VkDeviceMemory` block. 0 is treated as 1.
    ///
    /// In a memory type whose heap is smaller than this, shared blocks have the size of
    /// the heap instead. The heap is the entry `heap_index` of the memory type in
    /// [`AllocatorCreateDesc::memory_properties`]; it is used only if `heap_index` is
    /// below `memory_heap_count` (and [`vk::MAX_MEMORY_HEAPS`]) and its `size` is not 0.
    /// An allocation larger than the resulting size gets a block of its own size.
    pub block_size: u64,
}

impl<'a> AllocatorCreateDesc<'a> {
    /// Describes an allocator for `device`, with
    /// [`AllocatorCreateDesc::non_coherent_atom_size`] taken from
    /// `physical_device_properties.limits` and a [`AllocatorCreateDesc::block_size`] of
    /// 64 MiB.
    ///
    /// `memory_properties` and `physical_device_properties` must be those of the
    /// physical device `device` was created from.
    pub fn new(
        device: &'a Device,
        memory_properties: vk::PhysicalDeviceMemoryProperties,
        physical_device_properties: vk::PhysicalDeviceProperties,
    ) -> Self {
        Self {
            device,
            memory_properties,
            non_coherent_atom_size: physical_device_properties.limits.non_coherent_atom_size,
            block_size: DEFAULT_BLOCK_SIZE,
        }
    }
}

/// Allocates `VkDeviceMemory` in blocks and hands out ranges of them.
///
/// See the [crate documentation](crate) for the flow of use, mapping, and the order of
/// destruction.
///
/// # Dropping
///
/// Dropping the allocator makes no Vulkan calls. If allocations are still live, it does
/// not panic: it logs an error through the [`log`] crate with the number of live
/// allocations and their total size, then one line per allocation with its name, size,
/// offset, memory handle and memory type. The `VkDeviceMemory` of those allocations is
/// not freed. Call [`Allocator::free_all`] before dropping to release it.
///
/// # Threads
///
/// The allocator is [`Send`] but not [`Sync`]. Wrap it in a
/// [`Mutex`](std::sync::Mutex) to use it from several threads.
pub struct Allocator {
    device: Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    non_coherent_atom_size: u64,
    block_size: u64,
    memory_types: Vec<MemoryTypeAllocator>,
}

unsafe impl Send for Allocator {}

impl Allocator {
    /// Creates an allocator. No memory is allocated until the first
    /// [`Allocator::allocate`], and no Vulkan call is made here.
    pub fn new(desc: AllocatorCreateDesc<'_>) -> Self {
        let mut memory_properties = desc.memory_properties;
        memory_properties.memory_type_count = memory_properties
            .memory_type_count
            .min(vk::MAX_MEMORY_TYPES as u32);
        let memory_types = (0..memory_properties.memory_type_count)
            .map(MemoryTypeAllocator::new)
            .collect();
        Self {
            device: desc.device.clone(),
            memory_properties,
            non_coherent_atom_size: desc.non_coherent_atom_size.max(1),
            block_size: desc.block_size.max(1),
            memory_types,
        }
    }

    /// Allocates a range of device memory for a resource.
    ///
    /// Create the resource, query its memory requirements, pass them here, then bind the
    /// returned [`Allocation`] to the resource yourself.
    ///
    /// The memory type is chosen among those allowed by
    /// `desc.requirements.memory_type_bits` that have every flag
    /// [`AllocationCreateDesc::location`] requires: the lowest-indexed one that also has
    /// every preferred flag, or else the one with the most preferred flags (the lowest
    /// index on a tie). Memory types with `DEVICE_COHERENT_AMD` or `DEVICE_UNCACHED_AMD`
    /// are left out unless the required or preferred flags of the location include one
    /// of these two flags, and memory types with `PROTECTED` are left out unless those
    /// flags include `PROTECTED`; memory types of a tile memory heap are always left out
    /// (see [`MemoryLocation`](crate::MemoryLocation)). Only the chosen memory type is
    /// tried.
    ///
    /// The allocation is placed in an existing shared block of that type when one has
    /// room and was made for the same [`AllocationCreateDesc::linear`] and
    /// [`AllocationCreateDesc::memory_allocate_flags`]. Otherwise a new block is
    /// allocated with `vkAllocateMemory`: the smaller of
    /// [`AllocatorCreateDesc::block_size`] and the size of the memory heap of the type,
    /// or the allocation's size if that is larger. Dedicated allocations, exportable
    /// allocations and allocations whose [`AllocationCreateDesc::memory_allocate_flags`]
    /// contain [`vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT`] always get a new
    /// `VkDeviceMemory` of exactly `desc.requirements.size` bytes.
    ///
    /// An existing block that the allocation may share counts as having room whenever
    /// one of its free ranges can hold the allocation at its alignment. Looking for that
    /// range can take time proportional to the number of free ranges, in the
    /// allocation's size class or a higher one, of each block that is tried (see the
    /// [crate documentation](crate#what-the-allocator-does-and-does-not-do)).
    ///
    /// # Errors
    ///
    /// - [`AllocationError::NoCompatibleMemoryType`] if no allowed memory type has the
    ///   required flags, not counting the types that are left out as described above.
    /// - [`AllocationError::OutOfMemory`] if `desc.requirements.size` is 0.
    /// - [`AllocationError::Vulkan`] if `vkAllocateMemory` fails, for example with
    ///   [`vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY`].
    ///
    /// Nothing is allocated when an error is returned.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed, and
    ///   [`AllocatorCreateDesc::memory_properties`] must be those of its physical
    ///   device.
    /// - The `vkAllocateMemory` call made from `desc` must be valid. In particular:
    ///   non-empty [`AllocationCreateDesc::memory_allocate_flags`] need the matching
    ///   device features (such as `bufferDeviceAddress` for `DEVICE_ADDRESS`); a
    ///   location that names `DEVICE_COHERENT_AMD` or `DEVICE_UNCACHED_AMD` needs the
    ///   `deviceCoherentMemory` feature; a location that names `PROTECTED` needs the
    ///   `protectedMemory` feature; non-empty
    ///   [`AllocationCreateDesc::export_handle_types`] need the external-memory
    ///   extensions for those handle types enabled and supported for the chosen memory
    ///   type; `desc.requirements.size` must not be larger than the memory heap of the
    ///   chosen type; and the number of `VkDeviceMemory` objects on the device must stay
    ///   below `maxMemoryAllocationCount`.
    /// - For [`AllocationScheme::DedicatedBuffer`](crate::AllocationScheme::DedicatedBuffer)
    ///   and [`AllocationScheme::DedicatedImage`](crate::AllocationScheme::DedicatedImage),
    ///   the handle must be a resource of this device that is not bound to memory yet,
    ///   `desc.requirements` must be its memory requirements, and the device must
    ///   support dedicated allocations (Vulkan 1.1 or `VK_KHR_dedicated_allocation`).
    /// - The returned allocation may only be bound to resources whose memory
    ///   requirements it satisfies.
    /// - The resources bound to the returned allocation must be protected resources if
    ///   its memory type has `PROTECTED`, and must not be if it does not
    ///   ([`Allocator::allocation_memory_flags`] tells). A location that only prefers
    ///   `PROTECTED` can be given either; require the flag for a protected resource.
    /// - Every resource bound to the returned allocation must be a linear resource if
    ///   [`AllocationCreateDesc::linear`] is `true` and a non-linear resource if it is
    ///   `false`. The allocator places allocations of the same kind next to each other
    ///   without the `bufferImageGranularity` padding that a resource of the other kind
    ///   needs, so a misstated kind makes the resource alias its neighbours. Memory
    ///   bound to other objects, such as a video session or a data graph pipeline
    ///   session, may come from an allocation of either kind.
    pub unsafe fn allocate(
        &mut self,
        desc: AllocationCreateDesc<'_>,
    ) -> Result<Allocation, AllocationError> {
        let resource_class = if desc.linear {
            AllocationResourceClass::Linear
        } else {
            AllocationResourceClass::Optimal
        };
        let memory_type_index = find_memory_type_index(
            &self.memory_properties,
            desc.requirements.memory_type_bits,
            desc.location.required_flags(),
            desc.location.preferred_flags(),
        )
        .ok_or(AllocationError::NoCompatibleMemoryType)?;
        let requirements = self.adjusted_requirements(memory_type_index, desc.requirements)?;
        let allocation = unsafe {
            self.allocate_from_type(
                memory_type_index,
                requirements,
                resource_class,
                BlockRequest {
                    dedicated: desc.allocation_scheme.into(),
                    memory_allocate_flags: desc.memory_allocate_flags,
                    export_handle_types: desc.export_handle_types,
                },
            )
        }?;
        if let Some(name) = desc.name
            && let Ok(block) = block_mut(&mut self.memory_types, &allocation)
        {
            block.set_name(allocation.allocation_id, name);
        }
        Ok(allocation)
    }

    /// Frees an allocation made by [`Allocator::allocate`].
    ///
    /// If the allocation was mapped with [`Allocator::map_persistent`], that mapping is
    /// released. If it was the last allocation of its block, the block's
    /// `VkDeviceMemory` is freed with `vkFreeMemory` before this returns.
    ///
    /// # Errors
    ///
    /// A [`FreeError`] whose [`FreeError::error`] is
    /// [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    /// allocation, or if it still has a mapping made with [`Allocator::map_memory`]
    /// that was not released with [`Allocator::unmap_memory`].
    ///
    /// Nothing is freed or unmapped when an error is returned, and
    /// [`FreeError::allocation`] hands the allocation back: release its
    /// [`Allocator::map_memory`] references with [`Allocator::unmap_memory`] and free it
    /// again, or pass it to the allocator that made it.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - The device must have finished every use of the memory: no command that
    ///   accesses a resource bound to the allocation may be pending, because the range
    ///   can be handed out again and its `VkDeviceMemory` can be freed.
    /// - Resources bound to the allocation, and pointers obtained by mapping it, must
    ///   not be used after it was freed.
    pub unsafe fn free(&mut self, allocation: Allocation) -> Result<(), FreeError> {
        let freed = match memory_type_mut(&mut self.memory_types, &allocation) {
            Ok(memory_type) => unsafe { memory_type.free(&self.device, &allocation) },
            Err(error) => Err(error),
        };
        freed.map_err(|error| FreeError { allocation, error })
    }

    /// Maps the allocation and returns a pointer to its first byte. The allocation has
    /// [`Allocation::size`] bytes.
    ///
    /// The whole `VkDeviceMemory` block is mapped the first time any allocation in it is
    /// mapped. Each successful call takes one mapping reference for the allocation, which
    /// one call to [`Allocator::unmap_memory`] releases. While the allocation holds such
    /// a reference, [`Allocator::free`] rejects it. [`Allocator::map`] is the form that
    /// cannot be left unpaired.
    ///
    /// # Errors
    ///
    /// - [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    ///   allocation, if its memory type is not `HOST_VISIBLE`, if `vkMapMemory` returns
    ///   a null pointer, or if the address of the allocation (the address of the mapped
    ///   block plus [`Allocation::offset`]) cannot be represented as a non-null pointer.
    /// - [`AllocationError::Vulkan`] if `vkMapMemory` fails.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - [`Allocation::memory`] must not be mapped or unmapped other than through this
    ///   allocator.
    /// - The pointer may only be used for the [`Allocation::size`] bytes it points to,
    ///   and only until the [`Allocator::unmap_memory`] call that releases its reference
    ///   or until [`Allocator::free_all`]. References are counted per allocation, so with
    ///   several references outstanding, the pointer of any of them must not be used
    ///   after the last one is released.
    /// - Host access through the pointer must follow Vulkan's rules for host access to
    ///   device memory: it must not race with device access or with other host access
    ///   to the same bytes.
    pub unsafe fn map_memory(
        &mut self,
        allocation: &Allocation,
    ) -> Result<*mut c_void, AllocationError> {
        Ok(unsafe { self.map_host_visible(allocation) }?
            .as_ptr()
            .cast())
    }

    /// Maps the allocation and returns a guard that unmaps it when dropped.
    ///
    /// The guard borrows the allocator mutably, so no other allocator call can be made
    /// while it is alive. Its methods give access to the [`Allocation::size`] bytes of
    /// the allocation.
    ///
    /// # Errors
    ///
    /// The same as [`Allocator::map_memory`], and
    /// [`AllocationError::InvalidAllocation`] if the allocation is larger than
    /// [`isize::MAX`] bytes, which no slice can cover. Nothing is mapped in that case;
    /// [`Allocator::map_memory`] has no such limit.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - [`Allocation::memory`] must not be mapped or unmapped other than through this
    ///   allocator.
    /// - While the guard is alive, neither the device nor a pointer from
    ///   [`Allocator::map_memory`], [`Allocator::map_persistent`] or
    ///   [`MappedAllocation::as_ptr`] may access the allocation's bytes in a way that
    ///   conflicts with the slices the guard hands out, including through its safe
    ///   methods: no writes while a shared slice is in use, and no access at all while a
    ///   mutable slice is in use.
    pub unsafe fn map<'b>(
        &'b mut self,
        allocation: &'b Allocation,
    ) -> Result<MappedAllocation<'b>, AllocationError> {
        let len = usize::try_from(allocation.size)
            .ok()
            .filter(|&len| isize::try_from(len).is_ok())
            .ok_or(AllocationError::InvalidAllocation)?;
        let ptr = unsafe { self.map_host_visible(allocation) }?;
        Ok(MappedAllocation::new(self, allocation, ptr, len))
    }

    /// Maps the allocation until it is freed and returns a pointer to its first byte. The
    /// allocation has [`Allocation::size`] bytes.
    ///
    /// The pointer borrows nothing, so it can be stored next to the resource it belongs
    /// to. It stays valid until [`Allocator::free`] frees the allocation, which
    /// releases the mapping, or until [`Allocator::free_all`]. Calling this again for
    /// the same allocation returns the same pointer without taking another reference.
    /// [`Allocator::unmap_memory`] does not release this mapping.
    ///
    /// # Errors
    ///
    /// The same as [`Allocator::map_memory`].
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - [`Allocation::memory`] must not be mapped or unmapped other than through this
    ///   allocator.
    /// - The pointer may only be used for the [`Allocation::size`] bytes it points to,
    ///   and not after the allocation is freed or [`Allocator::free_all`] is called.
    /// - Host access through the pointer must follow Vulkan's rules for host access to
    ///   device memory: it must not race with device access or with other host access
    ///   to the same bytes.
    pub unsafe fn map_persistent(
        &mut self,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        self.ensure_host_visible(allocation)?;
        let block = block_mut(&mut self.memory_types, allocation)?;
        unsafe { block.map_persistent(&self.device, allocation) }
    }

    /// The pointer [`Allocator::map_persistent`] returned for this allocation, or `None`
    /// if the allocation is not persistently mapped or the allocator does not hold it.
    pub fn persistent_ptr(&self, allocation: &Allocation) -> Option<NonNull<u8>> {
        self.block(allocation).ok()?.persistent_ptr(allocation)
    }

    /// Releases one mapping reference taken with [`Allocator::map_memory`]. When no
    /// allocation of the block holds a reference any more, the block is unmapped with
    /// `vkUnmapMemory`.
    ///
    /// The guard of [`Allocator::map`] releases its reference by itself, and the mapping
    /// of [`Allocator::map_persistent`] is released by [`Allocator::free`].
    ///
    /// # Errors
    ///
    /// [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    /// allocation, if the allocation is not mapped, or if the only reference it holds is
    /// its persistent mapping. Nothing is released in these cases.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - The call must pair with an earlier [`Allocator::map_memory`] call for this
    ///   allocation, and the pointer that call returned must not be used afterwards.
    pub unsafe fn unmap_memory(&mut self, allocation: &Allocation) -> Result<(), AllocationError> {
        let block = block_mut(&mut self.memory_types, allocation)?;
        unsafe { block.unmap(&self.device, allocation.allocation_id) }
    }

    /// The property flags of memory type `memory_type_index`, or `None` if the index is
    /// not below `memory_type_count` of [`AllocatorCreateDesc::memory_properties`] or
    /// not below [`vk::MAX_MEMORY_TYPES`].
    pub fn memory_type_flags(&self, memory_type_index: u32) -> Option<vk::MemoryPropertyFlags> {
        memory_type_flags(&self.memory_properties, memory_type_index)
    }

    /// The property flags of the memory type the allocation was made in. Check
    /// `HOST_COHERENT` here to know whether mapped writes and reads need
    /// [`Allocator::flush_allocation`] and [`Allocator::invalidate_allocation`].
    ///
    /// # Errors
    ///
    /// [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    /// allocation.
    pub fn allocation_memory_flags(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MemoryPropertyFlags, AllocationError> {
        self.block(allocation)?;
        self.memory_type_flags(allocation.memory_type_index())
            .ok_or(AllocationError::InvalidAllocation)
    }

    /// Flushes the host writes to a mapped allocation with `vkFlushMappedMemoryRanges`, so
    /// that the device can see them. Only memory types without `HOST_COHERENT` need it.
    ///
    /// The flushed range is the allocation, widened to multiples of
    /// [`AllocatorCreateDesc::non_coherent_atom_size`] (or to the end of the block).
    ///
    /// # Errors
    ///
    /// - [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    ///   allocation or the allocation is not mapped.
    /// - [`AllocationError::Vulkan`] if `vkFlushMappedMemoryRanges` fails.
    ///
    /// # Safety
    ///
    /// The device the allocator was created with must not have been destroyed.
    pub unsafe fn flush_allocation(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device.flush_mapped_memory_ranges(&[range]) }?)
    }

    /// Invalidates the host's view of a mapped allocation with
    /// `vkInvalidateMappedMemoryRanges`, so that the host can see device writes that were
    /// made available to the host. Only memory types without `HOST_COHERENT` need it.
    ///
    /// The invalidated range is the allocation, widened to multiples of
    /// [`AllocatorCreateDesc::non_coherent_atom_size`] (or to the end of the block).
    ///
    /// # Errors
    ///
    /// - [`AllocationError::InvalidAllocation`] if the allocator does not hold the
    ///   allocation or the allocation is not mapped.
    /// - [`AllocationError::Vulkan`] if `vkInvalidateMappedMemoryRanges` fails.
    ///
    /// # Safety
    ///
    /// The device the allocator was created with must not have been destroyed.
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &Allocation,
    ) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device.invalidate_mapped_memory_ranges(&[range]) }?)
    }

    /// Totals of the memory the allocator holds, overall and per memory type.
    pub fn stats(&self) -> AllocatorStats {
        stats_from_memory_types(&self.memory_types)
    }

    /// The totals of [`Allocator::stats`] together with the layout of every block: each
    /// free and allocated region, in offset order.
    pub fn snapshot(&self) -> AllocatorSnapshot {
        snapshot_from_memory_types(&self.memory_types, &self.memory_properties)
    }

    /// The number of allocations that have not been freed.
    pub fn live_allocation_count(&self) -> usize {
        self.stats().allocation_count
    }

    /// Whether every allocation has been freed. The allocator then holds no
    /// `VkDeviceMemory`.
    pub fn is_empty(&self) -> bool {
        self.live_allocation_count() == 0
    }

    /// Every allocation that has not been freed, with the name it was created with,
    /// ordered by memory type, then block, then offset.
    pub fn live_allocations(&self) -> Vec<LiveAllocation> {
        self.memory_types
            .iter()
            .flat_map(MemoryTypeAllocator::live_allocations)
            .collect()
    }

    /// Frees every `VkDeviceMemory` of the allocator, including blocks that still hold
    /// live allocations or are mapped, and returns how many allocations were still
    /// live. The allocator is empty afterwards and can be used again.
    ///
    /// The [`Allocation`] values that were live are not held by the allocator any more:
    /// every function that takes one rejects them, also after new allocations were made
    /// in memory with the same `VkDeviceMemory` handle, offset and size.
    ///
    /// This is the shutdown path for an owner that cannot guarantee that everything was
    /// freed: stop using the resources first, then call this before dropping the
    /// allocator and destroying the device.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - No command that accesses a resource bound to memory of this allocator may be
    ///   pending, and such resources must not be used afterwards.
    /// - No pointer obtained by mapping memory of this allocator may be used afterwards.
    pub unsafe fn free_all(&mut self) -> usize {
        let live = self.live_allocation_count();
        for memory_type in &mut self.memory_types {
            unsafe { memory_type.free_all_blocks(&self.device) };
        }
        live
    }

    unsafe fn allocate_from_type(
        &mut self,
        memory_type_index: u32,
        requirements: vk::MemoryRequirements,
        resource_class: AllocationResourceClass,
        request: BlockRequest,
    ) -> Result<Allocation, AllocationError> {
        if requirements.size == 0 {
            return Err(AllocationError::OutOfMemory);
        }
        let shared_block_size = self.shared_block_size(memory_type_index);
        let memory_type = self
            .memory_types
            .get_mut(memory_type_index as usize)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        if !request.needs_own_block()
            && let Some(allocation) = memory_type.allocate_from_existing_blocks(
                requirements,
                resource_class,
                request.memory_allocate_flags,
            )
        {
            return Ok(allocation);
        }

        let block_size = request.block_size(requirements.size, shared_block_size);
        let mut block = unsafe {
            MemoryBlock::allocate(
                &self.device,
                memory_type_index,
                block_size,
                resource_class,
                request,
            )
        }?;
        let Some(suballocation) = block.suballocate(requirements.size, requirements.alignment)
        else {
            unsafe { block.free_memory(&self.device) };
            return Err(AllocationError::OutOfMemory);
        };
        Ok(memory_type.insert_block(block, suballocation))
    }

    unsafe fn map_host_visible(
        &mut self,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        self.ensure_host_visible(allocation)?;
        let block = block_mut(&mut self.memory_types, allocation)?;
        unsafe { block.map(&self.device, allocation) }
    }

    fn ensure_host_visible(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        if self
            .allocation_memory_flags(allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        {
            Ok(())
        } else {
            Err(AllocationError::InvalidAllocation)
        }
    }

    fn block(&self, allocation: &Allocation) -> Result<&MemoryBlock, AllocationError> {
        self.memory_types
            .get(allocation.memory_type_index as usize)
            .ok_or(AllocationError::InvalidAllocation)?
            .block(allocation)
    }

    fn mapped_range(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MappedMemoryRange<'static>, AllocationError> {
        self.block(allocation)?
            .mapped_range(allocation, self.non_coherent_atom_size)
    }

    fn shared_block_size(&self, memory_type_index: u32) -> u64 {
        let heap_size = self
            .memory_properties
            .memory_types
            .get(memory_type_index as usize)
            .and_then(|memory_type| memory_heap(&self.memory_properties, memory_type.heap_index))
            .map(|heap| heap.size)
            .filter(|&size| size != 0);
        heap_size.map_or(self.block_size, |heap_size| self.block_size.min(heap_size))
    }

    fn adjusted_requirements(
        &self,
        memory_type_index: u32,
        mut requirements: vk::MemoryRequirements,
    ) -> Result<vk::MemoryRequirements, AllocationError> {
        let flags = self
            .memory_type_flags(memory_type_index)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        requirements.alignment =
            effective_alignment(flags, requirements.alignment, self.non_coherent_atom_size);
        Ok(requirements)
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        let live = self.live_allocations();
        if live.is_empty() {
            return;
        }
        let bytes = live.iter().fold(0u64, |bytes, allocation| {
            bytes.saturating_add(allocation.size)
        });
        log::error!(
            "vulkan_alloc::Allocator dropped with {} live allocation(s), {bytes} bytes; \
             their VkDeviceMemory is not freed",
            live.len()
        );
        for allocation in &live {
            log::error!(
                "  leaked {:?}: {} bytes at offset {} of {:?} (memory type {})",
                allocation.name.as_deref().unwrap_or("<unnamed>"),
                allocation.size,
                allocation.offset,
                allocation.memory,
                allocation.memory_type_index
            );
        }
    }
}

fn memory_type_mut<'a>(
    memory_types: &'a mut [MemoryTypeAllocator],
    allocation: &Allocation,
) -> Result<&'a mut MemoryTypeAllocator, AllocationError> {
    memory_types
        .get_mut(allocation.memory_type_index as usize)
        .ok_or(AllocationError::InvalidAllocation)
}

fn block_mut<'a>(
    memory_types: &'a mut [MemoryTypeAllocator],
    allocation: &Allocation,
) -> Result<&'a mut MemoryBlock, AllocationError> {
    memory_type_mut(memory_types, allocation)?.block_mut(allocation)
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;
    use crate::allocation::AllocationScheme;
    use crate::memory_type::MemoryLocation;

    unsafe extern "system" fn no_commands(
        _device: vk::Device,
        _name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        None
    }

    fn device_without_commands() -> Device {
        let handle = vk::Device::null();
        Device::from_raw(handle, unsafe {
            vk::DeviceFn::load(Some(no_commands), handle)
        })
    }

    static NEXT_MEMORY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

    unsafe extern "system" fn allocate_memory(
        _device: vk::Device,
        _p_allocate_info: *const vk::MemoryAllocateInfo<'_>,
        _p_allocator: *const vk::AllocationCallbacks<'_>,
        p_memory: *mut vk::DeviceMemory,
    ) -> vk::VkResult {
        let handle = NEXT_MEMORY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        unsafe { p_memory.write(vk::DeviceMemory(handle)) };
        vk::VkResult::SUCCESS
    }

    unsafe extern "system" fn free_memory(
        _device: vk::Device,
        _memory: vk::DeviceMemory,
        _p_allocator: *const vk::AllocationCallbacks<'_>,
    ) {
    }

    unsafe extern "system" fn map_memory(
        _device: vk::Device,
        _memory: vk::DeviceMemory,
        _offset: vk::DeviceSize,
        _size: vk::DeviceSize,
        _flags: vk::MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> vk::VkResult {
        unsafe { pp_data.write(NonNull::<u64>::dangling().as_ptr().cast()) };
        vk::VkResult::SUCCESS
    }

    unsafe extern "system" fn unmap_memory(_device: vk::Device, _memory: vk::DeviceMemory) {}

    unsafe extern "system" fn memory_commands(
        _device: vk::Device,
        name: *const c_char,
    ) -> vk::PFN_vkVoidFunction {
        type Void = unsafe extern "system" fn();
        unsafe {
            match std::ffi::CStr::from_ptr(name).to_bytes() {
                b"vkAllocateMemory" => Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        *const vk::MemoryAllocateInfo<'_>,
                        *const vk::AllocationCallbacks<'_>,
                        *mut vk::DeviceMemory,
                    ) -> vk::VkResult,
                    Void,
                >(allocate_memory)),
                b"vkFreeMemory" => Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        vk::DeviceMemory,
                        *const vk::AllocationCallbacks<'_>,
                    ),
                    Void,
                >(free_memory)),
                b"vkMapMemory" => Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        vk::DeviceMemory,
                        vk::DeviceSize,
                        vk::DeviceSize,
                        vk::MemoryMapFlags,
                        *mut *mut c_void,
                    ) -> vk::VkResult,
                    Void,
                >(map_memory)),
                b"vkUnmapMemory" => Some(std::mem::transmute::<
                    unsafe extern "system" fn(vk::Device, vk::DeviceMemory),
                    Void,
                >(unmap_memory)),
                _ => None,
            }
        }
    }

    fn device_with_memory_commands() -> Device {
        let handle = vk::Device::null();
        Device::from_raw(handle, unsafe {
            vk::DeviceFn::load(Some(memory_commands), handle)
        })
    }

    #[test]
    fn with_power_of_two_atoms_and_alignments_atom_rounded_ranges_never_reach_another_allocation() {
        let device = device_with_memory_commands();
        let mut memory_properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 1,
            memory_heap_count: 1,
            ..Default::default()
        };
        memory_properties.memory_types[0].property_flags =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED;
        let mut seed = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        let desc = |size: u64, alignment: u64| AllocationCreateDesc {
            name: None,
            requirements: vk::MemoryRequirements {
                size,
                alignment,
                memory_type_bits: 1,
            },
            location: MemoryLocation::GpuToCpu,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        };
        let mut checked_neighbours = 0;
        for round in 0..48 {
            let atom = match next() % 4 {
                0 => 0,
                _ => 1 << (next() % 13),
            };
            let block_size = [1000, 4096, 65536, 100_000][(next() % 4) as usize];
            let create = || {
                Allocator::new(AllocatorCreateDesc {
                    device: &device,
                    memory_properties,
                    non_coherent_atom_size: atom,
                    block_size,
                })
            };
            let mut allocator = create();
            let mut with_alignment_1 = create();
            let mut live: Vec<(Allocation, Allocation, u64)> = Vec::new();
            for step in 0..300 {
                let context = format!("round {round} step {step}, atom {atom}");
                if !live.is_empty() && (live.len() >= 40 || next() % 3 == 0) {
                    let index = (next() % live.len() as u64) as usize;
                    let (allocation, twin, _) = live.swap_remove(index);
                    unsafe {
                        allocator.unmap_memory(&allocation).unwrap();
                        allocator.free(allocation).unwrap();
                        with_alignment_1.free(twin).unwrap();
                    }
                } else {
                    let size = 1 + next() % 3000;
                    let alignment = match next() % 4 {
                        0 => 0,
                        _ => 1 << (next() % 11),
                    };
                    let (allocation, twin) = unsafe {
                        (
                            allocator.allocate(desc(size, alignment)).unwrap(),
                            with_alignment_1
                                .allocate(desc(size, alignment.max(1)))
                                .unwrap(),
                        )
                    };
                    assert_eq!(
                        (allocation.offset(), allocation.size()),
                        (twin.offset(), twin.size()),
                        "{context}: an alignment of 0 must place allocations as an alignment of 1 does"
                    );
                    unsafe { allocator.map_memory(&allocation) }.unwrap();
                    live.push((allocation, twin, alignment));
                }

                let snapshot = allocator.snapshot();
                let block_sizes = snapshot
                    .memory_types
                    .iter()
                    .flat_map(|memory_type| &memory_type.blocks)
                    .map(|block| (block.memory, block.size))
                    .collect::<std::collections::HashMap<_, _>>();
                for (allocation, _, alignment) in &live {
                    assert_eq!(
                        allocation.offset() % (*alignment).max(1),
                        0,
                        "{context}: the offset {} is not a multiple of the requested alignment {alignment}",
                        allocation.offset()
                    );
                    assert_eq!(
                        allocation.offset() % atom.max(1),
                        0,
                        "{context}: the offset {} is not a multiple of the atom",
                        allocation.offset()
                    );
                    let range = allocator.mapped_range(allocation).unwrap();
                    let end = if range.size == vk::WHOLE_SIZE {
                        block_sizes[&allocation.memory()]
                    } else {
                        range.offset + range.size
                    };
                    assert!(
                        range.offset <= allocation.offset()
                            && allocation.offset() + allocation.size() <= end,
                        "{context}: the range {}..{end} does not cover the allocation",
                        range.offset
                    );
                    for (other, _, _) in &live {
                        if other == allocation || other.memory() != allocation.memory() {
                            continue;
                        }
                        checked_neighbours += 1;
                        assert!(
                            end <= other.offset() || other.offset() + other.size() <= range.offset,
                            "{context}: the range {}..{end} of the allocation at {} reaches into the allocation at {}..{}",
                            range.offset,
                            allocation.offset(),
                            other.offset(),
                            other.offset() + other.size()
                        );
                    }
                }
            }
            for (allocation, twin, _) in live {
                unsafe {
                    allocator.unmap_memory(&allocation).unwrap();
                    allocator.free(allocation).unwrap();
                    with_alignment_1.free(twin).unwrap();
                }
            }
            assert!(allocator.is_empty() && with_alignment_1.is_empty());
        }
        assert!(checked_neighbours > 10_000);
    }

    #[test]
    fn a_memory_type_count_of_any_size_gives_at_most_the_types_vulkan_can_report() {
        let device = device_without_commands();
        for count in [0, 1, 3, 31, 32, 33, 1000, 1 << 20, u32::MAX] {
            let mut allocator = Allocator::new(AllocatorCreateDesc {
                device: &device,
                memory_properties: vk::PhysicalDeviceMemoryProperties {
                    memory_type_count: count,
                    ..Default::default()
                },
                non_coherent_atom_size: 1,
                block_size: 4096,
            });
            let expected = count.min(vk::MAX_MEMORY_TYPES as u32);
            assert_eq!(allocator.memory_types.len(), expected as usize, "{count}");
            for index in [0, 1, 2, 30, 31, 32, 33, 1000, u32::MAX] {
                assert_eq!(
                    allocator.memory_type_flags(index).is_some(),
                    index < expected,
                    "type {index} of {count}"
                );
            }
            let not_allowed = unsafe {
                allocator.allocate(AllocationCreateDesc {
                    name: None,
                    requirements: vk::MemoryRequirements {
                        size: 64,
                        alignment: 1,
                        memory_type_bits: 0,
                    },
                    location: MemoryLocation::GpuOnly,
                    linear: true,
                    memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
                    export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
                    allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                })
            };
            assert_eq!(
                not_allowed,
                Err(AllocationError::NoCompatibleMemoryType),
                "{count}"
            );
            assert!(allocator.snapshot().memory_types.is_empty(), "{count}");
        }
    }
}
