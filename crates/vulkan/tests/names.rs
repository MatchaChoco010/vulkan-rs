use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::CStr;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::OnceLock;
use syn::{Expr, ImplItem, Item, Lit, Stmt, Type, UnOp};
use vulkan::vk;

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

#[derive(Default)]
struct Constants {
    listed: bool,
    variants: Vec<(i128, String)>,
    aliases: Vec<(i128, String)>,
    defined: BTreeMap<String, i128>,
}

struct Source {
    file: &'static str,
    text: String,
    types: BTreeMap<String, Constants>,
}

fn integer(expr: &Expr) -> Option<i128> {
    match expr {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Int(int) => int.base10_parse().ok(),
            _ => None,
        },
        Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)) => {
            integer(&unary.expr).map(|value: i128| -value)
        }
        _ => None,
    }
}

fn self_value(expr: &Expr) -> Option<i128> {
    let Expr::Call(call) = expr else { return None };
    let Expr::Path(path) = &*call.func else {
        return None;
    };
    if !path.path.is_ident("Self") || call.args.len() != 1 {
        return None;
    }
    integer(&call.args[0])
}

fn listed(name: &str, block: &syn::Block) -> Vec<(i128, String)> {
    let Some(Stmt::Expr(Expr::Reference(reference), None)) = block.stmts.last() else {
        panic!("{name}: the list is not `&[...]`");
    };
    let Expr::Array(array) = &*reference.expr else {
        panic!("{name}: the list is not `&[...]`");
    };
    array
        .elems
        .iter()
        .map(|element| {
            let Expr::Tuple(pair) = element else {
                panic!("{name}: an entry is not a pair");
            };
            let value = self_value(&pair.elems[0])
                .unwrap_or_else(|| panic!("{name}: an entry is not `Self(<integer>)`"));
            let Expr::Lit(syn::ExprLit {
                lit: Lit::Str(entry_name),
                ..
            }) = &pair.elems[1]
            else {
                panic!("{name}: an entry has no string name");
            };
            (value, entry_name.value())
        })
        .collect()
}

fn read_source(file: &'static str) -> Source {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src/vk/generated")
        .join(file);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let parsed = syn::parse_file(&text).unwrap_or_else(|e| panic!("{file}: {e}"));
    let mut types = BTreeMap::<String, Constants>::new();
    for item in &parsed.items {
        let Item::Impl(item) = item else { continue };
        if item.trait_.is_some() {
            continue;
        }
        let Type::Path(self_type) = &*item.self_ty else {
            continue;
        };
        let name = self_type.path.segments.last().unwrap().ident.to_string();
        let mut constants = Constants::default();
        let mut has_list = false;
        for member in &item.items {
            match member {
                ImplItem::Const(constant) => {
                    if let Some(value) = self_value(&constant.expr) {
                        constants.defined.insert(constant.ident.to_string(), value);
                    }
                }
                ImplItem::Fn(function) if function.sig.ident == "variants" => {
                    constants.variants = listed(&name, &function.block);
                    has_list = true;
                }
                ImplItem::Fn(function) if function.sig.ident == "aliases" => {
                    constants.aliases = listed(&name, &function.block);
                }
                _ => {}
            }
        }
        let entry = types.entry(name).or_default();
        entry.defined.extend(constants.defined);
        if has_list {
            entry.listed = true;
            entry.variants = constants.variants;
            entry.aliases = constants.aliases;
        }
    }
    types.retain(|_, constants| constants.listed);
    assert_eq!(
        types.len(),
        text.matches("pub const fn variants()").count(),
        "{file}: every `variants()` was read"
    );
    Source { file, text, types }
}

fn sources() -> &'static [Source; 2] {
    static SOURCES: OnceLock<[Source; 2]> = OnceLock::new();
    SOURCES.get_or_init(|| [read_source("bitmasks.rs"), read_source("enums.rs")])
}

#[test]
fn every_flags_and_enum_type_lists_each_value_once_sorted_with_the_name_of_its_constant() {
    let mut checked = [0usize; 2];
    for (index, source) in sources().iter().enumerate() {
        for (name, constants) in &source.types {
            checked[index] += 1;
            assert!(
                constants
                    .variants
                    .windows(2)
                    .all(|pair| pair[0].0 < pair[1].0),
                "{name}::variants() is not strictly sorted by value, which name() relies on to binary search"
            );
            let mut names = BTreeSet::new();
            for (value, entry) in constants.variants.iter().chain(&constants.aliases) {
                assert!(names.insert(entry), "{name}: {entry} is listed twice");
                assert_eq!(
                    constants.defined.get(entry),
                    Some(value),
                    "{name}: {entry} is listed with a value that its constant does not have"
                );
            }
            for (constant, value) in &constants.defined {
                assert!(
                    names.contains(constant),
                    "{name}::{constant} is not found by from_name"
                );
                assert!(
                    constants.variants.iter().any(|(listed, _)| listed == value),
                    "{name}::{constant} has a value that variants() leaves out, so name() does not name it"
                );
            }
        }
    }
    assert!(
        checked[0] >= 400 && checked[1] >= 190,
        "[flags types, enum types] = {checked:?}"
    );
}

fn bodies(
    text: &str,
    start: impl Fn(&str) -> Option<String>,
    end: &str,
) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let mut lines = text.lines();
    let mut current_impl = String::new();
    while let Some(line) = lines.next() {
        if let Some(rest) = line.strip_prefix("impl ")
            && let Some(name) = rest.strip_suffix(" {")
            && !name.contains(' ')
        {
            current_impl = name.to_string();
        }
        let Some(owner) = start(line) else { continue };
        let owner = if owner.is_empty() {
            current_impl.clone()
        } else {
            owner
        };
        let body: Vec<&str> = lines.by_ref().take_while(|line| *line != end).collect();
        let body = body.join("\n").replace(&format!("\"{owner}\""), "\"Self\"");
        assert!(
            found.insert(owner.clone(), body).is_none(),
            "{owner} has two of them"
        );
    }
    found
}

#[test]
fn every_flags_and_enum_type_names_and_shows_its_values_with_the_same_code() {
    for source in sources() {
        let listed: BTreeSet<&String> = source.types.keys().collect();
        let method =
            |signature: &'static str| move |line: &str| (line == signature).then(String::new);
        let debug = |line: &str| {
            line.strip_prefix("impl core::fmt::Debug for ")
                .and_then(|rest| rest.strip_suffix(" {"))
                .map(str::to_string)
        };
        for (what, found) in [
            (
                "name",
                bodies(
                    &source.text,
                    method("    pub fn name(self) -> Option<&'static str> {"),
                    "    }",
                ),
            ),
            (
                "from_name",
                bodies(
                    &source.text,
                    method("    pub fn from_name(name: &str) -> Option<Self> {"),
                    "    }",
                ),
            ),
            ("Debug", bodies(&source.text, debug, "}")),
        ] {
            let owners: BTreeSet<&String> = found.keys().collect();
            assert_eq!(
                owners, listed,
                "{}: the types with {what} are the types with variants()",
                source.file
            );
            let distinct: BTreeSet<String> = found
                .values()
                .map(|body| {
                    body.split_whitespace()
                        .collect::<String>()
                        .replace(",)", ")")
                        .replace("u64::from(self.0)", "self.0")
                        .replace("u64::from(flag.0)", "flag.0")
                })
                .collect();
            assert_eq!(
                distinct.len(),
                1,
                "{}: every type has the same {what}, with its own name: {distinct:#?}",
                source.file
            );
        }
    }
}

trait Table: Copy + Eq + Debug + 'static {
    const TYPE: &'static str;
    const BITS: u32;
    fn variants() -> &'static [(Self, &'static str)];
    fn name(self) -> Option<&'static str>;
    fn from_name(name: &str) -> Option<Self>;
    fn raw(self) -> i128;
    fn from_raw(raw: i128) -> Self;
    fn is_known(self) -> bool;
    fn all() -> Option<Self>;
}

macro_rules! tables {
    (@all flags $ty:ident) => {
        Some(vk::$ty::all())
    };
    (@all values $ty:ident) => {
        None
    };
    ($kind:ident: $($ty:ident: $raw:ty),* $(,)?) => {
        $(
            impl Table for vk::$ty {
                const TYPE: &'static str = stringify!($ty);
                const BITS: u32 = <$raw>::BITS;
                fn is_known(self) -> bool {
                    vk::$ty::is_known(self)
                }
                fn all() -> Option<Self> {
                    tables!(@all $kind $ty)
                }
                fn variants() -> &'static [(Self, &'static str)] {
                    vk::$ty::variants()
                }
                fn name(self) -> Option<&'static str> {
                    vk::$ty::name(self)
                }
                fn from_name(name: &str) -> Option<Self> {
                    vk::$ty::from_name(name)
                }
                fn raw(self) -> i128 {
                    i128::from(self.as_raw())
                }
                fn from_raw(raw: i128) -> Self {
                    vk::$ty::from_raw(<$raw>::try_from(raw).unwrap())
                }
            }
        )*
    };
}

tables!(
    flags: ImageUsageFlags: u32,
    CullModeFlags: u32,
    ShaderStageFlags: u32,
    ImageAspectFlags: u32,
    PipelineStageFlags2: u64,
    AccessFlags2: u64,
);

tables!(
    values: ShaderStageFlagBits: u32,
    SampleCountFlagBits: u32,
    Format: i32,
    PresentModeKHR: i32,
    ColorSpaceKHR: i32,
    ImageViewType: i32,
    VkResult: i32,
    OpacityMicromapSpecialIndexKHR: i32,
    StdVideoH264ProfileIdc: i32,
);

fn listed_name<T: Table>(value: T) -> Option<&'static str> {
    T::variants()
        .iter()
        .find(|(listed, _)| *listed == value)
        .map(|(_, name)| *name)
}

fn check_names_of_listed_values<T: Table>() {
    let source = sources()
        .iter()
        .find_map(|source| source.types.get(T::TYPE))
        .unwrap_or_else(|| panic!("{} is not in the generated sources", T::TYPE));
    let runtime: Vec<(i128, String)> = T::variants()
        .iter()
        .map(|(value, name)| (value.raw(), name.to_string()))
        .collect();
    assert_eq!(runtime, source.variants, "{}::variants()", T::TYPE);
    for &(value, name) in T::variants() {
        assert_eq!(value.name(), Some(name), "{}::{name}", T::TYPE);
        assert_eq!(T::from_name(name), Some(value), "{}::{name}", T::TYPE);
        for other in [
            name.to_ascii_lowercase(),
            format!("{name} "),
            format!(" {name}"),
            String::new(),
        ] {
            if other != name {
                assert_eq!(
                    T::from_name(&other),
                    None,
                    "{}::from_name({other:?})",
                    T::TYPE
                );
            }
        }
    }
    for (value, alias) in &source.aliases {
        let value = T::from_raw(*value);
        assert_eq!(
            T::from_name(alias),
            Some(value),
            "{}::from_name finds the alias {alias}",
            T::TYPE
        );
        assert_eq!(value.name(), listed_name(value), "{}::{alias}", T::TYPE);
        assert!(value.name().is_some(), "{}::{alias}", T::TYPE);
    }
}

#[test]
fn listed_values_and_aliases_are_found_by_name_and_from_name() {
    check_names_of_listed_values::<vk::ImageUsageFlags>();
    check_names_of_listed_values::<vk::CullModeFlags>();
    check_names_of_listed_values::<vk::ShaderStageFlags>();
    check_names_of_listed_values::<vk::ShaderStageFlagBits>();
    check_names_of_listed_values::<vk::ImageAspectFlags>();
    check_names_of_listed_values::<vk::SampleCountFlagBits>();
    check_names_of_listed_values::<vk::PipelineStageFlags2>();
    check_names_of_listed_values::<vk::AccessFlags2>();
    check_names_of_listed_values::<vk::Format>();
    check_names_of_listed_values::<vk::PresentModeKHR>();
    check_names_of_listed_values::<vk::ColorSpaceKHR>();
    check_names_of_listed_values::<vk::ImageViewType>();
    check_names_of_listed_values::<vk::VkResult>();
    check_names_of_listed_values::<vk::OpacityMicromapSpecialIndexKHR>();
    check_names_of_listed_values::<vk::StdVideoH264ProfileIdc>();
}

fn width_mask<T: Table>() -> u64 {
    if T::BITS == 64 {
        u64::MAX
    } else {
        (1 << T::BITS) - 1
    }
}

fn known_bits<T: Table>() -> Vec<u64> {
    T::variants()
        .iter()
        .map(|(value, _)| u64::try_from(value.raw()).unwrap())
        .collect()
}

fn flag_values<T: Table>() -> BoxedStrategy<T> {
    let mask = width_mask::<T>();
    let known = known_bits::<T>();
    let all = known.iter().fold(0, |all, bits| all | bits);
    let bits = T::BITS;
    prop_oneof![
        Just(0u64),
        Just(mask),
        Just(all),
        prop::sample::select(known.clone()),
        (0..bits).prop_map(|bit| 1 << bit),
        (
            prop::sample::select(known.clone()),
            prop::sample::select(known.clone()),
            prop::sample::select(known),
        )
            .prop_map(|(a, b, c)| a | b | c),
        any::<u64>().prop_map(move |value| value & all),
        any::<u64>().prop_map(move |value| value & mask),
    ]
    .prop_map(|raw| T::from_raw(i128::from(raw)))
    .boxed()
}

fn read_back<T: Table>(text: &str) -> Option<u64> {
    if text == "empty()" {
        return Some(0);
    }
    text.split(" | ").try_fold(0, |value, part| {
        let bits = match part.strip_prefix("0x") {
            Some(hex) => u64::from_str_radix(hex, 16).ok()?,
            None => u64::try_from(T::from_name(part)?.raw()).ok()?,
        };
        Some(value | bits)
    })
}

fn check_flags<T: Table>(value: T) -> Result<(), TestCaseError> {
    let shown = format!("{value:?}");
    prop_assert_eq!(
        read_back::<T>(&shown),
        Some(u64::try_from(value.raw()).unwrap()),
        "{}: {:?} does not read back to {:#x}",
        T::TYPE,
        shown,
        value.raw()
    );
    prop_assert_eq!(value.name(), listed_name(value), "{}", T::TYPE);
    let raw = u64::try_from(value.raw()).unwrap();
    let every_constant = known_bits::<T>().iter().fold(0, |all, bits| all | bits);
    match T::all() {
        Some(all) => {
            prop_assert_eq!(
                u64::try_from(all.raw()).unwrap(),
                every_constant,
                "{}::all() is every bit a constant sets",
                T::TYPE
            );
            prop_assert_eq!(
                value.is_known(),
                raw & !every_constant == 0,
                "{}::is_known({:#x}) tells whether every bit is a bit of all()",
                T::TYPE,
                raw
            );
        }
        None => prop_assert_eq!(
            value.is_known(),
            listed_name(value).is_some(),
            "{}::is_known({:#x}) tells whether a constant has the value",
            T::TYPE,
            raw
        ),
    }
    Ok(())
}

fn run<T: Table>(strategy: BoxedStrategy<T>, check: fn(T) -> Result<(), TestCaseError>) {
    let mut runner = TestRunner::new(cases(256));
    if let Err(error) = runner.run(&strategy, check) {
        panic!("{}: {error}", T::TYPE);
    }
}

#[test]
fn flags_debug_output_reads_back_to_the_value_through_from_name_and_hex() {
    run(flag_values::<vk::ImageUsageFlags>(), check_flags);
    run(flag_values::<vk::CullModeFlags>(), check_flags);
    run(flag_values::<vk::ShaderStageFlags>(), check_flags);
    run(flag_values::<vk::ShaderStageFlagBits>(), check_flags);
    run(flag_values::<vk::ImageAspectFlags>(), check_flags);
    run(flag_values::<vk::SampleCountFlagBits>(), check_flags);
    run(flag_values::<vk::PipelineStageFlags2>(), check_flags);
    run(flag_values::<vk::AccessFlags2>(), check_flags);
}

fn enum_values<T: Table>() -> BoxedStrategy<T> {
    let known: Vec<i32> = T::variants()
        .iter()
        .map(|(value, _)| i32::try_from(value.raw()).unwrap())
        .collect();
    prop_oneof![
        prop::sample::select(known.clone()),
        prop::sample::select(known).prop_map(|raw| raw.saturating_add(1)),
        prop::sample::select(vec![i32::MIN, -1, 0, 1, i32::MAX]),
        any::<i32>(),
    ]
    .prop_map(|raw| T::from_raw(i128::from(raw)))
    .boxed()
}

fn check_enum<T: Table>(value: T) -> Result<(), TestCaseError> {
    let name = listed_name(value);
    prop_assert_eq!(value.name(), name, "{}::name({})", T::TYPE, value.raw());
    prop_assert_eq!(
        value.is_known(),
        name.is_some(),
        "{}::is_known({})",
        T::TYPE,
        value.raw()
    );
    let shown = match name {
        Some(name) => name.to_string(),
        None => format!("{}({})", T::TYPE, value.raw()),
    };
    prop_assert_eq!(format!("{value:?}"), shown);
    Ok(())
}

#[test]
fn enum_debug_output_is_the_name_or_the_type_and_value() {
    run(enum_values::<vk::Format>(), check_enum);
    run(enum_values::<vk::PresentModeKHR>(), check_enum);
    run(enum_values::<vk::ColorSpaceKHR>(), check_enum);
    run(enum_values::<vk::ImageViewType>(), check_enum);
    run(enum_values::<vk::VkResult>(), check_enum);
    run(
        enum_values::<vk::OpacityMicromapSpecialIndexKHR>(),
        check_enum,
    );
    run(enum_values::<vk::StdVideoH264ProfileIdc>(), check_enum);
}

#[test]
fn flags_show_their_bits_by_name_and_the_rest_in_hex() {
    assert_eq!(format!("{:?}", vk::ImageUsageFlags::empty()), "empty()");
    assert_eq!(
        format!(
            "{:?}",
            vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::COLOR_ATTACHMENT
        ),
        "SAMPLED | COLOR_ATTACHMENT"
    );
    assert_eq!(
        format!("{:?}", vk::ImageUsageFlags::from_raw(0x4000_0001)),
        "TRANSFER_SRC | 0x40000000"
    );
    assert_eq!(format!("{:?}", vk::Format::from_raw(-5)), "Format(-5)");
}

fn sample<T: Debug>(strategy: &BoxedStrategy<T>, runner: &mut TestRunner) -> T {
    strategy.new_tree(runner).unwrap().current()
}

fn flag_kinds<T: Table>(runner: &mut TestRunner) -> [u32; 4] {
    let strategy = flag_values::<T>();
    let mut counts = [0; 4];
    for _ in 0..400 {
        let value = sample(&strategy, runner);
        let shown = format!("{value:?}");
        let kind = if value.raw() == 0 {
            0
        } else if value.name().is_some() {
            1
        } else if shown.contains("0x") {
            3
        } else {
            2
        };
        counts[kind] += 1;
    }
    counts
}

fn enum_kinds<T: Table>(runner: &mut TestRunner) -> [u32; 2] {
    let strategy = enum_values::<T>();
    let mut counts = [0; 2];
    for _ in 0..400 {
        counts[usize::from(sample(&strategy, runner).name().is_none())] += 1;
    }
    counts
}

#[test]
fn generated_values_cover_named_combined_unknown_and_unnamed_values() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    for (name, counts) in [
        (
            "ImageUsageFlags",
            flag_kinds::<vk::ImageUsageFlags>(&mut runner),
        ),
        (
            "ShaderStageFlags",
            flag_kinds::<vk::ShaderStageFlags>(&mut runner),
        ),
        (
            "PipelineStageFlags2",
            flag_kinds::<vk::PipelineStageFlags2>(&mut runner),
        ),
    ] {
        assert!(
            counts.iter().all(|&count| count >= 20),
            "{name}: [empty, one constant, several constants, unknown bits] = {counts:?}"
        );
    }
    for (name, counts) in [
        ("Format", enum_kinds::<vk::Format>(&mut runner)),
        ("VkResult", enum_kinds::<vk::VkResult>(&mut runner)),
    ] {
        assert!(
            counts.iter().all(|&count| count >= 50),
            "{name}: [named, unnamed] = {counts:?}"
        );
    }
}

#[test]
fn c_str_ptr_debug_is_the_debug_of_its_string() {
    let strings: [&CStr; 5] = [
        c"",
        c"VK_KHR_swapchain",
        c"quote \" and backslash \\",
        c"tab \t newline \n",
        c"bytes \xff \x01",
    ];
    for string in strings {
        assert_eq!(
            format!("{:?}", vk::CStrPtr::new(string)),
            format!("{string:?}")
        );
    }
}
