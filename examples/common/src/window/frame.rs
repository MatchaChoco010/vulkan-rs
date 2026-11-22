use anyhow::{Result, anyhow};
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
                // The fence is still signaled for the next frame.
                self.release(gpu, wait_stage, vk::Fence::null());
                return Err(error);
            }
            // Reset only right before the submission that signals it again.
            if let Err(error) = device.reset_fences(&[*self.in_flight]) {
                self.release(gpu, wait_stage, vk::Fence::null());
                return Err(error.into());
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
                // The fence is unsignaled now, and nothing else would signal it.
                self.release(gpu, wait_stage, *self.in_flight);
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
                Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => {
                    Ok(SwapchainStatus::Outdated)
                }
                Err(err) => Err(anyhow!("vkQueuePresentKHR failed: {err:?}")),
            }
        }
    }

    /// After a failure between the acquire and the submission of a frame: consumes the
    /// acquire's signal of `image_available`, which the next acquire signals again, and
    /// signals `fence`, unless it is null, which the next frame waits for. The acquired
    /// image stays acquired; the session stops at the first error anyway. A failure
    /// here, as on a lost device, leaves nothing that could be done instead, and the
    /// error of the frame is what the caller gets.
    unsafe fn release(&self, gpu: &Gpu, wait_stage: vk::PipelineStageFlags, fence: vk::Fence) {
        let wait_semaphores = [*self.image_available];
        let wait_stages = [wait_stage];
        let submit = vk::SubmitInfo::default()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&wait_stages);
        let _ = unsafe { gpu.device.queue_submit(gpu.queue, &[submit], fence) };
    }
}
