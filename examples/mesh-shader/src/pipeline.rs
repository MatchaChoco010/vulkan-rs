use anyhow::Result;
use std::mem::offset_of;
use vulkan::{Device, vk};
use vulkan_alloc::descriptor_heap::DescriptorSlots;
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

use crate::{EXTENT, FORMAT, PushData};

/// A graphics pipeline without a pipeline layout. The mappings connect the bindings of
/// the shaders to the heap indices in [`PushData`].
pub unsafe fn create_pipeline(device: &Device, slots: &DescriptorSlots) -> Result<vk::Pipeline> {
    unsafe {
        let task = create_shader_module(device, include_bytes!("shaders/task.spv"))?;
        let mesh = create_shader_module(device, include_bytes!("shaders/mesh.spv"))?;
        let fragment = create_shader_module(device, include_bytes!("shaders/fragment.spv"))?;

        // Binding `b` of set 0 is the descriptor at `index * stride` in the resource
        // heap, where `index` is the u32 at `push_offset` of the push data. The stride
        // is the distance between two slots of the run that `ResourceHeapPlan` laid out.
        let stride = u32::try_from(slots.stride)?;
        let mapping = |binding: u32, push_offset: usize| {
            vk::DescriptorSetAndBindingMappingEXT::default()
                .descriptor_set(0)
                .first_binding(binding)
                .binding_count(1)
                .resource_mask(vk::SpirvResourceTypeFlagBitsEXT::READ_ONLY_STORAGE_BUFFER.into())
                .source(vk::DescriptorMappingSourceEXT::HEAP_WITH_PUSH_INDEX)
                .source_data(vk::DescriptorMappingSourceDataEXT::push_index(
                    vk::DescriptorMappingSourcePushIndexEXT::default()
                        .push_offset(push_offset as u32)
                        .heap_index_stride(stride),
                ))
        };
        let mappings = [
            mapping(0, offset_of!(PushData, vertices)),
            mapping(1, offset_of!(PushData, indices)),
            mapping(2, offset_of!(PushData, draws)),
        ];
        // A stage ignores the mappings for bindings it does not use, so both stages can
        // take the same list. Each needs its own struct in its own chain.
        let mut task_mappings =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let mut mesh_mappings =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::TASK_EXT)
                .module(task)
                .name(c"main")
                .push_next(&mut task_mappings),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::MESH_EXT)
                .module(mesh)
                .name(c"main")
                .push_next(&mut mesh_mappings),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .module(fragment)
                .name(c"main"),
        ];

        // A negative height flips the image so that +Y is up.
        let viewports = [vk::Viewport::default()
            .y(EXTENT.height as f32)
            .width(EXTENT.width as f32)
            .height(-(EXTENT.height as f32))
            .max_depth(1.0)];
        let scissors = [vk::Rect2D::default().extent(EXTENT)];
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlagBits::_1);
        let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )];
        let color_blend_state =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let formats = [FORMAT];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .push_next(&mut rendering)
            .push_next(&mut flags);
        let result = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        for module in [fragment, mesh, task] {
            device.destroy_shader_module(module, None);
        }
        single_pipeline(device, result)
    }
}
