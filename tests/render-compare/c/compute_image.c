/*
 * A compute shader writes a 256x256 storage image from a palette in a uniform buffer and
 * push constants, counts the pixels of each palette entry with atomics in a storage buffer
 * that vkCmdFillBuffer cleared, and records the id of every workgroup there. The image and
 * the storage buffer are read back.
 */
#include "scenarios.h"

void scenario_compute_image(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float palette[16] = {
        1.0f,            64.0f / 255.0f,  32.0f / 255.0f,  1.0f,
        32.0f / 255.0f,  200.0f / 255.0f, 64.0f / 255.0f,  1.0f,
        16.0f / 255.0f,  32.0f / 255.0f,  240.0f / 255.0f, 1.0f,
        240.0f / 255.0f, 240.0f / 255.0f, 16.0f / 255.0f,  1.0f,
    };
    const uint32_t push[2] = {32, 1};

    ImageDesc image_info =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_STORAGE_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image image = image_create(ctx, &image_info);
    Buffer palette_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT, palette, sizeof(palette));
    Buffer counters = buffer_create(
        ctx, (4 + 32 * 32) * sizeof(uint32_t),
        VK_BUFFER_USAGE_STORAGE_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT,
        VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT | VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);

    VkDescriptorSetLayoutBinding bindings[] = {
        {0, VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, 1, VK_SHADER_STAGE_COMPUTE_BIT, NULL},
        {1, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 1, VK_SHADER_STAGE_COMPUTE_BIT, NULL},
        {2, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1, VK_SHADER_STAGE_COMPUTE_BIT, NULL},
    };
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = COUNT(bindings),
        .pBindings = bindings,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorPoolSize pool_sizes[] = {
        {VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, 1},
        {VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 1},
        {VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1},
    };
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .maxSets = 1,
        .poolSizeCount = COUNT(pool_sizes),
        .pPoolSizes = pool_sizes,
    };
    VkDescriptorPool pool;
    CHECK(vkCreateDescriptorPool(ctx->device, &pool_info, NULL, &pool));
    VkDescriptorSetAllocateInfo allocate_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
        .descriptorPool = pool,
        .descriptorSetCount = 1,
        .pSetLayouts = &set_layout,
    };
    VkDescriptorSet set;
    CHECK(vkAllocateDescriptorSets(ctx->device, &allocate_info, &set));
    VkDescriptorImageInfo storage_image = {VK_NULL_HANDLE, image.view, VK_IMAGE_LAYOUT_GENERAL};
    VkDescriptorBufferInfo uniform = {palette_buffer.buffer, 0, VK_WHOLE_SIZE};
    VkDescriptorBufferInfo storage = {counters.buffer, 0, VK_WHOLE_SIZE};
    VkWriteDescriptorSet writes[] = {
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 0,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_IMAGE,
            .pImageInfo = &storage_image,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 1,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            .pBufferInfo = &uniform,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 2,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
            .pBufferInfo = &storage,
        },
    };
    vkUpdateDescriptorSets(ctx->device, COUNT(writes), writes, 0, NULL);

    VkPushConstantRange range = {VK_SHADER_STAGE_COMPUTE_BIT, 0, sizeof(push)};
    VkPipelineLayout layout = pipeline_layout(ctx, &set_layout, 1, &range, 1);
    VkShaderModule module = shader_module(ctx, "compute_image.comp.spv");
    VkComputePipelineCreateInfo pipeline_info = {
        .sType = VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO,
        .stage =
            {
                .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                .stage = VK_SHADER_STAGE_COMPUTE_BIT,
                .module = module,
                .pName = "main",
            },
        .layout = layout,
    };
    VkPipeline pipeline;
    CHECK(vkCreateComputePipelines(ctx->device, VK_NULL_HANDLE, 1, &pipeline_info, NULL,
                                   &pipeline));
    vkDestroyShaderModule(ctx->device, module, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    image_barrier(command_buffer, image.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_GENERAL, VK_PIPELINE_STAGE_2_NONE,
                  VK_ACCESS_2_NONE, VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT,
                  VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT);
    vkCmdFillBuffer(command_buffer, counters.buffer, 0, VK_WHOLE_SIZE, 0);
    VkBufferMemoryBarrier2 cleared = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2,
        .srcStageMask = VK_PIPELINE_STAGE_2_CLEAR_BIT,
        .srcAccessMask = VK_ACCESS_2_TRANSFER_WRITE_BIT,
        .dstStageMask = VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT,
        .dstAccessMask = VK_ACCESS_2_SHADER_STORAGE_READ_BIT | VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT,
        .srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .buffer = counters.buffer,
        .offset = 0,
        .size = VK_WHOLE_SIZE,
    };
    VkDependencyInfo dependency = {
        .sType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
        .bufferMemoryBarrierCount = 1,
        .pBufferMemoryBarriers = &cleared,
    };
    vkCmdPipelineBarrier2(command_buffer, &dependency);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_COMPUTE, pipeline);
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_COMPUTE, layout, 0, 1, &set,
                            0, NULL);
    vkCmdPushConstants(command_buffer, layout, VK_SHADER_STAGE_COMPUTE_BIT, 0, sizeof(push),
                       push);
    vkCmdDispatch(command_buffer, width / 8, height / 8, 1);
    image_barrier(command_buffer, image.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_GENERAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT, VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT,
                  VK_PIPELINE_STAGE_2_COPY_BIT, VK_ACCESS_2_TRANSFER_READ_BIT);
    host_read_barrier(command_buffer, VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT,
                      VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT);
    commands_submit(ctx, command_buffer);
    readback(ctx, image.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");
    output_buffer(ctx, &counters, 4 + 32 * 32, 1, "counters", "r32ui");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    buffer_destroy(ctx, &counters);
    buffer_destroy(ctx, &palette_buffer);
    image_destroy(ctx, &image);
}
