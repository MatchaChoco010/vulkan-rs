mod fake_device;

use std::collections::HashMap;

use fake_device::{
    DEVICE_LOCAL, FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, MemoryObject, Random, desc,
};
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationResourceClass, AllocationScheme,
    Allocator, FreeError, MemoryRegionState,
};

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

unsafe fn allocate_in_sequence(allocator: &mut Allocator, seed: u64) -> Vec<Allocation> {
    let mut random = Random::new(seed);
    let mut allocations = Vec::new();
    for _ in 0..1 + random.below(12) {
        let memory_type =
            [DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT][random.below(3) as usize];
        let size = 1 + random.below(2048);
        allocations.push(unsafe { allocator.allocate(desc(size, 1, memory_type)) }.unwrap());
    }
    allocations
}

unsafe fn map_host_visible(allocator: &mut Allocator, allocations: &[Allocation]) -> Vec<bool> {
    allocations
        .iter()
        .enumerate()
        .map(|(index, allocation)| {
            if allocation.memory_type_index() == DEVICE_LOCAL {
                return false;
            }
            unsafe {
                allocator.map_persistent(allocation).unwrap();
                if index % 2 == 0 {
                    allocator.map_memory(allocation).unwrap();
                    return true;
                }
            }
            false
        })
        .collect()
}

unsafe fn assert_not_held(allocator: &mut Allocator, fake: &FakeDevice, stale: Allocation) {
    let snapshot = allocator.snapshot();
    let memory_objects = fake.memory_objects();
    let calls = fake.calls();
    unsafe {
        assert_eq!(
            allocator.allocation_memory_flags(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(
            allocator.map_memory(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(
            allocator.map(&stale).err(),
            Some(AllocationError::InvalidAllocation)
        );
        assert_eq!(
            allocator.map_persistent(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(allocator.persistent_ptr(&stale), None);
        assert_eq!(
            allocator.unmap_memory(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(
            allocator.flush_allocation(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(
            allocator.invalidate_allocation(&stale),
            Err(AllocationError::InvalidAllocation)
        );
        let FreeError { error, .. } = allocator.free(stale).unwrap_err();
        assert_eq!(error, AllocationError::InvalidAllocation);
    }
    assert_eq!(allocator.snapshot(), snapshot);
    assert_eq!(fake.memory_objects(), memory_objects);
    assert_eq!(fake.calls(), calls);
}

#[test]
fn allocations_from_before_free_all_are_rejected_whatever_is_allocated_afterwards() {
    for seed in 1..200 {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        unsafe {
            let stale = allocate_in_sequence(&mut allocator, seed);
            let layout = allocator.snapshot();
            assert_eq!(allocator.free_all(), stale.len());

            let live = allocate_in_sequence(&mut allocator, seed);
            let memories = |snapshot: &vulkan_alloc::AllocatorSnapshot| {
                snapshot
                    .memory_types
                    .iter()
                    .flat_map(|memory_type| &memory_type.blocks)
                    .map(|block| (block.memory, block.size, block.allocated_bytes))
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                memories(&allocator.snapshot()),
                memories(&layout),
                "seed {seed}: the device reuses the handles, and the layout repeats"
            );
            for (stale, live) in stale.iter().zip(&live) {
                assert_eq!(
                    (stale.memory(), stale.offset(), stale.size()),
                    (live.memory(), live.offset(), live.size()),
                    "seed {seed}"
                );
                assert_ne!(stale, live, "seed {seed}");
            }

            let mapped = map_host_visible(&mut allocator, &live);
            for stale in stale {
                assert_not_held(&mut allocator, &fake, stale);
            }
            for (live, mapped) in live.into_iter().zip(mapped) {
                if mapped {
                    allocator.unmap_memory(&live).unwrap();
                }
                assert_eq!(allocator.free(live), Ok(()), "seed {seed}");
            }
        }
        assert!(allocator.is_empty(), "seed {seed}");
        assert_eq!(fake.misuse(), Vec::<String>::new(), "seed {seed}");
    }
}

#[test]
fn allocations_of_another_allocator_with_the_same_layout_are_rejected() {
    for seed in 1..200 {
        let first_device = FakeDevice::new();
        let second_device = FakeDevice::new();
        let mut first = first_device.allocator();
        let mut second = second_device.allocator();
        unsafe {
            let of_first = allocate_in_sequence(&mut first, seed);
            let of_second = allocate_in_sequence(&mut second, seed);
            for (a, b) in of_first.iter().zip(&of_second) {
                assert_eq!(
                    (a.memory(), a.offset(), a.size(), a.memory_type_index()),
                    (b.memory(), b.offset(), b.size(), b.memory_type_index()),
                    "seed {seed}"
                );
            }
            let mapped = map_host_visible(&mut second, &of_second);
            for foreign in of_first {
                assert_not_held(&mut second, &second_device, foreign);
            }
            assert_eq!(second.live_allocation_count(), of_second.len());
            for (own, mapped) in of_second.into_iter().zip(mapped) {
                if mapped {
                    second.unmap_memory(&own).unwrap();
                }
                assert_eq!(second.free(own), Ok(()), "seed {seed}");
            }
            first.free_all();
        }
        assert!(second.is_empty(), "seed {seed}");
        assert_eq!(first_device.misuse(), Vec::<String>::new(), "seed {seed}");
        assert_eq!(second_device.misuse(), Vec::<String>::new(), "seed {seed}");
    }
}

#[test]
fn byte_ranges_outside_the_allocation_are_none() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mut random = Random::new(0x2545_f491_4f6c_dd1d);
    unsafe {
        let _neighbour = allocator.allocate(desc(100, 1, HOST_COHERENT)).unwrap();
        let allocation = allocator.allocate(desc(1000, 1, HOST_COHERENT)).unwrap();
        let length = allocation.size() as usize;
        let mut mapped = allocator.map(&allocation).unwrap();
        let base = mapped.as_ptr() as usize;

        let interesting = [
            0,
            1,
            length - 1,
            length,
            length + 1,
            usize::MAX - 1,
            usize::MAX,
        ];
        let mut cases = Vec::new();
        for offset in interesting {
            for size in interesting {
                cases.push((offset, size));
            }
        }
        for _ in 0..2000 {
            let bound = if random.below(4) == 0 {
                u64::MAX
            } else {
                2 * length as u64
            };
            cases.push((random.below(bound) as usize, random.below(bound) as usize));
        }

        for (offset, size) in cases {
            let inside = offset.checked_add(size).is_some_and(|end| end <= length);
            let expected = inside.then(|| (base + offset, size));
            assert_eq!(
                mapped
                    .byte_range(offset, size)
                    .map(|bytes| (bytes.as_ptr() as usize, bytes.len())),
                expected,
                "byte_range({offset}, {size}) of {length} bytes"
            );
            assert_eq!(
                mapped
                    .byte_range_mut(offset, size)
                    .map(|bytes| (bytes.as_ptr() as usize, bytes.len())),
                expected,
                "byte_range_mut({offset}, {size}) of {length} bytes"
            );
        }
        drop(mapped);
        allocator.free_all();
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
}

#[test]
fn linear_and_non_linear_allocations_never_share_a_memory_object() {
    let mut random = Random::new(0x0123_4567_89ab_cdef);
    for round in 0..50 {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        let mut live: Vec<(Allocation, bool)> = Vec::new();
        unsafe {
            for step in 0..200 {
                if live.is_empty() || random.below(3) != 0 {
                    let linear = random.below(2) == 0;
                    let memory_type =
                        [DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT][random.below(3) as usize];
                    let flags = if random.below(4) == 0 {
                        vk::MemoryAllocateFlags::DEVICE_ADDRESS
                    } else {
                        vk::MemoryAllocateFlags::empty()
                    };
                    let export = if random.below(8) == 0 {
                        vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT
                    } else {
                        vk::ExternalMemoryHandleTypeFlags::empty()
                    };
                    let scheme = match (random.below(8), linear) {
                        (0, true) => AllocationScheme::DedicatedBuffer(vk::Buffer(step + 1)),
                        (0, false) => AllocationScheme::DedicatedImage(vk::Image(step + 1)),
                        _ => AllocationScheme::GpuAllocatorManaged,
                    };
                    let allocation = allocator
                        .allocate(AllocationCreateDesc {
                            linear,
                            memory_allocate_flags: flags,
                            export_handle_types: export,
                            allocation_scheme: scheme,
                            ..desc(1 + random.below(1500), 1 << random.below(9), memory_type)
                        })
                        .unwrap();
                    live.push((allocation, linear));
                } else {
                    let index = random.below(live.len() as u64) as usize;
                    let (allocation, _) = live.swap_remove(index);
                    allocator.free(allocation).unwrap();
                }

                let mut kinds: HashMap<vk::DeviceMemory, bool> = HashMap::new();
                for (allocation, linear) in &live {
                    let kind = *kinds.entry(allocation.memory()).or_insert(*linear);
                    assert_eq!(
                        kind,
                        *linear,
                        "round {round}, step {step}: {:?} holds linear and non-linear allocations",
                        allocation.memory()
                    );
                }
                let snapshot = allocator.snapshot();
                let blocks = snapshot
                    .memory_types
                    .iter()
                    .flat_map(|memory_type| &memory_type.blocks)
                    .collect::<Vec<_>>();
                assert_eq!(blocks.len(), kinds.len(), "round {round}, step {step}");
                for block in blocks {
                    let linear = match block.resource_class {
                        AllocationResourceClass::Linear => true,
                        AllocationResourceClass::Optimal => false,
                    };
                    assert_eq!(
                        kinds.get(&block.memory),
                        Some(&linear),
                        "round {round}, step {step}"
                    );
                }
            }
            allocator.free_all();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "round {round}");
    }
}
