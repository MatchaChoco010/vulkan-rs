use anyhow::{Result, anyhow};
use vulkan::{Device, vk};

use super::swapchain::Swapchain;
use crate::device::Gpu;

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
pub struct Frame {
    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
    /// Signaled by the acquire, waited on by the submission.
    image_available: vk::Semaphore,
    /// Signaled by the submission, waited on before the command buffer is recorded again.
    in_flight: vk::Fence,
}

impl Frame {
    pub unsafe fn new(device: &Device, queue_family: u32) -> Result<Self> {
        unsafe {
            let command_pool = device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                    .queue_family_index(queue_family),
                None,
            )?;
            let command_buffer = device
                .allocate_command_buffers(
                    &vk::CommandBufferAllocateInfo::default()
                        .command_pool(command_pool)
                        .level(vk::CommandBufferLevel::PRIMARY)
                        .command_buffer_count(1),
                )?
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("vkAllocateCommandBuffers returned no command buffer"))?;
            let image_available =
                device.create_semaphore(&vk::SemaphoreCreateInfo::default(), None)?;
            // Signaled, so that the first frame does not wait for a submission that never
            // happened.
            let in_flight = device.create_fence(
                &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED),
                None,
            )?;
            Ok(Self {
                command_pool,
                command_buffer,
                image_available,
                in_flight,
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
            device.wait_for_fences(&[self.in_flight], true, u64::MAX)?;

            let (image_index, acquired_suboptimal) = match device.acquire_next_image_khr(
                swapchain.handle,
                u64::MAX,
                self.image_available,
                vk::Fence::null(),
            ) {
                Ok(Some(acquired)) => acquired,
                Ok(None) => return Ok(SwapchainStatus::Current),
                Err(err) if err == vk::VkResult::ERROR_OUT_OF_DATE_KHR => {
                    return Ok(SwapchainStatus::Outdated);
                }
                Err(err) => return Err(anyhow!("vkAcquireNextImageKHR failed: {err:?}")),
            };
            let render_finished = swapchain.render_finished[image_index as usize];
            // Reset only once an image was acquired: a frame that returns before the
            // submission leaves the fence signaled for the next one.
            device.reset_fences(&[self.in_flight])?;

            device
                .reset_command_buffer(self.command_buffer, vk::CommandBufferResetFlags::empty())?;
            device.begin_command_buffer(
                self.command_buffer,
                &vk::CommandBufferBeginInfo::default(),
            )?;
            record(self.command_buffer, image_index as usize);
            device.end_command_buffer(self.command_buffer)?;

            let wait_semaphores = [self.image_available];
            let wait_stages = [wait_stage];
            let command_buffers = [self.command_buffer];
            let signal_semaphores = [render_finished];
            let submit = vk::SubmitInfo::default()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores);
            device.queue_submit(gpu.queue, &[submit], self.in_flight)?;

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

    /// The device must be done with the frame.
    pub unsafe fn destroy(self, device: &Device) {
        unsafe {
            device.destroy_semaphore(self.image_available, None);
            device.destroy_fence(self.in_flight, None);
            // Destroying the pool also frees the command buffer.
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}
