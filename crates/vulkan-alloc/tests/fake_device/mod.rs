#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::ffi::{CStr, c_char, c_void};
use std::sync::Mutex;

use vulkan::{Device, vk};
use vulkan_alloc::{AllocationCreateDesc, Allocator, AllocatorCreateDesc, MemoryLocation};

pub const DEVICE_LOCAL: u32 = 0;
pub const HOST_COHERENT: u32 = 1;
pub const HOST_NON_COHERENT: u32 = 2;

pub const BLOCK_SIZE: u64 = 4096;
pub const NON_COHERENT_ATOM_SIZE: u64 = 64;
pub const LARGEST_BACKED_MEMORY: u64 = 1 << 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryObject {
    pub size: u64,
    pub memory_type_index: u32,
    pub mapped: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dedicated {
    Buffer(vk::Buffer),
    Image(vk::Image),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Chain {
    pub flags: Option<(vk::MemoryAllocateFlags, u32)>,
    pub dedicated: Option<Dedicated>,
    pub export: Option<vk::ExternalMemoryHandleTypeFlags>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Calls {
    pub allocate: usize,
    pub free: usize,
    pub map: usize,
    pub unmap: usize,
    pub create_resource: usize,
    pub destroy_resource: usize,
    pub query_requirements: usize,
    pub query_modifier: usize,
    pub bind: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    Buffer,
    Image,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceSetup {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
    pub prefers_dedicated: bool,
    pub requires_dedicated: bool,
    pub drm_format_modifier: u64,
}

impl Default for ResourceSetup {
    fn default() -> Self {
        Self {
            size: 256,
            alignment: 16,
            memory_type_bits: 0b111,
            prefers_dedicated: false,
            requires_dedicated: false,
            drm_format_modifier: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceObject {
    pub kind: ResourceKind,
    pub setup: ResourceSetup,
    pub needs_device_address: bool,
    pub export_handle_types: vk::ExternalMemoryHandleTypeFlags,
    pub bound_in_parts: bool,
    pub has_drm_format_modifier: bool,
    pub bound: Option<(vk::DeviceMemory, u64)>,
}

impl ResourceObject {
    pub fn handle(self, raw: u64) -> Dedicated {
        match self.kind {
            ResourceKind::Buffer => Dedicated::Buffer(vk::Buffer::from_raw(raw)),
            ResourceKind::Image => Dedicated::Image(vk::Image::from_raw(raw)),
        }
    }
}

const FIRST_RESOURCE_HANDLE: u64 = 1 << 40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Synchronization {
    Flush,
    Invalidate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SynchronizedRange {
    pub kind: Synchronization,
    pub memory: vk::DeviceMemory,
    pub offset: u64,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapFault {
    Fails(vk::VkResult),
    ReturnsNull,
    ReturnsAddress(usize),
}

struct Memory {
    object: MemoryObject,
    chain: Chain,
    storage: Option<Box<[u8]>>,
}

struct State {
    memories: HashMap<u64, Memory>,
    released_handles: BTreeSet<u64>,
    last_handle: u64,
    calls: Calls,
    misuse: Vec<String>,
    non_coherent_atom_size: u64,
    synchronized: Vec<SynchronizedRange>,
    next_allocation_fails: Option<vk::VkResult>,
    allocation_results: VecDeque<vk::VkResult>,
    allocated_memory_types: Vec<u32>,
    next_map_fault: Option<MapFault>,
    next_synchronization_fails: Option<vk::VkResult>,
    resources: HashMap<u64, ResourceObject>,
    destroyed_resources: BTreeSet<u64>,
    last_resource_handle: u64,
    resource_setup: ResourceSetup,
    next_creation_fails: Option<vk::VkResult>,
    next_modifier_query_fails: Option<vk::VkResult>,
    next_bind_fails: Option<vk::VkResult>,
}

pub struct FakeDevice {
    state: *mut Mutex<State>,
    device: Device,
}

impl FakeDevice {
    pub fn new() -> Self {
        Self::with_non_coherent_atom_size(NON_COHERENT_ATOM_SIZE)
    }

    pub fn with_non_coherent_atom_size(non_coherent_atom_size: u64) -> Self {
        let state = Box::into_raw(Box::new(Mutex::new(State {
            memories: HashMap::new(),
            released_handles: BTreeSet::new(),
            last_handle: 0,
            calls: Calls::default(),
            misuse: Vec::new(),
            non_coherent_atom_size: non_coherent_atom_size.max(1),
            synchronized: Vec::new(),
            next_allocation_fails: None,
            allocation_results: VecDeque::new(),
            allocated_memory_types: Vec::new(),
            next_map_fault: None,
            next_synchronization_fails: None,
            resources: HashMap::new(),
            destroyed_resources: BTreeSet::new(),
            last_resource_handle: FIRST_RESOURCE_HANDLE,
            resource_setup: ResourceSetup::default(),
            next_creation_fails: None,
            next_modifier_query_fails: None,
            next_bind_fails: None,
        })));
        let handle = vk::Device(state.cast());
        let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
        Self {
            state,
            device: Device::from_raw(handle, fp),
        }
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn memory_properties() -> vk::PhysicalDeviceMemoryProperties {
        let mut properties = vk::PhysicalDeviceMemoryProperties {
            memory_type_count: 3,
            memory_heap_count: 1,
            ..Default::default()
        };
        properties.memory_types[DEVICE_LOCAL as usize].property_flags =
            vk::MemoryPropertyFlags::DEVICE_LOCAL;
        properties.memory_types[HOST_COHERENT as usize].property_flags =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT;
        properties.memory_types[HOST_NON_COHERENT as usize].property_flags =
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_CACHED;
        properties
    }

    pub fn allocator(&self) -> Allocator {
        Allocator::new(AllocatorCreateDesc {
            device: &self.device,
            memory_properties: Self::memory_properties(),
            non_coherent_atom_size: NON_COHERENT_ATOM_SIZE,
            block_size: BLOCK_SIZE,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        })
    }

    pub fn memory_objects(&self) -> HashMap<vk::DeviceMemory, MemoryObject> {
        self.state()
            .memories
            .iter()
            .map(|(&handle, memory)| (vk::DeviceMemory(handle), memory.object))
            .collect()
    }

    pub fn chains(&self) -> HashMap<vk::DeviceMemory, Chain> {
        self.state()
            .memories
            .iter()
            .map(|(&handle, memory)| (vk::DeviceMemory(handle), memory.chain))
            .collect()
    }

    pub fn calls(&self) -> Calls {
        self.state().calls
    }

    pub fn misuse(&self) -> Vec<String> {
        self.state().misuse.clone()
    }

    pub fn synchronized(&self) -> Vec<SynchronizedRange> {
        self.state().synchronized.clone()
    }

    pub fn base_address(&self, memory: vk::DeviceMemory) -> Option<usize> {
        self.state()
            .memories
            .get(&memory.0)?
            .storage
            .as_ref()
            .map(|storage| storage.as_ptr() as usize)
    }

    pub fn bytes(&self, memory: vk::DeviceMemory, offset: u64, size: u64) -> Option<Vec<u8>> {
        let state = self.state();
        let storage = state.memories.get(&memory.0)?.storage.as_ref()?;
        let start = usize::try_from(offset).ok()?;
        let end = start.checked_add(usize::try_from(size).ok()?)?;
        storage.get(start..end).map(<[u8]>::to_vec)
    }

    pub fn fail_next_allocation(&self, result: vk::VkResult) {
        self.state().next_allocation_fails = Some(result);
    }

    pub fn script_allocations(&self, results: impl IntoIterator<Item = vk::VkResult>) {
        self.state().allocation_results.extend(results);
    }

    pub fn allocated_memory_types(&self) -> Vec<u32> {
        self.state().allocated_memory_types.clone()
    }

    pub fn fault_next_map(&self, fault: MapFault) {
        self.state().next_map_fault = Some(fault);
    }

    pub fn fail_next_synchronization(&self, result: vk::VkResult) {
        self.state().next_synchronization_fails = Some(result);
    }

    pub fn set_resource_setup(&self, setup: ResourceSetup) {
        self.state().resource_setup = setup;
    }

    pub fn resources(&self) -> HashMap<u64, ResourceObject> {
        self.state().resources.clone()
    }

    pub fn fail_next_resource_creation(&self, result: vk::VkResult) {
        self.state().next_creation_fails = Some(result);
    }

    pub fn fail_next_modifier_query(&self, result: vk::VkResult) {
        self.state().next_modifier_query_fails = Some(result);
    }

    pub fn fail_next_bind(&self, result: vk::VkResult) {
        self.state().next_bind_fails = Some(result);
    }

    pub fn clear_faults(&self) {
        let mut state = self.state();
        state.next_allocation_fails = None;
        state.allocation_results.clear();
        state.next_map_fault = None;
        state.next_synchronization_fails = None;
        state.next_creation_fails = None;
        state.next_modifier_query_fails = None;
        state.next_bind_fails = None;
    }

    fn state(&self) -> std::sync::MutexGuard<'_, State> {
        unsafe { &*self.state }.lock().unwrap()
    }
}

impl Drop for FakeDevice {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.state) });
    }
}

pub fn assert_only_kept_empty_blocks(
    allocator: &Allocator,
    shared_block_size: impl Fn(u32) -> u64,
    context: &str,
) {
    for memory_type in allocator.snapshot().memory_types {
        let index = memory_type.memory_type_index;
        let mut flags = Vec::new();
        for block in &memory_type.blocks {
            assert!(
                block.allocated_region_count == 0
                    && !block.dedicated
                    && block.size == shared_block_size(index),
                "{context}: after every allocation was freed, memory type {index} holds {block:?}, \
                 which is not an empty shared block of {} bytes",
                shared_block_size(index)
            );
            assert!(
                !flags.contains(&block.memory_allocate_flags),
                "{context}: memory type {index} keeps more than one empty block with the flags \
                 {:?}",
                block.memory_allocate_flags
            );
            flags.push(block.memory_allocate_flags);
        }
    }
}

pub fn desc(size: u64, alignment: u64, memory_type_index: u32) -> AllocationCreateDesc<'static> {
    AllocationCreateDesc::new(
        vk::MemoryRequirements {
            size,
            alignment,
            memory_type_bits: 1 << memory_type_index,
        },
        MemoryLocation::custom(
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::empty(),
        ),
        true,
    )
}

unsafe fn state_of<'a>(device: vk::Device) -> std::sync::MutexGuard<'a, State> {
    unsafe { &*device.0.cast::<Mutex<State>>() }.lock().unwrap()
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    type Void = unsafe extern "system" fn();
    type SynchronizeRanges = unsafe extern "system" fn(
        vk::Device,
        u32,
        *const vk::MappedMemoryRange<'_>,
    ) -> vk::VkResult;
    let name = unsafe { CStr::from_ptr(p_name) }.to_bytes();
    unsafe {
        match name {
            b"vkAllocateMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    *const vk::MemoryAllocateInfo<'_>,
                    *const vk::AllocationCallbacks<'_>,
                    *mut vk::DeviceMemory,
                ) -> vk::VkResult,
                Void,
            >(allocate_memory)),
            b"vkFreeMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::DeviceMemory,
                    *const vk::AllocationCallbacks<'_>,
                ),
                Void,
            >(free_memory)),
            b"vkMapMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::DeviceMemory,
                    vk::DeviceSize,
                    vk::DeviceSize,
                    vk::MemoryMapFlags,
                    *mut *mut c_void,
                ) -> vk::VkResult,
                Void,
            >(map_memory)),
            b"vkUnmapMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(vk::Device, vk::DeviceMemory),
                Void,
            >(unmap_memory)),
            b"vkFlushMappedMemoryRanges" => Some(std::mem::transmute::<SynchronizeRanges, Void>(
                flush_mapped_memory_ranges,
            )),
            b"vkInvalidateMappedMemoryRanges" => Some(
                std::mem::transmute::<SynchronizeRanges, Void>(invalidate_mapped_memory_ranges),
            ),
            b"vkCreateBuffer" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    *const vk::BufferCreateInfo<'_>,
                    *const vk::AllocationCallbacks<'_>,
                    *mut vk::Buffer,
                ) -> vk::VkResult,
                Void,
            >(create_buffer)),
            b"vkCreateImage" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    *const vk::ImageCreateInfo<'_>,
                    *const vk::AllocationCallbacks<'_>,
                    *mut vk::Image,
                ) -> vk::VkResult,
                Void,
            >(create_image)),
            b"vkDestroyBuffer" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::Buffer,
                    *const vk::AllocationCallbacks<'_>,
                ),
                Void,
            >(destroy_buffer)),
            b"vkDestroyImage" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::Image,
                    *const vk::AllocationCallbacks<'_>,
                ),
                Void,
            >(destroy_image)),
            b"vkGetBufferMemoryRequirements2" => {
                Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        *const vk::BufferMemoryRequirementsInfo2<'_>,
                        *mut vk::MemoryRequirements2<'_>,
                    ),
                    Void,
                >(get_buffer_memory_requirements2))
            }
            b"vkGetImageMemoryRequirements2" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    *const vk::ImageMemoryRequirementsInfo2<'_>,
                    *mut vk::MemoryRequirements2<'_>,
                ),
                Void,
            >(get_image_memory_requirements2)),
            b"vkGetImageDrmFormatModifierPropertiesEXT" => {
                Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        vk::Image,
                        *mut vk::ImageDrmFormatModifierPropertiesEXT<'_>,
                    ) -> vk::VkResult,
                    Void,
                >(get_image_drm_format_modifier_properties))
            }
            b"vkBindBufferMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::Buffer,
                    vk::DeviceMemory,
                    vk::DeviceSize,
                ) -> vk::VkResult,
                Void,
            >(bind_buffer_memory)),
            b"vkBindImageMemory" => Some(std::mem::transmute::<
                unsafe extern "system" fn(
                    vk::Device,
                    vk::Image,
                    vk::DeviceMemory,
                    vk::DeviceSize,
                ) -> vk::VkResult,
                Void,
            >(bind_image_memory)),
            _ => None,
        }
    }
}

unsafe fn read_chain(mut next: *const vk::BaseInStructure<'_>) -> Result<Chain, String> {
    let mut chain = Chain {
        flags: None,
        dedicated: None,
        export: None,
    };
    while !next.is_null() {
        let s_type = unsafe { (*next).s_type };
        let repeated = if s_type == vk::StructureType::MEMORY_ALLOCATE_FLAGS_INFO {
            let info = unsafe { &*next.cast::<vk::MemoryAllocateFlagsInfo<'_>>() };
            chain
                .flags
                .replace((info.flags, info.device_mask))
                .is_some()
        } else if s_type == vk::StructureType::MEMORY_DEDICATED_ALLOCATE_INFO {
            let info = unsafe { &*next.cast::<vk::MemoryDedicatedAllocateInfo<'_>>() };
            let dedicated = match (
                info.buffer == vk::Buffer::null(),
                info.image == vk::Image::null(),
            ) {
                (false, true) => Dedicated::Buffer(info.buffer),
                (true, false) => Dedicated::Image(info.image),
                _ => {
                    return Err(format!(
                        "VkMemoryDedicatedAllocateInfo with buffer {:?} and image {:?}",
                        info.buffer, info.image
                    ));
                }
            };
            chain.dedicated.replace(dedicated).is_some()
        } else if s_type == vk::StructureType::EXPORT_MEMORY_ALLOCATE_INFO {
            let info = unsafe { &*next.cast::<vk::ExportMemoryAllocateInfo<'_>>() };
            chain.export.replace(info.handle_types).is_some()
        } else {
            return Err(format!("unexpected {s_type:?} in the pNext chain"));
        };
        if repeated {
            return Err(format!("{s_type:?} appears twice in the pNext chain"));
        }
        next = unsafe { (*next).p_next };
    }
    Ok(chain)
}

unsafe extern "system" fn allocate_memory(
    device: vk::Device,
    p_allocate_info: *const vk::MemoryAllocateInfo<'_>,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
    p_memory: *mut vk::DeviceMemory,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    let info = unsafe { &*p_allocate_info };
    state.calls.allocate += 1;
    state.allocated_memory_types.push(info.memory_type_index);
    if info.s_type != vk::StructureType::MEMORY_ALLOCATE_INFO {
        state
            .misuse
            .push(format!("vkAllocateMemory with sType {:?}", info.s_type));
    }
    if info.allocation_size == 0 {
        state
            .misuse
            .push("vkAllocateMemory with allocationSize 0".to_string());
    }
    let chain = match unsafe { read_chain(info.p_next.cast()) } {
        Ok(chain) => chain,
        Err(problem) => {
            state.misuse.push(format!("vkAllocateMemory: {problem}"));
            return vk::VkResult::ERROR_UNKNOWN;
        }
    };
    if let Some(problem) = dedicated_allocation_problem(&state, &chain, info.allocation_size) {
        state.misuse.push(format!("vkAllocateMemory: {problem}"));
    }
    if let Some(result) = state.next_allocation_fails.take() {
        return result;
    }
    if let Some(result) = state
        .allocation_results
        .pop_front()
        .filter(|&result| result != vk::VkResult::SUCCESS)
    {
        return result;
    }
    let handle = state.released_handles.pop_first().unwrap_or_else(|| {
        state.last_handle += 1;
        state.last_handle
    });
    state.memories.insert(
        handle,
        Memory {
            object: MemoryObject {
                size: info.allocation_size,
                memory_type_index: info.memory_type_index,
                mapped: false,
            },
            chain,
            storage: None,
        },
    );
    unsafe { p_memory.write(vk::DeviceMemory(handle)) };
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn free_memory(
    device: vk::Device,
    memory: vk::DeviceMemory,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
) {
    let mut state = unsafe { state_of(device) };
    state.calls.free += 1;
    if state.memories.remove(&memory.0).is_some() {
        state.released_handles.insert(memory.0);
    } else {
        state
            .misuse
            .push(format!("vkFreeMemory of unknown memory {memory:?}"));
    }
}

unsafe extern "system" fn map_memory(
    device: vk::Device,
    memory: vk::DeviceMemory,
    offset: vk::DeviceSize,
    size: vk::DeviceSize,
    _flags: vk::MemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    state.calls.map += 1;
    let fault = state.next_map_fault.take();
    let problem = match state.memories.get_mut(&memory.0) {
        None => Some(format!("vkMapMemory of unknown memory {memory:?}")),
        Some(entry) if entry.object.mapped => {
            Some(format!("vkMapMemory of already mapped memory {memory:?}"))
        }
        Some(entry) if offset != 0 || size != entry.object.size => Some(format!(
            "vkMapMemory of {memory:?} with offset {offset} and size {size}"
        )),
        Some(entry) => match fault {
            Some(MapFault::Fails(result)) => return result,
            Some(MapFault::ReturnsNull) => {
                entry.object.mapped = true;
                unsafe { pp_data.write(std::ptr::null_mut()) };
                None
            }
            Some(MapFault::ReturnsAddress(address)) => {
                entry.object.mapped = true;
                unsafe { pp_data.write(std::ptr::without_provenance_mut(address)) };
                None
            }
            None if entry.object.size > LARGEST_BACKED_MEMORY => {
                entry.object.mapped = true;
                unsafe { pp_data.write(std::ptr::NonNull::<u8>::dangling().as_ptr().cast()) };
                None
            }
            None => {
                entry.object.mapped = true;
                let length = entry.object.size as usize;
                let storage = entry
                    .storage
                    .get_or_insert_with(|| vec![0; length].into_boxed_slice());
                unsafe { pp_data.write(storage.as_mut_ptr().cast()) };
                None
            }
        },
    };
    match problem {
        None => vk::VkResult::SUCCESS,
        Some(problem) => {
            state.misuse.push(problem);
            vk::VkResult::ERROR_MEMORY_MAP_FAILED
        }
    }
}

unsafe extern "system" fn unmap_memory(device: vk::Device, memory: vk::DeviceMemory) {
    let mut state = unsafe { state_of(device) };
    state.calls.unmap += 1;
    match state.memories.get_mut(&memory.0) {
        Some(entry) if entry.object.mapped => entry.object.mapped = false,
        _ => state.misuse.push(format!(
            "vkUnmapMemory of memory {memory:?} that is not mapped"
        )),
    }
}

unsafe extern "system" fn flush_mapped_memory_ranges(
    device: vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange<'_>,
) -> vk::VkResult {
    unsafe {
        synchronize(
            device,
            Synchronization::Flush,
            memory_range_count,
            p_memory_ranges,
        )
    }
}

unsafe extern "system" fn invalidate_mapped_memory_ranges(
    device: vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange<'_>,
) -> vk::VkResult {
    unsafe {
        synchronize(
            device,
            Synchronization::Invalidate,
            memory_range_count,
            p_memory_ranges,
        )
    }
}

unsafe fn synchronize(
    device: vk::Device,
    kind: Synchronization,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange<'_>,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    let ranges =
        unsafe { std::slice::from_raw_parts(p_memory_ranges, memory_range_count as usize) };
    let atom = state.non_coherent_atom_size;
    for range in ranges {
        let problem = match state.memories.get(&range.memory.0) {
            None => Some("unknown memory".to_string()),
            Some(entry) if !entry.object.mapped => Some("memory that is not mapped".to_string()),
            Some(entry) => mapped_range_problem(range.offset, range.size, entry.object.size, atom),
        };
        if let Some(problem) = problem {
            state.misuse.push(format!(
                "{kind:?} of offset {} size {} of {:?}: {problem}",
                range.offset, range.size, range.memory
            ));
        }
        state.synchronized.push(SynchronizedRange {
            kind,
            memory: range.memory,
            offset: range.offset,
            size: range.size,
        });
    }
    match state.next_synchronization_fails.take() {
        Some(result) => result,
        None => vk::VkResult::SUCCESS,
    }
}

fn mapped_range_problem(offset: u64, size: u64, mapped_size: u64, atom: u64) -> Option<String> {
    if !offset.is_multiple_of(atom) {
        return Some(format!(
            "the offset is not a multiple of nonCoherentAtomSize {atom}"
        ));
    }
    if size == vk::WHOLE_SIZE {
        return (offset >= mapped_size)
            .then(|| "the offset is outside the mapped range".to_string());
    }
    let Some(end) = offset.checked_add(size) else {
        return Some("the range overflows".to_string());
    };
    if end > mapped_size {
        return Some("the range is not inside the mapped range".to_string());
    }
    if !size.is_multiple_of(atom) && end != mapped_size {
        return Some(format!(
            "the size is not a multiple of nonCoherentAtomSize {atom} and the range does not end \
             at the end of the memory"
        ));
    }
    None
}

fn dedicated_allocation_problem(state: &State, chain: &Chain, size: u64) -> Option<String> {
    let (raw, kind) = match chain.dedicated? {
        Dedicated::Buffer(buffer) => (buffer.as_raw(), ResourceKind::Buffer),
        Dedicated::Image(image) => (image.as_raw(), ResourceKind::Image),
    };
    let Some(resource) = state.resources.get(&raw) else {
        return state
            .destroyed_resources
            .contains(&raw)
            .then(|| format!("dedicated to {raw:#x}, which was destroyed"));
    };
    if resource.kind != kind {
        return Some(format!(
            "dedicated to {raw:#x} as a {kind:?}, which it is not"
        ));
    }
    if resource.bound.is_some() {
        return Some(format!("dedicated to {raw:#x}, which is already bound"));
    }
    (resource.setup.size != size).then(|| {
        format!(
            "dedicated to {raw:#x} with allocationSize {size}, while its requirements have size {}",
            resource.setup.size
        )
    })
}

fn new_resource(state: &mut State, resource: ResourceObject) -> Result<u64, vk::VkResult> {
    state.calls.create_resource += 1;
    if let Some(result) = state.next_creation_fails.take() {
        return Err(result);
    }
    state.last_resource_handle += 1;
    let handle = state.last_resource_handle;
    state.resources.insert(handle, resource);
    Ok(handle)
}

unsafe fn for_each_in_chain(
    mut next: *const vk::BaseInStructure<'_>,
    mut visit: impl FnMut(*const vk::BaseInStructure<'_>),
) {
    while !next.is_null() {
        visit(next);
        next = unsafe { (*next).p_next };
    }
}

unsafe extern "system" fn create_buffer(
    device: vk::Device,
    p_create_info: *const vk::BufferCreateInfo<'_>,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
    p_buffer: *mut vk::Buffer,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    let info = unsafe { &*p_create_info };
    let mut usage = vk::BufferUsageFlags2::from_raw(u64::from(info.usage.as_raw()));
    let mut export_handle_types = vk::ExternalMemoryHandleTypeFlags::empty();
    unsafe {
        for_each_in_chain(info.p_next.cast(), |next| match (*next).s_type {
            vk::StructureType::BUFFER_USAGE_FLAGS_2_CREATE_INFO => {
                usage = (*next.cast::<vk::BufferUsageFlags2CreateInfo<'_>>()).usage;
            }
            vk::StructureType::EXTERNAL_MEMORY_BUFFER_CREATE_INFO => {
                export_handle_types =
                    (*next.cast::<vk::ExternalMemoryBufferCreateInfo<'_>>()).handle_types;
            }
            _ => {}
        });
    }
    let sparse = vk::BufferCreateFlags::SPARSE_BINDING
        | vk::BufferCreateFlags::SPARSE_RESIDENCY
        | vk::BufferCreateFlags::SPARSE_ALIASED;
    let resource = ResourceObject {
        kind: ResourceKind::Buffer,
        setup: state.resource_setup,
        needs_device_address: usage.intersects(
            vk::BufferUsageFlags2::SHADER_DEVICE_ADDRESS
                | vk::BufferUsageFlags2::DESCRIPTOR_HEAP_EXT,
        ),
        export_handle_types,
        bound_in_parts: info.flags.intersects(sparse),
        has_drm_format_modifier: false,
        bound: None,
    };
    match new_resource(&mut state, resource) {
        Ok(handle) => {
            unsafe { p_buffer.write(vk::Buffer::from_raw(handle)) };
            vk::VkResult::SUCCESS
        }
        Err(result) => result,
    }
}

unsafe extern "system" fn create_image(
    device: vk::Device,
    p_create_info: *const vk::ImageCreateInfo<'_>,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
    p_image: *mut vk::Image,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    let info = unsafe { &*p_create_info };
    let mut flags = vk::ImageCreateFlags2KHR::from_raw(u64::from(info.flags.as_raw()));
    let mut export_handle_types = vk::ExternalMemoryHandleTypeFlags::empty();
    unsafe {
        for_each_in_chain(info.p_next.cast(), |next| match (*next).s_type {
            vk::StructureType::IMAGE_CREATE_FLAGS_2_CREATE_INFO_KHR => {
                flags = (*next.cast::<vk::ImageCreateFlags2CreateInfoKHR<'_>>()).flags;
            }
            vk::StructureType::EXTERNAL_MEMORY_IMAGE_CREATE_INFO => {
                export_handle_types =
                    (*next.cast::<vk::ExternalMemoryImageCreateInfo<'_>>()).handle_types;
            }
            _ => {}
        });
    }
    let in_parts = vk::ImageCreateFlags2KHR::SPARSE_BINDING
        | vk::ImageCreateFlags2KHR::SPARSE_RESIDENCY
        | vk::ImageCreateFlags2KHR::SPARSE_ALIASED
        | vk::ImageCreateFlags2KHR::DISJOINT;
    let resource = ResourceObject {
        kind: ResourceKind::Image,
        setup: state.resource_setup,
        needs_device_address: false,
        export_handle_types,
        bound_in_parts: flags.intersects(in_parts),
        has_drm_format_modifier: info.tiling == vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT,
        bound: None,
    };
    match new_resource(&mut state, resource) {
        Ok(handle) => {
            unsafe { p_image.write(vk::Image::from_raw(handle)) };
            vk::VkResult::SUCCESS
        }
        Err(result) => result,
    }
}

fn destroy_resource(device: vk::Device, raw: u64, kind: ResourceKind) {
    let mut state = unsafe { state_of(device) };
    state.calls.destroy_resource += 1;
    if raw == 0 {
        return;
    }
    match state.resources.get(&raw) {
        Some(resource) if resource.kind == kind => {
            state.resources.remove(&raw);
            state.destroyed_resources.insert(raw);
        }
        Some(_) => state.misuse.push(format!(
            "destroying {raw:#x} as a {kind:?}, which it is not"
        )),
        None if state.destroyed_resources.contains(&raw) => state
            .misuse
            .push(format!("destroying the {kind:?} {raw:#x} a second time")),
        None => state
            .misuse
            .push(format!("destroying the unknown {kind:?} {raw:#x}")),
    }
}

unsafe extern "system" fn destroy_buffer(
    device: vk::Device,
    buffer: vk::Buffer,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
) {
    destroy_resource(device, buffer.as_raw(), ResourceKind::Buffer);
}

unsafe extern "system" fn destroy_image(
    device: vk::Device,
    image: vk::Image,
    _p_allocator: *const vk::AllocationCallbacks<'_>,
) {
    destroy_resource(device, image.as_raw(), ResourceKind::Image);
}

unsafe fn write_requirements(
    device: vk::Device,
    raw: u64,
    kind: ResourceKind,
    p_memory_requirements: *mut vk::MemoryRequirements2<'_>,
) {
    let mut state = unsafe { state_of(device) };
    state.calls.query_requirements += 1;
    let Some(resource) = state
        .resources
        .get(&raw)
        .copied()
        .filter(|resource| resource.kind == kind)
    else {
        state.misuse.push(format!(
            "memory requirements of the unknown {kind:?} {raw:#x}"
        ));
        return;
    };
    let setup = resource.setup;
    let requirements = unsafe { &mut *p_memory_requirements };
    requirements.memory_requirements = vk::MemoryRequirements {
        size: setup.size,
        alignment: setup.alignment,
        memory_type_bits: setup.memory_type_bits,
    };
    let mut next = requirements.p_next.cast::<vk::BaseOutStructure<'_>>();
    while !next.is_null() {
        let s_type = unsafe { (*next).s_type };
        if s_type == vk::StructureType::MEMORY_DEDICATED_REQUIREMENTS {
            let dedicated = unsafe { &mut *next.cast::<vk::MemoryDedicatedRequirements<'_>>() };
            dedicated.prefers_dedicated_allocation = vk::Bool32::from(setup.prefers_dedicated);
            dedicated.requires_dedicated_allocation = vk::Bool32::from(setup.requires_dedicated);
        } else {
            state.misuse.push(format!(
                "unexpected {s_type:?} in the chain of VkMemoryRequirements2"
            ));
        }
        next = unsafe { (*next).p_next };
    }
}

unsafe extern "system" fn get_buffer_memory_requirements2(
    device: vk::Device,
    p_info: *const vk::BufferMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut vk::MemoryRequirements2<'_>,
) {
    let buffer = unsafe { (*p_info).buffer };
    unsafe {
        write_requirements(
            device,
            buffer.as_raw(),
            ResourceKind::Buffer,
            p_memory_requirements,
        )
    };
}

unsafe extern "system" fn get_image_memory_requirements2(
    device: vk::Device,
    p_info: *const vk::ImageMemoryRequirementsInfo2<'_>,
    p_memory_requirements: *mut vk::MemoryRequirements2<'_>,
) {
    let image = unsafe { (*p_info).image };
    unsafe {
        write_requirements(
            device,
            image.as_raw(),
            ResourceKind::Image,
            p_memory_requirements,
        )
    };
}

unsafe extern "system" fn get_image_drm_format_modifier_properties(
    device: vk::Device,
    image: vk::Image,
    p_properties: *mut vk::ImageDrmFormatModifierPropertiesEXT<'_>,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    state.calls.query_modifier += 1;
    let raw = image.as_raw();
    match state.resources.get(&raw).copied() {
        Some(resource)
            if resource.kind == ResourceKind::Image && resource.has_drm_format_modifier =>
        {
            if let Some(result) = state.next_modifier_query_fails.take() {
                return result;
            }
            unsafe { (*p_properties).drm_format_modifier = resource.setup.drm_format_modifier };
            vk::VkResult::SUCCESS
        }
        _ => {
            state.misuse.push(format!(
                "vkGetImageDrmFormatModifierPropertiesEXT of {raw:#x}, which is not a live \
                 image with DRM_FORMAT_MODIFIER_EXT tiling"
            ));
            vk::VkResult::ERROR_UNKNOWN
        }
    }
}

fn bind_problem(
    state: &State,
    raw: u64,
    resource: &ResourceObject,
    memory: vk::DeviceMemory,
    offset: u64,
) -> Option<String> {
    if resource.bound.is_some() {
        return Some("the resource is already bound".to_string());
    }
    if resource.bound_in_parts {
        return Some("the resource is sparse or disjoint".to_string());
    }
    let Some(entry) = state.memories.get(&memory.0) else {
        return Some(format!("{memory:?} is not live memory"));
    };
    let setup = resource.setup;
    if setup.memory_type_bits & (1 << entry.object.memory_type_index) == 0 {
        return Some(format!(
            "memory type {} is not in the memory type bits {:#b}",
            entry.object.memory_type_index, setup.memory_type_bits
        ));
    }
    if !offset.is_multiple_of(setup.alignment.max(1)) {
        return Some(format!(
            "offset {offset} is not a multiple of {}",
            setup.alignment
        ));
    }
    if offset
        .checked_add(setup.size)
        .is_none_or(|end| end > entry.object.size)
    {
        return Some(format!(
            "{} bytes at offset {offset} do not fit in {} bytes",
            setup.size, entry.object.size
        ));
    }
    match entry.chain.dedicated {
        Some(dedicated) if dedicated != resource.handle(raw) => {
            return Some(format!("the memory is dedicated to {dedicated:?}"));
        }
        Some(_) if offset != 0 => return Some(format!("dedicated memory at offset {offset}")),
        None if setup.requires_dedicated => {
            return Some("the resource requires a dedicated allocation".to_string());
        }
        _ => {}
    }
    let flags = entry
        .chain
        .flags
        .map_or(vk::MemoryAllocateFlags::empty(), |(flags, _)| flags);
    if resource.needs_device_address && !flags.contains(vk::MemoryAllocateFlags::DEVICE_ADDRESS) {
        return Some("the buffer needs memory with DEVICE_ADDRESS".to_string());
    }
    if let Some(export) = entry.chain.export
        && !export.is_empty()
        && !export.intersects(resource.export_handle_types)
    {
        return Some(format!(
            "the memory is exportable as {export:?}, the resource was created for {:?}",
            resource.export_handle_types
        ));
    }
    None
}

fn bind(
    device: vk::Device,
    raw: u64,
    kind: ResourceKind,
    memory: vk::DeviceMemory,
    offset: u64,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    state.calls.bind += 1;
    let Some(resource) = state
        .resources
        .get(&raw)
        .copied()
        .filter(|resource| resource.kind == kind)
    else {
        state
            .misuse
            .push(format!("binding the unknown {kind:?} {raw:#x}"));
        return vk::VkResult::ERROR_UNKNOWN;
    };
    if let Some(problem) = bind_problem(&state, raw, &resource, memory, offset) {
        state.misuse.push(format!(
            "binding {kind:?} {raw:#x} to {memory:?} at {offset}: {problem}"
        ));
    }
    if let Some(result) = state.next_bind_fails.take() {
        return result;
    }
    if let Some(resource) = state.resources.get_mut(&raw) {
        resource.bound = Some((memory, offset));
    }
    vk::VkResult::SUCCESS
}

unsafe extern "system" fn bind_buffer_memory(
    device: vk::Device,
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
    offset: vk::DeviceSize,
) -> vk::VkResult {
    bind(
        device,
        buffer.as_raw(),
        ResourceKind::Buffer,
        memory,
        offset,
    )
}

unsafe extern "system" fn bind_image_memory(
    device: vk::Device,
    image: vk::Image,
    memory: vk::DeviceMemory,
    offset: vk::DeviceSize,
) -> vk::VkResult {
    bind(device, image.as_raw(), ResourceKind::Image, memory, offset)
}
