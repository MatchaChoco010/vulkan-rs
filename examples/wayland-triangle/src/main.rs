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

use anyhow::Result;
use vulkan::vk;
use vulkan_alloc::Allocator;
use vulkan_example_common::device::Gpu;
use vulkan_example_common::memory::{Buffer, create_allocator, upload_buffer};
use vulkan_example_common::window::frame::{Frame, SwapchainStatus};
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

struct App {
    gpu: Gpu,
    allocator: Allocator,
    surface: vk::SurfaceKHR,
    swapchain: Swapchain,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    frame: Frame,
    vertex_buffer: Buffer,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

impl Renderer for App {
    unsafe fn new(window: WaylandWindow, window_extent: vk::Extent2D) -> Result<Self> {
        unsafe {
            let (gpu, surface) = gpu::create_gpu(window)?;
            let device = &gpu.device;
            let mut allocator = create_allocator(&gpu);

            let swapchain = Swapchain::new(
                &gpu,
                surface,
                choose_surface_format(&gpu, surface)?,
                vk::PresentModeKHR::FIFO,
                window_extent,
            )?;
            let render_pass = pipeline::create_render_pass(device, swapchain.format.format)?;
            let framebuffers = pipeline::create_framebuffers(device, render_pass, &swapchain)?;
            let frame = Frame::new(device, gpu.queue_family)?;
            let vertex_buffer = upload_buffer(
                &gpu,
                &mut allocator,
                "triangle vertex buffer",
                vk::BufferUsageFlags::VERTEX_BUFFER,
                bytemuck::cast_slice(&VERTICES),
            )?;
            let pipeline_layout =
                device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None)?;
            let pipeline =
                pipeline::create_pipeline(device, render_pass, pipeline_layout, swapchain.extent)?;

            Ok(Self {
                gpu,
                allocator,
                surface,
                swapchain,
                render_pass,
                framebuffers,
                frame,
                vertex_buffer,
                pipeline_layout,
                pipeline,
            })
        }
    }

    unsafe fn draw(&mut self) -> Result<SwapchainStatus> {
        unsafe {
            let device = &self.gpu.device;
            // Where the render pass's dependency on the outside starts, so that its
            // layout transition comes after the acquire.
            self.frame.draw(
                &self.gpu,
                &self.swapchain,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                |command_buffer, image_index| {
                    let clear_values = [vk::ClearValue::color(vk::ClearColorValue::float32([
                        0.02, 0.02, 0.03, 1.0,
                    ]))];
                    device.cmd_begin_render_pass(
                        command_buffer,
                        &vk::RenderPassBeginInfo::default()
                            .render_pass(self.render_pass)
                            .framebuffer(self.framebuffers[image_index])
                            .render_area(vk::Rect2D::default().extent(self.swapchain.extent))
                            .clear_values(&clear_values),
                        vk::SubpassContents::INLINE,
                    );
                    device.cmd_bind_pipeline(
                        command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        self.pipeline,
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
            self.destroy_swapchain_resources();

            let device = &self.gpu.device;
            self.swapchain = Swapchain::new(
                &self.gpu,
                self.surface,
                choose_surface_format(&self.gpu, self.surface)?,
                vk::PresentModeKHR::FIFO,
                window_extent,
            )?;
            self.render_pass = pipeline::create_render_pass(device, self.swapchain.format.format)?;
            self.framebuffers =
                pipeline::create_framebuffers(device, self.render_pass, &self.swapchain)?;
            self.pipeline = pipeline::create_pipeline(
                device,
                self.render_pass,
                self.pipeline_layout,
                self.swapchain.extent,
            )?;
            Ok(())
        }
    }

    unsafe fn destroy(mut self) {
        unsafe {
            let _ = self.gpu.device.device_wait_idle();
            self.destroy_swapchain_resources();
            let device = &self.gpu.device;
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.frame.destroy(device);
            let _ = self.vertex_buffer.destroy(device, &mut self.allocator);
            drop(self.allocator);
            self.gpu.instance.destroy_surface_khr(self.surface, None);
            self.gpu.destroy();
        }
    }
}

impl App {
    /// Destroys what depends on the swapchain, and the swapchain. A recreation that
    /// failed halfway leaves null handles and empty lists, which this skips.
    unsafe fn destroy_swapchain_resources(&mut self) {
        unsafe {
            let device = &self.gpu.device;
            if !self.pipeline.is_null() {
                device.destroy_pipeline(self.pipeline, None);
                self.pipeline = vk::Pipeline::null();
            }
            for framebuffer in self.framebuffers.drain(..) {
                device.destroy_framebuffer(framebuffer, None);
            }
            if !self.render_pass.is_null() {
                device.destroy_render_pass(self.render_pass, None);
                self.render_pass = vk::RenderPass::null();
            }
            self.swapchain.destroy(device);
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
