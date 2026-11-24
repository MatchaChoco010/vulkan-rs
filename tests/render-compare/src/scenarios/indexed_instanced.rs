use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, GraphicsDesc, bytes_of, color_attachment_begin, color_attachment_end,
    image_desc, rendering_begin,
};

#[repr(C)]
#[derive(Clone, Copy)]
struct Instance {
    offset: [f32; 2],
    color: [u8; 4],
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let corners: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0];
    let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
    let instances: [Instance; 16] = std::array::from_fn(|i| Instance {
        offset: [-0.75 + 0.5 * (i % 4) as f32, -0.75 + 0.5 * (i / 4) as f32],
        color: [
            (i * 53 % 256) as u8,
            (i * 97 % 256) as u8,
            ((i * 29 + 64) % 256) as u8,
            255,
        ],
    });

    let corner_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&corners));
    let instance_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&instances));
    let index_buffer = ctx.buffer_with_data(vk::BufferUsageFlags::INDEX_BUFFER, bytes_of(&indices));
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let layout = ctx.pipeline_layout(&[], &[]);
        let vertex = ctx.shader_module("indexed_instanced.vert.spv");
        let fragment = ctx.shader_module("indexed_instanced.frag.spv");
        let bindings = [
            vk::VertexInputBindingDescription::default()
                .binding(0)
                .stride(8)
                .input_rate(vk::VertexInputRate::VERTEX),
            vk::VertexInputBindingDescription::default()
                .binding(1)
                .stride(size_of::<Instance>() as u32)
                .input_rate(vk::VertexInputRate::INSTANCE),
        ];
        let attributes = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(1)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(2)
                .binding(1)
                .format(vk::Format::R8G8B8A8_UNORM)
                .offset(8),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&bindings)
            .vertex_attribute_descriptions(&attributes);
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            blend_attachments: &[BLEND_DISABLED],
            rendering: Some(&mut rendering),
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        color_attachment_begin(&ctx.device, command_buffer, color.image);
        let clear = [24.0 / 255.0, 24.0 / 255.0, 32.0 / 255.0, 1.0];
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            clear,
            width,
            height,
        );
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        ctx.device.cmd_bind_vertex_buffers(
            command_buffer,
            0,
            &[corner_buffer.buffer, instance_buffer.buffer],
            &[0, 0],
        );
        ctx.device.cmd_bind_index_buffer(
            command_buffer,
            index_buffer.buffer,
            0,
            vk::IndexType::UINT16,
        );
        ctx.device
            .cmd_draw_indexed(command_buffer, indices.len() as u32, 8, 0, 0, 0);
        ctx.device
            .cmd_draw_indexed(command_buffer, indices.len() as u32, 8, 0, 0, 8);
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
    ctx.buffer_destroy(&index_buffer);
    ctx.buffer_destroy(&instance_buffer);
    ctx.buffer_destroy(&corner_buffer);
}
