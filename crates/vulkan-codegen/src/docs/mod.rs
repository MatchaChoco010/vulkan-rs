pub(crate) mod asciidoc;
mod items;
mod links;
mod other_names;
mod render;
mod tables;

pub(crate) use links::Links;
pub(crate) use other_names::assert_every_other_name_is_in_use;
pub(crate) use render::{Block, Doc};
pub(crate) use tables::{Table, read as read_table};

use crate::model::{Cfg, Model, parse_version};
use crate::naming::type_name;
use asciidoc::{Converted, Converter, Link, Unsupported, code_span};
use items::Part;
use std::collections::BTreeSet;
use tables::Cell;
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

    fn markdown(&self, from: &Cfg, page: &str, own: &str, text: &str) -> String {
        self.try_markdown(from, own, text)
            .unwrap_or_else(|unsupported| {
                panic!(
                    "the description of {own} on the page {page} has no Markdown form \
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
            Err(unread @ (Unsupported::EscapedSpan | Unsupported::Passthrough)) => panic!(
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
        let summary = self.markdown(from, &page.name, c_name, &page.desc);
        (!summary.trim().is_empty()).then(|| sentence(&summary))
    }

    pub(crate) fn described(&self, from: &Cfg, owner: &str, item: &str) -> Option<Vec<Block>> {
        let page = self.page(owner)?;
        let described = other_names::described(page, item)?;
        let text = &described.item.text;
        let parts = match items::read(text) {
            Ok(parts) => parts,
            Err(Unsupported::Include) => return None,
            Err(unsupported) => panic!(
                "the description of {item} on the page {} cannot be read ({unsupported:?}): {text}",
                page.name
            ),
        };
        let mut blocks = self.blocks(from, &page.name, item, parts);
        if let (Some(other), Some(Block::Text(lead))) =
            (described.under_the_name, blocks.first_mut())
        {
            *lead = format!(
                "`{item}`, which the specification describes under the name `{other}`: {lead}"
            );
        }
        Some(blocks)
    }

    fn blocks(&self, from: &Cfg, page: &str, own: &str, parts: Vec<Part>) -> Vec<Block> {
        let mut blocks = Vec::new();
        for part in parts {
            blocks.push(match part {
                Part::Paragraph(text) => Block::Text(self.markdown(from, page, own, &text)),
                Part::List(items) => Block::List(
                    items
                        .into_iter()
                        .map(|item| self.blocks(from, page, own, item))
                        .collect(),
                ),
                Part::Admonition {
                    label,
                    title,
                    parts,
                } => {
                    let caption = match title {
                        Some(title) => {
                            format!("**{label}: {}**", self.markdown(from, page, own, &title))
                        }
                        None => format!("**{label}**"),
                    };
                    let mut quoted = vec![Block::Text(caption)];
                    quoted.extend(self.blocks(from, page, own, parts));
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
            .alias(c_name)
            .of_item(&type_name(c_name))
    }
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
    use vulkan_registry::{DocItem, DocTable, Docs, Enum, EnumField};

    fn docs() -> Docs {
        Docs {
            commit: "000000000000".to_string(),
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
            ],
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
        ir
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
    fn an_item_that_goes_on_in_a_file_outside_the_sources_is_not_described() {
        let ir = ir();
        let docs = with_the_width_described_as(&[
            "pname:width is equivalent to the logical OR of:",
            "include::{generated}/sync/flagDefinitions/width.adoc[]",
        ]);
        let model = Model::with_docs(&ir, &docs);
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "width"),
            None
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
        for (cell, unread) in [("\\#none#", "EscapedSpan"), ("+none +x+", "Passthrough")] {
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
