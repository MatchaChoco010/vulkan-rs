#include "scenarios.h"

static const char *const DESCRIPTOR_KINDS_EXTENSIONS[] = {VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME};

static const char *const SHADER_OBJECTS_EXTENSIONS[] = {
    VK_EXT_SHADER_OBJECT_EXTENSION_NAME,
};

static const char *const RAY_QUERY_EXTENSIONS[] = {
    VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME, VK_KHR_RAY_QUERY_EXTENSION_NAME, VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME,
};

const Scenario SCENARIOS[] = {
    {"cts_smoke_triangle", scenario_cts_smoke_triangle, NULL, 0, EXTENSION_FEATURES_NONE},
    {"samples_hello_triangle_1_3", scenario_samples_hello_triangle_1_3, NULL, 0, EXTENSION_FEATURES_NONE},
    {"indexed_instanced", scenario_indexed_instanced, NULL, 0, EXTENSION_FEATURES_NONE},
    {"blend", scenario_blend, NULL, 0, EXTENSION_FEATURES_NONE},
    {"depth_stencil", scenario_depth_stencil, NULL, 0, EXTENSION_FEATURES_NONE},
    {"texture_mipmap", scenario_texture_mipmap, NULL, 0, EXTENSION_FEATURES_NONE},
    {"msaa_resolve", scenario_msaa_resolve, NULL, 0, EXTENSION_FEATURES_NONE},
    {"multiple_render_targets", scenario_multiple_render_targets, NULL, 0, EXTENSION_FEATURES_NONE},
    {"compute_image", scenario_compute_image, NULL, 0, EXTENSION_FEATURES_NONE},
    {"descriptor_kinds", scenario_descriptor_kinds, DESCRIPTOR_KINDS_EXTENSIONS, COUNT(DESCRIPTOR_KINDS_EXTENSIONS),
     EXTENSION_FEATURES_NONE},
    {"secondary_queries", scenario_secondary_queries, NULL, 0, EXTENSION_FEATURES_NONE},
    {"indirect_draws", scenario_indirect_draws, NULL, 0, EXTENSION_FEATURES_NONE},
    {"tessellation_geometry", scenario_tessellation_geometry, NULL, 0, EXTENSION_FEATURES_NONE},
    {"image_views", scenario_image_views, NULL, 0, EXTENSION_FEATURES_NONE},
    {"dynamic_rendering_msaa", scenario_dynamic_rendering_msaa, NULL, 0, EXTENSION_FEATURES_NONE},
    {"descriptor_indexing", scenario_descriptor_indexing, NULL, 0, EXTENSION_FEATURES_NONE},
    {"shader_objects", scenario_shader_objects, SHADER_OBJECTS_EXTENSIONS, COUNT(SHADER_OBJECTS_EXTENSIONS),
     EXTENSION_FEATURES_SHADER_OBJECT},
    {"ray_query", scenario_ray_query, RAY_QUERY_EXTENSIONS, COUNT(RAY_QUERY_EXTENSIONS),
     EXTENSION_FEATURES_RAY_QUERY},
};

const uint32_t SCENARIO_COUNT = COUNT(SCENARIOS);

/* A call that the validation layer has to report: a buffer of size 0. The comparison runs
 * it to see that a validation message makes the programs fail. */
static void scenario_invalid_call(Ctx *ctx) {
    VkBufferCreateInfo info = {
        .sType = VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
        .size = 0,
        .usage = VK_BUFFER_USAGE_VERTEX_BUFFER_BIT,
        .sharingMode = VK_SHARING_MODE_EXCLUSIVE,
    };
    VkBuffer buffer;
    CHECK(vkCreateBuffer(ctx->device, &info, NULL, &buffer));
    vkDestroyBuffer(ctx->device, buffer, NULL);
}

const Scenario INVALID_CALL = {"invalid_call", scenario_invalid_call, NULL, 0,
                               EXTENSION_FEATURES_NONE};
