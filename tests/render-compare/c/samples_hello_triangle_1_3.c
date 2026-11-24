/*
 * Based on the hello_triangle_1_3 sample of Khronos Vulkan-Samples
 * (https://github.com/KhronosGroup/Vulkan-Samples, samples/api/hello_triangle_1_3).
 *
 * Copyright (c) 2024-2026, Huawei Technologies Co., Ltd.
 * Copyright (c) 2025, Sascha Willems (the shaders)
 * SPDX-License-Identifier: Apache-2.0
 *
 * Taken from the sample: the vertices and their layout, the clear color, the pipeline
 * with dynamic viewport, scissor, cull mode, front face and topology, and the commands of
 * render_triangle: the barriers around a dynamic rendering instance, the dynamic state
 * and the draw. The shaders in shaders/vulkan-samples/hello_triangle_1_3/ are the Slang
 * shaders of the sample, unchanged. Changed: the image is an offscreen R8G8B8A8_SRGB image
 * of 256x256 instead of a swapchain image, it ends in TRANSFER_SRC_OPTIMAL for the copy to
 * the host instead of PRESENT_SRC_KHR, and the vertex buffer is host visible memory.
 */
#include "scenarios.h"

void scenario_samples_hello_triangle_1_3(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_SRGB;
    static const float vertices[] = {
        0.5f, -0.5f, 1.0f, 0.0f, 0.0f, 0.5f, 0.5f, 0.0f,
        1.0f, 0.0f, -0.5f, 0.5f, 0.0f, 0.0f, 1.0f,
    };

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "samples_hello_triangle_1_3.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "samples_hello_triangle_1_3.frag.spv");
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
    static const VkDynamicState dynamic_states[] = {
        VK_DYNAMIC_STATE_VIEWPORT,  VK_DYNAMIC_STATE_SCISSOR,
        VK_DYNAMIC_STATE_CULL_MODE, VK_DYNAMIC_STATE_FRONT_FACE,
        VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY,
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
        .dynamic_states = dynamic_states,
        .dynamic_state_count = COUNT(dynamic_states),
        .layout = layout,
        .rendering = &rendering,
        .width = width,
        .height = height,
    };
    VkPipeline pipeline = graphics_pipeline(ctx, &pipeline_desc);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    image_barrier(command_buffer, color.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                  VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT, VK_ACCESS_2_NONE,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT);
    VkRenderingAttachmentInfo color_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = color.view,
        .imageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .clearValue = {.color = {.float32 = {0.01f, 0.01f, 0.033f, 1.0f}}},
    };
    VkRenderingInfo rendering_info = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_INFO,
        .renderArea = {{0, 0}, {width, height}},
        .layerCount = 1,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_attachment,
    };
    vkCmdBeginRendering(command_buffer, &rendering_info);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    VkViewport viewport = {0.0f, 0.0f, (float)width, (float)height, 0.0f, 1.0f};
    vkCmdSetViewport(command_buffer, 0, 1, &viewport);
    VkRect2D scissor = {{0, 0}, {width, height}};
    vkCmdSetScissor(command_buffer, 0, 1, &scissor);
    vkCmdSetCullMode(command_buffer, VK_CULL_MODE_NONE);
    vkCmdSetFrontFace(command_buffer, VK_FRONT_FACE_CLOCKWISE);
    vkCmdSetPrimitiveTopology(command_buffer, VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST);
    VkDeviceSize offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &offset);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRendering(command_buffer);
    image_barrier(command_buffer, color.image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &vertex_buffer);
}
