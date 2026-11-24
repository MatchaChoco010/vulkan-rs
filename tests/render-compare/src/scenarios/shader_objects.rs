use vulkan::vk;

use crate::render::{
    Ctx, bytes_of, color_attachment_begin, color_attachment_end, image_desc, rect, rendering_begin,
};

fn ndc(pixel: u32) -> f32 {
    pixel as f32 / 128.0 - 1.0
}

fn rectangle(corners: [u32; 4], color: [f32; 4]) -> [f32; 36] {
    let [x0, y0, x1, y1] = corners.map(ndc);
    let [r, g, b, a] = color;
    [
        x0, y0, r, g, b, a, x1, y0, r, g, b, a, x1, y1, r, g, b, a, x1, y1, r, g, b, a, x0, y1, r,
        g, b, a, x0, y0, r, g, b, a,
    ]
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices = [
        rectangle([32, 32, 160, 160], [1.0, 0.0, 0.0, 1.0]),
        rectangle([96, 96, 224, 224], [0.0, 0.0, 1.0, 0.5]),
        rectangle([16, 176, 80, 240], [1.0, 1.0, 1.0, 1.0]),
    ]
    .concat();

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
        let vertex_code = ctx.shader_code("shader_objects.vert.spv");
        let fragment_code = ctx.shader_code("shader_objects.frag.spv");
        let shader_infos = [
            vk::ShaderCreateInfoEXT::default()
                .stage(vk::ShaderStageFlagBits::VERTEX)
                .next_stage(vk::ShaderStageFlags::FRAGMENT)
                .code_type(vk::ShaderCodeTypeEXT::SPIRV)
                .code(bytes_of(&vertex_code))
                .name(c"main"),
            vk::ShaderCreateInfoEXT::default()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .code_type(vk::ShaderCodeTypeEXT::SPIRV)
                .code(bytes_of(&fragment_code))
                .name(c"main"),
        ];
        let shaders = match ctx.device.create_shaders_ext(&shader_infos, None) {
            Ok(shaders) => shaders,
            Err((_, error)) => crate::render::fail(&format!("vkCreateShadersEXT returned {error}")),
        };
        drop(fragment_code);
        drop(vertex_code);

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            [32.0 / 255.0, 48.0 / 255.0, 64.0 / 255.0, 1.0],
            width,
            height,
        );
        let stages = [
            vk::ShaderStageFlagBits::VERTEX,
            vk::ShaderStageFlagBits::TESSELLATION_CONTROL,
            vk::ShaderStageFlagBits::TESSELLATION_EVALUATION,
            vk::ShaderStageFlagBits::GEOMETRY,
            vk::ShaderStageFlagBits::FRAGMENT,
        ];
        let bound = [
            shaders[0],
            vk::ShaderEXT::null(),
            vk::ShaderEXT::null(),
            vk::ShaderEXT::null(),
            shaders[1],
        ];
        ctx.device
            .cmd_bind_shaders_ext(command_buffer, &stages, Some(&bound));
        let viewports = [vk::Viewport::default()
            .width(width as f32)
            .height(height as f32)
            .max_depth(1.0)];
        ctx.device
            .cmd_set_viewport_with_count(command_buffer, &viewports);
        ctx.device
            .cmd_set_scissor_with_count(command_buffer, &[rect(width, height)]);
        ctx.device
            .cmd_set_rasterizer_discard_enable(command_buffer, false);
        let bindings = [vk::VertexInputBindingDescription2EXT::default()
            .binding(0)
            .stride(24)
            .input_rate(vk::VertexInputRate::VERTEX)
            .divisor(1)];
        let attributes = [
            vk::VertexInputAttributeDescription2EXT::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription2EXT::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(8),
        ];
        ctx.device
            .cmd_set_vertex_input_ext(command_buffer, &bindings, &attributes);
        ctx.device
            .cmd_set_primitive_topology(command_buffer, vk::PrimitiveTopology::TRIANGLE_LIST);
        ctx.device
            .cmd_set_primitive_restart_enable(command_buffer, false);
        ctx.device
            .cmd_set_rasterization_samples_ext(command_buffer, vk::SampleCountFlagBits::_1);
        ctx.device
            .cmd_set_sample_mask_ext(command_buffer, vk::SampleCountFlagBits::_1, Some(&[1]));
        ctx.device
            .cmd_set_alpha_to_coverage_enable_ext(command_buffer, false);
        ctx.device
            .cmd_set_polygon_mode_ext(command_buffer, vk::PolygonMode::FILL);
        ctx.device
            .cmd_set_cull_mode(command_buffer, vk::CullModeFlags::NONE);
        ctx.device
            .cmd_set_front_face(command_buffer, vk::FrontFace::COUNTER_CLOCKWISE);
        ctx.device.cmd_set_depth_test_enable(command_buffer, false);
        ctx.device.cmd_set_depth_write_enable(command_buffer, false);
        ctx.device.cmd_set_depth_bias_enable(command_buffer, false);
        ctx.device
            .cmd_set_stencil_test_enable(command_buffer, false);
        ctx.device
            .cmd_set_color_blend_enable_ext(command_buffer, 0, &[vk::FALSE]);
        let equations = [vk::ColorBlendEquationEXT {
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            alpha_blend_op: vk::BlendOp::ADD,
        }];
        ctx.device
            .cmd_set_color_blend_equation_ext(command_buffer, 0, &equations);
        let all_components = vk::ColorComponentFlags::R
            | vk::ColorComponentFlags::G
            | vk::ColorComponentFlags::B
            | vk::ColorComponentFlags::A;
        ctx.device
            .cmd_set_color_write_mask_ext(command_buffer, 0, &[all_components]);
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_draw(command_buffer, 6, 1, 0, 0);
        ctx.device
            .cmd_set_color_blend_enable_ext(command_buffer, 0, &[vk::TRUE]);
        ctx.device.cmd_draw(command_buffer, 6, 1, 6, 0);
        ctx.device
            .cmd_set_color_blend_enable_ext(command_buffer, 0, &[vk::FALSE]);
        let red_and_alpha = vk::ColorComponentFlags::R | vk::ColorComponentFlags::A;
        ctx.device
            .cmd_set_color_write_mask_ext(command_buffer, 0, &[red_and_alpha]);
        ctx.device.cmd_draw(command_buffer, 6, 1, 12, 0);
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

        ctx.device.destroy_shader_ext(shaders[1], None);
        ctx.device.destroy_shader_ext(shaders[0], None);
    }
    ctx.image_destroy(&color);
    ctx.buffer_destroy(&vertex_buffer);
}
