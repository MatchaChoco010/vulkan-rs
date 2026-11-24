/*
 * Based on dEQP-VK.api.smoke.triangle of the Khronos Vulkan Conformance Test Suite
 * (https://github.com/KhronosGroup/VK-GL-CTS,
 * external/vulkancts/modules/vulkan/api/vktApiSmokeTests.cpp, renderTriangleTest).
 *
 * Copyright (c) 2015 Google Inc.
 * SPDX-License-Identifier: Apache-2.0
 *
 * Taken from the test: the size and format of the color attachment, the clear color, the
 * vertices, the vertex format and the shaders (ported to Slang in
 * shaders/cts_smoke_triangle.slang), and the order of the commands: a render pass whose
 * attachment ends in COLOR_ATTACHMENT_OPTIMAL, then a barrier to TRANSFER_SRC_OPTIMAL and a
 * copy to a buffer. Changed: written with the helpers of render.h instead of the CTS
 * framework, the barriers use VK_KHR_synchronization2, and the host-to-vertex-input
 * barrier is left out because the vertex data is written before the submission.
 */
#include "scenarios.h"

void scenario_cts_smoke_triangle(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        -0.5f, -0.5f, 0.0f, 1.0f, 0.5f, -0.5f, 0.0f, 1.0f, 0.0f, 0.5f, 0.0f, 1.0f,
    };

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkAttachmentDescription attachment = {
        .format = format,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
        .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
        .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
        .finalLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
    };
    VkAttachmentReference color_reference = {0, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    VkSubpassDescription subpass = {
        .pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_reference,
    };
    VkRenderPassCreateInfo render_pass_info = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        .attachmentCount = 1,
        .pAttachments = &attachment,
        .subpassCount = 1,
        .pSubpasses = &subpass,
    };
    VkRenderPass render_pass;
    CHECK(vkCreateRenderPass(ctx->device, &render_pass_info, NULL, &render_pass));

    VkFramebufferCreateInfo framebuffer_info = {
        .sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        .renderPass = render_pass,
        .attachmentCount = 1,
        .pAttachments = &color.view,
        .width = width,
        .height = height,
        .layers = 1,
    };
    VkFramebuffer framebuffer;
    CHECK(vkCreateFramebuffer(ctx->device, &framebuffer_info, NULL, &framebuffer));

    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "cts_smoke_triangle.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "cts_smoke_triangle.frag.spv");
    VkVertexInputBindingDescription binding = {0, 4 * sizeof(float),
                                               VK_VERTEX_INPUT_RATE_VERTEX};
    VkVertexInputAttributeDescription attribute = {0, 0, VK_FORMAT_R32G32B32A32_SFLOAT, 0};
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = 1,
        .pVertexBindingDescriptions = &binding,
        .vertexAttributeDescriptionCount = 1,
        .pVertexAttributeDescriptions = &attribute,
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
        .render_pass = render_pass,
        .width = width,
        .height = height,
    };
    VkPipeline pipeline = graphics_pipeline(ctx, &pipeline_desc);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    VkClearValue clear = {.color = {.float32 = {0.125f, 0.25f, 0.75f, 1.0f}}};
    VkRenderPassBeginInfo begin = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        .renderPass = render_pass,
        .framebuffer = framebuffer,
        .renderArea = {{0, 0}, {width, height}},
        .clearValueCount = 1,
        .pClearValues = &clear,
    };
    vkCmdBeginRenderPass(command_buffer, &begin, VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    VkDeviceSize offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &offset);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRenderPass(command_buffer);
    image_barrier(command_buffer, color.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyFramebuffer(ctx->device, framebuffer, NULL);
    vkDestroyRenderPass(ctx->device, render_pass, NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &vertex_buffer);
}
