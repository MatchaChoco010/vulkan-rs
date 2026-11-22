use anyhow::{Result, ensure};
use vulkan::{Device, vk};

use crate::device::Gpu;
use crate::image::create_image_view;

/// A swapchain with a view and a semaphore for each of its images.
pub struct Swapchain {
    pub handle: vk::SwapchainKHR,
    pub format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    /// Signaled when the rendering to the image is done, waited on by its present.
    /// There is one per image rather than per frame: the present gives no signal when
    /// it is done with its semaphore, but once the image is acquired again, it is.
    pub render_finished: Vec<vk::Semaphore>,
}

impl Swapchain {
    /// A swapchain of color attachments for `surface`, in `format` and `present_mode`,
    /// both of which the surface has to support. `window_extent` is the size of the
    /// window, which the swapchain takes when the surface leaves the size to it, as
    /// Wayland surfaces do.
    pub unsafe fn new(
        gpu: &Gpu,
        surface: vk::SurfaceKHR,
        format: vk::SurfaceFormatKHR,
        present_mode: vk::PresentModeKHR,
        window_extent: vk::Extent2D,
    ) -> Result<Self> {
        unsafe {
            let caps = gpu
                .instance
                .get_physical_device_surface_capabilities_khr(gpu.physical_device, surface)?;
            let extent = choose_extent(&caps, window_extent)?;
            let info = vk::SwapchainCreateInfoKHR::default()
                .surface(surface)
                .min_image_count(image_count(&caps))
                .image_format(format.format)
                .image_color_space(format.color_space)
                .image_extent(extent)
                .image_array_layers(1)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(caps.current_transform)
                .composite_alpha(choose_composite_alpha(&caps))
                .present_mode(present_mode)
                .clipped(true)
                .old_swapchain(vk::SwapchainKHR::null());
            let handle = gpu.device.create_swapchain_khr(&info, None)?;

            let mut swapchain = Self {
                handle,
                format,
                extent,
                images: Vec::new(),
                image_views: Vec::new(),
                render_finished: Vec::new(),
            };
            if let Err(error) = swapchain.create_per_image(&gpu.device) {
                swapchain.destroy(&gpu.device);
                return Err(error);
            }
            Ok(swapchain)
        }
    }

    unsafe fn create_per_image(&mut self, device: &Device) -> Result<()> {
        unsafe {
            self.images = device.get_swapchain_images_khr(self.handle)?;
            for &image in &self.images {
                self.image_views.push(create_image_view(
                    device,
                    image,
                    self.format.format,
                    vk::ImageAspectFlags::COLOR,
                )?);
            }
            for _ in &self.images {
                self.render_finished
                    .push(device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?);
            }
            Ok(())
        }
    }

    /// Destroys the views, the semaphores and the swapchain, and leaves `self` empty,
    /// so that destroying it again does nothing. An example can then destroy its
    /// swapchain on exit whether or not the last recreation of it succeeded.
    ///
    /// The device must be done with all of them.
    pub unsafe fn destroy(&mut self, device: &Device) {
        unsafe {
            for view in self.image_views.drain(..) {
                device.destroy_image_view(view, None);
            }
            for semaphore in self.render_finished.drain(..) {
                device.destroy_semaphore(semaphore, None);
            }
            if !self.handle.is_null() {
                device.destroy_swapchain_khr(self.handle, None);
                self.handle = vk::SwapchainKHR::null();
            }
            self.images.clear();
        }
    }
}

/// One more image than the minimum, so that the next frame can be drawn while the
/// presentation engine holds the minimum, within the maximum.
fn image_count(caps: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let count = caps.min_image_count.saturating_add(1);
    // A maximum of 0 means that there is none.
    if caps.max_image_count != 0 {
        count.min(caps.max_image_count)
    } else {
        count
    }
}

/// The surface's current extent, or, when the surface has none, the window's limited to
/// what the surface supports. An error for a zero extent, which a swapchain cannot have:
/// a surface reports one while its window is minimized, and these examples, made for
/// Wayland surfaces that leave the extent to the swapchain, stop rather than wait.
fn choose_extent(
    caps: &vk::SurfaceCapabilitiesKHR,
    window_extent: vk::Extent2D,
) -> Result<vk::Extent2D> {
    // `u32::MAX` says that the size of the swapchain decides the size of the surface.
    let extent = if caps.current_extent.width != u32::MAX {
        caps.current_extent
    } else {
        // Not `clamp`, which panics when a surface reports a minimum above the maximum.
        let limit = |wanted: u32, min: u32, max: u32| wanted.max(min).min(max);
        vk::Extent2D {
            width: limit(
                window_extent.width,
                caps.min_image_extent.width,
                caps.max_image_extent.width,
            ),
            height: limit(
                window_extent.height,
                caps.min_image_extent.height,
                caps.max_image_extent.height,
            ),
        }
    };
    ensure!(
        extent.width > 0 && extent.height > 0,
        "the surface has a zero extent ({}x{}); is the window minimized?",
        extent.width,
        extent.height
    );
    Ok(extent)
}

/// Opaque when the compositor allows it, since the examples do not draw transparency.
fn choose_composite_alpha(caps: &vk::SurfaceCapabilitiesKHR) -> vk::CompositeAlphaFlagBitsKHR {
    let supported = caps.supported_composite_alpha;
    if supported.contains(vk::CompositeAlphaFlagsKHR::OPAQUE) {
        vk::CompositeAlphaFlagBitsKHR::OPAQUE
    } else if supported.contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED) {
        vk::CompositeAlphaFlagBitsKHR::PRE_MULTIPLIED
    } else if supported.contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED) {
        vk::CompositeAlphaFlagBitsKHR::POST_MULTIPLIED
    } else {
        vk::CompositeAlphaFlagBitsKHR::INHERIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pseudo-random values that favour the edges: 0, 1, `u32::MAX` and small numbers.
    fn values() -> impl FnMut() -> u32 {
        let mut state = 0x853c_49e6_748f_ea9b_u64;
        move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            match state % 6 {
                0 => 0,
                1 => 1,
                2 => u32::MAX,
                3 => (state >> 8) as u32 % 8,
                _ => (state >> 8) as u32 % 5000,
            }
        }
    }

    fn show(caps: &vk::SurfaceCapabilitiesKHR) -> String {
        let extent = |e: vk::Extent2D| format!("{}x{}", e.width, e.height);
        format!(
            "images {}..{}, current {}, min {}, max {}",
            caps.min_image_count,
            caps.max_image_count,
            extent(caps.current_extent),
            extent(caps.min_image_extent),
            extent(caps.max_image_extent)
        )
    }

    fn capabilities(next: &mut impl FnMut() -> u32) -> vk::SurfaceCapabilitiesKHR {
        let mut extent = || vk::Extent2D {
            width: next(),
            height: next(),
        };
        let (current_extent, min_image_extent, max_image_extent) = (extent(), extent(), extent());
        vk::SurfaceCapabilitiesKHR {
            min_image_count: next(),
            max_image_count: next(),
            current_extent,
            min_image_extent,
            max_image_extent,
            ..Default::default()
        }
    }

    #[test]
    fn the_image_count_never_overflows_and_is_valid_for_valid_capabilities() {
        let mut next = values();
        for _ in 0..10_000 {
            let caps = capabilities(&mut next);
            let count = image_count(&caps);
            let valid = caps.max_image_count == 0 || caps.min_image_count <= caps.max_image_count;
            if valid {
                assert!(count >= caps.min_image_count, "{count} for {}", show(&caps));
                assert!(
                    caps.max_image_count == 0 || count <= caps.max_image_count,
                    "{count} for {}",
                    show(&caps)
                );
            }
        }
    }

    #[test]
    fn the_extent_never_panics_is_never_zero_and_follows_the_surface() {
        let mut next = values();
        for _ in 0..10_000 {
            let caps = capabilities(&mut next);
            let window = vk::Extent2D {
                width: next(),
                height: next(),
            };
            let Ok(extent) = choose_extent(&caps, window) else {
                continue;
            };
            let chosen = format!("{}x{} for {}", extent.width, extent.height, show(&caps));
            assert!(extent.width > 0 && extent.height > 0, "{chosen}");
            if caps.current_extent.width != u32::MAX {
                assert!(extent == caps.current_extent, "{chosen}");
                continue;
            }
            for (value, min, max) in [
                (
                    extent.width,
                    caps.min_image_extent.width,
                    caps.max_image_extent.width,
                ),
                (
                    extent.height,
                    caps.min_image_extent.height,
                    caps.max_image_extent.height,
                ),
            ] {
                if min <= max {
                    assert!(min <= value && value <= max, "{chosen}");
                }
            }
        }
    }

    #[test]
    fn a_zero_current_extent_is_an_error() {
        let caps = vk::SurfaceCapabilitiesKHR {
            current_extent: vk::Extent2D {
                width: 0,
                height: 0,
            },
            ..Default::default()
        };
        let window = vk::Extent2D {
            width: 800,
            height: 600,
        };
        assert!(choose_extent(&caps, window).is_err());
    }
}
