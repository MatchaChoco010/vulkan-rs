mod fake_device;

use fake_device::{FakeDevice, Synchronization, SynchronizedRange, desc};
use proptest::prelude::*;
use vulkan::vk;
use vulkan_alloc::{Allocation, AllocationError, Allocator, AllocatorCreateDesc};

const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Span {
    Inside { offset: u64, size: u64 },
    Given { offset: u64, size: u64 },
}

impl Span {
    fn resolve(self, length: u64) -> (u64, u64) {
        match self {
            Span::Inside { offset, size } => {
                let offset = offset % (length + 1);
                (offset, size % (length - offset + 1))
            }
            Span::Given { offset, size } => (offset, size),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Sized(u64),
    FillsTheRestOfTheBlock,
    LargerThanTheBlock(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Route {
    Allocator,
    Guard,
}

#[derive(Debug, Clone)]
struct Case {
    atom: u64,
    block_size: u64,
    flags: vk::MemoryPropertyFlags,
    before: Vec<u64>,
    target: Target,
    after: Vec<u64>,
    span: Span,
    route: Route,
    kind: Synchronization,
}

struct Layout {
    fake: FakeDevice,
    allocator: Allocator,
    target: Allocation,
    target_block_size: u64,
    neighbours: Vec<Allocation>,
}

fn given_values() -> impl Strategy<Value = u64> {
    prop_oneof![
        3 => 0..=800u64,
        1 => prop::sample::select(vec![u64::MAX, u64::MAX - 1, 1 << 63, usize::MAX as u64]),
    ]
}

fn cases() -> impl Strategy<Value = Case> {
    (
        prop::sample::select(vec![1u64, 2, 4, 64, 256, 1024]),
        prop::sample::select(vec![16384u64, 4097, 3001, 2200]),
        prop::sample::select(vec![VISIBLE, VISIBLE | CACHED, VISIBLE | COHERENT]),
        prop::collection::vec(1..=700u64, 0..=2),
        prop_oneof![
            4 => (1..=700u64).prop_map(Target::Sized),
            2 => Just(Target::FillsTheRestOfTheBlock),
            1 => (1..=300u64).prop_map(Target::LargerThanTheBlock),
        ],
        prop::collection::vec(1..=700u64, 0..=2),
        prop_oneof![
            3 => (any::<u64>(), any::<u64>()).prop_map(|(offset, size)| Span::Inside { offset, size }),
            2 => (given_values(), given_values()).prop_map(|(offset, size)| Span::Given { offset, size }),
        ],
        prop::sample::select(vec![Route::Allocator, Route::Guard]),
        prop::sample::select(vec![Synchronization::Flush, Synchronization::Invalidate]),
    )
        .prop_map(
            |(atom, block_size, flags, before, target, after, span, route, kind)| Case {
                atom,
                block_size,
                flags,
                before,
                target,
                after,
                span,
                route,
                kind,
            },
        )
}

fn layout(case: &Case) -> Layout {
    let fake = FakeDevice::with_non_coherent_atom_size(case.atom);
    let mut memory_properties = vk::PhysicalDeviceMemoryProperties {
        memory_type_count: 1,
        memory_heap_count: 1,
        ..Default::default()
    };
    memory_properties.memory_types[0].property_flags = case.flags;
    let mut allocator = Allocator::new(AllocatorCreateDesc {
        device: fake.device(),
        memory_properties,
        non_coherent_atom_size: case.atom,
        block_size: case.block_size,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
    });
    unsafe {
        let mut allocate = |size: u64| allocator.allocate(desc(size, 1, 0)).unwrap();
        let mut neighbours = case
            .before
            .iter()
            .map(|&size| allocate(size))
            .collect::<Vec<_>>();
        let target = match case.target {
            Target::Sized(size) => allocate(size),
            Target::FillsTheRestOfTheBlock => {
                let alignment = if case.flags.contains(COHERENT) {
                    1
                } else {
                    case.atom
                };
                let free = neighbours.last().map_or(0, |last| {
                    (last.offset() + last.size()).next_multiple_of(alignment)
                });
                let target = allocate(case.block_size - free);
                assert_eq!(
                    (target.offset(), target.offset() + target.size()),
                    (free, case.block_size),
                    "the place of the allocation that fills the rest of the block"
                );
                target
            }
            Target::LargerThanTheBlock(extra) => allocate(case.block_size + extra),
        };
        let target_block_size = case.block_size.max(target.size());
        neighbours.extend(case.after.iter().map(|&size| allocate(size)));
        for neighbour in &neighbours {
            allocator.map_memory(neighbour).unwrap();
        }
        Layout {
            fake,
            allocator,
            target,
            target_block_size,
            neighbours,
        }
    }
}

fn synchronize_range(
    allocator: &mut Allocator,
    target: &Allocation,
    route: Route,
    kind: Synchronization,
    (offset, size): (u64, u64),
) -> Result<(), AllocationError> {
    unsafe {
        match route {
            Route::Allocator => {
                allocator.map_memory(target)?;
                let result = match kind {
                    Synchronization::Flush => {
                        allocator.flush_allocation_range(target, offset, size)
                    }
                    Synchronization::Invalidate => {
                        allocator.invalidate_allocation_range(target, offset, size)
                    }
                };
                allocator.unmap_memory(target)?;
                result
            }
            Route::Guard => {
                let offset = usize::try_from(offset).unwrap_or(usize::MAX);
                let size = usize::try_from(size).unwrap_or(usize::MAX);
                let mut mapped = allocator.map(target)?;
                match kind {
                    Synchronization::Flush => mapped.flush_range(offset, size),
                    Synchronization::Invalidate => mapped.invalidate_range(offset, size),
                }
            }
        }
    }
}

fn whole_allocation_range(allocation: &Allocation, atom: u64, block_size: u64) -> (u64, u64) {
    let start = allocation.offset() / atom * atom;
    let end = (allocation.offset() + allocation.size()).div_ceil(atom) * atom;
    (start, end.min(block_size))
}

fn check_range(case: &Case, layout: &Layout, requested: (u64, u64), range: &SynchronizedRange) {
    let target = &layout.target;
    let block_size = layout.target_block_size;
    let (requested_start, requested_end) = requested;
    assert_eq!(range.kind, case.kind, "the command that was called");
    assert_eq!(range.memory, target.memory(), "the memory of the range");
    assert!(
        range.offset.is_multiple_of(case.atom),
        "the range starts at {}, not a multiple of the atom {}",
        range.offset,
        case.atom
    );
    let end = if range.size == vk::WHOLE_SIZE {
        block_size
    } else {
        assert!(
            range.size.is_multiple_of(case.atom) || range.offset + range.size == block_size,
            "the range of {} bytes is neither a multiple of the atom {} nor reaches the end of \
             the memory",
            range.size,
            case.atom
        );
        range.offset + range.size
    };
    assert!(
        range.offset <= requested_start && requested_end <= end,
        "the range {}..{end} does not cover the requested bytes {requested_start}..{requested_end}",
        range.offset
    );
    let (whole_start, whole_end) = whole_allocation_range(target, case.atom, block_size);
    assert!(
        whole_start <= range.offset && end <= whole_end,
        "the range {}..{end} is not inside the range {whole_start}..{whole_end} of the whole \
         allocation",
        range.offset
    );
    for neighbour in &layout.neighbours {
        let (start, length) = (neighbour.offset(), neighbour.size());
        assert!(
            neighbour.memory() != target.memory() || start + length <= range.offset || end <= start,
            "the range {}..{end} reaches into the neighbour at {start} of {length} bytes",
            range.offset
        );
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1024,
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    #[test]
    fn a_flushed_or_invalidated_range_covers_the_request_and_stays_in_the_allocation(case in cases()) {
        let mut layout = layout(&case);
        let (offset, size) = case.span.resolve(layout.target.size());
        let inside = offset
            .checked_add(size)
            .is_some_and(|end| end <= layout.target.size());
        let synchronized_before = layout.fake.synchronized().len();
        let result = synchronize_range(&mut layout.allocator, &layout.target, case.route, case.kind, (offset, size));
        let ranges = layout.fake.synchronized()[synchronized_before..].to_vec();
        prop_assert_eq!(layout.fake.misuse(), Vec::<String>::new(), "misuse of the device");
        if !inside {
            prop_assert_eq!(result, Err(AllocationError::RangeOutOfBounds), "a range outside the allocation");
            prop_assert_eq!(ranges, Vec::new(), "ranges given to Vulkan for a range outside the allocation");
        } else if size == 0 || case.flags.contains(COHERENT) {
            prop_assert_eq!(result, Ok(()), "an empty range or coherent memory");
            prop_assert_eq!(ranges, Vec::new(), "ranges given to Vulkan for an empty range or coherent memory");
        } else {
            prop_assert_eq!(result, Ok(()), "a range inside the allocation");
            prop_assert_eq!(ranges.len(), 1, "ranges given to Vulkan for a range inside the allocation");
            let start = layout.target.offset() + offset;
            check_range(&case, &layout, (start, start + size), &ranges[0]);
        }
        unsafe { layout.allocator.free_all() };
    }

    #[test]
    fn the_whole_allocation_functions_act_as_the_range_of_the_whole_allocation(case in cases()) {
        let mut layout = layout(&case);
        let target = &layout.target;
        let allocator = &mut layout.allocator;
        unsafe {
            allocator.map_memory(target).unwrap();
            let before = layout.fake.synchronized().len();
            let whole = match case.kind {
                Synchronization::Flush => allocator.flush_allocation(target),
                Synchronization::Invalidate => allocator.invalidate_allocation(target),
            };
            let middle = layout.fake.synchronized().len();
            let range = match case.kind {
                Synchronization::Flush => allocator.flush_allocation_range(target, 0, target.size()),
                Synchronization::Invalidate => {
                    allocator.invalidate_allocation_range(target, 0, target.size())
                }
            };
            let logged = layout.fake.synchronized();
            prop_assert_eq!(&whole, &range, "the results of the whole allocation and of its range");
            prop_assert_eq!(&logged[before..middle], &logged[middle..], "the ranges given to Vulkan");
            allocator.free_all();
        }
    }
}
