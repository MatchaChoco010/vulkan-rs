/*
 * Eight cells, each with its own blend state, in which two translucent rectangles overlap
 * on the clear color. The rectangles come from push constants; the blend constants are
 * dynamic state.
 *
 * The color is the first member of the push constants: the lavapipe of Mesa 26.2 gives
 * the fragment shader the bytes at offset 0 instead of the member's offset for some of
 * these rectangles (those in the lower right quarter of the attachment), whatever the
 * blend state.
 */
#include "scenarios.h"

typedef struct {
    float color[4];
    float rect[4];
} Push;

static VkPipelineColorBlendAttachmentState blend_mode(uint32_t mode) {
    const VkColorComponentFlags all = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT |
                                      VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT;
    VkPipelineColorBlendAttachmentState state = {
        .blendEnable = VK_TRUE,
        .srcColorBlendFactor = VK_BLEND_FACTOR_ONE,
        .dstColorBlendFactor = VK_BLEND_FACTOR_ONE,
        .colorBlendOp = VK_BLEND_OP_ADD,
        .srcAlphaBlendFactor = VK_BLEND_FACTOR_ONE,
        .dstAlphaBlendFactor = VK_BLEND_FACTOR_ONE,
        .alphaBlendOp = VK_BLEND_OP_ADD,
        .colorWriteMask = all,
    };
    switch (mode) {
    case 0:
        state.srcColorBlendFactor = VK_BLEND_FACTOR_SRC_ALPHA;
        state.dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
        state.dstAlphaBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA;
        break;
    case 1:
        break;
    case 2:
        state.srcColorBlendFactor = VK_BLEND_FACTOR_DST_COLOR;
        state.dstColorBlendFactor = VK_BLEND_FACTOR_ZERO;
        state.srcAlphaBlendFactor = VK_BLEND_FACTOR_DST_ALPHA;
        state.dstAlphaBlendFactor = VK_BLEND_FACTOR_ZERO;
        break;
    case 3:
        state.srcColorBlendFactor = VK_BLEND_FACTOR_CONSTANT_COLOR;
        state.dstColorBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR;
        state.srcAlphaBlendFactor = VK_BLEND_FACTOR_CONSTANT_ALPHA;
        state.dstAlphaBlendFactor = VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA;
        break;
    case 4:
        state.colorBlendOp = VK_BLEND_OP_MIN;
        state.alphaBlendOp = VK_BLEND_OP_MIN;
        break;
    case 5:
        state.colorBlendOp = VK_BLEND_OP_MAX;
        state.alphaBlendOp = VK_BLEND_OP_MAX;
        break;
    case 6:
        state.colorBlendOp = VK_BLEND_OP_REVERSE_SUBTRACT;
        state.alphaBlendOp = VK_BLEND_OP_REVERSE_SUBTRACT;
        break;
    default:
        state.blendEnable = VK_FALSE;
        state.colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_A_BIT;
        break;
    }
    return state;
}

static float ndc(uint32_t pixel) { return (float)pixel / 128.0f - 1.0f; }

void scenario_blend(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkPushConstantRange range = {VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                                 sizeof(Push)};
    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, &range, 1);
    VkShaderModule vertex = shader_module(ctx, "blend.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "blend.frag.spv");
    static const VkDynamicState dynamic_states[] = {VK_DYNAMIC_STATE_BLEND_CONSTANTS};
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &format,
    };
    VkPipeline pipelines[8];
    for (uint32_t mode = 0; mode < 8; mode++) {
        VkPipelineColorBlendAttachmentState blend = blend_mode(mode);
        GraphicsDesc pipeline_desc = {
            .vertex = vertex,
            .fragment = fragment,
            .vertex_entry = "main",
            .fragment_entry = "main",
            .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            .samples = VK_SAMPLE_COUNT_1_BIT,
            .blend_attachments = &blend,
            .blend_attachment_count = 1,
            .dynamic_states = dynamic_states,
            .dynamic_state_count = COUNT(dynamic_states),
            .layout = layout,
            .rendering = &rendering,
            .width = width,
            .height = height,
        };
        pipelines[mode] = graphics_pipeline(ctx, &pipeline_desc);
    }
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {64.0f / 255.0f, 96.0f / 255.0f, 128.0f / 255.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    const float blend_constants[4] = {0.25f, 0.5f, 0.75f, 0.5f};
    vkCmdSetBlendConstants(command_buffer, blend_constants);
    for (uint32_t cell = 0; cell < 8; cell++) {
        uint32_t x = 64 * (cell % 4);
        uint32_t y = 128 * (cell / 4);
        vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipelines[cell]);
        Push first = {
            {200.0f / 255.0f, 40.0f / 255.0f, 40.0f / 255.0f, 153.0f / 255.0f},
            {ndc(x + 8), ndc(y + 8), ndc(x + 40), ndc(y + 88)},
        };
        vkCmdPushConstants(command_buffer, layout,
                           VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                           sizeof(first), &first);
        vkCmdDraw(command_buffer, 6, 1, 0, 0);
        Push second = {
            {30.0f / 255.0f, 180.0f / 255.0f, 220.0f / 255.0f, 102.0f / 255.0f},
            {ndc(x + 24), ndc(y + 40), ndc(x + 56), ndc(y + 120)},
        };
        vkCmdPushConstants(command_buffer, layout,
                           VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT, 0,
                           sizeof(second), &second);
        vkCmdDraw(command_buffer, 6, 1, 0, 0);
    }
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    for (uint32_t mode = 0; mode < 8; mode++) {
        vkDestroyPipeline(ctx->device, pipelines[mode], NULL);
    }
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &color);
}
