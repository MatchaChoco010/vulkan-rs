use super::definition;
use crate::emit::{doc_of, squash};
use crate::model::{CType, Model};
use crate::naming::field_name;
use crate::test_support::{counted, field, optional, registry, structure};
use std::collections::BTreeSet;
use vulkan_registry::{Field, Registry, Struct};

fn count_of<'s>(s: &'s Struct, m: &Field) -> Option<&'s Field> {
    let len = m.len.as_deref()?;
    let count = len.strip_suffix(",1").unwrap_or(len);
    s.members.iter().find(|c| {
        let ty = CType::of(c);
        c.name == count && ty.depth() == 0 && !ty.is_array()
    })
}

fn arrays() -> Registry {
    let mut ir = registry();
    ir.structs
        .push(structure("VkPartNV", vec![field("index", "uint32_t")]));
    ir.structs.push(structure(
        "VkArraysNV",
        vec![
            field("aloneCount", "uint32_t"),
            optional(counted("pAlone", "const uint32_t*", "aloneCount")),
            field("pairCount", "uint32_t"),
            counted("pRequired", "const uint32_t*", "pairCount"),
            optional(counted("pOptional", "const float*", "pairCount")),
            field("partCount", "uint32_t"),
            optional(counted("pParts", "const VkPartNV*", "partCount")),
            optional(counted("ppParts", "const VkPartNV* const*", "partCount,1")),
            field("dataSize", "size_t"),
            optional(counted("pData", "void*", "dataSize")),
            field("plainCount", "uint32_t"),
            counted("pPlain", "const uint32_t*", "plainCount"),
        ],
    ));
    ir
}

#[test]
fn an_empty_slice_makes_an_optional_array_absent() {
    let ir = arrays();
    let model = Model::new(&ir);
    let out = definition(&model, &ir.structs[1]);
    let squashed = squash(&out);
    for setter in [
        "pub fn alone(mut self, value: &'a [u32]) -> Self { if value.is_empty() { if !self.p_alone.is_null() { self.alone_count = 0; } self.p_alone = core::ptr::null(); } else { self.alone_count = len_as_count(value.len(), \"VkArraysNV::pAlone\", \"aloneCount\"); self.p_alone = value.as_ptr(); } self }",
        "pub fn optional(mut self, value: &'a [f32]) -> Self { if value.is_empty() { if !self.p_optional.is_null() && self.p_required.is_null() { self.pair_count = 0; } self.p_optional = core::ptr::null(); } else { if !self.p_required.is_null() { assert_eq!(self.pair_count as usize, value.len(), \"VkArraysNV::pOptional: every array sharing `pairCount` must have the same length\"); } self.pair_count = len_as_count(value.len(), \"VkArraysNV::pOptional\", \"pairCount\"); self.p_optional = value.as_ptr(); } self }",
        "pub fn parts(mut self, value: &'a [PartNV]) -> Self { if value.is_empty() { if !self.p_parts.is_null() && self.pp_parts.is_null() { self.part_count = 0; } self.p_parts = core::ptr::null(); } else {",
        "pub fn parts_ptrs(mut self, value: &'a [&'a PartNV]) -> Self { if value.is_empty() { if !self.pp_parts.is_null() && self.p_parts.is_null() { self.part_count = 0; } self.pp_parts = core::ptr::null(); } else {",
        "pub fn data(mut self, value: &'a mut [u8]) -> Self { if value.is_empty() { if !self.p_data.is_null() { self.data_size = 0; } self.p_data = core::ptr::null_mut(); } else { self.data_size = value.len() as _; self.p_data = value.as_mut_ptr().cast(); } self }",
    ] {
        assert!(squashed.contains(setter), "{setter}: {squashed}");
    }
    assert_eq!(
        doc_of(&out, "pub fn alone("),
        [
            "Sets [`p_alone`](Self::p_alone) to the start of a slice of elements and [`alone_count`](Self::alone_count) to its length.",
            "An empty slice stands for an array that is absent, which the registry allows for this member: it sets [`p_alone`](Self::p_alone) to a null pointer and leaves [`alone_count`](Self::alone_count) as it is. Only if the array was set before, it sets the count back to 0.",
            "# Panics",
            "Panics if the slice has more elements than `aloneCount`, a `uint32_t`, can count.",
        ]
    );
    assert_eq!(
        doc_of(&out, "pub fn optional(")[1],
        "An empty slice stands for an array that is absent, which the registry allows for this member: it sets [`p_optional`](Self::p_optional) to a null pointer and leaves [`pair_count`](Self::pair_count) as it is, and it is not compared with the other arrays that share the count. Only if this array was set before and none of the others is set, it sets the count back to 0."
    );
}

#[test]
fn an_empty_slice_of_an_array_that_is_not_optional_sets_the_count_to_zero() {
    let ir = arrays();
    let model = Model::new(&ir);
    let out = definition(&model, &ir.structs[1]);
    let squashed = squash(&out);
    for setter in [
        "pub fn required(mut self, value: &'a [u32]) -> Self { if !self.p_optional.is_null() { assert_eq!(self.pair_count as usize, value.len(), \"VkArraysNV::pRequired: every array sharing `pairCount` must have the same length\"); } self.pair_count = len_as_count(value.len(), \"VkArraysNV::pRequired\", \"pairCount\"); self.p_required = value.as_ptr(); self }",
        "pub fn plain(mut self, value: &'a [u32]) -> Self { self.plain_count = len_as_count(value.len(), \"VkArraysNV::pPlain\", \"plainCount\"); self.p_plain = value.as_ptr(); self }",
    ] {
        assert!(squashed.contains(setter), "{setter}: {squashed}");
    }
    assert_eq!(
        doc_of(&out, "pub fn plain("),
        [
            "Sets [`p_plain`](Self::p_plain) to the start of a slice of elements and [`plain_count`](Self::plain_count) to its length.",
            "# Panics",
            "Panics if the slice has more elements than `plainCount`, a `uint32_t`, can count.",
        ]
    );
}

#[test]
fn the_registry_setters_take_an_empty_slice_as_an_absent_array_exactly_where_the_pointer_is_optional()
 {
    let model = Model::new(Registry::get());
    let mut absent_when_empty = BTreeSet::new();
    let mut counted_arrays = 0;
    for s in model.ir.structs.iter().filter(|s| !s.union) {
        let out = squash(&definition(&model, s));
        for m in &s.members {
            let ty = CType::of(m);
            if ty.depth() == 0 || ty.is_array() {
                continue;
            }
            let Some(count) = count_of(s, m) else {
                continue;
            };
            let field = field_name(&m.name);
            let count_field = field_name(&count.name);
            if !out.contains(&format!("self.{field} = value.as_")) {
                continue;
            }
            counted_arrays += 1;
            let sets_null = out.contains(&format!("self.{field} = core::ptr::null"));
            let guards_the_count = out.contains(&format!(
                "-> Self {{ if value.is_empty() {{ if !self.{field}.is_null()"
            ));
            let null = if ty.outer_const() {
                "core::ptr::null()"
            } else {
                "core::ptr::null_mut()"
            };
            let leaves_the_count = out.contains(&format!(
                " {{ self.{count_field} = 0; }} self.{field} = {null}; }} else {{ "
            ));
            assert_eq!(
                (sets_null, guards_the_count, leaves_the_count),
                (m.optional, m.optional, m.optional),
                "{}::{}, whose pointer the registry marks as {}: {out}",
                s.name,
                m.name,
                if m.optional { "optional" } else { "required" }
            );
            if m.optional {
                absent_when_empty.insert(format!("{}::{}", s.name, m.name));
            }
        }
    }
    for expected in [
        "VkDescriptorSetLayoutBinding::pImmutableSamplers",
        "VkSubpassDescription::pResolveAttachments",
        "VkSubpassDescription2::pResolveAttachments",
        "VkPresentInfoKHR::pResults",
        "VkTensorDescriptionARM::pStrides",
        "VkGetLatencyMarkerInfoNV::pTimings",
        "VkAccelerationStructureBuildGeometryInfoKHR::ppGeometries",
    ] {
        assert!(
            absent_when_empty.contains(expected),
            "{expected} is not among {absent_when_empty:?}"
        );
    }
    assert!(
        absent_when_empty.len() >= 60 && counted_arrays >= 300,
        "{} of {counted_arrays} counted arrays are optional",
        absent_when_empty.len()
    );
}
