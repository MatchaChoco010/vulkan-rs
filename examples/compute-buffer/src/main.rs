//! A compute shader that updates a storage buffer, with `VK_EXT_descriptor_heap`.
//!
//! The shader (`shaders/compute.slang`) is ordinary: it declares its buffer with
//! `set = 0, binding = 0` and knows nothing about heaps. There is no descriptor set
//! layout, pipeline layout, descriptor pool or descriptor set on the Vulkan side either.
//! Instead:
//!
//! 1. The descriptor of the buffer is written into a *resource heap*, which is a buffer.
//! 2. The pipeline is created with a *mapping* that says "binding 0 is the descriptor at
//!    this byte offset of the resource heap".
//! 3. The heap is bound before the dispatch.
//!
//! This is the smallest of the three ways to reach a heap from a shader. The others are
//! an index taken from push data (`examples/mesh-shader`) and shaders that index the
//! heap themselves (`examples/descriptor-heap-14`).
//!
//! The result goes to `target/example-output/compute_buffer.ppm`.

use anyhow::{Context, Result, anyhow, bail};
use std::ffi::{CStr, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::descriptor_heap::ResourceHeapPlan;
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const ELEMENTS: usize = 64;
const BUFFER_BYTES: u64 = (ELEMENTS * size_of::<u32>()) as u64;

struct Gpu {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    device: Device,
    queue: vk::Queue,
    queue_family: u32,
    heap_properties: vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>,
}

/// A buffer that stays mapped for as long as it lives.
struct MappedBuffer {
    buffer: vk::Buffer,
    allocation: Allocation,
    address: vk::DeviceAddress,
    bytes: *mut u8,
    size: u64,
}

impl MappedBuffer {
    /// # Safety
    ///
    /// The buffer must be alive, and the device must not be writing the memory.
    unsafe fn bytes(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.bytes, self.size as usize) }
    }
}

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let Some(gpu) = create_gpu()? else {
            println!("skipped: no Vulkan 1.4 device with VK_EXT_descriptor_heap");
            return Ok(());
        };
        let device = &gpu.device;
        let mut allocator = Allocator::new(AllocatorCreateDesc::new(
            device,
            gpu.instance
                .get_physical_device_memory_properties(gpu.physical_device),
            gpu.instance
                .get_physical_device_properties(gpu.physical_device),
        ));

        // The data the shader works on.
        let mut values = create_mapped_buffer(
            &gpu,
            &mut allocator,
            "values",
            BUFFER_BYTES,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            1,
        )?;
        for (index, word) in values.bytes().chunks_exact_mut(4).enumerate() {
            word.copy_from_slice(&initial_value(index).to_ne_bytes());
        }

        // The resource heap: room for one buffer descriptor, then the range the
        // implementation reserves for itself.
        let mut plan = ResourceHeapPlan::new(&gpu.heap_properties);
        let slots = plan.buffers(1)?;
        let layout = plan.finish()?;
        let mut heap = create_mapped_buffer(
            &gpu,
            &mut allocator,
            "resource heap",
            layout.size,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT,
            layout.alignment,
        )?;

        // Vulkan writes the descriptor into host memory that we hand it as a slice.
        let range = vk::DeviceAddressRangeKHR::default()
            .address(values.address)
            .size(BUFFER_BYTES);
        let descriptor = vk::ResourceDescriptorInfoEXT::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .data(vk::ResourceDescriptorDataEXT::address_range(&range));
        let destination = slots
            .bytes(heap.bytes(), 0)
            .context("the heap is smaller than its layout")?;
        device.write_resource_descriptors_ext(
            &[descriptor],
            &[vk::HostAddressRangeEXT::default().address(destination)],
        )?;

        let heap_offset = slots.offset(0).context("slot 0 exists")?;
        let pipeline = create_pipeline(device, u32::try_from(heap_offset)?)?;

        let pool = device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(gpu.queue_family),
            None,
        )?;
        let command_buffer = device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )?[0];
        device.begin_command_buffer(command_buffer, &vk::CommandBufferBeginInfo::default())?;

        // What the host wrote becomes visible to the shader: the heap as descriptors,
        // the values as storage.
        let host_to_shader = [
            vk::BufferMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::HOST)
                .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .dst_access_mask(vk::AccessFlags2::RESOURCE_HEAP_READ_EXT)
                .buffer(heap.buffer)
                .size(layout.size),
            vk::BufferMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::HOST)
                .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
                .dst_access_mask(
                    vk::AccessFlags2::SHADER_STORAGE_READ | vk::AccessFlags2::SHADER_STORAGE_WRITE,
                )
                .buffer(values.buffer)
                .size(BUFFER_BYTES),
        ];
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&host_to_shader),
        );

        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::COMPUTE, pipeline);
        device.cmd_bind_resource_heap_ext(command_buffer, &layout.bind_info(heap.address)?);
        device.cmd_dispatch(command_buffer, 1, 1, 1);

        let shader_to_host = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COMPUTE_SHADER)
            .src_access_mask(vk::AccessFlags2::SHADER_STORAGE_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ)
            .buffer(values.buffer)
            .size(BUFFER_BYTES)];
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&shader_to_host),
        );
        device.end_command_buffer(command_buffer)?;

        let fence = device.create_fence(&vk::FenceCreateInfo::default(), None)?;
        let command_buffers =
            [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
        let submits = [vk::SubmitInfo2::default().command_buffer_infos(&command_buffers)];
        device.queue_submit2(gpu.queue, &submits, fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;

        let results = values
            .bytes()
            .chunks_exact(4)
            .map(|word| u32::from_ne_bytes([word[0], word[1], word[2], word[3]]))
            .collect::<Vec<_>>();
        for (index, value) in results.iter().copied().enumerate() {
            let expected = initial_value(index) * 2 + 1;
            if value != expected {
                bail!("compute output mismatch at {index}: got {value}, expected {expected}");
            }
        }
        let path = output_path("compute_buffer.ppm");
        write_ppm(&path, &results)?;
        println!("all {ELEMENTS} values match; wrote {}", path.display());

        device.destroy_fence(fence, None);
        device.destroy_command_pool(pool, None);
        device.destroy_pipeline(pipeline, None);
        for buffer in [heap, values] {
            device.destroy_buffer(buffer.buffer, None);
            // Freeing a persistently mapped allocation also ends its mapping.
            allocator.free(buffer.allocation)?;
        }
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

fn initial_value(index: usize) -> u32 {
    index as u32 * 3 + 11
}

/// A compute pipeline without a pipeline layout. `heap_offset` is where, in the resource
/// heap, the descriptor for the shader's `set = 0, binding = 0` is.
unsafe fn create_pipeline(device: &Device, heap_offset: u32) -> Result<vk::Pipeline> {
    unsafe {
        let code = spirv_words(include_bytes!("shaders/compute.spv"))?;
        let module = device
            .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&code), None)?;

        let mappings = [vk::DescriptorSetAndBindingMappingEXT::default()
            .descriptor_set(0)
            .first_binding(0)
            .binding_count(1)
            .resource_mask(vk::SpirvResourceTypeFlagBitsEXT::READ_WRITE_STORAGE_BUFFER.into())
            .source(vk::DescriptorMappingSourceEXT::HEAP_WITH_CONSTANT_OFFSET)
            .source_data(vk::DescriptorMappingSourceDataEXT::constant_offset(
                vk::DescriptorMappingSourceConstantOffsetEXT::default().heap_offset(heap_offset),
            ))];
        let mut mapping_info =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::COMPUTE)
            .module(module)
            .name(c"main")
            .push_next(&mut mapping_info);

        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::ComputePipelineCreateInfo::default()
            .stage(stage)
            .push_next(&mut flags);
        let result = device.create_compute_pipelines(vk::PipelineCache::null(), &[info], None);
        device.destroy_shader_module(module, None);
        match result {
            Ok(pipelines) => Ok(pipelines[0]),
            Err((pipelines, error)) => {
                for pipeline in pipelines.into_iter().filter(|p| !p.is_null()) {
                    device.destroy_pipeline(pipeline, None);
                }
                Err(anyhow!("vkCreateComputePipelines failed: {error}"))
            }
        }
    }
}

/// A host-visible buffer with a device address, mapped until it is freed. `alignment` is
/// what the device address has to be a multiple of.
unsafe fn create_mapped_buffer(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &'static str,
    size: u64,
    usage: vk::BufferUsageFlags,
    alignment: u64,
) -> Result<MappedBuffer> {
    unsafe {
        let device = &gpu.device;
        let buffer = device.create_buffer(
            &vk::BufferCreateInfo::default()
                .size(size)
                .usage(usage | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS),
            None,
        )?;
        let mut requirements = device.get_buffer_memory_requirements(buffer);
        requirements.alignment = requirements.alignment.max(alignment);
        let allocation = allocator.allocate(AllocationCreateDesc {
            name: Some(name),
            requirements,
            location: MemoryLocation::CpuToGpu,
            linear: true,
            // Required for vkGetBufferDeviceAddress.
            memory_allocate_flags: vk::MemoryAllocateFlags::DEVICE_ADDRESS,
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;
        device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;
        if !allocator
            .allocation_memory_flags(&allocation)?
            .contains(vk::MemoryPropertyFlags::HOST_COHERENT)
        {
            bail!("this example needs host-coherent memory");
        }
        let bytes = allocator.map_persistent(&allocation)?.as_ptr();
        let address = device
            .get_buffer_device_address(&vk::BufferDeviceAddressInfo::default().buffer(buffer));
        Ok(MappedBuffer {
            buffer,
            allocation,
            address,
            bytes,
            size,
        })
    }
}

fn write_ppm(path: &std::path::Path, values: &[u32]) -> Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    writeln!(file, "P6\n{} {}\n255", values.len(), 16)?;
    for _ in 0..16 {
        for value in values {
            let r = (value & 255) as u8;
            let g = ((value * 3) & 255) as u8;
            let b = 255 - r;
            file.write_all(&[r, g, b])?;
        }
    }
    Ok(())
}

fn spirv_words(bytes: &[u8]) -> Result<Vec<u32>> {
    if !bytes.len().is_multiple_of(4) {
        bail!("SPIR-V size is not a multiple of four bytes");
    }
    Ok(bytes
        .chunks_exact(4)
        .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect())
}

fn has_extension(available: &[vk::ExtensionProperties], name: &CStr) -> bool {
    available
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}

/// `None` when no device can run the example.
unsafe fn create_gpu() -> Result<Option<Gpu>> {
    unsafe {
        let entry = Entry::load()?;

        let validation = c"VK_LAYER_KHRONOS_validation";
        let has_validation = entry
            .enumerate_instance_layer_properties()?
            .iter()
            .any(|layer| layer.layer_name_as_c_str() == Ok(validation));
        let instance_extensions = entry.enumerate_instance_extension_properties(None)?;
        let debug = has_validation && has_extension(&instance_extensions, vk::EXT_DEBUG_UTILS_NAME);
        let mut layer_names = Vec::new();
        let mut extension_names = Vec::new();
        if debug {
            layer_names.push(vk::CStrPtr::new(validation));
            extension_names.push(vk::CStrPtr::new(vk::EXT_DEBUG_UTILS_NAME));
        }

        // With Vulkan 1.4 in use, VK_EXT_descriptor_heap needs no other extension. The
        // registry data in the bindings can say so.
        let api_version = vk::API_VERSION_1_4;
        let heap_extension = vk::ExtensionInfo::get(vk::EXT_DESCRIPTOR_HEAP_NAME)
            .context("VK_EXT_descriptor_heap is in the registry")?;
        debug_assert!(heap_extension.dependencies_met(api_version, |_| false));

        let app = vk::ApplicationInfo::default()
            .application_name(c"compute-buffer")
            .api_version(api_version);
        let instance = entry.create_instance(
            &vk::InstanceCreateInfo::default()
                .application_info(&app)
                .enabled_layer_names(&layer_names)
                .enabled_extension_names(&extension_names),
            None,
        )?;
        let messenger = if debug {
            instance.create_debug_utils_messenger_ext(
                &vk::DebugUtilsMessengerCreateInfoEXT::default()
                    .message_severity(
                        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                    )
                    .message_type(
                        vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
                    )
                    .pfn_user_callback(Some(debug_callback)),
                None,
            )?
        } else {
            vk::DebugUtilsMessengerEXT::null()
        };

        // The first Vulkan 1.4 device with a compute queue and descriptor heaps.
        let mut selected = None;
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);
            let extensions =
                instance.enumerate_device_extension_properties(physical_device, None)?;
            if properties.api_version < api_version
                || !has_extension(&extensions, vk::EXT_DESCRIPTOR_HEAP_NAME)
            {
                continue;
            }
            let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
            let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut heap)
                .push_next(&mut vulkan12);
            instance.get_physical_device_features2(physical_device, &mut features);
            let family = instance
                .get_physical_device_queue_family_properties(physical_device)
                .iter()
                .position(|family| family.queue_flags.contains(vk::QueueFlags::COMPUTE));
            if let Some(family) = family
                && heap.descriptor_heap != vk::FALSE
                && vulkan12.buffer_device_address != vk::FALSE
            {
                println!(
                    "using {}",
                    properties.device_name_as_c_str()?.to_string_lossy()
                );
                selected = Some((physical_device, family as u32));
                break;
            }
        }
        let Some((physical_device, queue_family)) = selected else {
            if !messenger.is_null() {
                instance.destroy_debug_utils_messenger_ext(messenger, None);
            }
            instance.destroy_instance(None);
            return Ok(None);
        };

        let mut heap_properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default();
        let mut properties =
            vk::PhysicalDeviceProperties2::default().push_next(&mut heap_properties);
        instance.get_physical_device_properties2(physical_device, &mut properties);
        // Detach the copy from the chain it was filled through.
        heap_properties.p_next = std::ptr::null_mut();
        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = [vk::CStrPtr::new(vk::EXT_DESCRIPTOR_HEAP_NAME)];
        let mut heap_features =
            vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
        let mut vulkan12 =
            vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default().synchronization2(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut heap_features)
                .push_next(&mut vulkan12)
                .push_next(&mut vulkan13),
            None,
        )?;
        let queue = device.get_device_queue(queue_family, 0);

        Ok(Some(Gpu {
            instance,
            messenger,
            physical_device,
            device,
            queue,
            queue_family,
            heap_properties,
        }))
    }
}

impl Gpu {
    unsafe fn destroy(self) {
        unsafe {
            self.device.destroy_device(None);
            if !self.messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    // SAFETY: Vulkan passes a valid callback data pointer with a NUL-terminated message.
    let message = unsafe { CStr::from_ptr((*data).p_message) };
    eprintln!("validation: {}", message.to_string_lossy());
    vk::FALSE
}

/// Example images go to `target/example-output/`, never into the source tree.
fn output_path(file_name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/example-output");
    std::fs::create_dir_all(&dir).expect("create target/example-output");
    dir.join(file_name)
}
