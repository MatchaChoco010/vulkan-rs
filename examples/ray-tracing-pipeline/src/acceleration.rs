use anyhow::Result;
use vulkan::{Device, vk};
use vulkan_alloc::MemoryLocation;
use vulkan_example_common::commands::submit_and_wait;
use vulkan_example_common::device::{Gpu, Owned};
use vulkan_example_common::memory::{
    Buffer, buffer_device_address, create_aligned_buffer, upload_aligned_buffer,
};

use crate::geometry::{INDICES, Instance, VERTICES, Vertex};

/// An acceleration structure and the buffer that holds it. The fields drop in order,
/// the acceleration structure before its buffer.
pub struct AccelerationStructure {
    pub handle: Owned<vk::AccelerationStructureKHR>,
    _buffer: Owned<Buffer>,
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
                    .acceleration_structure(*self.handle),
            )
        }
    }
}

/// The triangle, a bottom level that holds it, and a top level that holds one
/// instance of the bottom level. The fields drop in order, the reverse of creation.
pub struct Scene {
    pub top_level: AccelerationStructure,
    /// Kept for the top level, whose instance refers to it.
    _bottom_level: AccelerationStructure,
    /// The ray generation shader reads the vertices and indices of the triangle that a
    /// ray hits.
    pub index_buffer: Owned<Buffer>,
    pub vertex_buffer: Owned<Buffer>,
}

/// Uploads the triangle and builds both levels on the device.
pub unsafe fn build_scene(gpu: &Gpu, scratch_alignment: u64) -> Result<Scene> {
    unsafe {
        let device = &gpu.device;
        // The vertices and indices have to start at a multiple of the size of a
        // component and of an index, the instances at a multiple of 16 bytes.
        let vertex_buffer = upload_build_input(gpu, "triangle vertices", &VERTICES, 4)?;
        let index_buffer = upload_build_input(gpu, "triangle indices", &INDICES, 4)?;
        let bottom_geometries = [triangle_geometry(gpu, &vertex_buffer, &index_buffer)];
        let bottom_build = build_info(
            vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
            &bottom_geometries,
        );
        let bottom_level = create_acceleration_structure(gpu, "bottom level", &bottom_build)?;

        let instances = [Instance::new(bottom_level.device_address(device))];
        let instance_buffer = upload_build_input(gpu, "instances", &instances, 16)?;
        let top_geometries = [instance_geometry(gpu, &instance_buffer)];
        let top_build = build_info(vk::AccelerationStructureTypeKHR::TOP_LEVEL, &top_geometries);
        let top_level = create_acceleration_structure(gpu, "top level", &top_build)?;

        // The builds run one after the other and share the scratch memory.
        let scratch = create_aligned_buffer(
            gpu,
            "build scratch",
            bottom_level
                .build_scratch_size
                .max(top_level.build_scratch_size),
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            MemoryLocation::GpuOnly,
            scratch_alignment,
        )?;
        let scratch_data =
            vk::DeviceOrHostAddressKHR::device_address(buffer_device_address(gpu, scratch.buffer));
        submit_and_wait(gpu, |command_buffer| {
            build(
                device,
                command_buffer,
                bottom_build
                    .dst_acceleration_structure(*bottom_level.handle)
                    .scratch_data(scratch_data),
            );
            // The top level is built from the bottom level, and its build writes the
            // scratch memory that the first build wrote.
            build_barrier(
                device,
                command_buffer,
                vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
                vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR
                    | vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR,
            );
            build(
                device,
                command_buffer,
                top_build
                    .dst_acceleration_structure(*top_level.handle)
                    .scratch_data(scratch_data),
            );
            // The second scope reaches into the submission that traces rays.
            build_barrier(
                device,
                command_buffer,
                vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR,
                vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR,
            );
            Ok(())
        })?;
        // Only the builds read the scratch memory and the instances, which go at the
        // end of this function.
        Ok(Scene {
            top_level,
            _bottom_level: bottom_level,
            index_buffer,
            vertex_buffer,
        })
    }
}

/// A device-local buffer that a build reads through its device address, which has to
/// be a multiple of `alignment`.
unsafe fn upload_build_input<T: bytemuck::Pod>(
    gpu: &Gpu,
    name: &str,
    values: &[T],
    alignment: u64,
) -> Result<Owned<Buffer>> {
    unsafe {
        upload_aligned_buffer(
            gpu,
            name,
            vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            bytemuck::cast_slice(values),
            alignment,
        )
    }
}

/// The triangle as the geometry of the bottom level.
unsafe fn triangle_geometry(
    gpu: &Gpu,
    vertex_buffer: &Buffer,
    index_buffer: &Buffer,
) -> vk::AccelerationStructureGeometryKHR<'static> {
    unsafe {
        let triangles = vk::AccelerationStructureGeometryTrianglesDataKHR::default()
            .vertex_format(vk::Format::R32G32B32_SFLOAT)
            .vertex_data(vk::DeviceOrHostAddressConstKHR::device_address(
                buffer_device_address(gpu, vertex_buffer.buffer),
            ))
            .vertex_stride(size_of::<Vertex>() as u64)
            .max_vertex(VERTICES.len() as u32 - 1)
            .index_type(vk::IndexType::UINT32)
            .index_data(vk::DeviceOrHostAddressConstKHR::device_address(
                buffer_device_address(gpu, index_buffer.buffer),
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
    gpu: &Gpu,
    instance_buffer: &Buffer,
) -> vk::AccelerationStructureGeometryKHR<'static> {
    let instances = vk::AccelerationStructureGeometryInstancesDataKHR::default()
        .array_of_pointers(false)
        .data(vk::DeviceOrHostAddressConstKHR::device_address(unsafe {
            buffer_device_address(gpu, instance_buffer.buffer)
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
    unsafe {
        device.cmd_build_acceleration_structures_khr(command_buffer, &[info], &[Some(&ranges)])
    };
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
    gpu: &Gpu,
    name: &str,
    build: &vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
) -> Result<AccelerationStructure> {
    unsafe {
        let device = &gpu.device;
        let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
        device.get_acceleration_structure_build_sizes_khr(
            vk::AccelerationStructureBuildTypeKHR::DEVICE,
            build,
            Some(&[1]),
            &mut sizes,
        );
        let buffer = create_aligned_buffer(
            gpu,
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
        let handle = gpu.own(device.create_acceleration_structure_khr(&info, None)?);
        Ok(AccelerationStructure {
            handle,
            _buffer: buffer,
            size: sizes.acceleration_structure_size,
            build_scratch_size: sizes.build_scratch_size,
        })
    }
}

/// Makes what a build wrote, to the acceleration structure and to the scratch memory,
/// visible to the `access`es in `stage`.
unsafe fn build_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    stage: vk::PipelineStageFlags,
    access: vk::AccessFlags,
) {
    let barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR)
        .dst_access_mask(access);
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
