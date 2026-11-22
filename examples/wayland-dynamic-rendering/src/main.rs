//! A triangle on a Wayland surface with dynamic rendering.
//!
//! There is no render pass and no framebuffer: `vkCmdBeginRendering` takes the
//! swapchain image's view directly, and the frame moves the image into
//! `COLOR_ATTACHMENT_OPTIMAL` and back to `PRESENT_SRC_KHR` with barriers of its own.
//!
//! The swapchain parameters a compositor or an HDR-aware client cares about can be
//! chosen from the environment. The surface's support for each is printed at startup.
//!
//! - `VULKAN_EXAMPLE_PRESENT_MODE`: `fifo` (default), `fifo_relaxed`, `immediate`
//!   (tearing) or `mailbox`.
//! - `VULKAN_EXAMPLE_COLOR_SPACE`: `srgb` (default), `hdr10` (BT.2020 PQ, with
//!   `VK_EXT_hdr_metadata` when available) or `scrgb` (extended linear sRGB).
//! - `VULKAN_EXAMPLE_FRAMES`: exit after this many frames.
//!
//! The shader always writes the same values, so the triangle is only colorimetrically
//! right in `srgb`. The point of the other modes is the swapchain setup.

mod color_space;
mod gpu;
mod pipeline;
mod present_mode;
mod surface_support;

use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_alloc::Allocator;
use vulkan_example_common::device::Gpu;
use vulkan_example_common::image::subresource_range;
use vulkan_example_common::memory::{Buffer, create_allocator, upload_buffer};
use vulkan_example_common::window::frame::{Frame, SwapchainStatus};
use vulkan_example_common::window::swapchain::Swapchain;
use vulkan_example_common::window::{self, Renderer, WaylandWindow};

use color_space::{choose_surface_format, set_hdr_metadata};
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
    window::run::<App>("vulkan wayland dynamic rendering")
}

struct App {
    gpu: Gpu,
    /// `VK_EXT_hdr_metadata` is enabled on the device.
    hdr_metadata: bool,
    allocator: Allocator,
    surface: vk::SurfaceKHR,
    present_mode: vk::PresentModeKHR,
    swapchain: Swapchain,
    /// The layout each swapchain image was left in: `UNDEFINED` until it is first drawn
    /// to, `PRESENT_SRC_KHR` after.
    image_layouts: Vec<vk::ImageLayout>,
    frame: Frame,
    vertex_buffer: Buffer,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
}

impl Renderer for App {
    unsafe fn new(window: WaylandWindow, window_extent: vk::Extent2D) -> Result<Self> {
        unsafe {
            let (gpu, surface, extensions) = gpu::create_gpu(window)?;
            let device = &gpu.device;
            let mut allocator = create_allocator(&gpu);

            surface_support::print_surface_support(&gpu, surface, extensions.surface_maintenance1)?;
            let surface_format = choose_surface_format(&gpu, surface)?;
            let present_mode = present_mode::choose_present_mode(&gpu, surface)?;
            println!(
                "swapchain: {:?} / {:?}, present mode {:?}",
                surface_format.format, surface_format.color_space, present_mode
            );
            let swapchain =
                Swapchain::new(&gpu, surface, surface_format, present_mode, window_extent)?;
            if extensions.hdr_metadata {
                set_hdr_metadata(device, swapchain.handle, surface_format.color_space);
            }
            let image_layouts = vec![vk::ImageLayout::UNDEFINED; swapchain.images.len()];

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
            let pipeline = pipeline::create_pipeline(
                device,
                surface_format.format,
                pipeline_layout,
                swapchain.extent,
            )?;

            Ok(Self {
                gpu,
                hdr_metadata: extensions.hdr_metadata,
                allocator,
                surface,
                present_mode,
                swapchain,
                image_layouts,
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
            // The barrier into COLOR_ATTACHMENT_OPTIMAL starts at TOP_OF_PIPE, so the
            // frame has to wait for the image there for the transition to come after
            // the acquire.
            self.frame.draw(
                &self.gpu,
                &self.swapchain,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                |command_buffer, image_index| {
                    let image = self.swapchain.images[image_index];
                    to_color_attachment(
                        device,
                        command_buffer,
                        image,
                        self.image_layouts[image_index],
                    );

                    let color_attachments = [vk::RenderingAttachmentInfo::default()
                        .image_view(self.swapchain.image_views[image_index])
                        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                        .load_op(vk::AttachmentLoadOp::CLEAR)
                        .store_op(vk::AttachmentStoreOp::STORE)
                        .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                            0.02, 0.02, 0.03, 1.0,
                        ])))];
                    device.cmd_begin_rendering(
                        command_buffer,
                        &vk::RenderingInfo::default()
                            .render_area(vk::Rect2D::default().extent(self.swapchain.extent))
                            .layer_count(1)
                            .color_attachments(&color_attachments),
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
                    device.cmd_end_rendering(command_buffer);

                    to_present(device, command_buffer, image);
                    self.image_layouts[image_index] = vk::ImageLayout::PRESENT_SRC_KHR;
                },
            )
        }
    }

    unsafe fn recreate_swapchain(&mut self, window_extent: vk::Extent2D) -> Result<()> {
        unsafe {
            self.gpu.device.device_wait_idle()?;
            self.destroy_swapchain_resources();

            let device = &self.gpu.device;
            let surface_format = choose_surface_format(&self.gpu, self.surface)?;
            self.swapchain = Swapchain::new(
                &self.gpu,
                self.surface,
                surface_format,
                self.present_mode,
                window_extent,
            )?;
            if self.hdr_metadata {
                set_hdr_metadata(device, self.swapchain.handle, surface_format.color_space);
            }
            self.image_layouts = vec![vk::ImageLayout::UNDEFINED; self.swapchain.images.len()];
            self.pipeline = pipeline::create_pipeline(
                device,
                surface_format.format,
                self.pipeline_layout,
                self.swapchain.extent,
            )?;
            Ok(())
        }
    }

    unsafe fn destroy(mut self) -> Result<()> {
        unsafe {
            let idle = self.gpu.device.device_wait_idle();
            self.destroy_swapchain_resources();
            let device = &self.gpu.device;
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.frame.destroy(device);
            let freed = self.vertex_buffer.destroy(device, &mut self.allocator);
            drop(self.allocator);
            self.gpu.instance.destroy_surface_khr(self.surface, None);
            self.gpu.destroy();
            idle?;
            freed
        }
    }
}

impl App {
    /// Destroys the pipeline, whose viewport is the swapchain's size, and the swapchain.
    /// A recreation that failed halfway leaves null handles, which this skips.
    unsafe fn destroy_swapchain_resources(&mut self) {
        unsafe {
            let device = &self.gpu.device;
            if !self.pipeline.is_null() {
                device.destroy_pipeline(self.pipeline, None);
                self.pipeline = vk::Pipeline::null();
            }
            self.swapchain.destroy(device);
        }
    }
}

/// Moves a swapchain image from `old_layout` into `COLOR_ATTACHMENT_OPTIMAL` before the
/// rendering writes it.
unsafe fn to_color_attachment(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    old_layout: vk::ImageLayout,
) {
    let barriers = [vk::ImageMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::empty())
        .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .old_layout(old_layout)
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
