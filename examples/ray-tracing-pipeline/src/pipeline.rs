use anyhow::{Context, Result};
use vulkan::{Device, vk};
use vulkan_alloc::descriptor_heap::DescriptorSlots;
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

/// The shader groups: ray generation, miss, and a hit group with the closest hit
/// shader, in the order that the shader binding table holds their records.
pub const GROUP_COUNT: u32 = 3;

/// A ray tracing pipeline without a pipeline layout. `slots` is where, in the resource
/// heap, the descriptors of bindings 0 and 1 are.
pub unsafe fn create_pipeline(device: &Device, slots: &DescriptorSlots) -> Result<vk::Pipeline> {
    unsafe {
        let raygen = create_shader_module(device, include_bytes!("shaders/raygen.spv"))?;
        let miss = create_shader_module(device, include_bytes!("shaders/miss.spv"))?;
        let closest_hit = create_shader_module(device, include_bytes!("shaders/closesthit.spv"))?;

        // Only the ray generation shader uses resources. Each of its bindings is the
        // descriptor at a fixed byte offset of the resource heap.
        let mapping = |binding: u32, resource: vk::SpirvResourceTypeFlagBitsEXT| {
            let offset = slots
                .offset(binding)
                .with_context(|| format!("the resource heap has no slot {binding}"))?;
            Ok::<_, anyhow::Error>(
                vk::DescriptorSetAndBindingMappingEXT::default()
                    .descriptor_set(0)
                    .first_binding(binding)
                    .binding_count(1)
                    .resource_mask(resource.into())
                    .source(vk::DescriptorMappingSourceEXT::HEAP_WITH_CONSTANT_OFFSET)
                    .source_data(vk::DescriptorMappingSourceDataEXT::constant_offset(
                        vk::DescriptorMappingSourceConstantOffsetEXT::default()
                            .heap_offset(u32::try_from(offset)?),
                    )),
            )
        };
        let mappings = [
            mapping(0, vk::SpirvResourceTypeFlagBitsEXT::ACCELERATION_STRUCTURE)?,
            mapping(
                1,
                vk::SpirvResourceTypeFlagBitsEXT::READ_WRITE_STORAGE_BUFFER,
            )?,
        ];
        let mut raygen_mappings =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::RAYGEN_KHR)
                .module(raygen)
                .name(c"raygenMain")
                .push_next(&mut raygen_mappings),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::MISS_KHR)
                .module(miss)
                .name(c"missMain"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::CLOSEST_HIT_KHR)
                .module(closest_hit)
                .name(c"closestHitMain"),
        ];

        // Each group names its shaders by their index in `stages`.
        let group = |ty, general, closest_hit| {
            vk::RayTracingShaderGroupCreateInfoKHR::default()
                .ty(ty)
                .general_shader(general)
                .closest_hit_shader(closest_hit)
                .any_hit_shader(vk::SHADER_UNUSED_KHR)
                .intersection_shader(vk::SHADER_UNUSED_KHR)
        };
        let groups: [_; GROUP_COUNT as usize] = [
            group(
                vk::RayTracingShaderGroupTypeKHR::GENERAL,
                0,
                vk::SHADER_UNUSED_KHR,
            ),
            group(
                vk::RayTracingShaderGroupTypeKHR::GENERAL,
                1,
                vk::SHADER_UNUSED_KHR,
            ),
            group(
                vk::RayTracingShaderGroupTypeKHR::TRIANGLES_HIT_GROUP,
                vk::SHADER_UNUSED_KHR,
                2,
            ),
        ];

        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::RayTracingPipelineCreateInfoKHR::default()
            .stages(&stages)
            .groups(&groups)
            .max_pipeline_ray_recursion_depth(1)
            .push_next(&mut flags);
        let mut pipelines = [vk::Pipeline::null()];
        let result = device.create_ray_tracing_pipelines_khr(
            vk::DeferredOperationKHR::null(),
            vk::PipelineCache::null(),
            &[info],
            None,
            &mut pipelines,
        );
        for module in [closest_hit, miss, raygen] {
            device.destroy_shader_module(module, None);
        }
        let result = match result {
            Ok(vk::VkResult::SUCCESS) => Ok(pipelines.to_vec()),
            Ok(code) | Err(code) => Err((pipelines.to_vec(), code)),
        };
        single_pipeline(device, result)
    }
}
