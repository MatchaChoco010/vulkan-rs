use std::ffi::{CStr, CString, c_void};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use vulkan::vk;
use vulkan::{Device, Entry, Instance};

const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

static VALIDATION_MESSAGES: AtomicU32 = AtomicU32::new(0);
static VALIDATION_ENABLED: AtomicBool = AtomicBool::new(false);

pub const BLEND_DISABLED: vk::PipelineColorBlendAttachmentState =
    vk::PipelineColorBlendAttachmentState {
        blend_enable: vk::FALSE,
        src_color_blend_factor: vk::BlendFactor::ZERO,
        dst_color_blend_factor: vk::BlendFactor::ZERO,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::ZERO,
        dst_alpha_blend_factor: vk::BlendFactor::ZERO,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::from_raw(
            vk::ColorComponentFlags::R.as_raw()
                | vk::ColorComponentFlags::G.as_raw()
                | vk::ColorComponentFlags::B.as_raw()
                | vk::ColorComponentFlags::A.as_raw(),
        ),
    };

pub fn fail(message: &str) -> ! {
    eprintln!("{message}");
    std::process::exit(1);
}

pub trait Check<T> {
    fn check(self, call: &str) -> T;
}

impl<T> Check<T> for vk::Result<T> {
    fn check(self, call: &str) -> T {
        self.unwrap_or_else(|error| fail(&format!("{call} returned {error}")))
    }
}

pub struct OutputImage {
    name: &'static str,
    format: &'static str,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

pub struct Ctx {
    pub instance: Instance,
    pub messenger: vk::DebugUtilsMessengerEXT,
    pub memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub device: Device,
    pub queue: vk::Queue,
    pub command_pool: vk::CommandPool,
    pub shader_dir: PathBuf,
    images: Vec<OutputImage>,
}

#[derive(Clone, Copy)]
pub struct Buffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
    pub size: vk::DeviceSize,
}

#[derive(Clone, Copy)]
pub struct Image {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    pub view: vk::ImageView,
}

pub struct ImageDesc {
    pub info: vk::ImageCreateInfo<'static>,
    pub aspect: vk::ImageAspectFlags,
    pub view_type: vk::ImageViewType,
}

pub struct GraphicsDesc<'a> {
    pub vertex: vk::ShaderModule,
    pub fragment: vk::ShaderModule,
    pub vertex_entry: &'a CStr,
    pub fragment_entry: &'a CStr,
    pub vertex_input: Option<&'a vk::PipelineVertexInputStateCreateInfo<'a>>,
    pub vertex_specialization: Option<&'a vk::SpecializationInfo<'a>>,
    pub fragment_specialization: Option<&'a vk::SpecializationInfo<'a>>,
    pub topology: vk::PrimitiveTopology,
    pub primitive_restart: bool,
    pub rasterization: Option<&'a vk::PipelineRasterizationStateCreateInfo<'a>>,
    pub samples: vk::SampleCountFlagBits,
    pub multisample: Option<&'a vk::PipelineMultisampleStateCreateInfo<'a>>,
    pub depth_stencil: Option<&'a vk::PipelineDepthStencilStateCreateInfo<'a>>,
    pub blend_attachments: &'a [vk::PipelineColorBlendAttachmentState],
    pub dynamic_states: &'a [vk::DynamicState],
    pub layout: vk::PipelineLayout,
    pub render_pass: vk::RenderPass,
    pub subpass: u32,
    pub rendering: Option<&'a mut vk::PipelineRenderingCreateInfo<'a>>,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExtensionFeatures {
    None,
    RayQuery,
    ShaderObject,
}

pub struct Scenario {
    pub name: &'static str,
    pub run: fn(&mut Ctx),
    pub device_extensions: &'static [&'static CStr],
    pub extension_features: ExtensionFeatures,
}

fn require(supported: vk::Bool32, feature: &str) {
    if supported == vk::FALSE {
        fail(&format!("unsupported: the device has no {feature}"));
    }
}

unsafe extern "system" fn debug_callback(
    _severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    _types: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut c_void,
) -> vk::Bool32 {
    let (id, message) = unsafe {
        let id = (*data).p_message_id_name;
        (
            if id.is_null() {
                c""
            } else {
                CStr::from_ptr(id)
            },
            CStr::from_ptr((*data).p_message),
        )
    };
    eprintln!(
        "validation message: [{}] {}",
        id.to_string_lossy(),
        message.to_string_lossy()
    );
    VALIDATION_MESSAGES.fetch_add(1, Ordering::Relaxed);
    vk::FALSE
}

pub fn validation_messages() -> u32 {
    VALIDATION_MESSAGES.load(Ordering::Relaxed)
}

pub fn validation_enabled() -> bool {
    VALIDATION_ENABLED.load(Ordering::Relaxed)
}

fn has_layer(entry: &Entry, name: &CStr) -> bool {
    let layers = unsafe { entry.enumerate_instance_layer_properties() }
        .check("vkEnumerateInstanceLayerProperties");
    layers
        .iter()
        .any(|layer| layer.layer_name_as_c_str() == Ok(name))
}

fn messenger_info() -> vk::DebugUtilsMessengerCreateInfoEXT<'static> {
    vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(debug_callback))
}

impl Ctx {
    pub fn new(shader_dir: &Path, scenario: &Scenario) -> Ctx {
        unsafe {
            let entry = Entry::load().unwrap_or_else(|error| fail(&format!("{error}")));
            let validation = has_layer(&entry, VALIDATION_LAYER);
            VALIDATION_ENABLED.store(validation, Ordering::Relaxed);

            let mut instance_messenger_info = messenger_info();
            let capture_layer = std::env::var("RENDER_COMPARE_CAPTURE_LAYER")
                .ok()
                .filter(|name| !name.is_empty())
                .map(|name| CString::new(name).unwrap_or_else(|_| fail("a layer name with NUL")));
            let mut layers = Vec::new();
            if let Some(name) = &capture_layer {
                if !has_layer(&entry, name) {
                    fail(&format!(
                        "unsupported: the layer {} is not installed",
                        name.to_string_lossy()
                    ));
                }
                layers.push(vk::CStrPtr::new(name));
            }
            if validation {
                layers.push(vk::CStrPtr::new(VALIDATION_LAYER));
            }
            let extensions = [vk::CStrPtr::new(vk::EXT_DEBUG_UTILS_NAME)];
            let app = vk::ApplicationInfo::default()
                .application_name(c"vulkan-rs render comparison")
                .api_version(vk::API_VERSION_1_3);
            let mut instance_info = vk::InstanceCreateInfo::default().application_info(&app);
            if !layers.is_empty() {
                instance_info = instance_info.enabled_layer_names(&layers);
            }
            if validation {
                instance_info = instance_info
                    .push_next(&mut instance_messenger_info)
                    .enabled_extension_names(&extensions);
            }
            let instance = entry
                .create_instance(&instance_info, None)
                .check("vkCreateInstance");

            let messenger = if validation {
                instance
                    .create_debug_utils_messenger_ext(&messenger_info(), None)
                    .check("vkCreateDebugUtilsMessengerEXT")
            } else {
                vk::DebugUtilsMessengerEXT::null()
            };

            let physical_device = instance
                .enumerate_physical_devices()
                .check("vkEnumeratePhysicalDevices")
                .into_iter()
                .find(|&physical_device| {
                    instance
                        .get_physical_device_properties(physical_device)
                        .api_version
                        >= vk::API_VERSION_1_3
                })
                .unwrap_or_else(|| fail("unsupported: no Vulkan 1.3 device"));

            let wanted = vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE;
            let queue_family = instance
                .get_physical_device_queue_family_properties(physical_device)
                .iter()
                .position(|family| family.queue_flags.contains(wanted))
                .unwrap_or_else(|| fail("unsupported: no queue family with graphics and compute"))
                as u32;

            let extensions_found = instance
                .enumerate_device_extension_properties(physical_device, None)
                .check("vkEnumerateDeviceExtensionProperties");
            for wanted in scenario.device_extensions {
                if !extensions_found
                    .iter()
                    .any(|extension| extension.extension_name_as_c_str() == Ok(*wanted))
                {
                    fail(&format!(
                        "unsupported: the device has no {}",
                        wanted.to_string_lossy()
                    ));
                }
            }

            let mut shader_object = vk::PhysicalDeviceShaderObjectFeaturesEXT::default();
            let mut ray_query = vk::PhysicalDeviceRayQueryFeaturesKHR::default();
            let mut acceleration_structure =
                vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default();
            let mut features11 = vk::PhysicalDeviceVulkan11Features::default();
            let mut features12 = vk::PhysicalDeviceVulkan12Features::default();
            let mut features13 = vk::PhysicalDeviceVulkan13Features::default();
            let mut supported = vk::PhysicalDeviceFeatures2::default();
            match scenario.extension_features {
                ExtensionFeatures::None => {}
                ExtensionFeatures::RayQuery => {
                    supported = supported
                        .push_next(&mut ray_query)
                        .push_next(&mut acceleration_structure);
                }
                ExtensionFeatures::ShaderObject => {
                    supported = supported.push_next(&mut shader_object);
                }
            }
            supported = supported
                .push_next(&mut features11)
                .push_next(&mut features12)
                .push_next(&mut features13);
            instance.get_physical_device_features2(physical_device, &mut supported);
            let core = supported.features;
            require(core.occlusion_query_precise, "occlusionQueryPrecise");
            require(core.geometry_shader, "geometryShader");
            require(core.tessellation_shader, "tessellationShader");
            require(core.sample_rate_shading, "sampleRateShading");
            require(core.fragment_stores_and_atomics, "fragmentStoresAndAtomics");
            require(
                core.shader_sampled_image_array_dynamic_indexing,
                "shaderSampledImageArrayDynamicIndexing",
            );
            require(core.multi_draw_indirect, "multiDrawIndirect");
            require(
                core.draw_indirect_first_instance,
                "drawIndirectFirstInstance",
            );
            require(features11.shader_draw_parameters, "shaderDrawParameters");
            require(features12.draw_indirect_count, "drawIndirectCount");
            require(features12.descriptor_indexing, "descriptorIndexing");
            require(
                features12.shader_sampled_image_array_non_uniform_indexing,
                "shaderSampledImageArrayNonUniformIndexing",
            );
            require(
                features12.descriptor_binding_sampled_image_update_after_bind,
                "descriptorBindingSampledImageUpdateAfterBind",
            );
            require(
                features12.descriptor_binding_partially_bound,
                "descriptorBindingPartiallyBound",
            );
            require(
                features12.descriptor_binding_variable_descriptor_count,
                "descriptorBindingVariableDescriptorCount",
            );
            require(
                features12.runtime_descriptor_array,
                "runtimeDescriptorArray",
            );
            require(features12.buffer_device_address, "bufferDeviceAddress");
            require(features13.inline_uniform_block, "inlineUniformBlock");
            require(features13.synchronization2, "synchronization2");
            require(features13.dynamic_rendering, "dynamicRendering");
            match scenario.extension_features {
                ExtensionFeatures::None => {}
                ExtensionFeatures::RayQuery => {
                    require(
                        acceleration_structure.acceleration_structure,
                        "accelerationStructure",
                    );
                    require(ray_query.ray_query, "rayQuery");
                }
                ExtensionFeatures::ShaderObject => {
                    require(shader_object.shader_object, "shaderObject");
                }
            }

            let priorities = [1.0_f32];
            let queue_infos = [vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family)
                .queue_priorities(&priorities)];
            let mut shader_object =
                vk::PhysicalDeviceShaderObjectFeaturesEXT::default().shader_object(true);
            let mut ray_query = vk::PhysicalDeviceRayQueryFeaturesKHR::default().ray_query(true);
            let mut acceleration_structure =
                vk::PhysicalDeviceAccelerationStructureFeaturesKHR::default()
                    .acceleration_structure(true);
            let mut features11 =
                vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(true);
            let mut features12 = vk::PhysicalDeviceVulkan12Features::default()
                .draw_indirect_count(true)
                .descriptor_indexing(true)
                .shader_sampled_image_array_non_uniform_indexing(true)
                .descriptor_binding_sampled_image_update_after_bind(true)
                .descriptor_binding_partially_bound(true)
                .descriptor_binding_variable_descriptor_count(true)
                .runtime_descriptor_array(true)
                .buffer_device_address(true);
            let mut features13 = vk::PhysicalDeviceVulkan13Features::default()
                .inline_uniform_block(true)
                .synchronization2(true)
                .dynamic_rendering(true);
            let mut enabled = vk::PhysicalDeviceFeatures2::default().features(
                vk::PhysicalDeviceFeatures::default()
                    .geometry_shader(true)
                    .tessellation_shader(true)
                    .sample_rate_shading(true)
                    .multi_draw_indirect(true)
                    .draw_indirect_first_instance(true)
                    .fragment_stores_and_atomics(true)
                    .shader_sampled_image_array_dynamic_indexing(true)
                    .occlusion_query_precise(true),
            );
            let extension_names = scenario
                .device_extensions
                .iter()
                .map(|name| vk::CStrPtr::new(name))
                .collect::<Vec<_>>();
            let mut device_info = vk::DeviceCreateInfo::default();
            match scenario.extension_features {
                ExtensionFeatures::None => {}
                ExtensionFeatures::RayQuery => {
                    device_info = device_info
                        .push_next(&mut ray_query)
                        .push_next(&mut acceleration_structure);
                }
                ExtensionFeatures::ShaderObject => {
                    device_info = device_info.push_next(&mut shader_object);
                }
            }
            device_info = device_info
                .push_next(&mut features11)
                .push_next(&mut features12)
                .push_next(&mut features13)
                .push_next(&mut enabled)
                .queue_create_infos(&queue_infos);
            if !extension_names.is_empty() {
                device_info = device_info.enabled_extension_names(&extension_names);
            }
            let device = instance
                .create_device(physical_device, &device_info, None)
                .check("vkCreateDevice");
            let queue = device.get_device_queue(queue_family, 0);
            let memory_properties = instance.get_physical_device_memory_properties(physical_device);

            let pool_info = vk::CommandPoolCreateInfo::default().queue_family_index(queue_family);
            let command_pool = device
                .create_command_pool(&pool_info, None)
                .check("vkCreateCommandPool");

            Ctx {
                instance,
                messenger,
                memory_properties,
                device,
                queue,
                command_pool,
                shader_dir: shader_dir.to_path_buf(),
                images: Vec::new(),
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.device.destroy_command_pool(self.command_pool, None);
            self.device.destroy_device(None);
            if !self.messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.messenger, None);
            }
            self.instance.destroy_instance(None);
        }
    }

    pub fn memory_type(&self, type_bits: u32, flags: vk::MemoryPropertyFlags) -> u32 {
        self.memory_properties
            .memory_types_as_slice()
            .iter()
            .enumerate()
            .position(|(index, ty)| {
                type_bits & (1 << index) != 0 && ty.property_flags.contains(flags)
            })
            .map(|index| index as u32)
            .unwrap_or_else(|| {
                fail(&format!(
                    "no memory type with the flags {flags:?} among {type_bits:#x}"
                ))
            })
    }

    pub fn buffer_create(
        &self,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        flags: vk::MemoryPropertyFlags,
    ) -> Buffer {
        unsafe {
            let info = vk::BufferCreateInfo::default()
                .size(size)
                .usage(usage)
                .sharing_mode(vk::SharingMode::EXCLUSIVE);
            let buffer = self
                .device
                .create_buffer(&info, None)
                .check("vkCreateBuffer");
            let requirements = self.device.get_buffer_memory_requirements(buffer);
            let allocate = vk::MemoryAllocateInfo::default()
                .allocation_size(requirements.size)
                .memory_type_index(self.memory_type(requirements.memory_type_bits, flags));
            let memory = self
                .device
                .allocate_memory(&allocate, None)
                .check("vkAllocateMemory");
            self.device
                .bind_buffer_memory(buffer, memory, 0)
                .check("vkBindBufferMemory");
            Buffer {
                buffer,
                memory,
                size,
            }
        }
    }

    pub fn buffer_write(&self, buffer: &Buffer, data: &[u8]) {
        unsafe {
            let mapped = self
                .device
                .map_memory(
                    buffer.memory,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
                .check("vkMapMemory");
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapped.cast::<u8>(), data.len());
            self.device.unmap_memory(buffer.memory);
        }
    }

    pub fn buffer_destroy(&self, buffer: &Buffer) {
        unsafe {
            self.device.destroy_buffer(buffer.buffer, None);
            self.device.free_memory(buffer.memory, None);
        }
    }

    pub fn buffer_with_data(&self, usage: vk::BufferUsageFlags, data: &[u8]) -> Buffer {
        let buffer = self.buffer_create(
            data.len() as vk::DeviceSize,
            usage,
            vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
        );
        self.buffer_write(&buffer, data);
        buffer
    }

    pub fn image_create(&self, desc: &ImageDesc) -> Image {
        unsafe {
            let image = self
                .device
                .create_image(&desc.info, None)
                .check("vkCreateImage");
            let requirements = self.device.get_image_memory_requirements(image);
            let allocate = vk::MemoryAllocateInfo::default()
                .allocation_size(requirements.size)
                .memory_type_index(self.memory_type(
                    requirements.memory_type_bits,
                    vk::MemoryPropertyFlags::DEVICE_LOCAL,
                ));
            let memory = self
                .device
                .allocate_memory(&allocate, None)
                .check("vkAllocateMemory");
            self.device
                .bind_image_memory(image, memory, 0)
                .check("vkBindImageMemory");
            let view_info = vk::ImageViewCreateInfo::default()
                .image(image)
                .view_type(desc.view_type)
                .format(desc.info.format)
                .subresource_range(
                    subresource_range(desc.aspect, 0, desc.info.mip_levels)
                        .layer_count(desc.info.array_layers),
                );
            let view = self
                .device
                .create_image_view(&view_info, None)
                .check("vkCreateImageView");
            Image {
                image,
                memory,
                view,
            }
        }
    }

    pub fn image_destroy(&self, image: &Image) {
        unsafe {
            self.device.destroy_image_view(image.view, None);
            self.device.destroy_image(image.image, None);
            self.device.free_memory(image.memory, None);
        }
    }

    pub fn commands_begin(&self) -> vk::CommandBuffer {
        unsafe {
            let allocate = vk::CommandBufferAllocateInfo::default()
                .command_pool(self.command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);
            let command_buffer = self
                .device
                .allocate_command_buffers(&allocate)
                .check("vkAllocateCommandBuffers")[0];
            let begin = vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
            self.device
                .begin_command_buffer(command_buffer, &begin)
                .check("vkBeginCommandBuffer");
            command_buffer
        }
    }

    pub fn commands_submit(&self, command_buffer: vk::CommandBuffer) {
        unsafe {
            self.device
                .end_command_buffer(command_buffer)
                .check("vkEndCommandBuffer");
            let fence = self
                .device
                .create_fence(&vk::FenceCreateInfo::default(), None)
                .check("vkCreateFence");
            let command_buffer_infos =
                [vk::CommandBufferSubmitInfo::default().command_buffer(command_buffer)];
            let submits = [vk::SubmitInfo2::default().command_buffer_infos(&command_buffer_infos)];
            self.device
                .queue_submit2(self.queue, &submits, fence)
                .check("vkQueueSubmit2");
            let waited = self
                .device
                .wait_for_fences(&[fence], true, u64::MAX)
                .check("vkWaitForFences");
            if waited != vk::VkResult::SUCCESS {
                fail(&format!("vkWaitForFences returned {waited}"));
            }
            self.device.destroy_fence(fence, None);
            self.device
                .free_command_buffers(self.command_pool, &[command_buffer]);
        }
    }

    pub fn image_upload(
        &self,
        image: vk::Image,
        width: u32,
        height: u32,
        data: &[u8],
        final_layout: vk::ImageLayout,
    ) {
        unsafe {
            let staging = self.buffer_with_data(vk::BufferUsageFlags::TRANSFER_SRC, data);
            let command_buffer = self.commands_begin();
            image_barrier(
                &self.device,
                command_buffer,
                image,
                vk::ImageAspectFlags::COLOR,
                0,
                1,
                vk::ImageLayout::UNDEFINED,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
                (
                    vk::PipelineStageFlags2::COPY,
                    vk::AccessFlags2::TRANSFER_WRITE,
                ),
            );
            let region = vk::BufferImageCopy::default()
                .image_subresource(subresource_layers(vk::ImageAspectFlags::COLOR, 0))
                .image_extent(extent3d(width, height));
            self.device.cmd_copy_buffer_to_image(
                command_buffer,
                staging.buffer,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &[region],
            );
            image_barrier(
                &self.device,
                command_buffer,
                image,
                vk::ImageAspectFlags::COLOR,
                0,
                1,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                final_layout,
                (
                    vk::PipelineStageFlags2::COPY,
                    vk::AccessFlags2::TRANSFER_WRITE,
                ),
                (
                    vk::PipelineStageFlags2::ALL_COMMANDS,
                    vk::AccessFlags2::MEMORY_READ,
                ),
            );
            self.commands_submit(command_buffer);
            self.buffer_destroy(&staging);
        }
    }

    fn output_mapped(
        &mut self,
        buffer: &Buffer,
        size: usize,
        name: &'static str,
        format: &'static str,
        width: u32,
        height: u32,
    ) {
        unsafe {
            let mapped = self
                .device
                .map_memory(
                    buffer.memory,
                    0,
                    vk::WHOLE_SIZE,
                    vk::MemoryMapFlags::empty(),
                )
                .check("vkMapMemory");
            let data = std::slice::from_raw_parts(mapped.cast::<u8>(), size).to_vec();
            self.images.push(OutputImage {
                name,
                format,
                width,
                height,
                data,
            });
            self.device.unmap_memory(buffer.memory);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn readback(
        &mut self,
        image: vk::Image,
        aspect: vk::ImageAspectFlags,
        width: u32,
        height: u32,
        texel_size: u32,
        name: &'static str,
        format: &'static str,
    ) {
        unsafe {
            let size = width as usize * height as usize * texel_size as usize;
            let buffer = self.buffer_create(
                size as vk::DeviceSize,
                vk::BufferUsageFlags::TRANSFER_DST,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            );
            let command_buffer = self.commands_begin();
            let region = vk::BufferImageCopy::default()
                .image_subresource(subresource_layers(aspect, 0))
                .image_extent(extent3d(width, height));
            self.device.cmd_copy_image_to_buffer(
                command_buffer,
                image,
                vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
                buffer.buffer,
                &[region],
            );
            host_read_barrier(
                &self.device,
                command_buffer,
                vk::PipelineStageFlags2::COPY,
                vk::AccessFlags2::TRANSFER_WRITE,
            );
            self.commands_submit(command_buffer);
            self.output_mapped(&buffer, size, name, format, width, height);
            self.buffer_destroy(&buffer);
        }
    }

    pub fn output_buffer(
        &mut self,
        buffer: &Buffer,
        width: u32,
        height: u32,
        name: &'static str,
        format: &'static str,
    ) {
        self.output_mapped(buffer, buffer.size as usize, name, format, width, height);
    }

    pub fn output_words(&mut self, words: &[u32], name: &'static str) {
        self.images.push(OutputImage {
            name,
            format: "r32ui",
            width: words.len() as u32,
            height: 1,
            data: bytes_of(words).to_vec(),
        });
    }

    pub fn output_write(&self, path: &Path) {
        let mut out = Vec::new();
        out.extend_from_slice(b"render-compare 1\n");
        for image in &self.images {
            out.extend_from_slice(
                format!(
                    "image {} {} {} {} {}\n",
                    image.name,
                    image.format,
                    image.width,
                    image.height,
                    image.data.len()
                )
                .as_bytes(),
            );
            out.extend_from_slice(&image.data);
        }
        std::fs::File::create(path)
            .and_then(|mut file| file.write_all(&out))
            .unwrap_or_else(|error| fail(&format!("cannot write {}: {error}", path.display())));
    }

    pub fn shader_code(&self, file_name: &str) -> Vec<u32> {
        let path = self.shader_dir.join(file_name);
        std::fs::read(&path)
            .unwrap_or_else(|error| fail(&format!("cannot read {}: {error}", path.display())))
            .chunks_exact(4)
            .map(|word| u32::from_le_bytes([word[0], word[1], word[2], word[3]]))
            .collect()
    }

    pub fn shader_module(&self, file_name: &str) -> vk::ShaderModule {
        let code = self.shader_code(file_name);
        let info = vk::ShaderModuleCreateInfo::default().code(&code);
        unsafe { self.device.create_shader_module(&info, None) }.check("vkCreateShaderModule")
    }

    pub fn graphics_pipeline(&self, desc: GraphicsDesc<'_>) -> vk::Pipeline {
        unsafe {
            let stages = [
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlagBits::VERTEX)
                    .module(desc.vertex)
                    .name(desc.vertex_entry),
                vk::PipelineShaderStageCreateInfo::default()
                    .stage(vk::ShaderStageFlagBits::FRAGMENT)
                    .module(desc.fragment)
                    .name(desc.fragment_entry),
            ];
            let [mut vertex_stage, mut fragment_stage] = stages;
            if let Some(specialization) = desc.vertex_specialization {
                vertex_stage = vertex_stage.specialization_info(specialization);
            }
            if let Some(specialization) = desc.fragment_specialization {
                fragment_stage = fragment_stage.specialization_info(specialization);
            }
            let stages = [vertex_stage, fragment_stage];
            let no_vertex_input = vk::PipelineVertexInputStateCreateInfo::default();
            let input_assembly = vk::PipelineInputAssemblyStateCreateInfo::default()
                .topology(desc.topology)
                .primitive_restart_enable(desc.primitive_restart);
            let viewports = [vk::Viewport::default()
                .width(desc.width as f32)
                .height(desc.height as f32)
                .max_depth(1.0)];
            let scissors = [vk::Rect2D::default().extent(extent2d(desc.width, desc.height))];
            let viewport_state = vk::PipelineViewportStateCreateInfo::default()
                .viewports(&viewports)
                .scissors(&scissors);
            let rasterization = vk::PipelineRasterizationStateCreateInfo::default()
                .polygon_mode(vk::PolygonMode::FILL)
                .cull_mode(vk::CullModeFlags::NONE)
                .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
                .line_width(1.0);
            let multisample = vk::PipelineMultisampleStateCreateInfo::default()
                .rasterization_samples(desc.samples);
            let mut color_blend = vk::PipelineColorBlendStateCreateInfo::default();
            if !desc.blend_attachments.is_empty() {
                color_blend = color_blend.attachments(desc.blend_attachments);
            }
            let dynamic =
                vk::PipelineDynamicStateCreateInfo::default().dynamic_states(desc.dynamic_states);
            let mut info = vk::GraphicsPipelineCreateInfo::default()
                .stages(&stages)
                .vertex_input_state(desc.vertex_input.unwrap_or(&no_vertex_input))
                .input_assembly_state(&input_assembly)
                .viewport_state(&viewport_state)
                .rasterization_state(desc.rasterization.unwrap_or(&rasterization))
                .multisample_state(desc.multisample.unwrap_or(&multisample))
                .color_blend_state(&color_blend)
                .layout(desc.layout)
                .render_pass(desc.render_pass)
                .subpass(desc.subpass);
            if let Some(depth_stencil) = desc.depth_stencil {
                info = info.depth_stencil_state(depth_stencil);
            }
            if !desc.dynamic_states.is_empty() {
                info = info.dynamic_state(&dynamic);
            }
            if let Some(rendering) = desc.rendering {
                info = info.push_next(rendering);
            }
            match self
                .device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None)
            {
                Ok(pipelines) => pipelines[0],
                Err((_, error)) => fail(&format!("vkCreateGraphicsPipelines returned {error}")),
            }
        }
    }

    pub fn pipeline_layout(
        &self,
        set_layouts: &[vk::DescriptorSetLayout],
        ranges: &[vk::PushConstantRange],
    ) -> vk::PipelineLayout {
        let mut info = vk::PipelineLayoutCreateInfo::default();
        if !set_layouts.is_empty() {
            info = info.set_layouts(set_layouts);
        }
        if !ranges.is_empty() {
            info = info.push_constant_ranges(ranges);
        }
        unsafe { self.device.create_pipeline_layout(&info, None) }.check("vkCreatePipelineLayout")
    }
}

impl<'a> GraphicsDesc<'a> {
    pub fn new(
        vertex: vk::ShaderModule,
        fragment: vk::ShaderModule,
        layout: vk::PipelineLayout,
        width: u32,
        height: u32,
    ) -> GraphicsDesc<'a> {
        GraphicsDesc {
            vertex,
            fragment,
            vertex_entry: c"main",
            fragment_entry: c"main",
            vertex_input: None,
            vertex_specialization: None,
            fragment_specialization: None,
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart: false,
            rasterization: None,
            samples: vk::SampleCountFlagBits::_1,
            multisample: None,
            depth_stencil: None,
            blend_attachments: &[BLEND_DISABLED],
            dynamic_states: &[],
            layout,
            render_pass: vk::RenderPass::null(),
            subpass: 0,
            rendering: None,
            width,
            height,
        }
    }
}

pub fn image_desc(
    format: vk::Format,
    width: u32,
    height: u32,
    usage: vk::ImageUsageFlags,
    aspect: vk::ImageAspectFlags,
) -> ImageDesc {
    ImageDesc {
        info: vk::ImageCreateInfo::default()
            .image_type(vk::ImageType::_2D)
            .format(format)
            .extent(extent3d(width, height))
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlagBits::_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .initial_layout(vk::ImageLayout::UNDEFINED),
        aspect,
        view_type: vk::ImageViewType::_2D,
    }
}

pub fn extent2d(width: u32, height: u32) -> vk::Extent2D {
    vk::Extent2D { width, height }
}

pub fn extent3d(width: u32, height: u32) -> vk::Extent3D {
    vk::Extent3D {
        width,
        height,
        depth: 1,
    }
}

pub fn rect(width: u32, height: u32) -> vk::Rect2D {
    vk::Rect2D::default().extent(extent2d(width, height))
}

pub fn subresource_range(
    aspect: vk::ImageAspectFlags,
    base_mip: u32,
    mip_count: u32,
) -> vk::ImageSubresourceRange {
    vk::ImageSubresourceRange::default()
        .aspect_mask(aspect)
        .base_mip_level(base_mip)
        .level_count(mip_count)
        .base_array_layer(0)
        .layer_count(1)
}

pub fn subresource_layers(aspect: vk::ImageAspectFlags, mip: u32) -> vk::ImageSubresourceLayers {
    vk::ImageSubresourceLayers::default()
        .aspect_mask(aspect)
        .mip_level(mip)
        .base_array_layer(0)
        .layer_count(1)
}

pub fn depth_stencil_clear(depth: f32, stencil: u32) -> vk::ClearValue {
    vk::ClearValue::depth_stencil(vk::ClearDepthStencilValue { depth, stencil })
}

pub fn color_clear(rgba: [f32; 4]) -> vk::ClearValue {
    vk::ClearValue::color(vk::ClearColorValue::float32(rgba))
}

#[allow(clippy::too_many_arguments)]
pub fn image_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
    aspect: vk::ImageAspectFlags,
    base_mip: u32,
    mip_count: u32,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    src: (vk::PipelineStageFlags2, vk::AccessFlags2),
    dst: (vk::PipelineStageFlags2, vk::AccessFlags2),
) {
    let barriers = [vk::ImageMemoryBarrier2::default()
        .src_stage_mask(src.0)
        .src_access_mask(src.1)
        .dst_stage_mask(dst.0)
        .dst_access_mask(dst.1)
        .old_layout(old_layout)
        .new_layout(new_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .image(image)
        .subresource_range(subresource_range(aspect, base_mip, mip_count))];
    let dependency = vk::DependencyInfo::default().image_memory_barriers(&barriers);
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &dependency) };
}

pub fn host_read_barrier(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    src_stage: vk::PipelineStageFlags2,
    src_access: vk::AccessFlags2,
) {
    let barriers = [vk::MemoryBarrier2::default()
        .src_stage_mask(src_stage)
        .src_access_mask(src_access)
        .dst_stage_mask(vk::PipelineStageFlags2::HOST)
        .dst_access_mask(vk::AccessFlags2::HOST_READ)];
    let dependency = vk::DependencyInfo::default().memory_barriers(&barriers);
    unsafe { device.cmd_pipeline_barrier2(command_buffer, &dependency) };
}

pub fn bytes_of<T: Copy>(values: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(values.as_ptr().cast::<u8>(), std::mem::size_of_val(values))
    }
}

pub fn color_attachment_begin(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    image: vk::Image,
) {
    image_barrier(
        device,
        command_buffer,
        image,
        vk::ImageAspectFlags::COLOR,
        0,
        1,
        vk::ImageLayout::UNDEFINED,
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        (vk::PipelineStageFlags2::NONE, vk::AccessFlags2::NONE),
        (
            vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
        ),
    );
}

pub fn color_attachment_end(device: &Device, command_buffer: vk::CommandBuffer, image: vk::Image) {
    image_barrier(
        device,
        command_buffer,
        image,
        vk::ImageAspectFlags::COLOR,
        0,
        1,
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        vk::ImageLayout::TRANSFER_SRC_OPTIMAL,
        (
            vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
        ),
        (
            vk::PipelineStageFlags2::COPY,
            vk::AccessFlags2::TRANSFER_READ,
        ),
    );
}

pub fn rendering_begin(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    view: vk::ImageView,
    clear: [f32; 4],
    width: u32,
    height: u32,
) {
    let color_attachments = [vk::RenderingAttachmentInfo::default()
        .image_view(view)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(color_clear(clear))];
    let rendering_info = vk::RenderingInfo::default()
        .render_area(rect(width, height))
        .layer_count(1)
        .color_attachments(&color_attachments);
    unsafe { device.cmd_begin_rendering(command_buffer, &rendering_info) };
}
