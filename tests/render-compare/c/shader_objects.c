/*
 * Three rectangles drawn without a pipeline: the vertex and fragment shaders are shader
 * objects (VK_EXT_shader_object), bound together with empty tessellation and geometry
 * stages, and all the state a pipeline would hold is set by commands, among them the vertex
 * input, the sample mask, and per-attachment blend enable, blend equation and color write
 * mask. The second rectangle is blended over the first, the third writes only red and
 * alpha.
 */
#include "scenarios.h"

#include <stdlib.h>

#define LOAD(name) PFN_vk##name name = (PFN_vk##name)vkGetDeviceProcAddr(ctx->device, "vk" #name)
#define NDC(pixel) ((float)(pixel) / 128.0f - 1.0f)
#define RECT(x0, y0, x1, y1, r, g, b, a)                                                   \
    NDC(x0), NDC(y0), r, g, b, a, NDC(x1), NDC(y0), r, g, b, a, NDC(x1), NDC(y1), r, g, b, a, \
        NDC(x1), NDC(y1), r, g, b, a, NDC(x0), NDC(y1), r, g, b, a, NDC(x0), NDC(y0), r, g, b, a

void scenario_shader_objects(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        RECT(32, 32, 160, 160, 1.0f, 0.0f, 0.0f, 1.0f),
        RECT(96, 96, 224, 224, 0.0f, 0.0f, 1.0f, 0.5f),
        RECT(16, 176, 80, 240, 1.0f, 1.0f, 1.0f, 1.0f),
    };
    LOAD(CreateShadersEXT);
    LOAD(DestroyShaderEXT);
    LOAD(CmdBindShadersEXT);
    LOAD(CmdSetVertexInputEXT);
    LOAD(CmdSetRasterizationSamplesEXT);
    LOAD(CmdSetSampleMaskEXT);
    LOAD(CmdSetAlphaToCoverageEnableEXT);
    LOAD(CmdSetPolygonModeEXT);
    LOAD(CmdSetColorBlendEnableEXT);
    LOAD(CmdSetColorBlendEquationEXT);
    LOAD(CmdSetColorWriteMaskEXT);

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    size_t vertex_size = 0;
    size_t fragment_size = 0;
    uint32_t *vertex_code = shader_code(ctx, "shader_objects.vert.spv", &vertex_size);
    uint32_t *fragment_code = shader_code(ctx, "shader_objects.frag.spv", &fragment_size);
    VkShaderCreateInfoEXT shader_infos[2] = {
        {
            .sType = VK_STRUCTURE_TYPE_SHADER_CREATE_INFO_EXT,
            .stage = VK_SHADER_STAGE_VERTEX_BIT,
            .nextStage = VK_SHADER_STAGE_FRAGMENT_BIT,
            .codeType = VK_SHADER_CODE_TYPE_SPIRV_EXT,
            .codeSize = vertex_size,
            .pCode = vertex_code,
            .pName = "main",
        },
        {
            .sType = VK_STRUCTURE_TYPE_SHADER_CREATE_INFO_EXT,
            .stage = VK_SHADER_STAGE_FRAGMENT_BIT,
            .codeType = VK_SHADER_CODE_TYPE_SPIRV_EXT,
            .codeSize = fragment_size,
            .pCode = fragment_code,
            .pName = "main",
        },
    };
    VkShaderEXT shaders[2];
    CHECK(CreateShadersEXT(ctx->device, COUNT(shader_infos), shader_infos, NULL, shaders));
    free(fragment_code);
    free(vertex_code);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {32.0f / 255.0f, 48.0f / 255.0f, 64.0f / 255.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    static const VkShaderStageFlagBits stages[5] = {
        VK_SHADER_STAGE_VERTEX_BIT,
        VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT,
        VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT,
        VK_SHADER_STAGE_GEOMETRY_BIT,
        VK_SHADER_STAGE_FRAGMENT_BIT,
    };
    VkShaderEXT bound[5] = {shaders[0], VK_NULL_HANDLE, VK_NULL_HANDLE, VK_NULL_HANDLE,
                            shaders[1]};
    CmdBindShadersEXT(command_buffer, COUNT(stages), stages, bound);
    VkViewport viewport = {0.0f, 0.0f, (float)width, (float)height, 0.0f, 1.0f};
    VkRect2D scissor = {{0, 0}, {width, height}};
    vkCmdSetViewportWithCount(command_buffer, 1, &viewport);
    vkCmdSetScissorWithCount(command_buffer, 1, &scissor);
    vkCmdSetRasterizerDiscardEnable(command_buffer, VK_FALSE);
    VkVertexInputBindingDescription2EXT binding = {
        .sType = VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT,
        .binding = 0,
        .stride = 6 * sizeof(float),
        .inputRate = VK_VERTEX_INPUT_RATE_VERTEX,
        .divisor = 1,
    };
    VkVertexInputAttributeDescription2EXT attributes[2] = {
        {
            .sType = VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
            .location = 0,
            .binding = 0,
            .format = VK_FORMAT_R32G32_SFLOAT,
            .offset = 0,
        },
        {
            .sType = VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT,
            .location = 1,
            .binding = 0,
            .format = VK_FORMAT_R32G32B32A32_SFLOAT,
            .offset = 2 * sizeof(float),
        },
    };
    CmdSetVertexInputEXT(command_buffer, 1, &binding, COUNT(attributes), attributes);
    vkCmdSetPrimitiveTopology(command_buffer, VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST);
    vkCmdSetPrimitiveRestartEnable(command_buffer, VK_FALSE);
    CmdSetRasterizationSamplesEXT(command_buffer, VK_SAMPLE_COUNT_1_BIT);
    const VkSampleMask sample_mask = 1;
    CmdSetSampleMaskEXT(command_buffer, VK_SAMPLE_COUNT_1_BIT, &sample_mask);
    CmdSetAlphaToCoverageEnableEXT(command_buffer, VK_FALSE);
    CmdSetPolygonModeEXT(command_buffer, VK_POLYGON_MODE_FILL);
    vkCmdSetCullMode(command_buffer, VK_CULL_MODE_NONE);
    vkCmdSetFrontFace(command_buffer, VK_FRONT_FACE_COUNTER_CLOCKWISE);
    vkCmdSetDepthTestEnable(command_buffer, VK_FALSE);
    vkCmdSetDepthWriteEnable(command_buffer, VK_FALSE);
    vkCmdSetDepthBiasEnable(command_buffer, VK_FALSE);
    vkCmdSetStencilTestEnable(command_buffer, VK_FALSE);
    const VkBool32 blend_off = VK_FALSE;
    const VkBool32 blend_on = VK_TRUE;
    CmdSetColorBlendEnableEXT(command_buffer, 0, 1, &blend_off);
    const VkColorBlendEquationEXT equation = {
        .srcColorBlendFactor = VK_BLEND_FACTOR_SRC_ALPHA,
        .dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        .colorBlendOp = VK_BLEND_OP_ADD,
        .srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE,
        .dstAlphaBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
        .alphaBlendOp = VK_BLEND_OP_ADD,
    };
    CmdSetColorBlendEquationEXT(command_buffer, 0, 1, &equation);
    const VkColorComponentFlags all_components = VK_COLOR_COMPONENT_R_BIT |
                                                 VK_COLOR_COMPONENT_G_BIT |
                                                 VK_COLOR_COMPONENT_B_BIT |
                                                 VK_COLOR_COMPONENT_A_BIT;
    CmdSetColorWriteMaskEXT(command_buffer, 0, 1, &all_components);
    VkDeviceSize vertex_offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &vertex_offset);
    vkCmdDraw(command_buffer, 6, 1, 0, 0);
    CmdSetColorBlendEnableEXT(command_buffer, 0, 1, &blend_on);
    vkCmdDraw(command_buffer, 6, 1, 6, 0);
    CmdSetColorBlendEnableEXT(command_buffer, 0, 1, &blend_off);
    const VkColorComponentFlags red_and_alpha =
        VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_A_BIT;
    CmdSetColorWriteMaskEXT(command_buffer, 0, 1, &red_and_alpha);
    vkCmdDraw(command_buffer, 6, 1, 12, 0);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    DestroyShaderEXT(ctx->device, shaders[1], NULL);
    DestroyShaderEXT(ctx->device, shaders[0], NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &vertex_buffer);
}
