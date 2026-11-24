mod fake_device;

use fake_device::{FakeDevice, desc};
use proptest::prelude::*;
use proptest::sample::select;
use vulkan::vk;
use vulkan_alloc::{AllocationError, AllocationScheme, Allocator, AllocatorCreateDesc};

#[derive(Debug, Clone, Copy)]
enum Kind {
    Shared,
    DedicatedBuffer,
    DedicatedImage,
    Exportable,
    ZeroInitialized,
}

const KINDS: [Kind; 5] = [
    Kind::Shared,
    Kind::DedicatedBuffer,
    Kind::DedicatedImage,
    Kind::Exportable,
    Kind::ZeroInitialized,
];

fn heap_count() -> impl Strategy<Value = u32> {
    prop_oneof![
        0..=vk::MAX_MEMORY_HEAPS as u32 + 4,
        select(vec![u32::MAX - 1, u32::MAX]),
    ]
}

fn heap_index() -> impl Strategy<Value = u32> {
    prop_oneof![
        4 => 0..=vk::MAX_MEMORY_HEAPS as u32 + 4,
        1 => select(vec![u32::MAX - 1, u32::MAX]),
    ]
}

fn heap_size() -> impl Strategy<Value = u64> {
    prop_oneof![1 => Just(0u64), 6 => 1..=8192u64]
}

fn memory_properties(
    heap_count: u32,
    heap_index: u32,
    heap_size: u64,
) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 1,
        memory_heap_count: heap_count,
        ..Default::default()
    };
    properties.memory_types[0].property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
    properties.memory_types[0].heap_index = heap_index;
    for heap in &mut properties.memory_heaps {
        heap.size = heap_size;
    }
    properties
}

fn known_heap_size(heap_count: u32, heap_index: u32, heap_size: u64) -> Option<u64> {
    let inside = heap_index < heap_count && heap_index < vk::MAX_MEMORY_HEAPS as u32;
    (inside && heap_size != 0).then_some(heap_size)
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn an_allocation_larger_than_a_known_heap_is_refused_without_a_vulkan_call(
        heap_count in heap_count(),
        heap_index in heap_index(),
        heap_size in heap_size(),
        block_size in 0..=8192u64,
        size in 1..=16384u64,
        kind in select(KINDS.to_vec()),
        earlier_allocation in any::<bool>(),
    ) {
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: memory_properties(heap_count, heap_index, heap_size),
            non_coherent_atom_size: 1,
            block_size,
        });
        let earlier = earlier_allocation
            .then(|| unsafe { allocator.allocate(desc(1, 1, 0)) })
            .transpose();
        prop_assert!(
            earlier.is_ok(),
            "an allocation of 1 byte must succeed in any heap: {:?}", earlier
        );
        let earlier = earlier.unwrap();

        let mut description = desc(size, 1, 0);
        match kind {
            Kind::Shared => {}
            Kind::DedicatedBuffer => {
                description.allocation_scheme = AllocationScheme::DedicatedBuffer(vk::Buffer(1));
            }
            Kind::DedicatedImage => {
                description.allocation_scheme = AllocationScheme::DedicatedImage(vk::Image(1));
            }
            Kind::Exportable => {
                description.export_handle_types = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
            }
            Kind::ZeroInitialized => {
                description.memory_allocate_flags = vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
            }
        }

        let calls_before = fake.calls();
        let snapshot_before = allocator.snapshot();
        let result = unsafe { allocator.allocate(description) };
        let known = known_heap_size(heap_count, heap_index, heap_size);
        let context = format!(
            "heap count {heap_count}, heap index {heap_index}, heap size {heap_size}, \
             block size {block_size}, size {size}, {kind:?}, earlier allocation {earlier_allocation}"
        );

        if known.is_some_and(|heap| size > heap) {
            prop_assert_eq!(
                &result,
                &Err(AllocationError::OutOfMemory),
                "{}: an allocation larger than the heap must be refused", context
            );
            prop_assert_eq!(
                fake.calls().allocate,
                calls_before.allocate,
                "{}: a refused allocation must not call vkAllocateMemory", context
            );
            prop_assert_eq!(
                allocator.snapshot(),
                snapshot_before,
                "{}: a refused allocation must leave the allocator as it was", context
            );
        } else {
            prop_assert!(
                result.is_ok(),
                "{}: an allocation that is not larger than a known heap must succeed: {:?}",
                context,
                result
            );
            let allocation = result.unwrap();
            prop_assert!(allocation.size() >= size, "{}", context);
            if let Some(heap) = known {
                for object in fake.memory_objects().values() {
                    prop_assert!(
                        object.size <= heap,
                        "{}: a VkDeviceMemory of {} bytes is larger than the heap",
                        context,
                        object.size
                    );
                }
            }
            unsafe { allocator.free(allocation) }.unwrap();
        }

        if let Some(allocation) = earlier {
            unsafe { allocator.free(allocation) }.unwrap();
        }
        prop_assert!(allocator.is_empty(), "{}", context);
        prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "{}", context);
        prop_assert!(fake.memory_objects().is_empty(), "{}", context);
    }
}
