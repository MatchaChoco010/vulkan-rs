use crate::model::{CType, Model, is_opaque};
use vulkan_registry::{Field, Registry, Struct};

#[derive(Debug)]
pub(super) struct BoundedArray {
    pub(super) structure: &'static str,
    pub(super) member: &'static str,
    pub(super) len: &'static str,
    pub(super) largest: &'static str,
    pub(super) largest_bit: Option<LargestBit>,
    pub(super) because: &'static str,
    pub(super) stated_by: &'static str,
}

#[derive(Debug)]
pub(super) struct LargestBit {
    pub(super) member: &'static str,
    pub(super) bits: &'static str,
    pub(super) bit: &'static str,
}

pub(super) const BOUNDED_ARRAYS: &[BoundedArray] = &[
    BoundedArray {
        structure: "StdVideoH265HrdParameters",
        member: "pSubLayerHrdParametersNal",
        len: "*_max_sub_layers_minus1 + 1",
        largest: "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE",
        largest_bit: None,
        because: "`vps_max_sub_layers_minus1` and `sps_max_sub_layers_minus1` are at most 6",
        stated_by: "H.265 7.4.3.1 and 7.4.3.2.1, which give vps_max_sub_layers_minus1 and sps_max_sub_layers_minus1 the range 0 to 6",
    },
    BoundedArray {
        structure: "StdVideoH265HrdParameters",
        member: "pSubLayerHrdParametersVcl",
        len: "*_max_sub_layers_minus1 + 1",
        largest: "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE",
        largest_bit: None,
        because: "`vps_max_sub_layers_minus1` and `sps_max_sub_layers_minus1` are at most 6",
        stated_by: "H.265 7.4.3.1 and 7.4.3.2.1, which give vps_max_sub_layers_minus1 and sps_max_sub_layers_minus1 the range 0 to 6",
    },
    BoundedArray {
        structure: "VkPipelineMultisampleStateCreateInfo",
        member: "pSampleMask",
        len: "(rasterizationSamples + 31) / 32",
        largest: "2",
        largest_bit: Some(LargestBit {
            member: "rasterizationSamples",
            bits: "VkSampleCountFlagBits",
            bit: "VK_SAMPLE_COUNT_64_BIT",
        }),
        because: "`rasterizationSamples` is at most `VK_SAMPLE_COUNT_64_BIT`",
        stated_by: "the bits of VkSampleCountFlagBits, the type of rasterizationSamples",
    },
];

const CORRECT_THE_LIST: &str =
    "correct the list of arrays whose length is computed from values that their setter cannot see";

impl BoundedArray {
    fn assert_matches(&self, ir: &Registry, s: &Struct) {
        let Self {
            structure,
            member,
            len,
            largest_bit,
            stated_by,
            ..
        } = self;
        assert!(
            !s.union,
            "{structure} is a union, and only the members of structures are listed: {CORRECT_THE_LIST}, which takes `{member}` from {stated_by}"
        );
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
            ty.depth() == 1 && !ty.is_array() && !is_opaque(&ty.base),
            "{structure}::{member} is a `{}`, not a pointer to values of a type that the bindings declare: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            m.full_ty
        );
        assert!(
            m.len.as_deref() == Some(*len),
            "{structure}::{member}: the registry gives the length {:?}, not `{len}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            m.len
        );
        let Some(LargestBit {
            member: value,
            bits,
            bit,
        }) = largest_bit
        else {
            return;
        };
        let is_of_the_type = s
            .members
            .iter()
            .any(|m| m.name == *value && m.ty == *bits && CType::of(m).depth() == 0);
        let largest = ir
            .bitmasks
            .iter()
            .find(|bitmask| bitmask.name == *bits)
            .and_then(|bitmask| {
                bitmask
                    .fields
                    .iter()
                    .max_by_key(|field| field.value.parse::<u64>().unwrap_or(u64::MAX))
            })
            .map(|field| field.name.as_str());
        assert!(
            is_of_the_type && largest == Some(*bit),
            "{structure}::{member}: the length `{len}` is largest for the bit {bit} of {bits}, the type of `{value}`, and the registry {}: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            match largest {
                _ if !is_of_the_type => format!("has no member `{value}` of that type"),
                Some(largest) => format!("has {largest} as the largest bit"),
                None => "has no bits of that type".to_string(),
            }
        );
    }

    pub(super) fn largest_len(&self, model: &Model<'_>) -> String {
        if self.largest.bytes().all(|digit| digit.is_ascii_digit()) {
            return self.largest.to_string();
        }
        assert!(
            model.array_len_constants().contains(self.largest),
            "{}::{}: the bindings do not declare {} as a `usize`, because the registry has no such constant or no array has it as its length: {CORRECT_THE_LIST}, which takes this one from {}",
            self.structure,
            self.member,
            self.largest,
            self.stated_by
        );
        crate::naming::const_name(self.largest)
    }
}

pub(super) fn bounded_array(ir: &Registry, s: &Struct, m: &Field) -> Option<&'static BoundedArray> {
    let listed = BOUNDED_ARRAYS
        .iter()
        .find(|listed| listed.structure == s.name && listed.member == m.name)?;
    listed.assert_matches(ir, s);
    Some(listed)
}

pub(crate) fn assert_listed_bounded_arrays_exist(ir: &Registry) {
    for listed in BOUNDED_ARRAYS {
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
        listed.assert_matches(ir, s);
    }
}

#[cfg(test)]
mod tests {
    use super::super::definition;
    use super::*;
    use crate::emit::{doc_of, squash};
    use crate::test_support::{catch_quietly, counted, field, optional, registry, structure};

    fn listed_struct(name: &str) -> Struct {
        Registry::get()
            .structs
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("the registry has no {name}"))
            .clone()
    }

    #[test]
    fn every_length_that_is_computed_from_values_is_listed_with_its_largest_value() {
        let ir = Registry::get();
        assert_listed_bounded_arrays_exist(ir);
        let model = Model::new(ir);
        let mut computed = Vec::new();
        for s in &ir.structs {
            let names_a_member = |name: &str| s.members.iter().any(|member| member.name == name);
            for m in &s.members {
                let ty = CType::of(m);
                let Some(len) = m.len.as_deref() else {
                    continue;
                };
                if ty.depth() == 0 || ty.is_array() {
                    continue;
                }
                let is_a_list_of_names = len
                    .split(',')
                    .all(|part| part == "1" || names_a_member(part));
                let is_a_constant_expression = !len.contains(',')
                    && len
                        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                        .filter(|word| !word.is_empty())
                        .all(|word| {
                            word.bytes().all(|b| b.is_ascii_digit())
                                || ir.constants.iter().any(|c| c.name == word)
                        });
                if is_a_list_of_names || is_a_constant_expression {
                    assert!(
                        bounded_array(ir, s, m).is_none(),
                        "{}::{} is listed, and its length `{len}` is not computed",
                        s.name,
                        m.name
                    );
                    continue;
                }
                assert!(
                    len == "codeSize / 4" || (!s.union && bounded_array(ir, s, m).is_some()),
                    "{}::{}: the length `{len}` is computed from values that the setter cannot see, and the member is not listed with the largest value of the length",
                    s.name,
                    m.name
                );
                if len != "codeSize / 4" {
                    computed.push((s.name.as_str(), m.name.as_str()));
                }
            }
        }
        assert_eq!(
            computed,
            BOUNDED_ARRAYS
                .iter()
                .map(|listed| (listed.structure, listed.member))
                .collect::<Vec<_>>(),
            "the list names the arrays in the order of the registry"
        );
        for listed in BOUNDED_ARRAYS {
            let largest = listed.largest_len(&model);
            assert!(
                largest == listed.largest || largest == crate::naming::const_name(listed.largest),
                "{largest}"
            );
        }
    }

    #[test]
    fn the_largest_sample_mask_has_a_word_for_every_32_samples_of_the_largest_sample_count() {
        let ir = Registry::get();
        let listed = BOUNDED_ARRAYS
            .iter()
            .find(|listed| listed.member == "pSampleMask")
            .expect("the sample mask is listed");
        assert_eq!(listed.len, "(rasterizationSamples + 31) / 32");
        let s = listed_struct(listed.structure);
        let samples = s
            .members
            .iter()
            .find(|m| m.name == "rasterizationSamples")
            .expect("the member that the length names");
        assert_eq!(samples.ty, "VkSampleCountFlagBits");
        let bits = ir
            .bitmasks
            .iter()
            .find(|bits| bits.name == samples.ty)
            .expect("the bits of the sample counts");
        let largest = bits
            .fields
            .iter()
            .map(|bit| bit.value.parse::<u64>().expect("a decimal value"))
            .max()
            .expect("a sample count");
        assert_eq!(largest, 64, "the list says VK_SAMPLE_COUNT_64_BIT");
        assert!(listed.because.contains("VK_SAMPLE_COUNT_64_BIT"));
        assert!(
            bits.fields
                .iter()
                .any(|bit| bit.name == "VK_SAMPLE_COUNT_64_BIT" && bit.value == "64")
        );
        assert_eq!(listed.largest, largest.div_ceil(32).to_string());
    }

    #[test]
    fn the_largest_number_of_sub_layers_is_the_length_of_the_arrays_with_one_element_for_each() {
        let ir = Registry::get();
        let s = listed_struct("StdVideoH265HrdParameters");
        for member in ["pSubLayerHrdParametersNal", "pSubLayerHrdParametersVcl"] {
            let listed = BOUNDED_ARRAYS
                .iter()
                .find(|listed| listed.member == member)
                .unwrap_or_else(|| panic!("{member} is listed"));
            assert_eq!(listed.largest, "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE");
            assert!(listed.because.ends_with("are at most 6"));
        }
        let size = ir
            .constants
            .iter()
            .find(|c| c.name == "STD_VIDEO_H265_SUBLAYERS_LIST_SIZE")
            .expect("the constant");
        assert_eq!(
            size.value, "7",
            "one more than the largest *_max_sub_layers_minus1"
        );
        let for_each_sub_layer = s
            .members
            .iter()
            .filter(|m| m.fixed_size_array == ["STD_VIDEO_H265_SUBLAYERS_LIST_SIZE"])
            .map(|m| m.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            for_each_sub_layer,
            ["cpb_cnt_minus1", "elemental_duration_in_tc_minus1"],
            "the arrays that H.265 E.2.2 indexes with the sub-layer"
        );
    }

    #[test]
    fn the_setter_of_a_listed_array_takes_the_largest_array() {
        let model = Model::new(Registry::get());
        let hrd = definition(&model, &listed_struct("StdVideoH265HrdParameters"));
        let multisample = definition(
            &model,
            &listed_struct("VkPipelineMultisampleStateCreateInfo"),
        );
        for (out, setter) in [
            (
                &hrd,
                "pub fn sub_layer_hrd_parameters_nal(mut self, value: &'a [StdVideoH265SubLayerHrdParameters; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE]) -> Self { self.p_sub_layer_hrd_parameters_nal = value.as_ptr(); self }",
            ),
            (
                &hrd,
                "pub fn sub_layer_hrd_parameters_vcl(mut self, value: &'a [StdVideoH265SubLayerHrdParameters; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE]) -> Self { self.p_sub_layer_hrd_parameters_vcl = value.as_ptr(); self }",
            ),
            (
                &multisample,
                "pub fn sample_mask(mut self, value: &'a [SampleMask; 2]) -> Self { self.p_sample_mask = value.as_ptr(); self }",
            ),
        ] {
            assert!(squash(out).contains(setter), "{setter}: {out}");
        }
        assert_eq!(
            doc_of(&multisample, "pub fn sample_mask("),
            [
                "Sets [`p_sample_mask`](Self::p_sample_mask) to point to an array of `2` elements, the most that the C API accesses: it accesses `(rasterizationSamples + 31) / 32` elements, and `rasterizationSamples` is at most `VK_SAMPLE_COUNT_64_BIT`. It does not access the elements after those. The registry marks the member as optional: it is a null pointer in a structure made by `default()`, and the setter cannot make it null again."
            ]
        );
        let mut required = listed_struct("VkPipelineMultisampleStateCreateInfo");
        for m in required
            .members
            .iter_mut()
            .filter(|m| m.name == "pSampleMask")
        {
            m.optional = false;
            m.full_ty = "VkSampleMask*".to_string();
        }
        let out = definition(&model, &required);
        assert!(
            squash(&out).contains(
                "pub fn sample_mask(mut self, value: &'a mut [SampleMask; 2]) -> Self { self.p_sample_mask = value.as_mut_ptr(); self }"
            ),
            "{out}"
        );
        assert_eq!(
            doc_of(&out, "pub fn sample_mask("),
            [
                "Sets [`p_sample_mask`](Self::p_sample_mask) to point to an array of `2` elements, the most that the C API accesses: it accesses `(rasterizationSamples + 31) / 32` elements, and `rasterizationSamples` is at most `VK_SAMPLE_COUNT_64_BIT`. It does not access the elements after those."
            ]
        );
    }

    #[test]
    fn a_length_computed_from_values_that_is_not_listed_stops_the_generator() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkMasksInfo",
            vec![
                field("samples", "uint32_t"),
                optional(counted(
                    "pSampleMask",
                    "const uint32_t*",
                    "(samples + 31) / 32",
                )),
            ],
        ));
        let model = Model::new(&ir);
        assert_eq!(
            catch_quietly(|| definition(&model, &ir.structs[0])),
            Err("VkMasksInfo::pSampleMask: the length `(samples + 31) / 32` is neither a member, a constant nor a listed expression: find the largest value that the specification allows for it and add the member to the list of arrays whose length is computed from values that their setter cannot see".to_string())
        );
    }

    #[test]
    fn a_listed_array_that_the_registry_changes_stops_the_generator() {
        const CORRECT: &str = "correct the list of arrays whose length is computed from values that their setter cannot see, which takes this one from the bits of VkSampleCountFlagBits, the type of rasterizationSamples";
        let original = listed_struct("VkPipelineMultisampleStateCreateInfo");
        let changed = |change: fn(&mut Field)| {
            let mut s = original.clone();
            for m in s.members.iter_mut().filter(|m| m.name == "pSampleMask") {
                change(m);
            }
            s
        };
        let mut union = original.clone();
        union.union = true;
        for (s, message) in [
            (
                changed(|m| m.name = "pMask".to_string()),
                format!("VkPipelineMultisampleStateCreateInfo has no member `pSampleMask`: {CORRECT}"),
            ),
            (
                changed(|m| m.len = Some("(rasterizationSamples + 63) / 64".to_string())),
                format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask: the registry gives the length Some(\"(rasterizationSamples + 63) / 64\"), not `(rasterizationSamples + 31) / 32`: {CORRECT}"
                ),
            ),
            (
                changed(|m| m.len = None),
                format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask: the registry gives the length None, not `(rasterizationSamples + 31) / 32`: {CORRECT}"
                ),
            ),
            (
                changed(|m| m.full_ty = "const VkSampleMask* const*".to_string()),
                format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask is a `const VkSampleMask* const*`, not a pointer to values of a type that the bindings declare: {CORRECT}"
                ),
            ),
            (
                changed(|m| m.fixed_size_array = vec!["2".to_string()]),
                format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask is a `const VkSampleMask*`, not a pointer to values of a type that the bindings declare: {CORRECT}"
                ),
            ),
            (
                changed(|m| {
                    m.ty = "void".to_string();
                    m.full_ty = "const void*".to_string();
                }),
                format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask is a `const void*`, not a pointer to values of a type that the bindings declare: {CORRECT}"
                ),
            ),
            (
                union,
                "VkPipelineMultisampleStateCreateInfo is a union, and only the members of structures are listed: correct the list of arrays whose length is computed from values that their setter cannot see, which takes `pSampleMask` from the bits of VkSampleCountFlagBits, the type of rasterizationSamples".to_string(),
            ),
        ] {
            let mut ir = registry();
            ir.structs = Registry::get().structs.clone();
            ir.structs.retain(|other| other.name != s.name);
            ir.structs.push(s);
            assert_eq!(
                catch_quietly(|| assert_listed_bounded_arrays_exist(&ir)),
                Err(message)
            );
        }
        let with_bits = |change: fn(&mut Registry)| {
            let mut ir = registry();
            ir.structs = Registry::get().structs.clone();
            ir.bitmasks = Registry::get().bitmasks.clone();
            change(&mut ir);
            catch_quietly(|| assert_listed_bounded_arrays_exist(&ir))
        };
        assert_eq!(with_bits(|_| ()), Ok(()));
        assert_eq!(
            with_bits(|ir| {
                let bits = ir
                    .bitmasks
                    .iter_mut()
                    .find(|bits| bits.name == "VkSampleCountFlagBits")
                    .expect("the sample counts");
                bits.fields.push(vulkan_registry::EnumField {
                    name: "VK_SAMPLE_COUNT_128_BIT".to_string(),
                    value: "128".to_string(),
                    ..Default::default()
                });
            }),
            Err(format!(
                "VkPipelineMultisampleStateCreateInfo::pSampleMask: the length `(rasterizationSamples + 31) / 32` is largest for the bit VK_SAMPLE_COUNT_64_BIT of VkSampleCountFlagBits, the type of `rasterizationSamples`, and the registry has VK_SAMPLE_COUNT_128_BIT as the largest bit: {CORRECT}"
            ))
        );
        for change in [
            (|ir| {
                for s in &mut ir.structs {
                    for m in &mut s.members {
                        if m.name == "rasterizationSamples" {
                            m.ty = "uint32_t".to_string();
                        }
                    }
                }
            }) as fn(&mut Registry),
            |ir| {
                for s in &mut ir.structs {
                    for m in &mut s.members {
                        if m.name == "rasterizationSamples" {
                            m.full_ty = "const VkSampleCountFlagBits*".to_string();
                        }
                    }
                }
            },
        ] {
            assert_eq!(
                with_bits(change),
                Err(format!(
                    "VkPipelineMultisampleStateCreateInfo::pSampleMask: the length `(rasterizationSamples + 31) / 32` is largest for the bit VK_SAMPLE_COUNT_64_BIT of VkSampleCountFlagBits, the type of `rasterizationSamples`, and the registry has no member `rasterizationSamples` of that type: {CORRECT}"
                ))
            );
        }
        assert_eq!(
            with_bits(|ir| ir.bitmasks.clear()),
            Err(format!(
                "VkPipelineMultisampleStateCreateInfo::pSampleMask: the length `(rasterizationSamples + 31) / 32` is largest for the bit VK_SAMPLE_COUNT_64_BIT of VkSampleCountFlagBits, the type of `rasterizationSamples`, and the registry has no bits of that type: {CORRECT}"
            ))
        );
        let message = catch_quietly(|| assert_listed_bounded_arrays_exist(&registry()))
            .expect_err("the registry has none of the listed structures");
        assert!(
            message.starts_with(
                "the registry has no structure StdVideoH265HrdParameters: correct the list of arrays whose length is computed from values that their setter cannot see, which takes `pSubLayerHrdParametersNal` from H.265 "
            ),
            "{message}"
        );
        let mut ir = registry();
        ir.structs = vec![listed_struct("StdVideoH265HrdParameters")];
        let listed = BOUNDED_ARRAYS
            .iter()
            .find(|listed| listed.member == "pSubLayerHrdParametersNal")
            .expect("the listed array");
        let message = catch_quietly(|| listed.largest_len(&Model::new(&ir)))
            .expect_err("the registry has no constant for the largest length");
        assert!(
            message.starts_with(
                "StdVideoH265HrdParameters::pSubLayerHrdParametersNal: the bindings do not declare STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as a `usize`, because the registry has no such constant or no array has it as its length: "
            ),
            "{message}"
        );
    }
}
