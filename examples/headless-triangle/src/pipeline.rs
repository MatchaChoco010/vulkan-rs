use anyhow::Result;
use std::mem::{offset_of, size_of};
use vulkan::vk;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::pipeline::single_pipeline;
use vulkan_example_common::shader::create_shader_module;

/// The SPIR-V of the shaders, compiled from `shaders/triangle.slang`.
pub const VERTEX_SHADER: &[u8] = include_bytes!("shaders/triangle.vert.spv");
pub const FRAGMENT_SHADER: &[u8] = include_bytes!("shaders/triangle.frag.spv");

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

/// One color attachment that is cleared, drawn to, and left in `TRANSFER_SRC_OPTIMAL`
/// for the copy to the host.
pub unsafe fn create_render_pass(gpu: &Gpu, format: vk::Format) -> Result<Owned<vk::RenderPass>> {
    let attachments = [vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlagBits::_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)];
    let color_attachments = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpasses = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&color_attachments)];
    // The copy after the render pass reads what the subpass wrote.
    let dependencies = [vk::SubpassDependency::default()
        .src_subpass(0)
        .dst_subpass(vk::SUBPASS_EXTERNAL)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_access_mask(vk::AccessFlags::TRANSFER_READ)];
    let info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(&subpasses)
        .dependencies(&dependencies);
    unsafe { Ok(gpu.own(gpu.device.create_render_pass(&info, None)?)) }
}

pub unsafe fn create_framebuffer(
    gpu: &Gpu,
    render_pass: vk::RenderPass,
    image_view: vk::ImageView,
    extent: vk::Extent2D,
) -> Result<Owned<vk::Framebuffer>> {
    let attachments = [image_view];
    let info = vk::FramebufferCreateInfo::default()
        .render_pass(render_pass)
        .attachments(&attachments)
        .width(extent.width)
        .height(extent.height)
        .layers(1);
    unsafe { Ok(gpu.own(gpu.device.create_framebuffer(&info, None)?)) }
}

/// A pipeline that reads [`Vertex`] from vertex buffer binding 0 and draws to the whole
/// of an `extent`-sized framebuffer.
pub unsafe fn create_pipeline(
    gpu: &Gpu,
    render_pass: vk::RenderPass,
    layout: vk::PipelineLayout,
    extent: vk::Extent2D,
) -> Result<Owned<vk::Pipeline>> {
    unsafe {
        let vertex = create_shader_module(gpu, VERTEX_SHADER)?;
        let fragment = create_shader_module(gpu, FRAGMENT_SHADER)?;
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
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(layout)
            .render_pass(render_pass);
        let result = gpu
            .device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        single_pipeline(gpu, result)
    }
}
