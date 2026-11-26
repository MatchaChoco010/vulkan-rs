mod fake_device;

use fake_device::{FakeDevice, Synchronization, SynchronizedRange, desc};
use proptest::prelude::*;
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationError, Allocator, AllocatorCreateDesc};

const BLOCK_SIZE: u64 = 16384;
const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;
const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Unmapped,
    MappedWithMapMemory,
    MappedPersistently,
    FreedByFreeAll,
    FromAnotherAllocator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    Allocator,
    Guard,
}

#[derive(Debug, Clone)]
struct Case {
    atom: u64,
    coherent_flags: vk::MemoryPropertyFlags,
    non_coherent_flags: vk::MemoryPropertyFlags,
    before: Vec<u64>,
    target: u64,
    after: Vec<u64>,
    state: State,
    route: Route,
    kind: Synchronization,
}

#[derive(Debug)]
struct Outcome {
    result: Result<(), AllocationError>,
    ranges: Vec<SynchronizedRange>,
    misuse: Vec<String>,
    target: (vk::DeviceMemory, u64, u64),
    neighbours: Vec<(vk::DeviceMemory, u64, u64)>,
}

fn cases() -> impl Strategy<Value = Case> {
    (
        (0u32..=8).prop_map(|shift| 1u64 << shift),
        prop::sample::select(vec![
            VISIBLE | COHERENT,
            VISIBLE | COHERENT | CACHED,
            LOCAL | VISIBLE | COHERENT,
        ]),
        prop::sample::select(vec![VISIBLE, VISIBLE | CACHED, LOCAL | VISIBLE]),
        prop::collection::vec(1..=700u64, 0..=2),
        1..=700u64,
        prop::collection::vec(1..=700u64, 0..=2),
        prop::sample::select(vec![
            State::Unmapped,
            State::MappedWithMapMemory,
            State::MappedPersistently,
            State::FreedByFreeAll,
            State::FromAnotherAllocator,
        ]),
        prop::sample::select(vec![Route::Allocator, Route::Guard]),
        prop::sample::select(vec![Synchronization::Flush, Synchronization::Invalidate]),
    )
        .prop_map(
            |(
                atom,
                coherent_flags,
                non_coherent_flags,
                before,
                target,
                after,
                state,
                route,
                kind,
            )| Case {
                atom,
                coherent_flags,
                non_coherent_flags,
                before,
                target,
                after,
                state,
                route,
                kind,
            },
        )
}

fn allocator_of(fake: &FakeDevice, flags: vk::MemoryPropertyFlags, atom: u64) -> Allocator {
    let mut memory_properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 1,
        memory_heap_count: 1,
        ..Default::default()
    };
    memory_properties.memory_types[0].property_flags = flags;
    Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties,
        non_coherent_atom_size: atom,
        block_size: BLOCK_SIZE,
    })
}

fn identity(allocation: &Allocation) -> (vk::DeviceMemory, u64, u64) {
    (allocation.memory(), allocation.offset(), allocation.size())
}

fn run(case: &Case, coherent: bool) -> Outcome {
    let flags = if coherent {
        case.coherent_flags
    } else {
        case.non_coherent_flags
    };
    let fake = FakeDevice::with_non_coherent_atom_size(case.atom);
    let other_fake = FakeDevice::with_non_coherent_atom_size(case.atom);
    let mut allocator = allocator_of(&fake, flags, case.atom);
    let mut other = allocator_of(&other_fake, flags, case.atom);
    unsafe {
        let mut allocate = |size: u64| allocator.allocate(desc(size, 1, 0)).unwrap();
        let mut neighbours = case
            .before
            .iter()
            .map(|&size| allocate(size))
            .collect::<Vec<_>>();
        let mut held = allocate(case.target);
        neighbours.extend(case.after.iter().map(|&size| allocate(size)));
        let foreign = other.allocate(desc(case.target, 1, 0)).unwrap();
        for neighbour in &neighbours {
            allocator.map_memory(neighbour).unwrap();
        }
        let target_identity = identity(if case.state == State::FromAnotherAllocator {
            &foreign
        } else {
            &held
        });
        let neighbour_identities = neighbours.iter().map(identity).collect::<Vec<_>>();
        match case.state {
            State::Unmapped | State::FromAnotherAllocator => {}
            State::MappedWithMapMemory => {
                allocator.map_memory(&held).unwrap();
            }
            State::MappedPersistently => {
                allocator.map_persistent(&mut held).unwrap();
            }
            State::FreedByFreeAll => {
                allocator.free_all();
            }
        }
        let target = if case.state == State::FromAnotherAllocator {
            &foreign
        } else {
            &held
        };
        let synchronized_before = fake.synchronized().len();
        let result = match case.route {
            Route::Allocator => match case.kind {
                Synchronization::Flush => allocator.flush_allocation(target),
                Synchronization::Invalidate => allocator.invalidate_allocation(target),
            },
            Route::Guard => allocator
                .map(target)
                .and_then(|mut mapped| match case.kind {
                    Synchronization::Flush => mapped.flush(),
                    Synchronization::Invalidate => mapped.invalidate(),
                }),
        };
        let ranges = fake.synchronized()[synchronized_before..].to_vec();
        allocator.free_all();
        other.free_all();
        Outcome {
            result,
            ranges,
            misuse: fake.misuse(),
            target: target_identity,
            neighbours: neighbour_identities,
        }
    }
}

fn documented_result(case: &Case) -> Result<(), AllocationError> {
    let held = !matches!(
        case.state,
        State::FreedByFreeAll | State::FromAnotherAllocator
    );
    let mapped = case.route == Route::Guard
        || matches!(
            case.state,
            State::MappedWithMapMemory | State::MappedPersistently
        );
    if held && mapped {
        Ok(())
    } else {
        Err(AllocationError::InvalidAllocation)
    }
}

fn check_range(case: &Case, outcome: &Outcome, range: &SynchronizedRange) {
    let (memory, offset, size) = outcome.target;
    assert_eq!(range.kind, case.kind, "the command that was called");
    assert_eq!(range.memory, memory, "the memory of the range");
    assert!(
        range.offset.is_multiple_of(case.atom),
        "the range starts at {}, not a multiple of the atom {}",
        range.offset,
        case.atom
    );
    let end = if range.size == vk::WHOLE_SIZE {
        BLOCK_SIZE
    } else {
        assert!(
            range.size.is_multiple_of(case.atom) || range.offset + range.size == BLOCK_SIZE,
            "the range of {} bytes is neither a multiple of the atom {} nor reaches the end of \
             the memory",
            range.size,
            case.atom
        );
        range.offset + range.size
    };
    assert!(
        range.offset <= offset && offset + size <= end,
        "the range {}..{end} does not cover the allocation at {offset} of {size} bytes",
        range.offset
    );
    for &(other_memory, other_offset, other_size) in &outcome.neighbours {
        assert!(
            other_memory != memory
                || other_offset + other_size <= range.offset
                || end <= other_offset,
            "the range {}..{end} reaches into the neighbour at {other_offset} of {other_size} bytes",
            range.offset
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn flushing_and_invalidating_skip_vulkan_for_coherent_memory_and_fail_alike(case in cases()) {
        let coherent = run(&case, true);
        let non_coherent = run(&case, false);
        let expected = documented_result(&case);
        prop_assert_eq!(&coherent.result, &expected, "the result for coherent memory");
        prop_assert_eq!(&non_coherent.result, &expected, "the result for memory that is not coherent");
        prop_assert_eq!(&coherent.misuse, &Vec::<String>::new(), "misuse of the device with coherent memory");
        prop_assert_eq!(&non_coherent.misuse, &Vec::<String>::new(), "misuse of the device with memory that is not coherent");
        prop_assert_eq!(&coherent.ranges, &Vec::new(), "ranges given to Vulkan for coherent memory");
        if expected.is_ok() {
            prop_assert_eq!(non_coherent.ranges.len(), 1, "ranges given to Vulkan for memory that is not coherent");
            check_range(&case, &non_coherent, &non_coherent.ranges[0]);
        } else {
            prop_assert_eq!(&non_coherent.ranges, &Vec::new(), "ranges given to Vulkan by a refused call");
        }
    }
}
