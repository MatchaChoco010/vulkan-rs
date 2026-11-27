mod fake_device;

use std::collections::{HashMap, HashSet};
use std::ffi::c_void;
use std::ptr;

use fake_device::{Dedicated, FakeDevice, HOST_COHERENT, MapFault, ResourceKind, ResourceSetup};
use proptest::prelude::*;
use proptest::sample::Index;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use vulkan::vk;
use vulkan_alloc::{
    Allocation, AllocationError, AllocationResourceClass, Allocator, AllocatorCreateDesc,
    MemoryLocation, ResourceAllocationDesc,
};

const LONGEST_SEQUENCE: usize = 12;

macro_rules! ensure {
    ($condition:expr, $($message:tt)*) => {
        if !$condition {
            return Err(TestCaseError::fail(format!($($message)*)));
        }
    };
}

#[repr(C)]
struct ExternalFormat {
    s_type: vk::StructureType,
    p_next: *mut c_void,
    external_format: u64,
}

enum Node {
    Usage2(vk::BufferUsageFlags2CreateInfo<'static>),
    ExternalBuffer(vk::ExternalMemoryBufferCreateInfo<'static>),
    DedicatedBufferNv(vk::DedicatedAllocationBufferCreateInfoNV<'static>),
    Flags2(vk::ImageCreateFlags2CreateInfoKHR<'static>),
    ExternalImage(vk::ExternalMemoryImageCreateInfo<'static>),
    ExternalImageNv(vk::ExternalMemoryImageCreateInfoNV<'static>),
    DedicatedImageNv(vk::DedicatedAllocationImageCreateInfoNV<'static>),
    Swapchain(vk::ImageSwapchainCreateInfoKHR<'static>),
    ExternalFormat(ExternalFormat),
    Tagged(vk::BaseInStructure<'static>),
}

impl Node {
    fn base(&mut self) -> *mut vk::BaseOutStructure<'static> {
        match self {
            Node::Usage2(s) => ptr::from_mut(s).cast(),
            Node::ExternalBuffer(s) => ptr::from_mut(s).cast(),
            Node::DedicatedBufferNv(s) => ptr::from_mut(s).cast(),
            Node::Flags2(s) => ptr::from_mut(s).cast(),
            Node::ExternalImage(s) => ptr::from_mut(s).cast(),
            Node::ExternalImageNv(s) => ptr::from_mut(s).cast(),
            Node::DedicatedImageNv(s) => ptr::from_mut(s).cast(),
            Node::Swapchain(s) => ptr::from_mut(s).cast(),
            Node::ExternalFormat(s) => ptr::from_mut(s).cast(),
            Node::Tagged(s) => ptr::from_mut(s).cast(),
        }
    }
}

struct Chain(Vec<Node>);

impl Chain {
    fn link(&mut self) -> *const c_void {
        let mut head: *mut vk::BaseOutStructure<'static> = ptr::null_mut();
        for node in self.0.iter_mut().rev() {
            let base = node.base();
            unsafe { (*base).p_next = head };
            head = base;
        }
        head.cast_const().cast()
    }
}

fn tagged(s_type: vk::StructureType) -> Node {
    Node::Tagged(vk::BaseInStructure {
        s_type,
        p_next: ptr::null(),
        _marker: std::marker::PhantomData,
    })
}

#[derive(Debug, Clone)]
struct BufferMake {
    flags: vk::BufferCreateFlags,
    usage: vk::BufferUsageFlags,
    usage2: Option<vk::BufferUsageFlags2>,
    external: Option<vk::ExternalMemoryHandleTypeFlags>,
    dedicated_nv: Option<bool>,
    buffer_collection: bool,
}

#[derive(Debug, Clone)]
struct ImageMake {
    flags: vk::ImageCreateFlags,
    flags2: Option<vk::ImageCreateFlags2KHR>,
    tiling: vk::ImageTiling,
    external: Option<vk::ExternalMemoryHandleTypeFlags>,
    external_nv: Option<bool>,
    dedicated_nv: Option<bool>,
    swapchain: Option<bool>,
    external_format: Option<(vk::StructureType, u64)>,
    imported_from: Option<vk::StructureType>,
}

#[derive(Debug, Clone)]
enum Make {
    Buffer(BufferMake),
    Image(ImageMake),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fault {
    None,
    Creation(vk::VkResult),
    ModifierQuery(vk::VkResult),
    Allocation(vk::VkResult),
    Map(vk::VkResult),
    Bind(vk::VkResult),
}

#[derive(Debug, Clone)]
struct Create {
    make: Make,
    setup: ResourceSetup,
    location: MemoryLocation,
    mapped: bool,
    memory_allocate_flags: vk::MemoryAllocateFlags,
    alignment: u64,
    named: bool,
    fault: Fault,
}

#[derive(Debug, Clone)]
enum Op {
    Create(Create),
    Destroy { target: Index, mapping_held: bool },
}

fn flags_of<T: Copy + std::fmt::Debug + std::ops::BitOr<Output = T> + 'static>(
    empty: T,
    choices: Vec<T>,
) -> impl Strategy<Value = T> {
    prop_oneof![
        8 => Just(empty),
        1 => prop::sample::subsequence(choices.clone(), 0..=choices.len()).prop_map(move |chosen| {
            chosen.into_iter().fold(empty, |flags, flag| flags | flag)
        }),
    ]
}

fn handle_types() -> impl Strategy<Value = vk::ExternalMemoryHandleTypeFlags> {
    prop::sample::select(vec![
        vk::ExternalMemoryHandleTypeFlags::empty(),
        vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD,
        vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT,
        vk::ExternalMemoryHandleTypeFlags::OPAQUE_FD
            | vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT,
    ])
}

fn sometimes<T: std::fmt::Debug + Clone>(
    value: impl Strategy<Value = T>,
) -> impl Strategy<Value = Option<T>> {
    prop::option::weighted(0.04, value)
}

fn buffer_make() -> impl Strategy<Value = BufferMake> {
    (
        flags_of(
            vk::BufferCreateFlags::empty(),
            vec![
                vk::BufferCreateFlags::SPARSE_BINDING,
                vk::BufferCreateFlags::SPARSE_RESIDENCY,
                vk::BufferCreateFlags::SPARSE_ALIASED,
                vk::BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY,
                vk::BufferCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT,
                vk::BufferCreateFlags::VIDEO_PROFILE_INDEPENDENT_KHR,
            ],
        ),
        prop::sample::select(vec![
            vk::BufferUsageFlags::TRANSFER_SRC,
            vk::BufferUsageFlags::UNIFORM_BUFFER | vk::BufferUsageFlags::STORAGE_BUFFER,
            vk::BufferUsageFlags::VERTEX_BUFFER,
            vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS | vk::BufferUsageFlags::STORAGE_BUFFER,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT,
        ]),
        prop::option::weighted(
            0.3,
            prop::sample::select(vec![
                vk::BufferUsageFlags2::TRANSFER_DST,
                vk::BufferUsageFlags2::STORAGE_BUFFER,
                vk::BufferUsageFlags2::SHADER_DEVICE_ADDRESS,
                vk::BufferUsageFlags2::DESCRIPTOR_HEAP_EXT,
                vk::BufferUsageFlags2::MEMORY_DECOMPRESSION_EXT,
            ]),
        ),
        prop::option::weighted(0.15, handle_types()),
        sometimes(any::<bool>()),
        prop::bool::weighted(0.03),
    )
        .prop_map(
            |(flags, usage, usage2, external, dedicated_nv, buffer_collection)| BufferMake {
                flags,
                usage,
                usage2,
                external,
                dedicated_nv,
                buffer_collection,
            },
        )
}

fn image_flags() -> Vec<vk::ImageCreateFlags> {
    vec![
        vk::ImageCreateFlags::SPARSE_BINDING,
        vk::ImageCreateFlags::SPARSE_RESIDENCY,
        vk::ImageCreateFlags::DISJOINT,
        vk::ImageCreateFlags::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT,
        vk::ImageCreateFlags::MUTABLE_FORMAT,
        vk::ImageCreateFlags::ALIAS,
    ]
}

fn image_make() -> impl Strategy<Value = ImageMake> {
    (
        flags_of(vk::ImageCreateFlags::empty(), image_flags()),
        prop::option::weighted(
            0.2,
            flags_of(
                vk::ImageCreateFlags2KHR::empty(),
                image_flags()
                    .into_iter()
                    .map(|flag| vk::ImageCreateFlags2KHR::from_raw(u64::from(flag.as_raw())))
                    .collect(),
            ),
        ),
        prop::sample::select(vec![
            vk::ImageTiling::OPTIMAL,
            vk::ImageTiling::LINEAR,
            vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT,
        ]),
        prop::option::weighted(
            0.15,
            prop_oneof![
                4 => handle_types(),
                1 => Just(vk::ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID),
            ],
        ),
        sometimes(any::<bool>()),
        sometimes(any::<bool>()),
        sometimes(any::<bool>()),
        sometimes((
            prop::sample::select(vec![
                vk::StructureType::EXTERNAL_FORMAT_ANDROID,
                vk::StructureType::EXTERNAL_FORMAT_QNX,
                vk::StructureType::EXTERNAL_FORMAT_OHOS,
            ]),
            prop::sample::select(vec![0, 7]),
        )),
        prop::option::weighted(
            0.03,
            prop::sample::select(vec![
                vk::StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA,
                vk::StructureType::IMPORT_METAL_TEXTURE_INFO_EXT,
                vk::StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT,
            ]),
        ),
    )
        .prop_map(
            |(
                flags,
                flags2,
                tiling,
                external,
                external_nv,
                dedicated_nv,
                swapchain,
                external_format,
                imported_from,
            )| ImageMake {
                flags,
                flags2,
                tiling,
                external,
                external_nv,
                dedicated_nv,
                swapchain,
                external_format,
                imported_from,
            },
        )
}

fn setup() -> impl Strategy<Value = ResourceSetup> {
    (
        prop_oneof![
            3 => 1..=fake_device::BLOCK_SIZE / 8,
            1 => 1..=3 * fake_device::BLOCK_SIZE,
        ],
        prop::sample::select(vec![1u64, 2, 16, 16, 256, 1024, 4096]),
        prop_oneof![2 => Just(0b111u32), 1 => 1..=0b111u32],
        prop::bool::weighted(0.1),
        prop::bool::weighted(0.05),
        prop::sample::select(vec![0u64, 1, 0x0100_0000_0000_0001]),
    )
        .prop_map(
            |(size, alignment, memory_type_bits, prefers, requires, modifier)| ResourceSetup {
                size,
                alignment,
                memory_type_bits,
                prefers_dedicated: prefers,
                requires_dedicated: requires,
                drm_format_modifier: modifier,
            },
        )
}

fn host_locations() -> Vec<MemoryLocation> {
    vec![MemoryLocation::CpuToGpu, MemoryLocation::GpuToCpu]
}

fn location() -> impl Strategy<Value = MemoryLocation> {
    let mut locations = host_locations();
    locations.extend([
        MemoryLocation::GpuOnly,
        MemoryLocation::custom(
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::empty(),
        ),
        MemoryLocation::custom(
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_CACHED,
        ),
    ]);
    prop::sample::select(locations)
}

fn failure() -> impl Strategy<Value = vk::VkResult> {
    prop::sample::select(vec![
        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
        vk::VkResult::ERROR_UNKNOWN,
        vk::VkResult::ERROR_TOO_MANY_OBJECTS,
    ])
}

fn fault() -> impl Strategy<Value = Fault> {
    prop_oneof![
        10 => Just(Fault::None),
        1 => failure().prop_map(Fault::Creation),
        1 => failure().prop_map(Fault::ModifierQuery),
        1 => failure().prop_map(Fault::Allocation),
        1 => failure().prop_map(Fault::Map),
        1 => failure().prop_map(Fault::Bind),
    ]
}

fn create() -> impl Strategy<Value = Create> {
    (
        prop_oneof![
            buffer_make().prop_map(Make::Buffer),
            image_make().prop_map(Make::Image),
        ],
        setup(),
        location(),
        any::<bool>(),
        prop::sample::select(vec![
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT,
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::DEVICE_MASK,
        ]),
        prop::sample::select(vec![0u64, 1, 1, 1, 1, 2, 64, 256, 8192, 3, 48]),
        any::<bool>(),
        fault(),
        prop::sample::select(host_locations()),
    )
        .prop_map(
            |(
                make,
                mut setup,
                mut location,
                mut mapped,
                memory_allocate_flags,
                alignment,
                named,
                fault,
                host_location,
            )| {
                if matches!(fault, Fault::Allocation(_) | Fault::Map(_)) {
                    setup.requires_dedicated = true;
                }
                if matches!(fault, Fault::Map(_)) {
                    location = host_location;
                    mapped = true;
                }
                Create {
                    make,
                    setup,
                    location,
                    mapped,
                    memory_allocate_flags,
                    alignment,
                    named,
                    fault,
                }
            },
        )
}

fn operations() -> impl Strategy<Value = (Vec<Op>, vk::MemoryAllocateFlags)> {
    (
        prop::collection::vec(
            prop_oneof![
                3 => create().prop_map(Op::Create),
                1 => (any::<Index>(), prop::bool::weighted(0.3))
                    .prop_map(|(target, mapping_held)| Op::Destroy { target, mapping_held }),
            ],
            1..=LONGEST_SEQUENCE,
        ),
        prop::sample::select(vec![
            vk::MemoryAllocateFlags::empty(),
            vk::MemoryAllocateFlags::DEVICE_ADDRESS,
        ]),
    )
}

struct Expected {
    refused: bool,
    linear: bool,
    export: vk::ExternalMemoryHandleTypeFlags,
    device_address: bool,
}

fn expected_of(make: &Make, setup: &ResourceSetup) -> Expected {
    match make {
        Make::Buffer(buffer) => {
            let refused_flags = vk::BufferCreateFlags::SPARSE_BINDING
                | vk::BufferCreateFlags::SPARSE_RESIDENCY
                | vk::BufferCreateFlags::SPARSE_ALIASED
                | vk::BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY
                | vk::BufferCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT;
            let device_address = match buffer.usage2 {
                Some(usage) => usage.intersects(
                    vk::BufferUsageFlags2::SHADER_DEVICE_ADDRESS
                        | vk::BufferUsageFlags2::DESCRIPTOR_HEAP_EXT,
                ),
                None => buffer.usage.intersects(
                    vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
                        | vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT,
                ),
            };
            Expected {
                refused: buffer.flags.intersects(refused_flags)
                    || buffer.dedicated_nv == Some(true)
                    || buffer.buffer_collection,
                linear: true,
                export: buffer.external.unwrap_or_default(),
                device_address,
            }
        }
        Make::Image(image) => {
            let refused_flags = vk::ImageCreateFlags::SPARSE_BINDING
                | vk::ImageCreateFlags::SPARSE_RESIDENCY
                | vk::ImageCreateFlags::SPARSE_ALIASED
                | vk::ImageCreateFlags::DISJOINT
                | vk::ImageCreateFlags::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT;
            let flags_refused = match image.flags2 {
                Some(flags) => flags.intersects(vk::ImageCreateFlags2KHR::from_raw(u64::from(
                    refused_flags.as_raw(),
                ))),
                None => image.flags.intersects(refused_flags),
            };
            let export = image.external.unwrap_or_default();
            Expected {
                refused: flags_refused
                    || export.contains(
                        vk::ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID,
                    )
                    || image.external_nv == Some(true)
                    || image.dedicated_nv == Some(true)
                    || image.swapchain == Some(true)
                    || image.external_format.is_some_and(|(_, format)| format != 0)
                    || image.imported_from.is_some(),
                linear: image.tiling == vk::ImageTiling::LINEAR
                    || (image.tiling == vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT
                        && setup.drm_format_modifier == 0),
                export,
                device_address: false,
            }
        }
    }
}

fn buffer_chain(buffer: &BufferMake) -> Chain {
    let mut nodes = Vec::new();
    if let Some(usage) = buffer.usage2 {
        nodes.push(Node::Usage2(
            vk::BufferUsageFlags2CreateInfo::default().usage(usage),
        ));
    }
    if let Some(handle_types) = buffer.external {
        nodes.push(Node::ExternalBuffer(
            vk::ExternalMemoryBufferCreateInfo::default().handle_types(handle_types),
        ));
    }
    if let Some(dedicated) = buffer.dedicated_nv {
        nodes.push(Node::DedicatedBufferNv(
            vk::DedicatedAllocationBufferCreateInfoNV::default().dedicated_allocation(dedicated),
        ));
    }
    if buffer.buffer_collection {
        nodes.push(tagged(
            vk::StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA,
        ));
    }
    Chain(nodes)
}

fn image_chain(image: &ImageMake) -> Chain {
    let mut nodes = Vec::new();
    if let Some(flags) = image.flags2 {
        nodes.push(Node::Flags2(
            vk::ImageCreateFlags2CreateInfoKHR::default().flags(flags),
        ));
    }
    if let Some(handle_types) = image.external {
        nodes.push(Node::ExternalImage(
            vk::ExternalMemoryImageCreateInfo::default().handle_types(handle_types),
        ));
    }
    if let Some(non_empty) = image.external_nv {
        let handle_types = if non_empty {
            vk::ExternalMemoryHandleTypeFlagsNV::OPAQUE_WIN32
        } else {
            vk::ExternalMemoryHandleTypeFlagsNV::empty()
        };
        nodes.push(Node::ExternalImageNv(
            vk::ExternalMemoryImageCreateInfoNV::default().handle_types(handle_types),
        ));
    }
    if let Some(dedicated) = image.dedicated_nv {
        nodes.push(Node::DedicatedImageNv(
            vk::DedicatedAllocationImageCreateInfoNV::default().dedicated_allocation(dedicated),
        ));
    }
    if let Some(with_swapchain) = image.swapchain {
        let swapchain = if with_swapchain {
            vk::SwapchainKHR::from_raw(0x5a)
        } else {
            vk::SwapchainKHR::null()
        };
        nodes.push(Node::Swapchain(
            vk::ImageSwapchainCreateInfoKHR::default().swapchain(swapchain),
        ));
    }
    if let Some((s_type, external_format)) = image.external_format {
        nodes.push(Node::ExternalFormat(ExternalFormat {
            s_type,
            p_next: ptr::null_mut(),
            external_format,
        }));
    }
    if let Some(s_type) = image.imported_from {
        nodes.push(tagged(s_type));
    }
    Chain(nodes)
}

fn compatible(location: MemoryLocation, memory_type_bits: u32) -> bool {
    let required = if let MemoryLocation::Custom { required, .. } = location {
        required
    } else if location == MemoryLocation::GpuOnly {
        vk::MemoryPropertyFlags::DEVICE_LOCAL
    } else {
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
    };
    let properties = FakeDevice::memory_properties();
    (0..properties.memory_type_count).any(|index| {
        memory_type_bits & (1 << index) != 0
            && properties.memory_types[index as usize]
                .property_flags
                .contains(required)
    })
}

struct Live {
    raw: u64,
    kind: ResourceKind,
    allocation: Allocation,
    linear: bool,
}

struct Run<'f> {
    fake: &'f FakeDevice,
    allocator: Allocator,
    allocator_flags: vk::MemoryAllocateFlags,
    live: Vec<Live>,
    outcomes: Outcomes,
}

#[derive(Debug, Default)]
struct Outcomes {
    created: usize,
    created_images: usize,
    dedicated: usize,
    device_address_added: usize,
    shared_with_another: usize,
    linear_drm_images: usize,
    refused: usize,
    vulkan_errors: usize,
    no_compatible_type: usize,
    destroyed: usize,
    destroys_refused: usize,
}

fn empty_shared_blocks(allocator: &Allocator) -> HashSet<vk::DeviceMemory> {
    allocator
        .snapshot()
        .memory_types
        .iter()
        .flat_map(|memory_type| &memory_type.blocks)
        .filter(|block| block.allocated_region_count == 0 && !block.dedicated)
        .map(|block| block.memory)
        .collect()
}

impl Run<'_> {
    fn check_no_misuse(&self, context: &str) -> Result<(), TestCaseError> {
        let misuse = self.fake.misuse();
        ensure!(
            misuse.is_empty(),
            "{context}: the fake device saw misuse:\n{}",
            misuse.join("\n")
        );
        Ok(())
    }

    fn check_nothing_left_by(
        &self,
        context: &str,
        resources_before: &HashMap<u64, fake_device::ResourceObject>,
        memory_before: &HashSet<vk::DeviceMemory>,
        live_before: usize,
    ) -> Result<(), TestCaseError> {
        ensure!(
            self.fake.resources() == *resources_before,
            "{context}: the resources of the device changed: {:?} became {:?}",
            resources_before,
            self.fake.resources()
        );
        ensure!(
            self.allocator.live_allocation_count() == live_before,
            "{context}: {} allocations are live instead of {live_before}",
            self.allocator.live_allocation_count()
        );
        let memory_after: HashSet<_> = self.fake.memory_objects().into_keys().collect();
        ensure!(
            memory_before.is_subset(&memory_after),
            "{context}: memory that existed before was freed: {:?} became {:?}",
            memory_before,
            memory_after
        );
        let empty = empty_shared_blocks(&self.allocator);
        for memory in memory_after.difference(memory_before) {
            ensure!(
                empty.contains(memory),
                "{context}: {memory:?} was allocated and is not an empty shared block"
            );
        }
        Ok(())
    }

    unsafe fn create(&mut self, create: &Create) -> Result<(), TestCaseError> {
        let expected = expected_of(&create.make, &create.setup);
        let alignment_accepted = create.alignment == 0 || create.alignment.is_power_of_two();
        let flags_accepted = !(create.memory_allocate_flags | self.allocator_flags)
            .contains(vk::MemoryAllocateFlags::DEVICE_MASK);
        let refused = expected.refused || !alignment_accepted || !flags_accepted;
        let is_drm_image = matches!(
            &create.make,
            Make::Image(image) if image.tiling == vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT
        );
        let expected_error = if refused {
            Some(AllocationError::InvalidAllocationCreateDesc)
        } else {
            match create.fault {
                Fault::Creation(result) => Some(AllocationError::Vulkan(result)),
                Fault::ModifierQuery(result) if is_drm_image => {
                    Some(AllocationError::Vulkan(result))
                }
                _ if !compatible(create.location, create.setup.memory_type_bits) => {
                    Some(AllocationError::NoCompatibleMemoryType)
                }
                Fault::Allocation(result) | Fault::Map(result) | Fault::Bind(result) => {
                    Some(AllocationError::Vulkan(result))
                }
                _ => None,
            }
        };

        self.fake.set_resource_setup(create.setup);
        match create.fault {
            Fault::None => {}
            Fault::Creation(result) => self.fake.fail_next_resource_creation(result),
            Fault::ModifierQuery(result) => self.fake.fail_next_modifier_query(result),
            Fault::Allocation(result) => self.fake.fail_next_allocation(result),
            Fault::Map(result) => self.fake.fault_next_map(MapFault::Fails(result)),
            Fault::Bind(result) => self.fake.fail_next_bind(result),
        }
        let desc = ResourceAllocationDesc {
            name: create.named.then_some("resource"),
            mapped: create.mapped,
            memory_allocate_flags: create.memory_allocate_flags,
            alignment: create.alignment,
            ..ResourceAllocationDesc::new(create.location)
        };
        let resources_before = self.fake.resources();
        let memory_before: HashSet<_> = self.fake.memory_objects().into_keys().collect();
        let live_before = self.allocator.live_allocation_count();
        let calls_before = self.fake.calls();

        let (kind, result) = match &create.make {
            Make::Buffer(buffer) => {
                let mut chain = buffer_chain(buffer);
                let mut info = vk::BufferCreateInfo::default()
                    .flags(buffer.flags)
                    .size(create.setup.size)
                    .usage(buffer.usage);
                info.p_next = chain.link();
                let result = unsafe { self.allocator.create_buffer(&info, desc) };
                (
                    ResourceKind::Buffer,
                    result.map(|(buffer, allocation)| (buffer.as_raw(), allocation)),
                )
            }
            Make::Image(image) => {
                let mut chain = image_chain(image);
                let mut info = vk::ImageCreateInfo::default()
                    .flags(image.flags)
                    .image_type(vk::ImageType::_2D)
                    .format(vk::Format::R8G8B8A8_UNORM)
                    .extent(vk::Extent3D {
                        width: 16,
                        height: 16,
                        depth: 1,
                    })
                    .mip_levels(1)
                    .array_layers(1)
                    .tiling(image.tiling)
                    .usage(vk::ImageUsageFlags::SAMPLED);
                info.p_next = chain.link();
                let result = unsafe { self.allocator.create_image(&info, desc) };
                (
                    ResourceKind::Image,
                    result.map(|(image, allocation)| (image.as_raw(), allocation)),
                )
            }
        };
        self.fake.clear_faults();
        let context = format!("{create:?}");
        self.check_no_misuse(&context)?;

        let (raw, allocation) = match (result, expected_error) {
            (Ok(made), None) => made,
            (Err(error), Some(expected_error)) => {
                ensure!(
                    error == expected_error,
                    "{context}: the error is {error:?} instead of {expected_error:?}"
                );
                if refused {
                    ensure!(
                        self.fake.calls() == calls_before,
                        "{context}: a refused description made Vulkan calls: {:?} became {:?}",
                        calls_before,
                        self.fake.calls()
                    );
                    self.outcomes.refused += 1;
                } else if error == AllocationError::NoCompatibleMemoryType {
                    self.outcomes.no_compatible_type += 1;
                } else {
                    self.outcomes.vulkan_errors += 1;
                }
                return self.check_nothing_left_by(
                    &context,
                    &resources_before,
                    &memory_before,
                    live_before,
                );
            }
            (Ok((raw, allocation)), Some(expected_error)) => {
                return Err(TestCaseError::fail(format!(
                    "{context}: made {raw:#x} in {allocation:?} instead of failing with \
                     {expected_error:?}"
                )));
            }
            (Err(error), None) => {
                return Err(TestCaseError::fail(format!(
                    "{context}: failed with {error:?}"
                )));
            }
        };

        let resource = self
            .fake
            .resources()
            .get(&raw)
            .copied()
            .ok_or_else(|| TestCaseError::fail(format!("{context}: {raw:#x} is not live")))?;
        ensure!(
            resource.kind == kind,
            "{context}: {raw:#x} is a {:?}",
            resource.kind
        );
        ensure!(
            resource.bound == Some((allocation.memory(), allocation.offset())),
            "{context}: the resource is bound to {:?}, the allocation is at {:?} {}",
            resource.bound,
            allocation.memory(),
            allocation.offset()
        );
        let alignment = create.setup.alignment.max(create.alignment).max(1);
        ensure!(
            allocation.offset().is_multiple_of(alignment),
            "{context}: offset {} is not a multiple of {alignment}",
            allocation.offset()
        );

        let dedicated = create.setup.requires_dedicated || create.setup.prefers_dedicated;
        let mut flags = create.memory_allocate_flags | self.allocator_flags;
        if expected.device_address {
            flags |= vk::MemoryAllocateFlags::DEVICE_ADDRESS;
        }
        let chain = self
            .fake
            .chains()
            .get(&allocation.memory())
            .copied()
            .ok_or_else(|| TestCaseError::fail(format!("{context}: the memory is not live")))?;
        let expected_dedicated = dedicated.then(|| resource.handle(raw));
        ensure!(
            chain.dedicated == expected_dedicated,
            "{context}: vkAllocateMemory was given the dedicated resource {:?} instead of {:?}",
            chain.dedicated,
            expected_dedicated
        );
        let expected_export = (!expected.export.is_empty()).then_some(expected.export);
        ensure!(
            chain.export == expected_export,
            "{context}: vkAllocateMemory was given the export handle types {:?} instead of {:?}",
            chain.export,
            expected_export
        );
        let expected_flags = (!flags.is_empty()).then_some((flags, 0));
        ensure!(
            chain.flags == expected_flags,
            "{context}: vkAllocateMemory was given the flags {:?} instead of {:?}",
            chain.flags,
            expected_flags
        );
        let own_memory = dedicated
            || !expected.export.is_empty()
            || flags.contains(vk::MemoryAllocateFlags::ZERO_INITIALIZE_EXT);
        ensure!(
            allocation.is_dedicated() == own_memory,
            "{context}: is_dedicated is {} instead of {own_memory}",
            allocation.is_dedicated()
        );
        let host_visible = allocation
            .memory_properties()
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE);
        ensure!(
            allocation.mapped_ptr().is_some() == (create.mapped && host_visible),
            "{context}: the mapped pointer is {:?} for mapped {} in memory that is {}host-visible",
            allocation.mapped_ptr(),
            create.mapped,
            if host_visible { "" } else { "not " }
        );
        let expected_class = if expected.linear {
            AllocationResourceClass::Linear
        } else {
            AllocationResourceClass::Optimal
        };
        let class = self
            .allocator
            .snapshot()
            .memory_types
            .iter()
            .flat_map(|memory_type| &memory_type.blocks)
            .find(|block| block.memory == allocation.memory())
            .map(|block| block.resource_class);
        ensure!(
            class == Some(expected_class),
            "{context}: the block of the allocation holds {class:?} instead of {expected_class:?}"
        );
        for other in &self.live {
            if other.allocation.memory() == allocation.memory() {
                ensure!(
                    other.linear == expected.linear,
                    "{context}: a resource that is {}linear shares {:?} with one that is not",
                    if expected.linear { "" } else { "not " },
                    allocation.memory()
                );
                self.outcomes.shared_with_another += 1;
            }
        }

        self.outcomes.created += 1;
        if kind == ResourceKind::Image {
            self.outcomes.created_images += 1;
            if is_drm_image && expected.linear {
                self.outcomes.linear_drm_images += 1;
            }
        }
        if dedicated {
            self.outcomes.dedicated += 1;
        }
        if expected.device_address
            && !(create.memory_allocate_flags | self.allocator_flags)
                .contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS)
        {
            self.outcomes.device_address_added += 1;
        }
        self.live.push(Live {
            raw,
            kind,
            allocation,
            linear: expected.linear,
        });
        Ok(())
    }

    unsafe fn destroy_live(&mut self, live: Live) -> Result<(), FakeFree> {
        let result = unsafe {
            match live.kind {
                ResourceKind::Buffer => self
                    .allocator
                    .destroy_buffer(vk::Buffer::from_raw(live.raw), live.allocation),
                ResourceKind::Image => self
                    .allocator
                    .destroy_image(vk::Image::from_raw(live.raw), live.allocation),
            }
        };
        result.map_err(|error| FakeFree {
            live: Live {
                allocation: error.allocation,
                ..live
            },
            error: error.error,
        })
    }

    unsafe fn destroy(&mut self, target: Index, mapping_held: bool) -> Result<(), TestCaseError> {
        if self.live.is_empty() {
            return Ok(());
        }
        let live = self.live.remove(target.index(self.live.len()));
        let raw = live.raw;
        let memory = live.allocation.memory();
        let context = format!(
            "destroying {:?} {raw:#x} in {:?}",
            live.kind, live.allocation
        );
        let held = mapping_held
            && live
                .allocation
                .memory_properties()
                .contains(vk::MemoryPropertyFlags::HOST_VISIBLE);
        let live = if held {
            unsafe { self.allocator.map_memory(&live.allocation) }
                .map_err(|e| TestCaseError::fail(format!("{context}: map_memory: {e}")))?;
            let refused = match unsafe { self.destroy_live(live) } {
                Ok(()) => {
                    return Err(TestCaseError::fail(format!(
                        "{context}: destroyed while a mapping is held"
                    )));
                }
                Err(refused) => refused,
            };
            ensure!(
                refused.error == AllocationError::InvalidAllocation,
                "{context}: refused with {:?}",
                refused.error
            );
            ensure!(
                self.fake.resources().contains_key(&raw),
                "{context}: the resource was destroyed although its memory was not freed"
            );
            self.check_no_misuse(&context)?;
            self.outcomes.destroys_refused += 1;
            unsafe { self.allocator.unmap_memory(&refused.live.allocation) }
                .map_err(|e| TestCaseError::fail(format!("{context}: unmap_memory: {e}")))?;
            refused.live
        } else {
            live
        };
        let live_before = self.allocator.live_allocation_count();
        if let Err(refused) = unsafe { self.destroy_live(live) } {
            return Err(TestCaseError::fail(format!(
                "{context}: failed with {:?}",
                refused.error
            )));
        }
        self.check_no_misuse(&context)?;
        ensure!(
            !self.fake.resources().contains_key(&raw),
            "{context}: the resource is still live"
        );
        ensure!(
            self.allocator.live_allocation_count() + 1 == live_before,
            "{context}: the allocation was not freed"
        );
        let still_used = self
            .live
            .iter()
            .any(|other| other.allocation.memory() == memory);
        let memory_live = self.fake.memory_objects().contains_key(&memory);
        ensure!(
            still_used || !memory_live || empty_shared_blocks(&self.allocator).contains(&memory),
            "{context}: {memory:?} holds nothing and is neither freed nor an empty shared block"
        );
        ensure!(
            !still_used || memory_live,
            "{context}: {memory:?} was freed although other resources are bound to it"
        );
        self.outcomes.destroyed += 1;
        Ok(())
    }

    fn check_end(&self) -> Result<(), TestCaseError> {
        ensure!(
            self.fake.resources().is_empty(),
            "resources are left: {:?}",
            self.fake.resources()
        );
        ensure!(self.allocator.is_empty(), "allocations are left");
        let empty = empty_shared_blocks(&self.allocator);
        let memory: HashSet<_> = self.fake.memory_objects().into_keys().collect();
        ensure!(
            memory == empty,
            "after every resource was destroyed, the device holds {memory:?}, and the empty \
             shared blocks are {empty:?}"
        );
        for memory_type in self.allocator.snapshot().memory_types {
            let mut flags = Vec::new();
            for block in &memory_type.blocks {
                ensure!(
                    !flags.contains(&block.memory_allocate_flags),
                    "memory type {} keeps more than one empty block with {:?}",
                    memory_type.memory_type_index,
                    block.memory_allocate_flags
                );
                flags.push(block.memory_allocate_flags);
            }
        }
        self.check_no_misuse("at the end")
    }
}

struct FakeFree {
    live: Live,
    error: AllocationError,
}

fn run(
    operations: &[Op],
    allocator_flags: vk::MemoryAllocateFlags,
) -> Result<Outcomes, TestCaseError> {
    let fake = FakeDevice::new();
    let allocator = Allocator::new(AllocatorCreateDesc {
        memory_allocate_flags: allocator_flags,
        ..AllocatorCreateDesc {
            device: fake.device(),
            memory_properties: FakeDevice::memory_properties(),
            non_coherent_atom_size: fake_device::NON_COHERENT_ATOM_SIZE,
            block_size: fake_device::BLOCK_SIZE,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        }
    });
    let mut run = Run {
        fake: &fake,
        allocator,
        allocator_flags,
        live: Vec::new(),
        outcomes: Outcomes::default(),
    };
    for operation in operations {
        unsafe {
            match operation {
                Op::Create(create) => run.create(create)?,
                Op::Destroy {
                    target,
                    mapping_held,
                } => run.destroy(*target, *mapping_held)?,
            }
        }
    }
    while let Some(live) = run.live.pop() {
        let context = format!("destroying {:#x} at the end", live.raw);
        if let Err(refused) = unsafe { run.destroy_live(live) } {
            return Err(TestCaseError::fail(format!(
                "{context}: {:?}",
                refused.error
            )));
        }
    }
    run.check_end()?;
    Ok(run.outcomes)
}

fn config() -> ProptestConfig {
    ProptestConfig {
        cases: 512,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

proptest! {
    #![proptest_config(config())]

    #[test]
    fn resources_are_created_bound_and_destroyed_as_their_create_info_and_requirements_say(
        (operations, allocator_flags) in operations()
    ) {
        run(&operations, allocator_flags)?;
    }
}

#[test]
fn the_generated_cases_reach_every_outcome() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = operations();
    let mut total = Outcomes::default();
    for _ in 0..400 {
        let (operations, allocator_flags) = strategy.new_tree(&mut runner).unwrap().current();
        let outcomes = run(&operations, allocator_flags).unwrap();
        total.created += outcomes.created;
        total.created_images += outcomes.created_images;
        total.dedicated += outcomes.dedicated;
        total.device_address_added += outcomes.device_address_added;
        total.shared_with_another += outcomes.shared_with_another;
        total.linear_drm_images += outcomes.linear_drm_images;
        total.refused += outcomes.refused;
        total.vulkan_errors += outcomes.vulkan_errors;
        total.no_compatible_type += outcomes.no_compatible_type;
        total.destroyed += outcomes.destroyed;
        total.destroys_refused += outcomes.destroys_refused;
    }
    println!("{total:#?}");
    for (name, count) in [
        ("resources created", total.created),
        ("images created", total.created_images),
        ("dedicated allocations", total.dedicated),
        (
            "DEVICE_ADDRESS added for the usage",
            total.device_address_added,
        ),
        (
            "resources in memory with another",
            total.shared_with_another,
        ),
        ("linear DRM-modifier images", total.linear_drm_images),
        ("refused descriptions", total.refused),
        ("Vulkan errors", total.vulkan_errors),
        ("no compatible memory type", total.no_compatible_type),
        ("resources destroyed", total.destroyed),
        (
            "destroys refused for a held mapping",
            total.destroys_refused,
        ),
    ] {
        assert!(count > 0, "never reached: {name}");
    }
}

#[test]
fn a_uniform_buffer_is_created_in_host_coherent_memory_bound_to_it_and_mapped() {
    let fake = FakeDevice::new();
    let mut allocator = fake.allocator();
    let calls = fake.calls();
    let made = unsafe {
        allocator.create_buffer(
            &vk::BufferCreateInfo::default()
                .size(64)
                .usage(vk::BufferUsageFlags::UNIFORM_BUFFER),
            ResourceAllocationDesc::new(MemoryLocation::CpuToGpu),
        )
    };
    let (buffer, allocation) = made.expect("the fake device has the Vulkan 1.1 commands");
    assert_eq!(
        fake.calls().create_resource,
        calls.create_resource + 1,
        "one buffer is created"
    );
    assert_eq!(
        fake.resources().get(&buffer.as_raw()).and_then(|r| r.bound),
        Some((allocation.memory(), allocation.offset())),
        "the buffer is bound to its allocation"
    );
    assert_eq!(
        fake.chains()[&allocation.memory()].dedicated,
        None::<Dedicated>,
        "a buffer that does not ask for dedicated memory gets none"
    );
    assert_eq!(
        allocation.memory_type_index(),
        HOST_COHERENT,
        "CpuToGpu takes the host-coherent memory type"
    );
    assert!(
        allocation.mapped_ptr().is_some(),
        "a CpuToGpu buffer made with ResourceAllocationDesc::new is mapped"
    );
    unsafe { allocator.destroy_buffer(buffer, allocation) }.unwrap();
    assert!(fake.resources().is_empty(), "the buffer is destroyed");
    assert!(fake.misuse().is_empty(), "{:?}", fake.misuse());
}
