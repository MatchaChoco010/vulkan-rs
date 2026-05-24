#![allow(unsafe_op_in_unsafe_fn)]

use anyhow::{Result, anyhow};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use std::ffi::{CStr, CString, c_void};
use std::mem;
use vulkan::{Device, Entry, Instance, vk};
use vulkan_alloc::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
    MemoryLocation,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::Window;
use winit::window::WindowId;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut runner = Runner {
        app: None,
        window: None,
        frames: 0,
        max_frames: std::env::var("VULKAN_EXAMPLE_FRAMES")
            .ok()
            .and_then(|value| value.parse::<u32>().ok()),
    };
    event_loop.run_app(&mut runner)?;

    Ok(())
}

struct Runner {
    app: Option<App>,
    window: Option<Window>,
    frames: u32,
    max_frames: Option<u32>,
}

impl ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
        if self.window.is_some() {
            return;
        }

        let window = match event_loop.create_window(
            Window::default_attributes()
                .with_title("vulkan wayland triangle")
                .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT)),
        ) {
            Ok(window) => window,
            Err(err) => {
                eprintln!("{err:#}");
                event_loop.exit();
                return;
            }
        };

        let raw_display = match window.display_handle() {
            Ok(handle) => handle.as_raw(),
            Err(err) => {
                eprintln!("{err:#}");
                event_loop.exit();
                return;
            }
        };
        let raw_window = match window.window_handle() {
            Ok(handle) => handle.as_raw(),
            Err(err) => {
                eprintln!("{err:#}");
                event_loop.exit();
                return;
            }
        };
        let (wl_display, wl_surface) = match (raw_display, raw_window) {
            (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
                (display.display.as_ptr(), window.surface.as_ptr())
            }
            _ => {
                eprintln!("this example must run on a Wayland winit backend");
                event_loop.exit();
                return;
            }
        };

        let size = window.inner_size();
        let app = match unsafe {
            App::new(
                wl_display,
                wl_surface,
                size.width.max(1),
                size.height.max(1),
            )
        } {
            Ok(app) => app,
            Err(err) => {
                eprintln!("{err:#}");
                event_loop.exit();
                return;
            }
        };

        self.app = Some(app);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let Some(app) = self.app.as_mut() else {
                    return;
                };
                if let Err(err) =
                    unsafe { app.recreate_swapchain(size.width.max(1), size.height.max(1)) }
                {
                    eprintln!("{err:#}");
                    event_loop.exit();
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(app) = self.app.as_mut() else {
                    return;
                };
                match unsafe { app.draw() } {
                    Ok(true) => {
                        self.frames += 1;
                        if self.max_frames.is_some_and(|max| self.frames >= max) {
                            event_loop.exit();
                        }
                    }
                    Ok(false) => {
                        let Some(window) = self.window.as_ref() else {
                            return;
                        };
                        let size = window.inner_size();
                        if let Err(err) =
                            unsafe { app.recreate_swapchain(size.width.max(1), size.height.max(1)) }
                        {
                            eprintln!("{err:#}");
                            event_loop.exit();
                        }
                    }
                    Err(err) => {
                        eprintln!("{err:#}");
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.app.is_some() {
            let Some(window) = self.window.as_ref() else {
                return;
            };
            window.request_redraw();
        }
    }
}

struct App {
    _entry: Entry,
    instance: Instance,
    debug_messenger: vk::DebugUtilsMessengerEXT,
    physical_device: vk::PhysicalDevice,
    device: Device,
    allocator: Allocator,
    queue: vk::Queue,
    vertex_buffer: vk::Buffer,
    vertex_allocation: Option<Allocation>,
    surface: vk::SurfaceKHR,
    swapchain: vk::SwapchainKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    render_pass: vk::RenderPass,
    framebuffers: Vec<vk::Framebuffer>,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    image_available: vk::Semaphore,
    render_finished: Vec<vk::Semaphore>,
    in_flight: vk::Fence,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    extent: vk::Extent2D<'static>,
}

impl App {
    unsafe fn new(
        wl_display: *mut vk::wl_display,
        wl_surface: *mut vk::wl_surface,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let entry = Entry::load()?;
        let mut instance_extensions = vec![vk::KHR_SURFACE_NAME, vk::KHR_WAYLAND_SURFACE_NAME];
        if has_instance_extension(&entry, vk::EXT_DEBUG_UTILS_NAME)? {
            instance_extensions.push(vk::EXT_DEBUG_UTILS_NAME);
        }
        let debug_utils_enabled = instance_extensions.contains(&vk::EXT_DEBUG_UTILS_NAME);
        let instance = create_instance(&entry, &instance_extensions)?;
        let debug_messenger = create_debug_messenger(&instance, debug_utils_enabled)?;

        let surface_info = vk::WaylandSurfaceCreateInfoKHR::default()
            .display(wl_display)
            .surface(wl_surface);
        let surface = instance.create_wayland_surface_khr(&surface_info, None)?;

        let device_extensions = [vk::KHR_SWAPCHAIN_NAME];
        let selection = pick_physical_device(&instance, surface, &device_extensions)?;
        let (device, queue) = create_device_and_queue(
            &instance,
            selection.physical_device,
            selection.queue_family,
            &device_extensions,
        )?;
        let physical_properties =
            instance.get_physical_device_properties(selection.physical_device);
        let memory_properties =
            instance.get_physical_device_memory_properties(selection.physical_device);
        let allocator_desc =
            AllocatorCreateDesc::new(&device, memory_properties, physical_properties);
        let mut allocator = Allocator::new(allocator_desc);

        let caps = surface_capabilities(&instance, selection.physical_device, surface)?;
        let surface_format = choose_surface_format(&instance, selection.physical_device, surface)?;
        let requested_extent = if caps.current_extent.width != u32::MAX {
            vk::Extent2D::default()
                .width(caps.current_extent.width)
                .height(caps.current_extent.height)
        } else {
            vk::Extent2D::default().width(width).height(height)
        };
        let extent = clamp_extent(caps, requested_extent);

        let swapchain = create_swapchain(&device, surface, caps, surface_format, extent)?;
        let images = swapchain_images(&device, swapchain)?;
        let image_views = images
            .iter()
            .map(|&image| create_image_view(&device, image, surface_format.format))
            .collect::<Result<Vec<_>>>()?;
        let render_pass = create_render_pass(
            &device,
            surface_format.format,
            vk::ImageLayout::PRESENT_SRC_KHR,
        )?;
        let framebuffers = image_views
            .iter()
            .map(|&view| {
                create_framebuffer(&device, render_pass, view, extent.width, extent.height)
            })
            .collect::<Result<Vec<_>>>()?;
        let command_pool = create_command_pool(&device, selection.queue_family)?;
        let vertex = create_vertex_buffer(&device, &mut allocator, command_pool, queue)?;
        let command_buffer = allocate_command_buffer(&device, command_pool)?;
        let image_available = create_semaphore(&device)?;
        let render_finished = (0..images.len())
            .map(|_| create_semaphore(&device))
            .collect::<Result<Vec<_>>>()?;
        let in_flight = create_signaled_fence(&device)?;
        let pipeline_layout = create_pipeline_layout(&device)?;
        let pipeline = create_pipeline(
            &device,
            render_pass,
            pipeline_layout,
            extent.width,
            extent.height,
        )?;

        Ok(Self {
            _entry: entry,
            instance,
            debug_messenger,
            physical_device: selection.physical_device,
            device,
            allocator,
            queue,
            vertex_buffer: vertex.buffer,
            vertex_allocation: Some(vertex.allocation),
            surface,
            swapchain,
            images,
            image_views,
            render_pass,
            framebuffers,
            command_pool,
            command_buffer,
            image_available,
            render_finished,
            in_flight,
            pipeline_layout,
            pipeline,
            extent,
        })
    }

    unsafe fn draw(&mut self) -> Result<bool> {
        self.device
            .wait_for_fences(&[self.in_flight], true, u64::MAX)?;

        let (image_index, acquired_suboptimal) = match self.device.acquire_next_image_khr(
            self.swapchain,
            u64::MAX,
            self.image_available,
            vk::Fence::null(),
        ) {
            Ok(Some(value)) => value,
            Ok(None) => return Ok(true),
            Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => return Ok(false),
            Err(err) => return Err(anyhow!("vkAcquireNextImageKHR failed: {err:?}")),
        };
        let render_finished = self.render_finished[image_index as usize];
        self.device.reset_fences(&[self.in_flight])?;

        self.device
            .reset_command_buffer(self.command_buffer, vk::CommandBufferResetFlags::empty())?;
        {
            let begin = vk::CommandBufferBeginInfo::default();
            self.device
                .begin_command_buffer(self.command_buffer, &begin)?;
            let clear = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.02, 0.02, 0.03, 1.0],
                },
            };
            let clear_values = [clear];
            let render_area = vk::Rect2D::default()
                .offset(vk::Offset2D::default())
                .extent(self.extent);
            let render_begin = vk::RenderPassBeginInfo::default()
                .render_pass(self.render_pass)
                .framebuffer(self.framebuffers[image_index as usize])
                .render_area(render_area)
                .clear_values(&clear_values);
            self.device.cmd_begin_render_pass(
                self.command_buffer,
                &render_begin,
                vk::SubpassContents::INLINE,
            );
            self.device.cmd_bind_pipeline(
                self.command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );
            let vertex_buffers = [self.vertex_buffer];
            let vertex_offsets = [0];
            self.device.cmd_bind_vertex_buffers(
                self.command_buffer,
                0,
                Some(&vertex_buffers),
                &vertex_offsets,
            );
            self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);
            self.device.cmd_end_render_pass(self.command_buffer);
            self.device.end_command_buffer(self.command_buffer)?;
        }

        let wait_semaphores = [self.image_available];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [self.command_buffer];
        let signal_semaphores = [render_finished];
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages)
            .command_buffers(&command_buffers)
            .signal_semaphores(&signal_semaphores);
        self.device
            .queue_submit(self.queue, &[submit], self.in_flight)?;

        let swapchains = [self.swapchain];
        let image_indices = [image_index];
        let present = vk::PresentInfoKHR::default()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        match self.device.queue_present_khr(self.queue, &present) {
            Ok(present_suboptimal) => Ok(!acquired_suboptimal && !present_suboptimal),
            Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => Ok(false),
            Err(err) => Err(anyhow!("vkQueuePresentKHR failed: {err:?}")),
        }
    }

    unsafe fn recreate_swapchain(&mut self, width: u32, height: u32) -> Result<()> {
        self.device.device_wait_idle()?;
        self.destroy_swapchain_resources();

        let caps = surface_capabilities(&self.instance, self.physical_device, self.surface)?;
        let surface_format =
            choose_surface_format(&self.instance, self.physical_device, self.surface)?;
        let requested_extent = if caps.current_extent.width != u32::MAX {
            vk::Extent2D::default()
                .width(caps.current_extent.width)
                .height(caps.current_extent.height)
        } else {
            vk::Extent2D::default()
                .width(width.max(1))
                .height(height.max(1))
        };
        let extent = clamp_extent(caps, requested_extent);

        self.swapchain =
            create_swapchain(&self.device, self.surface, caps, surface_format, extent)?;
        self.images = swapchain_images(&self.device, self.swapchain)?;
        self.image_views = self
            .images
            .iter()
            .map(|&image| create_image_view(&self.device, image, surface_format.format))
            .collect::<Result<Vec<_>>>()?;
        self.render_pass = create_render_pass(
            &self.device,
            surface_format.format,
            vk::ImageLayout::PRESENT_SRC_KHR,
        )?;
        self.framebuffers = self
            .image_views
            .iter()
            .map(|&view| {
                create_framebuffer(
                    &self.device,
                    self.render_pass,
                    view,
                    extent.width,
                    extent.height,
                )
            })
            .collect::<Result<Vec<_>>>()?;
        self.render_finished = (0..self.images.len())
            .map(|_| create_semaphore(&self.device))
            .collect::<Result<Vec<_>>>()?;
        self.pipeline = create_pipeline(
            &self.device,
            self.render_pass,
            self.pipeline_layout,
            extent.width,
            extent.height,
        )?;
        self.extent = extent;
        Ok(())
    }

    unsafe fn destroy_swapchain_resources(&mut self) {
        if !self.pipeline.is_null() {
            self.device.destroy_pipeline(self.pipeline, None);
            self.pipeline = vk::Pipeline::null();
        }
        for framebuffer in self.framebuffers.drain(..) {
            self.device.destroy_framebuffer(framebuffer, None);
        }
        if !self.render_pass.is_null() {
            self.device.destroy_render_pass(self.render_pass, None);
            self.render_pass = vk::RenderPass::null();
        }
        for view in self.image_views.drain(..) {
            self.device.destroy_image_view(view, None);
        }
        for semaphore in self.render_finished.drain(..) {
            self.device.destroy_semaphore(semaphore, None);
        }
        if !self.swapchain.is_null() {
            self.device.destroy_swapchain_khr(self.swapchain, None);
            self.swapchain = vk::SwapchainKHR::null();
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        unsafe {
            let _ = self.device.device_wait_idle();
            self.destroy_swapchain_resources();
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_semaphore(self.image_available, None);
            self.device.destroy_fence(self.in_flight, None);
            self.device.destroy_command_pool(self.command_pool, None);
            if !self.vertex_buffer.is_null() {
                self.device.destroy_buffer(self.vertex_buffer, None);
                self.vertex_buffer = vk::Buffer::null();
            }
            if let Some(allocation) = self.vertex_allocation.take() {
                let _ = self.allocator.free(allocation);
            }
            self.device.destroy_device(None);
            if !self.debug_messenger.is_null() {
                self.instance
                    .destroy_debug_utils_messenger_ext(self.debug_messenger, None);
            }
            self.instance.destroy_surface_khr(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}

unsafe fn create_instance(entry: &Entry, extensions: &[&CStr]) -> Result<Instance> {
    let app_name = CString::new("vulkan-wayland-triangle")?;
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
    let validation = c"VK_LAYER_KHRONOS_validation";
    let has_validation = entry
        .enumerate_instance_layer_properties()?
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

unsafe fn pick_physical_device(
    instance: &Instance,
    surface: vk::SurfaceKHR,
    required_extensions: &[&CStr],
) -> Result<PhysicalDeviceSelection> {
    let mut best = None;
    for physical_device in instance.enumerate_physical_devices()? {
        if !device_supports_extensions(instance, physical_device, required_extensions)? {
            continue;
        }
        let Some(queue_family) =
            find_present_graphics_queue_family(instance, physical_device, surface)?
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
                    queue_family,
                },
            ));
        }
    }
    best.map(|(_, selection)| selection).ok_or_else(|| {
        anyhow!("no physical device supports graphics, present, swapchain, and required features")
    })
}

unsafe fn device_supports_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    required_extensions: &[&CStr],
) -> Result<bool> {
    let available = instance.enumerate_device_extension_properties(physical_device, None)?;
    Ok(required_extensions.iter().all(|&required| {
        available
            .iter()
            .any(|extension| CStr::from_ptr(extension.extension_name.as_ptr()) == required)
    }))
}

unsafe fn find_present_graphics_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<Option<u32>> {
    let props = instance.get_physical_device_queue_family_properties(physical_device)?;
    for (index, props) in props.iter().enumerate() {
        let supported = instance.get_physical_device_surface_support_khr(
            physical_device,
            index as u32,
            surface,
        )?;
        if props.queue_flags.contains(vk::QueueFlags::GRAPHICS) && supported {
            return Ok(Some(index as u32));
        }
    }
    Ok(None)
}

unsafe fn create_device_and_queue(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family: u32,
    extensions: &[&CStr],
) -> Result<(Device, vk::Queue)> {
    let priorities = [1.0_f32];
    let queue_info = [vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family)
        .queue_priorities(&priorities)];
    let extension_ptrs = extensions
        .iter()
        .map(|extension| extension.as_ptr())
        .collect::<Vec<_>>();
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&extension_ptrs);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

unsafe fn surface_capabilities(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceCapabilitiesKHR<'static>> {
    Ok(instance.get_physical_device_surface_capabilities_khr(physical_device, surface)?)
}

unsafe fn choose_surface_format(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceFormatKHR<'static>> {
    let formats = instance.get_physical_device_surface_formats_khr(physical_device, surface)?;
    Ok(formats
        .iter()
        .copied()
        .find(|f| f.format == vk::Format::B8G8R8A8_UNORM)
        .unwrap_or_else(|| formats[0]))
}

unsafe fn create_swapchain(
    device: &Device,
    surface: vk::SurfaceKHR,
    caps: vk::SurfaceCapabilitiesKHR<'_>,
    surface_format: vk::SurfaceFormatKHR<'_>,
    extent: vk::Extent2D<'static>,
) -> Result<vk::SwapchainKHR> {
    let mut image_count = caps.min_image_count + 1;
    if caps.max_image_count != 0 {
        image_count = image_count.min(caps.max_image_count);
    }
    let extent = clamp_extent(caps, extent);
    let composite_alpha = choose_composite_alpha(caps);
    let info = vk::SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(caps.current_transform)
        .composite_alpha(composite_alpha)
        .present_mode(vk::PresentModeKHR::PRESENT_MODE_FIFO_KHR)
        .clipped(true)
        .old_swapchain(vk::SwapchainKHR::null());
    Ok(device.create_swapchain_khr(&info, None)?)
}

fn clamp_extent(
    caps: vk::SurfaceCapabilitiesKHR<'_>,
    extent: vk::Extent2D<'static>,
) -> vk::Extent2D<'static> {
    if caps.current_extent.width != u32::MAX {
        vk::Extent2D::default()
            .width(caps.current_extent.width)
            .height(caps.current_extent.height)
    } else {
        vk::Extent2D::default()
            .width(
                extent
                    .width
                    .clamp(caps.min_image_extent.width, caps.max_image_extent.width),
            )
            .height(
                extent
                    .height
                    .clamp(caps.min_image_extent.height, caps.max_image_extent.height),
            )
    }
}

fn choose_composite_alpha(caps: vk::SurfaceCapabilitiesKHR<'_>) -> vk::CompositeAlphaFlagBitsKHR {
    if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::OPAQUE_KHR)
    {
        vk::CompositeAlphaFlagBitsKHR::OPAQUE_KHR
    } else if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED_KHR)
    {
        vk::CompositeAlphaFlagBitsKHR::PRE_MULTIPLIED_KHR
    } else if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED_KHR)
    {
        vk::CompositeAlphaFlagBitsKHR::POST_MULTIPLIED_KHR
    } else {
        vk::CompositeAlphaFlagBitsKHR::INHERIT_KHR
    }
}

unsafe fn swapchain_images(device: &Device, swapchain: vk::SwapchainKHR) -> Result<Vec<vk::Image>> {
    Ok(device.get_swapchain_images_khr(swapchain)?)
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

unsafe fn create_render_pass(
    device: &Device,
    format: vk::Format,
    final_layout: vk::ImageLayout,
) -> Result<vk::RenderPass> {
    let attachment = vk::AttachmentDescription::default()
        .format(format)
        .samples(vk::SampleCountFlagBits::_1)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .final_layout(final_layout);
    let attachment_ref = [vk::AttachmentReference::default()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)];
    let subpass = [vk::SubpassDescription::default()
        .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
        .color_attachments(&attachment_ref)];
    let attachments = [attachment];
    let info = vk::RenderPassCreateInfo::default()
        .attachments(&attachments)
        .subpasses(&subpass);
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

unsafe fn create_semaphore(device: &Device) -> Result<vk::Semaphore> {
    let info = vk::SemaphoreCreateInfo::default();
    Ok(device.create_semaphore(&info, None)?)
}

unsafe fn create_signaled_fence(device: &Device) -> Result<vk::Fence> {
    let info = vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
    Ok(device.create_fence(&info, None)?)
}

struct BufferAllocation {
    buffer: vk::Buffer,
    allocation: Allocation,
}

unsafe fn create_vertex_buffer(
    device: &Device,
    allocator: &mut Allocator,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
) -> Result<BufferAllocation> {
    let size = mem::size_of_val(&VERTICES) as u64;
    let staging_info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_SRC)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let staging_buffer = device.create_buffer(&staging_info, None)?;
    let staging_requirements = device.get_buffer_memory_requirements(staging_buffer);
    let staging_allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("triangle staging buffer"),
        requirements: staging_requirements,
        location: MemoryLocation::CpuToGpu,
        linear: true,
        memory_allocate_flags: vk::MemoryAllocateFlags::empty(),
        allocation_scheme: AllocationScheme::GpuAllocatorManaged,
    }) {
        Ok(allocation) => allocation,
        Err(err) => {
            device.destroy_buffer(staging_buffer, None);
            return Err(err.into());
        }
    };
    if let Err(err) = device.bind_buffer_memory(
        staging_buffer,
        staging_allocation.memory(),
        staging_allocation.offset(),
    ) {
        let _ = allocator.free(staging_allocation);
        device.destroy_buffer(staging_buffer, None);
        return Err(err.into());
    }

    let flags = allocator.allocation_memory_flags(&staging_allocation)?;
    {
        let mut mapped = allocator.map(&staging_allocation)?;
        let vertex_bytes = bytemuck::cast_slice(&VERTICES);
        mapped
            .byte_range_mut(0, vertex_bytes.len())
            .copy_from_slice(vertex_bytes);
        if !flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT) {
            mapped.flush()?;
        }
    }

    let info = vk::BufferCreateInfo::default()
        .size(size)
        .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);
    let buffer = device.create_buffer(&info, None)?;
    let requirements = device.get_buffer_memory_requirements(buffer);
    let allocation = match allocator.allocate(AllocationCreateDesc {
        name: Some("triangle gpu vertex buffer"),
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
        device.destroy_buffer(staging_buffer, None);
        let _ = allocator.free(staging_allocation);
        device.destroy_buffer(buffer, None);
        return Err(err.into());
    }

    let command_buffers = [allocate_command_buffer(device, command_pool)?];
    let begin =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    device.begin_command_buffer(command_buffers[0], &begin)?;
    let copy = [vk::BufferCopy::default().size(size)];
    device.cmd_copy_buffer(command_buffers[0], staging_buffer, buffer, &copy);
    let barrier = [vk::BufferMemoryBarrier::default()
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(vk::AccessFlags::VERTEX_ATTRIBUTE_READ)
        .buffer(buffer)
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

    device.destroy_buffer(staging_buffer, None);
    allocator.free(staging_allocation)?;

    Ok(BufferAllocation { buffer, allocation })
}

unsafe fn create_pipeline_layout(device: &Device) -> Result<vk::PipelineLayout> {
    let info = vk::PipelineLayoutCreateInfo::default();
    Ok(device.create_pipeline_layout(&info, None)?)
}

unsafe fn create_pipeline(
    device: &Device,
    render_pass: vk::RenderPass,
    layout: vk::PipelineLayout,
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
