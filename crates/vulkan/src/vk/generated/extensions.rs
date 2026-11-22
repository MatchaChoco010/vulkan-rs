#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

use core::ffi::CStr;

/// The name of the provisional device extension
/// [`VK_AMDX_dense_geometry_format`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_dense_geometry_format.html),
/// the C macro `VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME) and (Vulkan 1.4,
/// [`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME)).
#[doc(alias = "VK_AMDX_dense_geometry_format")]
#[doc(alias = "VK_AMDX_DENSE_GEOMETRY_FORMAT_EXTENSION_NAME")]
#[cfg(feature = "beta")]
pub const AMDX_DENSE_GEOMETRY_FORMAT_NAME: &CStr = c"VK_AMDX_dense_geometry_format";
/// The revision of the
/// [`VK_AMDX_dense_geometry_format`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_dense_geometry_format.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION`.
#[doc(alias = "VK_AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION")]
#[cfg(feature = "beta")]
pub const AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION: u32 = 1;

/// The name of the provisional device extension
/// [`VK_AMDX_shader_enqueue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_shader_enqueue.html),
/// the C macro `VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME),
/// [`VK_KHR_spirv_1_4`](KHR_SPIRV_1_4_NAME) and
/// [`VK_EXT_extended_dynamic_state`](EXT_EXTENDED_DYNAMIC_STATE_NAME)) or Vulkan 1.3),
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME) and
/// [`VK_KHR_pipeline_library`](KHR_PIPELINE_LIBRARY_NAME).
#[doc(alias = "VK_AMDX_shader_enqueue")]
#[doc(alias = "VK_AMDX_SHADER_ENQUEUE_EXTENSION_NAME")]
#[cfg(feature = "beta")]
pub const AMDX_SHADER_ENQUEUE_NAME: &CStr = c"VK_AMDX_shader_enqueue";
/// The revision of the
/// [`VK_AMDX_shader_enqueue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMDX_shader_enqueue.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION`.
#[doc(alias = "VK_AMDX_SHADER_ENQUEUE_SPEC_VERSION")]
#[cfg(feature = "beta")]
pub const AMDX_SHADER_ENQUEUE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_AMD_anti_lag`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_anti_lag.html),
/// the C macro `VK_AMD_ANTI_LAG_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_anti_lag")]
#[doc(alias = "VK_AMD_ANTI_LAG_EXTENSION_NAME")]
pub const AMD_ANTI_LAG_NAME: &CStr = c"VK_AMD_anti_lag";
/// The revision of the
/// [`VK_AMD_anti_lag`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_anti_lag.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_ANTI_LAG_SPEC_VERSION`.
#[doc(alias = "VK_AMD_ANTI_LAG_SPEC_VERSION")]
pub const AMD_ANTI_LAG_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_buffer_marker`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_buffer_marker.html),
/// the C macro `VK_AMD_BUFFER_MARKER_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_buffer_marker")]
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_NAME: &CStr = c"VK_AMD_buffer_marker";
/// The revision of the
/// [`VK_AMD_buffer_marker`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_buffer_marker.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_BUFFER_MARKER_SPEC_VERSION`.
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_device_coherent_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_device_coherent_memory.html),
/// the C macro `VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_device_coherent_memory")]
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME")]
pub const AMD_DEVICE_COHERENT_MEMORY_NAME: &CStr = c"VK_AMD_device_coherent_memory";
/// The revision of the
/// [`VK_AMD_device_coherent_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_device_coherent_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION")]
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_display_native_hdr`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_display_native_hdr.html),
/// the C macro `VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME) and
/// [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_AMD_display_native_hdr")]
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME")]
pub const AMD_DISPLAY_NATIVE_HDR_NAME: &CStr = c"VK_AMD_display_native_hdr";
/// The revision of the
/// [`VK_AMD_display_native_hdr`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_display_native_hdr.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION`.
#[doc(alias = "VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION")]
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_draw_indirect_count`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_draw_indirect_count.html),
/// the C macro `VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME`.
///
/// Promoted to [`VK_KHR_draw_indirect_count`](KHR_DRAW_INDIRECT_COUNT_NAME).
#[doc(alias = "VK_AMD_draw_indirect_count")]
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const AMD_DRAW_INDIRECT_COUNT_NAME: &CStr = c"VK_AMD_draw_indirect_count";
/// The revision of the
/// [`VK_AMD_draw_indirect_count`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_draw_indirect_count.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION`.
#[doc(alias = "VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_AMD_gcn_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gcn_shader.html),
/// the C macro `VK_AMD_GCN_SHADER_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_gcn_shader")]
#[doc(alias = "VK_AMD_GCN_SHADER_EXTENSION_NAME")]
pub const AMD_GCN_SHADER_NAME: &CStr = c"VK_AMD_gcn_shader";
/// The revision of the
/// [`VK_AMD_gcn_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gcn_shader.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_GCN_SHADER_SPEC_VERSION`.
#[doc(alias = "VK_AMD_GCN_SHADER_SPEC_VERSION")]
pub const AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_gpa_interface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpa_interface.html),
/// the C macro `VK_AMD_GPA_INTERFACE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_gpa_interface")]
#[doc(alias = "VK_AMD_GPA_INTERFACE_EXTENSION_NAME")]
pub const AMD_GPA_INTERFACE_NAME: &CStr = c"VK_AMD_gpa_interface";
/// The revision of the
/// [`VK_AMD_gpa_interface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpa_interface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_GPA_INTERFACE_SPEC_VERSION`.
#[doc(alias = "VK_AMD_GPA_INTERFACE_SPEC_VERSION")]
pub const AMD_GPA_INTERFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_gpu_shader_half_float`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpu_shader_half_float.html),
/// the C macro `VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME`.
///
/// Deprecated by [`VK_KHR_shader_float16_int8`](KHR_SHADER_FLOAT16_INT8_NAME).
#[doc(alias = "VK_AMD_gpu_shader_half_float")]
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_HALF_FLOAT_NAME: &CStr = c"VK_AMD_gpu_shader_half_float";
/// The revision of the
/// [`VK_AMD_gpu_shader_half_float`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpu_shader_half_float.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION`.
#[doc(alias = "VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION")]
pub const AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_AMD_gpu_shader_int16`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpu_shader_int16.html),
/// the C macro `VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME`.
///
/// Deprecated by [`VK_KHR_shader_float16_int8`](KHR_SHADER_FLOAT16_INT8_NAME).
#[doc(alias = "VK_AMD_gpu_shader_int16")]
#[doc(alias = "VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME")]
pub const AMD_GPU_SHADER_INT16_NAME: &CStr = c"VK_AMD_gpu_shader_int16";
/// The revision of the
/// [`VK_AMD_gpu_shader_int16`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_gpu_shader_int16.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_GPU_SHADER_INT16_SPEC_VERSION`.
#[doc(alias = "VK_AMD_GPU_SHADER_INT16_SPEC_VERSION")]
pub const AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_AMD_memory_overallocation_behavior`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_memory_overallocation_behavior.html),
/// the C macro `VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_memory_overallocation_behavior")]
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_NAME: &CStr = c"VK_AMD_memory_overallocation_behavior";
/// The revision of the
/// [`VK_AMD_memory_overallocation_behavior`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_memory_overallocation_behavior.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION`.
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_mixed_attachment_samples`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_mixed_attachment_samples.html),
/// the C macro `VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_mixed_attachment_samples")]
#[doc(alias = "VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME")]
pub const AMD_MIXED_ATTACHMENT_SAMPLES_NAME: &CStr = c"VK_AMD_mixed_attachment_samples";
/// The revision of the
/// [`VK_AMD_mixed_attachment_samples`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_mixed_attachment_samples.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION`.
#[doc(alias = "VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION")]
pub const AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_negative_viewport_height`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_negative_viewport_height.html),
/// the C macro `VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME`.
///
/// Obsoleted by [`VK_KHR_maintenance1`](KHR_MAINTENANCE_1_NAME).
#[doc(alias = "VK_AMD_negative_viewport_height")]
#[doc(alias = "VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME")]
pub const AMD_NEGATIVE_VIEWPORT_HEIGHT_NAME: &CStr = c"VK_AMD_negative_viewport_height";
/// The revision of the
/// [`VK_AMD_negative_viewport_height`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_negative_viewport_height.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION`.
#[doc(alias = "VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION")]
pub const AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_pipeline_compiler_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_pipeline_compiler_control.html),
/// the C macro `VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_pipeline_compiler_control")]
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME")]
pub const AMD_PIPELINE_COMPILER_CONTROL_NAME: &CStr = c"VK_AMD_pipeline_compiler_control";
/// The revision of the
/// [`VK_AMD_pipeline_compiler_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_pipeline_compiler_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION")]
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_rasterization_order`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_rasterization_order.html),
/// the C macro `VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_rasterization_order")]
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME")]
pub const AMD_RASTERIZATION_ORDER_NAME: &CStr = c"VK_AMD_rasterization_order";
/// The revision of the
/// [`VK_AMD_rasterization_order`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_rasterization_order.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION`.
#[doc(alias = "VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION")]
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_ballot`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_ballot.html),
/// the C macro `VK_AMD_SHADER_BALLOT_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_ballot")]
#[doc(alias = "VK_AMD_SHADER_BALLOT_EXTENSION_NAME")]
pub const AMD_SHADER_BALLOT_NAME: &CStr = c"VK_AMD_shader_ballot";
/// The revision of the
/// [`VK_AMD_shader_ballot`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_ballot.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_BALLOT_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_BALLOT_SPEC_VERSION")]
pub const AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_core_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties.html),
/// the C macro `VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_shader_core_properties")]
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_NAME: &CStr = c"VK_AMD_shader_core_properties";
/// The revision of the
/// [`VK_AMD_shader_core_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_AMD_shader_core_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties2.html),
/// the C macro `VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME`.
///
/// Requires [`VK_AMD_shader_core_properties`](AMD_SHADER_CORE_PROPERTIES_NAME).
#[doc(alias = "VK_AMD_shader_core_properties2")]
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME")]
pub const AMD_SHADER_CORE_PROPERTIES_2_NAME: &CStr = c"VK_AMD_shader_core_properties2";
/// The revision of the
/// [`VK_AMD_shader_core_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_core_properties2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION")]
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_early_and_late_fragment_tests`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_early_and_late_fragment_tests.html),
/// the C macro `VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_shader_early_and_late_fragment_tests")]
#[doc(alias = "VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_EXTENSION_NAME")]
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_NAME: &CStr =
    c"VK_AMD_shader_early_and_late_fragment_tests";
/// The revision of the
/// [`VK_AMD_shader_early_and_late_fragment_tests`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_early_and_late_fragment_tests.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION")]
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_explicit_vertex_parameter`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_explicit_vertex_parameter.html),
/// the C macro `VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_explicit_vertex_parameter")]
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_NAME: &CStr =
    c"VK_AMD_shader_explicit_vertex_parameter";
/// The revision of the
/// [`VK_AMD_shader_explicit_vertex_parameter`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_explicit_vertex_parameter.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION")]
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_fragment_mask`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_fragment_mask.html),
/// the C macro `VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_fragment_mask")]
#[doc(alias = "VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME")]
pub const AMD_SHADER_FRAGMENT_MASK_NAME: &CStr = c"VK_AMD_shader_fragment_mask";
/// The revision of the
/// [`VK_AMD_shader_fragment_mask`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_fragment_mask.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION")]
pub const AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_image_load_store_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_image_load_store_lod.html),
/// the C macro `VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_image_load_store_lod")]
#[doc(alias = "VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME")]
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_NAME: &CStr = c"VK_AMD_shader_image_load_store_lod";
/// The revision of the
/// [`VK_AMD_shader_image_load_store_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_image_load_store_lod.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION")]
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_info.html),
/// the C macro `VK_AMD_SHADER_INFO_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_info")]
#[doc(alias = "VK_AMD_SHADER_INFO_EXTENSION_NAME")]
pub const AMD_SHADER_INFO_NAME: &CStr = c"VK_AMD_shader_info";
/// The revision of the
/// [`VK_AMD_shader_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_info.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_INFO_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_INFO_SPEC_VERSION")]
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_shader_trinary_minmax`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_trinary_minmax.html),
/// the C macro `VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME`.
#[doc(alias = "VK_AMD_shader_trinary_minmax")]
#[doc(alias = "VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME")]
pub const AMD_SHADER_TRINARY_MINMAX_NAME: &CStr = c"VK_AMD_shader_trinary_minmax";
/// The revision of the
/// [`VK_AMD_shader_trinary_minmax`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_shader_trinary_minmax.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION`.
#[doc(alias = "VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION")]
pub const AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_AMD_texture_gather_bias_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_texture_gather_bias_lod.html),
/// the C macro `VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_AMD_texture_gather_bias_lod")]
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_NAME: &CStr = c"VK_AMD_texture_gather_bias_lod";
/// The revision of the
/// [`VK_AMD_texture_gather_bias_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_AMD_texture_gather_bias_lod.html)
/// extension that these bindings were generated from, the C macro
/// `VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION`.
#[doc(alias = "VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION")]
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ANDROID_external_format_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_format_resolve.html),
/// the C macro `VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_ANDROID_external_memory_android_hardware_buffer`](ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_NAME).
#[doc(alias = "VK_ANDROID_external_format_resolve")]
#[doc(alias = "VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_EXTENSION_NAME")]
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_FORMAT_RESOLVE_NAME: &CStr = c"VK_ANDROID_external_format_resolve";
/// The revision of the
/// [`VK_ANDROID_external_format_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_format_resolve.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION`.
#[doc(alias = "VK_ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION")]
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ANDROID_external_memory_android_hardware_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_memory_android_hardware_buffer.html),
/// the C macro `VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME),
/// [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) and
/// [`VK_KHR_dedicated_allocation`](KHR_DEDICATED_ALLOCATION_NAME)) or Vulkan 1.1) and
/// [`VK_EXT_queue_family_foreign`](EXT_QUEUE_FAMILY_FOREIGN_NAME).
#[doc(alias = "VK_ANDROID_external_memory_android_hardware_buffer")]
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_EXTENSION_NAME")]
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_NAME: &CStr =
    c"VK_ANDROID_external_memory_android_hardware_buffer";
/// The revision of the
/// [`VK_ANDROID_external_memory_android_hardware_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ANDROID_external_memory_android_hardware_buffer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION`.
#[doc(alias = "VK_ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION")]
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;

/// The name of the device extension
/// [`VK_ARM_data_graph`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph.html),
/// the C macro `VK_ARM_DATA_GRAPH_EXTENSION_NAME`.
///
/// Requires Vulkan 1.3, ([`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME)) and
/// [`VK_KHR_deferred_host_operations`](KHR_DEFERRED_HOST_OPERATIONS_NAME).
#[doc(alias = "VK_ARM_data_graph")]
#[doc(alias = "VK_ARM_DATA_GRAPH_EXTENSION_NAME")]
pub const ARM_DATA_GRAPH_NAME: &CStr = c"VK_ARM_data_graph";
/// The revision of the
/// [`VK_ARM_data_graph`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_DATA_GRAPH_SPEC_VERSION`.
#[doc(alias = "VK_ARM_DATA_GRAPH_SPEC_VERSION")]
pub const ARM_DATA_GRAPH_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_data_graph_instruction_set_tosa`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_instruction_set_tosa.html),
/// the C macro `VK_ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_EXTENSION_NAME`.
///
/// Requires [`VK_ARM_data_graph`](ARM_DATA_GRAPH_NAME).
#[doc(alias = "VK_ARM_data_graph_instruction_set_tosa")]
#[doc(alias = "VK_ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_EXTENSION_NAME")]
pub const ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_NAME: &CStr =
    c"VK_ARM_data_graph_instruction_set_tosa";
/// The revision of the
/// [`VK_ARM_data_graph_instruction_set_tosa`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_instruction_set_tosa.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_SPEC_VERSION`.
#[doc(alias = "VK_ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_SPEC_VERSION")]
pub const ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_data_graph_neural_accelerator_statistics`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_neural_accelerator_statistics.html),
/// the C macro `VK_ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_ARM_data_graph_neural_accelerator_statistics")]
#[doc(alias = "VK_ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_EXTENSION_NAME")]
pub const ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_NAME: &CStr =
    c"VK_ARM_data_graph_neural_accelerator_statistics";
/// The revision of the
/// [`VK_ARM_data_graph_neural_accelerator_statistics`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_neural_accelerator_statistics.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_SPEC_VERSION")]
pub const ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_data_graph_optical_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_optical_flow.html),
/// the C macro `VK_ARM_DATA_GRAPH_OPTICAL_FLOW_EXTENSION_NAME`.
///
/// Requires [`VK_ARM_data_graph`](ARM_DATA_GRAPH_NAME).
#[doc(alias = "VK_ARM_data_graph_optical_flow")]
#[doc(alias = "VK_ARM_DATA_GRAPH_OPTICAL_FLOW_EXTENSION_NAME")]
pub const ARM_DATA_GRAPH_OPTICAL_FLOW_NAME: &CStr = c"VK_ARM_data_graph_optical_flow";
/// The revision of the
/// [`VK_ARM_data_graph_optical_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_data_graph_optical_flow.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_DATA_GRAPH_OPTICAL_FLOW_SPEC_VERSION`.
#[doc(alias = "VK_ARM_DATA_GRAPH_OPTICAL_FLOW_SPEC_VERSION")]
pub const ARM_DATA_GRAPH_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_format_pack`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_format_pack.html),
/// the C macro `VK_ARM_FORMAT_PACK_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_ARM_format_pack")]
#[doc(alias = "VK_ARM_FORMAT_PACK_EXTENSION_NAME")]
pub const ARM_FORMAT_PACK_NAME: &CStr = c"VK_ARM_format_pack";
/// The revision of the
/// [`VK_ARM_format_pack`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_format_pack.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_FORMAT_PACK_SPEC_VERSION`.
#[doc(alias = "VK_ARM_FORMAT_PACK_SPEC_VERSION")]
pub const ARM_FORMAT_PACK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_performance_counters_by_region`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_performance_counters_by_region.html),
/// the C macro `VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_ARM_performance_counters_by_region")]
#[doc(alias = "VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_EXTENSION_NAME")]
pub const ARM_PERFORMANCE_COUNTERS_BY_REGION_NAME: &CStr = c"VK_ARM_performance_counters_by_region";
/// The revision of the
/// [`VK_ARM_performance_counters_by_region`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_performance_counters_by_region.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION`.
#[doc(alias = "VK_ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION")]
pub const ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_pipeline_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_pipeline_opacity_micromap.html),
/// the C macro `VK_ARM_PIPELINE_OPACITY_MICROMAP_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_opacity_micromap`](EXT_OPACITY_MICROMAP_NAME).
#[doc(alias = "VK_ARM_pipeline_opacity_micromap")]
#[doc(alias = "VK_ARM_PIPELINE_OPACITY_MICROMAP_EXTENSION_NAME")]
pub const ARM_PIPELINE_OPACITY_MICROMAP_NAME: &CStr = c"VK_ARM_pipeline_opacity_micromap";
/// The revision of the
/// [`VK_ARM_pipeline_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_pipeline_opacity_micromap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION`.
#[doc(alias = "VK_ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION")]
pub const ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_rasterization_order_attachment_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_rasterization_order_attachment_access.html),
/// the C macro `VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to
/// [`VK_EXT_rasterization_order_attachment_access`](EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME).
#[doc(alias = "VK_ARM_rasterization_order_attachment_access")]
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME: &CStr =
    c"VK_ARM_rasterization_order_attachment_access";
/// The revision of the
/// [`VK_ARM_rasterization_order_attachment_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_rasterization_order_attachment_access.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_render_pass_striped`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_render_pass_striped.html),
/// the C macro `VK_ARM_RENDER_PASS_STRIPED_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_ARM_render_pass_striped")]
#[doc(alias = "VK_ARM_RENDER_PASS_STRIPED_EXTENSION_NAME")]
pub const ARM_RENDER_PASS_STRIPED_NAME: &CStr = c"VK_ARM_render_pass_striped";
/// The revision of the
/// [`VK_ARM_render_pass_striped`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_render_pass_striped.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_RENDER_PASS_STRIPED_SPEC_VERSION`.
#[doc(alias = "VK_ARM_RENDER_PASS_STRIPED_SPEC_VERSION")]
pub const ARM_RENDER_PASS_STRIPED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_scheduling_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_scheduling_controls.html),
/// the C macro `VK_ARM_SCHEDULING_CONTROLS_EXTENSION_NAME`.
///
/// Requires [`VK_ARM_shader_core_builtins`](ARM_SHADER_CORE_BUILTINS_NAME).
#[doc(alias = "VK_ARM_scheduling_controls")]
#[doc(alias = "VK_ARM_SCHEDULING_CONTROLS_EXTENSION_NAME")]
pub const ARM_SCHEDULING_CONTROLS_NAME: &CStr = c"VK_ARM_scheduling_controls";
/// The revision of the
/// [`VK_ARM_scheduling_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_scheduling_controls.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_SCHEDULING_CONTROLS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_SCHEDULING_CONTROLS_SPEC_VERSION")]
pub const ARM_SCHEDULING_CONTROLS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_ARM_shader_core_builtins`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_builtins.html),
/// the C macro `VK_ARM_SHADER_CORE_BUILTINS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_ARM_shader_core_builtins")]
#[doc(alias = "VK_ARM_SHADER_CORE_BUILTINS_EXTENSION_NAME")]
pub const ARM_SHADER_CORE_BUILTINS_NAME: &CStr = c"VK_ARM_shader_core_builtins";
/// The revision of the
/// [`VK_ARM_shader_core_builtins`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_builtins.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_SHADER_CORE_BUILTINS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_SHADER_CORE_BUILTINS_SPEC_VERSION")]
pub const ARM_SHADER_CORE_BUILTINS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_ARM_shader_core_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_properties.html),
/// the C macro `VK_ARM_SHADER_CORE_PROPERTIES_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_ARM_shader_core_properties")]
#[doc(alias = "VK_ARM_SHADER_CORE_PROPERTIES_EXTENSION_NAME")]
pub const ARM_SHADER_CORE_PROPERTIES_NAME: &CStr = c"VK_ARM_shader_core_properties";
/// The revision of the
/// [`VK_ARM_shader_core_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_core_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION")]
pub const ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_shader_instrumentation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_instrumentation.html),
/// the C macro `VK_ARM_SHADER_INSTRUMENTATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_ARM_shader_instrumentation")]
#[doc(alias = "VK_ARM_SHADER_INSTRUMENTATION_EXTENSION_NAME")]
pub const ARM_SHADER_INSTRUMENTATION_NAME: &CStr = c"VK_ARM_shader_instrumentation";
/// The revision of the
/// [`VK_ARM_shader_instrumentation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_shader_instrumentation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_SHADER_INSTRUMENTATION_SPEC_VERSION`.
#[doc(alias = "VK_ARM_SHADER_INSTRUMENTATION_SPEC_VERSION")]
pub const ARM_SHADER_INSTRUMENTATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_tensor_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_tensor_controls.html),
/// the C macro `VK_ARM_TENSOR_CONTROLS_EXTENSION_NAME`.
///
/// Requires [`VK_ARM_tensors`](ARM_TENSORS_NAME).
#[doc(alias = "VK_ARM_tensor_controls")]
#[doc(alias = "VK_ARM_TENSOR_CONTROLS_EXTENSION_NAME")]
pub const ARM_TENSOR_CONTROLS_NAME: &CStr = c"VK_ARM_tensor_controls";
/// The revision of the
/// [`VK_ARM_tensor_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_tensor_controls.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_TENSOR_CONTROLS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_TENSOR_CONTROLS_SPEC_VERSION")]
pub const ARM_TENSOR_CONTROLS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_ARM_tensors`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_tensors.html),
/// the C macro `VK_ARM_TENSORS_EXTENSION_NAME`.
///
/// Requires Vulkan 1.3.
#[doc(alias = "VK_ARM_tensors")]
#[doc(alias = "VK_ARM_TENSORS_EXTENSION_NAME")]
pub const ARM_TENSORS_NAME: &CStr = c"VK_ARM_tensors";
/// The revision of the
/// [`VK_ARM_tensors`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_ARM_tensors.html)
/// extension that these bindings were generated from, the C macro
/// `VK_ARM_TENSORS_SPEC_VERSION`.
#[doc(alias = "VK_ARM_TENSORS_SPEC_VERSION")]
pub const ARM_TENSORS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_4444_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_4444_formats.html),
/// the C macro `VK_EXT_4444_FORMATS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_4444_formats")]
#[doc(alias = "VK_EXT_4444_FORMATS_EXTENSION_NAME")]
pub const EXT_4444_FORMATS_NAME: &CStr = c"VK_EXT_4444_formats";
/// The revision of the
/// [`VK_EXT_4444_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_4444_formats.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_4444_FORMATS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_4444_FORMATS_SPEC_VERSION")]
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_acquire_drm_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_drm_display.html),
/// the C macro `VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_direct_mode_display`](EXT_DIRECT_MODE_DISPLAY_NAME).
#[doc(alias = "VK_EXT_acquire_drm_display")]
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_DRM_DISPLAY_NAME: &CStr = c"VK_EXT_acquire_drm_display";
/// The revision of the
/// [`VK_EXT_acquire_drm_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_drm_display.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_acquire_xlib_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_xlib_display.html),
/// the C macro `VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_direct_mode_display`](EXT_DIRECT_MODE_DISPLAY_NAME).
#[doc(alias = "VK_EXT_acquire_xlib_display")]
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
#[cfg(feature = "xlib-xrandr")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_NAME: &CStr = c"VK_EXT_acquire_xlib_display";
/// The revision of the
/// [`VK_EXT_acquire_xlib_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_acquire_xlib_display.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
#[cfg(feature = "xlib-xrandr")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_astc_decode_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_astc_decode_mode.html),
/// the C macro `VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_astc_decode_mode")]
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME")]
pub const EXT_ASTC_DECODE_MODE_NAME: &CStr = c"VK_EXT_astc_decode_mode";
/// The revision of the
/// [`VK_EXT_astc_decode_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_astc_decode_mode.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION")]
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_attachment_feedback_loop_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_dynamic_state.html),
/// the C macro `VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and
/// [`VK_EXT_attachment_feedback_loop_layout`](EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_NAME).
#[doc(alias = "VK_EXT_attachment_feedback_loop_dynamic_state")]
#[doc(alias = "VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_NAME: &CStr =
    c"VK_EXT_attachment_feedback_loop_dynamic_state";
/// The revision of the
/// [`VK_EXT_attachment_feedback_loop_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_dynamic_state.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_attachment_feedback_loop_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_layout.html),
/// the C macro `VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_attachment_feedback_loop_layout")]
#[doc(alias = "VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_EXTENSION_NAME")]
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_NAME: &CStr =
    c"VK_EXT_attachment_feedback_loop_layout";
/// The revision of the
/// [`VK_EXT_attachment_feedback_loop_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_attachment_feedback_loop_layout.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION")]
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_blend_operation_advanced`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_blend_operation_advanced.html),
/// the C macro `VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_blend_operation_advanced")]
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME")]
pub const EXT_BLEND_OPERATION_ADVANCED_NAME: &CStr = c"VK_EXT_blend_operation_advanced";
/// The revision of the
/// [`VK_EXT_blend_operation_advanced`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_blend_operation_advanced.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION")]
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_border_color_swizzle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_border_color_swizzle.html),
/// the C macro `VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_custom_border_color`](EXT_CUSTOM_BORDER_COLOR_NAME).
#[doc(alias = "VK_EXT_border_color_swizzle")]
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_NAME: &CStr = c"VK_EXT_border_color_swizzle";
/// The revision of the
/// [`VK_EXT_border_color_swizzle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_border_color_swizzle.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_buffer_device_address`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_buffer_device_address.html),
/// the C macro `VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Deprecated by [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME).
#[doc(alias = "VK_EXT_buffer_device_address")]
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const EXT_BUFFER_DEVICE_ADDRESS_NAME: &CStr = c"VK_EXT_buffer_device_address";
/// The revision of the
/// [`VK_EXT_buffer_device_address`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_buffer_device_address.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_calibrated_timestamps`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_calibrated_timestamps.html),
/// the C macro `VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_calibrated_timestamps`](KHR_CALIBRATED_TIMESTAMPS_NAME).
#[doc(alias = "VK_EXT_calibrated_timestamps")]
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_NAME: &CStr = c"VK_EXT_calibrated_timestamps";
/// The revision of the
/// [`VK_EXT_calibrated_timestamps`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_calibrated_timestamps.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_color_write_enable`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_color_write_enable.html),
/// the C macro `VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_color_write_enable")]
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME")]
pub const EXT_COLOR_WRITE_ENABLE_NAME: &CStr = c"VK_EXT_color_write_enable";
/// The revision of the
/// [`VK_EXT_color_write_enable`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_color_write_enable.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION")]
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_conditional_rendering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conditional_rendering.html),
/// the C macro `VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_conditional_rendering")]
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME")]
pub const EXT_CONDITIONAL_RENDERING_NAME: &CStr = c"VK_EXT_conditional_rendering";
/// The revision of the
/// [`VK_EXT_conditional_rendering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conditional_rendering.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION`.
#[doc(alias = "VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION")]
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_conservative_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conservative_rasterization.html),
/// the C macro `VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_conservative_rasterization")]
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_CONSERVATIVE_RASTERIZATION_NAME: &CStr = c"VK_EXT_conservative_rasterization";
/// The revision of the
/// [`VK_EXT_conservative_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_conservative_rasterization.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION`.
#[doc(alias = "VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_cooperative_matrix_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_cooperative_matrix_maintenance1.html),
/// the C macro `VK_EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_cooperative_matrix`](KHR_COOPERATIVE_MATRIX_NAME).
#[doc(alias = "VK_EXT_cooperative_matrix_maintenance1")]
#[doc(alias = "VK_EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_EXTENSION_NAME")]
pub const EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_NAME: &CStr =
    c"VK_EXT_cooperative_matrix_maintenance1";
/// The revision of the
/// [`VK_EXT_cooperative_matrix_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_cooperative_matrix_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_SPEC_VERSION")]
pub const EXT_COOPERATIVE_MATRIX_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_custom_border_color`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_border_color.html),
/// the C macro `VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_custom_border_color")]
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME")]
pub const EXT_CUSTOM_BORDER_COLOR_NAME: &CStr = c"VK_EXT_custom_border_color";
/// The revision of the
/// [`VK_EXT_custom_border_color`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_border_color.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION`.
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION")]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;

/// The name of the device extension
/// [`VK_EXT_custom_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_resolve.html),
/// the C macro `VK_EXT_CUSTOM_RESOLVE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_custom_resolve")]
#[doc(alias = "VK_EXT_CUSTOM_RESOLVE_EXTENSION_NAME")]
pub const EXT_CUSTOM_RESOLVE_NAME: &CStr = c"VK_EXT_custom_resolve";
/// The revision of the
/// [`VK_EXT_custom_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_custom_resolve.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_CUSTOM_RESOLVE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_CUSTOM_RESOLVE_SPEC_VERSION")]
pub const EXT_CUSTOM_RESOLVE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_debug_marker`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_marker.html),
/// the C macro `VK_EXT_DEBUG_MARKER_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_debug_report`](EXT_DEBUG_REPORT_NAME). Promoted to
/// [`VK_EXT_debug_utils`](EXT_DEBUG_UTILS_NAME).
#[doc(alias = "VK_EXT_debug_marker")]
#[doc(alias = "VK_EXT_DEBUG_MARKER_EXTENSION_NAME")]
pub const EXT_DEBUG_MARKER_NAME: &CStr = c"VK_EXT_debug_marker";
/// The revision of the
/// [`VK_EXT_debug_marker`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_marker.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEBUG_MARKER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEBUG_MARKER_SPEC_VERSION")]
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;

/// The name of the instance extension
/// [`VK_EXT_debug_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_report.html),
/// the C macro `VK_EXT_DEBUG_REPORT_EXTENSION_NAME`.
///
/// Deprecated by [`VK_EXT_debug_utils`](EXT_DEBUG_UTILS_NAME).
#[doc(alias = "VK_EXT_debug_report")]
#[doc(alias = "VK_EXT_DEBUG_REPORT_EXTENSION_NAME")]
pub const EXT_DEBUG_REPORT_NAME: &CStr = c"VK_EXT_debug_report";
/// The revision of the
/// [`VK_EXT_debug_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_report.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEBUG_REPORT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEBUG_REPORT_SPEC_VERSION")]
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;

/// The name of the instance extension
/// [`VK_EXT_debug_utils`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_utils.html),
/// the C macro `VK_EXT_DEBUG_UTILS_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_debug_utils")]
#[doc(alias = "VK_EXT_DEBUG_UTILS_EXTENSION_NAME")]
pub const EXT_DEBUG_UTILS_NAME: &CStr = c"VK_EXT_debug_utils";
/// The revision of the
/// [`VK_EXT_debug_utils`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_utils.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEBUG_UTILS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEBUG_UTILS_SPEC_VERSION")]
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_depth_bias_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_bias_control.html),
/// the C macro `VK_EXT_DEPTH_BIAS_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_depth_bias_control")]
#[doc(alias = "VK_EXT_DEPTH_BIAS_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_BIAS_CONTROL_NAME: &CStr = c"VK_EXT_depth_bias_control";
/// The revision of the
/// [`VK_EXT_depth_bias_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_bias_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_depth_clamp_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_control.html),
/// the C macro `VK_EXT_DEPTH_CLAMP_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_depth_clamp_control")]
#[doc(alias = "VK_EXT_DEPTH_CLAMP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLAMP_CONTROL_NAME: &CStr = c"VK_EXT_depth_clamp_control";
/// The revision of the
/// [`VK_EXT_depth_clamp_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_depth_clamp_zero_one`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_zero_one.html),
/// the C macro `VK_EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_depth_clamp_zero_one`](KHR_DEPTH_CLAMP_ZERO_ONE_NAME).
#[doc(alias = "VK_EXT_depth_clamp_zero_one")]
#[doc(alias = "VK_EXT_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLAMP_ZERO_ONE_NAME: &CStr = c"VK_EXT_depth_clamp_zero_one";
/// The revision of the
/// [`VK_EXT_depth_clamp_zero_one`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clamp_zero_one.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION")]
pub const EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_depth_clip_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_control.html),
/// the C macro `VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_depth_clip_control")]
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_CONTROL_NAME: &CStr = c"VK_EXT_depth_clip_control";
/// The revision of the
/// [`VK_EXT_depth_clip_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_depth_clip_enable`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_enable.html),
/// the C macro `VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_depth_clip_enable")]
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_NAME: &CStr = c"VK_EXT_depth_clip_enable";
/// The revision of the
/// [`VK_EXT_depth_clip_enable`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_clip_enable.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_depth_range_unrestricted`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_range_unrestricted.html),
/// the C macro `VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_depth_range_unrestricted")]
#[doc(alias = "VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME")]
pub const EXT_DEPTH_RANGE_UNRESTRICTED_NAME: &CStr = c"VK_EXT_depth_range_unrestricted";
/// The revision of the
/// [`VK_EXT_depth_range_unrestricted`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_depth_range_unrestricted.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION")]
pub const EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_descriptor_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_buffer.html),
/// the C macro `VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME`.
///
/// Requires
/// (((([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME) and
/// [`VK_EXT_descriptor_indexing`](EXT_DESCRIPTOR_INDEXING_NAME)) or Vulkan 1.2) and
/// [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME)) or Vulkan 1.3. Deprecated by
/// [`VK_EXT_descriptor_heap`](EXT_DESCRIPTOR_HEAP_NAME).
#[doc(alias = "VK_EXT_descriptor_buffer")]
#[doc(alias = "VK_EXT_DESCRIPTOR_BUFFER_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_BUFFER_NAME: &CStr = c"VK_EXT_descriptor_buffer";
/// The revision of the
/// [`VK_EXT_descriptor_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_buffer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DESCRIPTOR_BUFFER_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_descriptor_heap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_heap.html),
/// the C macro `VK_EXT_DESCRIPTOR_HEAP_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME)) and
/// ([`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME) or Vulkan 1.2)) or Vulkan
/// 1.4.
#[doc(alias = "VK_EXT_descriptor_heap")]
#[doc(alias = "VK_EXT_DESCRIPTOR_HEAP_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_HEAP_NAME: &CStr = c"VK_EXT_descriptor_heap";
/// The revision of the
/// [`VK_EXT_descriptor_heap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_heap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DESCRIPTOR_HEAP_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DESCRIPTOR_HEAP_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_HEAP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_descriptor_indexing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_indexing.html),
/// the C macro `VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_maintenance3`](KHR_MAINTENANCE_3_NAME)) or Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_descriptor_indexing")]
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME")]
pub const EXT_DESCRIPTOR_INDEXING_NAME: &CStr = c"VK_EXT_descriptor_indexing";
/// The revision of the
/// [`VK_EXT_descriptor_indexing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_descriptor_indexing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION")]
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_device_address_binding_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_address_binding_report.html),
/// the C macro `VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_EXT_debug_utils`](EXT_DEBUG_UTILS_NAME).
#[doc(alias = "VK_EXT_device_address_binding_report")]
#[doc(alias = "VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_EXTENSION_NAME")]
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_NAME: &CStr = c"VK_EXT_device_address_binding_report";
/// The revision of the
/// [`VK_EXT_device_address_binding_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_address_binding_report.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION")]
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_device_fault`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_fault.html),
/// the C macro `VK_EXT_DEVICE_FAULT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_device_fault`](KHR_DEVICE_FAULT_NAME).
#[doc(alias = "VK_EXT_device_fault")]
#[doc(alias = "VK_EXT_DEVICE_FAULT_EXTENSION_NAME")]
pub const EXT_DEVICE_FAULT_NAME: &CStr = c"VK_EXT_device_fault";
/// The revision of the
/// [`VK_EXT_device_fault`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_fault.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEVICE_FAULT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEVICE_FAULT_SPEC_VERSION")]
pub const EXT_DEVICE_FAULT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_device_generated_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_generated_commands.html),
/// the C macro `VK_EXT_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME) or Vulkan 1.2)
/// and ([`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME))) or Vulkan 1.3.
#[doc(alias = "VK_EXT_device_generated_commands")]
#[doc(alias = "VK_EXT_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const EXT_DEVICE_GENERATED_COMMANDS_NAME: &CStr = c"VK_EXT_device_generated_commands";
/// The revision of the
/// [`VK_EXT_device_generated_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_generated_commands.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_device_memory_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_memory_report.html),
/// the C macro `VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_device_memory_report")]
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME")]
pub const EXT_DEVICE_MEMORY_REPORT_NAME: &CStr = c"VK_EXT_device_memory_report";
/// The revision of the
/// [`VK_EXT_device_memory_report`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_device_memory_report.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION")]
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_EXT_direct_mode_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_direct_mode_display.html),
/// the C macro `VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_display`](KHR_DISPLAY_NAME).
#[doc(alias = "VK_EXT_direct_mode_display")]
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_NAME: &CStr = c"VK_EXT_direct_mode_display";
/// The revision of the
/// [`VK_EXT_direct_mode_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_direct_mode_display.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_directfb_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_directfb_surface.html),
/// the C macro `VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_EXT_directfb_surface")]
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "directfb")]
pub const EXT_DIRECTFB_SURFACE_NAME: &CStr = c"VK_EXT_directfb_surface";
/// The revision of the
/// [`VK_EXT_directfb_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_directfb_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DIRECTFB_SURFACE_SPEC_VERSION")]
#[cfg(feature = "directfb")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_discard_rectangles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_discard_rectangles.html),
/// the C macro `VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_discard_rectangles")]
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME")]
pub const EXT_DISCARD_RECTANGLES_NAME: &CStr = c"VK_EXT_discard_rectangles";
/// The revision of the
/// [`VK_EXT_discard_rectangles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_discard_rectangles.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION")]
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_display_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_control.html),
/// the C macro `VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_display_surface_counter`](EXT_DISPLAY_SURFACE_COUNTER_NAME) and
/// [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_EXT_display_control")]
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME")]
pub const EXT_DISPLAY_CONTROL_NAME: &CStr = c"VK_EXT_display_control";
/// The revision of the
/// [`VK_EXT_display_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DISPLAY_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DISPLAY_CONTROL_SPEC_VERSION")]
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_display_surface_counter`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_surface_counter.html),
/// the C macro `VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_display`](KHR_DISPLAY_NAME).
#[doc(alias = "VK_EXT_display_surface_counter")]
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME")]
pub const EXT_DISPLAY_SURFACE_COUNTER_NAME: &CStr = c"VK_EXT_display_surface_counter";
/// The revision of the
/// [`VK_EXT_display_surface_counter`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_display_surface_counter.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION")]
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_dynamic_rendering_unused_attachments`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_dynamic_rendering_unused_attachments.html),
/// the C macro `VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_dynamic_rendering`](KHR_DYNAMIC_RENDERING_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_EXT_dynamic_rendering_unused_attachments")]
#[doc(alias = "VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_EXTENSION_NAME")]
pub const EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_NAME: &CStr =
    c"VK_EXT_dynamic_rendering_unused_attachments";
/// The revision of the
/// [`VK_EXT_dynamic_rendering_unused_attachments`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_dynamic_rendering_unused_attachments.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION")]
pub const EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_extended_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state.html),
/// the C macro `VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_extended_dynamic_state")]
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_NAME: &CStr = c"VK_EXT_extended_dynamic_state";
/// The revision of the
/// [`VK_EXT_extended_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_extended_dynamic_state2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state2.html),
/// the C macro `VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_extended_dynamic_state2")]
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_NAME: &CStr = c"VK_EXT_extended_dynamic_state2";
/// The revision of the
/// [`VK_EXT_extended_dynamic_state2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_extended_dynamic_state3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state3.html),
/// the C macro `VK_EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_extended_dynamic_state3")]
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_3_EXTENSION_NAME")]
pub const EXT_EXTENDED_DYNAMIC_STATE_3_NAME: &CStr = c"VK_EXT_extended_dynamic_state3";
/// The revision of the
/// [`VK_EXT_extended_dynamic_state3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_extended_dynamic_state3.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION")]
pub const EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_external_memory_acquire_unmodified`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_acquire_unmodified.html),
/// the C macro `VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_EXT_external_memory_acquire_unmodified")]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_NAME: &CStr =
    c"VK_EXT_external_memory_acquire_unmodified";
/// The revision of the
/// [`VK_EXT_external_memory_acquire_unmodified`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_acquire_unmodified.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_external_memory_dma_buf`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_dma_buf.html),
/// the C macro `VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory_fd`](KHR_EXTERNAL_MEMORY_FD_NAME).
#[doc(alias = "VK_EXT_external_memory_dma_buf")]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_NAME: &CStr = c"VK_EXT_external_memory_dma_buf";
/// The revision of the
/// [`VK_EXT_external_memory_dma_buf`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_dma_buf.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_external_memory_host`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_host.html),
/// the C macro `VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_EXT_external_memory_host")]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME")]
pub const EXT_EXTERNAL_MEMORY_HOST_NAME: &CStr = c"VK_EXT_external_memory_host";
/// The revision of the
/// [`VK_EXT_external_memory_host`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_host.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION")]
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_external_memory_metal`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_metal.html),
/// the C macro `VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_EXT_external_memory_metal")]
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_METAL_EXTENSION_NAME")]
#[cfg(feature = "metal")]
pub const EXT_EXTERNAL_MEMORY_METAL_NAME: &CStr = c"VK_EXT_external_memory_metal";
/// The revision of the
/// [`VK_EXT_external_memory_metal`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_external_memory_metal.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION")]
#[cfg(feature = "metal")]
pub const EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_filter_cubic`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_filter_cubic.html),
/// the C macro `VK_EXT_FILTER_CUBIC_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_filter_cubic")]
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_NAME: &CStr = c"VK_EXT_filter_cubic";
/// The revision of the
/// [`VK_EXT_filter_cubic`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_filter_cubic.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FILTER_CUBIC_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_fragment_density_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map.html),
/// the C macro `VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_fragment_density_map")]
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_NAME: &CStr = c"VK_EXT_fragment_density_map";
/// The revision of the
/// [`VK_EXT_fragment_density_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_fragment_density_map2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map2.html),
/// the C macro `VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_fragment_density_map`](EXT_FRAGMENT_DENSITY_MAP_NAME).
#[doc(alias = "VK_EXT_fragment_density_map2")]
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_NAME: &CStr = c"VK_EXT_fragment_density_map2";
/// The revision of the
/// [`VK_EXT_fragment_density_map2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_fragment_density_map_offset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map_offset.html),
/// the C macro `VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_EXT_fragment_density_map`](EXT_FRAGMENT_DENSITY_MAP_NAME),
/// ([`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME) or Vulkan 1.2) and (Vulkan 1.3
/// or [`VK_KHR_dynamic_rendering`](KHR_DYNAMIC_RENDERING_NAME)).
#[doc(alias = "VK_EXT_fragment_density_map_offset")]
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const EXT_FRAGMENT_DENSITY_MAP_OFFSET_NAME: &CStr = c"VK_EXT_fragment_density_map_offset";
/// The revision of the
/// [`VK_EXT_fragment_density_map_offset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_density_map_offset.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_fragment_shader_interlock`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_shader_interlock.html),
/// the C macro `VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_fragment_shader_interlock")]
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_NAME: &CStr = c"VK_EXT_fragment_shader_interlock";
/// The revision of the
/// [`VK_EXT_fragment_shader_interlock`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_fragment_shader_interlock.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_frame_boundary`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_frame_boundary.html),
/// the C macro `VK_EXT_FRAME_BOUNDARY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_frame_boundary")]
#[doc(alias = "VK_EXT_FRAME_BOUNDARY_EXTENSION_NAME")]
pub const EXT_FRAME_BOUNDARY_NAME: &CStr = c"VK_EXT_frame_boundary";
/// The revision of the
/// [`VK_EXT_frame_boundary`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_frame_boundary.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FRAME_BOUNDARY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FRAME_BOUNDARY_SPEC_VERSION")]
pub const EXT_FRAME_BOUNDARY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_full_screen_exclusive`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_full_screen_exclusive.html),
/// the C macro `VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_surface`](KHR_SURFACE_NAME),
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME) and
/// [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_EXT_full_screen_exclusive")]
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_NAME: &CStr = c"VK_EXT_full_screen_exclusive";
/// The revision of the
/// [`VK_EXT_full_screen_exclusive`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_full_screen_exclusive.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_EXT_global_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_global_priority.html),
/// the C macro `VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME`.
///
/// Promoted to [`VK_KHR_global_priority`](KHR_GLOBAL_PRIORITY_NAME).
#[doc(alias = "VK_EXT_global_priority")]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_NAME: &CStr = c"VK_EXT_global_priority";
/// The revision of the
/// [`VK_EXT_global_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_global_priority.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_global_priority_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_global_priority_query.html),
/// the C macro `VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_global_priority`](EXT_GLOBAL_PRIORITY_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1). Promoted to [`VK_KHR_global_priority`](KHR_GLOBAL_PRIORITY_NAME).
#[doc(alias = "VK_EXT_global_priority_query")]
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_QUERY_NAME: &CStr = c"VK_EXT_global_priority_query";
/// The revision of the
/// [`VK_EXT_global_priority_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_global_priority_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_graphics_pipeline_library`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_graphics_pipeline_library.html),
/// the C macro `VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_pipeline_library`](KHR_PIPELINE_LIBRARY_NAME).
#[doc(alias = "VK_EXT_graphics_pipeline_library")]
#[doc(alias = "VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_NAME: &CStr = c"VK_EXT_graphics_pipeline_library";
/// The revision of the
/// [`VK_EXT_graphics_pipeline_library`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_graphics_pipeline_library.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_hdr_metadata`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_hdr_metadata.html),
/// the C macro `VK_EXT_HDR_METADATA_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_EXT_hdr_metadata")]
#[doc(alias = "VK_EXT_HDR_METADATA_EXTENSION_NAME")]
pub const EXT_HDR_METADATA_NAME: &CStr = c"VK_EXT_hdr_metadata";
/// The revision of the
/// [`VK_EXT_hdr_metadata`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_hdr_metadata.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_HDR_METADATA_SPEC_VERSION`.
#[doc(alias = "VK_EXT_HDR_METADATA_SPEC_VERSION")]
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 3;

/// The name of the instance extension
/// [`VK_EXT_headless_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_headless_surface.html),
/// the C macro `VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_EXT_headless_surface")]
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME")]
pub const EXT_HEADLESS_SURFACE_NAME: &CStr = c"VK_EXT_headless_surface";
/// The revision of the
/// [`VK_EXT_headless_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_headless_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_HEADLESS_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_HEADLESS_SURFACE_SPEC_VERSION")]
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_host_image_copy`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_image_copy.html),
/// the C macro `VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_copy_commands2`](KHR_COPY_COMMANDS_2_NAME) and
/// [`VK_KHR_format_feature_flags2`](KHR_FORMAT_FEATURE_FLAGS_2_NAME)) or Vulkan 1.3. Promoted
/// to Vulkan 1.4.
#[doc(alias = "VK_EXT_host_image_copy")]
#[doc(alias = "VK_EXT_HOST_IMAGE_COPY_EXTENSION_NAME")]
pub const EXT_HOST_IMAGE_COPY_NAME: &CStr = c"VK_EXT_host_image_copy";
/// The revision of the
/// [`VK_EXT_host_image_copy`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_image_copy.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_HOST_IMAGE_COPY_SPEC_VERSION")]
pub const EXT_HOST_IMAGE_COPY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_host_query_reset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_query_reset.html),
/// the C macro `VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_host_query_reset")]
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME")]
pub const EXT_HOST_QUERY_RESET_NAME: &CStr = c"VK_EXT_host_query_reset";
/// The revision of the
/// [`VK_EXT_host_query_reset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_host_query_reset.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_HOST_QUERY_RESET_SPEC_VERSION`.
#[doc(alias = "VK_EXT_HOST_QUERY_RESET_SPEC_VERSION")]
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_2d_view_of_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_2d_view_of_3d.html),
/// the C macro `VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_maintenance1`](KHR_MAINTENANCE_1_NAME) and
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME)) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_image_2d_view_of_3d")]
#[doc(alias = "VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME")]
pub const EXT_IMAGE_2D_VIEW_OF_3D_NAME: &CStr = c"VK_EXT_image_2d_view_of_3d";
/// The revision of the
/// [`VK_EXT_image_2d_view_of_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_2d_view_of_3d.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION")]
pub const EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_compression_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control.html),
/// the C macro `VK_EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_image_compression_control")]
#[doc(alias = "VK_EXT_IMAGE_COMPRESSION_CONTROL_EXTENSION_NAME")]
pub const EXT_IMAGE_COMPRESSION_CONTROL_NAME: &CStr = c"VK_EXT_image_compression_control";
/// The revision of the
/// [`VK_EXT_image_compression_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION")]
pub const EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_compression_control_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control_swapchain.html),
/// the C macro `VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_image_compression_control`](EXT_IMAGE_COMPRESSION_CONTROL_NAME) and
/// [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_EXT_image_compression_control_swapchain")]
#[doc(alias = "VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_EXTENSION_NAME")]
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_NAME: &CStr =
    c"VK_EXT_image_compression_control_swapchain";
/// The revision of the
/// [`VK_EXT_image_compression_control_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_compression_control_swapchain.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION")]
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_drm_format_modifier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_drm_format_modifier.html),
/// the C macro `VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME`.
///
/// Requires ((([`VK_KHR_bind_memory2`](KHR_BIND_MEMORY_2_NAME),
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME)) or Vulkan 1.1) and
/// [`VK_KHR_image_format_list`](KHR_IMAGE_FORMAT_LIST_NAME)) or Vulkan 1.2.
#[doc(alias = "VK_EXT_image_drm_format_modifier")]
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME: &CStr = c"VK_EXT_image_drm_format_modifier";
/// The revision of the
/// [`VK_EXT_image_drm_format_modifier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_drm_format_modifier.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_image_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_robustness.html),
/// the C macro `VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_image_robustness")]
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_IMAGE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_image_robustness";
/// The revision of the
/// [`VK_EXT_image_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_robustness.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_sliced_view_of_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_sliced_view_of_3d.html),
/// the C macro `VK_EXT_IMAGE_SLICED_VIEW_OF_3D_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_maintenance1`](KHR_MAINTENANCE_1_NAME) and
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME)) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_image_sliced_view_of_3d")]
#[doc(alias = "VK_EXT_IMAGE_SLICED_VIEW_OF_3D_EXTENSION_NAME")]
pub const EXT_IMAGE_SLICED_VIEW_OF_3D_NAME: &CStr = c"VK_EXT_image_sliced_view_of_3d";
/// The revision of the
/// [`VK_EXT_image_sliced_view_of_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_sliced_view_of_3d.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION")]
pub const EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_tiling_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_tiling_control.html),
/// the C macro `VK_EXT_IMAGE_TILING_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_image_tiling_control")]
#[doc(alias = "VK_EXT_IMAGE_TILING_CONTROL_EXTENSION_NAME")]
pub const EXT_IMAGE_TILING_CONTROL_NAME: &CStr = c"VK_EXT_image_tiling_control";
/// The revision of the
/// [`VK_EXT_image_tiling_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_tiling_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_TILING_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_TILING_CONTROL_SPEC_VERSION")]
pub const EXT_IMAGE_TILING_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_image_view_min_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_view_min_lod.html),
/// the C macro `VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_image_view_min_lod")]
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME")]
pub const EXT_IMAGE_VIEW_MIN_LOD_NAME: &CStr = c"VK_EXT_image_view_min_lod";
/// The revision of the
/// [`VK_EXT_image_view_min_lod`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_image_view_min_lod.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION`.
#[doc(alias = "VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION")]
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_index_type_uint8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_index_type_uint8.html),
/// the C macro `VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_index_type_uint8`](KHR_INDEX_TYPE_UINT8_NAME).
#[doc(alias = "VK_EXT_index_type_uint8")]
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const EXT_INDEX_TYPE_UINT8_NAME: &CStr = c"VK_EXT_index_type_uint8";
/// The revision of the
/// [`VK_EXT_index_type_uint8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_index_type_uint8.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION`.
#[doc(alias = "VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_inline_uniform_block`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_inline_uniform_block.html),
/// the C macro `VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_maintenance1`](KHR_MAINTENANCE_1_NAME)) or Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_inline_uniform_block")]
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME")]
pub const EXT_INLINE_UNIFORM_BLOCK_NAME: &CStr = c"VK_EXT_inline_uniform_block";
/// The revision of the
/// [`VK_EXT_inline_uniform_block`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_inline_uniform_block.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION`.
#[doc(alias = "VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION")]
pub const EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_layer_settings`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_layer_settings.html),
/// the C macro `VK_EXT_LAYER_SETTINGS_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_layer_settings")]
#[doc(alias = "VK_EXT_LAYER_SETTINGS_EXTENSION_NAME")]
pub const EXT_LAYER_SETTINGS_NAME: &CStr = c"VK_EXT_layer_settings";
/// The revision of the
/// [`VK_EXT_layer_settings`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_layer_settings.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_LAYER_SETTINGS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_LAYER_SETTINGS_SPEC_VERSION")]
pub const EXT_LAYER_SETTINGS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_legacy_dithering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_dithering.html),
/// the C macro `VK_EXT_LEGACY_DITHERING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_legacy_dithering")]
#[doc(alias = "VK_EXT_LEGACY_DITHERING_EXTENSION_NAME")]
pub const EXT_LEGACY_DITHERING_NAME: &CStr = c"VK_EXT_legacy_dithering";
/// The revision of the
/// [`VK_EXT_legacy_dithering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_dithering.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_LEGACY_DITHERING_SPEC_VERSION`.
#[doc(alias = "VK_EXT_LEGACY_DITHERING_SPEC_VERSION")]
pub const EXT_LEGACY_DITHERING_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_legacy_vertex_attributes`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_vertex_attributes.html),
/// the C macro `VK_EXT_LEGACY_VERTEX_ATTRIBUTES_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_vertex_input_dynamic_state`](EXT_VERTEX_INPUT_DYNAMIC_STATE_NAME).
#[doc(alias = "VK_EXT_legacy_vertex_attributes")]
#[doc(alias = "VK_EXT_LEGACY_VERTEX_ATTRIBUTES_EXTENSION_NAME")]
pub const EXT_LEGACY_VERTEX_ATTRIBUTES_NAME: &CStr = c"VK_EXT_legacy_vertex_attributes";
/// The revision of the
/// [`VK_EXT_legacy_vertex_attributes`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_legacy_vertex_attributes.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION")]
pub const EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_line_rasterization.html),
/// the C macro `VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_line_rasterization`](KHR_LINE_RASTERIZATION_NAME).
#[doc(alias = "VK_EXT_line_rasterization")]
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_LINE_RASTERIZATION_NAME: &CStr = c"VK_EXT_line_rasterization";
/// The revision of the
/// [`VK_EXT_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_line_rasterization.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_LINE_RASTERIZATION_SPEC_VERSION`.
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_load_store_op_none`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_load_store_op_none.html),
/// the C macro `VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME`.
///
/// Promoted to [`VK_KHR_load_store_op_none`](KHR_LOAD_STORE_OP_NONE_NAME).
#[doc(alias = "VK_EXT_load_store_op_none")]
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME")]
pub const EXT_LOAD_STORE_OP_NONE_NAME: &CStr = c"VK_EXT_load_store_op_none";
/// The revision of the
/// [`VK_EXT_load_store_op_none`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_load_store_op_none.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION")]
pub const EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_map_memory_placed`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_map_memory_placed.html),
/// the C macro `VK_EXT_MAP_MEMORY_PLACED_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_map_memory2`](KHR_MAP_MEMORY_2_NAME) or Vulkan 1.4.
#[doc(alias = "VK_EXT_map_memory_placed")]
#[doc(alias = "VK_EXT_MAP_MEMORY_PLACED_EXTENSION_NAME")]
pub const EXT_MAP_MEMORY_PLACED_NAME: &CStr = c"VK_EXT_map_memory_placed";
/// The revision of the
/// [`VK_EXT_map_memory_placed`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_map_memory_placed.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MAP_MEMORY_PLACED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MAP_MEMORY_PLACED_SPEC_VERSION")]
pub const EXT_MAP_MEMORY_PLACED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_memory_budget`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_budget.html),
/// the C macro `VK_EXT_MEMORY_BUDGET_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_memory_budget")]
#[doc(alias = "VK_EXT_MEMORY_BUDGET_EXTENSION_NAME")]
pub const EXT_MEMORY_BUDGET_NAME: &CStr = c"VK_EXT_memory_budget";
/// The revision of the
/// [`VK_EXT_memory_budget`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_budget.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MEMORY_BUDGET_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MEMORY_BUDGET_SPEC_VERSION")]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_memory_decompression`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_decompression.html),
/// the C macro `VK_EXT_MEMORY_DECOMPRESSION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME).
#[doc(alias = "VK_EXT_memory_decompression")]
#[doc(alias = "VK_EXT_MEMORY_DECOMPRESSION_EXTENSION_NAME")]
pub const EXT_MEMORY_DECOMPRESSION_NAME: &CStr = c"VK_EXT_memory_decompression";
/// The revision of the
/// [`VK_EXT_memory_decompression`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_decompression.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MEMORY_DECOMPRESSION_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MEMORY_DECOMPRESSION_SPEC_VERSION")]
pub const EXT_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_memory_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_priority.html),
/// the C macro `VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_memory_priority")]
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME")]
pub const EXT_MEMORY_PRIORITY_NAME: &CStr = c"VK_EXT_memory_priority";
/// The revision of the
/// [`VK_EXT_memory_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_memory_priority.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MEMORY_PRIORITY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_SPEC_VERSION")]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_mesh_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mesh_shader.html),
/// the C macro `VK_EXT_MESH_SHADER_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_spirv_1_4`](KHR_SPIRV_1_4_NAME) or Vulkan 1.2.
#[doc(alias = "VK_EXT_mesh_shader")]
#[doc(alias = "VK_EXT_MESH_SHADER_EXTENSION_NAME")]
pub const EXT_MESH_SHADER_NAME: &CStr = c"VK_EXT_mesh_shader";
/// The revision of the
/// [`VK_EXT_mesh_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mesh_shader.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MESH_SHADER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MESH_SHADER_SPEC_VERSION")]
pub const EXT_MESH_SHADER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_metal_objects`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_metal_objects.html),
/// the C macro `VK_EXT_METAL_OBJECTS_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_metal_objects")]
#[doc(alias = "VK_EXT_METAL_OBJECTS_EXTENSION_NAME")]
#[cfg(feature = "metal")]
pub const EXT_METAL_OBJECTS_NAME: &CStr = c"VK_EXT_metal_objects";
/// The revision of the
/// [`VK_EXT_metal_objects`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_metal_objects.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_METAL_OBJECTS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_METAL_OBJECTS_SPEC_VERSION")]
#[cfg(feature = "metal")]
pub const EXT_METAL_OBJECTS_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_EXT_metal_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_metal_surface.html),
/// the C macro `VK_EXT_METAL_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_EXT_metal_surface")]
#[doc(alias = "VK_EXT_METAL_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "metal")]
pub const EXT_METAL_SURFACE_NAME: &CStr = c"VK_EXT_metal_surface";
/// The revision of the
/// [`VK_EXT_metal_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_metal_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_METAL_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_METAL_SURFACE_SPEC_VERSION")]
#[cfg(feature = "metal")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_multi_draw`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multi_draw.html),
/// the C macro `VK_EXT_MULTI_DRAW_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_multi_draw")]
#[doc(alias = "VK_EXT_MULTI_DRAW_EXTENSION_NAME")]
pub const EXT_MULTI_DRAW_NAME: &CStr = c"VK_EXT_multi_draw";
/// The revision of the
/// [`VK_EXT_multi_draw`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multi_draw.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MULTI_DRAW_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MULTI_DRAW_SPEC_VERSION")]
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_multisampled_render_to_single_sampled`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multisampled_render_to_single_sampled.html),
/// the C macro `VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME) and
/// [`VK_KHR_depth_stencil_resolve`](KHR_DEPTH_STENCIL_RESOLVE_NAME)) or Vulkan 1.2.
#[doc(alias = "VK_EXT_multisampled_render_to_single_sampled")]
#[doc(alias = "VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXTENSION_NAME")]
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_NAME: &CStr =
    c"VK_EXT_multisampled_render_to_single_sampled";
/// The revision of the
/// [`VK_EXT_multisampled_render_to_single_sampled`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multisampled_render_to_single_sampled.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION")]
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_multisampled_render_to_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multisampled_render_to_swapchain.html),
/// the C macro `VK_EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// [`VK_EXT_multisampled_render_to_single_sampled`](EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_NAME).
#[doc(alias = "VK_EXT_multisampled_render_to_swapchain")]
#[doc(alias = "VK_EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_EXTENSION_NAME")]
pub const EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_NAME: &CStr =
    c"VK_EXT_multisampled_render_to_swapchain";
/// The revision of the
/// [`VK_EXT_multisampled_render_to_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_multisampled_render_to_swapchain.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_SPEC_VERSION")]
pub const EXT_MULTISAMPLED_RENDER_TO_SWAPCHAIN_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_mutable_descriptor_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mutable_descriptor_type.html),
/// the C macro `VK_EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_maintenance3`](KHR_MAINTENANCE_3_NAME) or Vulkan 1.1.
#[doc(alias = "VK_EXT_mutable_descriptor_type")]
#[doc(alias = "VK_EXT_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const EXT_MUTABLE_DESCRIPTOR_TYPE_NAME: &CStr = c"VK_EXT_mutable_descriptor_type";
/// The revision of the
/// [`VK_EXT_mutable_descriptor_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_mutable_descriptor_type.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_nested_command_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_nested_command_buffer.html),
/// the C macro `VK_EXT_NESTED_COMMAND_BUFFER_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_nested_command_buffer")]
#[doc(alias = "VK_EXT_NESTED_COMMAND_BUFFER_EXTENSION_NAME")]
pub const EXT_NESTED_COMMAND_BUFFER_NAME: &CStr = c"VK_EXT_nested_command_buffer";
/// The revision of the
/// [`VK_EXT_nested_command_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_nested_command_buffer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION")]
pub const EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_non_seamless_cube_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_non_seamless_cube_map.html),
/// the C macro `VK_EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_non_seamless_cube_map")]
#[doc(alias = "VK_EXT_NON_SEAMLESS_CUBE_MAP_EXTENSION_NAME")]
pub const EXT_NON_SEAMLESS_CUBE_MAP_NAME: &CStr = c"VK_EXT_non_seamless_cube_map";
/// The revision of the
/// [`VK_EXT_non_seamless_cube_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_non_seamless_cube_map.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION`.
#[doc(alias = "VK_EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION")]
pub const EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_opacity_micromap.html),
/// the C macro `VK_EXT_OPACITY_MICROMAP_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME) and
/// ([`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME) or Vulkan 1.3). Promoted to
/// [`VK_KHR_opacity_micromap`](KHR_OPACITY_MICROMAP_NAME).
#[doc(alias = "VK_EXT_opacity_micromap")]
#[doc(alias = "VK_EXT_OPACITY_MICROMAP_EXTENSION_NAME")]
pub const EXT_OPACITY_MICROMAP_NAME: &CStr = c"VK_EXT_opacity_micromap";
/// The revision of the
/// [`VK_EXT_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_opacity_micromap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_OPACITY_MICROMAP_SPEC_VERSION`.
#[doc(alias = "VK_EXT_OPACITY_MICROMAP_SPEC_VERSION")]
pub const EXT_OPACITY_MICROMAP_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_pageable_device_local_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pageable_device_local_memory.html),
/// the C macro `VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_memory_priority`](EXT_MEMORY_PRIORITY_NAME).
#[doc(alias = "VK_EXT_pageable_device_local_memory")]
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_NAME: &CStr = c"VK_EXT_pageable_device_local_memory";
/// The revision of the
/// [`VK_EXT_pageable_device_local_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pageable_device_local_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pci_bus_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pci_bus_info.html),
/// the C macro `VK_EXT_PCI_BUS_INFO_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_pci_bus_info")]
#[doc(alias = "VK_EXT_PCI_BUS_INFO_EXTENSION_NAME")]
pub const EXT_PCI_BUS_INFO_NAME: &CStr = c"VK_EXT_pci_bus_info";
/// The revision of the
/// [`VK_EXT_pci_bus_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pci_bus_info.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PCI_BUS_INFO_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PCI_BUS_INFO_SPEC_VERSION")]
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_physical_device_drm`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_physical_device_drm.html),
/// the C macro `VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_physical_device_drm")]
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME")]
pub const EXT_PHYSICAL_DEVICE_DRM_NAME: &CStr = c"VK_EXT_physical_device_drm";
/// The revision of the
/// [`VK_EXT_physical_device_drm`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_physical_device_drm.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION")]
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pipeline_creation_cache_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_creation_cache_control.html),
/// the C macro `VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_pipeline_creation_cache_control")]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_NAME: &CStr =
    c"VK_EXT_pipeline_creation_cache_control";
/// The revision of the
/// [`VK_EXT_pipeline_creation_cache_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_creation_cache_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_pipeline_creation_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_creation_feedback.html),
/// the C macro `VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_pipeline_creation_feedback")]
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_NAME: &CStr = c"VK_EXT_pipeline_creation_feedback";
/// The revision of the
/// [`VK_EXT_pipeline_creation_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_creation_feedback.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION")]
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pipeline_library_group_handles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_library_group_handles.html),
/// the C macro `VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME) and
/// [`VK_KHR_pipeline_library`](KHR_PIPELINE_LIBRARY_NAME). Promoted to
/// [`VK_KHR_pipeline_library_group_handles`](KHR_PIPELINE_LIBRARY_GROUP_HANDLES_NAME).
#[doc(alias = "VK_EXT_pipeline_library_group_handles")]
#[doc(alias = "VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME")]
pub const EXT_PIPELINE_LIBRARY_GROUP_HANDLES_NAME: &CStr = c"VK_EXT_pipeline_library_group_handles";
/// The revision of the
/// [`VK_EXT_pipeline_library_group_handles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_library_group_handles.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION")]
pub const EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pipeline_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_properties.html),
/// the C macro `VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_pipeline_properties")]
#[doc(alias = "VK_EXT_PIPELINE_PROPERTIES_EXTENSION_NAME")]
pub const EXT_PIPELINE_PROPERTIES_NAME: &CStr = c"VK_EXT_pipeline_properties";
/// The revision of the
/// [`VK_EXT_pipeline_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_PROPERTIES_SPEC_VERSION")]
pub const EXT_PIPELINE_PROPERTIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pipeline_protected_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_protected_access.html),
/// the C macro `VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_EXT_pipeline_protected_access")]
#[doc(alias = "VK_EXT_PIPELINE_PROTECTED_ACCESS_EXTENSION_NAME")]
pub const EXT_PIPELINE_PROTECTED_ACCESS_NAME: &CStr = c"VK_EXT_pipeline_protected_access";
/// The revision of the
/// [`VK_EXT_pipeline_protected_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_protected_access.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION")]
pub const EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_pipeline_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_robustness.html),
/// the C macro `VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_EXT_pipeline_robustness")]
#[doc(alias = "VK_EXT_PIPELINE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_PIPELINE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_pipeline_robustness";
/// The revision of the
/// [`VK_EXT_pipeline_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_pipeline_robustness.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_post_depth_coverage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_post_depth_coverage.html),
/// the C macro `VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_post_depth_coverage")]
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME")]
pub const EXT_POST_DEPTH_COVERAGE_NAME: &CStr = c"VK_EXT_post_depth_coverage";
/// The revision of the
/// [`VK_EXT_post_depth_coverage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_post_depth_coverage.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION")]
pub const EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_present_mode_fifo_latest_ready`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_mode_fifo_latest_ready.html),
/// the C macro `VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME). Promoted to
/// [`VK_KHR_present_mode_fifo_latest_ready`](KHR_PRESENT_MODE_FIFO_LATEST_READY_NAME).
#[doc(alias = "VK_EXT_present_mode_fifo_latest_ready")]
#[doc(alias = "VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME")]
pub const EXT_PRESENT_MODE_FIFO_LATEST_READY_NAME: &CStr = c"VK_EXT_present_mode_fifo_latest_ready";
/// The revision of the
/// [`VK_EXT_present_mode_fifo_latest_ready`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_mode_fifo_latest_ready.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION")]
pub const EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_present_timing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_timing.html),
/// the C macro `VK_EXT_PRESENT_TIMING_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME),
/// [`VK_KHR_present_id2`](KHR_PRESENT_ID_2_NAME),
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME) and
/// [`VK_KHR_calibrated_timestamps`](KHR_CALIBRATED_TIMESTAMPS_NAME).
#[doc(alias = "VK_EXT_present_timing")]
#[doc(alias = "VK_EXT_PRESENT_TIMING_EXTENSION_NAME")]
pub const EXT_PRESENT_TIMING_NAME: &CStr = c"VK_EXT_present_timing";
/// The revision of the
/// [`VK_EXT_present_timing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_present_timing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRESENT_TIMING_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRESENT_TIMING_SPEC_VERSION")]
pub const EXT_PRESENT_TIMING_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_primitive_restart_index`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_restart_index.html),
/// the C macro `VK_EXT_PRIMITIVE_RESTART_INDEX_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_primitive_restart_index")]
#[doc(alias = "VK_EXT_PRIMITIVE_RESTART_INDEX_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_RESTART_INDEX_NAME: &CStr = c"VK_EXT_primitive_restart_index";
/// The revision of the
/// [`VK_EXT_primitive_restart_index`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_restart_index.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRIMITIVE_RESTART_INDEX_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRIMITIVE_RESTART_INDEX_SPEC_VERSION")]
pub const EXT_PRIMITIVE_RESTART_INDEX_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_primitive_topology_list_restart`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_topology_list_restart.html),
/// the C macro `VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_primitive_topology_list_restart")]
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_NAME: &CStr =
    c"VK_EXT_primitive_topology_list_restart";
/// The revision of the
/// [`VK_EXT_primitive_topology_list_restart`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitive_topology_list_restart.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION")]
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_primitives_generated_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitives_generated_query.html),
/// the C macro `VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_transform_feedback`](EXT_TRANSFORM_FEEDBACK_NAME).
#[doc(alias = "VK_EXT_primitives_generated_query")]
#[doc(alias = "VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME")]
pub const EXT_PRIMITIVES_GENERATED_QUERY_NAME: &CStr = c"VK_EXT_primitives_generated_query";
/// The revision of the
/// [`VK_EXT_primitives_generated_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_primitives_generated_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION")]
pub const EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_private_data`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_private_data.html),
/// the C macro `VK_EXT_PRIVATE_DATA_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_private_data")]
#[doc(alias = "VK_EXT_PRIVATE_DATA_EXTENSION_NAME")]
pub const EXT_PRIVATE_DATA_NAME: &CStr = c"VK_EXT_private_data";
/// The revision of the
/// [`VK_EXT_private_data`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_private_data.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PRIVATE_DATA_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PRIVATE_DATA_SPEC_VERSION")]
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_provoking_vertex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_provoking_vertex.html),
/// the C macro `VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_provoking_vertex")]
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME")]
pub const EXT_PROVOKING_VERTEX_NAME: &CStr = c"VK_EXT_provoking_vertex";
/// The revision of the
/// [`VK_EXT_provoking_vertex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_provoking_vertex.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_PROVOKING_VERTEX_SPEC_VERSION`.
#[doc(alias = "VK_EXT_PROVOKING_VERTEX_SPEC_VERSION")]
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_queue_family_foreign`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_queue_family_foreign.html),
/// the C macro `VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_EXT_queue_family_foreign")]
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME")]
pub const EXT_QUEUE_FAMILY_FOREIGN_NAME: &CStr = c"VK_EXT_queue_family_foreign";
/// The revision of the
/// [`VK_EXT_queue_family_foreign`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_queue_family_foreign.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION`.
#[doc(alias = "VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION")]
pub const EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_rasterization_order_attachment_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rasterization_order_attachment_access.html),
/// the C macro `VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_rasterization_order_attachment_access")]
#[doc(alias = "VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME")]
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME: &CStr =
    c"VK_EXT_rasterization_order_attachment_access";
/// The revision of the
/// [`VK_EXT_rasterization_order_attachment_access`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rasterization_order_attachment_access.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION")]
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_ray_tracing_invocation_reorder`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ray_tracing_invocation_reorder.html),
/// the C macro `VK_EXT_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME).
#[doc(alias = "VK_EXT_ray_tracing_invocation_reorder")]
#[doc(alias = "VK_EXT_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME")]
pub const EXT_RAY_TRACING_INVOCATION_REORDER_NAME: &CStr = c"VK_EXT_ray_tracing_invocation_reorder";
/// The revision of the
/// [`VK_EXT_ray_tracing_invocation_reorder`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ray_tracing_invocation_reorder.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION")]
pub const EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_rgba10x6_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rgba10x6_formats.html),
/// the C macro `VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME) or Vulkan
/// 1.1.
#[doc(alias = "VK_EXT_rgba10x6_formats")]
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME")]
pub const EXT_RGBA10X6_FORMATS_NAME: &CStr = c"VK_EXT_rgba10x6_formats";
/// The revision of the
/// [`VK_EXT_rgba10x6_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_rgba10x6_formats.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION")]
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_robustness2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_robustness2.html),
/// the C macro `VK_EXT_ROBUSTNESS_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_robustness2`](KHR_ROBUSTNESS_2_NAME).
#[doc(alias = "VK_EXT_robustness2")]
#[doc(alias = "VK_EXT_ROBUSTNESS_2_EXTENSION_NAME")]
pub const EXT_ROBUSTNESS_2_NAME: &CStr = c"VK_EXT_robustness2";
/// The revision of the
/// [`VK_EXT_robustness2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_robustness2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ROBUSTNESS_2_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ROBUSTNESS_2_SPEC_VERSION")]
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_sample_locations`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sample_locations.html),
/// the C macro `VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_sample_locations")]
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME")]
pub const EXT_SAMPLE_LOCATIONS_NAME: &CStr = c"VK_EXT_sample_locations";
/// The revision of the
/// [`VK_EXT_sample_locations`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sample_locations.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION")]
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_sampler_filter_minmax`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sampler_filter_minmax.html),
/// the C macro `VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_sampler_filter_minmax")]
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME")]
pub const EXT_SAMPLER_FILTER_MINMAX_NAME: &CStr = c"VK_EXT_sampler_filter_minmax";
/// The revision of the
/// [`VK_EXT_sampler_filter_minmax`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_sampler_filter_minmax.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION")]
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_scalar_block_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_scalar_block_layout.html),
/// the C macro `VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_scalar_block_layout")]
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const EXT_SCALAR_BLOCK_LAYOUT_NAME: &CStr = c"VK_EXT_scalar_block_layout";
/// The revision of the
/// [`VK_EXT_scalar_block_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_scalar_block_layout.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION")]
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_separate_stencil_usage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_separate_stencil_usage.html),
/// the C macro `VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_separate_stencil_usage")]
#[doc(alias = "VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME")]
pub const EXT_SEPARATE_STENCIL_USAGE_NAME: &CStr = c"VK_EXT_separate_stencil_usage";
/// The revision of the
/// [`VK_EXT_separate_stencil_usage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_separate_stencil_usage.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION")]
pub const EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_64bit_indexing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_64bit_indexing.html),
/// the C macro `VK_EXT_SHADER_64BIT_INDEXING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_64bit_indexing")]
#[doc(alias = "VK_EXT_SHADER_64BIT_INDEXING_EXTENSION_NAME")]
pub const EXT_SHADER_64BIT_INDEXING_NAME: &CStr = c"VK_EXT_shader_64bit_indexing";
/// The revision of the
/// [`VK_EXT_shader_64bit_indexing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_64bit_indexing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_64BIT_INDEXING_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_64BIT_INDEXING_SPEC_VERSION")]
pub const EXT_SHADER_64BIT_INDEXING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_atomic_float`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float.html),
/// the C macro `VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_atomic_float")]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_NAME: &CStr = c"VK_EXT_shader_atomic_float";
/// The revision of the
/// [`VK_EXT_shader_atomic_float`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_atomic_float2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float2.html),
/// the C macro `VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_shader_atomic_float`](EXT_SHADER_ATOMIC_FLOAT_NAME).
#[doc(alias = "VK_EXT_shader_atomic_float2")]
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_NAME: &CStr = c"VK_EXT_shader_atomic_float2";
/// The revision of the
/// [`VK_EXT_shader_atomic_float2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_atomic_float2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION")]
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_demote_to_helper_invocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_demote_to_helper_invocation.html),
/// the C macro `VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_shader_demote_to_helper_invocation")]
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_NAME: &CStr =
    c"VK_EXT_shader_demote_to_helper_invocation";
/// The revision of the
/// [`VK_EXT_shader_demote_to_helper_invocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_demote_to_helper_invocation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION")]
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_float8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_float8.html),
/// the C macro `VK_EXT_SHADER_FLOAT8_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_float8")]
#[doc(alias = "VK_EXT_SHADER_FLOAT8_EXTENSION_NAME")]
pub const EXT_SHADER_FLOAT8_NAME: &CStr = c"VK_EXT_shader_float8";
/// The revision of the
/// [`VK_EXT_shader_float8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_float8.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_FLOAT8_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_FLOAT8_SPEC_VERSION")]
pub const EXT_SHADER_FLOAT8_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_image_atomic_int64`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_image_atomic_int64.html),
/// the C macro `VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_image_atomic_int64")]
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_NAME: &CStr = c"VK_EXT_shader_image_atomic_int64";
/// The revision of the
/// [`VK_EXT_shader_image_atomic_int64`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_image_atomic_int64.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION")]
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_long_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_long_vector.html),
/// the C macro `VK_EXT_SHADER_LONG_VECTOR_EXTENSION_NAME`.
///
/// Requires Vulkan 1.2.
#[doc(alias = "VK_EXT_shader_long_vector")]
#[doc(alias = "VK_EXT_SHADER_LONG_VECTOR_EXTENSION_NAME")]
pub const EXT_SHADER_LONG_VECTOR_NAME: &CStr = c"VK_EXT_shader_long_vector";
/// The revision of the
/// [`VK_EXT_shader_long_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_long_vector.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_LONG_VECTOR_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_LONG_VECTOR_SPEC_VERSION")]
pub const EXT_SHADER_LONG_VECTOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_module_identifier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_module_identifier.html),
/// the C macro `VK_EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and
/// [`VK_EXT_pipeline_creation_cache_control`](EXT_PIPELINE_CREATION_CACHE_CONTROL_NAME)) or
/// Vulkan 1.3.
#[doc(alias = "VK_EXT_shader_module_identifier")]
#[doc(alias = "VK_EXT_SHADER_MODULE_IDENTIFIER_EXTENSION_NAME")]
pub const EXT_SHADER_MODULE_IDENTIFIER_NAME: &CStr = c"VK_EXT_shader_module_identifier";
/// The revision of the
/// [`VK_EXT_shader_module_identifier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_module_identifier.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION")]
pub const EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_object`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_object.html),
/// the C macro `VK_EXT_SHADER_OBJECT_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_dynamic_rendering`](KHR_DYNAMIC_RENDERING_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_EXT_shader_object")]
#[doc(alias = "VK_EXT_SHADER_OBJECT_EXTENSION_NAME")]
pub const EXT_SHADER_OBJECT_NAME: &CStr = c"VK_EXT_shader_object";
/// The revision of the
/// [`VK_EXT_shader_object`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_object.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_OBJECT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_OBJECT_SPEC_VERSION")]
pub const EXT_SHADER_OBJECT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_ocp_microscaling_types`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_ocp_microscaling_types.html),
/// the C macro `VK_EXT_SHADER_OCP_MICROSCALING_TYPES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_ocp_microscaling_types")]
#[doc(alias = "VK_EXT_SHADER_OCP_MICROSCALING_TYPES_EXTENSION_NAME")]
pub const EXT_SHADER_OCP_MICROSCALING_TYPES_NAME: &CStr = c"VK_EXT_shader_ocp_microscaling_types";
/// The revision of the
/// [`VK_EXT_shader_ocp_microscaling_types`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_ocp_microscaling_types.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_OCP_MICROSCALING_TYPES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_OCP_MICROSCALING_TYPES_SPEC_VERSION")]
pub const EXT_SHADER_OCP_MICROSCALING_TYPES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_replicated_composites`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_replicated_composites.html),
/// the C macro `VK_EXT_SHADER_REPLICATED_COMPOSITES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_replicated_composites")]
#[doc(alias = "VK_EXT_SHADER_REPLICATED_COMPOSITES_EXTENSION_NAME")]
pub const EXT_SHADER_REPLICATED_COMPOSITES_NAME: &CStr = c"VK_EXT_shader_replicated_composites";
/// The revision of the
/// [`VK_EXT_shader_replicated_composites`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_replicated_composites.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION")]
pub const EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_split_barrier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_split_barrier.html),
/// the C macro `VK_EXT_SHADER_SPLIT_BARRIER_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_split_barrier")]
#[doc(alias = "VK_EXT_SHADER_SPLIT_BARRIER_EXTENSION_NAME")]
pub const EXT_SHADER_SPLIT_BARRIER_NAME: &CStr = c"VK_EXT_shader_split_barrier";
/// The revision of the
/// [`VK_EXT_shader_split_barrier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_split_barrier.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_SPLIT_BARRIER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_SPLIT_BARRIER_SPEC_VERSION")]
pub const EXT_SHADER_SPLIT_BARRIER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_stencil_export`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_stencil_export.html),
/// the C macro `VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_shader_stencil_export")]
#[doc(alias = "VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME")]
pub const EXT_SHADER_STENCIL_EXPORT_NAME: &CStr = c"VK_EXT_shader_stencil_export";
/// The revision of the
/// [`VK_EXT_shader_stencil_export`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_stencil_export.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION")]
pub const EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_subgroup_ballot`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_ballot.html),
/// the C macro `VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME`.
///
/// Deprecated by Vulkan 1.2.
#[doc(alias = "VK_EXT_shader_subgroup_ballot")]
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME")]
pub const EXT_SHADER_SUBGROUP_BALLOT_NAME: &CStr = c"VK_EXT_shader_subgroup_ballot";
/// The revision of the
/// [`VK_EXT_shader_subgroup_ballot`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_ballot.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION")]
pub const EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_subgroup_partitioned`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_partitioned.html),
/// the C macro `VK_EXT_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_subgroup_partitioned")]
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME")]
pub const EXT_SHADER_SUBGROUP_PARTITIONED_NAME: &CStr = c"VK_EXT_shader_subgroup_partitioned";
/// The revision of the
/// [`VK_EXT_shader_subgroup_partitioned`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_partitioned.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION")]
pub const EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_subgroup_vote`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_vote.html),
/// the C macro `VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME`.
///
/// Deprecated by Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_subgroup_vote")]
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME")]
pub const EXT_SHADER_SUBGROUP_VOTE_NAME: &CStr = c"VK_EXT_shader_subgroup_vote";
/// The revision of the
/// [`VK_EXT_shader_subgroup_vote`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_subgroup_vote.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION")]
pub const EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_tile_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_tile_image.html),
/// the C macro `VK_EXT_SHADER_TILE_IMAGE_EXTENSION_NAME`.
///
/// Requires Vulkan 1.3.
#[doc(alias = "VK_EXT_shader_tile_image")]
#[doc(alias = "VK_EXT_SHADER_TILE_IMAGE_EXTENSION_NAME")]
pub const EXT_SHADER_TILE_IMAGE_NAME: &CStr = c"VK_EXT_shader_tile_image";
/// The revision of the
/// [`VK_EXT_shader_tile_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_tile_image.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_TILE_IMAGE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_TILE_IMAGE_SPEC_VERSION")]
pub const EXT_SHADER_TILE_IMAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_uniform_buffer_unsized_array`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_uniform_buffer_unsized_array.html),
/// the C macro `VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_shader_uniform_buffer_unsized_array")]
#[doc(alias = "VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_EXTENSION_NAME")]
pub const EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_NAME: &CStr =
    c"VK_EXT_shader_uniform_buffer_unsized_array";
/// The revision of the
/// [`VK_EXT_shader_uniform_buffer_unsized_array`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_uniform_buffer_unsized_array.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION")]
pub const EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_shader_viewport_index_layer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_viewport_index_layer.html),
/// the C macro `VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_EXT_shader_viewport_index_layer")]
#[doc(alias = "VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME")]
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_NAME: &CStr = c"VK_EXT_shader_viewport_index_layer";
/// The revision of the
/// [`VK_EXT_shader_viewport_index_layer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_shader_viewport_index_layer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION")]
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_subgroup_size_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subgroup_size_control.html),
/// the C macro `VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_subgroup_size_control")]
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME")]
pub const EXT_SUBGROUP_SIZE_CONTROL_NAME: &CStr = c"VK_EXT_subgroup_size_control";
/// The revision of the
/// [`VK_EXT_subgroup_size_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subgroup_size_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION")]
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_subpass_merge_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subpass_merge_feedback.html),
/// the C macro `VK_EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_subpass_merge_feedback")]
#[doc(alias = "VK_EXT_SUBPASS_MERGE_FEEDBACK_EXTENSION_NAME")]
pub const EXT_SUBPASS_MERGE_FEEDBACK_NAME: &CStr = c"VK_EXT_subpass_merge_feedback";
/// The revision of the
/// [`VK_EXT_subpass_merge_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_subpass_merge_feedback.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION")]
pub const EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_EXT_surface_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_surface_maintenance1.html),
/// the C macro `VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME) and
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME). Promoted to
/// [`VK_KHR_surface_maintenance1`](KHR_SURFACE_MAINTENANCE_1_NAME).
#[doc(alias = "VK_EXT_surface_maintenance1")]
#[doc(alias = "VK_EXT_SURFACE_MAINTENANCE_1_EXTENSION_NAME")]
pub const EXT_SURFACE_MAINTENANCE_1_NAME: &CStr = c"VK_EXT_surface_maintenance1";
/// The revision of the
/// [`VK_EXT_surface_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_surface_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION")]
pub const EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_swapchain_colorspace`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_colorspace.html),
/// the C macro `VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_EXT_swapchain_colorspace")]
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_NAME: &CStr = c"VK_EXT_swapchain_colorspace";
/// The revision of the
/// [`VK_EXT_swapchain_colorspace`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_colorspace.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 5;

/// The name of the device extension
/// [`VK_EXT_swapchain_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_maintenance1.html),
/// the C macro `VK_EXT_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME),
/// [`VK_EXT_surface_maintenance1`](EXT_SURFACE_MAINTENANCE_1_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1). Promoted to
/// [`VK_KHR_swapchain_maintenance1`](KHR_SWAPCHAIN_MAINTENANCE_1_NAME).
#[doc(alias = "VK_EXT_swapchain_maintenance1")]
#[doc(alias = "VK_EXT_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME")]
pub const EXT_SWAPCHAIN_MAINTENANCE_1_NAME: &CStr = c"VK_EXT_swapchain_maintenance1";
/// The revision of the
/// [`VK_EXT_swapchain_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_swapchain_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION")]
pub const EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_texel_buffer_alignment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texel_buffer_alignment.html),
/// the C macro `VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_texel_buffer_alignment")]
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_NAME: &CStr = c"VK_EXT_texel_buffer_alignment";
/// The revision of the
/// [`VK_EXT_texel_buffer_alignment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texel_buffer_alignment.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION`.
#[doc(alias = "VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION")]
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_texture_compression_astc_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_3d.html),
/// the C macro `VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_texture_compression_astc_3d")]
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_EXTENSION_NAME")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_3D_NAME: &CStr = c"VK_EXT_texture_compression_astc_3d";
/// The revision of the
/// [`VK_EXT_texture_compression_astc_3d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_3d.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION`.
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_texture_compression_astc_hdr`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_hdr.html),
/// the C macro `VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_texture_compression_astc_hdr")]
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_NAME: &CStr = c"VK_EXT_texture_compression_astc_hdr";
/// The revision of the
/// [`VK_EXT_texture_compression_astc_hdr`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_texture_compression_astc_hdr.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION`.
#[doc(alias = "VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION")]
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_tooling_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_tooling_info.html),
/// the C macro `VK_EXT_TOOLING_INFO_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_tooling_info")]
#[doc(alias = "VK_EXT_TOOLING_INFO_EXTENSION_NAME")]
pub const EXT_TOOLING_INFO_NAME: &CStr = c"VK_EXT_tooling_info";
/// The revision of the
/// [`VK_EXT_tooling_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_tooling_info.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_TOOLING_INFO_SPEC_VERSION`.
#[doc(alias = "VK_EXT_TOOLING_INFO_SPEC_VERSION")]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_transform_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_transform_feedback.html),
/// the C macro `VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_transform_feedback")]
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME")]
pub const EXT_TRANSFORM_FEEDBACK_NAME: &CStr = c"VK_EXT_transform_feedback";
/// The revision of the
/// [`VK_EXT_transform_feedback`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_transform_feedback.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION`.
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION")]
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_validation_cache`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_cache.html),
/// the C macro `VK_EXT_VALIDATION_CACHE_EXTENSION_NAME`.
#[doc(alias = "VK_EXT_validation_cache")]
#[doc(alias = "VK_EXT_VALIDATION_CACHE_EXTENSION_NAME")]
pub const EXT_VALIDATION_CACHE_NAME: &CStr = c"VK_EXT_validation_cache";
/// The revision of the
/// [`VK_EXT_validation_cache`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_cache.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VALIDATION_CACHE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VALIDATION_CACHE_SPEC_VERSION")]
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_EXT_validation_features`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_features.html),
/// the C macro `VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME`.
///
/// Deprecated by [`VK_EXT_layer_settings`](EXT_LAYER_SETTINGS_NAME).
#[doc(alias = "VK_EXT_validation_features")]
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME")]
pub const EXT_VALIDATION_FEATURES_NAME: &CStr = c"VK_EXT_validation_features";
/// The revision of the
/// [`VK_EXT_validation_features`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_features.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VALIDATION_FEATURES_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_SPEC_VERSION")]
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 6;

/// The name of the instance extension
/// [`VK_EXT_validation_flags`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_flags.html),
/// the C macro `VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME`.
///
/// Deprecated by [`VK_EXT_layer_settings`](EXT_LAYER_SETTINGS_NAME).
#[doc(alias = "VK_EXT_validation_flags")]
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME")]
pub const EXT_VALIDATION_FLAGS_NAME: &CStr = c"VK_EXT_validation_flags";
/// The revision of the
/// [`VK_EXT_validation_flags`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_validation_flags.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VALIDATION_FLAGS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VALIDATION_FLAGS_SPEC_VERSION")]
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_vertex_attribute_divisor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_divisor.html),
/// the C macro `VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to
/// [`VK_KHR_vertex_attribute_divisor`](KHR_VERTEX_ATTRIBUTE_DIVISOR_NAME).
#[doc(alias = "VK_EXT_vertex_attribute_divisor")]
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_NAME: &CStr = c"VK_EXT_vertex_attribute_divisor";
/// The revision of the
/// [`VK_EXT_vertex_attribute_divisor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_divisor.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_EXT_vertex_attribute_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_robustness.html),
/// the C macro `VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to [`VK_KHR_maintenance9`](KHR_MAINTENANCE_9_NAME).
#[doc(alias = "VK_EXT_vertex_attribute_robustness")]
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_EXTENSION_NAME")]
pub const EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_vertex_attribute_robustness";
/// The revision of the
/// [`VK_EXT_vertex_attribute_robustness`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_attribute_robustness.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION")]
pub const EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_vertex_input_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_input_dynamic_state.html),
/// the C macro `VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_vertex_input_dynamic_state")]
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_NAME: &CStr = c"VK_EXT_vertex_input_dynamic_state";
/// The revision of the
/// [`VK_EXT_vertex_input_dynamic_state`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_vertex_input_dynamic_state.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION`.
#[doc(alias = "VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION")]
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_EXT_ycbcr_2plane_444_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_2plane_444_formats.html),
/// the C macro `VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME) or Vulkan
/// 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_EXT_ycbcr_2plane_444_formats")]
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_NAME: &CStr = c"VK_EXT_ycbcr_2plane_444_formats";
/// The revision of the
/// [`VK_EXT_ycbcr_2plane_444_formats`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_2plane_444_formats.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_ycbcr_image_arrays`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_image_arrays.html),
/// the C macro `VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME) or Vulkan
/// 1.1.
#[doc(alias = "VK_EXT_ycbcr_image_arrays")]
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME")]
pub const EXT_YCBCR_IMAGE_ARRAYS_NAME: &CStr = c"VK_EXT_ycbcr_image_arrays";
/// The revision of the
/// [`VK_EXT_ycbcr_image_arrays`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_ycbcr_image_arrays.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION`.
#[doc(alias = "VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION")]
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_EXT_zero_initialize_device_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_zero_initialize_device_memory.html),
/// the C macro `VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_EXT_zero_initialize_device_memory")]
#[doc(alias = "VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_EXTENSION_NAME")]
pub const EXT_ZERO_INITIALIZE_DEVICE_MEMORY_NAME: &CStr = c"VK_EXT_zero_initialize_device_memory";
/// The revision of the
/// [`VK_EXT_zero_initialize_device_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_zero_initialize_device_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION")]
pub const EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_FUCHSIA_buffer_collection`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_buffer_collection.html),
/// the C macro `VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME`.
///
/// Requires [`VK_FUCHSIA_external_memory`](FUCHSIA_EXTERNAL_MEMORY_NAME) and
/// ([`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME) or Vulkan 1.1).
#[doc(alias = "VK_FUCHSIA_buffer_collection")]
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_EXTENSION_NAME")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_BUFFER_COLLECTION_NAME: &CStr = c"VK_FUCHSIA_buffer_collection";
/// The revision of the
/// [`VK_FUCHSIA_buffer_collection`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_buffer_collection.html)
/// extension that these bindings were generated from, the C macro
/// `VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION`.
#[doc(alias = "VK_FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_FUCHSIA_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_memory.html),
/// the C macro `VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_external_memory_capabilities`](KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME) and
/// [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME)) or Vulkan 1.1.
#[doc(alias = "VK_FUCHSIA_external_memory")]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_EXTENSION_NAME")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_MEMORY_NAME: &CStr = c"VK_FUCHSIA_external_memory";
/// The revision of the
/// [`VK_FUCHSIA_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_FUCHSIA_external_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_semaphore.html),
/// the C macro `VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_external_semaphore_capabilities`](KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_NAME) and
/// [`VK_KHR_external_semaphore`](KHR_EXTERNAL_SEMAPHORE_NAME).
#[doc(alias = "VK_FUCHSIA_external_semaphore")]
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_NAME: &CStr = c"VK_FUCHSIA_external_semaphore";
/// The revision of the
/// [`VK_FUCHSIA_external_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_external_semaphore.html)
/// extension that these bindings were generated from, the C macro
/// `VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION`.
#[doc(alias = "VK_FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_FUCHSIA_imagepipe_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_imagepipe_surface.html),
/// the C macro `VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_FUCHSIA_imagepipe_surface")]
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_NAME: &CStr = c"VK_FUCHSIA_imagepipe_surface";
/// The revision of the
/// [`VK_FUCHSIA_imagepipe_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_FUCHSIA_imagepipe_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION")]
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_GGP_frame_token`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_frame_token.html),
/// the C macro `VK_GGP_FRAME_TOKEN_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// [`VK_GGP_stream_descriptor_surface`](GGP_STREAM_DESCRIPTOR_SURFACE_NAME).
#[doc(alias = "VK_GGP_frame_token")]
#[doc(alias = "VK_GGP_FRAME_TOKEN_EXTENSION_NAME")]
#[cfg(feature = "ggp")]
pub const GGP_FRAME_TOKEN_NAME: &CStr = c"VK_GGP_frame_token";
/// The revision of the
/// [`VK_GGP_frame_token`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_frame_token.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GGP_FRAME_TOKEN_SPEC_VERSION`.
#[doc(alias = "VK_GGP_FRAME_TOKEN_SPEC_VERSION")]
#[cfg(feature = "ggp")]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_GGP_stream_descriptor_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_stream_descriptor_surface.html),
/// the C macro `VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_GGP_stream_descriptor_surface")]
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "ggp")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_NAME: &CStr = c"VK_GGP_stream_descriptor_surface";
/// The revision of the
/// [`VK_GGP_stream_descriptor_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GGP_stream_descriptor_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION")]
#[cfg(feature = "ggp")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_GOOGLE_decorate_string`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_decorate_string.html),
/// the C macro `VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME`.
#[doc(alias = "VK_GOOGLE_decorate_string")]
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME")]
pub const GOOGLE_DECORATE_STRING_NAME: &CStr = c"VK_GOOGLE_decorate_string";
/// The revision of the
/// [`VK_GOOGLE_decorate_string`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_decorate_string.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GOOGLE_DECORATE_STRING_SPEC_VERSION`.
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_SPEC_VERSION")]
pub const GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_GOOGLE_display_timing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_display_timing.html),
/// the C macro `VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_GOOGLE_display_timing")]
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_NAME: &CStr = c"VK_GOOGLE_display_timing";
/// The revision of the
/// [`VK_GOOGLE_display_timing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_display_timing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION`.
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_GOOGLE_hlsl_functionality1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_hlsl_functionality1.html),
/// the C macro `VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME`.
#[doc(alias = "VK_GOOGLE_hlsl_functionality1")]
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_NAME: &CStr = c"VK_GOOGLE_hlsl_functionality1";
/// The revision of the
/// [`VK_GOOGLE_hlsl_functionality1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_hlsl_functionality1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION`.
#[doc(alias = "VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION")]
pub const GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_GOOGLE_surfaceless_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_surfaceless_query.html),
/// the C macro `VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_GOOGLE_surfaceless_query")]
#[doc(alias = "VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME")]
pub const GOOGLE_SURFACELESS_QUERY_NAME: &CStr = c"VK_GOOGLE_surfaceless_query";
/// The revision of the
/// [`VK_GOOGLE_surfaceless_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_surfaceless_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION")]
pub const GOOGLE_SURFACELESS_QUERY_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_GOOGLE_user_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_user_type.html),
/// the C macro `VK_GOOGLE_USER_TYPE_EXTENSION_NAME`.
#[doc(alias = "VK_GOOGLE_user_type")]
#[doc(alias = "VK_GOOGLE_USER_TYPE_EXTENSION_NAME")]
pub const GOOGLE_USER_TYPE_NAME: &CStr = c"VK_GOOGLE_user_type";
/// The revision of the
/// [`VK_GOOGLE_user_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_GOOGLE_user_type.html)
/// extension that these bindings were generated from, the C macro
/// `VK_GOOGLE_USER_TYPE_SPEC_VERSION`.
#[doc(alias = "VK_GOOGLE_USER_TYPE_SPEC_VERSION")]
pub const GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_HUAWEI_cluster_culling_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_cluster_culling_shader.html),
/// the C macro `VK_HUAWEI_CLUSTER_CULLING_SHADER_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_HUAWEI_cluster_culling_shader")]
#[doc(alias = "VK_HUAWEI_CLUSTER_CULLING_SHADER_EXTENSION_NAME")]
pub const HUAWEI_CLUSTER_CULLING_SHADER_NAME: &CStr = c"VK_HUAWEI_cluster_culling_shader";
/// The revision of the
/// [`VK_HUAWEI_cluster_culling_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_cluster_culling_shader.html)
/// extension that these bindings were generated from, the C macro
/// `VK_HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION`.
#[doc(alias = "VK_HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION")]
pub const HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_HUAWEI_hdr_vivid`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_hdr_vivid.html),
/// the C macro `VK_HUAWEI_HDR_VIVID_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// [`VK_EXT_hdr_metadata`](EXT_HDR_METADATA_NAME).
#[doc(alias = "VK_HUAWEI_hdr_vivid")]
#[doc(alias = "VK_HUAWEI_HDR_VIVID_EXTENSION_NAME")]
pub const HUAWEI_HDR_VIVID_NAME: &CStr = c"VK_HUAWEI_hdr_vivid";
/// The revision of the
/// [`VK_HUAWEI_hdr_vivid`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_hdr_vivid.html)
/// extension that these bindings were generated from, the C macro
/// `VK_HUAWEI_HDR_VIVID_SPEC_VERSION`.
#[doc(alias = "VK_HUAWEI_HDR_VIVID_SPEC_VERSION")]
pub const HUAWEI_HDR_VIVID_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_HUAWEI_invocation_mask`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_invocation_mask.html),
/// the C macro `VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME) and
/// ([`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME) or Vulkan 1.3).
#[doc(alias = "VK_HUAWEI_invocation_mask")]
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME")]
pub const HUAWEI_INVOCATION_MASK_NAME: &CStr = c"VK_HUAWEI_invocation_mask";
/// The revision of the
/// [`VK_HUAWEI_invocation_mask`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_invocation_mask.html)
/// extension that these bindings were generated from, the C macro
/// `VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION`.
#[doc(alias = "VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION")]
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_HUAWEI_subpass_shading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_subpass_shading.html),
/// the C macro `VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME) or Vulkan 1.2) and
/// [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_HUAWEI_subpass_shading")]
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME")]
pub const HUAWEI_SUBPASS_SHADING_NAME: &CStr = c"VK_HUAWEI_subpass_shading";
/// The revision of the
/// [`VK_HUAWEI_subpass_shading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HUAWEI_subpass_shading.html)
/// extension that these bindings were generated from, the C macro
/// `VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION`.
#[doc(alias = "VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION")]
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_IMG_filter_cubic`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_filter_cubic.html),
/// the C macro `VK_IMG_FILTER_CUBIC_EXTENSION_NAME`.
#[doc(alias = "VK_IMG_filter_cubic")]
#[doc(alias = "VK_IMG_FILTER_CUBIC_EXTENSION_NAME")]
pub const IMG_FILTER_CUBIC_NAME: &CStr = c"VK_IMG_filter_cubic";
/// The revision of the
/// [`VK_IMG_filter_cubic`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_filter_cubic.html)
/// extension that these bindings were generated from, the C macro
/// `VK_IMG_FILTER_CUBIC_SPEC_VERSION`.
#[doc(alias = "VK_IMG_FILTER_CUBIC_SPEC_VERSION")]
pub const IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_IMG_filter_linear_2d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_filter_linear_2d.html),
/// the C macro `VK_IMG_FILTER_LINEAR_2D_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_format_feature_flags2`](KHR_FORMAT_FEATURE_FLAGS_2_NAME) or Vulkan 1.3.
#[doc(alias = "VK_IMG_filter_linear_2d")]
#[doc(alias = "VK_IMG_FILTER_LINEAR_2D_EXTENSION_NAME")]
pub const IMG_FILTER_LINEAR_2D_NAME: &CStr = c"VK_IMG_filter_linear_2d";
/// The revision of the
/// [`VK_IMG_filter_linear_2d`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_filter_linear_2d.html)
/// extension that these bindings were generated from, the C macro
/// `VK_IMG_FILTER_LINEAR_2D_SPEC_VERSION`.
#[doc(alias = "VK_IMG_FILTER_LINEAR_2D_SPEC_VERSION")]
pub const IMG_FILTER_LINEAR_2D_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_IMG_format_pvrtc`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_format_pvrtc.html),
/// the C macro `VK_IMG_FORMAT_PVRTC_EXTENSION_NAME`.
///
/// Deprecated.
#[doc(alias = "VK_IMG_format_pvrtc")]
#[doc(alias = "VK_IMG_FORMAT_PVRTC_EXTENSION_NAME")]
pub const IMG_FORMAT_PVRTC_NAME: &CStr = c"VK_IMG_format_pvrtc";
/// The revision of the
/// [`VK_IMG_format_pvrtc`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_format_pvrtc.html)
/// extension that these bindings were generated from, the C macro
/// `VK_IMG_FORMAT_PVRTC_SPEC_VERSION`.
#[doc(alias = "VK_IMG_FORMAT_PVRTC_SPEC_VERSION")]
pub const IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_IMG_relaxed_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_relaxed_line_rasterization.html),
/// the C macro `VK_IMG_RELAXED_LINE_RASTERIZATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_IMG_relaxed_line_rasterization")]
#[doc(alias = "VK_IMG_RELAXED_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const IMG_RELAXED_LINE_RASTERIZATION_NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";
/// The revision of the
/// [`VK_IMG_relaxed_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_IMG_relaxed_line_rasterization.html)
/// extension that these bindings were generated from, the C macro
/// `VK_IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION`.
#[doc(alias = "VK_IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION")]
pub const IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_INTEL_device_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_device_info.html),
/// the C macro `VK_INTEL_DEVICE_INFO_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_INTEL_device_info")]
#[doc(alias = "VK_INTEL_DEVICE_INFO_EXTENSION_NAME")]
pub const INTEL_DEVICE_INFO_NAME: &CStr = c"VK_INTEL_device_info";
/// The revision of the
/// [`VK_INTEL_device_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_device_info.html)
/// extension that these bindings were generated from, the C macro
/// `VK_INTEL_DEVICE_INFO_SPEC_VERSION`.
#[doc(alias = "VK_INTEL_DEVICE_INFO_SPEC_VERSION")]
pub const INTEL_DEVICE_INFO_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_INTEL_performance_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_performance_query.html),
/// the C macro `VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME`.
#[doc(alias = "VK_INTEL_performance_query")]
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_NAME: &CStr = c"VK_INTEL_performance_query";
/// The revision of the
/// [`VK_INTEL_performance_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_performance_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_INTEL_shader_integer_functions2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_shader_integer_functions2.html),
/// the C macro `VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_INTEL_shader_integer_functions2")]
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_NAME: &CStr = c"VK_INTEL_shader_integer_functions2";
/// The revision of the
/// [`VK_INTEL_shader_integer_functions2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_INTEL_shader_integer_functions2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION`.
#[doc(alias = "VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION")]
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_16bit_storage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_16bit_storage.html),
/// the C macro `VK_KHR_16BIT_STORAGE_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_storage_buffer_storage_class`](KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME)) or Vulkan
/// 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_16bit_storage")]
#[doc(alias = "VK_KHR_16BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_16BIT_STORAGE_NAME: &CStr = c"VK_KHR_16bit_storage";
/// The revision of the
/// [`VK_KHR_16bit_storage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_16bit_storage.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_16BIT_STORAGE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_16BIT_STORAGE_SPEC_VERSION")]
pub const KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_8bit_storage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_8bit_storage.html),
/// the C macro `VK_KHR_8BIT_STORAGE_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_storage_buffer_storage_class`](KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME)) or Vulkan
/// 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_8bit_storage")]
#[doc(alias = "VK_KHR_8BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_8BIT_STORAGE_NAME: &CStr = c"VK_KHR_8bit_storage";
/// The revision of the
/// [`VK_KHR_8bit_storage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_8bit_storage.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_8BIT_STORAGE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_8BIT_STORAGE_SPEC_VERSION")]
pub const KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_acceleration_structure.html),
/// the C macro `VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME`.
///
/// Requires ((Vulkan 1.1, [`VK_EXT_descriptor_indexing`](EXT_DESCRIPTOR_INDEXING_NAME) and
/// [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME)) or Vulkan 1.2) and
/// [`VK_KHR_deferred_host_operations`](KHR_DEFERRED_HOST_OPERATIONS_NAME).
#[doc(alias = "VK_KHR_acceleration_structure")]
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_NAME: &CStr = c"VK_KHR_acceleration_structure";
/// The revision of the
/// [`VK_KHR_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_acceleration_structure.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;

/// The name of the instance extension
/// [`VK_KHR_android_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_android_surface.html),
/// the C macro `VK_KHR_ANDROID_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_android_surface")]
#[doc(alias = "VK_KHR_ANDROID_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "android")]
pub const KHR_ANDROID_SURFACE_NAME: &CStr = c"VK_KHR_android_surface";
/// The revision of the
/// [`VK_KHR_android_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_android_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_ANDROID_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_ANDROID_SURFACE_SPEC_VERSION")]
#[cfg(feature = "android")]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;

/// The name of the device extension
/// [`VK_KHR_bind_memory2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_bind_memory2.html),
/// the C macro `VK_KHR_BIND_MEMORY_2_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_bind_memory2")]
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_NAME: &CStr = c"VK_KHR_bind_memory2";
/// The revision of the
/// [`VK_KHR_bind_memory2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_bind_memory2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_BIND_MEMORY_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_buffer_device_address`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_buffer_device_address.html),
/// the C macro `VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_device_group`](KHR_DEVICE_GROUP_NAME)) or Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_buffer_device_address")]
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME")]
pub const KHR_BUFFER_DEVICE_ADDRESS_NAME: &CStr = c"VK_KHR_buffer_device_address";
/// The revision of the
/// [`VK_KHR_buffer_device_address`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_buffer_device_address.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION")]
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_calibrated_timestamps`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_calibrated_timestamps.html),
/// the C macro `VK_KHR_CALIBRATED_TIMESTAMPS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_calibrated_timestamps")]
#[doc(alias = "VK_KHR_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const KHR_CALIBRATED_TIMESTAMPS_NAME: &CStr = c"VK_KHR_calibrated_timestamps";
/// The revision of the
/// [`VK_KHR_calibrated_timestamps`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_calibrated_timestamps.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_compute_shader_derivatives`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_compute_shader_derivatives.html),
/// the C macro `VK_KHR_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_compute_shader_derivatives")]
#[doc(alias = "VK_KHR_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const KHR_COMPUTE_SHADER_DERIVATIVES_NAME: &CStr = c"VK_KHR_compute_shader_derivatives";
/// The revision of the
/// [`VK_KHR_compute_shader_derivatives`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_compute_shader_derivatives.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_cooperative_matrix`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_cooperative_matrix.html),
/// the C macro `VK_KHR_COOPERATIVE_MATRIX_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_cooperative_matrix")]
#[doc(alias = "VK_KHR_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const KHR_COOPERATIVE_MATRIX_NAME: &CStr = c"VK_KHR_cooperative_matrix";
/// The revision of the
/// [`VK_KHR_cooperative_matrix`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_cooperative_matrix.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_COOPERATIVE_MATRIX_SPEC_VERSION`.
#[doc(alias = "VK_KHR_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const KHR_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_copy_commands2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_commands2.html),
/// the C macro `VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_copy_commands2")]
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME")]
pub const KHR_COPY_COMMANDS_2_NAME: &CStr = c"VK_KHR_copy_commands2";
/// The revision of the
/// [`VK_KHR_copy_commands2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_commands2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_COPY_COMMANDS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_COPY_COMMANDS_2_SPEC_VERSION")]
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_copy_memory_indirect`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_memory_indirect.html),
/// the C macro `VK_KHR_COPY_MEMORY_INDIRECT_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME)) or Vulkan 1.2.
#[doc(alias = "VK_KHR_copy_memory_indirect")]
#[doc(alias = "VK_KHR_COPY_MEMORY_INDIRECT_EXTENSION_NAME")]
pub const KHR_COPY_MEMORY_INDIRECT_NAME: &CStr = c"VK_KHR_copy_memory_indirect";
/// The revision of the
/// [`VK_KHR_copy_memory_indirect`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_copy_memory_indirect.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION")]
pub const KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_create_renderpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_create_renderpass2.html),
/// the C macro `VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_multiview`](KHR_MULTIVIEW_NAME) and
/// [`VK_KHR_maintenance2`](KHR_MAINTENANCE_2_NAME)) or Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_create_renderpass2")]
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME")]
pub const KHR_CREATE_RENDERPASS_2_NAME: &CStr = c"VK_KHR_create_renderpass2";
/// The revision of the
/// [`VK_KHR_create_renderpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_create_renderpass2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION")]
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_dedicated_allocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dedicated_allocation.html),
/// the C macro `VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_get_memory_requirements2`](KHR_GET_MEMORY_REQUIREMENTS_2_NAME) or Vulkan
/// 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_dedicated_allocation")]
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const KHR_DEDICATED_ALLOCATION_NAME: &CStr = c"VK_KHR_dedicated_allocation";
/// The revision of the
/// [`VK_KHR_dedicated_allocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dedicated_allocation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_KHR_deferred_host_operations`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_deferred_host_operations.html),
/// the C macro `VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME`.
#[doc(alias = "VK_KHR_deferred_host_operations")]
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME")]
pub const KHR_DEFERRED_HOST_OPERATIONS_NAME: &CStr = c"VK_KHR_deferred_host_operations";
/// The revision of the
/// [`VK_KHR_deferred_host_operations`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_deferred_host_operations.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION")]
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_KHR_depth_clamp_zero_one`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_clamp_zero_one.html),
/// the C macro `VK_KHR_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_depth_clamp_zero_one")]
#[doc(alias = "VK_KHR_DEPTH_CLAMP_ZERO_ONE_EXTENSION_NAME")]
pub const KHR_DEPTH_CLAMP_ZERO_ONE_NAME: &CStr = c"VK_KHR_depth_clamp_zero_one";
/// The revision of the
/// [`VK_KHR_depth_clamp_zero_one`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_clamp_zero_one.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION")]
pub const KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_depth_stencil_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_stencil_resolve.html),
/// the C macro `VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME) or Vulkan 1.2. Promoted
/// to Vulkan 1.2.
#[doc(alias = "VK_KHR_depth_stencil_resolve")]
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME")]
pub const KHR_DEPTH_STENCIL_RESOLVE_NAME: &CStr = c"VK_KHR_depth_stencil_resolve";
/// The revision of the
/// [`VK_KHR_depth_stencil_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_depth_stencil_resolve.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION")]
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_descriptor_update_template`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_descriptor_update_template.html),
/// the C macro `VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_descriptor_update_template")]
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_NAME: &CStr = c"VK_KHR_descriptor_update_template";
/// The revision of the
/// [`VK_KHR_descriptor_update_template`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_descriptor_update_template.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION")]
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_device_address_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_address_commands.html),
/// the C macro `VK_KHR_DEVICE_ADDRESS_COMMANDS_EXTENSION_NAME`.
///
/// Requires
/// (((([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME)) or Vulkan
/// 1.2), [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME) and
/// [`VK_EXT_extended_dynamic_state`](EXT_EXTENDED_DYNAMIC_STATE_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_KHR_device_address_commands")]
#[doc(alias = "VK_KHR_DEVICE_ADDRESS_COMMANDS_EXTENSION_NAME")]
pub const KHR_DEVICE_ADDRESS_COMMANDS_NAME: &CStr = c"VK_KHR_device_address_commands";
/// The revision of the
/// [`VK_KHR_device_address_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_address_commands.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION")]
pub const KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_device_fault`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_fault.html),
/// the C macro `VK_KHR_DEVICE_FAULT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_device_fault")]
#[doc(alias = "VK_KHR_DEVICE_FAULT_EXTENSION_NAME")]
pub const KHR_DEVICE_FAULT_NAME: &CStr = c"VK_KHR_device_fault";
/// The revision of the
/// [`VK_KHR_device_fault`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_fault.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEVICE_FAULT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEVICE_FAULT_SPEC_VERSION")]
pub const KHR_DEVICE_FAULT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_device_group`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_group.html),
/// the C macro `VK_KHR_DEVICE_GROUP_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_device_group_creation`](KHR_DEVICE_GROUP_CREATION_NAME). Promoted to
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_device_group")]
#[doc(alias = "VK_KHR_DEVICE_GROUP_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_NAME: &CStr = c"VK_KHR_device_group";
/// The revision of the
/// [`VK_KHR_device_group`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_group.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEVICE_GROUP_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEVICE_GROUP_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;

/// The name of the instance extension
/// [`VK_KHR_device_group_creation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_group_creation.html),
/// the C macro `VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_device_group_creation")]
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME")]
pub const KHR_DEVICE_GROUP_CREATION_NAME: &CStr = c"VK_KHR_device_group_creation";
/// The revision of the
/// [`VK_KHR_device_group_creation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_device_group_creation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION")]
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display.html),
/// the C macro `VK_KHR_DISPLAY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_display")]
#[doc(alias = "VK_KHR_DISPLAY_EXTENSION_NAME")]
pub const KHR_DISPLAY_NAME: &CStr = c"VK_KHR_display";
/// The revision of the
/// [`VK_KHR_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DISPLAY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DISPLAY_SPEC_VERSION")]
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;

/// The name of the device extension
/// [`VK_KHR_display_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display_swapchain.html),
/// the C macro `VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and [`VK_KHR_display`](KHR_DISPLAY_NAME).
#[doc(alias = "VK_KHR_display_swapchain")]
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_NAME: &CStr = c"VK_KHR_display_swapchain";
/// The revision of the
/// [`VK_KHR_display_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_display_swapchain.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;

/// The name of the device extension
/// [`VK_KHR_draw_indirect_count`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_draw_indirect_count.html),
/// the C macro `VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_draw_indirect_count")]
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME")]
pub const KHR_DRAW_INDIRECT_COUNT_NAME: &CStr = c"VK_KHR_draw_indirect_count";
/// The revision of the
/// [`VK_KHR_draw_indirect_count`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_draw_indirect_count.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION")]
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_driver_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_driver_properties.html),
/// the C macro `VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_driver_properties")]
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME")]
pub const KHR_DRIVER_PROPERTIES_NAME: &CStr = c"VK_KHR_driver_properties";
/// The revision of the
/// [`VK_KHR_driver_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_driver_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION")]
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_dynamic_rendering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering.html),
/// the C macro `VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_depth_stencil_resolve`](KHR_DEPTH_STENCIL_RESOLVE_NAME)) or Vulkan
/// 1.2. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_dynamic_rendering")]
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_NAME: &CStr = c"VK_KHR_dynamic_rendering";
/// The revision of the
/// [`VK_KHR_dynamic_rendering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_dynamic_rendering_local_read`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering_local_read.html),
/// the C macro `VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_dynamic_rendering`](KHR_DYNAMIC_RENDERING_NAME) or Vulkan 1.3. Promoted to
/// Vulkan 1.4.
#[doc(alias = "VK_KHR_dynamic_rendering_local_read")]
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_LOCAL_READ_NAME: &CStr = c"VK_KHR_dynamic_rendering_local_read";
/// The revision of the
/// [`VK_KHR_dynamic_rendering_local_read`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_dynamic_rendering_local_read.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION`.
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_extended_flags`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_extended_flags.html),
/// the C macro `VK_KHR_EXTENDED_FLAGS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_extended_flags")]
#[doc(alias = "VK_KHR_EXTENDED_FLAGS_EXTENSION_NAME")]
pub const KHR_EXTENDED_FLAGS_NAME: &CStr = c"VK_KHR_extended_flags";
/// The revision of the
/// [`VK_KHR_extended_flags`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_extended_flags.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTENDED_FLAGS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTENDED_FLAGS_SPEC_VERSION")]
pub const KHR_EXTENDED_FLAGS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_fence`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence.html),
/// the C macro `VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_fence_capabilities`](KHR_EXTERNAL_FENCE_CAPABILITIES_NAME).
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_fence")]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_NAME: &CStr = c"VK_KHR_external_fence";
/// The revision of the
/// [`VK_KHR_external_fence`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_FENCE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_external_fence_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_capabilities.html),
/// the C macro `VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_fence_capabilities")]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_NAME: &CStr = c"VK_KHR_external_fence_capabilities";
/// The revision of the
/// [`VK_KHR_external_fence_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_capabilities.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_fence_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_fd.html),
/// the C macro `VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_fence`](KHR_EXTERNAL_FENCE_NAME) or Vulkan 1.1.
#[doc(alias = "VK_KHR_external_fence_fd")]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_FD_NAME: &CStr = c"VK_KHR_external_fence_fd";
/// The revision of the
/// [`VK_KHR_external_fence_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_fd.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_fence_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_win32.html),
/// the C macro `VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_fence`](KHR_EXTERNAL_FENCE_NAME).
#[doc(alias = "VK_KHR_external_fence_win32")]
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_FENCE_WIN32_NAME: &CStr = c"VK_KHR_external_fence_win32";
/// The revision of the
/// [`VK_KHR_external_fence_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_fence_win32.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory.html),
/// the C macro `VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory_capabilities`](KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_memory")]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_NAME: &CStr = c"VK_KHR_external_memory";
/// The revision of the
/// [`VK_KHR_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_external_memory_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_capabilities.html),
/// the C macro `VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_memory_capabilities")]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME: &CStr = c"VK_KHR_external_memory_capabilities";
/// The revision of the
/// [`VK_KHR_external_memory_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_capabilities.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_memory_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_fd.html),
/// the C macro `VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_KHR_external_memory_fd")]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_NAME: &CStr = c"VK_KHR_external_memory_fd";
/// The revision of the
/// [`VK_KHR_external_memory_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_fd.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_memory_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_win32.html),
/// the C macro `VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_KHR_external_memory_win32")]
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_MEMORY_WIN32_NAME: &CStr = c"VK_KHR_external_memory_win32";
/// The revision of the
/// [`VK_KHR_external_memory_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_memory_win32.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore.html),
/// the C macro `VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_external_semaphore_capabilities`](KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_NAME).
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_semaphore")]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_NAME: &CStr = c"VK_KHR_external_semaphore";
/// The revision of the
/// [`VK_KHR_external_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_external_semaphore_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_capabilities.html),
/// the C macro `VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_external_semaphore_capabilities")]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_NAME: &CStr =
    c"VK_KHR_external_semaphore_capabilities";
/// The revision of the
/// [`VK_KHR_external_semaphore_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_capabilities.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_semaphore_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_fd.html),
/// the C macro `VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_semaphore`](KHR_EXTERNAL_SEMAPHORE_NAME) or Vulkan 1.1.
#[doc(alias = "VK_KHR_external_semaphore_fd")]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_NAME: &CStr = c"VK_KHR_external_semaphore_fd";
/// The revision of the
/// [`VK_KHR_external_semaphore_fd`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_fd.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_external_semaphore_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_win32.html),
/// the C macro `VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_semaphore`](KHR_EXTERNAL_SEMAPHORE_NAME).
#[doc(alias = "VK_KHR_external_semaphore_win32")]
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_NAME: &CStr = c"VK_KHR_external_semaphore_win32";
/// The revision of the
/// [`VK_KHR_external_semaphore_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_external_semaphore_win32.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION`.
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_format_feature_flags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_format_feature_flags2.html),
/// the C macro `VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_format_feature_flags2")]
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_NAME: &CStr = c"VK_KHR_format_feature_flags2";
/// The revision of the
/// [`VK_KHR_format_feature_flags2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_format_feature_flags2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_fragment_shader_barycentric`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shader_barycentric.html),
/// the C macro `VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_fragment_shader_barycentric")]
#[doc(alias = "VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_NAME: &CStr = c"VK_KHR_fragment_shader_barycentric";
/// The revision of the
/// [`VK_KHR_fragment_shader_barycentric`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shader_barycentric.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION`.
#[doc(alias = "VK_KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_fragment_shading_rate`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shading_rate.html),
/// the C macro `VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME)) or Vulkan 1.2.
#[doc(alias = "VK_KHR_fragment_shading_rate")]
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_NAME: &CStr = c"VK_KHR_fragment_shading_rate";
/// The revision of the
/// [`VK_KHR_fragment_shading_rate`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_fragment_shading_rate.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_KHR_get_display_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_display_properties2.html),
/// the C macro `VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_display`](KHR_DISPLAY_NAME).
#[doc(alias = "VK_KHR_get_display_properties2")]
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_NAME: &CStr = c"VK_KHR_get_display_properties2";
/// The revision of the
/// [`VK_KHR_get_display_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_display_properties2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_get_memory_requirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_memory_requirements2.html),
/// the C macro `VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_get_memory_requirements2")]
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_NAME: &CStr = c"VK_KHR_get_memory_requirements2";
/// The revision of the
/// [`VK_KHR_get_memory_requirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_memory_requirements2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION")]
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_get_physical_device_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_physical_device_properties2.html),
/// the C macro `VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_get_physical_device_properties2")]
#[doc(alias = "VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME")]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME: &CStr =
    c"VK_KHR_get_physical_device_properties2";
/// The revision of the
/// [`VK_KHR_get_physical_device_properties2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_physical_device_properties2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION")]
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_KHR_get_surface_capabilities2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_surface_capabilities2.html),
/// the C macro `VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_get_surface_capabilities2")]
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_NAME: &CStr = c"VK_KHR_get_surface_capabilities2";
/// The revision of the
/// [`VK_KHR_get_surface_capabilities2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_get_surface_capabilities2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION")]
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_global_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_global_priority.html),
/// the C macro `VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_global_priority")]
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const KHR_GLOBAL_PRIORITY_NAME: &CStr = c"VK_KHR_global_priority";
/// The revision of the
/// [`VK_KHR_global_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_global_priority.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_image_format_list`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_image_format_list.html),
/// the C macro `VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_image_format_list")]
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME")]
pub const KHR_IMAGE_FORMAT_LIST_NAME: &CStr = c"VK_KHR_image_format_list";
/// The revision of the
/// [`VK_KHR_image_format_list`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_image_format_list.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION`.
#[doc(alias = "VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION")]
pub const KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_imageless_framebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_imageless_framebuffer.html),
/// the C macro `VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME`.
///
/// Requires
/// ((([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_maintenance2`](KHR_MAINTENANCE_2_NAME)) or Vulkan 1.1) and
/// [`VK_KHR_image_format_list`](KHR_IMAGE_FORMAT_LIST_NAME)) or Vulkan 1.2. Promoted to Vulkan
/// 1.2.
#[doc(alias = "VK_KHR_imageless_framebuffer")]
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME")]
pub const KHR_IMAGELESS_FRAMEBUFFER_NAME: &CStr = c"VK_KHR_imageless_framebuffer";
/// The revision of the
/// [`VK_KHR_imageless_framebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_imageless_framebuffer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION`.
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION")]
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_incremental_present`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_incremental_present.html),
/// the C macro `VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_KHR_incremental_present")]
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME")]
pub const KHR_INCREMENTAL_PRESENT_NAME: &CStr = c"VK_KHR_incremental_present";
/// The revision of the
/// [`VK_KHR_incremental_present`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_incremental_present.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION")]
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_index_type_uint8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_index_type_uint8.html),
/// the C macro `VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_index_type_uint8")]
#[doc(alias = "VK_KHR_INDEX_TYPE_UINT8_EXTENSION_NAME")]
pub const KHR_INDEX_TYPE_UINT8_NAME: &CStr = c"VK_KHR_index_type_uint8";
/// The revision of the
/// [`VK_KHR_index_type_uint8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_index_type_uint8.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION`.
#[doc(alias = "VK_KHR_INDEX_TYPE_UINT8_SPEC_VERSION")]
pub const KHR_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_internally_synchronized_queues`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_internally_synchronized_queues.html),
/// the C macro `VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_KHR_internally_synchronized_queues")]
#[doc(alias = "VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_EXTENSION_NAME")]
pub const KHR_INTERNALLY_SYNCHRONIZED_QUEUES_NAME: &CStr = c"VK_KHR_internally_synchronized_queues";
/// The revision of the
/// [`VK_KHR_internally_synchronized_queues`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_internally_synchronized_queues.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION")]
pub const KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_line_rasterization.html),
/// the C macro `VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_line_rasterization")]
#[doc(alias = "VK_KHR_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const KHR_LINE_RASTERIZATION_NAME: &CStr = c"VK_KHR_line_rasterization";
/// The revision of the
/// [`VK_KHR_line_rasterization`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_line_rasterization.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_LINE_RASTERIZATION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_LINE_RASTERIZATION_SPEC_VERSION")]
pub const KHR_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_load_store_op_none`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_load_store_op_none.html),
/// the C macro `VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_load_store_op_none")]
#[doc(alias = "VK_KHR_LOAD_STORE_OP_NONE_EXTENSION_NAME")]
pub const KHR_LOAD_STORE_OP_NONE_NAME: &CStr = c"VK_KHR_load_store_op_none";
/// The revision of the
/// [`VK_KHR_load_store_op_none`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_load_store_op_none.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_LOAD_STORE_OP_NONE_SPEC_VERSION")]
pub const KHR_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance1.html),
/// the C macro `VK_KHR_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance1")]
#[doc(alias = "VK_KHR_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_maintenance1";
/// The revision of the
/// [`VK_KHR_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_maintenance10`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance10.html),
/// the C macro `VK_KHR_MAINTENANCE_10_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance10")]
#[doc(alias = "VK_KHR_MAINTENANCE_10_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_10_NAME: &CStr = c"VK_KHR_maintenance10";
/// The revision of the
/// [`VK_KHR_maintenance10`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance10.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_10_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_10_SPEC_VERSION")]
pub const KHR_MAINTENANCE_10_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance11`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance11.html),
/// the C macro `VK_KHR_MAINTENANCE_11_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance11")]
#[doc(alias = "VK_KHR_MAINTENANCE_11_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_11_NAME: &CStr = c"VK_KHR_maintenance11";
/// The revision of the
/// [`VK_KHR_maintenance11`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance11.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_11_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_11_SPEC_VERSION")]
pub const KHR_MAINTENANCE_11_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance2.html),
/// the C macro `VK_KHR_MAINTENANCE_2_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance2")]
#[doc(alias = "VK_KHR_MAINTENANCE_2_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_2_NAME: &CStr = c"VK_KHR_maintenance2";
/// The revision of the
/// [`VK_KHR_maintenance2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_2_SPEC_VERSION")]
pub const KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance3.html),
/// the C macro `VK_KHR_MAINTENANCE_3_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance3")]
#[doc(alias = "VK_KHR_MAINTENANCE_3_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_3_NAME: &CStr = c"VK_KHR_maintenance3";
/// The revision of the
/// [`VK_KHR_maintenance3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance3.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_3_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_3_SPEC_VERSION")]
pub const KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance4`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance4.html),
/// the C macro `VK_KHR_MAINTENANCE_4_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_maintenance4")]
#[doc(alias = "VK_KHR_MAINTENANCE_4_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_4_NAME: &CStr = c"VK_KHR_maintenance4";
/// The revision of the
/// [`VK_KHR_maintenance4`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance4.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_4_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_4_SPEC_VERSION")]
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_maintenance5`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance5.html),
/// the C macro `VK_KHR_MAINTENANCE_5_EXTENSION_NAME`.
///
/// Requires (Vulkan 1.1 and [`VK_KHR_dynamic_rendering`](KHR_DYNAMIC_RENDERING_NAME)) or Vulkan
/// 1.3. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_maintenance5")]
#[doc(alias = "VK_KHR_MAINTENANCE_5_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_5_NAME: &CStr = c"VK_KHR_maintenance5";
/// The revision of the
/// [`VK_KHR_maintenance5`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance5.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_5_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_5_SPEC_VERSION")]
pub const KHR_MAINTENANCE_5_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance6`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance6.html),
/// the C macro `VK_KHR_MAINTENANCE_6_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_maintenance6")]
#[doc(alias = "VK_KHR_MAINTENANCE_6_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_6_NAME: &CStr = c"VK_KHR_maintenance6";
/// The revision of the
/// [`VK_KHR_maintenance6`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance6.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_6_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_6_SPEC_VERSION")]
pub const KHR_MAINTENANCE_6_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance7`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance7.html),
/// the C macro `VK_KHR_MAINTENANCE_7_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance7")]
#[doc(alias = "VK_KHR_MAINTENANCE_7_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_7_NAME: &CStr = c"VK_KHR_maintenance7";
/// The revision of the
/// [`VK_KHR_maintenance7`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance7.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_7_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_7_SPEC_VERSION")]
pub const KHR_MAINTENANCE_7_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance8.html),
/// the C macro `VK_KHR_MAINTENANCE_8_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance8")]
#[doc(alias = "VK_KHR_MAINTENANCE_8_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_8_NAME: &CStr = c"VK_KHR_maintenance8";
/// The revision of the
/// [`VK_KHR_maintenance8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance8.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_8_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_8_SPEC_VERSION")]
pub const KHR_MAINTENANCE_8_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_maintenance9`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance9.html),
/// the C macro `VK_KHR_MAINTENANCE_9_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_maintenance9")]
#[doc(alias = "VK_KHR_MAINTENANCE_9_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_9_NAME: &CStr = c"VK_KHR_maintenance9";
/// The revision of the
/// [`VK_KHR_maintenance9`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_maintenance9.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAINTENANCE_9_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAINTENANCE_9_SPEC_VERSION")]
pub const KHR_MAINTENANCE_9_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_map_memory2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_map_memory2.html),
/// the C macro `VK_KHR_MAP_MEMORY_2_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_map_memory2")]
#[doc(alias = "VK_KHR_MAP_MEMORY_2_EXTENSION_NAME")]
pub const KHR_MAP_MEMORY_2_NAME: &CStr = c"VK_KHR_map_memory2";
/// The revision of the
/// [`VK_KHR_map_memory2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_map_memory2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MAP_MEMORY_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MAP_MEMORY_2_SPEC_VERSION")]
pub const KHR_MAP_MEMORY_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_multiview`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_multiview.html),
/// the C macro `VK_KHR_MULTIVIEW_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_multiview")]
#[doc(alias = "VK_KHR_MULTIVIEW_EXTENSION_NAME")]
pub const KHR_MULTIVIEW_NAME: &CStr = c"VK_KHR_multiview";
/// The revision of the
/// [`VK_KHR_multiview`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_multiview.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_MULTIVIEW_SPEC_VERSION`.
#[doc(alias = "VK_KHR_MULTIVIEW_SPEC_VERSION")]
pub const KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_opacity_micromap.html),
/// the C macro `VK_KHR_OPACITY_MICROMAP_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME) and
/// [`VK_KHR_device_address_commands`](KHR_DEVICE_ADDRESS_COMMANDS_NAME).
#[doc(alias = "VK_KHR_opacity_micromap")]
#[doc(alias = "VK_KHR_OPACITY_MICROMAP_EXTENSION_NAME")]
pub const KHR_OPACITY_MICROMAP_NAME: &CStr = c"VK_KHR_opacity_micromap";
/// The revision of the
/// [`VK_KHR_opacity_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_opacity_micromap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_OPACITY_MICROMAP_SPEC_VERSION`.
#[doc(alias = "VK_KHR_OPACITY_MICROMAP_SPEC_VERSION")]
pub const KHR_OPACITY_MICROMAP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_performance_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_performance_query.html),
/// the C macro `VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_performance_query")]
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_NAME: &CStr = c"VK_KHR_performance_query";
/// The revision of the
/// [`VK_KHR_performance_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_performance_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_pipeline_binary`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_binary.html),
/// the C macro `VK_KHR_PIPELINE_BINARY_EXTENSION_NAME`.
///
/// Requires Vulkan 1.4, [`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME).
#[doc(alias = "VK_KHR_pipeline_binary")]
#[doc(alias = "VK_KHR_PIPELINE_BINARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_BINARY_NAME: &CStr = c"VK_KHR_pipeline_binary";
/// The revision of the
/// [`VK_KHR_pipeline_binary`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_binary.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PIPELINE_BINARY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PIPELINE_BINARY_SPEC_VERSION")]
pub const KHR_PIPELINE_BINARY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_pipeline_executable_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_executable_properties.html),
/// the C macro `VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_pipeline_executable_properties")]
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_NAME: &CStr = c"VK_KHR_pipeline_executable_properties";
/// The revision of the
/// [`VK_KHR_pipeline_executable_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_executable_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION")]
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_pipeline_library`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_library.html),
/// the C macro `VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME`.
#[doc(alias = "VK_KHR_pipeline_library")]
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_NAME: &CStr = c"VK_KHR_pipeline_library";
/// The revision of the
/// [`VK_KHR_pipeline_library`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_library.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_pipeline_library_group_handles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_library_group_handles.html),
/// the C macro `VK_KHR_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME) and
/// [`VK_KHR_pipeline_library`](KHR_PIPELINE_LIBRARY_NAME).
#[doc(alias = "VK_KHR_pipeline_library_group_handles")]
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_GROUP_HANDLES_EXTENSION_NAME")]
pub const KHR_PIPELINE_LIBRARY_GROUP_HANDLES_NAME: &CStr = c"VK_KHR_pipeline_library_group_handles";
/// The revision of the
/// [`VK_KHR_pipeline_library_group_handles`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_pipeline_library_group_handles.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION")]
pub const KHR_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_portability_enumeration`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_portability_enumeration.html),
/// the C macro `VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME`.
#[doc(alias = "VK_KHR_portability_enumeration")]
#[doc(alias = "VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME")]
pub const KHR_PORTABILITY_ENUMERATION_NAME: &CStr = c"VK_KHR_portability_enumeration";
/// The revision of the
/// [`VK_KHR_portability_enumeration`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_portability_enumeration.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION")]
pub const KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;

/// The name of the provisional device extension
/// [`VK_KHR_portability_subset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_portability_subset.html),
/// the C macro `VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_portability_subset")]
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_EXTENSION_NAME")]
#[cfg(feature = "beta")]
pub const KHR_PORTABILITY_SUBSET_NAME: &CStr = c"VK_KHR_portability_subset";
/// The revision of the
/// [`VK_KHR_portability_subset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_portability_subset.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PORTABILITY_SUBSET_SPEC_VERSION")]
#[cfg(feature = "beta")]
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_present_id`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id.html),
/// the C macro `VK_KHR_PRESENT_ID_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1).
#[doc(alias = "VK_KHR_present_id")]
#[doc(alias = "VK_KHR_PRESENT_ID_EXTENSION_NAME")]
pub const KHR_PRESENT_ID_NAME: &CStr = c"VK_KHR_present_id";
/// The revision of the
/// [`VK_KHR_present_id`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PRESENT_ID_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PRESENT_ID_SPEC_VERSION")]
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_present_id2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id2.html),
/// the C macro `VK_KHR_PRESENT_ID_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME),
/// [`VK_KHR_surface`](KHR_SURFACE_NAME) and [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_KHR_present_id2")]
#[doc(alias = "VK_KHR_PRESENT_ID_2_EXTENSION_NAME")]
pub const KHR_PRESENT_ID_2_NAME: &CStr = c"VK_KHR_present_id2";
/// The revision of the
/// [`VK_KHR_present_id2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_id2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PRESENT_ID_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PRESENT_ID_2_SPEC_VERSION")]
pub const KHR_PRESENT_ID_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_present_mode_fifo_latest_ready`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_mode_fifo_latest_ready.html),
/// the C macro `VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_KHR_present_mode_fifo_latest_ready")]
#[doc(alias = "VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_EXTENSION_NAME")]
pub const KHR_PRESENT_MODE_FIFO_LATEST_READY_NAME: &CStr = c"VK_KHR_present_mode_fifo_latest_ready";
/// The revision of the
/// [`VK_KHR_present_mode_fifo_latest_ready`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_mode_fifo_latest_ready.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION")]
pub const KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_present_wait`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait.html),
/// the C macro `VK_KHR_PRESENT_WAIT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// [`VK_KHR_present_id`](KHR_PRESENT_ID_NAME).
#[doc(alias = "VK_KHR_present_wait")]
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_NAME: &CStr = c"VK_KHR_present_wait";
/// The revision of the
/// [`VK_KHR_present_wait`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PRESENT_WAIT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_present_wait2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait2.html),
/// the C macro `VK_KHR_PRESENT_WAIT_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME),
/// [`VK_KHR_surface`](KHR_SURFACE_NAME), [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME) and
/// [`VK_KHR_present_id2`](KHR_PRESENT_ID_2_NAME).
#[doc(alias = "VK_KHR_present_wait2")]
#[doc(alias = "VK_KHR_PRESENT_WAIT_2_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_2_NAME: &CStr = c"VK_KHR_present_wait2";
/// The revision of the
/// [`VK_KHR_present_wait2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_present_wait2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PRESENT_WAIT_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PRESENT_WAIT_2_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_push_descriptor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_push_descriptor.html),
/// the C macro `VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_push_descriptor")]
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME")]
pub const KHR_PUSH_DESCRIPTOR_NAME: &CStr = c"VK_KHR_push_descriptor";
/// The revision of the
/// [`VK_KHR_push_descriptor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_push_descriptor.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION`.
#[doc(alias = "VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION")]
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_ray_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_query.html),
/// the C macro `VK_KHR_RAY_QUERY_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_spirv_1_4`](KHR_SPIRV_1_4_NAME) or Vulkan 1.2) and
/// [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_KHR_ray_query")]
#[doc(alias = "VK_KHR_RAY_QUERY_EXTENSION_NAME")]
pub const KHR_RAY_QUERY_NAME: &CStr = c"VK_KHR_ray_query";
/// The revision of the
/// [`VK_KHR_ray_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_RAY_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_RAY_QUERY_SPEC_VERSION")]
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_ray_tracing_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_maintenance1.html),
/// the C macro `VK_KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_KHR_ray_tracing_maintenance1")]
#[doc(alias = "VK_KHR_RAY_TRACING_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_ray_tracing_maintenance1";
/// The revision of the
/// [`VK_KHR_ray_tracing_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_ray_tracing_pipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_pipeline.html),
/// the C macro `VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_spirv_1_4`](KHR_SPIRV_1_4_NAME) or Vulkan 1.2) and
/// [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_KHR_ray_tracing_pipeline")]
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_PIPELINE_NAME: &CStr = c"VK_KHR_ray_tracing_pipeline";
/// The revision of the
/// [`VK_KHR_ray_tracing_pipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_pipeline.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION")]
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_ray_tracing_position_fetch`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_position_fetch.html),
/// the C macro `VK_KHR_RAY_TRACING_POSITION_FETCH_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_KHR_ray_tracing_position_fetch")]
#[doc(alias = "VK_KHR_RAY_TRACING_POSITION_FETCH_EXTENSION_NAME")]
pub const KHR_RAY_TRACING_POSITION_FETCH_NAME: &CStr = c"VK_KHR_ray_tracing_position_fetch";
/// The revision of the
/// [`VK_KHR_ray_tracing_position_fetch`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_ray_tracing_position_fetch.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION`.
#[doc(alias = "VK_KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION")]
pub const KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_relaxed_block_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_relaxed_block_layout.html),
/// the C macro `VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_relaxed_block_layout")]
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME")]
pub const KHR_RELAXED_BLOCK_LAYOUT_NAME: &CStr = c"VK_KHR_relaxed_block_layout";
/// The revision of the
/// [`VK_KHR_relaxed_block_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_relaxed_block_layout.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION")]
pub const KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_robustness2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_robustness2.html),
/// the C macro `VK_KHR_ROBUSTNESS_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_robustness2")]
#[doc(alias = "VK_KHR_ROBUSTNESS_2_EXTENSION_NAME")]
pub const KHR_ROBUSTNESS_2_NAME: &CStr = c"VK_KHR_robustness2";
/// The revision of the
/// [`VK_KHR_robustness2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_robustness2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_ROBUSTNESS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_ROBUSTNESS_2_SPEC_VERSION")]
pub const KHR_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_sampler_mirror_clamp_to_edge`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_sampler_mirror_clamp_to_edge.html),
/// the C macro `VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_sampler_mirror_clamp_to_edge")]
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_NAME: &CStr = c"VK_KHR_sampler_mirror_clamp_to_edge";
/// The revision of the
/// [`VK_KHR_sampler_mirror_clamp_to_edge`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_sampler_mirror_clamp_to_edge.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION")]
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_KHR_sampler_ycbcr_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_sampler_ycbcr_conversion.html),
/// the C macro `VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_maintenance1`](KHR_MAINTENANCE_1_NAME),
/// [`VK_KHR_bind_memory2`](KHR_BIND_MEMORY_2_NAME),
/// [`VK_KHR_get_memory_requirements2`](KHR_GET_MEMORY_REQUIREMENTS_2_NAME) and
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME)) or
/// Vulkan 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_sampler_ycbcr_conversion")]
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_NAME: &CStr = c"VK_KHR_sampler_ycbcr_conversion";
/// The revision of the
/// [`VK_KHR_sampler_ycbcr_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_sampler_ycbcr_conversion.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION")]
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;

/// The name of the device extension
/// [`VK_KHR_separate_depth_stencil_layouts`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_separate_depth_stencil_layouts.html),
/// the C macro `VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_create_renderpass2`](KHR_CREATE_RENDERPASS_2_NAME)) or Vulkan 1.2.
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_separate_depth_stencil_layouts")]
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_NAME: &CStr = c"VK_KHR_separate_depth_stencil_layouts";
/// The revision of the
/// [`VK_KHR_separate_depth_stencil_layouts`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_separate_depth_stencil_layouts.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_abort`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_abort.html),
/// the C macro `VK_KHR_SHADER_ABORT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_device_fault`](KHR_DEVICE_FAULT_NAME) and
/// [`VK_KHR_shader_constant_data`](KHR_SHADER_CONSTANT_DATA_NAME).
#[doc(alias = "VK_KHR_shader_abort")]
#[doc(alias = "VK_KHR_SHADER_ABORT_EXTENSION_NAME")]
pub const KHR_SHADER_ABORT_NAME: &CStr = c"VK_KHR_shader_abort";
/// The revision of the
/// [`VK_KHR_shader_abort`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_abort.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_ABORT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_ABORT_SPEC_VERSION")]
pub const KHR_SHADER_ABORT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_atomic_int64`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_atomic_int64.html),
/// the C macro `VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_shader_atomic_int64")]
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME")]
pub const KHR_SHADER_ATOMIC_INT64_NAME: &CStr = c"VK_KHR_shader_atomic_int64";
/// The revision of the
/// [`VK_KHR_shader_atomic_int64`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_atomic_int64.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION")]
pub const KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_bfloat16`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_bfloat16.html),
/// the C macro `VK_KHR_SHADER_BFLOAT16_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_bfloat16")]
#[doc(alias = "VK_KHR_SHADER_BFLOAT16_EXTENSION_NAME")]
pub const KHR_SHADER_BFLOAT16_NAME: &CStr = c"VK_KHR_shader_bfloat16";
/// The revision of the
/// [`VK_KHR_shader_bfloat16`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_bfloat16.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_BFLOAT16_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_BFLOAT16_SPEC_VERSION")]
pub const KHR_SHADER_BFLOAT16_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_clock`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_clock.html),
/// the C macro `VK_KHR_SHADER_CLOCK_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_clock")]
#[doc(alias = "VK_KHR_SHADER_CLOCK_EXTENSION_NAME")]
pub const KHR_SHADER_CLOCK_NAME: &CStr = c"VK_KHR_shader_clock";
/// The revision of the
/// [`VK_KHR_shader_clock`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_clock.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_CLOCK_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_CLOCK_SPEC_VERSION")]
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_constant_data`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_constant_data.html),
/// the C macro `VK_KHR_SHADER_CONSTANT_DATA_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_constant_data")]
#[doc(alias = "VK_KHR_SHADER_CONSTANT_DATA_EXTENSION_NAME")]
pub const KHR_SHADER_CONSTANT_DATA_NAME: &CStr = c"VK_KHR_shader_constant_data";
/// The revision of the
/// [`VK_KHR_shader_constant_data`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_constant_data.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_CONSTANT_DATA_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_CONSTANT_DATA_SPEC_VERSION")]
pub const KHR_SHADER_CONSTANT_DATA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_draw_parameters`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_draw_parameters.html),
/// the C macro `VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_draw_parameters")]
#[doc(alias = "VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME")]
pub const KHR_SHADER_DRAW_PARAMETERS_NAME: &CStr = c"VK_KHR_shader_draw_parameters";
/// The revision of the
/// [`VK_KHR_shader_draw_parameters`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_draw_parameters.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION")]
pub const KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_expect_assume`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_expect_assume.html),
/// the C macro `VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_shader_expect_assume")]
#[doc(alias = "VK_KHR_SHADER_EXPECT_ASSUME_EXTENSION_NAME")]
pub const KHR_SHADER_EXPECT_ASSUME_NAME: &CStr = c"VK_KHR_shader_expect_assume";
/// The revision of the
/// [`VK_KHR_shader_expect_assume`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_expect_assume.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION")]
pub const KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_float16_int8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float16_int8.html),
/// the C macro `VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_shader_float16_int8")]
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT16_INT8_NAME: &CStr = c"VK_KHR_shader_float16_int8";
/// The revision of the
/// [`VK_KHR_shader_float16_int8`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float16_int8.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_float_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls.html),
/// the C macro `VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_shader_float_controls")]
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT_CONTROLS_NAME: &CStr = c"VK_KHR_shader_float_controls";
/// The revision of the
/// [`VK_KHR_shader_float_controls`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_KHR_shader_float_controls2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls2.html),
/// the C macro `VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1 and [`VK_KHR_shader_float_controls`](KHR_SHADER_FLOAT_CONTROLS_NAME).
/// Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_shader_float_controls2")]
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_2_EXTENSION_NAME")]
pub const KHR_SHADER_FLOAT_CONTROLS_2_NAME: &CStr = c"VK_KHR_shader_float_controls2";
/// The revision of the
/// [`VK_KHR_shader_float_controls2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_float_controls2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION")]
pub const KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_fma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_fma.html),
/// the C macro `VK_KHR_SHADER_FMA_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_fma")]
#[doc(alias = "VK_KHR_SHADER_FMA_EXTENSION_NAME")]
pub const KHR_SHADER_FMA_NAME: &CStr = c"VK_KHR_shader_fma";
/// The revision of the
/// [`VK_KHR_shader_fma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_fma.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_FMA_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_FMA_SPEC_VERSION")]
pub const KHR_SHADER_FMA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_integer_dot_product`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_integer_dot_product.html),
/// the C macro `VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_shader_integer_dot_product")]
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_NAME: &CStr = c"VK_KHR_shader_integer_dot_product";
/// The revision of the
/// [`VK_KHR_shader_integer_dot_product`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_integer_dot_product.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION")]
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_maximal_reconvergence`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_maximal_reconvergence.html),
/// the C macro `VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_maximal_reconvergence")]
#[doc(alias = "VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_EXTENSION_NAME")]
pub const KHR_SHADER_MAXIMAL_RECONVERGENCE_NAME: &CStr = c"VK_KHR_shader_maximal_reconvergence";
/// The revision of the
/// [`VK_KHR_shader_maximal_reconvergence`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_maximal_reconvergence.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION")]
pub const KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_non_semantic_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_non_semantic_info.html),
/// the C macro `VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_shader_non_semantic_info")]
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_NAME: &CStr = c"VK_KHR_shader_non_semantic_info";
/// The revision of the
/// [`VK_KHR_shader_non_semantic_info`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_non_semantic_info.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION")]
pub const KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_quad_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_quad_control.html),
/// the C macro `VK_KHR_SHADER_QUAD_CONTROL_EXTENSION_NAME`.
///
/// Requires ((Vulkan 1.1 and [`VK_KHR_vulkan_memory_model`](KHR_VULKAN_MEMORY_MODEL_NAME)) or
/// Vulkan 1.2) and
/// [`VK_KHR_shader_maximal_reconvergence`](KHR_SHADER_MAXIMAL_RECONVERGENCE_NAME).
#[doc(alias = "VK_KHR_shader_quad_control")]
#[doc(alias = "VK_KHR_SHADER_QUAD_CONTROL_EXTENSION_NAME")]
pub const KHR_SHADER_QUAD_CONTROL_NAME: &CStr = c"VK_KHR_shader_quad_control";
/// The revision of the
/// [`VK_KHR_shader_quad_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_quad_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_QUAD_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_QUAD_CONTROL_SPEC_VERSION")]
pub const KHR_SHADER_QUAD_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_relaxed_extended_instruction`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_relaxed_extended_instruction.html),
/// the C macro `VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_relaxed_extended_instruction")]
#[doc(alias = "VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_EXTENSION_NAME")]
pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_NAME: &CStr =
    c"VK_KHR_shader_relaxed_extended_instruction";
/// The revision of the
/// [`VK_KHR_shader_relaxed_extended_instruction`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_relaxed_extended_instruction.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION")]
pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_subgroup_extended_types`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_extended_types.html),
/// the C macro `VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_shader_subgroup_extended_types")]
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_NAME: &CStr = c"VK_KHR_shader_subgroup_extended_types";
/// The revision of the
/// [`VK_KHR_shader_subgroup_extended_types`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_extended_types.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_subgroup_rotate`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_rotate.html),
/// the C macro `VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_shader_subgroup_rotate")]
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_ROTATE_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_ROTATE_NAME: &CStr = c"VK_KHR_shader_subgroup_rotate";
/// The revision of the
/// [`VK_KHR_shader_subgroup_rotate`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_rotate.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_shader_subgroup_uniform_control_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_uniform_control_flow.html),
/// the C macro `VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_KHR_shader_subgroup_uniform_control_flow")]
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_NAME: &CStr =
    c"VK_KHR_shader_subgroup_uniform_control_flow";
/// The revision of the
/// [`VK_KHR_shader_subgroup_uniform_control_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_subgroup_uniform_control_flow.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION")]
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_terminate_invocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_terminate_invocation.html),
/// the C macro `VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_shader_terminate_invocation")]
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME")]
pub const KHR_SHADER_TERMINATE_INVOCATION_NAME: &CStr = c"VK_KHR_shader_terminate_invocation";
/// The revision of the
/// [`VK_KHR_shader_terminate_invocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_terminate_invocation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION")]
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shader_untyped_pointers`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_untyped_pointers.html),
/// the C macro `VK_KHR_SHADER_UNTYPED_POINTERS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME).
#[doc(alias = "VK_KHR_shader_untyped_pointers")]
#[doc(alias = "VK_KHR_SHADER_UNTYPED_POINTERS_EXTENSION_NAME")]
pub const KHR_SHADER_UNTYPED_POINTERS_NAME: &CStr = c"VK_KHR_shader_untyped_pointers";
/// The revision of the
/// [`VK_KHR_shader_untyped_pointers`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shader_untyped_pointers.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION")]
pub const KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_shared_presentable_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shared_presentable_image.html),
/// the C macro `VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME),
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1).
#[doc(alias = "VK_KHR_shared_presentable_image")]
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_NAME: &CStr = c"VK_KHR_shared_presentable_image";
/// The revision of the
/// [`VK_KHR_shared_presentable_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_shared_presentable_image.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_spirv_1_4`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_spirv_1_4.html),
/// the C macro `VK_KHR_SPIRV_1_4_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1 and [`VK_KHR_shader_float_controls`](KHR_SHADER_FLOAT_CONTROLS_NAME).
/// Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_spirv_1_4")]
#[doc(alias = "VK_KHR_SPIRV_1_4_EXTENSION_NAME")]
pub const KHR_SPIRV_1_4_NAME: &CStr = c"VK_KHR_spirv_1_4";
/// The revision of the
/// [`VK_KHR_spirv_1_4`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_spirv_1_4.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SPIRV_1_4_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SPIRV_1_4_SPEC_VERSION")]
pub const KHR_SPIRV_1_4_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_storage_buffer_storage_class`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_storage_buffer_storage_class.html),
/// the C macro `VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME`.
///
/// Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_storage_buffer_storage_class")]
#[doc(alias = "VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME")]
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME: &CStr = c"VK_KHR_storage_buffer_storage_class";
/// The revision of the
/// [`VK_KHR_storage_buffer_storage_class`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_storage_buffer_storage_class.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION")]
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface.html),
/// the C macro `VK_KHR_SURFACE_EXTENSION_NAME`.
#[doc(alias = "VK_KHR_surface")]
#[doc(alias = "VK_KHR_SURFACE_EXTENSION_NAME")]
pub const KHR_SURFACE_NAME: &CStr = c"VK_KHR_surface";
/// The revision of the
/// [`VK_KHR_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SURFACE_SPEC_VERSION")]
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;

/// The name of the instance extension
/// [`VK_KHR_surface_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_maintenance1.html),
/// the C macro `VK_KHR_SURFACE_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME) and
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME).
#[doc(alias = "VK_KHR_surface_maintenance1")]
#[doc(alias = "VK_KHR_SURFACE_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_SURFACE_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_surface_maintenance1";
/// The revision of the
/// [`VK_KHR_surface_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_surface_protected_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_protected_capabilities.html),
/// the C macro `VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1 and
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME).
#[doc(alias = "VK_KHR_surface_protected_capabilities")]
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_NAME: &CStr = c"VK_KHR_surface_protected_capabilities";
/// The revision of the
/// [`VK_KHR_surface_protected_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface_protected_capabilities.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION")]
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain.html),
/// the C macro `VK_KHR_SWAPCHAIN_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_swapchain")]
#[doc(alias = "VK_KHR_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_NAME: &CStr = c"VK_KHR_swapchain";
/// The revision of the
/// [`VK_KHR_swapchain`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SWAPCHAIN_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;

/// The name of the device extension
/// [`VK_KHR_swapchain_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_maintenance1.html),
/// the C macro `VK_KHR_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME),
/// [`VK_KHR_surface_maintenance1`](KHR_SURFACE_MAINTENANCE_1_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1).
#[doc(alias = "VK_KHR_swapchain_maintenance1")]
#[doc(alias = "VK_KHR_SWAPCHAIN_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_swapchain_maintenance1";
/// The revision of the
/// [`VK_KHR_swapchain_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_swapchain_mutable_format`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_mutable_format.html),
/// the C macro `VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME),
/// ([`VK_KHR_maintenance2`](KHR_MAINTENANCE_2_NAME) or Vulkan 1.1) and
/// ([`VK_KHR_image_format_list`](KHR_IMAGE_FORMAT_LIST_NAME) or Vulkan 1.2).
#[doc(alias = "VK_KHR_swapchain_mutable_format")]
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_NAME: &CStr = c"VK_KHR_swapchain_mutable_format";
/// The revision of the
/// [`VK_KHR_swapchain_mutable_format`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain_mutable_format.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION")]
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_synchronization2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_synchronization2.html),
/// the C macro `VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_synchronization2")]
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_NAME: &CStr = c"VK_KHR_synchronization2";
/// The revision of the
/// [`VK_KHR_synchronization2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_synchronization2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_timeline_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_timeline_semaphore.html),
/// the C macro `VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_timeline_semaphore")]
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_TIMELINE_SEMAPHORE_NAME: &CStr = c"VK_KHR_timeline_semaphore";
/// The revision of the
/// [`VK_KHR_timeline_semaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_timeline_semaphore.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION")]
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_unified_image_layouts`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_unified_image_layouts.html),
/// the C macro `VK_KHR_UNIFIED_IMAGE_LAYOUTS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_unified_image_layouts")]
#[doc(alias = "VK_KHR_UNIFIED_IMAGE_LAYOUTS_EXTENSION_NAME")]
pub const KHR_UNIFIED_IMAGE_LAYOUTS_NAME: &CStr = c"VK_KHR_unified_image_layouts";
/// The revision of the
/// [`VK_KHR_unified_image_layouts`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_unified_image_layouts.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION")]
pub const KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_uniform_buffer_standard_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_uniform_buffer_standard_layout.html),
/// the C macro `VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_uniform_buffer_standard_layout")]
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_NAME: &CStr = c"VK_KHR_uniform_buffer_standard_layout";
/// The revision of the
/// [`VK_KHR_uniform_buffer_standard_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_uniform_buffer_standard_layout.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION")]
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_variable_pointers`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_variable_pointers.html),
/// the C macro `VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_storage_buffer_storage_class`](KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME)) or Vulkan
/// 1.1. Promoted to Vulkan 1.1.
#[doc(alias = "VK_KHR_variable_pointers")]
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME")]
pub const KHR_VARIABLE_POINTERS_NAME: &CStr = c"VK_KHR_variable_pointers";
/// The revision of the
/// [`VK_KHR_variable_pointers`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_variable_pointers.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VARIABLE_POINTERS_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_SPEC_VERSION")]
pub const KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_vertex_attribute_divisor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vertex_attribute_divisor.html),
/// the C macro `VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.4.
#[doc(alias = "VK_KHR_vertex_attribute_divisor")]
#[doc(alias = "VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME")]
pub const KHR_VERTEX_ATTRIBUTE_DIVISOR_NAME: &CStr = c"VK_KHR_vertex_attribute_divisor";
/// The revision of the
/// [`VK_KHR_vertex_attribute_divisor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vertex_attribute_divisor.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION")]
pub const KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_decode_av1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_av1.html),
/// the C macro `VK_KHR_VIDEO_DECODE_AV1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_decode_queue`](KHR_VIDEO_DECODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_decode_av1")]
#[doc(alias = "VK_KHR_VIDEO_DECODE_AV1_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_AV1_NAME: &CStr = c"VK_KHR_video_decode_av1";
/// The revision of the
/// [`VK_KHR_video_decode_av1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_av1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_DECODE_AV1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_DECODE_AV1_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_AV1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_decode_h264`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h264.html),
/// the C macro `VK_KHR_VIDEO_DECODE_H264_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_decode_queue`](KHR_VIDEO_DECODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_decode_h264")]
#[doc(alias = "VK_KHR_VIDEO_DECODE_H264_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_H264_NAME: &CStr = c"VK_KHR_video_decode_h264";
/// The revision of the
/// [`VK_KHR_video_decode_h264`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h264.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_DECODE_H264_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_DECODE_H264_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 9;

/// The name of the device extension
/// [`VK_KHR_video_decode_h265`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h265.html),
/// the C macro `VK_KHR_VIDEO_DECODE_H265_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_decode_queue`](KHR_VIDEO_DECODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_decode_h265")]
#[doc(alias = "VK_KHR_VIDEO_DECODE_H265_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_H265_NAME: &CStr = c"VK_KHR_video_decode_h265";
/// The revision of the
/// [`VK_KHR_video_decode_h265`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_h265.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_DECODE_H265_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_DECODE_H265_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 8;

/// The name of the device extension
/// [`VK_KHR_video_decode_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_queue.html),
/// the C macro `VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME) and
/// ([`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME) or Vulkan 1.3).
#[doc(alias = "VK_KHR_video_decode_queue")]
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_QUEUE_NAME: &CStr = c"VK_KHR_video_decode_queue";
/// The revision of the
/// [`VK_KHR_video_decode_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_queue.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 8;

/// The name of the device extension
/// [`VK_KHR_video_decode_vp9`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_vp9.html),
/// the C macro `VK_KHR_VIDEO_DECODE_VP9_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_decode_queue`](KHR_VIDEO_DECODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_decode_vp9")]
#[doc(alias = "VK_KHR_VIDEO_DECODE_VP9_EXTENSION_NAME")]
pub const KHR_VIDEO_DECODE_VP9_NAME: &CStr = c"VK_KHR_video_decode_vp9";
/// The revision of the
/// [`VK_KHR_video_decode_vp9`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_decode_vp9.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_DECODE_VP9_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_DECODE_VP9_SPEC_VERSION")]
pub const KHR_VIDEO_DECODE_VP9_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_encode_av1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_av1.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_AV1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_encode_av1")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_AV1_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_AV1_NAME: &CStr = c"VK_KHR_video_encode_av1";
/// The revision of the
/// [`VK_KHR_video_encode_av1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_av1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_AV1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_AV1_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_AV1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_encode_feedback2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_feedback2.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_FEEDBACK_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_encode_feedback2")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_FEEDBACK_2_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_FEEDBACK_2_NAME: &CStr = c"VK_KHR_video_encode_feedback2";
/// The revision of the
/// [`VK_KHR_video_encode_feedback2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_feedback2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_FEEDBACK_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_FEEDBACK_2_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_FEEDBACK_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_encode_h264`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h264.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_H264_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_encode_h264")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_H264_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_H264_NAME: &CStr = c"VK_KHR_video_encode_h264";
/// The revision of the
/// [`VK_KHR_video_encode_h264`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h264.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_H264_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_H264_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 14;

/// The name of the device extension
/// [`VK_KHR_video_encode_h265`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h265.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_H265_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_encode_h265")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_H265_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_H265_NAME: &CStr = c"VK_KHR_video_encode_h265";
/// The revision of the
/// [`VK_KHR_video_encode_h265`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_h265.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_H265_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_H265_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 14;

/// The name of the device extension
/// [`VK_KHR_video_encode_intra_refresh`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_intra_refresh.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_encode_intra_refresh")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_INTRA_REFRESH_NAME: &CStr = c"VK_KHR_video_encode_intra_refresh";
/// The revision of the
/// [`VK_KHR_video_encode_intra_refresh`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_intra_refresh.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_encode_quantization_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_quantization_map.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME) and
/// ([`VK_KHR_format_feature_flags2`](KHR_FORMAT_FEATURE_FLAGS_2_NAME) or Vulkan 1.3).
#[doc(alias = "VK_KHR_video_encode_quantization_map")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUANTIZATION_MAP_NAME: &CStr = c"VK_KHR_video_encode_quantization_map";
/// The revision of the
/// [`VK_KHR_video_encode_quantization_map`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_quantization_map.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_KHR_video_encode_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_queue.html),
/// the C macro `VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME) and
/// ([`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME) or Vulkan 1.3).
#[doc(alias = "VK_KHR_video_encode_queue")]
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_ENCODE_QUEUE_NAME: &CStr = c"VK_KHR_video_encode_queue";
/// The revision of the
/// [`VK_KHR_video_encode_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_encode_queue.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 12;

/// The name of the device extension
/// [`VK_KHR_video_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance1.html),
/// the C macro `VK_KHR_VIDEO_MAINTENANCE_1_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_maintenance1")]
#[doc(alias = "VK_KHR_VIDEO_MAINTENANCE_1_EXTENSION_NAME")]
pub const KHR_VIDEO_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_video_maintenance1";
/// The revision of the
/// [`VK_KHR_video_maintenance1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance1.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION")]
pub const KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_maintenance2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance2.html),
/// the C macro `VK_KHR_VIDEO_MAINTENANCE_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME).
#[doc(alias = "VK_KHR_video_maintenance2")]
#[doc(alias = "VK_KHR_VIDEO_MAINTENANCE_2_EXTENSION_NAME")]
pub const KHR_VIDEO_MAINTENANCE_2_NAME: &CStr = c"VK_KHR_video_maintenance2";
/// The revision of the
/// [`VK_KHR_video_maintenance2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_maintenance2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION")]
pub const KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_KHR_video_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_queue.html),
/// the C macro `VK_KHR_VIDEO_QUEUE_EXTENSION_NAME`.
///
/// Requires (Vulkan 1.1 and [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME)) or Vulkan
/// 1.3.
#[doc(alias = "VK_KHR_video_queue")]
#[doc(alias = "VK_KHR_VIDEO_QUEUE_EXTENSION_NAME")]
pub const KHR_VIDEO_QUEUE_NAME: &CStr = c"VK_KHR_video_queue";
/// The revision of the
/// [`VK_KHR_video_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_video_queue.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VIDEO_QUEUE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VIDEO_QUEUE_SPEC_VERSION")]
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 8;

/// The name of the device extension
/// [`VK_KHR_vulkan_memory_model`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vulkan_memory_model.html),
/// the C macro `VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.2.
#[doc(alias = "VK_KHR_vulkan_memory_model")]
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME")]
pub const KHR_VULKAN_MEMORY_MODEL_NAME: &CStr = c"VK_KHR_vulkan_memory_model";
/// The revision of the
/// [`VK_KHR_vulkan_memory_model`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_vulkan_memory_model.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION`.
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION")]
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;

/// The name of the instance extension
/// [`VK_KHR_wayland_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_wayland_surface.html),
/// the C macro `VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_wayland_surface")]
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "wayland")]
pub const KHR_WAYLAND_SURFACE_NAME: &CStr = c"VK_KHR_wayland_surface";
/// The revision of the
/// [`VK_KHR_wayland_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_wayland_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_WAYLAND_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_WAYLAND_SURFACE_SPEC_VERSION")]
#[cfg(feature = "wayland")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;

/// The name of the device extension
/// [`VK_KHR_win32_keyed_mutex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_keyed_mutex.html),
/// the C macro `VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory_win32`](KHR_EXTERNAL_MEMORY_WIN32_NAME).
#[doc(alias = "VK_KHR_win32_keyed_mutex")]
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const KHR_WIN32_KEYED_MUTEX_NAME: &CStr = c"VK_KHR_win32_keyed_mutex";
/// The revision of the
/// [`VK_KHR_win32_keyed_mutex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_keyed_mutex.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION`.
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_win32_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_surface.html),
/// the C macro `VK_KHR_WIN32_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_win32_surface")]
#[doc(alias = "VK_KHR_WIN32_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const KHR_WIN32_SURFACE_NAME: &CStr = c"VK_KHR_win32_surface";
/// The revision of the
/// [`VK_KHR_win32_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_WIN32_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_WIN32_SURFACE_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;

/// The name of the device extension
/// [`VK_KHR_workgroup_memory_explicit_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_workgroup_memory_explicit_layout.html),
/// the C macro `VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_KHR_workgroup_memory_explicit_layout")]
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_NAME: &CStr =
    c"VK_KHR_workgroup_memory_explicit_layout";
/// The revision of the
/// [`VK_KHR_workgroup_memory_explicit_layout`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_workgroup_memory_explicit_layout.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION`.
#[doc(alias = "VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION")]
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_KHR_xcb_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xcb_surface.html),
/// the C macro `VK_KHR_XCB_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_xcb_surface")]
#[doc(alias = "VK_KHR_XCB_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "xcb")]
pub const KHR_XCB_SURFACE_NAME: &CStr = c"VK_KHR_xcb_surface";
/// The revision of the
/// [`VK_KHR_xcb_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xcb_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_XCB_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_XCB_SURFACE_SPEC_VERSION")]
#[cfg(feature = "xcb")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;

/// The name of the instance extension
/// [`VK_KHR_xlib_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xlib_surface.html),
/// the C macro `VK_KHR_XLIB_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_KHR_xlib_surface")]
#[doc(alias = "VK_KHR_XLIB_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "xlib")]
pub const KHR_XLIB_SURFACE_NAME: &CStr = c"VK_KHR_xlib_surface";
/// The revision of the
/// [`VK_KHR_xlib_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xlib_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_XLIB_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_KHR_XLIB_SURFACE_SPEC_VERSION")]
#[cfg(feature = "xlib")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;

/// The name of the device extension
/// [`VK_KHR_zero_initialize_workgroup_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_zero_initialize_workgroup_memory.html),
/// the C macro `VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to Vulkan 1.3.
#[doc(alias = "VK_KHR_zero_initialize_workgroup_memory")]
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_NAME: &CStr =
    c"VK_KHR_zero_initialize_workgroup_memory";
/// The revision of the
/// [`VK_KHR_zero_initialize_workgroup_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_zero_initialize_workgroup_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION")]
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_LUNARG_direct_driver_loading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUNARG_direct_driver_loading.html),
/// the C macro `VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME`.
#[doc(alias = "VK_LUNARG_direct_driver_loading")]
#[doc(alias = "VK_LUNARG_DIRECT_DRIVER_LOADING_EXTENSION_NAME")]
pub const LUNARG_DIRECT_DRIVER_LOADING_NAME: &CStr = c"VK_LUNARG_direct_driver_loading";
/// The revision of the
/// [`VK_LUNARG_direct_driver_loading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_LUNARG_direct_driver_loading.html)
/// extension that these bindings were generated from, the C macro
/// `VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION`.
#[doc(alias = "VK_LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION")]
pub const LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_MESA_image_alignment_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MESA_image_alignment_control.html),
/// the C macro `VK_MESA_IMAGE_ALIGNMENT_CONTROL_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_MESA_image_alignment_control")]
#[doc(alias = "VK_MESA_IMAGE_ALIGNMENT_CONTROL_EXTENSION_NAME")]
pub const MESA_IMAGE_ALIGNMENT_CONTROL_NAME: &CStr = c"VK_MESA_image_alignment_control";
/// The revision of the
/// [`VK_MESA_image_alignment_control`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MESA_image_alignment_control.html)
/// extension that these bindings were generated from, the C macro
/// `VK_MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION`.
#[doc(alias = "VK_MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION")]
pub const MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_MSFT_layered_driver`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MSFT_layered_driver.html),
/// the C macro `VK_MSFT_LAYERED_DRIVER_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_MSFT_layered_driver")]
#[doc(alias = "VK_MSFT_LAYERED_DRIVER_EXTENSION_NAME")]
pub const MSFT_LAYERED_DRIVER_NAME: &CStr = c"VK_MSFT_layered_driver";
/// The revision of the
/// [`VK_MSFT_layered_driver`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MSFT_layered_driver.html)
/// extension that these bindings were generated from, the C macro
/// `VK_MSFT_LAYERED_DRIVER_SPEC_VERSION`.
#[doc(alias = "VK_MSFT_LAYERED_DRIVER_SPEC_VERSION")]
pub const MSFT_LAYERED_DRIVER_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_MVK_ios_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_ios_surface.html),
/// the C macro `VK_MVK_IOS_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME). Deprecated by `VK_EXT_metal_surface`.
#[doc(alias = "VK_MVK_ios_surface")]
#[doc(alias = "VK_MVK_IOS_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "ios")]
pub const MVK_IOS_SURFACE_NAME: &CStr = c"VK_MVK_ios_surface";
/// The revision of the
/// [`VK_MVK_ios_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_ios_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_MVK_IOS_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_MVK_IOS_SURFACE_SPEC_VERSION")]
#[cfg(feature = "ios")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;

/// The name of the instance extension
/// [`VK_MVK_macos_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_macos_surface.html),
/// the C macro `VK_MVK_MACOS_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME). Deprecated by `VK_EXT_metal_surface`.
#[doc(alias = "VK_MVK_macos_surface")]
#[doc(alias = "VK_MVK_MACOS_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "macos")]
pub const MVK_MACOS_SURFACE_NAME: &CStr = c"VK_MVK_macos_surface";
/// The revision of the
/// [`VK_MVK_macos_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_MVK_macos_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_MVK_MACOS_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_MVK_MACOS_SURFACE_SPEC_VERSION")]
#[cfg(feature = "macos")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;

/// The name of the instance extension
/// [`VK_NN_vi_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NN_vi_surface.html),
/// the C macro `VK_NN_VI_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_NN_vi_surface")]
#[doc(alias = "VK_NN_VI_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "vi")]
pub const NN_VI_SURFACE_NAME: &CStr = c"VK_NN_vi_surface";
/// The revision of the
/// [`VK_NN_vi_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NN_vi_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NN_VI_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_NN_VI_SURFACE_SPEC_VERSION")]
#[cfg(feature = "vi")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NVX_binary_import`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_binary_import.html),
/// the C macro `VK_NVX_BINARY_IMPORT_EXTENSION_NAME`.
#[doc(alias = "VK_NVX_binary_import")]
#[doc(alias = "VK_NVX_BINARY_IMPORT_EXTENSION_NAME")]
pub const NVX_BINARY_IMPORT_NAME: &CStr = c"VK_NVX_binary_import";
/// The revision of the
/// [`VK_NVX_binary_import`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_binary_import.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NVX_BINARY_IMPORT_SPEC_VERSION`.
#[doc(alias = "VK_NVX_BINARY_IMPORT_SPEC_VERSION")]
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NVX_image_view_handle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_image_view_handle.html),
/// the C macro `VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME`.
#[doc(alias = "VK_NVX_image_view_handle")]
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME")]
pub const NVX_IMAGE_VIEW_HANDLE_NAME: &CStr = c"VK_NVX_image_view_handle";
/// The revision of the
/// [`VK_NVX_image_view_handle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_image_view_handle.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION`.
#[doc(alias = "VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION")]
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_NVX_multiview_per_view_attributes`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_multiview_per_view_attributes.html),
/// the C macro `VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_multiview`](KHR_MULTIVIEW_NAME) or Vulkan 1.1.
#[doc(alias = "VK_NVX_multiview_per_view_attributes")]
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_NAME: &CStr = c"VK_NVX_multiview_per_view_attributes";
/// The revision of the
/// [`VK_NVX_multiview_per_view_attributes`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NVX_multiview_per_view_attributes.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION`.
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_acquire_winrt_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_acquire_winrt_display.html),
/// the C macro `VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_direct_mode_display`](EXT_DIRECT_MODE_DISPLAY_NAME).
#[doc(alias = "VK_NV_acquire_winrt_display")]
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const NV_ACQUIRE_WINRT_DISPLAY_NAME: &CStr = c"VK_NV_acquire_winrt_display";
/// The revision of the
/// [`VK_NV_acquire_winrt_display`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_acquire_winrt_display.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION`.
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_clip_space_w_scaling`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_clip_space_w_scaling.html),
/// the C macro `VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME`.
#[doc(alias = "VK_NV_clip_space_w_scaling")]
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME")]
pub const NV_CLIP_SPACE_W_SCALING_NAME: &CStr = c"VK_NV_clip_space_w_scaling";
/// The revision of the
/// [`VK_NV_clip_space_w_scaling`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_clip_space_w_scaling.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION`.
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION")]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_cluster_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cluster_acceleration_structure.html),
/// the C macro `VK_NV_CLUSTER_ACCELERATION_STRUCTURE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_NV_cluster_acceleration_structure")]
#[doc(alias = "VK_NV_CLUSTER_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const NV_CLUSTER_ACCELERATION_STRUCTURE_NAME: &CStr = c"VK_NV_cluster_acceleration_structure";
/// The revision of the
/// [`VK_NV_cluster_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cluster_acceleration_structure.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION`.
#[doc(alias = "VK_NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_NV_command_buffer_inheritance`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_command_buffer_inheritance.html),
/// the C macro `VK_NV_COMMAND_BUFFER_INHERITANCE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_command_buffer_inheritance")]
#[doc(alias = "VK_NV_COMMAND_BUFFER_INHERITANCE_EXTENSION_NAME")]
pub const NV_COMMAND_BUFFER_INHERITANCE_NAME: &CStr = c"VK_NV_command_buffer_inheritance";
/// The revision of the
/// [`VK_NV_command_buffer_inheritance`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_command_buffer_inheritance.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION`.
#[doc(alias = "VK_NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION")]
pub const NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_compute_occupancy_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_occupancy_priority.html),
/// the C macro `VK_NV_COMPUTE_OCCUPANCY_PRIORITY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_compute_occupancy_priority")]
#[doc(alias = "VK_NV_COMPUTE_OCCUPANCY_PRIORITY_EXTENSION_NAME")]
pub const NV_COMPUTE_OCCUPANCY_PRIORITY_NAME: &CStr = c"VK_NV_compute_occupancy_priority";
/// The revision of the
/// [`VK_NV_compute_occupancy_priority`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_occupancy_priority.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION`.
#[doc(alias = "VK_NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION")]
pub const NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_compute_shader_derivatives`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_shader_derivatives.html),
/// the C macro `VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to
/// [`VK_KHR_compute_shader_derivatives`](KHR_COMPUTE_SHADER_DERIVATIVES_NAME).
#[doc(alias = "VK_NV_compute_shader_derivatives")]
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_NAME: &CStr = c"VK_NV_compute_shader_derivatives";
/// The revision of the
/// [`VK_NV_compute_shader_derivatives`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_compute_shader_derivatives.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION`.
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_cooperative_matrix`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix.html),
/// the C macro `VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_cooperative_matrix")]
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_NAME: &CStr = c"VK_NV_cooperative_matrix";
/// The revision of the
/// [`VK_NV_cooperative_matrix`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION`.
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_cooperative_matrix2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix2.html),
/// the C macro `VK_NV_COOPERATIVE_MATRIX_2_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_cooperative_matrix`](KHR_COOPERATIVE_MATRIX_NAME).
#[doc(alias = "VK_NV_cooperative_matrix2")]
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_2_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_2_NAME: &CStr = c"VK_NV_cooperative_matrix2";
/// The revision of the
/// [`VK_NV_cooperative_matrix2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COOPERATIVE_MATRIX_2_SPEC_VERSION`.
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_2_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_cooperative_matrix_decode_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix_decode_vector.html),
/// the C macro `VK_NV_COOPERATIVE_MATRIX_DECODE_VECTOR_EXTENSION_NAME`.
///
/// Requires [`VK_NV_cooperative_matrix2`](NV_COOPERATIVE_MATRIX_2_NAME).
#[doc(alias = "VK_NV_cooperative_matrix_decode_vector")]
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_DECODE_VECTOR_EXTENSION_NAME")]
pub const NV_COOPERATIVE_MATRIX_DECODE_VECTOR_NAME: &CStr =
    c"VK_NV_cooperative_matrix_decode_vector";
/// The revision of the
/// [`VK_NV_cooperative_matrix_decode_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_matrix_decode_vector.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COOPERATIVE_MATRIX_DECODE_VECTOR_SPEC_VERSION`.
#[doc(alias = "VK_NV_COOPERATIVE_MATRIX_DECODE_VECTOR_SPEC_VERSION")]
pub const NV_COOPERATIVE_MATRIX_DECODE_VECTOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_cooperative_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_vector.html),
/// the C macro `VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_cooperative_vector")]
#[doc(alias = "VK_NV_COOPERATIVE_VECTOR_EXTENSION_NAME")]
pub const NV_COOPERATIVE_VECTOR_NAME: &CStr = c"VK_NV_cooperative_vector";
/// The revision of the
/// [`VK_NV_cooperative_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cooperative_vector.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COOPERATIVE_VECTOR_SPEC_VERSION`.
#[doc(alias = "VK_NV_COOPERATIVE_VECTOR_SPEC_VERSION")]
pub const NV_COOPERATIVE_VECTOR_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_NV_copy_memory_indirect`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_copy_memory_indirect.html),
/// the C macro `VK_NV_COPY_MEMORY_INDIRECT_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME)) or Vulkan
/// 1.2. Promoted to [`VK_KHR_copy_memory_indirect`](KHR_COPY_MEMORY_INDIRECT_NAME).
#[doc(alias = "VK_NV_copy_memory_indirect")]
#[doc(alias = "VK_NV_COPY_MEMORY_INDIRECT_EXTENSION_NAME")]
pub const NV_COPY_MEMORY_INDIRECT_NAME: &CStr = c"VK_NV_copy_memory_indirect";
/// The revision of the
/// [`VK_NV_copy_memory_indirect`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_copy_memory_indirect.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COPY_MEMORY_INDIRECT_SPEC_VERSION`.
#[doc(alias = "VK_NV_COPY_MEMORY_INDIRECT_SPEC_VERSION")]
pub const NV_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_corner_sampled_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_corner_sampled_image.html),
/// the C macro `VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_corner_sampled_image")]
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME")]
pub const NV_CORNER_SAMPLED_IMAGE_NAME: &CStr = c"VK_NV_corner_sampled_image";
/// The revision of the
/// [`VK_NV_corner_sampled_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_corner_sampled_image.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION`.
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION")]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_coverage_reduction_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_coverage_reduction_mode.html),
/// the C macro `VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME`.
///
/// Requires [`VK_NV_framebuffer_mixed_samples`](NV_FRAMEBUFFER_MIXED_SAMPLES_NAME) and
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1).
#[doc(alias = "VK_NV_coverage_reduction_mode")]
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME")]
pub const NV_COVERAGE_REDUCTION_MODE_NAME: &CStr = c"VK_NV_coverage_reduction_mode";
/// The revision of the
/// [`VK_NV_coverage_reduction_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_coverage_reduction_mode.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION`.
#[doc(alias = "VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION")]
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;

/// The name of the provisional device extension
/// [`VK_NV_cuda_kernel_launch`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cuda_kernel_launch.html),
/// the C macro `VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_cuda_kernel_launch")]
#[doc(alias = "VK_NV_CUDA_KERNEL_LAUNCH_EXTENSION_NAME")]
#[cfg(feature = "beta")]
pub const NV_CUDA_KERNEL_LAUNCH_NAME: &CStr = c"VK_NV_cuda_kernel_launch";
/// The revision of the
/// [`VK_NV_cuda_kernel_launch`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_cuda_kernel_launch.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION`.
#[doc(alias = "VK_NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION")]
#[cfg(feature = "beta")]
pub const NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_dedicated_allocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_dedicated_allocation.html),
/// the C macro `VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME`.
///
/// Deprecated by [`VK_KHR_dedicated_allocation`](KHR_DEDICATED_ALLOCATION_NAME).
#[doc(alias = "VK_NV_dedicated_allocation")]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_NAME: &CStr = c"VK_NV_dedicated_allocation";
/// The revision of the
/// [`VK_NV_dedicated_allocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_dedicated_allocation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_dedicated_allocation_image_aliasing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_dedicated_allocation_image_aliasing.html),
/// the C macro `VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_dedicated_allocation`](KHR_DEDICATED_ALLOCATION_NAME) and
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME)) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_dedicated_allocation_image_aliasing")]
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_NAME: &CStr =
    c"VK_NV_dedicated_allocation_image_aliasing";
/// The revision of the
/// [`VK_NV_dedicated_allocation_image_aliasing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_dedicated_allocation_image_aliasing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION")]
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_descriptor_pool_overallocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_descriptor_pool_overallocation.html),
/// the C macro `VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_NV_descriptor_pool_overallocation")]
#[doc(alias = "VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_EXTENSION_NAME")]
pub const NV_DESCRIPTOR_POOL_OVERALLOCATION_NAME: &CStr = c"VK_NV_descriptor_pool_overallocation";
/// The revision of the
/// [`VK_NV_descriptor_pool_overallocation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_descriptor_pool_overallocation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION`.
#[doc(alias = "VK_NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION")]
pub const NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_device_diagnostic_checkpoints`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostic_checkpoints.html),
/// the C macro `VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_device_diagnostic_checkpoints")]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_NAME: &CStr = c"VK_NV_device_diagnostic_checkpoints";
/// The revision of the
/// [`VK_NV_device_diagnostic_checkpoints`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostic_checkpoints.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_device_diagnostics_config`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostics_config.html),
/// the C macro `VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_device_diagnostics_config")]
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_NAME: &CStr = c"VK_NV_device_diagnostics_config";
/// The revision of the
/// [`VK_NV_device_diagnostics_config`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_diagnostics_config.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_device_generated_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands.html),
/// the C macro `VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME`.
///
/// Requires (Vulkan 1.1 and [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME))
/// or Vulkan 1.2.
#[doc(alias = "VK_NV_device_generated_commands")]
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_NAME: &CStr = c"VK_NV_device_generated_commands";
/// The revision of the
/// [`VK_NV_device_generated_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_NV_device_generated_commands_compute`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands_compute.html),
/// the C macro `VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_EXTENSION_NAME`.
///
/// Requires [`VK_NV_device_generated_commands`](NV_DEVICE_GENERATED_COMMANDS_NAME).
#[doc(alias = "VK_NV_device_generated_commands_compute")]
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_EXTENSION_NAME")]
pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE_NAME: &CStr =
    c"VK_NV_device_generated_commands_compute";
/// The revision of the
/// [`VK_NV_device_generated_commands_compute`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_device_generated_commands_compute.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION`.
#[doc(alias = "VK_NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION")]
pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION: u32 = 2;

/// The name of the provisional device extension
/// [`VK_NV_displacement_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_displacement_micromap.html),
/// the C macro `VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_opacity_micromap`](EXT_OPACITY_MICROMAP_NAME). Deprecated by
/// [`VK_NV_cluster_acceleration_structure`](NV_CLUSTER_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_NV_displacement_micromap")]
#[doc(alias = "VK_NV_DISPLACEMENT_MICROMAP_EXTENSION_NAME")]
#[cfg(feature = "beta")]
pub const NV_DISPLACEMENT_MICROMAP_NAME: &CStr = c"VK_NV_displacement_micromap";
/// The revision of the
/// [`VK_NV_displacement_micromap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_displacement_micromap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION`.
#[doc(alias = "VK_NV_DISPLACEMENT_MICROMAP_SPEC_VERSION")]
#[cfg(feature = "beta")]
pub const NV_DISPLACEMENT_MICROMAP_SPEC_VERSION: u32 = 2;

/// The name of the instance extension
/// [`VK_NV_display_stereo`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_display_stereo.html),
/// the C macro `VK_NV_DISPLAY_STEREO_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_display`](KHR_DISPLAY_NAME) and
/// [`VK_KHR_get_display_properties2`](KHR_GET_DISPLAY_PROPERTIES_2_NAME).
#[doc(alias = "VK_NV_display_stereo")]
#[doc(alias = "VK_NV_DISPLAY_STEREO_EXTENSION_NAME")]
pub const NV_DISPLAY_STEREO_NAME: &CStr = c"VK_NV_display_stereo";
/// The revision of the
/// [`VK_NV_display_stereo`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_display_stereo.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_DISPLAY_STEREO_SPEC_VERSION`.
#[doc(alias = "VK_NV_DISPLAY_STEREO_SPEC_VERSION")]
pub const NV_DISPLAY_STEREO_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_extended_sparse_address_space`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_extended_sparse_address_space.html),
/// the C macro `VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_extended_sparse_address_space")]
#[doc(alias = "VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_EXTENSION_NAME")]
pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE_NAME: &CStr = c"VK_NV_extended_sparse_address_space";
/// The revision of the
/// [`VK_NV_extended_sparse_address_space`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_extended_sparse_address_space.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION")]
pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_external_compute_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_compute_queue.html),
/// the C macro `VK_NV_EXTERNAL_COMPUTE_QUEUE_EXTENSION_NAME`.
#[doc(alias = "VK_NV_external_compute_queue")]
#[doc(alias = "VK_NV_EXTERNAL_COMPUTE_QUEUE_EXTENSION_NAME")]
pub const NV_EXTERNAL_COMPUTE_QUEUE_NAME: &CStr = c"VK_NV_external_compute_queue";
/// The revision of the
/// [`VK_NV_external_compute_queue`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_compute_queue.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION")]
pub const NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory.html),
/// the C macro `VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME`.
///
/// Requires [`VK_NV_external_memory_capabilities`](NV_EXTERNAL_MEMORY_CAPABILITIES_NAME).
/// Deprecated by [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME).
#[doc(alias = "VK_NV_external_memory")]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_NAME: &CStr = c"VK_NV_external_memory";
/// The revision of the
/// [`VK_NV_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTERNAL_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_NV_external_memory_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_capabilities.html),
/// the C macro `VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME`.
///
/// Deprecated by
/// [`VK_KHR_external_memory_capabilities`](KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME).
#[doc(alias = "VK_NV_external_memory_capabilities")]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_NAME: &CStr = c"VK_NV_external_memory_capabilities";
/// The revision of the
/// [`VK_NV_external_memory_capabilities`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_capabilities.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_external_memory_rdma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_rdma.html),
/// the C macro `VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) or Vulkan 1.1.
#[doc(alias = "VK_NV_external_memory_rdma")]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_RDMA_NAME: &CStr = c"VK_NV_external_memory_rdma";
/// The revision of the
/// [`VK_NV_external_memory_rdma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_rdma.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_external_memory_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_win32.html),
/// the C macro `VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME`.
///
/// Requires [`VK_NV_external_memory`](NV_EXTERNAL_MEMORY_NAME). Deprecated by
/// [`VK_KHR_external_memory_win32`](KHR_EXTERNAL_MEMORY_WIN32_NAME).
#[doc(alias = "VK_NV_external_memory_win32")]
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const NV_EXTERNAL_MEMORY_WIN32_NAME: &CStr = c"VK_NV_external_memory_win32";
/// The revision of the
/// [`VK_NV_external_memory_win32`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_external_memory_win32.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION`.
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_fill_rectangle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fill_rectangle.html),
/// the C macro `VK_NV_FILL_RECTANGLE_EXTENSION_NAME`.
#[doc(alias = "VK_NV_fill_rectangle")]
#[doc(alias = "VK_NV_FILL_RECTANGLE_EXTENSION_NAME")]
pub const NV_FILL_RECTANGLE_NAME: &CStr = c"VK_NV_fill_rectangle";
/// The revision of the
/// [`VK_NV_fill_rectangle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fill_rectangle.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_FILL_RECTANGLE_SPEC_VERSION`.
#[doc(alias = "VK_NV_FILL_RECTANGLE_SPEC_VERSION")]
pub const NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_fragment_coverage_to_color`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_coverage_to_color.html),
/// the C macro `VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME`.
#[doc(alias = "VK_NV_fragment_coverage_to_color")]
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_NAME: &CStr = c"VK_NV_fragment_coverage_to_color";
/// The revision of the
/// [`VK_NV_fragment_coverage_to_color`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_coverage_to_color.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION`.
#[doc(alias = "VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION")]
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_fragment_shader_barycentric`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shader_barycentric.html),
/// the C macro `VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1. Promoted to
/// [`VK_KHR_fragment_shader_barycentric`](KHR_FRAGMENT_SHADER_BARYCENTRIC_NAME).
#[doc(alias = "VK_NV_fragment_shader_barycentric")]
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_NAME: &CStr = c"VK_NV_fragment_shader_barycentric";
/// The revision of the
/// [`VK_NV_fragment_shader_barycentric`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shader_barycentric.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION`.
#[doc(alias = "VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_fragment_shading_rate_enums`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shading_rate_enums.html),
/// the C macro `VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_fragment_shading_rate`](KHR_FRAGMENT_SHADING_RATE_NAME).
#[doc(alias = "VK_NV_fragment_shading_rate_enums")]
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_NAME: &CStr = c"VK_NV_fragment_shading_rate_enums";
/// The revision of the
/// [`VK_NV_fragment_shading_rate_enums`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_fragment_shading_rate_enums.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION`.
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_framebuffer_mixed_samples`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_framebuffer_mixed_samples.html),
/// the C macro `VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME`.
#[doc(alias = "VK_NV_framebuffer_mixed_samples")]
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_NAME: &CStr = c"VK_NV_framebuffer_mixed_samples";
/// The revision of the
/// [`VK_NV_framebuffer_mixed_samples`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_framebuffer_mixed_samples.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION`.
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_geometry_shader_passthrough`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_geometry_shader_passthrough.html),
/// the C macro `VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME`.
#[doc(alias = "VK_NV_geometry_shader_passthrough")]
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_NAME: &CStr = c"VK_NV_geometry_shader_passthrough";
/// The revision of the
/// [`VK_NV_geometry_shader_passthrough`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_geometry_shader_passthrough.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION`.
#[doc(alias = "VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION")]
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_glsl_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_glsl_shader.html),
/// the C macro `VK_NV_GLSL_SHADER_EXTENSION_NAME`.
///
/// Deprecated.
#[doc(alias = "VK_NV_glsl_shader")]
#[doc(alias = "VK_NV_GLSL_SHADER_EXTENSION_NAME")]
pub const NV_GLSL_SHADER_NAME: &CStr = c"VK_NV_glsl_shader";
/// The revision of the
/// [`VK_NV_glsl_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_glsl_shader.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_GLSL_SHADER_SPEC_VERSION`.
#[doc(alias = "VK_NV_GLSL_SHADER_SPEC_VERSION")]
pub const NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_inherited_viewport_scissor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_inherited_viewport_scissor.html),
/// the C macro `VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_inherited_viewport_scissor")]
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_NAME: &CStr = c"VK_NV_inherited_viewport_scissor";
/// The revision of the
/// [`VK_NV_inherited_viewport_scissor`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_inherited_viewport_scissor.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION`.
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_linear_color_attachment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_linear_color_attachment.html),
/// the C macro `VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_linear_color_attachment")]
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME")]
pub const NV_LINEAR_COLOR_ATTACHMENT_NAME: &CStr = c"VK_NV_linear_color_attachment";
/// The revision of the
/// [`VK_NV_linear_color_attachment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_linear_color_attachment.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION`.
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION")]
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_low_latency`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency.html),
/// the C macro `VK_NV_LOW_LATENCY_EXTENSION_NAME`.
///
/// Deprecated by [`VK_NV_low_latency2`](NV_LOW_LATENCY_2_NAME).
#[doc(alias = "VK_NV_low_latency")]
#[doc(alias = "VK_NV_LOW_LATENCY_EXTENSION_NAME")]
pub const NV_LOW_LATENCY_NAME: &CStr = c"VK_NV_low_latency";
/// The revision of the
/// [`VK_NV_low_latency`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_LOW_LATENCY_SPEC_VERSION`.
#[doc(alias = "VK_NV_LOW_LATENCY_SPEC_VERSION")]
pub const NV_LOW_LATENCY_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_low_latency2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency2.html),
/// the C macro `VK_NV_LOW_LATENCY_2_EXTENSION_NAME`.
///
/// Requires (Vulkan 1.2 or [`VK_KHR_timeline_semaphore`](KHR_TIMELINE_SEMAPHORE_NAME)) and
/// ([`VK_KHR_present_id`](KHR_PRESENT_ID_NAME) or
/// [`VK_KHR_present_id2`](KHR_PRESENT_ID_2_NAME)).
#[doc(alias = "VK_NV_low_latency2")]
#[doc(alias = "VK_NV_LOW_LATENCY_2_EXTENSION_NAME")]
pub const NV_LOW_LATENCY_2_NAME: &CStr = c"VK_NV_low_latency2";
/// The revision of the
/// [`VK_NV_low_latency2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_low_latency2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_LOW_LATENCY_2_SPEC_VERSION`.
#[doc(alias = "VK_NV_LOW_LATENCY_2_SPEC_VERSION")]
pub const NV_LOW_LATENCY_2_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_NV_memory_decompression`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_memory_decompression.html),
/// the C macro `VK_NV_MEMORY_DECOMPRESSION_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME)) or Vulkan
/// 1.2. Promoted to [`VK_EXT_memory_decompression`](EXT_MEMORY_DECOMPRESSION_NAME).
#[doc(alias = "VK_NV_memory_decompression")]
#[doc(alias = "VK_NV_MEMORY_DECOMPRESSION_EXTENSION_NAME")]
pub const NV_MEMORY_DECOMPRESSION_NAME: &CStr = c"VK_NV_memory_decompression";
/// The revision of the
/// [`VK_NV_memory_decompression`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_memory_decompression.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_MEMORY_DECOMPRESSION_SPEC_VERSION`.
#[doc(alias = "VK_NV_MEMORY_DECOMPRESSION_SPEC_VERSION")]
pub const NV_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_mesh_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_mesh_shader.html),
/// the C macro `VK_NV_MESH_SHADER_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_mesh_shader")]
#[doc(alias = "VK_NV_MESH_SHADER_EXTENSION_NAME")]
pub const NV_MESH_SHADER_NAME: &CStr = c"VK_NV_mesh_shader";
/// The revision of the
/// [`VK_NV_mesh_shader`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_mesh_shader.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_MESH_SHADER_SPEC_VERSION`.
#[doc(alias = "VK_NV_MESH_SHADER_SPEC_VERSION")]
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_optical_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_optical_flow.html),
/// the C macro `VK_NV_OPTICAL_FLOW_EXTENSION_NAME`.
///
/// Requires
/// (([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_format_feature_flags2`](KHR_FORMAT_FEATURE_FLAGS_2_NAME) and
/// [`VK_KHR_synchronization2`](KHR_SYNCHRONIZATION_2_NAME)) or Vulkan 1.3.
#[doc(alias = "VK_NV_optical_flow")]
#[doc(alias = "VK_NV_OPTICAL_FLOW_EXTENSION_NAME")]
pub const NV_OPTICAL_FLOW_NAME: &CStr = c"VK_NV_optical_flow";
/// The revision of the
/// [`VK_NV_optical_flow`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_optical_flow.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_OPTICAL_FLOW_SPEC_VERSION`.
#[doc(alias = "VK_NV_OPTICAL_FLOW_SPEC_VERSION")]
pub const NV_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_partitioned_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_partitioned_acceleration_structure.html),
/// the C macro `VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
#[doc(alias = "VK_NV_partitioned_acceleration_structure")]
#[doc(alias = "VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const NV_PARTITIONED_ACCELERATION_STRUCTURE_NAME: &CStr =
    c"VK_NV_partitioned_acceleration_structure";
/// The revision of the
/// [`VK_NV_partitioned_acceleration_structure`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_partitioned_acceleration_structure.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION`.
#[doc(alias = "VK_NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_per_stage_descriptor_set`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_per_stage_descriptor_set.html),
/// the C macro `VK_NV_PER_STAGE_DESCRIPTOR_SET_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_maintenance6`](KHR_MAINTENANCE_6_NAME) or Vulkan 1.4. Deprecated by
/// [`VK_EXT_descriptor_heap`](EXT_DESCRIPTOR_HEAP_NAME).
#[doc(alias = "VK_NV_per_stage_descriptor_set")]
#[doc(alias = "VK_NV_PER_STAGE_DESCRIPTOR_SET_EXTENSION_NAME")]
pub const NV_PER_STAGE_DESCRIPTOR_SET_NAME: &CStr = c"VK_NV_per_stage_descriptor_set";
/// The revision of the
/// [`VK_NV_per_stage_descriptor_set`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_per_stage_descriptor_set.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION`.
#[doc(alias = "VK_NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION")]
pub const NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_present_barrier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_barrier.html),
/// the C macro `VK_NV_PRESENT_BARRIER_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1), [`VK_KHR_surface`](KHR_SURFACE_NAME),
/// [`VK_KHR_get_surface_capabilities2`](KHR_GET_SURFACE_CAPABILITIES_2_NAME) and
/// [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
#[doc(alias = "VK_NV_present_barrier")]
#[doc(alias = "VK_NV_PRESENT_BARRIER_EXTENSION_NAME")]
pub const NV_PRESENT_BARRIER_NAME: &CStr = c"VK_NV_present_barrier";
/// The revision of the
/// [`VK_NV_present_barrier`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_barrier.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PRESENT_BARRIER_SPEC_VERSION`.
#[doc(alias = "VK_NV_PRESENT_BARRIER_SPEC_VERSION")]
pub const NV_PRESENT_BARRIER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_present_metering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_metering.html),
/// the C macro `VK_NV_PRESENT_METERING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_present_metering")]
#[doc(alias = "VK_NV_PRESENT_METERING_EXTENSION_NAME")]
pub const NV_PRESENT_METERING_NAME: &CStr = c"VK_NV_present_metering";
/// The revision of the
/// [`VK_NV_present_metering`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_present_metering.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PRESENT_METERING_SPEC_VERSION`.
#[doc(alias = "VK_NV_PRESENT_METERING_SPEC_VERSION")]
pub const NV_PRESENT_METERING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_private_data_base_handle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_private_data_base_handle.html),
/// the C macro `VK_NV_PRIVATE_DATA_BASE_HANDLE_EXTENSION_NAME`.
///
/// Requires Vulkan 1.3 or [`VK_EXT_private_data`](EXT_PRIVATE_DATA_NAME).
#[doc(alias = "VK_NV_private_data_base_handle")]
#[doc(alias = "VK_NV_PRIVATE_DATA_BASE_HANDLE_EXTENSION_NAME")]
pub const NV_PRIVATE_DATA_BASE_HANDLE_NAME: &CStr = c"VK_NV_private_data_base_handle";
/// The revision of the
/// [`VK_NV_private_data_base_handle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_private_data_base_handle.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PRIVATE_DATA_BASE_HANDLE_SPEC_VERSION`.
#[doc(alias = "VK_NV_PRIVATE_DATA_BASE_HANDLE_SPEC_VERSION")]
pub const NV_PRIVATE_DATA_BASE_HANDLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_push_constant_bank`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_push_constant_bank.html),
/// the C macro `VK_NV_PUSH_CONSTANT_BANK_EXTENSION_NAME`.
#[doc(alias = "VK_NV_push_constant_bank")]
#[doc(alias = "VK_NV_PUSH_CONSTANT_BANK_EXTENSION_NAME")]
pub const NV_PUSH_CONSTANT_BANK_NAME: &CStr = c"VK_NV_push_constant_bank";
/// The revision of the
/// [`VK_NV_push_constant_bank`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_push_constant_bank.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_PUSH_CONSTANT_BANK_SPEC_VERSION`.
#[doc(alias = "VK_NV_PUSH_CONSTANT_BANK_SPEC_VERSION")]
pub const NV_PUSH_CONSTANT_BANK_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_raw_access_chains`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_raw_access_chains.html),
/// the C macro `VK_NV_RAW_ACCESS_CHAINS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_raw_access_chains")]
#[doc(alias = "VK_NV_RAW_ACCESS_CHAINS_EXTENSION_NAME")]
pub const NV_RAW_ACCESS_CHAINS_NAME: &CStr = c"VK_NV_raw_access_chains";
/// The revision of the
/// [`VK_NV_raw_access_chains`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_raw_access_chains.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAW_ACCESS_CHAINS_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAW_ACCESS_CHAINS_SPEC_VERSION")]
pub const NV_RAW_ACCESS_CHAINS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_ray_tracing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing.html),
/// the C macro `VK_NV_RAY_TRACING_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) and
/// [`VK_KHR_get_memory_requirements2`](KHR_GET_MEMORY_REQUIREMENTS_2_NAME)) or Vulkan 1.1.
/// Deprecated by [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME).
#[doc(alias = "VK_NV_ray_tracing")]
#[doc(alias = "VK_NV_RAY_TRACING_EXTENSION_NAME")]
pub const NV_RAY_TRACING_NAME: &CStr = c"VK_NV_ray_tracing";
/// The revision of the
/// [`VK_NV_ray_tracing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAY_TRACING_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAY_TRACING_SPEC_VERSION")]
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_NV_ray_tracing_invocation_reorder`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_invocation_reorder.html),
/// the C macro `VK_NV_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME). Promoted to
/// [`VK_EXT_ray_tracing_invocation_reorder`](EXT_RAY_TRACING_INVOCATION_REORDER_NAME).
#[doc(alias = "VK_NV_ray_tracing_invocation_reorder")]
#[doc(alias = "VK_NV_RAY_TRACING_INVOCATION_REORDER_EXTENSION_NAME")]
pub const NV_RAY_TRACING_INVOCATION_REORDER_NAME: &CStr = c"VK_NV_ray_tracing_invocation_reorder";
/// The revision of the
/// [`VK_NV_ray_tracing_invocation_reorder`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_invocation_reorder.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION")]
pub const NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_ray_tracing_linear_swept_spheres`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_linear_swept_spheres.html),
/// the C macro `VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME).
#[doc(alias = "VK_NV_ray_tracing_linear_swept_spheres")]
#[doc(alias = "VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_EXTENSION_NAME")]
pub const NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_NAME: &CStr =
    c"VK_NV_ray_tracing_linear_swept_spheres";
/// The revision of the
/// [`VK_NV_ray_tracing_linear_swept_spheres`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_linear_swept_spheres.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION")]
pub const NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_ray_tracing_motion_blur`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_motion_blur.html),
/// the C macro `VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_ray_tracing_pipeline`](KHR_RAY_TRACING_PIPELINE_NAME).
#[doc(alias = "VK_NV_ray_tracing_motion_blur")]
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME")]
pub const NV_RAY_TRACING_MOTION_BLUR_NAME: &CStr = c"VK_NV_ray_tracing_motion_blur";
/// The revision of the
/// [`VK_NV_ray_tracing_motion_blur`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_motion_blur.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION")]
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_ray_tracing_validation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_validation.html),
/// the C macro `VK_NV_RAY_TRACING_VALIDATION_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_ray_tracing_validation")]
#[doc(alias = "VK_NV_RAY_TRACING_VALIDATION_EXTENSION_NAME")]
pub const NV_RAY_TRACING_VALIDATION_NAME: &CStr = c"VK_NV_ray_tracing_validation";
/// The revision of the
/// [`VK_NV_ray_tracing_validation`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_ray_tracing_validation.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_RAY_TRACING_VALIDATION_SPEC_VERSION`.
#[doc(alias = "VK_NV_RAY_TRACING_VALIDATION_SPEC_VERSION")]
pub const NV_RAY_TRACING_VALIDATION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_representative_fragment_test`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_representative_fragment_test.html),
/// the C macro `VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_representative_fragment_test")]
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_NAME: &CStr = c"VK_NV_representative_fragment_test";
/// The revision of the
/// [`VK_NV_representative_fragment_test`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_representative_fragment_test.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION`.
#[doc(alias = "VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION")]
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_sample_mask_override_coverage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_sample_mask_override_coverage.html),
/// the C macro `VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME`.
#[doc(alias = "VK_NV_sample_mask_override_coverage")]
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_NAME: &CStr = c"VK_NV_sample_mask_override_coverage";
/// The revision of the
/// [`VK_NV_sample_mask_override_coverage`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_sample_mask_override_coverage.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION`.
#[doc(alias = "VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION")]
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_scissor_exclusive`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_scissor_exclusive.html),
/// the C macro `VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_scissor_exclusive")]
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME")]
pub const NV_SCISSOR_EXCLUSIVE_NAME: &CStr = c"VK_NV_scissor_exclusive";
/// The revision of the
/// [`VK_NV_scissor_exclusive`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_scissor_exclusive.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION`.
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION")]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_shader_atomic_float16_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_atomic_float16_vector.html),
/// the C macro `VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_shader_atomic_float16_vector")]
#[doc(alias = "VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_EXTENSION_NAME")]
pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR_NAME: &CStr = c"VK_NV_shader_atomic_float16_vector";
/// The revision of the
/// [`VK_NV_shader_atomic_float16_vector`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_atomic_float16_vector.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION`.
#[doc(alias = "VK_NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION")]
pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_shader_image_footprint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_image_footprint.html),
/// the C macro `VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_shader_image_footprint")]
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME")]
pub const NV_SHADER_IMAGE_FOOTPRINT_NAME: &CStr = c"VK_NV_shader_image_footprint";
/// The revision of the
/// [`VK_NV_shader_image_footprint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_image_footprint.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION`.
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION")]
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_NV_shader_sm_builtins`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_sm_builtins.html),
/// the C macro `VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1.
#[doc(alias = "VK_NV_shader_sm_builtins")]
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME")]
pub const NV_SHADER_SM_BUILTINS_NAME: &CStr = c"VK_NV_shader_sm_builtins";
/// The revision of the
/// [`VK_NV_shader_sm_builtins`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_sm_builtins.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION`.
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION")]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_shader_subgroup_partitioned`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_subgroup_partitioned.html),
/// the C macro `VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME`.
///
/// Requires Vulkan 1.1. Promoted to
/// [`VK_EXT_shader_subgroup_partitioned`](EXT_SHADER_SUBGROUP_PARTITIONED_NAME).
#[doc(alias = "VK_NV_shader_subgroup_partitioned")]
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_NAME: &CStr = c"VK_NV_shader_subgroup_partitioned";
/// The revision of the
/// [`VK_NV_shader_subgroup_partitioned`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shader_subgroup_partitioned.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION`.
#[doc(alias = "VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION")]
pub const NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_shading_rate_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shading_rate_image.html),
/// the C macro `VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_NV_shading_rate_image")]
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME")]
pub const NV_SHADING_RATE_IMAGE_NAME: &CStr = c"VK_NV_shading_rate_image";
/// The revision of the
/// [`VK_NV_shading_rate_image`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_shading_rate_image.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION`.
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION")]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_NV_viewport_array2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_viewport_array2.html),
/// the C macro `VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME`.
#[doc(alias = "VK_NV_viewport_array2")]
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME")]
pub const NV_VIEWPORT_ARRAY_2_NAME: &CStr = c"VK_NV_viewport_array2";
/// The revision of the
/// [`VK_NV_viewport_array2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_viewport_array2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION`.
#[doc(alias = "VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION")]
pub const NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_viewport_swizzle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_viewport_swizzle.html),
/// the C macro `VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME`.
#[doc(alias = "VK_NV_viewport_swizzle")]
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_NAME: &CStr = c"VK_NV_viewport_swizzle";
/// The revision of the
/// [`VK_NV_viewport_swizzle`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_viewport_swizzle.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION`.
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_NV_win32_keyed_mutex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_win32_keyed_mutex.html),
/// the C macro `VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME`.
///
/// Requires [`VK_NV_external_memory_win32`](NV_EXTERNAL_MEMORY_WIN32_NAME). Promoted to
/// [`VK_KHR_win32_keyed_mutex`](KHR_WIN32_KEYED_MUTEX_NAME).
#[doc(alias = "VK_NV_win32_keyed_mutex")]
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
#[cfg(feature = "win32")]
pub const NV_WIN32_KEYED_MUTEX_NAME: &CStr = c"VK_NV_win32_keyed_mutex";
/// The revision of the
/// [`VK_NV_win32_keyed_mutex`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_NV_win32_keyed_mutex.html)
/// extension that these bindings were generated from, the C macro
/// `VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION`.
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION")]
#[cfg(feature = "win32")]
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_OHOS_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_external_memory.html),
/// the C macro `VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME),
/// [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) and
/// [`VK_KHR_dedicated_allocation`](KHR_DEDICATED_ALLOCATION_NAME)) or Vulkan 1.1) and
/// [`VK_EXT_queue_family_foreign`](EXT_QUEUE_FAMILY_FOREIGN_NAME).
#[doc(alias = "VK_OHOS_external_memory")]
#[doc(alias = "VK_OHOS_EXTERNAL_MEMORY_EXTENSION_NAME")]
#[cfg(feature = "ohos")]
pub const OHOS_EXTERNAL_MEMORY_NAME: &CStr = c"VK_OHOS_external_memory";
/// The revision of the
/// [`VK_OHOS_external_memory`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_external_memory.html)
/// extension that these bindings were generated from, the C macro
/// `VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION`.
#[doc(alias = "VK_OHOS_EXTERNAL_MEMORY_SPEC_VERSION")]
#[cfg(feature = "ohos")]
pub const OHOS_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_OHOS_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_surface.html),
/// the C macro `VK_OHOS_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_OHOS_surface")]
#[doc(alias = "VK_OHOS_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "ohos")]
pub const OHOS_SURFACE_NAME: &CStr = c"VK_OHOS_surface";
/// The revision of the
/// [`VK_OHOS_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_OHOS_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_OHOS_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_OHOS_SURFACE_SPEC_VERSION")]
#[cfg(feature = "ohos")]
pub const OHOS_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_cooperative_matrix_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_cooperative_matrix_conversion.html),
/// the C macro `VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_cooperative_matrix`](KHR_COOPERATIVE_MATRIX_NAME).
#[doc(alias = "VK_QCOM_cooperative_matrix_conversion")]
#[doc(alias = "VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_EXTENSION_NAME")]
pub const QCOM_COOPERATIVE_MATRIX_CONVERSION_NAME: &CStr = c"VK_QCOM_cooperative_matrix_conversion";
/// The revision of the
/// [`VK_QCOM_cooperative_matrix_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_cooperative_matrix_conversion.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION")]
pub const QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_data_graph_model`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_data_graph_model.html),
/// the C macro `VK_QCOM_DATA_GRAPH_MODEL_EXTENSION_NAME`.
///
/// Requires [`VK_ARM_data_graph`](ARM_DATA_GRAPH_NAME).
#[doc(alias = "VK_QCOM_data_graph_model")]
#[doc(alias = "VK_QCOM_DATA_GRAPH_MODEL_EXTENSION_NAME")]
pub const QCOM_DATA_GRAPH_MODEL_NAME: &CStr = c"VK_QCOM_data_graph_model";
/// The revision of the
/// [`VK_QCOM_data_graph_model`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_data_graph_model.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_DATA_GRAPH_MODEL_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_DATA_GRAPH_MODEL_SPEC_VERSION")]
pub const QCOM_DATA_GRAPH_MODEL_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_elapsed_timer_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_elapsed_timer_query.html),
/// the C macro `VK_QCOM_ELAPSED_TIMER_QUERY_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_elapsed_timer_query")]
#[doc(alias = "VK_QCOM_ELAPSED_TIMER_QUERY_EXTENSION_NAME")]
pub const QCOM_ELAPSED_TIMER_QUERY_NAME: &CStr = c"VK_QCOM_elapsed_timer_query";
/// The revision of the
/// [`VK_QCOM_elapsed_timer_query`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_elapsed_timer_query.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_ELAPSED_TIMER_QUERY_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_ELAPSED_TIMER_QUERY_SPEC_VERSION")]
pub const QCOM_ELAPSED_TIMER_QUERY_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_filter_cubic_clamp`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_clamp.html),
/// the C macro `VK_QCOM_FILTER_CUBIC_CLAMP_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_filter_cubic`](EXT_FILTER_CUBIC_NAME) and (Vulkan 1.2 or
/// [`VK_EXT_sampler_filter_minmax`](EXT_SAMPLER_FILTER_MINMAX_NAME)).
#[doc(alias = "VK_QCOM_filter_cubic_clamp")]
#[doc(alias = "VK_QCOM_FILTER_CUBIC_CLAMP_EXTENSION_NAME")]
pub const QCOM_FILTER_CUBIC_CLAMP_NAME: &CStr = c"VK_QCOM_filter_cubic_clamp";
/// The revision of the
/// [`VK_QCOM_filter_cubic_clamp`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_clamp.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION")]
pub const QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_filter_cubic_weights`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_weights.html),
/// the C macro `VK_QCOM_FILTER_CUBIC_WEIGHTS_EXTENSION_NAME`.
///
/// Requires [`VK_EXT_filter_cubic`](EXT_FILTER_CUBIC_NAME).
#[doc(alias = "VK_QCOM_filter_cubic_weights")]
#[doc(alias = "VK_QCOM_FILTER_CUBIC_WEIGHTS_EXTENSION_NAME")]
pub const QCOM_FILTER_CUBIC_WEIGHTS_NAME: &CStr = c"VK_QCOM_filter_cubic_weights";
/// The revision of the
/// [`VK_QCOM_filter_cubic_weights`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_filter_cubic_weights.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION")]
pub const QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_fragment_density_map_offset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_fragment_density_map_offset.html),
/// the C macro `VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and [`VK_EXT_fragment_density_map`](EXT_FRAGMENT_DENSITY_MAP_NAME). Promoted to
/// [`VK_EXT_fragment_density_map_offset`](EXT_FRAGMENT_DENSITY_MAP_OFFSET_NAME).
#[doc(alias = "VK_QCOM_fragment_density_map_offset")]
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_NAME: &CStr = c"VK_QCOM_fragment_density_map_offset";
/// The revision of the
/// [`VK_QCOM_fragment_density_map_offset`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_fragment_density_map_offset.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION")]
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 3;

/// The name of the device extension
/// [`VK_QCOM_image_processing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing.html),
/// the C macro `VK_QCOM_IMAGE_PROCESSING_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_format_feature_flags2`](KHR_FORMAT_FEATURE_FLAGS_2_NAME) or Vulkan 1.3.
#[doc(alias = "VK_QCOM_image_processing")]
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_EXTENSION_NAME")]
pub const QCOM_IMAGE_PROCESSING_NAME: &CStr = c"VK_QCOM_image_processing";
/// The revision of the
/// [`VK_QCOM_image_processing`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_IMAGE_PROCESSING_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_SPEC_VERSION")]
pub const QCOM_IMAGE_PROCESSING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_image_processing2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing2.html),
/// the C macro `VK_QCOM_IMAGE_PROCESSING_2_EXTENSION_NAME`.
///
/// Requires [`VK_QCOM_image_processing`](QCOM_IMAGE_PROCESSING_NAME).
#[doc(alias = "VK_QCOM_image_processing2")]
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_2_EXTENSION_NAME")]
pub const QCOM_IMAGE_PROCESSING_2_NAME: &CStr = c"VK_QCOM_image_processing2";
/// The revision of the
/// [`VK_QCOM_image_processing2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing2.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_IMAGE_PROCESSING_2_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_2_SPEC_VERSION")]
pub const QCOM_IMAGE_PROCESSING_2_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_image_processing3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing3.html),
/// the C macro `VK_QCOM_IMAGE_PROCESSING_3_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_image_processing3")]
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_3_EXTENSION_NAME")]
pub const QCOM_IMAGE_PROCESSING_3_NAME: &CStr = c"VK_QCOM_image_processing3";
/// The revision of the
/// [`VK_QCOM_image_processing3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_image_processing3.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_IMAGE_PROCESSING_3_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_IMAGE_PROCESSING_3_SPEC_VERSION")]
pub const QCOM_IMAGE_PROCESSING_3_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_multiview_per_view_render_areas`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_render_areas.html),
/// the C macro `VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_multiview_per_view_render_areas")]
#[doc(alias = "VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_EXTENSION_NAME")]
pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_NAME: &CStr =
    c"VK_QCOM_multiview_per_view_render_areas";
/// The revision of the
/// [`VK_QCOM_multiview_per_view_render_areas`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_render_areas.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION")]
pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_multiview_per_view_viewports`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_viewports.html),
/// the C macro `VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_multiview_per_view_viewports")]
#[doc(alias = "VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_EXTENSION_NAME")]
pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_NAME: &CStr = c"VK_QCOM_multiview_per_view_viewports";
/// The revision of the
/// [`VK_QCOM_multiview_per_view_viewports`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_multiview_per_view_viewports.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION")]
pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_queue_perf_hint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_queue_perf_hint.html),
/// the C macro `VK_QCOM_QUEUE_PERF_HINT_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_queue_perf_hint")]
#[doc(alias = "VK_QCOM_QUEUE_PERF_HINT_EXTENSION_NAME")]
pub const QCOM_QUEUE_PERF_HINT_NAME: &CStr = c"VK_QCOM_queue_perf_hint";
/// The revision of the
/// [`VK_QCOM_queue_perf_hint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_queue_perf_hint.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_QUEUE_PERF_HINT_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_QUEUE_PERF_HINT_SPEC_VERSION")]
pub const QCOM_QUEUE_PERF_HINT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_render_pass_shader_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_shader_resolve.html),
/// the C macro `VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME`.
///
/// Promoted to [`VK_EXT_custom_resolve`](EXT_CUSTOM_RESOLVE_NAME).
#[doc(alias = "VK_QCOM_render_pass_shader_resolve")]
#[doc(alias = "VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_SHADER_RESOLVE_NAME: &CStr = c"VK_QCOM_render_pass_shader_resolve";
/// The revision of the
/// [`VK_QCOM_render_pass_shader_resolve`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_shader_resolve.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION: u32 = 4;

/// The name of the device extension
/// [`VK_QCOM_render_pass_store_ops`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_store_ops.html),
/// the C macro `VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME`.
#[doc(alias = "VK_QCOM_render_pass_store_ops")]
#[doc(alias = "VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_STORE_OPS_NAME: &CStr = c"VK_QCOM_render_pass_store_ops";
/// The revision of the
/// [`VK_QCOM_render_pass_store_ops`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_store_ops.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_QCOM_render_pass_transform`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_transform.html),
/// the C macro `VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME`.
#[doc(alias = "VK_QCOM_render_pass_transform")]
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME")]
pub const QCOM_RENDER_PASS_TRANSFORM_NAME: &CStr = c"VK_QCOM_render_pass_transform";
/// The revision of the
/// [`VK_QCOM_render_pass_transform`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_render_pass_transform.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION")]
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 5;

/// The name of the device extension
/// [`VK_QCOM_rotated_copy_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_rotated_copy_commands.html),
/// the C macro `VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_copy_commands2`](KHR_COPY_COMMANDS_2_NAME) or Vulkan 1.3.
#[doc(alias = "VK_QCOM_rotated_copy_commands")]
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME")]
pub const QCOM_ROTATED_COPY_COMMANDS_NAME: &CStr = c"VK_QCOM_rotated_copy_commands";
/// The revision of the
/// [`VK_QCOM_rotated_copy_commands`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_rotated_copy_commands.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION")]
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_QCOM_shader_multiple_wait_queues`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_shader_multiple_wait_queues.html),
/// the C macro `VK_QCOM_SHADER_MULTIPLE_WAIT_QUEUES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_shader_multiple_wait_queues")]
#[doc(alias = "VK_QCOM_SHADER_MULTIPLE_WAIT_QUEUES_EXTENSION_NAME")]
pub const QCOM_SHADER_MULTIPLE_WAIT_QUEUES_NAME: &CStr = c"VK_QCOM_shader_multiple_wait_queues";
/// The revision of the
/// [`VK_QCOM_shader_multiple_wait_queues`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_shader_multiple_wait_queues.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_SHADER_MULTIPLE_WAIT_QUEUES_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_SHADER_MULTIPLE_WAIT_QUEUES_SPEC_VERSION")]
pub const QCOM_SHADER_MULTIPLE_WAIT_QUEUES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_tile_memory_heap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_memory_heap.html),
/// the C macro `VK_QCOM_TILE_MEMORY_HEAP_EXTENSION_NAME`.
///
/// Requires ([`VK_KHR_get_memory_requirements2`](KHR_GET_MEMORY_REQUIREMENTS_2_NAME) and
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME)) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_tile_memory_heap")]
#[doc(alias = "VK_QCOM_TILE_MEMORY_HEAP_EXTENSION_NAME")]
pub const QCOM_TILE_MEMORY_HEAP_NAME: &CStr = c"VK_QCOM_tile_memory_heap";
/// The revision of the
/// [`VK_QCOM_tile_memory_heap`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_memory_heap.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_TILE_MEMORY_HEAP_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_TILE_MEMORY_HEAP_SPEC_VERSION")]
pub const QCOM_TILE_MEMORY_HEAP_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_tile_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_properties.html),
/// the C macro `VK_QCOM_TILE_PROPERTIES_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_tile_properties")]
#[doc(alias = "VK_QCOM_TILE_PROPERTIES_EXTENSION_NAME")]
pub const QCOM_TILE_PROPERTIES_NAME: &CStr = c"VK_QCOM_tile_properties";
/// The revision of the
/// [`VK_QCOM_tile_properties`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_properties.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_TILE_PROPERTIES_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_TILE_PROPERTIES_SPEC_VERSION")]
pub const QCOM_TILE_PROPERTIES_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QCOM_tile_shading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_shading.html),
/// the C macro `VK_QCOM_TILE_SHADING_EXTENSION_NAME`.
///
/// Requires [`VK_QCOM_tile_properties`](QCOM_TILE_PROPERTIES_NAME).
#[doc(alias = "VK_QCOM_tile_shading")]
#[doc(alias = "VK_QCOM_TILE_SHADING_EXTENSION_NAME")]
pub const QCOM_TILE_SHADING_NAME: &CStr = c"VK_QCOM_tile_shading";
/// The revision of the
/// [`VK_QCOM_tile_shading`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_tile_shading.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_TILE_SHADING_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_TILE_SHADING_SPEC_VERSION")]
pub const QCOM_TILE_SHADING_SPEC_VERSION: u32 = 2;

/// The name of the device extension
/// [`VK_QCOM_ycbcr_degamma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_ycbcr_degamma.html),
/// the C macro `VK_QCOM_YCBCR_DEGAMMA_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_QCOM_ycbcr_degamma")]
#[doc(alias = "VK_QCOM_YCBCR_DEGAMMA_EXTENSION_NAME")]
pub const QCOM_YCBCR_DEGAMMA_NAME: &CStr = c"VK_QCOM_ycbcr_degamma";
/// The revision of the
/// [`VK_QCOM_ycbcr_degamma`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QCOM_ycbcr_degamma.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QCOM_YCBCR_DEGAMMA_SPEC_VERSION`.
#[doc(alias = "VK_QCOM_YCBCR_DEGAMMA_SPEC_VERSION")]
pub const QCOM_YCBCR_DEGAMMA_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_QNX_external_memory_screen_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_external_memory_screen_buffer.html),
/// the C macro `VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME`.
///
/// Requires (([`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME),
/// [`VK_KHR_external_memory`](KHR_EXTERNAL_MEMORY_NAME) and
/// [`VK_KHR_dedicated_allocation`](KHR_DEDICATED_ALLOCATION_NAME)) or Vulkan 1.1) and
/// [`VK_EXT_queue_family_foreign`](EXT_QUEUE_FAMILY_FOREIGN_NAME).
#[doc(alias = "VK_QNX_external_memory_screen_buffer")]
#[doc(alias = "VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_EXTENSION_NAME")]
#[cfg(feature = "screen")]
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_NAME: &CStr = c"VK_QNX_external_memory_screen_buffer";
/// The revision of the
/// [`VK_QNX_external_memory_screen_buffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_external_memory_screen_buffer.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION`.
#[doc(alias = "VK_QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION")]
#[cfg(feature = "screen")]
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_QNX_screen_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_screen_surface.html),
/// the C macro `VK_QNX_SCREEN_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_QNX_screen_surface")]
#[doc(alias = "VK_QNX_SCREEN_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "screen")]
pub const QNX_SCREEN_SURFACE_NAME: &CStr = c"VK_QNX_screen_surface";
/// The revision of the
/// [`VK_QNX_screen_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_QNX_screen_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_QNX_SCREEN_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_QNX_SCREEN_SURFACE_SPEC_VERSION")]
#[cfg(feature = "screen")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_SEC_amigo_profiling`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_amigo_profiling.html),
/// the C macro `VK_SEC_AMIGO_PROFILING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_SEC_amigo_profiling")]
#[doc(alias = "VK_SEC_AMIGO_PROFILING_EXTENSION_NAME")]
pub const SEC_AMIGO_PROFILING_NAME: &CStr = c"VK_SEC_amigo_profiling";
/// The revision of the
/// [`VK_SEC_amigo_profiling`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_amigo_profiling.html)
/// extension that these bindings were generated from, the C macro
/// `VK_SEC_AMIGO_PROFILING_SPEC_VERSION`.
#[doc(alias = "VK_SEC_AMIGO_PROFILING_SPEC_VERSION")]
pub const SEC_AMIGO_PROFILING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_SEC_pipeline_cache_incremental_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_pipeline_cache_incremental_mode.html),
/// the C macro `VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_SEC_pipeline_cache_incremental_mode")]
#[doc(alias = "VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_EXTENSION_NAME")]
pub const SEC_PIPELINE_CACHE_INCREMENTAL_MODE_NAME: &CStr =
    c"VK_SEC_pipeline_cache_incremental_mode";
/// The revision of the
/// [`VK_SEC_pipeline_cache_incremental_mode`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_pipeline_cache_incremental_mode.html)
/// extension that these bindings were generated from, the C macro
/// `VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION`.
#[doc(alias = "VK_SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION")]
pub const SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_SEC_throttle_hint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_throttle_hint.html),
/// the C macro `VK_SEC_THROTTLE_HINT_EXTENSION_NAME`.
#[doc(alias = "VK_SEC_throttle_hint")]
#[doc(alias = "VK_SEC_THROTTLE_HINT_EXTENSION_NAME")]
pub const SEC_THROTTLE_HINT_NAME: &CStr = c"VK_SEC_throttle_hint";
/// The revision of the
/// [`VK_SEC_throttle_hint`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_throttle_hint.html)
/// extension that these bindings were generated from, the C macro
/// `VK_SEC_THROTTLE_HINT_SPEC_VERSION`.
#[doc(alias = "VK_SEC_THROTTLE_HINT_SPEC_VERSION")]
pub const SEC_THROTTLE_HINT_SPEC_VERSION: u32 = 1;

/// The name of the instance extension
/// [`VK_SEC_ubm_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_ubm_surface.html),
/// the C macro `VK_SEC_UBM_SURFACE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_surface`](KHR_SURFACE_NAME).
#[doc(alias = "VK_SEC_ubm_surface")]
#[doc(alias = "VK_SEC_UBM_SURFACE_EXTENSION_NAME")]
#[cfg(feature = "ubm")]
pub const SEC_UBM_SURFACE_NAME: &CStr = c"VK_SEC_ubm_surface";
/// The revision of the
/// [`VK_SEC_ubm_surface`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_SEC_ubm_surface.html)
/// extension that these bindings were generated from, the C macro
/// `VK_SEC_UBM_SURFACE_SPEC_VERSION`.
#[doc(alias = "VK_SEC_UBM_SURFACE_SPEC_VERSION")]
#[cfg(feature = "ubm")]
pub const SEC_UBM_SURFACE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_buffer_device_address_allocation_alignment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_buffer_device_address_allocation_alignment.html),
/// the C macro `VK_VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_buffer_device_address`](KHR_BUFFER_DEVICE_ADDRESS_NAME) or Vulkan 1.2.
#[doc(alias = "VK_VALVE_buffer_device_address_allocation_alignment")]
#[doc(alias = "VK_VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_EXTENSION_NAME")]
pub const VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_NAME: &CStr =
    c"VK_VALVE_buffer_device_address_allocation_alignment";
/// The revision of the
/// [`VK_VALVE_buffer_device_address_allocation_alignment`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_buffer_device_address_allocation_alignment.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_SPEC_VERSION")]
pub const VALVE_BUFFER_DEVICE_ADDRESS_ALLOCATION_ALIGNMENT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_descriptor_set_host_mapping`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_descriptor_set_host_mapping.html),
/// the C macro `VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME`.
///
/// Requires
/// [`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1.
#[doc(alias = "VK_VALVE_descriptor_set_host_mapping")]
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_NAME: &CStr = c"VK_VALVE_descriptor_set_host_mapping";
/// The revision of the
/// [`VK_VALVE_descriptor_set_host_mapping`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_descriptor_set_host_mapping.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION")]
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_fragment_density_map_layered`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_fragment_density_map_layered.html),
/// the C macro `VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_EXTENSION_NAME`.
///
/// Requires (Vulkan 1.4, [`VK_KHR_extended_flags`](KHR_EXTENDED_FLAGS_NAME) or
/// [`VK_KHR_maintenance5`](KHR_MAINTENANCE_5_NAME)) and
/// [`VK_EXT_fragment_density_map`](EXT_FRAGMENT_DENSITY_MAP_NAME).
#[doc(alias = "VK_VALVE_fragment_density_map_layered")]
#[doc(alias = "VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_EXTENSION_NAME")]
pub const VALVE_FRAGMENT_DENSITY_MAP_LAYERED_NAME: &CStr = c"VK_VALVE_fragment_density_map_layered";
/// The revision of the
/// [`VK_VALVE_fragment_density_map_layered`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_fragment_density_map_layered.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION")]
pub const VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_mutable_descriptor_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_mutable_descriptor_type.html),
/// the C macro `VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_maintenance3`](KHR_MAINTENANCE_3_NAME). Promoted to
/// [`VK_EXT_mutable_descriptor_type`](EXT_MUTABLE_DESCRIPTOR_TYPE_NAME).
#[doc(alias = "VK_VALVE_mutable_descriptor_type")]
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_NAME: &CStr = c"VK_VALVE_mutable_descriptor_type";
/// The revision of the
/// [`VK_VALVE_mutable_descriptor_type`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_mutable_descriptor_type.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION")]
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_shader_mixed_float_dot_product`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_shader_mixed_float_dot_product.html),
/// the C macro `VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_EXTENSION_NAME`.
///
/// Requires
/// ([`VK_KHR_get_physical_device_properties2`](KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME) or
/// Vulkan 1.1) and ([`VK_KHR_shader_float16_int8`](KHR_SHADER_FLOAT16_INT8_NAME) or Vulkan
/// 1.2).
#[doc(alias = "VK_VALVE_shader_mixed_float_dot_product")]
#[doc(alias = "VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_EXTENSION_NAME")]
pub const VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_NAME: &CStr =
    c"VK_VALVE_shader_mixed_float_dot_product";
/// The revision of the
/// [`VK_VALVE_shader_mixed_float_dot_product`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_shader_mixed_float_dot_product.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION")]
pub const VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION: u32 = 1;

/// The name of the device extension
/// [`VK_VALVE_video_encode_rgb_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_video_encode_rgb_conversion.html),
/// the C macro `VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_EXTENSION_NAME`.
///
/// Requires [`VK_KHR_video_encode_queue`](KHR_VIDEO_ENCODE_QUEUE_NAME) and
/// ([`VK_KHR_sampler_ycbcr_conversion`](KHR_SAMPLER_YCBCR_CONVERSION_NAME) or Vulkan 1.1).
#[doc(alias = "VK_VALVE_video_encode_rgb_conversion")]
#[doc(alias = "VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_EXTENSION_NAME")]
pub const VALVE_VIDEO_ENCODE_RGB_CONVERSION_NAME: &CStr = c"VK_VALVE_video_encode_rgb_conversion";
/// The revision of the
/// [`VK_VALVE_video_encode_rgb_conversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_VALVE_video_encode_rgb_conversion.html)
/// extension that these bindings were generated from, the C macro
/// `VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION`.
#[doc(alias = "VK_VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION")]
pub const VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION: u32 = 1;

/// Every extension of the registry, sorted by name as bytes. [`ExtensionInfo::get`] searches it.
pub const EXTENSIONS: &[ExtensionInfo] = &[
    ExtensionInfo {
        name: c"VK_AMDX_dense_geometry_format",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_acceleration_structure"),
            Dependency::Any(&[
                Dependency::Version(make_api_version(0, 1, 4, 0)),
                Dependency::Extension(c"VK_KHR_extended_flags"),
                Dependency::Extension(c"VK_KHR_maintenance5"),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMDX_shader_enqueue",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Extension(c"VK_KHR_synchronization2"),
                    Dependency::Extension(c"VK_KHR_spirv_1_4"),
                    Dependency::Extension(c"VK_EXT_extended_dynamic_state"),
                ]),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_maintenance5"),
            Dependency::Extension(c"VK_KHR_pipeline_library"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_anti_lag",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_buffer_marker",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_device_coherent_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_display_native_hdr",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_draw_indirect_count",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_draw_indirect_count"),
    },
    ExtensionInfo {
        name: c"VK_AMD_gcn_shader",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_gpa_interface",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_gpu_shader_half_float",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_gpu_shader_int16",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_memory_overallocation_behavior",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_mixed_attachment_samples",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_negative_viewport_height",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_pipeline_compiler_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_rasterization_order",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_ballot",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_core_properties",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_core_properties2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_AMD_shader_core_properties")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_early_and_late_fragment_tests",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_explicit_vertex_parameter",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_fragment_mask",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_image_load_store_lod",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_info",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_shader_trinary_minmax",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_AMD_texture_gather_bias_lod",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ANDROID_external_format_resolve",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(
            c"VK_ANDROID_external_memory_android_hardware_buffer",
        )),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ANDROID_external_memory_android_hardware_buffer",
        spec_version: 5,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                    Dependency::Extension(c"VK_KHR_external_memory"),
                    Dependency::Extension(c"VK_KHR_dedicated_allocation"),
                ]),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_queue_family_foreign"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_data_graph",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Version(make_api_version(0, 1, 3, 0)),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_extended_flags"),
                Dependency::Extension(c"VK_KHR_maintenance5"),
            ]),
            Dependency::Extension(c"VK_KHR_deferred_host_operations"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_data_graph_instruction_set_tosa",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_ARM_data_graph")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_data_graph_neural_accelerator_statistics",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_data_graph_optical_flow",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_ARM_data_graph")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_format_pack",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_performance_counters_by_region",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_pipeline_opacity_micromap",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_opacity_micromap")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_rasterization_order_attachment_access",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_rasterization_order_attachment_access"),
    },
    ExtensionInfo {
        name: c"VK_ARM_render_pass_striped",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_synchronization2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_scheduling_controls",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_ARM_shader_core_builtins")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_shader_core_builtins",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_shader_core_properties",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_shader_instrumentation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_tensor_controls",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_ARM_tensors")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_ARM_tensors",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 3, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_4444_formats",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_acquire_drm_display",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_EXT_direct_mode_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_acquire_xlib_display",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_EXT_direct_mode_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_astc_decode_mode",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_attachment_feedback_loop_dynamic_state",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_attachment_feedback_loop_layout"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_attachment_feedback_loop_layout",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_blend_operation_advanced",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_border_color_swizzle",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_custom_border_color")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_buffer_device_address",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_calibrated_timestamps",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_calibrated_timestamps"),
    },
    ExtensionInfo {
        name: c"VK_EXT_color_write_enable",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_conditional_rendering",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_conservative_rasterization",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_cooperative_matrix_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_cooperative_matrix")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_custom_border_color",
        spec_version: 12,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_custom_resolve",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_debug_marker",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_debug_report")),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_debug_utils"),
    },
    ExtensionInfo {
        name: c"VK_EXT_debug_report",
        spec_version: 10,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_debug_utils",
        spec_version: 2,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_bias_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_clamp_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_clamp_zero_one",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_depth_clamp_zero_one"),
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_clip_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_clip_enable",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_depth_range_unrestricted",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_descriptor_buffer",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::All(&[
                        Dependency::Any(&[
                            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                            Dependency::Version(make_api_version(0, 1, 1, 0)),
                        ]),
                        Dependency::Extension(c"VK_KHR_buffer_device_address"),
                        Dependency::Extension(c"VK_EXT_descriptor_indexing"),
                    ]),
                    Dependency::Version(make_api_version(0, 1, 2, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_synchronization2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_descriptor_heap",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_extended_flags"),
                    Dependency::Extension(c"VK_KHR_maintenance5"),
                ]),
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_buffer_device_address"),
                    Dependency::Version(make_api_version(0, 1, 2, 0)),
                ]),
            ]),
            Dependency::Version(make_api_version(0, 1, 4, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_descriptor_indexing",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_maintenance3"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_device_address_binding_report",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_debug_utils"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_device_fault",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_device_fault"),
    },
    ExtensionInfo {
        name: c"VK_EXT_device_generated_commands",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_buffer_device_address"),
                    Dependency::Version(make_api_version(0, 1, 2, 0)),
                ]),
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_extended_flags"),
                    Dependency::Extension(c"VK_KHR_maintenance5"),
                ]),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_device_memory_report",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_direct_mode_display",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_directfb_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_discard_rectangles",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_display_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_EXT_display_surface_counter"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_display_surface_counter",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_dynamic_rendering_unused_attachments",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_dynamic_rendering"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_extended_dynamic_state",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_extended_dynamic_state2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_extended_dynamic_state3",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_external_memory_acquire_unmodified",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_external_memory_dma_buf",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_external_memory_fd")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_external_memory_host",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_external_memory_metal",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_filter_cubic",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_fragment_density_map",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_fragment_density_map2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_fragment_density_map")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_fragment_density_map_offset",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_fragment_density_map"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_create_renderpass2"),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
            Dependency::Any(&[
                Dependency::Version(make_api_version(0, 1, 3, 0)),
                Dependency::Extension(c"VK_KHR_dynamic_rendering"),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_fragment_shader_interlock",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_frame_boundary",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_full_screen_exclusive",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_global_priority",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_global_priority"),
    },
    ExtensionInfo {
        name: c"VK_EXT_global_priority_query",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_EXT_global_priority"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_global_priority"),
    },
    ExtensionInfo {
        name: c"VK_EXT_graphics_pipeline_library",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_pipeline_library"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_hdr_metadata",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_swapchain")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_headless_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_host_image_copy",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_copy_commands2"),
                Dependency::Extension(c"VK_KHR_format_feature_flags2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_host_query_reset",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_2d_view_of_3d",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_maintenance1"),
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_compression_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_compression_control_swapchain",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_EXT_image_compression_control"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_drm_format_modifier",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::All(&[
                        Dependency::Extension(c"VK_KHR_bind_memory2"),
                        Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                        Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                    ]),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_image_format_list"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_robustness",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_sliced_view_of_3d",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_maintenance1"),
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_tiling_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_image_view_min_lod",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_index_type_uint8",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_index_type_uint8"),
    },
    ExtensionInfo {
        name: c"VK_EXT_inline_uniform_block",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_maintenance1"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_layer_settings",
        spec_version: 2,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_legacy_dithering",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_legacy_vertex_attributes",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_vertex_input_dynamic_state")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_line_rasterization",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_line_rasterization"),
    },
    ExtensionInfo {
        name: c"VK_EXT_load_store_op_none",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_load_store_op_none"),
    },
    ExtensionInfo {
        name: c"VK_EXT_map_memory_placed",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_map_memory2"),
            Dependency::Version(make_api_version(0, 1, 4, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_memory_budget",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_memory_decompression",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Extension(c"VK_KHR_buffer_device_address"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_memory_priority",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_mesh_shader",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_spirv_1_4"),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_metal_objects",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_metal_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_multi_draw",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_multisampled_render_to_single_sampled",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_create_renderpass2"),
                Dependency::Extension(c"VK_KHR_depth_stencil_resolve"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_multisampled_render_to_swapchain",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_EXT_multisampled_render_to_single_sampled"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_mutable_descriptor_type",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_maintenance3"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_nested_command_buffer",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_non_seamless_cube_map",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_opacity_micromap",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_acceleration_structure"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_synchronization2"),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_opacity_micromap"),
    },
    ExtensionInfo {
        name: c"VK_EXT_pageable_device_local_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_memory_priority")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pci_bus_info",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_physical_device_drm",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_creation_cache_control",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_creation_feedback",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_library_group_handles",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_ray_tracing_pipeline"),
            Dependency::Extension(c"VK_KHR_pipeline_library"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_pipeline_library_group_handles"),
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_properties",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_protected_access",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_pipeline_robustness",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_post_depth_coverage",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_present_mode_fifo_latest_ready",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_swapchain")),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_present_mode_fifo_latest_ready"),
    },
    ExtensionInfo {
        name: c"VK_EXT_present_timing",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_present_id2"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_calibrated_timestamps"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_primitive_restart_index",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_primitive_topology_list_restart",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_primitives_generated_query",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_transform_feedback")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_private_data",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_provoking_vertex",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_queue_family_foreign",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_rasterization_order_attachment_access",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_ray_tracing_invocation_reorder",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_ray_tracing_pipeline")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_rgba10x6_formats",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_robustness2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_robustness2"),
    },
    ExtensionInfo {
        name: c"VK_EXT_sample_locations",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_sampler_filter_minmax",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_scalar_block_layout",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_separate_stencil_usage",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_64bit_indexing",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_atomic_float",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_atomic_float2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_shader_atomic_float")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_demote_to_helper_invocation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_float8",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_image_atomic_int64",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_long_vector",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 2, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_module_identifier",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_EXT_pipeline_creation_cache_control"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_object",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_dynamic_rendering"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_ocp_microscaling_types",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_replicated_composites",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_split_barrier",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_stencil_export",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_subgroup_ballot",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_subgroup_partitioned",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_subgroup_vote",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_tile_image",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 3, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_uniform_buffer_unsized_array",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_shader_viewport_index_layer",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_subgroup_size_control",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_subpass_merge_feedback",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_surface_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_surface_maintenance1"),
    },
    ExtensionInfo {
        name: c"VK_EXT_swapchain_colorspace",
        spec_version: 5,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_swapchain_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_EXT_surface_maintenance1"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_swapchain_maintenance1"),
    },
    ExtensionInfo {
        name: c"VK_EXT_texel_buffer_alignment",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_texture_compression_astc_3d",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_texture_compression_astc_hdr",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_tooling_info",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_transform_feedback",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_validation_cache",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_validation_features",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_validation_flags",
        spec_version: 3,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_vertex_attribute_divisor",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_vertex_attribute_divisor"),
    },
    ExtensionInfo {
        name: c"VK_EXT_vertex_attribute_robustness",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_maintenance9"),
    },
    ExtensionInfo {
        name: c"VK_EXT_vertex_input_dynamic_state",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_ycbcr_2plane_444_formats",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_ycbcr_image_arrays",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_EXT_zero_initialize_device_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_FUCHSIA_buffer_collection",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_FUCHSIA_external_memory"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_FUCHSIA_external_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_external_memory_capabilities"),
                Dependency::Extension(c"VK_KHR_external_memory"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_FUCHSIA_external_semaphore",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_external_semaphore_capabilities"),
            Dependency::Extension(c"VK_KHR_external_semaphore"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_FUCHSIA_imagepipe_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GGP_frame_token",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_GGP_stream_descriptor_surface"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GGP_stream_descriptor_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GOOGLE_decorate_string",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GOOGLE_display_timing",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_swapchain")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GOOGLE_hlsl_functionality1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GOOGLE_surfaceless_query",
        spec_version: 2,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_GOOGLE_user_type",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_HUAWEI_cluster_culling_shader",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_HUAWEI_hdr_vivid",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_EXT_hdr_metadata"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_HUAWEI_invocation_mask",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_ray_tracing_pipeline"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_synchronization2"),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_HUAWEI_subpass_shading",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_create_renderpass2"),
                    Dependency::Version(make_api_version(0, 1, 2, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_synchronization2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_IMG_filter_cubic",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_IMG_filter_linear_2d",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_format_feature_flags2"),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_IMG_format_pvrtc",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_IMG_relaxed_line_rasterization",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_INTEL_device_info",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_INTEL_performance_query",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_INTEL_shader_integer_functions2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_16bit_storage",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_storage_buffer_storage_class"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_8bit_storage",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_storage_buffer_storage_class"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_acceleration_structure",
        spec_version: 13,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                    Dependency::Extension(c"VK_EXT_descriptor_indexing"),
                    Dependency::Extension(c"VK_KHR_buffer_device_address"),
                ]),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_deferred_host_operations"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_android_surface",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_bind_memory2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_buffer_device_address",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_device_group"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_calibrated_timestamps",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_compute_shader_derivatives",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_cooperative_matrix",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_copy_commands2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_copy_memory_indirect",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_buffer_device_address"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_create_renderpass2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_multiview"),
                Dependency::Extension(c"VK_KHR_maintenance2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_dedicated_allocation",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_memory_requirements2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_deferred_host_operations",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_depth_clamp_zero_one",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_depth_stencil_resolve",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_create_renderpass2"),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_descriptor_update_template",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_device_address_commands",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::All(&[
                        Dependency::Any(&[
                            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                            Dependency::Version(make_api_version(0, 1, 1, 0)),
                        ]),
                        Dependency::Extension(c"VK_KHR_buffer_device_address"),
                    ]),
                    Dependency::Version(make_api_version(0, 1, 2, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_synchronization2"),
                Dependency::Extension(c"VK_EXT_extended_dynamic_state"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_device_fault",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_device_group",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_device_group_creation")),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_device_group_creation",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_display",
        spec_version: 23,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_display_swapchain",
        spec_version: 10,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_display"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_draw_indirect_count",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_driver_properties",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_dynamic_rendering",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_depth_stencil_resolve"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_dynamic_rendering_local_read",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_dynamic_rendering"),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_extended_flags",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_fence",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_external_fence_capabilities")),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_fence_capabilities",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_fence_fd",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_fence"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_fence_win32",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_external_fence")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory_capabilities"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_memory_capabilities",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_memory_fd",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_memory_win32",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_semaphore",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(
            c"VK_KHR_external_semaphore_capabilities",
        )),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_semaphore_capabilities",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_semaphore_fd",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_semaphore"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_external_semaphore_win32",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_external_semaphore")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_format_feature_flags2",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_fragment_shader_barycentric",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_fragment_shading_rate",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_create_renderpass2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_get_display_properties2",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_get_memory_requirements2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_get_physical_device_properties2",
        spec_version: 2,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_get_surface_capabilities2",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_global_priority",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_image_format_list",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_imageless_framebuffer",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::All(&[
                        Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                        Dependency::Extension(c"VK_KHR_maintenance2"),
                    ]),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_image_format_list"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_incremental_present",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_swapchain")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_index_type_uint8",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_internally_synchronized_queues",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_line_rasterization",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_load_store_op_none",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance1",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance10",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance11",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance3",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance4",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance5",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Version(make_api_version(0, 1, 1, 0)),
                Dependency::Extension(c"VK_KHR_dynamic_rendering"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance6",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance7",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance8",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_maintenance9",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_map_memory2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_multiview",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_opacity_micromap",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_acceleration_structure"),
            Dependency::Extension(c"VK_KHR_device_address_commands"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_performance_query",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_pipeline_binary",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Version(make_api_version(0, 1, 4, 0)),
            Dependency::Extension(c"VK_KHR_extended_flags"),
            Dependency::Extension(c"VK_KHR_maintenance5"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_pipeline_executable_properties",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_pipeline_library",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_pipeline_library_group_handles",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_ray_tracing_pipeline"),
            Dependency::Extension(c"VK_KHR_pipeline_library"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_portability_enumeration",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_portability_subset",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_present_id",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_present_id2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_present_mode_fifo_latest_ready",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_swapchain")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_present_wait",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_present_id"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_present_wait2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_present_id2"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_push_descriptor",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_ray_query",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_spirv_1_4"),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_acceleration_structure"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_ray_tracing_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_acceleration_structure")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_ray_tracing_pipeline",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_spirv_1_4"),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_acceleration_structure"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_ray_tracing_position_fetch",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_acceleration_structure")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_relaxed_block_layout",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_robustness2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_sampler_mirror_clamp_to_edge",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_sampler_ycbcr_conversion",
        spec_version: 14,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_maintenance1"),
                Dependency::Extension(c"VK_KHR_bind_memory2"),
                Dependency::Extension(c"VK_KHR_get_memory_requirements2"),
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_separate_depth_stencil_layouts",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_create_renderpass2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_abort",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_device_fault"),
            Dependency::Extension(c"VK_KHR_shader_constant_data"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_atomic_int64",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_bfloat16",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_clock",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_constant_data",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_draw_parameters",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_expect_assume",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_float16_int8",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_float_controls",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_float_controls2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Version(make_api_version(0, 1, 1, 0)),
            Dependency::Extension(c"VK_KHR_shader_float_controls"),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_fma",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_integer_dot_product",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_maximal_reconvergence",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_non_semantic_info",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_quad_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                    Dependency::Extension(c"VK_KHR_vulkan_memory_model"),
                ]),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_shader_maximal_reconvergence"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_relaxed_extended_instruction",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_subgroup_extended_types",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_subgroup_rotate",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_subgroup_uniform_control_flow",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_terminate_invocation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shader_untyped_pointers",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(
            c"VK_KHR_get_physical_device_properties2",
        )),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_shared_presentable_image",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_spirv_1_4",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Version(make_api_version(0, 1, 1, 0)),
            Dependency::Extension(c"VK_KHR_shader_float_controls"),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_storage_buffer_storage_class",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_surface",
        spec_version: 25,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_surface_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_surface_protected_capabilities",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::All(&[
            Dependency::Version(make_api_version(0, 1, 1, 0)),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_swapchain",
        spec_version: 70,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_swapchain_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Extension(c"VK_KHR_surface_maintenance1"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_swapchain_mutable_format",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_swapchain"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_maintenance2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_image_format_list"),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_synchronization2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_timeline_semaphore",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_unified_image_layouts",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_uniform_buffer_standard_layout",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_variable_pointers",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_storage_buffer_storage_class"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 1, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_vertex_attribute_divisor",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 4, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_decode_av1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_decode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_decode_h264",
        spec_version: 9,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_decode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_decode_h265",
        spec_version: 8,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_decode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_decode_queue",
        spec_version: 8,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_video_queue"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_synchronization2"),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_decode_vp9",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_decode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_av1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_encode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_feedback2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_encode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_h264",
        spec_version: 14,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_encode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_h265",
        spec_version: 14,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_encode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_intra_refresh",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_encode_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_quantization_map",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_video_encode_queue"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_format_feature_flags2"),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_encode_queue",
        spec_version: 12,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_video_queue"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_synchronization2"),
                Dependency::Version(make_api_version(0, 1, 3, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_maintenance1",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_maintenance2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_video_queue")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_video_queue",
        spec_version: 8,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Version(make_api_version(0, 1, 1, 0)),
                Dependency::Extension(c"VK_KHR_synchronization2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_vulkan_memory_model",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 2, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_wayland_surface",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_win32_keyed_mutex",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_external_memory_win32")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_win32_surface",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_workgroup_memory_explicit_layout",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_xcb_surface",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_xlib_surface",
        spec_version: 6,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_KHR_zero_initialize_workgroup_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: Some(make_api_version(0, 1, 3, 0)),
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_LUNARG_direct_driver_loading",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_MESA_image_alignment_control",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_MSFT_layered_driver",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_MVK_ios_surface",
        spec_version: 3,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_MVK_macos_surface",
        spec_version: 3,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NN_vi_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NVX_binary_import",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NVX_image_view_handle",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NVX_multiview_per_view_attributes",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_multiview"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_acquire_winrt_display",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_direct_mode_display")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_clip_space_w_scaling",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_cluster_acceleration_structure",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_acceleration_structure")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_command_buffer_inheritance",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_compute_occupancy_priority",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_compute_shader_derivatives",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_compute_shader_derivatives"),
    },
    ExtensionInfo {
        name: c"VK_NV_cooperative_matrix",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_cooperative_matrix2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_cooperative_matrix")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_cooperative_matrix_decode_vector",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_NV_cooperative_matrix2")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_cooperative_vector",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_copy_memory_indirect",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_buffer_device_address"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_copy_memory_indirect"),
    },
    ExtensionInfo {
        name: c"VK_NV_corner_sampled_image",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_coverage_reduction_mode",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_NV_framebuffer_mixed_samples"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_cuda_kernel_launch",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_dedicated_allocation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_dedicated_allocation_image_aliasing",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_dedicated_allocation"),
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_descriptor_pool_overallocation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_device_diagnostic_checkpoints",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_device_diagnostics_config",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_device_generated_commands",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Version(make_api_version(0, 1, 1, 0)),
                Dependency::Extension(c"VK_KHR_buffer_device_address"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_device_generated_commands_compute",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_NV_device_generated_commands")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_displacement_micromap",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_opacity_micromap")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_display_stereo",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_display"),
            Dependency::Extension(c"VK_KHR_get_display_properties2"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_extended_sparse_address_space",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_external_compute_queue",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_external_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_NV_external_memory_capabilities")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_external_memory_capabilities",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_external_memory_rdma",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_external_memory"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_external_memory_win32",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_NV_external_memory")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_fill_rectangle",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_fragment_coverage_to_color",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_fragment_shader_barycentric",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_fragment_shader_barycentric"),
    },
    ExtensionInfo {
        name: c"VK_NV_fragment_shading_rate_enums",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_fragment_shading_rate")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_framebuffer_mixed_samples",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_geometry_shader_passthrough",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_glsl_shader",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_inherited_viewport_scissor",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_linear_color_attachment",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_low_latency",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_low_latency2",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Version(make_api_version(0, 1, 2, 0)),
                Dependency::Extension(c"VK_KHR_timeline_semaphore"),
            ]),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_present_id"),
                Dependency::Extension(c"VK_KHR_present_id2"),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_memory_decompression",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_buffer_device_address"),
            ]),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_memory_decompression"),
    },
    ExtensionInfo {
        name: c"VK_NV_mesh_shader",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_optical_flow",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Any(&[
                    Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                    Dependency::Version(make_api_version(0, 1, 1, 0)),
                ]),
                Dependency::Extension(c"VK_KHR_format_feature_flags2"),
                Dependency::Extension(c"VK_KHR_synchronization2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_partitioned_acceleration_structure",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_acceleration_structure")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_per_stage_descriptor_set",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_maintenance6"),
            Dependency::Version(make_api_version(0, 1, 4, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_present_barrier",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_KHR_surface"),
            Dependency::Extension(c"VK_KHR_get_surface_capabilities2"),
            Dependency::Extension(c"VK_KHR_swapchain"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_present_metering",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_private_data_base_handle",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Version(make_api_version(0, 1, 3, 0)),
            Dependency::Extension(c"VK_EXT_private_data"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_push_constant_bank",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_raw_access_chains",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_ray_tracing",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Extension(c"VK_KHR_get_memory_requirements2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_ray_tracing_invocation_reorder",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_ray_tracing_pipeline")),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_ray_tracing_invocation_reorder"),
    },
    ExtensionInfo {
        name: c"VK_NV_ray_tracing_linear_swept_spheres",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_ray_tracing_pipeline")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_ray_tracing_motion_blur",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_ray_tracing_pipeline")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_ray_tracing_validation",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_representative_fragment_test",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_sample_mask_override_coverage",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_scissor_exclusive",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_shader_atomic_float16_vector",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_shader_image_footprint",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_shader_sm_builtins",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_shader_subgroup_partitioned",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Version(make_api_version(0, 1, 1, 0))),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_shader_subgroup_partitioned"),
    },
    ExtensionInfo {
        name: c"VK_NV_shading_rate_image",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_viewport_array2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_viewport_swizzle",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_NV_win32_keyed_mutex",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_NV_external_memory_win32")),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_KHR_win32_keyed_mutex"),
    },
    ExtensionInfo {
        name: c"VK_OHOS_external_memory",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                    Dependency::Extension(c"VK_KHR_external_memory"),
                    Dependency::Extension(c"VK_KHR_dedicated_allocation"),
                ]),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_queue_family_foreign"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_OHOS_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_cooperative_matrix_conversion",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_cooperative_matrix")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_data_graph_model",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_ARM_data_graph")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_elapsed_timer_query",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_filter_cubic_clamp",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_EXT_filter_cubic"),
            Dependency::Any(&[
                Dependency::Version(make_api_version(0, 1, 2, 0)),
                Dependency::Extension(c"VK_EXT_sampler_filter_minmax"),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_filter_cubic_weights",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_EXT_filter_cubic")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_fragment_density_map_offset",
        spec_version: 3,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_fragment_density_map"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_fragment_density_map_offset"),
    },
    ExtensionInfo {
        name: c"VK_QCOM_image_processing",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_format_feature_flags2"),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_image_processing2",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_QCOM_image_processing")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_image_processing3",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_multiview_per_view_render_areas",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_multiview_per_view_viewports",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_queue_perf_hint",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_render_pass_shader_resolve",
        spec_version: 4,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_custom_resolve"),
    },
    ExtensionInfo {
        name: c"VK_QCOM_render_pass_store_ops",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_render_pass_transform",
        spec_version: 5,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_rotated_copy_commands",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_copy_commands2"),
            Dependency::Version(make_api_version(0, 1, 3, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_shader_multiple_wait_queues",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_tile_memory_heap",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::All(&[
                Dependency::Extension(c"VK_KHR_get_memory_requirements2"),
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            ]),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_tile_properties",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_tile_shading",
        spec_version: 2,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_QCOM_tile_properties")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QCOM_ycbcr_degamma",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QNX_external_memory_screen_buffer",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::All(&[
                    Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                    Dependency::Extension(c"VK_KHR_external_memory"),
                    Dependency::Extension(c"VK_KHR_dedicated_allocation"),
                ]),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Extension(c"VK_EXT_queue_family_foreign"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_QNX_screen_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_SEC_amigo_profiling",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_SEC_pipeline_cache_incremental_mode",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_SEC_throttle_hint",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: None,
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_SEC_ubm_surface",
        spec_version: 1,
        level: ExtensionLevel::Instance,
        depends: Some(Dependency::Extension(c"VK_KHR_surface")),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_VALVE_buffer_device_address_allocation_alignment",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_buffer_device_address"),
            Dependency::Version(make_api_version(0, 1, 2, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_VALVE_descriptor_set_host_mapping",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Any(&[
            Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
            Dependency::Version(make_api_version(0, 1, 1, 0)),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_VALVE_fragment_density_map_layered",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Version(make_api_version(0, 1, 4, 0)),
                Dependency::Extension(c"VK_KHR_extended_flags"),
                Dependency::Extension(c"VK_KHR_maintenance5"),
            ]),
            Dependency::Extension(c"VK_EXT_fragment_density_map"),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_VALVE_mutable_descriptor_type",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::Extension(c"VK_KHR_maintenance3")),
        promoted_to_version: None,
        promoted_to_extension: Some(c"VK_EXT_mutable_descriptor_type"),
    },
    ExtensionInfo {
        name: c"VK_VALVE_shader_mixed_float_dot_product",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_get_physical_device_properties2"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_shader_float16_int8"),
                Dependency::Version(make_api_version(0, 1, 2, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
    ExtensionInfo {
        name: c"VK_VALVE_video_encode_rgb_conversion",
        spec_version: 1,
        level: ExtensionLevel::Device,
        depends: Some(Dependency::All(&[
            Dependency::Extension(c"VK_KHR_video_encode_queue"),
            Dependency::Any(&[
                Dependency::Extension(c"VK_KHR_sampler_ycbcr_conversion"),
                Dependency::Version(make_api_version(0, 1, 1, 0)),
            ]),
        ])),
        promoted_to_version: None,
        promoted_to_extension: None,
    },
];
