use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, GraphicsDesc, bytes_of, color_attachment_begin, color_attachment_end,
    color_clear, image_barrier, image_desc, rect,
};

fn depth_stencil_state(
    depth_test: bool,
    depth_write: bool,
    stencil_test: bool,
    stencil_compare: vk::CompareOp,
    stencil_pass: vk::StencilOp,
) -> vk::PipelineDepthStencilStateCreateInfo<'static> {
    let stencil = vk::StencilOpState::default()
        .fail_op(vk::StencilOp::KEEP)
        .pass_op(stencil_pass)
        .depth_fail_op(vk::StencilOp::KEEP)
        .compare_op(stencil_compare)
        .compare_mask(0xff)
        .write_mask(0xff)
        .reference(0);
    vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(depth_test)
        .depth_write_enable(depth_write)
        .depth_compare_op(vk::CompareOp::LESS)
        .stencil_test_enable(stencil_test)
        .front(stencil)
        .back(stencil)
        .max_depth_bounds(1.0)
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let color_format = vk::Format::R8G8B8A8_UNORM;
    let depth_format = vk::Format::D32_SFLOAT_S8_UINT;
    let depth_stencil_aspect = vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL;
    let vertices: [f32; 54] = [
        -0.75, -0.75, 0.5, 0.25, -0.75, 0.5, 0.25, 0.75, 0.5, 0.25, 0.75, 0.5, -0.75, 0.75, 0.5,
        -0.75, -0.75, 0.5, -0.5, -0.5, 0.3, 0.5, -0.5, 0.7, 0.5, 0.5, 0.7, 0.5, 0.5, 0.7, -0.5,
        0.5, 0.3, -0.5, -0.5, 0.3, -1.0, 0.25, 0.0, 1.0, 0.25, 0.0, 1.0, 0.5, 0.0, 1.0, 0.5, 0.0,
        -1.0, 0.5, 0.0, -1.0, 0.25, 0.0,
    ];

    let vertex_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&vertices));
    let color = ctx.image_create(&image_desc(
        color_format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));
    let depth = ctx.image_create(&image_desc(
        depth_format,
        width,
        height,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        depth_stencil_aspect,
    ));

    unsafe {
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .offset(0)
            .size(16)];
        let layout = ctx.pipeline_layout(&[], &ranges);
        let vertex = ctx.shader_module("depth_stencil.vert.spv");
        let fragment = ctx.shader_module("depth_stencil.frag.spv");
        let bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(12)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let attributes = [vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0)];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let dynamic_states = [vk::DynamicState::STENCIL_REFERENCE];
        let color_formats = [color_format];
        let states = [
            depth_stencil_state(
                true,
                true,
                true,
                vk::CompareOp::ALWAYS,
                vk::StencilOp::REPLACE,
            ),
            depth_stencil_state(
                true,
                true,
                false,
                vk::CompareOp::ALWAYS,
                vk::StencilOp::KEEP,
            ),
            depth_stencil_state(
                false,
                false,
                true,
                vk::CompareOp::EQUAL,
                vk::StencilOp::KEEP,
            ),
        ];
        let pipelines = states
            .iter()
            .map(|state| {
                let mut rendering = vk::PipelineRenderingCreateInfo::default()
                    .color_attachment_formats(&color_formats)
                    .depth_attachment_format(depth_format)
                    .stencil_attachment_format(depth_format);
                ctx.graphics_pipeline(GraphicsDesc {
                    vertex_input: Some(&vertex_input),
                    depth_stencil: Some(state),
                    blend_attachments: &[BLEND_DISABLED],
                    dynamic_states: &dynamic_states,
                    rendering: Some(&mut rendering),
                    ..GraphicsDesc::new(vertex, fragment, layout, width, height)
                })
            })
            .collect::<Vec<_>>();
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        image_barrier(
            &ctx.device,
            command_buffer,
            depth.image,
            depth_stencil_aspect,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_READ
                    | vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ),
        );
        let color_attachments = [vk::RenderingAttachmentInfo::default()
            .image_view(color.view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(color_clear([16.0 / 255.0, 16.0 / 255.0, 16.0 / 255.0, 1.0]))];
        let depth_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(depth.view)
            .image_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue::depth_stencil(vk::ClearDepthStencilValue {
                depth: 1.0,
                stencil: 0,
            }));
        let rendering_info = vk::RenderingInfo::default()
            .render_area(rect(width, height))
            .layer_count(1)
            .color_attachments(&color_attachments)
            .depth_attachment(&depth_attachment)
            .stencil_attachment(&depth_attachment);
        ctx.device
            .cmd_begin_rendering(command_buffer, &rendering_info);
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_set_stencil_reference(
            command_buffer,
            vk::StencilFaceFlags::FRONT_AND_BACK,
            1,
        );
        let colors: [[f32; 4]; 3] = [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ];
        for (index, (&pipeline, color)) in pipelines.iter().zip(&colors).enumerate() {
            ctx.device
                .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            ctx.device.cmd_push_constants(
                command_buffer,
                layout,
                vk::ShaderStageFlags::FRAGMENT,
                0,
                bytes_of(color),
            );
            ctx.device
                .cmd_draw(command_buffer, 6, 1, 6 * index as u32, 0);
        }
        ctx.device.cmd_end_rendering(command_buffer);
        color_attachment_end(&ctx.device, command_buffer, color.image);
        image_barrier(
            &ctx.device,
            command_buffer,
            depth.image,
            depth_stencil_aspect,
            0,
            1,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            (
                vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ),
            (
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
            ),
        );
        ctx.commands_submit(command_buffer);
        ctx.readback(
            color.image,
            vk::ImageAspectFlags::COLOR,
            width,
            height,
            4,
            "color",
            "rgba8",
        );
        ctx.readback(
            depth.image,
            vk::ImageAspectFlags::DEPTH,
            width,
            height,
            4,
            "depth",
            "d32f",
        );
        ctx.readback(
            depth.image,
            vk::ImageAspectFlags::STENCIL,
            width,
            height,
            1,
            "stencil",
            "s8",
        );

        for pipeline in pipelines {
            ctx.device.destroy_pipeline(pipeline, None);
        }
        ctx.device.destroy_pipeline_layout(layout, None);
    }
    ctx.image_destroy(&depth);
    ctx.image_destroy(&color);
    ctx.buffer_destroy(&vertex_buffer);
}
