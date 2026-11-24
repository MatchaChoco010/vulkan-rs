mod blend;
mod compute_image;
mod cts_smoke_triangle;
mod depth_stencil;
mod descriptor_kinds;
mod indexed_instanced;
mod msaa_resolve;
mod multiple_render_targets;
mod samples_hello_triangle_1_3;
mod secondary_queries;
mod texture_mipmap;

use vulkan::vk;

use crate::render::Scenario;

pub const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "cts_smoke_triangle",
        run: cts_smoke_triangle::run,
        device_extensions: &[],
    },
    Scenario {
        name: "samples_hello_triangle_1_3",
        run: samples_hello_triangle_1_3::run,
        device_extensions: &[],
    },
    Scenario {
        name: "indexed_instanced",
        run: indexed_instanced::run,
        device_extensions: &[],
    },
    Scenario {
        name: "blend",
        run: blend::run,
        device_extensions: &[],
    },
    Scenario {
        name: "depth_stencil",
        run: depth_stencil::run,
        device_extensions: &[],
    },
    Scenario {
        name: "texture_mipmap",
        run: texture_mipmap::run,
        device_extensions: &[],
    },
    Scenario {
        name: "msaa_resolve",
        run: msaa_resolve::run,
        device_extensions: &[],
    },
    Scenario {
        name: "multiple_render_targets",
        run: multiple_render_targets::run,
        device_extensions: &[],
    },
    Scenario {
        name: "compute_image",
        run: compute_image::run,
        device_extensions: &[],
    },
    Scenario {
        name: "descriptor_kinds",
        run: descriptor_kinds::run,
        device_extensions: &[vk::KHR_PUSH_DESCRIPTOR_NAME],
    },
    Scenario {
        name: "secondary_queries",
        run: secondary_queries::run,
        device_extensions: &[],
    },
];
