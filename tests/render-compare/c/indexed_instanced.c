/*
 * Sixteen quads from one indexed quad: binding 0 holds the four corners, binding 1 one
 * offset and one R8G8B8A8_UNORM color per instance, and two indexed draws of eight
 * instances each start at instance 0 and 8.
 */
#include "scenarios.h"

typedef struct {
    float offset[2];
    uint8_t color[4];
} Instance;

void scenario_indexed_instanced(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float corners[] = {-1.0f, -1.0f, 1.0f, -1.0f, 1.0f, 1.0f, -1.0f, 1.0f};
    static const uint16_t indices[] = {0, 1, 2, 2, 3, 0};
    Instance instances[16];
    for (uint32_t i = 0; i < 16; i++) {
        instances[i].offset[0] = -0.75f + 0.5f * (float)(i % 4);
        instances[i].offset[1] = -0.75f + 0.5f * (float)(i / 4);
        instances[i].color[0] = (uint8_t)(i * 53 % 256);
        instances[i].color[1] = (uint8_t)(i * 97 % 256);
        instances[i].color[2] = (uint8_t)((i * 29 + 64) % 256);
        instances[i].color[3] = 255;
    }

    Buffer corner_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, corners, sizeof(corners));
    Buffer instance_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, instances, sizeof(instances));
    Buffer index_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_INDEX_BUFFER_BIT, indices, sizeof(indices));
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "indexed_instanced.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "indexed_instanced.frag.spv");
    VkVertexInputBindingDescription bindings[] = {
        {0, 2 * sizeof(float), VK_VERTEX_INPUT_RATE_VERTEX},
        {1, sizeof(Instance), VK_VERTEX_INPUT_RATE_INSTANCE},
    };
    VkVertexInputAttributeDescription attributes[] = {
        {0, 0, VK_FORMAT_R32G32_SFLOAT, 0},
        {1, 1, VK_FORMAT_R32G32_SFLOAT, 0},
        {2, 1, VK_FORMAT_R8G8B8A8_UNORM, 2 * sizeof(float)},
    };
    VkPipelineVertexInputStateCreateInfo vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        .vertexBindingDescriptionCount = COUNT(bindings),
        .pVertexBindingDescriptions = bindings,
        .vertexAttributeDescriptionCount = COUNT(attributes),
        .pVertexAttributeDescriptions = attributes,
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
    const float clear[4] = {24.0f / 255.0f, 24.0f / 255.0f, 32.0f / 255.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    VkBuffer vertex_buffers[] = {corner_buffer.buffer, instance_buffer.buffer};
    VkDeviceSize offsets[] = {0, 0};
    vkCmdBindVertexBuffers(command_buffer, 0, COUNT(vertex_buffers), vertex_buffers, offsets);
    vkCmdBindIndexBuffer(command_buffer, index_buffer.buffer, 0, VK_INDEX_TYPE_UINT16);
    vkCmdDrawIndexed(command_buffer, COUNT(indices), 8, 0, 0, 0);
    vkCmdDrawIndexed(command_buffer, COUNT(indices), 8, 0, 0, 8);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &index_buffer);
    buffer_destroy(ctx, &instance_buffer);
    buffer_destroy(ctx, &corner_buffer);
}
