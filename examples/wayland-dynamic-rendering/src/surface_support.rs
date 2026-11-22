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
                let count = compatibility.present_mode_count;
                let written = filled(&compatible, count);
                println!("  {mode:?} can switch to {written:?}");
            }
        }
        Ok(())
    }
}

/// The part of `values` that a call reported as `count` filled in. A count larger than
/// the array it was given, which the call must not report, gives all of it.
fn filled<T>(values: &[T], count: u32) -> &[T] {
    let count = usize::try_from(count).unwrap_or(usize::MAX);
    &values[..count.min(values.len())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_filled_part_is_a_prefix_for_any_count() {
        for len in 0..5 {
            let values = (0..len).collect::<Vec<u32>>();
            for count in [0, 1, 2, 4, 5, 100, u32::MAX] {
                let part = filled(&values, count);
                assert_eq!(part.len(), (count as usize).min(len as usize));
                assert!(values.starts_with(part));
            }
        }
    }
}
