mod fake_device;

use fake_device::{FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, Random, desc};
use vulkan_alloc::AllocationError;

#[test]
fn a_rejected_free_changes_nothing() {
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
            }

            while !allocations.is_empty() {
                let index = random.below(allocations.len() as u64) as usize;
                let (allocation, raw_mappings) = allocations.swap_remove(index);
                let snapshot = allocator.snapshot();
                let memory_objects = fake.memory_objects();
                let calls = fake.calls();
                let result = allocator.free(allocation);
                if raw_mappings == 0 {
                    assert_eq!(result, Ok(()), "round {round}");
                    assert_eq!(
                        allocator.live_allocation_count(),
                        snapshot.stats.allocation_count - 1,
                        "round {round}"
                    );
                } else {
                    assert_eq!(
                        result,
                        Err(AllocationError::InvalidAllocation),
                        "round {round}"
                    );
                    assert_eq!(allocator.snapshot(), snapshot, "round {round}");
                    assert_eq!(fake.memory_objects(), memory_objects, "round {round}");
                    assert_eq!(fake.calls(), calls, "round {round}");
                }
            }
            allocator.free_all();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "round {round}");
        assert!(fake.memory_objects().is_empty(), "round {round}");
    }
}
