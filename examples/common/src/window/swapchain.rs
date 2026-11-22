use anyhow::Result;
use vulkan::vk;

use crate::device::{Gpu, Owned};
use crate::image::create_image_view;

/// A swapchain with a view and a semaphore for each of its images. Dropping it destroys
/// the views and the semaphores, then the swapchain; the device must be done with all of
/// them. It must be dropped before the surface it was created for.
pub struct Swapchain {
    gpu: Gpu,
    pub handle: vk::SwapchainKHR,
    pub format: vk::SurfaceFormatKHR,
    pub extent: vk::Extent2D,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<Owned<vk::ImageView>>,
    /// Signaled when the rendering to the image is done, waited on by its present.
    /// There is one per image rather than per frame: the present gives no signal when
    /// it is done with its semaphore, but once the image is acquired again, it is.
    pub render_finished: Vec<Owned<vk::Semaphore>>,
}

impl Swapchain {
    /// A swapchain of color attachments for `surface`, in `format` and `present_mode`,
    /// both of which the surface has to support. `window_extent` is the size of the
    /// window, which the swapchain takes when the surface leaves the size to it, as
    /// Wayland surfaces do. The surface must have no other swapchain.
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
            let extent = choose_extent(&caps, window_extent);
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
            let mut swapchain = Self {
                gpu: gpu.clone(),
                handle: gpu.device.create_swapchain_khr(&info, None)?,
                format,
                extent,
                images: Vec::new(),
                image_views: Vec::new(),
                render_finished: Vec::new(),
            };
            swapchain.images = gpu.device.get_swapchain_images_khr(swapchain.handle)?;
            for &image in &swapchain.images {
                swapchain.image_views.push(create_image_view(
                    gpu,
                    image,
                    format.format,
                    vk::ImageAspectFlags::COLOR,
                )?);
                swapchain.render_finished.push(
                    gpu.own(
                        gpu.device
                            .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?,
                    ),
                );
            }
            Ok(swapchain)
        }
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.image_views.clear();
        self.render_finished.clear();
        // SAFETY: the swapchain was created from this device, and what was created from
        // its images is gone.
        unsafe { self.gpu.device.destroy_swapchain_khr(self.handle, None) };
    }
}

/// One more image than the minimum, so that the next frame can be drawn while the
/// presentation engine holds the minimum.
fn image_count(caps: &vk::SurfaceCapabilitiesKHR) -> u32 {
    let count = caps.min_image_count + 1;
    // A maximum of 0 means that there is none.
    if caps.max_image_count != 0 {
        count.min(caps.max_image_count)
    } else {
        count
    }
}

/// The surface's current extent, or, when the surface has none, the window's clamped
/// to what the surface supports.
fn choose_extent(caps: &vk::SurfaceCapabilitiesKHR, window_extent: vk::Extent2D) -> vk::Extent2D {
    // `u32::MAX` says that the size of the swapchain decides the size of the surface.
    if caps.current_extent.width != u32::MAX {
        caps.current_extent
    } else {
        vk::Extent2D {
            width: window_extent
                .width
                .clamp(caps.min_image_extent.width, caps.max_image_extent.width),
            height: window_extent
                .height
                .clamp(caps.min_image_extent.height, caps.max_image_extent.height),
        }
    }
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
