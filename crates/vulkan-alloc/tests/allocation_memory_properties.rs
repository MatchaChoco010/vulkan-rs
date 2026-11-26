mod fake_device;

use fake_device::{BLOCK_SIZE, FakeDevice};
use proptest::prelude::*;
use proptest::sample::select;
use proptest::test_runner::Config;
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;

const PROPERTY_BITS: [vk::MemoryPropertyFlags; 7] = [
    LOCAL,
    VISIBLE,
    COHERENT,
    CACHED,
    vk::MemoryPropertyFlags::LAZILY_ALLOCATED,
    vk::MemoryPropertyFlags::PROTECTED,
    vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl Kind {
    fn has_memory_of_its_own(self) -> bool {
        self != Kind::Shared
    }
}

#[derive(Debug, Clone, Copy)]
struct Request {
    size: u64,
    memory_type_bits: u32,
    location: MemoryLocation,
    linear: bool,
    device_address: bool,
    kind: Kind,
    freed_at_once: bool,
    out_of_memory_first: usize,
}

fn property_flags(mask: u8) -> vk::MemoryPropertyFlags {
    PROPERTY_BITS
        .iter()
        .enumerate()
        .filter(|&(bit, _)| mask >> bit & 1 == 1)
        .fold(vk::MemoryPropertyFlags::empty(), |all, (_, &flag)| {
            all | flag
        })
}

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

fn create_desc(request: &Request, resource: u64) -> AllocationCreateDesc<'static> {
    let mut memory_allocate_flags = vk::MemoryAllocateFlags::empty();
    if request.device_address {
        memory_allocate_flags |= vk::MemoryAllocateFlags::DEVICE_ADDRESS;
    }
    if request.kind == Kind::ZeroInitialized {
        memory_allocate_flags |= vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
    }
    AllocationCreateDesc {
        name: None,
        mapped: false,
        requirements: vk::MemoryRequirements {
            size: request.size,
            alignment: 16,
            memory_type_bits: request.memory_type_bits,
        },
        location: request.location,
        linear: request.linear,
        memory_allocate_flags,
        export_handle_types: if request.kind == Kind::Exportable {
            vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT
        } else {
            vk::ExternalMemoryHandleTypeFlags::empty()
        },
        allocation_scheme: match request.kind {
            Kind::DedicatedBuffer => AllocationScheme::DedicatedBuffer(vk::Buffer(resource)),
            Kind::DedicatedImage => AllocationScheme::DedicatedImage(vk::Image(resource)),
            _ => AllocationScheme::GpuAllocatorManaged,
        },
    }
}

fn check_allocation(
    properties: &vk::PhysicalDeviceMemoryProperties,
    request: &Request,
    allocation: &Allocation,
) {
    let index = allocation.memory_type_index() as usize;
    assert!(
        index < properties.memory_type_count as usize,
        "{request:?}: memory type {index} is not one of the {} types",
        properties.memory_type_count
    );
    assert_eq!(
        allocation.memory_properties(),
        properties.memory_types[index].property_flags,
        "{request:?}: memory_properties() must be the flags of memory type {index}"
    );
    assert_eq!(
        allocation.is_dedicated(),
        request.kind.has_memory_of_its_own(),
        "{request:?}: is_dedicated() must be true exactly for dedicated, exportable and \
         zero-initialized allocations"
    );
}

fn allocate_all(types: &[vk::MemoryPropertyFlags], requests: &[Request]) -> usize {
    let fake = FakeDevice::new();
    let properties = memory_properties(types);
    let mut allocator = Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties: properties,
        non_coherent_atom_size: 64,
        block_size: BLOCK_SIZE,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
    });
    let mut live = Vec::new();
    let mut succeeded = 0;
    for (resource, request) in (1..).zip(requests) {
        fake.script_allocations(std::iter::repeat_n(
            vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            request.out_of_memory_first,
        ));
        let Ok(allocation) = (unsafe { allocator.allocate(create_desc(request, resource)) }) else {
            continue;
        };
        succeeded += 1;
        check_allocation(&properties, request, &allocation);
        if request.freed_at_once {
            unsafe { allocator.free(allocation) }.unwrap();
        } else {
            live.push((request, allocation));
        }
    }
    for (request, allocation) in live {
        check_allocation(&properties, request, &allocation);
        unsafe { allocator.free(allocation) }.unwrap();
    }
    assert!(allocator.is_empty());
    succeeded
}

fn location() -> impl Strategy<Value = MemoryLocation> {
    prop_oneof![
        Just(MemoryLocation::GpuOnly),
        Just(MemoryLocation::CpuToGpu),
        Just(MemoryLocation::GpuToCpu),
        (any::<u8>(), any::<u8>()).prop_map(|(required, preferred)| MemoryLocation::custom(
            property_flags(required & 0b1111),
            property_flags(preferred)
        )),
    ]
}

fn request() -> impl Strategy<Value = Request> {
    (
        1..=3 * BLOCK_SIZE,
        prop_oneof![Just(u32::MAX), any::<u32>()],
        location(),
        any::<bool>(),
        any::<bool>(),
        select(KINDS.to_vec()),
        any::<bool>(),
        prop_oneof![3 => Just(0usize), 2 => 1..=3usize],
    )
        .prop_map(
            |(
                size,
                memory_type_bits,
                location,
                linear,
                device_address,
                kind,
                freed_at_once,
                out_of_memory_first,
            )| {
                Request {
                    size,
                    memory_type_bits,
                    location,
                    linear,
                    device_address,
                    kind,
                    freed_at_once,
                    out_of_memory_first,
                }
            },
        )
}

#[test]
fn every_kind_of_allocation_reports_its_memory_type_flags_and_whether_it_has_its_own_memory() {
    let types = [LOCAL, VISIBLE | COHERENT, VISIBLE | CACHED];
    let mut requests = Vec::new();
    for kind in KINDS {
        for location in [
            MemoryLocation::GpuOnly,
            MemoryLocation::CpuToGpu,
            MemoryLocation::GpuToCpu,
            MemoryLocation::custom(VISIBLE, VISIBLE | CACHED),
        ] {
            for size in [256, BLOCK_SIZE, 2 * BLOCK_SIZE] {
                requests.push(Request {
                    size,
                    memory_type_bits: u32::MAX,
                    location,
                    linear: true,
                    device_address: false,
                    kind,
                    freed_at_once: false,
                    out_of_memory_first: 0,
                });
            }
        }
    }
    assert_eq!(
        allocate_all(&types, &requests),
        requests.len(),
        "every request has a memory type"
    );
}

#[test]
fn an_allocation_that_falls_back_reports_the_flags_of_the_type_it_was_made_in() {
    let types = [VISIBLE | COHERENT | CACHED, VISIBLE | COHERENT];
    let fake = FakeDevice::new();
    let mut allocator = Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties: memory_properties(&types),
        non_coherent_atom_size: 64,
        block_size: BLOCK_SIZE,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
    });
    fake.script_allocations([vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY; 5]);
    let request = Request {
        size: 256,
        memory_type_bits: u32::MAX,
        location: MemoryLocation::GpuToCpu,
        linear: true,
        device_address: false,
        kind: Kind::Shared,
        freed_at_once: false,
        out_of_memory_first: 5,
    };
    let allocation = unsafe { allocator.allocate(create_desc(&request, 1)) }.unwrap();
    assert_eq!(
        allocation.memory_type_index(),
        1,
        "the cached type is out of memory"
    );
    assert_eq!(
        allocation.memory_properties(),
        VISIBLE | COHERENT,
        "the flags of the type the allocation was made in, not of the type chosen first"
    );
    unsafe { allocator.free(allocation) }.unwrap();
}

#[test]
fn an_allocation_larger_than_a_shared_block_is_not_dedicated() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let mut desc = fake_device::desc(2 * BLOCK_SIZE, 1, fake_device::DEVICE_LOCAL);
    desc.location = MemoryLocation::GpuOnly;
    let allocation = unsafe { allocator.allocate(desc) }.unwrap();
    assert_eq!(allocation.size(), 2 * BLOCK_SIZE);
    assert!(
        !allocation.is_dedicated(),
        "a block of the size of a large shared allocation is not memory of its own"
    );
    assert_eq!(allocation.memory_properties(), LOCAL);
    unsafe { allocator.free(allocation) }.unwrap();
}

proptest! {
    #![proptest_config(Config {
        cases: 512,
        failure_persistence: None,
        ..Config::default()
    })]

    #[test]
    fn allocations_report_the_flags_of_their_memory_type_and_whether_they_have_their_own_memory(
        types in prop::collection::vec(any::<u8>().prop_map(property_flags), 1..=8),
        requests in prop::collection::vec(request(), 1..=16),
    ) {
        allocate_all(&types, &requests);
    }
}
