use glam::{Mat4, Vec3};
use vulkan::vk;

use crate::EXTENT;
use crate::heaps::DescriptorHandle;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

/// The per-instance vertex attributes.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Instance {
    pub translation_scale: [f32; 4],
    /// Which entry of the material buffer the sphere uses.
    pub material_index: u32,
    pub _pad: [u32; 3],
}

/// An entry of the material buffer, laid out as `MaterialData` in the shader.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Material {
    pub base_color_texture: DescriptorHandle,
    pub _pad: [u32; 2],
    pub tint: [f32; 4],
}

/// A checkerboard texture of [`TEXTURE_EXTENT`] in RGBA8.
pub struct Checker {
    pub name: &'static str,
    light: [u8; 4],
    dark: [u8; 4],
}

impl Checker {
    pub fn texels(&self) -> Vec<[u8; 4]> {
        let (width, height) = (TEXTURE_EXTENT.width, TEXTURE_EXTENT.height);
        (0..height)
            .flat_map(|y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| {
                if (x / 8 + y / 8) % 2 == 0 {
                    self.light
                } else {
                    self.dark
                }
            })
            .collect()
    }
}

pub const TEXTURE_EXTENT: vk::Extent2D = vk::Extent2D {
    width: 64,
    height: 64,
};

pub const TEXTURES: [Checker; 2] = [
    Checker {
        name: "warm checker texture",
        light: [255, 208, 102, 255],
        dark: [230, 74, 46, 255],
    },
    Checker {
        name: "cool checker texture",
        light: [142, 230, 197, 255],
        dark: [63, 125, 217, 255],
    },
];

/// One material per texture, which tints it.
pub fn materials(textures: [DescriptorHandle; TEXTURES.len()]) -> [Material; TEXTURES.len()] {
    let tints = [[1.0, 0.92, 0.86, 1.0], [0.82, 0.95, 1.0, 1.0]];
    std::array::from_fn(|index| Material {
        base_color_texture: textures[index],
        _pad: [0; 2],
        tint: tints[index],
    })
}

/// Four spheres in a row, alternating between the two materials.
pub const INSTANCES: [Instance; 4] = [
    Instance {
        translation_scale: [-1.35, 0.18, 4.0, 0.42],
        material_index: 0,
        _pad: [0; 3],
    },
    Instance {
        translation_scale: [-0.45, -0.16, 3.65, 0.48],
        material_index: 1,
        _pad: [0; 3],
    },
    Instance {
        translation_scale: [0.48, 0.15, 3.85, 0.45],
        material_index: 0,
        _pad: [0; 3],
    },
    Instance {
        translation_scale: [1.35, -0.12, 4.1, 0.44],
        material_index: 1,
        _pad: [0; 3],
    },
];

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

/// A unit sphere of `rings` rings of `segments` quads each, with the texture wrapped
/// around it once.
pub fn sphere(segments: u32, rings: u32) -> Mesh {
    let mut vertices = Vec::new();
    for y in 0..=rings {
        let v = y as f32 / rings as f32;
        let theta = v * std::f32::consts::PI;
        let (sin_theta, cos_theta) = theta.sin_cos();
        for x in 0..=segments {
            let u = x as f32 / segments as f32;
            let phi = u * std::f32::consts::TAU;
            let normal = [sin_theta * phi.cos(), cos_theta, sin_theta * phi.sin()];
            vertices.push(Vertex {
                position: normal,
                normal,
                uv: [u, v],
            });
        }
    }

    let mut indices = Vec::new();
    let row = segments + 1;
    for y in 0..rings {
        for x in 0..segments {
            let a = y * row + x;
            let b = a + 1;
            let c = a + row;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    Mesh { vertices, indices }
}

/// A camera at the origin that looks down +Z, where the spheres are.
pub fn clip_from_world() -> Mat4 {
    let view = glam::camera::rh::view::look_at_mat4(Vec3::ZERO, Vec3::Z, Vec3::Y);
    let projection = glam::camera::rh::proj::directx::perspective(
        48.0_f32.to_radians(),
        EXTENT.width as f32 / EXTENT.height as f32,
        0.1,
        16.0,
    );
    projection * view
}
