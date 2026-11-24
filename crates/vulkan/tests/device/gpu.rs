use std::ffi::{CStr, c_void};
use std::sync::{Mutex, MutexGuard, PoisonError};
use vulkan::vk;
use vulkan::{Device, Entry, Instance};

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

static LOADER_INSTANCE_AND_DEVICE_LISTS: Mutex<()> = Mutex::new(());

pub fn exclusive_access_to_the_loaders_instance_and_device_lists() -> MutexGuard<'static, ()> {
    LOADER_INSTANCE_AND_DEVICE_LISTS
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

pub struct Gpu {
    pub instance: Instance,
    messenger: vk::DebugUtilsMessengerEXT,
    errors: *mut Mutex<Vec<String>>,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub queue: vk::Queue,
    pub queue_family: u32,
    extensions: Vec<&'static CStr>,
    pub instance_extensions: Vec<&'static CStr>,
    pub ycbcr: bool,
    pub descriptor_heap: bool,
}

unsafe extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    user_data: *mut c_void,
) -> vk::Bool32 {
    unsafe {
        let errors = &*user_data.cast::<Mutex<Vec<String>>>();
        let message = CStr::from_ptr((*data).p_message).to_string_lossy();
        errors.lock().unwrap().push(message.into_owned());
    }
    vk::FALSE
}

impl Gpu {
    pub fn new(wanted: &[&'static CStr]) -> Option<Self> {
        Self::with_instance_extensions(&[], wanted)
    }

    pub fn with_instance_extensions(
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
            let InstanceSetup {
                instance,
                validation,
                extensions: instance_extensions,
            } = create_instance(&entry, instance_wanted)?;

            let errors = Box::into_raw(Box::new(Mutex::new(Vec::new())));
            let messenger = if validation {
                create_error_collector(&instance, errors)?
            } else {
                vk::DebugUtilsMessengerEXT::null()
            };

            let Some((physical_device, queue_family)) = pick_physical_device(&instance)? else {
                if !messenger.is_null() {
                    instance.destroy_debug_utils_messenger_ext(messenger, None);
                }
                instance.destroy_instance(None);
                drop(Box::from_raw(errors));
                return Err("no Vulkan 1.3 device with a graphics queue".to_string());
            };

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
            let features = OptionalFeatures::query(&instance, physical_device, &extensions);
            let device = create_device(
                &instance,
                physical_device,
                queue_family,
                &extensions,
                features,
            )?;
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
                instance_extensions,
                ycbcr: features.ycbcr,
                descriptor_heap: features.descriptor_heap,
            })
        }
    }

    pub fn has_debug_utils(&self) -> bool {
        !self.messenger.is_null()
    }

    pub fn has(&self, extension: &CStr) -> bool {
        self.extensions.contains(&extension)
    }

    pub fn memory_type(&self, type_bits: u32, flags: vk::MemoryPropertyFlags) -> Option<u32> {
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

    pub unsafe fn buffer(
        &self,
        size: u64,
        usage: vk::BufferUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
        address: bool,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        unsafe {
            let device = &self.device;
            let buffer = device
                .create_buffer(
                    &vk::BufferCreateInfo::default().size(size).usage(usage),
                    None,
                )
                .unwrap();
            let requirements = device.get_buffer_memory_requirements(buffer);
            let mut flags_info = vk::MemoryAllocateFlagsInfo::default()
                .flags(vk::MemoryAllocateFlags::DEVICE_ADDRESS);
            let mut info = vk::MemoryAllocateInfo::default()
                .allocation_size(requirements.size)
                .memory_type_index(
                    self.memory_type(requirements.memory_type_bits, memory_flags)
                        .unwrap(),
                );
            if address {
                info = info.push_next(&mut flags_info);
            }
            let memory = device.allocate_memory(&info, None).unwrap();
            device.bind_buffer_memory(buffer, memory, 0).unwrap();
            (buffer, memory)
        }
    }

    pub unsafe fn host_buffer(
        &self,
        bytes: &[u8],
        usage: vk::BufferUsageFlags,
        address: bool,
    ) -> (vk::Buffer, vk::DeviceMemory) {
        unsafe {
            let (buffer, memory) = self.buffer(
                bytes.len() as u64,
                usage,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                address,
            );
            let mapped = self
                .device
                .map_memory(memory, 0, bytes.len() as u64, vk::MemoryMapFlags::empty())
                .unwrap();
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), mapped.cast::<u8>(), bytes.len());
            self.device.unmap_memory(memory);
            (buffer, memory)
        }
    }

    pub unsafe fn image(
        &self,
        format: vk::Format,
        width: u32,
        height: u32,
        usage: vk::ImageUsageFlags,
    ) -> (vk::Image, vk::DeviceMemory) {
        unsafe {
            let device = &self.device;
            let image = device
                .create_image(
                    &vk::ImageCreateInfo::default()
                        .image_type(vk::ImageType::_2D)
                        .format(format)
                        .extent(vk::Extent3D {
                            width,
                            height,
                            depth: 1,
                        })
                        .usage(usage),
                    None,
                )
                .unwrap();
            let requirements = device.get_image_memory_requirements(image);
            let memory = device
                .allocate_memory(
                    &vk::MemoryAllocateInfo::default()
                        .allocation_size(requirements.size)
                        .memory_type_index(
                            self.memory_type(
                                requirements.memory_type_bits,
                                vk::MemoryPropertyFlags::DEVICE_LOCAL,
                            )
                            .unwrap(),
                        ),
                    None,
                )
                .unwrap();
            device.bind_image_memory(image, memory, 0).unwrap();
            (image, memory)
        }
    }

    pub unsafe fn submit(&self, record: impl FnOnce(vk::CommandBuffer)) {
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
            let serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
            self.device.destroy_device(None);
            drop(serialized);
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

struct InstanceSetup {
    instance: Instance,
    validation: bool,
    extensions: Vec<&'static CStr>,
}

unsafe fn create_instance(
    entry: &Entry,
    wanted: &[&'static CStr],
) -> Result<InstanceSetup, String> {
    unsafe {
        let layers = entry
            .enumerate_instance_layer_properties()
            .map_err(|e| e.to_string())?;
        let has_validation = layers
            .iter()
            .any(|layer| layer.layer_name_as_c_str() == Ok(VALIDATION_LAYER));
        let available = entry
            .enumerate_instance_extension_properties(None)
            .map_err(|e| e.to_string())?;
        let is_available = |name: &CStr| {
            available
                .iter()
                .any(|e| e.extension_name_as_c_str() == Ok(name))
        };
        let validation = has_validation && is_available(vk::EXT_DEBUG_UTILS_NAME);

        let mut enabled_layers = Vec::new();
        let mut enabled_extensions = Vec::new();
        if validation {
            enabled_layers.push(vk::CStrPtr::new(VALIDATION_LAYER));
            enabled_extensions.push(vk::CStrPtr::new(vk::EXT_DEBUG_UTILS_NAME));
        }
        let extensions = wanted
            .iter()
            .copied()
            .filter(|name| is_available(name))
            .collect::<Vec<_>>();
        enabled_extensions.extend(extensions.iter().map(|name| vk::CStrPtr::new(name)));

        let app = vk::ApplicationInfo::default()
            .application_name(c"vulkan-rs device test")
            .api_version(vk::API_VERSION_1_3);
        let instance_info = vk::InstanceCreateInfo::default()
            .application_info(&app)
            .enabled_layer_names(&enabled_layers)
            .enabled_extension_names(&enabled_extensions);
        let serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
        let instance = entry
            .create_instance(&instance_info, None)
            .map_err(|e| format!("vkCreateInstance: {e}"))?;
        drop(serialized);
        Ok(InstanceSetup {
            instance,
            validation,
            extensions,
        })
    }
}

unsafe fn create_error_collector(
    instance: &Instance,
    errors: *mut Mutex<Vec<String>>,
) -> Result<vk::DebugUtilsMessengerEXT, String> {
    let info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
        )
        .pfn_user_callback(Some(debug_callback))
        .user_data(errors.cast());
    unsafe { instance.create_debug_utils_messenger_ext(&info, None) }.map_err(|e| e.to_string())
}

unsafe fn pick_physical_device(
    instance: &Instance,
) -> Result<Option<(vk::PhysicalDevice, u32)>, String> {
    unsafe {
        for physical_device in instance
            .enumerate_physical_devices()
            .map_err(|e| e.to_string())?
        {
            let properties = instance.get_physical_device_properties(physical_device);
            if properties.api_version < vk::API_VERSION_1_3 {
                continue;
            }
            let families = instance.get_physical_device_queue_family_properties(physical_device);
            if let Some(family) = families
                .iter()
                .position(|f| f.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            {
                eprintln!(
                    "device test on {:?}",
                    properties.device_name_as_c_str().unwrap_or(c"?")
                );
                return Ok(Some((physical_device, family as u32)));
            }
        }
        Ok(None)
    }
}

#[derive(Copy, Clone)]
struct OptionalFeatures {
    ycbcr: bool,
    draw_parameters: bool,
    descriptor_heap: bool,
}

impl OptionalFeatures {
    unsafe fn query(
        instance: &Instance,
        physical_device: vk::PhysicalDevice,
        extensions: &[&'static CStr],
    ) -> Self {
        unsafe {
            let mut supported11 = vk::PhysicalDeviceVulkan11Features::default();
            let mut supported = vk::PhysicalDeviceFeatures2::default().push_next(&mut supported11);
            instance.get_physical_device_features2(physical_device, &mut supported);
            let ycbcr = supported11.sampler_ycbcr_conversion != vk::FALSE;
            let draw_parameters = supported11.shader_draw_parameters != vk::FALSE;

            let has_heap_extensions = extensions.contains(&vk::EXT_DESCRIPTOR_HEAP_NAME)
                && extensions.contains(&vk::KHR_SHADER_UNTYPED_POINTERS_NAME)
                && extensions.contains(&vk::KHR_MAINTENANCE_5_NAME);
            let mut descriptor_heap = false;
            if has_heap_extensions {
                let mut heap = vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default();
                let mut untyped = vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default();
                let mut supported12 = vk::PhysicalDeviceVulkan12Features::default();
                let mut supported = vk::PhysicalDeviceFeatures2::default()
                    .push_next(&mut heap)
                    .push_next(&mut untyped)
                    .push_next(&mut supported12);
                instance.get_physical_device_features2(physical_device, &mut supported);
                descriptor_heap = heap.descriptor_heap != vk::FALSE
                    && untyped.shader_untyped_pointers != vk::FALSE
                    && supported12.buffer_device_address != vk::FALSE
                    && draw_parameters;
            }
            Self {
                ycbcr,
                draw_parameters,
                descriptor_heap,
            }
        }
    }
}

unsafe fn create_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    extensions: &[&'static CStr],
    features: OptionalFeatures,
) -> Result<Device, String> {
    let extension_names = extensions
        .iter()
        .map(|name| vk::CStrPtr::new(name))
        .collect::<Vec<_>>();
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let mut vulkan11 = vk::PhysicalDeviceVulkan11Features::default()
        .sampler_ycbcr_conversion(features.ycbcr)
        .shader_draw_parameters(features.draw_parameters);
    let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default()
        .timeline_semaphore(true)
        .buffer_device_address(features.descriptor_heap);
    let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default()
        .dynamic_rendering(true)
        .synchronization2(true);
    let mut heap_features =
        vk::PhysicalDeviceDescriptorHeapFeaturesEXT::default().descriptor_heap(true);
    let mut untyped_features =
        vk::PhysicalDeviceShaderUntypedPointersFeaturesKHR::default().shader_untyped_pointers(true);
    let mut device_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_extension_names(&extension_names)
        .push_next(&mut vulkan11)
        .push_next(&mut vulkan12)
        .push_next(&mut vulkan13);
    if features.descriptor_heap {
        device_info = device_info
            .push_next(&mut heap_features)
            .push_next(&mut untyped_features);
    }
    let _serialized = exclusive_access_to_the_loaders_instance_and_device_lists();
    unsafe { instance.create_device(physical_device, &device_info, None) }
        .map_err(|e| format!("vkCreateDevice: {e}"))
}

pub fn color_range() -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .level_count(1)
        .layer_count(1)
}

pub fn color_layers() -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1)
}
