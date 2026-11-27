use regex::Regex;
use std::collections::BTreeMap;
use std::path::PathBuf;
use syn::{Expr, ImplItem, Item, Lit, Meta, UnOp};

const NOT_A_VALUE: i64 = 0x7FFF_FFFF;

#[derive(Debug, PartialEq)]
struct Enumerant {
    name: String,
    value: i64,
    alias_of: Option<String>,
}

fn enumerant_value(text: &str) -> Option<i64> {
    let (negative, digits) = match text.strip_prefix('-') {
        Some(digits) => (true, digits),
        None => (false, text),
    };
    let value = match digits.strip_prefix("0x") {
        Some(hexadecimal) => i64::from_str_radix(hexadecimal, 16).ok()?,
        None if digits.len() > 1 && digits.starts_with('0') => return None,
        None => digits.parse::<i64>().ok()?,
    };
    Some(if negative { -value } else { value })
}

fn header_enumerations(file: &str, header: &str) -> BTreeMap<String, Vec<Enumerant>> {
    let block_comment = Regex::new(r"(?s)/\*.*?\*/").expect("a regular expression");
    let line_comment = Regex::new(r"//[^\n]*").expect("a regular expression");
    let header = block_comment.replace_all(header, " ");
    let header = line_comment.replace_all(&header, " ");
    let definition = Regex::new(r"typedef\s+enum\s+(\w+)\s*\{([^{}]*)\}\s*(\w+)\s*;")
        .expect("a regular expression");
    let keyword = Regex::new(r"\benum\b").expect("a regular expression");
    let assignment = Regex::new(r"^(\w+)\s*=\s*(\S.*)$").expect("a regular expression");
    let identifier = Regex::new(r"^[A-Za-z_]\w*$").expect("a regular expression");

    let mut enumerations = BTreeMap::new();
    for definition in definition.captures_iter(&header) {
        let (name, body) = (&definition[1], &definition[2]);
        assert_eq!(
            name, &definition[3],
            "{file}: an enumeration with two names"
        );
        assert!(
            !body.contains('#'),
            "{file}: `{name}` has a preprocessor line among its enumerants, which the test does not read"
        );
        let mut enumerants = Vec::<Enumerant>::new();
        for entry in body.split(',').map(str::trim).filter(|e| !e.is_empty()) {
            let entry = entry.split_whitespace().collect::<Vec<_>>().join(" ");
            let Some(parts) = assignment.captures(&entry) else {
                panic!(
                    "{file}: the enumerant `{entry}` of `{name}` has no value, which the test does not read"
                );
            };
            let (enumerant, value) = (&parts[1], &parts[2]);
            let earlier = enumerants.iter().find(|earlier| earlier.name == value);
            let (value, alias_of) = match (enumerant_value(value), earlier) {
                (Some(value), _) => (value, None),
                (None, Some(earlier)) => {
                    let ends_at = earlier.alias_of.clone();
                    (earlier.value, ends_at.or(Some(earlier.name.clone())))
                }
                (None, None) if identifier.is_match(value) => panic!(
                    "{file}: `{enumerant}` of `{name}` is `{value}`, which is no earlier enumerant of `{name}`"
                ),
                (None, None) => panic!(
                    "{file}: `{enumerant}` of `{name}` has the value `{value}`, a form that the test does not read"
                ),
            };
            assert!(
                enumerants.iter().all(|earlier| earlier.name != enumerant),
                "{file}: `{name}` has `{enumerant}` twice"
            );
            enumerants.push(Enumerant {
                name: enumerant.to_string(),
                value,
                alias_of,
            });
        }
        let earlier = enumerations.insert(name.to_string(), enumerants);
        assert!(earlier.is_none(), "{file}: `{name}` is defined twice");
    }
    assert_eq!(
        keyword.find_iter(&header).count(),
        enumerations.len(),
        "{file}: an `enum` that is not a `typedef enum Name {{ ... }} Name;`, which the test does not read"
    );
    enumerations
}

struct Constant {
    ident: String,
    value: Option<i64>,
    names: Vec<String>,
    doc: String,
}

fn doc_attributes(attributes: &[syn::Attribute]) -> (String, Vec<String>) {
    let (mut doc, mut names) = (Vec::new(), Vec::new());
    for attribute in attributes.iter().filter(|a| a.path().is_ident("doc")) {
        match &attribute.meta {
            Meta::NameValue(pair) => {
                if let Expr::Lit(literal) = &pair.value
                    && let Lit::Str(line) = &literal.lit
                {
                    doc.push(line.value());
                }
            }
            Meta::List(list) => {
                let _ = list.parse_nested_meta(|nested| {
                    if nested.path.is_ident("alias") {
                        names.push(nested.value()?.parse::<syn::LitStr>()?.value());
                    }
                    Ok(())
                });
            }
            Meta::Path(_) => {}
        }
    }
    let doc = doc.join(" ");
    (doc.split_whitespace().collect::<Vec<_>>().join(" "), names)
}

fn constant_value(expression: &Expr) -> Option<i64> {
    let Expr::Call(call) = expression else {
        return None;
    };
    let Expr::Path(function) = &*call.func else {
        return None;
    };
    if !function.path.is_ident("Self") || call.args.len() != 1 {
        return None;
    }
    let integer = |expression: &Expr| match expression {
        Expr::Lit(literal) => match &literal.lit {
            Lit::Int(integer) => integer.base10_parse::<i64>().ok(),
            _ => None,
        },
        _ => None,
    };
    match &call.args[0] {
        Expr::Unary(unary) if matches!(unary.op, UnOp::Neg(_)) => {
            integer(&unary.expr).map(|value| -value)
        }
        other => integer(other),
    }
}

fn bindings_enumerations(source: &str) -> BTreeMap<String, Vec<Constant>> {
    let file = syn::parse_file(source).unwrap_or_else(|error| panic!("{error}"));
    let mut enumerations = BTreeMap::<String, Vec<Constant>>::new();
    for item in &file.items {
        match item {
            Item::Struct(item) if item.ident.to_string().starts_with("StdVideo") => {
                enumerations.entry(item.ident.to_string()).or_default();
            }
            Item::Impl(item) if item.trait_.is_none() => {
                let syn::Type::Path(owner) = &*item.self_ty else {
                    continue;
                };
                let Some(owner) = owner.path.get_ident().map(|ident| ident.to_string()) else {
                    continue;
                };
                if !owner.starts_with("StdVideo") {
                    continue;
                }
                let constants = enumerations.entry(owner).or_default();
                for member in &item.items {
                    let ImplItem::Const(constant) = member else {
                        continue;
                    };
                    let (doc, names) = doc_attributes(&constant.attrs);
                    constants.push(Constant {
                        ident: constant.ident.to_string(),
                        value: constant_value(&constant.expr),
                        names,
                        doc,
                    });
                }
            }
            _ => {}
        }
    }
    enumerations
}

#[derive(Debug, Default, PartialEq)]
struct Compared {
    failures: Vec<String>,
    enumerations: usize,
    enumerants: usize,
    aliases: usize,
    not_values: usize,
}

fn compare(headers: &[(String, String)], bindings: &str) -> Compared {
    let alias = Regex::new(r"^`(\w+)` is (?:an|a legacy) alias of \[`(\w+)`\]\(Self::(\w+)\)\.")
        .expect("a regular expression");
    let mut compared = Compared::default();
    let mut bindings = bindings_enumerations(bindings);
    let mut seen = BTreeMap::<String, &str>::new();
    for (file, header) in headers {
        for (name, mut enumerants) in header_enumerations(file, header) {
            let earlier = seen.insert(name.clone(), file);
            assert!(
                earlier.is_none(),
                "`{name}` is in {file} and in {earlier:?}"
            );
            assert!(
                name.starts_with("StdVideo"),
                "{file}: the name of `{name}` does not start with `StdVideo`, by which the test finds the enumerations of the bindings"
            );
            compared.enumerations += 1;
            match enumerants.last() {
                Some(last) if last.name.ends_with("_MAX_ENUM") && last.value == NOT_A_VALUE => {
                    compared.not_values += 1;
                    enumerants.pop();
                }
                last => compared.failures.push(format!(
                    "{name}: the header ends the enumeration with {last:?}, not with a `_MAX_ENUM` of 0x7FFFFFFF, which is the one enumerant that the bindings leave out"
                )),
            }
            let Some(constants) = bindings.remove(&name) else {
                compared.failures.push(format!(
                    "{name}: {file} has this enumeration, and the bindings have no such type"
                ));
                continue;
            };
            let constant_named = |c_name: &str| {
                let mut constants = constants.iter();
                constants.find(|constant| constant.names.iter().any(|name| name == c_name))
            };
            for enumerant in &enumerants {
                compared.enumerants += 1;
                compared.aliases += usize::from(enumerant.alias_of.is_some());
                let (c_name, value) = (&enumerant.name, enumerant.value);
                let Some(constant) = constant_named(c_name) else {
                    compared.failures.push(format!(
                        "{name}: {file} has `{c_name}` = {value}, and the bindings have no constant with this name"
                    ));
                    continue;
                };
                let item = format!("{name}::{}", constant.ident);
                if constant.value != Some(value) {
                    compared.failures.push(format!(
                        "{item}: {file} has `{c_name}` = {value}, and the bindings have {:?}",
                        constant.value
                    ));
                }
                let documented = alias
                    .captures(&constant.doc)
                    .filter(|_| constant.names.first() == Some(c_name));
                let documented = documented.map(|parts| {
                    let own = &parts[1] == c_name;
                    let links = constant_named(&parts[2]).map(|target| target.ident.as_str());
                    let target = (own && links == Some(&parts[3])).then(|| parts[2].to_string());
                    target.unwrap_or_else(|| parts[0].to_string())
                });
                let shares = constant.names.first() != Some(c_name);
                match (&enumerant.alias_of, documented) {
                    (Some(_), None) if shares => {}
                    (None, None) if !shares => {}
                    (Some(target), Some(documented)) if *target == documented => {}
                    (target, documented) => compared.failures.push(format!(
                        "{item}: {file} has `{c_name}` as an alias of {target:?}, and the bindings document it as an alias of {documented:?}, or as a second name of a constant"
                    )),
                }
            }
            for constant in &constants {
                for c_name in &constant.names {
                    if enumerants.iter().all(|enumerant| enumerant.name != *c_name) {
                        compared.failures.push(format!(
                            "{name}::{}: the bindings have `{c_name}`, and {file} has no such enumerant of `{name}` beside its `_MAX_ENUM`",
                            constant.ident
                        ));
                    }
                }
                if constant.names.is_empty() {
                    compared.failures.push(format!(
                        "{name}::{}: the constant does not name the enumerant of the header that it stands for",
                        constant.ident
                    ));
                }
            }
        }
    }
    for name in bindings.keys() {
        compared.failures.push(format!(
            "{name}: the bindings have this type, and no header has such an enumeration"
        ));
    }
    compared
}

fn headers() -> Result<Vec<(String, String)>, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let directory = manifest_dir.join("../../third_party/vulkan/include/vk_video");
    let entries = std::fs::read_dir(&directory).map_err(|error| {
        format!(
            "{} cannot be read ({error}); run `git submodule update --init`",
            directory.display()
        )
    })?;
    let mut headers = Vec::new();
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().is_some_and(|extension| extension == "h") {
            let file = path.file_name().expect("a file name").to_string_lossy();
            let text = std::fs::read_to_string(&path).map_err(|error| error.to_string())?;
            headers.push((file.into_owned(), text));
        }
    }
    headers.sort();
    Ok(headers)
}

#[test]
fn the_enumerations_of_the_video_headers_are_the_ones_of_the_bindings() {
    let headers = match headers() {
        Ok(headers) => headers,
        Err(reason) => {
            assert!(
                std::env::var_os("VULKAN_RS_REQUIRE_LAYOUT_TEST").is_none(),
                "the test of the Vulkan Video enumerations is required but cannot run: {reason}"
            );
            eprintln!("skipping the test of the Vulkan Video enumerations: {reason}");
            return;
        }
    };
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bindings = std::fs::read_to_string(manifest_dir.join("src/vk/generated/enums.rs"))
        .expect("the generated enumerations");
    let compared = compare(&headers, &bindings);
    assert!(
        compared.failures.is_empty(),
        "{} differences between the Vulkan Video headers and the bindings:\n{}",
        compared.failures.len(),
        compared.failures.join("\n")
    );
    assert_eq!(
        (
            headers.len(),
            compared.enumerations,
            compared.enumerants,
            compared.aliases,
            compared.not_values
        ),
        (12, 37, 309, 1, 37),
        "the headers under vk_video, their enumerations, the enumerants that the bindings have a constant for, how many of those the headers define as another enumerant, and the `_MAX_ENUM` enumerants that the bindings leave out"
    );
}

const FIXTURE_HEADER: &str = "
#define STD_VIDEO_THING_COUNT 4U
/* A thing
 * typedef enum StdVideoComment { STD_VIDEO_COMMENT_A = 0 } StdVideoComment;
 */
typedef enum StdVideoThingKind {
    STD_VIDEO_THING_KIND_PLAIN = 0,
    STD_VIDEO_THING_KIND_ODD = 1,
    STD_VIDEO_THING_KIND_BELOW = -2,
    STD_VIDEO_THING_KIND_INVALID = 0x7FFFFFFF,
  // STD_VIDEO_THING_KIND_SIMPLE is a legacy alias
    STD_VIDEO_THING_KIND_SIMPLE = STD_VIDEO_THING_KIND_PLAIN,
    STD_VIDEO_THING_KIND_BASIC = STD_VIDEO_THING_KIND_SIMPLE,
    STD_VIDEO_THING_KIND_MAX_ENUM = 0x7FFFFFFF
} StdVideoThingKind;
typedef struct StdVideoThing {
    StdVideoThingKind    kind;
} StdVideoThing;
typedef enum StdVideoThingMode {
    STD_VIDEO_THING_MODE_SLOW = 0,
    STD_VIDEO_THING_MODE_FAST_BIT = 1,
    STD_VIDEO_THING_MODE_FAST = STD_VIDEO_THING_MODE_FAST_BIT,
    STD_VIDEO_THING_MODE_MAX_ENUM = 0x7FFFFFFF
} StdVideoThingMode;
";

const FIXTURE_BINDINGS: &str = r#"
/// C enumeration `StdVideoThingKind`.
pub struct StdVideoThingKind(pub i32);

impl StdVideoThingKind {
    /// Wraps a value of the C enumeration.
    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }
    /// `STD_VIDEO_THING_KIND_PLAIN`, a value of `StdVideoThingKind`.
    #[doc(alias = "STD_VIDEO_THING_KIND_PLAIN")]
    pub const PLAIN: Self = Self(0);
    /// `STD_VIDEO_THING_KIND_ODD`, a value of `StdVideoThingKind`.
    #[doc(alias = "STD_VIDEO_THING_KIND_ODD")]
    pub const ODD: Self = Self(1);
    /// `STD_VIDEO_THING_KIND_BELOW`, a value of `StdVideoThingKind`.
    #[doc(alias = "STD_VIDEO_THING_KIND_BELOW")]
    pub const BELOW: Self = Self(-2);
    /// `STD_VIDEO_THING_KIND_INVALID`, a value of `StdVideoThingKind`.
    #[doc(alias = "STD_VIDEO_THING_KIND_INVALID")]
    pub const INVALID: Self = Self(2147483647);
    /// `STD_VIDEO_THING_KIND_SIMPLE` is a legacy alias of
    /// [`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).
    #[doc(alias = "STD_VIDEO_THING_KIND_SIMPLE")]
    pub const SIMPLE: Self = Self(0);
    /// `STD_VIDEO_THING_KIND_BASIC` is an alias of [`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).
    #[doc(alias = "STD_VIDEO_THING_KIND_BASIC")]
    pub const BASIC: Self = Self(0);
}

/// C enumeration `StdVideoThingMode`.
pub struct StdVideoThingMode(pub i32);

impl StdVideoThingMode {
    /// `STD_VIDEO_THING_MODE_SLOW`, a value of `StdVideoThingMode`.
    #[doc(alias = "STD_VIDEO_THING_MODE_SLOW")]
    pub const SLOW: Self = Self(0);
    /// `STD_VIDEO_THING_MODE_FAST_BIT`, a value of `StdVideoThingMode`.
    ///
    /// The C header also calls this value `STD_VIDEO_THING_MODE_FAST`, which is a legacy alias of
    /// `STD_VIDEO_THING_MODE_FAST_BIT`.
    #[doc(alias = "STD_VIDEO_THING_MODE_FAST_BIT")]
    #[doc(alias = "STD_VIDEO_THING_MODE_FAST")]
    pub const FAST: Self = Self(1);
}

/// C enumeration `VkColor`.
pub struct Color(pub i32);

impl Color {
    /// `VK_COLOR_RED`, a value of `VkColor`.
    #[doc(alias = "VK_COLOR_RED")]
    pub const RED: Self = Self(0);
}
"#;

fn fixture(header: &str, bindings: &str) -> Compared {
    compare(&[("thing.h".to_string(), header.to_string())], bindings)
}

fn with(text: &str, replace: &str, by: &str) -> String {
    assert_eq!(text.matches(replace).count(), 1, "`{replace}`");
    text.replacen(replace, by, 1)
}

#[test]
fn the_comparison_accepts_bindings_that_have_every_enumerant_of_the_header() {
    assert_eq!(
        fixture(FIXTURE_HEADER, FIXTURE_BINDINGS),
        Compared {
            failures: Vec::new(),
            enumerations: 2,
            enumerants: 9,
            aliases: 3,
            not_values: 2,
        }
    );
}

#[test]
fn the_reader_of_the_header_gives_an_alias_the_value_and_the_enumerant_that_it_ends_at() {
    let enumerations = header_enumerations("thing.h", FIXTURE_HEADER);
    assert_eq!(
        enumerations.keys().collect::<Vec<_>>(),
        ["StdVideoThingKind", "StdVideoThingMode"]
    );
    let kind = &enumerations["StdVideoThingKind"];
    let read = kind
        .iter()
        .map(|enumerant| {
            let name = enumerant.name.strip_prefix("STD_VIDEO_THING_KIND_");
            (
                name.expect("the prefix"),
                enumerant.value,
                enumerant.alias_of.as_deref(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        read,
        [
            ("PLAIN", 0, None),
            ("ODD", 1, None),
            ("BELOW", -2, None),
            ("INVALID", NOT_A_VALUE, None),
            ("SIMPLE", 0, Some("STD_VIDEO_THING_KIND_PLAIN")),
            ("BASIC", 0, Some("STD_VIDEO_THING_KIND_PLAIN")),
            ("MAX_ENUM", NOT_A_VALUE, None),
        ]
    );
}

#[test]
fn the_comparison_reports_every_difference_between_the_header_and_the_bindings() {
    let simple = "    /// `STD_VIDEO_THING_KIND_SIMPLE` is a legacy alias of
    /// [`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).
    #[doc(alias = \"STD_VIDEO_THING_KIND_SIMPLE\")]
    pub const SIMPLE: Self = Self(0);
";
    let simple_alias = "`STD_VIDEO_THING_KIND_SIMPLE` is a legacy alias of\n    /// [`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).";
    let alias_of_plain = "Some(\"STD_VIDEO_THING_KIND_PLAIN\")";
    let differs = |item: &str, c_name: &str, header: &str, bindings: &str| {
        format!(
            "{item}: thing.h has `{c_name}` as an alias of {header}, and the bindings document it as an alias of {bindings}, or as a second name of a constant"
        )
    };
    for (defect, header, bindings, failures) in [
        (
            "an alias of the header without a constant",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, simple, ""),
            vec![
                "StdVideoThingKind: thing.h has `STD_VIDEO_THING_KIND_SIMPLE` = 0, and the bindings have no constant with this name".to_string(),
            ],
        ),
        (
            "a value of the header without a constant",
            FIXTURE_HEADER.to_string(),
            with(
                FIXTURE_BINDINGS,
                "    #[doc(alias = \"STD_VIDEO_THING_KIND_ODD\")]\n    pub const ODD: Self = Self(1);\n",
                "    pub fn odd() {}\n",
            ),
            vec![
                "StdVideoThingKind: thing.h has `STD_VIDEO_THING_KIND_ODD` = 1, and the bindings have no constant with this name".to_string(),
            ],
        ),
        (
            "another value",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "pub const ODD: Self = Self(1);", "pub const ODD: Self = Self(3);"),
            vec![
                "StdVideoThingKind::ODD: thing.h has `STD_VIDEO_THING_KIND_ODD` = 1, and the bindings have Some(3)".to_string(),
            ],
        ),
        (
            "a value with the other sign",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "Self(-2)", "Self(2)"),
            vec![
                "StdVideoThingKind::BELOW: thing.h has `STD_VIDEO_THING_KIND_BELOW` = -2, and the bindings have Some(2)".to_string(),
            ],
        ),
        (
            "a value that is no number",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "pub const ODD: Self = Self(1);", "pub const ODD: Self = Self::PLAIN;"),
            vec![
                "StdVideoThingKind::ODD: thing.h has `STD_VIDEO_THING_KIND_ODD` = 1, and the bindings have None".to_string(),
            ],
        ),
        (
            "an alias with another value",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "pub const SIMPLE: Self = Self(0);", "pub const SIMPLE: Self = Self(1);"),
            vec![
                "StdVideoThingKind::SIMPLE: thing.h has `STD_VIDEO_THING_KIND_SIMPLE` = 0, and the bindings have Some(1)".to_string(),
            ],
        ),
        (
            "a constant that the header does not have",
            FIXTURE_HEADER.to_string(),
            with(
                FIXTURE_BINDINGS,
                "    pub const ODD: Self = Self(1);\n",
                "    pub const ODD: Self = Self(1);\n    /// `STD_VIDEO_THING_KIND_EVEN`, a value.\n    #[doc(alias = \"STD_VIDEO_THING_KIND_EVEN\")]\n    pub const EVEN: Self = Self(2);\n",
            ),
            vec![
                "StdVideoThingKind::EVEN: the bindings have `STD_VIDEO_THING_KIND_EVEN`, and thing.h has no such enumerant of `StdVideoThingKind` beside its `_MAX_ENUM`".to_string(),
            ],
        ),
        (
            "a constant for the enumerant that is no value",
            FIXTURE_HEADER.to_string(),
            with(
                FIXTURE_BINDINGS,
                "    pub const ODD: Self = Self(1);\n",
                "    pub const ODD: Self = Self(1);\n    /// `STD_VIDEO_THING_KIND_MAX_ENUM`, a value.\n    #[doc(alias = \"STD_VIDEO_THING_KIND_MAX_ENUM\")]\n    pub const MAX_ENUM: Self = Self(2147483647);\n",
            ),
            vec![
                "StdVideoThingKind::MAX_ENUM: the bindings have `STD_VIDEO_THING_KIND_MAX_ENUM`, and thing.h has no such enumerant of `StdVideoThingKind` beside its `_MAX_ENUM`".to_string(),
            ],
        ),
        (
            "a constant without the name of its enumerant",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "    #[doc(alias = \"STD_VIDEO_THING_KIND_ODD\")]\n", ""),
            vec![
                "StdVideoThingKind: thing.h has `STD_VIDEO_THING_KIND_ODD` = 1, and the bindings have no constant with this name".to_string(),
                "StdVideoThingKind::ODD: the constant does not name the enumerant of the header that it stands for".to_string(),
            ],
        ),
        (
            "an alias documented as a value",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, simple_alias, "`STD_VIDEO_THING_KIND_SIMPLE`, a value of `StdVideoThingKind`."),
            vec![differs("StdVideoThingKind::SIMPLE", "STD_VIDEO_THING_KIND_SIMPLE", alias_of_plain, "None")],
        ),
        (
            "an alias documented as an alias of another enumerant",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "[`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).\n    #[doc(alias = \"STD_VIDEO_THING_KIND_SIMPLE\")]", "[`STD_VIDEO_THING_KIND_ODD`](Self::ODD).\n    #[doc(alias = \"STD_VIDEO_THING_KIND_SIMPLE\")]"),
            vec![differs("StdVideoThingKind::SIMPLE", "STD_VIDEO_THING_KIND_SIMPLE", alias_of_plain, "Some(\"STD_VIDEO_THING_KIND_ODD\")")],
        ),
        (
            "an alias that links the constant of another enumerant",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "[`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).\n    #[doc(alias = \"STD_VIDEO_THING_KIND_SIMPLE\")]", "[`STD_VIDEO_THING_KIND_PLAIN`](Self::ODD).\n    #[doc(alias = \"STD_VIDEO_THING_KIND_SIMPLE\")]"),
            vec![differs("StdVideoThingKind::SIMPLE", "STD_VIDEO_THING_KIND_SIMPLE", alias_of_plain, "Some(\"`STD_VIDEO_THING_KIND_SIMPLE` is a legacy alias of [`STD_VIDEO_THING_KIND_PLAIN`](Self::ODD).\")")],
        ),
        (
            "an alias of an alias documented as an alias of the alias",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "is an alias of [`STD_VIDEO_THING_KIND_PLAIN`](Self::PLAIN).", "is an alias of [`STD_VIDEO_THING_KIND_SIMPLE`](Self::SIMPLE)."),
            vec![differs("StdVideoThingKind::BASIC", "STD_VIDEO_THING_KIND_BASIC", alias_of_plain, "Some(\"STD_VIDEO_THING_KIND_SIMPLE\")")],
        ),
        (
            "a value documented as an alias",
            with(FIXTURE_HEADER, "STD_VIDEO_THING_KIND_SIMPLE = STD_VIDEO_THING_KIND_PLAIN,", "STD_VIDEO_THING_KIND_SIMPLE = 0,"),
            FIXTURE_BINDINGS.to_string(),
            vec![
                differs("StdVideoThingKind::SIMPLE", "STD_VIDEO_THING_KIND_SIMPLE", "None", alias_of_plain),
                differs("StdVideoThingKind::BASIC", "STD_VIDEO_THING_KIND_BASIC", "Some(\"STD_VIDEO_THING_KIND_SIMPLE\")", alias_of_plain),
            ],
        ),
        (
            "a value of its own as the second name of a constant",
            with(FIXTURE_HEADER, "STD_VIDEO_THING_MODE_FAST = STD_VIDEO_THING_MODE_FAST_BIT,", "STD_VIDEO_THING_MODE_FAST = 1,"),
            FIXTURE_BINDINGS.to_string(),
            vec![differs("StdVideoThingMode::FAST", "STD_VIDEO_THING_MODE_FAST", "None", "None")],
        ),
        (
            "an enumeration without the enumerant that is no value",
            with(FIXTURE_HEADER, "    STD_VIDEO_THING_MODE_MAX_ENUM = 0x7FFFFFFF\n", ""),
            FIXTURE_BINDINGS.to_string(),
            vec![
                "StdVideoThingMode: the header ends the enumeration with Some(Enumerant { name: \"STD_VIDEO_THING_MODE_FAST\", value: 1, alias_of: Some(\"STD_VIDEO_THING_MODE_FAST_BIT\") }), not with a `_MAX_ENUM` of 0x7FFFFFFF, which is the one enumerant that the bindings leave out".to_string(),
            ],
        ),
        (
            "an enumeration that the bindings do not have",
            with(FIXTURE_HEADER, "typedef struct StdVideoThing {", "typedef enum StdVideoThingSize { STD_VIDEO_THING_SIZE_MAX_ENUM = 0x7FFFFFFF } StdVideoThingSize;\ntypedef struct StdVideoThing {"),
            FIXTURE_BINDINGS.to_string(),
            vec![
                "StdVideoThingSize: thing.h has this enumeration, and the bindings have no such type".to_string(),
            ],
        ),
        (
            "an enumeration that the header does not have",
            FIXTURE_HEADER.to_string(),
            with(FIXTURE_BINDINGS, "/// C enumeration `VkColor`.", "pub struct StdVideoThingSize(pub i32);\n/// C enumeration `VkColor`."),
            vec![
                "StdVideoThingSize: the bindings have this type, and no header has such an enumeration".to_string(),
            ],
        ),
    ] {
        assert_eq!(fixture(&header, &bindings).failures, failures, "{defect}");
    }
}

#[test]
fn the_reader_of_the_header_stops_at_a_form_that_it_does_not_read() {
    for (replace, by, message) in [
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD,",
            "thing.h: the enumerant `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` has no value, which the test does not read",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD = 1 << 0,",
            "thing.h: `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` has the value `1 << 0`, a form that the test does not read",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD = 010,",
            "thing.h: `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` has the value `010`, a form that the test does not read",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD = -07,",
            "thing.h: `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` has the value `-07`, a form that the test does not read",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD = 1U,",
            "thing.h: `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` has the value `1U`, a form that the test does not read",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_ODD = STD_VIDEO_THING_MODE_SLOW,",
            "thing.h: `STD_VIDEO_THING_KIND_ODD` of `StdVideoThingKind` is `STD_VIDEO_THING_MODE_SLOW`, which is no earlier enumerant of `StdVideoThingKind`",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "STD_VIDEO_THING_KIND_PLAIN = 1,",
            "thing.h: `StdVideoThingKind` has `STD_VIDEO_THING_KIND_PLAIN` twice",
        ),
        (
            "STD_VIDEO_THING_KIND_ODD = 1,",
            "#ifdef ODD\n    STD_VIDEO_THING_KIND_ODD = 1,\n#endif",
            "thing.h: `StdVideoThingKind` has a preprocessor line among its enumerants, which the test does not read",
        ),
        (
            "} StdVideoThingKind;",
            "} StdVideoThingSort;",
            "assertion `left == right` failed: thing.h: an enumeration with two names\n  left: \"StdVideoThingKind\"\n right: \"StdVideoThingSort\"",
        ),
        (
            "typedef struct StdVideoThing {",
            "typedef enum StdVideoThingMode { STD_VIDEO_THING_MODE_OTHER = 0 } StdVideoThingMode;\ntypedef struct StdVideoThing {",
            "thing.h: `StdVideoThingMode` is defined twice",
        ),
        (
            "typedef struct StdVideoThing {",
            "enum StdVideoThingSize { STD_VIDEO_THING_SIZE_SMALL = 0 };\ntypedef struct StdVideoThing {",
            "assertion `left == right` failed: thing.h: an `enum` that is not a `typedef enum Name { ... } Name;`, which the test does not read\n  left: 3\n right: 2",
        ),
    ] {
        let header = with(FIXTURE_HEADER, replace, by);
        let read = std::panic::catch_unwind(|| header_enumerations("thing.h", &header).len());
        let panic = read.expect_err(&format!("`{by}` is read"));
        let panic = panic
            .downcast_ref::<String>()
            .cloned()
            .or(panic.downcast_ref::<&str>().map(|text| text.to_string()));
        assert_eq!(panic.as_deref(), Some(message), "{by}");
    }
}
