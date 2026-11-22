pub(crate) mod asciidoc;
mod links;
mod render;

pub(crate) use links::Links;
pub(crate) use render::{Block, Doc};

use crate::model::{Cfg, Model, parse_version};
use crate::naming::type_name;
use asciidoc::{Converter, Link};
use vulkan_registry::DocPage;

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

fn nested_item(line: &str) -> Option<(usize, &str)> {
    let stars = line.chars().take_while(|c| *c == '*').count();
    let rest = &line[stars..];
    (stars >= 2 && rest.starts_with(' ')).then(|| (stars - 2, rest.trim_start()))
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

    fn markdown(&self, from: &Cfg, own: &str, text: &str) -> String {
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
        .unwrap_or_else(|unsupported| {
            panic!("the description of {own} has no Markdown form ({unsupported:?}): {text}")
        })
    }

    pub(crate) fn summary(&self, from: &Cfg, c_name: &str) -> Option<String> {
        let page = self.page(c_name)?;
        let summary = self.markdown(from, c_name, &page.desc);
        (!summary.trim().is_empty()).then(|| sentence(&summary))
    }

    pub(crate) fn described(&self, from: &Cfg, owner: &str, item: &str) -> Option<Vec<Block>> {
        let text = &self
            .page(owner)?
            .items
            .iter()
            .find(|described| described.name == item)?
            .text;
        let mut lead = String::new();
        let mut nested = Vec::<(usize, String)>::new();
        for line in text.lines() {
            if let Some((level, rest)) = nested_item(line) {
                nested.push((level, rest.to_string()));
            } else if let Some((_, last)) = nested.last_mut() {
                last.push('\n');
                last.push_str(line);
            } else {
                lead.push_str(line);
                lead.push('\n');
            }
        }
        let mut blocks = vec![Block::Text(self.markdown(from, item, &lead))];
        if !nested.is_empty() {
            let items = nested
                .into_iter()
                .map(|(level, text)| (level, self.markdown(from, item, &text)))
                .collect();
            blocks.push(Block::List(items));
        }
        Some(blocks)
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
    use vulkan_registry::{DocItem, Docs, Enum, EnumField};

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
            ],
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
                    "`width` is the width of the extent, see\n[`VkExtent2D`](Extent2D) and [`VkColor`](Color):\n"
                        .to_string()
                ),
                Block::List(vec![
                    (0, "[`VK_COLOR_RED`](Color::RED) for red\nand more".to_string()),
                    (0, "something else".to_string()),
                ]),
            ])
        );
        assert_eq!(
            model.described(&Cfg::default(), "VkExtent2D", "depth"),
            None
        );
    }

    #[test]
    #[should_panic(
        expected = "the description of height has no Markdown form (Attribute(\"unknown\"))"
    )]
    fn text_that_cannot_be_converted_aborts_generation() {
        let (ir, docs) = (ir(), docs());
        let model = Model::with_docs(&ir, &docs);
        model.described(&Cfg::default(), "VkExtent2D", "height");
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
