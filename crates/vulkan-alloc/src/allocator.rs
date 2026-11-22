use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::{Device, vk};

use crate::allocation::{Allocation, AllocationCreateDesc, AllocationResourceClass};
use crate::block::{BlockRequest, MemoryBlock, MemoryTypeAllocator};
use crate::error::AllocationError;
use crate::mapping::MappedAllocation;
use crate::memory_type::{effective_alignment, find_memory_type_index, memory_type_flags};
use crate::report::{
    AllocatorSnapshot, AllocatorStats, LiveAllocation, snapshot_from_memory_types,
    stats_from_memory_types,
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

pub struct Allocator {
    device: Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    non_coherent_atom_size: u64,
    buffer_image_granularity: u64,
    block_size: u64,
    memory_types: Vec<MemoryTypeAllocator>,
}

unsafe impl Send for Allocator {}

impl Allocator {
    pub fn new(desc: AllocatorCreateDesc<'_>) -> Self {
        let memory_types = (0..desc.memory_properties.memory_type_count)
            .map(MemoryTypeAllocator::new)
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
            && let Ok(memory_type) = memory_type_mut(&mut self.memory_types, &allocation)
            && let Ok(block) = memory_type.block_mut(&allocation)
        {
            block.set_name(allocation.allocation_id, name);
        }
        Ok(allocation)
    }

    pub unsafe fn free(&mut self, allocation: Allocation) -> Result<(), AllocationError> {
        let memory_type = memory_type_mut(&mut self.memory_types, &allocation)?;
        unsafe { memory_type.free(&self.device, &allocation) }
    }

    pub unsafe fn map_memory(
        &mut self,
        allocation: &Allocation,
    ) -> Result<*mut c_void, AllocationError> {
        Ok(unsafe { self.map_host_visible(allocation) }?
            .as_ptr()
            .cast())
    }

    pub unsafe fn map<'b>(
        &'b mut self,
        allocation: &'b Allocation,
    ) -> Result<MappedAllocation<'b>, AllocationError> {
        let ptr = unsafe { self.map_host_visible(allocation) }?;
        Ok(MappedAllocation::new(self, allocation, ptr))
    }

    pub unsafe fn map_persistent(
        &mut self,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        self.ensure_host_visible(allocation)?;
        let block = memory_type_mut(&mut self.memory_types, allocation)?.block_mut(allocation)?;
        unsafe { block.map_persistent(&self.device, allocation) }
    }

    pub fn persistent_ptr(&self, allocation: &Allocation) -> Option<NonNull<u8>> {
        self.block(allocation).ok()?.persistent_ptr(allocation)
    }

    pub unsafe fn unmap_memory(&mut self, allocation: &Allocation) -> Result<(), AllocationError> {
        let block = memory_type_mut(&mut self.memory_types, allocation)?.block_mut(allocation)?;
        unsafe { block.unmap(&self.device, allocation.allocation_id) }
    }

    pub fn memory_type_flags(&self, memory_type_index: u32) -> Option<vk::MemoryPropertyFlags> {
        memory_type_flags(&self.memory_properties, memory_type_index)
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
        Ok(unsafe { self.device.flush_mapped_memory_ranges(&[range]) }?)
    }

    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &Allocation,
    ) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device.invalidate_mapped_memory_ranges(&[range]) }?)
    }

    pub fn stats(&self) -> AllocatorStats {
        stats_from_memory_types(&self.memory_types)
    }

    pub fn snapshot(&self) -> AllocatorSnapshot {
        snapshot_from_memory_types(&self.memory_types, &self.memory_properties)
    }

    pub fn live_allocation_count(&self) -> usize {
        self.stats().allocation_count
    }

    pub fn is_empty(&self) -> bool {
        self.live_allocation_count() == 0
    }

    pub fn live_allocations(&self) -> Vec<LiveAllocation> {
        self.memory_types
            .iter()
            .flat_map(MemoryTypeAllocator::live_allocations)
            .collect()
    }

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

        let block_size = request.block_size(requirements.size, self.block_size);
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
        let block = memory_type_mut(&mut self.memory_types, allocation)?.block_mut(allocation)?;
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
            requirements.alignment,
            self.non_coherent_atom_size,
            self.buffer_image_granularity,
            resource_class,
        );
        Ok(requirements)
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
