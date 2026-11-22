use anyhow::Result;
use std::ffi::CStr;
use vulkan::{Device, Instance, vk};

use crate::extensions::has_extension;
use crate::instance::{DebugMessenger, destroy_instance};

/// Whether `info` enables `shaderDrawParameters`, which a shader with the SPIR-V
/// capability `DrawParameters` needs, through
/// `VkPhysicalDeviceShaderDrawParametersFeatures` or `VkPhysicalDeviceVulkan11Features`
/// in its `pNext` chain.
///
/// # Safety
///
/// The `pNext` chain of `info` must be valid.
pub unsafe fn enables_shader_draw_parameters(info: &vk::DeviceCreateInfo<'_>) -> bool {
    let mut next = info.p_next.cast::<vk::BaseInStructure<'_>>();
    // SAFETY: every structure in a valid chain starts with `sType` and `pNext`, and
    // `sType` tells which structure it is.
    while let Some(base) = unsafe { next.as_ref() } {
        let enabled = match base.s_type {
            vk::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceShaderDrawParametersFeatures<'_>>())
                    .shader_draw_parameters
            },
            vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceVulkan11Features<'_>>()).shader_draw_parameters
            },
            _ => vk::FALSE,
        };
        if enabled != vk::FALSE {
            return true;
        }
        next = base.p_next;
    }
    false
}

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

#[cfg(test)]
mod tests {
    use super::*;

    fn draw_parameters(enabled: bool) -> vk::PhysicalDeviceShaderDrawParametersFeatures<'static> {
        vk::PhysicalDeviceShaderDrawParametersFeatures::default().shader_draw_parameters(enabled)
    }

    fn vulkan11(enabled: bool) -> vk::PhysicalDeviceVulkan11Features<'static> {
        vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(enabled)
    }

    #[test]
    fn finds_shader_draw_parameters_anywhere_in_the_chain() {
        assert!(!unsafe { enables_shader_draw_parameters(&vk::DeviceCreateInfo::default()) });
        for first in [false, true] {
            for second in [false, true] {
                for behind_other_features in [false, true] {
                    let mut other = vk::PhysicalDeviceVulkan12Features::default();
                    let mut first_features = draw_parameters(first);
                    let mut second_features = vulkan11(second);
                    let mut info = vk::DeviceCreateInfo::default();
                    if behind_other_features {
                        info = info.push_next(&mut other);
                    }
                    let info = info
                        .push_next(&mut first_features)
                        .push_next(&mut second_features);
                    assert_eq!(
                        unsafe { enables_shader_draw_parameters(&info) },
                        first || second,
                        "draw parameters features: {first}, Vulkan 1.1 features: {second}, \
                         behind other features: {behind_other_features}"
                    );
                }
            }
        }
    }
}
