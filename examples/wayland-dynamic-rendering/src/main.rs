//! A triangle on a Wayland surface with dynamic rendering.
//!
//! The swapchain parameters a compositor or an HDR-aware client cares about can be
//! chosen from the environment. The surface's support for each is printed at startup.
//!
//! - `VULKAN_EXAMPLE_PRESENT_MODE`: `fifo` (default), `fifo_relaxed`, `immediate`
//!   (tearing) or `mailbox`.
//! - `VULKAN_EXAMPLE_COLOR_SPACE`: `srgb` (default), `hdr10` (BT.2020 PQ, with
//!   `VK_EXT_hdr_metadata` when available) or `scrgb` (extended linear sRGB).
//! - `VULKAN_EXAMPLE_FRAMES`: exit after this many frames.
//!
//! The shader always writes the same values, so the triangle is only colorimetrically
//! right in `srgb`. The point of the other modes is the swapchain setup.

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
                .with_title("vulkan wayland dynamic rendering")
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
    image_layouts: Vec<vk::ImageLayout>,
    surface_format: vk::Format,
    present_mode: vk::PresentModeKHR,
    /// `VK_EXT_hdr_metadata` is enabled on the device.
    hdr_metadata: bool,
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    image_available: vk::Semaphore,
    render_finished: Vec<vk::Semaphore>,
    in_flight: vk::Fence,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    extent: vk::Extent2D,
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
        // Optional instance extensions: debug messages, color spaces beyond sRGB, and
        // the queries that tell which present modes a swapchain can switch between.
        for optional in [
            vk::EXT_DEBUG_UTILS_NAME,
            vk::EXT_SWAPCHAIN_COLOR_SPACE_NAME,
            vk::KHR_GET_SURFACE_CAPABILITIES_2_NAME,
            vk::EXT_SURFACE_MAINTENANCE_1_NAME,
        ] {
            if has_instance_extension(&entry, optional)? {
                instance_extensions.push(optional);
            }
        }
        let surface_maintenance = instance_extensions
            .contains(&vk::KHR_GET_SURFACE_CAPABILITIES_2_NAME)
            && instance_extensions.contains(&vk::EXT_SURFACE_MAINTENANCE_1_NAME);
        let debug_utils_enabled = instance_extensions.contains(&vk::EXT_DEBUG_UTILS_NAME);
        let instance = create_instance(&entry, &instance_extensions)?;
        let debug_messenger = create_debug_messenger(&instance, debug_utils_enabled)?;

        let surface_info = vk::WaylandSurfaceCreateInfoKHR::default()
            .display(wl_display)
            .surface(wl_surface);
        let surface = instance.create_wayland_surface_khr(&surface_info, None)?;

        let required_extensions = [vk::KHR_SWAPCHAIN_NAME];
        let selection = pick_physical_device(&instance, surface, &required_extensions)?;
        let mut device_extensions = required_extensions.to_vec();
        let hdr_metadata = device_supports_extensions(
            &instance,
            selection.physical_device,
            &[vk::EXT_HDR_METADATA_NAME],
        )?;
        if hdr_metadata {
            device_extensions.push(vk::EXT_HDR_METADATA_NAME);
        }
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

        print_surface_support(
            &instance,
            selection.physical_device,
            surface,
            surface_maintenance,
        )?;
        let caps = surface_capabilities(&instance, selection.physical_device, surface)?;
        let surface_format = choose_surface_format(&instance, selection.physical_device, surface)?;
        let present_mode = choose_present_mode(&instance, selection.physical_device, surface)?;
        println!(
            "swapchain: {:?} / {:?}, present mode {:?}",
            surface_format.format, surface_format.color_space, present_mode
        );
        let requested_extent = if caps.current_extent.width != u32::MAX {
            vk::Extent2D::default()
                .width(caps.current_extent.width)
                .height(caps.current_extent.height)
        } else {
            vk::Extent2D::default().width(width).height(height)
        };
        let extent = clamp_extent(caps, requested_extent);

        let swapchain =
            create_swapchain(&device, surface, caps, surface_format, present_mode, extent)?;
        if hdr_metadata {
            set_hdr_metadata(&device, swapchain, surface_format.color_space);
        }
        let images = swapchain_images(&device, swapchain)?;
        let image_views = images
            .iter()
            .map(|&image| create_image_view(&device, image, surface_format.format))
            .collect::<Result<Vec<_>>>()?;
        let image_layouts = vec![vk::ImageLayout::UNDEFINED; images.len()];
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
            surface_format.format,
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
            image_layouts,
            surface_format: surface_format.format,
            present_mode,
            hdr_metadata,
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
            let command_buffer = self.command_buffer;
            let image_view = self.image_views[image_index as usize];
            let image = self.images[image_index as usize];
            let old_layout = self.image_layouts[image_index as usize];
            let begin = vk::CommandBufferBeginInfo::default();
            self.device.begin_command_buffer(command_buffer, &begin)?;
            let subresource_range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .level_count(1)
                .layer_count(1);
            let to_color_attachment = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::empty())
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .old_layout(old_layout)
                .new_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .image(image)
                .subresource_range(subresource_range)];
            self.device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::TOP_OF_PIPE,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &to_color_attachment,
            );

            let clear = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.02, 0.02, 0.03, 1.0],
                },
            };
            let render_area = vk::Rect2D::default()
                .offset(vk::Offset2D::default())
                .extent(self.extent);
            let color_attachment = [vk::RenderingAttachmentInfo::default()
                .image_view(image_view)
                .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .clear_value(clear)];
            let rendering = vk::RenderingInfo::default()
                .render_area(render_area)
                .layer_count(1)
                .color_attachments(&color_attachment);
            self.device.cmd_begin_rendering(command_buffer, &rendering);
            self.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                self.pipeline,
            );
            let vertex_buffers = [self.vertex_buffer];
            let vertex_offsets = [0];
            self.device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &vertex_buffers,
                &vertex_offsets,
            );
            self.device.cmd_draw(command_buffer, 3, 1, 0, 0);
            self.device.cmd_end_rendering(command_buffer);

            let to_present = [vk::ImageMemoryBarrier::default()
                .src_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .dst_access_mask(vk::AccessFlags::empty())
                .old_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .new_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .image(image)
                .subresource_range(subresource_range)];
            self.device.cmd_pipeline_barrier(
                command_buffer,
                vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                vk::DependencyFlags::empty(),
                &[],
                &[],
                &to_present,
            );
            self.device.end_command_buffer(command_buffer)?;
        }
        self.image_layouts[image_index as usize] = vk::ImageLayout::PRESENT_SRC_KHR;

        let wait_semaphores = [self.image_available];
        let wait_stages = [vk::PipelineStageFlags::TOP_OF_PIPE];
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

        self.swapchain = create_swapchain(
            &self.device,
            self.surface,
            caps,
            surface_format,
            self.present_mode,
            extent,
        )?;
        if self.hdr_metadata {
            set_hdr_metadata(&self.device, self.swapchain, surface_format.color_space);
        }
        self.images = swapchain_images(&self.device, self.swapchain)?;
        self.image_views = self
            .images
            .iter()
            .map(|&image| create_image_view(&self.device, image, surface_format.format))
            .collect::<Result<Vec<_>>>()?;
        self.image_layouts = vec![vk::ImageLayout::UNDEFINED; self.images.len()];
        self.surface_format = surface_format.format;
        self.render_finished = (0..self.images.len())
            .map(|_| create_semaphore(&self.device))
            .collect::<Result<Vec<_>>>()?;
        self.pipeline = create_pipeline(
            &self.device,
            self.surface_format,
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
    let app_name = CString::new("vulkan-wayland-dynamic-rendering")?;
    let enabled_layers = validation_layers(entry)?;
    let enabled_layer_ptrs = enabled_layers
        .iter()
        .map(|layer| vk::CStrPtr::new(layer))
        .collect::<Vec<_>>();
    let extension_ptrs = extensions
        .iter()
        .map(|extension| vk::CStrPtr::new(extension))
        .collect::<Vec<_>>();
    let app_info = vk::ApplicationInfo::default()
        .application_name(app_name.as_c_str())
        .engine_name(app_name.as_c_str())
        .api_version(vk::make_api_version(0, 1, 3, 0));
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
        .any(|extension| extension.extension_name_as_c_str() == Ok(name)))
}

unsafe fn validation_layers(entry: &Entry) -> Result<Vec<&'static CStr>> {
    let validation = c"VK_LAYER_KHRONOS_validation";
    let has_validation = entry
        .enumerate_instance_layer_properties()?
        .iter()
        .any(|layer| layer.layer_name_as_c_str() == Ok(validation));
    if has_validation {
        Ok(vec![validation])
    } else {
        Err(anyhow!(
            "VK_LAYER_KHRONOS_validation is required for this example"
        ))
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
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
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
        let mut dynamic_rendering_features = vk::PhysicalDeviceDynamicRenderingFeatures::default();
        {
            let mut features2 =
                vk::PhysicalDeviceFeatures2::default().push_next(&mut dynamic_rendering_features);
            instance.get_physical_device_features2(physical_device, &mut features2);
        }
        if dynamic_rendering_features.dynamic_rendering == 0 {
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
            .any(|extension| extension.extension_name_as_c_str() == Ok(required))
    }))
}

unsafe fn find_present_graphics_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<Option<u32>> {
    let props = instance.get_physical_device_queue_family_properties(physical_device);
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
        .map(|extension| vk::CStrPtr::new(extension))
        .collect::<Vec<_>>();
    let mut dynamic_rendering_features =
        vk::PhysicalDeviceDynamicRenderingFeatures::default().dynamic_rendering(true);
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_info)
        .enabled_extension_names(&extension_ptrs)
        .push_next(&mut dynamic_rendering_features);
    let device = instance.create_device(physical_device, &create_info, None)?;
    let queue = device.get_device_queue(queue_family, 0);
    Ok((device, queue))
}

unsafe fn surface_capabilities(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceCapabilitiesKHR> {
    Ok(instance.get_physical_device_surface_capabilities_khr(physical_device, surface)?)
}

/// The format and color space for the swapchain, following `VULKAN_EXAMPLE_COLOR_SPACE`.
/// Falls back to 8-bit sRGB when the surface does not offer the requested one.
unsafe fn choose_surface_format(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::SurfaceFormatKHR> {
    let formats = instance.get_physical_device_surface_formats_khr(physical_device, surface)?;
    let wanted = std::env::var("VULKAN_EXAMPLE_COLOR_SPACE").unwrap_or_default();
    let find = |color_space: vk::ColorSpaceKHR, candidates: &[vk::Format]| {
        candidates.iter().find_map(|&format| {
            formats
                .iter()
                .copied()
                .find(|f| f.format == format && f.color_space == color_space)
        })
    };
    let chosen = match wanted.as_str() {
        // BT.2020 primaries with the PQ transfer function; needs 10 bits or more.
        "hdr10" => find(
            vk::ColorSpaceKHR::HDR10_ST2084_EXT,
            &[
                vk::Format::A2B10G10R10_UNORM_PACK32,
                vk::Format::A2R10G10B10_UNORM_PACK32,
                vk::Format::R16G16B16A16_SFLOAT,
            ],
        ),
        // sRGB primaries, linear, values outside [0, 1] allowed.
        "scrgb" => find(
            vk::ColorSpaceKHR::EXTENDED_SRGB_LINEAR_EXT,
            &[vk::Format::R16G16B16A16_SFLOAT],
        ),
        _ => None,
    };
    if chosen.is_none() && !matches!(wanted.as_str(), "" | "srgb") {
        println!("color space `{wanted}` is not offered by the surface; using sRGB");
    }
    Ok(chosen
        .or_else(|| {
            find(
                vk::ColorSpaceKHR::SRGB_NONLINEAR,
                &[vk::Format::B8G8R8A8_UNORM, vk::Format::R8G8B8A8_UNORM],
            )
        })
        .unwrap_or(formats[0]))
}

/// The present mode for the swapchain, following `VULKAN_EXAMPLE_PRESENT_MODE`.
///
/// `IMMEDIATE` presents without waiting for vertical blank and may tear; `FIFO_RELAXED`
/// tears only when a frame is late. `FIFO` is the only mode every surface supports.
unsafe fn choose_present_mode(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<vk::PresentModeKHR> {
    let supported =
        instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?;
    let wanted = match std::env::var("VULKAN_EXAMPLE_PRESENT_MODE")
        .unwrap_or_default()
        .as_str()
    {
        "immediate" => vk::PresentModeKHR::IMMEDIATE,
        "fifo_relaxed" => vk::PresentModeKHR::FIFO_RELAXED,
        "mailbox" => vk::PresentModeKHR::MAILBOX,
        _ => vk::PresentModeKHR::FIFO,
    };
    if supported.contains(&wanted) {
        Ok(wanted)
    } else {
        println!("present mode {wanted:?} is not supported by the surface; using FIFO");
        Ok(vk::PresentModeKHR::FIFO)
    }
}

/// Prints what the surface offers: formats with color spaces, present modes, and, with
/// `VK_EXT_surface_maintenance1`, which present modes a swapchain can switch between
/// without being recreated.
unsafe fn print_surface_support(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_maintenance: bool,
) -> Result<()> {
    println!("surface formats:");
    for format in instance.get_physical_device_surface_formats_khr(physical_device, surface)? {
        println!("  {:?} / {:?}", format.format, format.color_space);
    }
    let present_modes =
        instance.get_physical_device_surface_present_modes_khr(physical_device, surface)?;
    println!("present modes: {present_modes:?}");

    if surface_maintenance {
        for &mode in &present_modes {
            // The two-call idiom inside a pNext struct: first the count, then the array.
            let mut query = vk::SurfacePresentModeEXT::default().present_mode(mode);
            let surface_info = vk::PhysicalDeviceSurfaceInfo2KHR::default()
                .surface(surface)
                .push_next(&mut query);
            let mut compatibility = vk::SurfacePresentModeCompatibilityEXT::default();
            let mut caps = vk::SurfaceCapabilities2KHR::default().push_next(&mut compatibility);
            instance.get_physical_device_surface_capabilities2_khr(
                physical_device,
                &surface_info,
                &mut caps,
            )?;
            let mut compatible =
                vec![vk::PresentModeKHR::default(); compatibility.present_mode_count as usize];
            let mut compatibility =
                vk::SurfacePresentModeCompatibilityEXT::default().present_modes(&mut compatible);
            let mut caps = vk::SurfaceCapabilities2KHR::default().push_next(&mut compatibility);
            instance.get_physical_device_surface_capabilities2_khr(
                physical_device,
                &surface_info,
                &mut caps,
            )?;
            let written = compatibility.present_mode_count as usize;
            println!("  {mode:?} can switch to {:?}", &compatible[..written]);
        }
    }
    Ok(())
}

/// Describes the mastering display to the compositor when the swapchain is HDR10.
unsafe fn set_hdr_metadata(
    device: &Device,
    swapchain: vk::SwapchainKHR,
    color_space: vk::ColorSpaceKHR,
) {
    if color_space != vk::ColorSpaceKHR::HDR10_ST2084_EXT {
        return;
    }
    let xy = |x: f32, y: f32| vk::XYColorEXT { x, y };
    // BT.2020 primaries, D65 white, a 1000 cd/m2 mastering display.
    let metadata = vk::HdrMetadataEXT::default()
        .display_primary_red(xy(0.708, 0.292))
        .display_primary_green(xy(0.170, 0.797))
        .display_primary_blue(xy(0.131, 0.046))
        .white_point(xy(0.3127, 0.3290))
        .max_luminance(1000.0)
        .min_luminance(0.001)
        .max_content_light_level(1000.0)
        .max_frame_average_light_level(400.0);
    device.set_hdr_metadata_ext(&[swapchain], &[metadata]);
    println!("set HDR10 metadata");
}

unsafe fn create_swapchain(
    device: &Device,
    surface: vk::SurfaceKHR,
    caps: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    extent: vk::Extent2D,
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
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(vk::SwapchainKHR::null());
    Ok(device.create_swapchain_khr(&info, None)?)
}

fn clamp_extent(caps: vk::SurfaceCapabilitiesKHR, extent: vk::Extent2D) -> vk::Extent2D {
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

fn choose_composite_alpha(caps: vk::SurfaceCapabilitiesKHR) -> vk::CompositeAlphaFlagBitsKHR {
    if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::OPAQUE)
    {
        vk::CompositeAlphaFlagBitsKHR::OPAQUE
    } else if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::PRE_MULTIPLIED)
    {
        vk::CompositeAlphaFlagBitsKHR::PRE_MULTIPLIED
    } else if caps
        .supported_composite_alpha
        .contains(vk::CompositeAlphaFlagsKHR::POST_MULTIPLIED)
    {
        vk::CompositeAlphaFlagBitsKHR::POST_MULTIPLIED
    } else {
        vk::CompositeAlphaFlagBitsKHR::INHERIT
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
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
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
        export_handle_types: vk::ExternalMemoryHandleTypeFlags::empty(),
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
    format: vk::Format,
    layout: vk::PipelineLayout,
    width: u32,
    height: u32,
) -> Result<vk::Pipeline> {
    let vert = create_shader_module(device, include_bytes!("shaders/triangle.vert.spv"))?;
    let frag = create_shader_module(device, include_bytes!("shaders/triangle.frag.spv"))?;
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
    let formats = [format];
    let mut rendering =
        vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);
    let info = vk::GraphicsPipelineCreateInfo::default()
        .stages(&stages)
        .vertex_input_state(&vertex_input)
        .input_assembly_state(&input_assembly)
        .viewport_state(&viewport_state)
        .rasterization_state(&raster)
        .multisample_state(&multisample)
        .color_blend_state(&color_blend)
        .layout(layout)
        .push_next(&mut rendering);
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
