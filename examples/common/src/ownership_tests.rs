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
        if and_later {
            fake::fail_all_from_now(fail_at);
        } else {
            fake::fail_from_now(Some(fail_at));
        }
        let first = draw();
        let failed = fake::report().failed;
        fake::fail_from_now(None);
        let later = vec![draw(), draw()];
        gpu.device.device_wait_idle()?;
        Ok((failed, first, later))
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
