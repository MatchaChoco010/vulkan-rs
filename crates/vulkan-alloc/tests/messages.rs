mod fake_device;

use std::collections::BTreeSet;
use std::error::Error;

use fake_device::{FakeDevice, HOST_COHERENT, desc};
use vulkan::vk;
use vulkan_alloc::descriptor_heap::HeapLayoutError;
use vulkan_alloc::{AllocationCreateDesc, AllocationError, AllocatorCreateDesc, FreeError};

#[test]
fn allocation_errors_have_distinct_messages_that_name_the_vulkan_result() {
    let errors = [
        AllocationError::NoCompatibleMemoryType,
        AllocationError::OutOfMemory,
        AllocationError::InvalidAllocation,
        AllocationError::Vulkan(vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY),
        AllocationError::Vulkan(vk::VkResult::ERROR_MEMORY_MAP_FAILED),
        AllocationError::InvalidAllocationCreateDesc,
        AllocationError::NotHostVisible,
        AllocationError::NotMapped,
        AllocationError::StillMapped,
        AllocationError::MappingFailed,
        AllocationError::RangeOutOfBounds,
    ];
    let messages = errors.iter().map(ToString::to_string).collect::<Vec<_>>();
    assert_eq!(
        messages.iter().collect::<BTreeSet<_>>().len(),
        errors.len(),
        "{messages:?}"
    );
    assert!(
        messages.iter().all(|message| message.len() > 10),
        "{messages:?}"
    );
    assert!(
        messages[3].contains("ERROR_OUT_OF_DEVICE_MEMORY"),
        "{}",
        messages[3]
    );
    assert!(
        messages[4].contains("ERROR_MEMORY_MAP_FAILED"),
        "{}",
        messages[4]
    );
}

#[test]
fn a_free_error_says_what_happened_and_why() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let error = unsafe {
        let allocation = allocator.allocate(desc(64, 1, HOST_COHERENT)).unwrap();
        allocator.map_memory(&allocation).unwrap();
        allocator.free(allocation).unwrap_err()
    };
    assert!(error.to_string().contains("not freed"), "{error}");
    let source = error.source().expect("the reason is the source");
    assert_eq!(source.to_string(), AllocationError::StillMapped.to_string());
    assert_eq!(error.error, AllocationError::StillMapped);

    let FreeError { allocation, error } = error;
    assert_eq!(
        AllocationError::from(FreeError {
            allocation,
            error: error.clone()
        }),
        error
    );
    unsafe { allocator.free_all() };
}

#[test]
fn heap_layout_errors_name_every_number_they_carry() {
    let errors = [
        (
            HeapLayoutError::TooLarge {
                size: 70_001,
                max: 60_002,
            },
            vec!["70001", "60002"],
        ),
        (
            HeapLayoutError::Misaligned {
                address: 0xabc_d123,
                alignment: 4096,
            },
            vec!["0xabcd123", "4096"],
        ),
        (HeapLayoutError::Overflow, vec![]),
        (
            HeapLayoutError::UnalignedDescriptorSize {
                size: 24,
                alignment: 16,
            },
            vec!["24", "16"],
        ),
        (HeapLayoutError::ZeroDescriptorSize, vec![]),
        (HeapLayoutError::ZeroDescriptorCount, vec![]),
        (
            HeapLayoutError::ReservedRangeOutsideHeap {
                reserved_offset: 81_003,
                reserved_size: 92_004,
                heap_size: 73_005,
            },
            vec!["81003", "92004", "73005"],
        ),
        (HeapLayoutError::NullAddress, vec![]),
        (
            HeapLayoutError::AddressRangeOverflow {
                address: 0xffff_0000_1234,
                size: 55_006,
            },
            vec!["0xffff00001234", "55006"],
        ),
    ];
    let messages = errors
        .iter()
        .map(|(error, _)| error.to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        messages.iter().collect::<BTreeSet<_>>().len(),
        errors.len(),
        "{messages:?}"
    );
    for ((_, numbers), message) in errors.iter().zip(&messages) {
        assert!(message.len() > 10, "{message}");
        for number in numbers {
            assert!(
                message.contains(number),
                "`{message}` does not name {number}"
            );
        }
    }
}

#[test]
fn the_debug_output_of_a_request_names_what_was_asked_for() {
    let request = AllocationCreateDesc {
        name: Some("shadow atlas"),
        ..desc(70_001, 4096, 5)
    };
    let output = format!("{request:?}");
    for part in ["shadow atlas", "70001", "4096", "32", "GpuAllocatorManaged"] {
        assert!(output.contains(part), "`{output}` does not name {part}");
    }
}

#[test]
fn a_new_description_has_64_mib_blocks_and_the_atom_size_of_the_device() {
    let fake = FakeDevice::new();
    let mut properties = vk::PhysicalDeviceProperties::default();
    properties.limits.non_coherent_atom_size = 192;
    let description =
        AllocatorCreateDesc::new(fake.device(), FakeDevice::memory_properties(), properties);
    assert_eq!(description.block_size, 64 * 1024 * 1024);
    assert_eq!(description.non_coherent_atom_size, 192);
    assert_eq!(
        description.memory_properties.memory_type_count,
        FakeDevice::memory_properties().memory_type_count
    );
}
