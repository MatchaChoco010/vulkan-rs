use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_alloc::{Allocator, MemoryLocation};
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::device::Gpu;
use vulkan_example_common::memory::{
    Buffer, buffer_device_address, create_aligned_buffer, upload_buffer,
};

use crate::geometry::{INDICES, Instance, VERTICES, Vertex};

/// An acceleration structure and the buffer that holds it.
pub struct AccelerationStructure {
    pub handle: vk::AccelerationStructureKHR,
    buffer: Buffer,
    pub size: u64,
    build_scratch_size: u64,
}

impl AccelerationStructure {
    /// The address that instances and descriptors refer to the acceleration structure
    /// by. It is not the address of the buffer that holds it.
    pub unsafe fn device_address(&self, device: &Device) -> vk::DeviceAddress {
        unsafe {
            device.get_acceleration_structure_device_address_khr(
                &vk::AccelerationStructureDeviceAddressInfoKHR::default()
                    .acceleration_structure(self.handle),
            )
        }
    }

    pub unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        unsafe {
            device.destroy_acceleration_structure_khr(self.handle, None);
            self.buffer.destroy(device, allocator)
        }
    }
}

/// The triangle, a bottom level that holds it, and a top level that holds one
/// instance of the bottom level.
pub struct Scene {
    /// The ray generation shader reads the vertices and indices of the triangle that a
    /// ray hits.
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub bottom_level: AccelerationStructure,
    pub top_level: AccelerationStructure,
}

impl Scene {
    pub unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        unsafe {
            self.top_level.destroy(device, allocator)?;
            self.bottom_level.destroy(device, allocator)?;
            self.index_buffer.destroy(device, allocator)?;
            self.vertex_buffer.destroy(device, allocator)
        }
    }
}

/// Uploads the triangle and builds both levels on the device.
pub unsafe fn build_scene(
    gpu: &Gpu,
    allocator: &mut Allocator,
    scratch_alignment: u64,
) -> Result<Scene> {
    unsafe {
        let device = &gpu.device;
        let vertex_buffer = upload_build_input(gpu, allocator, "triangle vertices", &VERTICES)?;
        let index_buffer = upload_build_input(gpu, allocator, "triangle indices", &INDICES)?;
        let bottom_geometries = [triangle_geometry(device, &vertex_buffer, &index_buffer)];
        let bottom_build = build_info(
            vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
            &bottom_geometries,
        );
        let bottom_level =
            create_acceleration_structure(device, allocator, "bottom level", &bottom_build)?;

        let instances = [Instance::new(bottom_level.device_address(device))];
        let instance_buffer = upload_build_input(gpu, allocator, "instances", &instances)?;
        let top_geometries = [instance_geometry(device, &instance_buffer)];
        let top_build = build_info(vk::AccelerationStructureTypeKHR::TOP_LEVEL, &top_geometries);
        let top_level = create_acceleration_structure(device, allocator, "top level", &top_build)?;

        // The builds run one after the other and share the scratch memory.
        let scratch = create_aligned_buffer(
            device,
            allocator,
            "build scratch",
            bottom_level
                .build_scratch_size
                .max(top_level.build_scratch_size),
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::GpuOnly,
            scratch_alignment,
        )?;
        let scratch_data = vk::DeviceOrHostAddressKHR::device_address(buffer_device_address(
            device,
            scratch.buffer,
        ));
        submit_and_wait(gpu, |command_buffer| {
            build(
                device,
                command_buffer,
                bottom_build
                    .dst_acceleration_structure(bottom_level.handle)
                    .scratch_data(scratch_data),
            );
            // The top level is built from the bottom level.
            build_barrier(
                device,
                command_buffer,
                vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
            );
            build(
                device,
                command_buffer,
                top_build
                    .dst_acceleration_structure(top_level.handle)
                    .scratch_data(scratch_data),
            );
            // The second scope reaches into the submission that traces rays.
            build_barrier(
                device,
                command_buffer,
                vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR,
            );
            Ok(())
        })?;
        // Only the builds read the scratch memory and the instances.
        scratch.destroy(device, allocator)?;
        instance_buffer.destroy(device, allocator)?;

        Ok(Scene {
            vertex_buffer,
            index_buffer,
            bottom_level,
            top_level,
        })
    }
}

/// A device-local buffer that a build reads through its device address.
unsafe fn upload_build_input<T: bytemuck::Pod>(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &str,
    values: &[T],
) -> Result<Buffer> {
    unsafe {
        upload_buffer(
            gpu,
            allocator,
            name,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            bytemuck::cast_slice(values),
        )
    }
}

/// The triangle as the geometry of the bottom level.
unsafe fn triangle_geometry(
    device: &Device,
    vertex_buffer: &Buffer,
    index_buffer: &Buffer,
) -> vk::AccelerationStructureGeometryKHR<'static> {
    unsafe {
        let triangles = vk::AccelerationStructureGeometryTrianglesDataKHR::default()
            .vertex_format(vk::Format::R32G32B32_SFLOAT)
            .vertex_data(vk::DeviceOrHostAddressConstKHR::device_address(
                buffer_device_address(device, vertex_buffer.buffer),
            ))
            .vertex_stride(size_of::<Vertex>() as u64)
            .max_vertex(VERTICES.len() as u32 - 1)
            .index_type(vk::IndexType::UINT32)
            .index_data(vk::DeviceOrHostAddressConstKHR::device_address(
                buffer_device_address(device, index_buffer.buffer),
            ));
        vk::AccelerationStructureGeometryKHR::default()
            .geometry_type(vk::GeometryTypeKHR::TRIANGLES)
            .geometry(vk::AccelerationStructureGeometryDataKHR::triangles(
                triangles,
            ))
            .flags(vk::GeometryFlagsKHR::OPAQUE)
    }
}

/// The instances in `instance_buffer`, an array of them, as the geometry of the top
/// level.
unsafe fn instance_geometry(
    device: &Device,
    instance_buffer: &Buffer,
) -> vk::AccelerationStructureGeometryKHR<'static> {
    let instances = vk::AccelerationStructureGeometryInstancesDataKHR::default()
        .array_of_pointers(false)
        .data(vk::DeviceOrHostAddressConstKHR::device_address(unsafe {
            buffer_device_address(device, instance_buffer.buffer)
        }));
    vk::AccelerationStructureGeometryKHR::default()
        .geometry_type(vk::GeometryTypeKHR::INSTANCES)
        .geometry(vk::AccelerationStructureGeometryDataKHR::instances(
            instances,
        ))
}

/// Records the build of a level with one primitive: the triangle in the bottom level,
/// the instance in the top level.
unsafe fn build(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    info: vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
) {
    let ranges = [vk::AccelerationStructureBuildRangeInfoKHR::default().primitive_count(1)];
    unsafe { device.cmd_build_acceleration_structures_khr(command_buffer, &[info], &[&ranges]) };
}

/// What a build of a level from `geometries` needs to know before the acceleration
/// structure and the scratch memory exist.
fn build_info<'a>(
    ty: vk::AccelerationStructureTypeKHR,
    geometries: &'a [vk::AccelerationStructureGeometryKHR<'a>],
) -> vk::AccelerationStructureBuildGeometryInfoKHR<'a> {
    vk::AccelerationStructureBuildGeometryInfoKHR::default()
        .ty(ty)
        .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE)
        .mode(vk::BuildAccelerationStructureModeKHR::BUILD)
        .geometries(geometries)
}

/// An acceleration structure large enough for `build`, whose geometries each have
/// one primitive.
unsafe fn create_acceleration_structure(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    build: &vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
) -> Result<AccelerationStructure> {
    unsafe {
        let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
        device.get_acceleration_structure_build_sizes_khr(
            vk::AccelerationStructureBuildTypeKHR::DEVICE,
            build,
            Some(&[1]),
            &mut sizes,
        );
        let buffer = create_aligned_buffer(
            device,
            allocator,
            name,
            sizes.acceleration_structure_size,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::GpuOnly,
            // The address that goes into a descriptor heap has to be a multiple of 256
            // (VUID-VkResourceDescriptorInfoEXT-type-11454). The acceleration structure
            // starts at offset 0 of this buffer, so the buffer is what gets aligned.
            256,
        )?;
        let info = vk::AccelerationStructureCreateInfoKHR::default()
            .buffer(buffer.buffer)
            .size(sizes.acceleration_structure_size)
            .ty(build.ty);
        let handle = match device.create_acceleration_structure_khr(&info, None) {
            Ok(handle) => handle,
            Err(error) => {
                buffer.destroy(device, allocator)?;
                return Err(error.into());
            }
        };
        Ok(AccelerationStructure {
            handle,
            buffer,
            size: sizes.acceleration_structure_size,
            build_scratch_size: sizes.build_scratch_size,
        })
    }
}

/// Makes what a build wrote visible to the reads of acceleration structures in `stage`.
unsafe fn build_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    stage: vk::PipelineStageFlags,
) {
    let barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR)
        .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR);
    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
            stage,
            vk::DependencyFlags::empty(),
            &[barrier],
            &[],
            &[],
        );
    }
}
