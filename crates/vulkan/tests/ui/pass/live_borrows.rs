//! The same shapes as `ui/fail`, with every borrow alive: all of this must compile.
use std::ffi::CString;
use vulkan::vk;

fn use_device_info(_: &vk::DeviceCreateInfo<'_>) {}
fn query(_: &mut vk::PhysicalDeviceFeatures2<'_>) {}
fn query_modifiers(_: &mut vk::FormatProperties2<'_>) {}

fn main() {
    let priorities = [1.0];
    let queues = [vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities)];
    let features = vk::PhysicalDeviceFeatures::default();
    let owned = CString::new("VK_KHR_swapchain").unwrap();
    let names = [vk::CStrPtr::new(&owned), vk::EXT_HDR_METADATA_NAME.into()];
    let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut vulkan13 = vk::PhysicalDeviceVulkan13Features::default();
    let info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queues)
        .enabled_features(&features)
        .enabled_extension_names(&names)
        .push_next(&mut vulkan12)
        .push_next(&mut vulkan13);
    use_device_info(&info);

    let app_name = CString::new("app").unwrap();
    let app = vk::ApplicationInfo::default().application_name(&app_name);
    let _instance_info = vk::InstanceCreateInfo::default().application_info(&app);

    let geometry = vk::AccelerationStructureGeometryKHR::default();
    let pointers = [&geometry];
    let _build = vk::AccelerationStructureBuildGeometryInfoKHR::default().geometries_ptrs(&pointers);

    let data = [0u8; 16];
    let _specialization = vk::SpecializationInfo::default().data(&data);

    let image = vk::DescriptorImageInfo::default();
    let _get = vk::DescriptorGetInfoEXT::default().data(vk::DescriptorDataEXT::sampled_image(&image));

    // Output chains: once the root is no longer used, the chained structs are readable.
    let mut out12 = vk::PhysicalDeviceVulkan12Features::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default().push_next(&mut out12);
    query(&mut features2);
    let _anisotropy = features2.features.sampler_anisotropy;
    let _timeline = out12.timeline_semaphore;

    // Output arrays: the two-call idiom inside a pNext struct.
    let mut modifiers = [vk::DrmFormatModifierPropertiesEXT::default(); 4];
    let mut list = vk::DrmFormatModifierPropertiesListEXT::default()
        .drm_format_modifier_properties(&mut modifiers);
    let mut properties = vk::FormatProperties2::default().push_next(&mut list);
    query_modifiers(&mut properties);
    let count = list.drm_format_modifier_count as usize;
    let _written = &modifiers[..count.min(modifiers.len())];

    // Structs without pointers have no lifetime and can be stored anywhere.
    static EXTENT: vk::Extent2D = vk::Extent2D {
        width: 1,
        height: 1,
    };
    let _ = EXTENT;
}
