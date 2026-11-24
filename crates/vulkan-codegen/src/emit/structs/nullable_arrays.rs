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
        structure: "VkDebugUtilsMessengerCallbackDataEXT",
        member: "pCmdBufLabels",
        stated_by: "the description of VkDebugUtilsMessengerCallbackDataEXT::pCmdBufLabels, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkDebugUtilsMessengerCallbackDataEXT",
        member: "pQueueLabels",
        stated_by: "the description of VkDebugUtilsMessengerCallbackDataEXT::pQueueLabels, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkImageCompressionControlEXT",
        member: "pFixedRateFlags",
        stated_by: "the description of VkImageCompressionControlEXT::pFixedRateFlags, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkMutableDescriptorTypeListEXT",
        member: "pDescriptorTypes",
        stated_by: "the description of VkMutableDescriptorTypeListEXT::pDescriptorTypes, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkPastPresentationTimingPropertiesEXT",
        member: "pPresentationTimings",
        stated_by: "the description of VkPastPresentationTimingPropertiesEXT, which returns the number of available timing records in presentationTimingCount if pPresentationTimings is NULL",
    },
    NullableArray {
        structure: "VkPhysicalDeviceGpaPropertiesAMD",
        member: "pPerfBlocks",
        stated_by: "the description of VkPhysicalDeviceGpaPropertiesAMD, which returns the number of performance blocks in perfBlockCount if pPerfBlocks is NULL",
    },
    NullableArray {
        structure: "VkPresentInfoKHR",
        member: "pWaitSemaphores",
        stated_by: "the description of VkPresentInfoKHR::pWaitSemaphores, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkRenderingAttachmentLocationInfo",
        member: "pColorAttachmentLocations",
        stated_by: "the description of VkRenderingAttachmentLocationInfo, for which a NULL pColorAttachmentLocations is equivalent to setting each element to its index within the array",
    },
    NullableArray {
        structure: "VkVideoDecodeInfoKHR",
        member: "pReferenceSlots",
        stated_by: "the description of VkVideoDecodeInfoKHR::pReferenceSlots, which is NULL or a pointer to an array",
    },
    NullableArray {
        structure: "VkVideoEncodeInfoKHR",
        member: "pReferenceSlots",
        stated_by: "the description of VkVideoEncodeInfoKHR::pReferenceSlots, which is NULL or a pointer to an array",
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
            "{structure}::{member} is a `{}` with the length {:?}, not a pointer to an array whose length a member stores: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            m.full_ty,
            m.len
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
    use super::*;
    use crate::test_support::{
        catch_quietly, chapters_of_the_specification, counted, field, optional, registry, structure,
    };

    fn description<'c>(chapters: &'c str, structure: &str) -> &'c str {
        let Some(at) = chapters.find(&format!("refpage='{structure}'")) else {
            return "";
        };
        let opened = &chapters[at..];
        let start = opened
            .find("\n--\n")
            .map_or(opened.len(), |start| start + 4);
        let body = &opened[start..];
        &body[..body.find("\n--\n").unwrap_or(body.len())]
    }

    fn gives_a_meaning_to_null(description: &str, member: &str) -> bool {
        let text = description.split_whitespace().collect::<Vec<_>>().join(" ");
        [
            format!("pname:{member} is `NULL` or a pointer "),
            format!("pname:{member} is `NULL`, "),
            format!("pname:{member} is not `NULL`"),
        ]
        .iter()
        .any(|statement| {
            text.match_indices(statement.as_str())
                .any(|(at, _)| !text[..at].ends_with(" of "))
        })
    }

    #[test]
    fn the_scan_for_arrays_that_may_be_null_reads_the_ways_the_specification_says_it() {
        for (text, expected) in [
            (
                "  * pname:pValues is `NULL` or a pointer to an array of\n    values.",
                true,
            ),
            (
                "If pname:pValues is `NULL`, then the number of values is returned.",
                true,
            ),
            (
                "the feature is not enabled, and if pname:pValues is `NULL`, it is the same",
                true,
            ),
            (
                "and pname:pValues is not\n`NULL`, each element must: be unique",
                true,
            ),
            (
                "  * pname:pValues is a pointer to an array of values.",
                false,
            ),
            ("pname:pValues must: not be `NULL`", false),
            ("pname:pOthers is `NULL` or a pointer to an array", false),
            (
                "If pname:pOthers is `NULL`, pname:pValues is ignored",
                false,
            ),
            (
                "the pname:pNext member of an element of pname:pValues is not `NULL`",
                false,
            ),
            ("pname:pValuesOfOthers is not `NULL`", false),
        ] {
            assert_eq!(gives_a_meaning_to_null(text, "pValues"), expected, "{text}");
        }
        let chapters = "[open,refpage='VkFirst',desc='First',type='structs']\n--\nabout the first\n--\n\n[open,refpage='VkSecond',desc='Second',type='structs']\n--\nabout the second\n";
        assert_eq!(description(chapters, "VkFirst"), "about the first");
        assert_eq!(description(chapters, "VkSecond"), "about the second\n");
        assert_eq!(description(chapters, "VkThird"), "");
    }

    #[test]
    fn every_array_that_the_specification_allows_to_be_null_is_optional_or_listed() {
        assert_listed_nullable_arrays_exist(Registry::get());
        let Some(chapters) = chapters_of_the_specification() else {
            return;
        };
        let mut found = Vec::new();
        for s in Registry::get().structs.iter().filter(|s| !s.union) {
            let description = description(&chapters, &s.name);
            for m in &s.members {
                let ty = CType::of(m);
                if ty.depth() == 0 || ty.is_array() || m.optional || count_member(s, m).is_none() {
                    continue;
                }
                let listed = NULLABLE_ARRAYS
                    .iter()
                    .any(|listed| listed.structure == s.name && listed.member == m.name);
                assert_eq!(
                    gives_a_meaning_to_null(description, &m.name),
                    listed,
                    "{}::{}: the registry does not mark the pointer as optional, and the list of arrays that the specification allows to be null {} it",
                    s.name,
                    m.name,
                    if listed { "names" } else { "does not name" }
                );
                if listed {
                    found.push((s.name.as_str(), m.name.as_str()));
                }
            }
        }
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
        const CORRECT: &str = "correct the list of arrays that the specification allows to be null although the registry does not mark their pointers as optional, which takes this one from the description of VkPresentInfoKHR::pWaitSemaphores, which is NULL or a pointer to an array";
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
                    "VkPresentInfoKHR::pWaitSemaphores is a `const VkSemaphore*` with the length None, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                counted_elsewhere,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `const VkSemaphore*` with the length Some(\"swapchainCount\"), not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                value,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `VkSemaphore` with the length None, not a pointer to an array whose length a member stores: {CORRECT}"
                ),
            ),
            (
                fixed,
                format!(
                    "VkPresentInfoKHR::pWaitSemaphores is a `const VkSemaphore*` with the length Some(\"waitSemaphoreCount\"), not a pointer to an array whose length a member stores: {CORRECT}"
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
                "the registry has no structure VkDebugUtilsMessengerCallbackDataEXT: correct the list of arrays that the specification allows to be null although the registry does not mark their pointers as optional, which takes `pCmdBufLabels` from "
            ),
            "{message}"
        );
    }
}
