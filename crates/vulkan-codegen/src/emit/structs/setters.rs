use super::MethodNames;
use crate::emit::Borrow;
use crate::model::{CType, Model, is_opaque};
use crate::naming::{field_name, pointer_setter_name};
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
    name: String,
    value_ty: String,
    assignments: String,
}

impl Setter {
    fn render(&self) -> String {
        let Setter {
            name,
            value_ty,
            assignments,
        } = self;
        format!("pub fn {name}(mut self, value: {value_ty}) -> Self {{ {assignments} self }}\n")
    }
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
            name: names.claim(&field),
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
        name: names.claim(&field),
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
        let reader = names.claim(&format!("{plain}_as_c_str"));
        return format!(
            "pub fn {reader}(&self) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {{ c_str_from_chars(&self.{field}) }}\n"
        );
    }
    let Some(count) = count_member(s, m) else {
        return String::new();
    };
    let reader = names.claim(&format!("{plain}_as_slice"));
    format!(
        "pub fn {reader}(&self) -> &[{}] {{ let len = (self.{} as usize).min(self.{field}.len()); &self.{field}[..len] }}\n",
        model.base_ty(&ty.base, "'a"),
        field_name(&count.name)
    )
}

fn pointer_setter_method_name(m: &Field, names: &mut MethodNames<'_>) -> String {
    let name = pointer_setter_name(&m.name);
    if names.is_taken(&name) {
        names.claim(&format!("{name}_ptrs"))
    } else {
        names.claim(&name)
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
    let (value_ty, assignments) = if let Some(count) = count_member(s, m) {
        let (element, cast) = match ty.base.as_str() {
            "void" => ("u8", ".cast()"),
            _ => (pointee.as_str(), ""),
        };
        (
            borrow.reference("'a", &format!("[{element}]")),
            format!(
                "{}self.{} = value.len() as _; self.{field} = value.{as_ptr}(){cast};",
                counted.assert_same_length_as_the_others(count, m),
                field_name(&count.name)
            ),
        )
    } else if m.len.as_deref() == Some(SPIRV_WORD_COUNT) {
        assert!(
            ty.base == "uint32_t" && borrow == Borrow::Shared,
            "{}: unexpected pCode",
            s.name
        );
        (
            "&'a [u32]".to_string(),
            format!(
                "self.code_size = core::mem::size_of_val(value); self.{field} = value.as_ptr();"
            ),
        )
    } else if m.null_terminated && ty.base == "char" && borrow == Borrow::Shared {
        (
            "&'a CStr".to_string(),
            format!("self.{field} = value.as_ptr();"),
        )
    } else if is_opaque(&ty.base) {
        (model.member_ty(m, "'a"), format!("self.{field} = value;"))
    } else if m.len.is_some() {
        (
            borrow.reference("'a", &format!("[{pointee}]")),
            format!("self.{field} = value.{as_ptr}();"),
        )
    } else {
        (
            borrow.reference("'a", &pointee),
            format!("self.{field} = {}(value);", borrow.ptr_from_reference()),
        )
    };
    Setter {
        name: pointer_setter_method_name(m, names),
        value_ty,
        assignments,
    }
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
    let element = if ty.base == "char" {
        assert!(
            m.null_terminated,
            "{}::{}: char** without NUL",
            s.name, m.name
        );
        "CStrPtr<'a>".to_string()
    } else {
        format!("&'a {}", model.base_ty(&ty.base, "'a"))
    };
    Some(Setter {
        name: pointer_setter_method_name(m, names),
        value_ty: format!("&'a [{element}]"),
        assignments: format!(
            "{}self.{} = value.len() as _; self.{} = value.as_ptr().cast();",
            counted.assert_same_length_as_the_others(count, m),
            field_name(&count.name),
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
        "pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self {{
            self.p_next = {head};
            self
        }}\n"
    )
}
