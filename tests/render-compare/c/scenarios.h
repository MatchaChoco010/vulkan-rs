#ifndef SCENARIOS_H
#define SCENARIOS_H

#include "render.h"

void scenario_cts_smoke_triangle(Ctx *ctx);
void scenario_samples_hello_triangle_1_3(Ctx *ctx);
void scenario_indexed_instanced(Ctx *ctx);
void scenario_blend(Ctx *ctx);
void scenario_depth_stencil(Ctx *ctx);
void scenario_texture_mipmap(Ctx *ctx);
void scenario_msaa_resolve(Ctx *ctx);
void scenario_multiple_render_targets(Ctx *ctx);
void scenario_compute_image(Ctx *ctx);
void scenario_descriptor_kinds(Ctx *ctx);
void scenario_secondary_queries(Ctx *ctx);
void scenario_indirect_draws(Ctx *ctx);
void scenario_tessellation_geometry(Ctx *ctx);
void scenario_image_views(Ctx *ctx);
void scenario_dynamic_rendering_msaa(Ctx *ctx);
void scenario_descriptor_indexing(Ctx *ctx);
void scenario_shader_objects(Ctx *ctx);
void scenario_ray_query(Ctx *ctx);

extern const Scenario SCENARIOS[];
extern const uint32_t SCENARIO_COUNT;
extern const Scenario INVALID_CALL;

#endif
