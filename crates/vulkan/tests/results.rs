use proptest::prelude::*;
use proptest::strategy::ValueTree;
use proptest::test_runner::{RngAlgorithm, TestRng, TestRunner};
use vulkan::vk::VkResult;

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

fn check_code(code: VkResult) -> Result<(), TestCaseError> {
    let raw = code.as_raw();
    prop_assert_eq!(code.is_error(), raw < 0, "is_error of {}", raw);
    let expected = if raw < 0 { Err(code) } else { Ok(()) };
    prop_assert_eq!(code.into_result(), expected, "into_result of {}", raw);
    let listed = VkResult::variants()
        .iter()
        .find(|(value, _)| *value == code)
        .map(|(_, name)| *name);
    prop_assert_eq!(code.name(), listed, "name of {}", raw);
    let shown = match listed {
        Some(name) => name.to_string(),
        None => format!("VkResult({raw})"),
    };
    prop_assert_eq!(format!("{code:?}"), shown.clone());
    prop_assert_eq!(code.to_string(), shown);
    Ok(())
}

fn any_code() -> impl Strategy<Value = VkResult> {
    let listed: Vec<i32> = VkResult::variants()
        .iter()
        .map(|(value, _)| value.as_raw())
        .collect();
    prop_oneof![
        prop::sample::select(vec![i32::MIN, -2, -1, 0, 1, 2, i32::MAX]),
        prop::sample::select(listed.clone()),
        prop::sample::select(listed).prop_map(|raw| raw.saturating_add(1)),
        any::<i32>(),
    ]
    .prop_map(VkResult::from_raw)
}

#[test]
fn every_listed_code_is_an_error_exactly_when_it_is_negative() {
    for &(code, name) in VkResult::variants() {
        check_code(code).unwrap_or_else(|e| panic!("{name}: {e}"));
    }
    assert!(
        VkResult::variants()
            .windows(2)
            .all(|pair| pair[0].0 < pair[1].0)
    );
}

proptest! {
    #![proptest_config(cases(4096))]

    #[test]
    fn every_code_is_an_error_exactly_when_it_is_negative(code in any_code()) {
        check_code(code)?;
    }
}

#[test]
fn codes_have_the_values_of_the_c_header() {
    let header = [
        (VkResult::SUCCESS, 0),
        (VkResult::NOT_READY, 1),
        (VkResult::TIMEOUT, 2),
        (VkResult::INCOMPLETE, 5),
        (VkResult::ERROR_OUT_OF_HOST_MEMORY, -1),
        (VkResult::ERROR_DEVICE_LOST, -4),
        (VkResult::ERROR_UNKNOWN, -13),
        (VkResult::SUBOPTIMAL_KHR, 1_000_001_003),
        (VkResult::ERROR_OUT_OF_DATE_KHR, -1_000_001_004),
    ];
    for (code, value) in header {
        assert_eq!(code.as_raw(), value, "{code}");
    }
    assert_eq!(VkResult::SUCCESS.into_result(), Ok(()));
    assert_eq!(VkResult::SUBOPTIMAL_KHR.into_result(), Ok(()));
    assert_eq!(
        VkResult::ERROR_DEVICE_LOST.into_result(),
        Err(VkResult::ERROR_DEVICE_LOST)
    );
}

#[test]
fn a_code_converts_into_a_boxed_error_with_question_mark() {
    fn fails() -> Result<(), Box<dyn std::error::Error>> {
        VkResult::ERROR_DEVICE_LOST.into_result()?;
        Ok(())
    }
    let error = fails().unwrap_err();
    assert_eq!(error.to_string(), "ERROR_DEVICE_LOST");
    assert_eq!(
        error.downcast_ref::<VkResult>(),
        Some(&VkResult::ERROR_DEVICE_LOST)
    );
}

#[test]
fn generated_codes_cover_errors_successes_and_unnamed_values() {
    let mut runner = TestRunner::new_with_rng(
        ProptestConfig::default(),
        TestRng::deterministic_rng(RngAlgorithm::ChaCha),
    );
    let strategy = any_code();
    let mut counts = [0u32; 5];
    for _ in 0..2000 {
        let code = strategy.new_tree(&mut runner).unwrap().current();
        let kind = match (code.as_raw(), code.name()) {
            (0, _) => 0,
            (raw, Some(_)) if raw < 0 => 1,
            (_, Some(_)) => 2,
            (raw, None) if raw < 0 => 3,
            (_, None) => 4,
        };
        counts[kind] += 1;
    }
    assert!(
        counts.iter().all(|&count| count >= 40),
        "[SUCCESS, named errors, named successes, unnamed negative, unnamed non-negative] = {counts:?}"
    );
}

#[cfg(feature = "std")]
fn load_error(path: &std::path::Path) -> vulkan::LoadError {
    match unsafe { vulkan::Entry::load_from(path) } {
        Ok(_) => panic!("{} loaded as a Vulkan loader", path.display()),
        Err(error) => error,
    }
}

#[cfg(feature = "std")]
fn assert_names_its_cause(error: &vulkan::LoadError) -> String {
    use std::error::Error;
    let message = error.to_string();
    let source = error
        .source()
        .expect("a load error has the error of the dynamic library loader as its source")
        .to_string();
    assert!(!source.is_empty());
    assert!(
        message.starts_with("failed to load the Vulkan loader") && message.contains(&source),
        "{message:?} does not name the cause {source:?}"
    );
    message
}

#[cfg(feature = "std")]
#[test]
fn loading_a_missing_library_reports_the_error_of_the_library_loader() {
    let path = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("no-such-vulkan-loader");
    assert!(!path.exists());
    let message = assert_names_its_cause(&load_error(&path));
    if cfg!(target_os = "linux") {
        assert!(message.contains("no-such-vulkan-loader"), "{message}");
    }
}

#[cfg(all(feature = "std", target_os = "linux"))]
#[test]
fn loading_a_library_without_vk_get_instance_proc_addr_reports_the_missing_symbol() {
    let message = assert_names_its_cause(&load_error(std::path::Path::new("libm.so.6")));
    assert!(message.contains("vkGetInstanceProcAddr"), "{message}");
}
