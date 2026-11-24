mod fake_device;

use fake_device::{DEVICE_LOCAL, FakeDevice, HOST_COHERENT, desc};
use proptest::prelude::*;
use proptest::sample::select;
use vulkan::vk;
use vulkan_alloc::{AllocationCreateDesc, AllocationError, AllocationScheme};

#[derive(Debug, Clone, Copy)]
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

const FLAGS: [vk::MemoryAllocateFlags; 4] = [
    vk::MemoryAllocateFlags::DEVICE_MASK,
    vk::MemoryAllocateFlags::DEVICE_ADDRESS,
    vk::MemoryAllocateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY,
    vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT,
];

fn flags_of(mask: u8) -> vk::MemoryAllocateFlags {
    FLAGS
        .iter()
        .enumerate()
        .filter(|&(bit, _)| mask >> bit & 1 == 1)
        .fold(vk::MemoryAllocateFlags::empty(), |all, (_, &flag)| {
            all | flag
        })
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
    }
    description
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn only_descriptions_with_the_device_mask_flag_are_refused(
        mask in 0u8..16,
        kind in select(KINDS.to_vec()),
        memory_type in select(vec![DEVICE_LOCAL, HOST_COHERENT]),
        earlier_allocation in any::<bool>(),
        size in 1..=8192u64,
    ) {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        let earlier = earlier_allocation
            .then(|| unsafe { allocator.allocate(desc(64, 1, memory_type)) }.unwrap());
        let flags = flags_of(mask);
        let context = format!(
            "flags {flags:?}, {kind:?}, memory type {memory_type}, size {size}, \
             earlier allocation {earlier_allocation}"
        );
        let description = of_kind(
            AllocationCreateDesc {
                linear: false,
                memory_allocate_flags: flags,
                ..desc(size, 1, memory_type)
            },
            kind,
        );

        let calls_before = fake.calls();
        let snapshot_before = allocator.snapshot();
        let memory_before = fake.memory_objects();
        let result = unsafe { allocator.allocate(description) };

        if flags.contains(vk::MemoryAllocateFlags::DEVICE_MASK) {
            prop_assert_eq!(
                &result,
                &Err(AllocationError::InvalidAllocationCreateDesc),
                "{}: DEVICE_MASK needs a deviceMask the allocator cannot pass", context
            );
            prop_assert_eq!(
                fake.calls(),
                calls_before,
                "{}: a refused description must not call the device", context
            );
            prop_assert_eq!(
                allocator.snapshot(),
                snapshot_before,
                "{}: a refused description must leave the allocator as it was", context
            );
            prop_assert_eq!(
                fake.memory_objects(),
                memory_before,
                "{}: a refused description must leave the memory of the device as it was",
                context
            );
        } else {
            prop_assert!(
                result.is_ok(),
                "{}: flags without DEVICE_MASK must be accepted: {:?}", context, result
            );
            let allocation = result.unwrap();
            prop_assert_eq!(
                fake.calls().allocate,
                calls_before.allocate + 1,
                "{}: the request must get new memory", context
            );
            let chain = fake.chains().get(&allocation.memory()).copied();
            prop_assert!(chain.is_some(), "{}: the new memory is not on the device", context);
            prop_assert_eq!(
                chain.unwrap().flags,
                (!flags.is_empty()).then_some((flags, 0)),
                "{}: VkMemoryAllocateFlagsInfo must be chained only for non-empty flags, \
                 with those flags and a deviceMask of 0",
                context
            );
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
