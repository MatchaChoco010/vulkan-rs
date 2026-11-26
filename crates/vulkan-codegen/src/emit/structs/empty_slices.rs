use super::definition;
use super::nullable_arrays::NULLABLE_ARRAYS;
use super::setters::count_member;
use crate::emit::{doc_of, squash};
use crate::model::Model;
use crate::naming::field_name;
use crate::test_support::catch_quietly;
use std::collections::{BTreeMap, BTreeSet};
use vulkan_registry::Registry;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Empty {
    Absent,
    Null,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct SliceSetter<'g> {
    pub(super) name: &'g str,
    pub(super) field: &'g str,
    pub(super) empty: Empty,
    pub(super) pointer: &'g str,
    pub(super) stores_a_length: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct Setter<'g> {
    name: &'g str,
    value: &'g str,
    body: &'g str,
}

fn setters<'g>(generated: &'g str, receiver: &str) -> Vec<Setter<'g>> {
    let value_parameter = format!("({receiver}value: ");
    let mut setters = Vec::new();
    let mut rest = generated;
    while let Some(at) = rest.find(&value_parameter) {
        let (_, name) = rest[..at]
            .rsplit_once("pub fn ")
            .unwrap_or_else(|| panic!("no name before `{}`", &rest[at..]));
        let (value, after) = rest[at + value_parameter.len()..]
            .split_once(") -> Self { ")
            .unwrap_or_else(|| panic!("{name}: no body"));
        let (body, after) = after
            .split_once(" self }")
            .or_else(|| after.split_once(" union }"))
            .unwrap_or_else(|| panic!("{name}: the body does not end with the value: {after}"));
        setters.push(Setter { name, value, body });
        rest = after;
    }
    setters
}

fn is_exclusive_slice(value: &str) -> Option<bool> {
    let referent = value.strip_prefix("&'a ")?;
    let (exclusive, referent) = match referent.strip_prefix("mut ") {
        Some(referent) => (true, referent),
        None => (false, referent),
    };
    let element = referent.strip_prefix('[')?.strip_suffix(']')?;
    let mut depth = 0_usize;
    for c in element.chars() {
        match c {
            '[' | '<' | '(' => depth += 1,
            ']' | '>' | ')' => depth = depth.checked_sub(1)?,
            ';' if depth == 0 => return None,
            _ => {}
        }
    }
    Some(exclusive)
}

fn starts_taken(code: &str) -> usize {
    code.matches(".as_ptr()").count() + code.matches(".as_mut_ptr()").count()
}

const CASTS: [&str; 4] = ["", ".cast()", ".cast_mut()", ".cast_mut().cast()"];

fn slice_setter<'g>(setter: &Setter<'g>, exclusive: bool) -> SliceSetter<'g> {
    let Setter { name, body, .. } = *setter;
    let (start, or_null) = if exclusive {
        ("value.as_mut_ptr()", "mut_ptr_or_null(value)")
    } else {
        ("value.as_ptr()", "ptr_or_null(value)")
    };
    let (empty, when_empty, stores) = match body.strip_prefix("if value.is_empty() { ") {
        Some(branches) => {
            let (when_empty, otherwise) = branches
                .split_once(" } else { ")
                .unwrap_or_else(|| panic!("{name}: an empty slice is not told apart: {body}"));
            let otherwise = otherwise
                .strip_suffix(" }")
                .unwrap_or_else(|| panic!("{name}: something follows the two branches: {body}"));
            (Empty::Absent, when_empty, otherwise)
        }
        None => (Empty::Null, "", body),
    };
    let (lengths, assignment) = stores
        .rsplit_once("self.")
        .unwrap_or_else(|| panic!("{name}: the setter assigns nothing: {body}"));
    let (field, pointer) = assignment
        .split_once(" = ")
        .and_then(|(field, pointer)| Some((field, pointer.strip_suffix(';')?)))
        .unwrap_or_else(|| panic!("{name}: the setter does not end with an assignment: {body}"));
    let (expected, starts, helpers) = match empty {
        Empty::Absent => (start, 1, 0),
        Empty::Null => (or_null, 0, 1),
    };
    assert!(
        pointer
            .strip_prefix(expected)
            .is_some_and(|casts| CASTS.contains(&casts)),
        "{name}: the setter takes a slice and stores `{pointer}`, which is not `{expected}` and casts: {body}"
    );
    assert!(
        starts_taken(body) == starts && body.matches("ptr_or_null(").count() == helpers,
        "{name}: the setter takes the start of the slice somewhere else than in `{pointer}`: {body}"
    );
    if empty == Empty::Absent {
        assert!(
            ["core::ptr::null()", "core::ptr::null_mut()"]
                .iter()
                .any(|null| when_empty.ends_with(&format!("self.{field} = {null};"))),
            "{name}: the branch for an empty slice does not end with a null pointer in `{field}`: {body}"
        );
    }
    SliceSetter {
        name,
        field,
        empty,
        pointer,
        stores_a_length: !lengths.is_empty(),
    }
}

pub(super) fn slice_setters(generated: &str) -> Vec<SliceSetter<'_>> {
    setters(generated, "mut self, ")
        .iter()
        .filter_map(|setter| Some(slice_setter(setter, is_exclusive_slice(setter.value)?)))
        .collect()
}

#[test]
fn the_scan_tells_slices_from_the_other_values_of_a_setter() {
    for (value, expected) in [
        ("&'a [u32]", Some(false)),
        ("&'a mut [u8]", Some(true)),
        ("&'a [&'a Geometry<'a>]", Some(false)),
        ("&'a [CStrPtr<'a>]", Some(false)),
        ("&'a [[f32; 4]]", Some(false)),
        ("&'a mut [(u32, [u8; 2])]", Some(true)),
        ("&'a [u8; 2 * UUID_SIZE]", None),
        ("&'a mut [SampleMask; 2]", None),
        ("&'a [[f32; 4]; 3]", None),
        ("&'a CStr", None),
        ("&'a Geometry<'a>", None),
        ("&'a mut u32", None),
        ("[f32; 4]", None),
        ("*const c_void", None),
        ("&[u32]", None),
        ("&'a [u32]]", None),
    ] {
        assert_eq!(is_exclusive_slice(value), expected, "{value}");
    }
}

#[test]
fn the_scan_reads_the_setters_and_the_constructors_of_generated_code() {
    let generated = "impl<'a> Info<'a> { pub fn count(mut self, value: u32) -> Self { self.count = value; self } pub fn values<T: Value>(mut self, value: &'a [T]) -> Self { self.count = value.len() as _; self.p_values = ptr_or_null(value).cast(); self } pub fn push_next<T: Extends<Self>>(mut self, next: &'a mut T) -> Self { self.p_next = push_chain(next, self.p_next); self } }";
    assert_eq!(
        setters(generated, "mut self, "),
        [
            Setter {
                name: "count",
                value: "u32",
                body: "self.count = value;"
            },
            Setter {
                name: "values<T: Value>",
                value: "&'a [T]",
                body: "self.count = value.len() as _; self.p_values = ptr_or_null(value).cast();"
            },
        ]
    );
    assert_eq!(
        slice_setters(generated),
        [SliceSetter {
            name: "values<T: Value>",
            field: "p_values",
            empty: Empty::Null,
            pointer: "ptr_or_null(value).cast()",
            stores_a_length: true,
        }]
    );
    let union = "impl<'a> Value<'a> { pub fn name(value: &'a CStr) -> Self { let mut union = Self::default(); union.p_name = value.as_ptr(); union } }";
    assert_eq!(
        setters(union, ""),
        [Setter {
            name: "name",
            value: "&'a CStr",
            body: "let mut union = Self::default(); union.p_name = value.as_ptr();"
        }]
    );
    assert!(setters(union, "mut self, ").is_empty());
}

#[test]
fn the_scan_reads_how_a_setter_treats_an_empty_slice() {
    let read = |value: &'static str, body: &'static str| {
        let setter = Setter {
            name: "values",
            value,
            body,
        };
        let exclusive = is_exclusive_slice(value).expect("a slice");
        catch_quietly(move || {
            let read = slice_setter(&setter, exclusive);
            (read.field, read.empty, read.pointer, read.stores_a_length)
        })
    };
    assert_eq!(
        read("&'a mut [u32]", "self.p_values = mut_ptr_or_null(value);"),
        Ok(("p_values", Empty::Null, "mut_ptr_or_null(value)", false))
    );
    assert_eq!(
        read(
            "&'a [u32]",
            "if !self.p_others.is_null() { assert_eq!(self.count as usize, value.len(), \"VkInfo::pValues\"); } self.count = value.len() as _; self.p_values = ptr_or_null(value).cast_mut().cast();"
        ),
        Ok((
            "p_values",
            Empty::Null,
            "ptr_or_null(value).cast_mut().cast()",
            true
        ))
    );
    assert_eq!(
        read(
            "&'a mut [u8]",
            "if value.is_empty() { if !self.p_values.is_null() { self.count = 0; } self.p_values = core::ptr::null_mut(); } else { self.count = value.len() as _; self.p_values = value.as_mut_ptr().cast(); }"
        ),
        Ok(("p_values", Empty::Absent, "value.as_mut_ptr().cast()", true))
    );
    for (value, body, message) in [
        (
            "&'a [u32]",
            "self.count = value.len() as _; self.p_values = value.as_ptr();",
            "values: the setter takes a slice and stores `value.as_ptr()`, which is not `ptr_or_null(value)` and casts: ",
        ),
        (
            "&'a [u32]",
            "self.p_values = value.as_ptr().cast();",
            "values: the setter takes a slice and stores `value.as_ptr().cast()`, which is not `ptr_or_null(value)` and casts: ",
        ),
        (
            "&'a mut [u32]",
            "self.p_values = value.as_mut_ptr();",
            "values: the setter takes a slice and stores `value.as_mut_ptr()`, which is not `mut_ptr_or_null(value)` and casts: ",
        ),
        (
            "&'a mut [u32]",
            "self.p_values = ptr_or_null(value);",
            "values: the setter takes a slice and stores `ptr_or_null(value)`, which is not `mut_ptr_or_null(value)` and casts: ",
        ),
        (
            "&'a [u32]",
            "self.p_values = ptr_or_null(value).add(1);",
            "values: the setter takes a slice and stores `ptr_or_null(value).add(1)`, which is not `ptr_or_null(value)` and casts: ",
        ),
        (
            "&'a [u32]",
            "self.p_first = value.as_ptr(); self.p_values = ptr_or_null(value);",
            "values: the setter takes the start of the slice somewhere else than in `ptr_or_null(value)`: ",
        ),
        (
            "&'a [u32]",
            "self.p_first = ptr_or_null(value); self.p_values = ptr_or_null(value);",
            "values: the setter takes the start of the slice somewhere else than in `ptr_or_null(value)`: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.p_values = value.as_ptr(); } else { self.p_values = value.as_ptr(); }",
            "values: the setter takes the start of the slice somewhere else than in `value.as_ptr()`: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.count = 0; } else { self.p_values = value.as_ptr(); }",
            "values: the branch for an empty slice does not end with a null pointer in `p_values`: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.p_others = core::ptr::null(); } else { self.p_values = value.as_ptr(); }",
            "values: the branch for an empty slice does not end with a null pointer in `p_values`: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.p_values = core::ptr::null(); } else { self.p_values = ptr_or_null(value); }",
            "values: the setter takes a slice and stores `ptr_or_null(value)`, which is not `value.as_ptr()` and casts: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.p_values = core::ptr::null(); }",
            "values: an empty slice is not told apart: ",
        ),
        (
            "&'a [u32]",
            "if value.is_empty() { self.p_values = core::ptr::null(); } else { self.p_values = value.as_ptr(); } self.count = 1;",
            "values: something follows the two branches: ",
        ),
        (
            "&'a [u32]",
            "let _ = value;",
            "values: the setter assigns nothing: ",
        ),
        (
            "&'a [u32]",
            "self.p_values = ptr_or_null(value); }",
            "values: the setter does not end with an assignment: ",
        ),
    ] {
        assert_eq!(read(value, body), Err(format!("{message}{body}")), "{body}");
    }
}

#[test]
fn a_setter_that_compares_its_slice_says_which_arrays_it_compares_it_with() {
    let model = Model::new(Registry::get());
    let mut comparing = 0;
    for s in model.ir.structs.iter().filter(|s| !s.union) {
        let generated = definition(&model, s);
        let out = squash(&generated);
        for setter in setters(&out, "mut self, ") {
            let name = format!("{}::{}", s.name, setter.name);
            let shared = setter
                .body
                .split_once(": every array sharing `")
                .map(|(before, after)| {
                    let condition = before
                        .rsplit_once("if ")
                        .and_then(|(_, after)| after.split_once(" { assert_eq!("))
                        .map(|(condition, _)| condition)
                        .unwrap_or_else(|| panic!("{name}: {before}"));
                    assert!(
                        condition.split(" || ").all(|array| {
                            array
                                .strip_prefix("!self.p")
                                .is_some_and(|array| array.ends_with(".is_null()"))
                        }),
                        "{name} compares its slice with something else than the arrays whose pointer is not null: {condition}"
                    );
                    after
                        .split_once("` must have the same length")
                        .unwrap_or_else(|| panic!("{name}: {after}"))
                        .0
                });
            let documented = doc_of(&generated, &format!("pub fn {}(", setter.name));
            let said = documented
                .iter()
                .filter(|block| block.contains("another array"))
                .collect::<Vec<_>>();
            match shared {
                Some(count) => {
                    comparing += 1;
                    let sentence = format!(
                        "Panics if another array that shares `{count}` is set to a slice that is not empty and the slice has a different length. An array that was set with an empty slice has a null pointer and is not compared."
                    );
                    assert!(
                        said.len() == 1 && said[0].starts_with(&sentence),
                        "{name}: {documented:?}"
                    );
                }
                None => assert!(said.is_empty(), "{name}: {documented:?}"),
            }
        }
    }
    assert_eq!(comparing, 55);
}

#[test]
fn every_slice_setter_of_the_registry_stores_a_null_pointer_for_an_empty_slice() {
    let model = Model::new(Registry::get());
    let mut kinds = BTreeMap::<(Empty, &str), usize>::new();
    let mut without_a_length = BTreeSet::new();
    let mut strings = 0;
    let mut arrays = 0;
    for s in &model.ir.structs {
        let generated = definition(&model, s);
        let out = squash(&generated);
        if s.union {
            for constructor in setters(&out, "") {
                assert!(
                    is_exclusive_slice(constructor.value).is_none()
                        && !constructor.body.contains("ptr_or_null("),
                    "{}: the constructor `{}` of a union takes a slice",
                    s.name,
                    constructor.name
                );
            }
            continue;
        }
        let mut starts = 0;
        let mut helpers = 0;
        for setter in setters(&out, "mut self, ") {
            let Some(exclusive) = is_exclusive_slice(setter.value) else {
                let taken = starts_taken(setter.body);
                let expected = match setter.value.strip_prefix("&'a ") {
                    Some("CStr") => {
                        strings += 1;
                        1
                    }
                    Some(referent) if referent.ends_with(']') => {
                        arrays += 1;
                        1
                    }
                    Some(_) => 0,
                    None => {
                        assert!(
                            !setter.value.contains('&'),
                            "{}::{} takes `{}`, a reference that this test does not read",
                            s.name,
                            setter.name,
                            setter.value
                        );
                        0
                    }
                };
                assert!(
                    taken == expected && !setter.body.contains("ptr_or_null("),
                    "{}::{} takes `{}`, which is not a slice: {}",
                    s.name,
                    setter.name,
                    setter.value,
                    setter.body
                );
                starts += taken;
                continue;
            };
            let read = slice_setter(&setter, exclusive);
            let m = s
                .members
                .iter()
                .find(|m| field_name(&m.name) == read.field)
                .unwrap_or_else(|| panic!("{} has no member `{}`", s.name, read.field));
            let name = format!("{}::{}", s.name, m.name);
            let count = count_member(s, m);
            let nullable = m.optional
                || NULLABLE_ARRAYS
                    .iter()
                    .any(|listed| listed.structure == s.name && listed.member == m.name);
            let expected = if nullable && count.is_some() {
                Empty::Absent
            } else {
                Empty::Null
            };
            assert_eq!(read.empty, expected, "{name}: {out}");
            let stores_a_length = count.is_some() || m.len.as_deref() == Some("codeSize / 4");
            assert_eq!(read.stores_a_length, stores_a_length, "{name}: {out}");
            let pointer = read
                .pointer
                .strip_prefix("value.as_mut_ptr()")
                .or_else(|| read.pointer.strip_prefix("mut_ptr_or_null(value)"))
                .map(|casts| ("&mut", casts))
                .or_else(|| {
                    read.pointer
                        .strip_prefix("value.as_ptr()")
                        .or_else(|| read.pointer.strip_prefix("ptr_or_null(value)"))
                        .map(|casts| ("&", casts))
                })
                .and_then(|(borrow, casts)| {
                    [
                        ("&", "", "&[T] as *const T"),
                        ("&", ".cast()", "&[T] as *const U"),
                        ("&", ".cast_mut()", "&[T] as *mut T"),
                        ("&", ".cast_mut().cast()", "&[T] as *mut U"),
                        ("&mut", "", "&mut [T] as *mut T"),
                        ("&mut", ".cast()", "&mut [T] as *mut U"),
                    ]
                    .into_iter()
                    .find(|(of, with, _)| (*of, *with) == (borrow, casts))
                })
                .map(|(_, _, pointer)| pointer)
                .unwrap_or_else(|| panic!("{name} stores `{}`", read.pointer));
            *kinds.entry((read.empty, pointer)).or_default() += 1;
            let documented = doc_of(&generated, &format!("pub fn {}(", read.name));
            let field = format!("[`{0}`](Self::{0})", read.field);
            let (block, sentence, is_the_whole_block) = match (read.empty, read.stores_a_length) {
                (Empty::Absent, _) => (
                    1,
                    format!(
                        "An empty slice stands for an array that is absent, which the {} allows for this member: it sets {field} to a null pointer and leaves ",
                        if m.optional {
                            "registry"
                        } else {
                            "specification"
                        }
                    ),
                    false,
                ),
                (Empty::Null, true) => (
                    1,
                    format!("An empty slice sets {field} to a null pointer."),
                    true,
                ),
                (Empty::Null, false) => {
                    without_a_length.insert(name.clone());
                    (
                        0,
                        format!(
                            "Sets {field} to the start of a slice, or to a null pointer if the slice is empty. No member stores the length: "
                        ),
                        false,
                    )
                }
            };
            let says_null = documented.get(block).is_some_and(|text| {
                text.starts_with(&sentence) && (text.len() == sentence.len() || !is_the_whole_block)
            });
            assert!(
                says_null,
                "{name}: the documentation does not say what an empty slice stores: {documented:?}"
            );
            assert_eq!(
                documented
                    .iter()
                    .filter(|block| {
                        block.contains("null pointer")
                            && (block.contains("An empty slice")
                                || block.contains("slice is empty"))
                    })
                    .count(),
                1,
                "{name}: {documented:?}"
            );
            match read.empty {
                Empty::Absent => starts += 1,
                Empty::Null => helpers += 1,
            }
        }
        assert_eq!(
            (starts_taken(&out), out.matches("ptr_or_null(").count()),
            (starts, helpers),
            "{}: something else than a setter takes the start of a slice: {out}",
            s.name
        );
    }
    assert_eq!(
        kinds.into_iter().collect::<Vec<_>>(),
        [
            ((Empty::Absent, "&[T] as *const T"), 53),
            ((Empty::Absent, "&[T] as *const U"), 7),
            ((Empty::Absent, "&[T] as *mut T"), 1),
            ((Empty::Absent, "&mut [T] as *mut T"), 16),
            ((Empty::Absent, "&mut [T] as *mut U"), 4),
            ((Empty::Null, "&[T] as *const T"), 205),
            ((Empty::Null, "&[T] as *const U"), 16),
            ((Empty::Null, "&[T] as *mut T"), 2),
            ((Empty::Null, "&[T] as *mut U"), 1),
            ((Empty::Null, "&mut [T] as *mut T"), 4),
            ((Empty::Null, "&mut [T] as *mut U"), 1),
        ],
        "the slice setters of the registry by what they do with an empty slice and by the pointer they store"
    );
    assert_eq!(
        without_a_length.into_iter().collect::<Vec<_>>(),
        [
            "VkDeviceFaultInfoEXT::pAddressInfos",
            "VkDeviceFaultInfoEXT::pVendorInfos",
        ],
        "the slice setters that store no length"
    );
    assert_eq!(
        (strings, arrays),
        (18, 6),
        "the setters that take a string and those that take a reference to an array, which store the pointer as it is"
    );
}
