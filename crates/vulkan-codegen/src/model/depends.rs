#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Depends {
    Version(u32, u32),
    Extension(String),
    All(Vec<Depends>),
    Any(Vec<Depends>),
}

impl Depends {
    pub(crate) fn parse(text: &str) -> Self {
        let tokens = Self::tokens(text);
        let mut position = 0;
        let parsed = Self::expression(&tokens, &mut position, text);
        assert_eq!(position, tokens.len(), "trailing input in `{text}`");
        parsed
    }

    fn tokens(text: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut name = String::new();
        for ch in text.chars() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                continue;
            }
            if !name.is_empty() {
                tokens.push(std::mem::take(&mut name));
            }
            match ch {
                '(' | ')' | '+' | ',' => tokens.push(ch.to_string()),
                ch if ch.is_whitespace() => {}
                other => panic!("unexpected `{other}` in dependency `{text}`"),
            }
        }
        if !name.is_empty() {
            tokens.push(name);
        }
        tokens
    }

    fn expression(tokens: &[String], position: &mut usize, text: &str) -> Self {
        let mut left = Self::term(tokens, position, text);
        while let Some(op) = tokens.get(*position).filter(|t| *t == "+" || *t == ",") {
            *position += 1;
            let right = Self::term(tokens, position, text);
            left = match (op.as_str(), left) {
                ("+", Depends::All(mut all)) => {
                    all.push(right);
                    Depends::All(all)
                }
                (",", Depends::Any(mut any)) => {
                    any.push(right);
                    Depends::Any(any)
                }
                ("+", left) => Depends::All(vec![left, right]),
                (_, left) => Depends::Any(vec![left, right]),
            };
        }
        left
    }

    fn term(tokens: &[String], position: &mut usize, text: &str) -> Self {
        let token = tokens
            .get(*position)
            .unwrap_or_else(|| panic!("unexpected end of dependency `{text}`"));
        *position += 1;
        if token == "(" {
            let inner = Self::expression(tokens, position, text);
            assert_eq!(
                tokens.get(*position).map(String::as_str),
                Some(")"),
                "missing `)` in `{text}`"
            );
            *position += 1;
            return inner;
        }
        if let Some((major, minor)) = named_version(token, || format!("dependency `{text}`")) {
            return Depends::Version(major, minor);
        }
        assert!(
            token.starts_with("VK_"),
            "unexpected name `{token}` in dependency `{text}`"
        );
        Depends::Extension(token.clone())
    }

    pub(crate) fn extensions(&self) -> Vec<&str> {
        match self {
            Depends::Version(..) => Vec::new(),
            Depends::Extension(name) => vec![name],
            Depends::All(items) | Depends::Any(items) => {
                items.iter().flat_map(Depends::extensions).collect()
            }
        }
    }
}

const MAX_MAJOR: u32 = (1 << 7) - 1;
const MAX_MINOR: u32 = (1 << 10) - 1;

pub(crate) fn parse_version(name: &str) -> Option<(u32, u32)> {
    let (major, minor) = name.strip_prefix("VK_VERSION_")?.split_once('_')?;
    let (major, minor) = (major.parse::<u32>().ok()?, minor.parse::<u32>().ok()?);
    (format!("VK_VERSION_{major}_{minor}") == name && major <= MAX_MAJOR && minor <= MAX_MINOR)
        .then_some((major, minor))
}

pub(crate) fn named_version(name: &str, used_in: impl FnOnce() -> String) -> Option<(u32, u32)> {
    if !name.starts_with("VK_VERSION_") {
        return None;
    }
    let version = parse_version(name).unwrap_or_else(|| {
        panic!(
            "`{name}` in {} is not a version name of the form `VK_VERSION_<major>_<minor>` \
             with a major version below {} and a minor version below {}",
            used_in(),
            MAX_MAJOR + 1,
            MAX_MINOR + 1
        )
    });
    Some(version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{catch_quietly, proptest_config};
    use proptest::prelude::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use vulkan_registry::Registry;

    #[test]
    fn dependency_expressions_associate_left_with_equal_precedence() {
        let ext = |name: &str| Depends::Extension(name.to_string());
        assert_eq!(Depends::parse("VK_KHR_surface"), ext("VK_KHR_surface"));
        assert_eq!(
            Depends::parse("VK_KHR_a+VK_KHR_b+VK_KHR_c"),
            Depends::All(vec![ext("VK_KHR_a"), ext("VK_KHR_b"), ext("VK_KHR_c")])
        );
        assert_eq!(
            Depends::parse("VK_KHR_a+VK_KHR_b,VK_VERSION_1_1"),
            Depends::Any(vec![
                Depends::All(vec![ext("VK_KHR_a"), ext("VK_KHR_b")]),
                Depends::Version(1, 1),
            ])
        );
        assert_eq!(
            Depends::parse("VK_KHR_a,VK_KHR_b+VK_KHR_c"),
            Depends::All(vec![
                Depends::Any(vec![ext("VK_KHR_a"), ext("VK_KHR_b")]),
                ext("VK_KHR_c"),
            ])
        );
        assert_eq!(
            Depends::parse(
                "((VK_KHR_extended_flags,VK_KHR_maintenance5)+(VK_KHR_buffer_device_address,VK_VERSION_1_2),VK_VERSION_1_4)"
            ),
            Depends::Any(vec![
                Depends::All(vec![
                    Depends::Any(vec![
                        ext("VK_KHR_extended_flags"),
                        ext("VK_KHR_maintenance5")
                    ]),
                    Depends::Any(vec![
                        ext("VK_KHR_buffer_device_address"),
                        Depends::Version(1, 2)
                    ]),
                ]),
                Depends::Version(1, 4),
            ])
        );
    }

    #[test]
    #[should_panic(expected = "missing `)`")]
    fn unbalanced_dependency_expressions_abort_generation() {
        Depends::parse("(VK_KHR_a+VK_KHR_b");
    }

    #[test]
    #[should_panic(
        expected = "`VK_VERSION_1_2_3` in dependency `VK_KHR_a+VK_VERSION_1_2_3` is not a version name of the form `VK_VERSION_<major>_<minor>`"
    )]
    fn a_version_name_that_is_not_a_version_aborts_generation() {
        Depends::parse("VK_KHR_a+VK_VERSION_1_2_3");
    }

    #[test]
    fn only_canonical_version_names_that_fit_an_api_version_are_versions() {
        for (name, version) in [
            ("VK_VERSION_1_0", (1, 0)),
            ("VK_VERSION_1_1", (1, 1)),
            ("VK_VERSION_1_2", (1, 2)),
            ("VK_VERSION_1_3", (1, 3)),
            ("VK_VERSION_1_4", (1, 4)),
            ("VK_VERSION_0_0", (0, 0)),
            ("VK_VERSION_127_1023", (127, 1023)),
        ] {
            assert_eq!(parse_version(name), Some(version), "{name}");
        }
        for name in [
            "VK_VERSION_+1_2",
            "VK_VERSION_1_+2",
            "VK_VERSION_01_2",
            "VK_VERSION_1_02",
            "VK_VERSION_1_1024",
            "VK_VERSION_128_0",
            "VK_VERSION_4294967296_0",
            "VK_VERSION_1_2_3",
            "VK_VERSION_1",
            "VK_VERSION_",
            "VK_KHR_surface",
        ] {
            assert_eq!(parse_version(name), None, "{name}");
        }
    }

    #[test]
    fn names_that_are_not_version_names_are_not_versions() {
        assert_eq!(named_version("VK_KHR_surface", || unreachable!()), None);
        assert_eq!(named_version("VKSC_VERSION_1_0", || unreachable!()), None);
        assert_eq!(
            named_version("VK_VERSION_1_3", || unreachable!()),
            Some((1, 3))
        );
        let message = catch_quietly(|| named_version("VK_VERSION_128_0", || "a test".to_string()))
            .expect_err("an out-of-range major version is accepted");
        assert!(
            message.contains("`VK_VERSION_128_0` in a test is not a version name"),
            "{message}"
        );
    }

    const MALFORMED_NAMES: &[&str] = &[
        "VK_VERSION_1",
        "VK_VERSION_1_2_3",
        "VK_VERSION_x_y",
        "VK_VERSION_1_x",
        "VK_VERSION_1_",
        "VK_VERSION__1",
        "VK_VERSION_",
        "VK_VERSION_99999999999_0",
        "VK_VERSION_01_2",
        "VK_VERSION_1_1024",
        "VK_VERSION_128_0",
        "VKSC_VERSION_1_0",
        "vk_KHR_surface",
        "KHR_surface",
        "1_2",
    ];

    fn leaf() -> impl Strategy<Value = Depends> {
        prop_oneof![
            (0u32..4, 0u32..12).prop_map(|(major, minor)| Depends::Version(major, minor)),
            "VK_[A-Z]{2,4}_[a-z0-9_]{1,8}".prop_map(Depends::Extension),
        ]
    }

    fn tree() -> impl Strategy<Value = Depends> {
        leaf().prop_recursive(4, 32, 4, |inner| {
            (any::<bool>(), proptest::collection::vec(inner, 2..5)).prop_map(|(all, children)| {
                if all {
                    Depends::All(children)
                } else {
                    Depends::Any(children)
                }
            })
        })
    }

    fn left_associated(tree: Depends) -> Depends {
        let (is_all, children) = match tree {
            Depends::All(children) => (true, children),
            Depends::Any(children) => (false, children),
            leaf => return leaf,
        };
        let mut children = children.into_iter().map(left_associated);
        let mut flat = match children.next().expect("compound expressions have children") {
            Depends::All(first) if is_all => first,
            Depends::Any(first) if !is_all => first,
            first => vec![first],
        };
        flat.extend(children);
        if is_all {
            Depends::All(flat)
        } else {
            Depends::Any(flat)
        }
    }

    struct Printer<'c> {
        choices: &'c [u8],
        next: usize,
        keeps_structure: bool,
        tokens: Vec<String>,
    }

    impl<'c> Printer<'c> {
        fn tokens(tree: &Depends, choices: &'c [u8], keeps_structure: bool) -> Vec<String> {
            let mut printer = Printer {
                choices,
                next: 0,
                keeps_structure,
                tokens: Vec::new(),
            };
            printer.print(tree, false);
            printer.tokens
        }

        fn choice(&mut self, range: u8) -> u8 {
            let choice = self.choices[self.next % self.choices.len()];
            self.next += 1;
            choice % range
        }

        fn print(&mut self, tree: &Depends, needs_parentheses: bool) {
            let parenthesized = (needs_parentheses && self.keeps_structure) || self.choice(4) == 0;
            if parenthesized {
                self.tokens.push("(".to_string());
            }
            match tree {
                Depends::Version(major, minor) => {
                    self.tokens.push(format!("VK_VERSION_{major}_{minor}"))
                }
                Depends::Extension(name) => self.tokens.push(name.clone()),
                Depends::All(children) | Depends::Any(children) => {
                    let op = if matches!(tree, Depends::All(_)) {
                        "+"
                    } else {
                        ","
                    };
                    for (index, child) in children.iter().enumerate() {
                        if index > 0 {
                            self.tokens.push(op.to_string());
                        }
                        let compound = matches!(child, Depends::All(_) | Depends::Any(_));
                        self.print(child, index > 0 && compound);
                    }
                }
            }
            if parenthesized {
                self.tokens.push(")".to_string());
            }
        }
    }

    fn spaced(tokens: &[String], choices: &[u8]) -> String {
        const SPACES: [&str; 5] = ["", " ", "  ", "\t", "\n"];
        let mut text = String::new();
        for (index, token) in tokens.iter().enumerate() {
            text.push_str(SPACES[choices[index % choices.len()] as usize % SPACES.len()]);
            text.push_str(token);
        }
        text.push_str(SPACES[choices[tokens.len() % choices.len()] as usize % SPACES.len()]);
        text
    }

    fn truth(seed: u64) -> impl Fn(&str) -> bool {
        move |name| {
            let mut hasher = DefaultHasher::new();
            (seed, name).hash(&mut hasher);
            hasher.finish() & 1 == 1
        }
    }

    fn evaluate(tree: &Depends, truth: &impl Fn(&str) -> bool) -> bool {
        match tree {
            Depends::Version(major, minor) => truth(&format!("VK_VERSION_{major}_{minor}")),
            Depends::Extension(name) => truth(name),
            Depends::All(children) => children.iter().all(|child| evaluate(child, truth)),
            Depends::Any(children) => children.iter().any(|child| evaluate(child, truth)),
        }
    }

    fn evaluate_left_to_right(text: &str, truth: &impl Fn(&str) -> bool) -> bool {
        fn combine(frame: &mut (Option<bool>, Option<char>), value: bool) {
            frame.0 = Some(match *frame {
                (None, _) => value,
                (Some(left), Some('+')) => left && value,
                (Some(left), Some(',')) => left || value,
                (Some(_), op) => panic!("no operator before a term: {op:?}"),
            });
        }
        let mut frames = vec![(None, None)];
        let mut name = String::new();
        for ch in text.chars().chain([' ']) {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                name.push(ch);
                continue;
            }
            if !name.is_empty() {
                combine(frames.last_mut().expect("a frame"), truth(&name));
                name.clear();
            }
            match ch {
                '(' => frames.push((None, None)),
                ')' => {
                    let (value, _) = frames.pop().expect("a frame");
                    combine(frames.last_mut().expect("a frame"), value.expect("a value"));
                }
                '+' | ',' => frames.last_mut().expect("a frame").1 = Some(ch),
                _ => {}
            }
        }
        frames.pop().and_then(|(value, _)| value).expect("a value")
    }

    fn mutated(tokens: &[String], kind: u8, at: usize, replacement: &str) -> Vec<String> {
        let mut tokens = tokens.to_vec();
        let index = at % tokens.len();
        let names = tokens
            .iter()
            .enumerate()
            .filter(|(_, token)| token.starts_with("VK_"))
            .map(|(index, _)| index)
            .collect::<Vec<_>>();
        let boundary = at % (tokens.len() + 1);
        match kind % 6 {
            0 => {
                tokens.remove(index);
            }
            1 => tokens.insert(boundary, ["+", ","][at % 2].to_string()),
            2 => tokens.insert(boundary, "VK_KHR_inserted".to_string()),
            3 => tokens.insert(boundary, ["(", ")"][at % 2].to_string()),
            4 => tokens[names[at % names.len()]] = replacement.to_string(),
            _ => tokens[index] = ["&", "|", "!", "-", ";"][at % 5].to_string(),
        }
        tokens
    }

    proptest! {
        #![proptest_config(proptest_config(2048))]

        #[test]
        fn printed_expressions_parse_back_to_the_same_tree(
            tree in tree(),
            choices in proptest::collection::vec(any::<u8>(), 1..64),
        ) {
            let tree = left_associated(tree);
            let text = spaced(&Printer::tokens(&tree, &choices, true), &choices);
            prop_assert_eq!(Depends::parse(&text), tree, "parsing {:?}", text);
        }

        #[test]
        fn parsed_expressions_evaluate_left_to_right_with_equal_precedence(
            tree in tree(),
            choices in proptest::collection::vec(any::<u8>(), 1..64),
            seed in any::<u64>(),
        ) {
            let text = spaced(&Printer::tokens(&tree, &choices, false), &choices);
            let truth = truth(seed);
            prop_assert_eq!(
                evaluate(&Depends::parse(&text), &truth),
                evaluate_left_to_right(&text, &truth),
                "evaluating {:?}", text
            );
        }

        #[test]
        fn expressions_with_one_error_abort_generation(
            tree in tree(),
            choices in proptest::collection::vec(any::<u8>(), 1..64),
            kind in any::<u8>(),
            at in any::<usize>(),
            replacement in proptest::sample::select(MALFORMED_NAMES),
        ) {
            let tokens = mutated(&Printer::tokens(&tree, &choices, true), kind, at, replacement);
            let text = tokens.join(" ");
            prop_assert!(
                catch_quietly(|| Depends::parse(&text)).is_err(),
                "`{}` is accepted", text
            );
        }
    }

    #[test]
    fn a_character_that_no_dependency_expression_has_aborts_generation_and_is_named() {
        for unexpected in ['&', '|', '-', '.', ':', '!', '[', '"', '*', 'é'] {
            let text = format!("VK_A{unexpected}VK_B");
            assert_eq!(
                catch_quietly(|| Depends::parse(&text)),
                Err(format!("unexpected `{unexpected}` in dependency `{text}`"))
            );
        }
        for space in [' ', '\t', '\n', '\u{a0}'] {
            let text = format!("{space}VK_A{space}+{space}VK_B{space}");
            assert_eq!(
                Depends::parse(&text),
                Depends::All(vec![
                    Depends::Extension("VK_A".to_string()),
                    Depends::Extension("VK_B".to_string())
                ])
            );
        }
    }

    #[test]
    fn empty_dependencies_abort_generation() {
        for text in ["", " ", "()", "(())"] {
            assert!(
                catch_quietly(|| Depends::parse(text)).is_err(),
                "`{text}` is accepted"
            );
        }
    }

    #[test]
    fn every_dependency_of_the_registry_parses_and_evaluates_left_to_right() {
        let depends = Registry::get()
            .extensions
            .iter()
            .filter_map(|e| e.depends.as_deref())
            .collect::<Vec<_>>();
        assert!(
            depends.len() > 300,
            "the registry has {} dependencies",
            depends.len()
        );
        for text in depends {
            let parsed = Depends::parse(text);
            for seed in 0..16 {
                let truth = truth(seed);
                assert_eq!(
                    evaluate(&parsed, &truth),
                    evaluate_left_to_right(text, &truth),
                    "evaluating `{text}`"
                );
            }
        }
    }
}
