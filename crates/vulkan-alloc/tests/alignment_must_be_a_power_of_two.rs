mod fake_device;

use fake_device::{
    DEVICE_LOCAL, FakeDevice, HOST_COHERENT, HOST_NON_COHERENT, NON_COHERENT_ATOM_SIZE, desc,
};
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
    ZeroInitialized,
}

const KINDS: [Kind; 5] = [
    Kind::Shared,
    Kind::DedicatedBuffer,
    Kind::DedicatedImage,
    Kind::Exportable,
    Kind::ZeroInitialized,
];

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

fn alignment() -> impl Strategy<Value = u64> {
    prop_oneof![
        2 => any::<u64>(),
        2 => (0..64u32).prop_map(|shift| 1u64 << shift),
        2 => (1..64u32, select(vec![-1i64, 1])).prop_map(|(shift, step)| {
            (1u64 << shift).wrapping_add_signed(step)
        }),
        1 => select(vec![
            0,
            1,
            2,
            3,
            6,
            12,
            48,
            65,
            1000,
            1 << 63,
            (1 << 63) + 1,
            u64::MAX - 1,
            u64::MAX,
        ]),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 4096,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn only_alignments_of_zero_or_a_power_of_two_are_accepted(
        alignment in alignment(),
        memory_type in select(vec![DEVICE_LOCAL, HOST_COHERENT, HOST_NON_COHERENT]),
        kind in select(KINDS.to_vec()),
        earlier_sizes in proptest::collection::vec(1..=600u64, 0..4),
        size in 1..=512u64,
    ) {
        let fake = FakeDevice::new();
        let mut allocator = fake.allocator();
        let mut earlier = Vec::new();
        for &earlier_size in &earlier_sizes {
            earlier.push(unsafe { allocator.allocate(desc(earlier_size, 1, memory_type)) }.unwrap());
        }
        let context = format!(
            "alignment {alignment}, memory type {memory_type}, {kind:?}, \
             earlier sizes {earlier_sizes:?}, size {size}"
        );

        let calls_before = fake.calls();
        let snapshot_before = allocator.snapshot();
        let result = unsafe { allocator.allocate(of_kind(desc(size, alignment, memory_type), kind)) };

        if alignment == 0 || alignment.is_power_of_two() {
            prop_assert!(
                result.is_ok(),
                "{}: an alignment of 0 or a power of two must be accepted: {:?}",
                context,
                result
            );
            let allocation = result.unwrap();
            prop_assert_eq!(
                allocation.offset() % alignment.max(1),
                0,
                "{}: the offset {} is not a multiple of the alignment",
                context,
                allocation.offset()
            );
            if memory_type == HOST_NON_COHERENT {
                prop_assert_eq!(
                    allocation.offset() % NON_COHERENT_ATOM_SIZE,
                    0,
                    "{}: the offset {} in non-coherent memory is not a multiple of the atom",
                    context,
                    allocation.offset()
                );
            }
            unsafe { allocator.free(allocation) }.unwrap();
        } else {
            prop_assert_eq!(
                &result,
                &Err(AllocationError::InvalidAllocationCreateDesc),
                "{}: an alignment that is neither 0 nor a power of two must be refused",
                context
            );
            prop_assert_eq!(
                fake.calls().allocate,
                calls_before.allocate,
                "{}: a refused allocation must not call vkAllocateMemory",
                context
            );
            prop_assert_eq!(
                allocator.snapshot(),
                snapshot_before,
                "{}: a refused allocation must leave the allocator as it was",
                context
            );
        }

        for allocation in earlier {
            unsafe { allocator.free(allocation) }.unwrap();
        }
        prop_assert!(allocator.is_empty(), "{}", context);
        prop_assert_eq!(fake.misuse(), Vec::<String>::new(), "{}", context);
        prop_assert!(fake.memory_objects().is_empty(), "{}", context);
    }
}
