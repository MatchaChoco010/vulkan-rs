//! Cross-checks the generated bindings against the Khronos C headers.
//!
//! `tests/generated/layout.c` is compiled with the host C compiler against
//! `third_party/vulkan/include` and prints what C sees: struct sizes, alignments, member
//! offsets, bit-field positions and constant values. This test compares that with what
//! Rust sees in `tests/generated/layout_table.rs`.
//!
//! The test needs the Vulkan-Headers submodule and a C compiler (`$CC`, default `cc`).
//! Without them it is skipped, unless `VULKAN_RS_REQUIRE_LAYOUT_TEST` is set, which CI does.

#[path = "generated/layout_table.rs"]
mod table;

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

struct CStruct {
    size: usize,
    align: usize,
    offsets: HashMap<String, usize>,
}

#[derive(Default)]
struct CFacts {
    structs: HashMap<String, CStruct>,
    bit_fields: HashMap<String, (usize, usize)>,
    values: HashMap<String, i64>,
}

/// Compiles and runs `layout.c`. `Err` explains why that was not possible.
fn c_facts() -> Result<CFacts, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let include = manifest_dir.join("../../third_party/vulkan/include");
    if !include.join("vulkan/vulkan_core.h").exists() {
        return Err(format!(
            "{} has no Vulkan headers; run `git submodule update --init`",
            include.display()
        ));
    }
    let source = manifest_dir.join("tests/generated/layout.c");
    let exe = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("vulkan-layout");
    let cc = std::env::var("CC").unwrap_or_else(|_| "cc".to_string());
    let output = Command::new(&cc)
        .arg("-std=c11")
        .arg("-I")
        .arg(&include)
        .arg(&source)
        .arg("-o")
        .arg(&exe)
        .output()
        .map_err(|e| format!("cannot run C compiler `{cc}`: {e}"))?;
    // A compile error means the generated C is wrong, which must fail the test.
    assert!(
        output.status.success(),
        "{cc} failed on {}:\n{}",
        source.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    let output = Command::new(&exe)
        .output()
        .unwrap_or_else(|e| panic!("cannot run {}: {e}", exe.display()));
    assert!(output.status.success(), "{} failed", exe.display());

    let mut facts = CFacts::default();
    for line in String::from_utf8(output.stdout).expect("UTF-8").lines() {
        let mut words = line.split_whitespace();
        let (kind, name) = (words.next().expect("kind"), words.next().expect("name"));
        let mut number = || words.next().expect("number");
        match kind {
            "S" => {
                let size = number().parse().expect("size");
                let align = number().parse().expect("align");
                let offsets = words
                    .map(|pair| {
                        let (member, offset) = pair.split_once('=').expect("member=offset");
                        (member.to_string(), offset.parse().expect("offset"))
                    })
                    .collect();
                facts.structs.insert(
                    name.to_string(),
                    CStruct {
                        size,
                        align,
                        offsets,
                    },
                );
            }
            "B" => {
                let first = number().parse().expect("bit offset");
                let count = number().parse().expect("bit width");
                facts.bit_fields.insert(name.to_string(), (first, count));
            }
            "V" => {
                let value = number().parse().expect("value");
                facts.values.insert(name.to_string(), value);
            }
            other => panic!("unexpected line kind {other}"),
        }
    }
    Ok(facts)
}

#[test]
fn bindings_match_c_headers() {
    let facts = match c_facts() {
        Ok(facts) => facts,
        Err(reason) => {
            assert!(
                std::env::var_os("VULKAN_RS_REQUIRE_LAYOUT_TEST").is_none(),
                "layout test is required but cannot run: {reason}"
            );
            eprintln!("skipping layout test: {reason}");
            return;
        }
    };

    let mut mismatches = Vec::new();

    for s in table::STRUCTS {
        let Some(c) = facts.structs.get(s.name) else {
            mismatches.push(format!("{}: missing from the C output", s.name));
            continue;
        };
        if (s.size, s.align) != (c.size, c.align) {
            mismatches.push(format!(
                "{}: Rust size {} align {}, C size {} align {}",
                s.name, s.size, s.align, c.size, c.align
            ));
        }
        for (member, offset) in s.offsets {
            match c.offsets.get(*member) {
                Some(c_offset) if c_offset == offset => {}
                other => mismatches.push(format!(
                    "{}.{member}: Rust offset {offset}, C offset {other:?}",
                    s.name
                )),
            }
        }
    }

    for bit in table::BIT_FIELDS {
        match facts.bit_fields.get(bit.name) {
            Some(&(offset, width)) if (offset, width) == (bit.offset, bit.width) => {}
            other => mismatches.push(format!(
                "{}: Rust bit offset {} width {}, C {other:?}",
                bit.name, bit.offset, bit.width
            )),
        }
    }

    for (name, value) in table::VALUES {
        match facts.values.get(*name) {
            Some(c_value) if c_value == value => {}
            other => mismatches.push(format!("{name}: Rust {value}, C {other:?}")),
        }
    }

    assert!(
        mismatches.is_empty(),
        "{} mismatches against the C headers:\n{}",
        mismatches.len(),
        mismatches.join("\n")
    );
    // Guard against a table that is accidentally empty.
    assert!(table::STRUCTS.len() > 1000);
    assert!(table::BIT_FIELDS.len() > 100);
    assert!(table::VALUES.len() > 5000);
    eprintln!(
        "checked {} structs, {} bit-fields, {} constants against C",
        table::STRUCTS.len(),
        table::BIT_FIELDS.len(),
        table::VALUES.len()
    );
}
