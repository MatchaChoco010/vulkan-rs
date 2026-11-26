use super::value_descriptions::describes_its_value;
use std::borrow::Cow;
use vulkan_registry::{DocPage, DocUnnamedItem};

const MEMBER_ANCHORS: [&str; 2] = ["features-", "limits-"];
const NAME_MACROS: [&str; 2] = ["pname:", "ename:"];
const VALUE_MACRO: &str = "ename:";

fn is_word(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn names_the_member(text: &str, member: &str) -> bool {
    let named = format!("pname:{member}");
    text.match_indices(&named)
        .any(|(at, _)| !text[at + named.len()..].starts_with(is_word))
}

fn anchored_members(item: &DocUnnamedItem) -> impl Iterator<Item = &str> {
    item.anchors
        .iter()
        .filter_map(|anchor| {
            MEMBER_ANCHORS
                .iter()
                .find_map(|prefix| anchor.strip_prefix(prefix))
        })
        .filter(|member| names_the_member(&item.text, member))
}

fn from_the_first_name(text: &str) -> Option<&str> {
    text.char_indices()
        .filter(|(at, _)| !text[..*at].ends_with(is_word))
        .map(|(at, _)| &text[at..])
        .find(|rest| NAME_MACROS.iter().any(|name| rest.starts_with(name)))
}

fn value_named_first(text: &str) -> Option<&str> {
    let name = from_the_first_name(text)?.strip_prefix(VALUE_MACRO)?;
    let name = &name[..name.find(|c| !is_word(c)).unwrap_or(name.len())];
    describes_its_value(name, from_the_first_name(text)?).then_some(name)
}

fn is_a_value_name(word: &str) -> bool {
    word.starts_with("VK_")
        && word
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn value_without_its_macro(text: &str) -> Option<&str> {
    let name = text.split_whitespace().next()?;
    (is_a_value_name(name) && describes_its_value(name, &format!("{VALUE_MACRO}{text}")))
        .then_some(name)
}

pub(super) fn described_names(item: &DocUnnamedItem) -> Vec<&str> {
    let value = value_without_its_macro(&item.text).or_else(|| value_named_first(&item.text));
    anchored_members(item).chain(value).collect()
}

pub(super) fn description<'a>(page: &'a DocPage, name: &str) -> Option<Cow<'a, str>> {
    let describing = page
        .unnamed_items
        .iter()
        .filter(|item| described_names(item).contains(&name))
        .collect::<Vec<_>>();
    let [item] = describing.as_slice() else {
        assert!(
            describing.is_empty(),
            "{} list items of the page {} describe {name} without starting with its name, so \
             that nothing says which one to take",
            describing.len(),
            page.name
        );
        return None;
    };
    if let Some(problem) = &item.problem {
        panic!(
            "the list item that describes {name} on the page {} {problem}",
            page.name
        );
    }
    Some(match value_without_its_macro(&item.text) {
        Some(_) => Cow::Owned(format!("{VALUE_MACRO}{}", item.text)),
        None => Cow::Borrowed(&item.text),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs::other_names;
    use crate::model::{Cfg, Model};
    use crate::test_support::{catch_quietly, deterministic_values, proptest_config};
    use proptest::prelude::*;
    use std::collections::BTreeSet;
    use vulkan_registry::{Docs, Registry};

    fn item(anchors: &[&str], text: &str) -> DocUnnamedItem {
        DocUnnamedItem {
            anchors: anchors.iter().map(|anchor| anchor.to_string()).collect(),
            text: text.to_string(),
            problem: None,
        }
    }

    #[test]
    fn an_item_describes_the_member_of_its_anchor_or_the_value_it_names_first_and_specifies() {
        for (anchors, text, described) in [
            (
                vec!["features-wide"],
                "If pname:wide is ename:VK_TRUE, lines are wide.",
                vec!["wide"],
            ),
            (
                vec!["limits-maxWidth"],
                "When the <<features-wide,pname:wide>> feature is enabled,\npname:maxWidth indicates the width.",
                vec!["maxWidth"],
            ),
            (
                vec!["other", "limits-maxWidth", "features-wide"],
                "Lines of pname:maxWidth[0] and pname:wide.",
                vec!["maxWidth", "wide"],
            ),
            (
                vec!["features-wide"],
                "If pname:wider is ename:VK_TRUE, lines are wider.",
                vec![],
            ),
            (
                vec!["features-wide"],
                "The feature wide of sname:VkWide::wide.",
                vec![],
            ),
            (
                vec!["wide", "extension-features-wide", "limit-wide", "features"],
                "If pname:wide is ename:VK_TRUE, lines are wide.",
                vec![],
            ),
            (
                vec![],
                "If pname:wide is ename:VK_TRUE, lines are wide.",
                vec![],
            ),
            (
                vec![],
                "When drawing lines, ename:VK_LINE_WIDE\nspecifies that they are wide.",
                vec!["VK_LINE_WIDE"],
            ),
            (
                vec![],
                "(ename:VK_LINE_WIDE specifies wide lines, ename:VK_LINE_THIN specifies thin ones.)",
                vec!["VK_LINE_WIDE"],
            ),
            (
                vec![],
                "When pname:wide is set, ename:VK_LINE_WIDE specifies wide lines.",
                vec![],
            ),
            (
                vec![],
                "With ename:VK_LINE_THIN, ename:VK_LINE_WIDE specifies wide lines.",
                vec![],
            ),
            (
                vec![],
                "When drawing lines, ename:VK_LINE_WIDE is set.",
                vec![],
            ),
            (
                vec![],
                "When drawing lines, ename:VK_LINE_WIDE, which specifies wide lines, is set.",
                vec![],
            ),
            (
                vec![],
                "When drawing lines, ename:VK_LINE_WIDE specifies",
                vec![],
            ),
            (
                vec![],
                "A typename:VK_LINE_WIDE specifies no value.",
                vec![],
            ),
            (
                vec![],
                "VK_LINE_WIDE specifies wide lines.",
                vec!["VK_LINE_WIDE"],
            ),
            (
                vec![],
                "VK_LINE_WIDE\nspecifies wide lines.",
                vec!["VK_LINE_WIDE"],
            ),
            (vec![], "VK_LINE_WIDE is set.", vec![]),
            (vec![], "VK_LINE_WIDE specifies", vec![]),
            (vec![], "VK_LINE_WIDE, which specifies wide lines", vec![]),
            (vec![], "Vk_LINE_WIDE specifies wide lines.", vec![]),
            (vec![], "VkLine specifies wide lines.", vec![]),
            (vec![], "WIDE specifies wide lines.", vec![]),
            (vec![], "code:VK_LINE_WIDE specifies wide lines.", vec![]),
            (
                vec![],
                "Otherwise, VK_LINE_WIDE specifies wide lines.",
                vec![],
            ),
        ] {
            assert_eq!(
                described_names(&item(&anchors, text)),
                described,
                "{anchors:?} {text}"
            );
        }
    }

    fn generated_item() -> impl Strategy<Value = DocUnnamedItem> {
        let anchor = prop::sample::select(vec![
            "features-wide",
            "limits-wide",
            "features-wider",
            "limits-maxWidth",
            "extension-features-wide",
            "wide",
            "features-",
        ]);
        let word = prop::sample::select(vec![
            "pname:wide",
            "pname:wider",
            "pname:maxWidth",
            "pname:wide,",
            "xpname:wide",
            "sname:wide",
            "wide",
            "ename:VK_A",
            "ename:VK_A_B",
            "ename:VK_A,",
            "typename:VK_A",
            "(ename:VK_A",
            "VK_A",
            "VK_A_B",
            "VK_a",
            "VKA",
            "specifies",
            "specifies,",
            "is",
            "the",
            "When",
            "",
        ]);
        let space = prop::sample::select(vec![" ", " ", "\n", "  ", "\n    ", ""]);
        (
            prop::collection::vec(anchor, 0..3),
            prop::collection::vec((word, space), 0..6),
        )
            .prop_map(|(anchors, pieces)| DocUnnamedItem {
                anchors: anchors.into_iter().map(str::to_string).collect(),
                text: pieces
                    .into_iter()
                    .map(|(word, space)| format!("{word}{space}"))
                    .collect(),
                problem: None,
            })
    }

    fn as_the_rules_say(item: &DocUnnamedItem) -> Vec<String> {
        let regex = |pattern: &str| regex::Regex::new(pattern).expect("a pattern of the rules");
        let mut names = Vec::new();
        for anchor in &item.anchors {
            if let Some(member) = regex(r"^(?:features|limits)-(.*)$").captures(anchor) {
                let named = format!(r"pname:{}(?:[^A-Za-z0-9_]|$)", regex::escape(&member[1]));
                if regex(&named).is_match(&item.text) {
                    names.push(member[1].to_string());
                }
            }
        }
        let specified = r"\s+specifies\s+\S";
        if let Some(bare) = regex(&format!("^(VK_[A-Z0-9_]+){specified}")).captures(&item.text) {
            names.push(bare[1].to_string());
        } else if let Some(first) = regex(r"(?-u:\b)(?:pname|ename):").find(&item.text)
            && let Some(value) =
                regex(&format!(r"^ename:(\w+){specified}")).captures(&item.text[first.start()..])
        {
            names.push(value[1].to_string());
        }
        names
    }

    proptest! {
        #![proptest_config(proptest_config(4096))]

        #[test]
        fn an_item_describes_exactly_the_names_that_the_patterns_of_the_rules_find(
            item in generated_item(),
        ) {
            prop_assert_eq!(
                described_names(&item),
                as_the_rules_say(&item),
                "{:?} {:?}", item.anchors, item.text
            );
        }
    }

    #[test]
    fn the_generated_items_describe_members_and_values_in_every_form_and_most_describe_nothing() {
        let items = deterministic_values(generated_item(), 8192);
        let described = |holds: &dyn Fn(&DocUnnamedItem) -> bool| {
            items
                .iter()
                .filter(|item| !described_names(item).is_empty() && holds(item))
                .count()
        };
        let members = described(&|item| anchored_members(item).next().is_some());
        let without_macro = described(&|item| value_without_its_macro(&item.text).is_some());
        let named_first = described(&|item| value_named_first(&item.text).is_some());
        let nothing = items.len() - described(&|_| true);
        for (what, count, at_least) in [
            ("members by their anchor", members, 500),
            ("values without their macro", without_macro, 10),
            ("values named first", named_first, 15),
            ("items that describe nothing", nothing, 6000),
        ] {
            assert!(count >= at_least, "only {count} {what}");
        }
    }

    fn page(items: &[DocUnnamedItem]) -> DocPage {
        DocPage {
            name: "VkLine".to_string(),
            unnamed_items: items.to_vec(),
            ..Default::default()
        }
    }

    #[test]
    fn the_description_of_a_name_is_the_one_item_that_describes_it_with_the_macro_of_a_value() {
        let page = page(&[
            item(&["features-wide"], "If pname:wide is set, lines are wide."),
            item(&[], "VK_LINE_WIDE specifies wide lines."),
            item(&[], "For thin lines, ename:VK_LINE_THIN specifies them."),
            item(&[], "ename:VK_LINE_THIN is set."),
        ]);
        for (name, text) in [
            ("wide", Some("If pname:wide is set, lines are wide.")),
            (
                "VK_LINE_WIDE",
                Some("ename:VK_LINE_WIDE specifies wide lines."),
            ),
            (
                "VK_LINE_THIN",
                Some("For thin lines, ename:VK_LINE_THIN specifies them."),
            ),
            ("thin", None),
            ("VK_LINE", None),
        ] {
            assert_eq!(description(&page, name).as_deref(), text, "{name}");
        }
    }

    #[test]
    fn several_items_that_describe_a_name_and_an_item_that_cannot_be_read_abort_generation() {
        let twice = page(&[
            item(&["features-wide"], "If pname:wide is set, lines are wide."),
            item(
                &["limits-wide"],
                "Unless pname:wide is set, lines are thin.",
            ),
            item(&[], "VK_LINE_WIDE specifies wide lines."),
        ]);
        assert_eq!(
            catch_quietly(|| description(&twice, "wide").is_some()),
            Err(
                "2 list items of the page VkLine describe wide without starting with its name, \
                 so that nothing says which one to take"
                    .to_string()
            )
        );
        assert!(description(&twice, "VK_LINE_WIDE").is_some());
        let mut unreadable = item(&[], "VK_LINE_WIDE specifies wide lines.");
        unreadable.problem =
            Some("cannot be read reliably: two list continuations in a row".to_string());
        let unreadable = page(&[
            unreadable,
            item(&["features-wide"], "If pname:wide is set, lines are wide."),
        ]);
        assert_eq!(
            catch_quietly(|| description(&unreadable, "VK_LINE_WIDE").is_some()),
            Err(
                "the list item that describes VK_LINE_WIDE on the page VkLine cannot be read \
                 reliably: two list continuations in a row"
                    .to_string()
            )
        );
        assert!(description(&unreadable, "wide").is_some());
    }

    const DESCRIBED_BY_AN_ITEM_THAT_STARTS_WITH_NO_NAME: &[(&str, &str)] = &[
        ("VkGeometryTypeKHR", "VK_GEOMETRY_TYPE_MICROMAP_KHR"),
        (
            "VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR",
            "triStripVertexOrderIndependentOfProvokingVertex",
        ),
        (
            "VkPhysicalDeviceMaintenance9PropertiesKHR",
            "image2DViewOf3DSparse",
        ),
        (
            "VkPhysicalDeviceVulkan12Features",
            "subgroupBroadcastDynamicId",
        ),
        (
            "VkPipelineCreateFlagBits",
            "VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT",
        ),
        (
            "VkPipelineCreateFlagBits2",
            "VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT",
        ),
    ];

    #[test]
    fn exactly_these_names_of_the_specification_are_described_by_an_item_that_starts_with_no_name()
    {
        let (ir, docs) = (Registry::get(), Docs::get());
        let model = Model::with_docs(ir, docs);
        let owned = |owner: &str| -> BTreeSet<&str> {
            let structs = ir.structs.iter().filter(|s| s.name == owner);
            let commands = ir.commands.iter().filter(|c| c.name == owner);
            let enums = ir.enums.iter().filter(|e| e.name == owner);
            let bitmasks = ir.bitmasks.iter().filter(|b| b.name == owner);
            let fields = structs
                .flat_map(|s| &s.members)
                .chain(commands.flat_map(|c| &c.params))
                .map(|field| field.name.as_str());
            let values = enums
                .flat_map(|e| &e.fields)
                .chain(bitmasks.flat_map(|b| &b.fields))
                .map(|field| field.name.as_str());
            fields.chain(values).collect()
        };
        let mut scanned = 0;
        let mut described = Vec::new();
        for page in &docs.pages {
            for item in &page.unnamed_items {
                scanned += 1;
                for name in described_names(item) {
                    described.push((page.name.as_str(), name));
                    assert!(
                        owned(&page.name).contains(name),
                        "the item names {name}, which {} does not have: {}",
                        page.name,
                        item.text
                    );
                    let found = other_names::described(page, name)
                        .unwrap_or_else(|| panic!("{name} has no description on {}", page.name));
                    assert!(
                        found.text.ends_with(&item.text),
                        "{name} on {} is described by another item: {}",
                        page.name,
                        found.text
                    );
                    assert!(model.described(&Cfg::default(), &page.name, name).is_some());
                }
            }
        }
        assert!(scanned >= 500, "only {scanned} items were read");
        assert_eq!(
            described, DESCRIBED_BY_AN_ITEM_THAT_STARTS_WITH_NO_NAME,
            "exactly these list items on the reference pages start with no name and have the \
             form of the description of a member or a value. An item that joins them becomes \
             the documentation of what it describes, so read it before listing it here"
        );
    }
}
