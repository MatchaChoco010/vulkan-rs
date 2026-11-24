use crate::model::CType;
use vulkan_registry::{Field, Registry, Struct};

#[derive(Debug)]
pub(super) struct ReadOnlyArray {
    pub(super) structure: &'static str,
    pub(super) member: &'static str,
    pub(super) stated_by: &'static str,
}

pub(super) const READ_ONLY_ARRAYS: &[ReadOnlyArray] = &[
    ReadOnlyArray {
        structure: "VkDataGraphProcessingEngineCreateInfoARM",
        member: "pProcessingEngines",
        stated_by: "the description of VkDataGraphProcessingEngineCreateInfoARM, whose array names the processing engines that the created object uses",
    },
    ReadOnlyArray {
        structure: "VkImageCompressionControlEXT",
        member: "pFixedRateFlags",
        stated_by: "the description of VkImageCompressionControlEXT::pFixedRateFlags, the allowed fixed-rate compression rates of each image plane",
    },
    ReadOnlyArray {
        structure: "VkPipelineBinaryDataKHR",
        member: "pData",
        stated_by: "the description of VkPipelineBinaryDataKHR::pData, pipeline binary data obtained from vkGetPipelineBinaryDataKHR that VkPipelineBinaryKeysAndDataKHR passes to vkCreatePipelineBinariesKHR",
    },
    ReadOnlyArray {
        structure: "VkRenderPassPerformanceCountersByRegionBeginInfoARM",
        member: "pCounterIndices",
        stated_by: "the description of VkRenderPassPerformanceCountersByRegionBeginInfoARM::pCounterIndices, the counters to enable in the render pass instance",
    },
];

impl ReadOnlyArray {
    fn assert_matches(&self, s: &Struct) {
        let Self {
            structure,
            member,
            stated_by,
        } = self;
        let remove = format!(
            "correct the list of arrays that the C API declares writable and only reads, which takes this one from {stated_by}"
        );
        let m = s
            .members
            .iter()
            .find(|m| m.name == *member)
            .unwrap_or_else(|| panic!("{structure} has no member `{member}`: {remove}"));
        let ty = CType::of(m);
        assert!(
            ty.pointers == [false] && !ty.is_array() && m.len.is_some(),
            "{structure}::{member} is a `{}`, not a writable pointer to an array of a stated length: {remove}",
            m.full_ty
        );
    }
}

pub(super) fn is_read_only(s: &Struct, m: &Field) -> bool {
    let Some(array) = READ_ONLY_ARRAYS
        .iter()
        .find(|array| array.structure == s.name && array.member == m.name)
    else {
        return false;
    };
    array.assert_matches(s);
    true
}

pub(crate) fn assert_listed_read_only_arrays_exist(ir: &Registry) {
    for array in READ_ONLY_ARRAYS {
        let s = ir
            .structs
            .iter()
            .find(|s| s.name == array.structure)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no structure {}: correct the list of arrays that the C API declares writable and only reads, which takes `{}` from {}",
                    array.structure, array.member, array.stated_by
                )
            });
        array.assert_matches(s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::squash;
    use crate::model::Model;
    use crate::test_support::{catch_quietly, registry};
    use vulkan_registry::Docs;

    const WRITTEN_BY_THE_IMPLEMENTATION: &[(&str, &str)] = &[
        ("VkDataGraphPipelinePropertyQueryResultARM", "pData"),
        ("VkDeviceFaultDebugInfoKHR", "pVendorBinaryData"),
        ("VkDeviceFaultShaderAbortMessageInfoKHR", "pMessageData"),
        (
            "VkDrmFormatModifierPropertiesList2EXT",
            "pDrmFormatModifierProperties",
        ),
        (
            "VkDrmFormatModifierPropertiesListEXT",
            "pDrmFormatModifierProperties",
        ),
        ("VkGetLatencyMarkerInfoNV", "pTimings"),
        ("VkHostAddressRangeEXT", "address"),
        ("VkLatencySurfaceCapabilitiesNV", "pPresentModes"),
        ("VkPastPresentationTimingEXT", "pPresentStages"),
        (
            "VkPastPresentationTimingPropertiesEXT",
            "pPresentationTimings",
        ),
        ("VkPhysicalDeviceGpaPropertiesAMD", "pPerfBlocks"),
        ("VkPhysicalDeviceHostImageCopyProperties", "pCopyDstLayouts"),
        ("VkPhysicalDeviceHostImageCopyProperties", "pCopySrcLayouts"),
        (
            "VkPhysicalDeviceLayeredApiPropertiesListKHR",
            "pLayeredApis",
        ),
        ("VkPhysicalDeviceVulkan14Properties", "pCopyDstLayouts"),
        ("VkPhysicalDeviceVulkan14Properties", "pCopySrcLayouts"),
        ("VkPipelineBinaryHandlesInfoKHR", "pPipelineBinaries"),
        (
            "VkPipelineCreationFeedbackCreateInfo",
            "pPipelineStageCreationFeedbacks",
        ),
        ("VkPipelineExecutableInternalRepresentationKHR", "pData"),
        ("VkPresentInfoKHR", "pResults"),
        ("VkSurfacePresentModeCompatibilityKHR", "pPresentModes"),
        ("VkSwapchainTimeDomainPropertiesEXT", "pTimeDomainIds"),
        ("VkSwapchainTimeDomainPropertiesEXT", "pTimeDomains"),
    ];

    #[test]
    fn every_writable_array_of_the_registry_is_known_as_written_or_read() {
        assert_listed_read_only_arrays_exist(Registry::get());
        let mut unknown = Vec::new();
        let mut written_found = 0;
        for s in &Registry::get().structs {
            for m in &s.members {
                let ty = CType::of(m);
                if m.name == "pNext" || m.len.is_none() || ty.pointers != [false] {
                    continue;
                }
                let read = is_read_only(s, m);
                let written = WRITTEN_BY_THE_IMPLEMENTATION.contains(&(&s.name, &m.name));
                assert!(!(read && written), "{}::{} is listed twice", s.name, m.name);
                written_found += usize::from(written);
                if !read && !written {
                    unknown.push(format!("{}::{}", s.name, m.name));
                }
            }
        }
        assert_eq!(
            written_found,
            WRITTEN_BY_THE_IMPLEMENTATION.len(),
            "a member listed as written by the implementation is not a writable array of the registry"
        );
        assert!(
            unknown.is_empty(),
            "these arrays are writable in C: find in the specification whether the implementation writes them or only reads them: {unknown:?}"
        );
    }

    #[test]
    fn the_setter_of_a_read_only_array_borrows_the_elements_shared() {
        let ir = Registry::get();
        let model = Model::with_docs(ir, Docs::get());
        let out = squash(&super::super::emit(&model));
        for (structure, setter, expected) in [
            (
                "PipelineBinaryDataKHR",
                "pub fn data(mut self, value: &'a [u8]) -> Self {",
                "self.p_data = value.as_ptr().cast_mut().cast(); self }",
            ),
            (
                "ImageCompressionControlEXT",
                "pub fn fixed_rate_flags(mut self, value: &'a [ImageCompressionFixedRateFlagsEXT]) -> Self {",
                "self.p_fixed_rate_flags = value.as_ptr().cast_mut(); self }",
            ),
            (
                "DataGraphProcessingEngineCreateInfoARM",
                "pub fn processing_engines(mut self, value: &'a [PhysicalDeviceDataGraphProcessingEngineARM]) -> Self {",
                "self.p_processing_engines = value.as_ptr().cast_mut(); self }",
            ),
            (
                "RenderPassPerformanceCountersByRegionBeginInfoARM",
                "pub fn counter_indices(mut self, value: &'a [u32]) -> Self {",
                "self.p_counter_indices = value.as_ptr().cast_mut(); self }",
            ),
        ] {
            let implementation = format!("impl<'a> {structure}<'a> {{");
            let at = out
                .find(&implementation)
                .unwrap_or_else(|| panic!("no `{implementation}`"));
            let start = at
                + out[at..]
                    .find(setter)
                    .unwrap_or_else(|| panic!("{structure}: no setter `{setter}`"));
            let end = start + out[start..].find(" self }").expect("the end of the setter") + 7;
            assert!(out[start..end].ends_with(expected), "{}", &out[start..end]);
        }
    }

    #[test]
    fn a_registry_without_a_listed_structure_stops_the_generator() {
        let message = catch_quietly(|| assert_listed_read_only_arrays_exist(&registry()))
            .expect_err("the registry has none of the listed structures");
        assert!(
            message.starts_with(
                "the registry has no structure VkDataGraphProcessingEngineCreateInfoARM: correct the list of arrays that the C API declares writable and only reads, which takes `pProcessingEngines` from "
            ),
            "{message}"
        );
    }

    #[test]
    fn a_listed_array_that_the_registry_changes_stops_the_generator() {
        for array in READ_ONLY_ARRAYS {
            let original = Registry::get()
                .structs
                .iter()
                .find(|s| s.name == array.structure)
                .expect("the listed structure")
                .clone();
            for change in [
                |m: &mut Field| m.full_ty = format!("const {}", m.full_ty),
                |m: &mut Field| m.len = None,
            ] {
                let mut changed = original.clone();
                for m in changed
                    .members
                    .iter_mut()
                    .filter(|m| m.name == array.member)
                {
                    change(m);
                }
                let m = changed
                    .members
                    .iter()
                    .find(|m| m.name == array.member)
                    .unwrap()
                    .clone();
                let message = catch_quietly(|| is_read_only(&changed, &m))
                    .expect_err("the member does not match the list");
                assert!(
                    message.starts_with(&format!("{}::{} is a `", array.structure, array.member)),
                    "{message}"
                );
            }
            let mut ir = registry();
            ir.structs = Registry::get().structs.clone();
            for s in ir.structs.iter_mut().filter(|s| s.name == array.structure) {
                for m in s.members.iter_mut().filter(|m| m.name == array.member) {
                    m.name = "pRenamed".to_string();
                }
            }
            let message = catch_quietly(|| assert_listed_read_only_arrays_exist(&ir))
                .expect_err("the listed member is missing");
            assert!(
                message.starts_with(&format!(
                    "{} has no member `{}`",
                    array.structure, array.member
                )),
                "{message}"
            );
        }
    }
}
