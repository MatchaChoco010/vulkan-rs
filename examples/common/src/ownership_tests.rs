//! Runs a setup that uses every shared helper against the fake implementation, once
//! for every call that can fail, and checks that each run destroys everything it
//! created, in an order that Vulkan allows.

use anyhow::{Context, Result};
use vulkan::{Entry, vk};
use vulkan_alloc::MemoryLocation;

use crate::commands::submit_and_wait;
use crate::device::{Gpu, find_queue_family, pick_physical_device};
use crate::fake;
use crate::image::{create_image_view, image_info_2d};
use crate::instance::create_instance;
use crate::memory::{
    create_buffer, create_image, create_mapped_buffer, read_buffer, upload_buffer, write_buffer,
};
use crate::pipeline::single_pipeline;
use crate::shader::create_shader_module;

/// What an example does on its way to its first frame, with every helper of this crate
/// that creates objects. Everything goes out of scope when it returns.
unsafe fn setup(entry: &Entry) -> Result<()> {
    unsafe {
        let instance = create_instance(entry, c"ownership test", vk::API_VERSION_1_3, &[])?;
        #[cfg(feature = "window")]
        let surface = crate::window::surface::create_wayland_surface(
            &instance,
            crate::window::WaylandWindow {
                display: std::ptr::null_mut(),
                surface: std::ptr::null_mut(),
            },
        )?;
        let (physical_device, queue_family) = pick_physical_device(&instance, |device, _| {
            Ok(find_queue_family(
                &instance,
                device,
                vk::QueueFlags::GRAPHICS,
            ))
        })?
        .context("the fake has a graphics queue")?;
        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let gpu = Gpu::create(
            instance,
            physical_device,
            queue_family,
            &vk::DeviceCreateInfo::default().queue_create_infos(&queues),
        )?;

        let vertices = upload_buffer(
            &gpu,
            "vertices",
            vk::BufferUsageFlags::VERTEX_BUFFER,
            &[1; 64],
        )?;
        let mut mapped =
            create_mapped_buffer(&gpu, "mapped", 64, vk::BufferUsageFlags::STORAGE_BUFFER, 16)?;
        mapped.bytes().fill(2);
        let readback = create_buffer(
            &gpu,
            "readback",
            64,
            vk::BufferUsageFlags::TRANSFER_DST,
            MemoryLocation::GpuToCpu,
        )?;
        write_buffer(&gpu, &readback, &[3; 16])?;
        let image = create_image(
            &gpu,
            "image",
            &image_info_2d(
                vk::Format::R8G8B8A8_UNORM,
                vk::Extent2D {
                    width: 4,
                    height: 4,
                },
                vk::ImageUsageFlags::COLOR_ATTACHMENT,
            ),
        )?;
        let linear_image = create_image(
            &gpu,
            "DRM-modifier image",
            &image_info_2d(
                vk::Format::R8G8B8A8_UNORM,
                vk::Extent2D {
                    width: 4,
                    height: 4,
                },
                vk::ImageUsageFlags::TRANSFER_SRC,
            )
            .tiling(vk::ImageTiling::DRM_FORMAT_MODIFIER_EXT),
        )?;
        let view = create_image_view(
            &gpu,
            image.image,
            vk::Format::R8G8B8A8_UNORM,
            vk::ImageAspectFlags::COLOR,
        )?;
        let module = create_shader_module(&gpu, &[0; 20])?;
        let layout = gpu.own(
            gpu.device
                .create_pipeline_layout(&vk::PipelineLayoutCreateInfo::default(), None)?,
        );
        let stages = [vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlagBits::VERTEX)
            .module(*module)
            .name(c"main")];
        let info = vk::GraphicsPipelineCreateInfo::default()
            .stages(&stages)
            .layout(*layout);
        let pipeline = single_pipeline(
            &gpu,
            gpu.device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[info], None),
        )?;
        submit_and_wait(&gpu, |_| Ok(()))?;
        read_buffer(&gpu, &readback)?;

        #[cfg(feature = "window")]
        {
            let swapchain = crate::window::swapchain::Swapchain::new(
                &gpu,
                surface.handle,
                vk::SurfaceFormatKHR::default(),
                vk::PresentModeKHR::FIFO,
                vk::Extent2D {
                    width: 64,
                    height: 64,
                },
            )?;
            let frame = crate::window::frame::Frame::new(&gpu)?;
            drop((frame, swapchain));
        }
        drop((pipeline, view, linear_image, vertices));
        Ok(())
    }
}

#[test]
fn a_setup_that_fails_at_any_call_destroys_everything_it_created_in_order() {
    let entry = fake::entry();
    let mut failing_runs = 0;
    for fail_at in 0.. {
        fake::reset(Some(fail_at));
        let result = unsafe { setup(&entry) };
        let report = fake::report();
        assert!(
            report.leaked.is_empty(),
            "failing call {fail_at}: leaked {:?}",
            report.leaked
        );
        assert!(
            report.misuse.is_empty(),
            "failing call {fail_at}: {:?}",
            report.misuse
        );
        if !report.failed {
            assert!(result.is_ok(), "a run without a failure fails: {result:?}");
            break;
        }
        assert!(
            result.is_err(),
            "failing call {fail_at} did not make the setup fail"
        );
        failing_runs += 1;
    }
    assert!(failing_runs >= 20, "only {failing_runs} calls can fail");
}

#[test]
fn a_successful_setup_creates_and_destroys_every_object() {
    let entry = fake::entry();
    fake::reset(None);
    unsafe { setup(&entry) }.unwrap();
    let report = fake::report();
    assert!(report.created >= 15, "only {} objects", report.created);
    assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
    assert!(report.misuse.is_empty(), "{:?}", report.misuse);
}

#[test]
fn a_lost_device_still_destroys_everything_and_nothing_in_use() {
    let entry = fake::entry();
    fake::reset(None);
    fake::lose_device();
    let result = unsafe { setup(&entry) };
    let report = fake::report();
    assert!(result.is_err(), "the wait for the submission succeeded");
    assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
    assert!(report.misuse.is_empty(), "{:?}", report.misuse);
}

/// Draws a frame whose `fail_at`-th call that can fail, counted from the start of the
/// draw, fails, and with `and_later` every call after it too, then draws two more frames
/// without failures. Returns whether the first draw reached the failing call, and how
/// the draws went.
#[cfg(feature = "window")]
unsafe fn draw_after_failure(
    entry: &Entry,
    fail_at: u32,
    and_later: bool,
) -> Result<(bool, Result<()>, Vec<Result<()>>)> {
    let arm = || {
        if and_later {
            fake::fail_all_from_now(fail_at);
        } else {
            fake::fail_from_now(Some(fail_at));
        }
    };
    let draws = unsafe { draw_after(entry, arm) }?;
    Ok((draws.failed, draws.first, draws.later))
}

/// How the draws of [`draw_after`] went.
#[cfg(feature = "window")]
struct Draws {
    /// Whether the first draw reached a failing call.
    failed: bool,
    first: Result<()>,
    /// The two draws after it.
    later: Vec<Result<()>>,
    /// How many calls that can fail the two later draws made.
    later_calls: u32,
}

/// Sets a frame up without failures, calls `arm`, which tells the fake what is to fail,
/// and draws; then draws two more frames without failing calls.
#[cfg(feature = "window")]
unsafe fn draw_after(entry: &Entry, arm: impl FnOnce()) -> Result<Draws> {
    unsafe {
        let instance = create_instance(entry, c"frame test", vk::API_VERSION_1_3, &[])?;
        let surface = crate::window::surface::create_wayland_surface(
            &instance,
            crate::window::WaylandWindow {
                display: std::ptr::null_mut(),
                surface: std::ptr::null_mut(),
            },
        )?;
        let (physical_device, queue_family) = pick_physical_device(&instance, |device, _| {
            Ok(find_queue_family(
                &instance,
                device,
                vk::QueueFlags::GRAPHICS,
            ))
        })?
        .context("the fake has a graphics queue")?;
        let priorities = [1.0];
        let queues = [vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family)
            .queue_priorities(&priorities)];
        let gpu = Gpu::create(
            instance,
            physical_device,
            queue_family,
            &vk::DeviceCreateInfo::default().queue_create_infos(&queues),
        )?;
        let swapchain = crate::window::swapchain::Swapchain::new(
            &gpu,
            surface.handle,
            vk::SurfaceFormatKHR::default(),
            vk::PresentModeKHR::FIFO,
            vk::Extent2D {
                width: 64,
                height: 64,
            },
        )?;
        let frame = crate::window::frame::Frame::new(&gpu)?;
        let draw = || {
            frame
                .draw(
                    &gpu,
                    &swapchain,
                    vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
                    |_, _| {},
                )
                .map(|_| ())
        };
        arm();
        let first = draw();
        let failed = fake::report().failed;
        fake::fail_from_now(None);
        let calls_before = fake::calls();
        let later = vec![draw(), draw()];
        let later_calls = fake::calls() - calls_before;
        gpu.device.device_wait_idle()?;
        Ok(Draws {
            failed,
            first,
            later,
            later_calls,
        })
    }
}

#[cfg(feature = "window")]
#[test]
fn a_frame_that_fails_at_any_call_leaves_the_next_frames_able_to_run() {
    let entry = fake::entry();
    let mut failing_runs = 0;
    for fail_at in 0.. {
        fake::reset(None);
        let (failed, first, later) = unsafe { draw_after_failure(&entry, fail_at, false) }.unwrap();
        let report = fake::report();
        assert!(
            report.misuse.is_empty(),
            "failing call {fail_at} of the frame: {:?}",
            report.misuse
        );
        assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
        if !failed {
            assert!(first.is_ok(), "a frame without a failure fails: {first:?}");
            break;
        }
        assert!(
            first.is_err(),
            "failing call {fail_at} did not fail the frame"
        );
        for (index, result) in later.iter().enumerate() {
            assert!(
                result.is_ok(),
                "after failing call {fail_at}, frame {} fails: {result:?}",
                index + 1
            );
        }
        failing_runs += 1;
    }
    assert!(
        failing_runs >= 7,
        "only {failing_runs} calls of a frame can fail"
    );
}

/// The calls of a frame that can fail, counted as [`draw_after_failure`] counts them:
/// the last one is the present.
#[cfg(feature = "window")]
fn failing_calls_of_a_frame(entry: &Entry) -> u32 {
    (0..)
        .find(|&fail_at| {
            fake::reset(None);
            let (failed, _, _) = unsafe { draw_after_failure(entry, fail_at, false) }.unwrap();
            !failed
        })
        .expect("a frame makes a finite number of calls")
}

/// After `VK_ERROR_UNKNOWN`, Vulkan does not say what state the fences of a failed
/// `vkResetFences` are in. The frame has to go on when the fence was reset all the same,
/// as it does when the reset failed for lack of memory and left the fence as it was.
#[cfg(feature = "window")]
#[test]
fn a_frame_goes_on_after_a_failed_fence_reset_that_reset_the_fence_all_the_same() {
    let entry = fake::entry();
    for fail_at in 0..failing_calls_of_a_frame(&entry) {
        fake::reset(None);
        fake::failed_resets_unsignal_the_fences();
        let (failed, first, later) = unsafe { draw_after_failure(&entry, fail_at, false) }.unwrap();
        let report = fake::report();
        assert!(
            failed && first.is_err(),
            "failing call {fail_at}: {first:?}"
        );
        assert!(
            report.misuse.is_empty(),
            "failing call {fail_at} of the frame: {:?}",
            report.misuse
        );
        assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
        for (index, result) in later.iter().enumerate() {
            assert!(
                result.is_ok(),
                "after failing call {fail_at}, frame {} fails: {result:?}",
                index + 1
            );
        }
    }
}

/// A fence that a failed reset left unsignaled and that cannot be replaced either
/// leaves nothing that the next frame could wait for: the frame is unusable, and its
/// later draws return an error without calling Vulkan. That is so for exactly one
/// failing call of a frame, the reset; after any other failing call no fence is needed.
#[cfg(feature = "window")]
#[test]
fn a_frame_whose_fence_can_be_neither_reset_nor_replaced_is_unusable() {
    let entry = fake::entry();
    let mut unusable = 0;
    for fail_at in 0..failing_calls_of_a_frame(&entry) {
        fake::reset(None);
        fake::failed_resets_unsignal_the_fences();
        let arm = || {
            fake::fail_from_now(Some(fail_at));
            fake::fail_fence_creations();
        };
        let Draws {
            failed,
            first,
            later,
            later_calls,
        } = unsafe { draw_after(&entry, arm) }.unwrap();
        let report = fake::report();
        assert!(
            failed && first.is_err(),
            "failing call {fail_at}: {first:?}"
        );
        assert!(
            report.misuse.is_empty(),
            "failing call {fail_at} of the frame: {:?}",
            report.misuse
        );
        assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
        if later.iter().all(Result::is_ok) {
            continue;
        }
        unusable += 1;
        assert!(
            format!("{first:?}").contains("ERROR_UNKNOWN"),
            "failing call {fail_at}: the frame reports the error of the reset: {first:?}"
        );
        assert!(
            later.iter().all(Result::is_err),
            "failing call {fail_at}: {later:?}"
        );
        assert_eq!(
            later_calls, 0,
            "failing call {fail_at}: an unusable frame calls nothing"
        );
    }
    assert_eq!(unusable, 1, "only a failed reset needs a new fence");
}

/// What a frame can do after a present failed.
#[cfg(feature = "window")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AfterFailedPresent {
    /// The draw reports an outdated swapchain, and the next frames run.
    Outdated,
    /// The draw returns the error, and the next frames run.
    Recovers,
    /// The draw returns the error, and the next frames fail without calling Vulkan.
    Unusable,
}

/// What the specification says about each error code that the registry lists for
/// `vkQueuePresentKHR`: whether the queue operations of the present, the wait for the
/// semaphore among them, were enqueued (both where it does not say), and what a frame
/// can do then.
///
/// - `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`: not enqueued, and
///   everything that the call referenced is as it was.
/// - `VK_ERROR_OUT_OF_DATE_KHR`, `VK_ERROR_SURFACE_LOST_KHR`,
///   `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`,
///   `VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT`: rejected by the presentation engine, and
///   still enqueued.
/// - `VK_ERROR_DEVICE_LOST`: returned when the implementation cannot guarantee that
///   everything is as it was. `VK_ERROR_UNKNOWN`, `VK_ERROR_VALIDATION_FAILED`: nothing
///   is said.
#[cfg(feature = "window")]
const PRESENT_ERRORS: [(vk::VkResult, &[bool], AfterFailedPresent); 9] = [
    (
        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY,
        &[false],
        AfterFailedPresent::Recovers,
    ),
    (
        vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
        &[false],
        AfterFailedPresent::Recovers,
    ),
    (
        vk::VkResult::ERROR_DEVICE_LOST,
        &[false, true],
        AfterFailedPresent::Unusable,
    ),
    (
        vk::VkResult::ERROR_OUT_OF_DATE_KHR,
        &[true],
        AfterFailedPresent::Outdated,
    ),
    (
        vk::VkResult::ERROR_SURFACE_LOST_KHR,
        &[true],
        AfterFailedPresent::Recovers,
    ),
    (
        vk::VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
        &[true],
        AfterFailedPresent::Recovers,
    ),
    (
        vk::VkResult::ERROR_UNKNOWN,
        &[false, true],
        AfterFailedPresent::Unusable,
    ),
    (
        vk::VkResult::ERROR_VALIDATION_FAILED,
        &[false, true],
        AfterFailedPresent::Unusable,
    ),
    (
        vk::VkResult::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT,
        &[true],
        AfterFailedPresent::Recovers,
    ),
];

/// A present fails with each error code that the registry lists for it, in each state
/// that the specification allows the semaphore and the image to be left in. The frame
/// must not wait for a semaphore that has no signal pending, and the next frames run,
/// or fail without calling Vulkan, as [`PRESENT_ERRORS`] says.
#[cfg(feature = "window")]
#[test]
fn a_frame_does_what_each_error_code_of_a_present_asks_for() {
    let sorted = |codes: &[vk::VkResult]| {
        let mut codes = codes.iter().map(|code| code.as_raw()).collect::<Vec<_>>();
        codes.sort_unstable();
        codes
    };
    assert_eq!(
        sorted(&PRESENT_ERRORS.map(|(error, _, _)| error)),
        sorted(vk::command::QUEUE_PRESENT_KHR.error_codes),
        "the error codes that the registry lists for vkQueuePresentKHR"
    );
    let entry = fake::entry();
    let present = failing_calls_of_a_frame(&entry) - 1;
    for (error, enqueued_states, expected) in PRESENT_ERRORS {
        for &enqueued in enqueued_states {
            fake::reset(None);
            fake::fail_presents_with(error, enqueued);
            let arm = || fake::fail_from_now(Some(present));
            let Draws {
                failed,
                first,
                later,
                later_calls,
            } = unsafe { draw_after(&entry, arm) }.unwrap();
            let report = fake::report();
            let case = format!("{error:?}, enqueued: {enqueued}");
            assert!(failed, "{case}: the present did not fail");
            assert!(report.misuse.is_empty(), "{case}: {:?}", report.misuse);
            assert!(
                report.leaked.is_empty(),
                "{case}: leaked {:?}",
                report.leaked
            );
            assert_eq!(
                first.is_ok(),
                expected == AfterFailedPresent::Outdated,
                "{case}: {first:?}"
            );
            for (index, result) in later.iter().enumerate() {
                assert_eq!(
                    result.is_ok(),
                    expected != AfterFailedPresent::Unusable,
                    "{case}: frame {} after the failure: {result:?}",
                    index + 1
                );
            }
            if expected == AfterFailedPresent::Unusable {
                assert_eq!(later_calls, 0, "{case}: an unusable frame calls nothing");
            }
        }
    }
}

/// What the specification says about each error code that the registry lists for
/// `vkQueueSubmit`: whether the submission, with its wait for the semaphore and its
/// signal of the fence, was enqueued (both where it does not say), and whether a frame
/// can go on.
///
/// - `VK_ERROR_OUT_OF_HOST_MEMORY`, `VK_ERROR_OUT_OF_DEVICE_MEMORY`: not enqueued, and
///   everything that the call referenced is as it was.
/// - `VK_ERROR_DEVICE_LOST`: returned when the implementation cannot guarantee that.
///   `VK_ERROR_UNKNOWN`, `VK_ERROR_VALIDATION_FAILED`: nothing is said.
#[cfg(feature = "window")]
const SUBMIT_ERRORS: [(vk::VkResult, &[bool], bool); 5] = [
    (vk::VkResult::ERROR_OUT_OF_HOST_MEMORY, &[false], true),
    (vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY, &[false], true),
    (vk::VkResult::ERROR_DEVICE_LOST, &[false, true], false),
    (vk::VkResult::ERROR_UNKNOWN, &[false, true], false),
    (vk::VkResult::ERROR_VALIDATION_FAILED, &[false, true], false),
];

/// The submission of a frame fails with each error code that the registry lists for
/// it, in each state that the specification allows the semaphores and the fence to be
/// left in. The frame must not wait for a semaphore that has no signal pending or give
/// its fence to a second submission, and the next frames run, or fail without calling
/// Vulkan, as [`SUBMIT_ERRORS`] says.
#[cfg(feature = "window")]
#[test]
fn a_frame_does_what_each_error_code_of_a_submission_asks_for() {
    let sorted = |codes: &[vk::VkResult]| {
        let mut codes = codes.iter().map(|code| code.as_raw()).collect::<Vec<_>>();
        codes.sort_unstable();
        codes
    };
    assert_eq!(
        sorted(&SUBMIT_ERRORS.map(|(error, _, _)| error)),
        sorted(vk::command::QUEUE_SUBMIT.error_codes),
        "the error codes that the registry lists for vkQueueSubmit"
    );
    let entry = fake::entry();
    let submit = failing_calls_of_a_frame(&entry) - 2;
    for (error, enqueued_states, goes_on) in SUBMIT_ERRORS {
        for &enqueued in enqueued_states {
            fake::reset(None);
            fake::fail_submits_with(error, enqueued);
            let arm = || fake::fail_from_now(Some(submit));
            let Draws {
                failed,
                first,
                later,
                later_calls,
            } = unsafe { draw_after(&entry, arm) }.unwrap();
            let report = fake::report();
            let case = format!("{error:?}, enqueued: {enqueued}");
            assert!(failed, "{case}: the submission did not fail");
            assert!(
                format!("{first:?}").contains(&format!("{error:?}")),
                "{case}: the frame reports the error of the submission: {first:?}"
            );
            assert!(report.misuse.is_empty(), "{case}: {:?}", report.misuse);
            assert!(
                report.leaked.is_empty(),
                "{case}: leaked {:?}",
                report.leaked
            );
            for (index, result) in later.iter().enumerate() {
                assert_eq!(
                    result.is_ok(),
                    goes_on,
                    "{case}: frame {} after the failure: {result:?}",
                    index + 1
                );
            }
            if !goes_on {
                assert_eq!(later_calls, 0, "{case}: an unusable frame calls nothing");
            }
        }
    }
}

#[cfg(feature = "window")]
#[test]
fn a_frame_whose_recovery_fails_too_reports_its_first_error_and_never_hangs() {
    let entry = fake::entry();
    for fail_at in 0.. {
        fake::reset(None);
        let (failed, alone, _) = unsafe { draw_after_failure(&entry, fail_at, false) }.unwrap();
        if !failed {
            break;
        }
        fake::reset(None);
        let (_, first, later) = unsafe { draw_after_failure(&entry, fail_at, true) }.unwrap();
        let report = fake::report();
        assert!(
            report.misuse.is_empty(),
            "failing calls from {fail_at} on: {:?}",
            report.misuse
        );
        assert!(report.leaked.is_empty(), "leaked {:?}", report.leaked);
        assert_eq!(
            first.map_err(|error| error.to_string()),
            alone.map_err(|error| error.to_string()),
            "failing calls from {fail_at} on: not the error of the first failure"
        );
        drop(later);
    }
}
