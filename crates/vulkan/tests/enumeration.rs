#![cfg(feature = "std")]

mod fake_driver;

use fake_driver::{
    Call, DEVICE, Enumeration, INSTANCE, Reply, Script, byte, cases, extension_name, fakes,
};
use proptest::collection::vec;
use proptest::prelude::*;
use proptest::sample::select;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use std::collections::BTreeMap;
use std::ffi::CStr;
use vulkan::vk::{self, Handle, VkResult};

const PHYSICAL_DEVICE: u64 = 0x9D;
const PIPELINE_CACHE: u64 = 0xCAC4_E000_0000_0001;
const MOST_ELEMENTS: u32 = 64;
const LAYERS: [Option<&CStr>; 3] = [None, Some(c""), Some(c"VK_LAYER_FAKE_validation")];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Listing {
    PhysicalDevices,
    PipelineCacheData,
    DeviceExtensions,
    InstanceExtensions,
    QueueFamilies,
    QueueFamilies2,
}

const RESULT_LISTINGS: [Listing; 4] = [
    Listing::PhysicalDevices,
    Listing::PipelineCacheData,
    Listing::DeviceExtensions,
    Listing::InstanceExtensions,
];

const VOID_LISTINGS: [Listing; 2] = [Listing::QueueFamilies, Listing::QueueFamilies2];

impl Listing {
    fn info(self) -> &'static vk::CommandInfo {
        match self {
            Listing::PhysicalDevices => &vk::command::ENUMERATE_PHYSICAL_DEVICES,
            Listing::PipelineCacheData => &vk::command::GET_PIPELINE_CACHE_DATA,
            Listing::DeviceExtensions => &vk::command::ENUMERATE_DEVICE_EXTENSION_PROPERTIES,
            Listing::InstanceExtensions => &vk::command::ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
            Listing::QueueFamilies => &vk::command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES,
            Listing::QueueFamilies2 => &vk::command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2,
        }
    }

    fn handles(self) -> Vec<u64> {
        match self {
            Listing::PhysicalDevices => vec![INSTANCE],
            Listing::PipelineCacheData => vec![DEVICE, PIPELINE_CACHE],
            Listing::DeviceExtensions | Listing::QueueFamilies | Listing::QueueFamilies2 => {
                vec![PHYSICAL_DEVICE]
            }
            Listing::InstanceExtensions => Vec::new(),
        }
    }

    fn element(self, id: u64) -> u64 {
        match self {
            Listing::PipelineCacheData => u64::from(byte(id)),
            _ => id,
        }
    }
}

fn extension_id(properties: &vk::ExtensionProperties) -> u64 {
    let id = u64::from(properties.spec_version);
    if properties.extension_name == extension_name(id) {
        id
    } else {
        u64::MAX
    }
}

fn queue_family2_id(properties: &vk::QueueFamilyProperties2<'_>) -> u64 {
    if properties.s_type == vk::StructureType::QUEUE_FAMILY_PROPERTIES_2
        && properties.p_next.is_null()
    {
        u64::from(properties.queue_family_properties.queue_count)
    } else {
        u64::MAX
    }
}

fn list(
    listing: Listing,
    layer: Option<&CStr>,
    replies: Vec<Reply>,
) -> (Result<Vec<u64>, VkResult>, Vec<Call>) {
    let fakes = fakes();
    let physical_device = vk::PhysicalDevice::from_raw_u64(PHYSICAL_DEVICE);
    let (result, calls) = fake_driver::run(
        Script {
            replies,
            ..Script::default()
        },
        || unsafe {
            match listing {
                Listing::PhysicalDevices => fakes
                    .instance
                    .enumerate_physical_devices()
                    .map(|devices| devices.iter().map(|device| device.as_raw_u64()).collect()),
                Listing::PipelineCacheData => fakes
                    .device
                    .get_pipeline_cache_data(vk::PipelineCache::from_raw_u64(PIPELINE_CACHE))
                    .map(|data| data.into_iter().map(u64::from).collect()),
                Listing::DeviceExtensions => fakes
                    .instance
                    .enumerate_device_extension_properties(physical_device, layer)
                    .map(|properties| properties.iter().map(extension_id).collect()),
                Listing::InstanceExtensions => fakes
                    .entry
                    .enumerate_instance_extension_properties(layer)
                    .map(|properties| properties.iter().map(extension_id).collect()),
                Listing::QueueFamilies => Ok(fakes
                    .instance
                    .get_physical_device_queue_family_properties(physical_device)
                    .iter()
                    .map(|properties| u64::from(properties.queue_count))
                    .collect()),
                Listing::QueueFamilies2 => Ok(fakes
                    .instance
                    .get_physical_device_queue_family_properties2(physical_device)
                    .iter()
                    .map(queue_family2_id)
                    .collect()),
            }
        },
    );
    let result = result.unwrap_or_else(|message| panic!("{listing:?} panicked: {message}"));
    (result, calls)
}

fn enumerations(calls: &[Call]) -> Vec<&Enumeration> {
    calls
        .iter()
        .map(|call| {
            call.enumeration
                .as_ref()
                .expect("only the enumeration command is called")
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Same,
    Grow(u32),
    Shrink(u32),
    Jump(u32),
    Fail(VkResult),
    OverReport(u32),
    IncompleteWithoutWriting,
}

fn available() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0),
        Just(1),
        2..=4u32,
        0..=MOST_ELEMENTS,
        Just(MOST_ELEMENTS - 1),
        Just(MOST_ELEMENTS),
    ]
}

fn error_code() -> impl Strategy<Value = VkResult> {
    let listed: Vec<VkResult> = RESULT_LISTINGS
        .iter()
        .flat_map(|listing| listing.info().error_codes)
        .copied()
        .filter(|code| code.is_error())
        .collect();
    prop_oneof![
        4 => select(listed),
        1 => Just(VkResult::from_raw(i32::MIN)),
        1 => (i32::MIN..0).prop_map(VkResult::from_raw),
    ]
}

fn conforming_step() -> BoxedStrategy<Step> {
    prop_oneof![
        6 => Just(Step::Same),
        2 => (1..=4u32).prop_map(Step::Grow),
        2 => (1..=4u32).prop_map(Step::Shrink),
        1 => available().prop_map(Step::Jump),
    ]
    .boxed()
}

fn failing_step() -> BoxedStrategy<Step> {
    prop_oneof![11 => conforming_step(), 1 => error_code().prop_map(Step::Fail)].boxed()
}

fn rule_breaking_step() -> BoxedStrategy<Step> {
    prop_oneof![
        10 => failing_step(),
        2 => (1..=8u32).prop_map(Step::OverReport),
        1 => Just(Step::IncompleteWithoutWriting),
    ]
    .boxed()
}

fn script(
    step: BoxedStrategy<Step>,
    calls: std::ops::RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<Reply>> {
    (available(), vec(step, calls)).prop_map(|(start, steps)| {
        let mut available = start;
        steps
            .into_iter()
            .map(|step| {
                available = match step {
                    Step::Same => available,
                    Step::Grow(more) => (available + more).min(MOST_ELEMENTS),
                    Step::Shrink(fewer) => available.saturating_sub(fewer),
                    Step::Jump(count) => count,
                    Step::Fail(code) => return Reply::Fail(code),
                    Step::OverReport(extra) => return Reply::OverReport(extra),
                    Step::IncompleteWithoutWriting => return Reply::IncompleteWithoutWriting,
                };
                Reply::Conforming(available)
            })
            .collect()
    })
}

fn conforming_script() -> impl Strategy<Value = Vec<Reply>> {
    script(failing_step(), 1..=10)
}

fn rule_breaking_script() -> impl Strategy<Value = Vec<Reply>> {
    script(rule_breaking_step(), 1..=10)
}

fn void_script() -> impl Strategy<Value = Vec<Reply>> {
    script(
        prop_oneof![4 => conforming_step(), 1 => (1..=8u32).prop_map(Step::OverReport)].boxed(),
        2..=2,
    )
}

fn check_result_enumeration(
    listing: Listing,
    replies: &[Reply],
    result: &Result<Vec<u64>, VkResult>,
    calls: &[Call],
    driver_follows_the_rules: bool,
) -> Result<(), TestCaseError> {
    for call in calls {
        prop_assert_eq!(call.command, listing.info().name);
        prop_assert_eq!(
            &call.handles,
            &listing.handles(),
            "the handles are passed as given"
        );
    }
    let calls = enumerations(calls);
    prop_assert!(!calls.is_empty(), "the wrapper calls the command");
    prop_assert!(
        calls.len() <= replies.len() + 1,
        "the wrapper stops at the error the driver returns once it has no more replies"
    );
    for (index, call) in calls.iter().enumerate() {
        if index % 2 == 0 {
            prop_assert!(
                !call.array,
                "call {} asks for the count with no array",
                index
            );
        } else {
            prop_assert!(call.array, "call {} passes an array", index);
            prop_assert_eq!(
                call.capacity,
                calls[index - 1].count,
                "call {} passes the count that the query before it returned",
                index
            );
        }
    }
    let (last, earlier) = calls.split_last().expect("at least one call");
    for (index, call) in earlier.iter().enumerate() {
        prop_assert!(
            !call.result.is_error(),
            "the wrapper returns at the first error, but call {} returned {:?}",
            index,
            call.result
        );
        if index % 2 == 1 {
            prop_assert_eq!(
                call.result,
                VkResult::INCOMPLETE,
                "the wrapper asks again only after VK_INCOMPLETE"
            );
        }
    }
    if last.result.is_error() {
        prop_assert_eq!(
            result,
            &Err(last.result),
            "the error of the driver is returned"
        );
        return Ok(());
    }
    if !last.array {
        prop_assert_eq!(
            last.count,
            0,
            "the wrapper returns after a query only if it reported no elements"
        );
        prop_assert_eq!(result, &Ok(Vec::new()));
        return Ok(());
    }
    prop_assert_ne!(
        last.result,
        VkResult::INCOMPLETE,
        "the wrapper does not return after VK_INCOMPLETE"
    );
    let returned = match result {
        Ok(returned) => returned,
        Err(code) => {
            return Err(TestCaseError::fail(format!(
                "the driver returned {:?}, but the wrapper returned Err({code:?})",
                last.result
            )));
        }
    };
    let written: Vec<u64> = last.written.iter().map(|&id| listing.element(id)).collect();
    if driver_follows_the_rules {
        prop_assert_eq!(
            returned,
            &written,
            "the wrapper returns the elements that the driver wrote in its last call"
        );
    } else {
        prop_assert!(
            returned.len() as u64 <= last.capacity,
            "{} elements are returned from an array of {}",
            returned.len(),
            last.capacity
        );
        let shared = returned.len().min(written.len());
        prop_assert_eq!(&returned[..shared], &written[..shared]);
    }
    Ok(())
}

fn check_void_enumeration(
    listing: Listing,
    returned: &[u64],
    calls: &[Call],
) -> Result<(), TestCaseError> {
    for call in calls {
        prop_assert_eq!(call.command, listing.info().name);
        prop_assert_eq!(&call.handles, &listing.handles());
    }
    let calls = enumerations(calls);
    prop_assert_eq!(calls.len(), 2, "a command without a result is called twice");
    let (query, fill) = (calls[0], calls[1]);
    prop_assert!(
        !query.array,
        "the first call asks for the count with no array"
    );
    prop_assert!(fill.array, "the second call passes an array");
    prop_assert_eq!(
        fill.capacity,
        query.count,
        "the array holds the queried count"
    );
    prop_assert_eq!(
        returned.len() as u64,
        fill.capacity.min(fill.count),
        "the length is the count the driver wrote, at most the capacity"
    );
    let written: Vec<u64> = fill.written.iter().map(|&id| listing.element(id)).collect();
    prop_assert_eq!(returned, &written[..]);
    if listing == Listing::QueueFamilies2 {
        prop_assert_eq!(fill.structures.len() as u64, fill.capacity);
        for &(s_type, p_next_is_null) in &fill.structures {
            prop_assert_eq!(
                s_type,
                vk::StructureType::QUEUE_FAMILY_PROPERTIES_2.as_raw(),
                "every element passed to the driver has its sType"
            );
            prop_assert!(
                p_next_is_null,
                "every element passed to the driver has no pNext"
            );
        }
    }
    Ok(())
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn an_enumeration_returns_the_elements_of_the_last_call_of_a_driver_that_follows_the_rules(
        listing in select(RESULT_LISTINGS.to_vec()),
        replies in conforming_script(),
    ) {
        let (result, calls) = list(listing, None, replies.clone());
        check_result_enumeration(listing, &replies, &result, &calls, true)?;
    }

    #[test]
    fn an_enumeration_never_returns_more_elements_than_its_array_held(
        listing in select(RESULT_LISTINGS.to_vec()),
        replies in rule_breaking_script(),
    ) {
        let (result, calls) = list(listing, None, replies.clone());
        check_result_enumeration(listing, &replies, &result, &calls, false)?;
    }

    #[test]
    fn an_extension_enumeration_passes_the_layer_name_on_every_call(
        listing in select(vec![Listing::DeviceExtensions, Listing::InstanceExtensions]),
        layer in select(LAYERS.to_vec()),
        replies in conforming_script(),
    ) {
        let (result, calls) = list(listing, layer, replies.clone());
        check_result_enumeration(listing, &replies, &result, &calls, true)?;
        let expected = layer.map(|layer| layer.to_bytes().to_vec());
        for call in &calls {
            prop_assert_eq!(&call.strings, &vec![expected.clone()]);
        }
    }

    #[test]
    fn an_enumeration_without_a_result_returns_what_the_driver_wrote_into_the_array(
        listing in select(VOID_LISTINGS.to_vec()),
        replies in void_script(),
    ) {
        let (result, calls) = list(listing, None, replies);
        let returned = result.expect("an enumeration without a result has no error");
        check_void_enumeration(listing, &returned, &calls)?;
    }
}

fn record_cases(
    cases: &mut BTreeMap<&'static str, u32>,
    result: &Result<Vec<u64>, VkResult>,
    calls: &[Call],
) {
    let calls = enumerations(calls);
    let mut reached = Vec::new();
    let fills = || calls.iter().skip(1).step_by(2);
    let available = |call: &&Enumeration| match call.reply {
        Some(Reply::Conforming(available)) => Some(u64::from(available)),
        _ => None,
    };
    if result.is_ok() && calls.len() == 2 {
        reached.push("one pair of calls");
    }
    if fills().any(|call| call.result == VkResult::INCOMPLETE) {
        reached.push("asked again after VK_INCOMPLETE");
    }
    if fills().any(|call| available(call).is_some_and(|available| available > call.capacity)) {
        reached.push("more elements than queried");
    }
    if fills().any(|call| available(call).is_some_and(|available| available < call.capacity)) {
        reached.push("fewer elements than queried");
    }
    if result.as_ref().is_ok_and(|elements| elements.is_empty()) {
        reached.push("no elements");
    }
    if result
        .as_ref()
        .is_ok_and(|elements| elements.len() == MOST_ELEMENTS as usize)
    {
        reached.push("the most elements");
    }
    if result.is_err() {
        if calls.len() % 2 == 1 {
            reached.push("error on a query");
        } else {
            reached.push("error on a fill");
        }
        if calls.len() > 2 {
            reached.push("error after asking again");
        }
    }
    for case in reached {
        *cases.entry(case).or_default() += 1;
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn the_generated_driver_scripts_reach_every_case_of_the_enumeration_rules() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = (select(RESULT_LISTINGS.to_vec()), conforming_script());
    let mut cases = BTreeMap::new();
    let mut listings = BTreeMap::new();
    let samples = 512;
    for _ in 0..samples {
        let (listing, replies) = strategy.new_tree(&mut runner).unwrap().current();
        let (result, calls) = list(listing, None, replies);
        record_cases(&mut cases, &result, &calls);
        *listings.entry(format!("{listing:?}")).or_insert(0) += 1;
    }
    for (case, count) in &cases {
        println!("{count:>5}  {case}");
    }
    for case in [
        "one pair of calls",
        "asked again after VK_INCOMPLETE",
        "more elements than queried",
        "fewer elements than queried",
        "no elements",
        "the most elements",
        "error on a query",
        "error on a fill",
        "error after asking again",
    ] {
        let count = cases.get(case).copied().unwrap_or(0);
        assert!(
            count >= samples / 40,
            "{case}: reached by {count} of {samples} scripts"
        );
    }
    for listing in RESULT_LISTINGS {
        let count = listings.get(&format!("{listing:?}")).copied().unwrap_or(0);
        assert!(
            count >= samples / 8,
            "{listing:?}: {count} of {samples} scripts"
        );
    }
}
