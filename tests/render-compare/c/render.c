#include "render.h"

#include <stdarg.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const char VALIDATION_LAYER[] = "VK_LAYER_KHRONOS_validation";

static uint32_t validation_message_count;
static int validation_enabled;
static PFN_vkDestroyDebugUtilsMessengerEXT destroy_debug_utils_messenger;

const VkPipelineColorBlendAttachmentState BLEND_DISABLED = {
    .blendEnable = VK_FALSE,
    .colorWriteMask = VK_COLOR_COMPONENT_R_BIT | VK_COLOR_COMPONENT_G_BIT |
                      VK_COLOR_COMPONENT_B_BIT | VK_COLOR_COMPONENT_A_BIT,
};

void render_fail(const char *format, ...) {
    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
    fputc('\n', stderr);
    exit(1);
}

static VKAPI_ATTR VkBool32 VKAPI_CALL debug_callback(
    VkDebugUtilsMessageSeverityFlagBitsEXT severity, VkDebugUtilsMessageTypeFlagsEXT types,
    const VkDebugUtilsMessengerCallbackDataEXT *data, void *user_data) {
    (void)severity;
    (void)types;
    (void)user_data;
    fprintf(stderr, "validation message: [%s] %s\n",
            data->pMessageIdName ? data->pMessageIdName : "", data->pMessage);
    validation_message_count++;
    return VK_FALSE;
}

uint32_t ctx_validation_messages(void) { return validation_message_count; }

int ctx_validation_enabled(void) { return validation_enabled; }

static int has_layer(const char *name) {
    uint32_t count = 0;
    CHECK(vkEnumerateInstanceLayerProperties(&count, NULL));
    VkLayerProperties *layers = calloc(count + 1, sizeof(*layers));
    CHECK(vkEnumerateInstanceLayerProperties(&count, layers));
    int found = 0;
    for (uint32_t i = 0; i < count; i++) {
        if (strcmp(layers[i].layerName, name) == 0) {
            found = 1;
        }
    }
    free(layers);
    return found;
}

static void require(VkBool32 supported, const char *feature) {
    if (!supported) {
        render_fail("unsupported: the device has no %s", feature);
    }
}

void ctx_create(Ctx *ctx, const char *shader_dir, const Scenario *scenario) {
    memset(ctx, 0, sizeof(*ctx));
    ctx->shader_dir = shader_dir;
    validation_enabled = has_layer(VALIDATION_LAYER);

    VkDebugUtilsMessengerCreateInfoEXT messenger_info = {
        .sType = VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
        .messageSeverity = VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT |
                           VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
        .messageType = VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT |
                       VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT |
                       VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT,
        .pfnUserCallback = debug_callback,
    };
    const char *layers[2];
    uint32_t layer_count = 0;
    const char *capture_layer = getenv("RENDER_COMPARE_CAPTURE_LAYER");
    if (capture_layer != NULL && capture_layer[0] != '\0') {
        if (!has_layer(capture_layer)) {
            render_fail("unsupported: the layer %s is not installed", capture_layer);
        }
        layers[layer_count++] = capture_layer;
    }
    if (validation_enabled) {
        layers[layer_count++] = VALIDATION_LAYER;
    }
    const char *extensions[] = {VK_EXT_DEBUG_UTILS_EXTENSION_NAME};
    VkApplicationInfo app = {
        .sType = VK_STRUCTURE_TYPE_APPLICATION_INFO,
        .pApplicationName = "vulkan-rs render comparison",
        .apiVersion = VK_API_VERSION_1_3,
    };
    VkInstanceCreateInfo instance_info = {
        .sType = VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        .pApplicationInfo = &app,
        .enabledLayerCount = layer_count,
        .ppEnabledLayerNames = layer_count ? layers : NULL,
    };
    if (validation_enabled) {
        instance_info.pNext = &messenger_info;
        instance_info.enabledExtensionCount = COUNT(extensions);
        instance_info.ppEnabledExtensionNames = extensions;
    }
    CHECK(vkCreateInstance(&instance_info, NULL, &ctx->instance));

    if (validation_enabled) {
        PFN_vkCreateDebugUtilsMessengerEXT create_messenger =
            (PFN_vkCreateDebugUtilsMessengerEXT)vkGetInstanceProcAddr(
                ctx->instance, "vkCreateDebugUtilsMessengerEXT");
        destroy_debug_utils_messenger = (PFN_vkDestroyDebugUtilsMessengerEXT)vkGetInstanceProcAddr(
            ctx->instance, "vkDestroyDebugUtilsMessengerEXT");
        CHECK(create_messenger(ctx->instance, &messenger_info, NULL, &ctx->messenger));
    }

    uint32_t device_count = 0;
    CHECK(vkEnumeratePhysicalDevices(ctx->instance, &device_count, NULL));
    VkPhysicalDevice *devices = calloc(device_count + 1, sizeof(*devices));
    CHECK(vkEnumeratePhysicalDevices(ctx->instance, &device_count, devices));
    for (uint32_t i = 0; i < device_count; i++) {
        VkPhysicalDeviceProperties properties;
        vkGetPhysicalDeviceProperties(devices[i], &properties);
        if (properties.apiVersion >= VK_API_VERSION_1_3) {
            ctx->physical_device = devices[i];
            break;
        }
    }
    free(devices);
    if (ctx->physical_device == VK_NULL_HANDLE) {
        render_fail("unsupported: no Vulkan 1.3 device");
    }

    uint32_t family_count = 0;
    vkGetPhysicalDeviceQueueFamilyProperties(ctx->physical_device, &family_count, NULL);
    VkQueueFamilyProperties *families = calloc(family_count + 1, sizeof(*families));
    vkGetPhysicalDeviceQueueFamilyProperties(ctx->physical_device, &family_count, families);
    uint32_t wanted = VK_QUEUE_GRAPHICS_BIT | VK_QUEUE_COMPUTE_BIT;
    ctx->queue_family = UINT32_MAX;
    for (uint32_t i = 0; i < family_count; i++) {
        if ((families[i].queueFlags & wanted) == wanted) {
            ctx->queue_family = i;
            break;
        }
    }
    free(families);
    if (ctx->queue_family == UINT32_MAX) {
        render_fail("unsupported: no queue family with graphics and compute");
    }

    uint32_t extension_count = 0;
    CHECK(vkEnumerateDeviceExtensionProperties(ctx->physical_device, NULL, &extension_count,
                                               NULL));
    VkExtensionProperties *extensions_found = calloc(extension_count + 1, sizeof(*extensions_found));
    CHECK(vkEnumerateDeviceExtensionProperties(ctx->physical_device, NULL, &extension_count,
                                               extensions_found));
    for (uint32_t wanted_index = 0; wanted_index < scenario->device_extension_count;
         wanted_index++) {
        const char *wanted_extension = scenario->device_extensions[wanted_index];
        int found = 0;
        for (uint32_t i = 0; i < extension_count; i++) {
            if (strcmp(extensions_found[i].extensionName, wanted_extension) == 0) {
                found = 1;
            }
        }
        if (!found) {
            render_fail("unsupported: the device has no %s", wanted_extension);
        }
    }
    free(extensions_found);

    VkPhysicalDeviceShaderObjectFeaturesEXT shader_object = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT,
    };
    VkPhysicalDeviceRayQueryFeaturesKHR ray_query = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR,
    };
    VkPhysicalDeviceAccelerationStructureFeaturesKHR acceleration_structure = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
        .pNext = &ray_query,
    };
    VkPhysicalDeviceVulkan11Features features11 = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
    };
    if (scenario->extension_features == EXTENSION_FEATURES_RAY_QUERY) {
        features11.pNext = &acceleration_structure;
    } else if (scenario->extension_features == EXTENSION_FEATURES_SHADER_OBJECT) {
        features11.pNext = &shader_object;
    }
    VkPhysicalDeviceVulkan12Features features12 = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
        .pNext = &features11,
    };
    VkPhysicalDeviceVulkan13Features features13 = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
        .pNext = &features12,
    };
    VkPhysicalDeviceFeatures2 supported = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
        .pNext = &features13,
    };
    vkGetPhysicalDeviceFeatures2(ctx->physical_device, &supported);
    require(supported.features.occlusionQueryPrecise, "occlusionQueryPrecise");
    require(supported.features.geometryShader, "geometryShader");
    require(supported.features.tessellationShader, "tessellationShader");
    require(supported.features.sampleRateShading, "sampleRateShading");
    require(supported.features.fragmentStoresAndAtomics, "fragmentStoresAndAtomics");
    require(supported.features.shaderSampledImageArrayDynamicIndexing,
            "shaderSampledImageArrayDynamicIndexing");
    require(supported.features.multiDrawIndirect, "multiDrawIndirect");
    require(supported.features.drawIndirectFirstInstance, "drawIndirectFirstInstance");
    require(features11.shaderDrawParameters, "shaderDrawParameters");
    require(features12.drawIndirectCount, "drawIndirectCount");
    require(features12.descriptorIndexing, "descriptorIndexing");
    require(features12.shaderSampledImageArrayNonUniformIndexing,
            "shaderSampledImageArrayNonUniformIndexing");
    require(features12.descriptorBindingSampledImageUpdateAfterBind,
            "descriptorBindingSampledImageUpdateAfterBind");
    require(features12.descriptorBindingPartiallyBound, "descriptorBindingPartiallyBound");
    require(features12.descriptorBindingVariableDescriptorCount,
            "descriptorBindingVariableDescriptorCount");
    require(features12.runtimeDescriptorArray, "runtimeDescriptorArray");
    require(features12.bufferDeviceAddress, "bufferDeviceAddress");
    require(features13.inlineUniformBlock, "inlineUniformBlock");
    require(features13.synchronization2, "synchronization2");
    require(features13.dynamicRendering, "dynamicRendering");
    if (scenario->extension_features == EXTENSION_FEATURES_RAY_QUERY) {
        require(acceleration_structure.accelerationStructure, "accelerationStructure");
        require(ray_query.rayQuery, "rayQuery");
    } else if (scenario->extension_features == EXTENSION_FEATURES_SHADER_OBJECT) {
        require(shader_object.shaderObject, "shaderObject");
    }

    float priority = 1.0f;
    VkDeviceQueueCreateInfo queue_info = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
        .queueFamilyIndex = ctx->queue_family,
        .queueCount = 1,
        .pQueuePriorities = &priority,
    };
    shader_object.shaderObject = VK_TRUE;
    ray_query.rayQuery = VK_TRUE;
    acceleration_structure = (VkPhysicalDeviceAccelerationStructureFeaturesKHR){
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR,
        .pNext = &ray_query,
        .accelerationStructure = VK_TRUE,
    };
    features11 = (VkPhysicalDeviceVulkan11Features){
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES,
        .pNext = features11.pNext,
        .shaderDrawParameters = VK_TRUE,
    };
    features12 = (VkPhysicalDeviceVulkan12Features){
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
        .pNext = &features11,
        .drawIndirectCount = VK_TRUE,
        .descriptorIndexing = VK_TRUE,
        .shaderSampledImageArrayNonUniformIndexing = VK_TRUE,
        .descriptorBindingSampledImageUpdateAfterBind = VK_TRUE,
        .descriptorBindingPartiallyBound = VK_TRUE,
        .descriptorBindingVariableDescriptorCount = VK_TRUE,
        .runtimeDescriptorArray = VK_TRUE,
        .bufferDeviceAddress = VK_TRUE,
    };
    features13 = (VkPhysicalDeviceVulkan13Features){
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
        .pNext = &features12,
        .inlineUniformBlock = VK_TRUE,
        .synchronization2 = VK_TRUE,
        .dynamicRendering = VK_TRUE,
    };
    VkPhysicalDeviceFeatures2 enabled = {
        .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2,
        .pNext = &features13,
        .features =
            {
                .geometryShader = VK_TRUE,
                .tessellationShader = VK_TRUE,
                .sampleRateShading = VK_TRUE,
                .multiDrawIndirect = VK_TRUE,
                .drawIndirectFirstInstance = VK_TRUE,
                .occlusionQueryPrecise = VK_TRUE,
                .fragmentStoresAndAtomics = VK_TRUE,
                .shaderSampledImageArrayDynamicIndexing = VK_TRUE,
            },
    };
    VkDeviceCreateInfo device_info = {
        .sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        .pNext = &enabled,
        .queueCreateInfoCount = 1,
        .pQueueCreateInfos = &queue_info,
        .enabledExtensionCount = scenario->device_extension_count,
        .ppEnabledExtensionNames =
            scenario->device_extension_count ? scenario->device_extensions : NULL,
    };
    CHECK(vkCreateDevice(ctx->physical_device, &device_info, NULL, &ctx->device));
    vkGetDeviceQueue(ctx->device, ctx->queue_family, 0, &ctx->queue);
    vkGetPhysicalDeviceMemoryProperties(ctx->physical_device, &ctx->memory_properties);

    VkCommandPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
        .queueFamilyIndex = ctx->queue_family,
    };
    CHECK(vkCreateCommandPool(ctx->device, &pool_info, NULL, &ctx->command_pool));
}

void ctx_destroy(Ctx *ctx) {
    vkDestroyCommandPool(ctx->device, ctx->command_pool, NULL);
    vkDestroyDevice(ctx->device, NULL);
    if (ctx->messenger != VK_NULL_HANDLE) {
        destroy_debug_utils_messenger(ctx->instance, ctx->messenger, NULL);
    }
    vkDestroyInstance(ctx->instance, NULL);
}

uint32_t memory_type(const Ctx *ctx, uint32_t type_bits, VkMemoryPropertyFlags flags) {
    for (uint32_t i = 0; i < ctx->memory_properties.memoryTypeCount; i++) {
        if ((type_bits & (1u << i)) &&
            (ctx->memory_properties.memoryTypes[i].propertyFlags & flags) == flags) {
            return i;
        }
    }
    render_fail("no memory type with the flags 0x%x among 0x%x", flags, type_bits);
}

Buffer buffer_create(Ctx *ctx, VkDeviceSize size, VkBufferUsageFlags usage,
                     VkMemoryPropertyFlags flags) {
    Buffer buffer = {.size = size};
    VkBufferCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = size,
        .usage = usage,
        .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
    };
    CHECK(vkCreateBuffer(ctx->device, &info, NULL, &buffer.buffer));
    VkMemoryRequirements requirements;
    vkGetBufferMemoryRequirements(ctx->device, buffer.buffer, &requirements);
    VkMemoryAllocateInfo allocate = {
        .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        .allocationSize = requirements.size,
        .memoryTypeIndex = memory_type(ctx, requirements.memoryTypeBits, flags),
    };
    CHECK(vkAllocateMemory(ctx->device, &allocate, NULL, &buffer.memory));
    CHECK(vkBindBufferMemory(ctx->device, buffer.buffer, buffer.memory, 0));
    return buffer;
}

void buffer_write(Ctx *ctx, const Buffer *buffer, const void *data, size_t size) {
    void *mapped = NULL;
    CHECK(vkMapMemory(ctx->device, buffer->memory, 0, VK_WHOLE_SIZE, 0, &mapped));
    memcpy(mapped, data, size);
    vkUnmapMemory(ctx->device, buffer->memory);
}

void buffer_destroy(Ctx *ctx, Buffer *buffer) {
    vkDestroyBuffer(ctx->device, buffer->buffer, NULL);
    vkFreeMemory(ctx->device, buffer->memory, NULL);
}

Buffer buffer_with_data(Ctx *ctx, VkBufferUsageFlags usage, const void *data, size_t size) {
    Buffer buffer = buffer_create(ctx, size, usage,
                                  VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                      VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    buffer_write(ctx, &buffer, data, size);
    return buffer;
}

ImageDesc image_desc(VkFormat format, uint32_t width, uint32_t height, VkImageUsageFlags usage,
                     VkImageAspectFlags aspect) {
    ImageDesc desc = {
        .info =
            {
                .sType = VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
                .imageType = VK_IMAGE_TYPE_2D,
                .format = format,
                .extent = {width, height, 1},
                .mipLevels = 1,
                .arrayLayers = 1,
                .samples = VK_SAMPLE_COUNT_1_BIT,
                .tiling = VK_IMAGE_TILING_OPTIMAL,
                .usage = usage,
                .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
                .initialLayout = VK_IMAGE_LAYOUT_UNDEFINED,
            },
        .aspect = aspect,
        .view_type = VK_IMAGE_VIEW_TYPE_2D,
    };
    return desc;
}

Image image_create(Ctx *ctx, const ImageDesc *desc) {
    Image image = {0};
    CHECK(vkCreateImage(ctx->device, &desc->info, NULL, &image.image));
    VkMemoryRequirements requirements;
    vkGetImageMemoryRequirements(ctx->device, image.image, &requirements);
    VkMemoryAllocateInfo allocate = {
        .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        .allocationSize = requirements.size,
        .memoryTypeIndex = memory_type(ctx, requirements.memoryTypeBits,
                                       VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT),
    };
    CHECK(vkAllocateMemory(ctx->device, &allocate, NULL, &image.memory));
    CHECK(vkBindImageMemory(ctx->device, image.image, image.memory, 0));
    VkImageViewCreateInfo view = {
        .sType = VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
        .image = image.image,
        .viewType = desc->view_type,
        .format = desc->info.format,
        .subresourceRange = {desc->aspect, 0, desc->info.mipLevels, 0, desc->info.arrayLayers},
    };
    CHECK(vkCreateImageView(ctx->device, &view, NULL, &image.view));
    return image;
}

void image_destroy(Ctx *ctx, Image *image) {
    vkDestroyImageView(ctx->device, image->view, NULL);
    vkDestroyImage(ctx->device, image->image, NULL);
    vkFreeMemory(ctx->device, image->memory, NULL);
}

VkCommandBuffer commands_begin(Ctx *ctx) {
    VkCommandBufferAllocateInfo allocate = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
        .commandPool = ctx->command_pool,
        .level = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
        .commandBufferCount = 1,
    };
    VkCommandBuffer command_buffer;
    CHECK(vkAllocateCommandBuffers(ctx->device, &allocate, &command_buffer));
    VkCommandBufferBeginInfo begin = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
        .flags = VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
    };
    CHECK(vkBeginCommandBuffer(command_buffer, &begin));
    return command_buffer;
}

void commands_submit(Ctx *ctx, VkCommandBuffer command_buffer) {
    CHECK(vkEndCommandBuffer(command_buffer));
    VkFenceCreateInfo fence_info = {.sType = VK_STRUCTURE_TYPE_FENCE_CREATE_INFO};
    VkFence fence;
    CHECK(vkCreateFence(ctx->device, &fence_info, NULL, &fence));
    VkCommandBufferSubmitInfo command_buffer_info = {
        .sType = VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO,
        .commandBuffer = command_buffer,
    };
    VkSubmitInfo2 submit = {
        .sType = VK_STRUCTURE_TYPE_SUBMIT_INFO_2,
        .commandBufferInfoCount = 1,
        .pCommandBufferInfos = &command_buffer_info,
    };
    CHECK(vkQueueSubmit2(ctx->queue, 1, &submit, fence));
    CHECK(vkWaitForFences(ctx->device, 1, &fence, VK_TRUE, UINT64_MAX));
    vkDestroyFence(ctx->device, fence, NULL);
    vkFreeCommandBuffers(ctx->device, ctx->command_pool, 1, &command_buffer);
}

void image_barrier(VkCommandBuffer command_buffer, VkImage image, VkImageAspectFlags aspect,
                   uint32_t base_mip, uint32_t mip_count, VkImageLayout old_layout,
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
        .subresourceRange = {aspect, base_mip, mip_count, 0, 1},
    };
    VkDependencyInfo dependency = {
        .sType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
        .imageMemoryBarrierCount = 1,
        .pImageMemoryBarriers = &barrier,
    };
    vkCmdPipelineBarrier2(command_buffer, &dependency);
}

void host_read_barrier(VkCommandBuffer command_buffer, VkPipelineStageFlags2 src_stage,
                       VkAccessFlags2 src_access) {
    VkMemoryBarrier2 barrier = {
        .sType = VK_STRUCTURE_TYPE_MEMORY_BARRIER_2,
        .srcStageMask = src_stage,
        .srcAccessMask = src_access,
        .dstStageMask = VK_PIPELINE_STAGE_2_HOST_BIT,
        .dstAccessMask = VK_ACCESS_2_HOST_READ_BIT,
    };
    VkDependencyInfo dependency = {
        .sType = VK_STRUCTURE_TYPE_DEPENDENCY_INFO,
        .memoryBarrierCount = 1,
        .pMemoryBarriers = &barrier,
    };
    vkCmdPipelineBarrier2(command_buffer, &dependency);
}

void color_attachment_begin(VkCommandBuffer command_buffer, VkImage image) {
    image_barrier(command_buffer, image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
                  VK_PIPELINE_STAGE_2_NONE, VK_ACCESS_2_NONE,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT);
}

void color_attachment_end(VkCommandBuffer command_buffer, VkImage image) {
    image_barrier(command_buffer, image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                  VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT,
                  VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_READ_BIT);
}

VkClearValue depth_stencil_clear(float depth, uint32_t stencil) {
    VkClearValue value;
    memset(&value, 0, sizeof(value));
    value.depthStencil.depth = depth;
    value.depthStencil.stencil = stencil;
    return value;
}

void rendering_begin(VkCommandBuffer command_buffer, VkImageView view, const float clear[4],
                     uint32_t width, uint32_t height) {
    VkRenderingAttachmentInfo color_attachment = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
        .imageView = view,
        .imageLayout = VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        .loadOp = VK_ATTACHMENT_LOAD_OP_CLEAR,
        .storeOp = VK_ATTACHMENT_STORE_OP_STORE,
        .clearValue = {.color = {.float32 = {clear[0], clear[1], clear[2], clear[3]}}},
    };
    VkRenderingInfo rendering_info = {
        .sType = VK_STRUCTURE_TYPE_RENDERING_INFO,
        .renderArea = {{0, 0}, {width, height}},
        .layerCount = 1,
        .colorAttachmentCount = 1,
        .pColorAttachments = &color_attachment,
    };
    vkCmdBeginRendering(command_buffer, &rendering_info);
}

void image_upload(Ctx *ctx, VkImage image, uint32_t width, uint32_t height, const void *data,
                  size_t size, VkImageLayout final_layout) {
    Buffer staging = buffer_with_data(ctx, VK_BUFFER_USAGE_TRANSFER_SRC_BIT, data, size);
    VkCommandBuffer command_buffer = commands_begin(ctx);
    image_barrier(command_buffer, image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_UNDEFINED, VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL,
                  VK_PIPELINE_STAGE_2_NONE, VK_ACCESS_2_NONE, VK_PIPELINE_STAGE_2_COPY_BIT,
                  VK_ACCESS_2_TRANSFER_WRITE_BIT);
    VkBufferImageCopy region = {
        .imageSubresource = {VK_IMAGE_ASPECT_COLOR_BIT, 0, 0, 1},
        .imageExtent = {width, height, 1},
    };
    vkCmdCopyBufferToImage(command_buffer, staging.buffer, image,
                           VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, 1, &region);
    image_barrier(command_buffer, image, VK_IMAGE_ASPECT_COLOR_BIT, 0, 1,
                  VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL, final_layout,
                  VK_PIPELINE_STAGE_2_COPY_BIT, VK_ACCESS_2_TRANSFER_WRITE_BIT,
                  VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT, VK_ACCESS_2_MEMORY_READ_BIT);
    commands_submit(ctx, command_buffer);
    buffer_destroy(ctx, &staging);
}

static void output_add(Ctx *ctx, const char *name, const char *format, uint32_t width,
                       uint32_t height, const void *data, size_t size) {
    if (ctx->image_count == MAX_OUTPUT_IMAGES) {
        render_fail("more than %d output images", MAX_OUTPUT_IMAGES);
    }
    OutputImage *image = &ctx->images[ctx->image_count++];
    image->name = name;
    image->format = format;
    image->width = width;
    image->height = height;
    image->size = size;
    image->data = malloc(size);
    memcpy(image->data, data, size);
}

static void output_mapped(Ctx *ctx, const Buffer *buffer, size_t size, const char *name,
                          const char *format, uint32_t width, uint32_t height) {
    void *mapped = NULL;
    CHECK(vkMapMemory(ctx->device, buffer->memory, 0, VK_WHOLE_SIZE, 0, &mapped));
    output_add(ctx, name, format, width, height, mapped, size);
    vkUnmapMemory(ctx->device, buffer->memory);
}

void readback(Ctx *ctx, VkImage image, VkImageAspectFlags aspect, uint32_t width,
              uint32_t height, uint32_t texel_size, const char *name, const char *format) {
    size_t size = (size_t)width * height * texel_size;
    Buffer buffer = buffer_create(ctx, size, VK_BUFFER_USAGE_TRANSFER_DST_BIT,
                                  VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                      VK_MEMORY_PROPERTY_HOST_COHERENT_BIT);
    VkCommandBuffer command_buffer = commands_begin(ctx);
    VkBufferImageCopy region = {
        .imageSubresource = {aspect, 0, 0, 1},
        .imageExtent = {width, height, 1},
    };
    vkCmdCopyImageToBuffer(command_buffer, image, VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL,
                           buffer.buffer, 1, &region);
    host_read_barrier(command_buffer, VK_PIPELINE_STAGE_2_COPY_BIT,
                      VK_ACCESS_2_TRANSFER_WRITE_BIT);
    commands_submit(ctx, command_buffer);
    output_mapped(ctx, &buffer, size, name, format, width, height);
    buffer_destroy(ctx, &buffer);
}

void output_buffer(Ctx *ctx, const Buffer *buffer, uint32_t width, uint32_t height,
                   const char *name, const char *format) {
    output_mapped(ctx, buffer, (size_t)buffer->size, name, format, width, height);
}

void output_words(Ctx *ctx, const uint32_t *words, uint32_t count, const char *name) {
    output_add(ctx, name, "r32ui", count, 1, words, (size_t)count * sizeof(uint32_t));
}

void output_write(Ctx *ctx, const char *path) {
    FILE *file = fopen(path, "wb");
    if (file == NULL) {
        render_fail("cannot create %s", path);
    }
    fprintf(file, "render-compare 1\n");
    for (uint32_t i = 0; i < ctx->image_count; i++) {
        OutputImage *image = &ctx->images[i];
        fprintf(file, "image %s %s %u %u %zu\n", image->name, image->format, image->width,
                image->height, image->size);
        fwrite(image->data, 1, image->size, file);
        free(image->data);
    }
    if (fclose(file) != 0) {
        render_fail("cannot write %s", path);
    }
}

uint32_t *shader_code(Ctx *ctx, const char *file_name, size_t *size) {
    char path[4096];
    snprintf(path, sizeof(path), "%s/%s", ctx->shader_dir, file_name);
    FILE *file = fopen(path, "rb");
    if (file == NULL) {
        render_fail("cannot open %s", path);
    }
    fseek(file, 0, SEEK_END);
    long length = ftell(file);
    fseek(file, 0, SEEK_SET);
    uint32_t *code = malloc((size_t)length + 4);
    if (fread(code, 1, (size_t)length, file) != (size_t)length) {
        render_fail("cannot read %s", path);
    }
    fclose(file);
    *size = (size_t)length;
    return code;
}

VkShaderModule shader_module(Ctx *ctx, const char *file_name) {
    size_t size = 0;
    uint32_t *code = shader_code(ctx, file_name, &size);
    VkShaderModuleCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
        .codeSize = size,
        .pCode = code,
    };
    VkShaderModule module;
    CHECK(vkCreateShaderModule(ctx->device, &info, NULL, &module));
    free(code);
    return module;
}

VkPipeline graphics_pipeline(Ctx *ctx, const GraphicsDesc *desc) {
    VkPipelineShaderStageCreateInfo stages[] = {
        {
            .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            .stage = VK_SHADER_STAGE_VERTEX_BIT,
            .module = desc->vertex,
            .pName = desc->vertex_entry,
            .pSpecializationInfo = desc->vertex_specialization,
        },
        {
            .sType = VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
            .stage = VK_SHADER_STAGE_FRAGMENT_BIT,
            .module = desc->fragment,
            .pName = desc->fragment_entry,
            .pSpecializationInfo = desc->fragment_specialization,
        },
    };
    VkPipelineVertexInputStateCreateInfo no_vertex_input = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
    };
    VkPipelineInputAssemblyStateCreateInfo input_assembly = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        .topology = desc->topology,
        .primitiveRestartEnable = desc->primitive_restart,
    };
    VkViewport viewport = {0.0f, 0.0f, (float)desc->width, (float)desc->height, 0.0f, 1.0f};
    VkRect2D scissor = {{0, 0}, {desc->width, desc->height}};
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
        .cullMode = VK_CULL_MODE_NONE,
        .frontFace = VK_FRONT_FACE_COUNTER_CLOCKWISE,
        .lineWidth = 1.0f,
    };
    VkPipelineMultisampleStateCreateInfo multisample = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        .rasterizationSamples = desc->samples,
    };
    VkPipelineColorBlendStateCreateInfo color_blend = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        .attachmentCount = desc->blend_attachment_count,
        .pAttachments = desc->blend_attachment_count ? desc->blend_attachments : NULL,
    };
    VkPipelineDynamicStateCreateInfo dynamic = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        .dynamicStateCount = desc->dynamic_state_count,
        .pDynamicStates = desc->dynamic_states,
    };
    VkGraphicsPipelineCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
        .pNext = desc->rendering,
        .stageCount = COUNT(stages),
        .pStages = stages,
        .pVertexInputState = desc->vertex_input ? desc->vertex_input : &no_vertex_input,
        .pInputAssemblyState = &input_assembly,
        .pViewportState = &viewport_state,
        .pRasterizationState = desc->rasterization ? desc->rasterization : &rasterization,
        .pMultisampleState = desc->multisample ? desc->multisample : &multisample,
        .pDepthStencilState = desc->depth_stencil,
        .pColorBlendState = &color_blend,
        .pDynamicState = desc->dynamic_state_count ? &dynamic : NULL,
        .layout = desc->layout,
        .renderPass = desc->render_pass,
        .subpass = desc->subpass,
    };
    VkPipeline pipeline;
    CHECK(vkCreateGraphicsPipelines(ctx->device, VK_NULL_HANDLE, 1, &info, NULL, &pipeline));
    return pipeline;
}

VkPipelineLayout pipeline_layout(Ctx *ctx, const VkDescriptorSetLayout *set_layouts,
                                 uint32_t set_layout_count, const VkPushConstantRange *ranges,
                                 uint32_t range_count) {
    VkPipelineLayoutCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
        .setLayoutCount = set_layout_count,
        .pSetLayouts = set_layout_count ? set_layouts : NULL,
        .pushConstantRangeCount = range_count,
        .pPushConstantRanges = range_count ? ranges : NULL,
    };
    VkPipelineLayout layout;
    CHECK(vkCreatePipelineLayout(ctx->device, &info, NULL, &layout));
    return layout;
}
