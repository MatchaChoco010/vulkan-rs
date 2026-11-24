use vulkan::Device;
use vulkan::vk;

use crate::render::{
    BLEND_DISABLED, Check, Ctx, GraphicsDesc, color_attachment_begin, color_attachment_end,
    extent3d, image_desc, rendering_begin, subresource_layers,
};

#[allow(clippy::too_many_arguments)]
fn layers_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    aspect: vk::ImageAspectFlags,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    src: (vk::PipelineStageFlags2, vk::AccessFlags2),
    dst: (vk::PipelineStageFlags2, vk::AccessFlags2),
) {
    let barriers = [vk::ImageMemoryBarrier2::default()
        .src_stage_mask(src.0)
        .src_access_mask(src.1)
        .dst_stage_mask(dst.0)
        .dst_access_mask(dst.1)
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(aspect)
                .base_mip_level(0)
                .level_count(vk::REMAINING_MIP_LEVELS)
                .base_array_layer(0)
                .layer_count(vk::REMAINING_ARRAY_LAYERS),
        )];
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &dependency) };
}

fn to_transfer_dst(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    aspect: vk::ImageAspectFlags,
) {
    layers_barrier(
        device,
        command_buffer,
        image,
        aspect,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        (
            vk::PipelineStageFlags2::ALL_TRANSFER,
            vk::AccessFlags2::TRANSFER_WRITE,
        ),
    );
}

fn to_shader_read(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    aspect: vk::ImageAspectFlags,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
) {
    layers_barrier(
        device,
        command_buffer,
        image,
        aspect,
        old_layout,
        new_layout,
        (
            vk::PipelineStageFlags2::ALL_TRANSFER,
            vk::AccessFlags2::TRANSFER_WRITE | vk::AccessFlags2::TRANSFER_READ,
        ),
        (
            vk::PipelineStageFlags2::FRAGMENT_SHADER,
            vk::AccessFlags2::SHADER_SAMPLED_READ,
        ),
    );
}

fn sampler_create(
    ctx: &Ctx,
    filter: vk::Filter,
    address_mode: vk::SamplerAddressMode,
    compare: bool,
) -> vk::Sampler {
    let info = vk::SamplerCreateInfo::default()
        .mag_filter(filter)
        .min_filter(filter)
        .mipmap_mode(vk::SamplerMipmapMode::NEAREST)
        .address_mode_u(address_mode)
        .address_mode_v(address_mode)
        .address_mode_w(address_mode)
        .max_anisotropy(1.0)
        .compare_enable(compare)
        .compare_op(if compare {
            vk::CompareOp::LESS
        } else {
            vk::CompareOp::NEVER
        })
        .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE);
    unsafe { ctx.device.create_sampler(&info, None) }.check("vkCreateSampler")
}

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let sampled_usage = vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED;
    let face_colors: [[u8; 4]; 6] = [
        [255, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 255, 255],
        [255, 255, 0, 255],
        [255, 0, 255, 255],
        [0, 255, 255, 255],
    ];
    let slice_colors: [[u8; 4]; 2] = [[200, 100, 50, 255], [50, 100, 200, 255]];
    let pair_texels: [u8; 8] = [250, 10, 10, 255, 10, 10, 250, 255];
    let staged = face_colors
        .iter()
        .chain(&slice_colors)
        .flat_map(|color| std::iter::repeat_n(*color, 16).flatten())
        .collect::<Vec<u8>>();

    let staging = ctx.buffer_with_data(vk::BufferUsageFlags::TRANSFER_SRC, &staged);
    let mut cube_desc = image_desc(
        format,
        4,
        4,
        sampled_usage | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    );
    cube_desc.info = cube_desc
        .info
        .flags(vk::ImageCreateFlags::CUBE_COMPATIBLE)
        .array_layers(6);
    cube_desc.view_type = vk::ImageViewType::CUBE;
    let cube = ctx.image_create(&cube_desc);
    let mut layers_desc = image_desc(format, 4, 4, sampled_usage, vk::ImageAspectFlags::COLOR);
    layers_desc.info = layers_desc.info.array_layers(3);
    layers_desc.view_type = vk::ImageViewType::_2D_ARRAY;
    let layers = ctx.image_create(&layers_desc);
    let mut volume_desc = image_desc(format, 4, 4, sampled_usage, vk::ImageAspectFlags::COLOR);
    volume_desc.info = volume_desc
        .info
        .image_type(vk::ImageType::_3D)
        .extent(vk::Extent3D {
            width: 4,
            height: 4,
            depth: 2,
        });
    volume_desc.view_type = vk::ImageViewType::_3D;
    let volume = ctx.image_create(&volume_desc);
    let integers = ctx.image_create(&image_desc(
        vk::Format::R8G8B8A8_UINT,
        1,
        1,
        sampled_usage,
        vk::ImageAspectFlags::COLOR,
    ));

    unsafe {
        let swapped_info = vk::ImageViewCreateInfo::default()
            .image(integers.image)
            .view_type(vk::ImageViewType::_2D)
            .format(vk::Format::R8G8B8A8_UINT)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::B,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::R,
                a: vk::ComponentSwizzle::A,
            })
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );
        let swapped_view = ctx
            .device
            .create_image_view(&swapped_info, None)
            .check("vkCreateImageView");
        let pair = ctx.image_create(&image_desc(
            format,
            2,
            1,
            sampled_usage,
            vk::ImageAspectFlags::COLOR,
        ));
        ctx.image_upload(
            pair.image,
            2,
            1,
            &pair_texels,
            vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        );
        let depth = ctx.image_create(&image_desc(
            vk::Format::D32_SFLOAT,
            4,
            4,
            sampled_usage | vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::ImageAspectFlags::DEPTH,
        ));
        let color = ctx.image_create(&image_desc(
            format,
            width,
            height,
            vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC,
            vk::ImageAspectFlags::COLOR,
        ));

        let command_buffer = ctx.commands_begin();
        let color_aspect = vk::ImageAspectFlags::COLOR;
        to_transfer_dst(&ctx.device, command_buffer, cube.image, color_aspect);
        to_transfer_dst(&ctx.device, command_buffer, layers.image, color_aspect);
        to_transfer_dst(&ctx.device, command_buffer, volume.image, color_aspect);
        to_transfer_dst(&ctx.device, command_buffer, integers.image, color_aspect);
        to_transfer_dst(
            &ctx.device,
            command_buffer,
            depth.image,
            vk::ImageAspectFlags::DEPTH,
        );
        let faces: [vk::BufferImageCopy; 6] = std::array::from_fn(|face| {
            vk::BufferImageCopy::default()
                .buffer_offset(face as vk::DeviceSize * 64)
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(color_aspect)
                        .mip_level(0)
                        .base_array_layer(face as u32)
                        .layer_count(1),
                )
                .image_extent(extent3d(4, 4))
        });
        ctx.device.cmd_copy_buffer_to_image(
            command_buffer,
            staging.buffer,
            cube.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &faces,
        );
        let slices = [vk::BufferImageCopy::default()
            .buffer_offset(6 * 64)
            .image_subresource(subresource_layers(color_aspect, 0))
            .image_extent(vk::Extent3D {
                width: 4,
                height: 4,
                depth: 2,
            })];
        ctx.device.cmd_copy_buffer_to_image(
            command_buffer,
            staging.buffer,
            volume.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &slices,
        );
        let layer_color =
            vk::ClearColorValue::float32([40.0 / 255.0, 80.0 / 255.0, 120.0 / 255.0, 1.0]);
        let first_layers = [vk::ImageSubresourceRange::default()
            .aspect_mask(color_aspect)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(2)];
        ctx.device.cmd_clear_color_image(
            command_buffer,
            layers.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &layer_color,
            &first_layers,
        );
        let integer_color = vk::ClearColorValue::uint32([10, 200, 30, 255]);
        let whole = [vk::ImageSubresourceRange::default()
            .aspect_mask(color_aspect)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)];
        ctx.device.cmd_clear_color_image(
            command_buffer,
            integers.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &integer_color,
            &whole,
        );
        let depth_value = vk::ClearDepthStencilValue {
            depth: 0.5,
            stencil: 0,
        };
        let whole_depth = [vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::DEPTH)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1)];
        ctx.device.cmd_clear_depth_stencil_image(
            command_buffer,
            depth.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &depth_value,
            &whole_depth,
        );
        layers_barrier(
            &ctx.device,
            command_buffer,
            cube.image,
            color_aspect,
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
        let face_to_layer = [vk::ImageCopy::default()
            .src_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(color_aspect)
                    .mip_level(0)
                    .base_array_layer(3)
                    .layer_count(1),
            )
            .dst_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(color_aspect)
                    .mip_level(0)
                    .base_array_layer(2)
                    .layer_count(1),
            )
            .extent(extent3d(4, 4))];
        ctx.device.cmd_copy_image(
            command_buffer,
            cube.image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            layers.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &face_to_layer,
        );
        let read_only = vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL;
        to_shader_read(
            &ctx.device,
            command_buffer,
            cube.image,
            color_aspect,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            read_only,
        );
        for image in [layers.image, volume.image, integers.image] {
            to_shader_read(
                &ctx.device,
                command_buffer,
                image,
                color_aspect,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                read_only,
            );
        }
        to_shader_read(
            &ctx.device,
            command_buffer,
            depth.image,
            vk::ImageAspectFlags::DEPTH,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
        );
        ctx.commands_submit(command_buffer);

        let clamp = vk::SamplerAddressMode::CLAMP_TO_EDGE;
        let nearest = sampler_create(ctx, vk::Filter::NEAREST, clamp, false);
        let linear = sampler_create(ctx, vk::Filter::LINEAR, clamp, false);
        let border = sampler_create(
            ctx,
            vk::Filter::NEAREST,
            vk::SamplerAddressMode::CLAMP_TO_BORDER,
            false,
        );
        let mirror = sampler_create(
            ctx,
            vk::Filter::NEAREST,
            vk::SamplerAddressMode::MIRRORED_REPEAT,
            false,
        );
        let comparison = sampler_create(ctx, vk::Filter::NEAREST, clamp, true);

        let combined = vk::DescriptorType::COMBINED_IMAGE_SAMPLER;
        let types = [
            combined,
            combined,
            combined,
            combined,
            vk::DescriptorType::SAMPLED_IMAGE,
            vk::DescriptorType::SAMPLER,
            vk::DescriptorType::SAMPLER,
            combined,
        ];
        let bindings: [vk::DescriptorSetLayoutBinding; 8] = std::array::from_fn(|index| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index as u32)
                .descriptor_type(types[index])
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        });
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pool_sizes = [
            vk::DescriptorPoolSize::default()
                .ty(combined)
                .descriptor_count(5),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::SAMPLED_IMAGE)
                .descriptor_count(1),
            vk::DescriptorPoolSize::default()
                .ty(vk::DescriptorType::SAMPLER)
                .descriptor_count(2),
        ];
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
        let image_info = |sampler: vk::Sampler, view: vk::ImageView, layout: vk::ImageLayout| {
            [vk::DescriptorImageInfo::default()
                .sampler(sampler)
                .image_view(view)
                .image_layout(layout)]
        };
        let image_infos = [
            image_info(nearest, cube.view, read_only),
            image_info(nearest, layers.view, read_only),
            image_info(linear, volume.view, read_only),
            image_info(nearest, swapped_view, read_only),
            image_info(vk::Sampler::null(), pair.view, read_only),
            image_info(border, vk::ImageView::null(), vk::ImageLayout::UNDEFINED),
            image_info(mirror, vk::ImageView::null(), vk::ImageLayout::UNDEFINED),
            image_info(
                comparison,
                depth.view,
                vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL,
            ),
        ];
        let writes: [vk::WriteDescriptorSet; 8] = std::array::from_fn(|index| {
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(index as u32)
                .descriptor_type(types[index])
                .image_info(&image_infos[index])
        });
        ctx.device.update_descriptor_sets(&writes, &[]);

        let layout = ctx.pipeline_layout(&[set_layout], &[]);
        let vertex = ctx.shader_module("image_views.vert.spv");
        let fragment = ctx.shader_module("image_views.frag.spv");
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
        ctx.device.destroy_sampler(comparison, None);
        ctx.device.destroy_sampler(mirror, None);
        ctx.device.destroy_sampler(border, None);
        ctx.device.destroy_sampler(linear, None);
        ctx.device.destroy_sampler(nearest, None);
        ctx.image_destroy(&color);
        ctx.image_destroy(&depth);
        ctx.image_destroy(&pair);
        ctx.device.destroy_image_view(swapped_view, None);
    }
    ctx.image_destroy(&integers);
    ctx.image_destroy(&volume);
    ctx.image_destroy(&layers);
    ctx.image_destroy(&cube);
    ctx.buffer_destroy(&staging);
}
