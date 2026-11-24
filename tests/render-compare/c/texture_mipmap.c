/*
 * A 64x64 checkerboard texture whose mip levels are made with vkCmdBlitImage, sampled
 * through a combined image sampler: eight quads at explicit levels 0 to 6 and one with an
 * implicit level, and a floor in perspective that goes through the levels.
 */
#include "scenarios.h"

typedef struct {
    float position[4];
    float uv[2];
} Vertex;

static void quad(Vertex *out, float x0, float y0, float x1, float y1, float u1, float v1) {
    const Vertex corners[6] = {
        {{x0, y0, 0.0f, 1.0f}, {0.0f, 0.0f}}, {{x1, y0, 0.0f, 1.0f}, {u1, 0.0f}},
        {{x1, y1, 0.0f, 1.0f}, {u1, v1}},     {{x1, y1, 0.0f, 1.0f}, {u1, v1}},
        {{x0, y1, 0.0f, 1.0f}, {0.0f, v1}},   {{x0, y0, 0.0f, 1.0f}, {0.0f, 0.0f}},
    };
    for (uint32_t i = 0; i < 6; i++) {
        out[i] = corners[i];
    }
}

static float ndc(uint32_t pixel) { return (float)pixel / 128.0f - 1.0f; }

void scenario_texture_mipmap(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    const uint32_t size = 64;
    const uint32_t levels = 7;
    static const float lods[9] = {0.0f, 1.0f, 2.0f, 3.0f, 4.0f, 5.0f, 6.0f, -1.0f, -1.0f};

    uint8_t texels[64 * 64 * 4];
    for (uint32_t y = 0; y < size; y++) {
        for (uint32_t x = 0; x < size; x++) {
            uint8_t *texel = &texels[(y * size + x) * 4];
            int light = (x / 8 + y / 8) % 2 == 0;
            texel[0] = light ? 240 : 16;
            texel[1] = light ? 240 : 32;
            texel[2] = light ? 240 : 128;
            texel[3] = 255;
        }
    }
    Vertex vertices[9 * 6];
    for (uint32_t i = 0; i < 8; i++) {
        uint32_t x = 64 * (i % 4);
        uint32_t y = 64 * (i / 4);
        float scale = i == 7 ? 4.0f : 1.0f;
        quad(&vertices[6 * i], ndc(x), ndc(y), ndc(x + 64), ndc(y + 64), scale, scale);
    }
    const Vertex floor[6] = {
        {{-1.0f, 1.0f, 0.0f, 1.0f}, {0.0f, 0.0f}}, {{1.0f, 1.0f, 0.0f, 1.0f}, {4.0f, 0.0f}},
        {{4.0f, 0.0f, 0.0f, 4.0f}, {4.0f, 8.0f}},  {{4.0f, 0.0f, 0.0f, 4.0f}, {4.0f, 8.0f}},
        {{-4.0f, 0.0f, 0.0f, 4.0f}, {0.0f, 8.0f}}, {{-1.0f, 1.0f, 0.0f, 1.0f}, {0.0f, 0.0f}},
    };
    for (uint32_t i = 0; i < 6; i++) {
        vertices[6 * 8 + i] = floor[i];
    }

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    Buffer staging = buffer_with_data(ctx, VK_BUFFER_USAGE_TRANSFER_SRC_BIT, texels, sizeof(texels));
    ImageDesc texture_desc = image_desc(format, size, size,
                                        VK_IMAGE_USAGE_TRANSFER_SRC_BIT |
                                            VK_IMAGE_USAGE_TRANSFER_DST_BIT |
                                            VK_IMAGE_USAGE_SAMPLED_BIT,
                                        VK_IMAGE_ASPECT_COLOR_BIT);
    texture_desc.info.mipLevels = levels;
    Image texture = image_create(ctx, &texture_desc);
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkSamplerCreateInfo sampler_info = {
        .sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
        .magFilter = VK_FILTER_LINEAR,
        .minFilter = VK_FILTER_LINEAR,
        .mipmapMode = VK_SAMPLER_MIPMAP_MODE_LINEAR,
        .addressModeU = VK_SAMPLER_ADDRESS_MODE_REPEAT,
        .addressModeV = VK_SAMPLER_ADDRESS_MODE_REPEAT,
        .addressModeW = VK_SAMPLER_ADDRESS_MODE_REPEAT,
        .maxAnisotropy = 1.0f,
        .compareOp = VK_COMPARE_OP_NEVER,
        .maxLod = (float)(levels - 1),
        .borderColor = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
    };
    VkSampler sampler;
    CHECK(vkCreateSampler(ctx->device, &sampler_info, NULL, &sampler));

    VkDescriptorSetLayoutBinding binding = {0, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 1,
                                            VK_SHADER_STAGE_FRAGMENT_BIT, NULL};
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = 1,
        .pBindings = &binding,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorPoolSize pool_size = {VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 1};
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .maxSets = 1,
        .poolSizeCount = 1,
        .pPoolSizes = &pool_size,
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
    VkDescriptorImageInfo image_info = {sampler, texture.view,
                                        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL};
    VkWriteDescriptorSet write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        .dstSet = set,
        .dstBinding = 0,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        .pImageInfo = &image_info,
    };
    vkUpdateDescriptorSets(ctx->device, 1, &write, 0, NULL);

    VkPushConstantRange range = {VK_SHADER_STAGE_FRAGMENT_BIT, 0, sizeof(float)};
    VkPipelineLayout layout = pipeline_layout(ctx, &set_layout, 1, &range, 1);
    VkShaderModule vertex = shader_module(ctx, "texture_mipmap.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "texture_mipmap.frag.spv");
    VkVertexInputBindingDescription vertex_binding = {0, sizeof(Vertex),
                                                      VK_VERTEX_INPUT_RATE_VERTEX};
    VkVertexInputAttributeDescription attributes[] = {
        {0, 0, VK_FORMAT_R32G32B32A32_SFLOAT, 0},
        {1, 0, VK_FORMAT_R32G32_SFLOAT, 4 * sizeof(float)},
    };
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = 1,
        .pVertexBindingDescriptions = &vertex_binding,
        .vertexAttributeDescriptionCount = COUNT(attributes),
        .pVertexAttributeDescriptions = attributes,
    };
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &format,
    };
    GraphicsDesc pipeline_desc = {
        .vertex = vertex,
        .fragment = fragment,
        .vertex_entry = "main",
        .fragment_entry = "main",
        .vertex_input = &vertex_input,
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .blend_attachments = &BLEND_DISABLED,
        .blend_attachment_count = 1,
        .layout = layout,
        .rendering = &rendering,
        .width = width,
        .height = height,
    };
    VkPipeline pipeline = graphics_pipeline(ctx, &pipeline_desc);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    image_barrier(command_buffer, texture.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, levels,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                  VK_PIPELINE_STAGE_2_NONE, VK_ACCESS_2_NONE,
                  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT);
    VkBufferImageCopy copy = {
        .imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1},
        .imageExtent = {size, size, 1},
    };
    vkCmdCopyBufferToImage(command_buffer, staging.buffer, texture.image,
                           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &copy);
    for (uint32_t level = 1; level < levels; level++) {
        image_barrier(command_buffer, texture.image, VK_IMAGE_ASPECT_COLOR_BIT, level - 1, 1,
                      VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                      VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT,
                      VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_READ_BIT);
        int32_t source = (int32_t)(size >> (level - 1));
        int32_t target = (int32_t)(size >> level);
        VkImageBlit blit = {
            .srcSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, level - 1, 0, 1},
            .srcOffsets = {{0, 0, 0}, {source, source, 1}},
            .dstSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, level, 0, 1},
            .dstOffsets = {{0, 0, 0}, {target, target, 1}},
        };
        vkCmdBlitImage(command_buffer, texture.image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                       texture.image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &blit,
                       VK_FILTER_LINEAR);
    }
    image_barrier(command_buffer, texture.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, levels - 1,
                  VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_READ_BIT,
                  VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT, VK_ACCESS_2_SHADER_SAMPLED_READ_BIT);
    image_barrier(command_buffer, texture.image, VK_IMAGE_ASPECT_COLOR_BIT, levels - 1, 1,
                  VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
                  VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT,
                  VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT, VK_ACCESS_2_SHADER_SAMPLED_READ_BIT);

    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {0.0f, 0.0f, 0.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, layout, 0, 1, &set,
                            0, NULL);
    VkDeviceSize offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &offset);
    for (uint32_t i = 0; i < 9; i++) {
        vkCmdPushConstants(command_buffer, layout, VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                           sizeof(float), &lods[i]);
        vkCmdDraw(command_buffer, 6, 1, 6 * i, 0);
    }
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    vkDestroySampler(ctx->device, sampler, NULL);
    image_destroy(ctx, &color);
    image_destroy(ctx, &texture);
    buffer_destroy(ctx, &staging);
    buffer_destroy(ctx, &vertex_buffer);
}
