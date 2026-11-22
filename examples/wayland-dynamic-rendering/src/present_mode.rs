use anyhow::Result;
use vulkan::vk;
use vulkan_example_common::device::Gpu;
use vulkan_example_common::env::{one_of, parsed_var};

/// The values of `VULKAN_EXAMPLE_PRESENT_MODE`.
const PRESENT_MODES: [(&str, vk::PresentModeKHR); 4] = [
    ("fifo", vk::PresentModeKHR::FIFO),
    ("fifo_relaxed", vk::PresentModeKHR::FIFO_RELAXED),
    ("immediate", vk::PresentModeKHR::IMMEDIATE),
    ("mailbox", vk::PresentModeKHR::MAILBOX),
];

/// The present mode that `VULKAN_EXAMPLE_PRESENT_MODE` asks for, `FIFO` when it is not
/// set. An error for a value that is not one of [`PRESENT_MODES`].
pub fn requested_present_mode() -> Result<vk::PresentModeKHR> {
    let requested = parsed_var("VULKAN_EXAMPLE_PRESENT_MODE", |value| {
        one_of(value, &PRESENT_MODES)
    })?;
    Ok(requested.unwrap_or(vk::PresentModeKHR::FIFO))
}

/// The present mode for the swapchain: `wanted` if the surface supports it, else `FIFO`.
///
/// `IMMEDIATE` presents without waiting for vertical blank and may tear; `FIFO_RELAXED`
/// tears only when a frame is late. `FIFO` is the only mode every surface supports.
pub unsafe fn choose_present_mode(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
    wanted: vk::PresentModeKHR,
) -> Result<vk::PresentModeKHR> {
    let supported = unsafe {
        gpu.instance
            .get_physical_device_surface_present_modes_khr(gpu.physical_device, surface)
    }?;
    if supported.contains(&wanted) {
        Ok(wanted)
    } else {
        println!("present mode {wanted:?} is not supported by the surface; using FIFO");
        Ok(vk::PresentModeKHR::FIFO)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_present_mode_is_named_once_and_other_values_are_errors() {
        for (index, (name, mode)) in PRESENT_MODES.iter().enumerate() {
            assert_eq!(one_of(name, &PRESENT_MODES).unwrap(), *mode, "{name}");
            assert!(
                PRESENT_MODES[index + 1..]
                    .iter()
                    .all(|(other_name, other_mode)| other_name != name && other_mode != mode),
                "{name} is listed twice"
            );
        }
        for value in ["", "FIFO", "Fifo", "fifo ", "relaxed", "tearing", "vsync"] {
            assert!(
                one_of(value, &PRESENT_MODES).is_err(),
                "{value:?} was accepted"
            );
        }
    }
}
