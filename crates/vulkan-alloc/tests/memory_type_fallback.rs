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
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;
const PROTECTED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::PROTECTED;
const AMD: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD;
const AMD_FLAGS: vk::MemoryPropertyFlags = AMD.union(vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD);

const OOM: vk::VkResult = vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY;
const BLOCK_SIZE: u64 = 4096;
const HEAP_COUNT: usize = 4;
const TILE_MEMORY_HEAP: usize = 3;

const TYPE_FLAGS: [vk::MemoryPropertyFlags; 8] = [
    LOCAL,
    VISIBLE.union(COHERENT),
    VISIBLE.union(COHERENT).union(CACHED),
    VISIBLE.union(CACHED),
    LOCAL.union(VISIBLE).union(COHERENT),
    LOCAL.union(VISIBLE).union(COHERENT).union(CACHED),
    LOCAL.union(PROTECTED),
    LOCAL.union(VISIBLE).union(COHERENT).union(AMD),
];

const NAMEABLE: [vk::MemoryPropertyFlags; 6] = [LOCAL, VISIBLE, COHERENT, CACHED, PROTECTED, AMD];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Shared,
    DedicatedBuffer,
    DedicatedImage,
    Exportable,
    ZeroInitialized,
    DeviceAddress,
}

const KINDS: [Kind; 6] = [
    Kind::Shared,
    Kind::DedicatedBuffer,
    Kind::DedicatedImage,
    Kind::Exportable,
    Kind::ZeroInitialized,
    Kind::DeviceAddress,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EarlierBlock {
    Used(u64),
    Empty,
}

#[derive(Debug, Clone)]
struct Device {
    types: Vec<(vk::MemoryPropertyFlags, u32)>,
    heap_sizes: [u64; HEAP_COUNT],
    heap_count: u32,
    tile_memory: bool,
}

impl Device {
    fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: self.types.len() as u32,
            memory_heap_count: self.heap_count,
            ..Default::default()
        };
        for (memory_type, &(flags, heap)) in properties.memory_types.iter_mut().zip(&self.types) {
            memory_type.property_flags = flags;
            memory_type.heap_index = heap;
        }
        for (index, heap) in properties.memory_heaps[..HEAP_COUNT].iter_mut().enumerate() {
            heap.size = self.heap_sizes[index];
            if self.tile_memory && index == TILE_MEMORY_HEAP {
                heap.flags = vk::MemoryHeapFlags::TILE_MEMORY_QCOM;
            }
        }
        properties
    }

    fn reported_heap(&self, memory_type: usize) -> Option<usize> {
        let heap = self.types[memory_type].1 as usize;
        (heap < self.heap_count as usize && heap < HEAP_COUNT).then_some(heap)
    }

    fn known_heap_size(&self, memory_type: usize) -> Option<u64> {
        self.reported_heap(memory_type)
            .map(|heap| self.heap_sizes[heap])
            .filter(|&size| size != 0)
    }

    fn shared_block_size(&self, memory_type: usize) -> u64 {
        self.known_heap_size(memory_type)
            .map_or(BLOCK_SIZE, |heap| heap.min(BLOCK_SIZE))
    }

    fn is_always_left_out(&self, memory_type: usize, named: vk::MemoryPropertyFlags) -> bool {
        let flags = self.types[memory_type].0;
        let tile_memory =
            self.tile_memory && self.reported_heap(memory_type) == Some(TILE_MEMORY_HEAP);
        tile_memory
            || (flags.intersects(AMD_FLAGS) && !named.intersects(AMD_FLAGS))
            || (flags.contains(PROTECTED) && !named.contains(PROTECTED))
    }
}

fn devices() -> impl Strategy<Value = Device> {
    (
        prop::collection::vec((select(TYPE_FLAGS.to_vec()), 0..=4u32), 1..=6),
        [
            select(vec![0u64, 1000, 3000, 1 << 20]),
            select(vec![0u64, 1000, 3000, 1 << 20]),
            select(vec![0u64, 1000, 3000, 1 << 20]),
            select(vec![0u64, 1000, 3000, 1 << 20]),
        ],
        1..=4u32,
        any::<bool>(),
    )
        .prop_map(|(types, heap_sizes, heap_count, tile_memory)| Device {
            types,
            heap_sizes,
            heap_count,
            tile_memory,
        })
}

fn flags_of(mask: u8) -> vk::MemoryPropertyFlags {
    NAMEABLE
        .iter()
        .enumerate()
        .filter(|&(bit, _)| mask >> bit & 1 == 1)
        .fold(vk::MemoryPropertyFlags::empty(), |all, (_, &flag)| {
            all | flag
        })
}

fn locations() -> impl Strategy<Value = MemoryLocation> {
    prop_oneof![
        2 => Just(MemoryLocation::GpuOnly),
        2 => Just(MemoryLocation::CpuToGpu),
        2 => Just(MemoryLocation::GpuToCpu),
        3 => (
            prop_oneof![2 => Just(0u8), 2 => (0..6u32).prop_map(|bit| 1u8 << bit), 1 => 0..64u8],
            0..64u8,
        )
            .prop_map(|(required, preferred)| {
                MemoryLocation::custom(flags_of(required), flags_of(preferred))
            }),
    ]
}

fn documented_flags(
    location: MemoryLocation,
) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
    match location {
        MemoryLocation::GpuOnly => (LOCAL, LOCAL),
        MemoryLocation::CpuToGpu => (VISIBLE | COHERENT, VISIBLE | COHERENT),
        MemoryLocation::GpuToCpu => (VISIBLE | COHERENT, VISIBLE | COHERENT | CACHED),
        MemoryLocation::Custom {
            required,
            preferred,
        } => (required, preferred),
    }
}

fn allocate_memory_results() -> impl Strategy<Value = Vec<vk::VkResult>> {
    prop::collection::vec(
        prop_oneof![
            3 => Just(vk::VkResult::SUCCESS),
            5 => Just(OOM),
            1 => Just(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY),
            1 => Just(vk::VkResult::ERROR_TOO_MANY_OBJECTS),
        ],
        0..=7,
    )
}

fn description(
    size: u64,
    memory_type_bits: u32,
    location: MemoryLocation,
    kind: Kind,
) -> AllocationCreateDesc<'static> {
    let mut description = AllocationCreateDesc {
        name: None,
        requirements: vk::MemoryRequirements {
            size,
            alignment: 1,
            memory_type_bits,
        },
        location,
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    };
    match kind {
        Kind::Shared => {}
        Kind::DedicatedBuffer => {
            description.allocation_scheme = AllocationScheme::DedicatedBuffer(vk::Buffer(1));
        }
        Kind::DedicatedImage => {
            description.allocation_scheme = AllocationScheme::DedicatedImage(vk::Image(1));
        }
        Kind::Exportable => {
            description.export_handle_types = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
        }
        Kind::ZeroInitialized => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
        }
        Kind::DeviceAddress => {
            description.memory_allocate_flags = vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
    }
    description
}

#[derive(Debug, PartialEq, Eq)]
struct Expected {
    result: Result<u32, AllocationError>,
    allocated_memory_types: Vec<u32>,
    new_memory_size: Option<u64>,
    freed_empty_blocks: Vec<u32>,
}

fn documented_outcome(
    device: &Device,
    size: u64,
    memory_type_bits: u32,
    location: MemoryLocation,
    kind: Kind,
    mut earlier_blocks: Vec<Option<EarlierBlock>>,
    allocate_memory_results: &[vk::VkResult],
) -> Expected {
    let (required, preferred) = documented_flags(location);
    let named = required | preferred;
    let mut candidates = (0..device.types.len())
        .filter(|&index| memory_type_bits >> index & 1 == 1)
        .filter(|&index| device.types[index].0.contains(required))
        .filter(|&index| !device.is_always_left_out(index, named))
        .collect::<Vec<_>>();
    let any_candidate = !candidates.is_empty();
    let mut results = allocate_memory_results.iter().copied();
    let mut allocated_memory_types = Vec::new();
    let mut freed_empty_blocks = Vec::new();
    let preferred_count =
        |index: usize| (device.types[index].0.as_raw() & preferred.as_raw()).count_ones();
    loop {
        let Some(most) = candidates.iter().copied().map(preferred_count).max() else {
            let result = Err(if any_candidate {
                AllocationError::OutOfMemory
            } else {
                AllocationError::NoCompatibleMemoryType
            });
            return Expected {
                result,
                allocated_memory_types,
                new_memory_size: None,
                freed_empty_blocks,
            };
        };
        let chosen = candidates
            .iter()
            .copied()
            .find(|&index| device.types[index].0.contains(preferred))
            .or_else(|| {
                candidates
                    .iter()
                    .copied()
                    .find(|&index| preferred_count(index) == most)
            })
            .unwrap();
        candidates.retain(|&index| index != chosen);
        if device
            .known_heap_size(chosen)
            .is_some_and(|heap| size > heap)
        {
            continue;
        }
        let used = match earlier_blocks[chosen] {
            Some(EarlierBlock::Used(used)) => Some(used),
            Some(EarlierBlock::Empty) => Some(0),
            None => None,
        };
        if kind == Kind::Shared
            && used.is_some_and(|used| size <= device.shared_block_size(chosen) - used)
        {
            return Expected {
                result: Ok(chosen as u32),
                allocated_memory_types,
                new_memory_size: None,
                freed_empty_blocks,
            };
        }
        allocated_memory_types.push(chosen as u32);
        let mut result = results.next().unwrap_or(vk::VkResult::SUCCESS);
        let heap = device.types[chosen].1;
        let empty_in_heap = (0..device.types.len())
            .filter(|&index| device.types[index].1 == heap)
            .filter(|&index| earlier_blocks[index] == Some(EarlierBlock::Empty))
            .collect::<Vec<_>>();
        if result == OOM && !empty_in_heap.is_empty() {
            for index in empty_in_heap {
                earlier_blocks[index] = None;
                freed_empty_blocks.push(index as u32);
            }
            allocated_memory_types.push(chosen as u32);
            result = results.next().unwrap_or(vk::VkResult::SUCCESS);
        }
        match result {
            vk::VkResult::SUCCESS => {
                let new_memory_size = if matches!(kind, Kind::Shared | Kind::DeviceAddress) {
                    device.shared_block_size(chosen).max(size)
                } else {
                    size
                };
                return Expected {
                    result: Ok(chosen as u32),
                    allocated_memory_types,
                    new_memory_size: Some(new_memory_size),
                    freed_empty_blocks,
                };
            }
            OOM => continue,
            failure => {
                return Expected {
                    result: Err(AllocationError::Vulkan(failure)),
                    allocated_memory_types,
                    new_memory_size: None,
                    freed_empty_blocks,
                };
            }
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn allocate_tries_the_next_memory_type_until_one_has_the_memory(
        device in devices(),
        size in prop_oneof![3 => 1..=1200u64, 1 => 1..=5000u64],
        memory_type_bits in prop_oneof![2 => Just(u32::MAX), 1 => any::<u32>()],
        location in locations(),
        kind in prop_oneof![4 => Just(Kind::Shared), 1 => select(KINDS.to_vec())],
        earlier in prop::collection::vec((0..6usize, prop::bool::weighted(0.4)), 0..=5),
        allocate_memory_results in allocate_memory_results(),
    ) {
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: device.memory_properties(),
            non_coherent_atom_size: 1,
            block_size: BLOCK_SIZE,
        });
        let mut earlier_blocks = vec![None; device.types.len()];
        let mut earlier_allocations = Vec::<Allocation>::new();
        let mut emptied = vec![false; device.types.len()];
        for (memory_type, empty_afterwards) in earlier {
            let memory_type = memory_type % device.types.len();
            emptied[memory_type] |= empty_afterwards;
            let request = description(
                1,
                1 << memory_type,
                MemoryLocation::custom(
                    vk::MemoryPropertyFlags::empty(),
                    vk::MemoryPropertyFlags::empty(),
                ),
                Kind::Shared,
            );
            if let Ok(allocation) = unsafe { allocator.allocate(request) } {
                prop_assert_eq!(allocation.memory_type_index(), memory_type as u32);
                earlier_allocations.push(allocation);
            }
        }
        let (to_free, kept): (Vec<_>, Vec<_>) = earlier_allocations
            .into_iter()
            .partition(|allocation| emptied[allocation.memory_type_index() as usize]);
        for allocation in to_free {
            earlier_blocks[allocation.memory_type_index() as usize] = Some(EarlierBlock::Empty);
            unsafe { allocator.free(allocation) }.unwrap();
        }
        for allocation in &kept {
            let block = &mut earlier_blocks[allocation.memory_type_index() as usize];
            let used = match block {
                Some(EarlierBlock::Used(used)) => *used,
                _ => 0,
            };
            *block = Some(EarlierBlock::Used(used + allocation.size()));
        }
        let earlier_allocations = kept;
        let context = format!(
            "{device:?}, size {size}, memory type bits {memory_type_bits:#b}, {location:?}, \
             {kind:?}, earlier blocks {earlier_blocks:?}, vkAllocateMemory \
             returns {allocate_memory_results:?}"
        );

        fake.script_allocations(allocate_memory_results.iter().copied());
        let calls_before = fake.allocated_memory_types().len();
        let live_before = allocator.live_allocations();
        let free_calls_before = fake.calls().free;
        let memory_before = fake.memory_objects();
        let result = unsafe {
            allocator.allocate(description(size, memory_type_bits, location, kind))
        };
        let expected = documented_outcome(
            &device,
            size,
            memory_type_bits,
            location,
            kind,
            earlier_blocks.clone(),
            &allocate_memory_results,
        );

        prop_assert_eq!(
            &fake.allocated_memory_types()[calls_before..],
            &expected.allocated_memory_types[..],
            "{}: the memory types vkAllocateMemory was called with, in order", context
        );
        prop_assert_eq!(
            result.as_ref().map(Allocation::memory_type_index).map_err(Clone::clone),
            expected.result.clone(),
            "{}: the result and the memory type of the allocation", context
        );
        let memory_of = |mut objects: Vec<(u32, u64)>| {
            objects.sort_unstable();
            objects
        };
        let memory_after = fake
            .memory_objects()
            .values()
            .map(|object| (object.memory_type_index, object.size))
            .collect::<Vec<_>>();
        let mut expected_memory = memory_before
            .values()
            .map(|object| (object.memory_type_index, object.size))
            .collect::<Vec<_>>();
        for &memory_type in &expected.freed_empty_blocks {
            let empty_block = (memory_type, device.shared_block_size(memory_type as usize));
            let position = expected_memory.iter().position(|&object| object == empty_block);
            prop_assert!(position.is_some(), "{}: no empty block of type {}", context, memory_type);
            expected_memory.swap_remove(position.unwrap());
        }
        if let (Some(new_size), Ok(memory_type)) = (expected.new_memory_size, &expected.result) {
            expected_memory.push((*memory_type, new_size));
        }
        prop_assert_eq!(
            memory_of(memory_after),
            memory_of(expected_memory),
            "{}: the memory types and sizes of the memory on the device", context
        );
        prop_assert_eq!(
            fake.calls().free - free_calls_before,
            expected.freed_empty_blocks.len(),
            "{}: vkFreeMemory calls", context
        );
        match result {
            Ok(allocation) => {
                prop_assert!(allocation.size() >= size, "{}", context);
                prop_assert_eq!(
                    allocation.memory_properties(),
                    device.types[allocation.memory_type_index() as usize].0,
                    "{}: the flags of the memory type the allocation was made in", context
                );
                unsafe { allocator.free(allocation) }.unwrap();
            }
            Err(_) => prop_assert_eq!(
                allocator.live_allocations(),
                live_before,
                "{}: a failed allocation must leave the allocations as they were", context
            ),
        }

        for allocation in earlier_allocations {
            unsafe { allocator.free(allocation) }.unwrap();
        }
        prop_assert!(allocator.is_empty(), "{}", context);
        drop(allocator);
        prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "{}", context);
        prop_assert!(fake.memory_objects().is_empty(), "{}", context);
    }
}

fn allocator_for(fake: &FakeDevice, types: &[(vk::MemoryPropertyFlags, u64)]) -> Allocator {
    let mut properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: types.len() as u32,
        memory_heap_count: types.len() as u32,
        ..Default::default()
    };
    for (index, &(flags, heap_size)) in types.iter().enumerate() {
        properties.memory_types[index].property_flags = flags;
        properties.memory_types[index].heap_index = index as u32;
        properties.memory_heaps[index].size = heap_size;
    }
    Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties: properties,
        non_coherent_atom_size: 1,
        block_size: BLOCK_SIZE,
    })
}

fn allocate(
    allocator: &mut Allocator,
    size: u64,
    location: MemoryLocation,
) -> Result<Allocation, AllocationError> {
    unsafe { allocator.allocate(description(size, u32::MAX, location, Kind::Shared)) }
}

#[test]
fn gpu_to_cpu_takes_coherent_memory_when_the_cached_heap_is_out_of_memory() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(
        &fake,
        &[
            (LOCAL, 1 << 30),
            (VISIBLE | COHERENT, 1 << 30),
            (VISIBLE | COHERENT | CACHED, 1 << 30),
        ],
    );
    fake.script_allocations([OOM]);
    let allocation = allocate(&mut allocator, 256, MemoryLocation::GpuToCpu)
        .expect("the coherent type that is not cached has memory");
    assert_eq!(
        allocation.memory_type_index(),
        1,
        "the allocation is made in the type that was tried second"
    );
    assert_eq!(
        fake.allocated_memory_types(),
        [2, 1],
        "the cached type is tried first, then the coherent type"
    );
    assert_eq!(allocator.snapshot().memory_types.len(), 1);
    unsafe { allocator.free(allocation) }.unwrap();
}

#[test]
fn allocate_falls_back_past_a_heap_that_is_too_small_without_a_vulkan_call() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(
        &fake,
        &[(LOCAL | VISIBLE | COHERENT, 1000), (LOCAL, 1 << 30)],
    );
    let allocation = allocate(&mut allocator, 2000, MemoryLocation::GpuOnly)
        .expect("the second device-local type has a large heap");
    assert_eq!(allocation.memory_type_index(), 1);
    assert_eq!(
        fake.allocated_memory_types(),
        [1],
        "the type whose heap is too small is not given to vkAllocateMemory"
    );
    unsafe { allocator.free(allocation) }.unwrap();
}

#[test]
fn allocate_is_out_of_memory_only_when_every_memory_type_is() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(
        &fake,
        &[
            (LOCAL, 1 << 30),
            (LOCAL | VISIBLE | COHERENT, 1000),
            (LOCAL, 1 << 30),
        ],
    );
    fake.script_allocations([OOM, OOM]);
    assert_eq!(
        allocate(&mut allocator, 2000, MemoryLocation::GpuOnly),
        Err(AllocationError::OutOfMemory)
    );
    assert_eq!(fake.allocated_memory_types(), [0, 2]);
    assert!(allocator.snapshot().memory_types.is_empty());
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn allocate_returns_an_error_that_is_not_out_of_memory_without_trying_another_type() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(&fake, &[(LOCAL, 1 << 30), (LOCAL, 1 << 30)]);
    fake.script_allocations([vk::VkResult::ERROR_OUT_OF_HOST_MEMORY]);
    assert_eq!(
        allocate(&mut allocator, 256, MemoryLocation::GpuOnly),
        Err(AllocationError::Vulkan(
            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
        ))
    );
    assert_eq!(fake.allocated_memory_types(), [0]);
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn a_dedicated_allocation_gets_the_memory_of_the_empty_block_of_its_heap() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(&fake, &[(LOCAL, BLOCK_SIZE)]);
    let shared = allocate(&mut allocator, 256, MemoryLocation::GpuOnly).unwrap();
    unsafe { allocator.free(shared) }.unwrap();
    assert_eq!(
        fake.memory_objects().len(),
        1,
        "the empty block that fills the heap is kept"
    );

    fake.script_allocations([OOM]);
    let dedicated = unsafe {
        allocator.allocate(description(
            256,
            u32::MAX,
            MemoryLocation::GpuOnly,
            Kind::DedicatedBuffer,
        ))
    }
    .expect("freeing the empty block makes room for the dedicated allocation");
    assert_eq!(
        fake.allocated_memory_types(),
        [0, 0, 0],
        "the shared block, the dedicated memory that did not fit, and the retry"
    );
    assert_eq!(fake.calls().free, 1, "the empty block is freed");
    assert_eq!(
        fake.memory_objects()
            .values()
            .map(|object| object.size)
            .collect::<Vec<_>>(),
        [256]
    );
    unsafe { allocator.free(dedicated) }.unwrap();
}

#[test]
fn an_empty_block_of_another_heap_is_not_freed_when_a_heap_is_out_of_memory() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_for(&fake, &[(LOCAL, 1 << 30), (LOCAL, 1 << 30)]);
    let shared = unsafe {
        allocator.allocate(description(
            256,
            0b10,
            MemoryLocation::GpuOnly,
            Kind::Shared,
        ))
    }
    .unwrap();
    unsafe { allocator.free(shared) }.unwrap();

    fake.script_allocations([OOM]);
    assert_eq!(
        unsafe {
            allocator.allocate(description(
                256,
                0b01,
                MemoryLocation::GpuOnly,
                Kind::DedicatedBuffer,
            ))
        },
        Err(AllocationError::OutOfMemory)
    );
    assert_eq!(
        fake.allocated_memory_types(),
        [1, 0],
        "no retry in memory type 0"
    );
    assert_eq!(
        fake.calls().free,
        0,
        "the empty block of the other heap is kept"
    );
    assert_eq!(fake.memory_objects().len(), 1);
}
