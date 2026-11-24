/*
 * A square patch of four control points that the tessellator splits, at level 2, into
 * eight triangles around its center (the one tessellation of a quad that the specification
 * fixes). The geometry shader colors each triangle by its quarter of the square and drops
 * the last quarter. The pipeline has five stages, a tessellation state with the lower left domain
 * origin in its pNext chain, and culls back faces, so the picture is there only when the
 * domain origin and the front face agree. The tessellation level comes from push constants
 * of two stages.
 */
#include "scenarios.h"

typedef struct {
    float level;
    uint32_t dropped;
} Push;

void scenario_tessellation_geometry(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        -0.75f, -0.75f, 0.75f, -0.75f, -0.75f, 0.75f, 0.75f, 0.75f,
    };
    static const Push push = {2.0f, 3};
    const VkShaderStageFlags push_stages =
        VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT | VK_SHADER_STAGE_GEOMETRY_BIT;

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkPushConstantRange range = {push_stages, 0, sizeof(Push)};
    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, &range, 1);
    static const struct {
        const char *file;
        VkShaderStageFlagBits stage;
    } sources[5] = {
        {"tessellation_geometry.vert.spv", VK_SHADER_STAGE_VERTEX_BIT},
        {"tessellation_geometry.tesc.spv", VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT},
        {"tessellation_geometry.tese.spv", VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT},
        {"tessellation_geometry.geom.spv", VK_SHADER_STAGE_GEOMETRY_BIT},
        {"tessellation_geometry.frag.spv", VK_SHADER_STAGE_FRAGMENT_BIT},
    };
    VkShaderModule modules[5];
    VkPipelineShaderStageCreateInfo stages[5];
    for (uint32_t i = 0; i < 5; i++) {
        modules[i] = shader_module(ctx, sources[i].file);
        stages[i] = (VkPipelineShaderStageCreateInfo){
            .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            .stage = sources[i].stage,
            .module = modules[i],
            .pName = "main",
        };
    }
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
    VkPipelineInputAssemblyStateCreateInfo input_assembly = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        .topology = VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
    };
    VkPipelineTessellationDomainOriginStateCreateInfo domain_origin = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO,
        .domainOrigin = VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT,
    };
    VkPipelineTessellationStateCreateInfo tessellation = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO,
        .pNext = &domain_origin,
        .patchControlPoints = 4,
    };
    VkViewport viewport = {0.0f, 0.0f, (float)width, (float)height, 0.0f, 1.0f};
    VkRect2D scissor = {{0, 0}, {width, height}};
    VkPipelineViewportStateCreateInfo viewport_state = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        .viewportCount = 1,
        .pViewports = &viewport,
        .scissorCount = 1,
        .pScissors = &scissor,
    };
    VkPipelineRasterizationStateCreateInfo rasterization = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        .polygonMode = VK_POLYGON_MODE_FILL,
        .cullMode = VK_CULL_MODE_BACK_BIT,
        .frontFace = VK_FRONT_FACE_COUNTER_CLOCKWISE,
        .lineWidth = 1.0f,
    };
    VkPipelineMultisampleStateCreateInfo multisample = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        .rasterizationSamples = VK_SAMPLE_COUNT_1_BIT,
    };
    VkPipelineColorBlendStateCreateInfo color_blend = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        .attachmentCount = 1,
        .pAttachments = &BLEND_DISABLED,
    };
    VkPipelineRenderingCreateInfo rendering = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
        .colorAttachmentCount = 1,
        .pColorAttachmentFormats = &format,
    };
    VkGraphicsPipelineCreateInfo pipeline_info = {
        .sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        .pNext = &rendering,
        .stageCount = COUNT(stages),
        .pStages = stages,
        .pVertexInputState = &vertex_input,
        .pInputAssemblyState = &input_assembly,
        .pTessellationState = &tessellation,
        .pViewportState = &viewport_state,
        .pRasterizationState = &rasterization,
        .pMultisampleState = &multisample,
        .pColorBlendState = &color_blend,
        .layout = layout,
    };
    VkPipeline pipeline;
    CHECK(vkCreateGraphicsPipelines(ctx->device, VK_NULL_HANDLE, 1, &pipeline_info, NULL,
                                    &pipeline));
    for (uint32_t i = 5; i > 0; i--) {
        vkDestroyShaderModule(ctx->device, modules[i - 1], NULL);
    }

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {16.0f / 255.0f, 16.0f / 255.0f, 16.0f / 255.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    VkDeviceSize vertex_offset = 0;
    vkCmdBindVertexBuffers(command_buffer, 0, 1, &vertex_buffer.buffer, &vertex_offset);
    vkCmdPushConstants(command_buffer, layout, push_stages, 0, sizeof(push), &push);
    vkCmdDraw(command_buffer, 4, 1, 0, 0);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    image_destroy(ctx, &color);
    buffer_destroy(ctx, &vertex_buffer);
}
