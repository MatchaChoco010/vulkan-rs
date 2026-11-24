use crate::model::CType;
use crate::naming::pointer_setter_name;
use vulkan_registry::{Command, Registry};

#[derive(Debug)]
pub(super) struct UncountedArray {
    pub(super) command: &'static str,
    pub(super) parameter: &'static str,
    pub(super) count: &'static str,
    pub(super) element: &'static str,
    pub(super) stated_by: &'static str,
}

pub(super) const UNCOUNTED_ARRAYS: &[UncountedArray] = &[UncountedArray {
    command: "vkGetShaderInstrumentationValuesARM",
    parameter: "pMetricValues",
    count: "pMetricBlockCount",
    element: "a metric block, a [`ShaderInstrumentationMetricDataHeaderARM`] followed by as many `u64` values as [`PhysicalDeviceShaderInstrumentationPropertiesARM::num_metrics`] says",
    stated_by: "the description of vkGetShaderInstrumentationValuesARM",
}];

const CORRECT_THE_LIST: &str =
    "correct the list of command parameters that the registry does not mark as counted arrays";

impl UncountedArray {
    fn assert_matches(&self, c: &Command) {
        let Self {
            command,
            parameter,
            count,
            stated_by,
            ..
        } = self;
        let array = c.params.iter().find(|p| p.name == *parameter);
        let array = array.unwrap_or_else(|| {
            panic!("{command} has no parameter `{parameter}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}")
        });
        assert!(
            CType::of(array).pointers == [false]
                && array.ty == "void"
                && array.len.is_none()
                && array.optional,
            "{command}::{parameter} is {} `{}` with the length {:?}, not an optional `void*` without a length: {CORRECT_THE_LIST}, which takes this one from {stated_by}",
            if array.optional {
                "an optional"
            } else {
                "a required"
            },
            array.full_ty,
            array.len
        );
        let counted = c.params.iter().find(|p| p.name == *count).is_some_and(|p| {
            CType::of(p).pointers == [false] && matches!(p.ty.as_str(), "uint32_t" | "size_t")
        });
        assert!(
            counted,
            "{command} has no parameter `{count}` that points to the number of elements of `{parameter}`: {CORRECT_THE_LIST}, which takes this one from {stated_by}"
        );
    }

    pub(super) fn note(&self) -> String {
        let count = pointer_setter_name(self.count);
        let array = pointer_setter_name(self.parameter);
        format!(
            "`{count}` is the element count of `{array}`, which the command reads and writes: if `{array}` is null, the command writes the number of elements that it can return to it; otherwise it has to hold the number of elements that `{array}` has room for when the wrapper is called, and the command overwrites it with the number of elements that it wrote. Each element is {}, so the size of an element depends on the device. The wrapper passes `{array}` as it is, without checking it: unless it is null, it has to be valid for writes of as many elements as `{count}` holds when the wrapper is called.",
            self.element
        )
    }
}

pub(super) fn uncounted_array(c: &Command) -> Option<&'static UncountedArray> {
    let array = UNCOUNTED_ARRAYS
        .iter()
        .find(|array| array.command == c.name)?;
    array.assert_matches(c);
    Some(array)
}

pub(crate) fn assert_listed_uncounted_arrays_exist(ir: &Registry) {
    for array in UNCOUNTED_ARRAYS {
        let c = ir
            .commands
            .iter()
            .find(|c| c.name == array.command)
            .unwrap_or_else(|| {
                panic!(
                    "the registry has no command {}: {CORRECT_THE_LIST}, which takes `{}` from {}",
                    array.command, array.parameter, array.stated_by
                )
            });
        array.assert_matches(c);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{catch_quietly, counted, field, optional, registry, result_command};

    fn points_to_a_count(c: &Command, p: &vulkan_registry::Field) -> bool {
        CType::of(p).pointers == [false]
            && matches!(p.ty.as_str(), "uint32_t" | "size_t")
            && !c
                .params
                .iter()
                .any(|q| q.len.as_deref() == Some(p.name.as_str()))
    }

    #[test]
    fn every_command_with_a_count_and_untyped_memory_without_a_length_is_listed() {
        let ir = Registry::get();
        assert_listed_uncounted_arrays_exist(ir);
        let mut found = Vec::new();
        for c in &ir.commands {
            let counts = c.params.iter().any(|p| points_to_a_count(c, p));
            let uncounted = c
                .params
                .iter()
                .any(|p| CType::of(p).pointers == [false] && p.ty == "void" && p.len.is_none());
            if counts && uncounted {
                found.push(c.name.as_str());
                assert!(
                    uncounted_array(c).is_some(),
                    "{}: a pointer to a count and a `void*` without a length: find in the specification whether the count counts its elements",
                    c.name
                );
            }
        }
        assert_eq!(found, ["vkGetShaderInstrumentationValuesARM"]);
    }

    fn listed() -> Command {
        result_command(
            "vkGetShaderInstrumentationValuesARM",
            &["VK_SUCCESS", "VK_INCOMPLETE"],
            vec![
                field("device", "VkDevice"),
                field("pMetricBlockCount", "uint32_t*"),
                optional(field("pMetricValues", "void*")),
            ],
        )
    }

    #[test]
    fn a_listed_array_that_the_registry_changes_stops_the_generator() {
        const CORRECT: &str = "correct the list of command parameters that the registry does not mark as counted arrays, which takes this one from the description of vkGetShaderInstrumentationValuesARM";
        let mut renamed = listed();
        renamed.params[2].name = "pValues".to_string();
        let mut counted_values = listed();
        counted_values.params[2] = counted("pMetricValues", "void*", "pMetricBlockCount");
        let mut typed = listed();
        typed.params[2] = field("pMetricValues", "uint64_t*");
        let mut required = listed();
        required.params[2] = field("pMetricValues", "void*");
        let mut without_count = listed();
        without_count.params[1] = field("pMetricBlockCount", "uint32_t");
        for (c, message) in [
            (
                renamed,
                format!(
                    "vkGetShaderInstrumentationValuesARM has no parameter `pMetricValues`: {CORRECT}"
                ),
            ),
            (
                counted_values,
                format!(
                    "vkGetShaderInstrumentationValuesARM::pMetricValues is a required `void*` with the length Some(\"pMetricBlockCount\"), not an optional `void*` without a length: {CORRECT}"
                ),
            ),
            (
                typed,
                format!(
                    "vkGetShaderInstrumentationValuesARM::pMetricValues is a required `uint64_t*` with the length None, not an optional `void*` without a length: {CORRECT}"
                ),
            ),
            (
                required,
                format!(
                    "vkGetShaderInstrumentationValuesARM::pMetricValues is a required `void*` with the length None, not an optional `void*` without a length: {CORRECT}"
                ),
            ),
            (
                without_count,
                format!(
                    "vkGetShaderInstrumentationValuesARM has no parameter `pMetricBlockCount` that points to the number of elements of `pMetricValues`: {CORRECT}"
                ),
            ),
        ] {
            assert_eq!(
                catch_quietly(|| uncounted_array(&c).is_some()),
                Err(message)
            );
        }
        assert_eq!(
            catch_quietly(|| uncounted_array(&listed()).is_some()),
            Ok(true)
        );
        let message = catch_quietly(|| assert_listed_uncounted_arrays_exist(&registry()))
            .expect_err("the registry has no listed command");
        assert!(
            message
                .starts_with("the registry has no command vkGetShaderInstrumentationValuesARM: "),
            "{message}"
        );
    }
}
