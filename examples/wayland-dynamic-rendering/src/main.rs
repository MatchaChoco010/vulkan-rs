//! A triangle on a Wayland surface with dynamic rendering.
//!
//! There is no render pass and no framebuffer: `vkCmdBeginRendering` takes the
//! swapchain image's view directly, and the frame moves the image into
//! `COLOR_ATTACHMENT_OPTIMAL` and back to `PRESENT_SRC_KHR` with barriers of its own.
//!
//! The swapchain parameters a compositor or an HDR-aware client cares about can be
//! chosen from the environment; any other value is an error. The surface's support for
//! each is printed at startup.
//!
//! - `VULKAN_EXAMPLE_PRESENT_MODE`: `fifo` (default), `fifo_relaxed`, `immediate`
//!   (tearing) or `mailbox`.
//! - `VULKAN_EXAMPLE_COLOR_SPACE`: `srgb` (default), `hdr10` (BT.2020 PQ, with
//!   `VK_EXT_hdr_metadata` when available) or `scrgb` (extended linear sRGB).
//! - `VULKAN_EXAMPLE_FRAMES`: exit after this many frames, at least 1.
//!
//! The shader always writes the same values, so the triangle is only colorimetrically
//! right in `srgb`. The point of the other modes is the swapchain setup.

mod color_space;
mod gpu;
mod pipeline;
mod present_mode;
mod surface_support;

use anyhow::{Context, Result};
use vulkan::{Device, vk};
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::image::subresource_range;
use vulkan_example_common::memory::{Buffer, upload_buffer};
use vulkan_example_common::window::frame::{Frame, SwapchainStatus};
use vulkan_example_common::window::surface::Surface;
use vulkan_example_common::window::swapchain::Swapchain;
use vulkan_example_common::window::{self, Renderer, WaylandWindow};

use color_space::{ColorSpace, choose_surface_format, set_hdr_metadata};
use pipeline::Vertex;

const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, -0.65],
        color: [1.0, 0.1, 0.1],
    },
    Vertex {
        position: [0.65, 0.65],
        color: [0.1, 1.0, 0.2],
    },
    Vertex {
        position: [-0.65, 0.65],
        color: [0.2, 0.3, 1.0],
    },
];

/// What the environment asks for, read before the window opens.
struct Settings {
    present_mode: vk::PresentModeKHR,
    color_space: ColorSpace,
}

fn main() -> Result<()> {
    let settings = Settings {
        present_mode: present_mode::requested_present_mode()?,
        color_space: color_space::requested_color_space()?,
    };
    // A failure to destroy something on the way out also fails the run.
    vulkan_example_common::device::finish(window::run::<App>(
        "vulkan wayland dynamic rendering",
        settings,
    ))
}

/// The fields drop in order: what depends on the swapchain, then the frame, the buffer
/// and the layout, then the surface, which goes after its swapchain and before the
/// window. The device and the instance go once nothing holds them any more.
struct App {
    /// `None` after a recreation of the swapchain failed.
    target: Option<Target>,
    frame: Frame,
    vertex_buffer: Owned<Buffer>,
    pipeline_layout: Owned<vk::PipelineLayout>,
    present_mode: vk::PresentModeKHR,
    /// `VK_EXT_hdr_metadata` is enabled on the device.
    hdr_metadata: bool,
    color_space: ColorSpace,
    surface: Surface,
    gpu: Gpu,
}

/// The swapchain and the pipeline, whose viewport is the swapchain's size, which are
/// recreated together. The fields drop in order, the reverse of creation.
struct Target {
    pipeline: Owned<vk::Pipeline>,
    swapchain: Swapchain,
}

impl Target {
    unsafe fn new(
        gpu: &Gpu,
        surface: vk::SurfaceKHR,
        surface_format: vk::SurfaceFormatKHR,
        present_mode: vk::PresentModeKHR,
        hdr_metadata: bool,
        pipeline_layout: vk::PipelineLayout,
        window_extent: vk::Extent2D,
    ) -> Result<Self> {
        unsafe {
            let swapchain =
                Swapchain::new(gpu, surface, surface_format, present_mode, window_extent)?;
            if hdr_metadata {
                set_hdr_metadata(&gpu.device, swapchain.handle, surface_format.color_space);
            }
            let pipeline = pipeline::create_pipeline(
                gpu,
                surface_format.format,
                pipeline_layout,
                swapchain.extent,
            )?;
            Ok(Self {
                pipeline,
                swapchain,
            })
        }
    }
}

impl Renderer for App {
    type Settings = Settings;

    unsafe fn new(
        window: WaylandWindow,
        window_extent: vk::Extent2D,
        settings: &Settings,
    ) -> Result<Self> {
        unsafe {
            // Every object is destroyed when it goes out of scope, also when a step fails
            // and `?` returns early.
            let (gpu, surface, extensions) = gpu::create_gpu(window)?;

            surface_support::print_surface_support(
                &gpu,
                surface.handle,
                extensions.surface_maintenance1,
            )?;
            let surface_format = choose_surface_format(&gpu, surface.handle, settings.color_space)?;
            let present_mode =
                present_mode::choose_present_mode(&gpu, surface.handle, settings.present_mode)?;
            println!(
                "swapchain: {:?} / {:?}, present mode {:?}",
                surface_format.format, surface_format.color_space, present_mode
            );
            let pipeline_layout = gpu.own(
                gpu.device
                    .create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None)?,
            );
            let vertex_buffer = upload_buffer(
                &gpu,
                "triangle vertex buffer",
                vk::BufferUsageFlags::VERTEX_BUFFER,
                bytemuck::cast_slice(&VERTICES),
            )?;
            let frame = Frame::new(&gpu)?;
            let target = Target::new(
                &gpu,
                surface.handle,
                surface_format,
                present_mode,
                extensions.hdr_metadata,
                *pipeline_layout,
                window_extent,
            )?;

            Ok(Self {
                target: Some(target),
                frame,
                vertex_buffer,
                pipeline_layout,
                present_mode,
                hdr_metadata: extensions.hdr_metadata,
                color_space: settings.color_space,
                surface,
                gpu,
            })
        }
    }

    unsafe fn draw(&mut self) -> Result<SwapchainStatus> {
        unsafe {
            let device = &self.gpu.device;
            let target = self.target.as_ref().context("there is no swapchain")?;
            // The barrier into COLOR_ATTACHMENT_OPTIMAL starts at TOP_OF_PIPE, so the
            // frame has to wait for the image there for the transition to come after
            // the acquire.
            self.frame.draw(
                &self.gpu,
                &target.swapchain,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                |command_buffer, image_index| {
                    let image = target.swapchain.images[image_index];
                    to_color_attachment(device, command_buffer, image);

                    let color_attachments = [vk::RenderingAttachmentInfo::default()
                        .image_view(*target.swapchain.image_views[image_index])
                        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                        .load_op(vk::AttachmentLoadOp::CLEAR)
                        .store_op(vk::AttachmentStoreOp::STORE)
                        .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                            0.02, 0.02, 0.03, 1.0,
                        ])))];
                    device.cmd_begin_rendering(
                        command_buffer,
                        &vk::RenderingInfo::default()
                            .render_area(vk::Rect2D::default().extent(target.swapchain.extent))
                            .layer_count(1)
                            .color_attachments(&color_attachments),
                    );
                    device.cmd_bind_pipeline(
                        command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        *target.pipeline,
                    );
                    device.cmd_bind_vertex_buffers(
                        command_buffer,
                        0,
                        &[self.vertex_buffer.buffer],
                        &[0],
                    );
                    device.cmd_draw(command_buffer, VERTICES.len() as u32, 1, 0, 0);
                    device.cmd_end_rendering(command_buffer);

                    to_present(device, command_buffer, image);
                },
            )
        }
    }

    unsafe fn recreate_swapchain(&mut self, window_extent: vk::Extent2D) -> Result<()> {
        unsafe {
            self.gpu.device.device_wait_idle()?;
            // The old swapchain goes first: a surface has one swapchain at a time.
            self.target = None;
            let surface_format =
                choose_surface_format(&self.gpu, self.surface.handle, self.color_space)?;
            self.target = Some(Target::new(
                &self.gpu,
                self.surface.handle,
                surface_format,
                self.present_mode,
                self.hdr_metadata,
                *self.pipeline_layout,
                window_extent,
            )?);
            Ok(())
        }
    }

    unsafe fn destroy(self) -> Result<()> {
        unsafe {
            let idle = self.gpu.device.device_wait_idle();
            drop(self);
            Ok(idle?)
        }
    }
}

/// Moves a swapchain image into `COLOR_ATTACHMENT_OPTIMAL` before the rendering writes
/// it. The old layout is given as `UNDEFINED`, which is right whatever layout the image
/// is in, whether it was never drawn to or a frame that drew to it failed before its
/// submission; it discards the contents, which the rendering clears anyway.
unsafe fn to_color_attachment(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
) {
    let barriers = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::empty())
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &barriers,
        );
    }
}

/// Moves a swapchain image from `COLOR_ATTACHMENT_OPTIMAL`, after the rendering wrote
/// it, into `PRESENT_SRC_KHR` for the present. The present waits on the submission's
/// semaphore, which makes the writes available to it.
unsafe fn to_present(device: &Device, command_buffer: vk::CommandBuffer, image: vk::Image) {
    let barriers = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_access_mask(vk::AccessFlags::empty())
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &barriers,
        );
    }
}
