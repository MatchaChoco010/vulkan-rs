//! Struct and union emitter.
//!
//! Lifetime rules, which the trybuild tests in `crates/vulkan/tests/ui` pin down:
//!
//! - A struct has a lifetime parameter `'a` exactly when it can hold a pointer.
//! - Every setter that stores a pointer takes a `&'a` reference or slice, so the struct
//!   cannot outlive what it points to.
//! - The only setters that take raw pointers are those for `void*` and opaque platform
//!   types, where Rust has no type to borrow.

use super::{header, w};
use crate::model::{CType, Model, provided_by};
use crate::naming::{const_name, enum_variant_name, field_name, pointer_setter_name, type_name};
use std::collections::{BTreeMap, BTreeSet};
use vulkan_registry::{Field, Struct};

/// A field of the emitted Rust struct.
pub enum Slot<'a> {
    Plain(&'a Field),
    /// Consecutive C bit-fields packed into one `u32` storage unit.
    Bits {
        storage: String,
        bits: Vec<Bit<'a>>,
    },
}

pub struct Bit<'a> {
    pub member: &'a Field,
    pub shift: u32,
    pub width: u32,
}

/// Groups C bit-fields into storage units the way the C ABIs used by Vulkan do: fields of
/// a 32-bit type are packed, in order, into 32-bit units, and a field that does not fit
/// starts a new unit.
pub fn slots<'a>(model: &Model<'_>, s: &'a Struct) -> Vec<Slot<'a>> {
    let mut out: Vec<Slot<'a>> = Vec::new();
    let mut used = 0;
    let mut open = false;
    let mut units = 0;
    for m in &s.members {
        let Some(width) = m.bit_field_width else {
            out.push(Slot::Plain(m));
            open = false;
            continue;
        };
        let base = model.canonical(&m.ty);
        assert!(
            base == "uint32_t" || model.is_flags(base),
            "{}::{}: bit-field of unsupported type {}",
            s.name,
            m.name,
            m.ty
        );
        assert!(
            (1..=32).contains(&width) && CType::of(m).depth() == 0,
            "{}::{}: unsupported bit-field",
            s.name,
            m.name
        );
        if !open || used + width > 32 {
            out.push(Slot::Bits {
                storage: format!("bitfield{units}"),
                bits: Vec::new(),
            });
            units += 1;
            used = 0;
            open = true;
        }
        let Some(Slot::Bits { bits, .. }) = out.last_mut() else {
            unreachable!()
        };
        bits.push(Bit {
            member: m,
            shift: used,
            width,
        });
        used += width;
    }
    out
}

pub fn emit_structs(model: &Model<'_>) -> String {
    let mut out = header(&["derivable_impls"]);
    out.push_str("use core::ffi::{c_char, c_void, CStr};\nuse core::marker::PhantomData;\n\n");
    for s in &model.ir.structs {
        if s.union {
            emit_union(&mut out, model, s);
        } else {
            emit_struct(&mut out, model, s);
        }
        let lt = if model.has_lifetime(&s.name) {
            "<'a>"
        } else {
            ""
        };
        for alias in &s.aliases {
            w!(
                out,
                "{}pub type {}{lt} = {}{lt};\n",
                model.cfg(s.protect.as_deref()),
                type_name(alias),
                type_name(&s.name)
            );
        }
    }
    for s in &model.ir.structs {
        for root in &s.extends {
            let Some(root) = model.get_struct(root) else {
                continue;
            };
            assert!(
                model.has_p_next(s) && model.has_p_next(root),
                "{} extends {} but one of them has no pNext",
                s.name,
                root.name
            );
            w!(
                out,
                "{}unsafe impl Extends<{}<'_>> for {}<'_> {{}}",
                cfg_all(model, &[s.protect.as_deref(), root.protect.as_deref()]),
                type_name(&root.name),
                type_name(&s.name)
            );
        }
    }
    out
}

fn cfg_all(model: &Model<'_>, protects: &[Option<&str>]) -> String {
    let features = protects
        .iter()
        .flatten()
        .map(|p| model.feature(p))
        .collect::<BTreeSet<_>>();
    match features.len() {
        0 => String::new(),
        1 => format!("#[cfg(feature = \"{}\")]\n", features.first().expect("one")),
        _ => format!(
            "#[cfg(all({}))]\n",
            features
                .iter()
                .map(|f| format!("feature = \"{f}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn doc(out: &mut String, s: &Struct) {
    if s.video_std_header.is_some() {
        w!(out, "/// `{}`", s.name);
    } else {
        w!(
            out,
            "/// [`{0}`](https://registry.khronos.org/vulkan/specs/latest/man/html/{0}.html)",
            s.name
        );
    }
    let provided = provided_by(&s.extensions, s.version.as_deref());
    if !provided.is_empty() {
        w!(out, "///\n/// {provided}");
    }
}

fn emit_struct(out: &mut String, model: &Model<'_>, s: &Struct) {
    let cfg = model.cfg(s.protect.as_deref());
    let name = type_name(&s.name);
    let lt = model.has_lifetime(&s.name);
    let (generics, anon) = if lt { ("<'a>", "<'_>") } else { ("", "") };
    let slots = slots(model, s);

    out.push_str(&cfg);
    doc(out, s);
    let mut derives = vec!["Copy", "Clone"];
    if model.derives_partial_eq(&s.name) {
        derives.push("PartialEq");
    }
    if model.derives_eq_hash(&s.name) {
        derives.extend(["Eq", "Hash"]);
    }
    w!(out, "#[repr(C)]");
    w!(out, "#[derive({})]", derives.join(", "));
    w!(out, "#[cfg_attr(feature = \"debug\", derive(Debug))]");
    w!(out, "pub struct {name}{generics} {{");
    for slot in &slots {
        match slot {
            Slot::Plain(m) => w!(
                out,
                "    pub {}: {},",
                field_name(&m.name),
                model.member_ty(m, "'a")
            ),
            Slot::Bits { storage, bits } => {
                let names = bits
                    .iter()
                    .map(|b| format!("`{}:{}`", b.member.name, b.width))
                    .collect::<Vec<_>>()
                    .join(", ");
                w!(
                    out,
                    "    /// C bit-fields {names}, first field in the low bits."
                );
                w!(out, "    pub {storage}: u32,");
            }
        }
    }
    if lt {
        w!(out, "    pub _marker: PhantomData<&'a ()>,");
    }
    w!(out, "}}\n");

    if lt {
        // Pointers stored in a struct are plain data. They are only ever dereferenced by
        // Vulkan inside an `unsafe` command call.
        w!(out, "{cfg}unsafe impl Send for {name}<'_> {{}}");
        w!(out, "{cfg}unsafe impl Sync for {name}<'_> {{}}\n");
    }

    w!(out, "{cfg}impl Default for {name}{anon} {{");
    w!(out, "    fn default() -> Self {{");
    w!(out, "        Self {{");
    for slot in &slots {
        match slot {
            Slot::Plain(m) => w!(
                out,
                "            {}: {},",
                field_name(&m.name),
                default_expr(model, s, m)
            ),
            Slot::Bits { storage, .. } => w!(out, "            {storage}: 0,"),
        }
    }
    if lt {
        w!(out, "            _marker: PhantomData,");
    }
    w!(out, "        }}\n    }}\n}}\n");

    w!(out, "{cfg}impl{generics} {name}{generics} {{");
    let mut emitted = BTreeSet::new();
    let counts = count_users(s);
    for slot in &slots {
        match slot {
            Slot::Plain(m) if m.name == "sType" || m.name == "pNext" => {}
            Slot::Plain(m) => emit_setters(out, model, s, m, &counts, &mut emitted),
            Slot::Bits { storage, bits } => {
                for bit in bits {
                    emit_bit_accessors(out, model, storage, bit, &mut emitted);
                }
            }
        }
    }
    if let Some(p_next) = s.members.iter().find(|m| m.name == "pNext") {
        // VkBaseInStructure / VkBaseOutStructure type their own `pNext`.
        let assign = if p_next.ty == "void" {
            "self.p_next = push_chain(next, self.p_next);"
        } else {
            "self.p_next = push_chain(next, self.p_next.cast()) as _;"
        };
        w!(out, "    /// Prepends `next` to the `pNext` chain.");
        w!(out, "    ///");
        w!(
            out,
            "    /// `next.p_next` is overwritten, so push extension structs one at a time."
        );
        w!(
            out,
            "    pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self {{"
        );
        w!(out, "        {assign}");
        w!(out, "        self\n    }}");
    }
    w!(out, "}}\n");

    if model.has_p_next(s)
        && let Some(s_type) = &s.s_type
    {
        w!(out, "{cfg}unsafe impl TaggedStructure for {name}<'_> {{");
        w!(
            out,
            "    const STRUCTURE_TYPE: StructureType = StructureType::{};",
            enum_variant_name("VkStructureType", s_type, &model.ir.vendor_tags)
        );
        w!(out, "}}\n");
    }
}

fn emit_union(out: &mut String, model: &Model<'_>, s: &Struct) {
    let cfg = model.cfg(s.protect.as_deref());
    let name = type_name(&s.name);
    let lt = model.has_lifetime(&s.name);
    let (generics, anon) = if lt { ("<'a>", "<'_>") } else { ("", "") };

    out.push_str(&cfg);
    doc(out, s);
    w!(out, "#[repr(C)]");
    w!(out, "#[derive(Copy, Clone)]");
    w!(out, "pub union {name}{generics} {{");
    for m in &s.members {
        assert!(
            m.bit_field_width.is_none(),
            "{}: bit-field in union",
            s.name
        );
        w!(
            out,
            "    pub {}: {},",
            field_name(&m.name),
            model.member_ty(m, "'a")
        );
    }
    if lt {
        w!(out, "    pub _marker: PhantomData<&'a ()>,");
    }
    w!(out, "}}\n");
    if lt {
        w!(out, "{cfg}unsafe impl Send for {name}<'_> {{}}");
        w!(out, "{cfg}unsafe impl Sync for {name}<'_> {{}}\n");
    }
    w!(out, "{cfg}impl Default for {name}{anon} {{");
    w!(out, "    fn default() -> Self {{");
    w!(
        out,
        "        // SAFETY: every member is an integer, float, pointer, or an aggregate of"
    );
    w!(
        out,
        "        // those, for which all-zero bytes are a valid value."
    );
    w!(out, "        unsafe {{ core::mem::zeroed() }}");
    w!(out, "    }}\n}}\n");
    w!(out, "{cfg}#[cfg(feature = \"debug\")]");
    w!(out, "impl core::fmt::Debug for {name}{anon} {{");
    w!(
        out,
        "    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{"
    );
    w!(
        out,
        "        f.debug_struct(\"{name}\").finish_non_exhaustive()"
    );
    w!(out, "    }}\n}}\n");

    w!(out, "{cfg}impl{generics} {name}{generics} {{");
    let mut emitted = BTreeSet::new();
    for m in &s.members {
        let ty = CType::of(m);
        let field = field_name(&m.name);
        let mut ctor = pointer_setter_name(&m.name);
        if !emitted.insert(ctor.clone()) {
            ctor = field.clone();
            assert!(emitted.insert(ctor.clone()), "{}: duplicate {ctor}", s.name);
        }
        let opaque = ty.base == "void" || model.is_external(&ty.base);
        if ty.depth() == 1 && ty.arrays.is_empty() && !opaque {
            let (reference, convert) = if ty.pointers[0] {
                ("&'a", "core::ptr::from_ref(value)")
            } else {
                ("&'a mut", "core::ptr::from_mut(value)")
            };
            w!(out, "    /// Creates the union with `{}` selected.", m.name);
            w!(
                out,
                "    pub fn {ctor}(value: {reference} {}) -> Self {{ Self {{ {field}: {convert} }} }}",
                model.base_ty(&ty.base, "'a")
            );
        } else {
            w!(out, "    /// Creates the union with `{}` selected.", m.name);
            w!(
                out,
                "    pub fn {ctor}(value: {}) -> Self {{ Self {{ {field}: value }} }}",
                model.member_ty(m, "'a")
            );
        }
    }
    w!(out, "}}\n");
}

/// The sibling member that holds the element count of pointer member `m`.
fn len_count<'s>(s: &'s Struct, m: &Field) -> Option<&'s Field> {
    let len = m.len.as_deref()?;
    // `geometryCount,1` describes `T* const*`: `geometryCount` pointers to one `T` each.
    let first = match len.split_once(',') {
        Some((first, "1")) => first,
        Some(_) => return None,
        None => len,
    };
    s.members
        .iter()
        .find(|c| c.name == first && CType::of(c).depth() == 0 && c.fixed_size_array.is_empty())
}

/// Count member name to the pointer members whose length it is.
fn count_users(s: &Struct) -> BTreeMap<&str, Vec<&Field>> {
    let mut map = BTreeMap::<&str, Vec<&Field>>::new();
    for m in &s.members {
        let ty = CType::of(m);
        if ty.depth() == 0 || !ty.arrays.is_empty() {
            continue;
        }
        if let Some(count) = len_count(s, m) {
            map.entry(count.name.as_str()).or_default().push(m);
        }
    }
    map
}

fn default_expr(model: &Model<'_>, s: &Struct, m: &Field) -> String {
    let ty = CType::of(m);
    let mut expr = scalar_default_expr(model, s, m, &ty);
    for len in ty.arrays.iter().rev() {
        expr = format!("[{expr}; {}]", model.array_len(len));
    }
    expr
}

fn scalar_default_expr(model: &Model<'_>, s: &Struct, m: &Field, ty: &CType) -> String {
    if m.name == "sType"
        && let Some(s_type) = &s.s_type
    {
        return format!(
            "StructureType::{}",
            enum_variant_name("VkStructureType", s_type, &model.ir.vendor_tags)
        );
    }
    if ty.depth() > 0 {
        return if ty.outer_const() {
            "core::ptr::null()".to_string()
        } else {
            "core::ptr::null_mut()".to_string()
        };
    }
    if model.is_func_pointer(&m.ty) {
        return "None".to_string();
    }
    // Defaults that differ from zero because zero is never a valid or neutral value.
    if matches!(
        m.name.as_str(),
        "srcQueueFamilyIndex" | "dstQueueFamilyIndex"
    ) {
        return const_name("VK_QUEUE_FAMILY_IGNORED");
    }
    if s.name == "VkImageCreateInfo" && matches!(m.name.as_str(), "mipLevels" | "arrayLayers") {
        return "1".to_string();
    }
    let base = model.canonical(&m.ty);
    match base {
        "VkSampleCountFlagBits" => "SampleCountFlagBits::_1".to_string(),
        "float" | "double" => "0.0".to_string(),
        "char" | "int" | "uint8_t" | "int8_t" | "uint16_t" | "int16_t" | "uint32_t" | "int32_t"
        | "uint64_t" | "int64_t" | "size_t" | "VkBool32" | "VkFlags" | "VkFlags64"
        | "VkDeviceSize" | "VkDeviceAddress" | "VkSampleMask" => "0".to_string(),
        // Platform typedefs are integers or raw pointers.
        _ if model.is_external(base) || base == "VkRemoteAddressNV" => {
            "unsafe { core::mem::zeroed() }".to_string()
        }
        _ => "Default::default()".to_string(),
    }
}

/// Reserves a method name; two members mapping to one name is a generator bug.
fn reserve(emitted: &mut BTreeSet<String>, owner: &str, name: &str) {
    assert!(
        emitted.insert(name.to_string()),
        "{owner}: duplicate method `{name}`"
    );
}

fn emit_setters(
    out: &mut String,
    model: &Model<'_>,
    s: &Struct,
    m: &Field,
    counts: &BTreeMap<&str, Vec<&Field>>,
    emitted: &mut BTreeSet<String>,
) {
    let ty = CType::of(m);
    let field = field_name(&m.name);
    let by_value = |out: &mut String, emitted: &mut BTreeSet<String>| {
        reserve(emitted, &s.name, &field);
        w!(
            out,
            "    pub fn {field}(mut self, value: {}) -> Self {{ self.{field} = value; self }}",
            model.member_ty(m, "'a")
        );
    };

    if !ty.arrays.is_empty() {
        by_value(out, emitted);
        if ty.depth() == 0 && ty.arrays.len() == 1 {
            let plain = field.trim_start_matches("r#");
            if ty.base == "char" {
                let getter = format!("{plain}_as_c_str");
                reserve(emitted, &s.name, &getter);
                w!(out, "    /// Reads `{}` up to its first NUL.", m.name);
                w!(
                    out,
                    "    pub fn {getter}(&self) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {{ c_str_from_chars(&self.{field}) }}"
                );
            } else if let Some(count) = len_count(s, m) {
                let getter = format!("{plain}_as_slice");
                reserve(emitted, &s.name, &getter);
                w!(
                    out,
                    "    /// The first `{}` elements of `{}`.",
                    count.name,
                    m.name
                );
                w!(
                    out,
                    "    pub fn {getter}(&self) -> &[{}] {{ let len = (self.{} as usize).min(self.{field}.len()); &self.{field}[..len] }}",
                    model.base_ty(&ty.base, "'a"),
                    field_name(&count.name)
                );
            }
        }
        return;
    }

    if ty.depth() == 0 {
        if model.canonical(&m.ty) == "VkBool32" {
            reserve(emitted, &s.name, &field);
            w!(
                out,
                "    pub fn {field}(mut self, value: bool) -> Self {{ self.{field} = value as Bool32; self }}"
            );
        } else {
            by_value(out, emitted);
        }
        return;
    }

    let count = len_count(s, m);
    let opaque = ty.base == "void" || model.is_external(&ty.base);
    let mut name = pointer_setter_name(&m.name);
    if emitted.contains(&name) {
        // `pGeometries` and `ppGeometries` would both be `geometries`.
        name = format!("{name}_ptrs");
    }
    // Other pointer members that share this member's count.
    let shared_check = |count: &Field| {
        let others = counts[count.name.as_str()]
            .iter()
            .filter(|other| other.name != m.name)
            .map(|other| format!("!self.{}.is_null()", field_name(&other.name)))
            .collect::<Vec<_>>();
        if others.is_empty() {
            String::new()
        } else {
            format!(
                "if {} {{ assert_eq!(self.{} as usize, value.len(), \"{}::{}: every array sharing `{}` must have the same length\"); }} ",
                others.join(" || "),
                field_name(&count.name),
                s.name,
                m.name,
                count.name
            )
        }
    };

    if ty.depth() == 1 {
        let is_const = ty.pointers[0];
        let (reference, as_ptr) = if is_const {
            ("&'a", "as_ptr")
        } else {
            ("&'a mut", "as_mut_ptr")
        };
        if let Some(count) = count {
            let (elem, cast) = if ty.base == "void" {
                ("u8".to_string(), ".cast()")
            } else {
                (model.base_ty(&ty.base, "'a"), "")
            };
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    /// Sets `{}` and `{}` from a slice.",
                m.name,
                count.name
            );
            w!(
                out,
                "    pub fn {name}(mut self, value: {reference} [{elem}]) -> Self {{ {}self.{} = value.len() as _; self.{field} = value.{as_ptr}(){cast}; self }}",
                shared_check(count),
                field_name(&count.name)
            );
        } else if m.len.as_deref() == Some("codeSize / 4") {
            assert!(
                ty.base == "uint32_t" && is_const,
                "{}: unexpected pCode",
                s.name
            );
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    /// Sets `{}` and `codeSize` (in bytes) from SPIR-V words.",
                m.name
            );
            w!(
                out,
                "    pub fn {name}(mut self, value: &'a [u32]) -> Self {{ self.code_size = core::mem::size_of_val(value); self.{field} = value.as_ptr(); self }}"
            );
        } else if m.null_terminated && ty.base == "char" && is_const {
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    pub fn {name}(mut self, value: &'a CStr) -> Self {{ self.{field} = value.as_ptr(); self }}"
            );
        } else if m.len.is_some() && !opaque {
            // The length is an expression of other members; only the pointer is stored.
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    /// Sets `{}`. Vulkan reads `{}` elements; no count member is updated.",
                m.name,
                m.len.as_deref().unwrap_or_default()
            );
            w!(
                out,
                "    pub fn {name}(mut self, value: {reference} [{}]) -> Self {{ self.{field} = value.{as_ptr}(); self }}",
                model.base_ty(&ty.base, "'a")
            );
        } else if opaque {
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    /// Stores a raw pointer. The borrow checker cannot track its lifetime."
            );
            w!(
                out,
                "    pub fn {name}(mut self, value: {}) -> Self {{ self.{field} = value; self }}",
                model.member_ty(m, "'a")
            );
        } else {
            let convert = if is_const {
                "core::ptr::from_ref(value)"
            } else {
                "core::ptr::from_mut(value)"
            };
            reserve(emitted, &s.name, &name);
            w!(
                out,
                "    pub fn {name}(mut self, value: {reference} {}) -> Self {{ self.{field} = {convert}; self }}",
                model.base_ty(&ty.base, "'a")
            );
        }
        return;
    }

    if ty.depth() == 2
        && ty.pointers == [true, true]
        && let Some(count) = count
        && (!opaque || (ty.base == "char" && m.null_terminated))
    {
        let elem = if ty.base == "char" {
            assert!(
                m.null_terminated,
                "{}::{}: char** without NUL",
                s.name, m.name
            );
            "CStrPtr<'a>".to_string()
        } else {
            format!("&'a {}", model.base_ty(&ty.base, "'a"))
        };
        reserve(emitted, &s.name, &name);
        w!(
            out,
            "    /// Sets `{}` and `{}` from a slice.",
            m.name,
            count.name
        );
        w!(
            out,
            "    pub fn {name}(mut self, value: &'a [{elem}]) -> Self {{ {}self.{} = value.len() as _; self.{field} = value.as_ptr().cast(); self }}",
            shared_check(count),
            field_name(&count.name)
        );
        return;
    }

    // Shapes without a borrowed form.
    reserve(emitted, &s.name, &field);
    w!(
        out,
        "    /// Stores a raw pointer. The borrow checker cannot track its lifetime."
    );
    w!(
        out,
        "    pub fn {field}(mut self, value: {}) -> Self {{ self.{field} = value; self }}",
        model.member_ty(m, "'a")
    );
}

fn emit_bit_accessors(
    out: &mut String,
    model: &Model<'_>,
    storage: &str,
    bit: &Bit<'_>,
    emitted: &mut BTreeSet<String>,
) {
    let m = bit.member;
    let field = field_name(&m.name);
    let plain = field.trim_start_matches("r#");
    let mask = if bit.width == 32 {
        u32::MAX
    } else {
        (1u32 << bit.width) - 1
    };
    let shift = bit.shift;
    assert!(emitted.insert(field.clone()), "duplicate setter {field}");
    let (ty, to_raw) = if model.is_flags(&m.ty) {
        (type_name(model.canonical(&m.ty)), "value.as_raw()")
    } else {
        ("u32".to_string(), "value")
    };
    // `<< 0` would trip clippy::identity_op in the generated code.
    let (shl, shr) = if shift == 0 {
        (String::new(), String::new())
    } else {
        (format!(" << {shift}"), format!(" >> {shift}"))
    };
    let read = format!("(self.{storage}{shr}) & {mask:#x}");
    let read = if ty == "u32" {
        read
    } else {
        format!("{ty}::from_raw({read})")
    };
    w!(
        out,
        "    /// Sets the {}-bit field `{}`. Higher bits of `value` are dropped.",
        bit.width,
        m.name
    );
    w!(
        out,
        "    pub fn {field}(mut self, value: {ty}) -> Self {{ self.{storage} = (self.{storage} & !({mask:#x}{shl})) | (({to_raw} & {mask:#x}){shl}); self }}"
    );
    w!(
        out,
        "    /// Reads the {}-bit field `{}`.",
        bit.width,
        m.name
    );
    w!(out, "    pub fn get_{plain}(&self) -> {ty} {{ {read} }}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{counted, field, optional, registry, structure, tagged};
    use vulkan_registry::{Bitmask, Flags, Handle};

    fn squash(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    #[test]
    fn plain_struct_has_no_lifetime_and_derives_comparisons() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkExtent2D",
            vec![field("width", "uint32_t"), field("height", "uint32_t")],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("#[derive(Copy, Clone, PartialEq, Eq, Hash)]"));
        assert!(out.contains("pub struct Extent2D { pub width: u32, pub height: u32, }"));
        assert!(out.contains("impl Extent2D {"));
        assert!(!out.contains("_marker"));
        assert!(!out.contains("unsafe impl Send for Extent2D"));
    }

    #[test]
    fn slice_setters_borrow_for_the_struct_lifetime() {
        let mut ir = registry();
        ir.structs.push(tagged(
            "VkDeviceQueueCreateInfo",
            "VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO",
            vec![
                field("queueCount", "uint32_t"),
                counted("pQueuePriorities", "const float*", "queueCount"),
            ],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub struct DeviceQueueCreateInfo<'a> {"));
        assert!(out.contains("pub _marker: PhantomData<&'a ()>,"));
        assert!(out.contains(
            "pub fn queue_priorities(mut self, value: &'a [f32]) -> Self { self.queue_count = value.len() as _; self.p_queue_priorities = value.as_ptr(); self }"
        ));
        assert!(out.contains("s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,"));
        assert!(out.contains("unsafe impl TaggedStructure for DeviceQueueCreateInfo<'_> {"));
        assert!(out.contains("unsafe impl Send for DeviceQueueCreateInfo<'_> {}"));
        // No setter takes a raw typed pointer.
        assert!(!out.contains("value: *const f32"));
    }

    #[test]
    fn arrays_sharing_a_count_check_each_other() {
        let mut ir = registry();
        ir.handles.push(Handle {
            name: "VkSemaphore".to_string(),
            ..Default::default()
        });
        ir.structs.push(tagged(
            "VkSubmitInfo",
            "VK_STRUCTURE_TYPE_SUBMIT_INFO",
            vec![
                field("waitSemaphoreCount", "uint32_t"),
                counted(
                    "pWaitSemaphores",
                    "const VkSemaphore*",
                    "waitSemaphoreCount",
                ),
                counted("pWaitDstStageMask", "const uint32_t*", "waitSemaphoreCount"),
            ],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains(
            "pub fn wait_semaphores(mut self, value: &'a [Semaphore]) -> Self { if !self.p_wait_dst_stage_mask.is_null() { assert_eq!(self.wait_semaphore_count as usize, value.len(),"
        ));
        assert!(out.contains(
            "pub fn wait_dst_stage_mask(mut self, value: &'a [u32]) -> Self { if !self.p_wait_semaphores.is_null() { assert_eq!("
        ));
    }

    #[test]
    fn string_arrays_use_lifetime_carrying_pointers() {
        let mut ir = registry();
        let mut names = counted(
            "ppEnabledExtensionNames",
            "const char* const*",
            "enabledExtensionCount",
        );
        names.null_terminated = true;
        let mut name = field("pApplicationName", "const char*");
        name.null_terminated = true;
        ir.structs.push(tagged(
            "VkInstanceCreateInfo",
            "VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO",
            vec![field("enabledExtensionCount", "uint32_t"), names, name],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub pp_enabled_extension_names: *const *const c_char,"));
        assert!(out.contains(
            "pub fn enabled_extension_names(mut self, value: &'a [CStrPtr<'a>]) -> Self { self.enabled_extension_count = value.len() as _; self.pp_enabled_extension_names = value.as_ptr().cast(); self }"
        ));
        assert!(out.contains(
            "pub fn application_name(mut self, value: &'a CStr) -> Self { self.p_application_name = value.as_ptr(); self }"
        ));
    }

    #[test]
    fn pointer_arrays_borrow_their_elements() {
        let mut ir = registry();
        ir.structs
            .push(structure("VkGeometry", vec![field("pData", "const void*")]));
        ir.structs.push(tagged(
            "VkBuildInfo",
            "VK_STRUCTURE_TYPE_BUILD_INFO",
            vec![
                field("geometryCount", "uint32_t"),
                optional(counted("pGeometries", "const VkGeometry*", "geometryCount")),
                optional(counted(
                    "ppGeometries",
                    "const VkGeometry* const*",
                    "geometryCount,1",
                )),
            ],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub fn geometries(mut self, value: &'a [Geometry<'a>]) -> Self {"));
        assert!(
            out.contains(
                "pub fn geometries_ptrs(mut self, value: &'a [&'a Geometry<'a>]) -> Self {"
            )
        );
        // void* has no borrowed form.
        assert!(out.contains("pub fn data(mut self, value: *const c_void) -> Self {"));
    }

    #[test]
    fn opaque_platform_pointers_stay_raw() {
        let mut ir = registry();
        ir.structs.push(tagged(
            "VkWaylandSurfaceCreateInfoKHR",
            "VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR",
            vec![field("display", "struct wl_display*")],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub display: *mut wl_display,"));
        assert!(out.contains("pub fn display(mut self, value: *mut wl_display) -> Self {"));
        assert!(!out.contains("&'a mut wl_display"));
    }

    #[test]
    fn push_next_overwrites_instead_of_walking_the_chain() {
        let mut ir = registry();
        ir.structs
            .push(tagged("VkRoot", "VK_STRUCTURE_TYPE_ROOT", vec![]));
        let mut ext = tagged("VkExt", "VK_STRUCTURE_TYPE_EXT", vec![]);
        ext.extends = vec!["VkRoot".to_string()];
        ir.structs.push(ext);
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains(
            "pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self { self.p_next = push_chain(next, self.p_next); self }"
        ));
        assert!(out.contains("unsafe impl Extends<Root<'_>> for Ext<'_> {}"));
        assert!(!out.contains("while"));
    }

    #[test]
    fn bit_fields_are_packed_into_storage_units() {
        let mut ir = registry();
        ir.bitmasks.push(Bitmask {
            name: "VkGeometryInstanceFlagBitsKHR".to_string(),
            flags_name: "VkGeometryInstanceFlagsKHR".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.flags.push(Flags {
            name: "VkGeometryInstanceFlagsKHR".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        let bits = |name: &str, ty: &str, width: u32| {
            let mut f = field(name, ty);
            f.bit_field_width = Some(width);
            f
        };
        ir.structs.push(structure(
            "VkAccelerationStructureInstanceKHR",
            vec![
                bits("instanceCustomIndex", "uint32_t", 24),
                bits("mask", "uint32_t", 8),
                bits("instanceShaderBindingTableRecordOffset", "uint32_t", 24),
                bits("flags", "VkGeometryInstanceFlagsKHR", 8),
                field("accelerationStructureReference", "uint64_t"),
            ],
        ));
        let model = Model::new(&ir);
        let s = model
            .get_struct("VkAccelerationStructureInstanceKHR")
            .unwrap();
        let slots = slots(&model, s);
        assert_eq!(slots.len(), 3);
        let out = squash(&emit_structs(&model));
        assert!(out.contains("pub bitfield0: u32,"));
        assert!(out.contains("pub bitfield1: u32,"));
        assert!(out.contains(
            "pub fn mask(mut self, value: u32) -> Self { self.bitfield0 = (self.bitfield0 & !(0xff << 24)) | ((value & 0xff) << 24); self }"
        ));
        assert!(out.contains(
            "pub fn get_instance_custom_index(&self) -> u32 { (self.bitfield0) & 0xffffff }"
        ));
        assert!(out.contains(
            "pub fn flags(mut self, value: GeometryInstanceFlagsKHR) -> Self { self.bitfield1 = (self.bitfield1 & !(0xff << 24)) | ((value.as_raw() & 0xff) << 24); self }"
        ));
    }

    #[test]
    fn bit_fields_that_do_not_fit_start_a_new_unit() {
        let mut ir = registry();
        let bits = |name: &str, width: u32| {
            let mut f = field(name, "uint32_t");
            f.bit_field_width = Some(width);
            f
        };
        ir.structs.push(structure(
            "StdVideoFlags",
            vec![bits("a", 20), bits("b", 20), bits("c", 12)],
        ));
        let model = Model::new(&ir);
        let slots = slots(&model, model.get_struct("StdVideoFlags").unwrap());
        let layout = slots
            .iter()
            .map(|slot| match slot {
                Slot::Bits { bits, .. } => bits.iter().map(|b| (b.shift, b.width)).collect(),
                Slot::Plain(_) => Vec::new(),
            })
            .collect::<Vec<Vec<_>>>();
        assert_eq!(layout, vec![vec![(0, 20)], vec![(0, 20), (20, 12)]]);
    }

    #[test]
    fn fixed_arrays_get_typed_readers() {
        let mut ir = registry();
        let mut name = field("deviceName", "char");
        name.fixed_size_array = vec!["256".to_string()];
        let mut types = counted("memoryTypes", "uint32_t", "memoryTypeCount");
        types.fixed_size_array = vec!["32".to_string()];
        ir.structs.push(structure(
            "VkProps",
            vec![name, field("memoryTypeCount", "uint32_t"), types],
        ));
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub fn device_name_as_c_str(&self)"));
        assert!(out.contains("pub fn memory_types_as_slice(&self) -> &[u32] {"));
    }

    #[test]
    fn unions_with_pointers_borrow_in_constructors() {
        let mut ir = registry();
        let mut u = structure(
            "VkDeviceOrHostAddressConstKHR",
            vec![
                field("deviceAddress", "VkDeviceAddress"),
                field("hostAddress", "const void*"),
            ],
        );
        u.union = true;
        ir.structs.push(u);
        ir.structs
            .push(structure("VkImageInfo", vec![field("layout", "uint32_t")]));
        let mut d = structure(
            "VkDescriptorDataEXT",
            vec![field("pSampledImage", "const VkImageInfo*")],
        );
        d.union = true;
        ir.structs.push(d);
        let out = squash(&emit_structs(&Model::new(&ir)));
        assert!(out.contains("pub union DeviceOrHostAddressConstKHR<'a> {"));
        assert!(out.contains(
            "pub fn host_address(value: *const c_void) -> Self { Self { host_address: value } }"
        ));
        assert!(out.contains(
            "pub fn sampled_image(value: &'a ImageInfo) -> Self { Self { p_sampled_image: core::ptr::from_ref(value) } }"
        ));
    }
}
