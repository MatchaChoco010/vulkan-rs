use std::collections::BTreeMap;
use std::fmt;
use vulkan_registry::{DocPage, Docs};

const DESCRIBES: &str = "specifies";

#[derive(Clone, Copy)]
pub(crate) enum Place<'a> {
    Page(&'a DocPage),
    OutsidePages(&'a str),
}

impl fmt::Display for Place<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Page(page) => write!(f, "the page {}", page.name),
            Self::OutsidePages(file) => {
                write!(f, "the lines of {file} outside the reference pages")
            }
        }
    }
}

impl<'a> Place<'a> {
    pub(super) fn items_with(self, name: &'a str, text: &'a str) -> Vec<(&'a str, &'a str)> {
        match self {
            Self::Page(page) => page
                .items
                .iter()
                .map(|item| (item.name.as_str(), item.text.as_str()))
                .collect(),
            Self::OutsidePages(_) => vec![(name, text)],
        }
    }

    pub(super) fn unnamed_items(self) -> Vec<(&'a str, Option<&'a str>)> {
        match self {
            Self::Page(page) => page
                .unnamed_items
                .iter()
                .map(|item| (item.text.as_str(), item.problem.as_deref()))
                .collect(),
            Self::OutsidePages(_) => Vec::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ValueDescription<'a> {
    pub(crate) place: Place<'a>,
    pub(crate) text: &'a str,
}

pub(crate) type ValueDescriptions<'a> = BTreeMap<&'a str, Vec<ValueDescription<'a>>>;

pub(crate) fn describes_its_value(name: &str, text: &str) -> bool {
    let mut words = text.split_whitespace();
    words.next().and_then(|word| word.strip_prefix("ename:")) == Some(name)
        && words.next() == Some(DESCRIBES)
        && words.next().is_some()
}

pub(crate) fn value_descriptions(docs: &Docs) -> ValueDescriptions<'_> {
    let on_pages = docs.pages.iter().flat_map(|page| {
        page.items
            .iter()
            .map(move |item| (Place::Page(page), &item.name, &item.text))
    });
    let outside_pages = docs
        .items_outside_pages
        .iter()
        .map(|item| (Place::OutsidePages(&item.file), &item.name, &item.text));
    let mut descriptions = ValueDescriptions::new();
    for (place, name, text) in on_pages.chain(outside_pages) {
        if describes_its_value(name, text) {
            descriptions
                .entry(name.as_str())
                .or_default()
                .push(ValueDescription { place, text });
        }
    }
    descriptions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Cfg, Model};
    use crate::test_support::{deterministic_values, proptest_config};
    use proptest::prelude::*;
    use vulkan_registry::{DocItem, DocItemOutsidePages, Registry};

    #[test]
    fn an_item_describes_its_value_when_specifies_and_more_follow_the_name() {
        for (text, describes) in [
            ("ename:VK_A specifies the first value.", true),
            ("ename:VK_A\nspecifies support for\nthe first value.", true),
            ("ename:VK_A", false),
            ("ename:VK_A <<SCID-8>>", false),
            ("ename:VK_A is set.", false),
            ("ename:VK_A indicates the first value.", false),
            ("ename:VK_A specified the first value.", false),
            ("ename:VK_A, ename:VK_B specifies both.", false),
            ("ename:VK_A and pname:scale equal to `100`", false),
            ("ename:VK_A bytes of data, which specifies them", false),
            ("ename:VK_A specifies", false),
            ("ename:VK_A_B specifies another value.", false),
            ("ename:VK specifies a shorter name.", false),
            ("pname:VK_A specifies a member.", false),
            ("VK_A specifies a name without its macro.", false),
        ] {
            assert_eq!(describes_its_value("VK_A", text), describes, "{text}");
        }
    }

    const NAMES: [&str; 3] = ["VK_A", "VK_A_B", "VK_B"];

    fn item_text() -> impl Strategy<Value = String> {
        let word = prop::sample::select(vec![
            "ename:VK_A",
            "ename:VK_A_B",
            "ename:VK_B",
            "ename:VK_A,",
            "pname:VK_A",
            "elink:VK_A",
            "VK_A",
            "specifies",
            "specifies,",
            "specified",
            "indicates",
            "is",
            "the",
            "<<a,b>>",
            "*",
            "",
        ]);
        let space = prop::sample::select(vec![" ", " ", "\n", "  ", "\t", "\n    ", ""]);
        prop::collection::vec((word, space), 0..6).prop_map(|pieces| {
            pieces
                .into_iter()
                .map(|(word, space)| format!("{word}{space}"))
                .collect()
        })
    }

    fn as_the_rule_says(name: &str, text: &str) -> bool {
        regex::Regex::new(&format!(r"^\s*ename:{name}\s+specifies\s+\S"))
            .expect("the pattern of a description")
            .is_match(text)
    }

    proptest! {
        #![proptest_config(proptest_config(4096))]

        #[test]
        fn an_item_describes_its_value_exactly_where_the_pattern_of_a_description_matches(
            name in prop::sample::select(NAMES.to_vec()),
            text in item_text(),
        ) {
            prop_assert_eq!(
                describes_its_value(name, &text),
                as_the_rule_says(name, &text),
                "{:?} for {}", text, name
            );
        }
    }

    #[test]
    fn the_generated_items_describe_and_only_mention_each_name() {
        let texts = deterministic_values(item_text(), 4096);
        for name in NAMES {
            let described = texts
                .iter()
                .filter(|text| describes_its_value(name, text))
                .count();
            let mentioned = texts
                .iter()
                .filter(|text| text.starts_with(&format!("ename:{name}")))
                .count();
            assert!(described >= 4, "{name} is described {described} times");
            assert!(
                mentioned >= described + 100,
                "{name} starts {mentioned} items and {described} of them describe it"
            );
        }
    }

    fn page(name: &str, items: &[(&str, &str)]) -> DocPage {
        DocPage {
            name: name.to_string(),
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

    fn outside_pages(file: &str, name: &str, text: &str) -> DocItemOutsidePages {
        DocItemOutsidePages {
            file: file.to_string(),
            name: name.to_string(),
            text: text.to_string(),
        }
    }

    #[test]
    fn the_descriptions_of_a_value_are_the_items_that_describe_it_on_any_page_and_outside_the_pages()
     {
        let docs = Docs {
            pages: vec![
                page(
                    "VkFirst",
                    &[
                        ("VK_A", "ename:VK_A specifies the first value."),
                        ("VK_B", "ename:VK_B"),
                    ],
                ),
                page(
                    "VkSecond",
                    &[
                        ("VK_A", "ename:VK_A specifies the first value again."),
                        ("member", "pname:member specifies a member."),
                    ],
                ),
            ],
            items_outside_pages: vec![
                outside_pages("chapters/b.adoc", "VK_A", "ename:VK_A"),
                outside_pages(
                    "chapters/b.adoc",
                    "VK_B",
                    "ename:VK_B specifies the second.",
                ),
            ],
            ..Default::default()
        };
        let descriptions = value_descriptions(&docs)
            .into_iter()
            .map(|(name, descriptions)| {
                let found = descriptions
                    .iter()
                    .map(|description| (description.place.to_string(), description.text))
                    .collect::<Vec<_>>();
                (name, found)
            })
            .collect::<Vec<_>>();
        assert_eq!(
            descriptions,
            [
                (
                    "VK_A",
                    vec![
                        (
                            "the page VkFirst".to_string(),
                            "ename:VK_A specifies the first value."
                        ),
                        (
                            "the page VkSecond".to_string(),
                            "ename:VK_A specifies the first value again."
                        ),
                    ]
                ),
                (
                    "VK_B",
                    vec![(
                        "the lines of chapters/b.adoc outside the reference pages".to_string(),
                        "ename:VK_B specifies the second."
                    )]
                ),
            ]
        );
    }

    const DESCRIBED_OUTSIDE_THE_PAGE_OF_THEIR_TYPE: &[(&str, &str)] = &[
        (
            "VK_FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_BIT_ARM",
            "the lines of chapters/formats.adoc outside the reference pages",
        ),
        (
            "VK_FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_BIT_ARM",
            "the lines of chapters/formats.adoc outside the reference pages",
        ),
        (
            "VK_FORMAT_FEATURE_2_TENSOR_SHADER_BIT_ARM",
            "the lines of chapters/formats.adoc outside the reference pages",
        ),
        (
            "VK_PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_BIT_EXT",
            "the page VkPipelineStageFlagBits",
        ),
        (
            "VK_VIDEO_ENCODE_CAPABILITY_EMPHASIS_MAP_BIT_KHR",
            "the lines of chapters/video/encode.adoc outside the reference pages",
        ),
        (
            "VK_VIDEO_ENCODE_CAPABILITY_QUANTIZATION_DELTA_MAP_BIT_KHR",
            "the lines of chapters/video/encode.adoc outside the reference pages",
        ),
    ];

    #[test]
    fn exactly_these_values_of_the_specification_are_described_outside_the_page_of_their_type() {
        let (ir, docs) = (Registry::get(), Docs::get());
        let model = Model::with_docs(ir, docs);
        let enums = ir.enums.iter().map(|e| (&e.name, &e.fields));
        let bitmasks = ir.bitmasks.iter().map(|b| (&b.name, &b.fields));
        let mut taken = Vec::new();
        for (owner, fields) in enums.chain(bitmasks) {
            if model.page(owner).is_none() {
                continue;
            }
            for field in fields {
                let value = field.name.as_str();
                let is_the_type_of_a_structure = owner == "VkStructureType"
                    && ir
                        .structs
                        .iter()
                        .any(|s| s.s_type.as_deref() == Some(value));
                let is_the_type_of_a_handle =
                    owner == "VkObjectType" && ir.handles.iter().any(|h| h.object_type == value);
                let described_before = model.described(&Cfg::default(), owner, value).is_some()
                    || is_the_type_of_a_structure
                    || is_the_type_of_a_handle
                    || model.tabulated(&Cfg::default(), owner, value).is_some();
                let elsewhere = model.value_descriptions.get(value);
                let (false, Some(descriptions)) = (described_before, elsewhere) else {
                    continue;
                };
                let places = descriptions
                    .iter()
                    .map(|description| description.place.to_string())
                    .collect::<Vec<_>>();
                taken.push((field.name.clone(), places.join(", ")));
                let described =
                    model.described_outside_the_page(&Cfg::default(), owner, &field.name);
                let starts_with_the_name = described.as_deref().is_some_and(|blocks| {
                    matches!(
                        blocks.first(),
                        Some(crate::docs::Block::Text(text))
                            if text.split_whitespace().take(2).collect::<Vec<_>>()
                                == [format!("`{}`", field.name).as_str(), DESCRIBES]
                    )
                });
                assert!(
                    starts_with_the_name,
                    "{} is described as {described:?}",
                    field.name
                );
            }
        }
        let outside_pages = &docs.items_outside_pages;
        let describing = outside_pages
            .iter()
            .filter(|item| describes_its_value(&item.name, &item.text))
            .count();
        assert_eq!(
            (outside_pages.len(), describing),
            (206, 9),
            "this many list items outside the reference pages start with a value, and this \
             many of them describe it"
        );
        taken.sort();
        let pinned = DESCRIBED_OUTSIDE_THE_PAGE_OF_THEIR_TYPE
            .iter()
            .map(|(value, place)| (value.to_string(), place.to_string()))
            .collect::<Vec<_>>();
        assert_eq!(
            taken, pinned,
            "exactly these values are described by nothing that comes first (a list item or a \
             table row of the reference page of their type, or the structure or the handle that \
             the registry has for a structure type or an object type) and by one list item \
             elsewhere. A value that joins them takes the text of that item, so read the item \
             before listing the value here"
        );
    }
}
