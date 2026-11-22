use anyhow::{Result, anyhow, bail};
use std::cell::Cell;
use vulkan::vk;

use super::swapchain::Swapchain;
use crate::device::{Gpu, Owned};

/// What [`Frame::draw`] found out about the swapchain.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwapchainStatus {
    Current,
    /// The swapchain is out of date or suboptimal for the surface, and has to be
    /// recreated before the next frame.
    Outdated,
}

/// The command buffer and synchronization of the one frame in flight: each frame waits
/// until the device has finished the one before.
///
/// A frame that fails after its image was acquired puts its fence and semaphores back
/// into a state that the next frame can start from, with work that the next frame
/// waits for. Two limits remain:
///
/// - The image stays acquired, since it was never presented. A swapchain with `S`
///   images keeps working while at most `S - minImageCount` images are acquired, so
///   after that many failed frames the next acquire may wait for ever; recreate the
///   swapchain after a failure.
/// - When the work that restores that state cannot be submitted either, as on a lost
///   device or for lack of memory, the frame is broken: every later [`Frame::draw`]
///   returns an error instead of waiting for a fence that nothing will signal.
///
/// The device must be done with the frame before it is dropped. The fields drop in
/// order, the reverse of creation.
pub struct Frame {
    /// Signaled by the submission, waited on before the command buffer is recorded again.
    in_flight: Owned<vk::Fence>,
    /// Signaled by the acquire, waited on by the submission.
    image_available: Owned<vk::Semaphore>,
    command_buffer: vk::CommandBuffer,
    /// Kept for the command buffer, which destroying the pool frees.
    _command_pool: Owned<vk::CommandPool>,
    /// An earlier failure could not be repaired.
    broken: Cell<bool>,
}

impl Frame {
    pub unsafe fn new(gpu: &Gpu) -> Result<Self> {
        unsafe {
            let device = &gpu.device;
            let command_pool = gpu.own(
                device.create_command_pool(
                    &vk::CommandPoolCreateInfo::default()
                        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                        .queue_family_index(gpu.queue_family),
                    None,
                )?,
            );
            let command_buffer = device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .command_pool(*command_pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )?
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("vkAllocateCommandBuffers returned no command buffer"))?;
            let image_available =
                gpu.own(device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?);
            // Signaled, so that the first frame does not wait for a submission that never
            // happened.
            let in_flight = gpu.own(device.create_fence(
                &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED),
                None,
            )?);
            Ok(Self {
                in_flight,
                image_available,
                command_buffer,
                _command_pool: command_pool,
                broken: Cell::new(false),
            })
        }
    }

    /// Acquires an image of `swapchain`, records the frame into a command buffer with
    /// `record`, submits it and presents the image.
    ///
    /// `record` gets the command buffer, which is already begun, and the index of the
    /// acquired image in `swapchain.images`. `wait_stage` is where the commands wait
    /// for the image: until the presentation engine has released it, nothing at or
    /// after that stage may touch it, a layout transition included.
    pub unsafe fn draw(
        &self,
        gpu: &Gpu,
        swapchain: &Swapchain,
        wait_stage: vk::PipelineStageFlags,
        record: impl FnOnce(vk::CommandBuffer, usize),
    ) -> Result<SwapchainStatus> {
        unsafe {
            if self.broken.get() {
                bail!("an earlier frame failed in a way that left this frame unusable");
            }
            let device = &gpu.device;
            device.wait_for_fences(&[*self.in_flight], true, u64::MAX)?;

            let (image_index, acquired_suboptimal) = match device.acquire_next_image_khr(
                swapchain.handle,
                u64::MAX,
                *self.image_available,
                vk::Fence::null(),
            ) {
                Ok(Some(acquired)) => acquired,
                Ok(None) => return Ok(SwapchainStatus::Current),
                Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => {
                    return Ok(SwapchainStatus::Outdated);
                }
                Err(err) => return Err(anyhow!("vkAcquireNextImageKHR failed: {err:?}")),
            };
            let render_finished = *swapchain.render_finished[image_index as usize];

            // From the acquire on, every way out leaves work behind that waits for
            // `image_available` and that the next frame's wait for the fence covers.
            if let Err(error) = device.reset_fences(&[*self.in_flight]) {
                // The fence may still be signaled, so it cannot mark the work.
                self.recover(gpu, *self.image_available, wait_stage, vk::Fence::null());
                return Err(error.into());
            }
            let recorded = (|| -> Result<()> {
                device.reset_command_buffer(
                    self.command_buffer,
                    vk::CommandBufferResetFlags::empty(),
                )?;
                device.begin_command_buffer(
                    self.command_buffer,
                    &vk::CommandBufferBeginInfo::default(),
                )?;
                record(self.command_buffer, image_index as usize);
                device.end_command_buffer(self.command_buffer)?;
                Ok(())
            })();
            if let Err(error) = recorded {
                self.recover(gpu, *self.image_available, wait_stage, *self.in_flight);
                return Err(error);
            }

            let wait_semaphores = [*self.image_available];
            let wait_stages = [wait_stage];
            let command_buffers = [self.command_buffer];
            let signal_semaphores = [render_finished];
            let submit = vk::SubmitInfo::default()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores);
            if let Err(error) = device.queue_submit(gpu.queue, &[submit], *self.in_flight) {
                self.recover(gpu, *self.image_available, wait_stage, *self.in_flight);
                return Err(error.into());
            }

            let swapchains = [swapchain.handle];
            let image_indices = [image_index];
            let present = vk::PresentInfoKHR::default()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&image_indices);
            match device.queue_present_khr(gpu.queue, &present) {
                Ok(present_suboptimal) if !acquired_suboptimal && !present_suboptimal => {
                    Ok(SwapchainStatus::Current)
                }
                Ok(_) => Ok(SwapchainStatus::Outdated),
                // A present that the presentation engine rejects still waits for
                // `render_finished`.
                Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => {
                    Ok(SwapchainStatus::Outdated)
                }
                Err(err) if err == vk::VkResult::ERROR_SURFACE_LOST_KHR => {
                    Err(anyhow!("vkQueuePresentKHR failed: {err:?}"))
                }
                Err(err) => {
                    // A present that could not be enqueued left `render_finished`
                    // signaled, and the next frame on this image signals it again.
                    self.recover(
                        gpu,
                        render_finished,
                        vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                        vk::Fence::null(),
                    );
                    Err(anyhow!("vkQueuePresentKHR failed: {err:?}"))
                }
            }
        }
    }

    /// After a failure: submits work that waits for `semaphore`, consuming its signal,
    /// and signals `fence`, which the next frame waits for. Without a fence it waits
    /// for the queue to be idle instead, so that the wait is over before the semaphore
    /// is signaled again. If that cannot be done either, as on a lost device, the frame
    /// is broken; the error of the frame is still what its caller gets.
    unsafe fn recover(
        &self,
        gpu: &Gpu,
        semaphore: vk::Semaphore,
        wait_stage: vk::PipelineStageFlags,
        fence: vk::Fence,
    ) {
        let wait_semaphores = [semaphore];
        let wait_stages = [wait_stage];
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages);
        let recovered = unsafe {
            gpu.device
                .queue_submit(gpu.queue, &[submit], fence)
                .and_then(|()| {
                    if fence.is_null() {
                        gpu.device.queue_wait_idle(gpu.queue)
                    } else {
                        Ok(())
                    }
                })
        };
        if recovered.is_err() {
            self.broken.set(true);
        }
    }
}
