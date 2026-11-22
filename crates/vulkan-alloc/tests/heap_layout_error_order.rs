use std::collections::BTreeMap;
use vulkan::vk;
use vulkan_alloc::descriptor_heap::{
    DescriptorHeapLayout, DescriptorSlots, HeapLayoutError, ResourceHeapPlan, SamplerHeapPlan,
};

const SIZES: [u64; 9] = [0, 1, 3, 16, 24, 1 << 32, 3 << 61, 1 << 63, u64::MAX];
const ALIGNMENTS: [u64; 8] = [0, 1, 2, 3, 16, 1 << 32, 1 << 62, 1 << 63];
const COUNTS: [u32; 5] = [0, 1, 2, 3, u32::MAX];
const RESERVED_SIZES: [u64; 3] = [0, 1, u64::MAX];
const MAX_SIZES: [u64; 3] = [0, 1 << 20, u64::MAX];

fn lcm(a: u64, b: u64) -> u128 {
    fn gcd(mut a: u128, mut b: u128) -> u128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a
    }
    let (a, b) = (u128::from(a.max(1)), u128::from(b.max(1)));
    a / gcd(a, b) * b
}

fn fits_in_u64(value: u128) -> bool {
    value <= u128::from(u64::MAX)
}

fn placement_overflows(cursor: u64, stride: u64, count: u32) -> bool {
    let stride = u128::from(stride);
    let first = u128::from(cursor).div_ceil(stride);
    let last = first + u128::from(count.saturating_sub(1));
    let end = (first + u128::from(count)).checked_mul(stride);
    first > u128::from(u32::MAX)
        || (count > 0 && last > u128::from(u32::MAX))
        || end.is_none_or(|end| !fits_in_u64(end))
}

fn typed_run_errors(cursor: u64, size: u64, alignment: u64, count: u32) -> Vec<HeapLayoutError> {
    if size == 0 {
        return vec![HeapLayoutError::ZeroDescriptorSize];
    }
    let mut errors = Vec::new();
    let alignment = alignment.max(1);
    if !size.is_multiple_of(alignment) {
        errors.push(HeapLayoutError::UnalignedDescriptorSize { size, alignment });
    }
    if placement_overflows(cursor, size, count) {
        errors.push(HeapLayoutError::Overflow);
    }
    errors
}

#[derive(Clone, Copy, Debug)]
struct Resource {
    buffer_size: u64,
    buffer_alignment: u64,
    image_size: u64,
    image_alignment: u64,
}

impl Resource {
    fn properties(
        self,
        reserved_size: u64,
        max_size: u64,
    ) -> vk::PhysicalDeviceDescriptorHeapPropertiesEXT<'static> {
        vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
            .buffer_descriptor_size(self.buffer_size)
            .buffer_descriptor_alignment(self.buffer_alignment)
            .image_descriptor_size(self.image_size)
            .image_descriptor_alignment(self.image_alignment)
            .min_resource_heap_reserved_range(reserved_size)
            .max_resource_heap_size(max_size)
    }

    fn uniform_stride(self) -> u128 {
        let alignment = lcm(self.buffer_alignment, self.image_alignment);
        let size = u128::from(self.buffer_size.max(self.image_size));
        size.div_ceil(alignment) * alignment
    }

    fn uniform_errors(self, cursor: u64, count: u32) -> Vec<HeapLayoutError> {
        let mut errors = Vec::new();
        if self.buffer_size == 0 || self.image_size == 0 {
            errors.push(HeapLayoutError::ZeroDescriptorSize);
        }
        let alignment = lcm(self.buffer_alignment, self.image_alignment);
        let overflows = match u64::try_from(self.uniform_stride()) {
            Ok(0) => false,
            Ok(stride) => placement_overflows(cursor, stride, count),
            Err(_) => true,
        };
        if !fits_in_u64(alignment) || overflows {
            errors.push(HeapLayoutError::Overflow);
        }
        errors
    }
}

fn finish_errors(
    cursor: u64,
    reserved_alignment: u128,
    reserved_size: u64,
    max_size: u64,
) -> (Vec<HeapLayoutError>, u128, u128) {
    let reserved_offset = u128::from(cursor).div_ceil(reserved_alignment) * reserved_alignment;
    let size = reserved_offset + u128::from(reserved_size);
    let mut errors = Vec::new();
    if !fits_in_u64(reserved_alignment) || !fits_in_u64(reserved_offset) || !fits_in_u64(size) {
        errors.push(HeapLayoutError::Overflow);
    }
    if size > u128::from(max_size) {
        errors.push(HeapLayoutError::TooLarge {
            size: u64::try_from(size).unwrap_or(u64::MAX),
            max: max_size,
        });
    }
    (errors, reserved_offset, size)
}

#[derive(Default)]
struct Seen {
    several_errors_at_once: BTreeMap<&'static str, usize>,
    successes: BTreeMap<&'static str, usize>,
}

impl Seen {
    fn record(&mut self, method: &'static str, applicable: &[HeapLayoutError]) {
        if applicable.len() >= 2 {
            *self.several_errors_at_once.entry(method).or_default() += 1;
        }
        if applicable.is_empty() {
            *self.successes.entry(method).or_default() += 1;
        }
    }

    fn assert_covers(&self, methods: &[&'static str]) {
        for method in methods {
            assert!(
                self.several_errors_at_once
                    .get(method)
                    .copied()
                    .unwrap_or(0)
                    > 0,
                "no input of {method} met two of its error conditions at once"
            );
            assert!(
                self.successes.get(method).copied().unwrap_or(0) > 0,
                "no input of {method} met none of its error conditions"
            );
        }
    }
}

fn assert_run(
    seen: &mut Seen,
    method: &'static str,
    context: &dyn core::fmt::Debug,
    result: Result<DescriptorSlots, HeapLayoutError>,
    applicable: &[HeapLayoutError],
    cursor: &mut u64,
) {
    seen.record(method, applicable);
    match (result, applicable.first()) {
        (Ok(slots), None) => *cursor = slots.end(),
        (Err(error), Some(first)) => assert_eq!(
            error, *first,
            "{method} returns the first error of its documentation that applies; all that apply: {applicable:?}; {context:?}, end of the plan {cursor}"
        ),
        (result, _) => panic!(
            "{method} returned {result:?}, and the errors of its documentation that apply are {applicable:?}; {context:?}, end of the plan {cursor}"
        ),
    }
}

fn assert_finish(
    seen: &mut Seen,
    method: &'static str,
    context: &dyn core::fmt::Debug,
    result: Result<DescriptorHeapLayout, HeapLayoutError>,
    expected: (Vec<HeapLayoutError>, u128, u128),
) {
    let (applicable, reserved_offset, size) = expected;
    seen.record(method, &applicable);
    match (result, applicable.first()) {
        (Ok(layout), None) => {
            assert_eq!(
                u128::from(layout.reserved_offset),
                reserved_offset,
                "{method} places the reserved range at the smallest multiple of the descriptor alignments at or after the end of the last run; {context:?}"
            );
            assert_eq!(
                u128::from(layout.size),
                reserved_offset + u128::from(layout.reserved_size),
                "{context:?}"
            );
            assert_eq!(u128::from(layout.size), size, "{context:?}");
        }
        (Err(error), Some(first)) => assert_eq!(
            error, *first,
            "{method} returns the first error of its documentation that applies; all that apply: {applicable:?}; {context:?}"
        ),
        (result, _) => panic!(
            "{method} returned {result:?}, and the errors of its documentation that apply are {applicable:?}; {context:?}"
        ),
    }
}

#[derive(Clone, Copy, Debug)]
enum Run {
    Buffers,
    Images,
    Uniform,
}

const ORDERS: [[Run; 3]; 3] = [
    [Run::Buffers, Run::Images, Run::Uniform],
    [Run::Images, Run::Uniform, Run::Buffers],
    [Run::Uniform, Run::Buffers, Run::Images],
];

#[test]
fn the_resource_heap_plan_returns_the_first_documented_error_that_applies() {
    let mut seen = Seen::default();
    for buffer_size in SIZES {
        for buffer_alignment in ALIGNMENTS {
            for image_size in SIZES {
                for image_alignment in ALIGNMENTS {
                    let resource = Resource {
                        buffer_size,
                        buffer_alignment,
                        image_size,
                        image_alignment,
                    };
                    for order in ORDERS {
                        for count in COUNTS {
                            resource_plan_follows_its_documentation(
                                &mut seen, resource, order, count,
                            );
                        }
                    }
                }
            }
        }
    }
    seen.assert_covers(&[
        "ResourceHeapPlan::buffers",
        "ResourceHeapPlan::images",
        "ResourceHeapPlan::uniform",
        "ResourceHeapPlan::finish",
    ]);
}

fn resource_plan_follows_its_documentation(
    seen: &mut Seen,
    resource: Resource,
    order: [Run; 3],
    count: u32,
) {
    let mut plan = ResourceHeapPlan::new(&resource.properties(0, u64::MAX));
    let mut cursor = 0;
    let context = (resource, order, count);
    for run in order {
        let (method, result, applicable) = match run {
            Run::Buffers => (
                "ResourceHeapPlan::buffers",
                plan.buffers(count),
                typed_run_errors(
                    cursor,
                    resource.buffer_size,
                    resource.buffer_alignment,
                    count,
                ),
            ),
            Run::Images => (
                "ResourceHeapPlan::images",
                plan.images(count),
                typed_run_errors(cursor, resource.image_size, resource.image_alignment, count),
            ),
            Run::Uniform => (
                "ResourceHeapPlan::uniform",
                plan.uniform(count),
                resource.uniform_errors(cursor, count),
            ),
        };
        assert_run(seen, method, &context, result, &applicable, &mut cursor);
    }
    for reserved_size in RESERVED_SIZES {
        for max_size in MAX_SIZES {
            let mut plan = ResourceHeapPlan::new(&resource.properties(reserved_size, max_size));
            let mut cursor = 0;
            for run in order {
                let result = match run {
                    Run::Buffers => plan.buffers(count),
                    Run::Images => plan.images(count),
                    Run::Uniform => plan.uniform(count),
                };
                if let Ok(slots) = result {
                    cursor = slots.end();
                }
            }
            assert_finish(
                seen,
                "ResourceHeapPlan::finish",
                &(context, cursor, reserved_size, max_size),
                plan.finish(),
                finish_errors(
                    cursor,
                    lcm(resource.buffer_alignment, resource.image_alignment),
                    reserved_size,
                    max_size,
                ),
            );
        }
    }
}

#[test]
fn the_sampler_heap_plan_returns_the_first_documented_error_that_applies() {
    let mut seen = Seen::default();
    for size in SIZES {
        for alignment in ALIGNMENTS {
            for first_count in COUNTS {
                for second_count in COUNTS {
                    for reserved_size in RESERVED_SIZES {
                        for max_size in MAX_SIZES {
                            let properties =
                                vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
                                    .sampler_descriptor_size(size)
                                    .sampler_descriptor_alignment(alignment)
                                    .min_sampler_heap_reserved_range(reserved_size)
                                    .max_sampler_heap_size(max_size);
                            let context = (size, alignment, first_count, second_count);
                            let mut plan = SamplerHeapPlan::new(&properties);
                            let mut cursor = 0;
                            for count in [first_count, second_count] {
                                let applicable = typed_run_errors(cursor, size, alignment, count);
                                assert_run(
                                    &mut seen,
                                    "SamplerHeapPlan::samplers",
                                    &context,
                                    plan.samplers(count),
                                    &applicable,
                                    &mut cursor,
                                );
                            }
                            assert_finish(
                                &mut seen,
                                "SamplerHeapPlan::finish",
                                &(context, cursor, reserved_size, max_size),
                                plan.finish(),
                                finish_errors(
                                    cursor,
                                    u128::from(alignment.max(1)),
                                    reserved_size,
                                    max_size,
                                ),
                            );
                        }
                    }
                }
            }
        }
    }
    seen.assert_covers(&["SamplerHeapPlan::samplers", "SamplerHeapPlan::finish"]);
}

#[test]
fn a_uniform_run_reports_a_zero_descriptor_size_before_an_overflow() {
    let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
        .buffer_descriptor_size(0)
        .buffer_descriptor_alignment(1 << 63)
        .image_descriptor_size(16)
        .image_descriptor_alignment(3);
    assert!(
        !fits_in_u64(lcm(1 << 63, 3)),
        "the least common multiple of the two alignments does not fit in u64"
    );
    assert_eq!(
        ResourceHeapPlan::new(&properties).uniform(1),
        Err(HeapLayoutError::ZeroDescriptorSize),
        "a descriptor size of 0 is reported before the overflow of the least common multiple"
    );
}

#[test]
fn a_typed_run_reports_an_unaligned_descriptor_size_before_an_overflow() {
    let (size, alignment) = (3 << 61, 1 << 62);
    let unaligned = Err(HeapLayoutError::UnalignedDescriptorSize { size, alignment });
    let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
        .buffer_descriptor_size(size)
        .buffer_descriptor_alignment(alignment)
        .image_descriptor_size(size)
        .image_descriptor_alignment(alignment)
        .sampler_descriptor_size(size)
        .sampler_descriptor_alignment(alignment);
    assert!(
        u64::from(3u32).checked_mul(size).is_none(),
        "the end of a run of 3 slots does not fit in u64"
    );
    assert_eq!(ResourceHeapPlan::new(&properties).buffers(3), unaligned);
    assert_eq!(ResourceHeapPlan::new(&properties).images(3), unaligned);
    assert_eq!(SamplerHeapPlan::new(&properties).samplers(3), unaligned);
}

#[test]
fn finish_reports_an_overflow_before_a_heap_that_is_too_large() {
    let resource = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
        .buffer_descriptor_size(16)
        .buffer_descriptor_alignment(16)
        .image_descriptor_size(16)
        .image_descriptor_alignment(16)
        .min_resource_heap_reserved_range(u64::MAX)
        .max_resource_heap_size(1 << 20);
    let mut plan = ResourceHeapPlan::new(&resource);
    assert_eq!(plan.buffers(1).map(|slots| slots.end()), Ok(16));
    assert_eq!(
        plan.finish(),
        Err(HeapLayoutError::Overflow),
        "16 + u64::MAX bytes do not fit in u64 and are more than maxResourceHeapSize"
    );

    let sampler = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
        .sampler_descriptor_size(16)
        .sampler_descriptor_alignment(16)
        .min_sampler_heap_reserved_range(u64::MAX)
        .max_sampler_heap_size(1 << 20);
    let mut plan = SamplerHeapPlan::new(&sampler);
    assert_eq!(plan.samplers(1).map(|slots| slots.end()), Ok(16));
    assert_eq!(
        plan.finish(),
        Err(HeapLayoutError::Overflow),
        "16 + u64::MAX bytes do not fit in u64 and are more than maxSamplerHeapSize"
    );
}

#[test]
fn finish_places_the_reserved_range_at_the_smallest_multiple_after_the_last_run() {
    let properties = vk::PhysicalDeviceDescriptorHeapPropertiesEXT::default()
        .buffer_descriptor_size(24)
        .buffer_descriptor_alignment(8)
        .image_descriptor_size(40)
        .image_descriptor_alignment(20)
        .min_resource_heap_reserved_range(64)
        .max_resource_heap_size(1 << 20);
    for (buffers, reserved_offset) in [(0, 0), (1, 40), (2, 80), (3, 80), (4, 120), (5, 120)] {
        let mut plan = ResourceHeapPlan::new(&properties);
        assert_eq!(
            plan.buffers(buffers).map(|slots| slots.end()),
            Ok(24 * u64::from(buffers))
        );
        let layout = plan.finish().expect("the heap fits");
        assert_eq!(
            layout.reserved_offset,
            reserved_offset,
            "the smallest multiple of 40, the least common multiple of 8 and 20, at or after {}",
            24 * buffers
        );
        assert_eq!(layout.size, reserved_offset + 64);
    }
}
