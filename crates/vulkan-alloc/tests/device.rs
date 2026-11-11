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

#[test]
fn allocator_is_send() {
    fn assert_send<T: Send>() {}
    assert_send::<Allocator>();
}
