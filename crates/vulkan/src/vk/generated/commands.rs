#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]

use crate::vk::*;

use core::ffi::{c_char, c_void};

#[derive(Copy, Clone)]
pub struct EntryFn {
    pub create_instance: PFN_vkCreateInstance,
    pub enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    pub enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
    pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion,
    pub get_external_compute_queue_data_nv: PFN_vkGetExternalComputeQueueDataNV,
    pub get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
}

impl EntryFn {
    pub unsafe fn load(get_instance_proc_addr: PFN_vkGetInstanceProcAddr) -> Self {
        let get = get_instance_proc_addr.expect("vkGetInstanceProcAddr is required");
        Self {
            create_instance: crate::load_pfn(get(
                Instance::null(),
                crate::c_name_ptr(b"vkCreateInstance\0"),
            )),
            enumerate_instance_extension_properties: crate::load_pfn(get(
                Instance::null(),
                crate::c_name_ptr(b"vkEnumerateInstanceExtensionProperties\0"),
            )),
            enumerate_instance_layer_properties: crate::load_pfn(get(
                Instance::null(),
                crate::c_name_ptr(b"vkEnumerateInstanceLayerProperties\0"),
            )),
            enumerate_instance_version: crate::load_pfn(get(
                Instance::null(),
                crate::c_name_ptr(b"vkEnumerateInstanceVersion\0"),
            )),
            get_external_compute_queue_data_nv: crate::load_pfn(get(
                Instance::null(),
                crate::c_name_ptr(b"vkGetExternalComputeQueueDataNV\0"),
            )),
            get_instance_proc_addr,
        }
    }
}

#[derive(Copy, Clone)]
pub struct InstanceFn {
    pub acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    #[cfg(feature = "win32")]
    pub acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
    #[cfg(feature = "xlib-xrandr")]
    pub acquire_xlib_display_ext: PFN_vkAcquireXlibDisplayEXT,
    #[cfg(feature = "android")]
    pub create_android_surface_khr: PFN_vkCreateAndroidSurfaceKHR,
    pub create_debug_report_callback_ext: PFN_vkCreateDebugReportCallbackEXT,
    pub create_debug_utils_messenger_ext: PFN_vkCreateDebugUtilsMessengerEXT,
    pub create_device: PFN_vkCreateDevice,
    #[cfg(feature = "directfb")]
    pub create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
    pub create_display_mode_khr: PFN_vkCreateDisplayModeKHR,
    pub create_display_plane_surface_khr: PFN_vkCreateDisplayPlaneSurfaceKHR,
    pub create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
    #[cfg(feature = "ios")]
    pub create_ios_surface_mvk: PFN_vkCreateIOSSurfaceMVK,
    #[cfg(feature = "fuchsia")]
    pub create_image_pipe_surface_fuchsia: PFN_vkCreateImagePipeSurfaceFUCHSIA,
    #[cfg(feature = "macos")]
    pub create_mac_os_surface_mvk: PFN_vkCreateMacOSSurfaceMVK,
    #[cfg(feature = "metal")]
    pub create_metal_surface_ext: PFN_vkCreateMetalSurfaceEXT,
    #[cfg(feature = "screen")]
    pub create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    #[cfg(feature = "ggp")]
    pub create_stream_descriptor_surface_ggp: PFN_vkCreateStreamDescriptorSurfaceGGP,
    #[cfg(feature = "ohos")]
    pub create_surface_ohos: PFN_vkCreateSurfaceOHOS,
    #[cfg(feature = "ubm")]
    pub create_ubm_surface_sec: PFN_vkCreateUbmSurfaceSEC,
    #[cfg(feature = "vi")]
    pub create_vi_surface_nn: PFN_vkCreateViSurfaceNN,
    #[cfg(feature = "wayland")]
    pub create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    #[cfg(feature = "win32")]
    pub create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    #[cfg(feature = "xcb")]
    pub create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    #[cfg(feature = "xlib")]
    pub create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    pub debug_report_message_ext: PFN_vkDebugReportMessageEXT,
    pub destroy_debug_report_callback_ext: PFN_vkDestroyDebugReportCallbackEXT,
    pub destroy_debug_utils_messenger_ext: PFN_vkDestroyDebugUtilsMessengerEXT,
    pub destroy_instance: PFN_vkDestroyInstance,
    pub destroy_surface_khr: PFN_vkDestroySurfaceKHR,
    pub enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    pub enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    pub enumerate_physical_device_groups: PFN_vkEnumeratePhysicalDeviceGroups,
    pub enumerate_physical_device_groups_khr: PFN_vkEnumeratePhysicalDeviceGroupsKHR,
    pub enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    pub enumerate_physical_device_shader_instrumentation_metrics_arm:
        PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM,
    pub enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    pub get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    pub get_display_mode_properties2_khr: PFN_vkGetDisplayModeProperties2KHR,
    pub get_display_mode_properties_khr: PFN_vkGetDisplayModePropertiesKHR,
    pub get_display_plane_capabilities2_khr: PFN_vkGetDisplayPlaneCapabilities2KHR,
    pub get_display_plane_capabilities_khr: PFN_vkGetDisplayPlaneCapabilitiesKHR,
    pub get_display_plane_supported_displays_khr: PFN_vkGetDisplayPlaneSupportedDisplaysKHR,
    pub get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
    pub get_physical_device_calibrateable_time_domains_ext:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
    pub get_physical_device_calibrateable_time_domains_khr:
        PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
    pub get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
    pub get_physical_device_cooperative_matrix_properties_khr:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
    pub get_physical_device_cooperative_matrix_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
    pub get_physical_device_cooperative_vector_properties_nv:
        PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
    pub get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
    #[cfg(feature = "directfb")]
    pub get_physical_device_direct_fb_presentation_support_ext:
        PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
    pub get_physical_device_display_plane_properties2_khr:
        PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    pub get_physical_device_display_plane_properties_khr:
        PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    pub get_physical_device_display_properties2_khr: PFN_vkGetPhysicalDeviceDisplayProperties2KHR,
    pub get_physical_device_display_properties_khr: PFN_vkGetPhysicalDeviceDisplayPropertiesKHR,
    pub get_physical_device_external_buffer_properties:
        PFN_vkGetPhysicalDeviceExternalBufferProperties,
    pub get_physical_device_external_buffer_properties_khr:
        PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR,
    pub get_physical_device_external_fence_properties:
        PFN_vkGetPhysicalDeviceExternalFenceProperties,
    pub get_physical_device_external_fence_properties_khr:
        PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR,
    pub get_physical_device_external_image_format_properties_nv:
        PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
    pub get_physical_device_external_semaphore_properties:
        PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
    pub get_physical_device_external_semaphore_properties_khr:
        PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR,
    pub get_physical_device_external_tensor_properties_arm:
        PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
    pub get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    pub get_physical_device_features2: PFN_vkGetPhysicalDeviceFeatures2,
    pub get_physical_device_features2_khr: PFN_vkGetPhysicalDeviceFeatures2KHR,
    pub get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    pub get_physical_device_format_properties2: PFN_vkGetPhysicalDeviceFormatProperties2,
    pub get_physical_device_format_properties2_khr: PFN_vkGetPhysicalDeviceFormatProperties2KHR,
    pub get_physical_device_fragment_shading_rates_khr:
        PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR,
    pub get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    pub get_physical_device_image_format_properties2: PFN_vkGetPhysicalDeviceImageFormatProperties2,
    pub get_physical_device_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceImageFormatProperties2KHR,
    pub get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    pub get_physical_device_memory_properties2: PFN_vkGetPhysicalDeviceMemoryProperties2,
    pub get_physical_device_memory_properties2_khr: PFN_vkGetPhysicalDeviceMemoryProperties2KHR,
    pub get_physical_device_multisample_properties_ext:
        PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
    pub get_physical_device_optical_flow_image_formats_nv:
        PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
    pub get_physical_device_present_rectangles_khr: PFN_vkGetPhysicalDevicePresentRectanglesKHR,
    pub get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    pub get_physical_device_properties2: PFN_vkGetPhysicalDeviceProperties2,
    pub get_physical_device_properties2_khr: PFN_vkGetPhysicalDeviceProperties2KHR,
    pub get_physical_device_queue_family_data_graph_engine_operation_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM,
    pub get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM,
    pub get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
    pub get_physical_device_queue_family_data_graph_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
    pub get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_queue_family_properties2: PFN_vkGetPhysicalDeviceQueueFamilyProperties2,
    pub get_physical_device_queue_family_properties2_khr:
        PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR,
    #[cfg(feature = "screen")]
    pub get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
    pub get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
    pub get_physical_device_sparse_image_format_properties2:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,
    pub get_physical_device_sparse_image_format_properties2_khr:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR,
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
    pub get_physical_device_surface_capabilities2_ext:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
    pub get_physical_device_surface_capabilities2_khr:
        PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    pub get_physical_device_surface_capabilities_khr: PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats2_khr: PFN_vkGetPhysicalDeviceSurfaceFormats2KHR,
    pub get_physical_device_surface_formats_khr: PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
    #[cfg(feature = "win32")]
    pub get_physical_device_surface_present_modes2_ext:
        PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
    pub get_physical_device_surface_present_modes_khr:
        PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
    pub get_physical_device_surface_support_khr: PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
    pub get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
    pub get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolPropertiesEXT,
    #[cfg(feature = "ubm")]
    pub get_physical_device_ubm_presentation_support_sec:
        PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC,
    pub get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    pub get_physical_device_video_encode_quality_level_properties_khr:
        PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
    pub get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
    #[cfg(feature = "wayland")]
    pub get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
    #[cfg(feature = "win32")]
    pub get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
    #[cfg(feature = "xcb")]
    pub get_physical_device_xcb_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
    #[cfg(feature = "xlib")]
    pub get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
    #[cfg(feature = "xlib-xrandr")]
    pub get_rand_r_output_display_ext: PFN_vkGetRandROutputDisplayEXT,
    #[cfg(feature = "win32")]
    pub get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
    pub release_display_ext: PFN_vkReleaseDisplayEXT,
    pub submit_debug_utils_message_ext: PFN_vkSubmitDebugUtilsMessageEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
        instance: Instance,
    ) -> Self {
        let get = get_instance_proc_addr.expect("vkGetInstanceProcAddr is required");
        Self {
            acquire_drm_display_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkAcquireDrmDisplayEXT\0"),
            )),
            #[cfg(feature = "win32")]
            acquire_winrt_display_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkAcquireWinrtDisplayNV\0"),
            )),
            #[cfg(feature = "xlib-xrandr")]
            acquire_xlib_display_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkAcquireXlibDisplayEXT\0"),
            )),
            #[cfg(feature = "android")]
            create_android_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateAndroidSurfaceKHR\0"),
            )),
            create_debug_report_callback_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateDebugReportCallbackEXT\0"),
            )),
            create_debug_utils_messenger_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateDebugUtilsMessengerEXT\0"),
            )),
            create_device: crate::load_pfn(get(instance, crate::c_name_ptr(b"vkCreateDevice\0"))),
            #[cfg(feature = "directfb")]
            create_direct_fb_surface_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateDirectFBSurfaceEXT\0"),
            )),
            create_display_mode_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateDisplayModeKHR\0"),
            )),
            create_display_plane_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateDisplayPlaneSurfaceKHR\0"),
            )),
            create_headless_surface_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateHeadlessSurfaceEXT\0"),
            )),
            #[cfg(feature = "ios")]
            create_ios_surface_mvk: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateIOSSurfaceMVK\0"),
            )),
            #[cfg(feature = "fuchsia")]
            create_image_pipe_surface_fuchsia: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateImagePipeSurfaceFUCHSIA\0"),
            )),
            #[cfg(feature = "macos")]
            create_mac_os_surface_mvk: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateMacOSSurfaceMVK\0"),
            )),
            #[cfg(feature = "metal")]
            create_metal_surface_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateMetalSurfaceEXT\0"),
            )),
            #[cfg(feature = "screen")]
            create_screen_surface_qnx: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateScreenSurfaceQNX\0"),
            )),
            #[cfg(feature = "ggp")]
            create_stream_descriptor_surface_ggp: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateStreamDescriptorSurfaceGGP\0"),
            )),
            #[cfg(feature = "ohos")]
            create_surface_ohos: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateSurfaceOHOS\0"),
            )),
            #[cfg(feature = "ubm")]
            create_ubm_surface_sec: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateUbmSurfaceSEC\0"),
            )),
            #[cfg(feature = "vi")]
            create_vi_surface_nn: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateViSurfaceNN\0"),
            )),
            #[cfg(feature = "wayland")]
            create_wayland_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateWaylandSurfaceKHR\0"),
            )),
            #[cfg(feature = "win32")]
            create_win32_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateWin32SurfaceKHR\0"),
            )),
            #[cfg(feature = "xcb")]
            create_xcb_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateXcbSurfaceKHR\0"),
            )),
            #[cfg(feature = "xlib")]
            create_xlib_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkCreateXlibSurfaceKHR\0"),
            )),
            debug_report_message_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkDebugReportMessageEXT\0"),
            )),
            destroy_debug_report_callback_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkDestroyDebugReportCallbackEXT\0"),
            )),
            destroy_debug_utils_messenger_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkDestroyDebugUtilsMessengerEXT\0"),
            )),
            destroy_instance: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkDestroyInstance\0"),
            )),
            destroy_surface_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkDestroySurfaceKHR\0"),
            )),
            enumerate_device_extension_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumerateDeviceExtensionProperties\0"),
            )),
            enumerate_device_layer_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumerateDeviceLayerProperties\0"),
            )),
            enumerate_physical_device_groups: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumeratePhysicalDeviceGroups\0"),
            )),
            enumerate_physical_device_groups_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumeratePhysicalDeviceGroupsKHR\0"),
            )),
            enumerate_physical_device_queue_family_performance_counters_by_region_arm:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM\0",
                    ),
                )),
            enumerate_physical_device_queue_family_performance_query_counters_khr: crate::load_pfn(
                get(
                    instance,
                    crate::c_name_ptr(
                        b"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0",
                    ),
                ),
            ),
            enumerate_physical_device_shader_instrumentation_metrics_arm: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM\0"),
            )),
            enumerate_physical_devices: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkEnumeratePhysicalDevices\0"),
            )),
            get_device_proc_addr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDeviceProcAddr\0"),
            )),
            get_display_mode_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDisplayModeProperties2KHR\0"),
            )),
            get_display_mode_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDisplayModePropertiesKHR\0"),
            )),
            get_display_plane_capabilities2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDisplayPlaneCapabilities2KHR\0"),
            )),
            get_display_plane_capabilities_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDisplayPlaneCapabilitiesKHR\0"),
            )),
            get_display_plane_supported_displays_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDisplayPlaneSupportedDisplaysKHR\0"),
            )),
            get_drm_display_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetDrmDisplayEXT\0"),
            )),
            get_physical_device_calibrateable_time_domains_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0"),
            )),
            get_physical_device_calibrateable_time_domains_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR\0"),
            )),
            get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV\0",
                    ),
                )),
            get_physical_device_cooperative_matrix_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR\0"),
            )),
            get_physical_device_cooperative_matrix_properties_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0"),
            )),
            get_physical_device_cooperative_vector_properties_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceCooperativeVectorPropertiesNV\0"),
            )),
            get_physical_device_descriptor_size_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDescriptorSizeEXT\0"),
            )),
            #[cfg(feature = "directfb")]
            get_physical_device_direct_fb_presentation_support_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDirectFBPresentationSupportEXT\0"),
            )),
            get_physical_device_display_plane_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0"),
            )),
            get_physical_device_display_plane_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0"),
            )),
            get_physical_device_display_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDisplayProperties2KHR\0"),
            )),
            get_physical_device_display_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceDisplayPropertiesKHR\0"),
            )),
            get_physical_device_external_buffer_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalBufferProperties\0"),
            )),
            get_physical_device_external_buffer_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0"),
            )),
            get_physical_device_external_fence_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalFenceProperties\0"),
            )),
            get_physical_device_external_fence_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalFencePropertiesKHR\0"),
            )),
            get_physical_device_external_image_format_properties_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0"),
            )),
            get_physical_device_external_semaphore_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalSemaphoreProperties\0"),
            )),
            get_physical_device_external_semaphore_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0"),
            )),
            get_physical_device_external_tensor_properties_arm: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceExternalTensorPropertiesARM\0"),
            )),
            get_physical_device_features: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFeatures\0"),
            )),
            get_physical_device_features2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFeatures2\0"),
            )),
            get_physical_device_features2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFeatures2KHR\0"),
            )),
            get_physical_device_format_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFormatProperties\0"),
            )),
            get_physical_device_format_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFormatProperties2\0"),
            )),
            get_physical_device_format_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFormatProperties2KHR\0"),
            )),
            get_physical_device_fragment_shading_rates_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceFragmentShadingRatesKHR\0"),
            )),
            get_physical_device_image_format_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceImageFormatProperties\0"),
            )),
            get_physical_device_image_format_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceImageFormatProperties2\0"),
            )),
            get_physical_device_image_format_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceImageFormatProperties2KHR\0"),
            )),
            get_physical_device_memory_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceMemoryProperties\0"),
            )),
            get_physical_device_memory_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceMemoryProperties2\0"),
            )),
            get_physical_device_memory_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceMemoryProperties2KHR\0"),
            )),
            get_physical_device_multisample_properties_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceMultisamplePropertiesEXT\0"),
            )),
            get_physical_device_optical_flow_image_formats_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceOpticalFlowImageFormatsNV\0"),
            )),
            get_physical_device_present_rectangles_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDevicePresentRectanglesKHR\0"),
            )),
            get_physical_device_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceProperties\0"),
            )),
            get_physical_device_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceProperties2\0"),
            )),
            get_physical_device_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceProperties2KHR\0"),
            )),
            get_physical_device_queue_family_data_graph_engine_operation_properties_arm:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM\0",
                    ),
                )),
            get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM\0",
                    ),
                )),
            get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM\0",
                    ),
                )),
            get_physical_device_queue_family_data_graph_properties_arm: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM\0"),
            )),
            get_physical_device_queue_family_performance_query_passes_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0"),
            )),
            get_physical_device_queue_family_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceQueueFamilyProperties\0"),
            )),
            get_physical_device_queue_family_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceQueueFamilyProperties2\0"),
            )),
            get_physical_device_queue_family_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0"),
            )),
            #[cfg(feature = "screen")]
            get_physical_device_screen_presentation_support_qnx: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceScreenPresentationSupportQNX\0"),
            )),
            get_physical_device_sparse_image_format_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSparseImageFormatProperties\0"),
            )),
            get_physical_device_sparse_image_format_properties2: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSparseImageFormatProperties2\0"),
            )),
            get_physical_device_sparse_image_format_properties2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0"),
            )),
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
                crate::load_pfn(get(
                    instance,
                    crate::c_name_ptr(
                        b"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0",
                    ),
                )),
            get_physical_device_surface_capabilities2_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0"),
            )),
            get_physical_device_surface_capabilities2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0"),
            )),
            get_physical_device_surface_capabilities_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0"),
            )),
            get_physical_device_surface_formats2_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceFormats2KHR\0"),
            )),
            get_physical_device_surface_formats_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceFormatsKHR\0"),
            )),
            #[cfg(feature = "win32")]
            get_physical_device_surface_present_modes2_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfacePresentModes2EXT\0"),
            )),
            get_physical_device_surface_present_modes_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfacePresentModesKHR\0"),
            )),
            get_physical_device_surface_support_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceSurfaceSupportKHR\0"),
            )),
            get_physical_device_tool_properties: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceToolProperties\0"),
            )),
            get_physical_device_tool_properties_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceToolPropertiesEXT\0"),
            )),
            #[cfg(feature = "ubm")]
            get_physical_device_ubm_presentation_support_sec: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceUbmPresentationSupportSEC\0"),
            )),
            get_physical_device_video_capabilities_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceVideoCapabilitiesKHR\0"),
            )),
            get_physical_device_video_encode_quality_level_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR\0"),
            )),
            get_physical_device_video_format_properties_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceVideoFormatPropertiesKHR\0"),
            )),
            #[cfg(feature = "wayland")]
            get_physical_device_wayland_presentation_support_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0"),
            )),
            #[cfg(feature = "win32")]
            get_physical_device_win32_presentation_support_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceWin32PresentationSupportKHR\0"),
            )),
            #[cfg(feature = "xcb")]
            get_physical_device_xcb_presentation_support_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceXcbPresentationSupportKHR\0"),
            )),
            #[cfg(feature = "xlib")]
            get_physical_device_xlib_presentation_support_khr: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetPhysicalDeviceXlibPresentationSupportKHR\0"),
            )),
            #[cfg(feature = "xlib-xrandr")]
            get_rand_r_output_display_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetRandROutputDisplayEXT\0"),
            )),
            #[cfg(feature = "win32")]
            get_winrt_display_nv: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkGetWinrtDisplayNV\0"),
            )),
            release_display_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkReleaseDisplayEXT\0"),
            )),
            submit_debug_utils_message_ext: crate::load_pfn(get(
                instance,
                crate::c_name_ptr(b"vkSubmitDebugUtilsMessageEXT\0"),
            )),
        }
    }
}

#[derive(Copy, Clone)]
pub struct DeviceFn {
    #[cfg(feature = "win32")]
    pub acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    pub acquire_next_image2_khr: PFN_vkAcquireNextImage2KHR,
    pub acquire_next_image_khr: PFN_vkAcquireNextImageKHR,
    pub acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    pub acquire_profiling_lock_khr: PFN_vkAcquireProfilingLockKHR,
    pub allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    pub allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    pub allocate_memory: PFN_vkAllocateMemory,
    pub anti_lag_update_amd: PFN_vkAntiLagUpdateAMD,
    pub begin_command_buffer: PFN_vkBeginCommandBuffer,
    pub bind_acceleration_structure_memory_nv: PFN_vkBindAccelerationStructureMemoryNV,
    pub bind_buffer_memory: PFN_vkBindBufferMemory,
    pub bind_buffer_memory2: PFN_vkBindBufferMemory2,
    pub bind_buffer_memory2_khr: PFN_vkBindBufferMemory2KHR,
    pub bind_data_graph_pipeline_session_memory_arm: PFN_vkBindDataGraphPipelineSessionMemoryARM,
    pub bind_image_memory: PFN_vkBindImageMemory,
    pub bind_image_memory2: PFN_vkBindImageMemory2,
    pub bind_image_memory2_khr: PFN_vkBindImageMemory2KHR,
    pub bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
    pub bind_tensor_memory_arm: PFN_vkBindTensorMemoryARM,
    pub bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    pub build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
    pub build_micromaps_ext: PFN_vkBuildMicromapsEXT,
    pub clear_shader_instrumentation_metrics_arm: PFN_vkClearShaderInstrumentationMetricsARM,
    pub cmd_begin_conditional_rendering2_ext: PFN_vkCmdBeginConditionalRendering2EXT,
    pub cmd_begin_conditional_rendering_ext: PFN_vkCmdBeginConditionalRenderingEXT,
    pub cmd_begin_custom_resolve_ext: PFN_vkCmdBeginCustomResolveEXT,
    pub cmd_begin_debug_utils_label_ext: PFN_vkCmdBeginDebugUtilsLabelEXT,
    pub cmd_begin_per_tile_execution_qcom: PFN_vkCmdBeginPerTileExecutionQCOM,
    pub cmd_begin_query: PFN_vkCmdBeginQuery,
    pub cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
    pub cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    pub cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    pub cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2KHR,
    pub cmd_begin_rendering: PFN_vkCmdBeginRendering,
    pub cmd_begin_rendering_khr: PFN_vkCmdBeginRenderingKHR,
    pub cmd_begin_shader_instrumentation_arm: PFN_vkCmdBeginShaderInstrumentationARM,
    pub cmd_begin_transform_feedback2_ext: PFN_vkCmdBeginTransformFeedback2EXT,
    pub cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
    pub cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    pub cmd_bind_descriptor_buffer_embedded_samplers2_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
    pub cmd_bind_descriptor_buffer_embedded_samplers_ext:
        PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
    pub cmd_bind_descriptor_buffers_ext: PFN_vkCmdBindDescriptorBuffersEXT,
    pub cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    pub cmd_bind_descriptor_sets2: PFN_vkCmdBindDescriptorSets2,
    pub cmd_bind_descriptor_sets2_khr: PFN_vkCmdBindDescriptorSets2KHR,
    pub cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    pub cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    pub cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2KHR,
    pub cmd_bind_index_buffer3_khr: PFN_vkCmdBindIndexBuffer3KHR,
    pub cmd_bind_invocation_mask_huawei: PFN_vkCmdBindInvocationMaskHUAWEI,
    pub cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    pub cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    pub cmd_bind_resource_heap_ext: PFN_vkCmdBindResourceHeapEXT,
    pub cmd_bind_sampler_heap_ext: PFN_vkCmdBindSamplerHeapEXT,
    pub cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
    pub cmd_bind_shading_rate_image_nv: PFN_vkCmdBindShadingRateImageNV,
    pub cmd_bind_tile_memory_qcom: PFN_vkCmdBindTileMemoryQCOM,
    pub cmd_bind_transform_feedback_buffers2_ext: PFN_vkCmdBindTransformFeedbackBuffers2EXT,
    pub cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
    pub cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    pub cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    pub cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2EXT,
    pub cmd_bind_vertex_buffers3_khr: PFN_vkCmdBindVertexBuffers3KHR,
    pub cmd_blit_image: PFN_vkCmdBlitImage,
    pub cmd_blit_image2: PFN_vkCmdBlitImage2,
    pub cmd_blit_image2_khr: PFN_vkCmdBlitImage2KHR,
    pub cmd_build_acceleration_structure_nv: PFN_vkCmdBuildAccelerationStructureNV,
    pub cmd_build_acceleration_structures_indirect_khr:
        PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
    pub cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
    pub cmd_build_cluster_acceleration_structure_indirect_nv:
        PFN_vkCmdBuildClusterAccelerationStructureIndirectNV,
    pub cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
    pub cmd_build_partitioned_acceleration_structures_nv:
        PFN_vkCmdBuildPartitionedAccelerationStructuresNV,
    pub cmd_clear_attachments: PFN_vkCmdClearAttachments,
    pub cmd_clear_color_image: PFN_vkCmdClearColorImage,
    pub cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    pub cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
    pub cmd_convert_cooperative_vector_matrix_nv: PFN_vkCmdConvertCooperativeVectorMatrixNV,
    pub cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    pub cmd_copy_acceleration_structure_nv: PFN_vkCmdCopyAccelerationStructureNV,
    pub cmd_copy_acceleration_structure_to_memory_khr:
        PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    pub cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    pub cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    pub cmd_copy_buffer2_khr: PFN_vkCmdCopyBuffer2KHR,
    pub cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    pub cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    pub cmd_copy_buffer_to_image2_khr: PFN_vkCmdCopyBufferToImage2KHR,
    pub cmd_copy_image: PFN_vkCmdCopyImage,
    pub cmd_copy_image2: PFN_vkCmdCopyImage2,
    pub cmd_copy_image2_khr: PFN_vkCmdCopyImage2KHR,
    pub cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    pub cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    pub cmd_copy_image_to_buffer2_khr: PFN_vkCmdCopyImageToBuffer2KHR,
    pub cmd_copy_image_to_memory_khr: PFN_vkCmdCopyImageToMemoryKHR,
    pub cmd_copy_memory_indirect_khr: PFN_vkCmdCopyMemoryIndirectKHR,
    pub cmd_copy_memory_indirect_nv: PFN_vkCmdCopyMemoryIndirectNV,
    pub cmd_copy_memory_khr: PFN_vkCmdCopyMemoryKHR,
    pub cmd_copy_memory_to_acceleration_structure_khr:
        PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    pub cmd_copy_memory_to_image_indirect_khr: PFN_vkCmdCopyMemoryToImageIndirectKHR,
    pub cmd_copy_memory_to_image_indirect_nv: PFN_vkCmdCopyMemoryToImageIndirectNV,
    pub cmd_copy_memory_to_image_khr: PFN_vkCmdCopyMemoryToImageKHR,
    pub cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
    pub cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
    pub cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
    pub cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    pub cmd_copy_query_pool_results_to_memory_khr: PFN_vkCmdCopyQueryPoolResultsToMemoryKHR,
    pub cmd_copy_tensor_arm: PFN_vkCmdCopyTensorARM,
    pub cmd_cu_launch_kernel_nvx: PFN_vkCmdCuLaunchKernelNVX,
    #[cfg(feature = "beta")]
    pub cmd_cuda_launch_kernel_nv: PFN_vkCmdCudaLaunchKernelNV,
    pub cmd_debug_marker_begin_ext: PFN_vkCmdDebugMarkerBeginEXT,
    pub cmd_debug_marker_end_ext: PFN_vkCmdDebugMarkerEndEXT,
    pub cmd_debug_marker_insert_ext: PFN_vkCmdDebugMarkerInsertEXT,
    pub cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
    pub cmd_decompress_memory_ext: PFN_vkCmdDecompressMemoryEXT,
    pub cmd_decompress_memory_indirect_count_ext: PFN_vkCmdDecompressMemoryIndirectCountEXT,
    pub cmd_decompress_memory_indirect_count_nv: PFN_vkCmdDecompressMemoryIndirectCountNV,
    pub cmd_decompress_memory_nv: PFN_vkCmdDecompressMemoryNV,
    pub cmd_dispatch: PFN_vkCmdDispatch,
    pub cmd_dispatch_base: PFN_vkCmdDispatchBase,
    pub cmd_dispatch_base_khr: PFN_vkCmdDispatchBaseKHR,
    pub cmd_dispatch_data_graph_arm: PFN_vkCmdDispatchDataGraphARM,
    #[cfg(feature = "beta")]
    pub cmd_dispatch_graph_amdx: PFN_vkCmdDispatchGraphAMDX,
    #[cfg(feature = "beta")]
    pub cmd_dispatch_graph_indirect_amdx: PFN_vkCmdDispatchGraphIndirectAMDX,
    #[cfg(feature = "beta")]
    pub cmd_dispatch_graph_indirect_count_amdx: PFN_vkCmdDispatchGraphIndirectCountAMDX,
    pub cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    pub cmd_dispatch_indirect2_khr: PFN_vkCmdDispatchIndirect2KHR,
    pub cmd_dispatch_tile_qcom: PFN_vkCmdDispatchTileQCOM,
    pub cmd_draw: PFN_vkCmdDraw,
    pub cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
    pub cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
    pub cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    pub cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    pub cmd_draw_indexed_indirect2_khr: PFN_vkCmdDrawIndexedIndirect2KHR,
    pub cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
    pub cmd_draw_indexed_indirect_count2_khr: PFN_vkCmdDrawIndexedIndirectCount2KHR,
    pub cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCountAMD,
    pub cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCountKHR,
    pub cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    pub cmd_draw_indirect2_khr: PFN_vkCmdDrawIndirect2KHR,
    pub cmd_draw_indirect_byte_count2_ext: PFN_vkCmdDrawIndirectByteCount2EXT,
    pub cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
    pub cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    pub cmd_draw_indirect_count2_khr: PFN_vkCmdDrawIndirectCount2KHR,
    pub cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCountAMD,
    pub cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCountKHR,
    pub cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
    pub cmd_draw_mesh_tasks_indirect2_ext: PFN_vkCmdDrawMeshTasksIndirect2EXT,
    pub cmd_draw_mesh_tasks_indirect_count2_ext: PFN_vkCmdDrawMeshTasksIndirectCount2EXT,
    pub cmd_draw_mesh_tasks_indirect_count_ext: PFN_vkCmdDrawMeshTasksIndirectCountEXT,
    pub cmd_draw_mesh_tasks_indirect_count_nv: PFN_vkCmdDrawMeshTasksIndirectCountNV,
    pub cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
    pub cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    pub cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    pub cmd_draw_multi_ext: PFN_vkCmdDrawMultiEXT,
    pub cmd_draw_multi_indexed_ext: PFN_vkCmdDrawMultiIndexedEXT,
    pub cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
    pub cmd_end_conditional_rendering_ext: PFN_vkCmdEndConditionalRenderingEXT,
    pub cmd_end_debug_utils_label_ext: PFN_vkCmdEndDebugUtilsLabelEXT,
    pub cmd_end_per_tile_execution_qcom: PFN_vkCmdEndPerTileExecutionQCOM,
    pub cmd_end_query: PFN_vkCmdEndQuery,
    pub cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
    pub cmd_end_render_pass: PFN_vkCmdEndRenderPass,
    pub cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    pub cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2KHR,
    pub cmd_end_rendering: PFN_vkCmdEndRendering,
    pub cmd_end_rendering2_ext: PFN_vkCmdEndRendering2EXT,
    pub cmd_end_rendering2_khr: PFN_vkCmdEndRendering2KHR,
    pub cmd_end_rendering_khr: PFN_vkCmdEndRenderingKHR,
    pub cmd_end_shader_instrumentation_arm: PFN_vkCmdEndShaderInstrumentationARM,
    pub cmd_end_transform_feedback2_ext: PFN_vkCmdEndTransformFeedback2EXT,
    pub cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
    pub cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
    pub cmd_execute_commands: PFN_vkCmdExecuteCommands,
    pub cmd_execute_generated_commands_ext: PFN_vkCmdExecuteGeneratedCommandsEXT,
    pub cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    pub cmd_fill_buffer: PFN_vkCmdFillBuffer,
    pub cmd_fill_memory_khr: PFN_vkCmdFillMemoryKHR,
    #[cfg(feature = "beta")]
    pub cmd_initialize_graph_scratch_memory_amdx: PFN_vkCmdInitializeGraphScratchMemoryAMDX,
    pub cmd_insert_debug_utils_label_ext: PFN_vkCmdInsertDebugUtilsLabelEXT,
    pub cmd_next_subpass: PFN_vkCmdNextSubpass,
    pub cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    pub cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2KHR,
    pub cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
    pub cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    pub cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    pub cmd_pipeline_barrier2_khr: PFN_vkCmdPipelineBarrier2KHR,
    pub cmd_preprocess_generated_commands_ext: PFN_vkCmdPreprocessGeneratedCommandsEXT,
    pub cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    pub cmd_push_constants: PFN_vkCmdPushConstants,
    pub cmd_push_constants2: PFN_vkCmdPushConstants2,
    pub cmd_push_constants2_khr: PFN_vkCmdPushConstants2KHR,
    pub cmd_push_data_ext: PFN_vkCmdPushDataEXT,
    pub cmd_push_descriptor_set: PFN_vkCmdPushDescriptorSet,
    pub cmd_push_descriptor_set2: PFN_vkCmdPushDescriptorSet2,
    pub cmd_push_descriptor_set2_khr: PFN_vkCmdPushDescriptorSet2KHR,
    pub cmd_push_descriptor_set_khr: PFN_vkCmdPushDescriptorSetKHR,
    pub cmd_push_descriptor_set_with_template: PFN_vkCmdPushDescriptorSetWithTemplate,
    pub cmd_push_descriptor_set_with_template2: PFN_vkCmdPushDescriptorSetWithTemplate2,
    pub cmd_push_descriptor_set_with_template2_khr: PFN_vkCmdPushDescriptorSetWithTemplate2KHR,
    pub cmd_push_descriptor_set_with_template_khr: PFN_vkCmdPushDescriptorSetWithTemplateKHR,
    pub cmd_reset_event: PFN_vkCmdResetEvent,
    pub cmd_reset_event2: PFN_vkCmdResetEvent2,
    pub cmd_reset_event2_khr: PFN_vkCmdResetEvent2KHR,
    pub cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    pub cmd_resolve_image: PFN_vkCmdResolveImage,
    pub cmd_resolve_image2: PFN_vkCmdResolveImage2,
    pub cmd_resolve_image2_khr: PFN_vkCmdResolveImage2KHR,
    pub cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
    pub cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
    pub cmd_set_attachment_feedback_loop_enable_ext: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
    pub cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    pub cmd_set_checkpoint_nv: PFN_vkCmdSetCheckpointNV,
    pub cmd_set_coarse_sample_order_nv: PFN_vkCmdSetCoarseSampleOrderNV,
    pub cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
    pub cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
    pub cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
    pub cmd_set_color_write_enable_ext: PFN_vkCmdSetColorWriteEnableEXT,
    pub cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
    pub cmd_set_compute_occupancy_priority_nv: PFN_vkCmdSetComputeOccupancyPriorityNV,
    pub cmd_set_conservative_rasterization_mode_ext: PFN_vkCmdSetConservativeRasterizationModeEXT,
    pub cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
    pub cmd_set_coverage_modulation_table_enable_nv: PFN_vkCmdSetCoverageModulationTableEnableNV,
    pub cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
    pub cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
    pub cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
    pub cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
    pub cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    pub cmd_set_cull_mode_ext: PFN_vkCmdSetCullModeEXT,
    pub cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    pub cmd_set_depth_bias2_ext: PFN_vkCmdSetDepthBias2EXT,
    pub cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    pub cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnableEXT,
    pub cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    pub cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    pub cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnableEXT,
    pub cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
    pub cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
    pub cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
    pub cmd_set_depth_clip_negative_one_to_one_ext: PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
    pub cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    pub cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOpEXT,
    pub cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    pub cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnableEXT,
    pub cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    pub cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnableEXT,
    pub cmd_set_descriptor_buffer_offsets2_ext: PFN_vkCmdSetDescriptorBufferOffsets2EXT,
    pub cmd_set_descriptor_buffer_offsets_ext: PFN_vkCmdSetDescriptorBufferOffsetsEXT,
    pub cmd_set_device_mask: PFN_vkCmdSetDeviceMask,
    pub cmd_set_device_mask_khr: PFN_vkCmdSetDeviceMaskKHR,
    pub cmd_set_discard_rectangle_ext: PFN_vkCmdSetDiscardRectangleEXT,
    pub cmd_set_discard_rectangle_enable_ext: PFN_vkCmdSetDiscardRectangleEnableEXT,
    pub cmd_set_discard_rectangle_mode_ext: PFN_vkCmdSetDiscardRectangleModeEXT,
    pub cmd_set_dispatch_parameters_arm: PFN_vkCmdSetDispatchParametersARM,
    pub cmd_set_event: PFN_vkCmdSetEvent,
    pub cmd_set_event2: PFN_vkCmdSetEvent2,
    pub cmd_set_event2_khr: PFN_vkCmdSetEvent2KHR,
    pub cmd_set_exclusive_scissor_enable_nv: PFN_vkCmdSetExclusiveScissorEnableNV,
    pub cmd_set_exclusive_scissor_nv: PFN_vkCmdSetExclusiveScissorNV,
    pub cmd_set_extra_primitive_overestimation_size_ext:
        PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
    pub cmd_set_fragment_shading_rate_enum_nv: PFN_vkCmdSetFragmentShadingRateEnumNV,
    pub cmd_set_fragment_shading_rate_khr: PFN_vkCmdSetFragmentShadingRateKHR,
    pub cmd_set_front_face: PFN_vkCmdSetFrontFace,
    pub cmd_set_front_face_ext: PFN_vkCmdSetFrontFaceEXT,
    pub cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
    pub cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
    pub cmd_set_line_stipple_ext: PFN_vkCmdSetLineStippleEXT,
    pub cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
    pub cmd_set_line_stipple_khr: PFN_vkCmdSetLineStippleKHR,
    pub cmd_set_line_width: PFN_vkCmdSetLineWidth,
    pub cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    pub cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
    pub cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    pub cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    pub cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    pub cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    pub cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
    pub cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
    pub cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnableEXT,
    pub cmd_set_primitive_restart_index_ext: PFN_vkCmdSetPrimitiveRestartIndexEXT,
    pub cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    pub cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopologyEXT,
    pub cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
    pub cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
    pub cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
    pub cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    pub cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnableEXT,
    pub cmd_set_ray_tracing_pipeline_stack_size_khr: PFN_vkCmdSetRayTracingPipelineStackSizeKHR,
    pub cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    pub cmd_set_rendering_attachment_locations_khr: PFN_vkCmdSetRenderingAttachmentLocationsKHR,
    pub cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
    pub cmd_set_rendering_input_attachment_indices_khr:
        PFN_vkCmdSetRenderingInputAttachmentIndicesKHR,
    pub cmd_set_representative_fragment_test_enable_nv:
        PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
    pub cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
    pub cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
    pub cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
    pub cmd_set_scissor: PFN_vkCmdSetScissor,
    pub cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    pub cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCountEXT,
    pub cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
    pub cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    pub cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    pub cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOpEXT,
    pub cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    pub cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    pub cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnableEXT,
    pub cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    pub cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
    pub cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
    pub cmd_set_viewport: PFN_vkCmdSetViewport,
    pub cmd_set_viewport_shading_rate_palette_nv: PFN_vkCmdSetViewportShadingRatePaletteNV,
    pub cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
    pub cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
    pub cmd_set_viewport_w_scaling_nv: PFN_vkCmdSetViewportWScalingNV,
    pub cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    pub cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCountEXT,
    pub cmd_subpass_shading_huawei: PFN_vkCmdSubpassShadingHUAWEI,
    pub cmd_trace_rays_indirect2_khr: PFN_vkCmdTraceRaysIndirect2KHR,
    pub cmd_trace_rays_indirect_khr: PFN_vkCmdTraceRaysIndirectKHR,
    pub cmd_trace_rays_khr: PFN_vkCmdTraceRaysKHR,
    pub cmd_trace_rays_nv: PFN_vkCmdTraceRaysNV,
    pub cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    pub cmd_update_memory_khr: PFN_vkCmdUpdateMemoryKHR,
    pub cmd_update_pipeline_indirect_buffer_nv: PFN_vkCmdUpdatePipelineIndirectBufferNV,
    pub cmd_wait_events: PFN_vkCmdWaitEvents,
    pub cmd_wait_events2: PFN_vkCmdWaitEvents2,
    pub cmd_wait_events2_khr: PFN_vkCmdWaitEvents2KHR,
    pub cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    pub cmd_write_acceleration_structures_properties_nv:
        PFN_vkCmdWriteAccelerationStructuresPropertiesNV,
    pub cmd_write_buffer_marker2_amd: PFN_vkCmdWriteBufferMarker2AMD,
    pub cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
    pub cmd_write_marker_to_memory_amd: PFN_vkCmdWriteMarkerToMemoryAMD,
    pub cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
    pub cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    pub cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    pub cmd_write_timestamp2_khr: PFN_vkCmdWriteTimestamp2KHR,
    pub compile_deferred_nv: PFN_vkCompileDeferredNV,
    pub convert_cooperative_vector_matrix_nv: PFN_vkConvertCooperativeVectorMatrixNV,
    pub copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    pub copy_image_to_image: PFN_vkCopyImageToImage,
    pub copy_image_to_image_ext: PFN_vkCopyImageToImageEXT,
    pub copy_image_to_memory: PFN_vkCopyImageToMemory,
    pub copy_image_to_memory_ext: PFN_vkCopyImageToMemoryEXT,
    pub copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    pub copy_memory_to_image: PFN_vkCopyMemoryToImage,
    pub copy_memory_to_image_ext: PFN_vkCopyMemoryToImageEXT,
    pub copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
    pub copy_micromap_ext: PFN_vkCopyMicromapEXT,
    pub copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
    pub create_acceleration_structure2_khr: PFN_vkCreateAccelerationStructure2KHR,
    pub create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    pub create_acceleration_structure_nv: PFN_vkCreateAccelerationStructureNV,
    pub create_buffer: PFN_vkCreateBuffer,
    #[cfg(feature = "fuchsia")]
    pub create_buffer_collection_fuchsia: PFN_vkCreateBufferCollectionFUCHSIA,
    pub create_buffer_view: PFN_vkCreateBufferView,
    pub create_command_pool: PFN_vkCreateCommandPool,
    pub create_compute_pipelines: PFN_vkCreateComputePipelines,
    pub create_cu_function_nvx: PFN_vkCreateCuFunctionNVX,
    pub create_cu_module_nvx: PFN_vkCreateCuModuleNVX,
    #[cfg(feature = "beta")]
    pub create_cuda_function_nv: PFN_vkCreateCudaFunctionNV,
    #[cfg(feature = "beta")]
    pub create_cuda_module_nv: PFN_vkCreateCudaModuleNV,
    pub create_data_graph_pipeline_session_arm: PFN_vkCreateDataGraphPipelineSessionARM,
    pub create_data_graph_pipelines_arm: PFN_vkCreateDataGraphPipelinesARM,
    pub create_deferred_operation_khr: PFN_vkCreateDeferredOperationKHR,
    pub create_descriptor_pool: PFN_vkCreateDescriptorPool,
    pub create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    pub create_descriptor_update_template: PFN_vkCreateDescriptorUpdateTemplate,
    pub create_descriptor_update_template_khr: PFN_vkCreateDescriptorUpdateTemplateKHR,
    pub create_event: PFN_vkCreateEvent,
    #[cfg(feature = "beta")]
    pub create_execution_graph_pipelines_amdx: PFN_vkCreateExecutionGraphPipelinesAMDX,
    pub create_external_compute_queue_nv: PFN_vkCreateExternalComputeQueueNV,
    pub create_fence: PFN_vkCreateFence,
    pub create_framebuffer: PFN_vkCreateFramebuffer,
    pub create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    pub create_image: PFN_vkCreateImage,
    pub create_image_view: PFN_vkCreateImageView,
    pub create_indirect_commands_layout_ext: PFN_vkCreateIndirectCommandsLayoutEXT,
    pub create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    pub create_indirect_execution_set_ext: PFN_vkCreateIndirectExecutionSetEXT,
    pub create_micromap_ext: PFN_vkCreateMicromapEXT,
    pub create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
    pub create_pipeline_binaries_khr: PFN_vkCreatePipelineBinariesKHR,
    pub create_pipeline_cache: PFN_vkCreatePipelineCache,
    pub create_pipeline_layout: PFN_vkCreatePipelineLayout,
    pub create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    pub create_private_data_slot_ext: PFN_vkCreatePrivateDataSlotEXT,
    pub create_query_pool: PFN_vkCreateQueryPool,
    pub create_ray_tracing_pipelines_khr: PFN_vkCreateRayTracingPipelinesKHR,
    pub create_ray_tracing_pipelines_nv: PFN_vkCreateRayTracingPipelinesNV,
    pub create_render_pass: PFN_vkCreateRenderPass,
    pub create_render_pass2: PFN_vkCreateRenderPass2,
    pub create_render_pass2_khr: PFN_vkCreateRenderPass2KHR,
    pub create_sampler: PFN_vkCreateSampler,
    pub create_sampler_ycbcr_conversion: PFN_vkCreateSamplerYcbcrConversion,
    pub create_sampler_ycbcr_conversion_khr: PFN_vkCreateSamplerYcbcrConversionKHR,
    pub create_semaphore: PFN_vkCreateSemaphore,
    pub create_shader_instrumentation_arm: PFN_vkCreateShaderInstrumentationARM,
    pub create_shader_module: PFN_vkCreateShaderModule,
    pub create_shaders_ext: PFN_vkCreateShadersEXT,
    pub create_shared_swapchains_khr: PFN_vkCreateSharedSwapchainsKHR,
    pub create_swapchain_khr: PFN_vkCreateSwapchainKHR,
    pub create_tensor_arm: PFN_vkCreateTensorARM,
    pub create_tensor_view_arm: PFN_vkCreateTensorViewARM,
    pub create_validation_cache_ext: PFN_vkCreateValidationCacheEXT,
    pub create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    pub create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    pub debug_marker_set_object_name_ext: PFN_vkDebugMarkerSetObjectNameEXT,
    pub debug_marker_set_object_tag_ext: PFN_vkDebugMarkerSetObjectTagEXT,
    pub deferred_operation_join_khr: PFN_vkDeferredOperationJoinKHR,
    pub destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    pub destroy_acceleration_structure_nv: PFN_vkDestroyAccelerationStructureNV,
    pub destroy_buffer: PFN_vkDestroyBuffer,
    #[cfg(feature = "fuchsia")]
    pub destroy_buffer_collection_fuchsia: PFN_vkDestroyBufferCollectionFUCHSIA,
    pub destroy_buffer_view: PFN_vkDestroyBufferView,
    pub destroy_command_pool: PFN_vkDestroyCommandPool,
    pub destroy_cu_function_nvx: PFN_vkDestroyCuFunctionNVX,
    pub destroy_cu_module_nvx: PFN_vkDestroyCuModuleNVX,
    #[cfg(feature = "beta")]
    pub destroy_cuda_function_nv: PFN_vkDestroyCudaFunctionNV,
    #[cfg(feature = "beta")]
    pub destroy_cuda_module_nv: PFN_vkDestroyCudaModuleNV,
    pub destroy_data_graph_pipeline_session_arm: PFN_vkDestroyDataGraphPipelineSessionARM,
    pub destroy_deferred_operation_khr: PFN_vkDestroyDeferredOperationKHR,
    pub destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    pub destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    pub destroy_descriptor_update_template: PFN_vkDestroyDescriptorUpdateTemplate,
    pub destroy_descriptor_update_template_khr: PFN_vkDestroyDescriptorUpdateTemplateKHR,
    pub destroy_device: PFN_vkDestroyDevice,
    pub destroy_event: PFN_vkDestroyEvent,
    pub destroy_external_compute_queue_nv: PFN_vkDestroyExternalComputeQueueNV,
    pub destroy_fence: PFN_vkDestroyFence,
    pub destroy_framebuffer: PFN_vkDestroyFramebuffer,
    pub destroy_image: PFN_vkDestroyImage,
    pub destroy_image_view: PFN_vkDestroyImageView,
    pub destroy_indirect_commands_layout_ext: PFN_vkDestroyIndirectCommandsLayoutEXT,
    pub destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
    pub destroy_indirect_execution_set_ext: PFN_vkDestroyIndirectExecutionSetEXT,
    pub destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
    pub destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
    pub destroy_pipeline: PFN_vkDestroyPipeline,
    pub destroy_pipeline_binary_khr: PFN_vkDestroyPipelineBinaryKHR,
    pub destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    pub destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    pub destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    pub destroy_private_data_slot_ext: PFN_vkDestroyPrivateDataSlotEXT,
    pub destroy_query_pool: PFN_vkDestroyQueryPool,
    pub destroy_render_pass: PFN_vkDestroyRenderPass,
    pub destroy_sampler: PFN_vkDestroySampler,
    pub destroy_sampler_ycbcr_conversion: PFN_vkDestroySamplerYcbcrConversion,
    pub destroy_sampler_ycbcr_conversion_khr: PFN_vkDestroySamplerYcbcrConversionKHR,
    pub destroy_semaphore: PFN_vkDestroySemaphore,
    pub destroy_shader_ext: PFN_vkDestroyShaderEXT,
    pub destroy_shader_instrumentation_arm: PFN_vkDestroyShaderInstrumentationARM,
    pub destroy_shader_module: PFN_vkDestroyShaderModule,
    pub destroy_swapchain_khr: PFN_vkDestroySwapchainKHR,
    pub destroy_tensor_arm: PFN_vkDestroyTensorARM,
    pub destroy_tensor_view_arm: PFN_vkDestroyTensorViewARM,
    pub destroy_validation_cache_ext: PFN_vkDestroyValidationCacheEXT,
    pub destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    pub destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    pub device_wait_idle: PFN_vkDeviceWaitIdle,
    pub display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    pub end_command_buffer: PFN_vkEndCommandBuffer,
    #[cfg(feature = "metal")]
    pub export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
    pub flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    pub free_command_buffers: PFN_vkFreeCommandBuffers,
    pub free_descriptor_sets: PFN_vkFreeDescriptorSets,
    pub free_memory: PFN_vkFreeMemory,
    pub get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
    pub get_acceleration_structure_device_address_khr:
        PFN_vkGetAccelerationStructureDeviceAddressKHR,
    pub get_acceleration_structure_handle_nv: PFN_vkGetAccelerationStructureHandleNV,
    pub get_acceleration_structure_memory_requirements_nv:
        PFN_vkGetAccelerationStructureMemoryRequirementsNV,
    pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
        PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
    #[cfg(feature = "android")]
    pub get_android_hardware_buffer_properties_android:
        PFN_vkGetAndroidHardwareBufferPropertiesANDROID,
    #[cfg(feature = "fuchsia")]
    pub get_buffer_collection_properties_fuchsia: PFN_vkGetBufferCollectionPropertiesFUCHSIA,
    pub get_buffer_device_address: PFN_vkGetBufferDeviceAddress,
    pub get_buffer_device_address_ext: PFN_vkGetBufferDeviceAddressEXT,
    pub get_buffer_device_address_khr: PFN_vkGetBufferDeviceAddressKHR,
    pub get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    pub get_buffer_memory_requirements2: PFN_vkGetBufferMemoryRequirements2,
    pub get_buffer_memory_requirements2_khr: PFN_vkGetBufferMemoryRequirements2KHR,
    pub get_buffer_opaque_capture_address: PFN_vkGetBufferOpaqueCaptureAddress,
    pub get_buffer_opaque_capture_address_khr: PFN_vkGetBufferOpaqueCaptureAddressKHR,
    pub get_buffer_opaque_capture_descriptor_data_ext:
        PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT,
    pub get_calibrated_timestamps_ext: PFN_vkGetCalibratedTimestampsEXT,
    pub get_calibrated_timestamps_khr: PFN_vkGetCalibratedTimestampsKHR,
    pub get_cluster_acceleration_structure_build_sizes_nv:
        PFN_vkGetClusterAccelerationStructureBuildSizesNV,
    #[cfg(feature = "beta")]
    pub get_cuda_module_cache_nv: PFN_vkGetCudaModuleCacheNV,
    pub get_data_graph_pipeline_available_properties_arm:
        PFN_vkGetDataGraphPipelineAvailablePropertiesARM,
    pub get_data_graph_pipeline_properties_arm: PFN_vkGetDataGraphPipelinePropertiesARM,
    pub get_data_graph_pipeline_session_bind_point_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM,
    pub get_data_graph_pipeline_session_memory_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM,
    pub get_deferred_operation_max_concurrency_khr: PFN_vkGetDeferredOperationMaxConcurrencyKHR,
    pub get_deferred_operation_result_khr: PFN_vkGetDeferredOperationResultKHR,
    pub get_descriptor_ext: PFN_vkGetDescriptorEXT,
    pub get_descriptor_set_host_mapping_valve: PFN_vkGetDescriptorSetHostMappingVALVE,
    pub get_descriptor_set_layout_binding_offset_ext: PFN_vkGetDescriptorSetLayoutBindingOffsetEXT,
    pub get_descriptor_set_layout_host_mapping_info_valve:
        PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    pub get_descriptor_set_layout_size_ext: PFN_vkGetDescriptorSetLayoutSizeEXT,
    pub get_descriptor_set_layout_support: PFN_vkGetDescriptorSetLayoutSupport,
    pub get_descriptor_set_layout_support_khr: PFN_vkGetDescriptorSetLayoutSupportKHR,
    pub get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
    pub get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    pub get_device_buffer_memory_requirements_khr: PFN_vkGetDeviceBufferMemoryRequirementsKHR,
    pub get_device_combined_image_sampler_index_nvx: PFN_vkGetDeviceCombinedImageSamplerIndexNVX,
    pub get_device_fault_debug_info_khr: PFN_vkGetDeviceFaultDebugInfoKHR,
    pub get_device_fault_info_ext: PFN_vkGetDeviceFaultInfoEXT,
    pub get_device_fault_reports_khr: PFN_vkGetDeviceFaultReportsKHR,
    pub get_device_group_peer_memory_features: PFN_vkGetDeviceGroupPeerMemoryFeatures,
    pub get_device_group_peer_memory_features_khr: PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR,
    pub get_device_group_present_capabilities_khr: PFN_vkGetDeviceGroupPresentCapabilitiesKHR,
    #[cfg(feature = "win32")]
    pub get_device_group_surface_present_modes2_ext: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
    pub get_device_group_surface_present_modes_khr: PFN_vkGetDeviceGroupSurfacePresentModesKHR,
    pub get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    pub get_device_image_memory_requirements_khr: PFN_vkGetDeviceImageMemoryRequirementsKHR,
    pub get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    pub get_device_image_sparse_memory_requirements_khr:
        PFN_vkGetDeviceImageSparseMemoryRequirementsKHR,
    pub get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    pub get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayoutKHR,
    pub get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    pub get_device_memory_opaque_capture_address: PFN_vkGetDeviceMemoryOpaqueCaptureAddress,
    pub get_device_memory_opaque_capture_address_khr: PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR,
    pub get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
    pub get_device_queue: PFN_vkGetDeviceQueue,
    pub get_device_queue2: PFN_vkGetDeviceQueue2,
    pub get_device_subpass_shading_max_workgroup_size_huawei:
        PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
    pub get_device_tensor_memory_requirements_arm: PFN_vkGetDeviceTensorMemoryRequirementsARM,
    pub get_dynamic_rendering_tile_properties_qcom: PFN_vkGetDynamicRenderingTilePropertiesQCOM,
    pub get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
    pub get_event_status: PFN_vkGetEventStatus,
    #[cfg(feature = "beta")]
    pub get_execution_graph_pipeline_node_index_amdx: PFN_vkGetExecutionGraphPipelineNodeIndexAMDX,
    #[cfg(feature = "beta")]
    pub get_execution_graph_pipeline_scratch_size_amdx:
        PFN_vkGetExecutionGraphPipelineScratchSizeAMDX,
    pub get_fence_fd_khr: PFN_vkGetFenceFdKHR,
    pub get_fence_status: PFN_vkGetFenceStatus,
    #[cfg(feature = "win32")]
    pub get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
    pub get_framebuffer_tile_properties_qcom: PFN_vkGetFramebufferTilePropertiesQCOM,
    pub get_generated_commands_memory_requirements_ext:
        PFN_vkGetGeneratedCommandsMemoryRequirementsEXT,
    pub get_generated_commands_memory_requirements_nv:
        PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    pub get_image_drm_format_modifier_properties_ext: PFN_vkGetImageDrmFormatModifierPropertiesEXT,
    pub get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    pub get_image_memory_requirements2: PFN_vkGetImageMemoryRequirements2,
    pub get_image_memory_requirements2_khr: PFN_vkGetImageMemoryRequirements2KHR,
    pub get_image_opaque_capture_data_ext: PFN_vkGetImageOpaqueCaptureDataEXT,
    pub get_image_opaque_capture_descriptor_data_ext: PFN_vkGetImageOpaqueCaptureDescriptorDataEXT,
    pub get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    pub get_image_sparse_memory_requirements2: PFN_vkGetImageSparseMemoryRequirements2,
    pub get_image_sparse_memory_requirements2_khr: PFN_vkGetImageSparseMemoryRequirements2KHR,
    pub get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    pub get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
    pub get_image_subresource_layout2_ext: PFN_vkGetImageSubresourceLayout2EXT,
    pub get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2KHR,
    pub get_image_view_address_nvx: PFN_vkGetImageViewAddressNVX,
    pub get_image_view_handle64_nvx: PFN_vkGetImageViewHandle64NVX,
    pub get_image_view_handle_nvx: PFN_vkGetImageViewHandleNVX,
    pub get_image_view_opaque_capture_descriptor_data_ext:
        PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT,
    pub get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
    #[cfg(feature = "android")]
    pub get_memory_android_hardware_buffer_android: PFN_vkGetMemoryAndroidHardwareBufferANDROID,
    pub get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    pub get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
    pub get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
    #[cfg(feature = "metal")]
    pub get_memory_metal_handle_ext: PFN_vkGetMemoryMetalHandleEXT,
    #[cfg(feature = "metal")]
    pub get_memory_metal_handle_properties_ext: PFN_vkGetMemoryMetalHandlePropertiesEXT,
    #[cfg(feature = "ohos")]
    pub get_memory_native_buffer_ohos: PFN_vkGetMemoryNativeBufferOHOS,
    pub get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
    #[cfg(feature = "win32")]
    pub get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    #[cfg(feature = "win32")]
    pub get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
    #[cfg(feature = "win32")]
    pub get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
    #[cfg(feature = "fuchsia")]
    pub get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
    #[cfg(feature = "fuchsia")]
    pub get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
    pub get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
    #[cfg(feature = "ohos")]
    pub get_native_buffer_properties_ohos: PFN_vkGetNativeBufferPropertiesOHOS,
    pub get_partitioned_acceleration_structures_build_sizes_nv:
        PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV,
    pub get_past_presentation_timing_ext: PFN_vkGetPastPresentationTimingEXT,
    pub get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
    pub get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
    pub get_pipeline_binary_data_khr: PFN_vkGetPipelineBinaryDataKHR,
    pub get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    pub get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
    pub get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    pub get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    pub get_pipeline_indirect_device_address_nv: PFN_vkGetPipelineIndirectDeviceAddressNV,
    pub get_pipeline_indirect_memory_requirements_nv: PFN_vkGetPipelineIndirectMemoryRequirementsNV,
    pub get_pipeline_key_khr: PFN_vkGetPipelineKeyKHR,
    pub get_pipeline_properties_ext: PFN_vkGetPipelinePropertiesEXT,
    pub get_private_data: PFN_vkGetPrivateData,
    pub get_private_data_ext: PFN_vkGetPrivateDataEXT,
    pub get_query_pool_results: PFN_vkGetQueryPoolResults,
    pub get_queue_checkpoint_data2_nv: PFN_vkGetQueueCheckpointData2NV,
    pub get_queue_checkpoint_data_nv: PFN_vkGetQueueCheckpointDataNV,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    pub get_ray_tracing_shader_group_handles_khr: PFN_vkGetRayTracingShaderGroupHandlesKHR,
    pub get_ray_tracing_shader_group_handles_nv: PFN_vkGetRayTracingShaderGroupHandlesNV,
    pub get_ray_tracing_shader_group_stack_size_khr: PFN_vkGetRayTracingShaderGroupStackSizeKHR,
    pub get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    pub get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    pub get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    pub get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularityKHR,
    pub get_sampler_opaque_capture_descriptor_data_ext:
        PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT,
    #[cfg(feature = "screen")]
    pub get_screen_buffer_properties_qnx: PFN_vkGetScreenBufferPropertiesQNX,
    pub get_semaphore_counter_value: PFN_vkGetSemaphoreCounterValue,
    pub get_semaphore_counter_value_khr: PFN_vkGetSemaphoreCounterValueKHR,
    pub get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
    #[cfg(feature = "win32")]
    pub get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
    #[cfg(feature = "fuchsia")]
    pub get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
    pub get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
    pub get_shader_info_amd: PFN_vkGetShaderInfoAMD,
    pub get_shader_instrumentation_values_arm: PFN_vkGetShaderInstrumentationValuesARM,
    pub get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
    pub get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    pub get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
    pub get_swapchain_images_khr: PFN_vkGetSwapchainImagesKHR,
    pub get_swapchain_status_khr: PFN_vkGetSwapchainStatusKHR,
    pub get_swapchain_time_domain_properties_ext: PFN_vkGetSwapchainTimeDomainPropertiesEXT,
    pub get_swapchain_timing_properties_ext: PFN_vkGetSwapchainTimingPropertiesEXT,
    pub get_tensor_memory_requirements_arm: PFN_vkGetTensorMemoryRequirementsARM,
    pub get_tensor_opaque_capture_data_arm: PFN_vkGetTensorOpaqueCaptureDataARM,
    pub get_tensor_opaque_capture_descriptor_data_arm:
        PFN_vkGetTensorOpaqueCaptureDescriptorDataARM,
    pub get_tensor_view_opaque_capture_descriptor_data_arm:
        PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM,
    pub get_validation_cache_data_ext: PFN_vkGetValidationCacheDataEXT,
    pub get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    pub import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    #[cfg(feature = "win32")]
    pub import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
    pub import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
    #[cfg(feature = "win32")]
    pub import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    #[cfg(feature = "fuchsia")]
    pub import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
    pub initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    pub invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    pub latency_sleep_nv: PFN_vkLatencySleepNV,
    pub map_memory: PFN_vkMapMemory,
    pub map_memory2: PFN_vkMapMemory2,
    pub map_memory2_khr: PFN_vkMapMemory2KHR,
    pub merge_pipeline_caches: PFN_vkMergePipelineCaches,
    pub merge_validation_caches_ext: PFN_vkMergeValidationCachesEXT,
    pub queue_begin_debug_utils_label_ext: PFN_vkQueueBeginDebugUtilsLabelEXT,
    pub queue_bind_sparse: PFN_vkQueueBindSparse,
    pub queue_end_debug_utils_label_ext: PFN_vkQueueEndDebugUtilsLabelEXT,
    pub queue_insert_debug_utils_label_ext: PFN_vkQueueInsertDebugUtilsLabelEXT,
    pub queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
    pub queue_present_khr: PFN_vkQueuePresentKHR,
    pub queue_set_perf_hint_qcom: PFN_vkQueueSetPerfHintQCOM,
    pub queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    pub queue_submit: PFN_vkQueueSubmit,
    pub queue_submit2: PFN_vkQueueSubmit2,
    pub queue_submit2_khr: PFN_vkQueueSubmit2KHR,
    pub queue_wait_idle: PFN_vkQueueWaitIdle,
    pub register_custom_border_color_ext: PFN_vkRegisterCustomBorderColorEXT,
    pub register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    pub register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    pub release_captured_pipeline_data_khr: PFN_vkReleaseCapturedPipelineDataKHR,
    #[cfg(feature = "win32")]
    pub release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    pub release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    pub release_profiling_lock_khr: PFN_vkReleaseProfilingLockKHR,
    pub release_swapchain_images_ext: PFN_vkReleaseSwapchainImagesEXT,
    pub release_swapchain_images_khr: PFN_vkReleaseSwapchainImagesKHR,
    pub reset_command_buffer: PFN_vkResetCommandBuffer,
    pub reset_command_pool: PFN_vkResetCommandPool,
    pub reset_descriptor_pool: PFN_vkResetDescriptorPool,
    pub reset_event: PFN_vkResetEvent,
    pub reset_fences: PFN_vkResetFences,
    pub reset_query_pool: PFN_vkResetQueryPool,
    pub reset_query_pool_ext: PFN_vkResetQueryPoolEXT,
    #[cfg(feature = "fuchsia")]
    pub set_buffer_collection_buffer_constraints_fuchsia:
        PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA,
    #[cfg(feature = "fuchsia")]
    pub set_buffer_collection_image_constraints_fuchsia:
        PFN_vkSetBufferCollectionImageConstraintsFUCHSIA,
    pub set_debug_utils_object_name_ext: PFN_vkSetDebugUtilsObjectNameEXT,
    pub set_debug_utils_object_tag_ext: PFN_vkSetDebugUtilsObjectTagEXT,
    pub set_device_memory_priority_ext: PFN_vkSetDeviceMemoryPriorityEXT,
    pub set_event: PFN_vkSetEvent,
    pub set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
    pub set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
    pub set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
    pub set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
    pub set_private_data: PFN_vkSetPrivateData,
    pub set_private_data_ext: PFN_vkSetPrivateDataEXT,
    pub set_swapchain_present_timing_queue_size_ext: PFN_vkSetSwapchainPresentTimingQueueSizeEXT,
    pub signal_semaphore: PFN_vkSignalSemaphore,
    pub signal_semaphore_khr: PFN_vkSignalSemaphoreKHR,
    pub transition_image_layout: PFN_vkTransitionImageLayout,
    pub transition_image_layout_ext: PFN_vkTransitionImageLayoutEXT,
    pub trim_command_pool: PFN_vkTrimCommandPool,
    pub trim_command_pool_khr: PFN_vkTrimCommandPoolKHR,
    pub uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    pub unmap_memory: PFN_vkUnmapMemory,
    pub unmap_memory2: PFN_vkUnmapMemory2,
    pub unmap_memory2_khr: PFN_vkUnmapMemory2KHR,
    pub unregister_custom_border_color_ext: PFN_vkUnregisterCustomBorderColorEXT,
    pub update_descriptor_set_with_template: PFN_vkUpdateDescriptorSetWithTemplate,
    pub update_descriptor_set_with_template_khr: PFN_vkUpdateDescriptorSetWithTemplateKHR,
    pub update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    pub update_indirect_execution_set_pipeline_ext: PFN_vkUpdateIndirectExecutionSetPipelineEXT,
    pub update_indirect_execution_set_shader_ext: PFN_vkUpdateIndirectExecutionSetShaderEXT,
    pub update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    pub wait_for_fences: PFN_vkWaitForFences,
    pub wait_for_present2_khr: PFN_vkWaitForPresent2KHR,
    pub wait_for_present_khr: PFN_vkWaitForPresentKHR,
    pub wait_semaphores: PFN_vkWaitSemaphores,
    pub wait_semaphores_khr: PFN_vkWaitSemaphoresKHR,
    pub write_acceleration_structures_properties_khr:
        PFN_vkWriteAccelerationStructuresPropertiesKHR,
    pub write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
    pub write_resource_descriptors_ext: PFN_vkWriteResourceDescriptorsEXT,
    pub write_sampler_descriptors_ext: PFN_vkWriteSamplerDescriptorsEXT,
}

impl DeviceFn {
    pub unsafe fn load(get_device_proc_addr: PFN_vkGetDeviceProcAddr, device: Device) -> Self {
        let get = get_device_proc_addr.expect("vkGetDeviceProcAddr is required");
        Self {
            #[cfg(feature = "win32")]
            acquire_full_screen_exclusive_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAcquireFullScreenExclusiveModeEXT\0"),
            )),
            acquire_next_image2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAcquireNextImage2KHR\0"),
            )),
            acquire_next_image_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAcquireNextImageKHR\0"),
            )),
            acquire_performance_configuration_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAcquirePerformanceConfigurationINTEL\0"),
            )),
            acquire_profiling_lock_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAcquireProfilingLockKHR\0"),
            )),
            allocate_command_buffers: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAllocateCommandBuffers\0"),
            )),
            allocate_descriptor_sets: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAllocateDescriptorSets\0"),
            )),
            allocate_memory: crate::load_pfn(get(device, crate::c_name_ptr(b"vkAllocateMemory\0"))),
            anti_lag_update_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkAntiLagUpdateAMD\0"),
            )),
            begin_command_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBeginCommandBuffer\0"),
            )),
            bind_acceleration_structure_memory_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindAccelerationStructureMemoryNV\0"),
            )),
            bind_buffer_memory: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindBufferMemory\0"),
            )),
            bind_buffer_memory2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindBufferMemory2\0"),
            )),
            bind_buffer_memory2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindBufferMemory2KHR\0"),
            )),
            bind_data_graph_pipeline_session_memory_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindDataGraphPipelineSessionMemoryARM\0"),
            )),
            bind_image_memory: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindImageMemory\0"),
            )),
            bind_image_memory2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindImageMemory2\0"),
            )),
            bind_image_memory2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindImageMemory2KHR\0"),
            )),
            bind_optical_flow_session_image_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindOpticalFlowSessionImageNV\0"),
            )),
            bind_tensor_memory_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindTensorMemoryARM\0"),
            )),
            bind_video_session_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBindVideoSessionMemoryKHR\0"),
            )),
            build_acceleration_structures_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBuildAccelerationStructuresKHR\0"),
            )),
            build_micromaps_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkBuildMicromapsEXT\0"),
            )),
            clear_shader_instrumentation_metrics_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkClearShaderInstrumentationMetricsARM\0"),
            )),
            cmd_begin_conditional_rendering2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginConditionalRendering2EXT\0"),
            )),
            cmd_begin_conditional_rendering_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginConditionalRenderingEXT\0"),
            )),
            cmd_begin_custom_resolve_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginCustomResolveEXT\0"),
            )),
            cmd_begin_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginDebugUtilsLabelEXT\0"),
            )),
            cmd_begin_per_tile_execution_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginPerTileExecutionQCOM\0"),
            )),
            cmd_begin_query: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdBeginQuery\0"))),
            cmd_begin_query_indexed_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginQueryIndexedEXT\0"),
            )),
            cmd_begin_render_pass: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginRenderPass\0"),
            )),
            cmd_begin_render_pass2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginRenderPass2\0"),
            )),
            cmd_begin_render_pass2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginRenderPass2KHR\0"),
            )),
            cmd_begin_rendering: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginRendering\0"),
            )),
            cmd_begin_rendering_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginRenderingKHR\0"),
            )),
            cmd_begin_shader_instrumentation_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginShaderInstrumentationARM\0"),
            )),
            cmd_begin_transform_feedback2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginTransformFeedback2EXT\0"),
            )),
            cmd_begin_transform_feedback_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginTransformFeedbackEXT\0"),
            )),
            cmd_begin_video_coding_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBeginVideoCodingKHR\0"),
            )),
            cmd_bind_descriptor_buffer_embedded_samplers2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT\0"),
            )),
            cmd_bind_descriptor_buffer_embedded_samplers_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorBufferEmbeddedSamplersEXT\0"),
            )),
            cmd_bind_descriptor_buffers_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorBuffersEXT\0"),
            )),
            cmd_bind_descriptor_sets: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorSets\0"),
            )),
            cmd_bind_descriptor_sets2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorSets2\0"),
            )),
            cmd_bind_descriptor_sets2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindDescriptorSets2KHR\0"),
            )),
            cmd_bind_index_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindIndexBuffer\0"),
            )),
            cmd_bind_index_buffer2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindIndexBuffer2\0"),
            )),
            cmd_bind_index_buffer2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindIndexBuffer2KHR\0"),
            )),
            cmd_bind_index_buffer3_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindIndexBuffer3KHR\0"),
            )),
            cmd_bind_invocation_mask_huawei: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindInvocationMaskHUAWEI\0"),
            )),
            cmd_bind_pipeline: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindPipeline\0"),
            )),
            cmd_bind_pipeline_shader_group_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindPipelineShaderGroupNV\0"),
            )),
            cmd_bind_resource_heap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindResourceHeapEXT\0"),
            )),
            cmd_bind_sampler_heap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindSamplerHeapEXT\0"),
            )),
            cmd_bind_shaders_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindShadersEXT\0"),
            )),
            cmd_bind_shading_rate_image_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindShadingRateImageNV\0"),
            )),
            cmd_bind_tile_memory_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindTileMemoryQCOM\0"),
            )),
            cmd_bind_transform_feedback_buffers2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindTransformFeedbackBuffers2EXT\0"),
            )),
            cmd_bind_transform_feedback_buffers_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindTransformFeedbackBuffersEXT\0"),
            )),
            cmd_bind_vertex_buffers: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindVertexBuffers\0"),
            )),
            cmd_bind_vertex_buffers2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindVertexBuffers2\0"),
            )),
            cmd_bind_vertex_buffers2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindVertexBuffers2EXT\0"),
            )),
            cmd_bind_vertex_buffers3_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBindVertexBuffers3KHR\0"),
            )),
            cmd_blit_image: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdBlitImage\0"))),
            cmd_blit_image2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdBlitImage2\0"))),
            cmd_blit_image2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBlitImage2KHR\0"),
            )),
            cmd_build_acceleration_structure_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildAccelerationStructureNV\0"),
            )),
            cmd_build_acceleration_structures_indirect_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildAccelerationStructuresIndirectKHR\0"),
            )),
            cmd_build_acceleration_structures_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildAccelerationStructuresKHR\0"),
            )),
            cmd_build_cluster_acceleration_structure_indirect_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildClusterAccelerationStructureIndirectNV\0"),
            )),
            cmd_build_micromaps_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildMicromapsEXT\0"),
            )),
            cmd_build_partitioned_acceleration_structures_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdBuildPartitionedAccelerationStructuresNV\0"),
            )),
            cmd_clear_attachments: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdClearAttachments\0"),
            )),
            cmd_clear_color_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdClearColorImage\0"),
            )),
            cmd_clear_depth_stencil_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdClearDepthStencilImage\0"),
            )),
            cmd_control_video_coding_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdControlVideoCodingKHR\0"),
            )),
            cmd_convert_cooperative_vector_matrix_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdConvertCooperativeVectorMatrixNV\0"),
            )),
            cmd_copy_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyAccelerationStructureKHR\0"),
            )),
            cmd_copy_acceleration_structure_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyAccelerationStructureNV\0"),
            )),
            cmd_copy_acceleration_structure_to_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyAccelerationStructureToMemoryKHR\0"),
            )),
            cmd_copy_buffer: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdCopyBuffer\0"))),
            cmd_copy_buffer2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyBuffer2\0"),
            )),
            cmd_copy_buffer2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyBuffer2KHR\0"),
            )),
            cmd_copy_buffer_to_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyBufferToImage\0"),
            )),
            cmd_copy_buffer_to_image2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyBufferToImage2\0"),
            )),
            cmd_copy_buffer_to_image2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyBufferToImage2KHR\0"),
            )),
            cmd_copy_image: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdCopyImage\0"))),
            cmd_copy_image2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdCopyImage2\0"))),
            cmd_copy_image2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyImage2KHR\0"),
            )),
            cmd_copy_image_to_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyImageToBuffer\0"),
            )),
            cmd_copy_image_to_buffer2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyImageToBuffer2\0"),
            )),
            cmd_copy_image_to_buffer2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyImageToBuffer2KHR\0"),
            )),
            cmd_copy_image_to_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyImageToMemoryKHR\0"),
            )),
            cmd_copy_memory_indirect_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryIndirectKHR\0"),
            )),
            cmd_copy_memory_indirect_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryIndirectNV\0"),
            )),
            cmd_copy_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryKHR\0"),
            )),
            cmd_copy_memory_to_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryToAccelerationStructureKHR\0"),
            )),
            cmd_copy_memory_to_image_indirect_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryToImageIndirectKHR\0"),
            )),
            cmd_copy_memory_to_image_indirect_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryToImageIndirectNV\0"),
            )),
            cmd_copy_memory_to_image_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryToImageKHR\0"),
            )),
            cmd_copy_memory_to_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMemoryToMicromapEXT\0"),
            )),
            cmd_copy_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMicromapEXT\0"),
            )),
            cmd_copy_micromap_to_memory_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyMicromapToMemoryEXT\0"),
            )),
            cmd_copy_query_pool_results: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyQueryPoolResults\0"),
            )),
            cmd_copy_query_pool_results_to_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyQueryPoolResultsToMemoryKHR\0"),
            )),
            cmd_copy_tensor_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCopyTensorARM\0"),
            )),
            cmd_cu_launch_kernel_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCuLaunchKernelNVX\0"),
            )),
            #[cfg(feature = "beta")]
            cmd_cuda_launch_kernel_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdCudaLaunchKernelNV\0"),
            )),
            cmd_debug_marker_begin_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDebugMarkerBeginEXT\0"),
            )),
            cmd_debug_marker_end_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDebugMarkerEndEXT\0"),
            )),
            cmd_debug_marker_insert_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDebugMarkerInsertEXT\0"),
            )),
            cmd_decode_video_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDecodeVideoKHR\0"),
            )),
            cmd_decompress_memory_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDecompressMemoryEXT\0"),
            )),
            cmd_decompress_memory_indirect_count_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDecompressMemoryIndirectCountEXT\0"),
            )),
            cmd_decompress_memory_indirect_count_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDecompressMemoryIndirectCountNV\0"),
            )),
            cmd_decompress_memory_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDecompressMemoryNV\0"),
            )),
            cmd_dispatch: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdDispatch\0"))),
            cmd_dispatch_base: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchBase\0"),
            )),
            cmd_dispatch_base_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchBaseKHR\0"),
            )),
            cmd_dispatch_data_graph_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchDataGraphARM\0"),
            )),
            #[cfg(feature = "beta")]
            cmd_dispatch_graph_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchGraphAMDX\0"),
            )),
            #[cfg(feature = "beta")]
            cmd_dispatch_graph_indirect_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchGraphIndirectAMDX\0"),
            )),
            #[cfg(feature = "beta")]
            cmd_dispatch_graph_indirect_count_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchGraphIndirectCountAMDX\0"),
            )),
            cmd_dispatch_indirect: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchIndirect\0"),
            )),
            cmd_dispatch_indirect2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchIndirect2KHR\0"),
            )),
            cmd_dispatch_tile_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDispatchTileQCOM\0"),
            )),
            cmd_draw: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdDraw\0"))),
            cmd_draw_cluster_huawei: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawClusterHUAWEI\0"),
            )),
            cmd_draw_cluster_indirect_huawei: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawClusterIndirectHUAWEI\0"),
            )),
            cmd_draw_indexed: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexed\0"),
            )),
            cmd_draw_indexed_indirect: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirect\0"),
            )),
            cmd_draw_indexed_indirect2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirect2KHR\0"),
            )),
            cmd_draw_indexed_indirect_count: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirectCount\0"),
            )),
            cmd_draw_indexed_indirect_count2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirectCount2KHR\0"),
            )),
            cmd_draw_indexed_indirect_count_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirectCountAMD\0"),
            )),
            cmd_draw_indexed_indirect_count_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndexedIndirectCountKHR\0"),
            )),
            cmd_draw_indirect: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirect\0"),
            )),
            cmd_draw_indirect2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirect2KHR\0"),
            )),
            cmd_draw_indirect_byte_count2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectByteCount2EXT\0"),
            )),
            cmd_draw_indirect_byte_count_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectByteCountEXT\0"),
            )),
            cmd_draw_indirect_count: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectCount\0"),
            )),
            cmd_draw_indirect_count2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectCount2KHR\0"),
            )),
            cmd_draw_indirect_count_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectCountAMD\0"),
            )),
            cmd_draw_indirect_count_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawIndirectCountKHR\0"),
            )),
            cmd_draw_mesh_tasks_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksEXT\0"),
            )),
            cmd_draw_mesh_tasks_indirect2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirect2EXT\0"),
            )),
            cmd_draw_mesh_tasks_indirect_count2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirectCount2EXT\0"),
            )),
            cmd_draw_mesh_tasks_indirect_count_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirectCountEXT\0"),
            )),
            cmd_draw_mesh_tasks_indirect_count_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirectCountNV\0"),
            )),
            cmd_draw_mesh_tasks_indirect_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirectEXT\0"),
            )),
            cmd_draw_mesh_tasks_indirect_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksIndirectNV\0"),
            )),
            cmd_draw_mesh_tasks_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMeshTasksNV\0"),
            )),
            cmd_draw_multi_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMultiEXT\0"),
            )),
            cmd_draw_multi_indexed_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdDrawMultiIndexedEXT\0"),
            )),
            cmd_encode_video_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEncodeVideoKHR\0"),
            )),
            cmd_end_conditional_rendering_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndConditionalRenderingEXT\0"),
            )),
            cmd_end_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndDebugUtilsLabelEXT\0"),
            )),
            cmd_end_per_tile_execution_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndPerTileExecutionQCOM\0"),
            )),
            cmd_end_query: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdEndQuery\0"))),
            cmd_end_query_indexed_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndQueryIndexedEXT\0"),
            )),
            cmd_end_render_pass: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRenderPass\0"),
            )),
            cmd_end_render_pass2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRenderPass2\0"),
            )),
            cmd_end_render_pass2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRenderPass2KHR\0"),
            )),
            cmd_end_rendering: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRendering\0"),
            )),
            cmd_end_rendering2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRendering2EXT\0"),
            )),
            cmd_end_rendering2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRendering2KHR\0"),
            )),
            cmd_end_rendering_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndRenderingKHR\0"),
            )),
            cmd_end_shader_instrumentation_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndShaderInstrumentationARM\0"),
            )),
            cmd_end_transform_feedback2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndTransformFeedback2EXT\0"),
            )),
            cmd_end_transform_feedback_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndTransformFeedbackEXT\0"),
            )),
            cmd_end_video_coding_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdEndVideoCodingKHR\0"),
            )),
            cmd_execute_commands: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdExecuteCommands\0"),
            )),
            cmd_execute_generated_commands_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdExecuteGeneratedCommandsEXT\0"),
            )),
            cmd_execute_generated_commands_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdExecuteGeneratedCommandsNV\0"),
            )),
            cmd_fill_buffer: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdFillBuffer\0"))),
            cmd_fill_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdFillMemoryKHR\0"),
            )),
            #[cfg(feature = "beta")]
            cmd_initialize_graph_scratch_memory_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdInitializeGraphScratchMemoryAMDX\0"),
            )),
            cmd_insert_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdInsertDebugUtilsLabelEXT\0"),
            )),
            cmd_next_subpass: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdNextSubpass\0"),
            )),
            cmd_next_subpass2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdNextSubpass2\0"),
            )),
            cmd_next_subpass2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdNextSubpass2KHR\0"),
            )),
            cmd_optical_flow_execute_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdOpticalFlowExecuteNV\0"),
            )),
            cmd_pipeline_barrier: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPipelineBarrier\0"),
            )),
            cmd_pipeline_barrier2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPipelineBarrier2\0"),
            )),
            cmd_pipeline_barrier2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPipelineBarrier2KHR\0"),
            )),
            cmd_preprocess_generated_commands_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPreprocessGeneratedCommandsEXT\0"),
            )),
            cmd_preprocess_generated_commands_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPreprocessGeneratedCommandsNV\0"),
            )),
            cmd_push_constants: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushConstants\0"),
            )),
            cmd_push_constants2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushConstants2\0"),
            )),
            cmd_push_constants2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushConstants2KHR\0"),
            )),
            cmd_push_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDataEXT\0"),
            )),
            cmd_push_descriptor_set: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSet\0"),
            )),
            cmd_push_descriptor_set2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSet2\0"),
            )),
            cmd_push_descriptor_set2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSet2KHR\0"),
            )),
            cmd_push_descriptor_set_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSetKHR\0"),
            )),
            cmd_push_descriptor_set_with_template: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSetWithTemplate\0"),
            )),
            cmd_push_descriptor_set_with_template2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSetWithTemplate2\0"),
            )),
            cmd_push_descriptor_set_with_template2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSetWithTemplate2KHR\0"),
            )),
            cmd_push_descriptor_set_with_template_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdPushDescriptorSetWithTemplateKHR\0"),
            )),
            cmd_reset_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdResetEvent\0"))),
            cmd_reset_event2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResetEvent2\0"),
            )),
            cmd_reset_event2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResetEvent2KHR\0"),
            )),
            cmd_reset_query_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResetQueryPool\0"),
            )),
            cmd_resolve_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResolveImage\0"),
            )),
            cmd_resolve_image2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResolveImage2\0"),
            )),
            cmd_resolve_image2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdResolveImage2KHR\0"),
            )),
            cmd_set_alpha_to_coverage_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetAlphaToCoverageEnableEXT\0"),
            )),
            cmd_set_alpha_to_one_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetAlphaToOneEnableEXT\0"),
            )),
            cmd_set_attachment_feedback_loop_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetAttachmentFeedbackLoopEnableEXT\0"),
            )),
            cmd_set_blend_constants: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetBlendConstants\0"),
            )),
            cmd_set_checkpoint_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCheckpointNV\0"),
            )),
            cmd_set_coarse_sample_order_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoarseSampleOrderNV\0"),
            )),
            cmd_set_color_blend_advanced_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetColorBlendAdvancedEXT\0"),
            )),
            cmd_set_color_blend_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetColorBlendEnableEXT\0"),
            )),
            cmd_set_color_blend_equation_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetColorBlendEquationEXT\0"),
            )),
            cmd_set_color_write_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetColorWriteEnableEXT\0"),
            )),
            cmd_set_color_write_mask_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetColorWriteMaskEXT\0"),
            )),
            cmd_set_compute_occupancy_priority_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetComputeOccupancyPriorityNV\0"),
            )),
            cmd_set_conservative_rasterization_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetConservativeRasterizationModeEXT\0"),
            )),
            cmd_set_coverage_modulation_mode_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageModulationModeNV\0"),
            )),
            cmd_set_coverage_modulation_table_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageModulationTableEnableNV\0"),
            )),
            cmd_set_coverage_modulation_table_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageModulationTableNV\0"),
            )),
            cmd_set_coverage_reduction_mode_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageReductionModeNV\0"),
            )),
            cmd_set_coverage_to_color_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageToColorEnableNV\0"),
            )),
            cmd_set_coverage_to_color_location_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCoverageToColorLocationNV\0"),
            )),
            cmd_set_cull_mode: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCullMode\0"),
            )),
            cmd_set_cull_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetCullModeEXT\0"),
            )),
            cmd_set_depth_bias: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBias\0"),
            )),
            cmd_set_depth_bias2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBias2EXT\0"),
            )),
            cmd_set_depth_bias_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBiasEnable\0"),
            )),
            cmd_set_depth_bias_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBiasEnableEXT\0"),
            )),
            cmd_set_depth_bounds: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBounds\0"),
            )),
            cmd_set_depth_bounds_test_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBoundsTestEnable\0"),
            )),
            cmd_set_depth_bounds_test_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthBoundsTestEnableEXT\0"),
            )),
            cmd_set_depth_clamp_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthClampEnableEXT\0"),
            )),
            cmd_set_depth_clamp_range_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthClampRangeEXT\0"),
            )),
            cmd_set_depth_clip_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthClipEnableEXT\0"),
            )),
            cmd_set_depth_clip_negative_one_to_one_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthClipNegativeOneToOneEXT\0"),
            )),
            cmd_set_depth_compare_op: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthCompareOp\0"),
            )),
            cmd_set_depth_compare_op_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthCompareOpEXT\0"),
            )),
            cmd_set_depth_test_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthTestEnable\0"),
            )),
            cmd_set_depth_test_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthTestEnableEXT\0"),
            )),
            cmd_set_depth_write_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthWriteEnable\0"),
            )),
            cmd_set_depth_write_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDepthWriteEnableEXT\0"),
            )),
            cmd_set_descriptor_buffer_offsets2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDescriptorBufferOffsets2EXT\0"),
            )),
            cmd_set_descriptor_buffer_offsets_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDescriptorBufferOffsetsEXT\0"),
            )),
            cmd_set_device_mask: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDeviceMask\0"),
            )),
            cmd_set_device_mask_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDeviceMaskKHR\0"),
            )),
            cmd_set_discard_rectangle_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDiscardRectangleEXT\0"),
            )),
            cmd_set_discard_rectangle_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDiscardRectangleEnableEXT\0"),
            )),
            cmd_set_discard_rectangle_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDiscardRectangleModeEXT\0"),
            )),
            cmd_set_dispatch_parameters_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetDispatchParametersARM\0"),
            )),
            cmd_set_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdSetEvent\0"))),
            cmd_set_event2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdSetEvent2\0"))),
            cmd_set_event2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetEvent2KHR\0"),
            )),
            cmd_set_exclusive_scissor_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetExclusiveScissorEnableNV\0"),
            )),
            cmd_set_exclusive_scissor_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetExclusiveScissorNV\0"),
            )),
            cmd_set_extra_primitive_overestimation_size_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetExtraPrimitiveOverestimationSizeEXT\0"),
            )),
            cmd_set_fragment_shading_rate_enum_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetFragmentShadingRateEnumNV\0"),
            )),
            cmd_set_fragment_shading_rate_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetFragmentShadingRateKHR\0"),
            )),
            cmd_set_front_face: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetFrontFace\0"),
            )),
            cmd_set_front_face_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetFrontFaceEXT\0"),
            )),
            cmd_set_line_rasterization_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineRasterizationModeEXT\0"),
            )),
            cmd_set_line_stipple: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineStipple\0"),
            )),
            cmd_set_line_stipple_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineStippleEXT\0"),
            )),
            cmd_set_line_stipple_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineStippleEnableEXT\0"),
            )),
            cmd_set_line_stipple_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineStippleKHR\0"),
            )),
            cmd_set_line_width: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLineWidth\0"),
            )),
            cmd_set_logic_op_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLogicOpEXT\0"),
            )),
            cmd_set_logic_op_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetLogicOpEnableEXT\0"),
            )),
            cmd_set_patch_control_points_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPatchControlPointsEXT\0"),
            )),
            cmd_set_performance_marker_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPerformanceMarkerINTEL\0"),
            )),
            cmd_set_performance_override_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPerformanceOverrideINTEL\0"),
            )),
            cmd_set_performance_stream_marker_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPerformanceStreamMarkerINTEL\0"),
            )),
            cmd_set_polygon_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPolygonModeEXT\0"),
            )),
            cmd_set_primitive_restart_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPrimitiveRestartEnable\0"),
            )),
            cmd_set_primitive_restart_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPrimitiveRestartEnableEXT\0"),
            )),
            cmd_set_primitive_restart_index_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPrimitiveRestartIndexEXT\0"),
            )),
            cmd_set_primitive_topology: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPrimitiveTopology\0"),
            )),
            cmd_set_primitive_topology_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetPrimitiveTopologyEXT\0"),
            )),
            cmd_set_provoking_vertex_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetProvokingVertexModeEXT\0"),
            )),
            cmd_set_rasterization_samples_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRasterizationSamplesEXT\0"),
            )),
            cmd_set_rasterization_stream_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRasterizationStreamEXT\0"),
            )),
            cmd_set_rasterizer_discard_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRasterizerDiscardEnable\0"),
            )),
            cmd_set_rasterizer_discard_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRasterizerDiscardEnableEXT\0"),
            )),
            cmd_set_ray_tracing_pipeline_stack_size_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRayTracingPipelineStackSizeKHR\0"),
            )),
            cmd_set_rendering_attachment_locations: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRenderingAttachmentLocations\0"),
            )),
            cmd_set_rendering_attachment_locations_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRenderingAttachmentLocationsKHR\0"),
            )),
            cmd_set_rendering_input_attachment_indices: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRenderingInputAttachmentIndices\0"),
            )),
            cmd_set_rendering_input_attachment_indices_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRenderingInputAttachmentIndicesKHR\0"),
            )),
            cmd_set_representative_fragment_test_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetRepresentativeFragmentTestEnableNV\0"),
            )),
            cmd_set_sample_locations_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetSampleLocationsEXT\0"),
            )),
            cmd_set_sample_locations_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetSampleLocationsEnableEXT\0"),
            )),
            cmd_set_sample_mask_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetSampleMaskEXT\0"),
            )),
            cmd_set_scissor: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdSetScissor\0"))),
            cmd_set_scissor_with_count: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetScissorWithCount\0"),
            )),
            cmd_set_scissor_with_count_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetScissorWithCountEXT\0"),
            )),
            cmd_set_shading_rate_image_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetShadingRateImageEnableNV\0"),
            )),
            cmd_set_stencil_compare_mask: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilCompareMask\0"),
            )),
            cmd_set_stencil_op: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilOp\0"),
            )),
            cmd_set_stencil_op_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilOpEXT\0"),
            )),
            cmd_set_stencil_reference: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilReference\0"),
            )),
            cmd_set_stencil_test_enable: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilTestEnable\0"),
            )),
            cmd_set_stencil_test_enable_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilTestEnableEXT\0"),
            )),
            cmd_set_stencil_write_mask: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetStencilWriteMask\0"),
            )),
            cmd_set_tessellation_domain_origin_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetTessellationDomainOriginEXT\0"),
            )),
            cmd_set_vertex_input_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetVertexInputEXT\0"),
            )),
            cmd_set_viewport: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewport\0"),
            )),
            cmd_set_viewport_shading_rate_palette_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportShadingRatePaletteNV\0"),
            )),
            cmd_set_viewport_swizzle_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportSwizzleNV\0"),
            )),
            cmd_set_viewport_w_scaling_enable_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportWScalingEnableNV\0"),
            )),
            cmd_set_viewport_w_scaling_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportWScalingNV\0"),
            )),
            cmd_set_viewport_with_count: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportWithCount\0"),
            )),
            cmd_set_viewport_with_count_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSetViewportWithCountEXT\0"),
            )),
            cmd_subpass_shading_huawei: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdSubpassShadingHUAWEI\0"),
            )),
            cmd_trace_rays_indirect2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdTraceRaysIndirect2KHR\0"),
            )),
            cmd_trace_rays_indirect_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdTraceRaysIndirectKHR\0"),
            )),
            cmd_trace_rays_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdTraceRaysKHR\0"),
            )),
            cmd_trace_rays_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdTraceRaysNV\0"),
            )),
            cmd_update_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdUpdateBuffer\0"),
            )),
            cmd_update_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdUpdateMemoryKHR\0"),
            )),
            cmd_update_pipeline_indirect_buffer_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdUpdatePipelineIndirectBufferNV\0"),
            )),
            cmd_wait_events: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCmdWaitEvents\0"))),
            cmd_wait_events2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWaitEvents2\0"),
            )),
            cmd_wait_events2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWaitEvents2KHR\0"),
            )),
            cmd_write_acceleration_structures_properties_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteAccelerationStructuresPropertiesKHR\0"),
            )),
            cmd_write_acceleration_structures_properties_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteAccelerationStructuresPropertiesNV\0"),
            )),
            cmd_write_buffer_marker2_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteBufferMarker2AMD\0"),
            )),
            cmd_write_buffer_marker_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteBufferMarkerAMD\0"),
            )),
            cmd_write_marker_to_memory_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteMarkerToMemoryAMD\0"),
            )),
            cmd_write_micromaps_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteMicromapsPropertiesEXT\0"),
            )),
            cmd_write_timestamp: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteTimestamp\0"),
            )),
            cmd_write_timestamp2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteTimestamp2\0"),
            )),
            cmd_write_timestamp2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCmdWriteTimestamp2KHR\0"),
            )),
            compile_deferred_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCompileDeferredNV\0"),
            )),
            convert_cooperative_vector_matrix_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkConvertCooperativeVectorMatrixNV\0"),
            )),
            copy_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyAccelerationStructureKHR\0"),
            )),
            copy_acceleration_structure_to_memory_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyAccelerationStructureToMemoryKHR\0"),
            )),
            copy_image_to_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyImageToImage\0"),
            )),
            copy_image_to_image_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyImageToImageEXT\0"),
            )),
            copy_image_to_memory: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyImageToMemory\0"),
            )),
            copy_image_to_memory_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyImageToMemoryEXT\0"),
            )),
            copy_memory_to_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMemoryToAccelerationStructureKHR\0"),
            )),
            copy_memory_to_image: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMemoryToImage\0"),
            )),
            copy_memory_to_image_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMemoryToImageEXT\0"),
            )),
            copy_memory_to_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMemoryToMicromapEXT\0"),
            )),
            copy_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMicromapEXT\0"),
            )),
            copy_micromap_to_memory_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCopyMicromapToMemoryEXT\0"),
            )),
            create_acceleration_structure2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateAccelerationStructure2KHR\0"),
            )),
            create_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateAccelerationStructureKHR\0"),
            )),
            create_acceleration_structure_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateAccelerationStructureNV\0"),
            )),
            create_buffer: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCreateBuffer\0"))),
            #[cfg(feature = "fuchsia")]
            create_buffer_collection_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateBufferCollectionFUCHSIA\0"),
            )),
            create_buffer_view: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateBufferView\0"),
            )),
            create_command_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateCommandPool\0"),
            )),
            create_compute_pipelines: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateComputePipelines\0"),
            )),
            create_cu_function_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateCuFunctionNVX\0"),
            )),
            create_cu_module_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateCuModuleNVX\0"),
            )),
            #[cfg(feature = "beta")]
            create_cuda_function_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateCudaFunctionNV\0"),
            )),
            #[cfg(feature = "beta")]
            create_cuda_module_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateCudaModuleNV\0"),
            )),
            create_data_graph_pipeline_session_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDataGraphPipelineSessionARM\0"),
            )),
            create_data_graph_pipelines_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDataGraphPipelinesARM\0"),
            )),
            create_deferred_operation_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDeferredOperationKHR\0"),
            )),
            create_descriptor_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDescriptorPool\0"),
            )),
            create_descriptor_set_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDescriptorSetLayout\0"),
            )),
            create_descriptor_update_template: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDescriptorUpdateTemplate\0"),
            )),
            create_descriptor_update_template_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateDescriptorUpdateTemplateKHR\0"),
            )),
            create_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCreateEvent\0"))),
            #[cfg(feature = "beta")]
            create_execution_graph_pipelines_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateExecutionGraphPipelinesAMDX\0"),
            )),
            create_external_compute_queue_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateExternalComputeQueueNV\0"),
            )),
            create_fence: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCreateFence\0"))),
            create_framebuffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateFramebuffer\0"),
            )),
            create_graphics_pipelines: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateGraphicsPipelines\0"),
            )),
            create_image: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCreateImage\0"))),
            create_image_view: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateImageView\0"),
            )),
            create_indirect_commands_layout_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateIndirectCommandsLayoutEXT\0"),
            )),
            create_indirect_commands_layout_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateIndirectCommandsLayoutNV\0"),
            )),
            create_indirect_execution_set_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateIndirectExecutionSetEXT\0"),
            )),
            create_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateMicromapEXT\0"),
            )),
            create_optical_flow_session_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateOpticalFlowSessionNV\0"),
            )),
            create_pipeline_binaries_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreatePipelineBinariesKHR\0"),
            )),
            create_pipeline_cache: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreatePipelineCache\0"),
            )),
            create_pipeline_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreatePipelineLayout\0"),
            )),
            create_private_data_slot: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreatePrivateDataSlot\0"),
            )),
            create_private_data_slot_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreatePrivateDataSlotEXT\0"),
            )),
            create_query_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateQueryPool\0"),
            )),
            create_ray_tracing_pipelines_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateRayTracingPipelinesKHR\0"),
            )),
            create_ray_tracing_pipelines_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateRayTracingPipelinesNV\0"),
            )),
            create_render_pass: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateRenderPass\0"),
            )),
            create_render_pass2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateRenderPass2\0"),
            )),
            create_render_pass2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateRenderPass2KHR\0"),
            )),
            create_sampler: crate::load_pfn(get(device, crate::c_name_ptr(b"vkCreateSampler\0"))),
            create_sampler_ycbcr_conversion: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateSamplerYcbcrConversion\0"),
            )),
            create_sampler_ycbcr_conversion_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateSamplerYcbcrConversionKHR\0"),
            )),
            create_semaphore: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateSemaphore\0"),
            )),
            create_shader_instrumentation_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateShaderInstrumentationARM\0"),
            )),
            create_shader_module: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateShaderModule\0"),
            )),
            create_shaders_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateShadersEXT\0"),
            )),
            create_shared_swapchains_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateSharedSwapchainsKHR\0"),
            )),
            create_swapchain_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateSwapchainKHR\0"),
            )),
            create_tensor_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateTensorARM\0"),
            )),
            create_tensor_view_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateTensorViewARM\0"),
            )),
            create_validation_cache_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateValidationCacheEXT\0"),
            )),
            create_video_session_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateVideoSessionKHR\0"),
            )),
            create_video_session_parameters_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkCreateVideoSessionParametersKHR\0"),
            )),
            debug_marker_set_object_name_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDebugMarkerSetObjectNameEXT\0"),
            )),
            debug_marker_set_object_tag_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDebugMarkerSetObjectTagEXT\0"),
            )),
            deferred_operation_join_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDeferredOperationJoinKHR\0"),
            )),
            destroy_acceleration_structure_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyAccelerationStructureKHR\0"),
            )),
            destroy_acceleration_structure_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyAccelerationStructureNV\0"),
            )),
            destroy_buffer: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroyBuffer\0"))),
            #[cfg(feature = "fuchsia")]
            destroy_buffer_collection_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyBufferCollectionFUCHSIA\0"),
            )),
            destroy_buffer_view: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyBufferView\0"),
            )),
            destroy_command_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyCommandPool\0"),
            )),
            destroy_cu_function_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyCuFunctionNVX\0"),
            )),
            destroy_cu_module_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyCuModuleNVX\0"),
            )),
            #[cfg(feature = "beta")]
            destroy_cuda_function_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyCudaFunctionNV\0"),
            )),
            #[cfg(feature = "beta")]
            destroy_cuda_module_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyCudaModuleNV\0"),
            )),
            destroy_data_graph_pipeline_session_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDataGraphPipelineSessionARM\0"),
            )),
            destroy_deferred_operation_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDeferredOperationKHR\0"),
            )),
            destroy_descriptor_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDescriptorPool\0"),
            )),
            destroy_descriptor_set_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDescriptorSetLayout\0"),
            )),
            destroy_descriptor_update_template: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDescriptorUpdateTemplate\0"),
            )),
            destroy_descriptor_update_template_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyDescriptorUpdateTemplateKHR\0"),
            )),
            destroy_device: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroyDevice\0"))),
            destroy_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroyEvent\0"))),
            destroy_external_compute_queue_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyExternalComputeQueueNV\0"),
            )),
            destroy_fence: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroyFence\0"))),
            destroy_framebuffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyFramebuffer\0"),
            )),
            destroy_image: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroyImage\0"))),
            destroy_image_view: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyImageView\0"),
            )),
            destroy_indirect_commands_layout_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyIndirectCommandsLayoutEXT\0"),
            )),
            destroy_indirect_commands_layout_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyIndirectCommandsLayoutNV\0"),
            )),
            destroy_indirect_execution_set_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyIndirectExecutionSetEXT\0"),
            )),
            destroy_micromap_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyMicromapEXT\0"),
            )),
            destroy_optical_flow_session_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyOpticalFlowSessionNV\0"),
            )),
            destroy_pipeline: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPipeline\0"),
            )),
            destroy_pipeline_binary_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPipelineBinaryKHR\0"),
            )),
            destroy_pipeline_cache: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPipelineCache\0"),
            )),
            destroy_pipeline_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPipelineLayout\0"),
            )),
            destroy_private_data_slot: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPrivateDataSlot\0"),
            )),
            destroy_private_data_slot_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyPrivateDataSlotEXT\0"),
            )),
            destroy_query_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyQueryPool\0"),
            )),
            destroy_render_pass: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyRenderPass\0"),
            )),
            destroy_sampler: crate::load_pfn(get(device, crate::c_name_ptr(b"vkDestroySampler\0"))),
            destroy_sampler_ycbcr_conversion: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroySamplerYcbcrConversion\0"),
            )),
            destroy_sampler_ycbcr_conversion_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroySamplerYcbcrConversionKHR\0"),
            )),
            destroy_semaphore: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroySemaphore\0"),
            )),
            destroy_shader_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyShaderEXT\0"),
            )),
            destroy_shader_instrumentation_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyShaderInstrumentationARM\0"),
            )),
            destroy_shader_module: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyShaderModule\0"),
            )),
            destroy_swapchain_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroySwapchainKHR\0"),
            )),
            destroy_tensor_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyTensorARM\0"),
            )),
            destroy_tensor_view_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyTensorViewARM\0"),
            )),
            destroy_validation_cache_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyValidationCacheEXT\0"),
            )),
            destroy_video_session_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyVideoSessionKHR\0"),
            )),
            destroy_video_session_parameters_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDestroyVideoSessionParametersKHR\0"),
            )),
            device_wait_idle: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDeviceWaitIdle\0"),
            )),
            display_power_control_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkDisplayPowerControlEXT\0"),
            )),
            end_command_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkEndCommandBuffer\0"),
            )),
            #[cfg(feature = "metal")]
            export_metal_objects_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkExportMetalObjectsEXT\0"),
            )),
            flush_mapped_memory_ranges: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkFlushMappedMemoryRanges\0"),
            )),
            free_command_buffers: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkFreeCommandBuffers\0"),
            )),
            free_descriptor_sets: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkFreeDescriptorSets\0"),
            )),
            free_memory: crate::load_pfn(get(device, crate::c_name_ptr(b"vkFreeMemory\0"))),
            get_acceleration_structure_build_sizes_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAccelerationStructureBuildSizesKHR\0"),
            )),
            get_acceleration_structure_device_address_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAccelerationStructureDeviceAddressKHR\0"),
            )),
            get_acceleration_structure_handle_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAccelerationStructureHandleNV\0"),
            )),
            get_acceleration_structure_memory_requirements_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAccelerationStructureMemoryRequirementsNV\0"),
            )),
            get_acceleration_structure_opaque_capture_descriptor_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT\0"),
            )),
            #[cfg(feature = "android")]
            get_android_hardware_buffer_properties_android: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetAndroidHardwareBufferPropertiesANDROID\0"),
            )),
            #[cfg(feature = "fuchsia")]
            get_buffer_collection_properties_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferCollectionPropertiesFUCHSIA\0"),
            )),
            get_buffer_device_address: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferDeviceAddress\0"),
            )),
            get_buffer_device_address_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferDeviceAddressEXT\0"),
            )),
            get_buffer_device_address_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferDeviceAddressKHR\0"),
            )),
            get_buffer_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferMemoryRequirements\0"),
            )),
            get_buffer_memory_requirements2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferMemoryRequirements2\0"),
            )),
            get_buffer_memory_requirements2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferMemoryRequirements2KHR\0"),
            )),
            get_buffer_opaque_capture_address: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferOpaqueCaptureAddress\0"),
            )),
            get_buffer_opaque_capture_address_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferOpaqueCaptureAddressKHR\0"),
            )),
            get_buffer_opaque_capture_descriptor_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetBufferOpaqueCaptureDescriptorDataEXT\0"),
            )),
            get_calibrated_timestamps_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetCalibratedTimestampsEXT\0"),
            )),
            get_calibrated_timestamps_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetCalibratedTimestampsKHR\0"),
            )),
            get_cluster_acceleration_structure_build_sizes_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetClusterAccelerationStructureBuildSizesNV\0"),
            )),
            #[cfg(feature = "beta")]
            get_cuda_module_cache_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetCudaModuleCacheNV\0"),
            )),
            get_data_graph_pipeline_available_properties_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDataGraphPipelineAvailablePropertiesARM\0"),
            )),
            get_data_graph_pipeline_properties_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDataGraphPipelinePropertiesARM\0"),
            )),
            get_data_graph_pipeline_session_bind_point_requirements_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDataGraphPipelineSessionBindPointRequirementsARM\0"),
            )),
            get_data_graph_pipeline_session_memory_requirements_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDataGraphPipelineSessionMemoryRequirementsARM\0"),
            )),
            get_deferred_operation_max_concurrency_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeferredOperationMaxConcurrencyKHR\0"),
            )),
            get_deferred_operation_result_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeferredOperationResultKHR\0"),
            )),
            get_descriptor_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorEXT\0"),
            )),
            get_descriptor_set_host_mapping_valve: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetHostMappingVALVE\0"),
            )),
            get_descriptor_set_layout_binding_offset_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetLayoutBindingOffsetEXT\0"),
            )),
            get_descriptor_set_layout_host_mapping_info_valve: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetLayoutHostMappingInfoVALVE\0"),
            )),
            get_descriptor_set_layout_size_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetLayoutSizeEXT\0"),
            )),
            get_descriptor_set_layout_support: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetLayoutSupport\0"),
            )),
            get_descriptor_set_layout_support_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDescriptorSetLayoutSupportKHR\0"),
            )),
            get_device_acceleration_structure_compatibility_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceAccelerationStructureCompatibilityKHR\0"),
            )),
            get_device_buffer_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceBufferMemoryRequirements\0"),
            )),
            get_device_buffer_memory_requirements_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceBufferMemoryRequirementsKHR\0"),
            )),
            get_device_combined_image_sampler_index_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceCombinedImageSamplerIndexNVX\0"),
            )),
            get_device_fault_debug_info_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceFaultDebugInfoKHR\0"),
            )),
            get_device_fault_info_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceFaultInfoEXT\0"),
            )),
            get_device_fault_reports_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceFaultReportsKHR\0"),
            )),
            get_device_group_peer_memory_features: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceGroupPeerMemoryFeatures\0"),
            )),
            get_device_group_peer_memory_features_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceGroupPeerMemoryFeaturesKHR\0"),
            )),
            get_device_group_present_capabilities_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceGroupPresentCapabilitiesKHR\0"),
            )),
            #[cfg(feature = "win32")]
            get_device_group_surface_present_modes2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceGroupSurfacePresentModes2EXT\0"),
            )),
            get_device_group_surface_present_modes_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceGroupSurfacePresentModesKHR\0"),
            )),
            get_device_image_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageMemoryRequirements\0"),
            )),
            get_device_image_memory_requirements_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageMemoryRequirementsKHR\0"),
            )),
            get_device_image_sparse_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageSparseMemoryRequirements\0"),
            )),
            get_device_image_sparse_memory_requirements_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageSparseMemoryRequirementsKHR\0"),
            )),
            get_device_image_subresource_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageSubresourceLayout\0"),
            )),
            get_device_image_subresource_layout_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceImageSubresourceLayoutKHR\0"),
            )),
            get_device_memory_commitment: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceMemoryCommitment\0"),
            )),
            get_device_memory_opaque_capture_address: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceMemoryOpaqueCaptureAddress\0"),
            )),
            get_device_memory_opaque_capture_address_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0"),
            )),
            get_device_micromap_compatibility_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceMicromapCompatibilityEXT\0"),
            )),
            get_device_queue: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceQueue\0"),
            )),
            get_device_queue2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceQueue2\0"),
            )),
            get_device_subpass_shading_max_workgroup_size_huawei: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0"),
            )),
            get_device_tensor_memory_requirements_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDeviceTensorMemoryRequirementsARM\0"),
            )),
            get_dynamic_rendering_tile_properties_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetDynamicRenderingTilePropertiesQCOM\0"),
            )),
            get_encoded_video_session_parameters_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetEncodedVideoSessionParametersKHR\0"),
            )),
            get_event_status: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetEventStatus\0"),
            )),
            #[cfg(feature = "beta")]
            get_execution_graph_pipeline_node_index_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetExecutionGraphPipelineNodeIndexAMDX\0"),
            )),
            #[cfg(feature = "beta")]
            get_execution_graph_pipeline_scratch_size_amdx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetExecutionGraphPipelineScratchSizeAMDX\0"),
            )),
            get_fence_fd_khr: crate::load_pfn(get(device, crate::c_name_ptr(b"vkGetFenceFdKHR\0"))),
            get_fence_status: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetFenceStatus\0"),
            )),
            #[cfg(feature = "win32")]
            get_fence_win32_handle_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetFenceWin32HandleKHR\0"),
            )),
            get_framebuffer_tile_properties_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetFramebufferTilePropertiesQCOM\0"),
            )),
            get_generated_commands_memory_requirements_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetGeneratedCommandsMemoryRequirementsEXT\0"),
            )),
            get_generated_commands_memory_requirements_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetGeneratedCommandsMemoryRequirementsNV\0"),
            )),
            get_image_drm_format_modifier_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageDrmFormatModifierPropertiesEXT\0"),
            )),
            get_image_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageMemoryRequirements\0"),
            )),
            get_image_memory_requirements2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageMemoryRequirements2\0"),
            )),
            get_image_memory_requirements2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageMemoryRequirements2KHR\0"),
            )),
            get_image_opaque_capture_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageOpaqueCaptureDataEXT\0"),
            )),
            get_image_opaque_capture_descriptor_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageOpaqueCaptureDescriptorDataEXT\0"),
            )),
            get_image_sparse_memory_requirements: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSparseMemoryRequirements\0"),
            )),
            get_image_sparse_memory_requirements2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSparseMemoryRequirements2\0"),
            )),
            get_image_sparse_memory_requirements2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSparseMemoryRequirements2KHR\0"),
            )),
            get_image_subresource_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSubresourceLayout\0"),
            )),
            get_image_subresource_layout2: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSubresourceLayout2\0"),
            )),
            get_image_subresource_layout2_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSubresourceLayout2EXT\0"),
            )),
            get_image_subresource_layout2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageSubresourceLayout2KHR\0"),
            )),
            get_image_view_address_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageViewAddressNVX\0"),
            )),
            get_image_view_handle64_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageViewHandle64NVX\0"),
            )),
            get_image_view_handle_nvx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageViewHandleNVX\0"),
            )),
            get_image_view_opaque_capture_descriptor_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetImageViewOpaqueCaptureDescriptorDataEXT\0"),
            )),
            get_latency_timings_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetLatencyTimingsNV\0"),
            )),
            #[cfg(feature = "android")]
            get_memory_android_hardware_buffer_android: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryAndroidHardwareBufferANDROID\0"),
            )),
            get_memory_fd_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryFdKHR\0"),
            )),
            get_memory_fd_properties_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryFdPropertiesKHR\0"),
            )),
            get_memory_host_pointer_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryHostPointerPropertiesEXT\0"),
            )),
            #[cfg(feature = "metal")]
            get_memory_metal_handle_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryMetalHandleEXT\0"),
            )),
            #[cfg(feature = "metal")]
            get_memory_metal_handle_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryMetalHandlePropertiesEXT\0"),
            )),
            #[cfg(feature = "ohos")]
            get_memory_native_buffer_ohos: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryNativeBufferOHOS\0"),
            )),
            get_memory_remote_address_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryRemoteAddressNV\0"),
            )),
            #[cfg(feature = "win32")]
            get_memory_win32_handle_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryWin32HandleKHR\0"),
            )),
            #[cfg(feature = "win32")]
            get_memory_win32_handle_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryWin32HandleNV\0"),
            )),
            #[cfg(feature = "win32")]
            get_memory_win32_handle_properties_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryWin32HandlePropertiesKHR\0"),
            )),
            #[cfg(feature = "fuchsia")]
            get_memory_zircon_handle_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryZirconHandleFUCHSIA\0"),
            )),
            #[cfg(feature = "fuchsia")]
            get_memory_zircon_handle_properties_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMemoryZirconHandlePropertiesFUCHSIA\0"),
            )),
            get_micromap_build_sizes_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetMicromapBuildSizesEXT\0"),
            )),
            #[cfg(feature = "ohos")]
            get_native_buffer_properties_ohos: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetNativeBufferPropertiesOHOS\0"),
            )),
            get_partitioned_acceleration_structures_build_sizes_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPartitionedAccelerationStructuresBuildSizesNV\0"),
            )),
            get_past_presentation_timing_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPastPresentationTimingEXT\0"),
            )),
            get_past_presentation_timing_google: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPastPresentationTimingGOOGLE\0"),
            )),
            get_performance_parameter_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPerformanceParameterINTEL\0"),
            )),
            get_pipeline_binary_data_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineBinaryDataKHR\0"),
            )),
            get_pipeline_cache_data: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineCacheData\0"),
            )),
            get_pipeline_executable_internal_representations_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineExecutableInternalRepresentationsKHR\0"),
            )),
            get_pipeline_executable_properties_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineExecutablePropertiesKHR\0"),
            )),
            get_pipeline_executable_statistics_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineExecutableStatisticsKHR\0"),
            )),
            get_pipeline_indirect_device_address_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineIndirectDeviceAddressNV\0"),
            )),
            get_pipeline_indirect_memory_requirements_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineIndirectMemoryRequirementsNV\0"),
            )),
            get_pipeline_key_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelineKeyKHR\0"),
            )),
            get_pipeline_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPipelinePropertiesEXT\0"),
            )),
            get_private_data: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPrivateData\0"),
            )),
            get_private_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetPrivateDataEXT\0"),
            )),
            get_query_pool_results: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetQueryPoolResults\0"),
            )),
            get_queue_checkpoint_data2_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetQueueCheckpointData2NV\0"),
            )),
            get_queue_checkpoint_data_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetQueueCheckpointDataNV\0"),
            )),
            get_ray_tracing_capture_replay_shader_group_handles_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0"),
            )),
            get_ray_tracing_shader_group_handles_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRayTracingShaderGroupHandlesKHR\0"),
            )),
            get_ray_tracing_shader_group_handles_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRayTracingShaderGroupHandlesNV\0"),
            )),
            get_ray_tracing_shader_group_stack_size_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRayTracingShaderGroupStackSizeKHR\0"),
            )),
            get_refresh_cycle_duration_google: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRefreshCycleDurationGOOGLE\0"),
            )),
            get_render_area_granularity: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRenderAreaGranularity\0"),
            )),
            get_rendering_area_granularity: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRenderingAreaGranularity\0"),
            )),
            get_rendering_area_granularity_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetRenderingAreaGranularityKHR\0"),
            )),
            get_sampler_opaque_capture_descriptor_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSamplerOpaqueCaptureDescriptorDataEXT\0"),
            )),
            #[cfg(feature = "screen")]
            get_screen_buffer_properties_qnx: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetScreenBufferPropertiesQNX\0"),
            )),
            get_semaphore_counter_value: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSemaphoreCounterValue\0"),
            )),
            get_semaphore_counter_value_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSemaphoreCounterValueKHR\0"),
            )),
            get_semaphore_fd_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSemaphoreFdKHR\0"),
            )),
            #[cfg(feature = "win32")]
            get_semaphore_win32_handle_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSemaphoreWin32HandleKHR\0"),
            )),
            #[cfg(feature = "fuchsia")]
            get_semaphore_zircon_handle_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSemaphoreZirconHandleFUCHSIA\0"),
            )),
            get_shader_binary_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetShaderBinaryDataEXT\0"),
            )),
            get_shader_info_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetShaderInfoAMD\0"),
            )),
            get_shader_instrumentation_values_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetShaderInstrumentationValuesARM\0"),
            )),
            get_shader_module_create_info_identifier_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetShaderModuleCreateInfoIdentifierEXT\0"),
            )),
            get_shader_module_identifier_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetShaderModuleIdentifierEXT\0"),
            )),
            get_swapchain_counter_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSwapchainCounterEXT\0"),
            )),
            get_swapchain_images_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSwapchainImagesKHR\0"),
            )),
            get_swapchain_status_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSwapchainStatusKHR\0"),
            )),
            get_swapchain_time_domain_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSwapchainTimeDomainPropertiesEXT\0"),
            )),
            get_swapchain_timing_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetSwapchainTimingPropertiesEXT\0"),
            )),
            get_tensor_memory_requirements_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetTensorMemoryRequirementsARM\0"),
            )),
            get_tensor_opaque_capture_data_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetTensorOpaqueCaptureDataARM\0"),
            )),
            get_tensor_opaque_capture_descriptor_data_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetTensorOpaqueCaptureDescriptorDataARM\0"),
            )),
            get_tensor_view_opaque_capture_descriptor_data_arm: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetTensorViewOpaqueCaptureDescriptorDataARM\0"),
            )),
            get_validation_cache_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetValidationCacheDataEXT\0"),
            )),
            get_video_session_memory_requirements_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkGetVideoSessionMemoryRequirementsKHR\0"),
            )),
            import_fence_fd_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkImportFenceFdKHR\0"),
            )),
            #[cfg(feature = "win32")]
            import_fence_win32_handle_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkImportFenceWin32HandleKHR\0"),
            )),
            import_semaphore_fd_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkImportSemaphoreFdKHR\0"),
            )),
            #[cfg(feature = "win32")]
            import_semaphore_win32_handle_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkImportSemaphoreWin32HandleKHR\0"),
            )),
            #[cfg(feature = "fuchsia")]
            import_semaphore_zircon_handle_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkImportSemaphoreZirconHandleFUCHSIA\0"),
            )),
            initialize_performance_api_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkInitializePerformanceApiINTEL\0"),
            )),
            invalidate_mapped_memory_ranges: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkInvalidateMappedMemoryRanges\0"),
            )),
            latency_sleep_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkLatencySleepNV\0"),
            )),
            map_memory: crate::load_pfn(get(device, crate::c_name_ptr(b"vkMapMemory\0"))),
            map_memory2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkMapMemory2\0"))),
            map_memory2_khr: crate::load_pfn(get(device, crate::c_name_ptr(b"vkMapMemory2KHR\0"))),
            merge_pipeline_caches: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkMergePipelineCaches\0"),
            )),
            merge_validation_caches_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkMergeValidationCachesEXT\0"),
            )),
            queue_begin_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueBeginDebugUtilsLabelEXT\0"),
            )),
            queue_bind_sparse: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueBindSparse\0"),
            )),
            queue_end_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueEndDebugUtilsLabelEXT\0"),
            )),
            queue_insert_debug_utils_label_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueInsertDebugUtilsLabelEXT\0"),
            )),
            queue_notify_out_of_band_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueNotifyOutOfBandNV\0"),
            )),
            queue_present_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueuePresentKHR\0"),
            )),
            queue_set_perf_hint_qcom: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueSetPerfHintQCOM\0"),
            )),
            queue_set_performance_configuration_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueSetPerformanceConfigurationINTEL\0"),
            )),
            queue_submit: crate::load_pfn(get(device, crate::c_name_ptr(b"vkQueueSubmit\0"))),
            queue_submit2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkQueueSubmit2\0"))),
            queue_submit2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkQueueSubmit2KHR\0"),
            )),
            queue_wait_idle: crate::load_pfn(get(device, crate::c_name_ptr(b"vkQueueWaitIdle\0"))),
            register_custom_border_color_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkRegisterCustomBorderColorEXT\0"),
            )),
            register_device_event_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkRegisterDeviceEventEXT\0"),
            )),
            register_display_event_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkRegisterDisplayEventEXT\0"),
            )),
            release_captured_pipeline_data_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleaseCapturedPipelineDataKHR\0"),
            )),
            #[cfg(feature = "win32")]
            release_full_screen_exclusive_mode_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleaseFullScreenExclusiveModeEXT\0"),
            )),
            release_performance_configuration_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleasePerformanceConfigurationINTEL\0"),
            )),
            release_profiling_lock_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleaseProfilingLockKHR\0"),
            )),
            release_swapchain_images_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleaseSwapchainImagesEXT\0"),
            )),
            release_swapchain_images_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkReleaseSwapchainImagesKHR\0"),
            )),
            reset_command_buffer: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkResetCommandBuffer\0"),
            )),
            reset_command_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkResetCommandPool\0"),
            )),
            reset_descriptor_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkResetDescriptorPool\0"),
            )),
            reset_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkResetEvent\0"))),
            reset_fences: crate::load_pfn(get(device, crate::c_name_ptr(b"vkResetFences\0"))),
            reset_query_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkResetQueryPool\0"),
            )),
            reset_query_pool_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkResetQueryPoolEXT\0"),
            )),
            #[cfg(feature = "fuchsia")]
            set_buffer_collection_buffer_constraints_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetBufferCollectionBufferConstraintsFUCHSIA\0"),
            )),
            #[cfg(feature = "fuchsia")]
            set_buffer_collection_image_constraints_fuchsia: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetBufferCollectionImageConstraintsFUCHSIA\0"),
            )),
            set_debug_utils_object_name_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetDebugUtilsObjectNameEXT\0"),
            )),
            set_debug_utils_object_tag_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetDebugUtilsObjectTagEXT\0"),
            )),
            set_device_memory_priority_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetDeviceMemoryPriorityEXT\0"),
            )),
            set_event: crate::load_pfn(get(device, crate::c_name_ptr(b"vkSetEvent\0"))),
            set_hdr_metadata_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetHdrMetadataEXT\0"),
            )),
            set_latency_marker_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetLatencyMarkerNV\0"),
            )),
            set_latency_sleep_mode_nv: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetLatencySleepModeNV\0"),
            )),
            set_local_dimming_amd: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetLocalDimmingAMD\0"),
            )),
            set_private_data: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetPrivateData\0"),
            )),
            set_private_data_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetPrivateDataEXT\0"),
            )),
            set_swapchain_present_timing_queue_size_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSetSwapchainPresentTimingQueueSizeEXT\0"),
            )),
            signal_semaphore: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSignalSemaphore\0"),
            )),
            signal_semaphore_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkSignalSemaphoreKHR\0"),
            )),
            transition_image_layout: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkTransitionImageLayout\0"),
            )),
            transition_image_layout_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkTransitionImageLayoutEXT\0"),
            )),
            trim_command_pool: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkTrimCommandPool\0"),
            )),
            trim_command_pool_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkTrimCommandPoolKHR\0"),
            )),
            uninitialize_performance_api_intel: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUninitializePerformanceApiINTEL\0"),
            )),
            unmap_memory: crate::load_pfn(get(device, crate::c_name_ptr(b"vkUnmapMemory\0"))),
            unmap_memory2: crate::load_pfn(get(device, crate::c_name_ptr(b"vkUnmapMemory2\0"))),
            unmap_memory2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUnmapMemory2KHR\0"),
            )),
            unregister_custom_border_color_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUnregisterCustomBorderColorEXT\0"),
            )),
            update_descriptor_set_with_template: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateDescriptorSetWithTemplate\0"),
            )),
            update_descriptor_set_with_template_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateDescriptorSetWithTemplateKHR\0"),
            )),
            update_descriptor_sets: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateDescriptorSets\0"),
            )),
            update_indirect_execution_set_pipeline_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateIndirectExecutionSetPipelineEXT\0"),
            )),
            update_indirect_execution_set_shader_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateIndirectExecutionSetShaderEXT\0"),
            )),
            update_video_session_parameters_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkUpdateVideoSessionParametersKHR\0"),
            )),
            wait_for_fences: crate::load_pfn(get(device, crate::c_name_ptr(b"vkWaitForFences\0"))),
            wait_for_present2_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWaitForPresent2KHR\0"),
            )),
            wait_for_present_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWaitForPresentKHR\0"),
            )),
            wait_semaphores: crate::load_pfn(get(device, crate::c_name_ptr(b"vkWaitSemaphores\0"))),
            wait_semaphores_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWaitSemaphoresKHR\0"),
            )),
            write_acceleration_structures_properties_khr: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWriteAccelerationStructuresPropertiesKHR\0"),
            )),
            write_micromaps_properties_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWriteMicromapsPropertiesEXT\0"),
            )),
            write_resource_descriptors_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWriteResourceDescriptorsEXT\0"),
            )),
            write_sampler_descriptors_ext: crate::load_pfn(get(
                device,
                crate::c_name_ptr(b"vkWriteSamplerDescriptorsEXT\0"),
            )),
        }
    }
}

impl crate::Entry {
    pub unsafe fn create_instance_raw(
        &self,
        p_create_info: *const InstanceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instance: *mut Instance,
    ) -> VkResult {
        (self
            .fp()
            .create_instance
            .expect("vkCreateInstance is not loaded"))(
            p_create_info, p_allocator, p_instance
        )
    }
    pub unsafe fn enumerate_instance_extension_properties_raw(
        &self,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_instance_extension_properties
            .expect("vkEnumerateInstanceExtensionProperties is not loaded"))(
            p_layer_name,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn enumerate_instance_layer_properties_raw(
        &self,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_instance_layer_properties
            .expect("vkEnumerateInstanceLayerProperties is not loaded"))(
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn enumerate_instance_version_raw(&self, p_api_version: *mut u32) -> VkResult {
        (self
            .fp()
            .enumerate_instance_version
            .expect("vkEnumerateInstanceVersion is not loaded"))(p_api_version)
    }
    pub unsafe fn get_external_compute_queue_data_nv_raw(
        &self,
        external_queue: ExternalComputeQueueNV,
        params: *mut ExternalComputeQueueDataParamsNV<'_>,
        p_data: *mut c_void,
    ) {
        (self
            .fp()
            .get_external_compute_queue_data_nv
            .expect("vkGetExternalComputeQueueDataNV is not loaded"))(
            external_queue,
            params,
            p_data,
        )
    }
    pub unsafe fn get_instance_proc_addr_raw(
        &self,
        instance: Instance,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        (self
            .fp()
            .get_instance_proc_addr
            .expect("vkGetInstanceProcAddr is not loaded"))(instance, p_name)
    }
}

impl crate::Instance {
    pub unsafe fn acquire_drm_display_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .acquire_drm_display_ext
            .expect("vkAcquireDrmDisplayEXT is not loaded"))(
            physical_device, drm_fd, display
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn acquire_winrt_display_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .acquire_winrt_display_nv
            .expect("vkAcquireWinrtDisplayNV is not loaded"))(physical_device, display)
    }
    #[cfg(feature = "xlib-xrandr")]
    pub unsafe fn acquire_xlib_display_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .acquire_xlib_display_ext
            .expect("vkAcquireXlibDisplayEXT is not loaded"))(physical_device, dpy, display)
    }
    #[cfg(feature = "android")]
    pub unsafe fn create_android_surface_khr_raw(
        &self,
        p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_android_surface_khr
            .expect("vkCreateAndroidSurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    pub unsafe fn create_debug_report_callback_ext_raw(
        &self,
        p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_debug_report_callback_ext
            .expect("vkCreateDebugReportCallbackEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_callback,
        )
    }
    pub unsafe fn create_debug_utils_messenger_ext_raw(
        &self,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_debug_utils_messenger_ext
            .expect("vkCreateDebugUtilsMessengerEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_messenger,
        )
    }
    pub unsafe fn create_device_raw(
        &self,
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_device: *mut Device,
    ) -> VkResult {
        (self
            .fp()
            .create_device
            .expect("vkCreateDevice is not loaded"))(
            physical_device,
            p_create_info,
            p_allocator,
            p_device,
        )
    }
    #[cfg(feature = "directfb")]
    pub unsafe fn create_direct_fb_surface_ext_raw(
        &self,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_direct_fb_surface_ext
            .expect("vkCreateDirectFBSurfaceEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    pub unsafe fn create_display_mode_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_mode: *mut DisplayModeKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_display_mode_khr
            .expect("vkCreateDisplayModeKHR is not loaded"))(
            physical_device,
            display,
            p_create_info,
            p_allocator,
            p_mode,
        )
    }
    pub unsafe fn create_display_plane_surface_khr_raw(
        &self,
        p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_display_plane_surface_khr
            .expect("vkCreateDisplayPlaneSurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    pub unsafe fn create_headless_surface_ext_raw(
        &self,
        p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_headless_surface_ext
            .expect("vkCreateHeadlessSurfaceEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "ios")]
    pub unsafe fn create_ios_surface_mvk_raw(
        &self,
        p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_ios_surface_mvk
            .expect("vkCreateIOSSurfaceMVK is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn create_image_pipe_surface_fuchsia_raw(
        &self,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_image_pipe_surface_fuchsia
            .expect("vkCreateImagePipeSurfaceFUCHSIA is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "macos")]
    pub unsafe fn create_mac_os_surface_mvk_raw(
        &self,
        p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_mac_os_surface_mvk
            .expect("vkCreateMacOSSurfaceMVK is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "metal")]
    pub unsafe fn create_metal_surface_ext_raw(
        &self,
        p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_metal_surface_ext
            .expect("vkCreateMetalSurfaceEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "screen")]
    pub unsafe fn create_screen_surface_qnx_raw(
        &self,
        p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_screen_surface_qnx
            .expect("vkCreateScreenSurfaceQNX is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "ggp")]
    pub unsafe fn create_stream_descriptor_surface_ggp_raw(
        &self,
        p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_stream_descriptor_surface_ggp
            .expect("vkCreateStreamDescriptorSurfaceGGP is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "ohos")]
    pub unsafe fn create_surface_ohos_raw(
        &self,
        p_create_info: *const SurfaceCreateInfoOHOS<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_surface_ohos
            .expect("vkCreateSurfaceOHOS is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "ubm")]
    pub unsafe fn create_ubm_surface_sec_raw(
        &self,
        p_create_info: *const UbmSurfaceCreateInfoSEC<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_ubm_surface_sec
            .expect("vkCreateUbmSurfaceSEC is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "vi")]
    pub unsafe fn create_vi_surface_nn_raw(
        &self,
        p_create_info: *const ViSurfaceCreateInfoNN<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_vi_surface_nn
            .expect("vkCreateViSurfaceNN is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "wayland")]
    pub unsafe fn create_wayland_surface_khr_raw(
        &self,
        p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_wayland_surface_khr
            .expect("vkCreateWaylandSurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn create_win32_surface_khr_raw(
        &self,
        p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_win32_surface_khr
            .expect("vkCreateWin32SurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "xcb")]
    pub unsafe fn create_xcb_surface_khr_raw(
        &self,
        p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_xcb_surface_khr
            .expect("vkCreateXcbSurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    #[cfg(feature = "xlib")]
    pub unsafe fn create_xlib_surface_khr_raw(
        &self,
        p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_xlib_surface_khr
            .expect("vkCreateXlibSurfaceKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_surface,
        )
    }
    pub unsafe fn debug_report_message_ext_raw(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ) {
        (self
            .fp()
            .debug_report_message_ext
            .expect("vkDebugReportMessageEXT is not loaded"))(
            self.handle(),
            flags,
            object_type,
            object,
            location,
            message_code,
            p_layer_prefix,
            p_message,
        )
    }
    pub unsafe fn destroy_debug_report_callback_ext_raw(
        &self,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_debug_report_callback_ext
            .expect("vkDestroyDebugReportCallbackEXT is not loaded"))(
            self.handle(),
            callback,
            p_allocator,
        )
    }
    pub unsafe fn destroy_debug_utils_messenger_ext_raw(
        &self,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_debug_utils_messenger_ext
            .expect("vkDestroyDebugUtilsMessengerEXT is not loaded"))(
            self.handle(),
            messenger,
            p_allocator,
        )
    }
    pub unsafe fn destroy_instance_raw(&self, p_allocator: *const AllocationCallbacks<'_>) {
        (self
            .fp()
            .destroy_instance
            .expect("vkDestroyInstance is not loaded"))(self.handle(), p_allocator)
    }
    pub unsafe fn destroy_surface_khr_raw(
        &self,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_surface_khr
            .expect("vkDestroySurfaceKHR is not loaded"))(
            self.handle(), surface, p_allocator
        )
    }
    pub unsafe fn enumerate_device_extension_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_device_extension_properties
            .expect("vkEnumerateDeviceExtensionProperties is not loaded"))(
            physical_device,
            p_layer_name,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn enumerate_device_layer_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_device_layer_properties
            .expect("vkEnumerateDeviceLayerProperties is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn enumerate_physical_device_groups_raw(
        &self,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_device_groups
            .expect("vkEnumeratePhysicalDeviceGroups is not loaded"))(
            self.handle(),
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    pub unsafe fn enumerate_physical_device_groups_khr_raw(
        &self,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_device_groups_khr
            .expect("vkEnumeratePhysicalDeviceGroupsKHR is not loaded"))(
            self.handle(),
            p_physical_device_group_count,
            p_physical_device_group_properties,
        )
    }
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterARM<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_device_queue_family_performance_counters_by_region_arm
            .expect(
                "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM is not loaded",
            ))(
            physical_device,
            queue_family_index,
            p_counter_count,
            p_counters,
            p_counter_descriptions,
        )
    }
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_device_queue_family_performance_query_counters_khr
            .expect(
                "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR is not loaded",
            ))(
            physical_device,
            queue_family_index,
            p_counter_count,
            p_counters,
            p_counter_descriptions,
        )
    }
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        p_description_count: *mut u32,
        p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_device_shader_instrumentation_metrics_arm
            .expect("vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM is not loaded"))(
            physical_device,
            p_description_count,
            p_descriptions,
        )
    }
    pub unsafe fn enumerate_physical_devices_raw(
        &self,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> VkResult {
        (self
            .fp()
            .enumerate_physical_devices
            .expect("vkEnumeratePhysicalDevices is not loaded"))(
            self.handle(),
            p_physical_device_count,
            p_physical_devices,
        )
    }
    pub unsafe fn get_device_proc_addr_raw(
        &self,
        device: Device,
        p_name: *const c_char,
    ) -> PFN_vkVoidFunction {
        (self
            .fp()
            .get_device_proc_addr
            .expect("vkGetDeviceProcAddr is not loaded"))(device, p_name)
    }
    pub unsafe fn get_display_mode_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_display_mode_properties2_khr
            .expect("vkGetDisplayModeProperties2KHR is not loaded"))(
            physical_device,
            display,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_display_mode_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_display_mode_properties_khr
            .expect("vkGetDisplayModePropertiesKHR is not loaded"))(
            physical_device,
            display,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_display_plane_capabilities2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_display_plane_capabilities2_khr
            .expect("vkGetDisplayPlaneCapabilities2KHR is not loaded"))(
            physical_device,
            p_display_plane_info,
            p_capabilities,
        )
    }
    pub unsafe fn get_display_plane_capabilities_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_display_plane_capabilities_khr
            .expect("vkGetDisplayPlaneCapabilitiesKHR is not loaded"))(
            physical_device,
            mode,
            plane_index,
            p_capabilities,
        )
    }
    pub unsafe fn get_display_plane_supported_displays_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_display_plane_supported_displays_khr
            .expect("vkGetDisplayPlaneSupportedDisplaysKHR is not loaded"))(
            physical_device,
            plane_index,
            p_display_count,
            p_displays,
        )
    }
    pub unsafe fn get_drm_display_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_drm_display_ext
            .expect("vkGetDrmDisplayEXT is not loaded"))(
            physical_device,
            drm_fd,
            connector_id,
            display,
        )
    }
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_calibrateable_time_domains_ext
            .expect("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT is not loaded"))(
            physical_device,
            p_time_domain_count,
            p_time_domains,
        )
    }
    pub unsafe fn get_physical_device_calibrateable_time_domains_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_calibrateable_time_domains_khr
            .expect("vkGetPhysicalDeviceCalibrateableTimeDomainsKHR is not loaded"))(
            physical_device,
            p_time_domain_count,
            p_time_domains,
        )
    }
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv
            .expect(
                "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV is not loaded",
            ))(physical_device, p_property_count, p_properties)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_cooperative_matrix_properties_khr
            .expect("vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_cooperative_matrix_properties_nv
            .expect("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeVectorPropertiesNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_cooperative_vector_properties_nv
            .expect("vkGetPhysicalDeviceCooperativeVectorPropertiesNV is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_descriptor_size_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> u64 {
        (self
            .fp()
            .get_physical_device_descriptor_size_ext
            .expect("vkGetPhysicalDeviceDescriptorSizeEXT is not loaded"))(
            physical_device,
            descriptor_type,
        )
    }
    #[cfg(feature = "directfb")]
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_direct_fb_presentation_support_ext
            .expect("vkGetPhysicalDeviceDirectFBPresentationSupportEXT is not loaded"))(
            physical_device,
            queue_family_index,
            dfb,
        )
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_display_plane_properties2_khr
            .expect("vkGetPhysicalDeviceDisplayPlaneProperties2KHR is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_display_plane_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_display_plane_properties_khr
            .expect("vkGetPhysicalDeviceDisplayPlanePropertiesKHR is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_display_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_display_properties2_khr
            .expect("vkGetPhysicalDeviceDisplayProperties2KHR is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_display_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_display_properties_khr
            .expect("vkGetPhysicalDeviceDisplayPropertiesKHR is not loaded"))(
            physical_device,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_external_buffer_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_buffer_properties
            .expect("vkGetPhysicalDeviceExternalBufferProperties is not loaded"))(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    pub unsafe fn get_physical_device_external_buffer_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_buffer_properties_khr
            .expect("vkGetPhysicalDeviceExternalBufferPropertiesKHR is not loaded"))(
            physical_device,
            p_external_buffer_info,
            p_external_buffer_properties,
        )
    }
    pub unsafe fn get_physical_device_external_fence_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_fence_properties
            .expect("vkGetPhysicalDeviceExternalFenceProperties is not loaded"))(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    pub unsafe fn get_physical_device_external_fence_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_fence_properties_khr
            .expect("vkGetPhysicalDeviceExternalFencePropertiesKHR is not loaded"))(
            physical_device,
            p_external_fence_info,
            p_external_fence_properties,
        )
    }
    pub unsafe fn get_physical_device_external_image_format_properties_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_external_image_format_properties_nv
            .expect("vkGetPhysicalDeviceExternalImageFormatPropertiesNV is not loaded"))(
            physical_device,
            format,
            r#type,
            tiling,
            usage,
            flags,
            external_handle_type,
            p_external_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_external_semaphore_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_semaphore_properties
            .expect("vkGetPhysicalDeviceExternalSemaphoreProperties is not loaded"))(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
    pub unsafe fn get_physical_device_external_semaphore_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_semaphore_properties_khr
            .expect("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR is not loaded"))(
            physical_device,
            p_external_semaphore_info,
            p_external_semaphore_properties,
        )
    }
    pub unsafe fn get_physical_device_external_tensor_properties_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
        p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_external_tensor_properties_arm
            .expect("vkGetPhysicalDeviceExternalTensorPropertiesARM is not loaded"))(
            physical_device,
            p_external_tensor_info,
            p_external_tensor_properties,
        )
    }
    pub unsafe fn get_physical_device_features_raw(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_features
            .expect("vkGetPhysicalDeviceFeatures is not loaded"))(
            physical_device, p_features
        )
    }
    pub unsafe fn get_physical_device_features2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_features2
            .expect("vkGetPhysicalDeviceFeatures2 is not loaded"))(
            physical_device, p_features
        )
    }
    pub unsafe fn get_physical_device_features2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_features2_khr
            .expect("vkGetPhysicalDeviceFeatures2KHR is not loaded"))(
            physical_device, p_features
        )
    }
    pub unsafe fn get_physical_device_format_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_format_properties
            .expect("vkGetPhysicalDeviceFormatProperties is not loaded"))(
            physical_device,
            format,
            p_format_properties,
        )
    }
    pub unsafe fn get_physical_device_format_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_format_properties2
            .expect("vkGetPhysicalDeviceFormatProperties2 is not loaded"))(
            physical_device,
            format,
            p_format_properties,
        )
    }
    pub unsafe fn get_physical_device_format_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_format_properties2_khr
            .expect("vkGetPhysicalDeviceFormatProperties2KHR is not loaded"))(
            physical_device,
            format,
            p_format_properties,
        )
    }
    pub unsafe fn get_physical_device_fragment_shading_rates_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_fragment_shading_rate_count: *mut u32,
        p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_fragment_shading_rates_khr
            .expect("vkGetPhysicalDeviceFragmentShadingRatesKHR is not loaded"))(
            physical_device,
            p_fragment_shading_rate_count,
            p_fragment_shading_rates,
        )
    }
    pub unsafe fn get_physical_device_image_format_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_image_format_properties
            .expect("vkGetPhysicalDeviceImageFormatProperties is not loaded"))(
            physical_device,
            format,
            r#type,
            tiling,
            usage,
            flags,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_image_format_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
        p_image_format_properties: *mut ImageFormatProperties2<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_image_format_properties2
            .expect("vkGetPhysicalDeviceImageFormatProperties2 is not loaded"))(
            physical_device,
            p_image_format_info,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_image_format_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
        p_image_format_properties: *mut ImageFormatProperties2<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_image_format_properties2_khr
            .expect("vkGetPhysicalDeviceImageFormatProperties2KHR is not loaded"))(
            physical_device,
            p_image_format_info,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_memory_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_memory_properties
            .expect("vkGetPhysicalDeviceMemoryProperties is not loaded"))(
            physical_device,
            p_memory_properties,
        )
    }
    pub unsafe fn get_physical_device_memory_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_memory_properties2
            .expect("vkGetPhysicalDeviceMemoryProperties2 is not loaded"))(
            physical_device,
            p_memory_properties,
        )
    }
    pub unsafe fn get_physical_device_memory_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_memory_properties2_khr
            .expect("vkGetPhysicalDeviceMemoryProperties2KHR is not loaded"))(
            physical_device,
            p_memory_properties,
        )
    }
    pub unsafe fn get_physical_device_multisample_properties_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_multisample_properties_ext
            .expect("vkGetPhysicalDeviceMultisamplePropertiesEXT is not loaded"))(
            physical_device,
            samples,
            p_multisample_properties,
        )
    }
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
        p_format_count: *mut u32,
        p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_optical_flow_image_formats_nv
            .expect("vkGetPhysicalDeviceOpticalFlowImageFormatsNV is not loaded"))(
            physical_device,
            p_optical_flow_image_format_info,
            p_format_count,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_present_rectangles_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_present_rectangles_khr
            .expect("vkGetPhysicalDevicePresentRectanglesKHR is not loaded"))(
            physical_device,
            surface,
            p_rect_count,
            p_rects,
        )
    }
    pub unsafe fn get_physical_device_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_properties
            .expect("vkGetPhysicalDeviceProperties is not loaded"))(
            physical_device, p_properties
        )
    }
    pub unsafe fn get_physical_device_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_properties2
            .expect("vkGetPhysicalDeviceProperties2 is not loaded"))(
            physical_device, p_properties
        )
    }
    pub unsafe fn get_physical_device_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_properties2_khr
            .expect("vkGetPhysicalDeviceProperties2KHR is not loaded"))(
            physical_device,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
        p_properties: *mut BaseOutStructure<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_queue_family_data_graph_engine_operation_properties_arm
            .expect(
                "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM is not loaded",
            ))(
            physical_device,
            queue_family_index,
            p_queue_family_data_graph_properties,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
        p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM<'_>,
        p_format_count: *mut u32,
        p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm
            .expect(
                "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM is not loaded",
            ))(
            physical_device,
            queue_family_index,
            p_queue_family_data_graph_properties,
            p_optical_flow_image_format_info,
            p_format_count,
            p_image_format_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
        p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
    ) {
        (self.fp().get_physical_device_queue_family_data_graph_processing_engine_properties_arm.expect("vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM is not loaded"))(physical_device, p_queue_family_data_graph_processing_engine_info, p_queue_family_data_graph_processing_engine_properties)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_property_count: *mut u32,
        p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_queue_family_data_graph_properties_arm
            .expect("vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM is not loaded"))(
            physical_device,
            queue_family_index,
            p_queue_family_data_graph_property_count,
            p_queue_family_data_graph_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
        p_num_passes: *mut u32,
    ) {
        (self
            .fp()
            .get_physical_device_queue_family_performance_query_passes_khr
            .expect("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR is not loaded"))(
            physical_device,
            p_performance_query_create_info,
            p_num_passes,
        )
    }
    pub unsafe fn get_physical_device_queue_family_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_queue_family_properties
            .expect("vkGetPhysicalDeviceQueueFamilyProperties is not loaded"))(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_queue_family_properties2
            .expect("vkGetPhysicalDeviceQueueFamilyProperties2 is not loaded"))(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_queue_family_properties2_khr
            .expect("vkGetPhysicalDeviceQueueFamilyProperties2KHR is not loaded"))(
            physical_device,
            p_queue_family_property_count,
            p_queue_family_properties,
        )
    }
    #[cfg(feature = "screen")]
    pub unsafe fn get_physical_device_screen_presentation_support_qnx_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_screen_presentation_support_qnx
            .expect("vkGetPhysicalDeviceScreenPresentationSupportQNX is not loaded"))(
            physical_device,
            queue_family_index,
            window,
        )
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_sparse_image_format_properties
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties is not loaded"))(
            physical_device,
            format,
            r#type,
            samples,
            usage,
            tiling,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_raw(
        &self,
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_sparse_image_format_properties2
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties2 is not loaded"))(
            physical_device,
            p_format_info,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    ) {
        (self
            .fp()
            .get_physical_device_sparse_image_format_properties2_khr
            .expect("vkGetPhysicalDeviceSparseImageFormatProperties2KHR is not loaded"))(
            physical_device,
            p_format_info,
            p_property_count,
            p_properties,
        )
    }
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv
            .expect(
                "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV is not loaded",
            ))(physical_device, p_combination_count, p_combinations)
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_capabilities2_ext
            .expect("vkGetPhysicalDeviceSurfaceCapabilities2EXT is not loaded"))(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
    pub unsafe fn get_physical_device_surface_capabilities2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_capabilities2_khr
            .expect("vkGetPhysicalDeviceSurfaceCapabilities2KHR is not loaded"))(
            physical_device,
            p_surface_info,
            p_surface_capabilities,
        )
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_capabilities_khr
            .expect("vkGetPhysicalDeviceSurfaceCapabilitiesKHR is not loaded"))(
            physical_device,
            surface,
            p_surface_capabilities,
        )
    }
    pub unsafe fn get_physical_device_surface_formats2_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_formats2_khr
            .expect("vkGetPhysicalDeviceSurfaceFormats2KHR is not loaded"))(
            physical_device,
            p_surface_info,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    pub unsafe fn get_physical_device_surface_formats_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_formats_khr
            .expect("vkGetPhysicalDeviceSurfaceFormatsKHR is not loaded"))(
            physical_device,
            surface,
            p_surface_format_count,
            p_surface_formats,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_physical_device_surface_present_modes2_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_present_modes2_ext
            .expect("vkGetPhysicalDeviceSurfacePresentModes2EXT is not loaded"))(
            physical_device,
            p_surface_info,
            p_present_mode_count,
            p_present_modes,
        )
    }
    pub unsafe fn get_physical_device_surface_present_modes_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_present_modes_khr
            .expect("vkGetPhysicalDeviceSurfacePresentModesKHR is not loaded"))(
            physical_device,
            surface,
            p_present_mode_count,
            p_present_modes,
        )
    }
    pub unsafe fn get_physical_device_surface_support_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_surface_support_khr
            .expect("vkGetPhysicalDeviceSurfaceSupportKHR is not loaded"))(
            physical_device,
            queue_family_index,
            surface,
            p_supported,
        )
    }
    pub unsafe fn get_physical_device_tool_properties_raw(
        &self,
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_tool_properties
            .expect("vkGetPhysicalDeviceToolProperties is not loaded"))(
            physical_device,
            p_tool_count,
            p_tool_properties,
        )
    }
    pub unsafe fn get_physical_device_tool_properties_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_tool_properties_ext
            .expect("vkGetPhysicalDeviceToolPropertiesEXT is not loaded"))(
            physical_device,
            p_tool_count,
            p_tool_properties,
        )
    }
    #[cfg(feature = "ubm")]
    pub unsafe fn get_physical_device_ubm_presentation_support_sec_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_ubm_presentation_support_sec
            .expect("vkGetPhysicalDeviceUbmPresentationSupportSEC is not loaded"))(
            physical_device,
            queue_family_index,
            device,
        )
    }
    pub unsafe fn get_physical_device_video_capabilities_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_video_profile: *const VideoProfileInfoKHR<'_>,
        p_capabilities: *mut VideoCapabilitiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_video_capabilities_khr
            .expect("vkGetPhysicalDeviceVideoCapabilitiesKHR is not loaded"))(
            physical_device,
            p_video_profile,
            p_capabilities,
        )
    }
    pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_video_encode_quality_level_properties_khr
            .expect("vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR is not loaded"))(
            physical_device,
            p_quality_level_info,
            p_quality_level_properties,
        )
    }
    pub unsafe fn get_physical_device_video_format_properties_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_physical_device_video_format_properties_khr
            .expect("vkGetPhysicalDeviceVideoFormatPropertiesKHR is not loaded"))(
            physical_device,
            p_video_format_info,
            p_video_format_property_count,
            p_video_format_properties,
        )
    }
    #[cfg(feature = "wayland")]
    pub unsafe fn get_physical_device_wayland_presentation_support_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_wayland_presentation_support_khr
            .expect("vkGetPhysicalDeviceWaylandPresentationSupportKHR is not loaded"))(
            physical_device,
            queue_family_index,
            display,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_physical_device_win32_presentation_support_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_win32_presentation_support_khr
            .expect("vkGetPhysicalDeviceWin32PresentationSupportKHR is not loaded"))(
            physical_device,
            queue_family_index,
        )
    }
    #[cfg(feature = "xcb")]
    pub unsafe fn get_physical_device_xcb_presentation_support_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_xcb_presentation_support_khr
            .expect("vkGetPhysicalDeviceXcbPresentationSupportKHR is not loaded"))(
            physical_device,
            queue_family_index,
            connection,
            visual_id,
        )
    }
    #[cfg(feature = "xlib")]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr_raw(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        (self
            .fp()
            .get_physical_device_xlib_presentation_support_khr
            .expect("vkGetPhysicalDeviceXlibPresentationSupportKHR is not loaded"))(
            physical_device,
            queue_family_index,
            dpy,
            visual_id,
        )
    }
    #[cfg(feature = "xlib-xrandr")]
    pub unsafe fn get_rand_r_output_display_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_rand_r_output_display_ext
            .expect("vkGetRandROutputDisplayEXT is not loaded"))(
            physical_device,
            dpy,
            rr_output,
            p_display,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_winrt_display_nv_raw(
        &self,
        physical_device: PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_winrt_display_nv
            .expect("vkGetWinrtDisplayNV is not loaded"))(
            physical_device,
            device_relative_id,
            p_display,
        )
    }
    pub unsafe fn release_display_ext_raw(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> VkResult {
        (self
            .fp()
            .release_display_ext
            .expect("vkReleaseDisplayEXT is not loaded"))(physical_device, display)
    }
    pub unsafe fn submit_debug_utils_message_ext_raw(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    ) {
        (self
            .fp()
            .submit_debug_utils_message_ext
            .expect("vkSubmitDebugUtilsMessageEXT is not loaded"))(
            self.handle(),
            message_severity,
            message_types,
            p_callback_data,
        )
    }
}

impl crate::Device {
    #[cfg(feature = "win32")]
    pub unsafe fn acquire_full_screen_exclusive_mode_ext_raw(
        &self,
        swapchain: SwapchainKHR,
    ) -> VkResult {
        (self
            .fp()
            .acquire_full_screen_exclusive_mode_ext
            .expect("vkAcquireFullScreenExclusiveModeEXT is not loaded"))(
            self.handle(), swapchain
        )
    }
    pub unsafe fn acquire_next_image2_khr_raw(
        &self,
        p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
        p_image_index: *mut u32,
    ) -> VkResult {
        (self
            .fp()
            .acquire_next_image2_khr
            .expect("vkAcquireNextImage2KHR is not loaded"))(
            self.handle(),
            p_acquire_info,
            p_image_index,
        )
    }
    pub unsafe fn acquire_next_image_khr_raw(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> VkResult {
        (self
            .fp()
            .acquire_next_image_khr
            .expect("vkAcquireNextImageKHR is not loaded"))(
            self.handle(),
            swapchain,
            timeout,
            semaphore,
            fence,
            p_image_index,
        )
    }
    pub unsafe fn acquire_performance_configuration_intel_raw(
        &self,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
        p_configuration: *mut PerformanceConfigurationINTEL,
    ) -> VkResult {
        (self
            .fp()
            .acquire_performance_configuration_intel
            .expect("vkAcquirePerformanceConfigurationINTEL is not loaded"))(
            self.handle(),
            p_acquire_info,
            p_configuration,
        )
    }
    pub unsafe fn acquire_profiling_lock_khr_raw(
        &self,
        p_info: *const AcquireProfilingLockInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .acquire_profiling_lock_khr
            .expect("vkAcquireProfilingLockKHR is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn allocate_command_buffers_raw(
        &self,
        p_allocate_info: *const CommandBufferAllocateInfo<'_>,
        p_command_buffers: *mut CommandBuffer,
    ) -> VkResult {
        (self
            .fp()
            .allocate_command_buffers
            .expect("vkAllocateCommandBuffers is not loaded"))(
            self.handle(),
            p_allocate_info,
            p_command_buffers,
        )
    }
    pub unsafe fn allocate_descriptor_sets_raw(
        &self,
        p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> VkResult {
        (self
            .fp()
            .allocate_descriptor_sets
            .expect("vkAllocateDescriptorSets is not loaded"))(
            self.handle(),
            p_allocate_info,
            p_descriptor_sets,
        )
    }
    pub unsafe fn allocate_memory_raw(
        &self,
        p_allocate_info: *const MemoryAllocateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_memory: *mut DeviceMemory,
    ) -> VkResult {
        (self
            .fp()
            .allocate_memory
            .expect("vkAllocateMemory is not loaded"))(
            self.handle(),
            p_allocate_info,
            p_allocator,
            p_memory,
        )
    }
    pub unsafe fn anti_lag_update_amd_raw(&self, p_data: *const AntiLagDataAMD<'_>) {
        (self
            .fp()
            .anti_lag_update_amd
            .expect("vkAntiLagUpdateAMD is not loaded"))(self.handle(), p_data)
    }
    pub unsafe fn begin_command_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .begin_command_buffer
            .expect("vkBeginCommandBuffer is not loaded"))(command_buffer, p_begin_info)
    }
    pub unsafe fn bind_acceleration_structure_memory_nv_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_acceleration_structure_memory_nv
            .expect("vkBindAccelerationStructureMemoryNV is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_buffer_memory_raw(
        &self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> VkResult {
        (self
            .fp()
            .bind_buffer_memory
            .expect("vkBindBufferMemory is not loaded"))(
            self.handle(),
            buffer,
            memory,
            memory_offset,
        )
    }
    pub unsafe fn bind_buffer_memory2_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_buffer_memory2
            .expect("vkBindBufferMemory2 is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_buffer_memory2_khr_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_buffer_memory2_khr
            .expect("vkBindBufferMemory2KHR is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_data_graph_pipeline_session_memory_arm_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_data_graph_pipeline_session_memory_arm
            .expect("vkBindDataGraphPipelineSessionMemoryARM is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_image_memory_raw(
        &self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> VkResult {
        (self
            .fp()
            .bind_image_memory
            .expect("vkBindImageMemory is not loaded"))(
            self.handle(), image, memory, memory_offset
        )
    }
    pub unsafe fn bind_image_memory2_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_image_memory2
            .expect("vkBindImageMemory2 is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_image_memory2_khr_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_image_memory2_khr
            .expect("vkBindImageMemory2KHR is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_optical_flow_session_image_nv_raw(
        &self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> VkResult {
        (self
            .fp()
            .bind_optical_flow_session_image_nv
            .expect("vkBindOpticalFlowSessionImageNV is not loaded"))(
            self.handle(),
            session,
            binding_point,
            view,
            layout,
        )
    }
    pub unsafe fn bind_tensor_memory_arm_raw(
        &self,
        bind_info_count: u32,
        p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_tensor_memory_arm
            .expect("vkBindTensorMemoryARM is not loaded"))(
            self.handle(),
            bind_info_count,
            p_bind_infos,
        )
    }
    pub unsafe fn bind_video_session_memory_khr_raw(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .bind_video_session_memory_khr
            .expect("vkBindVideoSessionMemoryKHR is not loaded"))(
            self.handle(),
            video_session,
            bind_session_memory_info_count,
            p_bind_session_memory_infos,
        )
    }
    pub unsafe fn build_acceleration_structures_khr_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .build_acceleration_structures_khr
            .expect("vkBuildAccelerationStructuresKHR is not loaded"))(
            self.handle(),
            deferred_operation,
            info_count,
            p_infos,
            pp_build_range_infos,
        )
    }
    pub unsafe fn build_micromaps_ext_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .build_micromaps_ext
            .expect("vkBuildMicromapsEXT is not loaded"))(
            self.handle(),
            deferred_operation,
            info_count,
            p_infos,
        )
    }
    pub unsafe fn clear_shader_instrumentation_metrics_arm_raw(
        &self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        (self
            .fp()
            .clear_shader_instrumentation_metrics_arm
            .expect("vkClearShaderInstrumentationMetricsARM is not loaded"))(
            self.handle(),
            instrumentation,
        )
    }
    pub unsafe fn cmd_begin_conditional_rendering2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_conditional_rendering2_ext
            .expect("vkCmdBeginConditionalRendering2EXT is not loaded"))(
            command_buffer,
            p_conditional_rendering_begin,
        )
    }
    pub unsafe fn cmd_begin_conditional_rendering_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_conditional_rendering_ext
            .expect("vkCmdBeginConditionalRenderingEXT is not loaded"))(
            command_buffer,
            p_conditional_rendering_begin,
        )
    }
    pub unsafe fn cmd_begin_custom_resolve_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_custom_resolve_ext
            .expect("vkCmdBeginCustomResolveEXT is not loaded"))(
            command_buffer,
            p_begin_custom_resolve_info,
        )
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_debug_utils_label_ext
            .expect("vkCmdBeginDebugUtilsLabelEXT is not loaded"))(
            command_buffer, p_label_info
        )
    }
    pub unsafe fn cmd_begin_per_tile_execution_qcom_raw(
        &self,
        command_buffer: CommandBuffer,
        p_per_tile_begin_info: *const PerTileBeginInfoQCOM<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_per_tile_execution_qcom
            .expect("vkCmdBeginPerTileExecutionQCOM is not loaded"))(
            command_buffer,
            p_per_tile_begin_info,
        )
    }
    pub unsafe fn cmd_begin_query_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        (self
            .fp()
            .cmd_begin_query
            .expect("vkCmdBeginQuery is not loaded"))(
            command_buffer, query_pool, query, flags
        )
    }
    pub unsafe fn cmd_begin_query_indexed_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        (self
            .fp()
            .cmd_begin_query_indexed_ext
            .expect("vkCmdBeginQueryIndexedEXT is not loaded"))(
            command_buffer,
            query_pool,
            query,
            flags,
            index,
        )
    }
    pub unsafe fn cmd_begin_render_pass_raw(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    ) {
        (self
            .fp()
            .cmd_begin_render_pass
            .expect("vkCmdBeginRenderPass is not loaded"))(
            command_buffer,
            p_render_pass_begin,
            contents,
        )
    }
    pub unsafe fn cmd_begin_render_pass2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_render_pass2
            .expect("vkCmdBeginRenderPass2 is not loaded"))(
            command_buffer,
            p_render_pass_begin,
            p_subpass_begin_info,
        )
    }
    pub unsafe fn cmd_begin_render_pass2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_render_pass2_khr
            .expect("vkCmdBeginRenderPass2KHR is not loaded"))(
            command_buffer,
            p_render_pass_begin,
            p_subpass_begin_info,
        )
    }
    pub unsafe fn cmd_begin_rendering_raw(
        &self,
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_rendering
            .expect("vkCmdBeginRendering is not loaded"))(command_buffer, p_rendering_info)
    }
    pub unsafe fn cmd_begin_rendering_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_rendering_khr
            .expect("vkCmdBeginRenderingKHR is not loaded"))(
            command_buffer, p_rendering_info
        )
    }
    pub unsafe fn cmd_begin_shader_instrumentation_arm_raw(
        &self,
        command_buffer: CommandBuffer,
        instrumentation: ShaderInstrumentationARM,
    ) {
        (self
            .fp()
            .cmd_begin_shader_instrumentation_arm
            .expect("vkCmdBeginShaderInstrumentationARM is not loaded"))(
            command_buffer,
            instrumentation,
        )
    }
    pub unsafe fn cmd_begin_transform_feedback2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_transform_feedback2_ext
            .expect("vkCmdBeginTransformFeedback2EXT is not loaded"))(
            command_buffer,
            first_counter_range,
            counter_range_count,
            p_counter_infos,
        )
    }
    pub unsafe fn cmd_begin_transform_feedback_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const u64,
    ) {
        (self
            .fp()
            .cmd_begin_transform_feedback_ext
            .expect("vkCmdBeginTransformFeedbackEXT is not loaded"))(
            command_buffer,
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers,
            p_counter_buffer_offsets,
        )
    }
    pub unsafe fn cmd_begin_video_coding_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_begin_video_coding_khr
            .expect("vkCmdBeginVideoCodingKHR is not loaded"))(command_buffer, p_begin_info)
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_buffer_embedded_samplers2_ext
            .expect("vkCmdBindDescriptorBufferEmbeddedSamplers2EXT is not loaded"))(
            command_buffer,
            p_bind_descriptor_buffer_embedded_samplers_info,
        )
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_buffer_embedded_samplers_ext
            .expect("vkCmdBindDescriptorBufferEmbeddedSamplersEXT is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
        )
    }
    pub unsafe fn cmd_bind_descriptor_buffers_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer_count: u32,
        p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_buffers_ext
            .expect("vkCmdBindDescriptorBuffersEXT is not loaded"))(
            command_buffer,
            buffer_count,
            p_binding_infos,
        )
    }
    pub unsafe fn cmd_bind_descriptor_sets_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_sets
            .expect("vkCmdBindDescriptorSets is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_set_count,
            p_descriptor_sets,
            dynamic_offset_count,
            p_dynamic_offsets,
        )
    }
    pub unsafe fn cmd_bind_descriptor_sets2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_sets2
            .expect("vkCmdBindDescriptorSets2 is not loaded"))(
            command_buffer,
            p_bind_descriptor_sets_info,
        )
    }
    pub unsafe fn cmd_bind_descriptor_sets2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_descriptor_sets2_khr
            .expect("vkCmdBindDescriptorSets2KHR is not loaded"))(
            command_buffer,
            p_bind_descriptor_sets_info,
        )
    }
    pub unsafe fn cmd_bind_index_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        index_type: IndexType,
    ) {
        (self
            .fp()
            .cmd_bind_index_buffer
            .expect("vkCmdBindIndexBuffer is not loaded"))(
            command_buffer,
            buffer,
            offset,
            index_type,
        )
    }
    pub unsafe fn cmd_bind_index_buffer2_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        size: u64,
        index_type: IndexType,
    ) {
        (self
            .fp()
            .cmd_bind_index_buffer2
            .expect("vkCmdBindIndexBuffer2 is not loaded"))(
            command_buffer,
            buffer,
            offset,
            size,
            index_type,
        )
    }
    pub unsafe fn cmd_bind_index_buffer2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        size: u64,
        index_type: IndexType,
    ) {
        (self
            .fp()
            .cmd_bind_index_buffer2_khr
            .expect("vkCmdBindIndexBuffer2KHR is not loaded"))(
            command_buffer,
            buffer,
            offset,
            size,
            index_type,
        )
    }
    pub unsafe fn cmd_bind_index_buffer3_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const BindIndexBuffer3InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_index_buffer3_khr
            .expect("vkCmdBindIndexBuffer3KHR is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_bind_invocation_mask_huawei_raw(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        (self
            .fp()
            .cmd_bind_invocation_mask_huawei
            .expect("vkCmdBindInvocationMaskHUAWEI is not loaded"))(
            command_buffer,
            image_view,
            image_layout,
        )
    }
    pub unsafe fn cmd_bind_pipeline_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        (self
            .fp()
            .cmd_bind_pipeline
            .expect("vkCmdBindPipeline is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        )
    }
    pub unsafe fn cmd_bind_pipeline_shader_group_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        (self
            .fp()
            .cmd_bind_pipeline_shader_group_nv
            .expect("vkCmdBindPipelineShaderGroupNV is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            pipeline,
            group_index,
        )
    }
    pub unsafe fn cmd_bind_resource_heap_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_resource_heap_ext
            .expect("vkCmdBindResourceHeapEXT is not loaded"))(command_buffer, p_bind_info)
    }
    pub unsafe fn cmd_bind_sampler_heap_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_sampler_heap_ext
            .expect("vkCmdBindSamplerHeapEXT is not loaded"))(command_buffer, p_bind_info)
    }
    pub unsafe fn cmd_bind_shaders_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        stage_count: u32,
        p_stages: *const ShaderStageFlagBits,
        p_shaders: *const ShaderEXT,
    ) {
        (self
            .fp()
            .cmd_bind_shaders_ext
            .expect("vkCmdBindShadersEXT is not loaded"))(
            command_buffer,
            stage_count,
            p_stages,
            p_shaders,
        )
    }
    pub unsafe fn cmd_bind_shading_rate_image_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        (self
            .fp()
            .cmd_bind_shading_rate_image_nv
            .expect("vkCmdBindShadingRateImageNV is not loaded"))(
            command_buffer,
            image_view,
            image_layout,
        )
    }
    pub unsafe fn cmd_bind_tile_memory_qcom_raw(
        &self,
        command_buffer: CommandBuffer,
        p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_tile_memory_qcom
            .expect("vkCmdBindTileMemoryQCOM is not loaded"))(
            command_buffer,
            p_tile_memory_bind_info,
        )
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_transform_feedback_buffers2_ext
            .expect("vkCmdBindTransformFeedbackBuffers2EXT is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_binding_infos,
        )
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const u64,
        p_sizes: *const u64,
    ) {
        (self
            .fp()
            .cmd_bind_transform_feedback_buffers_ext
            .expect("vkCmdBindTransformFeedbackBuffersEXT is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const u64,
    ) {
        (self
            .fp()
            .cmd_bind_vertex_buffers
            .expect("vkCmdBindVertexBuffers is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers2_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const u64,
        p_sizes: *const u64,
        p_strides: *const u64,
    ) {
        (self
            .fp()
            .cmd_bind_vertex_buffers2
            .expect("vkCmdBindVertexBuffers2 is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
            p_strides,
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const u64,
        p_sizes: *const u64,
        p_strides: *const u64,
    ) {
        (self
            .fp()
            .cmd_bind_vertex_buffers2_ext
            .expect("vkCmdBindVertexBuffers2EXT is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_buffers,
            p_offsets,
            p_sizes,
            p_strides,
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers3_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindVertexBuffer3InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_bind_vertex_buffers3_khr
            .expect("vkCmdBindVertexBuffers3KHR is not loaded"))(
            command_buffer,
            first_binding,
            binding_count,
            p_binding_infos,
        )
    }
    pub unsafe fn cmd_blit_image_raw(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit<'_>,
        filter: Filter,
    ) {
        (self
            .fp()
            .cmd_blit_image
            .expect("vkCmdBlitImage is not loaded"))(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
            filter,
        )
    }
    pub unsafe fn cmd_blit_image2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_blit_image2
            .expect("vkCmdBlitImage2 is not loaded"))(command_buffer, p_blit_image_info)
    }
    pub unsafe fn cmd_blit_image2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_blit_image2_khr
            .expect("vkCmdBlitImage2KHR is not loaded"))(command_buffer, p_blit_image_info)
    }
    pub unsafe fn cmd_build_acceleration_structure_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureInfoNV<'_>,
        instance_data: Buffer,
        instance_offset: u64,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: u64,
    ) {
        (self
            .fp()
            .cmd_build_acceleration_structure_nv
            .expect("vkCmdBuildAccelerationStructureNV is not loaded"))(
            command_buffer,
            p_info,
            instance_data,
            instance_offset,
            update,
            dst,
            src,
            scratch,
            scratch_offset,
        )
    }
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_indirect_device_addresses: *const u64,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    ) {
        (self
            .fp()
            .cmd_build_acceleration_structures_indirect_khr
            .expect("vkCmdBuildAccelerationStructuresIndirectKHR is not loaded"))(
            command_buffer,
            info_count,
            p_infos,
            p_indirect_device_addresses,
            p_indirect_strides,
            pp_max_primitive_counts,
        )
    }
    pub unsafe fn cmd_build_acceleration_structures_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_build_acceleration_structures_khr
            .expect("vkCmdBuildAccelerationStructuresKHR is not loaded"))(
            command_buffer,
            info_count,
            p_infos,
            pp_build_range_infos,
        )
    }
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_build_cluster_acceleration_structure_indirect_nv
            .expect("vkCmdBuildClusterAccelerationStructureIndirectNV is not loaded"))(
            command_buffer,
            p_command_infos,
        )
    }
    pub unsafe fn cmd_build_micromaps_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_build_micromaps_ext
            .expect("vkCmdBuildMicromapsEXT is not loaded"))(
            command_buffer, info_count, p_infos
        )
    }
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_build_info: *const BuildPartitionedAccelerationStructureInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_build_partitioned_acceleration_structures_nv
            .expect("vkCmdBuildPartitionedAccelerationStructuresNV is not loaded"))(
            command_buffer,
            p_build_info,
        )
    }
    pub unsafe fn cmd_clear_attachments_raw(
        &self,
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment<'_>,
        rect_count: u32,
        p_rects: *const ClearRect<'_>,
    ) {
        (self
            .fp()
            .cmd_clear_attachments
            .expect("vkCmdClearAttachments is not loaded"))(
            command_buffer,
            attachment_count,
            p_attachments,
            rect_count,
            p_rects,
        )
    }
    pub unsafe fn cmd_clear_color_image_raw(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue<'_>,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange<'_>,
    ) {
        (self
            .fp()
            .cmd_clear_color_image
            .expect("vkCmdClearColorImage is not loaded"))(
            command_buffer,
            image,
            image_layout,
            p_color,
            range_count,
            p_ranges,
        )
    }
    pub unsafe fn cmd_clear_depth_stencil_image_raw(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue<'_>,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange<'_>,
    ) {
        (self
            .fp()
            .cmd_clear_depth_stencil_image
            .expect("vkCmdClearDepthStencilImage is not loaded"))(
            command_buffer,
            image,
            image_layout,
            p_depth_stencil,
            range_count,
            p_ranges,
        )
    }
    pub unsafe fn cmd_control_video_coding_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_control_video_coding_khr
            .expect("vkCmdControlVideoCodingKHR is not loaded"))(
            command_buffer,
            p_coding_control_info,
        )
    }
    pub unsafe fn cmd_convert_cooperative_vector_matrix_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_convert_cooperative_vector_matrix_nv
            .expect("vkCmdConvertCooperativeVectorMatrixNV is not loaded"))(
            command_buffer,
            info_count,
            p_infos,
        )
    }
    pub unsafe fn cmd_copy_acceleration_structure_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_acceleration_structure_khr
            .expect("vkCmdCopyAccelerationStructureKHR is not loaded"))(
            command_buffer, p_info
        )
    }
    pub unsafe fn cmd_copy_acceleration_structure_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        (self
            .fp()
            .cmd_copy_acceleration_structure_nv
            .expect("vkCmdCopyAccelerationStructureNV is not loaded"))(
            command_buffer,
            dst,
            src,
            mode,
        )
    }
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_acceleration_structure_to_memory_khr
            .expect("vkCmdCopyAccelerationStructureToMemoryKHR is not loaded"))(
            command_buffer,
            p_info,
        )
    }
    pub unsafe fn cmd_copy_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer
            .expect("vkCmdCopyBuffer is not loaded"))(
            command_buffer,
            src_buffer,
            dst_buffer,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_buffer2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer2
            .expect("vkCmdCopyBuffer2 is not loaded"))(command_buffer, p_copy_buffer_info)
    }
    pub unsafe fn cmd_copy_buffer2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer2_khr
            .expect("vkCmdCopyBuffer2KHR is not loaded"))(command_buffer, p_copy_buffer_info)
    }
    pub unsafe fn cmd_copy_buffer_to_image_raw(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer_to_image
            .expect("vkCmdCopyBufferToImage is not loaded"))(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_buffer_to_image2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer_to_image2
            .expect("vkCmdCopyBufferToImage2 is not loaded"))(
            command_buffer,
            p_copy_buffer_to_image_info,
        )
    }
    pub unsafe fn cmd_copy_buffer_to_image2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_buffer_to_image2_khr
            .expect("vkCmdCopyBufferToImage2KHR is not loaded"))(
            command_buffer,
            p_copy_buffer_to_image_info,
        )
    }
    pub unsafe fn cmd_copy_image_raw(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image
            .expect("vkCmdCopyImage is not loaded"))(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_image2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image2
            .expect("vkCmdCopyImage2 is not loaded"))(command_buffer, p_copy_image_info)
    }
    pub unsafe fn cmd_copy_image2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image2_khr
            .expect("vkCmdCopyImage2KHR is not loaded"))(command_buffer, p_copy_image_info)
    }
    pub unsafe fn cmd_copy_image_to_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image_to_buffer
            .expect("vkCmdCopyImageToBuffer is not loaded"))(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_copy_image_to_buffer2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image_to_buffer2
            .expect("vkCmdCopyImageToBuffer2 is not loaded"))(
            command_buffer,
            p_copy_image_to_buffer_info,
        )
    }
    pub unsafe fn cmd_copy_image_to_buffer2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image_to_buffer2_khr
            .expect("vkCmdCopyImageToBuffer2KHR is not loaded"))(
            command_buffer,
            p_copy_image_to_buffer_info,
        )
    }
    pub unsafe fn cmd_copy_image_to_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_image_to_memory_khr
            .expect("vkCmdCopyImageToMemoryKHR is not loaded"))(
            command_buffer, p_copy_memory_info
        )
    }
    pub unsafe fn cmd_copy_memory_indirect_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_indirect_khr
            .expect("vkCmdCopyMemoryIndirectKHR is not loaded"))(
            command_buffer,
            p_copy_memory_indirect_info,
        )
    }
    pub unsafe fn cmd_copy_memory_indirect_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: u64,
        copy_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_copy_memory_indirect_nv
            .expect("vkCmdCopyMemoryIndirectNV is not loaded"))(
            command_buffer,
            copy_buffer_address,
            copy_count,
            stride,
        )
    }
    pub unsafe fn cmd_copy_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_khr
            .expect("vkCmdCopyMemoryKHR is not loaded"))(command_buffer, p_copy_memory_info)
    }
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_to_acceleration_structure_khr
            .expect("vkCmdCopyMemoryToAccelerationStructureKHR is not loaded"))(
            command_buffer,
            p_info,
        )
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_to_image_indirect_khr
            .expect("vkCmdCopyMemoryToImageIndirectKHR is not loaded"))(
            command_buffer,
            p_copy_memory_to_image_indirect_info,
        )
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: u64,
        copy_count: u32,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        p_image_subresources: *const ImageSubresourceLayers<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_to_image_indirect_nv
            .expect("vkCmdCopyMemoryToImageIndirectNV is not loaded"))(
            command_buffer,
            copy_buffer_address,
            copy_count,
            stride,
            dst_image,
            dst_image_layout,
            p_image_subresources,
        )
    }
    pub unsafe fn cmd_copy_memory_to_image_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_to_image_khr
            .expect("vkCmdCopyMemoryToImageKHR is not loaded"))(
            command_buffer, p_copy_memory_info
        )
    }
    pub unsafe fn cmd_copy_memory_to_micromap_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_memory_to_micromap_ext
            .expect("vkCmdCopyMemoryToMicromapEXT is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_copy_micromap_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_micromap_ext
            .expect("vkCmdCopyMicromapEXT is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_copy_micromap_to_memory_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_micromap_to_memory_ext
            .expect("vkCmdCopyMicromapToMemoryEXT is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_copy_query_pool_results_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: u64,
        stride: u64,
        flags: QueryResultFlags,
    ) {
        (self
            .fp()
            .cmd_copy_query_pool_results
            .expect("vkCmdCopyQueryPoolResults is not loaded"))(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        )
    }
    pub unsafe fn cmd_copy_query_pool_results_to_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        p_dst_range: *const StridedDeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        (self
            .fp()
            .cmd_copy_query_pool_results_to_memory_khr
            .expect("vkCmdCopyQueryPoolResultsToMemoryKHR is not loaded"))(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            p_dst_range,
            dst_flags,
            query_result_flags,
        )
    }
    pub unsafe fn cmd_copy_tensor_arm_raw(
        &self,
        command_buffer: CommandBuffer,
        p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
    ) {
        (self
            .fp()
            .cmd_copy_tensor_arm
            .expect("vkCmdCopyTensorARM is not loaded"))(command_buffer, p_copy_tensor_info)
    }
    pub unsafe fn cmd_cu_launch_kernel_nvx_raw(
        &self,
        command_buffer: CommandBuffer,
        p_launch_info: *const CuLaunchInfoNVX<'_>,
    ) {
        (self
            .fp()
            .cmd_cu_launch_kernel_nvx
            .expect("vkCmdCuLaunchKernelNVX is not loaded"))(command_buffer, p_launch_info)
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_cuda_launch_kernel_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_launch_info: *const CudaLaunchInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_cuda_launch_kernel_nv
            .expect("vkCmdCudaLaunchKernelNV is not loaded"))(command_buffer, p_launch_info)
    }
    pub unsafe fn cmd_debug_marker_begin_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_debug_marker_begin_ext
            .expect("vkCmdDebugMarkerBeginEXT is not loaded"))(command_buffer, p_marker_info)
    }
    pub unsafe fn cmd_debug_marker_end_ext_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_debug_marker_end_ext
            .expect("vkCmdDebugMarkerEndEXT is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_debug_marker_insert_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_debug_marker_insert_ext
            .expect("vkCmdDebugMarkerInsertEXT is not loaded"))(
            command_buffer, p_marker_info
        )
    }
    pub unsafe fn cmd_decode_video_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_decode_info: *const VideoDecodeInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_decode_video_khr
            .expect("vkCmdDecodeVideoKHR is not loaded"))(command_buffer, p_decode_info)
    }
    pub unsafe fn cmd_decompress_memory_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_decompress_memory_ext
            .expect("vkCmdDecompressMemoryEXT is not loaded"))(
            command_buffer,
            p_decompress_memory_info_ext,
        )
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: u64,
        indirect_commands_count_address: u64,
        max_decompression_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_decompress_memory_indirect_count_ext
            .expect("vkCmdDecompressMemoryIndirectCountEXT is not loaded"))(
            command_buffer,
            decompression_method,
            indirect_commands_address,
            indirect_commands_count_address,
            max_decompression_count,
            stride,
        )
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        indirect_commands_address: u64,
        indirect_commands_count_address: u64,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_decompress_memory_indirect_count_nv
            .expect("vkCmdDecompressMemoryIndirectCountNV is not loaded"))(
            command_buffer,
            indirect_commands_address,
            indirect_commands_count_address,
            stride,
        )
    }
    pub unsafe fn cmd_decompress_memory_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        decompress_region_count: u32,
        p_decompress_memory_regions: *const DecompressMemoryRegionNV<'_>,
    ) {
        (self
            .fp()
            .cmd_decompress_memory_nv
            .expect("vkCmdDecompressMemoryNV is not loaded"))(
            command_buffer,
            decompress_region_count,
            p_decompress_memory_regions,
        )
    }
    pub unsafe fn cmd_dispatch_raw(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self.fp().cmd_dispatch.expect("vkCmdDispatch is not loaded"))(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_dispatch_base_raw(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .fp()
            .cmd_dispatch_base
            .expect("vkCmdDispatchBase is not loaded"))(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_dispatch_base_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .fp()
            .cmd_dispatch_base_khr
            .expect("vkCmdDispatchBaseKHR is not loaded"))(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_dispatch_data_graph_arm_raw(
        &self,
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        p_info: *const DataGraphPipelineDispatchInfoARM<'_>,
    ) {
        (self
            .fp()
            .cmd_dispatch_data_graph_arm
            .expect("vkCmdDispatchDataGraphARM is not loaded"))(
            command_buffer, session, p_info
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_amdx_raw(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    ) {
        (self
            .fp()
            .cmd_dispatch_graph_amdx
            .expect("vkCmdDispatchGraphAMDX is not loaded"))(
            command_buffer,
            scratch,
            scratch_size,
            p_count_info,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_indirect_amdx_raw(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    ) {
        (self
            .fp()
            .cmd_dispatch_graph_indirect_amdx
            .expect("vkCmdDispatchGraphIndirectAMDX is not loaded"))(
            command_buffer,
            scratch,
            scratch_size,
            p_count_info,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_indirect_count_amdx_raw(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        count_info: u64,
    ) {
        (self
            .fp()
            .cmd_dispatch_graph_indirect_count_amdx
            .expect("vkCmdDispatchGraphIndirectCountAMDX is not loaded"))(
            command_buffer,
            scratch,
            scratch_size,
            count_info,
        )
    }
    pub unsafe fn cmd_dispatch_indirect_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
    ) {
        (self
            .fp()
            .cmd_dispatch_indirect
            .expect("vkCmdDispatchIndirect is not loaded"))(command_buffer, buffer, offset)
    }
    pub unsafe fn cmd_dispatch_indirect2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DispatchIndirect2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_dispatch_indirect2_khr
            .expect("vkCmdDispatchIndirect2KHR is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_dispatch_tile_qcom_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dispatch_tile_info: *const DispatchTileInfoQCOM<'_>,
    ) {
        (self
            .fp()
            .cmd_dispatch_tile_qcom
            .expect("vkCmdDispatchTileQCOM is not loaded"))(
            command_buffer, p_dispatch_tile_info
        )
    }
    pub unsafe fn cmd_draw_raw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        (self.fp().cmd_draw.expect("vkCmdDraw is not loaded"))(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_cluster_huawei_raw(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .fp()
            .cmd_draw_cluster_huawei
            .expect("vkCmdDrawClusterHUAWEI is not loaded"))(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_draw_cluster_indirect_huawei_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
    ) {
        (self
            .fp()
            .cmd_draw_cluster_indirect_huawei
            .expect("vkCmdDrawClusterIndirectHUAWEI is not loaded"))(
            command_buffer, buffer, offset
        )
    }
    pub unsafe fn cmd_draw_indexed_raw(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indexed
            .expect("vkCmdDrawIndexed is not loaded"))(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect
            .expect("vkCmdDrawIndexedIndirect is not loaded"))(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect2_khr
            .expect("vkCmdDrawIndexedIndirect2KHR is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect_count
            .expect("vkCmdDrawIndexedIndirectCount is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect_count2_khr
            .expect("vkCmdDrawIndexedIndirectCount2KHR is not loaded"))(
            command_buffer, p_info
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_amd_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect_count_amd
            .expect("vkCmdDrawIndexedIndirectCountAMD is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indexed_indirect_count_khr
            .expect("vkCmdDrawIndexedIndirectCountKHR is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect
            .expect("vkCmdDrawIndirect is not loaded"))(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_indirect2_khr
            .expect("vkCmdDrawIndirect2KHR is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_draw_indirect_byte_count2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_byte_count2_ext
            .expect("vkCmdDrawIndirectByteCount2EXT is not loaded"))(
            command_buffer,
            instance_count,
            first_instance,
            p_counter_info,
            counter_offset,
            vertex_stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_byte_count_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: u64,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_byte_count_ext
            .expect("vkCmdDrawIndirectByteCountEXT is not loaded"))(
            command_buffer,
            instance_count,
            first_instance,
            counter_buffer,
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_count
            .expect("vkCmdDrawIndirectCount is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_count2_khr
            .expect("vkCmdDrawIndirectCount2KHR is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_draw_indirect_count_amd_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_count_amd
            .expect("vkCmdDrawIndirectCountAMD is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_indirect_count_khr
            .expect("vkCmdDrawIndirectCountKHR is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_ext
            .expect("vkCmdDrawMeshTasksEXT is not loaded"))(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect2_ext
            .expect("vkCmdDrawMeshTasksIndirect2EXT is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect_count2_ext
            .expect("vkCmdDrawMeshTasksIndirectCount2EXT is not loaded"))(
            command_buffer, p_info
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect_count_ext
            .expect("vkCmdDrawMeshTasksIndirectCountEXT is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect_count_nv
            .expect("vkCmdDrawMeshTasksIndirectCountNV is not loaded"))(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect_ext
            .expect("vkCmdDrawMeshTasksIndirectEXT is not loaded"))(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_indirect_nv
            .expect("vkCmdDrawMeshTasksIndirectNV is not loaded"))(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        (self
            .fp()
            .cmd_draw_mesh_tasks_nv
            .expect("vkCmdDrawMeshTasksNV is not loaded"))(
            command_buffer, task_count, first_task
        )
    }
    pub unsafe fn cmd_draw_multi_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const MultiDrawInfoEXT<'_>,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        (self
            .fp()
            .cmd_draw_multi_ext
            .expect("vkCmdDrawMultiEXT is not loaded"))(
            command_buffer,
            draw_count,
            p_vertex_info,
            instance_count,
            first_instance,
            stride,
        )
    }
    pub unsafe fn cmd_draw_multi_indexed_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_index_info: *const MultiDrawIndexedInfoEXT<'_>,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ) {
        (self
            .fp()
            .cmd_draw_multi_indexed_ext
            .expect("vkCmdDrawMultiIndexedEXT is not loaded"))(
            command_buffer,
            draw_count,
            p_index_info,
            instance_count,
            first_instance,
            stride,
            p_vertex_offset,
        )
    }
    pub unsafe fn cmd_encode_video_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_encode_info: *const VideoEncodeInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_encode_video_khr
            .expect("vkCmdEncodeVideoKHR is not loaded"))(command_buffer, p_encode_info)
    }
    pub unsafe fn cmd_end_conditional_rendering_ext_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_conditional_rendering_ext
            .expect("vkCmdEndConditionalRenderingEXT is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_debug_utils_label_ext_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_debug_utils_label_ext
            .expect("vkCmdEndDebugUtilsLabelEXT is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_per_tile_execution_qcom_raw(
        &self,
        command_buffer: CommandBuffer,
        p_per_tile_end_info: *const PerTileEndInfoQCOM<'_>,
    ) {
        (self
            .fp()
            .cmd_end_per_tile_execution_qcom
            .expect("vkCmdEndPerTileExecutionQCOM is not loaded"))(
            command_buffer,
            p_per_tile_end_info,
        )
    }
    pub unsafe fn cmd_end_query_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        (self
            .fp()
            .cmd_end_query
            .expect("vkCmdEndQuery is not loaded"))(command_buffer, query_pool, query)
    }
    pub unsafe fn cmd_end_query_indexed_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ) {
        (self
            .fp()
            .cmd_end_query_indexed_ext
            .expect("vkCmdEndQueryIndexedEXT is not loaded"))(
            command_buffer,
            query_pool,
            query,
            index,
        )
    }
    pub unsafe fn cmd_end_render_pass_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_render_pass
            .expect("vkCmdEndRenderPass is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_render_pass2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_end_render_pass2
            .expect("vkCmdEndRenderPass2 is not loaded"))(command_buffer, p_subpass_end_info)
    }
    pub unsafe fn cmd_end_render_pass2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_end_render_pass2_khr
            .expect("vkCmdEndRenderPass2KHR is not loaded"))(
            command_buffer, p_subpass_end_info
        )
    }
    pub unsafe fn cmd_end_rendering_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_rendering
            .expect("vkCmdEndRendering is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_rendering2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_end_rendering2_ext
            .expect("vkCmdEndRendering2EXT is not loaded"))(
            command_buffer, p_rendering_end_info
        )
    }
    pub unsafe fn cmd_end_rendering2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_end_rendering2_khr
            .expect("vkCmdEndRendering2KHR is not loaded"))(
            command_buffer, p_rendering_end_info
        )
    }
    pub unsafe fn cmd_end_rendering_khr_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_rendering_khr
            .expect("vkCmdEndRenderingKHR is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_shader_instrumentation_arm_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_end_shader_instrumentation_arm
            .expect("vkCmdEndShaderInstrumentationARM is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_end_transform_feedback2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_end_transform_feedback2_ext
            .expect("vkCmdEndTransformFeedback2EXT is not loaded"))(
            command_buffer,
            first_counter_range,
            counter_range_count,
            p_counter_infos,
        )
    }
    pub unsafe fn cmd_end_transform_feedback_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const u64,
    ) {
        (self
            .fp()
            .cmd_end_transform_feedback_ext
            .expect("vkCmdEndTransformFeedbackEXT is not loaded"))(
            command_buffer,
            first_counter_buffer,
            counter_buffer_count,
            p_counter_buffers,
            p_counter_buffer_offsets,
        )
    }
    pub unsafe fn cmd_end_video_coding_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
    ) {
        (self
            .fp()
            .cmd_end_video_coding_khr
            .expect("vkCmdEndVideoCodingKHR is not loaded"))(
            command_buffer, p_end_coding_info
        )
    }
    pub unsafe fn cmd_execute_commands_raw(
        &self,
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        (self
            .fp()
            .cmd_execute_commands
            .expect("vkCmdExecuteCommands is not loaded"))(
            command_buffer,
            command_buffer_count,
            p_command_buffers,
        )
    }
    pub unsafe fn cmd_execute_generated_commands_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_execute_generated_commands_ext
            .expect("vkCmdExecuteGeneratedCommandsEXT is not loaded"))(
            command_buffer,
            is_preprocessed,
            p_generated_commands_info,
        )
    }
    pub unsafe fn cmd_execute_generated_commands_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_execute_generated_commands_nv
            .expect("vkCmdExecuteGeneratedCommandsNV is not loaded"))(
            command_buffer,
            is_preprocessed,
            p_generated_commands_info,
        )
    }
    pub unsafe fn cmd_fill_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: u64,
        size: u64,
        data: u32,
    ) {
        (self
            .fp()
            .cmd_fill_buffer
            .expect("vkCmdFillBuffer is not loaded"))(
            command_buffer,
            dst_buffer,
            dst_offset,
            size,
            data,
        )
    }
    pub unsafe fn cmd_fill_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        (self
            .fp()
            .cmd_fill_memory_khr
            .expect("vkCmdFillMemoryKHR is not loaded"))(
            command_buffer,
            p_dst_range,
            dst_flags,
            data,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_initialize_graph_scratch_memory_amdx_raw(
        &self,
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: u64,
        scratch_size: u64,
    ) {
        (self
            .fp()
            .cmd_initialize_graph_scratch_memory_amdx
            .expect("vkCmdInitializeGraphScratchMemoryAMDX is not loaded"))(
            command_buffer,
            execution_graph,
            scratch,
            scratch_size,
        )
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_insert_debug_utils_label_ext
            .expect("vkCmdInsertDebugUtilsLabelEXT is not loaded"))(
            command_buffer, p_label_info
        )
    }
    pub unsafe fn cmd_next_subpass_raw(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        (self
            .fp()
            .cmd_next_subpass
            .expect("vkCmdNextSubpass is not loaded"))(command_buffer, contents)
    }
    pub unsafe fn cmd_next_subpass2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_next_subpass2
            .expect("vkCmdNextSubpass2 is not loaded"))(
            command_buffer,
            p_subpass_begin_info,
            p_subpass_end_info,
        )
    }
    pub unsafe fn cmd_next_subpass2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_next_subpass2_khr
            .expect("vkCmdNextSubpass2KHR is not loaded"))(
            command_buffer,
            p_subpass_begin_info,
            p_subpass_end_info,
        )
    }
    pub unsafe fn cmd_optical_flow_execute_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_optical_flow_execute_nv
            .expect("vkCmdOpticalFlowExecuteNV is not loaded"))(
            command_buffer,
            session,
            p_execute_info,
        )
    }
    pub unsafe fn cmd_pipeline_barrier_raw(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier<'_>,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
    ) {
        (self
            .fp()
            .cmd_pipeline_barrier
            .expect("vkCmdPipelineBarrier is not loaded"))(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,
        )
    }
    pub unsafe fn cmd_pipeline_barrier2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_pipeline_barrier2
            .expect("vkCmdPipelineBarrier2 is not loaded"))(
            command_buffer, p_dependency_info
        )
    }
    pub unsafe fn cmd_pipeline_barrier2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_pipeline_barrier2_khr
            .expect("vkCmdPipelineBarrier2KHR is not loaded"))(
            command_buffer, p_dependency_info
        )
    }
    pub unsafe fn cmd_preprocess_generated_commands_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
        state_command_buffer: CommandBuffer,
    ) {
        (self
            .fp()
            .cmd_preprocess_generated_commands_ext
            .expect("vkCmdPreprocessGeneratedCommandsEXT is not loaded"))(
            command_buffer,
            p_generated_commands_info,
            state_command_buffer,
        )
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    ) {
        (self
            .fp()
            .cmd_preprocess_generated_commands_nv
            .expect("vkCmdPreprocessGeneratedCommandsNV is not loaded"))(
            command_buffer,
            p_generated_commands_info,
        )
    }
    pub unsafe fn cmd_push_constants_raw(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ) {
        (self
            .fp()
            .cmd_push_constants
            .expect("vkCmdPushConstants is not loaded"))(
            command_buffer,
            layout,
            stage_flags,
            offset,
            size,
            p_values,
        )
    }
    pub unsafe fn cmd_push_constants2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_constants2
            .expect("vkCmdPushConstants2 is not loaded"))(
            command_buffer, p_push_constants_info
        )
    }
    pub unsafe fn cmd_push_constants2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_constants2_khr
            .expect("vkCmdPushConstants2KHR is not loaded"))(
            command_buffer, p_push_constants_info
        )
    }
    pub unsafe fn cmd_push_data_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_data_info: *const PushDataInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_push_data_ext
            .expect("vkCmdPushDataEXT is not loaded"))(command_buffer, p_push_data_info)
    }
    pub unsafe fn cmd_push_descriptor_set_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set
            .expect("vkCmdPushDescriptorSet is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_write_count,
            p_descriptor_writes,
        )
    }
    pub unsafe fn cmd_push_descriptor_set2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set2
            .expect("vkCmdPushDescriptorSet2 is not loaded"))(
            command_buffer,
            p_push_descriptor_set_info,
        )
    }
    pub unsafe fn cmd_push_descriptor_set2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set2_khr
            .expect("vkCmdPushDescriptorSet2KHR is not loaded"))(
            command_buffer,
            p_push_descriptor_set_info,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set_khr
            .expect("vkCmdPushDescriptorSetKHR is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_write_count,
            p_descriptor_writes,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_raw(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set_with_template
            .expect("vkCmdPushDescriptorSetWithTemplate is not loaded"))(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            p_data,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set_with_template2
            .expect("vkCmdPushDescriptorSetWithTemplate2 is not loaded"))(
            command_buffer,
            p_push_descriptor_set_with_template_info,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set_with_template2_khr
            .expect("vkCmdPushDescriptorSetWithTemplate2KHR is not loaded"))(
            command_buffer,
            p_push_descriptor_set_with_template_info,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .cmd_push_descriptor_set_with_template_khr
            .expect("vkCmdPushDescriptorSetWithTemplateKHR is not loaded"))(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            p_data,
        )
    }
    pub unsafe fn cmd_reset_event_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        (self
            .fp()
            .cmd_reset_event
            .expect("vkCmdResetEvent is not loaded"))(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_event2_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        (self
            .fp()
            .cmd_reset_event2
            .expect("vkCmdResetEvent2 is not loaded"))(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_event2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        (self
            .fp()
            .cmd_reset_event2_khr
            .expect("vkCmdResetEvent2KHR is not loaded"))(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_query_pool_raw(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .fp()
            .cmd_reset_query_pool
            .expect("vkCmdResetQueryPool is not loaded"))(
            command_buffer,
            query_pool,
            first_query,
            query_count,
        )
    }
    pub unsafe fn cmd_resolve_image_raw(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve<'_>,
    ) {
        (self
            .fp()
            .cmd_resolve_image
            .expect("vkCmdResolveImage is not loaded"))(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            region_count,
            p_regions,
        )
    }
    pub unsafe fn cmd_resolve_image2_raw(
        &self,
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_resolve_image2
            .expect("vkCmdResolveImage2 is not loaded"))(
            command_buffer, p_resolve_image_info
        )
    }
    pub unsafe fn cmd_resolve_image2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    ) {
        (self
            .fp()
            .cmd_resolve_image2_khr
            .expect("vkCmdResolveImage2KHR is not loaded"))(
            command_buffer, p_resolve_image_info
        )
    }
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_coverage_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_alpha_to_coverage_enable_ext
            .expect("vkCmdSetAlphaToCoverageEnableEXT is not loaded"))(
            command_buffer,
            alpha_to_coverage_enable,
        )
    }
    pub unsafe fn cmd_set_alpha_to_one_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_one_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_alpha_to_one_enable_ext
            .expect("vkCmdSetAlphaToOneEnableEXT is not loaded"))(
            command_buffer,
            alpha_to_one_enable,
        )
    }
    pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        aspect_mask: ImageAspectFlags,
    ) {
        (self
            .fp()
            .cmd_set_attachment_feedback_loop_enable_ext
            .expect("vkCmdSetAttachmentFeedbackLoopEnableEXT is not loaded"))(
            command_buffer,
            aspect_mask,
        )
    }
    pub unsafe fn cmd_set_blend_constants_raw(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: [f32; 4],
    ) {
        (self
            .fp()
            .cmd_set_blend_constants
            .expect("vkCmdSetBlendConstants is not loaded"))(command_buffer, blend_constants)
    }
    pub unsafe fn cmd_set_checkpoint_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_checkpoint_marker: *const c_void,
    ) {
        (self
            .fp()
            .cmd_set_checkpoint_nv
            .expect("vkCmdSetCheckpointNV is not loaded"))(
            command_buffer, p_checkpoint_marker
        )
    }
    pub unsafe fn cmd_set_coarse_sample_order_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
    ) {
        (self
            .fp()
            .cmd_set_coarse_sample_order_nv
            .expect("vkCmdSetCoarseSampleOrderNV is not loaded"))(
            command_buffer,
            sample_order_type,
            custom_sample_order_count,
            p_custom_sample_orders,
        )
    }
    pub unsafe fn cmd_set_color_blend_advanced_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const ColorBlendAdvancedEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_color_blend_advanced_ext
            .expect("vkCmdSetColorBlendAdvancedEXT is not loaded"))(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_advanced,
        )
    }
    pub unsafe fn cmd_set_color_blend_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const Bool32,
    ) {
        (self
            .fp()
            .cmd_set_color_blend_enable_ext
            .expect("vkCmdSetColorBlendEnableEXT is not loaded"))(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_enables,
        )
    }
    pub unsafe fn cmd_set_color_blend_equation_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const ColorBlendEquationEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_color_blend_equation_ext
            .expect("vkCmdSetColorBlendEquationEXT is not loaded"))(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_blend_equations,
        )
    }
    pub unsafe fn cmd_set_color_write_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_color_write_enables: *const Bool32,
    ) {
        (self
            .fp()
            .cmd_set_color_write_enable_ext
            .expect("vkCmdSetColorWriteEnableEXT is not loaded"))(
            command_buffer,
            attachment_count,
            p_color_write_enables,
        )
    }
    pub unsafe fn cmd_set_color_write_mask_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const ColorComponentFlags,
    ) {
        (self
            .fp()
            .cmd_set_color_write_mask_ext
            .expect("vkCmdSetColorWriteMaskEXT is not loaded"))(
            command_buffer,
            first_attachment,
            attachment_count,
            p_color_write_masks,
        )
    }
    pub unsafe fn cmd_set_compute_occupancy_priority_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        p_parameters: *const ComputeOccupancyPriorityParametersNV<'_>,
    ) {
        (self
            .fp()
            .cmd_set_compute_occupancy_priority_nv
            .expect("vkCmdSetComputeOccupancyPriorityNV is not loaded"))(
            command_buffer,
            p_parameters,
        )
    }
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        (self
            .fp()
            .cmd_set_conservative_rasterization_mode_ext
            .expect("vkCmdSetConservativeRasterizationModeEXT is not loaded"))(
            command_buffer,
            conservative_rasterization_mode,
        )
    }
    pub unsafe fn cmd_set_coverage_modulation_mode_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        (self
            .fp()
            .cmd_set_coverage_modulation_mode_nv
            .expect("vkCmdSetCoverageModulationModeNV is not loaded"))(
            command_buffer,
            coverage_modulation_mode,
        )
    }
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_coverage_modulation_table_enable_nv
            .expect("vkCmdSetCoverageModulationTableEnableNV is not loaded"))(
            command_buffer,
            coverage_modulation_table_enable,
        )
    }
    pub unsafe fn cmd_set_coverage_modulation_table_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const f32,
    ) {
        (self
            .fp()
            .cmd_set_coverage_modulation_table_nv
            .expect("vkCmdSetCoverageModulationTableNV is not loaded"))(
            command_buffer,
            coverage_modulation_table_count,
            p_coverage_modulation_table,
        )
    }
    pub unsafe fn cmd_set_coverage_reduction_mode_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        (self
            .fp()
            .cmd_set_coverage_reduction_mode_nv
            .expect("vkCmdSetCoverageReductionModeNV is not loaded"))(
            command_buffer,
            coverage_reduction_mode,
        )
    }
    pub unsafe fn cmd_set_coverage_to_color_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_coverage_to_color_enable_nv
            .expect("vkCmdSetCoverageToColorEnableNV is not loaded"))(
            command_buffer,
            coverage_to_color_enable,
        )
    }
    pub unsafe fn cmd_set_coverage_to_color_location_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        (self
            .fp()
            .cmd_set_coverage_to_color_location_nv
            .expect("vkCmdSetCoverageToColorLocationNV is not loaded"))(
            command_buffer,
            coverage_to_color_location,
        )
    }
    pub unsafe fn cmd_set_cull_mode_raw(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        (self
            .fp()
            .cmd_set_cull_mode
            .expect("vkCmdSetCullMode is not loaded"))(command_buffer, cull_mode)
    }
    pub unsafe fn cmd_set_cull_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        (self
            .fp()
            .cmd_set_cull_mode_ext
            .expect("vkCmdSetCullModeEXT is not loaded"))(command_buffer, cull_mode)
    }
    pub unsafe fn cmd_set_depth_bias_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bias
            .expect("vkCmdSetDepthBias is not loaded"))(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        )
    }
    pub unsafe fn cmd_set_depth_bias2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_depth_bias2_ext
            .expect("vkCmdSetDepthBias2EXT is not loaded"))(
            command_buffer, p_depth_bias_info
        )
    }
    pub unsafe fn cmd_set_depth_bias_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bias_enable
            .expect("vkCmdSetDepthBiasEnable is not loaded"))(
            command_buffer, depth_bias_enable
        )
    }
    pub unsafe fn cmd_set_depth_bias_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bias_enable_ext
            .expect("vkCmdSetDepthBiasEnableEXT is not loaded"))(
            command_buffer, depth_bias_enable
        )
    }
    pub unsafe fn cmd_set_depth_bounds_raw(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bounds
            .expect("vkCmdSetDepthBounds is not loaded"))(
            command_buffer,
            min_depth_bounds,
            max_depth_bounds,
        )
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bounds_test_enable
            .expect("vkCmdSetDepthBoundsTestEnable is not loaded"))(
            command_buffer,
            depth_bounds_test_enable,
        )
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_bounds_test_enable_ext
            .expect("vkCmdSetDepthBoundsTestEnableEXT is not loaded"))(
            command_buffer,
            depth_bounds_test_enable,
        )
    }
    pub unsafe fn cmd_set_depth_clamp_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_clamp_enable_ext
            .expect("vkCmdSetDepthClampEnableEXT is not loaded"))(
            command_buffer,
            depth_clamp_enable,
        )
    }
    pub unsafe fn cmd_set_depth_clamp_range_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        p_depth_clamp_range: *const DepthClampRangeEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_depth_clamp_range_ext
            .expect("vkCmdSetDepthClampRangeEXT is not loaded"))(
            command_buffer,
            depth_clamp_mode,
            p_depth_clamp_range,
        )
    }
    pub unsafe fn cmd_set_depth_clip_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_clip_enable_ext
            .expect("vkCmdSetDepthClipEnableEXT is not loaded"))(
            command_buffer, depth_clip_enable
        )
    }
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_clip_negative_one_to_one_ext
            .expect("vkCmdSetDepthClipNegativeOneToOneEXT is not loaded"))(
            command_buffer,
            negative_one_to_one,
        )
    }
    pub unsafe fn cmd_set_depth_compare_op_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        (self
            .fp()
            .cmd_set_depth_compare_op
            .expect("vkCmdSetDepthCompareOp is not loaded"))(
            command_buffer, depth_compare_op
        )
    }
    pub unsafe fn cmd_set_depth_compare_op_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        (self
            .fp()
            .cmd_set_depth_compare_op_ext
            .expect("vkCmdSetDepthCompareOpEXT is not loaded"))(
            command_buffer, depth_compare_op
        )
    }
    pub unsafe fn cmd_set_depth_test_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_test_enable
            .expect("vkCmdSetDepthTestEnable is not loaded"))(
            command_buffer, depth_test_enable
        )
    }
    pub unsafe fn cmd_set_depth_test_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_test_enable_ext
            .expect("vkCmdSetDepthTestEnableEXT is not loaded"))(
            command_buffer, depth_test_enable
        )
    }
    pub unsafe fn cmd_set_depth_write_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_write_enable
            .expect("vkCmdSetDepthWriteEnable is not loaded"))(
            command_buffer, depth_write_enable
        )
    }
    pub unsafe fn cmd_set_depth_write_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_depth_write_enable_ext
            .expect("vkCmdSetDepthWriteEnableEXT is not loaded"))(
            command_buffer,
            depth_write_enable,
        )
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_descriptor_buffer_offsets2_ext
            .expect("vkCmdSetDescriptorBufferOffsets2EXT is not loaded"))(
            command_buffer,
            p_set_descriptor_buffer_offsets_info,
        )
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const u64,
    ) {
        (self
            .fp()
            .cmd_set_descriptor_buffer_offsets_ext
            .expect("vkCmdSetDescriptorBufferOffsetsEXT is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            set_count,
            p_buffer_indices,
            p_offsets,
        )
    }
    pub unsafe fn cmd_set_device_mask_raw(&self, command_buffer: CommandBuffer, device_mask: u32) {
        (self
            .fp()
            .cmd_set_device_mask
            .expect("vkCmdSetDeviceMask is not loaded"))(command_buffer, device_mask)
    }
    pub unsafe fn cmd_set_device_mask_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        device_mask: u32,
    ) {
        (self
            .fp()
            .cmd_set_device_mask_khr
            .expect("vkCmdSetDeviceMaskKHR is not loaded"))(command_buffer, device_mask)
    }
    pub unsafe fn cmd_set_discard_rectangle_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D<'_>,
    ) {
        (self
            .fp()
            .cmd_set_discard_rectangle_ext
            .expect("vkCmdSetDiscardRectangleEXT is not loaded"))(
            command_buffer,
            first_discard_rectangle,
            discard_rectangle_count,
            p_discard_rectangles,
        )
    }
    pub unsafe fn cmd_set_discard_rectangle_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_discard_rectangle_enable_ext
            .expect("vkCmdSetDiscardRectangleEnableEXT is not loaded"))(
            command_buffer,
            discard_rectangle_enable,
        )
    }
    pub unsafe fn cmd_set_discard_rectangle_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        (self
            .fp()
            .cmd_set_discard_rectangle_mode_ext
            .expect("vkCmdSetDiscardRectangleModeEXT is not loaded"))(
            command_buffer,
            discard_rectangle_mode,
        )
    }
    pub unsafe fn cmd_set_dispatch_parameters_arm_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dispatch_parameters: *const DispatchParametersARM<'_>,
    ) {
        (self
            .fp()
            .cmd_set_dispatch_parameters_arm
            .expect("vkCmdSetDispatchParametersARM is not loaded"))(
            command_buffer,
            p_dispatch_parameters,
        )
    }
    pub unsafe fn cmd_set_event_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        (self
            .fp()
            .cmd_set_event
            .expect("vkCmdSetEvent is not loaded"))(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_set_event2_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_event2
            .expect("vkCmdSetEvent2 is not loaded"))(
            command_buffer, event, p_dependency_info
        )
    }
    pub unsafe fn cmd_set_event2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_event2_khr
            .expect("vkCmdSetEvent2KHR is not loaded"))(
            command_buffer, event, p_dependency_info
        )
    }
    pub unsafe fn cmd_set_exclusive_scissor_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissor_enables: *const Bool32,
    ) {
        (self
            .fp()
            .cmd_set_exclusive_scissor_enable_nv
            .expect("vkCmdSetExclusiveScissorEnableNV is not loaded"))(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissor_enables,
        )
    }
    pub unsafe fn cmd_set_exclusive_scissor_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D<'_>,
    ) {
        (self
            .fp()
            .cmd_set_exclusive_scissor_nv
            .expect("vkCmdSetExclusiveScissorNV is not loaded"))(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissor_count,
            p_exclusive_scissors,
        )
    }
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        (self
            .fp()
            .cmd_set_extra_primitive_overestimation_size_ext
            .expect("vkCmdSetExtraPrimitiveOverestimationSizeEXT is not loaded"))(
            command_buffer,
            extra_primitive_overestimation_size,
        )
    }
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) {
        (self
            .fp()
            .cmd_set_fragment_shading_rate_enum_nv
            .expect("vkCmdSetFragmentShadingRateEnumNV is not loaded"))(
            command_buffer,
            shading_rate,
            combiner_ops,
        )
    }
    pub unsafe fn cmd_set_fragment_shading_rate_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_fragment_size: *const Extent2D<'_>,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) {
        (self
            .fp()
            .cmd_set_fragment_shading_rate_khr
            .expect("vkCmdSetFragmentShadingRateKHR is not loaded"))(
            command_buffer,
            p_fragment_size,
            combiner_ops,
        )
    }
    pub unsafe fn cmd_set_front_face_raw(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        (self
            .fp()
            .cmd_set_front_face
            .expect("vkCmdSetFrontFace is not loaded"))(command_buffer, front_face)
    }
    pub unsafe fn cmd_set_front_face_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        (self
            .fp()
            .cmd_set_front_face_ext
            .expect("vkCmdSetFrontFaceEXT is not loaded"))(command_buffer, front_face)
    }
    pub unsafe fn cmd_set_line_rasterization_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationMode,
    ) {
        (self
            .fp()
            .cmd_set_line_rasterization_mode_ext
            .expect("vkCmdSetLineRasterizationModeEXT is not loaded"))(
            command_buffer,
            line_rasterization_mode,
        )
    }
    pub unsafe fn cmd_set_line_stipple_raw(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self
            .fp()
            .cmd_set_line_stipple
            .expect("vkCmdSetLineStipple is not loaded"))(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        )
    }
    pub unsafe fn cmd_set_line_stipple_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self
            .fp()
            .cmd_set_line_stipple_ext
            .expect("vkCmdSetLineStippleEXT is not loaded"))(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        )
    }
    pub unsafe fn cmd_set_line_stipple_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_line_stipple_enable_ext
            .expect("vkCmdSetLineStippleEnableEXT is not loaded"))(
            command_buffer,
            stippled_line_enable,
        )
    }
    pub unsafe fn cmd_set_line_stipple_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        (self
            .fp()
            .cmd_set_line_stipple_khr
            .expect("vkCmdSetLineStippleKHR is not loaded"))(
            command_buffer,
            line_stipple_factor,
            line_stipple_pattern,
        )
    }
    pub unsafe fn cmd_set_line_width_raw(&self, command_buffer: CommandBuffer, line_width: f32) {
        (self
            .fp()
            .cmd_set_line_width
            .expect("vkCmdSetLineWidth is not loaded"))(command_buffer, line_width)
    }
    pub unsafe fn cmd_set_logic_op_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        logic_op: LogicOp,
    ) {
        (self
            .fp()
            .cmd_set_logic_op_ext
            .expect("vkCmdSetLogicOpEXT is not loaded"))(command_buffer, logic_op)
    }
    pub unsafe fn cmd_set_logic_op_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        logic_op_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_logic_op_enable_ext
            .expect("vkCmdSetLogicOpEnableEXT is not loaded"))(
            command_buffer, logic_op_enable
        )
    }
    pub unsafe fn cmd_set_patch_control_points_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        (self
            .fp()
            .cmd_set_patch_control_points_ext
            .expect("vkCmdSetPatchControlPointsEXT is not loaded"))(
            command_buffer,
            patch_control_points,
        )
    }
    pub unsafe fn cmd_set_performance_marker_intel_raw(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
    ) -> VkResult {
        (self
            .fp()
            .cmd_set_performance_marker_intel
            .expect("vkCmdSetPerformanceMarkerINTEL is not loaded"))(
            command_buffer, p_marker_info
        )
    }
    pub unsafe fn cmd_set_performance_override_intel_raw(
        &self,
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
    ) -> VkResult {
        (self
            .fp()
            .cmd_set_performance_override_intel
            .expect("vkCmdSetPerformanceOverrideINTEL is not loaded"))(
            command_buffer,
            p_override_info,
        )
    }
    pub unsafe fn cmd_set_performance_stream_marker_intel_raw(
        &self,
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
    ) -> VkResult {
        (self
            .fp()
            .cmd_set_performance_stream_marker_intel
            .expect("vkCmdSetPerformanceStreamMarkerINTEL is not loaded"))(
            command_buffer,
            p_marker_info,
        )
    }
    pub unsafe fn cmd_set_polygon_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        polygon_mode: PolygonMode,
    ) {
        (self
            .fp()
            .cmd_set_polygon_mode_ext
            .expect("vkCmdSetPolygonModeEXT is not loaded"))(command_buffer, polygon_mode)
    }
    pub unsafe fn cmd_set_primitive_restart_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_primitive_restart_enable
            .expect("vkCmdSetPrimitiveRestartEnable is not loaded"))(
            command_buffer,
            primitive_restart_enable,
        )
    }
    pub unsafe fn cmd_set_primitive_restart_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_primitive_restart_enable_ext
            .expect("vkCmdSetPrimitiveRestartEnableEXT is not loaded"))(
            command_buffer,
            primitive_restart_enable,
        )
    }
    pub unsafe fn cmd_set_primitive_restart_index_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_index: u32,
    ) {
        (self
            .fp()
            .cmd_set_primitive_restart_index_ext
            .expect("vkCmdSetPrimitiveRestartIndexEXT is not loaded"))(
            command_buffer,
            primitive_restart_index,
        )
    }
    pub unsafe fn cmd_set_primitive_topology_raw(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        (self
            .fp()
            .cmd_set_primitive_topology
            .expect("vkCmdSetPrimitiveTopology is not loaded"))(
            command_buffer, primitive_topology
        )
    }
    pub unsafe fn cmd_set_primitive_topology_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        (self
            .fp()
            .cmd_set_primitive_topology_ext
            .expect("vkCmdSetPrimitiveTopologyEXT is not loaded"))(
            command_buffer,
            primitive_topology,
        )
    }
    pub unsafe fn cmd_set_provoking_vertex_mode_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        (self
            .fp()
            .cmd_set_provoking_vertex_mode_ext
            .expect("vkCmdSetProvokingVertexModeEXT is not loaded"))(
            command_buffer,
            provoking_vertex_mode,
        )
    }
    pub unsafe fn cmd_set_rasterization_samples_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    ) {
        (self
            .fp()
            .cmd_set_rasterization_samples_ext
            .expect("vkCmdSetRasterizationSamplesEXT is not loaded"))(
            command_buffer,
            rasterization_samples,
        )
    }
    pub unsafe fn cmd_set_rasterization_stream_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        rasterization_stream: u32,
    ) {
        (self
            .fp()
            .cmd_set_rasterization_stream_ext
            .expect("vkCmdSetRasterizationStreamEXT is not loaded"))(
            command_buffer,
            rasterization_stream,
        )
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_rasterizer_discard_enable
            .expect("vkCmdSetRasterizerDiscardEnable is not loaded"))(
            command_buffer,
            rasterizer_discard_enable,
        )
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_rasterizer_discard_enable_ext
            .expect("vkCmdSetRasterizerDiscardEnableEXT is not loaded"))(
            command_buffer,
            rasterizer_discard_enable,
        )
    }
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        (self
            .fp()
            .cmd_set_ray_tracing_pipeline_stack_size_khr
            .expect("vkCmdSetRayTracingPipelineStackSizeKHR is not loaded"))(
            command_buffer,
            pipeline_stack_size,
        )
    }
    pub unsafe fn cmd_set_rendering_attachment_locations_raw(
        &self,
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_rendering_attachment_locations
            .expect("vkCmdSetRenderingAttachmentLocations is not loaded"))(
            command_buffer,
            p_location_info,
        )
    }
    pub unsafe fn cmd_set_rendering_attachment_locations_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_rendering_attachment_locations_khr
            .expect("vkCmdSetRenderingAttachmentLocationsKHR is not loaded"))(
            command_buffer,
            p_location_info,
        )
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices_raw(
        &self,
        command_buffer: CommandBuffer,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_rendering_input_attachment_indices
            .expect("vkCmdSetRenderingInputAttachmentIndices is not loaded"))(
            command_buffer,
            p_input_attachment_index_info,
        )
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_set_rendering_input_attachment_indices_khr
            .expect("vkCmdSetRenderingInputAttachmentIndicesKHR is not loaded"))(
            command_buffer,
            p_input_attachment_index_info,
        )
    }
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_representative_fragment_test_enable_nv
            .expect("vkCmdSetRepresentativeFragmentTestEnableNV is not loaded"))(
            command_buffer,
            representative_fragment_test_enable,
        )
    }
    pub unsafe fn cmd_set_sample_locations_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_sample_locations_ext
            .expect("vkCmdSetSampleLocationsEXT is not loaded"))(
            command_buffer,
            p_sample_locations_info,
        )
    }
    pub unsafe fn cmd_set_sample_locations_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_sample_locations_enable_ext
            .expect("vkCmdSetSampleLocationsEnableEXT is not loaded"))(
            command_buffer,
            sample_locations_enable,
        )
    }
    pub unsafe fn cmd_set_sample_mask_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        p_sample_mask: *const u32,
    ) {
        (self
            .fp()
            .cmd_set_sample_mask_ext
            .expect("vkCmdSetSampleMaskEXT is not loaded"))(
            command_buffer, samples, p_sample_mask
        )
    }
    pub unsafe fn cmd_set_scissor_raw(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D<'_>,
    ) {
        (self
            .fp()
            .cmd_set_scissor
            .expect("vkCmdSetScissor is not loaded"))(
            command_buffer,
            first_scissor,
            scissor_count,
            p_scissors,
        )
    }
    pub unsafe fn cmd_set_scissor_with_count_raw(
        &self,
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D<'_>,
    ) {
        (self
            .fp()
            .cmd_set_scissor_with_count
            .expect("vkCmdSetScissorWithCount is not loaded"))(
            command_buffer,
            scissor_count,
            p_scissors,
        )
    }
    pub unsafe fn cmd_set_scissor_with_count_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D<'_>,
    ) {
        (self
            .fp()
            .cmd_set_scissor_with_count_ext
            .expect("vkCmdSetScissorWithCountEXT is not loaded"))(
            command_buffer,
            scissor_count,
            p_scissors,
        )
    }
    pub unsafe fn cmd_set_shading_rate_image_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        shading_rate_image_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_shading_rate_image_enable_nv
            .expect("vkCmdSetShadingRateImageEnableNV is not loaded"))(
            command_buffer,
            shading_rate_image_enable,
        )
    }
    pub unsafe fn cmd_set_stencil_compare_mask_raw(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        (self
            .fp()
            .cmd_set_stencil_compare_mask
            .expect("vkCmdSetStencilCompareMask is not loaded"))(
            command_buffer,
            face_mask,
            compare_mask,
        )
    }
    pub unsafe fn cmd_set_stencil_op_raw(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        (self
            .fp()
            .cmd_set_stencil_op
            .expect("vkCmdSetStencilOp is not loaded"))(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    pub unsafe fn cmd_set_stencil_op_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        (self
            .fp()
            .cmd_set_stencil_op_ext
            .expect("vkCmdSetStencilOpEXT is not loaded"))(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    pub unsafe fn cmd_set_stencil_reference_raw(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        (self
            .fp()
            .cmd_set_stencil_reference
            .expect("vkCmdSetStencilReference is not loaded"))(
            command_buffer, face_mask, reference
        )
    }
    pub unsafe fn cmd_set_stencil_test_enable_raw(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_stencil_test_enable
            .expect("vkCmdSetStencilTestEnable is not loaded"))(
            command_buffer, stencil_test_enable
        )
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_stencil_test_enable_ext
            .expect("vkCmdSetStencilTestEnableEXT is not loaded"))(
            command_buffer,
            stencil_test_enable,
        )
    }
    pub unsafe fn cmd_set_stencil_write_mask_raw(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        (self
            .fp()
            .cmd_set_stencil_write_mask
            .expect("vkCmdSetStencilWriteMask is not loaded"))(
            command_buffer,
            face_mask,
            write_mask,
        )
    }
    pub unsafe fn cmd_set_tessellation_domain_origin_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) {
        (self
            .fp()
            .cmd_set_tessellation_domain_origin_ext
            .expect("vkCmdSetTessellationDomainOriginEXT is not loaded"))(
            command_buffer,
            domain_origin,
        )
    }
    pub unsafe fn cmd_set_vertex_input_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
    ) {
        (self
            .fp()
            .cmd_set_vertex_input_ext
            .expect("vkCmdSetVertexInputEXT is not loaded"))(
            command_buffer,
            vertex_binding_description_count,
            p_vertex_binding_descriptions,
            vertex_attribute_description_count,
            p_vertex_attribute_descriptions,
        )
    }
    pub unsafe fn cmd_set_viewport_raw(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport
            .expect("vkCmdSetViewport is not loaded"))(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewports,
        )
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport_shading_rate_palette_nv
            .expect("vkCmdSetViewportShadingRatePaletteNV is not loaded"))(
            command_buffer,
            first_viewport,
            viewport_count,
            p_shading_rate_palettes,
        )
    }
    pub unsafe fn cmd_set_viewport_swizzle_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const ViewportSwizzleNV<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport_swizzle_nv
            .expect("vkCmdSetViewportSwizzleNV is not loaded"))(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewport_swizzles,
        )
    }
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: Bool32,
    ) {
        (self
            .fp()
            .cmd_set_viewport_w_scaling_enable_nv
            .expect("vkCmdSetViewportWScalingEnableNV is not loaded"))(
            command_buffer,
            viewport_w_scaling_enable,
        )
    }
    pub unsafe fn cmd_set_viewport_w_scaling_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_w_scalings: *const ViewportWScalingNV<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport_w_scaling_nv
            .expect("vkCmdSetViewportWScalingNV is not loaded"))(
            command_buffer,
            first_viewport,
            viewport_count,
            p_viewport_w_scalings,
        )
    }
    pub unsafe fn cmd_set_viewport_with_count_raw(
        &self,
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport_with_count
            .expect("vkCmdSetViewportWithCount is not loaded"))(
            command_buffer,
            viewport_count,
            p_viewports,
        )
    }
    pub unsafe fn cmd_set_viewport_with_count_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport<'_>,
    ) {
        (self
            .fp()
            .cmd_set_viewport_with_count_ext
            .expect("vkCmdSetViewportWithCountEXT is not loaded"))(
            command_buffer,
            viewport_count,
            p_viewports,
        )
    }
    pub unsafe fn cmd_subpass_shading_huawei_raw(&self, command_buffer: CommandBuffer) {
        (self
            .fp()
            .cmd_subpass_shading_huawei
            .expect("vkCmdSubpassShadingHUAWEI is not loaded"))(command_buffer)
    }
    pub unsafe fn cmd_trace_rays_indirect2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        indirect_device_address: u64,
    ) {
        (self
            .fp()
            .cmd_trace_rays_indirect2_khr
            .expect("vkCmdTraceRaysIndirect2KHR is not loaded"))(
            command_buffer,
            indirect_device_address,
        )
    }
    pub unsafe fn cmd_trace_rays_indirect_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        indirect_device_address: u64,
    ) {
        (self
            .fp()
            .cmd_trace_rays_indirect_khr
            .expect("vkCmdTraceRaysIndirectKHR is not loaded"))(
            command_buffer,
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            indirect_device_address,
        )
    }
    pub unsafe fn cmd_trace_rays_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR<'_>,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self
            .fp()
            .cmd_trace_rays_khr
            .expect("vkCmdTraceRaysKHR is not loaded"))(
            command_buffer,
            p_raygen_shader_binding_table,
            p_miss_shader_binding_table,
            p_hit_shader_binding_table,
            p_callable_shader_binding_table,
            width,
            height,
            depth,
        )
    }
    pub unsafe fn cmd_trace_rays_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: u64,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: u64,
        miss_shader_binding_stride: u64,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: u64,
        hit_shader_binding_stride: u64,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: u64,
        callable_shader_binding_stride: u64,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        (self
            .fp()
            .cmd_trace_rays_nv
            .expect("vkCmdTraceRaysNV is not loaded"))(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        )
    }
    pub unsafe fn cmd_update_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: u64,
        data_size: u64,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .cmd_update_buffer
            .expect("vkCmdUpdateBuffer is not loaded"))(
            command_buffer,
            dst_buffer,
            dst_offset,
            data_size,
            p_data,
        )
    }
    pub unsafe fn cmd_update_memory_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        data_size: u64,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .cmd_update_memory_khr
            .expect("vkCmdUpdateMemoryKHR is not loaded"))(
            command_buffer,
            p_dst_range,
            dst_flags,
            data_size,
            p_data,
        )
    }
    pub unsafe fn cmd_update_pipeline_indirect_buffer_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        (self
            .fp()
            .cmd_update_pipeline_indirect_buffer_nv
            .expect("vkCmdUpdatePipelineIndirectBufferNV is not loaded"))(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        )
    }
    pub unsafe fn cmd_wait_events_raw(
        &self,
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barrier_count: u32,
        p_memory_barriers: *const MemoryBarrier<'_>,
        buffer_memory_barrier_count: u32,
        p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
        image_memory_barrier_count: u32,
        p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
    ) {
        (self
            .fp()
            .cmd_wait_events
            .expect("vkCmdWaitEvents is not loaded"))(
            command_buffer,
            event_count,
            p_events,
            src_stage_mask,
            dst_stage_mask,
            memory_barrier_count,
            p_memory_barriers,
            buffer_memory_barrier_count,
            p_buffer_memory_barriers,
            image_memory_barrier_count,
            p_image_memory_barriers,
        )
    }
    pub unsafe fn cmd_wait_events2_raw(
        &self,
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_wait_events2
            .expect("vkCmdWaitEvents2 is not loaded"))(
            command_buffer,
            event_count,
            p_events,
            p_dependency_infos,
        )
    }
    pub unsafe fn cmd_wait_events2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    ) {
        (self
            .fp()
            .cmd_wait_events2_khr
            .expect("vkCmdWaitEvents2KHR is not loaded"))(
            command_buffer,
            event_count,
            p_events,
            p_dependency_infos,
        )
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        (self
            .fp()
            .cmd_write_acceleration_structures_properties_khr
            .expect("vkCmdWriteAccelerationStructuresPropertiesKHR is not loaded"))(
            command_buffer,
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_nv_raw(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        (self
            .fp()
            .cmd_write_acceleration_structures_properties_nv
            .expect("vkCmdWriteAccelerationStructuresPropertiesNV is not loaded"))(
            command_buffer,
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_buffer_marker2_amd_raw(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: u64,
        marker: u32,
    ) {
        (self
            .fp()
            .cmd_write_buffer_marker2_amd
            .expect("vkCmdWriteBufferMarker2AMD is not loaded"))(
            command_buffer,
            stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }
    pub unsafe fn cmd_write_buffer_marker_amd_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: u64,
        marker: u32,
    ) {
        (self
            .fp()
            .cmd_write_buffer_marker_amd
            .expect("vkCmdWriteBufferMarkerAMD is not loaded"))(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }
    pub unsafe fn cmd_write_marker_to_memory_amd_raw(
        &self,
        command_buffer: CommandBuffer,
        p_info: *const MemoryMarkerInfoAMD<'_>,
    ) {
        (self
            .fp()
            .cmd_write_marker_to_memory_amd
            .expect("vkCmdWriteMarkerToMemoryAMD is not loaded"))(command_buffer, p_info)
    }
    pub unsafe fn cmd_write_micromaps_properties_ext_raw(
        &self,
        command_buffer: CommandBuffer,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        (self
            .fp()
            .cmd_write_micromaps_properties_ext
            .expect("vkCmdWriteMicromapsPropertiesEXT is not loaded"))(
            command_buffer,
            micromap_count,
            p_micromaps,
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_timestamp_raw(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    ) {
        (self
            .fp()
            .cmd_write_timestamp
            .expect("vkCmdWriteTimestamp is not loaded"))(
            command_buffer,
            pipeline_stage,
            query_pool,
            query,
        )
    }
    pub unsafe fn cmd_write_timestamp2_raw(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        (self
            .fp()
            .cmd_write_timestamp2
            .expect("vkCmdWriteTimestamp2 is not loaded"))(
            command_buffer, stage, query_pool, query
        )
    }
    pub unsafe fn cmd_write_timestamp2_khr_raw(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        (self
            .fp()
            .cmd_write_timestamp2_khr
            .expect("vkCmdWriteTimestamp2KHR is not loaded"))(
            command_buffer,
            stage,
            query_pool,
            query,
        )
    }
    pub unsafe fn compile_deferred_nv_raw(&self, pipeline: Pipeline, shader: u32) -> VkResult {
        (self
            .fp()
            .compile_deferred_nv
            .expect("vkCompileDeferredNV is not loaded"))(self.handle(), pipeline, shader)
    }
    pub unsafe fn convert_cooperative_vector_matrix_nv_raw(
        &self,
        p_info: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .convert_cooperative_vector_matrix_nv
            .expect("vkConvertCooperativeVectorMatrixNV is not loaded"))(
            self.handle(), p_info
        )
    }
    pub unsafe fn copy_acceleration_structure_khr_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_acceleration_structure_khr
            .expect("vkCopyAccelerationStructureKHR is not loaded"))(
            self.handle(),
            deferred_operation,
            p_info,
        )
    }
    pub unsafe fn copy_acceleration_structure_to_memory_khr_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_acceleration_structure_to_memory_khr
            .expect("vkCopyAccelerationStructureToMemoryKHR is not loaded"))(
            self.handle(),
            deferred_operation,
            p_info,
        )
    }
    pub unsafe fn copy_image_to_image_raw(
        &self,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_image_to_image
            .expect("vkCopyImageToImage is not loaded"))(
            self.handle(), p_copy_image_to_image_info
        )
    }
    pub unsafe fn copy_image_to_image_ext_raw(
        &self,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_image_to_image_ext
            .expect("vkCopyImageToImageEXT is not loaded"))(
            self.handle(),
            p_copy_image_to_image_info,
        )
    }
    pub unsafe fn copy_image_to_memory_raw(
        &self,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_image_to_memory
            .expect("vkCopyImageToMemory is not loaded"))(
            self.handle(),
            p_copy_image_to_memory_info,
        )
    }
    pub unsafe fn copy_image_to_memory_ext_raw(
        &self,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_image_to_memory_ext
            .expect("vkCopyImageToMemoryEXT is not loaded"))(
            self.handle(),
            p_copy_image_to_memory_info,
        )
    }
    pub unsafe fn copy_memory_to_acceleration_structure_khr_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_memory_to_acceleration_structure_khr
            .expect("vkCopyMemoryToAccelerationStructureKHR is not loaded"))(
            self.handle(),
            deferred_operation,
            p_info,
        )
    }
    pub unsafe fn copy_memory_to_image_raw(
        &self,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_memory_to_image
            .expect("vkCopyMemoryToImage is not loaded"))(
            self.handle(),
            p_copy_memory_to_image_info,
        )
    }
    pub unsafe fn copy_memory_to_image_ext_raw(
        &self,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_memory_to_image_ext
            .expect("vkCopyMemoryToImageEXT is not loaded"))(
            self.handle(),
            p_copy_memory_to_image_info,
        )
    }
    pub unsafe fn copy_memory_to_micromap_ext_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_memory_to_micromap_ext
            .expect("vkCopyMemoryToMicromapEXT is not loaded"))(
            self.handle(),
            deferred_operation,
            p_info,
        )
    }
    pub unsafe fn copy_micromap_ext_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_micromap_ext
            .expect("vkCopyMicromapEXT is not loaded"))(
            self.handle(), deferred_operation, p_info
        )
    }
    pub unsafe fn copy_micromap_to_memory_ext_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .copy_micromap_to_memory_ext
            .expect("vkCopyMicromapToMemoryEXT is not loaded"))(
            self.handle(),
            deferred_operation,
            p_info,
        )
    }
    pub unsafe fn create_acceleration_structure2_khr_raw(
        &self,
        p_create_info: *const AccelerationStructureCreateInfo2KHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_acceleration_structure2_khr
            .expect("vkCreateAccelerationStructure2KHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_acceleration_structure,
        )
    }
    pub unsafe fn create_acceleration_structure_khr_raw(
        &self,
        p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_acceleration_structure_khr
            .expect("vkCreateAccelerationStructureKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_acceleration_structure,
        )
    }
    pub unsafe fn create_acceleration_structure_nv_raw(
        &self,
        p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> VkResult {
        (self
            .fp()
            .create_acceleration_structure_nv
            .expect("vkCreateAccelerationStructureNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_acceleration_structure,
        )
    }
    pub unsafe fn create_buffer_raw(
        &self,
        p_create_info: *const BufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_buffer: *mut Buffer,
    ) -> VkResult {
        (self
            .fp()
            .create_buffer
            .expect("vkCreateBuffer is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_buffer,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn create_buffer_collection_fuchsia_raw(
        &self,
        p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_collection: *mut BufferCollectionFUCHSIA,
    ) -> VkResult {
        (self
            .fp()
            .create_buffer_collection_fuchsia
            .expect("vkCreateBufferCollectionFUCHSIA is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_collection,
        )
    }
    pub unsafe fn create_buffer_view_raw(
        &self,
        p_create_info: *const BufferViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut BufferView,
    ) -> VkResult {
        (self
            .fp()
            .create_buffer_view
            .expect("vkCreateBufferView is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_view,
        )
    }
    pub unsafe fn create_command_pool_raw(
        &self,
        p_create_info: *const CommandPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_command_pool: *mut CommandPool,
    ) -> VkResult {
        (self
            .fp()
            .create_command_pool
            .expect("vkCreateCommandPool is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_command_pool,
        )
    }
    pub unsafe fn create_compute_pipelines_raw(
        &self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_compute_pipelines
            .expect("vkCreateComputePipelines is not loaded"))(
            self.handle(),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_cu_function_nvx_raw(
        &self,
        p_create_info: *const CuFunctionCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CuFunctionNVX,
    ) -> VkResult {
        (self
            .fp()
            .create_cu_function_nvx
            .expect("vkCreateCuFunctionNVX is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_function,
        )
    }
    pub unsafe fn create_cu_module_nvx_raw(
        &self,
        p_create_info: *const CuModuleCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CuModuleNVX,
    ) -> VkResult {
        (self
            .fp()
            .create_cu_module_nvx
            .expect("vkCreateCuModuleNVX is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_module,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn create_cuda_function_nv_raw(
        &self,
        p_create_info: *const CudaFunctionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CudaFunctionNV,
    ) -> VkResult {
        (self
            .fp()
            .create_cuda_function_nv
            .expect("vkCreateCudaFunctionNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_function,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn create_cuda_module_nv_raw(
        &self,
        p_create_info: *const CudaModuleCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CudaModuleNV,
    ) -> VkResult {
        (self
            .fp()
            .create_cuda_module_nv
            .expect("vkCreateCudaModuleNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_module,
        )
    }
    pub unsafe fn create_data_graph_pipeline_session_arm_raw(
        &self,
        p_create_info: *const DataGraphPipelineSessionCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut DataGraphPipelineSessionARM,
    ) -> VkResult {
        (self
            .fp()
            .create_data_graph_pipeline_session_arm
            .expect("vkCreateDataGraphPipelineSessionARM is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_session,
        )
    }
    pub unsafe fn create_data_graph_pipelines_arm_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const DataGraphPipelineCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_data_graph_pipelines_arm
            .expect("vkCreateDataGraphPipelinesARM is not loaded"))(
            self.handle(),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_deferred_operation_khr_raw(
        &self,
        p_allocator: *const AllocationCallbacks<'_>,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_deferred_operation_khr
            .expect("vkCreateDeferredOperationKHR is not loaded"))(
            self.handle(),
            p_allocator,
            p_deferred_operation,
        )
    }
    pub unsafe fn create_descriptor_pool_raw(
        &self,
        p_create_info: *const DescriptorPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> VkResult {
        (self
            .fp()
            .create_descriptor_pool
            .expect("vkCreateDescriptorPool is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_descriptor_pool,
        )
    }
    pub unsafe fn create_descriptor_set_layout_raw(
        &self,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> VkResult {
        (self
            .fp()
            .create_descriptor_set_layout
            .expect("vkCreateDescriptorSetLayout is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_set_layout,
        )
    }
    pub unsafe fn create_descriptor_update_template_raw(
        &self,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> VkResult {
        (self
            .fp()
            .create_descriptor_update_template
            .expect("vkCreateDescriptorUpdateTemplate is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_descriptor_update_template,
        )
    }
    pub unsafe fn create_descriptor_update_template_khr_raw(
        &self,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> VkResult {
        (self
            .fp()
            .create_descriptor_update_template_khr
            .expect("vkCreateDescriptorUpdateTemplateKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_descriptor_update_template,
        )
    }
    pub unsafe fn create_event_raw(
        &self,
        p_create_info: *const EventCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_event: *mut Event,
    ) -> VkResult {
        (self.fp().create_event.expect("vkCreateEvent is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_event,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn create_execution_graph_pipelines_amdx_raw(
        &self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_execution_graph_pipelines_amdx
            .expect("vkCreateExecutionGraphPipelinesAMDX is not loaded"))(
            self.handle(),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_external_compute_queue_nv_raw(
        &self,
        p_create_info: *const ExternalComputeQueueCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_external_queue: *mut ExternalComputeQueueNV,
    ) -> VkResult {
        (self
            .fp()
            .create_external_compute_queue_nv
            .expect("vkCreateExternalComputeQueueNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_external_queue,
        )
    }
    pub unsafe fn create_fence_raw(
        &self,
        p_create_info: *const FenceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult {
        (self.fp().create_fence.expect("vkCreateFence is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_fence,
        )
    }
    pub unsafe fn create_framebuffer_raw(
        &self,
        p_create_info: *const FramebufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_framebuffer: *mut Framebuffer,
    ) -> VkResult {
        (self
            .fp()
            .create_framebuffer
            .expect("vkCreateFramebuffer is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_framebuffer,
        )
    }
    pub unsafe fn create_graphics_pipelines_raw(
        &self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_graphics_pipelines
            .expect("vkCreateGraphicsPipelines is not loaded"))(
            self.handle(),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_image_raw(
        &self,
        p_create_info: *const ImageCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_image: *mut Image,
    ) -> VkResult {
        (self.fp().create_image.expect("vkCreateImage is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_image,
        )
    }
    pub unsafe fn create_image_view_raw(
        &self,
        p_create_info: *const ImageViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut ImageView,
    ) -> VkResult {
        (self
            .fp()
            .create_image_view
            .expect("vkCreateImageView is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_view,
        )
    }
    pub unsafe fn create_indirect_commands_layout_ext_raw(
        &self,
        p_create_info: *const IndirectCommandsLayoutCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_indirect_commands_layout_ext
            .expect("vkCreateIndirectCommandsLayoutEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_indirect_commands_layout,
        )
    }
    pub unsafe fn create_indirect_commands_layout_nv_raw(
        &self,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> VkResult {
        (self
            .fp()
            .create_indirect_commands_layout_nv
            .expect("vkCreateIndirectCommandsLayoutNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_indirect_commands_layout,
        )
    }
    pub unsafe fn create_indirect_execution_set_ext_raw(
        &self,
        p_create_info: *const IndirectExecutionSetCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_execution_set: *mut IndirectExecutionSetEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_indirect_execution_set_ext
            .expect("vkCreateIndirectExecutionSetEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_indirect_execution_set,
        )
    }
    pub unsafe fn create_micromap_ext_raw(
        &self,
        p_create_info: *const MicromapCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_micromap: *mut MicromapEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_micromap_ext
            .expect("vkCreateMicromapEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_micromap,
        )
    }
    pub unsafe fn create_optical_flow_session_nv_raw(
        &self,
        p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut OpticalFlowSessionNV,
    ) -> VkResult {
        (self
            .fp()
            .create_optical_flow_session_nv
            .expect("vkCreateOpticalFlowSessionNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_session,
        )
    }
    pub unsafe fn create_pipeline_binaries_khr_raw(
        &self,
        p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .create_pipeline_binaries_khr
            .expect("vkCreatePipelineBinariesKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_binaries,
        )
    }
    pub unsafe fn create_pipeline_cache_raw(
        &self,
        p_create_info: *const PipelineCacheCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_cache: *mut PipelineCache,
    ) -> VkResult {
        (self
            .fp()
            .create_pipeline_cache
            .expect("vkCreatePipelineCache is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_pipeline_cache,
        )
    }
    pub unsafe fn create_pipeline_layout_raw(
        &self,
        p_create_info: *const PipelineLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> VkResult {
        (self
            .fp()
            .create_pipeline_layout
            .expect("vkCreatePipelineLayout is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_pipeline_layout,
        )
    }
    pub unsafe fn create_private_data_slot_raw(
        &self,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> VkResult {
        (self
            .fp()
            .create_private_data_slot
            .expect("vkCreatePrivateDataSlot is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_private_data_slot,
        )
    }
    pub unsafe fn create_private_data_slot_ext_raw(
        &self,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> VkResult {
        (self
            .fp()
            .create_private_data_slot_ext
            .expect("vkCreatePrivateDataSlotEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_private_data_slot,
        )
    }
    pub unsafe fn create_query_pool_raw(
        &self,
        p_create_info: *const QueryPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_query_pool: *mut QueryPool,
    ) -> VkResult {
        (self
            .fp()
            .create_query_pool
            .expect("vkCreateQueryPool is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_query_pool,
        )
    }
    pub unsafe fn create_ray_tracing_pipelines_khr_raw(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_ray_tracing_pipelines_khr
            .expect("vkCreateRayTracingPipelinesKHR is not loaded"))(
            self.handle(),
            deferred_operation,
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_ray_tracing_pipelines_nv_raw(
        &self,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult {
        (self
            .fp()
            .create_ray_tracing_pipelines_nv
            .expect("vkCreateRayTracingPipelinesNV is not loaded"))(
            self.handle(),
            pipeline_cache,
            create_info_count,
            p_create_infos,
            p_allocator,
            p_pipelines,
        )
    }
    pub unsafe fn create_render_pass_raw(
        &self,
        p_create_info: *const RenderPassCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult {
        (self
            .fp()
            .create_render_pass
            .expect("vkCreateRenderPass is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_render_pass,
        )
    }
    pub unsafe fn create_render_pass2_raw(
        &self,
        p_create_info: *const RenderPassCreateInfo2<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult {
        (self
            .fp()
            .create_render_pass2
            .expect("vkCreateRenderPass2 is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_render_pass,
        )
    }
    pub unsafe fn create_render_pass2_khr_raw(
        &self,
        p_create_info: *const RenderPassCreateInfo2<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult {
        (self
            .fp()
            .create_render_pass2_khr
            .expect("vkCreateRenderPass2KHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_render_pass,
        )
    }
    pub unsafe fn create_sampler_raw(
        &self,
        p_create_info: *const SamplerCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_sampler: *mut Sampler,
    ) -> VkResult {
        (self
            .fp()
            .create_sampler
            .expect("vkCreateSampler is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_sampler,
        )
    }
    pub unsafe fn create_sampler_ycbcr_conversion_raw(
        &self,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> VkResult {
        (self
            .fp()
            .create_sampler_ycbcr_conversion
            .expect("vkCreateSamplerYcbcrConversion is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_ycbcr_conversion,
        )
    }
    pub unsafe fn create_sampler_ycbcr_conversion_khr_raw(
        &self,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> VkResult {
        (self
            .fp()
            .create_sampler_ycbcr_conversion_khr
            .expect("vkCreateSamplerYcbcrConversionKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_ycbcr_conversion,
        )
    }
    pub unsafe fn create_semaphore_raw(
        &self,
        p_create_info: *const SemaphoreCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_semaphore: *mut Semaphore,
    ) -> VkResult {
        (self
            .fp()
            .create_semaphore
            .expect("vkCreateSemaphore is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_semaphore,
        )
    }
    pub unsafe fn create_shader_instrumentation_arm_raw(
        &self,
        p_create_info: *const ShaderInstrumentationCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instrumentation: *mut ShaderInstrumentationARM,
    ) -> VkResult {
        (self
            .fp()
            .create_shader_instrumentation_arm
            .expect("vkCreateShaderInstrumentationARM is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_instrumentation,
        )
    }
    pub unsafe fn create_shader_module_raw(
        &self,
        p_create_info: *const ShaderModuleCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shader_module: *mut ShaderModule,
    ) -> VkResult {
        (self
            .fp()
            .create_shader_module
            .expect("vkCreateShaderModule is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_shader_module,
        )
    }
    pub unsafe fn create_shaders_ext_raw(
        &self,
        create_info_count: u32,
        p_create_infos: *const ShaderCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shaders: *mut ShaderEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_shaders_ext
            .expect("vkCreateShadersEXT is not loaded"))(
            self.handle(),
            create_info_count,
            p_create_infos,
            p_allocator,
            p_shaders,
        )
    }
    pub unsafe fn create_shared_swapchains_khr_raw(
        &self,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchains: *mut SwapchainKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_shared_swapchains_khr
            .expect("vkCreateSharedSwapchainsKHR is not loaded"))(
            self.handle(),
            swapchain_count,
            p_create_infos,
            p_allocator,
            p_swapchains,
        )
    }
    pub unsafe fn create_swapchain_khr_raw(
        &self,
        p_create_info: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchain: *mut SwapchainKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_swapchain_khr
            .expect("vkCreateSwapchainKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_swapchain,
        )
    }
    pub unsafe fn create_tensor_arm_raw(
        &self,
        p_create_info: *const TensorCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_tensor: *mut TensorARM,
    ) -> VkResult {
        (self
            .fp()
            .create_tensor_arm
            .expect("vkCreateTensorARM is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_tensor,
        )
    }
    pub unsafe fn create_tensor_view_arm_raw(
        &self,
        p_create_info: *const TensorViewCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut TensorViewARM,
    ) -> VkResult {
        (self
            .fp()
            .create_tensor_view_arm
            .expect("vkCreateTensorViewARM is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_view,
        )
    }
    pub unsafe fn create_validation_cache_ext_raw(
        &self,
        p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> VkResult {
        (self
            .fp()
            .create_validation_cache_ext
            .expect("vkCreateValidationCacheEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_validation_cache,
        )
    }
    pub unsafe fn create_video_session_khr_raw(
        &self,
        p_create_info: *const VideoSessionCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session: *mut VideoSessionKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_video_session_khr
            .expect("vkCreateVideoSessionKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_video_session,
        )
    }
    pub unsafe fn create_video_session_parameters_khr_raw(
        &self,
        p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> VkResult {
        (self
            .fp()
            .create_video_session_parameters_khr
            .expect("vkCreateVideoSessionParametersKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_allocator,
            p_video_session_parameters,
        )
    }
    pub unsafe fn debug_marker_set_object_name_ext_raw(
        &self,
        p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .debug_marker_set_object_name_ext
            .expect("vkDebugMarkerSetObjectNameEXT is not loaded"))(
            self.handle(), p_name_info
        )
    }
    pub unsafe fn debug_marker_set_object_tag_ext_raw(
        &self,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .debug_marker_set_object_tag_ext
            .expect("vkDebugMarkerSetObjectTagEXT is not loaded"))(self.handle(), p_tag_info)
    }
    pub unsafe fn deferred_operation_join_khr_raw(
        &self,
        operation: DeferredOperationKHR,
    ) -> VkResult {
        (self
            .fp()
            .deferred_operation_join_khr
            .expect("vkDeferredOperationJoinKHR is not loaded"))(self.handle(), operation)
    }
    pub unsafe fn destroy_acceleration_structure_khr_raw(
        &self,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_acceleration_structure_khr
            .expect("vkDestroyAccelerationStructureKHR is not loaded"))(
            self.handle(),
            acceleration_structure,
            p_allocator,
        )
    }
    pub unsafe fn destroy_acceleration_structure_nv_raw(
        &self,
        acceleration_structure: AccelerationStructureNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_acceleration_structure_nv
            .expect("vkDestroyAccelerationStructureNV is not loaded"))(
            self.handle(),
            acceleration_structure,
            p_allocator,
        )
    }
    pub unsafe fn destroy_buffer_raw(
        &self,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_buffer
            .expect("vkDestroyBuffer is not loaded"))(self.handle(), buffer, p_allocator)
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn destroy_buffer_collection_fuchsia_raw(
        &self,
        collection: BufferCollectionFUCHSIA,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_buffer_collection_fuchsia
            .expect("vkDestroyBufferCollectionFUCHSIA is not loaded"))(
            self.handle(),
            collection,
            p_allocator,
        )
    }
    pub unsafe fn destroy_buffer_view_raw(
        &self,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_buffer_view
            .expect("vkDestroyBufferView is not loaded"))(
            self.handle(), buffer_view, p_allocator
        )
    }
    pub unsafe fn destroy_command_pool_raw(
        &self,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_command_pool
            .expect("vkDestroyCommandPool is not loaded"))(
            self.handle(), command_pool, p_allocator
        )
    }
    pub unsafe fn destroy_cu_function_nvx_raw(
        &self,
        function: CuFunctionNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_cu_function_nvx
            .expect("vkDestroyCuFunctionNVX is not loaded"))(
            self.handle(), function, p_allocator
        )
    }
    pub unsafe fn destroy_cu_module_nvx_raw(
        &self,
        module: CuModuleNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_cu_module_nvx
            .expect("vkDestroyCuModuleNVX is not loaded"))(
            self.handle(), module, p_allocator
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn destroy_cuda_function_nv_raw(
        &self,
        function: CudaFunctionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_cuda_function_nv
            .expect("vkDestroyCudaFunctionNV is not loaded"))(
            self.handle(), function, p_allocator
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn destroy_cuda_module_nv_raw(
        &self,
        module: CudaModuleNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_cuda_module_nv
            .expect("vkDestroyCudaModuleNV is not loaded"))(
            self.handle(), module, p_allocator
        )
    }
    pub unsafe fn destroy_data_graph_pipeline_session_arm_raw(
        &self,
        session: DataGraphPipelineSessionARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_data_graph_pipeline_session_arm
            .expect("vkDestroyDataGraphPipelineSessionARM is not loaded"))(
            self.handle(),
            session,
            p_allocator,
        )
    }
    pub unsafe fn destroy_deferred_operation_khr_raw(
        &self,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_deferred_operation_khr
            .expect("vkDestroyDeferredOperationKHR is not loaded"))(
            self.handle(),
            operation,
            p_allocator,
        )
    }
    pub unsafe fn destroy_descriptor_pool_raw(
        &self,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_descriptor_pool
            .expect("vkDestroyDescriptorPool is not loaded"))(
            self.handle(),
            descriptor_pool,
            p_allocator,
        )
    }
    pub unsafe fn destroy_descriptor_set_layout_raw(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_descriptor_set_layout
            .expect("vkDestroyDescriptorSetLayout is not loaded"))(
            self.handle(),
            descriptor_set_layout,
            p_allocator,
        )
    }
    pub unsafe fn destroy_descriptor_update_template_raw(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_descriptor_update_template
            .expect("vkDestroyDescriptorUpdateTemplate is not loaded"))(
            self.handle(),
            descriptor_update_template,
            p_allocator,
        )
    }
    pub unsafe fn destroy_descriptor_update_template_khr_raw(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_descriptor_update_template_khr
            .expect("vkDestroyDescriptorUpdateTemplateKHR is not loaded"))(
            self.handle(),
            descriptor_update_template,
            p_allocator,
        )
    }
    pub unsafe fn destroy_device_raw(&self, p_allocator: *const AllocationCallbacks<'_>) {
        (self
            .fp()
            .destroy_device
            .expect("vkDestroyDevice is not loaded"))(self.handle(), p_allocator)
    }
    pub unsafe fn destroy_event_raw(
        &self,
        event: Event,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_event
            .expect("vkDestroyEvent is not loaded"))(self.handle(), event, p_allocator)
    }
    pub unsafe fn destroy_external_compute_queue_nv_raw(
        &self,
        external_queue: ExternalComputeQueueNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_external_compute_queue_nv
            .expect("vkDestroyExternalComputeQueueNV is not loaded"))(
            self.handle(),
            external_queue,
            p_allocator,
        )
    }
    pub unsafe fn destroy_fence_raw(
        &self,
        fence: Fence,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_fence
            .expect("vkDestroyFence is not loaded"))(self.handle(), fence, p_allocator)
    }
    pub unsafe fn destroy_framebuffer_raw(
        &self,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_framebuffer
            .expect("vkDestroyFramebuffer is not loaded"))(
            self.handle(), framebuffer, p_allocator
        )
    }
    pub unsafe fn destroy_image_raw(
        &self,
        image: Image,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_image
            .expect("vkDestroyImage is not loaded"))(self.handle(), image, p_allocator)
    }
    pub unsafe fn destroy_image_view_raw(
        &self,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_image_view
            .expect("vkDestroyImageView is not loaded"))(
            self.handle(), image_view, p_allocator
        )
    }
    pub unsafe fn destroy_indirect_commands_layout_ext_raw(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_indirect_commands_layout_ext
            .expect("vkDestroyIndirectCommandsLayoutEXT is not loaded"))(
            self.handle(),
            indirect_commands_layout,
            p_allocator,
        )
    }
    pub unsafe fn destroy_indirect_commands_layout_nv_raw(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_indirect_commands_layout_nv
            .expect("vkDestroyIndirectCommandsLayoutNV is not loaded"))(
            self.handle(),
            indirect_commands_layout,
            p_allocator,
        )
    }
    pub unsafe fn destroy_indirect_execution_set_ext_raw(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_indirect_execution_set_ext
            .expect("vkDestroyIndirectExecutionSetEXT is not loaded"))(
            self.handle(),
            indirect_execution_set,
            p_allocator,
        )
    }
    pub unsafe fn destroy_micromap_ext_raw(
        &self,
        micromap: MicromapEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_micromap_ext
            .expect("vkDestroyMicromapEXT is not loaded"))(
            self.handle(), micromap, p_allocator
        )
    }
    pub unsafe fn destroy_optical_flow_session_nv_raw(
        &self,
        session: OpticalFlowSessionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_optical_flow_session_nv
            .expect("vkDestroyOpticalFlowSessionNV is not loaded"))(
            self.handle(),
            session,
            p_allocator,
        )
    }
    pub unsafe fn destroy_pipeline_raw(
        &self,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_pipeline
            .expect("vkDestroyPipeline is not loaded"))(self.handle(), pipeline, p_allocator)
    }
    pub unsafe fn destroy_pipeline_binary_khr_raw(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_pipeline_binary_khr
            .expect("vkDestroyPipelineBinaryKHR is not loaded"))(
            self.handle(),
            pipeline_binary,
            p_allocator,
        )
    }
    pub unsafe fn destroy_pipeline_cache_raw(
        &self,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_pipeline_cache
            .expect("vkDestroyPipelineCache is not loaded"))(
            self.handle(),
            pipeline_cache,
            p_allocator,
        )
    }
    pub unsafe fn destroy_pipeline_layout_raw(
        &self,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_pipeline_layout
            .expect("vkDestroyPipelineLayout is not loaded"))(
            self.handle(),
            pipeline_layout,
            p_allocator,
        )
    }
    pub unsafe fn destroy_private_data_slot_raw(
        &self,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_private_data_slot
            .expect("vkDestroyPrivateDataSlot is not loaded"))(
            self.handle(),
            private_data_slot,
            p_allocator,
        )
    }
    pub unsafe fn destroy_private_data_slot_ext_raw(
        &self,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_private_data_slot_ext
            .expect("vkDestroyPrivateDataSlotEXT is not loaded"))(
            self.handle(),
            private_data_slot,
            p_allocator,
        )
    }
    pub unsafe fn destroy_query_pool_raw(
        &self,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_query_pool
            .expect("vkDestroyQueryPool is not loaded"))(
            self.handle(), query_pool, p_allocator
        )
    }
    pub unsafe fn destroy_render_pass_raw(
        &self,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_render_pass
            .expect("vkDestroyRenderPass is not loaded"))(
            self.handle(), render_pass, p_allocator
        )
    }
    pub unsafe fn destroy_sampler_raw(
        &self,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_sampler
            .expect("vkDestroySampler is not loaded"))(self.handle(), sampler, p_allocator)
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_raw(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_sampler_ycbcr_conversion
            .expect("vkDestroySamplerYcbcrConversion is not loaded"))(
            self.handle(),
            ycbcr_conversion,
            p_allocator,
        )
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr_raw(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_sampler_ycbcr_conversion_khr
            .expect("vkDestroySamplerYcbcrConversionKHR is not loaded"))(
            self.handle(),
            ycbcr_conversion,
            p_allocator,
        )
    }
    pub unsafe fn destroy_semaphore_raw(
        &self,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_semaphore
            .expect("vkDestroySemaphore is not loaded"))(
            self.handle(), semaphore, p_allocator
        )
    }
    pub unsafe fn destroy_shader_ext_raw(
        &self,
        shader: ShaderEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_shader_ext
            .expect("vkDestroyShaderEXT is not loaded"))(self.handle(), shader, p_allocator)
    }
    pub unsafe fn destroy_shader_instrumentation_arm_raw(
        &self,
        instrumentation: ShaderInstrumentationARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_shader_instrumentation_arm
            .expect("vkDestroyShaderInstrumentationARM is not loaded"))(
            self.handle(),
            instrumentation,
            p_allocator,
        )
    }
    pub unsafe fn destroy_shader_module_raw(
        &self,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_shader_module
            .expect("vkDestroyShaderModule is not loaded"))(
            self.handle(),
            shader_module,
            p_allocator,
        )
    }
    pub unsafe fn destroy_swapchain_khr_raw(
        &self,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_swapchain_khr
            .expect("vkDestroySwapchainKHR is not loaded"))(
            self.handle(), swapchain, p_allocator
        )
    }
    pub unsafe fn destroy_tensor_arm_raw(
        &self,
        tensor: TensorARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_tensor_arm
            .expect("vkDestroyTensorARM is not loaded"))(self.handle(), tensor, p_allocator)
    }
    pub unsafe fn destroy_tensor_view_arm_raw(
        &self,
        tensor_view: TensorViewARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_tensor_view_arm
            .expect("vkDestroyTensorViewARM is not loaded"))(
            self.handle(),
            tensor_view,
            p_allocator,
        )
    }
    pub unsafe fn destroy_validation_cache_ext_raw(
        &self,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_validation_cache_ext
            .expect("vkDestroyValidationCacheEXT is not loaded"))(
            self.handle(),
            validation_cache,
            p_allocator,
        )
    }
    pub unsafe fn destroy_video_session_khr_raw(
        &self,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_video_session_khr
            .expect("vkDestroyVideoSessionKHR is not loaded"))(
            self.handle(),
            video_session,
            p_allocator,
        )
    }
    pub unsafe fn destroy_video_session_parameters_khr_raw(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self
            .fp()
            .destroy_video_session_parameters_khr
            .expect("vkDestroyVideoSessionParametersKHR is not loaded"))(
            self.handle(),
            video_session_parameters,
            p_allocator,
        )
    }
    pub unsafe fn device_wait_idle_raw(&self) -> VkResult {
        (self
            .fp()
            .device_wait_idle
            .expect("vkDeviceWaitIdle is not loaded"))(self.handle())
    }
    pub unsafe fn display_power_control_ext_raw(
        &self,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .display_power_control_ext
            .expect("vkDisplayPowerControlEXT is not loaded"))(
            self.handle(),
            display,
            p_display_power_info,
        )
    }
    pub unsafe fn end_command_buffer_raw(&self, command_buffer: CommandBuffer) -> VkResult {
        (self
            .fp()
            .end_command_buffer
            .expect("vkEndCommandBuffer is not loaded"))(command_buffer)
    }
    #[cfg(feature = "metal")]
    pub unsafe fn export_metal_objects_ext_raw(
        &self,
        p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
    ) {
        (self
            .fp()
            .export_metal_objects_ext
            .expect("vkExportMetalObjectsEXT is not loaded"))(
            self.handle(), p_metal_objects_info
        )
    }
    pub unsafe fn flush_mapped_memory_ranges_raw(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> VkResult {
        (self
            .fp()
            .flush_mapped_memory_ranges
            .expect("vkFlushMappedMemoryRanges is not loaded"))(
            self.handle(),
            memory_range_count,
            p_memory_ranges,
        )
    }
    pub unsafe fn free_command_buffers_raw(
        &self,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ) {
        (self
            .fp()
            .free_command_buffers
            .expect("vkFreeCommandBuffers is not loaded"))(
            self.handle(),
            command_pool,
            command_buffer_count,
            p_command_buffers,
        )
    }
    pub unsafe fn free_descriptor_sets_raw(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> VkResult {
        (self
            .fp()
            .free_descriptor_sets
            .expect("vkFreeDescriptorSets is not loaded"))(
            self.handle(),
            descriptor_pool,
            descriptor_set_count,
            p_descriptor_sets,
        )
    }
    pub unsafe fn free_memory_raw(
        &self,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks<'_>,
    ) {
        (self.fp().free_memory.expect("vkFreeMemory is not loaded"))(
            self.handle(),
            memory,
            p_allocator,
        )
    }
    pub unsafe fn get_acceleration_structure_build_sizes_khr_raw(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        (self
            .fp()
            .get_acceleration_structure_build_sizes_khr
            .expect("vkGetAccelerationStructureBuildSizesKHR is not loaded"))(
            self.handle(),
            build_type,
            p_build_info,
            p_max_primitive_counts,
            p_size_info,
        )
    }
    pub unsafe fn get_acceleration_structure_device_address_khr_raw(
        &self,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_acceleration_structure_device_address_khr
            .expect("vkGetAccelerationStructureDeviceAddressKHR is not loaded"))(
            self.handle(),
            p_info,
        )
    }
    pub unsafe fn get_acceleration_structure_handle_nv_raw(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_acceleration_structure_handle_nv
            .expect("vkGetAccelerationStructureHandleNV is not loaded"))(
            self.handle(),
            acceleration_structure,
            data_size,
            p_data,
        )
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nv_raw(
        &self,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_acceleration_structure_memory_requirements_nv
            .expect("vkGetAccelerationStructureMemoryRequirementsNV is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_acceleration_structure_opaque_capture_descriptor_data_ext_raw(
        &self,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_acceleration_structure_opaque_capture_descriptor_data_ext
            .expect("vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    #[cfg(feature = "android")]
    pub unsafe fn get_android_hardware_buffer_properties_android_raw(
        &self,
        buffer: *const AHardwareBuffer,
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_android_hardware_buffer_properties_android
            .expect("vkGetAndroidHardwareBufferPropertiesANDROID is not loaded"))(
            self.handle(),
            buffer,
            p_properties,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_buffer_collection_properties_fuchsia_raw(
        &self,
        collection: BufferCollectionFUCHSIA,
        p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_buffer_collection_properties_fuchsia
            .expect("vkGetBufferCollectionPropertiesFUCHSIA is not loaded"))(
            self.handle(),
            collection,
            p_properties,
        )
    }
    pub unsafe fn get_buffer_device_address_raw(
        &self,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_buffer_device_address
            .expect("vkGetBufferDeviceAddress is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_buffer_device_address_ext_raw(
        &self,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_buffer_device_address_ext
            .expect("vkGetBufferDeviceAddressEXT is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_buffer_device_address_khr_raw(
        &self,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_buffer_device_address_khr
            .expect("vkGetBufferDeviceAddressKHR is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_buffer_memory_requirements_raw(
        &self,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements<'_>,
    ) {
        (self
            .fp()
            .get_buffer_memory_requirements
            .expect("vkGetBufferMemoryRequirements is not loaded"))(
            self.handle(),
            buffer,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_buffer_memory_requirements2_raw(
        &self,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_buffer_memory_requirements2
            .expect("vkGetBufferMemoryRequirements2 is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_buffer_memory_requirements2_khr_raw(
        &self,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_buffer_memory_requirements2_khr
            .expect("vkGetBufferMemoryRequirements2KHR is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_buffer_opaque_capture_address_raw(
        &self,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_buffer_opaque_capture_address
            .expect("vkGetBufferOpaqueCaptureAddress is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_buffer_opaque_capture_address_khr_raw(
        &self,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_buffer_opaque_capture_address_khr
            .expect("vkGetBufferOpaqueCaptureAddressKHR is not loaded"))(
            self.handle(), p_info
        )
    }
    pub unsafe fn get_buffer_opaque_capture_descriptor_data_ext_raw(
        &self,
        p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_buffer_opaque_capture_descriptor_data_ext
            .expect("vkGetBufferOpaqueCaptureDescriptorDataEXT is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    pub unsafe fn get_calibrated_timestamps_ext_raw(
        &self,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_calibrated_timestamps_ext
            .expect("vkGetCalibratedTimestampsEXT is not loaded"))(
            self.handle(),
            timestamp_count,
            p_timestamp_infos,
            p_timestamps,
            p_max_deviation,
        )
    }
    pub unsafe fn get_calibrated_timestamps_khr_raw(
        &self,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_calibrated_timestamps_khr
            .expect("vkGetCalibratedTimestampsKHR is not loaded"))(
            self.handle(),
            timestamp_count,
            p_timestamp_infos,
            p_timestamps,
            p_max_deviation,
        )
    }
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv_raw(
        &self,
        p_info: *const ClusterAccelerationStructureInputInfoNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        (self
            .fp()
            .get_cluster_acceleration_structure_build_sizes_nv
            .expect("vkGetClusterAccelerationStructureBuildSizesNV is not loaded"))(
            self.handle(),
            p_info,
            p_size_info,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn get_cuda_module_cache_nv_raw(
        &self,
        module: CudaModuleNV,
        p_cache_size: *mut usize,
        p_cache_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_cuda_module_cache_nv
            .expect("vkGetCudaModuleCacheNV is not loaded"))(
            self.handle(),
            module,
            p_cache_size,
            p_cache_data,
        )
    }
    pub unsafe fn get_data_graph_pipeline_available_properties_arm_raw(
        &self,
        p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
        p_properties_count: *mut u32,
        p_properties: *mut DataGraphPipelinePropertyARM,
    ) -> VkResult {
        (self
            .fp()
            .get_data_graph_pipeline_available_properties_arm
            .expect("vkGetDataGraphPipelineAvailablePropertiesARM is not loaded"))(
            self.handle(),
            p_pipeline_info,
            p_properties_count,
            p_properties,
        )
    }
    pub unsafe fn get_data_graph_pipeline_properties_arm_raw(
        &self,
        p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
        properties_count: u32,
        p_properties: *mut DataGraphPipelinePropertyQueryResultARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_data_graph_pipeline_properties_arm
            .expect("vkGetDataGraphPipelinePropertiesARM is not loaded"))(
            self.handle(),
            p_pipeline_info,
            properties_count,
            p_properties,
        )
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm_raw(
        &self,
        p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
        p_bind_point_requirement_count: *mut u32,
        p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_data_graph_pipeline_session_bind_point_requirements_arm
            .expect("vkGetDataGraphPipelineSessionBindPointRequirementsARM is not loaded"))(
            self.handle(),
            p_info,
            p_bind_point_requirement_count,
            p_bind_point_requirements,
        )
    }
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm_raw(
        &self,
        p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_data_graph_pipeline_session_memory_requirements_arm
            .expect("vkGetDataGraphPipelineSessionMemoryRequirementsARM is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_deferred_operation_max_concurrency_khr_raw(
        &self,
        operation: DeferredOperationKHR,
    ) -> u32 {
        (self
            .fp()
            .get_deferred_operation_max_concurrency_khr
            .expect("vkGetDeferredOperationMaxConcurrencyKHR is not loaded"))(
            self.handle(),
            operation,
        )
    }
    pub unsafe fn get_deferred_operation_result_khr_raw(
        &self,
        operation: DeferredOperationKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_deferred_operation_result_khr
            .expect("vkGetDeferredOperationResultKHR is not loaded"))(
            self.handle(), operation
        )
    }
    pub unsafe fn get_descriptor_ext_raw(
        &self,
        p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
        data_size: usize,
        p_descriptor: *mut c_void,
    ) {
        (self
            .fp()
            .get_descriptor_ext
            .expect("vkGetDescriptorEXT is not loaded"))(
            self.handle(),
            p_descriptor_info,
            data_size,
            p_descriptor,
        )
    }
    pub unsafe fn get_descriptor_set_host_mapping_valve_raw(
        &self,
        descriptor_set: DescriptorSet,
        pp_data: *mut *mut c_void,
    ) {
        (self
            .fp()
            .get_descriptor_set_host_mapping_valve
            .expect("vkGetDescriptorSetHostMappingVALVE is not loaded"))(
            self.handle(),
            descriptor_set,
            pp_data,
        )
    }
    pub unsafe fn get_descriptor_set_layout_binding_offset_ext_raw(
        &self,
        layout: DescriptorSetLayout,
        binding: u32,
        p_offset: *mut u64,
    ) {
        (self
            .fp()
            .get_descriptor_set_layout_binding_offset_ext
            .expect("vkGetDescriptorSetLayoutBindingOffsetEXT is not loaded"))(
            self.handle(),
            layout,
            binding,
            p_offset,
        )
    }
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve_raw(
        &self,
        p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
        p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ) {
        (self
            .fp()
            .get_descriptor_set_layout_host_mapping_info_valve
            .expect("vkGetDescriptorSetLayoutHostMappingInfoVALVE is not loaded"))(
            self.handle(),
            p_binding_reference,
            p_host_mapping,
        )
    }
    pub unsafe fn get_descriptor_set_layout_size_ext_raw(
        &self,
        layout: DescriptorSetLayout,
        p_layout_size_in_bytes: *mut u64,
    ) {
        (self
            .fp()
            .get_descriptor_set_layout_size_ext
            .expect("vkGetDescriptorSetLayoutSizeEXT is not loaded"))(
            self.handle(),
            layout,
            p_layout_size_in_bytes,
        )
    }
    pub unsafe fn get_descriptor_set_layout_support_raw(
        &self,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    ) {
        (self
            .fp()
            .get_descriptor_set_layout_support
            .expect("vkGetDescriptorSetLayoutSupport is not loaded"))(
            self.handle(),
            p_create_info,
            p_support,
        )
    }
    pub unsafe fn get_descriptor_set_layout_support_khr_raw(
        &self,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    ) {
        (self
            .fp()
            .get_descriptor_set_layout_support_khr
            .expect("vkGetDescriptorSetLayoutSupportKHR is not loaded"))(
            self.handle(),
            p_create_info,
            p_support,
        )
    }
    pub unsafe fn get_device_acceleration_structure_compatibility_khr_raw(
        &self,
        p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        (self
            .fp()
            .get_device_acceleration_structure_compatibility_khr
            .expect("vkGetDeviceAccelerationStructureCompatibilityKHR is not loaded"))(
            self.handle(),
            p_version_info,
            p_compatibility,
        )
    }
    pub unsafe fn get_device_buffer_memory_requirements_raw(
        &self,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_buffer_memory_requirements
            .expect("vkGetDeviceBufferMemoryRequirements is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_device_buffer_memory_requirements_khr_raw(
        &self,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_buffer_memory_requirements_khr
            .expect("vkGetDeviceBufferMemoryRequirementsKHR is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_device_combined_image_sampler_index_nvx_raw(
        &self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        (self
            .fp()
            .get_device_combined_image_sampler_index_nvx
            .expect("vkGetDeviceCombinedImageSamplerIndexNVX is not loaded"))(
            self.handle(),
            image_view_index,
            sampler_index,
        )
    }
    pub unsafe fn get_device_fault_debug_info_khr_raw(
        &self,
        p_debug_info: *mut DeviceFaultDebugInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_device_fault_debug_info_khr
            .expect("vkGetDeviceFaultDebugInfoKHR is not loaded"))(
            self.handle(), p_debug_info
        )
    }
    pub unsafe fn get_device_fault_info_ext_raw(
        &self,
        p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
        p_fault_info: *mut DeviceFaultInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_device_fault_info_ext
            .expect("vkGetDeviceFaultInfoEXT is not loaded"))(
            self.handle(),
            p_fault_counts,
            p_fault_info,
        )
    }
    pub unsafe fn get_device_fault_reports_khr_raw(
        &self,
        timeout: u64,
        p_fault_counts: *mut u32,
        p_fault_info: *mut DeviceFaultInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_device_fault_reports_khr
            .expect("vkGetDeviceFaultReportsKHR is not loaded"))(
            self.handle(),
            timeout,
            p_fault_counts,
            p_fault_info,
        )
    }
    pub unsafe fn get_device_group_peer_memory_features_raw(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        (self
            .fp()
            .get_device_group_peer_memory_features
            .expect("vkGetDeviceGroupPeerMemoryFeatures is not loaded"))(
            self.handle(),
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    pub unsafe fn get_device_group_peer_memory_features_khr_raw(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ) {
        (self
            .fp()
            .get_device_group_peer_memory_features_khr
            .expect("vkGetDeviceGroupPeerMemoryFeaturesKHR is not loaded"))(
            self.handle(),
            heap_index,
            local_device_index,
            remote_device_index,
            p_peer_memory_features,
        )
    }
    pub unsafe fn get_device_group_present_capabilities_khr_raw(
        &self,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_device_group_present_capabilities_khr
            .expect("vkGetDeviceGroupPresentCapabilitiesKHR is not loaded"))(
            self.handle(),
            p_device_group_present_capabilities,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_device_group_surface_present_modes2_ext_raw(
        &self,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_device_group_surface_present_modes2_ext
            .expect("vkGetDeviceGroupSurfacePresentModes2EXT is not loaded"))(
            self.handle(),
            p_surface_info,
            p_modes,
        )
    }
    pub unsafe fn get_device_group_surface_present_modes_khr_raw(
        &self,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VkResult {
        (self
            .fp()
            .get_device_group_surface_present_modes_khr
            .expect("vkGetDeviceGroupSurfacePresentModesKHR is not loaded"))(
            self.handle(),
            surface,
            p_modes,
        )
    }
    pub unsafe fn get_device_image_memory_requirements_raw(
        &self,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_memory_requirements
            .expect("vkGetDeviceImageMemoryRequirements is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_device_image_memory_requirements_khr_raw(
        &self,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_memory_requirements_khr
            .expect("vkGetDeviceImageMemoryRequirementsKHR is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_raw(
        &self,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_sparse_memory_requirements
            .expect("vkGetDeviceImageSparseMemoryRequirements is not loaded"))(
            self.handle(),
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_khr_raw(
        &self,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_sparse_memory_requirements_khr
            .expect("vkGetDeviceImageSparseMemoryRequirementsKHR is not loaded"))(
            self.handle(),
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn get_device_image_subresource_layout_raw(
        &self,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_subresource_layout
            .expect("vkGetDeviceImageSubresourceLayout is not loaded"))(
            self.handle(),
            p_info,
            p_layout,
        )
    }
    pub unsafe fn get_device_image_subresource_layout_khr_raw(
        &self,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ) {
        (self
            .fp()
            .get_device_image_subresource_layout_khr
            .expect("vkGetDeviceImageSubresourceLayoutKHR is not loaded"))(
            self.handle(),
            p_info,
            p_layout,
        )
    }
    pub unsafe fn get_device_memory_commitment_raw(
        &self,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut u64,
    ) {
        (self
            .fp()
            .get_device_memory_commitment
            .expect("vkGetDeviceMemoryCommitment is not loaded"))(
            self.handle(),
            memory,
            p_committed_memory_in_bytes,
        )
    }
    pub unsafe fn get_device_memory_opaque_capture_address_raw(
        &self,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_device_memory_opaque_capture_address
            .expect("vkGetDeviceMemoryOpaqueCaptureAddress is not loaded"))(
            self.handle(), p_info
        )
    }
    pub unsafe fn get_device_memory_opaque_capture_address_khr_raw(
        &self,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_device_memory_opaque_capture_address_khr
            .expect("vkGetDeviceMemoryOpaqueCaptureAddressKHR is not loaded"))(
            self.handle(),
            p_info,
        )
    }
    pub unsafe fn get_device_micromap_compatibility_ext_raw(
        &self,
        p_version_info: *const MicromapVersionInfoEXT<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ) {
        (self
            .fp()
            .get_device_micromap_compatibility_ext
            .expect("vkGetDeviceMicromapCompatibilityEXT is not loaded"))(
            self.handle(),
            p_version_info,
            p_compatibility,
        )
    }
    pub unsafe fn get_device_queue_raw(
        &self,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ) {
        (self
            .fp()
            .get_device_queue
            .expect("vkGetDeviceQueue is not loaded"))(
            self.handle(),
            queue_family_index,
            queue_index,
            p_queue,
        )
    }
    pub unsafe fn get_device_queue2_raw(
        &self,
        p_queue_info: *const DeviceQueueInfo2<'_>,
        p_queue: *mut Queue,
    ) {
        (self
            .fp()
            .get_device_queue2
            .expect("vkGetDeviceQueue2 is not loaded"))(self.handle(), p_queue_info, p_queue)
    }
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei_raw(
        &self,
        renderpass: RenderPass,
        p_max_workgroup_size: *mut Extent2D<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_device_subpass_shading_max_workgroup_size_huawei
            .expect("vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI is not loaded"))(
            self.handle(),
            renderpass,
            p_max_workgroup_size,
        )
    }
    pub unsafe fn get_device_tensor_memory_requirements_arm_raw(
        &self,
        p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_device_tensor_memory_requirements_arm
            .expect("vkGetDeviceTensorMemoryRequirementsARM is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom_raw(
        &self,
        p_rendering_info: *const RenderingInfo<'_>,
        p_properties: *mut TilePropertiesQCOM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_dynamic_rendering_tile_properties_qcom
            .expect("vkGetDynamicRenderingTilePropertiesQCOM is not loaded"))(
            self.handle(),
            p_rendering_info,
            p_properties,
        )
    }
    pub unsafe fn get_encoded_video_session_parameters_khr_raw(
        &self,
        p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
        p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_encoded_video_session_parameters_khr
            .expect("vkGetEncodedVideoSessionParametersKHR is not loaded"))(
            self.handle(),
            p_video_session_parameters_info,
            p_feedback_info,
            p_data_size,
            p_data,
        )
    }
    pub unsafe fn get_event_status_raw(&self, event: Event) -> VkResult {
        (self
            .fp()
            .get_event_status
            .expect("vkGetEventStatus is not loaded"))(self.handle(), event)
    }
    #[cfg(feature = "beta")]
    pub unsafe fn get_execution_graph_pipeline_node_index_amdx_raw(
        &self,
        execution_graph: Pipeline,
        p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
        p_node_index: *mut u32,
    ) -> VkResult {
        (self
            .fp()
            .get_execution_graph_pipeline_node_index_amdx
            .expect("vkGetExecutionGraphPipelineNodeIndexAMDX is not loaded"))(
            self.handle(),
            execution_graph,
            p_node_info,
            p_node_index,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx_raw(
        &self,
        execution_graph: Pipeline,
        p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_execution_graph_pipeline_scratch_size_amdx
            .expect("vkGetExecutionGraphPipelineScratchSizeAMDX is not loaded"))(
            self.handle(),
            execution_graph,
            p_size_info,
        )
    }
    pub unsafe fn get_fence_fd_khr_raw(
        &self,
        p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult {
        (self
            .fp()
            .get_fence_fd_khr
            .expect("vkGetFenceFdKHR is not loaded"))(self.handle(), p_get_fd_info, p_fd)
    }
    pub unsafe fn get_fence_status_raw(&self, fence: Fence) -> VkResult {
        (self
            .fp()
            .get_fence_status
            .expect("vkGetFenceStatus is not loaded"))(self.handle(), fence)
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_fence_win32_handle_khr_raw(
        &self,
        p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult {
        (self
            .fp()
            .get_fence_win32_handle_khr
            .expect("vkGetFenceWin32HandleKHR is not loaded"))(
            self.handle(),
            p_get_win32_handle_info,
            p_handle,
        )
    }
    pub unsafe fn get_framebuffer_tile_properties_qcom_raw(
        &self,
        framebuffer: Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut TilePropertiesQCOM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_framebuffer_tile_properties_qcom
            .expect("vkGetFramebufferTilePropertiesQCOM is not loaded"))(
            self.handle(),
            framebuffer,
            p_properties_count,
            p_properties,
        )
    }
    pub unsafe fn get_generated_commands_memory_requirements_ext_raw(
        &self,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_generated_commands_memory_requirements_ext
            .expect("vkGetGeneratedCommandsMemoryRequirementsEXT is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_generated_commands_memory_requirements_nv_raw(
        &self,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_generated_commands_memory_requirements_nv
            .expect("vkGetGeneratedCommandsMemoryRequirementsNV is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_image_drm_format_modifier_properties_ext_raw(
        &self,
        image: Image,
        p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_image_drm_format_modifier_properties_ext
            .expect("vkGetImageDrmFormatModifierPropertiesEXT is not loaded"))(
            self.handle(),
            image,
            p_properties,
        )
    }
    pub unsafe fn get_image_memory_requirements_raw(
        &self,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements<'_>,
    ) {
        (self
            .fp()
            .get_image_memory_requirements
            .expect("vkGetImageMemoryRequirements is not loaded"))(
            self.handle(),
            image,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_image_memory_requirements2_raw(
        &self,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_image_memory_requirements2
            .expect("vkGetImageMemoryRequirements2 is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_image_memory_requirements2_khr_raw(
        &self,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_image_memory_requirements2_khr
            .expect("vkGetImageMemoryRequirements2KHR is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_image_opaque_capture_data_ext_raw(
        &self,
        image_count: u32,
        p_images: *const Image,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_image_opaque_capture_data_ext
            .expect("vkGetImageOpaqueCaptureDataEXT is not loaded"))(
            self.handle(),
            image_count,
            p_images,
            p_datas,
        )
    }
    pub unsafe fn get_image_opaque_capture_descriptor_data_ext_raw(
        &self,
        p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_image_opaque_capture_descriptor_data_ext
            .expect("vkGetImageOpaqueCaptureDescriptorDataEXT is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    pub unsafe fn get_image_sparse_memory_requirements_raw(
        &self,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements<'_>,
    ) {
        (self
            .fp()
            .get_image_sparse_memory_requirements
            .expect("vkGetImageSparseMemoryRequirements is not loaded"))(
            self.handle(),
            image,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn get_image_sparse_memory_requirements2_raw(
        &self,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_image_sparse_memory_requirements2
            .expect("vkGetImageSparseMemoryRequirements2 is not loaded"))(
            self.handle(),
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr_raw(
        &self,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_image_sparse_memory_requirements2_khr
            .expect("vkGetImageSparseMemoryRequirements2KHR is not loaded"))(
            self.handle(),
            p_info,
            p_sparse_memory_requirement_count,
            p_sparse_memory_requirements,
        )
    }
    pub unsafe fn get_image_subresource_layout_raw(
        &self,
        image: Image,
        p_subresource: *const ImageSubresource<'_>,
        p_layout: *mut SubresourceLayout<'_>,
    ) {
        (self
            .fp()
            .get_image_subresource_layout
            .expect("vkGetImageSubresourceLayout is not loaded"))(
            self.handle(),
            image,
            p_subresource,
            p_layout,
        )
    }
    pub unsafe fn get_image_subresource_layout2_raw(
        &self,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ) {
        (self
            .fp()
            .get_image_subresource_layout2
            .expect("vkGetImageSubresourceLayout2 is not loaded"))(
            self.handle(),
            image,
            p_subresource,
            p_layout,
        )
    }
    pub unsafe fn get_image_subresource_layout2_ext_raw(
        &self,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ) {
        (self
            .fp()
            .get_image_subresource_layout2_ext
            .expect("vkGetImageSubresourceLayout2EXT is not loaded"))(
            self.handle(),
            image,
            p_subresource,
            p_layout,
        )
    }
    pub unsafe fn get_image_subresource_layout2_khr_raw(
        &self,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ) {
        (self
            .fp()
            .get_image_subresource_layout2_khr
            .expect("vkGetImageSubresourceLayout2KHR is not loaded"))(
            self.handle(),
            image,
            p_subresource,
            p_layout,
        )
    }
    pub unsafe fn get_image_view_address_nvx_raw(
        &self,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_image_view_address_nvx
            .expect("vkGetImageViewAddressNVX is not loaded"))(
            self.handle(),
            image_view,
            p_properties,
        )
    }
    pub unsafe fn get_image_view_handle64_nvx_raw(
        &self,
        p_info: *const ImageViewHandleInfoNVX<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_image_view_handle64_nvx
            .expect("vkGetImageViewHandle64NVX is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_image_view_handle_nvx_raw(
        &self,
        p_info: *const ImageViewHandleInfoNVX<'_>,
    ) -> u32 {
        (self
            .fp()
            .get_image_view_handle_nvx
            .expect("vkGetImageViewHandleNVX is not loaded"))(self.handle(), p_info)
    }
    pub unsafe fn get_image_view_opaque_capture_descriptor_data_ext_raw(
        &self,
        p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_image_view_opaque_capture_descriptor_data_ext
            .expect("vkGetImageViewOpaqueCaptureDescriptorDataEXT is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    pub unsafe fn get_latency_timings_nv_raw(
        &self,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
    ) {
        (self
            .fp()
            .get_latency_timings_nv
            .expect("vkGetLatencyTimingsNV is not loaded"))(
            self.handle(),
            swapchain,
            p_latency_marker_info,
        )
    }
    #[cfg(feature = "android")]
    pub unsafe fn get_memory_android_hardware_buffer_android_raw(
        &self,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_android_hardware_buffer_android
            .expect("vkGetMemoryAndroidHardwareBufferANDROID is not loaded"))(
            self.handle(),
            p_info,
            p_buffer,
        )
    }
    pub unsafe fn get_memory_fd_khr_raw(
        &self,
        p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_fd_khr
            .expect("vkGetMemoryFdKHR is not loaded"))(self.handle(), p_get_fd_info, p_fd)
    }
    pub unsafe fn get_memory_fd_properties_khr_raw(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: core::ffi::c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_fd_properties_khr
            .expect("vkGetMemoryFdPropertiesKHR is not loaded"))(
            self.handle(),
            handle_type,
            fd,
            p_memory_fd_properties,
        )
    }
    pub unsafe fn get_memory_host_pointer_properties_ext_raw(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_host_pointer_properties_ext
            .expect("vkGetMemoryHostPointerPropertiesEXT is not loaded"))(
            self.handle(),
            handle_type,
            p_host_pointer,
            p_memory_host_pointer_properties,
        )
    }
    #[cfg(feature = "metal")]
    pub unsafe fn get_memory_metal_handle_ext_raw(
        &self,
        p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT<'_>,
        p_handle: *mut *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_metal_handle_ext
            .expect("vkGetMemoryMetalHandleEXT is not loaded"))(
            self.handle(),
            p_get_metal_handle_info,
            p_handle,
        )
    }
    #[cfg(feature = "metal")]
    pub unsafe fn get_memory_metal_handle_properties_ext_raw(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_handle: *const c_void,
        p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_metal_handle_properties_ext
            .expect("vkGetMemoryMetalHandlePropertiesEXT is not loaded"))(
            self.handle(),
            handle_type,
            p_handle,
            p_memory_metal_handle_properties,
        )
    }
    #[cfg(feature = "ohos")]
    pub unsafe fn get_memory_native_buffer_ohos_raw(
        &self,
        p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
        p_buffer: *mut *mut OH_NativeBuffer,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_native_buffer_ohos
            .expect("vkGetMemoryNativeBufferOHOS is not loaded"))(
            self.handle(), p_info, p_buffer
        )
    }
    pub unsafe fn get_memory_remote_address_nv_raw(
        &self,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
        p_address: *mut RemoteAddressNV,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_remote_address_nv
            .expect("vkGetMemoryRemoteAddressNV is not loaded"))(
            self.handle(),
            p_memory_get_remote_address_info,
            p_address,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_khr_raw(
        &self,
        p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_win32_handle_khr
            .expect("vkGetMemoryWin32HandleKHR is not loaded"))(
            self.handle(),
            p_get_win32_handle_info,
            p_handle,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_nv_raw(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_win32_handle_nv
            .expect("vkGetMemoryWin32HandleNV is not loaded"))(
            self.handle(),
            memory,
            handle_type,
            p_handle,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_properties_khr_raw(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_win32_handle_properties_khr
            .expect("vkGetMemoryWin32HandlePropertiesKHR is not loaded"))(
            self.handle(),
            handle_type,
            handle,
            p_memory_win32_handle_properties,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_memory_zircon_handle_fuchsia_raw(
        &self,
        p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_zircon_handle_fuchsia
            .expect("vkGetMemoryZirconHandleFUCHSIA is not loaded"))(
            self.handle(),
            p_get_zircon_handle_info,
            p_zircon_handle,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia_raw(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_memory_zircon_handle_properties_fuchsia
            .expect("vkGetMemoryZirconHandlePropertiesFUCHSIA is not loaded"))(
            self.handle(),
            handle_type,
            zircon_handle,
            p_memory_zircon_handle_properties,
        )
    }
    pub unsafe fn get_micromap_build_sizes_ext_raw(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const MicromapBuildInfoEXT<'_>,
        p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
    ) {
        (self
            .fp()
            .get_micromap_build_sizes_ext
            .expect("vkGetMicromapBuildSizesEXT is not loaded"))(
            self.handle(),
            build_type,
            p_build_info,
            p_size_info,
        )
    }
    #[cfg(feature = "ohos")]
    pub unsafe fn get_native_buffer_properties_ohos_raw(
        &self,
        buffer: *const OH_NativeBuffer,
        p_properties: *mut NativeBufferPropertiesOHOS<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_native_buffer_properties_ohos
            .expect("vkGetNativeBufferPropertiesOHOS is not loaded"))(
            self.handle(),
            buffer,
            p_properties,
        )
    }
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv_raw(
        &self,
        p_info: *const PartitionedAccelerationStructureInstancesInputNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        (self
            .fp()
            .get_partitioned_acceleration_structures_build_sizes_nv
            .expect("vkGetPartitionedAccelerationStructuresBuildSizesNV is not loaded"))(
            self.handle(),
            p_info,
            p_size_info,
        )
    }
    pub unsafe fn get_past_presentation_timing_ext_raw(
        &self,
        p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
        p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_past_presentation_timing_ext
            .expect("vkGetPastPresentationTimingEXT is not loaded"))(
            self.handle(),
            p_past_presentation_timing_info,
            p_past_presentation_timing_properties,
        )
    }
    pub unsafe fn get_past_presentation_timing_google_raw(
        &self,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_past_presentation_timing_google
            .expect("vkGetPastPresentationTimingGOOGLE is not loaded"))(
            self.handle(),
            swapchain,
            p_presentation_timing_count,
            p_presentation_timings,
        )
    }
    pub unsafe fn get_performance_parameter_intel_raw(
        &self,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_performance_parameter_intel
            .expect("vkGetPerformanceParameterINTEL is not loaded"))(
            self.handle(),
            parameter,
            p_value,
        )
    }
    pub unsafe fn get_pipeline_binary_data_khr_raw(
        &self,
        p_info: *const PipelineBinaryDataInfoKHR<'_>,
        p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
        p_pipeline_binary_data_size: *mut usize,
        p_pipeline_binary_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_binary_data_khr
            .expect("vkGetPipelineBinaryDataKHR is not loaded"))(
            self.handle(),
            p_info,
            p_pipeline_binary_key,
            p_pipeline_binary_data_size,
            p_pipeline_binary_data,
        )
    }
    pub unsafe fn get_pipeline_cache_data_raw(
        &self,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_cache_data
            .expect("vkGetPipelineCacheData is not loaded"))(
            self.handle(),
            pipeline_cache,
            p_data_size,
            p_data,
        )
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr_raw(
        &self,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_executable_internal_representations_khr
            .expect("vkGetPipelineExecutableInternalRepresentationsKHR is not loaded"))(
            self.handle(),
            p_executable_info,
            p_internal_representation_count,
            p_internal_representations,
        )
    }
    pub unsafe fn get_pipeline_executable_properties_khr_raw(
        &self,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_executable_properties_khr
            .expect("vkGetPipelineExecutablePropertiesKHR is not loaded"))(
            self.handle(),
            p_pipeline_info,
            p_executable_count,
            p_properties,
        )
    }
    pub unsafe fn get_pipeline_executable_statistics_khr_raw(
        &self,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_executable_statistics_khr
            .expect("vkGetPipelineExecutableStatisticsKHR is not loaded"))(
            self.handle(),
            p_executable_info,
            p_statistic_count,
            p_statistics,
        )
    }
    pub unsafe fn get_pipeline_indirect_device_address_nv_raw(
        &self,
        p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> u64 {
        (self
            .fp()
            .get_pipeline_indirect_device_address_nv
            .expect("vkGetPipelineIndirectDeviceAddressNV is not loaded"))(
            self.handle(), p_info
        )
    }
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv_raw(
        &self,
        p_create_info: *const ComputePipelineCreateInfo<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_pipeline_indirect_memory_requirements_nv
            .expect("vkGetPipelineIndirectMemoryRequirementsNV is not loaded"))(
            self.handle(),
            p_create_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_pipeline_key_khr_raw(
        &self,
        p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
        p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_key_khr
            .expect("vkGetPipelineKeyKHR is not loaded"))(
            self.handle(),
            p_pipeline_create_info,
            p_pipeline_key,
        )
    }
    pub unsafe fn get_pipeline_properties_ext_raw(
        &self,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_pipeline_properties: *mut BaseOutStructure<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_pipeline_properties_ext
            .expect("vkGetPipelinePropertiesEXT is not loaded"))(
            self.handle(),
            p_pipeline_info,
            p_pipeline_properties,
        )
    }
    pub unsafe fn get_private_data_raw(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ) {
        (self
            .fp()
            .get_private_data
            .expect("vkGetPrivateData is not loaded"))(
            self.handle(),
            object_type,
            object_handle,
            private_data_slot,
            p_data,
        )
    }
    pub unsafe fn get_private_data_ext_raw(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ) {
        (self
            .fp()
            .get_private_data_ext
            .expect("vkGetPrivateDataEXT is not loaded"))(
            self.handle(),
            object_type,
            object_handle,
            private_data_slot,
            p_data,
        )
    }
    pub unsafe fn get_query_pool_results_raw(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: u64,
        flags: QueryResultFlags,
    ) -> VkResult {
        (self
            .fp()
            .get_query_pool_results
            .expect("vkGetQueryPoolResults is not loaded"))(
            self.handle(),
            query_pool,
            first_query,
            query_count,
            data_size,
            p_data,
            stride,
            flags,
        )
    }
    pub unsafe fn get_queue_checkpoint_data2_nv_raw(
        &self,
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointData2NV<'_>,
    ) {
        (self
            .fp()
            .get_queue_checkpoint_data2_nv
            .expect("vkGetQueueCheckpointData2NV is not loaded"))(
            queue,
            p_checkpoint_data_count,
            p_checkpoint_data,
        )
    }
    pub unsafe fn get_queue_checkpoint_data_nv_raw(
        &self,
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV<'_>,
    ) {
        (self
            .fp()
            .get_queue_checkpoint_data_nv
            .expect("vkGetQueueCheckpointDataNV is not loaded"))(
            queue,
            p_checkpoint_data_count,
            p_checkpoint_data,
        )
    }
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr_raw(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_ray_tracing_capture_replay_shader_group_handles_khr
            .expect("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR is not loaded"))(
            self.handle(),
            pipeline,
            first_group,
            group_count,
            data_size,
            p_data,
        )
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_khr_raw(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_ray_tracing_shader_group_handles_khr
            .expect("vkGetRayTracingShaderGroupHandlesKHR is not loaded"))(
            self.handle(),
            pipeline,
            first_group,
            group_count,
            data_size,
            p_data,
        )
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_nv_raw(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_ray_tracing_shader_group_handles_nv
            .expect("vkGetRayTracingShaderGroupHandlesNV is not loaded"))(
            self.handle(),
            pipeline,
            first_group,
            group_count,
            data_size,
            p_data,
        )
    }
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr_raw(
        &self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> u64 {
        (self
            .fp()
            .get_ray_tracing_shader_group_stack_size_khr
            .expect("vkGetRayTracingShaderGroupStackSizeKHR is not loaded"))(
            self.handle(),
            pipeline,
            group,
            group_shader,
        )
    }
    pub unsafe fn get_refresh_cycle_duration_google_raw(
        &self,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_refresh_cycle_duration_google
            .expect("vkGetRefreshCycleDurationGOOGLE is not loaded"))(
            self.handle(),
            swapchain,
            p_display_timing_properties,
        )
    }
    pub unsafe fn get_render_area_granularity_raw(
        &self,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D<'_>,
    ) {
        (self
            .fp()
            .get_render_area_granularity
            .expect("vkGetRenderAreaGranularity is not loaded"))(
            self.handle(),
            render_pass,
            p_granularity,
        )
    }
    pub unsafe fn get_rendering_area_granularity_raw(
        &self,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D<'_>,
    ) {
        (self
            .fp()
            .get_rendering_area_granularity
            .expect("vkGetRenderingAreaGranularity is not loaded"))(
            self.handle(),
            p_rendering_area_info,
            p_granularity,
        )
    }
    pub unsafe fn get_rendering_area_granularity_khr_raw(
        &self,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D<'_>,
    ) {
        (self
            .fp()
            .get_rendering_area_granularity_khr
            .expect("vkGetRenderingAreaGranularityKHR is not loaded"))(
            self.handle(),
            p_rendering_area_info,
            p_granularity,
        )
    }
    pub unsafe fn get_sampler_opaque_capture_descriptor_data_ext_raw(
        &self,
        p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_sampler_opaque_capture_descriptor_data_ext
            .expect("vkGetSamplerOpaqueCaptureDescriptorDataEXT is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    #[cfg(feature = "screen")]
    pub unsafe fn get_screen_buffer_properties_qnx_raw(
        &self,
        buffer: *const _screen_buffer,
        p_properties: *mut ScreenBufferPropertiesQNX<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_screen_buffer_properties_qnx
            .expect("vkGetScreenBufferPropertiesQNX is not loaded"))(
            self.handle(),
            buffer,
            p_properties,
        )
    }
    pub unsafe fn get_semaphore_counter_value_raw(
        &self,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_semaphore_counter_value
            .expect("vkGetSemaphoreCounterValue is not loaded"))(
            self.handle(), semaphore, p_value
        )
    }
    pub unsafe fn get_semaphore_counter_value_khr_raw(
        &self,
        semaphore: Semaphore,
        p_value: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_semaphore_counter_value_khr
            .expect("vkGetSemaphoreCounterValueKHR is not loaded"))(
            self.handle(),
            semaphore,
            p_value,
        )
    }
    pub unsafe fn get_semaphore_fd_khr_raw(
        &self,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult {
        (self
            .fp()
            .get_semaphore_fd_khr
            .expect("vkGetSemaphoreFdKHR is not loaded"))(self.handle(), p_get_fd_info, p_fd)
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_semaphore_win32_handle_khr_raw(
        &self,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult {
        (self
            .fp()
            .get_semaphore_win32_handle_khr
            .expect("vkGetSemaphoreWin32HandleKHR is not loaded"))(
            self.handle(),
            p_get_win32_handle_info,
            p_handle,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia_raw(
        &self,
        p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VkResult {
        (self
            .fp()
            .get_semaphore_zircon_handle_fuchsia
            .expect("vkGetSemaphoreZirconHandleFUCHSIA is not loaded"))(
            self.handle(),
            p_get_zircon_handle_info,
            p_zircon_handle,
        )
    }
    pub unsafe fn get_shader_binary_data_ext_raw(
        &self,
        shader: ShaderEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_shader_binary_data_ext
            .expect("vkGetShaderBinaryDataEXT is not loaded"))(
            self.handle(),
            shader,
            p_data_size,
            p_data,
        )
    }
    pub unsafe fn get_shader_info_amd_raw(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_shader_info_amd
            .expect("vkGetShaderInfoAMD is not loaded"))(
            self.handle(),
            pipeline,
            shader_stage,
            info_type,
            p_info_size,
            p_info,
        )
    }
    pub unsafe fn get_shader_instrumentation_values_arm_raw(
        &self,
        instrumentation: ShaderInstrumentationARM,
        p_metric_block_count: *mut u32,
        p_metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> VkResult {
        (self
            .fp()
            .get_shader_instrumentation_values_arm
            .expect("vkGetShaderInstrumentationValuesARM is not loaded"))(
            self.handle(),
            instrumentation,
            p_metric_block_count,
            p_metric_values,
            flags,
        )
    }
    pub unsafe fn get_shader_module_create_info_identifier_ext_raw(
        &self,
        p_create_info: *const ShaderModuleCreateInfo<'_>,
        p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
    ) {
        (self
            .fp()
            .get_shader_module_create_info_identifier_ext
            .expect("vkGetShaderModuleCreateInfoIdentifierEXT is not loaded"))(
            self.handle(),
            p_create_info,
            p_identifier,
        )
    }
    pub unsafe fn get_shader_module_identifier_ext_raw(
        &self,
        shader_module: ShaderModule,
        p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
    ) {
        (self
            .fp()
            .get_shader_module_identifier_ext
            .expect("vkGetShaderModuleIdentifierEXT is not loaded"))(
            self.handle(),
            shader_module,
            p_identifier,
        )
    }
    pub unsafe fn get_swapchain_counter_ext_raw(
        &self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
        p_counter_value: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_swapchain_counter_ext
            .expect("vkGetSwapchainCounterEXT is not loaded"))(
            self.handle(),
            swapchain,
            counter,
            p_counter_value,
        )
    }
    pub unsafe fn get_swapchain_images_khr_raw(
        &self,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
    ) -> VkResult {
        (self
            .fp()
            .get_swapchain_images_khr
            .expect("vkGetSwapchainImagesKHR is not loaded"))(
            self.handle(),
            swapchain,
            p_swapchain_image_count,
            p_swapchain_images,
        )
    }
    pub unsafe fn get_swapchain_status_khr_raw(&self, swapchain: SwapchainKHR) -> VkResult {
        (self
            .fp()
            .get_swapchain_status_khr
            .expect("vkGetSwapchainStatusKHR is not loaded"))(self.handle(), swapchain)
    }
    pub unsafe fn get_swapchain_time_domain_properties_ext_raw(
        &self,
        swapchain: SwapchainKHR,
        p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
        p_time_domains_counter: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_swapchain_time_domain_properties_ext
            .expect("vkGetSwapchainTimeDomainPropertiesEXT is not loaded"))(
            self.handle(),
            swapchain,
            p_swapchain_time_domain_properties,
            p_time_domains_counter,
        )
    }
    pub unsafe fn get_swapchain_timing_properties_ext_raw(
        &self,
        swapchain: SwapchainKHR,
        p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
        p_swapchain_timing_properties_counter: *mut u64,
    ) -> VkResult {
        (self
            .fp()
            .get_swapchain_timing_properties_ext
            .expect("vkGetSwapchainTimingPropertiesEXT is not loaded"))(
            self.handle(),
            swapchain,
            p_swapchain_timing_properties,
            p_swapchain_timing_properties_counter,
        )
    }
    pub unsafe fn get_tensor_memory_requirements_arm_raw(
        &self,
        p_info: *const TensorMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ) {
        (self
            .fp()
            .get_tensor_memory_requirements_arm
            .expect("vkGetTensorMemoryRequirementsARM is not loaded"))(
            self.handle(),
            p_info,
            p_memory_requirements,
        )
    }
    pub unsafe fn get_tensor_opaque_capture_data_arm_raw(
        &self,
        tensor_count: u32,
        p_tensors: *const TensorARM,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_tensor_opaque_capture_data_arm
            .expect("vkGetTensorOpaqueCaptureDataARM is not loaded"))(
            self.handle(),
            tensor_count,
            p_tensors,
            p_datas,
        )
    }
    pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm_raw(
        &self,
        p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_tensor_opaque_capture_descriptor_data_arm
            .expect("vkGetTensorOpaqueCaptureDescriptorDataARM is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm_raw(
        &self,
        p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_tensor_view_opaque_capture_descriptor_data_arm
            .expect("vkGetTensorViewOpaqueCaptureDescriptorDataARM is not loaded"))(
            self.handle(),
            p_info,
            p_data,
        )
    }
    pub unsafe fn get_validation_cache_data_ext_raw(
        &self,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .get_validation_cache_data_ext
            .expect("vkGetValidationCacheDataEXT is not loaded"))(
            self.handle(),
            validation_cache,
            p_data_size,
            p_data,
        )
    }
    pub unsafe fn get_video_session_memory_requirements_khr_raw(
        &self,
        video_session: VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .get_video_session_memory_requirements_khr
            .expect("vkGetVideoSessionMemoryRequirementsKHR is not loaded"))(
            self.handle(),
            video_session,
            p_memory_requirements_count,
            p_memory_requirements,
        )
    }
    pub unsafe fn import_fence_fd_khr_raw(
        &self,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .import_fence_fd_khr
            .expect("vkImportFenceFdKHR is not loaded"))(
            self.handle(), p_import_fence_fd_info
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn import_fence_win32_handle_khr_raw(
        &self,
        p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .import_fence_win32_handle_khr
            .expect("vkImportFenceWin32HandleKHR is not loaded"))(
            self.handle(),
            p_import_fence_win32_handle_info,
        )
    }
    pub unsafe fn import_semaphore_fd_khr_raw(
        &self,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .import_semaphore_fd_khr
            .expect("vkImportSemaphoreFdKHR is not loaded"))(
            self.handle(),
            p_import_semaphore_fd_info,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn import_semaphore_win32_handle_khr_raw(
        &self,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .import_semaphore_win32_handle_khr
            .expect("vkImportSemaphoreWin32HandleKHR is not loaded"))(
            self.handle(),
            p_import_semaphore_win32_handle_info,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia_raw(
        &self,
        p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    ) -> VkResult {
        (self
            .fp()
            .import_semaphore_zircon_handle_fuchsia
            .expect("vkImportSemaphoreZirconHandleFUCHSIA is not loaded"))(
            self.handle(),
            p_import_semaphore_zircon_handle_info,
        )
    }
    pub unsafe fn initialize_performance_api_intel_raw(
        &self,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
    ) -> VkResult {
        (self
            .fp()
            .initialize_performance_api_intel
            .expect("vkInitializePerformanceApiINTEL is not loaded"))(
            self.handle(),
            p_initialize_info,
        )
    }
    pub unsafe fn invalidate_mapped_memory_ranges_raw(
        &self,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> VkResult {
        (self
            .fp()
            .invalidate_mapped_memory_ranges
            .expect("vkInvalidateMappedMemoryRanges is not loaded"))(
            self.handle(),
            memory_range_count,
            p_memory_ranges,
        )
    }
    pub unsafe fn latency_sleep_nv_raw(
        &self,
        swapchain: SwapchainKHR,
        p_sleep_info: *const LatencySleepInfoNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .latency_sleep_nv
            .expect("vkLatencySleepNV is not loaded"))(
            self.handle(), swapchain, p_sleep_info
        )
    }
    pub unsafe fn map_memory_raw(
        &self,
        memory: DeviceMemory,
        offset: u64,
        size: u64,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> VkResult {
        (self.fp().map_memory.expect("vkMapMemory is not loaded"))(
            self.handle(),
            memory,
            offset,
            size,
            flags,
            pp_data,
        )
    }
    pub unsafe fn map_memory2_raw(
        &self,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> VkResult {
        (self.fp().map_memory2.expect("vkMapMemory2 is not loaded"))(
            self.handle(),
            p_memory_map_info,
            pp_data,
        )
    }
    pub unsafe fn map_memory2_khr_raw(
        &self,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> VkResult {
        (self
            .fp()
            .map_memory2_khr
            .expect("vkMapMemory2KHR is not loaded"))(
            self.handle(), p_memory_map_info, pp_data
        )
    }
    pub unsafe fn merge_pipeline_caches_raw(
        &self,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> VkResult {
        (self
            .fp()
            .merge_pipeline_caches
            .expect("vkMergePipelineCaches is not loaded"))(
            self.handle(),
            dst_cache,
            src_cache_count,
            p_src_caches,
        )
    }
    pub unsafe fn merge_validation_caches_ext_raw(
        &self,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> VkResult {
        (self
            .fp()
            .merge_validation_caches_ext
            .expect("vkMergeValidationCachesEXT is not loaded"))(
            self.handle(),
            dst_cache,
            src_cache_count,
            p_src_caches,
        )
    }
    pub unsafe fn queue_begin_debug_utils_label_ext_raw(
        &self,
        queue: Queue,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ) {
        (self
            .fp()
            .queue_begin_debug_utils_label_ext
            .expect("vkQueueBeginDebugUtilsLabelEXT is not loaded"))(queue, p_label_info)
    }
    pub unsafe fn queue_bind_sparse_raw(
        &self,
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo<'_>,
        fence: Fence,
    ) -> VkResult {
        (self
            .fp()
            .queue_bind_sparse
            .expect("vkQueueBindSparse is not loaded"))(
            queue, bind_info_count, p_bind_info, fence
        )
    }
    pub unsafe fn queue_end_debug_utils_label_ext_raw(&self, queue: Queue) {
        (self
            .fp()
            .queue_end_debug_utils_label_ext
            .expect("vkQueueEndDebugUtilsLabelEXT is not loaded"))(queue)
    }
    pub unsafe fn queue_insert_debug_utils_label_ext_raw(
        &self,
        queue: Queue,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ) {
        (self
            .fp()
            .queue_insert_debug_utils_label_ext
            .expect("vkQueueInsertDebugUtilsLabelEXT is not loaded"))(queue, p_label_info)
    }
    pub unsafe fn queue_notify_out_of_band_nv_raw(
        &self,
        queue: Queue,
        p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>,
    ) {
        (self
            .fp()
            .queue_notify_out_of_band_nv
            .expect("vkQueueNotifyOutOfBandNV is not loaded"))(queue, p_queue_type_info)
    }
    pub unsafe fn queue_present_khr_raw(
        &self,
        queue: Queue,
        p_present_info: *const PresentInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .queue_present_khr
            .expect("vkQueuePresentKHR is not loaded"))(queue, p_present_info)
    }
    pub unsafe fn queue_set_perf_hint_qcom_raw(
        &self,
        queue: Queue,
        p_perf_hint_info: *const PerfHintInfoQCOM<'_>,
    ) -> VkResult {
        (self
            .fp()
            .queue_set_perf_hint_qcom
            .expect("vkQueueSetPerfHintQCOM is not loaded"))(queue, p_perf_hint_info)
    }
    pub unsafe fn queue_set_performance_configuration_intel_raw(
        &self,
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> VkResult {
        (self
            .fp()
            .queue_set_performance_configuration_intel
            .expect("vkQueueSetPerformanceConfigurationINTEL is not loaded"))(
            queue, configuration
        )
    }
    pub unsafe fn queue_submit_raw(
        &self,
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo<'_>,
        fence: Fence,
    ) -> VkResult {
        (self.fp().queue_submit.expect("vkQueueSubmit is not loaded"))(
            queue,
            submit_count,
            p_submits,
            fence,
        )
    }
    pub unsafe fn queue_submit2_raw(
        &self,
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> VkResult {
        (self
            .fp()
            .queue_submit2
            .expect("vkQueueSubmit2 is not loaded"))(queue, submit_count, p_submits, fence)
    }
    pub unsafe fn queue_submit2_khr_raw(
        &self,
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> VkResult {
        (self
            .fp()
            .queue_submit2_khr
            .expect("vkQueueSubmit2KHR is not loaded"))(
            queue, submit_count, p_submits, fence
        )
    }
    pub unsafe fn queue_wait_idle_raw(&self, queue: Queue) -> VkResult {
        (self
            .fp()
            .queue_wait_idle
            .expect("vkQueueWaitIdle is not loaded"))(queue)
    }
    pub unsafe fn register_custom_border_color_ext_raw(
        &self,
        p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
        request_index: Bool32,
        p_index: *mut u32,
    ) -> VkResult {
        (self
            .fp()
            .register_custom_border_color_ext
            .expect("vkRegisterCustomBorderColorEXT is not loaded"))(
            self.handle(),
            p_border_color,
            request_index,
            p_index,
        )
    }
    pub unsafe fn register_device_event_ext_raw(
        &self,
        p_device_event_info: *const DeviceEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult {
        (self
            .fp()
            .register_device_event_ext
            .expect("vkRegisterDeviceEventEXT is not loaded"))(
            self.handle(),
            p_device_event_info,
            p_allocator,
            p_fence,
        )
    }
    pub unsafe fn register_display_event_ext_raw(
        &self,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult {
        (self
            .fp()
            .register_display_event_ext
            .expect("vkRegisterDisplayEventEXT is not loaded"))(
            self.handle(),
            display,
            p_display_event_info,
            p_allocator,
            p_fence,
        )
    }
    pub unsafe fn release_captured_pipeline_data_khr_raw(
        &self,
        p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
    ) -> VkResult {
        (self
            .fp()
            .release_captured_pipeline_data_khr
            .expect("vkReleaseCapturedPipelineDataKHR is not loaded"))(
            self.handle(),
            p_info,
            p_allocator,
        )
    }
    #[cfg(feature = "win32")]
    pub unsafe fn release_full_screen_exclusive_mode_ext_raw(
        &self,
        swapchain: SwapchainKHR,
    ) -> VkResult {
        (self
            .fp()
            .release_full_screen_exclusive_mode_ext
            .expect("vkReleaseFullScreenExclusiveModeEXT is not loaded"))(
            self.handle(), swapchain
        )
    }
    pub unsafe fn release_performance_configuration_intel_raw(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> VkResult {
        (self
            .fp()
            .release_performance_configuration_intel
            .expect("vkReleasePerformanceConfigurationINTEL is not loaded"))(
            self.handle(),
            configuration,
        )
    }
    pub unsafe fn release_profiling_lock_khr_raw(&self) {
        (self
            .fp()
            .release_profiling_lock_khr
            .expect("vkReleaseProfilingLockKHR is not loaded"))(self.handle())
    }
    pub unsafe fn release_swapchain_images_ext_raw(
        &self,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .release_swapchain_images_ext
            .expect("vkReleaseSwapchainImagesEXT is not loaded"))(
            self.handle(), p_release_info
        )
    }
    pub unsafe fn release_swapchain_images_khr_raw(
        &self,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .release_swapchain_images_khr
            .expect("vkReleaseSwapchainImagesKHR is not loaded"))(
            self.handle(), p_release_info
        )
    }
    pub unsafe fn reset_command_buffer_raw(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> VkResult {
        (self
            .fp()
            .reset_command_buffer
            .expect("vkResetCommandBuffer is not loaded"))(command_buffer, flags)
    }
    pub unsafe fn reset_command_pool_raw(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> VkResult {
        (self
            .fp()
            .reset_command_pool
            .expect("vkResetCommandPool is not loaded"))(self.handle(), command_pool, flags)
    }
    pub unsafe fn reset_descriptor_pool_raw(
        &self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> VkResult {
        (self
            .fp()
            .reset_descriptor_pool
            .expect("vkResetDescriptorPool is not loaded"))(
            self.handle(), descriptor_pool, flags
        )
    }
    pub unsafe fn reset_event_raw(&self, event: Event) -> VkResult {
        (self.fp().reset_event.expect("vkResetEvent is not loaded"))(self.handle(), event)
    }
    pub unsafe fn reset_fences_raw(&self, fence_count: u32, p_fences: *const Fence) -> VkResult {
        (self.fp().reset_fences.expect("vkResetFences is not loaded"))(
            self.handle(),
            fence_count,
            p_fences,
        )
    }
    pub unsafe fn reset_query_pool_raw(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .fp()
            .reset_query_pool
            .expect("vkResetQueryPool is not loaded"))(
            self.handle(),
            query_pool,
            first_query,
            query_count,
        )
    }
    pub unsafe fn reset_query_pool_ext_raw(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        (self
            .fp()
            .reset_query_pool_ext
            .expect("vkResetQueryPoolEXT is not loaded"))(
            self.handle(),
            query_pool,
            first_query,
            query_count,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia_raw(
        &self,
        collection: BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult {
        (self
            .fp()
            .set_buffer_collection_buffer_constraints_fuchsia
            .expect("vkSetBufferCollectionBufferConstraintsFUCHSIA is not loaded"))(
            self.handle(),
            collection,
            p_buffer_constraints_info,
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia_raw(
        &self,
        collection: BufferCollectionFUCHSIA,
        p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult {
        (self
            .fp()
            .set_buffer_collection_image_constraints_fuchsia
            .expect("vkSetBufferCollectionImageConstraintsFUCHSIA is not loaded"))(
            self.handle(),
            collection,
            p_image_constraints_info,
        )
    }
    pub unsafe fn set_debug_utils_object_name_ext_raw(
        &self,
        p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .set_debug_utils_object_name_ext
            .expect("vkSetDebugUtilsObjectNameEXT is not loaded"))(
            self.handle(), p_name_info
        )
    }
    pub unsafe fn set_debug_utils_object_tag_ext_raw(
        &self,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .set_debug_utils_object_tag_ext
            .expect("vkSetDebugUtilsObjectTagEXT is not loaded"))(self.handle(), p_tag_info)
    }
    pub unsafe fn set_device_memory_priority_ext_raw(&self, memory: DeviceMemory, priority: f32) {
        (self
            .fp()
            .set_device_memory_priority_ext
            .expect("vkSetDeviceMemoryPriorityEXT is not loaded"))(
            self.handle(), memory, priority
        )
    }
    pub unsafe fn set_event_raw(&self, event: Event) -> VkResult {
        (self.fp().set_event.expect("vkSetEvent is not loaded"))(self.handle(), event)
    }
    pub unsafe fn set_hdr_metadata_ext_raw(
        &self,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT<'_>,
    ) {
        (self
            .fp()
            .set_hdr_metadata_ext
            .expect("vkSetHdrMetadataEXT is not loaded"))(
            self.handle(),
            swapchain_count,
            p_swapchains,
            p_metadata,
        )
    }
    pub unsafe fn set_latency_marker_nv_raw(
        &self,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
    ) {
        (self
            .fp()
            .set_latency_marker_nv
            .expect("vkSetLatencyMarkerNV is not loaded"))(
            self.handle(),
            swapchain,
            p_latency_marker_info,
        )
    }
    pub unsafe fn set_latency_sleep_mode_nv_raw(
        &self,
        swapchain: SwapchainKHR,
        p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
    ) -> VkResult {
        (self
            .fp()
            .set_latency_sleep_mode_nv
            .expect("vkSetLatencySleepModeNV is not loaded"))(
            self.handle(),
            swapchain,
            p_sleep_mode_info,
        )
    }
    pub unsafe fn set_local_dimming_amd_raw(
        &self,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ) {
        (self
            .fp()
            .set_local_dimming_amd
            .expect("vkSetLocalDimmingAMD is not loaded"))(
            self.handle(),
            swap_chain,
            local_dimming_enable,
        )
    }
    pub unsafe fn set_private_data_raw(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> VkResult {
        (self
            .fp()
            .set_private_data
            .expect("vkSetPrivateData is not loaded"))(
            self.handle(),
            object_type,
            object_handle,
            private_data_slot,
            data,
        )
    }
    pub unsafe fn set_private_data_ext_raw(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> VkResult {
        (self
            .fp()
            .set_private_data_ext
            .expect("vkSetPrivateDataEXT is not loaded"))(
            self.handle(),
            object_type,
            object_handle,
            private_data_slot,
            data,
        )
    }
    pub unsafe fn set_swapchain_present_timing_queue_size_ext_raw(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> VkResult {
        (self
            .fp()
            .set_swapchain_present_timing_queue_size_ext
            .expect("vkSetSwapchainPresentTimingQueueSizeEXT is not loaded"))(
            self.handle(),
            swapchain,
            size,
        )
    }
    pub unsafe fn signal_semaphore_raw(
        &self,
        p_signal_info: *const SemaphoreSignalInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .signal_semaphore
            .expect("vkSignalSemaphore is not loaded"))(self.handle(), p_signal_info)
    }
    pub unsafe fn signal_semaphore_khr_raw(
        &self,
        p_signal_info: *const SemaphoreSignalInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .signal_semaphore_khr
            .expect("vkSignalSemaphoreKHR is not loaded"))(self.handle(), p_signal_info)
    }
    pub unsafe fn transition_image_layout_raw(
        &self,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .transition_image_layout
            .expect("vkTransitionImageLayout is not loaded"))(
            self.handle(),
            transition_count,
            p_transitions,
        )
    }
    pub unsafe fn transition_image_layout_ext_raw(
        &self,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .transition_image_layout_ext
            .expect("vkTransitionImageLayoutEXT is not loaded"))(
            self.handle(),
            transition_count,
            p_transitions,
        )
    }
    pub unsafe fn trim_command_pool_raw(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        (self
            .fp()
            .trim_command_pool
            .expect("vkTrimCommandPool is not loaded"))(self.handle(), command_pool, flags)
    }
    pub unsafe fn trim_command_pool_khr_raw(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        (self
            .fp()
            .trim_command_pool_khr
            .expect("vkTrimCommandPoolKHR is not loaded"))(
            self.handle(), command_pool, flags
        )
    }
    pub unsafe fn uninitialize_performance_api_intel_raw(&self) {
        (self
            .fp()
            .uninitialize_performance_api_intel
            .expect("vkUninitializePerformanceApiINTEL is not loaded"))(self.handle())
    }
    pub unsafe fn unmap_memory_raw(&self, memory: DeviceMemory) {
        (self.fp().unmap_memory.expect("vkUnmapMemory is not loaded"))(self.handle(), memory)
    }
    pub unsafe fn unmap_memory2_raw(
        &self,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .unmap_memory2
            .expect("vkUnmapMemory2 is not loaded"))(self.handle(), p_memory_unmap_info)
    }
    pub unsafe fn unmap_memory2_khr_raw(
        &self,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> VkResult {
        (self
            .fp()
            .unmap_memory2_khr
            .expect("vkUnmapMemory2KHR is not loaded"))(self.handle(), p_memory_unmap_info)
    }
    pub unsafe fn unregister_custom_border_color_ext_raw(&self, index: u32) {
        (self
            .fp()
            .unregister_custom_border_color_ext
            .expect("vkUnregisterCustomBorderColorEXT is not loaded"))(self.handle(), index)
    }
    pub unsafe fn update_descriptor_set_with_template_raw(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .update_descriptor_set_with_template
            .expect("vkUpdateDescriptorSetWithTemplate is not loaded"))(
            self.handle(),
            descriptor_set,
            descriptor_update_template,
            p_data,
        )
    }
    pub unsafe fn update_descriptor_set_with_template_khr_raw(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ) {
        (self
            .fp()
            .update_descriptor_set_with_template_khr
            .expect("vkUpdateDescriptorSetWithTemplateKHR is not loaded"))(
            self.handle(),
            descriptor_set,
            descriptor_update_template,
            p_data,
        )
    }
    pub unsafe fn update_descriptor_sets_raw(
        &self,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet<'_>,
    ) {
        (self
            .fp()
            .update_descriptor_sets
            .expect("vkUpdateDescriptorSets is not loaded"))(
            self.handle(),
            descriptor_write_count,
            p_descriptor_writes,
            descriptor_copy_count,
            p_descriptor_copies,
        )
    }
    pub unsafe fn update_indirect_execution_set_pipeline_ext_raw(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT<'_>,
    ) {
        (self
            .fp()
            .update_indirect_execution_set_pipeline_ext
            .expect("vkUpdateIndirectExecutionSetPipelineEXT is not loaded"))(
            self.handle(),
            indirect_execution_set,
            execution_set_write_count,
            p_execution_set_writes,
        )
    }
    pub unsafe fn update_indirect_execution_set_shader_ext_raw(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT<'_>,
    ) {
        (self
            .fp()
            .update_indirect_execution_set_shader_ext
            .expect("vkUpdateIndirectExecutionSetShaderEXT is not loaded"))(
            self.handle(),
            indirect_execution_set,
            execution_set_write_count,
            p_execution_set_writes,
        )
    }
    pub unsafe fn update_video_session_parameters_khr_raw(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .update_video_session_parameters_khr
            .expect("vkUpdateVideoSessionParametersKHR is not loaded"))(
            self.handle(),
            video_session_parameters,
            p_update_info,
        )
    }
    pub unsafe fn wait_for_fences_raw(
        &self,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> VkResult {
        (self
            .fp()
            .wait_for_fences
            .expect("vkWaitForFences is not loaded"))(
            self.handle(),
            fence_count,
            p_fences,
            wait_all,
            timeout,
        )
    }
    pub unsafe fn wait_for_present2_khr_raw(
        &self,
        swapchain: SwapchainKHR,
        p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
    ) -> VkResult {
        (self
            .fp()
            .wait_for_present2_khr
            .expect("vkWaitForPresent2KHR is not loaded"))(
            self.handle(),
            swapchain,
            p_present_wait2_info,
        )
    }
    pub unsafe fn wait_for_present_khr_raw(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> VkResult {
        (self
            .fp()
            .wait_for_present_khr
            .expect("vkWaitForPresentKHR is not loaded"))(
            self.handle(),
            swapchain,
            present_id,
            timeout,
        )
    }
    pub unsafe fn wait_semaphores_raw(
        &self,
        p_wait_info: *const SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult {
        (self
            .fp()
            .wait_semaphores
            .expect("vkWaitSemaphores is not loaded"))(self.handle(), p_wait_info, timeout)
    }
    pub unsafe fn wait_semaphores_khr_raw(
        &self,
        p_wait_info: *const SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult {
        (self
            .fp()
            .wait_semaphores_khr
            .expect("vkWaitSemaphoresKHR is not loaded"))(
            self.handle(), p_wait_info, timeout
        )
    }
    pub unsafe fn write_acceleration_structures_properties_khr_raw(
        &self,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> VkResult {
        (self
            .fp()
            .write_acceleration_structures_properties_khr
            .expect("vkWriteAccelerationStructuresPropertiesKHR is not loaded"))(
            self.handle(),
            acceleration_structure_count,
            p_acceleration_structures,
            query_type,
            data_size,
            p_data,
            stride,
        )
    }
    pub unsafe fn write_micromaps_properties_ext_raw(
        &self,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> VkResult {
        (self
            .fp()
            .write_micromaps_properties_ext
            .expect("vkWriteMicromapsPropertiesEXT is not loaded"))(
            self.handle(),
            micromap_count,
            p_micromaps,
            query_type,
            data_size,
            p_data,
            stride,
        )
    }
    pub unsafe fn write_resource_descriptors_ext_raw(
        &self,
        resource_count: u32,
        p_resources: *const ResourceDescriptorInfoEXT<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .write_resource_descriptors_ext
            .expect("vkWriteResourceDescriptorsEXT is not loaded"))(
            self.handle(),
            resource_count,
            p_resources,
            p_descriptors,
        )
    }
    pub unsafe fn write_sampler_descriptors_ext_raw(
        &self,
        sampler_count: u32,
        p_samplers: *const SamplerCreateInfo<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> VkResult {
        (self
            .fp()
            .write_sampler_descriptors_ext
            .expect("vkWriteSamplerDescriptorsEXT is not loaded"))(
            self.handle(),
            sampler_count,
            p_samplers,
            p_descriptors,
        )
    }
}

impl crate::Entry {
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&core::ffi::CStr>,
    ) -> crate::vk::Result<alloc::vec::Vec<ExtensionProperties<'static>>> {
        let mut count = 0;
        self.enumerate_instance_extension_properties_raw(
            layer_name.map_or(core::ptr::null(), |x| x.as_ptr()),
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<ExtensionProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_instance_extension_properties_raw(
                layer_name.map_or(core::ptr::null(), |x| x.as_ptr()),
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
    ) -> crate::vk::Result<alloc::vec::Vec<LayerProperties<'static>>> {
        let mut count = 0;
        self.enumerate_instance_layer_properties_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<LayerProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result =
                self.enumerate_instance_layer_properties_raw(&mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn enumerate_instance_version(&self) -> crate::vk::Result<u32> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.enumerate_instance_version_raw(value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        name: &core::ffi::CStr,
    ) -> PFN_vkVoidFunction {
        self.get_instance_proc_addr_raw(instance, name.as_ptr())
    }
}

impl crate::Instance {
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> crate::vk::Result<()> {
        self.acquire_drm_display_ext_raw(physical_device, drm_fd, display)
            .into_result()
    }
    #[cfg(feature = "win32")]
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::vk::Result<()> {
        self.acquire_winrt_display_nv_raw(physical_device, display)
            .into_result()
    }
    #[cfg(feature = "xlib-xrandr")]
    pub unsafe fn acquire_xlib_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: &mut Display,
        display: DisplayKHR,
    ) -> crate::vk::Result<()> {
        self.acquire_xlib_display_ext_raw(physical_device, dpy as *mut _, display)
            .into_result()
    }
    #[cfg(feature = "android")]
    pub unsafe fn create_android_surface_khr(
        &self,
        create_info: &AndroidSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_android_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_debug_report_callback_ext(
        &self,
        create_info: &DebugReportCallbackCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DebugReportCallbackEXT> {
        let mut value = core::mem::MaybeUninit::<DebugReportCallbackEXT>::zeroed();
        let result = self.create_debug_report_callback_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_debug_utils_messenger_ext(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DebugUtilsMessengerEXT> {
        let mut value = core::mem::MaybeUninit::<DebugUtilsMessengerEXT>::zeroed();
        let result = self.create_debug_utils_messenger_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "directfb")]
    pub unsafe fn create_direct_fb_surface_ext(
        &self,
        create_info: &DirectFBSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_direct_fb_surface_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_display_mode_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DisplayModeKHR> {
        let mut value = core::mem::MaybeUninit::<DisplayModeKHR>::zeroed();
        let result = self.create_display_mode_khr_raw(
            physical_device,
            display,
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_display_plane_surface_khr(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_display_plane_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_headless_surface_ext(
        &self,
        create_info: &HeadlessSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_headless_surface_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "ios")]
    pub unsafe fn create_ios_surface_mvk(
        &self,
        create_info: &IOSSurfaceCreateInfoMVK<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_ios_surface_mvk_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn create_image_pipe_surface_fuchsia(
        &self,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_image_pipe_surface_fuchsia_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "macos")]
    pub unsafe fn create_mac_os_surface_mvk(
        &self,
        create_info: &MacOSSurfaceCreateInfoMVK<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_mac_os_surface_mvk_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "metal")]
    pub unsafe fn create_metal_surface_ext(
        &self,
        create_info: &MetalSurfaceCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_metal_surface_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "screen")]
    pub unsafe fn create_screen_surface_qnx(
        &self,
        create_info: &ScreenSurfaceCreateInfoQNX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_screen_surface_qnx_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "ggp")]
    pub unsafe fn create_stream_descriptor_surface_ggp(
        &self,
        create_info: &StreamDescriptorSurfaceCreateInfoGGP<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_stream_descriptor_surface_ggp_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "ohos")]
    pub unsafe fn create_surface_ohos(
        &self,
        create_info: &SurfaceCreateInfoOHOS<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_surface_ohos_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "ubm")]
    pub unsafe fn create_ubm_surface_sec(
        &self,
        create_info: &UbmSurfaceCreateInfoSEC<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_ubm_surface_sec_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "vi")]
    pub unsafe fn create_vi_surface_nn(
        &self,
        create_info: &ViSurfaceCreateInfoNN<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_vi_surface_nn_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "wayland")]
    pub unsafe fn create_wayland_surface_khr(
        &self,
        create_info: &WaylandSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_wayland_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn create_win32_surface_khr(
        &self,
        create_info: &Win32SurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_win32_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "xcb")]
    pub unsafe fn create_xcb_surface_khr(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_xcb_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "xlib")]
    pub unsafe fn create_xlib_surface_khr(
        &self,
        create_info: &XlibSurfaceCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SurfaceKHR> {
        let mut value = core::mem::MaybeUninit::<SurfaceKHR>::zeroed();
        let result = self.create_xlib_surface_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn debug_report_message_ext(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: &core::ffi::CStr,
        message: &core::ffi::CStr,
    ) {
        self.debug_report_message_ext_raw(
            flags,
            object_type,
            object,
            location,
            message_code,
            layer_prefix.as_ptr(),
            message.as_ptr(),
        )
    }
    pub unsafe fn destroy_debug_report_callback_ext(
        &self,
        callback: DebugReportCallbackEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_debug_report_callback_ext_raw(
            callback,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_debug_utils_messenger_ext(
        &self,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_debug_utils_messenger_ext_raw(
            messenger,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_surface_khr(
        &self,
        surface: SurfaceKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_surface_khr_raw(
            surface,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&core::ffi::CStr>,
    ) -> crate::vk::Result<alloc::vec::Vec<ExtensionProperties<'static>>> {
        let mut count = 0;
        self.enumerate_device_extension_properties_raw(
            physical_device,
            layer_name.map_or(core::ptr::null(), |x| x.as_ptr()),
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<ExtensionProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_device_extension_properties_raw(
                physical_device,
                layer_name.map_or(core::ptr::null(), |x| x.as_ptr()),
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<LayerProperties<'static>>> {
        let mut count = 0;
        self.enumerate_device_layer_properties_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<LayerProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_device_layer_properties_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_physical_device_groups(
        &self,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDeviceGroupProperties<'static>>> {
        let mut count = 0;
        self.enumerate_physical_device_groups_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDeviceGroupProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_physical_device_groups_raw(&mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn enumerate_physical_device_groups_count(&self) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.enumerate_physical_device_groups_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn enumerate_physical_device_groups_into(
        &self,
        values: &mut [PhysicalDeviceGroupProperties<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.enumerate_physical_device_groups_raw(&mut count, values.as_mut_ptr());
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_physical_device_groups_khr(
        &self,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDeviceGroupProperties<'static>>> {
        let mut count = 0;
        self.enumerate_physical_device_groups_khr_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDeviceGroupProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result =
                self.enumerate_physical_device_groups_khr_raw(&mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn enumerate_physical_device_groups_khr_count(&self) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.enumerate_physical_device_groups_khr_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn enumerate_physical_device_groups_khr_into(
        &self,
        values: &mut [PhysicalDeviceGroupProperties<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.enumerate_physical_device_groups_khr_raw(&mut count, values.as_mut_ptr());
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn enumerate_physical_device_queue_family_performance_counters_by_region_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counter_count: Option<&mut u32>,
        counters: Option<&mut [PerformanceCounterARM<'_>]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionARM<'_>]>,
    ) -> crate::vk::Result<VkResult> {
        let result = self
            .enumerate_physical_device_queue_family_performance_counters_by_region_arm_raw(
                physical_device,
                queue_family_index,
                counter_count.as_ref().map_or(core::ptr::null_mut(), |x| {
                    core::ptr::from_ref(&**x).cast_mut()
                }),
                counters
                    .as_ref()
                    .map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut()),
                counter_descriptions
                    .as_ref()
                    .map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut()),
            );
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn enumerate_physical_device_queue_family_performance_query_counters_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        counter_count: Option<&mut u32>,
        counters: Option<&mut [PerformanceCounterKHR<'_>]>,
        counter_descriptions: Option<&mut [PerformanceCounterDescriptionKHR<'_>]>,
    ) -> crate::vk::Result<VkResult> {
        let result = self
            .enumerate_physical_device_queue_family_performance_query_counters_khr_raw(
                physical_device,
                queue_family_index,
                counter_count.as_ref().map_or(core::ptr::null_mut(), |x| {
                    core::ptr::from_ref(&**x).cast_mut()
                }),
                counters
                    .as_ref()
                    .map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut()),
                counter_descriptions
                    .as_ref()
                    .map_or(core::ptr::null_mut(), |x| x.as_ptr().cast_mut()),
            );
        result.into_result()?;
        Ok(result)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<ShaderInstrumentationMetricDescriptionARM<'static>>>
    {
        let mut count = 0;
        self.enumerate_physical_device_shader_instrumentation_metrics_arm_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<ShaderInstrumentationMetricDescriptionARM<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_physical_device_shader_instrumentation_metrics_arm_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.enumerate_physical_device_shader_instrumentation_metrics_arm_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn enumerate_physical_device_shader_instrumentation_metrics_arm_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [ShaderInstrumentationMetricDescriptionARM<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.enumerate_physical_device_shader_instrumentation_metrics_arm_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn enumerate_physical_devices(
        &self,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDevice>> {
        let mut count = 0;
        self.enumerate_physical_devices_raw(&mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDevice> =
            alloc::vec![PhysicalDevice::null(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.enumerate_physical_devices_raw(&mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, PhysicalDevice::null());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    PhysicalDevice::null(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_device_proc_addr(
        &self,
        device: Device,
        name: &core::ffi::CStr,
    ) -> PFN_vkVoidFunction {
        self.get_device_proc_addr_raw(device, name.as_ptr())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_display_mode_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayModeProperties2KHR<'static>>> {
        let mut count = 0;
        self.get_display_mode_properties2_khr_raw(
            physical_device,
            display,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayModeProperties2KHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_display_mode_properties2_khr_raw(
                physical_device,
                display,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_display_mode_properties2_khr_count(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_display_mode_properties2_khr_raw(
            physical_device,
            display,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_display_mode_properties2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        values: &mut [DisplayModeProperties2KHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_display_mode_properties2_khr_raw(
            physical_device,
            display,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_display_mode_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayModePropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_display_mode_properties_khr_raw(
            physical_device,
            display,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayModePropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_display_mode_properties_khr_raw(
                physical_device,
                display,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_display_plane_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        display_plane_info: &DisplayPlaneInfo2KHR<'_>,
        capabilities: &mut DisplayPlaneCapabilities2KHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_display_plane_capabilities2_khr_raw(
            physical_device,
            display_plane_info as *const _,
            capabilities as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_display_plane_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> crate::vk::Result<DisplayPlaneCapabilitiesKHR<'static>> {
        let mut value = core::mem::MaybeUninit::<DisplayPlaneCapabilitiesKHR<'static>>::zeroed();
        let result = self.get_display_plane_capabilities_khr_raw(
            physical_device,
            mode,
            plane_index,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_display_plane_supported_displays_khr(
        &self,
        physical_device: PhysicalDevice,
        plane_index: u32,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayKHR>> {
        let mut count = 0;
        self.get_display_plane_supported_displays_khr_raw(
            physical_device,
            plane_index,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayKHR> =
            alloc::vec![DisplayKHR::null(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_display_plane_supported_displays_khr_raw(
                physical_device,
                plane_index,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, DisplayKHR::null());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    DisplayKHR::null(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
    ) -> crate::vk::Result<DisplayKHR> {
        let mut value = core::mem::MaybeUninit::<DisplayKHR>::zeroed();
        let result =
            self.get_drm_display_ext_raw(physical_device, drm_fd, connector_id, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<TimeDomainKHR>> {
        let mut count = 0;
        self.get_physical_device_calibrateable_time_domains_ext_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<TimeDomainKHR> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_calibrateable_time_domains_ext_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_calibrateable_time_domains_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<TimeDomainKHR>> {
        let mut count = 0;
        self.get_physical_device_calibrateable_time_domains_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<TimeDomainKHR> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_calibrateable_time_domains_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<CooperativeMatrixFlexibleDimensionsPropertiesNV<'static>>>
    {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<CooperativeMatrixFlexibleDimensionsPropertiesNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self
                .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_raw(
                    physical_device,
                    &mut count,
                    values.as_mut_ptr(),
                );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self
            .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<CooperativeMatrixPropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_properties_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<CooperativeMatrixPropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_cooperative_matrix_properties_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_properties_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_khr_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [CooperativeMatrixPropertiesKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_cooperative_matrix_properties_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<CooperativeMatrixPropertiesNV<'static>>> {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<CooperativeMatrixPropertiesNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_cooperative_matrix_properties_nv_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_cooperative_matrix_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_cooperative_matrix_properties_nv_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [CooperativeMatrixPropertiesNV<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_cooperative_matrix_properties_nv_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<CooperativeVectorPropertiesNV<'static>>> {
        let mut count = 0;
        self.get_physical_device_cooperative_vector_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<CooperativeVectorPropertiesNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_cooperative_vector_properties_nv_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_cooperative_vector_properties_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_cooperative_vector_properties_nv_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [CooperativeVectorPropertiesNV<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_cooperative_vector_properties_nv_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_descriptor_size_ext(
        &self,
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> u64 {
        self.get_physical_device_descriptor_size_ext_raw(physical_device, descriptor_type)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_display_plane_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayPlaneProperties2KHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_display_plane_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayPlaneProperties2KHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_display_plane_properties2_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_display_plane_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_display_plane_properties2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [DisplayPlaneProperties2KHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_display_plane_properties2_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_display_plane_properties_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayPlanePropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_display_plane_properties_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayPlanePropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_display_plane_properties_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_display_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayProperties2KHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_display_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayProperties2KHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_display_properties2_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_display_properties2_khr_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_display_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_display_properties2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [DisplayProperties2KHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_display_properties2_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_display_properties_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<DisplayPropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_display_properties_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DisplayPropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_display_properties_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_external_buffer_properties(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
        external_buffer_properties: &mut ExternalBufferProperties<'_>,
    ) {
        self.get_physical_device_external_buffer_properties_raw(
            physical_device,
            external_buffer_info as *const _,
            external_buffer_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_buffer_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo<'_>,
        external_buffer_properties: &mut ExternalBufferProperties<'_>,
    ) {
        self.get_physical_device_external_buffer_properties_khr_raw(
            physical_device,
            external_buffer_info as *const _,
            external_buffer_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_fence_properties(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
        external_fence_properties: &mut ExternalFenceProperties<'_>,
    ) {
        self.get_physical_device_external_fence_properties_raw(
            physical_device,
            external_fence_info as *const _,
            external_fence_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_fence_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_fence_info: &PhysicalDeviceExternalFenceInfo<'_>,
        external_fence_properties: &mut ExternalFenceProperties<'_>,
    ) {
        self.get_physical_device_external_fence_properties_khr_raw(
            physical_device,
            external_fence_info as *const _,
            external_fence_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_image_format_properties_nv(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::vk::Result<ExternalImageFormatPropertiesNV<'static>> {
        let mut value =
            core::mem::MaybeUninit::<ExternalImageFormatPropertiesNV<'static>>::zeroed();
        let result = self.get_physical_device_external_image_format_properties_nv_raw(
            physical_device,
            format,
            r#type,
            tiling,
            usage,
            flags,
            external_handle_type,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_physical_device_external_semaphore_properties(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
        external_semaphore_properties: &mut ExternalSemaphoreProperties<'_>,
    ) {
        self.get_physical_device_external_semaphore_properties_raw(
            physical_device,
            external_semaphore_info as *const _,
            external_semaphore_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_semaphore_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo<'_>,
        external_semaphore_properties: &mut ExternalSemaphoreProperties<'_>,
    ) {
        self.get_physical_device_external_semaphore_properties_khr_raw(
            physical_device,
            external_semaphore_info as *const _,
            external_semaphore_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_external_tensor_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM<'_>,
        external_tensor_properties: &mut ExternalTensorPropertiesARM<'_>,
    ) {
        self.get_physical_device_external_tensor_properties_arm_raw(
            physical_device,
            external_tensor_info as *const _,
            external_tensor_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures<'static> {
        let mut value = core::mem::MaybeUninit::<PhysicalDeviceFeatures<'static>>::zeroed();
        self.get_physical_device_features_raw(physical_device, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_physical_device_features2(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2<'_>,
    ) {
        self.get_physical_device_features2_raw(physical_device, features as *mut _)
    }
    pub unsafe fn get_physical_device_features2_khr(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures2<'_>,
    ) {
        self.get_physical_device_features2_khr_raw(physical_device, features as *mut _)
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties<'static> {
        let mut value = core::mem::MaybeUninit::<FormatProperties<'static>>::zeroed();
        self.get_physical_device_format_properties_raw(physical_device, format, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_physical_device_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2<'_>,
    ) {
        self.get_physical_device_format_properties2_raw(
            physical_device,
            format,
            format_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties2<'_>,
    ) {
        self.get_physical_device_format_properties2_khr_raw(
            physical_device,
            format,
            format_properties as *mut _,
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_fragment_shading_rates_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDeviceFragmentShadingRateKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_fragment_shading_rates_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDeviceFragmentShadingRateKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_fragment_shading_rates_khr_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_fragment_shading_rates_khr_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_fragment_shading_rates_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_fragment_shading_rates_khr_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [PhysicalDeviceFragmentShadingRateKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_fragment_shading_rates_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::vk::Result<ImageFormatProperties<'static>> {
        let mut value = core::mem::MaybeUninit::<ImageFormatProperties<'static>>::zeroed();
        let result = self.get_physical_device_image_format_properties_raw(
            physical_device,
            format,
            r#type,
            tiling,
            usage,
            flags,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_physical_device_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
        image_format_properties: &mut ImageFormatProperties2<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_image_format_properties2_raw(
            physical_device,
            image_format_info as *const _,
            image_format_properties as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_physical_device_image_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        image_format_info: &PhysicalDeviceImageFormatInfo2<'_>,
        image_format_properties: &mut ImageFormatProperties2<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_image_format_properties2_khr_raw(
            physical_device,
            image_format_info as *const _,
            image_format_properties as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties<'static> {
        let mut value = core::mem::MaybeUninit::<PhysicalDeviceMemoryProperties<'static>>::zeroed();
        self.get_physical_device_memory_properties_raw(physical_device, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_physical_device_memory_properties2(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        self.get_physical_device_memory_properties2_raw(
            physical_device,
            memory_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_memory_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        memory_properties: &mut PhysicalDeviceMemoryProperties2<'_>,
    ) {
        self.get_physical_device_memory_properties2_khr_raw(
            physical_device,
            memory_properties as *mut _,
        )
    }
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        multisample_properties: &mut MultisamplePropertiesEXT<'_>,
    ) {
        self.get_physical_device_multisample_properties_ext_raw(
            physical_device,
            samples,
            multisample_properties as *mut _,
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<OpticalFlowImageFormatPropertiesNV<'static>>> {
        let mut count = 0;
        self.get_physical_device_optical_flow_image_formats_nv_raw(
            physical_device,
            optical_flow_image_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<OpticalFlowImageFormatPropertiesNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_optical_flow_image_formats_nv_raw(
                physical_device,
                optical_flow_image_format_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv_count(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_optical_flow_image_formats_nv_raw(
            physical_device,
            optical_flow_image_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv_into(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV<'_>,
        values: &mut [OpticalFlowImageFormatPropertiesNV<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_optical_flow_image_formats_nv_raw(
            physical_device,
            optical_flow_image_format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_present_rectangles_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<Rect2D<'static>>> {
        let mut count = 0;
        self.get_physical_device_present_rectangles_khr_raw(
            physical_device,
            surface,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<Rect2D<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_present_rectangles_khr_raw(
                physical_device,
                surface,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties<'static> {
        let mut value = core::mem::MaybeUninit::<PhysicalDeviceProperties<'static>>::zeroed();
        self.get_physical_device_properties_raw(physical_device, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_physical_device_properties2(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2<'_>,
    ) {
        self.get_physical_device_properties2_raw(physical_device, properties as *mut _)
    }
    pub unsafe fn get_physical_device_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties2<'_>,
    ) {
        self.get_physical_device_properties2_khr_raw(physical_device, properties as *mut _)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        properties: &mut BaseOutStructure<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_queue_family_data_graph_engine_operation_properties_arm_raw(
            physical_device,
            queue_family_index,
            queue_family_data_graph_properties as *const _,
            properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<DataGraphOpticalFlowImageFormatPropertiesARM<'static>>>
    {
        let mut count = 0;
        self.get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_raw(
            physical_device,
            queue_family_index,
            queue_family_data_graph_properties as *const _,
            optical_flow_image_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DataGraphOpticalFlowImageFormatPropertiesARM<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self
                .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_raw(
                    physical_device,
                    queue_family_index,
                    queue_family_data_graph_properties as *const _,
                    optical_flow_image_format_info as *const _,
                    &mut count,
                    values.as_mut_ptr(),
                );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_count(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_raw(
            physical_device,
            queue_family_index,
            queue_family_data_graph_properties as *const _,
            optical_flow_image_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_into(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM<'_>,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM<'_>,
        values: &mut [DataGraphOpticalFlowImageFormatPropertiesARM<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self
            .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm_raw(
                physical_device,
                queue_family_index,
                queue_family_data_graph_properties as *const _,
                optical_flow_image_format_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
        queue_family_data_graph_processing_engine_properties: &mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
    ) {
        self.get_physical_device_queue_family_data_graph_processing_engine_properties_arm_raw(
            physical_device,
            queue_family_data_graph_processing_engine_info as *const _,
            queue_family_data_graph_processing_engine_properties as *mut _,
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk::Result<alloc::vec::Vec<QueueFamilyDataGraphPropertiesARM<'static>>> {
        let mut count = 0;
        self.get_physical_device_queue_family_data_graph_properties_arm_raw(
            physical_device,
            queue_family_index,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<QueueFamilyDataGraphPropertiesARM<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_queue_family_data_graph_properties_arm_raw(
                physical_device,
                queue_family_index,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm_count(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_queue_family_data_graph_properties_arm_raw(
            physical_device,
            queue_family_index,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm_into(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        values: &mut [QueueFamilyDataGraphPropertiesARM<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_queue_family_data_graph_properties_arm_raw(
            physical_device,
            queue_family_index,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_queue_family_performance_query_passes_khr(
        &self,
        physical_device: PhysicalDevice,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR<'_>,
    ) -> u32 {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        self.get_physical_device_queue_family_performance_query_passes_khr_raw(
            physical_device,
            performance_query_create_info as *const _,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<QueueFamilyProperties<'static>>> {
        let mut count = 0;
        self.get_physical_device_queue_family_properties_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<QueueFamilyProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_queue_family_properties_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_queue_family_properties2(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<QueueFamilyProperties2<'static>>> {
        let mut count = 0;
        self.get_physical_device_queue_family_properties2_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<QueueFamilyProperties2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_queue_family_properties2_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_queue_family_properties2_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> usize {
        let mut count = 0;
        self.get_physical_device_queue_family_properties2_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_physical_device_queue_family_properties2_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [QueueFamilyProperties2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_physical_device_queue_family_properties2_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_queue_family_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<QueueFamilyProperties2<'static>>> {
        let mut count = 0;
        self.get_physical_device_queue_family_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<QueueFamilyProperties2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_queue_family_properties2_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> usize {
        let mut count = 0;
        self.get_physical_device_queue_family_properties2_khr_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_physical_device_queue_family_properties2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [QueueFamilyProperties2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_physical_device_queue_family_properties2_khr_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        r#type: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageFormatProperties<'static>>> {
        let mut count = 0;
        self.get_physical_device_sparse_image_format_properties_raw(
            physical_device,
            format,
            r#type,
            samples,
            usage,
            tiling,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageFormatProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_sparse_image_format_properties_raw(
            physical_device,
            format,
            r#type,
            samples,
            usage,
            tiling,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_sparse_image_format_properties2(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageFormatProperties2<'static>>> {
        let mut count = 0;
        self.get_physical_device_sparse_image_format_properties2_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageFormatProperties2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_sparse_image_format_properties2_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_count(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_physical_device_sparse_image_format_properties2_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_into(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
        values: &mut [SparseImageFormatProperties2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_physical_device_sparse_image_format_properties2_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageFormatProperties2<'static>>> {
        let mut count = 0;
        self.get_physical_device_sparse_image_format_properties2_khr_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageFormatProperties2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_physical_device_sparse_image_format_properties2_khr_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr_count(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_physical_device_sparse_image_format_properties2_khr_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        format_info: &PhysicalDeviceSparseImageFormatInfo2<'_>,
        values: &mut [SparseImageFormatProperties2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_physical_device_sparse_image_format_properties2_khr_raw(
            physical_device,
            format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<FramebufferMixedSamplesCombinationNV<'static>>> {
        let mut count = 0;
        self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<FramebufferMixedSamplesCombinationNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self
                .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_raw(
                    physical_device,
                    &mut count,
                    values.as_mut_ptr(),
                );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [FramebufferMixedSamplesCombinationNV<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilities2EXT<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_surface_capabilities2_ext_raw(
            physical_device,
            surface,
            surface_capabilities as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_physical_device_surface_capabilities2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
        surface_capabilities: &mut SurfaceCapabilities2KHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_surface_capabilities2_khr_raw(
            physical_device,
            surface_info as *const _,
            surface_capabilities as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_physical_device_surface_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<SurfaceCapabilitiesKHR<'static>> {
        let mut value = core::mem::MaybeUninit::<SurfaceCapabilitiesKHR<'static>>::zeroed();
        let result = self.get_physical_device_surface_capabilities_khr_raw(
            physical_device,
            surface,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_surface_formats2_khr(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SurfaceFormat2KHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_surface_formats2_khr_raw(
            physical_device,
            surface_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<SurfaceFormat2KHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_surface_formats2_khr_raw(
                physical_device,
                surface_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_surface_formats2_khr_count(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_surface_formats2_khr_raw(
            physical_device,
            surface_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_surface_formats2_khr_into(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
        values: &mut [SurfaceFormat2KHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_surface_formats2_khr_raw(
            physical_device,
            surface_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_surface_formats_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<SurfaceFormatKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_surface_formats_khr_raw(
            physical_device,
            surface,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<SurfaceFormatKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_surface_formats_khr_raw(
                physical_device,
                surface,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "win32")]
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<PresentModeKHR>> {
        let mut count = 0;
        self.get_physical_device_surface_present_modes2_ext_raw(
            physical_device,
            surface_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PresentModeKHR> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_surface_present_modes2_ext_raw(
                physical_device,
                surface_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_surface_present_modes_khr(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<PresentModeKHR>> {
        let mut count = 0;
        self.get_physical_device_surface_present_modes_khr_raw(
            physical_device,
            surface,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PresentModeKHR> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_surface_present_modes_khr_raw(
                physical_device,
                surface,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_surface_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<bool> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.get_physical_device_surface_support_khr_raw(
            physical_device,
            queue_family_index,
            surface,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init() != 0)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDeviceToolProperties<'static>>> {
        let mut count = 0;
        self.get_physical_device_tool_properties_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDeviceToolProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_tool_properties_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_tool_properties_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_tool_properties_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_tool_properties_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [PhysicalDeviceToolProperties<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_tool_properties_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<alloc::vec::Vec<PhysicalDeviceToolProperties<'static>>> {
        let mut count = 0;
        self.get_physical_device_tool_properties_ext_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PhysicalDeviceToolProperties<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_tool_properties_ext_raw(
                physical_device,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_tool_properties_ext_count(
        &self,
        physical_device: PhysicalDevice,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_tool_properties_ext_raw(
            physical_device,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_tool_properties_ext_into(
        &self,
        physical_device: PhysicalDevice,
        values: &mut [PhysicalDeviceToolProperties<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_tool_properties_ext_raw(
            physical_device,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        video_profile: &VideoProfileInfoKHR<'_>,
        capabilities: &mut VideoCapabilitiesKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_video_capabilities_khr_raw(
            physical_device,
            video_profile as *const _,
            capabilities as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_physical_device_video_encode_quality_level_properties_khr_raw(
            physical_device,
            quality_level_info as *const _,
            quality_level_properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_physical_device_video_format_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<VideoFormatPropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_physical_device_video_format_properties_khr_raw(
            physical_device,
            video_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<VideoFormatPropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_physical_device_video_format_properties_khr_raw(
                physical_device,
                video_format_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_physical_device_video_format_properties_khr_count(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_physical_device_video_format_properties_khr_raw(
            physical_device,
            video_format_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_physical_device_video_format_properties_khr_into(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'_>,
        values: &mut [VideoFormatPropertiesKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_physical_device_video_format_properties_khr_raw(
            physical_device,
            video_format_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32 {
        self.get_physical_device_win32_presentation_support_khr_raw(
            physical_device,
            queue_family_index,
        )
    }
    #[cfg(feature = "xcb")]
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: &mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        self.get_physical_device_xcb_presentation_support_khr_raw(
            physical_device,
            queue_family_index,
            connection as *mut _,
            visual_id,
        )
    }
    #[cfg(feature = "xlib")]
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: &mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        self.get_physical_device_xlib_presentation_support_khr_raw(
            physical_device,
            queue_family_index,
            dpy as *mut _,
            visual_id,
        )
    }
    #[cfg(feature = "xlib-xrandr")]
    pub unsafe fn get_rand_r_output_display_ext(
        &self,
        physical_device: PhysicalDevice,
        dpy: &mut Display,
        rr_output: RROutput,
    ) -> crate::vk::Result<DisplayKHR> {
        let mut value = core::mem::MaybeUninit::<DisplayKHR>::zeroed();
        let result = self.get_rand_r_output_display_ext_raw(
            physical_device,
            dpy as *mut _,
            rr_output,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        device_relative_id: u32,
    ) -> crate::vk::Result<DisplayKHR> {
        let mut value = core::mem::MaybeUninit::<DisplayKHR>::zeroed();
        let result =
            self.get_winrt_display_nv_raw(physical_device, device_relative_id, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn release_display_ext(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::vk::Result<()> {
        self.release_display_ext_raw(physical_device, display)
            .into_result()
    }
    pub unsafe fn submit_debug_utils_message_ext(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT<'_>,
    ) {
        self.submit_debug_utils_message_ext_raw(
            message_severity,
            message_types,
            callback_data as *const _,
        )
    }
}

impl crate::Device {
    #[cfg(feature = "win32")]
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<()> {
        self.acquire_full_screen_exclusive_mode_ext_raw(swapchain)
            .into_result()
    }
    pub unsafe fn acquire_next_image2_khr(
        &self,
        acquire_info: &AcquireNextImageInfoKHR<'_>,
    ) -> crate::vk::Result<Option<(u32, bool)>> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.acquire_next_image2_khr_raw(acquire_info as *const _, value.as_mut_ptr());
        match result {
            VkResult::SUCCESS => Ok(Some((value.assume_init(), false))),
            VkResult::SUBOPTIMAL_KHR => Ok(Some((value.assume_init(), true))),
            VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),
            err => Err(err),
        }
    }
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
    ) -> crate::vk::Result<Option<(u32, bool)>> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.acquire_next_image_khr_raw(
            swapchain,
            timeout,
            semaphore,
            fence,
            value.as_mut_ptr(),
        );
        match result {
            VkResult::SUCCESS => Ok(Some((value.assume_init(), false))),
            VkResult::SUBOPTIMAL_KHR => Ok(Some((value.assume_init(), true))),
            VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),
            err => Err(err),
        }
    }
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL<'_>,
    ) -> crate::vk::Result<PerformanceConfigurationINTEL> {
        let mut value = core::mem::MaybeUninit::<PerformanceConfigurationINTEL>::zeroed();
        let result = self.acquire_performance_configuration_intel_raw(
            acquire_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn acquire_profiling_lock_khr(
        &self,
        info: &AcquireProfilingLockInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.acquire_profiling_lock_khr_raw(info as *const _)
            .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<CommandBuffer>> {
        let mut values =
            alloc::vec![CommandBuffer::null(); allocate_info.command_buffer_count as usize];
        self.allocate_command_buffers_raw(allocate_info as *const _, values.as_mut_ptr())
            .into_result()?;
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn allocate_descriptor_sets(
        &self,
        allocate_info: &DescriptorSetAllocateInfo<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<DescriptorSet>> {
        let mut values =
            alloc::vec![DescriptorSet::null(); allocate_info.descriptor_set_count as usize];
        self.allocate_descriptor_sets_raw(allocate_info as *const _, values.as_mut_ptr())
            .into_result()?;
        Ok(values)
    }
    pub unsafe fn allocate_memory(
        &self,
        allocate_info: &MemoryAllocateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DeviceMemory> {
        let mut value = core::mem::MaybeUninit::<DeviceMemory>::zeroed();
        let result = self.allocate_memory_raw(
            allocate_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn anti_lag_update_amd(&self, data: &AntiLagDataAMD<'_>) {
        self.anti_lag_update_amd_raw(data as *const _)
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.begin_command_buffer_raw(command_buffer, begin_info as *const _)
            .into_result()
    }
    pub unsafe fn bind_acceleration_structure_memory_nv(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_acceleration_structure_memory_nv_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> crate::vk::Result<()> {
        self.bind_buffer_memory_raw(buffer, memory, memory_offset)
            .into_result()
    }
    pub unsafe fn bind_buffer_memory2(
        &self,
        bind_infos: &[BindBufferMemoryInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_buffer_memory2_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_buffer_memory2_khr(
        &self,
        bind_infos: &[BindBufferMemoryInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_buffer_memory2_khr_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_data_graph_pipeline_session_memory_arm(
        &self,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_data_graph_pipeline_session_memory_arm_raw(
            bind_infos.len() as _,
            bind_infos.as_ptr(),
        )
        .into_result()
    }
    pub unsafe fn bind_image_memory(
        &self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> crate::vk::Result<()> {
        self.bind_image_memory_raw(image, memory, memory_offset)
            .into_result()
    }
    pub unsafe fn bind_image_memory2(
        &self,
        bind_infos: &[BindImageMemoryInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_image_memory2_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_image_memory2_khr(
        &self,
        bind_infos: &[BindImageMemoryInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_image_memory2_khr_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> crate::vk::Result<()> {
        self.bind_optical_flow_session_image_nv_raw(session, binding_point, view, layout)
            .into_result()
    }
    pub unsafe fn bind_tensor_memory_arm(
        &self,
        bind_infos: &[BindTensorMemoryInfoARM<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_tensor_memory_arm_raw(bind_infos.len() as _, bind_infos.as_ptr())
            .into_result()
    }
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR<'_>],
    ) -> crate::vk::Result<()> {
        self.bind_video_session_memory_khr_raw(
            video_session,
            bind_session_memory_infos.len() as _,
            bind_session_memory_infos.as_ptr(),
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR<'_>]],
    ) -> crate::vk::Result<()> {
        assert_eq!(
            infos.len(),
            build_range_infos.len(),
            "infos and build_range_infos must have matching info_count lengths"
        );
        for (info, ranges) in infos.iter().zip(build_range_infos.iter()) {
            assert_eq!(
                ranges.len(),
                info.geometry_count as usize,
                "each build_range_infos entry must match the corresponding geometry_count"
            );
        }
        let build_range_info_ptrs = build_range_infos
            .iter()
            .map(|ranges| ranges.as_ptr())
            .collect::<alloc::vec::Vec<_>>();
        self.build_acceleration_structures_khr_raw(
            deferred_operation,
            infos.len() as u32,
            infos.as_ptr(),
            build_range_info_ptrs.as_ptr(),
        )
        .into_result()
    }
    pub unsafe fn build_micromaps_ext(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT<'_>],
    ) -> crate::vk::Result<VkResult> {
        let result =
            self.build_micromaps_ext_raw(deferred_operation, infos.len() as _, infos.as_ptr());
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn clear_shader_instrumentation_metrics_arm(
        &self,
        instrumentation: ShaderInstrumentationARM,
    ) {
        self.clear_shader_instrumentation_metrics_arm_raw(instrumentation)
    }
    pub unsafe fn cmd_begin_conditional_rendering2_ext(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfo2EXT<'_>,
    ) {
        self.cmd_begin_conditional_rendering2_ext_raw(
            command_buffer,
            conditional_rendering_begin as *const _,
        )
    }
    pub unsafe fn cmd_begin_conditional_rendering_ext(
        &self,
        command_buffer: CommandBuffer,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT<'_>,
    ) {
        self.cmd_begin_conditional_rendering_ext_raw(
            command_buffer,
            conditional_rendering_begin as *const _,
        )
    }
    pub unsafe fn cmd_begin_custom_resolve_ext(
        &self,
        command_buffer: CommandBuffer,
        begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT<'_>>,
    ) {
        self.cmd_begin_custom_resolve_ext_raw(
            command_buffer,
            begin_custom_resolve_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_begin_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        self.cmd_begin_debug_utils_label_ext_raw(command_buffer, label_info as *const _)
    }
    pub unsafe fn cmd_begin_per_tile_execution_qcom(
        &self,
        command_buffer: CommandBuffer,
        per_tile_begin_info: &PerTileBeginInfoQCOM<'_>,
    ) {
        self.cmd_begin_per_tile_execution_qcom_raw(command_buffer, per_tile_begin_info as *const _)
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        self.cmd_begin_query_raw(command_buffer, query_pool, query, flags)
    }
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        self.cmd_begin_query_indexed_ext_raw(command_buffer, query_pool, query, flags, index)
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    ) {
        self.cmd_begin_render_pass_raw(command_buffer, render_pass_begin as *const _, contents)
    }
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        subpass_begin_info: &SubpassBeginInfo<'_>,
    ) {
        self.cmd_begin_render_pass2_raw(
            command_buffer,
            render_pass_begin as *const _,
            subpass_begin_info as *const _,
        )
    }
    pub unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        subpass_begin_info: &SubpassBeginInfo<'_>,
    ) {
        self.cmd_begin_render_pass2_khr_raw(
            command_buffer,
            render_pass_begin as *const _,
            subpass_begin_info as *const _,
        )
    }
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        self.cmd_begin_rendering_raw(command_buffer, rendering_info as *const _)
    }
    pub unsafe fn cmd_begin_rendering_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo<'_>,
    ) {
        self.cmd_begin_rendering_khr_raw(command_buffer, rendering_info as *const _)
    }
    pub unsafe fn cmd_begin_shader_instrumentation_arm(
        &self,
        command_buffer: CommandBuffer,
        instrumentation: ShaderInstrumentationARM,
    ) {
        self.cmd_begin_shader_instrumentation_arm_raw(command_buffer, instrumentation)
    }
    pub unsafe fn cmd_begin_transform_feedback2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_infos: Option<&[BindTransformFeedbackBuffer2InfoEXT<'_>]>,
    ) {
        self.cmd_begin_transform_feedback2_ext_raw(
            command_buffer,
            first_counter_range,
            counter_infos.map_or(0, |x| x.len() as _),
            counter_infos.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: Option<&[u64]>,
    ) {
        assert_eq!(
            counter_buffers.len(),
            counter_buffer_offsets.map_or(0, |x| x.len()),
            "counter_buffers and counter_buffer_offsets must have matching counter_buffer_count lengths"
        );
        self.cmd_begin_transform_feedback_ext_raw(
            command_buffer,
            first_counter_buffer,
            counter_buffers.len() as _,
            counter_buffers.as_ptr(),
            counter_buffer_offsets.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR<'_>,
    ) {
        self.cmd_begin_video_coding_khr_raw(command_buffer, begin_info as *const _)
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers2_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT<
            '_,
        >,
    ) {
        self.cmd_bind_descriptor_buffer_embedded_samplers2_ext_raw(
            command_buffer,
            bind_descriptor_buffer_embedded_samplers_info as *const _,
        )
    }
    pub unsafe fn cmd_bind_descriptor_buffer_embedded_samplers_ext(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        self.cmd_bind_descriptor_buffer_embedded_samplers_ext_raw(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
        )
    }
    pub unsafe fn cmd_bind_descriptor_buffers_ext(
        &self,
        command_buffer: CommandBuffer,
        binding_infos: &[DescriptorBufferBindingInfoEXT<'_>],
    ) {
        self.cmd_bind_descriptor_buffers_ext_raw(
            command_buffer,
            binding_infos.len() as _,
            binding_infos.as_ptr(),
        )
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: Option<&[DescriptorSet]>,
        dynamic_offsets: &[u32],
    ) {
        self.cmd_bind_descriptor_sets_raw(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            descriptor_sets.map_or(0, |x| x.len() as _),
            descriptor_sets.map_or(core::ptr::null(), |x| x.as_ptr()),
            dynamic_offsets.len() as _,
            dynamic_offsets.as_ptr(),
        )
    }
    pub unsafe fn cmd_bind_descriptor_sets2(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        self.cmd_bind_descriptor_sets2_raw(command_buffer, bind_descriptor_sets_info as *const _)
    }
    pub unsafe fn cmd_bind_descriptor_sets2_khr(
        &self,
        command_buffer: CommandBuffer,
        bind_descriptor_sets_info: &BindDescriptorSetsInfo<'_>,
    ) {
        self.cmd_bind_descriptor_sets2_khr_raw(
            command_buffer,
            bind_descriptor_sets_info as *const _,
        )
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        index_type: IndexType,
    ) {
        self.cmd_bind_index_buffer_raw(command_buffer, buffer, offset, index_type)
    }
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        size: u64,
        index_type: IndexType,
    ) {
        self.cmd_bind_index_buffer2_raw(command_buffer, buffer, offset, size, index_type)
    }
    pub unsafe fn cmd_bind_index_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        size: u64,
        index_type: IndexType,
    ) {
        self.cmd_bind_index_buffer2_khr_raw(command_buffer, buffer, offset, size, index_type)
    }
    pub unsafe fn cmd_bind_index_buffer3_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &BindIndexBuffer3InfoKHR<'_>,
    ) {
        self.cmd_bind_index_buffer3_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_bind_invocation_mask_huawei(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        self.cmd_bind_invocation_mask_huawei_raw(command_buffer, image_view, image_layout)
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        self.cmd_bind_pipeline_raw(command_buffer, pipeline_bind_point, pipeline)
    }
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        self.cmd_bind_pipeline_shader_group_nv_raw(
            command_buffer,
            pipeline_bind_point,
            pipeline,
            group_index,
        )
    }
    pub unsafe fn cmd_bind_resource_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT<'_>,
    ) {
        self.cmd_bind_resource_heap_ext_raw(command_buffer, bind_info as *const _)
    }
    pub unsafe fn cmd_bind_sampler_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT<'_>,
    ) {
        self.cmd_bind_sampler_heap_ext_raw(command_buffer, bind_info as *const _)
    }
    pub unsafe fn cmd_bind_shaders_ext(
        &self,
        command_buffer: CommandBuffer,
        stages: &[ShaderStageFlagBits],
        shaders: Option<&[ShaderEXT]>,
    ) {
        assert_eq!(
            stages.len(),
            shaders.map_or(0, |x| x.len()),
            "stages and shaders must have matching stage_count lengths"
        );
        self.cmd_bind_shaders_ext_raw(
            command_buffer,
            stages.len() as _,
            stages.as_ptr(),
            shaders.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_bind_shading_rate_image_nv(
        &self,
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ) {
        self.cmd_bind_shading_rate_image_nv_raw(command_buffer, image_view, image_layout)
    }
    pub unsafe fn cmd_bind_tile_memory_qcom(
        &self,
        command_buffer: CommandBuffer,
        tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM<'_>>,
    ) {
        self.cmd_bind_tile_memory_qcom_raw(
            command_buffer,
            tile_memory_bind_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_infos: Option<&[BindTransformFeedbackBuffer2InfoEXT<'_>]>,
    ) {
        self.cmd_bind_transform_feedback_buffers2_ext_raw(
            command_buffer,
            first_binding,
            binding_infos.map_or(0, |x| x.len() as _),
            binding_infos.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[u64],
        sizes: Option<&[u64]>,
    ) {
        assert_eq!(
            buffers.len(),
            offsets.len(),
            "buffers and offsets must have matching binding_count lengths"
        );
        assert_eq!(
            buffers.len(),
            sizes.map_or(0, |x| x.len()),
            "buffers and sizes must have matching binding_count lengths"
        );
        self.cmd_bind_transform_feedback_buffers_ext_raw(
            command_buffer,
            first_binding,
            buffers.len() as _,
            buffers.as_ptr(),
            offsets.as_ptr(),
            sizes.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: Option<&[Buffer]>,
        offsets: &[u64],
    ) {
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            offsets.len(),
            "buffers and offsets must have matching binding_count lengths"
        );
        self.cmd_bind_vertex_buffers_raw(
            command_buffer,
            first_binding,
            buffers.map_or(0, |x| x.len() as _),
            buffers.map_or(core::ptr::null(), |x| x.as_ptr()),
            offsets.as_ptr(),
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: Option<&[Buffer]>,
        offsets: &[u64],
        sizes: Option<&[u64]>,
        strides: Option<&[u64]>,
    ) {
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            offsets.len(),
            "buffers and offsets must have matching binding_count lengths"
        );
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            sizes.map_or(0, |x| x.len()),
            "buffers and sizes must have matching binding_count lengths"
        );
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            strides.map_or(0, |x| x.len()),
            "buffers and strides must have matching binding_count lengths"
        );
        self.cmd_bind_vertex_buffers2_raw(
            command_buffer,
            first_binding,
            buffers.map_or(0, |x| x.len() as _),
            buffers.map_or(core::ptr::null(), |x| x.as_ptr()),
            offsets.as_ptr(),
            sizes.map_or(core::ptr::null(), |x| x.as_ptr()),
            strides.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: Option<&[Buffer]>,
        offsets: &[u64],
        sizes: Option<&[u64]>,
        strides: Option<&[u64]>,
    ) {
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            offsets.len(),
            "buffers and offsets must have matching binding_count lengths"
        );
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            sizes.map_or(0, |x| x.len()),
            "buffers and sizes must have matching binding_count lengths"
        );
        assert_eq!(
            buffers.map_or(0, |x| x.len()),
            strides.map_or(0, |x| x.len()),
            "buffers and strides must have matching binding_count lengths"
        );
        self.cmd_bind_vertex_buffers2_ext_raw(
            command_buffer,
            first_binding,
            buffers.map_or(0, |x| x.len() as _),
            buffers.map_or(core::ptr::null(), |x| x.as_ptr()),
            offsets.as_ptr(),
            sizes.map_or(core::ptr::null(), |x| x.as_ptr()),
            strides.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_bind_vertex_buffers3_khr(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_infos: &[BindVertexBuffer3InfoKHR<'_>],
    ) {
        self.cmd_bind_vertex_buffers3_khr_raw(
            command_buffer,
            first_binding,
            binding_infos.len() as _,
            binding_infos.as_ptr(),
        )
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit<'_>],
        filter: Filter,
    ) {
        self.cmd_blit_image_raw(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as _,
            regions.as_ptr(),
            filter,
        )
    }
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        self.cmd_blit_image2_raw(command_buffer, blit_image_info as *const _)
    }
    pub unsafe fn cmd_blit_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2<'_>,
    ) {
        self.cmd_blit_image2_khr_raw(command_buffer, blit_image_info as *const _)
    }
    pub unsafe fn cmd_build_acceleration_structure_nv(
        &self,
        command_buffer: CommandBuffer,
        info: &AccelerationStructureInfoNV<'_>,
        instance_data: Buffer,
        instance_offset: u64,
        update: bool,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: u64,
    ) {
        self.cmd_build_acceleration_structure_nv_raw(
            command_buffer,
            info as *const _,
            instance_data,
            instance_offset,
            update as u32,
            dst,
            src,
            scratch,
            scratch_offset,
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        indirect_device_addresses: &[u64],
        indirect_strides: &[u32],
        max_primitive_counts: &[&[u32]],
    ) {
        assert_eq!(
            infos.len(),
            indirect_device_addresses.len(),
            "infos and indirect_device_addresses must have matching info_count lengths"
        );
        assert_eq!(
            infos.len(),
            indirect_strides.len(),
            "infos and indirect_strides must have matching info_count lengths"
        );
        assert_eq!(
            infos.len(),
            max_primitive_counts.len(),
            "infos and max_primitive_counts must have matching info_count lengths"
        );
        for (info, counts) in infos.iter().zip(max_primitive_counts.iter()) {
            assert_eq!(
                counts.len(),
                info.geometry_count as usize,
                "each max_primitive_counts entry must match the corresponding geometry_count"
            );
        }
        let max_primitive_count_ptrs = max_primitive_counts
            .iter()
            .map(|counts| counts.as_ptr())
            .collect::<alloc::vec::Vec<_>>();
        self.cmd_build_acceleration_structures_indirect_khr_raw(
            command_buffer,
            infos.len() as u32,
            infos.as_ptr(),
            indirect_device_addresses.as_ptr(),
            indirect_strides.as_ptr(),
            max_primitive_count_ptrs.as_ptr(),
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>],
        build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR<'_>]],
    ) {
        assert_eq!(
            infos.len(),
            build_range_infos.len(),
            "infos and build_range_infos must have matching info_count lengths"
        );
        for (info, ranges) in infos.iter().zip(build_range_infos.iter()) {
            assert_eq!(
                ranges.len(),
                info.geometry_count as usize,
                "each build_range_infos entry must match the corresponding geometry_count"
            );
        }
        let build_range_info_ptrs = build_range_infos
            .iter()
            .map(|ranges| ranges.as_ptr())
            .collect::<alloc::vec::Vec<_>>();
        self.cmd_build_acceleration_structures_khr_raw(
            command_buffer,
            infos.len() as u32,
            infos.as_ptr(),
            build_range_info_ptrs.as_ptr(),
        )
    }
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV<'_>,
    ) {
        self.cmd_build_cluster_acceleration_structure_indirect_nv_raw(
            command_buffer,
            command_infos as *const _,
        )
    }
    pub unsafe fn cmd_build_micromaps_ext(
        &self,
        command_buffer: CommandBuffer,
        infos: &[MicromapBuildInfoEXT<'_>],
    ) {
        self.cmd_build_micromaps_ext_raw(command_buffer, infos.len() as _, infos.as_ptr())
    }
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
        &self,
        command_buffer: CommandBuffer,
        build_info: &BuildPartitionedAccelerationStructureInfoNV<'_>,
    ) {
        self.cmd_build_partitioned_acceleration_structures_nv_raw(
            command_buffer,
            build_info as *const _,
        )
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachments: &[ClearAttachment<'_>],
        rects: &[ClearRect<'_>],
    ) {
        self.cmd_clear_attachments_raw(
            command_buffer,
            attachments.len() as _,
            attachments.as_ptr(),
            rects.len() as _,
            rects.as_ptr(),
        )
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue<'_>,
        ranges: &[ImageSubresourceRange<'_>],
    ) {
        self.cmd_clear_color_image_raw(
            command_buffer,
            image,
            image_layout,
            color as *const _,
            ranges.len() as _,
            ranges.as_ptr(),
        )
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue<'_>,
        ranges: &[ImageSubresourceRange<'_>],
    ) {
        self.cmd_clear_depth_stencil_image_raw(
            command_buffer,
            image,
            image_layout,
            depth_stencil as *const _,
            ranges.len() as _,
            ranges.as_ptr(),
        )
    }
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR<'_>,
    ) {
        self.cmd_control_video_coding_khr_raw(command_buffer, coding_control_info as *const _)
    }
    pub unsafe fn cmd_convert_cooperative_vector_matrix_nv(
        &self,
        command_buffer: CommandBuffer,
        infos: &[ConvertCooperativeVectorMatrixInfoNV<'_>],
    ) {
        self.cmd_convert_cooperative_vector_matrix_nv_raw(
            command_buffer,
            infos.len() as _,
            infos.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR<'_>,
    ) {
        self.cmd_copy_acceleration_structure_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_acceleration_structure_nv(
        &self,
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        self.cmd_copy_acceleration_structure_nv_raw(command_buffer, dst, src, mode)
    }
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) {
        self.cmd_copy_acceleration_structure_to_memory_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy<'_>],
    ) {
        self.cmd_copy_buffer_raw(
            command_buffer,
            src_buffer,
            dst_buffer,
            regions.len() as _,
            regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        self.cmd_copy_buffer2_raw(command_buffer, copy_buffer_info as *const _)
    }
    pub unsafe fn cmd_copy_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2<'_>,
    ) {
        self.cmd_copy_buffer2_khr_raw(command_buffer, copy_buffer_info as *const _)
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy<'_>],
    ) {
        self.cmd_copy_buffer_to_image_raw(
            command_buffer,
            src_buffer,
            dst_image,
            dst_image_layout,
            regions.len() as _,
            regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        self.cmd_copy_buffer_to_image2_raw(command_buffer, copy_buffer_to_image_info as *const _)
    }
    pub unsafe fn cmd_copy_buffer_to_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2<'_>,
    ) {
        self.cmd_copy_buffer_to_image2_khr_raw(
            command_buffer,
            copy_buffer_to_image_info as *const _,
        )
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy<'_>],
    ) {
        self.cmd_copy_image_raw(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as _,
            regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        self.cmd_copy_image2_raw(command_buffer, copy_image_info as *const _)
    }
    pub unsafe fn cmd_copy_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2<'_>,
    ) {
        self.cmd_copy_image2_khr_raw(command_buffer, copy_image_info as *const _)
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy<'_>],
    ) {
        self.cmd_copy_image_to_buffer_raw(
            command_buffer,
            src_image,
            src_image_layout,
            dst_buffer,
            regions.len() as _,
            regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        self.cmd_copy_image_to_buffer2_raw(command_buffer, copy_image_to_buffer_info as *const _)
    }
    pub unsafe fn cmd_copy_image_to_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2<'_>,
    ) {
        self.cmd_copy_image_to_buffer2_khr_raw(
            command_buffer,
            copy_image_to_buffer_info as *const _,
        )
    }
    pub unsafe fn cmd_copy_image_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR<'_>>,
    ) {
        self.cmd_copy_image_to_memory_khr_raw(
            command_buffer,
            copy_memory_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_copy_memory_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR<'_>,
    ) {
        self.cmd_copy_memory_indirect_khr_raw(command_buffer, copy_memory_indirect_info as *const _)
    }
    pub unsafe fn cmd_copy_memory_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: u64,
        copy_count: u32,
        stride: u32,
    ) {
        self.cmd_copy_memory_indirect_nv_raw(
            command_buffer,
            copy_buffer_address,
            copy_count,
            stride,
        )
    }
    pub unsafe fn cmd_copy_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryInfoKHR<'_>>,
    ) {
        self.cmd_copy_memory_khr_raw(
            command_buffer,
            copy_memory_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) {
        self.cmd_copy_memory_to_acceleration_structure_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR<'_>,
    ) {
        self.cmd_copy_memory_to_image_indirect_khr_raw(
            command_buffer,
            copy_memory_to_image_indirect_info as *const _,
        )
    }
    pub unsafe fn cmd_copy_memory_to_image_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_address: u64,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers<'_>],
    ) {
        self.cmd_copy_memory_to_image_indirect_nv_raw(
            command_buffer,
            copy_buffer_address,
            image_subresources.len() as _,
            stride,
            dst_image,
            dst_image_layout,
            image_subresources.as_ptr(),
        )
    }
    pub unsafe fn cmd_copy_memory_to_image_khr(
        &self,
        command_buffer: CommandBuffer,
        copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR<'_>>,
    ) {
        self.cmd_copy_memory_to_image_khr_raw(
            command_buffer,
            copy_memory_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_copy_memory_to_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToMicromapInfoEXT<'_>,
    ) {
        self.cmd_copy_memory_to_micromap_ext_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapInfoEXT<'_>,
    ) {
        self.cmd_copy_micromap_ext_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_micromap_to_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapToMemoryInfoEXT<'_>,
    ) {
        self.cmd_copy_micromap_to_memory_ext_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: u64,
        stride: u64,
        flags: QueryResultFlags,
    ) {
        self.cmd_copy_query_pool_results_raw(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_buffer,
            dst_offset,
            stride,
            flags,
        )
    }
    pub unsafe fn cmd_copy_query_pool_results_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_range: &StridedDeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        self.cmd_copy_query_pool_results_to_memory_khr_raw(
            command_buffer,
            query_pool,
            first_query,
            query_count,
            dst_range as *const _,
            dst_flags,
            query_result_flags,
        )
    }
    pub unsafe fn cmd_copy_tensor_arm(
        &self,
        command_buffer: CommandBuffer,
        copy_tensor_info: &CopyTensorInfoARM<'_>,
    ) {
        self.cmd_copy_tensor_arm_raw(command_buffer, copy_tensor_info as *const _)
    }
    pub unsafe fn cmd_cu_launch_kernel_nvx(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CuLaunchInfoNVX<'_>,
    ) {
        self.cmd_cu_launch_kernel_nvx_raw(command_buffer, launch_info as *const _)
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_cuda_launch_kernel_nv(
        &self,
        command_buffer: CommandBuffer,
        launch_info: &CudaLaunchInfoNV<'_>,
    ) {
        self.cmd_cuda_launch_kernel_nv_raw(command_buffer, launch_info as *const _)
    }
    pub unsafe fn cmd_debug_marker_begin_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        self.cmd_debug_marker_begin_ext_raw(command_buffer, marker_info as *const _)
    }
    pub unsafe fn cmd_debug_marker_end_ext(&self, command_buffer: CommandBuffer) {
        self.cmd_debug_marker_end_ext_raw(command_buffer)
    }
    pub unsafe fn cmd_debug_marker_insert_ext(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &DebugMarkerMarkerInfoEXT<'_>,
    ) {
        self.cmd_debug_marker_insert_ext_raw(command_buffer, marker_info as *const _)
    }
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        decode_info: &VideoDecodeInfoKHR<'_>,
    ) {
        self.cmd_decode_video_khr_raw(command_buffer, decode_info as *const _)
    }
    pub unsafe fn cmd_decompress_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_info_ext: &DecompressMemoryInfoEXT<'_>,
    ) {
        self.cmd_decompress_memory_ext_raw(command_buffer, decompress_memory_info_ext as *const _)
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_ext(
        &self,
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: u64,
        indirect_commands_count_address: u64,
        max_decompression_count: u32,
        stride: u32,
    ) {
        self.cmd_decompress_memory_indirect_count_ext_raw(
            command_buffer,
            decompression_method,
            indirect_commands_address,
            indirect_commands_count_address,
            max_decompression_count,
            stride,
        )
    }
    pub unsafe fn cmd_decompress_memory_indirect_count_nv(
        &self,
        command_buffer: CommandBuffer,
        indirect_commands_address: u64,
        indirect_commands_count_address: u64,
        stride: u32,
    ) {
        self.cmd_decompress_memory_indirect_count_nv_raw(
            command_buffer,
            indirect_commands_address,
            indirect_commands_count_address,
            stride,
        )
    }
    pub unsafe fn cmd_decompress_memory_nv(
        &self,
        command_buffer: CommandBuffer,
        decompress_memory_regions: &[DecompressMemoryRegionNV<'_>],
    ) {
        self.cmd_decompress_memory_nv_raw(
            command_buffer,
            decompress_memory_regions.len() as _,
            decompress_memory_regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        self.cmd_dispatch_raw(command_buffer, group_count_x, group_count_y, group_count_z)
    }
    pub unsafe fn cmd_dispatch_base(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        self.cmd_dispatch_base_raw(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_dispatch_base_khr(
        &self,
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        self.cmd_dispatch_base_khr_raw(
            command_buffer,
            base_group_x,
            base_group_y,
            base_group_z,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_dispatch_data_graph_arm(
        &self,
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM<'_>>,
    ) {
        self.cmd_dispatch_data_graph_arm_raw(
            command_buffer,
            session,
            info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        count_info: &DispatchGraphCountInfoAMDX<'_>,
    ) {
        self.cmd_dispatch_graph_amdx_raw(
            command_buffer,
            scratch,
            scratch_size,
            count_info as *const _,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_indirect_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        count_info: &DispatchGraphCountInfoAMDX<'_>,
    ) {
        self.cmd_dispatch_graph_indirect_amdx_raw(
            command_buffer,
            scratch,
            scratch_size,
            count_info as *const _,
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_dispatch_graph_indirect_count_amdx(
        &self,
        command_buffer: CommandBuffer,
        scratch: u64,
        scratch_size: u64,
        count_info: u64,
    ) {
        self.cmd_dispatch_graph_indirect_count_amdx_raw(
            command_buffer,
            scratch,
            scratch_size,
            count_info,
        )
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
    ) {
        self.cmd_dispatch_indirect_raw(command_buffer, buffer, offset)
    }
    pub unsafe fn cmd_dispatch_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &DispatchIndirect2InfoKHR<'_>,
    ) {
        self.cmd_dispatch_indirect2_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_dispatch_tile_qcom(
        &self,
        command_buffer: CommandBuffer,
        dispatch_tile_info: &DispatchTileInfoQCOM<'_>,
    ) {
        self.cmd_dispatch_tile_qcom_raw(command_buffer, dispatch_tile_info as *const _)
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        self.cmd_draw_raw(
            command_buffer,
            vertex_count,
            instance_count,
            first_vertex,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_cluster_huawei(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        self.cmd_draw_cluster_huawei_raw(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_draw_cluster_indirect_huawei(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
    ) {
        self.cmd_draw_cluster_indirect_huawei_raw(command_buffer, buffer, offset)
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        self.cmd_draw_indexed_raw(
            command_buffer,
            index_count,
            instance_count,
            first_index,
            vertex_offset,
            first_instance,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indexed_indirect_raw(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_draw_indexed_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        self.cmd_draw_indexed_indirect2_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indexed_indirect_count_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count2_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        self.cmd_draw_indexed_indirect_count2_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indexed_indirect_count_amd_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indexed_indirect_count_khr_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indirect_raw(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_draw_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        self.cmd_draw_indirect2_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_indirect_byte_count2_ext(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_info: &BindTransformFeedbackBuffer2InfoEXT<'_>,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        self.cmd_draw_indirect_byte_count2_ext_raw(
            command_buffer,
            instance_count,
            first_instance,
            counter_info as *const _,
            counter_offset,
            vertex_stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: u64,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        self.cmd_draw_indirect_byte_count_ext_raw(
            command_buffer,
            instance_count,
            first_instance,
            counter_buffer,
            counter_buffer_offset,
            counter_offset,
            vertex_stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indirect_count_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count2_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        self.cmd_draw_indirect_count2_khr_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_indirect_count_amd(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indirect_count_amd_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_indirect_count_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_indirect_count_khr_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        self.cmd_draw_mesh_tasks_ext_raw(
            command_buffer,
            group_count_x,
            group_count_y,
            group_count_z,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect2_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirect2InfoKHR<'_>,
    ) {
        self.cmd_draw_mesh_tasks_indirect2_ext_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count2_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &DrawIndirectCount2InfoKHR<'_>,
    ) {
        self.cmd_draw_mesh_tasks_indirect_count2_ext_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_mesh_tasks_indirect_count_ext_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        count_buffer: Buffer,
        count_buffer_offset: u64,
        max_draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_mesh_tasks_indirect_count_nv_raw(
            command_buffer,
            buffer,
            offset,
            count_buffer,
            count_buffer_offset,
            max_draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_mesh_tasks_indirect_ext_raw(
            command_buffer,
            buffer,
            offset,
            draw_count,
            stride,
        )
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: u64,
        draw_count: u32,
        stride: u32,
    ) {
        self.cmd_draw_mesh_tasks_indirect_nv_raw(command_buffer, buffer, offset, draw_count, stride)
    }
    pub unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        self.cmd_draw_mesh_tasks_nv_raw(command_buffer, task_count, first_task)
    }
    pub unsafe fn cmd_draw_multi_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_info: &[MultiDrawInfoEXT<'_>],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        self.cmd_draw_multi_ext_raw(
            command_buffer,
            vertex_info.len() as _,
            vertex_info.as_ptr(),
            instance_count,
            first_instance,
            stride,
        )
    }
    pub unsafe fn cmd_draw_multi_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        index_info: &[MultiDrawIndexedInfoEXT<'_>],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        self.cmd_draw_multi_indexed_ext_raw(
            command_buffer,
            index_info.len() as _,
            index_info.as_ptr(),
            instance_count,
            first_instance,
            stride,
            vertex_offset.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_encode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        encode_info: &VideoEncodeInfoKHR<'_>,
    ) {
        self.cmd_encode_video_khr_raw(command_buffer, encode_info as *const _)
    }
    pub unsafe fn cmd_end_conditional_rendering_ext(&self, command_buffer: CommandBuffer) {
        self.cmd_end_conditional_rendering_ext_raw(command_buffer)
    }
    pub unsafe fn cmd_end_debug_utils_label_ext(&self, command_buffer: CommandBuffer) {
        self.cmd_end_debug_utils_label_ext_raw(command_buffer)
    }
    pub unsafe fn cmd_end_per_tile_execution_qcom(
        &self,
        command_buffer: CommandBuffer,
        per_tile_end_info: &PerTileEndInfoQCOM<'_>,
    ) {
        self.cmd_end_per_tile_execution_qcom_raw(command_buffer, per_tile_end_info as *const _)
    }
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        self.cmd_end_query_raw(command_buffer, query_pool, query)
    }
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ) {
        self.cmd_end_query_indexed_ext_raw(command_buffer, query_pool, query, index)
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        self.cmd_end_render_pass_raw(command_buffer)
    }
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        self.cmd_end_render_pass2_raw(command_buffer, subpass_end_info as *const _)
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        self.cmd_end_render_pass2_khr_raw(command_buffer, subpass_end_info as *const _)
    }
    pub unsafe fn cmd_end_rendering(&self, command_buffer: CommandBuffer) {
        self.cmd_end_rendering_raw(command_buffer)
    }
    pub unsafe fn cmd_end_rendering2_ext(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR<'_>>,
    ) {
        self.cmd_end_rendering2_ext_raw(
            command_buffer,
            rendering_end_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_end_rendering2_khr(
        &self,
        command_buffer: CommandBuffer,
        rendering_end_info: Option<&RenderingEndInfoKHR<'_>>,
    ) {
        self.cmd_end_rendering2_khr_raw(
            command_buffer,
            rendering_end_info.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_end_rendering_khr(&self, command_buffer: CommandBuffer) {
        self.cmd_end_rendering_khr_raw(command_buffer)
    }
    pub unsafe fn cmd_end_shader_instrumentation_arm(&self, command_buffer: CommandBuffer) {
        self.cmd_end_shader_instrumentation_arm_raw(command_buffer)
    }
    pub unsafe fn cmd_end_transform_feedback2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_infos: Option<&[BindTransformFeedbackBuffer2InfoEXT<'_>]>,
    ) {
        self.cmd_end_transform_feedback2_ext_raw(
            command_buffer,
            first_counter_range,
            counter_infos.map_or(0, |x| x.len() as _),
            counter_infos.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: Option<&[u64]>,
    ) {
        assert_eq!(
            counter_buffers.len(),
            counter_buffer_offsets.map_or(0, |x| x.len()),
            "counter_buffers and counter_buffer_offsets must have matching counter_buffer_count lengths"
        );
        self.cmd_end_transform_feedback_ext_raw(
            command_buffer,
            first_counter_buffer,
            counter_buffers.len() as _,
            counter_buffers.as_ptr(),
            counter_buffer_offsets.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR<'_>,
    ) {
        self.cmd_end_video_coding_khr_raw(command_buffer, end_coding_info as *const _)
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffers: &[CommandBuffer],
    ) {
        self.cmd_execute_commands_raw(
            command_buffer,
            command_buffers.len() as _,
            command_buffers.as_ptr(),
        )
    }
    pub unsafe fn cmd_execute_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
    ) {
        self.cmd_execute_generated_commands_ext_raw(
            command_buffer,
            is_preprocessed as u32,
            generated_commands_info as *const _,
        )
    }
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        self.cmd_execute_generated_commands_nv_raw(
            command_buffer,
            is_preprocessed as u32,
            generated_commands_info as *const _,
        )
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: u64,
        size: u64,
        data: u32,
    ) {
        self.cmd_fill_buffer_raw(command_buffer, dst_buffer, dst_offset, size, data)
    }
    pub unsafe fn cmd_fill_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        dst_range: &DeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        self.cmd_fill_memory_khr_raw(command_buffer, dst_range as *const _, dst_flags, data)
    }
    #[cfg(feature = "beta")]
    pub unsafe fn cmd_initialize_graph_scratch_memory_amdx(
        &self,
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: u64,
        scratch_size: u64,
    ) {
        self.cmd_initialize_graph_scratch_memory_amdx_raw(
            command_buffer,
            execution_graph,
            scratch,
            scratch_size,
        )
    }
    pub unsafe fn cmd_insert_debug_utils_label_ext(
        &self,
        command_buffer: CommandBuffer,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        self.cmd_insert_debug_utils_label_ext_raw(command_buffer, label_info as *const _)
    }
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        self.cmd_next_subpass_raw(command_buffer, contents)
    }
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo<'_>,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        self.cmd_next_subpass2_raw(
            command_buffer,
            subpass_begin_info as *const _,
            subpass_end_info as *const _,
        )
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo<'_>,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        self.cmd_next_subpass2_khr_raw(
            command_buffer,
            subpass_begin_info as *const _,
            subpass_end_info as *const _,
        )
    }
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV<'_>,
    ) {
        self.cmd_optical_flow_execute_nv_raw(command_buffer, session, execute_info as *const _)
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        self.cmd_pipeline_barrier_raw(
            command_buffer,
            src_stage_mask,
            dst_stage_mask,
            dependency_flags,
            memory_barriers.len() as _,
            memory_barriers.as_ptr(),
            buffer_memory_barriers.len() as _,
            buffer_memory_barriers.as_ptr(),
            image_memory_barriers.len() as _,
            image_memory_barriers.as_ptr(),
        )
    }
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        self.cmd_pipeline_barrier2_raw(command_buffer, dependency_info as *const _)
    }
    pub unsafe fn cmd_pipeline_barrier2_khr(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo<'_>,
    ) {
        self.cmd_pipeline_barrier2_khr_raw(command_buffer, dependency_info as *const _)
    }
    pub unsafe fn cmd_preprocess_generated_commands_ext(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoEXT<'_>,
        state_command_buffer: CommandBuffer,
    ) {
        self.cmd_preprocess_generated_commands_ext_raw(
            command_buffer,
            generated_commands_info as *const _,
            state_command_buffer,
        )
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        self.cmd_preprocess_generated_commands_nv_raw(
            command_buffer,
            generated_commands_info as *const _,
        )
    }
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[u8],
    ) {
        self.cmd_push_constants_raw(
            command_buffer,
            layout,
            stage_flags,
            offset,
            values.len() as _,
            values.as_ptr().cast(),
        )
    }
    pub unsafe fn cmd_push_constants2(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        self.cmd_push_constants2_raw(command_buffer, push_constants_info as *const _)
    }
    pub unsafe fn cmd_push_constants2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_constants_info: &PushConstantsInfo<'_>,
    ) {
        self.cmd_push_constants2_khr_raw(command_buffer, push_constants_info as *const _)
    }
    pub unsafe fn cmd_push_data_ext(
        &self,
        command_buffer: CommandBuffer,
        push_data_info: &PushDataInfoEXT<'_>,
    ) {
        self.cmd_push_data_ext_raw(command_buffer, push_data_info as *const _)
    }
    pub unsafe fn cmd_push_descriptor_set(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        self.cmd_push_descriptor_set_raw(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_writes.len() as _,
            descriptor_writes.as_ptr(),
        )
    }
    pub unsafe fn cmd_push_descriptor_set2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        self.cmd_push_descriptor_set2_raw(command_buffer, push_descriptor_set_info as *const _)
    }
    pub unsafe fn cmd_push_descriptor_set2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_info: &PushDescriptorSetInfo<'_>,
    ) {
        self.cmd_push_descriptor_set2_khr_raw(command_buffer, push_descriptor_set_info as *const _)
    }
    pub unsafe fn cmd_push_descriptor_set_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet<'_>],
    ) {
        self.cmd_push_descriptor_set_khr_raw(
            command_buffer,
            pipeline_bind_point,
            layout,
            set,
            descriptor_writes.len() as _,
            descriptor_writes.as_ptr(),
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        self.cmd_push_descriptor_set_with_template_raw(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            data as *const _,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        self.cmd_push_descriptor_set_with_template2_raw(
            command_buffer,
            push_descriptor_set_with_template_info as *const _,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template2_khr(
        &self,
        command_buffer: CommandBuffer,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo<'_>,
    ) {
        self.cmd_push_descriptor_set_with_template2_khr_raw(
            command_buffer,
            push_descriptor_set_with_template_info as *const _,
        )
    }
    pub unsafe fn cmd_push_descriptor_set_with_template_khr(
        &self,
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        self.cmd_push_descriptor_set_with_template_khr_raw(
            command_buffer,
            descriptor_update_template,
            layout,
            set,
            data as *const _,
        )
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        self.cmd_reset_event_raw(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        self.cmd_reset_event2_raw(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        self.cmd_reset_event2_khr_raw(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        self.cmd_reset_query_pool_raw(command_buffer, query_pool, first_query, query_count)
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve<'_>],
    ) {
        self.cmd_resolve_image_raw(
            command_buffer,
            src_image,
            src_image_layout,
            dst_image,
            dst_image_layout,
            regions.len() as _,
            regions.as_ptr(),
        )
    }
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        self.cmd_resolve_image2_raw(command_buffer, resolve_image_info as *const _)
    }
    pub unsafe fn cmd_resolve_image2_khr(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2<'_>,
    ) {
        self.cmd_resolve_image2_khr_raw(command_buffer, resolve_image_info as *const _)
    }
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) {
        self.cmd_set_alpha_to_coverage_enable_ext_raw(
            command_buffer,
            alpha_to_coverage_enable as u32,
        )
    }
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_one_enable: bool,
    ) {
        self.cmd_set_alpha_to_one_enable_ext_raw(command_buffer, alpha_to_one_enable as u32)
    }
    pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        aspect_mask: ImageAspectFlags,
    ) {
        self.cmd_set_attachment_feedback_loop_enable_ext_raw(command_buffer, aspect_mask)
    }
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: [f32; 4],
    ) {
        self.cmd_set_blend_constants_raw(command_buffer, blend_constants)
    }
    pub unsafe fn cmd_set_checkpoint_nv(
        &self,
        command_buffer: CommandBuffer,
        checkpoint_marker: &c_void,
    ) {
        self.cmd_set_checkpoint_nv_raw(command_buffer, checkpoint_marker as *const _)
    }
    pub unsafe fn cmd_set_coarse_sample_order_nv(
        &self,
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV<'_>],
    ) {
        self.cmd_set_coarse_sample_order_nv_raw(
            command_buffer,
            sample_order_type,
            custom_sample_orders.len() as _,
            custom_sample_orders.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT<'_>],
    ) {
        self.cmd_set_color_blend_advanced_ext_raw(
            command_buffer,
            first_attachment,
            color_blend_advanced.len() as _,
            color_blend_advanced.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_color_blend_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_enables: &[Bool32],
    ) {
        self.cmd_set_color_blend_enable_ext_raw(
            command_buffer,
            first_attachment,
            color_blend_enables.len() as _,
            color_blend_enables.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_color_blend_equation_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT<'_>],
    ) {
        self.cmd_set_color_blend_equation_ext_raw(
            command_buffer,
            first_attachment,
            color_blend_equations.len() as _,
            color_blend_equations.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_color_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        color_write_enables: &[Bool32],
    ) {
        self.cmd_set_color_write_enable_ext_raw(
            command_buffer,
            color_write_enables.len() as _,
            color_write_enables.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_color_write_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_write_masks: Option<&[ColorComponentFlags]>,
    ) {
        self.cmd_set_color_write_mask_ext_raw(
            command_buffer,
            first_attachment,
            color_write_masks.map_or(0, |x| x.len() as _),
            color_write_masks.map_or(core::ptr::null(), |x| x.as_ptr()),
        )
    }
    pub unsafe fn cmd_set_compute_occupancy_priority_nv(
        &self,
        command_buffer: CommandBuffer,
        parameters: &ComputeOccupancyPriorityParametersNV<'_>,
    ) {
        self.cmd_set_compute_occupancy_priority_nv_raw(command_buffer, parameters as *const _)
    }
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        self.cmd_set_conservative_rasterization_mode_ext_raw(
            command_buffer,
            conservative_rasterization_mode,
        )
    }
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        self.cmd_set_coverage_modulation_mode_nv_raw(command_buffer, coverage_modulation_mode)
    }
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) {
        self.cmd_set_coverage_modulation_table_enable_nv_raw(
            command_buffer,
            coverage_modulation_table_enable as u32,
        )
    }
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table: &[f32],
    ) {
        self.cmd_set_coverage_modulation_table_nv_raw(
            command_buffer,
            coverage_modulation_table.len() as _,
            coverage_modulation_table.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        self.cmd_set_coverage_reduction_mode_nv_raw(command_buffer, coverage_reduction_mode)
    }
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_enable: bool,
    ) {
        self.cmd_set_coverage_to_color_enable_nv_raw(
            command_buffer,
            coverage_to_color_enable as u32,
        )
    }
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        self.cmd_set_coverage_to_color_location_nv_raw(command_buffer, coverage_to_color_location)
    }
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        self.cmd_set_cull_mode_raw(command_buffer, cull_mode)
    }
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        self.cmd_set_cull_mode_ext_raw(command_buffer, cull_mode)
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        self.cmd_set_depth_bias_raw(
            command_buffer,
            depth_bias_constant_factor,
            depth_bias_clamp,
            depth_bias_slope_factor,
        )
    }
    pub unsafe fn cmd_set_depth_bias2_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_info: &DepthBiasInfoEXT<'_>,
    ) {
        self.cmd_set_depth_bias2_ext_raw(command_buffer, depth_bias_info as *const _)
    }
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        self.cmd_set_depth_bias_enable_raw(command_buffer, depth_bias_enable as u32)
    }
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        self.cmd_set_depth_bias_enable_ext_raw(command_buffer, depth_bias_enable as u32)
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        self.cmd_set_depth_bounds_raw(command_buffer, min_depth_bounds, max_depth_bounds)
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        self.cmd_set_depth_bounds_test_enable_raw(command_buffer, depth_bounds_test_enable as u32)
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        self.cmd_set_depth_bounds_test_enable_ext_raw(
            command_buffer,
            depth_bounds_test_enable as u32,
        )
    }
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_enable: bool,
    ) {
        self.cmd_set_depth_clamp_enable_ext_raw(command_buffer, depth_clamp_enable as u32)
    }
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT<'_>>,
    ) {
        self.cmd_set_depth_clamp_range_ext_raw(
            command_buffer,
            depth_clamp_mode,
            depth_clamp_range.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: bool,
    ) {
        self.cmd_set_depth_clip_enable_ext_raw(command_buffer, depth_clip_enable as u32)
    }
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: bool,
    ) {
        self.cmd_set_depth_clip_negative_one_to_one_ext_raw(
            command_buffer,
            negative_one_to_one as u32,
        )
    }
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        self.cmd_set_depth_compare_op_raw(command_buffer, depth_compare_op)
    }
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        self.cmd_set_depth_compare_op_ext_raw(command_buffer, depth_compare_op)
    }
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: bool,
    ) {
        self.cmd_set_depth_test_enable_raw(command_buffer, depth_test_enable as u32)
    }
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: bool,
    ) {
        self.cmd_set_depth_test_enable_ext_raw(command_buffer, depth_test_enable as u32)
    }
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: bool,
    ) {
        self.cmd_set_depth_write_enable_raw(command_buffer, depth_write_enable as u32)
    }
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: bool,
    ) {
        self.cmd_set_depth_write_enable_ext_raw(command_buffer, depth_write_enable as u32)
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets2_ext(
        &self,
        command_buffer: CommandBuffer,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT<'_>,
    ) {
        self.cmd_set_descriptor_buffer_offsets2_ext_raw(
            command_buffer,
            set_descriptor_buffer_offsets_info as *const _,
        )
    }
    pub unsafe fn cmd_set_descriptor_buffer_offsets_ext(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[u64],
    ) {
        assert_eq!(
            buffer_indices.len(),
            offsets.len(),
            "buffer_indices and offsets must have matching set_count lengths"
        );
        self.cmd_set_descriptor_buffer_offsets_ext_raw(
            command_buffer,
            pipeline_bind_point,
            layout,
            first_set,
            buffer_indices.len() as _,
            buffer_indices.as_ptr(),
            offsets.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_device_mask(&self, command_buffer: CommandBuffer, device_mask: u32) {
        self.cmd_set_device_mask_raw(command_buffer, device_mask)
    }
    pub unsafe fn cmd_set_device_mask_khr(&self, command_buffer: CommandBuffer, device_mask: u32) {
        self.cmd_set_device_mask_khr_raw(command_buffer, device_mask)
    }
    pub unsafe fn cmd_set_discard_rectangle_ext(
        &self,
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangles: &[Rect2D<'_>],
    ) {
        self.cmd_set_discard_rectangle_ext_raw(
            command_buffer,
            first_discard_rectangle,
            discard_rectangles.len() as _,
            discard_rectangles.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_discard_rectangle_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_enable: bool,
    ) {
        self.cmd_set_discard_rectangle_enable_ext_raw(
            command_buffer,
            discard_rectangle_enable as u32,
        )
    }
    pub unsafe fn cmd_set_discard_rectangle_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ) {
        self.cmd_set_discard_rectangle_mode_ext_raw(command_buffer, discard_rectangle_mode)
    }
    pub unsafe fn cmd_set_dispatch_parameters_arm(
        &self,
        command_buffer: CommandBuffer,
        dispatch_parameters: &DispatchParametersARM<'_>,
    ) {
        self.cmd_set_dispatch_parameters_arm_raw(command_buffer, dispatch_parameters as *const _)
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        self.cmd_set_event_raw(command_buffer, event, stage_mask)
    }
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        self.cmd_set_event2_raw(command_buffer, event, dependency_info as *const _)
    }
    pub unsafe fn cmd_set_event2_khr(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo<'_>,
    ) {
        self.cmd_set_event2_khr_raw(command_buffer, event, dependency_info as *const _)
    }
    pub unsafe fn cmd_set_exclusive_scissor_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_enables: &[Bool32],
    ) {
        self.cmd_set_exclusive_scissor_enable_nv_raw(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissor_enables.len() as _,
            exclusive_scissor_enables.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_exclusive_scissor_nv(
        &self,
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissors: &[Rect2D<'_>],
    ) {
        self.cmd_set_exclusive_scissor_nv_raw(
            command_buffer,
            first_exclusive_scissor,
            exclusive_scissors.len() as _,
            exclusive_scissors.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        self.cmd_set_extra_primitive_overestimation_size_ext_raw(
            command_buffer,
            extra_primitive_overestimation_size,
        )
    }
    pub unsafe fn cmd_set_fragment_shading_rate_enum_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) {
        self.cmd_set_fragment_shading_rate_enum_nv_raw(command_buffer, shading_rate, combiner_ops)
    }
    pub unsafe fn cmd_set_fragment_shading_rate_khr(
        &self,
        command_buffer: CommandBuffer,
        fragment_size: &Extent2D<'_>,
        combiner_ops: [FragmentShadingRateCombinerOpKHR; 2],
    ) {
        self.cmd_set_fragment_shading_rate_khr_raw(
            command_buffer,
            fragment_size as *const _,
            combiner_ops,
        )
    }
    pub unsafe fn cmd_set_front_face(&self, command_buffer: CommandBuffer, front_face: FrontFace) {
        self.cmd_set_front_face_raw(command_buffer, front_face)
    }
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        self.cmd_set_front_face_ext_raw(command_buffer, front_face)
    }
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationMode,
    ) {
        self.cmd_set_line_rasterization_mode_ext_raw(command_buffer, line_rasterization_mode)
    }
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        self.cmd_set_line_stipple_raw(command_buffer, line_stipple_factor, line_stipple_pattern)
    }
    pub unsafe fn cmd_set_line_stipple_ext(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        self.cmd_set_line_stipple_ext_raw(command_buffer, line_stipple_factor, line_stipple_pattern)
    }
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: bool,
    ) {
        self.cmd_set_line_stipple_enable_ext_raw(command_buffer, stippled_line_enable as u32)
    }
    pub unsafe fn cmd_set_line_stipple_khr(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        self.cmd_set_line_stipple_khr_raw(command_buffer, line_stipple_factor, line_stipple_pattern)
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBuffer, line_width: f32) {
        self.cmd_set_line_width_raw(command_buffer, line_width)
    }
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        self.cmd_set_logic_op_ext_raw(command_buffer, logic_op)
    }
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        logic_op_enable: bool,
    ) {
        self.cmd_set_logic_op_enable_ext_raw(command_buffer, logic_op_enable as u32)
    }
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        self.cmd_set_patch_control_points_ext_raw(command_buffer, patch_control_points)
    }
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL<'_>,
    ) -> crate::vk::Result<()> {
        self.cmd_set_performance_marker_intel_raw(command_buffer, marker_info as *const _)
            .into_result()
    }
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL<'_>,
    ) -> crate::vk::Result<()> {
        self.cmd_set_performance_override_intel_raw(command_buffer, override_info as *const _)
            .into_result()
    }
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL<'_>,
    ) -> crate::vk::Result<()> {
        self.cmd_set_performance_stream_marker_intel_raw(command_buffer, marker_info as *const _)
            .into_result()
    }
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        polygon_mode: PolygonMode,
    ) {
        self.cmd_set_polygon_mode_ext_raw(command_buffer, polygon_mode)
    }
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        self.cmd_set_primitive_restart_enable_raw(command_buffer, primitive_restart_enable as u32)
    }
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        self.cmd_set_primitive_restart_enable_ext_raw(
            command_buffer,
            primitive_restart_enable as u32,
        )
    }
    pub unsafe fn cmd_set_primitive_restart_index_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_index: u32,
    ) {
        self.cmd_set_primitive_restart_index_ext_raw(command_buffer, primitive_restart_index)
    }
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        self.cmd_set_primitive_topology_raw(command_buffer, primitive_topology)
    }
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        self.cmd_set_primitive_topology_ext_raw(command_buffer, primitive_topology)
    }
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        self.cmd_set_provoking_vertex_mode_ext_raw(command_buffer, provoking_vertex_mode)
    }
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    ) {
        self.cmd_set_rasterization_samples_ext_raw(command_buffer, rasterization_samples)
    }
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_stream: u32,
    ) {
        self.cmd_set_rasterization_stream_ext_raw(command_buffer, rasterization_stream)
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        self.cmd_set_rasterizer_discard_enable_raw(command_buffer, rasterizer_discard_enable as u32)
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        self.cmd_set_rasterizer_discard_enable_ext_raw(
            command_buffer,
            rasterizer_discard_enable as u32,
        )
    }
    pub unsafe fn cmd_set_ray_tracing_pipeline_stack_size_khr(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stack_size: u32,
    ) {
        self.cmd_set_ray_tracing_pipeline_stack_size_khr_raw(command_buffer, pipeline_stack_size)
    }
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        self.cmd_set_rendering_attachment_locations_raw(command_buffer, location_info as *const _)
    }
    pub unsafe fn cmd_set_rendering_attachment_locations_khr(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo<'_>,
    ) {
        self.cmd_set_rendering_attachment_locations_khr_raw(
            command_buffer,
            location_info as *const _,
        )
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        self.cmd_set_rendering_input_attachment_indices_raw(
            command_buffer,
            input_attachment_index_info as *const _,
        )
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices_khr(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo<'_>,
    ) {
        self.cmd_set_rendering_input_attachment_indices_khr_raw(
            command_buffer,
            input_attachment_index_info as *const _,
        )
    }
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: bool,
    ) {
        self.cmd_set_representative_fragment_test_enable_nv_raw(
            command_buffer,
            representative_fragment_test_enable as u32,
        )
    }
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_info: &SampleLocationsInfoEXT<'_>,
    ) {
        self.cmd_set_sample_locations_ext_raw(command_buffer, sample_locations_info as *const _)
    }
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: bool,
    ) {
        self.cmd_set_sample_locations_enable_ext_raw(command_buffer, sample_locations_enable as u32)
    }
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        sample_mask: Option<&u32>,
    ) {
        self.cmd_set_sample_mask_ext_raw(
            command_buffer,
            samples,
            sample_mask.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissors: &[Rect2D<'_>],
    ) {
        self.cmd_set_scissor_raw(
            command_buffer,
            first_scissor,
            scissors.len() as _,
            scissors.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D<'_>],
    ) {
        self.cmd_set_scissor_with_count_raw(command_buffer, scissors.len() as _, scissors.as_ptr())
    }
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D<'_>],
    ) {
        self.cmd_set_scissor_with_count_ext_raw(
            command_buffer,
            scissors.len() as _,
            scissors.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate_image_enable: bool,
    ) {
        self.cmd_set_shading_rate_image_enable_nv_raw(
            command_buffer,
            shading_rate_image_enable as u32,
        )
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        self.cmd_set_stencil_compare_mask_raw(command_buffer, face_mask, compare_mask)
    }
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        self.cmd_set_stencil_op_raw(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        self.cmd_set_stencil_op_ext_raw(
            command_buffer,
            face_mask,
            fail_op,
            pass_op,
            depth_fail_op,
            compare_op,
        )
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        self.cmd_set_stencil_reference_raw(command_buffer, face_mask, reference)
    }
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: bool,
    ) {
        self.cmd_set_stencil_test_enable_raw(command_buffer, stencil_test_enable as u32)
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: bool,
    ) {
        self.cmd_set_stencil_test_enable_ext_raw(command_buffer, stencil_test_enable as u32)
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        self.cmd_set_stencil_write_mask_raw(command_buffer, face_mask, write_mask)
    }
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) {
        self.cmd_set_tessellation_domain_origin_ext_raw(command_buffer, domain_origin)
    }
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT<'_>],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT<'_>],
    ) {
        self.cmd_set_vertex_input_ext_raw(
            command_buffer,
            vertex_binding_descriptions.len() as _,
            vertex_binding_descriptions.as_ptr(),
            vertex_attribute_descriptions.len() as _,
            vertex_attribute_descriptions.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewports: &[Viewport<'_>],
    ) {
        self.cmd_set_viewport_raw(
            command_buffer,
            first_viewport,
            viewports.len() as _,
            viewports.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport_shading_rate_palette_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV<'_>],
    ) {
        self.cmd_set_viewport_shading_rate_palette_nv_raw(
            command_buffer,
            first_viewport,
            shading_rate_palettes.len() as _,
            shading_rate_palettes.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_swizzles: &[ViewportSwizzleNV<'_>],
    ) {
        self.cmd_set_viewport_swizzle_nv_raw(
            command_buffer,
            first_viewport,
            viewport_swizzles.len() as _,
            viewport_swizzles.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: bool,
    ) {
        self.cmd_set_viewport_w_scaling_enable_nv_raw(
            command_buffer,
            viewport_w_scaling_enable as u32,
        )
    }
    pub unsafe fn cmd_set_viewport_w_scaling_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_w_scalings: &[ViewportWScalingNV<'_>],
    ) {
        self.cmd_set_viewport_w_scaling_nv_raw(
            command_buffer,
            first_viewport,
            viewport_w_scalings.len() as _,
            viewport_w_scalings.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport<'_>],
    ) {
        self.cmd_set_viewport_with_count_raw(
            command_buffer,
            viewports.len() as _,
            viewports.as_ptr(),
        )
    }
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport<'_>],
    ) {
        self.cmd_set_viewport_with_count_ext_raw(
            command_buffer,
            viewports.len() as _,
            viewports.as_ptr(),
        )
    }
    pub unsafe fn cmd_subpass_shading_huawei(&self, command_buffer: CommandBuffer) {
        self.cmd_subpass_shading_huawei_raw(command_buffer)
    }
    pub unsafe fn cmd_trace_rays_indirect2_khr(
        &self,
        command_buffer: CommandBuffer,
        indirect_device_address: u64,
    ) {
        self.cmd_trace_rays_indirect2_khr_raw(command_buffer, indirect_device_address)
    }
    pub unsafe fn cmd_trace_rays_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        indirect_device_address: u64,
    ) {
        self.cmd_trace_rays_indirect_khr_raw(
            command_buffer,
            raygen_shader_binding_table as *const _,
            miss_shader_binding_table as *const _,
            hit_shader_binding_table as *const _,
            callable_shader_binding_table as *const _,
            indirect_device_address,
        )
    }
    pub unsafe fn cmd_trace_rays_khr(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR<'_>,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        self.cmd_trace_rays_khr_raw(
            command_buffer,
            raygen_shader_binding_table as *const _,
            miss_shader_binding_table as *const _,
            hit_shader_binding_table as *const _,
            callable_shader_binding_table as *const _,
            width,
            height,
            depth,
        )
    }
    pub unsafe fn cmd_trace_rays_nv(
        &self,
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: u64,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: u64,
        miss_shader_binding_stride: u64,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: u64,
        hit_shader_binding_stride: u64,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: u64,
        callable_shader_binding_stride: u64,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        self.cmd_trace_rays_nv_raw(
            command_buffer,
            raygen_shader_binding_table_buffer,
            raygen_shader_binding_offset,
            miss_shader_binding_table_buffer,
            miss_shader_binding_offset,
            miss_shader_binding_stride,
            hit_shader_binding_table_buffer,
            hit_shader_binding_offset,
            hit_shader_binding_stride,
            callable_shader_binding_table_buffer,
            callable_shader_binding_offset,
            callable_shader_binding_stride,
            width,
            height,
            depth,
        )
    }
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: u64,
        data: &[u8],
    ) {
        self.cmd_update_buffer_raw(
            command_buffer,
            dst_buffer,
            dst_offset,
            data.len() as _,
            data.as_ptr().cast(),
        )
    }
    pub unsafe fn cmd_update_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        dst_range: &DeviceAddressRangeKHR<'_>,
        dst_flags: AddressCommandFlagsKHR,
        data: &[u8],
    ) {
        self.cmd_update_memory_khr_raw(
            command_buffer,
            dst_range as *const _,
            dst_flags,
            data.len() as _,
            data.as_ptr().cast(),
        )
    }
    pub unsafe fn cmd_update_pipeline_indirect_buffer_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        self.cmd_update_pipeline_indirect_buffer_nv_raw(
            command_buffer,
            pipeline_bind_point,
            pipeline,
        )
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier<'_>],
        buffer_memory_barriers: &[BufferMemoryBarrier<'_>],
        image_memory_barriers: &[ImageMemoryBarrier<'_>],
    ) {
        self.cmd_wait_events_raw(
            command_buffer,
            events.len() as _,
            events.as_ptr(),
            src_stage_mask,
            dst_stage_mask,
            memory_barriers.len() as _,
            memory_barriers.as_ptr(),
            buffer_memory_barriers.len() as _,
            buffer_memory_barriers.as_ptr(),
            image_memory_barriers.len() as _,
            image_memory_barriers.as_ptr(),
        )
    }
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        assert_eq!(
            events.len(),
            dependency_infos.len(),
            "events and dependency_infos must have matching event_count lengths"
        );
        self.cmd_wait_events2_raw(
            command_buffer,
            events.len() as _,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        )
    }
    pub unsafe fn cmd_wait_events2_khr(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo<'_>],
    ) {
        assert_eq!(
            events.len(),
            dependency_infos.len(),
            "events and dependency_infos must have matching event_count lengths"
        );
        self.cmd_wait_events2_khr_raw(
            command_buffer,
            events.len() as _,
            events.as_ptr(),
            dependency_infos.as_ptr(),
        )
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        self.cmd_write_acceleration_structures_properties_khr_raw(
            command_buffer,
            acceleration_structures.len() as _,
            acceleration_structures.as_ptr(),
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_nv(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        self.cmd_write_acceleration_structures_properties_nv_raw(
            command_buffer,
            acceleration_structures.len() as _,
            acceleration_structures.as_ptr(),
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_buffer_marker2_amd(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: u64,
        marker: u32,
    ) {
        self.cmd_write_buffer_marker2_amd_raw(command_buffer, stage, dst_buffer, dst_offset, marker)
    }
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: u64,
        marker: u32,
    ) {
        self.cmd_write_buffer_marker_amd_raw(
            command_buffer,
            pipeline_stage,
            dst_buffer,
            dst_offset,
            marker,
        )
    }
    pub unsafe fn cmd_write_marker_to_memory_amd(
        &self,
        command_buffer: CommandBuffer,
        info: &MemoryMarkerInfoAMD<'_>,
    ) {
        self.cmd_write_marker_to_memory_amd_raw(command_buffer, info as *const _)
    }
    pub unsafe fn cmd_write_micromaps_properties_ext(
        &self,
        command_buffer: CommandBuffer,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        self.cmd_write_micromaps_properties_ext_raw(
            command_buffer,
            micromaps.len() as _,
            micromaps.as_ptr(),
            query_type,
            query_pool,
            first_query,
        )
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    ) {
        self.cmd_write_timestamp_raw(command_buffer, pipeline_stage, query_pool, query)
    }
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        self.cmd_write_timestamp2_raw(command_buffer, stage, query_pool, query)
    }
    pub unsafe fn cmd_write_timestamp2_khr(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        self.cmd_write_timestamp2_khr_raw(command_buffer, stage, query_pool, query)
    }
    pub unsafe fn compile_deferred_nv(
        &self,
        pipeline: Pipeline,
        shader: u32,
    ) -> crate::vk::Result<()> {
        self.compile_deferred_nv_raw(pipeline, shader).into_result()
    }
    pub unsafe fn convert_cooperative_vector_matrix_nv(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.convert_cooperative_vector_matrix_nv_raw(info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.copy_acceleration_structure_khr_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self
            .copy_acceleration_structure_to_memory_khr_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_image_to_image_raw(copy_image_to_image_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_image_to_image_ext(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_image_to_image_ext_raw(copy_image_to_image_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_image_to_memory_raw(copy_image_to_memory_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_image_to_memory_ext(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_image_to_memory_ext_raw(copy_image_to_memory_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self
            .copy_memory_to_acceleration_structure_khr_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_memory_to_image_raw(copy_memory_to_image_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_memory_to_image_ext(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.copy_memory_to_image_ext_raw(copy_memory_to_image_info as *const _)
            .into_result()
    }
    pub unsafe fn copy_memory_to_micromap_ext(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.copy_memory_to_micromap_ext_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_micromap_ext(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapInfoEXT<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.copy_micromap_ext_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn copy_micromap_to_memory_ext(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.copy_micromap_to_memory_ext_raw(deferred_operation, info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn create_acceleration_structure2_khr(
        &self,
        create_info: &AccelerationStructureCreateInfo2KHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<AccelerationStructureKHR> {
        let mut value = core::mem::MaybeUninit::<AccelerationStructureKHR>::zeroed();
        let result = self.create_acceleration_structure2_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<AccelerationStructureKHR> {
        let mut value = core::mem::MaybeUninit::<AccelerationStructureKHR>::zeroed();
        let result = self.create_acceleration_structure_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_acceleration_structure_nv(
        &self,
        create_info: &AccelerationStructureCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<AccelerationStructureNV> {
        let mut value = core::mem::MaybeUninit::<AccelerationStructureNV>::zeroed();
        let result = self.create_acceleration_structure_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_buffer(
        &self,
        create_info: &BufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Buffer> {
        let mut value = core::mem::MaybeUninit::<Buffer>::zeroed();
        let result = self.create_buffer_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn create_buffer_collection_fuchsia(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<BufferCollectionFUCHSIA> {
        let mut value = core::mem::MaybeUninit::<BufferCollectionFUCHSIA>::zeroed();
        let result = self.create_buffer_collection_fuchsia_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_buffer_view(
        &self,
        create_info: &BufferViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<BufferView> {
        let mut value = core::mem::MaybeUninit::<BufferView>::zeroed();
        let result = self.create_buffer_view_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<CommandPool> {
        let mut value = core::mem::MaybeUninit::<CommandPool>::zeroed();
        let result = self.create_command_pool_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_compute_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_compute_pipelines_raw(
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    pub unsafe fn create_cu_function_nvx(
        &self,
        create_info: &CuFunctionCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<CuFunctionNVX> {
        let mut value = core::mem::MaybeUninit::<CuFunctionNVX>::zeroed();
        let result = self.create_cu_function_nvx_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_cu_module_nvx(
        &self,
        create_info: &CuModuleCreateInfoNVX<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<CuModuleNVX> {
        let mut value = core::mem::MaybeUninit::<CuModuleNVX>::zeroed();
        let result = self.create_cu_module_nvx_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "beta")]
    pub unsafe fn create_cuda_function_nv(
        &self,
        create_info: &CudaFunctionCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<CudaFunctionNV> {
        let mut value = core::mem::MaybeUninit::<CudaFunctionNV>::zeroed();
        let result = self.create_cuda_function_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "beta")]
    pub unsafe fn create_cuda_module_nv(
        &self,
        create_info: &CudaModuleCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<CudaModuleNV> {
        let mut value = core::mem::MaybeUninit::<CudaModuleNV>::zeroed();
        let result = self.create_cuda_module_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_data_graph_pipeline_session_arm(
        &self,
        create_info: &DataGraphPipelineSessionCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DataGraphPipelineSessionARM> {
        let mut value = core::mem::MaybeUninit::<DataGraphPipelineSessionARM>::zeroed();
        let result = self.create_data_graph_pipeline_session_arm_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_data_graph_pipelines_arm(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_data_graph_pipelines_arm_raw(
            deferred_operation,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    pub unsafe fn create_deferred_operation_khr(
        &self,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DeferredOperationKHR> {
        let mut value = core::mem::MaybeUninit::<DeferredOperationKHR>::zeroed();
        let result = self.create_deferred_operation_khr_raw(
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DescriptorPool> {
        let mut value = core::mem::MaybeUninit::<DescriptorPool>::zeroed();
        let result = self.create_descriptor_pool_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DescriptorSetLayout> {
        let mut value = core::mem::MaybeUninit::<DescriptorSetLayout>::zeroed();
        let result = self.create_descriptor_set_layout_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DescriptorUpdateTemplate> {
        let mut value = core::mem::MaybeUninit::<DescriptorUpdateTemplate>::zeroed();
        let result = self.create_descriptor_update_template_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_descriptor_update_template_khr(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<DescriptorUpdateTemplate> {
        let mut value = core::mem::MaybeUninit::<DescriptorUpdateTemplate>::zeroed();
        let result = self.create_descriptor_update_template_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_event(
        &self,
        create_info: &EventCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Event> {
        let mut value = core::mem::MaybeUninit::<Event>::zeroed();
        let result = self.create_event_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "beta")]
    #[cfg(feature = "alloc")]
    pub unsafe fn create_execution_graph_pipelines_amdx(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_execution_graph_pipelines_amdx_raw(
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    pub unsafe fn create_external_compute_queue_nv(
        &self,
        create_info: &ExternalComputeQueueCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<ExternalComputeQueueNV> {
        let mut value = core::mem::MaybeUninit::<ExternalComputeQueueNV>::zeroed();
        let result = self.create_external_compute_queue_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_fence(
        &self,
        create_info: &FenceCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Fence> {
        let mut value = core::mem::MaybeUninit::<Fence>::zeroed();
        let result = self.create_fence_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Framebuffer> {
        let mut value = core::mem::MaybeUninit::<Framebuffer>::zeroed();
        let result = self.create_framebuffer_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_graphics_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_graphics_pipelines_raw(
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    pub unsafe fn create_image(
        &self,
        create_info: &ImageCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Image> {
        let mut value = core::mem::MaybeUninit::<Image>::zeroed();
        let result = self.create_image_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<ImageView> {
        let mut value = core::mem::MaybeUninit::<ImageView>::zeroed();
        let result = self.create_image_view_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_indirect_commands_layout_ext(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<IndirectCommandsLayoutEXT> {
        let mut value = core::mem::MaybeUninit::<IndirectCommandsLayoutEXT>::zeroed();
        let result = self.create_indirect_commands_layout_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<IndirectCommandsLayoutNV> {
        let mut value = core::mem::MaybeUninit::<IndirectCommandsLayoutNV>::zeroed();
        let result = self.create_indirect_commands_layout_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_indirect_execution_set_ext(
        &self,
        create_info: &IndirectExecutionSetCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<IndirectExecutionSetEXT> {
        let mut value = core::mem::MaybeUninit::<IndirectExecutionSetEXT>::zeroed();
        let result = self.create_indirect_execution_set_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_micromap_ext(
        &self,
        create_info: &MicromapCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<MicromapEXT> {
        let mut value = core::mem::MaybeUninit::<MicromapEXT>::zeroed();
        let result = self.create_micromap_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        create_info: &OpticalFlowSessionCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<OpticalFlowSessionNV> {
        let mut value = core::mem::MaybeUninit::<OpticalFlowSessionNV>::zeroed();
        let result = self.create_optical_flow_session_nv_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_pipeline_binaries_khr(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
        binaries: &mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.create_pipeline_binaries_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            binaries as *mut _,
        )
        .into_result()
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        create_info: &PipelineCacheCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<PipelineCache> {
        let mut value = core::mem::MaybeUninit::<PipelineCache>::zeroed();
        let result = self.create_pipeline_cache_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<PipelineLayout> {
        let mut value = core::mem::MaybeUninit::<PipelineLayout>::zeroed();
        let result = self.create_pipeline_layout_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_private_data_slot(
        &self,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<PrivateDataSlot> {
        let mut value = core::mem::MaybeUninit::<PrivateDataSlot>::zeroed();
        let result = self.create_private_data_slot_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_private_data_slot_ext(
        &self,
        create_info: &PrivateDataSlotCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<PrivateDataSlot> {
        let mut value = core::mem::MaybeUninit::<PrivateDataSlot>::zeroed();
        let result = self.create_private_data_slot_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_query_pool(
        &self,
        create_info: &QueryPoolCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<QueryPool> {
        let mut value = core::mem::MaybeUninit::<QueryPool>::zeroed();
        let result = self.create_query_pool_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_ray_tracing_pipelines_khr(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_ray_tracing_pipelines_khr_raw(
            deferred_operation,
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_ray_tracing_pipelines_nv(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> core::result::Result<alloc::vec::Vec<Pipeline>, (alloc::vec::Vec<Pipeline>, VkResult)>
    {
        let mut values = alloc::vec![Pipeline::null(); create_infos.len()];
        let result = self.create_ray_tracing_pipelines_nv_raw(
            pipeline_cache,
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        );
        if result == VkResult::SUCCESS {
            Ok(values)
        } else {
            Err((values, result))
        }
    }
    pub unsafe fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<RenderPass> {
        let mut value = core::mem::MaybeUninit::<RenderPass>::zeroed();
        let result = self.create_render_pass_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_render_pass2(
        &self,
        create_info: &RenderPassCreateInfo2<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<RenderPass> {
        let mut value = core::mem::MaybeUninit::<RenderPass>::zeroed();
        let result = self.create_render_pass2_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_render_pass2_khr(
        &self,
        create_info: &RenderPassCreateInfo2<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<RenderPass> {
        let mut value = core::mem::MaybeUninit::<RenderPass>::zeroed();
        let result = self.create_render_pass2_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_sampler(
        &self,
        create_info: &SamplerCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Sampler> {
        let mut value = core::mem::MaybeUninit::<Sampler>::zeroed();
        let result = self.create_sampler_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SamplerYcbcrConversion> {
        let mut value = core::mem::MaybeUninit::<SamplerYcbcrConversion>::zeroed();
        let result = self.create_sampler_ycbcr_conversion_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_sampler_ycbcr_conversion_khr(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SamplerYcbcrConversion> {
        let mut value = core::mem::MaybeUninit::<SamplerYcbcrConversion>::zeroed();
        let result = self.create_sampler_ycbcr_conversion_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Semaphore> {
        let mut value = core::mem::MaybeUninit::<Semaphore>::zeroed();
        let result = self.create_semaphore_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_shader_instrumentation_arm(
        &self,
        create_info: &ShaderInstrumentationCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<ShaderInstrumentationARM> {
        let mut value = core::mem::MaybeUninit::<ShaderInstrumentationARM>::zeroed();
        let result = self.create_shader_instrumentation_arm_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<ShaderModule> {
        let mut value = core::mem::MaybeUninit::<ShaderModule>::zeroed();
        let result = self.create_shader_module_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_shaders_ext(
        &self,
        create_infos: &[ShaderCreateInfoEXT<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<alloc::vec::Vec<ShaderEXT>> {
        let mut values = alloc::vec![ShaderEXT::null(); create_infos.len()];
        self.create_shaders_ext_raw(
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        )
        .into_result()?;
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn create_shared_swapchains_khr(
        &self,
        create_infos: &[SwapchainCreateInfoKHR<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<alloc::vec::Vec<SwapchainKHR>> {
        let mut values = alloc::vec![SwapchainKHR::null(); create_infos.len()];
        self.create_shared_swapchains_khr_raw(
            create_infos.len() as u32,
            create_infos.as_ptr(),
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            values.as_mut_ptr(),
        )
        .into_result()?;
        Ok(values)
    }
    pub unsafe fn create_swapchain_khr(
        &self,
        create_info: &SwapchainCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<SwapchainKHR> {
        let mut value = core::mem::MaybeUninit::<SwapchainKHR>::zeroed();
        let result = self.create_swapchain_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_tensor_arm(
        &self,
        create_info: &TensorCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<TensorARM> {
        let mut value = core::mem::MaybeUninit::<TensorARM>::zeroed();
        let result = self.create_tensor_arm_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_tensor_view_arm(
        &self,
        create_info: &TensorViewCreateInfoARM<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<TensorViewARM> {
        let mut value = core::mem::MaybeUninit::<TensorViewARM>::zeroed();
        let result = self.create_tensor_view_arm_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_validation_cache_ext(
        &self,
        create_info: &ValidationCacheCreateInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<ValidationCacheEXT> {
        let mut value = core::mem::MaybeUninit::<ValidationCacheEXT>::zeroed();
        let result = self.create_validation_cache_ext_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_video_session_khr(
        &self,
        create_info: &VideoSessionCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<VideoSessionKHR> {
        let mut value = core::mem::MaybeUninit::<VideoSessionKHR>::zeroed();
        let result = self.create_video_session_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<VideoSessionParametersKHR> {
        let mut value = core::mem::MaybeUninit::<VideoSessionParametersKHR>::zeroed();
        let result = self.create_video_session_parameters_khr_raw(
            create_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn debug_marker_set_object_name_ext(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.debug_marker_set_object_name_ext_raw(name_info as *const _)
            .into_result()
    }
    pub unsafe fn debug_marker_set_object_tag_ext(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.debug_marker_set_object_tag_ext_raw(tag_info as *const _)
            .into_result()
    }
    pub unsafe fn deferred_operation_join_khr(
        &self,
        operation: DeferredOperationKHR,
    ) -> crate::vk::Result<VkResult> {
        let result = self.deferred_operation_join_khr_raw(operation);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_acceleration_structure_khr_raw(
            acceleration_structure,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_acceleration_structure_nv(
        &self,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_acceleration_structure_nv_raw(
            acceleration_structure,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_buffer(
        &self,
        buffer: Buffer,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_buffer_raw(
            buffer,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn destroy_buffer_collection_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_buffer_collection_fuchsia_raw(
            collection,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_buffer_view_raw(
            buffer_view,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_command_pool(
        &self,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_command_pool_raw(
            command_pool,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_cu_function_nvx(
        &self,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_cu_function_nvx_raw(
            function,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_cu_module_nvx(
        &self,
        module: CuModuleNVX,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_cu_module_nvx_raw(
            module,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn destroy_cuda_function_nv(
        &self,
        function: CudaFunctionNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_cuda_function_nv_raw(
            function,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    #[cfg(feature = "beta")]
    pub unsafe fn destroy_cuda_module_nv(
        &self,
        module: CudaModuleNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_cuda_module_nv_raw(
            module,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        &self,
        session: DataGraphPipelineSessionARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_data_graph_pipeline_session_arm_raw(
            session,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_deferred_operation_khr(
        &self,
        operation: DeferredOperationKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_deferred_operation_khr_raw(
            operation,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_descriptor_pool_raw(
            descriptor_pool,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_descriptor_set_layout_raw(
            descriptor_set_layout,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_descriptor_update_template_raw(
            descriptor_update_template,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_descriptor_update_template_khr(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_descriptor_update_template_khr_raw(
            descriptor_update_template,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_event(&self, event: Event, allocator: Option<&AllocationCallbacks<'_>>) {
        self.destroy_event_raw(
            event,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_external_compute_queue_nv(
        &self,
        external_queue: ExternalComputeQueueNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_external_compute_queue_nv_raw(
            external_queue,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_fence(&self, fence: Fence, allocator: Option<&AllocationCallbacks<'_>>) {
        self.destroy_fence_raw(
            fence,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_framebuffer_raw(
            framebuffer,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_image(&self, image: Image, allocator: Option<&AllocationCallbacks<'_>>) {
        self.destroy_image_raw(
            image,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_image_view(
        &self,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_image_view_raw(
            image_view,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_indirect_commands_layout_ext(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_indirect_commands_layout_ext_raw(
            indirect_commands_layout,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_indirect_commands_layout_nv_raw(
            indirect_commands_layout,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_indirect_execution_set_ext(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_indirect_execution_set_ext_raw(
            indirect_execution_set,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_micromap_ext(
        &self,
        micromap: MicromapEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_micromap_ext_raw(
            micromap,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        session: OpticalFlowSessionNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_optical_flow_session_nv_raw(
            session,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_pipeline(
        &self,
        pipeline: Pipeline,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_pipeline_raw(
            pipeline,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_pipeline_binary_khr(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_pipeline_binary_khr_raw(
            pipeline_binary,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_pipeline_cache_raw(
            pipeline_cache,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_pipeline_layout_raw(
            pipeline_layout,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_private_data_slot(
        &self,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_private_data_slot_raw(
            private_data_slot,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_private_data_slot_ext(
        &self,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_private_data_slot_ext_raw(
            private_data_slot,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_query_pool(
        &self,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_query_pool_raw(
            query_pool,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_render_pass(
        &self,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_render_pass_raw(
            render_pass,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_sampler(
        &self,
        sampler: Sampler,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_sampler_raw(
            sampler,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_sampler_ycbcr_conversion_raw(
            ycbcr_conversion,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_sampler_ycbcr_conversion_khr(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_sampler_ycbcr_conversion_khr_raw(
            ycbcr_conversion,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_semaphore(
        &self,
        semaphore: Semaphore,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_semaphore_raw(
            semaphore,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_shader_ext(
        &self,
        shader: ShaderEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_shader_ext_raw(
            shader,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_shader_instrumentation_arm(
        &self,
        instrumentation: ShaderInstrumentationARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_shader_instrumentation_arm_raw(
            instrumentation,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_shader_module(
        &self,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_shader_module_raw(
            shader_module,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_swapchain_khr(
        &self,
        swapchain: SwapchainKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_swapchain_khr_raw(
            swapchain,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_tensor_arm(
        &self,
        tensor: TensorARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_tensor_arm_raw(
            tensor,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_tensor_view_arm(
        &self,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_tensor_view_arm_raw(
            tensor_view,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_validation_cache_ext(
        &self,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_validation_cache_ext_raw(
            validation_cache,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_video_session_khr(
        &self,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_video_session_khr_raw(
            video_session,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.destroy_video_session_parameters_khr_raw(
            video_session_parameters,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn device_wait_idle(&self) -> crate::vk::Result<()> {
        self.device_wait_idle_raw().into_result()
    }
    pub unsafe fn display_power_control_ext(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.display_power_control_ext_raw(display, display_power_info as *const _)
            .into_result()
    }
    pub unsafe fn end_command_buffer(
        &self,
        command_buffer: CommandBuffer,
    ) -> crate::vk::Result<()> {
        self.end_command_buffer_raw(command_buffer).into_result()
    }
    #[cfg(feature = "metal")]
    pub unsafe fn export_metal_objects_ext(
        &self,
        metal_objects_info: &mut ExportMetalObjectsInfoEXT<'_>,
    ) {
        self.export_metal_objects_ext_raw(metal_objects_info as *mut _)
    }
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::vk::Result<()> {
        self.flush_mapped_memory_ranges_raw(memory_ranges.len() as _, memory_ranges.as_ptr())
            .into_result()
    }
    pub unsafe fn free_command_buffers(
        &self,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        self.free_command_buffers_raw(
            command_pool,
            command_buffers.len() as _,
            command_buffers.as_ptr(),
        )
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> crate::vk::Result<()> {
        self.free_descriptor_sets_raw(
            descriptor_pool,
            descriptor_sets.len() as _,
            descriptor_sets.as_ptr(),
        )
        .into_result()
    }
    pub unsafe fn free_memory(
        &self,
        memory: DeviceMemory,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        self.free_memory_raw(
            memory,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
    }
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR<'_>,
        max_primitive_counts: Option<&[u32]>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        if let Some(value) = max_primitive_counts {
            assert_eq!(
                value.len(),
                build_info.geometry_count as usize,
                "max_primitive_counts length must match build_info.geometry_count"
            );
        }
        self.get_acceleration_structure_build_sizes_khr_raw(
            build_type,
            build_info as *const _,
            max_primitive_counts.map_or(core::ptr::null(), |x| x.as_ptr()),
            size_info as *mut _,
        )
    }
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> u64 {
        self.get_acceleration_structure_device_address_khr_raw(info as *const _)
    }
    pub unsafe fn get_acceleration_structure_handle_nv(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [u8],
    ) -> crate::vk::Result<()> {
        self.get_acceleration_structure_handle_nv_raw(
            acceleration_structure,
            data.len() as _,
            data.as_mut_ptr().cast(),
        )
        .into_result()
    }
    pub unsafe fn get_acceleration_structure_memory_requirements_nv(
        &self,
        info: &AccelerationStructureMemoryRequirementsInfoNV<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_acceleration_structure_memory_requirements_nv_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    #[cfg(feature = "android")]
    pub unsafe fn get_android_hardware_buffer_properties_android(
        &self,
        buffer: &AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> crate::vk::Result<()> {
        self.get_android_hardware_buffer_properties_android_raw(
            buffer as *const _,
            properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_buffer_collection_properties_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> crate::vk::Result<()> {
        self.get_buffer_collection_properties_fuchsia_raw(collection, properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_buffer_device_address(&self, info: &BufferDeviceAddressInfo<'_>) -> u64 {
        self.get_buffer_device_address_raw(info as *const _)
    }
    pub unsafe fn get_buffer_device_address_ext(&self, info: &BufferDeviceAddressInfo<'_>) -> u64 {
        self.get_buffer_device_address_ext_raw(info as *const _)
    }
    pub unsafe fn get_buffer_device_address_khr(&self, info: &BufferDeviceAddressInfo<'_>) -> u64 {
        self.get_buffer_device_address_khr_raw(info as *const _)
    }
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        buffer: Buffer,
    ) -> MemoryRequirements<'static> {
        let mut value = core::mem::MaybeUninit::<MemoryRequirements<'static>>::zeroed();
        self.get_buffer_memory_requirements_raw(buffer, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_buffer_memory_requirements2(
        &self,
        info: &BufferMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_buffer_memory_requirements2_raw(info as *const _, memory_requirements as *mut _)
    }
    pub unsafe fn get_buffer_memory_requirements2_khr(
        &self,
        info: &BufferMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_buffer_memory_requirements2_khr_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_buffer_opaque_capture_address(
        &self,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        self.get_buffer_opaque_capture_address_raw(info as *const _)
    }
    pub unsafe fn get_buffer_opaque_capture_address_khr(
        &self,
        info: &BufferDeviceAddressInfo<'_>,
    ) -> u64 {
        self.get_buffer_opaque_capture_address_khr_raw(info as *const _)
    }
    pub unsafe fn get_calibrated_timestamps_ext(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR<'_>],
        timestamps: &mut [u64],
    ) -> crate::vk::Result<u64> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_calibrated_timestamps_ext_raw(
            timestamp_infos.len() as _,
            timestamp_infos.as_ptr(),
            timestamps.as_mut_ptr(),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_calibrated_timestamps_khr(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR<'_>],
        timestamps: &mut [u64],
    ) -> crate::vk::Result<u64> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_calibrated_timestamps_khr_raw(
            timestamp_infos.len() as _,
            timestamp_infos.as_ptr(),
            timestamps.as_mut_ptr(),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV<'_>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        self.get_cluster_acceleration_structure_build_sizes_nv_raw(
            info as *const _,
            size_info as *mut _,
        )
    }
    #[cfg(feature = "beta")]
    #[cfg(feature = "alloc")]
    pub unsafe fn get_cuda_module_cache_nv(
        &self,
        module: CudaModuleNV,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_cuda_module_cache_nv_raw(module, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result =
                self.get_cuda_module_cache_nv_raw(module, &mut count, values.as_mut_ptr().cast());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_data_graph_pipeline_available_properties_arm(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<DataGraphPipelinePropertyARM>> {
        let mut count = 0;
        self.get_data_graph_pipeline_available_properties_arm_raw(
            pipeline_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DataGraphPipelinePropertyARM> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_data_graph_pipeline_available_properties_arm_raw(
                pipeline_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_data_graph_pipeline_properties_arm(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM<'_>,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM<'_>],
    ) -> crate::vk::Result<VkResult> {
        let result = self.get_data_graph_pipeline_properties_arm_raw(
            pipeline_info as *const _,
            properties.len() as _,
            properties.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(result)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<DataGraphPipelineSessionBindPointRequirementARM<'static>>>
    {
        let mut count = 0;
        self.get_data_graph_pipeline_session_bind_point_requirements_arm_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<DataGraphPipelineSessionBindPointRequirementARM<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_data_graph_pipeline_session_bind_point_requirements_arm_raw(
                info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm_count(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_data_graph_pipeline_session_bind_point_requirements_arm_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm_into(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
        values: &mut [DataGraphPipelineSessionBindPointRequirementARM<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_data_graph_pipeline_session_bind_point_requirements_arm_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm(
        &self,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_data_graph_pipeline_session_memory_requirements_arm_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_deferred_operation_max_concurrency_khr(
        &self,
        operation: DeferredOperationKHR,
    ) -> u32 {
        self.get_deferred_operation_max_concurrency_khr_raw(operation)
    }
    pub unsafe fn get_deferred_operation_result_khr(
        &self,
        operation: DeferredOperationKHR,
    ) -> crate::vk::Result<VkResult> {
        let result = self.get_deferred_operation_result_khr_raw(operation);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn get_descriptor_ext(
        &self,
        descriptor_info: &DescriptorGetInfoEXT<'_>,
        descriptor: &mut [u8],
    ) {
        self.get_descriptor_ext_raw(
            descriptor_info as *const _,
            descriptor.len() as _,
            descriptor.as_mut_ptr().cast(),
        )
    }
    pub unsafe fn get_descriptor_set_host_mapping_valve(
        &self,
        descriptor_set: DescriptorSet,
    ) -> *mut c_void {
        let mut value = core::mem::MaybeUninit::<*mut c_void>::zeroed();
        self.get_descriptor_set_host_mapping_valve_raw(descriptor_set, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_descriptor_set_layout_binding_offset_ext(
        &self,
        layout: DescriptorSetLayout,
        binding: u32,
    ) -> u64 {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        self.get_descriptor_set_layout_binding_offset_ext_raw(layout, binding, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_descriptor_set_layout_host_mapping_info_valve(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE<'_>,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ) {
        self.get_descriptor_set_layout_host_mapping_info_valve_raw(
            binding_reference as *const _,
            host_mapping as *mut _,
        )
    }
    pub unsafe fn get_descriptor_set_layout_size_ext(&self, layout: DescriptorSetLayout) -> u64 {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        self.get_descriptor_set_layout_size_ext_raw(layout, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_descriptor_set_layout_support(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        support: &mut DescriptorSetLayoutSupport<'_>,
    ) {
        self.get_descriptor_set_layout_support_raw(create_info as *const _, support as *mut _)
    }
    pub unsafe fn get_descriptor_set_layout_support_khr(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo<'_>,
        support: &mut DescriptorSetLayoutSupport<'_>,
    ) {
        self.get_descriptor_set_layout_support_khr_raw(create_info as *const _, support as *mut _)
    }
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR<'_>,
    ) -> AccelerationStructureCompatibilityKHR {
        let mut value = core::mem::MaybeUninit::<AccelerationStructureCompatibilityKHR>::zeroed();
        self.get_device_acceleration_structure_compatibility_khr_raw(
            version_info as *const _,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_device_buffer_memory_requirements_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_device_buffer_memory_requirements_khr(
        &self,
        info: &DeviceBufferMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_device_buffer_memory_requirements_khr_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_device_combined_image_sampler_index_nvx(
        &self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        self.get_device_combined_image_sampler_index_nvx_raw(image_view_index, sampler_index)
    }
    pub unsafe fn get_device_fault_debug_info_khr(
        &self,
        debug_info: &mut DeviceFaultDebugInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_device_fault_debug_info_khr_raw(debug_info as *mut _)
            .into_result()
    }
    pub unsafe fn get_device_fault_info_ext(
        &self,
        fault_counts: &mut DeviceFaultCountsEXT<'_>,
        fault_info: Option<&mut DeviceFaultInfoEXT<'_>>,
    ) -> crate::vk::Result<()> {
        self.get_device_fault_info_ext_raw(
            fault_counts as *mut _,
            fault_info.as_ref().map_or(core::ptr::null_mut(), |x| {
                core::ptr::from_ref(&**x).cast_mut()
            }),
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_device_fault_reports_khr(
        &self,
        timeout: u64,
    ) -> crate::vk::Result<alloc::vec::Vec<DeviceFaultInfoKHR<'static>>> {
        let mut count = 0;
        self.get_device_fault_reports_khr_raw(timeout, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<DeviceFaultInfoKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result =
                self.get_device_fault_reports_khr_raw(timeout, &mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_device_fault_reports_khr_count(
        &self,
        timeout: u64,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_device_fault_reports_khr_raw(timeout, &mut count, core::ptr::null_mut())
            .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_device_fault_reports_khr_into(
        &self,
        timeout: u64,
        values: &mut [DeviceFaultInfoKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result =
            self.get_device_fault_reports_khr_raw(timeout, &mut count, values.as_mut_ptr());
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        let mut value = core::mem::MaybeUninit::<PeerMemoryFeatureFlags>::zeroed();
        self.get_device_group_peer_memory_features_raw(
            heap_index,
            local_device_index,
            remote_device_index,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_device_group_peer_memory_features_khr(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        let mut value = core::mem::MaybeUninit::<PeerMemoryFeatureFlags>::zeroed();
        self.get_device_group_peer_memory_features_khr_raw(
            heap_index,
            local_device_index,
            remote_device_index,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_device_group_present_capabilities_khr(
        &self,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_device_group_present_capabilities_khr_raw(
            device_group_present_capabilities as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR<'_>,
    ) -> crate::vk::Result<DeviceGroupPresentModeFlagsKHR> {
        let mut value = core::mem::MaybeUninit::<DeviceGroupPresentModeFlagsKHR>::zeroed();
        let result = self.get_device_group_surface_present_modes2_ext_raw(
            surface_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_device_group_surface_present_modes_khr(
        &self,
        surface: SurfaceKHR,
    ) -> crate::vk::Result<DeviceGroupPresentModeFlagsKHR> {
        let mut value = core::mem::MaybeUninit::<DeviceGroupPresentModeFlagsKHR>::zeroed();
        let result =
            self.get_device_group_surface_present_modes_khr_raw(surface, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_device_image_memory_requirements_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_device_image_memory_requirements_khr(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_device_image_memory_requirements_khr_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageMemoryRequirements2<'static>>> {
        let mut count = 0;
        self.get_device_image_sparse_memory_requirements_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageMemoryRequirements2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_device_image_sparse_memory_requirements_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_count(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_device_image_sparse_memory_requirements_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_into(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        values: &mut [SparseImageMemoryRequirements2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_device_image_sparse_memory_requirements_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_device_image_sparse_memory_requirements_khr(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageMemoryRequirements2<'static>>> {
        let mut count = 0;
        self.get_device_image_sparse_memory_requirements_khr_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageMemoryRequirements2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_device_image_sparse_memory_requirements_khr_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_khr_count(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_device_image_sparse_memory_requirements_khr_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_device_image_sparse_memory_requirements_khr_into(
        &self,
        info: &DeviceImageMemoryRequirements<'_>,
        values: &mut [SparseImageMemoryRequirements2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_device_image_sparse_memory_requirements_khr_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        info: &DeviceImageSubresourceInfo<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        self.get_device_image_subresource_layout_raw(info as *const _, layout as *mut _)
    }
    pub unsafe fn get_device_image_subresource_layout_khr(
        &self,
        info: &DeviceImageSubresourceInfo<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        self.get_device_image_subresource_layout_khr_raw(info as *const _, layout as *mut _)
    }
    pub unsafe fn get_device_memory_commitment(&self, memory: DeviceMemory) -> u64 {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        self.get_device_memory_commitment_raw(memory, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        self.get_device_memory_opaque_capture_address_raw(info as *const _)
    }
    pub unsafe fn get_device_memory_opaque_capture_address_khr(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64 {
        self.get_device_memory_opaque_capture_address_khr_raw(info as *const _)
    }
    pub unsafe fn get_device_micromap_compatibility_ext(
        &self,
        version_info: &MicromapVersionInfoEXT<'_>,
    ) -> AccelerationStructureCompatibilityKHR {
        let mut value = core::mem::MaybeUninit::<AccelerationStructureCompatibilityKHR>::zeroed();
        self.get_device_micromap_compatibility_ext_raw(
            version_info as *const _,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let mut value = core::mem::MaybeUninit::<Queue>::zeroed();
        self.get_device_queue_raw(queue_family_index, queue_index, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_device_queue2(&self, queue_info: &DeviceQueueInfo2<'_>) -> Queue {
        let mut value = core::mem::MaybeUninit::<Queue>::zeroed();
        self.get_device_queue2_raw(queue_info as *const _, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_device_subpass_shading_max_workgroup_size_huawei(
        &self,
        renderpass: RenderPass,
        max_workgroup_size: &mut [Extent2D<'_>],
    ) -> crate::vk::Result<()> {
        self.get_device_subpass_shading_max_workgroup_size_huawei_raw(
            renderpass,
            max_workgroup_size.as_mut_ptr(),
        )
        .into_result()
    }
    pub unsafe fn get_device_tensor_memory_requirements_arm(
        &self,
        info: &DeviceTensorMemoryRequirementsARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_device_tensor_memory_requirements_arm_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_dynamic_rendering_tile_properties_qcom(
        &self,
        rendering_info: &RenderingInfo<'_>,
        properties: &mut TilePropertiesQCOM<'_>,
    ) -> crate::vk::Result<()> {
        self.get_dynamic_rendering_tile_properties_qcom_raw(
            rendering_info as *const _,
            properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_encoded_video_session_parameters_khr(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR<'_>,
        feedback_info: Option<&mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>>,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_encoded_video_session_parameters_khr_raw(
            video_session_parameters_info as *const _,
            feedback_info.as_ref().map_or(core::ptr::null_mut(), |x| {
                core::ptr::from_ref(&**x).cast_mut()
            }),
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result = self.get_encoded_video_session_parameters_khr_raw(
                video_session_parameters_info as *const _,
                feedback_info.as_ref().map_or(core::ptr::null_mut(), |x| {
                    core::ptr::from_ref(&**x).cast_mut()
                }),
                &mut count,
                values.as_mut_ptr().cast(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    pub unsafe fn get_event_status(&self, event: Event) -> crate::vk::Result<VkResult> {
        let result = self.get_event_status_raw(event);
        result.into_result()?;
        Ok(result)
    }
    #[cfg(feature = "beta")]
    pub unsafe fn get_execution_graph_pipeline_node_index_amdx(
        &self,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX<'_>,
    ) -> crate::vk::Result<u32> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.get_execution_graph_pipeline_node_index_amdx_raw(
            execution_graph,
            node_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "beta")]
    pub unsafe fn get_execution_graph_pipeline_scratch_size_amdx(
        &self,
        execution_graph: Pipeline,
        size_info: &mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> crate::vk::Result<()> {
        self.get_execution_graph_pipeline_scratch_size_amdx_raw(
            execution_graph,
            size_info as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_fence_fd_khr(
        &self,
        get_fd_info: &FenceGetFdInfoKHR<'_>,
    ) -> crate::vk::Result<core::ffi::c_int> {
        let mut value = core::mem::MaybeUninit::<core::ffi::c_int>::zeroed();
        let result = self.get_fence_fd_khr_raw(get_fd_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_fence_status(&self, fence: Fence) -> crate::vk::Result<VkResult> {
        let result = self.get_fence_status_raw(fence);
        result.into_result()?;
        Ok(result)
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR<'_>,
    ) -> crate::vk::Result<HANDLE> {
        let mut value = core::mem::MaybeUninit::<HANDLE>::zeroed();
        let result = self
            .get_fence_win32_handle_khr_raw(get_win32_handle_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_framebuffer_tile_properties_qcom(
        &self,
        framebuffer: Framebuffer,
    ) -> crate::vk::Result<alloc::vec::Vec<TilePropertiesQCOM<'static>>> {
        let mut count = 0;
        self.get_framebuffer_tile_properties_qcom_raw(
            framebuffer,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<TilePropertiesQCOM<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_framebuffer_tile_properties_qcom_raw(
                framebuffer,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_framebuffer_tile_properties_qcom_count(
        &self,
        framebuffer: Framebuffer,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_framebuffer_tile_properties_qcom_raw(
            framebuffer,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_framebuffer_tile_properties_qcom_into(
        &self,
        framebuffer: Framebuffer,
        values: &mut [TilePropertiesQCOM<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_framebuffer_tile_properties_qcom_raw(
            framebuffer,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_generated_commands_memory_requirements_ext(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_generated_commands_memory_requirements_ext_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_generated_commands_memory_requirements_nv_raw(
            info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_image_drm_format_modifier_properties_ext(
        &self,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.get_image_drm_format_modifier_properties_ext_raw(image, properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        image: Image,
    ) -> MemoryRequirements<'static> {
        let mut value = core::mem::MaybeUninit::<MemoryRequirements<'static>>::zeroed();
        self.get_image_memory_requirements_raw(image, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_image_memory_requirements2(
        &self,
        info: &ImageMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_image_memory_requirements2_raw(info as *const _, memory_requirements as *mut _)
    }
    pub unsafe fn get_image_memory_requirements2_khr(
        &self,
        info: &ImageMemoryRequirementsInfo2<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_image_memory_requirements2_khr_raw(info as *const _, memory_requirements as *mut _)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_image_opaque_capture_data_ext(
        &self,
        images: &[Image],
    ) -> crate::vk::Result<alloc::vec::Vec<HostAddressRangeEXT<'static>>> {
        let mut values = alloc::vec![Default::default(); images.len()];
        self.get_image_opaque_capture_data_ext_raw(
            images.len() as u32,
            images.as_ptr(),
            values.as_mut_ptr(),
        )
        .into_result()?;
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        image: Image,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageMemoryRequirements<'static>>> {
        let mut count = 0;
        self.get_image_sparse_memory_requirements_raw(image, &mut count, core::ptr::null_mut());
        let mut values: alloc::vec::Vec<SparseImageMemoryRequirements<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_image_sparse_memory_requirements_raw(image, &mut count, values.as_mut_ptr());
        values.truncate(count as usize);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_image_sparse_memory_requirements2(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageMemoryRequirements2<'static>>> {
        let mut count = 0;
        self.get_image_sparse_memory_requirements2_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageMemoryRequirements2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_image_sparse_memory_requirements2_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_image_sparse_memory_requirements2_count(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_image_sparse_memory_requirements2_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_image_sparse_memory_requirements2_into(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
        values: &mut [SparseImageMemoryRequirements2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_image_sparse_memory_requirements2_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_image_sparse_memory_requirements2_khr(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<SparseImageMemoryRequirements2<'static>>> {
        let mut count = 0;
        self.get_image_sparse_memory_requirements2_khr_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        let mut values: alloc::vec::Vec<SparseImageMemoryRequirements2<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_image_sparse_memory_requirements2_khr_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr_count(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
    ) -> usize {
        let mut count = 0;
        self.get_image_sparse_memory_requirements2_khr_raw(
            info as *const _,
            &mut count,
            core::ptr::null_mut(),
        );
        count as usize
    }
    pub unsafe fn get_image_sparse_memory_requirements2_khr_into(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2<'_>,
        values: &mut [SparseImageMemoryRequirements2<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_image_sparse_memory_requirements2_khr_raw(
            info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        count as usize
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        image: Image,
        subresource: &ImageSubresource<'_>,
    ) -> SubresourceLayout<'static> {
        let mut value = core::mem::MaybeUninit::<SubresourceLayout<'static>>::zeroed();
        self.get_image_subresource_layout_raw(image, subresource as *const _, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_image_subresource_layout2(
        &self,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        self.get_image_subresource_layout2_raw(image, subresource as *const _, layout as *mut _)
    }
    pub unsafe fn get_image_subresource_layout2_ext(
        &self,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        self.get_image_subresource_layout2_ext_raw(image, subresource as *const _, layout as *mut _)
    }
    pub unsafe fn get_image_subresource_layout2_khr(
        &self,
        image: Image,
        subresource: &ImageSubresource2<'_>,
        layout: &mut SubresourceLayout2<'_>,
    ) {
        self.get_image_subresource_layout2_khr_raw(image, subresource as *const _, layout as *mut _)
    }
    pub unsafe fn get_image_view_address_nvx(
        &self,
        image_view: ImageView,
        properties: &mut ImageViewAddressPropertiesNVX<'_>,
    ) -> crate::vk::Result<()> {
        self.get_image_view_address_nvx_raw(image_view, properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_image_view_handle64_nvx(&self, info: &ImageViewHandleInfoNVX<'_>) -> u64 {
        self.get_image_view_handle64_nvx_raw(info as *const _)
    }
    pub unsafe fn get_image_view_handle_nvx(&self, info: &ImageViewHandleInfoNVX<'_>) -> u32 {
        self.get_image_view_handle_nvx_raw(info as *const _)
    }
    pub unsafe fn get_latency_timings_nv(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &mut GetLatencyMarkerInfoNV<'_>,
    ) {
        self.get_latency_timings_nv_raw(swapchain, latency_marker_info as *mut _)
    }
    pub unsafe fn get_memory_fd_khr(
        &self,
        get_fd_info: &MemoryGetFdInfoKHR<'_>,
    ) -> crate::vk::Result<core::ffi::c_int> {
        let mut value = core::mem::MaybeUninit::<core::ffi::c_int>::zeroed();
        let result = self.get_memory_fd_khr_raw(get_fd_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: core::ffi::c_int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_memory_fd_properties_khr_raw(handle_type, fd, memory_fd_properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        host_pointer: &c_void,
        memory_host_pointer_properties: &mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.get_memory_host_pointer_properties_ext_raw(
            handle_type,
            host_pointer as *const _,
            memory_host_pointer_properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "metal")]
    pub unsafe fn get_memory_metal_handle_ext(
        &self,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT<'_>,
    ) -> crate::vk::Result<*mut c_void> {
        let mut value = core::mem::MaybeUninit::<*mut c_void>::zeroed();
        let result = self
            .get_memory_metal_handle_ext_raw(get_metal_handle_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "metal")]
    pub unsafe fn get_memory_metal_handle_properties_ext(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: &c_void,
        memory_metal_handle_properties: &mut MemoryMetalHandlePropertiesEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.get_memory_metal_handle_properties_ext_raw(
            handle_type,
            handle as *const _,
            memory_metal_handle_properties as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV<'_>,
    ) -> crate::vk::Result<RemoteAddressNV> {
        let mut value = core::mem::MaybeUninit::<RemoteAddressNV>::zeroed();
        let result = self.get_memory_remote_address_nv_raw(
            memory_get_remote_address_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR<'_>,
    ) -> crate::vk::Result<HANDLE> {
        let mut value = core::mem::MaybeUninit::<HANDLE>::zeroed();
        let result = self
            .get_memory_win32_handle_khr_raw(get_win32_handle_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::vk::Result<HANDLE> {
        let mut value = core::mem::MaybeUninit::<HANDLE>::zeroed();
        let result = self.get_memory_win32_handle_nv_raw(memory, handle_type, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        memory_win32_handle_properties: &mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_memory_win32_handle_properties_khr_raw(
            handle_type,
            handle,
            memory_win32_handle_properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::vk::Result<zx_handle_t> {
        let mut value = core::mem::MaybeUninit::<zx_handle_t>::zeroed();
        let result = self.get_memory_zircon_handle_fuchsia_raw(
            get_zircon_handle_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> crate::vk::Result<()> {
        self.get_memory_zircon_handle_properties_fuchsia_raw(
            handle_type,
            zircon_handle,
            memory_zircon_handle_properties as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_micromap_build_sizes_ext(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT<'_>,
        size_info: &mut MicromapBuildSizesInfoEXT<'_>,
    ) {
        self.get_micromap_build_sizes_ext_raw(
            build_type,
            build_info as *const _,
            size_info as *mut _,
        )
    }
    #[cfg(feature = "ohos")]
    pub unsafe fn get_native_buffer_properties_ohos(
        &self,
        buffer: &OH_NativeBuffer,
        properties: &mut NativeBufferPropertiesOHOS<'_>,
    ) -> crate::vk::Result<()> {
        self.get_native_buffer_properties_ohos_raw(buffer as *const _, properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV<'_>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        self.get_partitioned_acceleration_structures_build_sizes_nv_raw(
            info as *const _,
            size_info as *mut _,
        )
    }
    pub unsafe fn get_past_presentation_timing_ext(
        &self,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT<'_>,
        past_presentation_timing_properties: &mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.get_past_presentation_timing_ext_raw(
            past_presentation_timing_info as *const _,
            past_presentation_timing_properties as *mut _,
        )
        .into_result()
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<PastPresentationTimingGOOGLE<'static>>> {
        let mut count = 0;
        self.get_past_presentation_timing_google_raw(swapchain, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<PastPresentationTimingGOOGLE<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_past_presentation_timing_google_raw(
                swapchain,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_performance_parameter_intel(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::vk::Result<PerformanceValueINTEL<'static>> {
        let mut value = core::mem::MaybeUninit::<PerformanceValueINTEL<'static>>::zeroed();
        let result = self.get_performance_parameter_intel_raw(parameter, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_pipeline_binary_data_khr(
        &self,
        info: &PipelineBinaryDataInfoKHR<'_>,
        pipeline_binary_key: &mut PipelineBinaryKeyKHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_pipeline_binary_data_khr_raw(
            info as *const _,
            pipeline_binary_key as *mut _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        self.get_pipeline_binary_data_khr_raw(
            info as *const _,
            pipeline_binary_key as *mut _,
            &mut count,
            values.as_mut_ptr().cast(),
        )
        .into_result()?;
        values.truncate(count);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_pipeline_cache_data(
        &self,
        pipeline_cache: PipelineCache,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_pipeline_cache_data_raw(pipeline_cache, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result = self.get_pipeline_cache_data_raw(
                pipeline_cache,
                &mut count,
                values.as_mut_ptr().cast(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_pipeline_executable_internal_representations_khr(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<PipelineExecutableInternalRepresentationKHR<'static>>>
    {
        let mut count = 0;
        self.get_pipeline_executable_internal_representations_khr_raw(
            executable_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PipelineExecutableInternalRepresentationKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_pipeline_executable_internal_representations_khr_raw(
                executable_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr_count(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_pipeline_executable_internal_representations_khr_raw(
            executable_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr_into(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        values: &mut [PipelineExecutableInternalRepresentationKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_pipeline_executable_internal_representations_khr_raw(
            executable_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_pipeline_executable_properties_khr(
        &self,
        pipeline_info: &PipelineInfoKHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<PipelineExecutablePropertiesKHR<'static>>> {
        let mut count = 0;
        self.get_pipeline_executable_properties_khr_raw(
            pipeline_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PipelineExecutablePropertiesKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_pipeline_executable_properties_khr_raw(
                pipeline_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_pipeline_executable_properties_khr_count(
        &self,
        pipeline_info: &PipelineInfoKHR<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_pipeline_executable_properties_khr_raw(
            pipeline_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_pipeline_executable_properties_khr_into(
        &self,
        pipeline_info: &PipelineInfoKHR<'_>,
        values: &mut [PipelineExecutablePropertiesKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_pipeline_executable_properties_khr_raw(
            pipeline_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
    ) -> crate::vk::Result<alloc::vec::Vec<PipelineExecutableStatisticKHR<'static>>> {
        let mut count = 0;
        self.get_pipeline_executable_statistics_khr_raw(
            executable_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<PipelineExecutableStatisticKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_pipeline_executable_statistics_khr_raw(
                executable_info as *const _,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_pipeline_executable_statistics_khr_count(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_pipeline_executable_statistics_khr_raw(
            executable_info as *const _,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_pipeline_executable_statistics_khr_into(
        &self,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        values: &mut [PipelineExecutableStatisticKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_pipeline_executable_statistics_khr_raw(
            executable_info as *const _,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn get_pipeline_indirect_device_address_nv(
        &self,
        info: &PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> u64 {
        self.get_pipeline_indirect_device_address_nv_raw(info as *const _)
    }
    pub unsafe fn get_pipeline_indirect_memory_requirements_nv(
        &self,
        create_info: &ComputePipelineCreateInfo<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_pipeline_indirect_memory_requirements_nv_raw(
            create_info as *const _,
            memory_requirements as *mut _,
        )
    }
    pub unsafe fn get_pipeline_key_khr(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR<'_>>,
        pipeline_key: &mut PipelineBinaryKeyKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.get_pipeline_key_khr_raw(
            pipeline_create_info.map_or(core::ptr::null(), |x| x as *const _),
            pipeline_key as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_pipeline_properties_ext(
        &self,
        pipeline_info: &PipelineInfoKHR<'_>,
        pipeline_properties: &mut BaseOutStructure<'_>,
    ) -> crate::vk::Result<()> {
        self.get_pipeline_properties_ext_raw(
            pipeline_info as *const _,
            pipeline_properties as *mut _,
        )
        .into_result()
    }
    pub unsafe fn get_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        self.get_private_data_raw(
            object_type,
            object_handle,
            private_data_slot,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_private_data_ext(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        self.get_private_data_ext_raw(
            object_type,
            object_handle,
            private_data_slot,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_query_pool_results(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut [u8],
        stride: u64,
        flags: QueryResultFlags,
    ) -> crate::vk::Result<VkResult> {
        let result = self.get_query_pool_results_raw(
            query_pool,
            first_query,
            query_count,
            data.len() as _,
            data.as_mut_ptr().cast(),
            stride,
            flags,
        );
        result.into_result()?;
        Ok(result)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_queue_checkpoint_data2_nv(
        &self,
        queue: Queue,
    ) -> crate::vk::Result<alloc::vec::Vec<CheckpointData2NV<'static>>> {
        let mut count = 0;
        self.get_queue_checkpoint_data2_nv_raw(queue, &mut count, core::ptr::null_mut());
        let mut values: alloc::vec::Vec<CheckpointData2NV<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_queue_checkpoint_data2_nv_raw(queue, &mut count, values.as_mut_ptr());
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_queue_checkpoint_data2_nv_count(&self, queue: Queue) -> usize {
        let mut count = 0;
        self.get_queue_checkpoint_data2_nv_raw(queue, &mut count, core::ptr::null_mut());
        count as usize
    }
    pub unsafe fn get_queue_checkpoint_data2_nv_into(
        &self,
        queue: Queue,
        values: &mut [CheckpointData2NV<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_queue_checkpoint_data2_nv_raw(queue, &mut count, values.as_mut_ptr());
        count as usize
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_queue_checkpoint_data_nv(
        &self,
        queue: Queue,
    ) -> crate::vk::Result<alloc::vec::Vec<CheckpointDataNV<'static>>> {
        let mut count = 0;
        self.get_queue_checkpoint_data_nv_raw(queue, &mut count, core::ptr::null_mut());
        let mut values: alloc::vec::Vec<CheckpointDataNV<'static>> =
            alloc::vec![Default::default(); count as usize];
        self.get_queue_checkpoint_data_nv_raw(queue, &mut count, values.as_mut_ptr());
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_queue_checkpoint_data_nv_count(&self, queue: Queue) -> usize {
        let mut count = 0;
        self.get_queue_checkpoint_data_nv_raw(queue, &mut count, core::ptr::null_mut());
        count as usize
    }
    pub unsafe fn get_queue_checkpoint_data_nv_into(
        &self,
        queue: Queue,
        values: &mut [CheckpointDataNV<'_>],
    ) -> usize {
        let mut count = values.len() as _;
        self.get_queue_checkpoint_data_nv_raw(queue, &mut count, values.as_mut_ptr());
        count as usize
    }
    pub unsafe fn get_ray_tracing_capture_replay_shader_group_handles_khr(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::vk::Result<()> {
        self.get_ray_tracing_capture_replay_shader_group_handles_khr_raw(
            pipeline,
            first_group,
            group_count,
            data.len() as _,
            data.as_mut_ptr().cast(),
        )
        .into_result()
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_khr(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::vk::Result<()> {
        self.get_ray_tracing_shader_group_handles_khr_raw(
            pipeline,
            first_group,
            group_count,
            data.len() as _,
            data.as_mut_ptr().cast(),
        )
        .into_result()
    }
    pub unsafe fn get_ray_tracing_shader_group_handles_nv(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> crate::vk::Result<()> {
        self.get_ray_tracing_shader_group_handles_nv_raw(
            pipeline,
            first_group,
            group_count,
            data.len() as _,
            data.as_mut_ptr().cast(),
        )
        .into_result()
    }
    pub unsafe fn get_ray_tracing_shader_group_stack_size_khr(
        &self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> u64 {
        self.get_ray_tracing_shader_group_stack_size_khr_raw(pipeline, group, group_shader)
    }
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<RefreshCycleDurationGOOGLE<'static>> {
        let mut value = core::mem::MaybeUninit::<RefreshCycleDurationGOOGLE<'static>>::zeroed();
        let result = self.get_refresh_cycle_duration_google_raw(swapchain, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_render_area_granularity(&self, render_pass: RenderPass) -> Extent2D<'static> {
        let mut value = core::mem::MaybeUninit::<Extent2D<'static>>::zeroed();
        self.get_render_area_granularity_raw(render_pass, value.as_mut_ptr());
        value.assume_init()
    }
    pub unsafe fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D<'static> {
        let mut value = core::mem::MaybeUninit::<Extent2D<'static>>::zeroed();
        self.get_rendering_area_granularity_raw(
            rendering_area_info as *const _,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    pub unsafe fn get_rendering_area_granularity_khr(
        &self,
        rendering_area_info: &RenderingAreaInfo<'_>,
    ) -> Extent2D<'static> {
        let mut value = core::mem::MaybeUninit::<Extent2D<'static>>::zeroed();
        self.get_rendering_area_granularity_khr_raw(
            rendering_area_info as *const _,
            value.as_mut_ptr(),
        );
        value.assume_init()
    }
    #[cfg(feature = "screen")]
    pub unsafe fn get_screen_buffer_properties_qnx(
        &self,
        buffer: &_screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX<'_>,
    ) -> crate::vk::Result<()> {
        self.get_screen_buffer_properties_qnx_raw(buffer as *const _, properties as *mut _)
            .into_result()
    }
    pub unsafe fn get_semaphore_counter_value(
        &self,
        semaphore: Semaphore,
    ) -> crate::vk::Result<u64> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_semaphore_counter_value_raw(semaphore, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_semaphore_counter_value_khr(
        &self,
        semaphore: Semaphore,
    ) -> crate::vk::Result<u64> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_semaphore_counter_value_khr_raw(semaphore, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        get_fd_info: &SemaphoreGetFdInfoKHR<'_>,
    ) -> crate::vk::Result<core::ffi::c_int> {
        let mut value = core::mem::MaybeUninit::<core::ffi::c_int>::zeroed();
        let result = self.get_semaphore_fd_khr_raw(get_fd_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "win32")]
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> crate::vk::Result<HANDLE> {
        let mut value = core::mem::MaybeUninit::<HANDLE>::zeroed();
        let result = self.get_semaphore_win32_handle_khr_raw(
            get_win32_handle_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::vk::Result<zx_handle_t> {
        let mut value = core::mem::MaybeUninit::<zx_handle_t>::zeroed();
        let result = self.get_semaphore_zircon_handle_fuchsia_raw(
            get_zircon_handle_info as *const _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_shader_binary_data_ext(
        &self,
        shader: ShaderEXT,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_shader_binary_data_ext_raw(shader, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result =
                self.get_shader_binary_data_ext_raw(shader, &mut count, values.as_mut_ptr().cast());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_shader_info_amd(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_shader_info_amd_raw(
            pipeline,
            shader_stage,
            info_type,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result = self.get_shader_info_amd_raw(
                pipeline,
                shader_stage,
                info_type,
                &mut count,
                values.as_mut_ptr().cast(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        create_info: &ShaderModuleCreateInfo<'_>,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        self.get_shader_module_create_info_identifier_ext_raw(
            create_info as *const _,
            identifier as *mut _,
        )
    }
    pub unsafe fn get_shader_module_identifier_ext(
        &self,
        shader_module: ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        self.get_shader_module_identifier_ext_raw(shader_module, identifier as *mut _)
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
    ) -> crate::vk::Result<u64> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_swapchain_counter_ext_raw(swapchain, counter, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_swapchain_images_khr(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<Image>> {
        let mut count = 0;
        self.get_swapchain_images_khr_raw(swapchain, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<Image> = alloc::vec![Image::null(); count as usize];
        loop {
            count = values.len() as _;
            let result =
                self.get_swapchain_images_khr_raw(swapchain, &mut count, values.as_mut_ptr());
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Image::null());
            if values.len() <= count as usize {
                values.resize((count as usize).saturating_mul(2).max(1), Image::null());
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_swapchain_status_khr(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<VkResult> {
        let result = self.get_swapchain_status_khr_raw(swapchain);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn get_swapchain_time_domain_properties_ext(
        &self,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: &mut SwapchainTimeDomainPropertiesEXT<'_>,
    ) -> crate::vk::Result<(u64, VkResult)> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_swapchain_time_domain_properties_ext_raw(
            swapchain,
            swapchain_time_domain_properties as *mut _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((value.assume_init(), result))
    }
    pub unsafe fn get_swapchain_timing_properties_ext(
        &self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: &mut SwapchainTimingPropertiesEXT<'_>,
    ) -> crate::vk::Result<(u64, VkResult)> {
        let mut value = core::mem::MaybeUninit::<u64>::zeroed();
        let result = self.get_swapchain_timing_properties_ext_raw(
            swapchain,
            swapchain_timing_properties as *mut _,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((value.assume_init(), result))
    }
    pub unsafe fn get_tensor_memory_requirements_arm(
        &self,
        info: &TensorMemoryRequirementsInfoARM<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        self.get_tensor_memory_requirements_arm_raw(info as *const _, memory_requirements as *mut _)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        tensors: &[TensorARM],
    ) -> crate::vk::Result<alloc::vec::Vec<HostAddressRangeEXT<'static>>> {
        let mut values = alloc::vec![Default::default(); tensors.len()];
        self.get_tensor_opaque_capture_data_arm_raw(
            tensors.len() as u32,
            tensors.as_ptr(),
            values.as_mut_ptr(),
        )
        .into_result()?;
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_validation_cache_data_ext(
        &self,
        validation_cache: ValidationCacheEXT,
    ) -> crate::vk::Result<alloc::vec::Vec<u8>> {
        let mut count = 0;
        self.get_validation_cache_data_ext_raw(validation_cache, &mut count, core::ptr::null_mut())
            .into_result()?;
        let mut values: alloc::vec::Vec<u8> = alloc::vec![Default::default(); count];
        loop {
            count = values.len() as _;
            let result = self.get_validation_cache_data_ext_raw(
                validation_cache,
                &mut count,
                values.as_mut_ptr().cast(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count, Default::default());
            if values.len() <= count {
                values.resize((count).saturating_mul(2).max(1), Default::default());
            }
        }
        values.truncate(count);
        Ok(values)
    }
    #[cfg(feature = "alloc")]
    pub unsafe fn get_video_session_memory_requirements_khr(
        &self,
        video_session: VideoSessionKHR,
    ) -> crate::vk::Result<alloc::vec::Vec<VideoSessionMemoryRequirementsKHR<'static>>> {
        let mut count = 0;
        self.get_video_session_memory_requirements_khr_raw(
            video_session,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        let mut values: alloc::vec::Vec<VideoSessionMemoryRequirementsKHR<'static>> =
            alloc::vec![Default::default(); count as usize];
        loop {
            count = values.len() as _;
            let result = self.get_video_session_memory_requirements_khr_raw(
                video_session,
                &mut count,
                values.as_mut_ptr(),
            );
            if result != VkResult::INCOMPLETE {
                result.into_result()?;
                break;
            }
            values.resize(count as usize, Default::default());
            if values.len() <= count as usize {
                values.resize(
                    (count as usize).saturating_mul(2).max(1),
                    Default::default(),
                );
            }
        }
        values.truncate(count as usize);
        Ok(values)
    }
    pub unsafe fn get_video_session_memory_requirements_khr_count(
        &self,
        video_session: VideoSessionKHR,
    ) -> crate::vk::Result<usize> {
        let mut count = 0;
        self.get_video_session_memory_requirements_khr_raw(
            video_session,
            &mut count,
            core::ptr::null_mut(),
        )
        .into_result()?;
        Ok(count as usize)
    }
    pub unsafe fn get_video_session_memory_requirements_khr_into(
        &self,
        video_session: VideoSessionKHR,
        values: &mut [VideoSessionMemoryRequirementsKHR<'_>],
    ) -> crate::vk::Result<(usize, VkResult)> {
        let mut count = values.len() as _;
        let result = self.get_video_session_memory_requirements_khr_raw(
            video_session,
            &mut count,
            values.as_mut_ptr(),
        );
        result.into_result()?;
        Ok((count as usize, result))
    }
    pub unsafe fn import_fence_fd_khr(
        &self,
        import_fence_fd_info: &ImportFenceFdInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.import_fence_fd_khr_raw(import_fence_fd_info as *const _)
            .into_result()
    }
    #[cfg(feature = "win32")]
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.import_fence_win32_handle_khr_raw(import_fence_win32_handle_info as *const _)
            .into_result()
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.import_semaphore_fd_khr_raw(import_semaphore_fd_info as *const _)
            .into_result()
    }
    #[cfg(feature = "win32")]
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.import_semaphore_win32_handle_khr_raw(import_semaphore_win32_handle_info as *const _)
            .into_result()
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::vk::Result<()> {
        self.import_semaphore_zircon_handle_fuchsia_raw(
            import_semaphore_zircon_handle_info as *const _,
        )
        .into_result()
    }
    pub unsafe fn initialize_performance_api_intel(
        &self,
        initialize_info: &InitializePerformanceApiInfoINTEL<'_>,
    ) -> crate::vk::Result<()> {
        self.initialize_performance_api_intel_raw(initialize_info as *const _)
            .into_result()
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange<'_>],
    ) -> crate::vk::Result<()> {
        self.invalidate_mapped_memory_ranges_raw(memory_ranges.len() as _, memory_ranges.as_ptr())
            .into_result()
    }
    pub unsafe fn latency_sleep_nv(
        &self,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV<'_>,
    ) -> crate::vk::Result<()> {
        self.latency_sleep_nv_raw(swapchain, sleep_info as *const _)
            .into_result()
    }
    pub unsafe fn map_memory(
        &self,
        memory: DeviceMemory,
        offset: u64,
        size: u64,
        flags: MemoryMapFlags,
    ) -> crate::vk::Result<*mut c_void> {
        let mut value = core::mem::MaybeUninit::<*mut c_void>::zeroed();
        let result = self.map_memory_raw(memory, offset, size, flags, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn map_memory2(
        &self,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::vk::Result<*mut c_void> {
        let mut value = core::mem::MaybeUninit::<*mut c_void>::zeroed();
        let result = self.map_memory2_raw(memory_map_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn map_memory2_khr(
        &self,
        memory_map_info: &MemoryMapInfo<'_>,
    ) -> crate::vk::Result<*mut c_void> {
        let mut value = core::mem::MaybeUninit::<*mut c_void>::zeroed();
        let result = self.map_memory2_khr_raw(memory_map_info as *const _, value.as_mut_ptr());
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> crate::vk::Result<()> {
        self.merge_pipeline_caches_raw(dst_cache, src_caches.len() as _, src_caches.as_ptr())
            .into_result()
    }
    pub unsafe fn merge_validation_caches_ext(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> crate::vk::Result<()> {
        self.merge_validation_caches_ext_raw(dst_cache, src_caches.len() as _, src_caches.as_ptr())
            .into_result()
    }
    pub unsafe fn queue_begin_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        self.queue_begin_debug_utils_label_ext_raw(queue, label_info as *const _)
    }
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info: &[BindSparseInfo<'_>],
        fence: Fence,
    ) -> crate::vk::Result<()> {
        self.queue_bind_sparse_raw(queue, bind_info.len() as _, bind_info.as_ptr(), fence)
            .into_result()
    }
    pub unsafe fn queue_end_debug_utils_label_ext(&self, queue: Queue) {
        self.queue_end_debug_utils_label_ext_raw(queue)
    }
    pub unsafe fn queue_insert_debug_utils_label_ext(
        &self,
        queue: Queue,
        label_info: &DebugUtilsLabelEXT<'_>,
    ) {
        self.queue_insert_debug_utils_label_ext_raw(queue, label_info as *const _)
    }
    pub unsafe fn queue_notify_out_of_band_nv(
        &self,
        queue: Queue,
        queue_type_info: &OutOfBandQueueTypeInfoNV<'_>,
    ) {
        self.queue_notify_out_of_band_nv_raw(queue, queue_type_info as *const _)
    }
    pub unsafe fn queue_present_khr(
        &self,
        queue: Queue,
        present_info: &PresentInfoKHR<'_>,
    ) -> crate::vk::Result<bool> {
        let result = self.queue_present_khr_raw(queue, present_info as *const _);
        result.into_result()?;
        Ok(result == VkResult::SUBOPTIMAL_KHR)
    }
    pub unsafe fn queue_set_perf_hint_qcom(
        &self,
        queue: Queue,
        perf_hint_info: &PerfHintInfoQCOM<'_>,
    ) -> crate::vk::Result<()> {
        self.queue_set_perf_hint_qcom_raw(queue, perf_hint_info as *const _)
            .into_result()
    }
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::vk::Result<()> {
        self.queue_set_performance_configuration_intel_raw(queue, configuration)
            .into_result()
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo<'_>],
        fence: Fence,
    ) -> crate::vk::Result<()> {
        self.queue_submit_raw(queue, submits.len() as _, submits.as_ptr(), fence)
            .into_result()
    }
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::vk::Result<()> {
        self.queue_submit2_raw(queue, submits.len() as _, submits.as_ptr(), fence)
            .into_result()
    }
    pub unsafe fn queue_submit2_khr(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2<'_>],
        fence: Fence,
    ) -> crate::vk::Result<()> {
        self.queue_submit2_khr_raw(queue, submits.len() as _, submits.as_ptr(), fence)
            .into_result()
    }
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> crate::vk::Result<()> {
        self.queue_wait_idle_raw(queue).into_result()
    }
    pub unsafe fn register_custom_border_color_ext(
        &self,
        border_color: &SamplerCustomBorderColorCreateInfoEXT<'_>,
        request_index: bool,
    ) -> crate::vk::Result<u32> {
        let mut value = core::mem::MaybeUninit::<u32>::zeroed();
        let result = self.register_custom_border_color_ext_raw(
            border_color as *const _,
            request_index as u32,
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn register_device_event_ext(
        &self,
        device_event_info: &DeviceEventInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Fence> {
        let mut value = core::mem::MaybeUninit::<Fence>::zeroed();
        let result = self.register_device_event_ext_raw(
            device_event_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn register_display_event_ext(
        &self,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<Fence> {
        let mut value = core::mem::MaybeUninit::<Fence>::zeroed();
        let result = self.register_display_event_ext_raw(
            display,
            display_event_info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
            value.as_mut_ptr(),
        );
        result.into_result()?;
        Ok(value.assume_init())
    }
    pub unsafe fn release_captured_pipeline_data_khr(
        &self,
        info: &ReleaseCapturedPipelineDataInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::vk::Result<()> {
        self.release_captured_pipeline_data_khr_raw(
            info as *const _,
            allocator.map_or(core::ptr::null(), |x| x as *const _),
        )
        .into_result()
    }
    #[cfg(feature = "win32")]
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        swapchain: SwapchainKHR,
    ) -> crate::vk::Result<()> {
        self.release_full_screen_exclusive_mode_ext_raw(swapchain)
            .into_result()
    }
    pub unsafe fn release_performance_configuration_intel(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::vk::Result<()> {
        self.release_performance_configuration_intel_raw(configuration)
            .into_result()
    }
    pub unsafe fn release_profiling_lock_khr(&self) {
        self.release_profiling_lock_khr_raw()
    }
    pub unsafe fn release_swapchain_images_ext(
        &self,
        release_info: &ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.release_swapchain_images_ext_raw(release_info as *const _)
            .into_result()
    }
    pub unsafe fn release_swapchain_images_khr(
        &self,
        release_info: &ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.release_swapchain_images_khr_raw(release_info as *const _)
            .into_result()
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> crate::vk::Result<()> {
        self.reset_command_buffer_raw(command_buffer, flags)
            .into_result()
    }
    pub unsafe fn reset_command_pool(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::vk::Result<()> {
        self.reset_command_pool_raw(command_pool, flags)
            .into_result()
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::vk::Result<()> {
        self.reset_descriptor_pool_raw(descriptor_pool, flags)
            .into_result()
    }
    pub unsafe fn reset_event(&self, event: Event) -> crate::vk::Result<()> {
        self.reset_event_raw(event).into_result()
    }
    pub unsafe fn reset_fences(&self, fences: &[Fence]) -> crate::vk::Result<()> {
        self.reset_fences_raw(fences.len() as _, fences.as_ptr())
            .into_result()
    }
    pub unsafe fn reset_query_pool(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        self.reset_query_pool_raw(query_pool, first_query, query_count)
    }
    pub unsafe fn reset_query_pool_ext(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        self.reset_query_pool_ext_raw(query_pool, first_query, query_count)
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn set_buffer_collection_buffer_constraints_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA<'_>,
    ) -> crate::vk::Result<()> {
        self.set_buffer_collection_buffer_constraints_fuchsia_raw(
            collection,
            buffer_constraints_info as *const _,
        )
        .into_result()
    }
    #[cfg(feature = "fuchsia")]
    pub unsafe fn set_buffer_collection_image_constraints_fuchsia(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA<'_>,
    ) -> crate::vk::Result<()> {
        self.set_buffer_collection_image_constraints_fuchsia_raw(
            collection,
            image_constraints_info as *const _,
        )
        .into_result()
    }
    pub unsafe fn set_debug_utils_object_name_ext(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.set_debug_utils_object_name_ext_raw(name_info as *const _)
            .into_result()
    }
    pub unsafe fn set_debug_utils_object_tag_ext(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT<'_>,
    ) -> crate::vk::Result<()> {
        self.set_debug_utils_object_tag_ext_raw(tag_info as *const _)
            .into_result()
    }
    pub unsafe fn set_device_memory_priority_ext(&self, memory: DeviceMemory, priority: f32) {
        self.set_device_memory_priority_ext_raw(memory, priority)
    }
    pub unsafe fn set_event(&self, event: Event) -> crate::vk::Result<()> {
        self.set_event_raw(event).into_result()
    }
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        swapchains: &[SwapchainKHR],
        metadata: &[HdrMetadataEXT<'_>],
    ) {
        assert_eq!(
            swapchains.len(),
            metadata.len(),
            "swapchains and metadata must have matching swapchain_count lengths"
        );
        self.set_hdr_metadata_ext_raw(
            swapchains.len() as _,
            swapchains.as_ptr(),
            metadata.as_ptr(),
        )
    }
    pub unsafe fn set_latency_marker_nv(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV<'_>,
    ) {
        self.set_latency_marker_nv_raw(swapchain, latency_marker_info as *const _)
    }
    pub unsafe fn set_latency_sleep_mode_nv(
        &self,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV<'_>,
    ) -> crate::vk::Result<()> {
        self.set_latency_sleep_mode_nv_raw(swapchain, sleep_mode_info as *const _)
            .into_result()
    }
    pub unsafe fn set_local_dimming_amd(
        &self,
        swap_chain: SwapchainKHR,
        local_dimming_enable: bool,
    ) {
        self.set_local_dimming_amd_raw(swap_chain, local_dimming_enable as u32)
    }
    pub unsafe fn set_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::vk::Result<()> {
        self.set_private_data_raw(object_type, object_handle, private_data_slot, data)
            .into_result()
    }
    pub unsafe fn set_private_data_ext(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> crate::vk::Result<()> {
        self.set_private_data_ext_raw(object_type, object_handle, private_data_slot, data)
            .into_result()
    }
    pub unsafe fn set_swapchain_present_timing_queue_size_ext(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> crate::vk::Result<VkResult> {
        let result = self.set_swapchain_present_timing_queue_size_ext_raw(swapchain, size);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn signal_semaphore(
        &self,
        signal_info: &SemaphoreSignalInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.signal_semaphore_raw(signal_info as *const _)
            .into_result()
    }
    pub unsafe fn signal_semaphore_khr(
        &self,
        signal_info: &SemaphoreSignalInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.signal_semaphore_khr_raw(signal_info as *const _)
            .into_result()
    }
    pub unsafe fn transition_image_layout(
        &self,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.transition_image_layout_raw(transitions.len() as _, transitions.as_ptr())
            .into_result()
    }
    pub unsafe fn transition_image_layout_ext(
        &self,
        transitions: &[HostImageLayoutTransitionInfo<'_>],
    ) -> crate::vk::Result<()> {
        self.transition_image_layout_ext_raw(transitions.len() as _, transitions.as_ptr())
            .into_result()
    }
    pub unsafe fn trim_command_pool(&self, command_pool: CommandPool, flags: CommandPoolTrimFlags) {
        self.trim_command_pool_raw(command_pool, flags)
    }
    pub unsafe fn trim_command_pool_khr(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        self.trim_command_pool_khr_raw(command_pool, flags)
    }
    pub unsafe fn uninitialize_performance_api_intel(&self) {
        self.uninitialize_performance_api_intel_raw()
    }
    pub unsafe fn unmap_memory(&self, memory: DeviceMemory) {
        self.unmap_memory_raw(memory)
    }
    pub unsafe fn unmap_memory2(
        &self,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.unmap_memory2_raw(memory_unmap_info as *const _)
            .into_result()
    }
    pub unsafe fn unmap_memory2_khr(
        &self,
        memory_unmap_info: &MemoryUnmapInfo<'_>,
    ) -> crate::vk::Result<()> {
        self.unmap_memory2_khr_raw(memory_unmap_info as *const _)
            .into_result()
    }
    pub unsafe fn unregister_custom_border_color_ext(&self, index: u32) {
        self.unregister_custom_border_color_ext_raw(index)
    }
    pub unsafe fn update_descriptor_set_with_template(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
    ) {
        self.update_descriptor_set_with_template_raw(
            descriptor_set,
            descriptor_update_template,
            data as *const _,
        )
    }
    pub unsafe fn update_descriptor_set_with_template_khr(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
    ) {
        self.update_descriptor_set_with_template_khr_raw(
            descriptor_set,
            descriptor_update_template,
            data as *const _,
        )
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        descriptor_writes: &[WriteDescriptorSet<'_>],
        descriptor_copies: &[CopyDescriptorSet<'_>],
    ) {
        self.update_descriptor_sets_raw(
            descriptor_writes.len() as _,
            descriptor_writes.as_ptr(),
            descriptor_copies.len() as _,
            descriptor_copies.as_ptr(),
        )
    }
    pub unsafe fn update_indirect_execution_set_pipeline_ext(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT<'_>],
    ) {
        self.update_indirect_execution_set_pipeline_ext_raw(
            indirect_execution_set,
            execution_set_writes.len() as _,
            execution_set_writes.as_ptr(),
        )
    }
    pub unsafe fn update_indirect_execution_set_shader_ext(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT<'_>],
    ) {
        self.update_indirect_execution_set_shader_ext_raw(
            indirect_execution_set,
            execution_set_writes.len() as _,
            execution_set_writes.as_ptr(),
        )
    }
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> crate::vk::Result<()> {
        self.update_video_session_parameters_khr_raw(
            video_session_parameters,
            update_info as *const _,
        )
        .into_result()
    }
    pub unsafe fn wait_for_fences(
        &self,
        fences: &[Fence],
        wait_all: bool,
        timeout: u64,
    ) -> crate::vk::Result<VkResult> {
        let result =
            self.wait_for_fences_raw(fences.len() as _, fences.as_ptr(), wait_all as u32, timeout);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn wait_for_present2_khr(
        &self,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR<'_>,
    ) -> crate::vk::Result<VkResult> {
        let result = self.wait_for_present2_khr_raw(swapchain, present_wait2_info as *const _);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn wait_for_present_khr(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> crate::vk::Result<VkResult> {
        let result = self.wait_for_present_khr_raw(swapchain, present_id, timeout);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> crate::vk::Result<VkResult> {
        let result = self.wait_semaphores_raw(wait_info as *const _, timeout);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn wait_semaphores_khr(
        &self,
        wait_info: &SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> crate::vk::Result<VkResult> {
        let result = self.wait_semaphores_khr_raw(wait_info as *const _, timeout);
        result.into_result()?;
        Ok(result)
    }
    pub unsafe fn write_acceleration_structures_properties_khr(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> crate::vk::Result<()> {
        self.write_acceleration_structures_properties_khr_raw(
            acceleration_structures.len() as _,
            acceleration_structures.as_ptr(),
            query_type,
            data.len() as _,
            data.as_mut_ptr().cast(),
            stride,
        )
        .into_result()
    }
    pub unsafe fn write_micromaps_properties_ext(
        &self,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> crate::vk::Result<()> {
        self.write_micromaps_properties_ext_raw(
            micromaps.len() as _,
            micromaps.as_ptr(),
            query_type,
            data.len() as _,
            data.as_mut_ptr().cast(),
            stride,
        )
        .into_result()
    }
    pub unsafe fn write_resource_descriptors_ext(
        &self,
        resources: &[ResourceDescriptorInfoEXT<'_>],
        descriptors: &[HostAddressRangeEXT<'_>],
    ) -> crate::vk::Result<()> {
        assert_eq!(
            resources.len(),
            descriptors.len(),
            "resources and descriptors must have matching resource_count lengths"
        );
        self.write_resource_descriptors_ext_raw(
            resources.len() as _,
            resources.as_ptr(),
            descriptors.as_ptr(),
        )
        .into_result()
    }
    pub unsafe fn write_sampler_descriptors_ext(
        &self,
        samplers: &[SamplerCreateInfo<'_>],
        descriptors: &[HostAddressRangeEXT<'_>],
    ) -> crate::vk::Result<()> {
        assert_eq!(
            samplers.len(),
            descriptors.len(),
            "samplers and descriptors must have matching sampler_count lengths"
        );
        self.write_sampler_descriptors_ext_raw(
            samplers.len() as _,
            samplers.as_ptr(),
            descriptors.as_ptr(),
        )
        .into_result()
    }
}
