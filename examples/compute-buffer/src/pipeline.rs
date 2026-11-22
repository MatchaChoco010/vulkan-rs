use anyhow::Result;
use vulkan::vk;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

/// A compute pipeline without a pipeline layout. `heap_offset` is where, in the resource
/// heap, the descriptor for the shader's `set = 0, binding = 0` is.
pub unsafe fn create_pipeline(gpu: &Gpu, heap_offset: u32) -> Result<Owned<vk::Pipeline>> {
    unsafe {
        let module = create_shader_module(gpu, include_bytes!("shaders/compute.spv"))?;

        let mappings = [vk::DescriptorSetAndBindingMappingEXT::default()
            .descriptor_set(0)
            .first_binding(0)
            .binding_count(1)
            .resource_mask(vk::SpirvResourceTypeFlagBitsEXT::READ_WRITE_STORAGE_BUFFER.into())
            .source(vk::DescriptorMappingSourceEXT::HEAP_WITH_CONSTANT_OFFSET)
            .source_data(vk::DescriptorMappingSourceDataEXT::constant_offset(
                vk::DescriptorMappingSourceConstantOffsetEXT::default().heap_offset(heap_offset),
            ))];
        let mut mapping_info =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::COMPUTE)
            .module(*module)
            .name(c"main")
            .push_next(&mut mapping_info);

        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .push_next(&mut flags);
        let result = gpu
            .device
            .create_compute_pipelines(vk::PipelineCache::null(), &[info], None);
        single_pipeline(gpu, result)
    }
}
