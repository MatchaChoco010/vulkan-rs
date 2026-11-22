//! What the examples that present to a window have in common: the event loop, the
//! surface, the swapchain, and the acquire, submit and present of a frame. Built with
//! the `window` feature, for a Wayland session.

pub mod frame;
pub mod surface;
pub mod swapchain;

use anyhow::{Result, bail};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
use std::fmt::Display;
use vulkan::vk;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use frame::SwapchainStatus;

const WINDOW_SIZE: LogicalSize<u32> = LogicalSize {
    width: 800,
    height: 600,
};

/// The `wl_display` and `wl_surface` of a window, which a `VkSurfaceKHR` is created
/// from.
#[derive(Clone, Copy)]
pub struct WaylandWindow {
    pub display: *mut vk::wl_display,
    pub surface: *mut vk::wl_surface,
}

/// What an example draws to its window with. [`run`] creates it once the window exists
/// and destroys it before the window.
pub trait Renderer: Sized {
    /// `window_extent` is the size of the window in pixels.
    ///
    /// # Safety
    ///
    /// `window` stays valid until [`Renderer::destroy`] returns.
    unsafe fn new(window: WaylandWindow, window_extent: vk::Extent2D) -> Result<Self>;

    /// Draws a frame and presents it.
    unsafe fn draw(&mut self) -> Result<SwapchainStatus>;

    /// Replaces the swapchain after the window was resized, or after [`Renderer::draw`]
    /// found it outdated.
    unsafe fn recreate_swapchain(&mut self, window_extent: vk::Extent2D) -> Result<()>;

    unsafe fn destroy(self);
}

/// Opens a window titled `title` and draws to it until the window is closed or an error
/// is printed. When `VULKAN_EXAMPLE_FRAMES` is set, it stops after that many frames.
pub fn run<R: Renderer>(title: &str) -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut runner = Runner::<R> {
        title,
        renderer: None,
        window: None,
        frames: 0,
        max_frames: std::env::var("VULKAN_EXAMPLE_FRAMES")
            .ok()
            .and_then(|value| value.parse::<u32>().ok()),
    };
    event_loop.run_app(&mut runner)?;
    Ok(())
}

struct Runner<'a, R: Renderer> {
    title: &'a str,
    renderer: Option<R>,
    window: Option<Window>,
    frames: u32,
    max_frames: Option<u32>,
}

impl<R: Renderer> ApplicationHandler for Runner<'_, R> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Wait);
        if self.window.is_some() {
            return;
        }

        let window = match event_loop.create_window(
            Window::default_attributes()
                .with_title(self.title)
                .with_inner_size(WINDOW_SIZE),
        ) {
            Ok(window) => window,
            Err(err) => return exit_with_error(event_loop, err),
        };
        let wayland = match wayland_window(&window) {
            Ok(wayland) => wayland,
            Err(err) => return exit_with_error(event_loop, err),
        };
        // SAFETY: the renderer is destroyed before the window, in `Runner::drop`.
        match unsafe { R::new(wayland, window_extent(window.inner_size())) } {
            Ok(renderer) => {
                self.renderer = Some(renderer);
                self.window = Some(window);
            }
            Err(err) => exit_with_error(event_loop, err),
        }
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
                let Some(renderer) = self.renderer.as_mut() else {
                    return;
                };
                // SAFETY: the renderer upholds the valid usage rules of the commands it
                // calls; the validation layer, when installed, reports any violation.
                if let Err(err) = unsafe { renderer.recreate_swapchain(window_extent(size)) } {
                    exit_with_error(event_loop, err);
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(renderer) = self.renderer.as_mut() else {
                    return;
                };
                // SAFETY: as for `recreate_swapchain` above.
                match unsafe { renderer.draw() } {
                    Ok(SwapchainStatus::Current) => {
                        self.frames += 1;
                        if self.max_frames.is_some_and(|max| self.frames >= max) {
                            event_loop.exit();
                        }
                    }
                    Ok(SwapchainStatus::Outdated) => {
                        let Some(window) = self.window.as_ref() else {
                            return;
                        };
                        let extent = window_extent(window.inner_size());
                        // SAFETY: as for `recreate_swapchain` above.
                        if let Err(err) = unsafe { renderer.recreate_swapchain(extent) } {
                            exit_with_error(event_loop, err);
                        }
                    }
                    Err(err) => exit_with_error(event_loop, err),
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.renderer.is_some()
            && let Some(window) = self.window.as_ref()
        {
            window.request_redraw();
        }
    }
}

impl<R: Renderer> Drop for Runner<'_, R> {
    fn drop(&mut self) {
        // The Vulkan surface has to go before the Wayland surface it was created from,
        // which the window owns.
        if let Some(renderer) = self.renderer.take() {
            // SAFETY: the window is still alive, as `Renderer::new` requires.
            unsafe { renderer.destroy() };
        }
    }
}

fn wayland_window(window: &Window) -> Result<WaylandWindow> {
    let display = window.display_handle()?.as_raw();
    let surface = window.window_handle()?.as_raw();
    match (display, surface) {
        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(surface)) => {
            Ok(WaylandWindow {
                display: display.display.as_ptr(),
                surface: surface.surface.as_ptr(),
            })
        }
        _ => bail!("this example must run on a Wayland winit backend"),
    }
}

/// A window can be zero-sized while it is minimized, an image cannot.
fn window_extent(size: PhysicalSize<u32>) -> vk::Extent2D {
    vk::Extent2D {
        width: size.width.max(1),
        height: size.height.max(1),
    }
}

fn exit_with_error(event_loop: &ActiveEventLoop, error: impl Display) {
    eprintln!("{error:#}");
    event_loop.exit();
}
