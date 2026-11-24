use vulkan::vk;

use crate::render::{Check, Ctx, bytes_of, fail, host_read_barrier, image_barrier, image_desc};

pub fn run(ctx: &mut Ctx) {
    let width = 256;
    let height = 256;
    let format = vk::Format::R8G8B8A8_UNORM;
    let palette: [f32; 16] = [
        1.0,
        64.0 / 255.0,
        32.0 / 255.0,
        1.0,
        32.0 / 255.0,
        200.0 / 255.0,
        64.0 / 255.0,
        1.0,
        16.0 / 255.0,
        32.0 / 255.0,
        240.0 / 255.0,
        1.0,
        240.0 / 255.0,
        240.0 / 255.0,
        16.0 / 255.0,
        1.0,
    ];
    let push: [u32; 2] = [32, 1];

    let image = ctx.image_create(&image_desc(
        format,
        width,
        height,
        vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC,
        vk::ImageAspectFlags::COLOR,
    ));
    let palette_buffer =
        ctx.buffer_with_data(vk::BufferUsageFlags::UNIFORM_BUFFER, bytes_of(&palette));
    let counters = ctx.buffer_create(
        ((4 + 32 * 32) * size_of::<u32>()) as vk::DeviceSize,
        vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_DST,
        vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
    );

    unsafe {
        let binding = |index: u32, ty: vk::DescriptorType| {
            vk::DescriptorSetLayoutBinding::default()
                .binding(index)
                .descriptor_type(ty)
                .descriptor_count(1)
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
        };
        let bindings = [
            binding(0, vk::DescriptorType::STORAGE_IMAGE),
            binding(1, vk::DescriptorType::UNIFORM_BUFFER),
            binding(2, vk::DescriptorType::STORAGE_BUFFER),
        ];
        let set_layout_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let set_layout = ctx
            .device
            .create_descriptor_set_layout(&set_layout_info, None)
            .check("vkCreateDescriptorSetLayout");
        let pool_size =
            |ty: vk::DescriptorType| vk::DescriptorPoolSize::default().ty(ty).descriptor_count(1);
        let pool_sizes = [
            pool_size(vk::DescriptorType::STORAGE_IMAGE),
            pool_size(vk::DescriptorType::UNIFORM_BUFFER),
            pool_size(vk::DescriptorType::STORAGE_BUFFER),
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
        let storage_image = [vk::DescriptorImageInfo::default()
            .image_view(image.view)
            .image_layout(vk::ImageLayout::GENERAL)];
        let uniform = [vk::DescriptorBufferInfo::default()
            .buffer(palette_buffer.buffer)
            .offset(0)
            .range(vk::WHOLE_SIZE)];
        let storage = [vk::DescriptorBufferInfo::default()
            .buffer(counters.buffer)
            .offset(0)
            .range(vk::WHOLE_SIZE)];
        let writes = [
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(0)
                .descriptor_type(vk::DescriptorType::STORAGE_IMAGE)
                .image_info(&storage_image),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(1)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(&uniform),
            vk::WriteDescriptorSet::default()
                .dst_set(set)
                .dst_binding(2)
                .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                .buffer_info(&storage),
        ];
        ctx.device.update_descriptor_sets(&writes, &[]);

        let ranges = [vk::PushConstantRange::default()
            .stage_flags(vk::ShaderStageFlags::COMPUTE)
            .offset(0)
            .size(8)];
        let layout = ctx.pipeline_layout(&set_layouts, &ranges);
        let module = ctx.shader_module("compute_image.comp.spv");
        let pipeline_info = vk::ComputePipelineCreateInfo::default()
            .stage(
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlagBits::COMPUTE)
                    .module(module)
                    .name(c"main"),
            )
            .layout(layout);
        let pipeline = match ctx.device.create_compute_pipelines(
            vk::PipelineCache::null(),
            &[pipeline_info],
            None,
        ) {
            Ok(pipelines) => pipelines[0],
            Err((_, error)) => fail(&format!("vkCreateComputePipelines returned {error}")),
        };
        ctx.device.destroy_shader_module(module, None);

        let command_buffer = ctx.commands_begin();
        image_barrier(
            &ctx.device,
            command_buffer,
            image.image,
            vk::ImageAspectFlags::COLOR,
            0,
            1,
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::GENERAL,
            (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
            (
                vk::PipelineStageFlags2::COMPUTE_SHADER,
                vk::AccessFlags2::SHADER_STORAGE_WRITE,
            ),
        );
        ctx.device
            .cmd_fill_buffer(command_buffer, counters.buffer, 0, vk::WHOLE_SIZE, 0);
        let cleared = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::CLEAR)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .dst_access_mask(
                vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
            )
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .buffer(counters.buffer)
            .offset(0)
            .size(vk::WHOLE_SIZE)];
        let dependency = vk::DependencyInfo::default().buffer_memory_barriers(&cleared);
        ctx.device
            .cmd_pipeline_barrier2(command_buffer, &dependency);
        ctx.device
            .cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
        ctx.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            layout,
            0,
            &[set],
            &[],
        );
        ctx.device.cmd_push_constants(
            command_buffer,
            layout,
            vk::ShaderStageFlags::COMPUTE,
            0,
            bytes_of(&push),
        );
        ctx.device
            .cmd_dispatch(command_buffer, width / 8, height / 8, 1);
        image_barrier(
            &ctx.device,
            command_buffer,
            image.image,
            vk::ImageAspectFlags::COLOR,
            0,
            1,
            vk::ImageLayout::GENERAL,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            (
                vk::PipelineStageFlags2::COMPUTE_SHADER,
                vk::AccessFlags2::SHADER_STORAGE_WRITE,
            ),
            (
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_READ,
            ),
        );
        host_read_barrier(
            &ctx.device,
            command_buffer,
            vk::PipelineStageFlags2::COMPUTE_SHADER,
            vk::AccessFlags2::SHADER_STORAGE_WRITE,
        );
        ctx.commands_submit(command_buffer);
        ctx.readback(
            image.image,
            vk::ImageAspectFlags::COLOR,
            width,
            height,
            4,
            "color",
            "rgba8",
        );
        ctx.output_buffer(&counters, 4 + 32 * 32, 1, "counters", "r32ui");

        ctx.device.destroy_pipeline(pipeline, None);
        ctx.device.destroy_pipeline_layout(layout, None);
        ctx.device.destroy_descriptor_pool(pool, None);
        ctx.device.destroy_descriptor_set_layout(set_layout, None);
    }
    ctx.buffer_destroy(&counters);
    ctx.buffer_destroy(&palette_buffer);
    ctx.image_destroy(&image);
}
