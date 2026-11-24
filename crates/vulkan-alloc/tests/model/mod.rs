#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ptr::NonNull;

use proptest::prelude::*;
use proptest::strategy::Union;
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationResourceClass, AllocationScheme,
    Allocator, AllocatorCreateDesc, AllocatorSnapshot, FreeError, LiveAllocation, MemoryLocation,
    MemoryRegionState,
};

use crate::fake_device::{
    Calls, Chain, Dedicated, FakeDevice, MapFault, MemoryObject, Synchronization, SynchronizedRange,
};

pub const SMALLEST_REMAINDER_KEPT_FREE: u64 = 256;

macro_rules! ensure {
    ($condition:expr, $($message:tt)*) => {
        if !$condition {
            return Err(format!($($message)*));
        }
    };
}

macro_rules! ensure_eq {
    ($left:expr, $right:expr, $($message:tt)*) => {
        match (&$left, &$right) {
            (left, right) => {
                if left != right {
                    return Err(format!(
                        "{}: {:?} is not {:?}",
                        format!($($message)*),
                        left,
                        right
                    ));
                }
            }
        }
    };
}

type Checked = Result<(), String>;

pub fn config(cases: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(cases);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

const LOCAL: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
const VISIBLE: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_VISIBLE;
const COHERENT: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_COHERENT;
const CACHED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::HOST_CACHED;
const PROTECTED: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::PROTECTED;
const AMD: vk::MemoryPropertyFlags = vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD
    .union(vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD);

const NAMEABLE_FLAGS: [vk::MemoryPropertyFlags; 7] = [
    LOCAL,
    VISIBLE,
    COHERENT,
    CACHED,
    PROTECTED,
    vk::MemoryPropertyFlags::DEVICE_COHERENT_AMD,
    vk::MemoryPropertyFlags::DEVICE_UNCACHED_AMD,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryType {
    pub flags: vk::MemoryPropertyFlags,
    pub heap: u32,
}

impl MemoryType {
    pub const fn plain(flags: vk::MemoryPropertyFlags) -> Self {
        Self { flags, heap: 0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Heap {
    pub size: u64,
    pub tile_memory: bool,
}

const UNSIZED_HEAP: u32 = 0;
const TILE_MEMORY_HEAP: u32 = 1;
const SMALL_HEAP: u32 = 2;
const LARGE_HEAP: u32 = 3;
const HEAP_THAT_IS_NOT_REPORTED: u32 = 9;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct World {
    pub memory_types: Vec<MemoryType>,
    pub heaps: Vec<Heap>,
    pub block_size: u64,
    pub non_coherent_atom_size: u64,
}

impl World {
    pub fn plain() -> Self {
        Self {
            memory_types: vec![
                MemoryType::plain(LOCAL),
                MemoryType::plain(VISIBLE.union(COHERENT)),
                MemoryType::plain(VISIBLE.union(CACHED)),
            ],
            heaps: vec![Heap {
                size: 0,
                tile_memory: false,
            }],
            block_size: 4096,
            non_coherent_atom_size: 64,
        }
    }

    pub fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: self.memory_types.len() as u32,
            memory_heap_count: self.heaps.len() as u32,
            ..Default::default()
        };
        for (index, heap) in self.heaps.iter().enumerate() {
            properties.memory_heaps[index].size = heap.size;
            if heap.tile_memory {
                properties.memory_heaps[index].flags = vk::MemoryHeapFlags::TILE_MEMORY_QCOM;
            }
        }
        for (index, memory_type) in self.memory_types.iter().enumerate() {
            properties.memory_types[index].property_flags = memory_type.flags;
            properties.memory_types[index].heap_index = memory_type.heap;
        }
        properties
    }

    fn atom(&self) -> u64 {
        self.non_coherent_atom_size.max(1)
    }

    fn heap_of(&self, memory_type: &MemoryType) -> Option<&Heap> {
        self.heaps.get(memory_type.heap as usize)
    }

    fn is_tile_memory(&self, memory_type: &MemoryType) -> bool {
        self.heap_of(memory_type)
            .is_some_and(|heap| heap.tile_memory)
    }

    fn known_heap_size(&self, memory_type: u32) -> Option<u64> {
        self.heap_of(&self.memory_types[memory_type as usize])
            .map(|heap| heap.size)
            .filter(|&size| size != 0)
    }

    fn shared_block_size(&self, memory_type: u32) -> u64 {
        let block_size = self.block_size.max(1);
        match self.known_heap_size(memory_type) {
            Some(heap_size) => block_size.min(heap_size),
            None => block_size,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    GpuOnly,
    CpuToGpu,
    GpuToCpu,
    Custom { required: u8, preferred: u8 },
}

fn nameable(mask: u8) -> vk::MemoryPropertyFlags {
    NAMEABLE_FLAGS
        .iter()
        .enumerate()
        .filter(|(bit, _)| mask >> bit & 1 == 1)
        .fold(vk::MemoryPropertyFlags::empty(), |all, (_, &flag)| {
            all.union(flag)
        })
}

impl Location {
    fn documented_flags(self) -> (vk::MemoryPropertyFlags, vk::MemoryPropertyFlags) {
        match self {
            Location::GpuOnly => (LOCAL, LOCAL),
            Location::CpuToGpu => (VISIBLE.union(COHERENT), VISIBLE.union(COHERENT)),
            Location::GpuToCpu => (
                VISIBLE.union(COHERENT),
                VISIBLE.union(COHERENT).union(CACHED),
            ),
            Location::Custom {
                required,
                preferred,
            } => (nameable(required), nameable(preferred)),
        }
    }

    fn memory_location(self) -> MemoryLocation {
        match self {
            Location::GpuOnly => MemoryLocation::GpuOnly,
            Location::CpuToGpu => MemoryLocation::CpuToGpu,
            Location::GpuToCpu => MemoryLocation::GpuToCpu,
            Location::Custom {
                required,
                preferred,
            } => MemoryLocation::custom(nameable(required), nameable(preferred)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    Managed,
    DedicatedBuffer,
    DedicatedImage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Request {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
    pub location: Location,
    pub linear: bool,
    pub device_address: bool,
    pub device_mask: bool,
    pub zero_initialize: bool,
    pub export: bool,
    pub scheme: Scheme,
    pub named: bool,
}

impl Request {
    pub const fn plain(size: u64, alignment: u64, memory_type_index: u32) -> Self {
        Self {
            size,
            alignment,
            memory_type_bits: 1 << memory_type_index,
            location: Location::Custom {
                required: 0,
                preferred: 0,
            },
            linear: true,
            device_address: false,
            device_mask: false,
            zero_initialize: false,
            export: false,
            scheme: Scheme::Managed,
            named: false,
        }
    }

    fn needs_its_own_memory(&self) -> bool {
        self.export || self.zero_initialize || self.scheme != Scheme::Managed
    }

    fn allocate_flags(&self) -> vk::MemoryAllocateFlags {
        let mut flags = vk::MemoryAllocateFlags::empty();
        if self.device_address {
            flags |= vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
        if self.device_mask {
            flags |= vk::MemoryAllocateFlags::DEVICE_MASK;
        }
        if self.zero_initialize {
            flags |= vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT;
        }
        flags
    }

    fn export_handle_types(&self) -> vk::ExternalMemoryHandleTypeFlags {
        if self.export {
            vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT
        } else {
            vk::ExternalMemoryHandleTypeFlags::empty()
        }
    }
}

pub fn documented_memory_type(world: &World, request: &Request) -> Option<u32> {
    let (required, preferred) = request.location.documented_flags();
    let named = required.union(preferred);
    let candidates = world
        .memory_types
        .iter()
        .enumerate()
        .filter(|&(index, memory_type)| {
            index < 32
                && request.memory_type_bits >> index & 1 == 1
                && memory_type.flags.contains(required)
                && !world.is_tile_memory(memory_type)
                && (!memory_type.flags.intersects(AMD) || named.intersects(AMD))
                && (!memory_type.flags.contains(PROTECTED) || named.contains(PROTECTED))
        })
        .map(|(index, memory_type)| (index as u32, memory_type.flags))
        .collect::<Vec<_>>();
    let shared =
        |flags: vk::MemoryPropertyFlags| (flags.as_raw() & preferred.as_raw()).count_ones();
    let most = candidates.iter().map(|&(_, flags)| shared(flags)).max()?;
    candidates
        .iter()
        .find(|&&(_, flags)| flags.contains(preferred))
        .or_else(|| candidates.iter().find(|&&(_, flags)| shared(flags) == most))
        .map(|&(index, _)| index)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pick {
    pub index: u16,
    pub any: bool,
}

impl Pick {
    pub const fn any(index: u16) -> Self {
        Self { index, any: true }
    }

    pub const fn fitting(index: u16) -> Self {
        Self { index, any: false }
    }

    fn among(self, count: usize) -> Option<usize> {
        (count > 0).then(|| usize::from(self.index) % count)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardUse {
    pub flush: bool,
    pub invalidate: bool,
    pub flush_range: bool,
    pub invalidate_range: bool,
    pub range_offset: u64,
    pub range_size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Span {
    Inside { offset: u64, size: u64 },
    Given { offset: u64, size: u64 },
}

impl Span {
    fn resolve(self, length: u64) -> (u64, u64) {
        match self {
            Span::Inside { offset, size } => {
                let offset = offset % length.saturating_add(1);
                (offset, size % (length - offset).saturating_add(1))
            }
            Span::Given { offset, size } => (offset, size),
        }
    }
}

fn range_function(kind: Synchronization) -> &'static str {
    match kind {
        Synchronization::Flush => "flush_allocation_range",
        Synchronization::Invalidate => "invalidate_allocation_range",
    }
}

fn guard_range_function(kind: Synchronization) -> &'static str {
    match kind {
        Synchronization::Flush => "flush_range",
        Synchronization::Invalidate => "invalidate_range",
    }
}

struct GuardRange {
    kind: Synchronization,
    logged_before: usize,
    requested: Option<(u64, u64)>,
    event: String,
}

type GuardUsed = (usize, Vec<(Synchronization, usize)>, Vec<GuardRange>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleUse {
    Free,
    MapMemory,
    Guard,
    MapPersistent,
    PersistentPtr,
    UnmapMemory,
    Flush,
    Invalidate,
    FlushRange,
    InvalidateRange,
    MemoryFlags,
}

pub const HANDLE_USES: [HandleUse; 11] = [
    HandleUse::Free,
    HandleUse::MapMemory,
    HandleUse::Guard,
    HandleUse::MapPersistent,
    HandleUse::PersistentPtr,
    HandleUse::UnmapMemory,
    HandleUse::Flush,
    HandleUse::Invalidate,
    HandleUse::FlushRange,
    HandleUse::InvalidateRange,
    HandleUse::MemoryFlags,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Allocate(Request),
    Free(Pick),
    MapMemory(Pick),
    UnmapMemory(Pick),
    MapPersistent(Pick),
    Guard(Pick, GuardUse),
    Flush(Pick),
    Invalidate(Pick),
    FlushRange(Pick, Span),
    InvalidateRange(Pick, Span),
    FreeAll,
    UseStale(Pick, HandleUse),
    UseForeign(Pick, HandleUse),
    FailNextAllocation(vk::VkResult),
    FaultNextMap(MapFault),
    FailNextSynchronization(vk::VkResult),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpKind {
    Allocate,
    Free,
    MapMemory,
    UnmapMemory,
    MapPersistent,
    Guard,
    Flush,
    Invalidate,
    FreeAll,
    UseStale,
    UseForeign,
    FailNextAllocation,
    FaultNextMap,
    FailNextSynchronization,
}

pub const OP_KINDS: [OpKind; 14] = [
    OpKind::Allocate,
    OpKind::Free,
    OpKind::MapMemory,
    OpKind::UnmapMemory,
    OpKind::MapPersistent,
    OpKind::Guard,
    OpKind::Flush,
    OpKind::Invalidate,
    OpKind::FreeAll,
    OpKind::UseStale,
    OpKind::UseForeign,
    OpKind::FailNextAllocation,
    OpKind::FaultNextMap,
    OpKind::FailNextSynchronization,
];

impl Op {
    pub fn kind(&self) -> OpKind {
        match self {
            Op::Allocate(_) => OpKind::Allocate,
            Op::Free(_) => OpKind::Free,
            Op::MapMemory(_) => OpKind::MapMemory,
            Op::UnmapMemory(_) => OpKind::UnmapMemory,
            Op::MapPersistent(_) => OpKind::MapPersistent,
            Op::Guard(..) => OpKind::Guard,
            Op::Flush(_) | Op::FlushRange(..) => OpKind::Flush,
            Op::Invalidate(_) | Op::InvalidateRange(..) => OpKind::Invalidate,
            Op::FreeAll => OpKind::FreeAll,
            Op::UseStale(..) => OpKind::UseStale,
            Op::UseForeign(..) => OpKind::UseForeign,
            Op::FailNextAllocation(_) => OpKind::FailNextAllocation,
            Op::FaultNextMap(_) => OpKind::FaultNextMap,
            Op::FailNextSynchronization(_) => OpKind::FailNextSynchronization,
        }
    }
}

pub fn worlds() -> impl Strategy<Value = World> {
    let extra = prop_oneof![
        2 => Just(MemoryType::plain(LOCAL)),
        2 => Just(MemoryType::plain(VISIBLE.union(COHERENT))),
        2 => Just(MemoryType::plain(VISIBLE.union(CACHED))),
        2 => Just(MemoryType::plain(VISIBLE)),
        2 => Just(MemoryType::plain(LOCAL.union(VISIBLE).union(COHERENT))),
        1 => Just(MemoryType::plain(LOCAL.union(VISIBLE).union(COHERENT).union(CACHED))),
        1 => Just(MemoryType::plain(LOCAL.union(VISIBLE).union(CACHED))),
        2 => Just(MemoryType::plain(LOCAL.union(VISIBLE).union(COHERENT).union(AMD))),
        2 => Just(MemoryType::plain(LOCAL.union(PROTECTED))),
        1 => Just(MemoryType { flags: LOCAL, heap: TILE_MEMORY_HEAP }),
        1 => Just(MemoryType { flags: VISIBLE.union(COHERENT), heap: TILE_MEMORY_HEAP }),
    ];
    let heap = prop_oneof![
        6 => Just(UNSIZED_HEAP),
        4 => Just(SMALL_HEAP),
        1 => Just(LARGE_HEAP),
        1 => Just(HEAP_THAT_IS_NOT_REPORTED),
    ];
    let extra = (extra, heap).prop_map(|(mut memory_type, heap)| {
        if memory_type.heap != TILE_MEMORY_HEAP {
            memory_type.heap = heap;
        }
        memory_type
    });
    let memory_types = prop_oneof![
        5 => prop::collection::vec(extra.clone(), 0..=3)
            .prop_map(|mut memory_types| {
                memory_types.extend(World::plain().memory_types);
                memory_types
            })
            .prop_shuffle(),
        1 => prop::collection::vec(extra, 1..=6),
    ];
    (
        memory_types,
        prop::sample::select(vec![1u64, 300, 1000, 3000, 70_000]),
        prop_oneof![
            1 => Just(0u64),
            1 => Just(1u64),
            1 => Just(256u64),
            3 => Just(1024u64),
            6 => Just(4096u64),
            1 => Just(5000u64),
            3 => Just(65536u64),
        ],
        prop_oneof![
            1 => Just(0u64),
            2 => Just(1u64),
            2 => Just(16u64),
            4 => Just(64u64),
            2 => Just(256u64),
            1 => Just(4096u64),
        ],
    )
        .prop_map(
            |(memory_types, small_heap, block_size, non_coherent_atom_size)| World {
                memory_types,
                heaps: [0, 0, small_heap, u64::MAX]
                    .into_iter()
                    .enumerate()
                    .map(|(index, size)| Heap {
                        size,
                        tile_memory: index as u32 == TILE_MEMORY_HEAP,
                    })
                    .collect(),
                block_size,
                non_coherent_atom_size,
            },
        )
}

fn sizes(block_size: u64) -> impl Strategy<Value = u64> {
    let block = block_size.clamp(1, 1 << 16);
    prop_oneof![
        8 => 1..=64u64,
        6 => 1..=(block / 8).max(1),
        3 => 1..=1024u64,
        2 => prop::sample::select(vec![255u64, 256, 257, 511, 512, 513]),
        3 => (prop::sample::select(vec![block / 4, block / 2, block]), -2i64..=2)
            .prop_map(|(base, delta)| base.saturating_add_signed(delta)),
        2 => block.saturating_sub(300)..=block,
        1 => block..=block * 3,
        1 => Just(0u64),
    ]
}

fn alignments() -> impl Strategy<Value = u64> {
    prop_oneof![
        8 => Just(1u64),
        1 => Just(0u64),
        5 => (1u32..=8).prop_map(|shift| 1u64 << shift),
        1 => (9u32..=17).prop_map(|shift| 1u64 << shift),
        1 => prop_oneof![
            (2u32..=17).prop_map(|shift| (1u64 << shift) - 1),
            (1u32..=17).prop_map(|shift| (1u64 << shift) + 1),
            prop::sample::select(vec![3u64, 6, 12, 48, 96, 1000, (1 << 63) + 1, u64::MAX]),
        ],
    ]
}

fn locations() -> impl Strategy<Value = Location> {
    prop_oneof![
        4 => Just(Location::GpuOnly),
        6 => Just(Location::CpuToGpu),
        4 => Just(Location::GpuToCpu),
        3 => (
            prop_oneof![
                6 => Just(0u8),
                3 => (0u32..7).prop_map(|bit| 1u8 << bit),
                1 => (any::<u8>(), any::<u8>()).prop_map(|(a, b)| a & b & 0x7f),
            ],
            any::<u8>(),
        )
            .prop_map(|(required, preferred)| Location::Custom {
                required,
                preferred: preferred & 0x7f,
            }),
        2 => Just(Location::Custom { required: 0, preferred: 0 }),
    ]
}

pub fn requests(world: &World) -> impl Strategy<Value = Request> + use<> {
    let memory_type_bits = prop_oneof![
        12 => Just(u32::MAX),
        3 => any::<u32>(),
        1 => (0u32..8).prop_map(|index| 1u32 << index),
        1 => Just(0u32),
    ];
    let scheme = prop_oneof![
        18 => Just(Scheme::Managed),
        1 => Just(Scheme::DedicatedBuffer),
        1 => Just(Scheme::DedicatedImage),
    ];
    (
        sizes(world.block_size),
        alignments(),
        memory_type_bits,
        locations(),
        prop::bool::weighted(0.85),
        prop::bool::weighted(0.1),
        prop::bool::weighted(0.02),
        prop::bool::weighted(0.06),
        prop::bool::weighted(0.05),
        scheme,
        prop::bool::weighted(0.3),
    )
        .prop_map(
            |(
                size,
                alignment,
                memory_type_bits,
                location,
                linear,
                device_address,
                device_mask,
                zero_initialize,
                export,
                scheme,
                named,
            )| Request {
                size,
                alignment,
                memory_type_bits,
                location,
                linear,
                device_address,
                device_mask,
                zero_initialize,
                export,
                scheme,
                named,
            },
        )
}

fn picks() -> impl Strategy<Value = Pick> {
    (any::<u16>(), prop::bool::weighted(0.25)).prop_map(|(index, any)| Pick { index, any })
}

fn spans() -> impl Strategy<Value = Span> {
    let given = || {
        prop_oneof![
            3 => 0..=5000u64,
            1 => prop::sample::select(vec![u64::MAX, u64::MAX - 1, 1 << 63]),
        ]
    };
    prop_oneof![
        3 => (any::<u64>(), any::<u64>()).prop_map(|(offset, size)| Span::Inside { offset, size }),
        2 => (given(), given()).prop_map(|(offset, size)| Span::Given { offset, size }),
    ]
}

fn handle_uses() -> impl Strategy<Value = HandleUse> {
    prop::sample::select(HANDLE_USES.to_vec())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mix {
    pub weights: [u32; OP_KINDS.len()],
}

impl Mix {
    pub const EVERYTHING: Self = Self {
        weights: [40, 12, 8, 8, 6, 8, 8, 8, 1, 3, 3, 2, 2, 3],
    };
}

pub fn mixes() -> impl Strategy<Value = Mix> {
    prop_oneof![
        1 => Just(Mix::EVERYTHING),
        2 => prop::array::uniform14(prop::bool::weighted(0.6)).prop_map(|enabled| {
            let mut weights = Mix::EVERYTHING.weights;
            for (weight, enabled) in weights.iter_mut().zip(enabled).skip(1) {
                if !enabled {
                    *weight = 0;
                }
            }
            Mix { weights }
        }),
    ]
}

pub fn ops(world: &World, mix: Mix) -> impl Strategy<Value = Op> + use<> {
    let guard_use = (
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        prop_oneof![Just(0u64), 0..=64u64, 0..=5000u64, Just(u64::MAX)],
        prop_oneof![Just(0u64), 0..=64u64, 0..=5000u64, Just(u64::MAX)],
    )
        .prop_map(
            |(flush, invalidate, flush_range, invalidate_range, range_offset, range_size)| {
                GuardUse {
                    flush,
                    invalidate,
                    flush_range,
                    invalidate_range,
                    range_offset,
                    range_size,
                }
            },
        );
    let by_kind = |kind: OpKind| -> BoxedStrategy<Op> {
        match kind {
            OpKind::Allocate => requests(world).prop_map(Op::Allocate).boxed(),
            OpKind::Free => picks().prop_map(Op::Free).boxed(),
            OpKind::MapMemory => picks().prop_map(Op::MapMemory).boxed(),
            OpKind::UnmapMemory => picks().prop_map(Op::UnmapMemory).boxed(),
            OpKind::MapPersistent => picks().prop_map(Op::MapPersistent).boxed(),
            OpKind::Guard => (picks(), guard_use.clone())
                .prop_map(|(pick, guard_use)| Op::Guard(pick, guard_use))
                .boxed(),
            OpKind::Flush => prop_oneof![
                picks().prop_map(Op::Flush),
                (picks(), spans()).prop_map(|(pick, span)| Op::FlushRange(pick, span)),
            ]
            .boxed(),
            OpKind::Invalidate => prop_oneof![
                picks().prop_map(Op::Invalidate),
                (picks(), spans()).prop_map(|(pick, span)| Op::InvalidateRange(pick, span)),
            ]
            .boxed(),
            OpKind::FreeAll => Just(Op::FreeAll).boxed(),
            OpKind::UseStale => (picks(), handle_uses())
                .prop_map(|(pick, handle_use)| Op::UseStale(pick, handle_use))
                .boxed(),
            OpKind::UseForeign => (picks(), handle_uses())
                .prop_map(|(pick, handle_use)| Op::UseForeign(pick, handle_use))
                .boxed(),
            OpKind::FailNextAllocation => prop_oneof![
                2 => Just(vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY),
                1 => prop::sample::select(OTHER_ALLOCATION_FAILURES.to_vec()),
            ]
            .prop_map(Op::FailNextAllocation)
            .boxed(),
            OpKind::FaultNextMap => prop_oneof![
                Just(MapFault::ReturnsNull),
                prop::sample::select(MAP_FAILURES.to_vec()).prop_map(MapFault::Fails),
            ]
            .prop_map(Op::FaultNextMap)
            .boxed(),
            OpKind::FailNextSynchronization => {
                prop::sample::select(SYNCHRONIZATION_FAILURES.to_vec())
                    .prop_map(Op::FailNextSynchronization)
                    .boxed()
            }
        }
    };
    Union::new_weighted(
        OP_KINDS
            .iter()
            .zip(mix.weights)
            .filter(|&(_, weight)| weight > 0)
            .map(|(&kind, weight)| (weight, by_kind(kind)))
            .collect(),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    pub world: World,
    pub mix: Mix,
    pub ops: Vec<Op>,
}

pub fn scenarios(longest: usize) -> impl Strategy<Value = Scenario> {
    (worlds(), mixes()).prop_flat_map(move |(world, mix)| {
        prop::collection::vec(ops(&world, mix), 0..=longest).prop_map(move |ops| Scenario {
            world: world.clone(),
            mix,
            ops,
        })
    })
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Coverage {
    pub events: BTreeMap<String, u64>,
    pub pairs: BTreeSet<(String, String)>,
    last: Option<String>,
}

impl Coverage {
    fn step(&mut self, outcome: impl Into<String>) {
        let outcome = outcome.into();
        *self.events.entry(outcome.clone()).or_default() += 1;
        if let Some(last) = self.last.replace(outcome.clone()) {
            self.pairs.insert((last, outcome));
        }
    }

    fn note(&mut self, event: impl Into<String>) {
        *self.events.entry(event.into()).or_default() += 1;
    }

    pub fn merge(&mut self, other: &Coverage) {
        for (event, count) in &other.events {
            *self.events.entry(event.clone()).or_default() += count;
        }
        self.pairs.extend(other.pairs.iter().cloned());
    }

    pub fn count(&self, event: &str) -> u64 {
        self.events.get(event).copied().unwrap_or(0)
    }
}

struct Live {
    allocation: Allocation,
    twin: Option<Allocation>,
    requested_size: u64,
    alignment: u64,
    name: Option<String>,
    raw_mappings: usize,
    persistent: Option<NonNull<u8>>,
    allocation_id: u64,
    fill: Option<u8>,
    zero_initialized: bool,
}

impl Live {
    fn memory(&self) -> u64 {
        self.allocation.memory().0
    }

    fn range(&self) -> (u64, u64) {
        (self.allocation.offset(), self.allocation.size())
    }

    fn mapping_references(&self) -> usize {
        self.raw_mappings + usize::from(self.persistent.is_some())
    }

    fn mapping_state(&self) -> &'static str {
        match (self.raw_mappings, self.persistent.is_some()) {
            (0, false) => "unmapped",
            (0, true) => "persistent",
            (1, false) => "raw",
            (_, false) => "raw twice or more",
            (_, true) => "raw and persistent",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Block {
    size: u64,
    memory_type: u32,
    linear: bool,
    allocate_flags: vk::MemoryAllocateFlags,
    own: bool,
    block_index: usize,
}

struct Observation {
    snapshot: AllocatorSnapshot,
    memory_objects: HashMap<vk::DeviceMemory, MemoryObject>,
    calls: Calls,
    synchronized: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapExpectation {
    NotHostVisible,
    Fails(vk::VkResult),
    ReturnsNull,
    MapsTheBlock,
    SharesTheMapping,
}

pub struct Harness {
    allocator: Allocator,
    twin: Allocator,
    fake: FakeDevice,
    twin_fake: FakeDevice,
    world: World,
    live: Vec<Live>,
    stale: Vec<Allocation>,
    blocks: BTreeMap<u64, Block>,
    allocation_fails: Option<vk::VkResult>,
    map_fault: Option<MapFault>,
    synchronization_fails: Option<vk::VkResult>,
    seen_allocation_ids: BTreeSet<u64>,
    made: u64,
    pub coverage: Coverage,
}

const OTHER_ALLOCATION_FAILURES: [vk::VkResult; 4] = [
    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
    vk::VkResult::ERROR_TOO_MANY_OBJECTS,
    vk::VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
    vk::VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
];
const MAP_FAILURES: [vk::VkResult; 3] = [
    vk::VkResult::ERROR_MEMORY_MAP_FAILED,
    vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
];
const SYNCHRONIZATION_FAILURES: [vk::VkResult; 2] = [
    vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
    vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
];

fn gaps(size: u64, taken: &[(u64, u64)]) -> Vec<(u64, u64)> {
    let mut taken = taken.to_vec();
    taken.sort_unstable();
    let mut gaps = Vec::new();
    let mut end = 0;
    for (offset, length) in taken {
        if offset > end {
            gaps.push((end, offset - end));
        }
        end = offset + length;
    }
    if size > end {
        gaps.push((end, size - end));
    }
    gaps
}

fn fits(gap: (u64, u64), size: u64, alignment: u64) -> bool {
    let (start, length) = gap;
    let padding = (alignment - start % alignment) % alignment;
    padding
        .checked_add(size)
        .is_some_and(|needed| needed <= length)
}

impl Harness {
    pub fn new(world: &World) -> Self {
        let fake = FakeDevice::with_non_coherent_atom_size(world.non_coherent_atom_size);
        let twin_fake = FakeDevice::with_non_coherent_atom_size(world.non_coherent_atom_size);
        let allocator = |fake: &FakeDevice| {
            Allocator::new(AllocatorCreateDesc {
                device: fake.device(),
                memory_properties: world.memory_properties(),
                non_coherent_atom_size: world.non_coherent_atom_size,
                block_size: world.block_size,
            })
        };
        Self {
            allocator: allocator(&fake),
            twin: allocator(&twin_fake),
            fake,
            twin_fake,
            world: world.clone(),
            live: Vec::new(),
            stale: Vec::new(),
            blocks: BTreeMap::new(),
            allocation_fails: None,
            map_fault: None,
            synchronization_fails: None,
            seen_allocation_ids: BTreeSet::new(),
            made: 0,
            coverage: Coverage::default(),
        }
    }

    pub fn run(scenario: &Scenario) -> Result<Coverage, String> {
        let mut harness = Harness::new(&scenario.world);
        for (step, op) in scenario.ops.iter().enumerate() {
            harness
                .apply(op)
                .map_err(|problem| format!("step {step}, {op:?}: {problem}"))?;
        }
        harness
            .finish()
            .map_err(|problem| format!("after the last step: {problem}"))
    }

    pub fn apply(&mut self, op: &Op) -> Checked {
        match *op {
            Op::Allocate(request) => self.allocate(&request)?,
            Op::Free(pick) => self.free(pick)?,
            Op::MapMemory(pick) => self.map_memory(pick)?,
            Op::UnmapMemory(pick) => self.unmap_memory(pick)?,
            Op::MapPersistent(pick) => self.map_persistent(pick)?,
            Op::Guard(pick, guard_use) => self.guard(pick, guard_use)?,
            Op::Flush(pick) => self.synchronize(pick, Synchronization::Flush)?,
            Op::Invalidate(pick) => self.synchronize(pick, Synchronization::Invalidate)?,
            Op::FlushRange(pick, span) => {
                self.synchronize_range(pick, Synchronization::Flush, span)?
            }
            Op::InvalidateRange(pick, span) => {
                self.synchronize_range(pick, Synchronization::Invalidate, span)?
            }
            Op::FreeAll => self.free_all()?,
            Op::UseStale(pick, handle_use) => self.use_stale(pick, handle_use)?,
            Op::UseForeign(pick, handle_use) => self.use_foreign(pick, handle_use)?,
            Op::FailNextAllocation(failure) => {
                self.allocation_fails = Some(failure);
                self.fake.fail_next_allocation(failure);
                self.coverage.step("arm a failing vkAllocateMemory");
            }
            Op::FaultNextMap(fault) => {
                self.map_fault = Some(fault);
                self.fake.fault_next_map(fault);
                self.coverage.step("arm a faulty vkMapMemory");
            }
            Op::FailNextSynchronization(failure) => {
                self.synchronization_fails = Some(failure);
                self.fake.fail_next_synchronization(failure);
                self.coverage.step("arm a failing flush or invalidate");
            }
        }
        self.check()
    }

    pub fn finish(mut self) -> Result<Coverage, String> {
        unsafe {
            for index in (0..self.live.len()).rev() {
                let mut live = self.live.remove(index);
                while live.raw_mappings > 0 {
                    self.allocator
                        .unmap_memory(&live.allocation)
                        .map_err(|error| format!("unmapping at the end: {error:?}"))?;
                    live.raw_mappings -= 1;
                }
                let memory = live.memory();
                let last_of_its_block = !self.live.iter().any(|other| other.memory() == memory);
                self.allocator
                    .free(live.allocation)
                    .map_err(|error| format!("freeing at the end: {error:?}"))?;
                if last_of_its_block {
                    self.blocks.remove(&memory);
                }
                if let Some(twin) = live.twin {
                    self.twin
                        .free(twin)
                        .map_err(|error| format!("freeing the twin at the end: {error:?}"))?;
                }
                self.check()?;
            }
        }
        ensure!(self.allocator.is_empty(), "the allocator is not empty");
        ensure!(self.twin.is_empty(), "the twin allocator is not empty");
        ensure_eq!(
            self.fake.memory_objects(),
            HashMap::new(),
            "memory left on the device"
        );
        ensure_eq!(
            self.twin_fake.memory_objects(),
            HashMap::new(),
            "memory left on the twin device"
        );
        let calls = self.fake.calls();
        ensure!(
            calls.allocate >= calls.free,
            "more vkFreeMemory than vkAllocateMemory calls: {calls:?}"
        );
        Ok(std::mem::take(&mut self.coverage))
    }

    fn pick_live(&self, pick: Pick, fitting: impl Fn(&Live) -> bool) -> Option<usize> {
        if !pick.any {
            let fitting = (0..self.live.len())
                .filter(|&index| fitting(&self.live[index]))
                .collect::<Vec<_>>();
            if let Some(chosen) = pick.among(fitting.len()) {
                return Some(fitting[chosen]);
            }
        }
        pick.among(self.live.len())
    }

    fn is_host_visible(&self, live: &Live) -> bool {
        self.flags_of(live.allocation.memory_type_index())
            .contains(VISIBLE)
    }

    fn is_coherent(&self, live: &Live) -> bool {
        self.flags_of(live.allocation.memory_type_index())
            .contains(COHERENT)
    }

    fn shares_its_block(&self, live: &Live) -> bool {
        self.live
            .iter()
            .filter(|other| other.memory() == live.memory())
            .count()
            > 1
    }

    fn observe(&self) -> Observation {
        Observation {
            snapshot: self.allocator.snapshot(),
            memory_objects: self.fake.memory_objects(),
            calls: self.fake.calls(),
            synchronized: self.fake.synchronized().len(),
        }
    }

    fn state_is_as(&self, before: &Observation, what: &str) -> Checked {
        ensure_eq!(
            self.allocator.snapshot(),
            before.snapshot,
            "{what} changed the allocator"
        );
        ensure_eq!(
            self.fake.memory_objects(),
            before.memory_objects,
            "{what} changed the memory of the device"
        );
        Ok(())
    }

    fn nothing_happened_since(&self, before: &Observation, what: &str) -> Checked {
        self.state_is_as(before, what)?;
        ensure_eq!(self.fake.calls(), before.calls, "{what} called the device");
        ensure_eq!(
            self.fake.synchronized().len(),
            before.synchronized,
            "{what} flushed or invalidated"
        );
        Ok(())
    }

    fn flags_of(&self, memory_type: u32) -> vk::MemoryPropertyFlags {
        self.world.memory_types[memory_type as usize].flags
    }

    fn taken_in(&self, memory: u64) -> Vec<(u64, u64)> {
        self.live
            .iter()
            .filter(|live| live.memory() == memory)
            .map(Live::range)
            .collect()
    }

    fn mapping_references_in(&self, memory: u64) -> usize {
        self.live
            .iter()
            .filter(|live| live.memory() == memory)
            .map(Live::mapping_references)
            .sum()
    }

    fn effective_alignment(&self, memory_type: u32, alignment: u64) -> u64 {
        let flags = self.flags_of(memory_type);
        if flags.contains(VISIBLE) && !flags.contains(COHERENT) {
            alignment.max(1).max(self.world.atom())
        } else {
            alignment.max(1)
        }
    }

    fn allocate(&mut self, request: &Request) -> Checked {
        self.made += 1;
        let resource = 1000 + self.made;
        let name = request.named.then(|| format!("allocation {}", self.made));
        let before = self.observe();
        let result = unsafe {
            self.allocator
                .allocate(create_desc(request, name.as_deref(), resource))
        };

        let invalid = if request.size == 0 {
            Some("0 bytes")
        } else if request.alignment != 0 && !request.alignment.is_power_of_two() {
            Some("alignment that is not a power of two")
        } else if request.device_mask {
            Some("DEVICE_MASK")
        } else {
            None
        };
        if let Some(invalid) = invalid {
            ensure_eq!(
                result,
                Err(AllocationError::InvalidAllocationCreateDesc),
                "a description with {invalid}"
            );
            self.coverage
                .step(format!("allocate: invalid description: {invalid}"));
            match documented_memory_type(&self.world, request) {
                None => self
                    .coverage
                    .note("invalid description without a compatible memory type"),
                Some(memory_type)
                    if self
                        .world
                        .known_heap_size(memory_type)
                        .is_some_and(|heap_size| request.size > heap_size) =>
                {
                    self.coverage
                        .note("invalid description larger than the memory heap")
                }
                Some(_) => {}
            }
            return self.nothing_happened_since(&before, "an invalid description");
        }
        let Some(memory_type) = documented_memory_type(&self.world, request) else {
            ensure_eq!(
                result,
                Err(AllocationError::NoCompatibleMemoryType),
                "no memory type is allowed and suitable"
            );
            self.coverage.step("allocate: no compatible memory type");
            return self.nothing_happened_since(&before, "a refused allocation");
        };
        if self
            .world
            .known_heap_size(memory_type)
            .is_some_and(|heap_size| request.size > heap_size)
        {
            ensure_eq!(
                result,
                Err(AllocationError::OutOfMemory),
                "an allocation larger than the memory heap of its type"
            );
            self.coverage.step("allocate: larger than the memory heap");
            return self.nothing_happened_since(&before, "an allocation larger than its heap");
        }

        let alignment = self.effective_alignment(memory_type, request.alignment);
        let own = request.needs_its_own_memory();
        let allocate_flags = request.allocate_flags();
        let shareable = |block: &Block| {
            !block.own
                && block.memory_type == memory_type
                && block.linear == request.linear
                && block.allocate_flags == allocate_flags
        };
        let has_room = !own
            && self.blocks.iter().any(|(&memory, block)| {
                shareable(block)
                    && gaps(block.size, &self.taken_in(memory))
                        .into_iter()
                        .any(|gap| fits(gap, request.size, alignment))
            });

        let calls = self.fake.calls();
        let allocation = if has_room {
            let allocation =
                result.map_err(|error| format!("a shared block has room, but: {error:?}"))?;
            ensure_eq!(
                calls.allocate,
                before.calls.allocate,
                "vkAllocateMemory calls although a shared block has room"
            );
            let block = self.blocks.get(&allocation.memory().0).copied();
            ensure!(
                block.as_ref().is_some_and(shareable),
                "placed in {block:?}, which is not a block the request can share"
            );
            self.coverage.step("allocate: into a shared block");
            allocation
        } else {
            let block_size = if own {
                request.size
            } else {
                self.world.shared_block_size(memory_type).max(request.size)
            };
            ensure_eq!(
                calls.allocate,
                before.calls.allocate + 1,
                "vkAllocateMemory calls for a request that needs new memory"
            );
            if let Some(failure) = self.allocation_fails.take() {
                if failure == vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY {
                    ensure_eq!(
                        result,
                        Err(AllocationError::OutOfMemory),
                        "the result when vkAllocateMemory runs out of device memory"
                    );
                    self.coverage.step("allocate: the device is out of memory");
                } else {
                    ensure_eq!(
                        result,
                        Err(AllocationError::Vulkan(failure)),
                        "the result when vkAllocateMemory fails with {failure:?}"
                    );
                    self.coverage.step("allocate: another Vulkan error");
                }
                return self.state_is_as(&before, "a failed allocation");
            }
            let allocation =
                result.map_err(|error| format!("new memory was allocated, but: {error:?}"))?;
            let memory = allocation.memory();
            ensure!(
                !before.memory_objects.contains_key(&memory),
                "new memory has the handle of existing memory {memory:?}"
            );
            ensure_eq!(
                self.fake.memory_objects().get(&memory),
                Some(&MemoryObject {
                    size: block_size,
                    memory_type_index: memory_type,
                    mapped: false,
                }),
                "the new memory object"
            );
            let chain = Chain {
                flags: (!allocate_flags.is_empty()).then_some((allocate_flags, 0)),
                dedicated: match request.scheme {
                    Scheme::Managed => None,
                    Scheme::DedicatedBuffer => Some(Dedicated::Buffer(vk::Buffer(resource))),
                    Scheme::DedicatedImage => Some(Dedicated::Image(vk::Image(resource))),
                },
                export: request.export.then_some(request.export_handle_types()),
            };
            ensure_eq!(
                self.fake.chains().get(&memory),
                Some(&chain),
                "the pNext chain of vkAllocateMemory"
            );
            let block_index = self.block_index_of(memory)?;
            self.blocks.insert(
                memory.0,
                Block {
                    size: block_size,
                    memory_type,
                    linear: request.linear,
                    allocate_flags,
                    own,
                    block_index,
                },
            );
            if request.zero_initialize {
                self.coverage.note("allocation of zero-initialized memory");
            }
            if !own && self.world.shared_block_size(memory_type) < self.world.block_size {
                self.coverage
                    .note("shared block limited by its memory heap");
            }
            self.coverage.step(if own {
                "allocate: memory of its own"
            } else if request.size > self.world.shared_block_size(memory_type) {
                "allocate: a new block of the size of the allocation"
            } else {
                "allocate: a new shared block"
            });
            allocation
        };

        ensure_eq!(
            allocation.memory_type_index(),
            memory_type,
            "the memory type of the allocation"
        );
        let memory = allocation.memory().0;
        let block = self.blocks[&memory];
        let (offset, size) = (allocation.offset(), allocation.size());
        ensure!(
            offset % alignment == 0,
            "offset {offset} is not a multiple of {alignment}"
        );
        let gap = gaps(block.size, &self.taken_in(memory))
            .into_iter()
            .find(|&(start, length)| {
                start <= offset
                    && offset
                        .checked_add(size)
                        .is_some_and(|end| end <= start + length)
            });
        let Some((start, length)) = gap else {
            return Err(format!(
                "{size} bytes at {offset} are not inside one free range of the block"
            ));
        };
        ensure!(
            offset - start < alignment,
            "{} bytes are skipped before offset {offset}, more than alignment {alignment} needs",
            offset - start
        );
        ensure!(
            size >= request.size,
            "{size} bytes were given for a request of {} bytes",
            request.size
        );
        let remainder = start + length - offset - request.size;
        if own {
            ensure_eq!(
                (offset, size),
                (0, request.size),
                "the range of an allocation with memory of its own"
            );
        } else if remainder < SMALLEST_REMAINDER_KEPT_FREE {
            ensure_eq!(
                size,
                request.size + remainder,
                "the size when {remainder} bytes remain after the allocation"
            );
            if remainder > 0 {
                self.coverage.note("allocation takes a small remainder");
            }
        } else {
            ensure_eq!(
                size,
                request.size,
                "the size when {remainder} bytes remain after the allocation"
            );
        }
        if offset > start {
            self.coverage.note("allocation after alignment padding");
        }
        if alignment > request.alignment.max(1) {
            self.coverage
                .note("allocation aligned to nonCoherentAtomSize");
        }

        let allocation_id = self.allocation_id_at(memory, offset)?;
        ensure!(
            self.seen_allocation_ids.insert(allocation_id),
            "allocation id {allocation_id} was used before"
        );

        let twin = unsafe {
            self.twin
                .allocate(create_desc(request, name.as_deref(), resource))
        }
        .map_err(|error| format!("the twin allocator refused the same request: {error:?}"))?;
        ensure_eq!(
            (
                twin.memory(),
                twin.offset(),
                twin.size(),
                twin.memory_type_index()
            ),
            (
                allocation.memory(),
                offset,
                size,
                allocation.memory_type_index()
            ),
            "an allocator given the same requests placed this one elsewhere"
        );

        self.live.push(Live {
            allocation,
            twin: Some(twin),
            requested_size: request.size,
            alignment,
            name,
            raw_mappings: 0,
            persistent: None,
            allocation_id,
            fill: None,
            zero_initialized: request.zero_initialize,
        });
        Ok(())
    }

    fn block_index_of(&self, memory: vk::DeviceMemory) -> Result<usize, String> {
        self.allocator
            .snapshot()
            .memory_types
            .iter()
            .flat_map(|memory_type| &memory_type.blocks)
            .find(|block| block.memory == memory)
            .map(|block| block.block_index)
            .ok_or_else(|| format!("the snapshot has no block for {memory:?}"))
    }

    fn allocation_id_at(&self, memory: u64, offset: u64) -> Result<u64, String> {
        self.allocator
            .snapshot()
            .memory_types
            .iter()
            .flat_map(|memory_type| &memory_type.blocks)
            .filter(|block| block.memory.0 == memory)
            .flat_map(|block| &block.regions)
            .find_map(|region| match region.state {
                MemoryRegionState::Allocated { allocation_id, .. } if region.offset == offset => {
                    Some(allocation_id)
                }
                _ => None,
            })
            .ok_or_else(|| format!("the snapshot has no allocation at {offset} of memory {memory}"))
    }

    fn free(&mut self, pick: Pick) -> Checked {
        let Some(index) = self.pick_live(pick, |live| self.shares_its_block(live)) else {
            self.coverage.step("free: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let live = self.live.remove(index);
        let memory = live.memory();
        let identity = (
            live.allocation.memory(),
            live.allocation.offset(),
            live.allocation.size(),
            live.allocation.memory_type_index(),
        );
        let state = live.mapping_state();
        let was_mapped = before.memory_objects[&live.allocation.memory()].mapped;
        let Live {
            allocation,
            twin,
            requested_size,
            alignment,
            name,
            raw_mappings,
            persistent,
            allocation_id,
            fill,
            zero_initialized,
        } = live;
        let result = unsafe { self.allocator.free(allocation) };

        if raw_mappings > 0 {
            let FreeError { allocation, error } = match result {
                Err(error) => error,
                Ok(()) => return Err("a mapped allocation was freed".to_string()),
            };
            ensure_eq!(
                error,
                AllocationError::InvalidAllocation,
                "the error of freeing a mapped allocation"
            );
            ensure_eq!(
                (
                    allocation.memory(),
                    allocation.offset(),
                    allocation.size(),
                    allocation.memory_type_index()
                ),
                identity,
                "the allocation handed back by a refused free"
            );
            self.live.insert(
                index,
                Live {
                    allocation,
                    twin,
                    requested_size,
                    alignment,
                    name,
                    raw_mappings,
                    persistent,
                    allocation_id,
                    fill,
                    zero_initialized,
                },
            );
            self.coverage.step(format!("free refused: {state}"));
            return self.nothing_happened_since(&before, "a refused free");
        }

        ensure_eq!(result, Ok(()), "freeing an allocation that is {state}");
        let calls = self.fake.calls();
        let block_is_empty = !self.live.iter().any(|other| other.memory() == memory);
        if block_is_empty {
            self.blocks.remove(&memory);
            ensure_eq!(
                calls.free,
                before.calls.free + 1,
                "vkFreeMemory calls after the last allocation of a block was freed"
            );
            ensure!(
                calls.unmap - before.calls.unmap <= usize::from(was_mapped),
                "vkUnmapMemory calls while freeing a block: {} to {}",
                before.calls.unmap,
                calls.unmap
            );
            self.coverage
                .step(format!("free the last of a block: {state}"));
        } else {
            ensure_eq!(
                calls.free,
                before.calls.free,
                "vkFreeMemory calls while the block has other allocations"
            );
            let still_mapped = self.mapping_references_in(memory) > 0;
            ensure_eq!(
                calls.unmap - before.calls.unmap,
                usize::from(was_mapped && !still_mapped),
                "vkUnmapMemory calls after freeing an allocation that is {state}"
            );
            if was_mapped && !still_mapped {
                self.coverage
                    .note("free releases the last mapping of a block");
            }
            self.coverage.step(format!("free one of several: {state}"));
        }
        ensure_eq!(
            calls.allocate,
            before.calls.allocate,
            "vkAllocateMemory calls during a free"
        );
        ensure_eq!(
            calls.map,
            before.calls.map,
            "vkMapMemory calls during a free"
        );
        if let Some(twin) = twin {
            unsafe { self.twin.free(twin) }
                .map_err(|error| format!("the twin allocator refused the free: {error:?}"))?;
        }
        Ok(())
    }

    fn map_expectation(&mut self, index: usize) -> MapExpectation {
        let live = &self.live[index];
        if !self
            .flags_of(live.allocation.memory_type_index())
            .contains(VISIBLE)
        {
            return MapExpectation::NotHostVisible;
        }
        if self.mapping_references_in(live.memory()) > 0 {
            return MapExpectation::SharesTheMapping;
        }
        match self.map_fault.take() {
            Some(MapFault::Fails(result)) => MapExpectation::Fails(result),
            Some(MapFault::ReturnsNull) => MapExpectation::ReturnsNull,
            Some(MapFault::ReturnsAddress(_)) | None => MapExpectation::MapsTheBlock,
        }
    }

    fn check_refused_map(
        &self,
        expectation: MapExpectation,
        error: Option<AllocationError>,
        before: &Observation,
        what: &str,
    ) -> Result<bool, String> {
        let calls = self.fake.calls();
        let (expected, maps, unmaps) = match expectation {
            MapExpectation::NotHostVisible => (AllocationError::InvalidAllocation, 0, 0),
            MapExpectation::Fails(result) => (AllocationError::Vulkan(result), 1, 0),
            MapExpectation::ReturnsNull => (AllocationError::InvalidAllocation, 1, 1),
            MapExpectation::MapsTheBlock | MapExpectation::SharesTheMapping => return Ok(false),
        };
        ensure_eq!(error, Some(expected), "{what} with {expectation:?}");
        ensure_eq!(
            (
                calls.map - before.calls.map,
                calls.unmap - before.calls.unmap
            ),
            (maps, unmaps),
            "vkMapMemory and vkUnmapMemory calls of {what} with {expectation:?}"
        );
        self.state_is_as(before, what)?;
        Ok(true)
    }

    fn check_mapped(
        &self,
        index: usize,
        expectation: MapExpectation,
        pointer: usize,
        before: &Observation,
        what: &str,
    ) -> Checked {
        let live = &self.live[index];
        let calls = self.fake.calls();
        ensure_eq!(
            calls.map - before.calls.map,
            usize::from(expectation == MapExpectation::MapsTheBlock),
            "vkMapMemory calls of {what} with {expectation:?}"
        );
        ensure_eq!(
            calls.unmap,
            before.calls.unmap,
            "vkUnmapMemory calls of {what}"
        );
        let base = self
            .fake
            .base_address(live.allocation.memory())
            .ok_or("the device has not mapped the memory")?;
        ensure_eq!(
            pointer,
            base + live.allocation.offset() as usize,
            "the pointer of {what}"
        );
        Ok(())
    }

    fn map_memory(&mut self, pick: Pick) -> Checked {
        let Some(index) = self.pick_live(pick, |live| self.is_host_visible(live)) else {
            self.coverage.step("map_memory: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        let expectation = self.map_expectation(index);
        let result = unsafe { self.allocator.map_memory(&self.live[index].allocation) };
        self.coverage
            .step(format!("map_memory {expectation:?}: {state}"));
        if self.check_refused_map(expectation, result.clone().err(), &before, "map_memory")? {
            return Ok(());
        }
        let pointer = result.map_err(|error| format!("map_memory: {error:?}"))?;
        self.check_mapped(index, expectation, pointer as usize, &before, "map_memory")?;
        self.live[index].raw_mappings += 1;
        Ok(())
    }

    fn unmap_memory(&mut self, pick: Pick) -> Checked {
        let Some(index) = self.pick_live(pick, |live| live.raw_mappings > 0) else {
            self.coverage.step("unmap_memory: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        let result = unsafe { self.allocator.unmap_memory(&self.live[index].allocation) };
        if self.live[index].raw_mappings == 0 {
            ensure_eq!(
                result,
                Err(AllocationError::InvalidAllocation),
                "unmap_memory of an allocation that is {state}"
            );
            self.coverage.step(format!("unmap_memory refused: {state}"));
            return self.nothing_happened_since(&before, "a refused unmap_memory");
        }
        ensure_eq!(
            result,
            Ok(()),
            "unmap_memory of an allocation that is {state}"
        );
        self.live[index].raw_mappings -= 1;
        let unmapped = self.mapping_references_in(self.live[index].memory()) == 0;
        let calls = self.fake.calls();
        ensure_eq!(
            calls.unmap - before.calls.unmap,
            usize::from(unmapped),
            "vkUnmapMemory calls of unmap_memory"
        );
        ensure_eq!(
            calls.map,
            before.calls.map,
            "vkMapMemory calls of unmap_memory"
        );
        self.coverage.step(format!(
            "unmap_memory {}: {state}",
            if unmapped {
                "unmaps the block"
            } else {
                "keeps the block mapped"
            }
        ));
        Ok(())
    }

    fn map_persistent(&mut self, pick: Pick) -> Checked {
        let Some(index) = self.pick_live(pick, |live| self.is_host_visible(live)) else {
            self.coverage.step("map_persistent: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        if let Some(pointer) = self.live[index].persistent {
            let result = unsafe { self.allocator.map_persistent(&self.live[index].allocation) };
            ensure_eq!(
                result,
                Ok(pointer),
                "map_persistent of an allocation that is {state}"
            );
            self.coverage.step(format!("map_persistent again: {state}"));
            return self.nothing_happened_since(&before, "a repeated map_persistent");
        }
        let expectation = self.map_expectation(index);
        let result = unsafe { self.allocator.map_persistent(&self.live[index].allocation) };
        self.coverage
            .step(format!("map_persistent {expectation:?}: {state}"));
        if self.check_refused_map(expectation, result.clone().err(), &before, "map_persistent")? {
            return Ok(());
        }
        let pointer = result.map_err(|error| format!("map_persistent: {error:?}"))?;
        self.check_mapped(
            index,
            expectation,
            pointer.as_ptr() as usize,
            &before,
            "map_persistent",
        )?;
        self.live[index].persistent = Some(pointer);
        Ok(())
    }

    fn check_synchronized(
        &self,
        index: usize,
        kind: Synchronization,
        logged_before: usize,
        logged_at_least: usize,
        requested: (u64, u64),
    ) -> Checked {
        let live = &self.live[index];
        let logged = self.fake.synchronized();
        let block = self.blocks[&live.memory()];
        if self.flags_of(block.memory_type).contains(COHERENT) {
            ensure_eq!(
                logged.len(),
                logged_before,
                "ranges given to the device by {kind:?} of coherent memory"
            );
            return Ok(());
        }
        ensure!(
            logged.len() >= logged_before + logged_at_least && logged_at_least == 1,
            "no range was given to the device by {kind:?}"
        );
        let SynchronizedRange {
            kind: logged_kind,
            memory,
            offset,
            size,
        } = logged[logged_before];
        ensure_eq!(logged_kind, kind, "the command that was called");
        ensure_eq!(memory, live.allocation.memory(), "the memory of the range");
        let end = if size == vk::WHOLE_SIZE {
            block.size
        } else {
            offset + size
        };
        let (start_of_allocation, length) = live.range();
        let (requested_start, requested_end) = requested;
        ensure!(
            offset <= requested_start && requested_end <= end,
            "the range {offset}..{end} does not cover the bytes {requested_start}..{requested_end} \
             of the allocation at {start_of_allocation} of {length} bytes"
        );
        let atom = self.world.atom();
        let whole_start = start_of_allocation / atom * atom;
        let whole_end = (start_of_allocation + length)
            .div_ceil(atom)
            .checked_mul(atom)
            .map_or(block.size, |whole_end| whole_end.min(block.size));
        ensure!(
            whole_start <= offset && end <= whole_end,
            "the range {offset}..{end} leaves the range {whole_start}..{whole_end} of the whole \
             allocation at {start_of_allocation} of {length} bytes"
        );
        let widened_start = requested_start / atom * atom;
        let widened_end = requested_end
            .div_ceil(atom)
            .checked_mul(atom)
            .map_or(block.size, |widened_end| widened_end.min(block.size));
        ensure_eq!(
            (offset, end),
            (widened_start, widened_end),
            "the range given for the bytes {requested_start}..{requested_end}, widened to \
             multiples of {atom} within the block of {} bytes",
            block.size
        );
        for other in &self.live {
            let (other_start, other_length) = other.range();
            let apart = other.memory() != live.memory()
                || other_start == start_of_allocation
                || other_start + other_length <= offset
                || end <= other_start;
            ensure!(
                apart,
                "the range {offset}..{end} of the allocation at {start_of_allocation} reaches \
                 into the allocation at {other_start} of {other_length} bytes"
            );
        }
        Ok(())
    }

    fn synchronize(&mut self, pick: Pick, kind: Synchronization) -> Checked {
        let Some(index) = self.pick_live(pick, |live| live.mapping_references() > 0) else {
            self.coverage.step(format!("{kind:?}: nothing is live"));
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        let allocation = &self.live[index].allocation;
        let result = unsafe {
            match kind {
                Synchronization::Flush => self.allocator.flush_allocation(allocation),
                Synchronization::Invalidate => self.allocator.invalidate_allocation(allocation),
            }
        };
        if self.live[index].mapping_references() == 0 {
            ensure_eq!(
                result,
                Err(AllocationError::InvalidAllocation),
                "{kind:?} of an allocation that is not mapped"
            );
            self.coverage.step(format!("{kind:?} refused: {state}"));
            return self.nothing_happened_since(&before, "a refused flush or invalidate");
        }
        if self.is_coherent(&self.live[index]) {
            ensure_eq!(
                result,
                Ok(()),
                "{kind:?} of coherent memory that is {state}"
            );
            self.coverage.step(format!("{kind:?}: {state}"));
            self.coverage
                .note(format!("{kind:?} makes no Vulkan call for coherent memory"));
            return self
                .nothing_happened_since(&before, "a flush or invalidate of coherent memory");
        }
        let expected = match self.synchronization_fails.take() {
            Some(failure) => {
                self.coverage.step(format!("{kind:?} fails on the device"));
                self.coverage
                    .note(format!("{kind:?} fails on the device with {failure:?}"));
                Err(AllocationError::Vulkan(failure))
            }
            None => {
                self.coverage.step(format!("{kind:?}: {state}"));
                Ok(())
            }
        };
        ensure_eq!(
            result,
            expected,
            "{kind:?} of an allocation that is {state}"
        );
        let (start, length) = self.live[index].range();
        self.check_synchronized(index, kind, before.synchronized, 1, (start, start + length))?;
        ensure_eq!(
            self.fake.synchronized().len(),
            before.synchronized + 1,
            "ranges given to the device by one {kind:?}"
        );
        self.state_is_as(&before, "a flush or invalidate")
    }

    fn synchronize_range(&mut self, pick: Pick, kind: Synchronization, span: Span) -> Checked {
        let name = range_function(kind);
        let Some(index) = self.pick_live(pick, |live| live.mapping_references() > 0) else {
            self.coverage.step(format!("{name}: nothing is live"));
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        let (start_of_allocation, length) = self.live[index].range();
        let (offset, size) = span.resolve(length);
        let allocation = &self.live[index].allocation;
        let result = unsafe {
            match kind {
                Synchronization::Flush => self
                    .allocator
                    .flush_allocation_range(allocation, offset, size),
                Synchronization::Invalidate => self
                    .allocator
                    .invalidate_allocation_range(allocation, offset, size),
            }
        };
        let what = format!("{name}({offset}, {size}) of an allocation of {length} bytes");
        let inside = offset.checked_add(size).is_some_and(|end| end <= length);
        if self.live[index].mapping_references() == 0 || !inside {
            ensure_eq!(
                result,
                Err(AllocationError::InvalidAllocation),
                "{what} that is {state}"
            );
            self.coverage
                .step(if self.live[index].mapping_references() == 0 {
                    format!("{name} refused: {state}")
                } else {
                    format!("{name} outside the allocation")
                });
            return self
                .nothing_happened_since(&before, "a refused flush or invalidate of a range");
        }
        if size == 0 || self.is_coherent(&self.live[index]) {
            ensure_eq!(result, Ok(()), "{what} that is {state}");
            self.coverage.step(if size == 0 {
                format!("{name} of 0 bytes")
            } else {
                format!("{name} of coherent memory")
            });
            return self.nothing_happened_since(
                &before,
                "a flush or invalidate of an empty range or of coherent memory",
            );
        }
        let expected = match self.synchronization_fails.take() {
            Some(failure) => {
                self.coverage.step(format!("{name} fails on the device"));
                self.coverage
                    .note(format!("{name} fails on the device with {failure:?}"));
                Err(AllocationError::Vulkan(failure))
            }
            None => {
                self.coverage.step(format!("{name}: {state}"));
                Ok(())
            }
        };
        ensure_eq!(result, expected, "{what} that is {state}");
        let start = start_of_allocation + offset;
        self.check_synchronized(index, kind, before.synchronized, 1, (start, start + size))?;
        if size < length {
            self.coverage
                .note(format!("{name} of a part of the allocation"));
        }
        ensure_eq!(
            self.fake.synchronized().len(),
            before.synchronized + 1,
            "ranges given to the device by one {name}"
        );
        self.state_is_as(&before, "a flush or invalidate of a range")
    }

    fn guard(&mut self, pick: Pick, guard_use: GuardUse) -> Checked {
        let Some(index) = self.pick_live(pick, |live| self.is_host_visible(live)) else {
            self.coverage.step("map: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let state = self.live[index].mapping_state();
        let expectation = self.map_expectation(index);
        self.coverage.step(format!("map {expectation:?}: {state}"));
        let fill = (self.live[index].allocation_id % 251) as u8 + 1;
        let length = self.live[index].allocation.size() as usize;
        let untouched_and_zeroed =
            self.live[index].zero_initialized && self.live[index].fill.is_none();
        let coherent = self.is_coherent(&self.live[index]);

        let Self {
            allocator,
            live,
            fake,
            synchronization_fails,
            ..
        } = self;
        let used = unsafe { allocator.map(&live[index].allocation) }.map(
            |mut mapped| -> Result<GuardUsed, String> {
                let pointer = mapped.as_ptr() as usize;
                let bytes = unsafe { mapped.as_bytes_mut() };
                ensure_eq!(bytes.len(), length, "the length of the mapped bytes");
                ensure_eq!(
                    bytes.as_ptr() as usize,
                    pointer,
                    "the address of the mapped bytes"
                );
                ensure!(
                    !untouched_and_zeroed || bytes.iter().all(|&byte| byte == 0),
                    "a zero-initialized allocation holds bytes that are not 0 before its first write"
                );
                bytes.fill(fill);

                let (range_offset, range_size) = (
                    usize::try_from(guard_use.range_offset).unwrap_or(usize::MAX),
                    usize::try_from(guard_use.range_size).unwrap_or(usize::MAX),
                );
                let inside = range_offset
                    .checked_add(range_size)
                    .is_some_and(|end| end <= length);
                let expected = inside.then_some((pointer.wrapping_add(range_offset), range_size));
                ensure_eq!(
                    mapped
                        .byte_range(range_offset, range_size)
                        .map(|bytes| (bytes.as_ptr() as usize, bytes.len())),
                    expected,
                    "byte_range({range_offset}, {range_size}) of {length} bytes"
                );
                ensure_eq!(
                    mapped
                        .byte_range_mut(range_offset, range_size)
                        .map(|bytes| (bytes.as_ptr() as usize, bytes.len())),
                    expected,
                    "byte_range_mut({range_offset}, {range_size}) of {length} bytes"
                );

                let mut synchronized = Vec::new();
                for (wanted, kind) in [
                    (guard_use.flush, Synchronization::Flush),
                    (guard_use.invalidate, Synchronization::Invalidate),
                ] {
                    if !wanted {
                        continue;
                    }
                    let logged_before = fake.synchronized().len();
                    let result = unsafe {
                        match kind {
                            Synchronization::Flush => mapped.flush(),
                            Synchronization::Invalidate => mapped.invalidate(),
                        }
                    };
                    let expected = match synchronization_fails.take_if(|_| !coherent) {
                        Some(failure) => Err(AllocationError::Vulkan(failure)),
                        None => Ok(()),
                    };
                    ensure_eq!(result, expected, "{kind:?} through the guard");
                    synchronized.push((kind, logged_before));
                }

                let mut ranges = Vec::new();
                for (wanted, kind) in [
                    (guard_use.flush_range, Synchronization::Flush),
                    (guard_use.invalidate_range, Synchronization::Invalidate),
                ] {
                    if !wanted {
                        continue;
                    }
                    let name = guard_range_function(kind);
                    let logged_before = fake.synchronized().len();
                    let result = unsafe {
                        match kind {
                            Synchronization::Flush => mapped.flush_range(range_offset, range_size),
                            Synchronization::Invalidate => {
                                mapped.invalidate_range(range_offset, range_size)
                            }
                        }
                    };
                    let calls_the_device = inside && range_size > 0 && !coherent;
                    let expected = if inside {
                        match synchronization_fails.take_if(|_| calls_the_device) {
                            Some(failure) => Err(AllocationError::Vulkan(failure)),
                            None => Ok(()),
                        }
                    } else {
                        Err(AllocationError::InvalidAllocation)
                    };
                    ensure_eq!(
                        result,
                        expected,
                        "{name}({range_offset}, {range_size}) of {length} bytes through the guard"
                    );
                    ensure_eq!(
                        fake.synchronized().len(),
                        logged_before + usize::from(calls_the_device),
                        "ranges given to the device by {name}({range_offset}, {range_size}) of \
                         {length} bytes through the guard"
                    );
                    let event = if !inside {
                        format!("{name} through the guard outside the allocation")
                    } else if range_size == 0 {
                        format!("{name} through the guard of 0 bytes")
                    } else if coherent {
                        format!("{name} through the guard of coherent memory")
                    } else {
                        format!("{name} through the guard")
                    };
                    ranges.push(GuardRange {
                        kind,
                        logged_before,
                        requested: calls_the_device
                            .then_some((range_offset as u64, range_size as u64)),
                        event,
                    });
                }
                Ok((pointer, synchronized, ranges))
            },
        );
        let (pointer, synchronized, ranges) = match used {
            Ok(used) => used?,
            Err(error) => {
                let refusal =
                    self.check_refused_map(expectation, Some(error.clone()), &before, "map")?;
                ensure!(refusal, "map with {expectation:?}: {error:?}");
                return Ok(());
            }
        };
        ensure!(
            !matches!(
                expectation,
                MapExpectation::NotHostVisible
                    | MapExpectation::Fails(_)
                    | MapExpectation::ReturnsNull
            ),
            "map succeeded with {expectation:?}"
        );

        let calls = self.fake.calls();
        ensure_eq!(
            calls.map - before.calls.map,
            usize::from(expectation == MapExpectation::MapsTheBlock),
            "vkMapMemory calls of map with {expectation:?}"
        );
        let unmapped = self.mapping_references_in(self.live[index].memory()) == 0;
        ensure_eq!(
            calls.unmap - before.calls.unmap,
            usize::from(unmapped),
            "vkUnmapMemory calls after the guard of an allocation that is {state} was dropped"
        );
        let base = self
            .fake
            .base_address(self.live[index].allocation.memory())
            .ok_or("the device has not mapped the memory")?;
        ensure_eq!(
            pointer,
            base + self.live[index].allocation.offset() as usize,
            "the pointer of the guard"
        );
        let (start, length) = self.live[index].range();
        for (kind, logged_before) in synchronized {
            self.check_synchronized(index, kind, logged_before, 1, (start, start + length))?;
            self.coverage.note(if coherent {
                format!("{kind:?} through the guard makes no Vulkan call for coherent memory")
            } else {
                format!("{kind:?} through the guard")
            });
        }
        for range in ranges {
            if let Some((offset, size)) = range.requested {
                self.check_synchronized(
                    index,
                    range.kind,
                    range.logged_before,
                    1,
                    (start + offset, start + offset + size),
                )?;
            }
            self.coverage.note(range.event);
        }
        self.live[index].fill = Some(fill);
        self.state_is_as(&before, "a dropped guard")
    }

    fn free_all(&mut self) -> Checked {
        let before = self.observe();
        let mapped_blocks = before
            .memory_objects
            .values()
            .filter(|memory| memory.mapped)
            .count();
        let freed = unsafe { self.allocator.free_all() };
        ensure_eq!(freed, self.live.len(), "the count free_all returns");
        let calls = self.fake.calls();
        ensure_eq!(
            calls.free - before.calls.free,
            self.blocks.len(),
            "vkFreeMemory calls of free_all"
        );
        ensure_eq!(
            self.fake.memory_objects(),
            HashMap::new(),
            "memory left on the device after free_all"
        );
        self.coverage.step(match (self.live.len(), mapped_blocks) {
            (0, _) => "free_all: nothing is live",
            (_, 0) => "free_all: nothing is mapped",
            _ => "free_all: with mapped blocks",
        });
        self.blocks.clear();
        self.stale
            .extend(self.live.drain(..).map(|live| live.allocation));
        unsafe { self.twin.free_all() };
        Ok(())
    }

    fn use_stale(&mut self, pick: Pick, handle_use: HandleUse) -> Checked {
        let Some(index) = pick.among(self.stale.len()) else {
            self.coverage.step("stale handle: none exists");
            return Ok(());
        };
        let before = self.observe();
        let stale = self.stale.remove(index);
        let aliases_live = self.live.iter().any(|live| {
            live.allocation.memory() == stale.memory() && live.allocation.offset() == stale.offset()
        });
        let stale = refuse(&mut self.allocator, stale, handle_use)?;
        self.stale.insert(index, stale);
        self.coverage.step(format!("stale handle: {handle_use:?}"));
        if aliases_live {
            self.coverage
                .note("stale handle at the place of a live allocation");
        }
        self.nothing_happened_since(&before, "the use of a stale allocation")
    }

    fn use_foreign(&mut self, pick: Pick, handle_use: HandleUse) -> Checked {
        let Some(index) = pick.among(self.live.len()) else {
            self.coverage.step("foreign handle: nothing is live");
            return Ok(());
        };
        let before = self.observe();
        let foreign = self.live[index]
            .twin
            .take()
            .ok_or("the live allocation has no twin")?;
        let foreign = refuse(&mut self.allocator, foreign, handle_use)?;
        self.live[index].twin = Some(foreign);
        self.coverage
            .step(format!("foreign handle: {handle_use:?}"));
        self.nothing_happened_since(&before, "the use of an allocation of another allocator")
    }

    fn check(&self) -> Checked {
        ensure_eq!(
            self.fake.misuse(),
            Vec::<String>::new(),
            "invalid calls to the device"
        );
        ensure_eq!(
            self.twin_fake.misuse(),
            Vec::<String>::new(),
            "invalid calls to the twin device"
        );

        let mut expected_objects = HashMap::new();
        for (&memory, block) in &self.blocks {
            expected_objects.insert(
                vk::DeviceMemory(memory),
                MemoryObject {
                    size: block.size,
                    memory_type_index: block.memory_type,
                    mapped: self.mapping_references_in(memory) > 0,
                },
            );
        }
        ensure_eq!(
            self.fake.memory_objects(),
            expected_objects,
            "the memory the device holds"
        );

        let snapshot = self.allocator.snapshot();
        let stats = self.allocator.stats();
        ensure_eq!(snapshot.stats, stats, "the stats inside the snapshot");

        let mut expected_live = Vec::new();
        let mut seen_blocks = BTreeSet::new();
        let mut seen_ids = BTreeSet::new();
        let mut totals = (0u64, 0u64, 0u64, 0usize, 0usize);
        let mut previous_type = None;
        ensure_eq!(
            snapshot.memory_types.len(),
            stats.memory_types.len(),
            "memory types in the snapshot and in the stats"
        );
        for (memory_type, type_stats) in snapshot.memory_types.iter().zip(&stats.memory_types) {
            let index = memory_type.memory_type_index;
            ensure!(
                previous_type.is_none_or(|previous| previous < index),
                "memory types are not in increasing order"
            );
            previous_type = Some(index);
            ensure_eq!(
                memory_type.stats,
                *type_stats,
                "the stats of memory type {index}"
            );
            ensure_eq!(
                type_stats.memory_type_index,
                index,
                "the index in the stats of memory type {index}"
            );
            ensure_eq!(
                Some(memory_type.property_flags),
                self.world.memory_types.get(index as usize).map(|t| t.flags),
                "the property flags of memory type {index}"
            );
            ensure_eq!(
                self.allocator.memory_type_flags(index),
                Some(memory_type.property_flags),
                "memory_type_flags({index})"
            );
            ensure!(
                !memory_type.blocks.is_empty(),
                "memory type {index} is listed without blocks"
            );
            let mut type_totals = (0u64, 0u64, 0u64, 0usize, 0usize);
            let mut previous_block = None;
            for block in &memory_type.blocks {
                let memory = block.memory.0;
                ensure!(
                    previous_block.is_none_or(|previous| previous < block.block_index),
                    "blocks of memory type {index} are not in increasing block_index order"
                );
                previous_block = Some(block.block_index);
                let Some(model) = self.blocks.get(&memory) else {
                    return Err(format!(
                        "the snapshot has an unknown block {:?}",
                        block.memory
                    ));
                };
                ensure!(
                    seen_blocks.insert(memory),
                    "{:?} is listed twice",
                    block.memory
                );
                ensure_eq!(
                    (
                        block.size,
                        index,
                        block.dedicated,
                        block.block_index,
                        block.resource_class
                    ),
                    (
                        model.size,
                        model.memory_type,
                        model.own,
                        model.block_index,
                        if model.linear {
                            AllocationResourceClass::Linear
                        } else {
                            AllocationResourceClass::Optimal
                        }
                    ),
                    "block {:?}",
                    block.memory
                );

                let mut end = 0;
                let mut previous_free = false;
                let mut allocated = Vec::new();
                let mut free = Vec::new();
                for region in &block.regions {
                    ensure_eq!(
                        region.offset,
                        end,
                        "the start of a region of {:?}",
                        block.memory
                    );
                    ensure!(region.size > 0, "an empty region in {:?}", block.memory);
                    end = region.offset + region.size;
                    match region.state {
                        MemoryRegionState::Free => {
                            ensure!(
                                !previous_free,
                                "two free regions are adjacent in {:?}",
                                block.memory
                            );
                            previous_free = true;
                            free.push(region.size);
                        }
                        MemoryRegionState::Allocated {
                            allocation_id,
                            mapped_count,
                        } => {
                            previous_free = false;
                            ensure!(
                                seen_ids.insert(allocation_id),
                                "allocation id {allocation_id} appears twice"
                            );
                            allocated.push((
                                region.offset,
                                region.size,
                                allocation_id,
                                mapped_count,
                            ));
                        }
                    }
                }
                ensure_eq!(
                    end,
                    block.size,
                    "the end of the regions of {:?}",
                    block.memory
                );

                let mut expected_allocated = self
                    .live
                    .iter()
                    .filter(|live| live.memory() == memory)
                    .map(|live| {
                        (
                            live.allocation.offset(),
                            live.allocation.size(),
                            live.allocation_id,
                            live.mapping_references(),
                        )
                    })
                    .collect::<Vec<_>>();
                expected_allocated.sort_unstable();
                ensure_eq!(
                    allocated,
                    expected_allocated,
                    "the allocated regions of {:?}",
                    block.memory
                );
                ensure!(
                    !allocated.is_empty(),
                    "{:?} is kept without allocations",
                    block.memory
                );
                let allocated_bytes = allocated.iter().map(|region| region.1).sum::<u64>();
                ensure_eq!(
                    (
                        block.allocated_bytes,
                        block.free_bytes,
                        block.allocated_region_count,
                        block.free_region_count,
                        block.largest_free_region_bytes
                    ),
                    (
                        allocated_bytes,
                        block.size - allocated_bytes,
                        allocated.len(),
                        free.len(),
                        free.iter().copied().max().unwrap_or(0)
                    ),
                    "the totals of {:?}",
                    block.memory
                );
                type_totals.0 += block.size;
                type_totals.1 += allocated_bytes;
                type_totals.2 += block.size - allocated_bytes;
                type_totals.3 += allocated.len();
                type_totals.4 += 1;

                let mut in_block = self
                    .live
                    .iter()
                    .filter(|live| live.memory() == memory)
                    .collect::<Vec<_>>();
                in_block.sort_unstable_by_key(|live| live.allocation.offset());
                expected_live.extend(in_block.into_iter().map(|live| LiveAllocation {
                    name: live.name.clone(),
                    memory: live.allocation.memory(),
                    offset: live.allocation.offset(),
                    size: live.allocation.size(),
                    memory_type_index: index,
                }));
            }
            ensure_eq!(
                (
                    type_stats.total_block_bytes,
                    type_stats.allocated_bytes,
                    type_stats.free_bytes,
                    type_stats.allocation_count,
                    type_stats.block_count
                ),
                type_totals,
                "the totals of memory type {index}"
            );
            totals.0 += type_totals.0;
            totals.1 += type_totals.1;
            totals.2 += type_totals.2;
            totals.3 += type_totals.3;
            totals.4 += type_totals.4;
        }
        ensure_eq!(
            seen_blocks.len(),
            self.blocks.len(),
            "blocks in the snapshot"
        );
        ensure_eq!(
            (
                stats.total_block_bytes,
                stats.allocated_bytes,
                stats.free_bytes,
                stats.allocation_count,
                stats.block_count
            ),
            totals,
            "the totals of the allocator"
        );
        ensure_eq!(
            self.allocator.live_allocations(),
            expected_live,
            "live_allocations()"
        );
        ensure_eq!(
            self.allocator.live_allocation_count(),
            self.live.len(),
            "live_allocation_count()"
        );
        ensure_eq!(
            self.allocator.is_empty(),
            self.live.is_empty(),
            "is_empty()"
        );

        for live in &self.live {
            let flags = self.flags_of(live.allocation.memory_type_index());
            ensure_eq!(
                self.allocator.allocation_memory_flags(&live.allocation),
                Ok(flags),
                "allocation_memory_flags()"
            );
            ensure_eq!(
                self.allocator.persistent_ptr(&live.allocation),
                live.persistent,
                "persistent_ptr()"
            );
            if let Some(pointer) = live.persistent {
                let base = self
                    .fake
                    .base_address(live.allocation.memory())
                    .ok_or("the device has not mapped persistently mapped memory")?;
                ensure_eq!(
                    pointer.as_ptr() as usize,
                    base + live.allocation.offset() as usize,
                    "the persistent pointer"
                );
            }
            if let Some(fill) = live.fill {
                let (offset, size) = live.range();
                let bytes = self
                    .fake
                    .bytes(live.allocation.memory(), offset, size)
                    .ok_or("the device lost the bytes of an allocation")?;
                ensure!(
                    bytes.iter().all(|&byte| byte == fill),
                    "the {size} bytes at {offset} of {:?} were overwritten",
                    live.allocation.memory()
                );
            }
        }

        let layout = |snapshot: &AllocatorSnapshot| {
            snapshot
                .memory_types
                .iter()
                .flat_map(|memory_type| &memory_type.blocks)
                .map(|block| {
                    (
                        block.memory,
                        block.block_index,
                        block.size,
                        block
                            .regions
                            .iter()
                            .map(|region| {
                                (
                                    region.offset,
                                    region.size,
                                    region.state == MemoryRegionState::Free,
                                )
                            })
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        };
        ensure_eq!(
            layout(&self.twin.snapshot()),
            layout(&snapshot),
            "the layout of an allocator given the same requests"
        );
        Ok(())
    }
}

fn create_desc<'a>(
    request: &Request,
    name: Option<&'a str>,
    resource: u64,
) -> AllocationCreateDesc<'a> {
    AllocationCreateDesc {
        name,
        requirements: vk::MemoryRequirements {
            size: request.size,
            alignment: request.alignment,
            memory_type_bits: request.memory_type_bits,
        },
        location: request.location.memory_location(),
        linear: request.linear,
        memory_allocate_flags: request.allocate_flags(),
        export_handle_types: request.export_handle_types(),
        allocation_scheme: match request.scheme {
            Scheme::Managed => AllocationScheme::GpuAllocatorManaged,
            Scheme::DedicatedBuffer => AllocationScheme::DedicatedBuffer(vk::Buffer(resource)),
            Scheme::DedicatedImage => AllocationScheme::DedicatedImage(vk::Image(resource)),
        },
    }
}

fn refuse(
    allocator: &mut Allocator,
    allocation: Allocation,
    handle_use: HandleUse,
) -> Result<Allocation, String> {
    let invalid = Some(AllocationError::InvalidAllocation);
    unsafe {
        let error = match handle_use {
            HandleUse::Free => {
                let identity = (allocation.memory(), allocation.offset(), allocation.size());
                return match allocator.free(allocation) {
                    Err(FreeError {
                        allocation,
                        error: AllocationError::InvalidAllocation,
                    }) if (allocation.memory(), allocation.offset(), allocation.size())
                        == identity =>
                    {
                        Ok(allocation)
                    }
                    other => Err(format!("free of an allocation that is not held: {other:?}")),
                };
            }
            HandleUse::MapMemory => allocator.map_memory(&allocation).err(),
            HandleUse::Guard => allocator.map(&allocation).err(),
            HandleUse::MapPersistent => allocator.map_persistent(&allocation).err(),
            HandleUse::PersistentPtr => {
                ensure_eq!(
                    allocator.persistent_ptr(&allocation),
                    None,
                    "persistent_ptr of an allocation that is not held"
                );
                invalid.clone()
            }
            HandleUse::UnmapMemory => allocator.unmap_memory(&allocation).err(),
            HandleUse::Flush => allocator.flush_allocation(&allocation).err(),
            HandleUse::Invalidate => allocator.invalidate_allocation(&allocation).err(),
            HandleUse::FlushRange => allocator
                .flush_allocation_range(&allocation, 0, allocation.size())
                .err(),
            HandleUse::InvalidateRange => allocator
                .invalidate_allocation_range(&allocation, 0, allocation.size())
                .err(),
            HandleUse::MemoryFlags => allocator.allocation_memory_flags(&allocation).err(),
        };
        ensure_eq!(
            error,
            invalid,
            "{handle_use:?} of an allocation that is not held"
        );
    }
    Ok(allocation)
}
