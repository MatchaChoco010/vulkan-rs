#include "scenarios.h"

static const char *const DESCRIPTOR_KINDS_EXTENSIONS[] = {VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME};

const Scenario SCENARIOS[] = {
    {"cts_smoke_triangle", scenario_cts_smoke_triangle, NULL, 0},
    {"samples_hello_triangle_1_3", scenario_samples_hello_triangle_1_3, NULL, 0},
    {"indexed_instanced", scenario_indexed_instanced, NULL, 0},
    {"blend", scenario_blend, NULL, 0},
    {"depth_stencil", scenario_depth_stencil, NULL, 0},
    {"texture_mipmap", scenario_texture_mipmap, NULL, 0},
    {"msaa_resolve", scenario_msaa_resolve, NULL, 0},
    {"multiple_render_targets", scenario_multiple_render_targets, NULL, 0},
    {"compute_image", scenario_compute_image, NULL, 0},
    {"descriptor_kinds", scenario_descriptor_kinds, DESCRIPTOR_KINDS_EXTENSIONS, COUNT(DESCRIPTOR_KINDS_EXTENSIONS)},
    {"secondary_queries", scenario_secondary_queries, NULL, 0},
};

const uint32_t SCENARIO_COUNT = COUNT(SCENARIOS);
