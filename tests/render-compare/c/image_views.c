/*
 * Sixteen cells, each sampled from another kind of image or with another sampler: the six
 * faces of a cube map, three layers of a 2D array image, two slices of a 3D image and the
 * linear blend between them, an unsigned integer image seen through a view that swaps red
 * and blue, a clamp-to-border and a mirrored-repeat sampler on a two-texel image, and a
 * comparison sampler on a depth image. The images are filled with vkCmdCopyBufferToImage
 * (one region per cube face, one region for both slices), vkCmdClearColorImage with float
 * and unsigned integer values, vkCmdCopyImage between array layers and
 * vkCmdClearDepthStencilImage.
 */
#include "scenarios.h"

static void layers_barrier(VkCommandBuffer command_buffer, VkImage image,
                           VkImageAspectFlags aspect, VkImageLayout old_layout,
                           VkImageLayout new_layout, VkPipelineStageFlags2 src_stage,
                           VkAccessFlags2 src_access, VkPipelineStageFlags2 dst_stage,
                           VkAccessFlags2 dst_access) {
    VkImageMemoryBarrier2 barrier = {
        .sType = VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2,
        .srcStageMask = src_stage,
        .srcAccessMask = src_access,
        .dstStageMask = dst_stage,
        .dstAccessMask = dst_access,
        .oldLayout = old_layout,
        .newLayout = new_layout,
        .srcQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .dstQueueFamilyIndex = VK_QUEUE_FAMILY_IGNORED,
        .image = image,
        .subresourceRange = {aspect, 0, VK_REMAINING_MIP_LEVELS, 0, VK_REMAINING_ARRAY_LAYERS},
    };
    VkDependencyInfo dependency = {
        .sType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
        .imageMemoryBarrierCount = 1,
        .pImageMemoryBarriers = &barrier,
    };
    vkCmdPipelineBarrier2(command_buffer, &dependency);
}

static void to_transfer_dst(VkCommandBuffer command_buffer, VkImage image,
                            VkImageAspectFlags aspect) {
    layers_barrier(command_buffer, image, aspect, VK_IMAGE_LAYOUT_UNDEFINED,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_PIPELINE_STAGE_2_NONE,
                   VK_ACCESS_2_NONE, VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT,
                   VK_ACCESS_2_TRANSFER_WRITE_BIT);
}

static void to_shader_read(VkCommandBuffer command_buffer, VkImage image,
                           VkImageAspectFlags aspect, VkImageLayout old_layout,
                           VkImageLayout new_layout) {
    layers_barrier(command_buffer, image, aspect, old_layout, new_layout,
                   VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT,
                   VK_ACCESS_2_TRANSFER_WRITE_BIT | VK_ACCESS_2_TRANSFER_READ_BIT,
                   VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT, VK_ACCESS_2_SHADER_SAMPLED_READ_BIT);
}

static VkSampler sampler_create(Ctx *ctx, VkFilter filter, VkSamplerAddressMode address_mode,
                                VkBool32 compare) {
    VkSamplerCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
        .magFilter = filter,
        .minFilter = filter,
        .mipmapMode = VK_SAMPLER_MIPMAP_MODE_NEAREST,
        .addressModeU = address_mode,
        .addressModeV = address_mode,
        .addressModeW = address_mode,
        .maxAnisotropy = 1.0f,
        .compareEnable = compare,
        .compareOp = compare ? VK_COMPARE_OP_LESS : VK_COMPARE_OP_NEVER,
        .borderColor = VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE,
    };
    VkSampler sampler;
    CHECK(vkCreateSampler(ctx->device, &info, NULL, &sampler));
    return sampler;
}

void scenario_image_views(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    const VkImageUsageFlags sampled_usage =
        VK_IMAGE_USAGE_TRANSFER_DST_BIT | VK_IMAGE_USAGE_SAMPLED_BIT;
    static const uint8_t face_colors[6][4] = {
        {255, 0, 0, 255},   {0, 255, 0, 255},   {0, 0, 255, 255},
        {255, 255, 0, 255}, {255, 0, 255, 255}, {0, 255, 255, 255},
    };
    static const uint8_t slice_colors[2][4] = {{200, 100, 50, 255}, {50, 100, 200, 255}};
    static const uint8_t pair_texels[8] = {250, 10, 10, 255, 10, 10, 250, 255};
    uint8_t staged[(6 + 2) * 16 * 4];
    for (uint32_t block = 0; block < 8; block++) {
        const uint8_t *block_color = block < 6 ? face_colors[block] : slice_colors[block - 6];
        for (uint32_t texel = 0; texel < 16; texel++) {
            for (uint32_t channel = 0; channel < 4; channel++) {
                staged[(block * 16 + texel) * 4 + channel] = block_color[channel];
            }
        }
    }

    Buffer staging =
        buffer_with_data(ctx, VK_BUFFER_USAGE_TRANSFER_SRC_BIT, staged, sizeof(staged));
    ImageDesc cube_desc = image_desc(format, 4, 4,
                                     sampled_usage | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                                     VK_IMAGE_ASPECT_COLOR_BIT);
    cube_desc.info.flags = VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT;
    cube_desc.info.arrayLayers = 6;
    cube_desc.view_type = VK_IMAGE_VIEW_TYPE_CUBE;
    Image cube = image_create(ctx, &cube_desc);
    ImageDesc layers_desc = image_desc(format, 4, 4, sampled_usage, VK_IMAGE_ASPECT_COLOR_BIT);
    layers_desc.info.arrayLayers = 3;
    layers_desc.view_type = VK_IMAGE_VIEW_TYPE_2D_ARRAY;
    Image layers = image_create(ctx, &layers_desc);
    ImageDesc volume_desc = image_desc(format, 4, 4, sampled_usage, VK_IMAGE_ASPECT_COLOR_BIT);
    volume_desc.info.imageType = VK_IMAGE_TYPE_3D;
    volume_desc.info.extent.depth = 2;
    volume_desc.view_type = VK_IMAGE_VIEW_TYPE_3D;
    Image volume = image_create(ctx, &volume_desc);
    ImageDesc integers_desc = image_desc(VK_FORMAT_R8G8B8A8_UINT, 1, 1, sampled_usage,
                                         VK_IMAGE_ASPECT_COLOR_BIT);
    Image integers = image_create(ctx, &integers_desc);
    VkImageViewCreateInfo swapped_info = {
        .sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
        .image = integers.image,
        .viewType = VK_IMAGE_VIEW_TYPE_2D,
        .format = VK_FORMAT_R8G8B8A8_UINT,
        .components = {VK_COMPONENT_SWIZZLE_B, VK_COMPONENT_SWIZZLE_IDENTITY,
                       VK_COMPONENT_SWIZZLE_R, VK_COMPONENT_SWIZZLE_A},
        .subresourceRange = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1},
    };
    VkImageView swapped_view;
    CHECK(vkCreateImageView(ctx->device, &swapped_info, NULL, &swapped_view));
    ImageDesc pair_desc = image_desc(format, 2, 1, sampled_usage, VK_IMAGE_ASPECT_COLOR_BIT);
    Image pair = image_create(ctx, &pair_desc);
    image_upload(ctx, pair.image, 2, 1, pair_texels, sizeof(pair_texels),
                 VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    ImageDesc depth_desc =
        image_desc(VK_FORMAT_D32_SFLOAT, 4, 4,
                   sampled_usage | VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT,
                   VK_IMAGE_ASPECT_DEPTH_BIT);
    Image depth = image_create(ctx, &depth_desc);
    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);

    VkCommandBuffer command_buffer = commands_begin(ctx);
    to_transfer_dst(command_buffer, cube.image, VK_IMAGE_ASPECT_COLOR_BIT);
    to_transfer_dst(command_buffer, layers.image, VK_IMAGE_ASPECT_COLOR_BIT);
    to_transfer_dst(command_buffer, volume.image, VK_IMAGE_ASPECT_COLOR_BIT);
    to_transfer_dst(command_buffer, integers.image, VK_IMAGE_ASPECT_COLOR_BIT);
    to_transfer_dst(command_buffer, depth.image, VK_IMAGE_ASPECT_DEPTH_BIT);
    VkBufferImageCopy faces[6];
    for (uint32_t face = 0; face < 6; face++) {
        faces[face] = (VkBufferImageCopy){
            .bufferOffset = face * 64,
            .imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, face, 1},
            .imageExtent = {4, 4, 1},
        };
    }
    vkCmdCopyBufferToImage(command_buffer, staging.buffer, cube.image,
                           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, COUNT(faces), faces);
    VkBufferImageCopy slices = {
        .bufferOffset = 6 * 64,
        .imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1},
        .imageExtent = {4, 4, 2},
    };
    vkCmdCopyBufferToImage(command_buffer, staging.buffer, volume.image,
                           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &slices);
    VkClearColorValue layer_color = {
        .float32 = {40.0f / 255.0f, 80.0f / 255.0f, 120.0f / 255.0f, 1.0f}};
    VkImageSubresourceRange first_layers = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 2};
    vkCmdClearColorImage(command_buffer, layers.image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                         &layer_color, 1, &first_layers);
    VkClearColorValue integer_color = {.uint32 = {10, 200, 30, 255}};
    VkImageSubresourceRange whole = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 1, 0, 1};
    vkCmdClearColorImage(command_buffer, integers.image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                         &integer_color, 1, &whole);
    VkClearDepthStencilValue depth_value = {0.5f, 0};
    VkImageSubresourceRange whole_depth = {VK_IMAGE_ASPECT_DEPTH_BIT, 0, 1, 0, 1};
    vkCmdClearDepthStencilImage(command_buffer, depth.image,
                                VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, &depth_value, 1,
                                &whole_depth);
    layers_barrier(command_buffer, cube.image, VK_IMAGE_ASPECT_COLOR_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                   VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT,
                   VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT, VK_ACCESS_2_TRANSFER_READ_BIT);
    VkImageCopy face_to_layer = {
        .srcSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 3, 1},
        .dstSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 2, 1},
        .extent = {4, 4, 1},
    };
    vkCmdCopyImage(command_buffer, cube.image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                   layers.image, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &face_to_layer);
    to_shader_read(command_buffer, cube.image, VK_IMAGE_ASPECT_COLOR_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                   VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    to_shader_read(command_buffer, layers.image, VK_IMAGE_ASPECT_COLOR_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                   VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    to_shader_read(command_buffer, volume.image, VK_IMAGE_ASPECT_COLOR_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                   VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    to_shader_read(command_buffer, integers.image, VK_IMAGE_ASPECT_COLOR_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                   VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL);
    to_shader_read(command_buffer, depth.image, VK_IMAGE_ASPECT_DEPTH_BIT,
                   VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                   VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL);
    commands_submit(ctx, command_buffer);

    VkSampler nearest =
        sampler_create(ctx, VK_FILTER_NEAREST, VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE, VK_FALSE);
    VkSampler linear =
        sampler_create(ctx, VK_FILTER_LINEAR, VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE, VK_FALSE);
    VkSampler border = sampler_create(ctx, VK_FILTER_NEAREST,
                                      VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER, VK_FALSE);
    VkSampler mirror = sampler_create(ctx, VK_FILTER_NEAREST,
                                      VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT, VK_FALSE);
    VkSampler comparison =
        sampler_create(ctx, VK_FILTER_NEAREST, VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE, VK_TRUE);

    static const VkDescriptorType types[8] = {
        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
        VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE,          VK_DESCRIPTOR_TYPE_SAMPLER,
        VK_DESCRIPTOR_TYPE_SAMPLER,                VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER,
    };
    VkDescriptorSetLayoutBinding bindings[8];
    for (uint32_t i = 0; i < 8; i++) {
        bindings[i] = (VkDescriptorSetLayoutBinding){i, types[i], 1,
                                                     VK_SHADER_STAGE_FRAGMENT_BIT, NULL};
    }
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = COUNT(bindings),
        .pBindings = bindings,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorPoolSize pool_sizes[] = {
        {VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, 5},
        {VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, 1},
        {VK_DESCRIPTOR_TYPE_SAMPLER, 2},
    };
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .maxSets = 1,
        .poolSizeCount = COUNT(pool_sizes),
        .pPoolSizes = pool_sizes,
    };
    VkDescriptorPool pool;
    CHECK(vkCreateDescriptorPool(ctx->device, &pool_info, NULL, &pool));
    VkDescriptorSetAllocateInfo allocate_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
        .descriptorPool = pool,
        .descriptorSetCount = 1,
        .pSetLayouts = &set_layout,
    };
    VkDescriptorSet set;
    CHECK(vkAllocateDescriptorSets(ctx->device, &allocate_info, &set));
    const VkImageLayout read_only = VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL;
    VkDescriptorImageInfo image_infos[8] = {
        {nearest, cube.view, read_only},
        {nearest, layers.view, read_only},
        {linear, volume.view, read_only},
        {nearest, swapped_view, read_only},
        {VK_NULL_HANDLE, pair.view, read_only},
        {border, VK_NULL_HANDLE, VK_IMAGE_LAYOUT_UNDEFINED},
        {mirror, VK_NULL_HANDLE, VK_IMAGE_LAYOUT_UNDEFINED},
        {comparison, depth.view, VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL},
    };
    VkWriteDescriptorSet writes[8];
    for (uint32_t i = 0; i < 8; i++) {
        writes[i] = (VkWriteDescriptorSet){
            .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            .dstSet = set,
            .dstBinding = i,
            .descriptorCount = 1,
            .descriptorType = types[i],
            .pImageInfo = &image_infos[i],
        };
    }
    vkUpdateDescriptorSets(ctx->device, COUNT(writes), writes, 0, NULL);

    VkPipelineLayout layout = pipeline_layout(ctx, &set_layout, 1, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "image_views.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "image_views.frag.spv");
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

    command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {0.0f, 0.0f, 0.0f, 1.0f};
    rendering_begin(command_buffer, color.view, clear, width, height);
    vkCmdBindPipeline(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, pipeline);
    vkCmdBindDescriptorSets(command_buffer, VK_PIPELINE_BIND_POINT_GRAPHICS, layout, 0, 1, &set,
                            0, NULL);
    vkCmdDraw(command_buffer, 3, 1, 0, 0);
    vkCmdEndRendering(command_buffer);
    color_attachment_end(command_buffer, color.image);
    commands_submit(ctx, command_buffer);
    readback(ctx, color.image, VK_IMAGE_ASPECT_COLOR_BIT, width, height, 4, "color", "rgba8");

    vkDestroyPipeline(ctx->device, pipeline, NULL);
    vkDestroyPipelineLayout(ctx->device, layout, NULL);
    vkDestroyDescriptorPool(ctx->device, pool, NULL);
    vkDestroyDescriptorSetLayout(ctx->device, set_layout, NULL);
    vkDestroySampler(ctx->device, comparison, NULL);
    vkDestroySampler(ctx->device, mirror, NULL);
    vkDestroySampler(ctx->device, border, NULL);
    vkDestroySampler(ctx->device, linear, NULL);
    vkDestroySampler(ctx->device, nearest, NULL);
    image_destroy(ctx, &color);
    image_destroy(ctx, &depth);
    image_destroy(ctx, &pair);
    vkDestroyImageView(ctx->device, swapped_view, NULL);
    image_destroy(ctx, &integers);
    image_destroy(ctx, &volume);
    image_destroy(ctx, &layers);
    image_destroy(ctx, &cube);
    buffer_destroy(ctx, &staging);
}
