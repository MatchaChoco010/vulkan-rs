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
        let mut in_string = false;
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' if in_string => {
                    chars.next();
                }
                '"' => in_string = !in_string,
                '/' if !in_string && chars.peek() == Some(&'/') => break,
                '{' | '[' | '(' if !in_string => brackets.openers += 1,
                c if is_closer(c) && !in_string => brackets.closers += 1,
                _ => {}
            }
        }
        assert!(!in_string, "`{line}` ends inside a string literal");
        brackets
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
        for lines in deterministic_values(lines(), 256) {
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
