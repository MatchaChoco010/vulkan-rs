use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use regex::{Captures, Regex};
use syn::{Attribute, Expr, FnArg, ImplItem, Item, Lit, Meta, Pat, Type};

const REFERENCE_PAGES: &str = "https://docs.vulkan.org/refpages/latest/refpages/source/";

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/vulkan has a workspace root two levels up")
        .to_path_buf()
}

fn regex(pattern: &str) -> Regex {
    Regex::new(pattern).unwrap_or_else(|e| panic!("{pattern}: {e}"))
}

fn words(text: &str) -> String {
    let mut out = String::from(" ");
    for word in text
        .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
        .filter(|word| !word.is_empty())
    {
        out.push_str(word);
        out.push(' ');
    }
    out
}

struct Page {
    description: String,
    body: String,
    body_words: String,
}

struct Specification {
    pages: BTreeMap<String, Page>,
    extensions: BTreeSet<String>,
    attributes: BTreeMap<String, String>,
    titles: BTreeMap<String, String>,
    markup: Markup,
}

struct Markup {
    anchor: Regex,
    cross_reference: Regex,
    attribute_reference: Regex,
    labelled_url: Regex,
    role: Regex,
    macro_prefix: Regex,
}

impl Markup {
    fn new() -> Self {
        Self {
            anchor: regex(r"\[\[[^\]]*\]\]"),
            cross_reference: regex(r"(?s)<<([^,>]*)(?:,(.*?))?>>"),
            attribute_reference: regex(r"\{([\w-]+)\}"),
            labelled_url: regex(r"(?:link:)?(https?://[^\s\[]+)\[([^\]]*)\]"),
            role: regex(r"\[[\w.-]+\]#"),
            macro_prefix: regex(
                r"\b(?:pname|ptext|sname|stext|slink|fname|ftext|flink|ename|etext|elink|dname|dlink|tname|tlink|basetype|code|apiext|reflink|tag|attr|latexmath):",
            ),
        }
    }
}

struct Sources {
    root: PathBuf,
    defined: BTreeSet<String>,
    conditional: Regex,
    end_conditional: Regex,
    tagged_include: Regex,
    tag_marker: Regex,
    attribute_entry: Regex,
    attribute_reference: Regex,
}

fn xml_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(" {name}=\""))? + name.len() + 3;
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

impl Sources {
    fn new(root: &Path) -> Self {
        let registry = fs::read_to_string(root.join("xml/vk.xml")).expect("read xml/vk.xml");
        let mut defined = BTreeSet::new();
        for tag in regex(r"<(extension|feature)\s[^>]*>").find_iter(&registry) {
            let tag = tag.as_str();
            let apis = xml_attribute(tag, "supported").or_else(|| xml_attribute(tag, "api"));
            let for_vulkan = apis.is_some_and(|apis| apis.split(',').any(|api| api == "vulkan"));
            if let (true, Some(name)) = (for_vulkan, xml_attribute(tag, "name")) {
                defined.insert(name.to_string());
            }
        }
        Self {
            root: root.to_path_buf(),
            defined,
            conditional: regex(r"^(ifdef|ifndef)::([^\[]*)\[(.*)\]\s*$"),
            end_conditional: regex(r"^endif::[^\[]*\[\]\s*$"),
            tagged_include: regex(r"^include::\{chapters\}/([^\[]+)\[tags?=([^\],;]+)\]\s*$"),
            tag_marker: regex(r"^// (tag|end)::([^\[]+)\[\]\s*$"),
            attribute_entry: regex(r"^:([\w-]+)(!?):\s*(.*)$"),
            attribute_reference: regex(r"\{([\w-]+)\}"),
        }
    }

    fn holds(&self, directive: &str, target: &str) -> bool {
        let is_defined = |name: &str| self.defined.contains(name);
        let result = if target.contains(',') {
            target.split(',').any(is_defined)
        } else if target.contains('+') {
            target.split('+').all(is_defined)
        } else {
            is_defined(target)
        };
        result == (directive == "ifdef")
    }

    fn with_locals(&self, line: &str, locals: &BTreeMap<String, String>) -> String {
        self.attribute_reference
            .replace_all(line, |reference: &Captures<'_>| {
                locals
                    .get(&reference[1])
                    .cloned()
                    .unwrap_or_else(|| reference[0].to_string())
            })
            .into_owned()
    }

    fn lines(
        &self,
        file: &Path,
        tag: Option<&str>,
        locals: &mut BTreeMap<String, String>,
        out: &mut Vec<String>,
    ) {
        let text = fs::read_to_string(file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        let mut conditions = Vec::new();
        let mut in_region = tag.is_none();
        for line in text.lines() {
            if let Some(marker) = self.tag_marker.captures(line) {
                if tag == Some(&marker[2]) {
                    in_region = &marker[1] == "tag";
                }
                continue;
            }
            if !in_region {
                continue;
            }
            let active = conditions.iter().all(|holds| *holds);
            if let Some(conditional) = self.conditional.captures(line) {
                let holds = self.holds(&conditional[1], &conditional[2]);
                if conditional[3].is_empty() {
                    conditions.push(holds);
                } else if holds && active {
                    out.push(self.with_locals(&conditional[3], locals));
                }
                continue;
            }
            if self.end_conditional.is_match(line) {
                conditions.pop();
                continue;
            }
            if !active || line.starts_with("//") {
                continue;
            }
            if let Some(include) = self.tagged_include.captures(line) {
                let included = self.root.join("chapters").join(&include[1]);
                self.lines(&included, Some(&include[2]), locals, out);
                continue;
            }
            if let Some(entry) = self.attribute_entry.captures(line) {
                if entry[2].is_empty() {
                    let value = self.with_locals(&entry[3], locals);
                    locals.insert(entry[1].to_string(), value);
                } else {
                    locals.remove(&entry[1]);
                }
                out.push(String::new());
                continue;
            }
            out.push(self.with_locals(line, locals));
        }
    }

    fn files(&self) -> Vec<PathBuf> {
        fn collect(directory: &Path, files: &mut Vec<PathBuf>) {
            for entry in fs::read_dir(directory).expect("read a specification directory") {
                let path = entry.expect("directory entry").path();
                if path.is_dir() {
                    collect(&path, files);
                } else if path
                    .extension()
                    .is_some_and(|extension| extension == "adoc")
                {
                    files.push(path);
                }
            }
        }
        let mut files = Vec::new();
        collect(&self.root.join("chapters"), &mut files);
        collect(&self.root.join("appendices"), &mut files);
        files.sort();
        files
    }
}

impl Specification {
    fn read(root: &Path) -> Self {
        let sources = Sources::new(root);
        let open = regex(r"^\[open,(refpage=.*)\]\s*$");
        let page_attribute = regex(r"(\w+)='((?:[^'\\]|\\.)*)'");
        let anchor_line = regex(r"^\[\[([\w.-]+)\]\]\s*$");
        let title_line = regex(r#"^(?:=+ +|\.|image::.*\btitle=")([^\s.="][^"]*?)\s*(?:".*)?$"#);

        let mut pages = BTreeMap::new();
        let mut titles = BTreeMap::new();
        for file in sources.files() {
            let mut anchors = Vec::<String>::new();
            for line in fs::read_to_string(&file).expect("read a chapter").lines() {
                if let Some(anchor) = anchor_line.captures(line) {
                    anchors.push(anchor[1].to_string());
                } else if !line.trim().is_empty()
                    && !["ifdef::", "ifndef::", "endif::", "//", "["]
                        .iter()
                        .any(|prefix| line.starts_with(prefix))
                {
                    if let Some(title) = title_line.captures(line) {
                        for anchor in &anchors {
                            titles
                                .entry(anchor.clone())
                                .or_insert_with(|| title[1].to_string());
                        }
                    }
                    anchors.clear();
                }
            }

            let mut lines = Vec::new();
            sources.lines(&file, None, &mut BTreeMap::new(), &mut lines);
            let mut index = 0;
            while index + 1 < lines.len() {
                let Some(opened) = open.captures(&lines[index]) else {
                    index += 1;
                    continue;
                };
                if lines[index + 1].trim() != "--" {
                    index += 1;
                    continue;
                }
                let attributes = page_attribute
                    .captures_iter(&opened[1])
                    .map(|attribute| (attribute[1].to_string(), attribute[2].replace("\\'", "'")))
                    .collect::<BTreeMap<_, _>>();
                let end = (index + 2..lines.len())
                    .find(|line| lines[*line].trim() == "--")
                    .unwrap_or(lines.len());
                let body = lines[index + 2..end].join("\n");
                let description = attributes.get("desc").cloned().unwrap_or_default();
                let names = std::iter::once(attributes["refpage"].as_str()).chain(
                    attributes
                        .get("alias")
                        .map(String::as_str)
                        .unwrap_or_default()
                        .split_whitespace(),
                );
                for name in names {
                    pages.entry(name.to_string()).or_insert_with(|| Page {
                        description: description.clone(),
                        body: body.clone(),
                        body_words: String::new(),
                    });
                }
                index = end + 1;
            }
        }

        let mut attributes = BTreeMap::new();
        let mut configuration = Vec::new();
        sources.lines(
            &root.join("config/attribs.adoc"),
            None,
            &mut attributes,
            &mut configuration,
        );
        let passthrough = regex(r"^pass:\w*\[(.*)\]$");
        for value in attributes.values_mut() {
            if let Some(inner) = passthrough.captures(value) {
                *value = inner[1].to_string();
            }
        }
        let extensions = sources
            .defined
            .iter()
            .filter(|name| !name.contains("_VERSION_"))
            .cloned()
            .collect();
        let mut specification = Self {
            pages: BTreeMap::new(),
            extensions,
            attributes,
            titles,
            markup: Markup::new(),
        };
        for page in pages.values_mut() {
            page.body_words = specification.words(&page.body);
        }
        specification.pages = pages;
        specification
    }

    fn words(&self, text: &str) -> String {
        let markup = &self.markup;
        let text = markup.anchor.replace_all(text, "");
        let text = markup
            .cross_reference
            .replace_all(&text, |r: &Captures<'_>| {
                match r.get(2).map(|label| label.as_str().trim()) {
                    Some(label) if !label.is_empty() => label.to_string(),
                    _ => self
                        .titles
                        .get(r[1].trim())
                        .cloned()
                        .unwrap_or_else(|| r[1].to_string()),
                }
            });
        let text = markup
            .attribute_reference
            .replace_all(&text, |r: &Captures<'_>| {
                self.attributes
                    .get(&r[1])
                    .cloned()
                    .unwrap_or_else(|| r[0].to_string())
            });
        let text = markup.labelled_url.replace_all(&text, |r: &Captures<'_>| {
            match r[2].trim_end_matches('^').trim() {
                "" => r[1].to_string(),
                label => label.to_string(),
            }
        });
        let text = markup.role.replace_all(&text, "");
        let text = markup.macro_prefix.replace_all(&text, "");
        words(&text)
    }

    fn describes(&self, page: &str, macro_name: &str, item: &str) -> bool {
        self.described(page, macro_name)
            .iter()
            .any(|described| described == item)
    }

    fn described(&self, page: &str, macro_name: &str) -> Vec<String> {
        let bullet = regex(&format!(
            r"(?m)^[ \t]*\*[ \t]+(?:\[\[[^\]]*\]\]\s*)*{macro_name}:(\w+)\b"
        ));
        self.pages.get(page).map_or_else(Vec::new, |page| {
            bullet
                .captures_iter(&page.body)
                .map(|bullet| bullet[1].to_string())
                .collect()
        })
    }
}

fn doc_attributes(attributes: &[Attribute]) -> (String, Vec<String>) {
    let mut text = String::new();
    let mut aliases = Vec::new();
    for attribute in attributes.iter().filter(|a| a.path().is_ident("doc")) {
        match &attribute.meta {
            Meta::NameValue(pair) => {
                if let Expr::Lit(literal) = &pair.value
                    && let Lit::Str(line) = &literal.lit
                {
                    text.push_str(line.value().strip_prefix(' ').unwrap_or(&line.value()));
                    text.push('\n');
                }
            }
            Meta::List(list) => {
                let _ = list.parse_nested_meta(|nested| {
                    if nested.path.is_ident("alias") {
                        aliases.push(nested.value()?.parse::<syn::LitStr>()?.value());
                    }
                    Ok(())
                });
            }
            Meta::Path(_) => {}
        }
    }
    (text, aliases)
}

fn generated_words(markdown: &str) -> String {
    let mut text = String::with_capacity(markdown.len());
    let mut rest = markdown;
    while let Some(target) = rest.find("](") {
        text.push_str(&rest[..=target]);
        let after = &rest[target + 2..];
        rest = &after[after.find(')').map_or(after.len(), |close| close + 1)..];
    }
    text.push_str(rest);
    for tag in ["<sub>", "</sub>", "<sup>", "</sup>"] {
        text = text.replace(tag, " ");
    }
    words(&text)
}

fn squeeze(name: &str) -> String {
    name.replace('_', "").to_ascii_lowercase()
}

fn flat(doc: &str) -> String {
    doc.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_paragraph(doc: &str) -> &str {
    doc.split("\n\n").next().unwrap_or_default()
}

fn section_items(doc: &str, title: &str) -> Vec<String> {
    let mut items = Vec::<String>::new();
    let mut in_section = false;
    for line in doc.lines() {
        if let Some(heading) = line.strip_prefix("# ") {
            in_section = heading == title;
        } else if !in_section {
            continue;
        } else if let Some(item) = line.strip_prefix("- ") {
            items.push(item.to_string());
        } else if let Some(last) = items.last_mut() {
            last.push('\n');
            last.push_str(line);
        }
    }
    items
}

#[derive(Default)]
struct Checker {
    failures: Vec<String>,
    reference_links: usize,
    summaries: usize,
    described_members: usize,
    undescribed_members: usize,
    described_values: usize,
    undescribed_values: usize,
    described_parameters: usize,
    functions_with_arguments: usize,
}

struct Generated {
    reference_link: Regex,
    code_at_start: Regex,
    value_of: Regex,
    bits_of: Regex,
}

impl Generated {
    fn new() -> Self {
        Self {
            reference_link: regex(&format!(
                r"\[`([\w-]+)`\]\({}([\w-]+)\.html\)",
                regex::escape(REFERENCE_PAGES)
            )),
            code_at_start: regex(r"^`(\w+)`"),
            value_of: regex(r"^`\w+`, a value of (?:\[`(\w+)`\]|`(\w+)`)"),
            bits_of: regex(r"constants of \[`(\w+)`\]"),
        }
    }

    fn page_of(&self, doc: &str) -> Option<String> {
        self.reference_link
            .captures(doc)
            .map(|link| link[2].to_string())
    }
}

impl Checker {
    fn fail(&mut self, item: &str, problem: impl AsRef<str>) {
        self.failures.push(format!("{item}: {}", problem.as_ref()));
    }

    fn links(&mut self, spec: &Specification, generated: &Generated, item: &str, doc: &str) {
        for link in generated.reference_link.captures_iter(doc) {
            self.reference_links += 1;
            let page = &link[2];
            if !(spec.pages.contains_key(page) || spec.extensions.contains(page)) {
                self.fail(item, format!("links to the reference page `{page}`, which the specification does not have"));
            }
        }
        let other = doc
            .match_indices("docs.vulkan.org")
            .count()
            .saturating_sub(generated.reference_link.captures_iter(doc).count());
        if other != 0 {
            self.fail(item, "has a docs.vulkan.org link of an unexpected form");
        }
    }

    fn summary(&mut self, spec: &Specification, item: &str, doc: &str, aliases: &[String]) {
        for alias in aliases {
            let Some(page) = spec.pages.get(alias) else {
                continue;
            };
            if page.description.trim().is_empty() || doc.starts_with("Alias of ") {
                continue;
            }
            self.summaries += 1;
            let expected = spec.words(&page.description);
            let actual = generated_words(first_paragraph(doc));
            if expected != actual {
                self.fail(
                    item,
                    format!(
                        "starts with `{}`, but the specification describes `{alias}` as `{}`",
                        first_paragraph(doc),
                        page.description
                    ),
                );
            }
        }
    }

    fn contained(&mut self, spec: &Specification, item: &str, page: &str, text: &str) -> bool {
        let Some(block) = spec.pages.get(page) else {
            self.fail(
                item,
                format!("is described, but the specification has no page `{page}`"),
            );
            return false;
        };
        if block.body_words.contains(&generated_words(text)) {
            return true;
        }
        self.fail(
            item,
            format!(
                "`{}` is not in the text that the specification has for `{page}`",
                text.replace('\n', " ")
            ),
        );
        false
    }

    fn member(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        owner_page: Option<&str>,
        owner: &str,
        field: &str,
        doc: &str,
    ) {
        let item = format!("{owner}::{field}");
        let doc = &flat(doc);
        if doc.starts_with("The storage unit of the C bit-fields ")
            || doc.starts_with("Ties the value to the lifetime ")
            || doc.starts_with("The pointer to ")
        {
            return;
        }
        if let Some(declaration) = doc.strip_prefix("The C member `") {
            self.undescribed_members += 1;
            let declaration = declaration.split(['`', '[']).next().unwrap_or_default();
            let declarator = declaration.split(" : ").next().unwrap_or_default();
            let name = declarator.rsplit(' ').next().unwrap_or_default();
            if let Some(page) = owner_page
                && spec.describes(page, "pname", name)
            {
                self.fail(&item, format!("the specification describes `{name}` on the page `{page}`, but the documentation does not use it"));
            }
            return;
        }
        self.described_members += 1;
        let Some(page) = owner_page else {
            self.fail(
                &item,
                "is described although its type has no reference page",
            );
            return;
        };
        let Some(name) = generated.code_at_start.captures(doc) else {
            self.fail(&item, "does not start with the name of the member");
            return;
        };
        let same_name =
            squeeze(&name[1]) == squeeze(field) || (field == "ty" && &name[1] == "type");
        if !same_name {
            self.fail(
                &item,
                format!("is documented with the description of `{}`", &name[1]),
            );
        }
        self.contained(spec, &item, page, doc);
    }

    fn value(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        owner_page: Option<&str>,
        item: &str,
        doc: &str,
        aliases: &[String],
    ) {
        let doc = &flat(doc);
        let [c_name] = aliases else {
            return;
        };
        if !doc.starts_with(&format!("`{c_name}`")) {
            self.fail(item, format!("does not start with its C name `{c_name}`"));
            return;
        }
        let rest = &doc[c_name.len() + 2..];
        if rest.starts_with(" is an alias of [`")
            || rest.starts_with(", the structure type of ")
            || rest.starts_with(", the object type of ")
        {
            return;
        }
        if let Some(value_of) = generated.value_of.captures(doc) {
            self.undescribed_values += 1;
            let page = value_of
                .get(1)
                .or(value_of.get(2))
                .map_or("", |m| m.as_str());
            if spec.describes(page, "ename", c_name) {
                self.fail(item, format!("the specification describes `{c_name}` on the page `{page}`, but the documentation does not use it"));
            }
            return;
        }
        self.described_values += 1;
        match owner_page {
            Some(page) => {
                self.contained(spec, item, page, doc);
            }
            None => self.fail(item, "is described although its type has no reference page"),
        }
    }

    fn parameters(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        item: &str,
        doc: &str,
        arguments: &[String],
    ) {
        let parameters = section_items(doc, "Parameters");
        let Some(page) = generated.page_of(doc) else {
            if !parameters.is_empty() {
                self.fail(item, "lists parameters although it has no reference page");
            }
            return;
        };
        for parameter in &parameters {
            self.described_parameters += 1;
            self.contained(spec, item, &page, parameter);
        }
        if arguments.is_empty() {
            return;
        }
        self.functions_with_arguments += 1;
        let listed = parameters
            .iter()
            .filter_map(|parameter| generated.code_at_start.captures(parameter))
            .map(|name| squeeze(&name[1]))
            .collect::<BTreeSet<_>>();
        let arguments = arguments
            .iter()
            .map(|argument| squeeze(argument))
            .collect::<BTreeSet<_>>();
        for name in spec.described(&page, "pname") {
            if arguments.contains(&squeeze(&name)) && !listed.contains(&squeeze(&name)) {
                self.fail(item, format!("the specification describes the parameter `{name}` on the page `{page}`, but the documentation does not list it"));
            }
        }
    }
}

fn function_pointer_arguments(ty: &Type) -> Vec<String> {
    let Type::Path(path) = ty else {
        return Vec::new();
    };
    let Some(segment) = path.path.segments.last() else {
        return Vec::new();
    };
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return Vec::new();
    };
    let Some(syn::GenericArgument::Type(Type::BareFn(function))) = arguments.args.first() else {
        return Vec::new();
    };
    function
        .inputs
        .iter()
        .filter_map(|input| input.name.as_ref().map(|(name, _)| name.to_string()))
        .collect()
}

fn function_arguments(signature: &syn::Signature) -> Vec<String> {
    signature
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Typed(typed) => match &*typed.pat {
                Pat::Ident(name) => Some(name.ident.to_string()),
                _ => None,
            },
            FnArg::Receiver(_) => None,
        })
        .collect()
}

fn type_name(ty: &Type) -> String {
    match ty {
        Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or_default(),
        _ => String::new(),
    }
}

#[test]
fn generated_documentation_matches_the_specification() {
    let root = workspace_root();
    let spec_root = root.join("third_party/vulkan-docs");
    if !spec_root.join("chapters").is_dir() {
        assert!(
            std::env::var_os("VULKAN_RS_REQUIRE_DOCS_TEST").is_none(),
            "the documentation test is required but third_party/vulkan-docs is not checked out"
        );
        eprintln!("skipping the documentation test: third_party/vulkan-docs is not checked out");
        return;
    }
    let spec = Specification::read(&spec_root);
    let generated = Generated::new();
    let mut checker = Checker::default();

    let directory = root.join("crates/vulkan/src/vk/generated");
    let mut files = Vec::new();
    for entry in fs::read_dir(&directory).expect("read the generated sources") {
        let path = entry.expect("directory entry").path();
        let source = fs::read_to_string(&path).expect("read a generated source");
        let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        files.push(file);
    }

    check_files(&spec, &generated, &mut checker, &files);

    eprintln!(
        "checked {} reference links, {} summaries, {} described and {} undescribed members, {} described and {} undescribed values, {} parameters, the parameter lists of {} functions",
        checker.reference_links,
        checker.summaries,
        checker.described_members,
        checker.undescribed_members,
        checker.described_values,
        checker.undescribed_values,
        checker.described_parameters,
        checker.functions_with_arguments,
    );
    assert!(
        checker.failures.is_empty(),
        "{} generated doc comments do not match the specification:\n{}",
        checker.failures.len(),
        checker.failures.join("\n")
    );
    for (what, count, at_least) in [
        ("reference links", checker.reference_links, 5000),
        ("summaries", checker.summaries, 2500),
        ("described members", checker.described_members, 5000),
        ("described values", checker.described_values, 2500),
        ("parameters", checker.described_parameters, 2000),
        ("parameter lists", checker.functions_with_arguments, 800),
    ] {
        assert!(count >= at_least, "only {count} {what} were checked");
    }
}

fn check_files(
    spec: &Specification,
    generated: &Generated,
    checker: &mut Checker,
    files: &[syn::File],
) {
    let mut value_pages = BTreeMap::<String, String>::new();
    let mut c_names = BTreeMap::<String, String>::new();
    for item in files.iter().flat_map(|file| &file.items) {
        let (name, attributes) = match item {
            Item::Struct(item) => (item.ident.to_string(), &item.attrs),
            Item::Union(item) => (item.ident.to_string(), &item.attrs),
            _ => continue,
        };
        let (doc, aliases) = doc_attributes(attributes);
        if let Some(c_name) = aliases.first() {
            c_names.insert(c_name.clone(), name.clone());
        }
        if let Some(page) = generated.page_of(&doc) {
            value_pages.insert(name, page);
        }
    }
    for item in files.iter().flat_map(|file| &file.items) {
        if let Item::Struct(item) = item {
            let (doc, _) = doc_attributes(&item.attrs);
            if let Some(bits) = generated.bits_of.captures(&flat(&doc))
                && let Some(page) = c_names.get(&bits[1]).and_then(|bits| value_pages.get(bits))
            {
                value_pages.insert(item.ident.to_string(), page.clone());
            }
        }
    }

    for item in files.iter().flat_map(|file| &file.items) {
        check_item(spec, generated, checker, &value_pages, item);
    }
}

fn check_item(
    spec: &Specification,
    generated: &Generated,
    checker: &mut Checker,
    value_pages: &BTreeMap<String, String>,
    item: &Item,
) {
    match item {
        Item::Struct(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
            let page = generated.page_of(&doc);
            for field in item.fields.iter().filter(|field| field.ident.is_some()) {
                let field_name = field.ident.as_ref().expect("named field").to_string();
                let (doc, _) = doc_attributes(&field.attrs);
                checker.links(spec, generated, &name, &doc);
                checker.member(
                    spec,
                    generated,
                    page.as_deref(),
                    &name,
                    &field_name,
                    doc.trim(),
                );
            }
        }
        Item::Union(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
            let page = generated.page_of(&doc);
            for field in &item.fields.named {
                let field_name = field.ident.as_ref().expect("named field").to_string();
                let (doc, _) = doc_attributes(&field.attrs);
                checker.links(spec, generated, &name, &doc);
                checker.member(
                    spec,
                    generated,
                    page.as_deref(),
                    &name,
                    &field_name,
                    doc.trim(),
                );
            }
        }
        Item::Type(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
            let arguments = if generated.page_of(&doc).as_deref() == Some(name.as_str()) {
                function_pointer_arguments(&item.ty)
            } else {
                Vec::new()
            };
            checker.parameters(spec, generated, &name, &doc, &arguments);
            if let Some(target) = flat(&doc).strip_prefix("Alias of [`") {
                let target = target.split('`').next().unwrap_or_default();
                if target != type_name(&item.ty) {
                    checker.fail(&name, format!("is documented as an alias of `{target}`"));
                }
            }
        }
        Item::Const(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
        }
        Item::Mod(item) => {
            for nested in item.content.iter().flat_map(|(_, items)| items) {
                check_item(spec, generated, checker, value_pages, nested);
            }
        }
        Item::Impl(item) if item.trait_.is_none() => {
            let owner = type_name(&item.self_ty);
            for member in &item.items {
                match member {
                    ImplItem::Const(constant) => {
                        let (doc, aliases) = doc_attributes(&constant.attrs);
                        let name = constant.ident.to_string();
                        checker.links(spec, generated, &format!("{owner}::{name}"), &doc);
                        checker.value(
                            spec,
                            generated,
                            value_pages.get(&owner).map(String::as_str),
                            &format!("{owner}::{name}"),
                            doc.trim(),
                            &aliases,
                        );
                    }
                    ImplItem::Fn(function) => {
                        let (doc, aliases) = doc_attributes(&function.attrs);
                        let name = format!("{owner}::{}", function.sig.ident);
                        checker.links(spec, generated, &name, &doc);
                        checker.summary(spec, &name, &doc, &aliases);
                        let arguments = if flat(&doc).contains(" with the arguments as they are.") {
                            function_arguments(&function.sig)
                        } else {
                            Vec::new()
                        };
                        checker.parameters(spec, generated, &name, &doc, &arguments);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn fixture_specification() -> Specification {
    let mut specification = Specification {
        pages: BTreeMap::new(),
        extensions: BTreeSet::new(),
        attributes: BTreeMap::new(),
        titles: BTreeMap::new(),
        markup: Markup::new(),
    };
    let pages = [
        (
            "VkThing",
            "Structure specifying a thing",
            "* pname:count is the number of things.\n* pname:flags is reserved for future use.",
        ),
        (
            "VkColor",
            "Colors of a thing",
            "* ename:VK_COLOR_RED specifies red.\n* ename:VK_COLOR_GREEN specifies green.",
        ),
        (
            "vkCountThings",
            "Count the things",
            "* pname:device is the logical device.\n* pname:pThingCount is a pointer to the number of things.\n* pname:pAllocator controls host memory allocation.",
        ),
        (
            "PFN_vkThingCallback",
            "Application-defined thing callback",
            "* pname:pUserData is the user data.\n* pname:thing is the thing.",
        ),
    ];
    for (name, description, body) in pages {
        let page = Page {
            description: description.to_string(),
            body: body.to_string(),
            body_words: specification.words(body),
        };
        specification.pages.insert(name.to_string(), page);
    }
    specification
}

const FIXTURE_SOURCE: &str = r#"
/// Structure specifying a thing.
///
/// C structure [`VkThing`](PAGES/VkThing.html).
#[doc(alias = "VkThing")]
pub struct Thing {
    /// `count` is the number of things.
    pub count: u32,
    /// `flags` is reserved for future use.
    pub flags: u32,
    /// The C member `uint32_t padding`.
    pub padding: u32,
}

/// Alias of [`Thing`].
pub type ThingKHR = Thing;

/// Colors of a thing.
///
/// C enumeration [`VkColor`](PAGES/VkColor.html).
#[doc(alias = "VkColor")]
pub struct Color(pub i32);

impl Color {
    /// `VK_COLOR_RED` specifies red.
    #[doc(alias = "VK_COLOR_RED")]
    pub const RED: Self = Self(0);
    /// `VK_COLOR_BLUE`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_BLUE")]
    pub const BLUE: Self = Self(2);
}

/// Application-defined thing callback.
///
/// C function pointer type [`PFN_vkThingCallback`](PAGES/PFN_vkThingCallback.html).
///
/// # Parameters
///
/// - `pUserData` is the user data.
/// - `thing` is the thing.
#[doc(alias = "PFN_vkThingCallback")]
pub type PFN_vkThingCallback = Option<unsafe extern "system" fn(p_user_data: *mut c_void, thing: u32)>;

impl Device {
    /// Count the things.
    ///
    /// Calls the C command [`vkCountThings`](PAGES/vkCountThings.html) with the arguments as they are.
    ///
    /// # Parameters
    ///
    /// - `pThingCount` is a pointer to the number of things.
    /// - `pAllocator` controls host memory allocation.
    pub unsafe fn count_things_raw(&self, p_thing_count: *mut u32, p_allocator: *const c_void) {}

    /// Count the things.
    ///
    /// Calls the C command [`vkCountThings`](PAGES/vkCountThings.html) through [`Self::count_things_raw`], which describes the parameters.
    #[doc(alias = "vkCountThings")]
    pub unsafe fn count_things(&self, allocator: Option<&c_void>) -> u32 {
        0
    }
}
"#;

fn fixture_failures(source: &str) -> Vec<String> {
    let source = source.replace("PAGES/", REFERENCE_PAGES);
    let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
    let mut checker = Checker::default();
    check_files(
        &fixture_specification(),
        &Generated::new(),
        &mut checker,
        &[file],
    );
    checker.failures
}

fn assert_reported(defect: &str, replace: &str, with: &str, report: &str) {
    assert_eq!(
        FIXTURE_SOURCE.matches(replace).count(),
        1,
        "{defect}: `{replace}` is not exactly once in the fixture"
    );
    let failures = fixture_failures(&FIXTURE_SOURCE.replacen(replace, with, 1));
    assert!(
        failures.iter().any(|failure| failure.contains(report)),
        "{defect}: expected a failure containing `{report}`, got {failures:#?}"
    );
}

#[test]
fn checker_accepts_documentation_that_matches_the_specification() {
    let failures = fixture_failures(FIXTURE_SOURCE);
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn checker_reports_a_member_fallback_when_the_specification_describes_the_member() {
    assert_reported(
        "a member documented with its C declaration",
        "/// `count` is the number of things.",
        "/// The C member `uint32_t count`.",
        "Thing::count: the specification describes `count` on the page `VkThing`, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_value_fallback_when_the_specification_describes_the_value() {
    assert_reported(
        "a value documented only by its type",
        "/// `VK_COLOR_RED` specifies red.",
        "/// `VK_COLOR_RED`, a value of [`VkColor`](PAGES/VkColor.html).",
        "Color::RED: the specification describes `VK_COLOR_RED` on the page `VkColor`, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_parameter_left_out_of_the_parameters_of_a_command() {
    assert_reported(
        "a command without one of its parameters",
        "    /// - `pAllocator` controls host memory allocation.\n",
        "",
        "Device::count_things_raw: the specification describes the parameter `pAllocator` on the page `vkCountThings`, but the documentation does not list it",
    );
    assert_reported(
        "a command without its parameters section",
        "    ///\n    /// # Parameters\n    ///\n    /// - `pThingCount` is a pointer to the number of things.\n    /// - `pAllocator` controls host memory allocation.\n",
        "",
        "Device::count_things_raw: the specification describes the parameter `pThingCount` on the page `vkCountThings`, but the documentation does not list it",
    );
}

#[test]
fn checker_reports_a_parameter_left_out_of_the_parameters_of_a_function_pointer() {
    assert_reported(
        "a function pointer type without one of its parameters",
        "/// - `thing` is the thing.\n",
        "",
        "PFN_vkThingCallback: the specification describes the parameter `thing` on the page `PFN_vkThingCallback`, but the documentation does not list it",
    );
}

#[test]
fn checker_reports_each_other_kind_of_defect() {
    let cases = [
        (
            "a parameter description that the specification does not have",
            "is a pointer to the number of things.",
            "is a pointer to the number of widgets.",
            "Device::count_things_raw: ``pThingCount` is a pointer to the number of widgets.` is not in the text that the specification has for `vkCountThings`",
        ),
        (
            "parameters without a reference page",
            "/// C function pointer type [`PFN_vkThingCallback`](PAGES/PFN_vkThingCallback.html).",
            "/// C function pointer type `PFN_vkThingCallback`.",
            "PFN_vkThingCallback: lists parameters although it has no reference page",
        ),
        (
            "a link to a reference page that does not exist",
            "[`VkThing`](PAGES/VkThing.html)",
            "[`VkThingy`](PAGES/VkThingy.html)",
            "Thing: links to the reference page `VkThingy`, which the specification does not have",
        ),
        (
            "a member of a type whose reference page does not exist",
            "[`VkThing`](PAGES/VkThing.html)",
            "[`VkThingy`](PAGES/VkThingy.html)",
            "Thing::count: is described, but the specification has no page `VkThingy`",
        ),
        (
            "a docs.vulkan.org link that is not a reference page",
            "/// Alias of [`Thing`].",
            "/// Alias of [`Thing`]. See https://docs.vulkan.org/spec/latest/index.html.",
            "ThingKHR: has a docs.vulkan.org link of an unexpected form",
        ),
        (
            "a summary that differs from the specification",
            "/// Structure specifying a thing.",
            "/// Structure specifying a widget.",
            "Thing: starts with `Structure specifying a widget.`, but the specification describes `VkThing` as `Structure specifying a thing`",
        ),
        (
            "a described member of a type without a reference page",
            "/// C structure [`VkThing`](PAGES/VkThing.html).",
            "/// C structure `VkThing`.",
            "Thing::count: is described although its type has no reference page",
        ),
        (
            "a member description that does not start with the member",
            "/// `count` is the number of things.",
            "/// The number of things.",
            "Thing::count: does not start with the name of the member",
        ),
        (
            "a member with the description of another member",
            "/// `count` is the number of things.",
            "/// `flags` is reserved for future use.",
            "Thing::count: is documented with the description of `flags`",
        ),
        (
            "a member description that the specification does not have",
            "/// `count` is the number of things.",
            "/// `count` is the number of widgets.",
            "Thing::count: ``count` is the number of widgets.` is not in the text that the specification has for `VkThing`",
        ),
        (
            "a value description that does not start with the C name",
            "/// `VK_COLOR_RED` specifies red.",
            "/// Red.",
            "Color::RED: does not start with its C name `VK_COLOR_RED`",
        ),
        (
            "a described value of a type without a reference page",
            "/// C enumeration [`VkColor`](PAGES/VkColor.html).",
            "/// C enumeration `VkColor`.",
            "Color::RED: is described although its type has no reference page",
        ),
        (
            "a value description that the specification does not have",
            "/// `VK_COLOR_RED` specifies red.",
            "/// `VK_COLOR_RED` specifies crimson.",
            "Color::RED: ``VK_COLOR_RED` specifies crimson.` is not in the text that the specification has for `VkColor`",
        ),
        (
            "an alias documented as the alias of another type",
            "pub type ThingKHR = Thing;",
            "pub type ThingKHR = Color;",
            "ThingKHR: is documented as an alias of `Thing`",
        ),
    ];
    for (defect, replace, with, report) in cases {
        assert_reported(defect, replace, with, report);
    }
}
