//! A triangle on a Wayland surface, drawn with a render pass and a framebuffer for each
//! swapchain image.
//!
//! The render pass clears the image, leaves it in `PRESENT_SRC_KHR`, and orders its
//! layout transitions after the acquire with a dependency on the outside, so the frame
//! needs no barrier of its own. The render pass, the framebuffers and the pipeline,
//! whose viewport is fixed, depend on the swapchain and are recreated with it.
//!
//! - `VULKAN_EXAMPLE_FRAMES`: exit after this many frames.
//!
//! `examples/wayland-dynamic-rendering` draws the same triangle without a render pass.

mod gpu;
mod pipeline;

use anyhow::{Context, Result};
use vulkan::vk;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::memory::{Buffer, upload_buffer};
use vulkan_example_common::window::frame::{Frame, SwapchainStatus};
use vulkan_example_common::window::surface::Surface;
use vulkan_example_common::window::swapchain::Swapchain;
use vulkan_example_common::window::{self, Renderer, WaylandWindow};

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

fn main() -> Result<()> {
    window::run::<App>("vulkan wayland triangle")
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
    surface: Surface,
    gpu: Gpu,
}

/// The swapchain and what depends on it, which is recreated with it. The fields drop in
/// order, the reverse of creation.
struct Target {
    pipeline: Owned<vk::Pipeline>,
    framebuffers: Vec<Owned<vk::Framebuffer>>,
    render_pass: Owned<vk::RenderPass>,
    swapchain: Swapchain,
}

impl Target {
    unsafe fn new(
        gpu: &Gpu,
        surface: vk::SurfaceKHR,
        pipeline_layout: vk::PipelineLayout,
        window_extent: vk::Extent2D,
    ) -> Result<Self> {
        unsafe {
            let swapchain = Swapchain::new(
                gpu,
                surface,
                choose_surface_format(gpu, surface)?,
                vk::PresentModeKHR::FIFO,
                window_extent,
            )?;
            let render_pass = pipeline::create_render_pass(gpu, swapchain.format.format)?;
            let framebuffers = pipeline::create_framebuffers(gpu, *render_pass, &swapchain)?;
            let pipeline =
                pipeline::create_pipeline(gpu, *render_pass, pipeline_layout, swapchain.extent)?;
            Ok(Self {
                pipeline,
                framebuffers,
                render_pass,
                swapchain,
            })
        }
    }
}

impl Renderer for App {
    unsafe fn new(window: WaylandWindow, window_extent: vk::Extent2D) -> Result<Self> {
        unsafe {
            // Every object is destroyed when it goes out of scope, also when a step fails
            // and `?` returns early.
            let (gpu, surface) = gpu::create_gpu(window)?;
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
            let target = Target::new(&gpu, surface.handle, *pipeline_layout, window_extent)?;
            Ok(Self {
                target: Some(target),
                frame,
                vertex_buffer,
                pipeline_layout,
                surface,
                gpu,
            })
        }
    }

    unsafe fn draw(&mut self) -> Result<SwapchainStatus> {
        unsafe {
            let device = &self.gpu.device;
            let target = self.target.as_ref().context("there is no swapchain")?;
            // Where the render pass's dependency on the outside starts, so that its
            // layout transition comes after the acquire.
            self.frame.draw(
                &self.gpu,
                &target.swapchain,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                |command_buffer, image_index| {
                    let clear_values = [vk::ClearValue::color(vk::ClearColorValue::float32([
                        0.02, 0.02, 0.03, 1.0,
                    ]))];
                    device.cmd_begin_render_pass(
                        command_buffer,
                        &vk::RenderPassBeginInfo::default()
                            .render_pass(*target.render_pass)
                            .framebuffer(*target.framebuffers[image_index])
                            .render_area(vk::Rect2D::default().extent(target.swapchain.extent))
                            .clear_values(&clear_values),
                        vk::SubpassContents::INLINE,
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
                    device.cmd_end_render_pass(command_buffer);
                },
            )
        }
    }

    unsafe fn recreate_swapchain(&mut self, window_extent: vk::Extent2D) -> Result<()> {
        unsafe {
            self.gpu.device.device_wait_idle()?;
            // The old swapchain goes first: a surface has one swapchain at a time.
            self.target = None;
            self.target = Some(Target::new(
                &self.gpu,
                self.surface.handle,
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

/// 8-bit BGRA in whatever color space the surface pairs it with, or else the first
/// format the surface offers.
unsafe fn choose_surface_format(
    gpu: &Gpu,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceFormatKHR> {
    let formats = unsafe {
        gpu.instance
            .get_physical_device_surface_formats_khr(gpu.physical_device, surface)
    }?;
    Ok(formats
        .iter()
        .copied()
        .find(|format| format.format == vk::Format::B8G8R8A8_UNORM)
        .unwrap_or(formats[0]))
}
