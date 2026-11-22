use crate::model::Model;
use vulkan_registry::{DocItem, DocPage};

const DESCRIBED_UNDER_ANOTHER_NAME: &[(&str, &str, &str)] = &[
    ("vkCmdBindVertexBuffers3KHR", "pBindingInfos", "pInfos"),
    (
        "vkCmdEndTransformFeedback2EXT",
        "firstCounterRange",
        "firstCounterBuffer",
    ),
    ("vkGetDeviceFaultReportsKHR", "pFaultCounts", "pFaultCount"),
    ("vkGetDrmDisplayEXT", "display", "pDisplay"),
    (
        "vkSetLatencyMarkerNV",
        "pLatencyMarkerInfo",
        "pSetLatencyMarkerInfo",
    ),
];

pub(super) struct Described<'a> {
    pub(super) item: &'a DocItem,
    pub(super) under_the_name: Option<&'static str>,
}

fn after_a_separator(text: &str) -> Option<&str> {
    let (listed, rest) = match text.strip_prefix(',') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    let next = rest.trim_start();
    if next.len() == rest.len() {
        return None;
    }
    match next.strip_prefix("and") {
        Some(joined) if joined.starts_with(char::is_whitespace) => Some(joined.trim_start()),
        _ if listed => Some(next),
        _ => None,
    }
}

fn names_at_the_start(text: &str) -> Vec<&str> {
    let mut names = Vec::new();
    let Some(kind) = ["pname:", "ename:"]
        .into_iter()
        .find(|kind| text.starts_with(kind))
    else {
        return names;
    };
    let mut next = Some(text);
    while let Some(name) = next.and_then(|text| text.strip_prefix(kind)) {
        let len = name
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(name.len());
        if len == 0 {
            break;
        }
        names.push(&name[..len]);
        next = after_a_separator(&name[len..]);
    }
    names
}

fn names(item: &DocItem, name: &str) -> bool {
    item.name == name || names_at_the_start(&item.text).contains(&name)
}

pub(super) fn described<'a>(page: &'a DocPage, name: &str) -> Option<Described<'a>> {
    let item = |name: &str| page.items.iter().find(|item| item.name == name);
    let item_with_other_names = || page.items.iter().find(|item| names(item, name));
    if let Some(item) = item(name).or_else(item_with_other_names) {
        return Some(Described {
            item,
            under_the_name: None,
        });
    }
    let (_, _, other) = DESCRIBED_UNDER_ANOTHER_NAME
        .iter()
        .find(|(command, parameter, _)| *command == page.name && *parameter == name)?;
    Some(Described {
        item: item(other)?,
        under_the_name: Some(other),
    })
}

pub(crate) fn assert_every_other_name_is_in_use(model: &Model<'_>) {
    for (command, parameter, other) in DESCRIBED_UNDER_ANOTHER_NAME {
        let row =
            format!("the row ({command}, {parameter}, {other}) of DESCRIBED_UNDER_ANOTHER_NAME");
        let c = model
            .ir
            .commands
            .iter()
            .find(|c| c.name == *command)
            .unwrap_or_else(|| panic!("{row} names a command that the registry does not have"));
        let is_parameter = |name: &str| c.params.iter().any(|p| p.name == name);
        assert!(
            is_parameter(parameter),
            "{row} names a parameter that the command does not have"
        );
        assert!(
            !is_parameter(other),
            "{row} gives a parameter the description of another parameter of the command"
        );
        let page = model
            .pages
            .get(command)
            .unwrap_or_else(|| panic!("{row} names a command without a reference page"));
        let describes = |name: &str| page.items.iter().any(|item| names(item, name));
        assert!(
            !describes(parameter),
            "{row} is not needed any more: the specification describes {parameter} under its own name"
        );
        assert!(
            describes(other),
            "{row} names a description that the specification does not have"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs::Block;
    use crate::model::Cfg;
    use crate::test_support::{catch_quietly, registry};
    use std::collections::BTreeSet;
    use vulkan_registry::{Docs, Registry};

    fn page(name: &str, items: &[(&str, &str)]) -> DocPage {
        DocPage {
            name: name.to_string(),
            kind: "protos".to_string(),
            items: items
                .iter()
                .map(|(name, text)| DocItem {
                    name: name.to_string(),
                    text: text.to_string(),
                })
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn a_parameter_that_the_specification_names_differently_gets_that_description() {
        let ir = registry();
        let docs = Docs {
            pages: vec![
                page(
                    "vkGetDrmDisplayEXT",
                    &[
                        ("drmFd", "pname:drmFd is the DRM primary file descriptor."),
                        (
                            "pDisplay",
                            "pname:pDisplay is a pointer to a handle where the display\nis returned:\n** on success\n** always",
                        ),
                    ],
                ),
                page(
                    "vkGetOtherDisplayEXT",
                    &[("pDisplay", "pname:pDisplay is a pointer to a handle.")],
                ),
            ],
            ..Default::default()
        };
        let model = Model::with_docs(&ir, &docs);
        let from = Cfg::default();
        let nested = Block::list(["on success".to_string(), "always".to_string()]);
        assert_eq!(
            model.described(&from, "vkGetDrmDisplayEXT", "display"),
            Some(vec![
                Block::Text(
                    "`display`, which the specification describes under the name `pDisplay`: `pDisplay` is a pointer to a handle where the display\nis returned:"
                        .to_string()
                ),
                nested.clone(),
            ])
        );
        assert_eq!(
            model.described(&from, "vkGetDrmDisplayEXT", "drmFd"),
            Some(vec![Block::Text(
                "`drmFd` is the DRM primary file descriptor.".to_string()
            )])
        );
        assert_eq!(
            model.described(&from, "vkGetDrmDisplayEXT", "pDisplay"),
            Some(vec![
                Block::Text(
                    "`pDisplay` is a pointer to a handle where the display\nis returned:"
                        .to_string()
                ),
                nested,
            ]),
            "the description is still found under its own name"
        );
        assert_eq!(
            model.described(&from, "vkGetOtherDisplayEXT", "display"),
            None,
            "a row names one command"
        );
        assert_eq!(
            model.described(&from, "vkGetDrmDisplayEXT", "connectorId"),
            None
        );
    }

    #[test]
    fn a_description_under_the_own_name_of_the_parameter_is_preferred() {
        let page = page(
            "vkGetDrmDisplayEXT",
            &[
                ("pDisplay", "pname:pDisplay is the old text."),
                ("display", "pname:display is the display."),
            ],
        );
        let described = described(&page, "display").expect("described");
        assert_eq!(described.item.name, "display");
        assert_eq!(described.under_the_name, None);
    }

    #[test]
    fn an_item_describes_every_name_that_it_starts_with() {
        let page = page(
            "VkViewport",
            &[
                (
                    "x",
                    "pname:x and pname:y are the upper left corner [eq]#(x,y)#.",
                ),
                (
                    "width",
                    "pname:width, pname:height and pname:layers define the dimensions.",
                ),
                (
                    "srcSet",
                    "pname:srcSet, pname:srcBinding, and\npname:srcArrayElement are the source.",
                ),
                (
                    "minDepth",
                    "pname:minDepth and pname:maxDepth are the depth range,\nlike pname:near and pname:far.",
                ),
                ("maxDepth", "pname:maxDepth is the far end of the range."),
                (
                    "VK_FRONT",
                    "ename:VK_FRONT and ename:VK_BACK specify one face.",
                ),
            ],
        );
        for (name, item) in [
            ("x", "x"),
            ("y", "x"),
            ("width", "width"),
            ("height", "width"),
            ("layers", "width"),
            ("srcSet", "srcSet"),
            ("srcBinding", "srcSet"),
            ("srcArrayElement", "srcSet"),
            ("minDepth", "minDepth"),
            ("maxDepth", "maxDepth"),
            ("VK_FRONT", "VK_FRONT"),
            ("VK_BACK", "VK_FRONT"),
        ] {
            let described = described(&page, name)
                .unwrap_or_else(|| panic!("{name} has no description on the page"));
            assert_eq!(
                described.item.name, item,
                "{name} is described by the item of {item}"
            );
            assert_eq!(described.under_the_name, None, "{name}");
        }
        for name in ["near", "far", "depth", "corner"] {
            assert!(
                described(&page, name).is_none(),
                "{name} is not a name that an item starts with"
            );
        }
    }

    #[test]
    fn the_names_that_an_item_starts_with_are_a_list_of_one_kind_of_name() {
        for (text, names) in [
            ("pname:x and pname:y are the corner", vec!["x", "y"]),
            ("pname:a, pname:b and pname:c define", vec!["a", "b", "c"]),
            ("pname:a, pname:b, and\npname:c are", vec!["a", "b", "c"]),
            ("pname:a,\npname:b,\npname:c all have", vec!["a", "b", "c"]),
            ("ename:VK_A and ename:VK_B specify", vec!["VK_A", "VK_B"]),
            ("pname:a is like pname:b and pname:c", vec!["a"]),
            ("pname:a, if it is not 0, is", vec!["a"]),
            ("pname:a and, like pname:b, more", vec!["a"]),
            ("ename:VK_A and pname:scale equal to", vec!["VK_A"]),
            ("pname:a[0] and pname:b are", vec!["a"]),
            ("pname:a.b and pname:c are", vec!["a"]),
            ("pname:a or pname:b is", vec!["a"]),
            ("pname:a,pname:b are", vec!["a"]),
            ("pname:a andpname:b are", vec!["a"]),
            ("pname:a and and pname:b are", vec!["a"]),
            ("pname:a", vec!["a"]),
            ("If pname:a is set, pname:b is", vec![]),
            ("sname:VkThing and pname:a are", vec![]),
            ("pname: and pname:a are", vec![]),
            ("", vec![]),
        ] {
            assert_eq!(names_at_the_start(text), names, "{text}");
        }
    }

    #[test]
    fn every_row_is_in_use_in_the_embedded_specification() {
        assert_every_other_name_is_in_use(&Model::with_docs(Registry::get(), Docs::get()));
        let rows = DESCRIBED_UNDER_ANOTHER_NAME
            .iter()
            .map(|(command, parameter, _)| (command, parameter))
            .collect::<BTreeSet<_>>();
        assert_eq!(rows.len(), DESCRIBED_UNDER_ANOTHER_NAME.len());
    }

    fn stop(ir: &Registry, docs: &Docs) -> String {
        catch_quietly(|| assert_every_other_name_is_in_use(&Model::with_docs(ir, docs)))
            .expect_err("a row that is not in use stops the generator")
    }

    #[test]
    fn a_row_that_is_not_in_use_stops_the_generator() {
        let (ir, docs) = (Registry::get(), Docs::get());
        for (command, parameter, other) in DESCRIBED_UNDER_ANOTHER_NAME {
            let row = format!(
                "the row ({command}, {parameter}, {other}) of DESCRIBED_UNDER_ANOTHER_NAME"
            );
            let with_page = |change: &dyn Fn(&mut DocPage)| {
                let mut docs = docs.clone();
                let page = docs
                    .pages
                    .iter_mut()
                    .find(|page| page.name == *command)
                    .expect("the page of the command");
                change(page);
                docs
            };

            let corrected = with_page(&|page| {
                for item in &mut page.items {
                    if item.name == *other {
                        item.name = parameter.to_string();
                    }
                }
            });
            assert_eq!(
                stop(ir, &corrected),
                format!(
                    "{row} is not needed any more: the specification describes {parameter} under its own name"
                )
            );

            let removed = with_page(&|page| page.items.retain(|item| item.name != *other));
            assert_eq!(
                stop(ir, &removed),
                format!("{row} names a description that the specification does not have")
            );

            let mut without_page = docs.clone();
            without_page.pages.retain(|page| page.name != *command);
            assert_eq!(
                stop(ir, &without_page),
                format!("{row} names a command without a reference page")
            );

            let mut renamed = ir.clone();
            for c in renamed.commands.iter_mut().filter(|c| c.name == *command) {
                for p in c.params.iter_mut().filter(|p| p.name == *parameter) {
                    p.name = other.to_string();
                }
            }
            assert_eq!(
                stop(&renamed, docs),
                format!("{row} names a parameter that the command does not have")
            );

            let mut without_command = ir.clone();
            without_command.commands.retain(|c| c.name != *command);
            assert_eq!(
                stop(&without_command, docs),
                format!("{row} names a command that the registry does not have")
            );
        }
    }

    #[test]
    fn every_parameter_of_a_command_whose_page_describes_parameters_is_described() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut described = 0;
        let mut undescribed = BTreeSet::new();
        for c in &model.ir.commands {
            let Some(page) = model.page(&c.name) else {
                continue;
            };
            if page.items.is_empty() {
                continue;
            }
            for p in &c.params {
                match super::described(page, &p.name) {
                    Some(_) => described += 1,
                    None => {
                        undescribed.insert(format!("{}::{}", c.name, p.name));
                    }
                }
            }
        }
        assert!(described >= 2500, "{described}");
        assert_eq!(
            undescribed,
            BTreeSet::new(),
            "the specification describes these parameters under another name, which belongs in DESCRIBED_UNDER_ANOTHER_NAME, or not at all"
        );
    }

    #[test]
    fn every_parameter_of_a_function_pointer_whose_page_describes_parameters_is_described() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut described = 0;
        let mut undescribed = BTreeSet::new();
        for fp in &model.ir.func_pointers {
            let Some(page) = model.page(&fp.name) else {
                continue;
            };
            if page.items.is_empty() {
                continue;
            }
            for p in &fp.params {
                match super::described(page, &p.name) {
                    Some(_) => described += 1,
                    None => {
                        undescribed.insert(format!("{}::{}", fp.name, p.name));
                    }
                }
            }
        }
        assert!(described >= 20, "{described}");
        assert_eq!(undescribed, BTreeSet::new());
    }
}
