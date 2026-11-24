// Based on dEQP-VK.api.smoke.triangle of the Khronos Vulkan Conformance Test Suite
// (https://github.com/KhronosGroup/VK-GL-CTS,
// external/vulkancts/modules/vulkan/api/vktApiSmokeTests.cpp, renderTriangleTest).
//
// Copyright (c) 2015 Google Inc.
// SPDX-License-Identifier: Apache-2.0
//
// The Rust counterpart of c/cts_smoke_triangle.c, which describes what is taken from the
// test and what is changed.

use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, bytes_of, color_clear, image_barrier, image_desc,
    rect,
};

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices: [f32; 12] = [
        -0.5, -0.5, 0.0, 1.0, 0.5, -0.5, 0.0, 1.0, 0.0, 0.5, 0.0, 1.0,
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
        let attachments = [vk::AttachmentDescription::default()
            .format(format)
            .samples(vk::SampleCountFlagBits::_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let color_references = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpasses = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_references)];
        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses);
        let render_pass = ctx
            .device
            .create_render_pass(&render_pass_info, None)
            .check("vkCreateRenderPass");

        let views = [color.view];
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
        let vertex = ctx.shader_module("cts_smoke_triangle.vert.spv");
        let fragment = ctx.shader_module("cts_smoke_triangle.frag.spv");
        let bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(16)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let attributes = [vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(0)];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            blend_attachments: &[BLEND_DISABLED],
            render_pass,
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        let clear_values = [color_clear([0.125, 0.25, 0.75, 1.0])];
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
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        ctx.device.cmd_end_render_pass(command_buffer);
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
        ctx.device.destroy_framebuffer(framebuffer, None);
        ctx.device.destroy_render_pass(render_pass, None);
    }
    ctx.image_destroy(&color);
    ctx.buffer_destroy(&vertex_buffer);
}
