use std::ffi::CStr;
use std::mem::{align_of, size_of};
use vulkan::vk::{self, Handle};
use vulkan::{Device, Entry, Instance};

#[test]
fn shader_module_code_sets_size_in_bytes() {
    let code = [0x0723_0203_u32, 0, 0, 0];
    let info = vk::ShaderModuleCreateInfo::default().code(&code);
    assert_eq!(info.code_size, 16);
    assert_eq!(info.p_code, code.as_ptr());
}

#[test]
fn slice_setters_set_count_and_pointer() {
    let priorities = [1.0, 0.5];
    let info = vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities);
    assert_eq!(info.queue_count, 2);
    assert_eq!(info.p_queue_priorities, priorities.as_ptr());
}

#[test]
fn arrays_sharing_a_count_must_agree() {
    let semaphores = [vk::Semaphore::null(); 2];
    let stages = [vk::PipelineStageFlags::empty(); 2];
    let info = vk::SubmitInfo::default()
        .wait_semaphores(&semaphores)
        .wait_dst_stage_mask(&stages);
    assert_eq!(info.wait_semaphore_count, 2);
}

#[test]
#[should_panic(expected = "every array sharing `waitSemaphoreCount` must have the same length")]
fn arrays_sharing_a_count_panic_on_mismatch() {
    let semaphores = [vk::Semaphore::null(); 2];
    let stages = [vk::PipelineStageFlags::empty(); 3];
    let _ = vk::SubmitInfo::default()
        .wait_semaphores(&semaphores)
        .wait_dst_stage_mask(&stages);
}

#[test]
fn string_arrays_are_pointer_compatible() {
    assert_eq!(size_of::<vk::CStrPtr<'_>>(), size_of::<*const u8>());
    assert_eq!(align_of::<vk::CStrPtr<'_>>(), align_of::<*const u8>());
    let names = [
        vk::CStrPtr::new(vk::KHR_SWAPCHAIN_NAME),
        vk::EXT_HDR_METADATA_NAME.into(),
    ];
    let info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
    assert_eq!(info.enabled_extension_count, 2);
    let first = unsafe { CStr::from_ptr(*info.pp_enabled_extension_names) };
    assert_eq!(first, c"VK_KHR_swapchain");
    assert_eq!(names[1].as_c_str(), c"VK_EXT_hdr_metadata");
}

#[test]
fn push_next_prepends_and_overwrites_the_pushed_struct_p_next() {
    let mut vulkan11 = vk::PhysicalDeviceVulkan11Features::default();
    let mut vulkan12 = vk::PhysicalDeviceVulkan12Features {
        p_next: std::ptr::dangling_mut(),
        ..Default::default()
    };
    let vulkan11_ptr = std::ptr::from_mut(&mut vulkan11).cast::<std::ffi::c_void>();
    let vulkan12_ptr = std::ptr::from_mut(&mut vulkan12).cast::<std::ffi::c_void>();

    let features = vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut vulkan11)
        .push_next(&mut vulkan12);
    assert_eq!(features.p_next, vulkan12_ptr);
    assert_eq!(vulkan12.p_next, vulkan11_ptr);
    assert!(vulkan11.p_next.is_null());
}

struct ExtendsDeviceCreateInfo<T>(std::marker::PhantomData<T>);

trait FallbackWhenNotExtending {
    const EXTENDS: bool = false;
}

impl<T> FallbackWhenNotExtending for ExtendsDeviceCreateInfo<T> {}

impl<T: vk::Extends<vk::DeviceCreateInfo<'static>>> ExtendsDeviceCreateInfo<T> {
    const EXTENDS: bool = true;
}

#[test]
fn only_structs_the_registry_lists_as_extending_the_root_can_be_pushed() {
    let extends = [
        ExtendsDeviceCreateInfo::<vk::PhysicalDeviceVulkan12Features<'static>>::EXTENDS,
        ExtendsDeviceCreateInfo::<vk::PresentInfoKHR<'static>>::EXTENDS,
    ];
    assert_eq!(extends, [true, false]);
}

#[test]
fn defaults_use_vulkan_neutral_values() {
    let barrier = vk::ImageMemoryBarrier2::default();
    assert_eq!(barrier.s_type, vk::StructureType::IMAGE_MEMORY_BARRIER_2);
    assert!(barrier.p_next.is_null());
    assert_eq!(barrier.src_queue_family_index, vk::QUEUE_FAMILY_IGNORED);
    assert_eq!(barrier.dst_queue_family_index, vk::QUEUE_FAMILY_IGNORED);

    let image = vk::ImageCreateInfo::default();
    assert_eq!(image.mip_levels, 1);
    assert_eq!(image.array_layers, 1);
    assert_eq!(image.samples, vk::SampleCountFlagBits::_1);
    assert_eq!(image.usage, vk::ImageUsageFlags::empty());
}

#[test]
fn plain_structs_are_plain_values() {
    let extent = vk::Extent2D {
        width: 1920,
        height: 1080,
    };
    assert!(extent == vk::Extent2D::default().width(1920).height(1080));
    let mut set = std::collections::HashSet::new();
    set.insert(extent);
    assert!(set.contains(&extent));
    assert_eq!(size_of::<vk::Extent2D>(), 8);
}

#[test]
fn bool32_setters_take_bool() {
    let features = vk::PhysicalDeviceFeatures::default().sampler_anisotropy(true);
    assert_eq!(features.sampler_anisotropy, vk::TRUE);
}

#[test]
fn bit_fields_round_trip() {
    let instance = vk::AccelerationStructureInstanceKHR::default()
        .instance_custom_index(0x00ab_cdef)
        .mask(0xff)
        .instance_shader_binding_table_record_offset(3)
        .flags(vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE);
    assert_eq!(instance.bitfield0, 0xffab_cdef);
    assert_eq!(instance.get_instance_custom_index(), 0x00ab_cdef);
    assert_eq!(instance.get_mask(), 0xff);
    assert_eq!(
        instance.get_instance_shader_binding_table_record_offset(),
        3
    );
    assert_eq!(
        instance.get_flags(),
        vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE
    );
    assert_eq!(size_of::<vk::AccelerationStructureInstanceKHR>(), 64);

    let truncated = vk::AccelerationStructureInstanceKHR::default().mask(0x1ff);
    assert_eq!(truncated.get_mask(), 0xff);
    assert_eq!(truncated.get_instance_custom_index(), 0);

    let sps = vk::StdVideoH264SpsFlags::default()
        .frame_mbs_only_flag(1)
        .vui_parameters_present_flag(1);
    assert_eq!(size_of::<vk::StdVideoH264SpsFlags>(), 4);
    assert_eq!(sps.get_frame_mbs_only_flag(), 1);
    assert_eq!(sps.get_constraint_set0_flag(), 0);
}

#[test]
fn union_constructors_select_a_member() {
    let float = vk::ClearColorValue::float32([1.0, 0.5, 0.25, 1.0]);
    assert_eq!(unsafe { float.float32 }, [1.0, 0.5, 0.25, 1.0]);
    let clear = vk::ClearValue::color(float);
    assert_eq!(unsafe { clear.color.float32 }, [1.0, 0.5, 0.25, 1.0]);
}

#[test]
fn fixed_arrays_have_typed_readers() {
    let mut properties = vk::PhysicalDeviceProperties::default();
    for (dst, src) in properties.device_name.iter_mut().zip(b"lavapipe\0") {
        *dst = *src as _;
    }
    assert_eq!(properties.device_name_as_c_str().unwrap(), c"lavapipe");

    let memory = vk::PhysicalDeviceMemoryProperties::default().memory_type_count(2);
    assert_eq!(memory.memory_types_as_slice().len(), 2);
    let memory = memory.memory_type_count(1000);
    assert_eq!(memory.memory_types_as_slice().len(), vk::MAX_MEMORY_TYPES);
}

#[test]
fn enum_names_drop_type_prefix_and_own_vendor_tag() {
    assert_eq!(vk::PresentModeKHR::IMMEDIATE.as_raw(), 0);
    assert_eq!(vk::PresentModeKHR::FIFO_RELAXED.as_raw(), 3);
    assert_eq!(vk::ColorSpaceKHR::SRGB_NONLINEAR.as_raw(), 0);
    assert_eq!(vk::ColorSpaceKHR::HDR10_ST2084_EXT.as_raw(), 1000104008);
    assert_eq!(vk::CompositeAlphaFlagBitsKHR::OPAQUE.as_raw(), 1);
    assert_eq!(
        vk::ExternalMemoryHandleTypeFlags::DMA_BUF_EXT.as_raw(),
        0x200
    );
}

#[test]
fn handles_and_structs_print_their_type_and_contents() {
    assert_eq!(format!("{:?}", vk::Buffer::from_raw(0x10)), "Buffer(0x10)");
    #[cfg(feature = "debug")]
    assert_eq!(
        format!("{:?}", vk::Extent2D::default()),
        "Extent2D { width: 0, height: 0 }"
    );
}

#[test]
fn flags_have_set_operations() {
    let mut usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::STORAGE;
    assert!(usage.contains(vk::ImageUsageFlags::SAMPLED));
    assert!(usage.intersects(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC));
    assert!(!usage.contains(vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::TRANSFER_SRC));
    usage &= !vk::ImageUsageFlags::STORAGE;
    assert_eq!(usage, vk::ImageUsageFlags::SAMPLED);
    usage |= vk::ImageUsageFlagBits::TRANSFER_DST;
    assert_eq!(
        usage ^ vk::ImageUsageFlags::SAMPLED,
        vk::ImageUsageFlags::TRANSFER_DST
    );
    let stages: vk::ShaderStageFlags =
        vk::ShaderStageFlagBits::VERTEX | vk::ShaderStageFlagBits::FRAGMENT;
    assert_eq!(stages.as_raw(), 0x11);
    const _: vk::ImageUsageFlags = vk::ImageUsageFlags::SAMPLED.union(vk::ImageUsageFlags::STORAGE);
}

#[test]
fn handles_know_their_object_type() {
    assert_eq!(vk::Buffer::TYPE, vk::ObjectType::BUFFER);
    assert_eq!(vk::SwapchainKHR::TYPE, vk::ObjectType::SWAPCHAIN_KHR);
    assert_eq!(vk::Buffer::from_raw_u64(7).as_raw_u64(), 7);
    assert!(vk::Device::default().is_null());
}

#[test]
fn headers_tag_and_extension_names() {
    assert!(vk::HEADERS_TAG.starts_with("vulkan-sdk-"));
    assert_eq!(vk::KHR_SWAPCHAIN_NAME, c"VK_KHR_swapchain");
    assert_eq!(
        vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME,
        c"VK_EXT_image_drm_format_modifier"
    );
}

#[test]
fn crate_version_metadata_matches_generated_headers_tag() {
    let (_, metadata) = env!("CARGO_PKG_VERSION")
        .split_once('+')
        .expect("version has build metadata");
    assert_eq!(metadata, vk::HEADERS_TAG);
}

#[test]
fn everything_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Entry>();
    assert_send_sync::<Instance>();
    assert_send_sync::<Device>();
    assert_send_sync::<vk::Device>();
    assert_send_sync::<vk::Buffer>();
    assert_send_sync::<vk::BufferCreateInfo<'static>>();
    assert_send_sync::<vk::DescriptorDataEXT<'static>>();
    assert_send_sync::<vk::CStrPtr<'static>>();
    assert_send_sync::<vk::DeviceFn>();
}

#[test]
fn command_wrappers_have_pointer_free_signatures() {
    type CmdBindDescriptorSets = unsafe fn(
        &Device,
        vk::CommandBuffer,
        vk::PipelineBindPoint,
        vk::PipelineLayout,
        u32,
        &[vk::DescriptorSet],
        &[u32],
    );
    type CmdBindVertexBuffers = unsafe fn(&Device, vk::CommandBuffer, u32, &[vk::Buffer], &[u64]);
    type CmdSetBlendConstants = unsafe fn(&Device, vk::CommandBuffer, &[f32; 4]);
    type CreateGraphicsPipelines =
        unsafe fn(
            &Device,
            vk::PipelineCache,
            &[vk::GraphicsPipelineCreateInfo<'_>],
            Option<&vk::AllocationCallbacks<'_>>,
        ) -> Result<Vec<vk::Pipeline>, (Vec<vk::Pipeline>, vk::VkResult)>;
    type AcquireNextImage = unsafe fn(
        &Device,
        vk::SwapchainKHR,
        u64,
        vk::Semaphore,
        vk::Fence,
    ) -> vk::Result<Option<(u32, bool)>>;
    type QueuePresent = unsafe fn(&Device, vk::Queue, &vk::PresentInfoKHR<'_>) -> vk::Result<bool>;
    type WaitForFences = unsafe fn(&Device, &[vk::Fence], bool, u64) -> vk::Result<vk::VkResult>;
    type GetMemoryFd =
        unsafe fn(&Device, &vk::MemoryGetFdInfoKHR<'_>) -> vk::Result<std::ffi::c_int>;
    type GetSurfaceFormats = unsafe fn(
        &Instance,
        vk::PhysicalDevice,
        vk::SurfaceKHR,
    ) -> vk::Result<Vec<vk::SurfaceFormatKHR>>;
    type GetFeatures2 =
        unsafe fn(&Instance, vk::PhysicalDevice, &mut vk::PhysicalDeviceFeatures2<'_>);
    type GetSurfaceSupport =
        unsafe fn(&Instance, vk::PhysicalDevice, u32, vk::SurfaceKHR) -> vk::Result<bool>;
    type EnumerateExtensions =
        unsafe fn(&Entry, Option<&CStr>) -> vk::Result<Vec<vk::ExtensionProperties>>;

    let _: CmdBindDescriptorSets = Device::cmd_bind_descriptor_sets;
    let _: CmdBindVertexBuffers = Device::cmd_bind_vertex_buffers;
    let _: CmdSetBlendConstants = Device::cmd_set_blend_constants;
    let _: CreateGraphicsPipelines = Device::create_graphics_pipelines;
    let _: AcquireNextImage = Device::acquire_next_image_khr;
    let _: QueuePresent = Device::queue_present_khr;
    let _: WaitForFences = Device::wait_for_fences;
    let _: GetMemoryFd = Device::get_memory_fd_khr;
    let _: GetSurfaceFormats = Instance::get_physical_device_surface_formats_khr;
    let _: GetFeatures2 = Instance::get_physical_device_features2;
    let _: GetSurfaceSupport = Instance::get_physical_device_surface_support_khr;
    let _: EnumerateExtensions = Entry::enumerate_instance_extension_properties;
}

#[test]
fn array_parameters_use_the_c_abi() {
    let _: vk::PFN_vkCmdSetBlendConstants =
        None::<unsafe extern "system" fn(vk::CommandBuffer, *const [f32; 4])>;
}

#[test]
fn commands_on_child_handles_are_device_level() {
    let _ = Device::get_external_compute_queue_data_nv;
    let _ = |fp: &vk::DeviceFn| fp.get_external_compute_queue_data_nv;
    let _ = |fp: &vk::InstanceFn| fp.get_device_proc_addr;
}

#[cfg(feature = "wayland")]
#[test]
fn wayland_surface_takes_raw_display_and_surface() {
    let display = std::ptr::dangling_mut::<std::ffi::c_void>();
    let info = vk::WaylandSurfaceCreateInfoKHR::default()
        .display(display)
        .surface(display);
    assert_eq!(info.display, display);
    let _ = Instance::create_wayland_surface_khr;
}

#[cfg(all(feature = "ggp", target_pointer_width = "64"))]
#[test]
fn ggp_structs_have_the_layout_of_the_ggp_types() {
    use std::mem::offset_of;

    assert_eq!(
        size_of::<vk::GgpStreamDescriptor>(),
        size_of::<u32>(),
        "GgpStreamDescriptor is a uint32_t"
    );
    assert_eq!(
        size_of::<vk::GgpFrameToken>(),
        size_of::<u64>(),
        "GgpFrameToken is a uint64_t"
    );

    type StreamDescriptorInfo = vk::StreamDescriptorSurfaceCreateInfoGGP<'static>;
    assert_eq!(offset_of!(StreamDescriptorInfo, flags), 16);
    assert_eq!(
        offset_of!(StreamDescriptorInfo, stream_descriptor),
        20,
        "streamDescriptor follows the 32-bit flags without padding"
    );
    assert_eq!(size_of::<StreamDescriptorInfo>(), 24);
    assert_eq!(align_of::<StreamDescriptorInfo>(), 8);

    type FrameToken = vk::PresentFrameTokenGGP<'static>;
    assert_eq!(offset_of!(FrameToken, frame_token), 16);
    assert_eq!(size_of::<FrameToken>(), 24);
    assert_eq!(align_of::<FrameToken>(), 8);

    let info = StreamDescriptorInfo::default().stream_descriptor(u32::MAX);
    assert_eq!(info.stream_descriptor, u32::MAX);
    let token = FrameToken::default().frame_token(u64::MAX);
    assert_eq!(token.frame_token, u64::MAX);
}

#[test]
fn display_commands_have_expected_signatures() {
    type AcquireDrmDisplay =
        unsafe fn(&Instance, vk::PhysicalDevice, i32, vk::DisplayKHR) -> vk::Result<()>;
    type GetDrmDisplay =
        unsafe fn(&Instance, vk::PhysicalDevice, i32, u32) -> vk::Result<vk::DisplayKHR>;
    type CreateDisplayPlaneSurface = unsafe fn(
        &Instance,
        &vk::DisplaySurfaceCreateInfoKHR<'_>,
        Option<&vk::AllocationCallbacks<'_>>,
    ) -> vk::Result<vk::SurfaceKHR>;
    type GetDisplays = unsafe fn(
        &Instance,
        vk::PhysicalDevice,
    ) -> vk::Result<Vec<vk::DisplayPropertiesKHR<'static>>>;

    let _: AcquireDrmDisplay = Instance::acquire_drm_display_ext;
    let _: GetDrmDisplay = Instance::get_drm_display_ext;
    let _: CreateDisplayPlaneSurface = Instance::create_display_plane_surface_khr;
    let _: GetDisplays = Instance::get_physical_device_display_properties_khr;
}

#[test]
fn swapchain_maintenance_structs_chain_into_swapchain_and_present() {
    let modes = [
        vk::PresentModeKHR::FIFO,
        vk::PresentModeKHR::FIFO_RELAXED,
        vk::PresentModeKHR::IMMEDIATE,
    ];
    let mut compatible = vk::SwapchainPresentModesCreateInfoKHR::default().present_modes(&modes);
    let swapchain = vk::SwapchainCreateInfoKHR::default()
        .present_mode(vk::PresentModeKHR::FIFO)
        .image_color_space(vk::ColorSpaceKHR::HDR10_ST2084_EXT)
        .push_next(&mut compatible);
    assert!(!swapchain.p_next.is_null());
    assert_eq!(compatible.present_mode_count, 3);

    let per_present = [vk::PresentModeKHR::IMMEDIATE];
    let mut mode_info = vk::SwapchainPresentModeInfoKHR::default().present_modes(&per_present);
    let swapchains = [vk::SwapchainKHR::null()];
    let indices = [0];
    let present = vk::PresentInfoKHR::default()
        .swapchains(&swapchains)
        .image_indices(&indices)
        .push_next(&mut mode_info);
    assert_eq!(present.swapchain_count, 1);

    let _: vk::SwapchainPresentModesCreateInfoEXT<'_> =
        vk::SwapchainPresentModesCreateInfoKHR::default();
    let _ = Device::set_hdr_metadata_ext;
    let _ = Device::release_swapchain_images_khr;
}

#[test]
fn keyword_members_have_plain_names() {
    let size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::SAMPLED_IMAGE)
        .descriptor_count(4);
    assert_eq!(size.ty, vk::DescriptorType::SAMPLED_IMAGE);
}

#[test]
fn wrappers_are_cheap_to_clone() {
    assert!(std::mem::size_of::<vk::DeviceFn>() > 512 * std::mem::size_of::<usize>());
    assert!(std::mem::size_of::<Device>() <= 4 * std::mem::size_of::<usize>());
    assert!(std::mem::size_of::<Instance>() <= 4 * std::mem::size_of::<usize>());
}
