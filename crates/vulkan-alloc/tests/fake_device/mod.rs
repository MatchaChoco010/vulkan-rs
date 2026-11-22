#![allow(dead_code)]

use std::collections::HashMap;
use std::ffi::{CStr, c_char, c_void};
use std::sync::Mutex;

use vulkan::{Device, vk};
use vulkan_alloc::{
    AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc, MemoryLocation,
};

pub const DEVICE_LOCAL: u32 = 0;
pub const HOST_COHERENT: u32 = 1;
pub const HOST_NON_COHERENT: u32 = 2;

pub const BLOCK_SIZE: u64 = 4096;
pub const NON_COHERENT_ATOM_SIZE: u64 = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryObject {
    pub size: u64,
    pub memory_type_index: u32,
    pub mapped: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Calls {
    pub allocate: usize,
    pub free: usize,
    pub map: usize,
    pub unmap: usize,
}

struct Memory {
    object: MemoryObject,
    storage: Option<Box<[u8]>>,
}

#[derive(Default)]
struct State {
    memories: HashMap<u64, Memory>,
    released_handles: Vec<u64>,
    last_handle: u64,
    calls: Calls,
    misuse: Vec<String>,
}

pub struct FakeDevice {
    state: *mut Mutex<State>,
    device: Device,
}

impl FakeDevice {
    pub fn new() -> Self {
        let state = Box::into_raw(Box::new(Mutex::new(State::default())));
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
            buffer_image_granularity: 1,
            block_size: BLOCK_SIZE,
        })
    }

    pub fn memory_objects(&self) -> HashMap<vk::DeviceMemory, MemoryObject> {
        self.state()
            .memories
            .iter()
            .map(|(&handle, memory)| (vk::DeviceMemory(handle), memory.object))
            .collect()
    }

    pub fn calls(&self) -> Calls {
        self.state().calls
    }

    pub fn misuse(&self) -> Vec<String> {
        self.state().misuse.clone()
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

pub fn desc(size: u64, alignment: u64, memory_type_index: u32) -> AllocationCreateDesc<'static> {
    AllocationCreateDesc {
        name: None,
        requirements: vk::MemoryRequirements {
            size,
            alignment,
            memory_type_bits: 1 << memory_type_index,
        },
        location: MemoryLocation::custom(
            vk::MemoryPropertyFlags::empty(),
            vk::MemoryPropertyFlags::empty(),
        ),
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }
}

pub struct Random(u64);

impl Random {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    pub fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

unsafe fn state_of<'a>(device: vk::Device) -> std::sync::MutexGuard<'a, State> {
    unsafe { &*device.0.cast::<Mutex<State>>() }.lock().unwrap()
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    p_name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    type Void = unsafe extern "system" fn();
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
            b"vkFlushMappedMemoryRanges" | b"vkInvalidateMappedMemoryRanges" => {
                Some(std::mem::transmute::<
                    unsafe extern "system" fn(
                        vk::Device,
                        u32,
                        *const vk::MappedMemoryRange<'_>,
                    ) -> vk::VkResult,
                    Void,
                >(synchronize_mapped_memory_ranges))
            }
            _ => None,
        }
    }
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
    let handle = state.released_handles.pop().unwrap_or_else(|| {
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
        state.released_handles.push(memory.0);
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
    let problem = match state.memories.get_mut(&memory.0) {
        None => Some(format!("vkMapMemory of unknown memory {memory:?}")),
        Some(entry) if entry.object.mapped => {
            Some(format!("vkMapMemory of already mapped memory {memory:?}"))
        }
        Some(entry) if offset != 0 || size != entry.object.size => Some(format!(
            "vkMapMemory of {memory:?} with offset {offset} and size {size}"
        )),
        Some(entry) => {
            entry.object.mapped = true;
            let storage = entry
                .storage
                .insert(vec![0; entry.object.size as usize].into_boxed_slice());
            unsafe { pp_data.write(storage.as_mut_ptr().cast()) };
            None
        }
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
        Some(entry) if entry.object.mapped => {
            entry.object.mapped = false;
            entry.storage = None;
        }
        _ => state.misuse.push(format!(
            "vkUnmapMemory of memory {memory:?} that is not mapped"
        )),
    }
}

unsafe extern "system" fn synchronize_mapped_memory_ranges(
    device: vk::Device,
    memory_range_count: u32,
    p_memory_ranges: *const vk::MappedMemoryRange<'_>,
) -> vk::VkResult {
    let mut state = unsafe { state_of(device) };
    let ranges =
        unsafe { std::slice::from_raw_parts(p_memory_ranges, memory_range_count as usize) };
    for range in ranges {
        let mapped = state
            .memories
            .get(&range.memory.0)
            .is_some_and(|entry| entry.object.mapped);
        if !mapped {
            state.misuse.push(format!(
                "synchronized a range of memory {:?} that is not mapped",
                range.memory
            ));
        }
    }
    vk::VkResult::SUCCESS
}
