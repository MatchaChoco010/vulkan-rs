use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, Image, bytes_of, color_attachment_begin,
    color_attachment_end, image_desc, rendering_begin,
};

fn rgba(r: f32, g: f32, b: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, 1.0]
}

fn texel_image(ctx: &Ctx, format: vk::Format, texel: &[u8; 4]) -> Image {
    let image = ctx.image_create(&image_desc(
        format,
        1,
        1,
        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        vk::ImageAspectFlags::COLOR,
    ));
    ctx.image_upload(
        image.image,
        1,
        1,
        texel,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    );
    image
}

fn layout_binding(
    binding: u32,
    ty: vk::DescriptorType,
    count: u32,
) -> vk::DescriptorSetLayoutBinding<'static> {
    vk::DescriptorSetLayoutBinding::default()
        .binding(binding)
        .descriptor_type(ty)
        .descriptor_count(count)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
}

fn pool_size(ty: vk::DescriptorType, count: u32) -> vk::DescriptorPoolSize {
    vk::DescriptorPoolSize::default()
        .ty(ty)
        .descriptor_count(count)
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let inline_colors = [rgba(204.0, 51.0, 51.0), rgba(51.0, 204.0, 51.0)];
    let inline_bytes = bytes_of(&inline_colors);
    let inline_size = inline_bytes.len() as u32;
    let texel_colors: [u8; 16] = [
        10, 20, 30, 255, 40, 50, 60, 255, 70, 80, 90, 255, 100, 110, 120, 255,
    ];
    let texture_colors: [[u8; 4]; 3] = [[255, 128, 0, 255], [0, 128, 255, 255], [128, 0, 255, 255]];
    let array_capacity = 8;
    let array_counts = [4_u32];

    let texel_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::STORAGE_TEXEL_BUFFER, &texel_colors);

    unsafe {
        let buffer_view_info = vk::BufferViewCreateInfo::default()
            .buffer(texel_buffer.buffer)
            .format(format)
            .offset(0)
            .range(vk::WHOLE_SIZE);
        let buffer_view = ctx
            .device
            .create_buffer_view(&buffer_view_info, None)
            .check("vkCreateBufferView");
        let textures = texture_colors.map(|texel| texel_image(ctx, format, &texel));
        let color = ctx.image_create(&image_desc(
            format,
            width,
            height,
            vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
            vk::ImageAspectFlags::COLOR,
        ));

        let sampler_info = vk::SamplerCreateInfo::default()
            .mag_filter(vk::Filter::NEAREST)
            .min_filter(vk::Filter::NEAREST)
            .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
            .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
            .max_anisotropy(1.0)
            .compare_op(vk::CompareOp::NEVER)
            .border_color(vk::BorderColor::FLOAT_TRANSPARENT_BLACK);
        let sampler = ctx
            .device
            .create_sampler(&sampler_info, None)
            .check("vkCreateSampler");

        let immutable_samplers = [sampler];
        let bindings = [
            layout_binding(0, vk::DescriptorType::INLINE_UNIFORM_BLOCK, inline_size),
            layout_binding(1, vk::DescriptorType::STORAGE_TEXEL_BUFFER, 1),
            layout_binding(2, vk::DescriptorType::SAMPLER, 1)
                .immutable_samplers(&immutable_samplers),
            layout_binding(3, vk::DescriptorType::SAMPLED_IMAGE, array_capacity),
        ];
        let binding_flags = [
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::empty(),
            vk::DescriptorBindingFlags::UPDATE_AFTER_BIND
                | vk::DescriptorBindingFlags::PARTIALLY_BOUND
                | vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT,
        ];
        let mut binding_flags_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo::default().binding_flags(&binding_flags);
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .push_next(&mut binding_flags_info)
            .flags(vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL)
            .bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");

        let pool_sizes = [
            pool_size(vk::DescriptorType::INLINE_UNIFORM_BLOCK, inline_size),
            pool_size(vk::DescriptorType::STORAGE_TEXEL_BUFFER, 1),
            pool_size(vk::DescriptorType::SAMPLER, 1),
            pool_size(vk::DescriptorType::SAMPLED_IMAGE, array_capacity),
        ];
        let mut inline_pool_info = vk::DescriptorPoolInlineUniformBlockCreateInfo::default()
            .max_inline_uniform_block_bindings(1);
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .push_next(&mut inline_pool_info)
            .flags(vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND)
            .max_sets(1)
            .pool_sizes(&pool_sizes);
        let pool = ctx
            .device
            .create_descriptor_pool(&pool_info, None)
            .check("vkCreateDescriptorPool");
        let mut variable_count_info =
            vk::DescriptorSetVariableDescriptorCountAllocateInfo::default()
                .descriptor_counts(&array_counts);
        let set_layouts = [set_layout];
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .push_next(&mut variable_count_info)
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        let set = ctx
            .device
            .allocate_descriptor_sets(&allocate_info)
            .check("vkAllocateDescriptorSets")[0];

        let mut inline_data =
            vk::WriteDescriptorSetInlineUniformBlock::default().data(inline_bytes);
        let sampled = |texture: &Image| {
            vk::DescriptorImageInfo::default()
                .image_view(texture.view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        };
        let first_images = [sampled(&textures[0]), sampled(&textures[1])];
        let buffer_views = [buffer_view];
        let writes = [
            vk::WriteDescriptorSet::default()
                .push_next(&mut inline_data)
                .dst_set(set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_count(inline_size)
                .descriptor_type(vk::DescriptorType::INLINE_UNIFORM_BLOCK),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::STORAGE_TEXEL_BUFFER)
                .texel_buffer_view(&buffer_views),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(3)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&first_images),
        ];
        ctx.device.update_descriptor_sets(&writes, &[]);

        let layout = ctx.pipeline_layout(&[set_layout], &[]);
        let vertex = ctx.shader_module("descriptor_indexing.vert.spv");
        let fragment = ctx.shader_module("descriptor_indexing.frag.spv");
        let formats = [format];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        let pipeline = ctx.graphics_pipeline(GraphicsDesc {
            blend_attachments: &[BLEND_DISABLED],
            rendering: Some(&mut rendering),
            ..GraphicsDesc::new(vertex, fragment, layout, width, height)
        });
        ctx.device.destroy_shader_module(fragment, None);
        ctx.device.destroy_shader_module(vertex, None);

        let command_buffer = ctx.commands_begin();
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
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
        ctx.device.cmd_end_rendering(command_buffer);
        color_attachment_end(&ctx.device, command_buffer, color.image);
        let late_images = [sampled(&textures[2])];
        let late_writes = [vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(3)
            .dst_array_element(3)
            .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
            .image_info(&late_images)];
        ctx.device.update_descriptor_sets(&late_writes, &[]);
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
        ctx.image_destroy(&color);
        for texture in textures.iter().rev() {
            ctx.image_destroy(texture);
        }
        ctx.device.destroy_buffer_view(buffer_view, None);
    }
    ctx.buffer_destroy(&texel_buffer);
}
