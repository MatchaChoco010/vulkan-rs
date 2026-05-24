use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;

use vulkan::{Device, vk};

// A moderate default block size keeps the number of VkDeviceMemory objects low without making
// small examples reserve hundreds of MiB per memory type.
const DEFAULT_BLOCK_SIZE: u64 = 64 * 1024 * 1024;
// Do not split a tail smaller than this; tiny free blocks add fragmentation but are rarely useful
// for Vulkan resources because alignment requirements are commonly at least this large.
const MIN_SPLIT_SIZE: u64 = 256;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryLocation {
    GpuOnly,
    CpuToGpu,
    GpuToCpu,
    Custom {
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    },
}

impl MemoryLocation {
    pub const fn custom(
        required: vk::MemoryPropertyFlags,
        preferred: vk::MemoryPropertyFlags,
    ) -> Self {
        Self::Custom {
            required,
            preferred,
        }
    }

    fn required_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::CpuToGpu | Self::GpuToCpu => vk::MemoryPropertyFlags::HOST_VISIBLE,
            Self::Custom { required, .. } => required,
        }
    }

    fn preferred_flags(self) -> vk::MemoryPropertyFlags {
        match self {
            Self::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
            Self::CpuToGpu => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            }
            Self::GpuToCpu => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED
            }
            Self::Custom { preferred, .. } => preferred,
        }
    }
}

pub struct AllocatorCreateDesc<'a> {
    pub device: &'a Device,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties<'static>,
    pub non_coherent_atom_size: u64,
    pub buffer_image_granularity: u64,
    pub block_size: u64,
}

impl<'a> AllocatorCreateDesc<'a> {
    pub fn new(
        device: &'a Device,
        memory_properties: vk::PhysicalDeviceMemoryProperties<'static>,
        physical_device_properties: vk::PhysicalDeviceProperties<'static>,
    ) -> Self {
        Self {
            device,
            memory_properties,
            non_coherent_atom_size: physical_device_properties.limits.non_coherent_atom_size,
            buffer_image_granularity: physical_device_properties.limits.buffer_image_granularity,
            block_size: DEFAULT_BLOCK_SIZE,
        }
    }
}

#[derive(Copy, Clone)]
pub struct AllocationCreateDesc<'a> {
    pub name: Option<&'a str>,
    pub requirements: vk::MemoryRequirements<'static>,
    pub location: MemoryLocation,
    pub linear: bool,
    pub memory_allocate_flags: vk::MemoryAllocateFlags,
    pub allocation_scheme: AllocationScheme,
}

impl fmt::Debug for AllocationCreateDesc<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AllocationCreateDesc")
            .field("name", &self.name)
            .field("requirements.size", &self.requirements.size)
            .field("requirements.alignment", &self.requirements.alignment)
            .field(
                "requirements.memory_type_bits",
                &self.requirements.memory_type_bits,
            )
            .field("location", &self.location)
            .field("linear", &self.linear)
            .field("memory_allocate_flags", &self.memory_allocate_flags)
            .field("allocation_scheme", &self.allocation_scheme)
            .finish()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AllocationScheme {
    GpuAllocatorManaged,
    DedicatedBuffer(vk::Buffer),
    DedicatedImage(vk::Image),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Allocation {
    memory_type_index: u32,
    block_index: usize,
    allocation_id: u64,
    resource_class: AllocationResourceClass,
    memory: vk::DeviceMemory,
    offset: u64,
    size: u64,
}

impl Allocation {
    pub fn memory(&self) -> vk::DeviceMemory {
        self.memory
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn memory_type_index(&self) -> u32 {
        self.memory_type_index
    }
}

pub struct MappedAllocation<'a> {
    allocator: &'a mut Allocator,
    allocation: &'a Allocation,
    ptr: NonNull<u8>,
}

impl MappedAllocation<'_> {
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr().cast()
    }

    /// # Safety
    ///
    /// The caller must uphold Vulkan host/device synchronization for the mapped allocation and
    /// ensure no mutable host access aliases the returned bytes for the duration of the borrow.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr.as_ptr(), self.allocation.size() as usize) }
    }

    /// # Safety
    ///
    /// The caller must uphold Vulkan host/device synchronization for the mapped allocation and
    /// ensure the returned mutable byte slice is the only host access to this mapped range for
    /// the duration of the borrow.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            core::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.allocation.size() as usize)
        }
    }

    pub fn byte_range(&self, offset: usize, size: usize) -> &[u8] {
        let end = offset.checked_add(size).expect("mapped range end overflow");
        assert!(
            end <= self.allocation.size() as usize,
            "mapped byte range exceeds allocation size"
        );
        unsafe { &self.as_bytes()[offset..end] }
    }

    pub fn byte_range_mut(&mut self, offset: usize, size: usize) -> &mut [u8] {
        let end = offset.checked_add(size).expect("mapped range end overflow");
        assert!(
            end <= self.allocation.size() as usize,
            "mapped byte range exceeds allocation size"
        );
        unsafe { &mut self.as_bytes_mut()[offset..end] }
    }

    /// # Safety
    ///
    /// The caller must ensure all host writes that need to become visible to the device are
    /// complete and that Vulkan synchronization requirements for flushing this mapped range are
    /// satisfied.
    pub unsafe fn flush(&mut self) -> Result<(), AllocationError> {
        unsafe { self.allocator.flush_allocation(self.allocation) }
    }

    /// # Safety
    ///
    /// The caller must ensure device writes that need to become visible to the host are complete
    /// and that Vulkan synchronization requirements for invalidating this mapped range are
    /// satisfied.
    pub unsafe fn invalidate(&mut self) -> Result<(), AllocationError> {
        unsafe { self.allocator.invalidate_allocation(self.allocation) }
    }
}

impl Drop for MappedAllocation<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = self.allocator.unmap_memory(self.allocation);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorStats {
    pub total_block_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    pub allocation_count: usize,
    pub block_count: usize,
    pub memory_types: Vec<MemoryTypeStats>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeStats {
    pub memory_type_index: u32,
    pub total_block_bytes: u64,
    pub allocated_bytes: u64,
    pub free_bytes: u64,
    pub allocation_count: usize,
    pub block_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AllocatorSnapshot {
    pub stats: AllocatorStats,
    pub memory_types: Vec<MemoryTypeSnapshot>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MemoryTypeSnapshot {
    pub memory_type_index: u32,
    pub property_flags: vk::MemoryPropertyFlags,
    pub stats: MemoryTypeStats,
    pub blocks: Vec<MemoryBlockSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryBlockSnapshot {
    pub block_index: usize,
    pub resource_class: AllocationResourceClass,
    pub dedicated: bool,
    pub memory: vk::DeviceMemory,
    pub size: u64,
    pub free_bytes: u64,
    pub allocated_bytes: u64,
    pub largest_free_region_bytes: u64,
    pub free_region_count: usize,
    pub allocated_region_count: usize,
    pub regions: Vec<MemoryRegionSnapshot>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryRegionSnapshot {
    pub offset: u64,
    pub size: u64,
    pub state: MemoryRegionState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MemoryRegionState {
    Free,
    Allocated {
        allocation_id: u64,
        mapped_count: usize,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AllocationResourceClass {
    Unknown,
    Linear,
    Optimal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DedicatedResource {
    None,
    Buffer(vk::Buffer),
    Image(vk::Image),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    NoCompatibleMemoryType,
    OutOfMemory,
    Vulkan(vk::VkResult),
    InvalidAllocation,
}

impl std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCompatibleMemoryType => write!(f, "no compatible Vulkan memory type"),
            Self::OutOfMemory => write!(f, "Vulkan allocator is out of memory"),
            Self::Vulkan(err) => write!(f, "{err}"),
            Self::InvalidAllocation => write!(f, "invalid or already freed allocation"),
        }
    }
}

impl std::error::Error for AllocationError {}

impl From<vk::VkResult> for AllocationError {
    fn from(value: vk::VkResult) -> Self {
        Self::Vulkan(value)
    }
}

impl From<AllocationScheme> for DedicatedResource {
    fn from(value: AllocationScheme) -> Self {
        match value {
            AllocationScheme::GpuAllocatorManaged => Self::None,
            AllocationScheme::DedicatedBuffer(buffer) => Self::Buffer(buffer),
            AllocationScheme::DedicatedImage(image) => Self::Image(image),
        }
    }
}

pub struct Allocator {
    device_handle: vk::Device,
    device_fp: vk::DeviceFn,
    memory_properties: vk::PhysicalDeviceMemoryProperties<'static>,
    non_coherent_atom_size: u64,
    buffer_image_granularity: u64,
    block_size: u64,
    memory_types: Vec<MemoryTypeAllocator>,
}

impl Allocator {
    pub fn new(desc: AllocatorCreateDesc<'_>) -> Self {
        let memory_types = (0..desc.memory_properties.memory_type_count)
            .map(|index| MemoryTypeAllocator {
                memory_type_index: index,
                blocks: Vec::new(),
            })
            .collect();
        Self {
            device_handle: desc.device.handle(),
            device_fp: *desc.device.fp(),
            memory_properties: desc.memory_properties,
            non_coherent_atom_size: desc.non_coherent_atom_size.max(1),
            buffer_image_granularity: desc.buffer_image_granularity.max(1),
            block_size: desc.block_size.max(1),
            memory_types,
        }
    }

    fn device(&self) -> Device {
        Device::from_raw(self.device_handle, self.device_fp)
    }

    /// Allocates raw Vulkan device memory from this allocator.
    ///
    /// Create the Vulkan resource, query its memory requirements, pass those
    /// requirements here together with whether the resource is linear, then bind the
    /// returned allocation to the resource yourself. Use [`AllocationScheme`] to request
    /// dedicated allocation when the resource requires or benefits from it.
    ///
    /// # Safety
    ///
    /// The caller must bind and use the returned memory according to Vulkan's memory
    /// requirements, must account for dedicated-allocation requirements when using raw
    /// requirements, must not use the allocation after passing it to [`Allocator::free`],
    /// and must free every live allocation before the allocator is dropped.
    pub unsafe fn allocate(
        &mut self,
        desc: AllocationCreateDesc<'_>,
    ) -> Result<Allocation, AllocationError> {
        let resource_class = if desc.linear {
            AllocationResourceClass::Linear
        } else {
            AllocationResourceClass::Optimal
        };
        let memory_type_index = find_memory_type_index(
            &self.memory_properties,
            desc.requirements.memory_type_bits,
            desc.location.required_flags(),
            desc.location.preferred_flags(),
        )
        .ok_or(AllocationError::NoCompatibleMemoryType)?;
        let requirements =
            self.adjusted_requirements(memory_type_index, desc.requirements, resource_class)?;
        unsafe {
            self.allocate_from_type(
                memory_type_index,
                requirements,
                resource_class,
                desc.allocation_scheme.into(),
                desc.memory_allocate_flags,
            )
        }
    }

    /// Frees a raw allocation returned by [`Allocator::allocate`].
    ///
    /// # Safety
    ///
    /// The allocation must have been allocated by this allocator, must not currently be
    /// mapped through this allocator, and must not be used by any live Vulkan object.
    pub unsafe fn free(&mut self, allocation: Allocation) -> Result<(), AllocationError> {
        let memory = {
            let memory_type = self
                .memory_types
                .get_mut(allocation.memory_type_index as usize)
                .ok_or(AllocationError::InvalidAllocation)?;
            let slot = memory_type
                .blocks
                .get_mut(allocation.block_index)
                .ok_or(AllocationError::InvalidAllocation)?;
            let block = slot.as_mut().ok_or(AllocationError::InvalidAllocation)?;
            if block.memory != allocation.memory {
                return Err(AllocationError::InvalidAllocation);
            }
            block.validate_allocation(&allocation)?;
            if block.is_allocation_mapped(allocation.allocation_id) {
                return Err(AllocationError::InvalidAllocation);
            }
            block
                .suballocator
                .free(allocation.allocation_id)
                .ok_or(AllocationError::InvalidAllocation)?;
            if block.is_empty() {
                Some(
                    slot.take()
                        .expect("block exists after successful free")
                        .memory,
                )
            } else {
                None
            }
        };
        if let Some(memory) = memory {
            unsafe {
                self.device().free_memory(memory, None);
            }
        }
        Ok(())
    }

    /// Maps the allocation and returns a pointer to the allocation's start.
    ///
    /// Internally the containing `VkDeviceMemory` block is mapped once and reference
    /// counted per allocation.
    ///
    /// Prefer [`Allocator::map`] for normal use. It returns an RAII guard and cannot forget the
    /// matching unmap operation.
    ///
    /// # Safety
    ///
    /// The caller must only read/write bytes within the allocation range and must call
    /// [`Allocator::unmap_memory`] once for each successful call.
    pub unsafe fn map_memory(
        &mut self,
        allocation: &Allocation,
    ) -> Result<*mut c_void, AllocationError> {
        let device = self.device();
        let flags = self.allocation_memory_flags(allocation)?;
        if !flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
            return Err(AllocationError::InvalidAllocation);
        }
        let block = self.block_mut(allocation)?;
        let offset =
            usize::try_from(allocation.offset()).map_err(|_| AllocationError::InvalidAllocation)?;
        if block.mapped_ptr.is_none() {
            let ptr = unsafe {
                device.map_memory(
                    allocation.memory(),
                    0,
                    block.size,
                    vk::MemoryMapFlags::empty(),
                )
            }?;
            let Some(ptr) = NonNull::new(ptr) else {
                unsafe {
                    device.unmap_memory(allocation.memory());
                }
                return Err(AllocationError::InvalidAllocation);
            };
            block.mapped_ptr = Some(ptr);
        }
        let ptr = block.mapped_ptr.ok_or(AllocationError::InvalidAllocation)?;
        let map_count = block
            .map_count
            .checked_add(1)
            .ok_or(AllocationError::OutOfMemory)?;
        block.mark_mapped(allocation.allocation_id)?;
        block.map_count = map_count;
        Ok(unsafe { ptr.as_ptr().cast::<u8>().add(offset).cast() })
    }

    /// Maps the allocation and returns a guard that unmaps it on drop.
    ///
    /// # Safety
    ///
    /// The same safety requirements as [`Allocator::map_memory`] apply.
    pub unsafe fn map<'b>(
        &'b mut self,
        allocation: &'b Allocation,
    ) -> Result<MappedAllocation<'b>, AllocationError> {
        let ptr = NonNull::new(unsafe { self.map_memory(allocation) }?.cast::<u8>())
            .ok_or(AllocationError::InvalidAllocation)?;
        Ok(MappedAllocation {
            allocator: self,
            allocation,
            ptr,
        })
    }

    /// Releases one map reference for an allocation.
    ///
    /// # Safety
    ///
    /// `allocation` must have a currently active mapping created by this allocator, and
    /// no pointer returned by the matching map call may be used after this call releases
    /// the final reference for the underlying memory block.
    pub unsafe fn unmap_memory(&mut self, allocation: &Allocation) -> Result<(), AllocationError> {
        let device = self.device();
        let block = self.block_mut(allocation)?;
        if block.map_count == 0 {
            return Err(AllocationError::InvalidAllocation);
        }
        block.unmark_mapped(allocation.allocation_id)?;
        block.map_count -= 1;
        if block.map_count == 0 {
            unsafe {
                device.unmap_memory(allocation.memory());
            }
            block.mapped_ptr = None;
        }
        Ok(())
    }

    pub fn memory_type_flags(&self, memory_type_index: u32) -> Option<vk::MemoryPropertyFlags> {
        if memory_type_index >= self.memory_properties.memory_type_count {
            return None;
        }
        Some(self.memory_properties.memory_types[memory_type_index as usize].property_flags)
    }

    pub fn allocation_memory_flags(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MemoryPropertyFlags, AllocationError> {
        self.block(allocation)?;
        self.memory_type_flags(allocation.memory_type_index())
            .ok_or(AllocationError::InvalidAllocation)
    }

    /// Flushes a mapped allocation's non-coherent memory range.
    ///
    /// # Safety
    ///
    /// The allocation must currently be mapped through this allocator. The caller remains
    /// responsible for Vulkan synchronization and for calling this only when flushing is
    /// valid for the memory type and usage.
    pub unsafe fn flush_allocation(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device().flush_mapped_memory_ranges(&[range]) }?)
    }

    /// Invalidates a mapped allocation's non-coherent memory range.
    ///
    /// # Safety
    ///
    /// The allocation must currently be mapped through this allocator. The caller remains
    /// responsible for Vulkan synchronization and for calling this only when invalidating
    /// is valid for the memory type and usage.
    pub unsafe fn invalidate_allocation(
        &self,
        allocation: &Allocation,
    ) -> Result<(), AllocationError> {
        let range = self.mapped_range(allocation)?;
        Ok(unsafe { self.device().invalidate_mapped_memory_ranges(&[range]) }?)
    }

    pub fn stats(&self) -> AllocatorStats {
        stats_from_memory_types(&self.memory_types)
    }

    pub fn snapshot(&self) -> AllocatorSnapshot {
        snapshot_from_memory_types(&self.memory_types, &self.memory_properties)
    }

    pub fn live_allocation_count(&self) -> usize {
        self.memory_types
            .iter()
            .flat_map(|memory_type| memory_type.blocks.iter().flatten())
            .map(|block| block.suballocator.stats().allocation_count)
            .sum()
    }

    pub fn is_empty(&self) -> bool {
        self.live_allocation_count() == 0
    }

    unsafe fn allocate_from_type(
        &mut self,
        memory_type_index: u32,
        requirements: vk::MemoryRequirements<'static>,
        resource_class: AllocationResourceClass,
        dedicated_resource: DedicatedResource,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<Allocation, AllocationError> {
        if requirements.size == 0 {
            return Err(AllocationError::OutOfMemory);
        }
        if dedicated_resource == DedicatedResource::None {
            let memory_type = self
                .memory_types
                .get_mut(memory_type_index as usize)
                .ok_or(AllocationError::NoCompatibleMemoryType)?;
            for (block_index, block) in memory_type
                .blocks
                .iter_mut()
                .enumerate()
                .filter_map(|(index, block)| block.as_mut().map(|block| (index, block)))
            {
                if block.resource_class != resource_class {
                    continue;
                }
                if let Some(suballocation) = block
                    .suballocator
                    .allocate(requirements.size, requirements.alignment.max(1))
                {
                    return Ok(Allocation {
                        memory_type_index,
                        block_index,
                        allocation_id: suballocation.id,
                        resource_class,
                        memory: block.memory,
                        offset: suballocation.offset,
                        size: suballocation.size,
                    });
                }
            }
        }

        let block_size = if dedicated_resource == DedicatedResource::None {
            self.block_size.max(requirements.size)
        } else {
            requirements.size
        };
        let memory = unsafe {
            self.allocate_device_memory(
                block_size,
                memory_type_index,
                dedicated_resource,
                memory_allocate_flags,
            )
        }?;
        let mut block = MemoryBlock {
            memory,
            size: block_size,
            resource_class,
            dedicated: dedicated_resource != DedicatedResource::None,
            suballocator: TlsfSuballocator::new(block_size),
            mapped_ptr: None,
            map_count: 0,
            mapped_allocations: HashMap::new(),
        };
        let Some(suballocation) = block
            .suballocator
            .allocate(requirements.size, requirements.alignment.max(1))
        else {
            unsafe {
                self.device().free_memory(memory, None);
            }
            return Err(AllocationError::OutOfMemory);
        };
        let memory_type = self
            .memory_types
            .get_mut(memory_type_index as usize)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        let block_index = if let Some(index) = memory_type.blocks.iter().position(Option::is_none) {
            memory_type.blocks[index] = Some(block);
            index
        } else {
            let index = memory_type.blocks.len();
            memory_type.blocks.push(Some(block));
            index
        };
        Ok(Allocation {
            memory_type_index,
            block_index,
            allocation_id: suballocation.id,
            resource_class,
            memory,
            offset: suballocation.offset,
            size: suballocation.size,
        })
    }

    unsafe fn allocate_device_memory(
        &self,
        block_size: u64,
        memory_type_index: u32,
        dedicated_resource: DedicatedResource,
        memory_allocate_flags: vk::MemoryAllocateFlags,
    ) -> Result<vk::DeviceMemory, AllocationError> {
        match dedicated_resource {
            DedicatedResource::None => {
                let base = vk::MemoryAllocateInfo::default()
                    .allocation_size(block_size)
                    .memory_type_index(memory_type_index);
                if memory_allocate_flags.is_empty() {
                    return Ok(unsafe { self.device().allocate_memory(&base, None) }?);
                }
                let mut flags = vk::MemoryAllocateFlagsInfo::default().flags(memory_allocate_flags);
                let allocate_info = base.push_next(&mut flags);
                Ok(unsafe { self.device().allocate_memory(&allocate_info, None) }?)
            }
            DedicatedResource::Buffer(buffer) => {
                let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().buffer(buffer);
                let base = vk::MemoryAllocateInfo::default()
                    .allocation_size(block_size)
                    .memory_type_index(memory_type_index);
                if memory_allocate_flags.is_empty() {
                    let allocate_info = base.push_next(&mut dedicated);
                    Ok(unsafe { self.device().allocate_memory(&allocate_info, None) }?)
                } else {
                    let mut flags =
                        vk::MemoryAllocateFlagsInfo::default().flags(memory_allocate_flags);
                    let allocate_info = base.push_next(&mut flags).push_next(&mut dedicated);
                    Ok(unsafe { self.device().allocate_memory(&allocate_info, None) }?)
                }
            }
            DedicatedResource::Image(image) => {
                let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
                let base = vk::MemoryAllocateInfo::default()
                    .allocation_size(block_size)
                    .memory_type_index(memory_type_index);
                if memory_allocate_flags.is_empty() {
                    let allocate_info = base.push_next(&mut dedicated);
                    Ok(unsafe { self.device().allocate_memory(&allocate_info, None) }?)
                } else {
                    let mut flags =
                        vk::MemoryAllocateFlagsInfo::default().flags(memory_allocate_flags);
                    let allocate_info = base.push_next(&mut flags).push_next(&mut dedicated);
                    Ok(unsafe { self.device().allocate_memory(&allocate_info, None) }?)
                }
            }
        }
    }

    fn block(&self, allocation: &Allocation) -> Result<&MemoryBlock, AllocationError> {
        let memory_type = self
            .memory_types
            .get(allocation.memory_type_index as usize)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = memory_type
            .blocks
            .get(allocation.block_index)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = block.as_ref().ok_or(AllocationError::InvalidAllocation)?;
        if block.memory != allocation.memory {
            return Err(AllocationError::InvalidAllocation);
        }
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    fn block_mut(&mut self, allocation: &Allocation) -> Result<&mut MemoryBlock, AllocationError> {
        let memory_type = self
            .memory_types
            .get_mut(allocation.memory_type_index as usize)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = memory_type
            .blocks
            .get_mut(allocation.block_index)
            .ok_or(AllocationError::InvalidAllocation)?;
        let block = block.as_mut().ok_or(AllocationError::InvalidAllocation)?;
        if block.memory != allocation.memory {
            return Err(AllocationError::InvalidAllocation);
        }
        block.validate_allocation(allocation)?;
        Ok(block)
    }

    fn mapped_range(
        &self,
        allocation: &Allocation,
    ) -> Result<vk::MappedMemoryRange<'static>, AllocationError> {
        let block = self.block(allocation)?;
        if !block.is_allocation_mapped(allocation.allocation_id) {
            return Err(AllocationError::InvalidAllocation);
        }
        let end = allocation
            .offset()
            .checked_add(allocation.size())
            .ok_or(AllocationError::InvalidAllocation)?;
        if end > block.size {
            return Err(AllocationError::InvalidAllocation);
        }
        let (offset, size) = mapped_range_bounds(
            allocation.offset(),
            allocation.size(),
            block.size,
            self.non_coherent_atom_size,
        )
        .ok_or(AllocationError::InvalidAllocation)?;
        Ok(vk::MappedMemoryRange::default()
            .memory(allocation.memory())
            .offset(offset)
            .size(size))
    }

    fn adjusted_requirements(
        &self,
        memory_type_index: u32,
        mut requirements: vk::MemoryRequirements<'static>,
        resource_class: AllocationResourceClass,
    ) -> Result<vk::MemoryRequirements<'static>, AllocationError> {
        let flags = self
            .memory_type_flags(memory_type_index)
            .ok_or(AllocationError::NoCompatibleMemoryType)?;
        requirements.alignment = effective_alignment(
            flags,
            requirements.alignment.max(1),
            self.non_coherent_atom_size,
            self.buffer_image_granularity,
            resource_class,
        );
        Ok(requirements)
    }
}

fn stats_from_memory_types(memory_types: &[MemoryTypeAllocator]) -> AllocatorStats {
    let mut stats = AllocatorStats::default();
    for memory_type in memory_types {
        let type_stats = memory_type.stats();
        if type_stats.block_count == 0 {
            continue;
        }
        stats.total_block_bytes += type_stats.total_block_bytes;
        stats.allocated_bytes += type_stats.allocated_bytes;
        stats.free_bytes += type_stats.free_bytes;
        stats.allocation_count += type_stats.allocation_count;
        stats.block_count += type_stats.block_count;
        stats.memory_types.push(type_stats);
    }
    stats
}

fn snapshot_from_memory_types(
    memory_types: &[MemoryTypeAllocator],
    memory_properties: &vk::PhysicalDeviceMemoryProperties<'_>,
) -> AllocatorSnapshot {
    let stats = stats_from_memory_types(memory_types);
    let mut snapshots = Vec::new();
    for memory_type in memory_types {
        let stats = memory_type.stats();
        if stats.block_count == 0 {
            continue;
        }
        let property_flags =
            memory_properties.memory_types[memory_type.memory_type_index as usize].property_flags;
        snapshots.push(memory_type.snapshot(property_flags, stats));
    }
    AllocatorSnapshot {
        stats,
        memory_types: snapshots,
    }
}

impl Drop for Allocator {
    fn drop(&mut self) {
        let mut live_allocations = 0;
        for memory_type in &mut self.memory_types {
            for block in memory_type.blocks.drain(..) {
                let Some(block) = block else {
                    continue;
                };
                if !block.is_empty() {
                    live_allocations += block.suballocator.stats().allocation_count;
                }
            }
        }
        if live_allocations != 0 && !std::thread::panicking() {
            panic!(
                "vulkan_alloc::Allocator dropped while {live_allocations} allocation(s) are still live"
            );
        }
    }
}

struct MemoryTypeAllocator {
    memory_type_index: u32,
    blocks: Vec<Option<MemoryBlock>>,
}

impl MemoryTypeAllocator {
    fn stats(&self) -> MemoryTypeStats {
        let mut stats = MemoryTypeStats {
            memory_type_index: self.memory_type_index,
            ..Default::default()
        };
        for block in self.blocks.iter().flatten() {
            let block_stats = block.suballocator.stats();
            stats.total_block_bytes += block.size;
            stats.allocated_bytes += block_stats.allocated_bytes;
            stats.free_bytes += block_stats.free_bytes;
            stats.allocation_count += block_stats.allocation_count;
            stats.block_count += 1;
        }
        stats
    }

    fn snapshot(
        &self,
        property_flags: vk::MemoryPropertyFlags,
        stats: MemoryTypeStats,
    ) -> MemoryTypeSnapshot {
        MemoryTypeSnapshot {
            memory_type_index: self.memory_type_index,
            property_flags,
            stats,
            blocks: self
                .blocks
                .iter()
                .enumerate()
                .filter_map(|(index, block)| block.as_ref().map(|block| block.snapshot(index)))
                .collect(),
        }
    }
}

struct MemoryBlock {
    memory: vk::DeviceMemory,
    size: u64,
    resource_class: AllocationResourceClass,
    dedicated: bool,
    suballocator: TlsfSuballocator,
    mapped_ptr: Option<NonNull<c_void>>,
    map_count: usize,
    mapped_allocations: HashMap<u64, usize>,
}

impl MemoryBlock {
    fn is_empty(&self) -> bool {
        self.suballocator.is_wholly_free(self.size)
    }

    fn snapshot(&self, block_index: usize) -> MemoryBlockSnapshot {
        let regions = self.suballocator.region_snapshots(&self.mapped_allocations);
        let mut snapshot = MemoryBlockSnapshot {
            block_index,
            resource_class: self.resource_class,
            dedicated: self.dedicated,
            memory: self.memory,
            size: self.size,
            free_bytes: 0,
            allocated_bytes: 0,
            largest_free_region_bytes: 0,
            free_region_count: 0,
            allocated_region_count: 0,
            regions,
        };
        for region in &snapshot.regions {
            match region.state {
                MemoryRegionState::Free => {
                    snapshot.free_bytes += region.size;
                    snapshot.free_region_count += 1;
                    snapshot.largest_free_region_bytes =
                        snapshot.largest_free_region_bytes.max(region.size);
                }
                MemoryRegionState::Allocated { .. } => {
                    snapshot.allocated_bytes += region.size;
                    snapshot.allocated_region_count += 1;
                }
            }
        }
        snapshot
    }

    fn is_allocation_mapped(&self, allocation_id: u64) -> bool {
        self.mapped_allocations.contains_key(&allocation_id)
    }

    fn validate_allocation(&self, allocation: &Allocation) -> Result<(), AllocationError> {
        if self.resource_class != allocation.resource_class {
            return Err(AllocationError::InvalidAllocation);
        }
        if self
            .suballocator
            .allocation_offset(allocation.allocation_id)
            != Some(allocation.offset)
        {
            return Err(AllocationError::InvalidAllocation);
        }
        let block = self
            .suballocator
            .blocks
            .get(&allocation.offset)
            .ok_or(AllocationError::InvalidAllocation)?;
        if block.free || block.size != allocation.size {
            return Err(AllocationError::InvalidAllocation);
        }
        let end = allocation
            .offset
            .checked_add(allocation.size)
            .ok_or(AllocationError::InvalidAllocation)?;
        if end > self.size {
            return Err(AllocationError::InvalidAllocation);
        }
        Ok(())
    }

    fn mark_mapped(&mut self, allocation_id: u64) -> Result<(), AllocationError> {
        let count = self.mapped_allocations.entry(allocation_id).or_default();
        *count = count.checked_add(1).ok_or(AllocationError::OutOfMemory)?;
        Ok(())
    }

    fn unmark_mapped(&mut self, allocation_id: u64) -> Result<(), AllocationError> {
        let count = self
            .mapped_allocations
            .get_mut(&allocation_id)
            .ok_or(AllocationError::InvalidAllocation)?;
        *count -= 1;
        if *count == 0 {
            self.mapped_allocations.remove(&allocation_id);
        }
        Ok(())
    }
}

fn find_memory_type_index(
    properties: &vk::PhysicalDeviceMemoryProperties<'_>,
    memory_type_bits: u32,
    required: vk::MemoryPropertyFlags,
    preferred: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mut best = None;
    let mut best_score = 0;
    for index in 0..properties.memory_type_count {
        let type_bit = 1u32.checked_shl(index).unwrap_or(0);
        if memory_type_bits & type_bit == 0 {
            continue;
        }
        let flags = properties.memory_types[index as usize].property_flags;
        if !flags.contains(required) {
            continue;
        }
        let score = (flags.as_raw() & preferred.as_raw()).count_ones();
        if best.is_none() || score > best_score {
            best = Some(index);
            best_score = score;
        }
        if flags.contains(preferred) {
            return Some(index);
        }
    }
    best
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Suballocation {
    id: u64,
    offset: u64,
    size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SuballocatorStats {
    allocated_bytes: u64,
    free_bytes: u64,
    allocation_count: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Block {
    size: u64,
    free: bool,
}

#[derive(Debug, Clone)]
struct TlsfSuballocator {
    size: u64,
    blocks: BTreeMap<u64, Block>,
    free_bins: BTreeMap<u32, BTreeSet<u64>>,
    allocations: HashMap<u64, u64>,
    next_id: u64,
}

impl TlsfSuballocator {
    fn new(size: u64) -> Self {
        let mut this = Self {
            size,
            blocks: BTreeMap::new(),
            free_bins: BTreeMap::new(),
            allocations: HashMap::new(),
            next_id: 1,
        };
        this.insert_free_block(0, size);
        this
    }

    fn allocate(&mut self, requested_size: u64, alignment: u64) -> Option<Suballocation> {
        if requested_size == 0 || requested_size > self.size || self.next_id == u64::MAX {
            return None;
        }
        let alignment = alignment.max(1);
        let (block_offset, block, aligned_offset) =
            self.find_candidate_block(requested_size, alignment)?;
        let padding = aligned_offset.checked_sub(block_offset)?;

        self.remove_free_block(block_offset);
        self.blocks.remove(&block_offset);

        if padding > 0 {
            self.insert_free_block(block_offset, padding);
        }

        let allocation_offset = aligned_offset;
        let mut allocation_size = requested_size;
        self.blocks.insert(
            allocation_offset,
            Block {
                size: allocation_size,
                free: false,
            },
        );

        let used_end = allocation_offset.checked_add(allocation_size)?;
        let block_end = block_offset.checked_add(block.size)?;
        if block_end > used_end {
            let tail_size = block_end - used_end;
            if tail_size >= MIN_SPLIT_SIZE {
                self.insert_free_block(used_end, tail_size);
            } else if let Some(allocated) = self.blocks.get_mut(&allocation_offset) {
                allocated.size += tail_size;
                allocation_size += tail_size;
            }
        }

        let id = self.next_id;
        self.next_id += 1;
        self.allocations.insert(id, allocation_offset);
        Some(Suballocation {
            id,
            offset: allocation_offset,
            size: allocation_size,
        })
    }

    #[cfg(test)]
    fn can_allocate(&self, requested_size: u64, alignment: u64) -> bool {
        if requested_size == 0 || requested_size > self.size || self.next_id == u64::MAX {
            return false;
        }
        self.find_candidate_block(requested_size, alignment.max(1))
            .is_some()
    }

    fn find_candidate_block(
        &self,
        requested_size: u64,
        alignment: u64,
    ) -> Option<(u64, Block, u64)> {
        let min_bin = bin_index(requested_size);
        for (_, offsets) in self.free_bins.range(min_bin..) {
            for block_offset in offsets {
                let Some(block) = self.blocks.get(block_offset).copied() else {
                    continue;
                };
                if !block.free {
                    continue;
                }
                let Some(aligned_offset) = align_up(*block_offset, alignment) else {
                    continue;
                };
                let Some(padding) = aligned_offset.checked_sub(*block_offset) else {
                    continue;
                };
                let Some(needed) = padding.checked_add(requested_size) else {
                    continue;
                };
                if needed <= block.size {
                    return Some((*block_offset, block, aligned_offset));
                }
            }
        }
        None
    }

    fn allocation_offset(&self, id: u64) -> Option<u64> {
        self.allocations.get(&id).copied()
    }

    fn free(&mut self, id: u64) -> Option<()> {
        let offset = self.allocations.remove(&id)?;
        let block = self.blocks.remove(&offset)?;
        if block.free {
            return None;
        }

        let mut merged_offset = offset;
        let mut merged_size = block.size;

        if let Some((&prev_offset, prev_block)) = self.blocks.range(..offset).next_back()
            && prev_block.free
            && prev_offset.checked_add(prev_block.size) == Some(offset)
        {
            let prev = *prev_block;
            self.remove_free_block(prev_offset);
            self.blocks.remove(&prev_offset);
            merged_offset = prev_offset;
            merged_size += prev.size;
        }

        let next_offset = merged_offset.checked_add(merged_size)?;
        if let Some(next_block) = self.blocks.get(&next_offset).copied()
            && next_block.free
        {
            self.remove_free_block(next_offset);
            self.blocks.remove(&next_offset);
            merged_size += next_block.size;
        }

        self.insert_free_block(merged_offset, merged_size);
        Some(())
    }

    fn stats(&self) -> SuballocatorStats {
        let mut stats = SuballocatorStats {
            allocated_bytes: 0,
            free_bytes: 0,
            allocation_count: self.allocations.len(),
        };
        for block in self.blocks.values() {
            if block.free {
                stats.free_bytes += block.size;
            } else {
                stats.allocated_bytes += block.size;
            }
        }
        stats
    }

    fn is_wholly_free(&self, expected_size: u64) -> bool {
        self.allocations.is_empty()
            && self.blocks.len() == 1
            && self
                .blocks
                .get(&0)
                .is_some_and(|block| block.free && block.size == expected_size)
    }

    fn region_snapshots(
        &self,
        mapped_allocations: &HashMap<u64, usize>,
    ) -> Vec<MemoryRegionSnapshot> {
        let allocation_ids_by_offset = self
            .allocations
            .iter()
            .map(|(&id, &offset)| (offset, id))
            .collect::<HashMap<_, _>>();
        self.blocks
            .iter()
            .map(|(&offset, block)| {
                let state = if block.free {
                    MemoryRegionState::Free
                } else {
                    let allocation_id = allocation_ids_by_offset[&offset];
                    MemoryRegionState::Allocated {
                        allocation_id,
                        mapped_count: mapped_allocations.get(&allocation_id).copied().unwrap_or(0),
                    }
                };
                MemoryRegionSnapshot {
                    offset,
                    size: block.size,
                    state,
                }
            })
            .collect()
    }

    fn insert_free_block(&mut self, offset: u64, size: u64) {
        self.blocks.insert(offset, Block { size, free: true });
        self.free_bins
            .entry(bin_index(size))
            .or_default()
            .insert(offset);
    }

    fn remove_free_block(&mut self, offset: u64) {
        if let Some(block) = self.blocks.get(&offset) {
            let bin = bin_index(block.size);
            if let Some(offsets) = self.free_bins.get_mut(&bin) {
                offsets.remove(&offset);
                if offsets.is_empty() {
                    self.free_bins.remove(&bin);
                }
            }
        }
    }
}

fn bin_index(size: u64) -> u32 {
    63 - size.max(1).leading_zeros()
}

fn align_up(value: u64, alignment: u64) -> Option<u64> {
    if !alignment.is_power_of_two() {
        let rem = value % alignment;
        return if rem == 0 {
            Some(value)
        } else {
            value.checked_add(alignment - rem)
        };
    }
    value
        .checked_add(alignment - 1)
        .map(|x| x & !(alignment - 1))
}

fn align_down(value: u64, alignment: u64) -> u64 {
    let alignment = alignment.max(1);
    value - (value % alignment)
}

fn effective_alignment(
    flags: vk::MemoryPropertyFlags,
    requested_alignment: u64,
    non_coherent_atom_size: u64,
    buffer_image_granularity: u64,
    resource_class: AllocationResourceClass,
) -> u64 {
    let mut alignment = requested_alignment.max(1);
    if flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE)
        && !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
    {
        alignment = alignment.max(non_coherent_atom_size.max(1));
    }
    if resource_class == AllocationResourceClass::Unknown {
        alignment = alignment.max(buffer_image_granularity.max(1));
    }
    alignment
}

fn mapped_range_bounds(
    offset: u64,
    size: u64,
    block_size: u64,
    atom_size: u64,
) -> Option<(u64, u64)> {
    let end = offset.checked_add(size)?;
    if end > block_size {
        return None;
    }
    let atom = atom_size.max(1);
    let aligned_offset = align_down(offset, atom);
    let aligned_end = align_up(end, atom).unwrap_or(block_size).min(block_size);
    let range_size = if aligned_end == block_size {
        vk::WHOLE_SIZE
    } else {
        aligned_end.checked_sub(aligned_offset)?
    };
    Some((aligned_offset, range_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn memory_properties(
        flags: &[vk::MemoryPropertyFlags],
    ) -> vk::PhysicalDeviceMemoryProperties<'static> {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: flags.len() as u32,
            ..Default::default()
        };
        for (index, flags) in flags.iter().copied().enumerate() {
            properties.memory_types[index].property_flags = flags;
        }
        properties
    }

    fn check_invariants(allocator: &TlsfSuballocator) {
        let mut total = 0;
        let mut last_end = 0;
        let mut previous_free = false;
        for (&offset, block) in &allocator.blocks {
            assert_eq!(offset, last_end);
            assert!(block.size > 0);
            assert!(!(previous_free && block.free));
            if block.free {
                let bin = bin_index(block.size);
                assert!(
                    allocator
                        .free_bins
                        .get(&bin)
                        .is_some_and(|offsets| offsets.contains(&offset))
                );
            }
            previous_free = block.free;
            total += block.size;
            last_end = offset + block.size;
        }
        assert_eq!(total, allocator.size);

        for (&bin, offsets) in &allocator.free_bins {
            for offset in offsets {
                let block = allocator
                    .blocks
                    .get(offset)
                    .expect("free bin contains an offset with no block");
                assert!(block.free);
                assert_eq!(bin_index(block.size), bin);
            }
        }
    }

    #[test]
    fn allocates_aligned_block() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(128, 256).unwrap();
        assert_eq!(allocation.offset % 256, 0);
        assert!(allocation.size >= 128);
        check_invariants(&allocator);
    }

    #[test]
    fn rejects_zero_size_and_too_large_allocations() {
        let mut allocator = TlsfSuballocator::new(1024);
        let before = allocator.clone();
        assert!(allocator.allocate(0, 1).is_none());
        assert!(allocator.allocate(2048, 1).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);
        check_invariants(&allocator);
    }

    #[test]
    fn exact_fit_consumes_whole_block() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(1024, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.size, 1024);
        assert!(allocator.allocate(1, 1).is_none());
        let stats = allocator.stats();
        assert_eq!(stats.allocated_bytes, 1024);
        assert_eq!(stats.free_bytes, 0);
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn tiny_tail_is_absorbed_into_allocation() {
        let mut allocator = TlsfSuballocator::new(1024);
        let allocation = allocator.allocate(900, 1).unwrap();
        assert_eq!(allocation.size, 1024);
        assert_eq!(allocator.stats().free_bytes, 0);
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn alignment_prefix_is_split_and_remerged() {
        let mut allocator = TlsfSuballocator::new(2048);
        let first = allocator.allocate(60, 1).unwrap();
        let aligned = allocator.allocate(128, 256).unwrap();
        assert_eq!(aligned.offset, 256);
        assert!(allocator.blocks.contains_key(&60));
        allocator.free(first.id).unwrap();
        allocator.free(aligned.id).unwrap();
        assert_eq!(allocator.blocks.len(), 1);
        assert_eq!(allocator.stats().free_bytes, 2048);
        check_invariants(&allocator);
    }

    #[test]
    fn reuses_freed_space() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(256, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        allocator.free(a.id).unwrap();
        let c = allocator.allocate(128, 1).unwrap();
        assert_eq!(c.offset, a.offset);
        allocator.free(b.id).unwrap();
        allocator.free(c.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn merges_previous_and_next_blocks() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let b = allocator.allocate(128, 1).unwrap();
        let c = allocator.allocate(128, 1).unwrap();
        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        allocator.free(b.id).unwrap();
        assert_eq!(allocator.blocks.len(), 1);
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn detects_double_free() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        allocator.free(a.id).unwrap();
        assert!(allocator.free(a.id).is_none());
        check_invariants(&allocator);
    }

    #[test]
    fn invalid_free_does_not_change_stats() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let before = allocator.stats();
        assert!(allocator.free(a.id + 1).is_none());
        assert_eq!(allocator.stats(), before);
        allocator.free(a.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn reports_stats_after_fragmentation() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(128, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        allocator.free(a.id).unwrap();
        let stats = allocator.stats();
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.allocated_bytes, b.size);
        assert_eq!(stats.allocated_bytes + stats.free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn non_power_of_two_alignment_is_supported() {
        let mut allocator = TlsfSuballocator::new(4096);
        let first = allocator.allocate(17, 1).unwrap();
        let a = allocator.allocate(17, 192).unwrap();
        assert_eq!(a.offset % 192, 0);
        allocator.free(first.id).unwrap();
        allocator.free(a.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 4096);
        check_invariants(&allocator);
    }

    #[test]
    fn continues_after_alignment_unusable_candidate() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(32, 1).unwrap();
        let b = allocator.allocate(224, 1).unwrap();
        let c = allocator.allocate(32, 1).unwrap();
        allocator.free(b.id).unwrap();

        let allocation = allocator.allocate(200, 256).unwrap();
        assert!(allocation.offset >= c.offset + c.size);
        assert_eq!(allocation.offset % 256, 0);
        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, 1024);
        check_invariants(&allocator);
    }

    #[test]
    fn failed_fragmented_allocation_does_not_mutate_state() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(256, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        let c = allocator.allocate(256, 1).unwrap();
        let d = allocator.allocate(256, 1).unwrap();
        allocator.free(b.id).unwrap();
        allocator.free(d.id).unwrap();

        let before = allocator.clone();
        assert!(allocator.allocate(384, 1).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);

        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn failed_impossible_alignment_does_not_mutate_state() {
        let mut allocator = TlsfSuballocator::new(128);
        let first = allocator.allocate(1, 1).unwrap();
        let before = allocator.clone();
        assert!(allocator.allocate(127, 128).is_none());
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);
        allocator.free(first.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn stale_free_bin_entry_does_not_abort_search() {
        let mut allocator = TlsfSuballocator::new(1024);
        allocator
            .free_bins
            .entry(bin_index(64))
            .or_default()
            .insert(999);
        let allocation = allocator.allocate(64, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        allocator
            .free_bins
            .get_mut(&bin_index(64))
            .unwrap()
            .remove(&999);
        allocator.free(allocation.id).unwrap();
        check_invariants(&allocator);
    }

    #[test]
    fn randomized_model_stays_consistent() {
        let mut allocator = TlsfSuballocator::new(8192);
        let mut live = Vec::new();
        let mut seed = 0x1234_5678_u64;
        for _ in 0..2000 {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            if seed & 1 == 0 || live.is_empty() {
                let size = 1 + ((seed >> 8) % 512);
                let alignment = 1 << ((seed >> 20) % 8);
                if let Some(allocation) = allocator.allocate(size, alignment) {
                    assert_eq!(allocation.offset % alignment, 0);
                    live.push(allocation);
                }
            } else {
                let index = (seed as usize) % live.len();
                let allocation = live.swap_remove(index);
                allocator.free(allocation.id).unwrap();
            }
            check_invariants(&allocator);
            let stats = allocator.stats();
            assert_eq!(stats.allocated_bytes + stats.free_bytes, allocator.size);
            assert_eq!(stats.allocation_count, live.len());
        }
    }

    #[test]
    fn finds_preferred_memory_type_when_available() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b111,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(1));
    }

    #[test]
    fn preferred_memory_type_must_be_allowed_by_type_bits() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b001,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn falls_back_to_required_memory_type() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b01,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn rejects_incompatible_memory_type_bits() {
        let properties = memory_properties(&[vk::MemoryPropertyFlags::HOST_VISIBLE]);

        let index = find_memory_type_index(
            &properties,
            0,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
        );
        assert_eq!(index, None);
    }

    #[test]
    fn rejects_when_allowed_types_do_not_have_required_flags() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_COHERENT,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b11,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        assert_eq!(index, None);
    }

    #[test]
    fn finds_high_memory_type_bit() {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 32,
            ..Default::default()
        };
        properties.memory_types[31].property_flags = vk::MemoryPropertyFlags::DEVICE_LOCAL;

        let index = find_memory_type_index(
            &properties,
            1 << 31,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        );
        assert_eq!(index, Some(31));
    }

    #[test]
    fn gpu_to_cpu_prefers_cached_memory_over_uncached_coherent_memory() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);

        let index = find_memory_type_index(
            &properties,
            0b11,
            MemoryLocation::GpuToCpu.required_flags(),
            MemoryLocation::GpuToCpu.preferred_flags(),
        );
        assert_eq!(index, Some(1));
    }

    #[test]
    fn preferred_memory_type_score_counts_matching_flags() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);

        let preferred = vk::MemoryPropertyFlags::HOST_VISIBLE
            | vk::MemoryPropertyFlags::HOST_COHERENT
            | vk::MemoryPropertyFlags::HOST_CACHED;
        let index = find_memory_type_index(
            &properties,
            0b11,
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            preferred,
        );
        assert_eq!(index, Some(0));
    }

    #[test]
    fn memory_location_flags_match_intended_usage() {
        assert_eq!(
            MemoryLocation::GpuOnly.required_flags(),
            vk::MemoryPropertyFlags::DEVICE_LOCAL
        );
        assert_eq!(
            MemoryLocation::CpuToGpu.required_flags(),
            vk::MemoryPropertyFlags::HOST_VISIBLE
        );
        assert!(
            MemoryLocation::CpuToGpu
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        );
        assert!(
            MemoryLocation::GpuToCpu
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_CACHED)
        );

        let custom = MemoryLocation::custom(
            vk::MemoryPropertyFlags::HOST_VISIBLE,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT
                | vk::MemoryPropertyFlags::HOST_CACHED,
        );
        assert_eq!(
            custom.required_flags(),
            vk::MemoryPropertyFlags::HOST_VISIBLE
        );
        assert!(
            custom
                .preferred_flags()
                .contains(vk::MemoryPropertyFlags::HOST_CACHED)
        );
    }

    #[test]
    fn raw_allocation_desc_can_request_dedicated_resources() {
        let requirements = vk::MemoryRequirements {
            size: 1024,
            alignment: 256,
            memory_type_bits: 1,
            _marker: std::marker::PhantomData,
        };

        let buffer_desc = AllocationCreateDesc {
            name: Some("buffer"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedBuffer(vk::Buffer(7)),
        };
        assert_eq!(
            buffer_desc.allocation_scheme,
            AllocationScheme::DedicatedBuffer(vk::Buffer(7))
        );
        assert!(buffer_desc.linear);

        let image_desc = AllocationCreateDesc {
            name: Some("image"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: false,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            allocation_scheme: AllocationScheme::DedicatedImage(vk::Image(9)),
        };
        assert_eq!(
            image_desc.allocation_scheme,
            AllocationScheme::DedicatedImage(vk::Image(9))
        );
        assert!(!image_desc.linear);
    }

    #[test]
    fn non_coherent_host_visible_allocations_use_atom_alignment() {
        let non_coherent =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED;
        assert_eq!(
            effective_alignment(non_coherent, 16, 256, 1, AllocationResourceClass::Unknown),
            256
        );

        let coherent =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        assert_eq!(
            effective_alignment(coherent, 16, 256, 1, AllocationResourceClass::Unknown),
            16
        );

        let device_local = vk::MemoryPropertyFlags::DEVICE_LOCAL;
        assert_eq!(
            effective_alignment(device_local, 16, 256, 512, AllocationResourceClass::Optimal),
            16
        );
        assert_eq!(
            effective_alignment(device_local, 16, 256, 512, AllocationResourceClass::Unknown),
            512
        );

        let mut allocator = TlsfSuballocator::new(1024);
        let alignment =
            effective_alignment(non_coherent, 1, 256, 1, AllocationResourceClass::Unknown);
        let a = allocator.allocate(64, alignment).unwrap();
        let b = allocator.allocate(64, alignment).unwrap();
        assert_eq!(a.offset, 0);
        assert_eq!(b.offset, 256);
        check_invariants(&allocator);
    }

    #[test]
    fn memory_block_tracks_mapped_allocations_individually() {
        let mut block = MemoryBlock {
            memory: vk::DeviceMemory(31),
            size: 1024,
            resource_class: AllocationResourceClass::Unknown,
            dedicated: false,
            suballocator: TlsfSuballocator::new(1024),
            mapped_ptr: None,
            map_count: 0,
            mapped_allocations: HashMap::new(),
        };

        block.mark_mapped(1).unwrap();
        block.mark_mapped(1).unwrap();
        block.mark_mapped(2).unwrap();
        assert!(block.is_allocation_mapped(1));
        assert!(block.is_allocation_mapped(2));

        block.unmark_mapped(1).unwrap();
        assert!(block.is_allocation_mapped(1));
        block.unmark_mapped(1).unwrap();
        assert!(!block.is_allocation_mapped(1));
        assert!(block.is_allocation_mapped(2));
        block.unmark_mapped(2).unwrap();
        assert!(!block.is_allocation_mapped(2));
        assert_eq!(
            block.unmark_mapped(2),
            Err(AllocationError::InvalidAllocation)
        );
    }

    #[test]
    fn memory_block_validates_allocation_identity() {
        let mut block = MemoryBlock {
            memory: vk::DeviceMemory(32),
            size: 1024,
            resource_class: AllocationResourceClass::Optimal,
            dedicated: false,
            suballocator: TlsfSuballocator::new(1024),
            mapped_ptr: None,
            map_count: 0,
            mapped_allocations: HashMap::new(),
        };
        let suballocation = block.suballocator.allocate(128, 64).unwrap();
        let make_allocation = || Allocation {
            memory_type_index: 0,
            block_index: 0,
            allocation_id: suballocation.id,
            resource_class: AllocationResourceClass::Optimal,
            memory: vk::DeviceMemory(32),
            offset: suballocation.offset,
            size: suballocation.size,
        };
        let allocation = make_allocation();
        assert_eq!(block.validate_allocation(&allocation), Ok(()));

        let mut wrong_id = make_allocation();
        wrong_id.allocation_id += 1;
        assert_eq!(
            block.validate_allocation(&wrong_id),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_offset = make_allocation();
        wrong_offset.offset += 64;
        assert_eq!(
            block.validate_allocation(&wrong_offset),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_size = make_allocation();
        wrong_size.size += 64;
        assert_eq!(
            block.validate_allocation(&wrong_size),
            Err(AllocationError::InvalidAllocation)
        );

        let mut wrong_class = make_allocation();
        wrong_class.resource_class = AllocationResourceClass::Linear;
        assert_eq!(
            block.validate_allocation(&wrong_class),
            Err(AllocationError::InvalidAllocation)
        );
    }

    #[test]
    fn memory_type_stats_aggregate_blocks() {
        let memory_type = MemoryTypeAllocator {
            memory_type_index: 7,
            blocks: vec![
                Some(MemoryBlock {
                    memory: vk::DeviceMemory(11),
                    size: 1024,
                    resource_class: AllocationResourceClass::Unknown,
                    dedicated: false,
                    suballocator: TlsfSuballocator::new(1024),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                }),
                Some(MemoryBlock {
                    memory: vk::DeviceMemory(12),
                    size: 2048,
                    resource_class: AllocationResourceClass::Unknown,
                    dedicated: false,
                    suballocator: TlsfSuballocator::new(2048),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                }),
            ],
        };
        let mut memory_type = memory_type;
        let a = memory_type.blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();
        let b = memory_type.blocks[1]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(256, 1)
            .unwrap();
        let stats = memory_type.stats();
        assert_eq!(stats.memory_type_index, 7);
        assert_eq!(stats.block_count, 2);
        assert_eq!(stats.total_block_bytes, 3072);
        assert_eq!(stats.allocation_count, 2);
        assert_eq!(stats.allocated_bytes, a.size + b.size);
        assert_eq!(stats.allocated_bytes + stats.free_bytes, 3072);
    }

    #[test]
    fn stats_and_snapshot_ignore_released_block_slots() {
        let memory_type = MemoryTypeAllocator {
            memory_type_index: 5,
            blocks: vec![
                None,
                Some(MemoryBlock {
                    memory: vk::DeviceMemory(51),
                    size: 1024,
                    resource_class: AllocationResourceClass::Linear,
                    dedicated: false,
                    suballocator: TlsfSuballocator::new(1024),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                }),
            ],
        };
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 6,
            ..Default::default()
        };
        properties.memory_types[5].property_flags = vk::MemoryPropertyFlags::HOST_VISIBLE;

        let stats = memory_type.stats();
        assert_eq!(stats.block_count, 1);
        assert_eq!(stats.total_block_bytes, 1024);

        let snapshot = snapshot_from_memory_types(&[memory_type], &properties);
        assert_eq!(snapshot.memory_types.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks[0].block_index, 1);
    }

    #[test]
    fn allocator_stats_aggregate_non_empty_memory_types() {
        let mut memory_types = vec![
            MemoryTypeAllocator {
                memory_type_index: 0,
                blocks: Vec::new(),
            },
            MemoryTypeAllocator {
                memory_type_index: 1,
                blocks: vec![Some(MemoryBlock {
                    memory: vk::DeviceMemory(21),
                    size: 1024,
                    resource_class: AllocationResourceClass::Linear,
                    dedicated: false,
                    suballocator: TlsfSuballocator::new(1024),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                })],
            },
            MemoryTypeAllocator {
                memory_type_index: 2,
                blocks: vec![Some(MemoryBlock {
                    memory: vk::DeviceMemory(22),
                    size: 2048,
                    resource_class: AllocationResourceClass::Optimal,
                    dedicated: false,
                    suballocator: TlsfSuballocator::new(2048),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                })],
            },
        ];
        let a = memory_types[1].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();
        let b = memory_types[2].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(256, 1)
            .unwrap();

        let stats = stats_from_memory_types(&memory_types);
        assert_eq!(stats.memory_types.len(), 2);
        assert_eq!(stats.memory_types[0].memory_type_index, 1);
        assert_eq!(stats.memory_types[1].memory_type_index, 2);
        assert_eq!(stats.block_count, 2);
        assert_eq!(stats.total_block_bytes, 3072);
        assert_eq!(stats.allocation_count, 2);
        assert_eq!(stats.allocated_bytes, a.size + b.size);
        assert_eq!(
            stats.allocated_bytes + stats.free_bytes,
            stats.total_block_bytes
        );
    }

    #[test]
    fn block_snapshot_reports_fragmented_regions() {
        let mut block = MemoryBlock {
            memory: vk::DeviceMemory(41),
            size: 1024,
            resource_class: AllocationResourceClass::Linear,
            dedicated: false,
            suballocator: TlsfSuballocator::new(1024),
            mapped_ptr: None,
            map_count: 0,
            mapped_allocations: HashMap::new(),
        };
        let a = block.suballocator.allocate(256, 1).unwrap();
        let b = block.suballocator.allocate(256, 1).unwrap();
        let c = block.suballocator.allocate(256, 1).unwrap();
        let d = block.suballocator.allocate(256, 1).unwrap();
        block.suballocator.free(b.id).unwrap();
        block.suballocator.free(d.id).unwrap();
        block.mark_mapped(c.id).unwrap();

        let snapshot = block.snapshot(3);
        assert_eq!(snapshot.block_index, 3);
        assert_eq!(snapshot.resource_class, AllocationResourceClass::Linear);
        assert_eq!(snapshot.memory, vk::DeviceMemory(41));
        assert_eq!(snapshot.size, 1024);
        assert_eq!(snapshot.free_bytes, 512);
        assert_eq!(snapshot.allocated_bytes, 512);
        assert_eq!(snapshot.largest_free_region_bytes, 256);
        assert_eq!(snapshot.free_region_count, 2);
        assert_eq!(snapshot.allocated_region_count, 2);
        assert_eq!(
            snapshot.regions,
            vec![
                MemoryRegionSnapshot {
                    offset: 0,
                    size: 256,
                    state: MemoryRegionState::Allocated {
                        allocation_id: a.id,
                        mapped_count: 0,
                    },
                },
                MemoryRegionSnapshot {
                    offset: 256,
                    size: 256,
                    state: MemoryRegionState::Free,
                },
                MemoryRegionSnapshot {
                    offset: 512,
                    size: 256,
                    state: MemoryRegionState::Allocated {
                        allocation_id: c.id,
                        mapped_count: 1,
                    },
                },
                MemoryRegionSnapshot {
                    offset: 768,
                    size: 256,
                    state: MemoryRegionState::Free,
                },
            ]
        );
    }

    #[test]
    fn allocator_snapshot_keeps_stats_and_region_details_together() {
        let properties = memory_properties(&[
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED,
        ]);
        let mut memory_types = vec![
            MemoryTypeAllocator {
                memory_type_index: 0,
                blocks: Vec::new(),
            },
            MemoryTypeAllocator {
                memory_type_index: 1,
                blocks: vec![Some(MemoryBlock {
                    memory: vk::DeviceMemory(42),
                    size: 1024,
                    resource_class: AllocationResourceClass::Optimal,
                    dedicated: true,
                    suballocator: TlsfSuballocator::new(1024),
                    mapped_ptr: None,
                    map_count: 0,
                    mapped_allocations: HashMap::new(),
                })],
            },
        ];
        let allocation = memory_types[1].blocks[0]
            .as_mut()
            .unwrap()
            .suballocator
            .allocate(128, 1)
            .unwrap();

        let snapshot = snapshot_from_memory_types(&memory_types, &properties);
        assert_eq!(snapshot.stats.block_count, 1);
        assert_eq!(snapshot.stats.allocation_count, 1);
        assert_eq!(snapshot.memory_types.len(), 1);
        assert_eq!(snapshot.memory_types[0].memory_type_index, 1);
        assert_eq!(
            snapshot.memory_types[0].property_flags,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED
        );
        assert_eq!(
            snapshot.memory_types[0].stats,
            snapshot.stats.memory_types[0]
        );
        assert_eq!(snapshot.memory_types[0].blocks.len(), 1);
        assert_eq!(snapshot.memory_types[0].blocks[0].block_index, 0);
        assert_eq!(
            snapshot.memory_types[0].blocks[0].resource_class,
            AllocationResourceClass::Optimal
        );
        assert!(snapshot.memory_types[0].blocks[0].dedicated);
        assert_eq!(snapshot.memory_types[0].blocks[0].regions[0].offset, 0);
        assert_eq!(snapshot.memory_types[0].blocks[0].regions[0].size, 128);
        assert_eq!(
            snapshot.memory_types[0].blocks[0].regions[0].state,
            MemoryRegionState::Allocated {
                allocation_id: allocation.id,
                mapped_count: 0,
            }
        );
    }

    #[test]
    fn mapped_ranges_are_limited_to_the_allocation_and_atom_aligned() {
        let (offset, size) = mapped_range_bounds(300, 100, 1024, 128).unwrap();
        assert_eq!(offset, 256);
        assert_eq!(size, 256);

        let (offset, size) = mapped_range_bounds(1000, 10, 1024, 128).unwrap();
        assert_eq!(offset, 896);
        assert_eq!(size, vk::WHOLE_SIZE);

        assert!(mapped_range_bounds(900, 200, 1024, 128).is_none());
    }

    #[test]
    fn huge_suballocator_bounds_do_not_overflow() {
        let mut allocator = TlsfSuballocator::new(u64::MAX);
        let allocation = allocator.allocate(u64::MAX, 1).unwrap();
        assert_eq!(allocation.offset, 0);
        assert_eq!(allocation.size, u64::MAX);
        assert!(allocator.allocate(1, 1).is_none());
        allocator.free(allocation.id).unwrap();
        assert_eq!(allocator.stats().free_bytes, u64::MAX);
    }

    #[test]
    fn can_allocate_rejects_fragmented_or_alignment_impossible_requests_without_mutation() {
        let mut allocator = TlsfSuballocator::new(1024);
        let a = allocator.allocate(256, 1).unwrap();
        let b = allocator.allocate(256, 1).unwrap();
        let c = allocator.allocate(256, 1).unwrap();
        allocator.free(b.id).unwrap();

        let before = allocator.clone();
        assert!(allocator.can_allocate(128, 1));
        assert!(!allocator.can_allocate(300, 1));
        assert!(!allocator.can_allocate(256, 512));
        assert_eq!(allocator.blocks, before.blocks);
        assert_eq!(allocator.free_bins, before.free_bins);
        assert_eq!(allocator.allocations, before.allocations);

        allocator.free(a.id).unwrap();
        allocator.free(c.id).unwrap();
        assert!(allocator.can_allocate(1024, 1));
    }

    #[test]
    fn rejects_allocation_id_overflow_without_mutating_state() {
        let mut allocator = TlsfSuballocator::new(1024);
        allocator.next_id = u64::MAX;
        assert!(allocator.allocate(128, 1).is_none());
        assert_eq!(allocator.stats().free_bytes, 1024);
        assert_eq!(allocator.stats().allocation_count, 0);
        assert!(allocator.allocations.is_empty());
        check_invariants(&allocator);
    }
}
