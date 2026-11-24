use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use vulkan_codegen::{GENERATED_SRC, GENERATED_TESTS, generate};
use vulkan_registry::{Docs, Registry};

const REGENERATE: &str = "run ./tools/generate-vulkan and commit the files that it changes";

const RUSTFMT_MAX_WIDTH: usize = 100;

const WORDS_BEFORE_A_PARENTHESIZED_EXPRESSION: &[&str] = &[
    "as", "break", "else", "for", "if", "in", "let", "match", "move", "mut", "ref", "return",
    "while", "yield",
];

#[derive(Copy, Clone, Debug)]
struct Token {
    start: usize,
    end: usize,
    line: usize,
}

struct Canonical<'a> {
    source: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Canonical<'a> {
    fn of(source: &'a str) -> Self {
        let mut canonical = Canonical {
            source,
            tokens: lex(source),
        };
        canonical.drop_blank_lines_next_to_brackets();
        canonical.drop_trailing_commas_and_optional_brackets();
        canonical.sort_imports_and_module_declarations();
        canonical
    }

    fn text(&self, index: usize) -> &'a str {
        let token = self.tokens[index];
        &self.source[token.start..token.end]
    }

    fn texts(&self) -> impl Iterator<Item = &'a str> + '_ {
        (0..self.tokens.len()).map(|index| self.text(index))
    }

    fn is_blank_line(&self, index: usize) -> bool {
        let token = self.tokens[index];
        token.start == token.end
    }

    fn is_word(&self, index: usize) -> bool {
        self.text(index)
            .bytes()
            .next()
            .is_some_and(is_identifier_byte)
    }

    fn retain(&mut self, keep: &[bool]) {
        let mut keep = keep.iter();
        self.tokens
            .retain(|_| *keep.next().expect("one flag for each token"));
    }

    fn drop_blank_lines_next_to_brackets(&mut self) {
        let keep = (0..self.tokens.len())
            .map(|index| {
                !self.is_blank_line(index)
                    || !(matches!(self.text(index - 1), "(" | "[" | "{")
                        || self
                            .tokens
                            .get(index + 1)
                            .is_none_or(|_| matches!(self.text(index + 1), ")" | "]" | "}")))
            })
            .collect::<Vec<_>>();
        self.retain(&keep);
    }

    fn drop_trailing_commas_and_optional_brackets(&mut self) {
        struct Open {
            index: usize,
            before: Option<usize>,
            commas: usize,
            last_comma: Option<usize>,
            semicolons: usize,
        }
        let mut keep = vec![true; self.tokens.len()];
        let mut closes = vec![usize::MAX; self.tokens.len()];
        let mut stack = Vec::<Open>::new();
        let mut previous = None;
        for index in 0..self.tokens.len() {
            if self.is_blank_line(index) {
                continue;
            }
            match self.text(index) {
                "(" | "[" | "{" => stack.push(Open {
                    index,
                    before: previous,
                    commas: 0,
                    last_comma: None,
                    semicolons: 0,
                }),
                ";" => {
                    if let Some(open) = stack.last_mut() {
                        open.semicolons += 1;
                    }
                }
                "," => {
                    if let Some(open) = stack.last_mut() {
                        open.commas += 1;
                        open.last_comma = Some(index);
                    }
                }
                ">" => {
                    if let Some(comma) = previous
                        && self.text(comma) == ","
                    {
                        keep[comma] = false;
                    }
                }
                closer @ (")" | "]" | "}") => {
                    let open = stack.pop().unwrap_or_else(|| {
                        panic!(
                            "line {}: `{closer}` closes nothing",
                            self.tokens[index].line
                        )
                    });
                    closes[open.index] = index;
                    let parentheses = closer == ")" && self.text(open.index) == "(";
                    let follows_a_word = open.before.is_some_and(|before| {
                        self.is_word(before)
                            && !WORDS_BEFORE_A_PARENTHESIZED_EXPRESSION.contains(&self.text(before))
                    });
                    let follows_a_closer = open
                        .before
                        .is_some_and(|before| matches!(self.text(before), ")" | "]" | ">"));
                    if let Some(comma) = open.last_comma
                        && previous == Some(comma)
                    {
                        let one_element_tuple = parentheses
                            && open.commas == 1
                            && !follows_a_closer
                            && open.before.is_none_or(|before| !self.is_word(before));
                        if !one_element_tuple {
                            keep[comma] = false;
                        }
                    }
                    let wraps_only_parentheses = parentheses
                        && self.text(open.index + 1) == "("
                        && previous.is_some_and(|previous| closes[open.index + 1] == previous);
                    let is_the_expression_of_a_closure = closer == "}"
                        && open.semicolons == 0
                        && previous != Some(open.index)
                        && open.before.is_some_and(|before| self.text(before) == "|");
                    if is_the_expression_of_a_closure
                        || wraps_only_parentheses && !follows_a_word && !follows_a_closer
                    {
                        keep[open.index] = false;
                        keep[index] = false;
                    }
                }
                _ => {}
            }
            previous = Some(index);
        }
        assert!(stack.is_empty(), "a bracket is left open");
        self.retain(&keep);
    }

    fn statement_end(&self, start: usize) -> Option<usize> {
        let text = |index: usize| (index < self.tokens.len()).then(|| self.text(index));
        let keyword = if text(start) == Some("pub") {
            start + 1
        } else {
            start
        };
        match text(keyword)? {
            "mod" => (text(keyword + 2) == Some(";")).then_some(keyword + 3),
            "use" => {
                let end = (keyword..self.tokens.len())
                    .find(|index| self.is_blank_line(*index) || self.text(*index) == ";")?;
                (self.text(end) == ";").then_some(end + 1)
            }
            _ => None,
        }
    }

    fn sort_imports_and_module_declarations(&mut self) {
        let mut index = 0;
        while index < self.tokens.len() {
            let mut statements = Vec::new();
            let mut end = index;
            while let Some(next) = self.statement_end(end) {
                statements.push(end..next);
                end = next;
            }
            if statements.is_empty() {
                index += 1;
                continue;
            }
            for statement in &statements {
                self.sort_braced_list(statement.clone());
            }
            let mut sorted = statements
                .iter()
                .map(|statement| self.tokens[statement.clone()].to_vec())
                .collect::<Vec<_>>();
            sorted.sort_by_key(|tokens| self.key(tokens));
            self.tokens.splice(index..end, sorted.into_iter().flatten());
            index = end;
        }
    }

    fn key(&self, tokens: &[Token]) -> Vec<&'a str> {
        tokens
            .iter()
            .map(|token| &self.source[token.start..token.end])
            .collect()
    }

    fn sort_braced_list(&mut self, statement: std::ops::Range<usize>) {
        let braces = statement
            .clone()
            .filter(|index| matches!(self.text(*index), "{" | "}"))
            .collect::<Vec<_>>();
        let &[open, close] = braces.as_slice() else {
            return;
        };
        let inside = self.tokens[open + 1..close].to_vec();
        let mut entries = inside
            .split(|token| &self.source[token.start..token.end] == ",")
            .map(<[Token]>::to_vec)
            .collect::<Vec<_>>();
        let commas = inside
            .iter()
            .filter(|token| &self.source[token.start..token.end] == ",")
            .copied()
            .collect::<Vec<_>>();
        entries.sort_by_key(|tokens| self.key(tokens));
        let mut commas = commas.into_iter();
        let mut sorted = Vec::with_capacity(inside.len());
        for (position, entry) in entries.into_iter().enumerate() {
            if position > 0 {
                sorted.push(commas.next().expect("a comma between two entries"));
            }
            sorted.extend(entry);
        }
        self.tokens.splice(open + 1..close, sorted);
    }
}

fn is_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte >= 0x80
}

fn lex(source: &str) -> Vec<Token> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut line = 1;
    let mut newlines = 0;
    let mut at = 0;
    while at < bytes.len() {
        let byte = bytes[at];
        if byte == b'\n' {
            line += 1;
            newlines += 1;
            at += 1;
            continue;
        }
        if byte.is_ascii_whitespace() {
            at += 1;
            continue;
        }
        if newlines > 1 && !tokens.is_empty() {
            tokens.push(Token {
                start: at,
                end: at,
                line,
            });
        }
        newlines = 0;
        let end = match byte {
            b'/' if bytes.get(at + 1) == Some(&b'/') => comment_end(source, at),
            b'"' => string_end(bytes, at + 1),
            b'\'' => character_end(source, at),
            _ if is_identifier_byte(byte) => word_end(source, at),
            _ => at + 1,
        };
        tokens.push(Token {
            start: at,
            end,
            line,
        });
        if matches!(bytes[end - 1], b'"' | b'#') {
            line += source[at..end].matches('\n').count();
        }
        at = end;
    }
    tokens
}

fn comment_end(source: &str, start: usize) -> usize {
    let line_end = source[start..]
        .find('\n')
        .map_or(source.len(), |length| start + length);
    start + source[start..line_end].trim_end().len()
}

fn string_end(bytes: &[u8], body: usize) -> usize {
    let mut at = body;
    loop {
        match bytes.get(at) {
            Some(b'\\') => at += 2,
            Some(b'"') => return at + 1,
            Some(_) => at += 1,
            None => panic!("a string literal is not closed"),
        }
    }
}

fn character_end(source: &str, quote: usize) -> usize {
    let bytes = source.as_bytes();
    if bytes.get(quote + 1) == Some(&b'\\') {
        let closing = source[quote + 3..]
            .find('\'')
            .expect("a character literal is not closed");
        return quote + 3 + closing + 1;
    }
    let width = source[quote + 1..].chars().next().map_or(0, char::len_utf8);
    if bytes.get(quote + 1 + width) == Some(&b'\'') {
        quote + 2 + width
    } else {
        quote + 1
    }
}

fn word_end(source: &str, start: usize) -> usize {
    let bytes = source.as_bytes();
    let mut end = start;
    while end < bytes.len() && is_identifier_byte(bytes[end]) {
        end += 1;
    }
    if matches!(&bytes[start..end], b"r" | b"br" | b"cr") {
        let hashes = bytes[end..]
            .iter()
            .take_while(|byte| **byte == b'#')
            .count();
        if bytes.get(end + hashes) == Some(&b'"') {
            let terminator = format!("\"{}", "#".repeat(hashes));
            let body = end + hashes + 1;
            let length = source[body..]
                .find(&terminator)
                .expect("a raw string literal is not closed");
            return body + length + terminator.len();
        }
    }
    end
}

fn first_difference(generated: &str, committed: &str) -> Option<String> {
    first_different_token(generated, committed)
        .or_else(|| first_unformatted_line_with_another_indentation(generated, committed))
}

fn first_unformatted_line_with_another_indentation(
    generated: &str,
    committed: &str,
) -> Option<String> {
    let as_generated = generated.lines().map(str::trim_end).collect::<HashSet<_>>();
    let unindented = generated.lines().map(str::trim).collect::<HashSet<_>>();
    let (number, line) = committed
        .lines()
        .map(str::trim_end)
        .enumerate()
        .filter(|(_, line)| line.chars().count() > RUSTFMT_MAX_WIDTH)
        .filter(|(_, line)| !line.trim_start().starts_with("//"))
        .find(|(_, line)| unindented.contains(line.trim_start()) && !as_generated.contains(line))?;
    Some(format!(
        "line {} of the committed file, which is too wide for rustfmt to have laid it out, is a \
         line that the generator indents differently: {}",
        number + 1,
        line.trim_start()
    ))
}

fn first_different_token(generated: &str, committed: &str) -> Option<String> {
    let generated = Canonical::of(generated);
    let committed = Canonical::of(committed);
    let differs_at = generated
        .texts()
        .zip(committed.texts())
        .position(|(generated, committed)| generated != committed)
        .or_else(|| {
            (generated.tokens.len() != committed.tokens.len())
                .then(|| generated.tokens.len().min(committed.tokens.len()))
        })?;
    let excerpt = |canonical: &Canonical<'_>| {
        let from = differs_at.saturating_sub(8);
        let to = (differs_at + 16).min(canonical.tokens.len());
        let line = canonical.tokens.get(differs_at).map_or_else(
            || "the end".to_string(),
            |token| format!("line {}", token.line),
        );
        let tokens = (from..to)
            .map(|index| match canonical.text(index) {
                "" => "<blank line>",
                text => text,
            })
            .collect::<Vec<_>>()
            .join(" ");
        format!("{line}: {tokens}")
    };
    Some(format!(
        "the generator writes (at {})\nbut the committed file has (at {})",
        excerpt(&generated),
        excerpt(&committed)
    ))
}

fn same(generated: &str, committed: &str) -> bool {
    first_difference(generated, committed).is_none()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("the crate is two directories below the workspace root")
        .to_path_buf()
}

#[test]
fn generating_from_the_embedded_registry_reproduces_the_committed_generated_files() {
    let root = workspace_root();
    let files = generate(Registry::get(), Docs::get());
    let mut stale = Vec::new();

    let generated_paths = files
        .iter()
        .map(|file| file.path.clone())
        .collect::<BTreeSet<_>>();
    let committed_paths = [GENERATED_SRC, GENERATED_TESTS]
        .iter()
        .flat_map(|directory| {
            let entries = fs::read_dir(root.join(directory))
                .unwrap_or_else(|error| panic!("read {directory}: {error}"));
            entries.map(move |entry| {
                let name = entry
                    .unwrap_or_else(|error| panic!("read {directory}: {error}"))
                    .file_name();
                format!("{directory}/{}", name.to_string_lossy())
            })
        })
        .collect::<BTreeSet<_>>();
    for path in generated_paths.difference(&committed_paths) {
        stale.push(format!("{path} is generated but not committed"));
    }
    for path in committed_paths.difference(&generated_paths) {
        stale.push(format!("{path} is committed but no longer generated"));
    }

    let differences = std::thread::scope(|scope| {
        let checks = files
            .iter()
            .filter(|file| committed_paths.contains(&file.path))
            .map(|file| {
                let root = &root;
                scope.spawn(move || {
                    let committed = fs::read_to_string(root.join(&file.path))
                        .unwrap_or_else(|error| panic!("read {}: {error}", file.path));
                    if file.path.ends_with(".rs") {
                        first_difference(&file.contents, &committed)
                            .map(|difference| format!("{}: {difference}", file.path))
                    } else {
                        (file.contents != committed)
                            .then(|| format!("{} is not what the generator writes", file.path))
                    }
                })
            })
            .collect::<Vec<_>>();
        checks
            .into_iter()
            .filter_map(|check| check.join().expect("the comparison of one file"))
            .collect::<Vec<_>>()
    });
    stale.extend(differences);

    assert!(
        stale.is_empty(),
        "the committed generated files are not what the generator writes from the committed \
         registry; {REGENERATE}.\n\
         (Rust files are compared as rustfmt leaves them: by their tokens, doc comment lines and \
         blank lines, without trailing commas, doubled parentheses, braces around the expression \
         of a closure and the order of imports; a line that is too wide for rustfmt is compared \
         with its indentation. If \
         regenerating changes nothing, rustfmt rewrites the generated code in one more way, and \
         this comparison has to learn it.)\n\n{}",
        stale.join("\n\n")
    );
}

#[test]
fn the_comparison_ignores_what_rustfmt_rewrites() {
    let generated = "use core::ffi::{c_char, c_void, CStr};
use core::marker::PhantomData;
use alloc::vec::Vec;

mod b;
mod a;

impl A {
pub const fn null() -> Self { Self(0) }
fn set(mut self, value: u32,) -> Self {

self.bits = (self.bits & !(0x1)) | ((value & 0x1));
call(first, second,)
}
fn pointer(&self) -> *const u8 {
self.name.map_or(core::ptr::null(), |x| if x.is_empty() { core::ptr::null() } else { x.as_ptr() })
}

}
const ALL: &[(Self, &str)] = &[(Self(0), \"NONE\"),];
type F = Option<unsafe extern \"system\" fn(a: A, b: B,) -> C>;
type G = Map<A, B,>;
";
    let formatted = "use alloc::vec::Vec;
use core::ffi::{CStr, c_char, c_void};
use core::marker::PhantomData;

mod a;
mod b;

impl A {
    pub const fn null() -> Self {
        Self(0)
    }
    fn set(mut self, value: u32) -> Self {
        self.bits = (self.bits & !(0x1)) | (value & 0x1);
        call(
            first,
            second,
        )
    }
    fn pointer(&self) -> *const u8 {
        self.name.map_or(core::ptr::null(), |x| {
            if x.is_empty() {
                core::ptr::null()
            } else {
                x.as_ptr()
            }
        })
    }
}
const ALL: &[(Self, &str)] = &[(Self(0), \"NONE\")];
type F = Option<
    unsafe extern \"system\" fn(
        a: A,
        b: B,
    ) -> C,
>;
type G = Map<A, B>;
";
    assert_eq!(first_difference(generated, formatted), None);
}

#[test]
fn the_comparison_notices_what_rustfmt_keeps() {
    let changes = [
        ("fn f() -> u32 { 1 }", "fn f() -> u32 { 2 }"),
        ("fn f() -> u32 { 1 }", "fn g() -> u32 { 1 }"),
        ("fn f() { a.b() }", "fn f() { a.b }"),
        ("fn f() { a - b }", "fn f() { a + b }"),
        ("fn f() {}", "fn f() {}\nfn g() {}"),
        ("fn f() {}\nfn g() {}", "fn f() {}"),
        ("/// One.\nfn f() {}", "/// Two.\nfn f() {}"),
        ("/// One.\nfn f() {}", "///  One.\nfn f() {}"),
        ("/// One.\nfn f() {}", "///One.\nfn f() {}"),
        ("/// One.\nfn f() {}", "fn f() {}"),
        ("/// One\n/// two.\nfn f() {}", "/// One two.\nfn f() {}"),
        (
            "/// One.\n///\n/// Two.\nfn f() {}",
            "/// One.\n/// Two.\nfn f() {}",
        ),
        ("//! One.", "/// One."),
        ("const A: &str = \"a b\";", "const A: &str = \"a  b\";"),
        ("const A: &str = \"a b\";", "const A: &str = \"ab\";"),
        ("const A: &str = \"a\\\"b\";", "const A: &str = \"a\\\"c\";"),
        ("const A: &CStr = c\"a\";", "const A: &str = \"a\";"),
        ("const A: char = 'a';", "const A: char = 'b';"),
        ("const A: char = '\\'';", "const A: char = '\"';"),
        ("fn f<'a>(a: &'a A) {}", "fn f<'b>(a: &'b A) {}"),
        ("fn f() {}\n\nfn g() {}", "fn f() {}\nfn g() {}"),
        ("fn f() {}\nfn g() {}", "fn f() {}\n\nfn g() {}"),
        ("fn f() { a = (b,); }", "fn f() { a = (b); }"),
        ("fn f() { a(b, c) }", "fn f() { a((b, c)) }"),
        ("fn f() { a(b) }", "fn f() { a((b)) }"),
        ("fn f() { a[0] }", "fn f() { a(0) }"),
        ("fn f() { a(b, c) }", "fn f() { a(c, b) }"),
        ("use a::b;\n\nuse a::a;", "use a::a;\n\nuse a::b;"),
        ("mod a {}\nmod b;", "mod b;\nmod a {}"),
        ("struct A { a: u32, b: u32 }", "struct A { b: u32, a: u32 }"),
        ("#[cfg(a)]\nfn f() {}", "fn f() {}"),
        ("r#type", "type"),
        (
            "const A: &str = r#\"a b\"#;",
            "const A: &str = r#\"a  b\"#;",
        ),
        ("a = b;", "a == b;"),
        ("f(|x| { a; b })", "f(|x| { a; c })"),
        ("f(|| { a })", "f(|| { b })"),
        ("f(|x| { a; })", "f(|x| a)"),
        ("f(|x| {})", "f(|x| a)"),
        ("f(a, { b })", "f(a, b)"),
        ("a mut self", "a mutself"),
    ];
    let wide = format!("call(\"{}\");", "x".repeat(RUSTFMT_MAX_WIDTH));
    let indented = [
        format!("fn f() {{\n    g(\n        a,\n        {wide}\n    )\n}}"),
        format!("fn f() {{\n    g(\n        a,\n      {wide}\n    )\n}}"),
    ];
    let changes = changes
        .into_iter()
        .chain([(indented[0].as_str(), indented[1].as_str())]);
    for (generated, committed) in changes {
        assert!(
            !same(generated, committed),
            "`{generated}` and `{committed}` are taken for the same source"
        );
        assert!(
            same(generated, generated) && same(committed, committed),
            "`{generated}` or `{committed}` differs from itself"
        );
    }
}
