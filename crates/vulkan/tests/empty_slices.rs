use proptest::prelude::*;
use std::panic::{AssertUnwindSafe, catch_unwind};
use vulkan::vk;

const MAX: usize = 3;

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

fn elements<'a, T>(start: *const T, len: usize, at_the_start: bool) -> &'a [T] {
    assert!(len <= MAX);
    if len == 0 && !at_the_start {
        &[]
    } else {
        unsafe { std::slice::from_raw_parts(start, len) }
    }
}

fn elements_mut<'a, T>(start: *mut T, len: usize, at_the_start: bool) -> &'a mut [T] {
    assert!(len <= MAX);
    if len == 0 && !at_the_start {
        &mut []
    } else {
        unsafe { std::slice::from_raw_parts_mut(start, len) }
    }
}

struct Array<S> {
    name: &'static str,
    set: Box<dyn Fn(S, usize, bool) -> S>,
    pointer: fn(&S) -> *const (),
    start: *const (),
    stored_length: Option<fn(&S) -> usize>,
    length_of_an_element: usize,
    shared_count: Option<&'static str>,
    absent_when_empty: bool,
}

impl<S> Array<S> {
    fn new<T>(
        name: &'static str,
        start: *const T,
        set: impl Fn(S, usize, bool) -> S + 'static,
        pointer: fn(&S) -> *const (),
        stored_length: fn(&S) -> usize,
    ) -> Self {
        Self {
            stored_length: Some(stored_length),
            ..Self::without_a_stored_length(name, start, set, pointer)
        }
    }

    fn without_a_stored_length<T>(
        name: &'static str,
        start: *const T,
        set: impl Fn(S, usize, bool) -> S + 'static,
        pointer: fn(&S) -> *const (),
    ) -> Self {
        Self {
            name,
            set: Box::new(set),
            pointer,
            start: start.cast(),
            stored_length: None,
            length_of_an_element: 1,
            shared_count: None,
            absent_when_empty: false,
        }
    }

    fn sharing(mut self, count: &'static str) -> Self {
        self.shared_count = Some(count);
        self
    }

    fn absent_when_empty(mut self) -> Self {
        self.absent_when_empty = true;
        self
    }

    fn counted_in(mut self, length_of_an_element: usize) -> Self {
        self.length_of_an_element = length_of_an_element;
        self
    }
}

type Call = (usize, usize, bool);

fn calls() -> impl Strategy<Value = Vec<Call>> {
    proptest::collection::vec((0..12_usize, 0..=MAX, any::<bool>()), 0..24)
}

fn check<S: Copy>(arrays: &[Array<S>], calls: &[Call], mut value: S) -> Result<(), TestCaseError> {
    let mut last = vec![None::<usize>; arrays.len()];
    for (step, &(which, len, at_the_start)) in calls.iter().enumerate() {
        let which = which % arrays.len();
        let array = &arrays[which];
        let another_length = arrays.iter().zip(&last).any(|(other, last)| {
            other.name != array.name
                && other.shared_count.is_some()
                && other.shared_count == array.shared_count
                && last.is_some_and(|last| last != 0 && last != len)
        });
        let refused = another_length && !(array.absent_when_empty && len == 0);
        let outcome = catch_unwind(AssertUnwindSafe(|| (array.set)(value, len, at_the_start)));
        prop_assert_eq!(
            outcome.is_err(),
            refused,
            "call {}: `{}` with {} elements",
            step,
            array.name,
            len
        );
        if let Ok(set) = outcome {
            value = set;
            last[which] = Some(len);
        }
        for (array, last) in arrays.iter().zip(&last) {
            let pointer = (array.pointer)(&value);
            let Some(len) = last.filter(|len| *len != 0) else {
                prop_assert!(
                    pointer.is_null(),
                    "call {}: the pointer of `{}`, whose last slice was {:?}, is not null",
                    step,
                    array.name,
                    last
                );
                continue;
            };
            prop_assert_eq!(
                pointer,
                array.start,
                "call {}: the pointer of `{}`",
                step,
                array.name
            );
            if let Some(stored_length) = array.stored_length {
                prop_assert_eq!(
                    stored_length(&value),
                    len * array.length_of_an_element,
                    "call {}: the length stored for `{}`",
                    step,
                    array.name
                );
            }
        }
    }
    Ok(())
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn required_arrays_with_a_count_of_their_own_or_a_shared_one(calls in calls()) {
        let semaphores = [vk::Semaphore::null(); MAX];
        let masks = [vk::PipelineStageFlags::empty(); MAX];
        let buffers = [vk::CommandBuffer::null(); MAX];
        let signals = [vk::Semaphore::null(); MAX];
        let (semaphores, masks) = (semaphores.as_ptr(), masks.as_ptr());
        let (buffers, signals) = (buffers.as_ptr(), signals.as_ptr());
        let arrays = [
            Array::new(
                "wait_semaphores",
                semaphores,
                move |info: vk::SubmitInfo<'static>, len, at_the_start| {
                    info.wait_semaphores(elements(semaphores, len, at_the_start))
                },
                |info| info.p_wait_semaphores.cast(),
                |info| info.wait_semaphore_count as usize,
            )
            .sharing("waitSemaphoreCount"),
            Array::new(
                "wait_dst_stage_mask",
                masks,
                move |info: vk::SubmitInfo<'static>, len, at_the_start| {
                    info.wait_dst_stage_mask(elements(masks, len, at_the_start))
                },
                |info| info.p_wait_dst_stage_mask.cast(),
                |info| info.wait_semaphore_count as usize,
            )
            .sharing("waitSemaphoreCount"),
            Array::new(
                "command_buffers",
                buffers,
                move |info: vk::SubmitInfo<'static>, len, at_the_start| {
                    info.command_buffers(elements(buffers, len, at_the_start))
                },
                |info| info.p_command_buffers.cast(),
                |info| info.command_buffer_count as usize,
            ),
            Array::new(
                "signal_semaphores",
                signals,
                move |info: vk::SubmitInfo<'static>, len, at_the_start| {
                    info.signal_semaphores(elements(signals, len, at_the_start))
                },
                |info| info.p_signal_semaphores.cast(),
                |info| info.signal_semaphore_count as usize,
            ),
        ];
        check(&arrays, &calls, vk::SubmitInfo::default())?;
    }

    #[test]
    fn a_required_array_that_shares_its_count_with_an_optional_one(calls in calls()) {
        let inputs = [vk::AttachmentReference::default(); MAX];
        let colors = [vk::AttachmentReference::default(); MAX];
        let resolves = [vk::AttachmentReference::default(); MAX];
        let preserved = [0_u32; MAX];
        let (inputs, colors) = (inputs.as_ptr(), colors.as_ptr());
        let (resolves, preserved) = (resolves.as_ptr(), preserved.as_ptr());
        let arrays = [
            Array::new(
                "input_attachments",
                inputs,
                move |subpass: vk::SubpassDescription<'static>, len, at_the_start| {
                    subpass.input_attachments(elements(inputs, len, at_the_start))
                },
                |subpass| subpass.p_input_attachments.cast(),
                |subpass| subpass.input_attachment_count as usize,
            ),
            Array::new(
                "color_attachments",
                colors,
                move |subpass: vk::SubpassDescription<'static>, len, at_the_start| {
                    subpass.color_attachments(elements(colors, len, at_the_start))
                },
                |subpass| subpass.p_color_attachments.cast(),
                |subpass| subpass.color_attachment_count as usize,
            )
            .sharing("colorAttachmentCount"),
            Array::new(
                "resolve_attachments",
                resolves,
                move |subpass: vk::SubpassDescription<'static>, len, at_the_start| {
                    subpass.resolve_attachments(elements(resolves, len, at_the_start))
                },
                |subpass| subpass.p_resolve_attachments.cast(),
                |subpass| subpass.color_attachment_count as usize,
            )
            .sharing("colorAttachmentCount")
            .absent_when_empty(),
            Array::new(
                "preserve_attachments",
                preserved,
                move |subpass: vk::SubpassDescription<'static>, len, at_the_start| {
                    subpass.preserve_attachments(elements(preserved, len, at_the_start))
                },
                |subpass| subpass.p_preserve_attachments.cast(),
                |subpass| subpass.preserve_attachment_count as usize,
            ),
        ];
        check(&arrays, &calls, vk::SubpassDescription::default())?;
    }

    #[test]
    fn bytes_values_and_words_behind_a_pointer_to_void_or_a_size_in_bytes(calls in calls()) {
        let entries = [vk::SpecializationMapEntry::default(); MAX];
        let bytes = [0_u8; MAX];
        let (entries, bytes) = (entries.as_ptr(), bytes.as_ptr());
        let arrays = [
            Array::new(
                "map_entries",
                entries,
                move |info: vk::SpecializationInfo<'static>, len, at_the_start| {
                    info.map_entries(elements(entries, len, at_the_start))
                },
                |info| info.p_map_entries.cast(),
                |info| info.map_entry_count as usize,
            ),
            Array::new(
                "data",
                bytes,
                move |info: vk::SpecializationInfo<'static>, len, at_the_start| {
                    info.data(elements(bytes, len, at_the_start))
                },
                |info| info.p_data.cast(),
                |info| info.data_size,
            ),
        ];
        check(&arrays, &calls, vk::SpecializationInfo::default())?;

        let values = [0_u64; MAX];
        let values = values.as_ptr();
        let arrays = [Array::new(
            "values",
            values,
            move |setting: vk::LayerSettingEXT<'static>, len, at_the_start| {
                setting.values(elements(values, len, at_the_start))
            },
            |setting| setting.p_values.cast(),
            |setting| setting.value_count as usize,
        )];
        check(&arrays, &calls, vk::LayerSettingEXT::default())?;

        let words = [0_u32; MAX];
        let words = words.as_ptr();
        let arrays = [Array::new(
            "code",
            words,
            move |info: vk::ShaderModuleCreateInfo<'static>, len, at_the_start| {
                info.code(elements(words, len, at_the_start))
            },
            |info| info.p_code.cast(),
            |info| info.code_size,
        )
        .counted_in(size_of::<u32>())];
        check(&arrays, &calls, vk::ShaderModuleCreateInfo::default())?;
    }

    #[test]
    fn arrays_that_the_implementation_only_reads_through_a_writable_pointer(calls in calls()) {
        let bytes = [0_u8; MAX];
        let bytes = bytes.as_ptr();
        let arrays = [Array::new(
            "data",
            bytes,
            move |data: vk::PipelineBinaryDataKHR<'static>, len, at_the_start| {
                data.data(elements(bytes, len, at_the_start))
            },
            |data| data.p_data.cast_const().cast(),
            |data| data.data_size,
        )];
        check(&arrays, &calls, vk::PipelineBinaryDataKHR::default())?;

        let engines = [vk::PhysicalDeviceDataGraphProcessingEngineARM::default(); MAX];
        let engines = engines.as_ptr();
        let arrays = [Array::new(
            "processing_engines",
            engines,
            move |info: vk::DataGraphProcessingEngineCreateInfoARM<'static>, len, at_the_start| {
                info.processing_engines(elements(engines, len, at_the_start))
            },
            |info| info.p_processing_engines.cast_const().cast(),
            |info| info.processing_engine_count as usize,
        )];
        check(
            &arrays,
            &calls,
            vk::DataGraphProcessingEngineCreateInfoARM::default(),
        )?;
    }

    #[test]
    fn arrays_of_pointers_to_strings_and_to_structures(calls in calls()) {
        let layers = [vk::CStrPtr::new(c"layer"); MAX];
        let extensions = [vk::CStrPtr::new(c"extension"); MAX];
        let (layers, extensions) = (layers.as_ptr(), extensions.as_ptr());
        let arrays = [
            Array::new(
                "enabled_layer_names",
                layers,
                move |info: vk::InstanceCreateInfo<'static>, len, at_the_start| {
                    info.enabled_layer_names(elements(layers, len, at_the_start))
                },
                |info| info.pp_enabled_layer_names.cast(),
                |info| info.enabled_layer_count as usize,
            ),
            Array::new(
                "enabled_extension_names",
                extensions,
                move |info: vk::InstanceCreateInfo<'static>, len, at_the_start| {
                    info.enabled_extension_names(elements(extensions, len, at_the_start))
                },
                |info| info.pp_enabled_extension_names.cast(),
                |info| info.enabled_extension_count as usize,
            ),
        ];
        check(&arrays, &calls, vk::InstanceCreateInfo::default())?;

        static USAGE: vk::MicromapUsageEXT = vk::MicromapUsageEXT {
            count: 0,
            subdivision_level: 0,
            format: 0,
        };
        let usages = [USAGE; MAX];
        let references = [&USAGE; MAX];
        let (usages, references) = (usages.as_ptr(), references.as_ptr());
        type Micromap = vk::AccelerationStructureTrianglesOpacityMicromapEXT<'static>;
        let arrays = [
            Array::new(
                "usage_counts",
                usages,
                move |micromap: Micromap, len, at_the_start| {
                    micromap.usage_counts(elements(usages, len, at_the_start))
                },
                |micromap| micromap.p_usage_counts.cast(),
                |micromap| micromap.usage_counts_count as usize,
            )
            .sharing("usageCountsCount")
            .absent_when_empty(),
            Array::new(
                "usage_counts_ptrs",
                references,
                move |micromap: Micromap, len, at_the_start| {
                    micromap.usage_counts_ptrs(elements(references, len, at_the_start))
                },
                |micromap| micromap.pp_usage_counts.cast(),
                |micromap| micromap.usage_counts_count as usize,
            )
            .sharing("usageCountsCount")
            .absent_when_empty(),
        ];
        check(&arrays, &calls, Micromap::default())?;
    }

    #[test]
    fn arrays_that_the_implementation_writes(calls in calls()) {
        let mut feedbacks = [vk::PipelineCreationFeedback::default(); MAX];
        let feedbacks = feedbacks.as_mut_ptr();
        let arrays = [Array::new(
            "pipeline_stage_creation_feedbacks",
            feedbacks.cast_const(),
            move |info: vk::PipelineCreationFeedbackCreateInfo<'static>, len, at_the_start| {
                info.pipeline_stage_creation_feedbacks(elements_mut(feedbacks, len, at_the_start))
            },
            |info| info.p_pipeline_stage_creation_feedbacks.cast_const().cast(),
            |info| info.pipeline_stage_creation_feedback_count as usize,
        )];
        check(
            &arrays,
            &calls,
            vk::PipelineCreationFeedbackCreateInfo::default(),
        )?;

        let mut bytes = [0_u8; MAX];
        let bytes = bytes.as_mut_ptr();
        let arrays = [Array::new(
            "address",
            bytes.cast_const(),
            move |range: vk::HostAddressRangeEXT<'static>, len, at_the_start| {
                range.address(elements_mut(bytes, len, at_the_start))
            },
            |range| range.address.cast_const().cast(),
            |range| range.size,
        )];
        check(&arrays, &calls, vk::HostAddressRangeEXT::default())?;

        let mut timings = [vk::LatencyTimingsFrameReportNV::default(); MAX];
        let timings = timings.as_mut_ptr();
        let arrays = [Array::new(
            "timings",
            timings.cast_const(),
            move |info: vk::GetLatencyMarkerInfoNV<'static>, len, at_the_start| {
                info.timings(elements_mut(timings, len, at_the_start))
            },
            |info| info.p_timings.cast_const().cast(),
            |info| info.timing_count as usize,
        )
        .absent_when_empty()];
        check(&arrays, &calls, vk::GetLatencyMarkerInfoNV::default())?;

        let mut addresses = [vk::DeviceFaultAddressInfoKHR::default(); MAX];
        let mut vendors = [vk::DeviceFaultVendorInfoKHR::default(); MAX];
        let (addresses, vendors) = (addresses.as_mut_ptr(), vendors.as_mut_ptr());
        let arrays = [
            Array::without_a_stored_length(
                "address_infos",
                addresses.cast_const(),
                move |info: vk::DeviceFaultInfoEXT<'static>, len, at_the_start| {
                    info.address_infos(elements_mut(addresses, len, at_the_start))
                },
                |info| info.p_address_infos.cast_const().cast(),
            ),
            Array::without_a_stored_length(
                "vendor_infos",
                vendors.cast_const(),
                move |info: vk::DeviceFaultInfoEXT<'static>, len, at_the_start| {
                    info.vendor_infos(elements_mut(vendors, len, at_the_start))
                },
                |info| info.p_vendor_infos.cast_const().cast(),
            ),
        ];
        check(&arrays, &calls, vk::DeviceFaultInfoEXT::default())?;
    }
}
