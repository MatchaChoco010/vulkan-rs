/*
 * Four quadrants, each selected with a dynamic viewport and scissor and drawn with another
 * kind of draw: vkCmdDrawIndexedIndirectCount on a triangle list (one command is culled,
 * and the count buffer stops before the command that would cover the quadrant),
 * vkCmdDrawIndirect on a triangle strip, vkCmdDrawIndexedIndirect with two commands on
 * strips with primitive restart, and direct draws of a line list and a point list. The
 * indirect commands reach their buffer through vkCmdUpdateBuffer and vkCmdCopyBuffer.
 * Every pipeline specializes the shaders with its own constants.
 *
 * The strips with the restart index are not drawn by vkCmdDrawIndexedIndirectCount: the
 * lavapipe of Mesa 25.2 ignores the restart index in that command and no other.
 */
#include "scenarios.h"

#define PIXEL(value) ((float)(value) / 64.0f - 1.0f)
#define QUAD(x0, y0, x1, y1)                                                               \
    PIXEL(x0), PIXEL(y0), PIXEL(x1), PIXEL(y0), PIXEL(x0), PIXEL(y1), PIXEL(x1), PIXEL(y1)

typedef struct {
    uint32_t palette_base;
    float brightness;
    VkBool32 swap_red_and_blue;
} Constants;

void scenario_indirect_draws(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        QUAD(8, 8, 56, 56),
        QUAD(72, 40, 120, 120),
        QUAD(72, 8, 120, 24),
        QUAD(0, 0, 128, 128),
        PIXEL(8.5), PIXEL(16.5), PIXEL(120.5), PIXEL(16.5),
        PIXEL(16.5), PIXEL(24.5), PIXEL(16.5), PIXEL(120.5),
        PIXEL(40.5), PIXEL(40.5), PIXEL(80.5), PIXEL(40.5),
        PIXEL(40.5), PIXEL(80.5), PIXEL(80.5), PIXEL(80.5),
    };
    static const uint16_t indices[] = {
        0, 1, 3, 3, 2, 0, 0, 1, 2, 3, 0xffff, 4, 5, 6, 7, 0, 2, 3, 3, 1, 0,
    };
    static const VkDrawIndexedIndirectCommand list_commands[4] = {
        {6, 1, 0, 0, 1},
        {6, 1, 15, 12, 0},
        {6, 1, 0, 4, 2},
        {6, 1, 0, 12, 0},
    };
    static const struct {
        VkDrawIndexedIndirectCommand strips[2];
        VkDrawIndirectCommand strip;
    } staged = {
        {{9, 1, 6, 0, 4}, {4, 1, 6, 8, 5}},
        {4, 1, 0, 3},
    };
    static const uint32_t draw_count = 3;
    const VkDeviceSize strips_offset = sizeof(list_commands);
    const VkDeviceSize strip_offset = strips_offset + sizeof(staged.strips);

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    Buffer index_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_INDEX_BUFFER_BIT, indices, sizeof(indices));
    Buffer staging =
        buffer_with_data(ctx, VK_BUFFER_USAGE_TRANSFER_SRC_BIT, &staged, sizeof(staged));
    Buffer count_buffer = buffer_with_data(ctx, VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT,
                                           &draw_count, sizeof(draw_count));
    Buffer indirect_buffer = buffer_create(
        ctx, strip_offset + sizeof(staged.strip),
        VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT | VK_BUFFER_USAGE_TRANSFER_DST_BIT,
        VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT);
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "indirect_draws.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "indirect_draws.frag.spv");
    VkVertexInputBindingDescription binding = {0, 2 * sizeof(float),
                                               VK_VERTEX_INPUT_RATE_VERTEX};
    VkVertexInputAttributeDescription attribute = {0, 0, VK_FORMAT_R32G32_SFLOAT, 0};
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = 1,
        .pVertexBindingDescriptions = &binding,
        .vertexAttributeDescriptionCount = 1,
        .pVertexAttributeDescriptions = &attribute,
    };
    VkPipelineRasterizationStateCreateInfo culling = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        .polygonMode = VK_POLYGON_MODE_FILL,
        .cullMode = VK_CULL_MODE_BACK_BIT,
        .frontFace = VK_FRONT_FACE_CLOCKWISE,
        .lineWidth = 1.0f,
    };
    static const VkSpecializationMapEntry vertex_entries[] = {
        {0, offsetof(Constants, palette_base), sizeof(uint32_t)},
    };
    static const VkSpecializationMapEntry fragment_entries[] = {
        {1, offsetof(Constants, brightness), sizeof(float)},
        {2, offsetof(Constants, swap_red_and_blue), sizeof(VkBool32)},
    };
    static const Constants constants[5] = {
        {0, 1.0f, VK_FALSE}, {0, 1.0f, VK_TRUE}, {0, 1.0f, VK_FALSE},
        {6, 0.5f, VK_FALSE}, {7, 1.0f, VK_FALSE},
    };
    static const VkPrimitiveTopology topologies[5] = {
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,  VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP,
        VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP, VK_PRIMITIVE_TOPOLOGY_LINE_LIST,
        VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
    };
    static const VkDynamicState dynamic_states[] = {VK_DYNAMIC_STATE_VIEWPORT,
                                                    VK_DYNAMIC_STATE_SCISSOR};
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &format,
    };
    VkPipeline pipelines[5];
    for (uint32_t i = 0; i < 5; i++) {
        VkSpecializationInfo vertex_specialization = {
            .mapEntryCount = COUNT(vertex_entries),
            .pMapEntries = vertex_entries,
            .dataSize = sizeof(Constants),
            .pData = &constants[i],
        };
        VkSpecializationInfo fragment_specialization = {
            .mapEntryCount = COUNT(fragment_entries),
            .pMapEntries = fragment_entries,
            .dataSize = sizeof(Constants),
            .pData = &constants[i],
        };
        GraphicsDesc pipeline_desc = {
            .vertex = vertex,
            .fragment = fragment,
            .vertex_entry = "main",
            .fragment_entry = "main",
            .vertex_input = &vertex_input,
            .vertex_specialization = &vertex_specialization,
            .fragment_specialization = &fragment_specialization,
            .topology = topologies[i],
            .primitive_restart = i == 2,
            .rasterization = i == 0 ? &culling : NULL,
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
        pipelines[i] = graphics_pipeline(ctx, &pipeline_desc);
    }
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    vkCmdUpdateBuffer(command_buffer, indirect_buffer.buffer, 0, sizeof(list_commands),
                      list_commands);
    VkBufferCopy regions[2] = {
        {0, strips_offset, sizeof(staged.strips)},
        {sizeof(staged.strips), strip_offset, sizeof(staged.strip)},
    };
    vkCmdCopyBuffer(command_buffer, staging.buffer, indirect_buffer.buffer, COUNT(regions),
                    regions);
    VkBufferMemoryBarrier2 buffer_barrier = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2,
        .srcStageMask = VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT,
        .srcAccessMask = VK_ACCESS_2_TRANSFER_WRITE_BIT,
        .dstStageMask = VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT,
        .dstAccessMask = VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT,
        .srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .buffer = indirect_buffer.buffer,
        .offset = 0,
        .size = VK_WHOLE_SIZE,
    };
    VkDependencyInfo dependency = {
        .sType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
        .bufferMemoryBarrierCount = 1,
        .pBufferMemoryBarriers = &buffer_barrier,
    };
    vkCmdPipelineBarrier2(command_buffer, &dependency);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {16.0f / 255.0f, 16.0f / 255.0f, 16.0f / 255.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    VkDeviceSize vertex_offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &vertex_offset);
    vkCmdBindIndexBuffer(command_buffer, index_buffer.buffer, 0, VK_INDEX_TYPE_UINT16);
    for (uint32_t quadrant = 0; quadrant < 4; quadrant++) {
        VkViewport viewport = {
            128.0f * (float)(quadrant % 2), 128.0f * (float)(quadrant / 2), 128.0f, 128.0f,
            0.0f, 1.0f,
        };
        VkRect2D scissor = {{128 * (int32_t)(quadrant % 2), 128 * (int32_t)(quadrant / 2)},
                            {128, 128}};
        vkCmdSetViewport(command_buffer, 0, 1, &viewport);
        vkCmdSetScissor(command_buffer, 0, 1, &scissor);
        vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipelines[quadrant]);
        switch (quadrant) {
        case 0:
            vkCmdDrawIndexedIndirectCount(command_buffer, indirect_buffer.buffer, 0,
                                          count_buffer.buffer, 0, COUNT(list_commands),
                                          sizeof(VkDrawIndexedIndirectCommand));
            break;
        case 1:
            vkCmdDrawIndirect(command_buffer, indirect_buffer.buffer, strip_offset, 1,
                              sizeof(VkDrawIndirectCommand));
            break;
        case 2:
            vkCmdDrawIndexedIndirect(command_buffer, indirect_buffer.buffer, strips_offset,
                                     COUNT(staged.strips), sizeof(VkDrawIndexedIndirectCommand));
            break;
        default:
            vkCmdDraw(command_buffer, 4, 1, 16, 0);
            vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipelines[4]);
            vkCmdDraw(command_buffer, 4, 1, 20, 0);
            break;
        }
    }
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    for (uint32_t i = 0; i < 5; i++) {
        vkDestroyPipeline(ctx->device, pipelines[i], NULL);
    }
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &indirect_buffer);
    buffer_destroy(ctx, &count_buffer);
    buffer_destroy(ctx, &staging);
    buffer_destroy(ctx, &index_buffer);
    buffer_destroy(ctx, &vertex_buffer);
}
