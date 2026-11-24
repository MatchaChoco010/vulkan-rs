/*
 * A render pass with two subpasses: the first draws three triangles into three color
 * attachments at once, the second reads them back as input attachments and combines them
 * into a fourth attachment. All four attachments are read back.
 */
#include "scenarios.h"

static VkAttachmentDescription color_attachment(VkFormat format) {
    VkAttachmentDescription attachment = {
        .format = format,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
        .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
        .finalLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
    };
    return attachment;
}

void scenario_multiple_render_targets(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const char *const names[4] = {"albedo", "normal", "material", "color"};
    static const float vertices[] = {
        -0.9f,  -0.8f, 1.0f, 0.5f, 0.0f, 0.7f,  -0.6f, 1.0f, 0.5f, 0.0f,
        -0.3f,  0.9f,  1.0f, 0.5f, 0.0f, 0.95f, 0.1f,  0.0f, 0.75f, 0.25f,
        0.2f,   0.95f, 0.0f, 0.75f, 0.25f, -0.1f, -0.2f, 0.0f, 0.75f, 0.25f,
        -0.95f, 0.7f,  0.25f, 0.0f, 1.0f, 0.9f,  0.85f, 0.25f, 0.0f, 1.0f,
        0.9f,   0.8f,  0.25f, 0.0f, 1.0f,
    };

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    Image targets[4];
    for (uint32_t i = 0; i < 4; i++) {
        VkImageUsageFlags usage =
            VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT;
        if (i < 3) {
            usage |= VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT;
        }
        ImageDesc desc = image_desc(format, width, height, usage, VK_IMAGE_ASPECT_COLOR_BIT);
        targets[i] = image_create(ctx, &desc);
    }

    VkAttachmentDescription attachments[4];
    for (uint32_t i = 0; i < 4; i++) {
        attachments[i] = color_attachment(format);
    }
    VkAttachmentReference gbuffer_outputs[3] = {
        {0, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL},
        {1, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL},
        {2, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL},
    };
    VkAttachmentReference gbuffer_inputs[3] = {
        {0, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL},
        {1, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL},
        {2, VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL},
    };
    VkAttachmentReference composite_output = {3, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    VkSubpassDescription subpasses[2] = {
        {
            .pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS,
            .colorAttachmentCount = 3,
            .pColorAttachments = gbuffer_outputs,
        },
        {
            .pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS,
            .inputAttachmentCount = 3,
            .pInputAttachments = gbuffer_inputs,
            .colorAttachmentCount = 1,
            .pColorAttachments = &composite_output,
        },
    };
    VkSubpassDependency dependencies[2] = {
        {
            .srcSubpass = 0,
            .dstSubpass = 1,
            .srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            .dstStageMask = VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            .srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            .dstAccessMask = VK_ACCESS_INPUT_ATTACHMENT_READ_BIT,
            .dependencyFlags = VK_DEPENDENCY_BY_REGION_BIT,
        },
        {
            .srcSubpass = 1,
            .dstSubpass = VK_SUBPASS_EXTERNAL,
            .srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT |
                            VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT,
            .dstStageMask = VK_PIPELINE_STAGE_TRANSFER_BIT,
            .srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            .dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT,
        },
    };
    VkRenderPassCreateInfo render_pass_info = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        .attachmentCount = COUNT(attachments),
        .pAttachments = attachments,
        .subpassCount = COUNT(subpasses),
        .pSubpasses = subpasses,
        .dependencyCount = COUNT(dependencies),
        .pDependencies = dependencies,
    };
    VkRenderPass render_pass;
    CHECK(vkCreateRenderPass(ctx->device, &render_pass_info, NULL, &render_pass));
    VkImageView views[4];
    for (uint32_t i = 0; i < 4; i++) {
        views[i] = targets[i].view;
    }
    VkFramebufferCreateInfo framebuffer_info = {
        .sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        .renderPass = render_pass,
        .attachmentCount = COUNT(views),
        .pAttachments = views,
        .width = width,
        .height = height,
        .layers = 1,
    };
    VkFramebuffer framebuffer;
    CHECK(vkCreateFramebuffer(ctx->device, &framebuffer_info, NULL, &framebuffer));

    VkDescriptorSetLayoutBinding bindings[3];
    for (uint32_t i = 0; i < 3; i++) {
        VkDescriptorSetLayoutBinding binding = {i, VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT, 1,
                                                VK_SHADER_STAGE_FRAGMENT_BIT, NULL};
        bindings[i] = binding;
    }
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = COUNT(bindings),
        .pBindings = bindings,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorPoolSize pool_size = {VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT, 3};
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
    VkDescriptorImageInfo image_infos[3];
    VkWriteDescriptorSet writes[3];
    for (uint32_t i = 0; i < 3; i++) {
        VkDescriptorImageInfo image_info = {VK_NULL_HANDLE, targets[i].view,
                                            VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL};
        image_infos[i] = image_info;
        VkWriteDescriptorSet write = {
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = i,
            .descriptorCount = 1,
            .descriptorType = VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
            .pImageInfo = &image_infos[i],
        };
        writes[i] = write;
    }
    vkUpdateDescriptorSets(ctx->device, COUNT(writes), writes, 0, NULL);

    VkPipelineLayout gbuffer_layout = pipeline_layout(ctx, NULL, 0, NULL, 0);
    VkPipelineLayout composite_layout = pipeline_layout(ctx, &set_layout, 1, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "multiple_render_targets.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "multiple_render_targets.frag.spv");
    VkShaderModule composite_vertex =
        shader_module(ctx, "multiple_render_targets.composite.vert.spv");
    VkShaderModule composite_fragment =
        shader_module(ctx, "multiple_render_targets.composite.frag.spv");
    VkVertexInputBindingDescription binding = {0, 5 * sizeof(float),
                                               VK_VERTEX_INPUT_RATE_VERTEX};
    VkVertexInputAttributeDescription attributes[] = {
        {0, 0, VK_FORMAT_R32G32_SFLOAT, 0},
        {1, 0, VK_FORMAT_R32G32B32_SFLOAT, 2 * sizeof(float)},
    };
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = 1,
        .pVertexBindingDescriptions = &binding,
        .vertexAttributeDescriptionCount = COUNT(attributes),
        .pVertexAttributeDescriptions = attributes,
    };
    const VkPipelineColorBlendAttachmentState gbuffer_blend[3] = {BLEND_DISABLED, BLEND_DISABLED,
                                                                  BLEND_DISABLED};
    GraphicsDesc gbuffer_desc = {
        .vertex = vertex,
        .fragment = fragment,
        .vertex_entry = "main",
        .fragment_entry = "main",
        .vertex_input = &vertex_input,
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .blend_attachments = gbuffer_blend,
        .blend_attachment_count = COUNT(gbuffer_blend),
        .layout = gbuffer_layout,
        .render_pass = render_pass,
        .subpass = 0,
        .width = width,
        .height = height,
    };
    VkPipeline gbuffer_pipeline = graphics_pipeline(ctx, &gbuffer_desc);
    GraphicsDesc composite_desc = {
        .vertex = composite_vertex,
        .fragment = composite_fragment,
        .vertex_entry = "main",
        .fragment_entry = "main",
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .blend_attachments = &BLEND_DISABLED,
        .blend_attachment_count = 1,
        .layout = composite_layout,
        .render_pass = render_pass,
        .subpass = 1,
        .width = width,
        .height = height,
    };
    VkPipeline composite_pipeline = graphics_pipeline(ctx, &composite_desc);
    vkDestroyShaderModule(ctx->device, composite_fragment, NULL);
    vkDestroyShaderModule(ctx->device, composite_vertex, NULL);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    VkClearValue clears[4] = {
        {.color = {.float32 = {0.0f, 0.0f, 0.0f, 1.0f}}},
        {.color = {.float32 = {0.5f, 0.5f, 1.0f, 1.0f}}},
        {.color = {.float32 = {0.25f, 0.25f, 0.25f, 1.0f}}},
        {.color = {.float32 = {1.0f, 0.0f, 1.0f, 1.0f}}},
    };
    VkRenderPassBeginInfo begin = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        .renderPass = render_pass,
        .framebuffer = framebuffer,
        .renderArea = {{0, 0}, {width, height}},
        .clearValueCount = COUNT(clears),
        .pClearValues = clears,
    };
    vkCmdBeginRenderPass(command_buffer, &begin, VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, gbuffer_pipeline);
    VkDeviceSize offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &offset);
    vkCmdDraw(command_buffer, 9, 1, 0, 0);
    vkCmdNextSubpass(command_buffer, VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, composite_pipeline);
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, composite_layout, 0,
                            1, &set, 0, NULL);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRenderPass(command_buffer);
    commands_submit(ctx, command_buffer);
    for (uint32_t i = 0; i < 4; i++) {
        readback(ctx, targets[i].image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, names[i],
                 "rgba8");
    }

    vkDestroyPipeline(ctx->device, composite_pipeline, NULL);
    vkDestroyPipeline(ctx->device, gbuffer_pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, composite_layout, NULL);
    vkDestroyPipelineLayout(ctx->device, gbuffer_layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    vkDestroyFramebuffer(ctx->device, framebuffer, NULL);
    vkDestroyRenderPass(ctx->device, render_pass, NULL);
    for (uint32_t i = 0; i < 4; i++) {
        image_destroy(ctx, &targets[i]);
    }
    buffer_destroy(ctx, &vertex_buffer);
}
