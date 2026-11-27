mod fake_device;

use std::collections::{HashMap, HashSet};

use fake_device::{Chain, FakeDevice, MemoryObject};
use proptest::prelude::*;
use proptest::sample::select;
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationScheme, Allocator,
    AllocatorCreateDesc, MemoryLocation,
};

const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;

const MEMORY_TYPES: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Linear,
    Optimal,
    DeviceAddress,
    Dedicated,
    ZeroInitialized,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Allocate {
        memory_type: u32,
        size: u64,
        kind: Kind,
    },
    Free(usize),
    FreeEmptyBlocks,
}

#[derive(Debug, Clone)]
struct World {
    block_size: u64,
    heap_sizes: [u64; MEMORY_TYPES as usize],
    allocator_flags: vk::MemoryAllocateFlags,
}

impl World {
    fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: MEMORY_TYPES,
            memory_heap_count: MEMORY_TYPES,
            ..Default::default()
        };
        for (index, flags) in [LOCAL, VISIBLE | COHERENT].into_iter().enumerate() {
            properties.memory_types[index].property_flags = flags;
            properties.memory_types[index].heap_index = index as u32;
            properties.memory_heaps[index].size = self.heap_sizes[index];
        }
        properties
    }

    fn has_memory_of_its_own(&self, kind: Kind) -> bool {
        matches!(kind, Kind::Dedicated | Kind::ZeroInitialized)
            || self
                .allocator_flags
                .contains(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT)
    }

    fn known_heap_size(&self, memory_type: u32) -> Option<u64> {
        Some(self.heap_sizes[memory_type as usize]).filter(|&size| size != 0)
    }

    fn shared_block_size(&self, memory_type: u32) -> u64 {
        let block_size = self.block_size.max(1);
        match self.known_heap_size(memory_type) {
            Some(heap_size) => block_size.min((heap_size / 8).max(1)),
            None => block_size,
        }
    }

    fn allocator(&self, fake: &FakeDevice) -> Allocator {
        Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: self.memory_properties(),
            non_coherent_atom_size: 1,
            block_size: self.block_size,
            memory_allocate_flags: self.allocator_flags,
        })
    }
}

fn description(memory_type: u32, size: u64, kind: Kind) -> AllocationCreateDesc<'static> {
    let mut description = AllocationCreateDesc::new(
        vk::MemoryRequirements {
            size,
            alignment: 1,
            memory_type_bits: 1 << memory_type,
        },
        MemoryLocation::custom(
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::empty(),
        ),
        kind != Kind::Optimal,
    );
    match kind {
        Kind::Linear | Kind::Optimal => {}
        Kind::DeviceAddress => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
        Kind::Dedicated => {
            description.allocation_scheme = AllocationScheme::DedicatedBuffer(vk::Buffer(1));
        }
        Kind::ZeroInitialized => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
        }
    }
    description
}

fn is_shared(chain: &Chain) -> bool {
    chain.dedicated.is_none()
        && chain.export.is_none()
        && !chain
            .flags
            .is_some_and(|(flags, _)| flags.contains(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT))
}

fn documented_block_size(
    world: &World,
    memory_type: u32,
    size: u64,
    kind: Kind,
    largest_shared_block: u64,
) -> u64 {
    if world.has_memory_of_its_own(kind) {
        return size;
    }
    let shared = world.shared_block_size(memory_type);
    if size > shared / 2 {
        return size;
    }
    let mut block_size = shared;
    for _ in 0..3 {
        let half = block_size / 2;
        if half > largest_shared_block && half >= 2 * size {
            block_size = half;
        } else {
            break;
        }
    }
    block_size
}

fn largest_shared_block(
    objects: &HashMap<vk::DeviceMemory, MemoryObject>,
    chains: &HashMap<vk::DeviceMemory, Chain>,
    made_for_one_request: &HashSet<vk::DeviceMemory>,
    memory_type: u32,
) -> u64 {
    objects
        .iter()
        .filter(|(memory, object)| {
            object.memory_type_index == memory_type
                && is_shared(&chains[memory])
                && !made_for_one_request.contains(memory)
        })
        .map(|(_, object)| object.size)
        .max()
        .unwrap_or(0)
}

fn run(world: &World, ops: &[Op]) -> Result<Vec<u64>, TestCaseError> {
    let fake = FakeDevice::new();
    let mut allocator = world.allocator(&fake);
    let mut live = Vec::<Allocation>::new();
    let mut new_blocks = Vec::new();
    let mut made_for_one_request = HashSet::new();
    for (step, &op) in ops.iter().enumerate() {
        match op {
            Op::Allocate {
                memory_type,
                size,
                kind,
            } => {
                let objects = fake.memory_objects();
                let largest = largest_shared_block(
                    &objects,
                    &fake.chains(),
                    &made_for_one_request,
                    memory_type,
                );
                let calls = fake.calls().allocate;
                let result = unsafe { allocator.allocate(description(memory_type, size, kind)) };
                let context = format!(
                    "{world:?}, step {step}, {op:?}, largest shared block of the type {largest}"
                );
                if world
                    .known_heap_size(memory_type)
                    .is_some_and(|heap_size| size > heap_size)
                {
                    prop_assert_eq!(result, Err(AllocationError::OutOfMemory), "{}", context);
                    continue;
                }
                prop_assert!(result.is_ok(), "{}: {:?}", context, result);
                let allocation = result.unwrap();
                if fake.calls().allocate > calls {
                    let expected = documented_block_size(world, memory_type, size, kind, largest);
                    prop_assert_eq!(
                        fake.memory_objects()
                            .get(&allocation.memory())
                            .map(|object| object.size),
                        Some(expected),
                        "{}: the size of the new VkDeviceMemory",
                        context
                    );
                    prop_assert_eq!(fake.calls().allocate, calls + 1, "{}", context);
                    if size > world.shared_block_size(memory_type) / 2 {
                        made_for_one_request.insert(allocation.memory());
                    } else {
                        made_for_one_request.remove(&allocation.memory());
                    }
                    new_blocks.push(expected);
                } else {
                    prop_assert!(
                        objects.contains_key(&allocation.memory()),
                        "{}: the allocation is in memory that did not exist",
                        context
                    );
                }
                prop_assert_eq!(
                    allocation.is_dedicated(),
                    world.has_memory_of_its_own(kind),
                    "{}: is_dedicated() is decided by the flags of the allocation and the request, not \
                     by the size",
                    context
                );
                live.push(allocation);
            }
            Op::Free(pick) => {
                if !live.is_empty() {
                    let allocation = live.swap_remove(pick % live.len());
                    unsafe { allocator.free(allocation) }.unwrap();
                }
            }
            Op::FreeEmptyBlocks => {
                unsafe { allocator.free_empty_blocks() };
            }
        }
        for object in fake.memory_objects().values() {
            prop_assert!(
                world
                    .known_heap_size(object.memory_type_index)
                    .is_none_or(|heap_size| object.size <= heap_size),
                "{:?}, step {}: memory of {} bytes is larger than its heap",
                world,
                step,
                object.size
            );
        }
    }
    for allocation in live {
        unsafe { allocator.free(allocation) }.unwrap();
    }
    for memory_type in allocator.snapshot().memory_types {
        for block in memory_type.blocks {
            prop_assert!(
                block.size <= world.shared_block_size(memory_type.memory_type_index),
                "{:?}: an empty block of {} bytes is kept, larger than a shared block",
                world,
                block.size
            );
        }
    }
    drop(allocator);
    prop_assert_eq!(fake.misuse(), Vec::<String>::new());
    prop_assert!(fake.memory_objects().is_empty());
    Ok(new_blocks)
}

fn worlds() -> impl Strategy<Value = World> {
    let heap_size = prop_oneof![
        2 => Just(0u64),
        1 => 1..=64u64,
        2 => 64..=65536u64,
        1 => select(vec![8u64, 4096, 8192, 1 << 16, 1 << 20]),
    ];
    (
        prop_oneof![
            1 => Just(0u64),
            1 => 1..=64u64,
            3 => 64..=8192u64,
            2 => select(vec![256u64, 4096, 1 << 16]),
        ],
        [heap_size.clone(), heap_size],
        prop_oneof![
            4 => Just(vk::MemoryAllocateFlags::empty()),
            1 => Just(vk::MemoryAllocateFlags::DEVICE_ADDRESS),
            1 => Just(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT),
        ],
    )
        .prop_map(|(block_size, heap_sizes, allocator_flags)| World {
            block_size,
            heap_sizes,
            allocator_flags,
        })
}

fn ops(world: &World) -> impl Strategy<Value = Op> + use<> {
    let shared = [world.shared_block_size(0), world.shared_block_size(1)];
    let allocate = (0..MEMORY_TYPES).prop_flat_map(move |memory_type| {
        let shared = shared[memory_type as usize];
        (
            Just(memory_type),
            prop_oneof![
                4 => 1..=(shared / 16).max(1),
                3 => 1..=(shared / 2).max(1),
                2 => select(vec![shared / 8, shared / 4, shared / 2, shared / 2 + 1, shared])
                    .prop_map(|size| size.max(1)),
                1 => 1..=2 * shared + 1,
            ],
            prop_oneof![
                6 => Just(Kind::Linear),
                2 => Just(Kind::Optimal),
                2 => Just(Kind::DeviceAddress),
                1 => Just(Kind::Dedicated),
                1 => Just(Kind::ZeroInitialized),
            ],
        )
            .prop_map(|(memory_type, size, kind)| Op::Allocate {
                memory_type,
                size,
                kind,
            })
    });
    prop_oneof![
        8 => allocate,
        4 => any::<usize>().prop_map(Op::Free),
        1 => Just(Op::FreeEmptyBlocks),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1024,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn a_new_block_has_the_size_the_heap_the_request_and_the_blocks_of_its_type_call_for(
        (world, ops) in worlds().prop_flat_map(|world| {
            let ops = prop::collection::vec(ops(&world), 0..=40);
            (Just(world), ops)
        })
    ) {
        run(&world, &ops)?;
    }
}

fn small_requests(count: usize, size: u64) -> Vec<Op> {
    vec![
        Op::Allocate {
            memory_type: 0,
            size,
            kind: Kind::Linear,
        };
        count
    ]
}

#[test]
fn small_requests_one_after_another_get_an_eighth_then_a_quarter_then_half_then_whole_blocks() {
    let world = World {
        block_size: 4096,
        heap_sizes: [0, 0],
        allocator_flags: vk::MemoryAllocateFlags::empty(),
    };
    let new_blocks = run(&world, &small_requests(2 + 4 + 8 + 16 + 16 + 1, 256)).unwrap();
    assert_eq!(
        new_blocks,
        [512, 1024, 2048, 4096, 4096, 4096],
        "each block is filled by requests of 256 bytes before the next is made"
    );
}

#[test]
fn a_request_larger_than_half_the_block_size_gets_a_block_of_its_own_size() {
    let world = World {
        block_size: 4096,
        heap_sizes: [0, 0],
        allocator_flags: vk::MemoryAllocateFlags::empty(),
    };
    let new_blocks = run(&world, &small_requests(2, 2049)).unwrap();
    assert_eq!(
        new_blocks,
        [2049, 2049],
        "two requests of a little more than half the block size waste no block"
    );
    let fake = FakeDevice::new();
    let mut allocator = world.allocator(&fake);
    let allocation = unsafe { allocator.allocate(description(0, 2049, Kind::Linear)) }.unwrap();
    assert!(
        !allocation.is_dedicated(),
        "a block of the size of the request is a shared block"
    );
    unsafe { allocator.free(allocation) }.unwrap();
    assert_eq!(
        allocator.stats().block_count,
        1,
        "the empty block is kept, as it is not larger than a shared block"
    );
    let small = unsafe { allocator.allocate(description(0, 1000, Kind::Linear)) }.unwrap();
    assert_eq!(
        fake.calls().allocate,
        1,
        "a later request is placed in the kept block"
    );
    unsafe { allocator.free(small) }.unwrap();
}

#[test]
fn a_block_made_for_one_large_request_does_not_make_the_first_small_block_larger() {
    let world = World {
        block_size: 64,
        heap_sizes: [0, 0],
        allocator_flags: vk::MemoryAllocateFlags::empty(),
    };
    let new_blocks = run(
        &world,
        &[
            Op::Allocate {
                memory_type: 0,
                size: 40,
                kind: Kind::Optimal,
            },
            Op::Allocate {
                memory_type: 0,
                size: 1,
                kind: Kind::Linear,
            },
            Op::Allocate {
                memory_type: 0,
                size: 33,
                kind: Kind::Optimal,
            },
            Op::Allocate {
                memory_type: 0,
                size: 2,
                kind: Kind::Optimal,
            },
        ],
    )
    .unwrap();
    assert_eq!(
        new_blocks,
        [40, 8, 33, 16],
        "blocks of 40 and 33 units, made for one request each, leave the first small blocks \
         at an eighth and a quarter of the block size of 64"
    );
}

#[test]
fn a_small_heap_has_shared_blocks_of_an_eighth_of_its_size() {
    let world = World {
        block_size: 64 << 20,
        heap_sizes: [8192, 0],
        allocator_flags: vk::MemoryAllocateFlags::empty(),
    };
    let new_blocks = run(
        &world,
        &[
            Op::Allocate {
                memory_type: 0,
                size: 1,
                kind: Kind::Linear,
            },
            Op::Allocate {
                memory_type: 0,
                size: 1024,
                kind: Kind::Optimal,
            },
            Op::Allocate {
                memory_type: 0,
                size: 1025,
                kind: Kind::Optimal,
            },
        ],
    )
    .unwrap();
    assert_eq!(
        new_blocks,
        [128, 1024, 1025],
        "a heap of 8 KiB has shared blocks of 1 KiB, and the first is an eighth of that"
    );
}
