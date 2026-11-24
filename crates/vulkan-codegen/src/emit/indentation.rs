const INDENT: &str = "    ";

pub(crate) fn indent_by_nesting(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut depth = 0usize;
    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() {
            out.push('\n');
            continue;
        }
        let unbalanced = || panic!("`{line}` closes a bracket that was never opened");
        let brackets = Brackets::count(line);
        let line_depth = depth
            .checked_sub(brackets.leading_closers)
            .unwrap_or_else(unbalanced);
        depth = (depth + brackets.openers)
            .checked_sub(brackets.closers)
            .unwrap_or_else(unbalanced);
        out.push_str(&INDENT.repeat(line_depth));
        out.push_str(line);
        out.push('\n');
    }
    assert_eq!(depth, 0, "the generated source leaves a bracket open");
    out
}

#[derive(Default)]
struct Brackets {
    openers: usize,
    closers: usize,
    leading_closers: usize,
}

impl Brackets {
    fn count(line: &str) -> Self {
        let is_closer = |c: char| matches!(c, '}' | ']' | ')');
        let mut brackets = Brackets {
            leading_closers: line.chars().take_while(|c| is_closer(*c)).count(),
            ..Default::default()
        };
        let mut rest = line;
        while let Some(c) = rest.chars().next() {
            rest = &rest[c.len_utf8()..];
            match c {
                '"' => {
                    rest = after_string_literal(rest)
                        .unwrap_or_else(|| panic!("`{line}` ends inside a string literal"));
                }
                '\'' => rest = after_character_literal(rest).unwrap_or(rest),
                '/' if rest.starts_with('/') => break,
                '{' | '[' | '(' => brackets.openers += 1,
                c if is_closer(c) => brackets.closers += 1,
                _ => {}
            }
        }
        brackets
    }
}

fn after_string_literal(text: &str) -> Option<&str> {
    let mut chars = text.char_indices();
    while let Some((at, c)) = chars.next() {
        match c {
            '\\' => {
                chars.next();
            }
            '"' => return Some(&text[at + 1..]),
            _ => {}
        }
    }
    None
}

fn after_character_literal(text: &str) -> Option<&str> {
    let mut chars = text.char_indices();
    let (_, first) = chars.next()?;
    let (second_at, second) = chars.next()?;
    match (first, second) {
        ('\\', _) => chars
            .find(|(_, c)| *c == '\'')
            .map(|(at, _)| &text[at + 1..]),
        ('\'', _) => None,
        (_, '\'') => Some(&text[second_at + 1..]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{catch_quietly, deterministic_values, proptest_config};
    use proptest::prelude::*;
    use proptest::sample::select;
    use std::collections::BTreeMap;

    #[test]
    fn a_bracket_that_closes_nothing_aborts_generation_and_names_the_line() {
        for (source, line) in [
            ("}\n", "`}`"),
            ("f(a) + b)\n", "`f(a) + b)`"),
            ("{\n}\n  ] // x\n", "`] // x`"),
            ("{\n1 }, 2 }\n}\n", "`1 }, 2 }`"),
        ] {
            let message = catch_quietly(|| indent_by_nesting(source))
                .expect_err(&format!("{source:?} is indented"));
            assert!(
                message.contains(line)
                    && message.contains("closes a bracket that was never opened"),
                "the message `{message}` for {source:?} does not name the line {line}"
            );
        }
    }

    #[test]
    fn a_double_quote_or_a_bracket_in_a_character_literal_is_neither_a_string_nor_a_bracket() {
        for line in [
            "let quote = '\"';",
            "let quotes = ['\"', '\\'', '\"'];",
            "m('{', \"x{\", '(')",
            "matches!(c, 'a'|'{'|'\\n'|'('|'\\u{7b}'|'é'|']')",
        ] {
            assert_eq!(
                indent_by_nesting(&format!("{{\n{line}\n}}\n")),
                format!("{{\n    {line}\n}}\n")
            );
        }
    }

    #[test]
    fn a_character_literal_ends_at_the_quote_after_one_character_or_after_an_escape() {
        for (after_the_quote, rest) in [
            ("a' x", Some(" x")),
            ("\"'(", Some("(")),
            ("é'[", Some("[")),
            ("\\'' x", Some(" x")),
            ("\\\\' x", Some(" x")),
            ("\\n'", Some("")),
            ("\\u{7b}'}", Some("}")),
            ("a> x", None),
            ("a, 'b> x", None),
            ("_ x", None),
            ("a", None),
            ("", None),
            ("'' x", None),
            ("\\", None),
            ("\\n x", None),
        ] {
            assert_eq!(
                after_character_literal(after_the_quote),
                rest,
                "{after_the_quote:?}"
            );
        }
    }

    #[test]
    fn a_string_literal_ends_at_the_first_quote_that_no_backslash_escapes() {
        for (after_the_quote, rest) in [
            ("\" x", Some(" x")),
            ("a{b\"}", Some("}")),
            ("a\\\"b\" x", Some(" x")),
            ("a\\\\\" x", Some(" x")),
            ("é\"\"", Some("\"")),
            ("a", None),
            ("a\\\"", None),
            ("a\\", None),
            ("", None),
        ] {
            assert_eq!(
                after_string_literal(after_the_quote),
                rest,
                "{after_the_quote:?}"
            );
        }
    }

    #[test]
    fn a_lifetime_starts_no_character_literal() {
        assert_eq!(
            indent_by_nesting("fn f<'a>(x: &'a [u8]) -> &'a str {\nx('a', 'b')\n}\n"),
            "fn f<'a>(x: &'a [u8]) -> &'a str {\n    x('a', 'b')\n}\n"
        );
        assert_eq!(
            indent_by_nesting(
                "impl<'a> A<'a> for B<'_> {\nfn g(&'a self) {\n'l: loop {\n}\n}\n}\n"
            ),
            "impl<'a> A<'a> for B<'_> {\n    fn g(&'a self) {\n        'l: loop {\n        }\n    }\n}\n"
        );
    }

    #[test]
    fn a_line_that_ends_inside_a_string_literal_aborts_generation_and_names_the_line() {
        for (source, line) in [
            ("f(\"a\n\");\n", "`f(\"a`"),
            ("let a = \"b\\\";\n", "`let a = \"b\\\";`"),
        ] {
            let message = catch_quietly(|| indent_by_nesting(source))
                .expect_err(&format!("{source:?} is indented"));
            assert!(
                message.contains(line) && message.contains("ends inside a string literal"),
                "the message `{message}` for {source:?} does not name the line {line}"
            );
        }
    }

    const WITHOUT_OPEN_BRACKETS: &[&str] = &[
        "a",
        "else",
        "let x = a / b;",
        "x = a / b +",
        "g(|x| { x[0] })",
        "#[cfg(feature = \"beta\")]",
        "write!(f, \"A({:#x}) \\\" }\", self.0)",
        "s(\"a/b}\")",
        "s(\"http://x/(y\", \"\\\\\")",
        "c = '\\\\';",
        "let quote = '\"';",
        "m('{', \"x{\", '\\'', \"}\")",
        "matches!(c, 'a'|'{'|'\\n'|'('|'é'|']')",
        "fn f<'a>(x: &'a [u8], y: &'_ str) -> &'a str;",
    ];

    const COMMENTS: &[&str] = &[
        "// }",
        "/// Returns (a [b",
        "//! {",
        "// \"",
        "//(",
        "/// a / b",
    ];

    #[derive(Clone, Debug)]
    enum Line {
        Blank,
        Code {
            leading_closers: String,
            content: Option<&'static str>,
            later_closers: String,
            openers: String,
            comment: Option<&'static str>,
        },
    }

    fn brackets(kinds: &'static [char]) -> impl Strategy<Value = String> {
        proptest::collection::vec(select(kinds), 0..3)
            .prop_map(|brackets| brackets.into_iter().collect())
    }

    fn lines() -> impl Strategy<Value = Vec<Line>> {
        let code = (
            brackets(&['}', ']', ')']),
            proptest::option::of(select(WITHOUT_OPEN_BRACKETS)),
            brackets(&['}', ']', ')']),
            brackets(&['{', '[', '(']),
            proptest::option::weighted(0.2, select(COMMENTS)),
        )
            .prop_map(
                |(leading_closers, content, later_closers, openers, comment)| Line::Code {
                    leading_closers,
                    content,
                    later_closers,
                    openers,
                    comment,
                },
            );
        proptest::collection::vec(prop_oneof![1 => Just(Line::Blank), 9 => code], 0..12)
    }

    struct LaidOut {
        depth: usize,
        text: String,
        closes_after_other_code: bool,
    }

    fn laid_out(lines: &[Line]) -> Vec<LaidOut> {
        let mut depth = 0;
        let mut laid_out = Vec::new();
        for line in lines {
            let Line::Code {
                leading_closers,
                content,
                later_closers,
                openers,
                comment,
            } = line
            else {
                laid_out.push(LaidOut {
                    depth,
                    text: String::new(),
                    closes_after_other_code: false,
                });
                continue;
            };
            let leading_closers = &leading_closers[..leading_closers.len().min(depth)];
            let later_closers = match content {
                Some(_) => &later_closers[..later_closers.len().min(depth - leading_closers.len())],
                None => "",
            };
            let parts = [
                leading_closers,
                content.unwrap_or(""),
                later_closers,
                openers,
                comment.unwrap_or(""),
            ];
            let text = parts
                .into_iter()
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join(" ");
            laid_out.push(LaidOut {
                depth: depth - leading_closers.len(),
                text,
                closes_after_other_code: !later_closers.is_empty(),
            });
            depth = depth - leading_closers.len() - later_closers.len() + openers.len();
        }
        laid_out.extend((0..depth).rev().map(|depth| LaidOut {
            depth,
            text: "}".to_string(),
            closes_after_other_code: false,
        }));
        laid_out
    }

    fn source(lines: &[LaidOut]) -> String {
        lines
            .iter()
            .map(|line| format!("{}\n", line.text))
            .collect()
    }

    proptest! {
        #![proptest_config(proptest_config(1024))]

        #[test]
        fn every_line_is_indented_by_the_brackets_that_are_open_where_it_starts(
            lines in lines(),
            margins in proptest::collection::vec(("[ \t]{0,5}", "[ \t]{0,2}"), 40),
        ) {
            let laid_out = laid_out(&lines);
            prop_assert!(laid_out.len() <= margins.len());
            let source = laid_out
                .iter()
                .zip(&margins)
                .map(|(line, (before, after))| format!("{before}{}{after}\n", line.text))
                .collect::<String>();
            let indented = laid_out
                .iter()
                .map(|line| match line.text.as_str() {
                    "" => "\n".to_string(),
                    text => format!("{}{text}\n", "    ".repeat(line.depth)),
                })
                .collect::<String>();
            prop_assert_eq!(indent_by_nesting(&source), indented, "{:?}", source);
        }

        #[test]
        fn a_source_that_leaves_a_bracket_open_aborts_generation(
            lines in lines(),
            opener in select(&["{", "f(", "&[", "a / b ("][..]),
            at in any::<proptest::sample::Index>(),
        ) {
            let mut laid_out = laid_out(&lines);
            laid_out.insert(
                at.index(laid_out.len() + 1),
                LaidOut { depth: 0, text: opener.to_string(), closes_after_other_code: false },
            );
            let source = source(&laid_out);
            let message = catch_quietly(|| indent_by_nesting(&source));
            prop_assert!(
                message
                    .as_ref()
                    .is_err_and(|message| message.contains("leaves a bracket open")),
                "{:?} gives {:?}", source, message
            );
        }
    }

    #[test]
    fn the_generated_lines_open_and_close_brackets_in_every_position() {
        let mut reached = BTreeMap::<&str, usize>::new();
        let mut count = |case: &'static str, holds: bool| {
            *reached.entry(case).or_default() += usize::from(holds);
        };
        for lines in deterministic_values(lines(), 512) {
            let laid_out = laid_out(&lines);
            for (index, line) in laid_out.iter().enumerate() {
                let is_closer = |c: char| matches!(c, '}' | ']' | ')');
                let text = line.text.as_str();
                let code = text.split("//").next().unwrap_or("");
                count("a blank line", text.is_empty());
                count("a line below the top level", line.depth > 0);
                count("a line three levels deep", line.depth >= 3);
                count(
                    "a line that starts with a closer",
                    text.starts_with(is_closer),
                );
                count("a line that starts with two closers", {
                    text.chars().take_while(|c| is_closer(*c)).count() == 2
                });
                count("a closer after other code", line.closes_after_other_code);
                count(
                    "a line that ends with an opener",
                    code.trim_end().ends_with(['{', '[', '(']),
                );
                count("a bracket in a comment", {
                    text.contains("// }") || text.contains("//(") || text.contains("(a [b")
                });
                count("a slash in a string literal", text.contains("\"a/b}\""));
                count("two slashes in a string literal", text.contains("http://"));
                count("a division", text.contains("a / b"));
                count("an escaped quote", text.contains("\\\" }"));
                count(
                    "a double quote in a character literal",
                    text.contains("'\"'"),
                );
                count("a bracket in a character literal", text.contains("'{'"));
                count("a lifetime", text.contains("<'a>"));
                count("a line after the top level is reached again", {
                    line.depth == 0
                        && index > 0
                        && laid_out[..index].iter().any(|line| line.depth > 0)
                });
            }
        }
        for case in [
            "a blank line",
            "a line below the top level",
            "a line three levels deep",
            "a line that starts with a closer",
            "a line that starts with two closers",
            "a closer after other code",
            "a line that ends with an opener",
            "a bracket in a comment",
            "a slash in a string literal",
            "two slashes in a string literal",
            "a division",
            "an escaped quote",
            "a double quote in a character literal",
            "a bracket in a character literal",
            "a lifetime",
            "a line after the top level is reached again",
        ] {
            let lines = reached.get(case).copied().unwrap_or(0);
            assert!(
                lines >= 50,
                "only {lines} generated lines are {case}: {reached:?}"
            );
        }
    }
}
