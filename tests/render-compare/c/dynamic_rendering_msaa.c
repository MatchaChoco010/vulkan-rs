/*
 * Dynamic rendering to a color and a depth attachment of four samples, each with a resolve
 * attachment (the average of the color samples, sample zero of the depth). The pipelines
 * take the viewport and scissor with their counts, the depth test, depth write, depth
 * compare operator, rasterizer discard and more as dynamic state. Two pipelines have a
 * sample mask of two samples, one with sample zero and one without, and write depth, so
 * that the resolved depth is the drawn one under the first and the cleared one under the
 * second: no other depth resolve mode gives both. Between the draws vkCmdClearAttachments
 * clears a rectangle of both attachments, and afterwards vkCmdResolveImage resolves the
 * multisampled color image once more into a third image.
 */
#include "scenarios.h"

typedef struct {
    float color[4];
    float rect[4];
    float depth;
} Push;

#define NDC(pixel) ((float)(pixel) / 128.0f - 1.0f)

static VkPipeline msaa_pipeline(Ctx *ctx, VkShaderModule vertex, VkShaderModule fragment,
                                VkPipelineLayout layout, VkFormat color_format,
                                VkFormat depth_format, VkSampleMask sample_mask) {
    VkPipelineShaderStageCreateInfo stages[] = {
        {
            .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            .stage = VK_SHADER_STAGE_VERTEX_BIT,
            .module = vertex,
            .pName = "main",
        },
        {
            .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            .stage = VK_SHADER_STAGE_FRAGMENT_BIT,
            .module = fragment,
            .pName = "main",
        },
    };
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    };
    VkPipelineInputAssemblyStateCreateInfo input_assembly = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
    };
    VkPipelineViewportStateCreateInfo viewport_state = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
    };
    VkPipelineRasterizationStateCreateInfo rasterization = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        .polygonMode = VK_POLYGON_MODE_FILL,
        .cullMode = VK_CULL_MODE_NONE,
        .frontFace = VK_FRONT_FACE_COUNTER_CLOCKWISE,
        .lineWidth = 1.0f,
    };
    VkPipelineMultisampleStateCreateInfo multisample = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        .rasterizationSamples = VK_SAMPLE_COUNT_4_BIT,
        .sampleShadingEnable = VK_TRUE,
        .minSampleShading = 1.0f,
        .pSampleMask = &sample_mask,
    };
    VkPipelineDepthStencilStateCreateInfo depth_stencil = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        .depthCompareOp = VK_COMPARE_OP_ALWAYS,
        .maxDepthBounds = 1.0f,
    };
    VkPipelineColorBlendStateCreateInfo color_blend = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        .attachmentCount = 1,
        .pAttachments = &BLEND_DISABLED,
    };
    static const VkDynamicState dynamic_states[] = {
        VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT,       VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT,
        VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE,         VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE,
        VK_DYNAMIC_STATE_DEPTH_COMPARE_OP,          VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE,
        VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE,       VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE,
        VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE,         VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE,
    };
    VkPipelineDynamicStateCreateInfo dynamic = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        .dynamicStateCount = COUNT(dynamic_states),
        .pDynamicStates = dynamic_states,
    };
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &color_format,
        .depthAttachmentFormat = depth_format,
    };
    VkGraphicsPipelineCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        .pNext = &rendering,
        .stageCount = COUNT(stages),
        .pStages = stages,
        .pVertexInputState = &vertex_input,
        .pInputAssemblyState = &input_assembly,
        .pViewportState = &viewport_state,
        .pRasterizationState = &rasterization,
        .pMultisampleState = &multisample,
        .pDepthStencilState = &depth_stencil,
        .pColorBlendState = &color_blend,
        .pDynamicState = &dynamic,
        .layout = layout,
    };
    VkPipeline pipeline;
    CHECK(vkCreateGraphicsPipelines(ctx->device, VK_NULL_HANDLE, 1, &info, NULL, &pipeline));
    return pipeline;
}

static void draw_rect(VkCommandBuffer command_buffer, VkPipelineLayout layout, const Push *push) {
    vkCmdPushConstants(command_buffer, layout,
                       VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                       sizeof(*push), push);
    vkCmdDraw(command_buffer, 6, 1, 0, 0);
}

void scenario_dynamic_rendering_msaa(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    const VkFormat depth_format = VK_FORMAT_D32_SFLOAT;
    const VkImageAspectFlags color_aspect = VK_IMAGE_ASPECT_COLOR_BIT;
    const VkImageAspectFlags depth_aspect = VK_IMAGE_ASPECT_DEPTH_BIT;

    ImageDesc samples_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   color_aspect);
    samples_desc.info.samples = VK_SAMPLE_COUNT_4_BIT;
    Image samples = image_create(ctx, &samples_desc);
    ImageDesc depth_samples_desc = image_desc(
        depth_format, width, height, VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, depth_aspect);
    depth_samples_desc.info.samples = VK_SAMPLE_COUNT_4_BIT;
    Image depth_samples = image_create(ctx, &depth_samples_desc);
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   color_aspect);
    Image color = image_create(ctx, &color_desc);
    ImageDesc depth_desc =
        image_desc(depth_format, width, height,
                   VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   depth_aspect);
    Image depth = image_create(ctx, &depth_desc);
    ImageDesc resolved_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT |
                       VK_IMAGE_USAGE_SAMPLED_BIT,
                   color_aspect);
    Image resolved = image_create(ctx, &resolved_desc);

    const VkShaderStageFlags push_stages =
        VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT;
    VkPushConstantRange range = {push_stages, 0, sizeof(Push)};
    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, &range, 1);
    VkShaderModule vertex = shader_module(ctx, "dynamic_rendering_msaa.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "dynamic_rendering_msaa.frag.spv");
    VkPipeline all_samples =
        msaa_pipeline(ctx, vertex, fragment, layout, format, depth_format, 0xf);
    VkPipeline two_samples =
        msaa_pipeline(ctx, vertex, fragment, layout, format, depth_format, 0x5);
    VkPipeline other_samples =
        msaa_pipeline(ctx, vertex, fragment, layout, format, depth_format, 0xa);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, samples.image);
    color_attachment_begin(command_buffer, color.image);
    image_barrier(command_buffer, depth_samples.image, depth_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                  VK_PIPELINE_STAGE_2_NONE, VK_ACCESS_2_NONE,
                  VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT,
                  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT);
    image_barrier(command_buffer, depth.image, depth_aspect, 0, 1, VK_IMAGE_LAYOUT_UNDEFINED,
                  VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL, VK_PIPELINE_STAGE_2_NONE,
                  VK_ACCESS_2_NONE, VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT,
                  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT);
    VkRenderingAttachmentInfo color_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = samples.view,
        .imageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        .resolveMode = VK_RESOLVE_MODE_AVERAGE_BIT,
        .resolveImageView = color.view,
        .resolveImageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .clearValue = {.color = {.float32 = {32.0f / 255.0f, 32.0f / 255.0f, 48.0f / 255.0f,
                                             1.0f}}},
    };
    VkRenderingAttachmentInfo depth_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = depth_samples.view,
        .imageLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        .resolveMode = VK_RESOLVE_MODE_SAMPLE_ZERO_BIT,
        .resolveImageView = depth.view,
        .resolveImageLayout = VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
        .clearValue = depth_stencil_clear(1.0f, 0),
    };
    VkRenderingInfo rendering_info = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_INFO,
        .renderArea = {{0, 0}, {width, height}},
        .layerCount = 1,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_attachment,
        .pDepthAttachment = &depth_attachment,
    };
    vkCmdBeginRendering(command_buffer, &rendering_info);
    VkViewport viewport = {0.0f, 0.0f, (float)width, (float)height, 0.0f, 1.0f};
    VkRect2D scissor = {{0, 0}, {width, height}};
    vkCmdSetViewportWithCount(command_buffer, 1, &viewport);
    vkCmdSetScissorWithCount(command_buffer, 1, &scissor);
    vkCmdSetDepthTestEnable(command_buffer, VK_TRUE);
    vkCmdSetDepthWriteEnable(command_buffer, VK_TRUE);
    vkCmdSetDepthCompareOp(command_buffer, VK_COMPARE_OP_LESS);
    vkCmdSetDepthBoundsTestEnable(command_buffer, VK_FALSE);
    vkCmdSetStencilTestEnable(command_buffer, VK_FALSE);
    vkCmdSetRasterizerDiscardEnable(command_buffer, VK_FALSE);
    vkCmdSetDepthBiasEnable(command_buffer, VK_FALSE);
    vkCmdSetPrimitiveRestartEnable(command_buffer, VK_FALSE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, all_samples);
    const Push red = {{1.0f, 0.0f, 0.0f, 1.0f}, {NDC(16), NDC(16), NDC(144), NDC(144)}, 0.5f};
    draw_rect(command_buffer, layout, &red);
    const Push green = {{0.0f, 1.0f, 0.0f, 1.0f}, {NDC(80), NDC(80), NDC(208), NDC(208)}, 0.75f};
    draw_rect(command_buffer, layout, &green);
    vkCmdSetRasterizerDiscardEnable(command_buffer, VK_TRUE);
    const Push white = {{1.0f, 1.0f, 1.0f, 1.0f}, {NDC(0), NDC(0), NDC(256), NDC(256)}, 0.0f};
    draw_rect(command_buffer, layout, &white);
    vkCmdSetRasterizerDiscardEnable(command_buffer, VK_FALSE);
    vkCmdSetDepthCompareOp(command_buffer, VK_COMPARE_OP_GREATER);
    vkCmdSetDepthWriteEnable(command_buffer, VK_FALSE);
    const Push blue = {{0.0f, 0.0f, 1.0f, 1.0f}, {NDC(112), NDC(48), NDC(240), NDC(112)}, 0.625f};
    draw_rect(command_buffer, layout, &blue);
    VkClearAttachment clears[2] = {
        {
            .aspectMask = color_aspect,
            .colorAttachment = 0,
            .clearValue = {.color = {.float32 = {1.0f, 0.0f, 1.0f, 1.0f}}},
        },
        {
            .aspectMask = depth_aspect,
            .clearValue = depth_stencil_clear(0.25f, 0),
        },
    };
    VkClearRect clear_rect = {{{176, 16}, {64, 32}}, 0, 1};
    vkCmdClearAttachments(command_buffer, COUNT(clears), clears, 1, &clear_rect);
    vkCmdSetDepthCompareOp(command_buffer, VK_COMPARE_OP_LESS);
    vkCmdSetDepthWriteEnable(command_buffer, VK_TRUE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, two_samples);
    const Push yellow = {{1.0f, 1.0f, 0.0f, 1.0f},
                         {NDC(16), NDC(176), NDC(80), NDC(240)},
                         0.375f};
    draw_rect(command_buffer, layout, &yellow);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, other_samples);
    const Push cyan = {{0.0f, 1.0f, 1.0f, 1.0f},
                       {NDC(208), NDC(208), NDC(240), NDC(240)},
                       0.375f};
    draw_rect(command_buffer, layout, &cyan);
    vkCmdSetDepthTestEnable(command_buffer, VK_FALSE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, all_samples);
    const Push untested = {{1.0f, 1.0f, 1.0f, 1.0f}, {NDC(32), NDC(32), NDC(64), NDC(64)}, 0.875f};
    draw_rect(command_buffer, layout, &untested);
    vkCmdEndRendering(command_buffer);

    image_barrier(command_buffer, samples.image, color_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_RESOLVE_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
    color_attachment_end(command_buffer, color.image);
    image_barrier(command_buffer, depth.image, depth_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
                  VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT,
                  VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
    image_barrier(command_buffer, resolved.image, color_aspect, 0, 1, VK_IMAGE_LAYOUT_UNDEFINED,
                  VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_PIPELINE_STAGE_2_NONE,
                  VK_ACCESS_2_NONE, VK_PIPELINE_STAGE_2_RESOLVE_BIT,
                  VK_ACCESS_2_TRANSFER_WRITE_BIT);
    VkImageResolve region = {
        .srcSubresource = {color_aspect, 0, 0, 1},
        .dstSubresource = {color_aspect, 0, 0, 1},
        .extent = {width, height, 1},
    };
    vkCmdResolveImage(command_buffer, samples.image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                      resolved.image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);
    image_barrier(command_buffer, resolved.image, color_aspect, 0, 1,
                  VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_RESOLVE_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT,
                  VK_PIPELINE_STAGE_2_COPY_BIT, VK_ACCESS_2_TRANSFER_READ_BIT);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, color_aspect, width, height, 4, "color", "rgba8");
    readback(ctx, depth.image, depth_aspect, width, height, 4, "depth", "d32f");
    readback(ctx, resolved.image, color_aspect, width, height, 4, "resolved", "rgba8");

    vkDestroyPipeline(ctx->device, other_samples, NULL);
    vkDestroyPipeline(ctx->device, two_samples, NULL);
    vkDestroyPipeline(ctx->device, all_samples, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &resolved);
    image_destroy(ctx, &depth);
    image_destroy(ctx, &color);
    image_destroy(ctx, &depth_samples);
    image_destroy(ctx, &samples);
}
