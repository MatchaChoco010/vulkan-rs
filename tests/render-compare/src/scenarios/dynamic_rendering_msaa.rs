use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, bytes_of, color_attachment_begin, color_attachment_end, color_clear,
    depth_stencil_clear, extent2d, extent3d, image_barrier, image_desc, rect, subresource_layers,
};

#[derive(Clone, Copy)]
#[repr(C)]
struct Push {
    color: [f32; 4],
    rect: [f32; 4],
    depth: f32,
}

fn ndc(pixel: u32) -> f32 {
    pixel as f32 / 128.0 - 1.0
}

fn push(color: [f32; 4], rect: [u32; 4], depth: f32) -> [Push; 1] {
    [Push {
        color,
        rect: rect.map(ndc),
        depth,
    }]
}

fn msaa_pipeline(
    ctx: &Ctx,
    vertex: vk::ShaderModule,
    fragment: vk::ShaderModule,
    layout: vk::PipelineLayout,
    color_format: vk::Format,
    depth_format: vk::Format,
    sample_mask: vk::SampleMask,
) -> vk::Pipeline {
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::VERTEX)
            .module(vertex)
            .name(c"main"),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::FRAGMENT)
            .module(fragment)
            .name(c"main"),
    ];
    let vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let viewport_state = vk::PipelineViewportStateCreateInfo::default();
    let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);
    let sample_masks = [sample_mask, 0];
    let multisample = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlagBits::_4)
        .sample_shading_enable(true)
        .min_sample_shading(1.0)
        .sample_mask(&sample_masks);
    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_compare_op(vk::CompareOp::ALWAYS)
        .max_depth_bounds(1.0);
    let blend_attachments = [BLEND_DISABLED];
    let color_blend =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
    let dynamic_states = [
        vk::DynamicState::VIEWPORT_WITH_COUNT,
        vk::DynamicState::SCISSOR_WITH_COUNT,
        vk::DynamicState::DEPTH_TEST_ENABLE,
        vk::DynamicState::DEPTH_WRITE_ENABLE,
        vk::DynamicState::DEPTH_COMPARE_OP,
        vk::DynamicState::DEPTH_BOUNDS_TEST_ENABLE,
        vk::DynamicState::STENCIL_TEST_ENABLE,
        vk::DynamicState::RASTERIZER_DISCARD_ENABLE,
        vk::DynamicState::DEPTH_BIAS_ENABLE,
        vk::DynamicState::PRIMITIVE_RESTART_ENABLE,
    ];
    let dynamic = vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
    let color_formats = [color_format];
    let mut rendering = vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(&color_formats)
        .depth_attachment_format(depth_format);
    let info = vk::GraphicsPipelineCreateInfo::default()
        .push_next(&mut rendering)
        .stages(&stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization)
        .multisample_state(&multisample)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blend)
        .dynamic_state(&dynamic)
        .layout(layout);
    match unsafe {
        ctx.device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)
    } {
        Ok(pipelines) => pipelines[0],
        Err((_, error)) => {
            crate::render::fail(&format!("vkCreateGraphicsPipelines returned {error}"))
        }
    }
}

fn draw_rect(
    ctx: &Ctx,
    command_buffer: vk::CommandBuffer,
    layout: vk::PipelineLayout,
    push: &[Push; 1],
) {
    unsafe {
        ctx.device.cmd_push_constants(
            command_buffer,
            layout,
            vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT,
            0,
            bytes_of(push),
        );
        ctx.device.cmd_draw(command_buffer, 6, 1, 0, 0);
    }
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let depth_format = vk::Format::D32_SFLOAT;
    let color_aspect = vk::ImageAspectFlags::COLOR;
    let depth_aspect = vk::ImageAspectFlags::DEPTH;

    let mut samples_desc = image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        color_aspect,
    );
    samples_desc.info = samples_desc.info.samples(vk::SampleCountFlagBits::_4);
    let samples = ctx.image_create(&samples_desc);
    let mut depth_samples_desc = image_desc(
        depth_format,
        width,
        height,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        depth_aspect,
    );
    depth_samples_desc.info = depth_samples_desc.info.samples(vk::SampleCountFlagBits::_4);
    let depth_samples = ctx.image_create(&depth_samples_desc);
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        color_aspect,
    ));
    let depth = ctx.image_create(&image_desc(
        depth_format,
        width,
        height,
        vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        depth_aspect,
    ));
    let resolved = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::SAMPLED,
        color_aspect,
    ));

    unsafe {
        let push_stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(push_stages)
            .offset(0)
            .size(size_of::<Push>() as u32)];
        let layout = ctx.pipeline_layout(&[], &ranges);
        let vertex = ctx.shader_module("dynamic_rendering_msaa.vert.spv");
        let fragment = ctx.shader_module("dynamic_rendering_msaa.frag.spv");
        let all_samples = msaa_pipeline(ctx, vertex, fragment, layout, format, depth_format, 0xf);
        let two_samples = msaa_pipeline(ctx, vertex, fragment, layout, format, depth_format, 0x5);
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, samples.image);
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        image_barrier(
            &ctx.device,
            command_buffer,
            depth_samples.image,
            depth_aspect,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS,
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ),
        );
        image_barrier(
            &ctx.device,
            command_buffer,
            depth.image,
            depth_aspect,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
            ),
        );
        let color_attachments = [vk::RenderingAttachmentInfo::default()
            .image_view(samples.view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .resolve_mode(vk::ResolveModeFlagBits::AVERAGE)
            .resolve_image_view(color.view)
            .resolve_image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(color_clear([32.0 / 255.0, 32.0 / 255.0, 48.0 / 255.0, 1.0]))];
        let depth_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(depth_samples.view)
            .image_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .resolve_mode(vk::ResolveModeFlagBits::SAMPLE_ZERO)
            .resolve_image_view(depth.view)
            .resolve_image_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .clear_value(depth_stencil_clear(1.0, 0));
        let rendering_info = vk::RenderingInfo::default()
            .render_area(rect(width, height))
            .layer_count(1)
            .color_attachments(&color_attachments)
            .depth_attachment(&depth_attachment);
        ctx.device
            .cmd_begin_rendering(command_buffer, &rendering_info);
        let viewports = [vk::Viewport::default()
            .width(width as f32)
            .height(height as f32)
            .max_depth(1.0)];
        ctx.device
            .cmd_set_viewport_with_count(command_buffer, &viewports);
        ctx.device
            .cmd_set_scissor_with_count(command_buffer, &[rect(width, height)]);
        ctx.device.cmd_set_depth_test_enable(command_buffer, true);
        ctx.device.cmd_set_depth_write_enable(command_buffer, true);
        ctx.device
            .cmd_set_depth_compare_op(command_buffer, vk::CompareOp::LESS);
        ctx.device
            .cmd_set_depth_bounds_test_enable(command_buffer, false);
        ctx.device
            .cmd_set_stencil_test_enable(command_buffer, false);
        ctx.device
            .cmd_set_rasterizer_discard_enable(command_buffer, false);
        ctx.device.cmd_set_depth_bias_enable(command_buffer, false);
        ctx.device
            .cmd_set_primitive_restart_enable(command_buffer, false);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, all_samples);
        let red = push([1.0, 0.0, 0.0, 1.0], [16, 16, 144, 144], 0.5);
        draw_rect(ctx, command_buffer, layout, &red);
        let green = push([0.0, 1.0, 0.0, 1.0], [80, 80, 208, 208], 0.75);
        draw_rect(ctx, command_buffer, layout, &green);
        ctx.device
            .cmd_set_rasterizer_discard_enable(command_buffer, true);
        let white = push([1.0, 1.0, 1.0, 1.0], [0, 0, 256, 256], 0.0);
        draw_rect(ctx, command_buffer, layout, &white);
        ctx.device
            .cmd_set_rasterizer_discard_enable(command_buffer, false);
        ctx.device
            .cmd_set_depth_compare_op(command_buffer, vk::CompareOp::GREATER);
        ctx.device.cmd_set_depth_write_enable(command_buffer, false);
        let blue = push([0.0, 0.0, 1.0, 1.0], [112, 48, 240, 112], 0.625);
        draw_rect(ctx, command_buffer, layout, &blue);
        let clears = [
            vk::ClearAttachment {
                aspect_mask: color_aspect,
                color_attachment: 0,
                clear_value: color_clear([1.0, 0.0, 1.0, 1.0]),
            },
            vk::ClearAttachment {
                aspect_mask: depth_aspect,
                color_attachment: 0,
                clear_value: depth_stencil_clear(0.25, 0),
            },
        ];
        let clear_rects = [vk::ClearRect {
            rect: vk::Rect2D {
                offset: vk::Offset2D { x: 176, y: 16 },
                extent: extent2d(64, 32),
            },
            base_array_layer: 0,
            layer_count: 1,
        }];
        ctx.device
            .cmd_clear_attachments(command_buffer, &clears, &clear_rects);
        ctx.device.cmd_set_depth_test_enable(command_buffer, false);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, two_samples);
        let yellow = push([1.0, 1.0, 0.0, 1.0], [16, 176, 80, 240], 0.0);
        draw_rect(ctx, command_buffer, layout, &yellow);
        ctx.device.cmd_end_rendering(command_buffer);

        image_barrier(
            &ctx.device,
            command_buffer,
            samples.image,
            color_aspect,
            0,
            1,
            vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            (
                vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            ),
            (
                vk::PipelineStageFlags2::RESOLVE,
                vk::AccessFlags2::TRANSFER_READ,
            ),
        );
        color_attachment_end(&ctx.device, command_buffer, color.image);
        image_barrier(
            &ctx.device,
            command_buffer,
            depth.image,
            depth_aspect,
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
        image_barrier(
            &ctx.device,
            command_buffer,
            resolved.image,
            color_aspect,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::RESOLVE,
                vk::AccessFlags2::TRANSFER_WRITE,
            ),
        );
        let regions = [vk::ImageResolve::default()
            .src_subresource(subresource_layers(color_aspect, 0))
            .dst_subresource(subresource_layers(color_aspect, 0))
            .extent(extent3d(width, height))];
        ctx.device.cmd_resolve_image(
            command_buffer,
            samples.image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            resolved.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &regions,
        );
        image_barrier(
            &ctx.device,
            command_buffer,
            resolved.image,
            color_aspect,
            0,
            1,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            (
                vk::PipelineStageFlags2::RESOLVE,
                vk::AccessFlags2::TRANSFER_WRITE,
            ),
            (
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
            ),
        );
        ctx.commands_submit(command_buffer);
        ctx.readback(
            color.image,
            color_aspect,
            width,
            height,
            4,
            "color",
            "rgba8",
        );
        ctx.readback(depth.image, depth_aspect, width, height, 4, "depth", "d32f");
        ctx.readback(
            resolved.image,
            color_aspect,
            width,
            height,
            4,
            "resolved",
            "rgba8",
        );

        ctx.device.destroy_pipeline(two_samples, None);
        ctx.device.destroy_pipeline(all_samples, None);
        ctx.device.destroy_pipeline_layout(layout, None);
    }
    ctx.image_destroy(&resolved);
    ctx.image_destroy(&depth);
    ctx.image_destroy(&color);
    ctx.image_destroy(&depth_samples);
    ctx.image_destroy(&samples);
}
