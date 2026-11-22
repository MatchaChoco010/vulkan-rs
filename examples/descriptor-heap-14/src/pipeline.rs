use anyhow::Result;
use std::mem::{offset_of, size_of};
use vulkan::vk;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

use crate::scene::{Instance, Vertex};
use crate::{COLOR_FORMAT, DEPTH_FORMAT, EXTENT};

/// A graphics pipeline without a pipeline layout and without mappings: the fragment
/// shader reaches its descriptors through the heaps by itself.
pub unsafe fn create_pipeline(gpu: &Gpu) -> Result<Owned<vk::Pipeline>> {
    unsafe {
        let vertex = create_shader_module(gpu, include_bytes!("shaders/vertex.spv"))?;
        let fragment = create_shader_module(gpu, include_bytes!("shaders/fragment.spv"))?;
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::VERTEX)
                .module(*vertex)
                .name(c"main"),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .module(*fragment)
                .name(c"main"),
        ];

        // Binding 0 is the sphere mesh, binding 1 the instances.
        let bindings = [
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(size_of::<Vertex>() as u32)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(size_of::<Instance>() as u32)
                .input_rate(vk::VertexInputRate::INSTANCE),
        ];
        let attribute = |location: u32, binding: u32, format, offset: usize| {
            vk::VertexInputAttributeDescription::default()
                .location(location)
                .binding(binding)
                .format(format)
                .offset(offset as u32)
        };
        let attributes = [
            attribute(
                0,
                0,
                vk::Format::R32G32B32_SFLOAT,
                offset_of!(Vertex, position),
            ),
            attribute(
                1,
                0,
                vk::Format::R32G32B32_SFLOAT,
                offset_of!(Vertex, normal),
            ),
            attribute(2, 0, vk::Format::R32G32_SFLOAT, offset_of!(Vertex, uv)),
            attribute(
                3,
                1,
                vk::Format::R32G32B32A32_SFLOAT,
                offset_of!(Instance, translation_scale),
            ),
            attribute(
                4,
                1,
                vk::Format::R32_UINT,
                offset_of!(Instance, material_index),
            ),
        ];
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);

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
        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS);
        let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )];
        let color_blend_state =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let color_formats = [COLOR_FORMAT];
        let mut rendering = vk::PipelineRenderingCreateInfo::default()
            .color_attachment_formats(&color_formats)
            .depth_attachment_format(DEPTH_FORMAT);
        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .depth_stencil_state(&depth_stencil_state)
            .color_blend_state(&color_blend_state)
            .push_next(&mut rendering)
            .push_next(&mut flags);
        let result = gpu
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        single_pipeline(gpu, result)
    }
}
