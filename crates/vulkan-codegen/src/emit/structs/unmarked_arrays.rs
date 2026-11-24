use crate::model::{CType, is_opaque};
use vulkan_registry::{Field, Registry, Struct};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum Length {
    InMember(&'static str),
    NotInTheStructure(&'static str),
}

#[derive(Debug)]
pub(super) struct UnmarkedArray {
    pub(super) structure: &'static str,
    pub(super) member: &'static str,
    pub(super) length: Length,
    pub(super) stated_by: &'static str,
}

pub(super) const UNMARKED_ARRAYS: &[UnmarkedArray] = &[
    UnmarkedArray {
        structure: "StdVideoEncodeAV1PictureInfo",
        member: "pBufferRemovalTimes",
        length: Length::NotInTheStructure(
            "the C API reads one element for each operating point of the active sequence header, whose number is the [`VideoEncodeAV1SessionParametersCreateInfoKHR::std_operating_point_count`] that the video session parameters were created with",
        ),
        stated_by: "the description of StdVideoEncodeAV1PictureInfo in the section \"Std Picture Information\" of the chapter on AV1 encode operations",
    },
    UnmarkedArray {
        structure: "VkCommandBufferInheritanceViewportScissorInfoNV",
        member: "pViewportDepths",
        length: Length::InMember("viewportDepthCount"),
        stated_by: "VUID-VkCommandBufferInheritanceViewportScissorInfoNV-viewportScissor2D-04785",
    },
    UnmarkedArray {
        structure: "VkDeviceFaultInfoEXT",
        member: "pAddressInfos",
        length: Length::NotInTheStructure(
            "the command that writes the elements, [`get_device_fault_info_ext`](crate::Device::get_device_fault_info_ext), reads it from [`DeviceFaultCountsEXT::address_info_count`], which the caller sets to the number of elements of the slice",
        ),
        stated_by: "the descriptions of VkDeviceFaultInfoEXT::pAddressInfos and of vkGetDeviceFaultInfoEXT",
    },
    UnmarkedArray {
        structure: "VkDeviceFaultInfoEXT",
        member: "pVendorInfos",
        length: Length::NotInTheStructure(
            "the command that writes the elements, [`get_device_fault_info_ext`](crate::Device::get_device_fault_info_ext), reads it from [`DeviceFaultCountsEXT::vendor_info_count`], which the caller sets to the number of elements of the slice",
        ),
        stated_by: "the descriptions of VkDeviceFaultInfoEXT::pVendorInfos and of vkGetDeviceFaultInfoEXT",
    },
];

impl UnmarkedArray {
    fn assert_matches(&self, s: &Struct) {
        let Self {
            structure,
            member,
            length,
            stated_by,
        } = self;
        let remove = format!(
            "correct the list of arrays that the registry does not mark, which takes this one from {stated_by}"
        );
        assert!(
            !s.union,
            "{structure} is a union, and only the members of structures are listed as arrays: {remove}"
        );
        let m = s
            .members
            .iter()
            .find(|m| m.name == *member)
            .unwrap_or_else(|| panic!("{structure} has no member `{member}`: {remove}"));
        let ty = CType::of(m);
        assert!(
            ty.depth() == 1 && !ty.is_array() && !is_opaque(&ty.base),
            "{structure}::{member} is a `{}`, not a pointer to values of a type that the bindings declare: {remove}",
            m.full_ty
        );
        assert!(
            m.len.is_none(),
            "{structure}::{member}: the registry gives the length `{}` now: {remove}",
            m.len.as_deref().unwrap_or_default()
        );
        if let Length::InMember(count) = length {
            let counts = s.members.iter().any(|c| {
                let ty = CType::of(c);
                c.name == *count && ty.depth() == 0 && !ty.is_array()
            });
            assert!(
                counts,
                "{structure} has no member `{count}` that can hold the length of `{member}`: {remove}"
            );
        }
    }
}

pub(super) fn length(s: &Struct, m: &Field) -> Option<Length> {
    let array = UNMARKED_ARRAYS
        .iter()
        .find(|array| array.structure == s.name && array.member == m.name)?;
    array.assert_matches(s);
    Some(array.length)
}

pub(super) fn length_that_no_member_stores(s: &Struct, m: &Field) -> Option<String> {
    match length(s, m) {
        Some(Length::NotInTheStructure(stated)) => Some(stated.to_string()),
        Some(Length::InMember(_)) | None => None,
    }
}

pub(super) fn assert_listed_members_exist(s: &Struct) {
    for array in UNMARKED_ARRAYS
        .iter()
        .filter(|array| array.structure == s.name)
    {
        array.assert_matches(s);
    }
}

pub(crate) fn assert_listed_structures_exist(ir: &Registry) {
    for array in UNMARKED_ARRAYS {
        let s = ir
            .structs
            .iter()
            .find(|s| s.name == array.structure)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no structure {}: correct the list of arrays that the registry does not mark, which takes `{}` from {}",
                    array.structure, array.member, array.stated_by
                )
            });
        array.assert_matches(s);
    }
}

#[cfg(test)]
mod tests {
    use super::super::{definition, unions};
    use super::*;
    use crate::emit::{doc_of, squash};
    use crate::model::Model;
    use crate::naming::field_name;
    use crate::test_support::{catch_quietly, counted, field, registry, structure};
    use vulkan_registry::Docs;

    fn generated(model: &Model<'_>, name: &str) -> String {
        let s = model
            .get_struct(name)
            .unwrap_or_else(|| panic!("the registry has no {name}"));
        definition(model, s)
    }

    #[test]
    fn an_array_whose_length_is_not_in_the_structure_takes_a_slice_and_stores_no_length() {
        let model = Model::new(Registry::get());
        let fault = generated(&model, "VkDeviceFaultInfoEXT");
        for (setter, element, member) in [
            (
                "address_infos",
                "DeviceFaultAddressInfoKHR",
                "p_address_infos",
            ),
            ("vendor_infos", "DeviceFaultVendorInfoKHR", "p_vendor_infos"),
        ] {
            assert!(
                squash(&fault).contains(&format!(
                    "pub fn {setter}(mut self, value: &'a mut [{element}]) -> Self {{ self.{member} = if value.is_empty() {{ core::ptr::null_mut() }} else {{ value.as_mut_ptr() }}; self }}"
                )),
                "{setter}: {fault}"
            );
        }
        assert_eq!(
            doc_of(&fault, "pub fn address_infos("),
            [
                "Sets [`p_address_infos`](Self::p_address_infos) to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: the command that writes the elements, [`get_device_fault_info_ext`](crate::Device::get_device_fault_info_ext), reads it from [`DeviceFaultCountsEXT::address_info_count`], which the caller sets to the number of elements of the slice. The registry marks the member as optional, and the specification describes what a null pointer means for it."
            ]
        );
        let picture = generated(&model, "StdVideoEncodeAV1PictureInfo");
        assert!(
            squash(&picture).contains(
                "pub fn buffer_removal_times(mut self, value: &'a [u32]) -> Self { self.p_buffer_removal_times = if value.is_empty() { core::ptr::null() } else { value.as_ptr() }; self }"
            ),
            "{picture}"
        );
        assert_eq!(
            doc_of(&picture, "pub fn buffer_removal_times("),
            [
                "Sets [`p_buffer_removal_times`](Self::p_buffer_removal_times) to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: the C API reads one element for each operating point of the active sequence header, whose number is the [`VideoEncodeAV1SessionParametersCreateInfoKHR::std_operating_point_count`] that the video session parameters were created with. The registry marks the member as optional, and the specification describes what a null pointer means for it."
            ]
        );
    }

    #[test]
    fn an_array_whose_length_is_in_a_member_the_registry_does_not_name_sets_that_member() {
        let model = Model::new(Registry::get());
        let inheritance = generated(&model, "VkCommandBufferInheritanceViewportScissorInfoNV");
        assert!(
            squash(&inheritance).contains(
                "pub fn viewport_depths(mut self, value: &'a [Viewport]) -> Self { self.viewport_depth_count = len_as_count(value.len(), \"VkCommandBufferInheritanceViewportScissorInfoNV::pViewportDepths\", \"viewportDepthCount\"); self.p_viewport_depths = value.as_ptr(); self }"
            ),
            "{inheritance}"
        );
        assert_eq!(
            doc_of(&inheritance, "pub fn viewport_depths("),
            [
                "Sets [`p_viewport_depths`](Self::p_viewport_depths) to the start of a slice of elements and [`viewport_depth_count`](Self::viewport_depth_count) to its length.",
                "# Panics",
                "Panics if the slice has more elements than `viewportDepthCount`, a `uint32_t`, can count.",
            ]
        );
    }

    #[test]
    fn every_listed_array_is_in_the_registry() {
        assert_listed_structures_exist(Registry::get());
        let mut listed = UNMARKED_ARRAYS
            .iter()
            .map(|array| (array.structure, array.member))
            .collect::<Vec<_>>();
        let len = listed.len();
        listed.sort_unstable();
        listed.dedup();
        assert_eq!(listed.len(), len, "a member is listed twice");
        for array in UNMARKED_ARRAYS {
            assert!(
                !array.stated_by.is_empty(),
                "{}::{} is listed without the place where the specification states it",
                array.structure,
                array.member
            );
        }
    }

    #[test]
    fn a_registry_without_a_listed_structure_stops_the_generator() {
        let message = catch_quietly(|| assert_listed_structures_exist(&registry()))
            .expect_err("the registry has none of the listed structures");
        assert!(
            message.starts_with(
                "the registry has no structure StdVideoEncodeAV1PictureInfo: correct the list of arrays that the registry does not mark, which takes `pBufferRemovalTimes` from "
            ),
            "{message}"
        );
    }

    fn listed_structure(array: &UnmarkedArray) -> Struct {
        Registry::get()
            .structs
            .iter()
            .find(|s| s.name == array.structure)
            .unwrap_or_else(|| panic!("the registry has no {}", array.structure))
            .clone()
    }

    fn stops_the_generator(s: &Struct) -> String {
        let mut ir = registry();
        ir.structs.push(s.clone());
        catch_quietly(|| assert_listed_members_exist(&ir.structs[0]))
            .expect_err("the structure does not match the list")
    }

    #[test]
    fn a_listed_array_that_the_registry_changes_stops_the_generator() {
        for array in UNMARKED_ARRAYS {
            let UnmarkedArray {
                structure, member, ..
            } = array;
            let original = listed_structure(array);
            assert_listed_members_exist(&original);

            let mut marked = original.clone();
            for m in marked.members.iter_mut().filter(|m| m.name == *member) {
                m.len = Some("someCount".to_string());
            }
            let message = stops_the_generator(&marked);
            assert!(
                message.starts_with(&format!(
                    "{structure}::{member}: the registry gives the length `someCount` now: "
                )),
                "{message}"
            );

            let mut removed = original.clone();
            removed.members.retain(|m| m.name != *member);
            let message = stops_the_generator(&removed);
            assert!(
                message.starts_with(&format!("{structure} has no member `{member}`: ")),
                "{message}"
            );

            let mut by_value = original.clone();
            for m in by_value.members.iter_mut().filter(|m| m.name == *member) {
                m.full_ty = m.ty.clone();
            }
            let message = stops_the_generator(&by_value);
            assert!(
                message.starts_with(&format!("{structure}::{member} is a `")),
                "{message}"
            );

            let mut union = original.clone();
            union.union = true;
            let message = stops_the_generator(&union);
            assert!(
                message.starts_with(&format!("{structure} is a union, ")),
                "{message}"
            );

            if let Length::InMember(count) = array.length {
                let mut uncounted = original.clone();
                uncounted.members.retain(|m| m.name != count);
                let message = stops_the_generator(&uncounted);
                assert!(
                    message.starts_with(&format!(
                        "{structure} has no member `{count}` that can hold the length of `{member}`: "
                    )),
                    "{message}"
                );
            }
        }
    }

    #[test]
    fn structures_that_are_not_listed_are_generated_from_the_registry_alone() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkFaultInfoNV",
            vec![
                field("addressInfoCount", "uint32_t"),
                field("pAddressInfos", "uint32_t*"),
                counted("pVendorInfos", "uint32_t*", "addressInfoCount"),
            ],
        ));
        let s = &ir.structs[0];
        assert_listed_members_exist(s);
        assert_eq!(length(s, &s.members[1]), None);
        let out = squash(&definition(&Model::new(&ir), s));
        assert!(
            out.contains(
                "pub fn address_infos(mut self, value: &'a mut u32) -> Self { self.p_address_infos = core::ptr::from_mut(value); self }"
            ),
            "{out}"
        );
    }

    fn takes_one_reference(generated: &str, m: &Field) -> bool {
        let field = field_name(&m.name);
        ["from_ref", "from_mut"].iter().any(|from| {
            generated.contains(&format!("self.{field} = core::ptr::{from}(value);"))
                || generated.contains(&format!("union.{field} = core::ptr::{from}(value);"))
        })
    }

    #[test]
    fn no_member_that_the_specification_describes_as_an_array_takes_one_reference() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut references = 0;
        let mut described = 0;
        for s in &model.ir.structs {
            let generated = squash(&if s.union {
                unions::definition(&model, s)
            } else {
                definition(&model, s)
            });
            for m in &s.members {
                if !takes_one_reference(&generated, m) {
                    continue;
                }
                references += 1;
                let Some(description) = model
                    .page(&s.name)
                    .and_then(|page| page.items.iter().find(|described| described.name == m.name))
                else {
                    continue;
                };
                described += 1;
                let text = description
                    .text
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                assert!(
                    !text.contains("array") && !text.contains("null-terminated"),
                    "{}::{} takes a reference to one value, but the specification describes it as: {text}",
                    s.name,
                    m.name
                );
            }
        }
        assert!(
            references >= 150 && described >= 100,
            "only {described} of {references} members that take one reference are described"
        );
    }

    #[test]
    fn every_string_member_of_the_registry_takes_a_c_string() {
        let model = Model::new(Registry::get());
        let mut strings = 0;
        for s in &model.ir.structs {
            let generated = squash(&if s.union {
                unions::definition(&model, s)
            } else {
                definition(&model, s)
            });
            assert!(
                !generated.contains("value: &'a c_char")
                    && !generated.contains("value: &'a mut c_char"),
                "{} borrows one character: {generated}",
                s.name
            );
            for m in &s.members {
                let ty = CType::of(m);
                if ty.base != "char" || ty.depth() != 1 || ty.is_array() {
                    continue;
                }
                strings += 1;
                assert!(
                    m.null_terminated && ty.outer_const(),
                    "{}::{} is a `{}` that the registry does not mark as a NUL-terminated string",
                    s.name,
                    m.name,
                    m.full_ty
                );
                let field = field_name(&m.name);
                assert!(
                    generated.contains(&format!(
                        "(mut self, value: &'a CStr) -> Self {{ self.{field} = value.as_ptr(); self }}"
                    )) || generated.contains(&format!(
                        "(value: &'a CStr) -> Self {{ let mut union = Self::default(); union.{field} = value.as_ptr(); union }}"
                    )),
                    "{}::{} does not take a `&CStr`: {generated}",
                    s.name,
                    m.name
                );
            }
        }
        assert!(strings >= 10, "only {strings} string members were checked");
    }

    #[test]
    fn a_union_takes_a_nul_terminated_string_as_a_c_string() {
        let mut ir = registry();
        let mut string = field("valueString", "const char*");
        string.null_terminated = true;
        let mut u = structure(
            "VkValueDataNV",
            vec![
                field("value32", "uint32_t"),
                string,
                field("pBytes", "const char*"),
            ],
        );
        u.union = true;
        ir.structs.push(u);
        let out = unions::definition(&Model::new(&ir), &ir.structs[0]);
        assert!(
            squash(&out).contains(
                "pub fn value_string(value: &'a CStr) -> Self { let mut union = Self::default(); union.value_string = value.as_ptr(); union }"
            ),
            "{out}"
        );
        assert_eq!(
            doc_of(&out, "pub fn value_string("),
            [
                "Creates the union with [`value_string`](Self::value_string) pointing to the NUL-terminated string `value`. The bytes of the union outside [`value_string`](Self::value_string) are zero."
            ]
        );
        assert!(
            squash(&out).contains(
                "pub fn bytes(value: &'a c_char) -> Self { let mut union = Self::default(); union.p_bytes = core::ptr::from_ref(value); union }"
            ),
            "{out}"
        );
    }
}
