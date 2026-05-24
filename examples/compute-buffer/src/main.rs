#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::{anyhow, Result};
use std::ffi::{CStr, CString, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use vulkan::{vk, Device, Entry, Instance};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const ELEMENTS: usize = 64;

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
        let selection = pick_physical_device(&instance)?;
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
        let buffer = create_storage_buffer(&device, &mut allocator)?;
        initialize_buffer(&mut allocator, &buffer.allocation)?;
        let descriptor_set_layout = create_descriptor_set_layout(&device)?;
        let descriptor_pool = create_descriptor_pool(&device)?;
        let descriptor_set =
            allocate_descriptor_set(&device, descriptor_pool, descriptor_set_layout)?;
        update_descriptor_set(&device, descriptor_set, buffer.buffer);
        let pipeline_layout = create_pipeline_layout(&device, descriptor_set_layout)?;
        let pipeline = create_compute_pipeline(&device, pipeline_layout)?;

        {
            let begin = vk::CommandBufferBeginInfo::default();
            device.begin_command_buffer(command_buffer, &begin)?;
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
            let descriptor_sets = [descriptor_set];
            device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::COMPUTE,
                pipeline_layout,
                0,
                Some(&descriptor_sets),
                &[],
            );
            device.cmd_dispatch(command_buffer, 1, 1, 1);
            let barrier = vk::BufferMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::SHADER_WRITE)
                .dst_access_mask(vk::AccessFlags::HOST_READ)
                .buffer(buffer.buffer)
                .offset(0)
                .size(buffer.allocation.size());
            device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::COMPUTE_SHADER,
                vk::PipelineStageFlags::HOST,
                vk::DependencyFlags::empty(),
                &[],
                &[barrier],
                &[],
            );
            device.end_command_buffer(command_buffer)?;
        }
        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;

        verify_and_write_ppm(&mut allocator, &buffer.allocation, "compute_buffer.ppm")?;

        device.destroy_pipeline(pipeline, None);
        device.destroy_pipeline_layout(pipeline_layout, None);
        device.destroy_descriptor_pool(descriptor_pool, None);
        device.destroy_descriptor_set_layout(descriptor_set_layout, None);
        device.destroy_fence(fence, None);
        device.destroy_command_pool(command_pool, None);
        device.destroy_buffer(buffer.buffer, None);
        allocator.free(buffer.allocation)?;
        drop(allocator);
        device.destroy_device(None);
        if !debug_messenger.is_null() {
            instance.destroy_debug_utils_messenger_ext(debug_messenger, None);
        }
        instance.destroy_instance(None);
    }

    println!("wrote compute_buffer.ppm");
    Ok(())
}

struct PhysicalDeviceSelection {
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-compute-buffer")?;
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
        .api_version(vk::make_api_version(0, 1, 1, 0));
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

unsafe fn pick_physical_device(instance: &Instance) -> Result<PhysicalDeviceSelection> {
    let mut best = None;
    for physical_device in instance.enumerate_physical_devices()? {
        let props = instance.get_physical_device_queue_family_properties(physical_device)?;
        let Some(index) = props
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
                    queue_family: index as u32,
                },
            ));
        }
    }
    best.map(|(_, selection)| selection)
        .ok_or_else(|| anyhow!("no physical device with a compute queue"))
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
    let create_info = vk::DeviceCreateInfo::default().queue_create_infos(&queue_info);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

unsafe fn create_storage_buffer(
    device: &Device,
    allocator: &mut Allocator,
) -> Result<BufferAllocation> {
    let size = (ELEMENTS * core::mem::size_of::<u32>()) as u64;
    let info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("compute output"),
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
    let bindings = [vk::DescriptorSetLayoutBinding::default()
        .binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::COMPUTE)];
    let info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
    Ok(device.create_descriptor_set_layout(&info, None)?)
}

unsafe fn create_descriptor_pool(device: &Device) -> Result<vk::DescriptorPool> {
    let pool_sizes = [vk::DescriptorPoolSize::default()
        .r#type(vk::DescriptorType::STORAGE_BUFFER)
        .descriptor_count(1)];
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
    buffer: vk::Buffer,
) {
    let buffers = [vk::DescriptorBufferInfo::default()
        .buffer(buffer)
        .range((ELEMENTS * core::mem::size_of::<u32>()) as u64)];
    let writes = [vk::WriteDescriptorSet::default()
        .dst_set(descriptor_set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&buffers)];
    device.update_descriptor_sets(&writes, &[]);
}

unsafe fn create_pipeline_layout(
    device: &Device,
    descriptor_set_layout: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout> {
    let set_layouts = [descriptor_set_layout];
    let info = vk::PipelineLayoutCreateInfo::default().set_layouts(&set_layouts);
    Ok(device.create_pipeline_layout(&info, None)?)
}

unsafe fn create_compute_pipeline(
    device: &Device,
    layout: vk::PipelineLayout,
) -> Result<vk::Pipeline> {
    let shader = create_shader_module(
        device,
        include_bytes!(concat!(env!("OUT_DIR"), "/compute.spv")),
    )?;
    let main = c"main";
    let stage = vk::PipelineShaderStageCreateInfo::default()
        .stage(vk::ShaderStageFlagBits::COMPUTE)
        .module(shader)
        .name(main);
    let info = vk::ComputePipelineCreateInfo::default()
        .stage(stage)
        .layout(layout);
    let result = device.create_compute_pipelines(vk::PipelineCache::null(), &[info], None);
    device.destroy_shader_module(shader, None);
    let pipelines = match result {
        Ok(pipelines) => pipelines,
        Err((pipelines, err)) => {
            for pipeline in pipelines {
                if !pipeline.is_null() {
                    device.destroy_pipeline(pipeline, None);
                }
            }
            return Err(anyhow!("vkCreateComputePipelines failed: {err:?}"));
        }
    };
    pipelines
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("vkCreateComputePipelines returned no pipeline"))
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
    let info = vk::FenceCreateInfo::default();
    Ok(device.create_fence(&info, None)?)
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

unsafe fn initialize_buffer(allocator: &mut Allocator, allocation: &Allocation) -> Result<()> {
    let flags = allocator.allocation_memory_flags(allocation)?;
    let mut mapped = allocator.map(allocation)?;
    let values = bytemuck::cast_slice_mut::<u8, u32>(
        mapped.byte_range_mut(0, ELEMENTS * core::mem::size_of::<u32>()),
    );
    for (index, value) in values.iter_mut().enumerate() {
        *value = index as u32 * 3 + 11;
    }
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
    let values = bytemuck::cast_slice::<u8, u32>(
        mapped.byte_range(0, ELEMENTS * core::mem::size_of::<u32>()),
    );
    for (index, value) in values.iter().copied().enumerate() {
        let initial = index as u32 * 3 + 11;
        let expected = initial * 2 + 1;
        if value != expected {
            return Err(anyhow!(
                "compute output mismatch at {index}: got {value}, expected {expected}"
            ));
        }
    }
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "P6\n{} {}\n255", ELEMENTS, 16)?;
    for _y in 0..16 {
        for value in values.iter().copied() {
            let r = (value & 255) as u8;
            let g = ((value * 3) & 255) as u8;
            let b = (255_u32.saturating_sub(value & 255)) as u8;
            file.write_all(&[r, g, b])?;
        }
    }
    Ok(())
}
