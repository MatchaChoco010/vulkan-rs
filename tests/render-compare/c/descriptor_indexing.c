/*
 * Four columns colored from descriptors of Vulkan 1.2 and 1.3: an inline uniform block, a
 * storage texel buffer, and a variable-sized, partially bound array of sampled images that
 * the shader indexes non-uniformly. The set layout, the pool, the allocation and one write
 * carry their parameters in pNext chains (binding flags, inline uniform block bindings, the
 * variable descriptor count, the inline data). One element of the array is written after
 * the commands that use the set are recorded (update after bind), and one is never written.
 */
#include "scenarios.h"

#define RGBA(r, g, b) (r) / 255.0f, (g) / 255.0f, (b) / 255.0f, 1.0f

static Image texel_image(Ctx *ctx, VkFormat format, const uint8_t *texel) {
    ImageDesc desc =
        image_desc(format, 1, 1, VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image image = image_create(ctx, &desc);
    image_upload(ctx, image.image, 1, 1, texel, 4, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    return image;
}

void scenario_descriptor_indexing(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float inline_colors[8] = {RGBA(204, 51, 51), RGBA(51, 204, 51)};
    static const uint8_t texel_colors[16] = {
        10, 20, 30, 255, 40, 50, 60, 255, 70, 80, 90, 255, 100, 110, 120, 255,
    };
    static const uint8_t texture_colors[3][4] = {
        {255, 128, 0, 255},
        {0, 128, 255, 255},
        {128, 0, 255, 255},
    };
    const uint32_t array_capacity = 8;
    const uint32_t array_count = 4;

    Buffer texel_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT,
                                           texel_colors, sizeof(texel_colors));
    VkBufferViewCreateInfo buffer_view_info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
        .buffer = texel_buffer.buffer,
        .format = format,
        .offset = 0,
        .range = VK_WHOLE_SIZE,
    };
    VkBufferView buffer_view;
    CHECK(vkCreateBufferView(ctx->device, &buffer_view_info, NULL, &buffer_view));
    Image textures[3];
    for (uint32_t i = 0; i < 3; i++) {
        textures[i] = texel_image(ctx, format, texture_colors[i]);
    }
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkSamplerCreateInfo sampler_info = {
        .sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
        .magFilter = VK_FILTER_NEAREST,
        .minFilter = VK_FILTER_NEAREST,
        .mipmapMode = VK_SAMPLER_MIPMAP_MODE_NEAREST,
        .addressModeU = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
        .addressModeV = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
        .addressModeW = VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE,
        .maxAnisotropy = 1.0f,
        .compareOp = VK_COMPARE_OP_NEVER,
        .borderColor = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
    };
    VkSampler sampler;
    CHECK(vkCreateSampler(ctx->device, &sampler_info, NULL, &sampler));

    VkDescriptorSetLayoutBinding bindings[] = {
        {0, VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK, sizeof(inline_colors),
         VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {1, VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER, 1, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {2, VK_DESCRIPTOR_TYPE_SAMPLER, 1, VK_SHADER_STAGE_FRAGMENT_BIT, &sampler},
        {3, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, array_capacity, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
    };
    VkDescriptorBindingFlags binding_flags[] = {
        0,
        0,
        0,
        VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT | VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT |
            VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT,
    };
    VkDescriptorSetLayoutBindingFlagsCreateInfo binding_flags_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
        .bindingCount = COUNT(binding_flags),
        .pBindingFlags = binding_flags,
    };
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .pNext = &binding_flags_info,
        .flags = VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT,
        .bindingCount = COUNT(bindings),
        .pBindings = bindings,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));

    VkDescriptorPoolSize pool_sizes[] = {
        {VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK, sizeof(inline_colors)},
        {VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER, 1},
        {VK_DESCRIPTOR_TYPE_SAMPLER, 1},
        {VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, array_capacity},
    };
    VkDescriptorPoolInlineUniformBlockCreateInfo inline_pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO,
        .maxInlineUniformBlockBindings = 1,
    };
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .pNext = &inline_pool_info,
        .flags = VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT,
        .maxSets = 1,
        .poolSizeCount = COUNT(pool_sizes),
        .pPoolSizes = pool_sizes,
    };
    VkDescriptorPool pool;
    CHECK(vkCreateDescriptorPool(ctx->device, &pool_info, NULL, &pool));
    VkDescriptorSetVariableDescriptorCountAllocateInfo variable_count_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
        .descriptorSetCount = 1,
        .pDescriptorCounts = &array_count,
    };
    VkDescriptorSetAllocateInfo allocate_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
        .pNext = &variable_count_info,
        .descriptorPool = pool,
        .descriptorSetCount = 1,
        .pSetLayouts = &set_layout,
    };
    VkDescriptorSet set;
    CHECK(vkAllocateDescriptorSets(ctx->device, &allocate_info, &set));

    VkWriteDescriptorSetInlineUniformBlock inline_data = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK,
        .dataSize = sizeof(inline_colors),
        .pData = inline_colors,
    };
    VkDescriptorImageInfo first_images[2] = {
        {VK_NULL_HANDLE, textures[0].view, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL},
        {VK_NULL_HANDLE, textures[1].view, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL},
    };
    VkWriteDescriptorSet writes[] = {
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .pNext = &inline_data,
            .dstSet = set,
            .dstBinding = 0,
            .dstArrayElement = 0,
            .descriptorCount = sizeof(inline_colors),
            .descriptorType = VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 1,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER,
            .pTexelBufferView = &buffer_view,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 3,
            .dstArrayElement = 0,
            .descriptorCount = 2,
            .descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            .pImageInfo = first_images,
        },
    };
    vkUpdateDescriptorSets(ctx->device, COUNT(writes), writes, 0, NULL);

    VkPipelineLayout layout = pipeline_layout(ctx, &set_layout, 1, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "descriptor_indexing.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "descriptor_indexing.frag.spv");
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
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {0.0f, 0.0f, 0.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, layout, 0, 1, &set,
                            0, NULL);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    VkDescriptorImageInfo late_image = {VK_NULL_HANDLE, textures[2].view,
                                        VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL};
    VkWriteDescriptorSet late_write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        .dstSet = set,
        .dstBinding = 3,
        .dstArrayElement = 3,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
        .pImageInfo = &late_image,
    };
    vkUpdateDescriptorSets(ctx->device, 1, &late_write, 0, NULL);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    vkDestroySampler(ctx->device, sampler, NULL);
    image_destroy(ctx, &color);
    for (uint32_t i = 3; i > 0; i--) {
        image_destroy(ctx, &textures[i - 1]);
    }
    vkDestroyBufferView(ctx->device, buffer_view, NULL);
    buffer_destroy(ctx, &texel_buffer);
}
