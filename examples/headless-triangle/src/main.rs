#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::{Result, anyhow};
use std::ffi::{CStr, CString, c_void};
use std::fs::File;
use std::io::Write;
use std::mem;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const VERTICES: [Vertex; 3] = [
    Vertex {
        position: [0.0, -0.65],
        color: [1.0, 0.1, 0.1],
    },
    Vertex {
        position: [0.65, 0.65],
        color: [0.1, 1.0, 0.2],
    },
    Vertex {
        position: [-0.65, 0.65],
        color: [0.2, 0.3, 1.0],
    },
];
const INDICES: [u16; 3] = [0, 1, 2];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() -> Result<()> {
    unsafe {
        let entry = Entry::load()?;
        let mut instance_extensions = Vec::new();
        if has_instance_extension(&entry, vk::EXT_DEBUG_UTILS_NAME)? {
            instance_extensions.push(vk::EXT_DEBUG_UTILS_NAME);
        }
        let debug_utils_enabled = !instance_extensions.is_empty();
        let instance = create_instance(&entry, &instance_extensions)?;
        let debug_messenger = create_debug_messenger(&instance, debug_utils_enabled)?;
        let selection = pick_physical_device(&instance)?;
        let (device, queue) =
            create_device_and_queue(&instance, selection.physical_device, selection.queue_family)?;

        let physical_properties =
            instance.get_physical_device_properties(selection.physical_device);
        let memory_properties =
            instance.get_physical_device_memory_properties(selection.physical_device);
        let allocator_desc =
            AllocatorCreateDesc::new(&device, memory_properties, physical_properties);
        let mut allocator = Allocator::new(allocator_desc);

        let command_pool = create_command_pool(&device, selection.queue_family)?;
        let command_buffer = allocate_command_buffer(&device, command_pool)?;
        let fence = create_fence(&device)?;

        let vertex_buffer = create_uploaded_buffer(
            &device,
            &mut allocator,
            command_pool,
            queue,
            "triangle vertex buffer",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            bytemuck::cast_slice(&VERTICES),
        )?;
        let index_buffer = create_uploaded_buffer(
            &device,
            &mut allocator,
            command_pool,
            queue,
            "triangle index buffer",
            vk::BufferUsageFlags::INDEX_BUFFER,
            bytemuck::cast_slice(&INDICES),
        )?;

        let image_info = image_create_info(FORMAT, WIDTH, HEIGHT);
        let image =
            create_allocated_image(&device, &mut allocator, &image_info, MemoryLocation::GpuOnly)?;
        let image_view = create_image_view(&device, image.image, FORMAT)?;

        let readback_size = (WIDTH * HEIGHT * 4) as u64;
        let readback_info = buffer_create_info(readback_size);
        let readback_buffer = create_allocated_buffer(
            &device,
            &mut allocator,
            "readback buffer",
            &readback_info,
            MemoryLocation::GpuToCpu,
        )?;

        let render_pass = create_render_pass(&device, FORMAT)?;
        let framebuffer = create_framebuffer(&device, render_pass, image_view, WIDTH, HEIGHT)?;
        let pipeline_layout = create_pipeline_layout(&device)?;
        let pipeline = create_pipeline(&device, render_pass, pipeline_layout, FORMAT, WIDTH, HEIGHT)?;

        let begin = vk::CommandBufferBeginInfo::default();
        device.begin_command_buffer(command_buffer, &begin)?;

        let clear = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: [0.02, 0.02, 0.03, 1.0],
            },
        };
        let clear_values = [clear];
        let render_area = vk::Rect2D::default()
            .offset(vk::Offset2D::default())
            .extent(vk::Extent2D::default().width(WIDTH).height(HEIGHT));
        let render_begin = vk::RenderPassBeginInfo::default()
            .render_pass(render_pass)
            .framebuffer(framebuffer)
            .render_area(render_area)
            .clear_values(&clear_values);
        device.cmd_begin_render_pass(command_buffer, &render_begin, vk::SubpassContents::INLINE);
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        let vertex_buffers = [vertex_buffer.buffer];
        let vertex_offsets = [0];
        device.cmd_bind_vertex_buffers(command_buffer, 0, Some(&vertex_buffers), &vertex_offsets);
        device.cmd_bind_index_buffer(command_buffer, index_buffer.buffer, 0, vk::IndexType::UINT16);
        device.cmd_draw_indexed(command_buffer, INDICES.len() as u32, 1, 0, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        let copy = vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1),
            )
            .image_extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1));
        device.cmd_copy_image_to_buffer(
            command_buffer,
            image.image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback_buffer.buffer,
            &[copy],
        );
        let readback_barrier = vk::BufferMemoryBarrier::default()
            .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
            .dst_access_mask(vk::AccessFlags::HOST_READ)
            .buffer(readback_buffer.buffer)
            .offset(0)
            .size(readback_size);
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

        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;

        write_ppm(&mut allocator, &readback_buffer.allocation, WIDTH, HEIGHT, "triangle.ppm")?;
        let stats = allocator.stats();
        eprintln!(
            "allocator: {} blocks, {} allocated bytes, {} free bytes",
            stats.block_count, stats.allocated_bytes, stats.free_bytes
        );

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_framebuffer(framebuffer, None);
        device.destroy_render_pass(render_pass, None);
        device.destroy_image_view(image_view, None);
        device.destroy_buffer(readback_buffer.buffer, None);
        allocator.free(readback_buffer.allocation)?;
        device.destroy_buffer(index_buffer.buffer, None);
        allocator.free(index_buffer.allocation)?;
        device.destroy_buffer(vertex_buffer.buffer, None);
        allocator.free(vertex_buffer.allocation)?;
        device.destroy_image(image.image, None);
        allocator.free(image.allocation)?;
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        drop(allocator);
        device.destroy_device(None);
        if !debug_messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
        }
        instance.destroy_instance(None);
    }

    println!("wrote triangle.ppm");
    Ok(())
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-headless-triangle")?;
    let enabled_layers = validation_layers(entry)?;
    let enabled_layer_ptrs = enabled_layers
        .iter()
        .map(|layer| layer.as_ptr())
        .collect::<Vec<_>>();
    let extension_ptrs = extensions
        .iter()
        .map(|extension| extension.as_ptr())
        .collect::<Vec<_>>();
    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name.as_c_str())
        .engine_name(app_name.as_c_str())
        .api_version(vk::make_api_version(0, 1, 1, 0));
    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&app_info)
        .enabled_layer_names(&enabled_layer_ptrs)
        .enabled_extension_names(&extension_ptrs);
    Ok(entry.create_instance(&create_info, None)?)
}

unsafe fn has_instance_extension(entry: &Entry, name: &CStr) -> Result<bool> {
    Ok(entry
        .enumerate_instance_extension_properties(None)?
        .iter()
        .any(|extension| CStr::from_ptr(extension.extension_name.as_ptr()) == name))
}

unsafe fn validation_layers(entry: &Entry) -> Result<Vec<&'static CStr>> {
    let available = entry.enumerate_instance_layer_properties()?;
    let validation = c"VK_LAYER_KHRONOS_validation";
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

struct PhysicalDeviceSelection {
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
}

unsafe fn pick_physical_device(instance: &Instance) -> Result<PhysicalDeviceSelection> {
    let mut best = None;
    for physical_device in instance.enumerate_physical_devices()? {
        if !device_supports_shader_draw_parameters(instance, physical_device) {
            continue;
        }
        let Some(queue_family) = find_graphics_queue_family(instance, physical_device)? else {
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
                    queue_family,
                },
            ));
        }
    }
    best.map(|(_, selection)| selection)
        .ok_or_else(|| anyhow!("no physical device supports graphics and required features"))
}

unsafe fn device_supports_shader_draw_parameters(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> bool {
    let mut vulkan11 = vk::PhysicalDeviceVulkan11Features::default();
    {
        let mut features2 = vk::PhysicalDeviceFeatures2::default().push_next(&mut vulkan11);
        instance.get_physical_device_features2(physical_device, &mut features2);
    }
    vulkan11.shader_draw_parameters == vk::TRUE
}

unsafe fn find_graphics_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<Option<u32>> {
    let props = instance.get_physical_device_queue_family_properties(physical_device)?;
    Ok(props
        .iter()
        .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
        .map(|i| i as u32))
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
    let mut vulkan11_features =
        vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .push_next(&mut vulkan11_features);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
}

struct ImageAllocation {
    image: vk::Image,
    allocation: Allocation,
}

unsafe fn write_ppm(
    allocator: &mut Allocator,
    allocation: &Allocation,
    width: u32,
    height: u32,
    path: &str,
) -> Result<()> {
    let size = (width * height * 4) as usize;
    let flags = allocator.allocation_memory_flags(allocation)?;
    let mut mapped = allocator.map(allocation)?;
    if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
        mapped.invalidate()?;
    }
    let pixels = &mapped.as_bytes()[..size];
    let mut file = File::create(path)?;
    writeln!(file, "P6\n{} {}\n255", width, height)?;
    for px in pixels.chunks_exact(4) {
        file.write_all(&px[0..3])?;
    }
    Ok(())
}

unsafe fn create_allocated_buffer(
    device: &Device,
    allocator: &mut Allocator,
    name: &str,
    info: &vk::BufferCreateInfo<'_>,
    location: MemoryLocation,
) -> Result<BufferAllocation> {
    let buffer = device.create_buffer(info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some(name),
        requirements,
        location,
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

unsafe fn create_uploaded_buffer(
    device: &Device,
    allocator: &mut Allocator,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
    name: &str,
    usage: vk::BufferUsageFlags,
    bytes: &[u8],
) -> Result<BufferAllocation> {
    let size = bytes.len() as u64;
    let staging_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let staging = create_allocated_buffer(
        device,
        allocator,
        "staging buffer",
        &staging_info,
        MemoryLocation::CpuToGpu,
    )?;
    let flags = allocator.allocation_memory_flags(&staging.allocation)?;
    {
        let mut mapped = allocator.map(&staging.allocation)?;
        mapped.byte_range_mut(0, bytes.len()).copy_from_slice(bytes);
        if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
            mapped.flush()?;
        }
    }

    let dst_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(usage | vk::BufferUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let dst = create_allocated_buffer(device, allocator, name, &dst_info, MemoryLocation::GpuOnly)?;

    let command_buffers = [allocate_command_buffer(device, command_pool)?];
    let begin =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    device.begin_command_buffer(command_buffers[0], &begin)?;
    let copy = [vk::BufferCopy::default().size(size)];
    device.cmd_copy_buffer(command_buffers[0], staging.buffer, dst.buffer, &copy);
    let barrier = [vk::BufferMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(vk::AccessFlags::VERTEX_ATTRIBUTE_READ | vk::AccessFlags::INDEX_READ)
        .buffer(dst.buffer)
        .offset(0)
        .size(size)];
    device.cmd_pipeline_barrier(
        command_buffers[0],
        vk::PipelineStageFlags::TRANSFER,
        vk::PipelineStageFlags::VERTEX_INPUT,
        vk::DependencyFlags::empty(),
        &[],
        &barrier,
        &[],
    );
    device.end_command_buffer(command_buffers[0])?;
    let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
    device.queue_submit(queue, &[submit], vk::Fence::null())?;
    device.queue_wait_idle(queue)?;
    device.free_command_buffers(command_pool, &command_buffers);

    device.destroy_buffer(staging.buffer, None);
    allocator.free(staging.allocation)?;
    Ok(dst)
}

unsafe fn create_allocated_image(
    device: &Device,
    allocator: &mut Allocator,
    info: &vk::ImageCreateInfo<'_>,
    location: MemoryLocation,
) -> Result<ImageAllocation> {
    let image = device.create_image(info, None)?;
    let requirements = device.get_image_memory_requirements(image);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("offscreen color image"),
        requirements,
        location,
        linear: info.tiling == vk::ImageTiling::LINEAR,
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

fn image_create_info(format: vk::Format, width: u32, height: u32) -> vk::ImageCreateInfo<'static> {
    vk::ImageCreateInfo::default()
        .image_type(vk::ImageType::_2D)
        .format(format)
        .extent(vk::Extent3D::default().width(width).height(height).depth(1))
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlagBits::_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
}

unsafe fn create_image_view(
    device: &Device,
    image: vk::Image,
    format: vk::Format,
) -> Result<vk::ImageView> {
    let info = vk::ImageViewCreateInfo::default()
        .image(image)
        .view_type(vk::ImageViewType::_2D)
        .format(format)
        .subresource_range(
            vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1),
        );
    Ok(device.create_image_view(&info, None)?)
}

fn buffer_create_info(size: u64) -> vk::BufferCreateInfo<'static> {
    vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
}

unsafe fn create_render_pass(device: &Device, format: vk::Format) -> Result<vk::RenderPass> {
    let attachment = vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlagBits::_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL);
    let attachment_ref = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpass = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&attachment_ref)];
    let dependencies = [vk::SubpassDependency::default()
        .src_subpass(0)
        .dst_subpass(vk::SUBPASS_EXTERNAL)
        .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
        .dst_stage_mask(vk::PipelineStageFlags::TRANSFER)
        .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
        .dst_access_mask(vk::AccessFlags::TRANSFER_READ)];
    let attachments = [attachment];
    let info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(&subpass)
        .dependencies(&dependencies);
    Ok(device.create_render_pass(&info, None)?)
}

unsafe fn create_framebuffer(
    device: &Device,
    render_pass: vk::RenderPass,
    image_view: vk::ImageView,
    width: u32,
    height: u32,
) -> Result<vk::Framebuffer> {
    let attachments = [image_view];
    let info = vk::FramebufferCreateInfo::default()
        .render_pass(render_pass)
        .attachments(&attachments)
        .width(width)
        .height(height)
        .layers(1);
    Ok(device.create_framebuffer(&info, None)?)
}

unsafe fn create_pipeline_layout(device: &Device) -> Result<vk::PipelineLayout> {
    let info = vk::PipelineLayoutCreateInfo::default();
    Ok(device.create_pipeline_layout(&info, None)?)
}

unsafe fn create_pipeline(
    device: &Device,
    render_pass: vk::RenderPass,
    layout: vk::PipelineLayout,
    _format: vk::Format,
    width: u32,
    height: u32,
) -> Result<vk::Pipeline> {
    let vert = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/triangle.vert.spv")),
    )?;
    let frag = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/triangle.frag.spv")),
    )?;
    let main = c"main";
    let stages = [
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::VERTEX)
            .module(vert)
            .name(main),
        vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::FRAGMENT)
            .module(frag)
            .name(main),
    ];
    let vertex_bindings = [vk::VertexInputBindingDescription::default()
        .binding(0)
        .stride(mem::size_of::<Vertex>() as u32)
        .input_rate(vk::VertexInputRate::VERTEX)];
    let vertex_attributes = [
        vk::VertexInputAttributeDescription::default()
            .location(0)
            .binding(0)
            .format(vk::Format::R32G32_SFLOAT)
            .offset(0),
        vk::VertexInputAttributeDescription::default()
            .location(1)
            .binding(0)
            .format(vk::Format::R32G32B32_SFLOAT)
            .offset(mem::size_of::<[f32; 2]>() as u32),
    ];
    let vertex_input = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&vertex_bindings)
        .vertex_attribute_descriptions(&vertex_attributes);
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST);
    let viewport = [vk::Viewport::default()
        .width(width as f32)
        .height(height as f32)
        .max_depth(1.0)];
    let scissor = [vk::Rect2D::default()
        .offset(vk::Offset2D::default())
        .extent(vk::Extent2D::default().width(width).height(height))];
    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .viewports(&viewport)
        .scissors(&scissor);
    let raster = vk::PipelineRasterizationStateCreateInfo::default()
        .polygon_mode(vk::PolygonMode::FILL)
        .cull_mode(vk::CullModeFlags::empty())
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .line_width(1.0);
    let multisample = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(vk::SampleCountFlagBits::_1);
    let blend_attachment = [
        vk::PipelineColorBlendAttachmentState::default().color_write_mask(
            vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
        ),
    ];
    let color_blend =
        vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachment);
    let info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&raster)
        .multisample_state(&multisample)
        .color_blend_state(&color_blend)
        .layout(layout)
        .render_pass(render_pass);
    let result = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
    device.destroy_shader_module(vert, None);
    device.destroy_shader_module(frag, None);
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
        .ok_or_else(|| anyhow!("vkCreateGraphicsPipelines returned no pipeline"))
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
