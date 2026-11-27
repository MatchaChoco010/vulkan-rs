mod fake_device;

use fake_device::{
    FakeDevice, HOST_COHERENT, MapFault, NON_COHERENT_ATOM_SIZE, assert_only_kept_empty_blocks,
    desc,
};
use proptest::prelude::*;
use vulkan::vk;
use vulkan_alloc::{AllocationError, Allocator, AllocatorCreateDesc};

#[cfg(target_pointer_width = "64")]
#[test]
fn pointers_are_the_block_address_plus_the_offset_for_any_offset() {
    for first_size in [
        1 << 12,
        1 << 40,
        isize::MAX as u64,
        1 << 63,
        (1 << 63) + 4096,
    ] {
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: FakeDevice::memory_properties(),
            non_coherent_atom_size: NON_COHERENT_ATOM_SIZE,
            block_size: u64::MAX,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        });
        unsafe {
            let first = allocator
                .allocate(desc(first_size, 1, HOST_COHERENT))
                .unwrap();
            let mut second = allocator.allocate(desc(4096, 1, HOST_COHERENT)).unwrap();
            assert_eq!(first.memory(), second.memory());
            assert_eq!((first.offset(), second.offset()), (0, first_size));

            let base = allocator.map_memory(&first).unwrap() as usize;
            let expected = base.wrapping_add(first_size as usize);
            assert_eq!(
                allocator.map_memory(&second).unwrap() as usize,
                expected,
                "offset {first_size}"
            );
            let persistent = allocator.map_persistent(&mut second).unwrap();
            assert_eq!(
                persistent.as_ptr() as usize,
                expected,
                "offset {first_size}"
            );
            assert_eq!(second.mapped_ptr(), Some(persistent));

            allocator.unmap_memory(&second).unwrap();
            allocator.unmap_memory(&first).unwrap();
            allocator.free(second).unwrap();
            allocator.free(first).unwrap();
        }
        assert_only_kept_empty_blocks(&allocator, |_| u64::MAX, &format!("offset {first_size}"));
        drop(allocator);
        assert_eq!(fake.misuse(), Vec::<String>::new(), "offset {first_size}");
        assert!(fake.memory_objects().is_empty(), "offset {first_size}");
    }
}

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn a_mapping_whose_address_does_not_fit_in_a_pointer_is_refused_and_undone(
        first_size in prop_oneof![1..=4096u64, 1..=(1u64 << 20)],
        room in prop_oneof![0..=4096usize, 0..=(2usize << 20)],
        persistent in any::<bool>(),
    ) {
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: FakeDevice::memory_properties(),
            non_coherent_atom_size: NON_COHERENT_ATOM_SIZE,
            block_size: u64::MAX,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        });
        unsafe {
            let first = allocator.allocate(desc(first_size, 1, HOST_COHERENT)).unwrap();
            let mut second = allocator.allocate(desc(4096, 1, HOST_COHERENT)).unwrap();
            prop_assert_eq!((first.memory(), second.offset()), (second.memory(), first_size));

            let base = usize::MAX - room;
            let snapshot = allocator.snapshot();
            let calls = fake.calls();
            fake.fault_next_map(MapFault::ReturnsAddress(base));
            let mapped = if persistent {
                allocator.map_persistent(&mut second).map(|pointer| pointer.as_ptr() as usize)
            } else {
                allocator.map_memory(&second).map(|pointer| pointer as usize)
            };
            let after = fake.calls();
            prop_assert_eq!(after.map, calls.map + 1);
            match base.checked_add(first_size as usize) {
                Some(address) => {
                    prop_assert_eq!(mapped, Ok(address));
                    prop_assert_eq!(after.unmap, calls.unmap);
                    prop_assert!(fake.memory_objects()[&second.memory()].mapped);
                    prop_assert_eq!(
                        second.mapped_ptr().map(|pointer| pointer.as_ptr() as usize),
                        persistent.then_some(address)
                    );
                }
                None => {
                    prop_assert_eq!(mapped, Err(AllocationError::MappingFailed));
                    prop_assert_eq!(after.unmap, calls.unmap + 1);
                    prop_assert!(!fake.memory_objects()[&second.memory()].mapped);
                    prop_assert_eq!(allocator.snapshot(), snapshot);
                    prop_assert_eq!(second.mapped_ptr(), None);
                    prop_assert!(allocator.map_memory(&first).is_ok());
                    prop_assert!(allocator.map_memory(&second).is_ok());
                }
            }
            prop_assert_eq!(allocator.free_all(), 2);
        }
        prop_assert_eq!(fake.misuse(), Vec::<String>::new());
    }
}
