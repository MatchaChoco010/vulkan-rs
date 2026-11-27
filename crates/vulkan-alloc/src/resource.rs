use std::ffi::c_void;

use vulkan::{Device, vk};

use crate::allocation::{Allocation, AllocationCreateDesc, AllocationScheme};
use crate::allocator::Allocator;
use crate::error::{AllocationError, FreeError};
use crate::memory_type::MemoryLocation;

const DRM_FORMAT_MOD_LINEAR: u64 = 0;

const REFUSED_BUFFER_FLAGS: vk::BufferCreateFlags = vk::BufferCreateFlags::SPARSE_BINDING
    .union(vk::BufferCreateFlags::SPARSE_RESIDENCY)
    .union(vk::BufferCreateFlags::SPARSE_ALIASED)
    .union(vk::BufferCreateFlags::DEVICE_ADDRESS_CAPTURE_REPLAY)
    .union(vk::BufferCreateFlags::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT);

const REFUSED_IMAGE_FLAGS: vk::ImageCreateFlags2KHR = vk::ImageCreateFlags2KHR::SPARSE_BINDING
    .union(vk::ImageCreateFlags2KHR::SPARSE_RESIDENCY)
    .union(vk::ImageCreateFlags2KHR::SPARSE_ALIASED)
    .union(vk::ImageCreateFlags2KHR::DISJOINT)
    .union(vk::ImageCreateFlags2KHR::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT);

const DEVICE_ADDRESS_USAGE: vk::BufferUsageFlags2 =
    vk::BufferUsageFlags2::SHADER_DEVICE_ADDRESS.union(vk::BufferUsageFlags2::DESCRIPTOR_HEAP_EXT);

/// How [`Allocator::create_buffer`] and [`Allocator::create_image`] allocate the memory
/// of the resource they create.
///
/// The other fields of the [`AllocationCreateDesc`] that these functions pass to
/// [`Allocator::allocate`] follow from the resource: its memory requirements, whether it
/// is linear, whether it needs or prefers a dedicated allocation, the handle types it
/// can be exported as, and, for a buffer, whether its usage needs
/// [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`].
///
/// Create it with [`ResourceAllocationDesc::new`] and change the fields an allocation
/// needs, with struct update syntax or by assigning to them.
///
/// ```
/// use vulkan_alloc::{MemoryLocation, ResourceAllocationDesc};
///
/// let uniforms = ResourceAllocationDesc {
///     name: Some("uniforms"),
///     ..ResourceAllocationDesc::new(MemoryLocation::CpuToGpu)
/// };
/// assert!(uniforms.mapped);
///
/// let vertices = ResourceAllocationDesc {
///     alignment: 256,
///     ..ResourceAllocationDesc::new(MemoryLocation::GpuOnly)
/// };
/// assert!(!vertices.mapped);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResourceAllocationDesc<'a> {
    /// A name for diagnostics, as [`AllocationCreateDesc::name`].
    pub name: Option<&'a str>,
    /// The memory property flags the memory type must have and should have, as
    /// [`AllocationCreateDesc::location`].
    pub location: MemoryLocation,
    /// Whether the allocation is mapped before it is returned, if its memory type is
    /// `HOST_VISIBLE`, as [`AllocationCreateDesc::mapped`].
    ///
    /// [`ResourceAllocationDesc::new`] sets it to `true` for
    /// [`MemoryLocation::CpuToGpu`] and [`MemoryLocation::GpuToCpu`], the locations for
    /// memory that the host writes or reads, so that [`Allocation::mapped_slice_mut`]
    /// and [`Allocation::mapped_slice`] reach that memory without another call. For
    /// [`MemoryLocation::GpuOnly`] and [`MemoryLocation::Custom`] it sets it to `false`:
    /// mapping all such memory, which is host-visible on many integrated GPUs, would take
    /// address space that the host may not have, as on a 32-bit target.
    ///
    /// On a logical device that represents more than one physical device, memory of a
    /// heap with `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT` cannot be mapped (see the
    /// [crate documentation](crate#mapping)). If such memory can be chosen for a
    /// [`MemoryLocation::CpuToGpu`] or [`MemoryLocation::GpuToCpu`] allocation, set this
    /// to `false`.
    pub mapped: bool,
    /// Flags for `VkMemoryAllocateFlagsInfo`, as
    /// [`AllocationCreateDesc::memory_allocate_flags`]. The allocator adds
    /// [`AllocatorCreateDesc::memory_allocate_flags`][allocator flags]
    /// to them, and [`Allocator::create_buffer`] adds
    /// [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`] for a buffer whose usage needs it.
    ///
    /// [allocator flags]: crate::AllocatorCreateDesc::memory_allocate_flags
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    /// An alignment that the offset of the allocation must have besides the alignment of
    /// the memory requirements of the resource, such as the alignment that a device
    /// address in a descriptor needs. The larger of the two is used. 0 is treated as 1.
    ///
    /// It must be a power of two, or 0: another value is refused with
    /// [`AllocationError::InvalidAllocationCreateDesc`] before any Vulkan call.
    pub alignment: u64,
}

impl<'a> ResourceAllocationDesc<'a> {
    /// Describes memory in `location`, with [`ResourceAllocationDesc::name`] `None`,
    /// [`ResourceAllocationDesc::mapped`] `true` for [`MemoryLocation::CpuToGpu`] and
    /// [`MemoryLocation::GpuToCpu`] and `false` for the other locations (see that field),
    /// empty [`ResourceAllocationDesc::memory_allocate_flags`] and an
    /// [`ResourceAllocationDesc::alignment`] of 1, which adds no alignment to that of
    /// the memory requirements.
    pub const fn new(location: MemoryLocation) -> Self {
        Self {
            name: None,
            location,
            mapped: matches!(
                location,
                MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu
            ),
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            alignment: 1,
        }
    }
}

#[repr(C)]
struct ExternalFormat {
    base: vk::BaseInStructure<'static>,
    external_format: u64,
}

struct BufferFacts {
    usage: vk::BufferUsageFlags2,
    export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    accepted: bool,
}

struct ImageFacts {
    export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    accepted: bool,
}

unsafe fn for_each_in_chain(p_next: *const c_void, mut visit: impl FnMut(*const c_void)) {
    let mut next = p_next.cast::<vk::BaseInStructure<'_>>();
    while let Some(base) = unsafe { next.as_ref() } {
        visit(next.cast());
        next = base.p_next;
    }
}

unsafe fn structure_type(next: *const c_void) -> vk::StructureType {
    unsafe { (*next.cast::<vk::BaseInStructure<'_>>()).s_type }
}

unsafe fn buffer_facts(info: &vk::BufferCreateInfo<'_>) -> BufferFacts {
    let mut facts = BufferFacts {
        usage: vk::BufferUsageFlags2::from_raw(u64::from(info.usage.as_raw())),
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        accepted: !info.flags.intersects(REFUSED_BUFFER_FLAGS),
    };
    unsafe {
        for_each_in_chain(info.p_next, |next| match structure_type(next) {
            vk::StructureType::BUFFER_USAGE_FLAGS_2_CREATE_INFO => {
                facts.usage = (*next.cast::<vk::BufferUsageFlags2CreateInfo<'_>>()).usage;
            }
            vk::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => {
                facts.export_handle_types =
                    (*next.cast::<vk::ExternalMemoryBufferCreateInfo<'_>>()).handle_types;
            }
            vk::StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV => {
                let info = &*next.cast::<vk::DedicatedAllocationBufferCreateInfoNV<'_>>();
                facts.accepted &= info.dedicated_allocation == vk::FALSE;
            }
            vk::StructureType::BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA => {
                facts.accepted = false;
            }
            _ => {}
        });
    }
    facts
}

unsafe fn image_facts(info: &vk::ImageCreateInfo<'_>) -> ImageFacts {
    let mut flags = vk::ImageCreateFlags2KHR::from_raw(u64::from(info.flags.as_raw()));
    let mut facts = ImageFacts {
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        accepted: true,
    };
    unsafe {
        for_each_in_chain(info.p_next, |next| match structure_type(next) {
            vk::StructureType::IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR => {
                flags = (*next.cast::<vk::ImageCreateFlags2CreateInfoKHR<'_>>()).flags;
            }
            vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => {
                let handle_types =
                    (*next.cast::<vk::ExternalMemoryImageCreateInfo<'_>>()).handle_types;
                facts.export_handle_types = handle_types;
                facts.accepted &= !handle_types
                    .contains(vk::ExternalMemoryHandleTypeFlags::ANDROID_HARDWARE_BUFFER_ANDROID);
            }
            vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV => {
                let info = &*next.cast::<vk::ExternalMemoryImageCreateInfoNV<'_>>();
                facts.accepted &= info.handle_types.is_empty();
            }
            vk::StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV => {
                let info = &*next.cast::<vk::DedicatedAllocationImageCreateInfoNV<'_>>();
                facts.accepted &= info.dedicated_allocation == vk::FALSE;
            }
            vk::StructureType::IMAGE_SWAPCHAIN_CREATE_INFO_KHR => {
                let info = &*next.cast::<vk::ImageSwapchainCreateInfoKHR<'_>>();
                facts.accepted &= info.swapchain == vk::SwapchainKHR::null();
            }
            vk::StructureType::EXTERNAL_FORMAT_ANDROID
            | vk::StructureType::EXTERNAL_FORMAT_QNX
            | vk::StructureType::EXTERNAL_FORMAT_OHOS => {
                facts.accepted &= (*next.cast::<ExternalFormat>()).external_format == 0;
            }
            vk::StructureType::BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA
            | vk::StructureType::IMPORT_METAL_TEXTURE_INFO_EXT
            | vk::StructureType::IMPORT_METAL_IO_SURFACE_INFO_EXT => {
                facts.accepted = false;
            }
            _ => {}
        });
    }
    facts.accepted &= !flags.intersects(REFUSED_IMAGE_FLAGS);
    facts
}

fn dedicated(requirements: &vk::MemoryDedicatedRequirements<'_>) -> bool {
    requirements.requires_dedicated_allocation != vk::FALSE
        || requirements.prefers_dedicated_allocation != vk::FALSE
}

unsafe fn buffer_requirements(
    device: &Device,
    buffer: vk::Buffer,
) -> (vk::MemoryRequirements, bool) {
    let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
    let mut requirements =
        vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);
    unsafe {
        device.get_buffer_memory_requirements2(
            &vk::BufferMemoryRequirementsInfo2::default().buffer(buffer),
            &mut requirements,
        );
    }
    let memory_requirements = requirements.memory_requirements;
    (memory_requirements, dedicated(&dedicated_requirements))
}

unsafe fn image_requirements(device: &Device, image: vk::Image) -> (vk::MemoryRequirements, bool) {
    let mut dedicated_requirements = vk::MemoryDedicatedRequirements::default();
    let mut requirements =
        vk::MemoryRequirements2::default().push_next(&mut dedicated_requirements);
    unsafe {
        device.get_image_memory_requirements2(
            &vk::ImageMemoryRequirementsInfo2::default().image(image),
            &mut requirements,
        );
    }
    let memory_requirements = requirements.memory_requirements;
    (memory_requirements, dedicated(&dedicated_requirements))
}

unsafe fn is_linear_image(
    device: &Device,
    image: vk::Image,
    tiling: vk::ImageTiling,
) -> vk::Result<bool> {
    match tiling {
        vk::ImageTiling::LINEAR => Ok(true),
        vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT => {
            let mut properties = vk::ImageDrmFormatModifierPropertiesEXT::default();
            unsafe { device.get_image_drm_format_modifier_properties_ext(image, &mut properties) }?;
            Ok(properties.drm_format_modifier == DRM_FORMAT_MOD_LINEAR)
        }
        _ => Ok(false),
    }
}

impl Allocator {
    /// Creates a buffer, allocates memory for it and binds the buffer to that memory.
    ///
    /// This is the usual way to make a buffer with memory from the allocator. It does
    /// what a caller of [`Allocator::allocate`] does by hand, in this order:
    ///
    /// 1. Creates the buffer with `vkCreateBuffer` from `info`.
    /// 2. Queries its memory requirements with `vkGetBufferMemoryRequirements2`, with a
    ///    `VkMemoryDedicatedRequirements` in the chain.
    /// 3. Allocates with [`Allocator::allocate`]. The [`AllocationCreateDesc`] has the
    ///    [`ResourceAllocationDesc::name`], [`ResourceAllocationDesc::location`] and
    ///    [`ResourceAllocationDesc::mapped`] of `desc`, and:
    ///    - the memory requirements of the buffer, with an alignment raised to
    ///      [`ResourceAllocationDesc::alignment`] if that is larger;
    ///    - [`AllocationCreateDesc::linear`] `true`, as for every buffer;
    ///    - [`AllocationScheme::DedicatedBuffer`] with the buffer if the implementation
    ///      sets `requiresDedicatedAllocation` or `prefersDedicatedAllocation`, and
    ///      [`AllocationScheme::GpuAllocatorManaged`] otherwise;
    ///    - as [`AllocationCreateDesc::export_handle_types`], the `handleTypes` of the
    ///      `VkExternalMemoryBufferCreateInfo` in the `pNext` chain of `info`, or none if
    ///      the chain has no such structure;
    ///    - as [`AllocationCreateDesc::memory_allocate_flags`],
    ///      [`ResourceAllocationDesc::memory_allocate_flags`], together with
    ///      [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`] if the effective usage of the
    ///      buffer contains `SHADER_DEVICE_ADDRESS` or `DESCRIPTOR_HEAP_EXT`. Vulkan
    ///      requires the flag for the memory of such buffers (VUIDs
    ///      `vkBindBufferMemory-bufferDeviceAddress-03339` and
    ///      `vkBindBufferMemory-buffer-11408`). The effective usage is the `usage` of a
    ///      `VkBufferUsageFlags2CreateInfo` in the chain if there is one, which Vulkan
    ///      then uses instead of `info.usage`, and `info.usage` otherwise.
    /// 4. Binds the buffer to [`Allocation::memory`] at [`Allocation::offset`] with
    ///    `vkBindBufferMemory`.
    ///
    /// The buffer and the allocation go together from then on: pass both to
    /// [`Allocator::destroy_buffer`] when the device no longer uses the buffer.
    ///
    /// [`Allocator::allocate`] remains for memory that is not for a buffer or image of
    /// this kind, for resources that the caller creates and binds itself, such as a
    /// buffer bound with `vkBindBufferMemory2` and a chained structure, and for the
    /// resources this function refuses.
    ///
    /// # What this function does not create
    ///
    /// These buffers are refused with [`AllocationError::InvalidAllocationCreateDesc`]:
    ///
    /// - sparse buffers (`SPARSE_BINDING`, `SPARSE_RESIDENCY` or `SPARSE_ALIASED` in
    ///   `info.flags`), whose memory is bound later in ranges with `vkQueueBindSparse`;
    /// - buffers created for capture and replay (`DEVICE_ADDRESS_CAPTURE_REPLAY` or
    ///   `DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT` in `info.flags`), whose memory needs
    ///   `DEVICE_ADDRESS_CAPTURE_REPLAY` and, on replay, an opaque capture address that
    ///   the allocator cannot request;
    /// - buffers whose chain has a `VkDedicatedAllocationBufferCreateInfoNV` with
    ///   `dedicatedAllocation` set, whose memory needs a
    ///   `VkDedicatedAllocationMemoryAllocateInfoNV` that the allocator cannot chain;
    /// - buffers whose chain has a `VkBufferCollectionBufferCreateInfoFUCHSIA`, whose
    ///   memory is imported from the buffer collection.
    ///
    /// A buffer for memory that is imported, such as a dma-buf received from another
    /// process, is bound to memory allocated with `vkAllocateMemory` directly. The handle
    /// types of a `VkExternalMemoryBufferCreateInfo` make the new memory exportable as
    /// those types. A protected buffer (`PROTECTED` in `info.flags`) is created as any
    /// other, with the location that `desc` gives (see the safety section).
    ///
    /// # Errors
    ///
    /// When several of these apply, the one listed first is returned:
    ///
    /// 1. [`AllocationError::InvalidAllocationCreateDesc`] for the buffers listed above,
    ///    if [`ResourceAllocationDesc::alignment`] is neither 0 nor a power of two, or
    ///    if [`ResourceAllocationDesc::memory_allocate_flags`] or
    ///    [`AllocatorCreateDesc::memory_allocate_flags`][allocator flags]
    ///    contain [`vk::MemoryAllocateFlags::DEVICE_MASK`]. It is returned before any
    ///    Vulkan call.
    /// 2. [`AllocationError::Vulkan`] with the result if `vkCreateBuffer` fails.
    /// 3. The errors of [`Allocator::allocate`] for the description above: in
    ///    particular [`AllocationError::NoCompatibleMemoryType`],
    ///    [`AllocationError::OutOfMemory`], [`AllocationError::Vulkan`] if
    ///    `vkAllocateMemory` fails otherwise, and the errors of mapping the allocation
    ///    when [`ResourceAllocationDesc::mapped`] is `true`, in the order that
    ///    [`Allocator::allocate`] lists them.
    /// 4. [`AllocationError::Vulkan`] with the result if `vkBindBufferMemory` fails.
    ///
    /// Nothing is left when an error is returned: the buffer, if it was created, is
    /// destroyed with `vkDestroyBuffer`, and the allocation, if it was made, is freed as
    /// [`Allocator::free`] frees it, so a block allocated for it is freed with
    /// `vkFreeMemory` or kept empty. As for [`Allocator::allocate`], the empty blocks of
    /// a heap in which `vkAllocateMemory` ran out of memory may have been freed.
    ///
    /// # Panics
    ///
    /// Panics before any Vulkan call if `vkGetBufferMemoryRequirements2` is not loaded,
    /// as on a device whose Vulkan version is below 1.1 (see the safety section).
    ///
    /// Panics when [`Allocator::allocate`] panics, because no identifier is left for a
    /// new allocation after 2<sup>64</sup> - 2 allocations in the process. The buffer is
    /// created by then and is not destroyed.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed, and
    ///   [`AllocatorCreateDesc::memory_properties`](crate::AllocatorCreateDesc::memory_properties)
    ///   must be those of its physical device.
    /// - The device must support Vulkan 1.1: the `apiVersion` of its physical device and
    ///   the `VkApplicationInfo::apiVersion` of its instance must both be 1.1 or higher.
    ///   `vkGetBufferMemoryRequirements2` and `VkMemoryDedicatedRequirements`, which this
    ///   function uses, are Vulkan 1.1. On an older device, create the buffer, query its
    ///   requirements and allocate with [`Allocator::allocate`].
    /// - `info` must be valid for `vkCreateBuffer`, including every structure of its
    ///   `pNext` chain, which this function reads. Structures of the chain that are not
    ///   named here are passed to `vkCreateBuffer` as they are; they must not require
    ///   anything of the memory of the buffer that its memory requirements do not state,
    ///   such as memory that is imported or allocated with a structure in the chain of
    ///   `vkAllocateMemory`.
    /// - The `vkAllocateMemory` calls must be valid. The flags of the allocation, from
    ///   [`ResourceAllocationDesc::memory_allocate_flags`], from
    ///   [`AllocatorCreateDesc::memory_allocate_flags`][allocator flags]
    ///   and the `DEVICE_ADDRESS` this function adds, need the matching device features:
    ///   `DEVICE_ADDRESS` needs the `bufferDeviceAddress` feature of
    ///   `VkPhysicalDeviceBufferDeviceAddressFeatures` or
    ///   `VkPhysicalDeviceVulkan12Features` (the feature of
    ///   `VK_EXT_buffer_device_address` alone does not allow the flag), and
    ///   `DEVICE_ADDRESS_CAPTURE_REPLAY` needs `bufferDeviceAddressCaptureReplay` and
    ///   `DEVICE_ADDRESS` in the same flags. Flags with `ZERO_INITIALIZE_EXT` must not be
    ///   given to a memory type with `PROTECTED`. A location that names
    ///   `DEVICE_COHERENT_AMD` or `DEVICE_UNCACHED_AMD` needs the `deviceCoherentMemory`
    ///   feature, and one that names `PROTECTED` the `protectedMemory` feature. The
    ///   handle types of a `VkExternalMemoryBufferCreateInfo` in the chain must be
    ///   exportable together for this buffer (as `vkGetPhysicalDeviceExternalBufferProperties`
    ///   reports) from every memory type that may be chosen, with the extensions for them
    ///   enabled. If the allocator does not know the size of the memory heap of such a
    ///   type (see [`Allocator::allocate`]), the size of the buffer's memory must not be
    ///   larger than that heap. The number of `VkDeviceMemory` objects on the device must
    ///   stay below `maxMemoryAllocationCount`; each block of the allocator counts, also
    ///   an empty block that it keeps.
    /// - A buffer created with `PROTECTED` in `info.flags` must be given a location that
    ///   requires `PROTECTED`, and any other buffer a location that names `PROTECTED`
    ///   neither as required nor as preferred, because Vulkan binds protected buffers
    ///   only to protected memory and other buffers only to other memory.
    /// - On a logical device that represents more than one physical device,
    ///   [`ResourceAllocationDesc::mapped`] must be `false`, and the allocation must not
    ///   be mapped later, if its memory type belongs to a heap with
    ///   `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`.
    /// - When `vkAllocateMemory` runs out of memory, the empty blocks of the heap are
    ///   freed with `vkFreeMemory`, as for [`Allocator::allocate`]: every submitted
    ///   command that refers to memory of an empty block must have completed.
    /// - The allocator holds `VkDeviceMemory` once this returns. It must be dropped
    ///   before the device is destroyed, unless it holds no block by then, and when it is
    ///   dropped, every submitted command that refers to its memory must have completed;
    ///   the buffers bound to that memory and pointers obtained by mapping it must not be
    ///   used afterwards (see the [crate documentation](crate#order-of-destruction)).
    /// - The returned buffer is bound to the allocation, and must not be bound again.
    ///
    /// [allocator flags]: crate::AllocatorCreateDesc::memory_allocate_flags
    pub unsafe fn create_buffer(
        &mut self,
        info: &vk::BufferCreateInfo<'_>,
        desc: ResourceAllocationDesc<'_>,
    ) -> Result<(vk::Buffer, Allocation), AllocationError> {
        let device = self.device().clone();
        assert!(
            device.fp().get_buffer_memory_requirements2.is_some(),
            "vkGetBufferMemoryRequirements2 is not loaded"
        );
        let facts = unsafe { buffer_facts(info) };
        if !facts.accepted || !self.accepts(&desc) {
            return Err(AllocationError::InvalidAllocationCreateDesc);
        }
        let buffer =
            unsafe { device.create_buffer(info, None) }.map_err(AllocationError::Vulkan)?;
        let (requirements, dedicated) = unsafe { buffer_requirements(&device, buffer) };
        let memory_allocate_flags = if facts.usage.intersects(DEVICE_ADDRESS_USAGE) {
            desc.memory_allocate_flags | vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            desc.memory_allocate_flags
        };
        let allocation_desc = AllocationCreateDesc {
            memory_allocate_flags,
            export_handle_types: facts.export_handle_types,
            allocation_scheme: if dedicated {
                AllocationScheme::DedicatedBuffer(buffer)
            } else {
                AllocationScheme::GpuAllocatorManaged
            },
            ..allocation_desc(&desc, requirements, true)
        };
        let allocation = unsafe {
            self.allocate_and_bind(
                allocation_desc,
                |allocation| {
                    device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
                },
                || device.destroy_buffer(buffer, None),
            )
        }?;
        Ok((buffer, allocation))
    }

    /// Creates an image, allocates memory for it and binds the image to that memory.
    ///
    /// This is the usual way to make an image with memory from the allocator. It does
    /// what [`Allocator::create_buffer`] does for a buffer, in this order:
    ///
    /// 1. Creates the image with `vkCreateImage` from `info`.
    /// 2. For an image with [`vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT`], queries the
    ///    modifier the image was given with `vkGetImageDrmFormatModifierPropertiesEXT`:
    ///    the one that `info` names, or the one the implementation chose from a list.
    /// 3. Queries its memory requirements with `vkGetImageMemoryRequirements2`, with a
    ///    `VkMemoryDedicatedRequirements` in the chain.
    /// 4. Allocates with [`Allocator::allocate`]. The [`AllocationCreateDesc`] has the
    ///    [`ResourceAllocationDesc::name`], [`ResourceAllocationDesc::location`],
    ///    [`ResourceAllocationDesc::mapped`] and
    ///    [`ResourceAllocationDesc::memory_allocate_flags`] of `desc`, and:
    ///    - the memory requirements of the image, with an alignment raised to
    ///      [`ResourceAllocationDesc::alignment`] if that is larger;
    ///    - [`AllocationCreateDesc::linear`] `true` for [`vk::ImageTiling::LINEAR`] and
    ///      for [`vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT`] with the modifier
    ///      `DRM_FORMAT_MOD_LINEAR` (0), and `false` otherwise, which is how Vulkan
    ///      defines a linear image;
    ///    - [`AllocationScheme::DedicatedImage`] with the image if the implementation
    ///      sets `requiresDedicatedAllocation` or `prefersDedicatedAllocation`, and
    ///      [`AllocationScheme::GpuAllocatorManaged`] otherwise;
    ///    - as [`AllocationCreateDesc::export_handle_types`], the `handleTypes` of the
    ///      `VkExternalMemoryImageCreateInfo` in the `pNext` chain of `info`, or none if
    ///      the chain has no such structure.
    ///
    ///    No usage or create flag of an image needs
    ///    [`vk::MemoryAllocateFlags::DEVICE_ADDRESS`] except the capture and replay flag
    ///    refused below, so the flags are those of `desc`.
    /// 5. Binds the image to [`Allocation::memory`] at [`Allocation::offset`] with
    ///    `vkBindImageMemory`.
    ///
    /// The image and the allocation go together from then on: pass both to
    /// [`Allocator::destroy_image`] when the device no longer uses the image.
    ///
    /// # What this function does not create
    ///
    /// The effective create flags are `info.flags`, or the `flags` of a
    /// `VkImageCreateFlags2CreateInfoKHR` in the chain if there is one, which Vulkan then
    /// uses instead. These images are refused with
    /// [`AllocationError::InvalidAllocationCreateDesc`]:
    ///
    /// - sparse images (`SPARSE_BINDING`, `SPARSE_RESIDENCY` or `SPARSE_ALIASED` in the
    ///   effective flags), whose memory is bound later in ranges with `vkQueueBindSparse`;
    /// - disjoint images (`DISJOINT`), whose planes are bound to memory each by itself
    ///   with `vkBindImageMemory2`;
    /// - images created for capture and replay of descriptors
    ///   (`DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT`, also named
    ///   `DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT`), whose memory needs
    ///   `DEVICE_ADDRESS_CAPTURE_REPLAY` and, on replay, an opaque capture address that
    ///   the allocator cannot request;
    /// - images whose chain has a `VkExternalMemoryImageCreateInfo` with
    ///   `ANDROID_HARDWARE_BUFFER_ANDROID`, whose dedicated memory Vulkan requires to be
    ///   allocated with an `allocationSize` of 0, which the allocator does not do;
    /// - images whose chain has a `VkExternalMemoryImageCreateInfoNV` with handle types,
    ///   or a `VkDedicatedAllocationImageCreateInfoNV` with `dedicatedAllocation` set,
    ///   whose memory needs structures of `VK_NV_external_memory` and
    ///   `VK_NV_dedicated_allocation` that the allocator cannot chain;
    /// - images whose chain has a `VkImageSwapchainCreateInfoKHR` with a swapchain, which
    ///   are bound to the memory of the swapchain;
    /// - images whose chain has a `VkExternalFormatANDROID`, `VkExternalFormatQNX` or
    ///   `VkExternalFormatOHOS` with an external format other than 0, a
    ///   `VkBufferCollectionImageCreateInfoFUCHSIA`, a `VkImportMetalTextureInfoEXT` or a
    ///   `VkImportMetalIOSurfaceInfoEXT`, whose memory comes from the object the image is
    ///   imported from.
    ///
    /// Images for memory that is imported, such as a dma-buf received from another
    /// process, are bound to memory allocated with `vkAllocateMemory` directly. The handle
    /// types of a `VkExternalMemoryImageCreateInfo` make the new memory exportable as
    /// those types. A protected image (`PROTECTED`) is created as any other, with the
    /// location that `desc` gives (see the safety section).
    ///
    /// # Errors
    ///
    /// When several of these apply, the one listed first is returned:
    ///
    /// 1. [`AllocationError::InvalidAllocationCreateDesc`] for the images listed above,
    ///    if [`ResourceAllocationDesc::alignment`] is neither 0 nor a power of two, or
    ///    if [`ResourceAllocationDesc::memory_allocate_flags`] or
    ///    [`AllocatorCreateDesc::memory_allocate_flags`][allocator flags]
    ///    contain [`vk::MemoryAllocateFlags::DEVICE_MASK`]. It is returned before any
    ///    Vulkan call.
    /// 2. [`AllocationError::Vulkan`] with the result if `vkCreateImage` fails.
    /// 3. [`AllocationError::Vulkan`] with the result if
    ///    `vkGetImageDrmFormatModifierPropertiesEXT` fails.
    /// 4. The errors of [`Allocator::allocate`] for the description above: in
    ///    particular [`AllocationError::NoCompatibleMemoryType`], which images with
    ///    optimal tiling can get for a host-visible location,
    ///    [`AllocationError::OutOfMemory`], [`AllocationError::Vulkan`] if
    ///    `vkAllocateMemory` fails otherwise, and the errors of mapping the allocation
    ///    when [`ResourceAllocationDesc::mapped`] is `true`, in the order that
    ///    [`Allocator::allocate`] lists them.
    /// 5. [`AllocationError::Vulkan`] with the result if `vkBindImageMemory` fails.
    ///
    /// Nothing is left when an error is returned: the image, if it was created, is
    /// destroyed with `vkDestroyImage`, and the allocation, if it was made, is freed as
    /// [`Allocator::free`] frees it, so a block allocated for it is freed with
    /// `vkFreeMemory` or kept empty. As for [`Allocator::allocate`], the empty blocks of
    /// a heap in which `vkAllocateMemory` ran out of memory may have been freed.
    ///
    /// # Panics
    ///
    /// Panics before any Vulkan call if `vkGetImageMemoryRequirements2` is not loaded,
    /// as on a device whose Vulkan version is below 1.1, or if `info.tiling` is
    /// [`vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT`] and
    /// `vkGetImageDrmFormatModifierPropertiesEXT` is not loaded, as on a device created
    /// without `VK_EXT_image_drm_format_modifier` (see the safety section).
    ///
    /// Panics when [`Allocator::allocate`] panics, because no identifier is left for a
    /// new allocation after 2<sup>64</sup> - 2 allocations in the process. The image is
    /// created by then and is not destroyed.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed, and
    ///   [`AllocatorCreateDesc::memory_properties`](crate::AllocatorCreateDesc::memory_properties)
    ///   must be those of its physical device.
    /// - The device must support Vulkan 1.1: the `apiVersion` of its physical device and
    ///   the `VkApplicationInfo::apiVersion` of its instance must both be 1.1 or higher.
    ///   `vkGetImageMemoryRequirements2` and `VkMemoryDedicatedRequirements`, which this
    ///   function uses, are Vulkan 1.1. On an older device, create the image, query its
    ///   requirements and allocate with [`Allocator::allocate`].
    /// - `info` must be valid for `vkCreateImage`, including every structure of its
    ///   `pNext` chain, which this function reads. With
    ///   [`vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT`], that includes the
    ///   `VK_EXT_image_drm_format_modifier` extension, whose
    ///   `vkGetImageDrmFormatModifierPropertiesEXT` this function calls. Structures of
    ///   the chain that are not named here are passed to `vkCreateImage` as they are;
    ///   they must not require anything of the memory of the image that its memory
    ///   requirements do not state, such as memory that is imported or allocated with a
    ///   structure in the chain of `vkAllocateMemory`.
    /// - The `vkAllocateMemory` calls must be valid. The flags of the allocation, from
    ///   [`ResourceAllocationDesc::memory_allocate_flags`] and from
    ///   [`AllocatorCreateDesc::memory_allocate_flags`][allocator flags],
    ///   need the matching device features: `DEVICE_ADDRESS` needs the
    ///   `bufferDeviceAddress` feature, and `DEVICE_ADDRESS_CAPTURE_REPLAY` needs
    ///   `bufferDeviceAddressCaptureReplay` and `DEVICE_ADDRESS` in the same flags.
    ///   Flags with `ZERO_INITIALIZE_EXT` must not be given to a memory type with
    ///   `PROTECTED`. A location that names `DEVICE_COHERENT_AMD` or
    ///   `DEVICE_UNCACHED_AMD` needs the `deviceCoherentMemory` feature, and one that
    ///   names `PROTECTED` the `protectedMemory` feature. The handle types of a
    ///   `VkExternalMemoryImageCreateInfo` in the chain must be exportable together for
    ///   this image (as `vkGetPhysicalDeviceImageFormatProperties2` reports) from every
    ///   memory type that may be chosen, with the extensions for them enabled. If the
    ///   allocator does not know the size of the memory heap of such a type (see
    ///   [`Allocator::allocate`]), the size of the image's memory must not be larger than
    ///   that heap. The number of `VkDeviceMemory` objects on the device must stay below
    ///   `maxMemoryAllocationCount`; each block of the allocator counts, also an empty
    ///   block that it keeps.
    /// - An image created with `PROTECTED` in its effective create flags must be given a
    ///   location that requires `PROTECTED`, and any other image a location that names
    ///   `PROTECTED` neither as required nor as preferred, because Vulkan binds protected
    ///   images only to protected memory and other images only to other memory.
    /// - On a logical device that represents more than one physical device,
    ///   [`ResourceAllocationDesc::mapped`] must be `false`, and the allocation must not
    ///   be mapped later, if its memory type belongs to a heap with
    ///   `VK_MEMORY_HEAP_MULTI_INSTANCE_BIT`.
    /// - When `vkAllocateMemory` runs out of memory, the empty blocks of the heap are
    ///   freed with `vkFreeMemory`, as for [`Allocator::allocate`]: every submitted
    ///   command that refers to memory of an empty block must have completed.
    /// - The allocator holds `VkDeviceMemory` once this returns. It must be dropped
    ///   before the device is destroyed, unless it holds no block by then, and when it is
    ///   dropped, every submitted command that refers to its memory must have completed;
    ///   the images bound to that memory and pointers obtained by mapping it must not be
    ///   used afterwards (see the [crate documentation](crate#order-of-destruction)).
    /// - The returned image is bound to the allocation, and must not be bound again.
    ///
    /// [allocator flags]: crate::AllocatorCreateDesc::memory_allocate_flags
    pub unsafe fn create_image(
        &mut self,
        info: &vk::ImageCreateInfo<'_>,
        desc: ResourceAllocationDesc<'_>,
    ) -> Result<(vk::Image, Allocation), AllocationError> {
        let device = self.device().clone();
        assert!(
            device.fp().get_image_memory_requirements2.is_some(),
            "vkGetImageMemoryRequirements2 is not loaded"
        );
        assert!(
            info.tiling != vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT
                || device
                    .fp()
                    .get_image_drm_format_modifier_properties_ext
                    .is_some(),
            "vkGetImageDrmFormatModifierPropertiesEXT is not loaded"
        );
        let facts = unsafe { image_facts(info) };
        if !facts.accepted || !self.accepts(&desc) {
            return Err(AllocationError::InvalidAllocationCreateDesc);
        }
        let image = unsafe { device.create_image(info, None) }.map_err(AllocationError::Vulkan)?;
        let linear = match unsafe { is_linear_image(&device, image, info.tiling) } {
            Ok(linear) => linear,
            Err(result) => {
                unsafe { device.destroy_image(image, None) };
                return Err(AllocationError::Vulkan(result));
            }
        };
        let (requirements, dedicated) = unsafe { image_requirements(&device, image) };
        let allocation_desc = AllocationCreateDesc {
            memory_allocate_flags: desc.memory_allocate_flags,
            export_handle_types: facts.export_handle_types,
            allocation_scheme: if dedicated {
                AllocationScheme::DedicatedImage(image)
            } else {
                AllocationScheme::GpuAllocatorManaged
            },
            ..allocation_desc(&desc, requirements, linear)
        };
        let allocation = unsafe {
            self.allocate_and_bind(
                allocation_desc,
                |allocation| {
                    device.bind_image_memory(image, allocation.memory(), allocation.offset())
                },
                || device.destroy_image(image, None),
            )
        }?;
        Ok((image, allocation))
    }

    /// Frees the allocation of a buffer made by [`Allocator::create_buffer`], then
    /// destroys the buffer with `vkDestroyBuffer`.
    ///
    /// The allocation is freed as [`Allocator::free`] frees it. The memory is freed
    /// before the buffer is destroyed, which Vulkan allows: "Memory can be freed whilst
    /// still bound to resources, but those resources must not be used afterwards"
    /// (`vkFreeMemory`). So a failure to free leaves both as they were.
    ///
    /// It also takes a buffer that the caller created and bound to `allocation` itself.
    ///
    /// # Errors
    ///
    /// The errors of [`Allocator::free`]: a [`FreeError`] if the allocator does not hold
    /// the allocation, or if it still has a mapping made with
    /// [`Allocator::map_memory`]. Nothing is freed then, and the buffer is not
    /// destroyed: [`FreeError::allocation`] hands the allocation back, so that the cause
    /// can be removed and both passed to this function again.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - `buffer` must be a buffer of that device bound to `allocation`, or
    ///   [`vk::Buffer::null`], and must not be destroyed elsewhere.
    /// - The device must have finished every use of the buffer and of the memory: every
    ///   submitted command that refers to the buffer, directly or through a buffer view,
    ///   or to another resource bound to the allocation must have completed.
    /// - The buffer, other resources bound to the allocation, and pointers obtained by
    ///   mapping it, must not be used after this returns `Ok`.
    pub unsafe fn destroy_buffer(
        &mut self,
        buffer: vk::Buffer,
        allocation: Allocation,
    ) -> Result<(), FreeError> {
        unsafe {
            self.free(allocation)?;
            self.device().destroy_buffer(buffer, None);
        }
        Ok(())
    }

    /// Frees the allocation of an image made by [`Allocator::create_image`], then
    /// destroys the image with `vkDestroyImage`.
    ///
    /// The allocation is freed as [`Allocator::free`] frees it. The memory is freed
    /// before the image is destroyed, which Vulkan allows: "Memory can be freed whilst
    /// still bound to resources, but those resources must not be used afterwards"
    /// (`vkFreeMemory`). So a failure to free leaves both as they were.
    ///
    /// It also takes an image that the caller created and bound to `allocation` itself.
    ///
    /// # Errors
    ///
    /// The errors of [`Allocator::free`]: a [`FreeError`] if the allocator does not hold
    /// the allocation, or if it still has a mapping made with
    /// [`Allocator::map_memory`]. Nothing is freed then, and the image is not destroyed:
    /// [`FreeError::allocation`] hands the allocation back, so that the cause can be
    /// removed and both passed to this function again.
    ///
    /// # Safety
    ///
    /// - The device the allocator was created with must not have been destroyed.
    /// - `image` must be an image of that device bound to `allocation`, or
    ///   [`vk::Image::null`], and must not be destroyed elsewhere.
    /// - The device must have finished every use of the image and of the memory: every
    ///   submitted command that refers to the image, directly or through an image view,
    ///   or to another resource bound to the allocation must have completed.
    /// - The image, other resources bound to the allocation, and pointers obtained by
    ///   mapping it, must not be used after this returns `Ok`.
    pub unsafe fn destroy_image(
        &mut self,
        image: vk::Image,
        allocation: Allocation,
    ) -> Result<(), FreeError> {
        unsafe {
            self.free(allocation)?;
            self.device().destroy_image(image, None);
        }
        Ok(())
    }

    fn accepts(&self, desc: &ResourceAllocationDesc<'_>) -> bool {
        (desc.alignment == 0 || desc.alignment.is_power_of_two())
            && !(desc.memory_allocate_flags | self.memory_allocate_flags())
                .contains(vk::MemoryAllocateFlags::DEVICE_MASK)
    }

    unsafe fn allocate_and_bind(
        &mut self,
        desc: AllocationCreateDesc<'_>,
        bind: impl FnOnce(&Allocation) -> vk::Result<()>,
        destroy: impl FnOnce(),
    ) -> Result<Allocation, AllocationError> {
        let allocation = match unsafe { self.allocate(desc) } {
            Ok(allocation) => allocation,
            Err(error) => {
                destroy();
                return Err(error);
            }
        };
        match bind(&allocation) {
            Ok(()) => Ok(allocation),
            Err(result) => {
                let freed = unsafe { self.free(allocation) };
                debug_assert!(
                    freed.is_ok(),
                    "an allocation that this call made holds no map_memory reference, so it is \
                     freed"
                );
                destroy();
                Err(AllocationError::Vulkan(result))
            }
        }
    }
}

fn allocation_desc<'a>(
    desc: &ResourceAllocationDesc<'a>,
    mut requirements: vk::MemoryRequirements,
    linear: bool,
) -> AllocationCreateDesc<'a> {
    requirements.alignment = requirements.alignment.max(desc.alignment);
    AllocationCreateDesc {
        name: desc.name,
        mapped: desc.mapped,
        ..AllocationCreateDesc::new(requirements, desc.location, linear)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_resource_description_maps_host_locations_and_adds_nothing_else() {
        for (location, mapped) in [
            (MemoryLocation::GpuOnly, false),
            (MemoryLocation::CpuToGpu, true),
            (MemoryLocation::GpuToCpu, true),
            (
                MemoryLocation::custom(
                    vk::MemoryPropertyFlags::DEVICE_LOCAL,
                    vk::MemoryPropertyFlags::HOST_VISIBLE,
                ),
                false,
            ),
            (
                MemoryLocation::custom(
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                    vk::MemoryPropertyFlags::empty(),
                ),
                false,
            ),
        ] {
            let desc = ResourceAllocationDesc::new(location);
            assert_eq!(desc.name, None, "name for {location:?}");
            assert_eq!(desc.location, location, "location for {location:?}");
            assert_eq!(desc.mapped, mapped, "mapped for {location:?}");
            assert_eq!(
                desc.memory_allocate_flags,
                vk::MemoryAllocateFlags::empty(),
                "memory_allocate_flags for {location:?}"
            );
            assert_eq!(desc.alignment, 1, "alignment for {location:?}");
        }
    }

    unsafe extern "system" fn no_commands(
        _device: vk::Device,
        _name: *const std::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction {
        None
    }

    fn allocator_without_commands() -> Allocator {
        let handle = vk::Device::null();
        let device = Device::from_raw(handle, unsafe {
            vk::DeviceFn::load(Some(no_commands), handle)
        });
        let mut memory_properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 1,
            ..Default::default()
        };
        memory_properties.memory_types[0].property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        Allocator::new(crate::AllocatorCreateDesc {
            device: &device,
            memory_properties,
            non_coherent_atom_size: 1,
            block_size: 4096,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        })
    }

    #[test]
    #[should_panic(expected = "vkGetBufferMemoryRequirements2 is not loaded")]
    fn a_buffer_is_not_created_on_a_device_without_the_vulkan_1_1_query() {
        let mut allocator = allocator_without_commands();
        let _ = unsafe {
            allocator.create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(64)
                    .usage(vk::BufferUsageFlags::STORAGE_BUFFER),
                ResourceAllocationDesc::new(MemoryLocation::GpuOnly),
            )
        };
    }

    #[test]
    #[should_panic(expected = "vkGetImageMemoryRequirements2 is not loaded")]
    fn an_image_is_not_created_on_a_device_without_the_vulkan_1_1_query() {
        let mut allocator = allocator_without_commands();
        let _ = unsafe {
            allocator.create_image(
                &vk::ImageCreateInfo::default(),
                ResourceAllocationDesc::new(MemoryLocation::GpuOnly),
            )
        };
    }

    #[test]
    fn a_new_resource_description_can_be_made_in_a_constant() {
        const DESC: ResourceAllocationDesc<'static> =
            ResourceAllocationDesc::new(MemoryLocation::GpuOnly);
        assert_eq!(DESC.location, MemoryLocation::GpuOnly);
    }
}
