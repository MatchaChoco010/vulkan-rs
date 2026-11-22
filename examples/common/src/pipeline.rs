use anyhow::{Result, anyhow};
use vulkan::{Device, vk};

/// The pipeline from a `vkCreate*Pipelines` call that was given one create info.
///
/// A failed call still returns the pipelines that it could create, next to null handles
/// for those it could not; the created ones are destroyed here.
pub unsafe fn single_pipeline(
    device: &Device,
    result: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::VkResult)>,
) -> Result<vk::Pipeline> {
    match result {
        Ok(pipelines) => pipelines
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("pipeline creation returned no pipeline")),
        Err((pipelines, error)) => {
            for pipeline in pipelines.into_iter().filter(|p| !p.is_null()) {
                unsafe { device.destroy_pipeline(pipeline, None) };
            }
            Err(anyhow!("pipeline creation failed: {error}"))
        }
    }
}
