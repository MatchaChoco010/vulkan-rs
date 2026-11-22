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
