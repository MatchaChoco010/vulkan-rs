/*
 * Two render pass instances on one framebuffer. The first draws three rectangles inline,
 * each inside a precise occlusion query (the second rectangle lies outside the
 * attachment); the second loads the attachment and executes two secondary command
 * buffers that inherit the render pass. The query results are read with
 * vkCmdCopyQueryPoolResults (64 bits) and vkGetQueryPoolResults (32 bits). The shaders
 * are those of the blend scenario.
 */
#include "scenarios.h"

typedef struct {
    float color[4];
    float rect[4];
} Push;

static float ndc(int32_t pixel) { return (float)pixel / 128.0f - 1.0f; }

static Push rectangle(int32_t x0, int32_t y0, int32_t x1, int32_t y1, float r, float g,
                      float b) {
    Push push = {{r, g, b, 1.0f}, {ndc(x0), ndc(y0), ndc(x1), ndc(y1)}};
    return push;
}

void scenario_secondary_queries(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    const VkShaderStageFlags stages = VK_SHADER_STAGE_VERTEX_BIT | VK_SHADER_STAGE_FRAGMENT_BIT;
    const Push queried[3] = {
        rectangle(16, 16, 48, 48, 1.0f, 0.0f, 0.0f),
        rectangle(320, 16, 352, 48, 1.0f, 1.0f, 1.0f),
        rectangle(64, 16, 80, 24, 0.0f, 1.0f, 0.0f),
    };
    const Push secondary_rectangles[2] = {
        rectangle(16, 64, 112, 112, 0.0f, 0.0f, 1.0f),
        rectangle(128, 64, 240, 200, 1.0f, 1.0f, 0.0f),
    };

    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);
    Buffer results = buffer_create(ctx, 3 * sizeof(uint64_t), VK_BUFFER_USAGE_TRANSFER_DST_BIT,
                                   VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                       VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);

    VkAttachmentDescription attachments[2] = {
        {
            .format = format,
            .samples = VK_SAMPLE_COUNT_1_BIT,
            .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
            .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
            .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
            .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
            .finalLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        },
        {
            .format = format,
            .samples = VK_SAMPLE_COUNT_1_BIT,
            .loadOp = VK_ATTACHMENT_LOAD_OP_LOAD,
            .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
            .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
            .initialLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
            .finalLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        },
    };
    VkAttachmentReference color_reference = {0, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    VkSubpassDescription subpass = {
        .pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_reference,
    };
    VkSubpassDependency dependencies[2] = {
        {
            .srcSubpass = VK_SUBPASS_EXTERNAL,
            .dstSubpass = 0,
            .srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            .dstStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            .srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            .dstAccessMask =
                VK_ACCESS_COLOR_ATTACHMENT_READ_BIT | VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
        },
        {
            .srcSubpass = 0,
            .dstSubpass = VK_SUBPASS_EXTERNAL,
            .srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
            .dstStageMask = VK_PIPELINE_STAGE_TRANSFER_BIT,
            .srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
            .dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT,
        },
    };
    VkRenderPass render_passes[2];
    for (uint32_t i = 0; i < 2; i++) {
        VkRenderPassCreateInfo render_pass_info = {
            .sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            .attachmentCount = 1,
            .pAttachments = &attachments[i],
            .subpassCount = 1,
            .pSubpasses = &subpass,
            .dependencyCount = COUNT(dependencies),
            .pDependencies = dependencies,
        };
        CHECK(vkCreateRenderPass(ctx->device, &render_pass_info, NULL, &render_passes[i]));
    }
    VkFramebufferCreateInfo framebuffer_info = {
        .sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        .renderPass = render_passes[0],
        .attachmentCount = 1,
        .pAttachments = &color.view,
        .width = width,
        .height = height,
        .layers = 1,
    };
    VkFramebuffer framebuffer;
    CHECK(vkCreateFramebuffer(ctx->device, &framebuffer_info, NULL, &framebuffer));

    VkQueryPoolCreateInfo query_pool_info = {
        .sType = VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO,
        .queryType = VK_QUERY_TYPE_OCCLUSION,
        .queryCount = 3,
    };
    VkQueryPool query_pool;
    CHECK(vkCreateQueryPool(ctx->device, &query_pool_info, NULL, &query_pool));

    VkPushConstantRange range = {stages, 0, sizeof(Push)};
    VkPipelineLayout layout = pipeline_layout(ctx, NULL, 0, &range, 1);
    VkShaderModule vertex = shader_module(ctx, "blend.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "blend.frag.spv");
    GraphicsDesc pipeline_desc = {
        .vertex = vertex,
        .fragment = fragment,
        .vertex_entry = "main",
        .fragment_entry = "main",
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        .samples = VK_SAMPLE_COUNT_1_BIT,
        .blend_attachments = &BLEND_DISABLED,
        .blend_attachment_count = 1,
        .layout = layout,
        .render_pass = render_passes[0],
        .width = width,
        .height = height,
    };
    VkPipeline pipeline = graphics_pipeline(ctx, &pipeline_desc);
    vkDestroyShaderModule(ctx->device, fragment, NULL);
    vkDestroyShaderModule(ctx->device, vertex, NULL);

    VkCommandBufferAllocateInfo secondary_allocate = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        .commandPool = ctx->command_pool,
        .level = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
        .commandBufferCount = 2,
    };
    VkCommandBuffer secondaries[2];
    CHECK(vkAllocateCommandBuffers(ctx->device, &secondary_allocate, secondaries));
    VkCommandBufferInheritanceInfo inheritance = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO,
        .renderPass = render_passes[1],
        .subpass = 0,
        .framebuffer = framebuffer,
    };
    VkCommandBufferBeginInfo secondary_begin = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        .flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT |
                 VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT,
        .pInheritanceInfo = &inheritance,
    };
    for (uint32_t i = 0; i < 2; i++) {
        CHECK(vkBeginCommandBuffer(secondaries[i], &secondary_begin));
        vkCmdBindPipeline(secondaries[i], VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
        vkCmdPushConstants(secondaries[i], layout, stages, 0, sizeof(Push),
                           &secondary_rectangles[i]);
        vkCmdDraw(secondaries[i], 6, 1, 0, 0);
        CHECK(vkEndCommandBuffer(secondaries[i]));
    }

    VkCommandBuffer command_buffer = commands_begin(ctx);
    vkCmdResetQueryPool(command_buffer, query_pool, 0, 3);
    VkClearValue clear = {.color = {.float32 = {0.0f, 0.0f, 0.0f, 1.0f}}};
    VkRenderPassBeginInfo first_begin = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        .renderPass = render_passes[0],
        .framebuffer = framebuffer,
        .renderArea = {{0, 0}, {width, height}},
        .clearValueCount = 1,
        .pClearValues = &clear,
    };
    vkCmdBeginRenderPass(command_buffer, &first_begin, VK_SUBPASS_CONTENTS_INLINE);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    for (uint32_t i = 0; i < 3; i++) {
        vkCmdBeginQuery(command_buffer, query_pool, i, VK_QUERY_CONTROL_PRECISE_BIT);
        vkCmdPushConstants(command_buffer, layout, stages, 0, sizeof(Push), &queried[i]);
        vkCmdDraw(command_buffer, 6, 1, 0, 0);
        vkCmdEndQuery(command_buffer, query_pool, i);
    }
    vkCmdEndRenderPass(command_buffer);
    VkRenderPassBeginInfo second_begin = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
        .renderPass = render_passes[1],
        .framebuffer = framebuffer,
        .renderArea = {{0, 0}, {width, height}},
    };
    vkCmdBeginRenderPass(command_buffer, &second_begin,
                         VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS);
    vkCmdExecuteCommands(command_buffer, 2, secondaries);
    vkCmdEndRenderPass(command_buffer);
    vkCmdCopyQueryPoolResults(command_buffer, query_pool, 0, 3, results.buffer, 0,
                              sizeof(uint64_t),
                              VK_QUERY_RESULT_64_BIT | VK_QUERY_RESULT_WAIT_BIT);
    host_read_barrier(command_buffer, VK_PIPELINE_STAGE_2_COPY_BIT,
                      VK_ACCESS_2_TRANSFER_WRITE_BIT);
    commands_submit(ctx, command_buffer);
    vkFreeCommandBuffers(ctx->device, ctx->command_pool, 2, secondaries);

    uint32_t host_results[3];
    CHECK(vkGetQueryPoolResults(ctx->device, query_pool, 0, 3, sizeof(host_results),
                                host_results, sizeof(uint32_t), VK_QUERY_RESULT_WAIT_BIT));
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");
    output_buffer(ctx, &results, 6, 1, "copied_queries", "r32ui");
    output_words(ctx, host_results, 3, "host_queries");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyQueryPool(ctx->device, query_pool, NULL);
    vkDestroyFramebuffer(ctx->device, framebuffer, NULL);
    vkDestroyRenderPass(ctx->device, render_passes[1], NULL);
    vkDestroyRenderPass(ctx->device, render_passes[0], NULL);
    buffer_destroy(ctx, &results);
    image_destroy(ctx, &color);
}
