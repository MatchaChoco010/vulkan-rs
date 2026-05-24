#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::vk::*;

use core::ffi::CStr;

#[cfg(feature = "beta")]
pub const AMDX_DENSE_GEOMETRY_FORMAT_NAME: &CStr = c"VK_AMDX_dense_geometry_format";
#[cfg(feature = "beta")]
pub const AMDX_DENSE_GEOMETRY_FORMAT_SPEC_VERSION: u32 = 1;

#[cfg(feature = "beta")]
pub const AMDX_SHADER_ENQUEUE_NAME: &CStr = c"VK_AMDX_shader_enqueue";
#[cfg(feature = "beta")]
pub const AMDX_SHADER_ENQUEUE_SPEC_VERSION: u32 = 2;

pub const AMD_ANTI_LAG_NAME: &CStr = c"VK_AMD_anti_lag";
pub const AMD_ANTI_LAG_SPEC_VERSION: u32 = 1;

pub const AMD_BUFFER_MARKER_NAME: &CStr = c"VK_AMD_buffer_marker";
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;

pub const AMD_DEVICE_COHERENT_MEMORY_NAME: &CStr = c"VK_AMD_device_coherent_memory";
pub const AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;

pub const AMD_DISPLAY_NATIVE_HDR_NAME: &CStr = c"VK_AMD_display_native_hdr";
pub const AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;

pub const AMD_DRAW_INDIRECT_COUNT_NAME: &CStr = c"VK_AMD_draw_indirect_count";
pub const AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;

pub const AMD_GCN_SHADER_NAME: &CStr = c"VK_AMD_gcn_shader";
pub const AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;

pub const AMD_GPU_SHADER_HALF_FLOAT_NAME: &CStr = c"VK_AMD_gpu_shader_half_float";
pub const AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;

pub const AMD_GPU_SHADER_INT16_NAME: &CStr = c"VK_AMD_gpu_shader_int16";
pub const AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 2;

pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_NAME: &CStr = c"VK_AMD_memory_overallocation_behavior";
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;

pub const AMD_MIXED_ATTACHMENT_SAMPLES_NAME: &CStr = c"VK_AMD_mixed_attachment_samples";
pub const AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;

pub const AMD_NEGATIVE_VIEWPORT_HEIGHT_NAME: &CStr = c"VK_AMD_negative_viewport_height";
pub const AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: u32 = 1;

pub const AMD_PIPELINE_COMPILER_CONTROL_NAME: &CStr = c"VK_AMD_pipeline_compiler_control";
pub const AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;

pub const AMD_RASTERIZATION_ORDER_NAME: &CStr = c"VK_AMD_rasterization_order";
pub const AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_BALLOT_NAME: &CStr = c"VK_AMD_shader_ballot";
pub const AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_CORE_PROPERTIES_NAME: &CStr = c"VK_AMD_shader_core_properties";
pub const AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;

pub const AMD_SHADER_CORE_PROPERTIES_2_NAME: &CStr = c"VK_AMD_shader_core_properties2";
pub const AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_NAME: &CStr =
    c"VK_AMD_shader_early_and_late_fragment_tests";
pub const AMD_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_NAME: &CStr =
    c"VK_AMD_shader_explicit_vertex_parameter";
pub const AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_FRAGMENT_MASK_NAME: &CStr = c"VK_AMD_shader_fragment_mask";
pub const AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_NAME: &CStr = c"VK_AMD_shader_image_load_store_lod";
pub const AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_INFO_NAME: &CStr = c"VK_AMD_shader_info";
pub const AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;

pub const AMD_SHADER_TRINARY_MINMAX_NAME: &CStr = c"VK_AMD_shader_trinary_minmax";
pub const AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;

pub const AMD_TEXTURE_GATHER_BIAS_LOD_NAME: &CStr = c"VK_AMD_texture_gather_bias_lod";
pub const AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;

#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_FORMAT_RESOLVE_NAME: &CStr = c"VK_ANDROID_external_format_resolve";
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_FORMAT_RESOLVE_SPEC_VERSION: u32 = 1;

#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_NAME: &CStr =
    c"VK_ANDROID_external_memory_android_hardware_buffer";
#[cfg(feature = "android")]
pub const ANDROID_EXTERNAL_MEMORY_ANDROID_HARDWARE_BUFFER_SPEC_VERSION: u32 = 5;

pub const ARM_DATA_GRAPH_NAME: &CStr = c"VK_ARM_data_graph";
pub const ARM_DATA_GRAPH_SPEC_VERSION: u32 = 1;

pub const ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_NAME: &CStr =
    c"VK_ARM_data_graph_instruction_set_tosa";
pub const ARM_DATA_GRAPH_INSTRUCTION_SET_TOSA_SPEC_VERSION: u32 = 1;

pub const ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_NAME: &CStr =
    c"VK_ARM_data_graph_neural_accelerator_statistics";
pub const ARM_DATA_GRAPH_NEURAL_ACCELERATOR_STATISTICS_SPEC_VERSION: u32 = 1;

pub const ARM_DATA_GRAPH_OPTICAL_FLOW_NAME: &CStr = c"VK_ARM_data_graph_optical_flow";
pub const ARM_DATA_GRAPH_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;

pub const ARM_FORMAT_PACK_NAME: &CStr = c"VK_ARM_format_pack";
pub const ARM_FORMAT_PACK_SPEC_VERSION: u32 = 1;

pub const ARM_PERFORMANCE_COUNTERS_BY_REGION_NAME: &CStr = c"VK_ARM_performance_counters_by_region";
pub const ARM_PERFORMANCE_COUNTERS_BY_REGION_SPEC_VERSION: u32 = 1;

pub const ARM_PIPELINE_OPACITY_MICROMAP_NAME: &CStr = c"VK_ARM_pipeline_opacity_micromap";
pub const ARM_PIPELINE_OPACITY_MICROMAP_SPEC_VERSION: u32 = 1;

pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME: &CStr =
    c"VK_ARM_rasterization_order_attachment_access";
pub const ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;

pub const ARM_RENDER_PASS_STRIPED_NAME: &CStr = c"VK_ARM_render_pass_striped";
pub const ARM_RENDER_PASS_STRIPED_SPEC_VERSION: u32 = 1;

pub const ARM_SCHEDULING_CONTROLS_NAME: &CStr = c"VK_ARM_scheduling_controls";
pub const ARM_SCHEDULING_CONTROLS_SPEC_VERSION: u32 = 2;

pub const ARM_SHADER_CORE_BUILTINS_NAME: &CStr = c"VK_ARM_shader_core_builtins";
pub const ARM_SHADER_CORE_BUILTINS_SPEC_VERSION: u32 = 2;

pub const ARM_SHADER_CORE_PROPERTIES_NAME: &CStr = c"VK_ARM_shader_core_properties";
pub const ARM_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 1;

pub const ARM_SHADER_INSTRUMENTATION_NAME: &CStr = c"VK_ARM_shader_instrumentation";
pub const ARM_SHADER_INSTRUMENTATION_SPEC_VERSION: u32 = 1;

pub const ARM_TENSORS_NAME: &CStr = c"VK_ARM_tensors";
pub const ARM_TENSORS_SPEC_VERSION: u32 = 2;

pub const EXT_4444_FORMATS_NAME: &CStr = c"VK_EXT_4444_formats";
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;

pub const EXT_ACQUIRE_DRM_DISPLAY_NAME: &CStr = c"VK_EXT_acquire_drm_display";
pub const EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "xlib-xrandr")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_NAME: &CStr = c"VK_EXT_acquire_xlib_display";
#[cfg(feature = "xlib-xrandr")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;

pub const EXT_ASTC_DECODE_MODE_NAME: &CStr = c"VK_EXT_astc_decode_mode";
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;

pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_NAME: &CStr =
    c"VK_EXT_attachment_feedback_loop_dynamic_state";
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;

pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_NAME: &CStr =
    c"VK_EXT_attachment_feedback_loop_layout";
pub const EXT_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_SPEC_VERSION: u32 = 2;

pub const EXT_BLEND_OPERATION_ADVANCED_NAME: &CStr = c"VK_EXT_blend_operation_advanced";
pub const EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;

pub const EXT_BORDER_COLOR_SWIZZLE_NAME: &CStr = c"VK_EXT_border_color_swizzle";
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;

pub const EXT_BUFFER_DEVICE_ADDRESS_NAME: &CStr = c"VK_EXT_buffer_device_address";
pub const EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;

pub const EXT_CALIBRATED_TIMESTAMPS_NAME: &CStr = c"VK_EXT_calibrated_timestamps";
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;

pub const EXT_COLOR_WRITE_ENABLE_NAME: &CStr = c"VK_EXT_color_write_enable";
pub const EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;

pub const EXT_CONDITIONAL_RENDERING_NAME: &CStr = c"VK_EXT_conditional_rendering";
pub const EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;

pub const EXT_CONSERVATIVE_RASTERIZATION_NAME: &CStr = c"VK_EXT_conservative_rasterization";
pub const EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;

pub const EXT_CUSTOM_BORDER_COLOR_NAME: &CStr = c"VK_EXT_custom_border_color";
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;

pub const EXT_CUSTOM_RESOLVE_NAME: &CStr = c"VK_EXT_custom_resolve";
pub const EXT_CUSTOM_RESOLVE_SPEC_VERSION: u32 = 1;

pub const EXT_DEBUG_MARKER_NAME: &CStr = c"VK_EXT_debug_marker";
pub const EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;

pub const EXT_DEBUG_REPORT_NAME: &CStr = c"VK_EXT_debug_report";
pub const EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;

pub const EXT_DEBUG_UTILS_NAME: &CStr = c"VK_EXT_debug_utils";
pub const EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;

pub const EXT_DEPTH_BIAS_CONTROL_NAME: &CStr = c"VK_EXT_depth_bias_control";
pub const EXT_DEPTH_BIAS_CONTROL_SPEC_VERSION: u32 = 1;

pub const EXT_DEPTH_CLAMP_CONTROL_NAME: &CStr = c"VK_EXT_depth_clamp_control";
pub const EXT_DEPTH_CLAMP_CONTROL_SPEC_VERSION: u32 = 1;

pub const EXT_DEPTH_CLAMP_ZERO_ONE_NAME: &CStr = c"VK_EXT_depth_clamp_zero_one";
pub const EXT_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;

pub const EXT_DEPTH_CLIP_CONTROL_NAME: &CStr = c"VK_EXT_depth_clip_control";
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;

pub const EXT_DEPTH_CLIP_ENABLE_NAME: &CStr = c"VK_EXT_depth_clip_enable";
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;

pub const EXT_DEPTH_RANGE_UNRESTRICTED_NAME: &CStr = c"VK_EXT_depth_range_unrestricted";
pub const EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;

pub const EXT_DESCRIPTOR_BUFFER_NAME: &CStr = c"VK_EXT_descriptor_buffer";
pub const EXT_DESCRIPTOR_BUFFER_SPEC_VERSION: u32 = 1;

pub const EXT_DESCRIPTOR_HEAP_NAME: &CStr = c"VK_EXT_descriptor_heap";
pub const EXT_DESCRIPTOR_HEAP_SPEC_VERSION: u32 = 1;

pub const EXT_DESCRIPTOR_INDEXING_NAME: &CStr = c"VK_EXT_descriptor_indexing";
pub const EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;

pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_NAME: &CStr = c"VK_EXT_device_address_binding_report";
pub const EXT_DEVICE_ADDRESS_BINDING_REPORT_SPEC_VERSION: u32 = 1;

pub const EXT_DEVICE_FAULT_NAME: &CStr = c"VK_EXT_device_fault";
pub const EXT_DEVICE_FAULT_SPEC_VERSION: u32 = 2;

pub const EXT_DEVICE_GENERATED_COMMANDS_NAME: &CStr = c"VK_EXT_device_generated_commands";
pub const EXT_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 1;

pub const EXT_DEVICE_MEMORY_REPORT_NAME: &CStr = c"VK_EXT_device_memory_report";
pub const EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;

pub const EXT_DIRECT_MODE_DISPLAY_NAME: &CStr = c"VK_EXT_direct_mode_display";
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "directfb")]
pub const EXT_DIRECTFB_SURFACE_NAME: &CStr = c"VK_EXT_directfb_surface";
#[cfg(feature = "directfb")]
pub const EXT_DIRECTFB_SURFACE_SPEC_VERSION: u32 = 1;

pub const EXT_DISCARD_RECTANGLES_NAME: &CStr = c"VK_EXT_discard_rectangles";
pub const EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 2;

pub const EXT_DISPLAY_CONTROL_NAME: &CStr = c"VK_EXT_display_control";
pub const EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;

pub const EXT_DISPLAY_SURFACE_COUNTER_NAME: &CStr = c"VK_EXT_display_surface_counter";
pub const EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;

pub const EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_NAME: &CStr =
    c"VK_EXT_dynamic_rendering_unused_attachments";
pub const EXT_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_SPEC_VERSION: u32 = 1;

pub const EXT_EXTENDED_DYNAMIC_STATE_NAME: &CStr = c"VK_EXT_extended_dynamic_state";
pub const EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;

pub const EXT_EXTENDED_DYNAMIC_STATE_2_NAME: &CStr = c"VK_EXT_extended_dynamic_state2";
pub const EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;

pub const EXT_EXTENDED_DYNAMIC_STATE_3_NAME: &CStr = c"VK_EXT_extended_dynamic_state3";
pub const EXT_EXTENDED_DYNAMIC_STATE_3_SPEC_VERSION: u32 = 2;

pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_NAME: &CStr =
    c"VK_EXT_external_memory_acquire_unmodified";
pub const EXT_EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_SPEC_VERSION: u32 = 1;

pub const EXT_EXTERNAL_MEMORY_DMA_BUF_NAME: &CStr = c"VK_EXT_external_memory_dma_buf";
pub const EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;

pub const EXT_EXTERNAL_MEMORY_HOST_NAME: &CStr = c"VK_EXT_external_memory_host";
pub const EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;

#[cfg(feature = "metal")]
pub const EXT_EXTERNAL_MEMORY_METAL_NAME: &CStr = c"VK_EXT_external_memory_metal";
#[cfg(feature = "metal")]
pub const EXT_EXTERNAL_MEMORY_METAL_SPEC_VERSION: u32 = 1;

pub const EXT_FILTER_CUBIC_NAME: &CStr = c"VK_EXT_filter_cubic";
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;

pub const EXT_FRAGMENT_DENSITY_MAP_NAME: &CStr = c"VK_EXT_fragment_density_map";
pub const EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 3;

pub const EXT_FRAGMENT_DENSITY_MAP_2_NAME: &CStr = c"VK_EXT_fragment_density_map2";
pub const EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;

pub const EXT_FRAGMENT_DENSITY_MAP_OFFSET_NAME: &CStr = c"VK_EXT_fragment_density_map_offset";
pub const EXT_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;

pub const EXT_FRAGMENT_SHADER_INTERLOCK_NAME: &CStr = c"VK_EXT_fragment_shader_interlock";
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;

pub const EXT_FRAME_BOUNDARY_NAME: &CStr = c"VK_EXT_frame_boundary";
pub const EXT_FRAME_BOUNDARY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_NAME: &CStr = c"VK_EXT_full_screen_exclusive";
#[cfg(feature = "win32")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;

pub const EXT_GLOBAL_PRIORITY_NAME: &CStr = c"VK_EXT_global_priority";
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;

pub const EXT_GLOBAL_PRIORITY_QUERY_NAME: &CStr = c"VK_EXT_global_priority_query";
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;

pub const EXT_GRAPHICS_PIPELINE_LIBRARY_NAME: &CStr = c"VK_EXT_graphics_pipeline_library";
pub const EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;

pub const EXT_HDR_METADATA_NAME: &CStr = c"VK_EXT_hdr_metadata";
pub const EXT_HDR_METADATA_SPEC_VERSION: u32 = 3;

pub const EXT_HEADLESS_SURFACE_NAME: &CStr = c"VK_EXT_headless_surface";
pub const EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;

pub const EXT_HOST_IMAGE_COPY_NAME: &CStr = c"VK_EXT_host_image_copy";
pub const EXT_HOST_IMAGE_COPY_SPEC_VERSION: u32 = 1;

pub const EXT_HOST_QUERY_RESET_NAME: &CStr = c"VK_EXT_host_query_reset";
pub const EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_2D_VIEW_OF_3D_NAME: &CStr = c"VK_EXT_image_2d_view_of_3d";
pub const EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_COMPRESSION_CONTROL_NAME: &CStr = c"VK_EXT_image_compression_control";
pub const EXT_IMAGE_COMPRESSION_CONTROL_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_NAME: &CStr =
    c"VK_EXT_image_compression_control_swapchain";
pub const EXT_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_NAME: &CStr = c"VK_EXT_image_drm_format_modifier";
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;

pub const EXT_IMAGE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_image_robustness";
pub const EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_SLICED_VIEW_OF_3D_NAME: &CStr = c"VK_EXT_image_sliced_view_of_3d";
pub const EXT_IMAGE_SLICED_VIEW_OF_3D_SPEC_VERSION: u32 = 1;

pub const EXT_IMAGE_VIEW_MIN_LOD_NAME: &CStr = c"VK_EXT_image_view_min_lod";
pub const EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;

pub const EXT_INDEX_TYPE_UINT8_NAME: &CStr = c"VK_EXT_index_type_uint8";
pub const EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;

pub const EXT_INLINE_UNIFORM_BLOCK_NAME: &CStr = c"VK_EXT_inline_uniform_block";
pub const EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;

pub const EXT_LAYER_SETTINGS_NAME: &CStr = c"VK_EXT_layer_settings";
pub const EXT_LAYER_SETTINGS_SPEC_VERSION: u32 = 2;

pub const EXT_LEGACY_DITHERING_NAME: &CStr = c"VK_EXT_legacy_dithering";
pub const EXT_LEGACY_DITHERING_SPEC_VERSION: u32 = 2;

pub const EXT_LEGACY_VERTEX_ATTRIBUTES_NAME: &CStr = c"VK_EXT_legacy_vertex_attributes";
pub const EXT_LEGACY_VERTEX_ATTRIBUTES_SPEC_VERSION: u32 = 1;

pub const EXT_LINE_RASTERIZATION_NAME: &CStr = c"VK_EXT_line_rasterization";
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

pub const EXT_LOAD_STORE_OP_NONE_NAME: &CStr = c"VK_EXT_load_store_op_none";
pub const EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;

pub const EXT_MAP_MEMORY_PLACED_NAME: &CStr = c"VK_EXT_map_memory_placed";
pub const EXT_MAP_MEMORY_PLACED_SPEC_VERSION: u32 = 1;

pub const EXT_MEMORY_BUDGET_NAME: &CStr = c"VK_EXT_memory_budget";
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;

pub const EXT_MEMORY_DECOMPRESSION_NAME: &CStr = c"VK_EXT_memory_decompression";
pub const EXT_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;

pub const EXT_MEMORY_PRIORITY_NAME: &CStr = c"VK_EXT_memory_priority";
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;

pub const EXT_MESH_SHADER_NAME: &CStr = c"VK_EXT_mesh_shader";
pub const EXT_MESH_SHADER_SPEC_VERSION: u32 = 1;

#[cfg(feature = "metal")]
pub const EXT_METAL_OBJECTS_NAME: &CStr = c"VK_EXT_metal_objects";
#[cfg(feature = "metal")]
pub const EXT_METAL_OBJECTS_SPEC_VERSION: u32 = 2;

#[cfg(feature = "metal")]
pub const EXT_METAL_SURFACE_NAME: &CStr = c"VK_EXT_metal_surface";
#[cfg(feature = "metal")]
pub const EXT_METAL_SURFACE_SPEC_VERSION: u32 = 1;

pub const EXT_MULTI_DRAW_NAME: &CStr = c"VK_EXT_multi_draw";
pub const EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;

pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_NAME: &CStr =
    c"VK_EXT_multisampled_render_to_single_sampled";
pub const EXT_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_SPEC_VERSION: u32 = 1;

pub const EXT_MUTABLE_DESCRIPTOR_TYPE_NAME: &CStr = c"VK_EXT_mutable_descriptor_type";
pub const EXT_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;

pub const EXT_NESTED_COMMAND_BUFFER_NAME: &CStr = c"VK_EXT_nested_command_buffer";
pub const EXT_NESTED_COMMAND_BUFFER_SPEC_VERSION: u32 = 1;

pub const EXT_NON_SEAMLESS_CUBE_MAP_NAME: &CStr = c"VK_EXT_non_seamless_cube_map";
pub const EXT_NON_SEAMLESS_CUBE_MAP_SPEC_VERSION: u32 = 1;

pub const EXT_OPACITY_MICROMAP_NAME: &CStr = c"VK_EXT_opacity_micromap";
pub const EXT_OPACITY_MICROMAP_SPEC_VERSION: u32 = 2;

pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_NAME: &CStr = c"VK_EXT_pageable_device_local_memory";
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;

pub const EXT_PCI_BUS_INFO_NAME: &CStr = c"VK_EXT_pci_bus_info";
pub const EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;

pub const EXT_PHYSICAL_DEVICE_DRM_NAME: &CStr = c"VK_EXT_physical_device_drm";
pub const EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;

pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_NAME: &CStr =
    c"VK_EXT_pipeline_creation_cache_control";
pub const EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;

pub const EXT_PIPELINE_CREATION_FEEDBACK_NAME: &CStr = c"VK_EXT_pipeline_creation_feedback";
pub const EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;

pub const EXT_PIPELINE_LIBRARY_GROUP_HANDLES_NAME: &CStr = c"VK_EXT_pipeline_library_group_handles";
pub const EXT_PIPELINE_LIBRARY_GROUP_HANDLES_SPEC_VERSION: u32 = 1;

pub const EXT_PIPELINE_PROPERTIES_NAME: &CStr = c"VK_EXT_pipeline_properties";
pub const EXT_PIPELINE_PROPERTIES_SPEC_VERSION: u32 = 1;

pub const EXT_PIPELINE_PROTECTED_ACCESS_NAME: &CStr = c"VK_EXT_pipeline_protected_access";
pub const EXT_PIPELINE_PROTECTED_ACCESS_SPEC_VERSION: u32 = 1;

pub const EXT_PIPELINE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_pipeline_robustness";
pub const EXT_PIPELINE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

pub const EXT_POST_DEPTH_COVERAGE_NAME: &CStr = c"VK_EXT_post_depth_coverage";
pub const EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;

pub const EXT_PRESENT_MODE_FIFO_LATEST_READY_NAME: &CStr = c"VK_EXT_present_mode_fifo_latest_ready";
pub const EXT_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;

pub const EXT_PRESENT_TIMING_NAME: &CStr = c"VK_EXT_present_timing";
pub const EXT_PRESENT_TIMING_SPEC_VERSION: u32 = 3;

pub const EXT_PRIMITIVE_RESTART_INDEX_NAME: &CStr = c"VK_EXT_primitive_restart_index";
pub const EXT_PRIMITIVE_RESTART_INDEX_SPEC_VERSION: u32 = 1;

pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_NAME: &CStr =
    c"VK_EXT_primitive_topology_list_restart";
pub const EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;

pub const EXT_PRIMITIVES_GENERATED_QUERY_NAME: &CStr = c"VK_EXT_primitives_generated_query";
pub const EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION: u32 = 1;

pub const EXT_PRIVATE_DATA_NAME: &CStr = c"VK_EXT_private_data";
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;

pub const EXT_PROVOKING_VERTEX_NAME: &CStr = c"VK_EXT_provoking_vertex";
pub const EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;

pub const EXT_QUEUE_FAMILY_FOREIGN_NAME: &CStr = c"VK_EXT_queue_family_foreign";
pub const EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;

pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_NAME: &CStr =
    c"VK_EXT_rasterization_order_attachment_access";
pub const EXT_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;

pub const EXT_RAY_TRACING_INVOCATION_REORDER_NAME: &CStr = c"VK_EXT_ray_tracing_invocation_reorder";
pub const EXT_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 2;

pub const EXT_RGBA10X6_FORMATS_NAME: &CStr = c"VK_EXT_rgba10x6_formats";
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;

pub const EXT_ROBUSTNESS_2_NAME: &CStr = c"VK_EXT_robustness2";
pub const EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;

pub const EXT_SAMPLE_LOCATIONS_NAME: &CStr = c"VK_EXT_sample_locations";
pub const EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;

pub const EXT_SAMPLER_FILTER_MINMAX_NAME: &CStr = c"VK_EXT_sampler_filter_minmax";
pub const EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;

pub const EXT_SCALAR_BLOCK_LAYOUT_NAME: &CStr = c"VK_EXT_scalar_block_layout";
pub const EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;

pub const EXT_SEPARATE_STENCIL_USAGE_NAME: &CStr = c"VK_EXT_separate_stencil_usage";
pub const EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_64BIT_INDEXING_NAME: &CStr = c"VK_EXT_shader_64bit_indexing";
pub const EXT_SHADER_64BIT_INDEXING_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_ATOMIC_FLOAT_NAME: &CStr = c"VK_EXT_shader_atomic_float";
pub const EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_ATOMIC_FLOAT_2_NAME: &CStr = c"VK_EXT_shader_atomic_float2";
pub const EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_NAME: &CStr =
    c"VK_EXT_shader_demote_to_helper_invocation";
pub const EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_FLOAT8_NAME: &CStr = c"VK_EXT_shader_float8";
pub const EXT_SHADER_FLOAT8_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_IMAGE_ATOMIC_INT64_NAME: &CStr = c"VK_EXT_shader_image_atomic_int64";
pub const EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_LONG_VECTOR_NAME: &CStr = c"VK_EXT_shader_long_vector";
pub const EXT_SHADER_LONG_VECTOR_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_MODULE_IDENTIFIER_NAME: &CStr = c"VK_EXT_shader_module_identifier";
pub const EXT_SHADER_MODULE_IDENTIFIER_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_OBJECT_NAME: &CStr = c"VK_EXT_shader_object";
pub const EXT_SHADER_OBJECT_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_REPLICATED_COMPOSITES_NAME: &CStr = c"VK_EXT_shader_replicated_composites";
pub const EXT_SHADER_REPLICATED_COMPOSITES_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_STENCIL_EXPORT_NAME: &CStr = c"VK_EXT_shader_stencil_export";
pub const EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_SUBGROUP_BALLOT_NAME: &CStr = c"VK_EXT_shader_subgroup_ballot";
pub const EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_SUBGROUP_PARTITIONED_NAME: &CStr = c"VK_EXT_shader_subgroup_partitioned";
pub const EXT_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_SUBGROUP_VOTE_NAME: &CStr = c"VK_EXT_shader_subgroup_vote";
pub const EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_TILE_IMAGE_NAME: &CStr = c"VK_EXT_shader_tile_image";
pub const EXT_SHADER_TILE_IMAGE_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_NAME: &CStr =
    c"VK_EXT_shader_uniform_buffer_unsized_array";
pub const EXT_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_SPEC_VERSION: u32 = 1;

pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_NAME: &CStr = c"VK_EXT_shader_viewport_index_layer";
pub const EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;

pub const EXT_SUBGROUP_SIZE_CONTROL_NAME: &CStr = c"VK_EXT_subgroup_size_control";
pub const EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;

pub const EXT_SUBPASS_MERGE_FEEDBACK_NAME: &CStr = c"VK_EXT_subpass_merge_feedback";
pub const EXT_SUBPASS_MERGE_FEEDBACK_SPEC_VERSION: u32 = 2;

pub const EXT_SURFACE_MAINTENANCE_1_NAME: &CStr = c"VK_EXT_surface_maintenance1";
pub const EXT_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const EXT_SWAPCHAIN_COLOR_SPACE_NAME: &CStr = c"VK_EXT_swapchain_colorspace";
pub const EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 5;

pub const EXT_SWAPCHAIN_MAINTENANCE_1_NAME: &CStr = c"VK_EXT_swapchain_maintenance1";
pub const EXT_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const EXT_TEXEL_BUFFER_ALIGNMENT_NAME: &CStr = c"VK_EXT_texel_buffer_alignment";
pub const EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;

pub const EXT_TEXTURE_COMPRESSION_ASTC_3D_NAME: &CStr = c"VK_EXT_texture_compression_astc_3d";
pub const EXT_TEXTURE_COMPRESSION_ASTC_3D_SPEC_VERSION: u32 = 1;

pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_NAME: &CStr = c"VK_EXT_texture_compression_astc_hdr";
pub const EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;

pub const EXT_TOOLING_INFO_NAME: &CStr = c"VK_EXT_tooling_info";
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;

pub const EXT_TRANSFORM_FEEDBACK_NAME: &CStr = c"VK_EXT_transform_feedback";
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;

pub const EXT_VALIDATION_CACHE_NAME: &CStr = c"VK_EXT_validation_cache";
pub const EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;

pub const EXT_VALIDATION_FEATURES_NAME: &CStr = c"VK_EXT_validation_features";
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 6;

pub const EXT_VALIDATION_FLAGS_NAME: &CStr = c"VK_EXT_validation_flags";
pub const EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 3;

pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_NAME: &CStr = c"VK_EXT_vertex_attribute_divisor";
pub const EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;

pub const EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_NAME: &CStr = c"VK_EXT_vertex_attribute_robustness";
pub const EXT_VERTEX_ATTRIBUTE_ROBUSTNESS_SPEC_VERSION: u32 = 1;

pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_NAME: &CStr = c"VK_EXT_vertex_input_dynamic_state";
pub const EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;

pub const EXT_YCBCR_2PLANE_444_FORMATS_NAME: &CStr = c"VK_EXT_ycbcr_2plane_444_formats";
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;

pub const EXT_YCBCR_IMAGE_ARRAYS_NAME: &CStr = c"VK_EXT_ycbcr_image_arrays";
pub const EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;

pub const EXT_ZERO_INITIALIZE_DEVICE_MEMORY_NAME: &CStr = c"VK_EXT_zero_initialize_device_memory";
pub const EXT_ZERO_INITIALIZE_DEVICE_MEMORY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "fuchsia")]
pub const FUCHSIA_BUFFER_COLLECTION_NAME: &CStr = c"VK_FUCHSIA_buffer_collection";
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_BUFFER_COLLECTION_SPEC_VERSION: u32 = 2;

#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_MEMORY_NAME: &CStr = c"VK_FUCHSIA_external_memory";
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_NAME: &CStr = c"VK_FUCHSIA_external_semaphore";
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;

#[cfg(feature = "fuchsia")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_NAME: &CStr = c"VK_FUCHSIA_imagepipe_surface";
#[cfg(feature = "fuchsia")]
pub const FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION: u32 = 1;

#[cfg(feature = "ggp")]
pub const GGP_FRAME_TOKEN_NAME: &CStr = c"VK_GGP_frame_token";
#[cfg(feature = "ggp")]
pub const GGP_FRAME_TOKEN_SPEC_VERSION: u32 = 1;

#[cfg(feature = "ggp")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_NAME: &CStr = c"VK_GGP_stream_descriptor_surface";
#[cfg(feature = "ggp")]
pub const GGP_STREAM_DESCRIPTOR_SURFACE_SPEC_VERSION: u32 = 1;

pub const GOOGLE_DECORATE_STRING_NAME: &CStr = c"VK_GOOGLE_decorate_string";
pub const GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;

pub const GOOGLE_DISPLAY_TIMING_NAME: &CStr = c"VK_GOOGLE_display_timing";
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;

pub const GOOGLE_HLSL_FUNCTIONALITY_1_NAME: &CStr = c"VK_GOOGLE_hlsl_functionality1";
pub const GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;

pub const GOOGLE_SURFACELESS_QUERY_NAME: &CStr = c"VK_GOOGLE_surfaceless_query";
pub const GOOGLE_SURFACELESS_QUERY_SPEC_VERSION: u32 = 2;

pub const GOOGLE_USER_TYPE_NAME: &CStr = c"VK_GOOGLE_user_type";
pub const GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;

pub const HUAWEI_CLUSTER_CULLING_SHADER_NAME: &CStr = c"VK_HUAWEI_cluster_culling_shader";
pub const HUAWEI_CLUSTER_CULLING_SHADER_SPEC_VERSION: u32 = 3;

pub const HUAWEI_HDR_VIVID_NAME: &CStr = c"VK_HUAWEI_hdr_vivid";
pub const HUAWEI_HDR_VIVID_SPEC_VERSION: u32 = 1;

pub const HUAWEI_INVOCATION_MASK_NAME: &CStr = c"VK_HUAWEI_invocation_mask";
pub const HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;

pub const HUAWEI_SUBPASS_SHADING_NAME: &CStr = c"VK_HUAWEI_subpass_shading";
pub const HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 3;

pub const IMG_FILTER_CUBIC_NAME: &CStr = c"VK_IMG_filter_cubic";
pub const IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;

pub const IMG_FORMAT_PVRTC_NAME: &CStr = c"VK_IMG_format_pvrtc";
pub const IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;

pub const IMG_RELAXED_LINE_RASTERIZATION_NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";
pub const IMG_RELAXED_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

pub const INTEL_PERFORMANCE_QUERY_NAME: &CStr = c"VK_INTEL_performance_query";
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;

pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_NAME: &CStr = c"VK_INTEL_shader_integer_functions2";
pub const INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;

pub const KHR_16BIT_STORAGE_NAME: &CStr = c"VK_KHR_16bit_storage";
pub const KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;

pub const KHR_8BIT_STORAGE_NAME: &CStr = c"VK_KHR_8bit_storage";
pub const KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;

pub const KHR_ACCELERATION_STRUCTURE_NAME: &CStr = c"VK_KHR_acceleration_structure";
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;

#[cfg(feature = "android")]
pub const KHR_ANDROID_SURFACE_NAME: &CStr = c"VK_KHR_android_surface";
#[cfg(feature = "android")]
pub const KHR_ANDROID_SURFACE_SPEC_VERSION: u32 = 6;

pub const KHR_BIND_MEMORY_2_NAME: &CStr = c"VK_KHR_bind_memory2";
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;

pub const KHR_BUFFER_DEVICE_ADDRESS_NAME: &CStr = c"VK_KHR_buffer_device_address";
pub const KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;

pub const KHR_CALIBRATED_TIMESTAMPS_NAME: &CStr = c"VK_KHR_calibrated_timestamps";
pub const KHR_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 1;

pub const KHR_COMPUTE_SHADER_DERIVATIVES_NAME: &CStr = c"VK_KHR_compute_shader_derivatives";
pub const KHR_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;

pub const KHR_COOPERATIVE_MATRIX_NAME: &CStr = c"VK_KHR_cooperative_matrix";
pub const KHR_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 2;

pub const KHR_COPY_COMMANDS_2_NAME: &CStr = c"VK_KHR_copy_commands2";
pub const KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;

pub const KHR_COPY_MEMORY_INDIRECT_NAME: &CStr = c"VK_KHR_copy_memory_indirect";
pub const KHR_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;

pub const KHR_CREATE_RENDERPASS_2_NAME: &CStr = c"VK_KHR_create_renderpass2";
pub const KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;

pub const KHR_DEDICATED_ALLOCATION_NAME: &CStr = c"VK_KHR_dedicated_allocation";
pub const KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;

pub const KHR_DEFERRED_HOST_OPERATIONS_NAME: &CStr = c"VK_KHR_deferred_host_operations";
pub const KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;

pub const KHR_DEPTH_CLAMP_ZERO_ONE_NAME: &CStr = c"VK_KHR_depth_clamp_zero_one";
pub const KHR_DEPTH_CLAMP_ZERO_ONE_SPEC_VERSION: u32 = 1;

pub const KHR_DEPTH_STENCIL_RESOLVE_NAME: &CStr = c"VK_KHR_depth_stencil_resolve";
pub const KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;

pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_NAME: &CStr = c"VK_KHR_descriptor_update_template";
pub const KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;

pub const KHR_DEVICE_ADDRESS_COMMANDS_NAME: &CStr = c"VK_KHR_device_address_commands";
pub const KHR_DEVICE_ADDRESS_COMMANDS_SPEC_VERSION: u32 = 1;

pub const KHR_DEVICE_FAULT_NAME: &CStr = c"VK_KHR_device_fault";
pub const KHR_DEVICE_FAULT_SPEC_VERSION: u32 = 1;

pub const KHR_DEVICE_GROUP_NAME: &CStr = c"VK_KHR_device_group";
pub const KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;

pub const KHR_DEVICE_GROUP_CREATION_NAME: &CStr = c"VK_KHR_device_group_creation";
pub const KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;

pub const KHR_DISPLAY_NAME: &CStr = c"VK_KHR_display";
pub const KHR_DISPLAY_SPEC_VERSION: u32 = 23;

pub const KHR_DISPLAY_SWAPCHAIN_NAME: &CStr = c"VK_KHR_display_swapchain";
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;

pub const KHR_DRAW_INDIRECT_COUNT_NAME: &CStr = c"VK_KHR_draw_indirect_count";
pub const KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;

pub const KHR_DRIVER_PROPERTIES_NAME: &CStr = c"VK_KHR_driver_properties";
pub const KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;

pub const KHR_DYNAMIC_RENDERING_NAME: &CStr = c"VK_KHR_dynamic_rendering";
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;

pub const KHR_DYNAMIC_RENDERING_LOCAL_READ_NAME: &CStr = c"VK_KHR_dynamic_rendering_local_read";
pub const KHR_DYNAMIC_RENDERING_LOCAL_READ_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_FENCE_NAME: &CStr = c"VK_KHR_external_fence";
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_FENCE_CAPABILITIES_NAME: &CStr = c"VK_KHR_external_fence_capabilities";
pub const KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_FENCE_FD_NAME: &CStr = c"VK_KHR_external_fence_fd";
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_FENCE_WIN32_NAME: &CStr = c"VK_KHR_external_fence_win32";
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_MEMORY_NAME: &CStr = c"VK_KHR_external_memory";
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_NAME: &CStr = c"VK_KHR_external_memory_capabilities";
pub const KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_MEMORY_FD_NAME: &CStr = c"VK_KHR_external_memory_fd";
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_MEMORY_WIN32_NAME: &CStr = c"VK_KHR_external_memory_win32";
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_SEMAPHORE_NAME: &CStr = c"VK_KHR_external_semaphore";
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_NAME: &CStr =
    c"VK_KHR_external_semaphore_capabilities";
pub const KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;

pub const KHR_EXTERNAL_SEMAPHORE_FD_NAME: &CStr = c"VK_KHR_external_semaphore_fd";
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_NAME: &CStr = c"VK_KHR_external_semaphore_win32";
#[cfg(feature = "win32")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;

pub const KHR_FORMAT_FEATURE_FLAGS_2_NAME: &CStr = c"VK_KHR_format_feature_flags2";
pub const KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 2;

pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_NAME: &CStr = c"VK_KHR_fragment_shader_barycentric";
pub const KHR_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;

pub const KHR_FRAGMENT_SHADING_RATE_NAME: &CStr = c"VK_KHR_fragment_shading_rate";
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;

pub const KHR_GET_DISPLAY_PROPERTIES_2_NAME: &CStr = c"VK_KHR_get_display_properties2";
pub const KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;

pub const KHR_GET_MEMORY_REQUIREMENTS_2_NAME: &CStr = c"VK_KHR_get_memory_requirements2";
pub const KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;

pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_NAME: &CStr =
    c"VK_KHR_get_physical_device_properties2";
pub const KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;

pub const KHR_GET_SURFACE_CAPABILITIES_2_NAME: &CStr = c"VK_KHR_get_surface_capabilities2";
pub const KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;

pub const KHR_GLOBAL_PRIORITY_NAME: &CStr = c"VK_KHR_global_priority";
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;

pub const KHR_IMAGE_FORMAT_LIST_NAME: &CStr = c"VK_KHR_image_format_list";
pub const KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;

pub const KHR_IMAGELESS_FRAMEBUFFER_NAME: &CStr = c"VK_KHR_imageless_framebuffer";
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;

pub const KHR_INCREMENTAL_PRESENT_NAME: &CStr = c"VK_KHR_incremental_present";
pub const KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;

pub const KHR_INDEX_TYPE_UINT8_NAME: &CStr = c"VK_KHR_index_type_uint8";
pub const KHR_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;

pub const KHR_INTERNALLY_SYNCHRONIZED_QUEUES_NAME: &CStr = c"VK_KHR_internally_synchronized_queues";
pub const KHR_INTERNALLY_SYNCHRONIZED_QUEUES_SPEC_VERSION: u32 = 1;

pub const KHR_LINE_RASTERIZATION_NAME: &CStr = c"VK_KHR_line_rasterization";
pub const KHR_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;

pub const KHR_LOAD_STORE_OP_NONE_NAME: &CStr = c"VK_KHR_load_store_op_none";
pub const KHR_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_maintenance1";
pub const KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;

pub const KHR_MAINTENANCE_10_NAME: &CStr = c"VK_KHR_maintenance10";
pub const KHR_MAINTENANCE_10_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_11_NAME: &CStr = c"VK_KHR_maintenance11";
pub const KHR_MAINTENANCE_11_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_2_NAME: &CStr = c"VK_KHR_maintenance2";
pub const KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_3_NAME: &CStr = c"VK_KHR_maintenance3";
pub const KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_4_NAME: &CStr = c"VK_KHR_maintenance4";
pub const KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;

pub const KHR_MAINTENANCE_5_NAME: &CStr = c"VK_KHR_maintenance5";
pub const KHR_MAINTENANCE_5_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_6_NAME: &CStr = c"VK_KHR_maintenance6";
pub const KHR_MAINTENANCE_6_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_7_NAME: &CStr = c"VK_KHR_maintenance7";
pub const KHR_MAINTENANCE_7_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_8_NAME: &CStr = c"VK_KHR_maintenance8";
pub const KHR_MAINTENANCE_8_SPEC_VERSION: u32 = 1;

pub const KHR_MAINTENANCE_9_NAME: &CStr = c"VK_KHR_maintenance9";
pub const KHR_MAINTENANCE_9_SPEC_VERSION: u32 = 1;

pub const KHR_MAP_MEMORY_2_NAME: &CStr = c"VK_KHR_map_memory2";
pub const KHR_MAP_MEMORY_2_SPEC_VERSION: u32 = 1;

pub const KHR_MULTIVIEW_NAME: &CStr = c"VK_KHR_multiview";
pub const KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;

pub const KHR_PERFORMANCE_QUERY_NAME: &CStr = c"VK_KHR_performance_query";
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;

pub const KHR_PIPELINE_BINARY_NAME: &CStr = c"VK_KHR_pipeline_binary";
pub const KHR_PIPELINE_BINARY_SPEC_VERSION: u32 = 1;

pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_NAME: &CStr = c"VK_KHR_pipeline_executable_properties";
pub const KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;

pub const KHR_PIPELINE_LIBRARY_NAME: &CStr = c"VK_KHR_pipeline_library";
pub const KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;

pub const KHR_PORTABILITY_ENUMERATION_NAME: &CStr = c"VK_KHR_portability_enumeration";
pub const KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;

#[cfg(feature = "beta")]
pub const KHR_PORTABILITY_SUBSET_NAME: &CStr = c"VK_KHR_portability_subset";
#[cfg(feature = "beta")]
pub const KHR_PORTABILITY_SUBSET_SPEC_VERSION: u32 = 1;

pub const KHR_PRESENT_ID_NAME: &CStr = c"VK_KHR_present_id";
pub const KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;

pub const KHR_PRESENT_ID_2_NAME: &CStr = c"VK_KHR_present_id2";
pub const KHR_PRESENT_ID_2_SPEC_VERSION: u32 = 1;

pub const KHR_PRESENT_MODE_FIFO_LATEST_READY_NAME: &CStr = c"VK_KHR_present_mode_fifo_latest_ready";
pub const KHR_PRESENT_MODE_FIFO_LATEST_READY_SPEC_VERSION: u32 = 1;

pub const KHR_PRESENT_WAIT_NAME: &CStr = c"VK_KHR_present_wait";
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;

pub const KHR_PRESENT_WAIT_2_NAME: &CStr = c"VK_KHR_present_wait2";
pub const KHR_PRESENT_WAIT_2_SPEC_VERSION: u32 = 1;

pub const KHR_PUSH_DESCRIPTOR_NAME: &CStr = c"VK_KHR_push_descriptor";
pub const KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;

pub const KHR_RAY_QUERY_NAME: &CStr = c"VK_KHR_ray_query";
pub const KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;

pub const KHR_RAY_TRACING_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_ray_tracing_maintenance1";
pub const KHR_RAY_TRACING_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const KHR_RAY_TRACING_PIPELINE_NAME: &CStr = c"VK_KHR_ray_tracing_pipeline";
pub const KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;

pub const KHR_RAY_TRACING_POSITION_FETCH_NAME: &CStr = c"VK_KHR_ray_tracing_position_fetch";
pub const KHR_RAY_TRACING_POSITION_FETCH_SPEC_VERSION: u32 = 1;

pub const KHR_RELAXED_BLOCK_LAYOUT_NAME: &CStr = c"VK_KHR_relaxed_block_layout";
pub const KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;

pub const KHR_ROBUSTNESS_2_NAME: &CStr = c"VK_KHR_robustness2";
pub const KHR_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;

pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_NAME: &CStr = c"VK_KHR_sampler_mirror_clamp_to_edge";
pub const KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;

pub const KHR_SAMPLER_YCBCR_CONVERSION_NAME: &CStr = c"VK_KHR_sampler_ycbcr_conversion";
pub const KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;

pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_NAME: &CStr = c"VK_KHR_separate_depth_stencil_layouts";
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_ABORT_NAME: &CStr = c"VK_KHR_shader_abort";
pub const KHR_SHADER_ABORT_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_ATOMIC_INT64_NAME: &CStr = c"VK_KHR_shader_atomic_int64";
pub const KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_BFLOAT16_NAME: &CStr = c"VK_KHR_shader_bfloat16";
pub const KHR_SHADER_BFLOAT16_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_CLOCK_NAME: &CStr = c"VK_KHR_shader_clock";
pub const KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_CONSTANT_DATA_NAME: &CStr = c"VK_KHR_shader_constant_data";
pub const KHR_SHADER_CONSTANT_DATA_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_DRAW_PARAMETERS_NAME: &CStr = c"VK_KHR_shader_draw_parameters";
pub const KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_EXPECT_ASSUME_NAME: &CStr = c"VK_KHR_shader_expect_assume";
pub const KHR_SHADER_EXPECT_ASSUME_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_FLOAT16_INT8_NAME: &CStr = c"VK_KHR_shader_float16_int8";
pub const KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_FLOAT_CONTROLS_NAME: &CStr = c"VK_KHR_shader_float_controls";
pub const KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;

pub const KHR_SHADER_FLOAT_CONTROLS_2_NAME: &CStr = c"VK_KHR_shader_float_controls2";
pub const KHR_SHADER_FLOAT_CONTROLS_2_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_FMA_NAME: &CStr = c"VK_KHR_shader_fma";
pub const KHR_SHADER_FMA_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_INTEGER_DOT_PRODUCT_NAME: &CStr = c"VK_KHR_shader_integer_dot_product";
pub const KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_MAXIMAL_RECONVERGENCE_NAME: &CStr = c"VK_KHR_shader_maximal_reconvergence";
pub const KHR_SHADER_MAXIMAL_RECONVERGENCE_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_NON_SEMANTIC_INFO_NAME: &CStr = c"VK_KHR_shader_non_semantic_info";
pub const KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_QUAD_CONTROL_NAME: &CStr = c"VK_KHR_shader_quad_control";
pub const KHR_SHADER_QUAD_CONTROL_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_NAME: &CStr =
    c"VK_KHR_shader_relaxed_extended_instruction";
pub const KHR_SHADER_RELAXED_EXTENDED_INSTRUCTION_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_NAME: &CStr = c"VK_KHR_shader_subgroup_extended_types";
pub const KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_SUBGROUP_ROTATE_NAME: &CStr = c"VK_KHR_shader_subgroup_rotate";
pub const KHR_SHADER_SUBGROUP_ROTATE_SPEC_VERSION: u32 = 2;

pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_NAME: &CStr =
    c"VK_KHR_shader_subgroup_uniform_control_flow";
pub const KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_TERMINATE_INVOCATION_NAME: &CStr = c"VK_KHR_shader_terminate_invocation";
pub const KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;

pub const KHR_SHADER_UNTYPED_POINTERS_NAME: &CStr = c"VK_KHR_shader_untyped_pointers";
pub const KHR_SHADER_UNTYPED_POINTERS_SPEC_VERSION: u32 = 1;

pub const KHR_SHARED_PRESENTABLE_IMAGE_NAME: &CStr = c"VK_KHR_shared_presentable_image";
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;

pub const KHR_SPIRV_1_4_NAME: &CStr = c"VK_KHR_spirv_1_4";
pub const KHR_SPIRV_1_4_SPEC_VERSION: u32 = 1;

pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_NAME: &CStr = c"VK_KHR_storage_buffer_storage_class";
pub const KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;

pub const KHR_SURFACE_NAME: &CStr = c"VK_KHR_surface";
pub const KHR_SURFACE_SPEC_VERSION: u32 = 25;

pub const KHR_SURFACE_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_surface_maintenance1";
pub const KHR_SURFACE_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const KHR_SURFACE_PROTECTED_CAPABILITIES_NAME: &CStr = c"VK_KHR_surface_protected_capabilities";
pub const KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;

pub const KHR_SWAPCHAIN_NAME: &CStr = c"VK_KHR_swapchain";
pub const KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;

pub const KHR_SWAPCHAIN_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_swapchain_maintenance1";
pub const KHR_SWAPCHAIN_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_NAME: &CStr = c"VK_KHR_swapchain_mutable_format";
pub const KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;

pub const KHR_SYNCHRONIZATION_2_NAME: &CStr = c"VK_KHR_synchronization2";
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;

pub const KHR_TIMELINE_SEMAPHORE_NAME: &CStr = c"VK_KHR_timeline_semaphore";
pub const KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;

pub const KHR_UNIFIED_IMAGE_LAYOUTS_NAME: &CStr = c"VK_KHR_unified_image_layouts";
pub const KHR_UNIFIED_IMAGE_LAYOUTS_SPEC_VERSION: u32 = 1;

pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_NAME: &CStr = c"VK_KHR_uniform_buffer_standard_layout";
pub const KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;

pub const KHR_VARIABLE_POINTERS_NAME: &CStr = c"VK_KHR_variable_pointers";
pub const KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;

pub const KHR_VERTEX_ATTRIBUTE_DIVISOR_NAME: &CStr = c"VK_KHR_vertex_attribute_divisor";
pub const KHR_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_DECODE_AV1_NAME: &CStr = c"VK_KHR_video_decode_av1";
pub const KHR_VIDEO_DECODE_AV1_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_DECODE_H264_NAME: &CStr = c"VK_KHR_video_decode_h264";
pub const KHR_VIDEO_DECODE_H264_SPEC_VERSION: u32 = 9;

pub const KHR_VIDEO_DECODE_H265_NAME: &CStr = c"VK_KHR_video_decode_h265";
pub const KHR_VIDEO_DECODE_H265_SPEC_VERSION: u32 = 8;

pub const KHR_VIDEO_DECODE_QUEUE_NAME: &CStr = c"VK_KHR_video_decode_queue";
pub const KHR_VIDEO_DECODE_QUEUE_SPEC_VERSION: u32 = 8;

pub const KHR_VIDEO_DECODE_VP9_NAME: &CStr = c"VK_KHR_video_decode_vp9";
pub const KHR_VIDEO_DECODE_VP9_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_ENCODE_AV1_NAME: &CStr = c"VK_KHR_video_encode_av1";
pub const KHR_VIDEO_ENCODE_AV1_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_ENCODE_H264_NAME: &CStr = c"VK_KHR_video_encode_h264";
pub const KHR_VIDEO_ENCODE_H264_SPEC_VERSION: u32 = 14;

pub const KHR_VIDEO_ENCODE_H265_NAME: &CStr = c"VK_KHR_video_encode_h265";
pub const KHR_VIDEO_ENCODE_H265_SPEC_VERSION: u32 = 14;

pub const KHR_VIDEO_ENCODE_INTRA_REFRESH_NAME: &CStr = c"VK_KHR_video_encode_intra_refresh";
pub const KHR_VIDEO_ENCODE_INTRA_REFRESH_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_ENCODE_QUANTIZATION_MAP_NAME: &CStr = c"VK_KHR_video_encode_quantization_map";
pub const KHR_VIDEO_ENCODE_QUANTIZATION_MAP_SPEC_VERSION: u32 = 2;

pub const KHR_VIDEO_ENCODE_QUEUE_NAME: &CStr = c"VK_KHR_video_encode_queue";
pub const KHR_VIDEO_ENCODE_QUEUE_SPEC_VERSION: u32 = 12;

pub const KHR_VIDEO_MAINTENANCE_1_NAME: &CStr = c"VK_KHR_video_maintenance1";
pub const KHR_VIDEO_MAINTENANCE_1_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_MAINTENANCE_2_NAME: &CStr = c"VK_KHR_video_maintenance2";
pub const KHR_VIDEO_MAINTENANCE_2_SPEC_VERSION: u32 = 1;

pub const KHR_VIDEO_QUEUE_NAME: &CStr = c"VK_KHR_video_queue";
pub const KHR_VIDEO_QUEUE_SPEC_VERSION: u32 = 8;

pub const KHR_VULKAN_MEMORY_MODEL_NAME: &CStr = c"VK_KHR_vulkan_memory_model";
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;

#[cfg(feature = "wayland")]
pub const KHR_WAYLAND_SURFACE_NAME: &CStr = c"VK_KHR_wayland_surface";
#[cfg(feature = "wayland")]
pub const KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;

#[cfg(feature = "win32")]
pub const KHR_WIN32_KEYED_MUTEX_NAME: &CStr = c"VK_KHR_win32_keyed_mutex";
#[cfg(feature = "win32")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const KHR_WIN32_SURFACE_NAME: &CStr = c"VK_KHR_win32_surface";
#[cfg(feature = "win32")]
pub const KHR_WIN32_SURFACE_SPEC_VERSION: u32 = 6;

pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_NAME: &CStr =
    c"VK_KHR_workgroup_memory_explicit_layout";
pub const KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;

#[cfg(feature = "xcb")]
pub const KHR_XCB_SURFACE_NAME: &CStr = c"VK_KHR_xcb_surface";
#[cfg(feature = "xcb")]
pub const KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;

#[cfg(feature = "xlib")]
pub const KHR_XLIB_SURFACE_NAME: &CStr = c"VK_KHR_xlib_surface";
#[cfg(feature = "xlib")]
pub const KHR_XLIB_SURFACE_SPEC_VERSION: u32 = 6;

pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_NAME: &CStr =
    c"VK_KHR_zero_initialize_workgroup_memory";
pub const KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;

pub const LUNARG_DIRECT_DRIVER_LOADING_NAME: &CStr = c"VK_LUNARG_direct_driver_loading";
pub const LUNARG_DIRECT_DRIVER_LOADING_SPEC_VERSION: u32 = 1;

pub const MESA_IMAGE_ALIGNMENT_CONTROL_NAME: &CStr = c"VK_MESA_image_alignment_control";
pub const MESA_IMAGE_ALIGNMENT_CONTROL_SPEC_VERSION: u32 = 1;

pub const MSFT_LAYERED_DRIVER_NAME: &CStr = c"VK_MSFT_layered_driver";
pub const MSFT_LAYERED_DRIVER_SPEC_VERSION: u32 = 1;

#[cfg(feature = "ios")]
pub const MVK_IOS_SURFACE_NAME: &CStr = c"VK_MVK_ios_surface";
#[cfg(feature = "ios")]
pub const MVK_IOS_SURFACE_SPEC_VERSION: u32 = 3;

#[cfg(feature = "macos")]
pub const MVK_MACOS_SURFACE_NAME: &CStr = c"VK_MVK_macos_surface";
#[cfg(feature = "macos")]
pub const MVK_MACOS_SURFACE_SPEC_VERSION: u32 = 3;

#[cfg(feature = "vi")]
pub const NN_VI_SURFACE_NAME: &CStr = c"VK_NN_vi_surface";
#[cfg(feature = "vi")]
pub const NN_VI_SURFACE_SPEC_VERSION: u32 = 1;

pub const NVX_BINARY_IMPORT_NAME: &CStr = c"VK_NVX_binary_import";
pub const NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 2;

pub const NVX_IMAGE_VIEW_HANDLE_NAME: &CStr = c"VK_NVX_image_view_handle";
pub const NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 4;

pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_NAME: &CStr = c"VK_NVX_multiview_per_view_attributes";
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const NV_ACQUIRE_WINRT_DISPLAY_NAME: &CStr = c"VK_NV_acquire_winrt_display";
#[cfg(feature = "win32")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;

pub const NV_CLIP_SPACE_W_SCALING_NAME: &CStr = c"VK_NV_clip_space_w_scaling";
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;

pub const NV_CLUSTER_ACCELERATION_STRUCTURE_NAME: &CStr = c"VK_NV_cluster_acceleration_structure";
pub const NV_CLUSTER_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 4;

pub const NV_COMMAND_BUFFER_INHERITANCE_NAME: &CStr = c"VK_NV_command_buffer_inheritance";
pub const NV_COMMAND_BUFFER_INHERITANCE_SPEC_VERSION: u32 = 1;

pub const NV_COMPUTE_OCCUPANCY_PRIORITY_NAME: &CStr = c"VK_NV_compute_occupancy_priority";
pub const NV_COMPUTE_OCCUPANCY_PRIORITY_SPEC_VERSION: u32 = 1;

pub const NV_COMPUTE_SHADER_DERIVATIVES_NAME: &CStr = c"VK_NV_compute_shader_derivatives";
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;

pub const NV_COOPERATIVE_MATRIX_NAME: &CStr = c"VK_NV_cooperative_matrix";
pub const NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;

pub const NV_COOPERATIVE_MATRIX_2_NAME: &CStr = c"VK_NV_cooperative_matrix2";
pub const NV_COOPERATIVE_MATRIX_2_SPEC_VERSION: u32 = 1;

pub const NV_COOPERATIVE_VECTOR_NAME: &CStr = c"VK_NV_cooperative_vector";
pub const NV_COOPERATIVE_VECTOR_SPEC_VERSION: u32 = 4;

pub const NV_COPY_MEMORY_INDIRECT_NAME: &CStr = c"VK_NV_copy_memory_indirect";
pub const NV_COPY_MEMORY_INDIRECT_SPEC_VERSION: u32 = 1;

pub const NV_CORNER_SAMPLED_IMAGE_NAME: &CStr = c"VK_NV_corner_sampled_image";
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;

pub const NV_COVERAGE_REDUCTION_MODE_NAME: &CStr = c"VK_NV_coverage_reduction_mode";
pub const NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;

#[cfg(feature = "beta")]
pub const NV_CUDA_KERNEL_LAUNCH_NAME: &CStr = c"VK_NV_cuda_kernel_launch";
#[cfg(feature = "beta")]
pub const NV_CUDA_KERNEL_LAUNCH_SPEC_VERSION: u32 = 2;

pub const NV_DEDICATED_ALLOCATION_NAME: &CStr = c"VK_NV_dedicated_allocation";
pub const NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;

pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_NAME: &CStr =
    c"VK_NV_dedicated_allocation_image_aliasing";
pub const NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;

pub const NV_DESCRIPTOR_POOL_OVERALLOCATION_NAME: &CStr = c"VK_NV_descriptor_pool_overallocation";
pub const NV_DESCRIPTOR_POOL_OVERALLOCATION_SPEC_VERSION: u32 = 1;

pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_NAME: &CStr = c"VK_NV_device_diagnostic_checkpoints";
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;

pub const NV_DEVICE_DIAGNOSTICS_CONFIG_NAME: &CStr = c"VK_NV_device_diagnostics_config";
pub const NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 2;

pub const NV_DEVICE_GENERATED_COMMANDS_NAME: &CStr = c"VK_NV_device_generated_commands";
pub const NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;

pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE_NAME: &CStr =
    c"VK_NV_device_generated_commands_compute";
pub const NV_DEVICE_GENERATED_COMMANDS_COMPUTE_SPEC_VERSION: u32 = 2;

#[cfg(feature = "beta")]
pub const NV_DISPLACEMENT_MICROMAP_NAME: &CStr = c"VK_NV_displacement_micromap";
#[cfg(feature = "beta")]
pub const NV_DISPLACEMENT_MICROMAP_SPEC_VERSION: u32 = 2;

pub const NV_DISPLAY_STEREO_NAME: &CStr = c"VK_NV_display_stereo";
pub const NV_DISPLAY_STEREO_SPEC_VERSION: u32 = 1;

pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE_NAME: &CStr = c"VK_NV_extended_sparse_address_space";
pub const NV_EXTENDED_SPARSE_ADDRESS_SPACE_SPEC_VERSION: u32 = 1;

pub const NV_EXTERNAL_COMPUTE_QUEUE_NAME: &CStr = c"VK_NV_external_compute_queue";
pub const NV_EXTERNAL_COMPUTE_QUEUE_SPEC_VERSION: u32 = 1;

pub const NV_EXTERNAL_MEMORY_NAME: &CStr = c"VK_NV_external_memory";
pub const NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

pub const NV_EXTERNAL_MEMORY_CAPABILITIES_NAME: &CStr = c"VK_NV_external_memory_capabilities";
pub const NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;

pub const NV_EXTERNAL_MEMORY_RDMA_NAME: &CStr = c"VK_NV_external_memory_rdma";
pub const NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const NV_EXTERNAL_MEMORY_WIN32_NAME: &CStr = c"VK_NV_external_memory_win32";
#[cfg(feature = "win32")]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;

pub const NV_FILL_RECTANGLE_NAME: &CStr = c"VK_NV_fill_rectangle";
pub const NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;

pub const NV_FRAGMENT_COVERAGE_TO_COLOR_NAME: &CStr = c"VK_NV_fragment_coverage_to_color";
pub const NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;

pub const NV_FRAGMENT_SHADER_BARYCENTRIC_NAME: &CStr = c"VK_NV_fragment_shader_barycentric";
pub const NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;

pub const NV_FRAGMENT_SHADING_RATE_ENUMS_NAME: &CStr = c"VK_NV_fragment_shading_rate_enums";
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;

pub const NV_FRAMEBUFFER_MIXED_SAMPLES_NAME: &CStr = c"VK_NV_framebuffer_mixed_samples";
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;

pub const NV_GEOMETRY_SHADER_PASSTHROUGH_NAME: &CStr = c"VK_NV_geometry_shader_passthrough";
pub const NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;

pub const NV_GLSL_SHADER_NAME: &CStr = c"VK_NV_glsl_shader";
pub const NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;

pub const NV_INHERITED_VIEWPORT_SCISSOR_NAME: &CStr = c"VK_NV_inherited_viewport_scissor";
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;

pub const NV_LINEAR_COLOR_ATTACHMENT_NAME: &CStr = c"VK_NV_linear_color_attachment";
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;

pub const NV_LOW_LATENCY_NAME: &CStr = c"VK_NV_low_latency";
pub const NV_LOW_LATENCY_SPEC_VERSION: u32 = 1;

pub const NV_LOW_LATENCY_2_NAME: &CStr = c"VK_NV_low_latency2";
pub const NV_LOW_LATENCY_2_SPEC_VERSION: u32 = 2;

pub const NV_MEMORY_DECOMPRESSION_NAME: &CStr = c"VK_NV_memory_decompression";
pub const NV_MEMORY_DECOMPRESSION_SPEC_VERSION: u32 = 1;

pub const NV_MESH_SHADER_NAME: &CStr = c"VK_NV_mesh_shader";
pub const NV_MESH_SHADER_SPEC_VERSION: u32 = 1;

pub const NV_OPTICAL_FLOW_NAME: &CStr = c"VK_NV_optical_flow";
pub const NV_OPTICAL_FLOW_SPEC_VERSION: u32 = 1;

pub const NV_PARTITIONED_ACCELERATION_STRUCTURE_NAME: &CStr =
    c"VK_NV_partitioned_acceleration_structure";
pub const NV_PARTITIONED_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 1;

pub const NV_PER_STAGE_DESCRIPTOR_SET_NAME: &CStr = c"VK_NV_per_stage_descriptor_set";
pub const NV_PER_STAGE_DESCRIPTOR_SET_SPEC_VERSION: u32 = 1;

pub const NV_PRESENT_BARRIER_NAME: &CStr = c"VK_NV_present_barrier";
pub const NV_PRESENT_BARRIER_SPEC_VERSION: u32 = 1;

pub const NV_PRESENT_METERING_NAME: &CStr = c"VK_NV_present_metering";
pub const NV_PRESENT_METERING_SPEC_VERSION: u32 = 1;

pub const NV_PUSH_CONSTANT_BANK_NAME: &CStr = c"VK_NV_push_constant_bank";
pub const NV_PUSH_CONSTANT_BANK_SPEC_VERSION: u32 = 1;

pub const NV_RAW_ACCESS_CHAINS_NAME: &CStr = c"VK_NV_raw_access_chains";
pub const NV_RAW_ACCESS_CHAINS_SPEC_VERSION: u32 = 1;

pub const NV_RAY_TRACING_NAME: &CStr = c"VK_NV_ray_tracing";
pub const NV_RAY_TRACING_SPEC_VERSION: u32 = 3;

pub const NV_RAY_TRACING_INVOCATION_REORDER_NAME: &CStr = c"VK_NV_ray_tracing_invocation_reorder";
pub const NV_RAY_TRACING_INVOCATION_REORDER_SPEC_VERSION: u32 = 1;

pub const NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_NAME: &CStr =
    c"VK_NV_ray_tracing_linear_swept_spheres";
pub const NV_RAY_TRACING_LINEAR_SWEPT_SPHERES_SPEC_VERSION: u32 = 1;

pub const NV_RAY_TRACING_MOTION_BLUR_NAME: &CStr = c"VK_NV_ray_tracing_motion_blur";
pub const NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;

pub const NV_RAY_TRACING_VALIDATION_NAME: &CStr = c"VK_NV_ray_tracing_validation";
pub const NV_RAY_TRACING_VALIDATION_SPEC_VERSION: u32 = 1;

pub const NV_REPRESENTATIVE_FRAGMENT_TEST_NAME: &CStr = c"VK_NV_representative_fragment_test";
pub const NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;

pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_NAME: &CStr = c"VK_NV_sample_mask_override_coverage";
pub const NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;

pub const NV_SCISSOR_EXCLUSIVE_NAME: &CStr = c"VK_NV_scissor_exclusive";
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 2;

pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR_NAME: &CStr = c"VK_NV_shader_atomic_float16_vector";
pub const NV_SHADER_ATOMIC_FLOAT16_VECTOR_SPEC_VERSION: u32 = 1;

pub const NV_SHADER_IMAGE_FOOTPRINT_NAME: &CStr = c"VK_NV_shader_image_footprint";
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;

pub const NV_SHADER_SM_BUILTINS_NAME: &CStr = c"VK_NV_shader_sm_builtins";
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;

pub const NV_SHADER_SUBGROUP_PARTITIONED_NAME: &CStr = c"VK_NV_shader_subgroup_partitioned";
pub const NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;

pub const NV_SHADING_RATE_IMAGE_NAME: &CStr = c"VK_NV_shading_rate_image";
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;

pub const NV_VIEWPORT_ARRAY_2_NAME: &CStr = c"VK_NV_viewport_array2";
pub const NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;

pub const NV_VIEWPORT_SWIZZLE_NAME: &CStr = c"VK_NV_viewport_swizzle";
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;

#[cfg(feature = "win32")]
pub const NV_WIN32_KEYED_MUTEX_NAME: &CStr = c"VK_NV_win32_keyed_mutex";
#[cfg(feature = "win32")]
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;

#[cfg(feature = "ohos")]
pub const OHOS_EXTERNAL_MEMORY_NAME: &CStr = c"VK_OHOS_external_memory";
#[cfg(feature = "ohos")]
pub const OHOS_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;

#[cfg(feature = "ohos")]
pub const OHOS_SURFACE_NAME: &CStr = c"VK_OHOS_surface";
#[cfg(feature = "ohos")]
pub const OHOS_SURFACE_SPEC_VERSION: u32 = 1;

pub const QCOM_COOPERATIVE_MATRIX_CONVERSION_NAME: &CStr = c"VK_QCOM_cooperative_matrix_conversion";
pub const QCOM_COOPERATIVE_MATRIX_CONVERSION_SPEC_VERSION: u32 = 1;

pub const QCOM_DATA_GRAPH_MODEL_NAME: &CStr = c"VK_QCOM_data_graph_model";
pub const QCOM_DATA_GRAPH_MODEL_SPEC_VERSION: u32 = 1;

pub const QCOM_FILTER_CUBIC_CLAMP_NAME: &CStr = c"VK_QCOM_filter_cubic_clamp";
pub const QCOM_FILTER_CUBIC_CLAMP_SPEC_VERSION: u32 = 1;

pub const QCOM_FILTER_CUBIC_WEIGHTS_NAME: &CStr = c"VK_QCOM_filter_cubic_weights";
pub const QCOM_FILTER_CUBIC_WEIGHTS_SPEC_VERSION: u32 = 1;

pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_NAME: &CStr = c"VK_QCOM_fragment_density_map_offset";
pub const QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 3;

pub const QCOM_IMAGE_PROCESSING_NAME: &CStr = c"VK_QCOM_image_processing";
pub const QCOM_IMAGE_PROCESSING_SPEC_VERSION: u32 = 1;

pub const QCOM_IMAGE_PROCESSING_2_NAME: &CStr = c"VK_QCOM_image_processing2";
pub const QCOM_IMAGE_PROCESSING_2_SPEC_VERSION: u32 = 1;

pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_NAME: &CStr =
    c"VK_QCOM_multiview_per_view_render_areas";
pub const QCOM_MULTIVIEW_PER_VIEW_RENDER_AREAS_SPEC_VERSION: u32 = 1;

pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_NAME: &CStr = c"VK_QCOM_multiview_per_view_viewports";
pub const QCOM_MULTIVIEW_PER_VIEW_VIEWPORTS_SPEC_VERSION: u32 = 1;

pub const QCOM_QUEUE_PERF_HINT_NAME: &CStr = c"VK_QCOM_queue_perf_hint";
pub const QCOM_QUEUE_PERF_HINT_SPEC_VERSION: u32 = 1;

pub const QCOM_RENDER_PASS_SHADER_RESOLVE_NAME: &CStr = c"VK_QCOM_render_pass_shader_resolve";
pub const QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION: u32 = 4;

pub const QCOM_RENDER_PASS_STORE_OPS_NAME: &CStr = c"VK_QCOM_render_pass_store_ops";
pub const QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION: u32 = 2;

pub const QCOM_RENDER_PASS_TRANSFORM_NAME: &CStr = c"VK_QCOM_render_pass_transform";
pub const QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 5;

pub const QCOM_ROTATED_COPY_COMMANDS_NAME: &CStr = c"VK_QCOM_rotated_copy_commands";
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 2;

pub const QCOM_TILE_MEMORY_HEAP_NAME: &CStr = c"VK_QCOM_tile_memory_heap";
pub const QCOM_TILE_MEMORY_HEAP_SPEC_VERSION: u32 = 1;

pub const QCOM_TILE_PROPERTIES_NAME: &CStr = c"VK_QCOM_tile_properties";
pub const QCOM_TILE_PROPERTIES_SPEC_VERSION: u32 = 1;

pub const QCOM_TILE_SHADING_NAME: &CStr = c"VK_QCOM_tile_shading";
pub const QCOM_TILE_SHADING_SPEC_VERSION: u32 = 2;

pub const QCOM_YCBCR_DEGAMMA_NAME: &CStr = c"VK_QCOM_ycbcr_degamma";
pub const QCOM_YCBCR_DEGAMMA_SPEC_VERSION: u32 = 1;

#[cfg(feature = "screen")]
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_NAME: &CStr = c"VK_QNX_external_memory_screen_buffer";
#[cfg(feature = "screen")]
pub const QNX_EXTERNAL_MEMORY_SCREEN_BUFFER_SPEC_VERSION: u32 = 1;

#[cfg(feature = "screen")]
pub const QNX_SCREEN_SURFACE_NAME: &CStr = c"VK_QNX_screen_surface";
#[cfg(feature = "screen")]
pub const QNX_SCREEN_SURFACE_SPEC_VERSION: u32 = 1;

pub const SEC_AMIGO_PROFILING_NAME: &CStr = c"VK_SEC_amigo_profiling";
pub const SEC_AMIGO_PROFILING_SPEC_VERSION: u32 = 1;

pub const SEC_PIPELINE_CACHE_INCREMENTAL_MODE_NAME: &CStr =
    c"VK_SEC_pipeline_cache_incremental_mode";
pub const SEC_PIPELINE_CACHE_INCREMENTAL_MODE_SPEC_VERSION: u32 = 1;

pub const SEC_THROTTLE_HINT_NAME: &CStr = c"VK_SEC_throttle_hint";
pub const SEC_THROTTLE_HINT_SPEC_VERSION: u32 = 1;

#[cfg(feature = "ubm")]
pub const SEC_UBM_SURFACE_NAME: &CStr = c"VK_SEC_ubm_surface";
#[cfg(feature = "ubm")]
pub const SEC_UBM_SURFACE_SPEC_VERSION: u32 = 1;

pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_NAME: &CStr = c"VK_VALVE_descriptor_set_host_mapping";
pub const VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;

pub const VALVE_FRAGMENT_DENSITY_MAP_LAYERED_NAME: &CStr = c"VK_VALVE_fragment_density_map_layered";
pub const VALVE_FRAGMENT_DENSITY_MAP_LAYERED_SPEC_VERSION: u32 = 1;

pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_NAME: &CStr = c"VK_VALVE_mutable_descriptor_type";
pub const VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;

pub const VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_NAME: &CStr =
    c"VK_VALVE_shader_mixed_float_dot_product";
pub const VALVE_SHADER_MIXED_FLOAT_DOT_PRODUCT_SPEC_VERSION: u32 = 1;

pub const VALVE_VIDEO_ENCODE_RGB_CONVERSION_NAME: &CStr = c"VK_VALVE_video_encode_rgb_conversion";
pub const VALVE_VIDEO_ENCODE_RGB_CONVERSION_SPEC_VERSION: u32 = 1;
