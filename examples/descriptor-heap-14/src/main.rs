//! Textured spheres whose fragment shader finds its descriptors in the heaps by
//! itself, with `VK_EXT_descriptor_heap`.
//!
//! The shaders (`shaders/graphics.slang`) declare no bindings. The fragment shader
//! dereferences `DescriptorHandle`s, which hold indices into the resource heap and the
//! sampler heap: the push data hands it the handles of the material buffer and of the
//! sampler, and every material holds the handle of its texture. So the pipeline has
//! neither a layout nor mappings (`pipeline.rs`). The example writes each descriptor
//! where its handle points (`heaps.rs`), binds both heaps, and pushes the handles.
//!
//! `examples/compute-buffer` and `examples/mesh-shader` show shaders with ordinary
//! bindings, which mappings connect to the heap.
//!
//! The image goes to `target/example-output/descriptor_heap_spheres.ppm`.

mod gpu;
mod heaps;
mod pipeline;
mod scene;
mod targets;
mod texture;

use anyhow::{Result, bail};
use vulkan::vk;
use vulkan_alloc::MemoryLocation;
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::memory::{
    buffer_device_address, create_buffer, read_buffer, upload_aligned_buffer, upload_buffer,
};
use vulkan_example_common::output::{output_path, write_rgba_ppm};

use heaps::{DescriptorHandle, ResourceHeapLayout, SamplerHeapLayout};

const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 320,
    height: 240,
};
const COLOR_FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const DEPTH_FORMAT: vk::Format = vk::Format::D32_SFLOAT;
const IMAGE_BYTES: u64 = EXTENT.width as u64 * EXTENT.height as u64 * 4;

/// What `vkCmdPushDataEXT` sends; the shaders read it as their push constant block.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct PushData {
    clip_from_world: [[f32; 4]; 4],
    materials: DescriptorHandle,
    sampler: DescriptorHandle,
}

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let Some((gpu, properties)) = gpu::create_gpu()? else {
            println!("skipped: no Vulkan 1.4 graphics device with VK_EXT_descriptor_heap");
            return Ok(());
        };
        // Every object below is destroyed when it goes out of scope, in the reverse order
        // of creation, also when a step fails and `?` returns early.
        let device = &gpu.device;

        // Where each descriptor goes, and so the handle that the shader uses for it.
        let resource_layout =
            ResourceHeapLayout::new(&properties.heap, scene::TEXTURES.len() as u32)?;
        let sampler_layout = SamplerHeapLayout::new(&properties.heap)?;

        let mesh = scene::sphere(24, 16);
        let vertex_buffer = upload_buffer(
            &gpu,
            "sphere vertices",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            bytemuck::cast_slice(&mesh.vertices),
        )?;
        let index_buffer = upload_buffer(
            &gpu,
            "sphere indices",
            vk::BufferUsageFlags::INDEX_BUFFER,
            bytemuck::cast_slice(&mesh.indices),
        )?;
        let instance_buffer = upload_buffer(
            &gpu,
            "sphere instances",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            bytemuck::cast_slice(&scene::INSTANCES),
        )?;
        let materials =
            scene::materials([resource_layout.texture(0)?, resource_layout.texture(1)?]);
        let material_buffer = upload_aligned_buffer(
            &gpu,
            "materials",
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            bytemuck::cast_slice(&materials),
            properties.storage_buffer_alignment,
        )?;
        let textures = scene::TEXTURES
            .iter()
            .map(|checker| {
                texture::create_texture(
                    &gpu,
                    checker.name,
                    scene::TEXTURE_EXTENT,
                    &checker.texels(),
                )
            })
            .collect::<Result<Vec<_>>>()?;

        let mut resource_heap = heaps::create_heap(&gpu, "resource heap", &resource_layout.heap)?;
        heaps::write_resource_descriptors(
            &gpu,
            &mut resource_heap,
            &resource_layout,
            &material_buffer,
            &textures,
        )?;
        let mut sampler_heap = heaps::create_heap(&gpu, "sampler heap", &sampler_layout.heap)?;
        heaps::write_sampler_descriptor(device, &mut sampler_heap, &sampler_layout)?;

        let targets = targets::RenderTargets::new(&gpu)?;
        let readback = create_buffer(
            &gpu,
            "readback",
            IMAGE_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
        )?;
        let pipeline = pipeline::create_pipeline(&gpu)?;

        let push_data = PushData {
            clip_from_world: scene::clip_from_world().to_cols_array_2d(),
            materials: resource_layout.materials()?,
            sampler: sampler_layout.sampler()?,
        };
        submit_and_wait(&gpu, |command_buffer| {
            heaps::make_host_writes_visible(device, command_buffer, &resource_heap, &sampler_heap);
            targets.begin_rendering(device, command_buffer);
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, *pipeline);
            device.cmd_bind_resource_heap_ext(
                command_buffer,
                &resource_layout
                    .heap
                    .bind_info(buffer_device_address(&gpu, resource_heap.buffer))?,
            );
            device.cmd_bind_sampler_heap_ext(
                command_buffer,
                &sampler_layout
                    .heap
                    .bind_info(buffer_device_address(&gpu, sampler_heap.buffer))?,
            );
            device.cmd_push_data_ext(
                command_buffer,
                &vk::PushDataInfoEXT::default().data(
                    vk::HostAddressRangeConstEXT::default().address(bytemuck::bytes_of(&push_data)),
                ),
            );
            device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[vertex_buffer.buffer, instance_buffer.buffer],
                &[0, 0],
            );
            device.cmd_bind_index_buffer(
                command_buffer,
                index_buffer.buffer,
                0,
                vk::IndexType::UINT32,
            );
            device.cmd_draw_indexed(
                command_buffer,
                mesh.indices.len() as u32,
                scene::INSTANCES.len() as u32,
                0,
                0,
                0,
            );
            device.cmd_end_rendering(command_buffer);
            targets.copy_color_to(device, command_buffer, &readback);
            Ok(())
        })?;

        let rgba = read_buffer(&gpu, &readback)?;
        verify(&rgba)?;
        let path = output_path("descriptor_heap_spheres.ppm")?;
        write_rgba_ppm(&path, EXTENT.width, EXTENT.height, &rgba)?;
        println!("wrote {}", path.display());
        Ok(())
    }
}

/// Checks that enough of the image is lit, in each quarter of its width, which is where
/// the four spheres are.
fn verify(rgba: &[u8]) -> Result<()> {
    let width = EXTENT.width as usize;
    let mut bright_per_quarter = [0_usize; 4];
    for (index, pixel) in rgba.chunks_exact(4).enumerate() {
        if pixel[0] > 80 || pixel[1] > 80 || pixel[2] > 120 {
            bright_per_quarter[(index % width) * 4 / width] += 1;
        }
    }
    let bright_pixels = bright_per_quarter.iter().sum::<usize>();
    if bright_pixels < 1_500 {
        bail!("descriptor heap draw produced too few visible textured pixels: {bright_pixels}");
    }
    if bright_per_quarter.iter().any(|&count| count < 1_000) {
        bail!("descriptor heap draw did not cover all four sphere regions: {bright_per_quarter:?}");
    }
    Ok(())
}
