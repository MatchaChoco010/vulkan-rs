use vulkan::vk;

/// A vertex as the ray generation shader reads it through the vertex address. The
/// acceleration structure build reads only the first three components of `position`.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [f32; 4],
    pub color: [f32; 4],
}

pub const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [-0.9, -0.8, 0.0, 1.0],
        color: [0.95, 0.20, 0.16, 1.0],
    },
    Vertex {
        position: [0.9, -0.8, 0.0, 1.0],
        color: [0.16, 0.78, 0.34, 1.0],
    },
    Vertex {
        position: [0.0, 0.9, 0.0, 1.0],
        color: [0.22, 0.45, 1.0, 1.0],
    },
];

pub const INDICES: [u32; 3] = [0, 1, 2];

/// An instance in the layout of `VkAccelerationStructureInstanceKHR`, which the build
/// of the top level reads from the instance buffer.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Instance {
    transform: [[f32; 4]; 3],
    instance_custom_index_and_mask: u32,
    instance_shader_binding_table_record_offset_and_flags: u32,
    acceleration_structure_reference: u64,
}

impl Instance {
    /// The bottom level at `bottom_level_address`, untransformed, hit from both sides.
    pub fn new(bottom_level_address: vk::DeviceAddress) -> Self {
        // Let the bindings pack the C bit-fields, then copy the two storage words.
        let packed = vk::AccelerationStructureInstanceKHR::default()
            .instance_custom_index(0)
            .mask(0xff)
            .instance_shader_binding_table_record_offset(0)
            .flags(vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE);
        Self {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            instance_custom_index_and_mask: packed.bitfield0,
            instance_shader_binding_table_record_offset_and_flags: packed.bitfield1,
            acceleration_structure_reference: bottom_level_address,
        }
    }
}
