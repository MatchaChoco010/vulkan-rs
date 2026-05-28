#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use vulkan_registry::{Command, Member, Param, Registry, Struct};

#[cfg(test)]
use vulkan_registry::{Bitmask, Constant, Enum, FlagField, Flags, Handle};

struct Context<'a> {
    structs: BTreeMap<&'a str, &'a Struct>,
    unions: BTreeSet<&'a str>,
    handles: BTreeSet<&'a str>,
    enums: BTreeSet<&'a str>,
    flags: BTreeSet<&'a str>,
    func_pointers: BTreeSet<&'a str>,
    aliases: BTreeMap<&'a str, &'a str>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let out_dir = match args.as_slice() {
        [_program, out_dir] => Path::new(out_dir).to_path_buf(),
        _ => {
            eprintln!("usage: vulkan-codegen <out-dir>");
            std::process::exit(2);
        }
    };

    fs::create_dir_all(&out_dir).expect("create generated dir");

    let ir = Registry::get();
    let ctx = Context::new(ir);
    write_file(&out_dir, "mod.rs", &emit_mod());
    write_file(&out_dir, "version.rs", &emit_version(ir));
    write_file(&out_dir, "constants.rs", &emit_constants(ir));
    write_file(&out_dir, "handles.rs", &emit_handles(ir));
    write_file(&out_dir, "enums.rs", &emit_enums(ir));
    write_file(&out_dir, "bitmasks.rs", &emit_bitmasks(ir, &ctx));
    write_file(&out_dir, "func_pointers.rs", &emit_func_pointers(ir, &ctx));
    write_file(&out_dir, "extensions.rs", &emit_extensions(ir));
    write_file(&out_dir, "structs.rs", &emit_structs(ir, &ctx));
    write_file(&out_dir, "commands.rs", &emit_commands(ir, &ctx));
}

impl<'a> Context<'a> {
    fn new(ir: &'a Registry) -> Self {
        let structs = ir
            .structs
            .iter()
            .map(|s| (s.name.as_str(), s))
            .collect::<BTreeMap<_, _>>();
        let unions = ir
            .structs
            .iter()
            .filter(|s| s.union)
            .map(|s| s.name.as_str())
            .collect::<BTreeSet<_>>();
        let handles = ir
            .handles
            .iter()
            .map(|h| h.name.as_str())
            .collect::<BTreeSet<_>>();
        let enums = ir
            .enums
            .iter()
            .map(|e| e.name.as_str())
            .collect::<BTreeSet<_>>();
        let flags = ir
            .flags
            .iter()
            .map(|f| f.name.as_str())
            .collect::<BTreeSet<_>>();
        let func_pointers = ir
            .func_pointers
            .iter()
            .map(|f| f.name.as_str())
            .collect::<BTreeSet<_>>();
        let mut aliases = BTreeMap::new();
        for h in &ir.handles {
            for alias in &h.aliases {
                aliases.insert(alias.as_str(), h.name.as_str());
            }
        }
        for e in &ir.enums {
            for alias in &e.aliases {
                aliases.insert(alias.as_str(), e.name.as_str());
            }
        }
        for b in &ir.bitmasks {
            for alias in &b.aliases {
                aliases.insert(alias.as_str(), b.name.as_str());
            }
        }
        for f in &ir.flags {
            for alias in &f.aliases {
                aliases.insert(alias.as_str(), f.name.as_str());
            }
        }
        for s in &ir.structs {
            for alias in &s.aliases {
                aliases.insert(alias.as_str(), s.name.as_str());
            }
        }

        Self {
            structs,
            unions,
            handles,
            enums,
            flags,
            func_pointers,
            aliases,
        }
    }

    fn canonical<'b>(&'b self, name: &'b str) -> &'b str {
        self.aliases.get(name).copied().unwrap_or(name)
    }

    fn is_struct(&self, name: &str) -> bool {
        self.structs.contains_key(self.canonical(name))
    }

    fn is_union(&self, name: &str) -> bool {
        self.unions.contains(self.canonical(name))
    }

    fn is_known_vulkan_type(&self, name: &str) -> bool {
        let name = self.canonical(name);
        self.structs.contains_key(name)
            || self.unions.contains(name)
            || self.handles.contains(name)
            || self.enums.contains(name)
            || self.flags.contains(name)
            || self.func_pointers.contains(name)
    }
}

fn write_file(out_dir: &Path, name: &str, body: &str) {
    fs::write(out_dir.join(name), body).unwrap_or_else(|e| panic!("write {name}: {e}"));
}

fn emit_mod() -> String {
    [
        "mod version;",
        "mod constants;",
        "mod handles;",
        "mod enums;",
        "mod bitmasks;",
        "mod func_pointers;",
        "mod extensions;",
        "mod structs;",
        "mod commands;",
        "",
        "pub use version::*;",
        "pub use constants::*;",
        "pub use handles::*;",
        "pub use enums::*;",
        "pub use bitmasks::*;",
        "pub use func_pointers::*;",
        "pub use extensions::*;",
        "pub use structs::*;",
        "pub use commands::*;",
        "",
    ]
    .join("\n")
}

fn emit_version(ir: &Registry) -> String {
    let parts = parse_header_version_complete(&ir.header_version_complete);
    format!(
        "pub const VULKAN_HEADER_VERSION: u32 = {};\n\
         pub const VULKAN_HEADER_VERSION_COMPLETE: u32 = crate::vk::make_api_version(0, {}, {}, {});\n\
         pub const VULKAN_HEADERS_TAG: &str = \"{}\";\n\
         pub const VULKAN_HEADERS_COMMIT: &str = \"{}\";\n",
        ir.header_version, parts.0, parts.1, parts.2, ir.headers_tag, ir.headers_commit
    )
}

fn parse_header_version_complete(value: &str) -> (u32, u32, u32) {
    let mut it = value.split('.').filter_map(|x| x.parse::<u32>().ok());
    (
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
        it.next().unwrap_or(0),
    )
}

fn emit_constants(ir: &Registry) -> String {
    let mut out = prelude();
    let array_len_constants = array_len_constants(ir);
    for c in &ir.constants {
        let ty = if matches!(c.rust_name.as_str(), "TRUE" | "FALSE") {
            "Bool32"
        } else if array_len_constants.contains(&c.rust_name) {
            "usize"
        } else {
            match c.ty.as_str() {
                "float" => "f32",
                "VkBool32" => "Bool32",
                "uint8_t" => "u8",
                "uint32_t" => "u32",
                "uint64_t" => "u64",
                other => panic!("unsupported Vulkan constant type {other} for {}", c.name),
            }
        };
        writeln!(
            out,
            "pub const {}: {ty} = {};",
            raw_ident(&c.rust_name),
            c.value
        )
        .unwrap();
    }
    out
}

fn array_len_constants(ir: &Registry) -> BTreeSet<String> {
    let constant_names = ir
        .constants
        .iter()
        .map(|c| c.name.as_str())
        .collect::<BTreeSet<_>>();
    let mut names = BTreeSet::new();
    for s in &ir.structs {
        for m in &s.members {
            for len in &m.fixed_size_array {
                if constant_names.contains(len.as_str()) {
                    names.insert(rust_const_name(len));
                }
            }
        }
    }
    names
}

fn emit_handles(ir: &Registry) -> String {
    let mut out = prelude();
    out.push_str("use core::ffi::c_void;\n\n");
    for h in &ir.handles {
        cfg(&mut out, h.protect.as_deref());
        writeln!(
            out,
            "#[repr(transparent)]\n#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]\npub struct {}(pub {});\n",
            h.rust_name,
            if h.dispatchable { "*mut c_void" } else { "u64" }
        )
        .unwrap();
        cfg(&mut out, h.protect.as_deref());
        writeln!(
            out,
            "impl {} {{\n    pub const fn null() -> Self {{ Self({}) }}\n    pub const fn is_null(self) -> bool {{ {} }}\n    pub const fn from_raw(value: {}) -> Self {{ Self(value) }}\n    pub const fn as_raw(self) -> {} {{ self.0 }}\n}}\n",
            h.rust_name,
            if h.dispatchable { "core::ptr::null_mut()" } else { "0" },
            if h.dispatchable { "self.0.is_null()" } else { "self.0 == 0" },
            if h.dispatchable { "*mut c_void" } else { "u64" },
            if h.dispatchable { "*mut c_void" } else { "u64" },
        )
        .unwrap();
        for alias in &h.aliases {
            cfg(&mut out, h.protect.as_deref());
            writeln!(
                out,
                "pub type {} = {};\n",
                rust_type_name(alias),
                h.rust_name
            )
            .unwrap();
        }
    }
    out
}

fn emit_enums(ir: &Registry) -> String {
    let mut out = prelude();
    for e in &ir.enums {
        let repr = if e.bit_width == 64 { "i64" } else { "i32" };
        cfg(&mut out, e.protect.as_deref());
        writeln!(
            out,
            "#[repr(transparent)]\n#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]\npub struct {}(pub {repr});\n",
            e.rust_name
        )
        .unwrap();
        cfg(&mut out, e.protect.as_deref());
        writeln!(
            out,
            "impl {} {{\n    pub const fn from_raw(value: {repr}) -> Self {{ Self(value) }}\n    pub const fn as_raw(self) -> {repr} {{ self.0 }}",
            e.rust_name
        )
        .unwrap();
        for f in &e.fields {
            cfg(&mut out, f.protect.as_deref());
            writeln!(
                out,
                "    pub const {}: Self = Self({});",
                raw_ident(&f.rust_name),
                integer_literal(&f.value, repr)
            )
            .unwrap();
        }
        out.push_str("}\n\n");
        if e.rust_name == "VkResult" {
            out.push_str(
                "impl VkResult {\n    pub const fn into_result(self) -> core::result::Result<(), Self> {\n        if self.0 >= 0 { Ok(()) } else { Err(self) }\n    }\n}\n\n",
            );
        }
        for alias in &e.aliases {
            cfg(&mut out, e.protect.as_deref());
            writeln!(
                out,
                "pub type {} = {};\n",
                rust_type_name(alias),
                e.rust_name
            )
            .unwrap();
        }
    }
    out
}

fn emit_bitmasks(ir: &Registry, _ctx: &Context<'_>) -> String {
    let bitmask_fields = ir
        .bitmasks
        .iter()
        .map(|b| (b.name.as_str(), b))
        .collect::<BTreeMap<_, _>>();
    let flags_for_bitmask = ir
        .flags
        .iter()
        .filter_map(|f| f.bitmask.as_deref().map(|bitmask| (bitmask, f)))
        .collect::<BTreeMap<_, _>>();

    let mut out = prelude();
    for b in &ir.bitmasks {
        let repr = if b.bit_width == 64 { "u64" } else { "u32" };
        cfg(&mut out, b.protect.as_deref());
        writeln!(
            out,
            "#[repr(transparent)]\n#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]\npub struct {}(pub {repr});\n",
            b.rust_name
        )
        .unwrap();
        cfg(&mut out, b.protect.as_deref());
        writeln!(
            out,
            "impl {} {{\n    pub const fn from_raw(value: {repr}) -> Self {{ Self(value) }}\n    pub const fn as_raw(self) -> {repr} {{ self.0 }}",
            b.rust_name
        )
        .unwrap();
        for field in &b.fields {
            cfg(&mut out, field.protect.as_deref());
            writeln!(
                out,
                "    pub const {}: Self = Self({});",
                raw_ident(&field.rust_name),
                integer_literal(&field.value, repr)
            )
            .unwrap();
        }
        out.push_str("}\n\n");
        if let Some(flags) = flags_for_bitmask.get(b.name.as_str()) {
            cfg(&mut out, flags.protect.as_deref());
            writeln!(
                out,
                "impl From<{}> for {} {{\n    fn from(value: {}) -> Self {{ Self(value.0) }}\n}}\n",
                b.rust_name, flags.rust_name, b.rust_name
            )
            .unwrap();
            cfg(&mut out, flags.protect.as_deref());
            writeln!(
                out,
                "impl core::ops::BitOr for {} {{\n    type Output = {};\n    fn bitor(self, rhs: Self) -> {} {{ {}(self.0 | rhs.0) }}\n}}\n",
                b.rust_name, flags.rust_name, flags.rust_name, flags.rust_name
            )
            .unwrap();
            cfg(&mut out, flags.protect.as_deref());
            writeln!(
                out,
                "impl core::ops::BitOr<{}> for {} {{\n    type Output = {};\n    fn bitor(self, rhs: {}) -> {} {{ {}(self.0 | rhs.0) }}\n}}\n",
                flags.rust_name,
                b.rust_name,
                flags.rust_name,
                flags.rust_name,
                flags.rust_name,
                flags.rust_name
            )
            .unwrap();
            cfg(&mut out, flags.protect.as_deref());
            writeln!(
                out,
                "impl core::ops::BitOr<{}> for {} {{\n    type Output = Self;\n    fn bitor(self, rhs: {}) -> Self {{ Self(self.0 | rhs.0) }}\n}}\n",
                b.rust_name, flags.rust_name, b.rust_name
            )
            .unwrap();
            cfg(&mut out, flags.protect.as_deref());
            writeln!(
                out,
                "impl core::ops::BitOrAssign<{}> for {} {{\n    fn bitor_assign(&mut self, rhs: {}) {{ self.0 |= rhs.0; }}\n}}\n",
                b.rust_name, flags.rust_name, b.rust_name
            )
            .unwrap();
        }
        for alias in &b.aliases {
            cfg(&mut out, b.protect.as_deref());
            writeln!(
                out,
                "pub type {} = {};\n",
                rust_type_name(alias),
                b.rust_name
            )
            .unwrap();
        }
    }

    for f in &ir.flags {
        let repr = if f.bit_width == 64 { "u64" } else { "u32" };
        cfg(&mut out, f.protect.as_deref());
        writeln!(
            out,
            "#[repr(transparent)]\n#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]\npub struct {}(pub {repr});\n",
            f.rust_name
        )
        .unwrap();
        cfg(&mut out, f.protect.as_deref());
        writeln!(
            out,
            "impl {} {{\n    pub const fn empty() -> Self {{ Self(0) }}\n    pub const fn from_raw(value: {repr}) -> Self {{ Self(value) }}\n    pub const fn as_raw(self) -> {repr} {{ self.0 }}",
            f.rust_name
        )
        .unwrap();
        if let Some(bitmask) = f
            .bitmask
            .as_deref()
            .and_then(|name| bitmask_fields.get(name))
        {
            for field in &bitmask.fields {
                cfg(&mut out, field.protect.as_deref());
                writeln!(
                    out,
                    "    pub const {}: Self = Self({});",
                    raw_ident(&field.rust_name),
                    integer_literal(&field.value, repr)
                )
                .unwrap();
            }
        }
        out.push_str(
            "    pub const fn is_empty(self) -> bool { self.0 == 0 }\n    pub const fn contains(self, rhs: Self) -> bool { (self.0 & rhs.0) == rhs.0 }\n}\n\n",
        );
        cfg(&mut out, f.protect.as_deref());
        writeln!(
            out,
            "impl core::ops::BitOr for {} {{\n    type Output = Self;\n    fn bitor(self, rhs: Self) -> Self {{ Self(self.0 | rhs.0) }}\n}}\n",
            f.rust_name
        )
        .unwrap();
        cfg(&mut out, f.protect.as_deref());
        writeln!(
            out,
            "impl core::ops::BitOrAssign for {} {{\n    fn bitor_assign(&mut self, rhs: Self) {{ self.0 |= rhs.0; }}\n}}\n",
            f.rust_name
        )
        .unwrap();
        cfg(&mut out, f.protect.as_deref());
        writeln!(
            out,
            "impl core::ops::BitAnd for {} {{\n    type Output = Self;\n    fn bitand(self, rhs: Self) -> Self {{ Self(self.0 & rhs.0) }}\n}}\n",
            f.rust_name
        )
        .unwrap();
        for alias in &f.aliases {
            cfg(&mut out, f.protect.as_deref());
            writeln!(
                out,
                "pub type {} = {};\n",
                rust_type_name(alias),
                f.rust_name
            )
            .unwrap();
        }
    }
    out
}

fn emit_func_pointers(ir: &Registry, ctx: &Context<'_>) -> String {
    let mut out = prelude();
    out.push_str("use core::ffi::{c_char, c_void};\n\n");
    out.push_str("pub type PFN_vkVoidFunction = Option<unsafe extern \"system\" fn()>;\n\n");
    for fp in &ir.func_pointers {
        cfg(&mut out, fp.protect.as_deref());
        write!(
            out,
            "pub type {} = Option<unsafe extern \"system\" fn(",
            fp.name
        )
        .unwrap();
        for (i, p) in fp.params.iter().enumerate() {
            if i != 0 {
                out.push_str(", ");
            }
            out.push_str(&param_ty(p, ctx));
        }
        writeln!(out, ") -> {}>;\n", return_ty(&fp.return_ty, ctx)).unwrap();
    }
    out
}

fn emit_extensions(ir: &Registry) -> String {
    let mut out = prelude();
    out.push_str("use core::ffi::CStr;\n\n");

    for ext in &ir.extensions {
        cfg(&mut out, ext.protect.as_deref());
        writeln!(
            out,
            "pub const {}: &CStr = c\"{}\";",
            ext.name_const, ext.name
        )
        .unwrap();
        cfg(&mut out, ext.protect.as_deref());
        writeln!(
            out,
            "pub const {}: u32 = {};\n",
            ext.spec_version_const, ext.spec_version
        )
        .unwrap();
    }

    out
}

fn emit_structs(ir: &Registry, ctx: &Context<'_>) -> String {
    let mut out = prelude();
    out.push_str("use core::ffi::{c_char, c_void};\nuse core::marker::PhantomData;\n\n");
    for s in &ir.structs {
        cfg(&mut out, s.protect.as_deref());
        if s.name == "VkAccelerationStructureInstanceKHR" {
            // This struct contains C bitfields in Vulkan headers. VulkanObject exposes the
            // flattened member names, but Rust has no repr(C) bitfield syntax, so emit the ABI
            // layout as packed 24:8 words and generate setters for the logical fields.
            emit_acceleration_structure_instance(&mut out);
            for alias in &s.aliases {
                cfg(&mut out, s.protect.as_deref());
                writeln!(
                    out,
                    "pub type {}<'a> = {}<'a>;\n",
                    rust_type_name(alias),
                    s.rust_name
                )
                .unwrap();
            }
            continue;
        }
        if s.union {
            writeln!(
                out,
                "#[repr(C)]\n#[derive(Copy, Clone)]\npub union {}<'a> {{",
                s.rust_name
            )
            .unwrap();
            for m in &s.members {
                writeln!(out, "    pub {}: {},", m.rust_name, member_ty(m, ctx)).unwrap();
            }
            out.push_str("    pub _marker: PhantomData<&'a ()>,\n");
            out.push_str("}\n\n");
            emit_union_impls(&mut out, s, ctx);
        } else {
            writeln!(
                out,
                "#[repr(C)]\n#[derive(Copy, Clone)]\npub struct {}<'a> {{",
                s.rust_name
            )
            .unwrap();
            for m in &s.members {
                writeln!(out, "    pub {}: {},", m.rust_name, member_ty(m, ctx)).unwrap();
            }
            out.push_str("    pub _marker: PhantomData<&'a ()>,\n}\n\n");
            emit_struct_impls(&mut out, s, ctx);
        }
        for alias in &s.aliases {
            cfg(&mut out, s.protect.as_deref());
            writeln!(
                out,
                "pub type {}<'a> = {}<'a>;\n",
                rust_type_name(alias),
                s.rust_name
            )
            .unwrap();
        }
    }

    for s in &ir.structs {
        if s.extends.is_empty() || s.union {
            continue;
        }
        for root in &s.extends {
            if let Some(root_struct) = ctx.structs.get(root.as_str()) {
                if root_struct.union {
                    continue;
                }
                cfg(
                    &mut out,
                    s.protect.as_deref().or(root_struct.protect.as_deref()),
                );
                writeln!(
                    out,
                    "unsafe impl<'a, 'root> crate::vk::Extends<{}<'root>> for {}<'a> {{}}\n",
                    root_struct.rust_name, s.rust_name
                )
                .unwrap();
            }
        }
    }
    out
}

fn emit_acceleration_structure_instance(out: &mut String) {
    out.push_str(
        "#[repr(C)]\n\
         #[derive(Copy, Clone)]\n\
         pub struct AccelerationStructureInstanceKHR<'a> {\n\
             pub transform: TransformMatrixKHR<'a>,\n\
             pub instance_custom_index_and_mask: crate::vk::Packed24_8,\n\
             pub instance_shader_binding_table_record_offset_and_flags: crate::vk::Packed24_8,\n\
             pub acceleration_structure_reference: u64,\n\
             pub _marker: PhantomData<&'a ()>,\n\
         }\n\n\
         impl Default for AccelerationStructureInstanceKHR<'_> {\n\
             fn default() -> Self {\n\
                 Self {\n\
                     transform: Default::default(),\n\
                     instance_custom_index_and_mask: Default::default(),\n\
                     instance_shader_binding_table_record_offset_and_flags: Default::default(),\n\
                     acceleration_structure_reference: 0,\n\
                     _marker: PhantomData,\n\
                 }\n\
             }\n\
         }\n\n\
         impl<'a> AccelerationStructureInstanceKHR<'a> {\n\
             pub fn transform(mut self, value: TransformMatrixKHR<'a>) -> Self {\n\
                 self.transform = value;\n\
                 self\n\
             }\n\
             pub fn instance_custom_index_and_mask(mut self, value: crate::vk::Packed24_8) -> Self {\n\
                 self.instance_custom_index_and_mask = value;\n\
                 self\n\
             }\n\
             pub fn instance_shader_binding_table_record_offset_and_flags(\n\
                 mut self,\n\
                 value: crate::vk::Packed24_8,\n\
             ) -> Self {\n\
                 self.instance_shader_binding_table_record_offset_and_flags = value;\n\
                 self\n\
             }\n\
             pub fn instance_custom_index(mut self, value: u32) -> Self {\n\
                 self.instance_custom_index_and_mask = crate::vk::Packed24_8::new(\n\
                     value,\n\
                     self.instance_custom_index_and_mask.high_8(),\n\
                 );\n\
                 self\n\
             }\n\
             pub fn mask(mut self, value: u8) -> Self {\n\
                 self.instance_custom_index_and_mask = crate::vk::Packed24_8::new(\n\
                     self.instance_custom_index_and_mask.low_24(),\n\
                     value,\n\
                 );\n\
                 self\n\
             }\n\
             pub fn instance_shader_binding_table_record_offset(mut self, value: u32) -> Self {\n\
                 self.instance_shader_binding_table_record_offset_and_flags = crate::vk::Packed24_8::new(\n\
                     value,\n\
                     self.instance_shader_binding_table_record_offset_and_flags.high_8(),\n\
                 );\n\
                 self\n\
             }\n\
             pub fn flags(mut self, value: GeometryInstanceFlagsKHR) -> Self {\n\
                 self.instance_shader_binding_table_record_offset_and_flags = crate::vk::Packed24_8::new(\n\
                     self.instance_shader_binding_table_record_offset_and_flags.low_24(),\n\
                     value.0 as u8,\n\
                 );\n\
                 self\n\
             }\n\
             pub fn acceleration_structure_reference(mut self, value: u64) -> Self {\n\
                 self.acceleration_structure_reference = value;\n\
                 self\n\
             }\n\
         }\n\n",
    );
}

fn emit_union_impls(out: &mut String, s: &Struct, ctx: &Context<'_>) {
    cfg(out, s.protect.as_deref());
    writeln!(
        out,
        "impl Default for {}<'_> {{\n    fn default() -> Self {{\n        unsafe {{ core::mem::zeroed() }}\n    }}\n}}\n",
        s.rust_name
    )
    .unwrap();
    cfg(out, s.protect.as_deref());
    writeln!(out, "impl<'a> {}<'a> {{", s.rust_name).unwrap();
    let mut emitted = BTreeSet::new();
    for m in &s.members {
        let setter = setter_name(m);
        if !emitted.insert(setter.clone()) {
            continue;
        }
        if m.pointer && !m.ty.starts_with("PFN_") && m.fixed_size_array.is_empty() {
            let elem = pointed_ty(m, ctx);
            if should_use_raw_pointer_setter(m, ctx) {
                writeln!(
                    out,
                    "    pub fn {setter}(value: {}) -> Self {{\n        Self {{ {}: value }}\n    }}",
                    member_ty(m, ctx),
                    m.rust_name
                )
                .unwrap();
            } else {
                let ref_ty = if m.is_const {
                    format!("&'a {}", elem)
                } else {
                    format!("&'a mut {}", elem)
                };
                let cast = if m.is_const { "*const _" } else { "*mut _" };
                writeln!(
                    out,
                    "    pub fn {setter}(value: {ref_ty}) -> Self {{\n        Self {{ {}: value as {cast} }}\n    }}",
                    m.rust_name
                )
                .unwrap();
            }
        } else {
            writeln!(
                out,
                "    pub fn {setter}(value: {}) -> Self {{\n        Self {{ {}: value }}\n    }}",
                member_ty(m, ctx),
                m.rust_name
            )
            .unwrap();
        }
    }
    out.push_str("}\n\n");
}

fn should_use_raw_pointer_setter(m: &Member, ctx: &Context<'_>) -> bool {
    m.pointer
        && m.ty != "void"
        && m.ty != "char"
        && !m.ty.starts_with("PFN_")
        && !ctx.is_known_vulkan_type(&m.ty)
}

fn bool32_setter_value(m: &Member) -> Option<&'static str> {
    if !m.pointer && m.ty == "VkBool32" {
        Some("bool")
    } else {
        None
    }
}

fn bool32_assign_expr(field: &str) -> String {
    format!("        self.{field} = value as crate::vk::Bool32;")
}

fn emit_struct_impls(out: &mut String, s: &Struct, ctx: &Context<'_>) {
    cfg(out, s.protect.as_deref());
    writeln!(
        out,
        "impl Default for {}<'_> {{\n    fn default() -> Self {{\n        Self {{",
        s.rust_name
    )
    .unwrap();
    for m in &s.members {
        writeln!(
            out,
            "            {}: {},",
            m.rust_name,
            member_default_expr(s, m, ctx)
        )
        .unwrap();
    }
    out.push_str("            _marker: PhantomData,\n        }\n    }\n}\n\n");

    cfg(out, s.protect.as_deref());
    writeln!(out, "impl<'a> {}<'a> {{", s.rust_name).unwrap();
    let mut emitted_setters = BTreeSet::new();
    for m in &s.members {
        if m.name == "sType" || m.name == "pNext" {
            continue;
        }
        let setter = setter_name(m);
        if setter != m.rust_name && m.pointer {
            writeln!(
                out,
                "    pub fn {}(mut self, value: {}) -> Self {{\n        self.{} = value;\n        self\n    }}",
                m.rust_name,
                member_ty(m, ctx),
                m.rust_name
            )
            .unwrap();
        }
        if let Some(value_ty) = bool32_setter_value(m) {
            if !emitted_setters.insert(setter.clone()) {
                continue;
            }
            writeln!(
                out,
                "    pub fn {setter}(mut self, value: {value_ty}) -> Self {{\n{}\n        self\n    }}",
                bool32_assign_expr(&m.rust_name)
            )
            .unwrap();
            continue;
        }
        if m.fixed_size_array.is_empty() {
            if m.name == "pCode" && m.ty == "uint32_t" && m.len.as_deref() == Some("codeSize / 4") {
                if !emitted_setters.insert(setter.clone()) {
                    continue;
                }
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: &'a [u32]) -> Self {{\n        self.code_size = core::mem::size_of_val(value);\n        self.{} = value.as_ptr();\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                continue;
            }
            if m.name == "pCode" && m.ty == "void" && m.len.as_deref() == Some("codeSize") {
                if !emitted_setters.insert(setter.clone()) {
                    continue;
                }
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: &'a [u8]) -> Self {{\n        self.code_size = value.len();\n        self.{} = value.as_ptr().cast();\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                continue;
            }
            if m.ty == "void"
                && m.is_const
                && let Some(len_field) = simple_len_field(m.len.as_deref())
            {
                if !emitted_setters.insert(setter.clone()) {
                    continue;
                }
                let count_field = rust_field_name(len_field);
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: &'a [u8]) -> Self {{\n        self.{count_field} = value.len() as _;\n        self.{} = value.as_ptr().cast();\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                continue;
            }
            if m.ty == "void"
                && !m.is_const
                && let Some(len_field) = simple_len_field(m.len.as_deref())
            {
                if !emitted_setters.insert(setter.clone()) {
                    continue;
                }
                let count_field = rust_field_name(len_field);
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: &'a mut [u8]) -> Self {{\n        self.{count_field} = value.len() as _;\n        self.{} = value.as_mut_ptr().cast();\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                continue;
            }
            if let Some(len_field) = simple_len_field(m.len.as_deref()) {
                if !emitted_setters.insert(setter.clone()) {
                    continue;
                }
                let elem = pointed_ty(m, ctx);
                let count_field = rust_field_name(len_field);
                if m.optional || m.optional_pointer {
                    let ptr_expr = if m.is_const {
                        "value.as_ptr()"
                    } else {
                        "value.as_mut_ptr()"
                    };
                    let slice_ty = if m.is_const {
                        format!("&'a [{}]", elem)
                    } else {
                        format!("&'a mut [{}]", elem)
                    };
                    writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: {slice_ty}) -> Self {{\n        self.{count_field} = value.len() as _;\n        self.{} = {ptr_expr};\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                } else {
                    let slice_ty = if m.is_const {
                        format!("&'a [{}]", elem)
                    } else {
                        format!("&'a mut [{}]", elem)
                    };
                    let ptr_expr = if m.is_const {
                        "value.as_ptr()"
                    } else {
                        "value.as_mut_ptr()"
                    };
                    writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: {slice_ty}) -> Self {{\n        self.{count_field} = value.len() as _;\n        self.{} = {ptr_expr};\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
                }
                continue;
            }
        }
        if m.pointer && !m.ty.starts_with("PFN_") && m.fixed_size_array.is_empty() {
            if !emitted_setters.insert(setter.clone()) {
                continue;
            }
            if should_use_raw_pointer_setter(m, ctx) {
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: {}) -> Self {{\n        self.{} = value;\n        self\n    }}",
                    member_ty(m, ctx),
                    m.rust_name
                )
                .unwrap();
            } else if m.null_terminated && m.ty == "char" {
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: &'a core::ffi::CStr) -> Self {{\n        self.{} = value.as_ptr();\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
            } else {
                let elem = pointed_ty(m, ctx);
                let ref_ty = if m.is_const {
                    format!("&'a {}", elem)
                } else {
                    format!("&'a mut {}", elem)
                };
                let cast = if m.is_const { "*const _" } else { "*mut _" };
                writeln!(
                    out,
                    "    pub fn {setter}(mut self, value: {ref_ty}) -> Self {{\n        self.{} = value as {cast};\n        self\n    }}",
                    m.rust_name
                )
                .unwrap();
            }
        } else {
            if !emitted_setters.insert(setter.clone()) {
                continue;
            }
            writeln!(
                out,
                "    pub fn {setter}(mut self, value: {}) -> Self {{\n        self.{} = value;\n        self\n    }}",
                member_ty(m, ctx),
                m.rust_name
            )
            .unwrap();
        }
    }
    if s.members.iter().any(|m| m.name == "pNext") {
        let previous_next = s
            .members
            .iter()
            .find(|m| m.name == "pNext")
            .filter(|m| !m.is_const && m.ty == "void")
            .map_or("self.p_next as *mut c_void", |_| "self.p_next");
        writeln!(
            out,
        "    pub fn push_next<T>(mut self, next: &'a mut T) -> Self\n    where\n        T: crate::vk::Extends<Self> + crate::vk::TaggedStructure + ?Sized,\n    {{\n        unsafe {{\n            let next_ptr = next as *mut T as *mut crate::vk::PNextOutStructure;\n            let mut tail = next_ptr;\n            while !(*tail).p_next.is_null() {{\n                tail = (*tail).p_next.cast();\n            }}\n            (*tail).p_next = {previous_next};\n            self.p_next = next_ptr.cast();\n        }}\n        self\n    }}"
        )
        .unwrap();
    }
    out.push_str("}\n\n");

    if s.members.iter().any(|m| m.name == "pNext")
        && let Some(s_type) = &s.s_type
    {
        cfg(out, s.protect.as_deref());
        writeln!(
            out,
            "unsafe impl crate::vk::TaggedStructure for {}<'_> {{\n    const STRUCTURE_TYPE: StructureType = StructureType::{};\n    unsafe fn set_p_next(&mut self, next: *mut c_void) {{\n        self.p_next = next as _;\n    }}\n}}\n",
            s.rust_name,
            enum_value_const_name("VkStructureType", s_type)
        )
        .unwrap();
    }
}

fn emit_commands(ir: &Registry, ctx: &Context<'_>) -> String {
    let mut out = prelude_with_clippy_lints(&["missing_safety_doc", "too_many_arguments"]);
    out.push_str("use core::ffi::{c_char, c_void};\n\n");
    emit_fn_table(
        &mut out,
        "EntryFn",
        ir.commands.iter().filter(|c| c.dispatch == "entry"),
    );
    emit_fn_table(
        &mut out,
        "InstanceFn",
        ir.commands.iter().filter(|c| c.dispatch == "instance"),
    );
    emit_fn_table(
        &mut out,
        "DeviceFn",
        ir.commands.iter().filter(|c| c.dispatch == "device"),
    );
    emit_raw_wrappers(&mut out, ir, ctx);
    emit_wrappers(&mut out, ir, ctx);
    out
}

fn emit_fn_table<'a>(
    out: &mut String,
    name: &str,
    commands: impl Iterator<Item = &'a Command> + Clone,
) {
    writeln!(out, "#[derive(Copy, Clone)]\npub struct {name} {{").unwrap();
    for c in commands.clone() {
        cfg(out, c.protect.as_deref());
        writeln!(out, "    pub {}: PFN_{},", c.rust_name, c.name).unwrap();
    }
    out.push_str("}\n\n");

    if name == "EntryFn" {
        out.push_str("impl EntryFn {\n    pub unsafe fn load(get_instance_proc_addr: PFN_vkGetInstanceProcAddr) -> Self {\n        let get = get_instance_proc_addr.expect(\"vkGetInstanceProcAddr is required\");\n        Self {\n");
        for c in commands.clone() {
            if c.name == "vkGetInstanceProcAddr" {
                out.push_str("            get_instance_proc_addr,\n");
            } else {
                cfg(out, c.protect.as_deref());
                writeln!(
                    out,
                    "            {}: crate::load_pfn(get(Instance::null(), crate::c_name_ptr(b\"{}\\0\"))),",
                    c.rust_name, c.name
                )
                .unwrap();
            }
        }
        out.push_str("        }\n    }\n}\n\n");
    } else if name == "InstanceFn" {
        out.push_str("impl InstanceFn {\n    pub unsafe fn load(get_instance_proc_addr: PFN_vkGetInstanceProcAddr, instance: Instance) -> Self {\n        let get = get_instance_proc_addr.expect(\"vkGetInstanceProcAddr is required\");\n        Self {\n");
        for c in commands.clone() {
            cfg(out, c.protect.as_deref());
            writeln!(
                out,
                "            {}: crate::load_pfn(get(instance, crate::c_name_ptr(b\"{}\\0\"))),",
                c.rust_name, c.name
            )
            .unwrap();
        }
        out.push_str("        }\n    }\n}\n\n");
    } else {
        out.push_str("impl DeviceFn {\n    pub unsafe fn load(get_device_proc_addr: PFN_vkGetDeviceProcAddr, device: Device) -> Self {\n        let get = get_device_proc_addr.expect(\"vkGetDeviceProcAddr is required\");\n        Self {\n");
        for c in commands.clone() {
            cfg(out, c.protect.as_deref());
            writeln!(
                out,
                "            {}: crate::load_pfn(get(device, crate::c_name_ptr(b\"{}\\0\"))),",
                c.rust_name, c.name
            )
            .unwrap();
        }
        out.push_str("        }\n    }\n}\n\n");
    }
}

fn emit_raw_wrappers(out: &mut String, ir: &Registry, ctx: &Context<'_>) {
    for dispatch in ["entry", "instance", "device"] {
        let self_ty = match dispatch {
            "entry" => "crate::Entry",
            "instance" => "crate::Instance",
            _ => "crate::Device",
        };
        writeln!(out, "impl {self_ty} {{").unwrap();
        for c in ir.commands.iter().filter(|c| c.dispatch == dispatch) {
            cfg(out, c.protect.as_deref());
            let wrapper_handle = match dispatch {
                "instance" => Some("VkInstance"),
                "device" => Some("VkDevice"),
                _ => None,
            };
            let consumes_self_handle = wrapper_handle
                .zip(c.params.first())
                .is_some_and(|(handle, param)| param.ty == handle);
            let params = if consumes_self_handle {
                &c.params[1..]
            } else {
                &c.params[..]
            };
            write!(out, "    pub unsafe fn {}_raw(&self", c.rust_name).unwrap();
            for p in params {
                write!(out, ", {}: {}", p.rust_name, param_ty(p, ctx)).unwrap();
            }
            if c.return_ty == "void" {
                out.push_str(") {\n");
            } else {
                writeln!(out, ") -> {} {{", return_ty(&c.return_ty, ctx)).unwrap();
            }
            write!(
                out,
                "        (self.fp().{}.expect(\"{} is not loaded\"))(",
                c.rust_name, c.name
            )
            .unwrap();
            let mut first = true;
            if consumes_self_handle {
                out.push_str("self.handle()");
                first = false;
            }
            for p in params {
                if !first {
                    out.push_str(", ");
                }
                first = false;
                out.push_str(&p.rust_name);
            }
            out.push_str(")\n    }\n");
        }
        out.push_str("}\n\n");
    }
}

fn emit_wrappers(out: &mut String, ir: &Registry, ctx: &Context<'_>) {
    for dispatch in ["entry", "instance", "device"] {
        let self_ty = match dispatch {
            "entry" => "crate::Entry",
            "instance" => "crate::Instance",
            _ => "crate::Device",
        };
        writeln!(out, "impl {self_ty} {{").unwrap();
        for c in ir.commands.iter().filter(|c| c.dispatch == dispatch) {
            if matches!(
                c.name.as_str(),
                "vkCreateInstance" | "vkCreateDevice" | "vkDestroyInstance" | "vkDestroyDevice"
            ) || !supports_high_wrapper(c, dispatch, ctx)
            {
                continue;
            }
            emit_command_wrapper(out, c, dispatch, ctx);
        }
        out.push_str("}\n\n");
    }
}

fn emit_command_wrapper(out: &mut String, c: &Command, dispatch: &str, ctx: &Context<'_>) {
    let params = command_params_after_self(c, dispatch);
    // Acceleration-structure build commands use `ppBuildRangeInfos`: an array of pointers where
    // each pointed-to slice length is taken from the corresponding build-info geometry count.
    // The generic pointer/count recognizers only handle one slice per count parameter, so these
    // commands need a small shape-specific wrapper.
    if matches!(
        c.name.as_str(),
        "vkBuildAccelerationStructuresKHR" | "vkCmdBuildAccelerationStructuresKHR"
    ) {
        emit_build_acceleration_structures_wrapper(out, c);
    } else if c.name == "vkCmdBuildAccelerationStructuresIndirectKHR" {
        emit_cmd_build_acceleration_structures_indirect_wrapper(out, c);
    } else if let Some((count_i, data_i)) = enumeration_output_pair(params) {
        emit_enumeration_wrapper(out, c, params, count_i, data_i, ctx);
    } else if let Some((count_i, input_i, output_i)) = counted_output_pair(params) {
        emit_counted_output_wrapper(out, c, params, count_i, input_i, output_i, ctx);
    } else if let Some((info_i, field, output_i)) = struct_field_counted_output_pair(params) {
        emit_struct_field_counted_output_wrapper(out, c, params, info_i, &field, output_i, ctx);
    } else if let Some(output_i) = single_output_param(params) {
        emit_single_output_wrapper(out, c, params, output_i, ctx);
    } else {
        emit_plain_wrapper(out, c, params, ctx);
    }
}

fn supports_high_wrapper(c: &Command, dispatch: &str, ctx: &Context<'_>) -> bool {
    let params = command_params_after_self(c, dispatch);
    // The acceleration-structure build wrappers are supported even though their nested
    // pointer-to-slice parameter shape is outside the generic wrapper recognizers.
    if matches!(
        c.name.as_str(),
        "vkBuildAccelerationStructuresKHR"
            | "vkCmdBuildAccelerationStructuresKHR"
            | "vkCmdBuildAccelerationStructuresIndirectKHR"
    ) {
        return true;
    }
    if c.return_ty != "VkResult" && c.return_ty != "void" {
        return single_output_param(params).is_none() && wrapper_args_supported(params, &[], ctx);
    }
    if let Some((count_i, data_i)) = enumeration_output_pair(params) {
        return supports_output_param(&params[data_i], ctx)
            && wrapper_args_supported(params, &[count_i, data_i], ctx);
    }
    if let Some((count_i, input_i, output_i)) = counted_output_pair(params) {
        return supports_output_param(&params[output_i], ctx)
            && wrapper_args_supported(params, &[count_i, input_i, output_i], ctx);
    }
    if let Some((_info_i, _field, output_i)) = struct_field_counted_output_pair(params) {
        return supports_output_param(&params[output_i], ctx)
            && wrapper_args_supported(params, &[output_i], ctx);
    }
    if let Some(output_i) = single_output_param(params) {
        if should_emit_in_place_output_wrapper(c, &params[output_i], ctx) {
            return wrapper_args_supported(params, &[], ctx);
        }
        return supports_output_param(&params[output_i], ctx)
            && wrapper_args_supported(params, &[output_i], ctx);
    }
    wrapper_args_supported(params, &[], ctx)
}

fn emit_build_acceleration_structures_wrapper(out: &mut String, c: &Command) {
    cfg(out, c.protect.as_deref());
    out.push_str("    #[cfg(feature = \"alloc\")]\n");
    match c.name.as_str() {
        "vkBuildAccelerationStructuresKHR" => out.push_str(
            "    pub unsafe fn build_acceleration_structures_khr(\n\
                 &self,\n\
                 deferred_operation: DeferredOperationKHR,\n\
                 infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],\n\
                 build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR<'_>]],\n\
             ) -> crate::vk::Result<()> {\n",
        ),
        "vkCmdBuildAccelerationStructuresKHR" => out.push_str(
            "    pub unsafe fn cmd_build_acceleration_structures_khr(\n\
                 &self,\n\
                 command_buffer: CommandBuffer,\n\
                 infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],\n\
                 build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR<'_>]],\n\
             ) {\n",
        ),
        _ => unreachable!(),
    }
    out.push_str(
        "        assert_eq!(\n\
             infos.len(),\n\
             build_range_infos.len(),\n\
             \"infos and build_range_infos must have matching info_count lengths\"\n\
         );\n\
         for (info, ranges) in infos.iter().zip(build_range_infos.iter()) {\n\
             assert_eq!(\n\
                 ranges.len(),\n\
                 info.geometry_count as usize,\n\
                 \"each build_range_infos entry must match the corresponding geometry_count\"\n\
             );\n\
         }\n\
         let build_range_info_ptrs = build_range_infos\n\
             .iter()\n\
             .map(|ranges| ranges.as_ptr())\n\
             .collect::<alloc::vec::Vec<_>>();\n",
    );
    match c.name.as_str() {
        "vkBuildAccelerationStructuresKHR" => out.push_str(
            "        self.build_acceleration_structures_khr_raw(\n\
                 deferred_operation,\n\
                 infos.len() as u32,\n\
                 infos.as_ptr(),\n\
                 build_range_info_ptrs.as_ptr(),\n\
             )\n\
             .into_result()\n\
         }\n",
        ),
        "vkCmdBuildAccelerationStructuresKHR" => out.push_str(
            "        self.cmd_build_acceleration_structures_khr_raw(\n\
                 command_buffer,\n\
                 infos.len() as u32,\n\
                 infos.as_ptr(),\n\
                 build_range_info_ptrs.as_ptr(),\n\
             )\n\
         }\n",
        ),
        _ => unreachable!(),
    }
}

fn emit_cmd_build_acceleration_structures_indirect_wrapper(out: &mut String, c: &Command) {
    cfg(out, c.protect.as_deref());
    out.push_str(
        "    #[cfg(feature = \"alloc\")]\n\
         pub unsafe fn cmd_build_acceleration_structures_indirect_khr(\n\
             &self,\n\
             command_buffer: CommandBuffer,\n\
             infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],\n\
             indirect_device_addresses: &[u64],\n\
             indirect_strides: &[u32],\n\
             max_primitive_counts: &[&[u32]],\n\
         ) {\n\
             assert_eq!(\n\
                 infos.len(),\n\
                 indirect_device_addresses.len(),\n\
                 \"infos and indirect_device_addresses must have matching info_count lengths\"\n\
             );\n\
             assert_eq!(\n\
                 infos.len(),\n\
                 indirect_strides.len(),\n\
                 \"infos and indirect_strides must have matching info_count lengths\"\n\
             );\n\
             assert_eq!(\n\
                 infos.len(),\n\
                 max_primitive_counts.len(),\n\
                 \"infos and max_primitive_counts must have matching info_count lengths\"\n\
             );\n\
             for (info, counts) in infos.iter().zip(max_primitive_counts.iter()) {\n\
                 assert_eq!(\n\
                     counts.len(),\n\
                     info.geometry_count as usize,\n\
                     \"each max_primitive_counts entry must match the corresponding geometry_count\"\n\
                 );\n\
             }\n\
             let max_primitive_count_ptrs = max_primitive_counts\n\
                 .iter()\n\
                 .map(|counts| counts.as_ptr())\n\
                 .collect::<alloc::vec::Vec<_>>();\n\
             self.cmd_build_acceleration_structures_indirect_khr_raw(\n\
                 command_buffer,\n\
                 infos.len() as u32,\n\
                 infos.as_ptr(),\n\
                 indirect_device_addresses.as_ptr(),\n\
                 indirect_strides.as_ptr(),\n\
                 max_primitive_count_ptrs.as_ptr(),\n\
             )\n\
         }\n",
    );
}

fn emit_plain_wrapper(out: &mut String, c: &Command, params: &[Param], ctx: &Context<'_>) {
    cfg(out, c.protect.as_deref());
    let args = wrapper_args(params, &BTreeSet::new(), ctx);
    let ret = if c.return_ty == "VkResult" {
        if is_suboptimal_result_command(c) {
            "crate::vk::Result<bool>".to_string()
        } else if has_non_success_success_code(c) {
            "crate::vk::Result<VkResult>".to_string()
        } else {
            "crate::vk::Result<()>".to_string()
        }
    } else {
        return_ty(&c.return_ty, ctx)
    };
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    write_fn_return(out, &ret);
    emit_shared_count_len_checks(out, params);
    emit_struct_len_checks(out, params);
    if c.return_ty == "VkResult"
        && (has_non_success_success_code(c) || is_suboptimal_result_command(c))
    {
        write!(out, "        let result = self.{}_raw(", c.rust_name).unwrap();
    } else {
        write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    }
    write_raw_args_for_params(out, params, &args, &BTreeMap::new());
    if c.return_ty == "VkResult" {
        if is_suboptimal_result_command(c) {
            out.push_str(");\n        result.into_result()?;\n        Ok(result == VkResult::SUBOPTIMAL_KHR)\n");
        } else if has_non_success_success_code(c) {
            out.push_str(");\n        result.into_result()?;\n        Ok(result)\n");
        } else {
            out.push_str(").into_result()\n");
        }
    } else {
        out.push_str(")\n");
    }
    out.push_str("    }\n");
}

fn write_fn_return(out: &mut String, ret: &str) {
    if ret == "()" {
        out.push_str(") {\n");
    } else {
        writeln!(out, ") -> {ret} {{").unwrap();
    }
}

fn emit_shared_count_len_checks(out: &mut String, params: &[Param]) {
    let mut groups = BTreeMap::<String, Vec<&Param>>::new();
    for p in params {
        if p.pointer
            && p.fixed_size_array.is_empty()
            && let Some(len) = simple_len_field(p.len.as_deref())
        {
            groups.entry(len.to_string()).or_default().push(p);
        }
    }
    for (count, group) in groups {
        if group.len() < 2 {
            continue;
        }
        let Some(count_param) = params.iter().find(|p| p.name == count && !p.pointer) else {
            continue;
        };
        let expected = pointer_arg_name(group[0]);
        let expected_len = wrapper_slice_len_expr(group[0]);
        for actual_param in group.iter().skip(1) {
            let actual = pointer_arg_name(actual_param);
            let actual_len = wrapper_slice_len_expr(actual_param);
            writeln!(
                out,
                "        assert_eq!({expected_len}, {actual_len}, \"{} and {} must have matching {} lengths\");",
                expected, actual, count_param.rust_name
            )
            .unwrap();
        }
    }
}

fn emit_struct_len_checks(out: &mut String, params: &[Param]) {
    for p in params {
        if !p.pointer || !p.fixed_size_array.is_empty() {
            continue;
        }
        let Some((param_name, field_name)) = struct_len_expr(p.len.as_deref()) else {
            continue;
        };
        let Some(owner) = params.iter().find(|candidate| candidate.name == param_name) else {
            continue;
        };
        let arg_name = pointer_arg_name(p);
        let owner_name = wrapper_param_name(owner);
        let field_name = rust_field_name(field_name);
        if p.optional || p.optional_pointer {
            writeln!(
                out,
                "        if let Some(value) = {arg_name} {{ assert_eq!(value.len(), {owner_name}.{field_name} as usize, \"{arg_name} length must match {owner_name}.{field_name}\"); }}"
            )
            .unwrap();
        } else {
            writeln!(
                out,
                "        assert_eq!({arg_name}.len(), {owner_name}.{field_name} as usize, \"{arg_name} length must match {owner_name}.{field_name}\");"
            )
            .unwrap();
        }
    }
}

fn wrapper_slice_len_expr(p: &Param) -> String {
    let name = pointer_arg_name(p);
    if p.optional || p.optional_pointer {
        format!("{name}.map_or(0, |x| x.len())")
    } else {
        format!("{name}.len()")
    }
}

fn emit_single_output_wrapper(
    out: &mut String,
    c: &Command,
    params: &[Param],
    output_i: usize,
    ctx: &Context<'_>,
) {
    cfg(out, c.protect.as_deref());
    if should_emit_in_place_output_wrapper(c, &params[output_i], ctx) {
        emit_in_place_output_wrapper(out, c, params, ctx);
        return;
    }
    let mut skip = BTreeSet::new();
    skip.insert(output_i);
    let args = wrapper_args(params, &skip, ctx);
    let output_ty = output_param_ty(&params[output_i], ctx);
    let storage_ty = output_storage_ty(&params[output_i], &output_ty);
    let ret = if c.return_ty == "VkResult" {
        if is_acquire_next_image_command(c) {
            format!("crate::vk::Result<Option<({output_ty}, bool)>>")
        } else if is_suboptimal_result_command(c) {
            format!("crate::vk::Result<({output_ty}, bool)>")
        } else if has_non_success_success_code(c) {
            format!("crate::vk::Result<({output_ty}, VkResult)>")
        } else {
            format!("crate::vk::Result<{output_ty}>")
        }
    } else {
        output_ty.clone()
    };
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    write_fn_return(out, &ret);
    emit_struct_len_checks(out, params);
    writeln!(
        out,
        "        let mut value = core::mem::MaybeUninit::<{storage_ty}>::zeroed();"
    )
    .unwrap();
    if c.return_ty == "VkResult" {
        write!(out, "        let result = self.{}_raw(", c.rust_name).unwrap();
    } else {
        write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    }
    let mut extras = BTreeMap::new();
    extras.insert(output_i, "value.as_mut_ptr()".to_string());
    write_raw_args_for_params(out, params, &args, &extras);
    out.push(')');
    if c.return_ty == "VkResult" {
        if is_acquire_next_image_command(c) {
            out.push_str(
                ";\n        match result {\n            VkResult::SUCCESS => Ok(Some((value.assume_init(), false))),\n            VkResult::SUBOPTIMAL_KHR => Ok(Some((value.assume_init(), true))),\n            VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),\n            err => Err(err),\n        }\n",
            );
        } else {
            out.push_str(";\n        result.into_result()?;\n");
            if params[output_i].ty == "VkBool32" {
                if is_suboptimal_result_command(c) {
                    out.push_str(
                        "        Ok((value.assume_init() != 0, result == VkResult::SUBOPTIMAL_KHR))\n",
                    );
                } else if has_non_success_success_code(c) {
                    out.push_str("        Ok((value.assume_init() != 0, result))\n");
                } else {
                    out.push_str("        Ok(value.assume_init() != 0)\n");
                }
            } else if is_suboptimal_result_command(c) {
                out.push_str(
                    "        Ok((value.assume_init(), result == VkResult::SUBOPTIMAL_KHR))\n",
                );
            } else if has_non_success_success_code(c) {
                out.push_str("        Ok((value.assume_init(), result))\n");
            } else {
                out.push_str("        Ok(value.assume_init())\n");
            }
        }
    } else {
        if params[output_i].ty == "VkBool32" {
            out.push_str(";\n        value.assume_init() != 0\n");
        } else {
            out.push_str(";\n        value.assume_init()\n");
        }
    }
    out.push_str("    }\n");
}

fn should_emit_in_place_output_wrapper(c: &Command, output: &Param, ctx: &Context<'_>) -> bool {
    (c.return_ty == "void" || c.return_ty == "VkResult") && output_struct_has_p_next(output, ctx)
}

fn output_struct_has_p_next(output: &Param, ctx: &Context<'_>) -> bool {
    if !output.pointer || output.is_const {
        return false;
    }
    let name = ctx.canonical(&output.ty);
    let Some(s) = ctx.structs.get(name) else {
        return false;
    };
    s.members.iter().any(|m| m.name == "pNext")
}

fn emit_in_place_output_wrapper(
    out: &mut String,
    c: &Command,
    params: &[Param],
    ctx: &Context<'_>,
) {
    let args = wrapper_args(params, &BTreeSet::new(), ctx);
    let ret = if c.return_ty == "VkResult" {
        "crate::vk::Result<()>"
    } else {
        "()"
    };
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    write_fn_return(out, ret);
    emit_struct_len_checks(out, params);
    write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    write_raw_args_for_params(out, params, &args, &BTreeMap::new());
    if c.return_ty == "VkResult" {
        out.push_str(").into_result()\n");
    } else {
        out.push_str(")\n");
    }
    out.push_str("    }\n");
}

fn emit_enumeration_wrapper(
    out: &mut String,
    c: &Command,
    params: &[Param],
    count_i: usize,
    data_i: usize,
    ctx: &Context<'_>,
) {
    cfg(out, c.protect.as_deref());
    out.push_str("    #[cfg(feature = \"alloc\")]\n");
    let mut skip = BTreeSet::new();
    skip.insert(count_i);
    skip.insert(data_i);
    let args = wrapper_args(params, &skip, ctx);
    let elem_ty = if params[data_i].ty == "void" {
        "u8".to_string()
    } else {
        output_param_ty(&params[data_i], ctx)
    };
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    writeln!(out, ") -> crate::vk::Result<alloc::vec::Vec<{elem_ty}>> {{").unwrap();
    out.push_str("        let mut count = 0;\n");
    write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    let mut extras = BTreeMap::new();
    extras.insert(count_i, "&mut count".to_string());
    extras.insert(data_i, "core::ptr::null_mut()".to_string());
    write_raw_args_for_params(out, params, &args, &extras);
    out.push(')');
    if c.return_ty == "VkResult" {
        out.push_str(".into_result()?;\n");
    } else {
        out.push_str(";\n");
    }
    let count_usize = count_as_usize_expr(&params[count_i], "count", ctx);
    writeln!(
        out,
        "        let mut values: alloc::vec::Vec<{elem_ty}> = alloc::vec![{}; {count_usize}];",
        default_value_expr(&elem_ty, ctx)
    )
    .unwrap();
    if c.success_codes.iter().any(|x| x == "VK_INCOMPLETE") {
        out.push_str("        loop {\n");
        out.push_str("            count = values.len() as _;\n");
        write!(out, "            let result = self.{}_raw(", c.rust_name).unwrap();
        let mut extras = BTreeMap::new();
        extras.insert(count_i, "&mut count".to_string());
        extras.insert(
            data_i,
            if params[data_i].ty == "void" {
                "values.as_mut_ptr().cast()".to_string()
            } else {
                "values.as_mut_ptr()".to_string()
            },
        );
        write_raw_args_for_params(out, params, &args, &extras);
        out.push(')');
        if c.return_ty == "VkResult" {
            write!(
                out,
                ";\n            if result != VkResult::INCOMPLETE {{ result.into_result()?; break; }}\n            values.resize({count_usize}, "
            )
            .unwrap();
            out.push_str(&default_value_expr(&elem_ty, ctx));
            writeln!(
                out,
                ");\n            if values.len() <= {count_usize} {{\n                values.resize(({count_usize}).saturating_mul(2).max(1), "
            )
            .unwrap();
            out.push_str(&default_value_expr(&elem_ty, ctx));
            out.push_str(");\n            }\n        }\n");
        } else {
            out.push_str(";\n            break;\n        }\n");
        }
    } else {
        write!(out, "        self.{}_raw(", c.rust_name).unwrap();
        let mut extras = BTreeMap::new();
        extras.insert(count_i, "&mut count".to_string());
        extras.insert(
            data_i,
            if params[data_i].ty == "void" {
                "values.as_mut_ptr().cast()".to_string()
            } else {
                "values.as_mut_ptr()".to_string()
            },
        );
        write_raw_args_for_params(out, params, &args, &extras);
        out.push(')');
        if c.return_ty == "VkResult" {
            out.push_str(".into_result()?;\n");
        } else {
            out.push_str(";\n");
        }
    }
    writeln!(
        out,
        "        values.truncate({count_usize});\n        Ok(values)\n    }}"
    )
    .unwrap();

    if output_struct_has_p_next(&params[data_i], ctx) {
        let in_place_elem_ty = pointed_param_ty(&params[data_i], ctx);
        emit_enumeration_in_place_wrappers(
            out,
            c,
            params,
            (count_i, data_i),
            &args,
            &in_place_elem_ty,
            ctx,
        );
    }
}

fn emit_enumeration_in_place_wrappers(
    out: &mut String,
    c: &Command,
    params: &[Param],
    indices: (usize, usize),
    args: &[WrapperArg],
    elem_ty: &str,
    ctx: &Context<'_>,
) {
    let (count_i, data_i) = indices;
    cfg(out, c.protect.as_deref());
    let count_ret = if c.return_ty == "VkResult" {
        "crate::vk::Result<usize>"
    } else {
        "usize"
    };
    write!(out, "    pub unsafe fn {}_count(&self", c.rust_name).unwrap();
    for arg in args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    write_fn_return(out, count_ret);
    out.push_str("        let mut count = 0;\n");
    write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    let mut extras = BTreeMap::new();
    extras.insert(count_i, "&mut count".to_string());
    extras.insert(data_i, "core::ptr::null_mut()".to_string());
    write_raw_args_for_params(out, params, args, &extras);
    out.push(')');
    let count_usize = count_as_usize_expr(&params[count_i], "count", ctx);
    if c.return_ty == "VkResult" {
        writeln!(out, ".into_result()?;\n        Ok({count_usize})").unwrap();
    } else {
        writeln!(out, ";\n        {count_usize}").unwrap();
    }
    out.push_str("    }\n");

    cfg(out, c.protect.as_deref());
    let into_ret = if c.return_ty == "VkResult" {
        if c.success_codes.iter().any(|x| x == "VK_INCOMPLETE") {
            "crate::vk::Result<(usize, VkResult)>"
        } else {
            "crate::vk::Result<usize>"
        }
    } else {
        "usize"
    };
    write!(out, "    pub unsafe fn {}_into(&self", c.rust_name).unwrap();
    for arg in args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    out.push_str(", values: &mut [");
    out.push_str(elem_ty);
    out.push(']');
    write_fn_return(out, into_ret);
    out.push_str("        let mut count = values.len() as _;\n");
    if c.return_ty == "VkResult" {
        write!(out, "        let result = self.{}_raw(", c.rust_name).unwrap();
    } else {
        write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    }
    let mut extras = BTreeMap::new();
    extras.insert(count_i, "&mut count".to_string());
    extras.insert(data_i, "values.as_mut_ptr()".to_string());
    write_raw_args_for_params(out, params, args, &extras);
    out.push(')');
    let count_usize = count_as_usize_expr(&params[count_i], "count", ctx);
    if c.return_ty == "VkResult" {
        if c.success_codes.iter().any(|x| x == "VK_INCOMPLETE") {
            writeln!(
                out,
                ";\n        result.into_result()?;\n        Ok(({count_usize}, result))"
            )
            .unwrap();
        } else {
            writeln!(out, ".into_result()?;\n        Ok({count_usize})").unwrap();
        }
    } else {
        writeln!(out, ";\n        {count_usize}").unwrap();
    }
    out.push_str("    }\n");
}

fn count_as_usize_expr(count: &Param, name: &str, ctx: &Context<'_>) -> String {
    if return_ty(&count.ty, ctx) == "usize" {
        name.to_string()
    } else {
        format!("{name} as usize")
    }
}

fn emit_counted_output_wrapper(
    out: &mut String,
    c: &Command,
    params: &[Param],
    count_i: usize,
    input_i: usize,
    output_i: usize,
    ctx: &Context<'_>,
) {
    cfg(out, c.protect.as_deref());
    out.push_str("    #[cfg(feature = \"alloc\")]\n");
    let mut skip = BTreeSet::new();
    skip.insert(count_i);
    skip.insert(input_i);
    skip.insert(output_i);
    let mut args = wrapper_args(params, &skip, ctx);
    let input_name = pointer_arg_name(&params[input_i]);
    let input_ty = format!("&[{}]", pointed_param_ty(&params[input_i], ctx));
    let output_ty = output_param_ty(&params[output_i], ctx);
    let insert_at = count_i.min(args.len());
    args.insert(
        insert_at,
        WrapperArg {
            name: input_name.clone(),
            ty: input_ty,
            raw: format!("{input_name}.as_ptr()"),
        },
    );
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    if is_pipeline_creation_command(c) {
        // Pipeline creation commands may fail after writing some output handles. Return the
        // partially written vector with the error so callers can inspect/destroy any handles
        // produced by the driver instead of losing them.
        writeln!(
            out,
            ") -> core::result::Result<alloc::vec::Vec<{output_ty}>, (alloc::vec::Vec<{output_ty}>, VkResult)> {{"
        )
        .unwrap();
    } else {
        writeln!(
            out,
            ") -> crate::vk::Result<alloc::vec::Vec<{output_ty}>> {{"
        )
        .unwrap();
    }
    writeln!(
        out,
        "        let mut values = alloc::vec![{}; {input_name}.len()];",
        default_value_expr(&output_ty, ctx)
    )
    .unwrap();
    if is_pipeline_creation_command(c) {
        write!(out, "        let result = self.{}_raw(", c.rust_name).unwrap();
    } else {
        write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    }
    let mut raw = Vec::new();
    for (i, p) in params.iter().enumerate() {
        if i == count_i {
            raw.push(format!("{input_name}.len() as u32"));
        } else if i == input_i {
            raw.push(format!("{input_name}.as_ptr()"));
        } else if i == output_i {
            raw.push("values.as_mut_ptr()".to_string());
        } else if let Some(arg) = args.iter().find(|arg| arg.name == wrapper_param_name(p)) {
            raw.push(arg.raw.clone());
        }
    }
    out.push_str(&raw.join(", "));
    if is_pipeline_creation_command(c) {
        out.push_str(
            ");\n        if result == VkResult::SUCCESS {\n            Ok(values)\n        } else {\n            Err((values, result))\n        }\n    }\n",
        );
    } else {
        out.push_str(").into_result()?;\n        Ok(values)\n    }\n");
    }
}

fn has_non_success_success_code(c: &Command) -> bool {
    c.success_codes.iter().any(|code| code != "VK_SUCCESS")
}

fn is_pipeline_creation_command(c: &Command) -> bool {
    c.return_ty == "VkResult"
        && c.name.starts_with("vkCreate")
        && c.name.contains("Pipelines")
        && c.params
            .iter()
            .any(|p| p.ty == "VkPipeline" && p.pointer && !p.is_const)
}

fn is_acquire_next_image_command(c: &Command) -> bool {
    matches!(
        c.name.as_str(),
        "vkAcquireNextImageKHR" | "vkAcquireNextImage2KHR"
    ) && c
        .success_codes
        .iter()
        .any(|code| code == "VK_SUBOPTIMAL_KHR")
}

fn is_suboptimal_result_command(c: &Command) -> bool {
    c.name == "vkQueuePresentKHR"
        && c.success_codes
            .iter()
            .any(|code| code == "VK_SUBOPTIMAL_KHR")
}

fn emit_struct_field_counted_output_wrapper(
    out: &mut String,
    c: &Command,
    params: &[Param],
    _info_i: usize,
    field: &str,
    output_i: usize,
    ctx: &Context<'_>,
) {
    cfg(out, c.protect.as_deref());
    out.push_str("    #[cfg(feature = \"alloc\")]\n");
    let mut skip = BTreeSet::new();
    skip.insert(output_i);
    let args = wrapper_args(params, &skip, ctx);
    let output_ty = output_param_ty(&params[output_i], ctx);
    write!(out, "    pub unsafe fn {}(&self", c.rust_name).unwrap();
    for arg in &args {
        write!(out, ", {}: {}", arg.name, arg.ty).unwrap();
    }
    writeln!(
        out,
        ") -> crate::vk::Result<alloc::vec::Vec<{output_ty}>> {{"
    )
    .unwrap();
    let count_expr = format!("{}.{field} as usize", wrapper_param_name(&params[_info_i]));
    writeln!(
        out,
        "        let mut values = alloc::vec![{}; {count_expr}];",
        default_value_expr(&output_ty, ctx)
    )
    .unwrap();
    write!(out, "        self.{}_raw(", c.rust_name).unwrap();
    let mut extras = BTreeMap::new();
    extras.insert(output_i, "values.as_mut_ptr()".to_string());
    write_raw_args_for_params(out, params, &args, &extras);
    out.push_str(").into_result()?;\n        Ok(values)\n    }\n");
}

#[derive(Clone)]
struct WrapperArg {
    name: String,
    ty: String,
    raw: String,
}

fn wrapper_args(params: &[Param], skip: &BTreeSet<usize>, ctx: &Context<'_>) -> Vec<WrapperArg> {
    let mut args = Vec::new();
    let skipped_count_names = skip
        .iter()
        .filter_map(|&i| params.get(i))
        .filter_map(|p| simple_len_field(p.len.as_deref()))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    let slice_count_names = params
        .iter()
        .filter(|p| p.pointer)
        .filter_map(|p| simple_len_field(p.len.as_deref()))
        .map(str::to_string)
        .collect::<BTreeSet<_>>();
    for (i, p) in params.iter().enumerate() {
        if skip.contains(&i)
            || skipped_count_names.contains(&p.name)
            || (slice_count_names.contains(&p.name) && !p.pointer)
        {
            continue;
        }
        args.push(wrapper_arg(p, ctx));
    }
    args
}

fn wrapper_args_supported(params: &[Param], skip: &[usize], ctx: &Context<'_>) -> bool {
    let skip = skip.iter().copied().collect::<BTreeSet<_>>();
    for (i, p) in params.iter().enumerate() {
        if skip.contains(&i) {
            continue;
        }
        if p.pointer && p.full_ty.matches('*').count() > 1 {
            return false;
        }
        if p.pointer
            && !p.is_const
            && p.ty == "void"
            && p.len.is_none()
            && p.full_ty.matches('*').count() == 1
        {
            return false;
        }
    }
    let args = wrapper_args(params, &skip, ctx);
    let mut raw_args = String::new();
    write_raw_args_for_params(&mut raw_args, params, &args, &BTreeMap::new());
    !params.is_empty() || raw_args.is_empty()
}

fn wrapper_arg(p: &Param, ctx: &Context<'_>) -> WrapperArg {
    if !p.pointer && p.ty == "VkBool32" {
        let name = wrapper_param_name(p);
        return WrapperArg {
            name: name.clone(),
            ty: "bool".to_string(),
            raw: format!("{name} as u32"),
        };
    }
    if p.pointer && !p.ty.starts_with("PFN_") && p.fixed_size_array.is_empty() {
        let name = pointer_arg_name(p);
        if is_slice_len_expr(p.len.as_deref()) {
            let elem = if p.ty == "void" {
                "u8".to_string()
            } else {
                pointed_param_ty(p, ctx)
            };
            let ty = if p.optional || p.optional_pointer {
                if p.is_const {
                    format!("Option<&[{elem}]>")
                } else {
                    format!("Option<&mut [{elem}]>")
                }
            } else if p.is_const {
                format!("&[{elem}]")
            } else {
                format!("&mut [{elem}]")
            };
            let raw = if p.optional || p.optional_pointer {
                if p.is_const && p.ty == "void" {
                    format!("{name}.map_or(core::ptr::null(), |x| x.as_ptr().cast())")
                } else if p.is_const {
                    format!("{name}.map_or(core::ptr::null(), |x| x.as_ptr())")
                } else if p.ty == "void" {
                    format!(
                        "{name}.as_ref().map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut().cast())"
                    )
                } else {
                    format!(
                        "{name}.as_ref().map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut())"
                    )
                }
            } else if p.is_const && p.ty == "void" {
                format!("{name}.as_ptr().cast()")
            } else if p.is_const {
                format!("{name}.as_ptr()")
            } else if p.ty == "void" {
                format!("{name}.as_mut_ptr().cast()")
            } else {
                format!("{name}.as_mut_ptr()")
            };
            return WrapperArg { name, ty, raw };
        }
        if p.null_terminated && p.ty == "char" {
            let ty = if p.optional || p.optional_pointer {
                "Option<&core::ffi::CStr>".to_string()
            } else {
                "&core::ffi::CStr".to_string()
            };
            let raw = if p.optional || p.optional_pointer {
                format!("{name}.map_or(core::ptr::null(), |x| x.as_ptr())")
            } else {
                format!("{name}.as_ptr()")
            };
            return WrapperArg { name, ty, raw };
        }
        let elem = pointed_param_ty(p, ctx);
        let ty = if p.optional || p.optional_pointer {
            if p.is_const {
                format!("Option<&{elem}>")
            } else {
                format!("Option<&mut {elem}>")
            }
        } else if p.is_const {
            format!("&{elem}")
        } else {
            format!("&mut {elem}")
        };
        let raw = if p.optional || p.optional_pointer {
            let null = if p.is_const {
                "core::ptr::null()"
            } else {
                "core::ptr::null_mut()"
            };
            let cast = if p.is_const { "*const _" } else { "*mut _" };
            if p.is_const {
                format!("{name}.map_or({null}, |x| x as {cast})")
            } else {
                format!("{name}.as_ref().map_or({null}, |x| core::ptr::from_ref(&**x).cast_mut())")
            }
        } else {
            let cast = if p.is_const { "*const _" } else { "*mut _" };
            format!("{name} as {cast}")
        };
        return WrapperArg { name, ty, raw };
    }
    WrapperArg {
        name: wrapper_param_name(p),
        ty: param_ty(p, ctx),
        raw: wrapper_param_name(p),
    }
}

fn command_params_after_self<'a>(c: &'a Command, dispatch: &str) -> &'a [Param] {
    let wrapper_handle = match dispatch {
        "instance" => Some("VkInstance"),
        "device" => Some("VkDevice"),
        _ => None,
    };
    if wrapper_handle
        .zip(c.params.first())
        .is_some_and(|(handle, param)| param.ty == handle)
    {
        &c.params[1..]
    } else {
        &c.params[..]
    }
}

fn enumeration_output_pair(params: &[Param]) -> Option<(usize, usize)> {
    if params.len() < 2 {
        return None;
    }
    let count_i = params.len() - 2;
    let data_i = params.len() - 1;
    let count = &params[count_i];
    let data = &params[data_i];
    if count.pointer
        && !count.is_const
        && (count.ty == "uint32_t" || count.ty == "size_t")
        && data.pointer
        && !data.is_const
        && data.len.as_deref() == Some(count.name.as_str())
    {
        Some((count_i, data_i))
    } else {
        None
    }
}

fn counted_output_pair(params: &[Param]) -> Option<(usize, usize, usize)> {
    if params.len() < 3 {
        return None;
    }
    let output_i = params.len() - 1;
    let output = &params[output_i];
    if !output.pointer || output.is_const {
        return None;
    }
    let len = simple_len_field(output.len.as_deref())?;
    let count_i = params.iter().position(|p| p.name == len && !p.pointer)?;
    let input_i = params
        .iter()
        .position(|p| p.pointer && p.is_const && p.len.as_deref() == Some(len))?;
    Some((count_i, input_i, output_i))
}

fn struct_field_counted_output_pair(params: &[Param]) -> Option<(usize, String, usize)> {
    let output_i = params.len().checked_sub(1)?;
    let output = &params[output_i];
    if !output.pointer || output.is_const {
        return None;
    }
    let len = output.len.as_deref()?;
    let (param_name, field_name) = len.split_once("::").or_else(|| len.split_once("->"))?;
    let info_i = params.iter().position(|p| p.name == param_name)?;
    Some((info_i, rust_field_name(field_name), output_i))
}

fn single_output_param(params: &[Param]) -> Option<usize> {
    let i = params.len().checked_sub(1)?;
    let p = &params[i];
    if p.pointer
        && !p.is_const
        && !(p.ty == "void" && p.full_ty.matches('*').count() == 1)
        && p.len.is_none()
        && p.fixed_size_array.is_empty()
    {
        Some(i)
    } else {
        None
    }
}

fn write_raw_args_for_params(
    out: &mut String,
    params: &[Param],
    args: &[WrapperArg],
    extras: &BTreeMap<usize, String>,
) {
    let mut raw = Vec::new();
    for (i, p) in params.iter().enumerate() {
        if let Some(value) = extras.get(&i) {
            raw.push(value.clone());
            continue;
        }
        if let Some(slice_param) = params.iter().find(|candidate| {
            candidate.pointer && candidate.len.as_deref() == Some(p.name.as_str())
        }) && !p.pointer
        {
            let name = pointer_arg_name(slice_param);
            let count = if slice_param.optional || slice_param.optional_pointer {
                format!("{name}.map_or(0, |x| x.len() as _)")
            } else {
                format!("{name}.len() as _")
            };
            raw.push(count);
            continue;
        }
        if let Some(arg) = args.iter().find(|arg| arg.name == wrapper_param_name(p)) {
            raw.push(arg.raw.clone());
        }
    }
    out.push_str(&raw.join(", "));
}

fn pointer_arg_name(p: &Param) -> String {
    if p.name.starts_with("pp") && p.name.len() > 2 && p.name.as_bytes()[2].is_ascii_uppercase() {
        return rust_field_name(&p.name[2..]);
    }
    if p.name.starts_with('p') && p.name.len() > 1 && p.name.as_bytes()[1].is_ascii_uppercase() {
        return rust_field_name(&p.name[1..]);
    }
    p.rust_name.clone()
}

fn wrapper_param_name(p: &Param) -> String {
    if p.pointer {
        pointer_arg_name(p)
    } else {
        p.rust_name.clone()
    }
}

fn pointed_param_ty(p: &Param, ctx: &Context<'_>) -> String {
    let mut ty = rust_ty(&p.ty, ctx, true);
    let stars = p.full_ty.matches('*').count();
    for _ in 1..stars {
        ty = format!("*const {ty}");
    }
    ty
}

fn output_param_ty(p: &Param, ctx: &Context<'_>) -> String {
    if p.ty == "VkBool32" {
        return "bool".to_string();
    }
    let stars = p.full_ty.matches('*').count();
    if p.ty == "void" && stars > 1 {
        return "*mut c_void".to_string();
    }
    let name = ctx.canonical(&p.ty);
    if ctx.is_struct(name) {
        format!("{}<'static>", rust_type_name(name))
    } else {
        rust_ty(&p.ty, ctx, true)
    }
}

fn output_storage_ty(p: &Param, output_ty: &str) -> String {
    if p.ty == "VkBool32" {
        "u32".to_string()
    } else {
        output_ty.to_string()
    }
}

fn supports_output_param(p: &Param, _ctx: &Context<'_>) -> bool {
    if !p.pointer || p.is_const || !p.fixed_size_array.is_empty() {
        return false;
    }
    let stars = p.full_ty.matches('*').count();
    if stars == 1 {
        return true;
    }
    p.ty == "void" && stars == 2
}

fn default_value_expr(ty: &str, ctx: &Context<'_>) -> String {
    let vk_ty = format!("Vk{ty}");
    if ctx.handles.contains(ty) || ctx.handles.contains(vk_ty.as_str()) {
        format!("{ty}::null()")
    } else {
        "Default::default()".to_string()
    }
}

fn member_default_expr(s: &Struct, m: &Member, ctx: &Context<'_>) -> String {
    let mut expr = scalar_member_default_expr(s, m, ctx);
    for len in m.fixed_size_array.iter().rev() {
        expr = format!("[{expr}; {}]", array_len_expr(len));
    }
    expr
}

fn scalar_member_default_expr(s: &Struct, m: &Member, ctx: &Context<'_>) -> String {
    if m.name == "sType"
        && let Some(s_type) = &s.s_type
    {
        return format!(
            "StructureType::{}",
            enum_value_const_name("VkStructureType", s_type)
        );
    }
    if m.name == "pNext" {
        return if m.is_const {
            "core::ptr::null()".to_string()
        } else {
            "core::ptr::null_mut()".to_string()
        };
    }
    if matches!(
        m.name.as_str(),
        "srcQueueFamilyIndex" | "dstQueueFamilyIndex"
    ) {
        return "QUEUE_FAMILY_IGNORED".to_string();
    }
    if s.name == "VkImageCreateInfo" && matches!(m.name.as_str(), "mipLevels" | "arrayLayers") {
        return "1".to_string();
    }
    if m.ty.starts_with("PFN_") {
        return "None".to_string();
    }
    if m.pointer {
        return if m.is_const {
            "core::ptr::null()".to_string()
        } else {
            "core::ptr::null_mut()".to_string()
        };
    }
    let canonical = ctx.canonical(&m.ty);
    let rust = rust_ty(canonical, ctx, false);
    if ctx.handles.contains(canonical) {
        return format!("{rust}::null()");
    }
    if ctx.flags.contains(canonical) {
        return format!("{rust}::empty()");
    }
    if canonical == "VkSampleCountFlagBits" {
        return "SampleCountFlagBits::_1".to_string();
    }
    match canonical {
        "VkBool32" => "FALSE".to_string(),
        "float" | "double" => "0.0".to_string(),
        "char" | "int" | "uint8_t" | "int8_t" | "uint16_t" | "int16_t" | "uint32_t" | "int32_t"
        | "uint64_t" | "int64_t" | "size_t" | "VkFlags" | "VkFlags64" | "VkDeviceSize"
        | "VkDeviceAddress" | "VkSampleMask" => "0".to_string(),
        _ => "Default::default()".to_string(),
    }
}

fn member_ty(m: &Member, ctx: &Context<'_>) -> String {
    let base = rust_ty(&m.ty, ctx, false);
    let mut ty = if m.ty.starts_with("PFN_") {
        base
    } else if m.pointer {
        let ptr = if m.is_const { "*const" } else { "*mut" };
        if m.full_ty.matches('*').count() > 1 {
            let mut inner = base;
            for _ in 0..m.full_ty.matches('*').count() {
                inner = format!("{ptr} {inner}");
            }
            inner
        } else {
            format!("{ptr} {base}")
        }
    } else {
        base
    };
    for len in m.fixed_size_array.iter().rev() {
        ty = format!("[{}; {}]", ty, array_len_expr(len));
    }
    ty
}

fn param_ty(p: &Param, ctx: &Context<'_>) -> String {
    let base = rust_ty(&p.ty, ctx, true);
    let mut ty = if p.ty.starts_with("PFN_") {
        base
    } else if p.pointer {
        let stars = p.full_ty.matches('*').count().max(1);
        let mut inner = base;
        for _ in 0..stars {
            inner = format!("{} {}", if p.is_const { "*const" } else { "*mut" }, inner);
        }
        inner
    } else {
        base
    };
    for len in p.fixed_size_array.iter().rev() {
        ty = format!("[{}; {}]", ty, array_len_expr(len));
    }
    ty
}

fn pointed_ty(m: &Member, ctx: &Context<'_>) -> String {
    let mut ty = rust_ty(&m.ty, ctx, false);
    let stars = m.full_ty.matches('*').count();
    for _ in 1..stars {
        ty = format!("*const {ty}");
    }
    ty
}

fn return_ty(name: &str, ctx: &Context<'_>) -> String {
    if name == "void" {
        "()".to_string()
    } else if let Some(base) = name.strip_suffix('*') {
        format!("*mut {}", rust_ty(base.trim(), ctx, true))
    } else {
        rust_ty(name, ctx, true)
    }
}

fn rust_ty(name: &str, ctx: &Context<'_>, elided: bool) -> String {
    let name = ctx.canonical(name);
    match name {
        "void" => "c_void".to_string(),
        "char" => "c_char".to_string(),
        "float" => "f32".to_string(),
        "double" => "f64".to_string(),
        "int" => "core::ffi::c_int".to_string(),
        "uint8_t" => "u8".to_string(),
        "int8_t" => "i8".to_string(),
        "uint16_t" => "u16".to_string(),
        "int16_t" => "i16".to_string(),
        "uint32_t" => "u32".to_string(),
        "int32_t" => "i32".to_string(),
        "uint64_t" => "u64".to_string(),
        "int64_t" => "i64".to_string(),
        "size_t" => "usize".to_string(),
        "VkBool32" => "Bool32".to_string(),
        "VkFlags" => "u32".to_string(),
        "VkFlags64" => "u64".to_string(),
        "VkDeviceSize" => "u64".to_string(),
        "VkDeviceAddress" => "u64".to_string(),
        "VkRemoteAddressNV" => "RemoteAddressNV".to_string(),
        "VkSampleMask" => "u32".to_string(),
        _ if ctx.func_pointers.contains(name) => name.to_string(),
        _ if ctx.is_struct(name) => {
            format!(
                "{}<{}>",
                rust_type_name(name),
                if elided { "'_" } else { "'a" }
            )
        }
        _ => rust_type_name(name),
    }
}

fn array_len_expr(value: &str) -> String {
    if value.chars().all(|c| c.is_ascii_digit()) {
        value.to_string()
    } else {
        rust_const_name(value).to_string()
    }
}

fn simple_len_field(value: Option<&str>) -> Option<&str> {
    let value = value?;
    if value == "null-terminated"
        || value.contains("::")
        || value.contains(',')
        || value.contains('/')
        || value.contains('*')
        || value.contains('+')
        || value.contains('-')
    {
        None
    } else {
        Some(value)
    }
}

fn is_slice_len_expr(value: Option<&str>) -> bool {
    simple_len_field(value).is_some() || struct_len_expr(value).is_some()
}

fn struct_len_expr(value: Option<&str>) -> Option<(&str, &str)> {
    let value = value?;
    if value == "null-terminated"
        || value.contains(',')
        || value.contains('/')
        || value.contains('*')
        || value.contains('+')
    {
        return None;
    }
    value.split_once("->").or_else(|| value.split_once("::"))
}

fn setter_name(m: &Member) -> String {
    if m.name.starts_with("pp") && m.name.len() > 2 && m.name.as_bytes()[2].is_ascii_uppercase() {
        return rust_field_name(&m.name[2..]);
    }
    if m.name.starts_with('p') && m.name.len() > 1 && m.name.as_bytes()[1].is_ascii_uppercase() {
        rust_field_name(&m.name[1..])
    } else {
        m.rust_name.clone()
    }
}

fn prelude() -> String {
    prelude_with_clippy_lints(&[])
}

fn prelude_with_clippy_lints(lints: &[&str]) -> String {
    let mut out = "#![allow(dead_code)]\n#![allow(unused_imports)]\n#![allow(non_camel_case_types)]\n#![allow(non_snake_case)]\n#![allow(non_upper_case_globals)]\n#![allow(unsafe_op_in_unsafe_fn)]\n".to_string();
    for lint in lints {
        writeln!(out, "#![allow(clippy::{lint})]").unwrap();
    }
    out.push_str("\nuse crate::vk::*;\n\n");
    out
}

fn cfg(out: &mut String, protect: Option<&str>) {
    if let Some(feature) = protect.map(protect_feature) {
        writeln!(out, "#[cfg(feature = \"{feature}\")]").unwrap();
    }
}

fn protect_feature(protect: &str) -> &'static str {
    match protect {
        "VK_ENABLE_BETA_EXTENSIONS" => "beta",
        "VK_USE_PLATFORM_ANDROID_KHR" => "android",
        "VK_USE_PLATFORM_DIRECTFB_EXT" => "directfb",
        "VK_USE_PLATFORM_FUCHSIA" => "fuchsia",
        "VK_USE_PLATFORM_GGP" => "ggp",
        "VK_USE_PLATFORM_IOS_MVK" => "ios",
        "VK_USE_PLATFORM_MACOS_MVK" => "macos",
        "VK_USE_PLATFORM_METAL_EXT" => "metal",
        "VK_USE_PLATFORM_OHOS" => "ohos",
        "VK_USE_PLATFORM_SCREEN_QNX" => "screen",
        "VK_USE_PLATFORM_VI_NN" => "vi",
        "VK_USE_PLATFORM_WAYLAND_KHR" => "wayland",
        "VK_USE_PLATFORM_WIN32_KHR" => "win32",
        "VK_USE_PLATFORM_XCB_KHR" => "xcb",
        "VK_USE_PLATFORM_XLIB_KHR" => "xlib",
        "VK_USE_PLATFORM_XLIB_XRANDR_EXT" => "xlib-xrandr",
        "VK_USE_PLATFORM_UBM_SEC" => "ubm",
        "VK_USE_PLATFORM_SCI" => "sci",
        "VK_USE_PLATFORM_RDMA" => "rdma",
        _ => panic!("unhandled Vulkan protect macro: {protect}"),
    }
}

fn rust_type_name(name: &str) -> String {
    if name == "VkResult" {
        "VkResult".to_string()
    } else if let Some(rest) = name.strip_prefix("Vk") {
        rest.to_string()
    } else {
        name.to_string()
    }
}

fn rust_field_name(name: &str) -> String {
    raw_ident(&to_snake(name))
}

fn rust_const_name(name: &str) -> String {
    let name = name.strip_prefix("VK_").unwrap_or(name);
    raw_ident(&name.to_ascii_uppercase())
}

fn enum_value_const_name(parent: &str, value: &str) -> String {
    let mut name = value.strip_prefix("VK_").unwrap_or(value).to_string();
    let parent_prefix = to_screaming(&rust_type_name(parent));
    if let Some(rest) = name.strip_prefix(&(parent_prefix.clone() + "_")) {
        name = rest.to_string();
    }
    raw_ident(&name)
}

fn raw_ident(name: &str) -> String {
    if name.as_bytes().first().is_some_and(|b| b.is_ascii_digit()) {
        return format!("_{name}");
    }
    match name {
        "type" | "match" | "ref" | "box" | "loop" | "move" | "where" | "use" | "fn" | "mod"
        | "crate" | "self" | "super" | "in" | "as" | "pub" | "const" | "static" | "struct"
        | "enum" | "union" | "trait" | "impl" | "for" | "while" | "async" | "await" | "dyn" => {
            format!("r#{name}")
        }
        _ => name.to_string(),
    }
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    let chars = name.chars().collect::<Vec<_>>();
    for (i, ch) in chars.iter().copied().enumerate() {
        if ch.is_ascii_uppercase() {
            let prev = i.checked_sub(1).and_then(|i| chars.get(i)).copied();
            let next = chars.get(i + 1).copied();
            if i != 0
                && (prev.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    || next.is_some_and(|c| c.is_ascii_lowercase()))
            {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

fn to_screaming(name: &str) -> String {
    to_snake(name).to_ascii_uppercase()
}

fn integer_literal(value: &str, repr: &str) -> String {
    if value.starts_with('-') {
        format!("{value}{repr}")
    } else {
        let suffix = if repr.starts_with('u') { repr } else { "" };
        format!("{value}{suffix}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_registry() -> Registry {
        Registry {
            header_version: 350,
            header_version_complete: "1.4.350".to_string(),
            headers_tag: "vulkan-sdk-1.4.350.0".to_string(),
            headers_commit: "test".to_string(),
            constants: Vec::new(),
            handles: Vec::new(),
            enums: Vec::new(),
            bitmasks: Vec::new(),
            flags: Vec::new(),
            func_pointers: Vec::new(),
            structs: Vec::new(),
            commands: Vec::new(),
            extensions: Vec::new(),
        }
    }

    fn member(name: &str, ty: &str, full_ty: &str) -> Member {
        Member {
            name: name.to_string(),
            rust_name: rust_field_name(name),
            ty: ty.to_string(),
            full_ty: full_ty.to_string(),
            is_const: full_ty.trim_start().starts_with("const "),
            pointer: full_ty.contains('*'),
            optional: false,
            optional_pointer: false,
            len: None,
            null_terminated: false,
            fixed_size_array: Vec::new(),
        }
    }

    fn param(name: &str, ty: &str, full_ty: &str) -> Param {
        Param {
            name: name.to_string(),
            rust_name: rust_field_name(name),
            ty: ty.to_string(),
            full_ty: full_ty.to_string(),
            is_const: full_ty.trim_start().starts_with("const "),
            pointer: full_ty.contains('*'),
            optional: false,
            optional_pointer: false,
            len: None,
            null_terminated: false,
            fixed_size_array: Vec::new(),
        }
    }

    fn structure_type_enum() -> Enum {
        Enum {
            name: "VkStructureType".to_string(),
            rust_name: "StructureType".to_string(),
            aliases: Vec::new(),
            bit_width: 32,
            fields: Vec::new(),
            protect: None,
        }
    }

    fn bare_struct(name: &str) -> Struct {
        Struct {
            name: name.to_string(),
            rust_name: rust_type_name(name),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            members: Vec::new(),
            protect: None,
        }
    }

    fn add_device_and_result(ir: &mut Registry) {
        ir.enums.push(Enum {
            name: "VkResult".to_string(),
            rust_name: "VkResult".to_string(),
            aliases: Vec::new(),
            bit_width: 32,
            fields: Vec::new(),
            protect: None,
        });
        ir.handles.push(Handle {
            name: "VkDevice".to_string(),
            rust_name: "Device".to_string(),
            aliases: Vec::new(),
            dispatchable: true,
            protect: None,
        });
    }

    #[test]
    fn flag_bits_or_returns_flags_type() {
        let mut ir = empty_registry();
        ir.bitmasks.push(Bitmask {
            name: "VkMemoryPropertyFlagBits".to_string(),
            rust_name: "MemoryPropertyFlagBits".to_string(),
            aliases: Vec::new(),
            bit_width: 32,
            fields: vec![
                FlagField {
                    name: "VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT".to_string(),
                    rust_name: "HOST_VISIBLE".to_string(),
                    value: "1".to_string(),
                    protect: None,
                },
                FlagField {
                    name: "VK_MEMORY_PROPERTY_HOST_COHERENT_BIT".to_string(),
                    rust_name: "HOST_COHERENT".to_string(),
                    value: "2".to_string(),
                    protect: None,
                },
            ],
            protect: None,
        });
        ir.flags.push(Flags {
            name: "VkMemoryPropertyFlags".to_string(),
            rust_name: "MemoryPropertyFlags".to_string(),
            aliases: Vec::new(),
            bit_width: 32,
            bitmask: Some("VkMemoryPropertyFlagBits".to_string()),
            protect: None,
        });

        let out = emit_bitmasks(&ir, &Context::new(&ir));
        assert!(out.contains("impl core::ops::BitOr for MemoryPropertyFlagBits"));
        assert!(out.contains("type Output = MemoryPropertyFlags;"));
        assert!(
            out.contains("impl core::ops::BitOr<MemoryPropertyFlagBits> for MemoryPropertyFlags")
        );
    }

    #[test]
    fn emits_shader_module_code_slice_setter_from_complex_len_expr() {
        let mut ir = empty_registry();
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkShaderModuleCreateInfo".to_string(),
            rust_name: "ShaderModuleCreateInfo".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "const void*"),
                member(
                    "flags",
                    "VkShaderModuleCreateFlags",
                    "VkShaderModuleCreateFlags",
                ),
                member("codeSize", "size_t", "size_t"),
                {
                    let mut p_code = member("pCode", "uint32_t", "const uint32_t*");
                    p_code.len = Some("codeSize / 4".to_string());
                    p_code
                },
            ],
        });
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains("pub fn code(mut self, value: &'a [u32]) -> Self"));
        assert!(out.contains("self.code_size = core::mem::size_of_val(value);"));
        assert!(out.contains("self.p_code = value.as_ptr();"));
    }

    #[test]
    fn emits_void_data_slice_setter_as_bytes() {
        let mut ir = empty_registry();
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkWriteDescriptorSetInlineUniformBlock".to_string(),
            rust_name: "WriteDescriptorSetInlineUniformBlock".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "const void*"),
                member("dataSize", "uint32_t", "uint32_t"),
                {
                    let mut data = member("pData", "void", "const void*");
                    data.len = Some("dataSize".to_string());
                    data
                },
            ],
        });
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains("pub fn data(mut self, value: &'a [u8]) -> Self"));
        assert!(out.contains("self.data_size = value.len() as _;"));
        assert!(out.contains("self.p_data = value.as_ptr().cast();"));
    }

    #[test]
    fn emits_mut_void_range_setter_as_bytes() {
        let mut ir = empty_registry();
        let mut address = member("address", "void", "void*");
        address.len = Some("size".to_string());
        ir.structs.push(Struct {
            name: "VkHostAddressRangeEXT".to_string(),
            rust_name: "HostAddressRangeEXT".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            protect: None,
            members: vec![address, member("size", "size_t", "size_t")],
        });
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains("pub fn address(mut self, value: &'a mut [u8]) -> Self"));
        assert!(out.contains("self.size = value.len() as _;"));
        assert!(out.contains("self.address = value.as_mut_ptr().cast();"));
    }

    #[test]
    fn emits_constants_with_types_from_ir_and_array_len_usage() {
        let mut ir = empty_registry();
        ir.constants.push(Constant {
            name: "VK_QUEUE_FAMILY_IGNORED".to_string(),
            rust_name: "QUEUE_FAMILY_IGNORED".to_string(),
            value: "4294967295".to_string(),
            ty: "uint32_t".to_string(),
        });
        ir.constants.push(Constant {
            name: "VK_WHOLE_SIZE".to_string(),
            rust_name: "WHOLE_SIZE".to_string(),
            value: "18446744073709551615".to_string(),
            ty: "uint64_t".to_string(),
        });
        ir.constants.push(Constant {
            name: "VK_UUID_SIZE".to_string(),
            rust_name: "UUID_SIZE".to_string(),
            value: "16".to_string(),
            ty: "uint32_t".to_string(),
        });
        let mut uuid_member = member("uuid", "uint8_t", "uint8_t");
        uuid_member.fixed_size_array = vec!["VK_UUID_SIZE".to_string()];
        ir.structs.push(Struct {
            name: "VkPhysicalDeviceIDProperties".to_string(),
            rust_name: "PhysicalDeviceIDProperties".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            members: vec![uuid_member],
            protect: None,
        });
        let out = emit_constants(&ir);
        assert!(out.contains("pub const QUEUE_FAMILY_IGNORED: u32 = 4294967295;"));
        assert!(out.contains("pub const WHOLE_SIZE: u64 = 18446744073709551615;"));
        assert!(out.contains("pub const UUID_SIZE: usize = 16;"));
    }

    #[test]
    fn cfg_emits_known_platform_protects_and_panics_for_unknown_ones() {
        let mut out = String::new();
        cfg(&mut out, Some("VK_USE_PLATFORM_UBM_SEC"));
        cfg(&mut out, Some("VK_USE_PLATFORM_XLIB_XRANDR_EXT"));
        assert!(out.contains("#[cfg(feature = \"ubm\")]"));
        assert!(out.contains("#[cfg(feature = \"xlib-xrandr\")]"));

        let unknown = std::panic::catch_unwind(|| protect_feature("VK_USE_PLATFORM_NEW_THING"));
        assert!(unknown.is_err());
    }

    #[test]
    fn emits_union_pointer_constructors() {
        let mut ir = empty_registry();
        ir.structs.push(bare_struct("VkDeviceAddressRangeKHR"));
        ir.structs.push(Struct {
            name: "VkResourceDescriptorDataEXT".to_string(),
            rust_name: "ResourceDescriptorDataEXT".to_string(),
            aliases: Vec::new(),
            union: true,
            s_type: None,
            extends: Vec::new(),
            protect: None,
            members: vec![member(
                "pAddressRange",
                "VkDeviceAddressRangeKHR",
                "const VkDeviceAddressRangeKHR*",
            )],
        });
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains("pub fn address_range(value: &'a DeviceAddressRangeKHR<'a>) -> Self"));
        assert!(out.contains("Self { p_address_range: value as *const _ }"));
    }

    #[test]
    fn acceleration_structure_instance_has_bitfield_setters() {
        let mut ir = empty_registry();
        ir.structs.push(bare_struct("VkTransformMatrixKHR"));
        ir.structs
            .push(bare_struct("VkAccelerationStructureInstanceKHR"));
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains("pub fn instance_custom_index(mut self, value: u32) -> Self"));
        assert!(out.contains("pub fn mask(mut self, value: u8) -> Self"));
        assert!(
            out.contains(
                "pub fn instance_shader_binding_table_record_offset(mut self, value: u32)"
            )
        );
        assert!(out.contains("pub fn flags(mut self, value: GeometryInstanceFlagsKHR) -> Self"));
    }

    #[test]
    fn pnext_extends_impl_does_not_force_extension_and_root_lifetimes_to_match() {
        let mut ir = empty_registry();
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkPhysicalDeviceFeatures2".to_string(),
            rust_name: "PhysicalDeviceFeatures2".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "void*"),
            ],
        });
        ir.structs.push(Struct {
            name: "VkPhysicalDeviceVulkan12Features".to_string(),
            rust_name: "PhysicalDeviceVulkan12Features".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES".to_string()),
            extends: vec!["VkPhysicalDeviceFeatures2".to_string()],
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "void*"),
            ],
        });
        let ctx = Context::new(&ir);
        let out = emit_structs(&ir, &ctx);
        assert!(out.contains(
            "unsafe impl<'a, 'root> crate::vk::Extends<PhysicalDeviceFeatures2<'root>> for PhysicalDeviceVulkan12Features<'a>"
        ));
    }

    #[test]
    fn wrappers_check_lengths_for_multiple_slices_sharing_one_count() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.structs.push(bare_struct("VkResourceDescriptorInfoEXT"));
        ir.structs.push(bare_struct("VkHostAddressRangeEXT"));
        let mut resources = param(
            "pResources",
            "VkResourceDescriptorInfoEXT",
            "const VkResourceDescriptorInfoEXT*",
        );
        resources.len = Some("resourceCount".to_string());
        let mut descriptors = param(
            "pDescriptors",
            "VkHostAddressRangeEXT",
            "const VkHostAddressRangeEXT*",
        );
        descriptors.len = Some("resourceCount".to_string());
        ir.commands.push(Command {
            name: "vkWriteResourceDescriptorsEXT".to_string(),
            rust_name: "write_resource_descriptors_ext".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("resourceCount", "uint32_t", "uint32_t"),
                resources,
                descriptors,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("assert_eq!("));
        assert!(out.contains("resources.len()"));
        assert!(out.contains("descriptors.len()"));
        assert!(out.contains("matching resource_count lengths"));
    }

    #[test]
    fn skips_unsized_void_output_high_wrapper() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.commands.push(Command {
            name: "vkGetOpaqueDataEXT".to_string(),
            rust_name: "get_opaque_data_ext".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pData", "void", "void*"),
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn get_opaque_data_ext_raw"));
        assert!(!out.contains("pub unsafe fn get_opaque_data_ext(&self"));
    }

    #[test]
    fn emits_struct_field_len_slice_for_command_params() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkBuildInfo".to_string(),
            rust_name: "BuildInfo".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            protect: None,
            members: vec![member("geometryCount", "uint32_t", "uint32_t")],
        });
        ir.structs.push(Struct {
            name: "VkSizeInfo".to_string(),
            rust_name: "SizeInfo".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_SIZE_INFO".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "void*"),
            ],
        });
        let mut counts = param("pMaxPrimitiveCounts", "uint32_t", "const uint32_t*");
        counts.len = Some("pBuildInfo->geometryCount".to_string());
        ir.commands.push(Command {
            name: "vkGetBuildSizesKHR".to_string(),
            rust_name: "get_build_sizes_khr".to_string(),
            return_ty: "void".to_string(),
            success_codes: Vec::new(),
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pBuildInfo", "VkBuildInfo", "const VkBuildInfo*"),
                counts,
                param("pSizeInfo", "VkSizeInfo", "VkSizeInfo*"),
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("max_primitive_counts: &[u32]"));
        assert!(out.contains("max_primitive_counts.len()"));
        assert!(out.contains("build_info.geometry_count as usize"));
        assert!(out.contains("max_primitive_counts.as_ptr()"));
    }

    #[test]
    fn emits_acceleration_structure_build_wrappers_with_nested_len_checks() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.handles.push(Handle {
            name: "VkDeferredOperationKHR".to_string(),
            rust_name: "DeferredOperationKHR".to_string(),
            aliases: Vec::new(),
            dispatchable: false,
            protect: None,
        });
        ir.structs.push(Struct {
            name: "VkAccelerationStructureBuildGeometryInfoKHR".to_string(),
            rust_name: "AccelerationStructureBuildGeometryInfoKHR".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            protect: None,
            members: vec![member("geometryCount", "uint32_t", "uint32_t")],
        });
        ir.structs
            .push(bare_struct("VkAccelerationStructureBuildRangeInfoKHR"));
        let mut infos = param(
            "pInfos",
            "VkAccelerationStructureBuildGeometryInfoKHR",
            "const VkAccelerationStructureBuildGeometryInfoKHR*",
        );
        infos.len = Some("infoCount".to_string());
        let mut ranges = param(
            "ppBuildRangeInfos",
            "VkAccelerationStructureBuildRangeInfoKHR",
            "const VkAccelerationStructureBuildRangeInfoKHR* const*",
        );
        ranges.len = Some("infoCount".to_string());
        ir.commands.push(Command {
            name: "vkBuildAccelerationStructuresKHR".to_string(),
            rust_name: "build_acceleration_structures_khr".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param(
                    "deferredOperation",
                    "VkDeferredOperationKHR",
                    "VkDeferredOperationKHR",
                ),
                param("infoCount", "uint32_t", "uint32_t"),
                infos,
                ranges,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn build_acceleration_structures_khr("));
        assert!(
            out.contains("build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR<'_>]]")
        );
        assert!(out.contains("ranges.len()"));
        assert!(out.contains("info.geometry_count as usize"));
        assert!(out.contains(".into_result()"));
    }

    #[test]
    fn emits_pnext_enumeration_into_wrappers() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.handles.push(Handle {
            name: "VkPhysicalDevice".to_string(),
            rust_name: "PhysicalDevice".to_string(),
            aliases: Vec::new(),
            dispatchable: true,
            protect: None,
        });
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkQueueFamilyProperties2".to_string(),
            rust_name: "QueueFamilyProperties2".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "void*"),
            ],
        });
        let mut properties = param(
            "pQueueFamilyProperties",
            "VkQueueFamilyProperties2",
            "VkQueueFamilyProperties2*",
        );
        properties.len = Some("pQueueFamilyPropertyCount".to_string());
        ir.commands.push(Command {
            name: "vkGetPhysicalDeviceQueueFamilyProperties2".to_string(),
            rust_name: "get_physical_device_queue_family_properties2".to_string(),
            return_ty: "void".to_string(),
            success_codes: Vec::new(),
            error_codes: Vec::new(),
            params: vec![
                param("physicalDevice", "VkPhysicalDevice", "VkPhysicalDevice"),
                param("pQueueFamilyPropertyCount", "uint32_t", "uint32_t*"),
                properties,
            ],
            dispatch: "instance".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn get_physical_device_queue_family_properties2_count"));
        assert!(out.contains("pub unsafe fn get_physical_device_queue_family_properties2_into"));
        assert!(out.contains("values: &mut [QueueFamilyProperties2<'_>]"));
        assert!(out.contains("values.as_mut_ptr()"));
    }

    #[test]
    fn emits_acceleration_structure_indirect_wrapper_with_nested_len_checks() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.handles.push(Handle {
            name: "VkCommandBuffer".to_string(),
            rust_name: "CommandBuffer".to_string(),
            aliases: Vec::new(),
            dispatchable: true,
            protect: None,
        });
        ir.structs.push(Struct {
            name: "VkAccelerationStructureBuildGeometryInfoKHR".to_string(),
            rust_name: "AccelerationStructureBuildGeometryInfoKHR".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: None,
            extends: Vec::new(),
            protect: None,
            members: vec![member("geometryCount", "uint32_t", "uint32_t")],
        });
        let mut infos = param(
            "pInfos",
            "VkAccelerationStructureBuildGeometryInfoKHR",
            "const VkAccelerationStructureBuildGeometryInfoKHR*",
        );
        infos.len = Some("infoCount".to_string());
        let mut addresses = param(
            "pIndirectDeviceAddresses",
            "VkDeviceAddress",
            "const VkDeviceAddress*",
        );
        addresses.len = Some("infoCount".to_string());
        let mut strides = param("pIndirectStrides", "uint32_t", "const uint32_t*");
        strides.len = Some("infoCount".to_string());
        let mut counts = param("ppMaxPrimitiveCounts", "uint32_t", "const uint32_t* const*");
        counts.len = Some("infoCount".to_string());
        ir.commands.push(Command {
            name: "vkCmdBuildAccelerationStructuresIndirectKHR".to_string(),
            rust_name: "cmd_build_acceleration_structures_indirect_khr".to_string(),
            return_ty: "void".to_string(),
            success_codes: Vec::new(),
            error_codes: Vec::new(),
            params: vec![
                param("commandBuffer", "VkCommandBuffer", "VkCommandBuffer"),
                param("infoCount", "uint32_t", "uint32_t"),
                infos,
                addresses,
                strides,
                counts,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn cmd_build_acceleration_structures_indirect_khr("));
        assert!(out.contains("indirect_device_addresses: &[u64]"));
        assert!(out.contains("max_primitive_counts: &[&[u32]]"));
        assert!(out.contains("counts.len()"));
        assert!(out.contains("info.geometry_count as usize"));
        assert!(out.contains("max_primitive_count_ptrs.as_ptr()"));
    }

    #[test]
    fn emits_size_t_void_enumeration_as_vec_u8() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        let mut data = param("pData", "void", "void*");
        data.len = Some("pDataSize".to_string());
        ir.commands.push(Command {
            name: "vkGetCacheData".to_string(),
            rust_name: "get_cache_data".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pDataSize", "size_t", "size_t*"),
                data,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("-> crate::vk::Result<alloc::vec::Vec<u8>>"));
        assert!(out.contains("let mut values: alloc::vec::Vec<u8>"));
        assert!(out.contains("values.as_mut_ptr().cast()"));
    }

    #[test]
    fn incomplete_enumeration_loop_grows_when_count_does_not_increase() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.structs.push(bare_struct("VkExtensionProperties"));
        let mut data = param(
            "pProperties",
            "VkExtensionProperties",
            "VkExtensionProperties*",
        );
        data.len = Some("pPropertyCount".to_string());
        ir.commands.push(Command {
            name: "vkEnumerateDeviceExtensionProperties".to_string(),
            rust_name: "enumerate_device_extension_properties".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string(), "VK_INCOMPLETE".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pPropertyCount", "uint32_t", "uint32_t*"),
                data,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("result != VkResult::INCOMPLETE"));
        assert!(out.contains("count = values.len() as _;"));
        assert!(out.contains("values.len() <= count as usize"));
        assert!(out.contains("(count as usize).saturating_mul(2).max(1)"));
    }

    #[test]
    fn vk_result_pnext_output_uses_caller_storage() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.enums.push(structure_type_enum());
        ir.structs.push(Struct {
            name: "VkOutputProperties2".to_string(),
            rust_name: "OutputProperties2".to_string(),
            aliases: Vec::new(),
            union: false,
            s_type: Some("VK_STRUCTURE_TYPE_OUTPUT_PROPERTIES_2".to_string()),
            extends: Vec::new(),
            protect: None,
            members: vec![
                member("sType", "VkStructureType", "VkStructureType"),
                member("pNext", "void", "void*"),
            ],
        });
        ir.commands.push(Command {
            name: "vkGetOutputProperties2".to_string(),
            rust_name: "get_output_properties2".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec!["VK_SUCCESS".to_string()],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pProperties", "VkOutputProperties2", "VkOutputProperties2*"),
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("properties: &mut OutputProperties2<'_>"));
        assert!(out.contains("-> crate::vk::Result<()>"));
        assert!(out.contains("properties as *mut _"));
        assert!(!out.contains("MaybeUninit::<OutputProperties2<'static>>"));
    }

    #[test]
    fn acquire_next_image_does_not_read_output_for_not_ready_or_timeout() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        for name in ["VkSwapchainKHR", "VkSemaphore", "VkFence"] {
            ir.handles.push(Handle {
                name: name.to_string(),
                rust_name: rust_type_name(name),
                aliases: Vec::new(),
                dispatchable: false,
                protect: None,
            });
        }
        ir.commands.push(Command {
            name: "vkAcquireNextImageKHR".to_string(),
            rust_name: "acquire_next_image_khr".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec![
                "VK_SUCCESS".to_string(),
                "VK_TIMEOUT".to_string(),
                "VK_NOT_READY".to_string(),
                "VK_SUBOPTIMAL_KHR".to_string(),
            ],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("swapchain", "VkSwapchainKHR", "VkSwapchainKHR"),
                param("timeout", "uint64_t", "uint64_t"),
                param("semaphore", "VkSemaphore", "VkSemaphore"),
                param("fence", "VkFence", "VkFence"),
                param("pImageIndex", "uint32_t", "uint32_t*"),
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn acquire_next_image_khr("));
        assert!(out.contains("-> crate::vk::Result<Option<(u32, bool)>>"));
        assert!(out.contains("match result"));
        assert!(out.contains("VkResult::SUCCESS => Ok(Some((value.assume_init(), false)))"));
        assert!(out.contains("VkResult::SUBOPTIMAL_KHR => Ok(Some((value.assume_init(), true)))"));
        assert!(out.contains("VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None)"));
        assert!(out.contains("err => Err(err)"));
        let wrapper = out
            .split("pub unsafe fn acquire_next_image_khr(")
            .nth(1)
            .expect("wrapper should be emitted");
        assert!(!wrapper.contains("result.into_result()?;"));
    }

    #[test]
    fn all_pipeline_creation_wrappers_preserve_partial_outputs_on_non_success() {
        let mut ir = empty_registry();
        add_device_and_result(&mut ir);
        ir.handles.push(Handle {
            name: "VkPipelineCache".to_string(),
            rust_name: "PipelineCache".to_string(),
            aliases: Vec::new(),
            dispatchable: false,
            protect: None,
        });
        ir.handles.push(Handle {
            name: "VkPipeline".to_string(),
            rust_name: "Pipeline".to_string(),
            aliases: Vec::new(),
            dispatchable: false,
            protect: None,
        });
        ir.structs
            .push(bare_struct("VkRayTracingPipelineCreateInfoNV"));
        let mut infos = param(
            "pCreateInfos",
            "VkRayTracingPipelineCreateInfoNV",
            "const VkRayTracingPipelineCreateInfoNV*",
        );
        infos.len = Some("createInfoCount".to_string());
        let mut pipelines = param("pPipelines", "VkPipeline", "VkPipeline*");
        pipelines.len = Some("createInfoCount".to_string());
        ir.commands.push(Command {
            name: "vkCreateRayTracingPipelinesNV".to_string(),
            rust_name: "create_ray_tracing_pipelines_nv".to_string(),
            return_ty: "VkResult".to_string(),
            success_codes: vec![
                "VK_SUCCESS".to_string(),
                "VK_PIPELINE_COMPILE_REQUIRED_EXT".to_string(),
            ],
            error_codes: Vec::new(),
            params: vec![
                param("device", "VkDevice", "VkDevice"),
                param("pipelineCache", "VkPipelineCache", "VkPipelineCache"),
                param("createInfoCount", "uint32_t", "uint32_t"),
                infos,
                pipelines,
            ],
            dispatch: "device".to_string(),
            protect: None,
        });
        let ctx = Context::new(&ir);
        let out = emit_commands(&ir, &ctx);
        assert!(out.contains("pub unsafe fn create_ray_tracing_pipelines_nv("));
        assert!(out.contains(
            "-> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>"
        ));
        assert!(out.contains("let result = self.create_ray_tracing_pipelines_nv_raw("));
        assert!(out.contains("if result == VkResult::SUCCESS"));
        assert!(out.contains("Err((values, result))"));
        let wrapper = out
            .split("pub unsafe fn create_ray_tracing_pipelines_nv(")
            .nth(1)
            .expect("wrapper should be emitted");
        assert!(!wrapper.contains(".into_result()?;"));
    }
}
