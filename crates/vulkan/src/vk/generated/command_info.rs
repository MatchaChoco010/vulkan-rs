#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

/// One [`CommandInfo`] constant for each command of the registry, named after the method that calls the command.
pub mod command {
    use crate::vk::*;

    /// What the registry states about the C command [`vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html).
    pub const ACQUIRE_DRM_DISPLAY_EXT: CommandInfo = CommandInfo {
        name: "vkAcquireDrmDisplayEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_acquire_drm_display"],
    };
    /// What the registry states about the C command [`vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html).
    pub const ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkAcquireFullScreenExclusiveModeEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_full_screen_exclusive"],
    };
    /// What the registry states about the C command [`vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html).
    pub const ACQUIRE_NEXT_IMAGE2_KHR: CommandInfo = CommandInfo {
        name: "vkAcquireNextImage2KHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::TIMEOUT,
            VkResult::NOT_READY,
            VkResult::SUBOPTIMAL_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain", c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html).
    pub const ACQUIRE_NEXT_IMAGE_KHR: CommandInfo = CommandInfo {
        name: "vkAcquireNextImageKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::TIMEOUT,
            VkResult::NOT_READY,
            VkResult::SUBOPTIMAL_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain"],
    };
    /// What the registry states about the C command [`vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html).
    pub const ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL: CommandInfo = CommandInfo {
        name: "vkAcquirePerformanceConfigurationINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html).
    pub const ACQUIRE_PROFILING_LOCK_KHR: CommandInfo = CommandInfo {
        name: "vkAcquireProfilingLockKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::TIMEOUT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_performance_query"],
    };
    /// What the registry states about the C command [`vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html).
    pub const ACQUIRE_WINRT_DISPLAY_NV: CommandInfo = CommandInfo {
        name: "vkAcquireWinrtDisplayNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_acquire_winrt_display"],
    };
    /// What the registry states about the C command [`vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html).
    pub const ACQUIRE_XLIB_DISPLAY_EXT: CommandInfo = CommandInfo {
        name: "vkAcquireXlibDisplayEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_acquire_xlib_display"],
    };
    /// What the registry states about the C command [`vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html).
    pub const ALLOCATE_COMMAND_BUFFERS: CommandInfo = CommandInfo {
        name: "vkAllocateCommandBuffers",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html).
    pub const ALLOCATE_DESCRIPTOR_SETS: CommandInfo = CommandInfo {
        name: "vkAllocateDescriptorSets",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FRAGMENTED_POOL,
            VkResult::ERROR_OUT_OF_POOL_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html).
    pub const ALLOCATE_MEMORY: CommandInfo = CommandInfo {
        name: "vkAllocateMemory",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html).
    pub const ANTI_LAG_UPDATE_AMD: CommandInfo = CommandInfo {
        name: "vkAntiLagUpdateAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_anti_lag"],
    };
    /// What the registry states about the C command [`vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html).
    pub const BEGIN_COMMAND_BUFFER: CommandInfo = CommandInfo {
        name: "vkBeginCommandBuffer",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html).
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_NV: CommandInfo = CommandInfo {
        name: "vkBindAccelerationStructureMemoryNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html).
    pub const BIND_BUFFER_MEMORY: CommandInfo = CommandInfo {
        name: "vkBindBufferMemory",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html).
    pub const BIND_BUFFER_MEMORY2: CommandInfo = CommandInfo {
        name: "vkBindBufferMemory2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html).
    pub const BIND_BUFFER_MEMORY2_KHR: CommandInfo = CommandInfo {
        name: "vkBindBufferMemory2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_bind_memory2"],
    };
    /// What the registry states about the C command [`vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html).
    pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_ARM: CommandInfo = CommandInfo {
        name: "vkBindDataGraphPipelineSessionMemoryARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html).
    pub const BIND_IMAGE_MEMORY: CommandInfo = CommandInfo {
        name: "vkBindImageMemory",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html).
    pub const BIND_IMAGE_MEMORY2: CommandInfo = CommandInfo {
        name: "vkBindImageMemory2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html).
    pub const BIND_IMAGE_MEMORY2_KHR: CommandInfo = CommandInfo {
        name: "vkBindImageMemory2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_bind_memory2"],
    };
    /// What the registry states about the C command [`vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html).
    pub const BIND_OPTICAL_FLOW_SESSION_IMAGE_NV: CommandInfo = CommandInfo {
        name: "vkBindOpticalFlowSessionImageNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_optical_flow"],
    };
    /// What the registry states about the C command [`vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html).
    pub const BIND_TENSOR_MEMORY_ARM: CommandInfo = CommandInfo {
        name: "vkBindTensorMemoryARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html).
    pub const BIND_VIDEO_SESSION_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkBindVideoSessionMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html).
    pub const BUILD_ACCELERATION_STRUCTURES_KHR: CommandInfo = CommandInfo {
        name: "vkBuildAccelerationStructuresKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html).
    pub const BUILD_MICROMAPS_EXT: CommandInfo = CommandInfo {
        name: "vkBuildMicromapsEXT",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html).
    pub const CLEAR_SHADER_INSTRUMENTATION_METRICS_ARM: CommandInfo = CommandInfo {
        name: "vkClearShaderInstrumentationMetricsARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html).
    pub const CMD_BEGIN_CONDITIONAL_RENDERING2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginConditionalRendering2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html).
    pub const CMD_BEGIN_CONDITIONAL_RENDERING_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginConditionalRenderingEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_conditional_rendering"],
    };
    /// What the registry states about the C command [`vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html).
    pub const CMD_BEGIN_CUSTOM_RESOLVE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginCustomResolveEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_custom_resolve"],
    };
    /// What the registry states about the C command [`vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html).
    pub const CMD_BEGIN_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html).
    pub const CMD_BEGIN_GPA_SAMPLE_AMD: CommandInfo = CommandInfo {
        name: "vkCmdBeginGpaSampleAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html).
    pub const CMD_BEGIN_GPA_SESSION_AMD: CommandInfo = CommandInfo {
        name: "vkCmdBeginGpaSessionAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html).
    pub const CMD_BEGIN_PER_TILE_EXECUTION_QCOM: CommandInfo = CommandInfo {
        name: "vkCmdBeginPerTileExecutionQCOM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_QCOM_tile_shading"],
    };
    /// What the registry states about the C command [`vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html).
    pub const CMD_BEGIN_QUERY: CommandInfo = CommandInfo {
        name: "vkCmdBeginQuery",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html).
    pub const CMD_BEGIN_QUERY_INDEXED_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginQueryIndexedEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html).
    pub const CMD_BEGIN_RENDER_PASS: CommandInfo = CommandInfo {
        name: "vkCmdBeginRenderPass",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html).
    pub const CMD_BEGIN_RENDER_PASS2: CommandInfo = CommandInfo {
        name: "vkCmdBeginRenderPass2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html).
    pub const CMD_BEGIN_RENDER_PASS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBeginRenderPass2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_create_renderpass2"],
    };
    /// What the registry states about the C command [`vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html).
    pub const CMD_BEGIN_RENDERING: CommandInfo = CommandInfo {
        name: "vkCmdBeginRendering",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html).
    pub const CMD_BEGIN_RENDERING_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBeginRenderingKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_dynamic_rendering"],
    };
    /// What the registry states about the C command [`vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html).
    pub const CMD_BEGIN_SHADER_INSTRUMENTATION_ARM: CommandInfo = CommandInfo {
        name: "vkCmdBeginShaderInstrumentationARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html).
    pub const CMD_BEGIN_TRANSFORM_FEEDBACK2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginTransformFeedback2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html).
    pub const CMD_BEGIN_TRANSFORM_FEEDBACK_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBeginTransformFeedbackEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html).
    pub const CMD_BEGIN_VIDEO_CODING_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBeginVideoCodingKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html).
    pub const CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorBufferEmbeddedSamplers2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html).
    pub const CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorBufferEmbeddedSamplersEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html).
    pub const CMD_BIND_DESCRIPTOR_BUFFERS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorBuffersEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html).
    pub const CMD_BIND_DESCRIPTOR_SETS: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorSets",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html).
    pub const CMD_BIND_DESCRIPTOR_SETS2: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorSets2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html).
    pub const CMD_BIND_DESCRIPTOR_SETS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBindDescriptorSets2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html).
    pub const CMD_BIND_INDEX_BUFFER: CommandInfo = CommandInfo {
        name: "vkCmdBindIndexBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html).
    pub const CMD_BIND_INDEX_BUFFER2: CommandInfo = CommandInfo {
        name: "vkCmdBindIndexBuffer2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html).
    pub const CMD_BIND_INDEX_BUFFER2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBindIndexBuffer2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance5"],
    };
    /// What the registry states about the C command [`vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html).
    pub const CMD_BIND_INDEX_BUFFER3_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBindIndexBuffer3KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html).
    pub const CMD_BIND_INVOCATION_MASK_HUAWEI: CommandInfo = CommandInfo {
        name: "vkCmdBindInvocationMaskHUAWEI",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_HUAWEI_invocation_mask"],
    };
    /// What the registry states about the C command [`vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html).
    pub const CMD_BIND_PIPELINE: CommandInfo = CommandInfo {
        name: "vkCmdBindPipeline",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html).
    pub const CMD_BIND_PIPELINE_SHADER_GROUP_NV: CommandInfo = CommandInfo {
        name: "vkCmdBindPipelineShaderGroupNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html).
    pub const CMD_BIND_RESOURCE_HEAP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindResourceHeapEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html).
    pub const CMD_BIND_SAMPLER_HEAP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindSamplerHeapEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html).
    pub const CMD_BIND_SHADERS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindShadersEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html).
    pub const CMD_BIND_SHADING_RATE_IMAGE_NV: CommandInfo = CommandInfo {
        name: "vkCmdBindShadingRateImageNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_shading_rate_image"],
    };
    /// What the registry states about the C command [`vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html).
    pub const CMD_BIND_TILE_MEMORY_QCOM: CommandInfo = CommandInfo {
        name: "vkCmdBindTileMemoryQCOM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_QCOM_tile_memory_heap"],
    };
    /// What the registry states about the C command [`vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html).
    pub const CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindTransformFeedbackBuffers2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html).
    pub const CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindTransformFeedbackBuffersEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html).
    pub const CMD_BIND_VERTEX_BUFFERS: CommandInfo = CommandInfo {
        name: "vkCmdBindVertexBuffers",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html).
    pub const CMD_BIND_VERTEX_BUFFERS2: CommandInfo = CommandInfo {
        name: "vkCmdBindVertexBuffers2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html).
    pub const CMD_BIND_VERTEX_BUFFERS2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBindVertexBuffers2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html).
    pub const CMD_BIND_VERTEX_BUFFERS3_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBindVertexBuffers3KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html).
    pub const CMD_BLIT_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdBlitImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html).
    pub const CMD_BLIT_IMAGE2: CommandInfo = CommandInfo {
        name: "vkCmdBlitImage2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html).
    pub const CMD_BLIT_IMAGE2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBlitImage2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html).
    pub const CMD_BUILD_ACCELERATION_STRUCTURE_NV: CommandInfo = CommandInfo {
        name: "vkCmdBuildAccelerationStructureNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html).
    pub const CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBuildAccelerationStructuresIndirectKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html).
    pub const CMD_BUILD_ACCELERATION_STRUCTURES_KHR: CommandInfo = CommandInfo {
        name: "vkCmdBuildAccelerationStructuresKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html).
    pub const CMD_BUILD_CLUSTER_ACCELERATION_STRUCTURE_INDIRECT_NV: CommandInfo = CommandInfo {
        name: "vkCmdBuildClusterAccelerationStructureIndirectNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cluster_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html).
    pub const CMD_BUILD_MICROMAPS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdBuildMicromapsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html).
    pub const CMD_BUILD_PARTITIONED_ACCELERATION_STRUCTURES_NV: CommandInfo = CommandInfo {
        name: "vkCmdBuildPartitionedAccelerationStructuresNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_partitioned_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html).
    pub const CMD_CLEAR_ATTACHMENTS: CommandInfo = CommandInfo {
        name: "vkCmdClearAttachments",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html).
    pub const CMD_CLEAR_COLOR_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdClearColorImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html).
    pub const CMD_CLEAR_DEPTH_STENCIL_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdClearDepthStencilImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html).
    pub const CMD_CONTROL_VIDEO_CODING_KHR: CommandInfo = CommandInfo {
        name: "vkCmdControlVideoCodingKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html).
    pub const CMD_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: CommandInfo = CommandInfo {
        name: "vkCmdConvertCooperativeVectorMatrixNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cooperative_vector"],
    };
    /// What the registry states about the C command [`vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html).
    pub const CMD_COPY_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html).
    pub const CMD_COPY_ACCELERATION_STRUCTURE_NV: CommandInfo = CommandInfo {
        name: "vkCmdCopyAccelerationStructureNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html).
    pub const CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyAccelerationStructureToMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html).
    pub const CMD_COPY_BUFFER: CommandInfo = CommandInfo {
        name: "vkCmdCopyBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html).
    pub const CMD_COPY_BUFFER2: CommandInfo = CommandInfo {
        name: "vkCmdCopyBuffer2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html).
    pub const CMD_COPY_BUFFER2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyBuffer2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html).
    pub const CMD_COPY_BUFFER_TO_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdCopyBufferToImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html).
    pub const CMD_COPY_BUFFER_TO_IMAGE2: CommandInfo = CommandInfo {
        name: "vkCmdCopyBufferToImage2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html).
    pub const CMD_COPY_BUFFER_TO_IMAGE2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyBufferToImage2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html).
    pub const CMD_COPY_GPA_SESSION_RESULTS_AMD: CommandInfo = CommandInfo {
        name: "vkCmdCopyGpaSessionResultsAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html).
    pub const CMD_COPY_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdCopyImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html).
    pub const CMD_COPY_IMAGE2: CommandInfo = CommandInfo {
        name: "vkCmdCopyImage2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html).
    pub const CMD_COPY_IMAGE2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyImage2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html).
    pub const CMD_COPY_IMAGE_TO_BUFFER: CommandInfo = CommandInfo {
        name: "vkCmdCopyImageToBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html).
    pub const CMD_COPY_IMAGE_TO_BUFFER2: CommandInfo = CommandInfo {
        name: "vkCmdCopyImageToBuffer2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html).
    pub const CMD_COPY_IMAGE_TO_BUFFER2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyImageToBuffer2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html).
    pub const CMD_COPY_IMAGE_TO_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyImageToMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html).
    pub const CMD_COPY_MEMORY_INDIRECT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryIndirectKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_memory_indirect"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html).
    pub const CMD_COPY_MEMORY_INDIRECT_NV: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryIndirectNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_copy_memory_indirect"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html).
    pub const CMD_COPY_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html).
    pub const CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryToAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html).
    pub const CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryToImageIndirectKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_memory_indirect"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html).
    pub const CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryToImageIndirectNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_copy_memory_indirect"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html).
    pub const CMD_COPY_MEMORY_TO_IMAGE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryToImageKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html).
    pub const CMD_COPY_MEMORY_TO_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdCopyMemoryToMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html).
    pub const CMD_COPY_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdCopyMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html).
    pub const CMD_COPY_MICROMAP_TO_MEMORY_EXT: CommandInfo = CommandInfo {
        name: "vkCmdCopyMicromapToMemoryEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html).
    pub const CMD_COPY_QUERY_POOL_RESULTS: CommandInfo = CommandInfo {
        name: "vkCmdCopyQueryPoolResults",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html).
    pub const CMD_COPY_QUERY_POOL_RESULTS_TO_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdCopyQueryPoolResultsToMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html).
    pub const CMD_COPY_TENSOR_ARM: CommandInfo = CommandInfo {
        name: "vkCmdCopyTensorARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html).
    pub const CMD_CU_LAUNCH_KERNEL_NVX: CommandInfo = CommandInfo {
        name: "vkCmdCuLaunchKernelNVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_binary_import"],
    };
    /// What the registry states about the C command [`vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html).
    pub const CMD_CUDA_LAUNCH_KERNEL_NV: CommandInfo = CommandInfo {
        name: "vkCmdCudaLaunchKernelNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html).
    pub const CMD_DEBUG_MARKER_BEGIN_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDebugMarkerBeginEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_marker"],
    };
    /// What the registry states about the C command [`vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html).
    pub const CMD_DEBUG_MARKER_END_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDebugMarkerEndEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_marker"],
    };
    /// What the registry states about the C command [`vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html).
    pub const CMD_DEBUG_MARKER_INSERT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDebugMarkerInsertEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_marker"],
    };
    /// What the registry states about the C command [`vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html).
    pub const CMD_DECODE_VIDEO_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDecodeVideoKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_decode_queue"],
    };
    /// What the registry states about the C command [`vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html).
    pub const CMD_DECOMPRESS_MEMORY_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDecompressMemoryEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_memory_decompression"],
    };
    /// What the registry states about the C command [`vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html).
    pub const CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDecompressMemoryIndirectCountEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_memory_decompression"],
    };
    /// What the registry states about the C command [`vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html).
    pub const CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV: CommandInfo = CommandInfo {
        name: "vkCmdDecompressMemoryIndirectCountNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_memory_decompression"],
    };
    /// What the registry states about the C command [`vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html).
    pub const CMD_DECOMPRESS_MEMORY_NV: CommandInfo = CommandInfo {
        name: "vkCmdDecompressMemoryNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_memory_decompression"],
    };
    /// What the registry states about the C command [`vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html).
    pub const CMD_DISPATCH: CommandInfo = CommandInfo {
        name: "vkCmdDispatch",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html).
    pub const CMD_DISPATCH_BASE: CommandInfo = CommandInfo {
        name: "vkCmdDispatchBase",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html).
    pub const CMD_DISPATCH_BASE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDispatchBaseKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html).
    pub const CMD_DISPATCH_DATA_GRAPH_ARM: CommandInfo = CommandInfo {
        name: "vkCmdDispatchDataGraphARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html).
    pub const CMD_DISPATCH_GRAPH_AMDX: CommandInfo = CommandInfo {
        name: "vkCmdDispatchGraphAMDX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html).
    pub const CMD_DISPATCH_GRAPH_INDIRECT_AMDX: CommandInfo = CommandInfo {
        name: "vkCmdDispatchGraphIndirectAMDX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html).
    pub const CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX: CommandInfo = CommandInfo {
        name: "vkCmdDispatchGraphIndirectCountAMDX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html).
    pub const CMD_DISPATCH_INDIRECT: CommandInfo = CommandInfo {
        name: "vkCmdDispatchIndirect",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html).
    pub const CMD_DISPATCH_INDIRECT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDispatchIndirect2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html).
    pub const CMD_DISPATCH_TILE_QCOM: CommandInfo = CommandInfo {
        name: "vkCmdDispatchTileQCOM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_QCOM_tile_shading"],
    };
    /// What the registry states about the C command [`vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html).
    pub const CMD_DRAW: CommandInfo = CommandInfo {
        name: "vkCmdDraw",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html).
    pub const CMD_DRAW_CLUSTER_HUAWEI: CommandInfo = CommandInfo {
        name: "vkCmdDrawClusterHUAWEI",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_HUAWEI_cluster_culling_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html).
    pub const CMD_DRAW_CLUSTER_INDIRECT_HUAWEI: CommandInfo = CommandInfo {
        name: "vkCmdDrawClusterIndirectHUAWEI",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_HUAWEI_cluster_culling_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html).
    pub const CMD_DRAW_INDEXED: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexed",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html).
    pub const CMD_DRAW_INDEXED_INDIRECT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirect",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html).
    pub const CMD_DRAW_INDEXED_INDIRECT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirect2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html).
    pub const CMD_DRAW_INDEXED_INDIRECT_COUNT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirectCount",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html).
    pub const CMD_DRAW_INDEXED_INDIRECT_COUNT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirectCount2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html).
    pub const CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirectCountAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_draw_indirect_count"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html).
    pub const CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndexedIndirectCountKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_draw_indirect_count"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html).
    pub const CMD_DRAW_INDIRECT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirect",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html).
    pub const CMD_DRAW_INDIRECT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirect2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html).
    pub const CMD_DRAW_INDIRECT_BYTE_COUNT2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectByteCount2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html).
    pub const CMD_DRAW_INDIRECT_BYTE_COUNT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectByteCountEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html).
    pub const CMD_DRAW_INDIRECT_COUNT: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectCount",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html).
    pub const CMD_DRAW_INDIRECT_COUNT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectCount2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html).
    pub const CMD_DRAW_INDIRECT_COUNT_AMD: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectCountAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_draw_indirect_count"],
    };
    /// What the registry states about the C command [`vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html).
    pub const CMD_DRAW_INDIRECT_COUNT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdDrawIndirectCountKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_draw_indirect_count"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html).
    pub const CMD_DRAW_MESH_TASKS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirect2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT_COUNT2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirectCount2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirectCountEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirectCountNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirectEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html).
    pub const CMD_DRAW_MESH_TASKS_INDIRECT_NV: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksIndirectNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html).
    pub const CMD_DRAW_MESH_TASKS_NV: CommandInfo = CommandInfo {
        name: "vkCmdDrawMeshTasksNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_mesh_shader"],
    };
    /// What the registry states about the C command [`vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html).
    pub const CMD_DRAW_MULTI_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMultiEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_multi_draw"],
    };
    /// What the registry states about the C command [`vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html).
    pub const CMD_DRAW_MULTI_INDEXED_EXT: CommandInfo = CommandInfo {
        name: "vkCmdDrawMultiIndexedEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_multi_draw"],
    };
    /// What the registry states about the C command [`vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html).
    pub const CMD_ENCODE_VIDEO_KHR: CommandInfo = CommandInfo {
        name: "vkCmdEncodeVideoKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_encode_queue"],
    };
    /// What the registry states about the C command [`vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html).
    pub const CMD_END_CONDITIONAL_RENDERING_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndConditionalRenderingEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_conditional_rendering"],
    };
    /// What the registry states about the C command [`vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html).
    pub const CMD_END_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html).
    pub const CMD_END_GPA_SAMPLE_AMD: CommandInfo = CommandInfo {
        name: "vkCmdEndGpaSampleAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html).
    pub const CMD_END_GPA_SESSION_AMD: CommandInfo = CommandInfo {
        name: "vkCmdEndGpaSessionAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html).
    pub const CMD_END_PER_TILE_EXECUTION_QCOM: CommandInfo = CommandInfo {
        name: "vkCmdEndPerTileExecutionQCOM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_QCOM_tile_shading"],
    };
    /// What the registry states about the C command [`vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html).
    pub const CMD_END_QUERY: CommandInfo = CommandInfo {
        name: "vkCmdEndQuery",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html).
    pub const CMD_END_QUERY_INDEXED_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndQueryIndexedEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html).
    pub const CMD_END_RENDER_PASS: CommandInfo = CommandInfo {
        name: "vkCmdEndRenderPass",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html).
    pub const CMD_END_RENDER_PASS2: CommandInfo = CommandInfo {
        name: "vkCmdEndRenderPass2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html).
    pub const CMD_END_RENDER_PASS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdEndRenderPass2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_create_renderpass2"],
    };
    /// What the registry states about the C command [`vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html).
    pub const CMD_END_RENDERING: CommandInfo = CommandInfo {
        name: "vkCmdEndRendering",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html).
    pub const CMD_END_RENDERING2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndRendering2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_fragment_density_map_offset"],
    };
    /// What the registry states about the C command [`vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html).
    pub const CMD_END_RENDERING2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdEndRendering2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance10"],
    };
    /// What the registry states about the C command [`vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html).
    pub const CMD_END_RENDERING_KHR: CommandInfo = CommandInfo {
        name: "vkCmdEndRenderingKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_dynamic_rendering"],
    };
    /// What the registry states about the C command [`vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html).
    pub const CMD_END_SHADER_INSTRUMENTATION_ARM: CommandInfo = CommandInfo {
        name: "vkCmdEndShaderInstrumentationARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html).
    pub const CMD_END_TRANSFORM_FEEDBACK2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndTransformFeedback2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html).
    pub const CMD_END_TRANSFORM_FEEDBACK_EXT: CommandInfo = CommandInfo {
        name: "vkCmdEndTransformFeedbackEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_transform_feedback"],
    };
    /// What the registry states about the C command [`vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html).
    pub const CMD_END_VIDEO_CODING_KHR: CommandInfo = CommandInfo {
        name: "vkCmdEndVideoCodingKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html).
    pub const CMD_EXECUTE_COMMANDS: CommandInfo = CommandInfo {
        name: "vkCmdExecuteCommands",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html).
    pub const CMD_EXECUTE_GENERATED_COMMANDS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdExecuteGeneratedCommandsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html).
    pub const CMD_EXECUTE_GENERATED_COMMANDS_NV: CommandInfo = CommandInfo {
        name: "vkCmdExecuteGeneratedCommandsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html).
    pub const CMD_FILL_BUFFER: CommandInfo = CommandInfo {
        name: "vkCmdFillBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html).
    pub const CMD_FILL_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdFillMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html).
    pub const CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX: CommandInfo = CommandInfo {
        name: "vkCmdInitializeGraphScratchMemoryAMDX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html).
    pub const CMD_INSERT_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkCmdInsertDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html).
    pub const CMD_NEXT_SUBPASS: CommandInfo = CommandInfo {
        name: "vkCmdNextSubpass",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html).
    pub const CMD_NEXT_SUBPASS2: CommandInfo = CommandInfo {
        name: "vkCmdNextSubpass2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html).
    pub const CMD_NEXT_SUBPASS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdNextSubpass2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_create_renderpass2"],
    };
    /// What the registry states about the C command [`vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html).
    pub const CMD_OPTICAL_FLOW_EXECUTE_NV: CommandInfo = CommandInfo {
        name: "vkCmdOpticalFlowExecuteNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_optical_flow"],
    };
    /// What the registry states about the C command [`vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html).
    pub const CMD_PIPELINE_BARRIER: CommandInfo = CommandInfo {
        name: "vkCmdPipelineBarrier",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html).
    pub const CMD_PIPELINE_BARRIER2: CommandInfo = CommandInfo {
        name: "vkCmdPipelineBarrier2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html).
    pub const CMD_PIPELINE_BARRIER2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPipelineBarrier2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html).
    pub const CMD_PREPROCESS_GENERATED_COMMANDS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdPreprocessGeneratedCommandsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html).
    pub const CMD_PREPROCESS_GENERATED_COMMANDS_NV: CommandInfo = CommandInfo {
        name: "vkCmdPreprocessGeneratedCommandsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html).
    pub const CMD_PUSH_CONSTANTS: CommandInfo = CommandInfo {
        name: "vkCmdPushConstants",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html).
    pub const CMD_PUSH_CONSTANTS2: CommandInfo = CommandInfo {
        name: "vkCmdPushConstants2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html).
    pub const CMD_PUSH_CONSTANTS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPushConstants2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html).
    pub const CMD_PUSH_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkCmdPushDataEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html).
    pub const CMD_PUSH_DESCRIPTOR_SET: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSet",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html).
    pub const CMD_PUSH_DESCRIPTOR_SET2: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSet2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html).
    pub const CMD_PUSH_DESCRIPTOR_SET2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSet2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html).
    pub const CMD_PUSH_DESCRIPTOR_SET_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSetKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_push_descriptor"],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html).
    pub const CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSetWithTemplate",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html).
    pub const CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSetWithTemplate2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html).
    pub const CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSetWithTemplate2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html).
    pub const CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdPushDescriptorSetWithTemplateKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[
            c"VK_KHR_push_descriptor",
            c"VK_KHR_descriptor_update_template",
        ],
    };
    /// What the registry states about the C command [`vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html).
    pub const CMD_RESET_EVENT: CommandInfo = CommandInfo {
        name: "vkCmdResetEvent",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html).
    pub const CMD_RESET_EVENT2: CommandInfo = CommandInfo {
        name: "vkCmdResetEvent2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html).
    pub const CMD_RESET_EVENT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdResetEvent2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html).
    pub const CMD_RESET_QUERY_POOL: CommandInfo = CommandInfo {
        name: "vkCmdResetQueryPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html).
    pub const CMD_RESOLVE_IMAGE: CommandInfo = CommandInfo {
        name: "vkCmdResolveImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html).
    pub const CMD_RESOLVE_IMAGE2: CommandInfo = CommandInfo {
        name: "vkCmdResolveImage2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html).
    pub const CMD_RESOLVE_IMAGE2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdResolveImage2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_copy_commands2"],
    };
    /// What the registry states about the C command [`vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html).
    pub const CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetAlphaToCoverageEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html).
    pub const CMD_SET_ALPHA_TO_ONE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetAlphaToOneEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html).
    pub const CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetAttachmentFeedbackLoopEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_attachment_feedback_loop_dynamic_state"],
    };
    /// What the registry states about the C command [`vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html).
    pub const CMD_SET_BLEND_CONSTANTS: CommandInfo = CommandInfo {
        name: "vkCmdSetBlendConstants",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html).
    pub const CMD_SET_CHECKPOINT_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCheckpointNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_diagnostic_checkpoints"],
    };
    /// What the registry states about the C command [`vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html).
    pub const CMD_SET_COARSE_SAMPLE_ORDER_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoarseSampleOrderNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_shading_rate_image"],
    };
    /// What the registry states about the C command [`vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html).
    pub const CMD_SET_COLOR_BLEND_ADVANCED_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetColorBlendAdvancedEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html).
    pub const CMD_SET_COLOR_BLEND_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetColorBlendEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html).
    pub const CMD_SET_COLOR_BLEND_EQUATION_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetColorBlendEquationEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html).
    pub const CMD_SET_COLOR_WRITE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetColorWriteEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_color_write_enable"],
    };
    /// What the registry states about the C command [`vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html).
    pub const CMD_SET_COLOR_WRITE_MASK_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetColorWriteMaskEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html).
    pub const CMD_SET_COMPUTE_OCCUPANCY_PRIORITY_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetComputeOccupancyPriorityNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_compute_occupancy_priority"],
    };
    /// What the registry states about the C command [`vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html).
    pub const CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetConservativeRasterizationModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html).
    pub const CMD_SET_COVERAGE_MODULATION_MODE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageModulationModeNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html).
    pub const CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageModulationTableEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html).
    pub const CMD_SET_COVERAGE_MODULATION_TABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageModulationTableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html).
    pub const CMD_SET_COVERAGE_REDUCTION_MODE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageReductionModeNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html).
    pub const CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageToColorEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html).
    pub const CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetCoverageToColorLocationNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html).
    pub const CMD_SET_CULL_MODE: CommandInfo = CommandInfo {
        name: "vkCmdSetCullMode",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html).
    pub const CMD_SET_CULL_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetCullModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html).
    pub const CMD_SET_DEPTH_BIAS: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBias",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html).
    pub const CMD_SET_DEPTH_BIAS2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBias2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_depth_bias_control"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html).
    pub const CMD_SET_DEPTH_BIAS_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBiasEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html).
    pub const CMD_SET_DEPTH_BIAS_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBiasEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state2", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html).
    pub const CMD_SET_DEPTH_BOUNDS: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBounds",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html).
    pub const CMD_SET_DEPTH_BOUNDS_TEST_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBoundsTestEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html).
    pub const CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthBoundsTestEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html).
    pub const CMD_SET_DEPTH_CLAMP_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthClampEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html).
    pub const CMD_SET_DEPTH_CLAMP_RANGE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthClampRangeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_shader_object", c"VK_EXT_depth_clamp_control"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html).
    pub const CMD_SET_DEPTH_CLIP_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthClipEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html).
    pub const CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthClipNegativeOneToOneEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html).
    pub const CMD_SET_DEPTH_COMPARE_OP: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthCompareOp",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html).
    pub const CMD_SET_DEPTH_COMPARE_OP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthCompareOpEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html).
    pub const CMD_SET_DEPTH_TEST_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthTestEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html).
    pub const CMD_SET_DEPTH_TEST_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthTestEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html).
    pub const CMD_SET_DEPTH_WRITE_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthWriteEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html).
    pub const CMD_SET_DEPTH_WRITE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDepthWriteEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html).
    pub const CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDescriptorBufferOffsets2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance6"],
    };
    /// What the registry states about the C command [`vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html).
    pub const CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDescriptorBufferOffsetsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html).
    pub const CMD_SET_DEVICE_MASK: CommandInfo = CommandInfo {
        name: "vkCmdSetDeviceMask",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html).
    pub const CMD_SET_DEVICE_MASK_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetDeviceMaskKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html).
    pub const CMD_SET_DISCARD_RECTANGLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDiscardRectangleEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_discard_rectangles"],
    };
    /// What the registry states about the C command [`vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html).
    pub const CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDiscardRectangleEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_discard_rectangles"],
    };
    /// What the registry states about the C command [`vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html).
    pub const CMD_SET_DISCARD_RECTANGLE_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetDiscardRectangleModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_discard_rectangles"],
    };
    /// What the registry states about the C command [`vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html).
    pub const CMD_SET_DISPATCH_PARAMETERS_ARM: CommandInfo = CommandInfo {
        name: "vkCmdSetDispatchParametersARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_scheduling_controls"],
    };
    /// What the registry states about the C command [`vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html).
    pub const CMD_SET_EVENT: CommandInfo = CommandInfo {
        name: "vkCmdSetEvent",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html).
    pub const CMD_SET_EVENT2: CommandInfo = CommandInfo {
        name: "vkCmdSetEvent2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html).
    pub const CMD_SET_EVENT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetEvent2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html).
    pub const CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetExclusiveScissorEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_scissor_exclusive"],
    };
    /// What the registry states about the C command [`vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html).
    pub const CMD_SET_EXCLUSIVE_SCISSOR_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetExclusiveScissorNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_scissor_exclusive"],
    };
    /// What the registry states about the C command [`vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html).
    pub const CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetExtraPrimitiveOverestimationSizeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html).
    pub const CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetFragmentShadingRateEnumNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_fragment_shading_rate_enums"],
    };
    /// What the registry states about the C command [`vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html).
    pub const CMD_SET_FRAGMENT_SHADING_RATE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetFragmentShadingRateKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_fragment_shading_rate"],
    };
    /// What the registry states about the C command [`vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html).
    pub const CMD_SET_FRONT_FACE: CommandInfo = CommandInfo {
        name: "vkCmdSetFrontFace",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html).
    pub const CMD_SET_FRONT_FACE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetFrontFaceEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html).
    pub const CMD_SET_LINE_RASTERIZATION_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetLineRasterizationModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html).
    pub const CMD_SET_LINE_STIPPLE: CommandInfo = CommandInfo {
        name: "vkCmdSetLineStipple",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html).
    pub const CMD_SET_LINE_STIPPLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetLineStippleEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_line_rasterization"],
    };
    /// What the registry states about the C command [`vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html).
    pub const CMD_SET_LINE_STIPPLE_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetLineStippleEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html).
    pub const CMD_SET_LINE_STIPPLE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetLineStippleKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_line_rasterization"],
    };
    /// What the registry states about the C command [`vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html).
    pub const CMD_SET_LINE_WIDTH: CommandInfo = CommandInfo {
        name: "vkCmdSetLineWidth",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html).
    pub const CMD_SET_LOGIC_OP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetLogicOpEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state2", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html).
    pub const CMD_SET_LOGIC_OP_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetLogicOpEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html).
    pub const CMD_SET_PATCH_CONTROL_POINTS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetPatchControlPointsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state2", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html).
    pub const CMD_SET_PERFORMANCE_MARKER_INTEL: CommandInfo = CommandInfo {
        name: "vkCmdSetPerformanceMarkerINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html).
    pub const CMD_SET_PERFORMANCE_OVERRIDE_INTEL: CommandInfo = CommandInfo {
        name: "vkCmdSetPerformanceOverrideINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html).
    pub const CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL: CommandInfo = CommandInfo {
        name: "vkCmdSetPerformanceStreamMarkerINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html).
    pub const CMD_SET_POLYGON_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetPolygonModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html).
    pub const CMD_SET_PRIMITIVE_RESTART_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetPrimitiveRestartEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html).
    pub const CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetPrimitiveRestartEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state2", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html).
    pub const CMD_SET_PRIMITIVE_RESTART_INDEX_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetPrimitiveRestartIndexEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_primitive_restart_index"],
    };
    /// What the registry states about the C command [`vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html).
    pub const CMD_SET_PRIMITIVE_TOPOLOGY: CommandInfo = CommandInfo {
        name: "vkCmdSetPrimitiveTopology",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html).
    pub const CMD_SET_PRIMITIVE_TOPOLOGY_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetPrimitiveTopologyEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html).
    pub const CMD_SET_PROVOKING_VERTEX_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetProvokingVertexModeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html).
    pub const CMD_SET_RASTERIZATION_SAMPLES_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetRasterizationSamplesEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html).
    pub const CMD_SET_RASTERIZATION_STREAM_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetRasterizationStreamEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html).
    pub const CMD_SET_RASTERIZER_DISCARD_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetRasterizerDiscardEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html).
    pub const CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetRasterizerDiscardEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state2", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html).
    pub const CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetRayTracingPipelineStackSizeKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html).
    pub const CMD_SET_RENDERING_ATTACHMENT_LOCATIONS: CommandInfo = CommandInfo {
        name: "vkCmdSetRenderingAttachmentLocations",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html).
    pub const CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetRenderingAttachmentLocationsKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_dynamic_rendering_local_read"],
    };
    /// What the registry states about the C command [`vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html).
    pub const CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES: CommandInfo = CommandInfo {
        name: "vkCmdSetRenderingInputAttachmentIndices",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html).
    pub const CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR: CommandInfo = CommandInfo {
        name: "vkCmdSetRenderingInputAttachmentIndicesKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_dynamic_rendering_local_read"],
    };
    /// What the registry states about the C command [`vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html).
    pub const CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetRepresentativeFragmentTestEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html).
    pub const CMD_SET_SAMPLE_LOCATIONS_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetSampleLocationsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_sample_locations"],
    };
    /// What the registry states about the C command [`vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html).
    pub const CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetSampleLocationsEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html).
    pub const CMD_SET_SAMPLE_MASK_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetSampleMaskEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html).
    pub const CMD_SET_SCISSOR: CommandInfo = CommandInfo {
        name: "vkCmdSetScissor",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html).
    pub const CMD_SET_SCISSOR_WITH_COUNT: CommandInfo = CommandInfo {
        name: "vkCmdSetScissorWithCount",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html).
    pub const CMD_SET_SCISSOR_WITH_COUNT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetScissorWithCountEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html).
    pub const CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetShadingRateImageEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html).
    pub const CMD_SET_STENCIL_COMPARE_MASK: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilCompareMask",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html).
    pub const CMD_SET_STENCIL_OP: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilOp",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html).
    pub const CMD_SET_STENCIL_OP_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilOpEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html).
    pub const CMD_SET_STENCIL_REFERENCE: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilReference",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html).
    pub const CMD_SET_STENCIL_TEST_ENABLE: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilTestEnable",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html).
    pub const CMD_SET_STENCIL_TEST_ENABLE_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilTestEnableEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html).
    pub const CMD_SET_STENCIL_WRITE_MASK: CommandInfo = CommandInfo {
        name: "vkCmdSetStencilWriteMask",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html).
    pub const CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetTessellationDomainOriginEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html).
    pub const CMD_SET_VERTEX_INPUT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetVertexInputEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[
            c"VK_EXT_vertex_input_dynamic_state",
            c"VK_EXT_shader_object",
        ],
    };
    /// What the registry states about the C command [`vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html).
    pub const CMD_SET_VIEWPORT: CommandInfo = CommandInfo {
        name: "vkCmdSetViewport",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html).
    pub const CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportShadingRatePaletteNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_shading_rate_image"],
    };
    /// What the registry states about the C command [`vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html).
    pub const CMD_SET_VIEWPORT_SWIZZLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportSwizzleNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html).
    pub const CMD_SET_VIEWPORT_W_SCALING_ENABLE_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportWScalingEnableNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state3", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html).
    pub const CMD_SET_VIEWPORT_W_SCALING_NV: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportWScalingNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_clip_space_w_scaling"],
    };
    /// What the registry states about the C command [`vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html).
    pub const CMD_SET_VIEWPORT_WITH_COUNT: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportWithCount",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html).
    pub const CMD_SET_VIEWPORT_WITH_COUNT_EXT: CommandInfo = CommandInfo {
        name: "vkCmdSetViewportWithCountEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_extended_dynamic_state", c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html).
    pub const CMD_SUBPASS_SHADING_HUAWEI: CommandInfo = CommandInfo {
        name: "vkCmdSubpassShadingHUAWEI",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_HUAWEI_subpass_shading"],
    };
    /// What the registry states about the C command [`vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html).
    pub const CMD_TRACE_RAYS_INDIRECT2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdTraceRaysIndirect2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_maintenance1"],
    };
    /// What the registry states about the C command [`vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html).
    pub const CMD_TRACE_RAYS_INDIRECT_KHR: CommandInfo = CommandInfo {
        name: "vkCmdTraceRaysIndirectKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html).
    pub const CMD_TRACE_RAYS_KHR: CommandInfo = CommandInfo {
        name: "vkCmdTraceRaysKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html).
    pub const CMD_TRACE_RAYS_NV: CommandInfo = CommandInfo {
        name: "vkCmdTraceRaysNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html).
    pub const CMD_UPDATE_BUFFER: CommandInfo = CommandInfo {
        name: "vkCmdUpdateBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html).
    pub const CMD_UPDATE_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCmdUpdateMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html).
    pub const CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV: CommandInfo = CommandInfo {
        name: "vkCmdUpdatePipelineIndirectBufferNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands_compute"],
    };
    /// What the registry states about the C command [`vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html).
    pub const CMD_WAIT_EVENTS: CommandInfo = CommandInfo {
        name: "vkCmdWaitEvents",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html).
    pub const CMD_WAIT_EVENTS2: CommandInfo = CommandInfo {
        name: "vkCmdWaitEvents2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html).
    pub const CMD_WAIT_EVENTS2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdWaitEvents2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html).
    pub const CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkCmdWriteAccelerationStructuresPropertiesKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html).
    pub const CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV: CommandInfo = CommandInfo {
        name: "vkCmdWriteAccelerationStructuresPropertiesNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html).
    pub const CMD_WRITE_BUFFER_MARKER2_AMD: CommandInfo = CommandInfo {
        name: "vkCmdWriteBufferMarker2AMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_buffer_marker"],
    };
    /// What the registry states about the C command [`vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html).
    pub const CMD_WRITE_BUFFER_MARKER_AMD: CommandInfo = CommandInfo {
        name: "vkCmdWriteBufferMarkerAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_buffer_marker"],
    };
    /// What the registry states about the C command [`vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html).
    pub const CMD_WRITE_MARKER_TO_MEMORY_AMD: CommandInfo = CommandInfo {
        name: "vkCmdWriteMarkerToMemoryAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html).
    pub const CMD_WRITE_MICROMAPS_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkCmdWriteMicromapsPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html).
    pub const CMD_WRITE_TIMESTAMP: CommandInfo = CommandInfo {
        name: "vkCmdWriteTimestamp",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html).
    pub const CMD_WRITE_TIMESTAMP2: CommandInfo = CommandInfo {
        name: "vkCmdWriteTimestamp2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html).
    pub const CMD_WRITE_TIMESTAMP2_KHR: CommandInfo = CommandInfo {
        name: "vkCmdWriteTimestamp2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html).
    pub const COMPILE_DEFERRED_NV: CommandInfo = CommandInfo {
        name: "vkCompileDeferredNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html).
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: CommandInfo = CommandInfo {
        name: "vkConvertCooperativeVectorMatrixNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cooperative_vector"],
    };
    /// What the registry states about the C command [`vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html).
    pub const COPY_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkCopyAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html).
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR: CommandInfo = CommandInfo {
        name: "vkCopyAccelerationStructureToMemoryKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html).
    pub const COPY_IMAGE_TO_IMAGE: CommandInfo = CommandInfo {
        name: "vkCopyImageToImage",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html).
    pub const COPY_IMAGE_TO_IMAGE_EXT: CommandInfo = CommandInfo {
        name: "vkCopyImageToImageEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_host_image_copy"],
    };
    /// What the registry states about the C command [`vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html).
    pub const COPY_IMAGE_TO_MEMORY: CommandInfo = CommandInfo {
        name: "vkCopyImageToMemory",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html).
    pub const COPY_IMAGE_TO_MEMORY_EXT: CommandInfo = CommandInfo {
        name: "vkCopyImageToMemoryEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_host_image_copy"],
    };
    /// What the registry states about the C command [`vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html).
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkCopyMemoryToAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html).
    pub const COPY_MEMORY_TO_IMAGE: CommandInfo = CommandInfo {
        name: "vkCopyMemoryToImage",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html).
    pub const COPY_MEMORY_TO_IMAGE_EXT: CommandInfo = CommandInfo {
        name: "vkCopyMemoryToImageEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_host_image_copy"],
    };
    /// What the registry states about the C command [`vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html).
    pub const COPY_MEMORY_TO_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkCopyMemoryToMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html).
    pub const COPY_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkCopyMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html).
    pub const COPY_MICROMAP_TO_MEMORY_EXT: CommandInfo = CommandInfo {
        name: "vkCopyMicromapToMemoryEXT",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html).
    pub const CREATE_ACCELERATION_STRUCTURE2_KHR: CommandInfo = CommandInfo {
        name: "vkCreateAccelerationStructure2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_VALIDATION_FAILED,
            VkResult::ERROR_UNKNOWN,
        ],
        version: None,
        extensions: &[c"VK_KHR_device_address_commands"],
    };
    /// What the registry states about the C command [`vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html).
    pub const CREATE_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html).
    pub const CREATE_ACCELERATION_STRUCTURE_NV: CommandInfo = CommandInfo {
        name: "vkCreateAccelerationStructureNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html).
    pub const CREATE_ANDROID_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateAndroidSurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_android_surface"],
    };
    /// What the registry states about the C command [`vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html).
    pub const CREATE_BUFFER: CommandInfo = CommandInfo {
        name: "vkCreateBuffer",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html).
    pub const CREATE_BUFFER_COLLECTION_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkCreateBufferCollectionFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_buffer_collection"],
    };
    /// What the registry states about the C command [`vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html).
    pub const CREATE_BUFFER_VIEW: CommandInfo = CommandInfo {
        name: "vkCreateBufferView",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html).
    pub const CREATE_COMMAND_POOL: CommandInfo = CommandInfo {
        name: "vkCreateCommandPool",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html).
    pub const CREATE_COMPUTE_PIPELINES: CommandInfo = CommandInfo {
        name: "vkCreateComputePipelines",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::PIPELINE_COMPILE_REQUIRED],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_SHADER_NV,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html).
    pub const CREATE_CU_FUNCTION_NVX: CommandInfo = CommandInfo {
        name: "vkCreateCuFunctionNVX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NVX_binary_import"],
    };
    /// What the registry states about the C command [`vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html).
    pub const CREATE_CU_MODULE_NVX: CommandInfo = CommandInfo {
        name: "vkCreateCuModuleNVX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NVX_binary_import"],
    };
    /// What the registry states about the C command [`vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html).
    pub const CREATE_CUDA_FUNCTION_NV: CommandInfo = CommandInfo {
        name: "vkCreateCudaFunctionNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html).
    pub const CREATE_CUDA_MODULE_NV: CommandInfo = CommandInfo {
        name: "vkCreateCudaModuleNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html).
    pub const CREATE_DATA_GRAPH_PIPELINE_SESSION_ARM: CommandInfo = CommandInfo {
        name: "vkCreateDataGraphPipelineSessionARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html).
    pub const CREATE_DATA_GRAPH_PIPELINES_ARM: CommandInfo = CommandInfo {
        name: "vkCreateDataGraphPipelinesARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::PIPELINE_COMPILE_REQUIRED],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html).
    pub const CREATE_DEBUG_REPORT_CALLBACK_EXT: CommandInfo = CommandInfo {
        name: "vkCreateDebugReportCallbackEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_report"],
    };
    /// What the registry states about the C command [`vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html).
    pub const CREATE_DEBUG_UTILS_MESSENGER_EXT: CommandInfo = CommandInfo {
        name: "vkCreateDebugUtilsMessengerEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html).
    pub const CREATE_DEFERRED_OPERATION_KHR: CommandInfo = CommandInfo {
        name: "vkCreateDeferredOperationKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_deferred_host_operations"],
    };
    /// What the registry states about the C command [`vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html).
    pub const CREATE_DESCRIPTOR_POOL: CommandInfo = CommandInfo {
        name: "vkCreateDescriptorPool",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FRAGMENTATION,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html).
    pub const CREATE_DESCRIPTOR_SET_LAYOUT: CommandInfo = CommandInfo {
        name: "vkCreateDescriptorSetLayout",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html).
    pub const CREATE_DESCRIPTOR_UPDATE_TEMPLATE: CommandInfo = CommandInfo {
        name: "vkCreateDescriptorUpdateTemplate",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html).
    pub const CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateDescriptorUpdateTemplateKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_descriptor_update_template"],
    };
    /// What the registry states about the C command [`vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html).
    pub const CREATE_DEVICE: CommandInfo = CommandInfo {
        name: "vkCreateDevice",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_EXTENSION_NOT_PRESENT,
            VkResult::ERROR_FEATURE_NOT_PRESENT,
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html).
    pub const CREATE_DIRECT_FB_SURFACE_EXT: CommandInfo = CommandInfo {
        name: "vkCreateDirectFBSurfaceEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_directfb_surface"],
    };
    /// What the registry states about the C command [`vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html).
    pub const CREATE_DISPLAY_MODE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateDisplayModeKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html).
    pub const CREATE_DISPLAY_PLANE_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateDisplayPlaneSurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html).
    pub const CREATE_EVENT: CommandInfo = CommandInfo {
        name: "vkCreateEvent",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html).
    pub const CREATE_EXECUTION_GRAPH_PIPELINES_AMDX: CommandInfo = CommandInfo {
        name: "vkCreateExecutionGraphPipelinesAMDX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::PIPELINE_COMPILE_REQUIRED],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html).
    pub const CREATE_EXTERNAL_COMPUTE_QUEUE_NV: CommandInfo = CommandInfo {
        name: "vkCreateExternalComputeQueueNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_external_compute_queue"],
    };
    /// What the registry states about the C command [`vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html).
    pub const CREATE_FENCE: CommandInfo = CommandInfo {
        name: "vkCreateFence",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html).
    pub const CREATE_FRAMEBUFFER: CommandInfo = CommandInfo {
        name: "vkCreateFramebuffer",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html).
    pub const CREATE_GPA_SESSION_AMD: CommandInfo = CommandInfo {
        name: "vkCreateGpaSessionAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html).
    pub const CREATE_GRAPHICS_PIPELINES: CommandInfo = CommandInfo {
        name: "vkCreateGraphicsPipelines",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::PIPELINE_COMPILE_REQUIRED],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_SHADER_NV,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html).
    pub const CREATE_HEADLESS_SURFACE_EXT: CommandInfo = CommandInfo {
        name: "vkCreateHeadlessSurfaceEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_headless_surface"],
    };
    /// What the registry states about the C command [`vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html).
    pub const CREATE_IOS_SURFACE_MVK: CommandInfo = CommandInfo {
        name: "vkCreateIOSSurfaceMVK",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_MVK_ios_surface"],
    };
    /// What the registry states about the C command [`vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html).
    pub const CREATE_IMAGE: CommandInfo = CommandInfo {
        name: "vkCreateImage",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_COMPRESSION_EXHAUSTED_EXT,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html).
    pub const CREATE_IMAGE_PIPE_SURFACE_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkCreateImagePipeSurfaceFUCHSIA",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_imagepipe_surface"],
    };
    /// What the registry states about the C command [`vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html).
    pub const CREATE_IMAGE_VIEW: CommandInfo = CommandInfo {
        name: "vkCreateImageView",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html).
    pub const CREATE_INDIRECT_COMMANDS_LAYOUT_EXT: CommandInfo = CommandInfo {
        name: "vkCreateIndirectCommandsLayoutEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html).
    pub const CREATE_INDIRECT_COMMANDS_LAYOUT_NV: CommandInfo = CommandInfo {
        name: "vkCreateIndirectCommandsLayoutNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html).
    pub const CREATE_INDIRECT_EXECUTION_SET_EXT: CommandInfo = CommandInfo {
        name: "vkCreateIndirectExecutionSetEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html).
    pub const CREATE_INSTANCE: CommandInfo = CommandInfo {
        name: "vkCreateInstance",
        level: CommandLevel::Entry,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_LAYER_NOT_PRESENT,
            VkResult::ERROR_EXTENSION_NOT_PRESENT,
            VkResult::ERROR_INCOMPATIBLE_DRIVER,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html).
    pub const CREATE_MAC_OS_SURFACE_MVK: CommandInfo = CommandInfo {
        name: "vkCreateMacOSSurfaceMVK",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_MVK_macos_surface"],
    };
    /// What the registry states about the C command [`vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html).
    pub const CREATE_METAL_SURFACE_EXT: CommandInfo = CommandInfo {
        name: "vkCreateMetalSurfaceEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_metal_surface"],
    };
    /// What the registry states about the C command [`vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html).
    pub const CREATE_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkCreateMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html).
    pub const CREATE_OPTICAL_FLOW_SESSION_NV: CommandInfo = CommandInfo {
        name: "vkCreateOpticalFlowSessionNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_optical_flow"],
    };
    /// What the registry states about the C command [`vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html).
    pub const CREATE_PIPELINE_BINARIES_KHR: CommandInfo = CommandInfo {
        name: "vkCreatePipelineBinariesKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::INCOMPLETE,
            VkResult::PIPELINE_BINARY_MISSING_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_binary"],
    };
    /// What the registry states about the C command [`vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html).
    pub const CREATE_PIPELINE_CACHE: CommandInfo = CommandInfo {
        name: "vkCreatePipelineCache",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html).
    pub const CREATE_PIPELINE_LAYOUT: CommandInfo = CommandInfo {
        name: "vkCreatePipelineLayout",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html).
    pub const CREATE_PRIVATE_DATA_SLOT: CommandInfo = CommandInfo {
        name: "vkCreatePrivateDataSlot",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html).
    pub const CREATE_PRIVATE_DATA_SLOT_EXT: CommandInfo = CommandInfo {
        name: "vkCreatePrivateDataSlotEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_private_data"],
    };
    /// What the registry states about the C command [`vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html).
    pub const CREATE_QUERY_POOL: CommandInfo = CommandInfo {
        name: "vkCreateQueryPool",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html).
    pub const CREATE_RAY_TRACING_PIPELINES_KHR: CommandInfo = CommandInfo {
        name: "vkCreateRayTracingPipelinesKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::OPERATION_DEFERRED_KHR,
            VkResult::OPERATION_NOT_DEFERRED_KHR,
            VkResult::PIPELINE_COMPILE_REQUIRED,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html).
    pub const CREATE_RAY_TRACING_PIPELINES_NV: CommandInfo = CommandInfo {
        name: "vkCreateRayTracingPipelinesNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::PIPELINE_COMPILE_REQUIRED],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_SHADER_NV,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html).
    pub const CREATE_RENDER_PASS: CommandInfo = CommandInfo {
        name: "vkCreateRenderPass",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html).
    pub const CREATE_RENDER_PASS2: CommandInfo = CommandInfo {
        name: "vkCreateRenderPass2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html).
    pub const CREATE_RENDER_PASS2_KHR: CommandInfo = CommandInfo {
        name: "vkCreateRenderPass2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_create_renderpass2"],
    };
    /// What the registry states about the C command [`vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html).
    pub const CREATE_SAMPLER: CommandInfo = CommandInfo {
        name: "vkCreateSampler",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html).
    pub const CREATE_SAMPLER_YCBCR_CONVERSION: CommandInfo = CommandInfo {
        name: "vkCreateSamplerYcbcrConversion",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html).
    pub const CREATE_SAMPLER_YCBCR_CONVERSION_KHR: CommandInfo = CommandInfo {
        name: "vkCreateSamplerYcbcrConversionKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_sampler_ycbcr_conversion"],
    };
    /// What the registry states about the C command [`vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html).
    pub const CREATE_SCREEN_SURFACE_QNX: CommandInfo = CommandInfo {
        name: "vkCreateScreenSurfaceQNX",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_QNX_screen_surface"],
    };
    /// What the registry states about the C command [`vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html).
    pub const CREATE_SEMAPHORE: CommandInfo = CommandInfo {
        name: "vkCreateSemaphore",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html).
    pub const CREATE_SHADER_INSTRUMENTATION_ARM: CommandInfo = CommandInfo {
        name: "vkCreateShaderInstrumentationARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html).
    pub const CREATE_SHADER_MODULE: CommandInfo = CommandInfo {
        name: "vkCreateShaderModule",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_SHADER_NV,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html).
    pub const CREATE_SHADERS_EXT: CommandInfo = CommandInfo {
        name: "vkCreateShadersEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPATIBLE_SHADER_BINARY_EXT],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html).
    pub const CREATE_SHARED_SWAPCHAINS_KHR: CommandInfo = CommandInfo {
        name: "vkCreateSharedSwapchainsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INCOMPATIBLE_DISPLAY_KHR,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display_swapchain"],
    };
    /// What the registry states about the C command [`vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html).
    pub const CREATE_STREAM_DESCRIPTOR_SURFACE_GGP: CommandInfo = CommandInfo {
        name: "vkCreateStreamDescriptorSurfaceGGP",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_GGP_stream_descriptor_surface"],
    };
    /// What the registry states about the C command [`vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html).
    pub const CREATE_SURFACE_OHOS: CommandInfo = CommandInfo {
        name: "vkCreateSurfaceOHOS",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_OHOS_surface"],
    };
    /// What the registry states about the C command [`vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html).
    pub const CREATE_SWAPCHAIN_KHR: CommandInfo = CommandInfo {
        name: "vkCreateSwapchainKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_COMPRESSION_EXHAUSTED_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain"],
    };
    /// What the registry states about the C command [`vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html).
    pub const CREATE_TENSOR_ARM: CommandInfo = CommandInfo {
        name: "vkCreateTensorARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html).
    pub const CREATE_TENSOR_VIEW_ARM: CommandInfo = CommandInfo {
        name: "vkCreateTensorViewARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html).
    pub const CREATE_UBM_SURFACE_SEC: CommandInfo = CommandInfo {
        name: "vkCreateUbmSurfaceSEC",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_SEC_ubm_surface"],
    };
    /// What the registry states about the C command [`vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html).
    pub const CREATE_VALIDATION_CACHE_EXT: CommandInfo = CommandInfo {
        name: "vkCreateValidationCacheEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_validation_cache"],
    };
    /// What the registry states about the C command [`vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html).
    pub const CREATE_VI_SURFACE_NN: CommandInfo = CommandInfo {
        name: "vkCreateViSurfaceNN",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NATIVE_WINDOW_IN_USE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NN_vi_surface"],
    };
    /// What the registry states about the C command [`vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html).
    pub const CREATE_VIDEO_SESSION_KHR: CommandInfo = CommandInfo {
        name: "vkCreateVideoSessionKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR,
            VkResult::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html).
    pub const CREATE_VIDEO_SESSION_PARAMETERS_KHR: CommandInfo = CommandInfo {
        name: "vkCreateVideoSessionParametersKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html).
    pub const CREATE_WAYLAND_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateWaylandSurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_wayland_surface"],
    };
    /// What the registry states about the C command [`vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html).
    pub const CREATE_WIN32_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateWin32SurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_win32_surface"],
    };
    /// What the registry states about the C command [`vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html).
    pub const CREATE_XCB_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateXcbSurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_xcb_surface"],
    };
    /// What the registry states about the C command [`vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html).
    pub const CREATE_XLIB_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkCreateXlibSurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_xlib_surface"],
    };
    /// What the registry states about the C command [`vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html).
    pub const DEBUG_MARKER_SET_OBJECT_NAME_EXT: CommandInfo = CommandInfo {
        name: "vkDebugMarkerSetObjectNameEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_marker"],
    };
    /// What the registry states about the C command [`vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html).
    pub const DEBUG_MARKER_SET_OBJECT_TAG_EXT: CommandInfo = CommandInfo {
        name: "vkDebugMarkerSetObjectTagEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_marker"],
    };
    /// What the registry states about the C command [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html).
    pub const DEBUG_REPORT_MESSAGE_EXT: CommandInfo = CommandInfo {
        name: "vkDebugReportMessageEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_report"],
    };
    /// What the registry states about the C command [`vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html).
    pub const DEFERRED_OPERATION_JOIN_KHR: CommandInfo = CommandInfo {
        name: "vkDeferredOperationJoinKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::THREAD_DONE_KHR,
            VkResult::THREAD_IDLE_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_deferred_host_operations"],
    };
    /// What the registry states about the C command [`vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html).
    pub const DESTROY_ACCELERATION_STRUCTURE_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyAccelerationStructureKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html).
    pub const DESTROY_ACCELERATION_STRUCTURE_NV: CommandInfo = CommandInfo {
        name: "vkDestroyAccelerationStructureNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html).
    pub const DESTROY_BUFFER: CommandInfo = CommandInfo {
        name: "vkDestroyBuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html).
    pub const DESTROY_BUFFER_COLLECTION_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkDestroyBufferCollectionFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_FUCHSIA_buffer_collection"],
    };
    /// What the registry states about the C command [`vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html).
    pub const DESTROY_BUFFER_VIEW: CommandInfo = CommandInfo {
        name: "vkDestroyBufferView",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html).
    pub const DESTROY_COMMAND_POOL: CommandInfo = CommandInfo {
        name: "vkDestroyCommandPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html).
    pub const DESTROY_CU_FUNCTION_NVX: CommandInfo = CommandInfo {
        name: "vkDestroyCuFunctionNVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_binary_import"],
    };
    /// What the registry states about the C command [`vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html).
    pub const DESTROY_CU_MODULE_NVX: CommandInfo = CommandInfo {
        name: "vkDestroyCuModuleNVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_binary_import"],
    };
    /// What the registry states about the C command [`vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html).
    pub const DESTROY_CUDA_FUNCTION_NV: CommandInfo = CommandInfo {
        name: "vkDestroyCudaFunctionNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html).
    pub const DESTROY_CUDA_MODULE_NV: CommandInfo = CommandInfo {
        name: "vkDestroyCudaModuleNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html).
    pub const DESTROY_DATA_GRAPH_PIPELINE_SESSION_ARM: CommandInfo = CommandInfo {
        name: "vkDestroyDataGraphPipelineSessionARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html).
    pub const DESTROY_DEBUG_REPORT_CALLBACK_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyDebugReportCallbackEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_report"],
    };
    /// What the registry states about the C command [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html).
    pub const DESTROY_DEBUG_UTILS_MESSENGER_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyDebugUtilsMessengerEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html).
    pub const DESTROY_DEFERRED_OPERATION_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyDeferredOperationKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_deferred_host_operations"],
    };
    /// What the registry states about the C command [`vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html).
    pub const DESTROY_DESCRIPTOR_POOL: CommandInfo = CommandInfo {
        name: "vkDestroyDescriptorPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html).
    pub const DESTROY_DESCRIPTOR_SET_LAYOUT: CommandInfo = CommandInfo {
        name: "vkDestroyDescriptorSetLayout",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html).
    pub const DESTROY_DESCRIPTOR_UPDATE_TEMPLATE: CommandInfo = CommandInfo {
        name: "vkDestroyDescriptorUpdateTemplate",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html).
    pub const DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyDescriptorUpdateTemplateKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_descriptor_update_template"],
    };
    /// What the registry states about the C command [`vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html).
    pub const DESTROY_DEVICE: CommandInfo = CommandInfo {
        name: "vkDestroyDevice",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html).
    pub const DESTROY_EVENT: CommandInfo = CommandInfo {
        name: "vkDestroyEvent",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html).
    pub const DESTROY_EXTERNAL_COMPUTE_QUEUE_NV: CommandInfo = CommandInfo {
        name: "vkDestroyExternalComputeQueueNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_external_compute_queue"],
    };
    /// What the registry states about the C command [`vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html).
    pub const DESTROY_FENCE: CommandInfo = CommandInfo {
        name: "vkDestroyFence",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html).
    pub const DESTROY_FRAMEBUFFER: CommandInfo = CommandInfo {
        name: "vkDestroyFramebuffer",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html).
    pub const DESTROY_GPA_SESSION_AMD: CommandInfo = CommandInfo {
        name: "vkDestroyGpaSessionAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html).
    pub const DESTROY_IMAGE: CommandInfo = CommandInfo {
        name: "vkDestroyImage",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html).
    pub const DESTROY_IMAGE_VIEW: CommandInfo = CommandInfo {
        name: "vkDestroyImageView",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html).
    pub const DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyIndirectCommandsLayoutEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html).
    pub const DESTROY_INDIRECT_COMMANDS_LAYOUT_NV: CommandInfo = CommandInfo {
        name: "vkDestroyIndirectCommandsLayoutNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html).
    pub const DESTROY_INDIRECT_EXECUTION_SET_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyIndirectExecutionSetEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html).
    pub const DESTROY_INSTANCE: CommandInfo = CommandInfo {
        name: "vkDestroyInstance",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html).
    pub const DESTROY_MICROMAP_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyMicromapEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html).
    pub const DESTROY_OPTICAL_FLOW_SESSION_NV: CommandInfo = CommandInfo {
        name: "vkDestroyOpticalFlowSessionNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_optical_flow"],
    };
    /// What the registry states about the C command [`vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html).
    pub const DESTROY_PIPELINE: CommandInfo = CommandInfo {
        name: "vkDestroyPipeline",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html).
    pub const DESTROY_PIPELINE_BINARY_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyPipelineBinaryKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_pipeline_binary"],
    };
    /// What the registry states about the C command [`vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html).
    pub const DESTROY_PIPELINE_CACHE: CommandInfo = CommandInfo {
        name: "vkDestroyPipelineCache",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html).
    pub const DESTROY_PIPELINE_LAYOUT: CommandInfo = CommandInfo {
        name: "vkDestroyPipelineLayout",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html).
    pub const DESTROY_PRIVATE_DATA_SLOT: CommandInfo = CommandInfo {
        name: "vkDestroyPrivateDataSlot",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html).
    pub const DESTROY_PRIVATE_DATA_SLOT_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyPrivateDataSlotEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_private_data"],
    };
    /// What the registry states about the C command [`vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html).
    pub const DESTROY_QUERY_POOL: CommandInfo = CommandInfo {
        name: "vkDestroyQueryPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html).
    pub const DESTROY_RENDER_PASS: CommandInfo = CommandInfo {
        name: "vkDestroyRenderPass",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html).
    pub const DESTROY_SAMPLER: CommandInfo = CommandInfo {
        name: "vkDestroySampler",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html).
    pub const DESTROY_SAMPLER_YCBCR_CONVERSION: CommandInfo = CommandInfo {
        name: "vkDestroySamplerYcbcrConversion",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html).
    pub const DESTROY_SAMPLER_YCBCR_CONVERSION_KHR: CommandInfo = CommandInfo {
        name: "vkDestroySamplerYcbcrConversionKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_sampler_ycbcr_conversion"],
    };
    /// What the registry states about the C command [`vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html).
    pub const DESTROY_SEMAPHORE: CommandInfo = CommandInfo {
        name: "vkDestroySemaphore",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html).
    pub const DESTROY_SHADER_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyShaderEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html).
    pub const DESTROY_SHADER_INSTRUMENTATION_ARM: CommandInfo = CommandInfo {
        name: "vkDestroyShaderInstrumentationARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html).
    pub const DESTROY_SHADER_MODULE: CommandInfo = CommandInfo {
        name: "vkDestroyShaderModule",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html).
    pub const DESTROY_SURFACE_KHR: CommandInfo = CommandInfo {
        name: "vkDestroySurfaceKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_surface"],
    };
    /// What the registry states about the C command [`vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html).
    pub const DESTROY_SWAPCHAIN_KHR: CommandInfo = CommandInfo {
        name: "vkDestroySwapchainKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_swapchain"],
    };
    /// What the registry states about the C command [`vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html).
    pub const DESTROY_TENSOR_ARM: CommandInfo = CommandInfo {
        name: "vkDestroyTensorARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html).
    pub const DESTROY_TENSOR_VIEW_ARM: CommandInfo = CommandInfo {
        name: "vkDestroyTensorViewARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html).
    pub const DESTROY_VALIDATION_CACHE_EXT: CommandInfo = CommandInfo {
        name: "vkDestroyValidationCacheEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_validation_cache"],
    };
    /// What the registry states about the C command [`vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html).
    pub const DESTROY_VIDEO_SESSION_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyVideoSessionKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html).
    pub const DESTROY_VIDEO_SESSION_PARAMETERS_KHR: CommandInfo = CommandInfo {
        name: "vkDestroyVideoSessionParametersKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html).
    pub const DEVICE_WAIT_IDLE: CommandInfo = CommandInfo {
        name: "vkDeviceWaitIdle",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html).
    pub const DISPLAY_POWER_CONTROL_EXT: CommandInfo = CommandInfo {
        name: "vkDisplayPowerControlEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_display_control"],
    };
    /// What the registry states about the C command [`vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html).
    pub const END_COMMAND_BUFFER: CommandInfo = CommandInfo {
        name: "vkEndCommandBuffer",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html).
    pub const ENUMERATE_DEVICE_EXTENSION_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkEnumerateDeviceExtensionProperties",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_LAYER_NOT_PRESENT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html).
    pub const ENUMERATE_DEVICE_LAYER_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkEnumerateDeviceLayerProperties",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html).
    pub const ENUMERATE_INSTANCE_EXTENSION_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkEnumerateInstanceExtensionProperties",
        level: CommandLevel::Entry,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_LAYER_NOT_PRESENT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html).
    pub const ENUMERATE_INSTANCE_LAYER_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkEnumerateInstanceLayerProperties",
        level: CommandLevel::Entry,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html).
    pub const ENUMERATE_INSTANCE_VERSION: CommandInfo = CommandInfo {
        name: "vkEnumerateInstanceVersion",
        level: CommandLevel::Entry,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html).
    pub const ENUMERATE_PHYSICAL_DEVICE_GROUPS: CommandInfo = CommandInfo {
        name: "vkEnumeratePhysicalDeviceGroups",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html).
    pub const ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR: CommandInfo = CommandInfo {
        name: "vkEnumeratePhysicalDeviceGroupsKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_device_group_creation"],
    };
    /// What the registry states about the C command [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html).
    pub const ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_COUNTERS_BY_REGION_ARM:
        CommandInfo = CommandInfo {
        name: "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_performance_counters_by_region"],
    };
    /// What the registry states about the C command [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html).
    pub const ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR: CommandInfo =
        CommandInfo {
            name: "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
            level: CommandLevel::Instance,
            success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_INITIALIZATION_FAILED,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_KHR_performance_query"],
        };
    /// What the registry states about the C command [`vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html).
    pub const ENUMERATE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_METRICS_ARM: CommandInfo =
        CommandInfo {
            name: "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
            level: CommandLevel::Instance,
            success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_INITIALIZATION_FAILED,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_ARM_shader_instrumentation"],
        };
    /// What the registry states about the C command [`vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html).
    pub const ENUMERATE_PHYSICAL_DEVICES: CommandInfo = CommandInfo {
        name: "vkEnumeratePhysicalDevices",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html).
    pub const EXPORT_METAL_OBJECTS_EXT: CommandInfo = CommandInfo {
        name: "vkExportMetalObjectsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_metal_objects"],
    };
    /// What the registry states about the C command [`vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html).
    pub const FLUSH_MAPPED_MEMORY_RANGES: CommandInfo = CommandInfo {
        name: "vkFlushMappedMemoryRanges",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html).
    pub const FREE_COMMAND_BUFFERS: CommandInfo = CommandInfo {
        name: "vkFreeCommandBuffers",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html).
    pub const FREE_DESCRIPTOR_SETS: CommandInfo = CommandInfo {
        name: "vkFreeDescriptorSets",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html).
    pub const FREE_MEMORY: CommandInfo = CommandInfo {
        name: "vkFreeMemory",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html).
    pub const GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR: CommandInfo = CommandInfo {
        name: "vkGetAccelerationStructureBuildSizesKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html).
    pub const GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR: CommandInfo = CommandInfo {
        name: "vkGetAccelerationStructureDeviceAddressKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html).
    pub const GET_ACCELERATION_STRUCTURE_HANDLE_NV: CommandInfo = CommandInfo {
        name: "vkGetAccelerationStructureHandleNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html).
    pub const GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV: CommandInfo = CommandInfo {
        name: "vkGetAccelerationStructureMemoryRequirementsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html).
    pub const GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT: CommandInfo =
        CommandInfo {
            name: "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
            level: CommandLevel::Device,
            success_codes: &[VkResult::SUCCESS],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_EXT_descriptor_buffer"],
        };
    /// What the registry states about the C command [`vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html).
    pub const GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: CommandInfo = CommandInfo {
        name: "vkGetAndroidHardwareBufferPropertiesANDROID",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ANDROID_external_memory_android_hardware_buffer"],
    };
    /// What the registry states about the C command [`vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html).
    pub const GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkGetBufferCollectionPropertiesFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_buffer_collection"],
    };
    /// What the registry states about the C command [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html).
    pub const GET_BUFFER_DEVICE_ADDRESS: CommandInfo = CommandInfo {
        name: "vkGetBufferDeviceAddress",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html).
    pub const GET_BUFFER_DEVICE_ADDRESS_EXT: CommandInfo = CommandInfo {
        name: "vkGetBufferDeviceAddressEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_buffer_device_address"],
    };
    /// What the registry states about the C command [`vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html).
    pub const GET_BUFFER_DEVICE_ADDRESS_KHR: CommandInfo = CommandInfo {
        name: "vkGetBufferDeviceAddressKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_buffer_device_address"],
    };
    /// What the registry states about the C command [`vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html).
    pub const GET_BUFFER_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetBufferMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html).
    pub const GET_BUFFER_MEMORY_REQUIREMENTS2: CommandInfo = CommandInfo {
        name: "vkGetBufferMemoryRequirements2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html).
    pub const GET_BUFFER_MEMORY_REQUIREMENTS2_KHR: CommandInfo = CommandInfo {
        name: "vkGetBufferMemoryRequirements2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_memory_requirements2"],
    };
    /// What the registry states about the C command [`vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html).
    pub const GET_BUFFER_OPAQUE_CAPTURE_ADDRESS: CommandInfo = CommandInfo {
        name: "vkGetBufferOpaqueCaptureAddress",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html).
    pub const GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR: CommandInfo = CommandInfo {
        name: "vkGetBufferOpaqueCaptureAddressKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_buffer_device_address"],
    };
    /// What the registry states about the C command [`vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html).
    pub const GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetBufferOpaqueCaptureDescriptorDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html).
    pub const GET_CALIBRATED_TIMESTAMPS_EXT: CommandInfo = CommandInfo {
        name: "vkGetCalibratedTimestampsEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_calibrated_timestamps"],
    };
    /// What the registry states about the C command [`vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html).
    pub const GET_CALIBRATED_TIMESTAMPS_KHR: CommandInfo = CommandInfo {
        name: "vkGetCalibratedTimestampsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_calibrated_timestamps"],
    };
    /// What the registry states about the C command [`vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html).
    pub const GET_CLUSTER_ACCELERATION_STRUCTURE_BUILD_SIZES_NV: CommandInfo = CommandInfo {
        name: "vkGetClusterAccelerationStructureBuildSizesNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_cluster_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html).
    pub const GET_CUDA_MODULE_CACHE_NV: CommandInfo = CommandInfo {
        name: "vkGetCudaModuleCacheNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cuda_kernel_launch"],
    };
    /// What the registry states about the C command [`vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html).
    pub const GET_DATA_GRAPH_PIPELINE_AVAILABLE_PROPERTIES_ARM: CommandInfo = CommandInfo {
        name: "vkGetDataGraphPipelineAvailablePropertiesARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html).
    pub const GET_DATA_GRAPH_PIPELINE_PROPERTIES_ARM: CommandInfo = CommandInfo {
        name: "vkGetDataGraphPipelinePropertiesARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html).
    pub const GET_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_ARM: CommandInfo =
        CommandInfo {
            name: "vkGetDataGraphPipelineSessionBindPointRequirementsARM",
            level: CommandLevel::Device,
            success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_ARM_data_graph"],
        };
    /// What the registry states about the C command [`vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html).
    pub const GET_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_ARM: CommandInfo = CommandInfo {
        name: "vkGetDataGraphPipelineSessionMemoryRequirementsARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html).
    pub const GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeferredOperationMaxConcurrencyKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_deferred_host_operations"],
    };
    /// What the registry states about the C command [`vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html).
    pub const GET_DEFERRED_OPERATION_RESULT_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeferredOperationResultKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::NOT_READY],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_KHR_deferred_host_operations"],
    };
    /// What the registry states about the C command [`vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html).
    pub const GET_DESCRIPTOR_EXT: CommandInfo = CommandInfo {
        name: "vkGetDescriptorEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html).
    pub const GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetHostMappingVALVE",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_VALVE_descriptor_set_host_mapping"],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html).
    pub const GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetLayoutBindingOffsetEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html).
    pub const GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetLayoutHostMappingInfoVALVE",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_VALVE_descriptor_set_host_mapping"],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html).
    pub const GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetLayoutSizeEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html).
    pub const GET_DESCRIPTOR_SET_LAYOUT_SUPPORT: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetLayoutSupport",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html).
    pub const GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetDescriptorSetLayoutSupportKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance3"],
    };
    /// What the registry states about the C command [`vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html).
    pub const GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceAccelerationStructureCompatibilityKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html).
    pub const GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetDeviceBufferMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html).
    pub const GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceBufferMemoryRequirementsKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance4"],
    };
    /// What the registry states about the C command [`vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html).
    pub const GET_DEVICE_COMBINED_IMAGE_SAMPLER_INDEX_NVX: CommandInfo = CommandInfo {
        name: "vkGetDeviceCombinedImageSamplerIndexNVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_image_view_handle"],
    };
    /// What the registry states about the C command [`vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html).
    pub const GET_DEVICE_FAULT_DEBUG_INFO_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceFaultDebugInfoKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_NOT_ENOUGH_SPACE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_device_fault"],
    };
    /// What the registry states about the C command [`vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html).
    pub const GET_DEVICE_FAULT_INFO_EXT: CommandInfo = CommandInfo {
        name: "vkGetDeviceFaultInfoEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_device_fault"],
    };
    /// What the registry states about the C command [`vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html).
    pub const GET_DEVICE_FAULT_REPORTS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceFaultReportsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE, VkResult::TIMEOUT],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_device_fault"],
    };
    /// What the registry states about the C command [`vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html).
    pub const GET_DEVICE_GROUP_PEER_MEMORY_FEATURES: CommandInfo = CommandInfo {
        name: "vkGetDeviceGroupPeerMemoryFeatures",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html).
    pub const GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceGroupPeerMemoryFeaturesKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html).
    pub const GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceGroupPresentCapabilitiesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain", c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html).
    pub const GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2_EXT: CommandInfo = CommandInfo {
        name: "vkGetDeviceGroupSurfacePresentModes2EXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_full_screen_exclusive"],
    };
    /// What the registry states about the C command [`vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html).
    pub const GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceGroupSurfacePresentModesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain", c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html).
    pub const GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html).
    pub const GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageMemoryRequirementsKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance4"],
    };
    /// What the registry states about the C command [`vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html).
    pub const GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageSparseMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html).
    pub const GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageSparseMemoryRequirementsKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance4"],
    };
    /// What the registry states about the C command [`vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html).
    pub const GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageSubresourceLayout",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html).
    pub const GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceImageSubresourceLayoutKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance5"],
    };
    /// What the registry states about the C command [`vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html).
    pub const GET_DEVICE_MEMORY_COMMITMENT: CommandInfo = CommandInfo {
        name: "vkGetDeviceMemoryCommitment",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html).
    pub const GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS: CommandInfo = CommandInfo {
        name: "vkGetDeviceMemoryOpaqueCaptureAddress",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html).
    pub const GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDeviceMemoryOpaqueCaptureAddressKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_buffer_device_address"],
    };
    /// What the registry states about the C command [`vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html).
    pub const GET_DEVICE_MICROMAP_COMPATIBILITY_EXT: CommandInfo = CommandInfo {
        name: "vkGetDeviceMicromapCompatibilityEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html).
    pub const GET_DEVICE_PROC_ADDR: CommandInfo = CommandInfo {
        name: "vkGetDeviceProcAddr",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html).
    pub const GET_DEVICE_QUEUE: CommandInfo = CommandInfo {
        name: "vkGetDeviceQueue",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html).
    pub const GET_DEVICE_QUEUE2: CommandInfo = CommandInfo {
        name: "vkGetDeviceQueue2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html).
    pub const GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI: CommandInfo = CommandInfo {
        name: "vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_HUAWEI_subpass_shading"],
    };
    /// What the registry states about the C command [`vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html).
    pub const GET_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: CommandInfo = CommandInfo {
        name: "vkGetDeviceTensorMemoryRequirementsARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html).
    pub const GET_DISPLAY_MODE_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetDisplayModeProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_display_properties2"],
    };
    /// What the registry states about the C command [`vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html).
    pub const GET_DISPLAY_MODE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetDisplayModePropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html).
    pub const GET_DISPLAY_PLANE_CAPABILITIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetDisplayPlaneCapabilities2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_display_properties2"],
    };
    /// What the registry states about the C command [`vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html).
    pub const GET_DISPLAY_PLANE_CAPABILITIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetDisplayPlaneCapabilitiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html).
    pub const GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR: CommandInfo = CommandInfo {
        name: "vkGetDisplayPlaneSupportedDisplaysKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html).
    pub const GET_DRM_DISPLAY_EXT: CommandInfo = CommandInfo {
        name: "vkGetDrmDisplayEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_acquire_drm_display"],
    };
    /// What the registry states about the C command [`vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html).
    pub const GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM: CommandInfo = CommandInfo {
        name: "vkGetDynamicRenderingTilePropertiesQCOM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_QCOM_tile_properties"],
    };
    /// What the registry states about the C command [`vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html).
    pub const GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR: CommandInfo = CommandInfo {
        name: "vkGetEncodedVideoSessionParametersKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_encode_queue"],
    };
    /// What the registry states about the C command [`vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html).
    pub const GET_EVENT_STATUS: CommandInfo = CommandInfo {
        name: "vkGetEventStatus",
        level: CommandLevel::Device,
        success_codes: &[VkResult::EVENT_SET, VkResult::EVENT_RESET],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html).
    pub const GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX: CommandInfo = CommandInfo {
        name: "vkGetExecutionGraphPipelineNodeIndexAMDX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html).
    pub const GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: CommandInfo = CommandInfo {
        name: "vkGetExecutionGraphPipelineScratchSizeAMDX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMDX_shader_enqueue"],
    };
    /// What the registry states about the C command [`vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html).
    pub const GET_EXTERNAL_COMPUTE_QUEUE_DATA_NV: CommandInfo = CommandInfo {
        name: "vkGetExternalComputeQueueDataNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_external_compute_queue"],
    };
    /// What the registry states about the C command [`vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html).
    pub const GET_FENCE_FD_KHR: CommandInfo = CommandInfo {
        name: "vkGetFenceFdKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_fence_fd"],
    };
    /// What the registry states about the C command [`vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html).
    pub const GET_FENCE_STATUS: CommandInfo = CommandInfo {
        name: "vkGetFenceStatus",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::NOT_READY],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html).
    pub const GET_FENCE_WIN32_HANDLE_KHR: CommandInfo = CommandInfo {
        name: "vkGetFenceWin32HandleKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_fence_win32"],
    };
    /// What the registry states about the C command [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html).
    pub const GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM: CommandInfo = CommandInfo {
        name: "vkGetFramebufferTilePropertiesQCOM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_QCOM_tile_properties"],
    };
    /// What the registry states about the C command [`vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html).
    pub const GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT: CommandInfo = CommandInfo {
        name: "vkGetGeneratedCommandsMemoryRequirementsEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html).
    pub const GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV: CommandInfo = CommandInfo {
        name: "vkGetGeneratedCommandsMemoryRequirementsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html).
    pub const GET_GPA_DEVICE_CLOCK_INFO_AMD: CommandInfo = CommandInfo {
        name: "vkGetGpaDeviceClockInfoAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html).
    pub const GET_GPA_SESSION_RESULTS_AMD: CommandInfo = CommandInfo {
        name: "vkGetGpaSessionResultsAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html).
    pub const GET_GPA_SESSION_STATUS_AMD: CommandInfo = CommandInfo {
        name: "vkGetGpaSessionStatusAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html).
    pub const GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetImageDrmFormatModifierPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_image_drm_format_modifier"],
    };
    /// What the registry states about the C command [`vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html).
    pub const GET_IMAGE_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetImageMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html).
    pub const GET_IMAGE_MEMORY_REQUIREMENTS2: CommandInfo = CommandInfo {
        name: "vkGetImageMemoryRequirements2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html).
    pub const GET_IMAGE_MEMORY_REQUIREMENTS2_KHR: CommandInfo = CommandInfo {
        name: "vkGetImageMemoryRequirements2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_memory_requirements2"],
    };
    /// What the registry states about the C command [`vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html).
    pub const GET_IMAGE_OPAQUE_CAPTURE_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetImageOpaqueCaptureDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html).
    pub const GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetImageOpaqueCaptureDescriptorDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html).
    pub const GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS: CommandInfo = CommandInfo {
        name: "vkGetImageSparseMemoryRequirements",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html).
    pub const GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2: CommandInfo = CommandInfo {
        name: "vkGetImageSparseMemoryRequirements2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html).
    pub const GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR: CommandInfo = CommandInfo {
        name: "vkGetImageSparseMemoryRequirements2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_memory_requirements2"],
    };
    /// What the registry states about the C command [`vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html).
    pub const GET_IMAGE_SUBRESOURCE_LAYOUT: CommandInfo = CommandInfo {
        name: "vkGetImageSubresourceLayout",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html).
    pub const GET_IMAGE_SUBRESOURCE_LAYOUT2: CommandInfo = CommandInfo {
        name: "vkGetImageSubresourceLayout2",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html).
    pub const GET_IMAGE_SUBRESOURCE_LAYOUT2_EXT: CommandInfo = CommandInfo {
        name: "vkGetImageSubresourceLayout2EXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[
            c"VK_EXT_host_image_copy",
            c"VK_EXT_image_compression_control",
        ],
    };
    /// What the registry states about the C command [`vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html).
    pub const GET_IMAGE_SUBRESOURCE_LAYOUT2_KHR: CommandInfo = CommandInfo {
        name: "vkGetImageSubresourceLayout2KHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance5"],
    };
    /// What the registry states about the C command [`vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html).
    pub const GET_IMAGE_VIEW_ADDRESS_NVX: CommandInfo = CommandInfo {
        name: "vkGetImageViewAddressNVX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NVX_image_view_handle"],
    };
    /// What the registry states about the C command [`vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html).
    pub const GET_IMAGE_VIEW_HANDLE64_NVX: CommandInfo = CommandInfo {
        name: "vkGetImageViewHandle64NVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_image_view_handle"],
    };
    /// What the registry states about the C command [`vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html).
    pub const GET_IMAGE_VIEW_HANDLE_NVX: CommandInfo = CommandInfo {
        name: "vkGetImageViewHandleNVX",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NVX_image_view_handle"],
    };
    /// What the registry states about the C command [`vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html).
    pub const GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetImageViewOpaqueCaptureDescriptorDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html).
    pub const GET_INSTANCE_PROC_ADDR: CommandInfo = CommandInfo {
        name: "vkGetInstanceProcAddr",
        level: CommandLevel::Entry,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetLatencyTimingsLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html).
    pub const GET_LATENCY_TIMINGS_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkGetLatencyTimingsLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html).
    pub const GET_LATENCY_TIMINGS_NV: CommandInfo = CommandInfo {
        name: "vkGetLatencyTimingsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency2"],
    };
    /// What the registry states about the C command [`vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html).
    pub const GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID: CommandInfo = CommandInfo {
        name: "vkGetMemoryAndroidHardwareBufferANDROID",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ANDROID_external_memory_android_hardware_buffer"],
    };
    /// What the registry states about the C command [`vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html).
    pub const GET_MEMORY_FD_KHR: CommandInfo = CommandInfo {
        name: "vkGetMemoryFdKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_memory_fd"],
    };
    /// What the registry states about the C command [`vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html).
    pub const GET_MEMORY_FD_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetMemoryFdPropertiesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_memory_fd"],
    };
    /// What the registry states about the C command [`vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html).
    pub const GET_MEMORY_HOST_POINTER_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetMemoryHostPointerPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_external_memory_host"],
    };
    /// What the registry states about the C command [`vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html).
    pub const GET_MEMORY_METAL_HANDLE_EXT: CommandInfo = CommandInfo {
        name: "vkGetMemoryMetalHandleEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_external_memory_metal"],
    };
    /// What the registry states about the C command [`vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html).
    pub const GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetMemoryMetalHandlePropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_external_memory_metal"],
    };
    /// What the registry states about the C command [`vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html).
    pub const GET_MEMORY_NATIVE_BUFFER_OHOS: CommandInfo = CommandInfo {
        name: "vkGetMemoryNativeBufferOHOS",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_OHOS_external_memory"],
    };
    /// What the registry states about the C command [`vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html).
    pub const GET_MEMORY_REMOTE_ADDRESS_NV: CommandInfo = CommandInfo {
        name: "vkGetMemoryRemoteAddressNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_external_memory_rdma"],
    };
    /// What the registry states about the C command [`vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html).
    pub const GET_MEMORY_WIN32_HANDLE_KHR: CommandInfo = CommandInfo {
        name: "vkGetMemoryWin32HandleKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_memory_win32"],
    };
    /// What the registry states about the C command [`vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html).
    pub const GET_MEMORY_WIN32_HANDLE_NV: CommandInfo = CommandInfo {
        name: "vkGetMemoryWin32HandleNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_external_memory_win32"],
    };
    /// What the registry states about the C command [`vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html).
    pub const GET_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetMemoryWin32HandlePropertiesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_memory_win32"],
    };
    /// What the registry states about the C command [`vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html).
    pub const GET_MEMORY_ZIRCON_HANDLE_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkGetMemoryZirconHandleFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_external_memory"],
    };
    /// What the registry states about the C command [`vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html).
    pub const GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkGetMemoryZirconHandlePropertiesFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_external_memory"],
    };
    /// What the registry states about the C command [`vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html).
    pub const GET_MICROMAP_BUILD_SIZES_EXT: CommandInfo = CommandInfo {
        name: "vkGetMicromapBuildSizesEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html).
    pub const GET_NATIVE_BUFFER_PROPERTIES_OHOS: CommandInfo = CommandInfo {
        name: "vkGetNativeBufferPropertiesOHOS",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_OHOS_external_memory"],
    };
    /// What the registry states about the C command [`vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html).
    pub const GET_PARTITIONED_ACCELERATION_STRUCTURES_BUILD_SIZES_NV: CommandInfo = CommandInfo {
        name: "vkGetPartitionedAccelerationStructuresBuildSizesNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_partitioned_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html).
    pub const GET_PAST_PRESENTATION_TIMING_EXT: CommandInfo = CommandInfo {
        name: "vkGetPastPresentationTimingEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_present_timing"],
    };
    /// What the registry states about the C command [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html).
    pub const GET_PAST_PRESENTATION_TIMING_GOOGLE: CommandInfo = CommandInfo {
        name: "vkGetPastPresentationTimingGOOGLE",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_GOOGLE_display_timing"],
    };
    /// What the registry states about the C command [`vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html).
    pub const GET_PERFORMANCE_PARAMETER_INTEL: CommandInfo = CommandInfo {
        name: "vkGetPerformanceParameterINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html).
    pub const GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_calibrated_timestamps"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html).
    pub const GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_calibrated_timestamps"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html).
    pub const GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV:
        CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cooperative_matrix2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html).
    pub const GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES2_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCooperativeMatrixProperties2EXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_cooperative_matrix_maintenance1"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_cooperative_matrix"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html).
    pub const GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cooperative_matrix"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html).
    pub const GET_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_cooperative_vector"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html).
    pub const GET_PHYSICAL_DEVICE_DESCRIPTOR_SIZE_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDescriptorSizeEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html).
    pub const GET_PHYSICAL_DEVICE_DIRECT_FB_PRESENTATION_SUPPORT_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_directfb_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html).
    pub const GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_display_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html).
    pub const GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDisplayProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_display_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceDisplayPropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_display"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalBufferProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalBufferPropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_external_memory_capabilities"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalFenceProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalFencePropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_external_fence_capabilities"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_external_memory_capabilities"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalSemaphoreProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_external_semaphore_capabilities"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html).
    pub const GET_PHYSICAL_DEVICE_EXTERNAL_TENSOR_PROPERTIES_ARM: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceExternalTensorPropertiesARM",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html).
    pub const GET_PHYSICAL_DEVICE_FEATURES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFeatures",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html).
    pub const GET_PHYSICAL_DEVICE_FEATURES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFeatures2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html).
    pub const GET_PHYSICAL_DEVICE_FEATURES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFeatures2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html).
    pub const GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFormatProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFormatProperties2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFormatProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html).
    pub const GET_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceFragmentShadingRatesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_fragment_shading_rate"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html).
    pub const GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceImageFormatProperties",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceImageFormatProperties2",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceImageFormatProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html).
    pub const GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceMemoryProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html).
    pub const GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceMemoryProperties2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html).
    pub const GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceMemoryProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html).
    pub const GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceMultisamplePropertiesEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_sample_locations"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html).
    pub const GET_PHYSICAL_DEVICE_OPTICAL_FLOW_IMAGE_FORMATS_NV: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_EXTENSION_NOT_PRESENT,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_optical_flow"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html).
    pub const GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDevicePresentRectanglesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain", c"VK_KHR_device_group"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html).
    pub const GET_PHYSICAL_DEVICE_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html).
    pub const GET_PHYSICAL_DEVICE_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceProperties2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html).
    pub const GET_PHYSICAL_DEVICE_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_ENGINE_OPERATION_PROPERTIES_ARM:
        CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[
            c"VK_ARM_data_graph_instruction_set_tosa",
            c"VK_ARM_data_graph_optical_flow",
        ],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMATS_ARM:
        CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_EXTENSION_NOT_PRESENT,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_data_graph_optical_flow"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM:
        CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_data_graph"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: CommandInfo =
        CommandInfo {
            name: "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
            level: CommandLevel::Instance,
            success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_ARM_data_graph"],
        };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR: CommandInfo =
        CommandInfo {
            name: "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
            level: CommandLevel::Instance,
            success_codes: &[],
            error_codes: &[],
            version: None,
            extensions: &[c"VK_KHR_performance_query"],
        };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyProperties2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html).
    pub const GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceQueueFamilyProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html).
    pub const GET_PHYSICAL_DEVICE_SCREEN_PRESENTATION_SUPPORT_QNX: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceScreenPresentationSupportQNX",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_QNX_screen_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html).
    pub const GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSparseImageFormatProperties",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSparseImageFormatProperties2",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html).
    pub const GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_get_physical_device_properties2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html).
    pub const GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV: CommandInfo =
        CommandInfo {
            name: "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
            level: CommandLevel::Instance,
            success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_NV_coverage_reduction_mode"],
        };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceCapabilities2EXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_display_surface_counter"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceCapabilities2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_surface_capabilities2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_FORMATS2_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceFormats2KHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_get_surface_capabilities2"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceFormatsKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES2_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfacePresentModes2EXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_full_screen_exclusive"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfacePresentModesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html).
    pub const GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceSurfaceSupportKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html).
    pub const GET_PHYSICAL_DEVICE_TOOL_PROPERTIES: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceToolProperties",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html).
    pub const GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceToolPropertiesEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_tooling_info"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html).
    pub const GET_PHYSICAL_DEVICE_UBM_PRESENTATION_SUPPORT_SEC: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceUbmPresentationSupportSEC",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_SEC_ubm_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_VIDEO_CAPABILITIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceVideoCapabilitiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: CommandInfo =
        CommandInfo {
            name: "vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
            level: CommandLevel::Instance,
            success_codes: &[VkResult::SUCCESS],
            error_codes: &[
                VkResult::ERROR_OUT_OF_HOST_MEMORY,
                VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
                VkResult::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
                VkResult::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
                VkResult::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
                VkResult::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
                VkResult::ERROR_UNKNOWN,
                VkResult::ERROR_VALIDATION_FAILED,
            ],
            version: None,
            extensions: &[c"VK_KHR_video_encode_queue"],
        };
    /// What the registry states about the C command [`vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html).
    pub const GET_PHYSICAL_DEVICE_VIDEO_FORMAT_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceVideoFormatPropertiesKHR",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
            VkResult::ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html).
    pub const GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceWaylandPresentationSupportKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_wayland_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html).
    pub const GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceWin32PresentationSupportKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_win32_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html).
    pub const GET_PHYSICAL_DEVICE_XCB_PRESENTATION_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceXcbPresentationSupportKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_xcb_surface"],
    };
    /// What the registry states about the C command [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html).
    pub const GET_PHYSICAL_DEVICE_XLIB_PRESENTATION_SUPPORT_KHR: CommandInfo = CommandInfo {
        name: "vkGetPhysicalDeviceXlibPresentationSupportKHR",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_xlib_surface"],
    };
    /// What the registry states about the C command [`vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html).
    pub const GET_PIPELINE_BINARY_DATA_KHR: CommandInfo = CommandInfo {
        name: "vkGetPipelineBinaryDataKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_NOT_ENOUGH_SPACE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_binary"],
    };
    /// What the registry states about the C command [`vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html).
    pub const GET_PIPELINE_CACHE_DATA: CommandInfo = CommandInfo {
        name: "vkGetPipelineCacheData",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html).
    pub const GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR: CommandInfo = CommandInfo {
        name: "vkGetPipelineExecutableInternalRepresentationsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_executable_properties"],
    };
    /// What the registry states about the C command [`vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html).
    pub const GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkGetPipelineExecutablePropertiesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_executable_properties"],
    };
    /// What the registry states about the C command [`vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html).
    pub const GET_PIPELINE_EXECUTABLE_STATISTICS_KHR: CommandInfo = CommandInfo {
        name: "vkGetPipelineExecutableStatisticsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_executable_properties"],
    };
    /// What the registry states about the C command [`vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html).
    pub const GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV: CommandInfo = CommandInfo {
        name: "vkGetPipelineIndirectDeviceAddressNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands_compute"],
    };
    /// What the registry states about the C command [`vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html).
    pub const GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV: CommandInfo = CommandInfo {
        name: "vkGetPipelineIndirectMemoryRequirementsNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_generated_commands_compute"],
    };
    /// What the registry states about the C command [`vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html).
    pub const GET_PIPELINE_KEY_KHR: CommandInfo = CommandInfo {
        name: "vkGetPipelineKeyKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_pipeline_binary"],
    };
    /// What the registry states about the C command [`vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html).
    pub const GET_PIPELINE_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetPipelinePropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_pipeline_properties"],
    };
    /// What the registry states about the C command [`vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html).
    pub const GET_PRIVATE_DATA: CommandInfo = CommandInfo {
        name: "vkGetPrivateData",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html).
    pub const GET_PRIVATE_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetPrivateDataEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_private_data"],
    };
    /// What the registry states about the C command [`vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html).
    pub const GET_QUERY_POOL_RESULTS: CommandInfo = CommandInfo {
        name: "vkGetQueryPoolResults",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::NOT_READY],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html).
    pub const GET_QUEUE_CHECKPOINT_DATA2_NV: CommandInfo = CommandInfo {
        name: "vkGetQueueCheckpointData2NV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_diagnostic_checkpoints"],
    };
    /// What the registry states about the C command [`vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html).
    pub const GET_QUEUE_CHECKPOINT_DATA_NV: CommandInfo = CommandInfo {
        name: "vkGetQueueCheckpointDataNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_device_diagnostic_checkpoints"],
    };
    /// What the registry states about the C command [`vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html).
    pub const GET_RAND_R_OUTPUT_DISPLAY_EXT: CommandInfo = CommandInfo {
        name: "vkGetRandROutputDisplayEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_acquire_xlib_display"],
    };
    /// What the registry states about the C command [`vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html).
    pub const GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR: CommandInfo = CommandInfo {
        name: "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html).
    pub const GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR: CommandInfo = CommandInfo {
        name: "vkGetRayTracingShaderGroupHandlesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html).
    pub const GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV: CommandInfo = CommandInfo {
        name: "vkGetRayTracingShaderGroupHandlesNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_ray_tracing"],
    };
    /// What the registry states about the C command [`vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html).
    pub const GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR: CommandInfo = CommandInfo {
        name: "vkGetRayTracingShaderGroupStackSizeKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_ray_tracing_pipeline"],
    };
    /// What the registry states about the C command [`vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html).
    pub const GET_REFRESH_CYCLE_DURATION_GOOGLE: CommandInfo = CommandInfo {
        name: "vkGetRefreshCycleDurationGOOGLE",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_GOOGLE_display_timing"],
    };
    /// What the registry states about the C command [`vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html).
    pub const GET_RENDER_AREA_GRANULARITY: CommandInfo = CommandInfo {
        name: "vkGetRenderAreaGranularity",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html).
    pub const GET_RENDERING_AREA_GRANULARITY: CommandInfo = CommandInfo {
        name: "vkGetRenderingAreaGranularity",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html).
    pub const GET_RENDERING_AREA_GRANULARITY_KHR: CommandInfo = CommandInfo {
        name: "vkGetRenderingAreaGranularityKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance5"],
    };
    /// What the registry states about the C command [`vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html).
    pub const GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetSamplerOpaqueCaptureDescriptorDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_buffer"],
    };
    /// What the registry states about the C command [`vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html).
    pub const GET_SCREEN_BUFFER_PROPERTIES_QNX: CommandInfo = CommandInfo {
        name: "vkGetScreenBufferPropertiesQNX",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_QNX_external_memory_screen_buffer"],
    };
    /// What the registry states about the C command [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html).
    pub const GET_SEMAPHORE_COUNTER_VALUE: CommandInfo = CommandInfo {
        name: "vkGetSemaphoreCounterValue",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html).
    pub const GET_SEMAPHORE_COUNTER_VALUE_KHR: CommandInfo = CommandInfo {
        name: "vkGetSemaphoreCounterValueKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_timeline_semaphore"],
    };
    /// What the registry states about the C command [`vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html).
    pub const GET_SEMAPHORE_FD_KHR: CommandInfo = CommandInfo {
        name: "vkGetSemaphoreFdKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_semaphore_fd"],
    };
    /// What the registry states about the C command [`vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html).
    pub const GET_SEMAPHORE_WIN32_HANDLE_KHR: CommandInfo = CommandInfo {
        name: "vkGetSemaphoreWin32HandleKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_semaphore_win32"],
    };
    /// What the registry states about the C command [`vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html).
    pub const GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkGetSemaphoreZirconHandleFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_external_semaphore"],
    };
    /// What the registry states about the C command [`vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html).
    pub const GET_SHADER_BINARY_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetShaderBinaryDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_shader_object"],
    };
    /// What the registry states about the C command [`vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html).
    pub const GET_SHADER_INFO_AMD: CommandInfo = CommandInfo {
        name: "vkGetShaderInfoAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_FEATURE_NOT_PRESENT,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_shader_info"],
    };
    /// What the registry states about the C command [`vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html).
    pub const GET_SHADER_INSTRUMENTATION_VALUES_ARM: CommandInfo = CommandInfo {
        name: "vkGetShaderInstrumentationValuesARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_shader_instrumentation"],
    };
    /// What the registry states about the C command [`vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html).
    pub const GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT: CommandInfo = CommandInfo {
        name: "vkGetShaderModuleCreateInfoIdentifierEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_shader_module_identifier"],
    };
    /// What the registry states about the C command [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html).
    pub const GET_SHADER_MODULE_IDENTIFIER_EXT: CommandInfo = CommandInfo {
        name: "vkGetShaderModuleIdentifierEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_shader_module_identifier"],
    };
    /// What the registry states about the C command [`vkGetSleepStatusLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html).
    pub const GET_SLEEP_STATUS_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkGetSleepStatusLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html).
    pub const GET_SWAPCHAIN_COUNTER_EXT: CommandInfo = CommandInfo {
        name: "vkGetSwapchainCounterEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_display_control"],
    };
    /// What the registry states about the C command [`vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html).
    pub const GET_SWAPCHAIN_IMAGES_KHR: CommandInfo = CommandInfo {
        name: "vkGetSwapchainImagesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain"],
    };
    /// What the registry states about the C command [`vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html).
    pub const GET_SWAPCHAIN_STATUS_KHR: CommandInfo = CommandInfo {
        name: "vkGetSwapchainStatusKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::SUBOPTIMAL_KHR],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_shared_presentable_image"],
    };
    /// What the registry states about the C command [`vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html).
    pub const GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetSwapchainTimeDomainPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_present_timing"],
    };
    /// What the registry states about the C command [`vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html).
    pub const GET_SWAPCHAIN_TIMING_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkGetSwapchainTimingPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::NOT_READY],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_present_timing"],
    };
    /// What the registry states about the C command [`vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html).
    pub const GET_TENSOR_MEMORY_REQUIREMENTS_ARM: CommandInfo = CommandInfo {
        name: "vkGetTensorMemoryRequirementsARM",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html).
    pub const GET_TENSOR_OPAQUE_CAPTURE_DATA_ARM: CommandInfo = CommandInfo {
        name: "vkGetTensorOpaqueCaptureDataARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html).
    pub const GET_TENSOR_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM: CommandInfo = CommandInfo {
        name: "vkGetTensorOpaqueCaptureDescriptorDataARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html).
    pub const GET_TENSOR_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM: CommandInfo = CommandInfo {
        name: "vkGetTensorViewOpaqueCaptureDescriptorDataARM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_ARM_tensors"],
    };
    /// What the registry states about the C command [`vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html).
    pub const GET_VALIDATION_CACHE_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkGetValidationCacheDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_validation_cache"],
    };
    /// What the registry states about the C command [`vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html).
    pub const GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: CommandInfo = CommandInfo {
        name: "vkGetVideoSessionMemoryRequirementsKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::INCOMPLETE],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html).
    pub const GET_WINRT_DISPLAY_NV: CommandInfo = CommandInfo {
        name: "vkGetWinrtDisplayNV",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_acquire_winrt_display"],
    };
    /// What the registry states about the C command [`vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html).
    pub const IMPORT_FENCE_FD_KHR: CommandInfo = CommandInfo {
        name: "vkImportFenceFdKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_fence_fd"],
    };
    /// What the registry states about the C command [`vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html).
    pub const IMPORT_FENCE_WIN32_HANDLE_KHR: CommandInfo = CommandInfo {
        name: "vkImportFenceWin32HandleKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_fence_win32"],
    };
    /// What the registry states about the C command [`vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html).
    pub const IMPORT_SEMAPHORE_FD_KHR: CommandInfo = CommandInfo {
        name: "vkImportSemaphoreFdKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_semaphore_fd"],
    };
    /// What the registry states about the C command [`vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html).
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_KHR: CommandInfo = CommandInfo {
        name: "vkImportSemaphoreWin32HandleKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_external_semaphore_win32"],
    };
    /// What the registry states about the C command [`vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html).
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkImportSemaphoreZirconHandleFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_INVALID_EXTERNAL_HANDLE,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_external_semaphore"],
    };
    /// What the registry states about the C command [`vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html).
    pub const INITIALIZE_PERFORMANCE_API_INTEL: CommandInfo = CommandInfo {
        name: "vkInitializePerformanceApiINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html).
    pub const INVALIDATE_MAPPED_MEMORY_RANGES: CommandInfo = CommandInfo {
        name: "vkInvalidateMappedMemoryRanges",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkLatencySleepLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html).
    pub const LATENCY_SLEEP_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkLatencySleepLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html).
    pub const LATENCY_SLEEP_NV: CommandInfo = CommandInfo {
        name: "vkLatencySleepNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_NV_low_latency2"],
    };
    /// What the registry states about the C command [`vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html).
    pub const MAP_MEMORY: CommandInfo = CommandInfo {
        name: "vkMapMemory",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html).
    pub const MAP_MEMORY2: CommandInfo = CommandInfo {
        name: "vkMapMemory2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html).
    pub const MAP_MEMORY2_KHR: CommandInfo = CommandInfo {
        name: "vkMapMemory2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_map_memory2"],
    };
    /// What the registry states about the C command [`vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html).
    pub const MERGE_PIPELINE_CACHES: CommandInfo = CommandInfo {
        name: "vkMergePipelineCaches",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html).
    pub const MERGE_VALIDATION_CACHES_EXT: CommandInfo = CommandInfo {
        name: "vkMergeValidationCachesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_validation_cache"],
    };
    /// What the registry states about the C command [`vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html).
    pub const QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkQueueBeginDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html).
    pub const QUEUE_BIND_SPARSE: CommandInfo = CommandInfo {
        name: "vkQueueBindSparse",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html).
    pub const QUEUE_END_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkQueueEndDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html).
    pub const QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT: CommandInfo = CommandInfo {
        name: "vkQueueInsertDebugUtilsLabelEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkQueueNotifyOutOfBandLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html).
    pub const QUEUE_NOTIFY_OUT_OF_BAND_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkQueueNotifyOutOfBandLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html).
    pub const QUEUE_NOTIFY_OUT_OF_BAND_NV: CommandInfo = CommandInfo {
        name: "vkQueueNotifyOutOfBandNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency2"],
    };
    /// What the registry states about the C command [`vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html).
    pub const QUEUE_PRESENT_KHR: CommandInfo = CommandInfo {
        name: "vkQueuePresentKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::SUBOPTIMAL_KHR],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
            VkResult::ERROR_PRESENT_TIMING_QUEUE_FULL_EXT,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain"],
    };
    /// What the registry states about the C command [`vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html).
    pub const QUEUE_SET_PERF_HINT_QCOM: CommandInfo = CommandInfo {
        name: "vkQueueSetPerfHintQCOM",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_QCOM_queue_perf_hint"],
    };
    /// What the registry states about the C command [`vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html).
    pub const QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL: CommandInfo = CommandInfo {
        name: "vkQueueSetPerformanceConfigurationINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html).
    pub const QUEUE_SUBMIT: CommandInfo = CommandInfo {
        name: "vkQueueSubmit",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html).
    pub const QUEUE_SUBMIT2: CommandInfo = CommandInfo {
        name: "vkQueueSubmit2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html).
    pub const QUEUE_SUBMIT2_KHR: CommandInfo = CommandInfo {
        name: "vkQueueSubmit2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_synchronization2"],
    };
    /// What the registry states about the C command [`vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html).
    pub const QUEUE_WAIT_IDLE: CommandInfo = CommandInfo {
        name: "vkQueueWaitIdle",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html).
    pub const REGISTER_CUSTOM_BORDER_COLOR_EXT: CommandInfo = CommandInfo {
        name: "vkRegisterCustomBorderColorEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html).
    pub const REGISTER_DEVICE_EVENT_EXT: CommandInfo = CommandInfo {
        name: "vkRegisterDeviceEventEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_display_control"],
    };
    /// What the registry states about the C command [`vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html).
    pub const REGISTER_DISPLAY_EVENT_EXT: CommandInfo = CommandInfo {
        name: "vkRegisterDisplayEventEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_display_control"],
    };
    /// What the registry states about the C command [`vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html).
    pub const RELEASE_CAPTURED_PIPELINE_DATA_KHR: CommandInfo = CommandInfo {
        name: "vkReleaseCapturedPipelineDataKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_KHR_pipeline_binary"],
    };
    /// What the registry states about the C command [`vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html).
    pub const RELEASE_DISPLAY_EXT: CommandInfo = CommandInfo {
        name: "vkReleaseDisplayEXT",
        level: CommandLevel::Instance,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[c"VK_EXT_direct_mode_display"],
    };
    /// What the registry states about the C command [`vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html).
    pub const RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT: CommandInfo = CommandInfo {
        name: "vkReleaseFullScreenExclusiveModeEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_full_screen_exclusive"],
    };
    /// What the registry states about the C command [`vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html).
    pub const RELEASE_PERFORMANCE_CONFIGURATION_INTEL: CommandInfo = CommandInfo {
        name: "vkReleasePerformanceConfigurationINTEL",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_TOO_MANY_OBJECTS,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html).
    pub const RELEASE_PROFILING_LOCK_KHR: CommandInfo = CommandInfo {
        name: "vkReleaseProfilingLockKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_performance_query"],
    };
    /// What the registry states about the C command [`vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html).
    pub const RELEASE_SWAPCHAIN_IMAGES_EXT: CommandInfo = CommandInfo {
        name: "vkReleaseSwapchainImagesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_swapchain_maintenance1"],
    };
    /// What the registry states about the C command [`vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html).
    pub const RELEASE_SWAPCHAIN_IMAGES_KHR: CommandInfo = CommandInfo {
        name: "vkReleaseSwapchainImagesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_swapchain_maintenance1"],
    };
    /// What the registry states about the C command [`vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html).
    pub const RESET_COMMAND_BUFFER: CommandInfo = CommandInfo {
        name: "vkResetCommandBuffer",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html).
    pub const RESET_COMMAND_POOL: CommandInfo = CommandInfo {
        name: "vkResetCommandPool",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html).
    pub const RESET_DESCRIPTOR_POOL: CommandInfo = CommandInfo {
        name: "vkResetDescriptorPool",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[VkResult::ERROR_UNKNOWN, VkResult::ERROR_VALIDATION_FAILED],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html).
    pub const RESET_EVENT: CommandInfo = CommandInfo {
        name: "vkResetEvent",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html).
    pub const RESET_FENCES: CommandInfo = CommandInfo {
        name: "vkResetFences",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html).
    pub const RESET_GPA_SESSION_AMD: CommandInfo = CommandInfo {
        name: "vkResetGpaSessionAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html).
    pub const RESET_QUERY_POOL: CommandInfo = CommandInfo {
        name: "vkResetQueryPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html).
    pub const RESET_QUERY_POOL_EXT: CommandInfo = CommandInfo {
        name: "vkResetQueryPoolEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_host_query_reset"],
    };
    /// What the registry states about the C command [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html).
    pub const SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkSetBufferCollectionBufferConstraintsFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_buffer_collection"],
    };
    /// What the registry states about the C command [`vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html).
    pub const SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA: CommandInfo = CommandInfo {
        name: "vkSetBufferCollectionImageConstraintsFUCHSIA",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_FORMAT_NOT_SUPPORTED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_FUCHSIA_buffer_collection"],
    };
    /// What the registry states about the C command [`vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html).
    pub const SET_DEBUG_UTILS_OBJECT_NAME_EXT: CommandInfo = CommandInfo {
        name: "vkSetDebugUtilsObjectNameEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html).
    pub const SET_DEBUG_UTILS_OBJECT_TAG_EXT: CommandInfo = CommandInfo {
        name: "vkSetDebugUtilsObjectTagEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html).
    pub const SET_DEVICE_MEMORY_PRIORITY_EXT: CommandInfo = CommandInfo {
        name: "vkSetDeviceMemoryPriorityEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_pageable_device_local_memory"],
    };
    /// What the registry states about the C command [`vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html).
    pub const SET_EVENT: CommandInfo = CommandInfo {
        name: "vkSetEvent",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html).
    pub const SET_GPA_DEVICE_CLOCK_MODE_AMD: CommandInfo = CommandInfo {
        name: "vkSetGpaDeviceClockModeAMD",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_AMD_gpa_interface"],
    };
    /// What the registry states about the C command [`vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html).
    pub const SET_HDR_METADATA_EXT: CommandInfo = CommandInfo {
        name: "vkSetHdrMetadataEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_hdr_metadata"],
    };
    /// What the registry states about the C command [`vkSetLatencyMarkerLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html).
    pub const SET_LATENCY_MARKER_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkSetLatencyMarkerLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html).
    pub const SET_LATENCY_MARKER_NV: CommandInfo = CommandInfo {
        name: "vkSetLatencyMarkerNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency2"],
    };
    /// What the registry states about the C command [`vkSetLatencySleepModeLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html).
    pub const SET_LATENCY_SLEEP_MODE_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkSetLatencySleepModeLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html).
    pub const SET_LATENCY_SLEEP_MODE_NV: CommandInfo = CommandInfo {
        name: "vkSetLatencySleepModeNV",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_NV_low_latency2"],
    };
    /// What the registry states about the C command [`vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html).
    pub const SET_LOCAL_DIMMING_AMD: CommandInfo = CommandInfo {
        name: "vkSetLocalDimmingAMD",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_AMD_display_native_hdr"],
    };
    /// What the registry states about the C command [`vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html).
    pub const SET_PRIVATE_DATA: CommandInfo = CommandInfo {
        name: "vkSetPrivateData",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 3, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html).
    pub const SET_PRIVATE_DATA_EXT: CommandInfo = CommandInfo {
        name: "vkSetPrivateDataEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_private_data"],
    };
    /// What the registry states about the C command [`vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html).
    pub const SET_SWAPCHAIN_PRESENT_TIMING_QUEUE_SIZE_EXT: CommandInfo = CommandInfo {
        name: "vkSetSwapchainPresentTimingQueueSizeEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::NOT_READY],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_present_timing"],
    };
    /// What the registry states about the C command [`vkShutdownLatencyDeviceLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html).
    pub const SHUTDOWN_LATENCY_DEVICE_LEGACY_NV: CommandInfo = CommandInfo {
        name: "vkShutdownLatencyDeviceLegacyNV",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_NV_low_latency"],
    };
    /// What the registry states about the C command [`vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html).
    pub const SIGNAL_SEMAPHORE: CommandInfo = CommandInfo {
        name: "vkSignalSemaphore",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html).
    pub const SIGNAL_SEMAPHORE_KHR: CommandInfo = CommandInfo {
        name: "vkSignalSemaphoreKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_timeline_semaphore"],
    };
    /// What the registry states about the C command [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html).
    pub const SUBMIT_DEBUG_UTILS_MESSAGE_EXT: CommandInfo = CommandInfo {
        name: "vkSubmitDebugUtilsMessageEXT",
        level: CommandLevel::Instance,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_debug_utils"],
    };
    /// What the registry states about the C command [`vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html).
    pub const TRANSITION_IMAGE_LAYOUT: CommandInfo = CommandInfo {
        name: "vkTransitionImageLayout",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html).
    pub const TRANSITION_IMAGE_LAYOUT_EXT: CommandInfo = CommandInfo {
        name: "vkTransitionImageLayoutEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_INITIALIZATION_FAILED,
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_host_image_copy"],
    };
    /// What the registry states about the C command [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html).
    pub const TRIM_COMMAND_POOL: CommandInfo = CommandInfo {
        name: "vkTrimCommandPool",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html).
    pub const TRIM_COMMAND_POOL_KHR: CommandInfo = CommandInfo {
        name: "vkTrimCommandPoolKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_maintenance1"],
    };
    /// What the registry states about the C command [`vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html).
    pub const UNINITIALIZE_PERFORMANCE_API_INTEL: CommandInfo = CommandInfo {
        name: "vkUninitializePerformanceApiINTEL",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_INTEL_performance_query"],
    };
    /// What the registry states about the C command [`vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html).
    pub const UNMAP_MEMORY: CommandInfo = CommandInfo {
        name: "vkUnmapMemory",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html).
    pub const UNMAP_MEMORY2: CommandInfo = CommandInfo {
        name: "vkUnmapMemory2",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 4, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html).
    pub const UNMAP_MEMORY2_KHR: CommandInfo = CommandInfo {
        name: "vkUnmapMemory2KHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_MEMORY_MAP_FAILED,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_map_memory2"],
    };
    /// What the registry states about the C command [`vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html).
    pub const UNREGISTER_CUSTOM_BORDER_COLOR_EXT: CommandInfo = CommandInfo {
        name: "vkUnregisterCustomBorderColorEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html).
    pub const UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE: CommandInfo = CommandInfo {
        name: "vkUpdateDescriptorSetWithTemplate",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: Some(make_api_version(0, 1, 1, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html).
    pub const UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR: CommandInfo = CommandInfo {
        name: "vkUpdateDescriptorSetWithTemplateKHR",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_KHR_descriptor_update_template"],
    };
    /// What the registry states about the C command [`vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html).
    pub const UPDATE_DESCRIPTOR_SETS: CommandInfo = CommandInfo {
        name: "vkUpdateDescriptorSets",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html).
    pub const UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: CommandInfo = CommandInfo {
        name: "vkUpdateIndirectExecutionSetPipelineEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html).
    pub const UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT: CommandInfo = CommandInfo {
        name: "vkUpdateIndirectExecutionSetShaderEXT",
        level: CommandLevel::Device,
        success_codes: &[],
        error_codes: &[],
        version: None,
        extensions: &[c"VK_EXT_device_generated_commands"],
    };
    /// What the registry states about the C command [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html).
    pub const UPDATE_VIDEO_SESSION_PARAMETERS_KHR: CommandInfo = CommandInfo {
        name: "vkUpdateVideoSessionParametersKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_video_queue"],
    };
    /// What the registry states about the C command [`vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html).
    pub const WAIT_FOR_FENCES: CommandInfo = CommandInfo {
        name: "vkWaitForFences",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::TIMEOUT],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[],
    };
    /// What the registry states about the C command [`vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html).
    pub const WAIT_FOR_PRESENT2_KHR: CommandInfo = CommandInfo {
        name: "vkWaitForPresent2KHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::TIMEOUT,
            VkResult::SUBOPTIMAL_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_present_wait2"],
    };
    /// What the registry states about the C command [`vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html).
    pub const WAIT_FOR_PRESENT_KHR: CommandInfo = CommandInfo {
        name: "vkWaitForPresentKHR",
        level: CommandLevel::Device,
        success_codes: &[
            VkResult::SUCCESS,
            VkResult::TIMEOUT,
            VkResult::SUBOPTIMAL_KHR,
        ],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_OUT_OF_DATE_KHR,
            VkResult::ERROR_SURFACE_LOST_KHR,
            VkResult::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_present_wait"],
    };
    /// What the registry states about the C command [`vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html).
    pub const WAIT_SEMAPHORES: CommandInfo = CommandInfo {
        name: "vkWaitSemaphores",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::TIMEOUT],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: Some(make_api_version(0, 1, 2, 0)),
        extensions: &[],
    };
    /// What the registry states about the C command [`vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html).
    pub const WAIT_SEMAPHORES_KHR: CommandInfo = CommandInfo {
        name: "vkWaitSemaphoresKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS, VkResult::TIMEOUT],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_DEVICE_LOST,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_timeline_semaphore"],
    };
    /// What the registry states about the C command [`vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html).
    pub const WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR: CommandInfo = CommandInfo {
        name: "vkWriteAccelerationStructuresPropertiesKHR",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_KHR_acceleration_structure"],
    };
    /// What the registry states about the C command [`vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html).
    pub const WRITE_MICROMAPS_PROPERTIES_EXT: CommandInfo = CommandInfo {
        name: "vkWriteMicromapsPropertiesEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_opacity_micromap"],
    };
    /// What the registry states about the C command [`vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html).
    pub const WRITE_RESOURCE_DESCRIPTORS_EXT: CommandInfo = CommandInfo {
        name: "vkWriteResourceDescriptorsEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
    /// What the registry states about the C command [`vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html).
    pub const WRITE_SAMPLER_DESCRIPTORS_EXT: CommandInfo = CommandInfo {
        name: "vkWriteSamplerDescriptorsEXT",
        level: CommandLevel::Device,
        success_codes: &[VkResult::SUCCESS],
        error_codes: &[
            VkResult::ERROR_OUT_OF_HOST_MEMORY,
            VkResult::ERROR_OUT_OF_DEVICE_MEMORY,
            VkResult::ERROR_UNKNOWN,
            VkResult::ERROR_VALIDATION_FAILED,
        ],
        version: None,
        extensions: &[c"VK_EXT_descriptor_heap"],
    };
}

/// Every command of the registry, sorted by C name as bytes. [`CommandInfo::get`] searches it.
pub const COMMANDS: &[CommandInfo] = &[
    command::ACQUIRE_DRM_DISPLAY_EXT,
    command::ACQUIRE_FULL_SCREEN_EXCLUSIVE_MODE_EXT,
    command::ACQUIRE_NEXT_IMAGE2_KHR,
    command::ACQUIRE_NEXT_IMAGE_KHR,
    command::ACQUIRE_PERFORMANCE_CONFIGURATION_INTEL,
    command::ACQUIRE_PROFILING_LOCK_KHR,
    command::ACQUIRE_WINRT_DISPLAY_NV,
    command::ACQUIRE_XLIB_DISPLAY_EXT,
    command::ALLOCATE_COMMAND_BUFFERS,
    command::ALLOCATE_DESCRIPTOR_SETS,
    command::ALLOCATE_MEMORY,
    command::ANTI_LAG_UPDATE_AMD,
    command::BEGIN_COMMAND_BUFFER,
    command::BIND_ACCELERATION_STRUCTURE_MEMORY_NV,
    command::BIND_BUFFER_MEMORY,
    command::BIND_BUFFER_MEMORY2,
    command::BIND_BUFFER_MEMORY2_KHR,
    command::BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_ARM,
    command::BIND_IMAGE_MEMORY,
    command::BIND_IMAGE_MEMORY2,
    command::BIND_IMAGE_MEMORY2_KHR,
    command::BIND_OPTICAL_FLOW_SESSION_IMAGE_NV,
    command::BIND_TENSOR_MEMORY_ARM,
    command::BIND_VIDEO_SESSION_MEMORY_KHR,
    command::BUILD_ACCELERATION_STRUCTURES_KHR,
    command::BUILD_MICROMAPS_EXT,
    command::CLEAR_SHADER_INSTRUMENTATION_METRICS_ARM,
    command::CMD_BEGIN_CONDITIONAL_RENDERING2_EXT,
    command::CMD_BEGIN_CONDITIONAL_RENDERING_EXT,
    command::CMD_BEGIN_CUSTOM_RESOLVE_EXT,
    command::CMD_BEGIN_DEBUG_UTILS_LABEL_EXT,
    command::CMD_BEGIN_GPA_SAMPLE_AMD,
    command::CMD_BEGIN_GPA_SESSION_AMD,
    command::CMD_BEGIN_PER_TILE_EXECUTION_QCOM,
    command::CMD_BEGIN_QUERY,
    command::CMD_BEGIN_QUERY_INDEXED_EXT,
    command::CMD_BEGIN_RENDER_PASS,
    command::CMD_BEGIN_RENDER_PASS2,
    command::CMD_BEGIN_RENDER_PASS2_KHR,
    command::CMD_BEGIN_RENDERING,
    command::CMD_BEGIN_RENDERING_KHR,
    command::CMD_BEGIN_SHADER_INSTRUMENTATION_ARM,
    command::CMD_BEGIN_TRANSFORM_FEEDBACK2_EXT,
    command::CMD_BEGIN_TRANSFORM_FEEDBACK_EXT,
    command::CMD_BEGIN_VIDEO_CODING_KHR,
    command::CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS2_EXT,
    command::CMD_BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_EXT,
    command::CMD_BIND_DESCRIPTOR_BUFFERS_EXT,
    command::CMD_BIND_DESCRIPTOR_SETS,
    command::CMD_BIND_DESCRIPTOR_SETS2,
    command::CMD_BIND_DESCRIPTOR_SETS2_KHR,
    command::CMD_BIND_INDEX_BUFFER,
    command::CMD_BIND_INDEX_BUFFER2,
    command::CMD_BIND_INDEX_BUFFER2_KHR,
    command::CMD_BIND_INDEX_BUFFER3_KHR,
    command::CMD_BIND_INVOCATION_MASK_HUAWEI,
    command::CMD_BIND_PIPELINE,
    command::CMD_BIND_PIPELINE_SHADER_GROUP_NV,
    command::CMD_BIND_RESOURCE_HEAP_EXT,
    command::CMD_BIND_SAMPLER_HEAP_EXT,
    command::CMD_BIND_SHADERS_EXT,
    command::CMD_BIND_SHADING_RATE_IMAGE_NV,
    command::CMD_BIND_TILE_MEMORY_QCOM,
    command::CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS2_EXT,
    command::CMD_BIND_TRANSFORM_FEEDBACK_BUFFERS_EXT,
    command::CMD_BIND_VERTEX_BUFFERS,
    command::CMD_BIND_VERTEX_BUFFERS2,
    command::CMD_BIND_VERTEX_BUFFERS2_EXT,
    command::CMD_BIND_VERTEX_BUFFERS3_KHR,
    command::CMD_BLIT_IMAGE,
    command::CMD_BLIT_IMAGE2,
    command::CMD_BLIT_IMAGE2_KHR,
    command::CMD_BUILD_ACCELERATION_STRUCTURE_NV,
    command::CMD_BUILD_ACCELERATION_STRUCTURES_INDIRECT_KHR,
    command::CMD_BUILD_ACCELERATION_STRUCTURES_KHR,
    command::CMD_BUILD_CLUSTER_ACCELERATION_STRUCTURE_INDIRECT_NV,
    command::CMD_BUILD_MICROMAPS_EXT,
    command::CMD_BUILD_PARTITIONED_ACCELERATION_STRUCTURES_NV,
    command::CMD_CLEAR_ATTACHMENTS,
    command::CMD_CLEAR_COLOR_IMAGE,
    command::CMD_CLEAR_DEPTH_STENCIL_IMAGE,
    command::CMD_CONTROL_VIDEO_CODING_KHR,
    command::CMD_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV,
    command::CMD_COPY_ACCELERATION_STRUCTURE_KHR,
    command::CMD_COPY_ACCELERATION_STRUCTURE_NV,
    command::CMD_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR,
    command::CMD_COPY_BUFFER,
    command::CMD_COPY_BUFFER2,
    command::CMD_COPY_BUFFER2_KHR,
    command::CMD_COPY_BUFFER_TO_IMAGE,
    command::CMD_COPY_BUFFER_TO_IMAGE2,
    command::CMD_COPY_BUFFER_TO_IMAGE2_KHR,
    command::CMD_COPY_GPA_SESSION_RESULTS_AMD,
    command::CMD_COPY_IMAGE,
    command::CMD_COPY_IMAGE2,
    command::CMD_COPY_IMAGE2_KHR,
    command::CMD_COPY_IMAGE_TO_BUFFER,
    command::CMD_COPY_IMAGE_TO_BUFFER2,
    command::CMD_COPY_IMAGE_TO_BUFFER2_KHR,
    command::CMD_COPY_IMAGE_TO_MEMORY_KHR,
    command::CMD_COPY_MEMORY_INDIRECT_KHR,
    command::CMD_COPY_MEMORY_INDIRECT_NV,
    command::CMD_COPY_MEMORY_KHR,
    command::CMD_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR,
    command::CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_KHR,
    command::CMD_COPY_MEMORY_TO_IMAGE_INDIRECT_NV,
    command::CMD_COPY_MEMORY_TO_IMAGE_KHR,
    command::CMD_COPY_MEMORY_TO_MICROMAP_EXT,
    command::CMD_COPY_MICROMAP_EXT,
    command::CMD_COPY_MICROMAP_TO_MEMORY_EXT,
    command::CMD_COPY_QUERY_POOL_RESULTS,
    command::CMD_COPY_QUERY_POOL_RESULTS_TO_MEMORY_KHR,
    command::CMD_COPY_TENSOR_ARM,
    command::CMD_CU_LAUNCH_KERNEL_NVX,
    command::CMD_CUDA_LAUNCH_KERNEL_NV,
    command::CMD_DEBUG_MARKER_BEGIN_EXT,
    command::CMD_DEBUG_MARKER_END_EXT,
    command::CMD_DEBUG_MARKER_INSERT_EXT,
    command::CMD_DECODE_VIDEO_KHR,
    command::CMD_DECOMPRESS_MEMORY_EXT,
    command::CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_EXT,
    command::CMD_DECOMPRESS_MEMORY_INDIRECT_COUNT_NV,
    command::CMD_DECOMPRESS_MEMORY_NV,
    command::CMD_DISPATCH,
    command::CMD_DISPATCH_BASE,
    command::CMD_DISPATCH_BASE_KHR,
    command::CMD_DISPATCH_DATA_GRAPH_ARM,
    command::CMD_DISPATCH_GRAPH_AMDX,
    command::CMD_DISPATCH_GRAPH_INDIRECT_AMDX,
    command::CMD_DISPATCH_GRAPH_INDIRECT_COUNT_AMDX,
    command::CMD_DISPATCH_INDIRECT,
    command::CMD_DISPATCH_INDIRECT2_KHR,
    command::CMD_DISPATCH_TILE_QCOM,
    command::CMD_DRAW,
    command::CMD_DRAW_CLUSTER_HUAWEI,
    command::CMD_DRAW_CLUSTER_INDIRECT_HUAWEI,
    command::CMD_DRAW_INDEXED,
    command::CMD_DRAW_INDEXED_INDIRECT,
    command::CMD_DRAW_INDEXED_INDIRECT2_KHR,
    command::CMD_DRAW_INDEXED_INDIRECT_COUNT,
    command::CMD_DRAW_INDEXED_INDIRECT_COUNT2_KHR,
    command::CMD_DRAW_INDEXED_INDIRECT_COUNT_AMD,
    command::CMD_DRAW_INDEXED_INDIRECT_COUNT_KHR,
    command::CMD_DRAW_INDIRECT,
    command::CMD_DRAW_INDIRECT2_KHR,
    command::CMD_DRAW_INDIRECT_BYTE_COUNT2_EXT,
    command::CMD_DRAW_INDIRECT_BYTE_COUNT_EXT,
    command::CMD_DRAW_INDIRECT_COUNT,
    command::CMD_DRAW_INDIRECT_COUNT2_KHR,
    command::CMD_DRAW_INDIRECT_COUNT_AMD,
    command::CMD_DRAW_INDIRECT_COUNT_KHR,
    command::CMD_DRAW_MESH_TASKS_EXT,
    command::CMD_DRAW_MESH_TASKS_INDIRECT2_EXT,
    command::CMD_DRAW_MESH_TASKS_INDIRECT_COUNT2_EXT,
    command::CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_EXT,
    command::CMD_DRAW_MESH_TASKS_INDIRECT_COUNT_NV,
    command::CMD_DRAW_MESH_TASKS_INDIRECT_EXT,
    command::CMD_DRAW_MESH_TASKS_INDIRECT_NV,
    command::CMD_DRAW_MESH_TASKS_NV,
    command::CMD_DRAW_MULTI_EXT,
    command::CMD_DRAW_MULTI_INDEXED_EXT,
    command::CMD_ENCODE_VIDEO_KHR,
    command::CMD_END_CONDITIONAL_RENDERING_EXT,
    command::CMD_END_DEBUG_UTILS_LABEL_EXT,
    command::CMD_END_GPA_SAMPLE_AMD,
    command::CMD_END_GPA_SESSION_AMD,
    command::CMD_END_PER_TILE_EXECUTION_QCOM,
    command::CMD_END_QUERY,
    command::CMD_END_QUERY_INDEXED_EXT,
    command::CMD_END_RENDER_PASS,
    command::CMD_END_RENDER_PASS2,
    command::CMD_END_RENDER_PASS2_KHR,
    command::CMD_END_RENDERING,
    command::CMD_END_RENDERING2_EXT,
    command::CMD_END_RENDERING2_KHR,
    command::CMD_END_RENDERING_KHR,
    command::CMD_END_SHADER_INSTRUMENTATION_ARM,
    command::CMD_END_TRANSFORM_FEEDBACK2_EXT,
    command::CMD_END_TRANSFORM_FEEDBACK_EXT,
    command::CMD_END_VIDEO_CODING_KHR,
    command::CMD_EXECUTE_COMMANDS,
    command::CMD_EXECUTE_GENERATED_COMMANDS_EXT,
    command::CMD_EXECUTE_GENERATED_COMMANDS_NV,
    command::CMD_FILL_BUFFER,
    command::CMD_FILL_MEMORY_KHR,
    command::CMD_INITIALIZE_GRAPH_SCRATCH_MEMORY_AMDX,
    command::CMD_INSERT_DEBUG_UTILS_LABEL_EXT,
    command::CMD_NEXT_SUBPASS,
    command::CMD_NEXT_SUBPASS2,
    command::CMD_NEXT_SUBPASS2_KHR,
    command::CMD_OPTICAL_FLOW_EXECUTE_NV,
    command::CMD_PIPELINE_BARRIER,
    command::CMD_PIPELINE_BARRIER2,
    command::CMD_PIPELINE_BARRIER2_KHR,
    command::CMD_PREPROCESS_GENERATED_COMMANDS_EXT,
    command::CMD_PREPROCESS_GENERATED_COMMANDS_NV,
    command::CMD_PUSH_CONSTANTS,
    command::CMD_PUSH_CONSTANTS2,
    command::CMD_PUSH_CONSTANTS2_KHR,
    command::CMD_PUSH_DATA_EXT,
    command::CMD_PUSH_DESCRIPTOR_SET,
    command::CMD_PUSH_DESCRIPTOR_SET2,
    command::CMD_PUSH_DESCRIPTOR_SET2_KHR,
    command::CMD_PUSH_DESCRIPTOR_SET_KHR,
    command::CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE,
    command::CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2,
    command::CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE2_KHR,
    command::CMD_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_KHR,
    command::CMD_RESET_EVENT,
    command::CMD_RESET_EVENT2,
    command::CMD_RESET_EVENT2_KHR,
    command::CMD_RESET_QUERY_POOL,
    command::CMD_RESOLVE_IMAGE,
    command::CMD_RESOLVE_IMAGE2,
    command::CMD_RESOLVE_IMAGE2_KHR,
    command::CMD_SET_ALPHA_TO_COVERAGE_ENABLE_EXT,
    command::CMD_SET_ALPHA_TO_ONE_ENABLE_EXT,
    command::CMD_SET_ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT,
    command::CMD_SET_BLEND_CONSTANTS,
    command::CMD_SET_CHECKPOINT_NV,
    command::CMD_SET_COARSE_SAMPLE_ORDER_NV,
    command::CMD_SET_COLOR_BLEND_ADVANCED_EXT,
    command::CMD_SET_COLOR_BLEND_ENABLE_EXT,
    command::CMD_SET_COLOR_BLEND_EQUATION_EXT,
    command::CMD_SET_COLOR_WRITE_ENABLE_EXT,
    command::CMD_SET_COLOR_WRITE_MASK_EXT,
    command::CMD_SET_COMPUTE_OCCUPANCY_PRIORITY_NV,
    command::CMD_SET_CONSERVATIVE_RASTERIZATION_MODE_EXT,
    command::CMD_SET_COVERAGE_MODULATION_MODE_NV,
    command::CMD_SET_COVERAGE_MODULATION_TABLE_ENABLE_NV,
    command::CMD_SET_COVERAGE_MODULATION_TABLE_NV,
    command::CMD_SET_COVERAGE_REDUCTION_MODE_NV,
    command::CMD_SET_COVERAGE_TO_COLOR_ENABLE_NV,
    command::CMD_SET_COVERAGE_TO_COLOR_LOCATION_NV,
    command::CMD_SET_CULL_MODE,
    command::CMD_SET_CULL_MODE_EXT,
    command::CMD_SET_DEPTH_BIAS,
    command::CMD_SET_DEPTH_BIAS2_EXT,
    command::CMD_SET_DEPTH_BIAS_ENABLE,
    command::CMD_SET_DEPTH_BIAS_ENABLE_EXT,
    command::CMD_SET_DEPTH_BOUNDS,
    command::CMD_SET_DEPTH_BOUNDS_TEST_ENABLE,
    command::CMD_SET_DEPTH_BOUNDS_TEST_ENABLE_EXT,
    command::CMD_SET_DEPTH_CLAMP_ENABLE_EXT,
    command::CMD_SET_DEPTH_CLAMP_RANGE_EXT,
    command::CMD_SET_DEPTH_CLIP_ENABLE_EXT,
    command::CMD_SET_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT,
    command::CMD_SET_DEPTH_COMPARE_OP,
    command::CMD_SET_DEPTH_COMPARE_OP_EXT,
    command::CMD_SET_DEPTH_TEST_ENABLE,
    command::CMD_SET_DEPTH_TEST_ENABLE_EXT,
    command::CMD_SET_DEPTH_WRITE_ENABLE,
    command::CMD_SET_DEPTH_WRITE_ENABLE_EXT,
    command::CMD_SET_DESCRIPTOR_BUFFER_OFFSETS2_EXT,
    command::CMD_SET_DESCRIPTOR_BUFFER_OFFSETS_EXT,
    command::CMD_SET_DEVICE_MASK,
    command::CMD_SET_DEVICE_MASK_KHR,
    command::CMD_SET_DISCARD_RECTANGLE_EXT,
    command::CMD_SET_DISCARD_RECTANGLE_ENABLE_EXT,
    command::CMD_SET_DISCARD_RECTANGLE_MODE_EXT,
    command::CMD_SET_DISPATCH_PARAMETERS_ARM,
    command::CMD_SET_EVENT,
    command::CMD_SET_EVENT2,
    command::CMD_SET_EVENT2_KHR,
    command::CMD_SET_EXCLUSIVE_SCISSOR_ENABLE_NV,
    command::CMD_SET_EXCLUSIVE_SCISSOR_NV,
    command::CMD_SET_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT,
    command::CMD_SET_FRAGMENT_SHADING_RATE_ENUM_NV,
    command::CMD_SET_FRAGMENT_SHADING_RATE_KHR,
    command::CMD_SET_FRONT_FACE,
    command::CMD_SET_FRONT_FACE_EXT,
    command::CMD_SET_LINE_RASTERIZATION_MODE_EXT,
    command::CMD_SET_LINE_STIPPLE,
    command::CMD_SET_LINE_STIPPLE_EXT,
    command::CMD_SET_LINE_STIPPLE_ENABLE_EXT,
    command::CMD_SET_LINE_STIPPLE_KHR,
    command::CMD_SET_LINE_WIDTH,
    command::CMD_SET_LOGIC_OP_EXT,
    command::CMD_SET_LOGIC_OP_ENABLE_EXT,
    command::CMD_SET_PATCH_CONTROL_POINTS_EXT,
    command::CMD_SET_PERFORMANCE_MARKER_INTEL,
    command::CMD_SET_PERFORMANCE_OVERRIDE_INTEL,
    command::CMD_SET_PERFORMANCE_STREAM_MARKER_INTEL,
    command::CMD_SET_POLYGON_MODE_EXT,
    command::CMD_SET_PRIMITIVE_RESTART_ENABLE,
    command::CMD_SET_PRIMITIVE_RESTART_ENABLE_EXT,
    command::CMD_SET_PRIMITIVE_RESTART_INDEX_EXT,
    command::CMD_SET_PRIMITIVE_TOPOLOGY,
    command::CMD_SET_PRIMITIVE_TOPOLOGY_EXT,
    command::CMD_SET_PROVOKING_VERTEX_MODE_EXT,
    command::CMD_SET_RASTERIZATION_SAMPLES_EXT,
    command::CMD_SET_RASTERIZATION_STREAM_EXT,
    command::CMD_SET_RASTERIZER_DISCARD_ENABLE,
    command::CMD_SET_RASTERIZER_DISCARD_ENABLE_EXT,
    command::CMD_SET_RAY_TRACING_PIPELINE_STACK_SIZE_KHR,
    command::CMD_SET_RENDERING_ATTACHMENT_LOCATIONS,
    command::CMD_SET_RENDERING_ATTACHMENT_LOCATIONS_KHR,
    command::CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES,
    command::CMD_SET_RENDERING_INPUT_ATTACHMENT_INDICES_KHR,
    command::CMD_SET_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV,
    command::CMD_SET_SAMPLE_LOCATIONS_EXT,
    command::CMD_SET_SAMPLE_LOCATIONS_ENABLE_EXT,
    command::CMD_SET_SAMPLE_MASK_EXT,
    command::CMD_SET_SCISSOR,
    command::CMD_SET_SCISSOR_WITH_COUNT,
    command::CMD_SET_SCISSOR_WITH_COUNT_EXT,
    command::CMD_SET_SHADING_RATE_IMAGE_ENABLE_NV,
    command::CMD_SET_STENCIL_COMPARE_MASK,
    command::CMD_SET_STENCIL_OP,
    command::CMD_SET_STENCIL_OP_EXT,
    command::CMD_SET_STENCIL_REFERENCE,
    command::CMD_SET_STENCIL_TEST_ENABLE,
    command::CMD_SET_STENCIL_TEST_ENABLE_EXT,
    command::CMD_SET_STENCIL_WRITE_MASK,
    command::CMD_SET_TESSELLATION_DOMAIN_ORIGIN_EXT,
    command::CMD_SET_VERTEX_INPUT_EXT,
    command::CMD_SET_VIEWPORT,
    command::CMD_SET_VIEWPORT_SHADING_RATE_PALETTE_NV,
    command::CMD_SET_VIEWPORT_SWIZZLE_NV,
    command::CMD_SET_VIEWPORT_W_SCALING_ENABLE_NV,
    command::CMD_SET_VIEWPORT_W_SCALING_NV,
    command::CMD_SET_VIEWPORT_WITH_COUNT,
    command::CMD_SET_VIEWPORT_WITH_COUNT_EXT,
    command::CMD_SUBPASS_SHADING_HUAWEI,
    command::CMD_TRACE_RAYS_INDIRECT2_KHR,
    command::CMD_TRACE_RAYS_INDIRECT_KHR,
    command::CMD_TRACE_RAYS_KHR,
    command::CMD_TRACE_RAYS_NV,
    command::CMD_UPDATE_BUFFER,
    command::CMD_UPDATE_MEMORY_KHR,
    command::CMD_UPDATE_PIPELINE_INDIRECT_BUFFER_NV,
    command::CMD_WAIT_EVENTS,
    command::CMD_WAIT_EVENTS2,
    command::CMD_WAIT_EVENTS2_KHR,
    command::CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR,
    command::CMD_WRITE_ACCELERATION_STRUCTURES_PROPERTIES_NV,
    command::CMD_WRITE_BUFFER_MARKER2_AMD,
    command::CMD_WRITE_BUFFER_MARKER_AMD,
    command::CMD_WRITE_MARKER_TO_MEMORY_AMD,
    command::CMD_WRITE_MICROMAPS_PROPERTIES_EXT,
    command::CMD_WRITE_TIMESTAMP,
    command::CMD_WRITE_TIMESTAMP2,
    command::CMD_WRITE_TIMESTAMP2_KHR,
    command::COMPILE_DEFERRED_NV,
    command::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV,
    command::COPY_ACCELERATION_STRUCTURE_KHR,
    command::COPY_ACCELERATION_STRUCTURE_TO_MEMORY_KHR,
    command::COPY_IMAGE_TO_IMAGE,
    command::COPY_IMAGE_TO_IMAGE_EXT,
    command::COPY_IMAGE_TO_MEMORY,
    command::COPY_IMAGE_TO_MEMORY_EXT,
    command::COPY_MEMORY_TO_ACCELERATION_STRUCTURE_KHR,
    command::COPY_MEMORY_TO_IMAGE,
    command::COPY_MEMORY_TO_IMAGE_EXT,
    command::COPY_MEMORY_TO_MICROMAP_EXT,
    command::COPY_MICROMAP_EXT,
    command::COPY_MICROMAP_TO_MEMORY_EXT,
    command::CREATE_ACCELERATION_STRUCTURE2_KHR,
    command::CREATE_ACCELERATION_STRUCTURE_KHR,
    command::CREATE_ACCELERATION_STRUCTURE_NV,
    command::CREATE_ANDROID_SURFACE_KHR,
    command::CREATE_BUFFER,
    command::CREATE_BUFFER_COLLECTION_FUCHSIA,
    command::CREATE_BUFFER_VIEW,
    command::CREATE_COMMAND_POOL,
    command::CREATE_COMPUTE_PIPELINES,
    command::CREATE_CU_FUNCTION_NVX,
    command::CREATE_CU_MODULE_NVX,
    command::CREATE_CUDA_FUNCTION_NV,
    command::CREATE_CUDA_MODULE_NV,
    command::CREATE_DATA_GRAPH_PIPELINE_SESSION_ARM,
    command::CREATE_DATA_GRAPH_PIPELINES_ARM,
    command::CREATE_DEBUG_REPORT_CALLBACK_EXT,
    command::CREATE_DEBUG_UTILS_MESSENGER_EXT,
    command::CREATE_DEFERRED_OPERATION_KHR,
    command::CREATE_DESCRIPTOR_POOL,
    command::CREATE_DESCRIPTOR_SET_LAYOUT,
    command::CREATE_DESCRIPTOR_UPDATE_TEMPLATE,
    command::CREATE_DESCRIPTOR_UPDATE_TEMPLATE_KHR,
    command::CREATE_DEVICE,
    command::CREATE_DIRECT_FB_SURFACE_EXT,
    command::CREATE_DISPLAY_MODE_KHR,
    command::CREATE_DISPLAY_PLANE_SURFACE_KHR,
    command::CREATE_EVENT,
    command::CREATE_EXECUTION_GRAPH_PIPELINES_AMDX,
    command::CREATE_EXTERNAL_COMPUTE_QUEUE_NV,
    command::CREATE_FENCE,
    command::CREATE_FRAMEBUFFER,
    command::CREATE_GPA_SESSION_AMD,
    command::CREATE_GRAPHICS_PIPELINES,
    command::CREATE_HEADLESS_SURFACE_EXT,
    command::CREATE_IOS_SURFACE_MVK,
    command::CREATE_IMAGE,
    command::CREATE_IMAGE_PIPE_SURFACE_FUCHSIA,
    command::CREATE_IMAGE_VIEW,
    command::CREATE_INDIRECT_COMMANDS_LAYOUT_EXT,
    command::CREATE_INDIRECT_COMMANDS_LAYOUT_NV,
    command::CREATE_INDIRECT_EXECUTION_SET_EXT,
    command::CREATE_INSTANCE,
    command::CREATE_MAC_OS_SURFACE_MVK,
    command::CREATE_METAL_SURFACE_EXT,
    command::CREATE_MICROMAP_EXT,
    command::CREATE_OPTICAL_FLOW_SESSION_NV,
    command::CREATE_PIPELINE_BINARIES_KHR,
    command::CREATE_PIPELINE_CACHE,
    command::CREATE_PIPELINE_LAYOUT,
    command::CREATE_PRIVATE_DATA_SLOT,
    command::CREATE_PRIVATE_DATA_SLOT_EXT,
    command::CREATE_QUERY_POOL,
    command::CREATE_RAY_TRACING_PIPELINES_KHR,
    command::CREATE_RAY_TRACING_PIPELINES_NV,
    command::CREATE_RENDER_PASS,
    command::CREATE_RENDER_PASS2,
    command::CREATE_RENDER_PASS2_KHR,
    command::CREATE_SAMPLER,
    command::CREATE_SAMPLER_YCBCR_CONVERSION,
    command::CREATE_SAMPLER_YCBCR_CONVERSION_KHR,
    command::CREATE_SCREEN_SURFACE_QNX,
    command::CREATE_SEMAPHORE,
    command::CREATE_SHADER_INSTRUMENTATION_ARM,
    command::CREATE_SHADER_MODULE,
    command::CREATE_SHADERS_EXT,
    command::CREATE_SHARED_SWAPCHAINS_KHR,
    command::CREATE_STREAM_DESCRIPTOR_SURFACE_GGP,
    command::CREATE_SURFACE_OHOS,
    command::CREATE_SWAPCHAIN_KHR,
    command::CREATE_TENSOR_ARM,
    command::CREATE_TENSOR_VIEW_ARM,
    command::CREATE_UBM_SURFACE_SEC,
    command::CREATE_VALIDATION_CACHE_EXT,
    command::CREATE_VI_SURFACE_NN,
    command::CREATE_VIDEO_SESSION_KHR,
    command::CREATE_VIDEO_SESSION_PARAMETERS_KHR,
    command::CREATE_WAYLAND_SURFACE_KHR,
    command::CREATE_WIN32_SURFACE_KHR,
    command::CREATE_XCB_SURFACE_KHR,
    command::CREATE_XLIB_SURFACE_KHR,
    command::DEBUG_MARKER_SET_OBJECT_NAME_EXT,
    command::DEBUG_MARKER_SET_OBJECT_TAG_EXT,
    command::DEBUG_REPORT_MESSAGE_EXT,
    command::DEFERRED_OPERATION_JOIN_KHR,
    command::DESTROY_ACCELERATION_STRUCTURE_KHR,
    command::DESTROY_ACCELERATION_STRUCTURE_NV,
    command::DESTROY_BUFFER,
    command::DESTROY_BUFFER_COLLECTION_FUCHSIA,
    command::DESTROY_BUFFER_VIEW,
    command::DESTROY_COMMAND_POOL,
    command::DESTROY_CU_FUNCTION_NVX,
    command::DESTROY_CU_MODULE_NVX,
    command::DESTROY_CUDA_FUNCTION_NV,
    command::DESTROY_CUDA_MODULE_NV,
    command::DESTROY_DATA_GRAPH_PIPELINE_SESSION_ARM,
    command::DESTROY_DEBUG_REPORT_CALLBACK_EXT,
    command::DESTROY_DEBUG_UTILS_MESSENGER_EXT,
    command::DESTROY_DEFERRED_OPERATION_KHR,
    command::DESTROY_DESCRIPTOR_POOL,
    command::DESTROY_DESCRIPTOR_SET_LAYOUT,
    command::DESTROY_DESCRIPTOR_UPDATE_TEMPLATE,
    command::DESTROY_DESCRIPTOR_UPDATE_TEMPLATE_KHR,
    command::DESTROY_DEVICE,
    command::DESTROY_EVENT,
    command::DESTROY_EXTERNAL_COMPUTE_QUEUE_NV,
    command::DESTROY_FENCE,
    command::DESTROY_FRAMEBUFFER,
    command::DESTROY_GPA_SESSION_AMD,
    command::DESTROY_IMAGE,
    command::DESTROY_IMAGE_VIEW,
    command::DESTROY_INDIRECT_COMMANDS_LAYOUT_EXT,
    command::DESTROY_INDIRECT_COMMANDS_LAYOUT_NV,
    command::DESTROY_INDIRECT_EXECUTION_SET_EXT,
    command::DESTROY_INSTANCE,
    command::DESTROY_MICROMAP_EXT,
    command::DESTROY_OPTICAL_FLOW_SESSION_NV,
    command::DESTROY_PIPELINE,
    command::DESTROY_PIPELINE_BINARY_KHR,
    command::DESTROY_PIPELINE_CACHE,
    command::DESTROY_PIPELINE_LAYOUT,
    command::DESTROY_PRIVATE_DATA_SLOT,
    command::DESTROY_PRIVATE_DATA_SLOT_EXT,
    command::DESTROY_QUERY_POOL,
    command::DESTROY_RENDER_PASS,
    command::DESTROY_SAMPLER,
    command::DESTROY_SAMPLER_YCBCR_CONVERSION,
    command::DESTROY_SAMPLER_YCBCR_CONVERSION_KHR,
    command::DESTROY_SEMAPHORE,
    command::DESTROY_SHADER_EXT,
    command::DESTROY_SHADER_INSTRUMENTATION_ARM,
    command::DESTROY_SHADER_MODULE,
    command::DESTROY_SURFACE_KHR,
    command::DESTROY_SWAPCHAIN_KHR,
    command::DESTROY_TENSOR_ARM,
    command::DESTROY_TENSOR_VIEW_ARM,
    command::DESTROY_VALIDATION_CACHE_EXT,
    command::DESTROY_VIDEO_SESSION_KHR,
    command::DESTROY_VIDEO_SESSION_PARAMETERS_KHR,
    command::DEVICE_WAIT_IDLE,
    command::DISPLAY_POWER_CONTROL_EXT,
    command::END_COMMAND_BUFFER,
    command::ENUMERATE_DEVICE_EXTENSION_PROPERTIES,
    command::ENUMERATE_DEVICE_LAYER_PROPERTIES,
    command::ENUMERATE_INSTANCE_EXTENSION_PROPERTIES,
    command::ENUMERATE_INSTANCE_LAYER_PROPERTIES,
    command::ENUMERATE_INSTANCE_VERSION,
    command::ENUMERATE_PHYSICAL_DEVICE_GROUPS,
    command::ENUMERATE_PHYSICAL_DEVICE_GROUPS_KHR,
    command::ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_COUNTERS_BY_REGION_ARM,
    command::ENUMERATE_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_COUNTERS_KHR,
    command::ENUMERATE_PHYSICAL_DEVICE_SHADER_INSTRUMENTATION_METRICS_ARM,
    command::ENUMERATE_PHYSICAL_DEVICES,
    command::EXPORT_METAL_OBJECTS_EXT,
    command::FLUSH_MAPPED_MEMORY_RANGES,
    command::FREE_COMMAND_BUFFERS,
    command::FREE_DESCRIPTOR_SETS,
    command::FREE_MEMORY,
    command::GET_ACCELERATION_STRUCTURE_BUILD_SIZES_KHR,
    command::GET_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_KHR,
    command::GET_ACCELERATION_STRUCTURE_HANDLE_NV,
    command::GET_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_NV,
    command::GET_ACCELERATION_STRUCTURE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT,
    command::GET_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID,
    command::GET_BUFFER_COLLECTION_PROPERTIES_FUCHSIA,
    command::GET_BUFFER_DEVICE_ADDRESS,
    command::GET_BUFFER_DEVICE_ADDRESS_EXT,
    command::GET_BUFFER_DEVICE_ADDRESS_KHR,
    command::GET_BUFFER_MEMORY_REQUIREMENTS,
    command::GET_BUFFER_MEMORY_REQUIREMENTS2,
    command::GET_BUFFER_MEMORY_REQUIREMENTS2_KHR,
    command::GET_BUFFER_OPAQUE_CAPTURE_ADDRESS,
    command::GET_BUFFER_OPAQUE_CAPTURE_ADDRESS_KHR,
    command::GET_BUFFER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT,
    command::GET_CALIBRATED_TIMESTAMPS_EXT,
    command::GET_CALIBRATED_TIMESTAMPS_KHR,
    command::GET_CLUSTER_ACCELERATION_STRUCTURE_BUILD_SIZES_NV,
    command::GET_CUDA_MODULE_CACHE_NV,
    command::GET_DATA_GRAPH_PIPELINE_AVAILABLE_PROPERTIES_ARM,
    command::GET_DATA_GRAPH_PIPELINE_PROPERTIES_ARM,
    command::GET_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_ARM,
    command::GET_DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_ARM,
    command::GET_DEFERRED_OPERATION_MAX_CONCURRENCY_KHR,
    command::GET_DEFERRED_OPERATION_RESULT_KHR,
    command::GET_DESCRIPTOR_EXT,
    command::GET_DESCRIPTOR_SET_HOST_MAPPING_VALVE,
    command::GET_DESCRIPTOR_SET_LAYOUT_BINDING_OFFSET_EXT,
    command::GET_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE,
    command::GET_DESCRIPTOR_SET_LAYOUT_SIZE_EXT,
    command::GET_DESCRIPTOR_SET_LAYOUT_SUPPORT,
    command::GET_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR,
    command::GET_DEVICE_ACCELERATION_STRUCTURE_COMPATIBILITY_KHR,
    command::GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS,
    command::GET_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR,
    command::GET_DEVICE_COMBINED_IMAGE_SAMPLER_INDEX_NVX,
    command::GET_DEVICE_FAULT_DEBUG_INFO_KHR,
    command::GET_DEVICE_FAULT_INFO_EXT,
    command::GET_DEVICE_FAULT_REPORTS_KHR,
    command::GET_DEVICE_GROUP_PEER_MEMORY_FEATURES,
    command::GET_DEVICE_GROUP_PEER_MEMORY_FEATURES_KHR,
    command::GET_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR,
    command::GET_DEVICE_GROUP_SURFACE_PRESENT_MODES2_EXT,
    command::GET_DEVICE_GROUP_SURFACE_PRESENT_MODES_KHR,
    command::GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS,
    command::GET_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR,
    command::GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS,
    command::GET_DEVICE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_KHR,
    command::GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT,
    command::GET_DEVICE_IMAGE_SUBRESOURCE_LAYOUT_KHR,
    command::GET_DEVICE_MEMORY_COMMITMENT,
    command::GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS,
    command::GET_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_KHR,
    command::GET_DEVICE_MICROMAP_COMPATIBILITY_EXT,
    command::GET_DEVICE_PROC_ADDR,
    command::GET_DEVICE_QUEUE,
    command::GET_DEVICE_QUEUE2,
    command::GET_DEVICE_SUBPASS_SHADING_MAX_WORKGROUP_SIZE_HUAWEI,
    command::GET_DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM,
    command::GET_DISPLAY_MODE_PROPERTIES2_KHR,
    command::GET_DISPLAY_MODE_PROPERTIES_KHR,
    command::GET_DISPLAY_PLANE_CAPABILITIES2_KHR,
    command::GET_DISPLAY_PLANE_CAPABILITIES_KHR,
    command::GET_DISPLAY_PLANE_SUPPORTED_DISPLAYS_KHR,
    command::GET_DRM_DISPLAY_EXT,
    command::GET_DYNAMIC_RENDERING_TILE_PROPERTIES_QCOM,
    command::GET_ENCODED_VIDEO_SESSION_PARAMETERS_KHR,
    command::GET_EVENT_STATUS,
    command::GET_EXECUTION_GRAPH_PIPELINE_NODE_INDEX_AMDX,
    command::GET_EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX,
    command::GET_EXTERNAL_COMPUTE_QUEUE_DATA_NV,
    command::GET_FENCE_FD_KHR,
    command::GET_FENCE_STATUS,
    command::GET_FENCE_WIN32_HANDLE_KHR,
    command::GET_FRAMEBUFFER_TILE_PROPERTIES_QCOM,
    command::GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_EXT,
    command::GET_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_NV,
    command::GET_GPA_DEVICE_CLOCK_INFO_AMD,
    command::GET_GPA_SESSION_RESULTS_AMD,
    command::GET_GPA_SESSION_STATUS_AMD,
    command::GET_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT,
    command::GET_IMAGE_MEMORY_REQUIREMENTS,
    command::GET_IMAGE_MEMORY_REQUIREMENTS2,
    command::GET_IMAGE_MEMORY_REQUIREMENTS2_KHR,
    command::GET_IMAGE_OPAQUE_CAPTURE_DATA_EXT,
    command::GET_IMAGE_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT,
    command::GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS,
    command::GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2,
    command::GET_IMAGE_SPARSE_MEMORY_REQUIREMENTS2_KHR,
    command::GET_IMAGE_SUBRESOURCE_LAYOUT,
    command::GET_IMAGE_SUBRESOURCE_LAYOUT2,
    command::GET_IMAGE_SUBRESOURCE_LAYOUT2_EXT,
    command::GET_IMAGE_SUBRESOURCE_LAYOUT2_KHR,
    command::GET_IMAGE_VIEW_ADDRESS_NVX,
    command::GET_IMAGE_VIEW_HANDLE64_NVX,
    command::GET_IMAGE_VIEW_HANDLE_NVX,
    command::GET_IMAGE_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT,
    command::GET_INSTANCE_PROC_ADDR,
    command::GET_LATENCY_TIMINGS_LEGACY_NV,
    command::GET_LATENCY_TIMINGS_NV,
    command::GET_MEMORY_ANDROID_HARDWARE_BUFFER_ANDROID,
    command::GET_MEMORY_FD_KHR,
    command::GET_MEMORY_FD_PROPERTIES_KHR,
    command::GET_MEMORY_HOST_POINTER_PROPERTIES_EXT,
    command::GET_MEMORY_METAL_HANDLE_EXT,
    command::GET_MEMORY_METAL_HANDLE_PROPERTIES_EXT,
    command::GET_MEMORY_NATIVE_BUFFER_OHOS,
    command::GET_MEMORY_REMOTE_ADDRESS_NV,
    command::GET_MEMORY_WIN32_HANDLE_KHR,
    command::GET_MEMORY_WIN32_HANDLE_NV,
    command::GET_MEMORY_WIN32_HANDLE_PROPERTIES_KHR,
    command::GET_MEMORY_ZIRCON_HANDLE_FUCHSIA,
    command::GET_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA,
    command::GET_MICROMAP_BUILD_SIZES_EXT,
    command::GET_NATIVE_BUFFER_PROPERTIES_OHOS,
    command::GET_PARTITIONED_ACCELERATION_STRUCTURES_BUILD_SIZES_NV,
    command::GET_PAST_PRESENTATION_TIMING_EXT,
    command::GET_PAST_PRESENTATION_TIMING_GOOGLE,
    command::GET_PERFORMANCE_PARAMETER_INTEL,
    command::GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_EXT,
    command::GET_PHYSICAL_DEVICE_CALIBRATEABLE_TIME_DOMAINS_KHR,
    command::GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV,
    command::GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES2_EXT,
    command::GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV,
    command::GET_PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV,
    command::GET_PHYSICAL_DEVICE_DESCRIPTOR_SIZE_EXT,
    command::GET_PHYSICAL_DEVICE_DIRECT_FB_PRESENTATION_SUPPORT_EXT,
    command::GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_DISPLAY_PLANE_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_DISPLAY_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_BUFFER_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_FENCE_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_NV,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_EXTERNAL_TENSOR_PROPERTIES_ARM,
    command::GET_PHYSICAL_DEVICE_FEATURES,
    command::GET_PHYSICAL_DEVICE_FEATURES2,
    command::GET_PHYSICAL_DEVICE_FEATURES2_KHR,
    command::GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_FORMAT_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATES_KHR,
    command::GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_IMAGE_FORMAT_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_MEMORY_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_MULTISAMPLE_PROPERTIES_EXT,
    command::GET_PHYSICAL_DEVICE_OPTICAL_FLOW_IMAGE_FORMATS_NV,
    command::GET_PHYSICAL_DEVICE_PRESENT_RECTANGLES_KHR,
    command::GET_PHYSICAL_DEVICE_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_ENGINE_OPERATION_PROPERTIES_ARM,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_OPTICAL_FLOW_IMAGE_FORMATS_ARM,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PERFORMANCE_QUERY_PASSES_KHR,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_QUEUE_FAMILY_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_SCREEN_PRESENTATION_SUPPORT_QNX,
    command::GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2,
    command::GET_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_PROPERTIES2_KHR,
    command::GET_PHYSICAL_DEVICE_SUPPORTED_FRAMEBUFFER_MIXED_SAMPLES_COMBINATIONS_NV,
    command::GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_EXT,
    command::GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES2_KHR,
    command::GET_PHYSICAL_DEVICE_SURFACE_CAPABILITIES_KHR,
    command::GET_PHYSICAL_DEVICE_SURFACE_FORMATS2_KHR,
    command::GET_PHYSICAL_DEVICE_SURFACE_FORMATS_KHR,
    command::GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES2_EXT,
    command::GET_PHYSICAL_DEVICE_SURFACE_PRESENT_MODES_KHR,
    command::GET_PHYSICAL_DEVICE_SURFACE_SUPPORT_KHR,
    command::GET_PHYSICAL_DEVICE_TOOL_PROPERTIES,
    command::GET_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT,
    command::GET_PHYSICAL_DEVICE_UBM_PRESENTATION_SUPPORT_SEC,
    command::GET_PHYSICAL_DEVICE_VIDEO_CAPABILITIES_KHR,
    command::GET_PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_VIDEO_FORMAT_PROPERTIES_KHR,
    command::GET_PHYSICAL_DEVICE_WAYLAND_PRESENTATION_SUPPORT_KHR,
    command::GET_PHYSICAL_DEVICE_WIN32_PRESENTATION_SUPPORT_KHR,
    command::GET_PHYSICAL_DEVICE_XCB_PRESENTATION_SUPPORT_KHR,
    command::GET_PHYSICAL_DEVICE_XLIB_PRESENTATION_SUPPORT_KHR,
    command::GET_PIPELINE_BINARY_DATA_KHR,
    command::GET_PIPELINE_CACHE_DATA,
    command::GET_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATIONS_KHR,
    command::GET_PIPELINE_EXECUTABLE_PROPERTIES_KHR,
    command::GET_PIPELINE_EXECUTABLE_STATISTICS_KHR,
    command::GET_PIPELINE_INDIRECT_DEVICE_ADDRESS_NV,
    command::GET_PIPELINE_INDIRECT_MEMORY_REQUIREMENTS_NV,
    command::GET_PIPELINE_KEY_KHR,
    command::GET_PIPELINE_PROPERTIES_EXT,
    command::GET_PRIVATE_DATA,
    command::GET_PRIVATE_DATA_EXT,
    command::GET_QUERY_POOL_RESULTS,
    command::GET_QUEUE_CHECKPOINT_DATA2_NV,
    command::GET_QUEUE_CHECKPOINT_DATA_NV,
    command::GET_RAND_R_OUTPUT_DISPLAY_EXT,
    command::GET_RAY_TRACING_CAPTURE_REPLAY_SHADER_GROUP_HANDLES_KHR,
    command::GET_RAY_TRACING_SHADER_GROUP_HANDLES_KHR,
    command::GET_RAY_TRACING_SHADER_GROUP_HANDLES_NV,
    command::GET_RAY_TRACING_SHADER_GROUP_STACK_SIZE_KHR,
    command::GET_REFRESH_CYCLE_DURATION_GOOGLE,
    command::GET_RENDER_AREA_GRANULARITY,
    command::GET_RENDERING_AREA_GRANULARITY,
    command::GET_RENDERING_AREA_GRANULARITY_KHR,
    command::GET_SAMPLER_OPAQUE_CAPTURE_DESCRIPTOR_DATA_EXT,
    command::GET_SCREEN_BUFFER_PROPERTIES_QNX,
    command::GET_SEMAPHORE_COUNTER_VALUE,
    command::GET_SEMAPHORE_COUNTER_VALUE_KHR,
    command::GET_SEMAPHORE_FD_KHR,
    command::GET_SEMAPHORE_WIN32_HANDLE_KHR,
    command::GET_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA,
    command::GET_SHADER_BINARY_DATA_EXT,
    command::GET_SHADER_INFO_AMD,
    command::GET_SHADER_INSTRUMENTATION_VALUES_ARM,
    command::GET_SHADER_MODULE_CREATE_INFO_IDENTIFIER_EXT,
    command::GET_SHADER_MODULE_IDENTIFIER_EXT,
    command::GET_SLEEP_STATUS_LEGACY_NV,
    command::GET_SWAPCHAIN_COUNTER_EXT,
    command::GET_SWAPCHAIN_IMAGES_KHR,
    command::GET_SWAPCHAIN_STATUS_KHR,
    command::GET_SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT,
    command::GET_SWAPCHAIN_TIMING_PROPERTIES_EXT,
    command::GET_TENSOR_MEMORY_REQUIREMENTS_ARM,
    command::GET_TENSOR_OPAQUE_CAPTURE_DATA_ARM,
    command::GET_TENSOR_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM,
    command::GET_TENSOR_VIEW_OPAQUE_CAPTURE_DESCRIPTOR_DATA_ARM,
    command::GET_VALIDATION_CACHE_DATA_EXT,
    command::GET_VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
    command::GET_WINRT_DISPLAY_NV,
    command::IMPORT_FENCE_FD_KHR,
    command::IMPORT_FENCE_WIN32_HANDLE_KHR,
    command::IMPORT_SEMAPHORE_FD_KHR,
    command::IMPORT_SEMAPHORE_WIN32_HANDLE_KHR,
    command::IMPORT_SEMAPHORE_ZIRCON_HANDLE_FUCHSIA,
    command::INITIALIZE_PERFORMANCE_API_INTEL,
    command::INVALIDATE_MAPPED_MEMORY_RANGES,
    command::LATENCY_SLEEP_LEGACY_NV,
    command::LATENCY_SLEEP_NV,
    command::MAP_MEMORY,
    command::MAP_MEMORY2,
    command::MAP_MEMORY2_KHR,
    command::MERGE_PIPELINE_CACHES,
    command::MERGE_VALIDATION_CACHES_EXT,
    command::QUEUE_BEGIN_DEBUG_UTILS_LABEL_EXT,
    command::QUEUE_BIND_SPARSE,
    command::QUEUE_END_DEBUG_UTILS_LABEL_EXT,
    command::QUEUE_INSERT_DEBUG_UTILS_LABEL_EXT,
    command::QUEUE_NOTIFY_OUT_OF_BAND_LEGACY_NV,
    command::QUEUE_NOTIFY_OUT_OF_BAND_NV,
    command::QUEUE_PRESENT_KHR,
    command::QUEUE_SET_PERF_HINT_QCOM,
    command::QUEUE_SET_PERFORMANCE_CONFIGURATION_INTEL,
    command::QUEUE_SUBMIT,
    command::QUEUE_SUBMIT2,
    command::QUEUE_SUBMIT2_KHR,
    command::QUEUE_WAIT_IDLE,
    command::REGISTER_CUSTOM_BORDER_COLOR_EXT,
    command::REGISTER_DEVICE_EVENT_EXT,
    command::REGISTER_DISPLAY_EVENT_EXT,
    command::RELEASE_CAPTURED_PIPELINE_DATA_KHR,
    command::RELEASE_DISPLAY_EXT,
    command::RELEASE_FULL_SCREEN_EXCLUSIVE_MODE_EXT,
    command::RELEASE_PERFORMANCE_CONFIGURATION_INTEL,
    command::RELEASE_PROFILING_LOCK_KHR,
    command::RELEASE_SWAPCHAIN_IMAGES_EXT,
    command::RELEASE_SWAPCHAIN_IMAGES_KHR,
    command::RESET_COMMAND_BUFFER,
    command::RESET_COMMAND_POOL,
    command::RESET_DESCRIPTOR_POOL,
    command::RESET_EVENT,
    command::RESET_FENCES,
    command::RESET_GPA_SESSION_AMD,
    command::RESET_QUERY_POOL,
    command::RESET_QUERY_POOL_EXT,
    command::SET_BUFFER_COLLECTION_BUFFER_CONSTRAINTS_FUCHSIA,
    command::SET_BUFFER_COLLECTION_IMAGE_CONSTRAINTS_FUCHSIA,
    command::SET_DEBUG_UTILS_OBJECT_NAME_EXT,
    command::SET_DEBUG_UTILS_OBJECT_TAG_EXT,
    command::SET_DEVICE_MEMORY_PRIORITY_EXT,
    command::SET_EVENT,
    command::SET_GPA_DEVICE_CLOCK_MODE_AMD,
    command::SET_HDR_METADATA_EXT,
    command::SET_LATENCY_MARKER_LEGACY_NV,
    command::SET_LATENCY_MARKER_NV,
    command::SET_LATENCY_SLEEP_MODE_LEGACY_NV,
    command::SET_LATENCY_SLEEP_MODE_NV,
    command::SET_LOCAL_DIMMING_AMD,
    command::SET_PRIVATE_DATA,
    command::SET_PRIVATE_DATA_EXT,
    command::SET_SWAPCHAIN_PRESENT_TIMING_QUEUE_SIZE_EXT,
    command::SHUTDOWN_LATENCY_DEVICE_LEGACY_NV,
    command::SIGNAL_SEMAPHORE,
    command::SIGNAL_SEMAPHORE_KHR,
    command::SUBMIT_DEBUG_UTILS_MESSAGE_EXT,
    command::TRANSITION_IMAGE_LAYOUT,
    command::TRANSITION_IMAGE_LAYOUT_EXT,
    command::TRIM_COMMAND_POOL,
    command::TRIM_COMMAND_POOL_KHR,
    command::UNINITIALIZE_PERFORMANCE_API_INTEL,
    command::UNMAP_MEMORY,
    command::UNMAP_MEMORY2,
    command::UNMAP_MEMORY2_KHR,
    command::UNREGISTER_CUSTOM_BORDER_COLOR_EXT,
    command::UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE,
    command::UPDATE_DESCRIPTOR_SET_WITH_TEMPLATE_KHR,
    command::UPDATE_DESCRIPTOR_SETS,
    command::UPDATE_INDIRECT_EXECUTION_SET_PIPELINE_EXT,
    command::UPDATE_INDIRECT_EXECUTION_SET_SHADER_EXT,
    command::UPDATE_VIDEO_SESSION_PARAMETERS_KHR,
    command::WAIT_FOR_FENCES,
    command::WAIT_FOR_PRESENT2_KHR,
    command::WAIT_FOR_PRESENT_KHR,
    command::WAIT_SEMAPHORES,
    command::WAIT_SEMAPHORES_KHR,
    command::WRITE_ACCELERATION_STRUCTURES_PROPERTIES_KHR,
    command::WRITE_MICROMAPS_PROPERTIES_EXT,
    command::WRITE_RESOURCE_DESCRIPTORS_EXT,
    command::WRITE_SAMPLER_DESCRIPTORS_EXT,
];
