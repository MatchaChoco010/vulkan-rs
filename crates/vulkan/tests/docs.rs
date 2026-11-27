use std::cell::OnceCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use regex::{Captures, Regex};
use syn::{Attribute, Expr, FnArg, ImplItem, Item, Lit, Meta, Pat, Type};

const REFERENCE_PAGES: &str = "https://docs.vulkan.org/refpages/latest/refpages/source/";
const SPECIFICATION: &str = "https://docs.vulkan.org/spec/latest/";

const LEGACY_WITHOUT_A_REASON: &str =
    "Legacy functionality, for which the registry gives no reason.";
const LEGACY_AND_NOT_USED: &str = "Legacy functionality that is not used.";
const DESCRIBED_AS_NOT_USED: &str = "is legacy and not used";
const ALSO_CALLED: &str = "The C header also calls this value ";

const LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE: &[&str] =
    &["STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED"];

const LEGACY_MACROS_WITHOUT_AN_ITEM: &[&str] = &[
    "VK_API_VERSION",
    "VK_MAKE_VERSION",
    "VK_VERSION_MAJOR",
    "VK_VERSION_MINOR",
    "VK_VERSION_PATCH",
];

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

#[derive(Clone, Debug, PartialEq, Eq)]
struct LegacyMark {
    link: String,
    marked_by: Vec<String>,
    superseded_by: Option<String>,
}

#[derive(Debug, Default)]
struct LegacyRegistry {
    marks: BTreeMap<(String, String, String), LegacyMark>,
    entries: BTreeMap<String, usize>,
    entries_of_other_apis: usize,
    vendor_tags: Vec<String>,
    enumerations: BTreeSet<String>,
    type_aliases: BTreeSet<String>,
}

fn promotion(author: Option<&str>) -> u32 {
    match author {
        None => 0,
        Some("KHR") => 1,
        Some("EXT") => 2,
        Some(_) => 3,
    }
}

impl LegacyRegistry {
    fn read(registry: &str) -> Self {
        let child = regex(r"^<(\w+)\s[^>]*/>$");
        let mut legacy = Self::default();
        let mut interface = None::<(String, bool)>;
        let mut block = None::<(String, bool)>;
        for line in registry.lines().map(str::trim) {
            let names_vulkan = |apis: &str| apis.split(',').any(|api| api == "vulkan");
            let api_matches =
                |element: &str| xml_attribute(element, "api").is_none_or(names_vulkan);
            if line.starts_with("<type ")
                && let Some(_) = xml_attribute(line, "alias")
            {
                let name = xml_attribute(line, "name").expect("a type alias has a name");
                if api_matches(line) {
                    legacy.type_aliases.insert(name.to_string());
                }
            } else if line.starts_with("<tag ") {
                let tag = xml_attribute(line, "name").expect("a vendor tag has a name");
                legacy.vendor_tags.push(tag.to_string());
            } else if line.starts_with("<enums ")
                && let Some("enum" | "bitmask") = xml_attribute(line, "type")
            {
                let name = xml_attribute(line, "name").expect("an enumeration has a name");
                legacy.enumerations.insert(name.to_string());
            } else if line.starts_with("<feature ")
                && let Some(number) = xml_attribute(line, "number")
            {
                let version = format!("VK_VERSION_{}", number.replace('.', "_"));
                interface = Some((version, api_matches(line)));
            } else if line.starts_with("<extension ") {
                let name = xml_attribute(line, "name").expect("an extension has a name");
                let supported = xml_attribute(line, "supported").is_some_and(names_vulkan);
                interface = Some((name.to_string(), supported && api_matches(line)));
            } else if line == "</feature>" || line == "</extension>" {
                interface = None;
            } else if line.starts_with("<deprecate") {
                assert!(
                    interface.is_some(),
                    "`{line}` is in no version or extension"
                );
                let link = xml_attribute(line, "explanationlink")
                    .unwrap_or_else(|| panic!("`{line}` has no explanationlink"));
                block = Some((link.to_string(), api_matches(line)));
            } else if line == "</deprecate>" {
                block = None;
            } else if let (Some((link, block_matches)), Some((marked_by, interface_matches))) =
                (&block, &interface)
            {
                let kind = child
                    .captures(line)
                    .map(|child| child[1].to_string())
                    .filter(|kind| ["type", "command", "feature"].contains(&kind.as_str()))
                    .unwrap_or_else(|| panic!("the <deprecate> block of {marked_by} has `{line}`"));
                if !(*interface_matches && *block_matches && api_matches(line)) {
                    legacy.entries_of_other_apis += 1;
                    continue;
                }
                *legacy.entries.entry(kind.clone()).or_default() += 1;
                let name = xml_attribute(line, "name").expect("a marked name");
                let structure = xml_attribute(line, "struct").unwrap_or_default();
                let superseded_by = xml_attribute(line, "supersededby").map(str::to_string);
                let mark = legacy
                    .marks
                    .entry((kind, structure.to_string(), name.to_string()))
                    .or_insert_with(|| LegacyMark {
                        link: link.clone(),
                        marked_by: Vec::new(),
                        superseded_by: superseded_by.clone(),
                    });
                assert!(
                    mark.link == *link && mark.superseded_by == superseded_by,
                    "{marked_by} marks {name} with `{link}` and {superseded_by:?}, and another block with `{}` and {:?}",
                    mark.link,
                    mark.superseded_by
                );
                if !mark.marked_by.contains(marked_by) {
                    mark.marked_by.push(marked_by.clone());
                }
            }
        }
        legacy
    }

    fn names(&self, kind: &str) -> usize {
        self.marks.keys().filter(|(k, _, _)| k == kind).count()
    }

    fn vendor_tag(&self, c_name: &str) -> Option<&str> {
        let mut tags = self.vendor_tags.iter().map(String::as_str);
        tags.find(|tag| c_name.ends_with(tag))
    }

    fn is_type_alias(&self, kind: &str, name: &str) -> bool {
        kind == "type" && self.type_aliases.contains(name)
    }

    fn shown<'a>(
        &self,
        kind: &str,
        structure: &str,
        name: &str,
        mark: &'a LegacyMark,
    ) -> Vec<&'a str> {
        if self.is_type_alias(kind, name) {
            return Vec::new();
        }
        let marked = if structure.is_empty() {
            name
        } else {
            structure
        };
        let marked = promotion(self.vendor_tag(marked));
        let counts =
            |by: &&str| by.starts_with("VK_VERSION_") || promotion(by.split('_').nth(1)) <= marked;
        let marked_by = mark.marked_by.iter().map(String::as_str);
        marked_by.filter(counts).collect()
    }
}

struct ValueDescription {
    page: Option<String>,
    file: String,
    text: String,
}

impl ValueDescription {
    fn place(&self) -> String {
        match &self.page {
            Some(page) => format!("the page {page}"),
            None => format!("{} outside the reference pages", self.file),
        }
    }
}

struct Segment {
    page: Option<String>,
    file: String,
    lines: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DeprecatedAttribute {
    reason: String,
    alias: Option<String>,
}

#[derive(Debug, Default)]
struct DeprecatedAttributes {
    attributes: BTreeMap<(String, String, String), DeprecatedAttribute>,
    entries: BTreeMap<String, usize>,
    entries_of_other_apis: usize,
    aliases: BTreeMap<String, String>,
}

impl DeprecatedAttributes {
    fn read(&mut self, registry: &str) {
        let member_name = regex(r"<name>(\w+)</name>");
        let mut interface_matches = true;
        let mut require_matches = true;
        let mut group = None::<(String, bool)>;
        let mut structure = None::<String>;
        for line in registry.lines().map(str::trim) {
            let names_vulkan = |apis: &str| apis.split(',').any(|api| api == "vulkan");
            let api_matches =
                |element: &str| xml_attribute(element, "api").is_none_or(names_vulkan);
            if line.starts_with("<feature ") && xml_attribute(line, "number").is_some() {
                interface_matches = api_matches(line);
            } else if line.starts_with("<extension ") {
                let supported = xml_attribute(line, "supported").is_some_and(names_vulkan);
                interface_matches = supported && api_matches(line);
            } else if line == "</feature>" || line == "</extension>" {
                interface_matches = true;
            } else if line.starts_with("<require") {
                require_matches = api_matches(line);
            } else if line == "</require>" {
                require_matches = true;
            } else if line.starts_with("<enums ") {
                let name = xml_attribute(line, "name").expect("a group of enumerants has a name");
                let owner = match xml_attribute(line, "type") {
                    Some("enum" | "bitmask") => name,
                    _ => "",
                };
                group = (!line.ends_with("/>")).then(|| (owner.to_string(), api_matches(line)));
            } else if line == "</enums>" {
                group = None;
            } else if line.starts_with("<type ")
                && !line.ends_with("/>")
                && !line.contains("</type>")
            {
                structure = xml_attribute(line, "name").map(str::to_string);
            } else if line == "</type>" {
                structure = None;
            }
            let is_enumerant = line.starts_with("<enum ");
            let is_member = line.starts_with("<member");
            let group_matches = group.as_ref().is_none_or(|(_, matches)| *matches);
            let matches =
                interface_matches && require_matches && group_matches && api_matches(line);
            if is_enumerant
                && matches
                && let (Some(name), Some(alias)) =
                    (xml_attribute(line, "name"), xml_attribute(line, "alias"))
            {
                self.aliases.insert(name.to_string(), alias.to_string());
            }
            let Some(reason) = xml_attribute(line, "deprecated") else {
                continue;
            };
            assert!(
                is_enumerant || is_member,
                "the test knows the attribute `deprecated` on <enum> and <member>, and the registry has `{line}`"
            );
            assert!(
                ["aliased", "true", "unused"].contains(&reason),
                "the test does not know the value of `deprecated` in `{line}`"
            );
            if !matches {
                self.entries_of_other_apis += 1;
                continue;
            }
            *self.entries.entry(reason.to_string()).or_default() += 1;
            let (element, owner, name) = if is_enumerant {
                let in_a_group = group.as_ref().map(|(owner, _)| owner.as_str());
                let owner = xml_attribute(line, "extends").or(in_a_group);
                let name = xml_attribute(line, "name").expect("an enumerant has a name");
                ("enum", owner.unwrap_or_default(), name.to_string())
            } else {
                let owner = structure.as_deref().expect("a member is in a type");
                let name = member_name.captures(line).expect("a member has a name");
                ("member", owner, name[1].to_string())
            };
            let attribute = DeprecatedAttribute {
                reason: reason.to_string(),
                alias: xml_attribute(line, "alias").map(str::to_string),
            };
            let key = (element.to_string(), owner.to_string(), name);
            let earlier = self.attributes.entry(key).or_insert(attribute.clone());
            assert_eq!(
                *earlier, attribute,
                "`{line}` and another element say different things"
            );
        }
    }

    fn value_of<'a>(&'a self, mut name: &'a str) -> &'a str {
        let mut steps = 0;
        while let Some(alias) = self.aliases.get(name) {
            assert!(steps < 8, "the aliases of {name} do not end");
            (name, steps) = (alias.as_str(), steps + 1);
        }
        name
    }

    fn other_names<'a>(&'a self, name: &'a str) -> Vec<&'a str> {
        let value = self.value_of(name);
        let aliases = self.aliases.keys().map(String::as_str);
        let same_value = aliases.filter(|alias| self.value_of(alias) == value);
        same_value
            .chain([value])
            .filter(|other| *other != name)
            .collect()
    }
}

struct Specification {
    pages: BTreeMap<String, Page>,
    value_descriptions: BTreeMap<String, Vec<ValueDescription>>,
    unnamed_descriptions: BTreeMap<String, Vec<(String, String)>>,
    value_aliases: BTreeMap<String, BTreeSet<String>>,
    values_of_structures_and_handles: BTreeSet<String>,
    tables: Vec<Table>,
    extensions: BTreeSet<String>,
    attributes: BTreeMap<String, String>,
    titles: BTreeMap<String, String>,
    anchors: BTreeMap<String, BTreeSet<String>>,
    published: BTreeSet<String>,
    legacy: LegacyRegistry,
    deprecated: DeprecatedAttributes,
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
    reference_without_a_text: Regex,
    link_of_an_untitled_anchor: Regex,
    list_item: Regex,
    nested_item: Regex,
    other_list_item: Regex,
    term: Regex,
    note: Regex,
    mark: Regex,
    described_item: Regex,
    value_description: Regex,
    table_delimiter: Regex,
    leading_anchors: Regex,
    anchor_name: Regex,
    member_anchor: Regex,
    name_macro: Regex,
    value_without_its_macro: Regex,
    specified: Regex,
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
            value_description: regex(
                r"^[ \t]*\*[ \t]+((?:\[\[[^\]]*\]\]\s*)*)ename:(\w+)\s+specifies\s+\S",
            ),
            table_delimiter: regex(r"^\|={3,}\s*$"),
            leading_anchors: regex(r"^[ \t]*\*[ \t]+((?:\[\[[^\]]*\]\]\s*)*)"),
            anchor_name: regex(r"\[\[\s*([^\],\s]+)"),
            member_anchor: regex(r"^(?:features|limits)-([A-Za-z0-9_]+)$"),
            name_macro: regex(r"\b(pname|ename):(\w+)"),
            value_without_its_macro: regex(r"^(VK_[A-Z0-9_]+)\s+specifies\s+\S"),
            specified: regex(r"^\s+specifies\s+\S"),
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
            reference_without_a_text: regex(r"<<([\w.-]+)>>"),
            link_of_an_untitled_anchor: regex(&format!(
                r"\[\\\[([\w.-]+)\\\]\]\({}[\w/-]+\.html#[\w.-]+\)",
                regex::escape(SPECIFICATION)
            )),
        }
    }
}

struct Sources {
    root: PathBuf,
    defined: BTreeSet<String>,
    equivalent_flags: BTreeMap<String, Vec<String>>,
    sync_table_rows: BTreeMap<String, Vec<String>>,
    value_aliases: BTreeMap<String, BTreeSet<String>>,
    values_of_structures_and_handles: BTreeSet<String>,
    conditional: Regex,
    end_conditional: Regex,
    tagged_include: Regex,
    equivalent_flags_include: Regex,
    sync_table_rows_include: Regex,
    tag_marker: Regex,
    attribute_entry: Regex,
    attribute_reference: Regex,
}

const TABLE_ROWS_NOT_READ: [&str; 2] = [
    "include::{generated}/formats/",
    "include::{generated}/spirvcap/",
];

const COMMENT: &str = "//";
const ATTRIBUTE_ENTRY: &str = "//:";

fn xml_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let start = tag.find(&format!(" {name}=\""))? + name.len() + 3;
    let end = tag[start..].find('"')?;
    Some(&tag[start..start + end])
}

fn sync_flags_in_the_api(registry: &str) -> BTreeSet<&str> {
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
    in_the_api
}

fn sync_table_rows(registry: &str) -> BTreeMap<String, Vec<String>> {
    let in_the_api = sync_flags_in_the_api(registry);
    let element = regex(r"(?s)<sync(stage|access)(\s[^>]*?)(?:/>|>(.*?)</sync(?:stage|access)>)");
    let support_tag = regex(r"<syncsupport\s[^>]*>");
    let mut stages = Vec::new();
    let mut access_flags = Vec::new();
    for sync in element.captures_iter(registry) {
        let name = xml_attribute(&sync[2], "name").expect("the name of a sync element");
        if !in_the_api.contains(name) {
            continue;
        }
        let support = sync
            .get(3)
            .and_then(|body| support_tag.find(body.as_str()))
            .map(|tag| tag.as_str());
        let supported = |attribute: &str| {
            support.map(|tag| {
                xml_attribute(tag, attribute)
                    .unwrap_or_else(|| panic!("<syncsupport> of {name} has no `{attribute}`"))
                    .split(',')
                    .collect::<Vec<_>>()
            })
        };
        if &sync[1] == "stage" {
            let queues = match supported("queues") {
                Some(queues) => queues
                    .iter()
                    .map(|queue| format!("ename:{queue}"))
                    .collect::<Vec<_>>()
                    .join(" or "),
                None => "None required".to_string(),
            };
            stages.push(format!("|ename:{name} | {queues}"));
            continue;
        }
        match xml_attribute(&sync[2], "alias") {
            Some(alias) => {
                access_flags.push(format!("|ename:{name}, +"));
                access_flags.push(format!(" ename:{alias} |"));
            }
            None => access_flags.push(format!("|ename:{name} |")),
        }
        let Some(supporting) = supported("stage") else {
            access_flags.push("\tAny".to_string());
            continue;
        };
        for (index, stage) in supporting.iter().enumerate() {
            let comma = if index + 1 < supporting.len() {
                ","
            } else {
                ""
            };
            if in_the_api.contains(stage) {
                access_flags.push(format!("\tename:{stage}{comma}"));
            }
        }
    }
    BTreeMap::from([
        ("supportedPipelineStages".to_string(), stages),
        ("supportedAccessTypes".to_string(), access_flags),
    ])
}

fn equivalent_flags(registry: &str) -> BTreeMap<String, Vec<String>> {
    let in_the_api = sync_flags_in_the_api(registry);
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

fn values_of_structures_and_handles(registry: &str) -> BTreeSet<String> {
    let structure_type =
        regex(r#"<member\s[^>]*\bvalues="(\w+)"[^>]*>\s*<type>VkStructureType</type>"#);
    let handle = regex(r#"<type\s[^>]*\bcategory="handle"[^>]*>"#);
    let structure_types = structure_type
        .captures_iter(registry)
        .map(|member| member[1].to_string());
    let object_types = handle
        .find_iter(registry)
        .filter_map(|tag| xml_attribute(tag.as_str(), "objtypeenum"))
        .map(str::to_string);
    structure_types.chain(object_types).collect()
}

fn value_aliases(registry: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut alias_of = BTreeMap::new();
    for tag in regex(r"<enum\s[^>]*>").find_iter(registry) {
        let tag = tag.as_str();
        if let (Some(name), Some(alias)) = (xml_attribute(tag, "name"), xml_attribute(tag, "alias"))
        {
            alias_of.insert(name, alias);
        }
    }
    let mut aliases = BTreeMap::<String, BTreeSet<String>>::new();
    for name in alias_of.keys() {
        let mut value = *name;
        for _ in 0..alias_of.len() {
            match alias_of.get(value) {
                Some(next) => value = next,
                None => break,
            }
        }
        aliases
            .entry(value.to_string())
            .or_default()
            .insert(name.to_string());
    }
    aliases
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
            sync_table_rows: sync_table_rows(&registry),
            value_aliases: value_aliases(&registry),
            values_of_structures_and_handles: values_of_structures_and_handles(&registry),
            conditional: regex(r"^(ifdef|ifndef)::([^\[]*)\[(.*)\]\s*$"),
            end_conditional: regex(r"^endif::[^\[]*\[\]\s*$"),
            tagged_include: regex(r"^include::\{chapters\}/([^\[]+)\[tags?=([^\],;]+)\]\s*$"),
            equivalent_flags_include: regex(
                r"^include::\{generated\}/sync/flagDefinitions/(\w+)\.adoc\[\]\s*$",
            ),
            sync_table_rows_include: regex(
                r"^include::\{generated\}/sync/(supportedPipelineStages|supportedAccessTypes)\.adoc\[\]\s*$",
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
            if let Some(include) = self.sync_table_rows_include.captures(line) {
                out.extend(self.sync_table_rows[&include[1]].iter().cloned());
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
        let anchor_definition = regex(r"\[\[([\w.-]+)\]\]");
        let title_line = regex(r#"^(?:=+ +|\.|image::.*\btitle=")([^\s.="][^"]*?)\s*(?:".*)?$"#);
        let table_syntax = TableSyntax::new();

        let mut pages = BTreeMap::new();
        let mut segments = Vec::new();
        let mut tables = Vec::new();
        let mut titles = BTreeMap::new();
        let mut defined_anchors = BTreeMap::new();
        let mut published = BTreeSet::new();
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
            let file_name = file.file_name().and_then(|name| name.to_str());
            let text = fs::read_to_string(&file).expect("read a chapter");
            if is_published_page(file_name.expect("a file name"), &text) {
                published.insert(relative.clone());
            }
            let mut lines = Vec::new();
            sources.lines(&file, None, &mut BTreeMap::new(), &mut lines);
            for anchor in lines
                .iter()
                .flat_map(|line| anchor_definition.captures_iter(line))
            {
                defined_anchors
                    .entry(anchor[1].to_string())
                    .or_insert_with(BTreeSet::new)
                    .insert(relative.clone());
            }
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

            let mut outside_from = 0;
            for (first, last, name) in &blocks {
                segments.push(Segment {
                    page: None,
                    file: relative.clone(),
                    lines: lines[outside_from..*first].to_vec(),
                });
                segments.push(Segment {
                    page: Some(name.clone()),
                    file: relative.clone(),
                    lines: lines[first + 2..*last].to_vec(),
                });
                outside_from = (last + 1).min(lines.len());
            }
            segments.push(Segment {
                page: None,
                file: relative.clone(),
                lines: lines[outside_from..].to_vec(),
            });

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
                let content = &lines[index + 1..close];
                let includes = content
                    .iter()
                    .filter(|line| line.starts_with("include::"))
                    .collect::<Vec<_>>();
                for include in &includes {
                    assert!(
                        TABLE_ROWS_NOT_READ
                            .iter()
                            .any(|not_read| include.starts_with(not_read)),
                        "{relative}: a table takes rows from {include}, which the test neither reads nor knows as rows that describe no value"
                    );
                }
                if includes.is_empty()
                    && let Some(table) =
                        table_syntax.read(&relative, page, &lines[start..index], content)
                {
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
            value_descriptions: BTreeMap::new(),
            unnamed_descriptions: BTreeMap::new(),
            value_aliases: sources.value_aliases.clone(),
            values_of_structures_and_handles: sources.values_of_structures_and_handles.clone(),
            tables,
            extensions,
            attributes,
            titles,
            anchors: defined_anchors,
            published,
            legacy: LegacyRegistry::read(
                &fs::read_to_string(root.join("xml/vk.xml")).expect("read xml/vk.xml"),
            ),
            deprecated: {
                let mut deprecated = DeprecatedAttributes::default();
                for registry in ["xml/vk.xml", "xml/video.xml"] {
                    let registry = fs::read_to_string(root.join(registry));
                    deprecated.read(&registry.expect("read a registry file"));
                }
                deprecated
            },
            markup: Markup::new(),
        };
        for page in pages.values_mut() {
            page.body = specification.with_notes_at_their_marks(&page.body);
            page.body_words = specification.words(&page.body);
        }
        specification.pages = pages;
        specification.value_descriptions = specification.value_descriptions_in(&segments);
        specification.unnamed_descriptions = specification.unnamed_descriptions_of_the_pages();
        specification
    }

    fn unnamed_descriptions_of_the_pages(&self) -> BTreeMap<String, Vec<(String, String)>> {
        self.pages
            .keys()
            .map(|page| (page.clone(), self.unnamed_descriptions_on(page)))
            .collect()
    }

    fn unnamed_descriptions_on(&self, page: &str) -> Vec<(String, String)> {
        let Some(page) = self.pages.get(page) else {
            return Vec::new();
        };
        let markup = &self.markup;
        let lines = page.body.lines().collect::<Vec<_>>();
        let mut descriptions = Vec::new();
        let mut index = 0;
        while index < lines.len() {
            let start = index;
            index += 1;
            if !markup.list_item.is_match(lines[start]) {
                continue;
            }
            index = self.item_end(&lines, start);
            let text = lines[start..index].join("\n");
            if markup.described_item.is_match(&text) {
                continue;
            }
            let Some(leading) = markup.leading_anchors.captures(&text) else {
                continue;
            };
            if leading[1].contains("[[VUID-") {
                continue;
            }
            let said = &text[leading[0].len()..];
            for anchor in markup.anchor_name.captures_iter(&leading[1]) {
                if let Some(member) = markup.member_anchor.captures(&anchor[1])
                    && regex(&format!(r"(?-u:\b)pname:{}(?-u:\b)", &member[1])).is_match(said)
                {
                    descriptions.push((member[1].to_string(), text.clone()));
                }
            }
            let value = match markup.value_without_its_macro.captures(said) {
                Some(value) => Some(value[1].to_string()),
                None => markup
                    .name_macro
                    .captures(said)
                    .filter(|first| {
                        let after = &said[first.get(0).map_or(0, |name| name.end())..];
                        &first[1] == "ename" && markup.specified.is_match(after)
                    })
                    .map(|first| first[2].to_string()),
            };
            if let Some(value) = value {
                descriptions.push((value, text.clone()));
            }
        }
        descriptions
    }

    fn described_without_its_name_first(
        &self,
        page: &str,
        macro_name: &str,
        name: &str,
    ) -> Vec<&str> {
        if self.describes(page, macro_name, name) {
            return Vec::new();
        }
        self.unnamed_descriptions
            .get(page)
            .into_iter()
            .flatten()
            .filter(|(described, _)| described == name)
            .map(|(_, text)| text.as_str())
            .collect()
    }

    fn value_descriptions_in(
        &self,
        segments: &[Segment],
    ) -> BTreeMap<String, Vec<ValueDescription>> {
        let mut descriptions = BTreeMap::<String, Vec<ValueDescription>>::new();
        for segment in segments {
            let lines = segment.lines.iter().map(String::as_str).collect::<Vec<_>>();
            let mut index = 0;
            while index < lines.len() {
                let start = index;
                index += 1;
                if self.markup.table_delimiter.is_match(lines[start]) {
                    let delimiter = lines[start].trim_end();
                    let cells = lines[index..]
                        .iter()
                        .position(|line| line.trim_end() == delimiter)
                        .unwrap_or(lines.len() - index);
                    index += cells + 1;
                    continue;
                }
                if !self.markup.list_item.is_match(lines[start]) {
                    continue;
                }
                index = self.item_end(&lines, start);
                let text = lines[start..index].join("\n");
                if let Some(described) = self.markup.value_description.captures(&text)
                    && !described[1].contains("[[VUID-")
                {
                    descriptions
                        .entry(described[2].to_string())
                        .or_default()
                        .push(ValueDescription {
                            page: segment.page.clone(),
                            file: segment.file.clone(),
                            text,
                        });
                }
            }
        }
        descriptions
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
        self.table_rows_and_their_place(page, values, value).1
    }

    fn table_rows_and_their_place(
        &self,
        page: Option<&str>,
        values: &BTreeSet<String>,
        value: &str,
    ) -> (bool, Vec<(&Table, &[Cell])>) {
        let Some(page) = page.and_then(|page| self.pages.get(page)) else {
            return (true, Vec::new());
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
        let on_the_page = !rows.is_empty();
        if !on_the_page {
            rows = rows_in(true);
        }
        rows.retain(|(_, row)| {
            row[1..].iter().any(|cell| !cell.text.is_empty())
                && row.iter().all(|cell| self.cell_is_readable(cell))
        });
        (on_the_page, rows)
    }

    fn describes(&self, page: &str, macro_name: &str, item: &str) -> bool {
        self.described(page, macro_name)
            .iter()
            .any(|described| described == item)
    }

    fn described_outside_the_page(&self, page: &str, value: &str) -> &[ValueDescription] {
        if !self.pages.contains_key(page) || self.describes(page, "ename", value) {
            return &[];
        }
        self.value_descriptions
            .get(value)
            .map_or(&[], Vec::as_slice)
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

fn is_legacy_attribute_statement(paragraph: &str) -> bool {
    paragraph == LEGACY_WITHOUT_A_REASON
        || paragraph == LEGACY_AND_NOT_USED
        || paragraph.starts_with(ALSO_CALLED)
}

fn also_called(legacy_names: &[&str], value: &str) -> String {
    let names = legacy_names.iter().map(|name| format!("`{name}`"));
    let names = join_and(names.collect());
    match legacy_names.len() {
        1 => format!("{ALSO_CALLED}{names}, which is a legacy alias of {value}."),
        _ => format!("{ALSO_CALLED}{names}, which are legacy aliases of {value}."),
    }
}

fn is_legacy_statement(paragraph: &str) -> bool {
    (paragraph.starts_with("Legacy functionality") && !is_legacy_attribute_statement(paragraph))
        || paragraph.contains("[Legacy Functionality](")
}

fn legacy_statements(doc: &str) -> Vec<String> {
    doc.split("\n\n")
        .map(flat)
        .filter(|paragraph| is_legacy_statement(paragraph))
        .collect()
}

fn without_legacy_statements(doc: &str) -> String {
    let is_legacy = |paragraph: &str| {
        is_legacy_statement(paragraph) || is_legacy_attribute_statement(paragraph)
    };
    doc.split("\n\n")
        .filter(|paragraph| !is_legacy(&flat(paragraph)))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn join_and(mut items: Vec<String>) -> String {
    match items.len() {
        0 | 1 => items.pop().unwrap_or_default(),
        _ => {
            let last = items.pop().expect("more than one item");
            format!("{} and {last}", items.join(", "))
        }
    }
}

struct Documented {
    kind: &'static str,
    name: String,
    features: BTreeSet<String>,
    doc: String,
    aliases: Vec<String>,
    link_target: Option<(String, String)>,
}

impl Documented {
    fn key(&self) -> (&'static str, &str) {
        (self.kind, &self.name)
    }
}

fn features_of(generated: &Generated, attributes: &[Attribute]) -> BTreeSet<String> {
    attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("cfg"))
        .filter_map(|attribute| match &attribute.meta {
            Meta::List(list) => Some(list.tokens.to_string()),
            _ => None,
        })
        .flat_map(|condition| {
            generated
                .cfg_feature
                .captures_iter(&condition)
                .map(|feature| feature[1].to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn documented_items(generated: &Generated, items: &[Item], documented: &mut Vec<Documented>) {
    let named = |ident: &syn::Ident, attributes: &[Attribute]| {
        let (doc, aliases) = doc_attributes(attributes);
        let c_name = aliases.first().cloned().unwrap_or(ident.to_string());
        Documented {
            kind: "type",
            name: c_name.clone(),
            features: features_of(generated, attributes),
            doc,
            aliases,
            link_target: Some((c_name, ident.to_string())),
        }
    };
    for item in items {
        match item {
            Item::Struct(item) => {
                let owner = named(&item.ident, &item.attrs);
                let fields = item.fields.iter().filter(|field| field.ident.is_some());
                documented.extend(fields.map(|field| documented_field(&owner, &item.ident, field)));
                documented.push(owner);
            }
            Item::Union(item) => {
                let owner = named(&item.ident, &item.attrs);
                let fields = item.fields.named.iter();
                documented.extend(fields.map(|field| documented_field(&owner, &item.ident, field)));
                documented.push(owner);
            }
            Item::Type(item) => documented.push(named(&item.ident, &item.attrs)),
            Item::Const(item) => documented.push(named(&item.ident, &item.attrs)),
            Item::Mod(item) => {
                if let Some((_, nested)) = &item.content {
                    documented_items(generated, nested, documented);
                }
            }
            Item::Impl(item) if item.trait_.is_none() => {
                let owner = type_name(&item.self_ty);
                let owner_features = features_of(generated, &item.attrs);
                for member in &item.items {
                    let (ident, attributes) = match member {
                        ImplItem::Const(constant) => (&constant.ident, &constant.attrs),
                        ImplItem::Fn(function) => (&function.sig.ident, &function.attrs),
                        _ => continue,
                    };
                    let (doc, aliases) = doc_attributes(attributes);
                    let mut features = features_of(generated, attributes);
                    features.extend(owner_features.iter().cloned());
                    let method = format!("{owner}::{ident}");
                    let called = generated
                        .called_command
                        .captures(&flat(&doc))
                        .and_then(|called| called.get(1).or(called.get(2)))
                        .map(|called| called.as_str().to_string());
                    let is_function = matches!(member, ImplItem::Fn(_));
                    let (kind, name, link_target) = match called {
                        Some(command)
                            if is_function
                                && ident.to_string().ends_with("_raw")
                                && flat(&doc).contains(" with the arguments as they are.") =>
                        {
                            ("raw", command, None)
                        }
                        Some(command)
                            if is_function
                                && flat(&doc).contains(" through [`Self::")
                                && aliases.first() == Some(&command) =>
                        {
                            let path = format!("crate::{method}");
                            ("wrapper", command.clone(), Some((command, path)))
                        }
                        _ if is_function => ("method", method, None),
                        _ => ("value", method, None),
                    };
                    documented.push(Documented {
                        kind,
                        name,
                        features,
                        doc,
                        aliases,
                        link_target,
                    });
                }
            }
            _ => {}
        }
    }
}

type LinkTargets<'a> = BTreeMap<&'a str, (&'a str, &'a BTreeSet<String>)>;

fn is_published_page(file_name: &str, text: &str) -> bool {
    let named_like_a_page = file_name.starts_with(|c: char| c.is_ascii_alphabetic())
        && file_name.ends_with(".adoc")
        && file_name != "vulkanscdeviations.adoc";
    let first_title_level = text.lines().take(30).find_map(|line| {
        let (marks, title) = line.split_once(' ')?;
        let is_a_title = !marks.is_empty()
            && marks.chars().all(|mark| mark == '=' || mark == '#')
            && title.starts_with(|c: char| c.is_ascii_uppercase());
        is_a_title.then_some(marks.len())
    });
    named_like_a_page && first_title_level == Some(1)
}

fn expected_legacy_statement(
    spec: &Specification,
    targets: &LinkTargets<'_>,
    from: &BTreeSet<String>,
    mark: &LegacyMark,
    shown: &[&str],
) -> Result<String, String> {
    let link = |c_name: &str| {
        let (path, features) = targets
            .get(c_name)
            .ok_or_else(|| format!("the bindings have no item for `{c_name}`"))?;
        Ok::<_, String>(if features.is_subset(from) {
            format!("[`{c_name}`]({path})")
        } else {
            format!("`{c_name}`")
        })
    };
    let what = match &mark.superseded_by {
        Some(successor) => format!(", superseded by {}", link(successor)?),
        None => {
            let marked_by = shown
                .iter()
                .map(|by| match by.strip_prefix("VK_VERSION_") {
                    Some(version) => Ok(format!("Vulkan {}", version.replace('_', "."))),
                    None => link(by),
                })
                .collect::<Result<_, _>>()?;
            format!(" as of {}", join_and(marked_by))
        }
    };
    Ok(format!(
        "Legacy functionality{what}: see [Legacy Functionality]({}) in the Vulkan specification.",
        spec.section_url(&mark.link)?
    ))
}

impl Specification {
    fn untitled_references<'a>(&self, text: &'a str) -> Vec<&'a str> {
        let references = self.markup.reference_without_a_text.captures_iter(text);
        references
            .map(|reference| reference.get(1).map_or("", |anchor| anchor.as_str()))
            .filter(|anchor| !self.titles.contains_key(*anchor))
            .collect()
    }

    fn every_untitled_reference(&self) -> BTreeSet<(String, String)> {
        let mut references = BTreeSet::new();
        for page in self.pages.values() {
            for text in [&page.description, &page.body] {
                for anchor in self.untitled_references(text) {
                    references.insert((page.name.clone(), anchor.to_string()));
                }
            }
        }
        for table in &self.tables {
            let cells = table.rows.iter().flatten().map(|cell| &cell.text);
            let texts = table.title.iter().chain(&table.header).chain(cells);
            for anchor in texts.flat_map(|text| self.untitled_references(text)) {
                let place = table.page.clone().unwrap_or(table.file.clone());
                references.insert((place, anchor.to_string()));
            }
        }
        let outside = self.value_descriptions.values().flatten();
        for description in outside.filter(|description| description.page.is_none()) {
            for anchor in self.untitled_references(&description.text) {
                references.insert((description.place(), anchor.to_string()));
            }
        }
        references
    }

    fn section_url(&self, anchor: &str) -> Result<String, String> {
        let files = self
            .anchors
            .get(anchor)
            .map(|files| files.iter().collect::<Vec<_>>())
            .unwrap_or_default();
        let [file] = files.as_slice() else {
            return Err(format!(
                "{} source files of the specification define the anchor `{anchor}`",
                files.len()
            ));
        };
        let page = file
            .strip_suffix(".adoc")
            .filter(|_| self.published.contains(*file))
            .ok_or_else(|| {
                format!("the anchor `{anchor}` is in `{file}`, which is not published as a page of its own")
            })?;
        Ok(format!("{SPECIFICATION}{page}.html#{anchor}"))
    }
}

impl Checker {
    fn legacy(&mut self, spec: &Specification, generated: &Generated, files: &[syn::File]) {
        let mut documented = Vec::new();
        for file in files {
            documented_items(generated, &file.items, &mut documented);
        }
        let mut targets = LinkTargets::new();
        for item in &documented {
            if let Some((c_name, path)) = &item.link_target {
                targets
                    .entry(c_name)
                    .or_insert((path.as_str(), &item.features));
            }
        }
        let keys = documented
            .iter()
            .map(Documented::key)
            .collect::<BTreeSet<_>>();

        let mut expected = BTreeMap::<(&'static str, String), (&LegacyMark, Vec<&str>)>::new();
        for ((kind, structure, name), mark) in &spec.legacy.marks {
            if let Err(problem) = spec.section_url(&mark.link) {
                self.fail(name, format!("is marked as legacy, and {problem}"));
            }
            if kind == "type"
                && spec.legacy.enumerations.contains(name)
                && spec.legacy.vendor_tag(name).is_some()
            {
                self.fail(name, "is a marked enumeration with a vendor tag: the generator of the specification takes every enumeration for core, and the bindings go by the name");
            }
            let shown = spec.legacy.shown(kind, structure, name, mark);
            if shown.is_empty() {
                let what = match spec.legacy.is_type_alias(kind, name) {
                    true => "type alias",
                    false => kind.as_str(),
                };
                let not_shown = format!("{what} by {}", mark.marked_by.join(" and "));
                *self.legacy_not_shown.entry(not_shown).or_default() += 1;
                continue;
            }
            *self.legacy_shown.entry(kind.clone()).or_default() += 1;
            let mark = (mark, shown);
            match kind.as_str() {
                "type" if LEGACY_MACROS_WITHOUT_AN_ITEM.contains(&name.as_str()) => {
                    if targets.contains_key(name.as_str()) {
                        self.fail(name, "is in LEGACY_MACROS_WITHOUT_AN_ITEM, but the bindings have an item for it");
                    }
                }
                "type" => {
                    expected.insert(("type", name.clone()), mark);
                }
                "command" => {
                    if keys.contains(&("wrapper", name.as_str())) {
                        expected.insert(("wrapper", name.clone()), mark.clone());
                    }
                    expected.insert(("raw", name.clone()), mark);
                }
                _ => {
                    let member = targets.get(structure.as_str()).and_then(|(owner, _)| {
                        keys.iter()
                            .filter(|(kind, _)| *kind == "member")
                            .filter_map(|(_, member)| {
                                member.strip_prefix(owner)?.strip_prefix("::")
                            })
                            .find(|member| is_rust_name_of(member, name))
                            .map(|member| format!("{owner}::{member}"))
                    });
                    match member {
                        Some(member) => {
                            expected.insert(("method", member.clone()), mark.clone());
                            expected.insert(("member", member), mark);
                        }
                        None => self.fail(
                            &format!("{structure}::{name}"),
                            "is marked as legacy, and the bindings have no such member",
                        ),
                    }
                }
            }
        }

        let mut found = BTreeSet::new();
        for item in &documented {
            let statements = legacy_statements(&item.doc);
            let key = (item.kind, item.name.clone());
            let described = format!("the {} `{}`", item.kind, item.name);
            let Some((mark, shown)) = expected.get(&key) else {
                if !statements.is_empty() {
                    self.fail(
                        &described,
                        "has a legacy statement, but the registry has no mark for it that the specification shows",
                    );
                }
                continue;
            };
            found.insert(key);
            match expected_legacy_statement(spec, &targets, &item.features, mark, shown) {
                Ok(expected) if statements == [expected.as_str()] => {
                    *self.legacy_statements.entry(item.kind).or_default() += 1;
                }
                Ok(expected) => self.fail(
                    &described,
                    format!("the registry marks it as legacy, so its documentation has the one paragraph `{expected}`, but it has {statements:?}"),
                ),
                Err(problem) => self.fail(&described, format!("is marked as legacy, and {problem}")),
            }
        }
        for (kind, name) in expected.keys().filter(|key| !found.contains(key)) {
            self.fail(
                &format!("the {kind} `{name}`"),
                "is marked as legacy, and the bindings document no such item",
            );
        }
        self.legacy_attributes(spec, &documented);
    }

    fn legacy_attributes(&mut self, spec: &Specification, documented: &[Documented]) {
        let paragraphs = |item: &Documented| item.doc.split("\n\n").map(flat).collect::<Vec<_>>();
        let values_named = |c_name: &str| {
            let values = documented.iter().filter(|item| item.kind == "value");
            values
                .filter(|item| item.aliases.first().is_some_and(|alias| alias == c_name))
                .collect::<Vec<_>>()
        };
        let mut stated = BTreeSet::<(&'static str, &str)>::new();
        let mut sharing = BTreeMap::<(&'static str, &str), BTreeSet<&str>>::new();
        let state = |checker: &mut Self, what: &'static str, item: &Documented, expected: &str| {
            let said = paragraphs(item)
                .iter()
                .filter(|paragraph| paragraph.as_str() == expected)
                .count();
            if said == 1 {
                *checker.legacy_attribute_statements.entry(what).or_default() += 1;
            } else {
                checker.fail(
                    &described_item(item),
                    format!("the registry calls it legacy, so its documentation has the one paragraph `{expected}`, but it has {said} of them: {:?}", paragraphs(item)),
                );
            }
        };
        let deprecated = &spec.deprecated;
        for ((element, owner, name), attribute) in &deprecated.attributes {
            let alias = attribute.alias.as_deref();
            match (element.as_str(), owner.as_str(), attribute.reason.as_str(), alias) {
                ("enum", "", "aliased", Some(alias)) => {
                    let is_a_macro_of_an_extension =
                        alias.ends_with("_EXTENSION_NAME") || alias.ends_with("_SPEC_VERSION");
                    let has = |c_name: &str| {
                        let mut names = documented.iter().flat_map(|item| &item.aliases);
                        names.any(|alias| alias == c_name)
                    };
                    if is_a_macro_of_an_extension && has(alias) && !has(name) {
                        let extension_macros = "the legacy alias of the name or of the version macro of an extension";
                        let left_out = &mut self.legacy_aliases_without_a_constant;
                        left_out.entry(extension_macros).or_default().insert(name.clone());
                    } else {
                        self.fail(name, format!("is a legacy alias of `{alias}`, and the bindings leave out only the legacy aliases of the name and of the version macro of an extension that they have, and have no item for them"));
                    }
                }
                ("enum", _, "aliased", Some(_)) => {
                    let constants = values_named(name);
                    let value = deprecated.value_of(name);
                    for constant in &constants {
                        let type_name = constant.name.split("::").next().unwrap_or_default();
                        let of_the_value = values_named(value)
                            .into_iter()
                            .find(|item| item.name.split("::").next() == Some(type_name));
                        let Some(of_the_value) = of_the_value else {
                            self.fail(&described_item(constant), format!("is a legacy alias of `{value}`, which has no constant of `{type_name}`"));
                            continue;
                        };
                        let target = of_the_value.name.rsplit("::").next().unwrap_or_default();
                        let expected = format!("`{name}` is a legacy alias of [`{value}`](Self::{target}).");
                        stated.insert(("value", constant.name.as_str()));
                        state(self, "legacy alias", constant, &expected);
                    }
                    if !constants.is_empty() {
                        continue;
                    }
                    let without_bit = |c_name: &str| c_name.replace("_BIT", "");
                    let other_names = deprecated.other_names(name).into_iter();
                    let same_rust_name = other_names
                        .filter(|other| without_bit(other) == without_bit(name))
                        .flat_map(values_named)
                        .collect::<Vec<_>>();
                    for constant in &same_rust_name {
                        sharing.entry(constant.key()).or_default().insert(name);
                    }
                    let reason = if !same_rust_name.is_empty() {
                        "another name of the value differs from it only by `_BIT`, which the Rust names of flag bits leave out, so the two have one constant"
                    } else if LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE.contains(&name.as_str())
                        && !values_named(value).is_empty()
                    {
                        "LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE"
                    } else {
                        self.fail(name, format!("is a legacy alias of `{value}` in `{owner}`, and the bindings have no constant for it"));
                        continue;
                    };
                    let left_out = &mut self.legacy_aliases_without_a_constant;
                    left_out.entry(reason).or_default().insert(name.clone());
                }
                ("enum", _, "true", None) => {
                    let constants = values_named(name);
                    if constants.is_empty() {
                        self.fail(name, format!("is legacy, and the bindings have no constant for it in `{owner}`"));
                    }
                    for constant in constants {
                        stated.insert(("value", constant.name.as_str()));
                        state(self, "legacy value", constant, LEGACY_WITHOUT_A_REASON);
                    }
                }
                ("member", _, "unused", None) => {
                    let structure = documented.iter().find(|item| {
                        item.kind == "type" && item.aliases.first().is_some_and(|alias| alias == owner)
                    });
                    let type_name = structure.and_then(|item| item.link_target.as_ref());
                    let type_name = type_name.map_or("", |(_, type_name)| type_name.as_str());
                    let member = documented.iter().find(|item| {
                        let field = item.name.strip_prefix(type_name);
                        let field = field.and_then(|field| field.strip_prefix("::"));
                        item.kind == "member" && field.is_some_and(|field| is_rust_name_of(field, name))
                    });
                    let Some(member) = member else {
                        self.fail(&format!("{owner}::{name}"), "is legacy and not used, and the bindings have no such member");
                        continue;
                    };
                    stated.insert(("member", member.name.as_str()));
                    if flat(&member.doc).contains(DESCRIBED_AS_NOT_USED) {
                        let described = "unused member that its description calls so";
                        *self.legacy_attribute_statements.entry(described).or_default() += 1;
                        if paragraphs(member).iter().any(|paragraph| paragraph == LEGACY_AND_NOT_USED) {
                            self.fail(&described_item(member), "says twice that it is legacy and not used");
                        }
                    } else {
                        state(self, "unused member", member, LEGACY_AND_NOT_USED);
                    }
                    let field = member.name.rsplit("::").next().unwrap_or_default();
                    let sets = format!("Sets [`{field}`](Self::{field})");
                    let setters = documented.iter().filter(|item| {
                        item.kind == "method"
                            && item.name.split("::").next() == Some(type_name)
                            && flat(&item.doc).starts_with(&sets)
                    });
                    let setters = setters.collect::<Vec<_>>();
                    if setters.is_empty() {
                        self.fail(&described_item(member), "is legacy and not used, and has no setter");
                    }
                    for setter in setters {
                        stated.insert(("method", setter.name.as_str()));
                        state(self, "setter of an unused member", setter, LEGACY_AND_NOT_USED);
                    }
                }
                _ => self.fail(name, format!("has the attribute `deprecated` as {attribute:?} on <{element}> of `{owner}`, which the test does not know how the bindings state")),
            }
        }
        for item in documented {
            let paragraphs = paragraphs(item);
            let names = sharing.get(&item.key()).cloned().unwrap_or_default();
            let names = names.into_iter().collect::<Vec<_>>();
            let expected = (!names.is_empty()).then(|| {
                let own = item.aliases.first().map_or("", String::as_str);
                let value = deprecated.value_of(own);
                let type_name = item.name.split("::").next();
                let of_the_value = values_named(value)
                    .into_iter()
                    .find(|other| other.name.split("::").next() == type_name);
                let target = of_the_value.map_or("", |other| {
                    other.name.rsplit("::").next().unwrap_or_default()
                });
                match own == value {
                    true => also_called(&names, &format!("`{value}`")),
                    false => also_called(&names, &format!("[`{value}`](Self::{target})")),
                }
            });
            let said = paragraphs
                .iter()
                .filter(|paragraph| paragraph.starts_with(ALSO_CALLED))
                .collect::<Vec<_>>();
            let other_names = item.aliases.get(1..).unwrap_or_default();
            let says_so = paragraphs.iter().any(|paragraph| {
                is_legacy_attribute_statement(paragraph) || paragraph.contains("legacy alias")
            });
            if said != expected.iter().collect::<Vec<_>>()
                || (item.kind == "value" && other_names != names)
            {
                let paragraph = expected
                    .map_or("no paragraph about such names".to_string(), |expected| {
                        format!("the one paragraph `{expected}`")
                    });
                self.fail(&described_item(item), format!("the legacy aliases of the registry that have its Rust name and no constant of their own are {names:?}, so it has these as `#[doc(alias)]` after its own name and {paragraph}, but it has the names {other_names:?} and the paragraphs {said:?}"));
            } else if expected.is_some() {
                let shared = "constant that legacy aliases share";
                *self.legacy_attribute_statements.entry(shared).or_default() += 1;
            } else if says_so && !stated.contains(&item.key()) {
                self.fail(&described_item(item), "says that the registry calls it legacy, but the registry has no `deprecated` attribute for it");
            }
        }
    }
}

fn described_item(item: &Documented) -> String {
    format!("the {} `{}`", item.kind, item.name)
}

fn documented_field(
    owner: &Documented,
    owner_ident: &syn::Ident,
    field: &syn::Field,
) -> Documented {
    let ident = field.ident.as_ref().expect("a named field");
    Documented {
        kind: "member",
        name: format!("{owner_ident}::{ident}"),
        features: owner.features.clone(),
        doc: doc_attributes(&field.attrs).0,
        aliases: Vec::new(),
        link_target: None,
    }
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
    specification_links: usize,
    legacy_statements: BTreeMap<&'static str, usize>,
    legacy_shown: BTreeMap<String, usize>,
    legacy_not_shown: BTreeMap<String, usize>,
    described_outside_their_page: Vec<String>,
    legacy_attribute_statements: BTreeMap<&'static str, usize>,
    legacy_aliases_without_a_constant: BTreeMap<&'static str, BTreeSet<String>>,
    links_of_untitled_anchors: usize,
    untitled_anchors: Vec<String>,
    described_without_their_name_first: Vec<String>,
    described_under_an_alias: Vec<String>,
    rows_taken: Vec<String>,
}

struct Generated {
    legacy_link: Regex,
    called_command: Regex,
    cfg_feature: Regex,
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
            legacy_link: regex(&format!(
                r"\[Legacy Functionality\]\({}[\w/-]+\.html#[\w.-]+\)",
                regex::escape(SPECIFICATION)
            )),
            called_command: regex(r"Calls the C command (?:\[`(\w+)`\]|`(\w+)`)"),
            cfg_feature: regex(r#"feature\s*=\s*"([^"]+)""#),
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
        let specification_links = generated.legacy_link.find_iter(&flat(doc)).count();
        self.specification_links += specification_links;
        let anchor_links = spec
            .markup
            .link_of_an_untitled_anchor
            .find_iter(doc)
            .count();
        self.links_of_untitled_anchors += anchor_links;
        let other = doc
            .match_indices("docs.vulkan.org")
            .count()
            .saturating_sub(generated.reference_link.captures_iter(doc).count())
            .saturating_sub(specification_links)
            .saturating_sub(anchor_links);
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
        let untitled = spec.untitled_references(&source);
        for anchor in &untitled {
            let link = spec
                .section_url(anchor)
                .map(|url| format!("[\\[{anchor}\\]]({url})"));
            match link {
                Ok(link) if doc.contains(&link) => {
                    self.untitled_anchors.push(format!("{item}: {anchor}"));
                }
                Ok(link) => self.fail(
                    item,
                    format!("the description of `{name}` refers to the anchor `{anchor}`, which has no title, so the specification shows the link `[{anchor}]`, but the documentation has no `{link}`"),
                ),
                Err(problem) => self.fail(
                    item,
                    format!("the description of `{name}` refers to the anchor `{anchor}`, which has no title, and {problem}"),
                ),
            }
        }
        for link in spec.markup.link_of_an_untitled_anchor.captures_iter(doc) {
            if !untitled.contains(&&link[1]) {
                self.fail(
                    item,
                    format!("the description of `{name}` links the anchor `{}`, which its list item on the page `{page}` does not refer to without a text, or which has a title", &link[1]),
                );
            }
        }
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
            let doc = without_legacy_statements(&doc);
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
                let texts = spec.described_without_its_name_first(page, "pname", name);
                if !texts.is_empty() {
                    self.item_without_the_name_first(spec, &item, (page, name), None, &texts);
                }
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
        let described_without_its_name_first = spec
            .unnamed_descriptions
            .get(page)
            .into_iter()
            .flatten()
            .map(|(name, _)| name.as_str())
            .filter(|name| is_rust_name_of(field, name))
            .find(|name| !spec.describes(page, "pname", name))
            .filter(|_| !generated.described_under_another_name.is_match(doc));
        if let Some(name) = described_without_its_name_first {
            let texts = spec.described_without_its_name_first(page, "pname", name);
            self.item_without_the_name_first(spec, &item, (page, name), Some(doc), &texts);
            return;
        }
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

    fn item_without_the_name_first(
        &mut self,
        spec: &Specification,
        item: &str,
        (page, name): (&str, &str),
        doc: Option<&str>,
        texts: &[&str],
    ) {
        let Some(doc) = doc else {
            self.fail(item, format!("the specification describes `{name}` on the page `{page}` in a list item that does not start with the name, but the documentation does not use it"));
            return;
        };
        let [text] = texts else {
            self.fail(
                item,
                format!(
                    "is described, but {} list items of the page `{page}` describe `{name}` without starting with the name",
                    texts.len()
                ),
            );
            return;
        };
        self.described_without_their_name_first
            .push(format!("{page}::{name}"));
        if generated_words(doc) != spec.words(text) {
            self.fail(
                item,
                format!(
                    "`{doc}` is not the list item of the page `{page}` that describes `{name}`: `{}`",
                    flat(text)
                ),
            );
        }
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
        let [c_name, ..] = aliases else {
            return;
        };
        let is_an_alias = doc.starts_with(&format!("`{c_name}` is an alias of [`"));
        if let (false, Some(page)) = (is_an_alias, owner_page) {
            let texts = spec.described_without_its_name_first(page, "ename", c_name);
            if !texts.is_empty() {
                let described = !generated.value_of.is_match(doc);
                match described {
                    true => self.described_values += 1,
                    false => self.undescribed_values += 1,
                }
                let doc = described.then_some(doc.as_str());
                self.item_without_the_name_first(spec, item, (page, c_name), doc, &texts);
                return;
            }
        }
        if let Some(names) = generated.described_under_another_name.captures(doc) {
            self.described_values += 1;
            let (own, other) = (&names[1], &names[2]);
            let description = &doc[names[0].len()..];
            let Some(page) = owner_page else {
                self.fail(item, "is described although its type has no reference page");
                return;
            };
            if own != c_name {
                self.fail(
                    item,
                    format!("is documented with the description of `{own}`"),
                );
            }
            let is_an_alias = spec
                .value_aliases
                .get(c_name.as_str())
                .is_some_and(|aliases| aliases.contains(other));
            if !is_an_alias {
                self.fail(item, format!("the documentation takes the description of `{other}`, which the registry does not have as an alias of `{c_name}`"));
            }
            if spec.describes(page, "ename", c_name) {
                self.fail(item, format!("the specification describes `{c_name}` on the page `{page}` under its own name, but the documentation takes the description of its alias `{other}`"));
            }
            if !spec.describes(page, "ename", other) {
                self.fail(item, format!("the documentation takes the description of `{other}` for `{c_name}`, but the specification describes no `{other}` on the page `{page}`"));
            }
            if !generated.names_at_start(description).contains(&other) {
                self.fail(item, format!("the description that the documentation takes for `{c_name}` is not the one of `{other}`"));
            }
            if !spec.table_rows(Some(page), values, c_name).is_empty() {
                self.fail(item, format!("the specification describes `{c_name}` in a table row, which comes before the description under its alias name `{other}`"));
            }
            if let [elsewhere, ..] = spec.described_outside_the_page(page, c_name) {
                self.fail(item, format!("the specification describes `{c_name}` in a list item of {}, which comes before the description under its alias name `{other}`", elsewhere.place()));
            }
            if spec.values_of_structures_and_handles.contains(c_name) {
                self.fail(item, format!("the registry has a structure or a handle for `{c_name}`, which comes before the description under its alias name `{other}`"));
            }
            self.contained(spec, item, page, description);
            self.whole(spec, item, page, ("ename", other), description);
            self.described_under_an_alias
                .push(format!("{page}::{c_name} under {other}"));
            return;
        }
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
            || rest.starts_with(" is a legacy alias of [`")
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
            let under_an_alias = spec
                .value_aliases
                .get(c_name.as_str())
                .into_iter()
                .flatten()
                .find(|alias| spec.describes(page, "ename", alias));
            if let Some(alias) = under_an_alias {
                self.fail(item, format!("the specification describes `{c_name}` on the page `{page}` under its alias name `{alias}`, but the documentation does not use it"));
            }
            let elsewhere = spec.described_outside_the_page(page, c_name);
            if !elsewhere.is_empty() {
                let places = elsewhere
                    .iter()
                    .map(|description| description.place())
                    .collect::<Vec<_>>();
                self.fail(item, format!("the specification describes `{c_name}` outside the page `{page}` ({}), but the documentation does not use it", places.join(", ")));
            }
            return;
        }
        self.described_values += 1;
        if rest.starts_with(" in the table “")
            || rest.starts_with(" in a table of the specification:")
        {
            self.tabulated(spec, generated, owner, item, c_name, raw);
            let (on_the_page, _) = spec.table_rows_and_their_place(owner_page, values, c_name);
            let elsewhere = spec.described_outside_the_page(owner_page.unwrap_or_default(), c_name);
            if let (false, [description]) = (on_the_page, elsewhere) {
                self.fail(item, format!("the specification describes `{c_name}` in a list item of {}, which comes before the row of a table outside the page of its type", description.place()));
            }
            return;
        }
        let elsewhere = spec.described_outside_the_page(owner_page.unwrap_or_default(), c_name);
        match (owner_page, elsewhere) {
            (None, _) => self.fail(item, "is described although its type has no reference page"),
            (Some(page), [description]) => {
                let (on_the_page, rows) =
                    spec.table_rows_and_their_place(Some(page), values, c_name);
                if on_the_page && !rows.is_empty() {
                    self.fail(item, format!("the specification describes `{c_name}` in a table row of the page `{page}`, which comes before the list item of {}", description.place()));
                }
                self.described_outside_their_page
                    .push(format!("{c_name}: {}", description.place()));
                let expected = spec.words(&description.text);
                if generated_words(doc) != expected {
                    self.fail(
                        item,
                        format!(
                            "`{doc}` is not the list item of {} that describes `{c_name}`: `{}`",
                            description.place(),
                            flat(&description.text)
                        ),
                    );
                }
            }
            (Some(_), [_, _, ..]) => self.fail(
                item,
                format!(
                    "is described, but {} list items outside the page of its type describe `{c_name}`",
                    elsewhere.len()
                ),
            ),
            (Some(page), []) => {
                self.contained(spec, item, page, doc);
                self.whole(spec, item, page, ("ename", c_name), doc);
                self.not_valid_usage(spec, item, page, doc);
            }
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
            if let Some(title) = captures.get(2) {
                self.rows_taken
                    .push(format!("{c_name}: {}", title.as_str()));
            }
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

const DESCRIBED_BY_AN_ITEM_THAT_STARTS_WITH_NO_NAME: &[&str] = &[
    "VkGeometryTypeKHR::VK_GEOMETRY_TYPE_MICROMAP_KHR",
    "VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR::triStripVertexOrderIndependentOfProvokingVertex",
    "VkPhysicalDeviceMaintenance9PropertiesKHR::image2DViewOf3DSparse",
    "VkPhysicalDeviceVulkan12Features::subgroupBroadcastDynamicId",
    "VkPipelineCreateFlagBits::VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT",
    "VkPipelineCreateFlagBits2::VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT",
];

const TABLES_WITH_ROWS_FROM_THE_REGISTRY: [&str; 2] =
    ["Supported Pipeline Stage Flags", "Supported Access Types"];

const ROWS_FROM_THE_REGISTRY: &[&str] =
    &["VK_PIPELINE_STAGE_2_DATA_GRAPH_BIT_ARM: Supported Pipeline Stage Flags"];

const DESCRIBED_UNDER_AN_ALIAS: &[&str] = &[
    "VkImageCreateFlagBits::VK_IMAGE_CREATE_DESCRIPTOR_HEAP_CAPTURE_REPLAY_BIT_EXT under VK_IMAGE_CREATE_DESCRIPTOR_BUFFER_CAPTURE_REPLAY_BIT_EXT",
];

const DESCRIBED_OUTSIDE_THEIR_PAGE: &[&str] = &[
    "VK_FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_BIT_ARM: chapters/formats.adoc outside the reference pages",
    "VK_FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_BIT_ARM: chapters/formats.adoc outside the reference pages",
    "VK_FORMAT_FEATURE_2_TENSOR_SHADER_BIT_ARM: chapters/formats.adoc outside the reference pages",
    "VK_PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_BIT_EXT: the page VkPipelineStageFlagBits",
    "VK_VIDEO_ENCODE_CAPABILITY_EMPHASIS_MAP_BIT_KHR: chapters/video/encode.adoc outside the reference pages",
    "VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR: chapters/video/encode.adoc outside the reference pages",
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
    let entries = |kind: &str| spec.legacy.entries.get(kind).copied().unwrap_or(0);
    assert_eq!(
        [
            entries("type"),
            entries("command"),
            entries("feature"),
            spec.legacy.entries_of_other_apis
        ],
        [185, 116, 2, 0],
        "the <deprecate> blocks of the versions and extensions of the vulkan API list this many types, commands and features, and the last number is what the blocks of the other versions and extensions list, which the bindings leave out with them"
    );
    assert_eq!(
        ["type", "command", "feature"].map(|kind| spec.legacy.names(kind)),
        [128, 91, 2],
        "this many different types, commands and features are marked as legacy, some of them by several versions and extensions"
    );
    assert_eq!(
        ["type", "command", "feature"].map(|kind| checker
            .legacy_shown
            .get(kind)
            .copied()
            .unwrap_or(0)),
        [83, 61, 2],
        "the specification shows this many of the marked types, commands and features as legacy: those that a version marks, or an extension that is at least as promoted as they are (core, then KHR, then EXT, then a vendor), and that are not type aliases"
    );
    assert_eq!(
        checker
            .legacy_not_shown
            .iter()
            .map(|(marked, count)| (marked.as_str(), *count))
            .collect::<Vec<_>>(),
        [
            ("command by VK_EXT_descriptor_heap", 18),
            ("command by VK_KHR_device_address_commands", 12),
            ("type alias by VK_EXT_descriptor_heap", 4),
            ("type by VK_EXT_descriptor_heap", 31),
            ("type by VK_KHR_device_address_commands", 6),
            ("type by VK_KHR_extended_flags", 4),
        ],
        "the registry marks these as legacy and the specification does not show it, so the bindings do not state it either: only an extension that is less promoted than they are marks them, or they are type aliases, which the specification lists as a typedef without a warning"
    );
    assert_eq!(
        ["type", "raw", "wrapper", "member", "method"].map(|kind| checker
            .legacy_statements
            .get(kind)
            .copied()
            .unwrap_or(0)),
        [83 - LEGACY_MACROS_WITHOUT_AN_ITEM.len(), 61, 61, 2, 2],
        "the legacy statement is in the documentation of every type that is shown as legacy except the version macros that the bindings do not have, because they have the macros that supersede them; of the raw method and the wrapper of every such command; and of the member and the setter of every such feature"
    );
    assert_eq!(
        checker.specification_links,
        checker.legacy_statements.values().sum::<usize>(),
        "the legacy statements are the only links into the specification"
    );
    assert_eq!(
        checker
            .described_outside_their_page
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        DESCRIBED_OUTSIDE_THEIR_PAGE
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these values are documented with a list item from outside the reference page of their type, which has no item for them. An item out there can be about one use of a value, so read the item before listing a value here"
    );
    let untitled = |page: &str, anchor: &str| (page.to_string(), anchor.to_string());
    assert_eq!(
        spec.every_untitled_reference(),
        BTreeSet::from([
            untitled("Required_Limits", "VK_EXT_descriptor_buffer"),
            untitled("Required_Limits", "VK_KHR_device_fault"),
            untitled("VkPhysicalDeviceLimits", "samplers-maxAnisotropy"),
            untitled("VkPhysicalDeviceLimits", "samplers-mipLodBias"),
        ]),
        "the cross-references without a text to an anchor that the sources give no title, in all the reference pages and tables and in the descriptions of values outside the reference pages. The two of `VkPhysicalDeviceLimits` are in the descriptions of members. The two of `Required_Limits` are in a table of required limits that describes no item of the bindings, and name the appendix of an extension, whose title is generated when the specification is built"
    );
    assert_eq!(
        checker.untitled_anchors,
        [
            "PhysicalDeviceLimits::max_sampler_lod_bias: samplers-mipLodBias",
            "PhysicalDeviceLimits::max_sampler_anisotropy: samplers-maxAnisotropy",
        ],
        "the descriptions that link such an anchor"
    );
    assert_eq!(
        checker.links_of_untitled_anchors,
        checker.untitled_anchors.len(),
        "no other documentation links an anchor by its name"
    );
    let deprecated = &spec.deprecated;
    assert_eq!(
        ["aliased", "true", "unused"].map(|reason| deprecated
            .entries
            .get(reason)
            .copied()
            .unwrap_or(0)),
        [43, 1, 2],
        "the elements of vk.xml and video.xml that have the attribute `deprecated` for the vulkan API: 42 legacy aliases in vk.xml and 1 in video.xml, 1 legacy value and 2 unused members"
    );
    assert_eq!(
        (
            deprecated.attributes.len(),
            deprecated.entries_of_other_apis
        ),
        (46, 0),
        "each name has the attribute once, and no element of another API has it"
    );
    assert_eq!(
        checker
            .legacy_attribute_statements
            .iter()
            .map(|(what, count)| (*what, *count))
            .collect::<Vec<_>>(),
        [
            ("constant that legacy aliases share", 30),
            ("legacy alias", 21),
            ("legacy value", 1),
            ("setter of an unused member", 2),
            ("unused member that its description calls so", 2),
        ],
        "17 legacy aliases have a constant, 4 of them a second one in the bitmask type of their flag bits; 15 more have the Rust name of another name of their value, whose constant names them, in the type of the flag bits and in its bitmask type. A member that the registry calls unused has the paragraph `Legacy functionality that is not used.` unless its description says `is legacy and not used`: when `unused member` appears here, read the description of that member in the specification, which does not say so"
    );
    let without_a_constant = |names: &[&str]| names.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        checker.legacy_aliases_without_a_constant,
        BTreeMap::from([
            (
                "LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE",
                without_a_constant(LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE)
            ),
            (
                "another name of the value differs from it only by `_BIT`, which the Rust names of flag bits leave out, so the two have one constant",
                without_a_constant(&[
                    "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DATA_ACCESS_KHR",
                    "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISABLE_OPACITY_MICROMAPS_EXT",
                    "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV",
                    "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT",
                    "VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_OPACITY_MICROMAP_UPDATE_EXT",
                    "VK_GEOMETRY_INSTANCE_DISABLE_OPACITY_MICROMAPS_EXT",
                    "VK_GEOMETRY_INSTANCE_FORCE_OPACITY_MICROMAP_2_STATE_EXT",
                    "VK_HOST_IMAGE_COPY_MEMCPY",
                    "VK_HOST_IMAGE_COPY_MEMCPY_EXT",
                    "VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR",
                    "VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR",
                    "VK_PIPELINE_CREATE_DISPATCH_BASE",
                    "VK_PIPELINE_CREATE_DISPATCH_BASE_KHR",
                    "VK_RESOLVE_MODE_EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID",
                    "VK_SURFACE_COUNTER_VBLANK_EXT",
                ])
            ),
            (
                "the legacy alias of the name or of the version macro of an extension",
                without_a_constant(&[
                    "VK_GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME",
                    "VK_GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION",
                    "VK_KHR_MAINTENANCE1_EXTENSION_NAME",
                    "VK_KHR_MAINTENANCE1_SPEC_VERSION",
                    "VK_KHR_MAINTENANCE2_EXTENSION_NAME",
                    "VK_KHR_MAINTENANCE2_SPEC_VERSION",
                    "VK_KHR_MAINTENANCE3_EXTENSION_NAME",
                    "VK_KHR_MAINTENANCE3_SPEC_VERSION",
                    "VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME",
                    "VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION",
                ])
            ),
        ]),
        "the legacy aliases that the bindings have no constant of their own for: the Vulkan Video one because the registry data that the bindings are generated from has no aliases of the values of the Vulkan Video headers; 15 flag bits because the Rust name of the alias is the Rust name of another name of the same value, whose constant names the alias in a paragraph and as a `#[doc(alias)]`; 10 macros of extensions because the bindings have one constant for the name and one for the version of an extension"
    );
    assert_eq!(
        checker
            .described_without_their_name_first
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        DESCRIBED_BY_AN_ITEM_THAT_STARTS_WITH_NO_NAME
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these members and values are documented with a list item of their page that does not start with their name: it has the anchor of the feature or limit and names the member, or the value is the first name in it and `specifies` follows, or the item starts with the name of the value without its macro. Read an item before listing what it describes here"
    );
    assert_eq!(
        checker
            .described_under_an_alias
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        DESCRIBED_UNDER_AN_ALIAS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these values are documented with the list item that their page has under a name that the registry gives them as an alias"
    );
    for title in TABLES_WITH_ROWS_FROM_THE_REGISTRY {
        let rows = spec
            .tables
            .iter()
            .filter(|table| table.title.as_deref() == Some(title))
            .map(|table| table.rows.len())
            .collect::<Vec<_>>();
        assert!(
            matches!(rows[..], [rows] if rows >= 40),
            "the table “{title}” is read {} times, with {rows:?} rows",
            rows.len()
        );
    }
    assert_eq!(
        checker
            .rows_taken
            .iter()
            .map(String::as_str)
            .filter(|row| TABLES_WITH_ROWS_FROM_THE_REGISTRY
                .iter()
                .any(|title| row.ends_with(&format!(": {title}"))))
            .collect::<BTreeSet<_>>(),
        ROWS_FROM_THE_REGISTRY
            .iter()
            .copied()
            .collect::<BTreeSet<_>>(),
        "exactly these values are documented with their row of a table whose rows the registry gives"
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
    checker.legacy(spec, generated, files);
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
                            without_legacy_statements(&doc).trim(),
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
        value_descriptions: BTreeMap::new(),
        unnamed_descriptions: BTreeMap::new(),
        value_aliases: BTreeMap::from([(
            "VK_COLOR_CINNABAR".to_string(),
            BTreeSet::from(["VK_COLOR_VERMILION".to_string()]),
        )]),
        values_of_structures_and_handles: BTreeSet::new(),
        tables: fixture_tables(),
        extensions: BTreeSet::new(),
        attributes: BTreeMap::from([("times".to_string(), "×".to_string())]),
        titles: BTreeMap::new(),
        anchors: BTreeMap::new(),
        published: BTreeSet::new(),
        legacy: LegacyRegistry::default(),
        deprecated: DeprecatedAttributes::default(),
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
  * [[limits-margin]] When the <<limits-range,pname:range>> is laid out,
    pname:margin indicates the space around it.
  * [[limits-padding]] Things are padded and xpname:padding is no name.
  * [[things-padding]] The pname:padding is not described here.
  * A late anchor [[limits-padding]] of pname:padding is no anchor of the item.

A thing is empty with the following properties:

  * pname:elsewhere equal to `0`
  * ename:VK_COLOR_UMBER specifies umber,
    on the page of the things.
  * ename:VK_COLOR_OCHRE is set.

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
  ** ename:VK_COLOR_GOLD
  * VK_COLOR_IVORY specifies ivory,
    without its macro.
  * When things are dark, ename:VK_COLOR_EBONY specifies ebony:
  ** darker than ename:VK_COLOR_IVORY
  * Otherwise, ename:VK_COLOR_OCHRE is set.
  * With ename:VK_COLOR_RED, ename:VK_COLOR_OCHRE specifies nothing.
  * VK_COLOR_OCHRE is set.
  * ename:VK_COLOR_VERMILION specifies a red.",
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
    let mut segments = vec![fixture_segment(None, FIXTURE_RUNNING_TEXT)];
    for (name, description, body) in pages {
        segments.push(fixture_segment(Some(name), body));
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
    specification.value_descriptions = specification.value_descriptions_in(&segments);
    specification.unnamed_descriptions = specification.unnamed_descriptions_of_the_pages();
    specification
}

fn fixture_segment(page: Option<&str>, text: &str) -> Segment {
    Segment {
        page: page.map(str::to_string),
        file: FIXTURE_FILE.to_string(),
        lines: text.lines().map(str::to_string).collect(),
    }
}

const FIXTURE_RUNNING_TEXT: &str = "
Things are drawn in the following colors:

  * ename:VK_COLOR_AMBER
    specifies amber, outside the reference pages:
  ** a color of things
  * ename:VK_COLOR_OCHRE
  * ename:VK_COLOR_OCHRE, which specifies no color
  * ename:VK_COLOR_RED specifies red where things are drawn.
  * ename:VK_COLOR_BLUE specifies blue where things are drawn.

|====
|Colors |Use
a| * ename:VK_COLOR_OCHRE specifies no color in the first item of a cell
  * ename:VK_COLOR_OCHRE specifies no color in the last item of a cell
| any
|====

.Valid Usage
****
  * [[VUID-vkCountThings-color-00002]]
    ename:VK_COLOR_OCHRE specifies a requirement
****";

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
    /// When the range is laid out, `margin` indicates the space around it.
    pub margin: u32,
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
    /// `VK_COLOR_AMBER` specifies amber, outside the reference pages:
    ///
    /// - a color of things
    #[doc(alias = "VK_COLOR_AMBER")]
    pub const AMBER: Self = Self(17);
    /// `VK_COLOR_UMBER` specifies umber, on the page of the things.
    #[doc(alias = "VK_COLOR_UMBER")]
    pub const UMBER: Self = Self(18);
    /// `VK_COLOR_OCHRE`, a value of [`VkColor`](PAGES/VkColor.html).
    #[doc(alias = "VK_COLOR_OCHRE")]
    pub const OCHRE: Self = Self(19);
    /// `VK_COLOR_IVORY` specifies ivory, without its macro.
    #[doc(alias = "VK_COLOR_IVORY")]
    pub const IVORY: Self = Self(20);
    /// When things are dark, `VK_COLOR_EBONY` specifies ebony:
    ///
    /// - darker than [`VK_COLOR_IVORY`](Color::IVORY)
    #[doc(alias = "VK_COLOR_EBONY")]
    pub const EBONY: Self = Self(21);
    /// `VK_COLOR_CINNABAR`, which the specification describes under the name `VK_COLOR_VERMILION`:
    /// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.
    #[doc(alias = "VK_COLOR_CINNABAR")]
    pub const CINNABAR: Self = Self(22);
    /// `VK_COLOR_VERMILION` is an alias of [`VK_COLOR_CINNABAR`](Self::CINNABAR).
    #[doc(alias = "VK_COLOR_VERMILION")]
    pub const VERMILION: Self = Self(22);
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
fn the_rows_of_the_tables_of_stages_and_access_flags_read_as_the_registry_lists_them() {
    let registry = r#"
        <enums name="VkAccessFlagBits2" type="bitmask" bitwidth="64">
            <enum bitpos="0" name="VK_ACCESS_2_A_BIT"/>
            <enum bitpos="1" name="VK_ACCESS_2_B_BIT"/>
        </enums>
        <enums name="VkPipelineStageFlagBits2" type="bitmask" bitwidth="64">
            <enum value="0" name="VK_PIPELINE_STAGE_2_NONE"/>
            <enum bitpos="0" name="VK_PIPELINE_STAGE_2_A_BIT"/>
        </enums>
        <extension name="VK_KHR_c" number="3" supported="vulkan,vulkansc">
            <require>
                <enum bitpos="2" extends="VkAccessFlagBits2" name="VK_ACCESS_2_C_BIT_KHR"/>
                <enum bitpos="1" extends="VkPipelineStageFlagBits2" name="VK_PIPELINE_STAGE_2_C_BIT_KHR"/>
            </require>
        </extension>
        <extension name="VK_KHR_d" number="4" supported="vulkansc">
            <require>
                <enum bitpos="3" extends="VkAccessFlagBits2" name="VK_ACCESS_2_D_BIT_KHR"/>
                <enum bitpos="3" extends="VkPipelineStageFlagBits2" name="VK_PIPELINE_STAGE_2_D_BIT_KHR"/>
            </require>
        </extension>
        <sync>
            <syncstage name="VK_PIPELINE_STAGE_2_NONE"/>
            <syncstage name="VK_PIPELINE_STAGE_2_A_BIT" alias="VK_PIPELINE_STAGE_A_BIT">
            </syncstage>
            <syncstage name="VK_PIPELINE_STAGE_2_D_BIT_KHR">
                <syncsupport queues="VK_QUEUE_X_BIT"/>
            </syncstage>
            <syncstage name="VK_PIPELINE_STAGE_2_C_BIT_KHR">
                <syncsupport queues="VK_QUEUE_X_BIT,VK_QUEUE_Y_BIT" />
                <syncequivalent stage="VK_PIPELINE_STAGE_2_A_BIT"/>
            </syncstage>
            <syncaccess name="VK_ACCESS_2_A_BIT" alias="VK_ACCESS_A_BIT">
            </syncaccess>
            <syncaccess name="VK_ACCESS_2_D_BIT_KHR">
                <syncsupport stage="VK_PIPELINE_STAGE_2_A_BIT"/>
            </syncaccess>
            <syncaccess name="VK_ACCESS_2_B_BIT">
                <syncsupport stage="VK_PIPELINE_STAGE_2_C_BIT_KHR,VK_PIPELINE_STAGE_2_D_BIT_KHR,VK_PIPELINE_STAGE_2_A_BIT"/>
            </syncaccess>
            <syncaccess name="VK_ACCESS_2_C_BIT_KHR" alias="VK_ACCESS_C_BIT_KHR">
                <syncsupport stage="VK_PIPELINE_STAGE_2_A_BIT,VK_PIPELINE_STAGE_2_D_BIT_KHR"/>
            </syncaccess>
        </sync>
    "#;
    let rows = sync_table_rows(registry);
    assert_eq!(
        rows["supportedPipelineStages"],
        [
            "|ename:VK_PIPELINE_STAGE_2_NONE | None required",
            "|ename:VK_PIPELINE_STAGE_2_A_BIT | None required",
            "|ename:VK_PIPELINE_STAGE_2_C_BIT_KHR | ename:VK_QUEUE_X_BIT or ename:VK_QUEUE_Y_BIT",
        ]
    );
    assert_eq!(
        rows["supportedAccessTypes"],
        [
            "|ename:VK_ACCESS_2_A_BIT, +",
            " ename:VK_ACCESS_A_BIT |",
            "\tAny",
            "|ename:VK_ACCESS_2_B_BIT |",
            "\tename:VK_PIPELINE_STAGE_2_C_BIT_KHR,",
            "\tename:VK_PIPELINE_STAGE_2_A_BIT",
            "|ename:VK_ACCESS_2_C_BIT_KHR, +",
            " ename:VK_ACCESS_C_BIT_KHR |",
            "\tename:VK_PIPELINE_STAGE_2_A_BIT,",
        ],
        "the comma of a stage stays where the stage after it is not in the API, as in the files of the specification"
    );
    assert_eq!(rows.len(), 2);
}

#[test]
fn checker_reports_a_row_of_a_table_outside_the_page_taken_before_a_list_item_elsewhere() {
    let mut specification = fixture_specification();
    let item = fixture_segment(
        None,
        "  * ename:VK_COLOR_TEAL specifies teal where things are drawn.",
    );
    specification
        .value_descriptions
        .extend(specification.value_descriptions_in(&[item]));
    let failures = |source: &str| {
        let source = source.replace("PAGES/", REFERENCE_PAGES);
        let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
        let mut checker = Checker::default();
        check_files(&specification, &Generated::new(), &mut checker, &[file]);
        checker.failures
    };
    assert_eq!(
        failures(FIXTURE_SOURCE),
        [
            "Color::TEAL: the specification describes `VK_COLOR_TEAL` in a list item of chapters/things.adoc outside the reference pages, which comes before the row of a table outside the page of its type"
        ]
    );
    let row = "/// `VK_COLOR_TEAL` in the table “Teal Shades” of the specification:\n    ///\n    /// - Use: sea\n";
    assert_eq!(FIXTURE_SOURCE.matches(row).count(), 1);
    let described = FIXTURE_SOURCE.replacen(
        row,
        "/// `VK_COLOR_TEAL` specifies teal where things are drawn.\n",
        1,
    );
    assert_eq!(failures(&described), Vec::<String>::new());
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
fn checker_collects_the_values_that_a_list_item_outside_their_page_describes() {
    let checker = fixture_checker(FIXTURE_SOURCE);
    assert_eq!(
        checker.described_outside_their_page,
        [
            "VK_COLOR_AMBER: chapters/things.adoc outside the reference pages",
            "VK_COLOR_UMBER: the page VkThing",
        ]
    );
    let descriptions = fixture_specification()
        .value_descriptions
        .into_iter()
        .map(|(value, descriptions)| {
            let found = descriptions
                .into_iter()
                .map(|description| (description.place(), description.text))
                .collect::<Vec<_>>();
            (value, found)
        })
        .collect::<Vec<_>>();
    let on_the_page = |page: &str| format!("the page {page}");
    let outside_pages = || "chapters/things.adoc outside the reference pages".to_string();
    let list_item = |lines: &[&str]| lines.join("\n");
    assert_eq!(
        descriptions,
        [
            (
                "VK_COLOR_AMBER".to_string(),
                vec![(
                    outside_pages(),
                    list_item(&[
                        "  * ename:VK_COLOR_AMBER",
                        "    specifies amber, outside the reference pages:",
                        "  ** a color of things",
                    ])
                )]
            ),
            (
                "VK_COLOR_BLUE".to_string(),
                vec![(
                    outside_pages(),
                    list_item(&["  * ename:VK_COLOR_BLUE specifies blue where things are drawn."])
                )]
            ),
            (
                "VK_COLOR_GREEN".to_string(),
                vec![(
                    on_the_page("VkColor"),
                    list_item(&["  * ename:VK_COLOR_GREEN specifies green."])
                )]
            ),
            (
                "VK_COLOR_RED".to_string(),
                vec![
                    (
                        outside_pages(),
                        list_item(&[
                            "  * ename:VK_COLOR_RED specifies red where things are drawn."
                        ])
                    ),
                    (
                        on_the_page("VkColor"),
                        list_item(&["  * ename:VK_COLOR_RED specifies red."])
                    ),
                ]
            ),
            (
                "VK_COLOR_UMBER".to_string(),
                vec![(
                    on_the_page("VkThing"),
                    list_item(&[
                        "  * ename:VK_COLOR_UMBER specifies umber,",
                        "    on the page of the things.",
                    ])
                )]
            ),
            (
                "VK_COLOR_VERMILION".to_string(),
                vec![(
                    on_the_page("VkColor"),
                    list_item(&["  * ename:VK_COLOR_VERMILION specifies a red."])
                )]
            ),
        ],
        "an item that names a value in a list, in valid usage, among other words or in a cell of a table describes none"
    );
}

#[test]
fn the_aliases_of_a_value_are_the_names_that_the_registry_gives_it_through_other_aliases_too() {
    let aliases = value_aliases(
        r#"<enums name="VkFace"><enum value="0" name="VK_FACE_FRONT"/>
        <enum name="VK_FACE_FRONT_KHR" alias="VK_FACE_FRONT"/>
        <enum alias="VK_FACE_FRONT_KHR" name="VK_FACE_FRONT_EXT"/></enums>
        <require><enum extends="VkFace" name="VK_FACE_BACK_NV" alias="VK_FACE_BACK"/></require>"#,
    );
    let names = |value: &str| {
        aliases
            .get(value)
            .into_iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<_>>()
    };
    assert_eq!(
        names("VK_FACE_FRONT"),
        ["VK_FACE_FRONT_EXT", "VK_FACE_FRONT_KHR"]
    );
    assert_eq!(names("VK_FACE_BACK"), ["VK_FACE_BACK_NV"]);
    assert_eq!(names("VK_FACE_FRONT_KHR"), Vec::<&str>::new());
}

#[test]
fn checker_accepts_and_collects_a_value_described_under_an_alias_name() {
    let checker = fixture_checker(FIXTURE_SOURCE);
    assert_eq!(
        checker.described_under_an_alias,
        ["VkColor::VK_COLOR_CINNABAR under VK_COLOR_VERMILION"]
    );
    assert_reported(
        "a value documented only by its type although the page describes its alias",
        "/// `VK_COLOR_CINNABAR`, which the specification describes under the name `VK_COLOR_VERMILION`:\n    /// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.",
        "/// `VK_COLOR_CINNABAR`, a value of [`VkColor`](PAGES/VkColor.html).",
        "Color::CINNABAR: the specification describes `VK_COLOR_CINNABAR` on the page `VkColor` under its alias name `VK_COLOR_VERMILION`, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_value_that_takes_a_description_under_another_name_wrongly() {
    for (defect, replace, with, report) in [
        (
            "the description of a name that is no alias",
            "/// `VK_COLOR_CINNABAR`, which the specification describes under the name `VK_COLOR_VERMILION`:\n    /// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.",
            "/// `VK_COLOR_CINNABAR`, which the specification describes under the name `VK_COLOR_GREEN`:\n    /// [`VK_COLOR_GREEN`](Color::GREEN) specifies green.",
            "Color::CINNABAR: the documentation takes the description of `VK_COLOR_GREEN`, which the registry does not have as an alias of `VK_COLOR_CINNABAR`",
        ),
        (
            "other words than the item of the alias",
            "/// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.",
            "/// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a blue.",
            "Color::CINNABAR: `[`VK_COLOR_VERMILION`](Color::VERMILION) specifies a blue.` is not in the text that the specification has for `VkColor`",
        ),
        (
            "the description of another item under the name of the alias",
            "/// [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.",
            "/// [`VK_COLOR_GREEN`](Color::GREEN) specifies green.",
            "Color::CINNABAR: the description that the documentation takes for `VK_COLOR_CINNABAR` is not the one of `VK_COLOR_VERMILION`",
        ),
        (
            "a description under the name of a value that the page describes itself",
            "/// `VK_COLOR_RED` specifies red.",
            "/// `VK_COLOR_RED`, which the specification describes under the name `VK_COLOR_VERMILION`: [`VK_COLOR_VERMILION`](Color::VERMILION) specifies a red.",
            "Color::RED: the specification describes `VK_COLOR_RED` on the page `VkColor` under its own name, but the documentation takes the description of its alias `VK_COLOR_VERMILION`",
        ),
    ] {
        assert_reported(defect, replace, with, report);
    }
}

#[test]
fn checker_reports_a_description_under_an_alias_name_taken_before_one_under_the_own_name() {
    let source = FIXTURE_SOURCE.replace("PAGES/", REFERENCE_PAGES);
    let failures = |specification: &Specification| {
        let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
        let mut checker = Checker::default();
        check_files(specification, &Generated::new(), &mut checker, &[file]);
        checker.failures
    };
    let mut with_a_row = fixture_specification();
    with_a_row.tables.extend(fixture_table(
        FIXTURE_FILE,
        Some("VkColor"),
        &[".Reds", "[options=\"header\"]"],
        &["|Color |Use", "|ename:VK_COLOR_CINNABAR |paint"],
    ));
    assert_eq!(
        failures(&with_a_row),
        [
            "Color::CINNABAR: the specification describes `VK_COLOR_CINNABAR` in a table row, which comes before the description under its alias name `VK_COLOR_VERMILION`"
        ]
    );
    let mut with_a_structure = fixture_specification();
    with_a_structure.values_of_structures_and_handles = values_of_structures_and_handles(
        r#"<type category="struct" name="VkThing">
                <member values="VK_COLOR_CINNABAR"><type>VkStructureType</type> <name>sType</name></member>
                <member values="VK_COLOR_RED"><type>VkColor</type> <name>color</name></member>
            </type>
            <type category="handle" parent="VkDevice" objtypeenum="VK_COLOR_GOLD"><type>VK_DEFINE_HANDLE</type>(<name>VkThings</name>)</type>
            <type category="bitmask" objtypeenum="VK_COLOR_SILVER"/>"#,
    );
    assert_eq!(
        with_a_structure
            .values_of_structures_and_handles
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["VK_COLOR_CINNABAR", "VK_COLOR_GOLD"]
    );
    assert_eq!(
        failures(&with_a_structure),
        [
            "Color::CINNABAR: the registry has a structure or a handle for `VK_COLOR_CINNABAR`, which comes before the description under its alias name `VK_COLOR_VERMILION`"
        ]
    );
    let mut with_an_item_elsewhere = fixture_specification();
    let item = fixture_segment(
        None,
        "  * ename:VK_COLOR_CINNABAR specifies a red of the text.",
    );
    let elsewhere = with_an_item_elsewhere.value_descriptions_in(&[item]);
    with_an_item_elsewhere.value_descriptions.extend(elsewhere);
    assert_eq!(
        failures(&with_an_item_elsewhere),
        [
            "Color::CINNABAR: the specification describes `VK_COLOR_CINNABAR` in a list item of chapters/things.adoc outside the reference pages, which comes before the description under its alias name `VK_COLOR_VERMILION`"
        ]
    );
}

#[test]
fn checker_collects_what_a_list_item_that_starts_with_no_name_describes() {
    let checker = fixture_checker(FIXTURE_SOURCE);
    assert_eq!(
        checker.described_without_their_name_first,
        [
            "VkThing::margin",
            "VkColor::VK_COLOR_IVORY",
            "VkColor::VK_COLOR_EBONY",
        ]
    );
    let specification = fixture_specification();
    let described = |page: &str| {
        specification.unnamed_descriptions[page]
            .iter()
            .map(|(name, text)| (name.as_str(), text.as_str()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        described("VkThing"),
        [(
            "margin",
            "  * [[limits-margin]] When the <<limits-range,pname:range>> is laid out,\n    pname:margin indicates the space around it."
        )],
        "an anchor of a limit whose item does not name the member, and a name under another anchor, describe no member"
    );
    assert_eq!(
        described("VkColor"),
        [
            (
                "VK_COLOR_IVORY",
                "  * VK_COLOR_IVORY specifies ivory,\n    without its macro."
            ),
            (
                "VK_COLOR_EBONY",
                "  * When things are dark, ename:VK_COLOR_EBONY specifies ebony:\n  ** darker than ename:VK_COLOR_IVORY"
            ),
        ],
        "a value that is not the first name of its item, or that the item does not specify, is not described"
    );
    assert_eq!(described("VkColorKHR"), described("VkColor"));
    assert_eq!(described("VkLimits"), []);
}

#[test]
fn checker_reports_a_fallback_when_a_list_item_that_starts_with_no_name_describes_the_item() {
    assert_reported(
        "a member documented only by its declaration",
        "/// When the range is laid out, `margin` indicates the space around it.",
        "/// The C member `uint32_t margin`.",
        "Thing::margin: the specification describes `margin` on the page `VkThing` in a list item that does not start with the name, but the documentation does not use it",
    );
    assert_reported(
        "a value documented only by its type although an item names it without its macro",
        "/// `VK_COLOR_IVORY` specifies ivory, without its macro.",
        "/// `VK_COLOR_IVORY`, a value of [`VkColor`](PAGES/VkColor.html).",
        "Color::IVORY: the specification describes `VK_COLOR_IVORY` on the page `VkColor` in a list item that does not start with the name, but the documentation does not use it",
    );
    assert_reported(
        "a value documented only by its type although an item names it first",
        "/// When things are dark, `VK_COLOR_EBONY` specifies ebony:\n    ///\n    /// - darker than [`VK_COLOR_IVORY`](Color::IVORY)\n",
        "/// `VK_COLOR_EBONY`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        "Color::EBONY: the specification describes `VK_COLOR_EBONY` on the page `VkColor` in a list item that does not start with the name, but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_description_that_is_not_the_list_item_that_starts_with_no_name() {
    for (defect, replace, with, report) in [
        (
            "a member with other words",
            "/// When the range is laid out, `margin` indicates the space around it.",
            "/// When the range is laid out, `margin` indicates the space in it.",
            "Thing::margin: `When the range is laid out, `margin` indicates the space in it.` is not the list item of the page `VkThing` that describes `margin`: `* [[limits-margin]] When the <<limits-range,pname:range>> is laid out, pname:margin indicates the space around it.`",
        ),
        (
            "a member with the start of the item left out",
            "/// When the range is laid out, `margin` indicates the space around it.",
            "/// `margin` indicates the space around it.",
            "Thing::margin: ``margin` indicates the space around it.` is not the list item of the page `VkThing` that describes `margin`",
        ),
        (
            "a value with a part of the item left out",
            "    ///\n    /// - darker than [`VK_COLOR_IVORY`](Color::IVORY)\n",
            "",
            "Color::EBONY: `When things are dark, `VK_COLOR_EBONY` specifies ebony:` is not the list item of the page `VkColor` that describes `VK_COLOR_EBONY`",
        ),
        (
            "a value with more than the item",
            "/// `VK_COLOR_IVORY` specifies ivory, without its macro.",
            "/// `VK_COLOR_IVORY` specifies ivory, without its macro. It is white.",
            "Color::IVORY: ``VK_COLOR_IVORY` specifies ivory, without its macro. It is white.` is not the list item of the page `VkColor` that describes `VK_COLOR_IVORY`",
        ),
    ] {
        assert_reported(defect, replace, with, report);
    }
}

#[test]
fn checker_reports_what_several_list_items_that_start_with_no_name_describe() {
    let mut specification = fixture_specification();
    for descriptions in specification.unnamed_descriptions.values_mut() {
        let again = descriptions.clone();
        descriptions.extend(again);
    }
    let source = FIXTURE_SOURCE.replace("PAGES/", REFERENCE_PAGES);
    let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
    let mut checker = Checker::default();
    check_files(&specification, &Generated::new(), &mut checker, &[file]);
    assert_eq!(
        checker.failures,
        [
            "Thing::margin: is described, but 2 list items of the page `VkThing` describe `margin` without starting with the name",
            "Color::IVORY: is described, but 2 list items of the page `VkColor` describe `VK_COLOR_IVORY` without starting with the name",
            "Color::EBONY: is described, but 2 list items of the page `VkColor` describe `VK_COLOR_EBONY` without starting with the name",
        ]
    );
    assert!(checker.described_without_their_name_first.is_empty());
}

#[test]
fn checker_reports_a_value_fallback_when_a_list_item_outside_the_page_describes_the_value() {
    assert_reported(
        "a value documented only by its type although the running text describes it",
        "/// `VK_COLOR_AMBER` specifies amber, outside the reference pages:\n    ///\n    /// - a color of things\n",
        "/// `VK_COLOR_AMBER`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        "Color::AMBER: the specification describes `VK_COLOR_AMBER` outside the page `VkColor` (chapters/things.adoc outside the reference pages), but the documentation does not use it",
    );
    assert_reported(
        "a value documented only by its type although another page describes it",
        "/// `VK_COLOR_UMBER` specifies umber, on the page of the things.",
        "/// `VK_COLOR_UMBER`, a value of [`VkColor`](PAGES/VkColor.html).",
        "Color::UMBER: the specification describes `VK_COLOR_UMBER` outside the page `VkColor` (the page VkThing), but the documentation does not use it",
    );
}

#[test]
fn checker_reports_a_list_item_outside_the_page_taken_before_a_table_row() {
    assert_reported(
        "a value with a table row documented by an item of the running text",
        "/// `VK_COLOR_BLUE` in the table “Color Wavelengths” of the specification:\n    ///\n    /// - Wavelength:\n    ///   - short\n    ///   - or very short\n",
        "/// `VK_COLOR_BLUE` specifies blue where things are drawn.\n",
        "Color::BLUE: the specification describes `VK_COLOR_BLUE` in a table row of the page `VkColor`, which comes before the list item of chapters/things.adoc outside the reference pages",
    );
}

#[test]
fn checker_reports_a_description_that_is_not_the_list_item_outside_the_page() {
    for (defect, replace, with) in [
        (
            "a part of the item left out",
            "    ///\n    /// - a color of things\n",
            "",
        ),
        (
            "other words",
            "/// `VK_COLOR_AMBER` specifies amber, outside",
            "/// `VK_COLOR_AMBER` specifies umber, outside",
        ),
        (
            "more than the item",
            "    /// - a color of things\n",
            "    /// - a color of things\n    /// - and more\n",
        ),
    ] {
        assert_reported(
            defect,
            replace,
            with,
            "is not the list item of chapters/things.adoc outside the reference pages that describes `VK_COLOR_AMBER`: `* ename:VK_COLOR_AMBER specifies amber, outside the reference pages: ** a color of things`",
        );
    }
    assert_reported(
        "a description that no list item has",
        "/// `VK_COLOR_OCHRE`, a value of [`VkColor`](PAGES/VkColor.html).",
        "/// `VK_COLOR_OCHRE` specifies ochre.",
        "Color::OCHRE: ``VK_COLOR_OCHRE` specifies ochre.` is not in the text that the specification has for `VkColor`",
    );
}

#[test]
fn checker_reports_a_value_that_several_list_items_outside_its_page_describe() {
    let mut specification = fixture_specification();
    let again = fixture_segment(
        None,
        "  * ename:VK_COLOR_AMBER specifies amber, outside the reference pages:\n  ** a color of things",
    );
    let more = specification.value_descriptions_in(&[again]);
    for (value, descriptions) in more {
        specification
            .value_descriptions
            .entry(value)
            .or_default()
            .extend(descriptions);
    }
    let failures = |source: &str| {
        let source = source.replace("PAGES/", REFERENCE_PAGES);
        let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
        let mut checker = Checker::default();
        check_files(&specification, &Generated::new(), &mut checker, &[file]);
        assert!(
            checker
                .described_outside_their_page
                .iter()
                .all(|value| !value.contains("VK_COLOR_AMBER"))
        );
        checker.failures
    };
    assert_eq!(
        failures(FIXTURE_SOURCE),
        [
            "Color::AMBER: is described, but 2 list items outside the page of its type describe `VK_COLOR_AMBER`"
        ]
    );
    let undescribed = FIXTURE_SOURCE.replacen(
        "/// `VK_COLOR_AMBER` specifies amber, outside the reference pages:\n    ///\n    /// - a color of things\n",
        "/// `VK_COLOR_AMBER`, a value of [`VkColor`](PAGES/VkColor.html).\n",
        1,
    );
    assert_eq!(
        failures(&undescribed),
        [
            "Color::AMBER: the specification describes `VK_COLOR_AMBER` outside the page `VkColor` (chapters/things.adoc outside the reference pages, chapters/things.adoc outside the reference pages), but the documentation does not use it"
        ]
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
    link_of_an_untitled_anchor: Regex,
    link_target: Regex,
    patterns: Vec<(&'static str, Regex)>,
}

impl Leftovers {
    fn new() -> Self {
        let macros = "pname|ptext|sname|stext|fname|ftext|etext|dname|tname|code|slink|elink|tlink|basetype|ename|flink|dlink|apiext|reflink|tag|attr|latexmath|can|cannot|may|must|optional|optionally|required|should|undefined";
        Self {
            code_span: regex(r"``.*?``|`[^`\n]*`"),
            link_of_an_untitled_anchor: regex(r"\[\\\[([\w.-]+)\\\]\]\([^)\s]*\)"),
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
        let text = self.link_of_an_untitled_anchor.replace_all(&text, "$1");
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
        ("/// see \\[\\[anchor\\]\\]", "an anchor"),
        ("/// see [\\[anchor\\]] and (more)", "an anchor"),
        ("/// see [\\[an anchor\\]](https://a.b/c#d)", "an anchor"),
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
        "/// See [\\[samplers-mipLodBias\\]](https://a.b/samplers.html#samplers-mipLodBias).",
        "/// a set {0, 1} and `{x}`",
        "/// i \\< 2 \\<\\< 3 and a - b and --- and a-- b",
        "/// # Safety\n///\n/// - must be valid",
        "/// note: \\[x\\] and the C:\\\\path",
    ] {
        let found = asciidoc_leftovers(&format!("{doc}\npub struct Thing;\n"));
        assert!(found.is_empty(), "`{doc}` is reported: {found:?}");
    }
}

const FIXTURE_LEGACY_SOURCE: &str = r#"
/// The name of the device extension `VK_KHR_things`.
#[doc(alias = "VK_KHR_things")]
#[doc(alias = "VK_KHR_THINGS_EXTENSION_NAME")]
pub const KHR_THINGS_NAME: &CStr = c"VK_KHR_things";

/// Structure specifying a thing.
///
/// C structure [`VkThing`](PAGES/VkThing.html).
///
/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
#[doc(alias = "VkThing")]
pub struct Thing {
    /// `count` is the number of things.
    pub count: u32,
    /// `flags` is reserved for future use.
    ///
    /// Legacy functionality as of Vulkan 1.1: see [Legacy
    /// Functionality](SPEC/chapters/things.html#things-flags) in the Vulkan specification.
    pub flags: u32,
}

impl Thing {
    /// Sets [`count`](Self::count).
    pub fn count(mut self, value: u32) -> Self {
        self.count = value;
        self
    }
    /// Sets [`flags`](Self::flags).
    ///
    /// Legacy functionality as of Vulkan 1.1: see [Legacy
    /// Functionality](SPEC/chapters/things.html#things-flags) in the Vulkan specification.
    pub fn flags(mut self, value: u32) -> Self {
        self.flags = value;
        self
    }
}

/// C structure `VkThing2`.
#[doc(alias = "VkThing2")]
pub struct Thing2 {}

/// C structure `VkThing2KHR`.
///
/// Legacy functionality as of Vulkan 1.0, Vulkan 1.1 and
/// [`VK_KHR_things`](KHR_THINGS_NAME): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
#[doc(alias = "VkThing2KHR")]
pub struct Thing2KHR {}

/// Alias of [`Thing`], the C name `VkThingKHR`.
#[doc(alias = "VkThingKHR")]
pub type ThingKHR = Thing;

impl crate::Device {
    /// Calls the C command `vkUseThing`, an alias of [`vkUseThing2`](crate::Device::use_thing2),
    /// with the arguments as they are.
    ///
    /// Legacy functionality, superseded by [`vkUseThing2`](crate::Device::use_thing2): see [Legacy
    /// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
    ///
    /// # Safety
    ///
    /// The call has to satisfy the valid usage.
    pub unsafe fn use_thing_raw(&self) {}

    /// Calls the C command `vkUseThing2` with the arguments as they are.
    pub unsafe fn use_thing2_raw(&self) {}

    /// Calls the C command `vkMakeThing` with the arguments as they are.
    ///
    /// Legacy functionality as of Vulkan 1.1: see [Legacy
    /// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
    pub unsafe fn make_thing_raw(&self) {}
}

impl crate::Device {
    /// Calls the C command `vkUseThing` through [`Self::use_thing_raw`], which describes the
    /// parameters.
    ///
    /// Legacy functionality, superseded by [`vkUseThing2`](crate::Device::use_thing2): see [Legacy
    /// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
    #[doc(alias = "vkUseThing")]
    pub unsafe fn use_thing(&self) {}

    /// Calls the C command `vkUseThing2` through [`Self::use_thing2_raw`], which describes the
    /// parameters.
    #[doc(alias = "vkUseThing2")]
    pub unsafe fn use_thing2(&self) {}
}
"#;

fn fixture_legacy_mark(link: &str, marked_by: &[&str], superseded_by: Option<&str>) -> LegacyMark {
    LegacyMark {
        link: link.to_string(),
        marked_by: marked_by.iter().map(|by| by.to_string()).collect(),
        superseded_by: superseded_by.map(str::to_string),
    }
}

fn fixture_legacy_specification() -> Specification {
    let mut specification = fixture_specification();
    for (anchor, files) in [
        ("legacy-things", &["appendices/legacy.adoc"][..]),
        ("things-flags", &["chapters/things.adoc"]),
        ("things-nested", &["chapters/things/more.adoc"]),
        ("things-deep", &["chapters/things/deep.adoc"]),
        ("things-partial", &["appendices/VK_KHR_things.adoc"]),
        (
            "things-twice",
            &["chapters/things.adoc", "chapters/more.adoc"],
        ),
    ] {
        let files = files.iter().map(|file| file.to_string()).collect();
        specification.anchors.insert(anchor.to_string(), files);
    }
    specification.published = [
        "appendices/legacy.adoc",
        "chapters/things.adoc",
        "chapters/more.adoc",
        "chapters/things/deep.adoc",
    ]
    .map(str::to_string)
    .into();
    specification.legacy.vendor_tags = ["KHR", "EXT", "NV"].map(str::to_string).to_vec();
    specification.legacy.type_aliases = ["VkThingKHR".to_string()].into();
    let version = ["VK_VERSION_1_1"];
    let extension = ["VK_KHR_things"];
    let both = ["VK_VERSION_1_1", "VK_KHR_things"];
    let marks = [
        (
            ("type", "", "VkThing"),
            fixture_legacy_mark("legacy-things", &version, Some("VkThing2")),
        ),
        (
            ("type", "", "VkThing2"),
            fixture_legacy_mark("legacy-things", &extension, Some("VkThing")),
        ),
        (
            ("type", "", "VkThingKHR"),
            fixture_legacy_mark("legacy-things", &both, Some("VkThing2")),
        ),
        (
            ("type", "", "VkThing2KHR"),
            fixture_legacy_mark(
                "legacy-things",
                &["VK_VERSION_1_0", "VK_VERSION_1_1", "VK_KHR_things"],
                None,
            ),
        ),
        (
            ("type", "", "VK_MAKE_VERSION"),
            fixture_legacy_mark("legacy-things", &version, None),
        ),
        (
            ("command", "", "vkUseThing"),
            fixture_legacy_mark("legacy-things", &both, Some("vkUseThing2")),
        ),
        (
            ("command", "", "vkMakeThing"),
            fixture_legacy_mark("legacy-things", &version, None),
        ),
        (
            ("feature", "VkThing", "flags"),
            fixture_legacy_mark("things-flags", &both, None),
        ),
    ];
    for ((kind, structure, name), mark) in marks {
        let key = (kind.to_string(), structure.to_string(), name.to_string());
        specification.legacy.marks.insert(key, mark);
    }
    specification
}

fn fixture_legacy_checker(specification: &Specification, source: &str) -> Checker {
    let source = source
        .replace("PAGES/", REFERENCE_PAGES)
        .replace("SPEC/", SPECIFICATION);
    let file = syn::parse_file(&source).unwrap_or_else(|e| panic!("{e}:\n{source}"));
    let mut checker = Checker::default();
    check_files(specification, &Generated::new(), &mut checker, &[file]);
    checker
}

fn assert_legacy_reported(defect: &str, replace: &str, with: &str, reports: &[&str]) {
    assert_eq!(
        FIXTURE_LEGACY_SOURCE.matches(replace).count(),
        1,
        "{defect}: `{replace}` is not exactly once in the fixture"
    );
    let source = FIXTURE_LEGACY_SOURCE.replacen(replace, with, 1);
    let failures = fixture_legacy_checker(&fixture_legacy_specification(), &source).failures;
    assert_eq!(failures, reports, "{defect}");
}

#[test]
fn checker_accepts_the_legacy_statements_of_the_names_that_the_registry_marks() {
    let checker = fixture_legacy_checker(&fixture_legacy_specification(), FIXTURE_LEGACY_SOURCE);
    assert!(checker.failures.is_empty(), "{:#?}", checker.failures);
    assert_eq!(
        checker.legacy_statements,
        BTreeMap::from([
            ("type", 2),
            ("raw", 2),
            ("wrapper", 1),
            ("member", 1),
            ("method", 1)
        ])
    );
    assert_eq!(checker.specification_links, 7);
    assert_eq!(checker.described_members, 2);
    assert_eq!(
        checker.legacy_shown,
        BTreeMap::from([
            ("command".to_string(), 2),
            ("feature".to_string(), 1),
            ("type".to_string(), 3)
        ]),
        "with the version macro that has no item"
    );
    assert_eq!(
        checker.legacy_not_shown,
        BTreeMap::from([
            (
                "type alias by VK_VERSION_1_1 and VK_KHR_things".to_string(),
                1
            ),
            ("type by VK_KHR_things".to_string(), 1)
        ]),
        "a type alias, and a core structure that only a KHR extension marks"
    );
}

#[test]
fn an_extension_marks_only_what_is_not_more_promoted_than_the_extension_is() {
    let legacy = LegacyRegistry {
        vendor_tags: ["NVX", "NV", "KHR", "EXT"].map(str::to_string).to_vec(),
        ..Default::default()
    };
    let extensions = [
        "VK_KHR_things",
        "VK_EXT_things",
        "VK_NV_things",
        "VK_QCOM_things",
    ];
    for (structure, name, counted) in [
        ("", "VkThing", 0),
        ("", "VkKHRThing", 0),
        ("", "VkThingKhr", 0),
        ("", "VkThingKHR", 1),
        ("", "vkUseThingEXT", 2),
        ("", "VkThingNV", 4),
        ("", "VkThingNVX", 4),
        ("VkThing", "enabledNV", 0),
        ("VkThingEXT", "enabled", 2),
    ] {
        let mark = fixture_legacy_mark("legacy-things", &extensions, None);
        let kind = match structure {
            "" => "type",
            _ => "feature",
        };
        assert_eq!(
            legacy.shown(kind, structure, name, &mark),
            extensions[..counted],
            "{structure} {name}"
        );
        let reversed = [
            "VK_QCOM_things",
            "VK_VERSION_1_1",
            "VK_EXT_things",
            "VK_KHR_things",
        ];
        let mark = fixture_legacy_mark("legacy-things", &reversed, Some("VkThing2"));
        let shown = match counted {
            0 => &reversed[1..2],
            1 => &["VK_VERSION_1_1", "VK_KHR_things"][..],
            2 => &reversed[1..],
            _ => &reversed[..],
        };
        assert_eq!(
            legacy.shown(kind, structure, name, &mark),
            shown,
            "{structure} {name}"
        );
    }
    let aliases = LegacyRegistry {
        type_aliases: ["VkThingNV".to_string(), "vkUseThingEXT".to_string()].into(),
        vendor_tags: legacy.vendor_tags.clone(),
        ..Default::default()
    };
    let mark = fixture_legacy_mark("legacy-things", &["VK_VERSION_1_1"], Some("VkThing2"));
    for (kind, structure, name, shown) in [
        ("type", "", "VkThingNV", false),
        ("type", "", "VkThing", true),
        ("command", "", "VkThingNV", true),
        ("command", "", "vkUseThingEXT", true),
        ("feature", "VkThingNV", "enabled", true),
        ("feature", "VkThing", "VkThingNV", true),
    ] {
        assert_eq!(
            aliases.shown(kind, structure, name, &mark),
            ["VK_VERSION_1_1"][..usize::from(shown)],
            "{kind} {structure} {name}"
        );
        assert_eq!(
            legacy.shown(kind, structure, name, &mark),
            ["VK_VERSION_1_1"]
        );
    }
    assert_eq!(legacy.vendor_tag("VkThingNVX"), Some("NVX"));
    assert_eq!(legacy.vendor_tag("VkThingNV"), Some("NV"));
    assert_eq!(legacy.vendor_tag("VkThing"), None);
}

#[test]
fn a_source_file_is_a_page_if_its_first_title_is_a_top_level_one_in_its_first_thirty_lines() {
    let blank = |lines: usize| "\n".repeat(lines);
    for (file_name, text, published) in [
        ("legacy.adoc", "// Copyright\n\n[appendix]\n[[legacy]]\n= Legacy Functionality\n".to_string(), true),
        ("generatedcommands.adoc", "[[device-generated-commands]]\n# Device-Generated Commands\n".to_string(), true),
        ("VK_KHR_surface.adoc", "include::{generated}/meta/VK_KHR_surface.adoc[]\n\n=== Other Extension Metadata\n\n= Title\n".to_string(), false),
        ("partial.adoc", "== Section\n= Title\n".to_string(), false),
        ("partial.adoc", "=# Section\n= Title\n".to_string(), false),
        ("untitled.adoc", "Text = Title\n=Title\n=\n".to_string(), false),
        ("lowercase.adoc", "= lowercase\n".to_string(), false),
        ("lowercase.adoc", "= lowercase\n= Title\n".to_string(), true),
        ("lowercase.adoc", "= lowercase\n== Section\n= Title\n".to_string(), false),
        ("in_time.adoc", format!("{}= Title\n", blank(29)), true),
        ("late.adoc", format!("{}= Title\n", blank(30)), false),
        ("_private.adoc", "= Title\n".to_string(), false),
        ("1.adoc", "= Title\n".to_string(), false),
        ("vulkanscdeviations.adoc", "= Title\n".to_string(), false),
        ("title.txt", "= Title\n".to_string(), false),
    ] {
        assert_eq!(is_published_page(file_name, &text), published, "{file_name}: {text}");
    }
    let specification = fixture_legacy_specification();
    assert_eq!(
        specification.section_url("things-deep").as_deref(),
        Ok("https://docs.vulkan.org/spec/latest/chapters/things/deep.html#things-deep")
    );
}

#[test]
fn checker_reports_a_marked_enumeration_with_a_vendor_tag() {
    let mut specification = fixture_legacy_specification();
    for name in ["VkThing", "VkThing2KHR", "VkOtherKHR"] {
        specification.legacy.enumerations.insert(name.to_string());
    }
    assert_eq!(
        fixture_legacy_checker(&specification, FIXTURE_LEGACY_SOURCE).failures,
        [
            "VkThing2KHR: is a marked enumeration with a vendor tag: the generator of the specification takes every enumeration for core, and the bindings go by the name"
        ]
    );
}

const FIXTURE_LEGACY_THING: &str =
    "/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
";

const FIXTURE_LEGACY_THING_STATEMENT: &str = "Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.";

#[test]
fn checker_reports_a_marked_item_without_its_legacy_statement() {
    assert_legacy_reported(
        "a structure without the statement",
        &format!("///\n{FIXTURE_LEGACY_THING}"),
        "",
        &[&format!(
            "the type `VkThing`: the registry marks it as legacy, so its documentation has the one paragraph `{FIXTURE_LEGACY_THING_STATEMENT}`, but it has []"
        )],
    );
    let flags = "Legacy functionality as of Vulkan 1.1: see [Legacy Functionality](https://docs.vulkan.org/spec/latest/chapters/things.html#things-flags) in the Vulkan specification.";
    assert_legacy_reported(
        "a setter without the statement",
        "    /// Sets [`flags`](Self::flags).
    ///
    /// Legacy functionality as of Vulkan 1.1: see [Legacy
    /// Functionality](SPEC/chapters/things.html#things-flags) in the Vulkan specification.
",
        "    /// Sets [`flags`](Self::flags).\n",
        &[&format!(
            "the method `Thing::flags`: the registry marks it as legacy, so its documentation has the one paragraph `{flags}`, but it has []"
        )],
    );
    assert_legacy_reported(
        "a member without the statement",
        "    /// `flags` is reserved for future use.
    ///
    /// Legacy functionality as of Vulkan 1.1: see [Legacy
    /// Functionality](SPEC/chapters/things.html#things-flags) in the Vulkan specification.
",
        "    /// `flags` is reserved for future use.\n",
        &[&format!(
            "the member `Thing::flags`: the registry marks it as legacy, so its documentation has the one paragraph `{flags}`, but it has []"
        )],
    );
    let use_thing = "Legacy functionality, superseded by [`vkUseThing2`](crate::Device::use_thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.";
    assert_legacy_reported(
        "a wrapper without the statement",
        "    /// parameters.
    ///
    /// Legacy functionality, superseded by [`vkUseThing2`](crate::Device::use_thing2): see [Legacy
    /// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
    #[doc(alias = \"vkUseThing\")]
",
        "    /// parameters.\n    #[doc(alias = \"vkUseThing\")]\n",
        &[&format!(
            "the wrapper `vkUseThing`: the registry marks it as legacy, so its documentation has the one paragraph `{use_thing}`, but it has []"
        )],
    );
    assert_legacy_reported(
        "a marked command without a raw method",
        "    /// Calls the C command `vkMakeThing` with the arguments as they are.",
        "    /// Calls the C command `vkMakeThings` with the arguments as they are.",
        &[
            "the raw `vkMakeThings`: has a legacy statement, but the registry has no mark for it that the specification shows",
            "the raw `vkMakeThing`: is marked as legacy, and the bindings document no such item",
        ],
    );
    assert_legacy_reported(
        "a marked feature without a member",
        "    pub flags: u32,",
        "    pub flag: u32,",
        &[
            "Thing::flag: is documented with the description of `flags`",
            "VkThing::flags: is marked as legacy, and the bindings have no such member",
            "the member `Thing::flag`: has a legacy statement, but the registry has no mark for it that the specification shows",
            "the method `Thing::flags`: has a legacy statement, but the registry has no mark for it that the specification shows",
        ],
    );
}

#[test]
fn checker_reports_a_legacy_statement_that_differs_from_what_the_registry_states() {
    let expected = |statements: &str| {
        format!(
            "the type `VkThing`: the registry marks it as legacy, so its documentation has the one paragraph `{FIXTURE_LEGACY_THING_STATEMENT}`, but it has {statements}"
        )
    };
    for (defect, with, statements) in [
        (
            "another successor",
            "/// Legacy functionality, superseded by [`VkThing2KHR`](Thing2KHR): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
",
            "[\"Legacy functionality, superseded by [`VkThing2KHR`](Thing2KHR): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.\"]",
        ),
        (
            "a link to another item",
            "/// Legacy functionality, superseded by [`VkThing2`](Thing): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
",
            "[\"Legacy functionality, superseded by [`VkThing2`](Thing): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.\"]",
        ),
        (
            "the version that marks it in place of the successor",
            "/// Legacy functionality as of Vulkan 1.1: see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
",
            "[\"Legacy functionality as of Vulkan 1.1: see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.\"]",
        ),
        (
            "another section",
            "/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy
/// Functionality](SPEC/chapters/things.html#things-flags) in the Vulkan specification.
",
            "[\"Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/chapters/things.html#things-flags) in the Vulkan specification.\"]",
        ),
        (
            "another page for the anchor",
            "/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy
/// Functionality](SPEC/chapters/things.html#legacy-things) in the Vulkan specification.
",
            "[\"Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/chapters/things.html#legacy-things) in the Vulkan specification.\"]",
        ),
        (
            "the statement twice",
            &format!("{FIXTURE_LEGACY_THING}///\n{FIXTURE_LEGACY_THING}"),
            &format!("[{FIXTURE_LEGACY_THING_STATEMENT:?}, {FIXTURE_LEGACY_THING_STATEMENT:?}]"),
        ),
        (
            "a statement that claims more",
            "/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification. It
/// must not be used.
",
            "[\"Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification. It must not be used.\"]",
        ),
    ] {
        assert_legacy_reported(defect, FIXTURE_LEGACY_THING, with, &[&expected(statements)]);
    }
    assert_legacy_reported(
        "a statement in the paragraph before it",
        &format!("///\n{FIXTURE_LEGACY_THING}"),
        FIXTURE_LEGACY_THING,
        &[&expected(
            "[\"C structure [`VkThing`](https://docs.vulkan.org/refpages/latest/refpages/source/VkThing.html). Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.\"]",
        )],
    );
    assert_legacy_reported(
        "the versions and extensions in another order",
        "Vulkan 1.0, Vulkan 1.1 and\n/// [`VK_KHR_things`](KHR_THINGS_NAME)",
        "Vulkan 1.1, Vulkan 1.0 and\n/// [`VK_KHR_things`](KHR_THINGS_NAME)",
        &[
            "the type `VkThing2KHR`: the registry marks it as legacy, so its documentation has the one paragraph `Legacy functionality as of Vulkan 1.0, Vulkan 1.1 and [`VK_KHR_things`](KHR_THINGS_NAME): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.`, but it has [\"Legacy functionality as of Vulkan 1.1, Vulkan 1.0 and [`VK_KHR_things`](KHR_THINGS_NAME): see [Legacy Functionality](https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-things) in the Vulkan specification.\"]",
        ],
    );
}

#[test]
fn checker_reports_a_legacy_statement_on_an_item_that_the_registry_does_not_mark() {
    for (defect, replace, item) in [
        (
            "a structure that only a less promoted extension marks",
            "/// C structure `VkThing2`.\n",
            "the type `VkThing2`",
        ),
        (
            "an unmarked constant",
            "/// The name of the device extension `VK_KHR_things`.\n",
            "the type `VK_KHR_things`",
        ),
        (
            "an unmarked member",
            "    /// `count` is the number of things.\n",
            "the member `Thing::count`",
        ),
        (
            "an unmarked setter",
            "    /// Sets [`count`](Self::count).\n",
            "the method `Thing::count`",
        ),
        (
            "an unmarked raw method",
            "    /// Calls the C command `vkUseThing2` with the arguments as they are.\n",
            "the raw `vkUseThing2`",
        ),
    ] {
        let indent = &replace[..replace.find("///").expect("a doc comment")];
        let with = format!(
            "{replace}{indent}///\n{}",
            FIXTURE_LEGACY_THING.replace("/// ", &format!("{indent}/// "))
        );
        assert_legacy_reported(
            defect,
            replace,
            &with,
            &[&format!(
                "{item}: has a legacy statement, but the registry has no mark for it that the specification shows"
            )],
        );
    }
    assert_legacy_reported(
        "a marked type alias, which the specification lists without a warning",
        "/// Alias of [`Thing`], the C name `VkThingKHR`.\n",
        "/// Alias of [`Thing`], the C name `VkThingKHR`.\n///\n/// Legacy functionality, superseded by [`VkThing2`](Thing2): see [Legacy\n/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.\n",
        &[
            "the type `VkThingKHR`: has a legacy statement, but the registry has no mark for it that the specification shows",
        ],
    );
    assert_legacy_reported(
        "an unmarked wrapper",
        "    /// parameters.\n    #[doc(alias = \"vkUseThing2\")]\n",
        "    /// parameters.\n    ///\n    /// Legacy functionality.\n    ///\n    /// See [Legacy Functionality](SPEC/appendices/legacy.html#legacy-things).\n    #[doc(alias = \"vkUseThing2\")]\n",
        &[
            "the wrapper `vkUseThing2`: has a legacy statement, but the registry has no mark for it that the specification shows",
        ],
    );
}

#[test]
fn checker_links_a_successor_only_from_an_item_that_exists_whenever_the_successor_does() {
    let plain = FIXTURE_LEGACY_THING.replace("[`VkThing2`](Thing2)", "`VkThing2`");
    let gated = "#[cfg(feature = \"wayland\")]\n#[doc(alias = \"VkThing2\")]";
    let both_gated =
        "#[cfg(all(feature = \"beta\", feature = \"wayland\"))]\n#[doc(alias = \"VkThing\")]";
    for (thing, thing2, statement, accepted) in [
        (
            "#[doc(alias = \"VkThing\")]",
            gated,
            FIXTURE_LEGACY_THING,
            false,
        ),
        ("#[doc(alias = \"VkThing\")]", gated, plain.as_str(), true),
        (both_gated, gated, FIXTURE_LEGACY_THING, true),
        (both_gated, gated, plain.as_str(), false),
        (
            "#[cfg(feature = \"beta\")]\n#[doc(alias = \"VkThing\")]",
            gated,
            FIXTURE_LEGACY_THING,
            false,
        ),
        (
            both_gated,
            "#[doc(alias = \"VkThing2\")]",
            plain.as_str(),
            false,
        ),
    ] {
        let source = FIXTURE_LEGACY_SOURCE
            .replacen("#[doc(alias = \"VkThing\")]", thing, 1)
            .replacen("#[doc(alias = \"VkThing2\")]", thing2, 1)
            .replacen(FIXTURE_LEGACY_THING, statement, 1)
            .replacen(
                "/// Legacy functionality as of Vulkan 1.0, Vulkan 1.1 and
/// [`VK_KHR_things`](KHR_THINGS_NAME): see [Legacy
/// Functionality](SPEC/appendices/legacy.html#legacy-things) in the Vulkan specification.
#[doc(alias = \"VkThing2KHR\")]",
                "#[doc(alias = \"VkThing2KHR\")]",
                1,
            );
        let mut specification = fixture_legacy_specification();
        specification
            .legacy
            .marks
            .retain(|(_, _, name), _| name != "VkThing2KHR");
        let failures = fixture_legacy_checker(&specification, &source).failures;
        assert_eq!(
            failures.iter().any(|failure| failure
                .starts_with("the type `VkThing`: the registry marks it as legacy")),
            !accepted,
            "{thing} {thing2} {statement}: {failures:#?}"
        );
        assert!(failures.len() <= 1, "{failures:#?}");
    }
}

#[test]
fn checker_reports_a_legacy_mark_that_the_specification_or_the_bindings_cannot_support() {
    let failures_of = |name: &str, link: &str, superseded_by: Option<&str>, marked_by: &[&str]| {
        let mut specification = fixture_legacy_specification();
        let key = ("type".to_string(), String::new(), name.to_string());
        specification
            .legacy
            .marks
            .insert(key, fixture_legacy_mark(link, marked_by, superseded_by));
        fixture_legacy_checker(&specification, FIXTURE_LEGACY_SOURCE).failures
    };
    let failures_with = |link: &str, superseded_by: Option<&str>, marked_by: &[&str]| {
        failures_of("VkThing", link, superseded_by, marked_by)
    };
    let version = ["VK_VERSION_1_1"];
    for (link, problem) in [
        (
            "things-missing",
            "0 source files of the specification define the anchor `things-missing`",
        ),
        (
            "things-twice",
            "2 source files of the specification define the anchor `things-twice`",
        ),
        (
            "things-nested",
            "the anchor `things-nested` is in `chapters/things/more.adoc`, which is not published as a page of its own",
        ),
        (
            "things-partial",
            "the anchor `things-partial` is in `appendices/VK_KHR_things.adoc`, which is not published as a page of its own",
        ),
    ] {
        assert_eq!(
            failures_with(link, Some("VkThing2"), &version),
            [
                format!("VkThing: is marked as legacy, and {problem}"),
                format!("the type `VkThing`: is marked as legacy, and {problem}"),
            ]
        );
    }
    assert_eq!(
        failures_with("legacy-things", Some("VkThing3"), &version),
        ["the type `VkThing`: is marked as legacy, and the bindings have no item for `VkThing3`"]
    );
    assert_eq!(
        failures_of(
            "VkThing2KHR",
            "legacy-things",
            None,
            &["VK_VERSION_1_1", "VK_KHR_more"]
        ),
        [
            "the type `VkThing2KHR`: is marked as legacy, and the bindings have no item for `VK_KHR_more`"
        ]
    );
    assert_eq!(
        failures_with(
            "legacy-things",
            Some("VkThing2"),
            &["VK_VERSION_1_1", "VK_KHR_more"]
        ),
        Vec::<String>::new(),
        "an extension that does not count is not named, so it needs no item"
    );
    assert_eq!(
        failures_with("legacy-things", Some("VkThing2"), &["VK_KHR_things"]),
        [
            "the type `VkThing`: has a legacy statement, but the registry has no mark for it that the specification shows"
        ],
        "a mark that the specification does not show"
    );

    let mut specification = fixture_legacy_specification();
    let source = FIXTURE_LEGACY_SOURCE.replacen(
        "/// C structure `VkThing2`.\n#[doc(alias = \"VkThing2\")]",
        "/// C structure `VkThing2`.\n#[doc(alias = \"VK_MAKE_VERSION\")]",
        1,
    );
    assert_eq!(
        fixture_legacy_checker(&specification, &source).failures[0],
        "VK_MAKE_VERSION: is in LEGACY_MACROS_WITHOUT_AN_ITEM, but the bindings have an item for it"
    );
    let key = ("type".to_string(), String::new(), "VkGone".to_string());
    let gone = fixture_legacy_mark("legacy-things", &version, None);
    specification.legacy.marks.insert(key, gone);
    assert_eq!(
        fixture_legacy_checker(&specification, FIXTURE_LEGACY_SOURCE).failures,
        ["the type `VkGone`: is marked as legacy, and the bindings document no such item"]
    );
}

#[test]
fn checker_reports_a_link_into_the_specification_that_is_not_a_legacy_statement() {
    assert_reported(
        "a link into the specification",
        "/// Structure specifying a thing.\n",
        "/// Structure specifying a thing.\n///\n/// See [the chapter](https://docs.vulkan.org/spec/latest/chapters/things.html).\n",
        "Thing: has a docs.vulkan.org link of an unexpected form",
    );
    assert_reported(
        "the link of the legacy statements on an unmarked item",
        "/// Structure specifying a thing.\n",
        "/// Structure specifying a thing.\n///\n/// See [Legacy Functionality](https://docs.vulkan.org/spec/latest/chapters/things.html#things).\n",
        "the type `VkThing`: has a legacy statement, but the registry has no mark for it that the specification shows",
    );
}

const FIXTURE_REGISTRY: &str = r#"
<registry>
    <tags comment="Vulkan vendor/author tags for extensions and layers">
        <tag name="NV" author="NVIDIA Corporation" contact="Somebody @somebody"/>
        <tag name="KHR" author="Khronos" contact="Somebody @somebody"/>
    </tags>
    <enums name="API Constants" type="constants" comment="Vulkan hardcoded constants">
        <enum type="uint32_t" value="256" name="VK_MAX_THINGS"/>
    </enums>
    <enums name="VkOld" type="enum">
        <enum value="0" name="VK_OLD_ZERO"/>
    </enums>
    <enums name="VkOldFlagBits" type="bitmask">
    </enums>
    <types comment="Vulkan type definitions">
        <type category="struct" name="VkOld">
            <member><type>uint32_t</type> <name>old</name></member>
        </type>
        <type category="struct" name="VkOldKHR" alias="VkOld"/>
        <type api="vulkansc" category="struct" name="VkOldSC" alias="VkOld"/>
        <type api="vulkan,vulkansc" category="handle" name="VkOldEXT" alias="VkOld"/>
    </types>
    <feature api="vulkan,vulkansc" apitype="internal" name="VK_BASE_VERSION_1_1" number="1.1">
        <require>
            <type name="VkRequired"/>
            <feature name="required" struct="VkFeatures"/>
        </require>
        <deprecate explanationlink="legacy-one">
            <type name="VkOld" supersededby="VkNew"/>
            <command name="vkOld"/>
        </deprecate>
    </feature>
    <feature api="vulkan,vulkansc" apitype="internal" name="VK_GRAPHICS_VERSION_1_1" number="1.1" depends="VK_BASE_VERSION_1_1">
        <deprecate explanationlink="legacy-one">
            <command name="vkOld"/>
        </deprecate>
        <deprecate explanationlink="legacy-two">
            <type name="vkOld"/>
        </deprecate>
    </feature>
    <feature api="vulkan,vulkansc" name="VK_VERSION_1_1" number="1.1" depends="VK_GRAPHICS_VERSION_1_1">
        <require>
            <feature name="old" struct="VkFeatures"/>
        </require>
    </feature>
    <feature api="vulkansc" name="VKSC_VERSION_1_0" number="1.0">
        <deprecate explanationlink="legacy-three">
            <type name="VkOld"/>
        </deprecate>
    </feature>
    <extensions comment="Vulkan extension interface definitions">
        <extension name="VK_KHR_new" number="1" supported="vulkan,vulkansc">
            <deprecate explanationlink="legacy-one">
                <feature name="old" struct="VkFeatures"/>
                <type name="VkOld" supersededby="VkNew"/>
                <command name="vkOld"/>
            </deprecate>
        </extension>
        <extension name="VK_KHR_disabled" number="2" supported="disabled">
            <deprecate explanationlink="legacy-three">
                <type name="VkOld"/>
                <command name="vkOther"/>
            </deprecate>
        </extension>
        <extension name="VK_KHR_safety" number="3" supported="vulkansc">
            <deprecate explanationlink="legacy-three">
                <command name="vkOld" supersededby="vkNew"/>
            </deprecate>
        </extension>
        <extension name="VK_KHR_apis" number="4" supported="vulkan">
            <deprecate explanationlink="legacy-three" api="vulkansc">
                <command name="vkOtherApi"/>
            </deprecate>
            <deprecate explanationlink="legacy-two" api="vulkan,vulkansc">
                <type name="vkOld" api="vulkansc" supersededby="vkNew"/>
                <type name="vkOld" api="vulkan"/>
            </deprecate>
        </extension>
        <extension name="VK_KHR_other_api" number="5" supported="vulkan" api="vulkansc">
            <deprecate explanationlink="legacy-three">
                <type name="VkOld"/>
            </deprecate>
        </extension>
    </extensions>
</registry>
"#;

#[test]
fn the_deprecate_blocks_of_the_registry_are_read_for_the_vulkan_api() {
    let legacy = LegacyRegistry::read(FIXTURE_REGISTRY);
    let mark = |link: &str, marked_by: &[&str], superseded_by: Option<&str>| {
        fixture_legacy_mark(link, marked_by, superseded_by)
    };
    let key = |kind: &str, structure: &str, name: &str| {
        (kind.to_string(), structure.to_string(), name.to_string())
    };
    assert_eq!(
        legacy.marks,
        BTreeMap::from([
            (
                key("command", "", "vkOld"),
                mark("legacy-one", &["VK_VERSION_1_1", "VK_KHR_new"], None)
            ),
            (
                key("feature", "VkFeatures", "old"),
                mark("legacy-one", &["VK_KHR_new"], None)
            ),
            (
                key("type", "", "VkOld"),
                mark(
                    "legacy-one",
                    &["VK_VERSION_1_1", "VK_KHR_new"],
                    Some("VkNew")
                )
            ),
            (
                key("type", "", "vkOld"),
                mark("legacy-two", &["VK_VERSION_1_1", "VK_KHR_apis"], None)
            ),
        ])
    );
    assert_eq!(
        legacy.entries,
        BTreeMap::from([
            ("command".to_string(), 3),
            ("feature".to_string(), 1),
            ("type".to_string(), 4)
        ])
    );
    assert_eq!(
        legacy.entries_of_other_apis, 7,
        "four in a version or an extension of another API, and three in an extension, a block or an entry with an `api` that does not name vulkan"
    );
    assert_eq!(
        legacy.type_aliases,
        BTreeSet::from(["VkOldEXT".to_string(), "VkOldKHR".to_string()])
    );
    assert_eq!(legacy.vendor_tags, ["NV", "KHR"]);
    assert_eq!(
        legacy.enumerations,
        BTreeSet::from(["VkOld".to_string(), "VkOldFlagBits".to_string()])
    );
    assert_eq!(
        ["type", "command", "feature", "enum"].map(|kind| legacy.names(kind)),
        [2, 1, 1, 0]
    );
}

#[test]
fn a_deprecate_block_that_the_test_cannot_read_stops_it() {
    for (replace, with, message) in [
        (
            "<command name=\"vkOther\"/>",
            "<enum name=\"VK_OTHER\"/>",
            "the <deprecate> block of VK_KHR_disabled has `<enum name=\"VK_OTHER\"/>`",
        ),
        (
            "<command name=\"vkOther\"/>",
            "<command name=\"vkOther\">",
            "the <deprecate> block of VK_KHR_disabled has `<command name=\"vkOther\">`",
        ),
        (
            "<deprecate explanationlink=\"legacy-two\">",
            "<deprecate>",
            "`<deprecate>` has no explanationlink",
        ),
        (
            "<type name=\"vkOld\"/>",
            "<type name=\"VkOld\"/>",
            "VK_VERSION_1_1 marks VkOld with `legacy-two` and None, and another block with `legacy-one` and Some(\"VkNew\")",
        ),
        (
            "<feature name=\"old\" struct=\"VkFeatures\"/>\n                <type name=\"VkOld\" supersededby=\"VkNew\"/>",
            "<type name=\"VkOld\" supersededby=\"VkNewer\"/>",
            "VK_KHR_new marks VkOld with `legacy-one` and Some(\"VkNewer\"), and another block with `legacy-one` and Some(\"VkNew\")",
        ),
    ] {
        assert_eq!(FIXTURE_REGISTRY.matches(replace).count(), 1, "{replace}");
        let registry = FIXTURE_REGISTRY.replacen(replace, with, 1);
        let panic = std::panic::catch_unwind(|| LegacyRegistry::read(&registry))
            .expect_err(&format!("`{with}` is read"));
        let panic = panic.downcast_ref::<String>().expect("a message");
        assert_eq!(panic, message);
    }
}

const FIXTURE_DEPRECATED_SOURCE: &str = r#"
/// The name of the device extension `VK_KHR_things`.
#[doc(alias = "VK_KHR_things")]
#[doc(alias = "VK_KHR_THINGS_EXTENSION_NAME")]
pub const KHR_THINGS_NAME: &CStr = c"VK_KHR_things";

/// C enumeration `VkShade`.
#[doc(alias = "VkShade")]
pub struct Shade(pub i32);

impl Shade {
    /// `VK_SHADE_DARK`, a value of `VkShade`.
    #[doc(alias = "VK_SHADE_DARK")]
    pub const DARK: Self = Self(0);
    /// `VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK).
    #[doc(alias = "VK_SHADE_DIM")]
    pub const DIM: Self = Self(0);
    /// `VK_SHADE_DUSK` is an alias of [`VK_SHADE_DARK`](Self::DARK).
    #[doc(alias = "VK_SHADE_DUSK")]
    pub const DUSK: Self = Self(0);
    /// `VK_SHADE_PALE`, a value of `VkShade`.
    ///
    /// Legacy functionality, for which the registry gives no reason.
    #[doc(alias = "VK_SHADE_PALE")]
    pub const PALE: Self = Self(1);
    /// `VK_SHADE_FAST_BIT`, a value of `VkShade`.
    ///
    /// The C header also calls this value `VK_SHADE_FAST`, which is a legacy alias of `VK_SHADE_FAST_BIT`.
    #[doc(alias = "VK_SHADE_FAST_BIT")]
    #[doc(alias = "VK_SHADE_FAST")]
    pub const FAST: Self = Self(2);
    /// `VK_SHADE_QUICK_BIT` is an alias of [`VK_SHADE_FAST_BIT`](Self::FAST).
    ///
    /// The C header also calls this value `VK_SHADE_QUICK` and `VK_SHADE_QUICK_BIT_BIT`, which are legacy aliases of [`VK_SHADE_FAST_BIT`](Self::FAST).
    #[doc(alias = "VK_SHADE_QUICK_BIT")]
    #[doc(alias = "VK_SHADE_QUICK")]
    #[doc(alias = "VK_SHADE_QUICK_BIT_BIT")]
    pub const QUICK: Self = Self(2);
}

/// C enumeration `VkTint`.
#[doc(alias = "VkTint")]
pub struct Tint(pub i32);

impl Tint {
    /// `VK_SHADE_DARK`, a value of `VkTint`.
    #[doc(alias = "VK_SHADE_DARK")]
    pub const DARK_SHADE: Self = Self(0);
    /// `VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK_SHADE).
    #[doc(alias = "VK_SHADE_DIM")]
    pub const DIM_SHADE: Self = Self(0);
}

/// C structure `VkLayers`.
#[doc(alias = "VkLayers")]
pub struct Layers {
    /// The C member `uint32_t layerCount`.
    ///
    /// Legacy functionality that is not used.
    pub layer_count: u32,
    /// The C member `uint32_t used`.
    pub used: u32,
}

impl Layers {
    /// Sets [`layer_count`](Self::layer_count).
    ///
    /// Legacy functionality that is not used.
    pub fn layer_count(mut self, value: u32) -> Self {
        self.layer_count = value;
        self
    }
    /// Sets [`used`](Self::used).
    pub fn used(mut self, value: u32) -> Self {
        self.used = value;
        self
    }
}
"#;

fn fixture_deprecated_specification(
    more: &[(&str, &str, &str, &str, Option<&str>)],
) -> Specification {
    let mut specification = fixture_specification();
    let attributes = [
        (
            "enum",
            "VkShade",
            "VK_SHADE_DIM",
            "aliased",
            Some("VK_SHADE_DARK"),
        ),
        ("enum", "VkShade", "VK_SHADE_PALE", "true", None),
        (
            "enum",
            "VkShade",
            "VK_SHADE_FAST",
            "aliased",
            Some("VK_SHADE_FAST_BIT"),
        ),
        (
            "enum",
            "VkShade",
            "VK_SHADE_QUICK",
            "aliased",
            Some("VK_SHADE_FAST_BIT"),
        ),
        (
            "enum",
            "VkShade",
            "VK_SHADE_QUICK_BIT_BIT",
            "aliased",
            Some("VK_SHADE_QUICK_BIT"),
        ),
        (
            "enum",
            "",
            "VK_KHR_THINGS1_EXTENSION_NAME",
            "aliased",
            Some("VK_KHR_THINGS_EXTENSION_NAME"),
        ),
        ("member", "VkLayers", "layerCount", "unused", None),
    ];
    for (element, owner, name, reason, alias) in attributes.iter().chain(more) {
        let key = (element.to_string(), owner.to_string(), name.to_string());
        let attribute = DeprecatedAttribute {
            reason: reason.to_string(),
            alias: alias.map(str::to_string),
        };
        specification.deprecated.attributes.insert(key, attribute);
        if let (Some(alias), "enum") = (alias, *element) {
            let aliases = &mut specification.deprecated.aliases;
            aliases.insert(name.to_string(), alias.to_string());
        }
    }
    let aliases = &mut specification.deprecated.aliases;
    aliases.insert("VK_SHADE_DUSK".to_string(), "VK_SHADE_DARK".to_string());
    aliases.insert(
        "VK_SHADE_QUICK_BIT".to_string(),
        "VK_SHADE_FAST_BIT".to_string(),
    );
    specification
}

fn fixture_deprecated_failures(
    more: &[(&str, &str, &str, &str, Option<&str>)],
    replace: &str,
    with: &str,
) -> Vec<String> {
    assert_eq!(
        FIXTURE_DEPRECATED_SOURCE.matches(replace).count(),
        1,
        "`{replace}` is not exactly once in the fixture"
    );
    let source = FIXTURE_DEPRECATED_SOURCE.replacen(replace, with, 1);
    fixture_legacy_checker(&fixture_deprecated_specification(more), &source).failures
}

#[test]
fn checker_accepts_what_the_bindings_say_about_the_deprecated_attributes_of_the_registry() {
    let specification = fixture_deprecated_specification(&[(
        "enum",
        "VkShade",
        "STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED",
        "aliased",
        Some("VK_SHADE_PALE"),
    )]);
    let checker = fixture_legacy_checker(&specification, FIXTURE_DEPRECATED_SOURCE);
    assert!(checker.failures.is_empty(), "{:#?}", checker.failures);
    assert_eq!(
        checker.legacy_attribute_statements,
        BTreeMap::from([
            ("constant that legacy aliases share", 2),
            ("legacy alias", 2),
            ("legacy value", 1),
            ("setter of an unused member", 1),
            ("unused member", 1),
        ])
    );
    let names = |names: &[&str]| names.iter().map(|name| name.to_string()).collect();
    assert_eq!(
        checker.legacy_aliases_without_a_constant,
        BTreeMap::from([
            (
                "LEGACY_ALIASES_THAT_THE_BINDINGS_DO_NOT_HAVE",
                names(&["STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED"])
            ),
            (
                "another name of the value differs from it only by `_BIT`, which the Rust names of flag bits leave out, so the two have one constant",
                names(&["VK_SHADE_FAST", "VK_SHADE_QUICK", "VK_SHADE_QUICK_BIT_BIT"])
            ),
            (
                "the legacy alias of the name or of the version macro of an extension",
                names(&["VK_KHR_THINGS1_EXTENSION_NAME"])
            ),
        ])
    );
}

const FIXTURE_DIM: &str =
    "    /// `VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK).\n";
const FIXTURE_PALE: &str = "    /// `VK_SHADE_PALE`, a value of `VkShade`.
    ///
    /// Legacy functionality, for which the registry gives no reason.
";
const FIXTURE_LAYER_COUNT: &str = "    /// The C member `uint32_t layerCount`.
    ///
    /// Legacy functionality that is not used.
";
const FIXTURE_LAYER_COUNT_SETTER: &str = "    /// Sets [`layer_count`](Self::layer_count).
    ///
    /// Legacy functionality that is not used.
";

#[test]
fn checker_reports_an_item_that_does_not_say_what_its_deprecated_attribute_says() {
    let alias = "`VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK).";
    for (defect, replace, with, failure) in [
        (
            "a legacy alias documented as an alias",
            FIXTURE_DIM,
            "    /// `VK_SHADE_DIM` is an alias of [`VK_SHADE_DARK`](Self::DARK).\n".to_string(),
            format!(
                "the value `Shade::DIM`: the registry calls it legacy, so its documentation has the one paragraph `{alias}`, but it has 0 of them: [\"`VK_SHADE_DIM` is an alias of [`VK_SHADE_DARK`](Self::DARK).\"]"
            ),
        ),
        (
            "a legacy alias documented as an alias of another value",
            FIXTURE_DIM,
            "    /// `VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_PALE`](Self::PALE).\n".to_string(),
            format!(
                "the value `Shade::DIM`: the registry calls it legacy, so its documentation has the one paragraph `{alias}`, but it has 0 of them: [\"`VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_PALE`](Self::PALE).\"]"
            ),
        ),
        (
            "a legacy alias that links the constant of another type",
            FIXTURE_DIM,
            "    /// `VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK_SHADE).\n"
                .to_string(),
            format!(
                "the value `Shade::DIM`: the registry calls it legacy, so its documentation has the one paragraph `{alias}`, but it has 0 of them: [\"`VK_SHADE_DIM` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK_SHADE).\"]"
            ),
        ),
        (
            "a legacy alias that says so twice",
            FIXTURE_DIM,
            format!("{FIXTURE_DIM}    ///\n{FIXTURE_DIM}"),
            format!(
                "the value `Shade::DIM`: the registry calls it legacy, so its documentation has the one paragraph `{alias}`, but it has 2 of them: [{alias:?}, {alias:?}]"
            ),
        ),
        (
            "a legacy value without the paragraph",
            FIXTURE_PALE,
            "    /// `VK_SHADE_PALE`, a value of `VkShade`.\n".to_string(),
            format!(
                "the value `Shade::PALE`: the registry calls it legacy, so its documentation has the one paragraph `{LEGACY_WITHOUT_A_REASON}`, but it has 0 of them: [\"`VK_SHADE_PALE`, a value of `VkShade`.\"]"
            ),
        ),
        (
            "an unused member without the paragraph",
            FIXTURE_LAYER_COUNT,
            "    /// The C member `uint32_t layerCount`.\n".to_string(),
            format!(
                "the member `Layers::layer_count`: the registry calls it legacy, so its documentation has the one paragraph `{LEGACY_AND_NOT_USED}`, but it has 0 of them: [\"The C member `uint32_t layerCount`.\"]"
            ),
        ),
        (
            "an unused member that says so twice",
            FIXTURE_LAYER_COUNT,
            "    /// `layerCount` is legacy and not used.\n    ///\n    /// Legacy functionality that is not used.\n".to_string(),
            "the member `Layers::layer_count`: says twice that it is legacy and not used".to_string(),
        ),
        (
            "the setter of an unused member without the paragraph",
            FIXTURE_LAYER_COUNT_SETTER,
            "    /// Sets [`layer_count`](Self::layer_count).\n".to_string(),
            format!(
                "the method `Layers::layer_count`: the registry calls it legacy, so its documentation has the one paragraph `{LEGACY_AND_NOT_USED}`, but it has 0 of them: [\"Sets [`layer_count`](Self::layer_count).\"]"
            ),
        ),
        (
            "an unused member without a setter",
            FIXTURE_LAYER_COUNT_SETTER,
            "    /// Reads [`layer_count`](Self::layer_count).\n".to_string(),
            "the member `Layers::layer_count`: is legacy and not used, and has no setter".to_string(),
        ),
    ] {
        assert_eq!(
            fixture_deprecated_failures(&[], replace, &with)
                .iter()
                .filter(|reported| !reported.contains(": is described although") && !reported.contains("is documented with the description"))
                .collect::<Vec<_>>(),
            [&failure],
            "{defect}"
        );
    }
    assert_eq!(
        fixture_deprecated_failures(
            &[],
            FIXTURE_LAYER_COUNT,
            "    /// The C member `uint32_t layerCount`, which is legacy and\n    /// not used.\n"
        ),
        Vec::<String>::new(),
        "a description that says it stands for the paragraph"
    );
}

#[test]
fn checker_reports_a_constant_that_does_not_name_the_legacy_aliases_that_share_it() {
    let fast = "The C header also calls this value `VK_SHADE_FAST`, which is a legacy alias of `VK_SHADE_FAST_BIT`.";
    let quick = "The C header also calls this value `VK_SHADE_QUICK` and `VK_SHADE_QUICK_BIT_BIT`, which are legacy aliases of [`VK_SHADE_FAST_BIT`](Self::FAST).";
    let fast_line = format!("    /// {fast}\n");
    let quick_line = format!("    /// {quick}\n");
    let of_fast = format!(
        "the value `Shade::FAST`: the legacy aliases of the registry that have its Rust name and no constant of their own are [\"VK_SHADE_FAST\"], so it has these as `#[doc(alias)]` after its own name and the one paragraph `{fast}`, but it has"
    );
    let of_quick = format!(
        "the value `Shade::QUICK`: the legacy aliases of the registry that have its Rust name and no constant of their own are [\"VK_SHADE_QUICK\", \"VK_SHADE_QUICK_BIT_BIT\"], so it has these as `#[doc(alias)]` after its own name and the one paragraph `{quick}`, but it has"
    );
    let of_dark = "the value `Shade::DARK`: the legacy aliases of the registry that have its Rust name and no constant of their own are [], so it has these as `#[doc(alias)]` after its own name and no paragraph about such names, but it has";
    let dark = "    #[doc(alias = \"VK_SHADE_DARK\")]\n    pub const DARK:";
    for (defect, replace, with, failure) in [
        (
            "a shared constant without the paragraph",
            fast_line.as_str(),
            "    /// Another name.\n".to_string(),
            format!("{of_fast} the names [\"VK_SHADE_FAST\"] and the paragraphs []"),
        ),
        (
            "a shared constant with the paragraph twice",
            fast_line.as_str(),
            format!("{fast_line}    ///\n{fast_line}"),
            format!(
                "{of_fast} the names [\"VK_SHADE_FAST\"] and the paragraphs [{fast:?}, {fast:?}]"
            ),
        ),
        (
            "a shared constant that cannot be found by the legacy name",
            "    #[doc(alias = \"VK_SHADE_FAST\")]\n",
            String::new(),
            format!("{of_fast} the names [] and the paragraphs [{fast:?}]"),
        ),
        (
            "a shared constant that names the legacy alias as an alias of the constant of another name",
            fast_line.as_str(),
            fast_line.replace("`VK_SHADE_FAST_BIT`.", "[`VK_SHADE_FAST_BIT`](Self::FAST)."),
            format!(
                "{of_fast} the names [\"VK_SHADE_FAST\"] and the paragraphs [{:?}]",
                fast.replace("`VK_SHADE_FAST_BIT`.", "[`VK_SHADE_FAST_BIT`](Self::FAST).")
            ),
        ),
        (
            "a shared constant of an alias that does not link the constant of the value",
            quick_line.as_str(),
            quick_line.replace("[`VK_SHADE_FAST_BIT`](Self::FAST)", "`VK_SHADE_QUICK_BIT`"),
            format!(
                "{of_quick} the names [\"VK_SHADE_QUICK\", \"VK_SHADE_QUICK_BIT_BIT\"] and the paragraphs [{:?}]",
                quick.replace("[`VK_SHADE_FAST_BIT`](Self::FAST)", "`VK_SHADE_QUICK_BIT`")
            ),
        ),
        (
            "a shared constant that names one of two legacy aliases",
            quick_line.as_str(),
            quick_line.replace(
                "`VK_SHADE_QUICK` and `VK_SHADE_QUICK_BIT_BIT`, which are legacy aliases",
                "`VK_SHADE_QUICK`, which is a legacy alias",
            ),
            format!(
                "{of_quick} the names [\"VK_SHADE_QUICK\", \"VK_SHADE_QUICK_BIT_BIT\"] and the paragraphs [{:?}]",
                quick.replace(
                    "`VK_SHADE_QUICK` and `VK_SHADE_QUICK_BIT_BIT`, which are legacy aliases",
                    "`VK_SHADE_QUICK`, which is a legacy alias"
                )
            ),
        ),
        (
            "a shared constant that cannot be found by one of two legacy names",
            "    #[doc(alias = \"VK_SHADE_QUICK_BIT_BIT\")]\n",
            String::new(),
            format!("{of_quick} the names [\"VK_SHADE_QUICK\"] and the paragraphs [{quick:?}]"),
        ),
        (
            "a constant of its own that says it is shared",
            dark,
            format!("    ///\n{fast_line}{dark}"),
            format!("{of_dark} the names [] and the paragraphs [{fast:?}]"),
        ),
        (
            "a constant of its own with a second name",
            dark,
            "    #[doc(alias = \"VK_SHADE_DARK\")]\n    #[doc(alias = \"VK_SHADE_FAST\")]\n    pub const DARK:"
                .to_string(),
            format!("{of_dark} the names [\"VK_SHADE_FAST\"] and the paragraphs []"),
        ),
    ] {
        assert_eq!(
            fixture_deprecated_failures(&[], replace, &with),
            [failure],
            "{defect}"
        );
    }
    assert_eq!(
        fixture_deprecated_failures(
            &[],
            "/// C structure `VkLayers`.\n",
            &format!("/// C structure `VkLayers`.\n///\n/// {fast}\n")
        ),
        [format!(
            "the type `VkLayers`: the legacy aliases of the registry that have its Rust name and no constant of their own are [], so it has these as `#[doc(alias)]` after its own name and no paragraph about such names, but it has the names [] and the paragraphs [{fast:?}]"
        )]
    );
}

#[test]
fn checker_reports_an_item_that_says_it_is_legacy_without_a_deprecated_attribute() {
    let unmarked = "says that the registry calls it legacy, but the registry has no `deprecated` attribute for it";
    for (defect, replace, with, item) in [
        (
            "an alias called legacy",
            "    /// `VK_SHADE_DUSK` is an alias of [`VK_SHADE_DARK`](Self::DARK).\n",
            "    /// `VK_SHADE_DUSK` is a legacy alias of [`VK_SHADE_DARK`](Self::DARK).\n",
            "the value `Shade::DUSK`",
        ),
        (
            "a value called legacy",
            "    /// `VK_SHADE_DARK`, a value of `VkShade`.\n",
            "    /// `VK_SHADE_DARK`, a value of `VkShade`.\n    ///\n    /// Legacy functionality, for which the registry gives no reason.\n",
            "the value `Shade::DARK`",
        ),
        (
            "a member called unused",
            "    /// The C member `uint32_t used`.\n",
            "    /// The C member `uint32_t used`.\n    ///\n    /// Legacy functionality that is not used.\n",
            "the member `Layers::used`",
        ),
        (
            "a setter of a member that is used",
            "    /// Sets [`used`](Self::used).\n",
            "    /// Sets [`used`](Self::used).\n    ///\n    /// Legacy functionality that is not used.\n",
            "the method `Layers::used`",
        ),
        (
            "a type called legacy",
            "/// C structure `VkLayers`.\n",
            "/// C structure `VkLayers`.\n///\n/// Legacy functionality that is not used.\n",
            "the type `VkLayers`",
        ),
    ] {
        assert_eq!(
            fixture_deprecated_failures(&[], replace, with),
            [format!("{item}: {unmarked}")],
            "{defect}"
        );
    }
}

#[test]
fn checker_reports_a_deprecated_attribute_that_the_bindings_do_not_reflect() {
    let dim = FIXTURE_DIM;
    for (attribute, failure) in [
        (
            (
                "enum",
                "VkShade",
                "VK_SHADE_GONE",
                "aliased",
                Some("VK_SHADE_DARK"),
            ),
            "VK_SHADE_GONE: is a legacy alias of `VK_SHADE_DARK` in `VkShade`, and the bindings have no constant for it",
        ),
        (
            (
                "enum",
                "VkShade",
                "VK_SHADE_DARK_BIT_BIT",
                "aliased",
                Some("VK_SHADE_GONE"),
            ),
            "VK_SHADE_DARK_BIT_BIT: is a legacy alias of `VK_SHADE_GONE` in `VkShade`, and the bindings have no constant for it",
        ),
        (
            (
                "enum",
                "VkShade",
                "STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED",
                "aliased",
                Some("VK_SHADE_GONE"),
            ),
            "STD_VIDEO_AV1_COLOR_PRIMARIES_BT_UNSPECIFIED: is a legacy alias of `VK_SHADE_GONE` in `VkShade`, and the bindings have no constant for it",
        ),
        (
            ("enum", "VkShade", "VK_SHADE_GONE", "true", None),
            "VK_SHADE_GONE: is legacy, and the bindings have no constant for it in `VkShade`",
        ),
        (
            (
                "enum",
                "",
                "VK_KHR_OTHER1_EXTENSION_NAME",
                "aliased",
                Some("VK_KHR_OTHER_EXTENSION_NAME"),
            ),
            "VK_KHR_OTHER1_EXTENSION_NAME: is a legacy alias of `VK_KHR_OTHER_EXTENSION_NAME`, and the bindings leave out only the legacy aliases of the name and of the version macro of an extension that they have, and have no item for them",
        ),
        (
            (
                "enum",
                "",
                "VK_SHADE_GONE",
                "aliased",
                Some("VK_KHR_things"),
            ),
            "VK_SHADE_GONE: is a legacy alias of `VK_KHR_things`, and the bindings leave out only the legacy aliases of the name and of the version macro of an extension that they have, and have no item for them",
        ),
        (
            (
                "enum",
                "",
                "VK_KHR_things",
                "aliased",
                Some("VK_KHR_THINGS_EXTENSION_NAME"),
            ),
            "VK_KHR_things: is a legacy alias of `VK_KHR_THINGS_EXTENSION_NAME`, and the bindings leave out only the legacy aliases of the name and of the version macro of an extension that they have, and have no item for them",
        ),
        (
            ("member", "VkLayers", "gone", "unused", None),
            "VkLayers::gone: is legacy and not used, and the bindings have no such member",
        ),
        (
            ("member", "VkGone", "layerCount", "unused", None),
            "VkGone::layerCount: is legacy and not used, and the bindings have no such member",
        ),
        (
            ("member", "VkLayers", "used", "true", None),
            "used: has the attribute `deprecated` as DeprecatedAttribute { reason: \"true\", alias: None } on <member> of `VkLayers`, which the test does not know how the bindings state",
        ),
        (
            ("enum", "VkShade", "VK_SHADE_DARK", "unused", None),
            "VK_SHADE_DARK: has the attribute `deprecated` as DeprecatedAttribute { reason: \"unused\", alias: None } on <enum> of `VkShade`, which the test does not know how the bindings state",
        ),
        (
            ("enum", "VkShade", "VK_SHADE_DARK", "aliased", None),
            "VK_SHADE_DARK: has the attribute `deprecated` as DeprecatedAttribute { reason: \"aliased\", alias: None } on <enum> of `VkShade`, which the test does not know how the bindings state",
        ),
    ] {
        assert_eq!(
            fixture_deprecated_failures(&[attribute], dim, dim),
            [failure],
            "{attribute:?}"
        );
    }
}

const FIXTURE_DEPRECATED_REGISTRY: &str = r#"
<registry>
    <types>
        <type category="struct" name="VkLayers">
            <member deprecated="unused"><type>uint32_t</type> <name>layerCount</name></member>
            <member><type>uint32_t</type> <name>used</name></member>
        </type>
        <type category="basetype">typedef <type>uint32_t</type> <name>VkBool32</name>;</type>
        <type category="struct" name="VkLayersKHR" alias="VkLayers"/>
    </types>
    <enums name="API Constants" type="constants">
        <enum name="VK_OLD_MAX" alias="VK_MAX" deprecated="aliased"/>
    </enums>
    <enums name="VkShade" type="enum">
        <enum value="0" name="VK_SHADE_DARK"/>
        <enum api="vulkan" name="VK_SHADE_DIM" alias="VK_SHADE_DARK" deprecated="aliased"/>
        <enum api="vulkansc" name="VK_SHADE_DIM" alias="VK_SHADE_PALE" deprecated="aliased"/>
        <enum name="VK_SHADE_DUSK" alias="VK_SHADE_DIM"/>
    </enums>
    <enums api="vulkansc" name="VkSafeShade" type="enum">
        <enum name="VK_SAFE_SHADE_DIM" alias="VK_SAFE_SHADE_DARK" deprecated="aliased"/>
    </enums>
    <enums name="VkTintFlagBits" type="bitmask">
        <enum bitpos="0" name="VK_TINT_FAST_BIT" deprecated="true"/>
    </enums>
    <feature api="vulkan,vulkansc" name="VK_VERSION_1_1" number="1.1">
        <require>
            <feature name="old" struct="VkFeatures"/>
            <enum extends="VkShade" name="VK_SHADE_DIM" alias="VK_SHADE_DARK" deprecated="aliased"/>
        </require>
        <require api="vulkansc">
            <enum extends="VkShade" name="VK_SHADE_SAFE" alias="VK_SHADE_DARK" deprecated="aliased"/>
        </require>
    </feature>
    <feature api="vulkansc" name="VKSC_VERSION_1_0" number="1.0">
        <require>
            <enum extends="VkShade" name="VK_SHADE_SAFER" alias="VK_SHADE_DARK" deprecated="aliased"/>
        </require>
    </feature>
    <extensions>
        <extension name="VK_KHR_things" number="1" supported="vulkan" deprecatedby="VK_KHR_other">
            <require>
                <enum name="VK_KHR_THINGS1_EXTENSION_NAME" alias="VK_KHR_THINGS_EXTENSION_NAME" deprecated="aliased"/>
                <enum extends="VkTintFlagBits" name="VK_TINT_FAST" alias="VK_TINT_FAST_BIT" deprecated="aliased"/>
            </require>
        </extension>
        <extension name="VK_KHR_disabled" number="2" supported="disabled">
            <require>
                <enum extends="VkShade" name="VK_SHADE_OFF" alias="VK_SHADE_DARK" deprecated="aliased"/>
            </require>
        </extension>
        <extension name="VK_KHR_other_api" number="3" supported="vulkan" api="vulkansc">
            <require>
                <enum extends="VkShade" name="VK_SHADE_OTHER" alias="VK_SHADE_DARK" deprecated="aliased"/>
            </require>
        </extension>
    </extensions>
</registry>
"#;

#[test]
fn the_deprecated_attributes_of_the_registry_are_read_for_the_vulkan_api() {
    let mut deprecated = DeprecatedAttributes::default();
    deprecated.read(FIXTURE_DEPRECATED_REGISTRY);
    let attribute = |reason: &str, alias: Option<&str>| DeprecatedAttribute {
        reason: reason.to_string(),
        alias: alias.map(str::to_string),
    };
    let key = |element: &str, owner: &str, name: &str| {
        (element.to_string(), owner.to_string(), name.to_string())
    };
    assert_eq!(
        deprecated.attributes,
        BTreeMap::from([
            (
                key("enum", "", "VK_KHR_THINGS1_EXTENSION_NAME"),
                attribute("aliased", Some("VK_KHR_THINGS_EXTENSION_NAME"))
            ),
            (
                key("enum", "", "VK_OLD_MAX"),
                attribute("aliased", Some("VK_MAX"))
            ),
            (
                key("enum", "VkShade", "VK_SHADE_DIM"),
                attribute("aliased", Some("VK_SHADE_DARK"))
            ),
            (
                key("enum", "VkTintFlagBits", "VK_TINT_FAST"),
                attribute("aliased", Some("VK_TINT_FAST_BIT"))
            ),
            (
                key("enum", "VkTintFlagBits", "VK_TINT_FAST_BIT"),
                attribute("true", None)
            ),
            (
                key("member", "VkLayers", "layerCount"),
                attribute("unused", None)
            ),
        ])
    );
    assert_eq!(
        deprecated.entries,
        BTreeMap::from([
            ("aliased".to_string(), 5),
            ("true".to_string(), 1),
            ("unused".to_string(), 1)
        ])
    );
    assert_eq!(
        deprecated.entries_of_other_apis, 6,
        "an element, a group, a block and a version of another API, and an extension that is disabled or of another API"
    );
    assert_eq!(deprecated.value_of("VK_SHADE_DUSK"), "VK_SHADE_DARK");
    assert_eq!(deprecated.value_of("VK_SHADE_DARK"), "VK_SHADE_DARK");
    assert_eq!(deprecated.value_of("VK_SHADE_OFF"), "VK_SHADE_OFF");
    assert_eq!(
        deprecated.other_names("VK_SHADE_DIM"),
        ["VK_SHADE_DUSK", "VK_SHADE_DARK"]
    );
    assert_eq!(
        deprecated.other_names("VK_SHADE_DARK"),
        ["VK_SHADE_DIM", "VK_SHADE_DUSK"]
    );
    assert_eq!(deprecated.other_names("VK_TINT_FAST_BIT"), ["VK_TINT_FAST"]);
}

#[test]
fn a_deprecated_attribute_that_the_test_cannot_read_stops_it() {
    for (replace, with, message) in [
        (
            "<enum bitpos=\"0\" name=\"VK_TINT_FAST_BIT\" deprecated=\"true\"/>",
            "<enum bitpos=\"0\" name=\"VK_TINT_FAST_BIT\" deprecated=\"ignored\"/>",
            "the test does not know the value of `deprecated` in `<enum bitpos=\"0\" name=\"VK_TINT_FAST_BIT\" deprecated=\"ignored\"/>`",
        ),
        (
            "<type category=\"struct\" name=\"VkLayersKHR\" alias=\"VkLayers\"/>",
            "<type category=\"struct\" name=\"VkLayersKHR\" alias=\"VkLayers\" deprecated=\"aliased\"/>",
            "the test knows the attribute `deprecated` on <enum> and <member>, and the registry has `<type category=\"struct\" name=\"VkLayersKHR\" alias=\"VkLayers\" deprecated=\"aliased\"/>`",
        ),
        (
            "<enum extends=\"VkTintFlagBits\" name=\"VK_TINT_FAST\" alias=\"VK_TINT_FAST_BIT\" deprecated=\"aliased\"/>",
            "<enum extends=\"VkShade\" name=\"VK_SHADE_DIM\" alias=\"VK_SHADE_DUSK\" deprecated=\"aliased\"/>",
            "assertion `left == right` failed: `<enum extends=\"VkShade\" name=\"VK_SHADE_DIM\" alias=\"VK_SHADE_DUSK\" deprecated=\"aliased\"/>` and another element say different things\n  left: DeprecatedAttribute { reason: \"aliased\", alias: Some(\"VK_SHADE_DARK\") }\n right: DeprecatedAttribute { reason: \"aliased\", alias: Some(\"VK_SHADE_DUSK\") }",
        ),
    ] {
        assert_eq!(
            FIXTURE_DEPRECATED_REGISTRY.matches(replace).count(),
            1,
            "{replace}"
        );
        let registry = FIXTURE_DEPRECATED_REGISTRY.replacen(replace, with, 1);
        let panic = std::panic::catch_unwind(|| DeprecatedAttributes::default().read(&registry))
            .expect_err(&format!("`{with}` is read"));
        let panic = panic.downcast_ref::<String>().expect("a message");
        assert_eq!(panic, message);
    }
}

const FIXTURE_LIMITS_SOURCE: &str = r#"
/// Structure of limits.
///
/// C structure [`VkLimits`](PAGES/VkLimits.html).
#[doc(alias = "VkLimits")]
pub struct Limits {
    /// `bias` is clamped. See
    /// [\[things-flags\]](SPEC/chapters/things.html#things-flags).
    pub bias: f32,
    /// `size` is small. See A Title and the flags.
    pub size: f32,
}
"#;

const FIXTURE_LIMITS_LINK: &str =
    "    /// [\\[things-flags\\]](SPEC/chapters/things.html#things-flags).\n";

fn fixture_limits_failures(reference: &str, replace: &str, with: &str) -> Vec<String> {
    let mut specification = fixture_specification();
    for (anchor, files) in [
        ("things-flags", &["chapters/things.adoc"][..]),
        ("things-partial", &["appendices/VK_KHR_things.adoc"]),
        (
            "things-twice",
            &["chapters/things.adoc", "chapters/more.adoc"],
        ),
    ] {
        let files = files.iter().map(|file| file.to_string()).collect();
        specification.anchors.insert(anchor.to_string(), files);
    }
    specification.published = ["chapters/things.adoc".to_string()].into();
    let titles = &mut specification.titles;
    titles.insert("things-titled".to_string(), "A Title".to_string());
    let body = format!(
        "
  * pname:bias is clamped.
    See {reference}.
  * pname:size is small.
    See <<things-titled>> and <<things-flags, the flags>>."
    );
    let page = Page {
        name: "VkLimits".to_string(),
        file: FIXTURE_FILE.to_string(),
        description: "Structure of limits".to_string(),
        body_words: specification.words(&body),
        body,
        valid_usage: OnceCell::new(),
    };
    specification.pages.insert("VkLimits".to_string(), page);
    assert_eq!(
        FIXTURE_LIMITS_SOURCE.matches(replace).count(),
        1,
        "{replace}"
    );
    let source = FIXTURE_LIMITS_SOURCE.replacen(replace, with, 1);
    let checker = fixture_legacy_checker(&specification, &source);
    if checker.failures.is_empty() {
        assert_eq!(checker.untitled_anchors, ["Limits::bias: things-flags"]);
        assert_eq!(checker.links_of_untitled_anchors, 1);
        assert_eq!(
            specification.every_untitled_reference(),
            BTreeSet::from([("VkLimits".to_string(), "things-flags".to_string())])
        );
    }
    checker.failures
}

#[test]
fn checker_accepts_a_link_for_a_reference_without_a_text_to_an_anchor_without_a_title() {
    let link = FIXTURE_LIMITS_LINK;
    assert_eq!(
        fixture_limits_failures("<<things-flags>>", link, link),
        Vec::<String>::new()
    );
}

#[test]
fn checker_reports_a_reference_to_an_untitled_anchor_that_is_not_the_link_of_its_page() {
    let expected = "Limits::bias: the description of `bias` refers to the anchor `things-flags`, which has no title, so the specification shows the link `[things-flags]`, but the documentation has no `[\\[things-flags\\]](https://docs.vulkan.org/spec/latest/chapters/things.html#things-flags)`";
    let unreferenced = |anchor: &str| {
        format!(
            "Limits::bias: the description of `bias` links the anchor `{anchor}`, which its list item on the page `VkLimits` does not refer to without a text, or which has a title"
        )
    };
    for (defect, with, failures) in [
        (
            "the name of the anchor without a link",
            "    /// \\[things-flags\\].\n",
            vec![expected.to_string()],
        ),
        (
            "a link to another page",
            "    /// [\\[things-flags\\]](SPEC/chapters/more.html#things-flags).\n",
            vec![expected.to_string()],
        ),
        (
            "a link to another anchor of the page",
            "    /// [\\[things-flags\\]](SPEC/chapters/things.html#things).\n",
            vec![expected.to_string()],
        ),
        (
            "a link without the brackets of the name",
            "    /// [things-flags](SPEC/chapters/things.html#things-flags).\n",
            vec![
                "Limits: has a docs.vulkan.org link of an unexpected form".to_string(),
                expected.to_string(),
            ],
        ),
    ] {
        assert_eq!(
            fixture_limits_failures("<<things-flags>>", FIXTURE_LIMITS_LINK, with),
            failures,
            "{defect}"
        );
    }
    let link = FIXTURE_LIMITS_LINK;
    assert!(
        fixture_limits_failures("<<things-titled>>", link, link)
            .contains(&unreferenced("things-flags")),
        "a link where the anchor has a title"
    );
    assert!(
        fixture_limits_failures("<<things-flags, the flags>>", link, link)
            .contains(&unreferenced("things-flags")),
        "a link where the reference has a text"
    );
    for (reference, problem) in [
        (
            "<<things-partial>>",
            "the anchor `things-partial` is in `appendices/VK_KHR_things.adoc`, which is not published as a page of its own",
        ),
        (
            "<<things-twice>>",
            "2 source files of the specification define the anchor `things-twice`",
        ),
        (
            "<<things-missing>>",
            "0 source files of the specification define the anchor `things-missing`",
        ),
    ] {
        let anchor = reference.trim_matches(['<', '>']);
        let with = format!("    /// [\\[{anchor}\\]](SPEC/chapters/things.html#{anchor}).\n");
        assert_eq!(
            fixture_limits_failures(reference, link, &with),
            [format!(
                "Limits::bias: the description of `bias` refers to the anchor `{anchor}`, which has no title, and {problem}"
            )],
            "{reference}"
        );
    }
}
