use anyhow::Result;
use std::ffi::CStr;
use vulkan::{Instance, vk};

use super::WaylandWindow;

/// The instance extensions that [`create_wayland_surface`] needs.
pub const WAYLAND_SURFACE_EXTENSIONS: [&CStr; 2] =
    [vk::KHR_SURFACE_NAME, vk::KHR_WAYLAND_SURFACE_NAME];

/// The surface must be destroyed before the window.
pub unsafe fn create_wayland_surface(
    instance: &Instance,
    window: WaylandWindow,
) -> Result<vk::SurfaceKHR> {
    let info = vk::WaylandSurfaceCreateInfoKHR::default()
        .display(window.display)
        .surface(window.surface);
    Ok(unsafe { instance.create_wayland_surface_khr(&info, None) }?)
}

/// The first queue family that supports graphics and can present to `surface`.
pub unsafe fn find_present_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<Option<u32>> {
    unsafe {
        let families = instance.get_physical_device_queue_family_properties(physical_device);
        for (index, family) in families.iter().enumerate() {
            let index = index as u32;
            let can_present = instance.get_physical_device_surface_support_khr(
                physical_device,
                index,
                surface,
            )?;
            if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) && can_present {
                return Ok(Some(index));
            }
        }
        Ok(None)
    }
}
