use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Ctx, GraphicsDesc, bytes_of, color_attachment_begin, color_attachment_end,
    extent2d, image_desc, rendering_begin,
};

#[derive(Clone, Copy)]
#[repr(C)]
struct Constants {
    palette_base: u32,
    brightness: f32,
    swap_red_and_blue: vk::Bool32,
}

fn pixel(value: f32) -> f32 {
    value / 64.0 - 1.0
}

fn quad(x0: f32, y0: f32, x1: f32, y1: f32) -> [f32; 8] {
    [
        pixel(x0),
        pixel(y0),
        pixel(x1),
        pixel(y0),
        pixel(x0),
        pixel(y1),
        pixel(x1),
        pixel(y1),
    ]
}

fn indexed(
    index_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
) -> vk::DrawIndexedIndirectCommand {
    vk::DrawIndexedIndirectCommand {
        index_count,
        instance_count: 1,
        first_index,
        vertex_offset,
        first_instance,
    }
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let vertices: Vec<f32> = [
        quad(8.0, 8.0, 56.0, 56.0),
        quad(72.0, 40.0, 120.0, 120.0),
        quad(72.0, 8.0, 120.0, 24.0),
        quad(0.0, 0.0, 128.0, 128.0),
        [
            pixel(8.5),
            pixel(16.5),
            pixel(120.5),
            pixel(16.5),
            pixel(16.5),
            pixel(24.5),
            pixel(16.5),
            pixel(120.5),
        ],
        [
            pixel(40.5),
            pixel(40.5),
            pixel(80.5),
            pixel(40.5),
            pixel(40.5),
            pixel(80.5),
            pixel(80.5),
            pixel(80.5),
        ],
    ]
    .concat();
    let indices: [u16; 21] = [
        0, 1, 3, 3, 2, 0, 0, 1, 2, 3, 0xffff, 4, 5, 6, 7, 0, 2, 3, 3, 1, 0,
    ];
    let list_commands = [
        indexed(6, 0, 0, 1),
        indexed(6, 15, 12, 0),
        indexed(6, 0, 4, 2),
        indexed(6, 0, 12, 0),
    ];
    let strips = [indexed(9, 6, 0, 4), indexed(4, 6, 8, 5)];
    let strip = [vk::DrawIndirectCommand {
        vertex_count: 4,
        instance_count: 1,
        first_vertex: 0,
        first_instance: 3,
    }];
    let staged = [bytes_of(&strips), bytes_of(&strip)].concat();
    let draw_count: [u32; 1] = [3];
    let strips_offset = size_of_val(&list_commands) as vk::DeviceSize;
    let strip_offset = strips_offset + size_of_val(&strips) as vk::DeviceSize;

    let vertex_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::VERTEX_BUFFER, bytes_of(&vertices));
    let index_buffer = ctx.buffer_with_data(vk::BufferUsageFlags::INDEX_BUFFER, bytes_of(&indices));
    let staging = ctx.buffer_with_data(vk::BufferUsageFlags::TRANSFER_SRC, &staged);
    let count_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::INDIRECT_BUFFER, bytes_of(&draw_count));
    let indirect_buffer = ctx.buffer_create(
        strip_offset + size_of_val(&strip) as vk::DeviceSize,
        vk::BufferUsageFlags::INDIRECT_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::DEVICE_LOCAL,
    );
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let layout = ctx.pipeline_layout(&[], &[]);
        let vertex = ctx.shader_module("indirect_draws.vert.spv");
        let fragment = ctx.shader_module("indirect_draws.frag.spv");
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
        let culling = vk::PipelineRasterizationStateCreateInfo::default()
            .polygon_mode(vk::PolygonMode::FILL)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .line_width(1.0);
        let vertex_entries = [vk::SpecializationMapEntry::default()
            .constant_id(0)
            .offset(0)
            .size(4)];
        let fragment_entries = [
            vk::SpecializationMapEntry::default()
                .constant_id(1)
                .offset(4)
                .size(4),
            vk::SpecializationMapEntry::default()
                .constant_id(2)
                .offset(8)
                .size(4),
        ];
        let constants = [
            (0, 1.0, vk::FALSE),
            (0, 1.0, vk::TRUE),
            (0, 1.0, vk::FALSE),
            (6, 0.5, vk::FALSE),
            (7, 1.0, vk::FALSE),
        ]
        .map(|(palette_base, brightness, swap_red_and_blue)| {
            [Constants {
                palette_base,
                brightness,
                swap_red_and_blue,
            }]
        });
        let topologies = [
            vk::PrimitiveTopology::TRIANGLE_LIST,
            vk::PrimitiveTopology::TRIANGLE_STRIP,
            vk::PrimitiveTopology::TRIANGLE_STRIP,
            vk::PrimitiveTopology::LINE_LIST,
            vk::PrimitiveTopology::POINT_LIST,
        ];
        let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let formats = [format];
        let mut pipelines = [vk::Pipeline::null(); 5];
        for (index, pipeline) in pipelines.iter_mut().enumerate() {
            let vertex_specialization = vk::SpecializationInfo::default()
                .map_entries(&vertex_entries)
                .data(bytes_of(&constants[index]));
            let fragment_specialization = vk::SpecializationInfo::default()
                .map_entries(&fragment_entries)
                .data(bytes_of(&constants[index]));
            let mut rendering =
                vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
            *pipeline = ctx.graphics_pipeline(GraphicsDesc {
                vertex_input: Some(&vertex_input),
                vertex_specialization: Some(&vertex_specialization),
                fragment_specialization: Some(&fragment_specialization),
                topology: topologies[index],
                primitive_restart: index == 2,
                rasterization: (index == 0).then_some(&culling),
                blend_attachments: &[BLEND_DISABLED],
                dynamic_states: &dynamic_states,
                rendering: Some(&mut rendering),
                ..GraphicsDesc::new(vertex, fragment, layout, width, height)
            });
        }
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
        ctx.device.cmd_update_buffer(
            command_buffer,
            indirect_buffer.buffer,
            0,
            bytes_of(&list_commands),
        );
        let regions = [
            vk::BufferCopy::default()
                .src_offset(0)
                .dst_offset(strips_offset)
                .size(size_of_val(&strips) as vk::DeviceSize),
            vk::BufferCopy::default()
                .src_offset(size_of_val(&strips) as vk::DeviceSize)
                .dst_offset(strip_offset)
                .size(size_of_val(&strip) as vk::DeviceSize),
        ];
        ctx.device.cmd_copy_buffer(
            command_buffer,
            staging.buffer,
            indirect_buffer.buffer,
            &regions,
        );
        let buffer_barriers = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::ALL_TRANSFER)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::DRAW_INDIRECT)
            .dst_access_mask(vk::AccessFlags2::INDIRECT_COMMAND_READ)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .buffer(indirect_buffer.buffer)
            .offset(0)
            .size(vk::WHOLE_SIZE)];
        let dependency = vk::DependencyInfo::default().buffer_memory_barriers(&buffer_barriers);
        ctx.device
            .cmd_pipeline_barrier2(command_buffer, &dependency);
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
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        ctx.device.cmd_bind_index_buffer(
            command_buffer,
            index_buffer.buffer,
            0,
            vk::IndexType::UINT16,
        );
        for quadrant in 0..4_u32 {
            let viewports = [vk::Viewport::default()
                .x(128.0 * (quadrant % 2) as f32)
                .y(128.0 * (quadrant / 2) as f32)
                .width(128.0)
                .height(128.0)
                .min_depth(0.0)
                .max_depth(1.0)];
            let scissors = [vk::Rect2D::default()
                .offset(vk::Offset2D {
                    x: 128 * (quadrant % 2) as i32,
                    y: 128 * (quadrant / 2) as i32,
                })
                .extent(extent2d(128, 128))];
            ctx.device.cmd_set_viewport(command_buffer, 0, &viewports);
            ctx.device.cmd_set_scissor(command_buffer, 0, &scissors);
            ctx.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipelines[quadrant as usize],
            );
            match quadrant {
                0 => ctx.device.cmd_draw_indexed_indirect_count(
                    command_buffer,
                    indirect_buffer.buffer,
                    0,
                    count_buffer.buffer,
                    0,
                    list_commands.len() as u32,
                    size_of::<vk::DrawIndexedIndirectCommand>() as u32,
                ),
                1 => ctx.device.cmd_draw_indirect(
                    command_buffer,
                    indirect_buffer.buffer,
                    strip_offset,
                    1,
                    size_of::<vk::DrawIndirectCommand>() as u32,
                ),
                2 => ctx.device.cmd_draw_indexed_indirect(
                    command_buffer,
                    indirect_buffer.buffer,
                    strips_offset,
                    strips.len() as u32,
                    size_of::<vk::DrawIndexedIndirectCommand>() as u32,
                ),
                _ => {
                    ctx.device.cmd_draw(command_buffer, 4, 1, 16, 0);
                    ctx.device.cmd_bind_pipeline(
                        command_buffer,
                        vk::PipelineBindPoint::GRAPHICS,
                        pipelines[4],
                    );
                    ctx.device.cmd_draw(command_buffer, 4, 1, 20, 0);
                }
            }
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
    ctx.buffer_destroy(&indirect_buffer);
    ctx.buffer_destroy(&count_buffer);
    ctx.buffer_destroy(&staging);
    ctx.buffer_destroy(&index_buffer);
    ctx.buffer_destroy(&vertex_buffer);
}
