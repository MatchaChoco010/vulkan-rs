use anyhow::Result;
use vulkan::vk;

use crate::device::Gpu;

/// Records one command buffer with `record`, submits it and waits until the device has
/// executed it.
///
/// The command pool lives only for this call. That is wasteful for work that is
/// submitted every frame, and simple for work that is submitted once.
pub unsafe fn submit_and_wait(
    gpu: &Gpu,
    record: impl FnOnce(vk::CommandBuffer) -> Result<()>,
) -> Result<()> {
    unsafe {
        let device = &gpu.device;
        let pool = device.create_command_pool(
            &vk::CommandPoolCreateInfo::default().queue_family_index(gpu.queue_family),
            None,
        )?;
        let command_buffer = device.allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1),
        )?[0];
        device.begin_command_buffer(
            command_buffer,
            &vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
        )?;
        record(command_buffer)?;
        device.end_command_buffer(command_buffer)?;

        let fence = device.create_fence(&vk::FenceCreateInfo::default(), None)?;
        let command_buffers = [command_buffer];
        let submit = vk::SubmitInfo::default().command_buffers(&command_buffers);
        device.queue_submit(gpu.queue, &[submit], fence)?;
        device.wait_for_fences(&[fence], true, u64::MAX)?;
        device.destroy_fence(fence, None);
        // Destroying the pool also frees the command buffer.
        device.destroy_command_pool(pool, None);
        Ok(())
    }
}
