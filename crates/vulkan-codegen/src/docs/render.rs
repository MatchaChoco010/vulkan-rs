const WIDTH: usize = 92;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Block {
    Text(String),
    Heading(&'static str),
    List(Vec<(usize, String)>),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Doc {
    blocks: Vec<Block>,
    aliases: Vec<String>,
    item: Option<String>,
}

impl Doc {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn text(mut self, text: impl Into<String>) -> Self {
        let text = text.into();
        if !text.trim().is_empty() {
            self.blocks.push(Block::Text(text));
        }
        self
    }

    pub(crate) fn blocks(mut self, blocks: impl IntoIterator<Item = Block>) -> Self {
        self.blocks.extend(blocks);
        self
    }

    pub(crate) fn section(mut self, title: &'static str, text: impl Into<String>) -> Self {
        let text = text.into();
        if !text.trim().is_empty() {
            self.blocks.push(Block::Heading(title));
            self.blocks.push(Block::Text(text));
        }
        self
    }

    pub(crate) fn alias(mut self, c_name: &str) -> Self {
        self.aliases.push(c_name.to_string());
        self
    }

    pub(crate) fn of_item(mut self, rust_name: &str) -> Self {
        self.item = Some(rust_name.to_string());
        self
    }

    pub(crate) fn render(&self) -> String {
        assert!(!self.blocks.is_empty(), "an item without documentation");
        let mut out = String::new();
        for (index, block) in self.blocks.iter().enumerate() {
            if index > 0 {
                out.push_str("///\n");
            }
            match block {
                Block::Text(text) => push_wrapped(&mut out, "", "", text),
                Block::Heading(title) => {
                    out.push_str("/// # ");
                    out.push_str(title);
                    out.push('\n');
                }
                Block::List(items) => {
                    for (level, item) in items {
                        let indent = "  ".repeat(*level);
                        push_wrapped(
                            &mut out,
                            &format!("{indent}- "),
                            &format!("{indent}  "),
                            item,
                        );
                    }
                }
            }
        }
        let aliases = self
            .aliases
            .iter()
            .filter(|alias| self.item.as_ref() != Some(alias));
        for alias in aliases {
            out.push_str(&format!("#[doc(alias = \"{alias}\")]\n"));
        }
        out
    }
}

fn push_wrapped(out: &mut String, first_prefix: &str, next_prefix: &str, text: &str) {
    let mut line = String::new();
    let mut prefix = first_prefix;
    let mut flush = |line: &mut String, prefix: &str| {
        out.push_str("/// ");
        out.push_str(prefix);
        out.push_str(line);
        out.push('\n');
        line.clear();
    };
    for atom in atoms(text) {
        if line.is_empty() {
            line.push_str(&atom);
            continue;
        }
        let fits = prefix.len() + line.chars().count() + 1 + atom.chars().count() <= WIDTH;
        if fits || would_start_a_markdown_block(&atom) {
            line.push(' ');
            line.push_str(&atom);
        } else {
            flush(&mut line, prefix);
            prefix = next_prefix;
            line.push_str(&atom);
        }
    }
    if !line.is_empty() {
        flush(&mut line, prefix);
    }
}

fn would_start_a_markdown_block(atom: &str) -> bool {
    let numbered = atom
        .strip_suffix(['.', ')'])
        .is_some_and(|digits| !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()));
    numbered || atom.starts_with(['-', '+', '*', '>', '#', '=', '|', '~'])
}

fn atoms(text: &str) -> Vec<String> {
    let mut atoms = Vec::new();
    let mut atom = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                atom.push(c);
                atom.extend(chars.next());
            }
            '`' => {
                let mut run = 1;
                while chars.next_if_eq(&'`').is_some() {
                    run += 1;
                }
                atom.push_str(&"`".repeat(run));
                let mut closing = 0;
                while let Some(inner) = chars.next() {
                    atom.push(if inner == '\n' { ' ' } else { inner });
                    closing = if inner == '`' { closing + 1 } else { 0 };
                    if closing == run && chars.peek() != Some(&'`') {
                        break;
                    }
                }
            }
            c if c.is_whitespace() => {
                if !atom.is_empty() {
                    atoms.push(std::mem::take(&mut atom));
                }
            }
            c => atom.push(c),
        }
    }
    if !atom.is_empty() {
        atoms.push(atom);
    }
    atoms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_are_separated_by_empty_doc_lines_and_aliases_follow() {
        let doc = Doc::new()
            .text("Summary.")
            .section("Safety", "Read the manual.")
            .blocks([
                Block::Heading("Parameters"),
                Block::List(vec![
                    (0, "`a` is one.".to_string()),
                    (0, "`b` is two.".to_string()),
                ]),
            ])
            .alias("VkThing");
        assert_eq!(
            doc.render(),
            [
                "/// Summary.",
                "///",
                "/// # Safety",
                "///",
                "/// Read the manual.",
                "///",
                "/// # Parameters",
                "///",
                "/// - `a` is one.",
                "/// - `b` is two.",
                "#[doc(alias = \"VkThing\")]",
                "",
            ]
            .join("\n")
        );
    }

    #[test]
    fn an_alias_that_is_the_name_of_the_item_is_left_out() {
        let doc = Doc::new().text("Summary.").alias("StdThing").alias("OTHER");
        assert_eq!(
            doc.of_item("StdThing").render(),
            "/// Summary.\n#[doc(alias = \"OTHER\")]\n"
        );
    }

    #[test]
    fn empty_text_adds_nothing() {
        assert_eq!(
            Doc::new()
                .text("One.")
                .text("  ")
                .section("Errors", "")
                .render(),
            "/// One.\n"
        );
    }

    #[test]
    fn long_text_wraps_between_words_and_reflows_line_breaks() {
        let word = "abcdefghi";
        let text = [word; 12].join("\n");
        let rendered = Doc::new().text(text).render();
        let lines = rendered.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], format!("/// {}", [word; 9].join(" ")));
        assert_eq!(lines[1], format!("/// {}", [word; 3].join(" ")));
    }

    #[test]
    fn code_spans_stay_on_one_line() {
        let filler = "x".repeat(80);
        let rendered = Doc::new()
            .text(format!("{filler} `a b c d e f` and ``g ` h`` end"))
            .render();
        assert_eq!(
            rendered,
            format!("/// {filler}\n/// `a b c d e f` and ``g ` h`` end\n")
        );
    }

    #[test]
    fn a_wrapped_line_never_starts_with_a_markdown_block_marker() {
        let filler = "x".repeat(88);
        for marker in ["+", "-", "*", ">", "#1", "12.", "3)", "==="] {
            let rendered = Doc::new().text(format!("{filler} {marker} tail")).render();
            assert_eq!(
                rendered,
                format!("/// {filler} {marker}\n/// tail\n"),
                "{marker}"
            );
        }
    }

    #[test]
    fn list_items_wrap_with_a_hanging_indent() {
        let word = "abcdefghi";
        let rendered = Doc::new()
            .blocks([Block::List(vec![
                (0, [word; 10].join(" ")),
                (1, "nested".to_string()),
            ])])
            .render();
        assert_eq!(
            rendered,
            format!(
                "/// - {}\n///   {word}\n///   - nested\n",
                [word; 9].join(" ")
            )
        );
    }

    #[test]
    #[should_panic(expected = "without documentation")]
    fn an_empty_doc_does_not_render() {
        Doc::new().alias("VkThing").render();
    }
}
