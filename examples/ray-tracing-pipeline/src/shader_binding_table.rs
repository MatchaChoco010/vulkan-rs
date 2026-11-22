use anyhow::{Context, Result, bail};
use vulkan::vk;
use vulkan_alloc::MemoryLocation;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::memory::{
    Buffer, buffer_device_address, create_aligned_buffer, write_buffer,
};

use crate::pipeline::GROUP_COUNT;

/// One record for each shader group of the pipeline, each record a region of its own:
/// the ray generation shader, the miss shader and the hit group. No record holds more
/// than the group handle, since the shaders take no parameters from the table.
pub struct ShaderBindingTable {
    _buffer: Owned<Buffer>,
    pub raygen: vk::StridedDeviceAddressRegionKHR,
    pub miss: vk::StridedDeviceAddressRegionKHR,
    pub hit: vk::StridedDeviceAddressRegionKHR,
    /// The pipeline has no callable shaders.
    pub callable: vk::StridedDeviceAddressRegionKHR,
}

impl ShaderBindingTable {
    pub unsafe fn new(
        gpu: &Gpu,
        pipeline: vk::Pipeline,
        properties: &vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'_>,
    ) -> Result<Self> {
        unsafe {
            let handle_size = properties.shader_group_handle_size as usize;
            // A region has to start at a multiple of the base alignment.
            let base_alignment = u64::from(properties.shader_group_base_alignment);
            let record_stride = (handle_size as u64)
                .checked_next_multiple_of(base_alignment)
                .context("the shader group base alignment is zero")?;

            let mut handles = vec![0; handle_size * GROUP_COUNT as usize];
            gpu.device.get_ray_tracing_shader_group_handles_khr(
                pipeline,
                0,
                GROUP_COUNT,
                &mut handles,
            )?;
            let mut table = vec![0; record_stride as usize * GROUP_COUNT as usize];
            for (record, handle) in table
                .chunks_exact_mut(record_stride as usize)
                .zip(handles.chunks_exact(handle_size))
            {
                record[..handle_size].copy_from_slice(handle);
            }

            let buffer = create_aligned_buffer(
                gpu,
                "shader binding table",
                table.len() as u64,
                vk::BufferUsageFlags::SHADER_BINDING_TABLE_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                MemoryLocation::CpuToGpu,
                base_alignment,
            )?;
            let address = buffer_device_address(gpu, buffer.buffer);
            if !address.is_multiple_of(base_alignment) {
                bail!(
                    "shader binding table address {address:#x} is not aligned to {base_alignment}"
                );
            }
            write_buffer(gpu, &buffer, &table)?;

            let region = |record: u64| {
                vk::StridedDeviceAddressRegionKHR::default()
                    .device_address(address + record * record_stride)
                    .stride(record_stride)
                    .size(record_stride)
            };
            Ok(Self {
                _buffer: buffer,
                raygen: region(0),
                miss: region(1),
                hit: region(2),
                callable: vk::StridedDeviceAddressRegionKHR::default(),
            })
        }
    }
}
