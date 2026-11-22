const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "_",
];

const NOT_RAW_IDENTIFIERS: &[&str] = &["self", "Self", "super", "crate", "_"];

fn is_identifier(name: &str) -> bool {
    let plain = match name.strip_prefix("r#") {
        Some(rest) if NOT_RAW_IDENTIFIERS.contains(&rest) => return false,
        Some(rest) => rest,
        None if KEYWORDS.contains(&name) => return false,
        None => name,
    };
    let mut chars = plain.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn identifier(c_name: &str, rust_name: String) -> String {
    assert!(
        is_identifier(&rust_name),
        "the C name `{c_name}` becomes the Rust name `{rust_name}`, which is not a valid Rust \
         identifier"
    );
    rust_name
}

pub(crate) fn type_name(name: &str) -> String {
    if name == "VkResult" {
        return name.to_string();
    }
    identifier(name, name.strip_prefix("Vk").unwrap_or(name).to_string())
}

pub(crate) fn func_pointer_name(name: &str) -> String {
    identifier(name, name.to_string())
}

pub(crate) fn command_name(name: &str) -> String {
    identifier(name, snake_ident(name.strip_prefix("vk").unwrap_or(name)))
}

pub(crate) fn command_const_name(name: &str) -> String {
    let method = snake_ident(name.strip_prefix("vk").unwrap_or(name));
    identifier(name, method.trim_start_matches("r#").to_ascii_uppercase())
}

pub(crate) fn field_name(name: &str) -> String {
    identifier(name, snake_ident(name))
}

pub(crate) fn const_name(name: &str) -> String {
    identifier(name, screaming_ident(name))
}

pub(crate) fn extension_name_const(name_macro: &str) -> String {
    identifier(
        name_macro,
        screaming_ident(name_macro).replace("_EXTENSION_NAME", "_NAME"),
    )
}

pub(crate) fn pointer_setter_name(name: &str) -> String {
    let bytes = name.as_bytes();
    let short = if name.starts_with("pp") && bytes.len() > 2 && bytes[2].is_ascii_uppercase() {
        &name[2..]
    } else if name.starts_with('p') && bytes.len() > 1 && bytes[1].is_ascii_uppercase() {
        &name[1..]
    } else {
        name
    };
    identifier(name, snake_ident(short))
}

fn snake_ident(name: &str) -> String {
    raw_ident(&to_snake(name))
}

fn screaming_ident(name: &str) -> String {
    raw_ident(
        &name
            .strip_prefix("VK_")
            .unwrap_or(name)
            .to_ascii_uppercase(),
    )
}

fn split_vendor_tag<'a>(name: &'a str, vendor_tags: &'a [String]) -> (&'a str, Option<&'a str>) {
    let mut best: Option<&str> = None;
    for tag in vendor_tags {
        if name.ends_with(tag.as_str())
            && name.len() > tag.len()
            && best.is_none_or(|b| tag.len() > b.len())
        {
            best = Some(tag.as_str());
        }
    }
    match best {
        Some(tag) => (&name[..name.len() - tag.len()], Some(tag)),
        None => (name, None),
    }
}

pub(crate) fn enum_variant_name(enum_name: &str, value: &str, vendor_tags: &[String]) -> String {
    let rust_type = type_name(enum_name);
    let (base, tag) = split_vendor_tag(&rust_type, vendor_tags);
    let prefix = format!("{}_", to_screaming(base));
    let mut name = value.strip_prefix("VK_").unwrap_or(value);
    name = name.strip_prefix(prefix.as_str()).unwrap_or(name);
    name = strip_tag_suffix(name, tag);
    identifier(value, raw_ident(name))
}

pub(crate) fn flag_bit_name(bitmask_name: &str, value: &str, vendor_tags: &[String]) -> String {
    let rust_type = type_name(bitmask_name);
    let (base, tag) = split_vendor_tag(&rust_type, vendor_tags);
    let screaming = to_screaming(base);
    let prefix = match screaming.split_once("_FLAG_BITS") {
        Some((head, "")) => format!("{head}_"),
        Some((head, digits)) => format!("{head}_{digits}_"),
        None => format!("{screaming}_"),
    };
    let mut name = value.strip_prefix("VK_").unwrap_or(value).to_string();
    if let Some(rest) = name.strip_prefix(prefix.as_str()) {
        name = rest.to_string();
    }
    if let Some(rest) = name.strip_suffix("_BIT") {
        name = rest.to_string();
    } else if let Some(pos) = name.rfind("_BIT_") {
        let vendor = &name[pos + "_BIT_".len()..];
        if vendor_tags.iter().any(|t| t == vendor) {
            name = format!("{}_{}", &name[..pos], vendor);
        }
    }
    identifier(value, raw_ident(strip_tag_suffix(&name, tag)))
}

fn strip_tag_suffix<'a>(name: &'a str, tag: Option<&str>) -> &'a str {
    let Some(tag) = tag else { return name };
    match name.strip_suffix(tag) {
        Some(rest) if rest.ends_with('_') && rest.len() > 1 => &rest[..rest.len() - 1],
        _ => name,
    }
}

fn raw_ident(name: &str) -> String {
    if name.as_bytes().first().is_some_and(|b| b.is_ascii_digit()) {
        return format!("_{name}");
    }
    if name == "type" {
        return "ty".to_string();
    }
    if NOT_RAW_IDENTIFIERS.contains(&name) {
        return format!("{name}_");
    }
    if KEYWORDS.contains(&name) {
        return format!("r#{name}");
    }
    name.to_string()
}

fn to_snake(name: &str) -> String {
    let mut out = String::new();
    let chars = name.chars().collect::<Vec<_>>();
    for (i, ch) in chars.iter().copied().enumerate() {
        if ch.is_ascii_uppercase() {
            let prev = i.checked_sub(1).and_then(|i| chars.get(i)).copied();
            let next = chars.get(i + 1).copied();
            if i != 0
                && prev != Some('_')
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_support::{
        RUST_KEYWORDS, catch_quietly, deterministic_values, is_rust_identifier, proptest_config,
    };
    use proptest::prelude::*;
    use proptest::sample::select;
    use std::collections::BTreeMap;
    use vulkan_registry::Registry;

    fn tags() -> Vec<String> {
        ["KHR", "EXT", "NV", "NVX", "AMD", "QCOM", "ARM"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn capitalized(word: &str) -> String {
        let mut chars = word.chars();
        chars
            .next()
            .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
            .unwrap_or_default()
    }

    const API_PREFIXES: &[&str] = &["Vk", "vk", "VK_"];
    const DIGIT_LED_PIECES: &[&str] = &["2D", "3D", "8BIT", "420", "1", "16x16", "2_PLANE"];
    const TYPE_WORDS: &[&str] = &["Image", "View", "Type", "Sample", "Count", "H264", "Idc"];

    fn starts_with_a_digit(name: &str) -> bool {
        name.starts_with(|c: char| c.is_ascii_digit())
    }

    fn piece() -> impl Strategy<Value = String> {
        prop_oneof![
            2 => select(DIGIT_LED_PIECES).prop_map(str::to_string),
            1 => select(&["_BIT", "_EXTENSION_NAME", "KHR", "_KHR", "_EXT"][..])
                .prop_map(str::to_string),
            1 => select(RUST_KEYWORDS).prop_map(str::to_string),
            1 => select(RUST_KEYWORDS).prop_map(capitalized),
            1 => select(RUST_KEYWORDS).prop_map(str::to_ascii_uppercase),
            2 => "[A-Za-z0-9_]{1,4}",
        ]
    }

    fn c_identifier() -> impl Strategy<Value = String> {
        let head = prop_oneof![
            4 => select(API_PREFIXES).prop_map(str::to_string),
            1 => select(&["p", "pp", "PFN_", "_"][..]).prop_map(str::to_string),
            1 => select(RUST_KEYWORDS).prop_map(str::to_string),
            1 => select(RUST_KEYWORDS).prop_map(capitalized),
            1 => select(RUST_KEYWORDS).prop_map(str::to_ascii_uppercase),
            2 => "[A-Za-z_][A-Za-z0-9_]{0,3}",
        ];
        let any_shape = (head, proptest::collection::vec(piece(), 0..4))
            .prop_map(|(head, pieces)| head + &pieces.concat());
        let digit_after_the_prefix = (
            select(API_PREFIXES),
            select(DIGIT_LED_PIECES),
            proptest::collection::vec(piece(), 0..3),
        )
            .prop_map(|(prefix, digits, pieces)| format!("{prefix}{digits}{}", pieces.concat()));
        prop_oneof![4 => any_shape, 1 => digit_after_the_prefix]
    }

    #[derive(Clone, Debug)]
    struct TypeAndValue {
        type_name: String,
        value: String,
        enum_variant_starts_with_a_digit: bool,
        flag_bit_starts_with_a_digit: bool,
    }

    fn type_and_value() -> impl Strategy<Value = TypeAndValue> {
        let of_the_type = (
            select(&[("Vk", "VK_"), ("StdVideo", "STD_VIDEO_"), ("", "VK_")][..]),
            proptest::collection::vec(select(TYPE_WORDS), 1..4),
            select(&[("", ""), ("FlagBits", ""), ("FlagBits2", "2_")][..]),
            select(&["", "KHR", "EXT", "NVX"][..]),
            prop_oneof![select(DIGIT_LED_PIECES).prop_map(str::to_string), piece()],
            proptest::collection::vec(piece(), 0..2),
            select(&["", "_BIT", "_BIT_KHR", "_KHR", "_EXT", "_NVX"][..]),
        )
            .prop_map(
                |(
                    (type_prefix, value_prefix),
                    words,
                    (flag_bits, flag_bits_digits),
                    type_tag,
                    first_piece,
                    pieces,
                    value_suffix,
                )| {
                    let screaming = words
                        .iter()
                        .map(|word| word.to_ascii_uppercase())
                        .collect::<Vec<_>>()
                        .join("_");
                    TypeAndValue {
                        type_name: format!("{type_prefix}{}{flag_bits}{type_tag}", words.concat()),
                        value: format!(
                            "{value_prefix}{screaming}_{flag_bits_digits}{first_piece}{}{value_suffix}",
                            pieces.concat()
                        ),
                        enum_variant_starts_with_a_digit: flag_bits.is_empty()
                            && starts_with_a_digit(&first_piece),
                        flag_bit_starts_with_a_digit: starts_with_a_digit(&first_piece),
                    }
                },
            );
        let unrelated =
            (c_identifier(), c_identifier()).prop_map(|(type_name, value)| TypeAndValue {
                type_name,
                value,
                enum_variant_starts_with_a_digit: false,
                flag_bit_starts_with_a_digit: false,
            });
        prop_oneof![2 => of_the_type, 1 => unrelated]
    }

    fn assert_identifier_or_panic(function: &str, input: &str, convert: impl FnOnce() -> String) {
        if let Ok(name) = catch_quietly(convert) {
            assert!(
                is_rust_identifier(&name),
                "{function}({input:?}) returned `{name}`, which is not a Rust identifier"
            );
        }
    }

    #[test]
    fn raw_ident_turns_every_rust_keyword_into_an_identifier() {
        for keyword in RUST_KEYWORDS {
            let name = raw_ident(keyword);
            assert!(
                is_rust_identifier(&name),
                "raw_ident({keyword:?}) returned `{name}`, which is not a Rust identifier"
            );
        }
    }

    proptest! {
        #![proptest_config(proptest_config(512))]

        #[test]
        fn raw_ident_turns_every_c_identifier_into_a_rust_identifier(name in c_identifier()) {
            let rust = raw_ident(&name);
            prop_assert!(
                is_rust_identifier(&rust),
                "raw_ident({:?}) returned `{}`, which is not a Rust identifier", name, rust
            );
        }

        #[test]
        fn every_name_conversion_returns_a_rust_identifier_or_panics(
            a in c_identifier(),
            TypeAndValue { type_name: enum_name, value, .. } in type_and_value(),
        ) {
            let tags = tags();
            assert_identifier_or_panic("type_name", &a, || type_name(&a));
            assert_identifier_or_panic("func_pointer_name", &a, || func_pointer_name(&a));
            assert_identifier_or_panic("command_name", &a, || command_name(&a));
            assert_identifier_or_panic("command_const_name", &a, || command_const_name(&a));
            assert_identifier_or_panic("field_name", &a, || field_name(&a));
            assert_identifier_or_panic("const_name", &a, || const_name(&a));
            assert_identifier_or_panic("extension_name_const", &a, || extension_name_const(&a));
            assert_identifier_or_panic("pointer_setter_name", &a, || pointer_setter_name(&a));
            let pair = format!("{enum_name:?}, {value:?}");
            assert_identifier_or_panic("enum_variant_name", &pair, || {
                enum_variant_name(&enum_name, &value, &tags)
            });
            assert_identifier_or_panic("flag_bit_name", &pair, || {
                flag_bit_name(&enum_name, &value, &tags)
            });
        }

        #[test]
        fn is_identifier_agrees_with_the_rust_grammar(
            raw in select(&["", "r#"][..]),
            name in prop_oneof![
                select(RUST_KEYWORDS).prop_map(str::to_string),
                "[A-Za-z0-9_#.-]{0,4}",
            ],
        ) {
            let name = format!("{raw}{name}");
            prop_assert_eq!(
                is_identifier(&name),
                is_rust_identifier(&name),
                "is_identifier({:?}) contradicts the parser", name
            );
        }

        #[test]
        fn snake_case_joins_the_words_of_a_camel_case_name_with_underscores(
            words in camel_case_words(),
        ) {
            let lowercase = words
                .iter()
                .map(|word| word.to_ascii_lowercase())
                .collect::<Vec<_>>();
            prop_assert_eq!(to_snake(&words.concat()), lowercase.join("_"), "{:?}", words);
            prop_assert_eq!(
                to_screaming(&words.concat()),
                lowercase.join("_").to_ascii_uppercase(),
                "{:?}", words
            );
        }

        #[test]
        fn a_pointer_setter_is_named_like_the_member_without_its_hungarian_prefix(
            prefix in select(&["", "p", "pp", "ppp", "P", "s"][..]),
            rest in prop_oneof![
                Just(String::new()),
                "[A-Za-z0-9_]{1,2}",
                camel_case_words().prop_map(|words| words.concat()),
            ],
        ) {
            let name = format!("{prefix}{rest}");
            let starts_a_word = |rest: &&str| rest.starts_with(|c: char| c.is_ascii_uppercase());
            let member = name
                .strip_prefix("pp")
                .filter(starts_a_word)
                .or_else(|| name.strip_prefix('p').filter(starts_a_word))
                .unwrap_or(&name);
            prop_assert_eq!(
                catch_quietly(|| pointer_setter_name(&name)).ok(),
                catch_quietly(|| field_name(member)).ok(),
                "pointer_setter_name({:?}) is not field_name({:?})", name, member
            );
        }

        #[test]
        fn the_vendor_tag_of_a_name_is_the_longest_tag_that_ends_it_after_a_base(
            base in "[A-Za-z0-9]{0,2}",
            ending in select(&["", "R", "HR", "KHR", "X", "NVX", "T", "EXT"][..]),
            tags in Just(vec!["R", "HR", "KHR", "X", "NVX", "T", "EXT"]).prop_shuffle(),
            listed in 0..8usize,
        ) {
            let name = format!("{base}{ending}");
            let tags = tags
                .iter()
                .take(listed)
                .map(|tag| tag.to_string())
                .collect::<Vec<_>>();
            let longest = tags
                .iter()
                .filter(|tag| name.strip_suffix(tag.as_str()).is_some_and(|base| !base.is_empty()))
                .max_by_key(|tag| tag.len())
                .map(String::as_str);
            let (rest, tag) = split_vendor_tag(&name, &tags);
            prop_assert_eq!(tag, longest, "{:?} with the tags {:?}", name, tags);
            prop_assert_eq!(format!("{rest}{}", tag.unwrap_or("")), name);
        }

        #[test]
        fn a_flag_bit_loses_bit_at_its_end_or_before_a_vendor_tag_and_nowhere_else(
            name in "[A-Z][A-Z0-9]{0,3}(_[A-D0-9]{1,3}){0,1}",
            suffix in select(&["", "_KHR", "_EXT", "_NVX", "_DEPTH", "_KH", "_KHRX", "_8"][..]),
            type_tag in select(&["", "KHR", "EXT"][..]),
        ) {
            let tags = tags();
            let bitmask = format!("VkSampleCountFlagBits{type_tag}");
            let value = format!("VK_SAMPLE_COUNT_{name}_BIT{suffix}");
            let is_tag = tags.iter().any(|tag| suffix.strip_prefix('_') == Some(tag.as_str()));
            let expected = if suffix.is_empty() || suffix.strip_prefix('_') == Some(type_tag) {
                name.clone()
            } else if is_tag {
                format!("{name}{suffix}")
            } else {
                format!("{name}_BIT{suffix}")
            };
            prop_assert_eq!(
                flag_bit_name(&bitmask, &value, &tags),
                expected,
                "{:?} of {:?}", value, bitmask
            );
        }

        #[test]
        fn the_tag_of_the_type_is_dropped_from_a_value_only_after_an_underscore_and_a_name(
            stem in "[A-Z0-9]{0,3}",
            separator in select(&["", "_", "__"][..]),
            ending in select(&["KHR", "EXT", "HR", ""][..]),
            tag in proptest::option::of(select(&["KHR", "EXT"][..])),
        ) {
            let name = format!("{stem}{separator}{ending}");
            let expected = tag
                .and_then(|tag| name.strip_suffix(&format!("_{tag}")))
                .filter(|rest| !rest.is_empty())
                .unwrap_or(&name);
            prop_assert_eq!(strip_tag_suffix(&name, tag), expected, "{:?} with {:?}", name, tag);
        }
    }

    fn camel_case_words() -> impl Strategy<Value = Vec<String>> {
        let word = prop_oneof![
            3 => "[A-Z][a-z]{1,4}[0-9]{0,2}",
            1 => "[A-Z]{1,4}[0-9]{1,2}",
            1 => "[A-Z]{1,4}",
        ];
        (
            proptest::option::of("[a-z]{1,3}[0-9]{0,1}"),
            proptest::collection::vec(word, 0..5),
        )
            .prop_map(|(first, words)| {
                let mut kept = Vec::<String>::from_iter(first);
                for word in words {
                    let after_capitals = kept
                        .last()
                        .is_some_and(|last| last.ends_with(|c: char| c.is_ascii_uppercase()));
                    let all_capitals = !word.contains(|c: char| c.is_ascii_lowercase());
                    if !(after_capitals && all_capitals) {
                        kept.push(word);
                    }
                }
                kept
            })
    }

    #[test]
    fn the_generated_names_reach_every_outcome_of_every_conversion() {
        let tags = tags();
        let mut outcomes = BTreeMap::<(&str, &str), usize>::new();
        let mut count = |function: &'static str,
                         input: &str,
                         digit_led: bool,
                         result: Result<String, String>| {
            let outcome = match (&result, digit_led) {
                (Ok(name), true) if function == "type_name" => panic!(
                    "type_name({input}) returned `{name}` for a type that starts with a digit"
                ),
                (Ok(name), true) => {
                    assert!(
                        name.strip_prefix('_').is_some_and(starts_with_a_digit),
                        "{function}({input}) returned `{name}`, which is not the digit-led rest \
                         of the C name after one underscore"
                    );
                    "digit-led"
                }
                (Err(_), true) if function == "type_name" => "digit-led panic",
                (Err(message), true) => panic!(
                    "{function}({input}) rejects a name that starts with a digit after its \
                     prefix: {message}"
                ),
                (Ok(_), false) => "identifier",
                (Err(_), false) => "panic",
            };
            *outcomes.entry((function, outcome)).or_default() += 1;
        };
        for (name, pair) in deterministic_values((c_identifier(), type_and_value()), 512) {
            let digit_led_after =
                |prefix: &str| name.strip_prefix(prefix).is_some_and(starts_with_a_digit);
            count(
                "type_name",
                &name,
                digit_led_after("Vk"),
                catch_quietly(|| type_name(&name)),
            );
            count(
                "func_pointer_name",
                &name,
                false,
                catch_quietly(|| func_pointer_name(&name)),
            );
            count(
                "field_name",
                &name,
                false,
                catch_quietly(|| field_name(&name)),
            );
            count(
                "pointer_setter_name",
                &name,
                false,
                catch_quietly(|| pointer_setter_name(&name)),
            );
            count(
                "command_name",
                &name,
                digit_led_after("vk"),
                catch_quietly(|| command_name(&name)),
            );
            count(
                "command_const_name",
                &name,
                digit_led_after("vk"),
                catch_quietly(|| command_const_name(&name)),
            );
            count(
                "const_name",
                &name,
                digit_led_after("VK_"),
                catch_quietly(|| const_name(&name)),
            );
            count(
                "extension_name_const",
                &name,
                digit_led_after("VK_"),
                catch_quietly(|| extension_name_const(&name)),
            );
            let input = format!("{:?}, {:?}", pair.type_name, pair.value);
            count(
                "enum_variant_name",
                &input,
                pair.enum_variant_starts_with_a_digit,
                catch_quietly(|| enum_variant_name(&pair.type_name, &pair.value, &tags)),
            );
            count(
                "flag_bit_name",
                &input,
                pair.flag_bit_starts_with_a_digit,
                catch_quietly(|| flag_bit_name(&pair.type_name, &pair.value, &tags)),
            );
        }
        for (function, outcome, at_least) in [
            ("type_name", "identifier", 100),
            ("type_name", "panic", 20),
            ("type_name", "digit-led panic", 30),
            ("func_pointer_name", "identifier", 100),
            ("func_pointer_name", "panic", 10),
            ("field_name", "identifier", 100),
            ("pointer_setter_name", "identifier", 100),
            ("command_name", "identifier", 100),
            ("command_name", "digit-led", 30),
            ("command_const_name", "identifier", 100),
            ("command_const_name", "digit-led", 30),
            ("const_name", "identifier", 100),
            ("const_name", "digit-led", 30),
            ("extension_name_const", "identifier", 100),
            ("extension_name_const", "digit-led", 30),
            ("enum_variant_name", "identifier", 100),
            ("enum_variant_name", "digit-led", 30),
            ("flag_bit_name", "identifier", 100),
            ("flag_bit_name", "digit-led", 30),
        ] {
            let reached = outcomes.get(&(function, outcome)).copied().unwrap_or(0);
            assert!(
                reached >= at_least,
                "only {reached} of 512 generated names reach the outcome `{outcome}` of \
                 {function}: {outcomes:?}"
            );
        }
    }

    #[test]
    fn names_that_are_not_identifiers_panic_with_the_c_name() {
        type Convert = fn() -> String;
        let cases: [(&str, Convert); 6] = [
            ("`Vk2D`", || type_name("Vk2D")),
            ("`VkSelf`", || type_name("VkSelf")),
            ("`vk`", || command_name("vk")),
            ("`VK_`", || const_name("VK_")),
            ("`VK_FOO_`", || enum_variant_name("VkFoo", "VK_FOO_", &[])),
            ("`naïve`", || field_name("naïve")),
        ];
        for (c_name, convert) in cases {
            let message =
                catch_quietly(convert).expect_err(&format!("converting {c_name} does not panic"));
            assert!(
                message.contains(c_name) && message.contains("not a valid Rust identifier"),
                "the panic message `{message}` does not name {c_name}"
            );
        }
    }

    #[test]
    fn every_registry_name_converts_to_a_rust_identifier() {
        let ir = Registry::get();
        let mut converted = Vec::new();
        let types = ir
            .handles
            .iter()
            .flat_map(|h| std::iter::once(&h.name).chain(&h.aliases))
            .chain(
                ir.enums
                    .iter()
                    .flat_map(|e| std::iter::once(&e.name).chain(&e.aliases)),
            )
            .chain(
                ir.bitmasks
                    .iter()
                    .flat_map(|b| [&b.name, &b.flags_name].into_iter().chain(&b.aliases)),
            )
            .chain(
                ir.flags
                    .iter()
                    .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases)),
            )
            .chain(
                ir.structs
                    .iter()
                    .flat_map(|s| std::iter::once(&s.name).chain(&s.aliases)),
            );
        for name in types {
            converted.push((name.clone(), type_name(name)));
        }
        for fp in &ir.func_pointers {
            converted.push((fp.name.clone(), func_pointer_name(&fp.name)));
            for p in &fp.params {
                converted.push((p.name.clone(), field_name(&p.name)));
            }
        }
        for c in &ir.commands {
            converted.push((c.name.clone(), command_name(&c.name)));
            converted.push((c.name.clone(), command_const_name(&c.name)));
            let pfn = format!("PFN_{}", c.name);
            converted.push((pfn.clone(), func_pointer_name(&pfn)));
        }
        let fields = ir
            .structs
            .iter()
            .flat_map(|s| &s.members)
            .chain(ir.commands.iter().flat_map(|c| &c.params));
        for f in fields {
            converted.push((f.name.clone(), field_name(&f.name)));
            converted.push((f.name.clone(), pointer_setter_name(&f.name)));
            for len in f
                .fixed_size_array
                .iter()
                .filter(|len| len.starts_with("VK_"))
            {
                converted.push((len.clone(), const_name(len)));
            }
        }
        let constants = ir
            .constants
            .iter()
            .map(|c| &c.name)
            .chain(ir.versions.iter().map(|v| &v.name_api))
            .chain(ir.extensions.iter().map(|e| &e.spec_version_macro));
        for name in constants {
            converted.push((name.clone(), const_name(name)));
        }
        for e in &ir.extensions {
            converted.push((e.name_macro.clone(), extension_name_const(&e.name_macro)));
        }
        for e in &ir.enums {
            for value in e
                .fields
                .iter()
                .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases))
            {
                converted.push((
                    value.clone(),
                    enum_variant_name(&e.name, value, &ir.vendor_tags),
                ));
            }
        }
        for b in &ir.bitmasks {
            for value in b
                .fields
                .iter()
                .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases))
            {
                converted.push((
                    value.clone(),
                    flag_bit_name(&b.name, value, &ir.vendor_tags),
                ));
            }
        }
        assert!(
            converted.len() > 10_000,
            "the registry has {} names",
            converted.len()
        );
        for (c_name, rust_name) in converted {
            assert!(
                is_rust_identifier(&rust_name),
                "the C name `{c_name}` becomes `{rust_name}`, which is not a Rust identifier"
            );
        }
    }

    #[test]
    fn type_and_command_names() {
        assert_eq!(type_name("VkBufferCreateInfo"), "BufferCreateInfo");
        assert_eq!(type_name("VkResult"), "VkResult");
        assert_eq!(type_name("StdVideoH264SpsFlags"), "StdVideoH264SpsFlags");
        assert_eq!(command_name("vkCreateBuffer"), "create_buffer");
        assert_eq!(
            command_name("vkGetPhysicalDeviceSurfaceFormats2KHR"),
            "get_physical_device_surface_formats2_khr"
        );
        assert_eq!(command_name("vkCmdDrawIndexed"), "cmd_draw_indexed");
    }

    #[test]
    fn field_names_escape_keywords() {
        assert_eq!(field_name("sType"), "s_type");
        assert_eq!(field_name("pNext"), "p_next");
        assert_eq!(field_name("type"), "ty");
        assert_eq!(field_name("match"), "r#match");
        assert_eq!(field_name("ppEnabledLayerNames"), "pp_enabled_layer_names");
        assert_eq!(field_name("ScalingList4x4"), "scaling_list4x4");
        assert_eq!(field_name("self"), "self_");
    }

    #[test]
    fn pointer_setter_names_drop_hungarian_prefix() {
        assert_eq!(
            pointer_setter_name("pQueueCreateInfos"),
            "queue_create_infos"
        );
        assert_eq!(
            pointer_setter_name("ppEnabledLayerNames"),
            "enabled_layer_names"
        );
        assert_eq!(pointer_setter_name("pipeline"), "pipeline");
        assert_eq!(pointer_setter_name("display"), "display");
    }

    #[test]
    fn enum_variants_drop_type_prefix_and_own_vendor_tag() {
        let tags = tags();
        assert_eq!(
            enum_variant_name("VkPresentModeKHR", "VK_PRESENT_MODE_IMMEDIATE_KHR", &tags),
            "IMMEDIATE"
        );
        assert_eq!(
            enum_variant_name(
                "VkPresentModeKHR",
                "VK_PRESENT_MODE_FIFO_LATEST_READY_EXT",
                &tags
            ),
            "FIFO_LATEST_READY_EXT"
        );
        assert_eq!(
            enum_variant_name("VkColorSpaceKHR", "VK_COLOR_SPACE_HDR10_ST2084_EXT", &tags),
            "HDR10_ST2084_EXT"
        );
        assert_eq!(
            enum_variant_name("VkFormat", "VK_FORMAT_R8G8B8A8_UNORM", &tags),
            "R8G8B8A8_UNORM"
        );
        assert_eq!(
            enum_variant_name("VkResult", "VK_ERROR_OUT_OF_DATE_KHR", &tags),
            "ERROR_OUT_OF_DATE_KHR"
        );
        assert_eq!(
            enum_variant_name("VkImageViewType", "VK_IMAGE_VIEW_TYPE_2D", &tags),
            "_2D"
        );
        assert_eq!(
            enum_variant_name(
                "StdVideoH264ChromaFormatIdc",
                "STD_VIDEO_H264_CHROMA_FORMAT_IDC_420",
                &tags
            ),
            "_420"
        );
    }

    #[test]
    fn flag_bits_drop_bit_and_own_vendor_tag() {
        let tags = tags();
        assert_eq!(
            flag_bit_name(
                "VkImageUsageFlagBits",
                "VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT",
                &tags
            ),
            "COLOR_ATTACHMENT"
        );
        assert_eq!(
            flag_bit_name(
                "VkBufferUsageFlagBits",
                "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR",
                &tags
            ),
            "SHADER_DEVICE_ADDRESS_KHR"
        );
        assert_eq!(
            flag_bit_name("VkAccessFlagBits2", "VK_ACCESS_2_SHADER_READ_BIT", &tags),
            "SHADER_READ"
        );
        assert_eq!(
            flag_bit_name(
                "VkCompositeAlphaFlagBitsKHR",
                "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR",
                &tags
            ),
            "OPAQUE"
        );
        assert_eq!(
            flag_bit_name(
                "VkDebugUtilsMessageSeverityFlagBitsEXT",
                "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT",
                &tags
            ),
            "ERROR"
        );
        assert_eq!(
            flag_bit_name("VkSampleCountFlagBits", "VK_SAMPLE_COUNT_1_BIT", &tags),
            "_1"
        );
        assert_eq!(
            flag_bit_name("VkCullModeFlagBits", "VK_CULL_MODE_FRONT_AND_BACK", &tags),
            "FRONT_AND_BACK"
        );
        assert_eq!(
            flag_bit_name(
                "VkExternalMemoryHandleTypeFlagBits",
                "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT",
                &tags
            ),
            "DMA_BUF_EXT"
        );
    }
}
