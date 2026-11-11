//! What a chained struct points to must outlive the root as well.
use vulkan::vk;

fn main() {
    let mut present_modes = vk::SwapchainPresentModesCreateInfoKHR::default();
    let info;
    {
        let modes = [vk::PresentModeKHR::FIFO, vk::PresentModeKHR::IMMEDIATE];
        present_modes = present_modes.present_modes(&modes);
        info = vk::SwapchainCreateInfoKHR::default().push_next(&mut present_modes);
    }
    let _ = info;
}
