mod fake_device;

use fake_device::{FakeDevice, Random, desc};
use vulkan::vk;
use vulkan_alloc::{Allocation, Allocator, AllocatorCreateDesc};

fn pick(random: &mut Random, choices: &[u64]) -> u64 {
    choices[random.below(choices.len() as u64) as usize]
}

fn random_memory_properties(random: &mut Random) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 1 + random.below(4) as u32,
        memory_heap_count: random.below(vk::MAX_MEMORY_HEAPS as u64 + 8) as u32,
        ..Default::default()
    };
    for heap in &mut properties.memory_heaps {
        heap.size = match random.below(8) {
            0 => 0,
            1 => 1 + random.below(64),
            2 => 1 + random.below(1 << 20),
            _ => pick(
                random,
                &[1, 255, 4096, 1 << 20, 64 << 20, 1 << 40, u64::MAX],
            ),
        };
    }
    for memory_type in &mut properties.memory_types {
        memory_type.heap_index = random.below(vk::MAX_MEMORY_HEAPS as u64 + 8) as u32;
    }
    properties
}

fn expected_shared_block_size(
    properties: &vk::PhysicalDeviceMemoryProperties,
    memory_type_index: u32,
    block_size: u64,
    allocation_size: u64,
) -> u64 {
    let heap_count = properties
        .memory_heap_count
        .min(vk::MAX_MEMORY_HEAPS as u32);
    let heap_index = properties.memory_types[memory_type_index as usize].heap_index;
    let heap_size = if heap_index < heap_count {
        properties.memory_heaps[heap_index as usize].size
    } else {
        0
    };
    let block_size = block_size.max(1);
    let block_size = if heap_size == 0 {
        block_size
    } else {
        block_size.min(heap_size)
    };
    block_size.max(allocation_size)
}

#[test]
fn new_shared_blocks_take_the_smaller_of_the_block_size_and_the_heap_size() {
    for seed in 1..=200u64 {
        let mut random = Random::new(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let properties = random_memory_properties(&mut random);
        let block_size = match random.below(4) {
            0 => 1 + random.below(8192),
            _ => pick(&mut random, &[0, 1, 4096, 1 << 20, 64 << 20, u64::MAX]),
        };
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: properties,
            non_coherent_atom_size: 1,
            block_size,
        });
        let mut live: Vec<Allocation> = Vec::new();
        for step in 0..100 {
            if !live.is_empty() && (live.len() >= 32 || random.below(3) == 0) {
                let index = random.below(live.len() as u64) as usize;
                unsafe { allocator.free(live.swap_remove(index)) }.unwrap();
                continue;
            }
            let memory_type_index = random.below(u64::from(properties.memory_type_count)) as u32;
            let size = 1 + random.below(8192);
            let alignment = pick(&mut random, &[1, 16, 256]);
            let mut description = desc(size, alignment, memory_type_index);
            description.linear = random.below(2) == 0;
            let before = fake.memory_objects();
            let allocation = unsafe { allocator.allocate(description) }.unwrap();
            let after = fake.memory_objects();
            let created: Vec<_> = after
                .iter()
                .filter(|&(memory, _)| !before.contains_key(memory))
                .collect();
            assert!(
                created.len() <= 1,
                "seed {seed} step {step}: one allocation created several blocks"
            );
            if let Some(&(&memory, object)) = created.first() {
                assert_eq!(memory, allocation.memory(), "seed {seed} step {step}");
                assert_eq!(
                    object.size,
                    expected_shared_block_size(&properties, memory_type_index, block_size, size),
                    "seed {seed} step {step}: a new shared block must be min(block_size, heap size) \
                     bytes, or the allocation's size if that is larger (heap count {}, heap index {}, \
                     block size {block_size}, allocation size {size})",
                    properties.memory_heap_count,
                    properties.memory_types[memory_type_index as usize].heap_index,
                );
            }
            live.push(allocation);
        }
        for allocation in live {
            unsafe { allocator.free(allocation) }.unwrap();
        }
        assert_eq!(fake.misuse(), Vec::<String>::new(), "seed {seed}");
        assert!(fake.memory_objects().is_empty(), "seed {seed}");
    }
}
