mod fake_device;

use fake_device::{FakeDevice, HOST_COHERENT, desc};
use vulkan_alloc::AllocationError;

#[test]
fn the_guard_maps_only_allocations_a_slice_can_cover() {
    let largest = isize::MAX as u64;
    for size in [
        1 << 32,
        largest - 1,
        largest,
        largest + 1,
        largest + 2,
        u64::MAX - 1,
        u64::MAX,
    ] {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        unsafe {
            let allocation = allocator.allocate(desc(size, 1, HOST_COHERENT)).unwrap();
            assert_eq!(allocation.size(), size);
            let calls = fake.calls();
            let snapshot = allocator.snapshot();
            match allocator.map(&allocation) {
                Ok(mapped) => {
                    assert!(size <= largest, "{size} bytes were mapped as a slice");
                    assert!(!mapped.as_ptr().is_null());
                }
                Err(error) => {
                    assert!(size > largest, "{size} bytes were not mapped");
                    assert_eq!(error, AllocationError::InvalidAllocation);
                    assert_eq!(fake.calls(), calls, "{size} bytes");
                }
            }
            assert_eq!(allocator.snapshot(), snapshot, "{size} bytes");

            let pointer = allocator.map_memory(&allocation).unwrap();
            assert!(!pointer.is_null());
            allocator.unmap_memory(&allocation).unwrap();
            allocator.free(allocation).unwrap();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "{size} bytes");
        assert!(fake.memory_objects().is_empty(), "{size} bytes");
    }
}
