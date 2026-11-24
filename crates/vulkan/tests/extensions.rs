use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use std::collections::HashSet;
use std::ffi::{CStr, CString};
use vulkan::vk::{
    self, CommandInfo, CommandLevel, Dependency, ExtensionInfo, ExtensionLevel, VkResult,
};

fn cases(default: u32) -> ProptestConfig {
    let cases = std::env::var("PROPTEST_CASES")
        .ok()
        .and_then(|cases| cases.parse().ok())
        .unwrap_or(default);
    ProptestConfig {
        cases,
        failure_persistence: None,
        ..ProptestConfig::default()
    }
}

fn near_misses(name: &[u8]) -> Vec<Vec<u8>> {
    let mut misses = vec![
        name.to_vec(),
        name[..name.len() - 1].to_vec(),
        [name, b"_"].concat(),
        [name, b"a"].concat(),
        name.to_ascii_lowercase(),
    ];
    let last = name.len() - 1;
    for change in [1u8, u8::MAX] {
        let mut changed = name.to_vec();
        changed[last] = changed[last].wrapping_add(change);
        if changed[last].is_ascii_graphic() {
            misses.push(changed);
        }
    }
    misses
}

const PREFIXES: [&str; 5] = ["", "VK", "VK_", "VK_KHR_", "vk"];

#[test]
fn extensions_are_sorted_by_name_and_get_finds_exactly_the_listed_names() {
    assert!(
        vk::EXTENSIONS
            .windows(2)
            .all(|pair| pair[0].name < pair[1].name),
        "EXTENSIONS is strictly sorted by name, which get relies on to binary search"
    );
    let mut absent = 0;
    let candidates = vk::EXTENSIONS
        .iter()
        .flat_map(|info| near_misses(info.name.to_bytes()))
        .chain(PREFIXES.iter().map(|prefix| prefix.as_bytes().to_vec()));
    for candidate in candidates {
        let candidate = CString::new(candidate).unwrap();
        let listed = vk::EXTENSIONS
            .iter()
            .find(|info| info.name == candidate.as_c_str());
        assert_eq!(ExtensionInfo::get(&candidate), listed, "{candidate:?}");
        absent += usize::from(listed.is_none());
    }
    assert!(absent > 2 * vk::EXTENSIONS.len(), "{absent}");
}

#[test]
fn commands_are_sorted_by_name_and_get_finds_exactly_the_listed_names() {
    assert!(
        vk::COMMANDS
            .windows(2)
            .all(|pair| pair[0].name < pair[1].name),
        "COMMANDS is strictly sorted by name, which get relies on to binary search"
    );
    let mut absent = 0;
    let candidates = vk::COMMANDS
        .iter()
        .flat_map(|info| near_misses(info.name.as_bytes()))
        .chain(PREFIXES.iter().map(|prefix| prefix.as_bytes().to_vec()));
    for candidate in candidates {
        let candidate = String::from_utf8(candidate).unwrap();
        let listed = vk::COMMANDS.iter().find(|info| info.name == candidate);
        assert_eq!(CommandInfo::get(&candidate), listed, "{candidate:?}");
        absent += usize::from(listed.is_none());
    }
    assert!(absent > 2 * vk::COMMANDS.len(), "{absent}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tree {
    Version(u32),
    Extension(String),
    All(Vec<Tree>),
    Any(Vec<Tree>),
}

fn normalized(tree: Tree) -> Tree {
    let flatten = |children: Vec<Tree>, all: bool| {
        let mut flat = Vec::new();
        for child in children.into_iter().map(normalized) {
            match child {
                Tree::All(grandchildren) if all => flat.extend(grandchildren),
                Tree::Any(grandchildren) if !all => flat.extend(grandchildren),
                child => flat.push(child),
            }
        }
        if flat.len() == 1 {
            flat.pop().unwrap()
        } else if all {
            Tree::All(flat)
        } else {
            Tree::Any(flat)
        }
    };
    match tree {
        Tree::All(children) => flatten(children, true),
        Tree::Any(children) => flatten(children, false),
        leaf => leaf,
    }
}

fn tree_of(dependency: &Dependency) -> Tree {
    match *dependency {
        Dependency::Version(version) => Tree::Version(version),
        Dependency::Extension(name) => Tree::Extension(name.to_str().unwrap().to_string()),
        Dependency::All(all) => Tree::All(all.iter().map(tree_of).collect()),
        Dependency::Any(any) => Tree::Any(any.iter().map(tree_of).collect()),
    }
}

struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}

impl Parser<'_> {
    fn expression(&mut self) -> Result<Tree, String> {
        let mut tree = self.term()?;
        while let Some(&operator) = self.text.get(self.at) {
            let all = match operator {
                b'+' => true,
                b',' => false,
                _ => break,
            };
            self.at += 1;
            let right = self.term()?;
            tree = if all {
                Tree::All(vec![tree, right])
            } else {
                Tree::Any(vec![tree, right])
            };
        }
        Ok(tree)
    }

    fn term(&mut self) -> Result<Tree, String> {
        if self.text.get(self.at) == Some(&b'(') {
            self.at += 1;
            let tree = self.expression()?;
            if self.text.get(self.at) != Some(&b')') {
                return Err(format!("expected `)` at {}", self.at));
            }
            self.at += 1;
            return Ok(tree);
        }
        let start = self.at;
        while self
            .text
            .get(self.at)
            .is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_')
        {
            self.at += 1;
        }
        let name = std::str::from_utf8(&self.text[start..self.at]).unwrap();
        if name.is_empty() {
            return Err(format!("expected a name at {start}"));
        }
        if let Some(version) = name.strip_prefix("VK_VERSION_") {
            let (major, minor) = version
                .split_once('_')
                .ok_or_else(|| format!("{name} is not a version"))?;
            let major = major.parse().map_err(|_| name.to_string())?;
            let minor = minor.parse().map_err(|_| name.to_string())?;
            return Ok(Tree::Version(vk::make_api_version(0, major, minor, 0)));
        }
        Ok(Tree::Extension(name.to_string()))
    }
}

fn parse_depends(text: &str) -> Result<Tree, String> {
    let mut parser = Parser {
        text: text.as_bytes(),
        at: 0,
    };
    let tree = parser.expression()?;
    if parser.at != text.len() {
        return Err(format!("unexpected text at {}", parser.at));
    }
    Ok(tree)
}

#[test]
fn registry_depends_syntax_groups_left_to_right_with_parentheses_first() {
    let extension = |name: &str| Tree::Extension(name.to_string());
    assert_eq!(
        parse_depends("A+B,C"),
        Ok(Tree::Any(vec![
            Tree::All(vec![extension("A"), extension("B")]),
            extension("C")
        ]))
    );
    assert_eq!(
        parse_depends("A,B+C"),
        Ok(Tree::All(vec![
            Tree::Any(vec![extension("A"), extension("B")]),
            extension("C")
        ]))
    );
    assert_eq!(
        parse_depends("A,(B+VK_VERSION_1_2)"),
        Ok(Tree::Any(vec![
            extension("A"),
            Tree::All(vec![extension("B"), Tree::Version(vk::API_VERSION_1_2)])
        ]))
    );
    assert!(parse_depends("(A").is_err());
    assert!(parse_depends("A+").is_err());
}

fn leaves<'a>(tree: &'a Tree, found: &mut Vec<&'a Tree>) {
    match tree {
        Tree::All(children) | Tree::Any(children) => {
            children.iter().for_each(|child| leaves(child, found));
        }
        leaf => found.push(leaf),
    }
}

#[test]
fn every_depends_expression_displays_in_the_registry_syntax_and_names_known_extensions() {
    let mut checked = 0;
    for info in vk::EXTENSIONS {
        let Some(depends) = info.depends else {
            continue;
        };
        checked += 1;
        let shown = depends.to_string();
        let read =
            parse_depends(&shown).unwrap_or_else(|e| panic!("{:?}: {shown}: {e}", info.name));
        assert_eq!(
            normalized(read.clone()),
            normalized(tree_of(&depends)),
            "{:?}: {shown}",
            info.name
        );
        let mut found = Vec::new();
        leaves(&read, &mut found);
        for leaf in found {
            match leaf {
                Tree::Version(version) => assert!(
                    vk::api_version_major(*version) == 1 && *version <= vk::HEADER_VERSION_COMPLETE,
                    "{:?}: {shown}",
                    info.name
                ),
                Tree::Extension(name) => assert!(
                    ExtensionInfo::get(&CString::new(name.as_str()).unwrap()).is_some(),
                    "{:?} depends on {name}, which is not listed",
                    info.name
                ),
                _ => unreachable!(),
            }
        }
    }
    assert!(checked > 300, "{checked}");
}

fn leak(list: Vec<Dependency>) -> &'static [Dependency] {
    Box::leak(list.into_boxed_slice())
}

fn any_version() -> impl Strategy<Value = u32> {
    (0..=4u32).prop_map(|minor| vk::make_api_version(0, 1, minor, 0))
}

fn any_dependency() -> impl Strategy<Value = Dependency> {
    let leaf = prop_oneof![
        any_version().prop_map(Dependency::Version),
        prop::sample::select(
            vk::EXTENSIONS
                .iter()
                .map(|info| info.name)
                .collect::<Vec<_>>()
        )
        .prop_map(Dependency::Extension),
    ];
    leaf.prop_recursive(3, 24, 4, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 1..4).prop_map(|list| Dependency::All(leak(list))),
            prop::collection::vec(inner, 1..4).prop_map(|list| Dependency::Any(leak(list))),
        ]
    })
}

proptest! {
    #![proptest_config(cases(512))]

    #[test]
    fn any_dependency_displays_in_the_registry_syntax(dependency in any_dependency()) {
        let shown = dependency.to_string();
        let read = parse_depends(&shown).map_err(TestCaseError::fail)?;
        prop_assert_eq!(normalized(read), normalized(tree_of(&dependency)), "{}", shown);
    }
}

fn promotion_chain(name: &CStr) -> (Vec<&'static CStr>, Option<u32>) {
    let mut chain = Vec::new();
    let Some(mut info) = ExtensionInfo::get(name) else {
        return (chain, None);
    };
    while let Some(next) = info.promoted_to_extension {
        assert!(
            chain.len() <= vk::EXTENSIONS.len(),
            "{name:?} has a promotion cycle"
        );
        chain.push(next);
        info = ExtensionInfo::get(next).unwrap();
    }
    (chain, info.promoted_to_version)
}

fn documented_is_met(dependency: &Dependency, api_version: u32, enabled: &HashSet<&CStr>) -> bool {
    match *dependency {
        Dependency::Version(version) => api_version >= version,
        Dependency::Extension(name) => {
            let (chain, version) = promotion_chain(name);
            enabled.contains(name)
                || chain.iter().any(|successor| enabled.contains(successor))
                || version.is_some_and(|version| version <= api_version)
        }
        Dependency::All(all) => all
            .iter()
            .all(|d| documented_is_met(d, api_version, enabled)),
        Dependency::Any(any) => any
            .iter()
            .any(|d| documented_is_met(d, api_version, enabled)),
    }
}

const DENSITIES: [f64; 5] = [0.0, 0.05, 0.3, 0.7, 1.0];

fn enabled_extensions() -> impl Strategy<Value = Vec<bool>> {
    (0..DENSITIES.len()).prop_flat_map(|density| {
        prop::collection::vec(
            prop::bool::weighted(DENSITIES[density]),
            vk::EXTENSIONS.len(),
        )
    })
}

fn depended_on() -> Vec<usize> {
    let mut found = Vec::new();
    for info in vk::EXTENSIONS {
        let Some(depends) = info.depends else {
            continue;
        };
        let tree = tree_of(&depends);
        let mut named = Vec::new();
        leaves(&tree, &mut named);
        for leaf in named {
            if let Tree::Extension(name) = leaf {
                found.extend(
                    vk::EXTENSIONS
                        .iter()
                        .position(|info| info.name.to_str() == Ok(name.as_str())),
                );
            }
        }
    }
    found.sort_unstable();
    found.dedup();
    found
}

fn monotonic_case() -> impl Strategy<Value = (Vec<bool>, usize, u32)> {
    let added = prop_oneof![0..vk::EXTENSIONS.len(), prop::sample::select(depended_on()),];
    (enabled_extensions(), added, 0..=4u32)
}

fn names(enabled: &[bool]) -> HashSet<&'static CStr> {
    vk::EXTENSIONS
        .iter()
        .zip(enabled)
        .filter(|(_, enabled)| **enabled)
        .map(|(info, _)| info.name)
        .collect()
}

fn version(minor: u32) -> u32 {
    vk::make_api_version(0, 1, minor, 0)
}

#[derive(Default)]
struct Outcomes {
    met: u32,
    not_met: u32,
    met_by_the_added_extension: u32,
    met_by_the_next_version: u32,
}

fn check_monotonic(
    enabled: &[bool],
    added: usize,
    minor: u32,
    outcomes: &mut Outcomes,
) -> Result<(), TestCaseError> {
    let before = names(enabled);
    let mut more = before.clone();
    more.insert(vk::EXTENSIONS[added].name);
    for info in vk::EXTENSIONS {
        let Some(depends) = info.depends else {
            continue;
        };
        let met = info.dependencies_met(version(minor), |name: &CStr| before.contains(name));
        prop_assert_eq!(
            met,
            documented_is_met(&depends, version(minor), &before),
            "{:?}: {}",
            info.name,
            depends
        );
        let with_more = info.dependencies_met(version(minor), |name: &CStr| more.contains(name));
        prop_assert!(
            !met || with_more,
            "{:?}: enabling {:?} unmet {}",
            info.name,
            vk::EXTENSIONS[added].name,
            depends
        );
        if met {
            outcomes.met += 1;
        } else {
            outcomes.not_met += 1;
            outcomes.met_by_the_added_extension += u32::from(with_more);
        }
        if minor < 4 {
            let later =
                info.dependencies_met(version(minor + 1), |name: &CStr| before.contains(name));
            prop_assert!(
                !met || later,
                "{:?}: Vulkan 1.{} unmet {}",
                info.name,
                minor + 1,
                depends
            );
            outcomes.met_by_the_next_version += u32::from(!met && later);
        }
    }
    Ok(())
}

proptest! {
    #![proptest_config(cases(64))]

    #[test]
    fn dependencies_stay_met_when_an_extension_is_enabled_or_the_version_is_higher(
        (enabled, added, minor) in monotonic_case()
    ) {
        check_monotonic(&enabled, added, minor, &mut Outcomes::default())?;
    }
}

#[test]
fn generated_extension_sets_meet_and_miss_dependencies() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = monotonic_case();
    let mut outcomes = Outcomes::default();
    for _ in 0..32 {
        let (enabled, added, minor) = strategy.new_tree(&mut runner).unwrap().current();
        check_monotonic(&enabled, added, minor, &mut outcomes).unwrap();
    }
    assert!(
        outcomes.met >= 1000
            && outcomes.not_met >= 1000
            && outcomes.met_by_the_added_extension >= 10
            && outcomes.met_by_the_next_version >= 50,
        "met {}, not met {}, met once an extension is added {}, met on the next version {}",
        outcomes.met,
        outcomes.not_met,
        outcomes.met_by_the_added_extension,
        outcomes.met_by_the_next_version
    );
}

#[test]
fn every_dependency_is_met_with_every_extension_enabled_on_the_newest_version() {
    for info in vk::EXTENSIONS {
        assert!(
            info.dependencies_met(vk::API_VERSION_1_4, |_: &CStr| true),
            "{:?}",
            info.name
        );
    }
}

#[test]
fn dependency_on_an_extension_is_met_by_everything_it_was_promoted_to() {
    let nothing_enabled = |_: &CStr| false;
    let mut chains_longer_than_one_step = 0;
    for info in vk::EXTENSIONS {
        let dependency = Dependency::Extension(info.name);
        let mut successors = Vec::new();
        let mut core_version = info.promoted_to_version;
        let mut next = info.promoted_to_extension;
        while let Some(name) = next {
            assert!(
                name != info.name && !successors.contains(&name),
                "{info:?}: the promotion chain {successors:?} loops back to {name:?}"
            );
            successors.push(name);
            let successor = ExtensionInfo::get(name)
                .unwrap_or_else(|| panic!("{info:?}: promoted to {name:?}, which is not listed"));
            core_version = successor.promoted_to_version;
            next = successor.promoted_to_extension;
        }
        if successors.len() + usize::from(core_version.is_some()) >= 2 {
            chains_longer_than_one_step += 1;
        }
        for &successor in &successors {
            assert!(
                dependency.is_met(vk::API_VERSION_1_0, &|name: &CStr| name == successor),
                "a dependency on {:?} is not met with {successor:?}, which it was promoted to",
                info.name
            );
        }
        if let Some(version) = core_version {
            assert!(
                dependency.is_met(version, &nothing_enabled),
                "a dependency on {:?} is not met by the version it was promoted to",
                info.name
            );
            assert!(
                !dependency.is_met(version - vk::make_api_version(0, 0, 1, 0), &nothing_enabled),
                "a dependency on {:?} is met by the version before the one it was promoted to",
                info.name
            );
        }
        assert!(
            !dependency.is_met(vk::API_VERSION_1_0, &nothing_enabled)
                || core_version == Some(vk::API_VERSION_1_0),
            "a dependency on {:?} is met with nothing enabled on Vulkan 1.0",
            info.name
        );
    }
    assert!(
        chains_longer_than_one_step >= 7,
        "the registry has chains such as VK_EXT_index_type_uint8 -> VK_KHR_index_type_uint8 -> Vulkan 1.4, found {chains_longer_than_one_step}"
    );
}

#[test]
fn extensions_have_the_dependencies_and_promotions_of_the_specification() {
    let nothing = |_: &CStr| false;
    let swapchain = ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME).unwrap();
    assert_eq!(swapchain.name, c"VK_KHR_swapchain");
    assert_eq!(swapchain.level, ExtensionLevel::Device);
    assert_eq!(swapchain.spec_version, vk::KHR_SWAPCHAIN_SPEC_VERSION);
    assert_eq!(
        swapchain.depends,
        Some(Dependency::Extension(c"VK_KHR_surface"))
    );
    assert!(!swapchain.dependencies_met(vk::API_VERSION_1_4, nothing));
    assert!(
        swapchain.dependencies_met(vk::API_VERSION_1_0, |name: &CStr| name
            == vk::KHR_SURFACE_NAME)
    );

    let surface = ExtensionInfo::get(vk::KHR_SURFACE_NAME).unwrap();
    assert_eq!(surface.level, ExtensionLevel::Instance);
    assert!(surface.dependencies_met(vk::API_VERSION_1_0, nothing));
    assert!(ExtensionInfo::get(c"VK_KHR_win32_surface").is_some());

    assert_eq!(
        ExtensionInfo::get(vk::KHR_MAINTENANCE_5_NAME)
            .unwrap()
            .promoted_to_version,
        Some(vk::API_VERSION_1_4)
    );
    let dynamic_rendering = Dependency::Extension(c"VK_KHR_dynamic_rendering");
    assert!(dynamic_rendering.is_met(vk::API_VERSION_1_3, &nothing));
    assert!(!dynamic_rendering.is_met(vk::API_VERSION_1_2, &nothing));
    let index_type_uint8 = Dependency::Extension(c"VK_EXT_index_type_uint8");
    assert!(
        index_type_uint8.is_met(vk::API_VERSION_1_0, &|name: &CStr| name
            == c"VK_KHR_index_type_uint8")
    );
    assert!(index_type_uint8.is_met(vk::API_VERSION_1_4, &nothing));
    assert!(!index_type_uint8.is_met(vk::API_VERSION_1_3, &nothing));

    let unknown = Dependency::Extension(c"VK_VENDOR_not_an_extension");
    assert!(!unknown.is_met(vk::API_VERSION_1_4, &nothing));
    assert!(unknown.is_met(vk::API_VERSION_1_0, &|name: &CStr| name
        == c"VK_VENDOR_not_an_extension"));
}

#[test]
fn commands_have_the_levels_and_result_codes_of_the_specification() {
    let entry: Vec<&str> = vk::COMMANDS
        .iter()
        .filter(|info| info.level == CommandLevel::Entry)
        .map(|info| info.name)
        .collect();
    assert_eq!(
        entry,
        [
            "vkCreateInstance",
            "vkEnumerateInstanceExtensionProperties",
            "vkEnumerateInstanceLayerProperties",
            "vkEnumerateInstanceVersion",
            "vkGetInstanceProcAddr",
        ]
    );
    for info in vk::COMMANDS {
        if info.name.starts_with("vkCmd") {
            assert_eq!(info.level, CommandLevel::Device, "{}", info.name);
        }
        assert!(
            info.success_codes.iter().all(|code| !code.is_error()),
            "{}: success codes are not negative",
            info.name
        );
        assert!(
            info.error_codes
                .iter()
                .all(|code| !info.success_codes.contains(code)),
            "{}",
            info.name
        );
    }

    let submit = vk::command::QUEUE_SUBMIT;
    assert_eq!(submit.name, "vkQueueSubmit");
    assert_eq!(submit.level, CommandLevel::Device);
    assert!(submit.returns_result());
    assert!(submit.can_return(VkResult::SUCCESS));
    assert!(submit.can_return(VkResult::ERROR_DEVICE_LOST));
    assert!(!submit.can_return(VkResult::ERROR_SURFACE_LOST_KHR));

    let draw = vk::command::CMD_DRAW;
    assert!(!draw.returns_result());
    assert!(!draw.can_return(VkResult::SUCCESS));

    let acquire = vk::command::ACQUIRE_NEXT_IMAGE_KHR;
    assert!(acquire.can_return(VkResult::SUBOPTIMAL_KHR));
    assert!(acquire.can_return(VkResult::ERROR_OUT_OF_DATE_KHR));
    assert_eq!(acquire.extensions, [vk::KHR_SWAPCHAIN_NAME]);
    assert_eq!(acquire.version, None);

    assert_eq!(
        vk::command::CMD_BEGIN_RENDERING.version,
        Some(vk::API_VERSION_1_3)
    );
    assert_eq!(vk::command::CREATE_DEVICE.level, CommandLevel::Instance);
    assert_eq!(
        vk::command::GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR.level,
        CommandLevel::Instance
    );
    assert!(CommandInfo::get("vkCreateWin32SurfaceKHR").is_some());
}
