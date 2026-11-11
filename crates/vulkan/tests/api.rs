//! Shape of the generated API, checked without a Vulkan implementation.

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
    // A stale pointer in the pushed struct must not be followed.
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

    // Out-of-range values are truncated to the field width.
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
    // A count beyond the array is clamped instead of reading out of bounds.
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
fn debug_prints_names() {
    assert_eq!(
        format!("{:?}", vk::Format::R8G8B8A8_UNORM),
        "R8G8B8A8_UNORM"
    );
    assert_eq!(format!("{:?}", vk::Format::from_raw(-5)), "Format(-5)");
    assert_eq!(
        format!("{:?}", vk::VkResult::ERROR_OUT_OF_DATE_KHR),
        "ERROR_OUT_OF_DATE_KHR"
    );
    assert_eq!(
        vk::VkResult::ERROR_DEVICE_LOST.to_string(),
        "ERROR_DEVICE_LOST"
    );
    assert_eq!(
        format!(
            "{:?}",
            vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::COLOR_ATTACHMENT
        ),
        "SAMPLED | COLOR_ATTACHMENT"
    );
    assert_eq!(format!("{:?}", vk::ImageUsageFlags::empty()), "empty()");
    assert_eq!(
        format!("{:?}", vk::ImageUsageFlags::from_raw(0x4000_0001)),
        "TRANSFER_SRC | 0x40000000"
    );
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
    // Single bits combine into the flags type.
    let stages: vk::ShaderStageFlags =
        vk::ShaderStageFlagBits::VERTEX | vk::ShaderStageFlagBits::FRAGMENT;
    assert_eq!(stages.as_raw(), 0x11);
    const _: vk::ImageUsageFlags = vk::ImageUsageFlags::SAMPLED.union(vk::ImageUsageFlags::STORAGE);
}

#[test]
fn results_split_success_and_error_codes() {
    assert_eq!(vk::VkResult::SUCCESS.into_result(), Ok(()));
    assert_eq!(vk::VkResult::SUBOPTIMAL_KHR.into_result(), Ok(()));
    assert_eq!(
        vk::VkResult::ERROR_DEVICE_LOST.into_result(),
        Err(vk::VkResult::ERROR_DEVICE_LOST)
    );
    fn takes_error(_: &dyn std::error::Error) {}
    takes_error(&vk::VkResult::ERROR_DEVICE_LOST);
}

#[test]
fn handles_know_their_object_type() {
    assert_eq!(vk::Buffer::TYPE, vk::ObjectType::BUFFER);
    assert_eq!(vk::SwapchainKHR::TYPE, vk::ObjectType::SWAPCHAIN_KHR);
    assert_eq!(vk::Buffer::from_raw_u64(7).as_raw_u64(), 7);
    assert!(vk::Device::default().is_null());
}

#[test]
fn versions_and_extension_names() {
    assert_eq!(vk::API_VERSION_1_3, vk::make_api_version(0, 1, 3, 0));
    assert_eq!(vk::api_version_major(vk::HEADER_VERSION_COMPLETE), 1);
    assert_eq!(
        vk::api_version_patch(vk::HEADER_VERSION_COMPLETE),
        vk::HEADER_VERSION
    );
    assert!(vk::HEADERS_TAG.starts_with("vulkan-sdk-"));
    assert_eq!(vk::KHR_SWAPCHAIN_NAME, c"VK_KHR_swapchain");
    assert_eq!(
        vk::EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME,
        c"VK_EXT_image_drm_format_modifier"
    );
}

/// The crate version records the Vulkan-Headers tag it was generated from.
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

/// Array parameters are pointers in the C ABI, never by-value arrays.
#[test]
fn array_parameters_use_the_c_abi() {
    let _: vk::PFN_vkCmdSetBlendConstants =
        None::<unsafe extern "system" fn(vk::CommandBuffer, *const [f32; 4])>;
}

/// `vkGetExternalComputeQueueDataNV` takes a child handle of the device, so it is loaded
/// with `vkGetDeviceProcAddr`.
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

/// Direct-to-display output (DRM/KMS). Acquiring a display needs DRM master, so only
/// the shapes are checked here.
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

/// Present mode switching and tearing control without recreating the swapchain.
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

    // The EXT names are aliases of the promoted KHR types.
    let _: vk::SwapchainPresentModesCreateInfoEXT<'_> =
        vk::SwapchainPresentModesCreateInfoKHR::default();
    let _ = Device::set_hdr_metadata_ext;
    let _ = Device::release_swapchain_images_khr;
}

/// `vk::Format::info` is generated from the registry's `<formats>`, which the C headers
/// do not carry.
#[test]
fn formats_know_their_texel_layout() {
    use vk::{ChromaSubsampling, ComponentName, NumericFormat};

    assert!(vk::Format::UNDEFINED.info().is_none());
    assert!(vk::Format::from_raw(-1).info().is_none());

    let rgba = vk::Format::R8G8B8A8_UNORM.info().unwrap();
    assert_eq!(rgba.class, "32-bit");
    assert_eq!(rgba.block_size, 4);
    assert_eq!(rgba.block_extent, [1, 1, 1]);
    assert_eq!(rgba.components.len(), 4);
    assert_eq!(rgba.components[3].name, ComponentName::A);
    assert_eq!(rgba.components[0].bits, Some(8));
    assert_eq!(rgba.components[0].numeric_format, NumericFormat::Unorm);
    assert_eq!(rgba.plane_count(), 1);
    assert_eq!(rgba.aspects(), vk::ImageAspectFlags::COLOR);
    assert!(rgba.has_alpha() && !rgba.is_srgb() && !rgba.is_compressed());
    assert!(vk::Format::B8G8R8A8_SRGB.info().unwrap().is_srgb());
    // Same class: one can be viewed as the other.
    assert_eq!(vk::Format::B8G8R8A8_SRGB.info().unwrap().class, rgba.class);

    let depth_stencil = vk::Format::D24_UNORM_S8_UINT.info().unwrap();
    assert_eq!(
        depth_stencil.aspects(),
        vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL
    );
    assert_eq!(
        vk::Format::D32_SFLOAT.info().unwrap().aspects(),
        vk::ImageAspectFlags::DEPTH
    );

    // NV12: full-resolution luma, half-resolution interleaved chroma.
    let nv12 = vk::Format::G8_B8R8_2PLANE_420_UNORM.info().unwrap();
    assert_eq!(nv12.chroma, Some(ChromaSubsampling::_420));
    assert_eq!(nv12.plane_count(), 2);
    assert!(nv12.is_multiplanar());
    assert_eq!(nv12.planes[0].compatible, vk::Format::R8_UNORM);
    assert_eq!(nv12.planes[1].compatible, vk::Format::R8G8_UNORM);
    let full = vk::Extent2D {
        width: 1920,
        height: 1080,
    };
    let half = vk::Extent2D {
        width: 960,
        height: 540,
    };
    // `assert!` instead of `assert_eq!`: structs have `Debug` only with the `debug` feature.
    assert!(nv12.plane_extent(0, full) == Some(full));
    assert!(nv12.plane_extent(1, full) == Some(half));
    assert!(nv12.plane_extent(2, full).is_none());
    assert_eq!(nv12.plane_aspect(1), Some(vk::ImageAspectFlags::PLANE_1));
    assert_eq!(nv12.plane_aspect(2), None);
    assert_eq!(nv12.aspects(), vk::ImageAspectFlags::COLOR);
    assert!(rgba.plane_extent(0, full) == Some(full));
    assert_eq!(rgba.plane_aspect(0), None);

    let bc1 = vk::Format::BC1_RGB_UNORM_BLOCK.info().unwrap();
    assert_eq!(bc1.compression, Some("BC"));
    assert_eq!(bc1.block_extent, [4, 4, 1]);
    assert_eq!((bc1.block_size, bc1.texels_per_block), (8, 16));
    assert_eq!(bc1.components[0].bits, None);

    let packed = vk::Format::A2B10G10R10_UNORM_PACK32.info().unwrap();
    assert_eq!(packed.packed, Some(32));
    assert_eq!(
        packed
            .components
            .iter()
            .map(|c| c.bits.unwrap())
            .collect::<Vec<_>>(),
        [2, 10, 10, 10]
    );

    // `info` is usable in constants.
    const BLOCK: u32 = match vk::Format::R16G16B16A16_SFLOAT.info() {
        Some(info) => info.block_size,
        None => 0,
    };
    assert_eq!(BLOCK, 8);
}

/// Sanity of the whole table: in an uncompressed single-plane format the components fit
/// in the block. They need not fill it: `X8_D24`, `R10X6` and `E5B9G9R9` have bits that
/// are not a component.
#[test]
fn format_component_bits_fit_in_the_block() {
    let mut checked = 0;
    for raw in (0..200).chain(1_000_000_000..1_001_000_000) {
        let format = vk::Format::from_raw(raw);
        let Some(info) = format.info() else { continue };
        assert!(
            info.block_size > 0 && info.texels_per_block > 0,
            "{format:?}"
        );
        assert!(!info.components.is_empty(), "{format:?}");
        if info.is_compressed() || info.is_multiplanar() {
            continue;
        }
        let bits = info.components.iter().map(|c| c.bits.unwrap()).sum::<u32>();
        assert!(bits <= info.block_size * 8, "{format:?}");
        checked += 1;
    }
    assert!(checked > 100);
}

/// Extension dependencies come from the registry's `depends` attribute.
#[test]
fn extensions_know_their_dependencies() {
    use std::ffi::CStr;
    use vk::{Dependency, ExtensionInfo, ExtensionLevel};

    // The table is sorted, which `get` relies on, and has no duplicates.
    assert!(
        vk::EXTENSIONS
            .windows(2)
            .all(|pair| pair[0].name < pair[1].name)
    );
    for info in vk::EXTENSIONS {
        assert_eq!(ExtensionInfo::get(info.name), Some(info));
    }
    assert!(ExtensionInfo::get(c"VK_KHR_does_not_exist").is_none());

    let swapchain = ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME).unwrap();
    assert_eq!(swapchain.level, ExtensionLevel::Device);
    assert_eq!(swapchain.spec_version, vk::KHR_SWAPCHAIN_SPEC_VERSION);
    assert_eq!(
        swapchain.depends,
        Some(Dependency::Extension(c"VK_KHR_surface"))
    );
    let enabled = |list: &'static [&'static CStr]| move |name: &CStr| list.contains(&name);
    assert!(!swapchain.dependencies_met(vk::API_VERSION_1_4, enabled(&[])));
    assert!(swapchain.dependencies_met(vk::API_VERSION_1_0, enabled(&[vk::KHR_SURFACE_NAME])));

    let surface = ExtensionInfo::get(vk::KHR_SURFACE_NAME).unwrap();
    assert_eq!(surface.level, ExtensionLevel::Instance);
    assert!(surface.dependencies_met(vk::API_VERSION_1_0, enabled(&[])));

    // VK_EXT_descriptor_heap: usable on 1.4 alone; on 1.3 it needs VK_KHR_maintenance5,
    // and the buffer device address dependency is covered by the 1.2 promotion.
    let heap = ExtensionInfo::get(vk::EXT_DESCRIPTOR_HEAP_NAME).unwrap();
    assert!(heap.dependencies_met(vk::API_VERSION_1_4, enabled(&[])));
    assert!(!heap.dependencies_met(vk::API_VERSION_1_3, enabled(&[])));
    assert!(heap.dependencies_met(vk::API_VERSION_1_3, enabled(&[vk::KHR_MAINTENANCE_5_NAME])));
    assert!(!heap.dependencies_met(vk::API_VERSION_1_1, enabled(&[vk::KHR_MAINTENANCE_5_NAME])));
    assert_eq!(
        heap.depends.unwrap().to_string(),
        "(((VK_KHR_extended_flags,VK_KHR_maintenance5)+(VK_KHR_buffer_device_address,VK_VERSION_1_2)),VK_VERSION_1_4)"
    );

    // Promotion: a dependency on a promoted extension is met by the core version.
    let maintenance5 = ExtensionInfo::get(vk::KHR_MAINTENANCE_5_NAME).unwrap();
    assert_eq!(maintenance5.promoted_to_version, Some(vk::API_VERSION_1_4));
    let dynamic_rendering = Dependency::Extension(c"VK_KHR_dynamic_rendering");
    assert!(dynamic_rendering.is_met(vk::API_VERSION_1_3, &|_: &CStr| false));
    assert!(!dynamic_rendering.is_met(vk::API_VERSION_1_2, &|_: &CStr| false));

    // Platform extensions are listed whatever features are enabled.
    assert!(ExtensionInfo::get(c"VK_KHR_win32_surface").is_some());
}

/// Member and parameter names that are Rust keywords get a conventional spelling
/// instead of a raw identifier: `type` is `ty`.
#[test]
fn keyword_members_have_plain_names() {
    let size = vk::DescriptorPoolSize::default()
        .ty(vk::DescriptorType::SAMPLED_IMAGE)
        .descriptor_count(4);
    assert_eq!(size.ty, vk::DescriptorType::SAMPLED_IMAGE);
}
