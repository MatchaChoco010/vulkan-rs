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

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct MeshVertex {
    position: [f32; 4],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct MeshDraw {
    vertex_offset: u32,
    index_offset: u32,
    triangle_count: u32,
    _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct PushConstants {
    clip_from_world: [[f32; 4]; 4],
}

const MESH_VERTICES: [MeshVertex; 3] = [
    MeshVertex {
        position: [-0.6, -0.6, 0.0, 1.0],
        color: [1.0, 0.0, 0.0, 1.0],
    },
    MeshVertex {
        position: [0.6, -0.6, 0.0, 1.0],
        color: [0.0, 1.0, 0.0, 1.0],
    },
    MeshVertex {
        position: [0.0, 0.6, 0.0, 1.0],
        color: [0.0, 0.2, 1.0, 1.0],
    },
];

const MESH_INDICES: [u32; 3] = [0, 1, 2];

const MESH_DRAWS: [MeshDraw; 1] = [MeshDraw {
    vertex_offset: 0,
    index_offset: 0,
    triangle_count: 1,
    _pad: 0,
}];

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
            println!("skipped: no device with task shader, mesh shader, and dynamic rendering");
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
        let color = create_color_image(&device, &mut allocator)?;
        let color_view = create_image_view(&device, color.image)?;
        let readback = create_readback_buffer(&device, &mut allocator)?;
        let vertex_buffer = create_storage_buffer(
            &device,
            &mut allocator,
            command_buffer,
            queue,
            fence,
            "mesh shader vertices",
            &MESH_VERTICES,
        )?;
        let index_buffer = create_storage_buffer(
            &device,
            &mut allocator,
            command_buffer,
            queue,
            fence,
            "mesh shader indices",
            &MESH_INDICES,
        )?;
        let draw_buffer = create_storage_buffer(
            &device,
            &mut allocator,
            command_buffer,
            queue,
            fence,
            "mesh shader draws",
            &MESH_DRAWS,
        )?;
        let descriptor_set_layout = create_descriptor_set_layout(&device)?;
        let descriptor_pool = create_descriptor_pool(&device)?;
        let descriptor_set =
            allocate_descriptor_set(&device, descriptor_pool, descriptor_set_layout)?;
        update_descriptor_set(
            &device,
            descriptor_set,
            &vertex_buffer,
            &index_buffer,
            &draw_buffer,
        );
        let pipeline_layout = create_pipeline_layout(&device, descriptor_set_layout)?;
        let pipeline = create_mesh_pipeline(&device, pipeline_layout)?;

        {
            let begin = vk::CommandBufferBeginInfo::default();
            device.begin_command_buffer(command_buffer, &begin)?;
            let subresource_range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            let to_color = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(color.image)
                .subresource_range(subresource_range)];
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &to_color,
            );
            let geometry_barriers = [
                vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::SHADER_READ)
                    .buffer(vertex_buffer.buffer)
                    .offset(0)
                    .size(vertex_buffer.size),
                vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::SHADER_READ)
                    .buffer(index_buffer.buffer)
                    .offset(0)
                    .size(index_buffer.size),
                vk::BufferMemoryBarrier::default()
                    .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                    .dst_access_mask(vk::AccessFlags::SHADER_READ)
                    .buffer(draw_buffer.buffer)
                    .offset(0)
                    .size(draw_buffer.size),
            ];
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::TASK_SHADER_EXT | vk::PipelineStageFlags::MESH_SHADER_EXT,
                vk::DependencyFlags::empty(),
                &[],
                &geometry_barriers,
                &[],
            );

            let clear = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            };
            let attachment = [vk::RenderingAttachmentInfo::default()
                .image_view(color_view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(clear)];
            let rendering = vk::RenderingInfo::default()
                .render_area(
                    vk::Rect2D::default()
                        .extent(vk::Extent2D::default().width(WIDTH).height(HEIGHT)),
                )
                .layer_count(1)
                .color_attachments(&attachment);
            device.cmd_begin_rendering(command_buffer, &rendering);
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                Some(&[descriptor_set]),
                &[],
            );
            let push_constants = PushConstants {
                clip_from_world: mesh_clip_from_world().to_cols_array_2d(),
            };
            device.cmd_push_constants(
                command_buffer,
                pipeline_layout,
                vk::ShaderStageFlags::MESH_EXT,
                0,
                bytemuck::bytes_of(&push_constants),
            );
            device.cmd_draw_mesh_tasks_ext(command_buffer, 1, 1, 1);
            device.cmd_end_rendering(command_buffer);

            let to_transfer = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::TRANSFER_READ)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(color.image)
                .subresource_range(subresource_range)];
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::TRANSFER,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &to_transfer,
            );
            let copy = [vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1))];
            device.cmd_copy_image_to_buffer(
                command_buffer,
                color.image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                readback.buffer,
                &copy,
            );
            let readback_barrier = vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .buffer(readback.buffer)
                .offset(0)
                .size((WIDTH * HEIGHT * 4) as u64);
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[readback_barrier],
                &[],
            );
            device.end_command_buffer(command_buffer)?;
        }
        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;
        verify_and_write_ppm(&mut allocator, &readback.allocation, "mesh_shader.ppm")?;

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_descriptor_pool(descriptor_pool, None);
        device.destroy_descriptor_set_layout(descriptor_set_layout, None);
        device.destroy_buffer(draw_buffer.buffer, None);
        allocator.free(draw_buffer.allocation)?;
        device.destroy_buffer(index_buffer.buffer, None);
        allocator.free(index_buffer.allocation)?;
        device.destroy_buffer(vertex_buffer.buffer, None);
        allocator.free(vertex_buffer.allocation)?;
        device.destroy_image_view(color_view, None);
        device.destroy_image(color.image, None);
        allocator.free(color.allocation)?;
        device.destroy_buffer(readback.buffer, None);
        allocator.free(readback.allocation)?;
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        drop(allocator);
        device.destroy_device(None);
        if !debug_messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
        }
        instance.destroy_instance(None);
    }

    println!("wrote mesh_shader.ppm");
    Ok(())
}

struct PhysicalDeviceSelection {
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
    size: u64,
}

struct ImageAllocation {
    image: vk::Image,
    allocation: Allocation,
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-mesh-shader")?;
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
        .api_version(vk::make_api_version(0, 1, 3, 0));
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
    let mut best = None;
    for physical_device in instance.enumerate_physical_devices()? {
        let extensions = instance.enumerate_device_extension_properties(physical_device, None)?;
        if !has_extension(&extensions, vk::EXT_MESH_SHADER_NAME) {
            continue;
        }

        let mut mesh_features = vk::PhysicalDeviceMeshShaderFeaturesEXT::default();
        let mut dynamic_rendering_features = vk::PhysicalDeviceDynamicRenderingFeatures::default();
        {
            let mut features2 = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut dynamic_rendering_features)
                .push_next(&mut mesh_features);
            instance.get_physical_device_features2(physical_device, &mut features2);
        }
        if mesh_features.task_shader == 0
            || mesh_features.mesh_shader == 0
            || dynamic_rendering_features.dynamic_rendering == 0
        {
            continue;
        }

        let queue_families =
            instance.get_physical_device_queue_family_properties(physical_device)?;
        let Some(queue_family) = queue_families
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
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
    let extension_names = [vk::EXT_MESH_SHADER_NAME.as_ptr()];
    let mut mesh_features = vk::PhysicalDeviceMeshShaderFeaturesEXT::default()
        .task_shader(true)
        .mesh_shader(true);
    let mut dynamic_rendering_features =
        vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&extension_names)
        .push_next(&mut mesh_features)
        .push_next(&mut dynamic_rendering_features);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

unsafe fn create_color_image(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<ImageAllocation> {
    let info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(FORMAT)
        .extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1))
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let image = device.create_image(&info, None)?;
    let requirements = device.get_image_memory_requirements(image);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("mesh shader color image"),
        requirements,
        location: MemoryLocation::GpuOnly,
        linear: false,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }) {
        Ok(allocation) => allocation,
        Err(err) => {
            device.destroy_image(image, None);
            return Err(err.into());
        }
    };
    if let Err(err) = device.bind_image_memory(image, allocation.memory(), allocation.offset()) {
        let _ = allocator.free(allocation);
        device.destroy_image(image, None);
        return Err(err.into());
    }
    Ok(ImageAllocation { image, allocation })
}

unsafe fn create_image_view(device: &Device, image: vk::Image) -> Result<vk::ImageView> {
    let info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::_2D)
        .format(FORMAT)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1),
        );
    Ok(device.create_image_view(&info, None)?)
}

unsafe fn create_readback_buffer(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<BufferAllocation> {
    let info = vk::BufferCreateInfo::default()
        .size((WIDTH * HEIGHT * 4) as u64)
        .usage(vk::BufferUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("mesh shader readback"),
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
    Ok(BufferAllocation {
        buffer,
        allocation,
        size: (WIDTH * HEIGHT * 4) as u64,
    })
}

unsafe fn create_storage_buffer<T: bytemuck::Pod>(
    device: &Device,
    allocator: &mut Allocator,
    command_buffer: vk::CommandBuffer,
    queue: vk::Queue,
    fence: vk::Fence,
    name: &'static str,
    values: &[T],
) -> Result<BufferAllocation> {
    let size = core::mem::size_of_val(values) as u64;
    let info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::STORAGE_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some(name),
        requirements,
        location: MemoryLocation::GpuOnly,
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

    let staging = create_staging_buffer(device, allocator, size)?;
    {
        let flags = allocator.allocation_memory_flags(&staging.allocation)?;
        let mut mapped = allocator.map(&staging.allocation)?;
        mapped
            .byte_range_mut(0, size as usize)
            .copy_from_slice(bytemuck::cast_slice(values));
        if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
            mapped.flush()?;
        }
    }
    copy_buffer_one_time(device, command_buffer, queue, fence, staging.buffer, buffer, size)?;
    device.destroy_buffer(staging.buffer, None);
    allocator.free(staging.allocation)?;

    Ok(BufferAllocation {
        buffer,
        allocation,
        size,
    })
}

unsafe fn create_staging_buffer(
    device: &Device,
    allocator: &mut Allocator,
    size: u64,
) -> Result<BufferAllocation> {
    let info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("mesh shader staging"),
        requirements,
        location: MemoryLocation::CpuToGpu,
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
    Ok(BufferAllocation {
        buffer,
        allocation,
        size,
    })
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

unsafe fn create_descriptor_set_layout(device: &Device) -> Result<vk::DescriptorSetLayout> {
    let bindings = [
        vk::DescriptorSetLayoutBinding::default()
            .binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::MESH_EXT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::MESH_EXT),
        vk::DescriptorSetLayoutBinding::default()
            .binding(2)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::TASK_EXT),
    ];
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    Ok(device.create_descriptor_set_layout(&info, None)?)
}

unsafe fn create_descriptor_pool(device: &Device) -> Result<vk::DescriptorPool> {
    let pool_sizes = [vk::DescriptorPoolSize::default()
        .r#type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(3)];
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
    vertex_buffer: &BufferAllocation,
    index_buffer: &BufferAllocation,
    draw_buffer: &BufferAllocation,
) {
    let buffers = [
        vk::DescriptorBufferInfo::default()
            .buffer(vertex_buffer.buffer)
            .range(vertex_buffer.size),
        vk::DescriptorBufferInfo::default()
            .buffer(index_buffer.buffer)
            .range(index_buffer.size),
        vk::DescriptorBufferInfo::default()
            .buffer(draw_buffer.buffer)
            .range(draw_buffer.size),
    ];
    let writes = [
        vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(0)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffers[0..1]),
        vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(1)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffers[1..2]),
        vk::WriteDescriptorSet::default()
            .dst_set(descriptor_set)
            .dst_binding(2)
            .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
            .buffer_info(&buffers[2..3]),
    ];
    device.update_descriptor_sets(&writes, &[]);
}

unsafe fn create_pipeline_layout(
    device: &Device,
    descriptor_set_layout: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout> {
    let layouts = [descriptor_set_layout];
    let push_constant_ranges = [vk::PushConstantRange::default()
        .stage_flags(vk::ShaderStageFlags::MESH_EXT)
        .offset(0)
        .size(core::mem::size_of::<PushConstants>() as u32)];
    let info = vk::PipelineLayoutCreateInfo::default()
        .set_layouts(&layouts)
        .push_constant_ranges(&push_constant_ranges);
    Ok(device.create_pipeline_layout(&info, None)?)
}

fn mesh_clip_from_world() -> Mat4 {
    let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 2.2), Vec3::ZERO, Vec3::Y);
    let projection = Mat4::perspective_rh(
        55.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        16.0,
    );
    projection * view
}

unsafe fn create_mesh_pipeline(
    device: &Device,
    layout: vk::PipelineLayout,
) -> Result<vk::Pipeline> {
    let task = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/task.spv")),
    )?;
    let mesh = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/mesh.spv")),
    )?;
    let fragment = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv")),
    )?;
    let main = c"main";
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::TASK_EXT)
            .module(task)
            .name(main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::MESH_EXT)
            .module(mesh)
            .name(main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::FRAGMENT)
            .module(fragment)
            .name(main),
    ];
    let viewports = [vk::Viewport::default()
        .y(HEIGHT as f32)
        .width(WIDTH as f32)
        .height(-(HEIGHT as f32))
        .min_depth(0.0)
        .max_depth(1.0)];
    let scissors = [vk::Rect2D::default().extent(vk::Extent2D::default().width(64).height(64))];
    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .viewports(&viewports)
        .scissors(&scissors);
    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);
    let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlagBits::_1);
    let color_attachment = [
        vk::PipelineColorBlendAttachmentState::default().color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        ),
    ];
    let color_blend_state =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&color_attachment);
    let formats = [vk::Format::R8G8B8A8_UNORM];
    let mut rendering_info =
        vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
    let info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&stages)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization_state)
        .multisample_state(&multisample_state)
        .color_blend_state(&color_blend_state)
        .layout(layout)
        .push_next(&mut rendering_info);
    let result = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
    device.destroy_shader_module(fragment, None);
    device.destroy_shader_module(mesh, None);
    device.destroy_shader_module(task, None);
    let pipelines = match result {
        Ok(pipelines) => pipelines,
        Err((pipelines, err)) => {
            for pipeline in pipelines {
                if !pipeline.is_null() {
                    device.destroy_pipeline(pipeline, None);
                }
            }
            return Err(anyhow!("vkCreateGraphicsPipelines failed: {err:?}"));
        }
    };
    pipelines
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkCreateGraphicsPipelines returned no mesh pipeline"))
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
    let bytes = mapped.as_bytes();
    let pixels = bytes.chunks_exact(4).collect::<Vec<_>>();
    let colored_pixels = pixels
        .iter()
        .filter(|px| px[0] > 32 || px[1] > 32 || px[2] > 32)
        .count();
    if colored_pixels < 128 {
        return Err(anyhow!(
            "mesh shader draw covered too few pixels: {colored_pixels}"
        ));
    }

    let center = pixels[(HEIGHT as usize / 2) * WIDTH as usize + WIDTH as usize / 2];
    if center[0] <= 16 && center[1] <= 16 && center[2] <= 16 {
        return Err(anyhow!("mesh shader draw left the image center black"));
    }

    for &(x, y) in &[
        (0_usize, 0_usize),
        (WIDTH as usize - 1, 0_usize),
        (0_usize, HEIGHT as usize - 1),
        (WIDTH as usize - 1, HEIGHT as usize - 1),
    ] {
        let px = pixels[y * WIDTH as usize + x];
        if px[0] > 16 || px[1] > 16 || px[2] > 16 {
            return Err(anyhow!(
                "mesh shader draw colored a clear corner pixel at ({x}, {y})"
            ));
        }
    }

    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "P6\n{} {}\n255", WIDTH, HEIGHT)?;
    for pixel in pixels {
        writer.write_all(&pixel[..3])?;
    }
    Ok(())
}

fn has_extension(properties: &[vk::ExtensionProperties<'_>], name: &std::ffi::CStr) -> bool {
    properties.iter().any(|property| unsafe {
        std::ffi::CStr::from_ptr(property.extension_name.as_ptr()) == name
    })
}
