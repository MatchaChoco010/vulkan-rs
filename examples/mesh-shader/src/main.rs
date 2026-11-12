//! A task shader and a mesh shader that read their geometry from storage buffers, with
//! `VK_EXT_descriptor_heap`.
//!
//! The shaders (`shaders/mesh.slang`) declare three buffers with `set = 0` and bindings
//! 0 to 2, and a push constant block. There is no descriptor set and no pipeline layout.
//! The pipeline is created with a *mapping* for each binding that says "the index of
//! this descriptor in the resource heap is the `u32` at this offset of the push data".
//! So the draw chooses its buffers by pushing three indices, next to the matrix that the
//! shader reads as its push constants.
//!
//! `examples/compute-buffer` shows the simpler mapping with a fixed offset.
//!
//! The image goes to `target/example-output/mesh_shader.ppm`.

use anyhow::{Context, Result, anyhow, bail};
use glam::{Mat4, Vec3};
use std::ffi::{CStr, c_void};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::mem::offset_of;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::descriptor_heap::{DescriptorSlots, ResourceHeapPlan};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;
const FORMAT: vk::Format = vk::Format::R8G8B8A8_UNORM;
const IMAGE_BYTES: u64 = (WIDTH * HEIGHT * 4) as u64;

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

/// What `vkCmdPushDataEXT` sends for a draw.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct PushData {
    /// The shader's push constant block starts at offset 0 of the push data.
    clip_from_world: [[f32; 4]; 4],
    /// Indices into the resource heap, read by the mappings of the pipeline.
    vertices: u32,
    indices: u32,
    draws: u32,
    _pad: u32,
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

struct Gpu {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    device: Device,
    queue: vk::Queue,
    queue_family: u32,
    heap_properties: vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static>,
    storage_buffer_alignment: u64,
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
            println!("skipped: no Vulkan 1.4 device with mesh shaders and descriptor heaps");
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

        let vertices = create_storage_buffer(&gpu, &mut allocator, "vertices", &MESH_VERTICES)?;
        let indices = create_storage_buffer(&gpu, &mut allocator, "indices", &MESH_INDICES)?;
        let draws = create_storage_buffer(&gpu, &mut allocator, "draws", &MESH_DRAWS)?;
        let mut readback = create_mapped_buffer(
            &gpu,
            &mut allocator,
            "readback",
            IMAGE_BYTES,
            vk::BufferUsageFlags::TRANSFER_DST,
            1,
            MemoryLocation::GpuToCpu,
        )?;

        // The resource heap: three buffer descriptors, then the range the implementation
        // reserves for itself.
        let mut plan = ResourceHeapPlan::new(&gpu.heap_properties);
        let slots = plan.buffers(3)?;
        let layout = plan.finish()?;
        let mut heap = create_mapped_buffer(
            &gpu,
            &mut allocator,
            "resource heap",
            layout.size,
            vk::BufferUsageFlags::DESCRIPTOR_HEAP_EXT,
            layout.alignment,
            MemoryLocation::CpuToGpu,
        )?;

        // One call writes all three descriptors, each into its own slot of the heap.
        let ranges = [&vertices, &indices, &draws].map(|buffer| {
            vk::DeviceAddressRangeKHR::default()
                .address(buffer.address)
                .size(buffer.size)
        });
        let descriptors = [0, 1, 2].map(|slot| {
            vk::ResourceDescriptorInfoEXT::default()
                .ty(vk::DescriptorType::STORAGE_BUFFER)
                .data(vk::ResourceDescriptorDataEXT::address_range(&ranges[slot]))
        });
        let destinations = slots
            .slices(heap.bytes())
            .context("the heap is smaller than its layout")?
            .map(|bytes| vk::HostAddressRangeEXT::default().address(bytes))
            .collect::<Vec<_>>();
        device.write_resource_descriptors_ext(&descriptors, &destinations)?;

        let image = device.create_image(
            &vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::_2D)
                .format(FORMAT)
                .extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1))
                .mip_levels(1)
                .array_layers(1)
                .samples(vk::SampleCountFlagBits::_1)
                .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC),
            None,
        )?;
        let image_allocation = allocator.allocate(AllocationCreateDesc {
            name: Some("color image"),
            requirements: device.get_image_memory_requirements(image),
            location: MemoryLocation::GpuOnly,
            linear: false,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;
        device.bind_image_memory(image, image_allocation.memory(), image_allocation.offset())?;
        let color_range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let image_view = device.create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::_2D)
                .format(FORMAT)
                .subresource_range(color_range),
            None,
        )?;

        let pipeline = create_pipeline(device, &slots)?;

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

        // What the host wrote becomes visible to the shaders: the heap as descriptors,
        // the geometry as storage.
        let geometry_stages =
            vk::PipelineStageFlags2::TASK_SHADER_EXT | vk::PipelineStageFlags2::MESH_SHADER_EXT;
        let host_write = |buffer: &MappedBuffer, access| {
            vk::BufferMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::HOST)
                .src_access_mask(vk::AccessFlags2::HOST_WRITE)
                .dst_stage_mask(geometry_stages)
                .dst_access_mask(access)
                .buffer(buffer.buffer)
                .size(buffer.size)
        };
        let buffer_barriers = [
            host_write(&heap, vk::AccessFlags2::RESOURCE_HEAP_READ_EXT),
            host_write(&vertices, vk::AccessFlags2::SHADER_STORAGE_READ),
            host_write(&indices, vk::AccessFlags2::SHADER_STORAGE_READ),
            host_write(&draws, vk::AccessFlags2::SHADER_STORAGE_READ),
        ];
        let to_color = [vk::ImageMemoryBarrier2::default()
            .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .old_layout(vk::ImageLayout::UNDEFINED)
            .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .image(image)
            .subresource_range(color_range)];
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default()
                .buffer_memory_barriers(&buffer_barriers)
                .image_memory_barriers(&to_color),
        );

        let attachments = [vk::RenderingAttachmentInfo::default()
            .image_view(image_view)
            .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .clear_value(vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            })];
        let extent = vk::Extent2D::default().width(WIDTH).height(HEIGHT);
        device.cmd_begin_rendering(
            command_buffer,
            &vk::RenderingInfo::default()
                .render_area(vk::Rect2D::default().extent(extent))
                .layer_count(1)
                .color_attachments(&attachments),
        );
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_bind_resource_heap_ext(command_buffer, &layout.bind_info(heap.address)?);

        // The matrix for the shader, and which descriptors the draw uses.
        let push_data = PushData {
            clip_from_world: mesh_clip_from_world().to_cols_array_2d(),
            vertices: slots.index(0).context("slot 0 exists")?,
            indices: slots.index(1).context("slot 1 exists")?,
            draws: slots.index(2).context("slot 2 exists")?,
            _pad: 0,
        };
        device.cmd_push_data_ext(
            command_buffer,
            &vk::PushDataInfoEXT::default().data(
                vk::HostAddressRangeConstEXT::default().address(bytemuck::bytes_of(&push_data)),
            ),
        );
        device.cmd_draw_mesh_tasks_ext(command_buffer, 1, 1, 1);
        device.cmd_end_rendering(command_buffer);

        let to_transfer = [vk::ImageMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::COPY)
            .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
            .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
            .image(image)
            .subresource_range(color_range)];
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().image_memory_barriers(&to_transfer),
        );
        let regions = [vk::BufferImageCopy::default()
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .layer_count(1),
            )
            .image_extent(vk::Extent3D::default().width(WIDTH).height(HEIGHT).depth(1))];
        device.cmd_copy_image_to_buffer(
            command_buffer,
            image,
            vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
            readback.buffer,
            &regions,
        );
        let to_host = [vk::BufferMemoryBarrier2::default()
            .src_stage_mask(vk::PipelineStageFlags2::COPY)
            .src_access_mask(vk::AccessFlags2::TRANSFER_WRITE)
            .dst_stage_mask(vk::PipelineStageFlags2::HOST)
            .dst_access_mask(vk::AccessFlags2::HOST_READ)
            .buffer(readback.buffer)
            .size(IMAGE_BYTES)];
        device.cmd_pipeline_barrier2(
            command_buffer,
            &vk::DependencyInfo::default().buffer_memory_barriers(&to_host),
        );
        device.end_command_buffer(command_buffer)?;

        let fence = device.create_fence(&vk::FenceCreateInfo::default(), None)?;
        let command_buffers =
            [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
        let submits = [vk::SubmitInfo2::default().command_buffer_infos(&command_buffers)];
        device.queue_submit2(gpu.queue, &submits, fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;

        let path = output_path("mesh_shader.ppm");
        verify_and_write_ppm(readback.bytes(), &path)?;
        println!("wrote {}", path.display());

        device.destroy_fence(fence, None);
        device.destroy_command_pool(pool, None);
        device.destroy_pipeline(pipeline, None);
        device.destroy_image_view(image_view, None);
        device.destroy_image(image, None);
        allocator.free(image_allocation)?;
        for buffer in [heap, readback, draws, indices, vertices] {
            device.destroy_buffer(buffer.buffer, None);
            // Freeing a persistently mapped allocation also ends its mapping.
            allocator.free(buffer.allocation)?;
        }
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

/// A graphics pipeline without a pipeline layout. The mappings connect the bindings of
/// the shaders to the heap indices in [`PushData`].
unsafe fn create_pipeline(device: &Device, slots: &DescriptorSlots) -> Result<vk::Pipeline> {
    unsafe {
        let task = create_shader_module(device, include_bytes!("shaders/task.spv"))?;
        let mesh = create_shader_module(device, include_bytes!("shaders/mesh.spv"))?;
        let fragment = create_shader_module(device, include_bytes!("shaders/fragment.spv"))?;

        // Binding `b` of set 0 is the descriptor at `index * stride` in the resource
        // heap, where `index` is the u32 at `push_offset` of the push data. The stride
        // is the distance between two slots of the run that `ResourceHeapPlan` laid out.
        let stride = u32::try_from(slots.stride)?;
        let mapping = |binding: u32, push_offset: usize| {
            vk::DescriptorSetAndBindingMappingEXT::default()
                .descriptor_set(0)
                .first_binding(binding)
                .binding_count(1)
                .resource_mask(vk::SpirvResourceTypeFlagBitsEXT::READ_ONLY_STORAGE_BUFFER.into())
                .source(vk::DescriptorMappingSourceEXT::HEAP_WITH_PUSH_INDEX)
                .source_data(vk::DescriptorMappingSourceDataEXT::push_index(
                    vk::DescriptorMappingSourcePushIndexEXT::default()
                        .push_offset(push_offset as u32)
                        .heap_index_stride(stride),
                ))
        };
        let mappings = [
            mapping(0, offset_of!(PushData, vertices)),
            mapping(1, offset_of!(PushData, indices)),
            mapping(2, offset_of!(PushData, draws)),
        ];
        // A stage ignores the mappings for bindings it does not use, so both stages can
        // take the same list. Each needs its own struct in its own chain.
        let mut task_mappings =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let mut mesh_mappings =
            vk::ShaderDescriptorSetAndBindingMappingInfoEXT::default().mappings(&mappings);
        let stages = [
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::TASK_EXT)
                .module(task)
                .name(c"main")
                .push_next(&mut task_mappings),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::MESH_EXT)
                .module(mesh)
                .name(c"main")
                .push_next(&mut mesh_mappings),
            vk::PipelineShaderStageCreateInfo::default()
                .stage(vk::ShaderStageFlagBits::FRAGMENT)
                .module(fragment)
                .name(c"main"),
        ];

        // A negative height flips the image so that +Y is up.
        let viewports = [vk::Viewport::default()
            .y(HEIGHT as f32)
            .width(WIDTH as f32)
            .height(-(HEIGHT as f32))
            .max_depth(1.0)];
        let scissors =
            [vk::Rect2D::default().extent(vk::Extent2D::default().width(WIDTH).height(HEIGHT))];
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
        let blend_attachments = [vk::PipelineColorBlendAttachmentState::default()
            .color_write_mask(
                vk::ColorComponentFlags::R
                    | vk::ColorComponentFlags::G
                    | vk::ColorComponentFlags::B
                    | vk::ColorComponentFlags::A,
            )];
        let color_blend_state =
            vk::PipelineColorBlendStateCreateInfo::default().attachments(&blend_attachments);
        let formats = [FORMAT];
        let mut rendering =
            vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
        // `DESCRIPTOR_HEAP_EXT` is what allows the layout to be null.
        let mut flags = vk::PipelineCreateFlags2CreateInfo::default()
            .flags(vk::PipelineCreateFlags2::DESCRIPTOR_HEAP_EXT);
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .push_next(&mut rendering)
            .push_next(&mut flags);
        let result = device.create_graphics_pipelines(vk::PipelineCache::null(), &[info], None);
        for module in [fragment, mesh, task] {
            device.destroy_shader_module(module, None);
        }
        match result {
            Ok(pipelines) => Ok(pipelines[0]),
            Err((pipelines, error)) => {
                for pipeline in pipelines.into_iter().filter(|p| !p.is_null()) {
                    device.destroy_pipeline(pipeline, None);
                }
                Err(anyhow!("vkCreateGraphicsPipelines failed: {error}"))
            }
        }
    }
}

fn mesh_clip_from_world() -> Mat4 {
    let view = glam::camera::rh::view::look_at_mat4(Vec3::new(0.0, 0.0, 2.2), Vec3::ZERO, Vec3::Y);
    let projection = glam::camera::rh::proj::directx::perspective(
        55.0_f32.to_radians(),
        WIDTH as f32 / HEIGHT as f32,
        0.1,
        16.0,
    );
    projection * view
}

/// A storage buffer holding `values`, addressable by a descriptor in the heap.
unsafe fn create_storage_buffer<T: bytemuck::Pod>(
    gpu: &Gpu,
    allocator: &mut Allocator,
    name: &'static str,
    values: &[T],
) -> Result<MappedBuffer> {
    unsafe {
        let data: &[u8] = bytemuck::cast_slice(values);
        let mut buffer = create_mapped_buffer(
            gpu,
            allocator,
            name,
            data.len() as u64,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            // The address in a storage buffer descriptor has to be a multiple of this.
            gpu.storage_buffer_alignment,
            MemoryLocation::CpuToGpu,
        )?;
        buffer.bytes().copy_from_slice(data);
        Ok(buffer)
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
    location: MemoryLocation,
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
            location,
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

fn verify_and_write_ppm(bytes: &[u8], path: &std::path::Path) -> Result<()> {
    let pixels = bytes.chunks_exact(4).collect::<Vec<_>>();
    let colored_pixels = pixels
        .iter()
        .filter(|px| px[0] > 32 || px[1] > 32 || px[2] > 32)
        .count();
    if colored_pixels < 128 {
        bail!("mesh shader draw covered too few pixels: {colored_pixels}");
    }

    let center = pixels[(HEIGHT as usize / 2) * WIDTH as usize + WIDTH as usize / 2];
    if center[0] <= 16 && center[1] <= 16 && center[2] <= 16 {
        bail!("mesh shader draw left the image center black");
    }

    for &(x, y) in &[
        (0_usize, 0_usize),
        (WIDTH as usize - 1, 0_usize),
        (0_usize, HEIGHT as usize - 1),
        (WIDTH as usize - 1, HEIGHT as usize - 1),
    ] {
        let px = pixels[y * WIDTH as usize + x];
        if px[0] > 16 || px[1] > 16 || px[2] > 16 {
            bail!("mesh shader draw colored a clear corner pixel at ({x}, {y})");
        }
    }

    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "P6\n{WIDTH} {HEIGHT}\n255")?;
    for pixel in pixels {
        writer.write_all(&pixel[..3])?;
    }
    Ok(())
}

unsafe fn create_shader_module(device: &Device, bytes: &[u8]) -> Result<vk::ShaderModule> {
    if !bytes.len().is_multiple_of(4) {
        bail!("SPIR-V size is not a multiple of four bytes");
    }
    let code = bytes
        .chunks_exact(4)
        .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
        .collect::<Vec<_>>();
    unsafe {
        Ok(
            device
                .create_shader_module(&vk::ShaderModuleCreateInfo::default().code(&code), None)?,
        )
    }
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

        // With Vulkan 1.4 in use, neither extension needs another one.
        let api_version = vk::API_VERSION_1_4;
        let app = vk::ApplicationInfo::default()
            .application_name(c"mesh-shader")
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

        let device_extensions = [vk::EXT_MESH_SHADER_NAME, vk::EXT_DESCRIPTOR_HEAP_NAME];
        let mut selected = None;
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);
            let extensions =
                instance.enumerate_device_extension_properties(physical_device, None)?;
            if properties.api_version < api_version
                || !device_extensions
                    .iter()
                    .all(|name| has_extension(&extensions, name))
            {
                continue;
            }
            let mut mesh = vk::PhysicalDeviceMeshShaderFeaturesEXT::default();
            let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
            let mut features = vk::PhysicalDeviceFeatures2::default()
                .push_next(&mut mesh)
                .push_next(&mut heap);
            instance.get_physical_device_features2(physical_device, &mut features);
            let family = instance
                .get_physical_device_queue_family_properties(physical_device)
                .iter()
                .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS));
            if let Some(family) = family
                && mesh.task_shader != vk::FALSE
                && mesh.mesh_shader != vk::FALSE
                && heap.descriptor_heap != vk::FALSE
            {
                println!(
                    "using {}",
                    properties.device_name_as_c_str()?.to_string_lossy()
                );
                selected = Some((
                    physical_device,
                    family as u32,
                    properties.limits.min_storage_buffer_offset_alignment,
                ));
                break;
            }
        }
        let Some((physical_device, queue_family, storage_buffer_alignment)) = selected else {
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
        let extension_names = device_extensions.map(vk::CStrPtr::new);
        let mut mesh_features = vk::PhysicalDeviceMeshShaderFeaturesEXT::default()
            .task_shader(true)
            .mesh_shader(true);
        let mut heap_features =
            vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
        // Core since 1.2 and 1.3; a Vulkan 1.4 device has all three.
        let mut vulkan12 =
            vk::PhysicalDeviceVulkan12Features::default().buffer_device_address(true);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .synchronization2(true)
            .dynamic_rendering(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut mesh_features)
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
            storage_buffer_alignment,
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
