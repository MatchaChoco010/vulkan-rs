use anyhow::Result;
use vulkan::vk;
use vulkan_example_common::device::Gpu;

/// Prints what the surface offers: formats with color spaces, present modes, and, with
/// `VK_EXT_surface_maintenance1`, which present modes a swapchain can switch between
/// without being recreated.
pub unsafe fn print_surface_support(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
    surface_maintenance1: bool,
) -> Result<()> {
    unsafe {
        let instance = &gpu.instance;
        let physical_device = gpu.physical_device;
        println!("surface formats:");
        for format in instance.get_physical_device_surface_formats_khr(physical_device, surface)? {
            println!("  {:?} / {:?}", format.format, format.color_space);
        }
        let present_modes =
            instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?;
        println!("present modes: {present_modes:?}");

        if surface_maintenance1 {
            for &mode in &present_modes {
                // The two-call idiom inside a pNext struct: first the count, then the array.
                let mut query = vk::SurfacePresentModeEXT::default().present_mode(mode);
                let surface_info = vk::PhysicalDeviceSurfaceInfo2KHR::default()
                    .surface(surface)
                    .push_next(&mut query);
                let mut compatibility = vk::SurfacePresentModeCompatibilityEXT::default();
                let mut caps = vk::SurfaceCapabilities2KHR::default().push_next(&mut compatibility);
                instance.get_physical_device_surface_capabilities2_khr(
                    physical_device,
                    &surface_info,
                    &mut caps,
                )?;
                let mut compatible =
                    vec![vk::PresentModeKHR::default(); compatibility.present_mode_count as usize];
                let mut compatibility = vk::SurfacePresentModeCompatibilityEXT::default()
                    .present_modes(&mut compatible);
                let mut caps = vk::SurfaceCapabilities2KHR::default().push_next(&mut compatibility);
                instance.get_physical_device_surface_capabilities2_khr(
                    physical_device,
                    &surface_info,
                    &mut caps,
                )?;
                let written = compatibility.present_mode_count as usize;
                println!("  {mode:?} can switch to {:?}", &compatible[..written]);
            }
        }
        Ok(())
    }
}
