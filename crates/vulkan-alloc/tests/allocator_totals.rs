mod fake_device;

use fake_device::{FakeDevice, HOST_COHERENT, desc};
use vulkan_alloc::Allocator;

const HALF_OF_THE_ADDRESS_SPACE: u64 = 1 << 63;

fn allocator_holding_two_blocks_of_half_the_address_space(fake: &FakeDevice) -> Allocator {
    let mut allocator = fake.allocator();
    for _ in 0..2 {
        let allocation =
            unsafe { allocator.allocate(desc(HALF_OF_THE_ADDRESS_SPACE, 1, HOST_COHERENT)) }
                .unwrap();
        assert_eq!(allocation.size(), HALF_OF_THE_ADDRESS_SPACE);
    }
    allocator
}

#[test]
fn totals_beyond_u64_max_saturate_instead_of_panicking() {
    let fake = FakeDevice::new();
    let allocator = allocator_holding_two_blocks_of_half_the_address_space(&fake);
    assert_eq!(fake.memory_objects().len(), 2);
    assert!(
        fake.memory_objects().values().all(|object| !object.mapped),
        "the test must not map its huge blocks"
    );

    let stats = allocator.stats();
    assert_eq!(
        stats.total_block_bytes,
        u64::MAX,
        "total_block_bytes saturates"
    );
    assert_eq!(stats.allocated_bytes, u64::MAX, "allocated_bytes saturates");
    assert_eq!(stats.free_bytes, 0);
    assert_eq!(stats.allocation_count, 2);
    assert_eq!(stats.block_count, 2);
    assert_eq!(allocator.snapshot().stats, stats);
    assert_eq!(allocator.live_allocation_count(), 2);
    assert!(!allocator.is_empty());
    drop(allocator);
}

#[test]
fn dropping_an_allocator_whose_live_allocations_exceed_u64_max_does_not_panic() {
    let fake = FakeDevice::new();
    let allocator = allocator_holding_two_blocks_of_half_the_address_space(&fake);
    drop(allocator);
    assert_eq!(fake.calls().map, 0, "the test must not map its huge blocks");
    assert_eq!(
        fake.memory_objects().len(),
        2,
        "dropping the allocator must not free the memory of live allocations"
    );
}

#[test]
fn free_all_reports_every_allocation_when_the_totals_exceed_u64_max() {
    let fake = FakeDevice::new();
    let mut allocator = allocator_holding_two_blocks_of_half_the_address_space(&fake);
    assert_eq!(unsafe { allocator.free_all() }, 2);
    assert!(allocator.is_empty());
    assert!(fake.memory_objects().is_empty());
    assert_eq!(fake.misuse(), Vec::<String>::new());
}
