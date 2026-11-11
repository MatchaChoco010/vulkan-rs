//! Tests against a real Vulkan implementation.
//!
//! They need a Vulkan loader and a Vulkan 1.3 device, which lavapipe provides in CI.
//! Without one the tests are skipped, unless `VULKAN_RS_REQUIRE_DEVICE_TEST` is set.
//!
//! When `VK_LAYER_KHRONOS_validation` is installed it is enabled, and any validation
//! error fails the test. That is the main check that the generated structs carry the
//! values the setters were given.
//!
//! The tests are also the smallest working examples of what a compositor needs:
//! instance and device creation, `pNext` queries, dynamic rendering, synchronization2,
//! timeline semaphores, and memory and fence sharing through file descriptors.

// `Entry::load` needs the `std` feature.
#![cfg(feature = "std")]

use std::ffi::{CStr, c_void};
use std::sync::Mutex;
use vulkan::vk::{self, Handle};
use vulkan::{Device, Entry, Instance};

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

/// An instance and a device with one queue, torn down on drop.
struct Gpu {
    instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    /// Validation errors, filled by the debug messenger.
    errors: *mut Mutex<Vec<String>>,
    physical_device: vk::PhysicalDevice,
    device: Device,
    queue: vk::Queue,
    queue_family: u32,
    extensions: Vec<&'static CStr>,
    instance_extensions: Vec<&'static CStr>,
    /// `samplerYcbcrConversion` is supported and enabled.
    ycbcr: bool,
}

unsafe extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    user_data: *mut c_void,
) -> vk::Bool32 {
    // SAFETY: `user_data` is the `Mutex` leaked in `Gpu::new`, and Vulkan passes a valid
    // callback data pointer with a NUL-terminated message.
    unsafe {
        let errors = &*user_data.cast::<Mutex<Vec<String>>>();
        let message = CStr::from_ptr((*data).p_message).to_string_lossy();
        errors.lock().unwrap().push(message.into_owned());
    }
    vk::FALSE
}

impl Gpu {
    /// Creates a device with every extension of `wanted` that the device supports.
    /// Returns `None` when there is no usable Vulkan implementation.
    fn new(wanted: &[&'static CStr]) -> Option<Self> {
        Self::with_instance_extensions(&[], wanted)
    }

    /// Like [`Gpu::new`], and also enables the supported instance extensions of
    /// `instance_wanted`.
    fn with_instance_extensions(
        instance_wanted: &[&'static CStr],
        wanted: &[&'static CStr],
    ) -> Option<Self> {
        match unsafe { Self::create(instance_wanted, wanted) } {
            Ok(gpu) => Some(gpu),
            Err(reason) => {
                assert!(
                    std::env::var_os("VULKAN_RS_REQUIRE_DEVICE_TEST").is_none(),
                    "device test is required but cannot run: {reason}"
                );
                eprintln!("skipping device test: {reason}");
                None
            }
        }
    }

    unsafe fn create(
        instance_wanted: &[&'static CStr],
        wanted: &[&'static CStr],
    ) -> Result<Self, String> {
        unsafe {
            let entry = Entry::load().map_err(|e| e.to_string())?;

            let layers = entry
                .enumerate_instance_layer_properties()
                .map_err(|e| e.to_string())?;
            let has_validation = layers
                .iter()
                .any(|layer| layer.layer_name_as_c_str() == Ok(VALIDATION_LAYER));
            let instance_extensions = entry
                .enumerate_instance_extension_properties(None)
                .map_err(|e| e.to_string())?;
            let has_debug_utils = instance_extensions
                .iter()
                .any(|e| e.extension_name_as_c_str() == Ok(vk::EXT_DEBUG_UTILS_NAME));

            let mut enabled_layers = Vec::new();
            let mut enabled_extensions = Vec::new();
            if has_validation && has_debug_utils {
                enabled_layers.push(vk::CStrPtr::new(VALIDATION_LAYER));
                enabled_extensions.push(vk::CStrPtr::new(vk::EXT_DEBUG_UTILS_NAME));
            }
            let mut instance_extension_names = Vec::new();
            for &name in instance_wanted {
                if instance_extensions
                    .iter()
                    .any(|e| e.extension_name_as_c_str() == Ok(name))
                {
                    enabled_extensions.push(vk::CStrPtr::new(name));
                    instance_extension_names.push(name);
                }
            }
            let app = vk::ApplicationInfo::default()
                .application_name(c"vulkan-rs device test")
                .api_version(vk::API_VERSION_1_3);
            let instance_info = vk::InstanceCreateInfo::default()
                .application_info(&app)
                .enabled_layer_names(&enabled_layers)
                .enabled_extension_names(&enabled_extensions);
            let instance = entry
                .create_instance(&instance_info, None)
                .map_err(|e| format!("vkCreateInstance: {e}"))?;

            let errors = Box::into_raw(Box::new(Mutex::new(Vec::new())));
            let mut messenger = vk::DebugUtilsMessengerEXT::null();
            if !enabled_layers.is_empty() {
                let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                    .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
                    .message_type(
                        vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                            | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
                    )
                    .pfn_user_callback(Some(debug_callback))
                    .user_data(errors.cast());
                messenger = instance
                    .create_debug_utils_messenger_ext(&info, None)
                    .map_err(|e| e.to_string())?;
            }

            // The first Vulkan 1.3 device with a graphics queue.
            let mut selected = None;
            for physical_device in instance
                .enumerate_physical_devices()
                .map_err(|e| e.to_string())?
            {
                let properties = instance.get_physical_device_properties(physical_device);
                if properties.api_version < vk::API_VERSION_1_3 {
                    continue;
                }
                let families =
                    instance.get_physical_device_queue_family_properties(physical_device);
                if let Some(family) = families
                    .iter()
                    .position(|f| f.queue_flags.contains(vk::QueueFlags::GRAPHICS))
                {
                    selected = Some((physical_device, family as u32, properties));
                    break;
                }
            }
            let Some((physical_device, queue_family, properties)) = selected else {
                if !messenger.is_null() {
                    instance.destroy_debug_utils_messenger_ext(messenger, None);
                }
                instance.destroy_instance(None);
                drop(Box::from_raw(errors));
                return Err("no Vulkan 1.3 device with a graphics queue".to_string());
            };
            eprintln!(
                "device test on {:?}",
                properties.device_name_as_c_str().unwrap_or(c"?")
            );

            let available = instance
                .enumerate_device_extension_properties(physical_device, None)
                .map_err(|e| e.to_string())?;
            let extensions = wanted
                .iter()
                .copied()
                .filter(|name| {
                    available
                        .iter()
                        .any(|e| e.extension_name_as_c_str() == Ok(name))
                })
                .collect::<Vec<_>>();
            let extension_names = extensions
                .iter()
                .map(|name| vk::CStrPtr::new(name))
                .collect::<Vec<_>>();

            let priorities = [1.0];
            let queues = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family)
                .queue_priorities(&priorities)];
            // Optional feature: enable Y'CbCr conversion where the device has it.
            let mut supported11 = vk::PhysicalDeviceVulkan11Features::default();
            let mut supported = vk::PhysicalDeviceFeatures2::default().push_next(&mut supported11);
            instance.get_physical_device_features2(physical_device, &mut supported);
            let ycbcr = supported11.sampler_ycbcr_conversion != vk::FALSE;

            let mut vulkan11 =
                vk::PhysicalDeviceVulkan11Features::default().sampler_ycbcr_conversion(ycbcr);
            let mut vulkan12 =
                vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true);
            let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
                .dynamic_rendering(true)
                .synchronization2(true);
            let device_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(&queues)
                .enabled_extension_names(&extension_names)
                .push_next(&mut vulkan11)
                .push_next(&mut vulkan12)
                .push_next(&mut vulkan13);
            let device = instance
                .create_device(physical_device, &device_info, None)
                .map_err(|e| format!("vkCreateDevice: {e}"))?;
            let queue = device.get_device_queue(queue_family, 0);

            Ok(Self {
                instance,
                messenger,
                errors,
                physical_device,
                device,
                queue,
                queue_family,
                extensions,
                instance_extensions: instance_extension_names,
                ycbcr,
            })
        }
    }

    fn has(&self, extension: &CStr) -> bool {
        self.extensions.contains(&extension)
    }

    /// Index of a memory type allowed by `type_bits` that has all of `flags`.
    fn memory_type(&self, type_bits: u32, flags: vk::MemoryPropertyFlags) -> Option<u32> {
        let memory = unsafe {
            self.instance
                .get_physical_device_memory_properties(self.physical_device)
        };
        memory
            .memory_types_as_slice()
            .iter()
            .enumerate()
            .find(|(index, ty)| type_bits & (1 << index) != 0 && ty.property_flags.contains(flags))
            .map(|(index, _)| index as u32)
    }

    /// Records `record` into a one-time command buffer, submits it with
    /// `vkQueueSubmit2` and waits for it.
    unsafe fn submit(&self, record: impl FnOnce(vk::CommandBuffer)) {
        unsafe {
            let device = &self.device;
            let pool_info = vk::CommandPoolCreateInfo::default()
                .queue_family_index(self.queue_family)
                .flags(vk::CommandPoolCreateFlags::TRANSIENT);
            let pool = device.create_command_pool(&pool_info, None).unwrap();
            let allocate_info = vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command_buffer = device.allocate_command_buffers(&allocate_info).unwrap()[0];
            let begin = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            device.begin_command_buffer(command_buffer, &begin).unwrap();
            record(command_buffer);
            device.end_command_buffer(command_buffer).unwrap();

            let fence = device
                .create_fence(&vk::FenceCreateInfo::default(), None)
                .unwrap();
            let command_buffers =
                [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
            let submits = [vk::SubmitInfo2::default().command_buffer_infos(&command_buffers)];
            device.queue_submit2(self.queue, &submits, fence).unwrap();
            assert_eq!(
                device.wait_for_fences(&[fence], true, u64::MAX),
                Ok(vk::VkResult::SUCCESS)
            );
            device.destroy_fence(fence, None);
            device.destroy_command_pool(pool, None);
        }
    }
}

impl Drop for Gpu {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.device.destroy_device(None);
            if !self.messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messenger, None);
            }
            self.instance.destroy_instance(None);
            let errors = Box::from_raw(self.errors).into_inner().unwrap();
            if !errors.is_empty() && !std::thread::panicking() {
                panic!("validation errors:\n{}", errors.join("\n"));
            }
        }
    }
}

#[test]
fn queries_fill_p_next_chains() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    unsafe {
        let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
        let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default();
        let mut features = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut vulkan12)
            .push_next(&mut vulkan13);
        gpu.instance
            .get_physical_device_features2(gpu.physical_device, &mut features);
        // Both are required by Vulkan 1.3, so a chain that Vulkan could not follow would
        // leave them at zero.
        assert_eq!(vulkan12.timeline_semaphore, vk::TRUE);
        assert_eq!(vulkan13.dynamic_rendering, vk::TRUE);
        assert_eq!(vulkan13.synchronization2, vk::TRUE);

        let mut id = vk::PhysicalDeviceIDProperties::default();
        let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut id);
        gpu.instance
            .get_physical_device_properties2(gpu.physical_device, &mut properties);
        let core = properties.properties;
        assert!(core.api_version >= vk::API_VERSION_1_3);
        assert!(!core.device_name_as_c_str().unwrap().is_empty());
        assert_ne!(id.device_uuid, [0; vk::UUID_SIZE]);

        let families = gpu
            .instance
            .get_physical_device_queue_family_properties(gpu.physical_device);
        assert!(families[gpu.queue_family as usize].queue_count >= 1);
        assert!(!gpu.queue.is_null());
    }
}

#[test]
fn dynamic_rendering_clear_reads_back() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    const SIZE: u32 = 16;
    unsafe {
        let device = &gpu.device;
        let format = vk::Format::R8G8B8A8_UNORM;
        let extent = vk::Extent3D {
            width: SIZE,
            height: SIZE,
            depth: 1,
        };

        let image_info = vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(format)
            .extent(extent)
            .usage(vk::ImageUsageFlags::COLOR_ATTACHMENT | vk::ImageUsageFlags::TRANSFER_SRC);
        let image = device.create_image(&image_info, None).unwrap();
        let requirements = device.get_image_memory_requirements(image);
        let image_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_image_memory(image, image_memory, 0).unwrap();

        let range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .level_count(1)
            .layer_count(1);
        let view = device
            .create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::_2D)
                    .format(format)
                    .subresource_range(range),
                None,
            )
            .unwrap();

        let byte_count = u64::from(SIZE * SIZE * 4);
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(byte_count)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();
        let requirements = device.get_buffer_memory_requirements(buffer);
        let buffer_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::HOST_VISIBLE
                                | vk::MemoryPropertyFlags::HOST_COHERENT,
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

        gpu.submit(|command_buffer| {
            let to_attachment = [vk::ImageMemoryBarrier2::default()
                .dst_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .old_layout(vk::ImageLayout::UNDEFINED)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(range)];
            device.cmd_pipeline_barrier2(
                command_buffer,
                &vk::DependencyInfo::default().image_memory_barriers(&to_attachment),
            );

            // A clear on load is enough to produce known pixels without a shader.
            let attachments = [vk::RenderingAttachmentInfo::default()
                .image_view(view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(vk::ClearValue::color(vk::ClearColorValue::float32([
                    1.0, 0.0, 0.0, 1.0,
                ])))];
            let rendering = vk::RenderingInfo::default()
                .render_area(vk::Rect2D {
                    offset: vk::Offset2D::default(),
                    extent: vk::Extent2D {
                        width: SIZE,
                        height: SIZE,
                    },
                })
                .layer_count(1)
                .color_attachments(&attachments);
            device.cmd_begin_rendering(command_buffer, &rendering);
            device.cmd_end_rendering(command_buffer);

            let to_transfer = [vk::ImageMemoryBarrier2::default()
                .src_stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags2::COPY)
                .dst_access_mask(vk::AccessFlags2::TRANSFER_READ)
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::TRANSFER_SRC_OPTIMAL)
                .image(image)
                .subresource_range(range)];
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
                .image_extent(extent)];
            device.cmd_copy_image_to_buffer(
                command_buffer,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer,
                &regions,
            );
        });

        let mapped = device
            .map_memory(buffer_memory, 0, byte_count, vk::MemoryMapFlags::empty())
            .unwrap();
        let pixels = std::slice::from_raw_parts(mapped.cast::<u8>(), byte_count as usize);
        assert!(
            pixels
                .chunks_exact(4)
                .all(|pixel| pixel == [255, 0, 0, 255]),
            "first pixel is {:?}",
            &pixels[..4]
        );
        device.unmap_memory(buffer_memory);

        device.destroy_buffer(buffer, None);
        device.free_memory(buffer_memory, None);
        device.destroy_image_view(view, None);
        device.destroy_image(image, None);
        device.free_memory(image_memory, None);
    }
}

#[test]
fn timeline_semaphore_waits_report_timeout_as_success_code() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    unsafe {
        let device = &gpu.device;
        let mut timeline = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(1);
        let semaphore = device
            .create_semaphore(
                &vk::SemaphoreCreateInfo::default().push_next(&mut timeline),
                None,
            )
            .unwrap();
        assert_eq!(device.get_semaphore_counter_value(semaphore), Ok(1));

        device
            .signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(semaphore)
                    .value(5),
            )
            .unwrap();
        assert_eq!(device.get_semaphore_counter_value(semaphore), Ok(5));

        let semaphores = [semaphore];
        let reached = [5];
        let wait = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&reached);
        assert_eq!(device.wait_semaphores(&wait, 0), Ok(vk::VkResult::SUCCESS));

        // VK_TIMEOUT is a success code: it comes back in `Ok`, not as an error.
        let not_reached = [6];
        let wait = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&not_reached);
        assert_eq!(device.wait_semaphores(&wait, 0), Ok(vk::VkResult::TIMEOUT));

        device.destroy_semaphore(semaphore, None);
    }
}

#[test]
fn object_names_use_handle_object_types() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    // Object naming needs VK_EXT_debug_utils, which `Gpu` enables with validation.
    if gpu.messenger.is_null() {
        eprintln!("skipping: VK_EXT_debug_utils is not enabled");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let buffer = device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(64)
                    .usage(vk::BufferUsageFlags::TRANSFER_DST),
                None,
            )
            .unwrap();
        let name = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_type(vk::Buffer::TYPE)
            .object_handle(buffer.as_raw_u64())
            .object_name(c"named buffer");
        device.set_debug_utils_object_name_ext(&name).unwrap();
        device.destroy_buffer(buffer, None);
    }
}

/// Memory exported as a file descriptor and imported again is the same memory. This is
/// the mechanism behind dma-buf sharing between a compositor and its clients.
#[test]
fn memory_round_trips_through_a_file_descriptor() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_MEMORY_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_MEMORY_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_memory_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalMemoryHandleTypeFlagBits::OPAQUE_FD;
        let usage = vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST;

        let external_info = vk::PhysicalDeviceExternalBufferInfo::default()
            .usage(usage)
            .handle_type(handle_type);
        let mut external_properties = vk::ExternalBufferProperties::default();
        gpu.instance.get_physical_device_external_buffer_properties(
            gpu.physical_device,
            &external_info,
            &mut external_properties,
        );
        let features = external_properties
            .external_memory_properties
            .external_memory_features;
        if !features.contains(
            vk::ExternalMemoryFeatureFlags::EXPORTABLE | vk::ExternalMemoryFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: opaque fd buffers are not exportable and importable");
            return;
        }

        let create_buffer = || {
            let mut external =
                vk::ExternalMemoryBufferCreateInfo::default().handle_types(handle_type.into());
            let info = vk::BufferCreateInfo::default()
                .size(4096)
                .usage(usage)
                .push_next(&mut external);
            device.create_buffer(&info, None).unwrap()
        };

        let exported_buffer = create_buffer();
        let requirements = device.get_buffer_memory_requirements(exported_buffer);
        let memory_type = gpu
            .memory_type(
                requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            )
            .unwrap();
        let mut export = vk::ExportMemoryAllocateInfo::default().handle_types(handle_type.into());
        let exported_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type)
                    .push_next(&mut export),
                None,
            )
            .unwrap();
        device
            .bind_buffer_memory(exported_buffer, exported_memory, 0)
            .unwrap();

        let fd = device
            .get_memory_fd_khr(
                &vk::MemoryGetFdInfoKHR::default()
                    .memory(exported_memory)
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= 0);

        // A successful import takes ownership of the file descriptor.
        let imported_buffer = create_buffer();
        let mut import = vk::ImportMemoryFdInfoKHR::default()
            .handle_type(handle_type)
            .fd(fd);
        let imported_memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(memory_type)
                    .push_next(&mut import),
                None,
            )
            .unwrap();
        device
            .bind_buffer_memory(imported_buffer, imported_memory, 0)
            .unwrap();

        let written = device
            .map_memory(exported_memory, 0, 4096, vk::MemoryMapFlags::empty())
            .unwrap()
            .cast::<u8>();
        let read = device
            .map_memory(imported_memory, 0, 4096, vk::MemoryMapFlags::empty())
            .unwrap()
            .cast::<u8>();
        for i in 0..4096 {
            written.add(i).write((i % 251) as u8);
        }
        for i in 0..4096 {
            assert_eq!(read.add(i).read(), (i % 251) as u8, "byte {i}");
        }
        device.unmap_memory(exported_memory);
        device.unmap_memory(imported_memory);

        device.destroy_buffer(imported_buffer, None);
        device.free_memory(imported_memory, None);
        device.destroy_buffer(exported_buffer, None);
        device.free_memory(exported_memory, None);
    }
}

/// A fence payload exported as a sync file and imported into another fence: the hand-off
/// a compositor does with explicit sync.
#[test]
fn fence_round_trips_through_a_sync_file() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_FENCE_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_FENCE_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_fence_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalFenceHandleTypeFlagBits::SYNC_FD;

        let mut properties = vk::ExternalFenceProperties::default();
        gpu.instance.get_physical_device_external_fence_properties(
            gpu.physical_device,
            &vk::PhysicalDeviceExternalFenceInfo::default().handle_type(handle_type),
            &mut properties,
        );
        if !properties.external_fence_features.contains(
            vk::ExternalFenceFeatureFlags::EXPORTABLE | vk::ExternalFenceFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: sync fd fences are not exportable and importable");
            return;
        }

        let mut export = vk::ExportFenceCreateInfo::default().handle_types(handle_type.into());
        let source = device
            .create_fence(&vk::FenceCreateInfo::default().push_next(&mut export), None)
            .unwrap();

        // Give the fence a payload by submitting an empty batch with it.
        device.queue_submit2(gpu.queue, &[], source).unwrap();
        let fd = device
            .get_fence_fd_khr(
                &vk::FenceGetFdInfoKHR::default()
                    .fence(source)
                    .handle_type(handle_type),
            )
            .unwrap();
        // -1 is the valid sync fd for "already signaled".
        assert!(fd >= -1);

        // A temporary import: the payload is consumed by the next wait or reset.
        let target = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .unwrap();
        device
            .import_fence_fd_khr(
                &vk::ImportFenceFdInfoKHR::default()
                    .fence(target)
                    .flags(vk::FenceImportFlags::TEMPORARY)
                    .handle_type(handle_type)
                    .fd(fd),
            )
            .unwrap();
        assert_eq!(
            device.wait_for_fences(&[target], true, u64::MAX),
            Ok(vk::VkResult::SUCCESS)
        );

        device.destroy_fence(target, None);
        device.destroy_fence(source, None);
    }
}

/// The two-call idiom inside a `pNext` struct: ask for the count, then for the data.
#[test]
fn drm_format_modifier_list_is_filled_through_p_next() {
    let Some(gpu) = Gpu::new(&[vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME]) else {
        return;
    };
    if !gpu.has(vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME) {
        eprintln!("skipping: VK_EXT_image_drm_format_modifier is not supported");
        return;
    }
    unsafe {
        let format = vk::Format::B8G8R8A8_UNORM;

        let mut list = vk::DrmFormatModifierPropertiesListEXT::default();
        let mut properties = vk::FormatProperties2::default().push_next(&mut list);
        gpu.instance.get_physical_device_format_properties2(
            gpu.physical_device,
            format,
            &mut properties,
        );
        let count = list.drm_format_modifier_count as usize;
        assert!(count > 0, "no DRM format modifier for {format:?}");

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
        assert_eq!(written, count);
        for modifier in &modifiers[..written] {
            assert!(modifier.drm_format_modifier_plane_count >= 1);
            assert!(!modifier.drm_format_modifier_tiling_features.is_empty());
        }
    }
}

/// A binary semaphore's pending signal exported as a sync file and imported into
/// another semaphore: how a compositor waits for a client's rendering, and the reverse.
#[test]
fn semaphore_round_trips_through_a_sync_file() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_SEMAPHORE_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_SEMAPHORE_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_semaphore_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalSemaphoreHandleTypeFlagBits::SYNC_FD;

        let mut properties = vk::ExternalSemaphoreProperties::default();
        gpu.instance
            .get_physical_device_external_semaphore_properties(
                gpu.physical_device,
                &vk::PhysicalDeviceExternalSemaphoreInfo::default().handle_type(handle_type),
                &mut properties,
            );
        if !properties.external_semaphore_features.contains(
            vk::ExternalSemaphoreFeatureFlags::EXPORTABLE
                | vk::ExternalSemaphoreFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: sync fd semaphores are not exportable and importable");
            return;
        }

        let mut export = vk::ExportSemaphoreCreateInfo::default().handle_types(handle_type.into());
        let source = device
            .create_semaphore(
                &vk::SemaphoreCreateInfo::default().push_next(&mut export),
                None,
            )
            .unwrap();

        // A sync fd can only be exported while a signal operation is pending.
        let signals = [vk::SemaphoreSubmitInfo::default().semaphore(source)];
        let submits = [vk::SubmitInfo2::default().signal_semaphore_infos(&signals)];
        device
            .queue_submit2(gpu.queue, &submits, vk::Fence::null())
            .unwrap();
        let fd = device
            .get_semaphore_fd_khr(
                &vk::SemaphoreGetFdInfoKHR::default()
                    .semaphore(source)
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= -1);

        let target = device
            .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
            .unwrap();
        device
            .import_semaphore_fd_khr(
                &vk::ImportSemaphoreFdInfoKHR::default()
                    .semaphore(target)
                    .flags(vk::SemaphoreImportFlags::TEMPORARY)
                    .handle_type(handle_type)
                    .fd(fd),
            )
            .unwrap();

        // Waiting on the imported payload completes because the source was signaled.
        let fence = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .unwrap();
        let waits = [vk::SemaphoreSubmitInfo::default()
            .semaphore(target)
            .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
        let submits = [vk::SubmitInfo2::default().wait_semaphore_infos(&waits)];
        device.queue_submit2(gpu.queue, &submits, fence).unwrap();
        assert_eq!(
            device.wait_for_fences(&[fence], true, u64::MAX),
            Ok(vk::VkResult::SUCCESS)
        );

        device.destroy_fence(fence, None);
        device.destroy_semaphore(target, None);
        device.destroy_semaphore(source, None);
    }
}

/// A Y'CbCr conversion has to be chained into both the sampler and the image view.
#[test]
fn ycbcr_conversion_chains_into_sampler_and_view() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    if !gpu.ycbcr {
        eprintln!("skipping: samplerYcbcrConversion is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        // NV12, the usual video and camera layout.
        let format = vk::Format::G8_B8R8_2PLANE_420_UNORM;
        let features = gpu
            .instance
            .get_physical_device_format_properties(gpu.physical_device, format)
            .optimal_tiling_features;
        // A format supports midpoint chroma siting, cosited chroma siting, or both.
        let chroma_offset = if features.contains(vk::FormatFeatureFlags::MIDPOINT_CHROMA_SAMPLES) {
            vk::ChromaLocation::MIDPOINT
        } else if features.contains(vk::FormatFeatureFlags::COSITED_CHROMA_SAMPLES) {
            vk::ChromaLocation::COSITED_EVEN
        } else {
            eprintln!("skipping: {format:?} has no Y'CbCr conversion support");
            return;
        };
        if !features.contains(vk::FormatFeatureFlags::SAMPLED_IMAGE) {
            eprintln!("skipping: {format:?} cannot be sampled");
            return;
        }

        let conversion = device
            .create_sampler_ycbcr_conversion(
                &vk::SamplerYcbcrConversionCreateInfo::default()
                    .format(format)
                    .ycbcr_model(vk::SamplerYcbcrModelConversion::YCBCR_709)
                    .ycbcr_range(vk::SamplerYcbcrRange::ITU_NARROW)
                    .x_chroma_offset(chroma_offset)
                    .y_chroma_offset(chroma_offset)
                    .chroma_filter(vk::Filter::NEAREST),
                None,
            )
            .unwrap();

        let mut sampler_conversion =
            vk::SamplerYcbcrConversionInfo::default().conversion(conversion);
        let sampler = device
            .create_sampler(
                &vk::SamplerCreateInfo::default()
                    .address_mode_u(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_v(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_EDGE)
                    .push_next(&mut sampler_conversion),
                None,
            )
            .unwrap();

        let image = device
            .create_image(
                &vk::ImageCreateInfo::default()
                    .image_type(vk::ImageType::_2D)
                    .format(format)
                    .extent(vk::Extent3D {
                        width: 64,
                        height: 64,
                        depth: 1,
                    })
                    .usage(vk::ImageUsageFlags::SAMPLED),
                None,
            )
            .unwrap();
        let requirements = device.get_image_memory_requirements(image);
        let memory = device
            .allocate_memory(
                &vk::MemoryAllocateInfo::default()
                    .allocation_size(requirements.size)
                    .memory_type_index(
                        gpu.memory_type(
                            requirements.memory_type_bits,
                            vk::MemoryPropertyFlags::empty(),
                        )
                        .unwrap(),
                    ),
                None,
            )
            .unwrap();
        device.bind_image_memory(image, memory, 0).unwrap();

        let mut view_conversion = vk::SamplerYcbcrConversionInfo::default().conversion(conversion);
        let view = device
            .create_image_view(
                &vk::ImageViewCreateInfo::default()
                    .image(image)
                    .view_type(vk::ImageViewType::_2D)
                    .format(format)
                    .subresource_range(
                        vk::ImageSubresourceRange::default()
                            .aspect_mask(vk::ImageAspectFlags::COLOR)
                            .level_count(1)
                            .layer_count(1),
                    )
                    .push_next(&mut view_conversion),
                None,
            )
            .unwrap();

        device.destroy_image_view(view, None);
        device.destroy_image(image, None);
        device.free_memory(memory, None);
        device.destroy_sampler(sampler, None);
        device.destroy_sampler_ycbcr_conversion(conversion, None);
    }
}

/// `VK_KHR_display` enumeration works without owning the display. Acquiring one
/// (`vkAcquireDrmDisplayEXT`) needs DRM master and is not attempted here.
#[test]
fn displays_can_be_enumerated() {
    let Some(gpu) =
        Gpu::with_instance_extensions(&[vk::KHR_SURFACE_NAME, vk::KHR_DISPLAY_NAME], &[])
    else {
        return;
    };
    if !gpu.instance_extensions.contains(&vk::KHR_DISPLAY_NAME) {
        eprintln!("skipping: VK_KHR_display is not supported");
        return;
    }
    unsafe {
        let displays = gpu
            .instance
            .get_physical_device_display_properties_khr(gpu.physical_device)
            .unwrap();
        for display in &displays {
            assert!(!display.display.is_null());
            let modes = gpu
                .instance
                .get_display_mode_properties_khr(gpu.physical_device, display.display)
                .unwrap();
            let name = if display.display_name.is_null() {
                c"?"
            } else {
                CStr::from_ptr(display.display_name)
            };
            eprintln!(
                "display {name:?}: {}x{}, {} mode(s)",
                display.physical_resolution.width,
                display.physical_resolution.height,
                modes.len()
            );
        }
        let planes = gpu
            .instance
            .get_physical_device_display_plane_properties_khr(gpu.physical_device)
            .unwrap();
        eprintln!("{} display(s), {} plane(s)", displays.len(), planes.len());
    }
}
