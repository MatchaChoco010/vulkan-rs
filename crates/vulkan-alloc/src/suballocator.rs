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
        let split_tail = tail >= MIN_SPLIT_SIZE;
        let size = if split_tail {
            requested_size
        } else {
            requested_size + tail
        };
        self.remove_free(index);
        let allocated = if padding > 0 {
            self.regions[index as usize].size = padding;
            self.insert_free(index);
            self.insert_region_after(index, offset, size)
        } else {
            self.regions[index as usize].size = size;
            index
        };
        if split_tail {
            let rest = self.insert_region_after(allocated, offset + size, tail);
            self.insert_free(rest);
        }
        self.regions[allocated as usize].state = RegionState::Allocated { id };
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
    use proptest::prelude::*;

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

    fn gaps(block_size: u64, live: &[Suballocation]) -> Vec<(u64, u64)> {
        let mut taken = live
            .iter()
            .map(|allocation| (allocation.offset, allocation.size))
            .collect::<Vec<_>>();
        taken.sort_unstable();
        let mut gaps = Vec::new();
        let mut end = 0;
        for (offset, length) in taken {
            if offset > end {
                gaps.push((end, offset - end));
            }
            end = offset + length;
        }
        if block_size > end {
            gaps.push((end, block_size - end));
        }
        gaps
    }

    fn fits(gap: (u64, u64), size: u64, alignment: u64) -> bool {
        let (start, length) = gap;
        let padding = (alignment - start % alignment) % alignment;
        padding
            .checked_add(size)
            .is_some_and(|needed| needed <= length)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Step {
        Allocate { size: u64, alignment: u64 },
        Free { pick: usize },
    }

    #[derive(Debug, Default)]
    struct Seen {
        placed: usize,
        refused: usize,
        refused_although_enough_bytes_are_free: usize,
        padded: usize,
        took_the_remainder: usize,
        left_a_remainder: usize,
        freed: usize,
        merged: usize,
    }

    fn step_like_the_model(
        allocator: &mut TlsfSuballocator,
        live: &mut Vec<Suballocation>,
        step: Step,
        seen: &mut Seen,
    ) -> Result<(), String> {
        let block_size = allocator.size;
        let free_before = gaps(block_size, live);
        match step {
            Step::Allocate { size, alignment } => {
                let needed = alignment.max(1);
                let fitting = size > 0 && free_before.iter().any(|&gap| fits(gap, size, needed));
                let before = allocator.clone();
                match allocator.allocate(size, alignment) {
                    None => {
                        if fitting {
                            return Err(format!(
                                "{size} bytes aligned to {alignment} fit in {free_before:?} but were refused"
                            ));
                        }
                        if *allocator != before {
                            return Err("a refused request changed the suballocator".to_string());
                        }
                        seen.refused += 1;
                        if free_before.iter().map(|gap| gap.1).sum::<u64>() >= size {
                            seen.refused_although_enough_bytes_are_free += 1;
                        }
                    }
                    Some(allocation) => {
                        if !fitting {
                            return Err(format!(
                                "{size} bytes aligned to {alignment} do not fit in {free_before:?} but got {allocation:?}"
                            ));
                        }
                        let Some(&(start, length)) =
                            free_before.iter().find(|&&(start, length)| {
                                start <= allocation.offset
                                    && allocation.offset - start <= length
                                    && allocation.size <= length - (allocation.offset - start)
                            })
                        else {
                            return Err(format!(
                                "{allocation:?} is not inside one of {free_before:?}"
                            ));
                        };
                        let padding = allocation.offset - start;
                        if allocation.offset % needed != 0 || padding >= needed {
                            return Err(format!(
                                "{allocation:?} is not at the first multiple of {needed} from {start}"
                            ));
                        }
                        let remainder = length - padding - size;
                        let expected_size = if remainder < MIN_SPLIT_SIZE {
                            size + remainder
                        } else {
                            size
                        };
                        if allocation.size != expected_size {
                            return Err(format!(
                                "{allocation:?} for {size} bytes with {remainder} bytes after it"
                            ));
                        }
                        if live.iter().any(|other| other.id == allocation.id) {
                            return Err(format!("{allocation:?} repeats a live id"));
                        }
                        let mut undone = allocator.clone();
                        undone.free(allocation.id);
                        if layout(&undone) != layout(&before) {
                            return Err(format!(
                                "freeing {allocation:?} at once does not restore the regions"
                            ));
                        }
                        seen.placed += 1;
                        seen.padded += usize::from(padding > 0);
                        seen.took_the_remainder += usize::from(allocation.size > size);
                        seen.left_a_remainder += usize::from(remainder >= MIN_SPLIT_SIZE);
                        live.push(allocation);
                    }
                }
            }
            Step::Free { pick } => {
                if live.is_empty() {
                    return Ok(());
                }
                let allocation = live.remove(pick % live.len());
                let before = allocator.clone();
                if allocator.free(u64::MAX).is_some() || *allocator != before {
                    return Err("an id that was never given out was freed".to_string());
                }
                if allocator.free(allocation.id).is_none() {
                    return Err(format!("{allocation:?} could not be freed"));
                }
                let freed = allocator.clone();
                if allocator.allocation(allocation.id).is_some()
                    || allocator.free(allocation.id).is_some()
                    || *allocator != freed
                {
                    return Err(format!("{allocation:?} is still known after it was freed"));
                }
                seen.freed += 1;
                seen.merged += usize::from(gaps(block_size, live).len() <= free_before.len());
            }
        }
        check_invariants(allocator);
        if free_regions(allocator) != gaps(block_size, live) {
            return Err(format!(
                "free regions {:?} are not the gaps between the allocations {:?}",
                free_regions(allocator),
                gaps(block_size, live)
            ));
        }
        for allocation in live.iter() {
            if allocator.allocation(allocation.id) != Some(*allocation) {
                return Err(format!("{allocation:?} is no longer reported"));
            }
        }
        let stats = allocator.stats();
        let allocated = live.iter().map(|allocation| allocation.size).sum::<u64>();
        if (
            stats.allocated_bytes,
            stats.free_bytes,
            stats.allocation_count,
        ) != (allocated, block_size - allocated, live.len())
            || allocator.is_empty() != live.is_empty()
        {
            return Err(format!(
                "{stats:?} for {} allocations of {allocated} bytes",
                live.len()
            ));
        }
        Ok(())
    }

    fn block_sizes() -> impl Strategy<Value = u64> {
        prop_oneof![
            1 => Just(0u64),
            2 => 1..=64u64,
            2 => 250..=260u64,
            4 => 1000..=5000u64,
            4 => (8u32..=20).prop_map(|shift| 1u64 << shift),
            2 => 60_000..=70_000u64,
            1 => Just(1u64 << 40),
            1 => Just(u64::MAX),
        ]
    }

    fn steps(block_size: u64) -> impl Strategy<Value = Step> {
        let near_a_size_class =
            (0u32..=63, -2i64..=2, 0u64..32).prop_map(|(top, delta, second)| {
                let power = 1u64 << top;
                (power | (second << top.saturating_sub(SECOND_LEVEL_BITS)))
                    .saturating_add_signed(delta)
            });
        let size = prop_oneof![
            5 => 1..=64u64,
            3 => 1..=2048u64,
            2 => (MIN_SPLIT_SIZE - 2)..=(MIN_SPLIT_SIZE + 2),
            3 => near_a_size_class,
            3 => 1..=(block_size / 8).max(1),
            2 => (prop::sample::select(vec![block_size / 4, block_size / 2, block_size]), 0u64..=258)
                .prop_map(|(base, less)| base.saturating_sub(less)),
            1 => Just(0u64),
            1 => any::<u64>(),
        ];
        let alignment = prop_oneof![
            6 => Just(1u64),
            1 => Just(0u64),
            5 => (1u32..=12).prop_map(|shift| 1u64 << shift),
            2 => (13u32..=63).prop_map(|shift| 1u64 << shift),
            2 => prop::sample::select(vec![3u64, 7, 12, 100, 192, 255, 1000, 4097]),
            1 => any::<u64>(),
        ];
        prop_oneof![
            3 => (size, alignment).prop_map(|(size, alignment)| Step::Allocate { size, alignment }),
            2 => any::<usize>().prop_map(|pick| Step::Free { pick }),
        ]
    }

    fn scenarios(longest: usize) -> impl Strategy<Value = (u64, Vec<Step>)> {
        block_sizes().prop_flat_map(move |block_size| {
            (
                Just(block_size),
                prop::collection::vec(steps(block_size), 0..=longest),
            )
        })
    }

    fn run(block_size: u64, steps: &[Step], seen: &mut Seen) -> Result<(), String> {
        let mut allocator = TlsfSuballocator::new(block_size);
        let mut live = Vec::new();
        check_invariants(&allocator);
        for (index, &step) in steps.iter().enumerate() {
            step_like_the_model(&mut allocator, &mut live, step, seen)
                .map_err(|problem| format!("step {index}, {step:?}: {problem}"))?;
        }
        while !live.is_empty() {
            step_like_the_model(&mut allocator, &mut live, Step::Free { pick: 0 }, seen)?;
        }
        if block_size > 0 && free_regions(&allocator) != [(0, block_size)] {
            return Err("the block is not one free region after everything was freed".to_string());
        }
        Ok(())
    }

    fn cases(default: u32) -> ProptestConfig {
        let cases = std::env::var("PROPTEST_CASES")
            .ok()
            .and_then(|cases| cases.parse().ok())
            .unwrap_or(default);
        ProptestConfig {
            cases,
            failure_persistence: None,
            ..ProptestConfig::default()
        }
    }

    proptest! {
        #![proptest_config(cases(256))]

        #[test]
        fn every_request_is_placed_exactly_when_a_free_range_can_hold_it(
            (block_size, steps) in scenarios(120)
        ) {
            run(block_size, &steps, &mut Seen::default()).map_err(TestCaseError::fail)?;
        }

        #[test]
        fn a_size_class_holds_the_sizes_between_its_smallest_size_and_the_next(size in any::<u64>()) {
            let class = SizeClass::containing(size);
            prop_assert!(class.second < SECOND_LEVEL_COUNT);
            prop_assert!(class.first <= 64 - SECOND_LEVEL_BITS);
            prop_assert!(class.smallest_size() <= size);
            if let Some(smaller) = class.smallest_size().checked_sub(1) {
                prop_assert_eq!(SizeClass::containing(smaller).next(), class);
            }
            if class != SizeClass::containing(u64::MAX) {
                prop_assert!(size < class.next().smallest_size());
                prop_assert_eq!(SizeClass::containing(class.next().smallest_size()), class.next());
            }
        }
    }

    #[test]
    fn the_generated_steps_reach_every_way_a_request_can_end() {
        use proptest::strategy::ValueTree;
        use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};

        let mut runner = TestRunner::new_with_rng(
            ProptestConfig::default(),
            TestRng::deterministic_rng(RngAlgorithm::ChaCha),
        );
        let strategy = scenarios(120);
        let mut seen = Seen::default();
        for _ in 0..300 {
            let (block_size, steps) = strategy.new_tree(&mut runner).unwrap().current();
            run(block_size, &steps, &mut seen).unwrap();
        }
        let requests = seen.placed + seen.refused;
        assert!(seen.placed * 4 > requests, "{seen:?}");
        assert!(seen.refused * 10 > requests, "{seen:?}");
        assert!(
            seen.refused_although_enough_bytes_are_free * 50 > requests,
            "{seen:?}"
        );
        assert!(seen.padded * 20 > seen.placed, "{seen:?}");
        assert!(seen.took_the_remainder * 20 > seen.placed, "{seen:?}");
        assert!(seen.left_a_remainder * 4 > seen.placed, "{seen:?}");
        assert!(seen.merged * 2 > seen.freed, "{seen:?}");
    }

    fn try_every_sequence(
        allocator: &TlsfSuballocator,
        live: &[Suballocation],
        requests: &[(u64, u64)],
        remaining: usize,
        tried: &mut usize,
    ) {
        *tried += 1;
        if remaining == 0 {
            return;
        }
        let allocations = requests
            .iter()
            .map(|&(size, alignment)| Step::Allocate { size, alignment });
        let frees = (0..live.len()).map(|pick| Step::Free { pick });
        for step in allocations.chain(frees) {
            let mut allocator = allocator.clone();
            let mut live = live.to_vec();
            step_like_the_model(&mut allocator, &mut live, step, &mut Seen::default())
                .unwrap_or_else(|problem| panic!("{step:?} after {live:?}: {problem}"));
            try_every_sequence(&allocator, &live, requests, remaining - 1, tried);
        }
    }

    #[test]
    fn every_short_sequence_of_requests_and_frees_in_a_small_block_follows_the_model() {
        let requests = [(1, 1), (255, 1), (256, 1), (300, 256), (513, 1), (40, 96)];
        let mut tried = 0;
        try_every_sequence(&TlsfSuballocator::new(1024), &[], &requests, 5, &mut tried);
        assert!(tried > 20_000, "{tried} sequences");
    }

    #[test]
    fn slow_every_sequence_of_six_requests_and_frees_in_a_small_block_follows_the_model() {
        let requests = [
            (1, 1),
            (255, 1),
            (256, 1),
            (257, 1),
            (300, 256),
            (513, 1),
            (40, 96),
            (1024, 1),
        ];
        let mut tried = 0;
        try_every_sequence(&TlsfSuballocator::new(1024), &[], &requests, 6, &mut tried);
        assert!(tried > 500_000, "{tried} sequences");
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
