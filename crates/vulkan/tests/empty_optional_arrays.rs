use std::panic::catch_unwind;
use vulkan::vk;

#[test]
fn no_immutable_samplers_leaves_the_descriptor_count_alone_and_sets_null() {
    let none: Vec<vk::Sampler> = Vec::new();
    let binding = vk::DescriptorSetLayoutBinding::default()
        .descriptor_type(vk::DescriptorType::SAMPLER)
        .immutable_samplers(&none)
        .descriptor_count(1);
    assert_eq!(binding.descriptor_count, 1);
    assert!(
        binding.p_immutable_samplers.is_null(),
        "pImmutableSamplers is NULL, which makes the sampler of the binding dynamic"
    );

    let binding = vk::DescriptorSetLayoutBinding::default()
        .descriptor_type(vk::DescriptorType::SAMPLER)
        .descriptor_count(1)
        .immutable_samplers(&none);
    assert_eq!(
        binding.descriptor_count, 1,
        "an empty slice does not change the number of descriptors of the binding"
    );
    assert!(binding.p_immutable_samplers.is_null());
}

#[test]
fn no_resolve_attachments_can_be_said_with_an_empty_slice() {
    let colors = [vk::AttachmentReference::default(); 2];
    let color_then_resolve = vk::SubpassDescription::default()
        .color_attachments(&colors)
        .resolve_attachments(&[]);
    let resolve_then_color = vk::SubpassDescription::default()
        .resolve_attachments(&[])
        .color_attachments(&colors);
    for subpass in [color_then_resolve, resolve_then_color] {
        assert_eq!(subpass.color_attachment_count, 2);
        assert_eq!(subpass.p_color_attachments, colors.as_ptr());
        assert!(
            subpass.p_resolve_attachments.is_null(),
            "pResolveAttachments is NULL, which means that no attachment is resolved"
        );
    }

    let colors = [vk::AttachmentReference2::default(); 3];
    let subpass = vk::SubpassDescription2::default()
        .color_attachments(&colors)
        .resolve_attachments(&[]);
    assert_eq!(subpass.color_attachment_count, 3);
    assert!(subpass.p_resolve_attachments.is_null());
}

#[test]
fn other_optional_arrays_that_share_a_count_can_be_left_out_with_an_empty_slice() {
    let swapchains = [vk::SwapchainKHR::null()];
    let indices = [0_u32];
    let present = vk::PresentInfoKHR::default()
        .swapchains(&swapchains)
        .image_indices(&indices)
        .results(&mut []);
    assert_eq!(present.swapchain_count, 1);
    assert!(present.p_results.is_null());

    let dimensions = [4_i64, 4];
    let tensor = vk::TensorDescriptionARM::default()
        .dimensions(&dimensions)
        .strides(&[]);
    assert_eq!(tensor.dimension_count, 2);
    assert!(tensor.p_strides.is_null());
}

#[test]
fn an_optional_array_that_is_set_again_to_an_empty_slice_takes_its_count_back() {
    let offsets = [16_u32, 32];
    let bind = vk::BindDescriptorSetsInfo::default().dynamic_offsets(&offsets);
    assert_eq!(bind.dynamic_offset_count, 2);
    assert_eq!(bind.p_dynamic_offsets, offsets.as_ptr());
    let bind = bind.dynamic_offsets(&[]);
    assert_eq!(
        bind.dynamic_offset_count, 0,
        "the count that the first call stored does not outlive the array"
    );
    assert!(bind.p_dynamic_offsets.is_null());

    let samplers = [vk::Sampler::null(); 2];
    let binding = vk::DescriptorSetLayoutBinding::default()
        .immutable_samplers(&samplers)
        .immutable_samplers(&[]);
    assert_eq!(binding.descriptor_count, 0);
    assert!(binding.p_immutable_samplers.is_null());
}

#[test]
fn taking_an_array_away_resets_a_count_that_the_count_setter_stored_after_it() {
    let samplers = [vk::Sampler::null(); 2];
    let binding = vk::DescriptorSetLayoutBinding::default()
        .immutable_samplers(&samplers)
        .descriptor_count(1);
    assert_eq!(binding.descriptor_count, 1);
    assert_eq!(binding.p_immutable_samplers, samplers.as_ptr());
    let binding = binding.immutable_samplers(&[]);
    assert_eq!(
        binding.descriptor_count, 0,
        "the setter sees a pointer that is not null and cannot tell who stored the count"
    );
    assert!(binding.p_immutable_samplers.is_null());

    let binding = binding.descriptor_count(1).immutable_samplers(&[]);
    assert_eq!(
        binding.descriptor_count, 1,
        "with a null pointer, an empty slice leaves the count that the count setter stored"
    );
    assert!(binding.p_immutable_samplers.is_null());
}

#[test]
fn an_optional_array_that_is_taken_away_leaves_the_count_to_the_arrays_that_share_it() {
    let colors = [vk::AttachmentReference::default(); 2];
    let resolves = [vk::AttachmentReference::default(); 2];
    let subpass = vk::SubpassDescription::default()
        .color_attachments(&colors)
        .resolve_attachments(&resolves);
    assert_eq!(subpass.p_resolve_attachments, resolves.as_ptr());
    let subpass = subpass.resolve_attachments(&[]);
    assert_eq!(
        subpass.color_attachment_count, 2,
        "the color attachments still have two elements"
    );
    assert_eq!(subpass.p_color_attachments, colors.as_ptr());
    assert!(subpass.p_resolve_attachments.is_null());

    let source = [1_u64, 2];
    let destination = [3_u64, 4];
    let copy = vk::TensorCopyARM::default()
        .src_offset(&source)
        .dst_offset(&destination)
        .src_offset(&[]);
    assert_eq!(copy.dimension_count, 2);
    assert!(copy.p_src_offset.is_null());
    let copy = copy.dst_offset(&[]);
    assert_eq!(
        copy.dimension_count, 0,
        "no array that shares the count is left"
    );
    assert!(copy.p_dst_offset.is_null());
}

#[test]
fn an_empty_output_array_asks_for_the_count() {
    let info = vk::GetLatencyMarkerInfoNV::default().timings(&mut []);
    assert_eq!(info.timing_count, 0);
    assert!(
        info.p_timings.is_null(),
        "pTimings is NULL, for which the command returns the number of timings"
    );
    let representation = vk::PipelineExecutableInternalRepresentationKHR::default().data(&mut []);
    assert_eq!(representation.data_size, 0);
    assert!(representation.p_data.is_null());
}

#[test]
fn a_slice_that_is_not_empty_sets_the_pointer_and_the_count_of_an_optional_array() {
    let samplers = [vk::Sampler::null(); 3];
    for len in 1..=samplers.len() {
        let binding = vk::DescriptorSetLayoutBinding::default()
            .descriptor_count(7)
            .immutable_samplers(&samplers[..len]);
        assert_eq!(binding.descriptor_count as usize, len);
        assert_eq!(binding.p_immutable_samplers, samplers.as_ptr());
    }
    let colors = [vk::AttachmentReference::default(); 2];
    let resolves = [vk::AttachmentReference::default(); 3];
    let mismatch = catch_unwind(|| {
        vk::SubpassDescription::default()
            .color_attachments(&colors)
            .resolve_attachments(&resolves[..1])
            .color_attachment_count
    });
    assert!(
        mismatch.is_err(),
        "a resolve array that is not empty still has to be as long as the color attachments"
    );
}

#[test]
fn an_empty_slice_of_an_array_that_is_not_optional_sets_null_and_a_count_of_zero() {
    let queues = [vk::DeviceQueueCreateInfo::default(); 2];
    let info = vk::DeviceCreateInfo::default().queue_create_infos(&queues);
    assert_eq!(info.queue_create_info_count, 2);
    assert_eq!(info.p_queue_create_infos, queues.as_ptr());
    let info = info.queue_create_infos(&[]);
    assert_eq!(info.queue_create_info_count, 0);
    assert!(
        info.p_queue_create_infos.is_null(),
        "with a count of 0 the pointer is not read, and a null pointer does not look like an array"
    );
    let info = info.queue_create_infos(&queues[..0]);
    assert!(info.p_queue_create_infos.is_null());

    let info = vk::DeviceCreateInfo::default()
        .queue_create_info_count(3)
        .queue_create_infos(&[]);
    assert_eq!(
        info.queue_create_info_count, 0,
        "the array is not optional: an empty slice stands for zero elements, not for an array that is absent"
    );
    assert!(info.p_queue_create_infos.is_null());

    let colors = [vk::AttachmentReference::default(); 2];
    let resolves = [vk::AttachmentReference::default(); 2];
    let mismatch = catch_unwind(|| {
        vk::SubpassDescription::default()
            .resolve_attachments(&resolves)
            .color_attachments(&colors[..0])
            .color_attachment_count
    });
    assert!(
        mismatch.is_err(),
        "an empty array of color attachments, which are not optional, does not match two resolve attachments"
    );
    let semaphores = [vk::Semaphore::null(); 2];
    let mismatch = catch_unwind(|| {
        vk::SubmitInfo::default()
            .wait_semaphores(&semaphores)
            .wait_dst_stage_mask(&[])
            .wait_semaphore_count
    });
    assert!(
        mismatch.is_err(),
        "no stage masks do not match two semaphores"
    );
    let submit = vk::SubmitInfo::default()
        .wait_semaphores(&[])
        .wait_dst_stage_mask(&[]);
    assert_eq!(submit.wait_semaphore_count, 0);
    assert!(submit.p_wait_semaphores.is_null() && submit.p_wait_dst_stage_mask.is_null());
}

#[test]
fn an_empty_slice_is_null_where_the_specification_gives_null_a_meaning() {
    let locations = vk::RenderingAttachmentLocationInfo::default()
        .color_attachment_locations(&[])
        .color_attachment_count(2);
    assert_eq!(locations.color_attachment_count, 2);
    assert!(
        locations.p_color_attachment_locations.is_null(),
        "NULL maps each color attachment to its index; a pointer that is not null is read for two elements"
    );
    let locations = vk::RenderingAttachmentLocationInfo::default()
        .color_attachment_count(2)
        .color_attachment_locations(&[]);
    assert_eq!(locations.color_attachment_count, 2);
    assert!(locations.p_color_attachment_locations.is_null());
    let remapped = [1, 0];
    let locations = locations.color_attachment_locations(&remapped);
    assert_eq!(locations.color_attachment_count, 2);
    assert_eq!(locations.p_color_attachment_locations, remapped.as_ptr());
    let locations = locations.color_attachment_locations(&[]);
    assert_eq!(
        locations.color_attachment_count, 0,
        "the array that the count described is taken away"
    );
    assert!(locations.p_color_attachment_locations.is_null());

    let timings = vk::PastPresentationTimingPropertiesEXT::default().presentation_timings(&mut []);
    assert_eq!(timings.presentation_timing_count, 0);
    assert!(
        timings.p_presentation_timings.is_null(),
        "pPresentationTimings is NULL, for which the command returns the number of timing records"
    );
    let blocks = vk::PhysicalDeviceGpaPropertiesAMD::default().perf_blocks(&mut []);
    assert_eq!(blocks.perf_block_count, 0);
    assert!(blocks.p_perf_blocks.is_null());

    let present = vk::PresentInfoKHR::default().wait_semaphores(&[]);
    assert_eq!(present.wait_semaphore_count, 0);
    assert!(present.p_wait_semaphores.is_null());
    let data = vk::DebugUtilsMessengerCallbackDataEXT::default()
        .queue_labels(&[])
        .cmd_buf_labels(&[]);
    assert!(data.p_queue_labels.is_null() && data.p_cmd_buf_labels.is_null());
    let types = vk::MutableDescriptorTypeListEXT::default().descriptor_types(&[]);
    assert!(types.p_descriptor_types.is_null());
    let control = vk::ImageCompressionControlEXT::default().fixed_rate_flags(&[]);
    assert!(control.p_fixed_rate_flags.is_null());
    let decode = vk::VideoDecodeInfoKHR::default().reference_slots(&[]);
    assert!(decode.p_reference_slots.is_null());
    let encode = vk::VideoEncodeInfoKHR::default().reference_slots(&[]);
    assert!(encode.p_reference_slots.is_null());

    let tiles = vk::StdVideoAV1TileInfo::default()
        .tile_cols(3)
        .tile_rows(2)
        .width_in_sbs_minus1(&[])
        .height_in_sbs_minus1(&[]);
    assert_eq!((tiles.tile_cols, tiles.tile_rows), (3, 2));
    assert!(
        tiles.p_width_in_sbs_minus1.is_null() && tiles.p_height_in_sbs_minus1.is_null(),
        "NULL leaves the sizes of the 3 columns and 2 rows to the implementation"
    );
    let starts = [0_u16, 4, 8];
    let tiles = vk::StdVideoAV1TileInfo::default()
        .mi_col_starts(&starts)
        .width_in_sbs_minus1(&[]);
    assert_eq!(tiles.tile_cols, 3, "the starts still have three elements");
    assert!(tiles.p_width_in_sbs_minus1.is_null());
    let graph = vk::DataGraphPipelineCreateInfoARM::default().resource_infos(&[]);
    assert_eq!(graph.resource_info_count, 0);
    assert!(
        graph.p_resource_infos.is_null(),
        "pResourceInfos has to be NULL if resourceInfoCount is 0"
    );
    let device = vk::DeviceCreateInfo::default().enabled_layer_names(&[]);
    assert_eq!(device.enabled_layer_count, 0);
    assert!(
        device.pp_enabled_layer_names.is_null(),
        "ppEnabledLayerNames has to be NULL"
    );
}

#[test]
fn a_pipeline_layout_without_push_constant_ranges_stores_null() {
    let layout = vk::PipelineLayoutCreateInfo::default().push_constant_ranges(&[]);
    assert_eq!(layout.push_constant_range_count, 0);
    assert!(
        layout.p_push_constant_ranges.is_null(),
        "VUID-VkDataGraphPipelineCreateInfoARM-layout-09767 asks for a layout created with pPushConstantRanges equal to NULL"
    );
    let ranges = [vk::PushConstantRange::default(); 2];
    let layout = layout.push_constant_ranges(&ranges);
    assert_eq!(layout.push_constant_range_count, 2);
    assert_eq!(layout.p_push_constant_ranges, ranges.as_ptr());
    let layout = layout.push_constant_ranges(&ranges[2..]);
    assert_eq!(layout.push_constant_range_count, 0);
    assert!(layout.p_push_constant_ranges.is_null());
}
