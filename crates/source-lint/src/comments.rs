use std::ops::Range;

use ra_ap_rustc_lexer::{
    DocStyle, FrontmatterAllowed, LiteralKind, TokenKind, strip_shebang, tokenize,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommentKind {
    Line,
    Block,
    OuterDoc,
    InnerDoc,
    DocAttribute,
}

impl CommentKind {
    pub fn describe(self) -> &'static str {
        match self {
            CommentKind::Line => "line comment",
            CommentKind::Block => "block comment",
            CommentKind::OuterDoc => "outer doc comment",
            CommentKind::InnerDoc => "inner doc comment",
            CommentKind::DocAttribute => "doc attribute",
        }
    }

    pub fn is_doc(self) -> bool {
        !matches!(self, CommentKind::Line | CommentKind::Block)
    }

    fn of_comment_token(doc_style: Option<DocStyle>, plain: CommentKind) -> CommentKind {
        match doc_style {
            None => plain,
            Some(DocStyle::Outer) => CommentKind::OuterDoc,
            Some(DocStyle::Inner) => CommentKind::InnerDoc,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Comment {
    pub offset: usize,
    pub line: usize,
    pub kind: CommentKind,
}

const KEYWORDS: &[&str] = &[
    "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
    "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if",
    "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub",
    "ref", "return", "self", "static", "struct", "super", "trait", "true", "try", "type", "typeof",
    "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

const BINDING_OR_FIELD_PREFIXES: &[&str] = &["let", "mut", "ref", "."];

struct CodeToken<'a> {
    kind: TokenKind,
    text: &'a str,
    offset: usize,
    line: usize,
}

impl CodeToken<'_> {
    fn is_named(&self, name: &str) -> bool {
        match self.kind {
            TokenKind::Ident => self.text == name,
            TokenKind::RawIdent => self.text.strip_prefix("r#") == Some(name),
            _ => false,
        }
    }

    fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Ident | TokenKind::RawIdent)
    }

    fn can_name_a_macro(&self) -> bool {
        match self.kind {
            TokenKind::Ident => !KEYWORDS.contains(&self.text),
            TokenKind::RawIdent => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Literal {
                kind: LiteralKind::Str { .. } | LiteralKind::RawStr { .. },
                ..
            }
        )
    }
}

pub fn find_comments(source: &str) -> Vec<Comment> {
    let shebang_len = strip_shebang(source).unwrap_or(0);
    let mut line = 1;
    let mut offset = shebang_len;
    let mut comments = Vec::new();
    let mut code = Vec::new();
    for token in tokenize(&source[shebang_len..], FrontmatterAllowed::No) {
        let text = &source[offset..offset + token.len as usize];
        match token.kind {
            TokenKind::LineComment { doc_style } => comments.push(Comment {
                offset,
                line,
                kind: CommentKind::of_comment_token(doc_style, CommentKind::Line),
            }),
            TokenKind::BlockComment { doc_style, .. } => comments.push(Comment {
                offset,
                line,
                kind: CommentKind::of_comment_token(doc_style, CommentKind::Block),
            }),
            TokenKind::Whitespace => {}
            kind => code.push(CodeToken {
                kind,
                text,
                offset,
                line,
            }),
        }
        line += text.matches('\n').count();
        offset += text.len();
    }
    comments.extend(doc_attributes(&code));
    comments.sort_by_key(|comment| comment.offset);
    comments
}

fn doc_attributes(code: &[CodeToken<'_>]) -> Vec<Comment> {
    let mut found = Vec::new();
    let mut found_attributes: Vec<Range<usize>> = Vec::new();
    for (hash_index, hash) in code.iter().enumerate() {
        if hash.text != "#" {
            continue;
        }
        let mut open_index = hash_index + 1;
        if code.get(open_index).is_some_and(|token| token.text == "!") {
            open_index += 1;
        }
        if code.get(open_index).is_none_or(|token| token.text != "[") {
            continue;
        }
        let Some(close_index) = matching_close(code, open_index) else {
            continue;
        };
        if assigns_doc(&code[open_index + 1..close_index]) {
            found.push(Comment {
                offset: hash.offset,
                line: hash.line,
                kind: CommentKind::DocAttribute,
            });
            found_attributes.push(open_index..close_index);
        }
    }
    let in_macro_arguments = macro_arguments(code);
    for (index, name) in code.iter().enumerate() {
        let binds_a_variable_or_field = index
            .checked_sub(1)
            .is_some_and(|before| BINDING_OR_FIELD_PREFIXES.contains(&code[before].text));
        let assigns_documentation = name.is_named("doc")
            && !binds_a_variable_or_field
            && code.get(index + 1).is_some_and(|equals| equals.text == "=")
            && starts_documentation(code, index + 2);
        if assigns_documentation
            && in_macro_arguments[index]
            && !found_attributes
                .iter()
                .any(|attribute| attribute.contains(&index))
        {
            found.push(Comment {
                offset: name.offset,
                line: name.line,
                kind: CommentKind::DocAttribute,
            });
        }
    }
    found
}

fn starts_documentation(code: &[CodeToken<'_>], start: usize) -> bool {
    let Some(first) = code.get(start) else {
        return false;
    };
    if first.is_string() {
        return true;
    }
    if first.text == "$" {
        return code.get(start + 1).is_some_and(CodeToken::is_identifier);
    }
    let mut index = start;
    if is_path_separator(code, index) {
        index += 2;
    }
    loop {
        if !code.get(index).is_some_and(CodeToken::is_identifier) {
            return false;
        }
        index += 1;
        if !is_path_separator(code, index) {
            break;
        }
        index += 2;
    }
    opens_macro_arguments(code, index)
}

fn is_path_separator(code: &[CodeToken<'_>], index: usize) -> bool {
    code.get(index).is_some_and(|first| first.text == ":")
        && code.get(index + 1).is_some_and(|second| second.text == ":")
}

fn opens_macro_arguments(code: &[CodeToken<'_>], bang_index: usize) -> bool {
    code.get(bang_index).is_some_and(|bang| bang.text == "!")
        && code
            .get(bang_index + 1)
            .is_some_and(|open| matches!(open.text, "(" | "[" | "{"))
}

fn macro_arguments(code: &[CodeToken<'_>]) -> Vec<bool> {
    let mut inside = vec![false; code.len()];
    for bang_index in 0..code.len() {
        let invokes_a_macro = opens_macro_arguments(code, bang_index)
            && bang_index
                .checked_sub(1)
                .is_some_and(|name_index| code[name_index].can_name_a_macro());
        if !invokes_a_macro {
            continue;
        }
        let open_index = bang_index + 1;
        let close_index = matching_close(code, open_index).unwrap_or(code.len());
        inside[open_index + 1..close_index].fill(true);
    }
    inside
}

fn matching_close(code: &[CodeToken<'_>], open_index: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, token) in code.iter().enumerate().skip(open_index) {
        match token.text {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn assigns_doc(attribute: &[CodeToken<'_>]) -> bool {
    match attribute {
        [name, equals, ..] if name.is_named("doc") && equals.text == "=" => true,
        [name, open, arguments @ .., close]
            if name.is_named("cfg_attr") && open.text == "(" && close.text == ")" =>
        {
            split_top_level_commas(arguments)
                .into_iter()
                .skip(1)
                .any(assigns_doc)
        }
        _ => false,
    }
}

fn split_top_level_commas<'t, 'a>(tokens: &'t [CodeToken<'a>]) -> Vec<&'t [CodeToken<'a>]> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (index, token) in tokens.iter().enumerate() {
        match token.text {
            "(" | "[" | "{" => depth += 1,
            ")" | "]" | "}" => depth = depth.saturating_sub(1),
            "," if depth == 0 => {
                parts.push(&tokens[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    parts.push(&tokens[start..]);
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines_and_kinds(source: &str) -> Vec<(usize, CommentKind)> {
        find_comments(source)
            .into_iter()
            .map(|comment| (comment.line, comment.kind))
            .collect()
    }

    fn assert_no_comments(source: &str) {
        assert_eq!(lines_and_kinds(source), [], "{source}");
    }

    #[test]
    fn finds_every_comment_form() {
        let source = concat!(
            "// line\n",
            "/* block */\n",
            "/// outer doc\n",
            "//! inner doc\n",
            "/** outer block doc */\n",
            "/*! inner block doc */\n",
            "//// four slashes\n",
            "/**/\n",
            "/***/\n",
            "fn main() {}\n",
        );
        assert_eq!(
            lines_and_kinds(source),
            [
                (1, CommentKind::Line),
                (2, CommentKind::Block),
                (3, CommentKind::OuterDoc),
                (4, CommentKind::InnerDoc),
                (5, CommentKind::OuterDoc),
                (6, CommentKind::InnerDoc),
                (7, CommentKind::Line),
                (8, CommentKind::Block),
                (9, CommentKind::Block),
            ]
        );
    }

    #[test]
    fn finds_trailing_and_inline_comments() {
        let source = "let a = 1; // trailing\nlet b = /* inline */ 2;\nlet c = 3;";
        assert_eq!(
            lines_and_kinds(source),
            [(1, CommentKind::Line), (2, CommentKind::Block)]
        );
    }

    #[test]
    fn nested_block_comment_is_one_comment() {
        let source = "/* outer /* inner */ still outer */ let s = \"*/ //\";\nlet t = 1; // after";
        assert_eq!(
            lines_and_kinds(source),
            [(1, CommentKind::Block), (2, CommentKind::Line)]
        );
    }

    #[test]
    fn unterminated_block_comment_is_a_comment() {
        assert_eq!(
            lines_and_kinds("fn main() {}\n/* never closed\n"),
            [(2, CommentKind::Block)]
        );
    }

    #[test]
    fn multi_line_block_comment_is_reported_at_its_first_line() {
        let source = "let a = 1;\n/*\n\n\n*/\nlet b = 2; // here";
        assert_eq!(
            lines_and_kinds(source),
            [(2, CommentKind::Block), (6, CommentKind::Line)]
        );
    }

    #[test]
    fn string_literals_are_not_comments() {
        assert_no_comments(r#"let s = "// not a comment /* nor this */";"#);
        assert_no_comments(r#"let s = "escaped \" // still inside";"#);
        assert_no_comments("let s = \"first line\n// second line\n/* third */\";");
        assert_no_comments(r#"let s = "\\"; let t = "//";"#);
    }

    #[test]
    fn raw_string_literals_are_not_comments() {
        assert_no_comments(r##"let s = r"// not a comment";"##);
        assert_no_comments(r###"let s = r#"a "quote" // and /* this */"#;"###);
        assert_no_comments(r###"let s = r##"ends with "# // still inside"##;"###);
    }

    #[test]
    fn byte_and_c_string_literals_are_not_comments() {
        assert_no_comments(r#"let s = b"// not a comment";"#);
        assert_no_comments(r##"let s = br#"/* not a comment */"#;"##);
        assert_no_comments(r#"let s = c"// not a comment";"#);
        assert_no_comments(r##"let s = cr#"/* not a comment */"#;"##);
    }

    #[test]
    fn character_literals_are_not_comments() {
        assert_no_comments("let slash = '/'; let star = '*'; let pair = ['/', '/'];");
        assert_no_comments("let quote = '\"'; let s = \"//\";");
        assert_no_comments("let quote = '\\''; let s = \"/*\";");
        assert_no_comments("let slash = b'/'; let star = b'*';");
        assert_eq!(
            lines_and_kinds("let quote = '\"'; // real"),
            [(1, CommentKind::Line)]
        );
    }

    #[test]
    fn lifetimes_are_not_character_literals() {
        assert_no_comments("fn f<'a, 'b>(x: &'a str, y: &'b str) -> char { '/' }");
        assert_no_comments("struct S<'a>(&'a str); let s = \"/*\"; let t = '*';");
        assert_eq!(
            lines_and_kinds("impl<'a> S<'a> {} let c = 'x'; // real"),
            [(1, CommentKind::Line)]
        );
    }

    #[test]
    fn division_is_not_a_comment() {
        assert_no_comments("let x = a / b / c; let y = a /b; let z = (a)/(b);");
        assert_no_comments("let x = a /= 2; let p = *a / *b;");
    }

    #[test]
    fn shebang_is_not_a_comment() {
        assert_no_comments("#!/usr/bin/env run-cargo-script\nfn main() {}\n");
        assert_eq!(
            lines_and_kinds("#!/usr/bin/env run-cargo-script\n// real\n"),
            [(2, CommentKind::Line)]
        );
    }

    #[test]
    fn doc_attributes_are_comments() {
        let source = concat!(
            "#[doc = \"outer\"]\n",
            "#![doc = include_str!(\"../README.md\")]\n",
            "# [ doc = \"spaced\" ]\n",
            "#[cfg_attr(docsrs, doc = \"conditional\")]\n",
            "#[cfg_attr(a, cfg_attr(b, inline, doc = \"nested\"))]\n",
            "macro_rules! m { ($text:expr) => { #[doc = $text] struct S; }; }\n",
        );
        assert_eq!(
            lines_and_kinds(source),
            (1..=6)
                .map(|line| (line, CommentKind::DocAttribute))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn list_form_doc_attributes_are_not_comments() {
        assert_no_comments("#[doc(hidden)] pub fn f() {}");
        assert_no_comments("#[doc(alias = \"vkCreateInstance\")] pub fn f() {}");
        assert_no_comments("#[cfg_attr(docsrs, doc(cfg(feature = \"std\")))] pub fn f() {}");
        assert_no_comments("#![cfg_attr(doc, allow(unused))]");
    }

    #[test]
    fn other_uses_of_doc_are_not_comments() {
        assert_no_comments("let doc = 1; struct Doc; fn doc() {}");
        assert_no_comments("#[derive(Debug)] struct S { doc: u32 }");
        assert_no_comments("#[serde(rename = \"doc\")] struct S;");
        assert_no_comments("let s = \"#[doc = \\\"x\\\"]\";");
        assert_no_comments("let s = r#\"#[doc = \"x\"]\"#;");
        assert_no_comments("let v = arr[doc = 1];");
    }

    const DOC_NAMES: [&str; 2] = ["doc", "r#doc"];
    const CFG_ATTR_NAMES: [&str; 2] = ["cfg_attr", "r#cfg_attr"];
    const STRING_VALUES: [&str; 5] = [
        "\"x\"",
        "r\"x\"",
        "r#\"a \"quoted\" x\"#",
        "\"first\nsecond\"",
        "\"escaped \\\" doc = \\\"x\\\"\"",
    ];
    const MACRO_VALUES: [&str; 5] = [
        "$t",
        "$crate_text",
        "concat!(\"x\")",
        "include_str!(\"x.md\")",
        "::core::concat![\"x\"]",
    ];
    const GAPS: [&str; 4] = ["", " ", "\n", " \n\t "];
    const VALUES_THAT_ARE_NOT_DOCUMENTATION: [&str; 9] = [
        "doc.render()",
        "value",
        "a::b",
        "f(x)",
        "1",
        "$",
        "x!",
        "a::b!",
        "S { doc: \"x\" }",
    ];
    const KEYWORDS_BEFORE_EXPRESSIONS: [&str; 9] = [
        "if", "while", "return", "match", "in", "else", "break", "yield", "move",
    ];

    #[derive(Clone, Copy, Debug)]
    enum DocContext {
        Attribute(&'static str),
        MacroArgument(&'static str),
    }

    const DOC_CONTEXTS: [DocContext; 10] = [
        DocContext::Attribute("#[~META~]\npub fn f() {}\n"),
        DocContext::Attribute("#![~META~]\n"),
        DocContext::Attribute("m! {~#[~META~]~struct S;~}\n"),
        DocContext::MacroArgument("with!~(~META~);\n"),
        DocContext::MacroArgument("with![~META~];\n"),
        DocContext::MacroArgument("with!~{~META~}\n"),
        DocContext::MacroArgument("crate::m::with!(~META~);\n"),
        DocContext::MacroArgument("r#with!{~META~}\n"),
        DocContext::MacroArgument("outer! {~#[derive(Debug)]~inner!(~META~)~}\n"),
        DocContext::MacroArgument("fn f() {~with!(~META~);~}\n"),
    ];

    #[derive(Default)]
    struct GeneratedSource {
        text: String,
        doc_names: Vec<usize>,
    }

    impl GeneratedSource {
        fn push(&mut self, text: &str) {
            self.text.push_str(text);
        }

        fn push_doc(&mut self, name: &str, gap: &str, value: &str) {
            self.doc_names.push(self.text.len());
            self.text.push_str(&format!("{name}{gap}={gap}{value}"));
        }

        fn line_of(&self, offset: usize) -> usize {
            1 + self.text[..offset].matches('\n').count()
        }
    }

    struct DocCase {
        name: &'static str,
        value: &'static str,
        gap: &'static str,
        cfg_attr_depth: usize,
        both_spellings: bool,
        context: DocContext,
    }

    impl DocCase {
        fn push_meta(&self, source: &mut GeneratedSource) {
            let gap = self.gap;
            for level in 0..self.cfg_attr_depth {
                source.push(CFG_ATTR_NAMES[level % 2]);
                source.push(&format!("({gap}all(),{gap}inline,{gap}"));
            }
            source.push_doc(self.name, gap, self.value);
            if self.both_spellings {
                let other = DOC_NAMES.iter().find(|name| **name != self.name).unwrap();
                source.push(&format!(",{gap}"));
                source.push_doc(other, gap, self.value);
            }
            for _ in 0..self.cfg_attr_depth {
                source.push(&format!("{gap})"));
            }
        }

        fn source(&self) -> (GeneratedSource, Vec<usize>) {
            let template = match self.context {
                DocContext::Attribute(template) | DocContext::MacroArgument(template) => template,
            };
            let (before, after) = template.split_once("META").unwrap();
            let mut source = GeneratedSource::default();
            source.push("fn before() {}\n");
            let start = source.text.len();
            source.push(&before.replace('~', self.gap));
            let hash = start + source.text[start..].find('#').unwrap_or(0);
            self.push_meta(&mut source);
            source.push(&after.replace('~', self.gap));
            let expected = match self.context {
                DocContext::Attribute(_) => vec![hash],
                DocContext::MacroArgument(_) => source.doc_names.clone(),
            };
            (source, expected)
        }
    }

    fn doc_cases() -> Vec<DocCase> {
        let mut cases = Vec::new();
        for name in DOC_NAMES {
            for value in STRING_VALUES.into_iter().chain(MACRO_VALUES) {
                for gap in GAPS {
                    for cfg_attr_depth in 0..=2 {
                        for both_spellings in [false, true] {
                            for context in DOC_CONTEXTS {
                                let attribute = matches!(context, DocContext::Attribute(_));
                                if both_spellings && attribute && cfg_attr_depth == 0 {
                                    continue;
                                }
                                cases.push(DocCase {
                                    name,
                                    value,
                                    gap,
                                    cfg_attr_depth,
                                    both_spellings,
                                    context,
                                });
                            }
                        }
                    }
                }
            }
        }
        cases
    }

    #[test]
    fn every_spelling_of_a_doc_attribute_is_found_where_it_starts() {
        let cases = doc_cases();
        assert!(
            cases.len() > 1000,
            "only {} cases were generated",
            cases.len()
        );
        for case in cases {
            let (source, expected_offsets) = case.source();
            let expected: Vec<Comment> = expected_offsets
                .into_iter()
                .map(|offset| Comment {
                    offset,
                    line: source.line_of(offset),
                    kind: CommentKind::DocAttribute,
                })
                .collect();
            assert_eq!(
                find_comments(&source.text),
                expected,
                "the doc attributes of {:?} were not found where they start",
                source.text
            );
        }
    }

    #[test]
    fn names_that_only_resemble_doc_are_not_doc_attributes_in_any_context() {
        let names = ["docs", "r#docs", "undoc", "doc_", "Doc", "DOC", "r#Doc"];
        let mut forms = Vec::new();
        for gap in GAPS {
            for value in STRING_VALUES {
                for name in names {
                    forms.push(format!("{name}{gap}={gap}{value}"));
                }
                for name in DOC_NAMES {
                    forms.push(format!("#[{name}{gap}(hidden)]"));
                    forms.push(format!("#[{name}({gap}alias{gap}={gap}{value})]"));
                    forms.push(format!("#[cfg_attr(all(), {name}{gap}(alias = {value}))]"));
                    forms.push(format!("#[serde(rename{gap}={gap}{value})]"));
                    forms.push(format!("{name}{gap}=={gap}{value}"));
                    forms.push(format!("{name}{gap}!={gap}{value}"));
                    forms.push(format!("{name}{gap}:{gap}{value}"));
                    forms.push(format!("{name}{gap}({value})"));
                    forms.push(format!("x.{name}()"));
                    forms.push(format!("let {name}{gap}={gap}1;"));
                    forms.push(format!("{name}{gap}={gap}b{value}"));
                    forms.push(format!("{name}{gap}={gap}c{value}"));
                    forms.push(format!("{name}{gap}={gap}br\"x\""));
                    forms.push(format!("{name}{gap}={gap}'x'"));
                    forms.push(format!("{name}{gap}={gap}{{ {value} }}"));
                    forms.push(format!("\"{name}{gap}={gap}\\\"x\\\"\""));
                    forms.push(format!("r#\"{name}{gap}={gap}\"x\"\"#"));
                    forms.push(format!("let {name}{gap}={gap}{value};"));
                    forms.push(format!("let mut {name}{gap}={gap}{value};"));
                    forms.push(format!("let ref {name}{gap}={gap}{value};"));
                    forms.push(format!("s.{name}{gap}={gap}{value};"));
                    forms.push(format!("self{gap}.{gap}{name}{gap}={gap}{value};"));
                }
            }
            for name in DOC_NAMES {
                for value in VALUES_THAT_ARE_NOT_DOCUMENTATION {
                    forms.push(format!("{name}{gap}={gap}{value}"));
                }
                for value in MACRO_VALUES {
                    forms.push(format!("let {name}{gap}={gap}{value};"));
                    forms.push(format!("let mut {name}{gap}={gap}{value};"));
                    forms.push(format!("x.{name}{gap}={gap}{value};"));
                }
            }
        }
        for form in forms {
            for context in [
                format!("{form}\n"),
                format!("m!({form});\n"),
                format!("m![{form}];\n"),
                format!("m! {{ {form} }}\n"),
                format!("outer! {{ inner!({form}) }}\n"),
                format!("proptest! {{ #[test] fn t(x in 0..1u32) {{ {form} }} }}\n"),
            ] {
                assert_no_comments(&context);
            }
        }
    }

    #[test]
    fn doc_assignments_outside_attributes_and_macro_arguments_are_not_doc_attributes() {
        for name in DOC_NAMES {
            for gap in GAPS {
                for value in STRING_VALUES {
                    let assignment = format!("{name}{gap}={gap}{value}");
                    assert_no_comments(&format!("fn f() {{ let {assignment}; }}"));
                    assert_no_comments(&format!("fn f() {{ s.{assignment}; }}"));
                    assert_no_comments(&format!(
                        "macro_rules! m {{ () => {{ let {assignment}; }}; }}"
                    ));
                    for keyword in KEYWORDS_BEFORE_EXPRESSIONS {
                        assert_no_comments(&format!(
                            "fn f() {{ {keyword}{gap}!{gap}({gap}{assignment}{gap}) }}"
                        ));
                    }
                }
            }
        }
    }

    #[test]
    fn a_raw_keyword_names_a_macro() {
        for keyword in KEYWORDS_BEFORE_EXPRESSIONS {
            let source = format!("fn f() {{ r#{keyword}!(doc = \"x\") }}");
            let offset = source.find("doc").unwrap();
            assert_eq!(
                find_comments(&source),
                [Comment {
                    offset,
                    line: 1,
                    kind: CommentKind::DocAttribute,
                }],
                "{source}"
            );
        }
    }

    #[test]
    fn source_without_comments_has_none() {
        assert_no_comments("");
        assert_no_comments("fn main() {\n    println!(\"hello\");\n}\n");
    }
}
