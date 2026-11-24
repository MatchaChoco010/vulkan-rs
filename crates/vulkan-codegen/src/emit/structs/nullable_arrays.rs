use super::setters::count_member;
use crate::model::CType;
use vulkan_registry::{Field, Registry, Struct};

#[derive(Debug)]
pub(super) struct NullableArray {
    pub(super) structure: &'static str,
    pub(super) member: &'static str,
    pub(super) stated_by: &'static str,
}

pub(super) const NULLABLE_ARRAYS: &[NullableArray] = &[
    NullableArray {
        structure: "StdVideoAV1TileInfo",
        member: "pHeightInSbsMinus1",
        stated_by: "the description of StdVideoAV1TileInfo::pHeightInSbsMinus1 for AV1 encode operations as NULL or a pointer to an array of TileRows elements",
    },
    NullableArray {
        structure: "StdVideoAV1TileInfo",
        member: "pWidthInSbsMinus1",
        stated_by: "the description of StdVideoAV1TileInfo::pWidthInSbsMinus1 for AV1 encode operations as NULL or a pointer to an array of TileCols elements",
    },
    NullableArray {
        structure: "VkDataGraphPipelineCreateInfoARM",
        member: "pResourceInfos",
        stated_by: "VUID-VkDataGraphPipelineCreateInfoARM-resourceInfoCount-12364, by which pResourceInfos has to be NULL if resourceInfoCount is 0",
    },
    NullableArray {
        structure: "VkDebugUtilsMessengerCallbackDataEXT",
        member: "pCmdBufLabels",
        stated_by: "the description of VkDebugUtilsMessengerCallbackDataEXT::pCmdBufLabels as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkDebugUtilsMessengerCallbackDataEXT",
        member: "pQueueLabels",
        stated_by: "the description of VkDebugUtilsMessengerCallbackDataEXT::pQueueLabels as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkDeviceCreateInfo",
        member: "ppEnabledLayerNames",
        stated_by: "VUID-VkDeviceCreateInfo-ppEnabledLayerNames-12385, by which ppEnabledLayerNames has to be NULL",
    },
    NullableArray {
        structure: "VkImageCompressionControlEXT",
        member: "pFixedRateFlags",
        stated_by: "the description of VkImageCompressionControlEXT::pFixedRateFlags as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkMutableDescriptorTypeListEXT",
        member: "pDescriptorTypes",
        stated_by: "the description of VkMutableDescriptorTypeListEXT::pDescriptorTypes as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkPastPresentationTimingPropertiesEXT",
        member: "pPresentationTimings",
        stated_by: "the description of VkPastPresentationTimingPropertiesEXT, by which the command returns the number of available timing records in presentationTimingCount if pPresentationTimings is NULL",
    },
    NullableArray {
        structure: "VkPhysicalDeviceGpaPropertiesAMD",
        member: "pPerfBlocks",
        stated_by: "the description of VkPhysicalDeviceGpaPropertiesAMD, by which the number of performance blocks is returned in perfBlockCount if pPerfBlocks is NULL",
    },
    NullableArray {
        structure: "VkPresentInfoKHR",
        member: "pWaitSemaphores",
        stated_by: "the description of VkPresentInfoKHR::pWaitSemaphores as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkRenderingAttachmentLocationInfo",
        member: "pColorAttachmentLocations",
        stated_by: "the description of VkRenderingAttachmentLocationInfo, by which a NULL pColorAttachmentLocations is equivalent to setting each element to its index within the array",
    },
    NullableArray {
        structure: "VkVideoDecodeInfoKHR",
        member: "pReferenceSlots",
        stated_by: "the description of VkVideoDecodeInfoKHR::pReferenceSlots as NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkVideoEncodeInfoKHR",
        member: "pReferenceSlots",
        stated_by: "the description of VkVideoEncodeInfoKHR::pReferenceSlots as NULL or a pointer to an array",
    },
];

const CORRECT_THE_LIST: &str = "correct the list of arrays that the specification allows to be null although the registry does not mark their pointers as optional";

impl NullableArray {
    fn assert_matches(&self, s: &Struct) {
        let Self {
            structure,
            member,
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
        assert!(
            ty.depth() > 0 && !ty.is_array() && count_member(s, m).is_some(),
            "{structure}::{member} is {}, not a pointer to an array whose length a member stores: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            match (ty.is_array(), m.len.as_deref()) {
                (true, _) => format!("an array of a fixed size of `{}`", m.full_ty),
                (false, Some(len)) => format!("a `{}` with the length `{len}`", m.full_ty),
                (false, None) => format!("a `{}` without a length", m.full_ty),
            }
        );
        assert!(
            !m.optional,
            "the registry marks the pointer {structure}::{member} as optional: {CORRECT_THE_LIST}, which takes this one from {stated_by}"
        );
    }
}

pub(super) fn may_be_null(s: &Struct, m: &Field) -> bool {
    if m.optional {
        return true;
    }
    let Some(listed) = NULLABLE_ARRAYS
        .iter()
        .find(|listed| listed.structure == s.name && listed.member == m.name)
    else {
        return false;
    };
    listed.assert_matches(s);
    true
}

pub(crate) fn assert_listed_nullable_arrays_exist(ir: &Registry) {
    for listed in NULLABLE_ARRAYS {
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
    use super::super::definition;
    use super::*;
    use crate::emit::squash;
    use crate::model::Model;
    use crate::naming::field_name;
    use crate::test_support::{
        catch_quietly, chapters_of_the_specification, counted, description_in, field, optional,
        registry, structure,
    };

    fn statement_about_null<'t>(text: &'t str, markup: &str, member: &str) -> Option<&'t str> {
        [
            "is `NULL` or a pointer ",
            "is `NULL` or is a pointer ",
            "is `NULL`, ",
            "is not `NULL`",
            "must: be `NULL`",
            "must: equal `NULL`",
        ]
        .iter()
        .find_map(|says| {
            let statement = format!("{markup}:{member} {says}");
            text.match_indices(statement.as_str())
                .find(|(at, _)| !text[..*at].ends_with(" of "))
                .map(|(at, statement)| &text[at..at + statement.len()])
        })
    }

    fn joined(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    const WITHOUT_A_DESCRIPTION: &[&str] = &["VkCuModuleCreateInfoNVX"];

    #[test]
    fn the_scan_for_arrays_that_may_be_null_reads_the_ways_the_specification_says_it() {
        for (text, expected) in [
            (
                "  * pname:pValues is `NULL` or a pointer to an array of\n    values.",
                Some("pname:pValues is `NULL` or a pointer "),
            ),
            (
                "  * pname:pValues is `NULL` or is a pointer to an array",
                Some("pname:pValues is `NULL` or is a pointer "),
            ),
            (
                "If pname:pValues is `NULL`, then the number of values is returned.",
                Some("pname:pValues is `NULL`, "),
            ),
            (
                "the feature is not enabled, and if pname:pValues is `NULL`, it is the same",
                Some("pname:pValues is `NULL`, "),
            ),
            (
                "and pname:pValues is not\n`NULL`, each element must: be unique",
                Some("pname:pValues is not `NULL`"),
            ),
            (
                "If pname:valueCount is 0, pname:pValues\nmust: equal `NULL`",
                Some("pname:pValues must: equal `NULL`"),
            ),
            (
                "pname:pValues must: be `NULL`",
                Some("pname:pValues must: be `NULL`"),
            ),
            (
                "  * pname:pValues is a pointer to an array of values.",
                None,
            ),
            ("pname:pValues must: not be `NULL`", None),
            ("pname:pOthers is `NULL` or a pointer to an array", None),
            ("If pname:pOthers is `NULL`, pname:pValues is ignored", None),
            (
                "the pname:pNext member of an element of pname:pValues is not `NULL`",
                None,
            ),
            ("pname:pValuesOfOthers is not `NULL`", None),
            ("code:pValues is `NULL` or a pointer to an array", None),
        ] {
            assert_eq!(
                statement_about_null(&joined(text), "pname", "pValues"),
                expected,
                "{text}"
            );
        }
        assert_eq!(
            statement_about_null("code:pValues is `NULL` or a pointer to", "code", "pValues"),
            Some("code:pValues is `NULL` or a pointer ")
        );
        let chapters = "[open,refpage='VkFirst',desc='First',type='structs']\n--\nabout the first\n--\n\n[open,refpage='VkSecond',desc='Second',type='structs']\n--\nabout the second\n";
        assert_eq!(description_in(chapters, "VkFirst"), "about the first");
        assert_eq!(description_in(chapters, "VkSecond"), "about the second\n");
        assert_eq!(description_in(chapters, "VkThird"), "");
    }

    #[test]
    fn every_array_that_the_specification_allows_to_be_null_is_optional_or_listed() {
        assert_listed_nullable_arrays_exist(Registry::get());
        let Some(chapters) = chapters_of_the_specification() else {
            return;
        };
        let all_chapters = joined(&chapters);
        let model = Model::new(Registry::get());
        let mut found = Vec::new();
        let mut without_a_description = Vec::new();
        for s in Registry::get().structs.iter().filter(|s| !s.union) {
            let described_with_the_codec = s.video_std_header.is_some();
            let description = joined(description_in(&chapters, &s.name));
            let (text, markup) = if described_with_the_codec {
                assert!(
                    description.is_empty(),
                    "{} of a Vulkan Video Std header has a description of its own now, which this test does not read",
                    s.name
                );
                (&all_chapters, "code")
            } else {
                (&description, "pname")
            };
            let mut has_such_an_array = false;
            let generated = squash(&definition(&model, s));
            for m in &s.members {
                let ty = CType::of(m);
                let takes_a_slice =
                    generated.contains(&format!("self.{} = value.as_", field_name(&m.name)));
                if ty.depth() == 0
                    || ty.is_array()
                    || m.optional
                    || count_member(s, m).is_none()
                    || !takes_a_slice
                {
                    continue;
                }
                has_such_an_array = true;
                let listed = NULLABLE_ARRAYS
                    .iter()
                    .any(|listed| listed.structure == s.name && listed.member == m.name);
                let statement = statement_about_null(text, markup, &m.name);
                assert_eq!(
                    statement.is_some(),
                    listed,
                    "{}::{}: the registry does not mark the pointer as optional, the specification says {statement:?} about it, and the list of arrays that the specification allows to be null {} it",
                    s.name,
                    m.name,
                    if listed { "names" } else { "does not name" }
                );
                if listed {
                    found.push((s.name.as_str(), m.name.as_str()));
                }
            }
            if has_such_an_array && text.is_empty() {
                without_a_description.push(s.name.as_str());
            }
        }
        assert_eq!(
            without_a_description, WITHOUT_A_DESCRIPTION,
            "the structures with such an array that the specification does not describe"
        );
        found.sort_unstable();
        assert_eq!(
            found,
            NULLABLE_ARRAYS
                .iter()
                .map(|listed| (listed.structure, listed.member))
                .collect::<Vec<_>>(),
            "the list is sorted and names only arrays of structures"
        );
    }

    fn listed() -> Struct {
        structure(
            "VkPresentInfoKHR",
            vec![
                field("waitSemaphoreCount", "uint32_t"),
                counted(
                    "pWaitSemaphores",
                    "const VkSemaphore*",
                    "waitSemaphoreCount",
                ),
            ],
        )
    }

    #[test]
    fn a_listed_array_that_the_registry_changes_stops_the_generator() {
        const CORRECT: &str = "correct the list of arrays that the specification allows to be null although the registry does not mark their pointers as optional, which takes this one from the description of VkPresentInfoKHR::pWaitSemaphores as NULL or a pointer to an array";
        let mut renamed = listed();
        renamed.members[1].name = "pSemaphores".to_string();
        let mut uncounted = listed();
        uncounted.members[1].len = None;
        let mut counted_elsewhere = listed();
        counted_elsewhere.members[1].len = Some("swapchainCount".to_string());
        let mut value = listed();
        value.members[1] = field("pWaitSemaphores", "VkSemaphore");
        let mut fixed = listed();
        fixed.members[1].fixed_size_array = vec!["4".to_string()];
        let mut marked = listed();
        marked.members[1] = optional(marked.members[1].clone());
        for (s, message) in [
            (
                renamed,
                format!("VkPresentInfoKHR has no member `pWaitSemaphores`: {CORRECT}"),
            ),
            (
                uncounted,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `const VkSemaphore*` without a length, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                counted_elsewhere,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `const VkSemaphore*` with the length `swapchainCount`, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                value,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `VkSemaphore` without a length, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                fixed,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is an array of a fixed size of `const VkSemaphore*`, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
        ] {
            let mut ir = registry();
            ir.structs = Registry::get().structs.clone();
            ir.structs.retain(|other| other.name != s.name);
            ir.structs.push(s);
            assert_eq!(
                catch_quietly(|| assert_listed_nullable_arrays_exist(&ir)),
                Err(message)
            );
        }
        let m = marked.members[1].clone();
        let mut ir = registry();
        ir.structs = Registry::get().structs.clone();
        ir.structs.retain(|other| other.name != marked.name);
        ir.structs.push(marked.clone());
        assert_eq!(
            catch_quietly(|| assert_listed_nullable_arrays_exist(&ir)),
            Err(format!(
                "the registry marks the pointer VkPresentInfoKHR::pWaitSemaphores as optional: {CORRECT}"
            ))
        );
        assert_eq!(catch_quietly(|| may_be_null(&marked, &m)), Ok(true));
        let s = listed();
        assert_eq!(catch_quietly(|| may_be_null(&s, &s.members[1])), Ok(true));
        assert_eq!(catch_quietly(|| may_be_null(&s, &s.members[0])), Ok(false));
        let message = catch_quietly(|| assert_listed_nullable_arrays_exist(&registry()))
            .expect_err("the registry has none of the listed structures");
        assert!(
            message.starts_with(
                "the registry has no structure StdVideoAV1TileInfo: correct the list of arrays that the specification allows to be null although the registry does not mark their pointers as optional, which takes `pHeightInSbsMinus1` from "
            ),
            "{message}"
        );
    }
}
