use std::cell::OnceCell;
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
    name: String,
    file: String,
    description: String,
    body: String,
    body_words: String,
    valid_usage: OnceCell<Vec<String>>,
}

struct Cell {
    style: Option<char>,
    text: String,
}

struct Table {
    file: String,
    page: Option<String>,
    title: Option<String>,
    header: Vec<String>,
    rows: Vec<Vec<Cell>>,
}

struct TableSyntax {
    delimiter: Regex,
    block_header: Regex,
    cell_specifier: Regex,
    cell_at_the_start: Regex,
    columns: Regex,
    column_specifier: Regex,
    header_option: Regex,
    no_header_option: Regex,
    footer_option: Regex,
}

impl TableSyntax {
    fn new() -> Self {
        Self {
            delimiter: regex(r"^\|={3,}\s*$"),
            block_header: regex(r"^(?:\[.*\]|\.[^\s.].*)\s*$"),
            cell_specifier: regex(
                r"^((?:\d+(?:\.\d+)?|\.\d+)[+*])?[<^>]?(?:\.[<^>])?([adehlmsv])?$",
            ),
            cell_at_the_start: regex(
                r"^[ \t]*(?:(?:\d+(?:\.\d+)?|\.\d+)[+*])?[<^>]?(?:\.[<^>])?[adehlmsv]?\|",
            ),
            columns: regex(r#"\bcols="([^"]*)""#),
            column_specifier: regex(r"^(?:(\d+)\*)?(?:[<^>](?:\.[<^>]?)?|\.[<^>])?(?:\d+%?|~)?$"),
            header_option: regex(r#"\b(?:options|opts)="[^"]*\bheader\b|%header\b"#),
            no_header_option: regex(r"\bnoheader\b"),
            footer_option: regex(r#"\b(?:options|opts)="[^"]*\bfooter\b|%footer\b"#),
        }
    }

    fn cells(&self, content: &[String]) -> Option<Vec<(usize, Cell)>> {
        let mut cells = Vec::<(usize, Cell)>::new();
        for (number, line) in content.iter().enumerate() {
            let mut rest = line.as_str();
            loop {
                let bar = rest
                    .match_indices('|')
                    .map(|(at, _)| at)
                    .find(|at| !rest[..*at].ends_with('\\'));
                let Some(at) = bar else {
                    match cells.last_mut() {
                        Some((_, cell)) => {
                            cell.text.push_str(rest);
                            cell.text.push('\n');
                        }
                        None if !rest.trim().is_empty() => return None,
                        None => {}
                    }
                    break;
                };
                let before = &rest[..at];
                let token_start = before
                    .rfind(char::is_whitespace)
                    .map_or(0, |space| space + 1);
                let (text, style) = match self.cell_specifier.captures(&before[token_start..]) {
                    Some(specifier) if specifier.get(1).is_some() => return None,
                    Some(specifier) => (
                        &before[..token_start],
                        specifier
                            .get(2)
                            .and_then(|style| style.as_str().chars().next()),
                    ),
                    None => (before, None),
                };
                match cells.last_mut() {
                    Some((_, cell)) => cell.text.push_str(text),
                    None if !text.trim().is_empty() => return None,
                    None => {}
                }
                cells.push((
                    number,
                    Cell {
                        style,
                        text: String::new(),
                    },
                ));
                rest = &rest[at + 1..];
            }
        }
        for (_, cell) in &mut cells {
            cell.text = cell.text.replace("\\|", "|").trim().to_string();
        }
        Some(cells)
    }

    fn read(
        &self,
        file: &str,
        page: Option<String>,
        above: &[String],
        content: &[String],
    ) -> Option<Table> {
        let title = above
            .iter()
            .find_map(|line| line.strip_prefix('.'))
            .map(|title| title.trim().to_string());
        let attributes = above
            .iter()
            .filter(|line| line.starts_with('[') && !line.starts_with("[["))
            .cloned()
            .collect::<Vec<_>>()
            .join(",");
        if self.footer_option.is_match(&attributes) {
            return None;
        }
        let content = content
            .iter()
            .filter(|line| !line.starts_with("//"))
            .cloned()
            .collect::<Vec<_>>();
        let cells = self.cells(&content)?;
        let first_line = cells.first()?.0;
        let columns = match self.columns.captures(&attributes) {
            Some(cols) => cols[1]
                .split(',')
                .map(|column| {
                    let specifier = self.column_specifier.captures(column.trim())?;
                    specifier
                        .get(1)
                        .map_or(Some(1), |count| count.as_str().parse::<usize>().ok())
                })
                .sum::<Option<usize>>()?,
            None => {
                let next_row = (first_line + 1..content.len())
                    .find(|line| self.cell_at_the_start.is_match(&content[*line]));
                cells
                    .iter()
                    .filter(|(line, _)| next_row.is_none_or(|next_row| *line < next_row))
                    .count()
            }
        };
        if columns < 2 || cells.len() % columns != 0 || cells.len() == columns {
            return None;
        }
        let is_blank = |line: &String| line.trim().is_empty();
        let after_the_gap = content.iter().skip(1).find(|line| !is_blank(line));
        let implicit_header = first_line == 0
            && content.get(1).is_some_and(is_blank)
            && after_the_gap.is_none_or(|line| self.cell_at_the_start.is_match(line));
        let has_header = !self.no_header_option.is_match(&attributes)
            && (self.header_option.is_match(&attributes) || implicit_header);
        if !has_header {
            return None;
        }
        let mut cells = cells.into_iter().map(|(_, cell)| cell);
        let header = cells.by_ref().take(columns).map(|cell| cell.text).collect();
        let cells = cells.collect::<Vec<_>>();
        let mut rows = Vec::new();
        let mut cells = cells.into_iter().peekable();
        while cells.peek().is_some() {
            rows.push(cells.by_ref().take(columns).collect());
        }
        Some(Table {
            file: file.to_string(),
            page,
            title,
            header,
            rows,
        })
    }
}

struct Specification {
    pages: BTreeMap<String, Page>,
    tables: Vec<Table>,
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
    unconstrained_role: Regex,
    role: Regex,
    macro_prefix: Regex,
    normative: Regex,
    hard_break: Regex,
    math_block: Regex,
    asciidoc_block: Regex,
    unsupported_macro: Regex,
    list_item: Regex,
    nested_item: Regex,
    other_list_item: Regex,
    term: Regex,
    note: Regex,
    mark: Regex,
    described_item: Regex,
    block_attributes: Regex,
    block_title: Regex,
    block_delimiter: Regex,
    admonition: Regex,
    block_of_the_page: Regex,
}

impl Markup {
    fn new() -> Self {
        Self {
            list_item: regex(r"^[ \t]*\*[ \t]+\S"),
            nested_item: regex(r"^[ \t]*\*{2,}[ \t]+\S"),
            other_list_item: regex(
                r"^[ \t]*(?:(?:-|\u{2022}|\.{1,5}|\d+\.|[a-zA-Z]\.|[IVXivx]+\))[ \t]+\S|[^\s/].*?(?::{2,4}|;;)(?:$|[ \t]+\S))",
            ),
            term: regex(r"^[ \t]*[^\s/].*?(?::{2,4}|;;)(?:$|[ \t]+(\S.*)$)"),
            note: regex(r"^[ \t]*(\d{1,9})::(?:$|[ \t]+(\S.*)$)"),
            mark: regex(r"\^(\d{1,9})\^"),
            described_item: regex(
                r"^[ \t]*\*[ \t]+((?:\[\[[^\]]*\]\]\s*)*)(pname:\w+(?:(?:,\s+(?:and\s+)?|\s+and\s+)pname:\w+)*|ename:\w+(?:(?:,\s+(?:and\s+)?|\s+and\s+)ename:\w+)*)\b",
            ),
            block_attributes: regex(r#"^\[(?:|[\w.#%{,"'].*)\]$"#),
            block_title: regex(r"^\.\.?[^ \t.]"),
            block_delimiter: regex(r"^(?:--|={4,}|-{4,}|\.{4,}|\*{4,}|_{4,}|\+{4,}|\|={3,})$"),
            admonition: regex(r"^\[(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]$"),
            block_of_the_page: regex(
                r"^include::\{(?:generated\}/(?:validity|api)|chapters\}/commonvalidity)/",
            ),
            anchor: regex(r"\[\[[^\]]*\]\]"),
            cross_reference: regex(r"(?s)<<([^,>]*)(?:,(.*?))?>>"),
            attribute_reference: regex(r"\{([\w-]+)\}"),
            labelled_url: regex(r"(?:link:)?(https?://[^\s\[]+)\[([^\]]*)\]"),
            unconstrained_role: regex(r"(?s)\[[\w.-]+\]##(.+?)##"),
            role: regex(r"(?s)\[[\w.-]+\]#(.+?)#"),
            macro_prefix: regex(
                r"\b(?:pname|ptext|sname|stext|slink|fname|ftext|flink|ename|etext|elink|dname|dlink|tname|tlink|basetype|code|apiext|reflink|tag|attr|latexmath):",
            ),
            normative: regex(
                r"\b(can|cannot|may|must|optional|optionally|required|should|undefined):",
            ),
            hard_break: regex(r"[ \t]\+[ \t]*(?:\n|$)"),
            math_block: regex(r"(?s)\[latexmath\]\s*\n\s*\+\+\+\+\s*\n(.*?)\n\s*\+\+\+\+"),
            asciidoc_block: regex(
                r"(?m)^\s*(?:[*.!-]|\d+\.|\[[^\[].*\]\s*$|include::|image::|={4,}|-{4,}|\+{4,})",
            ),
            unsupported_macro: regex(r"\b(?:footnote|image|xref|stem|anchor|pass):\S"),
        }
    }
}

struct Sources {
    root: PathBuf,
    defined: BTreeSet<String>,
    equivalent_flags: BTreeMap<String, Vec<String>>,
    conditional: Regex,
    end_conditional: Regex,
    tagged_include: Regex,
    equivalent_flags_include: Regex,
    tag_marker: Regex,
    attribute_entry: Regex,
    attribute_reference: Regex,
}

const COMMENT: &str = "//";
const ATTRIBUTE_ENTRY: &str = "//:";

fn xml_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(" {name}=\""))? + name.len() + 3;
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

fn equivalent_flags(registry: &str) -> BTreeMap<String, Vec<String>> {
    const BITMASKS: [&str; 2] = ["VkPipelineStageFlagBits2", "VkAccessFlagBits2"];
    let enum_tag = regex(r"<enum\s[^>]*>");
    let mut in_the_api = BTreeSet::new();
    for bitmask in regex(r"(?s)<enums\s[^>]*[^/>]>.*?</enums>").find_iter(registry) {
        let bitmask = bitmask.as_str();
        let name = xml_attribute(&bitmask[..bitmask.find('>').unwrap_or(0)], "name");
        if name.is_some_and(|name| BITMASKS.contains(&name)) {
            in_the_api.extend(
                enum_tag
                    .find_iter(bitmask)
                    .filter_map(|flag| xml_attribute(flag.as_str(), "name")),
            );
        }
    }
    let providers = regex(r"(?s)<(?:extension|feature)\s[^>]*[^/>]>.*?</(?:extension|feature)>");
    for provider in providers.find_iter(registry) {
        let provider = provider.as_str();
        let tag = &provider[..provider.find('>').unwrap_or(0)];
        let apis = xml_attribute(tag, "supported").or_else(|| xml_attribute(tag, "api"));
        if !apis.is_some_and(|apis| apis.split(',').any(|api| api == "vulkan")) {
            continue;
        }
        for flag in enum_tag.find_iter(provider) {
            let extends = xml_attribute(flag.as_str(), "extends");
            if extends.is_some_and(|extends| BITMASKS.contains(&extends)) {
                in_the_api.extend(xml_attribute(flag.as_str(), "name"));
            }
        }
    }
    let equivalent = regex(r#"<syncequivalent\s+(?:stage|access)="([^"]*)""#);
    let synchronized = regex(
        r#"(?s)<sync(?:stage|access)\s+name="(\w+)"(?:[^>]*[^/>])?>(.*?)</sync(?:stage|access)>"#,
    );
    let mut flags = BTreeMap::new();
    for sync in synchronized.captures_iter(registry) {
        if let Some(equivalent) = equivalent.captures(&sync[2]) {
            let listed = equivalent[1]
                .split(',')
                .filter(|flag| in_the_api.contains(flag))
                .map(str::to_string)
                .collect();
            flags.insert(sync[1].to_string(), listed);
        }
    }
    flags
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
            equivalent_flags: equivalent_flags(&registry),
            conditional: regex(r"^(ifdef|ifndef)::([^\[]*)\[(.*)\]\s*$"),
            end_conditional: regex(r"^endif::[^\[]*\[\]\s*$"),
            tagged_include: regex(r"^include::\{chapters\}/([^\[]+)\[tags?=([^\],;]+)\]\s*$"),
            equivalent_flags_include: regex(
                r"^include::\{generated\}/sync/flagDefinitions/(\w+)\.adoc\[\]\s*$",
            ),
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
            if !active || line.starts_with("////") {
                continue;
            }
            if line.starts_with("//") {
                out.push(COMMENT.to_string());
                continue;
            }
            if let Some(include) = self.tagged_include.captures(line) {
                let included = self.root.join("chapters").join(&include[1]);
                self.lines(&included, Some(&include[2]), locals, out);
                continue;
            }
            if let Some(include) = self.equivalent_flags_include.captures(line) {
                let flags = self.equivalent_flags.get(&include[1]).unwrap_or_else(|| {
                    panic!(
                        "{}: the registry names no flags that {} is equivalent to",
                        file.display(),
                        &include[1]
                    )
                });
                out.extend(flags.iter().map(|flag| format!("  ** ename:{flag}")));
                continue;
            }
            if let Some(entry) = self.attribute_entry.captures(line) {
                if entry[2].is_empty() {
                    let value = self.with_locals(&entry[3], locals);
                    locals.insert(entry[1].to_string(), value);
                } else {
                    locals.remove(&entry[1]);
                }
                out.push(ATTRIBUTE_ENTRY.to_string());
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
        let table_syntax = TableSyntax::new();

        let mut pages = BTreeMap::new();
        let mut tables = Vec::new();
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

            let relative = file
                .strip_prefix(root)
                .expect("a source of the specification")
                .display()
                .to_string();
            let mut lines = Vec::new();
            sources.lines(&file, None, &mut BTreeMap::new(), &mut lines);
            let mut blocks = Vec::<(usize, usize, String)>::new();
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
                blocks.push((index, end, attributes["refpage"].clone()));
                for name in names {
                    pages.entry(name.to_string()).or_insert_with(|| Page {
                        name: attributes["refpage"].clone(),
                        file: relative.clone(),
                        description: description.clone(),
                        body: body.clone(),
                        body_words: String::new(),
                        valid_usage: OnceCell::new(),
                    });
                }
                index = end + 1;
            }

            let mut index = 0;
            while index < lines.len() {
                if !table_syntax.delimiter.is_match(&lines[index]) {
                    index += 1;
                    continue;
                }
                let delimiter = lines[index].trim_end();
                let close = (index + 1..lines.len())
                    .find(|line| lines[*line].trim_end() == delimiter)
                    .unwrap_or(lines.len());
                let mut start = index;
                while start > 0
                    && (table_syntax.block_header.is_match(&lines[start - 1])
                        || lines[start - 1] == COMMENT)
                {
                    start -= 1;
                }
                let page = blocks
                    .iter()
                    .find(|(first, last, _)| *first < index && index < *last)
                    .map(|(_, _, name)| name.clone());
                if let Some(table) = table_syntax.read(
                    &relative,
                    page,
                    &lines[start..index],
                    &lines[index + 1..close],
                ) {
                    tables.push(table);
                }
                index = close + 1;
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
            tables,
            extensions,
            attributes,
            titles,
            markup: Markup::new(),
        };
        for page in pages.values_mut() {
            page.body = specification.with_notes_at_their_marks(&page.body);
            page.body_words = specification.words(&page.body);
        }
        specification.pages = pages;
        specification
    }

    fn words(&self, text: &str) -> String {
        words(&self.plain(&self.without_block_markup(text)))
    }

    fn without_block_markup(&self, text: &str) -> String {
        let mut out = String::with_capacity(text.len());
        for line in text.lines() {
            let at_margin = line.trim_end();
            if let Some(admonition) = self.markup.admonition.captures(at_margin) {
                let label = &admonition[1];
                out.push_str(&label[..1]);
                out.push_str(&label[1..].to_ascii_lowercase());
            } else if !self.markup.block_attributes.is_match(at_margin) {
                out.push_str(line);
            }
            out.push('\n');
        }
        out
    }

    fn item_end(&self, lines: &[&str], start: usize) -> usize {
        let markup = &self.markup;
        let lacks_a_description = |line: &str| {
            markup
                .term
                .captures(line)
                .is_some_and(|term| term.get(1).is_none())
        };
        let mut index = start + 1;
        let mut attached = false;
        let mut inside = None;
        while index < lines.len() {
            let line = lines[index].trim_end();
            if let Some(delimiter) = inside {
                if line == delimiter {
                    inside = None;
                }
            } else if line == COMMENT {
                attached = false;
            } else if line.is_empty() {
                let after = lines[index..]
                    .iter()
                    .position(|line| !line.trim().is_empty())
                    .map(|skipped| (skipped, lines[index + skipped].trim_end()));
                let goes_on = match after {
                    None => false,
                    Some((_, COMMENT)) => lacks_a_description(lines[index - 1]),
                    Some((1, after)) if attached => !markup.list_item.is_match(after),
                    Some((_, after)) => {
                        after == "+"
                            || markup.nested_item.is_match(after)
                            || markup.other_list_item.is_match(after)
                            || (lacks_a_description(lines[index - 1])
                                && !markup.list_item.is_match(after))
                    }
                };
                if !goes_on {
                    break;
                }
                let (skipped, _) = after.expect("a line after the empty ones");
                attached = attached && skipped == 1;
                index += skipped;
                continue;
            } else if markup.list_item.is_match(line) || markup.block_of_the_page.is_match(line) {
                break;
            } else if line == "+" {
                attached = true;
            } else if markup.block_delimiter.is_match(line) {
                if !attached {
                    break;
                }
                inside = Some(line);
                attached = false;
            } else if markup.block_attributes.is_match(line)
                || markup.block_title.is_match(line)
                || line == ATTRIBUTE_ENTRY
            {
                if !attached {
                    break;
                }
            } else {
                attached = false;
            }
            index += 1;
        }
        index
    }

    fn with_notes_at_their_marks(&self, body: &str) -> String {
        let markup = &self.markup;
        let lines = body.lines().collect::<Vec<_>>();
        let mut items = Vec::new();
        let mut notes = BTreeMap::<u32, Vec<&str>>::new();
        let mut index = 0;
        while index < lines.len() {
            let start = index;
            index += 1;
            if !markup.list_item.is_match(lines[start]) {
                continue;
            }
            index = self.item_end(&lines, start);
            let text_end = (start + 1..index)
                .find(|line| markup.note.is_match(lines[*line]))
                .unwrap_or(index);
            let mut number = 0;
            for line in &lines[text_end..index] {
                let text = match markup.note.captures(line) {
                    Some(note) => {
                        number = note[1].parse().expect("the number of a note");
                        note.get(2).map_or("", |text| text.as_str())
                    }
                    None => *line,
                };
                if !text.trim().is_empty() {
                    notes.entry(number).or_default().push(text);
                }
            }
            items.push((start, text_end, index));
        }
        if notes.is_empty() {
            return body.to_string();
        }
        let mut out = Vec::<String>::new();
        let mut copied = 0;
        for (start, text_end, end) in items {
            let text = &lines[start..text_end];
            let empty_lines = text
                .iter()
                .rev()
                .take_while(|line| line.trim().is_empty())
                .count();
            let (text, after_the_text) = text.split_at(text.len() - empty_lines);
            out.extend(lines[copied..start].iter().map(|line| line.to_string()));
            let mut marks = BTreeSet::<u32>::new();
            for line in text {
                let referred = markup.mark.replace_all(line, |mark: &Captures<'_>| {
                    let before = &line[..mark.get(0).expect("the mark").start()];
                    let is_an_exponent = before.ends_with(|c: char| c.is_ascii_digit());
                    let number = mark[1].parse::<u32>().ok().filter(|_| !is_an_exponent);
                    match number.filter(|number| notes.contains_key(number)) {
                        Some(number) => {
                            marks.insert(number);
                            format!(" (note {number})")
                        }
                        None => mark[0].to_string(),
                    }
                });
                out.push(referred.into_owned());
            }
            for number in marks {
                out.push("+".to_string());
                out.push(format!("Note {number}"));
                out.extend(notes[&number].iter().map(|line| line.to_string()));
            }
            out.extend(after_the_text.iter().map(|line| line.to_string()));
            copied = end;
        }
        out.extend(lines[copied..].iter().map(|line| line.to_string()));
        out.join("\n")
    }

    fn items(&self, page: &str) -> Vec<(String, Vec<String>, String)> {
        let Some(page) = self.pages.get(page) else {
            return Vec::new();
        };
        let markup = &self.markup;
        let lines = page.body.lines().collect::<Vec<_>>();
        let mut items = Vec::new();
        let mut index = 0;
        while index < lines.len() {
            let start = index;
            index += 1;
            if !markup.list_item.is_match(lines[start]) {
                continue;
            }
            index = self.item_end(&lines, start);
            let text = lines[start..index].join("\n");
            if let Some(described) = markup.described_item.captures(&text)
                && !described[1].contains("[[VUID-")
            {
                let names = described[2]
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == ':'))
                    .filter_map(|name| name.split_once(':'))
                    .collect::<Vec<_>>();
                items.push((
                    names[0].0.to_string(),
                    names.iter().map(|(_, name)| name.to_string()).collect(),
                    text,
                ));
            }
        }
        items
    }

    fn item(&self, page: &str, macro_name: &str, name: &str) -> Option<String> {
        let items = self
            .items(page)
            .into_iter()
            .filter(|(item_macro, _, _)| item_macro == macro_name)
            .collect::<Vec<_>>();
        items
            .iter()
            .find(|(_, names, _)| names[0] == name)
            .or_else(|| {
                items
                    .iter()
                    .find(|(_, names, _)| names.iter().any(|n| n == name))
            })
            .map(|(_, _, text)| text.clone())
    }

    fn visible(&self, text: &str) -> String {
        let text = self.markup.hard_break.replace_all(text, "\n");
        let text = self.plain(&text);
        let text = self.markup.normative.replace_all(&text, "$1");
        marks_removed(&text)
    }

    fn cell_is_readable(&self, cell: &Cell) -> bool {
        let text = match cell.style {
            None | Some('d') => cell.text.clone(),
            Some('a') => {
                let text = self.markup.math_block.replace_all(&cell.text, "math");
                if self.markup.asciidoc_block.is_match(&text) {
                    return false;
                }
                text.into_owned()
            }
            Some(_) => return false,
        };
        let known_attributes = self
            .markup
            .attribute_reference
            .captures_iter(&text)
            .all(|reference| self.attributes.contains_key(&reference[1]));
        let one_paragraph = !text.lines().any(|line| {
            let line = line.trim();
            line.is_empty() || line == "+"
        });
        known_attributes && one_paragraph && !self.markup.unsupported_macro.is_match(&text)
    }

    fn cell_matches(&self, generated: &Generated, cell: &str, value: &str) -> bool {
        let Some(math) = self.markup.math_block.captures(cell) else {
            return self.visible(cell) == generated.visible(value);
        };
        let rest = self.markup.math_block.replace(cell, "");
        let squeezed = |text: &str| text.split_whitespace().collect::<String>();
        rest.trim().is_empty()
            && generated
                .code_span
                .captures(value.trim())
                .is_some_and(|code| squeezed(&code[1]) == squeezed(&math[1]))
    }

    fn plain(&self, text: &str) -> String {
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
        let text = markup.unconstrained_role.replace_all(&text, "$1");
        let text = markup.role.replace_all(&text, "$1");
        markup.macro_prefix.replace_all(&text, "").into_owned()
    }

    fn table_rows(
        &self,
        page: Option<&str>,
        values: &BTreeSet<String>,
        value: &str,
    ) -> Vec<(&Table, &[Cell])> {
        let Some(page) = page.and_then(|page| self.pages.get(page)) else {
            return Vec::new();
        };
        let names = |row: &[Cell], value: &str| row[0].text.strip_prefix("ename:") == Some(value);
        let about_the_type = |table: &Table| {
            table.title.is_some()
                && table.rows.iter().all(|row| {
                    row[0]
                        .text
                        .strip_prefix("ename:")
                        .is_some_and(|name| values.contains(name))
                })
        };
        let rows_in = |outside: bool| {
            self.tables
                .iter()
                .filter(|table| match &table.page {
                    Some(table_page) => !outside && *table_page == page.name,
                    None => outside && table.file == page.file && about_the_type(table),
                })
                .flat_map(|table| table.rows.iter().map(move |row| (table, row.as_slice())))
                .filter(|(_, row)| names(row, value))
                .collect::<Vec<_>>()
        };
        let mut rows = rows_in(false);
        if rows.is_empty() {
            rows = rows_in(true);
        }
        rows.retain(|(_, row)| {
            row[1..].iter().any(|cell| !cell.text.is_empty())
                && row.iter().all(|cell| self.cell_is_readable(cell))
        });
        rows
    }

    fn describes(&self, page: &str, macro_name: &str, item: &str) -> bool {
        self.described(page, macro_name)
            .iter()
            .any(|described| described == item)
    }

    fn described(&self, page: &str, macro_name: &str) -> Vec<String> {
        let bullet = regex(&format!(
            r"(?m)^[ \t]*\*[ \t]+((?:\[\[[^\]]*\]\]\s*)*)({macro_name}:\w+(?:(?:,\s+(?:and\s+)?|\s+and\s+){macro_name}:\w+)*)"
        ));
        let name = regex(&format!(r"{macro_name}:(\w+)"));
        self.pages.get(page).map_or_else(Vec::new, |page| {
            bullet
                .captures_iter(&page.body)
                .filter(|bullet| !bullet[1].contains("[[VUID-"))
                .flat_map(|bullet| {
                    name.captures_iter(&bullet[2])
                        .map(|name| name[1].to_string())
                        .collect::<Vec<_>>()
                })
                .collect()
        })
    }

    fn valid_usage(&self, page: &Page) -> Vec<String> {
        let statement = regex(r"^[ \t]*\*[ \t]+\[\[VUID-[^\]]*\]\]\s*(.*)$");
        let ends = regex(r"^[ \t]*(?:\*+[ \t]|\*{4,}$|$)");
        let mut statements = Vec::<String>::new();
        let mut open = false;
        for line in page.body.lines() {
            if let Some(start) = statement.captures(line) {
                statements.push(start[1].to_string());
                open = true;
            } else if ends.is_match(line) {
                open = false;
            } else if open && let Some(last) = statements.last_mut() {
                last.push('\n');
                last.push_str(line);
            }
        }
        statements
            .iter()
            .map(|statement| self.words(statement))
            .collect()
    }

    fn states_valid_usage(&self, page: &str, markdown: &str) -> bool {
        let Some(page) = self.pages.get(page) else {
            return false;
        };
        let stated = generated_words(markdown);
        let several_words = |words: &str| words.trim().contains(' ');
        several_words(&stated)
            && page
                .valid_usage
                .get_or_init(|| self.valid_usage(page))
                .iter()
                .any(|statement| {
                    several_words(statement)
                        && (stated.starts_with(statement) || statement.starts_with(&stated))
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

fn without_code_fences(markdown: &str) -> String {
    let mut pieces = markdown.split("```");
    let mut text = pieces.next().unwrap_or_default().to_string();
    for piece in pieces {
        text.push(' ');
        text.push_str(piece.trim_start_matches(|c: char| !c.is_whitespace()));
    }
    text
}

const WORDS_THAT_DO_NOT_END_A_SENTENCE: &[&str] = &[
    "a",
    "an",
    "and",
    "are",
    "as",
    "at",
    "be",
    "by",
    "either",
    "following",
    "for",
    "from",
    "in",
    "is",
    "of",
    "or",
    "than",
    "the",
    "to",
    "with",
];

fn ends_in_the_middle_of_a_sentence(doc: &str) -> bool {
    let doc = doc.trim_end();
    let last_word = doc
        .rsplit(char::is_whitespace)
        .next()
        .unwrap_or_default()
        .trim_matches(['*', '_'])
        .to_ascii_lowercase();
    doc.ends_with([':', ',']) || WORDS_THAT_DO_NOT_END_A_SENTENCE.contains(&last_word.as_str())
}

fn words_left_out(expected: &str, actual: &str) -> String {
    let expected = expected.split_whitespace().collect::<Vec<_>>();
    let kept = (1..=expected.len())
        .take_while(|end| actual.contains(&format!(" {} ", expected[..*end].join(" "))))
        .last()
        .unwrap_or(0);
    let from = kept.saturating_sub(4);
    let to = expected.len().min(kept + 12);
    format!(
        "{}[{}]",
        expected[from..kept]
            .iter()
            .map(|word| format!("{word} "))
            .collect::<String>(),
        expected[kept..to].join(" ")
    )
}

fn generated_words(markdown: &str) -> String {
    let markdown = &without_code_fences(markdown);
    let mut text = String::with_capacity(markdown.len());
    let mut rest = markdown.as_str();
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

fn is_rust_name_of(rust_name: &str, c_name: &str) -> bool {
    let rust_name = rust_name.strip_prefix("r#").unwrap_or(rust_name);
    squeeze(rust_name) == squeeze(c_name) || (rust_name == "ty" && c_name == "type")
}

fn declared_name(declaration: &str) -> &str {
    let declaration = declaration.split(['`', '[']).next().unwrap_or_default();
    let declarator = declaration.split(" : ").next().unwrap_or_default();
    declarator.rsplit(' ').next().unwrap_or_default()
}

fn list_items(list: &str) -> Vec<(usize, String)> {
    let mut items = Vec::<(usize, String)>::new();
    for line in list.lines() {
        let text = line.trim_start();
        let indent = line.len() - text.len();
        if let Some(item) = text.strip_prefix("- ") {
            items.push((indent / 2, item.to_string()));
        } else if let Some((_, last)) = items.last_mut() {
            last.push(' ');
            last.push_str(text);
        }
    }
    items
}

fn labelled_cells(items: &[(usize, String)]) -> Option<Vec<(String, Vec<String>)>> {
    let mut cells = Vec::<(String, Vec<String>)>::new();
    for (level, text) in items {
        match level {
            0 => match text.strip_suffix(':') {
                Some(label) => cells.push((label.to_string(), Vec::new())),
                None => cells.push((text.clone(), Vec::new())),
            },
            1 => cells.last_mut()?.1.push(text.clone()),
            _ => return None,
        }
    }
    Some(cells)
}

fn marks_removed(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_whitespace() && !"#~^*`\"'“”‘’".contains(*c))
        .collect()
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

#[derive(Clone, Copy)]
struct Owner<'a> {
    page: Option<&'a str>,
    values: &'a BTreeSet<String>,
}

#[derive(Default)]
struct Checker {
    failures: Vec<String>,
    reference_links: usize,
    summaries: usize,
    described_members: usize,
    undescribed_members: usize,
    described_values: usize,
    tabulated_values: usize,
    undescribed_values: usize,
    described_parameters: usize,
    functions_with_arguments: usize,
    required_parameters: usize,
    parameters_under_another_name: usize,
    undescribed_arguments: Vec<String>,
    whole_items: usize,
    ends_where_its_list_item_ends: Vec<String>,
    members_under_another_name: usize,
    descriptions_of_no_member: Vec<String>,
}

struct Generated {
    reference_link: Regex,
    names_at_start: Regex,
    name: Regex,
    described_under_another_name: Regex,
    value_of: Regex,
    bits_of: Regex,
    table_row: Regex,
    link: Regex,
    html_tag: Regex,
    escape: Regex,
    code_span: Regex,
}

impl Generated {
    fn new() -> Self {
        Self {
            reference_link: regex(&format!(
                r"\[`([\w-]+)`\]\({}([\w-]+)\.html\)",
                regex::escape(REFERENCE_PAGES)
            )),
            names_at_start: regex(
                r"^(?:\[`\w+`\]\([^)\s]*\)|`\w+`)(?:(?:,\s+(?:and\s+)?|\s+and\s+)(?:\[`\w+`\]\([^)\s]*\)|`\w+`))*",
            ),
            name: regex(r"`(\w+)`"),
            described_under_another_name: regex(
                r"^`(\w+)`,\s+which\s+the\s+specification\s+describes\s+under\s+the\s+name\s+`(\w+)`:\s+",
            ),
            value_of: regex(r"^`\w+`, a value of (?:\[`(\w+)`\]|`(\w+)`)"),
            bits_of: regex(r"constants of \[`(\w+)`\]"),
            table_row: regex(r"^`(\w+)` in (?:the table “(.+)”|a table) of the specification:$"),
            link: regex(r"\[([^\]]*)\]\([^)\s]*\)"),
            html_tag: regex(r"</?(?:sub|sup)>"),
            escape: regex(r"\\(.)"),
            code_span: regex(r"^`([^`]+)`$"),
        }
    }

    fn visible(&self, markdown: &str) -> String {
        let text = self.link.replace_all(markdown, "$1");
        let text = self.html_tag.replace_all(&text, "");
        let text = self.escape.replace_all(&text, "$1");
        marks_removed(&text)
    }

    fn page_of(&self, doc: &str) -> Option<String> {
        self.reference_link
            .captures(doc)
            .map(|link| link[2].to_string())
    }

    fn names_at_start<'a>(&self, description: &'a str) -> Vec<&'a str> {
        let Some(names) = self.names_at_start.find(description) else {
            return Vec::new();
        };
        self.name
            .captures_iter(names.as_str())
            .map(|name| name.get(1).map_or("", |name| name.as_str()))
            .collect()
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

    fn whole(
        &mut self,
        spec: &Specification,
        item: &str,
        page: &str,
        (macro_name, name): (&str, &str),
        doc: &str,
    ) {
        let description = format!("{item}: `{name}`");
        let in_the_middle = ends_in_the_middle_of_a_sentence(doc);
        if in_the_middle && ENDS_WHERE_ITS_LIST_ITEM_ENDS.contains(&description.as_str()) {
            self.ends_where_its_list_item_ends.push(description);
        } else if in_the_middle {
            let end = doc.trim_end();
            let tail = end
                .char_indices()
                .rev()
                .nth(59)
                .map_or(end, |(at, _)| &end[at..]);
            self.fail(
                item,
                format!(
                    "the description of `{name}` ends in the middle of a sentence: `…{}`",
                    flat(tail)
                ),
            );
        }
        let Some(source) = spec.item(page, macro_name, name) else {
            return;
        };
        self.whole_items += 1;
        let expected = spec.words(&source);
        let actual = generated_words(doc);
        if !actual.contains(&expected) {
            self.fail(
                item,
                format!(
                    "the description of `{name}` leaves out a part of its list item on the page `{page}`: `{}`",
                    words_left_out(&expected, &actual)
                ),
            );
        }
    }

    fn undescribed(
        &mut self,
        spec: &Specification,
        item: &str,
        page: &str,
        (macro_name, name): (&str, &str),
    ) {
        if !spec.describes(page, macro_name, name) {
            return;
        }
        self.fail(item, format!("the specification describes `{name}` on the page `{page}`, but the documentation does not use it"));
    }

    fn members<'a>(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        owner_page: Option<&str>,
        owner: &str,
        fields: impl IntoIterator<Item = &'a syn::Field>,
    ) {
        let mut c_names = BTreeSet::new();
        for field in fields {
            let name = field.ident.as_ref().expect("named field").to_string();
            let (doc, _) = doc_attributes(&field.attrs);
            self.links(spec, generated, owner, &doc);
            self.member(spec, generated, owner_page, owner, &name, doc.trim());
            let doc = flat(&doc);
            if let Some(storage) = doc.strip_prefix("The storage unit of the C bit-fields ") {
                let listed = storage.split('.').next().unwrap_or_default();
                c_names.extend(
                    generated
                        .name
                        .captures_iter(listed)
                        .map(|name| name[1].to_string()),
                );
            } else if let Some(declaration) = doc.strip_prefix("The C member `") {
                c_names.insert(declared_name(declaration).to_string());
            } else if let Some(names) = generated.described_under_another_name.captures(&doc) {
                c_names.extend([names[1].to_string(), names[2].to_string()]);
            } else {
                let names = generated.names_at_start(&doc);
                let own = names
                    .iter()
                    .find(|c_name| is_rust_name_of(&name, c_name))
                    .or(names.first());
                c_names.extend(own.map(|c_name| c_name.to_string()));
            }
        }
        let (Some(page), false) = (owner_page, c_names.is_empty()) else {
            return;
        };
        let described = spec
            .described(page, "pname")
            .into_iter()
            .collect::<BTreeSet<_>>();
        for name in described.difference(&c_names) {
            self.descriptions_of_no_member
                .push(format!("{page}::{name}"));
        }
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
            let name = declared_name(declaration);
            if let Some(page) = owner_page {
                self.undescribed(spec, &item, page, ("pname", name));
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
        let names = generated.names_at_start(doc);
        let Some(first) = names.first() else {
            self.fail(&item, "does not start with the name of the member");
            return;
        };
        let name = names
            .iter()
            .find(|name| is_rust_name_of(field, name))
            .unwrap_or_else(|| {
                self.fail(
                    &item,
                    format!("is documented with the description of `{first}`"),
                );
                first
            });
        let Some(names) = generated.described_under_another_name.captures(doc) else {
            self.contained(spec, &item, page, doc);
            self.whole(spec, &item, page, ("pname", name), doc);
            self.not_valid_usage(spec, &item, page, doc);
            return;
        };
        self.members_under_another_name += 1;
        let (own, other) = (&names[1], &names[2]);
        let description = &doc[names[0].len()..];
        if spec.describes(page, "pname", own) {
            self.fail(&item, format!("the specification describes the member `{own}` on the page `{page}` under its own name, but the documentation takes the description of `{other}` for it"));
        }
        if !spec.describes(page, "pname", other) {
            self.fail(&item, format!("the documentation takes the description of `{other}` for the member `{own}`, but the specification describes no `{other}` on the page `{page}`"));
        }
        if !description.starts_with(&format!("`{other}`")) {
            self.fail(&item, format!("the description that the documentation takes for the member `{own}` is not the one of `{other}`"));
        }
        self.contained(spec, &item, page, description);
        self.whole(spec, &item, page, ("pname", other), description);
    }

    fn not_valid_usage(&mut self, spec: &Specification, item: &str, page: &str, doc: &str) {
        if spec.states_valid_usage(page, doc) {
            self.fail(
                item,
                format!(
                    "is documented with valid usage of the page `{page}`, which states a requirement and does not describe the item: `{}`",
                    flat(doc)
                ),
            );
        }
    }

    fn value(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        owner: Owner<'_>,
        item: &str,
        doc: &str,
        aliases: &[String],
    ) {
        let Owner {
            page: owner_page,
            values,
        } = owner;
        let raw = doc;
        let doc = &flat(doc);
        let [c_name] = aliases else {
            return;
        };
        if !doc.starts_with(&format!("`{c_name}`")) {
            let names = generated.names_at_start(doc);
            match (names.contains(&c_name.as_str()), owner_page) {
                (true, Some(page)) => {
                    self.described_values += 1;
                    self.contained(spec, item, page, doc);
                    self.whole(spec, item, page, ("ename", c_name), doc);
                    self.not_valid_usage(spec, item, page, doc);
                }
                (true, None) => {
                    self.fail(item, "is described although its type has no reference page")
                }
                (false, _) => self.fail(item, format!("does not start with its C name `{c_name}`")),
            }
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
            self.undescribed(spec, item, page, ("ename", c_name));
            if !spec.table_rows(Some(page), values, c_name).is_empty() {
                self.fail(item, format!("the specification describes `{c_name}` in a table row, but the documentation does not use it"));
            }
            return;
        }
        self.described_values += 1;
        if rest.starts_with(" in the table “")
            || rest.starts_with(" in a table of the specification:")
        {
            self.tabulated(spec, generated, owner, item, c_name, raw);
            return;
        }
        match owner_page {
            Some(page) => {
                self.contained(spec, item, page, doc);
                self.whole(spec, item, page, ("ename", c_name), doc);
                self.not_valid_usage(spec, item, page, doc);
            }
            None => self.fail(item, "is described although its type has no reference page"),
        }
    }

    fn row_is_rendered(
        spec: &Specification,
        generated: &Generated,
        table: &Table,
        row: &[Cell],
        list: &str,
    ) -> bool {
        let Some(cells) = labelled_cells(&list_items(list)) else {
            return false;
        };
        let filled = row[1..].iter().filter(|cell| !cell.text.is_empty()).count();
        cells.len() == filled
            && cells.iter().all(|(text, lines)| {
                (1..row.len()).any(|column| {
                    let label = spec.visible(&table.header[column]);
                    let matches =
                        |value: &str| spec.cell_matches(generated, &row[column].text, value);
                    if !lines.is_empty() {
                        return generated.visible(text) == label && matches(&lines.join(" "));
                    }
                    text.match_indices(": ").any(|(at, _)| {
                        generated.visible(&text[..at]) == label && matches(&text[at + 2..])
                    })
                })
            })
    }

    fn tabulated(
        &mut self,
        spec: &Specification,
        generated: &Generated,
        owner: Owner<'_>,
        item: &str,
        c_name: &str,
        doc: &str,
    ) {
        let rows = spec.table_rows(owner.page, owner.values, c_name);
        let paragraphs = doc.trim().split("\n\n").collect::<Vec<_>>();
        if paragraphs.len() % 2 != 0 {
            self.fail(item, "has a table row without its cells");
            return;
        }
        for pair in paragraphs.chunks(2) {
            let lead = flat(pair[0]);
            let Some(captures) = generated.table_row.captures(&lead) else {
                self.fail(item, format!("`{lead}` does not name a table row"));
                return;
            };
            if &captures[1] != c_name {
                self.fail(
                    item,
                    format!("is documented with the table row of `{}`", &captures[1]),
                );
                return;
            }
            let title = captures
                .get(2)
                .map(|title| generated.visible(title.as_str()));
            let found = rows.iter().any(|(table, row)| {
                let same_title = match (&title, &table.title) {
                    (Some(generated), Some(source)) => *generated == spec.visible(source),
                    (None, None) => true,
                    _ => false,
                };
                same_title && Self::row_is_rendered(spec, generated, table, row, pair[1])
            });
            if !found {
                self.fail(
                    item,
                    format!(
                        "`{}` is not a row that the specification has for `{c_name}`",
                        flat(&pair.join(" "))
                    ),
                );
                return;
            }
        }
        self.tabulated_values += 1;
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
            let Some(names) = generated.described_under_another_name.captures(parameter) else {
                self.contained(spec, item, &page, parameter);
                if let Some(name) = generated.names_at_start(parameter).first() {
                    self.whole(spec, item, &page, ("pname", name), parameter);
                }
                self.not_valid_usage(spec, item, &page, parameter);
                continue;
            };
            self.parameters_under_another_name += 1;
            let (own, other) = (&names[1], &names[2]);
            let description = &parameter[names[0].len()..];
            if spec.describes(&page, "pname", own) {
                self.fail(item, format!("the specification describes the parameter `{own}` on the page `{page}` under its own name, but the documentation takes the description of `{other}` for it"));
            }
            if !spec.describes(&page, "pname", other) {
                self.fail(item, format!("the documentation takes the description of `{other}` for the parameter `{own}`, but the specification describes no `{other}` on the page `{page}`"));
            }
            if !description.starts_with(&format!("`{other}`")) {
                self.fail(item, format!("the description that the documentation takes for the parameter `{own}` is not the one of `{other}`"));
            }
            self.contained(spec, item, &page, description);
            self.whole(spec, item, &page, ("pname", other), description);
        }
        if arguments.is_empty() {
            return;
        }
        self.functions_with_arguments += 1;
        let listed = parameters
            .iter()
            .flat_map(|parameter| generated.names_at_start(parameter))
            .map(squeeze)
            .collect::<BTreeSet<_>>();
        let described = spec.described(&page, "pname");
        if !described.is_empty() {
            for argument in arguments {
                let plain = argument.strip_prefix("r#").unwrap_or(argument);
                let is_listed =
                    listed.contains(&squeeze(plain)) || (plain == "ty" && listed.contains("type"));
                if !is_listed {
                    self.undescribed_arguments
                        .push(format!("{item}: {argument}"));
                }
            }
        }
        for name in described {
            if !arguments
                .iter()
                .any(|argument| is_rust_name_of(argument, &name))
            {
                continue;
            }
            self.required_parameters += 1;
            if !listed.contains(&squeeze(&name)) {
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

const ENDS_WHERE_ITS_LIST_ITEM_ENDS: &[&str] = &[
    "Device::get_queue_checkpoint_data2_nv_raw: `queue`",
    "Device::get_queue_checkpoint_data_nv_raw: `queue`",
    "Device::get_shader_instrumentation_values_arm_raw: `instrumentation`",
    "ExecutionGraphPipelineCreateInfoAMDX::base_pipeline_handle: `basePipelineHandle`",
    "ExecutionGraphPipelineCreateInfoAMDX::base_pipeline_index: `basePipelineIndex`",
    "PhysicalDeviceExtendedFlagsFeaturesKHR::extended_flags: `extendedFlags`",
    "SetDescriptorBufferOffsetsInfoEXT::stage_flags: `stageFlags`",
    "TimeDomainKHR::CLOCK_MONOTONIC: `VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR`",
    "TimeDomainKHR::CLOCK_MONOTONIC_RAW: `VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR`",
    "TimeDomainKHR::QUERY_PERFORMANCE_COUNTER: `VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR`",
];

const DESCRIPTIONS_OF_NO_MEMBER: &[&str] = &[
    "VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV::mask",
    "VkDeviceAddressBindingCallbackDataEXT::messageSeverity",
    "VkDeviceAddressBindingCallbackDataEXT::messageTypes",
    "VkSubmitInfo::dstAccessMask",
    "VkSubmitInfo::dstStageMask",
    "VkSubmitInfo::newLayout",
    "VkSubmitInfo::oldLayout",
    "VkSubmitInfo::srcAccessMask",
    "VkSubmitInfo::srcStageMask",
    "VkSurfaceCapabilities2KHR::currentExtent",
    "VkSurfaceCapabilities2KHR::currentTransform",
    "VkSurfaceCapabilities2KHR::maxImageCount",
    "VkSurfaceCapabilities2KHR::minImageCount",
    "VkVideoEncodeH264PictureInfoKHR::pStdSliceHeader",
    "VkVideoEncodeH265PictureInfoKHR::pStdSliceSegmentHeader",
];

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
        "checked {} descriptions against the whole of their list item, {} reference links, {} summaries, {} described ({} under another name) and {} undescribed members, {} described ({} by a table row) and {} undescribed values, {} parameters ({} under another name), the parameter lists of {} functions with {} parameters that the specification describes",
        checker.whole_items,
        checker.reference_links,
        checker.summaries,
        checker.described_members,
        checker.members_under_another_name,
        checker.undescribed_members,
        checker.described_values,
        checker.tabulated_values,
        checker.undescribed_values,
        checker.described_parameters,
        checker.parameters_under_another_name,
        checker.functions_with_arguments,
        checker.required_parameters,
    );
    assert_eq!(
        checker.undescribed_arguments,
        Vec::<String>::new(),
        "the reference pages of these functions describe parameters, but the documentation describes none for these arguments"
    );
    assert!(
        checker.failures.is_empty(),
        "{} generated doc comments do not match the specification:\n{}",
        checker.failures.len(),
        checker.failures.join("\n")
    );
    assert_eq!(
        checker
            .ends_where_its_list_item_ends
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        ENDS_WHERE_ITS_LIST_ITEM_ENDS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these descriptions end like the middle of a sentence, because their list item in the specification ends there"
    );
    assert_eq!(
        checker
            .descriptions_of_no_member
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        DESCRIPTIONS_OF_NO_MEMBER
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these list items on the reference pages of structures start with a name that no member has, and they are not descriptions of members. An item that describes a member under another name belongs in DESCRIBED_UNDER_ANOTHER_NAME of vulkan-codegen"
    );
    for (what, count, at_least) in [
        (
            "descriptions with the whole of their list item",
            checker.whole_items,
            9000,
        ),
        ("reference links", checker.reference_links, 5000),
        ("summaries", checker.summaries, 2500),
        ("described members", checker.described_members, 5000),
        ("described values", checker.described_values, 2500),
        (
            "values described by a table row",
            checker.tabulated_values,
            150,
        ),
        ("parameters", checker.described_parameters, 2000),
        ("parameter lists", checker.functions_with_arguments, 800),
        (
            "parameters that the specification describes",
            checker.required_parameters,
            2000,
        ),
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
    let mut type_values = BTreeMap::<String, BTreeSet<String>>::new();
    for item in files.iter().flat_map(|file| &file.items) {
        if let Item::Impl(item) = item {
            let values = type_values.entry(type_name(&item.self_ty)).or_default();
            for member in &item.items {
                if let ImplItem::Const(constant) = member {
                    values.extend(doc_attributes(&constant.attrs).1);
                }
            }
        }
    }
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
        check_item(spec, generated, checker, &value_pages, &type_values, item);
    }
}

fn check_item(
    spec: &Specification,
    generated: &Generated,
    checker: &mut Checker,
    value_pages: &BTreeMap<String, String>,
    type_values: &BTreeMap<String, BTreeSet<String>>,
    item: &Item,
) {
    match item {
        Item::Struct(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
            let page = generated.page_of(&doc);
            let fields = item.fields.iter().filter(|field| field.ident.is_some());
            checker.members(spec, generated, page.as_deref(), &name, fields);
        }
        Item::Union(item) => {
            let name = item.ident.to_string();
            let (doc, aliases) = doc_attributes(&item.attrs);
            checker.links(spec, generated, &name, &doc);
            checker.summary(spec, &name, &doc, &aliases);
            let page = generated.page_of(&doc);
            checker.members(spec, generated, page.as_deref(), &name, &item.fields.named);
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
                check_item(spec, generated, checker, value_pages, type_values, nested);
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
                            Owner {
                                page: value_pages.get(&owner).map(String::as_str),
                                values: type_values.get(&owner).unwrap_or(&BTreeSet::new()),
                            },
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

fn fixture_table(
    file: &str,
    page: Option<&str>,
    above: &[&str],
    content: &[&str],
) -> Option<Table> {
    let lines = |lines: &[&str]| {
        lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
    };
    TableSyntax::new().read(
        file,
        page.map(str::to_string),
        &lines(above),
        &lines(content),
    )
}

const FIXTURE_FILE: &str = "chapters/things.adoc";

fn fixture_tables() -> Vec<Table> {
    let header = ["[options=\"header\"]"];
    [
        fixture_table(
            FIXTURE_FILE,
            Some("VkColor"),
            &[".Color Wavelengths", "[options=\"header\",cols=\"2,1\"]"],
            &[
                "|elink:VkColor |Wavelength",
                "|ename:VK_COLOR_RED |long",
                "|ename:VK_COLOR_BLUE",
                "|short +",
                "or very short",
                "|ename:VK_COLOR_BLACK |",
                "|ename:VK_COLOR_INDIGO |{times} 2",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            Some("VkColor"),
            &[".Cyan Shades", "[options=\"header\",cols=\"1,1a\"]"],
            &[
                "|Color |Formula",
                "|ename:VK_COLOR_CYAN |",
                "[latexmath]",
                "++++",
                "c = 1",
                "++++",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            Some("VkColor"),
            &[".Magenta Shades", "[options=\"header,footer\"]"],
            &[
                "|Color |Count",
                "|ename:VK_COLOR_MAGENTA |one",
                "|Total |one",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            Some("VkColor"),
            &[".Mixed Shades", "[options=\"header\",cols=\"1,1\"]"],
            &[
                "|Color |Steps",
                "|ename:VK_COLOR_YELLOW a|",
                "1. mix",
                "2. stir",
                "|ename:VK_COLOR_ORANGE a|",
                "!===",
                "!red !yellow",
                "!===",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            Some("VkColor"),
            &[".Advanced Colors", "[options=\"header\"]"],
            &[
                "|Color |Formula",
                "|ename:VK_COLOR_VIOLET a|",
                "[latexmath]",
                "++++",
                "    v = r   +   b \\\\",
                "      w = 0",
                "++++",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            None,
            &header,
            &["|Color |Use", "|ename:VK_COLOR_WHITE |snow"],
        ),
        fixture_table(
            "chapters/other.adoc",
            None,
            &[".Grey Shades", header[0]],
            &["|Color |Use", "|ename:VK_COLOR_GREY |ash"],
        ),
        fixture_table(
            FIXTURE_FILE,
            None,
            &[".Pink Things", header[0]],
            &[
                "|Value |Vertex",
                "|ename:VK_COLOR_PINK |i",
                "|ename:VK_THING_OTHER |i + 1",
            ],
        ),
        fixture_table(
            FIXTURE_FILE,
            None,
            &[".Teal Shades", header[0]],
            &["|Color |Use", "|ename:VK_COLOR_TEAL |sea"],
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn fixture_specification() -> Specification {
    let mut specification = Specification {
        pages: BTreeMap::new(),
        tables: fixture_tables(),
        extensions: BTreeSet::new(),
        attributes: BTreeMap::from([("times".to_string(), "×".to_string())]),
        titles: BTreeMap::new(),
        markup: Markup::new(),
    };
    let limits = [
        "  * pname:first is the first limit.",
        COMMENT,
        "    It is not the last.",
        "",
        COMMENT,
        "  ** This is not a part of the item.",
        "  * pname:feature specifies that things have features.",
        "+",
        "",
        ATTRIBUTE_ENTRY,
        "",
        "If the sname:VkLimits structure is included, it is filled in.",
        "",
        "It is filled in only once.",
        "  * pname:twice is followed by two empty lines.",
        "+",
        "",
        "",
        "This is not attached.",
        "  * pname:commented has a comment where its block would be.",
        "+",
        COMMENT,
        "",
        "This is not attached either.",
        "  * pname:last is the last limit.",
        "  ** It is nested.",
        "",
        "",
        "  ** This is nested as well.",
        "",
        "first::",
        "    For all limits, the note applies.",
        "",
        "second:: The second note applies as well.",
        "+",
        "It has a second paragraph.",
        "",
        "third::",
        "",
        "    Its description follows an empty line.",
        "",
        ".. The note has a step.",
        "",
        "The limits are described above.",
        "",
        "include::{generated}/validity/structs/VkLimits.adoc[]",
    ]
    .join("\n");
    let counts = [
        "The counts:",
        "",
        "  * pname:colors is a bitmask^1^ of colors.",
        "  * pname:depths is a bitmask^1^ of 2^1^ or 4^7^ depths^2^.",
        "  ** A nested item.",
        "",
        "  * pname:last is the last count.",
        "",
        "1::",
        "    For all bitmasks,",
        "    the first note applies.",
        "",
        "2:: The second note applies.",
        "",
        "The counts are described above, 2^1^ times.",
    ]
    .join("\n");
    let pages = [
        (
            "VkThing",
            "Structure specifying a thing",
            "
  * pname:count is the number of things.
  * pname:flags is reserved for future use.
  * pname:type is the kind of the thing.
  * pname:range is the
    [eq]#[minimum, maximum]# range of the things.
+
The range has the following requirements:
+
  ** It is not empty.
+
[NOTE]
====
Empty ranges are for another day.
====
  * pname:width, pname:height, and
    pname:depth are the size of the thing.
  * pname:sizeInBytes is the size of the thing in bytes.

A thing is empty with the following properties:

  * pname:elsewhere equal to `0`

.Valid Usage
****
  * [[VUID-VkThing-padding-00001]]
    pname:padding must: be `0`
  * [[VUID-VkThing-height-00002]]
    pname:height must: be greater than `0`
****",
        ),
        (
            "VkColor",
            "Colors of a thing",
            "
  * ename:VK_COLOR_RED specifies red.
  * ename:VK_COLOR_GREEN specifies green.
  * ename:VK_COLOR_GOLD and ename:VK_COLOR_SILVER specify a metal.
  * ename:VK_COLOR_ALL is equivalent to the logical OR of:
  ** ename:VK_COLOR_RED
  ** ename:VK_COLOR_GOLD",
        ),
        (
            "VkLimits",
            "Structure reporting the limits of things",
            limits.as_str(),
        ),
        (
            "VkCounts",
            "Structure reporting the counts of things",
            counts.as_str(),
        ),
        (
            "vkCountThings",
            "Count the things",
            "
  * pname:device is the logical device that owns the things.
  * pname:type is the type of the things to count.
  * pname:pThingCount is a pointer to a slink:VkThing structure in which
    the number of things is returned.
  * pname:pAllocator controls host memory allocation as described in the
    <<memory-allocation, Memory Allocation>> chapter.

.Valid Usage
****
  * [[VUID-vkCountThings-flags-00001]]
    pname:flags must: be `0`
****",
        ),
        (
            "PFN_vkThingCallback",
            "Application-defined thing callback",
            "
  * pname:pUserData is the user data.
  * pname:thing is the thing.
include::{generated}/validity/funcpointers/PFN_vkThingCallback.adoc[]",
        ),
    ];
    for (name, description, body) in pages {
        let body = specification.with_notes_at_their_marks(body);
        let body_words = specification.words(&body);
        let page = || Page {
            name: name.to_string(),
            file: FIXTURE_FILE.to_string(),
            description: description.to_string(),
            body: body.clone(),
            body_words: body_words.clone(),
            valid_usage: OnceCell::new(),
        };
        if name == "VkColor" {
            specification.pages.insert("VkColorKHR".to_string(), page());
        }
        specification.pages.insert(name.to_string(), page());
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
    /// `type` is the kind of the thing.
    pub ty: u32,
    /// `range` is the \[minimum, maximum\] range of the things.
    ///
    /// The range has the following requirements:
    ///
    /// - It is not empty.
    ///
    ///   > **Note**
    ///   >
    ///   > Empty ranges are for another day.
    pub range: [u32; 2],
    /// `width`, `height`, and `depth` are the size of the thing.
    pub width: u32,
    /// `width`, `height`, and `depth` are the size of the thing.
    pub height: u32,
    /// `width`, `height`, and `depth` are the size of the thing.
    pub depth: u32,
    /// `byteSize`, which the specification describes under the name `sizeInBytes`: `sizeInBytes` is
    /// the size of the thing in bytes.
    pub byte_size: u32,
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
    /// `VK_COLOR_BLUE` in the table “Color Wavelengths” of the specification:
    ///
    /// - Wavelength:
    ///   - short
    ///   - or very short
    #[doc(alias = "VK_COLOR_BLUE")]
    pub const BLUE: Self = Self(2);
    /// `VK_COLOR_BLACK`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_BLACK")]
    pub const BLACK: Self = Self(3);
    /// `VK_COLOR_INDIGO` in the table “Color Wavelengths” of the specification:
    ///
    /// - Wavelength: × 2
    #[doc(alias = "VK_COLOR_INDIGO")]
    pub const INDIGO: Self = Self(4);
    /// `VK_COLOR_VIOLET` in the table “Advanced Colors” of the specification:
    ///
    /// - Formula: `v = r + b \\ w = 0`
    #[doc(alias = "VK_COLOR_VIOLET")]
    pub const VIOLET: Self = Self(5);
    /// `VK_COLOR_TEAL` in the table “Teal Shades” of the specification:
    ///
    /// - Use: sea
    #[doc(alias = "VK_COLOR_TEAL")]
    pub const TEAL: Self = Self(6);
    /// `VK_COLOR_CYAN`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_CYAN")]
    pub const CYAN: Self = Self(7);
    /// `VK_COLOR_MAGENTA`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_MAGENTA")]
    pub const MAGENTA: Self = Self(8);
    /// `VK_COLOR_YELLOW`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_YELLOW")]
    pub const YELLOW: Self = Self(9);
    /// `VK_COLOR_ORANGE`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_ORANGE")]
    pub const ORANGE: Self = Self(10);
    /// `VK_COLOR_WHITE`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_WHITE")]
    pub const WHITE: Self = Self(11);
    /// `VK_COLOR_GREY`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_GREY")]
    pub const GREY: Self = Self(12);
    /// `VK_COLOR_PINK`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_PINK")]
    pub const PINK: Self = Self(13);
    /// `VK_COLOR_ALL` is equivalent to the logical OR of:
    ///
    /// - [`VK_COLOR_RED`](Color::RED)
    /// - [`VK_COLOR_GOLD`](Color::GOLD)
    #[doc(alias = "VK_COLOR_ALL")]
    pub const ALL: Self = Self(14);
    /// `VK_COLOR_GOLD` and [`VK_COLOR_SILVER`](Color::SILVER) specify a metal.
    #[doc(alias = "VK_COLOR_GOLD")]
    pub const GOLD: Self = Self(15);
    /// [`VK_COLOR_GOLD`](Color::GOLD) and `VK_COLOR_SILVER` specify a metal.
    #[doc(alias = "VK_COLOR_SILVER")]
    pub const SILVER: Self = Self(16);
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
    /// - `type` is the type of the things to count.
    /// - `pThingCount` is a pointer to a [`VkThing`](Thing) structure in which the number of things
    ///   is returned.
    /// - `pAllocator` controls host memory allocation as described in the Memory Allocation chapter.
    ///
    /// # Safety
    ///
    /// The call has to satisfy the valid usage that the Vulkan specification states for
    /// [`vkCountThings`](PAGES/vkCountThings.html).
    pub unsafe fn count_things_raw(
        &self,
        ty: u32,
        flags: u32,
        p_thing_count: *mut Thing,
        p_allocator: *const c_void,
    ) {
    }

    /// Count the things.
    ///
    /// Calls the C command [`vkCountThings`](PAGES/vkCountThings.html) through [`Self::count_things_raw`], which describes the parameters.
    #[doc(alias = "vkCountThings")]
    pub unsafe fn count_things(&self, allocator: Option<&c_void>) -> u32 {
        0
    }
}
"#;

fn fixture_checker(source: &str) -> Checker {
    let source = source.replace("PAGES/", REFERENCE_PAGES);
    let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
    let mut checker = Checker::default();
    check_files(
        &fixture_specification(),
        &Generated::new(),
        &mut checker,
        &[file],
    );
    checker
}

fn fixture_failures(source: &str) -> Vec<String> {
    fixture_checker(source).failures
}

const FIXTURE_ALLOCATOR_ITEM: &str = "    /// - `pAllocator` controls host memory allocation as described in the Memory Allocation chapter.\n";
const FIXTURE_ALLOCATOR_ARGUMENT: &str = "        p_allocator: *const c_void,\n    ) {\n";

fn fixture_with_the_allocator_as(item: &str) -> String {
    for unique in [FIXTURE_ALLOCATOR_ITEM, FIXTURE_ALLOCATOR_ARGUMENT] {
        assert_eq!(
            FIXTURE_SOURCE.matches(unique).count(),
            1,
            "`{unique}` is not exactly once in the fixture"
        );
    }
    FIXTURE_SOURCE
        .replacen(FIXTURE_ALLOCATOR_ITEM, &format!("    /// - {item}\n"), 1)
        .replacen(
            FIXTURE_ALLOCATOR_ARGUMENT,
            "        p_callbacks: *const c_void,\n    ) {\n",
            1,
        )
}

const FIXTURE_BYTE_SIZE: &str = "    /// `byteSize`, which the specification describes under the name `sizeInBytes`: `sizeInBytes` is
    /// the size of the thing in bytes.
";

#[test]
fn checker_accepts_a_member_that_takes_the_description_of_another_name() {
    let checker = fixture_checker(FIXTURE_SOURCE);
    assert!(checker.failures.is_empty(), "{:#?}", checker.failures);
    assert_eq!(checker.members_under_another_name, 1);
    assert_eq!(
        checker.descriptions_of_no_member,
        ["VkThing::elsewhere"],
        "`sizeInBytes` describes `byteSize`, and `elsewhere` is in a list that describes no member"
    );
}

#[test]
fn checker_collects_the_names_that_a_page_describes_and_no_member_has() {
    assert_eq!(
        FIXTURE_SOURCE.matches(FIXTURE_BYTE_SIZE).count(),
        1,
        "the description of `byte_size` is not exactly once in the fixture"
    );
    let undescribed = FIXTURE_SOURCE.replacen(
        FIXTURE_BYTE_SIZE,
        "    /// The C member `uint32_t byteSize`.\n",
        1,
    );
    let checker = fixture_checker(&undescribed);
    assert!(checker.failures.is_empty(), "{:#?}", checker.failures);
    assert_eq!(
        checker.descriptions_of_no_member,
        ["VkThing::elsewhere", "VkThing::sizeInBytes"],
        "a description under a name that differs from the name of the member is found"
    );
    let differing_in_case = FIXTURE_SOURCE.replacen(
        FIXTURE_BYTE_SIZE,
        "    /// The C member `uint32_t SizeInBytes`.\n",
        1,
    );
    assert_eq!(
        fixture_checker(&differing_in_case).descriptions_of_no_member,
        ["VkThing::elsewhere", "VkThing::sizeInBytes"],
        "the name of a member is compared as it is written in C"
    );
}

#[test]
fn checker_reports_a_member_description_taken_from_another_name_that_does_not_fit() {
    let taken = |own: &str, other: &str, description: &str| {
        format!(
            "    /// `{own}`, which the specification describes under the name `{other}`: {description}\n"
        )
    };
    let size = "`sizeInBytes` is the size of the thing in bytes.";
    for (defect, replace, with, report) in [
        (
            "a name that the page does not describe",
            FIXTURE_BYTE_SIZE,
            taken(
                "byteSize",
                "sizeOfThing",
                "`sizeOfThing` is the size of the thing in bytes.",
            ),
            "Thing::byte_size: the documentation takes the description of `sizeOfThing` for the member `byteSize`, but the specification describes no `sizeOfThing` on the page `VkThing`",
        ),
        (
            "a member that the page describes under its own name",
            "    /// `count` is the number of things.\n",
            taken("count", "sizeInBytes", size),
            "Thing::count: the specification describes the member `count` on the page `VkThing` under its own name, but the documentation takes the description of `sizeInBytes` for it",
        ),
        (
            "the description of a third member",
            FIXTURE_BYTE_SIZE,
            taken(
                "byteSize",
                "sizeInBytes",
                "`flags` is reserved for future use.",
            ),
            "Thing::byte_size: the description that the documentation takes for the member `byteSize` is not the one of `sizeInBytes`",
        ),
        (
            "a description that the page does not have",
            FIXTURE_BYTE_SIZE,
            taken(
                "byteSize",
                "sizeInBytes",
                "`sizeInBytes` is the size of everything.",
            ),
            "Thing::byte_size: ``sizeInBytes` is the size of everything.` is not in the text that the specification has for `VkThing`",
        ),
        (
            "the name of another member in front",
            FIXTURE_BYTE_SIZE,
            taken("count", "sizeInBytes", size),
            "Thing::byte_size: is documented with the description of `count`",
        ),
        (
            "a part of the item of the other name",
            FIXTURE_BYTE_SIZE,
            taken(
                "byteSize",
                "sizeInBytes",
                "`sizeInBytes` is the size of the thing.",
            ),
            "Thing::byte_size: the description of `sizeInBytes` leaves out a part of its list item on the page `VkThing`",
        ),
        (
            "the item of the other name cut in a sentence",
            FIXTURE_BYTE_SIZE,
            taken(
                "byteSize",
                "sizeInBytes",
                "`sizeInBytes` is the size of the",
            ),
            "Thing::byte_size: the description of `sizeInBytes` ends in the middle of a sentence",
        ),
    ] {
        assert_reported(defect, replace, &with, report);
    }
}

#[test]
fn checker_accepts_a_parameter_that_takes_the_description_of_another_name() {
    let checker = fixture_checker(&fixture_with_the_allocator_as(
        "`pCallbacks`, which the specification describes under the name `pAllocator`: `pAllocator` controls host\n    ///   memory allocation as described in the Memory Allocation chapter.",
    ));
    assert!(checker.failures.is_empty(), "{:#?}", checker.failures);
    assert_eq!(checker.parameters_under_another_name, 1);
    assert_eq!(
        checker.undescribed_arguments,
        ["Device::count_things_raw: flags"],
        "`p_callbacks` is described, and `flags` is the argument that the page does not describe"
    );
}

#[test]
fn checker_reports_a_description_taken_from_another_name_that_does_not_fit() {
    for (defect, item, report) in [
        (
            "a name that the page does not describe",
            "`pCallbacks`, which the specification describes under the name `pNothing`: `pNothing` controls host memory allocation as described in the Memory Allocation chapter.",
            "the documentation takes the description of `pNothing` for the parameter `pCallbacks`, but the specification describes no `pNothing` on the page `vkCountThings`",
        ),
        (
            "a parameter that the page describes under its own name",
            "`pThingCount`, which the specification describes under the name `pAllocator`: `pAllocator` controls host memory allocation as described in the Memory Allocation chapter.",
            "the specification describes the parameter `pThingCount` on the page `vkCountThings` under its own name, but the documentation takes the description of `pAllocator` for it",
        ),
        (
            "the description of a third parameter",
            "`pCallbacks`, which the specification describes under the name `pAllocator`: `type` is the type of the things to count.",
            "the description that the documentation takes for the parameter `pCallbacks` is not the one of `pAllocator`",
        ),
        (
            "a description that the page does not have",
            "`pCallbacks`, which the specification describes under the name `pAllocator`: `pAllocator` controls everything.",
            "``pAllocator` controls everything. ` is not in the text that the specification has for `vkCountThings`",
        ),
    ] {
        let failures = fixture_failures(&fixture_with_the_allocator_as(item));
        assert!(
            failures.iter().any(|failure| failure.contains(report)),
            "{defect}: expected a failure containing `{report}`, got {failures:#?}"
        );
    }
}

#[test]
fn checker_collects_the_arguments_that_a_page_with_parameters_does_not_describe() {
    let checker = fixture_checker(FIXTURE_SOURCE);
    assert_eq!(
        checker.undescribed_arguments,
        ["Device::count_things_raw: flags"],
        "only valid usage mentions `flags`"
    );
    let without_the_allocator = FIXTURE_SOURCE.replacen(FIXTURE_ALLOCATOR_ITEM, "", 1);
    assert_eq!(
        fixture_checker(&without_the_allocator).undescribed_arguments,
        [
            "Device::count_things_raw: flags",
            "Device::count_things_raw: p_allocator"
        ]
    );
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

const FIXTURE_RANGE: &str = "    /// `range` is the \\[minimum, maximum\\] range of the things.
    ///
    /// The range has the following requirements:
    ///
    /// - It is not empty.
    ///
    ///   > **Note**
    ///   >
    ///   > Empty ranges are for another day.
";

#[test]
fn checker_reports_a_description_that_leaves_out_a_part_of_its_list_item() {
    let lead = "    /// `range` is the \\[minimum, maximum\\] range of the things.\n";
    let requirements = "    ///\n    /// The range has the following requirements:\n";
    let list = "    ///\n    /// - It is not empty.\n";
    for (defect, doc, left_out) in [
        (
            "a description cut where a line of the item starts with markup",
            "    /// `range` is the\n".to_string(),
            "range is the [minimum maximum range of the things The range has the following requirements]",
        ),
        (
            "a description without what a list continuation attaches",
            lead.to_string(),
            "range of the things [The range has the following requirements It is not empty Note Empty]",
        ),
        (
            "a description without the attached list",
            format!("{lead}{requirements}"),
            "has the following requirements [It is not empty Note Empty ranges are for another day]",
        ),
        (
            "a description without the attached note",
            format!("{lead}{requirements}{list}"),
            "It is not empty [Note Empty ranges are for another day]",
        ),
    ] {
        assert_reported(
            defect,
            FIXTURE_RANGE,
            &doc,
            &format!(
                "Thing::range: the description of `range` leaves out a part of its list item on the page `VkThing`: `{left_out}`"
            ),
        );
    }
}

#[test]
fn checker_reports_a_description_that_ends_in_the_middle_of_a_sentence() {
    for end in [
        "`range` is the",
        "`range` is the \\[minimum, maximum\\] range of the things. The range has the following requirements:",
        "`range` is the \\[minimum,",
    ] {
        let tail = end
            .char_indices()
            .rev()
            .nth(59)
            .map_or(end, |(at, _)| &end[at..]);
        assert_reported(
            "a description cut in a sentence",
            FIXTURE_RANGE,
            &format!("    /// {end}\n"),
            &format!(
                "Thing::range: the description of `range` ends in the middle of a sentence: `…{}`",
                flat(tail)
            ),
        );
    }
    for whole in [
        "`a` is the range.",
        "`a` is [`VK_TRUE`](TRUE)",
        "`a` is one of: - this - those",
        "`a` is the corner (x,y)",
        "`a` is `b`\\[0\\]",
        "`a` is at least **1**",
        "`a` lists the formats: ```c++ a = b; ```",
    ] {
        assert!(
            !ends_in_the_middle_of_a_sentence(whole),
            "`{whole}` ends a sentence"
        );
    }
    for cut in [
        "`a` is the",
        "`a` is clamped to",
        "`a` is **either**",
        "`a` lists the formats:",
        "`a` depends on `b`,",
        "`a` is one of the following",
    ] {
        assert!(
            ends_in_the_middle_of_a_sentence(cut),
            "`{cut}` does not end a sentence"
        );
    }
}

const FIXTURE_ALL: &str = "    /// `VK_COLOR_ALL` is equivalent to the logical OR of:
    ///
    /// - [`VK_COLOR_RED`](Color::RED)
    /// - [`VK_COLOR_GOLD`](Color::GOLD)
";

#[test]
fn checker_reports_a_value_without_the_flags_that_its_list_item_goes_on_with() {
    assert_reported(
        "a fallback for a value whose list item goes on with nested items",
        FIXTURE_ALL,
        "    /// `VK_COLOR_ALL`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        "Color::ALL: the specification describes `VK_COLOR_ALL` on the page `VkColor`, but the documentation does not use it",
    );
    assert_reported(
        "a description that stops where the nested items start",
        FIXTURE_ALL,
        "    /// `VK_COLOR_ALL` is equivalent to the logical OR of:\n",
        "Color::ALL: the description of `VK_COLOR_ALL` leaves out a part of its list item on the page `VkColor`: `the logical OR of [VK_COLOR_RED VK_COLOR_GOLD]`",
    );
    assert_reported(
        "a description without one of the nested items",
        FIXTURE_ALL,
        "    /// `VK_COLOR_ALL` is equivalent to the logical OR of:\n    ///\n    /// - [`VK_COLOR_RED`](Color::RED)\n",
        "Color::ALL: the description of `VK_COLOR_ALL` leaves out a part of its list item on the page `VkColor`: `logical OR of VK_COLOR_RED [VK_COLOR_GOLD]`",
    );
}

#[test]
fn an_include_of_the_flags_that_a_flag_is_equivalent_to_reads_as_the_registry_lists_them() {
    let registry = r#"
        <enums name="VkAccessFlagBits2" type="bitmask" bitwidth="64">
            <enum bitpos="0" name="VK_ACCESS_2_A_BIT"/>
            <enum bitpos="1" name="VK_ACCESS_2_B_BIT"/>
        </enums>
        <enums name="VkEmptyFlagBits" type="bitmask"/>
        <enums name="VkPipelineStageFlagBits2" type="bitmask" bitwidth="64">
            <enum bitpos="0" name="VK_PIPELINE_STAGE_2_A_BIT"/>
        </enums>
        <enums name="VkOtherFlagBits" type="bitmask">
            <enum bitpos="0" name="VK_PIPELINE_STAGE_2_OTHER_BIT"/>
        </enums>
        <extension name="VK_KHR_reserved" number="2" supported="disabled"/>
        <extension name="VK_KHR_c" number="3" supported="vulkan,vulkansc">
            <require>
                <enum bitpos="2" extends="VkAccessFlagBits2" name="VK_ACCESS_2_C_BIT_KHR"/>
                <enum bitpos="1" extends="VkPipelineStageFlagBits2" name="VK_PIPELINE_STAGE_2_C_BIT_KHR"/>
                <enum bitpos="5" extends="VkOtherFlagBits" name="VK_PIPELINE_STAGE_2_E_BIT_KHR"/>
            </require>
        </extension>
        <extension name="VK_KHR_d" number="4" supported="vulkansc">
            <require>
                <enum bitpos="3" extends="VkAccessFlagBits2" name="VK_ACCESS_2_D_BIT_KHR"/>
            </require>
        </extension>
        <feature api="vulkan" name="VK_VERSION_1_4" number="1.4">
            <require>
                <enum bitpos="4" extends="VkPipelineStageFlagBits2" name="VK_PIPELINE_STAGE_2_F_BIT"/>
            </require>
        </feature>
        <sync>
            <syncstage name="VK_PIPELINE_STAGE_2_A_BIT" alias="VK_PIPELINE_STAGE_A_BIT">
                <syncsupport queues="VK_QUEUE_GRAPHICS_BIT"/>
            </syncstage>
            <syncstage name="VK_PIPELINE_STAGE_2_NONE"/>
            <syncstage name="VK_PIPELINE_STAGE_2_ALL_BIT">
                <syncsupport queues="VK_QUEUE_GRAPHICS_BIT"/>
                <syncequivalent stage="VK_PIPELINE_STAGE_2_F_BIT,VK_PIPELINE_STAGE_2_C_BIT_KHR,VK_PIPELINE_STAGE_2_OTHER_BIT,VK_PIPELINE_STAGE_2_E_BIT_KHR,VK_PIPELINE_STAGE_2_A_BIT"/>
            </syncstage>
            <syncaccess name="VK_ACCESS_2_ALL_BIT" alias="VK_ACCESS_ALL_BIT">
                <syncequivalent access="VK_ACCESS_2_B_BIT,VK_ACCESS_2_D_BIT_KHR,VK_ACCESS_2_C_BIT_KHR,VK_ACCESS_2_A_BIT"/>
            </syncaccess>
        </sync>
    "#;
    let flags = |names: &[&str]| {
        names
            .iter()
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
    };
    assert_eq!(
        equivalent_flags(registry),
        BTreeMap::from([
            (
                "VK_ACCESS_2_ALL_BIT".to_string(),
                flags(&[
                    "VK_ACCESS_2_B_BIT",
                    "VK_ACCESS_2_C_BIT_KHR",
                    "VK_ACCESS_2_A_BIT"
                ])
            ),
            (
                "VK_PIPELINE_STAGE_2_ALL_BIT".to_string(),
                flags(&[
                    "VK_PIPELINE_STAGE_2_F_BIT",
                    "VK_PIPELINE_STAGE_2_C_BIT_KHR",
                    "VK_PIPELINE_STAGE_2_A_BIT"
                ])
            ),
        ])
    );
}

#[test]
fn the_list_items_of_the_specification_are_read_to_their_end() {
    let specification = fixture_specification();
    let item = |page: &str, macro_name: &str, name: &str| {
        specification
            .item(page, macro_name, name)
            .unwrap_or_else(|| panic!("no item {name} on the page {page}"))
    };
    assert_eq!(
        item("VkThing", "pname", "type"),
        "  * pname:type is the kind of the thing."
    );
    let range = item("VkThing", "pname", "range");
    assert!(
        range.starts_with("  * pname:range is the\n    [eq]#[minimum, maximum]# range"),
        "{range}"
    );
    assert!(
        range.ends_with("[NOTE]\n====\nEmpty ranges are for another day.\n===="),
        "{range}"
    );
    assert_eq!(
        specification.words(&range),
        " range is the minimum maximum range of the things The range has the following requirements It is not empty Note Empty ranges are for another day "
    );
    assert_eq!(
        specification.item("VkThing", "pname", "padding"),
        None,
        "valid usage is not a description"
    );
    assert_eq!(specification.item("VkThing", "ename", "type"), None);
    assert_eq!(
        item("vkCountThings", "pname", "pAllocator"),
        "  * pname:pAllocator controls host memory allocation as described in the\n    <<memory-allocation, Memory Allocation>> chapter."
    );
    assert_eq!(
        item("VkColor", "ename", "VK_COLOR_ALL"),
        "  * ename:VK_COLOR_ALL is equivalent to the logical OR of:\n  ** ename:VK_COLOR_RED\n  ** ename:VK_COLOR_GOLD"
    );
    assert_eq!(
        item("PFN_vkThingCallback", "pname", "thing"),
        "  * pname:thing is the thing.",
        "the generated valid usage is a block of the page, not a part of the item"
    );
    assert_eq!(
        specification.words(
            "pname:size[i] is [eq]#pname:pSizes[i]# for all [eq]#0 {times} i\n    < [2]# and [eq]##n##s"
        ),
        " size i is pSizes i for all 0 i 2 and ns ",
        "a role span keeps what is in it, brackets included"
    );
    for (page, macro_name, names, text) in [
        (
            "VkThing",
            "pname",
            ["width", "height", "depth"].as_slice(),
            "  * pname:width, pname:height, and\n    pname:depth are the size of the thing.",
        ),
        (
            "VkColor",
            "ename",
            ["VK_COLOR_GOLD", "VK_COLOR_SILVER"].as_slice(),
            "  * ename:VK_COLOR_GOLD and ename:VK_COLOR_SILVER specify a metal.",
        ),
    ] {
        for name in names {
            assert_eq!(
                item(page, macro_name, name),
                text,
                "the item of {name} is the one that starts with the names of {names:?}"
            );
        }
    }
}

#[test]
fn a_list_item_of_the_specification_goes_on_past_empty_lines_as_asciidoctor_reads_it() {
    let specification = fixture_specification();
    let words = |name: &str| {
        let item = specification
            .item("VkLimits", "pname", name)
            .unwrap_or_else(|| panic!("no item {name} on the page VkLimits"));
        specification.words(&item)
    };
    assert_eq!(
        words("first"),
        " first is the first limit It is not the last ",
        "a comment after an empty line ends the item, whatever follows the comment"
    );
    assert_eq!(
        words("feature"),
        " feature specifies that things have features If the VkLimits structure is included it is filled in ",
        "one empty line and a line that sets an attribute do not detach what a list continuation attaches"
    );
    assert_eq!(
        words("twice"),
        " twice is followed by two empty lines ",
        "two empty lines detach what a list continuation attaches"
    );
    assert_eq!(
        words("commented"),
        " commented has a comment where its block would be ",
        "a comment takes the place of the block that a list continuation attaches"
    );
    assert_eq!(
        words("last"),
        " last is the last limit It is nested This is nested as well first For all limits the note applies second The second note applies as well It has a second paragraph third Its description follows an empty line The note has a step ",
        "nested items and lists of another kind go on past empty lines, and a term without a description takes the text after an empty line"
    );
}

#[test]
fn a_comment_after_an_empty_line_ends_an_item_unless_a_term_waits_for_its_description() {
    let specification = fixture_specification();
    let with_a_term = [
        "  * pname:a is the size.",
        "",
        "1::",
        "",
        COMMENT,
        "The note.",
        "",
        "Outside.",
    ];
    assert_eq!(specification.item_end(&with_a_term, 0), 6);
    let with_a_described_term = [
        "  * pname:a is the size.",
        "",
        "1:: The note.",
        "",
        COMMENT,
        "Outside.",
    ];
    assert_eq!(specification.item_end(&with_a_described_term, 0), 3);
    let without_a_term = ["  * pname:a is the size.", "", COMMENT, "  ** Outside."];
    assert_eq!(specification.item_end(&without_a_term, 0), 1);
}

#[test]
fn a_numbered_note_of_the_specification_belongs_to_the_items_that_carry_its_mark() {
    let specification = fixture_specification();
    let words = |name: &str| {
        let item = specification
            .item("VkCounts", "pname", name)
            .unwrap_or_else(|| panic!("no item {name} on the page VkCounts"));
        specification.words(&item)
    };
    assert_eq!(
        words("colors"),
        " colors is a bitmask note 1 of colors Note 1 For all bitmasks the first note applies "
    );
    assert_eq!(
        words("depths"),
        " depths is a bitmask note 1 of 2 1 or 4 7 depths note 2 A nested item Note 1 For all bitmasks the first note applies Note 2 The second note applies ",
        "a superscript after a digit is an exponent and no mark, also with the number of a note, and the notes follow the item in the order of their numbers"
    );
    assert_eq!(
        words("last"),
        " last is the last count ",
        "the notes are no part of the item that they are nested in"
    );
    let body = &specification.pages["VkCounts"].body;
    assert!(
        body.ends_with("\n\nThe counts are described above, 2^1^ times."),
        "a superscript outside the items is left as it is: {body}"
    );
    assert_eq!(
        specification.with_notes_at_their_marks("  * pname:a is 2^1^.\n\nterm:: no note"),
        "  * pname:a is 2^1^.\n\nterm:: no note",
        "a page without numbered notes has no marks"
    );
}

const FIXTURE_HEIGHT: &str =
    "    /// `width`, `height`, and `depth` are the size of the thing.\n    pub height: u32,\n";
const FIXTURE_SILVER: &str =
    "    /// [`VK_COLOR_GOLD`](Color::GOLD) and `VK_COLOR_SILVER` specify a metal.\n";

#[test]
fn checker_reports_a_fallback_for_a_name_that_an_item_starts_with_among_others() {
    assert_reported(
        "a member that the item of another member describes as well",
        FIXTURE_HEIGHT,
        "    /// The C member `uint32_t height`.\n    pub height: u32,\n",
        "Thing::height: the specification describes `height` on the page `VkThing`, but the documentation does not use it",
    );
    assert_reported(
        "a value that the item of another value describes as well",
        FIXTURE_SILVER,
        "    /// `VK_COLOR_SILVER`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        "Color::SILVER: the specification describes `VK_COLOR_SILVER` on the page `VkColor`, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_part_left_out_of_the_item_of_a_name_that_is_not_the_first() {
    assert_reported(
        "a member with a part of the item that it shares with other members",
        FIXTURE_HEIGHT,
        "    /// `width`, `height`, and `depth` are the size.\n    pub height: u32,\n",
        "Thing::height: the description of `height` leaves out a part of its list item on the page `VkThing`: `depth are the size [of the thing]`",
    );
    assert_reported(
        "a value with an item that it shares with another value, cut in a sentence",
        FIXTURE_SILVER,
        "    /// [`VK_COLOR_GOLD`](Color::GOLD) and `VK_COLOR_SILVER` specify a\n",
        "Color::SILVER: the description of `VK_COLOR_SILVER` ends in the middle of a sentence: `…",
    );
}

#[test]
fn checker_reports_valid_usage_taken_for_a_description() {
    assert_reported(
        "a member documented with a requirement on it",
        FIXTURE_HEIGHT,
        "    /// `height` must be greater than `0`\n    pub height: u32,\n",
        "Thing::height: is documented with valid usage of the page `VkThing`, which states a requirement and does not describe the item: ``height` must be greater than `0``",
    );
    let specification = fixture_specification();
    for (description, states) in [
        ("`height` must be greater than `0`", true),
        ("`height` must be greater", true),
        ("`padding` must be `0` or more", true),
        ("`height` is greater than `0`", false),
        (
            "`width`, `height`, and `depth` are the size of the thing.",
            false,
        ),
        ("`height`", false),
    ] {
        assert_eq!(
            specification.states_valid_usage("VkThing", description),
            states,
            "{description}"
        );
    }
}

#[test]
fn checker_reports_a_description_that_starts_with_other_names_only() {
    assert_reported(
        "a member with the description of other members",
        FIXTURE_HEIGHT,
        "    /// `width` and `depth` are the size of the thing.\n    pub height: u32,\n",
        "Thing::height: is documented with the description of `width`",
    );
    assert_reported(
        "a value with the description of other values",
        FIXTURE_SILVER,
        "    /// [`VK_COLOR_GOLD`](Color::GOLD) and `VK_COLOR_RED` specify a metal.\n",
        "Color::SILVER: does not start with its C name `VK_COLOR_SILVER`",
    );
    let generated = Generated::new();
    for (description, names) in [
        ("`a` is one.", vec!["a"]),
        ("`a` and `b` are two.", vec!["a", "b"]),
        (
            "`a`, [`b`](Color::B),\nand `c` are three.",
            vec!["a", "b", "c"],
        ),
        (
            "[`a`](Color::A), `b` and [`c`](Color::C) are three.",
            vec!["a", "b", "c"],
        ),
        ("`a`, which is like `b` and `c`.", vec!["a"]),
        ("`a` or `b` is one.", vec!["a"]),
        ("The `a` and `b`.", vec![]),
    ] {
        assert_eq!(
            generated.names_at_start(description),
            names,
            "{description}"
        );
    }
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
fn checker_reports_a_value_fallback_when_a_table_row_of_the_specification_describes_the_value() {
    assert_reported(
        "a value documented only by its type although a table row describes it",
        "/// `VK_COLOR_BLUE` in the table “Color Wavelengths” of the specification:\n    ///\n    /// - Wavelength:\n    ///   - short\n    ///   - or very short\n",
        "/// `VK_COLOR_BLUE`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        "Color::BLUE: the specification describes `VK_COLOR_BLUE` in a table row, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_table_row_that_the_specification_does_not_have() {
    for (defect, replace, with, title) in [
        (
            "a cell with other words",
            "///   - or very short",
            "///   - or very long",
            "Color Wavelengths",
        ),
        (
            "a cell under another header",
            "/// - Wavelength:\n",
            "/// - Frequency:\n",
            "Color Wavelengths",
        ),
        (
            "a cell line left out",
            "    ///   - or very short\n",
            "",
            "Color Wavelengths",
        ),
        (
            "a table with another title",
            "/// `VK_COLOR_BLUE` in the table “Color Wavelengths”",
            "/// `VK_COLOR_BLUE` in the table “Color Names”",
            "Color Names",
        ),
    ] {
        assert_reported(
            defect,
            replace,
            with,
            &format!("Color::BLUE: ``VK_COLOR_BLUE` in the table “{title}” of the specification:"),
        );
    }
    assert_reported(
        "a second table row of another value",
        "    ///   - or very short\n",
        "    ///   - or very short\n    ///\n    /// `VK_COLOR_RED` in the table “Color Wavelengths” of the specification:\n    ///\n    /// - Wavelength: long\n",
        "Color::BLUE: is documented with the table row of `VK_COLOR_RED`",
    );
}

fn color_values() -> BTreeSet<String> {
    [
        "RED", "BLUE", "BLACK", "INDIGO", "VIOLET", "TEAL", "CYAN", "MAGENTA", "YELLOW", "ORANGE",
        "WHITE", "GREY", "PINK", "ALL", "GOLD", "SILVER",
    ]
    .iter()
    .map(|color| format!("VK_COLOR_{color}"))
    .collect()
}

#[test]
fn table_rows_name_their_value_in_the_first_cell_and_keep_cells_over_several_lines() {
    let specification = fixture_specification();
    let values = color_values();
    let rows = specification.table_rows(Some("VkColor"), &values, "VK_COLOR_BLUE");
    assert_eq!(rows.len(), 1);
    assert_eq!(
        rows[0]
            .1
            .iter()
            .map(|cell| cell.text.as_str())
            .collect::<Vec<_>>(),
        ["ename:VK_COLOR_BLUE", "short +\nor very short"]
    );
    assert_eq!(
        specification
            .table_rows(Some("VkColorKHR"), &values, "VK_COLOR_BLUE")
            .len(),
        1,
        "the page of an alias is the page of the type"
    );
    assert!(
        specification
            .table_rows(Some("VkColor"), &values, "VK_COLOR_BLACK")
            .is_empty(),
        "a row with only empty cells describes nothing"
    );
    assert!(
        specification
            .table_rows(Some("VkThing"), &values, "VK_COLOR_RED")
            .is_empty(),
        "a table on the page of another type does not describe the value"
    );
    assert_eq!(
        specification
            .table_rows(Some("VkColor"), &values, "VK_COLOR_TEAL")
            .len(),
        1,
        "a titled table outside any page, in the file of the page, whose rows are all values of the type"
    );
}

#[test]
fn the_implicit_header_of_a_table_is_its_first_row_when_an_empty_line_follows_the_first_line() {
    let header_and_rows = |content: &[&str]| {
        fixture_table(FIXTURE_FILE, None, &["[cols=\"1,1\"]"], content)
            .map(|table| (table.header.join(" "), table.rows.len()))
    };
    for (content, rows) in [
        (&["|a |b |c |d", "", "|e |f"][..], 2),
        (&["|a |b |c", "", "|d"][..], 1),
        (&["|a", "", "|b |e |f"][..], 1),
        (&["|a |b", "", "", "  |c |d"][..], 1),
        (&["|a |b", "// a comment", "", "a|c |d"][..], 1),
    ] {
        assert_eq!(
            header_and_rows(content),
            Some(("a b".to_string(), rows)),
            "{content:?}"
        );
    }
    for content in [
        &["|a", "|b", "", "|c |d"][..],
        &["|a |b", "|c |d", ""][..],
        &["", "|a |b", "", "|c |d"][..],
        &["|a |b", "", "more of b", "|c |d"][..],
        &["|a |b", "", "x |c |d"][..],
    ] {
        assert_eq!(header_and_rows(content), None, "{content:?}");
    }
}

#[test]
fn the_columns_of_a_table_are_counted_as_asciidoctor_counts_them_or_the_table_is_not_read() {
    let columns = |above: &[&str], content: &[&str]| {
        fixture_table(FIXTURE_FILE, None, above, content).map(|table| table.header.len())
    };
    let header = "[options=\"header\"]";
    for (content, counted) in [
        (&["|a |b", "|c |d"][..], 2),
        (&["|a |b", "x |c", "|d |e |f"][..], 3),
        (&["|a |b", "x |c |d", "|e |f |g |h"][..], 4),
        (&["|a |b", "more of b", "  |c |d"][..], 2),
    ] {
        assert_eq!(columns(&[header], content), Some(counted), "{content:?}");
    }
    let content = ["|a |b |c |d |e |f", "|g |h |i |j |k |l"];
    for (cols, counted) in [
        ("59%,28%,13%", Some(3)),
        ("<.,^.2,2*>.,1", None),
        ("<.,^.2,2*>.,1,~", Some(6)),
        ("3*,.>2,<.^1", None),
        ("2*,.>2,<.^1,~,1,<", None),
        ("2*,.>2,<.^1,~,1", Some(6)),
        ("1,2a,3", None),
        ("2;1", None),
        ("1,,2", Some(3)),
        ("1,%,2", None),
        ("1,.,2", None),
        ("1,+1*,2", None),
        ("0*", None),
        ("", None),
    ] {
        let attributes = format!("[options=\"header\",cols=\"{cols}\"]");
        assert_eq!(
            columns(&[attributes.as_str()], &content),
            counted,
            "{cols:?}"
        );
    }
}

#[test]
fn tables_that_could_be_misread_or_misattributed_describe_no_value() {
    let specification = fixture_specification();
    let values = color_values();
    for (value, shape) in [
        ("VK_COLOR_CYAN", "a column with a style in `cols`"),
        ("VK_COLOR_MAGENTA", "a table with a footer"),
        ("VK_COLOR_YELLOW", "an AsciiDoc cell with an ordered list"),
        ("VK_COLOR_ORANGE", "an AsciiDoc cell with a nested table"),
        ("VK_COLOR_WHITE", "an untitled table outside any page"),
        ("VK_COLOR_GREY", "a table outside any page in another file"),
        (
            "VK_COLOR_PINK",
            "a table outside any page with a row of another type",
        ),
    ] {
        assert!(
            specification
                .table_rows(Some("VkColor"), &values, value)
                .is_empty(),
            "{shape} must not be used for `{value}`"
        );
        let constant = value.trim_start_matches("VK_COLOR_");
        assert_reported(
            shape,
            &format!("/// `{value}`, a value of [`VkColor`](PAGES/VkColor.html).\n"),
            &format!(
                "/// `{value}` in the table “Shades” of the specification:\n    ///\n    /// - Use: x\n"
            ),
            &format!(
                "Color::{constant}: ``{value}` in the table “Shades” of the specification: - Use: x` is not a row that the specification has for `{value}`"
            ),
        );
    }
}

#[test]
fn checker_compares_operators_and_whole_formulas_in_table_cells() {
    for (defect, replace, with) in [
        (
            "an operator changed",
            "/// - Wavelength: × 2",
            "/// - Wavelength: + 2",
        ),
        (
            "a formula cut short",
            "`v = r + b \\\\ w = 0`",
            "`v = r + b`",
        ),
        ("a formula left empty", "`v = r + b \\\\ w = 0`", "` `"),
        (
            "a formula without its code span",
            "`v = r + b \\\\ w = 0`",
            "v = r + b \\\\ w = 0",
        ),
    ] {
        assert_reported(
            defect,
            replace,
            with,
            "is not a row that the specification has for `VK_COLOR_",
        );
    }
}

#[test]
fn checker_finds_the_table_rows_of_a_type_linked_through_an_alias_page() {
    let failures = fixture_failures(&FIXTURE_SOURCE.replacen(
        "/// C enumeration [`VkColor`](PAGES/VkColor.html).",
        "/// C enumeration [`VkColorKHR`](PAGES/VkColorKHR.html).",
        1,
    ));
    assert!(
        !failures
            .iter()
            .any(|failure| failure.contains("is not a row that")),
        "{failures:#?}"
    );
}

#[test]
fn checker_reports_a_table_lead_without_cells_or_without_a_table() {
    assert_reported(
        "a table lead without its cells",
        "    ///\n    /// - Use: sea\n",
        "",
        "Color::TEAL: has a table row without its cells",
    );
    assert_reported(
        "a table lead that does not name a table",
        "/// `VK_COLOR_TEAL` in the table “Teal Shades” of the specification:",
        "/// `VK_COLOR_TEAL` in the table “Teal Shades” of a document:",
        "Color::TEAL: ``VK_COLOR_TEAL` in the table “Teal Shades” of a document:` does not name a table row",
    );
}

#[test]
fn checker_reports_a_parameter_left_out_of_the_parameters_of_a_command() {
    assert_reported(
        "a command without one of its parameters",
        "    /// - `pAllocator` controls host memory allocation as described in the Memory Allocation chapter.\n",
        "",
        "Device::count_things_raw: the specification describes the parameter `pAllocator` on the page `vkCountThings`, but the documentation does not list it",
    );
    assert_reported(
        "a command without the parameter that its Rust name `ty` stands for",
        "    /// - `type` is the type of the things to count.\n",
        "",
        "Device::count_things_raw: the specification describes the parameter `type` on the page `vkCountThings`, but the documentation does not list it",
    );
    assert_reported(
        "a command without one of its wrapped parameter items",
        "    /// - `pThingCount` is a pointer to a [`VkThing`](Thing) structure in which the number of things\n    ///   is returned.\n",
        "",
        "Device::count_things_raw: the specification describes the parameter `pThingCount` on the page `vkCountThings`, but the documentation does not list it",
    );
    let section_start = FIXTURE_SOURCE
        .find("    /// # Parameters\n    ///\n    /// - `type`")
        .expect("the parameters of count_things_raw");
    let section_end = section_start
        + FIXTURE_SOURCE[section_start..]
            .find("    /// # Safety")
            .expect("the safety section of count_things_raw");
    assert_reported(
        "a command without its parameters section",
        &FIXTURE_SOURCE[section_start..section_end],
        "",
        "Device::count_things_raw: the specification describes the parameter `pThingCount` on the page `vkCountThings`, but the documentation does not list it",
    );
}

#[test]
fn checker_does_not_take_valid_usage_for_a_description() {
    let specification = fixture_specification();
    let thing = specification.described("VkThing", "pname");
    assert_eq!(
        thing,
        [
            "count",
            "flags",
            "type",
            "range",
            "width",
            "height",
            "depth",
            "sizeInBytes",
            "elsewhere"
        ],
        "an item describes every name that it starts with"
    );
    let count_things = specification.described("vkCountThings", "pname");
    assert_eq!(
        count_things,
        ["device", "type", "pThingCount", "pAllocator"]
    );
    let failures = fixture_failures(FIXTURE_SOURCE);
    assert!(
        !failures
            .iter()
            .any(|failure| failure.contains("`flags`") || failure.contains("`padding`")),
        "an argument or a member that only valid usage mentions is taken as described: {failures:#?}"
    );
}

#[test]
fn rust_names_are_matched_with_the_c_names_they_were_made_from() {
    for (rust_name, c_name) in [
        ("p_thing_count", "pThingCount"),
        ("ty", "type"),
        ("r#match", "match"),
        ("self_", "self"),
        ("_2d", "2D"),
    ] {
        assert!(
            is_rust_name_of(rust_name, c_name),
            "`{rust_name}` is the Rust name of `{c_name}`"
        );
    }
    for (rust_name, c_name) in [("ty", "typeCount"), ("p_thing_count", "thingCount")] {
        assert!(
            !is_rust_name_of(rust_name, c_name),
            "`{rust_name}` is not the Rust name of `{c_name}`"
        );
    }
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
            "structure in which the number of things",
            "structure in which the number of widgets",
            "Device::count_things_raw: ``pThingCount` is a pointer to a [`VkThing`](Thing) structure in which the number of widgets",
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

struct Leftovers {
    code_span: Regex,
    link_target: Regex,
    patterns: Vec<(&'static str, Regex)>,
}

impl Leftovers {
    fn new() -> Self {
        let macros = "pname|ptext|sname|stext|fname|ftext|etext|dname|tname|code|slink|elink|tlink|basetype|ename|flink|dlink|apiext|reflink|tag|attr|latexmath|can|cannot|may|must|optional|optionally|required|should|undefined";
        Self {
            code_span: regex(r"``.*?``|`[^`\n]*`"),
            link_target: regex(r"\]\([^)\s]*\)"),
            patterns: vec![
                ("an unconstrained mark `##`", regex("##")),
                ("a role span", regex(r"\\?\[[A-Za-z0-9._-]+\\?\]#")),
                (
                    "a double hyphen that AsciiDoc replaces with an em dash",
                    regex(r"(?m)(?:^|\s)--(?:\s|$)|\w--\w"),
                ),
                (
                    "a macro with a bracketed argument",
                    regex(r"\b[a-z]+:\\?\["),
                ),
                (
                    "a specification macro",
                    regex(&format!(r"\b(?:{macros}):\S")),
                ),
                ("a cross reference", regex("<<|>>")),
                ("an anchor", regex(r"\\?\[\\?\[")),
                (
                    "a preprocessor directive",
                    regex(r"\b(?:ifdef|ifndef|endif|include)::"),
                ),
                (
                    "an attribute reference",
                    regex(r"\{[A-Za-z][A-Za-z0-9_-]*\}"),
                ),
            ],
        }
    }

    fn found(&self, doc: &str) -> Vec<&'static str> {
        let text = self.code_span.replace_all(doc, "");
        let text = self.link_target.replace_all(&text, "]");
        self.patterns
            .iter()
            .filter(|(_, pattern)| pattern.is_match(&text))
            .map(|(what, _)| *what)
            .collect()
    }
}

fn doc_blocks(source: &str) -> Vec<String> {
    let mut blocks = Vec::<String>::new();
    let mut in_block = false;
    for line in source.lines() {
        match line.trim_start().strip_prefix("///") {
            Some(doc) => {
                if !in_block {
                    blocks.push(String::new());
                }
                let block = blocks.last_mut().expect("a block");
                block.push_str(doc.strip_prefix(' ').unwrap_or(doc));
                block.push('\n');
                in_block = true;
            }
            None => in_block = false,
        }
    }
    blocks
}

fn asciidoc_leftovers(source: &str) -> Vec<String> {
    let leftovers = Leftovers::new();
    doc_blocks(source)
        .iter()
        .flat_map(|doc| {
            leftovers
                .found(doc)
                .into_iter()
                .map(move |what| format!("{what} in `{}`", flat(doc)))
        })
        .collect()
}

#[test]
fn generated_documentation_has_no_asciidoc_left() {
    let directory = workspace_root().join("crates/vulkan/src/vk/generated");
    let mut checked = 0;
    let mut failures = Vec::new();
    for entry in fs::read_dir(&directory).expect("read the generated sources") {
        let path = entry.expect("directory entry").path();
        let source = fs::read_to_string(&path).expect("read a generated source");
        checked += doc_blocks(&source).len();
        failures.extend(asciidoc_leftovers(&source).into_iter().map(|failure| {
            let file = path.file_name().unwrap_or_default().to_string_lossy();
            format!("{file}: {failure}")
        }));
    }
    assert!(checked > 10_000, "only {checked} doc comments were checked");
    assert!(
        failures.is_empty(),
        "{} generated doc comments keep AsciiDoc markup:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

fn spaces_before_punctuation(source: &str) -> Vec<String> {
    let code_span = regex(r"``.*?``|`[^`\n]*`");
    let separated = regex(r"\S [.,;:!?)]+(?: |$)");
    doc_blocks(source)
        .iter()
        .filter_map(|doc| {
            let text = doc.split("```").step_by(2).collect::<Vec<_>>().join(" ");
            let text = flat(&code_span.replace_all(&text, "`code`"));
            let found = separated.find(&text)?;
            Some(format!("`{}` in `{}`", found.as_str().trim(), flat(doc)))
        })
        .collect()
}

#[test]
fn generated_documentation_has_no_space_before_closing_punctuation() {
    let directory = workspace_root().join("crates/vulkan/src/vk/generated");
    let mut checked = 0;
    let mut failures = Vec::new();
    for entry in fs::read_dir(&directory).expect("read the generated sources") {
        let path = entry.expect("directory entry").path();
        let source = fs::read_to_string(&path).expect("read a generated source");
        checked += doc_blocks(&source).len();
        failures.extend(
            spaces_before_punctuation(&source)
                .into_iter()
                .map(|failure| {
                    let file = path.file_name().unwrap_or_default().to_string_lossy();
                    format!("{file}: {failure}")
                }),
        );
    }
    assert!(checked > 10_000, "only {checked} doc comments were checked");
    assert!(
        failures.is_empty(),
        "{} generated doc comments have a space before closing punctuation, which is where a line of the specification starts with the punctuation:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn a_space_before_closing_punctuation_is_reported() {
    for (doc, found) in [
        ("/// a copy . If more", "y ."),
        ("/// is either :\n///\n/// - this", "r :"),
        ("/// the command\n/// , or the other", "d ,"),
        (
            "/// via [`vkTrace`](crate::Device::trace) ,\n/// or not",
            ") ,",
        ),
        ("/// `a` , `b` and `c`", "` ,"),
        ("/// (see the list ) and", "t )"),
        ("/// - an item\n///   ; the end", "m ;"),
        ("/// really ?!", "y ?!"),
    ] {
        let reported = spaces_before_punctuation(&format!("{doc}\npub struct Thing;\n"));
        assert_eq!(reported.len(), 1, "`{doc}`: {reported:?}");
        assert!(
            reported[0].starts_with(&format!("`{found}` in `")),
            "`{doc}`: {reported:?}"
        );
    }
    for doc in [
        "/// a copy. If more, then: this; that (or those)!",
        "/// `a . b`, `x ,` and ``c ` ; d``",
        "/// ```c\n/// a = b ? c : d ;\n/// ```",
        "/// a .5 factor and ...more, a?b:c",
        "/// - `a`: one\n/// - `b`: two",
        "/// x — y - z",
    ] {
        let reported = spaces_before_punctuation(&format!("{doc}\npub struct Thing;\n"));
        assert!(reported.is_empty(), "`{doc}` is reported: {reported:?}");
    }
}

#[test]
fn asciidoc_left_in_documentation_is_reported() {
    for (doc, what) in [
        ("/// zero, Nan##s and", "an unconstrained mark `##`"),
        ("/// the [eq]#x# value", "a role span"),
        ("/// the \\[eq\\]#x# value", "a role span"),
        ("/// parameters -- no substitution", "a double hyphen"),
        ("/// parameters --\n/// no substitution", "a double hyphen"),
        ("/// a word--word", "a double hyphen"),
        (
            "/// press kbd:\\[Ctrl\\]",
            "a macro with a bracketed argument",
        ),
        ("/// see pname:width", "a specification macro"),
        ("/// it must:be", "a specification macro"),
        ("/// see <<fundamentals>>", "a cross reference"),
        ("/// [[anchor]]text", "an anchor"),
        ("/// ifdef::VK_KHR_surface[]", "a preprocessor directive"),
        ("/// {maxBlockSize} bytes", "an attribute reference"),
    ] {
        let found = asciidoc_leftovers(&format!("{doc}\npub struct Thing;\n"));
        assert!(
            found.iter().any(|failure| failure.starts_with(what)),
            "`{doc}` is not reported as {what}: {found:?}"
        );
    }
}

#[test]
fn markdown_that_resembles_asciidoc_is_not_reported() {
    for doc in [
        "/// `a ## b` and ``x`##`y``",
        "/// `--flag`, `a -- b` and `pname:x`",
        "/// [`VkExtent2D`](Extent2D) and [the registry](https://a.b/c--d#x)",
        "/// a set {0, 1} and `{x}`",
        "/// i \\< 2 \\<\\< 3 and a - b and --- and a-- b",
        "/// # Safety\n///\n/// - must be valid",
        "/// note: \\[x\\] and the C:\\\\path",
    ] {
        let found = asciidoc_leftovers(&format!("{doc}\npub struct Thing;\n"));
        assert!(found.is_empty(), "`{doc}` is reported: {found:?}");
    }
}
