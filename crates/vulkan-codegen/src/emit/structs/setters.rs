use super::{MethodNames, field_link, member_origin};
use crate::docs::{Block, Doc};
use crate::emit::Borrow;
use crate::model::{CType, Model, TypedValues, VoidArrayLength, is_opaque, null_ptr};
use crate::model::{void_array_len, void_array_length};
use crate::naming::{field_name, pointer_setter_name, type_name};
use std::collections::BTreeMap;
use vulkan_registry::{Field, Struct};

const SPIRV_WORD_COUNT: &str = "codeSize / 4";

pub(super) struct CountedArrays<'s> {
    s: &'s Struct,
    arrays_of_count: BTreeMap<&'s str, Vec<&'s Field>>,
}

impl<'s> CountedArrays<'s> {
    pub(super) fn new(s: &'s Struct) -> Self {
        let mut arrays_of_count = BTreeMap::<&str, Vec<&Field>>::new();
        for m in &s.members {
            let ty = CType::of(m);
            if ty.depth() == 0 || ty.is_array() {
                continue;
            }
            if let Some(count) = count_member(s, m) {
                arrays_of_count.entry(&count.name).or_default().push(m);
            }
        }
        Self { s, arrays_of_count }
    }

    fn shares_its_count(&self, count: &Field, m: &Field) -> bool {
        self.arrays_of_count[count.name.as_str()]
            .iter()
            .any(|other| other.name != m.name)
    }

    fn slice_doc(
        &self,
        model: &Model<'_>,
        count: &Field,
        m: &Field,
        of: &str,
        element_types: Vec<Block>,
    ) -> Doc {
        let doc = Doc::new()
            .text(format!(
                "Sets {} to the start of a slice of {of} and {} to its length.",
                field_link(&m.name),
                field_link(&count.name)
            ))
            .blocks(element_types);
        let mut panics = Vec::new();
        if self.shares_its_count(count, m) {
            panics.push(format!(
                "Panics if another array that shares `{}` is already set and the slice has a different length.",
                count.name
            ));
        }
        if !model.holds_every_slice_len(&count.ty) {
            panics.push(format!(
                "Panics if the slice has more elements than `{}`, a `{}`, can count.",
                count.name, count.ty
            ));
        }
        if panics.is_empty() {
            return doc;
        }
        doc.section("Panics", panics.join(" "))
    }

    fn set_count(&self, model: &Model<'_>, count: &Field, m: &Field) -> String {
        let field = field_name(&count.name);
        if model.holds_every_slice_len(&count.ty) {
            return format!("self.{field} = value.len() as _;");
        }
        format!(
            "self.{field} = len_as_count(value.len(), \"{}::{}\", \"{}\");",
            self.s.name, m.name, count.name
        )
    }

    fn assert_same_length_as_the_others(&self, count: &Field, m: &Field) -> String {
        let others_are_set = self.arrays_of_count[count.name.as_str()]
            .iter()
            .filter(|other| other.name != m.name)
            .map(|other| format!("!self.{}.is_null()", field_name(&other.name)))
            .collect::<Vec<_>>();
        if others_are_set.is_empty() {
            return String::new();
        }
        format!(
            "if {} {{ assert_eq!(self.{} as usize, value.len(), \"{}::{}: every array sharing `{}` must have the same length\"); }} ",
            others_are_set.join(" || "),
            field_name(&count.name),
            self.s.name,
            m.name,
            count.name
        )
    }
}

fn count_member<'s>(s: &'s Struct, m: &Field) -> Option<&'s Field> {
    let len = m.len.as_deref()?;
    let count = match len.split_once(',') {
        Some((count, "1")) => count,
        Some(_) => return None,
        None => len,
    };
    s.members
        .iter()
        .find(|c| c.name == count && CType::of(c).depth() == 0 && !CType::of(c).is_array())
}

struct Setter {
    doc: Doc,
    name: String,
    generics: &'static str,
    value_ty: String,
    assignments: String,
}

impl Setter {
    fn render(&self) -> String {
        let Setter {
            doc,
            name,
            generics,
            value_ty,
            assignments,
        } = self;
        format!(
            "{}pub fn {name}{generics}(mut self, value: {value_ty}) -> Self {{ {assignments} self }}\n",
            doc.render()
        )
    }
}

fn sets(m: &Field, how: &str) -> Doc {
    Doc::new().text(format!("Sets {}{how}.", field_link(&m.name)))
}

pub(super) fn member_methods(
    model: &Model<'_>,
    s: &Struct,
    m: &Field,
    counted: &CountedArrays<'_>,
    names: &mut MethodNames<'_>,
) -> String {
    let ty = CType::of(m);
    let field = field_name(&m.name);
    if ty.is_array() {
        let setter = value_setter(model, m, names).render();
        return setter + &fixed_array_reader(model, s, m, &ty, names);
    }
    let setter = match ty.depth() {
        0 if model.canonical(&m.ty) == "VkBool32" => Setter {
            doc: sets(m, " to `VK_TRUE` or `VK_FALSE`"),
            name: names.claim(&field, &member_origin(m)),
            generics: "",
            value_ty: "bool".to_string(),
            assignments: format!("self.{field} = value as Bool32;"),
        },
        0 => value_setter(model, m, names),
        1 => pointer_setter(model, s, m, &ty, counted, names),
        _ => pointer_array_setter(model, s, m, &ty, counted, names)
            .unwrap_or_else(|| value_setter(model, m, names)),
    };
    setter.render()
}

fn value_setter(model: &Model<'_>, m: &Field, names: &mut MethodNames<'_>) -> Setter {
    let field = field_name(&m.name);
    Setter {
        doc: sets(m, ""),
        name: names.claim(&field, &member_origin(m)),
        generics: "",
        value_ty: model.member_ty(m, "'a"),
        assignments: format!("self.{field} = value;"),
    }
}

fn fixed_array_reader(
    model: &Model<'_>,
    s: &Struct,
    m: &Field,
    ty: &CType,
    names: &mut MethodNames<'_>,
) -> String {
    if ty.depth() != 0 || ty.arrays.len() != 1 {
        return String::new();
    }
    let field = field_name(&m.name);
    let plain = field.trim_start_matches("r#");
    if ty.base == "char" {
        let reader = names.claim(&format!("{plain}_as_c_str"), &member_origin(m));
        let doc = Doc::new()
            .text(format!(
                "Reads {} as a C string that ends at its first NUL.",
                field_link(&m.name)
            ))
            .section("Errors", "Returns an error if the array contains no NUL.")
            .render();
        return format!(
            "{doc}pub fn {reader}(&self) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {{ c_str_from_chars(&self.{field}) }}\n"
        );
    }
    let Some(count) = count_member(s, m) else {
        return String::new();
    };
    let reader = names.claim(&format!("{plain}_as_slice"), &member_origin(m));
    let doc = Doc::new()
        .text(format!(
            "The first {} elements of {}, or all of them if the count is larger than the array.",
            field_link(&count.name),
            field_link(&m.name)
        ))
        .render();
    format!(
        "{doc}pub fn {reader}(&self) -> &[{}] {{ let len = (self.{} as usize).min(self.{field}.len()); &self.{field}[..len] }}\n",
        model.base_ty(&ty.base, "'a"),
        field_name(&count.name)
    )
}

fn pointer_setter_method_name(m: &Field, names: &mut MethodNames<'_>) -> String {
    let name = pointer_setter_name(&m.name);
    if names.is_taken(&name) {
        names.claim(&format!("{name}_ptrs"), &member_origin(m))
    } else {
        names.claim(&name, &member_origin(m))
    }
}

fn pointer_setter(
    model: &Model<'_>,
    s: &Struct,
    m: &Field,
    ty: &CType,
    counted: &CountedArrays<'_>,
    names: &mut MethodNames<'_>,
) -> Setter {
    let field = field_name(&m.name);
    let borrow = Borrow::of_pointer(ty.outer_const());
    let as_ptr = borrow.slice_as_ptr();
    let pointee = model.base_ty(&ty.base, "'a");
    let mut generics = "";
    let (doc, value_ty, assignments) = if let Some(count) = count_member(s, m) {
        let length = void_array_len(m).map(|_| void_array_length(&s.name, m));
        let (element, cast, of, element_types) = match length {
            Some(VoidArrayLength::Bytes) => ("u8", ".cast()", "bytes", Vec::new()),
            Some(VoidArrayLength::Values(typed)) => {
                generics = "<T>";
                let element_types = typed_values_doc(model, s, count, typed);
                ("T", ".cast()", "values", element_types)
            }
            None => (pointee.as_str(), "", "elements", Vec::new()),
        };
        (
            counted.slice_doc(model, count, m, of, element_types),
            borrow.reference("'a", &format!("[{element}]")),
            format!(
                "{}{} self.{field} = value.{as_ptr}(){cast};",
                counted.assert_same_length_as_the_others(count, m),
                counted.set_count(model, count, m)
            ),
        )
    } else if m.len.as_deref() == Some(SPIRV_WORD_COUNT) {
        assert!(
            ty.base == "uint32_t" && borrow == Borrow::Shared,
            "{}: unexpected pCode",
            s.name
        );
        (
            Doc::new().text(format!(
                "Sets {} to the start of a slice of SPIR-V words and [`code_size`](Self::code_size) to its size in bytes.",
                field_link(&m.name)
            )),
            "&'a [u32]".to_string(),
            format!(
                "self.code_size = core::mem::size_of_val(value); self.{field} = value.as_ptr();"
            ),
        )
    } else if m.null_terminated && ty.base == "char" && borrow == Borrow::Shared {
        (
            sets(m, " to a NUL-terminated string"),
            "&'a CStr".to_string(),
            format!("self.{field} = value.as_ptr();"),
        )
    } else if is_opaque(&ty.base) {
        (
            sets(m, ""),
            model.member_ty(m, "'a"),
            format!("self.{field} = value;"),
        )
    } else if let Some(len) = m.len.as_deref() {
        let null = if m.optional {
            "The registry marks the member as optional, and the specification describes what a null pointer means for it."
        } else {
            "The member is not optional: the C API needs the elements."
        };
        (
            Doc::new().text(format!(
                "Sets {} to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: the C API expects `{len}` elements. {null}",
                field_link(&m.name)
            )),
            borrow.reference("'a", &format!("[{pointee}]")),
            format!(
                "self.{field} = if value.is_empty() {{ {} }} else {{ value.{as_ptr}() }};",
                null_ptr(ty.outer_const())
            ),
        )
    } else {
        (
            sets(m, " to point to `value`"),
            borrow.reference("'a", &pointee),
            format!("self.{field} = {}(value);", borrow.ptr_from_reference()),
        )
    };
    Setter {
        doc,
        name: pointer_setter_method_name(m, names),
        generics,
        value_ty,
        assignments,
    }
}

fn typed_values_doc(
    model: &Model<'_>,
    s: &Struct,
    count: &Field,
    typed: &TypedValues,
) -> Vec<Block> {
    let type_member = s
        .members
        .iter()
        .find(|m| m.name == typed.type_member && m.ty == typed.type_enum)
        .unwrap_or_else(|| {
            panic!(
                "{} has no member `{}` of the type {}",
                s.name, typed.type_member, typed.type_enum
            )
        });
    let values = &model.get_enum(typed.type_enum).fields;
    let listed = typed.value_types.iter().map(|(value, _)| *value);
    assert!(
        listed.eq(values.iter().map(|value| value.name.as_str())),
        "{}: the Rust types listed for the values of {} have to follow its values in the registry",
        s.name,
        typed.type_enum
    );
    let types = typed.value_types.iter().map(|(value, rust_ty)| {
        let variant = model.enum_variant(typed.type_enum, value);
        (
            0,
            format!(
                "[`{}::{variant}`] stands for {rust_ty}",
                type_name(typed.type_enum)
            ),
        )
    });
    vec![
        Block::Text(format!(
            "`T` has to be the type of the values that {} names, because the C API reads `{}` values of that type from the array:",
            field_link(&type_member.name),
            count.name
        )),
        Block::List(types.collect()),
    ]
}

fn pointer_array_setter(
    model: &Model<'_>,
    s: &Struct,
    m: &Field,
    ty: &CType,
    counted: &CountedArrays<'_>,
    names: &mut MethodNames<'_>,
) -> Option<Setter> {
    let count = count_member(s, m)?;
    if ty.pointers != [true, true] || is_opaque(&ty.base) {
        return None;
    }
    let (element, of) = if ty.base == "char" {
        assert!(
            m.null_terminated,
            "{}::{}: char** without NUL",
            s.name, m.name
        );
        (
            "CStrPtr<'a>".to_string(),
            "pointers to NUL-terminated strings",
        )
    } else {
        (
            format!("&'a {}", model.base_ty(&ty.base, "'a")),
            "references, passed as an array of pointers",
        )
    };
    Some(Setter {
        doc: counted.slice_doc(model, count, m, of, Vec::new()),
        name: pointer_setter_method_name(m, names),
        generics: "",
        value_ty: format!("&'a [{element}]"),
        assignments: format!(
            "{}{} self.{} = value.as_ptr().cast();",
            counted.assert_same_length_as_the_others(count, m),
            counted.set_count(model, count, m),
            field_name(&m.name)
        ),
    })
}

pub(super) fn push_next(p_next: &Field) -> String {
    let head = match p_next.ty.as_str() {
        "void" => "push_chain(next, self.p_next)",
        _ => "push_chain(next, self.p_next.cast()) as _",
    };
    format!(
        "/// Puts `next` at the head of the `pNext` chain of this structure.
        ///
        /// `next` has to be a structure that the Vulkan registry lists as extending this one,
        /// and it stays mutably borrowed for as long as this structure is used. Its own `pNext`
        /// is overwritten with the previous head of the chain, so structures pushed earlier stay
        /// in the chain behind it.
        pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self {{
            self.p_next = {head};
            self
        }}\n"
    )
}

#[cfg(test)]
mod tests {
    use super::super::definition;
    use super::*;
    use crate::emit::{doc_of, squash};
    use crate::test_support::{counted, field, optional, registry, structure};
    use std::collections::BTreeSet;
    use vulkan_registry::Registry;

    #[test]
    fn slices_without_a_count_member_set_null_when_empty_and_counted_slices_do_not() {
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
                counted("pVersionData", "const uint8_t*", "2*VK_UUID_SIZE"),
                field("wordCount", "uint32_t"),
                optional(counted("pWords", "const uint32_t*", "wordCount")),
            ],
        ));
        let model = Model::new(&ir);
        let out = definition(&model, &ir.structs[0]);
        let squashed = squash(&out);
        for (setter, element, member) in [
            ("sample_mask", "u32", "p_sample_mask"),
            ("version_data", "u8", "p_version_data"),
        ] {
            assert!(
                squashed.contains(&format!(
                    "pub fn {setter}(mut self, value: &'a [{element}]) -> Self {{ self.{member} = if value.is_empty() {{ core::ptr::null() }} else {{ value.as_ptr() }}; self }}"
                )),
                "{setter}: {squashed}"
            );
        }
        assert!(
            squashed.contains(
                "pub fn words(mut self, value: &'a [u32]) -> Self { self.word_count = len_as_count(value.len(), \"VkMasksInfo::pWords\", \"wordCount\"); self.p_words = value.as_ptr(); self }"
            ),
            "{squashed}"
        );
        assert_eq!(
            doc_of(&out, "pub fn sample_mask("),
            [
                "Sets [`p_sample_mask`](Self::p_sample_mask) to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: the C API expects `(samples + 31) / 32` elements. The registry marks the member as optional, and the specification describes what a null pointer means for it."
            ]
        );
        assert_eq!(
            doc_of(&out, "pub fn version_data("),
            [
                "Sets [`p_version_data`](Self::p_version_data) to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: the C API expects `2*VK_UUID_SIZE` elements. The member is not optional: the C API needs the elements."
            ]
        );
    }

    fn registry_struct(name: &str) -> &'static Struct {
        Registry::get()
            .structs
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("the registry has no {name}"))
    }

    #[test]
    fn a_void_array_counted_in_values_takes_a_typed_slice_and_stores_the_number_of_values() {
        let model = Model::new(Registry::get());
        let out = definition(&model, registry_struct("VkLayerSettingEXT"));
        assert!(
            squash(&out).contains(
                "pub fn values<T>(mut self, value: &'a [T]) -> Self { self.value_count = len_as_count(value.len(), \"VkLayerSettingEXT::pValues\", \"valueCount\"); self.p_values = value.as_ptr().cast(); self }"
            ),
            "{out}"
        );
        assert_eq!(
            doc_of(&out, "pub fn values<T>("),
            [
                "Sets [`p_values`](Self::p_values) to the start of a slice of values and [`value_count`](Self::value_count) to its length.",
                "`T` has to be the type of the values that [`ty`](Self::ty) names, because the C API reads `valueCount` values of that type from the array:",
                "- [`LayerSettingTypeEXT::BOOL32`] stands for [`Bool32`] - [`LayerSettingTypeEXT::INT32`] stands for `i32` - [`LayerSettingTypeEXT::INT64`] stands for `i64` - [`LayerSettingTypeEXT::UINT32`] stands for `u32` - [`LayerSettingTypeEXT::UINT64`] stands for `u64` - [`LayerSettingTypeEXT::FLOAT32`] stands for `f32` - [`LayerSettingTypeEXT::FLOAT64`] stands for `f64` - [`LayerSettingTypeEXT::STRING`] stands for [`CStrPtr`], or `*const c_char`, a pointer to a NUL-terminated string",
                "# Panics",
                "Panics if the slice has more elements than `valueCount`, a `uint32_t`, can count.",
            ]
        );
    }

    #[test]
    fn a_setting_type_without_a_listed_rust_type_stops_the_generator() {
        let mut ir = Registry::get().clone();
        let types = ir
            .enums
            .iter_mut()
            .find(|e| e.name == "VkLayerSettingTypeEXT")
            .expect("VkLayerSettingTypeEXT");
        let mut added = types.fields[0].clone();
        added.name = "VK_LAYER_SETTING_TYPE_INT16_EXT".to_string();
        added.value = "8".to_string();
        types.fields.push(added);
        let model = Model::new(&ir);
        let message = crate::test_support::catch_quietly(|| {
            definition(&model, registry_struct("VkLayerSettingEXT"))
        })
        .expect_err("a value without a Rust type");
        assert_eq!(
            message,
            "VkLayerSettingEXT: the Rust types listed for the values of VkLayerSettingTypeEXT have to follow its values in the registry"
        );
    }

    #[test]
    fn a_void_array_counted_in_bytes_takes_bytes() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkBlobInfo",
            vec![
                field("dataSize", "size_t"),
                counted("pData", "const void*", "dataSize"),
            ],
        ));
        let model = Model::new(&ir);
        let out = definition(&model, &ir.structs[0]);
        assert!(
            squash(&out).contains(
                "pub fn data(mut self, value: &'a [u8]) -> Self { self.data_size = value.len() as _; self.p_data = value.as_ptr().cast(); self }"
            ),
            "{out}"
        );
        assert_eq!(
            doc_of(&out, "pub fn data("),
            [
                "Sets [`p_data`](Self::p_data) to the start of a slice of bytes and [`data_size`](Self::data_size) to its length."
            ]
        );
    }

    #[test]
    #[should_panic(
        expected = "VkBlobInfo::pData: the registry gives `blobCount` as the length of this `void` array, which is not the name of a size in bytes"
    )]
    fn a_void_array_whose_length_is_not_named_as_a_size_stops_the_generator() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkBlobInfo",
            vec![
                field("blobCount", "uint32_t"),
                counted("pData", "const void*", "blobCount"),
            ],
        ));
        definition(&Model::new(&ir), &ir.structs[0]);
    }

    #[test]
    fn the_void_array_setters_of_the_registry_take_bytes_unless_the_length_counts_values() {
        let model = Model::new(Registry::get());
        let mut checked = 0;
        for s in &model.ir.structs {
            let out = squash(&definition(&model, s));
            for m in s.members.iter().filter(|m| void_array_len(m).is_some()) {
                let Some(count) = count_member(s, m) else {
                    panic!("{}::{}: no member holds the length", s.name, m.name);
                };
                let setter = pointer_setter_name(&m.name);
                let (generics, element) = match void_array_length(&s.name, m) {
                    VoidArrayLength::Bytes => ("", "u8"),
                    VoidArrayLength::Values(_) => ("<T>", "T"),
                };
                let mutability = if CType::of(m).outer_const() {
                    ""
                } else {
                    "mut "
                };
                let signature = format!(
                    "pub fn {setter}{generics}(mut self, value: &'a {mutability}[{element}]) -> Self {{"
                );
                assert!(out.contains(&signature), "{}: {signature}: {out}", s.name);
                assert!(
                    out.contains(&format!("self.{} = ", field_name(&count.name))),
                    "{}::{}",
                    s.name,
                    m.name
                );
                checked += 1;
            }
        }
        assert!(checked >= 20, "{checked}");
    }

    #[test]
    fn the_slice_setters_of_the_registry_without_a_count_member_set_null_when_empty() {
        let model = Model::new(Registry::get());
        let mut uncounted = BTreeSet::new();
        for s in &model.ir.structs {
            let out = squash(&definition(&model, s));
            for m in &s.members {
                let ty = CType::of(m);
                let is_uncounted_slice = ty.depth() == 1
                    && !ty.is_array()
                    && m.len.is_some()
                    && count_member(s, m).is_none()
                    && m.len.as_deref() != Some(SPIRV_WORD_COUNT)
                    && !(m.null_terminated && ty.base == "char")
                    && !is_opaque(&ty.base);
                let field = field_name(&m.name);
                let sets_null_when_empty =
                    out.contains(&format!("self.{field} = if value.is_empty() {{"));
                assert_eq!(
                    sets_null_when_empty, is_uncounted_slice,
                    "{}::{}: {out}",
                    s.name, m.name
                );
                if is_uncounted_slice {
                    uncounted.insert(format!("{}::{}", s.name, m.name));
                }
            }
        }
        assert_eq!(
            uncounted,
            [
                "StdVideoH265HrdParameters::pSubLayerHrdParametersNal",
                "StdVideoH265HrdParameters::pSubLayerHrdParametersVcl",
                "VkAccelerationStructureVersionInfoKHR::pVersionData",
                "VkMicromapVersionInfoEXT::pVersionData",
                "VkPipelineMultisampleStateCreateInfo::pSampleMask",
            ]
            .into_iter()
            .map(String::from)
            .collect::<BTreeSet<_>>(),
            "crates/vulkan/tests/uncounted_slices.rs checks each of these setters"
        );
    }
}
