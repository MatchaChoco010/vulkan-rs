mod fake_device;

use std::collections::{BTreeMap, HashMap};

use fake_device::{
    BLOCK_SIZE, DEVICE_LOCAL, FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, MemoryObject,
    NON_COHERENT_ATOM_SIZE, desc,
};
use proptest::prelude::*;
use proptest::sample::select;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationResourceClass, AllocationScheme, Allocator,
};

const MEMORY_TYPES: [u32; 3] = [DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Memory {
    Shared,
    DedicatedBuffer,
    DedicatedImage,
    Exportable,
    ZeroInitialized,
}

#[derive(Debug, Clone, Copy)]
struct Request {
    memory_type: u32,
    size: u64,
    linear: bool,
    device_address: bool,
    memory: Memory,
}

impl Request {
    fn has_its_own_memory(self) -> bool {
        self.memory != Memory::Shared
    }

    fn flags(self) -> vk::MemoryAllocateFlags {
        let mut flags = vk::MemoryAllocateFlags::empty();
        if self.device_address {
            flags |= vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
        if self.memory == Memory::ZeroInitialized {
            flags |= vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
        }
        flags
    }

    fn alignment(self) -> u64 {
        if self.memory_type == HOST_NON_COHERENT {
            NON_COHERENT_ATOM_SIZE
        } else {
            1
        }
    }

    fn description(self, resource: u64) -> AllocationCreateDesc<'static> {
        let mut description = AllocationCreateDesc {
            linear: self.linear,
            memory_allocate_flags: self.flags(),
            ..desc(self.size, 1, self.memory_type)
        };
        match self.memory {
            Memory::DedicatedBuffer => {
                description.allocation_scheme =
                    AllocationScheme::DedicatedBuffer(vk::Buffer(resource));
            }
            Memory::DedicatedImage => {
                description.allocation_scheme =
                    AllocationScheme::DedicatedImage(vk::Image(resource));
            }
            Memory::Exportable => {
                description.export_handle_types = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
            }
            Memory::Shared | Memory::ZeroInitialized => {}
        }
        description
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Allocate(Request),
    Free(usize),
    MapPersistent(usize),
    FreeEmptyBlocks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    Drop,
    FreeAllThenDrop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    memory_type: u32,
    size: u64,
    linear: bool,
    flags: vk::MemoryAllocateFlags,
    own: bool,
    made_for_one_request: bool,
    block_index: usize,
}

struct Live {
    allocation: Allocation,
    persistent: bool,
}

type Events = BTreeMap<&'static str, usize>;

#[derive(Default)]
struct Model {
    blocks: BTreeMap<u64, Block>,
    live: Vec<Live>,
    made: u64,
    events: Events,
}

fn gaps(size: u64, taken: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut taken = taken.to_vec();
    taken.sort_unstable();
    let mut gaps = Vec::new();
    let mut end = 0;
    for (offset, length) in taken {
        if offset > end {
            gaps.push((end, offset - end));
        }
        end = offset + length;
    }
    if size > end {
        gaps.push((end, size - end));
    }
    gaps
}

fn fits((start, length): (u64, u64), size: u64, alignment: u64) -> bool {
    let padding = (alignment - start % alignment) % alignment;
    padding + size <= length
}

fn fail(message: String) -> TestCaseError {
    TestCaseError::fail(message)
}

impl Model {
    fn note(&mut self, event: &'static str) {
        *self.events.entry(event).or_default() += 1;
    }

    fn is_empty_block(&self, memory: u64) -> bool {
        !self
            .live
            .iter()
            .any(|live| live.allocation.memory().0 == memory)
    }

    fn empty_blocks(&self) -> Vec<u64> {
        self.blocks
            .keys()
            .copied()
            .filter(|&memory| self.is_empty_block(memory))
            .collect()
    }

    fn taken_in(&self, memory: u64) -> Vec<(u64, u64)> {
        self.live
            .iter()
            .filter(|live| live.allocation.memory().0 == memory)
            .map(|live| (live.allocation.offset(), live.allocation.size()))
            .collect()
    }

    fn new_block_size(&self, request: Request) -> u64 {
        if request.has_its_own_memory() || request.size > BLOCK_SIZE / 2 {
            return request.size;
        }
        let largest = self
            .blocks
            .values()
            .filter(|block| {
                block.memory_type == request.memory_type
                    && !block.own
                    && !block.made_for_one_request
            })
            .map(|block| block.size)
            .max()
            .unwrap_or(0);
        let mut size = BLOCK_SIZE;
        for _ in 0..3 {
            if size / 2 > largest && size / 2 >= 2 * request.size {
                size /= 2;
            } else {
                break;
            }
        }
        size
    }

    fn first_with_room(&self, request: Request, empty: bool) -> Option<u64> {
        self.blocks
            .iter()
            .filter(|&(&memory, block)| {
                !request.has_its_own_memory()
                    && !block.own
                    && block.memory_type == request.memory_type
                    && block.flags == request.flags()
                    && self.is_empty_block(memory) == empty
                    && (empty || block.linear == request.linear)
                    && gaps(block.size, &self.taken_in(memory))
                        .into_iter()
                        .any(|gap| fits(gap, request.size, request.alignment()))
            })
            .min_by_key(|(_, block)| block.block_index)
            .map(|(&memory, _)| memory)
    }

    fn apply(
        &mut self,
        fake: &FakeDevice,
        allocator: &mut Allocator,
        op: Op,
    ) -> Result<(), TestCaseError> {
        match op {
            Op::Allocate(request) => self.allocate(fake, allocator, request),
            Op::Free(pick) => self.free(fake, allocator, pick),
            Op::MapPersistent(pick) => self.map_persistent(allocator, pick),
            Op::FreeEmptyBlocks => self.free_empty_blocks(fake, allocator),
        }
    }

    fn allocate(
        &mut self,
        fake: &FakeDevice,
        allocator: &mut Allocator,
        request: Request,
    ) -> Result<(), TestCaseError> {
        self.made += 1;
        let before = fake.calls();
        let new_block_size = self.new_block_size(request);
        let holding = self.first_with_room(request, false);
        let empty = self.first_with_room(request, true);
        let empty_with_other_flags = self.blocks.iter().any(|(&memory, block)| {
            !request.has_its_own_memory()
                && block.memory_type == request.memory_type
                && block.flags != request.flags()
                && self.is_empty_block(memory)
        });
        let allocation = unsafe { allocator.allocate(request.description(1000 + self.made)) }
            .map_err(|error| fail(format!("the allocation was refused: {error:?}")))?;
        let memory = allocation.memory().0;
        let calls = fake.calls();
        if let Some(chosen) = holding.or(empty) {
            prop_assert_eq!(
                calls.allocate,
                before.allocate,
                "a request that fits in a block it may share must not allocate memory"
            );
            prop_assert_eq!(
                memory,
                chosen,
                "the allocation must go to the first block with room that holds allocations \
                 of its kind, or else to the first empty block with room"
            );
            if holding.is_some() && empty.is_some() {
                self.note("allocate: tries the empty block after the blocks that hold allocations");
            }
            if holding.is_none() {
                let block = self.blocks.get_mut(&memory).unwrap();
                let other_kind = block.linear != request.linear;
                block.linear = request.linear;
                self.note(if other_kind {
                    "allocate: reuses the empty block for the other kind of resource"
                } else {
                    "allocate: reuses the empty block"
                });
            } else {
                self.note("allocate: shares a block that holds allocations");
            }
        } else {
            prop_assert_eq!(
                calls.allocate,
                before.allocate + 1,
                "a request that fits in no block it may share must allocate memory"
            );
            prop_assert!(
                !self.blocks.contains_key(&memory),
                "the new memory has the handle of a block that exists"
            );
            let own = request.has_its_own_memory();
            let block_index = allocator
                .snapshot()
                .memory_types
                .iter()
                .flat_map(|memory_type| &memory_type.blocks)
                .find(|block| block.memory == allocation.memory())
                .map(|block| block.block_index);
            prop_assert!(block_index.is_some(), "the snapshot lacks the new block");
            self.blocks.insert(
                memory,
                Block {
                    memory_type: request.memory_type,
                    size: new_block_size,
                    linear: request.linear,
                    flags: request.flags(),
                    own,
                    made_for_one_request: request.size > BLOCK_SIZE / 2,
                    block_index: block_index.unwrap(),
                },
            );
            if empty_with_other_flags {
                self.note("allocate: does not reuse the empty block with other flags");
            }
            if !own && new_block_size < BLOCK_SIZE {
                self.note("allocate: a new block smaller than the block size");
            }
            self.note("allocate: new memory");
        }
        self.live.push(Live {
            allocation,
            persistent: false,
        });
        Ok(())
    }

    fn free(
        &mut self,
        fake: &FakeDevice,
        allocator: &mut Allocator,
        pick: usize,
    ) -> Result<(), TestCaseError> {
        if self.live.is_empty() {
            return Ok(());
        }
        let live = self.live.remove(pick % self.live.len());
        let memory = live.allocation.memory().0;
        let before = fake.calls();
        unsafe { allocator.free(live.allocation) }
            .map_err(|error| fail(format!("the free was refused: {error:?}")))?;
        let calls = fake.calls();
        if !self.is_empty_block(memory) {
            prop_assert_eq!(
                calls.free,
                before.free,
                "a block that still holds allocations must not be freed"
            );
            self.note("free: one of several");
            return Ok(());
        }
        let block = self.blocks[&memory];
        let another_empty_block = |same_flags: bool| {
            self.blocks.iter().find(|&(&other, other_block)| {
                other != memory
                    && other_block.memory_type == block.memory_type
                    && (other_block.flags == block.flags) == same_flags
                    && self.is_empty_block(other)
            })
        };
        let candidate = !block.own && block.size <= BLOCK_SIZE;
        if let Some((&other, other_block)) = another_empty_block(true)
            && candidate
            && block.size > other_block.size
        {
            prop_assert_eq!(
                calls.free,
                before.free + 1,
                "the smaller of two empty blocks must be freed: {:?} and {:?}",
                block,
                other_block
            );
            self.blocks.remove(&other);
            self.note("free: the larger of two empty blocks is kept");
        } else if candidate && another_empty_block(true).is_none() {
            prop_assert_eq!(
                calls.free,
                before.free,
                "the only empty shared block of its memory type and flags must be kept"
            );
            if another_empty_block(false).is_some() {
                self.note("free: the empty block is kept beside one with other flags");
            }
            self.note("free: the empty block is kept");
        } else {
            prop_assert_eq!(
                calls.free,
                before.free + 1,
                "an empty block that is not kept must be freed at once: {:?}",
                block
            );
            self.blocks.remove(&memory);
            self.note(if block.own {
                "free: memory of its own is freed"
            } else if block.size > BLOCK_SIZE {
                "free: a block larger than the block size is freed"
            } else {
                "free: a second empty block is freed"
            });
        }
        Ok(())
    }

    fn map_persistent(
        &mut self,
        allocator: &mut Allocator,
        pick: usize,
    ) -> Result<(), TestCaseError> {
        let host_visible = (0..self.live.len())
            .filter(|&index| self.live[index].allocation.memory_type_index() != DEVICE_LOCAL)
            .collect::<Vec<_>>();
        if host_visible.is_empty() {
            return Ok(());
        }
        let index = host_visible[pick % host_visible.len()];
        unsafe { allocator.map_persistent(&mut self.live[index].allocation) }
            .map_err(|error| fail(format!("map_persistent: {error:?}")))?;
        self.live[index].persistent = true;
        self.note("map_persistent");
        Ok(())
    }

    fn free_empty_blocks(
        &mut self,
        fake: &FakeDevice,
        allocator: &mut Allocator,
    ) -> Result<(), TestCaseError> {
        let empty = self.empty_blocks();
        let before = fake.calls();
        let freed = unsafe { allocator.free_empty_blocks() };
        prop_assert_eq!(
            freed,
            empty.len(),
            "free_empty_blocks must return the number of empty blocks"
        );
        prop_assert_eq!(
            fake.calls().free - before.free,
            empty.len(),
            "free_empty_blocks must free each empty block"
        );
        for memory in &empty {
            self.blocks.remove(memory);
        }
        if !empty.is_empty() {
            self.note("free_empty_blocks: frees a block");
        }
        Ok(())
    }

    fn check(&self, fake: &FakeDevice, allocator: &Allocator) -> Result<(), TestCaseError> {
        prop_assert_eq!(
            fake.misuse(),
            Vec::<String>::new(),
            "invalid calls to the device"
        );
        let expected =
            self.blocks
                .iter()
                .map(|(&memory, block)| {
                    (
                        vk::DeviceMemory(memory),
                        MemoryObject {
                            size: block.size,
                            memory_type_index: block.memory_type,
                            mapped: self.live.iter().any(|live| {
                                live.persistent && live.allocation.memory().0 == memory
                            }),
                        },
                    )
                })
                .collect::<HashMap<_, _>>();
        prop_assert_eq!(
            fake.memory_objects(),
            expected,
            "the memory of the device, where empty blocks are not mapped"
        );

        let snapshot = allocator.snapshot();
        for memory_type in &snapshot.memory_types {
            let mut empty = BTreeMap::<u32, usize>::new();
            for block in &memory_type.blocks {
                if block.allocated_region_count == 0 {
                    *empty
                        .entry(block.memory_allocate_flags.as_raw())
                        .or_default() += 1;
                }
            }
            prop_assert!(
                empty.values().all(|&count| count <= 1),
                "memory type {} has more than one empty block with the same flags: {:?}",
                memory_type.memory_type_index,
                empty
            );
            for block in &memory_type.blocks {
                let model = self.blocks.get(&block.memory.0);
                prop_assert!(model.is_some(), "the snapshot has an unknown block");
                let model = model.unwrap();
                prop_assert_eq!(
                    (
                        block.size,
                        block.dedicated,
                        block.memory_allocate_flags,
                        block.block_index,
                        block.resource_class
                    ),
                    (
                        model.size,
                        model.own,
                        model.flags,
                        model.block_index,
                        if model.linear {
                            AllocationResourceClass::Linear
                        } else {
                            AllocationResourceClass::Optimal
                        }
                    ),
                    "the block {:?} in the snapshot",
                    block.memory
                );
            }
        }
        let stats = allocator.stats();
        prop_assert_eq!(
            (stats.block_count, stats.total_block_bytes),
            (
                self.blocks.len(),
                self.blocks.values().map(|block| block.size).sum::<u64>()
            ),
            "the stats count every block, the empty ones too"
        );
        prop_assert_eq!(
            stats.allocated_bytes + stats.free_bytes,
            stats.total_block_bytes,
            "the bytes of the stats"
        );
        prop_assert_eq!(
            allocator.is_empty(),
            self.live.is_empty(),
            "is_empty tells whether every allocation was freed"
        );
        Ok(())
    }

    fn end(
        &mut self,
        fake: &FakeDevice,
        mut allocator: Allocator,
        ending: Ending,
    ) -> Result<(), TestCaseError> {
        let before = fake.calls();
        let blocks = self.blocks.len();
        match ending {
            Ending::Drop => {
                if !self.live.is_empty() {
                    self.note("drop: with live allocations");
                }
                drop(allocator);
                let calls = fake.calls();
                if blocks == 0 {
                    prop_assert_eq!(
                        calls,
                        before,
                        "dropping an allocator without blocks must not call the device"
                    );
                } else {
                    self.note("drop: frees the blocks");
                }
                prop_assert_eq!(
                    calls.free - before.free,
                    blocks,
                    "dropping the allocator must free each of its blocks"
                );
            }
            Ending::FreeAllThenDrop => {
                let freed = unsafe { allocator.free_all() };
                prop_assert_eq!(freed, self.live.len(), "the count free_all returns");
                let calls = fake.calls();
                prop_assert_eq!(
                    calls.free - before.free,
                    blocks,
                    "free_all must free each block"
                );
                drop(allocator);
                prop_assert_eq!(
                    fake.calls(),
                    calls,
                    "dropping the allocator after free_all must not call the device"
                );
                self.note("free_all, then drop");
            }
        }
        self.live.clear();
        self.blocks.clear();
        prop_assert_eq!(
            fake.memory_objects(),
            HashMap::new(),
            "memory left on the device after the allocator is gone"
        );
        prop_assert_eq!(
            fake.misuse(),
            Vec::<String>::new(),
            "invalid calls to the device"
        );
        Ok(())
    }
}

fn run(ops: &[Op], ending: Ending) -> Result<Events, TestCaseError> {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mut model = Model::default();
    for (step, &op) in ops.iter().enumerate() {
        model
            .apply(&fake, &mut allocator, op)
            .and_then(|()| model.check(&fake, &allocator))
            .map_err(|error| fail(format!("step {step}, {op:?}: {error}")))?;
    }
    model
        .end(&fake, allocator, ending)
        .map_err(|error| fail(format!("at the end, {ending:?}: {error}")))?;
    Ok(model.events)
}

fn requests() -> impl Strategy<Value = Request> {
    (
        select(MEMORY_TYPES.to_vec()),
        prop_oneof![
            6 => 1..=1024u64,
            2 => 1..=BLOCK_SIZE,
            1 => Just(BLOCK_SIZE),
            1 => BLOCK_SIZE + 1..=2 * BLOCK_SIZE,
        ],
        any::<bool>(),
        prop::bool::weighted(0.3),
        prop_oneof![
            12 => Just(Memory::Shared),
            1 => Just(Memory::DedicatedBuffer),
            1 => Just(Memory::DedicatedImage),
            1 => Just(Memory::Exportable),
            1 => Just(Memory::ZeroInitialized),
        ],
    )
        .prop_map(
            |(memory_type, size, linear, device_address, memory)| Request {
                memory_type,
                size,
                linear,
                device_address,
                memory,
            },
        )
}

fn ops() -> impl Strategy<Value = Op> {
    prop_oneof![
        8 => requests().prop_map(Op::Allocate),
        7 => any::<usize>().prop_map(Op::Free),
        2 => any::<usize>().prop_map(Op::MapPersistent),
        1 => Just(Op::FreeEmptyBlocks),
    ]
}

fn scenarios() -> impl Strategy<Value = (Vec<Op>, Ending)> {
    (
        prop::collection::vec(ops(), 0..=60),
        select(vec![Ending::Drop, Ending::FreeAllThenDrop]),
    )
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn each_memory_type_and_set_of_flags_keeps_at_most_one_empty_block_and_drop_frees_every_block(
        (ops, ending) in scenarios()
    ) {
        run(&ops, ending)?;
    }

    #[test]
    fn allocating_and_freeing_one_allocation_again_and_again_allocates_memory_once(
        memory_type in select(MEMORY_TYPES.to_vec()),
        device_address in any::<bool>(),
        size in 1..=BLOCK_SIZE,
        rounds in prop::collection::vec((any::<bool>(), any::<bool>()), 1..=16),
    ) {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        for (round, &(linear, mapped)) in rounds.iter().enumerate() {
            let request = Request {
                memory_type,
                size,
                linear,
                device_address,
                memory: Memory::Shared,
            };
            let mut allocation = unsafe { allocator.allocate(request.description(0)) }.unwrap();
            if mapped && memory_type != DEVICE_LOCAL {
                unsafe { allocator.map_persistent(&mut allocation) }.unwrap();
            }
            unsafe { allocator.free(allocation) }.unwrap();
            prop_assert_eq!(
                (fake.calls().allocate, fake.calls().free),
                (1, 0),
                "round {}: {:?} must reuse the block of the first round", round, request
            );
            prop_assert!(
                fake.memory_objects().values().all(|object| !object.mapped),
                "round {}: the empty block must not stay mapped", round
            );
        }
        drop(allocator);
        prop_assert_eq!(fake.calls().free, 1, "dropping the allocator must free the empty block");
        prop_assert!(fake.memory_objects().is_empty());
        prop_assert_eq!(fake.misuse(), Vec::<String>::new());
    }
}

#[test]
fn allocating_and_freeing_a_buffer_three_times_allocates_memory_once() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    for _ in 0..3 {
        let allocation = unsafe { allocator.allocate(desc(256, 1, HOST_COHERENT)) }.unwrap();
        unsafe { allocator.free(allocation) }.unwrap();
    }
    assert_eq!(
        fake.calls().allocate,
        1,
        "the block of the first allocation must be kept and reused"
    );
    assert_eq!(fake.calls().free, 0, "the empty block must not be freed");
    assert_eq!(allocator.stats().block_count, 1);
    assert!(allocator.is_empty());
    drop(allocator);
    assert_eq!(
        fake.calls().free,
        1,
        "dropping the allocator must free the empty block"
    );
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn an_empty_block_is_tried_after_the_blocks_that_hold_allocations() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let non_linear = AllocationCreateDesc {
        linear: false,
        ..desc(256, 1, DEVICE_LOCAL)
    };
    let linear = desc(256, 1, DEVICE_LOCAL);
    unsafe {
        let first = allocator.allocate(non_linear).unwrap();
        let second = allocator.allocate(linear).unwrap();
        assert_ne!(first.memory(), second.memory());
        allocator.free(first).unwrap();
        assert_eq!(
            allocator.stats().block_count,
            2,
            "the block of the first allocation is kept empty, before the block of the second"
        );

        let third = allocator.allocate(linear).unwrap();
        assert_eq!(
            third.memory(),
            second.memory(),
            "a linear request must go to the block that holds linear allocations, not to the \
             empty block before it"
        );
        let fourth = allocator.allocate(non_linear).unwrap();
        assert_ne!(fourth.memory(), second.memory());
        assert_eq!(
            fake.calls().allocate,
            2,
            "the non-linear request must find the empty block still empty"
        );
        for allocation in [second, third, fourth] {
            allocator.free(allocation).unwrap();
        }
    }
    drop(allocator);
    assert!(fake.memory_objects().is_empty());
    assert_eq!(fake.misuse(), Vec::<String>::new());
}

#[test]
fn allocating_and_freeing_with_and_without_device_address_in_turn_allocates_memory_once_for_each() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    for round in 0..3 {
        for flags in [
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::DEVICE_ADDRESS,
        ] {
            let description = AllocationCreateDesc {
                memory_allocate_flags: flags,
                ..desc(256, 1, HOST_COHERENT)
            };
            let allocation = unsafe { allocator.allocate(description) }.unwrap();
            unsafe { allocator.free(allocation) }.unwrap();
            assert_eq!(
                (fake.calls().allocate, fake.calls().free),
                (if round == 0 && flags.is_empty() { 1 } else { 2 }, 0),
                "round {round}, flags {flags:?}: each set of flags must keep its own empty block"
            );
        }
    }
    assert_eq!(allocator.stats().block_count, 2);
    drop(allocator);
    assert_eq!(fake.calls().free, 2);
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn the_generated_sequences_reach_every_rule_of_the_empty_blocks() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = scenarios();
    let mut events = Events::new();
    for _ in 0..300 {
        let (ops, ending) = strategy.new_tree(&mut runner).unwrap().current();
        for (event, count) in run(&ops, ending).unwrap() {
            *events.entry(event).or_default() += count;
        }
    }
    for (event, count) in &events {
        println!("{count:>8}  {event}");
    }
    for event in [
        "allocate: new memory",
        "allocate: shares a block that holds allocations",
        "allocate: reuses the empty block",
        "allocate: reuses the empty block for the other kind of resource",
        "allocate: does not reuse the empty block with other flags",
        "free: one of several",
        "free: the empty block is kept",
        "free: the empty block is kept beside one with other flags",
        "allocate: tries the empty block after the blocks that hold allocations",
        "free: a second empty block is freed",
        "free: the larger of two empty blocks is kept",
        "allocate: a new block smaller than the block size",
        "free: a block larger than the block size is freed",
        "free: memory of its own is freed",
        "map_persistent",
        "free_empty_blocks: frees a block",
        "drop: with live allocations",
        "drop: frees the blocks",
        "free_all, then drop",
    ] {
        assert!(
            events.get(event).is_some_and(|&count| count > 0),
            "never reached: {event}"
        );
    }
}
