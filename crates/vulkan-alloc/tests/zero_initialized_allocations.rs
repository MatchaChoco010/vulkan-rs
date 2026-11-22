mod fake_device;

use fake_device::{DEVICE_LOCAL, FakeDevice, HOST_COHERENT, Random, desc};
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationCreateDesc};

const ZERO_INITIALIZE: vk::MemoryAllocateFlags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;

fn zero_initialized(size: u64, memory_type_index: u32) -> AllocationCreateDesc<'static> {
    AllocationCreateDesc {
        memory_allocate_flags: ZERO_INITIALIZE,
        ..desc(size, 1, memory_type_index)
    }
}

#[test]
fn a_zero_initialized_allocation_never_sees_the_bytes_of_a_freed_allocation() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    unsafe {
        let keep = allocator
            .allocate(zero_initialized(256, HOST_COHERENT))
            .unwrap();
        allocator.map_persistent(&keep).unwrap();

        let written = allocator
            .allocate(zero_initialized(256, HOST_COHERENT))
            .unwrap();
        {
            let mut mapped = allocator.map(&written).unwrap();
            mapped.as_bytes_mut().fill(0xAA);
        }
        allocator.free(written).unwrap();

        let fresh = allocator
            .allocate(zero_initialized(256, HOST_COHERENT))
            .unwrap();
        {
            let mapped = allocator.map(&fresh).unwrap();
            assert!(
                mapped.as_bytes().iter().all(|&byte| byte == 0),
                "a zero-initialized allocation holds bytes written to a freed allocation"
            );
        }
        allocator.free(fresh).unwrap();
        allocator.free(keep).unwrap();
    }
    assert_eq!(fake.misuse(), Vec::<String>::new());
    assert!(fake.memory_objects().is_empty());
}

struct Live {
    allocation: Allocation,
    zero_initialized: bool,
    requested_size: u64,
}

#[test]
fn zero_initialized_allocations_always_get_memory_of_their_own() {
    for seed in 1..=64u64 {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        let mut random = Random::new(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut live: Vec<Live> = Vec::new();
        for step in 0..300 {
            if !live.is_empty() && (live.len() >= 48 || random.below(5) < 2) {
                let index = random.below(live.len() as u64) as usize;
                let freed = live.swap_remove(index);
                unsafe { allocator.free(freed.allocation) }.unwrap();
            } else {
                let memory_type_index = if random.below(2) == 0 {
                    DEVICE_LOCAL
                } else {
                    HOST_COHERENT
                };
                let size = 1 + random.below(2048);
                let alignment = [1, 16, 256][random.below(3) as usize];
                let memory_allocate_flags = [
                    vk::MemoryAllocateFlags::empty(),
                    vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                    ZERO_INITIALIZE,
                    ZERO_INITIALIZE | vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                ][random.below(4) as usize];
                let zero_initialized = memory_allocate_flags.contains(ZERO_INITIALIZE);
                let allocations_before = fake.calls().allocate;
                let allocation = unsafe {
                    allocator.allocate(AllocationCreateDesc {
                        memory_allocate_flags,
                        ..desc(size, alignment, memory_type_index)
                    })
                }
                .unwrap();
                if zero_initialized {
                    assert_eq!(
                        fake.calls().allocate,
                        allocations_before + 1,
                        "seed {seed} step {step}: a zero-initialized allocation must make exactly one vkAllocateMemory call"
                    );
                }
                live.push(Live {
                    allocation,
                    zero_initialized,
                    requested_size: size,
                });
            }

            let objects = fake.memory_objects();
            for (index, zeroed) in live.iter().enumerate() {
                if !zeroed.zero_initialized {
                    continue;
                }
                let memory = zeroed.allocation.memory();
                assert_eq!(
                    zeroed.allocation.offset(),
                    0,
                    "seed {seed} step {step}: a zero-initialized allocation must start at offset 0"
                );
                assert_eq!(
                    objects.get(&memory).map(|object| object.size),
                    Some(zeroed.requested_size),
                    "seed {seed} step {step}: the memory of a zero-initialized allocation must be exactly the requested size"
                );
                for (other_index, other) in live.iter().enumerate() {
                    assert!(
                        other_index == index || other.allocation.memory() != memory,
                        "seed {seed} step {step}: a zero-initialized allocation shares its VkDeviceMemory with another live allocation"
                    );
                }
            }
        }
        for remaining in live {
            unsafe { allocator.free(remaining.allocation) }.unwrap();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "seed {seed}");
        assert!(fake.memory_objects().is_empty(), "seed {seed}");
    }
}
