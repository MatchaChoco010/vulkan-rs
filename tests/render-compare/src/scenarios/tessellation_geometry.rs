use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, bytes_of, color_attachment_begin, color_attachment_end, extent2d,
    image_desc, rendering_begin,
};

#[derive(Clone, Copy)]
#[repr(C)]
struct Push {
    level: f32,
    dropped: u32,
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices: [f32; 8] = [-0.75, -0.75, 0.75, -0.75, -0.75, 0.75, 0.75, 0.75];
    let push = [Push {
        level: 2.0,
        dropped: 3,
    }];
    let push_stages = vk::ShaderStageFlags::TESSELLATION_CONTROL | vk::ShaderStageFlags::GEOMETRY;

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
        let ranges = [vk::PushConstantRange::default()
            .stage_flags(push_stages)
            .offset(0)
            .size(size_of::<Push>() as u32)];
        let layout = ctx.pipeline_layout(&[], &ranges);
        let sources = [
            (
                "tessellation_geometry.vert.spv",
                vk::ShaderStageFlagBits::VERTEX,
            ),
            (
                "tessellation_geometry.tesc.spv",
                vk::ShaderStageFlagBits::TESSELLATION_CONTROL,
            ),
            (
                "tessellation_geometry.tese.spv",
                vk::ShaderStageFlagBits::TESSELLATION_EVALUATION,
            ),
            (
                "tessellation_geometry.geom.spv",
                vk::ShaderStageFlagBits::GEOMETRY,
            ),
            (
                "tessellation_geometry.frag.spv",
                vk::ShaderStageFlagBits::FRAGMENT,
            ),
        ];
        let modules = sources.map(|(file, _)| ctx.shader_module(file));
        let stages: [vk::PipelineShaderStageCreateInfo; 5] = std::array::from_fn(|index| {
            vk::PipelineShaderStageCreateInfo::default()
                .stage(sources[index].1)
                .module(modules[index])
                .name(c"main")
        });
        let bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(8)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let attributes = [vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0)];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(vk::PrimitiveTopology::PATCH_LIST);
        let mut domain_origin = vk::PipelineTessellationDomainOriginStateCreateInfo::default()
            .domain_origin(vk::TessellationDomainOrigin::LOWER_LEFT);
        let tessellation = vk::PipelineTessellationStateCreateInfo::default()
            .push_next(&mut domain_origin)
            .patch_control_points(4);
        let viewports = [vk::Viewport::default()
            .width(width as f32)
            .height(height as f32)
            .max_depth(1.0)];
        let scissors = [vk::Rect2D::default().extent(extent2d(width, height))];
        let viewport_state = vk::PipelineViewportStateCreateInfo::default()
            .viewports(&viewports)
            .scissors(&scissors);
        let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
            .line_width(1.0);
        let multisample = vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(vk::SampleCountFlagBits::_1);
        let blend_attachments = [BLEND_DISABLED];
        let color_blend =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline_info = vk::GraphicsPipelineCreateInfo::default()
            .push_next(&mut rendering)
            .stages(&stages)
            .vertex_input_state(&vertex_input)
            .input_assembly_state(&input_assembly)
            .tessellation_state(&tessellation)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization)
            .multisample_state(&multisample)
            .color_blend_state(&color_blend)
            .layout(layout);
        let pipeline = match ctx.device.create_graphics_pipelines(
            vk::PipelineCache::null(),
            &[pipeline_info],
            None,
        ) {
            Ok(pipelines) => pipelines[0],
            Err((_, error)) => {
                crate::render::fail(&format!("vkCreateGraphicsPipelines returned {error}"))
            }
        };
        for module in modules.iter().rev() {
            ctx.device.destroy_shader_module(*module, None);
        }

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            [16.0 / 255.0, 16.0 / 255.0, 16.0 / 255.0, 1.0],
            width,
            height,
        );
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device
            .cmd_push_constants(command_buffer, layout, push_stages, 0, bytes_of(&push));
        ctx.device.cmd_draw(command_buffer, 4, 1, 0, 0);
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

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
    }
    ctx.image_destroy(&color);
    ctx.buffer_destroy(&vertex_buffer);
}
