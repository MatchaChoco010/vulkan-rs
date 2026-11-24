use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, bytes_of, color_clear, host_read_barrier, image_desc,
    rect,
};

fn ndc(pixel: i32) -> f32 {
    pixel as f32 / 128.0 - 1.0
}

fn rectangle(x0: i32, y0: i32, x1: i32, y1: i32, r: f32, g: f32, b: f32) -> [f32; 8] {
    [r, g, b, 1.0, ndc(x0), ndc(y0), ndc(x1), ndc(y1)]
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
    let queried = [
        rectangle(16, 16, 48, 48, 1.0, 0.0, 0.0),
        rectangle(320, 16, 352, 48, 1.0, 1.0, 1.0),
        rectangle(64, 16, 80, 24, 0.0, 1.0, 0.0),
    ];
    let secondary_rectangles = [
        rectangle(16, 64, 112, 112, 0.0, 0.0, 1.0),
        rectangle(128, 64, 240, 200, 1.0, 1.0, 0.0),
    ];

    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));
    let results = ctx.buffer_create(
        3 * size_of::<u64>() as vk::DeviceSize,
        vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    unsafe {
        let attachment = |load_op, initial_layout, final_layout| {
            vk::AttachmentDescription::default()
                .format(format)
                .samples(vk::SampleCountFlagBits::_1)
                .load_op(load_op)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(initial_layout)
                .final_layout(final_layout)
        };
        let attachments = [
            attachment(
                vk::AttachmentLoadOp::CLEAR,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ),
            attachment(
                vk::AttachmentLoadOp::LOAD,
                vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            ),
        ];
        let color_references = [vk::AttachmentReference::default()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpasses = [vk::SubpassDescription::default()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&color_references)];
        let dependencies = [
            vk::SubpassDependency::default()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(
                    vk::AccessFlags::COLOR_ATTACHMENT_READ
                        | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
                ),
            vk::SubpassDependency::default()
                .src_subpass(0)
                .dst_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ),
        ];
        let render_passes: [_; 2] = std::array::from_fn(|i| {
            let render_pass_info = vk::RenderPassCreateInfo::default()
                .attachments(std::slice::from_ref(&attachments[i]))
                .subpasses(&subpasses)
                .dependencies(&dependencies);
            ctx.device
                .create_render_pass(&render_pass_info, None)
                .check("vkCreateRenderPass")
        });
        let views = [color.view];
        let framebuffer_info = vk::FramebufferCreateInfo::default()
            .render_pass(render_passes[0])
            .attachments(&views)
            .width(width)
            .height(height)
            .layers(1);
        let framebuffer = ctx
            .device
            .create_framebuffer(&framebuffer_info, None)
            .check("vkCreateFramebuffer");

        let query_pool_info = vk::QueryPoolCreateInfo::default()
            .query_type(vk::QueryType::OCCLUSION)
            .query_count(3);
        let query_pool = ctx
            .device
            .create_query_pool(&query_pool_info, None)
            .check("vkCreateQueryPool");

        let ranges = [vk::PushConstantRange::default()
            .stage_flags(stages)
            .offset(0)
            .size(32)];
        let layout = ctx.pipeline_layout(&[], &ranges);
        let vertex = ctx.shader_module("blend.vert.spv");
        let fragment = ctx.shader_module("blend.frag.spv");
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            blend_attachments: &[BLEND_DISABLED],
            render_pass: render_passes[0],
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let secondary_allocate = vk::CommandBufferAllocateInfo::default()
            .command_pool(ctx.command_pool)
            .level(vk::CommandBufferLevel::SECONDARY)
            .command_buffer_count(2);
        let secondaries = ctx
            .device
            .allocate_command_buffers(&secondary_allocate)
            .check("vkAllocateCommandBuffers");
        let inheritance = vk::CommandBufferInheritanceInfo::default()
            .render_pass(render_passes[1])
            .subpass(0)
            .framebuffer(framebuffer);
        let secondary_begin = vk::CommandBufferBeginInfo::default()
            .flags(
                vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT
                    | vk::CommandBufferUsageFlags::RENDER_PASS_CONTINUE,
            )
            .inheritance_info(&inheritance);
        for (&secondary, push) in secondaries.iter().zip(&secondary_rectangles) {
            ctx.device
                .begin_command_buffer(secondary, &secondary_begin)
                .check("vkBeginCommandBuffer");
            ctx.device
                .cmd_bind_pipeline(secondary, vk::PipelineBindPoint::GRAPHICS, pipeline);
            ctx.device
                .cmd_push_constants(secondary, layout, stages, 0, bytes_of(push));
            ctx.device.cmd_draw(secondary, 6, 1, 0, 0);
            ctx.device
                .end_command_buffer(secondary)
                .check("vkEndCommandBuffer");
        }

        let command_buffer = ctx.commands_begin();
        ctx.device
            .cmd_reset_query_pool(command_buffer, query_pool, 0, 3);
        let clear_values = [color_clear([0.0, 0.0, 0.0, 1.0])];
        let first_begin = vk::RenderPassBeginInfo::default()
            .render_pass(render_passes[0])
            .framebuffer(framebuffer)
            .render_area(rect(width, height))
            .clear_values(&clear_values);
        ctx.device
            .cmd_begin_render_pass(command_buffer, &first_begin, vk::SubpassContents::INLINE);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        for (query, push) in queried.iter().enumerate() {
            ctx.device.cmd_begin_query(
                command_buffer,
                query_pool,
                query as u32,
                vk::QueryControlFlags::PRECISE,
            );
            ctx.device
                .cmd_push_constants(command_buffer, layout, stages, 0, bytes_of(push));
            ctx.device.cmd_draw(command_buffer, 6, 1, 0, 0);
            ctx.device
                .cmd_end_query(command_buffer, query_pool, query as u32);
        }
        ctx.device.cmd_end_render_pass(command_buffer);
        let second_begin = vk::RenderPassBeginInfo::default()
            .render_pass(render_passes[1])
            .framebuffer(framebuffer)
            .render_area(rect(width, height));
        ctx.device.cmd_begin_render_pass(
            command_buffer,
            &second_begin,
            vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
        );
        ctx.device
            .cmd_execute_commands(command_buffer, &secondaries);
        ctx.device.cmd_end_render_pass(command_buffer);
        ctx.device.cmd_copy_query_pool_results(
            command_buffer,
            query_pool,
            0,
            3,
            results.buffer,
            0,
            size_of::<u64>() as vk::DeviceSize,
            vk::QueryResultFlags::_64 | vk::QueryResultFlags::WAIT,
        );
        host_read_barrier(
            &ctx.device,
            command_buffer,
            vk::PipelineStageFlags2::COPY,
            vk::AccessFlags2::TRANSFER_WRITE,
        );
        ctx.commands_submit(command_buffer);
        ctx.device
            .free_command_buffers(ctx.command_pool, &secondaries);

        let mut host_results = [0_u32; 3];
        let mut bytes = [0_u8; 12];
        ctx.device
            .get_query_pool_results(
                query_pool,
                0,
                3,
                &mut bytes,
                size_of::<u32>() as vk::DeviceSize,
                vk::QueryResultFlags::WAIT,
            )
            .check("vkGetQueryPoolResults");
        for (result, word) in host_results.iter_mut().zip(bytes.chunks_exact(4)) {
            *result = u32::from_ne_bytes(word.try_into().unwrap());
        }
        ctx.readback(
            color.image,
            vk::ImageAspectFlags::COLOR,
            width,
            height,
            4,
            "color",
            "rgba8",
        );
        ctx.output_buffer(&results, 6, 1, "copied_queries", "r32ui");
        ctx.output_words(&host_results, "host_queries");

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_query_pool(query_pool, None);
        ctx.device.destroy_framebuffer(framebuffer, None);
        ctx.device.destroy_render_pass(render_passes[1], None);
        ctx.device.destroy_render_pass(render_passes[0], None);
    }
    ctx.buffer_destroy(&results);
    ctx.image_destroy(&color);
}
