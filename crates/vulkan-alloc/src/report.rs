use vulkan::vk;

use crate::allocation::AllocationResourceClass;
use crate::block::MemoryTypeAllocator;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorStats {
    pub total_block_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    pub allocation_count: usize,
    pub block_count: usize,
    pub memory_types: Vec<MemoryTypeStats>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeStats {
    pub memory_type_index: u32,
    pub total_block_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    pub allocation_count: usize,
    pub block_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorSnapshot {
    pub stats: AllocatorStats,
    pub memory_types: Vec<MemoryTypeSnapshot>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeSnapshot {
    pub memory_type_index: u32,
    pub property_flags: vk::MemoryPropertyFlags,
    pub stats: MemoryTypeStats,
    pub blocks: Vec<MemoryBlockSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryBlockSnapshot {
    pub block_index: usize,
    pub resource_class: AllocationResourceClass,
    pub dedicated: bool,
    pub memory: vk::DeviceMemory,
    pub size: u64,
    pub free_bytes: u64,
    pub allocated_bytes: u64,
    pub largest_free_region_bytes: u64,
    pub free_region_count: usize,
    pub allocated_region_count: usize,
    pub regions: Vec<MemoryRegionSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRegionSnapshot {
    pub offset: u64,
    pub size: u64,
    pub state: MemoryRegionState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryRegionState {
    Free,
    Allocated {
        allocation_id: u64,
        mapped_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAllocation {
    pub name: Option<String>,
    pub memory: vk::DeviceMemory,
    pub offset: u64,
    pub size: u64,
    pub memory_type_index: u32,
}

pub(crate) fn stats_from_memory_types(memory_types: &[MemoryTypeAllocator]) -> AllocatorStats {
    let mut stats = AllocatorStats::default();
    for memory_type in memory_types {
        let type_stats = memory_type.stats();
        if type_stats.block_count == 0 {
            continue;
        }
        stats.total_block_bytes += type_stats.total_block_bytes;
        stats.allocated_bytes += type_stats.allocated_bytes;
        stats.free_bytes += type_stats.free_bytes;
        stats.allocation_count += type_stats.allocation_count;
        stats.block_count += type_stats.block_count;
        stats.memory_types.push(type_stats);
    }
    stats
}

pub(crate) fn snapshot_from_memory_types(
    memory_types: &[MemoryTypeAllocator],
    memory_properties: &vk::PhysicalDeviceMemoryProperties,
) -> AllocatorSnapshot {
    let stats = stats_from_memory_types(memory_types);
    let mut snapshots = Vec::new();
    for memory_type in memory_types {
        let stats = memory_type.stats();
        if stats.block_count == 0 {
            continue;
        }
        let property_flags =
            memory_properties.memory_types[memory_type.memory_type_index as usize].property_flags;
        snapshots.push(memory_type.snapshot(property_flags, stats));
    }
    AllocatorSnapshot {
        stats,
        memory_types: snapshots,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::MemoryBlock;
    use crate::memory_type::tests::memory_properties;

    #[test]
    fn allocator_stats_aggregate_non_empty_memory_types() {
        let mut memory_types = vec![
            MemoryTypeAllocator {
                memory_type_index: 0,
                blocks: Vec::new(),
            },
            MemoryTypeAllocator {
                memory_type_index: 1,
                blocks: vec![Some(MemoryBlock::new(
                    vk::DeviceMemory(21),
                    1024,
                    AllocationResourceClass::Linear,
                    false,
                    vk::MemoryAllocateFlags::empty(),
                ))],
            },
            MemoryTypeAllocator {
                memory_type_index: 2,
                blocks: vec![Some(MemoryBlock::new(
                    vk::DeviceMemory(22),
                    2048,
                    AllocationResourceClass::Optimal,
                    false,
                    vk::MemoryAllocateFlags::empty(),
                ))],
            },
        ];
        let a = memory_types[1].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();
        let b = memory_types[2].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(256, 1)
            .unwrap();

        let stats = stats_from_memory_types(&memory_types);
        assert_eq!(stats.memory_types.len(), 2);
        assert_eq!(stats.memory_types[0].memory_type_index, 1);
        assert_eq!(stats.memory_types[1].memory_type_index, 2);
        assert_eq!(stats.block_count, 2);
        assert_eq!(stats.total_block_bytes, 3072);
        assert_eq!(stats.allocation_count, 2);
        assert_eq!(stats.allocated_bytes, a.size + b.size);
        assert_eq!(
            stats.allocated_bytes + stats.free_bytes,
            stats.total_block_bytes
        );
    }

    #[test]
    fn allocator_snapshot_keeps_stats_and_region_details_together() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);
        let mut memory_types = vec![
            MemoryTypeAllocator {
                memory_type_index: 0,
                blocks: Vec::new(),
            },
            MemoryTypeAllocator {
                memory_type_index: 1,
                blocks: vec![Some(MemoryBlock::new(
                    vk::DeviceMemory(42),
                    1024,
                    AllocationResourceClass::Optimal,
                    true,
                    vk::MemoryAllocateFlags::empty(),
                ))],
            },
        ];
        let allocation = memory_types[1].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();

        let snapshot = snapshot_from_memory_types(&memory_types, &properties);
        assert_eq!(snapshot.stats.block_count, 1);
        assert_eq!(snapshot.stats.allocation_count, 1);
        assert_eq!(snapshot.memory_types.len(), 1);
        assert_eq!(snapshot.memory_types[0].memory_type_index, 1);
        assert_eq!(
            snapshot.memory_types[0].property_flags,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED
        );
        assert_eq!(
            snapshot.memory_types[0].stats,
            snapshot.stats.memory_types[0]
        );
        assert_eq!(snapshot.memory_types[0].blocks.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks[0].block_index, 0);
        assert_eq!(
            snapshot.memory_types[0].blocks[0].resource_class,
            AllocationResourceClass::Optimal
        );
        assert!(snapshot.memory_types[0].blocks[0].dedicated);
        assert_eq!(snapshot.memory_types[0].blocks[0].regions[0].offset, 0);
        assert_eq!(snapshot.memory_types[0].blocks[0].regions[0].size, 128);
        assert_eq!(
            snapshot.memory_types[0].blocks[0].regions[0].state,
            MemoryRegionState::Allocated {
                allocation_id: allocation.id,
                mapped_count: 0,
            }
        );
    }
}
