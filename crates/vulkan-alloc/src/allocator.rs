use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::{Device, vk};

use crate::allocation::{Allocation, AllocationCreateDesc, AllocationResourceClass};
use crate::block::{BlockRequest, DedicatedResource, MemoryBlock, MemoryTypeAllocator};
use crate::error::AllocationError;
use crate::mapping::{MappedAllocation, mapped_range_bounds};
use crate::memory_type::{effective_alignment, find_memory_type_index};
use crate::report::{
    AllocatorSnapshot, AllocatorStats, LiveAllocation, MemoryRegionState,
    snapshot_from_memory_types, stats_from_memory_types,
};

const DEFAULT_BLOCK_SIZE: u64 = 64 * 1024 * 1024;

pub struct AllocatorCreateDesc<'a> {
    pub device: &'a Device,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub non_coherent_atom_size: u64,
    pub buffer_image_granularity: u64,
    pub block_size: u64,
}

impl<'a> AllocatorCreateDesc<'a> {
    pub fn new(
        device: &'a Device,
        memory_properties: vk::PhysicalDeviceMemoryProperties,
        physical_device_properties: vk::PhysicalDeviceProperties,
    ) -> Self {
        Self {
            device,
            memory_properties,
            non_coherent_atom_size: physical_device_properties.limits.non_coherent_atom_size,
            buffer_image_granularity: physical_device_properties.limits.buffer_image_granularity,
            block_size: DEFAULT_BLOCK_SIZE,
        }
    }
}

unsafe impl Send for Allocator {}

pub struct Allocator {
    device: Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    non_coherent_atom_size: u64,
    buffer_image_granularity: u64,
    block_size: u64,
    memory_types: Vec<MemoryTypeAllocator>,
}

impl Allocator {
    pub fn new(desc: AllocatorCreateDesc<'_>) -> Self {
        let memory_types = (0..desc.memory_properties.memory_type_count)
            .map(|index| MemoryTypeAllocator {
                memory_type_index: index,
                blocks: Vec::new(),
            })
            .collect();
        Self {
            device: desc.device.clone(),
            memory_properties: desc.memory_properties,
            non_coherent_atom_size: desc.non_coherent_atom_size.max(1),
            buffer_image_granularity: desc.buffer_image_granularity.max(1),
            block_size: desc.block_size.max(1),
            memory_types,
        }
    }

    fn device(&self) -> Device {
        self.device.clone()
    }

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
        let requirements =
            self.adjusted_requirements(memory_type_index, desc.requirements, resource_class)?;
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
            && let Ok(block) = self.block_mut(&allocation)
        {
            block.names.insert(allocation.allocation_id, name.into());
        }
        Ok(allocation)
    }

    pub unsafe fn free(&mut self, allocation: Allocation) -> Result<(), AllocationError> {
        let device = self.device();
        let memory = {
            let memory_type = self
                .memory_types
                .get_mut(allocation.memory_type_index as usize)
                .ok_or(AllocationError::InvalidAllocation)?;
            let slot = memory_type
                .blocks
                .get_mut(allocation.block_index)
                .ok_or(AllocationError::InvalidAllocation)?;
            let block = slot.as_mut().ok_or(AllocationError::InvalidAllocation)?;
            if block.memory != allocation.memory {
                return Err(AllocationError::InvalidAllocation);
            }
            block.validate_allocation(&allocation)?;
            if block.persistent.remove(&allocation.allocation_id) {
                block.unmark_mapped(allocation.allocation_id)?;
                block.map_count -= 1;
                if block.map_count == 0 {
                    unsafe { device.unmap_memory(block.memory) };
                    block.mapped_ptr = None;
                }
            }
            if block.is_allocation_mapped(allocation.allocation_id) {
                return Err(AllocationError::InvalidAllocation);
            }
            block
                .suballocator
                .free(allocation.allocation_id)
                .ok_or(AllocationError::InvalidAllocation)?;
            block.names.remove(&allocation.allocation_id);
            if block.is_empty() {
                Some(
                    slot.take()
                        .expect("block exists after successful free")
                        .memory,
                )
            } else {
                None
            }
        };
        if let Some(memory) = memory {
            unsafe {
                device.free_memory(memory, None);
            }
        }
        Ok(())
    }

    pub unsafe fn map_memory(
        &mut self,
        allocation: &Allocation,
    ) -> Result<*mut c_void, AllocationError> {
        let device = self.device();
        let flags = self.allocation_memory_flags(allocation)?;
        if !flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
            return Err(AllocationError::InvalidAllocation);
        }
        let block = self.block_mut(allocation)?;
        let offset =
            usize::try_from(allocation.offset()).map_err(|_| AllocationError::InvalidAllocation)?;
        if block.mapped_ptr.is_none() {
            let ptr = unsafe {
                device.map_memory(
                    allocation.memory(),
                    0,
                    block.size,
                    vk::MemoryMapFlags::empty(),
                )
            }?;
            let Some(ptr) = NonNull::new(ptr) else {
                unsafe {
                    device.unmap_memory(allocation.memory());
                }
                return Err(AllocationError::InvalidAllocation);
            };
            block.mapped_ptr = Some(ptr);
        }
        let ptr = block.mapped_ptr.ok_or(AllocationError::InvalidAllocation)?;
        let map_count = block
            .map_count
            .checked_add(1)
            .ok_or(AllocationError::OutOfMemory)?;
        block.mark_mapped(allocation.allocation_id)?;
        block.map_count = map_count;
        Ok(unsafe { ptr.as_ptr().cast::<u8>().add(offset).cast() })
    }

    pub unsafe fn map<'b>(
        &'b mut self,
        allocation: &'b Allocation,
    ) -> Result<MappedAllocation<'b>, AllocationError> {
        let ptr = NonNull::new(unsafe { self.map_memory(allocation) }?.cast::<u8>())
            .ok_or(AllocationError::InvalidAllocation)?;
        Ok(MappedAllocation {
            allocator: self,
            allocation,
            ptr,
        })
    }

    pub unsafe fn map_persistent(
        &mut self,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        if let Some(pointer) = self.persistent_ptr(allocation) {
            return Ok(pointer);
        }
        let pointer = unsafe { self.map_memory(allocation) }?;
        self.block_mut(allocation)?
            .persistent
            .insert(allocation.allocation_id);
        NonNull::new(pointer.cast::<u8>()).ok_or(AllocationError::InvalidAllocation)
    }

    pub fn persistent_ptr(&self, allocation: &Allocation) -> Option<NonNull<u8>> {
        let block = self.block(allocation).ok()?;
        if !block.persistent.contains(&allocation.allocation_id) {
            return None;
        }
        let offset = usize::try_from(allocation.offset()).ok()?;
        let base = block.mapped_ptr?.cast::<u8>();
        Some(unsafe { base.add(offset) })
    }

    pub unsafe fn unmap_memory(&mut self, allocation: &Allocation) -> Result<(), AllocationError> {
        let device = self.device();
        let block = self.block_mut(allocation)?;
        if block.map_count == 0 {
            return Err(AllocationError::InvalidAllocation);
        }
        block.unmark_mapped(allocation.allocation_id)?;
        block.map_count -= 1;
        if block.map_count == 0 {
            unsafe {
                device.unmap_memory(allocation.memory());
            }
            block.mapped_ptr = None;
        }
        Ok(())
    }

    pub fn memory_type_flags(&self, memory_type_index: u32) -> Option<vk::MemoryPropertyFlags> {
        if memory_type_index >= self.memory_properties.memory_type_count {
            return None;
        }
        Some(self.memory_properties.memory_types[memory_type_index as usize].property_flags)
    }

    pub fn allocation_memory_flags(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MemoryPropertyFlags, AllocationError> {
        self.block(allocation)?;
        self.memory_type_flags(allocation.memory_type_index())
            .ok_or(AllocationError::InvalidAllocation)
    }

    pub unsafe fn flush_allocation(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device().flush_mapped_memory_ranges(&[range]) }?)
    }

    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &Allocation,
    ) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device().invalidate_mapped_memory_ranges(&[range]) }?)
    }

    pub fn stats(&self) -> AllocatorStats {
        stats_from_memory_types(&self.memory_types)
    }

    pub fn snapshot(&self) -> AllocatorSnapshot {
        snapshot_from_memory_types(&self.memory_types, &self.memory_properties)
    }

    pub fn live_allocation_count(&self) -> usize {
        self.memory_types
            .iter()
            .flat_map(|memory_type| memory_type.blocks.iter().flatten())
            .map(|block| block.suballocator.stats().allocation_count)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.live_allocation_count() == 0
    }

    pub fn live_allocations(&self) -> Vec<LiveAllocation> {
        let mut live = Vec::new();
        for memory_type in &self.memory_types {
            for block in memory_type.blocks.iter().flatten() {
                for region in block
                    .suballocator
                    .region_snapshots(&block.mapped_allocations)
                {
                    if let MemoryRegionState::Allocated { allocation_id, .. } = region.state {
                        live.push(LiveAllocation {
                            name: block.names.get(&allocation_id).map(|name| name.to_string()),
                            memory: block.memory,
                            offset: region.offset,
                            size: region.size,
                            memory_type_index: memory_type.memory_type_index,
                        });
                    }
                }
            }
        }
        live
    }

    pub unsafe fn free_all(&mut self) -> usize {
        let live = self.live_allocation_count();
        let device = self.device();
        for memory_type in &mut self.memory_types {
            for block in memory_type.blocks.drain(..).flatten() {
                unsafe { device.free_memory(block.memory, None) };
            }
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
        if !request.needs_own_block() {
            let memory_type = self
                .memory_types
                .get_mut(memory_type_index as usize)
                .ok_or(AllocationError::NoCompatibleMemoryType)?;
            for (block_index, block) in memory_type
                .blocks
                .iter_mut()
                .enumerate()
                .filter_map(|(index, block)| block.as_mut().map(|block| (index, block)))
            {
                if block.resource_class != resource_class
                    || block.dedicated
                    || block.memory_allocate_flags != request.memory_allocate_flags
                {
                    continue;
                }
                if let Some(suballocation) = block
                    .suballocator
                    .allocate(requirements.size, requirements.alignment.max(1))
                {
                    return Ok(Allocation {
                        memory_type_index,
                        block_index,
                        allocation_id: suballocation.id,
                        resource_class,
                        memory: block.memory,
                        offset: suballocation.offset,
                        size: suballocation.size,
                    });
                }
            }
        }

        let block_size = if request.needs_own_block() {
            requirements.size
        } else {
            self.block_size.max(requirements.size)
        };
        let memory =
            unsafe { self.allocate_device_memory(block_size, memory_type_index, request) }?;
        let mut block = MemoryBlock::new(
            memory,
            block_size,
            resource_class,
            request.needs_own_block(),
            request.memory_allocate_flags,
        );
        let Some(suballocation) = block
            .suballocator
            .allocate(requirements.size, requirements.alignment.max(1))
        else {
            unsafe {
                self.device().free_memory(memory, None);
            }
            return Err(AllocationError::OutOfMemory);
        };
        let memory_type = self
            .memory_types
            .get_mut(memory_type_index as usize)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        let block_index = if let Some(index) = memory_type.blocks.iter().position(Option::is_none) {
            memory_type.blocks[index] = Some(block);
            index
        } else {
            let index = memory_type.blocks.len();
            memory_type.blocks.push(Some(block));
            index
        };
        Ok(Allocation {
            memory_type_index,
            block_index,
            allocation_id: suballocation.id,
            resource_class,
            memory,
            offset: suballocation.offset,
            size: suballocation.size,
        })
    }

    unsafe fn allocate_device_memory(
        &self,
        block_size: u64,
        memory_type_index: u32,
        request: BlockRequest,
    ) -> Result<vk::DeviceMemory, AllocationError> {
        let mut flags_info =
            vk::MemoryAllocateFlagsInfo::default().flags(request.memory_allocate_flags);
        let mut dedicated_info = match request.dedicated {
            DedicatedResource::None => None,
            DedicatedResource::Buffer(buffer) => {
                Some(vk::MemoryDedicatedAllocateInfo::default().buffer(buffer))
            }
            DedicatedResource::Image(image) => {
                Some(vk::MemoryDedicatedAllocateInfo::default().image(image))
            }
        };
        let mut export_info =
            vk::ExportMemoryAllocateInfo::default().handle_types(request.export_handle_types);

        let mut allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(block_size)
            .memory_type_index(memory_type_index);
        if !request.memory_allocate_flags.is_empty() {
            allocate_info = allocate_info.push_next(&mut flags_info);
        }
        if let Some(dedicated_info) = dedicated_info.as_mut() {
            allocate_info = allocate_info.push_next(dedicated_info);
        }
        if !request.export_handle_types.is_empty() {
            allocate_info = allocate_info.push_next(&mut export_info);
        }
        Ok(unsafe { self.device.allocate_memory(&allocate_info, None) }?)
    }

    fn block(&self, allocation: &Allocation) -> Result<&MemoryBlock, AllocationError> {
        let memory_type = self
            .memory_types
            .get(allocation.memory_type_index as usize)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = memory_type
            .blocks
            .get(allocation.block_index)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = block.as_ref().ok_or(AllocationError::InvalidAllocation)?;
        if block.memory != allocation.memory {
            return Err(AllocationError::InvalidAllocation);
        }
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    fn block_mut(&mut self, allocation: &Allocation) -> Result<&mut MemoryBlock, AllocationError> {
        let memory_type = self
            .memory_types
            .get_mut(allocation.memory_type_index as usize)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = memory_type
            .blocks
            .get_mut(allocation.block_index)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = block.as_mut().ok_or(AllocationError::InvalidAllocation)?;
        if block.memory != allocation.memory {
            return Err(AllocationError::InvalidAllocation);
        }
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    fn mapped_range(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MappedMemoryRange<'static>, AllocationError> {
        let block = self.block(allocation)?;
        if !block.is_allocation_mapped(allocation.allocation_id) {
            return Err(AllocationError::InvalidAllocation);
        }
        let end = allocation
            .offset()
            .checked_add(allocation.size())
            .ok_or(AllocationError::InvalidAllocation)?;
        if end > block.size {
            return Err(AllocationError::InvalidAllocation);
        }
        let (offset, size) = mapped_range_bounds(
            allocation.offset(),
            allocation.size(),
            block.size,
            self.non_coherent_atom_size,
        )
        .ok_or(AllocationError::InvalidAllocation)?;
        Ok(vk::MappedMemoryRange::default()
            .memory(allocation.memory())
            .offset(offset)
            .size(size))
    }

    fn adjusted_requirements(
        &self,
        memory_type_index: u32,
        mut requirements: vk::MemoryRequirements,
        resource_class: AllocationResourceClass,
    ) -> Result<vk::MemoryRequirements, AllocationError> {
        let flags = self
            .memory_type_flags(memory_type_index)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        requirements.alignment = effective_alignment(
            flags,
            requirements.alignment.max(1),
            self.non_coherent_atom_size,
            self.buffer_image_granularity,
            resource_class,
        );
        Ok(requirements)
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        let live = self.live_allocations();
        if live.is_empty() {
            return;
        }
        let bytes = live.iter().map(|allocation| allocation.size).sum::<u64>();
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
