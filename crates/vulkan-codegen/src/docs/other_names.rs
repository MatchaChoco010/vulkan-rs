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

pub(super) fn described<'a>(page: &'a DocPage, name: &str) -> Option<Described<'a>> {
    let item = |name: &str| page.items.iter().find(|item| item.name == name);
    if let Some(item) = item(name) {
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
        let describes = |name: &str| page.items.iter().any(|item| item.name == name);
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
