// Based on the hello_triangle_1_3 sample of Khronos Vulkan-Samples
// (https://github.com/KhronosGroup/Vulkan-Samples, samples/api/hello_triangle_1_3).
//
// Copyright (c) 2024-2026, Huawei Technologies Co., Ltd.
// SPDX-License-Identifier: Apache-2.0
//
// The Rust counterpart of c/samples_hello_triangle_1_3.c, which describes what is taken
// from the sample and what is changed.

use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, GraphicsDesc, bytes_of, color_clear, image_barrier, image_desc, rect,
};

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_SRGB;
    let vertices: [f32; 15] = [
        0.5, -0.5, 1.0, 0.0, 0.0, 0.5, 0.5, 0.0, 1.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0,
    ];

    let vertex_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&vertices));
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
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
        let dynamic_states = [
            vk::DynamicState::VIEWPORT,
            vk::DynamicState::SCISSOR,
            vk::DynamicState::CULL_MODE,
            vk::DynamicState::FRONT_FACE,
            vk::DynamicState::PRIMITIVE_TOPOLOGY,
        ];
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            blend_attachments: &[BLEND_DISABLED],
            dynamic_states: &dynamic_states,
            rendering: Some(&mut rendering),
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        image_barrier(
            &ctx.device,
            command_buffer,
            color.image,
            vk::ImageAspectFlags::COLOR,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            (vk::PipelineStageFlags2::TOP_OF_PIPE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            ),
        );
        let color_attachments = [vk::RenderingAttachmentInfo::default()
            .image_view(color.view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(color_clear([0.01, 0.01, 0.033, 1.0]))];
        let rendering_info = vk::RenderingInfo::default()
            .render_area(rect(width, height))
            .layer_count(1)
            .color_attachments(&color_attachments);
        ctx.device
            .cmd_begin_rendering(command_buffer, &rendering_info);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        let viewports = [vk::Viewport::default()
            .width(width as f32)
            .height(height as f32)
            .max_depth(1.0)];
        ctx.device.cmd_set_viewport(command_buffer, 0, &viewports);
        ctx.device
            .cmd_set_scissor(command_buffer, 0, &[rect(width, height)]);
        ctx.device
            .cmd_set_cull_mode(command_buffer, vk::CullModeFlags::NONE);
        ctx.device
            .cmd_set_front_face(command_buffer, vk::FrontFace::CLOCKWISE);
        ctx.device
            .cmd_set_primitive_topology(command_buffer, vk::PrimitiveTopology::TRIANGLE_LIST);
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        ctx.device.cmd_end_rendering(command_buffer);
        image_barrier(
            &ctx.device,
            command_buffer,
            color.image,
            vk::ImageAspectFlags::COLOR,
            0,
            1,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            (
                vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
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

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
    }
    ctx.image_destroy(&color);
    ctx.buffer_destroy(&vertex_buffer);
}
