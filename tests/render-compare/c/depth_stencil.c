/*
 * Depth and stencil with dynamic rendering: a red rectangle at depth 0.5 writes stencil
 * 1, a green rectangle whose depth grows from left to right is hidden where it lies behind
 * the red one, and a blue band without a depth test is drawn only where the stencil is 1.
 * The color, the depth and the stencil are read back.
 */
#include "scenarios.h"

static VkPipelineDepthStencilStateCreateInfo depth_stencil_state(VkBool32 depth_test,
                                                                 VkBool32 depth_write,
                                                                 VkBool32 stencil_test,
                                                                 VkCompareOp stencil_compare,
                                                                 VkStencilOp stencil_pass) {
    VkStencilOpState stencil = {
        .failOp = VK_STENCIL_OP_KEEP,
        .passOp = stencil_pass,
        .depthFailOp = VK_STENCIL_OP_KEEP,
        .compareOp = stencil_compare,
        .compareMask = 0xff,
        .writeMask = 0xff,
        .reference = 0,
    };
    VkPipelineDepthStencilStateCreateInfo state = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        .depthTestEnable = depth_test,
        .depthWriteEnable = depth_write,
        .depthCompareOp = VK_COMPARE_OP_LESS,
        .stencilTestEnable = stencil_test,
        .front = stencil,
        .back = stencil,
        .maxDepthBounds = 1.0f,
    };
    return state;
}

void scenario_depth_stencil(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat color_format = VK_FORMAT_R8G8B8A8_UNORM;
    const VkFormat depth_format = VK_FORMAT_D32_SFLOAT_S8_UINT;
    const VkImageAspectFlags depth_stencil_aspect =
        VK_IMAGE_ASPECT_DEPTH_BIT | VK_IMAGE_ASPECT_STENCIL_BIT;
    static const float vertices[] = {
        -0.75f, -0.75f, 0.5f, 0.25f, -0.75f, 0.5f, 0.25f, 0.75f, 0.5f,
        0.25f,  0.75f,  0.5f, -0.75f, 0.75f, 0.5f, -0.75f, -0.75f, 0.5f,
        -0.5f,  -0.5f,  0.3f, 0.5f,  -0.5f,  0.7f, 0.5f,  0.5f,  0.7f,
        0.5f,   0.5f,   0.7f, -0.5f, 0.5f,   0.3f, -0.5f, -0.5f, 0.3f,
        -1.0f,  0.25f,  0.0f, 1.0f,  0.25f,  0.0f, 1.0f,  0.5f,  0.0f,
        1.0f,   0.5f,   0.0f, -1.0f, 0.5f,   0.0f, -1.0f, 0.25f, 0.0f,
    };

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc color_desc =
        image_desc(color_format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);
    ImageDesc depth_desc = image_desc(
        depth_format, width, height,
        VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
        depth_stencil_aspect);
    Image depth = image_create(ctx, &depth_desc);

    VkPushConstantRange range = {VK_SHADER_STAGE_FRAGMENT_BIT, 0, 4 * sizeof(float)};
    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, &range, 1);
    VkShaderModule vertex = shader_module(ctx, "depth_stencil.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "depth_stencil.frag.spv");
    VkVertexInputBindingDescription binding = {0, 3 * sizeof(float),
                                               VK_VERTEX_INPUT_RATE_VERTEX};
    VkVertexInputAttributeDescription attribute = {0, 0, VK_FORMAT_R32G32B32_SFLOAT, 0};
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = 1,
        .pVertexBindingDescriptions = &binding,
        .vertexAttributeDescriptionCount = 1,
        .pVertexAttributeDescriptions = &attribute,
    };
    static const VkDynamicState dynamic_states[] = {VK_DYNAMIC_STATE_STENCIL_REFERENCE};
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &color_format,
        .depthAttachmentFormat = depth_format,
        .stencilAttachmentFormat = depth_format,
    };
    VkPipelineDepthStencilStateCreateInfo states[] = {
        depth_stencil_state(VK_TRUE, VK_TRUE, VK_TRUE, VK_COMPARE_OP_ALWAYS,
                            VK_STENCIL_OP_REPLACE),
        depth_stencil_state(VK_TRUE, VK_TRUE, VK_FALSE, VK_COMPARE_OP_ALWAYS,
                            VK_STENCIL_OP_KEEP),
        depth_stencil_state(VK_FALSE, VK_FALSE, VK_TRUE, VK_COMPARE_OP_EQUAL,
                            VK_STENCIL_OP_KEEP),
    };
    VkPipeline pipelines[3];
    for (uint32_t i = 0; i < 3; i++) {
        GraphicsDesc pipeline_desc = {
            .vertex = vertex,
            .fragment = fragment,
            .vertex_entry = "main",
            .fragment_entry = "main",
            .vertex_input = &vertex_input,
            .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            .samples = VK_SAMPLE_COUNT_1_BIT,
            .depth_stencil = &states[i],
            .blend_attachments = &BLEND_DISABLED,
            .blend_attachment_count = 1,
            .dynamic_states = dynamic_states,
            .dynamic_state_count = COUNT(dynamic_states),
            .layout = layout,
            .rendering = &rendering,
            .width = width,
            .height = height,
        };
        pipelines[i] = graphics_pipeline(ctx, &pipeline_desc);
    }
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    image_barrier(command_buffer, depth.image, depth_stencil_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                  VK_PIPELINE_STAGE_2_NONE, VK_ACCESS_2_NONE,
                  VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT |
                      VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT,
                  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT |
                      VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT);
    VkRenderingAttachmentInfo color_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = color.view,
        .imageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .clearValue = {.color = {.float32 = {16.0f / 255.0f, 16.0f / 255.0f, 16.0f / 255.0f,
                                             1.0f}}},
    };
    VkRenderingAttachmentInfo depth_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = depth.view,
        .imageLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .clearValue = {.depthStencil = {1.0f, 0}},
    };
    VkRenderingInfo rendering_info = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_INFO,
        .renderArea = {{0, 0}, {width, height}},
        .layerCount = 1,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_attachment,
        .pDepthAttachment = &depth_attachment,
        .pStencilAttachment = &depth_attachment,
    };
    vkCmdBeginRendering(command_buffer, &rendering_info);
    VkDeviceSize offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &offset);
    vkCmdSetStencilReference(command_buffer, VK_STENCIL_FACE_FRONT_AND_BACK, 1);
    static const float colors[3][4] = {
        {1.0f, 0.0f, 0.0f, 1.0f},
        {0.0f, 1.0f, 0.0f, 1.0f},
        {0.0f, 0.0f, 1.0f, 1.0f},
    };
    for (uint32_t i = 0; i < 3; i++) {
        vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipelines[i]);
        vkCmdPushConstants(command_buffer, layout, VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                           sizeof(colors[i]), colors[i]);
        vkCmdDraw(command_buffer, 6, 1, 6 * i, 0);
    }
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    image_barrier(command_buffer, depth.image, depth_stencil_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                  VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT,
                  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");
    readback(ctx, depth.image, VK_IMAGE_ASPECT_DEPTH_BIT, width, height, 4, "depth", "d32f");
    readback(ctx, depth.image, VK_IMAGE_ASPECT_STENCIL_BIT, width, height, 1, "stencil", "s8");

    for (uint32_t i = 0; i < 3; i++) {
        vkDestroyPipeline(ctx->device, pipelines[i], NULL);
    }
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &depth);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &vertex_buffer);
}
