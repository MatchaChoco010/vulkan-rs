//! What the examples that present to a window have in common: the event loop, the
//! surface, the swapchain, and the acquire, submit and present of a frame. Built with
//! the `window` feature, for a Wayland session.

pub mod frame;
pub mod surface;
pub mod swapchain;

use anyhow::{Result, bail};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle};
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

    /// Destroys everything the renderer created, also after a step of it failed, and
    /// returns the first error.
    unsafe fn destroy(self) -> Result<()>;
}

/// Opens a window titled `title` and draws to it until the window is closed or an error
/// occurs. When `VULKAN_EXAMPLE_FRAMES` is set, it stops after that many frames.
///
/// # Errors
///
/// The first error of creating the window or the renderer, of drawing, of recreating
/// the swapchain, of the event loop, or of destroying the renderer.
pub fn run<R: Renderer>(title: &str) -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut runner = Runner::<R> {
        title,
        session: Session::new(
            std::env::var("VULKAN_EXAMPLE_FRAMES")
                .ok()
                .and_then(|value| value.parse::<u32>().ok()),
        ),
        window: None,
    };
    let event_loop_result = event_loop.run_app(&mut runner).map_err(anyhow::Error::from);
    runner.session.finish(event_loop_result)
}

/// The fields drop in order: the session destroys a renderer that is still there
/// before the window goes, since the Vulkan surface has to go before the Wayland
/// surface it was created from.
struct Runner<'a, R: Renderer> {
    title: &'a str,
    session: Session<R>,
    window: Option<Window>,
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
            Err(error) => return self.session.fail(error.into()).apply(event_loop),
        };
        let extent = window_extent(window.inner_size());
        let flow = self.session.start(|| {
            let wayland = wayland_window(&window)?;
            // SAFETY: the window is kept until the session has destroyed the renderer,
            // which the order of the fields of `Runner` ensures.
            unsafe { R::new(wayland, extent) }
        });
        // A renderer exists only if it was just made from this window, which has to
        // outlive it: with a window kept, `resumed` returns above.
        if self.session.has_renderer() {
            self.window = Some(window);
        }
        flow.apply(event_loop);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let flow = match event {
            WindowEvent::CloseRequested => Flow::Exit,
            WindowEvent::Resized(size) => self.session.resize(window_extent(size)),
            WindowEvent::RedrawRequested => match self.window.as_ref() {
                Some(window) => self.session.redraw(window_extent(window.inner_size())),
                None => Flow::Continue,
            },
            _ => Flow::Continue,
        };
        flow.apply(event_loop);
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.session.is_running()
            && let Some(window) = self.window.as_ref()
        {
            window.request_redraw();
        }
    }
}

/// Whether the event loop goes on after an event.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Flow {
    Continue,
    Exit,
}

impl Flow {
    fn apply(self, event_loop: &ActiveEventLoop) {
        if self == Flow::Exit {
            event_loop.exit();
        }
    }
}

/// The renderer and what happened to it, apart from the window and the event loop: it
/// stops at the first error and keeps that error for [`Session::finish`].
struct Session<R: Renderer> {
    renderer: Option<R>,
    frames: u32,
    max_frames: Option<u32>,
    error: Option<anyhow::Error>,
}

impl<R: Renderer> Session<R> {
    fn new(max_frames: Option<u32>) -> Self {
        Self {
            renderer: None,
            frames: 0,
            max_frames,
            error: None,
        }
    }

    /// Whether there is a renderer, also after a failure: its window has to be kept until
    /// the session has destroyed it.
    fn has_renderer(&self) -> bool {
        self.renderer.is_some()
    }

    /// Whether there is a renderer and nothing has failed. After a failure the renderer
    /// may be half torn down, so the session only destroys it.
    fn is_running(&self) -> bool {
        self.renderer.is_some() && self.error.is_none()
    }

    /// Creates the renderer with `create`, unless a step has failed or there is a
    /// renderer already: a second `Resumed` must not replace the renderer whose window
    /// is kept, nor start over after a failure.
    fn start(&mut self, create: impl FnOnce() -> Result<R>) -> Flow {
        if self.error.is_some() || self.renderer.is_some() {
            return Flow::Exit;
        }
        match create() {
            Ok(renderer) => {
                self.renderer = Some(renderer);
                Flow::Continue
            }
            Err(error) => self.fail(error),
        }
    }

    fn resize(&mut self, extent: vk::Extent2D) -> Flow {
        if self.error.is_some() {
            return Flow::Exit;
        }
        let Some(renderer) = self.renderer.as_mut() else {
            return Flow::Continue;
        };
        // SAFETY: the renderer upholds the valid usage rules of the commands it calls;
        // the validation layer, when installed, reports any violation.
        match unsafe { renderer.recreate_swapchain(extent) } {
            Ok(()) => Flow::Continue,
            Err(error) => self.fail(error),
        }
    }

    /// `extent` is the size of the window, for a swapchain that turns out outdated.
    fn redraw(&mut self, extent: vk::Extent2D) -> Flow {
        if self.error.is_some() {
            return Flow::Exit;
        }
        let Some(renderer) = self.renderer.as_mut() else {
            return Flow::Continue;
        };
        // SAFETY: as for `Session::resize`.
        match unsafe { renderer.draw() } {
            Ok(SwapchainStatus::Current) => {
                self.frames += 1;
                if self.max_frames.is_some_and(|max| self.frames >= max) {
                    Flow::Exit
                } else {
                    Flow::Continue
                }
            }
            Ok(SwapchainStatus::Outdated) => self.resize(extent),
            Err(error) => self.fail(error),
        }
    }

    /// Prints the error at once, so that it reaches stderr even if the process does not
    /// get to return it, and keeps the first one for [`Session::finish`].
    fn fail(&mut self, error: anyhow::Error) -> Flow {
        eprintln!("{error:#}");
        if self.error.is_none() {
            self.error = Some(error);
        }
        Flow::Exit
    }

    /// Destroys the renderer and tells how the run ended, given how the event loop did.
    fn finish(&mut self, event_loop: Result<()>) -> Result<()> {
        let destroyed = match self.renderer.take() {
            // SAFETY: the window is still alive, as `Renderer::new` requires.
            Some(renderer) => unsafe { renderer.destroy() },
            None => Ok(()),
        };
        first_error([self.error.take().map_or(Ok(()), Err), event_loop, destroyed])
    }
}

impl<R: Renderer> Drop for Session<R> {
    fn drop(&mut self) {
        if let Some(renderer) = self.renderer.take() {
            // SAFETY: as in `Session::finish`.
            if let Err(error) = unsafe { renderer.destroy() } {
                eprintln!("{error:#}");
            }
        }
    }
}

/// The first error of `results` in their order, after printing the later ones; success
/// only when every one of them succeeded.
pub fn first_error(results: impl IntoIterator<Item = Result<()>>) -> Result<()> {
    let mut first = Ok(());
    for result in results {
        match (&first, result) {
            (_, Ok(())) => {}
            (Ok(()), Err(error)) => first = Err(error),
            (Err(_), Err(error)) => eprintln!("{error:#}"),
        }
    }
    first
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::cell::Cell;
    use std::rc::Rc;

    const EXTENT: vk::Extent2D = vk::Extent2D {
        width: 8,
        height: 8,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    enum Step {
        Create,
        Draw,
        Recreate,
        Destroy,
    }

    /// What a [`Fake`] renderer does: `failing` is a step and the call of it, counted
    /// from 0, that fails; with `outdated`, every other draw finds the swapchain
    /// outdated.
    #[derive(Clone, Copy, Debug)]
    struct Script {
        failing: Option<(Step, u32)>,
        outdated: bool,
        max_frames: Option<u32>,
    }

    struct Fake {
        script: Script,
        draws: u32,
        recreations: u32,
        failed: Rc<Cell<Option<Step>>>,
        calls_after_failure: Rc<Cell<u32>>,
    }

    impl Fake {
        fn new(
            script: Script,
            failed: Rc<Cell<Option<Step>>>,
            calls_after_failure: Rc<Cell<u32>>,
        ) -> Result<Self> {
            let fake = Fake {
                script,
                draws: 0,
                recreations: 0,
                failed,
                calls_after_failure,
            };
            fake.check(Step::Create, 0)?;
            Ok(fake)
        }

        fn check(&self, step: Step, call: u32) -> Result<()> {
            if self.failed.get().is_some() && step != Step::Destroy {
                self.calls_after_failure
                    .set(self.calls_after_failure.get() + 1);
            }
            if self.script.failing == Some((step, call)) {
                self.failed.set(Some(step));
                bail!("{step:?} failed");
            }
            Ok(())
        }
    }

    impl Renderer for Fake {
        unsafe fn new(_window: WaylandWindow, _window_extent: vk::Extent2D) -> Result<Self> {
            bail!("the tests create the renderer with Session::start")
        }

        unsafe fn draw(&mut self) -> Result<SwapchainStatus> {
            self.check(Step::Draw, self.draws)?;
            self.draws += 1;
            Ok(if self.script.outdated && self.draws.is_multiple_of(2) {
                SwapchainStatus::Outdated
            } else {
                SwapchainStatus::Current
            })
        }

        unsafe fn recreate_swapchain(&mut self, _window_extent: vk::Extent2D) -> Result<()> {
            self.check(Step::Recreate, self.recreations)?;
            self.recreations += 1;
            Ok(())
        }

        unsafe fn destroy(self) -> Result<()> {
            self.check(Step::Destroy, 0)
        }
    }

    /// Runs a session the way the event loop drives it: a redraw, a resize, a redraw,
    /// and so on. The events go on after a step asks to exit, since winit finishes the
    /// iteration of the loop first, and a resize in it is followed by a redraw. Returns
    /// how the run ended and the step that failed, if one did.
    fn run_script(script: Script, event_loop: Result<()>) -> (Result<()>, Option<Step>) {
        let failed = Rc::new(Cell::new(None));
        let calls_after_failure = Rc::new(Cell::new(0));
        let mut session = Session::<Fake>::new(script.max_frames);
        let mut flow =
            session.start(|| Fake::new(script, failed.clone(), calls_after_failure.clone()));
        for event in 0..12 {
            if failed.get().is_some() {
                assert_eq!(
                    flow,
                    Flow::Exit,
                    "{script:?}: a failed step must end the loop"
                );
                assert!(!session.is_running(), "{script:?}: the session still runs");
            }
            flow = if event % 3 == 1 {
                session.resize(EXTENT)
            } else {
                session.redraw(EXTENT)
            };
        }
        assert_eq!(
            calls_after_failure.get(),
            0,
            "{script:?}: the renderer drew or recreated its swapchain after a failure"
        );
        (session.finish(event_loop), failed.get())
    }

    #[test]
    fn a_run_fails_exactly_when_a_step_or_the_event_loop_failed() {
        let steps = [Step::Create, Step::Draw, Step::Recreate, Step::Destroy];
        let failures = [None]
            .into_iter()
            .chain(
                steps
                    .into_iter()
                    .flat_map(|step| (0..3).map(move |call| Some((step, call)))),
            )
            .collect::<Vec<_>>();
        let mut steps_that_failed = Vec::new();
        for failing in failures {
            for outdated in [false, true] {
                for max_frames in [None, Some(1), Some(3)] {
                    for event_loop_fails in [false, true] {
                        let script = Script {
                            failing,
                            outdated,
                            max_frames,
                        };
                        let event_loop = if event_loop_fails {
                            Err(anyhow!("the event loop failed"))
                        } else {
                            Ok(())
                        };
                        let (result, failed) = run_script(script, event_loop);
                        let expected = match failed {
                            Some(step @ (Step::Create | Step::Draw | Step::Recreate)) => {
                                Some(format!("{step:?} failed"))
                            }
                            _ if event_loop_fails => Some("the event loop failed".to_owned()),
                            Some(Step::Destroy) => Some("Destroy failed".to_owned()),
                            None => None,
                        };
                        assert_eq!(
                            result.err().map(|error| error.to_string()),
                            expected,
                            "{script:?}, event loop fails: {event_loop_fails}"
                        );
                        steps_that_failed.extend(failed);
                    }
                }
            }
        }
        for step in steps {
            assert!(
                steps_that_failed.contains(&step),
                "no script made {step:?} fail"
            );
        }
    }

    #[test]
    fn a_run_without_errors_stops_after_max_frames_and_succeeds() {
        for max_frames in 1..5 {
            let script = Script {
                failing: None,
                outdated: false,
                max_frames: Some(max_frames),
            };
            let mut session = Session::<Fake>::new(script.max_frames);
            let flow = session.start(|| Fake::new(script, Rc::default(), Rc::default()));
            assert_eq!(flow, Flow::Continue);
            for frame in 1..max_frames {
                assert_eq!(session.redraw(EXTENT), Flow::Continue, "frame {frame}");
            }
            assert_eq!(session.redraw(EXTENT), Flow::Exit, "frame {max_frames}");
            assert!(session.finish(Ok(())).is_ok());
        }
    }

    #[test]
    fn a_second_start_creates_nothing_after_a_failure_or_a_renderer() {
        let script = Script {
            failing: None,
            outdated: false,
            max_frames: None,
        };
        let created = Cell::new(0);
        let create = || {
            created.set(created.get() + 1);
            Fake::new(script, Rc::default(), Rc::default())
        };

        let mut failed = Session::<Fake>::new(None);
        assert_eq!(
            failed.fail(anyhow!("the window could not be created")),
            Flow::Exit
        );
        assert_eq!(failed.start(create), Flow::Exit);
        assert_eq!(created.get(), 0, "a renderer was created after a failure");
        assert!(!failed.has_renderer());

        let mut running = Session::<Fake>::new(None);
        assert_eq!(running.start(create), Flow::Continue);
        assert_eq!(running.start(create), Flow::Exit);
        assert_eq!(created.get(), 1, "a second renderer was created");
        assert!(running.has_renderer());
    }

    #[test]
    fn a_later_error_does_not_replace_the_first() {
        let mut session = Session::<Fake>::new(None);
        assert_eq!(session.fail(anyhow!("first")), Flow::Exit);
        assert_eq!(session.fail(anyhow!("second")), Flow::Exit);
        let error = session.finish(Err(anyhow!("event loop"))).unwrap_err();
        assert_eq!(error.to_string(), "first");
    }

    #[test]
    fn first_error_is_the_first_error_of_any_combination() {
        for bits in 0..8u32 {
            let results = (0..3).map(|index| {
                if bits & (1 << index) != 0 {
                    Err(anyhow!("{index}"))
                } else {
                    Ok(())
                }
            });
            let expected = (bits != 0).then(|| bits.trailing_zeros().to_string());
            assert_eq!(
                first_error(results).err().map(|error| error.to_string()),
                expected,
                "failing results {bits:03b}"
            );
        }
    }
}
