use vulkan::vk;

use crate::allocation::AllocationResourceClass;
use crate::block::MemoryTypeAllocator;

/// Totals of the memory an allocator holds, returned by
/// [`Allocator::stats`](crate::Allocator::stats).
///
/// `allocated_bytes + free_bytes` equals `total_block_bytes`, unless a total is larger
/// than [`u64::MAX`]: each total saturates at [`u64::MAX`], and the equality may then
/// not hold.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorStats {
    /// The total size of all `VkDeviceMemory` blocks.
    pub total_block_bytes: u64,
    /// The bytes inside allocations, counted with [`Allocation::size`](crate::Allocation::size).
    pub allocated_bytes: u64,
    /// The bytes of the blocks outside any allocation, including gaps left by
    /// alignment.
    pub free_bytes: u64,
    /// The number of live allocations.
    pub allocation_count: usize,
    /// The number of `VkDeviceMemory` blocks.
    pub block_count: usize,
    /// The same totals for each memory type that has at least one block, in memory
    /// type order.
    pub memory_types: Vec<MemoryTypeStats>,
}

/// Totals of the memory an allocator holds in one memory type. See
/// [`AllocatorStats`] for the meaning of the fields.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeStats {
    /// The index of the memory type.
    pub memory_type_index: u32,
    /// The total size of the blocks of this memory type.
    pub total_block_bytes: u64,
    /// The bytes inside allocations.
    pub allocated_bytes: u64,
    /// The bytes of the blocks outside any allocation.
    pub free_bytes: u64,
    /// The number of live allocations.
    pub allocation_count: usize,
    /// The number of blocks.
    pub block_count: usize,
}

/// The state of an allocator down to each region of each block, returned by
/// [`Allocator::snapshot`](crate::Allocator::snapshot).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorSnapshot {
    /// The totals, as [`Allocator::stats`](crate::Allocator::stats) returns them.
    pub stats: AllocatorStats,
    /// Each memory type that has at least one block, in memory type order.
    pub memory_types: Vec<MemoryTypeSnapshot>,
}

/// The blocks of one memory type in an [`AllocatorSnapshot`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeSnapshot {
    /// The index of the memory type.
    pub memory_type_index: u32,
    /// The property flags of the memory type.
    pub property_flags: vk::MemoryPropertyFlags,
    /// The totals of this memory type.
    pub stats: MemoryTypeStats,
    /// Each block of this memory type, in [`MemoryBlockSnapshot::block_index`] order.
    pub blocks: Vec<MemoryBlockSnapshot>,
}

/// One `VkDeviceMemory` block in a [`MemoryTypeSnapshot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryBlockSnapshot {
    /// The position of the block among the blocks of its memory type. It stays the same
    /// while the block exists; the position of a freed block can be taken by a new one,
    /// and positions of freed blocks that were not reused are skipped.
    pub block_index: usize,
    /// The kind of resource the block holds.
    pub resource_class: AllocationResourceClass,
    /// Whether the block is the `VkDeviceMemory` of a single dedicated, exportable or
    /// zero-initialized allocation.
    pub dedicated: bool,
    /// The `VkDeviceMemory` of the block.
    pub memory: vk::DeviceMemory,
    /// The size of the block in bytes.
    pub size: u64,
    /// The bytes in free regions.
    pub free_bytes: u64,
    /// The bytes in allocated regions.
    pub allocated_bytes: u64,
    /// The size of the largest free region. An allocation larger than this does not
    /// fit in the block.
    pub largest_free_region_bytes: u64,
    /// The number of free regions.
    pub free_region_count: usize,
    /// The number of allocated regions, one per live allocation in the block.
    pub allocated_region_count: usize,
    /// The regions of the block in offset order. They cover the block without gaps or
    /// overlaps.
    pub regions: Vec<MemoryRegionSnapshot>,
}

/// A free or allocated range of a block in a [`MemoryBlockSnapshot`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRegionSnapshot {
    /// The byte offset of the region in the block.
    pub offset: u64,
    /// The size of the region in bytes.
    pub size: u64,
    /// Whether the region is free or allocated.
    pub state: MemoryRegionState,
}

/// Whether a [`MemoryRegionSnapshot`] is free or allocated.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryRegionState {
    /// The region is not part of any allocation.
    Free,
    /// The region is one live allocation.
    Allocated {
        /// An identifier of the allocation. No two allocations made in one process
        /// have the same identifier, whichever allocator made them.
        allocation_id: u64,
        /// The number of mapping references the allocation holds: one for each live
        /// [`MappedAllocation`](crate::MappedAllocation) guard and each
        /// [`Allocator::map_memory`](crate::Allocator::map_memory) call not yet
        /// unmapped, and one for a persistent mapping.
        mapped_count: usize,
    },
}

/// An allocation that has not been freed, returned by
/// [`Allocator::live_allocations`](crate::Allocator::live_allocations).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveAllocation {
    /// The [`AllocationCreateDesc::name`](crate::AllocationCreateDesc::name) the
    /// allocation was made with.
    pub name: Option<String>,
    /// The `VkDeviceMemory` that holds the allocation, as [`Allocation::memory`](crate::Allocation::memory).
    pub memory: vk::DeviceMemory,
    /// The byte offset in `memory`, as [`Allocation::offset`](crate::Allocation::offset).
    pub offset: u64,
    /// The size in bytes, as [`Allocation::size`](crate::Allocation::size).
    pub size: u64,
    /// The memory type, as [`Allocation::memory_type_index`](crate::Allocation::memory_type_index).
    pub memory_type_index: u32,
}

pub(crate) fn stats_from_memory_types(memory_types: &[MemoryTypeAllocator]) -> AllocatorStats {
    let mut stats = AllocatorStats::default();
    for memory_type in memory_types {
        let type_stats = memory_type.stats();
        if type_stats.block_count == 0 {
            continue;
        }
        stats.total_block_bytes = stats
            .total_block_bytes
            .saturating_add(type_stats.total_block_bytes);
        stats.allocated_bytes = stats
            .allocated_bytes
            .saturating_add(type_stats.allocated_bytes);
        stats.free_bytes = stats.free_bytes.saturating_add(type_stats.free_bytes);
        stats.allocation_count = stats
            .allocation_count
            .saturating_add(type_stats.allocation_count);
        stats.block_count = stats.block_count.saturating_add(type_stats.block_count);
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
            memory_properties.memory_types[memory_type.memory_type_index() as usize].property_flags;
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
    use crate::suballocator::Suballocation;

    fn memory_type_holding(
        memory_type_index: u32,
        mut block: MemoryBlock,
        allocation_size: u64,
    ) -> (MemoryTypeAllocator, Suballocation) {
        let suballocation = block.suballocate(allocation_size, 1).unwrap();
        let mut memory_type = MemoryTypeAllocator::new(memory_type_index);
        memory_type.insert_block(block, suballocation);
        (memory_type, suballocation)
    }

    fn saturating_total(values: impl IntoIterator<Item = u64>) -> u64 {
        let total: u128 = values.into_iter().map(u128::from).sum();
        u64::try_from(total).unwrap_or(u64::MAX)
    }

    struct ExpectedTotals {
        total_block_bytes: u64,
        allocated_bytes: u64,
        free_bytes: u64,
        allocation_count: usize,
        block_count: usize,
    }

    fn memory_type_with_blocks(
        memory_type_index: u32,
        block_sizes: &[u64],
        first_handle: u64,
    ) -> (MemoryTypeAllocator, ExpectedTotals) {
        let mut memory_type = MemoryTypeAllocator::new(memory_type_index);
        let mut allocated = Vec::new();
        for (index, &size) in block_sizes.iter().enumerate() {
            let mut block = MemoryBlock::new(
                vk::DeviceMemory(first_handle + index as u64),
                size,
                AllocationResourceClass::Linear,
                false,
                vk::MemoryAllocateFlags::empty(),
            );
            let suballocation = block.suballocate((size / 2).max(1), 1).unwrap();
            allocated.push(suballocation.size);
            memory_type.insert_block(block, suballocation);
        }
        let expected = ExpectedTotals {
            total_block_bytes: saturating_total(block_sizes.iter().copied()),
            allocated_bytes: saturating_total(allocated.iter().copied()),
            free_bytes: saturating_total(
                block_sizes
                    .iter()
                    .zip(&allocated)
                    .map(|(&size, &allocated)| size - allocated),
            ),
            allocation_count: block_sizes.len(),
            block_count: block_sizes.len(),
        };
        (memory_type, expected)
    }

    #[test]
    fn totals_saturate_at_u64_max_and_are_exact_below_it() {
        let block_size_sets: [&[&[u64]]; 8] = [
            &[&[1 << 63, 1 << 63]],
            &[&[u64::MAX, 1]],
            &[&[u64::MAX, u64::MAX, u64::MAX]],
            &[&[(1 << 63) - 1, 1 << 63]],
            &[&[1 << 62, 1 << 62], &[1 << 62]],
            &[&[1 << 63], &[1 << 63]],
            &[&[u64::MAX], &[4096], &[u64::MAX, 1 << 40]],
            &[&[1024, 2048], &[4096]],
        ];
        for memory_type_blocks in block_size_sets {
            let properties = memory_properties(&vec![
                vk::MemoryPropertyFlags::DEVICE_LOCAL;
                memory_type_blocks.len()
            ]);
            let mut memory_types = Vec::new();
            let mut expected = Vec::new();
            for (index, block_sizes) in memory_type_blocks.iter().enumerate() {
                let (memory_type, totals) =
                    memory_type_with_blocks(index as u32, block_sizes, 100 * (index as u64 + 1));
                memory_types.push(memory_type);
                expected.push(totals);
            }

            let stats = stats_from_memory_types(&memory_types);
            let label = format!("{memory_type_blocks:?}");
            assert_eq!(
                stats.total_block_bytes,
                saturating_total(expected.iter().map(|totals| totals.total_block_bytes)),
                "{label}: total_block_bytes must be the sum, saturated at u64::MAX"
            );
            assert_eq!(
                stats.allocated_bytes,
                saturating_total(expected.iter().map(|totals| totals.allocated_bytes)),
                "{label}: allocated_bytes must be the sum, saturated at u64::MAX"
            );
            assert_eq!(
                stats.free_bytes,
                saturating_total(expected.iter().map(|totals| totals.free_bytes)),
                "{label}: free_bytes must be the sum, saturated at u64::MAX"
            );
            assert_eq!(
                stats.allocation_count,
                expected
                    .iter()
                    .map(|totals| totals.allocation_count)
                    .sum::<usize>(),
                "{label}: allocation_count must be exact"
            );
            assert_eq!(
                stats.block_count,
                expected
                    .iter()
                    .map(|totals| totals.block_count)
                    .sum::<usize>(),
                "{label}: block_count must be exact"
            );
            assert_eq!(stats.memory_types.len(), expected.len(), "{label}");
            for (type_stats, totals) in stats.memory_types.iter().zip(&expected) {
                assert_eq!(
                    (
                        type_stats.total_block_bytes,
                        type_stats.allocated_bytes,
                        type_stats.free_bytes,
                        type_stats.allocation_count,
                        type_stats.block_count,
                    ),
                    (
                        totals.total_block_bytes,
                        totals.allocated_bytes,
                        totals.free_bytes,
                        totals.allocation_count,
                        totals.block_count,
                    ),
                    "{label}: the totals of memory type {} must be the sums, saturated at u64::MAX",
                    type_stats.memory_type_index
                );
            }

            let snapshot = snapshot_from_memory_types(&memory_types, &properties);
            assert_eq!(
                snapshot.stats, stats,
                "{label}: the snapshot must carry the same totals"
            );
            for (type_snapshot, type_stats) in snapshot.memory_types.iter().zip(&stats.memory_types)
            {
                assert_eq!(&type_snapshot.stats, type_stats, "{label}");
            }
        }
    }

    #[test]
    fn allocator_stats_aggregate_non_empty_memory_types() {
        let (linear, a) = memory_type_holding(
            1,
            MemoryBlock::new(
                vk::DeviceMemory(21),
                1024,
                AllocationResourceClass::Linear,
                false,
                vk::MemoryAllocateFlags::empty(),
            ),
            128,
        );
        let (optimal, b) = memory_type_holding(
            2,
            MemoryBlock::new(
                vk::DeviceMemory(22),
                2048,
                AllocationResourceClass::Optimal,
                false,
                vk::MemoryAllocateFlags::empty(),
            ),
            256,
        );
        let memory_types = vec![MemoryTypeAllocator::new(0), linear, optimal];

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
        let (dedicated, allocation) = memory_type_holding(
            1,
            MemoryBlock::new(
                vk::DeviceMemory(42),
                1024,
                AllocationResourceClass::Optimal,
                true,
                vk::MemoryAllocateFlags::empty(),
            ),
            128,
        );
        let memory_types = vec![MemoryTypeAllocator::new(0), dedicated];

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
