use std::collections::{BTreeMap, BTreeSet, HashMap};

use crate::align::align_up;
use crate::report::{MemoryRegionSnapshot, MemoryRegionState};

const MIN_SPLIT_SIZE: u64 = 256;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Suballocation {
    pub(crate) id: u64,
    pub(crate) offset: u64,
    pub(crate) size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SuballocatorStats {
    pub(crate) allocated_bytes: u64,
    pub(crate) free_bytes: u64,
    pub(crate) allocation_count: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) struct Block {
    pub(crate) size: u64,
    pub(crate) free: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TlsfSuballocator {
    size: u64,
    pub(crate) blocks: BTreeMap<u64, Block>,
    free_bins: BTreeMap<u32, BTreeSet<u64>>,
    allocations: HashMap<u64, u64>,
    next_id: u64,
}

impl TlsfSuballocator {
    pub(crate) fn new(size: u64) -> Self {
        let mut this = Self {
            size,
            blocks: BTreeMap::new(),
            free_bins: BTreeMap::new(),
            allocations: HashMap::new(),
            next_id: 1,
        };
        this.insert_free_block(0, size);
        this
    }

    pub(crate) fn allocate(
        &mut self,
        requested_size: u64,
        alignment: u64,
    ) -> Option<Suballocation> {
        if requested_size == 0 || requested_size > self.size || self.next_id == u64::MAX {
            return None;
        }
        let alignment = alignment.max(1);
        let (block_offset, block, aligned_offset) =
            self.find_candidate_block(requested_size, alignment)?;
        let padding = aligned_offset.checked_sub(block_offset)?;

        self.remove_free_block(block_offset);
        self.blocks.remove(&block_offset);

        if padding > 0 {
            self.insert_free_block(block_offset, padding);
        }

        let allocation_offset = aligned_offset;
        let mut allocation_size = requested_size;
        self.blocks.insert(
            allocation_offset,
            Block {
                size: allocation_size,
                free: false,
            },
        );

        let used_end = allocation_offset.checked_add(allocation_size)?;
        let block_end = block_offset.checked_add(block.size)?;
        if block_end > used_end {
            let tail_size = block_end - used_end;
            if tail_size >= MIN_SPLIT_SIZE {
                self.insert_free_block(used_end, tail_size);
            } else if let Some(allocated) = self.blocks.get_mut(&allocation_offset) {
                allocated.size += tail_size;
                allocation_size += tail_size;
            }
        }

        let id = self.next_id;
        self.next_id += 1;
        self.allocations.insert(id, allocation_offset);
        Some(Suballocation {
            id,
            offset: allocation_offset,
            size: allocation_size,
        })
    }

    fn find_candidate_block(
        &self,
        requested_size: u64,
        alignment: u64,
    ) -> Option<(u64, Block, u64)> {
        let min_bin = bin_index(requested_size);
        for (_, offsets) in self.free_bins.range(min_bin..) {
            for block_offset in offsets {
                let Some(block) = self.blocks.get(block_offset).copied() else {
                    continue;
                };
                if !block.free {
                    continue;
                }
                let Some(aligned_offset) = align_up(*block_offset, alignment) else {
                    continue;
                };
                let Some(padding) = aligned_offset.checked_sub(*block_offset) else {
                    continue;
                };
                let Some(needed) = padding.checked_add(requested_size) else {
                    continue;
                };
                if needed <= block.size {
                    return Some((*block_offset, block, aligned_offset));
                }
            }
        }
        None
    }

    pub(crate) fn allocation_offset(&self, id: u64) -> Option<u64> {
        self.allocations.get(&id).copied()
    }

    pub(crate) fn free(&mut self, id: u64) -> Option<()> {
        let offset = self.allocations.remove(&id)?;
        let block = self.blocks.remove(&offset)?;
        if block.free {
            return None;
        }

        let mut merged_offset = offset;
        let mut merged_size = block.size;

        if let Some((&prev_offset, prev_block)) = self.blocks.range(..offset).next_back()
            && prev_block.free
            && prev_offset.checked_add(prev_block.size) == Some(offset)
        {
            let prev = *prev_block;
            self.remove_free_block(prev_offset);
            self.blocks.remove(&prev_offset);
            merged_offset = prev_offset;
            merged_size += prev.size;
        }

        let next_offset = merged_offset.checked_add(merged_size)?;
        if let Some(next_block) = self.blocks.get(&next_offset).copied()
            && next_block.free
        {
            self.remove_free_block(next_offset);
            self.blocks.remove(&next_offset);
            merged_size += next_block.size;
        }

        self.insert_free_block(merged_offset, merged_size);
        Some(())
    }

    pub(crate) fn stats(&self) -> SuballocatorStats {
        let mut stats = SuballocatorStats {
            allocated_bytes: 0,
            free_bytes: 0,
            allocation_count: self.allocations.len(),
        };
        for block in self.blocks.values() {
            if block.free {
                stats.free_bytes += block.size;
            } else {
                stats.allocated_bytes += block.size;
            }
        }
        stats
    }

    pub(crate) fn is_wholly_free(&self, expected_size: u64) -> bool {
        self.allocations.is_empty()
            && self.blocks.len() == 1
            && self
                .blocks
                .get(&0)
                .is_some_and(|block| block.free && block.size == expected_size)
    }

    pub(crate) fn region_snapshots(
        &self,
        mapped_allocations: &HashMap<u64, usize>,
    ) -> Vec<MemoryRegionSnapshot> {
        let allocation_ids_by_offset = self
            .allocations
            .iter()
            .map(|(&id, &offset)| (offset, id))
            .collect::<HashMap<_, _>>();
        self.blocks
            .iter()
            .map(|(&offset, block)| {
                let state = if block.free {
                    MemoryRegionState::Free
                } else {
                    let allocation_id = allocation_ids_by_offset[&offset];
                    MemoryRegionState::Allocated {
                        allocation_id,
                        mapped_count: mapped_allocations.get(&allocation_id).copied().unwrap_or(0),
                    }
                };
                MemoryRegionSnapshot {
                    offset,
                    size: block.size,
                    state,
                }
            })
            .collect()
    }

    fn insert_free_block(&mut self, offset: u64, size: u64) {
        self.blocks.insert(offset, Block { size, free: true });
        self.free_bins
            .entry(bin_index(size))
            .or_default()
            .insert(offset);
    }

    fn remove_free_block(&mut self, offset: u64) {
        if let Some(block) = self.blocks.get(&offset) {
            let bin = bin_index(block.size);
            if let Some(offsets) = self.free_bins.get_mut(&bin) {
                offsets.remove(&offset);
                if offsets.is_empty() {
                    self.free_bins.remove(&bin);
                }
            }
        }
    }
}

fn bin_index(size: u64) -> u32 {
    63 - size.max(1).leading_zeros()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn check_invariants(allocator: &TlsfSuballocator) {
        let mut total = 0;
        let mut last_end = 0;
        let mut previous_free = false;
        for (&offset, block) in &allocator.blocks {
            assert_eq!(offset, last_end);
            assert!(block.size > 0);
            assert!(!(previous_free && block.free));
            if block.free {
                let bin = bin_index(block.size);
                assert!(
                    allocator
                        .free_bins
                        .get(&bin)
                        .is_some_and(|offsets| offsets.contains(&offset))
                );
            }
            previous_free = block.free;
            total += block.size;
            last_end = offset + block.size;
        }
        assert_eq!(total, allocator.size);

        for (&bin, offsets) in &allocator.free_bins {
            for offset in offsets {
                let block = allocator
                    .blocks
                    .get(offset)
                    .expect("free bin contains an offset with no block");
                assert!(block.free);
                assert_eq!(bin_index(block.size), bin);
            }
        }
    }

    #[test]
    fn allocates_aligned_block() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(128, 256).unwrap();
        assert_eq!(allocation.offset % 256, 0);
        assert!(allocation.size >= 128);
        check_invariants(&allocator);
    }

    #[test]
    fn rejects_zero_size_and_too_large_allocations() {
        let mut allocator = TlsfSuballocator::new(1024);
        let before = allocator.clone();
        assert!(allocator.allocate(0, 1).is_none());
        assert!(allocator.allocate(2048, 1).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);
        check_invariants(&allocator);
    }

    #[test]
    fn exact_fit_consumes_whole_block() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(1024, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.size, 1024);
        assert!(allocator.allocate(1, 1).is_none());
        let stats = allocator.stats();
        assert_eq!(stats.allocated_bytes, 1024);
        assert_eq!(stats.free_bytes, 0);
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn tiny_tail_is_absorbed_into_allocation() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(900, 1).unwrap();
        assert_eq!(allocation.size, 1024);
        assert_eq!(allocator.stats().free_bytes, 0);
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn alignment_prefix_is_split_and_remerged() {
        let mut allocator = TlsfSuballocator::new(2048);
        let first = allocator.allocate(60, 1).unwrap();
        let aligned = allocator.allocate(128, 256).unwrap();
        assert_eq!(aligned.offset, 256);
        assert!(allocator.blocks.contains_key(&60));
        allocator.free(first.id).unwrap();
        allocator.free(aligned.id).unwrap();
        assert_eq!(allocator.blocks.len(), 1);
        assert_eq!(allocator.stats().free_bytes, 2048);
        check_invariants(&allocator);
    }

    #[test]
    fn reuses_freed_space() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(256, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        allocator.free(a.id).unwrap();
        let c = allocator.allocate(128, 1).unwrap();
        assert_eq!(c.offset, a.offset);
        allocator.free(b.id).unwrap();
        allocator.free(c.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn merges_previous_and_next_blocks() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let b = allocator.allocate(128, 1).unwrap();
        let c = allocator.allocate(128, 1).unwrap();
        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        allocator.free(b.id).unwrap();
        assert_eq!(allocator.blocks.len(), 1);
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn detects_double_free() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        allocator.free(a.id).unwrap();
        assert!(allocator.free(a.id).is_none());
        check_invariants(&allocator);
    }

    #[test]
    fn invalid_free_does_not_change_stats() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let before = allocator.stats();
        assert!(allocator.free(a.id + 1).is_none());
        assert_eq!(allocator.stats(), before);
        allocator.free(a.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn reports_stats_after_fragmentation() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        allocator.free(a.id).unwrap();
        let stats = allocator.stats();
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.allocated_bytes, b.size);
        assert_eq!(stats.allocated_bytes + stats.free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn non_power_of_two_alignment_is_supported() {
        let mut allocator = TlsfSuballocator::new(4096);
        let first = allocator.allocate(17, 1).unwrap();
        let a = allocator.allocate(17, 192).unwrap();
        assert_eq!(a.offset % 192, 0);
        allocator.free(first.id).unwrap();
        allocator.free(a.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 4096);
        check_invariants(&allocator);
    }

    #[test]
    fn continues_after_alignment_unusable_candidate() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(32, 1).unwrap();
        let b = allocator.allocate(224, 1).unwrap();
        let c = allocator.allocate(32, 1).unwrap();
        allocator.free(b.id).unwrap();

        let allocation = allocator.allocate(200, 256).unwrap();
        assert!(allocation.offset >= c.offset + c.size);
        assert_eq!(allocation.offset % 256, 0);
        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn failed_fragmented_allocation_does_not_mutate_state() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(256, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        let c = allocator.allocate(256, 1).unwrap();
        let d = allocator.allocate(256, 1).unwrap();
        allocator.free(b.id).unwrap();
        allocator.free(d.id).unwrap();

        let before = allocator.clone();
        assert!(allocator.allocate(384, 1).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);

        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn failed_impossible_alignment_does_not_mutate_state() {
        let mut allocator = TlsfSuballocator::new(128);
        let first = allocator.allocate(1, 1).unwrap();
        let before = allocator.clone();
        assert!(allocator.allocate(127, 128).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);
        allocator.free(first.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn stale_free_bin_entry_does_not_abort_search() {
        let mut allocator = TlsfSuballocator::new(1024);
        allocator
            .free_bins
            .entry(bin_index(64))
            .or_default()
            .insert(999);
        let allocation = allocator.allocate(64, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        allocator
            .free_bins
            .get_mut(&bin_index(64))
            .unwrap()
            .remove(&999);
        allocator.free(allocation.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn randomized_model_stays_consistent() {
        let mut allocator = TlsfSuballocator::new(8192);
        let mut live = Vec::new();
        let mut seed = 0x1234_5678_u64;
        for _ in 0..2000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            if seed & 1 == 0 || live.is_empty() {
                let size = 1 + ((seed >> 8) % 512);
                let alignment = 1 << ((seed >> 20) % 8);
                if let Some(allocation) = allocator.allocate(size, alignment) {
                    assert_eq!(allocation.offset % alignment, 0);
                    live.push(allocation);
                }
            } else {
                let index = (seed as usize) % live.len();
                let allocation = live.swap_remove(index);
                allocator.free(allocation.id).unwrap();
            }
            check_invariants(&allocator);
            let stats = allocator.stats();
            assert_eq!(stats.allocated_bytes + stats.free_bytes, allocator.size);
            assert_eq!(stats.allocation_count, live.len());
        }
    }

    #[test]
    fn huge_suballocator_bounds_do_not_overflow() {
        let mut allocator = TlsfSuballocator::new(u64::MAX);
        let allocation = allocator.allocate(u64::MAX, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.size, u64::MAX);
        assert!(allocator.allocate(1, 1).is_none());
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, u64::MAX);
    }

    #[test]
    fn rejects_allocation_id_overflow_without_mutating_state() {
        let mut allocator = TlsfSuballocator::new(1024);
        allocator.next_id = u64::MAX;
        assert!(allocator.allocate(128, 1).is_none());
        assert_eq!(allocator.stats().free_bytes, 1024);
        assert_eq!(allocator.stats().allocation_count, 0);
        assert!(allocator.allocations.is_empty());
        check_invariants(&allocator);
    }
}
