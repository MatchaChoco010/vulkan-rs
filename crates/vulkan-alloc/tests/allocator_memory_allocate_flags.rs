mod fake_device;

use fake_device::{
    BLOCK_SIZE, DEVICE_LOCAL, FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, MapFault,
    NON_COHERENT_ATOM_SIZE, desc,
};
use proptest::prelude::*;
use proptest::sample::select;
use proptest::test_runner::Config;
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationScheme, Allocator,
    AllocatorCreateDesc,
};

const FLAGS: [vk::MemoryAllocateFlags; 4] = [
    vk::MemoryAllocateFlags::DEVICE_ADDRESS,
    vk::MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY,
    vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT,
    vk::MemoryAllocateFlags::DEVICE_MASK,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Shared,
    DedicatedBuffer,
    DedicatedImage,
    Exportable,
}

const KINDS: [Kind; 4] = [
    Kind::Shared,
    Kind::DedicatedBuffer,
    Kind::DedicatedImage,
    Kind::Exportable,
];

#[derive(Debug, Clone, Copy)]
struct Request {
    flags: u8,
    kind: Kind,
    size: u64,
    memory_type: u32,
    linear: bool,
}

struct Placed {
    allocation: Allocation,
    shared: bool,
    union: vk::MemoryAllocateFlags,
    linear: bool,
}

fn flags_of(mask: u8) -> vk::MemoryAllocateFlags {
    FLAGS
        .iter()
        .enumerate()
        .filter(|&(bit, _)| mask >> bit & 1 == 1)
        .fold(vk::MemoryAllocateFlags::empty(), |all, (_, &flag)| {
            all | flag
        })
}

fn allocator(fake: &FakeDevice, flags: vk::MemoryAllocateFlags) -> Allocator {
    Allocator::new(AllocatorCreateDesc {
        memory_allocate_flags: flags,
        ..AllocatorCreateDesc::new(
            fake.device(),
            FakeDevice::memory_properties(),
            vk::PhysicalDeviceProperties {
                limits: vk::PhysicalDeviceLimits {
                    non_coherent_atom_size: NON_COHERENT_ATOM_SIZE,
                    ..Default::default()
                },
                ..Default::default()
            },
        )
    })
}

fn description(request: &Request, resource: u64) -> AllocationCreateDesc<'static> {
    let mut description = AllocationCreateDesc {
        linear: request.linear,
        memory_allocate_flags: flags_of(request.flags),
        ..desc(request.size, 1, request.memory_type)
    };
    match request.kind {
        Kind::Shared => {}
        Kind::DedicatedBuffer => {
            description.allocation_scheme = AllocationScheme::DedicatedBuffer(vk::Buffer(resource));
        }
        Kind::DedicatedImage => {
            description.allocation_scheme = AllocationScheme::DedicatedImage(vk::Image(resource));
        }
        Kind::Exportable => {
            description.export_handle_types = vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT;
        }
    }
    description
}

fn check_sequence(allocator_mask: u8, requests: &[Request]) {
    assert!(
        requests.iter().map(|request| request.size).sum::<u64>()
            + requests.len() as u64 * NON_COHERENT_ATOM_SIZE
            <= BLOCK_SIZE,
        "the requests of one test case must fit in one block"
    );
    let allocator_flags = flags_of(allocator_mask);
    let fake = FakeDevice::new();
    let mut allocator = allocator(&fake, allocator_flags);
    let mut placed: Vec<Placed> = Vec::new();
    for (resource, request) in (1..).zip(requests) {
        let union = flags_of(request.flags) | allocator_flags;
        let context = format!(
            "allocator flags {allocator_flags:?}, {request:?}, flags of the allocation {union:?}"
        );
        let shared = request.kind == Kind::Shared
            && !union.contains(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
        let calls_before = fake.calls().allocate;
        let result = unsafe { allocator.allocate(description(request, resource)) };
        if union.contains(vk::MemoryAllocateFlags::DEVICE_MASK) {
            assert_eq!(
                result,
                Err(AllocationError::InvalidAllocationCreateDesc),
                "{context}: DEVICE_MASK in the union of the flags must be refused"
            );
            assert_eq!(
                fake.calls().allocate,
                calls_before,
                "{context}: a refused request must not allocate memory"
            );
            continue;
        }
        let allocation = result.unwrap_or_else(|error| panic!("{context}: {error:?}"));
        let sharing = placed
            .iter()
            .find(|earlier| {
                shared
                    && earlier.shared
                    && earlier.union == union
                    && earlier.linear == request.linear
                    && earlier.allocation.memory_type_index() == request.memory_type
            })
            .map(|earlier| earlier.allocation.memory());
        match sharing {
            Some(memory) => {
                assert_eq!(
                    allocation.memory(),
                    memory,
                    "{context}: the request must share the block of an earlier allocation with \
                     the same union of flags"
                );
                assert_eq!(fake.calls().allocate, calls_before, "{context}");
            }
            None => {
                assert!(
                    placed
                        .iter()
                        .all(|earlier| earlier.allocation.memory() != allocation.memory()),
                    "{context}: the request must not share a block with an allocation whose \
                     union of flags, kind or memory type differs"
                );
                assert_eq!(fake.calls().allocate, calls_before + 1, "{context}");
                assert_eq!(
                    fake.chains()[&allocation.memory()].flags,
                    (!union.is_empty()).then_some((union, 0)),
                    "{context}: VkMemoryAllocateFlagsInfo must carry the union of the flags, and \
                     be left out when it is empty"
                );
            }
        }
        assert_eq!(
            allocation.is_dedicated(),
            !shared,
            "{context}: the allocation has memory of its own exactly when it is dedicated, \
             exportable or zero-initialized by the union of the flags"
        );
        let block = allocator
            .snapshot()
            .memory_types
            .into_iter()
            .flat_map(|memory_type| memory_type.blocks)
            .find(|block| block.memory == allocation.memory())
            .unwrap_or_else(|| panic!("{context}: the snapshot has no block for the allocation"));
        assert_eq!(
            block.memory_allocate_flags, union,
            "{context}: the snapshot must report the union of the flags of the block"
        );
        placed.push(Placed {
            allocation,
            shared,
            union,
            linear: request.linear,
        });
    }
    for earlier in placed {
        unsafe { allocator.free(earlier.allocation) }.unwrap();
    }
    drop(allocator);
    assert!(fake.memory_objects().is_empty());
    assert_eq!(fake.misuse(), Vec::<String>::new());
}

fn requests() -> impl Strategy<Value = Request> {
    (
        prop_oneof![
            3 => Just(0u8),
            1 => Just(1u8),
            1 => 0u8..16,
        ],
        prop_oneof![
            6 => Just(Kind::Shared),
            1 => select(KINDS.to_vec()),
        ],
        1..=200u64,
        select(vec![DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT]),
        prop::bool::weighted(0.8),
    )
        .prop_map(|(flags, kind, size, memory_type, linear)| Request {
            flags,
            kind,
            size,
            memory_type,
            linear,
        })
}

#[test]
fn device_address_of_the_allocator_lets_requests_with_and_without_it_share_a_block() {
    let fake = FakeDevice::new();
    let mut allocator = allocator(&fake, vk::MemoryAllocateFlags::DEVICE_ADDRESS);
    unsafe {
        let plain = allocator.allocate(desc(256, 1, DEVICE_LOCAL)).unwrap();
        let with_address = allocator
            .allocate(AllocationCreateDesc {
                memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                ..desc(256, 1, DEVICE_LOCAL)
            })
            .unwrap();
        assert_eq!(
            plain.memory(),
            with_address.memory(),
            "a request without the flag and one with it have the same flags in this allocator"
        );
        assert_eq!(fake.calls().allocate, 1);
        assert_eq!(
            fake.chains()[&plain.memory()].flags,
            Some((vk::MemoryAllocateFlags::DEVICE_ADDRESS, 0)),
            "the block of a request without flags must be allocated with the flag of the allocator"
        );
        allocator.free(plain).unwrap();
        allocator.free(with_address).unwrap();
        assert_eq!(allocator.stats().block_count, 1, "the empty block is kept");
        let again = allocator
            .allocate(AllocationCreateDesc {
                memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                ..desc(256, 1, DEVICE_LOCAL)
            })
            .unwrap();
        assert_eq!(
            fake.calls().allocate,
            1,
            "a request with the flag of the allocator must reuse the empty block made without it"
        );
        allocator.free(again).unwrap();
    }
}

#[test]
fn zero_initialize_and_device_mask_of_the_allocator_apply_to_every_request() {
    let fake = FakeDevice::new();
    let mut zeroing = allocator(&fake, vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
    unsafe {
        let first = zeroing.allocate(desc(256, 1, DEVICE_LOCAL)).unwrap();
        let second = zeroing.allocate(desc(256, 1, DEVICE_LOCAL)).unwrap();
        assert_ne!(first.memory(), second.memory());
        assert!(first.is_dedicated() && second.is_dedicated());
        assert_eq!(fake.memory_objects()[&first.memory()].size, 256);
        assert_eq!(
            fake.chains()[&first.memory()].flags,
            Some((vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT, 0))
        );
        zeroing.free(first).unwrap();
        zeroing.free(second).unwrap();
        assert_eq!(zeroing.stats().block_count, 0);
    }
    let mut masked = allocator(&fake, vk::MemoryAllocateFlags::DEVICE_MASK);
    assert_eq!(
        unsafe { masked.allocate(desc(256, 1, DEVICE_LOCAL)) },
        Err(AllocationError::InvalidAllocationCreateDesc)
    );
    assert_eq!(fake.calls().allocate, 2);
}

#[test]
fn a_mapped_allocation_gets_memory_of_its_own_when_the_allocator_adds_zero_initialize() {
    let fake = FakeDevice::new();
    let mut allocator = allocator(&fake, vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
    unsafe {
        let allocation = allocator
            .allocate(AllocationCreateDesc {
                mapped: true,
                ..desc(256, 1, HOST_COHERENT)
            })
            .unwrap();
        assert!(allocation.is_dedicated());
        assert_eq!(
            fake.memory_objects()[&allocation.memory()],
            fake_device::MemoryObject {
                size: 256,
                memory_type_index: HOST_COHERENT,
                mapped: true,
            },
            "a mapped allocation of a zero-initializing allocator has mapped memory of its own"
        );
        assert_eq!(
            allocation
                .mapped_ptr()
                .map(|pointer| pointer.as_ptr() as usize),
            fake.base_address(allocation.memory())
        );
        allocator.free(allocation).unwrap();
        assert_eq!(allocator.stats().block_count, 0);
    }
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn a_new_block_of_an_allocation_whose_mapping_fails_is_kept_by_the_flags_of_the_allocation() {
    let fake = FakeDevice::new();
    let mut addressing = allocator(&fake, vk::MemoryAllocateFlags::DEVICE_ADDRESS);
    unsafe {
        fake.fault_next_map(MapFault::Fails(vk::VkResult::ERROR_MEMORY_MAP_FAILED));
        assert_eq!(
            addressing.allocate(AllocationCreateDesc {
                mapped: true,
                ..desc(256, 1, HOST_COHERENT)
            }),
            Err(AllocationError::Vulkan(
                vk::VkResult::ERROR_MEMORY_MAP_FAILED
            ))
        );
        let kept = addressing
            .snapshot()
            .memory_types
            .into_iter()
            .flat_map(|memory_type| memory_type.blocks)
            .collect::<Vec<_>>();
        assert_eq!(kept.len(), 1, "the new shared block is kept empty");
        assert_eq!(
            kept[0].size,
            8 << 20,
            "the first block of a memory type is an eighth of the block size of 64 MiB"
        );
        assert_eq!(
            kept[0].memory_allocate_flags,
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        );
        let with_address = addressing
            .allocate(AllocationCreateDesc {
                memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                ..desc(256, 1, HOST_COHERENT)
            })
            .unwrap();
        assert_eq!(
            (with_address.memory(), fake.calls().allocate),
            (kept[0].memory, 1),
            "a request that names the flag of the allocator must reuse the kept block"
        );
        addressing.free(with_address).unwrap();
    }

    let fake = FakeDevice::new();
    let mut zeroing = allocator(&fake, vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
    unsafe {
        fake.fault_next_map(MapFault::Fails(vk::VkResult::ERROR_MEMORY_MAP_FAILED));
        assert_eq!(
            zeroing.allocate(AllocationCreateDesc {
                mapped: true,
                ..desc(256, 1, HOST_COHERENT)
            }),
            Err(AllocationError::Vulkan(
                vk::VkResult::ERROR_MEMORY_MAP_FAILED
            ))
        );
    }
    assert_eq!(
        (zeroing.stats().block_count, fake.calls().free),
        (0, 1),
        "memory of its own for ZERO_INITIALIZE_EXT of the allocator is not kept"
    );
    assert!(fake.memory_objects().is_empty());
}

#[test]
fn a_new_allocator_description_adds_no_flags() {
    let fake = FakeDevice::new();
    let description = AllocatorCreateDesc::new(
        fake.device(),
        FakeDevice::memory_properties(),
        vk::PhysicalDeviceProperties::default(),
    );
    assert_eq!(
        description.memory_allocate_flags,
        vk::MemoryAllocateFlags::empty()
    );
}

proptest! {
    #![proptest_config(Config {
        cases: 512,
        failure_persistence: None,
        ..Config::default()
    })]

    #[test]
    fn the_flags_of_an_allocation_are_the_union_of_those_of_the_allocator_and_the_request(
        allocator_mask in prop_oneof![1 => Just(0u8), 2 => Just(1u8), 2 => 0u8..16],
        requests in prop::collection::vec(requests(), 1..=12),
    ) {
        check_sequence(allocator_mask, &requests);
    }
}
