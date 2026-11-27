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
    UnknownMacro(String),
    MacroUse(String),
    EscapedSpan,
    Passthrough,
    Unterminated(&'static str),
    Block(String),
    Include,
    AnchorWithoutPage(String),
}

pub(crate) type Converted = Result<String, Unsupported>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Argument {
    Nothing,
    OptionalWord,
    Word,
    WordWithStars,
    WordWithHyphens,
    MemberPath,
    MemberPathWithStars,
    Code,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Rendering {
    Name,
    Code,
    Link(Link),
}

const SPEC_MACROS: &[(&str, Argument, Rendering)] = &[
    ("can", Argument::OptionalWord, Rendering::Name),
    ("cannot", Argument::OptionalWord, Rendering::Name),
    ("may", Argument::OptionalWord, Rendering::Name),
    ("must", Argument::OptionalWord, Rendering::Name),
    ("optional", Argument::OptionalWord, Rendering::Name),
    ("optionally", Argument::OptionalWord, Rendering::Name),
    ("required", Argument::OptionalWord, Rendering::Name),
    ("should", Argument::OptionalWord, Rendering::Name),
    (
        "reflink",
        Argument::WordWithHyphens,
        Rendering::Link(Link::ReferencePage),
    ),
    ("apiext", Argument::Word, Rendering::Link(Link::Extension)),
    ("flink", Argument::Word, Rendering::Link(Link::Command)),
    ("fname", Argument::Word, Rendering::Code),
    ("ftext", Argument::WordWithStars, Rendering::Code),
    ("sname", Argument::Word, Rendering::Code),
    ("slink", Argument::Word, Rendering::Link(Link::Type)),
    ("stext", Argument::WordWithStars, Rendering::Code),
    ("ename", Argument::Word, Rendering::Link(Link::EnumValue)),
    ("elink", Argument::Word, Rendering::Link(Link::Type)),
    ("etext", Argument::WordWithStars, Rendering::Code),
    ("pname", Argument::MemberPath, Rendering::Code),
    ("ptext", Argument::MemberPathWithStars, Rendering::Code),
    ("dname", Argument::Word, Rendering::Code),
    ("dlink", Argument::Word, Rendering::Link(Link::Define)),
    ("tname", Argument::Word, Rendering::Code),
    ("tlink", Argument::Word, Rendering::Link(Link::Type)),
    ("basetype", Argument::Word, Rendering::Link(Link::Type)),
    ("code", Argument::Code, Rendering::Code),
    ("attr", Argument::Word, Rendering::Code),
    ("tag", Argument::Word, Rendering::Code),
    ("undefined", Argument::Nothing, Rendering::Name),
];

const UNSUPPORTED_MACROS: &[&str] = &["footnote", "image", "xref", "stem", "anchor", "pass"];

pub(crate) struct Converter<'a> {
    pub(crate) attributes: &'a BTreeMap<String, String>,
    pub(crate) titles: &'a BTreeMap<String, String>,
    pub(crate) section: &'a dyn Fn(&str) -> Option<String>,
    pub(crate) resolve: &'a dyn Fn(Link, &str) -> Option<String>,
}

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_any_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn word_len(text: &str, extra: &[char]) -> usize {
    text.find(|c: char| !(is_word(c) || extra.contains(&c)))
        .unwrap_or(text.len())
}

fn member_path_len(text: &str, extra: &[char]) -> usize {
    let mut len = word_len(text, extra);
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
        let next = word_len(&rest[separator..], extra);
        if next == 0 {
            return len;
        }
        len += separator + next;
    }
}

fn hyphenated_word_len(text: &str) -> usize {
    let mut len = 0;
    while let Some(c) = text[len..].chars().next() {
        let rest = &text[len..];
        let arrow = rest.starts_with("->");
        let dash = rest.starts_with("--")
            && text[..len].ends_with(is_any_word)
            && rest[2..].starts_with(is_any_word);
        if !(is_word(c) || c == '-') || arrow || dash {
            break;
        }
        len += 1;
    }
    len
}

impl Argument {
    fn len(self, text: &str) -> Option<usize> {
        let len = match self {
            Argument::Nothing => return Some(0),
            Argument::OptionalWord => return Some(word_len(text, &[])),
            Argument::Word => word_len(text, &[]),
            Argument::WordWithStars => word_len(text, &['*']),
            Argument::WordWithHyphens => hyphenated_word_len(text),
            Argument::MemberPath => member_path_len(text, &[]),
            Argument::MemberPathWithStars => member_path_len(text, &['*']),
            Argument::Code => code_len(text),
        };
        (len > 0).then_some(len)
    }
}

fn spec_macro_argument_len(
    order: usize,
    argument: &str,
    in_a_word: bool,
) -> Result<Option<usize>, Unsupported> {
    let (name, taken, _) = SPEC_MACROS[order];
    let Some(mut len) = taken.len(argument) else {
        return Ok(None);
    };
    if let Some(after) = argument[len..].strip_prefix(':') {
        for (earlier, (earlier_name, _, _)) in SPEC_MACROS[..order].iter().enumerate() {
            let Some(before) = argument[..len].strip_suffix(earlier_name) else {
                continue;
            };
            if spec_macro_argument_len(earlier, after, before.ends_with(is_word))?.is_some() {
                let Some(cut_short) = taken.len(before) else {
                    return Ok(None);
                };
                len = cut_short;
                break;
            }
        }
    }
    let leaves_a_word_out = taken == Argument::OptionalWord && len > 0;
    match (in_a_word, len) {
        (true, 0) => Ok(None),
        (false, _) if !leaves_a_word_out => Ok(Some(len)),
        _ => Err(Unsupported::MacroUse(format!(
            "{name}:{}",
            &argument[..len]
        ))),
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

pub(super) fn code_span(code: &str) -> String {
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
        self.formatted_with(text, true)
    }

    fn formatted_with(&self, text: &str, replacements: bool) -> Converted {
        let mut out = String::with_capacity(text.len() + text.len() / 4);
        let mut index = 0;
        let mut previous = None::<char>;
        while index < text.len() {
            let rest = &text[index..];
            let at_word_start = previous.is_none_or(|c| !is_word(c));
            if let Some((consumed, markdown)) = self.named_macro(rest, at_word_start)? {
                out.push_str(&markdown);
                previous = rest[..consumed].chars().next_back();
                index += consumed;
                continue;
            }
            let (consumed, markdown) = self.markup(rest, previous, replacements)?;
            out.push_str(&markdown);
            previous = rest[..consumed].chars().next_back();
            index += consumed;
        }
        Ok(out)
    }

    fn named_macro(
        &self,
        rest: &str,
        at_word_start: bool,
    ) -> Result<Option<(usize, String)>, Unsupported> {
        let name_len = rest
            .find(|c: char| !c.is_ascii_lowercase())
            .unwrap_or(rest.len());
        if name_len == 0 || !rest[name_len..].starts_with(':') {
            return Ok(None);
        }
        let name = &rest[..name_len];
        let argument = &rest[name_len + 1..];
        let consumed = |argument_len: usize| name_len + 1 + argument_len;

        if let Some(order) = SPEC_MACROS.iter().position(|(spec, _, _)| *spec == name) {
            let Some(len) = spec_macro_argument_len(order, argument, !at_word_start)? else {
                return Ok(None);
            };
            let target = &argument[..len];
            let markdown = match SPEC_MACROS[order].2 {
                Rendering::Name => name.to_string(),
                Rendering::Code => code_span(target),
                Rendering::Link(link) => match (self.resolve)(link, target) {
                    Some(destination) => format!("[`{target}`]({destination})"),
                    None => code_span(target),
                },
            };
            return Ok(Some((consumed(len), markdown)));
        }
        if !at_word_start {
            return Ok(None);
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
        let target_len = argument
            .find(|c: char| c.is_whitespace() || c == '[' || c == ']')
            .unwrap_or(argument.len());
        if argument[target_len..].starts_with('[') {
            return Err(Unsupported::UnknownMacro(name.to_string()));
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

    fn markup(
        &self,
        rest: &str,
        previous: Option<char>,
        replacements: bool,
    ) -> Result<(usize, String), Unsupported> {
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
                ("", None) => {
                    let section = (self.section)(anchor)
                        .ok_or_else(|| Unsupported::AnchorWithoutPage(anchor.to_string()))?;
                    return Ok((close + 2, format!("[\\[{anchor}\\]]({section})")));
                }
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
            && let Some(close) = role_len(after)
        {
            let body = &after[close + 2..];
            if let Some(body) = body.strip_prefix('#') {
                let first = body.chars().next().map_or(0, char::len_utf8);
                if let Some(end) = body[first..].find("##") {
                    let end = first + end;
                    return Ok((1 + close + 3 + end + 2, self.formatted(&body[..end])?));
                }
            } else if let Some(end) = body.find('#') {
                return Ok((1 + close + 2 + end + 1, self.formatted(&body[..end])?));
            }
        }
        if c == '\\'
            && (after.starts_with('#') || after.starts_with('[') && role_len(&after[1..]).is_some())
        {
            return Err(Unsupported::EscapedSpan);
        }
        if c == '\\' && after.starts_with("--") {
            return Ok((3, "--".to_string()));
        }
        if replacements && rest.starts_with("--") {
            let next = rest[2..].chars().next();
            let bounds = |c: Option<char>| c.is_none_or(|c| c == ' ' || c == '\n');
            let between_words = previous.is_some_and(is_any_word) && next.is_some_and(is_any_word);
            if between_words || (bounds(previous) && bounds(next)) {
                return Ok((2, "—".to_string()));
            }
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
            if let Some(literal) = passthrough(inner).filter(|len| *len == inner.len()) {
                return Ok((end + 2, code_span(&inner[1..literal - 1])));
            }
            if contains_passthrough(inner) {
                return Err(Unsupported::Passthrough);
            }
            let markdown =
                if self.formatted_with(inner, false)? == escape(inner) || inner.contains('\\') {
                    code_span(inner)
                } else {
                    self.formatted(inner)?
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
        if c == '+' && previous.is_none_or(|p| !is_word(p)) {
            if let Some(len) = passthrough(rest) {
                return Ok((len, escape(&rest[1..len - 1])));
            }
            if opens_a_passthrough(rest) {
                return Err(Unsupported::Passthrough);
            }
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
        if closes {
            return Some(end);
        }
        search_from = end + marker.len();
    }
    None
}

fn passthrough(text: &str) -> Option<usize> {
    let body = text.strip_prefix('+')?;
    let end = body.find(['+', '\n'])?;
    let content = &body[..end];
    let closes = body[end..].starts_with('+')
        && !content.is_empty()
        && !content.starts_with(char::is_whitespace)
        && !content.ends_with(char::is_whitespace)
        && body[end + 1..].chars().next().is_none_or(|c| !is_word(c));
    closes.then_some(end + 2)
}

fn opens_a_passthrough(text: &str) -> bool {
    let Some(body) = text.strip_prefix('+') else {
        return false;
    };
    if body.is_empty() || body.starts_with(char::is_whitespace) {
        return false;
    }
    body.starts_with('+')
        || body.match_indices('+').any(|(at, _)| {
            !body[..at].ends_with(char::is_whitespace)
                && body[at + 1..].chars().next().is_none_or(|c| !is_word(c))
        })
}

fn contains_passthrough(text: &str) -> bool {
    text.char_indices().any(|(at, c)| {
        c == '+'
            && text[..at].chars().next_back().is_none_or(|p| !is_word(p))
            && (passthrough(&text[at..]).is_some() || opens_a_passthrough(&text[at..]))
    })
}

fn role_len(text: &str) -> Option<usize> {
    let close = text.find("]#")?;
    text[..close]
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
        .then_some(close)
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
    use proptest::prelude::*;

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
        let section = |anchor: &str| {
            (anchor != "nowhere").then(|| format!("https://example.org/spec.html#{anchor}"))
        };
        Converter {
            attributes: &attributes,
            titles: &titles,
            section: &section,
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

    fn refused(text: &str, macro_use: &str) {
        assert_eq!(
            convert(text),
            Err(Unsupported::MacroUse(macro_use.to_string())),
            "{text:?}"
        );
    }

    #[test]
    fn a_word_after_the_colon_of_a_normative_macro_is_refused() {
        for (text, macro_use) in [
            ("must:be x", "must:be"),
            ("it can:not y", "can:not"),
            ("may:_", "may:_"),
            ("optional:1x.", "optional:1x"),
            ("a must:should: z", "must:should"),
            ("pname:a.must:be", "must:be"),
        ] {
            refused(text, macro_use);
        }
        assert_eq!(
            markdown("required:. undefined:x and must:: and should:can: y"),
            "required. undefinedx and must: and shouldcan y"
        );
    }

    #[test]
    fn a_macro_of_the_specification_in_a_word_is_refused_and_one_that_takes_nothing_is_text() {
        for (text, macro_use) in [
            ("the barcode:x", "code:x"),
            ("xpname:foo.bar", "pname:foo.bar"),
            ("a_pname:y", "pname:y"),
            ("scan:x", "can:x"),
            ("1must:be", "must:be"),
            ("pname:xename:VK_TRUE", "ename:VK_TRUE"),
            ("pname:xmust:be", "must:be"),
        ] {
            refused(text, macro_use);
        }
        let text =
            "scan: x, a rescan:, xundefined:y, encode: this, pseudocode: and note: pname: alone";
        assert_eq!(markdown(text), text);
        assert_eq!(
            markdown("pname:xmust: y and VkFoo::pname:x"),
            "`xmust`: y and VkFoo::`x`"
        );
        assert_eq!(
            markdown("_pname:pNext chain_ and *must:* and (pname:x)"),
            "_`pNext` chain_ and **must** and (`x`)"
        );
    }

    #[test]
    fn member_paths_of_ptext_and_the_names_of_tags_and_attributes_become_code() {
        assert_eq!(
            markdown("ptext:pCreateInfo->p* and ptext:a.b"),
            "`pCreateInfo->p*` and `a.b`"
        );
        assert_eq!(markdown("tag:enums and attr:len"), "`enums` and `len`");
    }

    #[test]
    fn a_macro_that_the_specification_registers_earlier_is_read_first() {
        assert_eq!(
            markdown("pname:ename:VK_TRUE and ename:pname:x"),
            "pname:[`VK_TRUE`](TRUE) and `pname`:x"
        );
        assert_eq!(
            markdown("pname:a.must: and should:can:"),
            "`a`.must and shouldcan"
        );
        assert_eq!(markdown("pname:pname:x"), "`pname`:x");
    }

    #[test]
    fn a_reference_page_name_ends_before_an_arrow_and_before_a_dash_between_words() {
        assert_eq!(
            markdown("reflink:provisional-headers- and reflink:-x"),
            "[`provisional-headers-`](https://example.org/provisional-headers-.html) and [`-x`](https://example.org/-x.html)"
        );
        assert_eq!(
            markdown("reflink:a->b and reflink:a--b and reflink:a-- b"),
            "[`a`](https://example.org/a.html)-\\>b and [`a`](https://example.org/a.html)—b and [`a--`](https://example.org/a--.html) b"
        );
        assert_eq!(
            markdown("reflink:a--é and é--a and é-- a"),
            "[`a`](https://example.org/a.html)—é and é—a and é-- a",
            "a word character on either side of a dash is any letter or digit"
        );
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
        assert_eq!(
            markdown("See <<fundamentals>> and <<samplers-mipLodBias>>."),
            "See [\\[fundamentals\\]](https://example.org/spec.html#fundamentals) and [\\[samplers-mipLodBias\\]](https://example.org/spec.html#samplers-mipLodBias)."
        );
        assert_eq!(
            convert("See <<nowhere>>."),
            Err(Unsupported::AnchorWithoutPage("nowhere".to_string()))
        );
        assert_eq!(markdown("See <<nowhere, somewhere>>."), "See somewhere.");
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
    fn unconstrained_role_spans_keep_only_their_text_anywhere_in_a_word() {
        assert_eq!(markdown("a zero, [eq]##Nan##s and"), "a zero, Nans and");
        assert_eq!(markdown("[eq]##x##y at the start"), "xy at the start");
        assert_eq!(markdown("at the end [eq]##x##"), "at the end x");
        assert_eq!(markdown("in[eq]##side##words"), "insidewords");
        assert_eq!(markdown("[eq]##pname:a {times} *b*##s"), "`a` × **b**s");
        assert_eq!(markdown("[eq]##a [eq]#b# c##"), "a b c");
        assert_eq!(markdown("[eq]#x# and [eq]##y##"), "x and y");
        assert_eq!(markdown("[eq]####x## y"), "##x y");
        assert_eq!(markdown("[eq]##a##b##"), "ab##");
    }

    #[test]
    fn unclosed_role_spans_stay_text() {
        assert_eq!(markdown("[eq]##x"), "\\[eq\\]##x");
        assert_eq!(markdown("[eq]####"), "\\[eq\\]####");
    }

    #[test]
    fn escaped_marks_and_role_spans_are_refused() {
        for text in ["a \\#b# c", "a \\##b## c", "\\[eq]#x#", "\\[eq]##x##s"] {
            assert_eq!(convert(text), Err(Unsupported::EscapedSpan), "{text}");
        }
        assert_eq!(markdown("a \\b and \\[x\\]"), "a \\\\b and \\\\\\[x\\\\\\]");
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Emission {
        Strong(String),
        Text(String),
        Code,
        Link(Vec<String>),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct Specified {
        name: String,
        pattern: String,
        emission: Emission,
    }

    fn specified_macros() -> Vec<Specified> {
        let specified = |name: &str, pattern: String, emission: Emission| Specified {
            name: name.to_string(),
            pattern,
            emission,
        };
        let strong = |name: &str| {
            specified(
                name,
                format!("{name}:(\\w*)"),
                Emission::Strong(name.to_string()),
            )
        };
        let code = |name: &str, argument: &str| {
            specified(name, format!("{name}:({argument})"), Emission::Code)
        };
        let link = |name: &str, argument: &str, dictionaries: &[&str]| {
            let dictionaries = dictionaries.iter().map(|name| name.to_string()).collect();
            specified(
                name,
                format!("{name}:({argument})"),
                Emission::Link(dictionaries),
            )
        };
        vec![
            strong("can"),
            strong("cannot"),
            strong("may"),
            strong("must"),
            strong("optional"),
            strong("optionally"),
            strong("required"),
            strong("should"),
            link("reflink", r"[-\w]+", &[]),
            link("apiext", r"\w+", &["features"]),
            link("flink", r"\w+", &["protos"]),
            code("fname", r"\w+"),
            code("ftext", r"[\w\*]+"),
            code("sname", r"\w+"),
            link("slink", r"\w+", &["structs", "handles"]),
            code("stext", r"[\w\*]+"),
            link("ename", r"\w+", &["consts", "enums"]),
            link("elink", r"\w+", &["enums"]),
            code("etext", r"[\w\*]+"),
            code("pname", r"\w+((\.|&#8594;)\w+)*"),
            code("ptext", r"[\w\*]+((\.|&#8594;)[\w\*]+)*"),
            code("dname", r"\w+"),
            link("dlink", r"\w+", &["defines"]),
            code("tname", r"\w+"),
            link("tlink", r"\w+", &["flags", "funcpointers", "defines"]),
            link("basetype", r"\w+", &["basetypes"]),
            code("code", r"\w+([.*]\w+)*\**"),
            code("attr", r"\w+"),
            code("tag", r"\w+"),
            specified(
                "undefined",
                "undefined:".to_string(),
                Emission::Text("undefined".to_string()),
            ),
        ]
    }

    #[derive(Default)]
    struct RubyClass {
        name: String,
        base: String,
        macro_name: Option<String>,
        pattern: Option<String>,
        text: Option<String>,
        quoted: Vec<String>,
        links: bool,
        dictionaries: Vec<String>,
    }

    fn ruby_classes(source: &str) -> Vec<RubyClass> {
        const DICTIONARY: &str = "$apiNames.";
        let mut classes = Vec::<RubyClass>::new();
        let mut lines = source.lines().map(str::trim);
        while let Some(line) = lines.next() {
            let header = line
                .strip_prefix("class ")
                .and_then(|header| header.split_once(" < "));
            if let Some((name, base)) = header {
                classes.push(RubyClass {
                    name: name.to_string(),
                    base: base.to_string(),
                    ..Default::default()
                });
                continue;
            }
            let Some(class) = classes.last_mut() else {
                continue;
            };
            if line.starts_with('#') {
                continue;
            }
            if let Some(name) = line.strip_prefix("named :") {
                class.macro_name = Some(name.to_string());
            } else if let Some(pattern) = line
                .strip_prefix("match /")
                .and_then(|pattern| pattern.strip_suffix('/'))
            {
                class.pattern = Some(pattern.to_string());
            } else if line == "def text" {
                class.text = lines.next().map(|text| text.trim_matches('\'').to_string());
            } else if let Some((_, expression)) = line.split_once("create_inline parent, :quoted, ")
            {
                class.quoted.push(expression.to_string());
            } else if line.contains("Inline.new(parent, :anchor, linkname,") {
                class.links = true;
            }
            for (at, _) in line.match_indices(DICTIONARY) {
                let name = &line[at + DICTIONARY.len()..];
                let name = &name[..word_len(name, &[])];
                if !class.dictionaries.iter().any(|known| known == name) {
                    class.dictionaries.push(name.to_string());
                }
            }
        }
        classes
    }

    fn emission(classes: &[RubyClass], class: &RubyClass) -> Emission {
        let mut emitter = class;
        while emitter.quoted.is_empty() && !emitter.links {
            emitter = classes
                .iter()
                .find(|base| base.name == emitter.base)
                .unwrap_or_else(|| panic!("{} and its base classes emit nothing", class.name));
        }
        let quoted = emitter
            .quoted
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let literal = |expression: &str| {
            expression
                .strip_prefix('\'')
                .and_then(|text| text.strip_suffix('\''))
                .filter(|text| word_len(text, &[]) == text.len())
                .map(str::to_string)
        };
        match (quoted.as_slice(), emitter.links) {
            (["'<strong class=\"purple\">' + text + '</strong>'"], false) => Emission::Strong(
                class
                    .text
                    .clone()
                    .unwrap_or_else(|| panic!("{} has no text", class.name)),
            ),
            (["'<code>' + target.gsub('&#8594;', '-&gt;') + '</code>'"], false) => Emission::Code,
            (["(mark ? '?? ' : '') + '<code>' + linkxref + '</code>'"], true) => {
                Emission::Link(class.dictionaries.clone())
            }
            ([expression], false) if literal(expression).is_some() => {
                Emission::Text(literal(expression).expect("a literal"))
            }
            _ => panic!(
                "the test does not know what {} emits: {quoted:?}",
                class.name
            ),
        }
    }

    fn registered_macros() -> Option<Vec<Specified>> {
        let config = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../third_party/vulkan-docs/config");
        let read = |file: &str| std::fs::read_to_string(config.join(file)).ok();
        let (Some(registration), Some(definitions)) =
            (read("spec-macros.rb"), read("spec-macros/extension.rb"))
        else {
            assert!(
                std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
                "the macro test is required but third_party/vulkan-docs is not checked out"
            );
            eprintln!("skipping the macro test: third_party/vulkan-docs is not checked out");
            return None;
        };
        let classes = ruby_classes(&definitions);
        let registered = registration
            .lines()
            .filter_map(|line| line.trim().strip_prefix("inline_macro "))
            .collect::<Vec<_>>();
        let defined = classes
            .iter()
            .filter(|class| class.macro_name.is_some())
            .map(|class| class.name.as_str())
            .collect::<Vec<_>>();
        for name in &defined {
            assert!(
                registered.contains(name),
                "the specification defines the macro class {name} and does not register it"
            );
        }
        let specified = registered.iter().map(|name| {
            let class = classes
                .iter()
                .find(|class| class.name == *name)
                .unwrap_or_else(|| panic!("the registered macro class {name} is not defined"));
            let part = |part: &Option<String>, what: &str| {
                part.clone()
                    .unwrap_or_else(|| panic!("the macro class {name} has no {what}"))
            };
            Specified {
                name: part(&class.macro_name, "name"),
                pattern: part(&class.pattern, "pattern"),
                emission: emission(&classes, class),
            }
        });
        Some(specified.collect())
    }

    #[test]
    fn the_macros_under_test_are_the_inline_macros_that_the_specification_registers() {
        let Some(registered) = registered_macros() else {
            return;
        };
        assert!(
            registered.len() >= 30,
            "only {} inline macros were found",
            registered.len()
        );
        let specified = specified_macros();
        for (index, registered) in registered.iter().enumerate() {
            assert_eq!(
                specified.get(index),
                Some(registered),
                "the macro that the specification registers at position {index}"
            );
        }
        assert_eq!(specified.len(), registered.len());
    }

    #[test]
    fn the_converter_knows_the_macros_under_test_in_the_order_of_the_specification() {
        let specified = specified_macros();
        assert_eq!(
            SPEC_MACROS
                .iter()
                .map(|(name, _, _)| *name)
                .collect::<Vec<_>>(),
            specified
                .iter()
                .map(|specified| specified.name.as_str())
                .collect::<Vec<_>>()
        );
        for (index, earlier) in specified.iter().enumerate() {
            for later in &specified[index + 1..] {
                assert!(
                    !later.name.ends_with(&earlier.name),
                    "`{}:` is read before `{}:`, which ends with its name: reading from left to \
                     right does not give what the specification gives",
                    earlier.name,
                    later.name
                );
            }
        }
    }

    const ARROW: char = '\u{1}';
    const DASH: char = '\u{2}';

    fn after_the_replacements(text: &str) -> String {
        let chars = text.chars().collect::<Vec<_>>();
        let in_a_word = |c: Option<&char>| c.is_some_and(|c| c.is_alphanumeric() || *c == '_');
        let at_a_bound = |c: Option<&char>| c.is_none_or(|c| *c == ' ' || *c == '\n');
        let mut out = String::with_capacity(text.len());
        let mut index = 0;
        while index < chars.len() {
            let before = index.checked_sub(1).map(|before| &chars[before]);
            let after = chars.get(index + 2);
            let hyphens = chars[index] == '-' && chars.get(index + 1) == Some(&'-');
            let dash = hyphens
                && (in_a_word(before) && in_a_word(after)
                    || at_a_bound(before) && at_a_bound(after));
            if dash {
                out.push(DASH);
                index += 2;
            } else {
                out.push(chars[index]);
                index += 1;
            }
        }
        out.replace("->", &ARROW.to_string())
    }

    fn link_kind(dictionaries: &[String]) -> Link {
        match dictionaries.first().map(String::as_str) {
            None => Link::ReferencePage,
            Some("structs" | "enums" | "flags" | "basetypes") => Link::Type,
            Some("consts") => Link::EnumValue,
            Some("protos") => Link::Command,
            Some("defines") => Link::Define,
            Some("features") => Link::Extension,
            Some(other) => panic!("the test does not know the dictionary `{other}`"),
        }
    }

    fn rendered(specified: &Specified, target: &str) -> String {
        let target = target.replace(ARROW, "->");
        match &specified.emission {
            Emission::Strong(text) | Emission::Text(text) => text.clone(),
            Emission::Code => format!("`{target}`"),
            Emission::Link(dictionaries) => {
                format!("[`{target}`]({:?}/{target})", link_kind(dictionaries))
            }
        }
    }

    fn patterns() -> &'static [(Specified, regex::Regex)] {
        static PATTERNS: std::sync::OnceLock<Vec<(Specified, regex::Regex)>> =
            std::sync::OnceLock::new();
        PATTERNS.get_or_init(|| {
            let compiled = |specified: Specified| {
                let pattern = specified.pattern.replace("&#8594;", "\\x01");
                let pattern = regex::Regex::new(&format!("(?-u:{pattern})"))
                    .unwrap_or_else(|error| panic!("{}: {error}", specified.pattern));
                (specified, pattern)
            };
            specified_macros().into_iter().map(compiled).collect()
        })
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum Piece {
        Text(String),
        Rendered { markdown: String, source: String },
        Refused(String),
    }

    fn last_character(pieces: &[Piece]) -> Option<char> {
        pieces.iter().rev().find_map(|piece| match piece {
            Piece::Text(source) | Piece::Refused(source) | Piece::Rendered { source, .. } => {
                source.chars().next_back()
            }
        })
    }

    fn leaves_a_word_out(specified: &Specified, target: &str) -> bool {
        matches!(specified.emission, Emission::Strong(_)) && !target.is_empty()
    }

    fn pieces_in_the_order(text: &str, order: &mut dyn Iterator<Item = usize>) -> Vec<Piece> {
        let mut pieces = vec![Piece::Text(after_the_replacements(text))];
        for index in order {
            let (specified, pattern) = &patterns()[index];
            let mut split = Vec::new();
            for piece in pieces {
                let Piece::Text(text) = piece else {
                    split.push(piece);
                    continue;
                };
                let mut end = 0;
                for found in pattern.captures_iter(&text) {
                    let whole = found.get(0).expect("the whole match");
                    let target = found.get(1).map_or("", |target| target.as_str());
                    let before = text[..whole.start()]
                        .chars()
                        .next_back()
                        .or_else(|| last_character(&split));
                    let in_a_word = before.is_some_and(is_word);
                    if in_a_word && target.is_empty() {
                        continue;
                    }
                    let source = whole.as_str().to_string();
                    split.push(Piece::Text(text[end..whole.start()].to_string()));
                    split.push(if in_a_word || leaves_a_word_out(specified, target) {
                        Piece::Refused(source)
                    } else {
                        Piece::Rendered {
                            markdown: rendered(specified, target),
                            source,
                        }
                    });
                    end = whole.end();
                }
                split.push(Piece::Text(text[end..].to_string()));
            }
            pieces = split;
        }
        pieces
    }

    fn as_the_converter_renders(text: &str) -> Option<String> {
        pieces_in_the_order(text, &mut (0..patterns().len()))
            .iter()
            .map(|piece| match piece {
                Piece::Text(text) => Some(
                    text.replace(ARROW, "-\\>")
                        .replace(DASH, "—")
                        .replace('*', "\\*"),
                ),
                Piece::Rendered { markdown, .. } => Some(markdown.clone()),
                Piece::Refused(_) => None,
            })
            .collect()
    }

    fn with_every_link_resolved<T>(read: impl FnOnce(&Converter) -> T) -> T {
        let (attributes, titles) = (BTreeMap::new(), BTreeMap::new());
        let resolve = |link: Link, name: &str| Some(format!("{link:?}/{name}"));
        let section = |anchor: &str| Some(format!("Section/{anchor}"));
        read(&Converter {
            attributes: &attributes,
            titles: &titles,
            section: &section,
            resolve: &resolve,
        })
    }

    fn taken_by_the_pattern(
        specified: &Specified,
        pattern: &regex::Regex,
        source: &str,
        in_a_word: bool,
    ) -> Result<Option<(usize, String)>, Unsupported> {
        let replaced = after_the_replacements(source);
        let Some(found) = pattern.captures(&replaced) else {
            return Ok(None);
        };
        let whole = found.get(0).expect("the whole match");
        assert_eq!(whole.start(), 0, "{source:?}");
        let target = found.get(1).map_or("", |target| target.as_str());
        let len = whole.as_str().replace(ARROW, "->").len();
        match (in_a_word, target.is_empty()) {
            (true, true) => Ok(None),
            (false, _) if !leaves_a_word_out(specified, target) => {
                Ok(Some((len, rendered(specified, target))))
            }
            _ => Err(Unsupported::MacroUse(source[..len].to_string())),
        }
    }

    fn assert_read_as_the_pattern_matches(converter: &Converter, index: usize, tail: &str) {
        let (specified, pattern) = &patterns()[index];
        let source = format!("{}:{tail}", specified.name);
        for in_a_word in [false, true] {
            assert_eq!(
                converter.named_macro(&source, !in_a_word),
                taken_by_the_pattern(specified, pattern, &source, in_a_word),
                "{source:?} is read as its pattern matches, except that a word after a \
                 normative macro and an argument of a macro in a word are refused and a macro \
                 that takes nothing in a word is text (in a word: {in_a_word})"
            );
        }
    }

    #[test]
    fn every_macro_takes_what_its_pattern_in_the_specification_matches() {
        let alphabet = ['a', 'Z', '0', '_', '.', '*', '-', '>', ':', ' ', 'é'];
        let mut tails = vec![String::new()];
        let mut longest = tails.clone();
        for _ in 0..3 {
            longest = longest
                .iter()
                .flat_map(|tail| alphabet.iter().map(move |c| format!("{tail}{c}")))
                .collect();
            tails.extend(longest.iter().cloned());
        }
        with_every_link_resolved(|converter| {
            for index in 0..patterns().len() {
                for tail in &tails {
                    assert_read_as_the_pattern_matches(converter, index, tail);
                }
            }
        });
    }

    #[test]
    fn passthroughs_keep_their_text_as_it_is() {
        assert_eq!(markdown("a +x -- y+ b"), "a x -- y b");
        assert_eq!(markdown("a +x ## y+ b"), "a x ## y b");
        assert_eq!(markdown("a +*not bold*+ b"), "a \\*not bold\\* b");
        assert_eq!(
            markdown("code `+a -- b+` and `+x##y+`"),
            "code `a -- b` and `x##y`"
        );
        assert_eq!(markdown("1+2+3 and a + b + c"), "1+2+3 and a + b + c");
        assert_eq!(markdown("C++ and i++ and a + b"), "C++ and i++ and a + b");
    }

    #[test]
    fn passthroughs_that_the_converter_does_not_read_are_refused() {
        for text in [
            "a ++x++ b",
            "a +++x+++ b",
            "a +x\ny+ b",
            "a +x+y+ b",
            "a +x +y+ b",
            "code `+a+ and b`",
            "code `a +b+`",
        ] {
            assert_eq!(convert(text), Err(Unsupported::Passthrough), "{text:?}");
        }
    }

    #[test]
    fn unconstrained_role_spans_may_start_and_continue_with_any_character() {
        assert_eq!(markdown("[eq]##¬##s"), "¬s");
        assert_eq!(markdown("[eq]##{times}##s"), "×s");
        assert_eq!(markdown("[eq]##a¬b##c"), "a¬bc");
        assert_eq!(markdown("[eq]##Δ##t and [eq]##全角##"), "Δt and 全角");
        assert_eq!(markdown("[eq]##¬x## and [eq]#¬y#"), "¬x and ¬y");
    }

    fn non_ascii_markup() -> impl Strategy<Value = (String, Vec<String>)> {
        let text = proptest::sample::select(vec!["¬", "×", "Δ", "—", "全角", "¬x", "x¬", "aΔb"]);
        let markup = proptest::sample::select(vec![
            "{}",
            "*{}*",
            "**{}**",
            "_{}_",
            "`{}`",
            "[eq]#{}#",
            "[eq]##{}##",
            "[eq]##{}##s",
            "+{}+",
            "`+{}+`",
            "x^{}^",
            "H~{}~",
            "https://a.b/c[{}]",
            "<<a,{}>>",
            "pname:a{}",
            "\"`{}`\"",
        ]);
        proptest::collection::vec((markup, text), 1..6).prop_map(|pieces| {
            let words = pieces
                .iter()
                .map(|(markup, text)| markup.replace("{}", text))
                .collect::<Vec<_>>();
            let texts = pieces.iter().map(|(_, text)| text.to_string()).collect();
            (words.join(" "), texts)
        })
    }

    fn macro_tail() -> impl Strategy<Value = String> {
        let piece = prop_oneof![
            6 => "[abXYZ0-9_]{1,3}",
            2 => Just(".".to_string()),
            2 => Just("->".to_string()),
            2 => Just("*".to_string()),
            2 => Just("-".to_string()),
            1 => Just("--".to_string()),
            1 => proptest::sample::select(vec![":", " ", ">", "é", "..", ","])
                .prop_map(str::to_string),
        ];
        proptest::collection::vec(piece, 0..8).prop_map(|pieces| pieces.concat())
    }

    fn text_with_macros() -> impl Strategy<Value = String> {
        let names = SPEC_MACROS
            .iter()
            .map(|(name, _, _)| *name)
            .collect::<Vec<_>>();
        let words = vec!["a", "Bc", "x1", "A_", "a_b", "re", "VK_X"];
        let arguments = vec!["a.b", "p->q.r", "V*x", "a-b", "x_1", "a--b", "a->"];
        let marks = vec![".", "->", "-", "--", "*", ":", " ", ", ", "(", ")", "..."];
        let piece = prop_oneof![
            4 => proptest::sample::select(names.clone()).prop_map(|name| format!("{name}:")),
            2 => (
                proptest::sample::select(names.clone()),
                proptest::sample::select(arguments),
            )
                .prop_map(|(name, argument)| format!("{name}:{argument}")),
            1 => proptest::sample::select(names).prop_map(str::to_string),
            3 => proptest::sample::select(words).prop_map(str::to_string),
            3 => proptest::sample::select(marks).prop_map(str::to_string),
        ];
        proptest::collection::vec(piece, 1..10)
            .prop_map(|pieces| pieces.concat())
            .prop_filter("stars that could mark bold text", |text| {
                text.split('*').count() <= 2 || text.contains("**") && text.split('*').count() == 3
            })
    }

    #[test]
    fn the_generated_texts_have_macros_in_every_position() {
        let texts = crate::test_support::deterministic_values(text_with_macros(), 1024);
        let mut reached = BTreeMap::<&str, usize>::new();
        for text in &texts {
            let in_order = pieces_in_the_order(text, &mut (0..patterns().len()));
            let reversed = pieces_in_the_order(text, &mut (0..patterns().len()).rev());
            let is_rendered = |piece: &Piece| matches!(piece, Piece::Rendered { .. });
            let is_refused = |piece: &Piece| matches!(piece, Piece::Refused(_));
            let source = after_the_replacements(text);
            let any_text = |holds: &dyn Fn(&str) -> bool| {
                in_order
                    .iter()
                    .any(|piece| matches!(piece, Piece::Text(text) if holds(text)))
            };
            let any_rendered = |holds: &dyn Fn(&str) -> bool| {
                in_order.iter().any(
                    |piece| matches!(piece, Piece::Rendered { markdown, .. } if holds(markdown)),
                )
            };
            let any_refused = |holds: &dyn Fn(&str) -> bool| {
                in_order
                    .iter()
                    .any(|piece| matches!(piece, Piece::Refused(source) if holds(source)))
            };
            let is_normative = |source: &str| {
                SPEC_MACROS.iter().any(|(name, argument, _)| {
                    *argument == Argument::OptionalWord
                        && source
                            .strip_prefix(name)
                            .is_some_and(|rest| rest.starts_with(':'))
                })
            };
            for (case, holds) in [
                ("a macro", any_rendered(&|_| true)),
                (
                    "two macros",
                    in_order.iter().filter(|piece| is_rendered(piece)).count() >= 2,
                ),
                ("no refused use", !in_order.iter().any(is_refused)),
                (
                    "a refused word after a normative macro",
                    any_refused(&is_normative),
                ),
                (
                    "a refused macro in a word",
                    any_refused(&|source| !is_normative(source)),
                ),
                ("macros whose order matters", in_order != reversed),
                (
                    "a member path",
                    any_rendered(&|code| code.contains("->") || code.contains('.')),
                ),
                (
                    "a star in an argument",
                    any_rendered(&|code| code.contains('*')),
                ),
                (
                    "a star outside a macro",
                    any_text(&|text| text.contains('*')),
                ),
                ("a dash", source.contains(DASH)),
                (
                    "an arrow outside a macro",
                    any_text(&|text| text.contains(ARROW)),
                ),
            ] {
                *reached.entry(case).or_default() += usize::from(holds);
            }
        }
        for (case, at_least) in [
            ("a macro", 400),
            ("two macros", 100),
            ("no refused use", 400),
            ("a refused word after a normative macro", 200),
            ("a refused macro in a word", 150),
            ("macros whose order matters", 250),
            ("a member path", 5),
            ("a star in an argument", 5),
            ("a star outside a macro", 100),
            ("a dash", 80),
            ("an arrow outside a macro", 150),
        ] {
            let texts = reached.get(case).copied().unwrap_or(0);
            assert!(
                texts >= at_least,
                "only {texts} of 1024 generated texts have {case}: {reached:?}"
            );
        }
    }

    #[test]
    fn a_macro_without_an_argument_stays_text() {
        let macros = SPEC_MACROS.iter().filter(|(_, argument, _)| {
            !matches!(argument, Argument::Nothing | Argument::OptionalWord)
        });
        for (name, _, _) in macros {
            for text in [
                format!("{name}: x"),
                format!("a {name}:"),
                format!("({name}:)"),
            ] {
                assert_eq!(markdown(&text), text);
            }
        }
    }

    #[test]
    fn attribute_names_have_letters_digits_underscores_and_hyphens() {
        for name in ["max-size", "max_size", "a1", "-", "_", "0"] {
            assert_eq!(
                convert(&format!("a {{{name}}} b")),
                Err(Unsupported::Attribute(name.to_string()))
            );
        }
        for text in [
            "a {} b",
            "a {x y} b",
            "a {x.y} b",
            "a {x b",
            "a {x",
            "a {",
            "{}",
            "}{",
        ] {
            assert_eq!(markdown(text), text);
        }
        assert_eq!(markdown("{times}{times} {{times}}"), "×× {×}");
    }

    #[test]
    fn only_a_plus_between_a_space_and_the_end_of_a_line_is_a_line_break() {
        assert_eq!(markdown("first +\nsecond"), "first \nsecond");
        assert_eq!(markdown("a b\nc x\nd"), "a b\nc x\nd");
        assert_eq!(markdown("a+\nb and a +b\nc"), "a+\nb and a +b\nc");
    }

    #[test]
    fn subscripts_and_superscripts_have_text_without_spaces() {
        assert_eq!(markdown("a~i~ and b^2^"), "a<sub>i</sub> and b<sup>2</sup>");
        assert_eq!(markdown("a ~~ b ^^ c"), "a \\~\\~ b ^^ c");
        assert_eq!(markdown("x~ y~ and x^ y^"), "x\\~ y\\~ and x^ y^");
    }

    #[test]
    fn a_mark_closes_after_text_and_a_single_mark_only_before_the_end_of_a_word() {
        assert_eq!(markdown("*a *b* c"), "**a \\*b** c");
        assert_eq!(markdown("*a * b* c"), "**a \\* b** c");
        assert_eq!(markdown("*a*b c* d"), "**a\\*b c** d");
        assert_eq!(markdown("**a **b** c"), "**a \\*\\*b** c");
        assert_eq!(markdown("**a**b"), "**a**b");
        assert_eq!(markdown("* a* and ** a**"), "\\* a\\* and \\*\\* a\\*\\*");
    }

    #[test]
    fn a_plus_in_code_is_refused_only_where_it_opens_a_passthrough() {
        assert_eq!(
            markdown("`a+b` and `i++` and `a + b` and `+ a`"),
            "`a+b` and `i++` and `a + b` and `+ a`"
        );
        for text in ["`+x\ny+`", "`a +x\ny+`", "`++x++`", "`a +x+ b`"] {
            assert_eq!(convert(text), Err(Unsupported::Passthrough), "{text:?}");
        }
    }

    #[test]
    fn a_passthrough_in_code_is_refused_also_where_a_macro_would_take_its_text() {
        for text in ["`https://a.b/+x\ny+`", "`latexmath:[+x\ny+]`"] {
            assert_eq!(convert(text), Err(Unsupported::Passthrough), "{text:?}");
        }
    }

    #[test]
    fn a_plus_that_no_later_plus_can_close_stays_text() {
        assert_eq!(markdown("a +x + y"), "a +x + y");
        assert_eq!(markdown("a +x+y z"), "a +x+y z");
        assert_eq!(markdown("a +x y"), "a +x y");
        assert_eq!(markdown("a + x+ y"), "a + x+ y");
    }

    #[test]
    fn math_is_not_searched_for_attributes() {
        assert_eq!(markdown("x latexmath:[a{b}] y"), "x `a{b}` y");
        assert_eq!(markdown("latexmath:[a[i] + {b}] y"), "`a[i] + {b}` y");
        assert_eq!(
            markdown("{times} latexmath:[{times}] {times} latexmath:[a_{i}] {times}"),
            "× `{times}` × `a_{i}` ×"
        );
        assert_eq!(
            convert("latexmath:[{a} b"),
            Err(Unsupported::Unterminated("latexmath"))
        );
        assert_eq!(
            convert("a latexmath:[b"),
            Err(Unsupported::Unterminated("latexmath"))
        );
    }

    proptest! {
        #![proptest_config(crate::test_support::proptest_config(1024))]

        #[test]
        fn a_macro_takes_what_its_pattern_in_the_specification_matches_in_a_longer_text(
            index in 0..SPEC_MACROS.len(),
            tail in macro_tail(),
        ) {
            with_every_link_resolved(|converter| {
                assert_read_as_the_pattern_matches(converter, index, &tail);
            });
        }

        #[test]
        fn macros_are_rendered_as_the_passes_of_the_specification_render_them_or_refused(
            text in text_with_macros(),
        ) {
            let converted = with_every_link_resolved(|converter| converter.inline(&text));
            match as_the_converter_renders(&text) {
                Some(rendered) => prop_assert_eq!(converted, Ok(rendered), "{:?}", text),
                None => prop_assert!(
                    matches!(converted, Err(Unsupported::MacroUse(_))),
                    "{:?} has a word after a normative macro or a macro in a word, and is {:?}",
                    text, converted
                ),
            }
        }

        #[test]
        fn a_url_without_a_label_is_linked_without_the_punctuation_after_it(
            (scheme, address) in proptest::sample::select(
                &[("https:", "https:"), ("http:", "http:"), ("link:https:", "https:")][..]
            ),
            path in "[a-z0-9/_#-]{1,6}",
            punctuation in "[.,;)]{0,3}",
            rest in proptest::sample::select(&["", " d", "\nd"][..]),
        ) {
            let source = format!("at {scheme}//a.b/{path}{punctuation}{rest}");
            prop_assert_eq!(
                convert(&source),
                Ok(format!("at <{address}//a.b/{path}>{punctuation}{rest}")),
                "{:?}", source
            );
        }

        #[test]
        fn a_role_span_keeps_only_its_text_whatever_the_role_is(
            role in proptest::sample::select(&["eq", "a", "small", ".big", "x-y_z"][..]),
            marks in proptest::sample::select(&["#", "##"][..]),
            text in "[a-z]{1,3}( [a-z]{1,3}){0,1}",
            after in proptest::sample::select(&["", " b", ". b"][..]),
        ) {
            let source = format!("a [{role}]{marks}{text}{marks}{after}");
            prop_assert_eq!(convert(&source), Ok(format!("a {text}{after}")), "{:?}", source);
        }

        #[test]
        fn markup_around_non_ascii_text_keeps_the_text(
            (source, texts) in non_ascii_markup()
        ) {
            let converted = crate::test_support::catch_quietly(|| convert(&source));
            let converted = converted.unwrap_or_else(|panic| panic!("`{source}` panics: {panic}"));
            let markdown = converted
                .unwrap_or_else(|unsupported| panic!("`{source}` is refused: {unsupported:?}"));
            let alphabet = |text: &str| {
                text.chars().filter(|c| "¬×Δ—全角".contains(*c)).collect::<String>()
            };
            prop_assert_eq!(
                alphabet(&markdown),
                texts.iter().map(|text| alphabet(text)).collect::<String>(),
                "`{}` became `{}`", source, markdown
            );
            prop_assert!(
                !markdown.contains("eq") && !markdown.contains("##"),
                "`{}` keeps a role span: `{}`", source, markdown
            );
        }
    }

    #[test]
    fn double_hyphens_and_hashes_in_urls_stay_as_they_are() {
        assert_eq!(
            markdown("see https://a.b/c--d#e[the -- page] or https://a.b/x--y##z."),
            "see [the — page](https://a.b/c--d#e) or <https://a.b/x--y##z>."
        );
    }

    #[test]
    fn double_hyphens_become_em_dashes_between_words_and_spaces() {
        assert_eq!(markdown("a -- b"), "a — b");
        assert_eq!(
            markdown("the parameters --\nno substitution"),
            "the parameters —\nno substitution"
        );
        assert_eq!(markdown("-- at the start"), "— at the start");
        assert_eq!(markdown("at the end --"), "at the end —");
        assert_eq!(markdown("word--word"), "word—word");
        assert_eq!(markdown("a --b, a-- b and ---"), "a --b, a-- b and ---");
        assert_eq!(markdown("an escaped \\-- dash"), "an escaped -- dash");
    }

    #[test]
    fn double_hyphens_in_code_stay_as_they_are() {
        assert_eq!(
            markdown("run `a -- b` and `x--y`"),
            "run `a -- b` and `x--y`"
        );
        assert_eq!(markdown("pname:x -- y"), "`x` — y");
        assert_eq!(
            markdown("latexmath:[a -- b] and `pname:x -- y`"),
            "`a -- b` and `x` — y"
        );
    }

    #[test]
    fn unknown_macros_with_brackets_are_refused() {
        for (text, name) in [
            ("press kbd:[Ctrl]", "kbd"),
            ("menu:File[Save] now", "menu"),
            ("btn:[OK]", "btn"),
            ("mailto:a@b.c[mail]", "mailto"),
            ("indexterm:[x]", "indexterm"),
        ] {
            assert_eq!(
                convert(text),
                Err(Unsupported::UnknownMacro(name.to_string())),
                "{text}"
            );
        }
        assert_eq!(
            markdown("pname:pData[i] and note: [x]"),
            "`pData`\\[i\\] and note: \\[x\\]"
        );
        assert_eq!(markdown("a:b [c]"), "a:b \\[c\\]");
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
