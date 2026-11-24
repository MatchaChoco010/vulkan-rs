use crate::model::{CType, is_opaque};
use vulkan_registry::{Field, Registry, Struct};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum PointsTo {
    Array,
    OneValue,
}

#[derive(Debug)]
pub(super) struct ReadOnlyMember {
    pub(super) structure: &'static str,
    pub(super) member: &'static str,
    pub(super) points_to: PointsTo,
    pub(super) stated_by: &'static str,
}

pub(super) const READ_ONLY_MEMBERS: &[ReadOnlyMember] = &[
    ReadOnlyMember {
        structure: "VkClusterAccelerationStructureOpInputNV",
        member: "pClustersBottomLevel",
        points_to: PointsTo::OneValue,
        stated_by: "the description of VkClusterAccelerationStructureOpInputNV::pClustersBottomLevel, an upper threshold on the parameters of the operation, which the commands reach through a const VkClusterAccelerationStructureInputInfoNV",
    },
    ReadOnlyMember {
        structure: "VkClusterAccelerationStructureOpInputNV",
        member: "pMoveObjects",
        points_to: PointsTo::OneValue,
        stated_by: "the description of VkClusterAccelerationStructureOpInputNV::pMoveObjects, an upper threshold on the parameters of the operation, which the commands reach through a const VkClusterAccelerationStructureInputInfoNV",
    },
    ReadOnlyMember {
        structure: "VkClusterAccelerationStructureOpInputNV",
        member: "pTriangleClusters",
        points_to: PointsTo::OneValue,
        stated_by: "the description of VkClusterAccelerationStructureOpInputNV::pTriangleClusters, an upper threshold on the parameters of the operation, which the commands reach through a const VkClusterAccelerationStructureInputInfoNV",
    },
    ReadOnlyMember {
        structure: "VkDataGraphProcessingEngineCreateInfoARM",
        member: "pProcessingEngines",
        points_to: PointsTo::Array,
        stated_by: "the description of VkDataGraphProcessingEngineCreateInfoARM, whose array names the processing engines that the created object uses",
    },
    ReadOnlyMember {
        structure: "VkImageCompressionControlEXT",
        member: "pFixedRateFlags",
        points_to: PointsTo::Array,
        stated_by: "the description of VkImageCompressionControlEXT::pFixedRateFlags, the allowed fixed-rate compression rates of each image plane",
    },
    ReadOnlyMember {
        structure: "VkPipelineBinaryDataKHR",
        member: "pData",
        points_to: PointsTo::Array,
        stated_by: "the description of VkPipelineBinaryDataKHR::pData, pipeline binary data obtained from vkGetPipelineBinaryDataKHR that VkPipelineBinaryKeysAndDataKHR passes to vkCreatePipelineBinariesKHR",
    },
    ReadOnlyMember {
        structure: "VkRenderPassPerformanceCountersByRegionBeginInfoARM",
        member: "pCounterIndices",
        points_to: PointsTo::Array,
        stated_by: "the description of VkRenderPassPerformanceCountersByRegionBeginInfoARM::pCounterIndices, the counters to enable in the render pass instance",
    },
];

const CORRECT_THE_LIST: &str =
    "correct the list of members that the C API declares writable and only reads";

impl PointsTo {
    pub(super) fn what(self) -> &'static str {
        match self {
            PointsTo::Array => "an array",
            PointsTo::OneValue => "one value",
        }
    }
}

impl ReadOnlyMember {
    fn assert_matches(&self, s: &Struct) {
        let Self {
            structure,
            member,
            points_to,
            stated_by,
        } = self;
        let m = s
            .members
            .iter()
            .find(|m| m.name == *member)
            .unwrap_or_else(|| {
                panic!(
                    "{structure} has no member `{member}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}"
                )
            });
        let ty = CType::of(m);
        let (matches, expected) = match points_to {
            PointsTo::Array => (m.len.is_some(), "an array of a stated length"),
            PointsTo::OneValue => (
                m.len.is_none() && !is_opaque(&ty.base),
                "one value of a type that a reference can borrow",
            ),
        };
        assert!(
            ty.pointers == [false] && !ty.is_array() && matches,
            "{structure}::{member} is a `{}` with the length {:?}, not a writable pointer to {expected}: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            m.full_ty,
            m.len
        );
    }

    pub(super) fn note(&self, borrower: &str) -> String {
        let (reads, them) = match self.points_to {
            PointsTo::Array => ("the elements", "them"),
            PointsTo::OneValue => ("the value", "it"),
        };
        format!(
            "The C declaration of the member is a pointer to writable memory, but the implementation only reads {reads}, so the {borrower} borrows {them} shared."
        )
    }
}

pub(super) fn read_only(s: &Struct, m: &Field) -> Option<&'static ReadOnlyMember> {
    let listed = READ_ONLY_MEMBERS
        .iter()
        .find(|listed| listed.structure == s.name && listed.member == m.name)?;
    listed.assert_matches(s);
    Some(listed)
}

pub(crate) fn assert_listed_read_only_members_exist(ir: &Registry) {
    for listed in READ_ONLY_MEMBERS {
        let s = ir
            .structs
            .iter()
            .find(|s| s.name == listed.structure)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no structure {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    listed.structure, listed.member, listed.stated_by
                )
            });
        listed.assert_matches(s);
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
        ("VkBindMemoryStatus", "pResult"),
        ("VkConvertCooperativeVectorMatrixInfoNV", "pDstSize"),
        ("VkDataGraphPipelinePropertyQueryResultARM", "pData"),
        ("VkDeviceFaultDebugInfoKHR", "pVendorBinaryData"),
        ("VkDeviceFaultInfoEXT", "pAddressInfos"),
        ("VkDeviceFaultInfoEXT", "pVendorInfos"),
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
            "pPipelineCreationFeedback",
        ),
        (
            "VkPipelineCreationFeedbackCreateInfo",
            "pPipelineStageCreationFeedbacks",
        ),
        ("VkPipelineExecutableInternalRepresentationKHR", "pData"),
        ("VkPresentInfoKHR", "pResults"),
        (
            "VkRenderPassCreationFeedbackCreateInfoEXT",
            "pRenderPassFeedback",
        ),
        (
            "VkRenderPassSubpassFeedbackCreateInfoEXT",
            "pSubpassFeedback",
        ),
        ("VkSurfacePresentModeCompatibilityKHR", "pPresentModes"),
        ("VkSwapchainTimeDomainPropertiesEXT", "pTimeDomainIds"),
        ("VkSwapchainTimeDomainPropertiesEXT", "pTimeDomains"),
    ];

    fn borrowed_through_a_writable_pointer(m: &Field) -> bool {
        let ty = CType::of(m);
        m.name != "pNext"
            && ty.pointers == [false]
            && !ty.is_array()
            && (m.len.is_some() || !is_opaque(&ty.base))
    }

    #[test]
    fn every_writable_pointer_of_the_registry_is_known_as_written_or_read() {
        assert_listed_read_only_members_exist(Registry::get());
        let mut unknown = Vec::new();
        let mut written_found = 0;
        let mut read_found = 0;
        for s in &Registry::get().structs {
            for m in &s.members {
                let ty = CType::of(m);
                assert!(
                    ty.depth() < 2 || !ty.pointers.contains(&false),
                    "{}::{} is a `{}`, a pointer to pointers with a writable level, which this test does not classify",
                    s.name,
                    m.name,
                    m.full_ty
                );
            }
            for m in s
                .members
                .iter()
                .filter(|m| borrowed_through_a_writable_pointer(m))
            {
                let read = read_only(s, m).is_some();
                let written = WRITTEN_BY_THE_IMPLEMENTATION.contains(&(&s.name, &m.name));
                assert!(!(read && written), "{}::{} is listed twice", s.name, m.name);
                written_found += usize::from(written);
                read_found += usize::from(read);
                if !read && !written {
                    unknown.push(format!("{}::{}", s.name, m.name));
                }
            }
        }
        assert_eq!(
            written_found,
            WRITTEN_BY_THE_IMPLEMENTATION.len(),
            "a member listed as written by the implementation is not a writable pointer of the registry"
        );
        assert_eq!(read_found, READ_ONLY_MEMBERS.len());
        assert!(
            unknown.is_empty(),
            "these members are pointers to writable memory in C: find in the specification whether the implementation writes through them or only reads: {unknown:?}"
        );
    }

    #[test]
    fn the_scan_looks_only_at_writable_pointers_that_a_setter_borrows() {
        let member = |name: &str, full_ty: &str, ty: &str, len: Option<&str>| Field {
            name: name.to_string(),
            ty: ty.to_string(),
            full_ty: full_ty.to_string(),
            len: len.map(str::to_string),
            ..Default::default()
        };
        for (m, borrowed) in [
            (
                member("pValues", "uint32_t*", "uint32_t", Some("count")),
                true,
            ),
            (member("pValue", "uint32_t*", "uint32_t", None), true),
            (member("pData", "void*", "void", Some("dataSize")), true),
            (member("pData", "void*", "void", None), false),
            (member("display", "wl_display*", "wl_display", None), false),
            (member("pNext", "void*", "void", None), false),
            (member("pNext", "VkThing*", "VkThing", None), false),
            (
                member("pValues", "const uint32_t*", "uint32_t", Some("count")),
                false,
            ),
            (member("pValue", "const uint32_t*", "uint32_t", None), false),
            (
                member("ppValues", "uint32_t**", "uint32_t", Some("count")),
                false,
            ),
            (member("value", "uint32_t", "uint32_t", None), false),
        ] {
            assert_eq!(
                borrowed_through_a_writable_pointer(&m),
                borrowed,
                "{} {}",
                m.full_ty,
                m.name
            );
        }
        let mut values = member("values", "uint32_t*", "uint32_t", None);
        values.fixed_size_array = vec!["4".to_string()];
        assert!(!borrowed_through_a_writable_pointer(&values));
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
                "self.p_fixed_rate_flags = value.as_ptr().cast_mut(); } self }",
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
    fn the_constructor_of_a_read_only_union_member_borrows_the_value_shared() {
        let ir = Registry::get();
        let model = Model::with_docs(ir, Docs::get());
        let generated = super::super::emit(&model);
        let out = squash(&generated);
        for (constructor, ty, field) in [
            (
                "clusters_bottom_level",
                "ClusterAccelerationStructureClustersBottomLevelInputNV",
                "p_clusters_bottom_level",
            ),
            (
                "triangle_clusters",
                "ClusterAccelerationStructureTriangleClusterInputNV",
                "p_triangle_clusters",
            ),
            (
                "move_objects",
                "ClusterAccelerationStructureMoveObjectsInputNV",
                "p_move_objects",
            ),
        ] {
            let expected = format!(
                "pub fn {constructor}(value: &'a {ty}<'a>) -> Self {{ let mut union = Self::default(); union.{field} = core::ptr::from_ref(value).cast_mut(); union }}"
            );
            assert!(out.contains(&expected), "no `{expected}`");
            let documented = crate::emit::doc_of(&generated, &format!("pub fn {constructor}("));
            assert!(
                documented.join(" ").contains(
                    "The C declaration of the member is a pointer to writable memory, but the implementation only reads the value, so the constructor borrows it shared."
                ),
                "{documented:?}"
            );
        }
    }

    #[test]
    fn a_read_only_pointer_to_one_value_of_a_structure_is_borrowed_shared() {
        let model = Model::new(Registry::get());
        let mut s = Registry::get()
            .structs
            .iter()
            .find(|s| s.name == "VkClusterAccelerationStructureOpInputNV")
            .expect("the listed union")
            .clone();
        s.union = false;
        let out = squash(&super::super::definition(&model, &s));
        assert!(
            out.contains(
                "pub fn move_objects(mut self, value: &'a ClusterAccelerationStructureMoveObjectsInputNV<'a>) -> Self { self.p_move_objects = core::ptr::from_ref(value).cast_mut(); self }"
            ),
            "{out}"
        );
    }

    #[test]
    fn a_registry_without_a_listed_structure_stops_the_generator() {
        let message = catch_quietly(|| assert_listed_read_only_members_exist(&registry()))
            .expect_err("the registry has none of the listed structures");
        assert!(
            message.starts_with(
                "the registry has no structure VkClusterAccelerationStructureOpInputNV: correct the list of members that the C API declares writable and only reads, which takes `pClustersBottomLevel` from "
            ),
            "{message}"
        );
    }

    #[test]
    fn a_listed_member_that_the_registry_changes_stops_the_generator() {
        for listed in READ_ONLY_MEMBERS {
            let original = Registry::get()
                .structs
                .iter()
                .find(|s| s.name == listed.structure)
                .expect("the listed structure")
                .clone();
            let changes: [fn(&mut Field); 3] = [
                |m| m.full_ty = format!("const {}", m.full_ty),
                |m| {
                    m.len = match m.len {
                        Some(_) => None,
                        None => Some("count".to_string()),
                    }
                },
                |m| m.fixed_size_array = vec!["4".to_string()],
            ];
            for change in changes {
                let mut changed = original.clone();
                for m in changed
                    .members
                    .iter_mut()
                    .filter(|m| m.name == listed.member)
                {
                    change(m);
                }
                let m = changed
                    .members
                    .iter()
                    .find(|m| m.name == listed.member)
                    .unwrap()
                    .clone();
                let message = catch_quietly(|| read_only(&changed, &m).is_some())
                    .expect_err("the member does not match the list");
                assert!(
                    message.starts_with(&format!(
                        "{}::{} is a `",
                        listed.structure, listed.member
                    )) && message.contains(match listed.points_to {
                        PointsTo::Array => ", not a writable pointer to an array of a stated length: ",
                        PointsTo::OneValue =>
                            ", not a writable pointer to one value of a type that a reference can borrow: ",
                    }),
                    "{message}"
                );
            }
            let mut ir = registry();
            ir.structs = Registry::get().structs.clone();
            for s in ir.structs.iter_mut().filter(|s| s.name == listed.structure) {
                for m in s.members.iter_mut().filter(|m| m.name == listed.member) {
                    m.name = "pRenamed".to_string();
                }
            }
            let message = catch_quietly(|| assert_listed_read_only_members_exist(&ir))
                .expect_err("the listed member is missing");
            assert!(
                message.starts_with(&format!(
                    "{} has no member `{}`",
                    listed.structure, listed.member
                )),
                "{message}"
            );
        }
    }

    #[test]
    fn a_listed_pointer_to_one_opaque_value_stops_the_generator() {
        let mut s = Registry::get()
            .structs
            .iter()
            .find(|s| s.name == "VkClusterAccelerationStructureOpInputNV")
            .expect("the listed union")
            .clone();
        let m = s
            .members
            .iter_mut()
            .find(|m| m.name == "pMoveObjects")
            .expect("the listed member");
        m.ty = "void".to_string();
        m.full_ty = "void*".to_string();
        let m = m.clone();
        let message = catch_quietly(|| read_only(&s, &m).is_some())
            .expect_err("a reference cannot borrow a void");
        assert!(
            message.starts_with(
                "VkClusterAccelerationStructureOpInputNV::pMoveObjects is a `void*` with the length None, not a writable pointer to one value of a type that a reference can borrow: "
            ),
            "{message}"
        );
    }

    #[test]
    fn a_listed_member_that_its_setter_or_constructor_borrows_otherwise_stops_the_generator() {
        let model = Model::new(Registry::get());
        let binary_data = Registry::get()
            .structs
            .iter()
            .find(|s| s.name == "VkPipelineBinaryDataKHR")
            .expect("the listed structure");
        let mut string = binary_data.clone();
        let m = string
            .members
            .iter_mut()
            .find(|m| m.name == "pData")
            .expect("the listed member");
        m.ty = "char".to_string();
        m.full_ty = "char*".to_string();
        m.len = Some("null-terminated".to_string());
        m.null_terminated = true;
        assert_eq!(
            catch_quietly(|| super::super::definition(&model, &string)),
            Err("VkPipelineBinaryDataKHR::pData: the list of members that the implementation only reads says that the member points to an array, and the setter borrows neither".to_string())
        );
        let mut union = binary_data.clone();
        union.union = true;
        union.members.retain(|m| m.name == "pData");
        assert_eq!(
            catch_quietly(|| super::super::unions::definition(&model, &union)),
            Err("VkPipelineBinaryDataKHR::pData: the list of members that the implementation only reads names the member, and the constructor of a union borrows only one value that is not a string".to_string())
        );
    }
}
