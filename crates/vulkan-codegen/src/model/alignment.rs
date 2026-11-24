use super::{CType, Model};
use vulkan_registry::{Command, Field, Registry, Struct};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AlignedOutput {
    pub(crate) command: &'static str,
    pub(crate) parameter: &'static str,
    pub(crate) alignment: usize,
    pub(crate) stated_by: &'static str,
    pub(crate) use_of_the_bytes: &'static str,
}

pub(crate) const ALIGNED_OUTPUTS: &[AlignedOutput] = &[AlignedOutput {
    command: "vkGetShaderBinaryDataEXT",
    parameter: "pData",
    alignment: 16,
    stated_by: "VUID-vkGetShaderBinaryDataEXT-None-08499",
    use_of_the_bytes: "Binary shader code that is passed to [`ShaderCreateInfoEXT::code`] has to be aligned to 16 bytes as well, so copy the bytes to memory with that alignment unless the vector happens to have it.",
}];

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AlignedInput {
    pub(crate) structure: &'static str,
    pub(crate) member: &'static str,
    pub(crate) requirement: &'static str,
    pub(crate) stated_by: &'static str,
}

pub(crate) const ALIGNED_INPUTS: &[AlignedInput] = &[AlignedInput {
    structure: "VkShaderCreateInfoEXT",
    member: "pCode",
    requirement: "The C API requires the bytes to start at an address that is a multiple of 16 for binary shader code and a multiple of 4 for SPIR-V. A `&[u8]` guarantees neither, so the caller has to borrow the bytes from memory with that alignment.",
    stated_by: "VUID-VkShaderCreateInfoEXT-pCode-08492 and VUID-VkShaderCreateInfoEXT-pCode-08493",
}];

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AlignedSlice {
    pub(crate) command: &'static str,
    pub(crate) parameter: &'static str,
    pub(crate) alignment: usize,
    pub(crate) wider: Option<WiderAlignment>,
    pub(crate) stated_by: &'static str,
    pub(crate) unchecked: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct WiderAlignment {
    pub(crate) flags: &'static str,
    pub(crate) bits: &'static str,
    pub(crate) bit: &'static str,
    pub(crate) alignment: usize,
}

pub(crate) const ALIGNED_SLICES: &[AlignedSlice] = &[AlignedSlice {
    command: "vkGetQueryPoolResults",
    parameter: "pData",
    alignment: 4,
    wider: Some(WiderAlignment {
        flags: "flags",
        bits: "VkQueryResultFlagBits",
        bit: "VK_QUERY_RESULT_64_BIT",
        alignment: 8,
    }),
    stated_by: "VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815",
    unchecked: "For a pool of performance queries, VUID-vkGetQueryPoolResults-queryType-03229 requires the alignment of [`PerformanceCounterResultKHR`] instead, which the wrapper does not check, since it does not know the type of the pool.",
}];

const CORRECT_THE_LIST: &str =
    "correct the list of memory that the specification requires to be aligned";

impl AlignedOutput {
    fn assert_matches(&self, c: &Command) {
        let Self {
            command,
            parameter,
            alignment,
            stated_by,
            ..
        } = self;
        assert!(
            alignment.is_power_of_two(),
            "{command}::{parameter}: the listed alignment {alignment} is not a power of two"
        );
        let p = c
            .params
            .iter()
            .find(|p| p.name == *parameter)
            .unwrap_or_else(|| {
                panic!(
                    "{} has no parameter `{parameter}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
                    c.name
                )
            });
        let ty = CType::of(p);
        assert!(
            ty.pointers == [false]
                && !ty.is_array()
                && matches!(ty.base.as_str(), "void" | "uint8_t"),
            "{}::{parameter} is a `{}`, not a pointer to bytes that the command writes: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            c.name,
            p.full_ty
        );
    }
}

impl AlignedSlice {
    fn assert_matches(&self, c: &Command) {
        let Self {
            command,
            parameter,
            alignment,
            wider,
            stated_by,
            ..
        } = self;
        let alignments = [
            Some(*alignment),
            wider.as_ref().map(|wider| wider.alignment),
        ];
        for alignment in alignments.into_iter().flatten() {
            assert!(
                alignment.is_power_of_two(),
                "{command}::{parameter}: the listed alignment {alignment} is not a power of two"
            );
        }
        let p = c
            .params
            .iter()
            .find(|p| p.name == *parameter)
            .unwrap_or_else(|| {
                panic!(
                    "{} has no parameter `{parameter}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
                    c.name
                )
            });
        let ty = CType::of(p);
        assert!(
            ty.pointers == [false]
                && !ty.is_array()
                && p.len.is_some()
                && matches!(ty.base.as_str(), "void" | "uint8_t"),
            "{}::{parameter} is a `{}`, not a pointer to bytes of a stated length that the caller passes: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            c.name,
            p.full_ty
        );
        if let Some(wider) = wider {
            let flags = c.params.iter().find(|p| p.name == wider.flags);
            assert!(
                flags.is_some_and(|flags| CType::of(flags).depth() == 0),
                "{} has no parameter `{}` that holds flags: {CORRECT_THE_LIST}, which takes `{parameter}` from {stated_by}",
                c.name,
                wider.flags
            );
        }
    }
}

impl AlignedInput {
    fn assert_matches(&self, s: &Struct) {
        let Self {
            structure,
            member,
            stated_by,
            ..
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
            ty.pointers == [true] && !ty.is_array() && m.len.is_some(),
            "{structure}::{member} is a `{}`, not a pointer to an array of a stated length that the C API reads: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            m.full_ty
        );
    }
}

impl Model<'_> {
    fn command_behind<'n>(&'n self, name: &'n str) -> &'n str {
        self.command_aliases.get(name).copied().unwrap_or(name)
    }

    pub(crate) fn aligned_slice(&self, c: &Command) -> Option<&'static AlignedSlice> {
        let command = self.command_behind(&c.name);
        let slice = ALIGNED_SLICES
            .iter()
            .find(|slice| self.command_behind(slice.command) == command)?;
        slice.assert_matches(c);
        let wider = slice.wider.as_ref();
        if let Some(wider) = wider {
            let has_the_bit = self.ir.bitmasks.iter().any(|bits| {
                bits.name == wider.bits && bits.fields.iter().any(|bit| bit.name == wider.bit)
            });
            assert!(
                has_the_bit,
                "{} has no bit {}: {CORRECT_THE_LIST}, which takes `{}` of {} from {}",
                wider.bits, wider.bit, slice.parameter, slice.command, slice.stated_by
            );
        }
        Some(slice)
    }

    pub(crate) fn aligned_outputs(&self, c: &Command) -> Vec<&'static AlignedOutput> {
        let command = self.command_behind(&c.name);
        let listed = ALIGNED_OUTPUTS
            .iter()
            .filter(|output| self.command_behind(output.command) == command)
            .collect::<Vec<_>>();
        for output in &listed {
            output.assert_matches(c);
        }
        listed
    }
}

pub(crate) fn aligned_input(s: &Struct, m: &Field) -> Option<&'static AlignedInput> {
    let input = ALIGNED_INPUTS
        .iter()
        .find(|input| input.structure == s.name && input.member == m.name)?;
    input.assert_matches(s);
    Some(input)
}

pub(crate) fn assert_listed_aligned_memory_exists(ir: &Registry) {
    for output in ALIGNED_OUTPUTS {
        let c = ir
            .commands
            .iter()
            .find(|c| c.name == output.command)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no command {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    output.command, output.parameter, output.stated_by
                )
            });
        output.assert_matches(c);
    }
    for input in ALIGNED_INPUTS {
        let s = ir
            .structs
            .iter()
            .find(|s| s.name == input.structure)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no structure {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    input.structure, input.member, input.stated_by
                )
            });
        input.assert_matches(s);
    }
    for slice in ALIGNED_SLICES {
        let c = ir
            .commands
            .iter()
            .find(|c| c.name == slice.command)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no command {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    slice.command, slice.parameter, slice.stated_by
                )
            });
        slice.assert_matches(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::{commands, doc_of, squash, structs};
    use crate::test_support::{catch_quietly, counted, field, optional, registry, result_command};
    use std::collections::BTreeSet;
    use vulkan_registry::{EnumField, Handle};

    const LISTED: &str = "vkGetShaderBinaryDataEXT";

    fn shader_binary_data(name: &str) -> Command {
        let mut c = result_command(
            name,
            &["VK_SUCCESS", "VK_INCOMPLETE"],
            vec![
                field("device", "VkDevice"),
                field("shader", "VkShaderEXT"),
                optional(field("pDataSize", "size_t*")),
                optional(counted("pData", "void*", "pDataSize")),
            ],
        );
        c.device = true;
        c
    }

    fn registry_with(commands: Vec<Command>) -> Registry {
        let mut ir = registry();
        let results = ir.enums.iter_mut().find(|e| e.name == "VkResult").unwrap();
        for (name, value) in [
            ("VK_SUCCESS", "0"),
            ("VK_NOT_READY", "1"),
            ("VK_INCOMPLETE", "5"),
        ] {
            results.fields.push(EnumField {
                name: name.to_string(),
                value: value.to_string(),
                ..Default::default()
            });
        }
        for (name, dispatchable) in [("VkDevice", true), ("VkShaderEXT", false)] {
            ir.handles.push(Handle {
                name: name.to_string(),
                object_type: format!("VK_OBJECT_TYPE_{}", name.to_uppercase()),
                dispatchable,
                ..Default::default()
            });
        }
        ir.commands = commands;
        ir
    }

    fn generated(ir: &Registry) -> String {
        squash(&commands::emit(&Model::new(ir)))
    }

    const ALIGNED_BODY: &str = "loop { let mut count: usize = 0; self.get_shader_binary_data_ext_raw(shader, &mut count, core::ptr::null_mut()).into_result()?; let mut values: Vec<u8> = alloc::vec![0u8; count.saturating_add(15)]; let padding = values.as_ptr().addr().wrapping_neg() % 16; let result = self.get_shader_binary_data_ext_raw(shader, &mut count, values.as_mut_ptr().add(padding).cast()); if result == VkResult::INCOMPLETE { continue; } result.into_result()?; let written = count.min(values.len() - 15); values.copy_within(padding..padding + written, 0); values.truncate(written); return Ok(values); }";

    #[test]
    fn a_listed_command_writes_at_an_aligned_address_and_returns_the_bytes_from_the_start() {
        let ir = registry_with(vec![shader_binary_data(LISTED)]);
        let out = generated(&ir);
        assert!(
            out.contains(&format!(
                "pub unsafe fn get_shader_binary_data_ext(&self, shader: ShaderEXT) -> crate::vk::Result<Vec<u8>> {{ unsafe {{ {ALIGNED_BODY} }} }}"
            )),
            "{out}"
        );
        let doc = doc_of(
            &commands::emit(&Model::new(&ir)),
            "pub unsafe fn get_shader_binary_data_ext(",
        )
        .join(" ");
        assert!(
            doc.contains(
                "and returns what it writes to `pData`. The command requires `pData` to be aligned to 16 bytes, so the wrapper lets it write at such an address and then moves the bytes to the start of the returned vector, which does not have to be aligned to 16 bytes itself. Binary shader code that is passed to [`ShaderCreateInfoEXT::code`] has to be aligned to 16 bytes as well, so copy the bytes to memory with that alignment unless the vector happens to have it."
            ),
            "{doc}"
        );
    }

    #[test]
    fn a_command_that_is_not_listed_writes_to_the_start_of_the_vector() {
        let out = generated(&registry_with(vec![shader_binary_data(
            "vkGetProgramBinaryDataNV",
        )]));
        assert!(
            out.contains(
                "let mut values: Vec<u8> = alloc::vec![0u8; count]; let result = self.get_program_binary_data_nv_raw(shader, &mut count, values.as_mut_ptr().cast()); if result == VkResult::INCOMPLETE { continue; } result.into_result()?; values.truncate(count); return Ok(values);"
            ),
            "{out}"
        );
        assert!(!out.contains("padding"), "{out}");
    }

    #[test]
    fn a_command_without_a_result_aligns_the_bytes_too() {
        let mut c = shader_binary_data(LISTED);
        c.return_ty = "void".to_string();
        c.success_codes.clear();
        let out = generated(&registry_with(vec![c]));
        assert!(
            out.contains(
                "let mut count: usize = 0; self.get_shader_binary_data_ext_raw(shader, &mut count, core::ptr::null_mut()); let mut values: Vec<u8> = alloc::vec![0u8; count.saturating_add(15)]; let padding = values.as_ptr().addr().wrapping_neg() % 16; self.get_shader_binary_data_ext_raw(shader, &mut count, values.as_mut_ptr().add(padding).cast()); let written = count.min(values.len() - 15); values.copy_within(padding..padding + written, 0); values.truncate(written); values"
            ),
            "{out}"
        );
    }

    #[test]
    fn a_count_narrower_than_usize_is_widened_before_the_padding_is_added() {
        let mut c = shader_binary_data(LISTED);
        c.params[2] = optional(field("pDataSize", "uint32_t*"));
        let out = generated(&registry_with(vec![c]));
        assert!(
            out.contains(
                "let mut values: Vec<u8> = alloc::vec![0u8; (count as usize).saturating_add(15)];"
            ) && out.contains("let written = (count as usize).min(values.len() - 15);"),
            "{out}"
        );
    }

    #[test]
    fn the_aliases_of_a_listed_command_are_aligned_as_well() {
        let mut alias = shader_binary_data("vkGetShaderBinaryDataKHR");
        alias.alias = Some(LISTED.to_string());
        let out = generated(&registry_with(vec![shader_binary_data(LISTED), alias]));
        assert_eq!(out.matches(".add(padding).cast()").count(), 2, "{out}");

        let mut listed = shader_binary_data(LISTED);
        listed.alias = Some("vkGetShaderBinaryData".to_string());
        let out = generated(&registry_with(vec![
            shader_binary_data("vkGetShaderBinaryData"),
            listed,
        ]));
        assert_eq!(out.matches(".add(padding).cast()").count(), 2, "{out}");
    }

    #[test]
    fn a_listed_command_that_the_wrapper_cannot_align_stops_the_generator() {
        let mut not_an_enumeration = shader_binary_data(LISTED);
        not_an_enumeration
            .success_codes
            .push("VK_NOT_READY".to_string());
        let mut typed = shader_binary_data(LISTED);
        typed.params[3] = optional(counted("pData", "uint64_t*", "pDataSize"));
        let mut renamed = shader_binary_data(LISTED);
        renamed.params[3] = optional(counted("pBinary", "void*", "pDataSize"));
        let mut taken = shader_binary_data(LISTED);
        taken.params[1] = field("padding", "uint32_t");
        for (c, message) in [
            (
                not_an_enumeration,
                "vkGetShaderBinaryDataEXT: `pData` is listed as memory that has to be aligned to 16 bytes (VUID-vkGetShaderBinaryDataEXT-None-08499), but the wrapper does not allocate it: only an enumeration that returns the bytes does",
            ),
            (
                typed,
                "vkGetShaderBinaryDataEXT::pData is a `uint64_t*`, not a pointer to bytes that the command writes: correct the list of memory that the specification requires to be aligned, which takes this one from VUID-vkGetShaderBinaryDataEXT-None-08499",
            ),
            (
                renamed,
                "vkGetShaderBinaryDataEXT has no parameter `pData`: correct the list of memory that the specification requires to be aligned, which takes this one from VUID-vkGetShaderBinaryDataEXT-None-08499",
            ),
            (
                taken,
                "vkGetShaderBinaryDataEXT: the C parameter `padding` and the local `padding` that places the C parameter `pData` at an aligned address both become the Rust name `padding`",
            ),
        ] {
            let ir = registry_with(vec![c]);
            assert_eq!(
                catch_quietly(|| generated(&ir)).expect_err("the generator stops"),
                message
            );
        }
    }

    #[test]
    fn a_listed_member_that_is_not_a_counted_array_of_input_stops_the_generator() {
        const CORRECT: &str = "correct the list of memory that the specification requires to be aligned, which takes this one from VUID-VkShaderCreateInfoEXT-pCode-08492 and VUID-VkShaderCreateInfoEXT-pCode-08493";
        let size = || field("codeSize", "size_t");
        for (members, message) in [
            (
                vec![size(), counted("pBinary", "const void*", "codeSize")],
                "VkShaderCreateInfoEXT has no member `pCode`".to_string(),
            ),
            (
                vec![size(), field("pCode", "const void*")],
                "VkShaderCreateInfoEXT::pCode is a `const void*`, not a pointer to an array of a stated length that the C API reads".to_string(),
            ),
            (
                vec![size(), counted("pCode", "void*", "codeSize")],
                "VkShaderCreateInfoEXT::pCode is a `void*`, not a pointer to an array of a stated length that the C API reads".to_string(),
            ),
            (
                vec![size(), counted("pCode", "const void* const*", "codeSize")],
                "VkShaderCreateInfoEXT::pCode is a `const void* const*`, not a pointer to an array of a stated length that the C API reads".to_string(),
            ),
        ] {
            let mut ir = registry_with(vec![shader_binary_data(LISTED)]);
            ir.structs.push(crate::test_support::structure(
                "VkShaderCreateInfoEXT",
                members,
            ));
            assert_eq!(
                catch_quietly(|| assert_listed_aligned_memory_exists(&ir)),
                Err(format!("{message}: {CORRECT}"))
            );
            let s = &ir.structs[0];
            if let Some(code) = s.members.iter().find(|m| m.name == "pCode") {
                assert_eq!(
                    catch_quietly(|| aligned_input(s, code).is_some()),
                    Err(format!("{message}: {CORRECT}"))
                );
            }
        }
        let mut ir = registry_with(vec![shader_binary_data(LISTED), query()]);
        ir.structs.push(crate::test_support::structure(
            "VkShaderCreateInfoEXT",
            vec![size(), counted("pCode", "const void*", "codeSize")],
        ));
        assert_listed_aligned_memory_exists(&ir);
        let s = &ir.structs[0];
        assert_eq!(aligned_input(s, &s.members[1]), Some(&ALIGNED_INPUTS[0]));
        assert_eq!(aligned_input(s, &s.members[0]), None);
    }

    #[test]
    fn everything_listed_is_in_the_registry() {
        assert_listed_aligned_memory_exists(Registry::get());
        for output in ALIGNED_OUTPUTS {
            assert!(
                output.stated_by.starts_with("VUID-"),
                "{}::{} is listed without its valid usage statement",
                output.command,
                output.parameter
            );
        }
        for slice in ALIGNED_SLICES {
            assert!(
                slice.stated_by.starts_with("VUID-"),
                "{}::{} is listed without its valid usage statement",
                slice.command,
                slice.parameter
            );
        }
        for input in ALIGNED_INPUTS {
            assert!(
                input.stated_by.starts_with("VUID-"),
                "{}::{} is listed without its valid usage statement",
                input.structure,
                input.member
            );
        }
        let message = catch_quietly(|| assert_listed_aligned_memory_exists(&registry()))
            .expect_err("an empty registry has nothing that is listed");
        assert_eq!(
            message,
            "the registry has no command vkGetShaderBinaryDataEXT: correct the list of memory that the specification requires to be aligned, which takes `pData` from VUID-vkGetShaderBinaryDataEXT-None-08499"
        );
    }

    fn chapters_of_the_specification() -> Option<String> {
        let chapters = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../third_party/vulkan-docs/chapters");
        if !chapters.is_dir() {
            assert!(
                std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
                "the valid usage test is required but third_party/vulkan-docs is not checked out"
            );
            eprintln!("skipping the valid usage test: third_party/vulkan-docs is not checked out");
            return None;
        }
        let mut text = String::new();
        let mut directories = vec![chapters];
        while let Some(directory) = directories.pop() {
            let entries = std::fs::read_dir(&directory)
                .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
            for entry in entries {
                let path = entry.expect("a directory entry").path();
                if path.is_dir() {
                    directories.push(path);
                } else if path
                    .extension()
                    .is_some_and(|extension| extension == "adoc")
                {
                    let chapter = std::fs::read_to_string(&path)
                        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
                    text.push_str(&chapter);
                    text.push('\n');
                }
            }
        }
        Some(text)
    }

    fn valid_usage_statement(chapters: &str, vuid: &str) -> String {
        let anchor = format!("[[{vuid}]]");
        assert_eq!(
            chapters.matches(&anchor).count(),
            1,
            "{vuid} is not one valid usage statement of the specification"
        );
        let after = chapters
            .split_once(&anchor)
            .map(|(_, after)| after)
            .unwrap_or_default();
        let end = ["[[VUID-", "****"]
            .iter()
            .filter_map(|next| after.find(next))
            .min()
            .unwrap_or(after.len());
        after[..end]
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn the_listed_valid_usage_statements_of_the_specification_require_the_listed_alignment() {
        let Some(chapters) = chapters_of_the_specification() else {
            return;
        };
        for output in ALIGNED_OUTPUTS {
            let statement = valid_usage_statement(&chapters, output.stated_by);
            assert!(
                statement.contains(&format!("pname:{}", output.parameter))
                    && statement
                        .contains(&format!("must: be aligned to `{}` bytes", output.alignment)),
                "{} does not require `{}` to be aligned to {} bytes: {statement}",
                output.stated_by,
                output.parameter,
                output.alignment
            );
        }
        for slice in ALIGNED_SLICES {
            let alignments = [
                Some(slice.alignment),
                slice.wider.as_ref().map(|wider| wider.alignment),
            ];
            let vuids = slice.stated_by.split(" and ").collect::<Vec<_>>();
            assert_eq!(
                vuids.len(),
                alignments.iter().flatten().count(),
                "{}",
                slice.stated_by
            );
            for (vuid, alignment) in vuids.into_iter().zip(alignments.into_iter().flatten()) {
                let statement = valid_usage_statement(&chapters, vuid);
                assert!(
                    statement.contains(&format!(
                        "pname:{} must: be aligned to a multiple of `{alignment}`",
                        slice.parameter
                    )),
                    "{vuid} does not require `{}` to be aligned to {alignment} bytes: {statement}",
                    slice.parameter
                );
                if let Some(wider) = slice
                    .wider
                    .as_ref()
                    .filter(|wider| wider.alignment == alignment)
                {
                    assert!(
                        statement.contains(&format!(
                            "ename:{} is set in pname:{}",
                            wider.bit, wider.flags
                        )),
                        "{vuid} does not ask for {alignment} bytes when {} is set: {statement}",
                        wider.bit
                    );
                }
            }
        }
        for input in ALIGNED_INPUTS {
            for vuid in input.stated_by.split(" and ") {
                let statement = valid_usage_statement(&chapters, vuid);
                assert!(
                    statement.contains(&format!("pname:{} must: be aligned to `", input.member)),
                    "{vuid} does not require `{}` to be aligned: {statement}",
                    input.member
                );
            }
        }
    }

    const ALIGNED_RAW_POINTERS: &[&str] =
        &["VUID-vkGetMemoryHostPointerPropertiesEXT-pHostPointer-01753"];

    #[test]
    fn every_command_parameter_that_the_specification_requires_to_be_aligned_is_listed() {
        let Some(chapters) = chapters_of_the_specification() else {
            return;
        };
        let listed = ALIGNED_OUTPUTS
            .iter()
            .map(|output| output.stated_by)
            .chain(
                ALIGNED_SLICES
                    .iter()
                    .flat_map(|slice| [slice.stated_by, slice.unchecked]),
            )
            .chain(ALIGNED_RAW_POINTERS.iter().copied())
            .collect::<Vec<_>>()
            .join(" ");
        let mut found = 0;
        for (index, _) in chapters.match_indices("[[VUID-vk") {
            let vuid = &chapters[index + 2..];
            let vuid = &vuid[..vuid.find("]]").expect("the end of the anchor")];
            let statement = valid_usage_statement(&chapters, vuid);
            let aligned = statement.split("pname:").skip(1).any(|after| {
                let (name, rest) = after.split_once(' ').unwrap_or((after, ""));
                name.starts_with('p')
                    && name.chars().all(|c| c.is_ascii_alphanumeric())
                    && (rest.starts_with("must: be aligned")
                        || rest.starts_with("must: be a pointer aligned"))
            });
            if aligned {
                found += 1;
                assert!(
                    listed.contains(vuid),
                    "{vuid} requires a parameter to be aligned, and no list of aligned memory names it: {statement}"
                );
            }
        }
        assert!(
            found >= 4,
            "only {found} statements require a parameter to be aligned"
        );
    }

    #[test]
    fn a_listed_slice_is_checked_before_the_call() {
        let model = Model::with_docs(Registry::get(), vulkan_registry::Docs::get());
        let out = squash(&commands::emit(&model));
        assert!(
            out.contains(
                "unsafe { let data_size_len: usize = data.len(); assert!(data.as_ptr().addr().is_multiple_of(if flags.contains(QueryResultFlags::_64) { 8 } else { 4 }), \"vkGetQueryPoolResults: `data` has to start at an address aligned to 4 bytes, or to 8 bytes if `flags` contains `VK_QUERY_RESULT_64_BIT` (VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815)\"); let result = self.get_query_pool_results_raw("
            ),
            "{}",
            &out[out.find("pub unsafe fn get_query_pool_results(").unwrap()..][..1200]
        );
        let checked = model
            .ir
            .commands
            .iter()
            .filter(|c| model.aligned_slice(c).is_some())
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(checked, ["vkGetQueryPoolResults"]);
    }

    fn query() -> Command {
        let mut c = result_command(
            "vkGetQueryPoolResults",
            &["VK_SUCCESS", "VK_NOT_READY"],
            vec![
                field("device", "VkDevice"),
                field("dataSize", "size_t"),
                counted("pData", "void*", "dataSize"),
                field("flags", "VkQueryResultFlags"),
            ],
        );
        c.device = true;
        c
    }

    #[test]
    fn a_listed_slice_that_the_registry_changes_stops_the_generator() {
        const CORRECT: &str =
            "correct the list of memory that the specification requires to be aligned";
        let mut renamed = query();
        renamed.params[2].name = "pResults".to_string();
        let mut typed = query();
        typed.params[2] = counted("pData", "uint64_t*", "dataSize");
        let mut uncounted = query();
        uncounted.params[2] = field("pData", "void*");
        let mut without_flags = query();
        without_flags.params.pop();
        for (c, message) in [
            (
                renamed,
                format!(
                    "vkGetQueryPoolResults has no parameter `pData`: {CORRECT}, which takes this one from VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815"
                ),
            ),
            (
                typed,
                format!(
                    "vkGetQueryPoolResults::pData is a `uint64_t*`, not a pointer to bytes of a stated length that the caller passes: {CORRECT}, which takes this one from VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815"
                ),
            ),
            (
                uncounted,
                format!(
                    "vkGetQueryPoolResults::pData is a `void*`, not a pointer to bytes of a stated length that the caller passes: {CORRECT}, which takes this one from VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815"
                ),
            ),
            (
                without_flags,
                format!(
                    "vkGetQueryPoolResults has no parameter `flags` that holds flags: {CORRECT}, which takes `pData` from VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815"
                ),
            ),
        ] {
            let ir = registry_with(vec![c]);
            assert_eq!(
                catch_quietly(|| Model::new(&ir).aligned_slice(&ir.commands[0]).is_some()),
                Err(message)
            );
        }
        let bits = |name: &str, bit: &str| vulkan_registry::Bitmask {
            name: name.to_string(),
            flags_name: name.replace("FlagBits", "Flags"),
            bit_width: 32,
            fields: vec![EnumField {
                name: bit.to_string(),
                value: "1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        for bitmasks in [
            vec![],
            vec![bits("VkQueryResultFlagBits", "VK_QUERY_RESULT_WAIT_BIT")],
            vec![
                bits("VkQueryResultFlagBits", "VK_QUERY_RESULT_WAIT_BIT"),
                bits("VkOtherFlagBits", "VK_QUERY_RESULT_64_BIT"),
            ],
        ] {
            let mut ir = registry_with(vec![query()]);
            ir.bitmasks = bitmasks;
            assert_eq!(
                catch_quietly(|| Model::new(&ir).aligned_slice(&ir.commands[0]).is_some()),
                Err(format!(
                    "VkQueryResultFlagBits has no bit VK_QUERY_RESULT_64_BIT: {CORRECT}, which takes `pData` of vkGetQueryPoolResults from VUID-vkGetQueryPoolResults-flags-02828 and VUID-vkGetQueryPoolResults-flags-00815"
                ))
            );
        }
        let mut ir = registry_with(vec![query()]);
        ir.bitmasks = vec![bits("VkQueryResultFlagBits", "VK_QUERY_RESULT_64_BIT")];
        assert_eq!(
            catch_quietly(|| Model::new(&ir).aligned_slice(&ir.commands[0]).is_some()),
            Ok(true)
        );
    }

    #[test]
    fn the_wrappers_of_the_registry_that_allocate_bytes_align_them_where_listed() {
        let model = Model::with_docs(Registry::get(), vulkan_registry::Docs::get());
        let out = squash(&commands::emit(&model));
        let mut allocating = BTreeSet::new();
        let mut aligning = BTreeSet::new();
        for c in &model.ir.commands {
            let wrapper = format!(
                "pub unsafe fn {}(&self",
                crate::naming::command_name(&c.name)
            );
            let Some(start) = out.find(&wrapper) else {
                continue;
            };
            let body = &out[start..];
            let body = &body[..body[1..]
                .find("pub unsafe fn ")
                .map_or(body.len(), |end| end + 1)];
            if body.contains("let mut values: Vec<u8> = alloc::vec![") {
                allocating.insert(c.name.as_str());
            }
            if body.contains("padding") {
                aligning.insert(c.name.as_str());
            }
            assert_eq!(
                body.contains(".add(padding)"),
                !model.aligned_outputs(c).is_empty(),
                "{}: {body}",
                c.name
            );
        }
        assert_eq!(aligning, BTreeSet::from([LISTED]));
        assert_eq!(
            allocating,
            BTreeSet::from([
                "vkGetCudaModuleCacheNV",
                "vkGetEncodedVideoSessionParametersKHR",
                "vkGetGpaSessionResultsAMD",
                "vkGetPipelineBinaryDataKHR",
                "vkGetPipelineCacheData",
                "vkGetShaderBinaryDataEXT",
                "vkGetShaderInfoAMD",
                "vkGetValidationCacheDataEXT",
            ]),
            "the wrappers that allocate the bytes a command writes: the valid usage of each of \
             these commands was read for an alignment of the buffer, and the commands that \
             require one are listed in ALIGNED_OUTPUTS"
        );
    }

    #[test]
    fn the_setter_of_a_listed_member_documents_the_alignment() {
        let model = Model::new(Registry::get());
        let out = structs::emit(&model);
        let requirement = ALIGNED_INPUTS[0].requirement;
        let flat = out
            .lines()
            .map(|line| line.trim().trim_start_matches("///").trim())
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            flat.matches(requirement).count(),
            ALIGNED_INPUTS.len(),
            "the requirement is documented once, on the setter of the listed member"
        );
        assert!(
            flat.contains(&format!(
                "Sets [`p_code`](Self::p_code) to the start of a slice of bytes and [`code_size`](Self::code_size) to its length.  {requirement} pub fn code(mut self, value: &'a [u8]) -> Self {{"
            )),
            "the requirement follows the first paragraph of the documentation of ShaderCreateInfoEXT::code"
        );
    }
}
