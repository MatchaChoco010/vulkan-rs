#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

use core::ffi::{c_char, c_void};

/// Application-defined memory allocation function.
///
/// C function pointer type
/// [`PFN_vkAllocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkAllocationFunction.html).
///
/// # Parameters
///
/// - `pUserData` is the value specified for
///   [`VkAllocationCallbacks`](AllocationCallbacks)::`pUserData` in the allocator specified by
///   the application.
/// - `size` is the size in bytes of the requested allocation.
/// - `alignment` is the requested alignment of the allocation in bytes and must be a power of
///   two.
/// - `allocationScope` is a [`VkSystemAllocationScope`](SystemAllocationScope) value specifying
///   the allocation scope of the lifetime of the allocation, as described here.
pub type PFN_vkAllocationFunction = Option<
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void,
>;

/// Application-defined debug report callback function.
///
/// C function pointer type
/// [`PFN_vkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugReportCallbackEXT.html).
///
/// # Parameters
///
/// - `flags` specifies the [`VkDebugReportFlagBitsEXT`](DebugReportFlagBitsEXT) that triggered
///   this callback.
/// - `objectType` is a [`VkDebugReportObjectTypeEXT`](DebugReportObjectTypeEXT) value
///   specifying the type of object being used or created at the time the event was triggered.
/// - `object` is the object where the issue was detected. If `objectType` is
///   [`VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`](DebugReportObjectTypeEXT::UNKNOWN), `object`
///   is undefined.
/// - `location` is a component (layer, driver, loader) defined value specifying the _location_
///   of the trigger. This is an optional value.
/// - `messageCode` is a layer-defined value indicating what test triggered this callback.
/// - `pLayerPrefix` is a null-terminated UTF-8 string that is an abbreviation of the name of
///   the component making the callback. `pLayerPrefix` is only valid for the duration of the
///   callback.
/// - `pMessage` is a null-terminated UTF-8 string detailing the trigger conditions. `pMessage`
///   is only valid for the duration of the callback.
/// - `pUserData` is the application-defined user data pointer, equal to the value of
///   [`VkDebugReportCallbackCreateInfoEXT`](DebugReportCallbackCreateInfoEXT)::`pUserData`
///   specified when the [`VkDebugReportCallbackEXT`](DebugReportCallbackEXT) object was
///   created.
pub type PFN_vkDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
        p_user_data: *mut c_void,
    ) -> Bool32,
>;

/// Application-defined debug messenger callback function.
///
/// C function pointer type
/// [`PFN_vkDebugUtilsMessengerCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDebugUtilsMessengerCallbackEXT.html).
///
/// # Parameters
///
/// - `messageSeverity` specifies the
///   [`VkDebugUtilsMessageSeverityFlagBitsEXT`](DebugUtilsMessageSeverityFlagBitsEXT) that
///   triggered this callback.
/// - `messageTypes` is a bitmask of
///   [`VkDebugUtilsMessageTypeFlagBitsEXT`](DebugUtilsMessageTypeFlagBitsEXT) specifying which
///   type of event(s) triggered this callback.
/// - `pCallbackData` contains all the callback related data in the
///   [`VkDebugUtilsMessengerCallbackDataEXT`](DebugUtilsMessengerCallbackDataEXT) structure.
/// - `pUserData` is the application-defined user data pointer, equal to the value of
///   [`VkDebugUtilsMessengerCreateInfoEXT`](DebugUtilsMessengerCreateInfoEXT)::`pUserData`
///   specified when the [`VkDebugUtilsMessengerEXT`](DebugUtilsMessengerEXT) object was
///   created.
pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<
    unsafe extern "system" fn(
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
        p_user_data: *mut c_void,
    ) -> Bool32,
>;

/// Application-defined device memory report callback function.
///
/// C function pointer type
/// [`PFN_vkDeviceMemoryReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkDeviceMemoryReportCallbackEXT.html).
///
/// # Parameters
///
/// - `pCallbackData` contains all the callback related data in the
///   [`VkDeviceMemoryReportCallbackDataEXT`](DeviceMemoryReportCallbackDataEXT) structure.
/// - `pUserData` is the application-defined user data pointer, equal to the value of
///   [`VkDeviceDeviceMemoryReportCreateInfoEXT`](DeviceDeviceMemoryReportCreateInfoEXT)::`pUserData`
///   specified when the [`VkDevice`](Device) object was created.
pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<
    unsafe extern "system" fn(
        p_callback_data: *const DeviceMemoryReportCallbackDataEXT<'_>,
        p_user_data: *mut c_void,
    ),
>;

/// Application-defined memory free function.
///
/// C function pointer type
/// [`PFN_vkFreeFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkFreeFunction.html).
///
/// # Parameters
///
/// - `pUserData` is the value specified for
///   [`VkAllocationCallbacks`](AllocationCallbacks)::`pUserData` in the allocator specified by
///   the application.
/// - `pMemory` is the allocation to be freed.
pub type PFN_vkFreeFunction =
    Option<unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void)>;

/// Type definition for vkGetInstanceProcAddr.
///
/// C function pointer type
/// [`PFN_vkGetInstanceProcAddrLUNARG`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkGetInstanceProcAddrLUNARG.html).
///
/// # Parameters
///
/// - `instance` is a [`VkInstance`](Instance) handle.
/// - `pName` is the name of a Vulkan command.
pub type PFN_vkGetInstanceProcAddrLUNARG = Option<
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction,
>;

/// Application-defined memory allocation notification function.
///
/// C function pointer type
/// [`PFN_vkInternalAllocationNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalAllocationNotification.html).
///
/// # Parameters
///
/// - `pUserData` is the value specified for
///   [`VkAllocationCallbacks`](AllocationCallbacks)::`pUserData` in the allocator specified by
///   the application.
/// - `size` is the requested size of an allocation.
/// - `allocationType` is a [`VkInternalAllocationType`](InternalAllocationType) value
///   specifying the requested type of an allocation.
/// - `allocationScope` is a [`VkSystemAllocationScope`](SystemAllocationScope) value specifying
///   the allocation scope of the lifetime of the allocation, as described here.
pub type PFN_vkInternalAllocationNotification = Option<
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

/// Application-defined memory free notification function.
///
/// C function pointer type
/// [`PFN_vkInternalFreeNotification`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkInternalFreeNotification.html).
///
/// # Parameters
///
/// - `pUserData` is the value specified for
///   [`VkAllocationCallbacks`](AllocationCallbacks)::`pUserData` in the allocator specified by
///   the application.
/// - `size` is the requested size of an allocation.
/// - `allocationType` is a [`VkInternalAllocationType`](InternalAllocationType) value
///   specifying the requested type of an allocation.
/// - `allocationScope` is a [`VkSystemAllocationScope`](SystemAllocationScope) value specifying
///   the allocation scope of the lifetime of the allocation, as described here.
pub type PFN_vkInternalFreeNotification = Option<
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        size: usize,
        allocation_type: InternalAllocationType,
        allocation_scope: SystemAllocationScope,
    ),
>;

/// Application-defined memory reallocation function.
///
/// C function pointer type
/// [`PFN_vkReallocationFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkReallocationFunction.html).
///
/// # Parameters
///
/// - `pUserData` is the value specified for
///   [`VkAllocationCallbacks`](AllocationCallbacks)::`pUserData` in the allocator specified by
///   the application.
/// - `pOriginal` must be either `NULL` or a pointer previously returned by `pfnReallocation` or
///   `pfnAllocation` of a compatible allocator.
/// - `size` is the size in bytes of the requested allocation.
/// - `alignment` is the requested alignment of the allocation in bytes and must be a power of
///   two.
/// - `allocationScope` is a [`VkSystemAllocationScope`](SystemAllocationScope) value specifying
///   the allocation scope of the lifetime of the allocation, as described here.
pub type PFN_vkReallocationFunction = Option<
    unsafe extern "system" fn(
        p_user_data: *mut c_void,
        p_original: *mut c_void,
        size: usize,
        alignment: usize,
        allocation_scope: SystemAllocationScope,
    ) -> *mut c_void,
>;

/// Placeholder function pointer type returned by queries.
///
/// C function pointer type
/// [`PFN_vkVoidFunction`](https://docs.vulkan.org/refpages/latest/refpages/source/PFN_vkVoidFunction.html).
pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

/// The type of a pointer to the C command
/// [`vkAcquireDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html),
/// which [`vkAcquireDrmDisplayEXT`](crate::Instance::acquire_drm_display_ext) calls.
pub type PFN_vkAcquireDrmDisplayEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquireFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html),
/// which
/// [`vkAcquireFullScreenExclusiveModeEXT`](crate::Device::acquire_full_screen_exclusive_mode_ext)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkAcquireNextImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html),
/// which [`vkAcquireNextImage2KHR`](crate::Device::acquire_next_image2_khr) calls.
pub type PFN_vkAcquireNextImage2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const AcquireNextImageInfoKHR<'_>,
        p_image_index: *mut u32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquireNextImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html),
/// which [`vkAcquireNextImageKHR`](crate::Device::acquire_next_image_khr) calls.
pub type PFN_vkAcquireNextImageKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
        p_image_index: *mut u32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquirePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html),
/// which
/// [`vkAcquirePerformanceConfigurationINTEL`](crate::Device::acquire_performance_configuration_intel)
/// calls.
pub type PFN_vkAcquirePerformanceConfigurationINTEL = Option<
    unsafe extern "system" fn(
        device: Device,
        p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL<'_>,
        p_configuration: *mut PerformanceConfigurationINTEL,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquireProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html),
/// which [`vkAcquireProfilingLockKHR`](crate::Device::acquire_profiling_lock_khr) calls.
pub type PFN_vkAcquireProfilingLockKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AcquireProfilingLockInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquireWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html),
/// which [`vkAcquireWinrtDisplayNV`](crate::Instance::acquire_winrt_display_nv) calls.
#[cfg(feature = "win32")]
pub type PFN_vkAcquireWinrtDisplayNV = Option<
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAcquireXlibDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html),
/// which [`vkAcquireXlibDisplayEXT`](crate::Instance::acquire_xlib_display_ext) calls.
#[cfg(feature = "xlib-xrandr")]
pub type PFN_vkAcquireXlibDisplayEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAllocateCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html),
/// which [`vkAllocateCommandBuffers`](crate::Device::allocate_command_buffers) calls.
pub type PFN_vkAllocateCommandBuffers = Option<
    unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const CommandBufferAllocateInfo<'_>,
        p_command_buffers: *mut CommandBuffer,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAllocateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html),
/// which [`vkAllocateDescriptorSets`](crate::Device::allocate_descriptor_sets) calls.
pub type PFN_vkAllocateDescriptorSets = Option<
    unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
        p_descriptor_sets: *mut DescriptorSet,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAllocateMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html),
/// which [`vkAllocateMemory`](crate::Device::allocate_memory) calls.
pub type PFN_vkAllocateMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        p_allocate_info: *const MemoryAllocateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_memory: *mut DeviceMemory,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkAntiLagUpdateAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html),
/// which [`vkAntiLagUpdateAMD`](crate::Device::anti_lag_update_amd) calls.
pub type PFN_vkAntiLagUpdateAMD =
    Option<unsafe extern "system" fn(device: Device, p_data: *const AntiLagDataAMD<'_>)>;

/// The type of a pointer to the C command
/// [`vkBeginCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html),
/// which [`vkBeginCommandBuffer`](crate::Device::begin_command_buffer) calls.
pub type PFN_vkBeginCommandBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const CommandBufferBeginInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindAccelerationStructureMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html),
/// which
/// [`vkBindAccelerationStructureMemoryNV`](crate::Device::bind_acceleration_structure_memory_nv)
/// calls.
pub type PFN_vkBindAccelerationStructureMemoryNV = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindAccelerationStructureMemoryInfoNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindBufferMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html),
/// which [`vkBindBufferMemory`](crate::Device::bind_buffer_memory) calls.
pub type PFN_vkBindBufferMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindBufferMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html),
/// which [`vkBindBufferMemory2`](crate::Device::bind_buffer_memory2) calls.
pub type PFN_vkBindBufferMemory2 = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindBufferMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html),
/// which [`vkBindBufferMemory2KHR`](crate::Device::bind_buffer_memory2_khr) calls.
pub type PFN_vkBindBufferMemory2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindBufferMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindDataGraphPipelineSessionMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html),
/// which
/// [`vkBindDataGraphPipelineSessionMemoryARM`](crate::Device::bind_data_graph_pipeline_session_memory_arm)
/// calls.
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindImageMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html),
/// which [`vkBindImageMemory`](crate::Device::bind_image_memory) calls.
pub type PFN_vkBindImageMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindImageMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html),
/// which [`vkBindImageMemory2`](crate::Device::bind_image_memory2) calls.
pub type PFN_vkBindImageMemory2 = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindImageMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html),
/// which [`vkBindImageMemory2KHR`](crate::Device::bind_image_memory2_khr) calls.
pub type PFN_vkBindImageMemory2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindImageMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindOpticalFlowSessionImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html),
/// which [`vkBindOpticalFlowSessionImageNV`](crate::Device::bind_optical_flow_session_image_nv)
/// calls.
pub type PFN_vkBindOpticalFlowSessionImageNV = Option<
    unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindTensorMemoryARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html),
/// which [`vkBindTensorMemoryARM`](crate::Device::bind_tensor_memory_arm) calls.
pub type PFN_vkBindTensorMemoryARM = Option<
    unsafe extern "system" fn(
        device: Device,
        bind_info_count: u32,
        p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBindVideoSessionMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html),
/// which [`vkBindVideoSessionMemoryKHR`](crate::Device::bind_video_session_memory_khr) calls.
pub type PFN_vkBindVideoSessionMemoryKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_info_count: u32,
        p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html),
/// which [`vkBuildAccelerationStructuresKHR`](crate::Device::build_acceleration_structures_khr)
/// calls.
pub type PFN_vkBuildAccelerationStructuresKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html),
/// which [`vkBuildMicromapsEXT`](crate::Device::build_micromaps_ext) calls.
pub type PFN_vkBuildMicromapsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkClearShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html),
/// which
/// [`vkClearShaderInstrumentationMetricsARM`](crate::Device::clear_shader_instrumentation_metrics_arm)
/// calls.
pub type PFN_vkClearShaderInstrumentationMetricsARM =
    Option<unsafe extern "system" fn(device: Device, instrumentation: ShaderInstrumentationARM)>;

/// The type of a pointer to the C command
/// [`vkCmdBeginConditionalRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html),
/// which
/// [`vkCmdBeginConditionalRendering2EXT`](crate::Device::cmd_begin_conditional_rendering2_ext)
/// calls.
pub type PFN_vkCmdBeginConditionalRendering2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfo2EXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html),
/// which
/// [`vkCmdBeginConditionalRenderingEXT`](crate::Device::cmd_begin_conditional_rendering_ext)
/// calls.
pub type PFN_vkCmdBeginConditionalRenderingEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginCustomResolveEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html),
/// which [`vkCmdBeginCustomResolveEXT`](crate::Device::cmd_begin_custom_resolve_ext) calls.
pub type PFN_vkCmdBeginCustomResolveEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_custom_resolve_info: *const BeginCustomResolveInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html),
/// which [`vkCmdBeginDebugUtilsLabelEXT`](crate::Device::cmd_begin_debug_utils_label_ext)
/// calls.
pub type PFN_vkCmdBeginDebugUtilsLabelEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html),
/// which [`vkCmdBeginGpaSampleAMD`](crate::Device::cmd_begin_gpa_sample_amd) calls.
pub type PFN_vkCmdBeginGpaSampleAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        p_gpa_sample_begin_info: *const GpaSampleBeginInfoAMD<'_>,
        p_sample_id: *mut u32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html),
/// which [`vkCmdBeginGpaSessionAMD`](crate::Device::cmd_begin_gpa_session_amd) calls.
pub type PFN_vkCmdBeginGpaSessionAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html),
/// which [`vkCmdBeginPerTileExecutionQCOM`](crate::Device::cmd_begin_per_tile_execution_qcom)
/// calls.
pub type PFN_vkCmdBeginPerTileExecutionQCOM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_per_tile_begin_info: *const PerTileBeginInfoQCOM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html),
/// which [`vkCmdBeginQuery`](crate::Device::cmd_begin_query) calls.
pub type PFN_vkCmdBeginQuery = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html),
/// which [`vkCmdBeginQueryIndexedEXT`](crate::Device::cmd_begin_query_indexed_ext) calls.
pub type PFN_vkCmdBeginQueryIndexedEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html),
/// which [`vkCmdBeginRenderPass`](crate::Device::cmd_begin_render_pass) calls.
pub type PFN_vkCmdBeginRenderPass = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        contents: SubpassContents,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html),
/// which [`vkCmdBeginRenderPass2`](crate::Device::cmd_begin_render_pass2) calls.
pub type PFN_vkCmdBeginRenderPass2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html),
/// which [`vkCmdBeginRenderPass2KHR`](crate::Device::cmd_begin_render_pass2_khr) calls.
pub type PFN_vkCmdBeginRenderPass2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_render_pass_begin: *const RenderPassBeginInfo<'_>,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html),
/// which [`vkCmdBeginRendering`](crate::Device::cmd_begin_rendering) calls.
pub type PFN_vkCmdBeginRendering = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html),
/// which [`vkCmdBeginRenderingKHR`](crate::Device::cmd_begin_rendering_khr) calls.
pub type PFN_vkCmdBeginRenderingKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_info: *const RenderingInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html),
/// which
/// [`vkCmdBeginShaderInstrumentationARM`](crate::Device::cmd_begin_shader_instrumentation_arm)
/// calls.
pub type PFN_vkCmdBeginShaderInstrumentationARM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instrumentation: ShaderInstrumentationARM,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html),
/// which [`vkCmdBeginTransformFeedback2EXT`](crate::Device::cmd_begin_transform_feedback2_ext)
/// calls.
pub type PFN_vkCmdBeginTransformFeedback2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html),
/// which [`vkCmdBeginTransformFeedbackEXT`](crate::Device::cmd_begin_transform_feedback_ext)
/// calls.
pub type PFN_vkCmdBeginTransformFeedbackEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBeginVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html),
/// which [`vkCmdBeginVideoCodingKHR`](crate::Device::cmd_begin_video_coding_khr) calls.
pub type PFN_vkCmdBeginVideoCodingKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html),
/// which
/// [`vkCmdBindDescriptorBufferEmbeddedSamplers2EXT`](crate::Device::cmd_bind_descriptor_buffer_embedded_samplers2_ext)
/// calls.
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = Option<unsafe extern "system" fn(command_buffer: CommandBuffer, p_bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>)>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html),
/// which
/// [`vkCmdBindDescriptorBufferEmbeddedSamplersEXT`](crate::Device::cmd_bind_descriptor_buffer_embedded_samplers_ext)
/// calls.
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html),
/// which [`vkCmdBindDescriptorBuffersEXT`](crate::Device::cmd_bind_descriptor_buffers_ext)
/// calls.
pub type PFN_vkCmdBindDescriptorBuffersEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer_count: u32,
        p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html),
/// which [`vkCmdBindDescriptorSets`](crate::Device::cmd_bind_descriptor_sets) calls.
pub type PFN_vkCmdBindDescriptorSets = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
        dynamic_offset_count: u32,
        p_dynamic_offsets: *const u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorSets2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html),
/// which [`vkCmdBindDescriptorSets2`](crate::Device::cmd_bind_descriptor_sets2) calls.
pub type PFN_vkCmdBindDescriptorSets2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindDescriptorSets2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html),
/// which [`vkCmdBindDescriptorSets2KHR`](crate::Device::cmd_bind_descriptor_sets2_khr) calls.
pub type PFN_vkCmdBindDescriptorSets2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindIndexBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html),
/// which [`vkCmdBindIndexBuffer`](crate::Device::cmd_bind_index_buffer) calls.
pub type PFN_vkCmdBindIndexBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindIndexBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html),
/// which [`vkCmdBindIndexBuffer2`](crate::Device::cmd_bind_index_buffer2) calls.
pub type PFN_vkCmdBindIndexBuffer2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindIndexBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html),
/// which [`vkCmdBindIndexBuffer2KHR`](crate::Device::cmd_bind_index_buffer2_khr) calls.
pub type PFN_vkCmdBindIndexBuffer2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindIndexBuffer3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html),
/// which [`vkCmdBindIndexBuffer3KHR`](crate::Device::cmd_bind_index_buffer3_khr) calls.
pub type PFN_vkCmdBindIndexBuffer3KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const BindIndexBuffer3InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindInvocationMaskHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html),
/// which [`vkCmdBindInvocationMaskHUAWEI`](crate::Device::cmd_bind_invocation_mask_huawei)
/// calls.
pub type PFN_vkCmdBindInvocationMaskHUAWEI = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html),
/// which [`vkCmdBindPipeline`](crate::Device::cmd_bind_pipeline) calls.
pub type PFN_vkCmdBindPipeline = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindPipelineShaderGroupNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html),
/// which [`vkCmdBindPipelineShaderGroupNV`](crate::Device::cmd_bind_pipeline_shader_group_nv)
/// calls.
pub type PFN_vkCmdBindPipelineShaderGroupNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindResourceHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html),
/// which [`vkCmdBindResourceHeapEXT`](crate::Device::cmd_bind_resource_heap_ext) calls.
pub type PFN_vkCmdBindResourceHeapEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindSamplerHeapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html),
/// which [`vkCmdBindSamplerHeapEXT`](crate::Device::cmd_bind_sampler_heap_ext) calls.
pub type PFN_vkCmdBindSamplerHeapEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_bind_info: *const BindHeapInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html),
/// which [`vkCmdBindShadersEXT`](crate::Device::cmd_bind_shaders_ext) calls.
pub type PFN_vkCmdBindShadersEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage_count: u32,
        p_stages: *const ShaderStageFlagBits,
        p_shaders: *const ShaderEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindShadingRateImageNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html),
/// which [`vkCmdBindShadingRateImageNV`](crate::Device::cmd_bind_shading_rate_image_nv) calls.
pub type PFN_vkCmdBindShadingRateImageNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image_view: ImageView,
        image_layout: ImageLayout,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindTileMemoryQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html),
/// which [`vkCmdBindTileMemoryQCOM`](crate::Device::cmd_bind_tile_memory_qcom) calls.
pub type PFN_vkCmdBindTileMemoryQCOM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_tile_memory_bind_info: *const TileMemoryBindInfoQCOM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindTransformFeedbackBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html),
/// which
/// [`vkCmdBindTransformFeedbackBuffers2EXT`](crate::Device::cmd_bind_transform_feedback_buffers2_ext)
/// calls.
pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindTransformFeedbackBuffersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html),
/// which
/// [`vkCmdBindTransformFeedbackBuffersEXT`](crate::Device::cmd_bind_transform_feedback_buffers_ext)
/// calls.
pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindVertexBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html),
/// which [`vkCmdBindVertexBuffers`](crate::Device::cmd_bind_vertex_buffers) calls.
pub type PFN_vkCmdBindVertexBuffers = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindVertexBuffers2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html),
/// which [`vkCmdBindVertexBuffers2`](crate::Device::cmd_bind_vertex_buffers2) calls.
pub type PFN_vkCmdBindVertexBuffers2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindVertexBuffers2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html),
/// which [`vkCmdBindVertexBuffers2EXT`](crate::Device::cmd_bind_vertex_buffers2_ext) calls.
pub type PFN_vkCmdBindVertexBuffers2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
        p_strides: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBindVertexBuffers3KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html),
/// which [`vkCmdBindVertexBuffers3KHR`](crate::Device::cmd_bind_vertex_buffers3_khr) calls.
pub type PFN_vkCmdBindVertexBuffers3KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_binding_infos: *const BindVertexBuffer3InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBlitImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html),
/// which [`vkCmdBlitImage`](crate::Device::cmd_blit_image) calls.
pub type PFN_vkCmdBlitImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageBlit,
        filter: Filter,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBlitImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html),
/// which [`vkCmdBlitImage2`](crate::Device::cmd_blit_image2) calls.
pub type PFN_vkCmdBlitImage2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBlitImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html),
/// which [`vkCmdBlitImage2KHR`](crate::Device::cmd_blit_image2_khr) calls.
pub type PFN_vkCmdBlitImage2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_blit_image_info: *const BlitImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html),
/// which
/// [`vkCmdBuildAccelerationStructureNV`](crate::Device::cmd_build_acceleration_structure_nv)
/// calls.
pub type PFN_vkCmdBuildAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const AccelerationStructureInfoNV<'_>,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: Bool32,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildAccelerationStructuresIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html),
/// which
/// [`vkCmdBuildAccelerationStructuresIndirectKHR`](crate::Device::cmd_build_acceleration_structures_indirect_khr)
/// calls.
pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_indirect_device_addresses: *const DeviceAddress,
        p_indirect_strides: *const u32,
        pp_max_primitive_counts: *const *const u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildAccelerationStructuresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html),
/// which
/// [`vkCmdBuildAccelerationStructuresKHR`](crate::Device::cmd_build_acceleration_structures_khr)
/// calls.
pub type PFN_vkCmdBuildAccelerationStructuresKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildClusterAccelerationStructureIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html),
/// which
/// [`vkCmdBuildClusterAccelerationStructureIndirectNV`](crate::Device::cmd_build_cluster_acceleration_structure_indirect_nv)
/// calls.
pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_command_infos: *const ClusterAccelerationStructureCommandsInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildMicromapsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html),
/// which [`vkCmdBuildMicromapsEXT`](crate::Device::cmd_build_micromaps_ext) calls.
pub type PFN_vkCmdBuildMicromapsEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const MicromapBuildInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdBuildPartitionedAccelerationStructuresNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html),
/// which
/// [`vkCmdBuildPartitionedAccelerationStructuresNV`](crate::Device::cmd_build_partitioned_acceleration_structures_nv)
/// calls.
pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_build_info: *const BuildPartitionedAccelerationStructureInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdClearAttachments`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html),
/// which [`vkCmdClearAttachments`](crate::Device::cmd_clear_attachments) calls.
pub type PFN_vkCmdClearAttachments = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_attachments: *const ClearAttachment,
        rect_count: u32,
        p_rects: *const ClearRect,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdClearColorImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html),
/// which [`vkCmdClearColorImage`](crate::Device::cmd_clear_color_image) calls.
pub type PFN_vkCmdClearColorImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_color: *const ClearColorValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdClearDepthStencilImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html),
/// which [`vkCmdClearDepthStencilImage`](crate::Device::cmd_clear_depth_stencil_image) calls.
pub type PFN_vkCmdClearDepthStencilImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        p_depth_stencil: *const ClearDepthStencilValue,
        range_count: u32,
        p_ranges: *const ImageSubresourceRange,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdControlVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html),
/// which [`vkCmdControlVideoCodingKHR`](crate::Device::cmd_control_video_coding_khr) calls.
pub type PFN_vkCmdControlVideoCodingKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html),
/// which
/// [`vkCmdConvertCooperativeVectorMatrixNV`](crate::Device::cmd_convert_cooperative_vector_matrix_nv)
/// calls.
pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        info_count: u32,
        p_infos: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html),
/// which
/// [`vkCmdCopyAccelerationStructureKHR`](crate::Device::cmd_copy_acceleration_structure_khr)
/// calls.
pub type PFN_vkCmdCopyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html),
/// which
/// [`vkCmdCopyAccelerationStructureNV`](crate::Device::cmd_copy_acceleration_structure_nv)
/// calls.
pub type PFN_vkCmdCopyAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html),
/// which
/// [`vkCmdCopyAccelerationStructureToMemoryKHR`](crate::Device::cmd_copy_acceleration_structure_to_memory_khr)
/// calls.
pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html),
/// which [`vkCmdCopyBuffer`](crate::Device::cmd_copy_buffer) calls.
pub type PFN_vkCmdCopyBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferCopy,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html),
/// which [`vkCmdCopyBuffer2`](crate::Device::cmd_copy_buffer2) calls.
pub type PFN_vkCmdCopyBuffer2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html),
/// which [`vkCmdCopyBuffer2KHR`](crate::Device::cmd_copy_buffer2_khr) calls.
pub type PFN_vkCmdCopyBuffer2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_info: *const CopyBufferInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBufferToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html),
/// which [`vkCmdCopyBufferToImage`](crate::Device::cmd_copy_buffer_to_image) calls.
pub type PFN_vkCmdCopyBufferToImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBufferToImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html),
/// which [`vkCmdCopyBufferToImage2`](crate::Device::cmd_copy_buffer_to_image2) calls.
pub type PFN_vkCmdCopyBufferToImage2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyBufferToImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html),
/// which [`vkCmdCopyBufferToImage2KHR`](crate::Device::cmd_copy_buffer_to_image2_khr) calls.
pub type PFN_vkCmdCopyBufferToImage2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html),
/// which [`vkCmdCopyGpaSessionResultsAMD`](crate::Device::cmd_copy_gpa_session_results_amd)
/// calls.
pub type PFN_vkCmdCopyGpaSessionResultsAMD =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD)>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html),
/// which [`vkCmdCopyImage`](crate::Device::cmd_copy_image) calls.
pub type PFN_vkCmdCopyImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageCopy,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html),
/// which [`vkCmdCopyImage2`](crate::Device::cmd_copy_image2) calls.
pub type PFN_vkCmdCopyImage2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html),
/// which [`vkCmdCopyImage2KHR`](crate::Device::cmd_copy_image2_khr) calls.
pub type PFN_vkCmdCopyImage2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_info: *const CopyImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImageToBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html),
/// which [`vkCmdCopyImageToBuffer`](crate::Device::cmd_copy_image_to_buffer) calls.
pub type PFN_vkCmdCopyImageToBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        region_count: u32,
        p_regions: *const BufferImageCopy,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImageToBuffer2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html),
/// which [`vkCmdCopyImageToBuffer2`](crate::Device::cmd_copy_image_to_buffer2) calls.
pub type PFN_vkCmdCopyImageToBuffer2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImageToBuffer2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html),
/// which [`vkCmdCopyImageToBuffer2KHR`](crate::Device::cmd_copy_image_to_buffer2_khr) calls.
pub type PFN_vkCmdCopyImageToBuffer2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyImageToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html),
/// which [`vkCmdCopyImageToMemoryKHR`](crate::Device::cmd_copy_image_to_memory_khr) calls.
pub type PFN_vkCmdCopyImageToMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html),
/// which [`vkCmdCopyMemoryIndirectKHR`](crate::Device::cmd_copy_memory_indirect_khr) calls.
pub type PFN_vkCmdCopyMemoryIndirectKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html),
/// which [`vkCmdCopyMemoryIndirectNV`](crate::Device::cmd_copy_memory_indirect_nv) calls.
pub type PFN_vkCmdCopyMemoryIndirectNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html),
/// which [`vkCmdCopyMemoryKHR`](crate::Device::cmd_copy_memory_khr) calls.
pub type PFN_vkCmdCopyMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html),
/// which
/// [`vkCmdCopyMemoryToAccelerationStructureKHR`](crate::Device::cmd_copy_memory_to_acceleration_structure_khr)
/// calls.
pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryToImageIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html),
/// which
/// [`vkCmdCopyMemoryToImageIndirectKHR`](crate::Device::cmd_copy_memory_to_image_indirect_khr)
/// calls.
pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryToImageIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html),
/// which
/// [`vkCmdCopyMemoryToImageIndirectNV`](crate::Device::cmd_copy_memory_to_image_indirect_nv)
/// calls.
pub type PFN_vkCmdCopyMemoryToImageIndirectNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        p_image_subresources: *const ImageSubresourceLayers,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryToImageKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html),
/// which [`vkCmdCopyMemoryToImageKHR`](crate::Device::cmd_copy_memory_to_image_khr) calls.
pub type PFN_vkCmdCopyMemoryToImageKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_memory_info: *const CopyDeviceMemoryImageInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html),
/// which [`vkCmdCopyMemoryToMicromapEXT`](crate::Device::cmd_copy_memory_to_micromap_ext)
/// calls.
pub type PFN_vkCmdCopyMemoryToMicromapEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html),
/// which [`vkCmdCopyMicromapEXT`](crate::Device::cmd_copy_micromap_ext) calls.
pub type PFN_vkCmdCopyMicromapEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html),
/// which [`vkCmdCopyMicromapToMemoryEXT`](crate::Device::cmd_copy_micromap_to_memory_ext)
/// calls.
pub type PFN_vkCmdCopyMicromapToMemoryEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html),
/// which [`vkCmdCopyQueryPoolResults`](crate::Device::cmd_copy_query_pool_results) calls.
pub type PFN_vkCmdCopyQueryPoolResults = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyQueryPoolResultsToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html),
/// which
/// [`vkCmdCopyQueryPoolResultsToMemoryKHR`](crate::Device::cmd_copy_query_pool_results_to_memory_khr)
/// calls.
pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        p_dst_range: *const StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCopyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html),
/// which [`vkCmdCopyTensorARM`](crate::Device::cmd_copy_tensor_arm) calls.
pub type PFN_vkCmdCopyTensorARM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCuLaunchKernelNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html),
/// which [`vkCmdCuLaunchKernelNVX`](crate::Device::cmd_cu_launch_kernel_nvx) calls.
pub type PFN_vkCmdCuLaunchKernelNVX = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_launch_info: *const CuLaunchInfoNVX<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdCudaLaunchKernelNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html),
/// which [`vkCmdCudaLaunchKernelNV`](crate::Device::cmd_cuda_launch_kernel_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkCmdCudaLaunchKernelNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_launch_info: *const CudaLaunchInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDebugMarkerBeginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html),
/// which [`vkCmdDebugMarkerBeginEXT`](crate::Device::cmd_debug_marker_begin_ext) calls.
pub type PFN_vkCmdDebugMarkerBeginEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDebugMarkerEndEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html),
/// which [`vkCmdDebugMarkerEndEXT`](crate::Device::cmd_debug_marker_end_ext) calls.
pub type PFN_vkCmdDebugMarkerEndEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdDebugMarkerInsertEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html),
/// which [`vkCmdDebugMarkerInsertEXT`](crate::Device::cmd_debug_marker_insert_ext) calls.
pub type PFN_vkCmdDebugMarkerInsertEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const DebugMarkerMarkerInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDecodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html),
/// which [`vkCmdDecodeVideoKHR`](crate::Device::cmd_decode_video_khr) calls.
pub type PFN_vkCmdDecodeVideoKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_decode_info: *const VideoDecodeInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDecompressMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html),
/// which [`vkCmdDecompressMemoryEXT`](crate::Device::cmd_decompress_memory_ext) calls.
pub type PFN_vkCmdDecompressMemoryEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_decompress_memory_info_ext: *const DecompressMemoryInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDecompressMemoryIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html),
/// which
/// [`vkCmdDecompressMemoryIndirectCountEXT`](crate::Device::cmd_decompress_memory_indirect_count_ext)
/// calls.
pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDecompressMemoryIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html),
/// which
/// [`vkCmdDecompressMemoryIndirectCountNV`](crate::Device::cmd_decompress_memory_indirect_count_nv)
/// calls.
pub type PFN_vkCmdDecompressMemoryIndirectCountNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDecompressMemoryNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html),
/// which [`vkCmdDecompressMemoryNV`](crate::Device::cmd_decompress_memory_nv) calls.
pub type PFN_vkCmdDecompressMemoryNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        decompress_region_count: u32,
        p_decompress_memory_regions: *const DecompressMemoryRegionNV,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatch`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html),
/// which [`vkCmdDispatch`](crate::Device::cmd_dispatch) calls.
pub type PFN_vkCmdDispatch = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchBase`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html),
/// which [`vkCmdDispatchBase`](crate::Device::cmd_dispatch_base) calls.
pub type PFN_vkCmdDispatchBase = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchBaseKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html),
/// which [`vkCmdDispatchBaseKHR`](crate::Device::cmd_dispatch_base_khr) calls.
pub type PFN_vkCmdDispatchBaseKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchDataGraphARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html),
/// which [`vkCmdDispatchDataGraphARM`](crate::Device::cmd_dispatch_data_graph_arm) calls.
pub type PFN_vkCmdDispatchDataGraphARM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        p_info: *const DataGraphPipelineDispatchInfoARM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchGraphAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html),
/// which [`vkCmdDispatchGraphAMDX`](crate::Device::cmd_dispatch_graph_amdx) calls.
#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphAMDX = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchGraphIndirectAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html),
/// which [`vkCmdDispatchGraphIndirectAMDX`](crate::Device::cmd_dispatch_graph_indirect_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        p_count_info: *const DispatchGraphCountInfoAMDX<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchGraphIndirectCountAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html),
/// which
/// [`vkCmdDispatchGraphIndirectCountAMDX`](crate::Device::cmd_dispatch_graph_indirect_count_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html),
/// which [`vkCmdDispatchIndirect`](crate::Device::cmd_dispatch_indirect) calls.
pub type PFN_vkCmdDispatchIndirect = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html),
/// which [`vkCmdDispatchIndirect2KHR`](crate::Device::cmd_dispatch_indirect2_khr) calls.
pub type PFN_vkCmdDispatchIndirect2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DispatchIndirect2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDispatchTileQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html),
/// which [`vkCmdDispatchTileQCOM`](crate::Device::cmd_dispatch_tile_qcom) calls.
pub type PFN_vkCmdDispatchTileQCOM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dispatch_tile_info: *const DispatchTileInfoQCOM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDraw`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html), which
/// [`vkCmdDraw`](crate::Device::cmd_draw) calls.
pub type PFN_vkCmdDraw = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawClusterHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html),
/// which [`vkCmdDrawClusterHUAWEI`](crate::Device::cmd_draw_cluster_huawei) calls.
pub type PFN_vkCmdDrawClusterHUAWEI = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawClusterIndirectHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html),
/// which [`vkCmdDrawClusterIndirectHUAWEI`](crate::Device::cmd_draw_cluster_indirect_huawei)
/// calls.
pub type PFN_vkCmdDrawClusterIndirectHUAWEI = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexed`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html),
/// which [`vkCmdDrawIndexed`](crate::Device::cmd_draw_indexed) calls.
pub type PFN_vkCmdDrawIndexed = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html),
/// which [`vkCmdDrawIndexedIndirect`](crate::Device::cmd_draw_indexed_indirect) calls.
pub type PFN_vkCmdDrawIndexedIndirect = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html),
/// which [`vkCmdDrawIndexedIndirect2KHR`](crate::Device::cmd_draw_indexed_indirect2_khr) calls.
pub type PFN_vkCmdDrawIndexedIndirect2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html),
/// which [`vkCmdDrawIndexedIndirectCount`](crate::Device::cmd_draw_indexed_indirect_count)
/// calls.
pub type PFN_vkCmdDrawIndexedIndirectCount = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html),
/// which
/// [`vkCmdDrawIndexedIndirectCount2KHR`](crate::Device::cmd_draw_indexed_indirect_count2_khr)
/// calls.
pub type PFN_vkCmdDrawIndexedIndirectCount2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html),
/// which
/// [`vkCmdDrawIndexedIndirectCountAMD`](crate::Device::cmd_draw_indexed_indirect_count_amd)
/// calls.
pub type PFN_vkCmdDrawIndexedIndirectCountAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndexedIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html),
/// which
/// [`vkCmdDrawIndexedIndirectCountKHR`](crate::Device::cmd_draw_indexed_indirect_count_khr)
/// calls.
pub type PFN_vkCmdDrawIndexedIndirectCountKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirect`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html),
/// which [`vkCmdDrawIndirect`](crate::Device::cmd_draw_indirect) calls.
pub type PFN_vkCmdDrawIndirect = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html),
/// which [`vkCmdDrawIndirect2KHR`](crate::Device::cmd_draw_indirect2_khr) calls.
pub type PFN_vkCmdDrawIndirect2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectByteCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html),
/// which [`vkCmdDrawIndirectByteCount2EXT`](crate::Device::cmd_draw_indirect_byte_count2_ext)
/// calls.
pub type PFN_vkCmdDrawIndirectByteCount2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        p_counter_info: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
        counter_offset: u32,
        vertex_stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectByteCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html),
/// which [`vkCmdDrawIndirectByteCountEXT`](crate::Device::cmd_draw_indirect_byte_count_ext)
/// calls.
pub type PFN_vkCmdDrawIndirectByteCountEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html),
/// which [`vkCmdDrawIndirectCount`](crate::Device::cmd_draw_indirect_count) calls.
pub type PFN_vkCmdDrawIndirectCount = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectCount2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html),
/// which [`vkCmdDrawIndirectCount2KHR`](crate::Device::cmd_draw_indirect_count2_khr) calls.
pub type PFN_vkCmdDrawIndirectCount2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectCountAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html),
/// which [`vkCmdDrawIndirectCountAMD`](crate::Device::cmd_draw_indirect_count_amd) calls.
pub type PFN_vkCmdDrawIndirectCountAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawIndirectCountKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html),
/// which [`vkCmdDrawIndirectCountKHR`](crate::Device::cmd_draw_indirect_count_khr) calls.
pub type PFN_vkCmdDrawIndirectCountKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksEXT.html),
/// which [`vkCmdDrawMeshTasksEXT`](crate::Device::cmd_draw_mesh_tasks_ext) calls.
pub type PFN_vkCmdDrawMeshTasksEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirect2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html),
/// which [`vkCmdDrawMeshTasksIndirect2EXT`](crate::Device::cmd_draw_mesh_tasks_indirect2_ext)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirect2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirect2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirectCount2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html),
/// which
/// [`vkCmdDrawMeshTasksIndirectCount2EXT`](crate::Device::cmd_draw_mesh_tasks_indirect_count2_ext)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const DrawIndirectCount2InfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirectCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountEXT.html),
/// which
/// [`vkCmdDrawMeshTasksIndirectCountEXT`](crate::Device::cmd_draw_mesh_tasks_indirect_count_ext)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirectCountNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html),
/// which
/// [`vkCmdDrawMeshTasksIndirectCountNV`](crate::Device::cmd_draw_mesh_tasks_indirect_count_nv)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirectCountNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirectEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectEXT.html),
/// which [`vkCmdDrawMeshTasksIndirectEXT`](crate::Device::cmd_draw_mesh_tasks_indirect_ext)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirectEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksIndirectNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html),
/// which [`vkCmdDrawMeshTasksIndirectNV`](crate::Device::cmd_draw_mesh_tasks_indirect_nv)
/// calls.
pub type PFN_vkCmdDrawMeshTasksIndirectNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMeshTasksNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html),
/// which [`vkCmdDrawMeshTasksNV`](crate::Device::cmd_draw_mesh_tasks_nv) calls.
pub type PFN_vkCmdDrawMeshTasksNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, task_count: u32, first_task: u32),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMultiEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html),
/// which [`vkCmdDrawMultiEXT`](crate::Device::cmd_draw_multi_ext) calls.
pub type PFN_vkCmdDrawMultiEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_vertex_info: *const MultiDrawInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdDrawMultiIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html),
/// which [`vkCmdDrawMultiIndexedEXT`](crate::Device::cmd_draw_multi_indexed_ext) calls.
pub type PFN_vkCmdDrawMultiIndexedEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        draw_count: u32,
        p_index_info: *const MultiDrawIndexedInfoEXT,
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        p_vertex_offset: *const i32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEncodeVideoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html),
/// which [`vkCmdEncodeVideoKHR`](crate::Device::cmd_encode_video_khr) calls.
pub type PFN_vkCmdEncodeVideoKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_encode_info: *const VideoEncodeInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndConditionalRenderingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html),
/// which [`vkCmdEndConditionalRenderingEXT`](crate::Device::cmd_end_conditional_rendering_ext)
/// calls.
pub type PFN_vkCmdEndConditionalRenderingEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html),
/// which [`vkCmdEndDebugUtilsLabelEXT`](crate::Device::cmd_end_debug_utils_label_ext) calls.
pub type PFN_vkCmdEndDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndGpaSampleAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html),
/// which [`vkCmdEndGpaSampleAMD`](crate::Device::cmd_end_gpa_sample_amd) calls.
pub type PFN_vkCmdEndGpaSampleAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html),
/// which [`vkCmdEndGpaSessionAMD`](crate::Device::cmd_end_gpa_session_amd) calls.
pub type PFN_vkCmdEndGpaSessionAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdEndPerTileExecutionQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html),
/// which [`vkCmdEndPerTileExecutionQCOM`](crate::Device::cmd_end_per_tile_execution_qcom)
/// calls.
pub type PFN_vkCmdEndPerTileExecutionQCOM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_per_tile_end_info: *const PerTileEndInfoQCOM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndQuery`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html),
/// which [`vkCmdEndQuery`](crate::Device::cmd_end_query) calls.
pub type PFN_vkCmdEndQuery = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndQueryIndexedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html),
/// which [`vkCmdEndQueryIndexedEXT`](crate::Device::cmd_end_query_indexed_ext) calls.
pub type PFN_vkCmdEndQueryIndexedEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html),
/// which [`vkCmdEndRenderPass`](crate::Device::cmd_end_render_pass) calls.
pub type PFN_vkCmdEndRenderPass = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html),
/// which [`vkCmdEndRenderPass2`](crate::Device::cmd_end_render_pass2) calls.
pub type PFN_vkCmdEndRenderPass2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html),
/// which [`vkCmdEndRenderPass2KHR`](crate::Device::cmd_end_render_pass2_khr) calls.
pub type PFN_vkCmdEndRenderPass2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndRendering`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html),
/// which [`vkCmdEndRendering`](crate::Device::cmd_end_rendering) calls.
pub type PFN_vkCmdEndRendering = Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndRendering2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html),
/// which [`vkCmdEndRendering2EXT`](crate::Device::cmd_end_rendering2_ext) calls.
pub type PFN_vkCmdEndRendering2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndRendering2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html),
/// which [`vkCmdEndRendering2KHR`](crate::Device::cmd_end_rendering2_khr) calls.
pub type PFN_vkCmdEndRendering2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_rendering_end_info: *const RenderingEndInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndRenderingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html),
/// which [`vkCmdEndRenderingKHR`](crate::Device::cmd_end_rendering_khr) calls.
pub type PFN_vkCmdEndRenderingKHR =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html),
/// which
/// [`vkCmdEndShaderInstrumentationARM`](crate::Device::cmd_end_shader_instrumentation_arm)
/// calls.
pub type PFN_vkCmdEndShaderInstrumentationARM =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdEndTransformFeedback2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html),
/// which [`vkCmdEndTransformFeedback2EXT`](crate::Device::cmd_end_transform_feedback2_ext)
/// calls.
pub type PFN_vkCmdEndTransformFeedback2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_range: u32,
        counter_range_count: u32,
        p_counter_infos: *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndTransformFeedbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html),
/// which [`vkCmdEndTransformFeedbackEXT`](crate::Device::cmd_end_transform_feedback_ext) calls.
pub type PFN_vkCmdEndTransformFeedbackEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdEndVideoCodingKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html),
/// which [`vkCmdEndVideoCodingKHR`](crate::Device::cmd_end_video_coding_khr) calls.
pub type PFN_vkCmdEndVideoCodingKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdExecuteCommands`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html),
/// which [`vkCmdExecuteCommands`](crate::Device::cmd_execute_commands) calls.
pub type PFN_vkCmdExecuteCommands = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdExecuteGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html),
/// which
/// [`vkCmdExecuteGeneratedCommandsEXT`](crate::Device::cmd_execute_generated_commands_ext)
/// calls.
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdExecuteGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html),
/// which [`vkCmdExecuteGeneratedCommandsNV`](crate::Device::cmd_execute_generated_commands_nv)
/// calls.
pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdFillBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html),
/// which [`vkCmdFillBuffer`](crate::Device::cmd_fill_buffer) calls.
pub type PFN_vkCmdFillBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdFillMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html),
/// which [`vkCmdFillMemoryKHR`](crate::Device::cmd_fill_memory_khr) calls.
pub type PFN_vkCmdFillMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdInitializeGraphScratchMemoryAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html),
/// which
/// [`vkCmdInitializeGraphScratchMemoryAMDX`](crate::Device::cmd_initialize_graph_scratch_memory_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html),
/// which [`vkCmdInsertDebugUtilsLabelEXT`](crate::Device::cmd_insert_debug_utils_label_ext)
/// calls.
pub type PFN_vkCmdInsertDebugUtilsLabelEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_label_info: *const DebugUtilsLabelEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdNextSubpass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html),
/// which [`vkCmdNextSubpass`](crate::Device::cmd_next_subpass) calls.
pub type PFN_vkCmdNextSubpass =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents)>;

/// The type of a pointer to the C command
/// [`vkCmdNextSubpass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html),
/// which [`vkCmdNextSubpass2`](crate::Device::cmd_next_subpass2) calls.
pub type PFN_vkCmdNextSubpass2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdNextSubpass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html),
/// which [`vkCmdNextSubpass2KHR`](crate::Device::cmd_next_subpass2_khr) calls.
pub type PFN_vkCmdNextSubpass2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_subpass_begin_info: *const SubpassBeginInfo<'_>,
        p_subpass_end_info: *const SubpassEndInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdOpticalFlowExecuteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html),
/// which [`vkCmdOpticalFlowExecuteNV`](crate::Device::cmd_optical_flow_execute_nv) calls.
pub type PFN_vkCmdOpticalFlowExecuteNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        p_execute_info: *const OpticalFlowExecuteInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPipelineBarrier`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html),
/// which [`vkCmdPipelineBarrier`](crate::Device::cmd_pipeline_barrier) calls.
pub type PFN_vkCmdPipelineBarrier = Option<
    unsafe extern "system" fn(
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
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPipelineBarrier2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html),
/// which [`vkCmdPipelineBarrier2`](crate::Device::cmd_pipeline_barrier2) calls.
pub type PFN_vkCmdPipelineBarrier2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPipelineBarrier2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html),
/// which [`vkCmdPipelineBarrier2KHR`](crate::Device::cmd_pipeline_barrier2_khr) calls.
pub type PFN_vkCmdPipelineBarrier2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dependency_info: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPreprocessGeneratedCommandsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html),
/// which
/// [`vkCmdPreprocessGeneratedCommandsEXT`](crate::Device::cmd_preprocess_generated_commands_ext)
/// calls.
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
        state_command_buffer: CommandBuffer,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPreprocessGeneratedCommandsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html),
/// which
/// [`vkCmdPreprocessGeneratedCommandsNV`](crate::Device::cmd_preprocess_generated_commands_nv)
/// calls.
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html),
/// which [`vkCmdPushConstants`](crate::Device::cmd_push_constants) calls.
pub type PFN_vkCmdPushConstants = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        size: u32,
        p_values: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushConstants2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html),
/// which [`vkCmdPushConstants2`](crate::Device::cmd_push_constants2) calls.
pub type PFN_vkCmdPushConstants2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushConstants2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html),
/// which [`vkCmdPushConstants2KHR`](crate::Device::cmd_push_constants2_khr) calls.
pub type PFN_vkCmdPushConstants2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_constants_info: *const PushConstantsInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html),
/// which [`vkCmdPushDataEXT`](crate::Device::cmd_push_data_ext) calls.
pub type PFN_vkCmdPushDataEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_data_info: *const PushDataInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html),
/// which [`vkCmdPushDescriptorSet`](crate::Device::cmd_push_descriptor_set) calls.
pub type PFN_vkCmdPushDescriptorSet = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSet2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html),
/// which [`vkCmdPushDescriptorSet2`](crate::Device::cmd_push_descriptor_set2) calls.
pub type PFN_vkCmdPushDescriptorSet2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSet2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html),
/// which [`vkCmdPushDescriptorSet2KHR`](crate::Device::cmd_push_descriptor_set2_khr) calls.
pub type PFN_vkCmdPushDescriptorSet2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_info: *const PushDescriptorSetInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSetKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html),
/// which [`vkCmdPushDescriptorSetKHR`](crate::Device::cmd_push_descriptor_set_khr) calls.
pub type PFN_vkCmdPushDescriptorSetKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html),
/// which
/// [`vkCmdPushDescriptorSetWithTemplate`](crate::Device::cmd_push_descriptor_set_with_template)
/// calls.
pub type PFN_vkCmdPushDescriptorSetWithTemplate = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSetWithTemplate2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html),
/// which
/// [`vkCmdPushDescriptorSetWithTemplate2`](crate::Device::cmd_push_descriptor_set_with_template2)
/// calls.
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSetWithTemplate2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html),
/// which
/// [`vkCmdPushDescriptorSetWithTemplate2KHR`](crate::Device::cmd_push_descriptor_set_with_template2_khr)
/// calls.
pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdPushDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html),
/// which
/// [`vkCmdPushDescriptorSetWithTemplateKHR`](crate::Device::cmd_push_descriptor_set_with_template_khr)
/// calls.
pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html),
/// which [`vkCmdResetEvent`](crate::Device::cmd_reset_event) calls.
pub type PFN_vkCmdResetEvent = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html),
/// which [`vkCmdResetEvent2`](crate::Device::cmd_reset_event2) calls.
pub type PFN_vkCmdResetEvent2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html),
/// which [`vkCmdResetEvent2KHR`](crate::Device::cmd_reset_event2_khr) calls.
pub type PFN_vkCmdResetEvent2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html),
/// which [`vkCmdResetQueryPool`](crate::Device::cmd_reset_query_pool) calls.
pub type PFN_vkCmdResetQueryPool = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResolveImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html),
/// which [`vkCmdResolveImage`](crate::Device::cmd_resolve_image) calls.
pub type PFN_vkCmdResolveImage = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        region_count: u32,
        p_regions: *const ImageResolve,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResolveImage2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html),
/// which [`vkCmdResolveImage2`](crate::Device::cmd_resolve_image2) calls.
pub type PFN_vkCmdResolveImage2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdResolveImage2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html),
/// which [`vkCmdResolveImage2KHR`](crate::Device::cmd_resolve_image2_khr) calls.
pub type PFN_vkCmdResolveImage2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_resolve_image_info: *const ResolveImageInfo2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetAlphaToCoverageEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html),
/// which
/// [`vkCmdSetAlphaToCoverageEnableEXT`](crate::Device::cmd_set_alpha_to_coverage_enable_ext)
/// calls.
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetAlphaToOneEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html),
/// which [`vkCmdSetAlphaToOneEnableEXT`](crate::Device::cmd_set_alpha_to_one_enable_ext) calls.
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetAttachmentFeedbackLoopEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html),
/// which
/// [`vkCmdSetAttachmentFeedbackLoopEnableEXT`](crate::Device::cmd_set_attachment_feedback_loop_enable_ext)
/// calls.
pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags)>;

/// The type of a pointer to the C command
/// [`vkCmdSetBlendConstants`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html),
/// which [`vkCmdSetBlendConstants`](crate::Device::cmd_set_blend_constants) calls.
pub type PFN_vkCmdSetBlendConstants = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4]),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCheckpointNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html),
/// which [`vkCmdSetCheckpointNV`](crate::Device::cmd_set_checkpoint_nv) calls.
pub type PFN_vkCmdSetCheckpointNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const c_void),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoarseSampleOrderNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html),
/// which [`vkCmdSetCoarseSampleOrderNV`](crate::Device::cmd_set_coarse_sample_order_nv) calls.
pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_order_count: u32,
        p_custom_sample_orders: *const CoarseSampleOrderCustomNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetColorBlendAdvancedEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html),
/// which [`vkCmdSetColorBlendAdvancedEXT`](crate::Device::cmd_set_color_blend_advanced_ext)
/// calls.
pub type PFN_vkCmdSetColorBlendAdvancedEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_advanced: *const ColorBlendAdvancedEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetColorBlendEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html),
/// which [`vkCmdSetColorBlendEnableEXT`](crate::Device::cmd_set_color_blend_enable_ext) calls.
pub type PFN_vkCmdSetColorBlendEnableEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_enables: *const Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetColorBlendEquationEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html),
/// which [`vkCmdSetColorBlendEquationEXT`](crate::Device::cmd_set_color_blend_equation_ext)
/// calls.
pub type PFN_vkCmdSetColorBlendEquationEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_blend_equations: *const ColorBlendEquationEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetColorWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html),
/// which [`vkCmdSetColorWriteEnableEXT`](crate::Device::cmd_set_color_write_enable_ext) calls.
pub type PFN_vkCmdSetColorWriteEnableEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        attachment_count: u32,
        p_color_write_enables: *const Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetColorWriteMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html),
/// which [`vkCmdSetColorWriteMaskEXT`](crate::Device::cmd_set_color_write_mask_ext) calls.
pub type PFN_vkCmdSetColorWriteMaskEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_attachment: u32,
        attachment_count: u32,
        p_color_write_masks: *const ColorComponentFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetComputeOccupancyPriorityNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html),
/// which
/// [`vkCmdSetComputeOccupancyPriorityNV`](crate::Device::cmd_set_compute_occupancy_priority_nv)
/// calls.
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_parameters: *const ComputeOccupancyPriorityParametersNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetConservativeRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html),
/// which
/// [`vkCmdSetConservativeRasterizationModeEXT`](crate::Device::cmd_set_conservative_rasterization_mode_ext)
/// calls.
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageModulationModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html),
/// which
/// [`vkCmdSetCoverageModulationModeNV`](crate::Device::cmd_set_coverage_modulation_mode_nv)
/// calls.
pub type PFN_vkCmdSetCoverageModulationModeNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageModulationTableEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html),
/// which
/// [`vkCmdSetCoverageModulationTableEnableNV`](crate::Device::cmd_set_coverage_modulation_table_enable_nv)
/// calls.
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageModulationTableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html),
/// which
/// [`vkCmdSetCoverageModulationTableNV`](crate::Device::cmd_set_coverage_modulation_table_nv)
/// calls.
pub type PFN_vkCmdSetCoverageModulationTableNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_modulation_table_count: u32,
        p_coverage_modulation_table: *const f32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageReductionModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html),
/// which [`vkCmdSetCoverageReductionModeNV`](crate::Device::cmd_set_coverage_reduction_mode_nv)
/// calls.
pub type PFN_vkCmdSetCoverageReductionModeNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageToColorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html),
/// which
/// [`vkCmdSetCoverageToColorEnableNV`](crate::Device::cmd_set_coverage_to_color_enable_nv)
/// calls.
pub type PFN_vkCmdSetCoverageToColorEnableNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCoverageToColorLocationNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html),
/// which
/// [`vkCmdSetCoverageToColorLocationNV`](crate::Device::cmd_set_coverage_to_color_location_nv)
/// calls.
pub type PFN_vkCmdSetCoverageToColorLocationNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetCullMode`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html),
/// which [`vkCmdSetCullMode`](crate::Device::cmd_set_cull_mode) calls.
pub type PFN_vkCmdSetCullMode =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags)>;

/// The type of a pointer to the C command
/// [`vkCmdSetCullModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html),
/// which [`vkCmdSetCullModeEXT`](crate::Device::cmd_set_cull_mode_ext) calls.
pub type PFN_vkCmdSetCullModeEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBias`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html),
/// which [`vkCmdSetDepthBias`](crate::Device::cmd_set_depth_bias) calls.
pub type PFN_vkCmdSetDepthBias = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBias2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html),
/// which [`vkCmdSetDepthBias2EXT`](crate::Device::cmd_set_depth_bias2_ext) calls.
pub type PFN_vkCmdSetDepthBias2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_depth_bias_info: *const DepthBiasInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBiasEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html),
/// which [`vkCmdSetDepthBiasEnable`](crate::Device::cmd_set_depth_bias_enable) calls.
pub type PFN_vkCmdSetDepthBiasEnable =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBiasEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html),
/// which [`vkCmdSetDepthBiasEnableEXT`](crate::Device::cmd_set_depth_bias_enable_ext) calls.
pub type PFN_vkCmdSetDepthBiasEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBounds`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html),
/// which [`vkCmdSetDepthBounds`](crate::Device::cmd_set_depth_bounds) calls.
pub type PFN_vkCmdSetDepthBounds = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBoundsTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html),
/// which [`vkCmdSetDepthBoundsTestEnable`](crate::Device::cmd_set_depth_bounds_test_enable)
/// calls.
pub type PFN_vkCmdSetDepthBoundsTestEnable = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthBoundsTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html),
/// which
/// [`vkCmdSetDepthBoundsTestEnableEXT`](crate::Device::cmd_set_depth_bounds_test_enable_ext)
/// calls.
pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthClampEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html),
/// which [`vkCmdSetDepthClampEnableEXT`](crate::Device::cmd_set_depth_clamp_enable_ext) calls.
pub type PFN_vkCmdSetDepthClampEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthClampRangeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html),
/// which [`vkCmdSetDepthClampRangeEXT`](crate::Device::cmd_set_depth_clamp_range_ext) calls.
pub type PFN_vkCmdSetDepthClampRangeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        p_depth_clamp_range: *const DepthClampRangeEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthClipEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html),
/// which [`vkCmdSetDepthClipEnableEXT`](crate::Device::cmd_set_depth_clip_enable_ext) calls.
pub type PFN_vkCmdSetDepthClipEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthClipNegativeOneToOneEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html),
/// which
/// [`vkCmdSetDepthClipNegativeOneToOneEXT`](crate::Device::cmd_set_depth_clip_negative_one_to_one_ext)
/// calls.
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthCompareOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html),
/// which [`vkCmdSetDepthCompareOp`](crate::Device::cmd_set_depth_compare_op) calls.
pub type PFN_vkCmdSetDepthCompareOp =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthCompareOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html),
/// which [`vkCmdSetDepthCompareOpEXT`](crate::Device::cmd_set_depth_compare_op_ext) calls.
pub type PFN_vkCmdSetDepthCompareOpEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html),
/// which [`vkCmdSetDepthTestEnable`](crate::Device::cmd_set_depth_test_enable) calls.
pub type PFN_vkCmdSetDepthTestEnable =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html),
/// which [`vkCmdSetDepthTestEnableEXT`](crate::Device::cmd_set_depth_test_enable_ext) calls.
pub type PFN_vkCmdSetDepthTestEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthWriteEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html),
/// which [`vkCmdSetDepthWriteEnable`](crate::Device::cmd_set_depth_write_enable) calls.
pub type PFN_vkCmdSetDepthWriteEnable =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDepthWriteEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html),
/// which [`vkCmdSetDepthWriteEnableEXT`](crate::Device::cmd_set_depth_write_enable_ext) calls.
pub type PFN_vkCmdSetDepthWriteEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDescriptorBufferOffsets2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html),
/// which
/// [`vkCmdSetDescriptorBufferOffsets2EXT`](crate::Device::cmd_set_descriptor_buffer_offsets2_ext)
/// calls.
pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDescriptorBufferOffsetsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html),
/// which
/// [`vkCmdSetDescriptorBufferOffsetsEXT`](crate::Device::cmd_set_descriptor_buffer_offsets_ext)
/// calls.
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        set_count: u32,
        p_buffer_indices: *const u32,
        p_offsets: *const DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDeviceMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html),
/// which [`vkCmdSetDeviceMask`](crate::Device::cmd_set_device_mask) calls.
pub type PFN_vkCmdSetDeviceMask =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDeviceMaskKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html),
/// which [`vkCmdSetDeviceMaskKHR`](crate::Device::cmd_set_device_mask_khr) calls.
pub type PFN_vkCmdSetDeviceMaskKHR =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetDiscardRectangleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html),
/// which [`vkCmdSetDiscardRectangleEXT`](crate::Device::cmd_set_discard_rectangle_ext) calls.
pub type PFN_vkCmdSetDiscardRectangleEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_discard_rectangle: u32,
        discard_rectangle_count: u32,
        p_discard_rectangles: *const Rect2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDiscardRectangleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html),
/// which
/// [`vkCmdSetDiscardRectangleEnableEXT`](crate::Device::cmd_set_discard_rectangle_enable_ext)
/// calls.
pub type PFN_vkCmdSetDiscardRectangleEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDiscardRectangleModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html),
/// which [`vkCmdSetDiscardRectangleModeEXT`](crate::Device::cmd_set_discard_rectangle_mode_ext)
/// calls.
pub type PFN_vkCmdSetDiscardRectangleModeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        discard_rectangle_mode: DiscardRectangleModeEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetDispatchParametersARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html),
/// which [`vkCmdSetDispatchParametersARM`](crate::Device::cmd_set_dispatch_parameters_arm)
/// calls.
pub type PFN_vkCmdSetDispatchParametersARM = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dispatch_parameters: *const DispatchParametersARM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html),
/// which [`vkCmdSetEvent`](crate::Device::cmd_set_event) calls.
pub type PFN_vkCmdSetEvent = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetEvent2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html),
/// which [`vkCmdSetEvent2`](crate::Device::cmd_set_event2) calls.
pub type PFN_vkCmdSetEvent2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetEvent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html),
/// which [`vkCmdSetEvent2KHR`](crate::Device::cmd_set_event2_khr) calls.
pub type PFN_vkCmdSetEvent2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event: Event,
        p_dependency_info: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetExclusiveScissorEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html),
/// which
/// [`vkCmdSetExclusiveScissorEnableNV`](crate::Device::cmd_set_exclusive_scissor_enable_nv)
/// calls.
pub type PFN_vkCmdSetExclusiveScissorEnableNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissor_enables: *const Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetExclusiveScissorNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html),
/// which [`vkCmdSetExclusiveScissorNV`](crate::Device::cmd_set_exclusive_scissor_nv) calls.
pub type PFN_vkCmdSetExclusiveScissorNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_exclusive_scissor: u32,
        exclusive_scissor_count: u32,
        p_exclusive_scissors: *const Rect2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetExtraPrimitiveOverestimationSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html),
/// which
/// [`vkCmdSetExtraPrimitiveOverestimationSizeEXT`](crate::Device::cmd_set_extra_primitive_overestimation_size_ext)
/// calls.
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetFragmentShadingRateEnumNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html),
/// which
/// [`vkCmdSetFragmentShadingRateEnumNV`](crate::Device::cmd_set_fragment_shading_rate_enum_nv)
/// calls.
pub type PFN_vkCmdSetFragmentShadingRateEnumNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetFragmentShadingRateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html),
/// which [`vkCmdSetFragmentShadingRateKHR`](crate::Device::cmd_set_fragment_shading_rate_khr)
/// calls.
pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_fragment_size: *const Extent2D,
        combiner_ops: *const [FragmentShadingRateCombinerOpKHR; 2],
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetFrontFace`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html),
/// which [`vkCmdSetFrontFace`](crate::Device::cmd_set_front_face) calls.
pub type PFN_vkCmdSetFrontFace =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace)>;

/// The type of a pointer to the C command
/// [`vkCmdSetFrontFaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html),
/// which [`vkCmdSetFrontFaceEXT`](crate::Device::cmd_set_front_face_ext) calls.
pub type PFN_vkCmdSetFrontFaceEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace)>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineRasterizationModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html),
/// which
/// [`vkCmdSetLineRasterizationModeEXT`](crate::Device::cmd_set_line_rasterization_mode_ext)
/// calls.
pub type PFN_vkCmdSetLineRasterizationModeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationMode,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineStipple`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html),
/// which [`vkCmdSetLineStipple`](crate::Device::cmd_set_line_stipple) calls.
pub type PFN_vkCmdSetLineStipple = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineStippleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html),
/// which [`vkCmdSetLineStippleEXT`](crate::Device::cmd_set_line_stipple_ext) calls.
pub type PFN_vkCmdSetLineStippleEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineStippleEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html),
/// which [`vkCmdSetLineStippleEnableEXT`](crate::Device::cmd_set_line_stipple_enable_ext)
/// calls.
pub type PFN_vkCmdSetLineStippleEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineStippleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html),
/// which [`vkCmdSetLineStippleKHR`](crate::Device::cmd_set_line_stipple_khr) calls.
pub type PFN_vkCmdSetLineStippleKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetLineWidth`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html),
/// which [`vkCmdSetLineWidth`](crate::Device::cmd_set_line_width) calls.
pub type PFN_vkCmdSetLineWidth =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetLogicOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html),
/// which [`vkCmdSetLogicOpEXT`](crate::Device::cmd_set_logic_op_ext) calls.
pub type PFN_vkCmdSetLogicOpEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op: LogicOp)>;

/// The type of a pointer to the C command
/// [`vkCmdSetLogicOpEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html),
/// which [`vkCmdSetLogicOpEnableEXT`](crate::Device::cmd_set_logic_op_enable_ext) calls.
pub type PFN_vkCmdSetLogicOpEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetPatchControlPointsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html),
/// which [`vkCmdSetPatchControlPointsEXT`](crate::Device::cmd_set_patch_control_points_ext)
/// calls.
pub type PFN_vkCmdSetPatchControlPointsEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, patch_control_points: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetPerformanceMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html),
/// which [`vkCmdSetPerformanceMarkerINTEL`](crate::Device::cmd_set_performance_marker_intel)
/// calls.
pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceMarkerInfoINTEL<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPerformanceOverrideINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html),
/// which
/// [`vkCmdSetPerformanceOverrideINTEL`](crate::Device::cmd_set_performance_override_intel)
/// calls.
pub type PFN_vkCmdSetPerformanceOverrideINTEL = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_override_info: *const PerformanceOverrideInfoINTEL<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPerformanceStreamMarkerINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html),
/// which
/// [`vkCmdSetPerformanceStreamMarkerINTEL`](crate::Device::cmd_set_performance_stream_marker_intel)
/// calls.
pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_marker_info: *const PerformanceStreamMarkerInfoINTEL<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPolygonModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html),
/// which [`vkCmdSetPolygonModeEXT`](crate::Device::cmd_set_polygon_mode_ext) calls.
pub type PFN_vkCmdSetPolygonModeEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode)>;

/// The type of a pointer to the C command
/// [`vkCmdSetPrimitiveRestartEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html),
/// which [`vkCmdSetPrimitiveRestartEnable`](crate::Device::cmd_set_primitive_restart_enable)
/// calls.
pub type PFN_vkCmdSetPrimitiveRestartEnable = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPrimitiveRestartEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html),
/// which
/// [`vkCmdSetPrimitiveRestartEnableEXT`](crate::Device::cmd_set_primitive_restart_enable_ext)
/// calls.
pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPrimitiveRestartIndexEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html),
/// which
/// [`vkCmdSetPrimitiveRestartIndexEXT`](crate::Device::cmd_set_primitive_restart_index_ext)
/// calls.
pub type PFN_vkCmdSetPrimitiveRestartIndexEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_index: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetPrimitiveTopology`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html),
/// which [`vkCmdSetPrimitiveTopology`](crate::Device::cmd_set_primitive_topology) calls.
pub type PFN_vkCmdSetPrimitiveTopology = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetPrimitiveTopologyEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html),
/// which [`vkCmdSetPrimitiveTopologyEXT`](crate::Device::cmd_set_primitive_topology_ext) calls.
pub type PFN_vkCmdSetPrimitiveTopologyEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetProvokingVertexModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html),
/// which [`vkCmdSetProvokingVertexModeEXT`](crate::Device::cmd_set_provoking_vertex_mode_ext)
/// calls.
pub type PFN_vkCmdSetProvokingVertexModeEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRasterizationSamplesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html),
/// which [`vkCmdSetRasterizationSamplesEXT`](crate::Device::cmd_set_rasterization_samples_ext)
/// calls.
pub type PFN_vkCmdSetRasterizationSamplesEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRasterizationStreamEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html),
/// which [`vkCmdSetRasterizationStreamEXT`](crate::Device::cmd_set_rasterization_stream_ext)
/// calls.
pub type PFN_vkCmdSetRasterizationStreamEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetRasterizerDiscardEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html),
/// which [`vkCmdSetRasterizerDiscardEnable`](crate::Device::cmd_set_rasterizer_discard_enable)
/// calls.
pub type PFN_vkCmdSetRasterizerDiscardEnable = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRasterizerDiscardEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html),
/// which
/// [`vkCmdSetRasterizerDiscardEnableEXT`](crate::Device::cmd_set_rasterizer_discard_enable_ext)
/// calls.
pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRayTracingPipelineStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html),
/// which
/// [`vkCmdSetRayTracingPipelineStackSizeKHR`](crate::Device::cmd_set_ray_tracing_pipeline_stack_size_khr)
/// calls.
pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, pipeline_stack_size: u32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetRenderingAttachmentLocations`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html),
/// which
/// [`vkCmdSetRenderingAttachmentLocations`](crate::Device::cmd_set_rendering_attachment_locations)
/// calls.
pub type PFN_vkCmdSetRenderingAttachmentLocations = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRenderingAttachmentLocationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html),
/// which
/// [`vkCmdSetRenderingAttachmentLocationsKHR`](crate::Device::cmd_set_rendering_attachment_locations_khr)
/// calls.
pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_location_info: *const RenderingAttachmentLocationInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRenderingInputAttachmentIndices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html),
/// which
/// [`vkCmdSetRenderingInputAttachmentIndices`](crate::Device::cmd_set_rendering_input_attachment_indices)
/// calls.
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRenderingInputAttachmentIndicesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html),
/// which
/// [`vkCmdSetRenderingInputAttachmentIndicesKHR`](crate::Device::cmd_set_rendering_input_attachment_indices_khr)
/// calls.
pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetRepresentativeFragmentTestEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html),
/// which
/// [`vkCmdSetRepresentativeFragmentTestEnableNV`](crate::Device::cmd_set_representative_fragment_test_enable_nv)
/// calls.
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetSampleLocationsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html),
/// which [`vkCmdSetSampleLocationsEXT`](crate::Device::cmd_set_sample_locations_ext) calls.
pub type PFN_vkCmdSetSampleLocationsEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_sample_locations_info: *const SampleLocationsInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetSampleLocationsEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html),
/// which
/// [`vkCmdSetSampleLocationsEnableEXT`](crate::Device::cmd_set_sample_locations_enable_ext)
/// calls.
pub type PFN_vkCmdSetSampleLocationsEnableEXT = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetSampleMaskEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html),
/// which [`vkCmdSetSampleMaskEXT`](crate::Device::cmd_set_sample_mask_ext) calls.
pub type PFN_vkCmdSetSampleMaskEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        p_sample_mask: *const SampleMask,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetScissor`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html),
/// which [`vkCmdSetScissor`](crate::Device::cmd_set_scissor) calls.
pub type PFN_vkCmdSetScissor = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetScissorWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html),
/// which [`vkCmdSetScissorWithCount`](crate::Device::cmd_set_scissor_with_count) calls.
pub type PFN_vkCmdSetScissorWithCount = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetScissorWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html),
/// which [`vkCmdSetScissorWithCountEXT`](crate::Device::cmd_set_scissor_with_count_ext) calls.
pub type PFN_vkCmdSetScissorWithCountEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        scissor_count: u32,
        p_scissors: *const Rect2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetShadingRateImageEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html),
/// which
/// [`vkCmdSetShadingRateImageEnableNV`](crate::Device::cmd_set_shading_rate_image_enable_nv)
/// calls.
pub type PFN_vkCmdSetShadingRateImageEnableNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilCompareMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html),
/// which [`vkCmdSetStencilCompareMask`](crate::Device::cmd_set_stencil_compare_mask) calls.
pub type PFN_vkCmdSetStencilCompareMask = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilOp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html),
/// which [`vkCmdSetStencilOp`](crate::Device::cmd_set_stencil_op) calls.
pub type PFN_vkCmdSetStencilOp = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilOpEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html),
/// which [`vkCmdSetStencilOpEXT`](crate::Device::cmd_set_stencil_op_ext) calls.
pub type PFN_vkCmdSetStencilOpEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilReference`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html),
/// which [`vkCmdSetStencilReference`](crate::Device::cmd_set_stencil_reference) calls.
pub type PFN_vkCmdSetStencilReference = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilTestEnable`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html),
/// which [`vkCmdSetStencilTestEnable`](crate::Device::cmd_set_stencil_test_enable) calls.
pub type PFN_vkCmdSetStencilTestEnable =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilTestEnableEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html),
/// which [`vkCmdSetStencilTestEnableEXT`](crate::Device::cmd_set_stencil_test_enable_ext)
/// calls.
pub type PFN_vkCmdSetStencilTestEnableEXT =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32)>;

/// The type of a pointer to the C command
/// [`vkCmdSetStencilWriteMask`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html),
/// which [`vkCmdSetStencilWriteMask`](crate::Device::cmd_set_stencil_write_mask) calls.
pub type PFN_vkCmdSetStencilWriteMask = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetTessellationDomainOriginEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html),
/// which
/// [`vkCmdSetTessellationDomainOriginEXT`](crate::Device::cmd_set_tessellation_domain_origin_ext)
/// calls.
pub type PFN_vkCmdSetTessellationDomainOriginEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetVertexInputEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html),
/// which [`vkCmdSetVertexInputEXT`](crate::Device::cmd_set_vertex_input_ext) calls.
pub type PFN_vkCmdSetVertexInputEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        vertex_binding_description_count: u32,
        p_vertex_binding_descriptions: *const VertexInputBindingDescription2EXT<'_>,
        vertex_attribute_description_count: u32,
        p_vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html),
/// which [`vkCmdSetViewport`](crate::Device::cmd_set_viewport) calls.
pub type PFN_vkCmdSetViewport = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportShadingRatePaletteNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html),
/// which
/// [`vkCmdSetViewportShadingRatePaletteNV`](crate::Device::cmd_set_viewport_shading_rate_palette_nv)
/// calls.
pub type PFN_vkCmdSetViewportShadingRatePaletteNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_shading_rate_palettes: *const ShadingRatePaletteNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportSwizzleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html),
/// which [`vkCmdSetViewportSwizzleNV`](crate::Device::cmd_set_viewport_swizzle_nv) calls.
pub type PFN_vkCmdSetViewportSwizzleNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_swizzles: *const ViewportSwizzleNV,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportWScalingEnableNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html),
/// which
/// [`vkCmdSetViewportWScalingEnableNV`](crate::Device::cmd_set_viewport_w_scaling_enable_nv)
/// calls.
pub type PFN_vkCmdSetViewportWScalingEnableNV = Option<
    unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportWScalingNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html),
/// which [`vkCmdSetViewportWScalingNV`](crate::Device::cmd_set_viewport_w_scaling_nv) calls.
pub type PFN_vkCmdSetViewportWScalingNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_count: u32,
        p_viewport_w_scalings: *const ViewportWScalingNV,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportWithCount`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html),
/// which [`vkCmdSetViewportWithCount`](crate::Device::cmd_set_viewport_with_count) calls.
pub type PFN_vkCmdSetViewportWithCount = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSetViewportWithCountEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html),
/// which [`vkCmdSetViewportWithCountEXT`](crate::Device::cmd_set_viewport_with_count_ext)
/// calls.
pub type PFN_vkCmdSetViewportWithCountEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        viewport_count: u32,
        p_viewports: *const Viewport,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdSubpassShadingHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html),
/// which [`vkCmdSubpassShadingHUAWEI`](crate::Device::cmd_subpass_shading_huawei) calls.
pub type PFN_vkCmdSubpassShadingHUAWEI =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer)>;

/// The type of a pointer to the C command
/// [`vkCmdTraceRaysIndirect2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html),
/// which [`vkCmdTraceRaysIndirect2KHR`](crate::Device::cmd_trace_rays_indirect2_khr) calls.
pub type PFN_vkCmdTraceRaysIndirect2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        indirect_device_address: DeviceAddress,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdTraceRaysIndirectKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html),
/// which [`vkCmdTraceRaysIndirectKHR`](crate::Device::cmd_trace_rays_indirect_khr) calls.
pub type PFN_vkCmdTraceRaysIndirectKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdTraceRaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html),
/// which [`vkCmdTraceRaysKHR`](crate::Device::cmd_trace_rays_khr) calls.
pub type PFN_vkCmdTraceRaysKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        p_callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdTraceRaysNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html),
/// which [`vkCmdTraceRaysNV`](crate::Device::cmd_trace_rays_nv) calls.
pub type PFN_vkCmdTraceRaysNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdUpdateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html),
/// which [`vkCmdUpdateBuffer`](crate::Device::cmd_update_buffer) calls.
pub type PFN_vkCmdUpdateBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data_size: DeviceSize,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdUpdateMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html),
/// which [`vkCmdUpdateMemoryKHR`](crate::Device::cmd_update_memory_khr) calls.
pub type PFN_vkCmdUpdateMemoryKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_dst_range: *const DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data_size: DeviceSize,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdUpdatePipelineIndirectBufferNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html),
/// which
/// [`vkCmdUpdatePipelineIndirectBufferNV`](crate::Device::cmd_update_pipeline_indirect_buffer_nv)
/// calls.
pub type PFN_vkCmdUpdatePipelineIndirectBufferNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWaitEvents`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html),
/// which [`vkCmdWaitEvents`](crate::Device::cmd_wait_events) calls.
pub type PFN_vkCmdWaitEvents = Option<
    unsafe extern "system" fn(
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
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWaitEvents2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html),
/// which [`vkCmdWaitEvents2`](crate::Device::cmd_wait_events2) calls.
pub type PFN_vkCmdWaitEvents2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWaitEvents2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html),
/// which [`vkCmdWaitEvents2KHR`](crate::Device::cmd_wait_events2_khr) calls.
pub type PFN_vkCmdWaitEvents2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        event_count: u32,
        p_events: *const Event,
        p_dependency_infos: *const DependencyInfo<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html),
/// which
/// [`vkCmdWriteAccelerationStructuresPropertiesKHR`](crate::Device::cmd_write_acceleration_structures_properties_khr)
/// calls.
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteAccelerationStructuresPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html),
/// which
/// [`vkCmdWriteAccelerationStructuresPropertiesNV`](crate::Device::cmd_write_acceleration_structures_properties_nv)
/// calls.
pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureNV,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteBufferMarker2AMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarker2AMD.html),
/// which [`vkCmdWriteBufferMarker2AMD`](crate::Device::cmd_write_buffer_marker2_amd) calls.
pub type PFN_vkCmdWriteBufferMarker2AMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteBufferMarkerAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteBufferMarkerAMD.html),
/// which [`vkCmdWriteBufferMarkerAMD`](crate::Device::cmd_write_buffer_marker_amd) calls.
pub type PFN_vkCmdWriteBufferMarkerAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteMarkerToMemoryAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html),
/// which [`vkCmdWriteMarkerToMemoryAMD`](crate::Device::cmd_write_marker_to_memory_amd) calls.
pub type PFN_vkCmdWriteMarkerToMemoryAMD = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_info: *const MemoryMarkerInfoAMD<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html),
/// which
/// [`vkCmdWriteMicromapsPropertiesEXT`](crate::Device::cmd_write_micromaps_properties_ext)
/// calls.
pub type PFN_vkCmdWriteMicromapsPropertiesEXT = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteTimestamp`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html),
/// which [`vkCmdWriteTimestamp`](crate::Device::cmd_write_timestamp) calls.
pub type PFN_vkCmdWriteTimestamp = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteTimestamp2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html),
/// which [`vkCmdWriteTimestamp2`](crate::Device::cmd_write_timestamp2) calls.
pub type PFN_vkCmdWriteTimestamp2 = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCmdWriteTimestamp2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html),
/// which [`vkCmdWriteTimestamp2KHR`](crate::Device::cmd_write_timestamp2_khr) calls.
pub type PFN_vkCmdWriteTimestamp2KHR = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkCompileDeferredNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html),
/// which [`vkCompileDeferredNV`](crate::Device::compile_deferred_nv) calls.
pub type PFN_vkCompileDeferredNV =
    Option<unsafe extern "system" fn(device: Device, pipeline: Pipeline, shader: u32) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkConvertCooperativeVectorMatrixNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html),
/// which
/// [`vkConvertCooperativeVectorMatrixNV`](crate::Device::convert_cooperative_vector_matrix_nv)
/// calls.
pub type PFN_vkConvertCooperativeVectorMatrixNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html),
/// which [`vkCopyAccelerationStructureKHR`](crate::Device::copy_acceleration_structure_khr)
/// calls.
pub type PFN_vkCopyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyAccelerationStructureToMemoryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html),
/// which
/// [`vkCopyAccelerationStructureToMemoryKHR`](crate::Device::copy_acceleration_structure_to_memory_khr)
/// calls.
pub type PFN_vkCopyAccelerationStructureToMemoryKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyImageToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html),
/// which [`vkCopyImageToImage`](crate::Device::copy_image_to_image) calls.
pub type PFN_vkCopyImageToImage = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyImageToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html),
/// which [`vkCopyImageToImageEXT`](crate::Device::copy_image_to_image_ext) calls.
pub type PFN_vkCopyImageToImageEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_image_info: *const CopyImageToImageInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyImageToMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html),
/// which [`vkCopyImageToMemory`](crate::Device::copy_image_to_memory) calls.
pub type PFN_vkCopyImageToMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyImageToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html),
/// which [`vkCopyImageToMemoryEXT`](crate::Device::copy_image_to_memory_ext) calls.
pub type PFN_vkCopyImageToMemoryEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_image_to_memory_info: *const CopyImageToMemoryInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMemoryToAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html),
/// which
/// [`vkCopyMemoryToAccelerationStructureKHR`](crate::Device::copy_memory_to_acceleration_structure_khr)
/// calls.
pub type PFN_vkCopyMemoryToAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMemoryToImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html),
/// which [`vkCopyMemoryToImage`](crate::Device::copy_memory_to_image) calls.
pub type PFN_vkCopyMemoryToImage = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMemoryToImageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html),
/// which [`vkCopyMemoryToImageEXT`](crate::Device::copy_memory_to_image_ext) calls.
pub type PFN_vkCopyMemoryToImageEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_copy_memory_to_image_info: *const CopyMemoryToImageInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMemoryToMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html),
/// which [`vkCopyMemoryToMicromapEXT`](crate::Device::copy_memory_to_micromap_ext) calls.
pub type PFN_vkCopyMemoryToMicromapEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMemoryToMicromapInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html),
/// which [`vkCopyMicromapEXT`](crate::Device::copy_micromap_ext) calls.
pub type PFN_vkCopyMicromapEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCopyMicromapToMemoryEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html),
/// which [`vkCopyMicromapToMemoryEXT`](crate::Device::copy_micromap_to_memory_ext) calls.
pub type PFN_vkCopyMicromapToMemoryEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        p_info: *const CopyMicromapToMemoryInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateAccelerationStructure2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html),
/// which
/// [`vkCreateAccelerationStructure2KHR`](crate::Device::create_acceleration_structure2_khr)
/// calls.
pub type PFN_vkCreateAccelerationStructure2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfo2KHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html),
/// which [`vkCreateAccelerationStructureKHR`](crate::Device::create_acceleration_structure_khr)
/// calls.
pub type PFN_vkCreateAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html),
/// which [`vkCreateAccelerationStructureNV`](crate::Device::create_acceleration_structure_nv)
/// calls.
pub type PFN_vkCreateAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const AccelerationStructureCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_acceleration_structure: *mut AccelerationStructureNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateAndroidSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html),
/// which [`vkCreateAndroidSurfaceKHR`](crate::Instance::create_android_surface_khr) calls.
#[cfg(feature = "android")]
pub type PFN_vkCreateAndroidSurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html),
/// which [`vkCreateBuffer`](crate::Device::create_buffer) calls.
pub type PFN_vkCreateBuffer = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_buffer: *mut Buffer,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html),
/// which [`vkCreateBufferCollectionFUCHSIA`](crate::Device::create_buffer_collection_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferCollectionCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_collection: *mut BufferCollectionFUCHSIA,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html),
/// which [`vkCreateBufferView`](crate::Device::create_buffer_view) calls.
pub type PFN_vkCreateBufferView = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const BufferViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut BufferView,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html),
/// which [`vkCreateCommandPool`](crate::Device::create_command_pool) calls.
pub type PFN_vkCreateCommandPool = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CommandPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_command_pool: *mut CommandPool,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateComputePipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html),
/// which [`vkCreateComputePipelines`](crate::Device::create_compute_pipelines) calls.
pub type PFN_vkCreateComputePipelines = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ComputePipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html),
/// which [`vkCreateCuFunctionNVX`](crate::Device::create_cu_function_nvx) calls.
pub type PFN_vkCreateCuFunctionNVX = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuFunctionCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CuFunctionNVX,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html),
/// which [`vkCreateCuModuleNVX`](crate::Device::create_cu_module_nvx) calls.
pub type PFN_vkCreateCuModuleNVX = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CuModuleCreateInfoNVX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CuModuleNVX,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html),
/// which [`vkCreateCudaFunctionNV`](crate::Device::create_cuda_function_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkCreateCudaFunctionNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CudaFunctionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_function: *mut CudaFunctionNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html),
/// which [`vkCreateCudaModuleNV`](crate::Device::create_cuda_module_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkCreateCudaModuleNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const CudaModuleCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_module: *mut CudaModuleNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html),
/// which
/// [`vkCreateDataGraphPipelineSessionARM`](crate::Device::create_data_graph_pipeline_session_arm)
/// calls.
pub type PFN_vkCreateDataGraphPipelineSessionARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DataGraphPipelineSessionCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut DataGraphPipelineSessionARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDataGraphPipelinesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html),
/// which [`vkCreateDataGraphPipelinesARM`](crate::Device::create_data_graph_pipelines_arm)
/// calls.
pub type PFN_vkCreateDataGraphPipelinesARM = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const DataGraphPipelineCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html),
/// which [`vkCreateDebugReportCallbackEXT`](crate::Instance::create_debug_report_callback_ext)
/// calls.
pub type PFN_vkCreateDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DebugReportCallbackCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_callback: *mut DebugReportCallbackEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html),
/// which [`vkCreateDebugUtilsMessengerEXT`](crate::Instance::create_debug_utils_messenger_ext)
/// calls.
pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DebugUtilsMessengerCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_messenger: *mut DebugUtilsMessengerEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html),
/// which [`vkCreateDeferredOperationKHR`](crate::Device::create_deferred_operation_khr) calls.
pub type PFN_vkCreateDeferredOperationKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_allocator: *const AllocationCallbacks<'_>,
        p_deferred_operation: *mut DeferredOperationKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html),
/// which [`vkCreateDescriptorPool`](crate::Device::create_descriptor_pool) calls.
pub type PFN_vkCreateDescriptorPool = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_pool: *mut DescriptorPool,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html),
/// which [`vkCreateDescriptorSetLayout`](crate::Device::create_descriptor_set_layout) calls.
pub type PFN_vkCreateDescriptorSetLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_set_layout: *mut DescriptorSetLayout,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html),
/// which [`vkCreateDescriptorUpdateTemplate`](crate::Device::create_descriptor_update_template)
/// calls.
pub type PFN_vkCreateDescriptorUpdateTemplate = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html),
/// which
/// [`vkCreateDescriptorUpdateTemplateKHR`](crate::Device::create_descriptor_update_template_khr)
/// calls.
pub type PFN_vkCreateDescriptorUpdateTemplateKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorUpdateTemplateCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_descriptor_update_template: *mut DescriptorUpdateTemplate,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html),
/// which [`vkCreateDevice`](crate::Instance::create_device) calls.
pub type PFN_vkCreateDevice = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_create_info: *const DeviceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_device: *mut Device,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDirectFBSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html),
/// which [`vkCreateDirectFBSurfaceEXT`](crate::Instance::create_direct_fb_surface_ext) calls.
#[cfg(feature = "directfb")]
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html),
/// which [`vkCreateDisplayModeKHR`](crate::Instance::create_display_mode_khr) calls.
pub type PFN_vkCreateDisplayModeKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_create_info: *const DisplayModeCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_mode: *mut DisplayModeKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateDisplayPlaneSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html),
/// which [`vkCreateDisplayPlaneSurfaceKHR`](crate::Instance::create_display_plane_surface_khr)
/// calls.
pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const DisplaySurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html),
/// which [`vkCreateEvent`](crate::Device::create_event) calls.
pub type PFN_vkCreateEvent = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const EventCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_event: *mut Event,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateExecutionGraphPipelinesAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html),
/// which
/// [`vkCreateExecutionGraphPipelinesAMDX`](crate::Device::create_execution_graph_pipelines_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html),
/// which [`vkCreateExternalComputeQueueNV`](crate::Device::create_external_compute_queue_nv)
/// calls.
pub type PFN_vkCreateExternalComputeQueueNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ExternalComputeQueueCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_external_queue: *mut ExternalComputeQueueNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html),
/// which [`vkCreateFence`](crate::Device::create_fence) calls.
pub type PFN_vkCreateFence = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const FenceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html),
/// which [`vkCreateFramebuffer`](crate::Device::create_framebuffer) calls.
pub type PFN_vkCreateFramebuffer = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const FramebufferCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_framebuffer: *mut Framebuffer,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html),
/// which [`vkCreateGpaSessionAMD`](crate::Device::create_gpa_session_amd) calls.
pub type PFN_vkCreateGpaSessionAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const GpaSessionCreateInfoAMD<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_gpa_session: *mut GpaSessionAMD,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateGraphicsPipelines`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html),
/// which [`vkCreateGraphicsPipelines`](crate::Device::create_graphics_pipelines) calls.
pub type PFN_vkCreateGraphicsPipelines = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateHeadlessSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html),
/// which [`vkCreateHeadlessSurfaceEXT`](crate::Instance::create_headless_surface_ext) calls.
pub type PFN_vkCreateHeadlessSurfaceEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateIOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIOSSurfaceMVK.html),
/// which [`vkCreateIOSSurfaceMVK`](crate::Instance::create_ios_surface_mvk) calls.
#[cfg(feature = "ios")]
pub type PFN_vkCreateIOSSurfaceMVK = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html),
/// which [`vkCreateImage`](crate::Device::create_image) calls.
pub type PFN_vkCreateImage = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ImageCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_image: *mut Image,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateImagePipeSurfaceFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html),
/// which
/// [`vkCreateImagePipeSurfaceFUCHSIA`](crate::Instance::create_image_pipe_surface_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html),
/// which [`vkCreateImageView`](crate::Device::create_image_view) calls.
pub type PFN_vkCreateImageView = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ImageViewCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut ImageView,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html),
/// which
/// [`vkCreateIndirectCommandsLayoutEXT`](crate::Device::create_indirect_commands_layout_ext)
/// calls.
pub type PFN_vkCreateIndirectCommandsLayoutEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html),
/// which
/// [`vkCreateIndirectCommandsLayoutNV`](crate::Device::create_indirect_commands_layout_nv)
/// calls.
pub type PFN_vkCreateIndirectCommandsLayoutNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html),
/// which [`vkCreateIndirectExecutionSetEXT`](crate::Device::create_indirect_execution_set_ext)
/// calls.
pub type PFN_vkCreateIndirectExecutionSetEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectExecutionSetCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_execution_set: *mut IndirectExecutionSetEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html),
/// which [`vkCreateInstance`](crate::Entry::create_instance) calls.
pub type PFN_vkCreateInstance = Option<
    unsafe extern "system" fn(
        p_create_info: *const InstanceCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instance: *mut Instance,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateMacOSSurfaceMVK`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html),
/// which [`vkCreateMacOSSurfaceMVK`](crate::Instance::create_mac_os_surface_mvk) calls.
#[cfg(feature = "macos")]
pub type PFN_vkCreateMacOSSurfaceMVK = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateMetalSurfaceEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMetalSurfaceEXT.html),
/// which [`vkCreateMetalSurfaceEXT`](crate::Instance::create_metal_surface_ext) calls.
#[cfg(feature = "metal")]
pub type PFN_vkCreateMetalSurfaceEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const MetalSurfaceCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html),
/// which [`vkCreateMicromapEXT`](crate::Device::create_micromap_ext) calls.
pub type PFN_vkCreateMicromapEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const MicromapCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_micromap: *mut MicromapEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html),
/// which [`vkCreateOpticalFlowSessionNV`](crate::Device::create_optical_flow_session_nv) calls.
pub type PFN_vkCreateOpticalFlowSessionNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const OpticalFlowSessionCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_session: *mut OpticalFlowSessionNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreatePipelineBinariesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html),
/// which [`vkCreatePipelineBinariesKHR`](crate::Device::create_pipeline_binaries_khr) calls.
pub type PFN_vkCreatePipelineBinariesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreatePipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html),
/// which [`vkCreatePipelineCache`](crate::Device::create_pipeline_cache) calls.
pub type PFN_vkCreatePipelineCache = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineCacheCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_cache: *mut PipelineCache,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreatePipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html),
/// which [`vkCreatePipelineLayout`](crate::Device::create_pipeline_layout) calls.
pub type PFN_vkCreatePipelineLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PipelineLayoutCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipeline_layout: *mut PipelineLayout,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreatePrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html),
/// which [`vkCreatePrivateDataSlot`](crate::Device::create_private_data_slot) calls.
pub type PFN_vkCreatePrivateDataSlot = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreatePrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html),
/// which [`vkCreatePrivateDataSlotEXT`](crate::Device::create_private_data_slot_ext) calls.
pub type PFN_vkCreatePrivateDataSlotEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const PrivateDataSlotCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_private_data_slot: *mut PrivateDataSlot,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html),
/// which [`vkCreateQueryPool`](crate::Device::create_query_pool) calls.
pub type PFN_vkCreateQueryPool = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const QueryPoolCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_query_pool: *mut QueryPool,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateRayTracingPipelinesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html),
/// which [`vkCreateRayTracingPipelinesKHR`](crate::Device::create_ray_tracing_pipelines_khr)
/// calls.
pub type PFN_vkCreateRayTracingPipelinesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateRayTracingPipelinesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html),
/// which [`vkCreateRayTracingPipelinesNV`](crate::Device::create_ray_tracing_pipelines_nv)
/// calls.
pub type PFN_vkCreateRayTracingPipelinesNV = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        p_create_infos: *const RayTracingPipelineCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_pipelines: *mut Pipeline,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html),
/// which [`vkCreateRenderPass`](crate::Device::create_render_pass) calls.
pub type PFN_vkCreateRenderPass = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateRenderPass2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html),
/// which [`vkCreateRenderPass2`](crate::Device::create_render_pass2) calls.
pub type PFN_vkCreateRenderPass2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo2<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateRenderPass2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html),
/// which [`vkCreateRenderPass2KHR`](crate::Device::create_render_pass2_khr) calls.
pub type PFN_vkCreateRenderPass2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const RenderPassCreateInfo2<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_render_pass: *mut RenderPass,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html),
/// which [`vkCreateSampler`](crate::Device::create_sampler) calls.
pub type PFN_vkCreateSampler = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_sampler: *mut Sampler,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html),
/// which [`vkCreateSamplerYcbcrConversion`](crate::Device::create_sampler_ycbcr_conversion)
/// calls.
pub type PFN_vkCreateSamplerYcbcrConversion = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html),
/// which
/// [`vkCreateSamplerYcbcrConversionKHR`](crate::Device::create_sampler_ycbcr_conversion_khr)
/// calls.
pub type PFN_vkCreateSamplerYcbcrConversionKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SamplerYcbcrConversionCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_ycbcr_conversion: *mut SamplerYcbcrConversion,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateScreenSurfaceQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html),
/// which [`vkCreateScreenSurfaceQNX`](crate::Instance::create_screen_surface_qnx) calls.
#[cfg(feature = "screen")]
pub type PFN_vkCreateScreenSurfaceQNX = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html),
/// which [`vkCreateSemaphore`](crate::Device::create_semaphore) calls.
pub type PFN_vkCreateSemaphore = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SemaphoreCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_semaphore: *mut Semaphore,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html),
/// which [`vkCreateShaderInstrumentationARM`](crate::Device::create_shader_instrumentation_arm)
/// calls.
pub type PFN_vkCreateShaderInstrumentationARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ShaderInstrumentationCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_instrumentation: *mut ShaderInstrumentationARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html),
/// which [`vkCreateShaderModule`](crate::Device::create_shader_module) calls.
pub type PFN_vkCreateShaderModule = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shader_module: *mut ShaderModule,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateShadersEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html),
/// which [`vkCreateShadersEXT`](crate::Device::create_shaders_ext) calls.
pub type PFN_vkCreateShadersEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        create_info_count: u32,
        p_create_infos: *const ShaderCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shaders: *mut ShaderEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSharedSwapchainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html),
/// which [`vkCreateSharedSwapchainsKHR`](crate::Device::create_shared_swapchains_khr) calls.
pub type PFN_vkCreateSharedSwapchainsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_create_infos: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchains: *mut SwapchainKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateStreamDescriptorSurfaceGGP`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateStreamDescriptorSurfaceGGP.html),
/// which
/// [`vkCreateStreamDescriptorSurfaceGGP`](crate::Instance::create_stream_descriptor_surface_ggp)
/// calls.
#[cfg(feature = "ggp")]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSurfaceOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSurfaceOHOS.html),
/// which [`vkCreateSurfaceOHOS`](crate::Instance::create_surface_ohos) calls.
#[cfg(feature = "ohos")]
pub type PFN_vkCreateSurfaceOHOS = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const SurfaceCreateInfoOHOS<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html),
/// which [`vkCreateSwapchainKHR`](crate::Device::create_swapchain_khr) calls.
pub type PFN_vkCreateSwapchainKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const SwapchainCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_swapchain: *mut SwapchainKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html),
/// which [`vkCreateTensorARM`](crate::Device::create_tensor_arm) calls.
pub type PFN_vkCreateTensorARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const TensorCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_tensor: *mut TensorARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html),
/// which [`vkCreateTensorViewARM`](crate::Device::create_tensor_view_arm) calls.
pub type PFN_vkCreateTensorViewARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const TensorViewCreateInfoARM<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_view: *mut TensorViewARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateUbmSurfaceSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html),
/// which [`vkCreateUbmSurfaceSEC`](crate::Instance::create_ubm_surface_sec) calls.
#[cfg(feature = "ubm")]
pub type PFN_vkCreateUbmSurfaceSEC = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const UbmSurfaceCreateInfoSEC<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html),
/// which [`vkCreateValidationCacheEXT`](crate::Device::create_validation_cache_ext) calls.
pub type PFN_vkCreateValidationCacheEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ValidationCacheCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_validation_cache: *mut ValidationCacheEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateViSurfaceNN`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html),
/// which [`vkCreateViSurfaceNN`](crate::Instance::create_vi_surface_nn) calls.
#[cfg(feature = "vi")]
pub type PFN_vkCreateViSurfaceNN = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const ViSurfaceCreateInfoNN<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html),
/// which [`vkCreateVideoSessionKHR`](crate::Device::create_video_session_khr) calls.
pub type PFN_vkCreateVideoSessionKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session: *mut VideoSessionKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html),
/// which
/// [`vkCreateVideoSessionParametersKHR`](crate::Device::create_video_session_parameters_khr)
/// calls.
pub type PFN_vkCreateVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_video_session_parameters: *mut VideoSessionParametersKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateWaylandSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html),
/// which [`vkCreateWaylandSurfaceKHR`](crate::Instance::create_wayland_surface_khr) calls.
#[cfg(feature = "wayland")]
pub type PFN_vkCreateWaylandSurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateWin32SurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html),
/// which [`vkCreateWin32SurfaceKHR`](crate::Instance::create_win32_surface_khr) calls.
#[cfg(feature = "win32")]
pub type PFN_vkCreateWin32SurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateXcbSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html),
/// which [`vkCreateXcbSurfaceKHR`](crate::Instance::create_xcb_surface_khr) calls.
#[cfg(feature = "xcb")]
pub type PFN_vkCreateXcbSurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkCreateXlibSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html),
/// which [`vkCreateXlibSurfaceKHR`](crate::Instance::create_xlib_surface_khr) calls.
#[cfg(feature = "xlib")]
pub type PFN_vkCreateXlibSurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_surface: *mut SurfaceKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkDebugMarkerSetObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html),
/// which [`vkDebugMarkerSetObjectNameEXT`](crate::Device::debug_marker_set_object_name_ext)
/// calls.
pub type PFN_vkDebugMarkerSetObjectNameEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugMarkerObjectNameInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkDebugMarkerSetObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html),
/// which [`vkDebugMarkerSetObjectTagEXT`](crate::Device::debug_marker_set_object_tag_ext)
/// calls.
pub type PFN_vkDebugMarkerSetObjectTagEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugMarkerObjectTagInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkDebugReportMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html),
/// which [`vkDebugReportMessageEXT`](crate::Instance::debug_report_message_ext) calls.
pub type PFN_vkDebugReportMessageEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        p_layer_prefix: *const c_char,
        p_message: *const c_char,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDeferredOperationJoinKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html),
/// which [`vkDeferredOperationJoinKHR`](crate::Device::deferred_operation_join_khr) calls.
pub type PFN_vkDeferredOperationJoinKHR =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkDestroyAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html),
/// which
/// [`vkDestroyAccelerationStructureKHR`](crate::Device::destroy_acceleration_structure_khr)
/// calls.
pub type PFN_vkDestroyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html),
/// which [`vkDestroyAccelerationStructureNV`](crate::Device::destroy_acceleration_structure_nv)
/// calls.
pub type PFN_vkDestroyAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html),
/// which [`vkDestroyBuffer`](crate::Device::destroy_buffer) calls.
pub type PFN_vkDestroyBuffer = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html),
/// which [`vkDestroyBufferCollectionFUCHSIA`](crate::Device::destroy_buffer_collection_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html),
/// which [`vkDestroyBufferView`](crate::Device::destroy_buffer_view) calls.
pub type PFN_vkDestroyBufferView = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer_view: BufferView,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html),
/// which [`vkDestroyCommandPool`](crate::Device::destroy_command_pool) calls.
pub type PFN_vkDestroyCommandPool = Option<
    unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html),
/// which [`vkDestroyCuFunctionNVX`](crate::Device::destroy_cu_function_nvx) calls.
pub type PFN_vkDestroyCuFunctionNVX = Option<
    unsafe extern "system" fn(
        device: Device,
        function: CuFunctionNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html),
/// which [`vkDestroyCuModuleNVX`](crate::Device::destroy_cu_module_nvx) calls.
pub type PFN_vkDestroyCuModuleNVX = Option<
    unsafe extern "system" fn(
        device: Device,
        module: CuModuleNVX,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html),
/// which [`vkDestroyCudaFunctionNV`](crate::Device::destroy_cuda_function_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkDestroyCudaFunctionNV = Option<
    unsafe extern "system" fn(
        device: Device,
        function: CudaFunctionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html),
/// which [`vkDestroyCudaModuleNV`](crate::Device::destroy_cuda_module_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkDestroyCudaModuleNV = Option<
    unsafe extern "system" fn(
        device: Device,
        module: CudaModuleNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html),
/// which
/// [`vkDestroyDataGraphPipelineSessionARM`](crate::Device::destroy_data_graph_pipeline_session_arm)
/// calls.
pub type PFN_vkDestroyDataGraphPipelineSessionARM = Option<
    unsafe extern "system" fn(
        device: Device,
        session: DataGraphPipelineSessionARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html),
/// which
/// [`vkDestroyDebugReportCallbackEXT`](crate::Instance::destroy_debug_report_callback_ext)
/// calls.
pub type PFN_vkDestroyDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        callback: DebugReportCallbackEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html),
/// which
/// [`vkDestroyDebugUtilsMessengerEXT`](crate::Instance::destroy_debug_utils_messenger_ext)
/// calls.
pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        messenger: DebugUtilsMessengerEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html),
/// which [`vkDestroyDeferredOperationKHR`](crate::Device::destroy_deferred_operation_khr)
/// calls.
pub type PFN_vkDestroyDeferredOperationKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        operation: DeferredOperationKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html),
/// which [`vkDestroyDescriptorPool`](crate::Device::destroy_descriptor_pool) calls.
pub type PFN_vkDestroyDescriptorPool = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html),
/// which [`vkDestroyDescriptorSetLayout`](crate::Device::destroy_descriptor_set_layout) calls.
pub type PFN_vkDestroyDescriptorSetLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html),
/// which
/// [`vkDestroyDescriptorUpdateTemplate`](crate::Device::destroy_descriptor_update_template)
/// calls.
pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDescriptorUpdateTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html),
/// which
/// [`vkDestroyDescriptorUpdateTemplateKHR`](crate::Device::destroy_descriptor_update_template_khr)
/// calls.
pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html),
/// which [`vkDestroyDevice`](crate::Device::destroy_device) calls.
pub type PFN_vkDestroyDevice =
    Option<unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks<'_>)>;

/// The type of a pointer to the C command
/// [`vkDestroyEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html),
/// which [`vkDestroyEvent`](crate::Device::destroy_event) calls.
pub type PFN_vkDestroyEvent = Option<
    unsafe extern "system" fn(
        device: Device,
        event: Event,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html),
/// which [`vkDestroyExternalComputeQueueNV`](crate::Device::destroy_external_compute_queue_nv)
/// calls.
pub type PFN_vkDestroyExternalComputeQueueNV = Option<
    unsafe extern "system" fn(
        device: Device,
        external_queue: ExternalComputeQueueNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyFence`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html),
/// which [`vkDestroyFence`](crate::Device::destroy_fence) calls.
pub type PFN_vkDestroyFence = Option<
    unsafe extern "system" fn(
        device: Device,
        fence: Fence,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html),
/// which [`vkDestroyFramebuffer`](crate::Device::destroy_framebuffer) calls.
pub type PFN_vkDestroyFramebuffer = Option<
    unsafe extern "system" fn(
        device: Device,
        framebuffer: Framebuffer,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html),
/// which [`vkDestroyGpaSessionAMD`](crate::Device::destroy_gpa_session_amd) calls.
pub type PFN_vkDestroyGpaSessionAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        gpa_session: GpaSessionAMD,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyImage`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html),
/// which [`vkDestroyImage`](crate::Device::destroy_image) calls.
pub type PFN_vkDestroyImage = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html),
/// which [`vkDestroyImageView`](crate::Device::destroy_image_view) calls.
pub type PFN_vkDestroyImageView = Option<
    unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html),
/// which
/// [`vkDestroyIndirectCommandsLayoutEXT`](crate::Device::destroy_indirect_commands_layout_ext)
/// calls.
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html),
/// which
/// [`vkDestroyIndirectCommandsLayoutNV`](crate::Device::destroy_indirect_commands_layout_nv)
/// calls.
pub type PFN_vkDestroyIndirectCommandsLayoutNV = Option<
    unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html),
/// which
/// [`vkDestroyIndirectExecutionSetEXT`](crate::Device::destroy_indirect_execution_set_ext)
/// calls.
pub type PFN_vkDestroyIndirectExecutionSetEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html),
/// which [`vkDestroyInstance`](crate::Instance::destroy_instance) calls.
pub type PFN_vkDestroyInstance = Option<
    unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks<'_>),
>;

/// The type of a pointer to the C command
/// [`vkDestroyMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html),
/// which [`vkDestroyMicromapEXT`](crate::Device::destroy_micromap_ext) calls.
pub type PFN_vkDestroyMicromapEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        micromap: MicromapEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html),
/// which [`vkDestroyOpticalFlowSessionNV`](crate::Device::destroy_optical_flow_session_nv)
/// calls.
pub type PFN_vkDestroyOpticalFlowSessionNV = Option<
    unsafe extern "system" fn(
        device: Device,
        session: OpticalFlowSessionNV,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html),
/// which [`vkDestroyPipeline`](crate::Device::destroy_pipeline) calls.
pub type PFN_vkDestroyPipeline = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html),
/// which [`vkDestroyPipelineBinaryKHR`](crate::Device::destroy_pipeline_binary_khr) calls.
pub type PFN_vkDestroyPipelineBinaryKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_binary: PipelineBinaryKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html),
/// which [`vkDestroyPipelineCache`](crate::Device::destroy_pipeline_cache) calls.
pub type PFN_vkDestroyPipelineCache = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html),
/// which [`vkDestroyPipelineLayout`](crate::Device::destroy_pipeline_layout) calls.
pub type PFN_vkDestroyPipelineLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_layout: PipelineLayout,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html),
/// which [`vkDestroyPrivateDataSlot`](crate::Device::destroy_private_data_slot) calls.
pub type PFN_vkDestroyPrivateDataSlot = Option<
    unsafe extern "system" fn(
        device: Device,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyPrivateDataSlotEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html),
/// which [`vkDestroyPrivateDataSlotEXT`](crate::Device::destroy_private_data_slot_ext) calls.
pub type PFN_vkDestroyPrivateDataSlotEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        private_data_slot: PrivateDataSlot,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html),
/// which [`vkDestroyQueryPool`](crate::Device::destroy_query_pool) calls.
pub type PFN_vkDestroyQueryPool = Option<
    unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html),
/// which [`vkDestroyRenderPass`](crate::Device::destroy_render_pass) calls.
pub type PFN_vkDestroyRenderPass = Option<
    unsafe extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySampler`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html),
/// which [`vkDestroySampler`](crate::Device::destroy_sampler) calls.
pub type PFN_vkDestroySampler = Option<
    unsafe extern "system" fn(
        device: Device,
        sampler: Sampler,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html),
/// which [`vkDestroySamplerYcbcrConversion`](crate::Device::destroy_sampler_ycbcr_conversion)
/// calls.
pub type PFN_vkDestroySamplerYcbcrConversion = Option<
    unsafe extern "system" fn(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySamplerYcbcrConversionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html),
/// which
/// [`vkDestroySamplerYcbcrConversionKHR`](crate::Device::destroy_sampler_ycbcr_conversion_khr)
/// calls.
pub type PFN_vkDestroySamplerYcbcrConversionKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        ycbcr_conversion: SamplerYcbcrConversion,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html),
/// which [`vkDestroySemaphore`](crate::Device::destroy_semaphore) calls.
pub type PFN_vkDestroySemaphore = Option<
    unsafe extern "system" fn(
        device: Device,
        semaphore: Semaphore,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html),
/// which [`vkDestroyShaderEXT`](crate::Device::destroy_shader_ext) calls.
pub type PFN_vkDestroyShaderEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        shader: ShaderEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html),
/// which
/// [`vkDestroyShaderInstrumentationARM`](crate::Device::destroy_shader_instrumentation_arm)
/// calls.
pub type PFN_vkDestroyShaderInstrumentationARM = Option<
    unsafe extern "system" fn(
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html),
/// which [`vkDestroyShaderModule`](crate::Device::destroy_shader_module) calls.
pub type PFN_vkDestroyShaderModule = Option<
    unsafe extern "system" fn(
        device: Device,
        shader_module: ShaderModule,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html),
/// which [`vkDestroySurfaceKHR`](crate::Instance::destroy_surface_khr) calls.
pub type PFN_vkDestroySurfaceKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        surface: SurfaceKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroySwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html),
/// which [`vkDestroySwapchainKHR`](crate::Device::destroy_swapchain_khr) calls.
pub type PFN_vkDestroySwapchainKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html),
/// which [`vkDestroyTensorARM`](crate::Device::destroy_tensor_arm) calls.
pub type PFN_vkDestroyTensorARM = Option<
    unsafe extern "system" fn(
        device: Device,
        tensor: TensorARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html),
/// which [`vkDestroyTensorViewARM`](crate::Device::destroy_tensor_view_arm) calls.
pub type PFN_vkDestroyTensorViewARM = Option<
    unsafe extern "system" fn(
        device: Device,
        tensor_view: TensorViewARM,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html),
/// which [`vkDestroyValidationCacheEXT`](crate::Device::destroy_validation_cache_ext) calls.
pub type PFN_vkDestroyValidationCacheEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html),
/// which [`vkDestroyVideoSessionKHR`](crate::Device::destroy_video_session_khr) calls.
pub type PFN_vkDestroyVideoSessionKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDestroyVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html),
/// which
/// [`vkDestroyVideoSessionParametersKHR`](crate::Device::destroy_video_session_parameters_khr)
/// calls.
pub type PFN_vkDestroyVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkDeviceWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html),
/// which [`vkDeviceWaitIdle`](crate::Device::device_wait_idle) calls.
pub type PFN_vkDeviceWaitIdle = Option<unsafe extern "system" fn(device: Device) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkDisplayPowerControlEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html),
/// which [`vkDisplayPowerControlEXT`](crate::Device::display_power_control_ext) calls.
pub type PFN_vkDisplayPowerControlEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        display: DisplayKHR,
        p_display_power_info: *const DisplayPowerInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEndCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html),
/// which [`vkEndCommandBuffer`](crate::Device::end_command_buffer) calls.
pub type PFN_vkEndCommandBuffer =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkEnumerateDeviceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html),
/// which
/// [`vkEnumerateDeviceExtensionProperties`](crate::Instance::enumerate_device_extension_properties)
/// calls.
pub type PFN_vkEnumerateDeviceExtensionProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumerateDeviceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html),
/// which
/// [`vkEnumerateDeviceLayerProperties`](crate::Instance::enumerate_device_layer_properties)
/// calls.
pub type PFN_vkEnumerateDeviceLayerProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumerateInstanceExtensionProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html),
/// which
/// [`vkEnumerateInstanceExtensionProperties`](crate::Entry::enumerate_instance_extension_properties)
/// calls.
pub type PFN_vkEnumerateInstanceExtensionProperties = Option<
    unsafe extern "system" fn(
        p_layer_name: *const c_char,
        p_property_count: *mut u32,
        p_properties: *mut ExtensionProperties,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumerateInstanceLayerProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html),
/// which
/// [`vkEnumerateInstanceLayerProperties`](crate::Entry::enumerate_instance_layer_properties)
/// calls.
pub type PFN_vkEnumerateInstanceLayerProperties = Option<
    unsafe extern "system" fn(
        p_property_count: *mut u32,
        p_properties: *mut LayerProperties,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumerateInstanceVersion`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html),
/// which [`vkEnumerateInstanceVersion`](crate::Entry::enumerate_instance_version) calls.
pub type PFN_vkEnumerateInstanceVersion =
    Option<unsafe extern "system" fn(p_api_version: *mut u32) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDeviceGroups`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html),
/// which [`vkEnumeratePhysicalDeviceGroups`](crate::Instance::enumerate_physical_device_groups)
/// calls.
pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDeviceGroupsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html),
/// which
/// [`vkEnumeratePhysicalDeviceGroupsKHR`](crate::Instance::enumerate_physical_device_groups_khr)
/// calls.
pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_group_count: *mut u32,
        p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html),
/// which
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM`](crate::Instance::enumerate_physical_device_queue_family_performance_counters_by_region_arm)
/// calls.
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterARM<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html),
/// which
/// [`vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`](crate::Instance::enumerate_physical_device_queue_family_performance_query_counters_khr)
/// calls.
pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR<'_>,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html),
/// which
/// [`vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM`](crate::Instance::enumerate_physical_device_shader_instrumentation_metrics_arm)
/// calls.
pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_description_count: *mut u32,
        p_descriptions: *mut ShaderInstrumentationMetricDescriptionARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkEnumeratePhysicalDevices`](https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html),
/// which [`vkEnumeratePhysicalDevices`](crate::Instance::enumerate_physical_devices) calls.
pub type PFN_vkEnumeratePhysicalDevices = Option<
    unsafe extern "system" fn(
        instance: Instance,
        p_physical_device_count: *mut u32,
        p_physical_devices: *mut PhysicalDevice,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkExportMetalObjectsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html),
/// which [`vkExportMetalObjectsEXT`](crate::Device::export_metal_objects_ext) calls.
#[cfg(feature = "metal")]
pub type PFN_vkExportMetalObjectsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_metal_objects_info: *mut ExportMetalObjectsInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkFlushMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html),
/// which [`vkFlushMappedMemoryRanges`](crate::Device::flush_mapped_memory_ranges) calls.
pub type PFN_vkFlushMappedMemoryRanges = Option<
    unsafe extern "system" fn(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkFreeCommandBuffers`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html),
/// which [`vkFreeCommandBuffers`](crate::Device::free_command_buffers) calls.
pub type PFN_vkFreeCommandBuffers = Option<
    unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        p_command_buffers: *const CommandBuffer,
    ),
>;

/// The type of a pointer to the C command
/// [`vkFreeDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html),
/// which [`vkFreeDescriptorSets`](crate::Device::free_descriptor_sets) calls.
pub type PFN_vkFreeDescriptorSets = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        p_descriptor_sets: *const DescriptorSet,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkFreeMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html),
/// which [`vkFreeMemory`](crate::Device::free_memory) calls.
pub type PFN_vkFreeMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_allocator: *const AllocationCallbacks<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetAccelerationStructureBuildSizesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html),
/// which
/// [`vkGetAccelerationStructureBuildSizesKHR`](crate::Device::get_acceleration_structure_build_sizes_khr)
/// calls.
pub type PFN_vkGetAccelerationStructureBuildSizesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        p_max_primitive_counts: *const u32,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetAccelerationStructureDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html),
/// which
/// [`vkGetAccelerationStructureDeviceAddressKHR`](crate::Device::get_acceleration_structure_device_address_khr)
/// calls.
pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureDeviceAddressInfoKHR<'_>,
    ) -> DeviceAddress,
>;

/// The type of a pointer to the C command
/// [`vkGetAccelerationStructureHandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html),
/// which
/// [`vkGetAccelerationStructureHandleNV`](crate::Device::get_acceleration_structure_handle_nv)
/// calls.
pub type PFN_vkGetAccelerationStructureHandleNV = Option<
    unsafe extern "system" fn(
        device: Device,
        acceleration_structure: AccelerationStructureNV,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetAccelerationStructureMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html),
/// which
/// [`vkGetAccelerationStructureMemoryRequirementsNV`](crate::Device::get_acceleration_structure_memory_requirements_nv)
/// calls.
pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html),
/// which
/// [`vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT`](crate::Device::get_acceleration_structure_opaque_capture_descriptor_data_ext)
/// calls.
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetAndroidHardwareBufferPropertiesANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html),
/// which
/// [`vkGetAndroidHardwareBufferPropertiesANDROID`](crate::Device::get_android_hardware_buffer_properties_android)
/// calls.
#[cfg(feature = "android")]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: *const AHardwareBuffer,
        p_properties: *mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferCollectionPropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html),
/// which
/// [`vkGetBufferCollectionPropertiesFUCHSIA`](crate::Device::get_buffer_collection_properties_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_properties: *mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferDeviceAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html),
/// which [`vkGetBufferDeviceAddress`](crate::Device::get_buffer_device_address) calls.
pub type PFN_vkGetBufferDeviceAddress = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferDeviceAddressEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html),
/// which [`vkGetBufferDeviceAddressEXT`](crate::Device::get_buffer_device_address_ext) calls.
pub type PFN_vkGetBufferDeviceAddressEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferDeviceAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html),
/// which [`vkGetBufferDeviceAddressKHR`](crate::Device::get_buffer_device_address_khr) calls.
pub type PFN_vkGetBufferDeviceAddressKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferDeviceAddressInfo<'_>,
    ) -> DeviceAddress,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html),
/// which [`vkGetBufferMemoryRequirements`](crate::Device::get_buffer_memory_requirements)
/// calls.
pub type PFN_vkGetBufferMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: Buffer,
        p_memory_requirements: *mut MemoryRequirements,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetBufferMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html),
/// which [`vkGetBufferMemoryRequirements2`](crate::Device::get_buffer_memory_requirements2)
/// calls.
pub type PFN_vkGetBufferMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetBufferMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html),
/// which
/// [`vkGetBufferMemoryRequirements2KHR`](crate::Device::get_buffer_memory_requirements2_khr)
/// calls.
pub type PFN_vkGetBufferMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetBufferOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html),
/// which [`vkGetBufferOpaqueCaptureAddress`](crate::Device::get_buffer_opaque_capture_address)
/// calls.
pub type PFN_vkGetBufferOpaqueCaptureAddress = Option<
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo<'_>) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html),
/// which
/// [`vkGetBufferOpaqueCaptureAddressKHR`](crate::Device::get_buffer_opaque_capture_address_khr)
/// calls.
pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = Option<
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo<'_>) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetBufferOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html),
/// which
/// [`vkGetBufferOpaqueCaptureDescriptorDataEXT`](crate::Device::get_buffer_opaque_capture_descriptor_data_ext)
/// calls.
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetCalibratedTimestampsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html),
/// which [`vkGetCalibratedTimestampsEXT`](crate::Device::get_calibrated_timestamps_ext) calls.
pub type PFN_vkGetCalibratedTimestampsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetCalibratedTimestampsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html),
/// which [`vkGetCalibratedTimestampsKHR`](crate::Device::get_calibrated_timestamps_khr) calls.
pub type PFN_vkGetCalibratedTimestampsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoKHR<'_>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetClusterAccelerationStructureBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html),
/// which
/// [`vkGetClusterAccelerationStructureBuildSizesNV`](crate::Device::get_cluster_acceleration_structure_build_sizes_nv)
/// calls.
pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ClusterAccelerationStructureInputInfoNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetCudaModuleCacheNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html),
/// which [`vkGetCudaModuleCacheNV`](crate::Device::get_cuda_module_cache_nv) calls.
#[cfg(feature = "beta")]
pub type PFN_vkGetCudaModuleCacheNV = Option<
    unsafe extern "system" fn(
        device: Device,
        module: CudaModuleNV,
        p_cache_size: *mut usize,
        p_cache_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDataGraphPipelineAvailablePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html),
/// which
/// [`vkGetDataGraphPipelineAvailablePropertiesARM`](crate::Device::get_data_graph_pipeline_available_properties_arm)
/// calls.
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
        p_properties_count: *mut u32,
        p_properties: *mut DataGraphPipelinePropertyARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDataGraphPipelinePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html),
/// which
/// [`vkGetDataGraphPipelinePropertiesARM`](crate::Device::get_data_graph_pipeline_properties_arm)
/// calls.
pub type PFN_vkGetDataGraphPipelinePropertiesARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
        properties_count: u32,
        p_properties: *mut DataGraphPipelinePropertyQueryResultARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDataGraphPipelineSessionBindPointRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html),
/// which
/// [`vkGetDataGraphPipelineSessionBindPointRequirementsARM`](crate::Device::get_data_graph_pipeline_session_bind_point_requirements_arm)
/// calls.
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
        p_bind_point_requirement_count: *mut u32,
        p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDataGraphPipelineSessionMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html),
/// which
/// [`vkGetDataGraphPipelineSessionMemoryRequirementsARM`](crate::Device::get_data_graph_pipeline_session_memory_requirements_arm)
/// calls.
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeferredOperationMaxConcurrencyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html),
/// which
/// [`vkGetDeferredOperationMaxConcurrencyKHR`](crate::Device::get_deferred_operation_max_concurrency_khr)
/// calls.
pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> u32>;

/// The type of a pointer to the C command
/// [`vkGetDeferredOperationResultKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html),
/// which [`vkGetDeferredOperationResultKHR`](crate::Device::get_deferred_operation_result_khr)
/// calls.
pub type PFN_vkGetDeferredOperationResultKHR =
    Option<unsafe extern "system" fn(device: Device, operation: DeferredOperationKHR) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html),
/// which [`vkGetDescriptorEXT`](crate::Device::get_descriptor_ext) calls.
pub type PFN_vkGetDescriptorEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
        data_size: usize,
        p_descriptor: *mut c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetHostMappingVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html),
/// which
/// [`vkGetDescriptorSetHostMappingVALVE`](crate::Device::get_descriptor_set_host_mapping_valve)
/// calls.
pub type PFN_vkGetDescriptorSetHostMappingVALVE = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        pp_data: *mut *mut c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetLayoutBindingOffsetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html),
/// which
/// [`vkGetDescriptorSetLayoutBindingOffsetEXT`](crate::Device::get_descriptor_set_layout_binding_offset_ext)
/// calls.
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        layout: DescriptorSetLayout,
        binding: u32,
        p_offset: *mut DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetLayoutHostMappingInfoVALVE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html),
/// which
/// [`vkGetDescriptorSetLayoutHostMappingInfoVALVE`](crate::Device::get_descriptor_set_layout_host_mapping_info_valve)
/// calls.
pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = Option<
    unsafe extern "system" fn(
        device: Device,
        p_binding_reference: *const DescriptorSetBindingReferenceVALVE<'_>,
        p_host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetLayoutSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html),
/// which [`vkGetDescriptorSetLayoutSizeEXT`](crate::Device::get_descriptor_set_layout_size_ext)
/// calls.
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        layout: DescriptorSetLayout,
        p_layout_size_in_bytes: *mut DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetLayoutSupport`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html),
/// which [`vkGetDescriptorSetLayoutSupport`](crate::Device::get_descriptor_set_layout_support)
/// calls.
pub type PFN_vkGetDescriptorSetLayoutSupport = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDescriptorSetLayoutSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html),
/// which
/// [`vkGetDescriptorSetLayoutSupportKHR`](crate::Device::get_descriptor_set_layout_support_khr)
/// calls.
pub type PFN_vkGetDescriptorSetLayoutSupportKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
        p_support: *mut DescriptorSetLayoutSupport<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceAccelerationStructureCompatibilityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html),
/// which
/// [`vkGetDeviceAccelerationStructureCompatibilityKHR`](crate::Device::get_device_acceleration_structure_compatibility_khr)
/// calls.
pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_version_info: *const AccelerationStructureVersionInfoKHR<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceBufferMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html),
/// which
/// [`vkGetDeviceBufferMemoryRequirements`](crate::Device::get_device_buffer_memory_requirements)
/// calls.
pub type PFN_vkGetDeviceBufferMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceBufferMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html),
/// which
/// [`vkGetDeviceBufferMemoryRequirementsKHR`](crate::Device::get_device_buffer_memory_requirements_khr)
/// calls.
pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceBufferMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceCombinedImageSamplerIndexNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html),
/// which
/// [`vkGetDeviceCombinedImageSamplerIndexNVX`](crate::Device::get_device_combined_image_sampler_index_nvx)
/// calls.
pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX = Option<
    unsafe extern "system" fn(device: Device, image_view_index: u64, sampler_index: u64) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceFaultDebugInfoKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html),
/// which [`vkGetDeviceFaultDebugInfoKHR`](crate::Device::get_device_fault_debug_info_khr)
/// calls.
pub type PFN_vkGetDeviceFaultDebugInfoKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_debug_info: *mut DeviceFaultDebugInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceFaultInfoEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html),
/// which [`vkGetDeviceFaultInfoEXT`](crate::Device::get_device_fault_info_ext) calls.
pub type PFN_vkGetDeviceFaultInfoEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_fault_counts: *mut DeviceFaultCountsEXT<'_>,
        p_fault_info: *mut DeviceFaultInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceFaultReportsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html),
/// which [`vkGetDeviceFaultReportsKHR`](crate::Device::get_device_fault_reports_khr) calls.
pub type PFN_vkGetDeviceFaultReportsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        timeout: u64,
        p_fault_counts: *mut u32,
        p_fault_info: *mut DeviceFaultInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceGroupPeerMemoryFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html),
/// which
/// [`vkGetDeviceGroupPeerMemoryFeatures`](crate::Device::get_device_group_peer_memory_features)
/// calls.
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = Option<
    unsafe extern "system" fn(
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceGroupPeerMemoryFeaturesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html),
/// which
/// [`vkGetDeviceGroupPeerMemoryFeaturesKHR`](crate::Device::get_device_group_peer_memory_features_khr)
/// calls.
pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
        p_peer_memory_features: *mut PeerMemoryFeatureFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceGroupPresentCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html),
/// which
/// [`vkGetDeviceGroupPresentCapabilitiesKHR`](crate::Device::get_device_group_present_capabilities_khr)
/// calls.
pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceGroupSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html),
/// which
/// [`vkGetDeviceGroupSurfacePresentModes2EXT`](crate::Device::get_device_group_surface_present_modes2_ext)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceGroupSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html),
/// which
/// [`vkGetDeviceGroupSurfacePresentModesKHR`](crate::Device::get_device_group_surface_present_modes_khr)
/// calls.
pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        surface: SurfaceKHR,
        p_modes: *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html),
/// which
/// [`vkGetDeviceImageMemoryRequirements`](crate::Device::get_device_image_memory_requirements)
/// calls.
pub type PFN_vkGetDeviceImageMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html),
/// which
/// [`vkGetDeviceImageMemoryRequirementsKHR`](crate::Device::get_device_image_memory_requirements_khr)
/// calls.
pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html),
/// which
/// [`vkGetDeviceImageSparseMemoryRequirements`](crate::Device::get_device_image_sparse_memory_requirements)
/// calls.
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageSparseMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html),
/// which
/// [`vkGetDeviceImageSparseMemoryRequirementsKHR`](crate::Device::get_device_image_sparse_memory_requirements_khr)
/// calls.
pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageMemoryRequirements<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html),
/// which
/// [`vkGetDeviceImageSubresourceLayout`](crate::Device::get_device_image_subresource_layout)
/// calls.
pub type PFN_vkGetDeviceImageSubresourceLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceImageSubresourceLayoutKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html),
/// which
/// [`vkGetDeviceImageSubresourceLayoutKHR`](crate::Device::get_device_image_subresource_layout_khr)
/// calls.
pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceImageSubresourceInfo<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceMemoryCommitment`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html),
/// which [`vkGetDeviceMemoryCommitment`](crate::Device::get_device_memory_commitment) calls.
pub type PFN_vkGetDeviceMemoryCommitment = Option<
    unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        p_committed_memory_in_bytes: *mut DeviceSize,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceMemoryOpaqueCaptureAddress`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html),
/// which
/// [`vkGetDeviceMemoryOpaqueCaptureAddress`](crate::Device::get_device_memory_opaque_capture_address)
/// calls.
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html),
/// which
/// [`vkGetDeviceMemoryOpaqueCaptureAddressKHR`](crate::Device::get_device_memory_opaque_capture_address_khr)
/// calls.
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceMemoryOpaqueCaptureAddressInfo<'_>,
    ) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceMicromapCompatibilityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html),
/// which
/// [`vkGetDeviceMicromapCompatibilityEXT`](crate::Device::get_device_micromap_compatibility_ext)
/// calls.
pub type PFN_vkGetDeviceMicromapCompatibilityEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_version_info: *const MicromapVersionInfoEXT<'_>,
        p_compatibility: *mut AccelerationStructureCompatibilityKHR,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceProcAddr.html),
/// which [`vkGetDeviceProcAddr`](crate::Instance::get_device_proc_addr) calls.
pub type PFN_vkGetDeviceProcAddr =
    Option<unsafe extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction>;

/// The type of a pointer to the C command
/// [`vkGetDeviceQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html),
/// which [`vkGetDeviceQueue`](crate::Device::get_device_queue) calls.
pub type PFN_vkGetDeviceQueue = Option<
    unsafe extern "system" fn(
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        p_queue: *mut Queue,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceQueue2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html),
/// which [`vkGetDeviceQueue2`](crate::Device::get_device_queue2) calls.
pub type PFN_vkGetDeviceQueue2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_queue_info: *const DeviceQueueInfo2<'_>,
        p_queue: *mut Queue,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html),
/// which
/// [`vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI`](crate::Device::get_device_subpass_shading_max_workgroup_size_huawei)
/// calls.
pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = Option<
    unsafe extern "system" fn(
        device: Device,
        renderpass: RenderPass,
        p_max_workgroup_size: *mut Extent2D,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDeviceTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html),
/// which
/// [`vkGetDeviceTensorMemoryRequirementsARM`](crate::Device::get_device_tensor_memory_requirements_arm)
/// calls.
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetDisplayModeProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html),
/// which [`vkGetDisplayModeProperties2KHR`](crate::Instance::get_display_mode_properties2_khr)
/// calls.
pub type PFN_vkGetDisplayModeProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModeProperties2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDisplayModePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html),
/// which [`vkGetDisplayModePropertiesKHR`](crate::Instance::get_display_mode_properties_khr)
/// calls.
pub type PFN_vkGetDisplayModePropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
        p_property_count: *mut u32,
        p_properties: *mut DisplayModePropertiesKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDisplayPlaneCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html),
/// which
/// [`vkGetDisplayPlaneCapabilities2KHR`](crate::Instance::get_display_plane_capabilities2_khr)
/// calls.
pub type PFN_vkGetDisplayPlaneCapabilities2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_display_plane_info: *const DisplayPlaneInfo2KHR<'_>,
        p_capabilities: *mut DisplayPlaneCapabilities2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDisplayPlaneCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html),
/// which
/// [`vkGetDisplayPlaneCapabilitiesKHR`](crate::Instance::get_display_plane_capabilities_khr)
/// calls.
pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        mode: DisplayModeKHR,
        plane_index: u32,
        p_capabilities: *mut DisplayPlaneCapabilitiesKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDisplayPlaneSupportedDisplaysKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html),
/// which
/// [`vkGetDisplayPlaneSupportedDisplaysKHR`](crate::Instance::get_display_plane_supported_displays_khr)
/// calls.
pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        plane_index: u32,
        p_display_count: *mut u32,
        p_displays: *mut DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDrmDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html),
/// which [`vkGetDrmDisplayEXT`](crate::Instance::get_drm_display_ext) calls.
pub type PFN_vkGetDrmDisplayEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetDynamicRenderingTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html),
/// which
/// [`vkGetDynamicRenderingTilePropertiesQCOM`](crate::Device::get_dynamic_rendering_tile_properties_qcom)
/// calls.
pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_rendering_info: *const RenderingInfo<'_>,
        p_properties: *mut TilePropertiesQCOM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetEncodedVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html),
/// which
/// [`vkGetEncodedVideoSessionParametersKHR`](crate::Device::get_encoded_video_session_parameters_khr)
/// calls.
pub type PFN_vkGetEncodedVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
        p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetEventStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html),
/// which [`vkGetEventStatus`](crate::Device::get_event_status) calls.
pub type PFN_vkGetEventStatus =
    Option<unsafe extern "system" fn(device: Device, event: Event) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkGetExecutionGraphPipelineNodeIndexAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html),
/// which
/// [`vkGetExecutionGraphPipelineNodeIndexAMDX`](crate::Device::get_execution_graph_pipeline_node_index_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = Option<
    unsafe extern "system" fn(
        device: Device,
        execution_graph: Pipeline,
        p_node_info: *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
        p_node_index: *mut u32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetExecutionGraphPipelineScratchSizeAMDX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html),
/// which
/// [`vkGetExecutionGraphPipelineScratchSizeAMDX`](crate::Device::get_execution_graph_pipeline_scratch_size_amdx)
/// calls.
#[cfg(feature = "beta")]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = Option<
    unsafe extern "system" fn(
        device: Device,
        execution_graph: Pipeline,
        p_size_info: *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetExternalComputeQueueDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExternalComputeQueueDataNV.html),
/// which [`vkGetExternalComputeQueueDataNV`](crate::Device::get_external_compute_queue_data_nv)
/// calls.
pub type PFN_vkGetExternalComputeQueueDataNV = Option<
    unsafe extern "system" fn(
        external_queue: ExternalComputeQueueNV,
        params: *mut ExternalComputeQueueDataParamsNV<'_>,
        p_data: *mut c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html),
/// which [`vkGetFenceFdKHR`](crate::Device::get_fence_fd_khr) calls.
pub type PFN_vkGetFenceFdKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const FenceGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetFenceStatus`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html),
/// which [`vkGetFenceStatus`](crate::Device::get_fence_status) calls.
pub type PFN_vkGetFenceStatus =
    Option<unsafe extern "system" fn(device: Device, fence: Fence) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkGetFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html),
/// which [`vkGetFenceWin32HandleKHR`](crate::Device::get_fence_win32_handle_khr) calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetFenceWin32HandleKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetFramebufferTilePropertiesQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html),
/// which
/// [`vkGetFramebufferTilePropertiesQCOM`](crate::Device::get_framebuffer_tile_properties_qcom)
/// calls.
pub type PFN_vkGetFramebufferTilePropertiesQCOM = Option<
    unsafe extern "system" fn(
        device: Device,
        framebuffer: Framebuffer,
        p_properties_count: *mut u32,
        p_properties: *mut TilePropertiesQCOM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetGeneratedCommandsMemoryRequirementsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html),
/// which
/// [`vkGetGeneratedCommandsMemoryRequirementsEXT`](crate::Device::get_generated_commands_memory_requirements_ext)
/// calls.
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetGeneratedCommandsMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html),
/// which
/// [`vkGetGeneratedCommandsMemoryRequirementsNV`](crate::Device::get_generated_commands_memory_requirements_nv)
/// calls.
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetGpaDeviceClockInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html),
/// which [`vkGetGpaDeviceClockInfoAMD`](crate::Device::get_gpa_device_clock_info_amd) calls.
pub type PFN_vkGetGpaDeviceClockInfoAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *mut GpaDeviceGetClockInfoAMD<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetGpaSessionResultsAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html),
/// which [`vkGetGpaSessionResultsAMD`](crate::Device::get_gpa_session_results_amd) calls.
pub type PFN_vkGetGpaSessionResultsAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        p_size_in_bytes: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetGpaSessionStatusAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html),
/// which [`vkGetGpaSessionStatusAMD`](crate::Device::get_gpa_session_status_amd) calls.
pub type PFN_vkGetGpaSessionStatusAMD =
    Option<unsafe extern "system" fn(device: Device, gpa_session: GpaSessionAMD) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkGetImageDrmFormatModifierPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html),
/// which
/// [`vkGetImageDrmFormatModifierPropertiesEXT`](crate::Device::get_image_drm_format_modifier_properties_ext)
/// calls.
pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_properties: *mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetImageMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html),
/// which [`vkGetImageMemoryRequirements`](crate::Device::get_image_memory_requirements) calls.
pub type PFN_vkGetImageMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_memory_requirements: *mut MemoryRequirements,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html),
/// which [`vkGetImageMemoryRequirements2`](crate::Device::get_image_memory_requirements2)
/// calls.
pub type PFN_vkGetImageMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html),
/// which
/// [`vkGetImageMemoryRequirements2KHR`](crate::Device::get_image_memory_requirements2_khr)
/// calls.
pub type PFN_vkGetImageMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageMemoryRequirementsInfo2<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageOpaqueCaptureDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html),
/// which [`vkGetImageOpaqueCaptureDataEXT`](crate::Device::get_image_opaque_capture_data_ext)
/// calls.
pub type PFN_vkGetImageOpaqueCaptureDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        image_count: u32,
        p_images: *const Image,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetImageOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html),
/// which
/// [`vkGetImageOpaqueCaptureDescriptorDataEXT`](crate::Device::get_image_opaque_capture_descriptor_data_ext)
/// calls.
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetImageSparseMemoryRequirements`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html),
/// which
/// [`vkGetImageSparseMemoryRequirements`](crate::Device::get_image_sparse_memory_requirements)
/// calls.
pub type PFN_vkGetImageSparseMemoryRequirements = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSparseMemoryRequirements2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html),
/// which
/// [`vkGetImageSparseMemoryRequirements2`](crate::Device::get_image_sparse_memory_requirements2)
/// calls.
pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSparseMemoryRequirements2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html),
/// which
/// [`vkGetImageSparseMemoryRequirements2KHR`](crate::Device::get_image_sparse_memory_requirements2_khr)
/// calls.
pub type PFN_vkGetImageSparseMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageSparseMemoryRequirementsInfo2<'_>,
        p_sparse_memory_requirement_count: *mut u32,
        p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSubresourceLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html),
/// which [`vkGetImageSubresourceLayout`](crate::Device::get_image_subresource_layout) calls.
pub type PFN_vkGetImageSubresourceLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource,
        p_layout: *mut SubresourceLayout,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSubresourceLayout2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html),
/// which [`vkGetImageSubresourceLayout2`](crate::Device::get_image_subresource_layout2) calls.
pub type PFN_vkGetImageSubresourceLayout2 = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSubresourceLayout2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html),
/// which [`vkGetImageSubresourceLayout2EXT`](crate::Device::get_image_subresource_layout2_ext)
/// calls.
pub type PFN_vkGetImageSubresourceLayout2EXT = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageSubresourceLayout2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html),
/// which [`vkGetImageSubresourceLayout2KHR`](crate::Device::get_image_subresource_layout2_khr)
/// calls.
pub type PFN_vkGetImageSubresourceLayout2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        image: Image,
        p_subresource: *const ImageSubresource2<'_>,
        p_layout: *mut SubresourceLayout2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetImageViewAddressNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html),
/// which [`vkGetImageViewAddressNVX`](crate::Device::get_image_view_address_nvx) calls.
pub type PFN_vkGetImageViewAddressNVX = Option<
    unsafe extern "system" fn(
        device: Device,
        image_view: ImageView,
        p_properties: *mut ImageViewAddressPropertiesNVX<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetImageViewHandle64NVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html),
/// which [`vkGetImageViewHandle64NVX`](crate::Device::get_image_view_handle64_nvx) calls.
pub type PFN_vkGetImageViewHandle64NVX = Option<
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u64,
>;

/// The type of a pointer to the C command
/// [`vkGetImageViewHandleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html),
/// which [`vkGetImageViewHandleNVX`](crate::Device::get_image_view_handle_nvx) calls.
pub type PFN_vkGetImageViewHandleNVX = Option<
    unsafe extern "system" fn(device: Device, p_info: *const ImageViewHandleInfoNVX<'_>) -> u32,
>;

/// The type of a pointer to the C command
/// [`vkGetImageViewOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html),
/// which
/// [`vkGetImageViewOpaqueCaptureDescriptorDataEXT`](crate::Device::get_image_view_opaque_capture_descriptor_data_ext)
/// calls.
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetInstanceProcAddr`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html),
/// which [`vkGetInstanceProcAddr`](crate::Entry::get_instance_proc_addr) calls.
pub type PFN_vkGetInstanceProcAddr = Option<
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction,
>;

/// The type of a pointer to the C command
/// [`vkGetLatencyTimingsLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html),
/// which [`vkGetLatencyTimingsLegacyNV`](crate::Device::get_latency_timings_legacy_nv) calls.
pub type PFN_vkGetLatencyTimingsLegacyNV =
    Option<unsafe extern "system" fn(device: Device, p_timings: *mut c_void)>;

/// The type of a pointer to the C command
/// [`vkGetLatencyTimingsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html),
/// which [`vkGetLatencyTimingsNV`](crate::Device::get_latency_timings_nv) calls.
pub type PFN_vkGetLatencyTimingsNV = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryAndroidHardwareBufferANDROID`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html),
/// which
/// [`vkGetMemoryAndroidHardwareBufferANDROID`](crate::Device::get_memory_android_hardware_buffer_android)
/// calls.
#[cfg(feature = "android")]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
        p_buffer: *mut *mut AHardwareBuffer,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html),
/// which [`vkGetMemoryFdKHR`](crate::Device::get_memory_fd_khr) calls.
pub type PFN_vkGetMemoryFdKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryFdPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html),
/// which [`vkGetMemoryFdPropertiesKHR`](crate::Device::get_memory_fd_properties_khr) calls.
pub type PFN_vkGetMemoryFdPropertiesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: core::ffi::c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryHostPointerPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html),
/// which
/// [`vkGetMemoryHostPointerPropertiesEXT`](crate::Device::get_memory_host_pointer_properties_ext)
/// calls.
pub type PFN_vkGetMemoryHostPointerPropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_host_pointer: *const c_void,
        p_memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryMetalHandleEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html),
/// which [`vkGetMemoryMetalHandleEXT`](crate::Device::get_memory_metal_handle_ext) calls.
#[cfg(feature = "metal")]
pub type PFN_vkGetMemoryMetalHandleEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT<'_>,
        p_handle: *mut *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryMetalHandlePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html),
/// which
/// [`vkGetMemoryMetalHandlePropertiesEXT`](crate::Device::get_memory_metal_handle_properties_ext)
/// calls.
#[cfg(feature = "metal")]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        p_handle: *const c_void,
        p_memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryNativeBufferOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html),
/// which [`vkGetMemoryNativeBufferOHOS`](crate::Device::get_memory_native_buffer_ohos) calls.
#[cfg(feature = "ohos")]
pub type PFN_vkGetMemoryNativeBufferOHOS = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const MemoryGetNativeBufferInfoOHOS<'_>,
        p_buffer: *mut *mut OH_NativeBuffer,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryRemoteAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html),
/// which [`vkGetMemoryRemoteAddressNV`](crate::Device::get_memory_remote_address_nv) calls.
pub type PFN_vkGetMemoryRemoteAddressNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV<'_>,
        p_address: *mut RemoteAddressNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html),
/// which [`vkGetMemoryWin32HandleKHR`](crate::Device::get_memory_win32_handle_khr) calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandleKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryWin32HandleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html),
/// which [`vkGetMemoryWin32HandleNV`](crate::Device::get_memory_win32_handle_nv) calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandleNV = Option<
    unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryWin32HandlePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html),
/// which
/// [`vkGetMemoryWin32HandlePropertiesKHR`](crate::Device::get_memory_win32_handle_properties_khr)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html),
/// which [`vkGetMemoryZirconHandleFUCHSIA`](crate::Device::get_memory_zircon_handle_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMemoryZirconHandlePropertiesFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html),
/// which
/// [`vkGetMemoryZirconHandlePropertiesFUCHSIA`](crate::Device::get_memory_zircon_handle_properties_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        p_memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetMicromapBuildSizesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html),
/// which [`vkGetMicromapBuildSizesEXT`](crate::Device::get_micromap_build_sizes_ext) calls.
pub type PFN_vkGetMicromapBuildSizesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        p_build_info: *const MicromapBuildInfoEXT<'_>,
        p_size_info: *mut MicromapBuildSizesInfoEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetNativeBufferPropertiesOHOS`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html),
/// which [`vkGetNativeBufferPropertiesOHOS`](crate::Device::get_native_buffer_properties_ohos)
/// calls.
#[cfg(feature = "ohos")]
pub type PFN_vkGetNativeBufferPropertiesOHOS = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: *const OH_NativeBuffer,
        p_properties: *mut NativeBufferPropertiesOHOS<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPartitionedAccelerationStructuresBuildSizesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html),
/// which
/// [`vkGetPartitionedAccelerationStructuresBuildSizesNV`](crate::Device::get_partitioned_acceleration_structures_build_sizes_nv)
/// calls.
pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const PartitionedAccelerationStructureInstancesInputNV<'_>,
        p_size_info: *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPastPresentationTimingEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html),
/// which [`vkGetPastPresentationTimingEXT`](crate::Device::get_past_presentation_timing_ext)
/// calls.
pub type PFN_vkGetPastPresentationTimingEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
        p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPastPresentationTimingGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html),
/// which
/// [`vkGetPastPresentationTimingGOOGLE`](crate::Device::get_past_presentation_timing_google)
/// calls.
pub type PFN_vkGetPastPresentationTimingGOOGLE = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPerformanceParameterINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html),
/// which [`vkGetPerformanceParameterINTEL`](crate::Device::get_performance_parameter_intel)
/// calls.
pub type PFN_vkGetPerformanceParameterINTEL = Option<
    unsafe extern "system" fn(
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
        p_value: *mut PerformanceValueINTEL<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html),
/// which
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsEXT`](crate::Instance::get_physical_device_calibrateable_time_domains_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html),
/// which
/// [`vkGetPhysicalDeviceCalibrateableTimeDomainsKHR`](crate::Instance::get_physical_device_calibrateable_time_domains_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html),
/// which
/// [`vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV`](crate::Instance::get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html),
/// which
/// [`vkGetPhysicalDeviceCooperativeMatrixProperties2EXT`](crate::Instance::get_physical_device_cooperative_matrix_properties2_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixProperties2EXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_cooperative_matrix_info: *const PhysicalDeviceCooperativeMatrixInfo2EXT<'_>,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixProperties2EXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR`](crate::Instance::get_physical_device_cooperative_matrix_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesNV.html),
/// which
/// [`vkGetPhysicalDeviceCooperativeMatrixPropertiesNV`](crate::Instance::get_physical_device_cooperative_matrix_properties_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeMatrixPropertiesNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html),
/// which
/// [`vkGetPhysicalDeviceCooperativeVectorPropertiesNV`](crate::Instance::get_physical_device_cooperative_vector_properties_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut CooperativeVectorPropertiesNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDescriptorSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html),
/// which
/// [`vkGetPhysicalDeviceDescriptorSizeEXT`](crate::Instance::get_physical_device_descriptor_size_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> DeviceSize,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html),
/// which
/// [`vkGetPhysicalDeviceDirectFBPresentationSupportEXT`](crate::Instance::get_physical_device_direct_fb_presentation_support_ext)
/// calls.
#[cfg(feature = "directfb")]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html),
/// which
/// [`vkGetPhysicalDeviceDisplayPlaneProperties2KHR`](crate::Instance::get_physical_device_display_plane_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlaneProperties2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceDisplayPlanePropertiesKHR`](crate::Instance::get_physical_device_display_plane_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPlanePropertiesKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDisplayProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html),
/// which
/// [`vkGetPhysicalDeviceDisplayProperties2KHR`](crate::Instance::get_physical_device_display_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayProperties2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceDisplayPropertiesKHR`](crate::Instance::get_physical_device_display_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_property_count: *mut u32,
        p_properties: *mut DisplayPropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalBufferProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalBufferProperties`](crate::Instance::get_physical_device_external_buffer_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalBufferPropertiesKHR`](crate::Instance::get_physical_device_external_buffer_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo<'_>,
        p_external_buffer_properties: *mut ExternalBufferProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalFenceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalFenceProperties`](crate::Instance::get_physical_device_external_fence_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalFencePropertiesKHR`](crate::Instance::get_physical_device_external_fence_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_fence_info: *const PhysicalDeviceExternalFenceInfo<'_>,
        p_external_fence_properties: *mut ExternalFenceProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html),
/// which
/// [`vkGetPhysicalDeviceExternalImageFormatPropertiesNV`](crate::Instance::get_physical_device_external_image_format_properties_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalSemaphoreProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalSemaphoreProperties`](crate::Instance::get_physical_device_external_semaphore_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html),
/// which
/// [`vkGetPhysicalDeviceExternalSemaphorePropertiesKHR`](crate::Instance::get_physical_device_external_semaphore_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        p_external_semaphore_properties: *mut ExternalSemaphoreProperties<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceExternalTensorPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html),
/// which
/// [`vkGetPhysicalDeviceExternalTensorPropertiesARM`](crate::Instance::get_physical_device_external_tensor_properties_arm)
/// calls.
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
        p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFeatures`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html),
/// which [`vkGetPhysicalDeviceFeatures`](crate::Instance::get_physical_device_features) calls.
pub type PFN_vkGetPhysicalDeviceFeatures = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFeatures2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html),
/// which [`vkGetPhysicalDeviceFeatures2`](crate::Instance::get_physical_device_features2)
/// calls.
pub type PFN_vkGetPhysicalDeviceFeatures2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFeatures2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html),
/// which
/// [`vkGetPhysicalDeviceFeatures2KHR`](crate::Instance::get_physical_device_features2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceFeatures2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_features: *mut PhysicalDeviceFeatures2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html),
/// which
/// [`vkGetPhysicalDeviceFormatProperties`](crate::Instance::get_physical_device_format_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceFormatProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceFormatProperties2`](crate::Instance::get_physical_device_format_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceFormatProperties2KHR`](crate::Instance::get_physical_device_format_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        p_format_properties: *mut FormatProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceFragmentShadingRatesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html),
/// which
/// [`vkGetPhysicalDeviceFragmentShadingRatesKHR`](crate::Instance::get_physical_device_fragment_shading_rates_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_fragment_shading_rate_count: *mut u32,
        p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html),
/// which
/// [`vkGetPhysicalDeviceImageFormatProperties`](crate::Instance::get_physical_device_image_format_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        p_image_format_properties: *mut ImageFormatProperties,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceImageFormatProperties2`](crate::Instance::get_physical_device_image_format_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
        p_image_format_properties: *mut ImageFormatProperties2<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceImageFormatProperties2KHR`](crate::Instance::get_physical_device_image_format_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_image_format_info: *const PhysicalDeviceImageFormatInfo2<'_>,
        p_image_format_properties: *mut ImageFormatProperties2<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceMemoryProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html),
/// which
/// [`vkGetPhysicalDeviceMemoryProperties`](crate::Instance::get_physical_device_memory_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceMemoryProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html),
/// which
/// [`vkGetPhysicalDeviceMemoryProperties2`](crate::Instance::get_physical_device_memory_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceMemoryProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html),
/// which
/// [`vkGetPhysicalDeviceMemoryProperties2KHR`](crate::Instance::get_physical_device_memory_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_memory_properties: *mut PhysicalDeviceMemoryProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceMultisamplePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html),
/// which
/// [`vkGetPhysicalDeviceMultisamplePropertiesEXT`](crate::Instance::get_physical_device_multisample_properties_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
        p_multisample_properties: *mut MultisamplePropertiesEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html),
/// which
/// [`vkGetPhysicalDeviceOpticalFlowImageFormatsNV`](crate::Instance::get_physical_device_optical_flow_image_formats_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV<'_>,
        p_format_count: *mut u32,
        p_image_format_properties: *mut OpticalFlowImageFormatPropertiesNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDevicePresentRectanglesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html),
/// which
/// [`vkGetPhysicalDevicePresentRectanglesKHR`](crate::Instance::get_physical_device_present_rectangles_khr)
/// calls.
pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_rect_count: *mut u32,
        p_rects: *mut Rect2D,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html),
/// which [`vkGetPhysicalDeviceProperties`](crate::Instance::get_physical_device_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html),
/// which [`vkGetPhysicalDeviceProperties2`](crate::Instance::get_physical_device_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html),
/// which
/// [`vkGetPhysicalDeviceProperties2KHR`](crate::Instance::get_physical_device_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_properties: *mut PhysicalDeviceProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM`](crate::Instance::get_physical_device_queue_family_data_graph_engine_operation_properties_arm)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
        p_properties: *mut BaseOutStructure<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM`](crate::Instance::get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_properties: *const QueueFamilyDataGraphPropertiesARM<'_>,
        p_optical_flow_image_format_info: *const DataGraphOpticalFlowImageFormatInfoARM<'_>,
        p_format_count: *mut u32,
        p_image_format_properties: *mut DataGraphOpticalFlowImageFormatPropertiesARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM`](crate::Instance::get_physical_device_queue_family_data_graph_processing_engine_properties_arm)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = Option<unsafe extern "system" fn(physical_device: PhysicalDevice, p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>, p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>)>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM`](crate::Instance::get_physical_device_queue_family_data_graph_properties_arm)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_property_count: *mut u32,
        p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`](crate::Instance::get_physical_device_queue_family_performance_query_passes_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR<'_>,
        p_num_passes: *mut u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyProperties`](crate::Instance::get_physical_device_queue_family_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyProperties2`](crate::Instance::get_physical_device_queue_family_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html),
/// which
/// [`vkGetPhysicalDeviceQueueFamilyProperties2KHR`](crate::Instance::get_physical_device_queue_family_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_queue_family_property_count: *mut u32,
        p_queue_family_properties: *mut QueueFamilyProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceScreenPresentationSupportQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html),
/// which
/// [`vkGetPhysicalDeviceScreenPresentationSupportQNX`](crate::Instance::get_physical_device_screen_presentation_support_qnx)
/// calls.
#[cfg(feature = "screen")]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSparseImageFormatProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html),
/// which
/// [`vkGetPhysicalDeviceSparseImageFormatProperties`](crate::Instance::get_physical_device_sparse_image_format_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2`](crate::Instance::get_physical_device_sparse_image_format_properties2)
/// calls.
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html),
/// which
/// [`vkGetPhysicalDeviceSparseImageFormatProperties2KHR`](crate::Instance::get_physical_device_sparse_image_format_properties2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_format_info: *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        p_property_count: *mut u32,
        p_properties: *mut SparseImageFormatProperties2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html),
/// which
/// [`vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV`](crate::Instance::get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)
/// calls.
pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_combination_count: *mut u32,
        p_combinations: *mut FramebufferMixedSamplesCombinationNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceCapabilities2EXT`](crate::Instance::get_physical_device_surface_capabilities2_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilities2EXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceCapabilities2KHR`](crate::Instance::get_physical_device_surface_capabilities2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_surface_capabilities: *mut SurfaceCapabilities2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceCapabilitiesKHR`](crate::Instance::get_physical_device_surface_capabilities_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_capabilities: *mut SurfaceCapabilitiesKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceFormats2KHR`](crate::Instance::get_physical_device_surface_formats2_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormat2KHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceFormatsKHR`](crate::Instance::get_physical_device_surface_formats_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_surface_format_count: *mut u32,
        p_surface_formats: *mut SurfaceFormatKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfacePresentModes2EXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html),
/// which
/// [`vkGetPhysicalDeviceSurfacePresentModes2EXT`](crate::Instance::get_physical_device_surface_present_modes2_ext)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_surface_info: *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfacePresentModesKHR`](crate::Instance::get_physical_device_surface_present_modes_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
        p_present_mode_count: *mut u32,
        p_present_modes: *mut PresentModeKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceSurfaceSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html),
/// which
/// [`vkGetPhysicalDeviceSurfaceSupportKHR`](crate::Instance::get_physical_device_surface_support_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        surface: SurfaceKHR,
        p_supported: *mut Bool32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceToolProperties`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html),
/// which
/// [`vkGetPhysicalDeviceToolProperties`](crate::Instance::get_physical_device_tool_properties)
/// calls.
pub type PFN_vkGetPhysicalDeviceToolProperties = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceToolPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html),
/// which
/// [`vkGetPhysicalDeviceToolPropertiesEXT`](crate::Instance::get_physical_device_tool_properties_ext)
/// calls.
pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_tool_count: *mut u32,
        p_tool_properties: *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceUbmPresentationSupportSEC`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html),
/// which
/// [`vkGetPhysicalDeviceUbmPresentationSupportSEC`](crate::Instance::get_physical_device_ubm_presentation_support_sec)
/// calls.
#[cfg(feature = "ubm")]
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceVideoCapabilitiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceVideoCapabilitiesKHR`](crate::Instance::get_physical_device_video_capabilities_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_profile: *const VideoProfileInfoKHR<'_>,
        p_capabilities: *mut VideoCapabilitiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR`](crate::Instance::get_physical_device_video_encode_quality_level_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceVideoFormatPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html),
/// which
/// [`vkGetPhysicalDeviceVideoFormatPropertiesKHR`](crate::Instance::get_physical_device_video_format_properties_khr)
/// calls.
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
        p_video_format_property_count: *mut u32,
        p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html),
/// which
/// [`vkGetPhysicalDeviceWaylandPresentationSupportKHR`](crate::Instance::get_physical_device_wayland_presentation_support_khr)
/// calls.
#[cfg(feature = "wayland")]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html),
/// which
/// [`vkGetPhysicalDeviceWin32PresentationSupportKHR`](crate::Instance::get_physical_device_win32_presentation_support_khr)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR = Option<
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html),
/// which
/// [`vkGetPhysicalDeviceXcbPresentationSupportKHR`](crate::Instance::get_physical_device_xcb_presentation_support_khr)
/// calls.
#[cfg(feature = "xcb")]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html),
/// which
/// [`vkGetPhysicalDeviceXlibPresentationSupportKHR`](crate::Instance::get_physical_device_xlib_presentation_support_khr)
/// calls.
#[cfg(feature = "xlib")]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineBinaryDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html),
/// which [`vkGetPipelineBinaryDataKHR`](crate::Device::get_pipeline_binary_data_khr) calls.
pub type PFN_vkGetPipelineBinaryDataKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const PipelineBinaryDataInfoKHR<'_>,
        p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
        p_pipeline_binary_data_size: *mut usize,
        p_pipeline_binary_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineCacheData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html),
/// which [`vkGetPipelineCacheData`](crate::Device::get_pipeline_cache_data) calls.
pub type PFN_vkGetPipelineCacheData = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline_cache: PipelineCache,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineExecutableInternalRepresentationsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html),
/// which
/// [`vkGetPipelineExecutableInternalRepresentationsKHR`](crate::Device::get_pipeline_executable_internal_representations_khr)
/// calls.
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineExecutablePropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html),
/// which
/// [`vkGetPipelineExecutablePropertiesKHR`](crate::Device::get_pipeline_executable_properties_khr)
/// calls.
pub type PFN_vkGetPipelineExecutablePropertiesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_executable_count: *mut u32,
        p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineExecutableStatisticsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html),
/// which
/// [`vkGetPipelineExecutableStatisticsKHR`](crate::Device::get_pipeline_executable_statistics_khr)
/// calls.
pub type PFN_vkGetPipelineExecutableStatisticsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_statistic_count: *mut u32,
        p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineIndirectDeviceAddressNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html),
/// which
/// [`vkGetPipelineIndirectDeviceAddressNV`](crate::Device::get_pipeline_indirect_device_address_nv)
/// calls.
pub type PFN_vkGetPipelineIndirectDeviceAddressNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const PipelineIndirectDeviceAddressInfoNV<'_>,
    ) -> DeviceAddress,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineIndirectMemoryRequirementsNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html),
/// which
/// [`vkGetPipelineIndirectMemoryRequirementsNV`](crate::Device::get_pipeline_indirect_memory_requirements_nv)
/// calls.
pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ComputePipelineCreateInfo<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPipelineKeyKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html),
/// which [`vkGetPipelineKeyKHR`](crate::Device::get_pipeline_key_khr) calls.
pub type PFN_vkGetPipelineKeyKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
        p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPipelinePropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html),
/// which [`vkGetPipelinePropertiesEXT`](crate::Device::get_pipeline_properties_ext) calls.
pub type PFN_vkGetPipelinePropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_pipeline_info: *const PipelineInfoKHR<'_>,
        p_pipeline_properties: *mut BaseOutStructure<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html),
/// which [`vkGetPrivateData`](crate::Device::get_private_data) calls.
pub type PFN_vkGetPrivateData = Option<
    unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html),
/// which [`vkGetPrivateDataEXT`](crate::Device::get_private_data_ext) calls.
pub type PFN_vkGetPrivateDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        p_data: *mut u64,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetQueryPoolResults`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html),
/// which [`vkGetQueryPoolResults`](crate::Device::get_query_pool_results) calls.
pub type PFN_vkGetQueryPoolResults = Option<
    unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: usize,
        p_data: *mut c_void,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetQueueCheckpointData2NV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html),
/// which [`vkGetQueueCheckpointData2NV`](crate::Device::get_queue_checkpoint_data2_nv) calls.
pub type PFN_vkGetQueueCheckpointData2NV = Option<
    unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointData2NV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetQueueCheckpointDataNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html),
/// which [`vkGetQueueCheckpointDataNV`](crate::Device::get_queue_checkpoint_data_nv) calls.
pub type PFN_vkGetQueueCheckpointDataNV = Option<
    unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointDataNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetRandROutputDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html),
/// which [`vkGetRandROutputDisplayEXT`](crate::Instance::get_rand_r_output_display_ext) calls.
#[cfg(feature = "xlib-xrandr")]
pub type PFN_vkGetRandROutputDisplayEXT = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html),
/// which
/// [`vkGetRayTracingCaptureReplayShaderGroupHandlesKHR`](crate::Device::get_ray_tracing_capture_replay_shader_group_handles_khr)
/// calls.
pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetRayTracingShaderGroupHandlesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html),
/// which
/// [`vkGetRayTracingShaderGroupHandlesKHR`](crate::Device::get_ray_tracing_shader_group_handles_khr)
/// calls.
pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetRayTracingShaderGroupHandlesNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html),
/// which
/// [`vkGetRayTracingShaderGroupHandlesNV`](crate::Device::get_ray_tracing_shader_group_handles_nv)
/// calls.
pub type PFN_vkGetRayTracingShaderGroupHandlesNV = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data_size: usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetRayTracingShaderGroupStackSizeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html),
/// which
/// [`vkGetRayTracingShaderGroupStackSizeKHR`](crate::Device::get_ray_tracing_shader_group_stack_size_khr)
/// calls.
pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize,
>;

/// The type of a pointer to the C command
/// [`vkGetRefreshCycleDurationGOOGLE`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html),
/// which [`vkGetRefreshCycleDurationGOOGLE`](crate::Device::get_refresh_cycle_duration_google)
/// calls.
pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetRenderAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html),
/// which [`vkGetRenderAreaGranularity`](crate::Device::get_render_area_granularity) calls.
pub type PFN_vkGetRenderAreaGranularity = Option<
    unsafe extern "system" fn(
        device: Device,
        render_pass: RenderPass,
        p_granularity: *mut Extent2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetRenderingAreaGranularity`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html),
/// which [`vkGetRenderingAreaGranularity`](crate::Device::get_rendering_area_granularity)
/// calls.
pub type PFN_vkGetRenderingAreaGranularity = Option<
    unsafe extern "system" fn(
        device: Device,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetRenderingAreaGranularityKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html),
/// which
/// [`vkGetRenderingAreaGranularityKHR`](crate::Device::get_rendering_area_granularity_khr)
/// calls.
pub type PFN_vkGetRenderingAreaGranularityKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_rendering_area_info: *const RenderingAreaInfo<'_>,
        p_granularity: *mut Extent2D,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetSamplerOpaqueCaptureDescriptorDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html),
/// which
/// [`vkGetSamplerOpaqueCaptureDescriptorDataEXT`](crate::Device::get_sampler_opaque_capture_descriptor_data_ext)
/// calls.
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetScreenBufferPropertiesQNX`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html),
/// which [`vkGetScreenBufferPropertiesQNX`](crate::Device::get_screen_buffer_properties_qnx)
/// calls.
#[cfg(feature = "screen")]
pub type PFN_vkGetScreenBufferPropertiesQNX = Option<
    unsafe extern "system" fn(
        device: Device,
        buffer: *const _screen_buffer,
        p_properties: *mut ScreenBufferPropertiesQNX<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSemaphoreCounterValue`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html),
/// which [`vkGetSemaphoreCounterValue`](crate::Device::get_semaphore_counter_value) calls.
pub type PFN_vkGetSemaphoreCounterValue = Option<
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSemaphoreCounterValueKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html),
/// which [`vkGetSemaphoreCounterValueKHR`](crate::Device::get_semaphore_counter_value_khr)
/// calls.
pub type PFN_vkGetSemaphoreCounterValueKHR = Option<
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html),
/// which [`vkGetSemaphoreFdKHR`](crate::Device::get_semaphore_fd_khr) calls.
pub type PFN_vkGetSemaphoreFdKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
        p_fd: *mut core::ffi::c_int,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html),
/// which [`vkGetSemaphoreWin32HandleKHR`](crate::Device::get_semaphore_win32_handle_khr) calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html),
/// which
/// [`vkGetSemaphoreZirconHandleFUCHSIA`](crate::Device::get_semaphore_zircon_handle_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetShaderBinaryDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html),
/// which [`vkGetShaderBinaryDataEXT`](crate::Device::get_shader_binary_data_ext) calls.
pub type PFN_vkGetShaderBinaryDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        shader: ShaderEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetShaderInfoAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html),
/// which [`vkGetShaderInfoAMD`](crate::Device::get_shader_info_amd) calls.
pub type PFN_vkGetShaderInfoAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlagBits,
        info_type: ShaderInfoTypeAMD,
        p_info_size: *mut usize,
        p_info: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetShaderInstrumentationValuesARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html),
/// which
/// [`vkGetShaderInstrumentationValuesARM`](crate::Device::get_shader_instrumentation_values_arm)
/// calls.
pub type PFN_vkGetShaderInstrumentationValuesARM = Option<
    unsafe extern "system" fn(
        device: Device,
        instrumentation: ShaderInstrumentationARM,
        p_metric_block_count: *mut u32,
        p_metric_values: *mut c_void,
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetShaderModuleCreateInfoIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html),
/// which
/// [`vkGetShaderModuleCreateInfoIdentifierEXT`](crate::Device::get_shader_module_create_info_identifier_ext)
/// calls.
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_create_info: *const ShaderModuleCreateInfo<'_>,
        p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetShaderModuleIdentifierEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html),
/// which [`vkGetShaderModuleIdentifierEXT`](crate::Device::get_shader_module_identifier_ext)
/// calls.
pub type PFN_vkGetShaderModuleIdentifierEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        shader_module: ShaderModule,
        p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetSleepStatusLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html),
/// which [`vkGetSleepStatusLegacyNV`](crate::Device::get_sleep_status_legacy_nv) calls.
pub type PFN_vkGetSleepStatusLegacyNV =
    Option<unsafe extern "system" fn(device: Device, p_low_latency_mode: *mut Bool32)>;

/// The type of a pointer to the C command
/// [`vkGetSwapchainCounterEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html),
/// which [`vkGetSwapchainCounterEXT`](crate::Device::get_swapchain_counter_ext) calls.
pub type PFN_vkGetSwapchainCounterEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
        p_counter_value: *mut u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html),
/// which [`vkGetSwapchainImagesKHR`](crate::Device::get_swapchain_images_khr) calls.
pub type PFN_vkGetSwapchainImagesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_image_count: *mut u32,
        p_swapchain_images: *mut Image,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSwapchainStatusKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html),
/// which [`vkGetSwapchainStatusKHR`](crate::Device::get_swapchain_status_khr) calls.
pub type PFN_vkGetSwapchainStatusKHR =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkGetSwapchainTimeDomainPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html),
/// which
/// [`vkGetSwapchainTimeDomainPropertiesEXT`](crate::Device::get_swapchain_time_domain_properties_ext)
/// calls.
pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
        p_time_domains_counter: *mut u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetSwapchainTimingPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html),
/// which
/// [`vkGetSwapchainTimingPropertiesEXT`](crate::Device::get_swapchain_timing_properties_ext)
/// calls.
pub type PFN_vkGetSwapchainTimingPropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
        p_swapchain_timing_properties_counter: *mut u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetTensorMemoryRequirementsARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html),
/// which
/// [`vkGetTensorMemoryRequirementsARM`](crate::Device::get_tensor_memory_requirements_arm)
/// calls.
pub type PFN_vkGetTensorMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const TensorMemoryRequirementsInfoARM<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkGetTensorOpaqueCaptureDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html),
/// which [`vkGetTensorOpaqueCaptureDataARM`](crate::Device::get_tensor_opaque_capture_data_arm)
/// calls.
pub type PFN_vkGetTensorOpaqueCaptureDataARM = Option<
    unsafe extern "system" fn(
        device: Device,
        tensor_count: u32,
        p_tensors: *const TensorARM,
        p_datas: *mut HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetTensorOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html),
/// which
/// [`vkGetTensorOpaqueCaptureDescriptorDataARM`](crate::Device::get_tensor_opaque_capture_descriptor_data_arm)
/// calls.
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html),
/// which
/// [`vkGetTensorViewOpaqueCaptureDescriptorDataARM`](crate::Device::get_tensor_view_opaque_capture_descriptor_data_arm)
/// calls.
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetValidationCacheDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html),
/// which [`vkGetValidationCacheDataEXT`](crate::Device::get_validation_cache_data_ext) calls.
pub type PFN_vkGetValidationCacheDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        validation_cache: ValidationCacheEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetVideoSessionMemoryRequirementsKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html),
/// which
/// [`vkGetVideoSessionMemoryRequirementsKHR`](crate::Device::get_video_session_memory_requirements_khr)
/// calls.
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        video_session: VideoSessionKHR,
        p_memory_requirements_count: *mut u32,
        p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkGetWinrtDisplayNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html),
/// which [`vkGetWinrtDisplayNV`](crate::Instance::get_winrt_display_nv) calls.
#[cfg(feature = "win32")]
pub type PFN_vkGetWinrtDisplayNV = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut DisplayKHR,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkImportFenceFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html),
/// which [`vkImportFenceFdKHR`](crate::Device::import_fence_fd_khr) calls.
pub type PFN_vkImportFenceFdKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_import_fence_fd_info: *const ImportFenceFdInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkImportFenceWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html),
/// which [`vkImportFenceWin32HandleKHR`](crate::Device::import_fence_win32_handle_khr) calls.
#[cfg(feature = "win32")]
pub type PFN_vkImportFenceWin32HandleKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkImportSemaphoreFdKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html),
/// which [`vkImportSemaphoreFdKHR`](crate::Device::import_semaphore_fd_khr) calls.
pub type PFN_vkImportSemaphoreFdKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkImportSemaphoreWin32HandleKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html),
/// which [`vkImportSemaphoreWin32HandleKHR`](crate::Device::import_semaphore_win32_handle_khr)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkImportSemaphoreZirconHandleFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html),
/// which
/// [`vkImportSemaphoreZirconHandleFUCHSIA`](crate::Device::import_semaphore_zircon_handle_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkInitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html),
/// which [`vkInitializePerformanceApiINTEL`](crate::Device::initialize_performance_api_intel)
/// calls.
pub type PFN_vkInitializePerformanceApiINTEL = Option<
    unsafe extern "system" fn(
        device: Device,
        p_initialize_info: *const InitializePerformanceApiInfoINTEL<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkInvalidateMappedMemoryRanges`](https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html),
/// which [`vkInvalidateMappedMemoryRanges`](crate::Device::invalidate_mapped_memory_ranges)
/// calls.
pub type PFN_vkInvalidateMappedMemoryRanges = Option<
    unsafe extern "system" fn(
        device: Device,
        memory_range_count: u32,
        p_memory_ranges: *const MappedMemoryRange<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkLatencySleepLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html),
/// which [`vkLatencySleepLegacyNV`](crate::Device::latency_sleep_legacy_nv) calls.
pub type PFN_vkLatencySleepLegacyNV =
    Option<unsafe extern "system" fn(device: Device, signal_semaphore: Semaphore, value: u64)>;

/// The type of a pointer to the C command
/// [`vkLatencySleepNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html),
/// which [`vkLatencySleepNV`](crate::Device::latency_sleep_nv) calls.
pub type PFN_vkLatencySleepNV = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_sleep_info: *const LatencySleepInfoNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkMapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html),
/// which [`vkMapMemory`](crate::Device::map_memory) calls.
pub type PFN_vkMapMemory = Option<
    unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        pp_data: *mut *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkMapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html),
/// which [`vkMapMemory2`](crate::Device::map_memory2) calls.
pub type PFN_vkMapMemory2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkMapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html),
/// which [`vkMapMemory2KHR`](crate::Device::map_memory2_khr) calls.
pub type PFN_vkMapMemory2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_memory_map_info: *const MemoryMapInfo<'_>,
        pp_data: *mut *mut c_void,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkMergePipelineCaches`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html),
/// which [`vkMergePipelineCaches`](crate::Device::merge_pipeline_caches) calls.
pub type PFN_vkMergePipelineCaches = Option<
    unsafe extern "system" fn(
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        p_src_caches: *const PipelineCache,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkMergeValidationCachesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html),
/// which [`vkMergeValidationCachesEXT`](crate::Device::merge_validation_caches_ext) calls.
pub type PFN_vkMergeValidationCachesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        dst_cache: ValidationCacheEXT,
        src_cache_count: u32,
        p_src_caches: *const ValidationCacheEXT,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueBeginDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html),
/// which [`vkQueueBeginDebugUtilsLabelEXT`](crate::Device::queue_begin_debug_utils_label_ext)
/// calls.
pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>)>;

/// The type of a pointer to the C command
/// [`vkQueueBindSparse`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html),
/// which [`vkQueueBindSparse`](crate::Device::queue_bind_sparse) calls.
pub type PFN_vkQueueBindSparse = Option<
    unsafe extern "system" fn(
        queue: Queue,
        bind_info_count: u32,
        p_bind_info: *const BindSparseInfo<'_>,
        fence: Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueEndDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html),
/// which [`vkQueueEndDebugUtilsLabelEXT`](crate::Device::queue_end_debug_utils_label_ext)
/// calls.
pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<unsafe extern "system" fn(queue: Queue)>;

/// The type of a pointer to the C command
/// [`vkQueueInsertDebugUtilsLabelEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html),
/// which [`vkQueueInsertDebugUtilsLabelEXT`](crate::Device::queue_insert_debug_utils_label_ext)
/// calls.
pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(queue: Queue, p_label_info: *const DebugUtilsLabelEXT<'_>)>;

/// The type of a pointer to the C command
/// [`vkQueueNotifyOutOfBandLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html),
/// which [`vkQueueNotifyOutOfBandLegacyNV`](crate::Device::queue_notify_out_of_band_legacy_nv)
/// calls.
pub type PFN_vkQueueNotifyOutOfBandLegacyNV =
    Option<unsafe extern "system" fn(queue: Queue, queue_type: u32)>;

/// The type of a pointer to the C command
/// [`vkQueueNotifyOutOfBandNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html),
/// which [`vkQueueNotifyOutOfBandNV`](crate::Device::queue_notify_out_of_band_nv) calls.
pub type PFN_vkQueueNotifyOutOfBandNV = Option<
    unsafe extern "system" fn(queue: Queue, p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>),
>;

/// The type of a pointer to the C command
/// [`vkQueuePresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html),
/// which [`vkQueuePresentKHR`](crate::Device::queue_present_khr) calls.
pub type PFN_vkQueuePresentKHR = Option<
    unsafe extern "system" fn(queue: Queue, p_present_info: *const PresentInfoKHR<'_>) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueSetPerfHintQCOM`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html),
/// which [`vkQueueSetPerfHintQCOM`](crate::Device::queue_set_perf_hint_qcom) calls.
pub type PFN_vkQueueSetPerfHintQCOM = Option<
    unsafe extern "system" fn(
        queue: Queue,
        p_perf_hint_info: *const PerfHintInfoQCOM<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueSetPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html),
/// which
/// [`vkQueueSetPerformanceConfigurationINTEL`](crate::Device::queue_set_performance_configuration_intel)
/// calls.
pub type PFN_vkQueueSetPerformanceConfigurationINTEL = Option<
    unsafe extern "system" fn(
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueSubmit`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html),
/// which [`vkQueueSubmit`](crate::Device::queue_submit) calls.
pub type PFN_vkQueueSubmit = Option<
    unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo<'_>,
        fence: Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueSubmit2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html),
/// which [`vkQueueSubmit2`](crate::Device::queue_submit2) calls.
pub type PFN_vkQueueSubmit2 = Option<
    unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueSubmit2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html),
/// which [`vkQueueSubmit2KHR`](crate::Device::queue_submit2_khr) calls.
pub type PFN_vkQueueSubmit2KHR = Option<
    unsafe extern "system" fn(
        queue: Queue,
        submit_count: u32,
        p_submits: *const SubmitInfo2<'_>,
        fence: Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkQueueWaitIdle`](https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html),
/// which [`vkQueueWaitIdle`](crate::Device::queue_wait_idle) calls.
pub type PFN_vkQueueWaitIdle = Option<unsafe extern "system" fn(queue: Queue) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkRegisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html),
/// which [`vkRegisterCustomBorderColorEXT`](crate::Device::register_custom_border_color_ext)
/// calls.
pub type PFN_vkRegisterCustomBorderColorEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
        request_index: Bool32,
        p_index: *mut u32,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkRegisterDeviceEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html),
/// which [`vkRegisterDeviceEventEXT`](crate::Device::register_device_event_ext) calls.
pub type PFN_vkRegisterDeviceEventEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_device_event_info: *const DeviceEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkRegisterDisplayEventEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html),
/// which [`vkRegisterDisplayEventEXT`](crate::Device::register_display_event_ext) calls.
pub type PFN_vkRegisterDisplayEventEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkReleaseCapturedPipelineDataKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html),
/// which
/// [`vkReleaseCapturedPipelineDataKHR`](crate::Device::release_captured_pipeline_data_khr)
/// calls.
pub type PFN_vkReleaseCapturedPipelineDataKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkReleaseDisplayEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html),
/// which [`vkReleaseDisplayEXT`](crate::Instance::release_display_ext) calls.
pub type PFN_vkReleaseDisplayEXT = Option<
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkReleaseFullScreenExclusiveModeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html),
/// which
/// [`vkReleaseFullScreenExclusiveModeEXT`](crate::Device::release_full_screen_exclusive_mode_ext)
/// calls.
#[cfg(feature = "win32")]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    Option<unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkReleasePerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html),
/// which
/// [`vkReleasePerformanceConfigurationINTEL`](crate::Device::release_performance_configuration_intel)
/// calls.
pub type PFN_vkReleasePerformanceConfigurationINTEL = Option<
    unsafe extern "system" fn(
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkReleaseProfilingLockKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html),
/// which [`vkReleaseProfilingLockKHR`](crate::Device::release_profiling_lock_khr) calls.
pub type PFN_vkReleaseProfilingLockKHR = Option<unsafe extern "system" fn(device: Device)>;

/// The type of a pointer to the C command
/// [`vkReleaseSwapchainImagesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html),
/// which [`vkReleaseSwapchainImagesEXT`](crate::Device::release_swapchain_images_ext) calls.
pub type PFN_vkReleaseSwapchainImagesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkReleaseSwapchainImagesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html),
/// which [`vkReleaseSwapchainImagesKHR`](crate::Device::release_swapchain_images_khr) calls.
pub type PFN_vkReleaseSwapchainImagesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_release_info: *const ReleaseSwapchainImagesInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkResetCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html),
/// which [`vkResetCommandBuffer`](crate::Device::reset_command_buffer) calls.
pub type PFN_vkResetCommandBuffer = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkResetCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html),
/// which [`vkResetCommandPool`](crate::Device::reset_command_pool) calls.
pub type PFN_vkResetCommandPool = Option<
    unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkResetDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html),
/// which [`vkResetDescriptorPool`](crate::Device::reset_descriptor_pool) calls.
pub type PFN_vkResetDescriptorPool = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkResetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html),
/// which [`vkResetEvent`](crate::Device::reset_event) calls.
pub type PFN_vkResetEvent =
    Option<unsafe extern "system" fn(device: Device, event: Event) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkResetFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html),
/// which [`vkResetFences`](crate::Device::reset_fences) calls.
pub type PFN_vkResetFences = Option<
    unsafe extern "system" fn(device: Device, fence_count: u32, p_fences: *const Fence) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkResetGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html),
/// which [`vkResetGpaSessionAMD`](crate::Device::reset_gpa_session_amd) calls.
pub type PFN_vkResetGpaSessionAMD =
    Option<unsafe extern "system" fn(device: Device, gpa_session: GpaSessionAMD) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkResetQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html),
/// which [`vkResetQueryPool`](crate::Device::reset_query_pool) calls.
pub type PFN_vkResetQueryPool = Option<
    unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkResetQueryPoolEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html),
/// which [`vkResetQueryPoolEXT`](crate::Device::reset_query_pool_ext) calls.
pub type PFN_vkResetQueryPoolEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html),
/// which
/// [`vkSetBufferCollectionBufferConstraintsFUCHSIA`](crate::Device::set_buffer_collection_buffer_constraints_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetBufferCollectionImageConstraintsFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html),
/// which
/// [`vkSetBufferCollectionImageConstraintsFUCHSIA`](crate::Device::set_buffer_collection_image_constraints_fuchsia)
/// calls.
#[cfg(feature = "fuchsia")]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = Option<
    unsafe extern "system" fn(
        device: Device,
        collection: BufferCollectionFUCHSIA,
        p_image_constraints_info: *const ImageConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetDebugUtilsObjectNameEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html),
/// which [`vkSetDebugUtilsObjectNameEXT`](crate::Device::set_debug_utils_object_name_ext)
/// calls.
pub type PFN_vkSetDebugUtilsObjectNameEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_name_info: *const DebugUtilsObjectNameInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetDebugUtilsObjectTagEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html),
/// which [`vkSetDebugUtilsObjectTagEXT`](crate::Device::set_debug_utils_object_tag_ext) calls.
pub type PFN_vkSetDebugUtilsObjectTagEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        p_tag_info: *const DebugUtilsObjectTagInfoEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetDeviceMemoryPriorityEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html),
/// which [`vkSetDeviceMemoryPriorityEXT`](crate::Device::set_device_memory_priority_ext) calls.
pub type PFN_vkSetDeviceMemoryPriorityEXT =
    Option<unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32)>;

/// The type of a pointer to the C command
/// [`vkSetEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html),
/// which [`vkSetEvent`](crate::Device::set_event) calls.
pub type PFN_vkSetEvent =
    Option<unsafe extern "system" fn(device: Device, event: Event) -> VkResult>;

/// The type of a pointer to the C command
/// [`vkSetGpaDeviceClockModeAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html),
/// which [`vkSetGpaDeviceClockModeAMD`](crate::Device::set_gpa_device_clock_mode_amd) calls.
pub type PFN_vkSetGpaDeviceClockModeAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        p_info: *mut GpaDeviceClockModeInfoAMD<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetHdrMetadataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html),
/// which [`vkSetHdrMetadataEXT`](crate::Device::set_hdr_metadata_ext) calls.
pub type PFN_vkSetHdrMetadataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkSetLatencyMarkerLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html),
/// which [`vkSetLatencyMarkerLegacyNV`](crate::Device::set_latency_marker_legacy_nv) calls.
pub type PFN_vkSetLatencyMarkerLegacyNV =
    Option<unsafe extern "system" fn(device: Device, frame_id: u64, marker: u32)>;

/// The type of a pointer to the C command
/// [`vkSetLatencyMarkerNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html),
/// which [`vkSetLatencyMarkerNV`](crate::Device::set_latency_marker_nv) calls.
pub type PFN_vkSetLatencyMarkerNV = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkSetLatencySleepModeLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html),
/// which [`vkSetLatencySleepModeLegacyNV`](crate::Device::set_latency_sleep_mode_legacy_nv)
/// calls.
pub type PFN_vkSetLatencySleepModeLegacyNV = Option<
    unsafe extern "system" fn(
        device: Device,
        low_latency_mode: Bool32,
        low_latency_boost: Bool32,
        minimum_interval_us: u32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkSetLatencySleepModeNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html),
/// which [`vkSetLatencySleepModeNV`](crate::Device::set_latency_sleep_mode_nv) calls.
pub type PFN_vkSetLatencySleepModeNV = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetLocalDimmingAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html),
/// which [`vkSetLocalDimmingAMD`](crate::Device::set_local_dimming_amd) calls.
pub type PFN_vkSetLocalDimmingAMD = Option<
    unsafe extern "system" fn(
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ),
>;

/// The type of a pointer to the C command
/// [`vkSetPrivateData`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html),
/// which [`vkSetPrivateData`](crate::Device::set_private_data) calls.
pub type PFN_vkSetPrivateData = Option<
    unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetPrivateDataEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html),
/// which [`vkSetPrivateDataEXT`](crate::Device::set_private_data_ext) calls.
pub type PFN_vkSetPrivateDataEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSetSwapchainPresentTimingQueueSizeEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html),
/// which
/// [`vkSetSwapchainPresentTimingQueueSizeEXT`](crate::Device::set_swapchain_present_timing_queue_size_ext)
/// calls.
pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT = Option<
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, size: u32) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkShutdownLatencyDeviceLegacyNV`](https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html),
/// which [`vkShutdownLatencyDeviceLegacyNV`](crate::Device::shutdown_latency_device_legacy_nv)
/// calls.
pub type PFN_vkShutdownLatencyDeviceLegacyNV = Option<unsafe extern "system" fn(device: Device)>;

/// The type of a pointer to the C command
/// [`vkSignalSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html),
/// which [`vkSignalSemaphore`](crate::Device::signal_semaphore) calls.
pub type PFN_vkSignalSemaphore = Option<
    unsafe extern "system" fn(
        device: Device,
        p_signal_info: *const SemaphoreSignalInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSignalSemaphoreKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html),
/// which [`vkSignalSemaphoreKHR`](crate::Device::signal_semaphore_khr) calls.
pub type PFN_vkSignalSemaphoreKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_signal_info: *const SemaphoreSignalInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkSubmitDebugUtilsMessageEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html),
/// which [`vkSubmitDebugUtilsMessageEXT`](crate::Instance::submit_debug_utils_message_ext)
/// calls.
pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<
    unsafe extern "system" fn(
        instance: Instance,
        message_severity: DebugUtilsMessageSeverityFlagBitsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const DebugUtilsMessengerCallbackDataEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkTransitionImageLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html),
/// which [`vkTransitionImageLayout`](crate::Device::transition_image_layout) calls.
pub type PFN_vkTransitionImageLayout = Option<
    unsafe extern "system" fn(
        device: Device,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkTransitionImageLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html),
/// which [`vkTransitionImageLayoutEXT`](crate::Device::transition_image_layout_ext) calls.
pub type PFN_vkTransitionImageLayoutEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        transition_count: u32,
        p_transitions: *const HostImageLayoutTransitionInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkTrimCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html),
/// which [`vkTrimCommandPool`](crate::Device::trim_command_pool) calls.
pub type PFN_vkTrimCommandPool = Option<
    unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkTrimCommandPoolKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html),
/// which [`vkTrimCommandPoolKHR`](crate::Device::trim_command_pool_khr) calls.
pub type PFN_vkTrimCommandPoolKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUninitializePerformanceApiINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html),
/// which
/// [`vkUninitializePerformanceApiINTEL`](crate::Device::uninitialize_performance_api_intel)
/// calls.
pub type PFN_vkUninitializePerformanceApiINTEL = Option<unsafe extern "system" fn(device: Device)>;

/// The type of a pointer to the C command
/// [`vkUnmapMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html),
/// which [`vkUnmapMemory`](crate::Device::unmap_memory) calls.
pub type PFN_vkUnmapMemory =
    Option<unsafe extern "system" fn(device: Device, memory: DeviceMemory)>;

/// The type of a pointer to the C command
/// [`vkUnmapMemory2`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html),
/// which [`vkUnmapMemory2`](crate::Device::unmap_memory2) calls.
pub type PFN_vkUnmapMemory2 = Option<
    unsafe extern "system" fn(
        device: Device,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkUnmapMemory2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html),
/// which [`vkUnmapMemory2KHR`](crate::Device::unmap_memory2_khr) calls.
pub type PFN_vkUnmapMemory2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_memory_unmap_info: *const MemoryUnmapInfo<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkUnregisterCustomBorderColorEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html),
/// which
/// [`vkUnregisterCustomBorderColorEXT`](crate::Device::unregister_custom_border_color_ext)
/// calls.
pub type PFN_vkUnregisterCustomBorderColorEXT =
    Option<unsafe extern "system" fn(device: Device, index: u32)>;

/// The type of a pointer to the C command
/// [`vkUpdateDescriptorSetWithTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html),
/// which
/// [`vkUpdateDescriptorSetWithTemplate`](crate::Device::update_descriptor_set_with_template)
/// calls.
pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUpdateDescriptorSetWithTemplateKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html),
/// which
/// [`vkUpdateDescriptorSetWithTemplateKHR`](crate::Device::update_descriptor_set_with_template_khr)
/// calls.
pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        p_data: *const c_void,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUpdateDescriptorSets`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html),
/// which [`vkUpdateDescriptorSets`](crate::Device::update_descriptor_sets) calls.
pub type PFN_vkUpdateDescriptorSets = Option<
    unsafe extern "system" fn(
        device: Device,
        descriptor_write_count: u32,
        p_descriptor_writes: *const WriteDescriptorSet<'_>,
        descriptor_copy_count: u32,
        p_descriptor_copies: *const CopyDescriptorSet<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUpdateIndirectExecutionSetPipelineEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html),
/// which
/// [`vkUpdateIndirectExecutionSetPipelineEXT`](crate::Device::update_indirect_execution_set_pipeline_ext)
/// calls.
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUpdateIndirectExecutionSetShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html),
/// which
/// [`vkUpdateIndirectExecutionSetShaderEXT`](crate::Device::update_indirect_execution_set_shader_ext)
/// calls.
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_write_count: u32,
        p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT<'_>,
    ),
>;

/// The type of a pointer to the C command
/// [`vkUpdateVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html),
/// which
/// [`vkUpdateVideoSessionParametersKHR`](crate::Device::update_video_session_parameters_khr)
/// calls.
pub type PFN_vkUpdateVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWaitForFences`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html),
/// which [`vkWaitForFences`](crate::Device::wait_for_fences) calls.
pub type PFN_vkWaitForFences = Option<
    unsafe extern "system" fn(
        device: Device,
        fence_count: u32,
        p_fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWaitForPresent2KHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html),
/// which [`vkWaitForPresent2KHR`](crate::Device::wait_for_present2_khr) calls.
pub type PFN_vkWaitForPresent2KHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_present_wait2_info: *const PresentWait2InfoKHR<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWaitForPresentKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html),
/// which [`vkWaitForPresentKHR`](crate::Device::wait_for_present_khr) calls.
pub type PFN_vkWaitForPresentKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWaitSemaphores`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html),
/// which [`vkWaitSemaphores`](crate::Device::wait_semaphores) calls.
pub type PFN_vkWaitSemaphores = Option<
    unsafe extern "system" fn(
        device: Device,
        p_wait_info: *const SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWaitSemaphoresKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html),
/// which [`vkWaitSemaphoresKHR`](crate::Device::wait_semaphores_khr) calls.
pub type PFN_vkWaitSemaphoresKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        p_wait_info: *const SemaphoreWaitInfo<'_>,
        timeout: u64,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWriteAccelerationStructuresPropertiesKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html),
/// which
/// [`vkWriteAccelerationStructuresPropertiesKHR`](crate::Device::write_acceleration_structures_properties_khr)
/// calls.
pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = Option<
    unsafe extern "system" fn(
        device: Device,
        acceleration_structure_count: u32,
        p_acceleration_structures: *const AccelerationStructureKHR,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWriteMicromapsPropertiesEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html),
/// which [`vkWriteMicromapsPropertiesEXT`](crate::Device::write_micromaps_properties_ext)
/// calls.
pub type PFN_vkWriteMicromapsPropertiesEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        micromap_count: u32,
        p_micromaps: *const MicromapEXT,
        query_type: QueryType,
        data_size: usize,
        p_data: *mut c_void,
        stride: usize,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWriteResourceDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html),
/// which [`vkWriteResourceDescriptorsEXT`](crate::Device::write_resource_descriptors_ext)
/// calls.
pub type PFN_vkWriteResourceDescriptorsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        resource_count: u32,
        p_resources: *const ResourceDescriptorInfoEXT<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;

/// The type of a pointer to the C command
/// [`vkWriteSamplerDescriptorsEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html),
/// which [`vkWriteSamplerDescriptorsEXT`](crate::Device::write_sampler_descriptors_ext) calls.
pub type PFN_vkWriteSamplerDescriptorsEXT = Option<
    unsafe extern "system" fn(
        device: Device,
        sampler_count: u32,
        p_samplers: *const SamplerCreateInfo<'_>,
        p_descriptors: *const HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;
