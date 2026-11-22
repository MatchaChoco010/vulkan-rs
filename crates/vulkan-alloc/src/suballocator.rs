use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::align::align_up;
use crate::report::{MemoryRegionSnapshot, MemoryRegionState};

const MIN_SPLIT_SIZE: u64 = 256;
const SECOND_LEVEL_BITS: u32 = 5;
const SECOND_LEVEL_COUNT: u32 = 1 << SECOND_LEVEL_BITS;
const NO_REGION: u32 = u32::MAX;

static NEXT_ALLOCATION_ID: AtomicU64 = AtomicU64::new(1);

fn take_allocation_id(next: &AtomicU64) -> Option<u64> {
    next.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
        .ok()
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SizeClass {
    first: u32,
    second: u32,
}

impl SizeClass {
    fn containing(size: u64) -> Self {
        if size < u64::from(SECOND_LEVEL_COUNT) {
            return Self {
                first: 0,
                second: size as u32,
            };
        }
        let top = 63 - size.leading_zeros();
        Self {
            first: top - SECOND_LEVEL_BITS + 1,
            second: (size >> (top - SECOND_LEVEL_BITS)) as u32 & (SECOND_LEVEL_COUNT - 1),
        }
    }

    fn next(self) -> Self {
        if self.second + 1 < SECOND_LEVEL_COUNT {
            Self {
                first: self.first,
                second: self.second + 1,
            }
        } else {
            Self {
                first: self.first + 1,
                second: 0,
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ListSearch {
    Heads,
    Whole,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RegionState {
    Vacant,
    Free { previous: u32, next: u32 },
    Allocated { id: u64 },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Region {
    offset: u64,
    size: u64,
    previous: u32,
    next: u32,
    state: RegionState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TlsfSuballocator {
    size: u64,
    regions: Vec<Region>,
    vacant: Vec<u32>,
    first_level: u64,
    second_levels: Vec<u32>,
    free_lists: Vec<u32>,
    allocations: HashMap<u64, u32>,
    allocated_bytes: u64,
}

impl TlsfSuballocator {
    pub(crate) fn new(size: u64) -> Self {
        let first_levels = if size == 0 {
            0
        } else {
            SizeClass::containing(size).first as usize + 1
        };
        let mut this = Self {
            size,
            regions: Vec::new(),
            vacant: Vec::new(),
            first_level: 0,
            second_levels: vec![0; first_levels],
            free_lists: vec![NO_REGION; first_levels * SECOND_LEVEL_COUNT as usize],
            allocations: HashMap::new(),
            allocated_bytes: 0,
        };
        if size > 0 {
            this.regions.push(Region {
                offset: 0,
                size,
                previous: NO_REGION,
                next: NO_REGION,
                state: RegionState::Vacant,
            });
            this.insert_free(0);
        }
        this
    }

    pub(crate) fn allocate(
        &mut self,
        requested_size: u64,
        alignment: u64,
    ) -> Option<Suballocation> {
        self.allocate_with_id_from(&NEXT_ALLOCATION_ID, requested_size, alignment)
    }

    fn allocate_with_id_from(
        &mut self,
        next_id: &AtomicU64,
        requested_size: u64,
        alignment: u64,
    ) -> Option<Suballocation> {
        if requested_size == 0 || requested_size > self.size {
            return None;
        }
        let (index, offset) = self.find_free_region(requested_size, alignment.max(1))?;
        if !self.has_room_for_regions(2) {
            return None;
        }
        let id = take_allocation_id(next_id)?;

        let found = self.regions[index as usize];
        let padding = offset - found.offset;
        let tail = found.size - padding - requested_size;
        self.remove_free(index);
        let allocated = if padding > 0 {
            self.regions[index as usize].size = padding;
            self.insert_free(index);
            self.insert_region_after(index, offset, found.size - padding)
        } else {
            index
        };
        let size = if tail >= MIN_SPLIT_SIZE {
            let rest = self.insert_region_after(allocated, offset + requested_size, tail);
            self.insert_free(rest);
            requested_size
        } else {
            requested_size + tail
        };
        let region = &mut self.regions[allocated as usize];
        region.size = size;
        region.state = RegionState::Allocated { id };
        self.allocations.insert(id, allocated);
        self.allocated_bytes += size;
        Some(Suballocation { id, offset, size })
    }

    fn find_free_region(&self, size: u64, alignment: u64) -> Option<(u32, u64)> {
        self.find_in_free_lists(size, alignment, ListSearch::Heads)
            .or_else(|| self.find_in_free_lists(size, alignment, ListSearch::Whole))
    }

    fn find_in_free_lists(
        &self,
        size: u64,
        alignment: u64,
        search: ListSearch,
    ) -> Option<(u32, u64)> {
        let mut class = self.first_free_list_from(SizeClass::containing(size));
        while let Some(current) = class {
            let mut index = self.free_lists[Self::list_index(current)];
            while index != NO_REGION {
                if let Some(offset) = self.placement(index, size, alignment) {
                    return Some((index, offset));
                }
                index = match (search, self.regions[index as usize].state) {
                    (ListSearch::Whole, RegionState::Free { next, .. }) => next,
                    _ => NO_REGION,
                };
            }
            class = self.first_free_list_from(current.next());
        }
        None
    }

    fn placement(&self, index: u32, size: u64, alignment: u64) -> Option<u64> {
        let region = &self.regions[index as usize];
        let offset = align_up(region.offset, alignment)?;
        let needed = (offset - region.offset).checked_add(size)?;
        (needed <= region.size).then_some(offset)
    }

    fn first_free_list_from(&self, class: SizeClass) -> Option<SizeClass> {
        let seconds = *self.second_levels.get(class.first as usize)?;
        let in_this_level = seconds & (u32::MAX << class.second);
        if in_this_level != 0 {
            return Some(SizeClass {
                first: class.first,
                second: in_this_level.trailing_zeros(),
            });
        }
        let in_higher_levels =
            self.first_level & u64::MAX.checked_shl(class.first + 1).unwrap_or(0);
        if in_higher_levels == 0 {
            return None;
        }
        let first = in_higher_levels.trailing_zeros();
        Some(SizeClass {
            first,
            second: self.second_levels[first as usize].trailing_zeros(),
        })
    }

    fn list_index(class: SizeClass) -> usize {
        (class.first * SECOND_LEVEL_COUNT + class.second) as usize
    }

    fn has_room_for_regions(&self, count: usize) -> bool {
        self.vacant.len() + (NO_REGION as usize - self.regions.len()) >= count
    }

    fn insert_region_after(&mut self, existing: u32, offset: u64, size: u64) -> u32 {
        let next = self.regions[existing as usize].next;
        let region = Region {
            offset,
            size,
            previous: existing,
            next,
            state: RegionState::Vacant,
        };
        let index = match self.vacant.pop() {
            Some(index) => {
                self.regions[index as usize] = region;
                index
            }
            None => {
                self.regions.push(region);
                (self.regions.len() - 1) as u32
            }
        };
        self.regions[existing as usize].next = index;
        if next != NO_REGION {
            self.regions[next as usize].previous = index;
        }
        index
    }

    fn remove_region_merged_into_its_predecessor(&mut self, index: u32) {
        let Region { previous, next, .. } = self.regions[index as usize];
        self.regions[previous as usize].next = next;
        if next != NO_REGION {
            self.regions[next as usize].previous = previous;
        }
        self.regions[index as usize].state = RegionState::Vacant;
        self.vacant.push(index);
    }

    fn insert_free(&mut self, index: u32) {
        let class = SizeClass::containing(self.regions[index as usize].size);
        let list = Self::list_index(class);
        let head = self.free_lists[list];
        self.regions[index as usize].state = RegionState::Free {
            previous: NO_REGION,
            next: head,
        };
        if head != NO_REGION
            && let RegionState::Free { previous, .. } = &mut self.regions[head as usize].state
        {
            *previous = index;
        }
        self.free_lists[list] = index;
        self.second_levels[class.first as usize] |= 1 << class.second;
        self.first_level |= 1 << class.first;
    }

    fn remove_free(&mut self, index: u32) {
        let RegionState::Free { previous, next } = self.regions[index as usize].state else {
            return;
        };
        let class = SizeClass::containing(self.regions[index as usize].size);
        if previous != NO_REGION {
            if let RegionState::Free { next: link, .. } = &mut self.regions[previous as usize].state
            {
                *link = next;
            }
        } else {
            self.free_lists[Self::list_index(class)] = next;
        }
        if next != NO_REGION
            && let RegionState::Free { previous: link, .. } = &mut self.regions[next as usize].state
        {
            *link = previous;
        }
        if self.free_lists[Self::list_index(class)] == NO_REGION {
            self.second_levels[class.first as usize] &= !(1 << class.second);
            if self.second_levels[class.first as usize] == 0 {
                self.first_level &= !(1 << class.first);
            }
        }
        self.regions[index as usize].state = RegionState::Vacant;
    }

    fn is_free(&self, index: u32) -> bool {
        index != NO_REGION && matches!(self.regions[index as usize].state, RegionState::Free { .. })
    }

    pub(crate) fn allocation(&self, id: u64) -> Option<Suballocation> {
        let region = &self.regions[*self.allocations.get(&id)? as usize];
        Some(Suballocation {
            id,
            offset: region.offset,
            size: region.size,
        })
    }

    pub(crate) fn free(&mut self, id: u64) -> Option<()> {
        let mut index = self.allocations.remove(&id)?;
        self.allocated_bytes -= self.regions[index as usize].size;

        let next = self.regions[index as usize].next;
        if self.is_free(next) {
            self.remove_free(next);
            self.regions[index as usize].size += self.regions[next as usize].size;
            self.remove_region_merged_into_its_predecessor(next);
        }
        let previous = self.regions[index as usize].previous;
        if self.is_free(previous) {
            self.remove_free(previous);
            self.regions[previous as usize].size += self.regions[index as usize].size;
            self.remove_region_merged_into_its_predecessor(index);
            index = previous;
        }
        self.insert_free(index);
        Some(())
    }

    pub(crate) fn stats(&self) -> SuballocatorStats {
        SuballocatorStats {
            allocated_bytes: self.allocated_bytes,
            free_bytes: self.size - self.allocated_bytes,
            allocation_count: self.allocations.len(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.allocations.is_empty()
    }

    fn first_region(&self) -> u32 {
        if self.regions.is_empty() {
            NO_REGION
        } else {
            0
        }
    }

    pub(crate) fn region_snapshots(
        &self,
        mapped_allocations: &HashMap<u64, usize>,
    ) -> Vec<MemoryRegionSnapshot> {
        let mut snapshots = Vec::new();
        let mut index = self.first_region();
        while index != NO_REGION {
            let region = &self.regions[index as usize];
            let state = match region.state {
                RegionState::Allocated { id } => MemoryRegionState::Allocated {
                    allocation_id: id,
                    mapped_count: mapped_allocations.get(&id).copied().unwrap_or(0),
                },
                RegionState::Free { .. } | RegionState::Vacant => MemoryRegionState::Free,
            };
            snapshots.push(MemoryRegionSnapshot {
                offset: region.offset,
                size: region.size,
                state,
            });
            index = region.next;
        }
        snapshots
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    impl SizeClass {
        fn smallest_size(self) -> u64 {
            if self.first == 0 {
                return u64::from(self.second);
            }
            let top = self.first + SECOND_LEVEL_BITS - 1;
            (1 << top) | (u64::from(self.second) << (top - SECOND_LEVEL_BITS))
        }
    }

    fn layout(allocator: &TlsfSuballocator) -> Vec<MemoryRegionSnapshot> {
        allocator.region_snapshots(&HashMap::new())
    }

    fn free_regions(allocator: &TlsfSuballocator) -> Vec<(u64, u64)> {
        layout(allocator)
            .into_iter()
            .filter(|region| region.state == MemoryRegionState::Free)
            .map(|region| (region.offset, region.size))
            .collect()
    }

    pub(crate) fn check_invariants(allocator: &TlsfSuballocator) {
        let mut end = 0;
        let mut previous = NO_REGION;
        let mut previous_free = false;
        let mut index = allocator.first_region();
        let mut linked = 0;
        let mut allocated_bytes = 0;
        let mut allocated = 0;
        let mut free = Vec::new();
        while index != NO_REGION {
            let region = &allocator.regions[index as usize];
            assert_eq!(region.offset, end);
            assert_eq!(region.previous, previous);
            assert!(region.size > 0);
            match region.state {
                RegionState::Vacant => panic!("a linked region is vacant"),
                RegionState::Free { .. } => {
                    assert!(!previous_free, "two free regions are adjacent");
                    free.push(index);
                    previous_free = true;
                }
                RegionState::Allocated { id } => {
                    assert_eq!(allocator.allocations.get(&id), Some(&index));
                    allocated_bytes += region.size;
                    allocated += 1;
                    previous_free = false;
                }
            }
            end = region.offset + region.size;
            previous = index;
            index = region.next;
            linked += 1;
            assert!(
                linked <= allocator.regions.len(),
                "the regions form a cycle"
            );
        }
        assert_eq!(end, allocator.size);
        assert_eq!(allocated, allocator.allocations.len());
        assert_eq!(allocated_bytes, allocator.allocated_bytes);
        assert_eq!(linked + allocator.vacant.len(), allocator.regions.len());
        for &index in &allocator.vacant {
            assert_eq!(allocator.regions[index as usize].state, RegionState::Vacant);
        }
        let mut vacant = allocator.vacant.clone();
        vacant.sort_unstable();
        vacant.dedup();
        assert_eq!(
            vacant.len(),
            allocator.vacant.len(),
            "a vacant slot is listed twice"
        );

        let mut listed = Vec::new();
        for (list, &head) in allocator.free_lists.iter().enumerate() {
            let class = SizeClass {
                first: list as u32 / SECOND_LEVEL_COUNT,
                second: list as u32 % SECOND_LEVEL_COUNT,
            };
            let second_bit = allocator.second_levels[class.first as usize] >> class.second & 1;
            assert_eq!(second_bit == 1, head != NO_REGION);
            let mut previous = NO_REGION;
            let mut index = head;
            while index != NO_REGION {
                let region = &allocator.regions[index as usize];
                assert_eq!(SizeClass::containing(region.size), class);
                let RegionState::Free {
                    previous: link,
                    next,
                } = region.state
                else {
                    panic!("a region in a free list is not free");
                };
                assert_eq!(link, previous);
                listed.push(index);
                assert!(
                    listed.len() <= allocator.regions.len(),
                    "a free list forms a cycle"
                );
                previous = index;
                index = next;
            }
        }
        for (first, &seconds) in allocator.second_levels.iter().enumerate() {
            assert_eq!(allocator.first_level >> first & 1 == 1, seconds != 0);
        }
        listed.sort_unstable();
        free.sort_unstable();
        assert_eq!(listed, free);
    }

    #[test]
    fn size_classes_split_each_power_of_two_into_equal_ranges() {
        let mut sizes = (1..5000_u64).collect::<Vec<_>>();
        for bit in 5..64 {
            let power = 1_u64 << bit;
            let step = power >> SECOND_LEVEL_BITS;
            for second in 0..u64::from(SECOND_LEVEL_COUNT) {
                let start = power + second * step;
                sizes.extend([start, start + step / 2, start + (step - 1)]);
            }
        }
        sizes.push(u64::MAX);
        sizes.sort_unstable();
        sizes.dedup();

        let mut previous = SizeClass::containing(sizes[0]);
        for &size in &sizes {
            let class = SizeClass::containing(size);
            assert!(class.second < SECOND_LEVEL_COUNT);
            assert!(class.smallest_size() <= size, "size {size}");
            assert!(class >= previous, "size {size}");
            if size >= u64::from(SECOND_LEVEL_COUNT) {
                let power = 1_u64 << (63 - size.leading_zeros());
                let step = power >> SECOND_LEVEL_BITS;
                assert_eq!(class.smallest_size(), size - (size - power) % step);
                assert_eq!(u64::from(class.second), (size - power) / step);
            } else {
                assert_eq!(class.smallest_size(), size);
            }
            if size < u64::MAX {
                let next = SizeClass::containing(size + 1);
                assert!(next == class || next == class.next(), "size {size}");
            }
            previous = class;
        }
        assert_eq!(
            SizeClass::containing(u64::MAX),
            SizeClass {
                first: 64 - SECOND_LEVEL_BITS,
                second: SECOND_LEVEL_COUNT - 1
            }
        );
    }

    #[test]
    fn free_regions_are_listed_by_both_levels_of_their_size() {
        let mut allocator = TlsfSuballocator::new(1 << 20);
        let sizes = [1024, 1024 + 32, 1024 + 512, 2048, 2048 + 64, 300, 308];
        let mut kept = Vec::new();
        let mut released = Vec::new();
        for size in sizes {
            released.push(allocator.allocate(size, 1).unwrap());
            kept.push(allocator.allocate(1, 1).unwrap());
        }
        for allocation in &released {
            allocator.free(allocation.id).unwrap();
        }
        check_invariants(&allocator);

        let classes = released
            .iter()
            .map(|allocation| SizeClass::containing(allocation.size))
            .collect::<Vec<_>>();
        for (index, class) in classes.iter().enumerate() {
            assert_eq!(
                allocator.free_lists[TlsfSuballocator::list_index(*class)],
                allocator.allocations_region_before(kept[index].id),
                "the region of {} bytes is the only one of its class",
                sizes[index]
            );
            assert!(!classes[..index].contains(class));
        }
        assert_eq!(classes[0].first, classes[1].first);
        assert_eq!(classes[0].first, classes[2].first);
        assert_ne!(classes[0].first, classes[3].first);
    }

    impl TlsfSuballocator {
        fn allocations_region_before(&self, id: u64) -> u32 {
            self.regions[self.allocations[&id] as usize].previous
        }
    }

    #[test]
    fn a_request_takes_the_first_list_whose_head_fits() {
        let mut allocator = TlsfSuballocator::new(1 << 20);
        let mut small = Vec::new();
        for _ in 0..64 {
            small.push(allocator.allocate(300, 1).unwrap());
            allocator.allocate(1, 1).unwrap();
        }
        let large = allocator.allocate(4096, 1).unwrap();
        allocator.allocate(1, 1).unwrap();
        for allocation in small {
            allocator.free(allocation.id).unwrap();
        }
        allocator.free(large.id).unwrap();
        check_invariants(&allocator);

        let allocation = allocator.allocate(290, 16).unwrap();
        assert_eq!(allocation.offset, large.offset);
        check_invariants(&allocator);
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
        assert_eq!(allocator, before);
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
    fn a_whole_block_is_allocated_with_any_alignment() {
        for alignment in [1, 2, 64, 4096, 1 << 20, 1 << 40, 3, 4097] {
            let size = 64 * 1024 * 1024;
            let mut allocator = TlsfSuballocator::new(size);
            let allocation = allocator.allocate(size, alignment).unwrap();
            assert_eq!((allocation.offset, allocation.size), (0, size));
            check_invariants(&allocator);
        }
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
        assert!(free_regions(&allocator).contains(&(60, 196)));
        allocator.free(first.id).unwrap();
        allocator.free(aligned.id).unwrap();
        assert_eq!(free_regions(&allocator), [(0, 2048)]);
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
        assert_eq!(free_regions(&allocator), [(0, 1024)]);
        assert!(allocator.is_empty());
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn a_freed_range_is_reused_by_the_next_request_of_its_size() {
        for (size, alignment) in [(300, 1), (1 << 20, 4096), (4096, 4096), (77, 16)] {
            let mut allocator = TlsfSuballocator::new(64 << 20);
            let _before = allocator.allocate(size, alignment).unwrap();
            let middle = allocator.allocate(size, alignment).unwrap();
            let _after = allocator.allocate(size, alignment).unwrap();
            allocator.free(middle.id).unwrap();
            let again = allocator.allocate(size, alignment).unwrap();
            assert_eq!(
                again.offset, middle.offset,
                "{size} bytes aligned to {alignment}"
            );
            check_invariants(&allocator);
        }
    }

    #[test]
    fn detects_double_free() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        allocator.free(a.id).unwrap();
        let before = allocator.clone();
        assert!(allocator.free(a.id).is_none());
        assert_eq!(allocator, before);
        check_invariants(&allocator);
    }

    #[test]
    fn invalid_free_does_not_change_stats() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let before = allocator.clone();
        assert!(allocator.free(a.id + 1).is_none());
        assert_eq!(allocator, before);
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
        assert_eq!(allocator, before);

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
        assert_eq!(allocator, before);
        allocator.free(first.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn allocation_succeeds_exactly_when_a_free_region_can_hold_it() {
        let mut seed = 0x1234_5678_9abc_def1_u64;
        let mut next = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for block_size in [1 << 16, 100_000, 1 << 24] {
            let mut allocator = TlsfSuballocator::new(block_size);
            let mut live: Vec<Suballocation> = Vec::new();
            let mut refused = 0;
            for step in 0..6000 {
                if live.is_empty() || next() % 3 != 0 {
                    let size = match next() % 4 {
                        0 => 1 + next() % 64,
                        1 => 1 + next() % 2048,
                        2 => 1 + next() % (block_size / 8),
                        _ => 1 + next() % block_size,
                    };
                    let alignment = match next() % 4 {
                        0 => 1,
                        1 => 1 << (next() % 13),
                        2 => 1 + next() % 300,
                        _ => 1 << (next() % 20),
                    };
                    let mut taken = live
                        .iter()
                        .map(|allocation| (allocation.offset, allocation.size))
                        .collect::<Vec<_>>();
                    taken.sort_unstable();
                    let mut free_ranges = Vec::new();
                    let mut end = 0;
                    for (offset, length) in taken {
                        if offset > end {
                            free_ranges.push((end, offset - end));
                        }
                        end = offset + length;
                    }
                    if block_size > end {
                        free_ranges.push((end, block_size - end));
                    }
                    assert_eq!(free_regions(&allocator), free_ranges, "step {step}");
                    let fits = free_ranges.iter().any(|&(offset, length)| {
                        let aligned = offset.next_multiple_of(alignment);
                        aligned - offset + size <= length
                    });
                    let before = allocator.clone();
                    match allocator.allocate(size, alignment) {
                        Some(allocation) => {
                            assert!(fits, "step {step}: {size} bytes aligned to {alignment}");
                            assert_eq!(allocation.offset % alignment, 0);
                            assert!(allocation.size >= size);
                            assert!(allocation.size - size < MIN_SPLIT_SIZE);
                            let was_free = free_ranges.iter().any(|&(offset, length)| {
                                offset <= allocation.offset
                                    && allocation.offset + allocation.size <= offset + length
                            });
                            assert!(was_free, "step {step}: the range was not free");
                            assert_eq!(allocator.allocation(allocation.id), Some(allocation));
                            live.push(allocation);
                        }
                        None => {
                            assert!(!fits, "step {step}: {size} bytes aligned to {alignment}");
                            assert_eq!(allocator, before);
                            refused += 1;
                        }
                    }
                } else {
                    let index = (next() % live.len() as u64) as usize;
                    let allocation = live.swap_remove(index);
                    allocator.free(allocation.id).unwrap();
                    assert_eq!(allocator.allocation(allocation.id), None);
                }
                check_invariants(&allocator);
                for allocation in &live {
                    assert_eq!(allocator.allocation(allocation.id), Some(*allocation));
                }
                let stats = allocator.stats();
                assert_eq!(stats.allocated_bytes + stats.free_bytes, block_size);
                assert_eq!(stats.allocation_count, live.len());
                assert_eq!(
                    stats.allocated_bytes,
                    live.iter().map(|allocation| allocation.size).sum::<u64>()
                );
            }
            assert!(
                refused > 100,
                "the block of {block_size} bytes never filled up"
            );
            for allocation in live {
                allocator.free(allocation.id).unwrap();
            }
            assert_eq!(free_regions(&allocator), [(0, block_size)]);
            assert!(allocator.is_empty());
            check_invariants(&allocator);
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

        allocator.allocate(1, 1).unwrap();
        let high = allocator.allocate(1 << 62, 1 << 63).unwrap();
        assert_eq!(high.offset, 1 << 63);
        assert!(allocator.allocate(u64::MAX, u64::MAX).is_none());
        assert!(allocator.allocate(1 << 62, 1 << 63).is_none());
        assert!(allocator.allocate(1 << 62, 1 << 62).is_some());
        check_invariants(&allocator);
    }

    #[test]
    fn an_empty_suballocator_refuses_everything() {
        let mut allocator = TlsfSuballocator::new(0);
        assert!(allocator.allocate(1, 1).is_none());
        assert!(layout(&allocator).is_empty());
        assert!(allocator.is_empty());
        check_invariants(&allocator);
    }

    #[test]
    fn rejects_allocation_id_overflow_without_mutating_state() {
        let mut allocator = TlsfSuballocator::new(1024);
        let before = allocator.clone();
        let exhausted = AtomicU64::new(u64::MAX);
        assert!(
            allocator
                .allocate_with_id_from(&exhausted, 128, 1)
                .is_none()
        );
        assert_eq!(exhausted.load(Ordering::Relaxed), u64::MAX);
        assert_eq!(allocator, before);
        check_invariants(&allocator);
    }

    #[test]
    fn allocation_ids_are_never_repeated_across_suballocators() {
        let mut seen = std::collections::HashSet::new();
        for _ in 0..64 {
            let mut allocator = TlsfSuballocator::new(1024);
            for _ in 0..4 {
                let allocation = allocator.allocate(256, 1).unwrap();
                assert!(seen.insert(allocation.id));
                allocator.free(allocation.id).unwrap();
                let allocation = allocator.allocate(256, 1).unwrap();
                assert!(seen.insert(allocation.id));
            }
        }
    }
}
