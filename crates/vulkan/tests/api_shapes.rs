use core::mem;
use vulkan::{Device, vk};

#[test]
fn shader_module_code_sets_size_and_pointer() {
    let code = [0x0723_0203_u32, 0, 0, 0];
    let info = vk::ShaderModuleCreateInfo::default().code(&code);
    assert_eq!(info.code_size, mem::size_of_val(&code));
    assert_eq!(info.p_code, code.as_ptr());
}

#[test]
fn generated_union_constructors_initialize_selected_field() {
    let float = vk::ClearColorValue::float32([1.0, 0.5, 0.25, 1.0]);
    assert_eq!(unsafe { float.float32 }, [1.0, 0.5, 0.25, 1.0]);

    let int = vk::ClearColorValue::int32([1, 2, 3, 4]);
    assert_eq!(unsafe { int.int32 }, [1, 2, 3, 4]);

    let uint = vk::ClearColorValue::uint32([5, 6, 7, 8]);
    assert_eq!(unsafe { uint.uint32 }, [5, 6, 7, 8]);
}

#[test]
fn counted_slice_setters_update_count_and_pointer() {
    let layouts = [
        vk::DescriptorSetLayout::null(),
        vk::DescriptorSetLayout::null(),
    ];
    let info = vk::IndirectExecutionSetShaderLayoutInfoEXT::default().set_layouts(&layouts);
    assert_eq!(info.set_layout_count, layouts.len() as u32);
    assert_eq!(info.p_set_layouts, layouts.as_ptr());
}

#[test]
fn generated_defaults_use_vulkan_neutral_values() {
    let barrier = vk::BufferMemoryBarrier2::default();
    assert_eq!(barrier.s_type, vk::StructureType::BUFFER_MEMORY_BARRIER_2);
    assert!(barrier.p_next.is_null());
    assert_eq!(barrier.src_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    assert_eq!(barrier.dst_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    assert_eq!(barrier.buffer, vk::Buffer::null());
    assert_eq!(barrier.src_access_mask, vk::AccessFlags2::empty());
    assert_eq!(barrier.dst_access_mask, vk::AccessFlags2::empty());

    let info = vk::DeviceQueueCreateInfo::default();
    assert_eq!(info.s_type, vk::StructureType::DEVICE_QUEUE_CREATE_INFO);
    assert!(info.p_next.is_null());
    assert_eq!(info.flags, vk::DeviceQueueCreateFlags::empty());
    assert!(info.p_queue_priorities.is_null());

    let image = vk::ImageCreateInfo::default();
    assert_eq!(image.s_type, vk::StructureType::IMAGE_CREATE_INFO);
    assert_eq!(image.mip_levels, 1);
    assert_eq!(image.array_layers, 1);
    assert_eq!(image.samples, vk::SampleCountFlagBits::_1);

    let multisample = vk::PipelineMultisampleStateCreateInfo::default();
    assert_eq!(
        multisample.rasterization_samples,
        vk::SampleCountFlagBits::_1
    );
}

#[test]
fn extension_name_constants_match_vulkan_strings() {
    assert_eq!(
        vk::KHR_SWAPCHAIN_NAME.to_bytes_with_nul(),
        b"VK_KHR_swapchain\0"
    );
}

#[test]
fn command_wrapper_method_items_have_pointer_free_shapes() {
    type CmdBindDescriptorSets = unsafe fn(
        &Device,
        vk::CommandBuffer,
        vk::PipelineBindPoint,
        vk::PipelineLayout,
        u32,
        Option<&[vk::DescriptorSet]>,
        &[u32],
    );

    type CreateGraphicsPipelines =
        unsafe fn(
            &Device,
            vk::PipelineCache,
            &[vk::GraphicsPipelineCreateInfo<'_>],
            Option<&vk::AllocationCallbacks<'_>>,
        )
            -> core::result::Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::VkResult)>;

    let _: CmdBindDescriptorSets = Device::cmd_bind_descriptor_sets;
    let _: CreateGraphicsPipelines = Device::create_graphics_pipelines;
}
