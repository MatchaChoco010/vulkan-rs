use ra_ap_rustc_lexer::{DocStyle, FrontmatterAllowed, TokenKind, strip_shebang, tokenize};

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
    pub line: usize,
    pub kind: CommentKind,
}

struct CodeToken<'a> {
    text: &'a str,
    line: usize,
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
                line,
                kind: CommentKind::of_comment_token(doc_style, CommentKind::Line),
            }),
            TokenKind::BlockComment { doc_style, .. } => comments.push(Comment {
                line,
                kind: CommentKind::of_comment_token(doc_style, CommentKind::Block),
            }),
            TokenKind::Whitespace => {}
            _ => code.push(CodeToken { text, line }),
        }
        line += text.matches('\n').count();
        offset += text.len();
    }
    comments.extend(doc_attributes(&code));
    comments.sort_by_key(|comment| comment.line);
    comments
}

fn doc_attributes(code: &[CodeToken<'_>]) -> Vec<Comment> {
    let mut found = Vec::new();
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
                line: hash.line,
                kind: CommentKind::DocAttribute,
            });
        }
    }
    found
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
        [name, equals, ..] if name.text == "doc" && equals.text == "=" => true,
        [name, open, arguments @ .., close]
            if name.text == "cfg_attr" && open.text == "(" && close.text == ")" =>
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

    #[test]
    fn source_without_comments_has_none() {
        assert_no_comments("");
        assert_no_comments("fn main() {\n    println!(\"hello\");\n}\n");
    }
}
