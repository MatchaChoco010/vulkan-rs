use super::CType;
use vulkan_registry::Field;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum VoidArrayLength {
    Bytes,
    Values(&'static TypedValues),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TypedValues {
    pub(crate) type_member: &'static str,
    pub(crate) type_enum: &'static str,
    pub(crate) value_types: &'static [(&'static str, &'static str)],
}

const LAYER_SETTING_VALUES: TypedValues = TypedValues {
    type_member: "type",
    type_enum: "VkLayerSettingTypeEXT",
    value_types: &[
        ("VK_LAYER_SETTING_TYPE_BOOL32_EXT", "[`Bool32`]"),
        ("VK_LAYER_SETTING_TYPE_INT32_EXT", "`i32`"),
        ("VK_LAYER_SETTING_TYPE_INT64_EXT", "`i64`"),
        ("VK_LAYER_SETTING_TYPE_UINT32_EXT", "`u32`"),
        ("VK_LAYER_SETTING_TYPE_UINT64_EXT", "`u64`"),
        ("VK_LAYER_SETTING_TYPE_FLOAT32_EXT", "`f32`"),
        ("VK_LAYER_SETTING_TYPE_FLOAT64_EXT", "`f64`"),
        (
            "VK_LAYER_SETTING_TYPE_STRING_EXT",
            "[`CStrPtr`], or `*const c_char`, a pointer to a NUL-terminated string",
        ),
    ],
};

const COUNTED_IN_VALUES: &[(&str, &str, &str, &TypedValues)] = &[(
    "VkLayerSettingEXT",
    "pValues",
    "valueCount",
    &LAYER_SETTING_VALUES,
)];

fn is_named_as_a_size_in_bytes(len: &str) -> bool {
    len == "size" || len.ends_with("Size") || len.ends_with("SizeInBytes")
}

pub(crate) fn void_array_len(array: &Field) -> Option<&str> {
    let ty = CType::of(array);
    let is_void_array = ty.base == "void" && ty.depth() == 1 && !ty.is_array();
    array.len.as_deref().filter(|_| is_void_array)
}

pub(crate) fn void_array_length(owner: &str, array: &Field) -> VoidArrayLength {
    let len = void_array_len(array)
        .unwrap_or_else(|| panic!("{owner}::{} is not a `void` array", array.name));
    let listed = COUNTED_IN_VALUES
        .iter()
        .find(|row| (row.0, row.1, row.2) == (owner, array.name.as_str(), len));
    match listed {
        Some(row) => VoidArrayLength::Values(row.3),
        None if is_named_as_a_size_in_bytes(len) => VoidArrayLength::Bytes,
        None => panic!(
            "{owner}::{}: the registry gives `{len}` as the length of this `void` array, which is not the name of a size in bytes; add the array to COUNTED_IN_VALUES if the specification counts it in values",
            array.name
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{catch_quietly, counted, field};
    use std::collections::BTreeSet;
    use vulkan_registry::Registry;

    #[test]
    fn a_void_array_whose_length_is_named_as_a_size_is_counted_in_bytes() {
        for len in [
            "dataSize",
            "pDataSize",
            "initialDataSize",
            "tagSize",
            "size",
            "pSizeInBytes",
        ] {
            assert_eq!(
                void_array_length("VkBlobInfo", &counted("pData", "const void*", len)),
                VoidArrayLength::Bytes,
                "{len}"
            );
        }
    }

    #[test]
    fn a_void_array_with_any_other_length_stops_the_generator_until_it_is_listed() {
        for len in [
            "valueCount",
            "count",
            "dataSizes",
            "sizeInWords",
            "resize",
            "Size / 4",
        ] {
            let message = catch_quietly(|| {
                void_array_length("VkBlobInfo", &counted("pData", "const void*", len))
            })
            .expect_err(len);
            assert!(
                message.contains(&format!(
                    "VkBlobInfo::pData: the registry gives `{len}` as the length of this `void` array"
                )),
                "{len}: {message}"
            );
        }
    }

    #[test]
    fn a_listed_array_is_counted_in_values_only_under_its_own_structure_and_length() {
        let values = counted("pValues", "const void*", "valueCount");
        assert!(matches!(
            void_array_length("VkLayerSettingEXT", &values),
            VoidArrayLength::Values(typed) if typed.type_enum == "VkLayerSettingTypeEXT"
        ));
        assert!(catch_quietly(|| void_array_length("VkOtherSettingEXT", &values)).is_err());
        assert_eq!(
            void_array_length(
                "VkLayerSettingEXT",
                &counted("pValues", "const void*", "valuesSize")
            ),
            VoidArrayLength::Bytes
        );
    }

    #[test]
    fn only_single_pointers_to_void_with_a_length_are_void_arrays() {
        assert_eq!(
            void_array_len(&counted("pData", "const void*", "dataSize")),
            Some("dataSize")
        );
        assert_eq!(
            void_array_len(&counted("pData", "void*", "pDataSize")),
            Some("pDataSize")
        );
        assert_eq!(void_array_len(&field("pNext", "const void*")), None);
        assert_eq!(
            void_array_len(&counted("pParams", "const void* const *", "paramCount")),
            None
        );
        assert_eq!(
            void_array_len(&counted("pData", "const uint8_t*", "dataSize")),
            None
        );
    }

    fn void_arrays_of_the_registry() -> Vec<(&'static str, &'static Field)> {
        let ir = Registry::get();
        let members = ir
            .structs
            .iter()
            .flat_map(|s| s.members.iter().map(|m| (s.name.as_str(), m)));
        let params = ir
            .commands
            .iter()
            .flat_map(|c| c.params.iter().map(|p| (c.name.as_str(), p)));
        members
            .chain(params)
            .filter(|(_, field)| void_array_len(field).is_some())
            .collect()
    }

    #[test]
    fn the_void_arrays_of_the_registry_whose_length_does_not_end_in_size_are_the_known_ones() {
        let arrays = void_arrays_of_the_registry();
        assert!(arrays.len() >= 39, "{}", arrays.len());
        let not_ending_in_size = arrays
            .iter()
            .filter_map(|(owner, array)| {
                let len = void_array_len(array)?;
                let length = void_array_length(owner, array);
                (!len.ends_with("Size")).then(|| {
                    let unit = match length {
                        VoidArrayLength::Bytes => "bytes",
                        VoidArrayLength::Values(_) => "values",
                    };
                    (*owner, array.name.as_str(), len, unit)
                })
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(
            not_ending_in_size,
            BTreeSet::from([
                ("VkHostAddressRangeConstEXT", "address", "size", "bytes"),
                ("VkHostAddressRangeEXT", "address", "size", "bytes"),
                ("VkLayerSettingEXT", "pValues", "valueCount", "values"),
                ("VkPushConstantsInfo", "pValues", "size", "bytes"),
                ("vkCmdPushConstants", "pValues", "size", "bytes"),
                (
                    "vkGetGpaSessionResultsAMD",
                    "pData",
                    "pSizeInBytes",
                    "bytes"
                ),
            ]),
            "check in the specification what the length of a new array counts"
        );
    }

    #[test]
    fn every_listed_array_is_a_void_array_of_the_registry() {
        let arrays = void_arrays_of_the_registry()
            .into_iter()
            .filter_map(|(owner, array)| Some((owner, array.name.as_str(), void_array_len(array)?)))
            .collect::<BTreeSet<_>>();
        for row in COUNTED_IN_VALUES {
            assert!(arrays.contains(&(row.0, row.1, row.2)), "{row:?}");
        }
    }

    #[test]
    fn every_value_type_of_the_typed_arrays_has_a_rust_type() {
        let ir = Registry::get();
        for row in COUNTED_IN_VALUES {
            let typed = row.3;
            let owner = ir
                .structs
                .iter()
                .find(|s| s.name == row.0)
                .unwrap_or_else(|| panic!("{} is not a structure", row.0));
            let type_member = owner
                .members
                .iter()
                .find(|m| m.name == typed.type_member)
                .unwrap_or_else(|| panic!("{} has no member {}", row.0, typed.type_member));
            assert_eq!(type_member.ty, typed.type_enum, "{}", row.0);
            let values = ir
                .enums
                .iter()
                .find(|e| e.name == typed.type_enum)
                .unwrap_or_else(|| panic!("{} is not an enumeration", typed.type_enum))
                .fields
                .iter()
                .map(|f| f.name.as_str())
                .collect::<Vec<_>>();
            let listed = typed
                .value_types
                .iter()
                .map(|(value, _)| *value)
                .collect::<Vec<_>>();
            assert_eq!(listed, values, "{}", typed.type_enum);
        }
    }
}
