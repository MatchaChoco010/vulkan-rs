mod fake_device;

use fake_device::FakeDevice;
use proptest::prelude::*;
use proptest::test_runner::Config;
use vulkan::vk;
use vulkan_alloc::{
    AllocationCreateDesc, AllocationError, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;
const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

const KINDS: [vk::MemoryPropertyFlags; 8] = [
    LOCAL,
    VISIBLE,
    VISIBLE.union(CACHED),
    VISIBLE.union(COHERENT),
    VISIBLE.union(COHERENT).union(CACHED),
    LOCAL.union(VISIBLE).union(COHERENT),
    LOCAL.union(VISIBLE).union(CACHED),
    LOCAL.union(VISIBLE).union(COHERENT).union(CACHED),
];

fn memory_properties(types: &[vk::MemoryPropertyFlags]) -> vk::PhysicalDeviceMemoryProperties {
    let mut properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: types.len() as u32,
        memory_heap_count: 1,
        ..Default::default()
    };
    properties.memory_heaps[0].size = 1 << 20;
    for (memory_type, &flags) in properties.memory_types.iter_mut().zip(types) {
        memory_type.property_flags = flags;
    }
    properties
}

fn documented_choice(
    types: &[vk::MemoryPropertyFlags],
    memory_type_bits: u32,
    location: MemoryLocation,
) -> Option<u32> {
    let coherent = (0..types.len())
        .filter(|&index| memory_type_bits & (1 << index) != 0)
        .filter(|&index| types[index].contains(VISIBLE | COHERENT))
        .collect::<Vec<_>>();
    let chosen = match location {
        MemoryLocation::GpuToCpu => coherent
            .iter()
            .copied()
            .find(|&index| types[index].contains(CACHED))
            .or(coherent.first().copied()),
        _ => coherent.first().copied(),
    };
    chosen.map(|index| index as u32)
}

fn allocated_type(
    types: &[vk::MemoryPropertyFlags],
    memory_type_bits: u32,
    location: MemoryLocation,
) -> Result<(u32, vk::MemoryPropertyFlags), AllocationError> {
    let fake = FakeDevice::new();
    let mut allocator = Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties: memory_properties(types),
        non_coherent_atom_size: 64,
        block_size: 4096,
    });
    unsafe {
        let allocation = allocator.allocate(AllocationCreateDesc {
            name: None,
            requirements: vk::MemoryRequirements {
                size: 256,
                alignment: 1,
                memory_type_bits,
            },
            location,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;
        let chosen = (
            allocation.memory_type_index(),
            allocator.allocation_memory_flags(&allocation)?,
        );
        allocator.free(allocation).unwrap();
        Ok(chosen)
    }
}

fn check_host_location(
    types: &[vk::MemoryPropertyFlags],
    memory_type_bits: u32,
    location: MemoryLocation,
) {
    let context = format!("types {types:?}, allowed {memory_type_bits:#b}, {location:?}");
    match (
        allocated_type(types, memory_type_bits, location),
        documented_choice(types, memory_type_bits, location),
    ) {
        (Ok((index, flags)), Some(expected)) => {
            assert!(
                flags.contains(VISIBLE | COHERENT),
                "{context}: chose type {index} with {flags:?}, which is not host-visible and \
                 host-coherent"
            );
            assert_eq!(
                index, expected,
                "{context}: chose type {index} instead of the lowest-indexed allowed coherent \
                 type, cached first for GpuToCpu"
            );
        }
        (Err(error), None) => assert_eq!(
            error,
            AllocationError::NoCompatibleMemoryType,
            "{context}: no allowed type is host-visible and host-coherent"
        ),
        (Ok((index, flags)), None) => panic!(
            "{context}: chose type {index} with {flags:?} although no allowed type is \
             host-visible and host-coherent"
        ),
        (Err(error), Some(expected)) => {
            panic!("{context}: failed with {error:?} although type {expected} is allowed")
        }
    }
}

#[test]
fn gpu_to_cpu_takes_coherent_memory_over_an_earlier_cached_type_that_is_not_coherent() {
    let types = [VISIBLE | COHERENT, VISIBLE | CACHED];
    assert_eq!(
        allocated_type(&types, 0b11, MemoryLocation::GpuToCpu),
        Ok((0, VISIBLE | COHERENT))
    );
    check_host_location(&types, 0b11, MemoryLocation::GpuToCpu);
    check_host_location(&types, 0b11, MemoryLocation::CpuToGpu);
}

#[test]
fn gpu_to_cpu_takes_coherent_cached_memory_over_an_earlier_cached_type_that_is_not_coherent() {
    let types = [VISIBLE | CACHED, VISIBLE | COHERENT | CACHED];
    assert_eq!(
        allocated_type(&types, 0b11, MemoryLocation::GpuToCpu),
        Ok((1, VISIBLE | COHERENT | CACHED))
    );
    check_host_location(&types, 0b11, MemoryLocation::GpuToCpu);
    check_host_location(&types, 0b11, MemoryLocation::CpuToGpu);
}

#[test]
fn host_locations_fail_when_no_allowed_type_is_coherent() {
    let types = [VISIBLE | CACHED, VISIBLE | COHERENT];
    for location in [MemoryLocation::CpuToGpu, MemoryLocation::GpuToCpu] {
        assert_eq!(
            allocated_type(&types, 0b01, location),
            Err(AllocationError::NoCompatibleMemoryType),
            "{location:?} with only a non-coherent type allowed"
        );
    }
}

#[test]
fn a_custom_location_still_takes_cached_memory_that_is_not_coherent() {
    let types = [VISIBLE | COHERENT, VISIBLE | CACHED];
    assert_eq!(
        allocated_type(
            &types,
            0b11,
            MemoryLocation::custom(VISIBLE, VISIBLE | CACHED)
        ),
        Ok((1, VISIBLE | CACHED))
    );
}

proptest! {
    #![proptest_config(Config {
        cases: 1024,
        failure_persistence: None,
        ..Config::default()
    })]

    #[test]
    fn host_locations_choose_the_documented_coherent_memory_type(
        types in prop::collection::vec(prop::sample::select(KINDS.to_vec()), 1..=6),
        memory_type_bits in any::<u32>(),
    ) {
        for location in [MemoryLocation::CpuToGpu, MemoryLocation::GpuToCpu] {
            check_host_location(&types, memory_type_bits, location);
        }
    }
}
