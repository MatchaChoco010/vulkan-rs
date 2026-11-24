use crate::model::{CType, Model, is_opaque};
use vulkan_registry::{Command, Registry};

#[derive(Debug)]
pub(super) struct ReadOutput {
    pub(super) command: &'static str,
    pub(super) parameter: &'static str,
    pub(super) contract: &'static str,
    pub(super) stated_by: &'static str,
}

pub(super) const READ_OUTPUTS: &[ReadOutput] = &[ReadOutput {
    command: "vkRegisterCustomBorderColorEXT",
    parameter: "pIndex",
    contract: "if `request_index` is true, it is the index that the command tries to register, and the wrapper returns that same index; otherwise the command ignores it and the wrapper returns the new index",
    stated_by: "the description of vkRegisterCustomBorderColorEXT",
}];

const CORRECT_THE_LIST: &str =
    "correct the list of values that a command reads before it writes them";

impl ReadOutput {
    fn assert_matches(&self, model: &Model<'_>, c: &Command) {
        let Self {
            command,
            parameter,
            stated_by,
            ..
        } = self;
        let last = c.params.last().filter(|p| p.name == *parameter);
        let p = last.unwrap_or_else(|| {
            panic!(
                "the last parameter of {command} is not `{parameter}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}"
            )
        });
        let ty = CType::of(p);
        assert!(
            ty.pointers == [false]
                && !ty.is_array()
                && p.len.is_none()
                && !is_opaque(&ty.base)
                && model.get_struct(&ty.base).is_none()
                && model.canonical(&p.ty) != "VkBool32",
            "{command}::{parameter} is a `{}` {}, not a pointer to one value that the wrapper can take and return: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            p.full_ty,
            match p.len.as_deref() {
                Some(len) => format!("with the length `{len}`"),
                None => "without a length".to_string(),
            }
        );
    }

    pub(super) fn note(&self, argument: &str) -> String {
        format!(
            "`{argument}` is the value that `{}` points to when the command is called: {}.",
            self.parameter, self.contract
        )
    }
}

pub(super) fn read_output(model: &Model<'_>, c: &Command) -> Option<&'static ReadOutput> {
    let command = model.command_behind(&c.name);
    let listed = READ_OUTPUTS
        .iter()
        .find(|listed| model.command_behind(listed.command) == command)?;
    listed.assert_matches(model, c);
    Some(listed)
}

pub(crate) fn assert_listed_read_outputs_exist(ir: &Registry) {
    let model = Model::new(ir);
    for listed in READ_OUTPUTS {
        let c = ir
            .commands
            .iter()
            .find(|c| c.name == listed.command)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no command {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    listed.command, listed.parameter, listed.stated_by
                )
            });
        listed.assert_matches(&model, c);
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::Shape;
    use super::super::{commands, creates_a_dispatching_handle, wrapper};
    use super::*;
    use crate::emit::{doc_of, squash};
    use crate::model::Dispatch;
    use crate::test_support::{
        catch_quietly, chapters_of_the_specification, counted, description_in, field, registry,
        result_command,
    };

    const WITHOUT_A_DESCRIPTION: &[&str] = &[
        "vkCreateCuFunctionNVX",
        "vkCreateCuModuleNVX",
        "vkGetDescriptorSetHostMappingVALVE",
        "vkGetSleepStatusLegacyNV",
    ];

    fn reads_before_it_writes<'t>(description: &'t str, parameter: &str) -> Option<&'t str> {
        [
            format!("value present in pname:{parameter} "),
            format!("value stored in pname:{parameter} "),
            format!("pname:{parameter} must: point to a variable set by "),
        ]
        .iter()
        .find_map(|statement| {
            description
                .find(statement.as_str())
                .map(|at| &description[at..at + statement.len()])
        })
    }

    #[test]
    fn the_scan_for_values_that_a_command_reads_knows_the_ways_the_specification_says_it() {
        for (text, expected) in [
            (
                "the value present in pname:pIndex when passed to the command is a requested index",
                Some("value present in pname:pIndex "),
            ),
            (
                "the value stored in pname:pIndex is ignored",
                Some("value stored in pname:pIndex "),
            ),
            (
                "Otherwise, pname:pIndex must: point to a variable set by the application",
                Some("pname:pIndex must: point to a variable set by "),
            ),
            ("pname:pIndex is a pointer to an index value", None),
            ("the value present in pname:pIndexes when passed", None),
            ("the value stored in pname:pOther is ignored", None),
        ] {
            assert_eq!(reads_before_it_writes(text, "pIndex"), expected, "{text}");
        }
    }

    #[test]
    fn every_value_that_a_command_reads_before_it_writes_it_is_listed() {
        let ir = Registry::get();
        assert_listed_read_outputs_exist(ir);
        let Some(chapters) = chapters_of_the_specification() else {
            return;
        };
        let model = Model::new(ir);
        let mut found = Vec::new();
        let mut undescribed = Vec::new();
        let mut returned = 0;
        for dispatch in Dispatch::ALL {
            for c in commands(&model, dispatch).filter(|c| !creates_a_dispatching_handle(c)) {
                let params = dispatch.explicit_params(c);
                let Shape::ReturnsValue { out } = Shape::detect(&model, c, params) else {
                    continue;
                };
                returned += 1;
                let description = description_in(&chapters, model.command_behind(&c.name))
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                if description.is_empty() {
                    undescribed.push(c.name.as_str());
                }
                let statement = reads_before_it_writes(&description, &params[out].name);
                let listed = read_output(&model, c).is_some();
                assert_eq!(
                    statement.is_some(),
                    listed,
                    "{}: the wrapper returns what the command writes to `{}`, the specification says {statement:?} about it, and the list of values that a command reads before it writes them {} it",
                    c.name,
                    params[out].name,
                    if listed { "names" } else { "does not name" }
                );
                if listed {
                    found.push(c.name.as_str());
                }
            }
        }
        assert_eq!(
            found,
            READ_OUTPUTS
                .iter()
                .map(|listed| listed.command)
                .collect::<Vec<_>>()
        );
        assert!(returned >= 130, "only {returned} wrappers return a value");
        assert_eq!(
            undescribed, WITHOUT_A_DESCRIPTION,
            "the commands whose wrapper returns such a value and that the specification does not describe"
        );
    }

    fn listed() -> Command {
        let mut c = result_command(
            "vkRegisterCustomBorderColorEXT",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("requestIndex", "VkBool32"),
                field("pIndex", "uint32_t*"),
            ],
        );
        c.device = true;
        c
    }

    #[test]
    fn the_wrapper_takes_the_value_that_the_command_reads_and_returns_what_it_writes() {
        let mut ir = registry();
        ir.commands.push(listed());
        let model = Model::new(&ir);
        let out = wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device);
        assert!(
            squash(&out).contains(
                "pub unsafe fn register_custom_border_color_ext(&self, request_index: bool, index: u32) -> crate::vk::Result<u32> { unsafe { let mut value: u32 = index; self.register_custom_border_color_ext_raw(request_index as Bool32, &mut value).into_result()?; Ok(value) } }"
            ),
            "{out}"
        );
        let documented = doc_of(&out, "pub unsafe fn register_custom_border_color_ext(").join(" ");
        assert!(
            documented.contains(
                "Returns what the command writes to `pIndex`. `index` is the value that `pIndex` points to when the command is called: if `request_index` is true, it is the index that the command tries to register, and the wrapper returns that same index; otherwise the command ignores it and the wrapper returns the new index."
            ),
            "{documented}"
        );
        let model = Model::new(Registry::get());
        let c = Registry::get()
            .commands
            .iter()
            .find(|c| c.name == "vkRegisterCustomBorderColorEXT")
            .expect("the listed command");
        let out = squash(&wrapper::wrapper(&model, c, Dispatch::Device));
        assert!(
            out.contains(
                "pub unsafe fn register_custom_border_color_ext(&self, border_color: &SamplerCustomBorderColorCreateInfoEXT<'_>, request_index: bool, index: u32) -> crate::vk::Result<u32> { unsafe { let mut value: u32 = index; "
            ),
            "{out}"
        );
    }

    #[test]
    fn a_listed_value_that_the_registry_changes_stops_the_generator() {
        const CORRECT: &str = "correct the list of values that a command reads before it writes them, which takes this one from the description of vkRegisterCustomBorderColorEXT";
        let mut renamed = listed();
        renamed.params[2].name = "pRegistered".to_string();
        let mut not_last = listed();
        not_last.params.push(field("flags", "uint32_t"));
        let mut read_only = listed();
        read_only.params[2] = field("pIndex", "const uint32_t*");
        let mut array = listed();
        array.params[2] = counted("pIndex", "uint32_t*", "requestIndex");
        let mut value = listed();
        value.params[2] = field("pIndex", "uint32_t");
        let mut untyped = listed();
        untyped.params[2] = field("pIndex", "void*");
        let mut boolean = listed();
        boolean.params[2] = field("pIndex", "VkBool32*");
        let mut structure = listed();
        structure.params[2] = field("pIndex", "VkIndexNV*");
        for (c, message) in [
            (
                renamed,
                format!(
                    "the last parameter of vkRegisterCustomBorderColorEXT is not `pIndex`: {CORRECT}"
                ),
            ),
            (
                not_last,
                format!(
                    "the last parameter of vkRegisterCustomBorderColorEXT is not `pIndex`: {CORRECT}"
                ),
            ),
            (
                read_only,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `const uint32_t*` without a length, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
            (
                array,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `uint32_t*` with the length `requestIndex`, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
            (
                value,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `uint32_t` without a length, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
            (
                untyped,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `void*` without a length, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
            (
                boolean,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `VkBool32*` without a length, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
            (
                structure,
                format!(
                    "vkRegisterCustomBorderColorEXT::pIndex is a `VkIndexNV*` without a length, not a pointer to one value that the wrapper can take and return: {CORRECT}"
                ),
            ),
        ] {
            let mut ir = registry();
            ir.structs.push(crate::test_support::structure(
                "VkIndexNV",
                vec![field("index", "uint32_t")],
            ));
            ir.commands.push(c);
            assert_eq!(
                catch_quietly(|| assert_listed_read_outputs_exist(&ir)),
                Err(message)
            );
        }
        let message = catch_quietly(|| assert_listed_read_outputs_exist(&registry()))
            .expect_err("the registry has no listed command");
        assert!(
            message.starts_with("the registry has no command vkRegisterCustomBorderColorEXT: "),
            "{message}"
        );
    }

    #[test]
    fn a_listed_value_that_the_wrapper_does_not_return_stops_the_generator() {
        let mut c = listed();
        c.return_ty = "VkBool32".to_string();
        c.success_codes.clear();
        let mut ir = registry();
        ir.commands.push(c);
        let model = Model::new(&ir);
        assert_eq!(
            catch_quietly(|| wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device)),
            Err("vkRegisterCustomBorderColorEXT: the list of values that a command reads before it writes them names `pIndex`, and the wrapper does not return what the command writes to it".to_string())
        );
    }
}
