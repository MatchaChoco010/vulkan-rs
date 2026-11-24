mod blend;
mod compute_image;
mod cts_smoke_triangle;
mod depth_stencil;
mod descriptor_indexing;
mod descriptor_kinds;
mod dynamic_rendering_msaa;
mod image_views;
mod indexed_instanced;
mod indirect_draws;
mod msaa_resolve;
mod multiple_render_targets;
mod ray_query;
mod samples_hello_triangle_1_3;
mod secondary_queries;
mod shader_objects;
mod tessellation_geometry;
mod texture_mipmap;

use vulkan::vk;

use crate::render::{Ctx, ExtensionFeatures, Scenario};

pub const SCENARIOS: &[Scenario] = &[
    Scenario {
        name: "cts_smoke_triangle",
        run: cts_smoke_triangle::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "samples_hello_triangle_1_3",
        run: samples_hello_triangle_1_3::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "indexed_instanced",
        run: indexed_instanced::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "blend",
        run: blend::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "depth_stencil",
        run: depth_stencil::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "texture_mipmap",
        run: texture_mipmap::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "msaa_resolve",
        run: msaa_resolve::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "multiple_render_targets",
        run: multiple_render_targets::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "compute_image",
        run: compute_image::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "descriptor_kinds",
        run: descriptor_kinds::run,
        device_extensions: &[vk::KHR_PUSH_DESCRIPTOR_NAME],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "secondary_queries",
        run: secondary_queries::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "indirect_draws",
        run: indirect_draws::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "tessellation_geometry",
        run: tessellation_geometry::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "image_views",
        run: image_views::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "dynamic_rendering_msaa",
        run: dynamic_rendering_msaa::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "descriptor_indexing",
        run: descriptor_indexing::run,
        device_extensions: &[],
        extension_features: ExtensionFeatures::None,
    },
    Scenario {
        name: "shader_objects",
        run: shader_objects::run,
        device_extensions: &[vk::EXT_SHADER_OBJECT_NAME],
        extension_features: ExtensionFeatures::ShaderObject,
    },
    Scenario {
        name: "ray_query",
        run: ray_query::run,
        device_extensions: &[
            vk::KHR_ACCELERATION_STRUCTURE_NAME,
            vk::KHR_RAY_QUERY_NAME,
            vk::KHR_DEFERRED_HOST_OPERATIONS_NAME,
        ],
        extension_features: ExtensionFeatures::RayQuery,
    },
];

pub const INVALID_CALL: Scenario = Scenario {
    name: "invalid_call",
    run: invalid_call,
    device_extensions: &[],
    extension_features: ExtensionFeatures::None,
};

fn invalid_call(ctx: &mut Ctx) {
    let info = vk::BufferCreateInfo::default()
        .size(0)
        .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    unsafe {
        let buffer = ctx
            .device
            .create_buffer(&info, None)
            .unwrap_or_else(|error| {
                crate::render::fail(&format!("vkCreateBuffer returned {error}"))
            });
        ctx.device.destroy_buffer(buffer, None);
    }
}
