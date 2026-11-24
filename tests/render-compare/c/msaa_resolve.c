/*
 * Three overlapping triangles drawn with four samples per pixel in a render pass whose
 * subpass resolves the multisampled attachment into a single-sampled one. The shaders are
 * those of samples_hello_triangle_1_3.
 */
#include "scenarios.h"

void scenario_msaa_resolve(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        -0.9f,  -0.8f, 1.0f, 0.0f, 0.0f, 0.7f,  -0.6f, 1.0f, 0.0f, 0.0f,
        -0.3f,  0.9f,  1.0f, 0.0f, 0.0f, 0.95f, 0.1f,  0.0f, 1.0f, 0.0f,
        0.2f,   0.95f, 0.0f, 1.0f, 0.0f, -0.1f, -0.2f, 0.0f, 1.0f, 0.0f,
        -0.95f, 0.7f,  0.0f, 0.0f, 1.0f, 0.9f,  0.85f, 0.0f, 0.0f, 1.0f,
        0.9f,   0.8f,  0.0f, 0.0f, 1.0f,
    };

    Buffer vertex_buffer =
        buffer_with_data(ctx, VK_BUFFER_USAGE_VERTEX_BUFFER_BIT, vertices, sizeof(vertices));
    ImageDesc multisampled_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    multisampled_desc.info.samples = VK_SAMPLE_COUNT_4_BIT;
    Image multisampled = image_create(ctx, &multisampled_desc);
    ImageDesc resolved_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image resolved = image_create(ctx, &resolved_desc);

    VkAttachmentDescription attachments[] = {
        {
            .format = format,
            .samples = VK_SAMPLE_COUNT_4_BIT,
            .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
            .storeOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
            .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
            .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
            .finalLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        },
        {
            .format = format,
            .samples = VK_SAMPLE_COUNT_1_BIT,
            .loadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
            .stencilLoadOp = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
            .stencilStoreOp = VK_ATTACHMENT_STORE_OP_DONT_CARE,
            .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
            .finalLayout = VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
        },
    };
    VkAttachmentReference color_reference = {0, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    VkAttachmentReference resolve_reference = {1, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL};
    VkSubpassDescription subpass = {
        .pipelineBindPoint = VK_PIPELINE_BIND_POINT_GRAPHICS,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_reference,
        .pResolveAttachments = &resolve_reference,
    };
    VkSubpassDependency dependency = {
        .srcSubpass = 0,
        .dstSubpass = VK_SUBPASS_EXTERNAL,
        .srcStageMask = VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
        .dstStageMask = VK_PIPELINE_STAGE_TRANSFER_BIT,
        .srcAccessMask = VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT,
        .dstAccessMask = VK_ACCESS_TRANSFER_READ_BIT,
    };
    VkRenderPassCreateInfo render_pass_info = {
        .sType = VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
        .attachmentCount = COUNT(attachments),
        .pAttachments = attachments,
        .subpassCount = 1,
        .pSubpasses = &subpass,
        .dependencyCount = 1,
        .pDependencies = &dependency,
    };
    VkRenderPass render_pass;
    CHECK(vkCreateRenderPass(ctx->device, &render_pass_info, NULL, &render_pass));
    VkImageView views[] = {multisampled.view, resolved.view};
    VkFramebufferCreateInfo framebuffer_info = {
        .sType = VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
        .renderPass = render_pass,
        .attachmentCount = COUNT(views),
        .pAttachments = views,
        .width = width,
        .height = height,
        .layers = 1,
    };
    VkFramebuffer framebuffer;
    CHECK(vkCreateFramebuffer(ctx->device, &framebuffer_info, NULL, &framebuffer));

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
    GraphicsDesc pipeline_desc = {
        .vertex = vertex,
        .fragment = fragment,
        .vertex_entry = "main",
        .fragment_entry = "main",
        .vertex_input = &vertex_input,
        .topology = VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
        .samples = VK_SAMPLE_COUNT_4_BIT,
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
    VkClearValue clear = {.color = {.float32 = {0.0f, 0.0f, 0.0f, 1.0f}}};
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
    vkCmdDraw(command_buffer, 9, 1, 0, 0);
    vkCmdEndRenderPass(command_buffer);
    commands_submit(ctx, command_buffer);
    readback(ctx, resolved.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color",
             "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyFramebuffer(ctx->device, framebuffer, NULL);
    vkDestroyRenderPass(ctx->device, render_pass, NULL);
    image_destroy(ctx, &resolved);
    image_destroy(ctx, &multisampled);
    buffer_destroy(ctx, &vertex_buffer);
}
