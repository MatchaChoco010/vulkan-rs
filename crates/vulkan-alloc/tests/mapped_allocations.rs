mod fake_device;

use std::collections::{HashMap, HashSet};

use fake_device::{FakeDevice, MapFault, MemoryObject};
use proptest::prelude::*;
use proptest::test_runner::Config;
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationScheme, Allocator,
    AllocatorCreateDesc, AllocatorSnapshot, MemoryLocation,
};

const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;
const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

const BLOCK_SIZE: u64 = 4096;

fn new_block_size(size: u64, largest_block: u64) -> u64 {
    if size > BLOCK_SIZE / 2 {
        return size;
    }
    let mut block_size = BLOCK_SIZE;
    for _ in 0..3 {
        if block_size / 2 > largest_block && block_size / 2 >= 2 * size {
            block_size /= 2;
        } else {
            break;
        }
    }
    block_size
}

const KINDS: [vk::MemoryPropertyFlags; 6] = [
    LOCAL,
    VISIBLE.union(COHERENT),
    VISIBLE.union(CACHED),
    VISIBLE,
    LOCAL.union(VISIBLE).union(COHERENT),
    LOCAL.union(VISIBLE).union(CACHED),
];

const LOCATIONS: [MemoryLocation; 4] = [
    MemoryLocation::GpuOnly,
    MemoryLocation::CpuToGpu,
    MemoryLocation::GpuToCpu,
    MemoryLocation::Custom {
        required: vk::MemoryPropertyFlags::empty(),
        preferred: vk::MemoryPropertyFlags::empty(),
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Shared,
    SharedWithDeviceAddress,
    Dedicated,
    ZeroInitialized,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Allocate {
        size: u64,
        memory_type: usize,
        location: usize,
        kind: Kind,
        mapped: bool,
    },
    Free(usize),
    MapPersistent(usize),
    WriteEveryMappedAllocation,
    FaultNextMap(MapFault),
}

fn ops() -> impl Strategy<Value = Op> {
    let kind = prop_oneof![
        6 => Just(Kind::Shared),
        2 => Just(Kind::SharedWithDeviceAddress),
        1 => Just(Kind::Dedicated),
        1 => Just(Kind::ZeroInitialized),
    ];
    let size = prop_oneof![
        6 => 1..=1500u64,
        1 => Just(BLOCK_SIZE),
        1 => BLOCK_SIZE + 1..=2 * BLOCK_SIZE,
    ];
    prop_oneof![
        8 => (size, any::<usize>(), 0..LOCATIONS.len(), kind, prop::bool::weighted(0.7))
            .prop_map(|(size, memory_type, location, kind, mapped)| Op::Allocate {
                size,
                memory_type,
                location,
                kind,
                mapped,
            }),
        3 => any::<usize>().prop_map(Op::Free),
        2 => any::<usize>().prop_map(Op::MapPersistent),
        2 => Just(Op::WriteEveryMappedAllocation),
        2 => prop_oneof![
            Just(MapFault::ReturnsNull),
            Just(MapFault::Fails(vk::VkResult::ERROR_MEMORY_MAP_FAILED)),
            Just(MapFault::Fails(vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY)),
        ]
        .prop_map(Op::FaultNextMap),
    ]
}

fn scenarios() -> impl Strategy<Value = (Vec<vk::MemoryPropertyFlags>, Vec<Op>)> {
    (
        prop::collection::vec(prop::sample::select(KINDS.to_vec()), 1..=5),
        prop::collection::vec(ops(), 1..=40),
    )
}

fn required_flags(location: MemoryLocation) -> vk::MemoryPropertyFlags {
    match location {
        MemoryLocation::GpuOnly => LOCAL,
        MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu => VISIBLE | COHERENT,
        MemoryLocation::Custom { required, .. } => required,
    }
}

struct Live {
    allocation: Allocation,
    persistent: Option<usize>,
    fill: Option<u8>,
}

struct Run {
    types: Vec<vk::MemoryPropertyFlags>,
    allocator: Allocator,
    fake: FakeDevice,
    live: Vec<Live>,
    fault: Option<MapFault>,
    fills: u8,
    resources: u64,
    made_for_one_request: HashSet<vk::DeviceMemory>,
}

fn mapped_memory(objects: &HashMap<vk::DeviceMemory, MemoryObject>) -> Vec<vk::DeviceMemory> {
    let mut mapped = objects
        .iter()
        .filter(|(_, object)| object.mapped)
        .map(|(&memory, _)| memory)
        .collect::<Vec<_>>();
    mapped.sort_by_key(|memory| memory.0);
    mapped
}

fn layout_without_empty_blocks(snapshot: &AllocatorSnapshot) -> Vec<(vk::DeviceMemory, String)> {
    snapshot
        .memory_types
        .iter()
        .flat_map(|memory_type| &memory_type.blocks)
        .filter(|block| block.allocated_region_count > 0)
        .map(|block| (block.memory, format!("{:?}", block.regions)))
        .collect()
}

fn error_of(fault: MapFault) -> AllocationError {
    match fault {
        MapFault::Fails(result) => AllocationError::Vulkan(result),
        MapFault::ReturnsNull | MapFault::ReturnsAddress(_) => AllocationError::MappingFailed,
    }
}

impl Run {
    fn new(types: &[vk::MemoryPropertyFlags]) -> Self {
        let fake = FakeDevice::new();
        let mut memory_properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: types.len() as u32,
            memory_heap_count: 1,
            ..Default::default()
        };
        memory_properties.memory_heaps[0].size = 1 << 20;
        for (memory_type, &flags) in memory_properties.memory_types.iter_mut().zip(types) {
            memory_type.property_flags = flags;
        }
        let allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties,
            non_coherent_atom_size: 64,
            block_size: BLOCK_SIZE,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        });
        Self {
            types: types.to_vec(),
            fake,
            allocator,
            live: Vec::new(),
            fault: None,
            fills: 0,
            resources: 0,
            made_for_one_request: HashSet::new(),
        }
    }

    fn expected_pointer(&self, allocation: &Allocation) -> Option<usize> {
        let base = self.fake.base_address(allocation.memory())?;
        Some(base + allocation.offset() as usize)
    }

    fn apply(&mut self, op: Op) -> Result<(), TestCaseError> {
        match op {
            Op::Allocate {
                size,
                memory_type,
                location,
                kind,
                mapped,
            } => self.allocate(size, memory_type % self.types.len(), location, kind, mapped),
            Op::Free(pick) => self.free(pick),
            Op::MapPersistent(pick) => self.map_persistent(pick),
            Op::WriteEveryMappedAllocation => self.write_every_mapped_allocation(),
            Op::FaultNextMap(fault) => {
                self.fake.fault_next_map(fault);
                self.fault = Some(fault);
                Ok(())
            }
        }
    }

    fn allocate(
        &mut self,
        size: u64,
        memory_type: usize,
        location: usize,
        kind: Kind,
        mapped: bool,
    ) -> Result<(), TestCaseError> {
        self.resources += 1;
        let location = LOCATIONS[location];
        let mut desc = AllocationCreateDesc {
            mapped,
            ..AllocationCreateDesc::new(
                vk::MemoryRequirements {
                    size,
                    alignment: 16,
                    memory_type_bits: 1 << memory_type,
                },
                location,
                true,
            )
        };
        match kind {
            Kind::Shared => {}
            Kind::SharedWithDeviceAddress => {
                desc.memory_allocate_flags = vk::MemoryAllocateFlags::DEVICE_ADDRESS;
            }
            Kind::Dedicated => {
                desc.allocation_scheme =
                    AllocationScheme::DedicatedBuffer(vk::Buffer(self.resources));
            }
            Kind::ZeroInitialized => {
                desc.memory_allocate_flags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
            }
        }
        let flags = self.types[memory_type];
        let before_objects = self.fake.memory_objects();
        let before_snapshot = self.allocator.snapshot();
        let largest_block = before_snapshot
            .memory_types
            .iter()
            .filter(|candidate| candidate.memory_type_index == memory_type as u32)
            .flat_map(|memory_type| &memory_type.blocks)
            .filter(|block| !block.dedicated && !self.made_for_one_request.contains(&block.memory))
            .map(|block| block.size)
            .max()
            .unwrap_or(0);
        let before_calls = self.fake.calls();
        let result = unsafe { self.allocator.allocate(desc) };
        for memory in self.fake.memory_objects().into_keys() {
            if !before_objects.contains_key(&memory) {
                if size > BLOCK_SIZE / 2 {
                    self.made_for_one_request.insert(memory);
                } else {
                    self.made_for_one_request.remove(&memory);
                }
            }
        }
        let calls = self.fake.calls();
        let maps = calls.map - before_calls.map;
        prop_assert_eq!(self.fake.misuse(), Vec::<String>::new());

        if !flags.contains(required_flags(location)) {
            prop_assert_eq!(result, Err(AllocationError::NoCompatibleMemoryType));
            return Ok(());
        }
        if !mapped || !flags.contains(VISIBLE) {
            let allocation = result.map_err(|error| {
                TestCaseError::fail(format!(
                    "an allocation that is not mapped failed: {error:?}"
                ))
            })?;
            prop_assert_eq!(
                maps,
                0,
                "vkMapMemory was called for an allocation made with mapped {} in memory {:?}",
                mapped,
                flags
            );
            prop_assert_eq!(allocation.mapped_ptr(), None);
            let slice = unsafe { allocation.mapped_slice() };
            prop_assert!(slice.is_none(), "an unmapped allocation has a mapped slice");
            self.live.push(Live {
                allocation,
                persistent: None,
                fill: None,
            });
            return Ok(());
        }

        match (maps, self.fault) {
            (1, Some(fault)) => {
                self.fault = None;
                prop_assert_eq!(
                    result,
                    Err(error_of(fault)),
                    "allocate when vkMapMemory gives {:?}",
                    fault
                );
                prop_assert_eq!(
                    calls.unmap - before_calls.unmap,
                    usize::from(fault == MapFault::ReturnsNull),
                    "vkUnmapMemory calls of an allocation whose mapping gives {:?}",
                    fault
                );
                let after = self.allocator.snapshot();
                prop_assert_eq!(
                    after.stats.allocation_count,
                    before_snapshot.stats.allocation_count
                );
                prop_assert_eq!(
                    after.stats.allocated_bytes,
                    before_snapshot.stats.allocated_bytes
                );
                prop_assert_eq!(
                    layout_without_empty_blocks(&after),
                    layout_without_empty_blocks(&before_snapshot),
                    "the blocks that hold allocations after an undone allocation"
                );
                prop_assert_eq!(
                    self.allocator.live_allocation_count(),
                    self.live.len(),
                    "live allocations after an undone allocation"
                );
                let objects = self.fake.memory_objects();
                prop_assert_eq!(
                    mapped_memory(&objects),
                    mapped_memory(&before_objects),
                    "the mapped memory after an undone allocation"
                );
                let new = objects
                    .keys()
                    .filter(|memory| !before_objects.contains_key(memory))
                    .collect::<Vec<_>>();
                prop_assert!(new.len() <= 1, "an undone allocation left {:?}", new);
                for memory in new {
                    let block = after
                        .memory_types
                        .iter()
                        .flat_map(|memory_type| &memory_type.blocks)
                        .find(|block| block.memory == *memory)
                        .ok_or_else(|| TestCaseError::fail("an unknown memory object"))?;
                    prop_assert!(
                        block.allocated_region_count == 0
                            && !block.dedicated
                            && block.size == new_block_size(size, largest_block),
                        "the block left by an undone allocation is {:?}",
                        block
                    );
                    let empty_with_these_flags = after
                        .memory_types
                        .iter()
                        .find(|candidate| candidate.memory_type_index == memory_type as u32)
                        .into_iter()
                        .flat_map(|memory_type| &memory_type.blocks)
                        .filter(|other| {
                            other.allocated_region_count == 0
                                && other.memory_allocate_flags == block.memory_allocate_flags
                        })
                        .count();
                    prop_assert_eq!(
                        empty_with_these_flags,
                        1,
                        "the empty blocks of memory type {} with the flags of the block left by an \
                         undone allocation",
                        memory_type
                    );
                }
                for (memory, object) in &objects {
                    if let Some(earlier) = before_objects.get(memory) {
                        prop_assert_eq!(object, earlier, "the memory {:?} changed", memory);
                    }
                }
                return Ok(());
            }
            (0 | 1, _) => {}
            (more, _) => {
                return Err(TestCaseError::fail(format!(
                    "{more} vkMapMemory calls for one allocation"
                )));
            }
        }

        let allocation = result.map_err(|error| {
            TestCaseError::fail(format!(
                "a mapped allocation of host-visible memory failed: {error:?}"
            ))
        })?;
        let was_mapped = before_objects
            .get(&allocation.memory())
            .is_some_and(|object| object.mapped);
        prop_assert_eq!(
            maps,
            usize::from(!was_mapped),
            "vkMapMemory calls for an allocation in memory that was {}mapped",
            if was_mapped { "" } else { "not " }
        );
        prop_assert!(self.fake.memory_objects()[&allocation.memory()].mapped);
        let pointer = allocation
            .mapped_ptr()
            .map(|pointer| pointer.as_ptr() as usize);
        prop_assert!(pointer.is_some(), "a mapped allocation has no pointer");
        prop_assert_eq!(pointer, self.expected_pointer(&allocation));
        prop_assert_eq!(
            unsafe { allocation.mapped_slice() }.map(<[u8]>::len),
            Some(allocation.size() as usize)
        );
        self.live.push(Live {
            persistent: pointer,
            allocation,
            fill: None,
        });
        Ok(())
    }

    fn free(&mut self, pick: usize) -> Result<(), TestCaseError> {
        if self.live.is_empty() {
            return Ok(());
        }
        let live = self.live.remove(pick % self.live.len());
        let memory = live.allocation.memory();
        let before = self.fake.calls();
        let was_mapped = self.fake.memory_objects()[&memory].mapped;
        let other_mappings = self
            .live
            .iter()
            .any(|other| other.allocation.memory() == memory && other.persistent.is_some());
        unsafe { self.allocator.free(live.allocation) }
            .map_err(|error| TestCaseError::fail(format!("free: {error:?}")))?;
        let unmaps = self.fake.calls().unmap - before.unmap;
        let still_mapped = self
            .fake
            .memory_objects()
            .get(&memory)
            .is_some_and(|object| object.mapped);
        prop_assert_eq!(
            (unmaps, still_mapped),
            (
                usize::from(was_mapped && !other_mappings),
                was_mapped && other_mappings
            ),
            "vkUnmapMemory calls and the mapping of the block after freeing an allocation that \
             was {}persistently mapped, with {}other persistently mapped allocations in it",
            if live.persistent.is_some() {
                ""
            } else {
                "not "
            },
            if other_mappings { "" } else { "no " }
        );
        Ok(())
    }

    fn map_persistent(&mut self, pick: usize) -> Result<(), TestCaseError> {
        if self.live.is_empty() {
            return Ok(());
        }
        let index = pick % self.live.len();
        let memory = self.live[index].allocation.memory();
        let host_visible = self.live[index]
            .allocation
            .memory_properties()
            .contains(VISIBLE);
        let was_mapped = self.fake.memory_objects()[&memory].mapped;
        let before = self.fake.calls();
        let result = unsafe {
            self.allocator
                .map_persistent(&mut self.live[index].allocation)
        };
        let maps = self.fake.calls().map - before.map;
        if !host_visible {
            prop_assert_eq!(result, Err(AllocationError::NotHostVisible));
            prop_assert_eq!(maps, 0);
            prop_assert_eq!(self.live[index].allocation.mapped_ptr(), None);
            return Ok(());
        }
        if let (1, Some(fault)) = (maps, self.fault) {
            self.fault = None;
            prop_assert_eq!(result, Err(error_of(fault)));
            prop_assert_eq!(self.live[index].allocation.mapped_ptr(), None);
            return Ok(());
        }
        let pointer = result
            .map_err(|error| TestCaseError::fail(format!("map_persistent: {error:?}")))?
            .as_ptr() as usize;
        prop_assert_eq!(maps, usize::from(!was_mapped));
        prop_assert_eq!(
            Some(pointer),
            self.expected_pointer(&self.live[index].allocation)
        );
        if let Some(earlier) = self.live[index].persistent {
            prop_assert_eq!(pointer, earlier, "map_persistent moved the mapping");
        }
        prop_assert_eq!(
            self.live[index]
                .allocation
                .mapped_ptr()
                .map(|pointer| pointer.as_ptr() as usize),
            Some(pointer),
            "mapped_ptr() after map_persistent"
        );
        prop_assert_eq!(
            unsafe { self.live[index].allocation.mapped_slice() }.map(<[u8]>::len),
            Some(self.live[index].allocation.size() as usize)
        );
        self.live[index].persistent = Some(pointer);
        Ok(())
    }

    fn write_every_mapped_allocation(&mut self) -> Result<(), TestCaseError> {
        let mut fills = self.fills;
        let mut slices = Vec::new();
        for live in &mut self.live {
            if live.persistent.is_none() {
                continue;
            }
            fills = fills.wrapping_add(1);
            live.fill = Some(fills);
            let slice = unsafe { live.allocation.mapped_slice_mut() }
                .ok_or_else(|| TestCaseError::fail("a mapped allocation has no slice"))?;
            slices.push((fills, slice));
        }
        self.fills = fills;
        let mut ranges = slices
            .iter()
            .map(|(_, slice)| {
                let range = slice.as_ptr_range();
                (range.start as usize, range.end as usize)
            })
            .collect::<Vec<_>>();
        ranges.sort_unstable();
        for pair in ranges.windows(2) {
            prop_assert!(pair[0].1 <= pair[1].0, "mapped slices overlap: {:?}", pair);
        }
        for (fill, slice) in slices {
            slice.fill(fill);
        }
        for live in &self.live {
            let Some(fill) = live.fill else { continue };
            let bytes = self
                .fake
                .bytes(
                    live.allocation.memory(),
                    live.allocation.offset(),
                    live.allocation.size(),
                )
                .ok_or_else(|| TestCaseError::fail("the device lost the bytes"))?;
            prop_assert!(
                bytes.iter().all(|&byte| byte == fill),
                "the device memory of {:?} does not hold the byte {} written through its \
                 mapped slice",
                live.allocation,
                fill
            );
        }
        Ok(())
    }

    fn finish(mut self) -> Result<(), TestCaseError> {
        while let Some(live) = self.live.pop() {
            unsafe { self.allocator.free(live.allocation) }
                .map_err(|error| TestCaseError::fail(format!("free: {error:?}")))?;
        }
        prop_assert!(
            mapped_memory(&self.fake.memory_objects()).is_empty(),
            "memory stays mapped after every allocation was freed"
        );
        drop(self.allocator);
        prop_assert_eq!(self.fake.misuse(), Vec::<String>::new());
        prop_assert!(self.fake.memory_objects().is_empty());
        let calls = self.fake.calls();
        prop_assert!(calls.map >= calls.unmap, "{:?}", calls);
        Ok(())
    }
}

fn config() -> Config {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(512);
    Config {
        cases,
        failure_persistence: None,
        ..Config::default()
    }
}

proptest! {
    #![proptest_config(config())]

    #[test]
    fn allocations_made_mapped_hold_their_mapping_until_they_are_freed(
        (types, ops) in scenarios(),
    ) {
        let mut run = Run::new(&types);
        for (step, &op) in ops.iter().enumerate() {
            run.apply(op).map_err(|error| {
                TestCaseError::fail(format!("step {step}, {op:?}: {error}"))
            })?;
            prop_assert_eq!(run.fake.misuse(), Vec::<String>::new(), "step {}", step);
            for live in &run.live {
                prop_assert_eq!(
                    live.allocation.mapped_ptr().map(|pointer| pointer.as_ptr() as usize),
                    live.persistent,
                    "step {}: mapped_ptr() of {:?}",
                    step,
                    live.allocation
                );
            }
        }
        run.finish()?;
    }
}

#[test]
fn a_mapping_whose_address_does_not_fit_in_a_pointer_undoes_the_allocation() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mapped = |size| AllocationCreateDesc {
        mapped: true,
        ..fake_device::desc(size, 1, fake_device::HOST_COHERENT)
    };
    unsafe {
        fake.fault_next_map(MapFault::ReturnsAddress(usize::MAX - 8));
        let first = allocator.allocate(mapped(16)).unwrap();
        assert_eq!(
            first.mapped_ptr().map(|pointer| pointer.as_ptr() as usize),
            Some(usize::MAX - 8)
        );
        let snapshot = allocator.snapshot();
        let calls = fake.calls();
        assert_eq!(
            allocator.allocate(mapped(16)),
            Err(AllocationError::MappingFailed),
            "an allocation at offset 16 of memory mapped at usize::MAX - 8"
        );
        assert_eq!(allocator.snapshot(), snapshot);
        assert_eq!(fake.calls(), calls);
        assert!(fake.memory_objects()[&first.memory()].mapped);
        assert_eq!(allocator.free_all(), 1);
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
}

#[test]
fn allocations_are_send_and_sync() {
    fn assert_send_and_sync<T: Send + Sync>() {}
    assert_send_and_sync::<Allocation>();
}
