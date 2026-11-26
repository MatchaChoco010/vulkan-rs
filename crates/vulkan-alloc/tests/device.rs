use std::ffi::{CStr, c_void};
use std::os::fd::{FromRawFd, OwnedFd};
use std::ptr::NonNull;
use std::sync::{Mutex, MutexGuard, PoisonError};

use proptest::prelude::*;
use proptest::sample::Index;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationError, AllocationScheme, Allocator,
    AllocatorCreateDesc, AllocatorSnapshot, MemoryLocation,
};

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

static LOADER_INSTANCE_AND_DEVICE_LISTS: Mutex<()> = Mutex::new(());

fn exclusive_access_to_the_loaders_instance_and_device_lists() -> MutexGuard<'static, ()> {
    LOADER_INSTANCE_AND_DEVICE_LISTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

struct Gpu {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    messages: Box<Mutex<Vec<String>>>,
    physical_device: vk::PhysicalDevice,
    device: Device,
    external_memory_fd: bool,
    destroyed: bool,
}

unsafe extern "system" fn collect_message(
    severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    user_data: *mut c_void,
) -> vk::Bool32 {
    unsafe {
        let messages = &*user_data.cast::<Mutex<Vec<String>>>();
        let message = CStr::from_ptr((*data).p_message).to_string_lossy();
        messages
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(format!("{severity:?}: {message}"));
    }
    vk::FALSE
}

fn message_collector(
    messages: &Mutex<Vec<String>>,
) -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
        )
        .pfn_user_callback(Some(collect_message))
        .user_data(std::ptr::from_ref(messages).cast_mut().cast())
}

unsafe fn validation_layer_extensions(entry: &Entry) -> Result<Vec<&'static CStr>, String> {
    unsafe {
        let layers = entry
            .enumerate_instance_layer_properties()
            .map_err(|e| e.to_string())?;
        let has_layer = layers
            .iter()
            .any(|layer| layer.layer_name_as_c_str() == Ok(VALIDATION_LAYER));
        if !has_layer {
            return Ok(Vec::new());
        }
        let extensions = entry
            .enumerate_instance_extension_properties(Some(VALIDATION_LAYER))
            .map_err(|e| e.to_string())?;
        Ok([vk::EXT_DEBUG_UTILS_NAME, vk::EXT_LAYER_SETTINGS_NAME]
            .into_iter()
            .filter(|name| {
                extensions
                    .iter()
                    .any(|e| e.extension_name_as_c_str() == Ok(name))
            })
            .collect())
    }
}

impl Gpu {
    fn new() -> Option<Self> {
        match unsafe { Self::create() } {
            Ok(gpu) => Some(gpu),
            Err(reason) => {
                assert!(
                    std::env::var_os("VULKAN_RS_REQUIRE_DEVICE_TEST").is_none(),
                    "device test is required but cannot run: {reason}"
                );
                eprintln!("skipping device test: {reason}");
                None
            }
        }
    }

    unsafe fn create() -> Result<Self, String> {
        unsafe {
            let entry = Entry::load().map_err(|e| e.to_string())?;
            let layer_extensions = validation_layer_extensions(&entry)?;
            let validation = layer_extensions.contains(&vk::EXT_DEBUG_UTILS_NAME);
            if !validation {
                eprintln!(
                    "skipping validation: {} with {} is not available",
                    VALIDATION_LAYER.to_string_lossy(),
                    vk::EXT_DEBUG_UTILS_NAME.to_string_lossy()
                );
            }
            let messages = Box::new(Mutex::new(Vec::new()));
            let mut collector = message_collector(&messages);
            let layers = [vk::CStrPtr::new(VALIDATION_LAYER)];
            let instance_extensions = layer_extensions
                .iter()
                .map(|name| vk::CStrPtr::new(name))
                .collect::<Vec<_>>();
            let every_message = [vk::FALSE];
            let settings = [vk::LayerSettingEXT::default()
                .layer_name(VALIDATION_LAYER)
                .setting_name(c"enable_message_limit")
                .ty(vk::LayerSettingTypeEXT::BOOL32)
                .values(&every_message)];
            let mut layer_settings = vk::LayerSettingsCreateInfoEXT::default().settings(&settings);
            let app = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_2);
            let mut instance_info = vk::InstanceCreateInfo::default().application_info(&app);
            if validation {
                instance_info = instance_info
                    .enabled_layer_names(&layers)
                    .enabled_extension_names(&instance_extensions)
                    .push_next(&mut collector);
                if layer_extensions.contains(&vk::EXT_LAYER_SETTINGS_NAME) {
                    instance_info = instance_info.push_next(&mut layer_settings);
                }
            }
            let serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
            let instance = entry
                .create_instance(&instance_info, None)
                .map_err(|e| format!("vkCreateInstance: {e}"))?;
            drop(serialized);
            let messenger = if validation {
                match instance.create_debug_utils_messenger_ext(&message_collector(&messages), None)
                {
                    Ok(messenger) => messenger,
                    Err(e) => {
                        instance.destroy_instance(None);
                        return Err(format!("vkCreateDebugUtilsMessengerEXT: {e}"));
                    }
                }
            } else {
                vk::DebugUtilsMessengerEXT::null()
            };
            match Self::create_device(&instance) {
                Ok((physical_device, device, external_memory_fd)) => Ok(Self {
                    instance,
                    messenger,
                    messages,
                    physical_device,
                    device,
                    external_memory_fd,
                    destroyed: false,
                }),
                Err(reason) => {
                    if !messenger.is_null() {
                        instance.destroy_debug_utils_messenger_ext(messenger, None);
                    }
                    instance.destroy_instance(None);
                    Err(reason)
                }
            }
        }
    }

    unsafe fn create_device(
        instance: &Instance,
    ) -> Result<(vk::PhysicalDevice, Device, bool), String> {
        unsafe {
            let mut selected = None;
            for physical_device in instance
                .enumerate_physical_devices()
                .map_err(|e| e.to_string())?
            {
                let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
                let mut features = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan12);
                instance.get_physical_device_features2(physical_device, &mut features);
                let properties = instance.get_physical_device_properties(physical_device);
                if properties.api_version >= vk::API_VERSION_1_2
                    && vulkan12.buffer_device_address != vk::FALSE
                {
                    selected = Some(physical_device);
                    break;
                }
            }
            let Some(physical_device) = selected else {
                return Err("no Vulkan 1.2 device with bufferDeviceAddress".to_string());
            };

            let extensions = instance
                .enumerate_device_extension_properties(physical_device, None)
                .map_err(|e| e.to_string())?;
            let has = |name: &CStr| {
                extensions
                    .iter()
                    .any(|e| e.extension_name_as_c_str() == Ok(name))
            };
            let external_memory_fd = has(vk::KHR_EXTERNAL_MEMORY_FD_NAME);
            let mut extension_names = Vec::new();
            if external_memory_fd {
                extension_names.push(vk::CStrPtr::new(vk::KHR_EXTERNAL_MEMORY_FD_NAME));
            }

            let priorities = [1.0];
            let queues = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(0)
                .queue_priorities(&priorities)];
            let mut vulkan12 =
                vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
            let serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
            let device = instance
                .create_device(
                    physical_device,
                    &vk::DeviceCreateInfo::default()
                        .queue_create_infos(&queues)
                        .enabled_extension_names(&extension_names)
                        .push_next(&mut vulkan12),
                    None,
                )
                .map_err(|e| format!("vkCreateDevice: {e}"))?;
            drop(serialized);
            Ok((physical_device, device, external_memory_fd))
        }
    }

    fn take_validation_messages(&self) -> Vec<String> {
        std::mem::take(&mut *self.messages.lock().unwrap_or_else(PoisonError::into_inner))
    }

    fn assert_no_validation_messages(&self) {
        let messages = self.take_validation_messages();
        assert!(
            messages.is_empty(),
            "validation messages:\n{}",
            messages.join("\n")
        );
    }

    fn memory_properties(&self) -> vk::PhysicalDeviceMemoryProperties {
        unsafe {
            self.instance
                .get_physical_device_memory_properties(self.physical_device)
        }
    }

    fn allocator(&self) -> Allocator {
        self.allocator_with_block_size(None)
    }

    fn allocator_with_block_size(&self, block_size: Option<u64>) -> Allocator {
        let mut desc = AllocatorCreateDesc::new(&self.device, self.memory_properties(), unsafe {
            self.instance
                .get_physical_device_properties(self.physical_device)
        });
        if let Some(block_size) = block_size {
            desc.block_size = block_size;
        }
        Allocator::new(desc)
    }

    unsafe fn buffer(&self, usage: vk::BufferUsageFlags) -> vk::Buffer {
        unsafe {
            self.device
                .create_buffer(
                    &vk::BufferCreateInfo::default().size(4096).usage(usage),
                    None,
                )
                .unwrap()
        }
    }
}

impl Gpu {
    fn destroy(&mut self) -> Vec<String> {
        if !self.destroyed {
            self.destroyed = true;
            unsafe {
                let serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
                self.device.destroy_device(None);
                drop(serialized);
                if !self.messenger.is_null() {
                    self.instance
                        .destroy_debug_utils_messenger_ext(self.messenger, None);
                }
                self.instance.destroy_instance(None);
            }
        }
        self.take_validation_messages()
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        let messages = self.destroy();
        if !messages.is_empty() && !std::thread::panicking() {
            panic!(
                "validation messages while destroying the device:\n{}",
                messages.join("\n")
            );
        }
    }
}

fn desc(
    requirements: vk::MemoryRequirements,
    location: MemoryLocation,
) -> AllocationCreateDesc<'static> {
    AllocationCreateDesc {
        name: None,
        requirements,
        location,
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }
}

#[test]
fn mapped_allocations_hold_data_and_share_a_block() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let first = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let second = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let requirements = gpu.device.get_buffer_memory_requirements(first);
        let a = allocator
            .allocate(desc(requirements, MemoryLocation::CpuToGpu))
            .unwrap();
        let b = allocator
            .allocate(desc(requirements, MemoryLocation::CpuToGpu))
            .unwrap();
        assert_eq!(a.memory(), b.memory());
        assert_ne!(a.offset(), b.offset());
        gpu.device
            .bind_buffer_memory(first, a.memory(), a.offset())
            .unwrap();
        gpu.device
            .bind_buffer_memory(second, b.memory(), b.offset())
            .unwrap();

        {
            let mut mapped = allocator.map(&a).unwrap();
            mapped
                .byte_range_mut(0, 4)
                .unwrap()
                .copy_from_slice(&[1, 2, 3, 4]);
        }
        {
            let mapped = allocator.map(&a).unwrap();
            assert_eq!(mapped.byte_range(0, 4), Some(&[1, 2, 3, 4][..]));
        }

        gpu.device.destroy_buffer(first, None);
        gpu.device.destroy_buffer(second, None);
        allocator.free(a).unwrap();
        allocator.free(b).unwrap();
    }
    assert!(allocator.is_empty());
    gpu.assert_no_validation_messages();
}

#[test]
fn allocations_with_different_allocate_flags_do_not_share_memory() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let plain_buffer = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let address_buffer = gpu.buffer(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS);

        let plain = allocator
            .allocate(desc(
                gpu.device.get_buffer_memory_requirements(plain_buffer),
                MemoryLocation::CpuToGpu,
            ))
            .unwrap();
        let with_address = allocator
            .allocate(AllocationCreateDesc {
                memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
                ..desc(
                    gpu.device.get_buffer_memory_requirements(address_buffer),
                    MemoryLocation::CpuToGpu,
                )
            })
            .unwrap();
        if plain.memory_type_index() == with_address.memory_type_index() {
            assert_ne!(plain.memory(), with_address.memory());
        }

        gpu.device
            .bind_buffer_memory(plain_buffer, plain.memory(), plain.offset())
            .unwrap();
        gpu.device
            .bind_buffer_memory(address_buffer, with_address.memory(), with_address.offset())
            .unwrap();
        let address = gpu.device.get_buffer_device_address(
            &vk::BufferDeviceAddressInfo::default().buffer(address_buffer),
        );
        assert_ne!(address, 0);

        gpu.device.destroy_buffer(plain_buffer, None);
        gpu.device.destroy_buffer(address_buffer, None);
        allocator.free(plain).unwrap();
        allocator.free(with_address).unwrap();
    }
    assert!(allocator.is_empty());
    gpu.assert_no_validation_messages();
}

#[test]
fn exportable_allocations_are_not_suballocated() {
    let Some(gpu) = Gpu::new() else { return };
    if !gpu.external_memory_fd {
        eprintln!("skipping: VK_KHR_external_memory_fd is not supported");
        return;
    }
    let mut allocator = gpu.allocator();
    unsafe {
        let handle_type = vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC;
        let exportable_buffer = || {
            let mut external =
                vk::ExternalMemoryBufferCreateInfo::default().handle_types(handle_type.into());
            gpu.device
                .create_buffer(
                    &vk::BufferCreateInfo::default()
                        .size(4096)
                        .usage(usage)
                        .push_next(&mut external),
                    None,
                )
                .unwrap()
        };
        let first = exportable_buffer();
        let second = exportable_buffer();
        let requirements = gpu.device.get_buffer_memory_requirements(first);
        let exportable = || AllocationCreateDesc {
            export_handle_types: handle_type.into(),
            ..desc(requirements, MemoryLocation::GpuOnly)
        };
        let a = allocator.allocate(exportable()).unwrap();
        let b = allocator.allocate(exportable()).unwrap();
        assert_ne!(a.memory(), b.memory());
        assert_eq!(a.offset(), 0);
        gpu.device.bind_buffer_memory(first, a.memory(), 0).unwrap();
        gpu.device
            .bind_buffer_memory(second, b.memory(), 0)
            .unwrap();

        let fd = gpu
            .device
            .get_memory_fd_khr(
                &vk::MemoryGetFdInfoKHR::default()
                    .memory(a.memory())
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= 0);
        drop(OwnedFd::from_raw_fd(fd));

        gpu.device.destroy_buffer(first, None);
        gpu.device.destroy_buffer(second, None);
        allocator.free(a).unwrap();
        allocator.free(b).unwrap();
    }
    assert!(allocator.is_empty());
    gpu.assert_no_validation_messages();
}

struct CapturedLog;

static CAPTURED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

impl log::Log for CapturedLog {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    fn log(&self, record: &log::Record<'_>) {
        CAPTURED
            .lock()
            .unwrap()
            .push(format!("{} {}", record.level(), record.args()));
    }

    fn flush(&self) {}
}

#[test]
fn dropping_with_live_allocations_reports_them_and_frees_their_memory() {
    let Some(mut gpu) = Gpu::new() else { return };
    log::set_logger(&CapturedLog).expect("no other logger in this test binary");
    log::set_max_level(log::LevelFilter::Error);

    let mut allocator = gpu.allocator();
    let buffer;
    let leaked_memory;
    unsafe {
        buffer = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let leaked = allocator
            .allocate(AllocationCreateDesc {
                name: Some("script texture"),
                ..desc(
                    gpu.device.get_buffer_memory_requirements(buffer),
                    MemoryLocation::GpuOnly,
                )
            })
            .unwrap();
        gpu.device
            .bind_buffer_memory(buffer, leaked.memory(), leaked.offset())
            .unwrap();
        leaked_memory = leaked.memory();
    }

    let live = allocator.live_allocations();
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].name.as_deref(), Some("script texture"));
    assert_eq!(live[0].memory, leaked_memory);

    unsafe { gpu.device.destroy_buffer(buffer, None) };
    drop(allocator);

    let records = CAPTURED.lock().unwrap().join("\n");
    assert!(
        records.contains("ERROR vulkan_alloc::Allocator dropped with 1 live allocation(s)"),
        "{records}"
    );
    assert!(records.contains("\"script texture\""), "{records}");
    gpu.assert_no_validation_messages();
    let messages = gpu.destroy();
    assert!(
        messages.is_empty(),
        "dropping the allocator must have freed the memory of the live allocation, so that no \
         VkDeviceMemory is left when the device is destroyed:\n{}",
        messages.join("\n")
    );
}

#[test]
fn free_all_releases_memory_that_still_has_live_allocations() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let buffer = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let allocation = allocator
            .allocate(desc(
                gpu.device.get_buffer_memory_requirements(buffer),
                MemoryLocation::CpuToGpu,
            ))
            .unwrap();
        gpu.device
            .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
            .unwrap();
        {
            let mapped = allocator.map(&allocation).unwrap();
            assert!(mapped.byte_range(0, 4096).is_some());
            assert!(mapped.byte_range(4096, usize::MAX).is_none());
            assert!(mapped.byte_range(usize::MAX - 1, 1).is_none());
        }

        gpu.device.destroy_buffer(buffer, None);
        assert_eq!(allocator.free_all(), 1);
    }
    assert!(allocator.is_empty());
    assert!(allocator.live_allocations().is_empty());
    gpu.assert_no_validation_messages();
}

#[test]
fn persistent_mappings_last_until_the_allocation_is_freed() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let first = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let second = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let requirements = gpu.device.get_buffer_memory_requirements(first);
        let a = allocator
            .allocate(desc(requirements, MemoryLocation::CpuToGpu))
            .unwrap();
        let b = allocator
            .allocate(desc(requirements, MemoryLocation::CpuToGpu))
            .unwrap();
        gpu.device
            .bind_buffer_memory(first, a.memory(), a.offset())
            .unwrap();
        gpu.device
            .bind_buffer_memory(second, b.memory(), b.offset())
            .unwrap();

        assert!(allocator.persistent_ptr(&a).is_none());
        let pointer = allocator.map_persistent(&a).unwrap();
        let other = allocator.map_persistent(&b).unwrap();
        assert_ne!(pointer, other);
        assert_eq!(allocator.map_persistent(&a).unwrap(), pointer);
        assert_eq!(allocator.persistent_ptr(&a), Some(pointer));

        let bytes = std::slice::from_raw_parts_mut(pointer.as_ptr(), a.size() as usize);
        bytes[..4].copy_from_slice(&[9, 8, 7, 6]);
        {
            let mapped = allocator.map(&a).unwrap();
            assert_eq!(mapped.byte_range(0, 4), Some(&[9, 8, 7, 6][..]));
        }
        assert_eq!(allocator.persistent_ptr(&a), Some(pointer));

        gpu.device.destroy_buffer(first, None);
        allocator.free(a).unwrap();
        assert_eq!(allocator.persistent_ptr(&b), Some(other));
        std::slice::from_raw_parts_mut(other.as_ptr(), 4).copy_from_slice(&[1, 2, 3, 4]);

        gpu.device.destroy_buffer(second, None);
        allocator.free(b).unwrap();
    }
    assert!(allocator.is_empty());
    gpu.assert_no_validation_messages();
}

#[test]
fn unmapping_a_persistent_mapping_is_an_error() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let buffer = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let allocation = allocator
            .allocate(desc(
                gpu.device.get_buffer_memory_requirements(buffer),
                MemoryLocation::CpuToGpu,
            ))
            .unwrap();
        let pointer = allocator.map_persistent(&allocation).unwrap();

        assert_eq!(
            allocator.unmap_memory(&allocation),
            Err(AllocationError::InvalidAllocation)
        );
        assert_eq!(allocator.persistent_ptr(&allocation), Some(pointer));

        allocator.map_memory(&allocation).unwrap();
        allocator.unmap_memory(&allocation).unwrap();
        assert_eq!(allocator.persistent_ptr(&allocation), Some(pointer));

        gpu.device.destroy_buffer(buffer, None);
        allocator.free(allocation).unwrap();
    }
    assert!(allocator.is_empty());
    gpu.assert_no_validation_messages();
}

#[test]
fn allocator_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Allocator>();
}

const SEQUENCE_BLOCK_SIZE: u64 = 64 * 1024;
const SEQUENCE_LIVE_BYTES_LIMIT: u64 = 4 * 1024 * 1024;
const LONGEST_SEQUENCE: usize = 64;

macro_rules! ensure {
    ($condition:expr, $($message:tt)*) => {
        if !$condition {
            return Err(format!($($message)*));
        }
    };
}

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

#[derive(Debug, Clone)]
enum Op {
    Buffer {
        size: u64,
        usage: vk::BufferUsageFlags,
        location: MemoryLocation,
        dedicated: bool,
    },
    Image {
        width: u32,
        height: u32,
        linear: bool,
        location: MemoryLocation,
        dedicated: bool,
    },
    Fill {
        target: Index,
        persistent: bool,
    },
    Free {
        target: Index,
    },
    FreeAll,
}

fn buffer_size() -> impl Strategy<Value = u64> {
    prop_oneof![
        3 => 1..=16 * 1024u64,
        2 => 16 * 1024..=320 * 1024u64,
        3 => prop::sample::select(vec![
            1,
            255,
            256,
            257,
            4095,
            4096,
            4097,
            SEQUENCE_BLOCK_SIZE / 2,
            SEQUENCE_BLOCK_SIZE - 256,
            SEQUENCE_BLOCK_SIZE - 1,
            SEQUENCE_BLOCK_SIZE,
            SEQUENCE_BLOCK_SIZE + 1,
            2 * SEQUENCE_BLOCK_SIZE,
        ]),
    ]
}

fn buffer_usage() -> impl Strategy<Value = vk::BufferUsageFlags> {
    prop::sample::select(vec![
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER,
        vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS | vk::BufferUsageFlags::STORAGE_BUFFER,
    ])
}

fn location() -> impl Strategy<Value = MemoryLocation> {
    prop::sample::select(vec![
        MemoryLocation::GpuOnly,
        MemoryLocation::CpuToGpu,
        MemoryLocation::GpuToCpu,
    ])
}

fn image_extent() -> impl Strategy<Value = u32> {
    prop_oneof![
        1..=128u32,
        prop::sample::select(vec![1, 2, 63, 64, 65, 128])
    ]
}

fn operation() -> impl Strategy<Value = Op> {
    prop_oneof![
        8 => (buffer_size(), buffer_usage(), location(), prop::bool::weighted(0.15)).prop_map(
            |(size, usage, location, dedicated)| Op::Buffer {
                size,
                usage,
                location,
                dedicated,
            }
        ),
        4 => (
            image_extent(),
            image_extent(),
            prop::bool::weighted(0.3),
            location(),
            prop::bool::weighted(0.15),
        )
            .prop_map(|(width, height, linear, location, dedicated)| Op::Image {
                width,
                height,
                linear,
                location,
                dedicated,
            }),
        6 => (any::<Index>(), any::<bool>())
            .prop_map(|(target, persistent)| Op::Fill { target, persistent }),
        6 => any::<Index>().prop_map(|target| Op::Free { target }),
        1 => Just(Op::FreeAll),
    ]
}

fn operations() -> impl Strategy<Value = Vec<Op>> {
    prop::collection::vec(operation(), 1..=LONGEST_SEQUENCE)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ending {
    FreeAllThenDrop,
    Drop,
}

fn sequences() -> impl Strategy<Value = (Vec<Op>, Ending)> {
    (
        operations(),
        prop::sample::select(vec![Ending::FreeAllThenDrop, Ending::Drop]),
    )
}

#[derive(Debug, Clone, Copy)]
enum Resource {
    Buffer(vk::Buffer),
    Image(vk::Image),
}

struct Live {
    allocation: Allocation,
    resource: Resource,
    requirements: vk::MemoryRequirements,
    location: MemoryLocation,
    linear: bool,
    device_address: bool,
    dedicated: bool,
    contents: Option<u8>,
    persistent: Option<NonNull<u8>>,
}

#[derive(Debug, Default)]
struct Coverage {
    buffers: usize,
    images: usize,
    linear_images: usize,
    dedicated: usize,
    device_address: usize,
    larger_than_a_block: usize,
    into_a_block_with_another_allocation: usize,
    blocks_emptied_by_free: usize,
    into_an_empty_block: usize,
    drops_that_free_memory: usize,
    guard_fills: usize,
    persistent_fills: usize,
    refills: usize,
    reads: usize,
    fills_with_nothing_live: usize,
    frees_with_nothing_live: usize,
    free_alls_with_live_allocations: usize,
    over_the_live_bytes_limit: usize,
}

impl Coverage {
    fn merge(&mut self, other: &Coverage) {
        self.buffers += other.buffers;
        self.images += other.images;
        self.linear_images += other.linear_images;
        self.dedicated += other.dedicated;
        self.device_address += other.device_address;
        self.larger_than_a_block += other.larger_than_a_block;
        self.into_a_block_with_another_allocation += other.into_a_block_with_another_allocation;
        self.blocks_emptied_by_free += other.blocks_emptied_by_free;
        self.into_an_empty_block += other.into_an_empty_block;
        self.drops_that_free_memory += other.drops_that_free_memory;
        self.guard_fills += other.guard_fills;
        self.persistent_fills += other.persistent_fills;
        self.refills += other.refills;
        self.reads += other.reads;
        self.fills_with_nothing_live += other.fills_with_nothing_live;
        self.frees_with_nothing_live += other.frees_with_nothing_live;
        self.free_alls_with_live_allocations += other.free_alls_with_live_allocations;
        self.over_the_live_bytes_limit += other.over_the_live_bytes_limit;
    }
}

fn required_flags(location: MemoryLocation) -> vk::MemoryPropertyFlags {
    match location {
        MemoryLocation::GpuOnly => vk::MemoryPropertyFlags::DEVICE_LOCAL,
        MemoryLocation::CpuToGpu | MemoryLocation::GpuToCpu => {
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
        }
        MemoryLocation::Custom { required, .. } => required,
    }
}

fn holds_only(bytes: &[u8], byte: u8) -> bool {
    let pattern = [byte; 4096];
    bytes
        .chunks(pattern.len())
        .all(|chunk| chunk == &pattern[..chunk.len()])
}

struct Sequence<'g> {
    gpu: &'g Gpu,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    allocator: Allocator,
    live: Vec<Live>,
    fills: u8,
    coverage: Coverage,
}

impl<'g> Sequence<'g> {
    fn run(gpu: &'g Gpu, operations: &[Op], ending: Ending) -> Result<Coverage, String> {
        let mut sequence = Sequence {
            gpu,
            memory_properties: gpu.memory_properties(),
            allocator: gpu.allocator_with_block_size(Some(SEQUENCE_BLOCK_SIZE)),
            live: Vec::new(),
            fills: 0,
            coverage: Coverage::default(),
        };
        let result = unsafe { sequence.apply_all(operations) };
        unsafe { sequence.release_everything(ending) };
        let Sequence {
            allocator,
            mut coverage,
            ..
        } = sequence;
        if allocator.stats().block_count > 0 {
            coverage.drops_that_free_memory += 1;
        }
        drop(allocator);
        let messages = gpu.take_validation_messages();
        match result {
            Ok(()) if messages.is_empty() => Ok(coverage),
            Ok(()) => Err(format!(
                "validation messages while releasing the resources:\n{}",
                messages.join("\n")
            )),
            Err(e) if messages.is_empty() => Err(e),
            Err(e) => Err(format!(
                "{e}\nvalidation messages not checked yet:\n{}",
                messages.join("\n")
            )),
        }
    }

    unsafe fn apply_all(&mut self, operations: &[Op]) -> Result<(), String> {
        unsafe {
            for (step, operation) in operations.iter().enumerate() {
                self.apply(operation)
                    .and_then(|()| self.check())
                    .map_err(|e| format!("after operation {step} ({operation:?}): {e}"))?;
            }
            while let Some(live) = self.live.pop() {
                self.destroy(live.resource);
                self.allocator
                    .free(live.allocation)
                    .map_err(|e| format!("freeing what is left: {e:?}"))?;
            }
            ensure!(self.allocator.is_empty(), "the allocator is not empty");
            check_empty_blocks(&self.allocator.snapshot())?;
            self.check_messages()
        }
    }

    unsafe fn release_everything(&mut self, ending: Ending) {
        unsafe {
            for live in std::mem::take(&mut self.live) {
                self.destroy(live.resource);
            }
            if ending == Ending::FreeAllThenDrop {
                self.allocator.free_all();
            }
        }
    }

    unsafe fn destroy(&self, resource: Resource) {
        unsafe {
            match resource {
                Resource::Buffer(buffer) => self.gpu.device.destroy_buffer(buffer, None),
                Resource::Image(image) => self.gpu.device.destroy_image(image, None),
            }
        }
    }

    unsafe fn apply(&mut self, operation: &Op) -> Result<(), String> {
        unsafe {
            match *operation {
                Op::Buffer {
                    size,
                    usage,
                    location,
                    dedicated,
                } => self.allocate_buffer(size, usage, location, dedicated),
                Op::Image {
                    width,
                    height,
                    linear,
                    location,
                    dedicated,
                } => self.allocate_image(width, height, linear, location, dedicated),
                Op::Fill { target, persistent } => self.fill(target, persistent),
                Op::Free { target } => self.free(target),
                Op::FreeAll => self.free_all(),
            }
        }
    }

    unsafe fn allocate_buffer(
        &mut self,
        size: u64,
        usage: vk::BufferUsageFlags,
        location: MemoryLocation,
        dedicated: bool,
    ) -> Result<(), String> {
        unsafe {
            let buffer = self
                .gpu
                .device
                .create_buffer(
                    &vk::BufferCreateInfo::default().size(size).usage(usage),
                    None,
                )
                .map_err(|e| format!("vkCreateBuffer: {e}"))?;
            let requirements = self.gpu.device.get_buffer_memory_requirements(buffer);
            self.coverage.buffers += 1;
            self.allocate_and_bind(
                Resource::Buffer(buffer),
                requirements,
                location,
                true,
                usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS),
                dedicated,
            )
        }
    }

    unsafe fn allocate_image(
        &mut self,
        width: u32,
        height: u32,
        linear: bool,
        location: MemoryLocation,
        dedicated: bool,
    ) -> Result<(), String> {
        unsafe {
            let (tiling, usage) = if linear {
                (
                    vk::ImageTiling::LINEAR,
                    vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST,
                )
            } else {
                (
                    vk::ImageTiling::OPTIMAL,
                    vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST,
                )
            };
            let image = self
                .gpu
                .device
                .create_image(
                    &vk::ImageCreateInfo::default()
                        .image_type(vk::ImageType::_2D)
                        .format(vk::Format::R8G8B8A8_UNORM)
                        .extent(vk::Extent3D {
                            width,
                            height,
                            depth: 1,
                        })
                        .tiling(tiling)
                        .usage(usage),
                    None,
                )
                .map_err(|e| format!("vkCreateImage: {e}"))?;
            let requirements = self.gpu.device.get_image_memory_requirements(image);
            self.coverage.images += 1;
            if linear {
                self.coverage.linear_images += 1;
            }
            self.allocate_and_bind(
                Resource::Image(image),
                requirements,
                location,
                linear,
                false,
                dedicated,
            )
        }
    }

    unsafe fn allocate_and_bind(
        &mut self,
        resource: Resource,
        requirements: vk::MemoryRequirements,
        location: MemoryLocation,
        linear: bool,
        device_address: bool,
        dedicated: bool,
    ) -> Result<(), String> {
        unsafe {
            let live_bytes: u64 = self.live.iter().map(|live| live.allocation.size()).sum();
            if live_bytes + requirements.size > SEQUENCE_LIVE_BYTES_LIMIT {
                self.coverage.over_the_live_bytes_limit += 1;
                self.destroy(resource);
                return Ok(());
            }
            let allocation_scheme = match (dedicated, resource) {
                (false, _) => AllocationScheme::GpuAllocatorManaged,
                (true, Resource::Buffer(buffer)) => AllocationScheme::DedicatedBuffer(buffer),
                (true, Resource::Image(image)) => AllocationScheme::DedicatedImage(image),
            };
            let memory_allocate_flags = if device_address {
                vk::MemoryAllocateFlags::DEVICE_ADDRESS
            } else {
                vk::MemoryAllocateFlags::empty()
            };
            let empty_blocks = self
                .allocator
                .snapshot()
                .memory_types
                .iter()
                .flat_map(|memory_type| &memory_type.blocks)
                .filter(|block| block.allocated_region_count == 0)
                .map(|block| block.memory)
                .collect::<Vec<_>>();
            let allocation = match self.allocator.allocate(AllocationCreateDesc {
                name: None,
                requirements,
                location,
                linear,
                memory_allocate_flags,
                export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
                allocation_scheme,
            }) {
                Ok(allocation) => allocation,
                Err(e) => {
                    self.destroy(resource);
                    return Err(format!(
                        "allocate {} bytes aligned to {} in memory types {:#b}: {e}",
                        requirements.size, requirements.alignment, requirements.memory_type_bits
                    ));
                }
            };
            let bound = match resource {
                Resource::Buffer(buffer) => self.gpu.device.bind_buffer_memory(
                    buffer,
                    allocation.memory(),
                    allocation.offset(),
                ),
                Resource::Image(image) => self.gpu.device.bind_image_memory(
                    image,
                    allocation.memory(),
                    allocation.offset(),
                ),
            };
            if self
                .live
                .iter()
                .any(|live| live.allocation.memory() == allocation.memory())
            {
                self.coverage.into_a_block_with_another_allocation += 1;
            }
            if empty_blocks.contains(&allocation.memory()) {
                self.coverage.into_an_empty_block += 1;
            }
            if dedicated {
                self.coverage.dedicated += 1;
            }
            if device_address {
                self.coverage.device_address += 1;
            }
            if requirements.size > SEQUENCE_BLOCK_SIZE {
                self.coverage.larger_than_a_block += 1;
            }
            self.live.push(Live {
                allocation,
                resource,
                requirements,
                location,
                linear,
                device_address,
                dedicated,
                contents: None,
                persistent: None,
            });
            bound.map_err(|e| format!("binding the resource: {e}"))?;
            self.check_placement(self.live.last().unwrap())
        }
    }

    fn check_placement(&self, live: &Live) -> Result<(), String> {
        let allocation = &live.allocation;
        let requirements = live.requirements;
        ensure!(
            allocation
                .offset()
                .is_multiple_of(requirements.alignment.max(1)),
            "offset {} is not a multiple of the alignment {}",
            allocation.offset(),
            requirements.alignment
        );
        ensure!(
            allocation.size() >= requirements.size,
            "size {} is less than the requested {}",
            allocation.size(),
            requirements.size
        );
        let index = allocation.memory_type_index();
        ensure!(
            index < self.memory_properties.memory_type_count
                && requirements.memory_type_bits & (1 << index) != 0,
            "memory type {index} is not in the allowed bits {:#b}",
            requirements.memory_type_bits
        );
        let flags = self.memory_properties.memory_types[index as usize].property_flags;
        ensure!(
            flags.contains(required_flags(live.location)),
            "memory type {index} has {flags:?}, which {:?} does not allow",
            live.location
        );
        Ok(())
    }

    fn pick(&self, target: Index) -> Option<usize> {
        (!self.live.is_empty()).then(|| target.index(self.live.len()))
    }

    unsafe fn fill(&mut self, target: Index, persistent: bool) -> Result<(), String> {
        unsafe {
            let Some(index) = self.pick(target) else {
                self.coverage.fills_with_nothing_live += 1;
                return Ok(());
            };
            let live = &self.live[index];
            let flags = live.allocation.memory_properties();
            if !flags.contains(vk::MemoryPropertyFlags::HOST_VISIBLE) {
                return Ok(());
            }
            self.fills = self.fills.wrapping_add(1);
            let byte = self.fills;
            let len = live.allocation.size() as usize;
            if live.contents.is_some() {
                self.coverage.refills += 1;
            }
            if persistent {
                let pointer = self
                    .allocator
                    .map_persistent(&live.allocation)
                    .map_err(|e| format!("map_persistent: {e}"))?;
                if let Some(earlier) = live.persistent {
                    ensure!(
                        earlier == pointer,
                        "map_persistent moved the mapping from {earlier:?} to {pointer:?}"
                    );
                }
                std::ptr::write_bytes(pointer.as_ptr(), byte, len);
                self.allocator
                    .flush_allocation(&live.allocation)
                    .map_err(|e| format!("flush_allocation: {e}"))?;
                self.live[index].persistent = Some(pointer);
                self.coverage.persistent_fills += 1;
            } else {
                let mut mapped = self
                    .allocator
                    .map(&live.allocation)
                    .map_err(|e| format!("map: {e}"))?;
                ensure!(
                    mapped.as_bytes().len() == len,
                    "the mapping has {} bytes instead of {len}",
                    mapped.as_bytes().len()
                );
                std::ptr::write_bytes(mapped.as_ptr().cast::<u8>(), byte, len);
                mapped.flush().map_err(|e| format!("flush: {e}"))?;
                drop(mapped);
                self.coverage.guard_fills += 1;
            }
            self.live[index].contents = Some(byte);
            Ok(())
        }
    }

    unsafe fn free(&mut self, target: Index) -> Result<(), String> {
        unsafe {
            let Some(index) = self.pick(target) else {
                self.coverage.frees_with_nothing_live += 1;
                return Ok(());
            };
            let live = self.live.remove(index);
            if !self
                .live
                .iter()
                .any(|other| other.allocation.memory() == live.allocation.memory())
            {
                self.coverage.blocks_emptied_by_free += 1;
            }
            self.destroy(live.resource);
            self.allocator
                .free(live.allocation)
                .map_err(|e| format!("free: {e:?}"))
        }
    }

    unsafe fn free_all(&mut self) -> Result<(), String> {
        unsafe {
            let expected = self.live.len();
            if expected > 0 {
                self.coverage.free_alls_with_live_allocations += 1;
            }
            for live in std::mem::take(&mut self.live) {
                self.destroy(live.resource);
            }
            let freed = self.allocator.free_all();
            ensure!(
                freed == expected,
                "free_all reported {freed} live allocations instead of {expected}"
            );
            Ok(())
        }
    }

    unsafe fn check(&mut self) -> Result<(), String> {
        unsafe {
            self.check_sharing()?;
            self.check_stats()?;
            self.check_contents()?;
            self.check_messages()
        }
    }

    fn check_sharing(&self) -> Result<(), String> {
        for (i, a) in self.live.iter().enumerate() {
            if a.dedicated {
                ensure!(
                    a.allocation.offset() == 0 && a.allocation.size() == a.requirements.size,
                    "dedicated {:?} is not the whole of a memory of {} bytes",
                    a.allocation,
                    a.requirements.size
                );
            }
            for b in &self.live[i + 1..] {
                if a.allocation.memory() != b.allocation.memory() {
                    continue;
                }
                let (first, second) = if a.allocation.offset() <= b.allocation.offset() {
                    (&a.allocation, &b.allocation)
                } else {
                    (&b.allocation, &a.allocation)
                };
                ensure!(
                    first.offset() + first.size() <= second.offset(),
                    "{first:?} overlaps {second:?}"
                );
                ensure!(
                    !a.dedicated && !b.dedicated,
                    "a dedicated allocation shares memory: {:?} and {:?}",
                    a.allocation,
                    b.allocation
                );
                ensure!(
                    a.linear == b.linear,
                    "a linear and a non-linear resource share memory: {:?} and {:?}",
                    a.allocation,
                    b.allocation
                );
                ensure!(
                    a.device_address == b.device_address,
                    "allocations with and without DEVICE_ADDRESS share memory: {:?} and {:?}",
                    a.allocation,
                    b.allocation
                );
            }
        }
        Ok(())
    }

    fn check_stats(&self) -> Result<(), String> {
        let stats = self.allocator.stats();
        let allocated: u64 = self.live.iter().map(|live| live.allocation.size()).sum();
        let mut memories: Vec<_> = self
            .live
            .iter()
            .map(|live| live.allocation.memory())
            .collect();
        memories.sort_by_key(|memory| memory.as_raw());
        memories.dedup();
        ensure!(
            stats.allocation_count == self.live.len(),
            "stats count {} allocations, {} are live",
            stats.allocation_count,
            self.live.len()
        );
        ensure!(
            stats.allocated_bytes == allocated,
            "stats count {} allocated bytes, the live allocations have {allocated}",
            stats.allocated_bytes
        );
        ensure!(
            stats.allocated_bytes + stats.free_bytes == stats.total_block_bytes,
            "allocated {} + free {} != total {}",
            stats.allocated_bytes,
            stats.free_bytes,
            stats.total_block_bytes
        );
        let snapshot = self.allocator.snapshot();
        let holding = snapshot
            .memory_types
            .iter()
            .flat_map(|memory_type| &memory_type.blocks)
            .filter(|block| block.allocated_region_count > 0)
            .count();
        ensure!(
            holding == memories.len(),
            "{holding} blocks hold allocations, and the live allocations are in {} memory objects",
            memories.len()
        );
        check_empty_blocks(&snapshot)
    }

    unsafe fn check_contents(&mut self) -> Result<(), String> {
        unsafe {
            for live in &self.live {
                let Some(byte) = live.contents else {
                    continue;
                };
                let mut mapped = self
                    .allocator
                    .map(&live.allocation)
                    .map_err(|e| format!("map: {e}"))?;
                mapped
                    .invalidate()
                    .map_err(|e| format!("invalidate: {e}"))?;
                ensure!(
                    holds_only(mapped.as_bytes(), byte),
                    "{:?} no longer holds only the byte {byte}",
                    live.allocation
                );
                if let Some(pointer) = live.persistent {
                    ensure!(
                        pointer.as_ptr() == mapped.as_ptr().cast(),
                        "the persistent mapping {pointer:?} and the guard {:?} differ",
                        mapped.as_ptr()
                    );
                }
                drop(mapped);
                self.coverage.reads += 1;
            }
            Ok(())
        }
    }

    fn check_messages(&self) -> Result<(), String> {
        let messages = self.gpu.take_validation_messages();
        ensure!(
            messages.is_empty(),
            "validation messages:\n{}",
            messages.join("\n")
        );
        Ok(())
    }
}

fn check_empty_blocks(snapshot: &AllocatorSnapshot) -> Result<(), String> {
    for memory_type in &snapshot.memory_types {
        let mut flags_of_empty_blocks = Vec::new();
        for block in &memory_type.blocks {
            if block.allocated_region_count > 0 {
                continue;
            }
            ensure!(
                !block.dedicated,
                "{:?} of memory type {} is kept empty, although it is the memory of a dedicated \
                 allocation",
                block.memory,
                memory_type.memory_type_index
            );
            ensure!(
                !flags_of_empty_blocks.contains(&block.memory_allocate_flags),
                "memory type {} holds more than one empty block with the flags {:?}",
                memory_type.memory_type_index,
                block.memory_allocate_flags
            );
            flags_of_empty_blocks.push(block.memory_allocate_flags);
        }
    }
    Ok(())
}

#[test]
fn random_sequences_of_allocations_bind_map_and_free_without_validation_messages() {
    let Some(gpu) = Gpu::new() else { return };
    gpu.assert_no_validation_messages();
    let mut runner = TestRunner::new(cases(1000));
    let result = runner.run(&sequences(), |(operations, ending)| {
        Sequence::run(&gpu, &operations, ending)
            .map(drop)
            .map_err(TestCaseError::fail)
    });
    if let Err(error) = result {
        panic!("{error}");
    }
    gpu.assert_no_validation_messages();
}

#[test]
fn the_generated_sequences_reach_every_kind_of_placement_and_mapping() {
    let Some(gpu) = Gpu::new() else { return };
    gpu.assert_no_validation_messages();
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = sequences();
    let mut coverage = Coverage::default();
    for _ in 0..24 {
        let (operations, ending) = strategy.new_tree(&mut runner).unwrap().current();
        coverage.merge(&Sequence::run(&gpu, &operations, ending).unwrap());
    }
    println!("{coverage:#?}");
    for (name, count) in [
        ("buffers", coverage.buffers),
        ("images", coverage.images),
        ("linear images", coverage.linear_images),
        ("dedicated allocations", coverage.dedicated),
        ("allocations with DEVICE_ADDRESS", coverage.device_address),
        (
            "allocations larger than a block",
            coverage.larger_than_a_block,
        ),
        (
            "allocations into a block with another allocation",
            coverage.into_a_block_with_another_allocation,
        ),
        ("blocks emptied by free", coverage.blocks_emptied_by_free),
        (
            "allocations into an empty block",
            coverage.into_an_empty_block,
        ),
        ("drops that free memory", coverage.drops_that_free_memory),
        ("fills through the guard", coverage.guard_fills),
        (
            "fills through the persistent mapping",
            coverage.persistent_fills,
        ),
        ("fills of a filled allocation", coverage.refills),
        ("reads", coverage.reads),
        ("fills with nothing live", coverage.fills_with_nothing_live),
        ("frees with nothing live", coverage.frees_with_nothing_live),
        (
            "free_all with live allocations",
            coverage.free_alls_with_live_allocations,
        ),
    ] {
        assert!(count > 0, "never reached: {name}");
    }
    gpu.assert_no_validation_messages();
}
