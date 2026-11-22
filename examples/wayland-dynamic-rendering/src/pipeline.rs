use anyhow::Result;
use std::mem::{offset_of, size_of};
use vulkan::vk;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

/// A pipeline that reads [`Vertex`] from vertex buffer binding 0 and draws to the whole
/// of an `extent`-sized color attachment in `format`.
///
/// Without a render pass, the pipeline is told the formats of the attachments it will
/// draw to with `VkPipelineRenderingCreateInfo`; `vkCmdBeginRendering` has to match them.
pub unsafe fn create_pipeline(
    gpu: &Gpu,
    format: vk::Format,
    layout: vk::PipelineLayout,
    extent: vk::Extent2D,
) -> Result<Owned<vk::Pipeline>> {
    unsafe {
        let vertex = create_shader_module(gpu, include_bytes!("shaders/triangle.vert.spv"))?;
        let fragment = create_shader_module(gpu, include_bytes!("shaders/triangle.frag.spv"))?;
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
        let vertex_bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let vertex_attributes = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(offset_of!(Vertex, position) as u32),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(offset_of!(Vertex, color) as u32),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&vertex_bindings)
            .vertex_attribute_descriptions(&vertex_attributes);
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewports = [vk::Viewport::default()
            .width(extent.width as f32)
            .height(extent.height as f32)
            .max_depth(1.0)];
        let scissors = [vk::Rect2D::default().extent(extent)];
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::NONE)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlagBits::_1);
        let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )];
        let color_blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(layout)
            .push_next(&mut rendering);
        let result = gpu
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        single_pipeline(gpu, result)
    }
}
