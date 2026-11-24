use super::asciidoc::Unsupported;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Part {
    Paragraph(String),
    List(Vec<Vec<Part>>),
    Terms(Vec<(String, Vec<Part>)>),
    Admonition {
        label: &'static str,
        title: Option<String>,
        parts: Vec<Part>,
    },
    Math(String),
    Listing {
        language: String,
        text: String,
    },
}

const LIST_CONTINUATION: &str = "+";

const ADMONITIONS: &[(&str, &str)] = &[
    ("NOTE", "Note"),
    ("TIP", "Tip"),
    ("IMPORTANT", "Important"),
    ("WARNING", "Warning"),
    ("CAUTION", "Caution"),
];

#[derive(Copy, Clone, PartialEq, Eq)]
enum Within {
    ListItem,
    Block,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Marker {
    Stars(usize),
    Term,
}

struct OpenList {
    marker: Marker,
    terms: Vec<String>,
    items: Vec<Vec<Part>>,
}

struct Reader<'a> {
    within: Within,
    parts: Vec<Part>,
    lists: Vec<OpenList>,
    paragraph: Vec<&'a str>,
    header: Vec<&'a str>,
    attaching: bool,
    after_an_empty_line: bool,
}

pub(super) fn read(text: &str) -> Result<Vec<Part>, Unsupported> {
    let lines = text.lines().collect::<Vec<_>>();
    read_lines(&lines, Within::ListItem)
}

fn note_number(term: &str) -> Option<u32> {
    term.chars()
        .all(|c| c.is_ascii_digit())
        .then(|| term.parse().ok())
        .flatten()
}

pub(super) fn may_have_notes(text: &str) -> bool {
    text.lines()
        .filter_map(|line| term(line.trim()))
        .any(|(name, _)| note_number(name).is_some())
}

pub(super) fn take_notes(parts: &mut Vec<Part>) -> Vec<(u32, Vec<Part>)> {
    let mut notes = Vec::new();
    for part in std::mem::take(parts) {
        let mut part = part;
        match &mut part {
            Part::Terms(terms) if terms.iter().all(|(term, _)| note_number(term).is_some()) => {
                notes.extend(
                    terms
                        .drain(..)
                        .filter_map(|(term, note)| Some((note_number(&term)?, note))),
                );
                continue;
            }
            Part::Terms(terms) => {
                for (_, described) in terms {
                    notes.extend(take_notes(described));
                }
            }
            Part::List(items) => {
                for item in items {
                    notes.extend(take_notes(item));
                }
            }
            Part::Admonition { parts, .. } => notes.extend(take_notes(parts)),
            Part::Paragraph(_) | Part::Math(_) | Part::Listing { .. } => {}
        }
        parts.push(part);
    }
    notes
}

fn with_marks(text: &str, mut referred: impl FnMut(u32) -> String) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(open) = rest.find('^') {
        let after = &rest[open + 1..];
        let digits = after
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(after.len());
        let number = after[digits..]
            .starts_with('^')
            .then(|| note_number(&after[..digits]))
            .flatten();
        match number {
            Some(number) => {
                out.push_str(&rest[..open]);
                out.push_str(&referred(number));
                rest = &after[digits + 1..];
            }
            None => {
                out.push_str(&rest[..=open]);
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}

pub(super) fn refer_to_notes(parts: &mut [Part]) -> BTreeSet<u32> {
    let mut marks = BTreeSet::new();
    for part in parts {
        match part {
            Part::Paragraph(text) => {
                *text = with_marks(text, |number| {
                    marks.insert(number);
                    format!(" (note {number})")
                });
            }
            Part::List(items) => {
                for item in items {
                    marks.extend(refer_to_notes(item));
                }
            }
            Part::Terms(terms) => {
                for (_, described) in terms {
                    marks.extend(refer_to_notes(described));
                }
            }
            Part::Admonition { parts, .. } => marks.extend(refer_to_notes(parts)),
            Part::Math(_) | Part::Listing { .. } => {}
        }
    }
    marks
}

fn unread(what: impl Into<String>) -> Unsupported {
    Unsupported::Block(what.into())
}

fn is_delimiter(line: &str) -> bool {
    let uniform = |c: char, at_least: usize| line.len() >= at_least && line.chars().all(|x| x == c);
    line == "--"
        || ['=', '-', '.', '*', '_', '+', '/']
            .iter()
            .any(|c| uniform(*c, 4))
        || uniform('`', 3)
        || (line.len() >= 4
            && line.starts_with(['|', ',', ':', '!'])
            && line[1..].chars().all(|c| c == '='))
}

fn is_attributes(line: &str) -> bool {
    let Some(inner) = line
        .strip_prefix('[')
        .and_then(|line| line.strip_suffix(']'))
    else {
        return false;
    };
    inner.chars().next().is_none_or(|c| {
        c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '#' | '%' | '{' | ',' | '"' | '\'')
    })
}

fn title(line: &str) -> Option<&str> {
    line.strip_prefix('.')
        .filter(|title| !title.starts_with(['.', ' ', '\t']) && !title.is_empty())
}

fn list_item(line: &str) -> Option<(usize, &str)> {
    let stars = line.chars().take_while(|c| *c == '*').count();
    let rest = &line[stars..];
    (stars > 0 && rest.starts_with([' ', '\t']) && !rest.trim().is_empty())
        .then(|| (stars, rest.trim_start()))
}

fn starts_another_kind_of_list(line: &str) -> bool {
    let marker_len = line.find([' ', '\t']).unwrap_or(line.len());
    let (marker, rest) = line.split_at(marker_len);
    if rest.trim().is_empty() {
        return false;
    }
    let numbered = marker.strip_suffix('.').is_some_and(|number| {
        !number.is_empty() && number.chars().all(|c| c.is_ascii_digit())
            || number.len() == 1 && number.chars().all(|c| c.is_ascii_alphabetic())
    });
    let roman = marker
        .strip_suffix(')')
        .is_some_and(|number| !number.is_empty() && number.chars().all(|c| "IVXivx".contains(c)));
    let dotted = !marker.is_empty() && marker.chars().all(|c| c == '.');
    matches!(marker, "-" | "\u{2022}") || numbered || roman || dotted
}

fn names_a_term(line: &str) -> bool {
    ["::", ";;"].iter().any(|separator| {
        line.match_indices(separator).any(|(at, _)| {
            let after = line[at + separator.len()..].trim_start_matches(':');
            at > 0 && (after.is_empty() || after.starts_with([' ', '\t']))
        })
    })
}

fn term(line: &str) -> Option<(&str, &str)> {
    let at = line.find("::").filter(|at| *at > 0)?;
    let description = &line[at + 2..];
    (description.is_empty() || description.starts_with([' ', '\t']))
        .then(|| (line[..at].trim_end(), description.trim_start()))
}

fn is_attribute_entry(line: &str) -> bool {
    line.strip_prefix(':')
        .and_then(|rest| rest.split_once(':'))
        .is_some_and(|(name, value)| {
            let name = name.strip_suffix('!').unwrap_or(name);
            !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-'))
                && (value.is_empty() || value.starts_with([' ', '\t']))
        })
}

fn read_lines(lines: &[&str], within: Within) -> Result<Vec<Part>, Unsupported> {
    let mut reader = Reader {
        within,
        parts: Vec::new(),
        lists: Vec::new(),
        paragraph: Vec::new(),
        header: Vec::new(),
        attaching: false,
        after_an_empty_line: within == Within::Block,
    };
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index].trim_end();
        index += 1;
        if is_delimiter(line) {
            let close = lines[index..]
                .iter()
                .position(|inside| inside.trim_end() == line)
                .ok_or(Unsupported::Unterminated("delimited block"))?;
            reader.block(line, &lines[index..index + close])?;
            index += close + 1;
        } else {
            reader.line(line)?;
        }
    }
    reader.finish()
}

impl<'a> Reader<'a> {
    fn target(&mut self) -> &mut Vec<Part> {
        match self.lists.last_mut() {
            Some(list) => list.items.last_mut().expect("an open list has an item"),
            None => &mut self.parts,
        }
    }

    fn end_paragraph(&mut self) {
        if !self.paragraph.is_empty() {
            let text = std::mem::take(&mut self.paragraph).join("\n");
            let text = text.strip_suffix(" +").unwrap_or(&text).to_string();
            self.target().push(Part::Paragraph(text));
        }
    }

    fn close_lists(&mut self, open: usize) {
        self.end_paragraph();
        while self.lists.len() > open {
            let list = self.lists.pop().expect("a list to close");
            let part = match list.marker {
                Marker::Stars(_) => Part::List(list.items),
                Marker::Term => Part::Terms(list.terms.into_iter().zip(list.items).collect()),
            };
            self.target().push(part);
        }
    }

    fn start_item(&mut self, marker: Marker) {
        self.end_paragraph();
        match self.lists.iter().position(|list| list.marker == marker) {
            Some(depth) => self.close_lists(depth + 1),
            None => self.lists.push(OpenList {
                marker,
                terms: Vec::new(),
                items: Vec::new(),
            }),
        }
        let list = self.lists.last_mut().expect("the list of the item");
        list.items.push(Vec::new());
        self.attaching = false;
    }

    fn in_a_list_item(&self) -> bool {
        self.within == Within::ListItem || !self.lists.is_empty()
    }

    fn outermost_item(&self) -> usize {
        match self.within {
            Within::ListItem => 0,
            Within::Block => 1,
        }
    }

    fn line(&mut self, line: &'a str) -> Result<(), Unsupported> {
        let text = line.trim_start();
        if text.is_empty() {
            if self.attaching || !self.header.is_empty() {
                return Err(unread("an empty line where a block is expected"));
            }
            self.end_paragraph();
            self.after_an_empty_line = true;
            return Ok(());
        }
        let after_an_empty_line = std::mem::take(&mut self.after_an_empty_line);
        if line == LIST_CONTINUATION {
            if self.attaching || !self.header.is_empty() {
                return Err(unread("a list continuation where a block is expected"));
            }
            if !self.in_a_list_item() {
                return Err(unread("a list continuation outside a list"));
            }
            if after_an_empty_line {
                self.close_lists(self.outermost_item());
            }
            self.end_paragraph();
            self.attaching = true;
            return Ok(());
        }
        let starts_a_block = self.attaching || after_an_empty_line;
        if is_attributes(line) || (starts_a_block && title(line).is_some()) {
            if !starts_a_block {
                return Err(unread(format!("the attributes `{line}` in a paragraph")));
            }
            if !self.attaching {
                self.close_lists(0);
            }
            self.header.push(line);
            return Ok(());
        }
        if let Some((marker, rest)) = list_item(text) {
            if !self.header.is_empty() {
                return Err(unread("a list with attributes"));
            }
            let in_a_paragraph_of_a_block =
                !self.in_a_list_item() && !self.paragraph.is_empty() && !after_an_empty_line;
            if in_a_paragraph_of_a_block {
                return Err(unread("a list item in a paragraph"));
            }
            self.start_item(Marker::Stars(marker));
            self.paragraph.push(rest);
            return Ok(());
        }
        if !self.header.is_empty() {
            return Err(unread(format!(
                "a paragraph with the attributes {:?}",
                self.header
            )));
        }
        if text.starts_with("include::") {
            return Err(Unsupported::Include);
        }
        if is_attribute_entry(text) {
            return Err(unread(format!("the attribute entry `{text}`")));
        }
        let starts_the_item = self.within == Within::ListItem
            && self.parts.is_empty()
            && self.lists.is_empty()
            && self.paragraph.is_empty();
        let could_start_a_list = self.in_a_list_item() && !starts_the_item;
        let in_a_description_list = self.lists.iter().any(|list| list.marker == Marker::Term);
        if could_start_a_list
            && (after_an_empty_line || in_a_description_list)
            && let Some((name, description)) = term(text)
        {
            self.start_item(Marker::Term);
            let list = self.lists.last_mut().expect("the list of the term");
            list.terms.push(name.to_string());
            if !description.is_empty() {
                self.paragraph.push(description);
            }
            return Ok(());
        }
        if could_start_a_list && (starts_another_kind_of_list(text) || names_a_term(text)) {
            return Err(unread(format!("a list that starts with `{text}`")));
        }
        let describes_a_term = self.paragraph.is_empty()
            && self.lists.last().is_some_and(|list| {
                list.marker == Marker::Term && list.items.last().is_some_and(Vec::is_empty)
            });
        if after_an_empty_line && !self.attaching && !describes_a_term {
            if self.within == Within::ListItem {
                return Err(unread("a paragraph after an empty line"));
            }
            self.close_lists(0);
        }
        self.attaching = false;
        self.paragraph.push(text);
        Ok(())
    }

    fn block(&mut self, delimiter: &str, content: &[&'a str]) -> Result<(), Unsupported> {
        let after_an_empty_line = std::mem::take(&mut self.after_an_empty_line);
        if !self.attaching && self.header.is_empty() {
            let starts_a_block = self.within == Within::Block && after_an_empty_line;
            if !starts_a_block {
                return Err(unread(format!(
                    "a `{delimiter}` block that nothing attaches"
                )));
            }
            self.close_lists(0);
        }
        self.end_paragraph();
        let header = std::mem::take(&mut self.header);
        let mut style = Vec::new();
        let mut block_title = None;
        for line in header {
            match title(line) {
                Some(text) => block_title = Some(text.trim().to_string()),
                _ => style.extend(
                    line[1..line.len() - 1]
                        .split(',')
                        .map(|attribute| attribute.trim().trim_matches('"')),
                ),
            }
        }
        let fence = delimiter.chars().next().expect("a delimiter");
        let admonition = ADMONITIONS
            .iter()
            .find(|(name, _)| style.as_slice() == [*name])
            .map(|(_, label)| *label);
        let part = match (fence, style.as_slice(), admonition) {
            ('=', _, Some(label)) => Part::Admonition {
                label,
                title: block_title.take(),
                parts: read_lines(content, Within::Block)?,
            },
            ('+', ["latexmath"], _) => Part::Math(
                content
                    .iter()
                    .flat_map(|line| line.split_whitespace())
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            ('-', ["source", language], _) if delimiter.len() >= 4 => {
                let indent = content
                    .iter()
                    .filter(|line| !line.trim().is_empty())
                    .map(|line| line.len() - line.trim_start().len())
                    .min()
                    .unwrap_or(0);
                let text = content
                    .iter()
                    .map(|line| line.get(indent..).unwrap_or("").trim_end())
                    .collect::<Vec<_>>()
                    .join("\n");
                Part::Listing {
                    language: language.to_string(),
                    text,
                }
            }
            _ => {
                return Err(unread(format!(
                    "a `{delimiter}` block with the attributes {style:?}"
                )));
            }
        };
        if let Some(title) = block_title {
            return Err(unread(format!("a block with the title `{title}`")));
        }
        self.target().push(part);
        self.attaching = false;
        Ok(())
    }

    fn finish(mut self) -> Result<Vec<Part>, Unsupported> {
        if self.attaching || !self.header.is_empty() {
            return Err(unread("the end of the text where a block is expected"));
        }
        self.close_lists(0);
        Ok(self.parts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::path::Path;
    use std::process::{Command, Stdio};
    use vulkan_registry::Docs;

    fn paragraph(text: &str) -> Part {
        Part::Paragraph(text.to_string())
    }

    fn list<const N: usize>(items: [Vec<Part>; N]) -> Part {
        Part::List(items.into())
    }

    fn note(parts: Vec<Part>) -> Part {
        Part::Admonition {
            label: "Note",
            title: None,
            parts,
        }
    }

    fn parts(lines: &[&str]) -> Vec<Part> {
        let text = lines.join("\n");
        read(&text).unwrap_or_else(|unsupported| panic!("{text}: {unsupported:?}"))
    }

    #[test]
    fn a_dot_alone_or_before_a_blank_starts_no_block_title() {
        assert_eq!(
            parts(&["pname:a is:", "+", ".", "text"]),
            [paragraph("pname:a is:"), paragraph(".\ntext")]
        );
        for lead in ["+", ""] {
            for item in [". the first", ".\tthe first"] {
                let text = ["pname:a is:", lead, item, "text"].join("\n");
                assert_eq!(
                    read(&text),
                    Err(Unsupported::Block(format!(
                        "a list that starts with `{item}`"
                    ))),
                    "{text}"
                );
            }
        }
    }

    #[test]
    fn the_text_goes_on_over_lines_that_start_with_markup() {
        for lines in [
            [
                "pname:bias is clamped to the range",
                "[eq]#[-pname:maxBias,+pname:maxBias]#.",
                "See <<limits-bias>>.",
            ],
            [
                "ename:VK_COPY creates a copy with the same parameters",
                ".",
                "If pname:src refers to other structures, so does pname:dst.",
            ],
            [
                "pname:size is calculated as follows: +",
                "[eq]#width = {lceil} pname:w / grid {rceil}# +",
                "where grid is the grid size.",
            ],
            [
                "pname:range[2] is the",
                " [eq]#[minimum, maximum]# range of pname:limits[0]",
                "..and of every other limit.",
            ],
        ] {
            assert_eq!(
                parts(&lines),
                [paragraph(&lines.join("\n").replace("\n ", "\n"))]
            );
        }
    }

    #[test]
    fn a_hard_line_break_at_the_end_of_a_paragraph_is_left_out() {
        assert_eq!(
            parts(&["pname:a is one of: +", "** this +"]),
            [
                paragraph("pname:a is one of:"),
                list([vec![paragraph("this")]])
            ]
        );
    }

    #[test]
    fn a_list_item_starts_with_stars_a_blank_and_text() {
        for stars in 0..6 {
            for blank in ["", " ", "  ", "\t"] {
                for text in ["a", "a b", "*a*", "**a**", ""] {
                    if text.starts_with('*') && blank.is_empty() {
                        continue;
                    }
                    let line = format!("{}{blank}{text}", "*".repeat(stars));
                    let item = (stars > 0 && !blank.is_empty() && !text.is_empty())
                        .then_some((stars, text));
                    assert_eq!(list_item(&line), item, "{line:?}");
                }
            }
        }
        for line in ["**a** b", " ** a", " a", "** \t "] {
            assert_eq!(list_item(line), None, "{line:?}");
        }
    }

    #[test]
    fn nested_items_follow_their_markers() {
        assert_eq!(
            parts(&[
                "pname:a is either:",
                "** the first,",
                "which goes on,",
                "*** with a detail",
                "*** and another",
                "** or the second.",
            ]),
            [
                paragraph("pname:a is either:"),
                list([
                    vec![
                        paragraph("the first,\nwhich goes on,"),
                        list([
                            vec![paragraph("with a detail")],
                            vec![paragraph("and another")]
                        ]),
                    ],
                    vec![paragraph("or the second.")],
                ]),
            ]
        );
        assert_eq!(
            parts(&[
                "pname:a is:",
                "*** deep",
                "** deeper still",
                "*** deep again"
            ]),
            [
                paragraph("pname:a is:"),
                list([
                    vec![paragraph("deep"), list([vec![paragraph("deeper still")]])],
                    vec![paragraph("deep again")],
                ]),
            ],
            "a marker that no open list has starts a list in the item above it"
        );
        assert_eq!(
            parts(&["pname:a is:", "** one", "", "** two"]),
            [
                paragraph("pname:a is:"),
                list([vec![paragraph("one")], vec![paragraph("two")]]),
            ],
            "an empty line between two items does not end the list"
        );
    }

    #[test]
    fn a_list_continuation_attaches_the_next_block_to_the_innermost_item() {
        assert_eq!(
            parts(&[
                "pname:formats lists what is supported.",
                "+",
                "The formats have the following requirements:",
                "+",
                "** The first requirement.",
                "** The second requirement.",
                "+",
                "The functions have the following requirements:",
                "+",
                "** The third requirement.",
            ]),
            [
                paragraph("pname:formats lists what is supported."),
                paragraph("The formats have the following requirements:"),
                list([
                    vec![paragraph("The first requirement.")],
                    vec![
                        paragraph("The second requirement."),
                        paragraph("The functions have the following requirements:"),
                    ],
                    vec![paragraph("The third requirement.")],
                ]),
            ]
        );
    }

    #[test]
    fn a_list_continuation_after_an_empty_line_attaches_to_the_item_itself() {
        assert_eq!(
            parts(&[
                "pname:a is one of:",
                "** this",
                "*** or rather this",
                "",
                "+",
                "Whichever it is, it stays.",
            ]),
            [
                paragraph("pname:a is one of:"),
                list([vec![
                    paragraph("this"),
                    list([vec![paragraph("or rather this")]])
                ]]),
                paragraph("Whichever it is, it stays."),
            ]
        );
        assert_eq!(
            parts(&[
                "pname:a is:",
                "+",
                "[NOTE]",
                "====",
                "* this",
                "** or rather this",
                "",
                "+",
                "Whichever it is, it stays.",
                "====",
            ]),
            [
                paragraph("pname:a is:"),
                note(vec![list([vec![
                    paragraph("this"),
                    list([vec![paragraph("or rather this")]]),
                    paragraph("Whichever it is, it stays."),
                ]])]),
            ],
            "in a block, the outermost item is the item of the first list"
        );
    }

    fn terms<const N: usize>(items: [(&str, Vec<Part>); N]) -> Part {
        Part::Terms(
            items
                .into_iter()
                .map(|(term, parts)| (term.to_string(), parts))
                .collect(),
        )
    }

    #[test]
    fn a_term_after_an_empty_line_starts_a_description_list_in_the_innermost_item() {
        assert_eq!(
            parts(&[
                "pname:a is the size.",
                "",
                "1::",
                "For all sizes, the note",
                "applies.",
                "",
                "2:: The second note.",
                "** with a detail",
                "the third of sname:VkThing::\tThe last note.",
            ]),
            [
                paragraph("pname:a is the size."),
                terms([
                    ("1", vec![paragraph("For all sizes, the note\napplies.")]),
                    (
                        "2",
                        vec![
                            paragraph("The second note."),
                            list([vec![paragraph("with a detail")]]),
                        ]
                    ),
                    (
                        "the third of sname:VkThing",
                        vec![paragraph("The last note.")]
                    ),
                ]),
            ]
        );
        assert_eq!(
            parts(&[
                "pname:a is one of:",
                "** this",
                "",
                "why::",
                "Because.",
                "+",
                "Really.",
                "** or that",
                "",
                "+",
                "Whichever it is, it stays.",
            ]),
            [
                paragraph("pname:a is one of:"),
                list([
                    vec![
                        paragraph("this"),
                        terms([("why", vec![paragraph("Because."), paragraph("Really.")])]),
                    ],
                    vec![paragraph("or that")],
                ]),
                paragraph("Whichever it is, it stays."),
            ]
        );
    }

    #[test]
    fn a_term_without_a_description_takes_the_text_after_an_empty_line() {
        assert_eq!(
            parts(&[
                "pname:a is:",
                "",
                "1::",
                "",
                "The note",
                "goes on.",
                "",
                "2::",
                "",
                "Another note.",
                "** in detail",
            ]),
            [
                paragraph("pname:a is:"),
                terms([
                    ("1", vec![paragraph("The note\ngoes on.")]),
                    (
                        "2",
                        vec![
                            paragraph("Another note."),
                            list([vec![paragraph("in detail")]])
                        ]
                    ),
                ]),
            ]
        );
    }

    #[test]
    fn numbered_terms_are_notes_and_are_taken_out_of_the_item_that_they_are_nested_in() {
        let mut item = parts(&[
            "pname:a is a bitmask^1^ of 2^n^ values^12^, x^^ and ^3",
            "** nested^1^",
            "",
            "1::",
            "The first note.",
            "",
            "12:: The other note.",
        ]);
        assert_eq!(
            take_notes(&mut item),
            [
                (1, vec![paragraph("The first note.")]),
                (12, vec![paragraph("The other note.")]),
            ]
        );
        assert_eq!(
            item,
            [
                paragraph("pname:a is a bitmask^1^ of 2^n^ values^12^, x^^ and ^3"),
                list([vec![paragraph("nested^1^")]]),
            ]
        );
        assert_eq!(refer_to_notes(&mut item), BTreeSet::from([1, 12]));
        assert_eq!(
            item,
            [
                paragraph("pname:a is a bitmask (note 1) of 2^n^ values (note 12), x^^ and ^3"),
                list([vec![paragraph("nested (note 1)")]]),
            ]
        );
        let mut described = parts(&["pname:a is:", "", "1::", "A step.", "", "then:: Another."]);
        let before = described.clone();
        assert!(take_notes(&mut described).is_empty());
        assert_eq!(
            described, before,
            "a list with a term that is no number is no list of notes"
        );
        for (text, notes) in [
            ("pname:a is:\n\n1::\nThe note.", true),
            ("pname:a is:\n\n  12:: The note.", true),
            ("pname:a is:\n\nterm:: The note.", false),
            ("pname:a is 1::2 and\n1: or 1::: x", false),
            ("pname:a is:", false),
        ] {
            assert_eq!(may_have_notes(text), notes, "{text:?}");
        }
    }

    #[test]
    fn a_term_names_what_is_before_two_colons_at_the_end_of_a_word() {
        for (line, named) in [
            ("1::", Some(("1", ""))),
            ("1:: x", Some(("1", "x"))),
            ("a b ::\t x:: y", Some(("a b", "x:: y"))),
            ("sname:VkThing:: x", Some(("sname:VkThing", "x"))),
            ("::", None),
            (":: x", None),
            ("1:", None),
            ("1::x", None),
            ("1::: x", None),
            ("1;; x", None),
            ("sname:VkThing::pname:a is:: x", None),
        ] {
            assert_eq!(term(line), named, "{line:?}");
        }
    }

    #[test]
    fn attached_blocks_are_read_with_what_is_in_them() {
        assert_eq!(
            parts(&[
                "pname:a is computed as follows:",
                "+",
                "[latexmath]",
                "++++",
                "\\begin{aligned}",
                "  a &= b \\\\",
                "\\end{aligned}",
                "++++",
                "+",
                "[NOTE]",
                "====",
                "The formula is exact.",
                "",
                "It has two cases:",
                "",
                "  * the first",
                "    case,",
                "  ** in detail",
                "  * the second case.",
                "",
                "Both are common.",
                "====",
                "+",
                "[source,c++]",
                "----",
                "    int a = b;",
                "",
                "      a += 1;",
                "----",
                "+",
                "The value is never negative.",
            ]),
            [
                paragraph("pname:a is computed as follows:"),
                Part::Math("\\begin{aligned} a &= b \\\\ \\end{aligned}".to_string()),
                note(vec![
                    paragraph("The formula is exact."),
                    paragraph("It has two cases:"),
                    list([
                        vec![
                            paragraph("the first\ncase,"),
                            list([vec![paragraph("in detail")]])
                        ],
                        vec![paragraph("the second case.")],
                    ]),
                    paragraph("Both are common."),
                ]),
                Part::Listing {
                    language: "c++".to_string(),
                    text: "int a = b;\n\n  a += 1;".to_string(),
                },
                paragraph("The value is never negative."),
            ]
        );
        assert_eq!(
            parts(&[
                "pname:a is:",
                "** one",
                "+",
                ".Why One",
                "[WARNING]",
                "====",
                "Because.",
                "====",
            ]),
            [
                paragraph("pname:a is:"),
                list([vec![
                    paragraph("one"),
                    Part::Admonition {
                        label: "Warning",
                        title: Some("Why One".to_string()),
                        parts: vec![paragraph("Because.")],
                    },
                ]]),
            ]
        );
    }

    #[test]
    fn text_in_a_file_that_is_not_part_of_the_sources_is_told_apart() {
        assert_eq!(
            read(
                "ename:VK_ALL is equivalent to the logical OR of:\ninclude::{generated}/sync/all.adoc[]"
            ),
            Err(Unsupported::Include)
        );
    }

    #[test]
    fn what_the_reader_does_not_know_is_refused() {
        for (lines, what) in [
            (
                vec!["pname:a is in the table.", "+", "|====", "| a | b", "|===="],
                "a `|====` block with the attributes []",
            ),
            (
                vec!["pname:a is:", "+", "[source]", "----", "a", "----"],
                "a `----` block with the attributes [\"source\"]",
            ),
            (
                vec!["pname:a is:", "+", "--", "open", "--"],
                "a `--` block with the attributes []",
            ),
            (
                vec!["pname:a is:", "+", "[source,c++]", "--", "a", "--"],
                "a `--` block with the attributes [\"source\", \"c++\"]",
            ),
            (
                vec![
                    "pname:a is:",
                    "+",
                    ".Title",
                    "[latexmath]",
                    "++++",
                    "a",
                    "++++",
                ],
                "a block with the title `Title`",
            ),
            (
                vec!["pname:a is:", "+", "[NOTE]", "A paragraph as a note."],
                "a paragraph with the attributes [\"[NOTE]\"]",
            ),
            (
                vec![
                    "pname:a is:",
                    "+",
                    "[NOTE]",
                    "====",
                    "a",
                    "[TIP]",
                    "b",
                    "====",
                ],
                "the attributes `[TIP]` in a paragraph",
            ),
            (
                vec!["pname:a is:", "+", "[NOTE]", "====", "a", "* b", "===="],
                "a list item in a paragraph",
            ),
            (
                vec!["pname:a is:", "+", "[NOTE]", "====", "a", "+", "b", "===="],
                "a list continuation outside a list",
            ),
            (
                vec!["pname:a is:", "", "A new paragraph."],
                "a paragraph after an empty line",
            ),
            (
                vec!["pname:a is:", "----", "a", "----"],
                "a `----` block that nothing attaches",
            ),
            (
                vec!["pname:a is:", "", "----", "a", "----"],
                "a `----` block that nothing attaches",
            ),
            (
                vec![
                    "pname:a is:",
                    "+",
                    "[NOTE]",
                    "====",
                    "a",
                    "----",
                    "b",
                    "----",
                    "====",
                ],
                "a `----` block that nothing attaches",
            ),
            (
                vec!["pname:a is:", "+", "+", "b"],
                "a list continuation where a block is expected",
            ),
            (
                vec!["pname:a is:", "+", "", "b"],
                "an empty line where a block is expected",
            ),
            (
                vec!["pname:a is:", "+"],
                "the end of the text where a block is expected",
            ),
            (
                vec!["pname:a is:", ":attribute: value"],
                "the attribute entry `:attribute: value`",
            ),
            (
                vec!["pname:a is one of", "1. the first"],
                "a list that starts with `1. the first`",
            ),
            (
                vec!["pname:a is one of", "- the first"],
                "a list that starts with `- the first`",
            ),
            (
                vec!["pname:a is one of", ". the first"],
                "a list that starts with `. the first`",
            ),
            (
                vec!["pname:a is one of", "a. the first"],
                "a list that starts with `a. the first`",
            ),
            (
                vec!["pname:a is one of", "iv) the fourth"],
                "a list that starts with `iv) the fourth`",
            ),
            (
                vec!["pname:a is", "term:: the definition"],
                "a list that starts with `term:: the definition`",
            ),
            (
                vec!["pname:a is a member", "of sname:VkThing::"],
                "a list that starts with `of sname:VkThing::`",
            ),
            (
                vec!["pname:a is:", "+", "term:: attached"],
                "a list that starts with `term:: attached`",
            ),
            (
                vec!["pname:a is:", "** one", "term:: in the item"],
                "a list that starts with `term:: in the item`",
            ),
            (
                vec!["pname:a is:", "", "term::: deeper"],
                "a list that starts with `term::: deeper`",
            ),
            (
                vec!["pname:a is:", "", "term;; other"],
                "a list that starts with `term;; other`",
            ),
            (
                vec![
                    "pname:a is:",
                    "",
                    "1::",
                    "",
                    "The note.",
                    "",
                    "A second paragraph.",
                ],
                "a paragraph after an empty line",
            ),
            (
                vec![
                    "pname:a is:",
                    "",
                    "1:: The note.",
                    "",
                    "A second paragraph.",
                ],
                "a paragraph after an empty line",
            ),
        ] {
            let text = lines.join("\n");
            assert_eq!(
                read(&text),
                Err(Unsupported::Block(what.to_string())),
                "{text}"
            );
        }
        assert_eq!(
            read("pname:a is:\n+\n[NOTE]\n====\nnever closed"),
            Err(Unsupported::Unterminated("delimited block"))
        );
    }

    #[test]
    fn text_that_only_resembles_a_list_or_a_block_stays_text() {
        for lines in [
            [
                "term:: is how the text of pname:a starts, like sname:VkThing::",
                "pname:a is slink:VkThing::pname:a, i.e. the",
                "e.g. first -- or",
                "-1.",
            ],
            [
                "pname:a is one,",
                "etc. and so on,",
                ":: names no term,",
                ":x:y sets no attribute",
            ],
            [
                "pname:a is 2 * 3",
                "...and more,",
                "*not* a list item,",
                "(iv) nor a numbered one",
            ],
            [
                "pname:a[0] is the",
                "[first] element",
                "[eq]#a[0]# or [eq]#a[1]#",
                "---",
            ],
        ] {
            assert_eq!(parts(&lines), [paragraph(&lines.join("\n"))]);
        }
        assert_eq!(
            parts(&["pname:a is", " +", " [eq]#a[0] + a[1]", " ----"]),
            [paragraph("pname:a is\n+\n[eq]#a[0] + a[1]\n----")],
            "an indented line is text, whatever it would be at the margin"
        );
    }

    fn dumped(source: &[&str]) -> Option<Result<Vec<(String, String)>, String>> {
        let tool = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tools/generate-vulkan");
        let script = [
            "import importlib.machinery, importlib.util, json, pathlib, sys, tempfile",
            "loader = importlib.machinery.SourceFileLoader('generate_vulkan', sys.argv[1])",
            "specification = importlib.util.spec_from_loader('generate_vulkan', loader)",
            "tool = importlib.util.module_from_spec(specification)",
            "loader.exec_module(tool)",
            "with tempfile.TemporaryDirectory() as directory:",
            "    source = pathlib.Path(directory) / 'page.adoc'",
            "    source.write_text(sys.stdin.read())",
            "    generated = {'sync/flagDefinitions/VK_PAGE.adoc': ['  ** ename:VK_A', '  ** ename:VK_B']}",
            "    body = list(tool.spec_lines(source, {'VK_KHR_defined'}, generated=generated))",
            "items = tool.spec_described_items(body, 'VkPage')",
            "page = {'name': 'VkPage', 'kind': '', 'desc': '', 'aliases': [], 'file': '', 'items': items}",
            "docs = {'commit': '', 'attributes': {}, 'titles': {}, 'pages': [page], 'tables': []}",
            "print(json.dumps(docs))",
        ]
        .join("\n");
        let spawned = Command::new("python3")
            .args(["-B", "-c", &script])
            .arg(&tool)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn();
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) => {
                assert!(
                    std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
                    "the test of the dump is required but python3 does not start: {error}"
                );
                eprintln!("skipping the test of the dump: python3 does not start: {error}");
                return None;
            }
        };
        child
            .stdin
            .take()
            .expect("the input of python3")
            .write_all(source.join("\n").as_bytes())
            .expect("write the source");
        let output = child.wait_with_output().expect("run python3");
        if !output.status.success() {
            return Some(Err(String::from_utf8_lossy(&output.stderr).into_owned()));
        }
        let json = String::from_utf8(output.stdout).expect("a dump in UTF-8");
        let docs = Docs::from_json(&json).unwrap_or_else(|error| panic!("{error}: {json}"));
        Some(Ok(docs.pages[0]
            .items
            .iter()
            .map(|item| (item.name.clone(), item.text.clone()))
            .collect()))
    }

    fn assert_dumped(source: &[&str], expected: &[(&str, &[&str])]) {
        let Some(dumped) = dumped(source) else {
            return;
        };
        let dumped = dumped.unwrap_or_else(|error| panic!("{}:\n{error}", source.join("\n")));
        let expected = expected
            .iter()
            .map(|(name, lines)| (name.to_string(), lines.join("\n")))
            .collect::<Vec<_>>();
        assert_eq!(dumped, expected, "{}", source.join("\n"));
    }

    const NEXT: (&str, &[&str]) = ("next", &["pname:next is the next member."]);

    #[test]
    fn the_dump_keeps_text_that_goes_on_with_markup_at_the_start_of_a_line() {
        assert_dumped(
            &[
                "  * [[limits-bias]] pname:bias is clamped to the range",
                "    [eq]#[-pname:maxBias,+pname:maxBias]#.",
                "    See <<limits-bias>>.",
                "  * [[limits-range]]",
                "    pname:range[2] is the",
                "    [eq]#[minimum, maximum]# range of pname:limits[0]",
                "    [first] and",
                "    [eq]#a[0] + a[1]",
                "// a comment",
                "    .",
                "  * pname:next is the next member.",
                "",
                "Not a part of the list.",
            ],
            &[
                (
                    "bias",
                    &[
                        "pname:bias is clamped to the range",
                        "[eq]#[-pname:maxBias,+pname:maxBias]#.",
                        "See <<limits-bias>>.",
                    ],
                ),
                (
                    "range",
                    &[
                        "pname:range[2] is the",
                        "    [eq]#[minimum, maximum]# range of pname:limits[0]",
                        "[first] and",
                        "    [eq]#a[0] + a[1]",
                        ".",
                    ],
                ),
                NEXT,
            ],
        );
    }

    #[test]
    fn the_dump_keeps_what_list_continuations_attach() {
        assert_dumped(
            &[
                "  * pname:formats lists the following formats:",
                "+",
                "  ** ename:VK_FORMAT_A",
                "  ** ename:VK_FORMAT_B",
                "+",
                "To query more, use the command.",
                "+",
                "[NOTE]",
                ".Why",
                "====",
                "The list is short.",
                "",
                "  * pname:inside is not an item of the page.",
                "====",
                "  * pname:nested has nested items:",
                "  ** one",
                "",
                "  ** two",
                "",
                "+",
                "Both count.",
                "",
                "  * pname:next is the next member.",
                "+",
                "",
                "  * pname:last is the last member.",
                "",
                "+",
                "It is attached all the same.",
                "",
                "Not a part of the list.",
            ],
            &[
                (
                    "formats",
                    &[
                        "pname:formats lists the following formats:",
                        "+",
                        "** ename:VK_FORMAT_A",
                        "** ename:VK_FORMAT_B",
                        "+",
                        "To query more, use the command.",
                        "+",
                        "[NOTE]",
                        ".Why",
                        "====",
                        "The list is short.",
                        "",
                        "  * pname:inside is not an item of the page.",
                        "====",
                    ],
                ),
                (
                    "nested",
                    &[
                        "pname:nested has nested items:",
                        "** one",
                        "",
                        "** two",
                        "",
                        "+",
                        "Both count.",
                    ],
                ),
                NEXT,
                (
                    "last",
                    &[
                        "pname:last is the last member.",
                        "",
                        "+",
                        "It is attached all the same.",
                    ],
                ),
            ],
        );
    }

    #[test]
    fn the_dump_keeps_text_after_a_conditional_part() {
        assert_dumped(
            &[
                "  * ename:VK_COPY creates a copy with the same parameters",
                "ifdef::VK_KHR_defined[]",
                "    with the exception of pname:size",
                "endif::VK_KHR_defined[]",
                "ifdef::VK_KHR_undefined[]",
                "    and of pname:offset",
                "endif::VK_KHR_undefined[]",
                "    .",
                "    If pname:src refers to other structures, so does pname:dst.",
                "  * pname:reference is either:",
                "  ** a handle obtained from",
                "ifdef::VK_KHR_defined[flink:vkGetDefined]",
                "ifdef::VK_KHR_defined+VK_KHR_undefined[or]",
                "ifdef::VK_KHR_undefined[flink:vkGetUndefined]",
                "     (used by the device), or",
                "ifndef::VK_KHR_defined[]",
                "  ** nothing at all, or",
                "endif::VK_KHR_defined[]",
                "  ** an object.",
                "ifdef::VK_KHR_defined[]",
                "  * pname:next is the next member.",
                "endif::VK_KHR_defined[]",
            ],
            &[
                (
                    "VK_COPY",
                    &[
                        "ename:VK_COPY creates a copy with the same parameters",
                        "with the exception of pname:size",
                        ".",
                        "If pname:src refers to other structures, so does pname:dst.",
                    ],
                ),
                (
                    "reference",
                    &[
                        "pname:reference is either:",
                        "** a handle obtained from",
                        "flink:vkGetDefined",
                        "(used by the device), or",
                        "** an object.",
                    ],
                ),
                NEXT,
            ],
        );
    }

    #[test]
    fn the_dump_ends_an_item_where_a_block_of_the_page_starts() {
        assert_dumped(
            &[
                "  * pname:table is described by the table below.",
                "[options=\"header\"]",
                "|====",
                "| pname:table | Meaning",
                "|====",
                "  * pname:listing comes before a listing.",
                "----",
                "code",
                "----",
                "  * pname:validity comes before the valid usage.",
                "include::{generated}/validity/structs/VkPage.adoc[]",
                "  * pname:paragraph comes before a paragraph.",
                "",
                "pname:paragraph is described here as well.",
                "  * pname:next is the next member.",
            ],
            &[
                ("table", &["pname:table is described by the table below."]),
                ("listing", &["pname:listing comes before a listing."]),
                (
                    "validity",
                    &["pname:validity comes before the valid usage."],
                ),
                ("paragraph", &["pname:paragraph comes before a paragraph."]),
                NEXT,
            ],
        );
    }

    #[test]
    fn the_dump_has_the_generated_items_in_place_of_their_include() {
        assert_dumped(
            &[
                "  * pname:generated is equivalent to the logical OR of:",
                "include::{generated}/sync/flagDefinitions/VK_PAGE.adoc[]",
                "  * pname:next is the next member.",
            ],
            &[
                (
                    "generated",
                    &[
                        "pname:generated is equivalent to the logical OR of:",
                        "** ename:VK_A",
                        "** ename:VK_B",
                    ],
                ),
                NEXT,
            ],
        );
        for include in [
            "include::{generated}/sync/flagDefinitions/VK_OTHER.adoc[]",
            "include::{generated}/sync/flagDefinitions/VK_PAGE.adoc[tag=x]",
            "include::{chapters}/other.adoc[]",
        ] {
            let source = ["  * pname:a is equivalent to the logical OR of:", include];
            let Some(dumped) = dumped(&source) else {
                return;
            };
            let error = dumped.expect_err(include);
            assert!(
                error.contains(&format!(
                    "VkPage: the list item of a goes on in a file that is not read: {include}"
                )),
                "{error}"
            );
        }
        assert_dumped(
            &[
                "  * Otherwise, the flags are:",
                "include::{generated}/sync/flagDefinitions/VK_OTHER.adoc[]",
                "  * pname:next is the next member.",
            ],
            &[NEXT],
        );
    }

    #[test]
    fn the_dump_keeps_a_list_of_another_kind_that_follows_an_empty_line() {
        assert_dumped(
            &[
                "  * pname:last is the last member.",
                "",
                "1::",
                "    For all members, the",
                "    note applies.",
                "",
                "",
                "2:: The second note.",
                "",
                "Not a part of the list.",
                "  * pname:numbered has steps:",
                "",
                ". the first",
                "  .. in detail",
                "  * pname:dashed has parts:",
                "",
                "  - one",
                "  * pname:lettered has cases:",
                "",
                "a. the first",
                "  * pname:counted has cases:",
                "",
                "    iv) the fourth",
                "  * pname:defined has terms:",
                "",
                "term;; its description",
                "  * pname:next is the next member.",
                "",
                "sname:VkPage::pname:next is not a term.",
            ],
            &[
                (
                    "last",
                    &[
                        "pname:last is the last member.",
                        "",
                        "1::",
                        "For all members, the",
                        "note applies.",
                        "",
                        "2:: The second note.",
                    ],
                ),
                (
                    "numbered",
                    &[
                        "pname:numbered has steps:",
                        "",
                        ". the first",
                        ".. in detail",
                    ],
                ),
                ("dashed", &["pname:dashed has parts:", "", "- one"]),
                (
                    "lettered",
                    &["pname:lettered has cases:", "", "a. the first"],
                ),
                (
                    "counted",
                    &["pname:counted has cases:", "", "iv) the fourth"],
                ),
                (
                    "defined",
                    &["pname:defined has terms:", "", "term;; its description"],
                ),
                NEXT,
            ],
        );
    }

    #[test]
    fn the_dump_gives_a_term_without_a_description_the_text_after_an_empty_line() {
        assert_dumped(
            &[
                "  * pname:last is the last member.",
                "",
                "1::",
                "",
                "    The note after an empty line",
                "    goes on.",
                "",
                "2::",
                "",
                "",
                "The second note.",
                "",
                "Not a part of the list.",
                "  * pname:lists has notes:",
                "",
                "1::",
                "",
                "  ** a nested item, not the description",
                "",
                "2:: with a description",
                "",
                "Not a part of the list either.",
                "  * pname:next is the next member.",
                "+",
                "  ** attached",
                "  ** and nested",
                "",
                "  * pname:end is the last member.",
            ],
            &[
                (
                    "last",
                    &[
                        "pname:last is the last member.",
                        "",
                        "1::",
                        "",
                        "The note after an empty line",
                        "goes on.",
                        "",
                        "2::",
                        "",
                        "The second note.",
                    ],
                ),
                (
                    "lists",
                    &[
                        "pname:lists has notes:",
                        "",
                        "1::",
                        "",
                        "** a nested item, not the description",
                        "",
                        "2:: with a description",
                    ],
                ),
                (
                    "next",
                    &[
                        "pname:next is the next member.",
                        "+",
                        "** attached",
                        "** and nested",
                    ],
                ),
                ("end", &["pname:end is the last member."]),
            ],
        );
    }

    #[test]
    fn the_dump_ends_an_item_at_a_comment_that_follows_an_empty_line() {
        assert_dumped(
            &[
                "  * pname:a is one of:",
                "",
                "// a comment",
                "  ** not a part of the item",
                "  * pname:b is one of:",
                "// a comment",
                "",
                "  ** a part of the item",
                "  * pname:next is the next member.",
            ],
            &[
                ("a", &["pname:a is one of:"]),
                ("b", &["pname:b is one of:", "", "** a part of the item"]),
                NEXT,
            ],
        );
    }

    #[test]
    fn the_dump_attaches_a_block_that_follows_a_list_continuation_after_one_empty_line() {
        assert_dumped(
            &[
                "  * pname:feature specifies the feature.",
                "+",
                "",
                ":refpage: VkPage",
                "",
                "If the sname:{refpage} structure is included, it is filled in.",
                "",
                "Not a part of the list.",
                "  * pname:note has a note.",
                "+",
                "",
                "[NOTE]",
                "====",
                "It is attached.",
                "====",
                "  * pname:twice is followed by two empty lines.",
                "+",
                "",
                "",
                "Not attached.",
                "  * pname:commented has a comment in the way.",
                "+",
                "// a comment",
                "",
                "Not attached either.",
                "  * pname:skipped has a comment before its block.",
                "+",
                "// a comment",
                "It is attached.",
                "  * pname:next is the next member.",
                "+",
                "",
                "  * pname:last is the last member.",
            ],
            &[
                (
                    "feature",
                    &[
                        "pname:feature specifies the feature.",
                        "+",
                        "If the sname:VkPage structure is included, it is filled in.",
                    ],
                ),
                (
                    "note",
                    &[
                        "pname:note has a note.",
                        "+",
                        "[NOTE]",
                        "====",
                        "It is attached.",
                        "====",
                    ],
                ),
                ("twice", &["pname:twice is followed by two empty lines."]),
                ("commented", &["pname:commented has a comment in the way."]),
                (
                    "skipped",
                    &[
                        "pname:skipped has a comment before its block.",
                        "+",
                        "It is attached.",
                    ],
                ),
                NEXT,
                ("last", &["pname:last is the last member."]),
            ],
        );
    }

    #[test]
    fn the_dump_stops_at_a_described_item_that_cannot_be_read_reliably() {
        for (source, problem) in [
            (
                vec!["  * pname:a is:", "+", "+", "More."],
                "two list continuations in a row",
            ),
            (
                vec!["  * pname:a is:", "+", "====", "Never closed."],
                "the block that `====` opens does not end",
            ),
            (
                vec!["  * pname:a is:", "[NOTE]", "A paragraph as a note."],
                "the attributes `[NOTE]` in the text of the item",
            ),
            (
                vec!["  * pname:a is:", "", "    An indented paragraph."],
                "an indented paragraph after an empty line",
            ),
            (
                vec!["  * pname:a is:", "+", "", "// a comment", "More."],
                "a comment after a list continuation and an empty line",
            ),
            (
                vec!["  * pname:a is:", "+", "", "", "// a comment", "More."],
                "a comment after a list continuation and an empty line",
            ),
            (
                vec![
                    "  * pname:a is:",
                    "+",
                    "[NOTE]",
                    "",
                    "====",
                    "A note.",
                    "====",
                ],
                "an empty line between the attributes of a block and the block",
            ),
            (
                vec!["  * pname:a is:", "+", ".Title", "", "", "  ** nested"],
                "a list continuation that attaches nothing",
            ),
            (
                vec!["  * pname:a is:", "+", "", "", "  ** nested"],
                "a list continuation that attaches nothing",
            ),
            (
                vec!["  * pname:a is:", "+", "    An indented paragraph."],
                "an indented line after a list continuation",
            ),
            (
                vec!["  * pname:a is:", "+", "", "    An indented paragraph."],
                "an indented line after a list continuation",
            ),
            (
                vec![
                    "  * pname:a is:",
                    "+",
                    "  ** nested",
                    "  * pname:b is next.",
                ],
                "a first-level item in the indented lines after a list continuation",
            ),
            (
                vec![
                    "  * pname:a is:",
                    "+",
                    "",
                    "  ** nested",
                    "more of it",
                    "  * pname:b is next.",
                ],
                "a first-level item in the indented lines after a list continuation",
            ),
            (
                vec!["  * pname:a is:", "    // not a comment", "    more"],
                "an indented line that starts with `//`",
            ),
            (
                vec!["  * pname:a is:", "", "    // not a comment"],
                "an indented line that starts with `//`",
            ),
            (
                vec![
                    "  * pname:a is:",
                    "",
                    "1::",
                    "",
                    "[NOTE]",
                    "====",
                    "A note.",
                    "====",
                ],
                "a term whose description after an empty line is not text",
            ),
        ] {
            let Some(dumped) = dumped(&source) else {
                return;
            };
            let error = dumped.expect_err(&source.join("\n"));
            assert!(
                error.contains(&format!(
                    "VkPage: the list item of a cannot be read reliably: {problem}"
                )),
                "{error}"
            );
        }
        assert_dumped(
            &[
                "  * Otherwise:",
                "+",
                "+",
                "More.",
                "  * pname:next is the next member.",
            ],
            &[NEXT],
        );
    }
}
