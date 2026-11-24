/*
 * Eight bands, each colored from another kind of descriptor: a dynamic uniform buffer, a
 * storage buffer, a uniform texel buffer, a sampled image with an immutable sampler, an
 * array of combined image samplers indexed by a push constant, and a push descriptor
 * (VK_KHR_push_descriptor). The set is filled with vkUpdateDescriptorSets, a
 * VkCopyDescriptorSet from a second set, and a descriptor update template.
 */
#include "scenarios.h"

#define RGBA(r, g, b) {(r) / 255.0f, (g) / 255.0f, (b) / 255.0f, 1.0f}

static Image single_color_image(Ctx *ctx, VkFormat format, uint32_t width, uint32_t height,
                                const uint8_t *texels) {
    ImageDesc desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image image = image_create(ctx, &desc);
    image_upload(ctx, image.image, width, height, texels, (size_t)width * height * 4,
                 VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    return image;
}

void scenario_descriptor_kinds(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    float uniform_colors[128] = {0};
    const float uniform_decoy[4] = RGBA(10, 10, 10);
    const float uniform_wanted[4] = RGBA(200, 30, 30);
    for (uint32_t i = 0; i < 4; i++) {
        uniform_colors[i] = uniform_decoy[i];
        uniform_colors[64 + i] = uniform_wanted[i];
    }
    const float storage_first[4] = RGBA(90, 60, 30);
    const float storage_wanted[4] = RGBA(30, 200, 30);
    float storage_data[8];
    for (uint32_t i = 0; i < 4; i++) {
        storage_data[i] = storage_first[i];
        storage_data[4 + i] = storage_wanted[i];
    }
    static const uint8_t texel_colors[16] = {
        10, 10, 10, 255, 20, 20, 20, 255, 30, 30, 200, 255, 40, 40, 40, 255,
    };
    static const uint8_t quad_texels[16] = {
        50, 50, 50, 255, 200, 200, 30, 255, 60, 60, 60, 255, 70, 70, 70, 255,
    };
    static const uint8_t single_texels[3][4] = {
        {120, 120, 120, 255},
        {80, 80, 80, 255},
        {30, 200, 200, 255},
    };
    const float pushed_color[4] = RGBA(200, 30, 200);
    const uint32_t push_index = 2;

    Buffer uniform_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
                                             uniform_colors, sizeof(uniform_colors));
    Buffer storage_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_STORAGE_BUFFER_BIT,
                                             storage_data, sizeof(storage_data));
    Buffer texel_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT,
                                           texel_colors, sizeof(texel_colors));
    Buffer pushed_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT,
                                            pushed_color, sizeof(pushed_color));
    VkBufferViewCreateInfo buffer_view_info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO,
        .buffer = texel_buffer.buffer,
        .format = format,
        .offset = 0,
        .range = VK_WHOLE_SIZE,
    };
    VkBufferView buffer_view;
    CHECK(vkCreateBufferView(ctx->device, &buffer_view_info, NULL, &buffer_view));
    Image quad_image = single_color_image(ctx, format, 2, 2, quad_texels);
    Image singles[3];
    for (uint32_t i = 0; i < 3; i++) {
        singles[i] = single_color_image(ctx, format, 1, 1, single_texels[i]);
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
    VkSampler immutable_sampler;
    CHECK(vkCreateSampler(ctx->device, &sampler_info, NULL, &immutable_sampler));
    VkSampler sampler;
    CHECK(vkCreateSampler(ctx->device, &sampler_info, NULL, &sampler));

    VkDescriptorSetLayoutBinding bindings[] = {
        {0, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC, 1, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {1, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 1, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {2, VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER, 1, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {3, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, 1, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
        {4, VK_DESCRIPTOR_TYPE_SAMPLER, 1, VK_SHADER_STAGE_FRAGMENT_BIT, &immutable_sampler},
        {5, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 3, VK_SHADER_STAGE_FRAGMENT_BIT, NULL},
    };
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = COUNT(bindings),
        .pBindings = bindings,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorSetLayoutBinding pushed_binding = {0, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, 1,
                                                   VK_SHADER_STAGE_FRAGMENT_BIT, NULL};
    VkDescriptorSetLayoutCreateInfo pushed_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .flags = VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR,
        .bindingCount = 1,
        .pBindings = &pushed_binding,
    };
    VkDescriptorSetLayout pushed_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &pushed_layout_info, NULL, &pushed_layout));

    VkDescriptorPoolSize pool_sizes[] = {
        {VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC, 2},
        {VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, 2},
        {VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER, 2},
        {VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, 2},
        {VK_DESCRIPTOR_TYPE_SAMPLER, 2},
        {VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 6},
    };
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .maxSets = 2,
        .poolSizeCount = COUNT(pool_sizes),
        .pPoolSizes = pool_sizes,
    };
    VkDescriptorPool pool;
    CHECK(vkCreateDescriptorPool(ctx->device, &pool_info, NULL, &pool));
    VkDescriptorSetLayout set_layouts[2] = {set_layout, set_layout};
    VkDescriptorSetAllocateInfo allocate_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
        .descriptorPool = pool,
        .descriptorSetCount = 2,
        .pSetLayouts = set_layouts,
    };
    VkDescriptorSet sets[2];
    CHECK(vkAllocateDescriptorSets(ctx->device, &allocate_info, sets));
    VkDescriptorSet set = sets[0];
    VkDescriptorSet source = sets[1];

    VkDescriptorBufferInfo storage_info = {storage_buffer.buffer, 0, VK_WHOLE_SIZE};
    VkWriteDescriptorSet source_write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        .dstSet = source,
        .dstBinding = 1,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_STORAGE_BUFFER,
        .pBufferInfo = &storage_info,
    };
    vkUpdateDescriptorSets(ctx->device, 1, &source_write, 0, NULL);

    VkDescriptorImageInfo sampled_info = {VK_NULL_HANDLE, quad_image.view,
                                          VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL};
    VkDescriptorImageInfo combined_infos[3];
    for (uint32_t i = 0; i < 3; i++) {
        VkDescriptorImageInfo info = {sampler, singles[i].view,
                                      VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL};
        combined_infos[i] = info;
    }
    VkWriteDescriptorSet writes[] = {
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 2,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER,
            .pTexelBufferView = &buffer_view,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 3,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,
            .pImageInfo = &sampled_info,
        },
        {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = 5,
            .descriptorCount = 3,
            .descriptorType = VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
            .pImageInfo = combined_infos,
        },
    };
    VkCopyDescriptorSet copy = {
        .sType = VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET,
        .srcSet = source,
        .srcBinding = 1,
        .dstSet = set,
        .dstBinding = 1,
        .descriptorCount = 1,
    };
    vkUpdateDescriptorSets(ctx->device, COUNT(writes), writes, 1, &copy);

    VkPushConstantRange range = {VK_SHADER_STAGE_FRAGMENT_BIT, 0, sizeof(push_index)};
    VkDescriptorSetLayout pipeline_set_layouts[2] = {set_layout, pushed_layout};
    VkPipelineLayout layout = pipeline_layout(ctx, pipeline_set_layouts, 2, &range, 1);

    VkDescriptorUpdateTemplateEntry template_entry = {
        .dstBinding = 0,
        .dstArrayElement = 0,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC,
        .offset = 0,
        .stride = sizeof(VkDescriptorBufferInfo),
    };
    VkDescriptorUpdateTemplateCreateInfo template_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO,
        .descriptorUpdateEntryCount = 1,
        .pDescriptorUpdateEntries = &template_entry,
        .templateType = VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,
        .descriptorSetLayout = set_layout,
    };
    VkDescriptorUpdateTemplate update_template;
    CHECK(vkCreateDescriptorUpdateTemplate(ctx->device, &template_info, NULL, &update_template));
    VkDescriptorBufferInfo uniform_info = {uniform_buffer.buffer, 0, 4 * sizeof(float)};
    vkUpdateDescriptorSetWithTemplate(ctx->device, set, update_template, &uniform_info);

    VkShaderModule vertex = shader_module(ctx, "descriptor_kinds.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "descriptor_kinds.frag.spv");
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

    PFN_vkCmdPushDescriptorSetKHR push_descriptor_set =
        (PFN_vkCmdPushDescriptorSetKHR)vkGetDeviceProcAddr(ctx->device,
                                                          "vkCmdPushDescriptorSetKHR");
    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {0.0f, 0.0f, 0.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    const uint32_t dynamic_offset = 256;
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, layout, 0, 1, &set,
                            1, &dynamic_offset);
    VkDescriptorBufferInfo pushed_info = {pushed_buffer.buffer, 0, VK_WHOLE_SIZE};
    VkWriteDescriptorSet pushed_write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        .dstBinding = 0,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER,
        .pBufferInfo = &pushed_info,
    };
    push_descriptor_set(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, layout, 1, 1,
                        &pushed_write);
    vkCmdPushConstants(command_buffer, layout, VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                       sizeof(push_index), &push_index);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyDescriptorUpdateTemplate(ctx->device, update_template, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, pushed_layout, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    vkDestroySampler(ctx->device, sampler, NULL);
    vkDestroySampler(ctx->device, immutable_sampler, NULL);
    image_destroy(ctx, &color);
    for (uint32_t i = 0; i < 3; i++) {
        image_destroy(ctx, &singles[i]);
    }
    image_destroy(ctx, &quad_image);
    vkDestroyBufferView(ctx->device, buffer_view, NULL);
    buffer_destroy(ctx, &pushed_buffer);
    buffer_destroy(ctx, &texel_buffer);
    buffer_destroy(ctx, &storage_buffer);
    buffer_destroy(ctx, &uniform_buffer);
}
