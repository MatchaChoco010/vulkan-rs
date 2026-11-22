use anyhow::Result;
use std::ffi::CStr;
use vulkan::{Device, Instance, vk};

use crate::extensions::has_extension;
use crate::instance::{DebugMessenger, destroy_instance};

/// An instance and a device with the one queue that the example submits to.
pub struct Gpu {
    pub instance: Instance,
    pub messenger: DebugMessenger,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub queue_family: u32,
    pub queue: vk::Queue,
}

impl Gpu {
    /// Takes queue 0 of `queue_family`, which the device must have been created with.
    pub unsafe fn new(
        instance: Instance,
        messenger: DebugMessenger,
        physical_device: vk::PhysicalDevice,
        device: Device,
        queue_family: u32,
    ) -> Self {
        let queue = unsafe { device.get_device_queue(queue_family, 0) };
        Self {
            instance,
            messenger,
            physical_device,
            device,
            queue_family,
            queue,
        }
    }

    /// Destroys the device and the instance. Everything created from them, the
    /// allocator included, must be gone already.
    pub unsafe fn destroy(self) {
        unsafe {
            self.device.destroy_device(None);
            destroy_instance(self.instance, self.messenger);
        }
    }
}

/// Asks `suitable` about every physical device and picks one of those it accepts:
/// a discrete GPU over an integrated one, that over a virtual one, that over the rest.
/// Among equals the first wins. `suitable` returns what it found out about the device,
/// such as the queue family to use, and `None` to reject it.
pub unsafe fn pick_physical_device<T>(
    instance: &Instance,
    mut suitable: impl FnMut(vk::PhysicalDevice, &vk::PhysicalDeviceProperties) -> Result<Option<T>>,
) -> Result<Option<(vk::PhysicalDevice, T)>> {
    unsafe {
        let mut best = None;
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);
            let Some(found) = suitable(physical_device, &properties)? else {
                continue;
            };
            let rank = match properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => 3,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
                vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
                _ => 0,
            };
            if best
                .as_ref()
                .is_none_or(|(best_rank, _, _)| rank > *best_rank)
            {
                best = Some((rank, physical_device, found));
            }
        }
        Ok(best.map(|(_, physical_device, found)| (physical_device, found)))
    }
}

/// Whether the physical device has every extension in `names`.
pub unsafe fn supports_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    names: &[&CStr],
) -> Result<bool> {
    let available =
        unsafe { instance.enumerate_device_extension_properties(physical_device, None) }?;
    Ok(names.iter().all(|name| has_extension(&available, name)))
}

/// The first queue family that supports all of `flags`.
pub unsafe fn find_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    flags: vk::QueueFlags,
) -> Option<u32> {
    unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
        .iter()
        .position(|family| family.queue_flags.contains(flags))
        .map(|index| index as u32)
}
