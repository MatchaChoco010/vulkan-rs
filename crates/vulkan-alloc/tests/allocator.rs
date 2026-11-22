mod fake_device;

use std::collections::HashMap;

use fake_device::{FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, MemoryObject, Random, desc};
use vulkan_alloc::{AllocationError, Allocator, FreeError, MemoryRegionState};

fn assert_device_holds_what_the_allocator_reports(allocator: &Allocator, fake: &FakeDevice) {
    let mut reported = HashMap::new();
    for memory_type in &allocator.snapshot().memory_types {
        for block in &memory_type.blocks {
            let mapped = block.regions.iter().any(|region| {
                matches!(
                    region.state,
                    MemoryRegionState::Allocated { mapped_count, .. } if mapped_count > 0
                )
            });
            reported.insert(
                block.memory,
                MemoryObject {
                    size: block.size,
                    memory_type_index: memory_type.memory_type_index,
                    mapped,
                },
            );
        }
    }
    assert_eq!(fake.memory_objects(), reported);
}

#[test]
fn a_rejected_free_changes_nothing_and_returns_the_allocation() {
    let mut random = Random::new(0x9e37_79b9_7f4a_7c15);
    for round in 0..200 {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        let mut allocations = Vec::new();
        unsafe {
            for _ in 0..1 + random.below(6) {
                let memory_type = [HOST_COHERENT, HOST_NON_COHERENT][random.below(2) as usize];
                let allocation = allocator
                    .allocate(desc(1 + random.below(1024), 1, memory_type))
                    .unwrap();
                let persistent = random.below(2) == 0;
                let raw_mappings = random.below(3);
                if persistent {
                    allocator.map_persistent(&allocation).unwrap();
                }
                for _ in 0..raw_mappings {
                    allocator.map_memory(&allocation).unwrap();
                }
                allocations.push((allocation, raw_mappings));
                assert_device_holds_what_the_allocator_reports(&allocator, &fake);
            }

            while !allocations.is_empty() {
                let index = random.below(allocations.len() as u64) as usize;
                let (allocation, raw_mappings) = allocations.swap_remove(index);
                let snapshot = allocator.snapshot();
                let memory_objects = fake.memory_objects();
                let calls = fake.calls();
                let memory = allocation.memory();
                let offset = allocation.offset();
                let result = allocator.free(allocation);
                if raw_mappings == 0 {
                    assert_eq!(result, Ok(()), "round {round}");
                    assert_eq!(
                        allocator.live_allocation_count(),
                        snapshot.stats.allocation_count - 1,
                        "round {round}"
                    );
                } else {
                    let FreeError { allocation, error } = result.unwrap_err();
                    assert_eq!(error, AllocationError::InvalidAllocation, "round {round}");
                    assert_eq!(
                        (allocation.memory(), allocation.offset()),
                        (memory, offset),
                        "round {round}"
                    );
                    assert_eq!(allocator.snapshot(), snapshot, "round {round}");
                    assert_eq!(fake.memory_objects(), memory_objects, "round {round}");
                    assert_eq!(fake.calls(), calls, "round {round}");
                    allocator.unmap_memory(&allocation).unwrap();
                    allocations.push((allocation, raw_mappings - 1));
                }
                assert_device_holds_what_the_allocator_reports(&allocator, &fake);
            }
        }
        assert!(allocator.is_empty(), "round {round}");
        assert_eq!(fake.misuse(), Vec::<String>::new(), "round {round}");
        assert!(fake.memory_objects().is_empty(), "round {round}");
    }
}

#[test]
fn an_allocation_of_another_allocator_is_returned_and_stays_usable() {
    let fake = FakeDevice::new();
    let mut owner = fake.allocator();
    let mut other = fake.allocator();
    unsafe {
        let allocation = owner.allocate(desc(256, 1, HOST_COHERENT)).unwrap();
        let pointer = owner.map_persistent(&allocation).unwrap();
        other.allocate(desc(256, 1, HOST_COHERENT)).unwrap();
        let snapshot = owner.snapshot();

        let FreeError { allocation, error } = other.free(allocation).unwrap_err();
        assert_eq!(error, AllocationError::InvalidAllocation);
        assert_eq!(owner.snapshot(), snapshot);
        assert_eq!(owner.persistent_ptr(&allocation), Some(pointer));
        assert_eq!(owner.free(allocation), Ok(()));
        assert!(owner.is_empty());
        assert_eq!(other.free_all(), 1);
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
    assert!(fake.memory_objects().is_empty());
}
