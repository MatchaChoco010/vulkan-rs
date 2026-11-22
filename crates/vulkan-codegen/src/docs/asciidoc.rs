use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Link {
    Type,
    EnumValue,
    Command,
    Define,
    Extension,
    ReferencePage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Unsupported {
    Attribute(String),
    Macro(String),
    Unterminated(&'static str),
}

pub(crate) type Converted = Result<String, Unsupported>;

const NORMATIVE_WORDS: &[&str] = &[
    "can",
    "cannot",
    "may",
    "must",
    "optional",
    "optionally",
    "required",
    "should",
    "undefined",
];

const LINK_MACROS: &[(&str, Link)] = &[
    ("slink", Link::Type),
    ("elink", Link::Type),
    ("tlink", Link::Type),
    ("basetype", Link::Type),
    ("ename", Link::EnumValue),
    ("flink", Link::Command),
    ("dlink", Link::Define),
    ("apiext", Link::Extension),
    ("reflink", Link::ReferencePage),
];

const CODE_MACROS: &[&str] = &[
    "pname", "ptext", "sname", "stext", "fname", "ftext", "etext", "dname", "tname", "code",
];

const STRONG_MACROS: &[&str] = &["tag", "attr"];

const UNSUPPORTED_MACROS: &[&str] = &["footnote", "image", "xref", "stem", "anchor", "pass"];

pub(crate) struct Converter<'a> {
    pub(crate) attributes: &'a BTreeMap<String, String>,
    pub(crate) titles: &'a BTreeMap<String, String>,
    pub(crate) resolve: &'a dyn Fn(Link, &str) -> Option<String>,
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn word_len(text: &str, extra: &[char]) -> usize {
    text.find(|c: char| !(is_word(c) || extra.contains(&c)))
        .unwrap_or(text.len())
}

fn member_path_len(text: &str) -> usize {
    let mut len = word_len(text, &[]);
    if len == 0 {
        return 0;
    }
    loop {
        let rest = &text[len..];
        let separator = if rest.starts_with("->") {
            2
        } else if rest.starts_with('.') {
            1
        } else {
            return len;
        };
        let next = word_len(&rest[separator..], &[]);
        if next == 0 {
            return len;
        }
        len += separator + next;
    }
}

fn code_len(text: &str) -> usize {
    let mut len = word_len(text, &[]);
    if len == 0 {
        return 0;
    }
    loop {
        let rest = &text[len..];
        let joined = rest.starts_with('.') || rest.starts_with('*');
        let next = if joined { word_len(&rest[1..], &[]) } else { 0 };
        if next == 0 {
            break;
        }
        len += 1 + next;
    }
    len + text[len..].chars().take_while(|c| *c == '*').count()
}

fn code_span(code: &str) -> String {
    if code.contains('`') {
        format!("`` {code} ``")
    } else {
        format!("`{code}`")
    }
}

fn closing_bracket(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    for (index, c) in text.char_indices() {
        match c {
            '[' => depth += 1,
            ']' if depth == 0 => return Some(index),
            ']' => depth -= 1,
            _ => {}
        }
    }
    None
}

impl Converter<'_> {
    pub(crate) fn inline(&self, text: &str) -> Converted {
        let text = self.substitute_attributes(text)?;
        self.formatted(&text)
    }

    fn substitute_attributes(&self, text: &str) -> Converted {
        const MATH: &str = "latexmath:[";
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(open) = rest.find('{') {
            if let Some(math) = rest[..open].find(MATH) {
                let body = math + MATH.len();
                let close =
                    closing_bracket(&rest[body..]).ok_or(Unsupported::Unterminated("latexmath"))?;
                out.push_str(&rest[..body + close + 1]);
                rest = &rest[body + close + 1..];
                continue;
            }
            out.push_str(&rest[..open]);
            let after = &rest[open + 1..];
            let name_len = after
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '-'))
                .unwrap_or(after.len());
            if name_len > 0 && after[name_len..].starts_with('}') {
                let name = &after[..name_len];
                let value = self
                    .attributes
                    .get(name)
                    .ok_or_else(|| Unsupported::Attribute(name.to_string()))?;
                out.push_str(value);
                rest = &after[name_len + 1..];
            } else {
                out.push('{');
                rest = after;
            }
        }
        out.push_str(rest);
        Ok(out)
    }

    fn formatted(&self, text: &str) -> Converted {
        let mut out = String::with_capacity(text.len() + text.len() / 4);
        let mut index = 0;
        let mut previous = None::<char>;
        while index < text.len() {
            let rest = &text[index..];
            let at_word_start = previous.is_none_or(|c| !is_word(c));
            if at_word_start && let Some((consumed, markdown)) = self.named_macro(rest)? {
                out.push_str(&markdown);
                previous = rest[..consumed].chars().next_back();
                index += consumed;
                continue;
            }
            let (consumed, markdown) = self.markup(rest, previous)?;
            out.push_str(&markdown);
            previous = rest[..consumed].chars().next_back();
            index += consumed;
        }
        Ok(out)
    }

    fn named_macro(&self, rest: &str) -> Result<Option<(usize, String)>, Unsupported> {
        let name_len = rest
            .find(|c: char| !c.is_ascii_lowercase())
            .unwrap_or(rest.len());
        if name_len == 0 || !rest[name_len..].starts_with(':') {
            return Ok(None);
        }
        let name = &rest[..name_len];
        let argument = &rest[name_len + 1..];
        let consumed = |argument_len: usize| name_len + 1 + argument_len;

        if NORMATIVE_WORDS.contains(&name) {
            let suffix = word_len(argument, &[]);
            return Ok(Some((
                consumed(suffix),
                format!("{name}{}", &argument[..suffix]),
            )));
        }
        if let Some((_, link)) = LINK_MACROS
            .iter()
            .find(|(macro_name, _)| *macro_name == name)
        {
            let len = match link {
                Link::ReferencePage => word_len(argument, &['-']),
                _ => word_len(argument, &[]),
            };
            if len == 0 {
                return Ok(None);
            }
            let target = &argument[..len];
            let markdown = match (self.resolve)(*link, target) {
                Some(destination) => format!("[`{target}`]({destination})"),
                None => code_span(target),
            };
            return Ok(Some((consumed(len), markdown)));
        }
        if CODE_MACROS.contains(&name) {
            let len = match name {
                "pname" => member_path_len(argument),
                "ptext" | "stext" | "ftext" | "etext" => word_len(argument, &['*']),
                "code" => code_len(argument),
                _ => word_len(argument, &[]),
            };
            if len == 0 {
                return Ok(None);
            }
            return Ok(Some((consumed(len), code_span(&argument[..len]))));
        }
        if STRONG_MACROS.contains(&name) {
            let len = word_len(argument, &[]);
            if len == 0 {
                return Ok(None);
            }
            return Ok(Some((consumed(len), format!("**{}**", &argument[..len]))));
        }
        if name == "latexmath" && argument.starts_with('[') {
            let close =
                closing_bracket(&argument[1..]).ok_or(Unsupported::Unterminated("latexmath"))?;
            return Ok(Some((
                consumed(close + 2),
                code_span(argument[1..1 + close].trim()),
            )));
        }
        if matches!(name, "http" | "https" | "link") && !argument.is_empty() {
            return self.url(name, argument, name_len).map(Some);
        }
        if UNSUPPORTED_MACROS.contains(&name) && argument.starts_with(|c: char| !c.is_whitespace())
        {
            return Err(Unsupported::Macro(name.to_string()));
        }
        Ok(None)
    }

    fn url(
        &self,
        scheme: &str,
        argument: &str,
        name_len: usize,
    ) -> Result<(usize, String), Unsupported> {
        let url_len = argument
            .find(|c: char| c.is_whitespace() || c == '[')
            .unwrap_or(argument.len());
        let address = match scheme {
            "link" => argument[..url_len].to_string(),
            _ => format!("{scheme}:{}", &argument[..url_len]),
        };
        let after = &argument[url_len..];
        let Some(label) = after.strip_prefix('[') else {
            let trimmed = address.trim_end_matches(['.', ',', ')', ';']);
            let consumed = name_len + 1 + url_len - (address.len() - trimmed.len());
            return Ok((consumed, format!("<{trimmed}>")));
        };
        let close = closing_bracket(label).ok_or(Unsupported::Unterminated("link"))?;
        let consumed = name_len + 1 + url_len + close + 2;
        let label = label[..close].trim_end_matches("^").trim();
        if label.is_empty() {
            return Ok((consumed, format!("<{address}>")));
        }
        Ok((consumed, format!("[{}]({address})", self.formatted(label)?)))
    }

    fn markup(&self, rest: &str, previous: Option<char>) -> Result<(usize, String), Unsupported> {
        let c = rest
            .chars()
            .next()
            .expect("markup is called before the end");
        let after = &rest[c.len_utf8()..];
        if rest.starts_with("<<") {
            let close = rest
                .find(">>")
                .ok_or(Unsupported::Unterminated("cross reference"))?;
            let (anchor, label) = match rest[2..close].split_once(',') {
                Some((anchor, label)) => (anchor.trim(), label.trim()),
                None => (rest[2..close].trim(), ""),
            };
            let label = match (label, self.titles.get(anchor)) {
                ("", Some(title)) => title,
                ("", None) => return Ok((close + 2, format!("\\[{anchor}\\]"))),
                (label, _) => label,
            };
            let label = label.split_whitespace().collect::<Vec<_>>().join(" ");
            return Ok((close + 2, self.formatted(&label)?));
        }
        if rest.starts_with("[[")
            && let Some(close) = rest.find("]]")
        {
            return Ok((close + 2, String::new()));
        }
        if c == '['
            && let Some(close) = after.find("]#")
            && after[..close]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
            && let Some(end) = after[close + 2..].find('#')
        {
            let inner = &after[close + 2..close + 2 + end];
            return Ok((1 + close + 2 + end + 1, self.formatted(inner)?));
        }
        for (quote, open, close) in [("\"`", '“', '”'), ("'`", '‘', '’')] {
            let closing = quote.chars().rev().collect::<String>();
            if rest.starts_with(quote)
                && let Some(end) = rest[2..].find(&closing)
            {
                let inner = self.formatted(&rest[2..2 + end])?;
                return Ok((2 + end + 2, format!("{open}{inner}{close}")));
            }
        }
        if c == '`' {
            let Some(end) = after.find('`') else {
                return Ok((1, "\\`".to_string()));
            };
            let inner = &after[..end];
            let converted = self.formatted(inner)?;
            let markdown = if converted == escape(inner) || inner.contains('\\') {
                code_span(inner)
            } else {
                converted
            };
            return Ok((end + 2, markdown));
        }
        if matches!(c, '*' | '_') {
            let doubled = after.starts_with(c);
            let marker_len = if doubled { 2 } else { 1 };
            let marker = &rest[..marker_len];
            let body = &rest[marker_len..];
            let opens = doubled || previous.is_none_or(|p| !is_word(p));
            if opens
                && !body.starts_with(char::is_whitespace)
                && let Some(end) = closing_marker(body, marker, doubled)
            {
                let inner = self.formatted(&body[..end])?;
                let markdown = match c {
                    '*' => format!("**{inner}**"),
                    _ => format!("_{inner}_"),
                };
                return Ok((marker_len + end + marker_len, markdown));
            }
            return Ok((
                1,
                if c == '*' {
                    "\\*".to_string()
                } else {
                    "_".to_string()
                },
            ));
        }
        if matches!(c, '~' | '^')
            && let Some(end) = after.find(c)
            && end > 0
            && !after[..end].contains(char::is_whitespace)
        {
            let tag = if c == '~' { "sub" } else { "sup" };
            let inner = self.formatted(&after[..end])?;
            return Ok((end + 2, format!("<{tag}>{inner}</{tag}>")));
        }
        if c == '+' && previous == Some(' ') && after.starts_with('\n') {
            return Ok((1, String::new()));
        }
        Ok((c.len_utf8(), escape(&rest[..c.len_utf8()])))
    }
}

fn closing_marker(body: &str, marker: &str, unconstrained: bool) -> Option<usize> {
    let mut search_from = 0;
    while let Some(found) = body[search_from..].find(marker) {
        let end = search_from + found;
        let before = body[..end].chars().next_back();
        let after = body[end + marker.len()..].chars().next();
        let closes = before.is_some_and(|c| !c.is_whitespace())
            && (unconstrained || after.is_none_or(|c| !is_word(c)));
        if closes && end > 0 {
            return Some(end);
        }
        search_from = end + marker.len();
    }
    None
}

fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if matches!(c, '\\' | '[' | ']' | '<' | '>' | '`' | '*' | '~' | '|') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(text: &str) -> Converted {
        let attributes = BTreeMap::from([
            ("times".to_string(), "×".to_string()),
            ("YCbCr".to_string(), "Y′C~B~C~R~".to_string()),
            ("plus".to_string(), "+ ".to_string()),
        ]);
        let titles = BTreeMap::from([("clears-values".to_string(), "Clear Values".to_string())]);
        let resolve = |link: Link, name: &str| match (link, name) {
            (Link::Type, "VkExtent2D") => Some("Extent2D".to_string()),
            (Link::EnumValue, "VK_TRUE") => Some("TRUE".to_string()),
            (Link::Command, "vkCreateDevice") => Some("crate::Device::create_device".to_string()),
            (Link::Extension, "VK_KHR_surface") => Some("KHR_SURFACE_NAME".to_string()),
            (Link::ReferencePage, name) => Some(format!("https://example.org/{name}.html")),
            _ => None,
        };
        Converter {
            attributes: &attributes,
            titles: &titles,
            resolve: &resolve,
        }
        .inline(text)
    }

    fn markdown(text: &str) -> String {
        convert(text).unwrap_or_else(|error| panic!("{text}: {error:?}"))
    }

    #[test]
    fn names_of_parameters_and_members_become_code() {
        assert_eq!(
            markdown("pname:width is the width, see pname:pCreateInfo->flags and pname:a.b."),
            "`width` is the width, see `pCreateInfo->flags` and `a.b`."
        );
        assert_eq!(
            markdown("code:uint32_t* and code:float"),
            "`uint32_t*` and `float`"
        );
        assert_eq!(
            markdown("etext:VK_*_BIT, sname:VkFoo, fname:vkBar, dname:VK_X, tname:VkT"),
            "`VK_*_BIT`, `VkFoo`, `vkBar`, `VK_X`, `VkT`"
        );
    }

    #[test]
    fn link_macros_link_what_resolves_and_keep_the_c_name_as_text() {
        assert_eq!(
            markdown("a slink:VkExtent2D structure or slink:VkUnknown"),
            "a [`VkExtent2D`](Extent2D) structure or `VkUnknown`"
        );
        assert_eq!(
            markdown("ename:VK_TRUE, flink:vkCreateDevice, apiext:VK_KHR_surface"),
            "[`VK_TRUE`](TRUE), [`vkCreateDevice`](crate::Device::create_device), [`VK_KHR_surface`](KHR_SURFACE_NAME)"
        );
        assert_eq!(
            markdown("slink:VkExtent2D::pname:width"),
            "[`VkExtent2D`](Extent2D)::`width`"
        );
        assert_eq!(
            markdown("reflink:provisional-headers"),
            "[`provisional-headers`](https://example.org/provisional-headers.html)"
        );
    }

    #[test]
    fn normative_words_lose_their_colon() {
        assert_eq!(
            markdown("must: be valid, can: be NULL, should: not, may: be, cannot: be"),
            "must be valid, can be NULL, should not, may be, cannot be"
        );
        assert_eq!(markdown("this is undefined:."), "this is undefined.");
    }

    #[test]
    fn words_ending_in_a_macro_name_are_not_macros() {
        assert_eq!(
            markdown("the barcode:x and encode: this"),
            "the barcode:x and encode: this"
        );
        assert_eq!(markdown("note: pname: alone"), "note: pname: alone");
    }

    #[test]
    fn cross_references_keep_their_text() {
        assert_eq!(
            markdown(
                "See <<devsandqueues-priority,Queue Priority>> and <<a, pname:x of\nthe page>>."
            ),
            "See Queue Priority and `x` of the page."
        );
        assert_eq!(markdown("as in <<clears-values>>."), "as in Clear Values.");
        assert_eq!(markdown("See <<fundamentals>>."), "See \\[fundamentals\\].");
    }

    #[test]
    fn attributes_are_replaced_and_unknown_ones_are_refused() {
        assert_eq!(markdown("width {times} height"), "width × height");
        assert_eq!(
            markdown("{YCbCr} data"),
            "Y′C<sub>B</sub>C<sub>R</sub> data"
        );
        assert_eq!(markdown("a {0, 1} set"), "a {0, 1} set");
        assert_eq!(
            convert("{maxBlockSize} bytes"),
            Err(Unsupported::Attribute("maxBlockSize".to_string()))
        );
    }

    #[test]
    fn formatting_becomes_markdown() {
        assert_eq!(
            markdown("is *not* a _real_ value"),
            "is **not** a _real_ value"
        );
        assert_eq!(markdown("2 * x and a_b_c"), "2 \\* x and a_b_c");
        assert_eq!(markdown("is `NULL` or `0`"), "is `NULL` or `0`");
        assert_eq!(
            markdown("the `apiext:VK_KHR_surface` extension"),
            "the [`VK_KHR_surface`](KHR_SURFACE_NAME) extension"
        );
        assert_eq!(markdown("a \"`quoted`\" word"), "a “quoted” word");
        assert_eq!(
            markdown("[eq]#(pname:offset {plus} pname:size)#"),
            "(`offset` +  `size`)"
        );
        assert_eq!(
            markdown("x^2^ and H~2~O"),
            "x<sup>2</sup> and H<sub>2</sub>O"
        );
    }

    #[test]
    fn characters_that_markdown_would_interpret_are_escaped() {
        assert_eq!(
            markdown("pname:pProperties[i] where 0 < i and i > 1 in <stddef.h>"),
            "`pProperties`\\[i\\] where 0 \\< i and i \\> 1 in \\<stddef.h\\>"
        );
        assert_eq!(markdown("a | b ~ c"), "a \\| b \\~ c");
    }

    #[test]
    fn anchors_are_dropped_and_math_becomes_code() {
        assert_eq!(markdown("[[limits-x]]pname:x is"), "`x` is");
        assert_eq!(
            markdown("of latexmath:[\\lceil{\\mathit{n} \\over 32}\\rceil] words"),
            "of `\\lceil{\\mathit{n} \\over 32}\\rceil` words"
        );
        assert_eq!(markdown("latexmath:[a[i]] ok"), "`a[i]` ok");
    }

    #[test]
    fn urls_become_links() {
        assert_eq!(
            markdown(
                "see https://www.khronos.org/registry[the registry] or link:https://a.b/c[this^]."
            ),
            "see [the registry](https://www.khronos.org/registry) or [this](https://a.b/c)."
        );
        assert_eq!(markdown("at https://a.b/c."), "at <https://a.b/c>.");
    }

    #[test]
    fn constructs_without_a_markdown_form_are_refused() {
        assert_eq!(
            convert("text footnote:[more]"),
            Err(Unsupported::Macro("footnote".to_string()))
        );
        assert_eq!(
            convert("see <<a,b"),
            Err(Unsupported::Unterminated("cross reference"))
        );
    }
}
