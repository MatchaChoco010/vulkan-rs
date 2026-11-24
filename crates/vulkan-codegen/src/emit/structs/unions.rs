use super::read_only_members::{PointsTo, read_only};
use super::{
    MARKER_DOC, MethodNames, field_link, marker_field, member_doc, member_origin,
    thread_safety_impls, type_doc,
};
use crate::docs::Doc;
use crate::emit::{Borrow, assert_distinct_names, w};
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
    assert_distinct_names(
        &s.name,
        s.members
            .iter()
            .map(|m| (format!("the C member `{}`", m.name), field_name(&m.name)))
            .chain(marker_field(borrows)),
    );
    for m in &s.members {
        assert!(
            m.bit_field_width.is_none(),
            "{}: bit-field in union",
            s.name
        );
        w!(
            fields,
            "{}pub {}: {},",
            member_doc(model, &cfg, s, m).render(),
            field_name(&m.name),
            model.member_ty(m, "'a")
        );
        constructors.push_str(&constructor(model, s, m, &mut names));
    }
    if borrows {
        w!(fields, "{MARKER_DOC}pub _marker: PhantomData<&'a ()>,");
    }

    let mut out = String::new();
    w!(
        out,
        r#"{doc}{cfg}#[repr(C)]
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
        doc = type_doc(model, &cfg, s).render(),
        thread_safety = thread_safety_impls(&cfg, &name, borrows),
    );
    out
}

fn constructor(model: &Model<'_>, s: &Struct, m: &Field, names: &mut MethodNames<'_>) -> String {
    let ty = CType::of(m);
    let field = field_name(&m.name);
    let short_name = pointer_setter_name(&m.name);
    let name = if names.is_taken(&short_name) {
        names.claim(&field, &member_origin(m))
    } else {
        names.claim(&short_name, &member_origin(m))
    };
    let is_pointer = ty.depth() == 1 && !ty.is_array();
    let borrows_a_string = is_pointer && m.null_terminated && ty.base == "char" && ty.outer_const();
    let borrows_one_value = is_pointer && !borrows_a_string && !is_opaque(&ty.base);
    let read_only = read_only(s, m);
    assert!(
        read_only.is_none_or(|listed| listed.points_to == PointsTo::OneValue && borrows_one_value),
        "{}::{}: the list of members that the implementation only reads names the member, and the constructor of a union borrows only one value that is not a string",
        s.name,
        m.name
    );
    let (value_ty, value) = if borrows_a_string {
        ("&'a CStr".to_string(), "value.as_ptr()".to_string())
    } else if borrows_one_value {
        let borrow = Borrow::of_pointer(ty.outer_const() || read_only.is_some());
        let cast_mut = if read_only.is_some() {
            ".cast_mut()"
        } else {
            ""
        };
        (
            borrow.reference("'a", &model.base_ty(&ty.base, "'a")),
            format!("{}(value){cast_mut}", borrow.ptr_from_reference()),
        )
    } else {
        (model.member_ty(m, "'a"), "value".to_string())
    };
    let how = if borrows_a_string {
        " pointing to the NUL-terminated string `value`"
    } else if borrows_one_value {
        " pointing to `value`"
    } else {
        " set to `value`"
    };
    let doc = Doc::new()
        .text(format!(
            "Creates the union with {link}{how}. The bytes of the union outside {link} are zero.",
            link = field_link(&m.name)
        ))
        .text(read_only.map_or_else(String::new, |listed| listed.note("constructor")))
        .render();
    format!(
        "{doc}pub fn {name}(value: {value_ty}) -> Self {{ let mut union = Self::default(); union.{field} = {value}; union }}\n"
    )
}
