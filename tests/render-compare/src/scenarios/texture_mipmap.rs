use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, bytes_of, color_attachment_begin,
    color_attachment_end, extent3d, image_barrier, image_desc, rendering_begin, subresource_layers,
};

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 4],
    uv: [f32; 2],
}

fn vertex(position: [f32; 4], uv: [f32; 2]) -> Vertex {
    Vertex { position, uv }
}

fn quad(x0: f32, y0: f32, x1: f32, y1: f32, u1: f32, v1: f32) -> [Vertex; 6] {
    [
        vertex([x0, y0, 0.0, 1.0], [0.0, 0.0]),
        vertex([x1, y0, 0.0, 1.0], [u1, 0.0]),
        vertex([x1, y1, 0.0, 1.0], [u1, v1]),
        vertex([x1, y1, 0.0, 1.0], [u1, v1]),
        vertex([x0, y1, 0.0, 1.0], [0.0, v1]),
        vertex([x0, y0, 0.0, 1.0], [0.0, 0.0]),
    ]
}

fn ndc(pixel: u32) -> f32 {
    pixel as f32 / 128.0 - 1.0
}

fn offset(x: i32, y: i32, z: i32) -> vk::Offset3D {
    vk::Offset3D { x, y, z }
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let size = 64_u32;
    let levels = 7_u32;
    let lods: [f32; 9] = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, -1.0, -1.0];

    let mut texels = vec![0_u8; 64 * 64 * 4];
    for y in 0..size {
        for x in 0..size {
            let at = ((y * size + x) * 4) as usize;
            let light = (x / 8 + y / 8) % 2 == 0;
            texels[at] = if light { 240 } else { 16 };
            texels[at + 1] = if light { 240 } else { 32 };
            texels[at + 2] = if light { 240 } else { 128 };
            texels[at + 3] = 255;
        }
    }
    let mut vertices = Vec::with_capacity(9 * 6);
    for i in 0..8 {
        let x = 64 * (i % 4);
        let y = 64 * (i / 4);
        let scale = if i == 7 { 4.0 } else { 1.0 };
        vertices.extend(quad(ndc(x), ndc(y), ndc(x + 64), ndc(y + 64), scale, scale));
    }
    vertices.extend([
        vertex([-1.0, 1.0, 0.0, 1.0], [0.0, 0.0]),
        vertex([1.0, 1.0, 0.0, 1.0], [4.0, 0.0]),
        vertex([4.0, 0.0, 0.0, 4.0], [4.0, 8.0]),
        vertex([4.0, 0.0, 0.0, 4.0], [4.0, 8.0]),
        vertex([-4.0, 0.0, 0.0, 4.0], [0.0, 8.0]),
        vertex([-1.0, 1.0, 0.0, 1.0], [0.0, 0.0]),
    ]);

    let vertex_buffer = ctx.buffer_with_data(
        vk::BufferUsageFlags::VERTEX_BUFFER,
        bytes_of(vertices.as_slice()),
    );
    let staging = ctx.buffer_with_data(vk::BufferUsageFlags::TRANSFER_SRC, &texels);
    let mut texture_desc = image_desc(
        format,
        size,
        size,
        vk::ImageUsageFlags::TRANSFER_SRC
            | vk::ImageUsageFlags::TRANSFER_DST
            | vk::ImageUsageFlags::SAMPLED,
        vk::ImageAspectFlags::COLOR,
    );
    texture_desc.info = texture_desc.info.mip_levels(levels);
    let texture = ctx.image_create(&texture_desc);
    let color = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let sampler_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::REPEAT)
            .max_anisotropy(1.0)
            .compare_op(vk::CompareOp::NEVER)
            .max_lod((levels - 1) as f32)
            .border_color(vk::BorderColor::FLOAT_TRANSPARENT_BLACK);
        let sampler = ctx
            .device
            .create_sampler(&sampler_info, None)
            .check("vkCreateSampler");

        let bindings = [vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)];
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pool_sizes = [vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)];
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
        let image_infos = [vk::DescriptorImageInfo::default()
            .sampler(sampler)
            .image_view(texture.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let writes = [vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .image_info(&image_infos)];
        ctx.device.update_descriptor_sets(&writes, &[]);

        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .offset(0)
            .size(4)];
        let layout = ctx.pipeline_layout(&set_layouts, &ranges);
        let vertex_module = ctx.shader_module("texture_mipmap.vert.spv");
        let fragment_module = ctx.shader_module("texture_mipmap.frag.spv");
        let vertex_bindings = [vk::VertexInputBindingDescription::default()
            .binding(0)
            .stride(size_of::<Vertex>() as u32)
            .input_rate(vk::VertexInputRate::VERTEX)];
        let attributes = [
            vk::VertexInputAttributeDescription::default()
                .location(0)
                .binding(0)
                .format(vk::Format::R32G32B32A32_SFLOAT)
                .offset(0),
            vk::VertexInputAttributeDescription::default()
                .location(1)
                .binding(0)
                .format(vk::Format::R32G32_SFLOAT)
                .offset(16),
        ];
        let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
            .vertex_binding_descriptions(&vertex_bindings)
            .vertex_attribute_descriptions(&attributes);
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            vertex_input: Some(&vertex_input),
            blend_attachments: &[BLEND_DISABLED],
            rendering: Some(&mut rendering),
            ..GraphicsDesc::new(vertex_module, fragment_module, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment_module, None);
        ctx.device.destroy_shader_module(vertex_module, None);

        let command_buffer = ctx.commands_begin();
        image_barrier(
            &ctx.device,
            command_buffer,
            texture.image,
            vk::ImageAspectFlags::COLOR,
            0,
            levels,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::ALL_TRANSFER,
                vk::AccessFlags2::TRANSFER_WRITE,
            ),
        );
        let copy = vk::BufferImageCopy::default()
            .image_subresource(subresource_layers(vk::ImageAspectFlags::COLOR, 0))
            .image_extent(extent3d(size, size));
        ctx.device.cmd_copy_buffer_to_image(
            command_buffer,
            staging.buffer,
            texture.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[copy],
        );
        for level in 1..levels {
            image_barrier(
                &ctx.device,
                command_buffer,
                texture.image,
                vk::ImageAspectFlags::COLOR,
                level - 1,
                1,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                (
                    vk::PipelineStageFlags2::ALL_TRANSFER,
                    vk::AccessFlags2::TRANSFER_WRITE,
                ),
                (
                    vk::PipelineStageFlags2::ALL_TRANSFER,
                    vk::AccessFlags2::TRANSFER_READ,
                ),
            );
            let source = (size >> (level - 1)) as i32;
            let target = (size >> level) as i32;
            let blit = vk::ImageBlit::default()
                .src_subresource(subresource_layers(vk::ImageAspectFlags::COLOR, level - 1))
                .src_offsets([offset(0, 0, 0), offset(source, source, 1)])
                .dst_subresource(subresource_layers(vk::ImageAspectFlags::COLOR, level))
                .dst_offsets([offset(0, 0, 0), offset(target, target, 1)]);
            ctx.device.cmd_blit_image(
                command_buffer,
                texture.image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                texture.image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[blit],
                vk::Filter::LINEAR,
            );
        }
        image_barrier(
            &ctx.device,
            command_buffer,
            texture.image,
            vk::ImageAspectFlags::COLOR,
            0,
            levels - 1,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            (
                vk::PipelineStageFlags2::ALL_TRANSFER,
                vk::AccessFlags2::TRANSFER_READ,
            ),
            (
                vk::PipelineStageFlags2::FRAGMENT_SHADER,
                vk::AccessFlags2::SHADER_SAMPLED_READ,
            ),
        );
        image_barrier(
            &ctx.device,
            command_buffer,
            texture.image,
            vk::ImageAspectFlags::COLOR,
            levels - 1,
            1,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            (
                vk::PipelineStageFlags2::ALL_TRANSFER,
                vk::AccessFlags2::TRANSFER_WRITE,
            ),
            (
                vk::PipelineStageFlags2::FRAGMENT_SHADER,
                vk::AccessFlags2::SHADER_SAMPLED_READ,
            ),
        );

        color_attachment_begin(&ctx.device, command_buffer, color.image);
        rendering_begin(
            &ctx.device,
            command_buffer,
            color.view,
            [0.0, 0.0, 0.0, 1.0],
            width,
            height,
        );
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        ctx.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            0,
            &[set],
            &[],
        );
        ctx.device
            .cmd_bind_vertex_buffers(command_buffer, 0, &[vertex_buffer.buffer], &[0]);
        for (index, lod) in lods.iter().enumerate() {
            ctx.device.cmd_push_constants(
                command_buffer,
                layout,
                vk::ShaderStageFlags::FRAGMENT,
                0,
                &lod.to_ne_bytes(),
            );
            ctx.device
                .cmd_draw(command_buffer, 6, 1, 6 * index as u32, 0);
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

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_descriptor_pool(pool, None);
        ctx.device.destroy_descriptor_set_layout(set_layout, None);
        ctx.device.destroy_sampler(sampler, None);
    }
    ctx.image_destroy(&color);
    ctx.image_destroy(&texture);
    ctx.buffer_destroy(&staging);
    ctx.buffer_destroy(&vertex_buffer);
}
