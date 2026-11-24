mod fake_device;

use fake_device::FakeDevice;
use proptest::prelude::*;
use proptest::sample::select;
use vulkan::vk;
use vulkan_alloc::{
    AllocationCreateDesc, AllocationError, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

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

const ALLOCATE_MEMORY_RESULTS: [vk::VkResult; 6] = [
    vk::VkResult::SUCCESS,
    vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
    vk::VkResult::ERROR_TOO_MANY_OBJECTS,
    vk::VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
    vk::VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
];

#[derive(Debug, Clone)]
struct Device {
    heap_indices: Vec<u32>,
    heap_count: u32,
    heap_sizes: [u64; 3],
}

impl Device {
    fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: self.heap_indices.len() as u32,
            memory_heap_count: self.heap_count,
            ..Default::default()
        };
        for (memory_type, &heap_index) in properties.memory_types.iter_mut().zip(&self.heap_indices)
        {
            memory_type.property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
            memory_type.heap_index = heap_index;
        }
        for (heap, &size) in properties.memory_heaps.iter_mut().zip(&self.heap_sizes) {
            heap.size = size;
        }
        properties
    }

    fn chosen_memory_type(&self, memory_type_bits: u32) -> Option<usize> {
        (0..self.heap_indices.len()).find(|&index| memory_type_bits >> index & 1 == 1)
    }

    fn known_heap_size(&self, memory_type: usize) -> Option<u64> {
        let heap_index = self.heap_indices[memory_type];
        let reported = heap_index < self.heap_count && heap_index < vk::MAX_MEMORY_HEAPS as u32;
        let size = if reported {
            self.heap_sizes
                .get(heap_index as usize)
                .copied()
                .unwrap_or(0)
        } else {
            0
        };
        (size != 0).then_some(size)
    }
}

fn devices() -> impl Strategy<Value = Device> {
    (
        prop::collection::vec(0..=4u32, 1..=3),
        0..=4u32,
        [
            prop_oneof![1 => Just(0u64), 4 => 1..=8192u64],
            prop_oneof![1 => Just(0u64), 4 => 1..=8192u64],
            prop_oneof![1 => Just(0u64), 4 => 1..=8192u64],
        ],
    )
        .prop_map(|(heap_indices, heap_count, heap_sizes)| Device {
            heap_indices,
            heap_count,
            heap_sizes,
        })
}

fn sizes() -> impl Strategy<Value = u64> {
    prop_oneof![1 => Just(0u64), 6 => 1..=16384u64]
}

fn alignments() -> impl Strategy<Value = u64> {
    prop_oneof![
        1 => Just(0u64),
        3 => (0..64u32).prop_map(|shift| 1u64 << shift),
        2 => (1..64u32, select(vec![-1i64, 1])).prop_map(|(shift, step)| {
            (1u64 << shift).wrapping_add_signed(step)
        }),
        1 => any::<u64>(),
    ]
}

fn memory_type_bits() -> impl Strategy<Value = u32> {
    prop_oneof![4 => any::<u32>(), 1 => Just(0u32), 1 => Just(0b1000u32)]
}

fn of_kind(
    mut description: AllocationCreateDesc<'static>,
    kind: Kind,
) -> AllocationCreateDesc<'static> {
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
    description
}

fn description(
    size: u64,
    alignment: u64,
    memory_type_bits: u32,
    linear: bool,
) -> AllocationCreateDesc<'static> {
    AllocationCreateDesc {
        name: None,
        requirements: vk::MemoryRequirements {
            size,
            alignment,
            memory_type_bits,
        },
        location: MemoryLocation::custom(
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::empty(),
        ),
        linear,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Expected {
    Refused(AllocationError),
    Failed(AllocationError),
    Allocated,
}

fn expected(
    device: &Device,
    size: u64,
    alignment: u64,
    memory_type_bits: u32,
    device_mask: bool,
    allocate_memory_result: vk::VkResult,
) -> Expected {
    if size == 0 || (alignment != 0 && !alignment.is_power_of_two()) || device_mask {
        return Expected::Refused(AllocationError::InvalidAllocationCreateDesc);
    }
    let Some(memory_type) = device.chosen_memory_type(memory_type_bits) else {
        return Expected::Refused(AllocationError::NoCompatibleMemoryType);
    };
    if device
        .known_heap_size(memory_type)
        .is_some_and(|heap_size| size > heap_size)
    {
        return Expected::Refused(AllocationError::OutOfMemory);
    }
    match allocate_memory_result {
        vk::VkResult::SUCCESS => Expected::Allocated,
        vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY => Expected::Failed(AllocationError::OutOfMemory),
        result => Expected::Failed(AllocationError::Vulkan(result)),
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn allocate_reports_an_invalid_description_then_no_memory_type_then_a_full_heap_then_the_device(
        device in devices(),
        size in sizes(),
        alignment in alignments(),
        memory_type_bits in memory_type_bits(),
        kind in select(KINDS.to_vec()),
        device_mask in prop::bool::weighted(0.1),
        allocate_memory_result in select(ALLOCATE_MEMORY_RESULTS.to_vec()),
        earlier_allocation in any::<bool>(),
    ) {
        let fake = FakeDevice::new();
        let mut allocator = Allocator::new(AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: device.memory_properties(),
            non_coherent_atom_size: 1,
            block_size: 4096,
        });
        let earlier = earlier_allocation
            .then(|| unsafe { allocator.allocate(description(1, 1, u32::MAX, false)) })
            .transpose();
        prop_assert!(
            earlier.is_ok(),
            "an allocation of 1 byte must succeed on any of these devices: {:?}", earlier
        );
        let earlier = earlier.unwrap();
        let context = format!(
            "{device:?}, size {size}, alignment {alignment}, memory type bits \
             {memory_type_bits:#b}, {kind:?}, DEVICE_MASK {device_mask}, vkAllocateMemory \
             returns {allocate_memory_result:?}, earlier allocation {earlier_allocation}"
        );
        let mut request = of_kind(description(size, alignment, memory_type_bits, true), kind);
        if device_mask {
            request.memory_allocate_flags |= vk::MemoryAllocateFlags::DEVICE_MASK;
        }

        if allocate_memory_result != vk::VkResult::SUCCESS {
            fake.fail_next_allocation(allocate_memory_result);
        }
        let calls_before = fake.calls();
        let snapshot_before = allocator.snapshot();
        let memory_before = fake.memory_objects();
        let result = unsafe { allocator.allocate(request) };
        let expected = expected(
            &device,
            size,
            alignment,
            memory_type_bits,
            device_mask,
            allocate_memory_result,
        );
        let fails = expected != Expected::Allocated;

        match expected {
            Expected::Refused(error) => {
                prop_assert_eq!(&result, &Err(error), "{}: the error of the refusal", context);
                prop_assert_eq!(
                    fake.calls(),
                    calls_before,
                    "{}: a refused allocation must not call the device", context
                );
            }
            Expected::Failed(error) => {
                prop_assert_eq!(
                    &result,
                    &Err(error),
                    "{}: the error when vkAllocateMemory fails", context
                );
                prop_assert_eq!(
                    fake.calls().allocate,
                    calls_before.allocate + 1,
                    "{}: a request for new memory must call vkAllocateMemory once", context
                );
                prop_assert_eq!(
                    fake.calls().free,
                    calls_before.free,
                    "{}: a failed vkAllocateMemory must not be followed by vkFreeMemory", context
                );
            }
            Expected::Allocated => {
                prop_assert!(result.is_ok(), "{}: the allocation must succeed: {:?}", context, result);
                let allocation = result.unwrap();
                prop_assert!(allocation.size() >= size, "{}", context);
                unsafe { allocator.free(allocation) }.unwrap();
            }
        }
        if fails {
            prop_assert_eq!(
                allocator.snapshot(),
                snapshot_before,
                "{}: a failed allocation must leave the allocator as it was", context
            );
            prop_assert_eq!(
                fake.memory_objects(),
                memory_before,
                "{}: a failed allocation must leave the memory of the device as it was", context
            );
        }

        if let Some(allocation) = earlier {
            unsafe { allocator.free(allocation) }.unwrap();
        }
        prop_assert!(allocator.is_empty(), "{}", context);
        prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "{}", context);
        prop_assert!(fake.memory_objects().is_empty(), "{}", context);
    }
}
