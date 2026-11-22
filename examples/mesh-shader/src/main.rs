//! A task shader and a mesh shader that read their geometry from storage buffers, with
//! `VK_EXT_descriptor_heap`.
//!
//! The shaders (`shaders/mesh.slang`) declare three buffers with `set = 0` and bindings
//! 0 to 2, and a push constant block. There is no descriptor set and no pipeline layout.
//! The pipeline is created with a *mapping* for each binding that says "the index of
//! this descriptor in the resource heap is the `u32` at this offset of the push data"
//! (`pipeline.rs`). So the draw chooses its buffers by pushing three indices, next to
//! the matrix that the shader reads as its push constants.
//!
//! `examples/compute-buffer` shows the simpler mapping with a fixed offset.
//!
//! The image goes to `target/example-output/mesh_shader.ppm`.

mod gpu;
mod pipeline;

use anyhow::{Context, Result, bail};
use glam::{Mat4, Vec3};
use vulkan::{Device, vk};
use vulkan_alloc::descriptor_heap::ResourceHeapPlan;
use vulkan_alloc::{Allocator, MemoryLocation};
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::image::{
    copy_image_to_buffer, create_image_view, image_info_2d, subresource_range,
};
use vulkan_example_common::memory::{
    Buffer, MappedBuffer, buffer_device_address, create_allocator, create_buffer, create_image,
    create_mapped_buffer, read_buffer,
};
use vulkan_example_common::output::{output_path, write_rgba_ppm};

const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 64,
    height: 64,
};
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const IMAGE_BYTES: u64 = EXTENT.width as u64 * EXTENT.height as u64 * 4;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct MeshVertex {
    position: [f32; 4],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct MeshDraw {
    vertex_offset: u32,
    index_offset: u32,
    triangle_count: u32,
    _pad: u32,
}

/// What `vkCmdPushDataEXT` sends for a draw.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct PushData {
    /// The shader's push constant block starts at offset 0 of the push data.
    clip_from_world: [[f32; 4]; 4],
    /// Indices into the resource heap, read by the mappings of the pipeline.
    vertices: u32,
    indices: u32,
    draws: u32,
    _pad: u32,
}

const MESH_VERTICES: [MeshVertex; 3] = [
    MeshVertex {
        position: [-0.6, -0.6, 0.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    MeshVertex {
        position: [0.6, -0.6, 0.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    MeshVertex {
        position: [0.0, 0.6, 0.0, 1.0],
        color: [0.0, 0.2, 1.0, 1.0],
    },
];

const MESH_INDICES: [u32; 3] = [0, 1, 2];

const MESH_DRAWS: [MeshDraw; 1] = [MeshDraw {
    vertex_offset: 0,
    index_offset: 0,
    triangle_count: 1,
    _pad: 0,
}];

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let Some((gpu, properties)) = gpu::create_gpu()? else {
            println!("skipped: no Vulkan 1.4 device with mesh shaders and descriptor heaps");
            return Ok(());
        };
        let device = &gpu.device;
        let mut allocator = create_allocator(&gpu);

        let alignment = properties.storage_buffer_alignment;
        let geometry = [
            create_storage_buffer(
                device,
                &mut allocator,
                "vertices",
                &MESH_VERTICES,
                alignment,
            )?,
            create_storage_buffer(device, &mut allocator, "indices", &MESH_INDICES, alignment)?,
            create_storage_buffer(device, &mut allocator, "draws", &MESH_DRAWS, alignment)?,
        ];
        let readback = create_buffer(
            device,
            &mut allocator,
            "readback",
            IMAGE_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
        )?;

        // The resource heap: three buffer descriptors, then the range the implementation
        // reserves for itself.
        let mut plan = ResourceHeapPlan::new(&properties.heap);
        let slots = plan.buffers(3)?;
        let layout = plan.finish()?;
        let mut heap = create_mapped_buffer(
            device,
            &mut allocator,
            "resource heap",
            layout.size,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::CpuToGpu,
            layout.alignment,
        )?;

        // One call writes all three descriptors, each into its own slot of the heap.
        let ranges = geometry.each_ref().map(|buffer| {
            vk::DeviceAddressRangeKHR::default()
                .address(buffer_device_address(device, buffer.buffer))
                .size(buffer.size)
        });
        let descriptors = ranges.each_ref().map(|range| {
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::STORAGE_BUFFER)
                .data(vk::ResourceDescriptorDataEXT::address_range(range))
        });
        let destinations = slots
            .slices(heap.bytes())
            .context("the heap is smaller than its layout")?
            .map(|bytes| vk::HostAddressRangeEXT::default().address(bytes))
            .collect::<Vec<_>>();
        device.write_resource_descriptors_ext(&descriptors, &destinations)?;

        let image = create_image(
            device,
            &mut allocator,
            "color image",
            &image_info_2d(
                FORMAT,
                EXTENT,
                vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
            ),
        )?;
        let image_view =
            create_image_view(device, image.image, FORMAT, vk::ImageAspectFlags::COLOR)?;
        let pipeline = pipeline::create_pipeline(device, &slots)?;

        // The matrix for the shader, and which descriptors the draw uses.
        let push_data = PushData {
            clip_from_world: clip_from_world().to_cols_array_2d(),
            vertices: slots.index(0).context("slot 0 exists")?,
            indices: slots.index(1).context("slot 1 exists")?,
            draws: slots.index(2).context("slot 2 exists")?,
            _pad: 0,
        };
        submit_and_wait(&gpu, |command_buffer| {
            make_host_writes_visible(device, command_buffer, &heap, &geometry);
            prepare_color_attachment(device, command_buffer, image.image);

            let attachments = [vk::RenderingAttachmentInfo::default()
                .image_view(image_view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                    0.0, 0.0, 0.0, 1.0,
                ])))];
            device.cmd_begin_rendering(
                command_buffer,
                &vk::RenderingInfo::default()
                    .render_area(vk::Rect2D::default().extent(EXTENT))
                    .layer_count(1)
                    .color_attachments(&attachments),
            );
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_bind_resource_heap_ext(
                command_buffer,
                &layout.bind_info(buffer_device_address(device, heap.buffer))?,
            );
            device.cmd_push_data_ext(
                command_buffer,
                &vk::PushDataInfoEXT::default().data(
                    vk::HostAddressRangeConstEXT::default().address(bytemuck::bytes_of(&push_data)),
                ),
            );
            device.cmd_draw_mesh_tasks_ext(command_buffer, 1, 1, 1);
            device.cmd_end_rendering(command_buffer);

            copy_to_readback(device, command_buffer, image.image, &readback);
            Ok(())
        })?;

        let pixels = read_buffer(&mut allocator, &readback)?;
        verify(&pixels)?;
        let path = output_path("mesh_shader.ppm")?;
        write_rgba_ppm(&path, EXTENT.width, EXTENT.height, &pixels)?;
        println!("wrote {}", path.display());

        device.destroy_pipeline(pipeline, None);
        device.destroy_image_view(image_view, None);
        image.destroy(device, &mut allocator)?;
        heap.destroy(device, &mut allocator)?;
        readback.destroy(device, &mut allocator)?;
        for buffer in geometry.into_iter().rev() {
            buffer.destroy(device, &mut allocator)?;
        }
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

fn clip_from_world() -> Mat4 {
    let view = glam::camera::rh::view::look_at_mat4(Vec3::new(0.0, 0.0, 2.2), Vec3::ZERO, Vec3::Y);
    let projection = glam::camera::rh::proj::directx::perspective(
        55.0_f32.to_radians(),
        EXTENT.width as f32 / EXTENT.height as f32,
        0.1,
        16.0,
    );
    projection * view
}

/// A storage buffer holding `values`, addressable by a descriptor in the heap.
unsafe fn create_storage_buffer<T: bytemuck::Pod>(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    values: &[T],
    alignment: u64,
) -> Result<MappedBuffer> {
    unsafe {
        let data: &[u8] = bytemuck::cast_slice(values);
        let mut buffer = create_mapped_buffer(
            device,
            allocator,
            name,
            data.len() as u64,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::CpuToGpu,
            alignment,
        )?;
        buffer.bytes().copy_from_slice(data);
        Ok(buffer)
    }
}

/// What the host wrote becomes visible to the shaders: the heap as descriptors, the
/// geometry as storage.
unsafe fn make_host_writes_visible(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    heap: &MappedBuffer,
    geometry: &[MappedBuffer],
) {
    let host_write = |buffer: &MappedBuffer, access| {
        vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::HOST)
            .src_access_mask(vk::AccessFlags2::HOST_WRITE)
            .dst_stage_mask(
                vk::PipelineStageFlags2::TASK_SHADER_EXT | vk::PipelineStageFlags2::MESH_SHADER_EXT,
            )
            .dst_access_mask(access)
            .buffer(buffer.buffer)
            .size(buffer.size)
    };
    let barriers = std::iter::once(host_write(heap, vk::AccessFlags2::RESOURCE_HEAP_READ_EXT))
        .chain(
            geometry
                .iter()
                .map(|buffer| host_write(buffer, vk::AccessFlags2::SHADER_STORAGE_READ)),
        )
        .collect::<Vec<_>>();
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&barriers),
        );
    }
}

unsafe fn prepare_color_attachment(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
) {
    let to_color = [vk::ImageMemoryBarrier2::default()
        .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().image_memory_barriers(&to_color),
        );
    }
}

/// Copies the rendered image to `readback`, where [`read_buffer`] can read it once the
/// submission is waited for.
unsafe fn copy_to_readback(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    readback: &Buffer,
) {
    let to_transfer = [vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
        .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::COPY)
        .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
        .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .image(image)
        .subresource_range(subresource_range(vk::ImageAspectFlags::COLOR))];
    let to_host = [vk::BufferMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::COPY)
        .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ)
        .buffer(readback.buffer)
        .size(readback.size)];
    unsafe {
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().image_memory_barriers(&to_transfer),
        );
        copy_image_to_buffer(device, command_buffer, image, EXTENT, readback.buffer);
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&to_host),
        );
    }
}

/// Checks that the triangle covers the center and leaves the corners clear.
fn verify(rgba: &[u8]) -> Result<()> {
    let (width, height) = (EXTENT.width as usize, EXTENT.height as usize);
    let pixels = rgba.chunks_exact(4).collect::<Vec<_>>();
    let colored_pixels = pixels
        .iter()
        .filter(|px| px[0] > 32 || px[1] > 32 || px[2] > 32)
        .count();
    if colored_pixels < 128 {
        bail!("mesh shader draw covered too few pixels: {colored_pixels}");
    }

    let center = pixels[(height / 2) * width + width / 2];
    if center[0] <= 16 && center[1] <= 16 && center[2] <= 16 {
        bail!("mesh shader draw left the image center black");
    }

    for (x, y) in [
        (0, 0),
        (width - 1, 0),
        (0, height - 1),
        (width - 1, height - 1),
    ] {
        let px = pixels[y * width + x];
        if px[0] > 16 || px[1] > 16 || px[2] > 16 {
            bail!("mesh shader draw colored a clear corner pixel at ({x}, {y})");
        }
    }
    Ok(())
}
