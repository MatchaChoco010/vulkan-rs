use anyhow::Result;
use vulkan::vk;
use vulkan_example_common::device::Gpu;

/// The present mode for the swapchain, following `VULKAN_EXAMPLE_PRESENT_MODE`.
///
/// `IMMEDIATE` presents without waiting for vertical blank and may tear; `FIFO_RELAXED`
/// tears only when a frame is late. `FIFO` is the only mode every surface supports.
pub unsafe fn choose_present_mode(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
) -> Result<vk::PresentModeKHR> {
    let supported = unsafe {
        gpu.instance
            .get_physical_device_surface_present_modes_khr(gpu.physical_device, surface)
    }?;
    let wanted = match std::env::var("VULKAN_EXAMPLE_PRESENT_MODE")
        .unwrap_or_default()
        .as_str()
    {
        "immediate" => vk::PresentModeKHR::IMMEDIATE,
        "fifo_relaxed" => vk::PresentModeKHR::FIFO_RELAXED,
        "mailbox" => vk::PresentModeKHR::MAILBOX,
        _ => vk::PresentModeKHR::FIFO,
    };
    if supported.contains(&wanted) {
        Ok(wanted)
    } else {
        println!("present mode {wanted:?} is not supported by the surface; using FIFO");
        Ok(vk::PresentModeKHR::FIFO)
    }
}
