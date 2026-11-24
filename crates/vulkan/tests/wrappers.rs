#![cfg(feature = "std")]

mod fake_driver;

use fake_driver::{Call, DEVICE, Script, cases, fakes, run};
use proptest::collection::vec;
use proptest::prelude::*;
use std::collections::BTreeSet;
use vulkan::vk::{self, CommandInfo, CommandLevel, Handle, VkResult};

const PHYSICAL_DEVICE: u64 = 0x9D;
const QUEUE: u64 = 0x9E;
const SURFACE: u64 = 0x5F_0000_0000_0001;
const SWAPCHAIN: u64 = 0x5A_0000_0000_0002;
const SEMAPHORE: u64 = 0x5E_0000_0000_0003;
const FENCE: u64 = 0xFE_0000_0000_0004;
const PIPELINE_CACHE: u64 = 0xCA_0000_0000_0005;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Class {
    Success,
    Error,
    Unlisted,
}

fn class(info: &CommandInfo, code: VkResult) -> Class {
    if code.is_error() || info.error_codes.contains(&code) {
        Class::Error
    } else if info.success_codes.contains(&code) {
        Class::Success
    } else {
        Class::Unlisted
    }
}

fn codes(info: &CommandInfo) -> Vec<VkResult> {
    let mut codes: Vec<VkResult> = info
        .success_codes
        .iter()
        .chain(info.error_codes)
        .copied()
        .chain(VkResult::variants().iter().map(|&(code, _)| code))
        .chain(
            [i32::MIN, -1_000_000_000, -2, 6, 1_000_000_000, i32::MAX]
                .into_iter()
                .map(VkResult::from_raw),
        )
        .collect();
    codes.sort_by_key(|code| code.as_raw());
    codes.dedup();
    codes
}

fn each_code(info: &CommandInfo, mut check: impl FnMut(VkResult, Class, u64)) {
    let codes = codes(info);
    let classes: BTreeSet<Class> = codes.iter().map(|&code| class(info, code)).collect();
    assert_eq!(
        classes.len(),
        3,
        "{}: every class of code is tried",
        info.name
    );
    for (index, code) in codes.into_iter().enumerate() {
        check(
            code,
            class(info, code),
            0x0B0F_0000_0000_0000 + index as u64,
        );
    }
}

fn only_call(calls: &[Call], command: &str) -> Call {
    assert_eq!(calls.len(), 1, "{command} is called once: {calls:?}");
    assert_eq!(calls[0].command, command);
    calls[0].clone()
}

#[test]
fn a_wrapper_without_outputs_returns_ok_for_a_success_code_and_the_code_for_an_error() {
    let info = &vk::command::DEVICE_WAIT_IDLE;
    each_code(info, |code, class, _| {
        let (result, calls) = run(
            Script {
                results: vec![code],
                ..Script::default()
            },
            || unsafe { fakes().device.device_wait_idle() },
        );
        let call = only_call(&calls, info.name);
        assert_eq!(call.handles, [DEVICE], "the device handle is passed");
        match class {
            Class::Success => assert_eq!(result, Ok(Ok(())), "{code:?}"),
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
}

#[test]
fn a_wrapper_with_an_output_returns_what_the_driver_wrote_for_a_success_code() {
    let info = &vk::command::CREATE_BUFFER;
    each_code(info, |code, class, value| {
        let (result, calls) = run(
            Script {
                results: vec![code],
                outputs: vec![value],
                ..Script::default()
            },
            || unsafe {
                fakes()
                    .device
                    .create_buffer(&vk::BufferCreateInfo::default(), None)
            },
        );
        only_call(&calls, info.name);
        match class {
            Class::Success => assert_eq!(result, Ok(Ok(vk::Buffer::from_raw_u64(value)))),
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
}

#[test]
fn queue_present_returns_whether_the_driver_reported_suboptimal() {
    let info = &vk::command::QUEUE_PRESENT_KHR;
    each_code(info, |code, class, _| {
        let (result, calls) = run(
            Script {
                results: vec![code],
                ..Script::default()
            },
            || unsafe {
                fakes().device.queue_present_khr(
                    vk::Queue::from_raw_u64(QUEUE),
                    &vk::PresentInfoKHR::default(),
                )
            },
        );
        let call = only_call(&calls, info.name);
        assert_eq!(call.handles, [QUEUE], "the queue handle is passed");
        match class {
            Class::Success => {
                assert_eq!(result, Ok(Ok(code == VkResult::SUBOPTIMAL_KHR)), "{code:?}")
            }
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
}

#[test]
fn acquire_next_image_returns_the_index_none_on_a_timeout_and_the_code_for_an_error() {
    let info = &vk::command::ACQUIRE_NEXT_IMAGE_KHR;
    each_code(info, |code, class, value| {
        let index = value as u32;
        let (result, calls) = run(
            Script {
                results: vec![code],
                outputs: vec![u64::from(index)],
                ..Script::default()
            },
            || unsafe {
                fakes().device.acquire_next_image_khr(
                    vk::SwapchainKHR::from_raw_u64(SWAPCHAIN),
                    u64::MAX - 1,
                    vk::Semaphore::from_raw_u64(SEMAPHORE),
                    vk::Fence::from_raw_u64(FENCE),
                )
            },
        );
        let call = only_call(&calls, info.name);
        assert_eq!(call.handles, [DEVICE, SWAPCHAIN, SEMAPHORE, FENCE]);
        assert_eq!(call.scalars, [u64::MAX - 1], "the timeout is passed");
        let expected = match (class, code) {
            (Class::Success, VkResult::SUCCESS) => Ok(Some((index, false))),
            (Class::Success, VkResult::SUBOPTIMAL_KHR) => Ok(Some((index, true))),
            (Class::Success, VkResult::TIMEOUT | VkResult::NOT_READY) => Ok(None),
            (Class::Success, _) => panic!("the documentation does not describe {code:?}"),
            (Class::Error, _) => Err(code),
            (Class::Unlisted, _) => return,
        };
        assert_eq!(result, Ok(expected), "{code:?}");
    });
}

#[test]
fn a_wrapper_with_several_success_codes_returns_the_success_code() {
    let fences = [vk::Fence::from_raw_u64(FENCE)];
    let info = &vk::command::WAIT_FOR_FENCES;
    each_code(info, |code, class, _| {
        let (result, calls) = run(
            Script {
                results: vec![code],
                ..Script::default()
            },
            || unsafe { fakes().device.wait_for_fences(&fences, true, 0) },
        );
        only_call(&calls, info.name);
        match class {
            Class::Success => assert_eq!(result, Ok(Ok(code))),
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
    let info = &vk::command::GET_FENCE_STATUS;
    each_code(info, |code, class, _| {
        let (result, calls) = run(
            Script {
                results: vec![code],
                ..Script::default()
            },
            || unsafe {
                fakes()
                    .device
                    .get_fence_status(vk::Fence::from_raw_u64(FENCE))
            },
        );
        let call = only_call(&calls, info.name);
        assert_eq!(call.handles, [DEVICE, FENCE], "the fence handle is passed");
        match class {
            Class::Success => assert_eq!(result, Ok(Ok(code))),
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
}

#[test]
fn a_wrapper_with_an_output_and_several_success_codes_returns_both() {
    let info = &vk::command::GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT;
    each_code(info, |code, class, value| {
        let mut properties = vk::SwapchainTimeDomainPropertiesEXT {
            time_domain_count: 3,
            ..Default::default()
        };
        let (result, calls) = run(
            Script {
                results: vec![code],
                outputs: vec![value],
                ..Script::default()
            },
            || unsafe {
                fakes().device.get_swapchain_time_domain_properties_ext(
                    vk::SwapchainKHR::from_raw_u64(SWAPCHAIN),
                    &mut properties,
                )
            },
        );
        let call = only_call(&calls, info.name);
        assert_eq!(call.handles, [DEVICE, SWAPCHAIN]);
        assert_eq!(call.arrays, [Some(vec![3])], "the structure is passed");
        match class {
            Class::Success => assert_eq!(result, Ok(Ok((value, code)))),
            Class::Error => assert_eq!(result, Ok(Err(code)), "{code:?}"),
            Class::Unlisted => {}
        }
    });
}

type CreatedPipelines = Result<Vec<u64>, (Vec<u64>, VkResult)>;

fn create_pipelines(code: VkResult, written: &[u64]) -> (CreatedPipelines, Call) {
    let infos = [
        vk::GraphicsPipelineCreateInfo::default().stage_count(1),
        vk::GraphicsPipelineCreateInfo::default().stage_count(2),
        vk::GraphicsPipelineCreateInfo::default().stage_count(3),
    ];
    let (result, calls) = run(
        Script {
            results: vec![code],
            outputs: written.to_vec(),
            ..Script::default()
        },
        || unsafe {
            fakes().device.create_graphics_pipelines(
                vk::PipelineCache::from_raw_u64(PIPELINE_CACHE),
                &infos,
                None,
            )
        },
    );
    let raw = |pipelines: Vec<vk::Pipeline>| -> Vec<u64> {
        pipelines
            .iter()
            .map(|pipeline| pipeline.as_raw_u64())
            .collect()
    };
    let result = result
        .expect("the wrapper does not panic")
        .map(raw)
        .map_err(|(pipelines, code)| (raw(pipelines), code));
    let call = only_call(&calls, "vkCreateGraphicsPipelines");
    assert_eq!(call.handles, [DEVICE, PIPELINE_CACHE]);
    assert_eq!(call.scalars, [3], "the count is the number of create infos");
    assert_eq!(
        call.arrays,
        [Some(vec![1, 2, 3]), None],
        "the create infos are passed, and no allocator as null"
    );
    (result, call)
}

#[test]
fn creating_several_pipelines_returns_every_code_but_success_as_the_error_with_the_pipelines() {
    let info = &vk::command::CREATE_GRAPHICS_PIPELINES;
    each_code(info, |code, class, value| {
        let written = [value, 0, value + 1];
        let (result, _) = create_pipelines(code, &written);
        match class {
            Class::Success if code == VkResult::SUCCESS => assert_eq!(result, Ok(written.to_vec())),
            Class::Success | Class::Error => {
                assert_eq!(result, Err((written.to_vec(), code)), "{code:?}")
            }
            Class::Unlisted => {}
        }
    });
}

#[test]
fn an_unlisted_non_negative_code_is_not_an_error_where_the_documentation_says_so() {
    let device = &fakes().device;
    let fences = [vk::Fence::from_raw_u64(FENCE)];
    for code in codes(&vk::command::DEVICE_WAIT_IDLE) {
        if code.is_error() {
            continue;
        }
        let script = |outputs: Vec<u64>| Script {
            results: vec![code],
            outputs,
            ..Script::default()
        };
        let unlisted = |info: &CommandInfo| class(info, code) == Class::Unlisted;
        if unlisted(&vk::command::DEVICE_WAIT_IDLE) {
            let (result, _) = run(script(Vec::new()), || unsafe { device.device_wait_idle() });
            assert_eq!(result, Ok(Ok(())), "vkDeviceWaitIdle {code:?}");
        }
        if unlisted(&vk::command::CREATE_BUFFER) {
            let (result, _) = run(script(vec![0xB0F]), || unsafe {
                device.create_buffer(&vk::BufferCreateInfo::default(), None)
            });
            assert_eq!(
                result,
                Ok(Ok(vk::Buffer::from_raw_u64(0xB0F))),
                "vkCreateBuffer {code:?}"
            );
        }
        if unlisted(&vk::command::QUEUE_PRESENT_KHR) {
            let (result, _) = run(script(Vec::new()), || unsafe {
                device.queue_present_khr(
                    vk::Queue::from_raw_u64(QUEUE),
                    &vk::PresentInfoKHR::default(),
                )
            });
            assert_eq!(result, Ok(Ok(false)), "vkQueuePresentKHR {code:?}");
        }
        if unlisted(&vk::command::WAIT_FOR_FENCES) {
            let (result, _) = run(script(Vec::new()), || unsafe {
                device.wait_for_fences(&fences, false, 0)
            });
            assert_eq!(result, Ok(Ok(code)), "vkWaitForFences {code:?}");
        }
        if unlisted(&vk::command::GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT) {
            let mut properties = vk::SwapchainTimeDomainPropertiesEXT::default();
            let (result, _) = run(script(vec![42]), || unsafe {
                device.get_swapchain_time_domain_properties_ext(
                    vk::SwapchainKHR::from_raw_u64(SWAPCHAIN),
                    &mut properties,
                )
            });
            assert_eq!(
                result,
                Ok(Ok((42, code))),
                "vkGetSwapchainTimeDomainPropertiesEXT {code:?}"
            );
        }
        if unlisted(&vk::command::GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR) {
            let (result, _) = run(script(vec![1]), || unsafe {
                fakes().instance.get_physical_device_surface_support_khr(
                    vk::PhysicalDevice::from_raw_u64(PHYSICAL_DEVICE),
                    0,
                    vk::SurfaceKHR::from_raw_u64(SURFACE),
                )
            });
            assert_eq!(
                result,
                Ok(Ok(true)),
                "vkGetPhysicalDeviceSurfaceSupportKHR {code:?}"
            );
        }
        if unlisted(&vk::command::CREATE_GRAPHICS_PIPELINES) {
            let (result, _) = create_pipelines(code, &[1, 2, 3]);
            assert_eq!(
                result,
                Err((vec![1, 2, 3], code)),
                "vkCreateGraphicsPipelines {code:?}"
            );
        }
    }
}

fn handle() -> impl Strategy<Value = u64> + Clone {
    prop_oneof![Just(0), Just(1), Just(u64::MAX), 0..8u64, any::<u64>(),]
}

fn dispatchable_handle() -> impl Strategy<Value = u64> {
    prop_oneof![Just(1), Just(u64::from(u32::MAX)), 1..=u64::from(u32::MAX)]
}

fn scalar() -> impl Strategy<Value = u64> + Clone {
    prop_oneof![
        Just(0),
        Just(1),
        Just(u64::MAX),
        Just(u64::from(u32::MAX)),
        any::<u64>(),
    ]
}

fn elements<T: Strategy + Clone>(element: T) -> impl Strategy<Value = Vec<T::Value>> {
    prop_oneof![Just(0usize), Just(1usize), Just(16usize), 0..=16usize,]
        .prop_flat_map(move |len| vec(element.clone(), len))
}

fn same_or_other_length(len: usize) -> impl Strategy<Value = usize> {
    prop_oneof![3 => Just(len), 1 => 0..=17usize]
}

fn passed(array: &Option<Vec<u64>>) -> Vec<u64> {
    array.clone().unwrap_or_default()
}

proptest! {
    #![proptest_config(cases(256))]

    #[test]
    fn a_slice_is_passed_as_its_length_and_a_pointer_to_its_elements(
        fences in elements(handle()),
        wait_all in any::<bool>(),
        timeout in scalar(),
    ) {
        let fences: Vec<vk::Fence> = fences.into_iter().map(vk::Fence::from_raw_u64).collect();
        let (result, calls) = run(
            Script { results: vec![VkResult::SUCCESS], ..Script::default() },
            || unsafe { fakes().device.wait_for_fences(&fences, wait_all, timeout) },
        );
        prop_assert_eq!(result, Ok(Ok(VkResult::SUCCESS)));
        let call = only_call(&calls, "vkWaitForFences");
        prop_assert_eq!(&call.handles, &vec![DEVICE]);
        prop_assert_eq!(
            &call.scalars,
            &vec![fences.len() as u64, u64::from(wait_all), timeout],
            "the count is the length, and true is passed as VK_TRUE"
        );
        let raw: Vec<u64> = fences.iter().map(|fence| fence.as_raw_u64()).collect();
        prop_assert_eq!(passed(&call.arrays[0]), raw);
    }

    #[test]
    fn an_optional_reference_is_passed_as_null_or_as_a_pointer_to_the_same_value(
        size in scalar(),
        user_data in proptest::option::of(0..=usize::from(u16::MAX)),
        buffer in handle(),
    ) {
        let info = vk::BufferCreateInfo::default().size(size);
        let allocator = user_data.map(|user_data| {
            vk::AllocationCallbacks::default().user_data(std::ptr::without_provenance_mut(user_data))
        });
        let (result, calls) = run(
            Script { results: vec![VkResult::SUCCESS], outputs: vec![buffer], ..Script::default() },
            || unsafe { fakes().device.create_buffer(&info, allocator.as_ref()) },
        );
        prop_assert_eq!(result, Ok(Ok(vk::Buffer::from_raw_u64(buffer))));
        let call = only_call(&calls, "vkCreateBuffer");
        prop_assert_eq!(&call.handles, &vec![DEVICE]);
        prop_assert_eq!(&call.arrays[0], &Some(vec![size]), "the create info is passed");
        prop_assert_eq!(
            &call.arrays[1],
            &user_data.map(|user_data| vec![user_data as u64]),
            "no allocator is passed as null"
        );
    }

    #[test]
    fn a_vk_bool32_output_is_true_exactly_when_it_is_not_zero(
        supported in prop_oneof![Just(0u32), Just(1), Just(2), Just(u32::MAX), any::<u32>()],
        physical_device in dispatchable_handle(),
        queue_family_index in any::<u32>(),
        surface in handle(),
    ) {
        let (result, calls) = run(
            Script {
                results: vec![VkResult::SUCCESS],
                outputs: vec![u64::from(supported)],
                ..Script::default()
            },
            || unsafe {
                fakes().instance.get_physical_device_surface_support_khr(
                    vk::PhysicalDevice::from_raw_u64(physical_device),
                    queue_family_index,
                    vk::SurfaceKHR::from_raw_u64(surface),
                )
            },
        );
        prop_assert_eq!(result, Ok(Ok(supported != 0)));
        let call = only_call(&calls, "vkGetPhysicalDeviceSurfaceSupportKHR");
        prop_assert_eq!(&call.handles, &vec![physical_device, surface]);
        prop_assert_eq!(&call.scalars, &vec![u64::from(queue_family_index)]);
    }

    #[test]
    fn slices_that_share_a_count_are_passed_with_one_count_or_panic_before_the_call(
        command_buffer in dispatchable_handle(),
        first_binding in any::<u32>(),
        (buffers, offsets) in elements(handle()).prop_flat_map(|buffers| {
            let len = buffers.len();
            (Just(buffers), same_or_other_length(len).prop_flat_map(|len| vec(scalar(), len)))
        }),
    ) {
        let buffer_handles: Vec<vk::Buffer> = buffers.iter().copied().map(vk::Buffer::from_raw_u64).collect();
        let (result, calls) = run(Script::default(), || unsafe {
            fakes().device.cmd_bind_vertex_buffers(
                vk::CommandBuffer::from_raw_u64(command_buffer),
                first_binding,
                &buffer_handles,
                &offsets,
            )
        });
        if buffers.len() != offsets.len() {
            let message = result.expect_err("different lengths panic");
            prop_assert!(message.contains("vkCmdBindVertexBuffers"), "{}", message);
            prop_assert!(calls.is_empty(), "the command is not called");
        } else {
            prop_assert_eq!(result, Ok(()));
            let call = only_call(&calls, "vkCmdBindVertexBuffers");
            prop_assert_eq!(&call.handles, &vec![command_buffer]);
            prop_assert_eq!(&call.scalars, &vec![u64::from(first_binding), buffers.len() as u64]);
            prop_assert_eq!(passed(&call.arrays[0]), buffers);
            prop_assert_eq!(passed(&call.arrays[1]), offsets);
        }
    }

    #[test]
    fn optional_slices_that_share_a_count_are_null_when_absent_and_checked_when_present(
        (buffers, offsets, sizes, strides) in elements(handle()).prop_flat_map(|buffers| {
            let len = buffers.len();
            let other = move || same_or_other_length(len).prop_flat_map(|len| vec(scalar(), len));
            (
                Just(buffers),
                prop_oneof![4 => vec(scalar(), len), 1 => other()],
                proptest::option::of(other()),
                proptest::option::of(other()),
            )
        }),
    ) {
        let buffer_handles: Vec<vk::Buffer> = buffers.iter().copied().map(vk::Buffer::from_raw_u64).collect();
        let (result, calls) = run(Script::default(), || unsafe {
            fakes().device.cmd_bind_vertex_buffers2(
                vk::CommandBuffer::from_raw_u64(1),
                0,
                &buffer_handles,
                &offsets,
                sizes.as_deref(),
                strides.as_deref(),
            )
        });
        let agree = |array: Option<&Vec<u64>>| array.is_none_or(|array| array.len() == buffers.len());
        if !(agree(Some(&offsets)) && agree(sizes.as_ref()) && agree(strides.as_ref())) {
            let message = result.expect_err("different lengths panic");
            prop_assert!(message.contains("vkCmdBindVertexBuffers2"), "{}", message);
            prop_assert!(calls.is_empty(), "the command is not called");
        } else {
            prop_assert_eq!(result, Ok(()));
            let call = only_call(&calls, "vkCmdBindVertexBuffers2");
            prop_assert_eq!(&call.scalars, &vec![0, buffers.len() as u64]);
            prop_assert_eq!(passed(&call.arrays[0]), buffers);
            prop_assert_eq!(passed(&call.arrays[1]), offsets);
            for (array, given) in [(&call.arrays[2], &sizes), (&call.arrays[3], &strides)] {
                match given {
                    None => prop_assert_eq!(array, &None, "an absent slice is passed as null"),
                    Some(given) => prop_assert_eq!(&passed(array), given),
                }
            }
        }
    }

    #[test]
    fn a_slice_of_optional_slices_is_passed_as_an_array_of_pointers(
        (geometry_counts, ranges) in vec(0..=3u32, 0..=6).prop_flat_map(|geometry_counts| {
            let entries: Vec<_> = geometry_counts
                .iter()
                .map(|&count| {
                    proptest::option::of(
                        prop_oneof![6 => Just(count as usize), 1 => 0..=4usize]
                            .prop_flat_map(|len| vec(any::<u32>(), len)),
                    )
                })
                .collect();
            let outer = prop_oneof![
                8 => Just(None),
                1 => proptest::option::of(proptest::option::of(vec(any::<u32>(), 0..=3))),
            ];
            (Just(geometry_counts), entries, outer).prop_map(|(geometry_counts, mut entries, outer)| {
                match outer {
                    None => {}
                    Some(Some(extra)) => entries.push(Some(extra)),
                    Some(None) => {
                        if entries.is_empty() {
                            entries.push(None);
                        } else {
                            entries.pop();
                        }
                    }
                }
                (geometry_counts, entries)
            })
        }),
    ) {
        let infos: Vec<vk::AccelerationStructureBuildGeometryInfoKHR<'_>> = geometry_counts
            .iter()
            .map(|&geometry_count| vk::AccelerationStructureBuildGeometryInfoKHR {
                geometry_count,
                ..Default::default()
            })
            .collect();
        let range_infos: Vec<Option<Vec<vk::AccelerationStructureBuildRangeInfoKHR>>> = ranges
            .iter()
            .map(|entry| {
                entry.as_ref().map(|primitive_counts| {
                    primitive_counts
                        .iter()
                        .map(|&primitive_count| vk::AccelerationStructureBuildRangeInfoKHR {
                            primitive_count,
                            ..Default::default()
                        })
                        .collect()
                })
            })
            .collect();
        let build_range_infos: Vec<Option<&[vk::AccelerationStructureBuildRangeInfoKHR]>> =
            range_infos.iter().map(|entry| entry.as_deref()).collect();
        let (result, calls) = run(Script::default(), || unsafe {
            fakes().device.cmd_build_acceleration_structures_khr(
                vk::CommandBuffer::from_raw_u64(1),
                &infos,
                &build_range_infos,
            )
        });
        let fits = ranges.len() == geometry_counts.len()
            && ranges.iter().zip(&geometry_counts).all(|(entry, &count)| {
                entry.as_ref().is_none_or(|entry| entry.len() == count as usize)
            });
        if !fits {
            let message = result.expect_err("an entry of another length panics");
            prop_assert!(message.contains("vkCmdBuildAccelerationStructuresKHR"), "{}", message);
            prop_assert!(calls.is_empty(), "the command is not called");
        } else {
            prop_assert_eq!(result, Ok(()));
            let call = only_call(&calls, "vkCmdBuildAccelerationStructuresKHR");
            prop_assert_eq!(&call.scalars, &vec![geometry_counts.len() as u64]);
            let counts: Vec<u64> = geometry_counts.iter().map(|&count| u64::from(count)).collect();
            prop_assert_eq!(passed(&call.arrays[0]), counts);
            prop_assert_eq!(call.arrays.len(), ranges.len() + 1, "one pointer per info");
            for (array, entry) in call.arrays[1..].iter().zip(&ranges) {
                match entry {
                    None => prop_assert_eq!(array, &None, "None is passed as a null pointer"),
                    Some(entry) => {
                        let expected: Vec<u64> = entry.iter().map(|&count| u64::from(count)).collect();
                        prop_assert_eq!(passed(array), expected);
                    }
                }
            }
        }
    }
}

fn names_of(level: CommandLevel) -> BTreeSet<String> {
    vk::COMMANDS
        .iter()
        .filter(|info| info.level == level)
        .map(|info| info.name.to_string())
        .collect()
}

const EVERY_PLATFORM: bool = cfg!(all(
    feature = "android",
    feature = "beta",
    feature = "directfb",
    feature = "fuchsia",
    feature = "ggp",
    feature = "ios",
    feature = "macos",
    feature = "metal",
    feature = "ohos",
    feature = "sci",
    feature = "screen",
    feature = "ubm",
    feature = "vi",
    feature = "wayland",
    feature = "win32",
    feature = "xcb",
    feature = "xlib",
    feature = "xlib-xrandr",
));

fn check_requests(
    table: &str,
    requests: &[(u64, String)],
    handle: u64,
    level: CommandLevel,
    loaded_without_asking: &[&str],
) -> BTreeSet<String> {
    assert!(
        requests.iter().all(|(asked_with, _)| *asked_with == handle),
        "{table} asks with the handle {handle:#x}: {:?}",
        requests
            .iter()
            .find(|(asked_with, _)| *asked_with != handle)
    );
    let names: BTreeSet<String> = requests.iter().map(|(_, name)| name.clone()).collect();
    assert_eq!(
        names.len(),
        requests.len(),
        "{table} asks for each command once"
    );
    let mut expected = names_of(level);
    for name in loaded_without_asking {
        assert!(expected.remove(*name), "{name} is a command of the level");
    }
    assert!(
        names.is_subset(&expected),
        "{table} asks only for commands of its level: {:?}",
        names.difference(&expected).collect::<Vec<_>>()
    );
    if EVERY_PLATFORM {
        assert_eq!(
            names, expected,
            "{table} asks for every command of its level"
        );
    }
    names
}

#[test]
fn every_table_asks_for_each_command_of_its_level_once_with_the_handle_it_was_given() {
    fake_driver::reset();
    let _ = unsafe { vk::EntryFn::load(fake_driver::instance_proc_addr()) };
    check_requests(
        "EntryFn::load",
        &fake_driver::take_requests(),
        0,
        CommandLevel::Entry,
        &["vkGetInstanceProcAddr"],
    );
    let instance = 0x1234_5678;
    let _ = unsafe {
        vk::InstanceFn::load(
            fake_driver::instance_proc_addr(),
            vk::Instance::from_raw_u64(instance),
        )
    };
    let instance_names = check_requests(
        "InstanceFn::load",
        &fake_driver::take_requests(),
        instance,
        CommandLevel::Instance,
        &[],
    );
    let device = 0x8765_4321;
    let _ = unsafe {
        vk::DeviceFn::load(
            fake_driver::device_proc_addr(),
            vk::Device::from_raw_u64(device),
        )
    };
    let device_names = check_requests(
        "DeviceFn::load",
        &fake_driver::take_requests(),
        device,
        CommandLevel::Device,
        &[],
    );

    let entry = fake_driver::entry();
    fake_driver::take_requests();
    let loaded = unsafe { vulkan::Instance::load(&entry, vk::Instance::from_raw_u64(instance)) };
    let names = check_requests(
        "Instance::load",
        &fake_driver::take_requests(),
        instance,
        CommandLevel::Instance,
        &[],
    );
    assert_eq!(names, instance_names);
    let _ = unsafe { vulkan::Device::load(&loaded, vk::Device::from_raw_u64(device)) };
    let names = check_requests(
        "Device::load",
        &fake_driver::take_requests(),
        device,
        CommandLevel::Device,
        &[],
    );
    assert_eq!(
        names, device_names,
        "Device::load asks the vkGetDeviceProcAddr of the instance"
    );
}

#[test]
fn creating_an_instance_or_a_device_loads_its_commands_with_the_new_handle() {
    let created_instance = 0x1A_0001;
    let (result, calls) = run(
        Script {
            results: vec![VkResult::SUCCESS],
            outputs: vec![created_instance],
            ..Script::default()
        },
        || unsafe {
            fakes()
                .entry
                .create_instance(&vk::InstanceCreateInfo::default(), None)
        },
    );
    let instance = result.unwrap().expect("vkCreateInstance succeeds");
    assert_eq!(instance.handle().as_raw_u64(), created_instance);
    assert_eq!(
        only_call(&calls, "vkCreateInstance").arrays,
        [Some(vec![0]), None]
    );
    check_requests(
        "Entry::create_instance",
        &fake_driver::take_requests(),
        created_instance,
        CommandLevel::Instance,
        &[],
    );

    let created_device = 0xDE_0001;
    let physical_device = vk::PhysicalDevice::from_raw_u64(PHYSICAL_DEVICE);
    let (result, calls) = run(
        Script {
            results: vec![VkResult::SUCCESS],
            outputs: vec![created_device],
            ..Script::default()
        },
        || unsafe {
            instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
        },
    );
    let device = result.unwrap().expect("vkCreateDevice succeeds");
    assert_eq!(device.handle().as_raw_u64(), created_device);
    let call = only_call(&calls, "vkCreateDevice");
    assert_eq!(call.handles, [PHYSICAL_DEVICE]);
    assert_eq!(call.arrays, [Some(vec![0]), None]);
    check_requests(
        "Instance::create_device",
        &fake_driver::take_requests(),
        created_device,
        CommandLevel::Device,
        &[],
    );

    for code in [
        VkResult::ERROR_INCOMPATIBLE_DRIVER,
        VkResult::from_raw(i32::MIN),
    ] {
        let (result, _) = run(
            Script {
                results: vec![code],
                outputs: vec![created_instance],
                ..Script::default()
            },
            || unsafe {
                fakes()
                    .entry
                    .create_instance(&vk::InstanceCreateInfo::default(), None)
            },
        );
        assert_eq!(result.unwrap().err(), Some(code));
        assert!(
            fake_driver::take_requests().is_empty(),
            "nothing is loaded after an error"
        );
        let (result, _) = run(
            Script {
                results: vec![code],
                outputs: vec![created_device],
                ..Script::default()
            },
            || unsafe {
                instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
            },
        );
        assert_eq!(result.unwrap().err(), Some(code));
        assert!(
            fake_driver::take_requests().is_empty(),
            "nothing is loaded after an error"
        );
    }
}

#[test]
fn the_instance_version_is_none_when_the_loader_does_not_provide_the_command() {
    let without = fake_driver::entry_without_commands();
    assert_eq!(unsafe { without.enumerate_instance_version() }, Ok(None));
    let with = fake_driver::entry();
    assert_eq!(
        unsafe { with.enumerate_instance_version() },
        Ok(Some(fake_driver::INSTANCE_VERSION))
    );
}

type UnloadedCall<'a> = (&'static CommandInfo, Box<dyn Fn() + 'a>);

#[test]
fn a_command_that_is_not_loaded_panics_with_its_name() {
    let entry = fake_driver::entry_without_commands();
    let instance = fake_driver::instance_without_commands();
    let device = fake_driver::device_without_commands();
    let physical_device = vk::PhysicalDevice::from_raw_u64(PHYSICAL_DEVICE);
    let command_buffer = vk::CommandBuffer::from_raw_u64(1);
    let fences = [vk::Fence::from_raw_u64(FENCE)];
    let calls: Vec<UnloadedCall<'_>> = vec![
        (
            &vk::command::ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
            Box::new(|| {
                let _ = unsafe { entry.enumerate_instance_extension_properties(None) };
            }),
        ),
        (
            &vk::command::CREATE_INSTANCE,
            Box::new(|| {
                let _ = unsafe { entry.create_instance(&vk::InstanceCreateInfo::default(), None) };
            }),
        ),
        (
            &vk::command::ENUMERATE_PHYSICAL_DEVICES,
            Box::new(|| {
                let _ = unsafe { instance.enumerate_physical_devices() };
            }),
        ),
        (
            &vk::command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2,
            Box::new(|| {
                let _ = unsafe {
                    instance.get_physical_device_queue_family_properties2(physical_device)
                };
            }),
        ),
        (
            &vk::command::GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR,
            Box::new(|| {
                let _ = unsafe {
                    instance.get_physical_device_surface_support_khr(
                        physical_device,
                        0,
                        vk::SurfaceKHR::null(),
                    )
                };
            }),
        ),
        (
            &vk::command::CREATE_DEVICE,
            Box::new(|| {
                let _ = unsafe {
                    instance.create_device(physical_device, &vk::DeviceCreateInfo::default(), None)
                };
            }),
        ),
        (
            &vk::command::CMD_DRAW,
            Box::new(|| unsafe { device.cmd_draw(command_buffer, 3, 1, 0, 0) }),
        ),
        (
            &vk::command::CMD_BIND_VERTEX_BUFFERS,
            Box::new(|| unsafe {
                device.cmd_bind_vertex_buffers(command_buffer, 0, &[vk::Buffer::null()], &[0])
            }),
        ),
        (
            &vk::command::DEVICE_WAIT_IDLE,
            Box::new(|| {
                let _ = unsafe { device.device_wait_idle() };
            }),
        ),
        (
            &vk::command::CREATE_BUFFER,
            Box::new(|| {
                let _ = unsafe { device.create_buffer(&vk::BufferCreateInfo::default(), None) };
            }),
        ),
        (
            &vk::command::QUEUE_PRESENT_KHR,
            Box::new(|| {
                let _ = unsafe {
                    device.queue_present_khr(vk::Queue::null(), &vk::PresentInfoKHR::default())
                };
            }),
        ),
        (
            &vk::command::ACQUIRE_NEXT_IMAGE_KHR,
            Box::new(|| {
                let _ = unsafe {
                    device.acquire_next_image_khr(
                        vk::SwapchainKHR::null(),
                        0,
                        vk::Semaphore::null(),
                        vk::Fence::null(),
                    )
                };
            }),
        ),
        (
            &vk::command::WAIT_FOR_FENCES,
            Box::new(|| {
                let _ = unsafe { device.wait_for_fences(&fences, true, 0) };
            }),
        ),
        (
            &vk::command::GET_PIPELINE_CACHE_DATA,
            Box::new(|| {
                let _ = unsafe { device.get_pipeline_cache_data(vk::PipelineCache::null()) };
            }),
        ),
        (
            &vk::command::CREATE_GRAPHICS_PIPELINES,
            Box::new(|| {
                let _ = unsafe {
                    device.create_graphics_pipelines(
                        vk::PipelineCache::null(),
                        &[vk::GraphicsPipelineCreateInfo::default()],
                        None,
                    )
                };
            }),
        ),
    ];
    for (info, call) in calls {
        let (result, driver_calls) = run(Script::default(), call);
        assert_eq!(
            result.err(),
            Some(format!("{} is not loaded", info.name)),
            "the panic names the command"
        );
        assert!(driver_calls.is_empty());
    }
}
