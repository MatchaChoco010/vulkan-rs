use super::{MethodNames, thread_safety_impls};
use crate::emit::{Borrow, w};
use crate::model::{CType, Model, is_opaque};
use crate::naming::{field_name, pointer_setter_name, type_name};
use vulkan_registry::{Field, Struct};

pub(super) fn definition(model: &Model<'_>, s: &Struct) -> String {
    let cfg = model.cfg(s.protect.as_deref());
    let name = type_name(&s.name);
    let borrows = model.has_lifetime(&s.name);
    let (generics, anonymous) = if borrows { ("<'a>", "<'_>") } else { ("", "") };

    let mut fields = String::new();
    let mut constructors = String::new();
    let mut names = MethodNames::new(&s.name);
    for m in &s.members {
        assert!(
            m.bit_field_width.is_none(),
            "{}: bit-field in union",
            s.name
        );
        w!(
            fields,
            "pub {}: {},",
            field_name(&m.name),
            model.member_ty(m, "'a")
        );
        constructors.push_str(&constructor(model, m, &mut names));
    }
    if borrows {
        w!(fields, "pub _marker: PhantomData<&'a ()>,");
    }

    let mut out = String::new();
    w!(
        out,
        r#"{cfg}#[repr(C)]
#[derive(Copy, Clone)]
pub union {name}{generics} {{
    {fields}}}

{thread_safety}{cfg}impl Default for {name}{anonymous} {{
    fn default() -> Self {{
        unsafe {{ core::mem::zeroed() }}
    }}
}}

{cfg}#[cfg(feature = "debug")]
impl core::fmt::Debug for {name}{anonymous} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        f.debug_struct("{name}").finish_non_exhaustive()
    }}
}}

{cfg}impl{generics} {name}{generics} {{
    {constructors}}}
"#,
        thread_safety = thread_safety_impls(&cfg, &name, borrows),
    );
    out
}

fn constructor(model: &Model<'_>, m: &Field, names: &mut MethodNames<'_>) -> String {
    let ty = CType::of(m);
    let field = field_name(&m.name);
    let short_name = pointer_setter_name(&m.name);
    let name = if names.is_taken(&short_name) {
        names.claim(&field)
    } else {
        names.claim(&short_name)
    };
    let borrows_one_value = ty.depth() == 1 && !ty.is_array() && !is_opaque(&ty.base);
    let (value_ty, value) = if borrows_one_value {
        let borrow = Borrow::of_pointer(ty.outer_const());
        (
            borrow.reference("'a", &model.base_ty(&ty.base, "'a")),
            format!("{}(value)", borrow.ptr_from_reference()),
        )
    } else {
        (model.member_ty(m, "'a"), "value".to_string())
    };
    format!("pub fn {name}(value: {value_ty}) -> Self {{ Self {{ {field}: {value} }} }}\n")
}
