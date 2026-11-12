//! Allocator tests against a real Vulkan implementation (lavapipe in CI).
//!
//! Skipped without a Vulkan 1.2 device, unless `VULKAN_RS_REQUIRE_DEVICE_TEST` is set.

use std::ffi::CStr;
use std::os::fd::{FromRawFd, OwnedFd};
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc, MemoryLocation,
};

struct Gpu {
    instance: Instance,
    physical_device: vk::PhysicalDevice,
    device: Device,
    external_memory_fd: bool,
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
            let app = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_2);
            let instance = entry
                .create_instance(
                    &vk::InstanceCreateInfo::default().application_info(&app),
                    None,
                )
                .map_err(|e| format!("vkCreateInstance: {e}"))?;

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
                instance.destroy_instance(None);
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
            Ok(Self {
                instance,
                physical_device,
                device,
                external_memory_fd,
            })
        }
    }

    fn allocator(&self) -> Allocator {
        unsafe {
            Allocator::new(AllocatorCreateDesc::new(
                &self.device,
                self.instance
                    .get_physical_device_memory_properties(self.physical_device),
                self.instance
                    .get_physical_device_properties(self.physical_device),
            ))
        }
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

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
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
        // Same kind of memory: both come from one VkDeviceMemory.
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
            mapped.byte_range_mut(0, 4).copy_from_slice(&[1, 2, 3, 4]);
        }
        {
            let mapped = allocator.map(&a).unwrap();
            assert_eq!(mapped.byte_range(0, 4), &[1, 2, 3, 4]);
        }

        gpu.device.destroy_buffer(first, None);
        gpu.device.destroy_buffer(second, None);
        allocator.free(a).unwrap();
        allocator.free(b).unwrap();
    }
    assert!(allocator.is_empty());
}

/// Allocate flags are fixed when a `VkDeviceMemory` is created, so a buffer that needs
/// `DEVICE_ADDRESS` must not be placed in a block that was created without it.
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
        // Only valid because the memory was allocated with DEVICE_ADDRESS.
        let address = gpu.device.get_buffer_device_address(
            &vk::BufferDeviceAddressInfo::default().buffer(address_buffer),
        );
        assert_ne!(address, 0);

        gpu.device.destroy_buffer(plain_buffer, None);
        gpu.device.destroy_buffer(address_buffer, None);
        allocator.free(plain).unwrap();
        allocator.free(with_address).unwrap();
    }
}

/// Exportable memory gets a `VkDeviceMemory` of its own, and the allocator's export
/// request reaches Vulkan: the memory can be exported as a file descriptor.
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
        // The exported fd is a new reference that the caller owns.
        drop(OwnedFd::from_raw_fd(fd));

        gpu.device.destroy_buffer(first, None);
        gpu.device.destroy_buffer(second, None);
        allocator.free(a).unwrap();
        allocator.free(b).unwrap();
    }
}

/// Collects `log` records so that the leak report can be asserted on.
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

/// A host that runs scripts cannot guarantee that every allocation is freed. Dropping
/// the allocator must report what is left and carry on, not bring the process down.
#[test]
fn dropping_with_live_allocations_reports_and_continues() {
    let Some(gpu) = Gpu::new() else { return };
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
        // `leaked` is forgotten on purpose: it is never passed to `Allocator::free`.
    }

    let live = allocator.live_allocations();
    assert_eq!(live.len(), 1);
    assert_eq!(live[0].name.as_deref(), Some("script texture"));
    assert_eq!(live[0].memory, leaked_memory);

    drop(allocator); // Must not panic.

    let records = CAPTURED.lock().unwrap().join("\n");
    assert!(
        records.contains("ERROR vulkan_alloc::Allocator dropped with 1 live allocation(s)"),
        "{records}"
    );
    assert!(records.contains("\"script texture\""), "{records}");

    // The memory was left alone, so the buffer is still valid until we clean up.
    unsafe {
        gpu.device.destroy_buffer(buffer, None);
        gpu.device.free_memory(leaked_memory, None);
    }
}

/// The explicit shutdown path: release everything, live or not.
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
            // Out-of-range requests are an error value, not a panic.
            assert!(mapped.try_byte_range(0, 4096).is_some());
            assert!(mapped.try_byte_range(4096, usize::MAX).is_none());
            assert!(mapped.try_byte_range(1 << 40, 1).is_none());
        }

        // The resource goes first, then the memory.
        gpu.device.destroy_buffer(buffer, None);
        assert_eq!(allocator.free_all(), 1);
    }
    assert!(allocator.is_empty());
    assert!(allocator.live_allocations().is_empty());
    // Nothing is left to report on drop, and the device has no memory to complain about.
}

/// A buffer that stays mapped for its whole life: the pointer borrows nothing, and
/// freeing the allocation ends the mapping.
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
        // The allocator is free for other work while the mapping lives.
        let other = allocator.map_persistent(&b).unwrap();
        assert_ne!(pointer, other);
        assert_eq!(allocator.map_persistent(&a).unwrap(), pointer);
        assert_eq!(allocator.persistent_ptr(&a), Some(pointer));

        let bytes = std::slice::from_raw_parts_mut(pointer.as_ptr(), a.size() as usize);
        bytes[..4].copy_from_slice(&[9, 8, 7, 6]);
        {
            // The guard form sees the same memory and can be used alongside.
            let mapped = allocator.map(&a).unwrap();
            assert_eq!(mapped.byte_range(0, 4), &[9, 8, 7, 6]);
        }
        assert_eq!(allocator.persistent_ptr(&a), Some(pointer));

        // Freeing a persistently mapped allocation is not an error: it ends the mapping.
        gpu.device.destroy_buffer(first, None);
        allocator.free(a).unwrap();
        // `b` shares the block and is still mapped at the same address.
        assert_eq!(allocator.persistent_ptr(&b), Some(other));
        std::slice::from_raw_parts_mut(other.as_ptr(), 4).copy_from_slice(&[1, 2, 3, 4]);

        gpu.device.destroy_buffer(second, None);
        allocator.free(b).unwrap();
    }
    assert!(allocator.is_empty());
}

/// Host-only memory cannot be mapped persistently either: an error, not a panic.
#[test]
fn persistent_mapping_of_device_local_memory_is_an_error() {
    let Some(gpu) = Gpu::new() else { return };
    let mut allocator = gpu.allocator();
    unsafe {
        let buffer = gpu.buffer(vk::BufferUsageFlags::TRANSFER_SRC);
        let allocation = allocator
            .allocate(desc(
                gpu.device.get_buffer_memory_requirements(buffer),
                MemoryLocation::GpuOnly,
            ))
            .unwrap();
        let host_visible = allocator
            .allocation_memory_flags(&allocation)
            .unwrap()
            .contains(vk::MemoryPropertyFlags::HOST_VISIBLE);
        // Integrated and software devices have no memory that is not host-visible.
        if !host_visible {
            assert!(allocator.map_persistent(&allocation).is_err());
            assert!(allocator.persistent_ptr(&allocation).is_none());
        }
        gpu.device.destroy_buffer(buffer, None);
        allocator.free(allocation).unwrap();
    }
}

#[test]
fn allocator_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Allocator>();
}
