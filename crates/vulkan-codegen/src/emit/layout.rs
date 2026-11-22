use super::structs::{Slot, slots};
use super::variants::{Variant, bitmask_variants, enum_variants};
use super::w;
use crate::model::{BETA_PROTECT, Cfg, Model};
use crate::naming::{const_name, field_name, type_name};
use vulkan_registry::Struct;

fn is_declared_for_the_c_program(protect: Option<&str>) -> bool {
    matches!(protect, None | Some(BETA_PROTECT))
}

fn checked_structs<'a>(model: &'a Model<'_>) -> impl Iterator<Item = &'a Struct> {
    model
        .ir
        .structs
        .iter()
        .filter(|s| is_declared_for_the_c_program(s.protect.as_deref()))
}

struct CheckedValue<'a> {
    c_name: &'a str,
    cfg: Cfg,
    rust_exprs: Vec<String>,
}

fn checked_values<'a>(model: &Model<'a>) -> Vec<CheckedValue<'a>> {
    let mut values = Vec::new();
    for c in model.ir.constants.iter().filter(|c| c.ty != "float") {
        values.push(CheckedValue {
            c_name: &c.name,
            cfg: Cfg::default(),
            rust_exprs: vec![format!("vk::{}", const_name(&c.name))],
        });
    }
    let mut push_variants =
        |protect: Option<&str>, variants: Vec<Variant<'a>>, rust_types: &[&str]| {
            if !is_declared_for_the_c_program(protect) {
                return;
            }
            for v in variants {
                if !is_declared_for_the_c_program(v.protect) {
                    continue;
                }
                values.push(CheckedValue {
                    c_name: v.c_name,
                    cfg: model.cfg(protect).or_else(|| v.cfg.clone()),
                    rust_exprs: rust_types
                        .iter()
                        .map(|ty| format!("vk::{}::{}.0", type_name(ty), v.name))
                        .collect(),
                });
            }
        };
    for e in &model.ir.enums {
        push_variants(e.protect.as_deref(), enum_variants(model, e), &[&e.name]);
    }
    for b in &model.ir.bitmasks {
        push_variants(
            b.protect.as_deref(),
            bitmask_variants(model, b),
            &[&b.name, &b.flags_name],
        );
    }
    values
}

pub(crate) fn emit_table(model: &Model<'_>) -> String {
    let mut struct_rows = String::new();
    let mut bit_field_rows = String::new();
    for s in checked_structs(model) {
        let cfg = model.cfg(s.protect.as_deref());
        let rust = if model.has_lifetime(&s.name) {
            format!("vk::{}<'static>", type_name(&s.name))
        } else {
            format!("vk::{}", type_name(&s.name))
        };
        let mut offsets = String::new();
        for slot in slots(model, s) {
            match slot {
                Slot::Plain(m) => w!(
                    offsets,
                    "(\"{}\", offset_of!({rust}, {})),",
                    m.name,
                    field_name(&m.name)
                ),
                Slot::Bits { storage, bits } => {
                    for bit in bits {
                        let shift = match bit.shift {
                            0 => String::new(),
                            shift => format!(" + {shift}"),
                        };
                        w!(
                            bit_field_rows,
                            "{cfg}BitLayout {{ name: \"{}.{}\", offset: offset_of!({rust}, {storage}) * 8{shift}, width: {} }},",
                            s.name,
                            bit.member.name,
                            bit.width
                        );
                    }
                }
            }
        }
        w!(
            struct_rows,
            "{cfg}StructLayout {{
                name: \"{name}\",
                size: size_of::<{rust}>(),
                align: align_of::<{rust}>(),
                offsets: &[
                    {offsets}],
            }},",
            name = s.name
        );
    }
    let mut value_rows = String::new();
    for value in checked_values(model) {
        for rust_expr in &value.rust_exprs {
            w!(
                value_rows,
                "{}(\"{}\", {rust_expr} as i64),",
                value.cfg,
                value.c_name
            );
        }
    }
    format!(
        "use core::mem::{{align_of, offset_of, size_of}};
use vulkan::vk;

pub struct StructLayout {{
    pub name: &'static str,
    pub size: usize,
    pub align: usize,
    pub offsets: &'static [(&'static str, usize)],
}}

pub struct BitLayout {{
    pub name: &'static str,
    pub offset: usize,
    pub width: usize,
}}

pub const STRUCTS: &[StructLayout] = &[
    {struct_rows}];

pub const BIT_FIELDS: &[BitLayout] = &[
    {bit_field_rows}];

pub const VALUES: &[(&str, i64)] = &[
    {value_rows}];
"
    )
}

const C_PROGRAM_START: &str = r#"
// Prints the position of the bits that are set in `p`, which holds a zeroed struct
// with a single bit-field set to all ones.
static void bits(const char* name, const void* p, size_t size) {
    const unsigned char* bytes = (const unsigned char*)p;
    long first = -1;
    long count = 0;
    for (size_t i = 0; i < size * 8; i++) {
        if (bytes[i / 8] & (1u << (i % 8))) {
            if (first < 0) first = (long)i;
            count++;
        }
    }
    printf("B %s %ld %ld\n", name, first, count);
}

int main(void) {
"#;

pub(crate) fn emit_c_program(model: &Model<'_>) -> String {
    let mut out = String::from("// @generated by vulkan-codegen. Do not edit.\n\n");
    out.push_str("#include <stddef.h>\n#include <stdio.h>\n#include <string.h>\n\n");
    w!(out, "#define {BETA_PROTECT} 1");
    out.push_str("#include <vulkan/vulkan.h>\n");
    for header in &model.ir.video_headers {
        w!(out, "#include <{}>", header.header_file);
    }
    out.push_str(C_PROGRAM_START);
    for s in checked_structs(model) {
        let name = &s.name;
        let slots = slots(model, s);
        w!(
            out,
            "    printf(\"S {name} %zu %zu\", sizeof({name}), _Alignof({name}));"
        );
        for slot in &slots {
            if let Slot::Plain(m) = slot {
                w!(
                    out,
                    "    printf(\" {0}=%zu\", offsetof({name}, {0}));",
                    m.name
                );
            }
        }
        w!(out, "    printf(\"\\n\");");
        for slot in &slots {
            if let Slot::Bits { bits, .. } = slot {
                for bit in bits {
                    w!(
                        out,
                        "    {{ {name} s; memset(&s, 0, sizeof s); s.{0} = ~0u; bits(\"{name}.{0}\", &s, sizeof s); }}",
                        bit.member.name
                    );
                }
            }
        }
    }
    for value in checked_values(model) {
        w!(
            out,
            "    printf(\"V {0} %lld\\n\", (long long)({0}));",
            value.c_name
        );
    }
    out.push_str("    return 0;\n}\n");
    out
}
