//! A triangle drawn with a render pass into an offscreen image, without a window.
//!
//! The vertex and index buffers are uploaded to device-local memory from `vulkan-alloc`.
//! After the draw, the image is copied to a host-visible buffer and written to
//! `target/example-output/triangle.ppm`. The allocator statistics printed at the end
//! show how the allocations were placed in memory blocks.

mod gpu;
mod pipeline;

use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_alloc::MemoryLocation;
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::image::{copy_image_to_buffer, create_image_view, image_info_2d};
use vulkan_example_common::memory::{
    Buffer, create_allocator, create_buffer, create_image, read_buffer, upload_buffer,
};
use vulkan_example_common::output::{output_path, write_rgba_ppm};

use pipeline::Vertex;

const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 512,
    height: 512,
};
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const READBACK_BYTES: u64 = EXTENT.width as u64 * EXTENT.height as u64 * 4;

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
const INDICES: [u16; 3] = [0, 1, 2];

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let gpu = gpu::create_gpu()?;
        let device = &gpu.device;
        let mut allocator = create_allocator(&gpu);

        let vertex_buffer = upload_buffer(
            &gpu,
            &mut allocator,
            "triangle vertex buffer",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            bytemuck::cast_slice(&VERTICES),
        )?;
        let index_buffer = upload_buffer(
            &gpu,
            &mut allocator,
            "triangle index buffer",
            vk::BufferUsageFlags::INDEX_BUFFER,
            bytemuck::cast_slice(&INDICES),
        )?;
        let image = create_image(
            device,
            &mut allocator,
            "offscreen color image",
            &image_info_2d(
                FORMAT,
                EXTENT,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
            ),
        )?;
        let image_view =
            create_image_view(device, image.image, FORMAT, vk::ImageAspectFlags::COLOR)?;
        let readback = create_buffer(
            device,
            &mut allocator,
            "readback buffer",
            READBACK_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
        )?;

        let render_pass = pipeline::create_render_pass(device, FORMAT)?;
        let framebuffer = pipeline::create_framebuffer(device, render_pass, image_view, EXTENT)?;
        let pipeline_layout =
            device.create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None)?;
        let pipeline = pipeline::create_pipeline(device, render_pass, pipeline_layout, EXTENT)?;

        submit_and_wait(&gpu, |command_buffer| {
            let clear_values = [vk::ClearValue::color(vk::ClearColorValue::float32([
                0.02, 0.02, 0.03, 1.0,
            ]))];
            device.cmd_begin_render_pass(
                command_buffer,
                &vk::RenderPassBeginInfo::default()
                    .render_pass(render_pass)
                    .framebuffer(framebuffer)
                    .render_area(vk::Rect2D::default().extent(EXTENT))
                    .clear_values(&clear_values),
                vk::SubpassContents::INLINE,
            );
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
            device.cmd_bind_index_buffer(
                command_buffer,
                index_buffer.buffer,
                0,
                vk::IndexType::UINT16,
            );
            device.cmd_draw_indexed(command_buffer, INDICES.len() as u32, 1, 0, 0, 0);
            device.cmd_end_render_pass(command_buffer);

            // The render pass left the image in TRANSFER_SRC_OPTIMAL.
            copy_image_to_buffer(device, command_buffer, image.image, EXTENT, readback.buffer);
            make_visible_to_host(device, command_buffer, &readback);
            Ok(())
        })?;

        let path = output_path("triangle.ppm")?;
        write_rgba_ppm(
            &path,
            EXTENT.width,
            EXTENT.height,
            &read_buffer(&mut allocator, &readback)?,
        )?;
        let stats = allocator.stats();
        eprintln!(
            "allocator: {} blocks, {} allocated bytes, {} free bytes",
            stats.block_count, stats.allocated_bytes, stats.free_bytes
        );

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_framebuffer(framebuffer, None);
        device.destroy_render_pass(render_pass, None);
        device.destroy_image_view(image_view, None);
        for buffer in [readback, index_buffer, vertex_buffer] {
            buffer.destroy(device, &mut allocator)?;
        }
        image.destroy(device, &mut allocator)?;
        drop(allocator);
        gpu.destroy();

        println!("wrote {}", path.display());
        Ok(())
    }
}

/// Makes what the copy wrote to `buffer` visible to the host once the submission is
/// waited for.
unsafe fn make_visible_to_host(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    buffer: &Buffer,
) {
    let barrier = vk::BufferMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(vk::AccessFlags::HOST_READ)
        .buffer(buffer.buffer)
        .size(buffer.size);
    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[barrier],
            &[],
        );
    }
}
