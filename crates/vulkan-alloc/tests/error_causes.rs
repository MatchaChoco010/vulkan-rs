mod fake_device;

use std::collections::BTreeMap;

use fake_device::{
    DEVICE_LOCAL, FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, MapFault, Synchronization, desc,
};
use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationError, Allocator, FreeError};

const SMALLEST_TOO_LARGE_FOR_A_SLICE: u64 = isize::MAX as u64 + 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Holder {
    TheAllocator,
    NobodyAfterFreeAll,
    AnotherAllocator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Mapping {
    raw: usize,
    persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuardStep {
    Nothing,
    Whole(Synchronization),
    Range(Synchronization, u64, u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operation {
    Free,
    MapMemory,
    MapPersistent,
    UnmapMemory,
    Whole(Synchronization),
    Range(Synchronization, u64, u64),
    Guard(GuardStep),
}

#[derive(Debug, Clone)]
struct Case {
    memory_type: u32,
    size: u64,
    neighbour: Option<(u64, bool)>,
    mapping: Mapping,
    holder: Holder,
    reallocated: bool,
    map_fault: Option<MapFault>,
    synchronization_fault: Option<vk::VkResult>,
    operation: Operation,
}

fn kinds() -> impl Strategy<Value = Synchronization> {
    prop::sample::select(vec![Synchronization::Flush, Synchronization::Invalidate])
}

fn range_bounds() -> impl Strategy<Value = (u64, u64)> {
    let bound = prop_oneof![
        6 => 0..=700u64,
        1 => Just(u64::MAX),
        1 => Just(u64::MAX - 1),
        1 => Just(1u64 << 32),
    ];
    (bound.clone(), bound)
}

fn operations() -> impl Strategy<Value = Operation> {
    let guard_steps = prop_oneof![
        Just(GuardStep::Nothing),
        kinds().prop_map(GuardStep::Whole),
        (kinds(), range_bounds())
            .prop_map(|(kind, (offset, size))| GuardStep::Range(kind, offset, size)),
    ];
    prop_oneof![
        4 => Just(Operation::Free),
        3 => Just(Operation::MapMemory),
        3 => Just(Operation::MapPersistent),
        2 => Just(Operation::UnmapMemory),
        2 => kinds().prop_map(Operation::Whole),
        2 => (kinds(), range_bounds())
            .prop_map(|(kind, (offset, size))| Operation::Range(kind, offset, size)),
        4 => guard_steps.prop_map(Operation::Guard),
    ]
}

fn map_faults() -> impl Strategy<Value = Option<MapFault>> {
    prop::option::weighted(
        0.6,
        prop_oneof![
            1 => Just(MapFault::Fails(vk::VkResult::ERROR_MEMORY_MAP_FAILED)),
            1 => Just(MapFault::Fails(vk::VkResult::ERROR_OUT_OF_HOST_MEMORY)),
            3 => Just(MapFault::ReturnsNull),
            2 => (0..=64usize).prop_map(|room| MapFault::ReturnsAddress(usize::MAX - room)),
        ],
    )
}

fn cases() -> impl Strategy<Value = Case> {
    (
        prop::sample::select(vec![DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT]),
        prop_oneof![
            5 => 1..=600u64,
            1 => SMALLEST_TOO_LARGE_FOR_A_SLICE..=u64::MAX,
        ],
        prop::option::of((1..=600u64, any::<bool>())),
        (
            prop_oneof![2 => Just(0usize), 1 => 1..=2usize],
            any::<bool>(),
        ),
        prop::sample::select(vec![
            Holder::TheAllocator,
            Holder::TheAllocator,
            Holder::TheAllocator,
            Holder::NobodyAfterFreeAll,
            Holder::AnotherAllocator,
        ]),
        any::<bool>(),
        map_faults(),
        prop::option::of(prop::sample::select(vec![
            vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
            vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
        ])),
        operations(),
    )
        .prop_map(
            |(
                memory_type,
                size,
                neighbour,
                (raw, persistent),
                holder,
                reallocated,
                map_fault,
                synchronization_fault,
                operation,
            )| {
                let visible = memory_type != DEVICE_LOCAL;
                Case {
                    memory_type,
                    size,
                    neighbour: neighbour.map(|(size, mapped)| (size, mapped && visible)),
                    mapping: if visible {
                        Mapping { raw, persistent }
                    } else {
                        Mapping {
                            raw: 0,
                            persistent: false,
                        }
                    },
                    holder,
                    reallocated,
                    map_fault,
                    synchronization_fault,
                    operation,
                }
            },
        )
}

fn documented_result(case: &Case, offset: u64) -> Result<(), AllocationError> {
    if case.holder != Holder::TheAllocator {
        return Err(AllocationError::InvalidAllocation);
    }
    let visible = case.memory_type != DEVICE_LOCAL;
    let coherent = case.memory_type == HOST_COHERENT;
    let Mapping { raw, persistent } = case.mapping;
    let mapped = raw > 0 || persistent;
    let shares_a_block_with_a_mapped_neighbour = case.size <= 600
        && case
            .neighbour
            .is_some_and(|(_, neighbour_mapped)| neighbour_mapped);
    let block_is_mapped = mapped || shares_a_block_with_a_mapped_neighbour;
    let map = |as_a_slice: bool| {
        if !visible {
            return Err(AllocationError::NotHostVisible);
        }
        if as_a_slice && case.size >= SMALLEST_TOO_LARGE_FOR_A_SLICE {
            return Err(AllocationError::MappingFailed);
        }
        if block_is_mapped {
            return Ok(());
        }
        match case.map_fault {
            Some(MapFault::Fails(result)) => Err(AllocationError::Vulkan(result)),
            Some(MapFault::ReturnsNull) => Err(AllocationError::MappingFailed),
            Some(MapFault::ReturnsAddress(address))
                if address.checked_add(offset as usize).is_none() =>
            {
                Err(AllocationError::MappingFailed)
            }
            _ => Ok(()),
        }
    };
    let synchronize = |bytes: u64| match case.synchronization_fault {
        Some(result) if !coherent && bytes > 0 => Err(AllocationError::Vulkan(result)),
        _ => Ok(()),
    };
    let inside =
        |offset: u64, size: u64| offset.checked_add(size).is_some_and(|end| end <= case.size);
    match case.operation {
        Operation::Free if raw > 0 => Err(AllocationError::StillMapped),
        Operation::Free => Ok(()),
        Operation::MapMemory => map(false),
        Operation::MapPersistent if persistent => Ok(()),
        Operation::MapPersistent => map(false),
        Operation::UnmapMemory if raw == 0 => Err(AllocationError::NotMapped),
        Operation::UnmapMemory => Ok(()),
        Operation::Whole(_) if !mapped => Err(AllocationError::NotMapped),
        Operation::Whole(_) => synchronize(case.size),
        Operation::Range(_, offset, size) if !inside(offset, size) => {
            Err(AllocationError::RangeOutOfBounds)
        }
        Operation::Range(..) if !mapped => Err(AllocationError::NotMapped),
        Operation::Range(_, _, size) => synchronize(size),
        Operation::Guard(step) => {
            map(true)?;
            match step {
                GuardStep::Nothing => Ok(()),
                GuardStep::Whole(_) => synchronize(case.size),
                GuardStep::Range(_, offset, size) => {
                    let (offset, size) = guard_range(offset, size);
                    if inside(offset as u64, size as u64) {
                        synchronize(size as u64)
                    } else {
                        Err(AllocationError::RangeOutOfBounds)
                    }
                }
            }
        }
    }
}

fn guard_range(offset: u64, size: u64) -> (usize, usize) {
    (
        usize::try_from(offset).unwrap_or(usize::MAX),
        usize::try_from(size).unwrap_or(usize::MAX),
    )
}

unsafe fn prepare(allocator: &mut Allocator, case: &Case) -> (Option<Allocation>, Allocation) {
    unsafe {
        let neighbour = case.neighbour.map(|(size, mapped)| {
            let neighbour = allocator.allocate(desc(size, 1, case.memory_type)).unwrap();
            if mapped {
                allocator.map_memory(&neighbour).unwrap();
            }
            neighbour
        });
        let mut target = allocator
            .allocate(desc(case.size, 1, case.memory_type))
            .unwrap();
        for _ in 0..case.mapping.raw {
            allocator.map_memory(&target).unwrap();
        }
        if case.mapping.persistent {
            allocator.map_persistent(&mut target).unwrap();
        }
        (neighbour, target)
    }
}

unsafe fn apply(
    allocator: &mut Allocator,
    target: Allocation,
    operation: Operation,
) -> (Result<(), AllocationError>, Option<Allocation>) {
    unsafe {
        let mut target = target;
        let result = match operation {
            Operation::Free => {
                return match allocator.free(target) {
                    Ok(()) => (Ok(()), None),
                    Err(FreeError { allocation, error }) => (Err(error), Some(allocation)),
                };
            }
            Operation::MapMemory => allocator.map_memory(&target).map(drop),
            Operation::MapPersistent => allocator.map_persistent(&mut target).map(drop),
            Operation::UnmapMemory => allocator.unmap_memory(&target),
            Operation::Whole(Synchronization::Flush) => allocator.flush_allocation(&target),
            Operation::Whole(Synchronization::Invalidate) => {
                allocator.invalidate_allocation(&target)
            }
            Operation::Range(Synchronization::Flush, offset, size) => {
                allocator.flush_allocation_range(&target, offset, size)
            }
            Operation::Range(Synchronization::Invalidate, offset, size) => {
                allocator.invalidate_allocation_range(&target, offset, size)
            }
            Operation::Guard(step) => allocator.map(&target).and_then(|mut mapped| match step {
                GuardStep::Nothing => Ok(()),
                GuardStep::Whole(Synchronization::Flush) => mapped.flush(),
                GuardStep::Whole(Synchronization::Invalidate) => mapped.invalidate(),
                GuardStep::Range(kind, offset, size) => {
                    let (offset, size) = guard_range(offset, size);
                    match kind {
                        Synchronization::Flush => mapped.flush_range(offset, size),
                        Synchronization::Invalidate => mapped.invalidate_range(offset, size),
                    }
                }
            }),
        };
        (result, Some(target))
    }
}

fn identity(allocation: &Allocation) -> (vk::DeviceMemory, u64, u64, u32) {
    (
        allocation.memory(),
        allocation.offset(),
        allocation.size(),
        allocation.memory_type_index(),
    )
}

fn run(case: &Case) -> Result<Result<(), AllocationError>, TestCaseError> {
    let fake = FakeDevice::new();
    let other_fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mut other = other_fake.allocator();
    let expected;
    unsafe {
        let target = match case.holder {
            Holder::TheAllocator => prepare(&mut allocator, case).1,
            Holder::NobodyAfterFreeAll => {
                let (_, target) = prepare(&mut allocator, case);
                allocator.free_all();
                if case.reallocated {
                    prepare(&mut allocator, case);
                }
                target
            }
            Holder::AnotherAllocator => {
                if case.reallocated {
                    prepare(&mut allocator, case);
                }
                prepare(&mut other, case).1
            }
        };
        expected = documented_result(case, target.offset());
        let snapshot = allocator.snapshot();
        let other_snapshot = other.snapshot();
        let memory_objects = fake.memory_objects();
        let calls = fake.calls();
        let target_identity = identity(&target);
        let pointer = target.mapped_ptr();
        if let Some(fault) = case.map_fault {
            fake.fault_next_map(fault);
        }
        if let Some(result) = case.synchronization_fault {
            fake.fail_next_synchronization(result);
        }

        let (result, target) = apply(&mut allocator, target, case.operation);
        prop_assert_eq!(&result, &expected, "{:?}", case.operation);
        if let Err(error) = &result {
            let target = target
                .as_ref()
                .expect("a refused operation keeps the allocation");
            prop_assert_eq!(identity(target), target_identity, "the refused allocation");
            prop_assert_eq!(
                target.mapped_ptr(),
                pointer,
                "mapped_ptr() after {:?}",
                error
            );
            prop_assert_eq!(
                &allocator.snapshot(),
                &snapshot,
                "the allocator after {:?}",
                error
            );
            prop_assert_eq!(&other.snapshot(), &other_snapshot, "the other allocator");
            prop_assert_eq!(
                fake.memory_objects(),
                memory_objects,
                "the memory after {:?}",
                error
            );
            let after = fake.calls();
            prop_assert_eq!(
                (after.allocate, after.free),
                (calls.allocate, calls.free),
                "vkAllocateMemory and vkFreeMemory calls of a refused operation"
            );
            let refused_inside_a_guard = matches!(case.operation, Operation::Guard(_))
                && *error == AllocationError::RangeOutOfBounds;
            if !refused_inside_a_guard
                && !matches!(
                    error,
                    AllocationError::Vulkan(_) | AllocationError::MappingFailed
                )
            {
                prop_assert_eq!(
                    after,
                    calls,
                    "Vulkan calls of an operation refused with {:?}",
                    error
                );
            }
        }
        allocator.free_all();
        other.free_all();
    }
    prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "misuse of the device");
    prop_assert_eq!(
        other_fake.misuse(),
        Vec::<String>::new(),
        "misuse of the other device"
    );
    Ok(expected)
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn each_refusal_names_the_first_documented_cause_and_changes_nothing(case in cases()) {
        let _ = run(&case)?;
    }
}

fn name_of(result: &Result<(), AllocationError>) -> &'static str {
    match result {
        Ok(()) => "Ok",
        Err(AllocationError::InvalidAllocation) => "InvalidAllocation",
        Err(AllocationError::NotHostVisible) => "NotHostVisible",
        Err(AllocationError::NotMapped) => "NotMapped",
        Err(AllocationError::StillMapped) => "StillMapped",
        Err(AllocationError::MappingFailed) => "MappingFailed",
        Err(AllocationError::RangeOutOfBounds) => "RangeOutOfBounds",
        Err(AllocationError::Vulkan(_)) => "Vulkan",
        Err(_) => "an error no operation on an allocation returns",
    }
}

#[test]
fn the_generated_cases_reach_every_result_an_operation_on_an_allocation_can_have() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = cases();
    let mut reached = BTreeMap::<&str, usize>::new();
    for _ in 0..2048 {
        let case = strategy.new_tree(&mut runner).unwrap().current();
        let expected = run(&case).unwrap_or_else(|problem| panic!("{case:?}: {problem}"));
        *reached.entry(name_of(&expected)).or_default() += 1;
    }
    for result in [
        "Ok",
        "InvalidAllocation",
        "NotHostVisible",
        "NotMapped",
        "StillMapped",
        "MappingFailed",
        "RangeOutOfBounds",
        "Vulkan",
    ] {
        let count = reached.get(result).copied().unwrap_or(0);
        assert!(count >= 20, "{result} reached {count} times: {reached:?}");
    }
}

#[test]
fn a_free_refused_for_a_mapping_succeeds_once_the_mapping_is_released() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    unsafe {
        let allocation = allocator.allocate(desc(64, 1, HOST_COHERENT)).unwrap();
        allocator.map_memory(&allocation).unwrap();
        allocator.map_memory(&allocation).unwrap();
        let FreeError { allocation, error } = allocator.free(allocation).unwrap_err();
        assert_eq!(
            error,
            AllocationError::StillMapped,
            "freeing an allocation that holds mappings made with map_memory"
        );
        let mut released = 0;
        loop {
            match allocator.unmap_memory(&allocation) {
                Ok(()) => released += 1,
                Err(AllocationError::NotMapped) => break,
                Err(error) => panic!("unmap_memory of a held allocation: {error:?}"),
            }
        }
        assert_eq!(released, 2, "references released before NotMapped");
        assert_eq!(allocator.free(allocation), Ok(()));
        assert!(allocator.is_empty());
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
}

#[test]
fn a_free_of_an_allocation_of_another_allocator_is_refused_as_not_held() {
    let fake = FakeDevice::new();
    let other_fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mut other = other_fake.allocator();
    unsafe {
        let held = allocator.allocate(desc(64, 1, HOST_COHERENT)).unwrap();
        let foreign = other.allocate(desc(64, 1, HOST_COHERENT)).unwrap();
        assert_eq!(identity(&held), identity(&foreign));
        let FreeError { allocation, error } = allocator.free(foreign).unwrap_err();
        assert_eq!(
            error,
            AllocationError::InvalidAllocation,
            "freeing an allocation that another allocator made"
        );
        assert_eq!(
            allocator.unmap_memory(&allocation),
            Err(AllocationError::InvalidAllocation),
            "unmap_memory of an allocation that another allocator made"
        );
        assert_eq!(allocator.live_allocation_count(), 1);
        assert_eq!(other.free(allocation), Ok(()));
        assert_eq!(allocator.free(held), Ok(()));
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
    assert_eq!(other_fake.misuse(), Vec::<String>::new());
}
