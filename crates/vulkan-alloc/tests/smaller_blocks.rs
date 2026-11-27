mod fake_device;

use fake_device::FakeDevice;
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

const OOM: vk::VkResult = vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY;
const SUCCESS: vk::VkResult = vk::VkResult::SUCCESS;

const PREFERRED: u32 = 0;
const FALLBACK: u32 = 1;
const NEIGHBOUR: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Shared,
    DeviceAddress,
    DedicatedBuffer,
    Exportable,
    ZeroInitialized,
}

impl Kind {
    fn has_memory_of_its_own(self) -> bool {
        matches!(
            self,
            Kind::DedicatedBuffer | Kind::Exportable | Kind::ZeroInitialized
        )
    }
}

fn memory_properties(heap_size: u64) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 3,
        memory_heap_count: 2,
        ..Default::default()
    };
    for (index, (flags, heap)) in [
        (LOCAL | VISIBLE | COHERENT, 0),
        (VISIBLE | COHERENT, 1),
        (LOCAL | VISIBLE | COHERENT, 0),
    ]
    .into_iter()
    .enumerate()
    {
        properties.memory_types[index].property_flags = flags;
        properties.memory_types[index].heap_index = heap;
    }
    properties.memory_heaps[0].size = heap_size;
    properties
}

fn allocator(fake: &FakeDevice, block_size: u64, heap_size: u64) -> Allocator {
    allocator_with_flags(
        fake,
        block_size,
        heap_size,
        vk::MemoryAllocateFlags::empty(),
    )
}

fn allocator_with_flags(
    fake: &FakeDevice,
    block_size: u64,
    heap_size: u64,
    memory_allocate_flags: vk::MemoryAllocateFlags,
) -> Allocator {
    Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties: memory_properties(heap_size),
        non_coherent_atom_size: 1,
        block_size,
        memory_allocate_flags,
    })
}

fn description(size: u64, memory_type_bits: u32, kind: Kind) -> AllocationCreateDesc<'static> {
    let mut description = AllocationCreateDesc::new(
        vk::MemoryRequirements {
            size,
            alignment: 1,
            memory_type_bits,
        },
        MemoryLocation::CpuToGpu,
        true,
    );
    match kind {
        Kind::Shared => {}
        Kind::DeviceAddress => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
        Kind::DedicatedBuffer => {
            description.allocation_scheme = AllocationScheme::DedicatedBuffer(vk::Buffer(1));
        }
        Kind::Exportable => {
            description.export_handle_types = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
        }
        Kind::ZeroInitialized => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
        }
    }
    description
}

fn request(size: u64, kind: Kind) -> AllocationCreateDesc<'static> {
    description(size, 1 << PREFERRED | 1 << FALLBACK, kind)
}

fn shared_block_size(block_size: u64, heap_size: Option<u64>) -> u64 {
    heap_size.map_or(block_size.max(1), |heap_size| {
        block_size.max(1).min(heap_size)
    })
}

fn known(heap_size: u64) -> Option<u64> {
    (heap_size != 0).then_some(heap_size)
}

#[derive(Debug, Clone)]
struct Case {
    block_size: u64,
    heap_size: u64,
    size: u64,
    kind: Kind,
    empty_block_in_the_heap: bool,
    full_block: bool,
    allocator_flags: vk::MemoryAllocateFlags,
    results: Vec<vk::VkResult>,
}

#[derive(Debug, PartialEq, Eq)]
struct Expected {
    calls: Vec<(u32, u64)>,
    result: Result<(u32, u64), AllocationError>,
    frees_the_empty_block: bool,
}

fn documented_outcome(case: &Case) -> Expected {
    let mut results = case.results.iter().copied();
    let mut next = || results.next().unwrap_or(SUCCESS);
    let mut calls = Vec::new();
    let zero_initialized_by_the_allocator = case
        .allocator_flags
        .contains(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
    let mut empty_block = case.empty_block_in_the_heap && !zero_initialized_by_the_allocator;
    let mut frees_the_empty_block = false;
    let own = case.kind.has_memory_of_its_own() || zero_initialized_by_the_allocator;
    for (memory_type, heap_size) in [(PREFERRED, known(case.heap_size)), (FALLBACK, None)] {
        if heap_size.is_some_and(|heap_size| case.size > heap_size) {
            continue;
        }
        let first = if own {
            case.size
        } else {
            shared_block_size(case.block_size, heap_size).max(case.size)
        };
        let mut tried = vec![first];
        calls.push((memory_type, first));
        let mut result = next();
        if result == OOM && memory_type == PREFERRED && empty_block {
            empty_block = false;
            frees_the_empty_block = true;
            calls.push((memory_type, first));
            result = next();
        }
        if result == OOM && !own {
            for smaller in [first / 2, first / 4, first / 8] {
                if smaller < case.size {
                    continue;
                }
                tried.push(smaller);
                calls.push((memory_type, smaller));
                result = next();
                if result != OOM {
                    break;
                }
            }
            if result == OOM && !tried.contains(&case.size) {
                tried.push(case.size);
                calls.push((memory_type, case.size));
                result = next();
            }
        }
        match result {
            SUCCESS => {
                return Expected {
                    calls,
                    result: Ok((memory_type, *tried.last().unwrap())),
                    frees_the_empty_block,
                };
            }
            OOM => {}
            failure => {
                return Expected {
                    calls,
                    result: Err(AllocationError::Vulkan(failure)),
                    frees_the_empty_block,
                };
            }
        }
    }
    Expected {
        calls,
        result: Err(AllocationError::OutOfMemory),
        frees_the_empty_block,
    }
}

fn cases() -> impl Strategy<Value = Case> {
    (
        prop_oneof![
            1 => Just(0u64),
            2 => 1..=16u64,
            3 => 1..=4096u64,
            3 => select(vec![64u64, 256, 1024, 4096]),
        ],
        prop_oneof![1 => Just(0u64), 1 => 1..=8192u64],
    )
        .prop_flat_map(|(block_size, heap_size)| {
            let shared = shared_block_size(block_size, known(heap_size));
            let around = vec![
                shared / 8,
                shared / 8 + 1,
                shared / 4,
                shared / 2,
                shared / 2 + 1,
                shared - 1,
                shared,
                shared + 1,
            ];
            (
                Just(block_size),
                Just(heap_size),
                prop_oneof![
                    4 => 1..=(shared / 8).max(1),
                    3 => 1..=shared,
                    3 => select(around).prop_map(|size| size.max(1)),
                    1 => shared..=2 * shared + 1,
                ],
                prop_oneof![
                    6 => Just(Kind::Shared),
                    2 => Just(Kind::DeviceAddress),
                    1 => Just(Kind::DedicatedBuffer),
                    1 => Just(Kind::Exportable),
                    1 => Just(Kind::ZeroInitialized),
                ],
                any::<bool>(),
                prop::bool::weighted(0.3),
                prop_oneof![
                    4 => Just(vk::MemoryAllocateFlags::empty()),
                    1 => Just(vk::MemoryAllocateFlags::DEVICE_ADDRESS),
                    1 => Just(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT),
                ],
                prop::collection::vec(
                    prop_oneof![
                        3 => Just(SUCCESS),
                        6 => Just(OOM),
                        1 => Just(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
                        1 => Just(vk::VkResult::ERROR_TOO_MANY_OBJECTS),
                    ],
                    0..=14,
                ),
            )
        })
        .prop_map(
            |(
                block_size,
                heap_size,
                size,
                kind,
                empty_block_in_the_heap,
                full_block,
                allocator_flags,
                results,
            )| Case {
                block_size,
                heap_size,
                size,
                kind,
                empty_block_in_the_heap,
                full_block,
                allocator_flags,
                results,
            },
        )
}

fn calls_since(fake: &FakeDevice, before: usize) -> Vec<(u32, u64)> {
    fake.allocated_memory_types()
        .into_iter()
        .zip(fake.allocated_sizes())
        .skip(before)
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn out_of_memory_for_a_new_block_tries_smaller_blocks_before_the_next_memory_type(
        case in cases()
    ) {
        let fake = FakeDevice::new();
        let mut allocator =
            allocator_with_flags(&fake, case.block_size, case.heap_size, case.allocator_flags);
        let shared = shared_block_size(case.block_size, known(case.heap_size));
        let full = if case.full_block {
            Some(unsafe { allocator.allocate(description(shared, 1 << PREFERRED, Kind::Shared)) })
        } else {
            None
        };
        if case.empty_block_in_the_heap {
            let neighbour =
                unsafe { allocator.allocate(description(1, 1 << NEIGHBOUR, Kind::Shared)) };
            prop_assert!(neighbour.is_ok(), "{:?}: {:?}", case, neighbour);
            unsafe { allocator.free(neighbour.unwrap()) }.unwrap();
        }
        let full = full.transpose();
        prop_assert!(full.is_ok(), "{:?}: {:?}", case, full);
        let full = full.unwrap();

        fake.script_allocations(case.results.iter().copied());
        let before = fake.allocated_memory_types().len();
        let frees_before = fake.calls().free;
        let result = unsafe { allocator.allocate(request(case.size, case.kind)) };
        let expected = documented_outcome(&case);

        prop_assert_eq!(
            calls_since(&fake, before),
            expected.calls.clone(),
            "{:?}: the memory types and sizes given to vkAllocateMemory, in order", case
        );
        prop_assert_eq!(
            fake.calls().free - frees_before,
            usize::from(expected.frees_the_empty_block),
            "{:?}: vkFreeMemory calls for the empty block of the heap", case
        );
        let placed = result.as_ref().map_err(Clone::clone).map(|allocation: &Allocation| {
            (
                allocation.memory_type_index(),
                fake.memory_objects()[&allocation.memory()].size,
            )
        });
        prop_assert_eq!(
            placed,
            expected.result,
            "{:?}: the memory type of the allocation and the size of its new memory", case
        );
        if let Ok(allocation) = result {
            prop_assert_eq!(allocation.offset(), 0, "{:?}", case);
            prop_assert!(allocation.size() >= case.size, "{:?}", case);
            unsafe { allocator.free(allocation) }.unwrap();
        }
        if let Some(full) = full {
            unsafe { allocator.free(full) }.unwrap();
        }
        prop_assert!(allocator.is_empty(), "{:?}", case);
        drop(allocator);
        prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "{:?}", case);
        prop_assert!(fake.memory_objects().is_empty(), "{:?}", case);
    }
}

#[test]
fn a_full_preferred_heap_gives_a_quarter_of_a_block_before_it_gives_another_memory_type() {
    let fake = FakeDevice::new();
    let mut allocator = allocator(&fake, 4096, 16384);
    fake.script_allocations([OOM, OOM]);
    let allocation = unsafe { allocator.allocate(request(256, Kind::Shared)) }
        .expect("a block of a quarter of the block size has room");
    assert_eq!(
        calls_since(&fake, 0),
        [(PREFERRED, 4096), (PREFERRED, 2048), (PREFERRED, 1024)],
        "the block size, then half of it, then a quarter of it"
    );
    assert_eq!(
        allocation.memory_type_index(),
        PREFERRED,
        "the device-local memory type that CpuToGpu prefers"
    );
    assert_eq!(fake.memory_objects()[&allocation.memory()].size, 1024);
    let second = unsafe { allocator.allocate(request(768, Kind::Shared)) }.unwrap();
    assert_eq!(
        second.memory(),
        allocation.memory(),
        "the smaller block is shared by later allocations"
    );
    assert_eq!(fake.calls().allocate, 3);
    unsafe {
        allocator.free(allocation).unwrap();
        allocator.free(second).unwrap();
    }
}

#[test]
fn a_request_moves_to_the_next_memory_type_only_after_a_block_of_its_own_size_fails() {
    let fake = FakeDevice::new();
    let mut allocator = allocator(&fake, 64, 256);
    fake.script_allocations([OOM; 5]);
    let allocation = unsafe { allocator.allocate(request(3, Kind::Shared)) }.unwrap();
    assert_eq!(
        calls_since(&fake, 0),
        [
            (PREFERRED, 64),
            (PREFERRED, 32),
            (PREFERRED, 16),
            (PREFERRED, 8),
            (PREFERRED, 3),
            (FALLBACK, 64),
        ],
    );
    assert_eq!(allocation.memory_type_index(), FALLBACK);
    unsafe { allocator.free(allocation) }.unwrap();
}

#[test]
fn of_two_empty_blocks_the_larger_is_kept_whichever_is_emptied_last() {
    for larger_emptied_last in [true, false] {
        let fake = FakeDevice::new();
        let mut allocator = allocator(&fake, 64, 0);
        let full = unsafe { allocator.allocate(request(64, Kind::Shared)) }.unwrap();
        fake.script_allocations([OOM, OOM]);
        let small = unsafe { allocator.allocate(request(1, Kind::Shared)) }.unwrap();
        assert_eq!(
            calls_since(&fake, 0),
            [
                (PREFERRED, 64),
                (PREFERRED, 64),
                (PREFERRED, 32),
                (PREFERRED, 16)
            ]
        );
        let (first, last) = if larger_emptied_last {
            (small, full)
        } else {
            (full, small)
        };
        let first_memory = first.memory();
        let last_memory = last.memory();
        unsafe { allocator.free(first) }.unwrap();
        assert_eq!(
            (fake.calls().free, allocator.stats().block_count),
            (0, 2),
            "larger emptied last {larger_emptied_last}: the first block to be emptied is kept"
        );
        unsafe { allocator.free(last) }.unwrap();
        assert_eq!(
            fake.memory_objects()
                .iter()
                .map(|(&memory, object)| (memory, object.size))
                .collect::<Vec<_>>(),
            [(
                if larger_emptied_last {
                    last_memory
                } else {
                    first_memory
                },
                64
            )],
            "larger emptied last {larger_emptied_last}: the block of the block size is kept and \
             the block of 16 bytes is freed"
        );
        let again = unsafe { allocator.allocate(request(64, Kind::Shared)) }.unwrap();
        assert_eq!(
            fake.calls().allocate,
            4,
            "the kept block has room for a request of the block size"
        );
        unsafe { allocator.free(again) }.unwrap();
        drop(allocator);
        assert!(fake.memory_objects().is_empty());
        assert_eq!(fake.misuse(), Vec::<String>::new());
    }
}
