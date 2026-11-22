use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::{Device, vk};

use crate::allocation::{Allocation, AllocationResourceClass, AllocationScheme};
use crate::error::AllocationError;
use crate::mapping::mapped_range_bounds;
use crate::report::{
    LiveAllocation, MemoryBlockSnapshot, MemoryRegionState, MemoryTypeSnapshot, MemoryTypeStats,
};
use crate::suballocator::{Suballocation, TlsfSuballocator};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum DedicatedResource {
    None,
    Buffer(vk::Buffer),
    Image(vk::Image),
}

impl From<AllocationScheme> for DedicatedResource {
    fn from(value: AllocationScheme) -> Self {
        match value {
            AllocationScheme::GpuAllocatorManaged => Self::None,
            AllocationScheme::DedicatedBuffer(buffer) => Self::Buffer(buffer),
            AllocationScheme::DedicatedImage(image) => Self::Image(image),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct BlockRequest {
    pub(crate) dedicated: DedicatedResource,
    pub(crate) memory_allocate_flags: vk::MemoryAllocateFlags,
    pub(crate) export_handle_types: vk::ExternalMemoryHandleTypeFlags,
}

impl BlockRequest {
    pub(crate) fn needs_own_block(self) -> bool {
        self.dedicated != DedicatedResource::None || !self.export_handle_types.is_empty()
    }

    pub(crate) fn block_size(self, allocation_size: u64, shared_block_size: u64) -> u64 {
        if self.needs_own_block() {
            allocation_size
        } else {
            shared_block_size.max(allocation_size)
        }
    }

    unsafe fn allocate_memory(
        self,
        device: &Device,
        memory_type_index: u32,
        size: u64,
    ) -> Result<vk::DeviceMemory, AllocationError> {
        let mut flags_info =
            vk::MemoryAllocateFlagsInfo::default().flags(self.memory_allocate_flags);
        let mut dedicated_info = match self.dedicated {
            DedicatedResource::None => None,
            DedicatedResource::Buffer(buffer) => {
                Some(vk::MemoryDedicatedAllocateInfo::default().buffer(buffer))
            }
            DedicatedResource::Image(image) => {
                Some(vk::MemoryDedicatedAllocateInfo::default().image(image))
            }
        };
        let mut export_info =
            vk::ExportMemoryAllocateInfo::default().handle_types(self.export_handle_types);

        let mut allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(size)
            .memory_type_index(memory_type_index);
        if !self.memory_allocate_flags.is_empty() {
            allocate_info = allocate_info.push_next(&mut flags_info);
        }
        if let Some(dedicated_info) = dedicated_info.as_mut() {
            allocate_info = allocate_info.push_next(dedicated_info);
        }
        if !self.export_handle_types.is_empty() {
            allocate_info = allocate_info.push_next(&mut export_info);
        }
        Ok(unsafe { device.allocate_memory(&allocate_info, None) }?)
    }
}

pub(crate) struct MemoryTypeAllocator {
    memory_type_index: u32,
    blocks: Vec<Option<MemoryBlock>>,
}

impl MemoryTypeAllocator {
    pub(crate) fn new(memory_type_index: u32) -> Self {
        Self {
            memory_type_index,
            blocks: Vec::new(),
        }
    }

    pub(crate) fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }

    pub(crate) fn allocate_from_existing_blocks(
        &mut self,
        requirements: vk::MemoryRequirements,
        resource_class: AllocationResourceClass,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Option<Allocation> {
        let memory_type_index = self.memory_type_index;
        self.blocks
            .iter_mut()
            .enumerate()
            .find_map(|(block_index, slot)| {
                let block = slot.as_mut().filter(|block| {
                    block.is_shareable_with(resource_class, memory_allocate_flags)
                })?;
                let suballocation = block.suballocate(requirements.size, requirements.alignment)?;
                Some(block.allocation(memory_type_index, block_index, suballocation))
            })
    }

    pub(crate) fn insert_block(
        &mut self,
        block: MemoryBlock,
        suballocation: Suballocation,
    ) -> Allocation {
        let block_index = self.vacant_slot_index();
        let allocation = block.allocation(self.memory_type_index, block_index, suballocation);
        self.blocks[block_index] = Some(block);
        allocation
    }

    fn vacant_slot_index(&mut self) -> usize {
        if let Some(index) = self.blocks.iter().position(Option::is_none) {
            return index;
        }
        self.blocks.push(None);
        self.blocks.len() - 1
    }

    pub(crate) fn block(&self, allocation: &Allocation) -> Result<&MemoryBlock, AllocationError> {
        let block = self
            .blocks
            .get(allocation.block_index)
            .and_then(Option::as_ref)
            .ok_or(AllocationError::InvalidAllocation)?;
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    pub(crate) fn block_mut(
        &mut self,
        allocation: &Allocation,
    ) -> Result<&mut MemoryBlock, AllocationError> {
        let block = self
            .blocks
            .get_mut(allocation.block_index)
            .and_then(Option::as_mut)
            .ok_or(AllocationError::InvalidAllocation)?;
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    pub(crate) unsafe fn free(
        &mut self,
        device: &Device,
        allocation: &Allocation,
    ) -> Result<(), AllocationError> {
        let block = self.block_mut(allocation)?;
        unsafe { block.free_allocation(device, allocation.allocation_id) }?;
        if block.is_empty()
            && let Some(block) = self
                .blocks
                .get_mut(allocation.block_index)
                .and_then(Option::take)
        {
            unsafe { block.free_memory(device) };
        }
        Ok(())
    }

    pub(crate) unsafe fn free_all_blocks(&mut self, device: &Device) {
        for block in self.blocks.drain(..).flatten() {
            unsafe { block.free_memory(device) };
        }
    }

    pub(crate) fn stats(&self) -> MemoryTypeStats {
        let mut stats = MemoryTypeStats {
            memory_type_index: self.memory_type_index,
            ..Default::default()
        };
        for block in self.blocks.iter().flatten() {
            let block_stats = block.suballocator.stats();
            stats.total_block_bytes += block.size;
            stats.allocated_bytes += block_stats.allocated_bytes;
            stats.free_bytes += block_stats.free_bytes;
            stats.allocation_count += block_stats.allocation_count;
            stats.block_count += 1;
        }
        stats
    }

    pub(crate) fn snapshot(
        &self,
        property_flags: vk::MemoryPropertyFlags,
        stats: MemoryTypeStats,
    ) -> MemoryTypeSnapshot {
        MemoryTypeSnapshot {
            memory_type_index: self.memory_type_index,
            property_flags,
            stats,
            blocks: self
                .blocks
                .iter()
                .enumerate()
                .filter_map(|(index, block)| block.as_ref().map(|block| block.snapshot(index)))
                .collect(),
        }
    }

    pub(crate) fn live_allocations(&self) -> impl Iterator<Item = LiveAllocation> + '_ {
        self.blocks
            .iter()
            .flatten()
            .flat_map(|block| block.live_allocations(self.memory_type_index))
    }
}

pub(crate) struct MemoryBlock {
    memory: vk::DeviceMemory,
    size: u64,
    resource_class: AllocationResourceClass,
    dedicated: bool,
    memory_allocate_flags: vk::MemoryAllocateFlags,
    suballocator: TlsfSuballocator,
    mapped_ptr: Option<NonNull<c_void>>,
    mapped_allocations: HashMap<u64, usize>,
    names: HashMap<u64, Box<str>>,
    persistently_mapped: HashSet<u64>,
}

impl MemoryBlock {
    pub(crate) fn new(
        memory: vk::DeviceMemory,
        size: u64,
        resource_class: AllocationResourceClass,
        dedicated: bool,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Self {
        Self {
            memory,
            size,
            resource_class,
            dedicated,
            memory_allocate_flags,
            suballocator: TlsfSuballocator::new(size),
            mapped_ptr: None,
            mapped_allocations: HashMap::new(),
            names: HashMap::new(),
            persistently_mapped: HashSet::new(),
        }
    }

    pub(crate) unsafe fn allocate(
        device: &Device,
        memory_type_index: u32,
        size: u64,
        resource_class: AllocationResourceClass,
        request: BlockRequest,
    ) -> Result<Self, AllocationError> {
        let memory = unsafe { request.allocate_memory(device, memory_type_index, size) }?;
        Ok(Self::new(
            memory,
            size,
            resource_class,
            request.needs_own_block(),
            request.memory_allocate_flags,
        ))
    }

    pub(crate) unsafe fn free_memory(self, device: &Device) {
        unsafe { device.free_memory(self.memory, None) };
    }

    fn is_shareable_with(
        &self,
        resource_class: AllocationResourceClass,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> bool {
        !self.dedicated
            && self.resource_class == resource_class
            && self.memory_allocate_flags == memory_allocate_flags
    }

    pub(crate) fn suballocate(&mut self, size: u64, alignment: u64) -> Option<Suballocation> {
        self.suballocator.allocate(size, alignment)
    }

    fn allocation(
        &self,
        memory_type_index: u32,
        block_index: usize,
        suballocation: Suballocation,
    ) -> Allocation {
        Allocation {
            memory_type_index,
            block_index,
            allocation_id: suballocation.id,
            resource_class: self.resource_class,
            memory: self.memory,
            offset: suballocation.offset,
            size: suballocation.size,
        }
    }

    fn validate_allocation(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        let expected = Suballocation {
            id: allocation.allocation_id,
            offset: allocation.offset,
            size: allocation.size,
        };
        if self.memory != allocation.memory
            || self.resource_class != allocation.resource_class
            || self.suballocator.allocation(allocation.allocation_id) != Some(expected)
        {
            return Err(AllocationError::InvalidAllocation);
        }
        let end = allocation
            .offset
            .checked_add(allocation.size)
            .ok_or(AllocationError::InvalidAllocation)?;
        if end > self.size {
            return Err(AllocationError::InvalidAllocation);
        }
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.suballocator.is_empty()
    }

    pub(crate) fn set_name(&mut self, allocation_id: u64, name: &str) {
        self.names.insert(allocation_id, name.into());
    }

    pub(crate) unsafe fn map(
        &mut self,
        device: &Device,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        let offset =
            usize::try_from(allocation.offset).map_err(|_| AllocationError::InvalidAllocation)?;
        let base = match self.mapped_ptr {
            Some(base) => base,
            None => unsafe { self.map_whole_block(device) }?,
        };
        self.mark_mapped(allocation.allocation_id)?;
        Ok(unsafe { base.cast::<u8>().add(offset) })
    }

    unsafe fn map_whole_block(
        &mut self,
        device: &Device,
    ) -> Result<NonNull<c_void>, AllocationError> {
        let ptr =
            unsafe { device.map_memory(self.memory, 0, self.size, vk::MemoryMapFlags::empty()) }?;
        let Some(ptr) = NonNull::new(ptr) else {
            unsafe { device.unmap_memory(self.memory) };
            return Err(AllocationError::InvalidAllocation);
        };
        self.mapped_ptr = Some(ptr);
        Ok(ptr)
    }

    pub(crate) unsafe fn map_persistent(
        &mut self,
        device: &Device,
        allocation: &Allocation,
    ) -> Result<NonNull<u8>, AllocationError> {
        if let Some(ptr) = self.persistent_ptr(allocation) {
            return Ok(ptr);
        }
        let ptr = unsafe { self.map(device, allocation) }?;
        self.persistently_mapped.insert(allocation.allocation_id);
        Ok(ptr)
    }

    pub(crate) fn persistent_ptr(&self, allocation: &Allocation) -> Option<NonNull<u8>> {
        if !self.persistently_mapped.contains(&allocation.allocation_id) {
            return None;
        }
        let offset = usize::try_from(allocation.offset).ok()?;
        Some(unsafe { self.mapped_ptr?.cast::<u8>().add(offset) })
    }

    pub(crate) unsafe fn unmap(
        &mut self,
        device: &Device,
        allocation_id: u64,
    ) -> Result<(), AllocationError> {
        if self.holds_only_its_persistent_mapping(allocation_id) {
            return Err(AllocationError::InvalidAllocation);
        }
        unsafe { self.release_mapping(device, allocation_id) }
    }

    fn holds_only_its_persistent_mapping(&self, allocation_id: u64) -> bool {
        self.persistently_mapped.contains(&allocation_id)
            && self.mapped_allocations.get(&allocation_id) == Some(&1)
    }

    unsafe fn release_mapping(
        &mut self,
        device: &Device,
        allocation_id: u64,
    ) -> Result<(), AllocationError> {
        self.unmark_mapped(allocation_id)?;
        if self.mapped_allocations.is_empty() {
            unsafe { device.unmap_memory(self.memory) };
            self.mapped_ptr = None;
        }
        Ok(())
    }

    unsafe fn free_allocation(
        &mut self,
        device: &Device,
        allocation_id: u64,
    ) -> Result<(), AllocationError> {
        if self.suballocator.allocation(allocation_id).is_none()
            || self.holds_mapping_besides_its_persistent_one(allocation_id)
        {
            return Err(AllocationError::InvalidAllocation);
        }
        if self.persistently_mapped.remove(&allocation_id) {
            unsafe { self.release_mapping(device, allocation_id) }?;
        }
        self.suballocator
            .free(allocation_id)
            .ok_or(AllocationError::InvalidAllocation)?;
        self.names.remove(&allocation_id);
        Ok(())
    }

    fn holds_mapping_besides_its_persistent_one(&self, allocation_id: u64) -> bool {
        let references = self
            .mapped_allocations
            .get(&allocation_id)
            .copied()
            .unwrap_or(0);
        references > usize::from(self.persistently_mapped.contains(&allocation_id))
    }

    fn is_allocation_mapped(&self, allocation_id: u64) -> bool {
        self.mapped_allocations.contains_key(&allocation_id)
    }

    fn mark_mapped(&mut self, allocation_id: u64) -> Result<(), AllocationError> {
        let count = self.mapped_allocations.entry(allocation_id).or_default();
        *count = count.checked_add(1).ok_or(AllocationError::OutOfMemory)?;
        Ok(())
    }

    fn unmark_mapped(&mut self, allocation_id: u64) -> Result<(), AllocationError> {
        let count = self
            .mapped_allocations
            .get_mut(&allocation_id)
            .ok_or(AllocationError::InvalidAllocation)?;
        *count -= 1;
        if *count == 0 {
            self.mapped_allocations.remove(&allocation_id);
        }
        Ok(())
    }

    pub(crate) fn mapped_range(
        &self,
        allocation: &Allocation,
        non_coherent_atom_size: u64,
    ) -> Result<vk::MappedMemoryRange<'static>, AllocationError> {
        if !self.is_allocation_mapped(allocation.allocation_id) {
            return Err(AllocationError::InvalidAllocation);
        }
        let (offset, size) = mapped_range_bounds(
            allocation.offset,
            allocation.size,
            self.size,
            non_coherent_atom_size,
        )
        .ok_or(AllocationError::InvalidAllocation)?;
        Ok(vk::MappedMemoryRange::default()
            .memory(self.memory)
            .offset(offset)
            .size(size))
    }

    fn snapshot(&self, block_index: usize) -> MemoryBlockSnapshot {
        let regions = self.suballocator.region_snapshots(&self.mapped_allocations);
        let mut snapshot = MemoryBlockSnapshot {
            block_index,
            resource_class: self.resource_class,
            dedicated: self.dedicated,
            memory: self.memory,
            size: self.size,
            free_bytes: 0,
            allocated_bytes: 0,
            largest_free_region_bytes: 0,
            free_region_count: 0,
            allocated_region_count: 0,
            regions,
        };
        for region in &snapshot.regions {
            match region.state {
                MemoryRegionState::Free => {
                    snapshot.free_bytes += region.size;
                    snapshot.free_region_count += 1;
                    snapshot.largest_free_region_bytes =
                        snapshot.largest_free_region_bytes.max(region.size);
                }
                MemoryRegionState::Allocated { .. } => {
                    snapshot.allocated_bytes += region.size;
                    snapshot.allocated_region_count += 1;
                }
            }
        }
        snapshot
    }

    fn live_allocations(
        &self,
        memory_type_index: u32,
    ) -> impl Iterator<Item = LiveAllocation> + '_ {
        self.suballocator
            .region_snapshots(&self.mapped_allocations)
            .into_iter()
            .filter_map(move |region| match region.state {
                MemoryRegionState::Allocated { allocation_id, .. } => Some(LiveAllocation {
                    name: self.names.get(&allocation_id).map(|name| name.to_string()),
                    memory: self.memory,
                    offset: region.offset,
                    size: region.size,
                    memory_type_index,
                }),
                MemoryRegionState::Free => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{MemoryRegionSnapshot, snapshot_from_memory_types};

    #[test]
    fn memory_block_tracks_mapped_allocations_individually() {
        let mut block = MemoryBlock::new(
            vk::DeviceMemory(31),
            1024,
            AllocationResourceClass::Linear,
            false,
            vk::MemoryAllocateFlags::empty(),
        );

        block.mark_mapped(1).unwrap();
        block.mark_mapped(1).unwrap();
        block.mark_mapped(2).unwrap();
        assert!(block.is_allocation_mapped(1));
        assert!(block.is_allocation_mapped(2));

        block.unmark_mapped(1).unwrap();
        assert!(block.is_allocation_mapped(1));
        block.unmark_mapped(1).unwrap();
        assert!(!block.is_allocation_mapped(1));
        assert!(block.is_allocation_mapped(2));
        block.unmark_mapped(2).unwrap();
        assert!(!block.is_allocation_mapped(2));
        assert_eq!(
            block.unmark_mapped(2),
            Err(AllocationError::InvalidAllocation)
        );
    }

    #[test]
    fn memory_block_validates_allocation_identity() {
        let mut block = MemoryBlock::new(
            vk::DeviceMemory(32),
            1024,
            AllocationResourceClass::Optimal,
            false,
            vk::MemoryAllocateFlags::empty(),
        );
        let suballocation = block.suballocator.allocate(128, 64).unwrap();
        let make_allocation = || Allocation {
            memory_type_index: 0,
            block_index: 0,
            allocation_id: suballocation.id,
            resource_class: AllocationResourceClass::Optimal,
            memory: vk::DeviceMemory(32),
            offset: suballocation.offset,
            size: suballocation.size,
        };
        let allocation = make_allocation();
        assert_eq!(block.validate_allocation(&allocation), Ok(()));

        let mut wrong_id = make_allocation();
        wrong_id.allocation_id += 1;
        assert_eq!(
            block.validate_allocation(&wrong_id),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_offset = make_allocation();
        wrong_offset.offset += 64;
        assert_eq!(
            block.validate_allocation(&wrong_offset),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_size = make_allocation();
        wrong_size.size += 64;
        assert_eq!(
            block.validate_allocation(&wrong_size),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_memory = make_allocation();
        wrong_memory.memory = vk::DeviceMemory(33);
        assert_eq!(
            block.validate_allocation(&wrong_memory),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_class = make_allocation();
        wrong_class.resource_class = AllocationResourceClass::Linear;
        assert_eq!(
            block.validate_allocation(&wrong_class),
            Err(AllocationError::InvalidAllocation)
        );
    }

    #[test]
    fn memory_type_stats_aggregate_blocks() {
        let mut memory_type = MemoryTypeAllocator {
            memory_type_index: 7,
            blocks: vec![
                Some(MemoryBlock::new(
                    vk::DeviceMemory(11),
                    1024,
                    AllocationResourceClass::Linear,
                    false,
                    vk::MemoryAllocateFlags::empty(),
                )),
                Some(MemoryBlock::new(
                    vk::DeviceMemory(12),
                    2048,
                    AllocationResourceClass::Linear,
                    false,
                    vk::MemoryAllocateFlags::empty(),
                )),
            ],
        };
        let a = memory_type.blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();
        let b = memory_type.blocks[1]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(256, 1)
            .unwrap();
        let stats = memory_type.stats();
        assert_eq!(stats.memory_type_index, 7);
        assert_eq!(stats.block_count, 2);
        assert_eq!(stats.total_block_bytes, 3072);
        assert_eq!(stats.allocation_count, 2);
        assert_eq!(stats.allocated_bytes, a.size + b.size);
        assert_eq!(stats.allocated_bytes + stats.free_bytes, 3072);
    }

    #[test]
    fn stats_and_snapshot_ignore_released_block_slots() {
        let memory_type = MemoryTypeAllocator {
            memory_type_index: 5,
            blocks: vec![
                None,
                Some(MemoryBlock::new(
                    vk::DeviceMemory(51),
                    1024,
                    AllocationResourceClass::Linear,
                    false,
                    vk::MemoryAllocateFlags::empty(),
                )),
            ],
        };
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 6,
            ..Default::default()
        };
        properties.memory_types[5].property_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;

        let stats = memory_type.stats();
        assert_eq!(stats.block_count, 1);
        assert_eq!(stats.total_block_bytes, 1024);

        let snapshot = snapshot_from_memory_types(&[memory_type], &properties);
        assert_eq!(snapshot.memory_types.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks[0].block_index, 1);
    }

    #[test]
    fn block_snapshot_reports_fragmented_regions() {
        let mut block = MemoryBlock::new(
            vk::DeviceMemory(41),
            1024,
            AllocationResourceClass::Linear,
            false,
            vk::MemoryAllocateFlags::empty(),
        );
        let a = block.suballocator.allocate(256, 1).unwrap();
        let b = block.suballocator.allocate(256, 1).unwrap();
        let c = block.suballocator.allocate(256, 1).unwrap();
        let d = block.suballocator.allocate(256, 1).unwrap();
        block.suballocator.free(b.id).unwrap();
        block.suballocator.free(d.id).unwrap();
        block.mark_mapped(c.id).unwrap();

        let snapshot = block.snapshot(3);
        assert_eq!(snapshot.block_index, 3);
        assert_eq!(snapshot.resource_class, AllocationResourceClass::Linear);
        assert_eq!(snapshot.memory, vk::DeviceMemory(41));
        assert_eq!(snapshot.size, 1024);
        assert_eq!(snapshot.free_bytes, 512);
        assert_eq!(snapshot.allocated_bytes, 512);
        assert_eq!(snapshot.largest_free_region_bytes, 256);
        assert_eq!(snapshot.free_region_count, 2);
        assert_eq!(snapshot.allocated_region_count, 2);
        assert_eq!(
            snapshot.regions,
            vec![
                MemoryRegionSnapshot {
                    offset: 0,
                    size: 256,
                    state: MemoryRegionState::Allocated {
                        allocation_id: a.id,
                        mapped_count: 0,
                    },
                },
                MemoryRegionSnapshot {
                    offset: 256,
                    size: 256,
                    state: MemoryRegionState::Free,
                },
                MemoryRegionSnapshot {
                    offset: 512,
                    size: 256,
                    state: MemoryRegionState::Allocated {
                        allocation_id: c.id,
                        mapped_count: 1,
                    },
                },
                MemoryRegionSnapshot {
                    offset: 768,
                    size: 256,
                    state: MemoryRegionState::Free,
                },
            ]
        );
    }
}
