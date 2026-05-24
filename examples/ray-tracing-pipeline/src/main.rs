#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::{Result, anyhow};
use glam::{Mat4, Vec3};
use std::ffi::{CStr, CString, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() -> Result<()> {
    unsafe {
        let entry = Entry::load()?;
        let mut instance_extensions = Vec::new();
        if has_instance_extension(&entry, vk::EXT_DEBUG_UTILS_NAME)? {
            instance_extensions.push(vk::EXT_DEBUG_UTILS_NAME);
        }
        let debug_utils_enabled = instance_extensions.contains(&vk::EXT_DEBUG_UTILS_NAME);
        let instance = create_instance(&entry, &instance_extensions)?;
        let debug_messenger = create_debug_messenger(&instance, debug_utils_enabled)?;
        let Some(selection) = pick_physical_device(&instance)? else {
            println!("skipped: no device with KHR ray tracing pipeline support");
            if !debug_messenger.is_null() {
                instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
            }
            instance.destroy_instance(None);
            return Ok(());
        };
        let (device, queue) =
            create_device_and_queue(&instance, selection.physical_device, selection.queue_family)?;
        let physical_properties =
            instance.get_physical_device_properties(selection.physical_device);
        let memory_properties =
            instance.get_physical_device_memory_properties(selection.physical_device);
        let mut allocator = Allocator::new(AllocatorCreateDesc::new(
            &device,
            memory_properties,
            physical_properties,
        ));

        let command_pool = create_command_pool(&device, selection.queue_family)?;
        let command_buffer = allocate_command_buffer(&device, command_pool)?;
        let fence = create_fence(&device)?;
        let output = create_output_buffer(&device, &mut allocator)?;
        initialize_output(&mut allocator, &output.allocation)?;
        let scene = create_scene_acceleration_structures(
            &device,
            &mut allocator,
            command_buffer,
            queue,
            fence,
            &selection.acceleration_structure_properties,
        )?;
        let descriptor_set_layout = create_descriptor_set_layout(&device)?;
        let descriptor_pool = create_descriptor_pool(&device)?;
        let descriptor_set =
            allocate_descriptor_set(&device, descriptor_pool, descriptor_set_layout)?;
        update_descriptor_set(
            &device,
            descriptor_set,
            scene.top_level.handle,
            output.buffer,
        );
        let pipeline_layout = create_pipeline_layout(&device, descriptor_set_layout)?;
        let pipeline = create_ray_tracing_pipeline(&device, pipeline_layout)?;
        let shader_binding_table = create_shader_binding_table_buffer(
            &device,
            &mut allocator,
            pipeline,
            &selection.ray_tracing_properties,
        )?;

        let begin = vk::CommandBufferBeginInfo::default();
        device.begin_command_buffer(command_buffer, &begin)?;
        device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::RAY_TRACING_KHR,
            pipeline,
        );
        let descriptor_sets = [descriptor_set];
        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::RAY_TRACING_KHR,
            pipeline_layout,
            0,
            Some(&descriptor_sets),
            &[],
        );
        let push_constants = RayTracingPushConstants {
            vertex_address: buffer_device_address(&device, scene.vertex_buffer.buffer),
            index_address: buffer_device_address(&device, scene.index_buffer.buffer),
            camera_origin: ray_tracing_camera_origin().to_array(),
            _pad0: 0.0,
            world_from_clip: ray_tracing_world_from_clip().to_cols_array_2d(),
            background_bottom: [0.03, 0.05, 0.09],
            _pad1: 0.0,
            background_top: [0.24, 0.35, 0.58],
            _pad2: 0.0,
        };
        device.cmd_push_constants(
            command_buffer,
            pipeline_layout,
            vk::ShaderStageFlags::RAYGEN_KHR,
            0,
            bytemuck::bytes_of(&push_constants),
        );
        let address_info =
            vk::BufferDeviceAddressInfo::default().buffer(shader_binding_table.buffer);
        let record_stride = align_up(
            selection.ray_tracing_properties.shader_group_handle_size as u64,
            selection.ray_tracing_properties.shader_group_base_alignment as u64,
        );
        let base_address = device.get_buffer_device_address(&address_info);
        let raygen_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(base_address)
            .stride(record_stride)
            .size(record_stride);
        let miss_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(base_address + record_stride)
            .stride(record_stride)
            .size(record_stride);
        let hit_region = vk::StridedDeviceAddressRegionKHR::default()
            .device_address(base_address + record_stride * 2)
            .stride(record_stride)
            .size(record_stride);
        let callable_region = vk::StridedDeviceAddressRegionKHR::default();
        device.cmd_trace_rays_khr(
            command_buffer,
            &raygen_region,
            &miss_region,
            &hit_region,
            &callable_region,
            WIDTH,
            HEIGHT,
            1,
        );
        let output_barrier = vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::SHADER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .buffer(output.buffer)
            .offset(0)
            .size((WIDTH * HEIGHT * 4) as u64);
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR,
            vk::PipelineStageFlags::HOST,
            vk::DependencyFlags::empty(),
            &[],
            &[output_barrier],
            &[],
        );
        device.end_command_buffer(command_buffer)?;
        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;
        verify_and_write_ppm(&mut allocator, &output.allocation, "ray_tracing.ppm")?;

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_descriptor_pool(descriptor_pool, None);
        device.destroy_descriptor_set_layout(descriptor_set_layout, None);
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        scene.destroy(&device, &mut allocator)?;
        device.destroy_buffer(shader_binding_table.buffer, None);
        allocator.free(shader_binding_table.allocation)?;
        device.destroy_buffer(output.buffer, None);
        allocator.free(output.allocation)?;
        drop(allocator);
        device.destroy_device(None);
        if !debug_messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
        }
        instance.destroy_instance(None);
    }

    println!("wrote ray_tracing.ppm");
    Ok(())
}

struct PhysicalDeviceSelection {
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    ray_tracing_properties: vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'static>,
    acceleration_structure_properties:
        vk::PhysicalDeviceAccelerationStructurePropertiesKHR<'static>,
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
}

struct AccelerationStructureAllocation {
    handle: vk::AccelerationStructureKHR,
    buffer: BufferAllocation,
    build_scratch_size: u64,
}

struct SceneAccelerationStructures {
    vertex_buffer: BufferAllocation,
    index_buffer: BufferAllocation,
    instance_buffer: BufferAllocation,
    scratch_buffer: BufferAllocation,
    bottom_level: AccelerationStructureAllocation,
    top_level: AccelerationStructureAllocation,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct RayTracingPushConstants {
    vertex_address: u64,
    index_address: u64,
    camera_origin: [f32; 3],
    _pad0: f32,
    world_from_clip: [[f32; 4]; 4],
    background_bottom: [f32; 3],
    _pad1: f32,
    background_top: [f32; 3],
    _pad2: f32,
}

fn ray_tracing_camera_origin() -> Vec3 {
    Vec3::new(0.0, 0.0, -2.6)
}

fn ray_tracing_world_from_clip() -> Mat4 {
    let origin = ray_tracing_camera_origin();
    let view = Mat4::look_at_rh(origin, Vec3::ZERO, Vec3::Y);
    let projection = Mat4::perspective_rh(
        50.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        1000.0,
    );
    (projection * view).inverse()
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct PodAccelerationStructureInstance {
    transform: [[f32; 4]; 3],
    instance_custom_index_and_mask: u32,
    instance_shader_binding_table_record_offset_and_flags: u32,
    acceleration_structure_reference: u64,
}

impl PodAccelerationStructureInstance {
    fn new(acceleration_structure_reference: u64) -> Self {
        Self {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
            ],
            instance_custom_index_and_mask: vk::Packed24_8::new(0, 0xff).0,
            instance_shader_binding_table_record_offset_and_flags: vk::Packed24_8::new(
                0,
                vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE_KHR.as_raw() as u8,
            )
            .0,
            acceleration_structure_reference,
        }
    }
}

impl SceneAccelerationStructures {
    unsafe fn destroy(self, device: &Device, allocator: &mut Allocator) -> Result<()> {
        device.destroy_acceleration_structure_khr(self.top_level.handle, None);
        device.destroy_buffer(self.top_level.buffer.buffer, None);
        allocator.free(self.top_level.buffer.allocation)?;
        device.destroy_acceleration_structure_khr(self.bottom_level.handle, None);
        device.destroy_buffer(self.bottom_level.buffer.buffer, None);
        allocator.free(self.bottom_level.buffer.allocation)?;
        device.destroy_buffer(self.scratch_buffer.buffer, None);
        allocator.free(self.scratch_buffer.allocation)?;
        device.destroy_buffer(self.instance_buffer.buffer, None);
        allocator.free(self.instance_buffer.allocation)?;
        device.destroy_buffer(self.index_buffer.buffer, None);
        allocator.free(self.index_buffer.allocation)?;
        device.destroy_buffer(self.vertex_buffer.buffer, None);
        allocator.free(self.vertex_buffer.allocation)?;
        Ok(())
    }
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-ray-tracing-pipeline")?;
    let enabled_layers = validation_layers(entry)?;
    let enabled_layer_names = enabled_layers
        .iter()
        .map(|layer| layer.as_ptr())
        .collect::<Vec<_>>();
    let extension_names = extensions
        .iter()
        .map(|extension| extension.as_ptr())
        .collect::<Vec<_>>();
    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name.as_c_str())
        .engine_name(app_name.as_c_str())
        .api_version(vk::make_api_version(0, 1, 2, 0));
    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_layer_names(&enabled_layer_names)
        .enabled_extension_names(&extension_names);
    Ok(entry.create_instance(&create_info, None)?)
}

unsafe fn has_instance_extension(entry: &Entry, name: &CStr) -> Result<bool> {
    Ok(entry
        .enumerate_instance_extension_properties(None)?
        .iter()
        .any(|extension| CStr::from_ptr(extension.extension_name.as_ptr()) == name))
}

unsafe fn validation_layers(entry: &Entry) -> Result<Vec<&'static CStr>> {
    let validation = c"VK_LAYER_KHRONOS_validation";
    let available = entry.enumerate_instance_layer_properties()?;
    let has_validation = available
        .iter()
        .any(|layer| CStr::from_ptr(layer.layer_name.as_ptr()) == validation);
    if has_validation {
        Ok(vec![validation])
    } else {
        Err(anyhow!("VK_LAYER_KHRONOS_validation is required for this example"))
    }
}

unsafe fn create_debug_messenger(
    instance: &Instance,
    enabled: bool,
) -> Result<vk::DebugUtilsMessengerEXT> {
    if !enabled {
        return Ok(vk::DebugUtilsMessengerEXT::null());
    }
    let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL_EXT
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION_EXT
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE_EXT,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));
    Ok(instance.create_debug_utils_messenger_ext(&info, None)?)
}

unsafe extern "system" fn vulkan_debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut c_void,
) -> u32 {
    if !data.is_null() && !(*data).p_message.is_null() {
        eprintln!("{}", CStr::from_ptr((*data).p_message).to_string_lossy());
    }
    vk::FALSE
}

unsafe fn pick_physical_device(instance: &Instance) -> Result<Option<PhysicalDeviceSelection>> {
    let required_extensions = [
        vk::KHR_DEFERRED_HOST_OPERATIONS_NAME,
        vk::KHR_ACCELERATION_STRUCTURE_NAME,
        vk::KHR_RAY_TRACING_PIPELINE_NAME,
        vk::KHR_BUFFER_DEVICE_ADDRESS_NAME,
    ];
    let mut best = None;
    for physical_device in instance.enumerate_physical_devices()? {
        let extensions = instance.enumerate_device_extension_properties(physical_device, None)?;
        if required_extensions
            .iter()
            .any(|name| !has_extension(&extensions, name))
        {
            continue;
        }

        let mut buffer_device_address_features =
            vk::PhysicalDeviceBufferDeviceAddressFeatures::default();
        let mut acceleration_structure_features =
            vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
        let mut ray_tracing_features = vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default();
        let shader_int64 = {
            let mut features2 = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut buffer_device_address_features)
                .push_next(&mut acceleration_structure_features)
                .push_next(&mut ray_tracing_features);
            instance.get_physical_device_features2(physical_device, &mut features2);
            features2.features.shader_int64
        };
        if ray_tracing_features.ray_tracing_pipeline == 0
            || acceleration_structure_features.acceleration_structure == 0
            || buffer_device_address_features.buffer_device_address == 0
            || shader_int64 == 0
        {
            continue;
        }
        let mut ray_tracing_properties =
            vk::PhysicalDeviceRayTracingPipelinePropertiesKHR::default();
        let mut acceleration_structure_properties =
            vk::PhysicalDeviceAccelerationStructurePropertiesKHR::default();
        {
            let mut properties2 = vk::PhysicalDeviceProperties2::default()
                .push_next(&mut ray_tracing_properties)
                .push_next(&mut acceleration_structure_properties);
            instance.get_physical_device_properties2(physical_device, &mut properties2);
        }

        let queue_families =
            instance.get_physical_device_queue_family_properties(physical_device)?;
        let Some(queue_family) = queue_families
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::COMPUTE))
        else {
            continue;
        };
        let properties = instance.get_physical_device_properties(physical_device);
        let score = match properties.device_type {
            vk::PhysicalDeviceType::DISCRETE_GPU => 3,
            vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
            vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
            _ => 0,
        };
        if best
            .as_ref()
            .is_none_or(|(best_score, _)| score > *best_score)
        {
            best = Some((
                score,
                PhysicalDeviceSelection {
                    physical_device,
                    queue_family: queue_family as u32,
                    ray_tracing_properties,
                    acceleration_structure_properties,
                },
            ));
        }
    }
    Ok(best.map(|(_, selection)| selection))
}

unsafe fn create_device_and_queue(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
) -> Result<(Device, vk::Queue)> {
    let priorities = [1.0_f32];
    let queue_info = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let extension_names = [
        vk::KHR_DEFERRED_HOST_OPERATIONS_NAME.as_ptr(),
        vk::KHR_ACCELERATION_STRUCTURE_NAME.as_ptr(),
        vk::KHR_RAY_TRACING_PIPELINE_NAME.as_ptr(),
        vk::KHR_BUFFER_DEVICE_ADDRESS_NAME.as_ptr(),
    ];
    let mut buffer_device_address_features =
        vk::PhysicalDeviceBufferDeviceAddressFeatures::default().buffer_device_address(true);
    let mut acceleration_structure_features =
        vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default().acceleration_structure(true);
    let mut ray_tracing_features =
        vk::PhysicalDeviceRayTracingPipelineFeaturesKHR::default().ray_tracing_pipeline(true);
    let enabled_features = vk::PhysicalDeviceFeatures::default().shader_int64(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&extension_names)
        .enabled_features(&enabled_features)
        .push_next(&mut buffer_device_address_features)
        .push_next(&mut acceleration_structure_features)
        .push_next(&mut ray_tracing_features);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

unsafe fn create_scene_acceleration_structures(
    device: &Device,
    allocator: &mut Allocator,
    command_buffer: vk::CommandBuffer,
    queue: vk::Queue,
    fence: vk::Fence,
    properties: &vk::PhysicalDeviceAccelerationStructurePropertiesKHR<'_>,
) -> Result<SceneAccelerationStructures> {
    let vertices = [
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
    let indices = [0_u32, 1, 2];
    let vertex_buffer = create_gpu_buffer_from_data(
        device,
        allocator,
        command_buffer,
        queue,
        fence,
        vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        &vertices,
    )?;

    let index_buffer = create_gpu_buffer_from_data(
        device,
        allocator,
        command_buffer,
        queue,
        fence,
        vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        &indices,
    )?;

    let vertex_address = buffer_device_address(device, vertex_buffer.buffer);
    let index_address = buffer_device_address(device, index_buffer.buffer);
    let triangles = vk::AccelerationStructureGeometryTrianglesDataKHR::default()
        .vertex_format(vk::Format::R32G32B32_SFLOAT)
        .vertex_data(vk::DeviceOrHostAddressConstKHR::device_address(
            vertex_address,
        ))
        .vertex_stride(core::mem::size_of::<Vertex>() as u64)
        .max_vertex(2)
        .index_type(vk::IndexType::UINT32)
        .index_data(vk::DeviceOrHostAddressConstKHR::device_address(
            index_address,
        ));
    let blas_geometry = vk::AccelerationStructureGeometryKHR::default()
        .geometry_type(vk::GeometryTypeKHR::GEOMETRY_TYPE_TRIANGLES_KHR)
        .geometry(vk::AccelerationStructureGeometryDataKHR::triangles(
            triangles,
        ))
        .flags(vk::GeometryFlagsKHR::OPAQUE_KHR);
    let blas_geometries = [blas_geometry];
    let blas_build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
        .r#type(vk::AccelerationStructureTypeKHR::ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR)
        .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE_KHR)
        .mode(vk::BuildAccelerationStructureModeKHR::BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR)
        .geometries(&blas_geometries);
    let bottom_level =
        create_acceleration_structure(device, allocator, &blas_build_info, &[1], "bottom level")?;
    let bottom_level_address = device.get_acceleration_structure_device_address_khr(
        &vk::AccelerationStructureDeviceAddressInfoKHR::default()
            .acceleration_structure(bottom_level.handle),
    );

    let instances = [PodAccelerationStructureInstance::new(bottom_level_address)];
    let instance_buffer = create_gpu_buffer_from_data(
        device,
        allocator,
        command_buffer,
        queue,
        fence,
        vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        &instances,
    )?;

    let instances_data = vk::AccelerationStructureGeometryInstancesDataKHR::default()
        .array_of_pointers(false)
        .data(vk::DeviceOrHostAddressConstKHR::device_address(
            buffer_device_address(device, instance_buffer.buffer),
        ));
    let tlas_geometry = vk::AccelerationStructureGeometryKHR::default()
        .geometry_type(vk::GeometryTypeKHR::GEOMETRY_TYPE_INSTANCES_KHR)
        .geometry(vk::AccelerationStructureGeometryDataKHR::instances(
            instances_data,
        ));
    let tlas_geometries = [tlas_geometry];
    let tlas_build_info = vk::AccelerationStructureBuildGeometryInfoKHR::default()
        .r#type(vk::AccelerationStructureTypeKHR::ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR)
        .flags(vk::BuildAccelerationStructureFlagsKHR::PREFER_FAST_TRACE_KHR)
        .mode(vk::BuildAccelerationStructureModeKHR::BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR)
        .geometries(&tlas_geometries);
    let top_level =
        create_acceleration_structure(device, allocator, &tlas_build_info, &[1], "top level")?;

    let scratch_size = bottom_level
        .build_scratch_size
        .max(top_level.build_scratch_size);
    let scratch_buffer = {
        let info = vk::BufferCreateInfo::default()
            .size(scratch_size)
            .usage(
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = device.create_buffer(&info, None)?;
        let mut requirements = device.get_buffer_memory_requirements(buffer);
        requirements.alignment = requirements
            .alignment
            .max(properties.min_acceleration_structure_scratch_offset_alignment as u64);
        let allocation = match allocator.allocate(AllocationCreateDesc {
            name: Some("ray tracing acceleration scratch"),
            requirements,
            location: MemoryLocation::GpuOnly,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        }) {
            Ok(allocation) => allocation,
            Err(err) => {
                device.destroy_buffer(buffer, None);
                return Err(err.into());
            }
        };
        if let Err(err) =
            device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
        {
            let _ = allocator.free(allocation);
            device.destroy_buffer(buffer, None);
            return Err(err.into());
        }
        BufferAllocation { buffer, allocation }
    };
    let scratch_address = buffer_device_address(device, scratch_buffer.buffer);

    let blas_build_ranges = [vk::AccelerationStructureBuildRangeInfoKHR::default()
        .primitive_count(1)
        .primitive_offset(0)
        .first_vertex(0)
        .transform_offset(0)];
    let tlas_build_ranges = [vk::AccelerationStructureBuildRangeInfoKHR::default()
        .primitive_count(1)
        .primitive_offset(0)
        .first_vertex(0)
        .transform_offset(0)];
    let blas_build_info = blas_build_info
        .dst_acceleration_structure(bottom_level.handle)
        .scratch_data(vk::DeviceOrHostAddressKHR::device_address(scratch_address));
    let tlas_build_info = tlas_build_info
        .dst_acceleration_structure(top_level.handle)
        .scratch_data(vk::DeviceOrHostAddressKHR::device_address(scratch_address));

    let begin =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    device.begin_command_buffer(command_buffer, &begin)?;
    let upload_barriers = [
        vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR)
            .buffer(vertex_buffer.buffer)
            .offset(0)
            .size(core::mem::size_of_val(&vertices) as u64),
        vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR)
            .buffer(index_buffer.buffer)
            .offset(0)
            .size(core::mem::size_of_val(&indices) as u64),
        vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR)
            .buffer(instance_buffer.buffer)
            .offset(0)
            .size(core::mem::size_of_val(&instances) as u64),
    ];
    device.cmd_pipeline_barrier(
        command_buffer,
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
        vk::DependencyFlags::empty(),
        &[],
        &upload_barriers,
        &[],
    );
    device.cmd_build_acceleration_structures_khr(
        command_buffer,
        &[blas_build_info],
        &[&blas_build_ranges],
    );
    let build_barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR)
        .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR);
    device.cmd_pipeline_barrier(
        command_buffer,
        vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
        vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
        vk::DependencyFlags::empty(),
        &[build_barrier],
        &[],
        &[],
    );
    device.cmd_build_acceleration_structures_khr(
        command_buffer,
        &[tlas_build_info],
        &[&tlas_build_ranges],
    );
    let trace_barrier = vk::MemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_WRITE_KHR)
        .dst_access_mask(vk::AccessFlags::ACCELERATION_STRUCTURE_READ_KHR);
    device.cmd_pipeline_barrier(
        command_buffer,
        vk::PipelineStageFlags::ACCELERATION_STRUCTURE_BUILD_KHR,
        vk::PipelineStageFlags::RAY_TRACING_SHADER_KHR,
        vk::DependencyFlags::empty(),
        &[trace_barrier],
        &[],
        &[],
    );
    device.end_command_buffer(command_buffer)?;
    let command_buffers = [command_buffer];
    let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
    device.queue_submit(queue, &[submit], fence)?;
    device.wait_for_fences(&[fence], true, u64::MAX)?;
    device.reset_fences(&[fence])?;
    device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;

    Ok(SceneAccelerationStructures {
        vertex_buffer,
        index_buffer,
        instance_buffer,
        scratch_buffer,
        bottom_level,
        top_level,
    })
}

unsafe fn create_acceleration_structure(
    device: &Device,
    allocator: &mut Allocator,
    build_info: &vk::AccelerationStructureBuildGeometryInfoKHR<'_>,
    primitive_counts: &[u32],
    name: &'static str,
) -> Result<AccelerationStructureAllocation> {
    let mut sizes = vk::AccelerationStructureBuildSizesInfoKHR::default();
    device.get_acceleration_structure_build_sizes_khr(
        vk::AccelerationStructureBuildTypeKHR::ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR,
        build_info,
        Some(primitive_counts),
        &mut sizes,
    );
    let buffer = create_buffer_allocation(
        device,
        allocator,
        name,
        sizes.acceleration_structure_size,
        vk::BufferUsageFlags::ACCELERATION_STRUCTURE_STORAGE_KHR
            | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        MemoryLocation::GpuOnly,
        vk::MemoryAllocateFlags::DEVICE_ADDRESS,
    )?;
    let create_info = vk::AccelerationStructureCreateInfoKHR::default()
        .buffer(buffer.buffer)
        .size(sizes.acceleration_structure_size)
        .r#type(build_info.r#type);
    let handle = match device.create_acceleration_structure_khr(&create_info, None) {
        Ok(handle) => handle,
        Err(err) => {
            device.destroy_buffer(buffer.buffer, None);
            allocator.free(buffer.allocation)?;
            return Err(err.into());
        }
    };
    Ok(AccelerationStructureAllocation {
        handle,
        buffer,
        build_scratch_size: sizes.build_scratch_size,
    })
}

unsafe fn create_buffer_allocation(
    device: &Device,
    allocator: &mut Allocator,
    name: &'static str,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
    memory_allocate_flags: vk::MemoryAllocateFlags,
) -> Result<BufferAllocation> {
    let info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some(name),
        requirements,
        location,
        linear: true,
        memory_allocate_flags,
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }) {
        Ok(allocation) => allocation,
        Err(err) => {
            device.destroy_buffer(buffer, None);
            return Err(err.into());
        }
    };
    if let Err(err) = device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()) {
        let _ = allocator.free(allocation);
        device.destroy_buffer(buffer, None);
        return Err(err.into());
    }
    Ok(BufferAllocation { buffer, allocation })
}

unsafe fn create_gpu_buffer_from_data<T: bytemuck::Pod>(
    device: &Device,
    allocator: &mut Allocator,
    command_buffer: vk::CommandBuffer,
    queue: vk::Queue,
    fence: vk::Fence,
    usage: vk::BufferUsageFlags,
    values: &[T],
) -> Result<BufferAllocation> {
    let bytes: &[u8] = bytemuck::cast_slice(values);
    let buffer = create_buffer_allocation(
        device,
        allocator,
        "ray tracing GPU upload target",
        bytes.len() as u64,
        usage | vk::BufferUsageFlags::TRANSFER_DST,
        MemoryLocation::GpuOnly,
        vk::MemoryAllocateFlags::DEVICE_ADDRESS,
    )?;
    let staging = create_buffer_allocation(
        device,
        allocator,
        "ray tracing staging buffer",
        bytes.len() as u64,
        vk::BufferUsageFlags::TRANSFER_SRC,
        MemoryLocation::CpuToGpu,
        vk::MemoryAllocateFlags::empty(),
    )?;
    upload_pod_slice(allocator, &staging.allocation, values)?;
    copy_buffer_one_time(
        device,
        command_buffer,
        queue,
        fence,
        staging.buffer,
        buffer.buffer,
        bytes.len() as u64,
    )?;
    device.destroy_buffer(staging.buffer, None);
    allocator.free(staging.allocation)?;
    Ok(buffer)
}

unsafe fn copy_buffer_one_time(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    queue: vk::Queue,
    fence: vk::Fence,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: u64,
) -> Result<()> {
    device.reset_fences(&[fence])?;
    device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;
    let begin = vk::CommandBufferBeginInfo::default()
        .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    device.begin_command_buffer(command_buffer, &begin)?;
    let region = vk::BufferCopy::default().size(size);
    device.cmd_copy_buffer(command_buffer, src, dst, &[region]);
    device.end_command_buffer(command_buffer)?;
    let command_buffers = [command_buffer];
    let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
    device.queue_submit(queue, &[submit], fence)?;
    device.wait_for_fences(&[fence], true, u64::MAX)?;
    device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())?;
    device.reset_fences(&[fence])?;
    Ok(())
}

unsafe fn upload_pod_slice<T: bytemuck::Pod>(
    allocator: &mut Allocator,
    allocation: &Allocation,
    values: &[T],
) -> Result<()> {
    let flags = allocator.allocation_memory_flags(allocation)?;
    let mut mapped = allocator.map(allocation)?;
    let bytes = bytemuck::cast_slice(values);
    mapped.byte_range_mut(0, bytes.len()).copy_from_slice(bytes);
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.flush()?;
    }
    Ok(())
}

unsafe fn buffer_device_address(device: &Device, buffer: vk::Buffer) -> u64 {
    device.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer))
}

unsafe fn create_shader_binding_table_buffer(
    device: &Device,
    allocator: &mut Allocator,
    pipeline: vk::Pipeline,
    properties: &vk::PhysicalDeviceRayTracingPipelinePropertiesKHR<'_>,
) -> Result<BufferAllocation> {
    let handle_size = properties.shader_group_handle_size as usize;
    let record_stride = align_up(
        handle_size as u64,
        properties.shader_group_base_alignment as u64,
    );
    let group_count = 3_u32;
    let info = vk::BufferCreateInfo::default()
        .size(record_stride * group_count as u64)
        .usage(
            vk::BufferUsageFlags::SHADER_BINDING_TABLE_KHR
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        )
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let mut requirements = device.get_buffer_memory_requirements(buffer);
    requirements.alignment = requirements
        .alignment
        .max(properties.shader_group_base_alignment as u64);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("ray tracing shader binding table"),
        requirements,
        location: MemoryLocation::CpuToGpu,
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }) {
        Ok(allocation) => allocation,
        Err(err) => {
            device.destroy_buffer(buffer, None);
            return Err(err.into());
        }
    };
    if let Err(err) = device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()) {
        let _ = allocator.free(allocation);
        device.destroy_buffer(buffer, None);
        return Err(err.into());
    }
    let address =
        device.get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer));
    let base_alignment = properties.shader_group_base_alignment as u64;
    if !address.is_multiple_of(base_alignment) {
        let _ = allocator.free(allocation);
        device.destroy_buffer(buffer, None);
        return Err(anyhow!(
            "shader binding table address {address:#x} is not aligned to {base_alignment}"
        ));
    }
    let mut handles = vec![0_u8; handle_size * group_count as usize];
    device.get_ray_tracing_shader_group_handles_khr(pipeline, 0, group_count, &mut handles)?;
    let flags = allocator.allocation_memory_flags(&allocation)?;
    {
        let mut mapped = allocator.map(&allocation)?;
        let bytes = mapped.as_bytes_mut();
        bytes[..(record_stride * group_count as u64) as usize].fill(0);
        for group in 0..group_count as usize {
            let dst = group * record_stride as usize;
            let src = group * handle_size;
            bytes[dst..dst + handle_size].copy_from_slice(&handles[src..src + handle_size]);
        }
        if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
            mapped.flush()?;
        }
    }
    Ok(BufferAllocation { buffer, allocation })
}

unsafe fn create_output_buffer(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<BufferAllocation> {
    let info = vk::BufferCreateInfo::default()
        .size((WIDTH * HEIGHT * core::mem::size_of::<u32>() as u32) as u64)
        .usage(vk::BufferUsageFlags::STORAGE_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("ray tracing output"),
        requirements,
        location: MemoryLocation::GpuToCpu,
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }) {
        Ok(allocation) => allocation,
        Err(err) => {
            device.destroy_buffer(buffer, None);
            return Err(err.into());
        }
    };
    if let Err(err) = device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()) {
        let _ = allocator.free(allocation);
        device.destroy_buffer(buffer, None);
        return Err(err.into());
    }
    Ok(BufferAllocation { buffer, allocation })
}

unsafe fn create_descriptor_set_layout(device: &Device) -> Result<vk::DescriptorSetLayout> {
    let bindings = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::RAYGEN_KHR),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::RAYGEN_KHR),
    ];
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    Ok(device.create_descriptor_set_layout(&info, None)?)
}

unsafe fn create_descriptor_pool(device: &Device) -> Result<vk::DescriptorPool> {
    let pool_sizes = [
        vk::DescriptorPoolSize::default()
            .r#type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
            .descriptor_count(1),
        vk::DescriptorPoolSize::default()
            .r#type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1),
    ];
    let info = vk::DescriptorPoolCreateInfo::default()
        .max_sets(1)
        .pool_sizes(&pool_sizes);
    Ok(device.create_descriptor_pool(&info, None)?)
}

unsafe fn allocate_descriptor_set(
    device: &Device,
    pool: vk::DescriptorPool,
    layout: vk::DescriptorSetLayout,
) -> Result<vk::DescriptorSet> {
    let layouts = [layout];
    let info = vk::DescriptorSetAllocateInfo::default()
        .descriptor_pool(pool)
        .set_layouts(&layouts);
    device
        .allocate_descriptor_sets(&info)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkAllocateDescriptorSets returned no descriptor set"))
}

unsafe fn update_descriptor_set(
    device: &Device,
    descriptor_set: vk::DescriptorSet,
    acceleration_structure: vk::AccelerationStructureKHR,
    buffer: vk::Buffer,
) {
    let acceleration_structures = [acceleration_structure];
    let mut acceleration_structure_info = vk::WriteDescriptorSetAccelerationStructureKHR::default()
        .acceleration_structures(&acceleration_structures);
    let buffers = [vk::DescriptorBufferInfo::default()
        .buffer(buffer)
        .range((WIDTH * HEIGHT * core::mem::size_of::<u32>() as u32) as u64)];
    let writes = [
        vk::WriteDescriptorSet::default()
            .push_next(&mut acceleration_structure_info)
            .dst_set(descriptor_set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::ACCELERATION_STRUCTURE_KHR)
            .descriptor_count(1),
        vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffers),
    ];
    device.update_descriptor_sets(&writes, &[]);
}

unsafe fn create_pipeline_layout(
    device: &Device,
    descriptor_set_layout: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout> {
    let set_layouts = [descriptor_set_layout];
    let push_constant_ranges = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::RAYGEN_KHR)
        .offset(0)
        .size(core::mem::size_of::<RayTracingPushConstants>() as u32)];
    let info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&set_layouts)
        .push_constant_ranges(&push_constant_ranges);
    Ok(device.create_pipeline_layout(&info, None)?)
}

unsafe fn create_ray_tracing_pipeline(
    device: &Device,
    layout: vk::PipelineLayout,
) -> Result<vk::Pipeline> {
    let raygen = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/raygen.spv")),
    )?;
    let miss = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/miss.spv")),
    )?;
    let closest_hit = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/closesthit.spv")),
    )?;
    let raygen_main = c"raygenMain";
    let miss_main = c"missMain";
    let closest_hit_main = c"closestHitMain";
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::RAYGEN_KHR)
            .module(raygen)
            .name(raygen_main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::MISS_KHR)
            .module(miss)
            .name(miss_main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::CLOSEST_HIT_KHR)
            .module(closest_hit)
            .name(closest_hit_main),
    ];
    let groups = [
        vk::RayTracingShaderGroupCreateInfoKHR::default()
            .r#type(vk::RayTracingShaderGroupTypeKHR::RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR)
            .general_shader(0)
            .closest_hit_shader(vk::SHADER_UNUSED_KHR)
            .any_hit_shader(vk::SHADER_UNUSED_KHR)
            .intersection_shader(vk::SHADER_UNUSED_KHR),
        vk::RayTracingShaderGroupCreateInfoKHR::default()
            .r#type(vk::RayTracingShaderGroupTypeKHR::RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR)
            .general_shader(1)
            .closest_hit_shader(vk::SHADER_UNUSED_KHR)
            .any_hit_shader(vk::SHADER_UNUSED_KHR)
            .intersection_shader(vk::SHADER_UNUSED_KHR),
        vk::RayTracingShaderGroupCreateInfoKHR::default()
            .r#type(
                vk::RayTracingShaderGroupTypeKHR::RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR,
            )
            .general_shader(vk::SHADER_UNUSED_KHR)
            .closest_hit_shader(2)
            .any_hit_shader(vk::SHADER_UNUSED_KHR)
            .intersection_shader(vk::SHADER_UNUSED_KHR),
    ];
    let info = vk::RayTracingPipelineCreateInfoKHR::default()
        .stages(&stages)
        .groups(&groups)
        .max_pipeline_ray_recursion_depth(1)
        .layout(layout);
    let result = device.create_ray_tracing_pipelines_khr(
        vk::DeferredOperationKHR::null(),
        vk::PipelineCache::null(),
        &[info],
        None,
    );
    device.destroy_shader_module(raygen, None);
    device.destroy_shader_module(miss, None);
    device.destroy_shader_module(closest_hit, None);
    let pipelines = match result {
        Ok(pipelines) => pipelines,
        Err((pipelines, err)) => {
            for pipeline in pipelines {
                if !pipeline.is_null() {
                    device.destroy_pipeline(pipeline, None);
                }
            }
            return Err(anyhow!("vkCreateRayTracingPipelinesKHR failed: {err:?}"));
        }
    };
    pipelines
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkCreateRayTracingPipelinesKHR returned no pipeline"))
}

unsafe fn create_shader_module(device: &Device, bytes: &[u8]) -> Result<vk::ShaderModule> {
    let code = spirv_words(bytes)?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&code);
    Ok(device.create_shader_module(&create_info, None)?)
}

fn spirv_words(bytes: &[u8]) -> Result<Vec<u32>> {
    if !bytes.len().is_multiple_of(4) {
        return Err(anyhow!("SPIR-V byte length is not divisible by 4"));
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
        .collect())
}

unsafe fn create_command_pool(device: &Device, queue_family: u32) -> Result<vk::CommandPool> {
    let info = vk::CommandPoolCreateInfo::default()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(queue_family);
    Ok(device.create_command_pool(&info, None)?)
}

unsafe fn allocate_command_buffer(
    device: &Device,
    pool: vk::CommandPool,
) -> Result<vk::CommandBuffer> {
    let info = vk::CommandBufferAllocateInfo::default()
        .command_pool(pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    device
        .allocate_command_buffers(&info)?
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkAllocateCommandBuffers returned no command buffer"))
}

unsafe fn create_fence(device: &Device) -> Result<vk::Fence> {
    let info = vk::FenceCreateInfo::default();
    Ok(device.create_fence(&info, None)?)
}

unsafe fn initialize_output(allocator: &mut Allocator, allocation: &Allocation) -> Result<()> {
    let flags = allocator.allocation_memory_flags(allocation)?;
    let mut mapped = allocator.map(allocation)?;
    mapped.as_bytes_mut().fill(0);
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.flush()?;
    }
    Ok(())
}

unsafe fn verify_and_write_ppm(
    allocator: &mut Allocator,
    allocation: &Allocation,
    path: &str,
) -> Result<()> {
    let flags = allocator.allocation_memory_flags(allocation)?;
    let mut mapped = allocator.map(allocation)?;
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.invalidate()?;
    }
    let pixels =
        bytemuck::cast_slice::<u8, u32>(mapped.byte_range(0, (WIDTH * HEIGHT * 4) as usize));
    let hit_pixels = pixels
        .iter()
        .filter(|&&px| (px & 255) > 120 || ((px >> 8) & 255) > 120)
        .count();
    if hit_pixels < 500 {
        return Err(anyhow!(
            "ray tracing output has too few closest-hit pixels: {hit_pixels}"
        ));
    }

    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "P6\n{} {}\n255", WIDTH, HEIGHT)?;
    for px in pixels.iter().copied() {
        file.write_all(&[
            (px & 255) as u8,
            ((px >> 8) & 255) as u8,
            ((px >> 16) & 255) as u8,
        ])?;
    }
    Ok(())
}

fn align_up(value: u64, alignment: u64) -> u64 {
    if alignment == 0 {
        return value;
    }
    value.div_ceil(alignment) * alignment
}

fn has_extension(properties: &[vk::ExtensionProperties<'_>], name: &std::ffi::CStr) -> bool {
    properties.iter().any(|property| unsafe {
        std::ffi::CStr::from_ptr(property.extension_name.as_ptr()) == name
    })
}
