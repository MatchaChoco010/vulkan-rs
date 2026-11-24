/*
 * A fragment shader that casts one ray per pixel into a top level acceleration structure
 * with three instances of a quad (VK_KHR_acceleration_structure, VK_KHR_ray_query). The
 * instances carry a custom index, a mask, a shader binding table offset and flags in bit
 * fields; the mask hides the third instance, which would cover the picture. The structures
 * are built on the device from buffers given by device address, with the geometry in a
 * union and the build ranges behind a pointer to pointers.
 */
#include "scenarios.h"

#include <string.h>

#define LOAD(name) PFN_vk##name name = (PFN_vk##name)vkGetDeviceProcAddr(ctx->device, "vk" #name)

typedef struct {
    Buffer buffer;
    VkDeviceAddress address;
} AddressBuffer;

typedef struct {
    VkAccelerationStructureKHR structure;
    AddressBuffer storage;
} Structure;

static AddressBuffer address_buffer(Ctx *ctx, VkDeviceSize size, VkBufferUsageFlags usage,
                                    const void *data) {
    AddressBuffer result = {.buffer = {.size = size}};
    VkBufferCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = size,
        .usage = usage | VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
        .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
    };
    CHECK(vkCreateBuffer(ctx->device, &info, NULL, &result.buffer.buffer));
    VkMemoryRequirements requirements;
    vkGetBufferMemoryRequirements(ctx->device, result.buffer.buffer, &requirements);
    VkMemoryAllocateFlagsInfo flags_info = {
        .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO,
        .flags = VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT,
    };
    VkMemoryAllocateInfo allocate = {
        .sType = VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
        .pNext = &flags_info,
        .allocationSize = requirements.size,
        .memoryTypeIndex = memory_type(ctx, requirements.memoryTypeBits,
                                       VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT |
                                           VK_MEMORY_PROPERTY_HOST_COHERENT_BIT),
    };
    CHECK(vkAllocateMemory(ctx->device, &allocate, NULL, &result.buffer.memory));
    CHECK(vkBindBufferMemory(ctx->device, result.buffer.buffer, result.buffer.memory, 0));
    if (data != NULL) {
        buffer_write(ctx, &result.buffer, data, (size_t)size);
    }
    VkBufferDeviceAddressInfo address_info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO,
        .buffer = result.buffer.buffer,
    };
    result.address = vkGetBufferDeviceAddress(ctx->device, &address_info);
    return result;
}

static Structure structure_build(Ctx *ctx, VkAccelerationStructureTypeKHR type,
                                 const VkAccelerationStructureGeometryKHR *geometry,
                                 uint32_t primitive_count) {
    LOAD(GetAccelerationStructureBuildSizesKHR);
    LOAD(CreateAccelerationStructureKHR);
    LOAD(CmdBuildAccelerationStructuresKHR);
    Structure result;
    VkAccelerationStructureBuildGeometryInfoKHR build = {
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR,
        .type = type,
        .flags = VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR,
        .mode = VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR,
        .geometryCount = 1,
        .pGeometries = geometry,
    };
    VkAccelerationStructureBuildSizesInfoKHR sizes = {
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR,
    };
    GetAccelerationStructureBuildSizesKHR(ctx->device,
                                          VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR, &build,
                                          &primitive_count, &sizes);
    result.storage = address_buffer(ctx, sizes.accelerationStructureSize,
                                    VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR, NULL);
    VkAccelerationStructureCreateInfoKHR create_info = {
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR,
        .buffer = result.storage.buffer.buffer,
        .offset = 0,
        .size = sizes.accelerationStructureSize,
        .type = type,
    };
    CHECK(CreateAccelerationStructureKHR(ctx->device, &create_info, NULL, &result.structure));
    AddressBuffer scratch = address_buffer(ctx, sizes.buildScratchSize,
                                           VK_BUFFER_USAGE_STORAGE_BUFFER_BIT, NULL);
    build.dstAccelerationStructure = result.structure;
    build.scratchData.deviceAddress = scratch.address;
    VkAccelerationStructureBuildRangeInfoKHR range = {.primitiveCount = primitive_count};
    const VkAccelerationStructureBuildRangeInfoKHR *ranges = &range;
    VkCommandBuffer command_buffer = commands_begin(ctx);
    CmdBuildAccelerationStructuresKHR(command_buffer, 1, &build, &ranges);
    commands_submit(ctx, command_buffer);
    buffer_destroy(ctx, &scratch.buffer);
    return result;
}

static VkAccelerationStructureInstanceKHR instance(float scale_x, float scale_y, float x,
                                                   float y, float z, uint32_t custom_index,
                                                   uint32_t mask, uint32_t table_offset,
                                                   VkGeometryInstanceFlagsKHR flags,
                                                   VkDeviceAddress reference) {
    VkAccelerationStructureInstanceKHR result = {
        .transform = {{{scale_x, 0.0f, 0.0f, x}, {0.0f, scale_y, 0.0f, y}, {0.0f, 0.0f, 1.0f, z}}},
        .instanceCustomIndex = custom_index,
        .mask = mask,
        .instanceShaderBindingTableRecordOffset = table_offset,
        .flags = flags,
        .accelerationStructureReference = reference,
    };
    return result;
}

void scenario_ray_query(Ctx *ctx) {
    const uint32_t width = 256;
    const uint32_t height = 256;
    const VkFormat format = VK_FORMAT_R8G8B8A8_UNORM;
    static const float vertices[] = {
        -0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f, -0.5f, 0.5f, 0.0f, 0.5f, 0.5f, 0.0f,
    };
    static const uint16_t indices[] = {0, 1, 2, 2, 1, 3};
    LOAD(GetAccelerationStructureDeviceAddressKHR);
    LOAD(DestroyAccelerationStructureKHR);
    const VkBufferUsageFlags input_usage =
        VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR;

    AddressBuffer vertex_buffer = address_buffer(ctx, sizeof(vertices), input_usage, vertices);
    AddressBuffer index_buffer = address_buffer(ctx, sizeof(indices), input_usage, indices);
    VkAccelerationStructureGeometryKHR triangles = {
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR,
        .geometryType = VK_GEOMETRY_TYPE_TRIANGLES_KHR,
        .geometry = {.triangles =
                         {
                             .sType =
                                 VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR,
                             .vertexFormat = VK_FORMAT_R32G32B32_SFLOAT,
                             .vertexData = {.deviceAddress = vertex_buffer.address},
                             .vertexStride = 3 * sizeof(float),
                             .maxVertex = 3,
                             .indexType = VK_INDEX_TYPE_UINT16,
                             .indexData = {.deviceAddress = index_buffer.address},
                         }},
        .flags = VK_GEOMETRY_OPAQUE_BIT_KHR,
    };
    Structure bottom =
        structure_build(ctx, VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR, &triangles, 2);
    VkAccelerationStructureDeviceAddressInfoKHR bottom_address_info = {
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR,
        .accelerationStructure = bottom.structure,
    };
    VkDeviceAddress bottom_address =
        GetAccelerationStructureDeviceAddressKHR(ctx->device, &bottom_address_info);

    VkAccelerationStructureInstanceKHR instances[3] = {
        instance(0.5f, 0.375f, -0.5f, -0.5f, 0.0f, 1, 0xff, 0,
                 VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR, bottom_address),
        instance(0.75f, 0.625f, 0.375f, 0.25f, 0.5f, 2, 0x01, 5,
                 VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR, bottom_address),
        instance(4.0f, 4.0f, 0.0f, 0.0f, -0.5f, 3, 0x02, 9, 0, bottom_address),
    };
    AddressBuffer instance_buffer =
        address_buffer(ctx, sizeof(instances), input_usage, instances);
    VkAccelerationStructureGeometryKHR instance_geometry;
    memset(&instance_geometry, 0, sizeof(instance_geometry));
    instance_geometry.sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR;
    instance_geometry.geometryType = VK_GEOMETRY_TYPE_INSTANCES_KHR;
    instance_geometry.geometry.instances = (VkAccelerationStructureGeometryInstancesDataKHR){
        .sType = VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR,
        .arrayOfPointers = VK_FALSE,
        .data = {.deviceAddress = instance_buffer.address},
    };
    instance_geometry.flags = VK_GEOMETRY_OPAQUE_BIT_KHR;
    Structure top = structure_build(ctx, VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
                                    &instance_geometry, COUNT(instances));

    ImageDesc color_desc =
        image_desc(format, width, height,
                   VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT | VK_IMAGE_USAGE_TRANSFER_SRC_BIT,
                   VK_IMAGE_ASPECT_COLOR_BIT);
    Image color = image_create(ctx, &color_desc);
    VkDescriptorSetLayoutBinding binding = {0, VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR, 1,
                                            VK_SHADER_STAGE_FRAGMENT_BIT, NULL};
    VkDescriptorSetLayoutCreateInfo set_layout_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        .bindingCount = 1,
        .pBindings = &binding,
    };
    VkDescriptorSetLayout set_layout;
    CHECK(vkCreateDescriptorSetLayout(ctx->device, &set_layout_info, NULL, &set_layout));
    VkDescriptorPoolSize pool_size = {VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR, 1};
    VkDescriptorPoolCreateInfo pool_info = {
        .sType = VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
        .maxSets = 1,
        .poolSizeCount = 1,
        .pPoolSizes = &pool_size,
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
    VkWriteDescriptorSetAccelerationStructureKHR structure_write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR,
        .accelerationStructureCount = 1,
        .pAccelerationStructures = &top.structure,
    };
    VkWriteDescriptorSet write = {
        .sType = VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
        .pNext = &structure_write,
        .dstSet = set,
        .dstBinding = 0,
        .descriptorCount = 1,
        .descriptorType = VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR,
    };
    vkUpdateDescriptorSets(ctx->device, 1, &write, 0, NULL);

    VkPipelineLayout layout = pipeline_layout(ctx, &set_layout, 1, NULL, 0);
    VkShaderModule vertex = shader_module(ctx, "ray_query.vert.spv");
    VkShaderModule fragment = shader_module(ctx, "ray_query.frag.spv");
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

    VkCommandBuffer command_buffer = commands_begin(ctx);
    color_attachment_begin(command_buffer, color.image);
    const float clear[4] = {0.5f, 0.5f, 0.5f, 1.0f};
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
    image_destroy(ctx, &color);
    DestroyAccelerationStructureKHR(ctx->device, top.structure, NULL);
    buffer_destroy(ctx, &top.storage.buffer);
    buffer_destroy(ctx, &instance_buffer.buffer);
    DestroyAccelerationStructureKHR(ctx->device, bottom.structure, NULL);
    buffer_destroy(ctx, &bottom.storage.buffer);
    buffer_destroy(ctx, &index_buffer.buffer);
    buffer_destroy(ctx, &vertex_buffer.buffer);
}
