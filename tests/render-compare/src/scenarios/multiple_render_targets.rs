use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, bytes_of, color_clear, image_desc, rect,
};

fn color_attachment(format: vk::Format) -> vk::AttachmentDescription {
    vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlagBits::_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
}

fn reference(attachment: u32, layout: vk::ImageLayout) -> vk::AttachmentReference {
    vk::AttachmentReference::default()
        .attachment(attachment)
        .layout(layout)
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let names = ["albedo", "normal", "material", "color"];
    let vertices: [f32; 45] = [
        -0.9, -0.8, 1.0, 0.5, 0.0, 0.7, -0.6, 1.0, 0.5, 0.0, -0.3, 0.9, 1.0, 0.5, 0.0, 0.95, 0.1,
        0.0, 0.75, 0.25, 0.2, 0.95, 0.0, 0.75, 0.25, -0.1, -0.2, 0.0, 0.75, 0.25, -0.95, 0.7, 0.25,
        0.0, 1.0, 0.9, 0.85, 0.25, 0.0, 1.0, 0.9, 0.8, 0.25, 0.0, 1.0,
    ];

    let vertex_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&vertices));
    let targets: [_; 4] = std::array::from_fn(|i| {
        let mut usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;
        if i < 3 {
            usage |= vk::ImageUsageFlags::INPUT_ATTACHMENT;
        }
        ctx.image_create(&image_desc(
            format,
            width,
            height,
            usage,
            vk::ImageAspectFlags::COLOR,
        ))
    });

    unsafe {
        let attachments = [color_attachment(format); 4];
        let gbuffer_outputs: [_; 3] =
            std::array::from_fn(|i| reference(i as u32, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL));
        let gbuffer_inputs: [_; 3] =
            std::array::from_fn(|i| reference(i as u32, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL));
        let composite_outputs = [reference(3, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
        let subpasses = [
            vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(&gbuffer_outputs),
            vk::SubpassDescription::default()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .input_attachments(&gbuffer_inputs)
                .color_attachments(&composite_outputs),
        ];
        let dependencies = [
            vk::SubpassDependency::default()
                .src_subpass(0)
                .dst_subpass(1)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_stage_mask(vk::PipelineStageFlags::FRAGMENT_SHADER)
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::INPUT_ATTACHMENT_READ)
                .dependency_flags(vk::DependencyFlags::BY_REGION),
            vk::SubpassDependency::default()
                .src_subpass(1)
                .dst_subpass(vk::SUBPASS_EXTERNAL)
                .src_stage_mask(
                    vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT
                        | vk::PipelineStageFlags::FRAGMENT_SHADER,
                )
                .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ),
        ];
        let render_pass_info = vk::RenderPassCreateInfo::default()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        let render_pass = ctx
            .device
            .create_render_pass(&render_pass_info, None)
            .check("vkCreateRenderPass");
        let views = targets.map(|target| target.view);
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

        let bindings: [_; 3] = std::array::from_fn(|i| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(i as u32)
                .descriptor_type(vk::DescriptorType::INPUT_ATTACHMENT)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        });
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pool_sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::INPUT_ATTACHMENT)
            .descriptor_count(3)];
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(1)
            .pool_sizes(&pool_sizes);
        let pool = ctx
            .device
            .create_descriptor_pool(&pool_info, None)
            .check("vkCreateDescriptorPool");
        let set_layouts = [set_layout];
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        let set = ctx
            .device
            .allocate_descriptor_sets(&allocate_info)
            .check("vkAllocateDescriptorSets")[0];
        let image_infos: [_; 3] = std::array::from_fn(|i| {
            [vk::DescriptorImageInfo::default()
                .image_view(targets[i].view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)]
        });
        let writes: [_; 3] = std::array::from_fn(|i| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(i as u32)
                .descriptor_type(vk::DescriptorType::INPUT_ATTACHMENT)
                .image_info(&image_infos[i])
        });
        ctx.device.update_descriptor_sets(&writes, &[]);

        let gbuffer_layout = ctx.pipeline_layout(&[], &[]);
        let composite_layout = ctx.pipeline_layout(&set_layouts, &[]);
        let vertex = ctx.shader_module("multiple_render_targets.vert.spv");
        let fragment = ctx.shader_module("multiple_render_targets.frag.spv");
        let composite_vertex = ctx.shader_module("multiple_render_targets.composite.vert.spv");
        let composite_fragment = ctx.shader_module("multiple_render_targets.composite.frag.spv");
        let vertex_bindings = [vk::VertexInputBindingDescription::default()
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
            .vertex_binding_descriptions(&vertex_bindings)
            .vertex_attribute_descriptions(&attributes);
        let gbuffer_pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            blend_attachments: &[BLEND_DISABLED; 3],
            render_pass,
            subpass: 0,
            ..GraphicsDesc::new(vertex, fragment, gbuffer_layout, width, height)
        });
        let composite_pipeline = ctx.graphics_pipeline(GraphicsDesc {
            blend_attachments: &[BLEND_DISABLED],
            render_pass,
            subpass: 1,
            ..GraphicsDesc::new(
                composite_vertex,
                composite_fragment,
                composite_layout,
                width,
                height,
            )
        });
        ctx.device.destroy_shader_module(composite_fragment, None);
        ctx.device.destroy_shader_module(composite_vertex, None);
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        let clears = [
            color_clear([0.0, 0.0, 0.0, 1.0]),
            color_clear([0.5, 0.5, 1.0, 1.0]),
            color_clear([0.25, 0.25, 0.25, 1.0]),
            color_clear([1.0, 0.0, 1.0, 1.0]),
        ];
        let begin = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(rect(width, height))
            .clear_values(&clears);
        ctx.device
            .cmd_begin_render_pass(command_buffer, &begin, vk::SubpassContents::INLINE);
        ctx.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            gbuffer_pipeline,
        );
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_draw(command_buffer, 9, 1, 0, 0);
        ctx.device
            .cmd_next_subpass(command_buffer, vk::SubpassContents::INLINE);
        ctx.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            composite_pipeline,
        );
        ctx.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            composite_layout,
            0,
            &[set],
            &[],
        );
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        ctx.device.cmd_end_render_pass(command_buffer);
        ctx.commands_submit(command_buffer);
        for (target, name) in targets.iter().zip(names) {
            ctx.readback(
                target.image,
                vk::ImageAspectFlags::COLOR,
                width,
                height,
                4,
                name,
                "rgba8",
            );
        }

        ctx.device.destroy_pipeline(composite_pipeline, None);
        ctx.device.destroy_pipeline(gbuffer_pipeline, None);
        ctx.device.destroy_pipeline_layout(composite_layout, None);
        ctx.device.destroy_pipeline_layout(gbuffer_layout, None);
        ctx.device.destroy_descriptor_pool(pool, None);
        ctx.device.destroy_descriptor_set_layout(set_layout, None);
        ctx.device.destroy_framebuffer(framebuffer, None);
        ctx.device.destroy_render_pass(render_pass, None);
    }
    for target in &targets {
        ctx.image_destroy(target);
    }
    ctx.buffer_destroy(&vertex_buffer);
}
