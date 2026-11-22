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

    #[test]
    fn lines_are_indented_by_the_brackets_open_before_them() {
        let source = "impl A {
fn f(&self) -> B {
            B {
   a: [
1,
],
        }
}
    }

const C: u32 = 1;
";
        assert_eq!(
            indent_by_nesting(source),
            "impl A {
    fn f(&self) -> B {
        B {
            a: [
                1,
            ],
        }
    }
}

const C: u32 = 1;
"
        );
    }

    #[test]
    fn attributes_get_the_indentation_of_the_item_they_apply_to() {
        assert_eq!(
            indent_by_nesting("&[\n#[cfg(feature = \"beta\")]\nRow { a: 1 },\n];\n"),
            "&[\n    #[cfg(feature = \"beta\")]\n    Row { a: 1 },\n];\n"
        );
    }

    #[test]
    fn brackets_in_string_literals_and_closed_on_the_same_line_do_not_count() {
        let source = "fn f() {
write!(f, \"A({:#x}) \\\" }\", self.0)
} else {
g(|x| { x })
}
";
        assert_eq!(
            indent_by_nesting(&format!("{{\n{source}}}\n")),
            "{
    fn f() {
        write!(f, \"A({:#x}) \\\" }\", self.0)
    } else {
        g(|x| { x })
    }
}
"
        );
    }

    #[test]
    fn brackets_in_comment_lines_do_not_count() {
        let source = [
            "impl A {",
            "/// Returns (a",
            "/// [b",
            "fn f() {} // }",
            "}",
            "",
        ]
        .join("\n");
        assert_eq!(
            indent_by_nesting(&source),
            [
                "impl A {",
                "    /// Returns (a",
                "    /// [b",
                "    fn f() {} // }",
                "}",
                ""
            ]
            .join("\n")
        );
    }

    #[test]
    #[should_panic(expected = "leaves a bracket open")]
    fn an_unclosed_bracket_aborts_generation() {
        indent_by_nesting("fn f() {\n");
    }
}
