pub(crate) mod asciidoc;
mod items;
mod legacy;
mod links;
mod other_names;
mod render;
mod tables;
mod value_descriptions;

pub(crate) use legacy::{
    LegacyMarks, assert_every_legacy_mark_has_an_item, assert_listed_legacy_macros_are_marked,
    legacy_marks,
};
pub(crate) use links::Links;
pub(crate) use other_names::assert_every_other_name_is_in_use;
pub(crate) use render::{Block, Doc};
pub(crate) use tables::{Table, read as read_table};
pub(crate) use value_descriptions::{ValueDescriptions, value_descriptions};

use crate::model::{Cfg, Model, parse_version};
use crate::naming::type_name;
use asciidoc::{Converted, Converter, Link, Unsupported, code_span};
use items::Part;
use std::collections::{BTreeMap, BTreeSet};
use tables::Cell;
use value_descriptions::Place;
use vulkan_registry::{DocPage, DocTable};

pub(crate) const REFERENCE_PAGES: &str = "https://docs.vulkan.org/refpages/latest/refpages/source/";

pub(crate) fn reference_url(page: &str) -> String {
    format!("{REFERENCE_PAGES}{page}.html")
}

pub(crate) fn sentence(text: &str) -> String {
    let text = text.trim();
    if text.is_empty() || text.ends_with(['.', '!', '?']) {
        text.to_string()
    } else {
        format!("{text}.")
    }
}

pub(crate) fn join_or(mut items: Vec<String>) -> String {
    match items.len() {
        0 | 1 => items.pop().unwrap_or_default(),
        _ => {
            let last = items.pop().expect("more than one item");
            format!("{} or {last}", items.join(", "))
        }
    }
}

pub(crate) fn join_and(mut items: Vec<String>) -> String {
    match items.len() {
        0 | 1 => items.pop().unwrap_or_default(),
        _ => {
            let last = items.pop().expect("more than one item");
            format!("{} and {last}", items.join(", "))
        }
    }
}

impl<'a> Model<'a> {
    pub(crate) fn page(&self, c_name: &str) -> Option<&'a DocPage> {
        self.pages
            .get(c_name)
            .or_else(|| self.pages.get(self.canonical(c_name)))
            .or_else(|| self.pages.get(*self.command_aliases.get(c_name)?))
            .copied()
    }

    pub(crate) fn reference(&self, c_name: &str) -> String {
        match self.page(c_name) {
            Some(page) => format!("[`{c_name}`]({})", reference_url(&page.name)),
            None => format!("`{c_name}`"),
        }
    }

    pub(crate) fn extension_reference(&self, extension: &str) -> String {
        format!("[`{extension}`]({})", reference_url(extension))
    }

    pub(crate) fn link(&self, from: &Cfg, c_name: &str) -> String {
        match self.links.get(c_name) {
            Some(target) if target.cfg.is_implied_by(from) => {
                format!("[`{c_name}`]({})", target.path)
            }
            _ => format!("`{c_name}`"),
        }
    }

    fn markdown(&self, from: &Cfg, place: Place<'_>, own: &str, text: &str) -> String {
        self.try_markdown(from, own, text)
            .unwrap_or_else(|unsupported| {
                panic!(
                    "the description of {own} on {place} has no Markdown form \
                     ({unsupported:?}): {text}"
                )
            })
    }

    fn cell_markdown(
        &self,
        from: &Cfg,
        source: &DocTable,
        item: &str,
        text: &str,
    ) -> Option<String> {
        match self.try_markdown(from, item, text) {
            Ok(markdown) => Some(markdown),
            Err(Unsupported::UnknownMacro(name)) => panic!(
                "a table of {} uses the AsciiDoc macro `{name}:`, which the generator does not \
                 know: {text}",
                source.page.as_deref().unwrap_or(&source.file)
            ),
            Err(
                unread @ (Unsupported::EscapedSpan
                | Unsupported::Passthrough
                | Unsupported::MacroUse(_)),
            ) => panic!(
                "a table of {} has markup that the generator does not read ({unread:?}): {text}",
                source.page.as_deref().unwrap_or(&source.file)
            ),
            Err(_) => None,
        }
    }

    fn try_markdown(&self, from: &Cfg, own: &str, text: &str) -> Converted {
        let resolve = |link: Link, name: &str| {
            if name == own {
                return None;
            }
            if link == Link::ReferencePage {
                return self.pages.contains_key(name).then(|| reference_url(name));
            }
            let target = self.links.get(name)?;
            target.cfg.is_implied_by(from).then(|| target.path.clone())
        };
        Converter {
            attributes: &self.docs.attributes,
            titles: &self.docs.titles,
            resolve: &resolve,
        }
        .inline(text)
    }

    pub(crate) fn summary(&self, from: &Cfg, c_name: &str) -> Option<String> {
        let page = self.page(c_name)?;
        let summary = self.markdown(from, Place::Page(page), c_name, &page.desc);
        (!summary.trim().is_empty()).then(|| sentence(&summary))
    }

    pub(crate) fn described(&self, from: &Cfg, owner: &str, item: &str) -> Option<Vec<Block>> {
        let page = self.page(owner)?;
        let Some(described) = other_names::described(page, item) else {
            return self.described_outside_the_page(from, item);
        };
        let mut blocks = self.description(from, Place::Page(page), item, &described.item.text);
        if let (Some(other), Some(Block::Text(lead))) =
            (described.under_the_name, blocks.first_mut())
        {
            *lead = format!(
                "`{item}`, which the specification describes under the name `{other}`: {lead}"
            );
        }
        Some(blocks)
    }

    fn described_outside_the_page(&self, from: &Cfg, value: &str) -> Option<Vec<Block>> {
        let descriptions = self.value_descriptions.get(value)?;
        let [description] = descriptions.as_slice() else {
            let places = descriptions
                .iter()
                .map(|description| description.place.to_string())
                .collect::<Vec<_>>();
            panic!(
                "the reference page of its type does not describe {value}, and {} list items \
                 elsewhere do, so that nothing says which one to take: {}",
                places.len(),
                places.join(", ")
            );
        };
        Some(self.description(from, description.place, value, description.text))
    }

    fn description(&self, from: &Cfg, place: Place<'_>, item: &str, text: &str) -> Vec<Block> {
        let mut parts = item_parts(place, item, text);
        let notes = notes_of(place, &place.items_with(item, text));
        items::take_notes(&mut parts);
        let marks = match notes.is_empty() {
            true => BTreeSet::new(),
            false => items::refer_to_notes(&mut parts),
        };
        let mut blocks = self.blocks(from, place, item, parts);
        for number in marks {
            let mut note = vec![Block::Text(format!("**Note {number}**"))];
            note.extend(self.blocks(from, place, item, notes[&number].clone()));
            blocks.push(Block::Quote(note));
        }
        blocks
    }

    fn blocks(&self, from: &Cfg, place: Place<'_>, own: &str, parts: Vec<Part>) -> Vec<Block> {
        let mut blocks = Vec::new();
        for part in parts {
            blocks.push(match part {
                Part::Paragraph(text) => Block::Text(without_space_before_punctuation(
                    &self.markdown(from, place, own, &text),
                )),
                Part::List(items) => Block::List(
                    items
                        .into_iter()
                        .map(|item| self.blocks(from, place, own, item))
                        .collect(),
                ),
                Part::Terms(items) => Block::List(
                    items
                        .into_iter()
                        .map(|(term, parts)| {
                            let mut item =
                                vec![Block::Text(self.markdown(from, place, own, &term))];
                            item.extend(self.blocks(from, place, own, parts));
                            item
                        })
                        .collect(),
                ),
                Part::Admonition {
                    label,
                    title,
                    parts,
                } => {
                    let caption = match title {
                        Some(title) => {
                            format!("**{label}: {}**", self.markdown(from, place, own, &title))
                        }
                        None => format!("**{label}**"),
                    };
                    let mut quoted = vec![Block::Text(caption)];
                    quoted.extend(self.blocks(from, place, own, parts));
                    Block::Quote(quoted)
                }
                Part::Math(formula) => Block::Text(code_span(&formula)),
                Part::Listing { language, text } => Block::Code { language, text },
            });
        }
        blocks
    }

    fn values_of(&self, owner: &str) -> BTreeSet<&'a str> {
        let enum_fields = self
            .ir
            .enums
            .iter()
            .filter(|e| e.name == owner)
            .flat_map(|e| &e.fields);
        let bitmask_fields = self
            .ir
            .bitmasks
            .iter()
            .filter(|b| b.name == owner)
            .flat_map(|b| &b.fields);
        enum_fields
            .chain(bitmask_fields)
            .flat_map(|field| std::iter::once(&field.name).chain(&field.aliases))
            .map(String::as_str)
            .collect()
    }

    pub(crate) fn tabulated(&self, from: &Cfg, owner: &str, item: &str) -> Option<Vec<Block>> {
        let page = self.page(owner)?;
        let values = self.values_of(owner);
        let about_the_type = |table: &Table| {
            table.title.is_some()
                && table
                    .rows
                    .iter()
                    .all(|row| value_named(row).is_some_and(|value| values.contains(value)))
        };
        let rows_in = |outside: bool| {
            self.tables
                .iter()
                .filter(|(source, table)| match &source.page {
                    Some(table_page) => !outside && *table_page == page.name,
                    None => outside && source.file == page.file && about_the_type(table),
                })
                .flat_map(|(source, table)| table.rows.iter().map(move |row| (*source, table, row)))
                .filter(|(_, _, row)| value_named(row) == Some(item))
                .collect::<Vec<_>>()
        };
        let mut rows = rows_in(false);
        if rows.is_empty() {
            rows = rows_in(true);
        }
        let blocks = rows
            .into_iter()
            .filter_map(|(source, table, row)| self.table_row(from, source, item, table, row))
            .flatten()
            .collect::<Vec<_>>();
        (!blocks.is_empty()).then_some(blocks)
    }

    fn table_row(
        &self,
        from: &Cfg,
        source: &DocTable,
        item: &str,
        table: &Table,
        row: &[Cell],
    ) -> Option<Vec<Block>> {
        let header = table.header.as_ref()?;
        let mut items = Vec::new();
        for (label, cell) in header.iter().zip(row).skip(1) {
            let Cell::Lines(lines) = cell else {
                return None;
            };
            if lines.is_empty() {
                continue;
            }
            let label =
                self.cell_markdown(from, source, item, without_emphasis(label.single_line()?))?;
            if label.trim().is_empty() {
                return None;
            }
            let mut lines = lines
                .iter()
                .map(|line| self.cell_markdown(from, source, item, line))
                .collect::<Option<Vec<_>>>()?;
            if lines.len() == 1 {
                items.push(vec![Block::Text(format!("{label}: {}", lines.remove(0)))]);
            } else {
                items.push(vec![Block::Text(format!("{label}:")), Block::list(lines)]);
            }
        }
        if items.is_empty() {
            return None;
        }
        let lead = match &table.title {
            Some(title) => format!(
                "`{item}` in the table “{}” of the specification:",
                self.cell_markdown(from, source, item, title)?
            ),
            None => format!("`{item}` in a table of the specification:"),
        };
        Some(vec![Block::Text(lead), Block::List(items)])
    }

    pub(crate) fn provenance(
        &self,
        from: &Cfg,
        version: Option<&str>,
        extensions: &[String],
        video_std_header: Option<&str>,
    ) -> String {
        if let Some(header) = video_std_header {
            return format!("the Vulkan Video Std header `{header}`");
        }
        let mut sources = Vec::new();
        match version {
            Some(version) => {
                let (major, minor) = parse_version(version)
                    .unwrap_or_else(|| panic!("{version} is not a Vulkan version"));
                sources.push(format!("Vulkan {major}.{minor}"));
            }
            None if extensions.is_empty() => sources.push("Vulkan 1.0".to_string()),
            None => {}
        }
        sources.extend(extensions.iter().map(|name| self.link(from, name)));
        join_or(sources)
    }

    pub(crate) fn type_doc(&self, from: &Cfg, c_name: &str, kind: &str, provenance: &str) -> Doc {
        let reference = self.reference(c_name);
        let origin = if provenance.is_empty() {
            format!("{kind} {reference}.")
        } else {
            format!("{kind} {reference}, provided by {provenance}.")
        };
        Doc::new()
            .text(self.summary(from, c_name).unwrap_or_default())
            .text(origin)
            .text(self.legacy_type(from, c_name))
            .alias(c_name)
            .of_item(&type_name(c_name))
    }
}

fn item_parts(place: Place<'_>, item: &str, text: &str) -> Vec<Part> {
    items::read(text).unwrap_or_else(|unsupported| {
        panic!("the description of {item} on {place} cannot be read ({unsupported:?}): {text}")
    })
}

fn notes_of(place: Place<'_>, items: &[(&str, &str)]) -> BTreeMap<u32, Vec<Part>> {
    let mut notes = BTreeMap::new();
    if !items.iter().any(|(_, text)| items::may_have_notes(text)) {
        return notes;
    }
    let mut marks = BTreeSet::new();
    for (name, text) in items {
        let mut parts = item_parts(place, name, text);
        for (number, note) in items::take_notes(&mut parts) {
            let twice = notes.insert(number, note).is_some();
            assert!(!twice, "{place} has two notes {number}");
        }
        marks.extend(items::refer_to_notes(&mut parts));
    }
    let numbers = notes.keys().copied().collect::<BTreeSet<_>>();
    assert!(
        notes.is_empty() || marks == numbers,
        "the notes {numbers:?} and the marks {marks:?} on {place} do not pair up"
    );
    notes
}

const CLOSING_PUNCTUATION: [char; 7] = ['.', ',', ';', ':', '!', '?', ')'];

fn without_space_before_punctuation(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    for (number, line) in markdown.split('\n').enumerate() {
        let mark = line.split_whitespace().next().unwrap_or_default();
        let closes_up = !mark.is_empty() && mark.chars().all(|c| CLOSING_PUNCTUATION.contains(&c));
        if number > 0 && closes_up {
            out.truncate(out.trim_end().len());
            out.push_str(line.trim_start());
        } else {
            if number > 0 {
                out.push('\n');
            }
            out.push_str(line);
        }
    }
    out
}

fn value_named(row: &[Cell]) -> Option<&str> {
    row.first()
        .and_then(Cell::single_line)
        .and_then(|cell| cell.strip_prefix("ename:"))
}

fn without_emphasis(text: &str) -> &str {
    let text = text.trim();
    for marker in ["**", "__", "*", "_"] {
        if let Some(inner) = text
            .strip_prefix(marker)
            .and_then(|text| text.strip_suffix(marker))
            && !inner.trim().is_empty()
            && !inner.contains(marker)
        {
            return inner.trim();
        }
    }
    text
}

pub(crate) fn alias_doc(target: &str, alias_c_name: &str) -> Doc {
    Doc::new()
        .text(format!(
            "Alias of [`{target}`], the C name `{alias_c_name}`."
        ))
        .alias(alias_c_name)
        .of_item(&type_name(alias_c_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{registry, structure};
    use std::collections::BTreeMap;
    use vulkan_registry::{DocItem, DocItemOutsidePages, DocTable, Docs, Enum, EnumField};

    fn docs() -> Docs {
        Docs {
            commit: "000000000000".to_string(),
            anchors: BTreeMap::new(),
            attributes: [("times".to_string(), "×".to_string())].into(),
            titles: BTreeMap::new(),
            pages: vec![
                DocPage {
                    name: "VkExtent2D".to_string(),
                    kind: "structs".to_string(),
                    desc: "Structure specifying a two-dimensional extent".to_string(),
                    aliases: vec!["VkExtent2DKHR".to_string()],
                    file: "chapters/extents.adoc".to_string(),
                    items: vec![
                        DocItem {
                            name: "width".to_string(),
                            text: "pname:width is the width of the extent, see\nslink:VkExtent2D and elink:VkColor:\n** ename:VK_COLOR_RED for red\nand more\n** something else".to_string(),
                        },
                        DocItem {
                            name: "height".to_string(),
                            text: "pname:height is {unknown}.".to_string(),
                        },
                    ],
                },
                DocPage {
                    name: "VkShade".to_string(),
                    kind: "enums".to_string(),
                    desc: "Shades of a color".to_string(),
                    file: "chapters/shades.adoc".to_string(),
                    ..Default::default()
                },
                DocPage {
                    name: "VkTintFlagBits".to_string(),
                    kind: "enums".to_string(),
                    desc: "Tints of a color".to_string(),
                    file: "chapters/tints.adoc".to_string(),
                    ..Default::default()
                },
            ],
            tables: vec![
                table(
                    Some("VkShade"),
                    "chapters/shades.adoc",
                    &[
                        ".Shades and Their Extents",
                        "[options=\"header\",cols=\"2,1,1\"]",
                        "|====",
                        "|elink:VkShade |Extent [eq]#(w {times} h)# |*Mixes*",
                        "|ename:VK_SHADE_LIGHT |slink:VkExtent2D, not ename:VK_SHADE_LIGHT",
                        "|[eq]#a~0~# +",
                        "[eq]#b~0~#",
                        "|ename:VK_SHADE_DARK |{unknown} |",
                        "|ename:VK_SHADE_PALE |none |",
                        "|ename:VK_SHADE_GREY |grey m|code",
                        "|====",
                    ],
                ),
                table(
                    None,
                    "chapters/shades.adoc",
                    &[
                        ".Uses of Shades",
                        "[cols=\"1,1\",options=\"header\"]",
                        "|===",
                        "|Shade |Use",
                        "|ename:VK_SHADE_PALE |never",
                        "|ename:VK_SHADE_DEEP |always",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/shades.adoc",
                    &[
                        "[options=\"header\"]",
                        "|===",
                        "|Shade |Untitled",
                        "|ename:VK_SHADE_UNTITLED |somewhere",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/other.adoc",
                    &[
                        ".Shades Elsewhere",
                        "[options=\"header\"]",
                        "|===",
                        "|Shade |Use",
                        "|ename:VK_SHADE_ELSEWHERE |in another chapter",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/shades.adoc",
                    &[
                        ".Shades and Colors",
                        "[options=\"header\"]",
                        "|===",
                        "|Value |Vertex",
                        "|ename:VK_SHADE_MIXED |i",
                        "|ename:VK_COLOR_RED |i + 1",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/shades.adoc",
                    &[
                        ".Bare Shades",
                        "|===",
                        "|ename:VK_SHADE_BARE |without a header",
                        "|===",
                    ],
                ),
                table(
                    Some("VkExtent2D"),
                    "chapters/shades.adoc",
                    &[
                        ".Foreign Shades",
                        "[options=\"header\"]",
                        "|===",
                        "|Shade |Use",
                        "|ename:VK_SHADE_FOREIGN |elsewhere",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/tints.adoc",
                    &[
                        ".Uses of Tints",
                        "[options=\"header\"]",
                        "|===",
                        "|Tint |Use",
                        "|ename:VK_TINT_WARM_BIT |sunsets",
                        "|ename:VK_TINT_COOL_BIT |shadows",
                        "|===",
                    ],
                ),
                table(
                    None,
                    "chapters/shades.adoc",
                    &[
                        ".Shades and Tints",
                        "[options=\"header\"]",
                        "|===",
                        "|Value |Use",
                        "|ename:VK_SHADE_TINTED |sometimes",
                        "|ename:VK_TINT_WARM_BIT |often",
                        "|===",
                    ],
                ),
            ],
            items_outside_pages: Vec::new(),
        }
    }

    fn table(page: Option<&str>, file: &str, lines: &[&str]) -> DocTable {
        DocTable {
            file: file.to_string(),
            page: page.map(str::to_string),
            text: lines.join("\n"),
        }
    }

    fn ir() -> vulkan_registry::Registry {
        let mut ir = registry();
        ir.platforms.insert(
            "wayland".to_string(),
            "VK_USE_PLATFORM_WAYLAND_KHR".to_string(),
        );
        ir.structs.push(structure("VkExtent2D", vec![]));
        let mut gated = structure("VkGated", vec![]);
        gated.protect = Some("VK_USE_PLATFORM_WAYLAND_KHR".to_string());
        ir.structs.push(gated);
        ir.enums.push(Enum {
            name: "VkColor".to_string(),
            bit_width: 32,
            fields: vec![EnumField {
                name: "VK_COLOR_RED".to_string(),
                value: "0".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        });
        ir.enums.push(Enum {
            name: "VkShade".to_string(),
            bit_width: 32,
            fields: [
                "VK_SHADE_LIGHT",
                "VK_SHADE_DARK",
                "VK_SHADE_PALE",
                "VK_SHADE_GREY",
                "VK_SHADE_DEEP",
                "VK_SHADE_UNTITLED",
                "VK_SHADE_ELSEWHERE",
                "VK_SHADE_MIXED",
                "VK_SHADE_BARE",
                "VK_SHADE_FOREIGN",
                "VK_SHADE_TINTED",
            ]
            .iter()
            .enumerate()
            .map(|(value, name)| EnumField {
                name: name.to_string(),
                value: value.to_string(),
                ..Default::default()
            })
            .collect(),
            ..Default::default()
        });
        ir.bitmasks.push(vulkan_registry::Bitmask {
            name: "VkTintFlagBits".to_string(),
            flags_name: "VkTintFlags".to_string(),
            bit_width: 32,
            fields: vec![
                EnumField {
                    name: "VK_TINT_WARM_BIT".to_string(),
                    value: "1".to_string(),
                    ..Default::default()
                },
                EnumField {
                    name: "VK_TINT_COLD_BIT".to_string(),
                    aliases: vec!["VK_TINT_COOL_BIT".to_string()],
                    value: "2".to_string(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        ir
    }

    #[test]
    fn a_table_outside_reference_pages_describes_the_values_of_one_bitmask_and_their_aliases() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        for (value, usage) in [
            ("VK_TINT_WARM_BIT", "sunsets"),
            ("VK_TINT_COOL_BIT", "shadows"),
        ] {
            assert_eq!(
                model.tabulated(&Cfg::default(), "VkTintFlagBits", value),
                Some(vec![
                    Block::Text(format!(
                        "`{value}` in the table “Uses of Tints” of the specification:"
                    )),
                    Block::list([format!("Use: {usage}")]),
                ])
            );
        }
        assert_eq!(
            model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_TINTED"),
            None,
            "a table that also has rows for the values of a bitmask does not describe the value"
        );
        assert_eq!(
            model.tabulated(&Cfg::default(), "VkTintFlagBits", "VK_TINT_COLD_BIT"),
            None,
            "the table has no row for the value"
        );
    }

    #[test]
    fn pages_are_found_by_name_page_alias_and_registry_alias() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.page("VkExtent2D").map(|p| p.name.as_str()),
            Some("VkExtent2D")
        );
        assert_eq!(
            model.page("VkExtent2DKHR").map(|p| p.name.as_str()),
            Some("VkExtent2D")
        );
        assert!(model.page("VkColor").is_none());
        assert_eq!(
            model.reference("VkExtent2DKHR"),
            "[`VkExtent2DKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent2D.html)"
        );
        assert_eq!(model.reference("VkColor"), "`VkColor`");
    }

    #[test]
    fn the_summary_is_the_description_of_the_page_as_a_sentence() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.summary(&Cfg::default(), "VkExtent2D").as_deref(),
            Some("Structure specifying a two-dimensional extent.")
        );
        assert_eq!(model.summary(&Cfg::default(), "VkColor"), None);
    }

    #[test]
    fn described_items_link_other_items_but_not_themselves() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![
                Block::Text(
                    "`width` is the width of the extent, see\n[`VkExtent2D`](Extent2D) and [`VkColor`](Color):"
                        .to_string()
                ),
                Block::list([
                    "[`VK_COLOR_RED`](Color::RED) for red\nand more".to_string(),
                    "something else".to_string(),
                ]),
            ])
        );
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "depth"),
            None
        );
    }

    fn with_items_elsewhere(
        on_the_page_of_the_tints: &[(&str, &str)],
        outside_pages: &[(&str, &str)],
    ) -> Docs {
        let mut docs = docs();
        let tints = docs
            .pages
            .iter_mut()
            .find(|page| page.name == "VkTintFlagBits")
            .expect("the page of the tints");
        tints
            .items
            .extend(on_the_page_of_the_tints.iter().map(|(name, text)| DocItem {
                name: name.to_string(),
                text: text.to_string(),
            }));
        docs.items_outside_pages = outside_pages
            .iter()
            .map(|(name, text)| DocItemOutsidePages {
                file: "chapters/extents.adoc".to_string(),
                name: name.to_string(),
                text: text.to_string(),
            })
            .collect();
        docs
    }

    #[test]
    fn a_value_that_its_page_does_not_describe_takes_the_one_item_that_describes_it_elsewhere() {
        let ir = ir();
        let docs = with_items_elsewhere(
            &[
                (
                    "VK_SHADE_DEEP",
                    "ename:VK_SHADE_DEEP specifies the deepest\nelink:VkShade, deeper than ename:VK_SHADE_DARK.",
                ),
                ("VK_SHADE_DARK", "ename:VK_SHADE_DARK is set."),
            ],
            &[
                (
                    "VK_SHADE_PALE",
                    "ename:VK_SHADE_PALE\nspecifies a pale shade:\n** paler than ename:VK_SHADE_PALE",
                ),
                ("VK_COLOR_RED", "ename:VK_COLOR_RED specifies red."),
                ("VK_SHADE_DARK", "ename:VK_SHADE_DARK"),
                (
                    "VK_SHADE_GREY",
                    "ename:VK_SHADE_GREY and ename:VK_SHADE_DARK",
                ),
            ],
        );
        let model = Model::with_docs(&ir, &docs);
        let text = |text: &str| Block::Text(text.to_string());
        let described = |owner: &str, value: &str| model.described(&Cfg::default(), owner, value);
        assert_eq!(
            described("VkShade", "VK_SHADE_DEEP"),
            Some(vec![text(
                "`VK_SHADE_DEEP` specifies the deepest\n[`VkShade`](Shade), deeper than [`VK_SHADE_DARK`](Shade::DARK)."
            )]),
            "an item on the page of another type"
        );
        assert_eq!(
            described("VkShade", "VK_SHADE_PALE"),
            Some(vec![
                text("`VK_SHADE_PALE`\nspecifies a pale shade:"),
                Block::list(["paler than `VK_SHADE_PALE`".to_string()]),
            ]),
            "an item outside the reference pages"
        );
        assert_eq!(
            described("VkColor", "VK_COLOR_RED"),
            None,
            "a value of a type without a reference page"
        );
        for mentioned in ["VK_SHADE_DARK", "VK_SHADE_GREY", "VK_SHADE_LIGHT"] {
            assert_eq!(
                described("VkShade", mentioned),
                None,
                "{mentioned} is named in lists, which does not describe it"
            );
        }
    }

    #[test]
    fn an_item_on_the_page_of_the_type_is_preferred_to_the_items_that_describe_a_value_elsewhere() {
        let ir = ir();
        let mut docs = with_items_elsewhere(
            &[("VK_SHADE_LIGHT", "ename:VK_SHADE_LIGHT specifies a tint.")],
            &[
                (
                    "VK_SHADE_LIGHT",
                    "ename:VK_SHADE_LIGHT specifies an extent.",
                ),
                ("VK_SHADE_LIGHT", "ename:VK_SHADE_LIGHT specifies it again."),
            ],
        );
        let shades = docs
            .pages
            .iter_mut()
            .find(|page| page.name == "VkShade")
            .expect("the page of the shades");
        shades.items.push(DocItem {
            name: "VK_SHADE_LIGHT".to_string(),
            text: "ename:VK_SHADE_LIGHT is the lightest shade.".to_string(),
        });
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.described(&Cfg::default(), "VkShade", "VK_SHADE_LIGHT"),
            Some(vec![Block::Text(
                "`VK_SHADE_LIGHT` is the lightest shade.".to_string()
            )])
        );
    }

    #[test]
    fn several_items_that_describe_a_value_outside_its_page_abort_generation() {
        let ir = ir();
        let docs = with_items_elsewhere(
            &[("VK_SHADE_DEEP", "ename:VK_SHADE_DEEP specifies a tint.")],
            &[("VK_SHADE_DEEP", "ename:VK_SHADE_DEEP specifies an extent.")],
        );
        let model = Model::with_docs(&ir, &docs);
        let panic = crate::test_support::catch_quietly(|| {
            model.described(&Cfg::default(), "VkShade", "VK_SHADE_DEEP")
        })
        .expect_err("one of the two items is taken");
        assert_eq!(
            panic,
            "the reference page of its type does not describe VK_SHADE_DEEP, and 2 list items \
             elsewhere do, so that nothing says which one to take: the page VkTintFlagBits, the \
             lines of chapters/extents.adoc outside the reference pages"
        );
    }

    #[test]
    fn an_item_outside_the_pages_has_its_own_notes_and_an_item_of_another_page_the_notes_of_that_page()
     {
        let ir = ir();
        let docs = with_items_elsewhere(
            &[
                (
                    "VK_SHADE_DARK",
                    "ename:VK_SHADE_DARK specifies a dark^2^ shade.",
                ),
                (
                    "VK_TINT_WARM_BIT",
                    "ename:VK_TINT_WARM_BIT specifies warmth.\n\n2:: Like the night.",
                ),
            ],
            &[
                (
                    "VK_SHADE_PALE",
                    "ename:VK_SHADE_PALE specifies a pale^1^ shade.\n\n1:: Not white.",
                ),
                (
                    "VK_SHADE_DEEP",
                    "ename:VK_SHADE_DEEP specifies a deep^1^ shade.\n\n1:: Not black.",
                ),
            ],
        );
        let model = Model::with_docs(&ir, &docs);
        let text = |text: &str| Block::Text(text.to_string());
        for (value, described, note) in [
            (
                "VK_SHADE_DARK",
                "a dark (note 2) shade.",
                ("2", "Like the night."),
            ),
            (
                "VK_SHADE_PALE",
                "a pale (note 1) shade.",
                ("1", "Not white."),
            ),
            (
                "VK_SHADE_DEEP",
                "a deep (note 1) shade.",
                ("1", "Not black."),
            ),
        ] {
            assert_eq!(
                model.described(&Cfg::default(), "VkShade", value),
                Some(vec![
                    text(&format!("`{value}` specifies {described}")),
                    Block::Quote(vec![text(&format!("**Note {}**", note.0)), text(note.1)]),
                ])
            );
        }
    }

    #[test]
    #[should_panic(
        expected = "the description of VK_SHADE_PALE on the lines of chapters/extents.adoc outside the reference pages has no Markdown form (Attribute(\"unknown\")): ename:VK_SHADE_PALE specifies {unknown}."
    )]
    fn an_item_outside_the_pages_that_cannot_be_converted_aborts_generation() {
        let ir = ir();
        let docs = with_items_elsewhere(
            &[],
            &[("VK_SHADE_PALE", "ename:VK_SHADE_PALE specifies {unknown}.")],
        );
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkShade", "VK_SHADE_PALE");
    }

    fn with_the_width_described_as(lines: &[&str]) -> Docs {
        let mut docs = docs();
        docs.pages[0].items[0].text = lines.join("\n");
        docs
    }

    #[test]
    fn everything_that_belongs_to_a_described_item_is_kept() {
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is clamped to the range",
            "[eq]#[0,{times}]#.",
            "+",
            "It has requirements:",
            "+",
            "** The first, for slink:VkExtent2D.",
            "+",
            "[NOTE]",
            ".Why",
            "====",
            "Because pname:width is *wide*.",
            "====",
            "",
            "+",
            "[latexmath]",
            "++++",
            "w \\geq 0",
            "++++",
            "+",
            "[source,c++]",
            "----",
            "width = 0;",
            "----",
        ]);
        let model = Model::with_docs(&ir, &docs);
        let text = |text: &str| Block::Text(text.to_string());
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![
                text("`width` is clamped to the range\n\\[0,×\\]."),
                text("It has requirements:"),
                Block::List(vec![vec![
                    text("The first, for [`VkExtent2D`](Extent2D)."),
                    Block::Quote(vec![
                        text("**Note: Why**"),
                        text("Because `width` is **wide**."),
                    ]),
                ]]),
                text("`w \\geq 0`"),
                Block::Code {
                    language: "c++".to_string(),
                    text: "width = 0;".to_string(),
                },
            ])
        );
    }

    #[test]
    #[should_panic(
        expected = "the description of width on the page VkExtent2D cannot be read (Include): pname:width is equivalent to the logical OR of:"
    )]
    fn an_include_that_is_left_in_a_description_aborts_generation() {
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is equivalent to the logical OR of:",
            "include::{generated}/sync/flagDefinitions/width.adoc[]",
        ]);
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkExtent2D", "width");
    }

    fn with_the_members_described_as(width: &[&str], height: &[&str]) -> Docs {
        let mut docs = docs();
        docs.pages[0].items[0].text = width.join("\n");
        docs.pages[0].items[1].text = height.join("\n");
        docs
    }

    #[test]
    fn a_numbered_note_is_attached_to_the_items_that_carry_its_mark() {
        let ir = ir();
        let docs = with_the_members_described_as(
            &[
                "pname:width is a bitmask^2^ of 2^pname:width^ values^1^ or 2^1^.",
                "** It is wide^2^.",
            ],
            &[
                "pname:height is the height.",
                "",
                "1::",
                "For all values of slink:VkExtent2D,",
                "the first note applies.",
                "",
                "2:: The second note applies.",
                "** In detail.",
            ],
        );
        let model = Model::with_docs(&ir, &docs);
        let text = |text: &str| Block::Text(text.to_string());
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![
                text(
                    "`width` is a bitmask (note 2) of 2<sup>`width`</sup> values (note 1) or 2<sup>1</sup>."
                ),
                Block::list(["It is wide (note 2).".to_string()]),
                Block::Quote(vec![
                    text("**Note 1**"),
                    text("For all values of [`VkExtent2D`](Extent2D),\nthe first note applies."),
                ]),
                Block::Quote(vec![
                    text("**Note 2**"),
                    text("The second note applies."),
                    Block::list(["In detail.".to_string()]),
                ]),
            ])
        );
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "height"),
            Some(vec![text("`height` is the height.")]),
            "a note is not a part of the item that it is nested in"
        );
    }

    #[test]
    fn a_superscript_is_no_mark_on_a_page_without_notes() {
        let ir = ir();
        let docs = with_the_members_described_as(
            &["pname:width is at most 2^32^-1."],
            &["pname:height is at most 2^16^."],
        );
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![Block::Text(
                "`width` is at most 2<sup>32</sup>-1.".to_string()
            )])
        );
    }

    #[test]
    fn notes_and_marks_that_do_not_pair_up_abort_generation() {
        let ir = ir();
        for (width, height, message) in [
            (
                "pname:width is a bitmask^1^ of 2^1^ values^32^.",
                "pname:height is the height.\n\n1:: The note.",
                "the notes {1} and the marks {1, 32} on the page VkExtent2D do not pair up",
            ),
            (
                "pname:width is a bitmask^1^.",
                "pname:height is the height.\n\n1:: The note.\n\n2:: Another note.",
                "the notes {1, 2} and the marks {1} on the page VkExtent2D do not pair up",
            ),
            (
                "pname:width is a bitmask.",
                "pname:height is the height.\n\n1:: The note.",
                "the notes {1} and the marks {} on the page VkExtent2D do not pair up",
            ),
            (
                "pname:width is a bitmask^1^.\n\n1:: A note.",
                "pname:height is the height.\n\n1:: The note.",
                "the page VkExtent2D has two notes 1",
            ),
        ] {
            let docs = with_the_members_described_as(&[width], &[height]);
            let model = Model::with_docs(&ir, &docs);
            for member in ["width", "height"] {
                let panic = crate::test_support::catch_quietly(|| {
                    model.described(&Cfg::default(), "VkExtent2D", member)
                })
                .expect_err(&format!("{member} is described: {width} / {height}"));
                assert!(panic.contains(message), "{panic}");
            }
        }
    }

    #[test]
    fn a_description_list_becomes_a_list_of_its_terms_with_what_describes_them() {
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is the width.",
            "",
            "1::",
            "For all widths of slink:VkExtent2D,",
            "the note applies.",
            "",
            "pname:width::  Itself.",
            "** In detail.",
        ]);
        let model = Model::with_docs(&ir, &docs);
        let text = |text: &str| Block::Text(text.to_string());
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![
                text("`width` is the width."),
                Block::List(vec![
                    vec![
                        text("1"),
                        text("For all widths of [`VkExtent2D`](Extent2D),\nthe note applies."),
                    ],
                    vec![
                        text("`width`"),
                        text("Itself."),
                        Block::list(["In detail.".to_string()]),
                    ],
                ]),
            ])
        );
    }

    #[test]
    fn a_line_that_starts_with_closing_punctuation_closes_up_to_the_line_before_it() {
        for (markdown, closed_up) in [
            ("a copy\n.\nIf more", "a copy.\nIf more"),
            ("is either\n:", "is either:"),
            ("the command\n, or the other", "the command, or the other"),
            ("(see the list \n  ) and", "(see the list) and"),
            ("a\n?! b\n;\nc", "a?! b;\nc"),
            ("a\n... and more", "a... and more"),
            (".\na", ".\na"),
            ("  , a\n. b", "  , a. b"),
            ("a\n.b\n,c\n...d\n-\n(\n\"", "a\n.b\n,c\n...d\n-\n(\n\""),
            ("a\n\n.", "a."),
            ("", ""),
        ] {
            assert_eq!(
                without_space_before_punctuation(markdown),
                closed_up,
                "{markdown:?}"
            );
        }
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is the width",
            ".",
            "If so, it is required",
            ":",
            "** slink:VkExtent2D",
            ", or not",
        ]);
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            Some(vec![
                Block::Text("`width` is the width.\nIf so, it is required:".to_string()),
                Block::list(["[`VkExtent2D`](Extent2D), or not".to_string()]),
            ])
        );
    }

    #[test]
    #[should_panic(
        expected = "the description of width on the page VkExtent2D cannot be read (Block(\"a `|===` block with the attributes []\")): pname:width is in the table."
    )]
    fn a_block_that_the_generator_does_not_read_aborts_generation() {
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is in the table.",
            "+",
            "|===",
            "| a | b",
            "|===",
        ]);
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkExtent2D", "width");
    }

    #[test]
    #[should_panic(
        expected = "the description of height on the page VkExtent2D has no Markdown form (Attribute(\"unknown\"))"
    )]
    fn text_that_cannot_be_converted_aborts_generation() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkExtent2D", "height");
    }

    #[test]
    #[should_panic(
        expected = "the description of height on the page VkExtent2D has no Markdown form (UnknownMacro(\"kbd\"))"
    )]
    fn an_unknown_macro_in_a_description_aborts_generation() {
        let (ir, mut docs) = (ir(), docs());
        docs.pages[0].items[1].text = "pname:height is kbd:[Ctrl].".to_string();
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkExtent2D", "height");
    }

    #[test]
    #[should_panic(expected = "a table of VkShade uses the AsciiDoc macro `kbd:`")]
    fn an_unknown_macro_in_a_table_aborts_generation() {
        let (ir, mut docs) = (ir(), docs());
        docs.tables[0].text = docs.tables[0].text.replace(
            "|ename:VK_SHADE_PALE |none |",
            "|ename:VK_SHADE_PALE |kbd:[none] |",
        );
        let model = Model::with_docs(&ir, &docs);
        model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_PALE");
    }

    #[test]
    fn unread_markup_in_a_table_aborts_generation() {
        for (cell, unread) in [
            ("\\#none#", "EscapedSpan"),
            ("+none +x+", "Passthrough"),
            ("must:be none", "MacroUse(\"must:be\")"),
            ("xpname:none", "MacroUse(\"pname:none\")"),
        ] {
            let (ir, mut docs) = (ir(), docs());
            docs.tables[0].text = docs.tables[0].text.replace(
                "|ename:VK_SHADE_PALE |none |",
                &format!("|ename:VK_SHADE_PALE |{cell} |"),
            );
            let model = Model::with_docs(&ir, &docs);
            let message = crate::test_support::catch_quietly(|| {
                model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_PALE")
            })
            .expect_err(&format!("`{cell}` in a table cell is accepted"));
            assert!(
                message.contains(&format!(
                    "a table of VkShade has markup that the generator does not read ({unread})"
                )),
                "{message}"
            );
        }
    }

    #[test]
    fn a_value_named_in_a_table_row_is_described_by_the_other_cells_under_their_headers() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_LIGHT"),
            Some(vec![
                Block::Text(
                    "`VK_SHADE_LIGHT` in the table “Shades and Their Extents” of the specification:"
                        .to_string()
                ),
                Block::List(vec![
                    vec![Block::Text(
                        "Extent (w × h): [`VkExtent2D`](Extent2D), not `VK_SHADE_LIGHT`".to_string()
                    )],
                    vec![
                        Block::Text("Mixes:".to_string()),
                        Block::list(["a<sub>0</sub>".to_string(), "b<sub>0</sub>".to_string()]),
                    ],
                ]),
            ])
        );
        assert_eq!(
            model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_PALE"),
            Some(vec![
                Block::Text(
                    "`VK_SHADE_PALE` in the table “Shades and Their Extents” of the specification:"
                        .to_string()
                ),
                Block::list(["Extent (w × h): none".to_string()]),
            ]),
            "a table on the page of the type is preferred and an empty cell is left out"
        );
    }

    #[test]
    fn tables_outside_reference_pages_describe_values_that_the_page_has_no_row_for() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.tabulated(&Cfg::default(), "VkShade", "VK_SHADE_DEEP"),
            Some(vec![
                Block::Text(
                    "`VK_SHADE_DEEP` in the table “Uses of Shades” of the specification:"
                        .to_string()
                ),
                Block::list(["Use: always".to_string()]),
            ])
        );
        for (value, decoy) in [
            ("VK_SHADE_FOREIGN", "a table on the page of another item"),
            ("VK_SHADE_UNTITLED", "an untitled table outside any page"),
            (
                "VK_SHADE_ELSEWHERE",
                "a table in another file than the page of the type",
            ),
            (
                "VK_SHADE_MIXED",
                "a table whose rows are not all values of the type",
            ),
        ] {
            assert_eq!(
                model.tabulated(&Cfg::default(), "VkShade", value),
                None,
                "{decoy} does not describe the value"
            );
        }
    }

    #[test]
    fn header_labels_lose_their_emphasis() {
        for (label, plain) in [
            ("*Command Data*", "Command Data"),
            ("**Data**", "Data"),
            ("_Data_", "Data"),
            ("*a* and *b*", "*a* and *b*"),
            ("Data", "Data"),
        ] {
            assert_eq!(without_emphasis(label), plain, "{label}");
        }
    }

    #[test]
    fn rows_that_cannot_be_understood_leave_the_value_undescribed() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        for value in [
            "VK_SHADE_DARK",
            "VK_SHADE_GREY",
            "VK_SHADE_BARE",
            "VK_SHADE_MISSING",
        ] {
            assert_eq!(
                model.tabulated(&Cfg::default(), "VkShade", value),
                None,
                "{value}"
            );
        }
    }

    #[test]
    fn items_behind_a_feature_are_linked_only_from_items_behind_the_same_feature() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        let wayland = model.cfg(Some("VK_USE_PLATFORM_WAYLAND_KHR"));
        assert_eq!(model.link(&Cfg::default(), "VkGated"), "`VkGated`");
        assert_eq!(model.link(&wayland, "VkGated"), "[`VkGated`](Gated)");
        assert_eq!(
            model.link(&wayland, "VkExtent2D"),
            "[`VkExtent2D`](Extent2D)"
        );
        assert_eq!(model.link(&wayland, "VkNothing"), "`VkNothing`");
    }

    #[test]
    fn provenance_names_versions_extensions_and_video_headers() {
        let mut ir = ir();
        ir.extensions.push(vulkan_registry::Extension {
            name: "VK_KHR_surface".to_string(),
            name_macro: "VK_KHR_SURFACE_EXTENSION_NAME".to_string(),
            ..Default::default()
        });
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        let from = Cfg::default();
        assert_eq!(model.provenance(&from, None, &[], None), "Vulkan 1.0");
        assert_eq!(
            model.provenance(
                &from,
                Some("VK_VERSION_1_1"),
                &["VK_KHR_surface".to_string(), "VK_KHR_other".to_string()],
                None
            ),
            "Vulkan 1.1, [`VK_KHR_surface`](KHR_SURFACE_NAME) or `VK_KHR_other`"
        );
        assert_eq!(
            model.provenance(&from, None, &[], Some("vulkan_video_codec_h264std")),
            "the Vulkan Video Std header `vulkan_video_codec_h264std`"
        );
    }

    #[test]
    fn type_documentation_has_the_summary_the_reference_and_a_search_alias() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model
                .type_doc(&Cfg::default(), "VkExtent2D", "C structure", "Vulkan 1.0")
                .render(),
            "/// Structure specifying a two-dimensional extent.\n///\n/// C structure\n/// [`VkExtent2D`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExtent2D.html),\n/// provided by Vulkan 1.0.\n#[doc(alias = \"VkExtent2D\")]\n"
        );
        assert_eq!(
            model
                .type_doc(&Cfg::default(), "VkColor", "C enumeration", "")
                .render(),
            "/// C enumeration `VkColor`.\n#[doc(alias = \"VkColor\")]\n"
        );
    }

    #[test]
    fn lists_join_with_commas_and_a_conjunction() {
        let items = |n: usize| (1..=n).map(|i| i.to_string()).collect::<Vec<_>>();
        assert_eq!(join_or(items(0)), "");
        assert_eq!(join_or(items(1)), "1");
        assert_eq!(join_or(items(2)), "1 or 2");
        assert_eq!(join_and(items(3)), "1, 2 and 3");
        assert_eq!(sentence("Text"), "Text.");
        assert_eq!(sentence("Text. "), "Text.");
    }
}
