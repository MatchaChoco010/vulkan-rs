#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::{Result, anyhow};
use glam::{Mat4, Vec3};
use std::ffi::{CStr, CString, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::ManuallyDrop;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const DEPTH_FORMAT: vk::Format = vk::Format::D32_SFLOAT;
const TEXTURE_SIZE: u32 = 64;
const TEXTURE_COUNT: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct InstanceData {
    translation_scale: [f32; 4],
    material_index: u32,
    _pad: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct MaterialData {
    base_color_texture: DescriptorHandle,
    _pad: [u32; 2],
    tint: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct PushConstants {
    clip_from_world: [[f32; 4]; 4],
    materials_handle: DescriptorHandle,
    sampler_handle: DescriptorHandle,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct DescriptorHandle {
    index: u32,
    reserved: u32,
}

impl DescriptorHandle {
    const fn new(index: u32) -> Self {
        Self { index, reserved: 0 }
    }
}

struct PhysicalDeviceSelection {
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    descriptor_heap_properties: vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>,
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
    size: u64,
}

struct ImageAllocation {
    image: vk::Image,
    view: vk::ImageView,
    allocation: Allocation,
}

struct ResourceHeapLayout {
    instances_index: u32,
    materials_index: u32,
    texture_indices: [u32; TEXTURE_COUNT],
    buffer_descriptor_size: u64,
    image_descriptor_size: u64,
    reserved_range_offset: u64,
    heap_size: u64,
}

struct SamplerHeapLayout {
    sampler_index: u32,
    sampler_descriptor_size: u64,
    reserved_range_offset: u64,
    heap_size: u64,
}

fn main() -> Result<()> {
    unsafe {
        let entry = Entry::load()?;
        let mut instance_extensions = Vec::new();
        if has_instance_extension(&entry, vk::EXT_DEBUG_UTILS_NAME)? {
            instance_extensions.push(vk::EXT_DEBUG_UTILS_NAME);
        }
        let instance = create_instance(&entry, &instance_extensions)?;
        let debug_messenger = create_debug_messenger(
            &instance,
            instance_extensions.contains(&vk::EXT_DEBUG_UTILS_NAME),
        )?;

        let Some(selection) = pick_physical_device(&instance)? else {
            println!("skipped: no Vulkan 1.4 graphics device with VK_EXT_descriptor_heap");
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
        let mut allocator = ManuallyDrop::new(Allocator::new(AllocatorCreateDesc::new(
            &device,
            memory_properties,
            physical_properties,
        )));

        let (vertices, indices) = sphere_mesh(24, 16);
        let instances = instance_data();
        let resource_layout = resource_heap_layout(&selection.descriptor_heap_properties);
        let sampler_layout = sampler_heap_layout(&selection.descriptor_heap_properties);

        let resource_heap = create_descriptor_heap_buffer(
            &device,
            &mut allocator,
            "descriptor heap resource heap",
            resource_layout.heap_size,
        )?;
        let sampler_heap = create_descriptor_heap_buffer(
            &device,
            &mut allocator,
            "descriptor heap sampler heap",
            sampler_layout.heap_size,
        )?;

        let vertex_buffer = create_gpu_buffer_from_data(
            &device,
            &mut allocator,
            selection.queue_family,
            queue,
            "sphere vertex buffer",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            &vertices,
        )?;
        let index_buffer = create_gpu_buffer_from_data(
            &device,
            &mut allocator,
            selection.queue_family,
            queue,
            "sphere index buffer",
            vk::BufferUsageFlags::INDEX_BUFFER,
            &indices,
        )?;
        let instance_buffer = create_gpu_buffer_from_data(
            &device,
            &mut allocator,
            selection.queue_family,
            queue,
            "sphere instance data",
            vk::BufferUsageFlags::VERTEX_BUFFER
                | vk::BufferUsageFlags::STORAGE_BUFFER
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            &instances,
        )?;
        let materials = material_data(&resource_layout);
        let material_buffer = create_gpu_buffer_from_data(
            &device,
            &mut allocator,
            selection.queue_family,
            queue,
            "bindless material data",
            vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            &materials,
        )?;

        let textures = [
            create_texture_image(
                &device,
                &mut allocator,
                selection.queue_family,
                queue,
                "warm checker texture",
                &texture0(),
            )?,
            create_texture_image(
                &device,
                &mut allocator,
                selection.queue_family,
                queue,
                "cool checker texture",
                &texture1(),
            )?,
        ];
        let color = create_color_target(&device, &mut allocator)?;
        let depth = create_depth_target(&device, &mut allocator)?;
        let readback = create_buffer(
            &device,
            &mut allocator,
            "descriptor heap readback image",
            (WIDTH * HEIGHT * 4) as u64,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
            vk::MemoryAllocateFlags::empty(),
        )?;

        write_resource_heap_descriptors(
            &device,
            &mut allocator,
            &resource_heap,
            &resource_layout,
            &instance_buffer,
            &material_buffer,
            &textures,
        )?;
        write_sampler_heap_descriptors(&device, &mut allocator, &sampler_heap, &sampler_layout)?;

        let command_pool = create_command_pool(&device, selection.queue_family)?;
        let command_buffer = allocate_command_buffer(&device, command_pool)?;
        let fence = create_fence(&device)?;
        let pipeline = create_graphics_pipeline(&device)?;

        let begin = vk::CommandBufferBeginInfo::default();
        device.begin_command_buffer(command_buffer, &begin)?;

        bind_descriptor_heaps(
            &device,
            command_buffer,
            &resource_heap,
            &resource_layout,
            &sampler_heap,
            &sampler_layout,
            &selection.descriptor_heap_properties,
        );
        barrier_descriptor_heaps(
            &device,
            command_buffer,
            &resource_heap,
            &resource_layout,
            &sampler_heap,
            &sampler_layout,
        );

        let color_to_attachment = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_NONE)
            .src_access_mask(vk::AccessFlags2::empty())
            .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags2::ACCESS_2_COLOR_ATTACHMENT_WRITE)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .image(color.image)
            .subresource_range(color_subresource_range());
        cmd_image_barrier(&device, command_buffer, color_to_attachment);

        let depth_to_attachment = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_NONE)
            .src_access_mask(vk::AccessFlags2::empty())
            .dst_stage_mask(
                vk::PipelineStageFlags2::PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS
                    | vk::PipelineStageFlags2::PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS,
            )
            .dst_access_mask(
                vk::AccessFlags2::ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ
                    | vk::AccessFlags2::ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE,
            )
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .image(depth.image)
            .subresource_range(depth_subresource_range());
        cmd_image_barrier(&device, command_buffer, depth_to_attachment);

        let clear = vk::ClearValue::color(vk::ClearColorValue::float32([0.02, 0.03, 0.05, 1.0]));
        let color_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(color.view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(clear);
        let depth_clear =
            vk::ClearValue::depth_stencil(vk::ClearDepthStencilValue::default().depth(1.0));
        let depth_attachment = vk::RenderingAttachmentInfo::default()
            .image_view(depth.view)
            .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .clear_value(depth_clear);
        let color_attachments = [color_attachment];
        let render_area = vk::Rect2D::default()
            .offset(vk::Offset2D::default())
            .extent(vk::Extent2D::default().width(WIDTH).height(HEIGHT));
        let rendering = vk::RenderingInfo::default()
            .render_area(render_area)
            .layer_count(1)
            .color_attachments(&color_attachments)
            .depth_attachment(&depth_attachment);
        device.cmd_begin_rendering(command_buffer, &rendering);

        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        let vertex_buffers = [vertex_buffer.buffer, instance_buffer.buffer];
        let vertex_offsets = [0_u64, 0_u64];
        device.cmd_bind_vertex_buffers(command_buffer, 0, Some(&vertex_buffers), &vertex_offsets);
        device.cmd_bind_index_buffer(
            command_buffer,
            index_buffer.buffer,
            0,
            vk::IndexType::UINT32,
        );
        let push = PushConstants {
            clip_from_world: clip_from_world().to_cols_array_2d(),
            materials_handle: DescriptorHandle::new(resource_layout.materials_index),
            sampler_handle: DescriptorHandle::new(sampler_layout.sampler_index),
        };
        let push_data = vk::PushDataInfoEXT::default()
            .data(vk::HostAddressRangeConstEXT::default().address(bytemuck::bytes_of(&push)));
        device.cmd_push_data_ext(command_buffer, &push_data);
        bind_descriptor_heaps(
            &device,
            command_buffer,
            &resource_heap,
            &resource_layout,
            &sampler_heap,
            &sampler_layout,
            &selection.descriptor_heap_properties,
        );
        device.cmd_draw_indexed(
            command_buffer,
            indices.len() as u32,
            instances.len() as u32,
            0,
            0,
            0,
        );
        device.cmd_end_rendering(command_buffer);

        let color_to_copy_src = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::ACCESS_2_COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY)
            .dst_access_mask(vk::AccessFlags2::ACCESS_2_TRANSFER_READ)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .image(color.image)
            .subresource_range(color_subresource_range());
        cmd_image_barrier(&device, command_buffer, color_to_copy_src);
        copy_image_to_buffer(&device, command_buffer, color.image, readback.buffer);

        let readback_barrier = vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY)
            .src_access_mask(vk::AccessFlags2::ACCESS_2_TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_HOST)
            .dst_access_mask(vk::AccessFlags2::ACCESS_2_HOST_READ)
            .buffer(readback.buffer)
            .offset(0)
            .size(readback.size);
        let readback_barriers = [readback_barrier];
        let readback_dependency =
            vk::DependencyInfo::default().buffer_memory_barriers(&readback_barriers);
        device.cmd_pipeline_barrier2(command_buffer, &readback_dependency);

        device.end_command_buffer(command_buffer)?;
        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;

        verify_and_write_ppm(
            &mut allocator,
            &readback.allocation,
            "descriptor_heap_spheres.ppm",
        )?;

        device.destroy_pipeline(pipeline, None);
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        destroy_buffer(&device, &mut allocator, readback)?;
        destroy_image(&device, &mut allocator, depth)?;
        destroy_image(&device, &mut allocator, color)?;
        for texture in textures {
            destroy_image(&device, &mut allocator, texture)?;
        }
        destroy_buffer(&device, &mut allocator, material_buffer)?;
        destroy_buffer(&device, &mut allocator, instance_buffer)?;
        destroy_buffer(&device, &mut allocator, index_buffer)?;
        destroy_buffer(&device, &mut allocator, vertex_buffer)?;
        destroy_buffer(&device, &mut allocator, sampler_heap)?;
        destroy_buffer(&device, &mut allocator, resource_heap)?;
        ManuallyDrop::drop(&mut allocator);
        device.destroy_device(None);
        if !debug_messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
        }
        instance.destroy_instance(None);
    }

    println!("wrote descriptor_heap_spheres.ppm");
    Ok(())
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-descriptor-heap-14")?;
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
        .api_version(vk::make_api_version(0, 1, 4, 0));
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
        let properties = instance.get_physical_device_properties(physical_device);
        if properties.api_version < vk::make_api_version(0, 1, 4, 0) {
            continue;
        }
        let extensions = instance.enumerate_device_extension_properties(physical_device, None)?;
        if !has_extension(&extensions, vk::EXT_DESCRIPTOR_HEAP_NAME)
            || !has_extension(&extensions, vk::KHR_SHADER_UNTYPED_POINTERS_NAME)
        {
            continue;
        }

        let mut vulkan11_features = vk::PhysicalDeviceVulkan11Features::default();
        let mut vulkan12_features = vk::PhysicalDeviceVulkan12Features::default();
        let mut vulkan13_features = vk::PhysicalDeviceVulkan13Features::default();
        let mut untyped_pointer_features =
            vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default();
        let mut descriptor_heap_features = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
        {
            let mut features2 = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut descriptor_heap_features)
                .push_next(&mut untyped_pointer_features)
                .push_next(&mut vulkan13_features)
                .push_next(&mut vulkan12_features)
                .push_next(&mut vulkan11_features);
            instance.get_physical_device_features2(physical_device, &mut features2);
        }
        if descriptor_heap_features.descriptor_heap == 0
            || untyped_pointer_features.shader_untyped_pointers == 0
            || vulkan11_features.shader_draw_parameters == 0
            || vulkan12_features.buffer_device_address == 0
            || vulkan13_features.synchronization2 == 0
            || vulkan13_features.dynamic_rendering == 0
        {
            continue;
        }

        let mut descriptor_heap_properties =
            vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        {
            let mut properties2 =
                vk::PhysicalDeviceProperties2::default().push_next(&mut descriptor_heap_properties);
            instance.get_physical_device_properties2(physical_device, &mut properties2);
        }

        let queue_families =
            instance.get_physical_device_queue_family_properties(physical_device)?;
        let Some(queue_family) = queue_families
            .iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
        else {
            continue;
        };
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
                    descriptor_heap_properties,
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
        vk::EXT_DESCRIPTOR_HEAP_NAME.as_ptr(),
        vk::KHR_SHADER_UNTYPED_POINTERS_NAME.as_ptr(),
    ];
    let mut vulkan11_features =
        vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(true);
    let mut descriptor_heap_features =
        vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
    let mut untyped_pointer_features =
        vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default().shader_untyped_pointers(true);
    let mut vulkan13_features = vk::PhysicalDeviceVulkan13Features::default()
        .synchronization2(true)
        .dynamic_rendering(true);
    let mut vulkan12_features =
        vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&extension_names)
        .push_next(&mut descriptor_heap_features)
        .push_next(&mut untyped_pointer_features)
        .push_next(&mut vulkan13_features)
        .push_next(&mut vulkan12_features)
        .push_next(&mut vulkan11_features);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

fn resource_heap_layout(
    props: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>,
) -> ResourceHeapLayout {
    let buffer_size = props.buffer_descriptor_size.max(1);
    let image_size = props.image_descriptor_size.max(1);
    let buffer_alignment = lcm(buffer_size, props.buffer_descriptor_alignment.max(1));
    let image_alignment = lcm(image_size, props.image_descriptor_alignment.max(1));

    let instances_offset = 0;
    let mut cursor = buffer_size;
    cursor = align_up(cursor, buffer_alignment);
    let materials_offset = cursor;
    cursor += buffer_size;

    cursor = align_up(cursor, image_alignment);
    let texture0_offset = cursor;
    cursor = align_up(texture0_offset + image_size, image_alignment);
    let texture1_offset = cursor;
    cursor += image_size;
    let reserved_range_offset = align_up(cursor, props.resource_heap_alignment.max(1));
    let heap_size = align_up(
        reserved_range_offset + props.min_resource_heap_reserved_range,
        props.resource_heap_alignment.max(1),
    );
    ResourceHeapLayout {
        instances_index: (instances_offset / buffer_size) as u32,
        materials_index: (materials_offset / buffer_size) as u32,
        texture_indices: [
            (texture0_offset / image_size) as u32,
            (texture1_offset / image_size) as u32,
        ],
        buffer_descriptor_size: buffer_size,
        image_descriptor_size: image_size,
        reserved_range_offset,
        heap_size,
    }
}

fn sampler_heap_layout(
    props: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>,
) -> SamplerHeapLayout {
    let sampler_size = props.sampler_descriptor_size.max(1);
    let reserved_range_offset = align_up(sampler_size, props.sampler_heap_alignment.max(1));
    let heap_size = align_up(
        reserved_range_offset + props.min_sampler_heap_reserved_range,
        props.sampler_heap_alignment.max(1),
    );
    SamplerHeapLayout {
        sampler_index: 0,
        sampler_descriptor_size: sampler_size,
        reserved_range_offset,
        heap_size,
    }
}

unsafe fn create_descriptor_heap_buffer(
    device: &Device,
    allocator: &mut Allocator,
    name: &'static str,
    size: u64,
) -> Result<BufferAllocation> {
    create_buffer(
        device,
        allocator,
        name,
        size,
        vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        MemoryLocation::CpuToGpu,
        vk::MemoryAllocateFlags::DEVICE_ADDRESS,
    )
}

unsafe fn create_buffer(
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
        allocation_scheme: if memory_allocate_flags.is_empty() {
            AllocationScheme::GpuAllocatorManaged
        } else {
            AllocationScheme::DedicatedBuffer(buffer)
        },
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

unsafe fn create_gpu_buffer_from_data<T: bytemuck::Pod>(
    device: &Device,
    allocator: &mut Allocator,
    queue_family: u32,
    queue: vk::Queue,
    name: &'static str,
    usage: vk::BufferUsageFlags,
    values: &[T],
) -> Result<BufferAllocation> {
    let size = core::mem::size_of_val(values) as u64;
    let buffer = create_buffer(
        device,
        allocator,
        name,
        size,
        usage | vk::BufferUsageFlags::TRANSFER_DST,
        MemoryLocation::GpuOnly,
        if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {
            vk::MemoryAllocateFlags::DEVICE_ADDRESS
        } else {
            vk::MemoryAllocateFlags::empty()
        },
    )?;
    let staging = create_buffer(
        device,
        allocator,
        "staging upload",
        size,
        vk::BufferUsageFlags::TRANSFER_SRC,
        MemoryLocation::CpuToGpu,
        vk::MemoryAllocateFlags::empty(),
    )?;
    write_allocation(allocator, &staging.allocation, values)?;
    upload_buffer_now(
        device,
        queue_family,
        queue,
        staging.buffer,
        buffer.buffer,
        size,
    )?;
    destroy_buffer(device, allocator, staging)?;
    Ok(buffer)
}

unsafe fn upload_buffer_now(
    device: &Device,
    queue_family: u32,
    queue: vk::Queue,
    src: vk::Buffer,
    dst: vk::Buffer,
    size: u64,
) -> Result<()> {
    let pool = create_command_pool(device, queue_family)?;
    let command_buffer = allocate_command_buffer(device, pool)?;
    let fence = create_fence(device)?;
    let begin = vk::CommandBufferBeginInfo::default();
    device.begin_command_buffer(command_buffer, &begin)?;
    let copy = vk::BufferCopy::default().size(size);
    device.cmd_copy_buffer(command_buffer, src, dst, &[copy]);
    let barrier = vk::BufferMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY)
        .src_access_mask(vk::AccessFlags2::ACCESS_2_TRANSFER_WRITE)
        .dst_stage_mask(
            vk::PipelineStageFlags2::PIPELINE_STAGE_2_VERTEX_INPUT
                | vk::PipelineStageFlags2::PIPELINE_STAGE_2_VERTEX_SHADER
                | vk::PipelineStageFlags2::PIPELINE_STAGE_2_FRAGMENT_SHADER,
        )
        .dst_access_mask(
            vk::AccessFlags2::ACCESS_2_VERTEX_ATTRIBUTE_READ
                | vk::AccessFlags2::ACCESS_2_INDEX_READ
                | vk::AccessFlags2::ACCESS_2_SHADER_STORAGE_READ,
        )
        .buffer(dst)
        .offset(0)
        .size(size);
    let barriers = [barrier];
    let dependency = vk::DependencyInfo::default().buffer_memory_barriers(&barriers);
    device.cmd_pipeline_barrier2(command_buffer, &dependency);
    device.end_command_buffer(command_buffer)?;
    let command_buffers = [command_buffer];
    let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
    device.queue_submit(queue, &[submit], fence)?;
    device.wait_for_fences(&[fence], true, u64::MAX)?;
    device.destroy_fence(fence, None);
    device.destroy_command_pool(pool, None);
    Ok(())
}

unsafe fn write_allocation<T: bytemuck::Pod>(
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

unsafe fn create_texture_image(
    device: &Device,
    allocator: &mut Allocator,
    queue_family: u32,
    queue: vk::Queue,
    name: &'static str,
    pixels: &[u32],
) -> Result<ImageAllocation> {
    let extent = vk::Extent3D::default()
        .width(TEXTURE_SIZE)
        .height(TEXTURE_SIZE)
        .depth(1);
    let image_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(FORMAT)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    let image = create_image(
        device,
        allocator,
        name,
        &image_info,
        vk::ImageAspectFlags::COLOR,
    )?;
    let staging = create_buffer(
        device,
        allocator,
        "texture staging upload",
        core::mem::size_of_val(pixels) as u64,
        vk::BufferUsageFlags::TRANSFER_SRC,
        MemoryLocation::CpuToGpu,
        vk::MemoryAllocateFlags::empty(),
    )?;
    write_allocation(allocator, &staging.allocation, pixels)?;
    upload_texture_now(device, queue_family, queue, staging.buffer, image.image)?;
    destroy_buffer(device, allocator, staging)?;
    Ok(image)
}

unsafe fn create_color_target(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<ImageAllocation> {
    let extent = vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1);
    let image_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(FORMAT)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    create_image(device, allocator, "descriptor heap color target", &image_info, vk::ImageAspectFlags::COLOR)
}

unsafe fn create_depth_target(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<ImageAllocation> {
    let extent = vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1);
    let image_info = vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(DEPTH_FORMAT)
        .extent(extent)
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);
    create_image(device, allocator, "descriptor heap depth target", &image_info, vk::ImageAspectFlags::DEPTH)
}

unsafe fn create_image(
    device: &Device,
    allocator: &mut Allocator,
    name: &'static str,
    info: &vk::ImageCreateInfo<'_>,
    aspect: vk::ImageAspectFlags,
) -> Result<ImageAllocation> {
    let image = device.create_image(info, None)?;
    let requirements = device.get_image_memory_requirements(image);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some(name),
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
    let range = image_subresource_range(aspect);
    let view_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::_2D)
        .format(info.format)
        .subresource_range(range);
    let view = device.create_image_view(&view_info, None)?;
    Ok(ImageAllocation {
        image,
        view,
        allocation,
    })
}

unsafe fn upload_texture_now(
    device: &Device,
    queue_family: u32,
    queue: vk::Queue,
    src: vk::Buffer,
    dst: vk::Image,
) -> Result<()> {
    let pool = create_command_pool(device, queue_family)?;
    let command_buffer = allocate_command_buffer(device, pool)?;
    let fence = create_fence(device)?;
    let begin = vk::CommandBufferBeginInfo::default();
    device.begin_command_buffer(command_buffer, &begin)?;
    let texture_to_copy_dst = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_NONE)
        .src_access_mask(vk::AccessFlags2::empty())
        .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY)
        .dst_access_mask(vk::AccessFlags2::ACCESS_2_TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .image(dst)
        .subresource_range(color_subresource_range());
    cmd_image_barrier(device, command_buffer, texture_to_copy_dst);
    copy_buffer_to_image(device, command_buffer, src, dst, TEXTURE_SIZE, TEXTURE_SIZE);
    let texture_to_shader_read = vk::ImageMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_COPY)
        .src_access_mask(vk::AccessFlags2::ACCESS_2_TRANSFER_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::ACCESS_2_SHADER_SAMPLED_READ)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
        .image(dst)
        .subresource_range(color_subresource_range());
    cmd_image_barrier(device, command_buffer, texture_to_shader_read);
    device.end_command_buffer(command_buffer)?;
    let command_buffers = [command_buffer];
    let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
    device.queue_submit(queue, &[submit], fence)?;
    device.wait_for_fences(&[fence], true, u64::MAX)?;
    device.destroy_fence(fence, None);
    device.destroy_command_pool(pool, None);
    Ok(())
}

unsafe fn write_resource_heap_descriptors(
    device: &Device,
    allocator: &mut Allocator,
    heap: &BufferAllocation,
    layout: &ResourceHeapLayout,
    instances: &BufferAllocation,
    materials: &BufferAllocation,
    textures: &[ImageAllocation; TEXTURE_COUNT],
) -> Result<()> {
    let instance_range = device_address_range(device, instances);
    let material_range = device_address_range(device, materials);
    let range_descriptors = [
        vk::ResourceDescriptorInfoEXT::default()
            .r#type(vk::DescriptorType::STORAGE_BUFFER)
            .data(vk::ResourceDescriptorDataEXT::address_range(
                &instance_range,
            )),
        vk::ResourceDescriptorInfoEXT::default()
            .r#type(vk::DescriptorType::STORAGE_BUFFER)
            .data(vk::ResourceDescriptorDataEXT::address_range(
                &material_range,
            )),
    ];
    let texture_view_infos = [
        vk::ImageViewCreateInfo::default()
            .image(textures[0].image)
            .view_type(vk::ImageViewType::_2D)
            .format(FORMAT)
            .subresource_range(color_subresource_range()),
        vk::ImageViewCreateInfo::default()
            .image(textures[1].image)
            .view_type(vk::ImageViewType::_2D)
            .format(FORMAT)
            .subresource_range(color_subresource_range()),
    ];
    let image_infos = [
        vk::ImageDescriptorInfoEXT::default()
            .view(&texture_view_infos[0])
            .layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
        vk::ImageDescriptorInfoEXT::default()
            .view(&texture_view_infos[1])
            .layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL),
    ];
    let image_descriptors = [
        vk::ResourceDescriptorInfoEXT::default()
            .r#type(vk::DescriptorType::SAMPLED_IMAGE)
            .data(vk::ResourceDescriptorDataEXT::image(&image_infos[0])),
        vk::ResourceDescriptorInfoEXT::default()
            .r#type(vk::DescriptorType::SAMPLED_IMAGE)
            .data(vk::ResourceDescriptorDataEXT::image(&image_infos[1])),
    ];

    let flags = allocator.allocation_memory_flags(&heap.allocation)?;
    let mut mapped = allocator.map(&heap.allocation)?;
    for (index, descriptor) in [
        layout.instances_index as usize,
        layout.materials_index as usize,
    ]
    .into_iter()
    .zip(range_descriptors.iter())
    {
        let offset = index * layout.buffer_descriptor_size as usize;
        let dst = mapped.byte_range_mut(offset, layout.buffer_descriptor_size as usize);
        let output = [vk::HostAddressRangeEXT::default().address(dst)];
        device.write_resource_descriptors_ext(core::slice::from_ref(descriptor), &output)?;
    }
    for (i, descriptor) in image_descriptors.iter().enumerate() {
        let offset = layout.texture_indices[i] as usize * layout.image_descriptor_size as usize;
        let dst = mapped.byte_range_mut(offset, layout.image_descriptor_size as usize);
        let output = [vk::HostAddressRangeEXT::default().address(dst)];
        device.write_resource_descriptors_ext(core::slice::from_ref(descriptor), &output)?;
    }
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.flush()?;
    }
    Ok(())
}

unsafe fn write_sampler_heap_descriptors(
    device: &Device,
    allocator: &mut Allocator,
    heap: &BufferAllocation,
    layout: &SamplerHeapLayout,
) -> Result<()> {
    let sampler = vk::SamplerCreateInfo::default()
        .mag_filter(vk::Filter::LINEAR)
        .min_filter(vk::Filter::LINEAR)
        .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
        .address_mode_u(vk::SamplerAddressMode::REPEAT)
        .address_mode_v(vk::SamplerAddressMode::REPEAT)
        .address_mode_w(vk::SamplerAddressMode::REPEAT)
        .min_lod(0.0)
        .max_lod(0.0);
    let flags = allocator.allocation_memory_flags(&heap.allocation)?;
    let mut mapped = allocator.map(&heap.allocation)?;
    let size = layout.sampler_descriptor_size as usize;
    let dst = mapped.byte_range_mut(layout.sampler_index as usize * size, size);
    let output = [vk::HostAddressRangeEXT::default().address(dst)];
    device.write_sampler_descriptors_ext(core::slice::from_ref(&sampler), &output)?;
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.flush()?;
    }
    Ok(())
}

unsafe fn device_address_range(
    device: &Device,
    buffer: &BufferAllocation,
) -> vk::DeviceAddressRangeKHR<'static> {
    let address = device
        .get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer.buffer));
    vk::DeviceAddressRangeKHR::default()
        .address(address)
        .size(buffer.size)
}

unsafe fn bind_descriptor_heaps(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    resource_heap: &BufferAllocation,
    resource_layout: &ResourceHeapLayout,
    sampler_heap: &BufferAllocation,
    sampler_layout: &SamplerHeapLayout,
    props: &vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'_>,
) {
    let resource_address = device.get_buffer_device_address(
        &vk::BufferDeviceAddressInfo::default().buffer(resource_heap.buffer),
    );
    let resource_range = vk::DeviceAddressRangeKHR::default()
        .address(resource_address)
        .size(resource_layout.heap_size);
    let resource_bind = vk::BindHeapInfoEXT::default()
        .heap_range(resource_range)
        .reserved_range_offset(resource_layout.reserved_range_offset)
        .reserved_range_size(props.min_resource_heap_reserved_range);
    device.cmd_bind_resource_heap_ext(command_buffer, &resource_bind);

    let sampler_address = device.get_buffer_device_address(
        &vk::BufferDeviceAddressInfo::default().buffer(sampler_heap.buffer),
    );
    let sampler_range = vk::DeviceAddressRangeKHR::default()
        .address(sampler_address)
        .size(sampler_layout.heap_size);
    let sampler_bind = vk::BindHeapInfoEXT::default()
        .heap_range(sampler_range)
        .reserved_range_offset(sampler_layout.reserved_range_offset)
        .reserved_range_size(props.min_sampler_heap_reserved_range);
    device.cmd_bind_sampler_heap_ext(command_buffer, &sampler_bind);
}

unsafe fn barrier_descriptor_heaps(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    resource_heap: &BufferAllocation,
    resource_layout: &ResourceHeapLayout,
    sampler_heap: &BufferAllocation,
    sampler_layout: &SamplerHeapLayout,
) {
    let resource_barrier = vk::BufferMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_HOST)
        .src_access_mask(vk::AccessFlags2::ACCESS_2_HOST_WRITE)
        .dst_stage_mask(
            vk::PipelineStageFlags2::PIPELINE_STAGE_2_VERTEX_SHADER
                | vk::PipelineStageFlags2::PIPELINE_STAGE_2_FRAGMENT_SHADER,
        )
        .dst_access_mask(vk::AccessFlags2::ACCESS_2_RESOURCE_HEAP_READ_EXT)
        .buffer(resource_heap.buffer)
        .offset(0)
        .size(resource_layout.heap_size);
    let sampler_barrier = vk::BufferMemoryBarrier2::default()
        .src_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_HOST)
        .src_access_mask(vk::AccessFlags2::ACCESS_2_HOST_WRITE)
        .dst_stage_mask(vk::PipelineStageFlags2::PIPELINE_STAGE_2_FRAGMENT_SHADER)
        .dst_access_mask(vk::AccessFlags2::ACCESS_2_SAMPLER_HEAP_READ_EXT)
        .buffer(sampler_heap.buffer)
        .offset(0)
        .size(sampler_layout.heap_size);
    let barriers = [resource_barrier, sampler_barrier];
    let dependency = vk::DependencyInfo::default().buffer_memory_barriers(&barriers);
    device.cmd_pipeline_barrier2(command_buffer, &dependency);
}

unsafe fn create_graphics_pipeline(device: &Device) -> Result<vk::Pipeline> {
    let vertex_shader = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/vertex.spv")),
    )?;
    let fragment_shader = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/fragment.spv")),
    )?;
    let main = c"main";
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::VERTEX)
            .module(vertex_shader)
            .name(main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::FRAGMENT)
            .module(fragment_shader)
            .name(main),
    ];
    let binding = [vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(core::mem::size_of::<Vertex>() as u32)
        .input_rate(vk::VertexInputRate::VERTEX),
        vk::VertexInputBindingDescription::default()
            .binding(1)
            .stride(core::mem::size_of::<InstanceData>() as u32)
            .input_rate(vk::VertexInputRate::INSTANCE)];
    let attributes = [
        vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(0),
        vk::VertexInputAttributeDescription::default()
            .location(1)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(12),
        vk::VertexInputAttributeDescription::default()
            .location(2)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(24),
        vk::VertexInputAttributeDescription::default()
            .location(3)
            .binding(1)
            .format(vk::Format::R32G32B32A32_SFLOAT)
            .offset(0),
        vk::VertexInputAttributeDescription::default()
            .location(4)
            .binding(1)
            .format(vk::Format::R32_UINT)
            .offset(16),
    ];
    let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&binding)
        .vertex_attribute_descriptions(&attributes);
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let viewport = [vk::Viewport::default()
        .y(HEIGHT as f32)
        .width(WIDTH as f32)
        .height(-(HEIGHT as f32))
        .min_depth(0.0)
        .max_depth(1.0)];
    let scissor = [vk::Rect2D::default()
        .offset(vk::Offset2D::default())
        .extent(vk::Extent2D::default().width(WIDTH).height(HEIGHT))];
    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .viewports(&viewport)
        .scissors(&scissor);
    let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::NONE)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);
    let multisample = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlagBits::_1);
    let depth_stencil = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(true)
        .depth_write_enable(true)
        .depth_compare_op(vk::CompareOp::LESS);
    let color_blend_attachment = [vk::PipelineColorBlendAttachmentState::default()
        .color_write_mask(
            vk::ColorComponentFlagBits::R
                | vk::ColorComponentFlagBits::G
                | vk::ColorComponentFlagBits::B
                | vk::ColorComponentFlagBits::A,
        )];
    let color_blend =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&color_blend_attachment);
    let color_formats = [FORMAT];
    let mut rendering = vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(&color_formats)
        .depth_attachment_format(DEPTH_FORMAT);
    let mut flags2 = vk::PipelineCreateFlags2CreateInfo::default()
        .flags(vk::PipelineCreateFlags2::PIPELINE_CREATE_2_DESCRIPTOR_HEAP_EXT);
    let info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization)
        .multisample_state(&multisample)
        .depth_stencil_state(&depth_stencil)
        .color_blend_state(&color_blend)
        .push_next(&mut flags2)
        .push_next(&mut rendering);
    let result = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
    device.destroy_shader_module(fragment_shader, None);
    device.destroy_shader_module(vertex_shader, None);
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
    let pipeline = pipelines
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkCreateGraphicsPipelines returned no pipeline"))?;
    Ok(pipeline)
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

unsafe fn cmd_image_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    barrier: vk::ImageMemoryBarrier2<'_>,
) {
    let barriers = [barrier];
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    device.cmd_pipeline_barrier2(command_buffer, &dependency);
}

unsafe fn copy_buffer_to_image(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    src: vk::Buffer,
    dst: vk::Image,
    width: u32,
    height: u32,
) {
    let subresource = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1);
    let region = vk::BufferImageCopy2::default()
        .image_subresource(subresource)
        .image_extent(vk::Extent3D::default().width(width).height(height).depth(1));
    let regions = [region];
    let copy = vk::CopyBufferToImageInfo2::default()
        .src_buffer(src)
        .dst_image(dst)
        .dst_image_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .regions(&regions);
    device.cmd_copy_buffer_to_image2(command_buffer, &copy);
}

unsafe fn copy_image_to_buffer(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    src: vk::Image,
    dst: vk::Buffer,
) {
    let subresource = vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .mip_level(0)
        .base_array_layer(0)
        .layer_count(1);
    let region = vk::BufferImageCopy2::default()
        .image_subresource(subresource)
        .image_extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1));
    let regions = [region];
    let copy = vk::CopyImageToBufferInfo2::default()
        .src_image(src)
        .src_image_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
        .dst_buffer(dst)
        .regions(&regions);
    device.cmd_copy_image_to_buffer2(command_buffer, &copy);
}

fn color_subresource_range() -> vk::ImageSubresourceRange<'static> {
    image_subresource_range(vk::ImageAspectFlags::COLOR)
}

fn depth_subresource_range() -> vk::ImageSubresourceRange<'static> {
    image_subresource_range(vk::ImageAspectFlags::DEPTH)
}

fn image_subresource_range(aspect: vk::ImageAspectFlags) -> vk::ImageSubresourceRange<'static> {
    vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(1)
}

unsafe fn create_command_pool(device: &Device, queue_family: u32) -> Result<vk::CommandPool> {
    let info = vk::CommandPoolCreateInfo::default().queue_family_index(queue_family);
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
    Ok(device.create_fence(&vk::FenceCreateInfo::default(), None)?)
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
    let mut slice_bright = [0_usize; 4];
    let bright_pixels = pixels
        .iter()
        .enumerate()
        .filter(|&(index, &px)| {
            let r = px & 255;
            let g = (px >> 8) & 255;
            let b = (px >> 16) & 255;
            let bright = r > 80 || g > 80 || b > 120;
            if bright {
                let x = (index as u32) % WIDTH;
                slice_bright[(x * 4 / WIDTH) as usize] += 1;
            }
            bright
        })
        .count();
    if bright_pixels < 1_500 {
        return Err(anyhow!(
            "descriptor heap draw produced too few visible textured pixels: {bright_pixels}"
        ));
    }
    if slice_bright.iter().any(|&count| count < 1_000) {
        return Err(anyhow!(
            "descriptor heap draw did not cover all four sphere regions: {slice_bright:?}"
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

unsafe fn destroy_buffer(
    device: &Device,
    allocator: &mut Allocator,
    buffer: BufferAllocation,
) -> Result<()> {
    device.destroy_buffer(buffer.buffer, None);
    allocator.free(buffer.allocation)?;
    Ok(())
}

unsafe fn destroy_image(
    device: &Device,
    allocator: &mut Allocator,
    image: ImageAllocation,
) -> Result<()> {
    device.destroy_image_view(image.view, None);
    device.destroy_image(image.image, None);
    allocator.free(image.allocation)?;
    Ok(())
}

fn sphere_mesh(segments: u32, rings: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    for y in 0..=rings {
        let v = y as f32 / rings as f32;
        let theta = v * core::f32::consts::PI;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        for x in 0..=segments {
            let u = x as f32 / segments as f32;
            let phi = u * core::f32::consts::TAU;
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
    (vertices, indices)
}

fn instance_data() -> [InstanceData; 4] {
    [
        InstanceData {
            translation_scale: [-1.35, 0.18, 4.0, 0.42],
            material_index: 0,
            _pad: [0; 3],
        },
        InstanceData {
            translation_scale: [-0.45, -0.16, 3.65, 0.48],
            material_index: 1,
            _pad: [0; 3],
        },
        InstanceData {
            translation_scale: [0.48, 0.15, 3.85, 0.45],
            material_index: 0,
            _pad: [0; 3],
        },
        InstanceData {
            translation_scale: [1.35, -0.12, 4.1, 0.44],
            material_index: 1,
            _pad: [0; 3],
        },
    ]
}

fn material_data(layout: &ResourceHeapLayout) -> [MaterialData; 2] {
    [
        MaterialData {
            base_color_texture: DescriptorHandle::new(layout.texture_indices[0]),
            _pad: [0; 2],
            tint: [1.0, 0.92, 0.86, 1.0],
        },
        MaterialData {
            base_color_texture: DescriptorHandle::new(layout.texture_indices[1]),
            _pad: [0; 2],
            tint: [0.82, 0.95, 1.0, 1.0],
        },
    ]
}

fn clip_from_world() -> Mat4 {
    let view = Mat4::look_at_rh(Vec3::ZERO, Vec3::Z, Vec3::Y);
    let projection = Mat4::perspective_rh(
        48.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        16.0,
    );
    projection * view
}

fn texture0() -> Vec<u32> {
    checker_texture(rgba8(255, 208, 102), rgba8(230, 74, 46))
}

fn texture1() -> Vec<u32> {
    checker_texture(rgba8(142, 230, 197), rgba8(63, 125, 217))
}

fn rgba8(r: u8, g: u8, b: u8) -> u32 {
    u32::from(r) | (u32::from(g) << 8) | (u32::from(b) << 16) | 0xff00_0000
}

fn checker_texture(a: u32, b: u32) -> Vec<u32> {
    let mut values = vec![0_u32; (TEXTURE_SIZE * TEXTURE_SIZE) as usize];
    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let checker = ((x / 8) + (y / 8)) % 2 == 0;
            values[(y * TEXTURE_SIZE + x) as usize] = if checker { a } else { b };
        }
    }
    values
}

fn has_extension(properties: &[vk::ExtensionProperties<'_>], name: &CStr) -> bool {
    properties
        .iter()
        .any(|property| unsafe { CStr::from_ptr(property.extension_name.as_ptr()) == name })
}

fn align_up(value: u64, alignment: u64) -> u64 {
    if alignment <= 1 {
        value
    } else {
        value.div_ceil(alignment) * alignment
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        1
    } else {
        a / gcd(a, b) * b
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let next = a % b;
        a = b;
        b = next;
    }
    a
}
