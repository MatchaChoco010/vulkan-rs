mod fake_device;

use fake_device::{FakeDevice, HOST_COHERENT, NON_COHERENT_ATOM_SIZE, desc};
use vulkan_alloc::{Allocator, AllocatorCreateDesc};

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
        });
        unsafe {
            let first = allocator
                .allocate(desc(first_size, 1, HOST_COHERENT))
                .unwrap();
            let second = allocator.allocate(desc(4096, 1, HOST_COHERENT)).unwrap();
            assert_eq!(first.memory(), second.memory());
            assert_eq!((first.offset(), second.offset()), (0, first_size));

            let base = allocator.map_memory(&first).unwrap() as usize;
            let expected = base.wrapping_add(first_size as usize);
            assert_eq!(
                allocator.map_memory(&second).unwrap() as usize,
                expected,
                "offset {first_size}"
            );
            let persistent = allocator.map_persistent(&second).unwrap();
            assert_eq!(
                persistent.as_ptr() as usize,
                expected,
                "offset {first_size}"
            );
            assert_eq!(allocator.persistent_ptr(&second), Some(persistent));

            allocator.unmap_memory(&second).unwrap();
            allocator.unmap_memory(&first).unwrap();
            allocator.free(second).unwrap();
            allocator.free(first).unwrap();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "offset {first_size}");
        assert!(fake.memory_objects().is_empty(), "offset {first_size}");
    }
}
