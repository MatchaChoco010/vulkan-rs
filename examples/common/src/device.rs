use anyhow::Result;
use std::ffi::CStr;
use vulkan::{Device, Instance, vk};

use crate::extensions::has_extension;
use crate::instance::{DebugMessenger, destroy_instance};

/// Whether a device created with `info` may run shaders that declare the SPIR-V
/// capability `DrawParameters`. The registry lists three ways to allow it: the feature
/// `shaderDrawParameters` in `VkPhysicalDeviceShaderDrawParametersFeatures` or in
/// `VkPhysicalDeviceVulkan11Features` in the `pNext` chain, or the device extension
/// `VK_KHR_shader_draw_parameters`.
///
/// # Safety
///
/// The `pNext` chain and the extension names of `info` must be valid.
pub unsafe fn enables_draw_parameters(info: &vk::DeviceCreateInfo<'_>) -> bool {
    unsafe { enables_draw_parameters_extension(info) || enables_shader_draw_parameters(info) }
}

unsafe fn enables_draw_parameters_extension(info: &vk::DeviceCreateInfo<'_>) -> bool {
    (0..info.enabled_extension_count as usize).any(|index| {
        // SAFETY: the caller guarantees an array of `enabledExtensionCount` valid
        // null-terminated strings.
        let name = unsafe { CStr::from_ptr(*info.pp_enabled_extension_names.add(index)) };
        name == vk::KHR_SHADER_DRAW_PARAMETERS_NAME
    })
}

unsafe fn enables_shader_draw_parameters(info: &vk::DeviceCreateInfo<'_>) -> bool {
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
    use std::ffi::c_void;
    use std::ptr;

    const ORDERS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    /// Whether a create info allows `DrawParameters` when its `pNext` chain holds the
    /// draw parameters features (index 0), the Vulkan 1.1 features (1) and the unrelated
    /// Vulkan 1.2 features (2) in `order`, and it enables `extensions`.
    fn allows(
        draw_parameters: bool,
        vulkan11: bool,
        order: [usize; 3],
        extensions: &[&CStr],
    ) -> bool {
        let mut draw_parameters_features =
            vk::PhysicalDeviceShaderDrawParametersFeatures::default()
                .shader_draw_parameters(draw_parameters);
        let mut vulkan11_features =
            vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(vulkan11);
        let mut vulkan12_features = vk::PhysicalDeviceVulkan12Features::default();
        let structures: [*mut vk::BaseOutStructure<'_>; 3] = [
            ptr::from_mut(&mut draw_parameters_features).cast(),
            ptr::from_mut(&mut vulkan11_features).cast(),
            ptr::from_mut(&mut vulkan12_features).cast(),
        ];
        for pair in order.windows(2) {
            unsafe { (*structures[pair[0]]).p_next = structures[pair[1]] };
        }
        unsafe { (*structures[order[2]]).p_next = ptr::null_mut() };
        let names = extensions
            .iter()
            .map(|name| vk::CStrPtr::new(name))
            .collect::<Vec<_>>();
        let mut info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
        info.p_next = structures[order[0]].cast::<c_void>().cast_const();
        unsafe { enables_draw_parameters(&info) }
    }

    #[test]
    fn an_empty_create_info_does_not_allow_draw_parameters() {
        assert!(!unsafe { enables_draw_parameters(&vk::DeviceCreateInfo::default()) });
    }

    #[test]
    fn finds_every_enabler_of_draw_parameters_at_any_position() {
        let without = [vk::KHR_SWAPCHAIN_NAME];
        let with = [vk::KHR_SWAPCHAIN_NAME, vk::KHR_SHADER_DRAW_PARAMETERS_NAME];
        for order in ORDERS {
            for draw_parameters in [false, true] {
                for vulkan11 in [false, true] {
                    for extension in [false, true] {
                        let extensions: &[&CStr] = if extension { &with } else { &without };
                        assert_eq!(
                            allows(draw_parameters, vulkan11, order, extensions),
                            draw_parameters || vulkan11 || extension,
                            "order {order:?}, draw parameters features: {draw_parameters}, \
                             Vulkan 1.1 features: {vulkan11}, extension: {extension}"
                        );
                    }
                }
            }
        }
    }
}
