use vulkan::vk;

use crate::render::{
    Ctx, GraphicsDesc, bytes_of, color_attachment_begin, color_attachment_end, image_desc,
    rendering_begin,
};

fn blend_mode(mode: u32) -> vk::PipelineColorBlendAttachmentState {
    let all = vk::ColorComponentFlags::R
        | vk::ColorComponentFlags::G
        | vk::ColorComponentFlags::B
        | vk::ColorComponentFlags::A;
    let state = vk::PipelineColorBlendAttachmentState::default()
        .blend_enable(true)
        .src_color_blend_factor(vk::BlendFactor::ONE)
        .dst_color_blend_factor(vk::BlendFactor::ONE)
        .color_blend_op(vk::BlendOp::ADD)
        .src_alpha_blend_factor(vk::BlendFactor::ONE)
        .dst_alpha_blend_factor(vk::BlendFactor::ONE)
        .alpha_blend_op(vk::BlendOp::ADD)
        .color_write_mask(all);
    match mode {
        0 => state
            .src_color_blend_factor(vk::BlendFactor::SRC_ALPHA)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_SRC_ALPHA),
        1 => state,
        2 => state
            .src_color_blend_factor(vk::BlendFactor::DST_COLOR)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .src_alpha_blend_factor(vk::BlendFactor::DST_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO),
        3 => state
            .src_color_blend_factor(vk::BlendFactor::CONSTANT_COLOR)
            .dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_CONSTANT_COLOR)
            .src_alpha_blend_factor(vk::BlendFactor::CONSTANT_ALPHA)
            .dst_alpha_blend_factor(vk::BlendFactor::ONE_MINUS_CONSTANT_ALPHA),
        4 => state
            .color_blend_op(vk::BlendOp::MIN)
            .alpha_blend_op(vk::BlendOp::MIN),
        5 => state
            .color_blend_op(vk::BlendOp::MAX)
            .alpha_blend_op(vk::BlendOp::MAX),
        6 => state
            .color_blend_op(vk::BlendOp::REVERSE_SUBTRACT)
            .alpha_blend_op(vk::BlendOp::REVERSE_SUBTRACT),
        _ => state
            .blend_enable(false)
            .color_write_mask(vk::ColorComponentFlags::R | vk::ColorComponentFlags::A),
    }
}

fn ndc(pixel: u32) -> f32 {
    pixel as f32 / 128.0 - 1.0
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(stages)
            .offset(0)
            .size(32)];
        let layout = ctx.pipeline_layout(&[], &ranges);
        let vertex = ctx.shader_module("blend.vert.spv");
        let fragment = ctx.shader_module("blend.frag.spv");
        let dynamic_states = [vk::DynamicState::BLEND_CONSTANTS];
        let formats = [format];
        let pipelines = (0..8)
            .map(|mode| {
                let blend = [blend_mode(mode)];
                let mut rendering =
                    vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
                ctx.graphics_pipeline(GraphicsDesc {
                    blend_attachments: &blend,
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
        let clear = [64.0 / 255.0, 96.0 / 255.0, 128.0 / 255.0, 1.0];
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            clear,
            width,
            height,
        );
        ctx.device
            .cmd_set_blend_constants(command_buffer, &[0.25, 0.5, 0.75, 0.5]);
        for (cell, &pipeline) in pipelines.iter().enumerate() {
            let cell = cell as u32;
            let x = 64 * (cell % 4);
            let y = 128 * (cell / 4);
            ctx.device
                .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            let first: [f32; 8] = [
                200.0 / 255.0,
                40.0 / 255.0,
                40.0 / 255.0,
                153.0 / 255.0,
                ndc(x + 8),
                ndc(y + 8),
                ndc(x + 40),
                ndc(y + 88),
            ];
            ctx.device
                .cmd_push_constants(command_buffer, layout, stages, 0, bytes_of(&first));
            ctx.device.cmd_draw(command_buffer, 6, 1, 0, 0);
            let second: [f32; 8] = [
                30.0 / 255.0,
                180.0 / 255.0,
                220.0 / 255.0,
                102.0 / 255.0,
                ndc(x + 24),
                ndc(y + 40),
                ndc(x + 56),
                ndc(y + 120),
            ];
            ctx.device
                .cmd_push_constants(command_buffer, layout, stages, 0, bytes_of(&second));
            ctx.device.cmd_draw(command_buffer, 6, 1, 0, 0);
        }
        ctx.device.cmd_end_rendering(command_buffer);
        color_attachment_end(&ctx.device, command_buffer, color.image);
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

        for pipeline in pipelines {
            ctx.device.destroy_pipeline(pipeline, None);
        }
        ctx.device.destroy_pipeline_layout(layout, None);
    }
    ctx.image_destroy(&color);
}
