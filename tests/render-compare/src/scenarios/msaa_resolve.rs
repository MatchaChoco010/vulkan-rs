use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, bytes_of, color_clear, image_desc, rect,
};

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices: [f32; 45] = [
        -0.9, -0.8, 1.0, 0.0, 0.0, 0.7, -0.6, 1.0, 0.0, 0.0, -0.3, 0.9, 1.0, 0.0, 0.0, 0.95, 0.1,
        0.0, 1.0, 0.0, 0.2, 0.95, 0.0, 1.0, 0.0, -0.1, -0.2, 0.0, 1.0, 0.0, -0.95, 0.7, 0.0, 0.0,
        1.0, 0.9, 0.85, 0.0, 0.0, 1.0, 0.9, 0.8, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&vertices));
    let mut multisampled_desc = image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
        vk::ImageAspectFlags::COLOR,
    );
    multisampled_desc.info = multisampled_desc.info.samples(vk::SampleCountFlagBits::_4);
    let multisampled = ctx.image_create(&multisampled_desc);
    let resolved = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let attachments = [
            vk::AttachmentDescription::default()
                .format(format)
                .samples(vk::SampleCountFlagBits::_4)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::DONT_CARE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL),
            vk::AttachmentDescription::default()
                .format(format)
                .samples(vk::SampleCountFlagBits::_1)
                .load_op(vk::AttachmentLoadOp::DONT_CARE)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL),
        ];
        let color_references = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let resolve_references = [vk::AttachmentReference::default()
            .attachment(1)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpasses = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_references)
            .resolve_attachments(&resolve_references)];
        let dependencies = [vk::SubpassDependency::default()
            .src_subpass(0)
            .dst_subpass(vk::SUBPASS_EXTERNAL)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
            .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .dst_access_mask(vk::AccessFlags::TRANSFER_READ)];
        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        let render_pass = ctx
            .device
            .create_render_pass(&render_pass_info, None)
            .check("vkCreateRenderPass");
        let views = [multisampled.view, resolved.view];
        let framebuffer_info = vk::FramebufferCreateInfo::default()
            .render_pass(render_pass)
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);
        let framebuffer = ctx
            .device
            .create_framebuffer(&framebuffer_info, None)
            .check("vkCreateFramebuffer");

        let layout = ctx.pipeline_layout(&[], &[]);
        let vertex = ctx.shader_module("samples_hello_triangle_1_3.vert.spv");
        let fragment = ctx.shader_module("samples_hello_triangle_1_3.frag.spv");
        let bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(20)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let attributes = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32_SFLOAT)
                .offset(8),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            samples: vk::SampleCountFlagBits::_4,
            blend_attachments: &[BLEND_DISABLED],
            render_pass,
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        let clear_values = [color_clear([0.0, 0.0, 0.0, 1.0])];
        let begin = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(rect(width, height))
            .clear_values(&clear_values);
        ctx.device
            .cmd_begin_render_pass(command_buffer, &begin, vk::SubpassContents::INLINE);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_draw(command_buffer, 9, 1, 0, 0);
        ctx.device.cmd_end_render_pass(command_buffer);
        ctx.commands_submit(command_buffer);
        ctx.readback(
            resolved.image,
            vk::ImageAspectFlags::COLOR,
            width,
            height,
            4,
            "color",
            "rgba8",
        );

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_framebuffer(framebuffer, None);
        ctx.device.destroy_render_pass(render_pass, None);
    }
    ctx.image_destroy(&resolved);
    ctx.image_destroy(&multisampled);
    ctx.buffer_destroy(&vertex_buffer);
}
