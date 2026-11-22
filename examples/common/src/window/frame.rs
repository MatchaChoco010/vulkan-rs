use anyhow::{Result, anyhow, bail};
use std::cell::{Cell, RefCell};
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

/// What [`Frame::draw`] does after `vkAcquireNextImageKHR` returned an error.
///
/// The specification says: "If `vkAcquireNextImageKHR` does not successfully acquire an
/// image, `semaphore` and `fence` are unaffected." So no error code of the command
/// leaves anything to repair.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AfterAcquireError {
    /// The swapchain no longer matches the surface: the frame reports
    /// [`SwapchainStatus::Outdated`].
    RecreateSwapchain,
    /// The frame returns the error and is as it was before the call.
    ReturnError,
    /// The result is not an error code of the command, so what the command did to the
    /// semaphore is not known: the frame returns the error and is broken.
    Broken,
}

/// The action for each result that `vkAcquireNextImageKHR` can fail with.
pub fn after_acquire_error(error: vk::VkResult) -> AfterAcquireError {
    match error {
        vk::VkResult::ERROR_OUT_OF_DATE_KHR => AfterAcquireError::RecreateSwapchain,
        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY
        | vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY
        | vk::VkResult::ERROR_DEVICE_LOST
        | vk::VkResult::ERROR_SURFACE_LOST_KHR
        | vk::VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
        | vk::VkResult::ERROR_UNKNOWN
        | vk::VkResult::ERROR_VALIDATION_FAILED => AfterAcquireError::ReturnError,
        _ => AfterAcquireError::Broken,
    }
}

/// What [`Frame::draw`] does after `vkQueuePresentKHR` returned an error. It depends on
/// whether the queue operations of the present, the wait for `render_finished` among
/// them, were enqueued.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AfterPresentError {
    /// The presentation engine rejected the request because the swapchain no longer
    /// matches the surface. The queue operations were still enqueued, so the wait for
    /// `render_finished` executes: the frame reports [`SwapchainStatus::Outdated`].
    RecreateSwapchain,
    /// The presentation engine rejected the request for a reason that a new swapchain
    /// does not remove. The queue operations were still enqueued, so the wait for
    /// `render_finished` executes and nothing is left to repair: the frame returns the
    /// error.
    ReturnError,
    /// The queue operations were not enqueued, and the implementation left everything
    /// that the call referenced as it was: `render_finished` is still signaled, and
    /// the image is still acquired. The frame submits work that waits for the
    /// semaphore, so that the next frame on this image can signal it again, and
    /// returns the error.
    ConsumeSignal,
    /// The device is lost, or the result says nothing about the state of the
    /// semaphore. A wait for a semaphore that may have no signal pending is not valid,
    /// so the frame submits nothing: it returns the error and is broken.
    Broken,
}

/// The action for each result that `vkQueuePresentKHR` can fail with.
///
/// The specification says: "If `vkQueuePresentKHR` fails to enqueue the corresponding
/// set of queue operations, it may return `VK_ERROR_OUT_OF_HOST_MEMORY` or
/// `VK_ERROR_OUT_OF_DEVICE_MEMORY`. If it does, the implementation must ensure that the
/// state and contents of any resources or synchronization primitives referenced is
/// unaffected by the call or its failure. If `vkQueuePresentKHR` fails in such a way
/// that the implementation is unable to make that guarantee, the implementation must
/// return `VK_ERROR_DEVICE_LOST`. However, if the presentation request is rejected by
/// the presentation engine with an error `VK_ERROR_OUT_OF_DATE_KHR`,
/// `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT`,
/// `VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT`, or `VK_ERROR_SURFACE_LOST_KHR`, the set of
/// queue operations are still considered to be enqueued and thus any semaphore wait
/// operation specified in `VkPresentInfoKHR` will execute when the corresponding queue
/// operation is complete."
///
/// It says nothing of that kind for `VK_ERROR_UNKNOWN` and
/// `VK_ERROR_VALIDATION_FAILED`.
pub fn after_present_error(error: vk::VkResult) -> AfterPresentError {
    match error {
        vk::VkResult::ERROR_OUT_OF_DATE_KHR => AfterPresentError::RecreateSwapchain,
        vk::VkResult::ERROR_SURFACE_LOST_KHR
        | vk::VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT
        | vk::VkResult::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT => AfterPresentError::ReturnError,
        vk::VkResult::ERROR_OUT_OF_HOST_MEMORY | vk::VkResult::ERROR_OUT_OF_DEVICE_MEMORY => {
            AfterPresentError::ConsumeSignal
        }
        _ => AfterPresentError::Broken,
    }
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
/// - When that state cannot be restored, the frame is broken: every later
///   [`Frame::draw`] returns an error instead of waiting for a fence that nothing will
///   signal. That is so when the work that restores the state cannot be submitted or
///   the fence cannot be replaced, as on a lost device or for lack of memory, and when
///   a present or an acquire fails with a result that says nothing about the state of
///   the semaphores ([`AfterPresentError::Broken`], [`AfterAcquireError::Broken`]).
///
/// The device must be done with the frame before it is dropped. The fields drop in
/// order, the reverse of creation.
pub struct Frame {
    /// Signaled by the submission, waited on before the command buffer is recorded again.
    /// Replaced by a new fence when it could not be reset.
    in_flight: RefCell<Owned<vk::Fence>>,
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
                in_flight: RefCell::new(in_flight),
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
            let in_flight = **self.in_flight.borrow();
            device.wait_for_fences(&[in_flight], true, u64::MAX)?;

            let (image_index, acquired_suboptimal) = match device.acquire_next_image_khr(
                swapchain.handle,
                u64::MAX,
                *self.image_available,
                vk::Fence::null(),
            ) {
                Ok(Some(acquired)) => acquired,
                Ok(None) => return Ok(SwapchainStatus::Current),
                Err(err) => {
                    match after_acquire_error(err) {
                        AfterAcquireError::RecreateSwapchain => {
                            return Ok(SwapchainStatus::Outdated);
                        }
                        AfterAcquireError::ReturnError => {}
                        AfterAcquireError::Broken => self.broken.set(true),
                    }
                    return Err(anyhow!("vkAcquireNextImageKHR failed: {err:?}"));
                }
            };
            let render_finished = *swapchain.render_finished[image_index as usize];

            // From the acquire on, every way out leaves work behind that waits for
            // `image_available` and that the next frame's wait for the fence covers.
            if let Err(error) = device.reset_fences(&[in_flight]) {
                // After VK_ERROR_OUT_OF_DEVICE_MEMORY the fence is as it was, still
                // signaled: errors for lack of memory do not modify existing objects.
                // After VK_ERROR_UNKNOWN or VK_ERROR_VALIDATION_FAILED, Vulkan does not
                // say what state it is in. A signaled fence cannot mark the work that
                // consumes the signal of the acquire, and nothing would signal an
                // unsignaled one, so that the next frame would wait for it for ever.
                // So the signal is consumed without the fence, and the next frame gets
                // a new fence, after any error: that is simpler than telling the errors
                // apart, and right for each of them.
                self.recover(gpu, *self.image_available, wait_stage, vk::Fence::null());
                self.replace_fence(gpu);
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
                self.recover(gpu, *self.image_available, wait_stage, in_flight);
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
            if let Err(error) = device.queue_submit(gpu.queue, &[submit], in_flight) {
                self.recover(gpu, *self.image_available, wait_stage, in_flight);
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
                Err(err) => {
                    match after_present_error(err) {
                        AfterPresentError::RecreateSwapchain => {
                            return Ok(SwapchainStatus::Outdated);
                        }
                        AfterPresentError::ReturnError => {}
                        AfterPresentError::ConsumeSignal => self.recover(
                            gpu,
                            render_finished,
                            vk::PipelineStageFlags::BOTTOM_OF_PIPE,
                            vk::Fence::null(),
                        ),
                        AfterPresentError::Broken => self.broken.set(true),
                    }
                    Err(anyhow!("vkQueuePresentKHR failed: {err:?}"))
                }
            }
        }
    }

    /// After `vkResetFences` failed: replaces the fence by a new one that is signaled,
    /// as the fence of a frame is before its first submission. The old fence can be
    /// destroyed whether the failed call reset it or not, because the frame has waited
    /// for the only submission that it was given to. If no fence can be created, the
    /// frame is broken.
    unsafe fn replace_fence(&self, gpu: &Gpu) {
        let created = unsafe {
            gpu.device.create_fence(
                &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED),
                None,
            )
        };
        match created {
            Ok(fence) => drop(self.in_flight.replace(unsafe { gpu.own(fence) })),
            Err(_) => self.broken.set(true),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    use vulkan::vk::VkResult;

    fn listed<T: Copy>(actions: &[(VkResult, T)]) -> BTreeMap<i32, T> {
        let by_code = actions
            .iter()
            .map(|(code, action)| (code.as_raw(), *action))
            .collect::<BTreeMap<_, _>>();
        assert_eq!(by_code.len(), actions.len(), "a code is listed twice");
        by_code
    }

    fn sorted(codes: &[VkResult]) -> Vec<i32> {
        let mut codes = codes.iter().map(|code| code.as_raw()).collect::<Vec<_>>();
        codes.sort_unstable();
        codes
    }

    /// The action for every error code that the registry lists for
    /// `vkQueuePresentKHR`. An error code that the registry gains fails the comparison
    /// of the two lists until its action is decided from the specification.
    #[test]
    fn every_error_code_of_present_has_the_action_that_the_specification_asks_for() {
        let expected = listed(&[
            (
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                AfterPresentError::ConsumeSignal,
            ),
            (
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                AfterPresentError::ConsumeSignal,
            ),
            (VkResult::ERROR_DEVICE_LOST, AfterPresentError::Broken),
            (
                VkResult::ERROR_OUT_OF_DATE_KHR,
                AfterPresentError::RecreateSwapchain,
            ),
            (
                VkResult::ERROR_SURFACE_LOST_KHR,
                AfterPresentError::ReturnError,
            ),
            (
                VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
                AfterPresentError::ReturnError,
            ),
            (VkResult::ERROR_UNKNOWN, AfterPresentError::Broken),
            (VkResult::ERROR_VALIDATION_FAILED, AfterPresentError::Broken),
            (
                VkResult::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT,
                AfterPresentError::ReturnError,
            ),
        ]);
        let command = vk::command::QUEUE_PRESENT_KHR;
        assert_eq!(
            sorted(command.error_codes),
            expected.keys().copied().collect::<Vec<_>>(),
            "the error codes that the registry lists for {}",
            command.name
        );
        for code in command.error_codes {
            assert_eq!(
                after_present_error(*code),
                expected[&code.as_raw()],
                "{code:?}"
            );
        }
    }

    /// The same for `vkAcquireNextImageKHR` and `vkAcquireNextImage2KHR`.
    #[test]
    fn every_error_code_of_acquire_has_the_action_that_the_specification_asks_for() {
        let expected = listed(&[
            (
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                AfterAcquireError::ReturnError,
            ),
            (
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                AfterAcquireError::ReturnError,
            ),
            (VkResult::ERROR_DEVICE_LOST, AfterAcquireError::ReturnError),
            (
                VkResult::ERROR_OUT_OF_DATE_KHR,
                AfterAcquireError::RecreateSwapchain,
            ),
            (
                VkResult::ERROR_SURFACE_LOST_KHR,
                AfterAcquireError::ReturnError,
            ),
            (
                VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
                AfterAcquireError::ReturnError,
            ),
            (VkResult::ERROR_UNKNOWN, AfterAcquireError::ReturnError),
            (
                VkResult::ERROR_VALIDATION_FAILED,
                AfterAcquireError::ReturnError,
            ),
        ]);
        for command in [
            vk::command::ACQUIRE_NEXT_IMAGE_KHR,
            vk::command::ACQUIRE_NEXT_IMAGE2_KHR,
        ] {
            assert_eq!(
                sorted(command.error_codes),
                expected.keys().copied().collect::<Vec<_>>(),
                "the error codes that the registry lists for {}",
                command.name
            );
            for code in command.error_codes {
                assert_eq!(
                    after_acquire_error(*code),
                    expected[&code.as_raw()],
                    "{code:?}"
                );
            }
        }
    }

    /// A result that the registry does not list as an error code of the command says
    /// nothing about the semaphores, whatever its sign.
    #[test]
    fn a_result_that_is_not_an_error_code_of_the_command_breaks_the_frame() {
        let not_listed = [
            VkResult::SUCCESS,
            VkResult::NOT_READY,
            VkResult::TIMEOUT,
            VkResult::INCOMPLETE,
            VkResult::SUBOPTIMAL_KHR,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_FRAGMENTED_POOL,
            VkResult::from_raw(i32::MIN),
            VkResult::from_raw(i32::MAX),
        ];
        for code in not_listed {
            for command in [
                vk::command::QUEUE_PRESENT_KHR,
                vk::command::ACQUIRE_NEXT_IMAGE_KHR,
            ] {
                assert!(
                    !command.error_codes.contains(&code),
                    "{code:?} is an error code of {}",
                    command.name
                );
            }
            assert_eq!(
                after_present_error(code),
                AfterPresentError::Broken,
                "{code:?}"
            );
            assert_eq!(
                after_acquire_error(code),
                AfterAcquireError::Broken,
                "{code:?}"
            );
        }
    }
}
