use anyhow::{Result, anyhow};
use vulkan::vk;

use crate::device::{Gpu, Owned};

/// The pipeline from a `vkCreate*Pipelines` call that was given one create info.
///
/// A failed call still returns the pipelines that it could create, next to null handles
/// for those it could not; the created ones are destroyed here.
pub unsafe fn single_pipeline(
    gpu: &Gpu,
    result: Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::VkResult)>,
) -> Result<Owned<vk::Pipeline>> {
    let own = |pipelines: Vec<vk::Pipeline>| {
        pipelines
            .into_iter()
            .filter(|pipeline| !pipeline.is_null())
            .map(|pipeline| unsafe { gpu.own(pipeline) })
            .collect::<Vec<_>>()
    };
    match result {
        Ok(pipelines) => own(pipelines)
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("pipeline creation returned no pipeline")),
        Err((pipelines, error)) => {
            drop(own(pipelines));
            Err(anyhow!("pipeline creation failed: {error}"))
        }
    }
}
