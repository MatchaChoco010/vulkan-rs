//! A ray tracing pipeline that traces one triangle, with `VK_EXT_descriptor_heap`.
//!
//! The triangle goes into a bottom-level acceleration structure, and one instance of
//! that into the top level that rays are traced against (`acceleration.rs`). The
//! pipeline has a ray generation shader, a miss shader and a closest hit shader, and
//! the shader binding table holds one record for each (`shader_binding_table.rs`).
//!
//! The ray generation shader (`shaders/raygen.slang`) declares the acceleration
//! structure and an output buffer with `set = 0` and bindings 0 and 1, and a push
//! constant block. There is no descriptor set and no pipeline layout. Both descriptors
//! are written into the resource heap, the pipeline is created with a mapping from each
//! binding to the byte offset of its descriptor (`pipeline.rs`), and
//! `vkCmdPushDataEXT` supplies the push constants.
//!
//! The image goes to `target/example-output/ray_tracing.ppm`.

mod acceleration;
mod geometry;
mod gpu;
mod pipeline;
mod shader_binding_table;

use anyhow::{Context, Result, bail};
use glam::{Mat4, Vec3};
use vulkan::{Device, vk};
use vulkan_alloc::MemoryLocation;
use vulkan_alloc::descriptor_heap::{DescriptorSlots, ResourceHeapPlan};
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::memory::{
    Buffer, MappedBuffer, buffer_device_address, create_aligned_buffer, create_allocator,
    create_mapped_buffer, read_buffer, write_buffer,
};
use vulkan_example_common::output::{output_path, write_ppm};

use acceleration::AccelerationStructure;
use shader_binding_table::ShaderBindingTable;

/// The size that the ray generation shader assumes.
const EXTENT: vk::Extent2D = vk::Extent2D {
    width: 128,
    height: 128,
};
/// One `uint` per pixel.
const OUTPUT_BYTES: u64 = EXTENT.width as u64 * EXTENT.height as u64 * 4;

/// What `vkCmdPushDataEXT` sends; the ray generation shader reads it as its push
/// constant block.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct PushData {
    /// Where the shader looks up the colors of the triangle that a ray hit.
    vertex_address: u64,
    index_address: u64,
    camera_origin: [f32; 3],
    _pad0: f32,
    world_from_clip: [[f32; 4]; 4],
    background_bottom: [f32; 3],
    _pad1: f32,
    background_top: [f32; 3],
    _pad2: f32,
}

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let Some((gpu, properties)) = gpu::create_gpu()? else {
            println!(
                "skipped: no Vulkan 1.4 device with ray tracing pipelines and descriptor heaps"
            );
            return Ok(());
        };
        let device = &gpu.device;
        let mut allocator = create_allocator(&gpu);

        let scene = acceleration::build_scene(&gpu, &mut allocator, properties.scratch_alignment)?;
        let output = create_aligned_buffer(
            device,
            &mut allocator,
            "output",
            OUTPUT_BYTES,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::GpuToCpu,
            properties.storage_buffer_alignment,
        )?;
        // Cleared, so that a pixel that the shader did not write cannot pass the check.
        write_buffer(&mut allocator, &output, &vec![0; OUTPUT_BYTES as usize])?;

        // The resource heap: one descriptor for the acceleration structure and one for
        // the output buffer, then the range the implementation reserves for itself.
        let mut plan = ResourceHeapPlan::new(&properties.heap);
        let slots = plan.buffers(2)?;
        let layout = plan.finish()?;
        // The acceleration structure goes into a slot sized for a buffer descriptor:
        // both hold an address range. Ask for the exact size instead of assuming it fits.
        let acceleration_structure_descriptor_size =
            gpu.instance.get_physical_device_descriptor_size_ext(
                gpu.physical_device,
                vk::DescriptorType::ACCELERATION_STRUCTURE_KHR,
            );
        if acceleration_structure_descriptor_size > slots.descriptor_size {
            bail!(
                "an acceleration structure descriptor needs {acceleration_structure_descriptor_size} bytes, a buffer slot has {}",
                slots.descriptor_size
            );
        }
        let mut heap = create_mapped_buffer(
            device,
            &mut allocator,
            "resource heap",
            layout.size,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            layout.alignment,
        )?;
        write_descriptors(device, &mut heap, &slots, &scene.top_level, &output)?;

        let pipeline = pipeline::create_pipeline(device, &slots)?;
        let shader_binding_table =
            ShaderBindingTable::new(device, &mut allocator, pipeline, &properties.ray_tracing)?;

        let push_data = PushData {
            vertex_address: buffer_device_address(device, scene.vertex_buffer.buffer),
            index_address: buffer_device_address(device, scene.index_buffer.buffer),
            camera_origin: CAMERA_ORIGIN.to_array(),
            _pad0: 0.0,
            world_from_clip: world_from_clip().to_cols_array_2d(),
            background_bottom: [0.03, 0.05, 0.09],
            _pad1: 0.0,
            background_top: [0.24, 0.35, 0.58],
            _pad2: 0.0,
        };
        submit_and_wait(&gpu, |command_buffer| {
            device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::RAY_TRACING_KHR,
                pipeline,
            );
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
            // One ray generation invocation per pixel.
            device.cmd_trace_rays_khr(
                command_buffer,
                &shader_binding_table.raygen,
                &shader_binding_table.miss,
                &shader_binding_table.hit,
                &shader_binding_table.callable,
                EXTENT.width,
                EXTENT.height,
                1,
            );
            make_output_visible_to_host(device, command_buffer, &output);
            Ok(())
        })?;

        let pixels = read_buffer(&mut allocator, &output)?
            .chunks_exact(4)
            .map(|word| u32::from_ne_bytes([word[0], word[1], word[2], word[3]]))
            .collect::<Vec<_>>();
        verify(&pixels)?;
        let path = output_path("ray_tracing.ppm")?;
        write_ppm(
            &path,
            EXTENT.width,
            EXTENT.height,
            pixels.iter().map(|&pixel| rgb(pixel)),
        )?;
        println!("wrote {}", path.display());

        shader_binding_table.destroy(device, &mut allocator)?;
        device.destroy_pipeline(pipeline, None);
        heap.destroy(device, &mut allocator)?;
        output.destroy(device, &mut allocator)?;
        scene.destroy(device, &mut allocator)?;
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

const CAMERA_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, -2.6);

/// The inverse of the camera's view and projection, with which the ray generation
/// shader turns a pixel into the direction of its ray.
fn world_from_clip() -> Mat4 {
    let view = glam::camera::rh::view::look_at_mat4(CAMERA_ORIGIN, Vec3::ZERO, Vec3::Y);
    let projection = glam::camera::rh::proj::directx::perspective(
        50.0_f32.to_radians(),
        EXTENT.width as f32 / EXTENT.height as f32,
        0.1,
        1000.0,
    );
    (projection * view).inverse()
}

/// Writes the descriptors of the top level and the output buffer into the heap, one
/// per slot.
unsafe fn write_descriptors(
    device: &Device,
    heap: &mut MappedBuffer,
    slots: &DescriptorSlots,
    top_level: &AccelerationStructure,
    output: &Buffer,
) -> Result<()> {
    unsafe {
        let top_level_range = vk::DeviceAddressRangeKHR::default()
            .address(top_level.device_address(device))
            .size(top_level.size);
        let output_range = vk::DeviceAddressRangeKHR::default()
            .address(buffer_device_address(device, output.buffer))
            .size(output.size);
        let descriptors = [
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
                .data(vk::ResourceDescriptorDataEXT::address_range(
                    &top_level_range,
                )),
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::STORAGE_BUFFER)
                .data(vk::ResourceDescriptorDataEXT::address_range(&output_range)),
        ];
        let destinations = slots
            .slices(heap.bytes())
            .context("the resource heap is smaller than its layout")?
            .map(|bytes| vk::HostAddressRangeEXT::default().address(bytes))
            .collect::<Vec<_>>();
        device.write_resource_descriptors_ext(&descriptors, &destinations)?;
        Ok(())
    }
}

/// Makes what the shader wrote to `output` visible to the host once the submission is
/// waited for.
unsafe fn make_output_visible_to_host(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    output: &Buffer,
) {
    let barrier = vk::BufferMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::SHADER_WRITE)
        .dst_access_mask(vk::AccessFlags::HOST_READ)
        .buffer(output.buffer)
        .size(output.size);
    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[barrier],
            &[],
        );
    }
}

/// The shader packs each pixel into a `uint`, red in the low bits.
fn rgb(pixel: u32) -> [u8; 3] {
    [pixel as u8, (pixel >> 8) as u8, (pixel >> 16) as u8]
}

/// Checks that enough rays hit the triangle: the background is nowhere this red or
/// green.
fn verify(pixels: &[u32]) -> Result<()> {
    let hit_pixels = pixels
        .iter()
        .filter(|&&pixel| {
            let [r, g, _] = rgb(pixel);
            r > 120 || g > 120
        })
        .count();
    if hit_pixels < 500 {
        bail!("ray tracing output has too few closest-hit pixels: {hit_pixels}");
    }
    Ok(())
}
