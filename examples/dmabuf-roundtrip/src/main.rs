//! Exports a rendered image as a dma-buf and imports it again, the way a Wayland client
//! and a compositor share buffers.
//!
//! 1. Client side: create an image with a DRM format modifier, render into it, and
//!    describe it the way `zwp_linux_dmabuf_v1` does: fd, format, modifier, and the
//!    offset and stride of each plane.
//! 2. Compositor side: check that the description can be imported, create an image with
//!    exactly that layout, import the fd as its memory, and read the pixels back.
//!
//! Both sides run on one device here. Nothing in the import path depends on that.
//!
//! The image goes to `target/example-output/dmabuf_roundtrip.ppm`.

use anyhow::{Context, Result, anyhow, bail};
use std::ffi::{CStr, c_void};
use std::fs::File;
use std::io::Write;
use std::os::fd::{FromRawFd, IntoRawFd, OwnedFd};
use vulkan::vk;
use vulkan::{Device, Entry, Instance};
use vulkan_alloc::{
    AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc, MemoryLocation,
};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
/// `DRM_FORMAT_ARGB8888` in DRM terms.
const FORMAT: vk::Format = vk::Format::B8G8R8A8_UNORM;
const DMA_BUF: vk::ExternalMemoryHandleTypeFlagBits =
    vk::ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT;

const DEVICE_EXTENSIONS: [&CStr; 4] = [
    vk::KHR_EXTERNAL_MEMORY_FD_NAME,
    vk::EXT_EXTERNAL_MEMORY_DMA_BUF_NAME,
    vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME,
    vk::EXT_QUEUE_FAMILY_FOREIGN_NAME,
];

/// What a client sends to a compositor for one buffer.
struct Dmabuf {
    fd: OwnedFd,
    width: u32,
    height: u32,
    format: vk::Format,
    modifier: u64,
    planes: Vec<Plane>,
}

/// One memory plane of a dma-buf, as `zwp_linux_buffer_params_v1.add` describes it.
#[derive(Copy, Clone)]
struct Plane {
    offset: u64,
    stride: u64,
}

struct Gpu {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    device: Device,
    queue: vk::Queue,
    queue_family: u32,
}

fn main() -> Result<()> {
    // SAFETY: the example upholds the valid usage rules of the commands it calls; the
    // validation layer, when installed, reports any violation.
    unsafe { run() }
}

unsafe fn run() -> Result<()> {
    unsafe {
        let gpu = create_gpu()?;
        print_drm_nodes(&gpu);

        let memory_properties = gpu
            .instance
            .get_physical_device_memory_properties(gpu.physical_device);
        let properties = gpu
            .instance
            .get_physical_device_properties(gpu.physical_device);
        let mut allocator = Allocator::new(AllocatorCreateDesc::new(
            &gpu.device,
            memory_properties,
            properties,
        ));

        let (client_image, client_allocation, dmabuf) = export_image(&gpu, &mut allocator)?;
        println!(
            "exported {}x{} {:?}, modifier {:#018x}, {} plane(s), offset {} stride {}",
            dmabuf.width,
            dmabuf.height,
            dmabuf.format,
            dmabuf.modifier,
            dmabuf.planes.len(),
            dmabuf.planes[0].offset,
            dmabuf.planes[0].stride,
        );

        let (width, height) = (dmabuf.width, dmabuf.height);
        let (compositor_image, compositor_memory) = import_image(&gpu, dmabuf)?;
        let pixels = read_back(&gpu, &mut allocator, compositor_image, width, height)?;
        verify(&pixels, width, height)?;
        let path = output_path("dmabuf_roundtrip.ppm");
        write_ppm(&path, &pixels, width, height)?;
        println!("imported pixels match; wrote {}", path.display());

        gpu.device.destroy_image(compositor_image, None);
        gpu.device.free_memory(compositor_memory, None);
        gpu.device.destroy_image(client_image, None);
        allocator.free(client_allocation)?;
        drop(allocator);
        gpu.destroy();
        Ok(())
    }
}

/// Client side: render into an exportable image and describe it.
unsafe fn export_image(
    gpu: &Gpu,
    allocator: &mut Allocator,
) -> Result<(vk::Image, vulkan_alloc::Allocation, Dmabuf)> {
    unsafe {
        let device = &gpu.device;
        let usage = vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC;

        // Let the driver pick among the modifiers that can do what we need and that
        // use one memory plane. A real client would intersect this list with the
        // modifiers the compositor advertises.
        let needed =
            vk::FormatFeatureFlags::COLOR_ATTACHMENT | vk::FormatFeatureFlags::TRANSFER_SRC;
        let candidates = format_modifiers(gpu, FORMAT)?
            .into_iter()
            .filter(|m| {
                m.drm_format_modifier_plane_count == 1
                    && m.drm_format_modifier_tiling_features.contains(needed)
            })
            .map(|m| m.drm_format_modifier)
            .collect::<Vec<_>>();
        if candidates.is_empty() {
            bail!("no single-plane DRM format modifier supports rendering to {FORMAT:?}");
        }

        let mut modifier_list = vk::ImageDrmFormatModifierListCreateInfoEXT::default()
            .drm_format_modifiers(&candidates);
        let mut external =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF.into());
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(FORMAT)
            .extent(vk::Extent3D {
                width: WIDTH,
                height: HEIGHT,
                depth: 1,
            })
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(usage)
            .push_next(&mut modifier_list)
            .push_next(&mut external);
        let image = device.create_image(&image_info, None)?;

        // vulkan-alloc gives exportable memory its own VkDeviceMemory, so the exported
        // fd refers to this image only.
        let allocation = allocator.allocate(AllocationCreateDesc {
            name: Some("exported image"),
            requirements: device.get_image_memory_requirements(image),
            location: MemoryLocation::GpuOnly,
            linear: false,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: DMA_BUF.into(),
            allocation_scheme: AllocationScheme::DedicatedImage(image),
        })?;
        device.bind_image_memory(image, allocation.memory(), allocation.offset())?;

        let view = device.create_image_view(
            &vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(vk::ImageViewType::_2D)
                .format(FORMAT)
                .subresource_range(color_range()),
            None,
        )?;

        submit(gpu, |command_buffer| {
            let to_attachment = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(color_range())];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_attachment),
            );

            // Clears are enough to draw a recognizable picture without shaders: an
            // orange background with a blue rectangle in the lower right quadrant.
            let attachments = [vk::RenderingAttachmentInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(clear_color(BACKGROUND))];
            let full = vk::Rect2D {
                offset: vk::Offset2D::default(),
                extent: vk::Extent2D {
                    width: WIDTH,
                    height: HEIGHT,
                },
            };
            device.cmd_begin_rendering(
                command_buffer,
                &vk::RenderingInfo::default()
                    .render_area(full)
                    .layer_count(1)
                    .color_attachments(&attachments),
            );
            device.cmd_clear_attachments(
                command_buffer,
                &[vk::ClearAttachment::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .color_attachment(0)
                    .clear_value(clear_color(RECTANGLE))],
                &[vk::ClearRect::default()
                    .rect(vk::Rect2D {
                        offset: vk::Offset2D {
                            x: (WIDTH / 2) as i32,
                            y: (HEIGHT / 2) as i32,
                        },
                        extent: vk::Extent2D {
                            width: WIDTH / 2,
                            height: HEIGHT / 2,
                        },
                    })
                    .layer_count(1)],
            );
            device.cmd_end_rendering(command_buffer);

            // Hand the image to whoever imports it: release ownership to the foreign
            // queue family. The importer acquires it with the mirror image of this.
            let release = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::GENERAL)
                .src_queue_family_index(gpu.queue_family)
                .dst_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .image(image)
                .subresource_range(color_range())];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&release),
            );
        })?;
        device.destroy_image_view(view, None);

        // Which modifier did the driver pick, and where are the planes?
        let mut chosen = vk::ImageDrmFormatModifierPropertiesEXT::default();
        device.get_image_drm_format_modifier_properties_ext(image, &mut chosen)?;
        let plane = device.get_image_subresource_layout(
            image,
            &vk::ImageSubresource::default().aspect_mask(vk::ImageAspectFlags::MEMORY_PLANE_0_EXT),
        );

        // The returned fd is a new reference that we own.
        let fd = device.get_memory_fd_khr(
            &vk::MemoryGetFdInfoKHR::default()
                .memory(allocation.memory())
                .handle_type(DMA_BUF),
        )?;
        let fd = OwnedFd::from_raw_fd(fd);

        Ok((
            image,
            allocation,
            Dmabuf {
                fd,
                width: WIDTH,
                height: HEIGHT,
                format: FORMAT,
                modifier: chosen.drm_format_modifier,
                planes: vec![Plane {
                    offset: plane.offset,
                    stride: plane.row_pitch,
                }],
            },
        ))
    }
}

/// Compositor side: turn a client's dma-buf into an image that can be sampled or copied.
unsafe fn import_image(gpu: &Gpu, dmabuf: Dmabuf) -> Result<(vk::Image, vk::DeviceMemory)> {
    unsafe {
        let device = &gpu.device;
        let usage = vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::SAMPLED;
        if dmabuf.planes.len() != 1 {
            bail!("this example imports single-plane buffers only");
        }

        // 1. Can this format, modifier and usage be imported at this size?
        let mut modifier_info = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT::default()
            .drm_format_modifier(dmabuf.modifier)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let mut external_info =
            vk::PhysicalDeviceExternalImageFormatInfo::default().handle_type(DMA_BUF);
        let format_info = vk::PhysicalDeviceImageFormatInfo2::default()
            .format(dmabuf.format)
            .r#type(vk::ImageType::_2D)
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(usage)
            .push_next(&mut modifier_info)
            .push_next(&mut external_info);
        let mut external_properties = vk::ExternalImageFormatProperties::default();
        let mut format_properties =
            vk::ImageFormatProperties2::default().push_next(&mut external_properties);
        gpu.instance
            .get_physical_device_image_format_properties2(
                gpu.physical_device,
                &format_info,
                &mut format_properties,
            )
            .context("the device cannot import this format and modifier")?;
        let limits = format_properties.image_format_properties.max_extent;
        if dmabuf.width > limits.width || dmabuf.height > limits.height {
            bail!("the buffer is larger than the device can import");
        }
        if !external_properties
            .external_memory_properties
            .external_memory_features
            .contains(vk::ExternalMemoryFeatureFlags::IMPORTABLE)
        {
            bail!("dma-bufs with this format and modifier are not importable");
        }

        // 2. An image with exactly the layout the client described. Only the offset and
        //    the row pitch are given; Vulkan requires the other members to be zero.
        let plane_layouts = dmabuf
            .planes
            .iter()
            .map(|plane| {
                vk::SubresourceLayout::default()
                    .offset(plane.offset)
                    .row_pitch(plane.stride)
            })
            .collect::<Vec<_>>();
        let mut explicit = vk::ImageDrmFormatModifierExplicitCreateInfoEXT::default()
            .drm_format_modifier(dmabuf.modifier)
            .plane_layouts(&plane_layouts);
        let mut external =
            vk::ExternalMemoryImageCreateInfo::default().handle_types(DMA_BUF.into());
        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(dmabuf.format)
            .extent(vk::Extent3D {
                width: dmabuf.width,
                height: dmabuf.height,
                depth: 1,
            })
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT)
            .usage(usage)
            .push_next(&mut explicit)
            .push_next(&mut external);
        let image = device.create_image(&image_info, None)?;

        // 3. Import the fd as the image's memory. The memory type must be one that both
        //    the image and the dma-buf allow.
        let requirements = device.get_image_memory_requirements(image);
        let mut fd_properties = vk::MemoryFdPropertiesKHR::default();
        let raw_fd = {
            use std::os::fd::AsRawFd;
            dmabuf.fd.as_raw_fd()
        };
        device.get_memory_fd_properties_khr(DMA_BUF, raw_fd, &mut fd_properties)?;
        let type_bits = requirements.memory_type_bits & fd_properties.memory_type_bits;
        let Some(memory_type) = (0..u32::BITS).find(|index| type_bits & (1 << index) != 0) else {
            device.destroy_image(image, None);
            bail!("no memory type fits both the image and the dma-buf");
        };

        // A successful import takes ownership of the fd; on failure it is still ours.
        let raw_fd = dmabuf.fd.into_raw_fd();
        let mut import = vk::ImportMemoryFdInfoKHR::default()
            .handle_type(DMA_BUF)
            .fd(raw_fd);
        let mut dedicated = vk::MemoryDedicatedAllocateInfo::default().image(image);
        let allocate_info = vk::MemoryAllocateInfo::default()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type)
            .push_next(&mut import)
            .push_next(&mut dedicated);
        let memory = match device.allocate_memory(&allocate_info, None) {
            Ok(memory) => memory,
            Err(error) => {
                drop(OwnedFd::from_raw_fd(raw_fd));
                device.destroy_image(image, None);
                return Err(error).context("importing the dma-buf");
            }
        };
        device.bind_image_memory(image, memory, 0)?;
        Ok((image, memory))
    }
}

/// Copies an imported image into host memory as tightly packed BGRA.
unsafe fn read_back(
    gpu: &Gpu,
    allocator: &mut Allocator,
    image: vk::Image,
    width: u32,
    height: u32,
) -> Result<Vec<u8>> {
    unsafe {
        let device = &gpu.device;
        let size = u64::from(width) * u64::from(height) * 4;
        let buffer = device.create_buffer(
            &vk::BufferCreateInfo::default()
                .size(size)
                .usage(vk::BufferUsageFlags::TRANSFER_DST),
            None,
        )?;
        let allocation = allocator.allocate(AllocationCreateDesc {
            name: Some("readback"),
            requirements: device.get_buffer_memory_requirements(buffer),
            location: MemoryLocation::GpuToCpu,
            linear: true,
            memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
            export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })?;
        device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset())?;

        submit(gpu, |command_buffer| {
            // Acquire ownership from the foreign queue family. The contents of an
            // imported image are only defined when it arrives this way: a transition
            // from UNDEFINED would allow the driver to discard them.
            let acquire = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::GENERAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .src_queue_family_index(vk::QUEUE_FAMILY_FOREIGN_EXT)
                .dst_queue_family_index(gpu.queue_family)
                .image(image)
                .subresource_range(color_range())];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&acquire),
            );
            let regions = [vk::BufferImageCopy::default()
                .image_subresource(
                    vk::ImageSubresourceLayers::default()
                        .aspect_mask(vk::ImageAspectFlags::COLOR)
                        .layer_count(1),
                )
                .image_extent(vk::Extent3D {
                    width,
                    height,
                    depth: 1,
                })];
            device.cmd_copy_image_to_buffer(
                command_buffer,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer,
                &regions,
            );
        })?;

        let pixels = {
            let mapped = allocator.map(&allocation)?;
            mapped.byte_range(0, size as usize).to_vec()
        };
        device.destroy_buffer(buffer, None);
        allocator.free(allocation)?;
        Ok(pixels)
    }
}

const BACKGROUND: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
const RECTANGLE: [f32; 4] = [0.0, 0.25, 1.0, 1.0];

fn clear_color(rgba: [f32; 4]) -> vk::ClearValue {
    vk::ClearValue::color(vk::ClearColorValue::float32(rgba))
}

fn color_range() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1)
}

/// Checks that the imported image holds what the exporter drew.
fn verify(bgra: &[u8], width: u32, height: u32) -> Result<()> {
    let expect = |rgba: [f32; 4]| {
        let byte = |value: f32| (value * 255.0).round() as u8;
        [byte(rgba[2]), byte(rgba[1]), byte(rgba[0]), byte(rgba[3])]
    };
    for (x, y, color) in [
        (width / 4, height / 4, BACKGROUND),
        (width * 3 / 4, height / 4, BACKGROUND),
        (width / 4, height * 3 / 4, BACKGROUND),
        (width * 3 / 4, height * 3 / 4, RECTANGLE),
    ] {
        let index = ((y * width + x) * 4) as usize;
        let pixel = &bgra[index..index + 4];
        let expected = expect(color);
        // Allow one step of rounding difference between implementations.
        if pixel.iter().zip(expected).any(|(a, b)| a.abs_diff(b) > 1) {
            bail!("pixel ({x}, {y}) is {pixel:?}, expected {expected:?} (BGRA)");
        }
    }
    Ok(())
}

fn write_ppm(path: &std::path::Path, bgra: &[u8], width: u32, height: u32) -> Result<()> {
    let mut file = File::create(path)?;
    write!(file, "P6\n{width} {height}\n255\n")?;
    let rgb = bgra
        .chunks_exact(4)
        .flat_map(|pixel| [pixel[2], pixel[1], pixel[0]])
        .collect::<Vec<_>>();
    file.write_all(&rgb)?;
    Ok(())
}

/// Every DRM format modifier the device supports for `format`.
unsafe fn format_modifiers(
    gpu: &Gpu,
    format: vk::Format,
) -> Result<Vec<vk::DrmFormatModifierPropertiesEXT>> {
    unsafe {
        // First call: how many?
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let count = list.drm_format_modifier_count as usize;

        // Second call: fill our array.
        let mut modifiers = vec![vk::DrmFormatModifierPropertiesEXT::default(); count];
        let mut list = vk::DrmFormatModifierPropertiesListEXT::default()
            .drm_format_modifier_properties(&mut modifiers);
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let written = list.drm_format_modifier_count as usize;
        modifiers.truncate(written);
        Ok(modifiers)
    }
}

/// The DRM device nodes of the physical device, which is how a compositor matches a
/// Vulkan device to the KMS device it drives.
unsafe fn print_drm_nodes(gpu: &Gpu) {
    unsafe {
        let Ok(extensions) = gpu
            .instance
            .enumerate_device_extension_properties(gpu.physical_device, None)
        else {
            return;
        };
        if !has_extension(&extensions, vk::EXT_PHYSICAL_DEVICE_DRM_NAME) {
            println!("VK_EXT_physical_device_drm is not supported");
            return;
        }
        let mut drm = vk::PhysicalDeviceDrmPropertiesEXT::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut drm);
        gpu.instance
            .get_physical_device_properties2(gpu.physical_device, &mut properties);
        if drm.has_primary != 0 {
            println!(
                "DRM primary node {}:{}",
                drm.primary_major, drm.primary_minor
            );
        }
        if drm.has_render != 0 {
            println!("DRM render node {}:{}", drm.render_major, drm.render_minor);
        }
    }
}

fn has_extension(available: &[vk::ExtensionProperties], name: &CStr) -> bool {
    available
        .iter()
        .any(|extension| extension.extension_name_as_c_str() == Ok(name))
}

unsafe fn create_gpu() -> Result<Gpu> {
    unsafe {
        let entry = Entry::load()?;

        // Validation is optional here so that the example also runs where the layer
        // is not installed.
        let layers = entry.enumerate_instance_layer_properties()?;
        let validation = c"VK_LAYER_KHRONOS_validation";
        let has_validation = layers
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
        let app = vk::ApplicationInfo::default()
            .application_name(c"dmabuf-roundtrip")
            .api_version(vk::API_VERSION_1_3);
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

        // The first Vulkan 1.3 device that has every extension the round trip needs.
        let mut selected = None;
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);
            let extensions =
                instance.enumerate_device_extension_properties(physical_device, None)?;
            let families = instance.get_physical_device_queue_family_properties(physical_device);
            let family = families
                .iter()
                .position(|family| family.queue_flags.contains(vk::QueueFlags::GRAPHICS));
            let supported = properties.api_version >= vk::API_VERSION_1_3
                && DEVICE_EXTENSIONS
                    .iter()
                    .all(|name| has_extension(&extensions, name));
            if let (true, Some(family)) = (supported, family) {
                println!(
                    "using {}",
                    properties.device_name_as_c_str()?.to_string_lossy()
                );
                selected = Some((physical_device, family as u32));
                break;
            }
        }
        let (physical_device, queue_family) = selected.ok_or_else(|| {
            anyhow!("no Vulkan 1.3 device supports dma-buf import and export with modifiers")
        })?;

        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let extension_names = DEVICE_EXTENSIONS.map(vk::CStrPtr::new);
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true);
        let device = instance.create_device(
            physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut vulkan13),
            None,
        )?;
        let queue = device.get_device_queue(queue_family, 0);

        Ok(Gpu {
            instance,
            messenger,
            physical_device,
            device,
            queue,
            queue_family,
        })
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

/// Records, submits and waits for one command buffer.
unsafe fn submit(gpu: &Gpu, record: impl FnOnce(vk::CommandBuffer)) -> Result<()> {
    unsafe {
        let device = &gpu.device;
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
        device.begin_command_buffer(
            command_buffer,
            &vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
        )?;
        record(command_buffer);
        device.end_command_buffer(command_buffer)?;

        let fence = device.create_fence(&vk::FenceCreateInfo::default(), None)?;
        let command_buffers =
            [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
        let submits = [vk::SubmitInfo2::default().command_buffer_infos(&command_buffers)];
        device.queue_submit2(gpu.queue, &submits, fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;
        device.destroy_fence(fence, None);
        device.destroy_command_pool(pool, None);
        Ok(())
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
