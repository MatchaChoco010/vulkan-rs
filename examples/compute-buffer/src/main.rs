//! A compute shader that updates a storage buffer, with `VK_EXT_descriptor_heap`.
//!
//! The shader (`shaders/compute.slang`) is ordinary: it declares its buffer with
//! `set = 0, binding = 0` and knows nothing about heaps. There is no descriptor set
//! layout, pipeline layout, descriptor pool or descriptor set on the Vulkan side either.
//! Instead:
//!
//! 1. The descriptor of the buffer is written into a *resource heap*, which is a buffer.
//! 2. The pipeline is created with a *mapping* that says "binding 0 is the descriptor at
//!    this byte offset of the resource heap" (`pipeline.rs`).
//! 3. The heap is bound before the dispatch.
//!
//! This is the smallest of the three ways to reach a heap from a shader. The others are
//! an index taken from push data (`examples/mesh-shader`) and shaders that index the
//! heap themselves (`examples/descriptor-heap-14`).
//!
//! The result goes to `target/example-output/compute_buffer.ppm`.

mod gpu;
mod pipeline;

use anyhow::{Context, Result, bail};
use vulkan::vk;
use vulkan_alloc::MemoryLocation;
use vulkan_alloc::descriptor_heap::ResourceHeapPlan;
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::memory::{
    buffer_device_address, create_allocator, create_mapped_buffer,
};
use vulkan_example_common::output::{output_path, write_ppm};

const ELEMENTS: usize = 64;
const BUFFER_BYTES: u64 = (ELEMENTS * size_of::<u32>()) as u64;

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let Some((gpu, heap_properties)) = gpu::create_gpu()? else {
            println!("skipped: no Vulkan 1.4 device with VK_EXT_descriptor_heap");
            return Ok(());
        };
        let device = &gpu.device;
        let mut allocator = create_allocator(&gpu);

        // The data the shader works on.
        let mut values = create_mapped_buffer(
            device,
            &mut allocator,
            "values",
            BUFFER_BYTES,
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::CpuToGpu,
            1,
        )?;
        for (index, word) in values.bytes().chunks_exact_mut(4).enumerate() {
            word.copy_from_slice(&initial_value(index).to_ne_bytes());
        }

        // The resource heap: room for one buffer descriptor, then the range the
        // implementation reserves for itself.
        let mut plan = ResourceHeapPlan::new(&heap_properties);
        let slots = plan.buffers(1)?;
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

        // Vulkan writes the descriptor into host memory that we hand it as a slice.
        let range = vk::DeviceAddressRangeKHR::default()
            .address(buffer_device_address(device, values.buffer))
            .size(BUFFER_BYTES);
        let descriptor = vk::ResourceDescriptorInfoEXT::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .data(vk::ResourceDescriptorDataEXT::address_range(&range));
        let destination = slots
            .bytes(heap.bytes(), 0)
            .context("the heap is smaller than its layout")?;
        device.write_resource_descriptors_ext(
            &[descriptor],
            &[vk::HostAddressRangeEXT::default().address(destination)],
        )?;

        let heap_offset = slots.offset(0).context("slot 0 exists")?;
        let pipeline = pipeline::create_pipeline(device, u32::try_from(heap_offset)?)?;

        submit_and_wait(&gpu, |command_buffer| {
            // What the host wrote becomes visible to the shader: the heap as descriptors,
            // the values as storage.
            let host_to_shader = [
                vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::HOST)
                    .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                    .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                    .dst_access_mask(vk::AccessFlags2::RESOURCE_HEAP_READ_EXT)
                    .buffer(heap.buffer)
                    .size(layout.size),
                vk::BufferMemoryBarrier2::default()
                    .src_stage_mask(vk::PipelineStageFlags2::HOST)
                    .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                    .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                    .dst_access_mask(
                        vk::AccessFlags2::SHADER_STORAGE_READ
                            | vk::AccessFlags2::SHADER_STORAGE_WRITE,
                    )
                    .buffer(values.buffer)
                    .size(BUFFER_BYTES),
            ];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().buffer_memory_barriers(&host_to_shader),
            );

            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
            device.cmd_bind_resource_heap_ext(
                command_buffer,
                &layout.bind_info(buffer_device_address(device, heap.buffer))?,
            );
            device.cmd_dispatch(command_buffer, 1, 1, 1);

            let shader_to_host = [vk::BufferMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::HOST)
                .dst_access_mask(vk::AccessFlags2::HOST_READ)
                .buffer(values.buffer)
                .size(BUFFER_BYTES)];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().buffer_memory_barriers(&shader_to_host),
            );
            Ok(())
        })?;

        let results = values
            .bytes()
            .chunks_exact(4)
            .map(|word| u32::from_ne_bytes([word[0], word[1], word[2], word[3]]))
            .collect::<Vec<_>>();
        for (index, value) in results.iter().copied().enumerate() {
            let expected = initial_value(index) * 2 + 1;
            if value != expected {
                bail!("compute output mismatch at {index}: got {value}, expected {expected}");
            }
        }
        let path = output_path("compute_buffer.ppm")?;
        write_image(&path, &results)?;
        println!("all {ELEMENTS} values match; wrote {}", path.display());

        device.destroy_pipeline(pipeline, None);
        heap.destroy(device, &mut allocator)?;
        values.destroy(device, &mut allocator)?;
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

fn initial_value(index: usize) -> u32 {
    index as u32 * 3 + 11
}

/// One column of pixels per value, colored by the value.
fn write_image(path: &std::path::Path, values: &[u32]) -> Result<()> {
    const ROWS: u32 = 16;
    let color = |value: u32| {
        let r = (value & 255) as u8;
        let g = ((value * 3) & 255) as u8;
        [r, g, 255 - r]
    };
    write_ppm(
        path,
        values.len() as u32,
        ROWS,
        (0..ROWS).flat_map(|_| values.iter().map(|&value| color(value))),
    )
}
