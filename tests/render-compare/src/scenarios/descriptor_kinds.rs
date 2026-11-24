use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, Image, bytes_of, color_attachment_begin,
    color_attachment_end, image_desc, rendering_begin,
};

fn rgba(r: f32, g: f32, b: f32) -> [f32; 4] {
    [r / 255.0, g / 255.0, b / 255.0, 1.0]
}

fn single_color_image(
    ctx: &Ctx,
    format: vk::Format,
    width: u32,
    height: u32,
    texels: &[u8],
) -> Image {
    let image = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
        vk::ImageAspectFlags::COLOR,
    ));
    ctx.image_upload(
        image.image,
        width,
        height,
        texels,
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
    let mut uniform_colors = [0.0_f32; 128];
    uniform_colors[..4].copy_from_slice(&rgba(10.0, 10.0, 10.0));
    uniform_colors[64..68].copy_from_slice(&rgba(200.0, 30.0, 30.0));
    let mut storage_data = [0.0_f32; 8];
    storage_data[..4].copy_from_slice(&rgba(90.0, 60.0, 30.0));
    storage_data[4..].copy_from_slice(&rgba(30.0, 200.0, 30.0));
    let texel_colors: [u8; 16] = [
        10, 10, 10, 255, 20, 20, 20, 255, 30, 30, 200, 255, 40, 40, 40, 255,
    ];
    let quad_texels: [u8; 16] = [
        50, 50, 50, 255, 200, 200, 30, 255, 60, 60, 60, 255, 70, 70, 70, 255,
    ];
    let single_texels: [[u8; 4]; 3] =
        [[120, 120, 120, 255], [80, 80, 80, 255], [30, 200, 200, 255]];
    let pushed_color = rgba(200.0, 30.0, 200.0);
    let push_index: u32 = 2;

    let uniform_buffer = ctx.buffer_with_data(
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        bytes_of(&uniform_colors),
    );
    let storage_buffer = ctx.buffer_with_data(
        vk::BufferUsageFlags::STORAGE_BUFFER,
        bytes_of(&storage_data),
    );
    let texel_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::UNIFORM_TEXEL_BUFFER, &texel_colors);
    let pushed_buffer = ctx.buffer_with_data(
        vk::BufferUsageFlags::UNIFORM_BUFFER,
        bytes_of(&pushed_color),
    );

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
        let quad_image = single_color_image(ctx, format, 2, 2, &quad_texels);
        let singles = single_texels.map(|texels| single_color_image(ctx, format, 1, 1, &texels));
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
        let immutable_sampler = ctx
            .device
            .create_sampler(&sampler_info, None)
            .check("vkCreateSampler");
        let sampler = ctx
            .device
            .create_sampler(&sampler_info, None)
            .check("vkCreateSampler");

        let immutable_samplers = [immutable_sampler];
        let bindings = [
            layout_binding(0, vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC, 1),
            layout_binding(1, vk::DescriptorType::STORAGE_BUFFER, 1),
            layout_binding(2, vk::DescriptorType::UNIFORM_TEXEL_BUFFER, 1),
            layout_binding(3, vk::DescriptorType::SAMPLED_IMAGE, 1),
            layout_binding(4, vk::DescriptorType::SAMPLER, 1)
                .immutable_samplers(&immutable_samplers),
            layout_binding(5, vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 3),
        ];
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pushed_bindings = [layout_binding(0, vk::DescriptorType::UNIFORM_BUFFER, 1)];
        let pushed_layout_info = vk::DescriptorSetLayoutCreateInfo::default()
            .flags(vk::DescriptorSetLayoutCreateFlags::PUSH_DESCRIPTOR_KHR)
            .bindings(&pushed_bindings);
        let pushed_layout = ctx
            .device
            .create_descriptor_set_layout(&pushed_layout_info, None)
            .check("vkCreateDescriptorSetLayout");

        let pool_sizes = [
            pool_size(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC, 2),
            pool_size(vk::DescriptorType::STORAGE_BUFFER, 2),
            pool_size(vk::DescriptorType::UNIFORM_TEXEL_BUFFER, 2),
            pool_size(vk::DescriptorType::SAMPLED_IMAGE, 2),
            pool_size(vk::DescriptorType::SAMPLER, 2),
            pool_size(vk::DescriptorType::COMBINED_IMAGE_SAMPLER, 6),
        ];
        let pool_info = vk::DescriptorPoolCreateInfo::default()
            .max_sets(2)
            .pool_sizes(&pool_sizes);
        let pool = ctx
            .device
            .create_descriptor_pool(&pool_info, None)
            .check("vkCreateDescriptorPool");
        let set_layouts = [set_layout, set_layout];
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(pool)
            .set_layouts(&set_layouts);
        let sets = ctx
            .device
            .allocate_descriptor_sets(&allocate_info)
            .check("vkAllocateDescriptorSets");
        let (set, source) = (sets[0], sets[1]);

        let storage_info = [vk::DescriptorBufferInfo::default()
            .buffer(storage_buffer.buffer)
            .offset(0)
            .range(vk::WHOLE_SIZE)];
        let source_writes = [vk::WriteDescriptorSet::default()
            .dst_set(source)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&storage_info)];
        ctx.device.update_descriptor_sets(&source_writes, &[]);

        let sampled_info = [vk::DescriptorImageInfo::default()
            .image_view(quad_image.view)
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)];
        let combined_infos = singles.map(|single| {
            vk::DescriptorImageInfo::default()
                .sampler(sampler)
                .image_view(single.view)
                .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        });
        let buffer_views = [buffer_view];
        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(2)
                .descriptor_type(vk::DescriptorType::UNIFORM_TEXEL_BUFFER)
                .texel_buffer_view(&buffer_views),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(3)
                .descriptor_type(vk::DescriptorType::SAMPLED_IMAGE)
                .image_info(&sampled_info),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(5)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(&combined_infos),
        ];
        let copies = [vk::CopyDescriptorSet::default()
            .src_set(source)
            .src_binding(1)
            .dst_set(set)
            .dst_binding(1)
            .descriptor_count(1)];
        ctx.device.update_descriptor_sets(&writes, &copies);

        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::FRAGMENT)
            .offset(0)
            .size(4)];
        let layout = ctx.pipeline_layout(&[set_layout, pushed_layout], &ranges);

        let template_entries = [vk::DescriptorUpdateTemplateEntry::default()
            .dst_binding(0)
            .dst_array_element(0)
            .descriptor_count(1)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC)
            .offset(0)
            .stride(size_of::<vk::DescriptorBufferInfo>())];
        let template_info = vk::DescriptorUpdateTemplateCreateInfo::default()
            .descriptor_update_entries(&template_entries)
            .template_type(vk::DescriptorUpdateTemplateType::DESCRIPTOR_SET)
            .descriptor_set_layout(set_layout);
        let update_template = ctx
            .device
            .create_descriptor_update_template(&template_info, None)
            .check("vkCreateDescriptorUpdateTemplate");
        let uniform_info = vk::DescriptorBufferInfo::default()
            .buffer(uniform_buffer.buffer)
            .offset(0)
            .range(16);
        ctx.device.update_descriptor_set_with_template(
            set,
            update_template,
            std::ptr::from_ref(&uniform_info).cast(),
        );

        let vertex = ctx.shader_module("descriptor_kinds.vert.spv");
        let fragment = ctx.shader_module("descriptor_kinds.frag.spv");
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
            &[256],
        );
        let pushed_info = [vk::DescriptorBufferInfo::default()
            .buffer(pushed_buffer.buffer)
            .offset(0)
            .range(vk::WHOLE_SIZE)];
        let pushed_writes = [vk::WriteDescriptorSet::default()
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .buffer_info(&pushed_info)];
        ctx.device.cmd_push_descriptor_set_khr(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            layout,
            1,
            &pushed_writes,
        );
        ctx.device.cmd_push_constants(
            command_buffer,
            layout,
            vk::ShaderStageFlags::FRAGMENT,
            0,
            &push_index.to_ne_bytes(),
        );
        ctx.device.cmd_draw(command_buffer, 3, 1, 0, 0);
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
        ctx.device
            .destroy_descriptor_update_template(update_template, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_descriptor_pool(pool, None);
        ctx.device
            .destroy_descriptor_set_layout(pushed_layout, None);
        ctx.device.destroy_descriptor_set_layout(set_layout, None);
        ctx.device.destroy_sampler(sampler, None);
        ctx.device.destroy_sampler(immutable_sampler, None);
        ctx.image_destroy(&color);
        for single in &singles {
            ctx.image_destroy(single);
        }
        ctx.image_destroy(&quad_image);
        ctx.device.destroy_buffer_view(buffer_view, None);
    }
    ctx.buffer_destroy(&pushed_buffer);
    ctx.buffer_destroy(&texel_buffer);
    ctx.buffer_destroy(&storage_buffer);
    ctx.buffer_destroy(&uniform_buffer);
}
