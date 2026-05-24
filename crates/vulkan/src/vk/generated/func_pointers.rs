#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::vk::*;

use core::ffi::{c_char, c_void};

pub type PFN_vkVoidFunction = Option<unsafe extern "system" fn()>;

pub type PFN_vkAcquireDrmDisplayEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, i32, DisplayKHR) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkAcquireFullScreenExclusiveModeEXT =
    Option<unsafe extern "system" fn(Device, SwapchainKHR) -> VkResult>;

pub type PFN_vkAcquireNextImage2KHR = Option<
    unsafe extern "system" fn(Device, *const AcquireNextImageInfoKHR<'_>, *mut u32) -> VkResult,
>;

pub type PFN_vkAcquireNextImageKHR = Option<
    unsafe extern "system" fn(Device, SwapchainKHR, u64, Semaphore, Fence, *mut u32) -> VkResult,
>;

pub type PFN_vkAcquirePerformanceConfigurationINTEL = Option<
    unsafe extern "system" fn(
        Device,
        *const PerformanceConfigurationAcquireInfoINTEL<'_>,
        *mut PerformanceConfigurationINTEL,
    ) -> VkResult,
>;

pub type PFN_vkAcquireProfilingLockKHR =
    Option<unsafe extern "system" fn(Device, *const AcquireProfilingLockInfoKHR<'_>) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkAcquireWinrtDisplayNV =
    Option<unsafe extern "system" fn(PhysicalDevice, DisplayKHR) -> VkResult>;

#[cfg(feature = "xlib-xrandr")]
pub type PFN_vkAcquireXlibDisplayEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut Display, DisplayKHR) -> VkResult>;

pub type PFN_vkAllocateCommandBuffers = Option<
    unsafe extern "system" fn(
        Device,
        *const CommandBufferAllocateInfo<'_>,
        *mut CommandBuffer,
    ) -> VkResult,
>;

pub type PFN_vkAllocateDescriptorSets = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorSetAllocateInfo<'_>,
        *mut DescriptorSet,
    ) -> VkResult,
>;

pub type PFN_vkAllocateMemory = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryAllocateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut DeviceMemory,
    ) -> VkResult,
>;

pub type PFN_vkAllocationFunction = Option<
    unsafe extern "system" fn(*mut c_void, usize, usize, SystemAllocationScope) -> *mut c_void,
>;

pub type PFN_vkAntiLagUpdateAMD =
    Option<unsafe extern "system" fn(Device, *const AntiLagDataAMD<'_>) -> ()>;

pub type PFN_vkBeginCommandBuffer =
    Option<unsafe extern "system" fn(CommandBuffer, *const CommandBufferBeginInfo<'_>) -> VkResult>;

pub type PFN_vkBindAccelerationStructureMemoryNV = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const BindAccelerationStructureMemoryInfoNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkBindBufferMemory =
    Option<unsafe extern "system" fn(Device, Buffer, DeviceMemory, u64) -> VkResult>;

pub type PFN_vkBindBufferMemory2 =
    Option<unsafe extern "system" fn(Device, u32, *const BindBufferMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkBindBufferMemory2KHR =
    Option<unsafe extern "system" fn(Device, u32, *const BindBufferMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkBindImageMemory =
    Option<unsafe extern "system" fn(Device, Image, DeviceMemory, u64) -> VkResult>;

pub type PFN_vkBindImageMemory2 =
    Option<unsafe extern "system" fn(Device, u32, *const BindImageMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkBindImageMemory2KHR =
    Option<unsafe extern "system" fn(Device, u32, *const BindImageMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkBindOpticalFlowSessionImageNV = Option<
    unsafe extern "system" fn(
        Device,
        OpticalFlowSessionNV,
        OpticalFlowSessionBindingPointNV,
        ImageView,
        ImageLayout,
    ) -> VkResult,
>;

pub type PFN_vkBindTensorMemoryARM =
    Option<unsafe extern "system" fn(Device, u32, *const BindTensorMemoryInfoARM<'_>) -> VkResult>;

pub type PFN_vkBindVideoSessionMemoryKHR = Option<
    unsafe extern "system" fn(
        Device,
        VideoSessionKHR,
        u32,
        *const BindVideoSessionMemoryInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkBuildAccelerationStructuresKHR = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        u32,
        *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        *const *const AccelerationStructureBuildRangeInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkBuildMicromapsEXT = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        u32,
        *const MicromapBuildInfoEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkClearShaderInstrumentationMetricsARM =
    Option<unsafe extern "system" fn(Device, ShaderInstrumentationARM) -> ()>;

pub type PFN_vkCmdBeginConditionalRendering2EXT = Option<
    unsafe extern "system" fn(CommandBuffer, *const ConditionalRenderingBeginInfo2EXT<'_>) -> (),
>;

pub type PFN_vkCmdBeginConditionalRenderingEXT = Option<
    unsafe extern "system" fn(CommandBuffer, *const ConditionalRenderingBeginInfoEXT<'_>) -> (),
>;

pub type PFN_vkCmdBeginCustomResolveEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const BeginCustomResolveInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdBeginDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DebugUtilsLabelEXT<'_>) -> ()>;

pub type PFN_vkCmdBeginPerTileExecutionQCOM =
    Option<unsafe extern "system" fn(CommandBuffer, *const PerTileBeginInfoQCOM<'_>) -> ()>;

pub type PFN_vkCmdBeginQuery =
    Option<unsafe extern "system" fn(CommandBuffer, QueryPool, u32, QueryControlFlags) -> ()>;

pub type PFN_vkCmdBeginQueryIndexedEXT =
    Option<unsafe extern "system" fn(CommandBuffer, QueryPool, u32, QueryControlFlags, u32) -> ()>;

pub type PFN_vkCmdBeginRenderPass = Option<
    unsafe extern "system" fn(CommandBuffer, *const RenderPassBeginInfo<'_>, SubpassContents) -> (),
>;

pub type PFN_vkCmdBeginRenderPass2 = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const RenderPassBeginInfo<'_>,
        *const SubpassBeginInfo<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBeginRenderPass2KHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const RenderPassBeginInfo<'_>,
        *const SubpassBeginInfo<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBeginRendering =
    Option<unsafe extern "system" fn(CommandBuffer, *const RenderingInfo<'_>) -> ()>;

pub type PFN_vkCmdBeginRenderingKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const RenderingInfo<'_>) -> ()>;

pub type PFN_vkCmdBeginShaderInstrumentationARM =
    Option<unsafe extern "system" fn(CommandBuffer, ShaderInstrumentationARM) -> ()>;

pub type PFN_vkCmdBeginTransformFeedback2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBeginTransformFeedbackEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Buffer, *const u64) -> ()>;

pub type PFN_vkCmdBeginVideoCodingKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const VideoBeginCodingInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const BindDescriptorBufferEmbeddedSamplersInfoEXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineBindPoint, PipelineLayout, u32) -> ()>;

pub type PFN_vkCmdBindDescriptorBuffersEXT = Option<
    unsafe extern "system" fn(CommandBuffer, u32, *const DescriptorBufferBindingInfoEXT<'_>) -> (),
>;

pub type PFN_vkCmdBindDescriptorSets = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineBindPoint,
        PipelineLayout,
        u32,
        u32,
        *const DescriptorSet,
        u32,
        *const u32,
    ) -> (),
>;

pub type PFN_vkCmdBindDescriptorSets2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const BindDescriptorSetsInfo<'_>) -> ()>;

pub type PFN_vkCmdBindDescriptorSets2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const BindDescriptorSetsInfo<'_>) -> ()>;

pub type PFN_vkCmdBindIndexBuffer =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, IndexType) -> ()>;

pub type PFN_vkCmdBindIndexBuffer2 =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u64, IndexType) -> ()>;

pub type PFN_vkCmdBindIndexBuffer2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u64, IndexType) -> ()>;

pub type PFN_vkCmdBindIndexBuffer3KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const BindIndexBuffer3InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdBindInvocationMaskHUAWEI =
    Option<unsafe extern "system" fn(CommandBuffer, ImageView, ImageLayout) -> ()>;

pub type PFN_vkCmdBindPipeline =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineBindPoint, Pipeline) -> ()>;

pub type PFN_vkCmdBindPipelineShaderGroupNV =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineBindPoint, Pipeline, u32) -> ()>;

pub type PFN_vkCmdBindResourceHeapEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const BindHeapInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdBindSamplerHeapEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const BindHeapInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdBindShadersEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const ShaderStageFlagBits,
        *const ShaderEXT,
    ) -> (),
>;

pub type PFN_vkCmdBindShadingRateImageNV =
    Option<unsafe extern "system" fn(CommandBuffer, ImageView, ImageLayout) -> ()>;

pub type PFN_vkCmdBindTileMemoryQCOM =
    Option<unsafe extern "system" fn(CommandBuffer, *const TileMemoryBindInfoQCOM<'_>) -> ()>;

pub type PFN_vkCmdBindTransformFeedbackBuffers2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBindTransformFeedbackBuffersEXT = Option<
    unsafe extern "system" fn(CommandBuffer, u32, u32, *const Buffer, *const u64, *const u64) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Buffer, *const u64) -> ()>;

pub type PFN_vkCmdBindVertexBuffers2 = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const Buffer,
        *const u64,
        *const u64,
        *const u64,
    ) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const Buffer,
        *const u64,
        *const u64,
        *const u64,
    ) -> (),
>;

pub type PFN_vkCmdBindVertexBuffers3KHR = Option<
    unsafe extern "system" fn(CommandBuffer, u32, u32, *const BindVertexBuffer3InfoKHR<'_>) -> (),
>;

pub type PFN_vkCmdBlitImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        Image,
        ImageLayout,
        u32,
        *const ImageBlit<'_>,
        Filter,
    ) -> (),
>;

pub type PFN_vkCmdBlitImage2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const BlitImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdBlitImage2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const BlitImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdBuildAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const AccelerationStructureInfoNV<'_>,
        Buffer,
        u64,
        Bool32,
        AccelerationStructureNV,
        AccelerationStructureNV,
        Buffer,
        u64,
    ) -> (),
>;

pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        *const u64,
        *const u32,
        *const *const u32,
    ) -> (),
>;

pub type PFN_vkCmdBuildAccelerationStructuresKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        *const *const AccelerationStructureBuildRangeInfoKHR<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBuildClusterAccelerationStructureIndirectNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const ClusterAccelerationStructureCommandsInfoNV<'_>,
    ) -> (),
>;

pub type PFN_vkCmdBuildMicromapsEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const MicromapBuildInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdBuildPartitionedAccelerationStructuresNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const BuildPartitionedAccelerationStructureInfoNV<'_>,
    ) -> (),
>;

pub type PFN_vkCmdClearAttachments = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const ClearAttachment<'_>,
        u32,
        *const ClearRect<'_>,
    ) -> (),
>;

pub type PFN_vkCmdClearColorImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        *const ClearColorValue<'_>,
        u32,
        *const ImageSubresourceRange<'_>,
    ) -> (),
>;

pub type PFN_vkCmdClearDepthStencilImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        *const ClearDepthStencilValue<'_>,
        u32,
        *const ImageSubresourceRange<'_>,
    ) -> (),
>;

pub type PFN_vkCmdControlVideoCodingKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const VideoCodingControlInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdConvertCooperativeVectorMatrixNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const ConvertCooperativeVectorMatrixInfoNV<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(CommandBuffer, *const CopyAccelerationStructureInfoKHR<'_>) -> (),
>;

pub type PFN_vkCmdCopyAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        AccelerationStructureNV,
        AccelerationStructureNV,
        CopyAccelerationStructureModeKHR,
    ) -> (),
>;

pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyBuffer = Option<
    unsafe extern "system" fn(CommandBuffer, Buffer, Buffer, u32, *const BufferCopy<'_>) -> (),
>;

pub type PFN_vkCmdCopyBuffer2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyBufferInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyBuffer2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyBufferInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyBufferToImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Buffer,
        Image,
        ImageLayout,
        u32,
        *const BufferImageCopy<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyBufferToImage2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyBufferToImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyBufferToImage2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyBufferToImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        Image,
        ImageLayout,
        u32,
        *const ImageCopy<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyImage2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyImage2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyImageToBuffer = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        Buffer,
        u32,
        *const BufferImageCopy<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyImageToBuffer2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyImageToBufferInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyImageToBuffer2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyImageToBufferInfo2<'_>) -> ()>;

pub type PFN_vkCmdCopyImageToMemoryKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyDeviceMemoryImageInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdCopyMemoryIndirectKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyMemoryIndirectInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdCopyMemoryIndirectNV =
    Option<unsafe extern "system" fn(CommandBuffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdCopyMemoryKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyDeviceMemoryInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageIndirectKHR = Option<
    unsafe extern "system" fn(CommandBuffer, *const CopyMemoryToImageIndirectInfoKHR<'_>) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageIndirectNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u64,
        u32,
        u32,
        Image,
        ImageLayout,
        *const ImageSubresourceLayers<'_>,
    ) -> (),
>;

pub type PFN_vkCmdCopyMemoryToImageKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyDeviceMemoryImageInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdCopyMemoryToMicromapEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyMemoryToMicromapInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdCopyMicromapEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyMicromapInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdCopyMicromapToMemoryEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyMicromapToMemoryInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdCopyQueryPoolResults = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        QueryPool,
        u32,
        u32,
        Buffer,
        u64,
        u64,
        QueryResultFlags,
    ) -> (),
>;

pub type PFN_vkCmdCopyQueryPoolResultsToMemoryKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        QueryPool,
        u32,
        u32,
        *const StridedDeviceAddressRangeKHR<'_>,
        AddressCommandFlagsKHR,
        QueryResultFlags,
    ) -> (),
>;

pub type PFN_vkCmdCopyTensorARM =
    Option<unsafe extern "system" fn(CommandBuffer, *const CopyTensorInfoARM<'_>) -> ()>;

pub type PFN_vkCmdCuLaunchKernelNVX =
    Option<unsafe extern "system" fn(CommandBuffer, *const CuLaunchInfoNVX<'_>) -> ()>;

#[cfg(feature = "beta")]
pub type PFN_vkCmdCudaLaunchKernelNV =
    Option<unsafe extern "system" fn(CommandBuffer, *const CudaLaunchInfoNV<'_>) -> ()>;

pub type PFN_vkCmdDebugMarkerBeginEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DebugMarkerMarkerInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdDebugMarkerEndEXT = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdDebugMarkerInsertEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DebugMarkerMarkerInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdDecodeVideoKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const VideoDecodeInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDecompressMemoryEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DecompressMemoryInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdDecompressMemoryIndirectCountEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        MemoryDecompressionMethodFlagsEXT,
        u64,
        u64,
        u32,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdDecompressMemoryIndirectCountNV =
    Option<unsafe extern "system" fn(CommandBuffer, u64, u64, u32) -> ()>;

pub type PFN_vkCmdDecompressMemoryNV = Option<
    unsafe extern "system" fn(CommandBuffer, u32, *const DecompressMemoryRegionNV<'_>) -> (),
>;

pub type PFN_vkCmdDispatch = Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDispatchBase =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDispatchBaseKHR =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDispatchDataGraphARM = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        DataGraphPipelineSessionARM,
        *const DataGraphPipelineDispatchInfoARM<'_>,
    ) -> (),
>;

#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphAMDX = Option<
    unsafe extern "system" fn(CommandBuffer, u64, u64, *const DispatchGraphCountInfoAMDX<'_>) -> (),
>;

#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphIndirectAMDX = Option<
    unsafe extern "system" fn(CommandBuffer, u64, u64, *const DispatchGraphCountInfoAMDX<'_>) -> (),
>;

#[cfg(feature = "beta")]
pub type PFN_vkCmdDispatchGraphIndirectCountAMDX =
    Option<unsafe extern "system" fn(CommandBuffer, u64, u64, u64) -> ()>;

pub type PFN_vkCmdDispatchIndirect =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64) -> ()>;

pub type PFN_vkCmdDispatchIndirect2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DispatchIndirect2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDispatchTileQCOM =
    Option<unsafe extern "system" fn(CommandBuffer, *const DispatchTileInfoQCOM<'_>) -> ()>;

pub type PFN_vkCmdDraw = Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDrawClusterHUAWEI =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64) -> ()>;

pub type PFN_vkCmdDrawIndexed =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32, i32, u32) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirect =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirect2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirect2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirectCount =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirectCount2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirectCount2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirectCountAMD =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndexedIndirectCountKHR =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndirect =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndirect2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirect2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawIndirectByteCount2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const BindTransformFeedbackBuffer2InfoEXT<'_>,
        u32,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdDrawIndirectByteCountEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndirectCount =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndirectCount2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirectCount2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawIndirectCountAMD =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawIndirectCountKHR =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirect2EXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirect2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirectCount2EXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DrawIndirectCount2InfoKHR<'_>) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirectCountEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirectCountNV =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirectEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksIndirectNV =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMeshTasksNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32) -> ()>;

pub type PFN_vkCmdDrawMultiEXT = Option<
    unsafe extern "system" fn(CommandBuffer, u32, *const MultiDrawInfoEXT<'_>, u32, u32, u32) -> (),
>;

pub type PFN_vkCmdDrawMultiIndexedEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const MultiDrawIndexedInfoEXT<'_>,
        u32,
        u32,
        u32,
        *const i32,
    ) -> (),
>;

pub type PFN_vkCmdEncodeVideoKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const VideoEncodeInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdEndConditionalRenderingEXT =
    Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndDebugUtilsLabelEXT = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndPerTileExecutionQCOM =
    Option<unsafe extern "system" fn(CommandBuffer, *const PerTileEndInfoQCOM<'_>) -> ()>;

pub type PFN_vkCmdEndQuery = Option<unsafe extern "system" fn(CommandBuffer, QueryPool, u32) -> ()>;

pub type PFN_vkCmdEndQueryIndexedEXT =
    Option<unsafe extern "system" fn(CommandBuffer, QueryPool, u32, u32) -> ()>;

pub type PFN_vkCmdEndRenderPass = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndRenderPass2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const SubpassEndInfo<'_>) -> ()>;

pub type PFN_vkCmdEndRenderPass2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const SubpassEndInfo<'_>) -> ()>;

pub type PFN_vkCmdEndRendering = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndRendering2EXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const RenderingEndInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdEndRendering2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const RenderingEndInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdEndRenderingKHR = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndShaderInstrumentationARM =
    Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdEndTransformFeedback2EXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        u32,
        *const BindTransformFeedbackBuffer2InfoEXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdEndTransformFeedbackEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Buffer, *const u64) -> ()>;

pub type PFN_vkCmdEndVideoCodingKHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const VideoEndCodingInfoKHR<'_>) -> ()>;

pub type PFN_vkCmdExecuteCommands =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const CommandBuffer) -> ()>;

pub type PFN_vkCmdExecuteGeneratedCommandsEXT = Option<
    unsafe extern "system" fn(CommandBuffer, Bool32, *const GeneratedCommandsInfoEXT<'_>) -> (),
>;

pub type PFN_vkCmdExecuteGeneratedCommandsNV = Option<
    unsafe extern "system" fn(CommandBuffer, Bool32, *const GeneratedCommandsInfoNV<'_>) -> (),
>;

pub type PFN_vkCmdFillBuffer =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u64, u32) -> ()>;

pub type PFN_vkCmdFillMemoryKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const DeviceAddressRangeKHR<'_>,
        AddressCommandFlagsKHR,
        u32,
    ) -> (),
>;

#[cfg(feature = "beta")]
pub type PFN_vkCmdInitializeGraphScratchMemoryAMDX =
    Option<unsafe extern "system" fn(CommandBuffer, Pipeline, u64, u64) -> ()>;

pub type PFN_vkCmdInsertDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DebugUtilsLabelEXT<'_>) -> ()>;

pub type PFN_vkCmdNextSubpass =
    Option<unsafe extern "system" fn(CommandBuffer, SubpassContents) -> ()>;

pub type PFN_vkCmdNextSubpass2 = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const SubpassBeginInfo<'_>,
        *const SubpassEndInfo<'_>,
    ) -> (),
>;

pub type PFN_vkCmdNextSubpass2KHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const SubpassBeginInfo<'_>,
        *const SubpassEndInfo<'_>,
    ) -> (),
>;

pub type PFN_vkCmdOpticalFlowExecuteNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        OpticalFlowSessionNV,
        *const OpticalFlowExecuteInfoNV<'_>,
    ) -> (),
>;

pub type PFN_vkCmdPipelineBarrier = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineStageFlags,
        PipelineStageFlags,
        DependencyFlags,
        u32,
        *const MemoryBarrier<'_>,
        u32,
        *const BufferMemoryBarrier<'_>,
        u32,
        *const ImageMemoryBarrier<'_>,
    ) -> (),
>;

pub type PFN_vkCmdPipelineBarrier2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const DependencyInfo<'_>) -> ()>;

pub type PFN_vkCmdPipelineBarrier2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const DependencyInfo<'_>) -> ()>;

pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const GeneratedCommandsInfoEXT<'_>,
        CommandBuffer,
    ) -> (),
>;

pub type PFN_vkCmdPreprocessGeneratedCommandsNV =
    Option<unsafe extern "system" fn(CommandBuffer, *const GeneratedCommandsInfoNV<'_>) -> ()>;

pub type PFN_vkCmdPushConstants = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineLayout,
        ShaderStageFlags,
        u32,
        u32,
        *const c_void,
    ) -> (),
>;

pub type PFN_vkCmdPushConstants2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const PushConstantsInfo<'_>) -> ()>;

pub type PFN_vkCmdPushConstants2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const PushConstantsInfo<'_>) -> ()>;

pub type PFN_vkCmdPushDataEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const PushDataInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdPushDescriptorSet = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineBindPoint,
        PipelineLayout,
        u32,
        u32,
        *const WriteDescriptorSet<'_>,
    ) -> (),
>;

pub type PFN_vkCmdPushDescriptorSet2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const PushDescriptorSetInfo<'_>) -> ()>;

pub type PFN_vkCmdPushDescriptorSet2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const PushDescriptorSetInfo<'_>) -> ()>;

pub type PFN_vkCmdPushDescriptorSetKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineBindPoint,
        PipelineLayout,
        u32,
        u32,
        *const WriteDescriptorSet<'_>,
    ) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplate = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        DescriptorUpdateTemplate,
        PipelineLayout,
        u32,
        *const c_void,
    ) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = Option<
    unsafe extern "system" fn(CommandBuffer, *const PushDescriptorSetWithTemplateInfo<'_>) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplate2KHR = Option<
    unsafe extern "system" fn(CommandBuffer, *const PushDescriptorSetWithTemplateInfo<'_>) -> (),
>;

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        DescriptorUpdateTemplate,
        PipelineLayout,
        u32,
        *const c_void,
    ) -> (),
>;

pub type PFN_vkCmdResetEvent =
    Option<unsafe extern "system" fn(CommandBuffer, Event, PipelineStageFlags) -> ()>;

pub type PFN_vkCmdResetEvent2 =
    Option<unsafe extern "system" fn(CommandBuffer, Event, PipelineStageFlags2) -> ()>;

pub type PFN_vkCmdResetEvent2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, Event, PipelineStageFlags2) -> ()>;

pub type PFN_vkCmdResetQueryPool =
    Option<unsafe extern "system" fn(CommandBuffer, QueryPool, u32, u32) -> ()>;

pub type PFN_vkCmdResolveImage = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Image,
        ImageLayout,
        Image,
        ImageLayout,
        u32,
        *const ImageResolve<'_>,
    ) -> (),
>;

pub type PFN_vkCmdResolveImage2 =
    Option<unsafe extern "system" fn(CommandBuffer, *const ResolveImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdResolveImage2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, *const ResolveImageInfo2<'_>) -> ()>;

pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, ImageAspectFlags) -> ()>;

pub type PFN_vkCmdSetBlendConstants =
    Option<unsafe extern "system" fn(CommandBuffer, [f32; 4]) -> ()>;

pub type PFN_vkCmdSetCheckpointNV =
    Option<unsafe extern "system" fn(CommandBuffer, *const c_void) -> ()>;

pub type PFN_vkCmdSetCoarseSampleOrderNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        CoarseSampleOrderTypeNV,
        u32,
        *const CoarseSampleOrderCustomNV<'_>,
    ) -> (),
>;

pub type PFN_vkCmdSetColorBlendAdvancedEXT = Option<
    unsafe extern "system" fn(CommandBuffer, u32, u32, *const ColorBlendAdvancedEXT<'_>) -> (),
>;

pub type PFN_vkCmdSetColorBlendEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Bool32) -> ()>;

pub type PFN_vkCmdSetColorBlendEquationEXT = Option<
    unsafe extern "system" fn(CommandBuffer, u32, u32, *const ColorBlendEquationEXT<'_>) -> (),
>;

pub type PFN_vkCmdSetColorWriteEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const Bool32) -> ()>;

pub type PFN_vkCmdSetColorWriteMaskEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const ColorComponentFlags) -> ()>;

pub type PFN_vkCmdSetComputeOccupancyPriorityNV = Option<
    unsafe extern "system" fn(CommandBuffer, *const ComputeOccupancyPriorityParametersNV<'_>) -> (),
>;

pub type PFN_vkCmdSetConservativeRasterizationModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, ConservativeRasterizationModeEXT) -> ()>;

pub type PFN_vkCmdSetCoverageModulationModeNV =
    Option<unsafe extern "system" fn(CommandBuffer, CoverageModulationModeNV) -> ()>;

pub type PFN_vkCmdSetCoverageModulationTableEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetCoverageModulationTableNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const f32) -> ()>;

pub type PFN_vkCmdSetCoverageReductionModeNV =
    Option<unsafe extern "system" fn(CommandBuffer, CoverageReductionModeNV) -> ()>;

pub type PFN_vkCmdSetCoverageToColorEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetCoverageToColorLocationNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetCullMode =
    Option<unsafe extern "system" fn(CommandBuffer, CullModeFlags) -> ()>;

pub type PFN_vkCmdSetCullModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, CullModeFlags) -> ()>;

pub type PFN_vkCmdSetDepthBias =
    Option<unsafe extern "system" fn(CommandBuffer, f32, f32, f32) -> ()>;

pub type PFN_vkCmdSetDepthBias2EXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const DepthBiasInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdSetDepthBiasEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthBiasEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthBounds = Option<unsafe extern "system" fn(CommandBuffer, f32, f32) -> ()>;

pub type PFN_vkCmdSetDepthBoundsTestEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthBoundsTestEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthClampEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthClampRangeEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        DepthClampModeEXT,
        *const DepthClampRangeEXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdSetDepthClipEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthCompareOp =
    Option<unsafe extern "system" fn(CommandBuffer, CompareOp) -> ()>;

pub type PFN_vkCmdSetDepthCompareOpEXT =
    Option<unsafe extern "system" fn(CommandBuffer, CompareOp) -> ()>;

pub type PFN_vkCmdSetDepthTestEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthTestEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthWriteEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDepthWriteEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDescriptorBufferOffsets2EXT = Option<
    unsafe extern "system" fn(CommandBuffer, *const SetDescriptorBufferOffsetsInfoEXT<'_>) -> (),
>;

pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        PipelineBindPoint,
        PipelineLayout,
        u32,
        u32,
        *const u32,
        *const u64,
    ) -> (),
>;

pub type PFN_vkCmdSetDeviceMask = Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetDeviceMaskKHR = Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetDiscardRectangleEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Rect2D<'_>) -> ()>;

pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetDiscardRectangleModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, DiscardRectangleModeEXT) -> ()>;

pub type PFN_vkCmdSetDispatchParametersARM =
    Option<unsafe extern "system" fn(CommandBuffer, *const DispatchParametersARM<'_>) -> ()>;

pub type PFN_vkCmdSetEvent =
    Option<unsafe extern "system" fn(CommandBuffer, Event, PipelineStageFlags) -> ()>;

pub type PFN_vkCmdSetEvent2 =
    Option<unsafe extern "system" fn(CommandBuffer, Event, *const DependencyInfo<'_>) -> ()>;

pub type PFN_vkCmdSetEvent2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, Event, *const DependencyInfo<'_>) -> ()>;

pub type PFN_vkCmdSetExclusiveScissorEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Bool32) -> ()>;

pub type PFN_vkCmdSetExclusiveScissorNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Rect2D<'_>) -> ()>;

pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, f32) -> ()>;

pub type PFN_vkCmdSetFragmentShadingRateEnumNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        FragmentShadingRateNV,
        [FragmentShadingRateCombinerOpKHR; 2],
    ) -> (),
>;

pub type PFN_vkCmdSetFragmentShadingRateKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const Extent2D<'_>,
        [FragmentShadingRateCombinerOpKHR; 2],
    ) -> (),
>;

pub type PFN_vkCmdSetFrontFace = Option<unsafe extern "system" fn(CommandBuffer, FrontFace) -> ()>;

pub type PFN_vkCmdSetFrontFaceEXT =
    Option<unsafe extern "system" fn(CommandBuffer, FrontFace) -> ()>;

pub type PFN_vkCmdSetLineRasterizationModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, LineRasterizationMode) -> ()>;

pub type PFN_vkCmdSetLineStipple = Option<unsafe extern "system" fn(CommandBuffer, u32, u16) -> ()>;

pub type PFN_vkCmdSetLineStippleEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u16) -> ()>;

pub type PFN_vkCmdSetLineStippleEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetLineStippleKHR =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u16) -> ()>;

pub type PFN_vkCmdSetLineWidth = Option<unsafe extern "system" fn(CommandBuffer, f32) -> ()>;

pub type PFN_vkCmdSetLogicOpEXT = Option<unsafe extern "system" fn(CommandBuffer, LogicOp) -> ()>;

pub type PFN_vkCmdSetLogicOpEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetPatchControlPointsEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetPerformanceMarkerINTEL = Option<
    unsafe extern "system" fn(CommandBuffer, *const PerformanceMarkerInfoINTEL<'_>) -> VkResult,
>;

pub type PFN_vkCmdSetPerformanceOverrideINTEL = Option<
    unsafe extern "system" fn(CommandBuffer, *const PerformanceOverrideInfoINTEL<'_>) -> VkResult,
>;

pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const PerformanceStreamMarkerInfoINTEL<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCmdSetPolygonModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, PolygonMode) -> ()>;

pub type PFN_vkCmdSetPrimitiveRestartEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetPrimitiveRestartEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetPrimitiveRestartIndexEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetPrimitiveTopology =
    Option<unsafe extern "system" fn(CommandBuffer, PrimitiveTopology) -> ()>;

pub type PFN_vkCmdSetPrimitiveTopologyEXT =
    Option<unsafe extern "system" fn(CommandBuffer, PrimitiveTopology) -> ()>;

pub type PFN_vkCmdSetProvokingVertexModeEXT =
    Option<unsafe extern "system" fn(CommandBuffer, ProvokingVertexModeEXT) -> ()>;

pub type PFN_vkCmdSetRasterizationSamplesEXT =
    Option<unsafe extern "system" fn(CommandBuffer, SampleCountFlagBits) -> ()>;

pub type PFN_vkCmdSetRasterizationStreamEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetRasterizerDiscardEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetRasterizerDiscardEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR =
    Option<unsafe extern "system" fn(CommandBuffer, u32) -> ()>;

pub type PFN_vkCmdSetRenderingAttachmentLocations = Option<
    unsafe extern "system" fn(CommandBuffer, *const RenderingAttachmentLocationInfo<'_>) -> (),
>;

pub type PFN_vkCmdSetRenderingAttachmentLocationsKHR = Option<
    unsafe extern "system" fn(CommandBuffer, *const RenderingAttachmentLocationInfo<'_>) -> (),
>;

pub type PFN_vkCmdSetRenderingInputAttachmentIndices = Option<
    unsafe extern "system" fn(CommandBuffer, *const RenderingInputAttachmentIndexInfo<'_>) -> (),
>;

pub type PFN_vkCmdSetRenderingInputAttachmentIndicesKHR = Option<
    unsafe extern "system" fn(CommandBuffer, *const RenderingInputAttachmentIndexInfo<'_>) -> (),
>;

pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetSampleLocationsEXT =
    Option<unsafe extern "system" fn(CommandBuffer, *const SampleLocationsInfoEXT<'_>) -> ()>;

pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetSampleMaskEXT =
    Option<unsafe extern "system" fn(CommandBuffer, SampleCountFlagBits, *const u32) -> ()>;

pub type PFN_vkCmdSetScissor =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Rect2D<'_>) -> ()>;

pub type PFN_vkCmdSetScissorWithCount =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const Rect2D<'_>) -> ()>;

pub type PFN_vkCmdSetScissorWithCountEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const Rect2D<'_>) -> ()>;

pub type PFN_vkCmdSetShadingRateImageEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetStencilCompareMask =
    Option<unsafe extern "system" fn(CommandBuffer, StencilFaceFlags, u32) -> ()>;

pub type PFN_vkCmdSetStencilOp = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        StencilFaceFlags,
        StencilOp,
        StencilOp,
        StencilOp,
        CompareOp,
    ) -> (),
>;

pub type PFN_vkCmdSetStencilOpEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        StencilFaceFlags,
        StencilOp,
        StencilOp,
        StencilOp,
        CompareOp,
    ) -> (),
>;

pub type PFN_vkCmdSetStencilReference =
    Option<unsafe extern "system" fn(CommandBuffer, StencilFaceFlags, u32) -> ()>;

pub type PFN_vkCmdSetStencilTestEnable =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetStencilTestEnableEXT =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetStencilWriteMask =
    Option<unsafe extern "system" fn(CommandBuffer, StencilFaceFlags, u32) -> ()>;

pub type PFN_vkCmdSetTessellationDomainOriginEXT =
    Option<unsafe extern "system" fn(CommandBuffer, TessellationDomainOrigin) -> ()>;

pub type PFN_vkCmdSetVertexInputEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const VertexInputBindingDescription2EXT<'_>,
        u32,
        *const VertexInputAttributeDescription2EXT<'_>,
    ) -> (),
>;

pub type PFN_vkCmdSetViewport =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const Viewport<'_>) -> ()>;

pub type PFN_vkCmdSetViewportShadingRatePaletteNV = Option<
    unsafe extern "system" fn(CommandBuffer, u32, u32, *const ShadingRatePaletteNV<'_>) -> (),
>;

pub type PFN_vkCmdSetViewportSwizzleNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const ViewportSwizzleNV<'_>) -> ()>;

pub type PFN_vkCmdSetViewportWScalingEnableNV =
    Option<unsafe extern "system" fn(CommandBuffer, Bool32) -> ()>;

pub type PFN_vkCmdSetViewportWScalingNV =
    Option<unsafe extern "system" fn(CommandBuffer, u32, u32, *const ViewportWScalingNV<'_>) -> ()>;

pub type PFN_vkCmdSetViewportWithCount =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const Viewport<'_>) -> ()>;

pub type PFN_vkCmdSetViewportWithCountEXT =
    Option<unsafe extern "system" fn(CommandBuffer, u32, *const Viewport<'_>) -> ()>;

pub type PFN_vkCmdSubpassShadingHUAWEI = Option<unsafe extern "system" fn(CommandBuffer) -> ()>;

pub type PFN_vkCmdTraceRaysIndirect2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, u64) -> ()>;

pub type PFN_vkCmdTraceRaysIndirectKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        u64,
    ) -> (),
>;

pub type PFN_vkCmdTraceRaysKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        *const StridedDeviceAddressRegionKHR<'_>,
        u32,
        u32,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdTraceRaysNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        Buffer,
        u64,
        Buffer,
        u64,
        u64,
        Buffer,
        u64,
        u64,
        Buffer,
        u64,
        u64,
        u32,
        u32,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdUpdateBuffer =
    Option<unsafe extern "system" fn(CommandBuffer, Buffer, u64, u64, *const c_void) -> ()>;

pub type PFN_vkCmdUpdateMemoryKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        *const DeviceAddressRangeKHR<'_>,
        AddressCommandFlagsKHR,
        u64,
        *const c_void,
    ) -> (),
>;

pub type PFN_vkCmdUpdatePipelineIndirectBufferNV =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineBindPoint, Pipeline) -> ()>;

pub type PFN_vkCmdWaitEvents = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const Event,
        PipelineStageFlags,
        PipelineStageFlags,
        u32,
        *const MemoryBarrier<'_>,
        u32,
        *const BufferMemoryBarrier<'_>,
        u32,
        *const ImageMemoryBarrier<'_>,
    ) -> (),
>;

pub type PFN_vkCmdWaitEvents2 = Option<
    unsafe extern "system" fn(CommandBuffer, u32, *const Event, *const DependencyInfo<'_>) -> (),
>;

pub type PFN_vkCmdWaitEvents2KHR = Option<
    unsafe extern "system" fn(CommandBuffer, u32, *const Event, *const DependencyInfo<'_>) -> (),
>;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const AccelerationStructureKHR,
        QueryType,
        QueryPool,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const AccelerationStructureNV,
        QueryType,
        QueryPool,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdWriteBufferMarker2AMD =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineStageFlags2, Buffer, u64, u32) -> ()>;

pub type PFN_vkCmdWriteBufferMarkerAMD =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineStageFlagBits, Buffer, u64, u32) -> ()>;

pub type PFN_vkCmdWriteMarkerToMemoryAMD =
    Option<unsafe extern "system" fn(CommandBuffer, *const MemoryMarkerInfoAMD<'_>) -> ()>;

pub type PFN_vkCmdWriteMicromapsPropertiesEXT = Option<
    unsafe extern "system" fn(
        CommandBuffer,
        u32,
        *const MicromapEXT,
        QueryType,
        QueryPool,
        u32,
    ) -> (),
>;

pub type PFN_vkCmdWriteTimestamp =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineStageFlagBits, QueryPool, u32) -> ()>;

pub type PFN_vkCmdWriteTimestamp2 =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineStageFlags2, QueryPool, u32) -> ()>;

pub type PFN_vkCmdWriteTimestamp2KHR =
    Option<unsafe extern "system" fn(CommandBuffer, PipelineStageFlags2, QueryPool, u32) -> ()>;

pub type PFN_vkCompileDeferredNV =
    Option<unsafe extern "system" fn(Device, Pipeline, u32) -> VkResult>;

pub type PFN_vkConvertCooperativeVectorMatrixNV = Option<
    unsafe extern "system" fn(Device, *const ConvertCooperativeVectorMatrixInfoNV<'_>) -> VkResult,
>;

pub type PFN_vkCopyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyAccelerationStructureInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCopyAccelerationStructureToMemoryKHR = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyAccelerationStructureToMemoryInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCopyImageToImage =
    Option<unsafe extern "system" fn(Device, *const CopyImageToImageInfo<'_>) -> VkResult>;

pub type PFN_vkCopyImageToImageEXT =
    Option<unsafe extern "system" fn(Device, *const CopyImageToImageInfo<'_>) -> VkResult>;

pub type PFN_vkCopyImageToMemory =
    Option<unsafe extern "system" fn(Device, *const CopyImageToMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkCopyImageToMemoryEXT =
    Option<unsafe extern "system" fn(Device, *const CopyImageToMemoryInfo<'_>) -> VkResult>;

pub type PFN_vkCopyMemoryToAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyMemoryToAccelerationStructureInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCopyMemoryToImage =
    Option<unsafe extern "system" fn(Device, *const CopyMemoryToImageInfo<'_>) -> VkResult>;

pub type PFN_vkCopyMemoryToImageEXT =
    Option<unsafe extern "system" fn(Device, *const CopyMemoryToImageInfo<'_>) -> VkResult>;

pub type PFN_vkCopyMemoryToMicromapEXT = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyMemoryToMicromapInfoEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCopyMicromapEXT = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyMicromapInfoEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCopyMicromapToMemoryEXT = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        *const CopyMicromapToMemoryInfoEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCreateAccelerationStructure2KHR = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureCreateInfo2KHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut AccelerationStructureKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut AccelerationStructureKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut AccelerationStructureNV,
    ) -> VkResult,
>;

#[cfg(feature = "android")]
pub type PFN_vkCreateAndroidSurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const AndroidSurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateBuffer = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Buffer,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkCreateBufferCollectionFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferCollectionCreateInfoFUCHSIA<'_>,
        *const AllocationCallbacks<'_>,
        *mut BufferCollectionFUCHSIA,
    ) -> VkResult,
>;

pub type PFN_vkCreateBufferView = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferViewCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut BufferView,
    ) -> VkResult,
>;

pub type PFN_vkCreateCommandPool = Option<
    unsafe extern "system" fn(
        Device,
        *const CommandPoolCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut CommandPool,
    ) -> VkResult,
>;

pub type PFN_vkCreateComputePipelines = Option<
    unsafe extern "system" fn(
        Device,
        PipelineCache,
        u32,
        *const ComputePipelineCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateCuFunctionNVX = Option<
    unsafe extern "system" fn(
        Device,
        *const CuFunctionCreateInfoNVX<'_>,
        *const AllocationCallbacks<'_>,
        *mut CuFunctionNVX,
    ) -> VkResult,
>;

pub type PFN_vkCreateCuModuleNVX = Option<
    unsafe extern "system" fn(
        Device,
        *const CuModuleCreateInfoNVX<'_>,
        *const AllocationCallbacks<'_>,
        *mut CuModuleNVX,
    ) -> VkResult,
>;

#[cfg(feature = "beta")]
pub type PFN_vkCreateCudaFunctionNV = Option<
    unsafe extern "system" fn(
        Device,
        *const CudaFunctionCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut CudaFunctionNV,
    ) -> VkResult,
>;

#[cfg(feature = "beta")]
pub type PFN_vkCreateCudaModuleNV = Option<
    unsafe extern "system" fn(
        Device,
        *const CudaModuleCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut CudaModuleNV,
    ) -> VkResult,
>;

pub type PFN_vkCreateDataGraphPipelineSessionARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DataGraphPipelineSessionCreateInfoARM<'_>,
        *const AllocationCallbacks<'_>,
        *mut DataGraphPipelineSessionARM,
    ) -> VkResult,
>;

pub type PFN_vkCreateDataGraphPipelinesARM = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        PipelineCache,
        u32,
        *const DataGraphPipelineCreateInfoARM<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        Instance,
        *const DebugReportCallbackCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut DebugReportCallbackEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateDebugUtilsMessengerEXT = Option<
    unsafe extern "system" fn(
        Instance,
        *const DebugUtilsMessengerCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut DebugUtilsMessengerEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateDeferredOperationKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const AllocationCallbacks<'_>,
        *mut DeferredOperationKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateDescriptorPool = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorPoolCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut DescriptorPool,
    ) -> VkResult,
>;

pub type PFN_vkCreateDescriptorSetLayout = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorSetLayoutCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut DescriptorSetLayout,
    ) -> VkResult,
>;

pub type PFN_vkCreateDescriptorUpdateTemplate = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorUpdateTemplateCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut DescriptorUpdateTemplate,
    ) -> VkResult,
>;

pub type PFN_vkCreateDescriptorUpdateTemplateKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorUpdateTemplateCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut DescriptorUpdateTemplate,
    ) -> VkResult,
>;

pub type PFN_vkCreateDevice = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const DeviceCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Device,
    ) -> VkResult,
>;

#[cfg(feature = "directfb")]
pub type PFN_vkCreateDirectFBSurfaceEXT = Option<
    unsafe extern "system" fn(
        Instance,
        *const DirectFBSurfaceCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateDisplayModeKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        DisplayKHR,
        *const DisplayModeCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut DisplayModeKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateDisplayPlaneSurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const DisplaySurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateEvent = Option<
    unsafe extern "system" fn(
        Device,
        *const EventCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Event,
    ) -> VkResult,
>;

#[cfg(feature = "beta")]
pub type PFN_vkCreateExecutionGraphPipelinesAMDX = Option<
    unsafe extern "system" fn(
        Device,
        PipelineCache,
        u32,
        *const ExecutionGraphPipelineCreateInfoAMDX<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateExternalComputeQueueNV = Option<
    unsafe extern "system" fn(
        Device,
        *const ExternalComputeQueueCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut ExternalComputeQueueNV,
    ) -> VkResult,
>;

pub type PFN_vkCreateFence = Option<
    unsafe extern "system" fn(
        Device,
        *const FenceCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Fence,
    ) -> VkResult,
>;

pub type PFN_vkCreateFramebuffer = Option<
    unsafe extern "system" fn(
        Device,
        *const FramebufferCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Framebuffer,
    ) -> VkResult,
>;

pub type PFN_vkCreateGraphicsPipelines = Option<
    unsafe extern "system" fn(
        Device,
        PipelineCache,
        u32,
        *const GraphicsPipelineCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateHeadlessSurfaceEXT = Option<
    unsafe extern "system" fn(
        Instance,
        *const HeadlessSurfaceCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "ios")]
pub type PFN_vkCreateIOSSurfaceMVK = Option<
    unsafe extern "system" fn(
        Instance,
        *const IOSSurfaceCreateInfoMVK<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateImage = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Image,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = Option<
    unsafe extern "system" fn(
        Instance,
        *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateImageView = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageViewCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut ImageView,
    ) -> VkResult,
>;

pub type PFN_vkCreateIndirectCommandsLayoutEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const IndirectCommandsLayoutCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut IndirectCommandsLayoutEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateIndirectCommandsLayoutNV = Option<
    unsafe extern "system" fn(
        Device,
        *const IndirectCommandsLayoutCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut IndirectCommandsLayoutNV,
    ) -> VkResult,
>;

pub type PFN_vkCreateIndirectExecutionSetEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const IndirectExecutionSetCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut IndirectExecutionSetEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateInstance = Option<
    unsafe extern "system" fn(
        *const InstanceCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Instance,
    ) -> VkResult,
>;

#[cfg(feature = "macos")]
pub type PFN_vkCreateMacOSSurfaceMVK = Option<
    unsafe extern "system" fn(
        Instance,
        *const MacOSSurfaceCreateInfoMVK<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "metal")]
pub type PFN_vkCreateMetalSurfaceEXT = Option<
    unsafe extern "system" fn(
        Instance,
        *const MetalSurfaceCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateMicromapEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const MicromapCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut MicromapEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateOpticalFlowSessionNV = Option<
    unsafe extern "system" fn(
        Device,
        *const OpticalFlowSessionCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut OpticalFlowSessionNV,
    ) -> VkResult,
>;

pub type PFN_vkCreatePipelineBinariesKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineBinaryCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkCreatePipelineCache = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineCacheCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut PipelineCache,
    ) -> VkResult,
>;

pub type PFN_vkCreatePipelineLayout = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineLayoutCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut PipelineLayout,
    ) -> VkResult,
>;

pub type PFN_vkCreatePrivateDataSlot = Option<
    unsafe extern "system" fn(
        Device,
        *const PrivateDataSlotCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut PrivateDataSlot,
    ) -> VkResult,
>;

pub type PFN_vkCreatePrivateDataSlotEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const PrivateDataSlotCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut PrivateDataSlot,
    ) -> VkResult,
>;

pub type PFN_vkCreateQueryPool = Option<
    unsafe extern "system" fn(
        Device,
        *const QueryPoolCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut QueryPool,
    ) -> VkResult,
>;

pub type PFN_vkCreateRayTracingPipelinesKHR = Option<
    unsafe extern "system" fn(
        Device,
        DeferredOperationKHR,
        PipelineCache,
        u32,
        *const RayTracingPipelineCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateRayTracingPipelinesNV = Option<
    unsafe extern "system" fn(
        Device,
        PipelineCache,
        u32,
        *const RayTracingPipelineCreateInfoNV<'_>,
        *const AllocationCallbacks<'_>,
        *mut Pipeline,
    ) -> VkResult,
>;

pub type PFN_vkCreateRenderPass = Option<
    unsafe extern "system" fn(
        Device,
        *const RenderPassCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut RenderPass,
    ) -> VkResult,
>;

pub type PFN_vkCreateRenderPass2 = Option<
    unsafe extern "system" fn(
        Device,
        *const RenderPassCreateInfo2<'_>,
        *const AllocationCallbacks<'_>,
        *mut RenderPass,
    ) -> VkResult,
>;

pub type PFN_vkCreateRenderPass2KHR = Option<
    unsafe extern "system" fn(
        Device,
        *const RenderPassCreateInfo2<'_>,
        *const AllocationCallbacks<'_>,
        *mut RenderPass,
    ) -> VkResult,
>;

pub type PFN_vkCreateSampler = Option<
    unsafe extern "system" fn(
        Device,
        *const SamplerCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Sampler,
    ) -> VkResult,
>;

pub type PFN_vkCreateSamplerYcbcrConversion = Option<
    unsafe extern "system" fn(
        Device,
        *const SamplerYcbcrConversionCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut SamplerYcbcrConversion,
    ) -> VkResult,
>;

pub type PFN_vkCreateSamplerYcbcrConversionKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const SamplerYcbcrConversionCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut SamplerYcbcrConversion,
    ) -> VkResult,
>;

#[cfg(feature = "screen")]
pub type PFN_vkCreateScreenSurfaceQNX = Option<
    unsafe extern "system" fn(
        Instance,
        *const ScreenSurfaceCreateInfoQNX<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateSemaphore = Option<
    unsafe extern "system" fn(
        Device,
        *const SemaphoreCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut Semaphore,
    ) -> VkResult,
>;

pub type PFN_vkCreateShaderInstrumentationARM = Option<
    unsafe extern "system" fn(
        Device,
        *const ShaderInstrumentationCreateInfoARM<'_>,
        *const AllocationCallbacks<'_>,
        *mut ShaderInstrumentationARM,
    ) -> VkResult,
>;

pub type PFN_vkCreateShaderModule = Option<
    unsafe extern "system" fn(
        Device,
        *const ShaderModuleCreateInfo<'_>,
        *const AllocationCallbacks<'_>,
        *mut ShaderModule,
    ) -> VkResult,
>;

pub type PFN_vkCreateShadersEXT = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const ShaderCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut ShaderEXT,
    ) -> VkResult,
>;

pub type PFN_vkCreateSharedSwapchainsKHR = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const SwapchainCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SwapchainKHR,
    ) -> VkResult,
>;

#[cfg(feature = "ggp")]
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = Option<
    unsafe extern "system" fn(
        Instance,
        *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "ohos")]
pub type PFN_vkCreateSurfaceOHOS = Option<
    unsafe extern "system" fn(
        Instance,
        *const SurfaceCreateInfoOHOS<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateSwapchainKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const SwapchainCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SwapchainKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateTensorARM = Option<
    unsafe extern "system" fn(
        Device,
        *const TensorCreateInfoARM<'_>,
        *const AllocationCallbacks<'_>,
        *mut TensorARM,
    ) -> VkResult,
>;

pub type PFN_vkCreateTensorViewARM = Option<
    unsafe extern "system" fn(
        Device,
        *const TensorViewCreateInfoARM<'_>,
        *const AllocationCallbacks<'_>,
        *mut TensorViewARM,
    ) -> VkResult,
>;

#[cfg(feature = "ubm")]
pub type PFN_vkCreateUbmSurfaceSEC = Option<
    unsafe extern "system" fn(
        Instance,
        *const UbmSurfaceCreateInfoSEC<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateValidationCacheEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const ValidationCacheCreateInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut ValidationCacheEXT,
    ) -> VkResult,
>;

#[cfg(feature = "vi")]
pub type PFN_vkCreateViSurfaceNN = Option<
    unsafe extern "system" fn(
        Instance,
        *const ViSurfaceCreateInfoNN<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateVideoSessionKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const VideoSessionCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut VideoSessionKHR,
    ) -> VkResult,
>;

pub type PFN_vkCreateVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const VideoSessionParametersCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut VideoSessionParametersKHR,
    ) -> VkResult,
>;

#[cfg(feature = "wayland")]
pub type PFN_vkCreateWaylandSurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const WaylandSurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkCreateWin32SurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const Win32SurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "xcb")]
pub type PFN_vkCreateXcbSurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const XcbSurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

#[cfg(feature = "xlib")]
pub type PFN_vkCreateXlibSurfaceKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *const XlibSurfaceCreateInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
        *mut SurfaceKHR,
    ) -> VkResult,
>;

pub type PFN_vkDebugMarkerSetObjectNameEXT =
    Option<unsafe extern "system" fn(Device, *const DebugMarkerObjectNameInfoEXT<'_>) -> VkResult>;

pub type PFN_vkDebugMarkerSetObjectTagEXT =
    Option<unsafe extern "system" fn(Device, *const DebugMarkerObjectTagInfoEXT<'_>) -> VkResult>;

pub type PFN_vkDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        DebugReportFlagsEXT,
        DebugReportObjectTypeEXT,
        u64,
        usize,
        i32,
        *const c_char,
        *const c_char,
        *mut c_void,
    ) -> Bool32,
>;

pub type PFN_vkDebugReportMessageEXT = Option<
    unsafe extern "system" fn(
        Instance,
        DebugReportFlagsEXT,
        DebugReportObjectTypeEXT,
        u64,
        usize,
        i32,
        *const c_char,
        *const c_char,
    ) -> (),
>;

pub type PFN_vkDebugUtilsMessengerCallbackEXT = Option<
    unsafe extern "system" fn(
        DebugUtilsMessageSeverityFlagBitsEXT,
        DebugUtilsMessageTypeFlagsEXT,
        *const DebugUtilsMessengerCallbackDataEXT<'_>,
        *mut c_void,
    ) -> Bool32,
>;

pub type PFN_vkDeferredOperationJoinKHR =
    Option<unsafe extern "system" fn(Device, DeferredOperationKHR) -> VkResult>;

pub type PFN_vkDestroyAccelerationStructureKHR = Option<
    unsafe extern "system" fn(
        Device,
        AccelerationStructureKHR,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyAccelerationStructureNV = Option<
    unsafe extern "system" fn(
        Device,
        AccelerationStructureNV,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyBuffer =
    Option<unsafe extern "system" fn(Device, Buffer, *const AllocationCallbacks<'_>) -> ()>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkDestroyBufferCollectionFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        BufferCollectionFUCHSIA,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyBufferView =
    Option<unsafe extern "system" fn(Device, BufferView, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyCommandPool =
    Option<unsafe extern "system" fn(Device, CommandPool, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyCuFunctionNVX =
    Option<unsafe extern "system" fn(Device, CuFunctionNVX, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyCuModuleNVX =
    Option<unsafe extern "system" fn(Device, CuModuleNVX, *const AllocationCallbacks<'_>) -> ()>;

#[cfg(feature = "beta")]
pub type PFN_vkDestroyCudaFunctionNV =
    Option<unsafe extern "system" fn(Device, CudaFunctionNV, *const AllocationCallbacks<'_>) -> ()>;

#[cfg(feature = "beta")]
pub type PFN_vkDestroyCudaModuleNV =
    Option<unsafe extern "system" fn(Device, CudaModuleNV, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyDataGraphPipelineSessionARM = Option<
    unsafe extern "system" fn(
        Device,
        DataGraphPipelineSessionARM,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyDebugReportCallbackEXT = Option<
    unsafe extern "system" fn(
        Instance,
        DebugReportCallbackEXT,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyDebugUtilsMessengerEXT = Option<
    unsafe extern "system" fn(
        Instance,
        DebugUtilsMessengerEXT,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyDeferredOperationKHR = Option<
    unsafe extern "system" fn(Device, DeferredOperationKHR, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyDescriptorPool =
    Option<unsafe extern "system" fn(Device, DescriptorPool, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyDescriptorSetLayout = Option<
    unsafe extern "system" fn(Device, DescriptorSetLayout, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyDescriptorUpdateTemplate = Option<
    unsafe extern "system" fn(
        Device,
        DescriptorUpdateTemplate,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = Option<
    unsafe extern "system" fn(
        Device,
        DescriptorUpdateTemplate,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyDevice =
    Option<unsafe extern "system" fn(Device, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyEvent =
    Option<unsafe extern "system" fn(Device, Event, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyExternalComputeQueueNV = Option<
    unsafe extern "system" fn(Device, ExternalComputeQueueNV, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyFence =
    Option<unsafe extern "system" fn(Device, Fence, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyFramebuffer =
    Option<unsafe extern "system" fn(Device, Framebuffer, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyImage =
    Option<unsafe extern "system" fn(Device, Image, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyImageView =
    Option<unsafe extern "system" fn(Device, ImageView, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyIndirectCommandsLayoutEXT = Option<
    unsafe extern "system" fn(
        Device,
        IndirectCommandsLayoutEXT,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyIndirectCommandsLayoutNV = Option<
    unsafe extern "system" fn(
        Device,
        IndirectCommandsLayoutNV,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyIndirectExecutionSetEXT = Option<
    unsafe extern "system" fn(
        Device,
        IndirectExecutionSetEXT,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyInstance =
    Option<unsafe extern "system" fn(Instance, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyMicromapEXT =
    Option<unsafe extern "system" fn(Device, MicromapEXT, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyOpticalFlowSessionNV = Option<
    unsafe extern "system" fn(Device, OpticalFlowSessionNV, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyPipeline =
    Option<unsafe extern "system" fn(Device, Pipeline, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyPipelineBinaryKHR = Option<
    unsafe extern "system" fn(Device, PipelineBinaryKHR, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyPipelineCache =
    Option<unsafe extern "system" fn(Device, PipelineCache, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyPipelineLayout =
    Option<unsafe extern "system" fn(Device, PipelineLayout, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyPrivateDataSlot = Option<
    unsafe extern "system" fn(Device, PrivateDataSlot, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyPrivateDataSlotEXT = Option<
    unsafe extern "system" fn(Device, PrivateDataSlot, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyQueryPool =
    Option<unsafe extern "system" fn(Device, QueryPool, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyRenderPass =
    Option<unsafe extern "system" fn(Device, RenderPass, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroySampler =
    Option<unsafe extern "system" fn(Device, Sampler, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroySamplerYcbcrConversion = Option<
    unsafe extern "system" fn(Device, SamplerYcbcrConversion, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroySamplerYcbcrConversionKHR = Option<
    unsafe extern "system" fn(Device, SamplerYcbcrConversion, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroySemaphore =
    Option<unsafe extern "system" fn(Device, Semaphore, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyShaderEXT =
    Option<unsafe extern "system" fn(Device, ShaderEXT, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyShaderInstrumentationARM = Option<
    unsafe extern "system" fn(
        Device,
        ShaderInstrumentationARM,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDestroyShaderModule =
    Option<unsafe extern "system" fn(Device, ShaderModule, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroySurfaceKHR =
    Option<unsafe extern "system" fn(Instance, SurfaceKHR, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroySwapchainKHR =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyTensorARM =
    Option<unsafe extern "system" fn(Device, TensorARM, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyTensorViewARM =
    Option<unsafe extern "system" fn(Device, TensorViewARM, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkDestroyValidationCacheEXT = Option<
    unsafe extern "system" fn(Device, ValidationCacheEXT, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyVideoSessionKHR = Option<
    unsafe extern "system" fn(Device, VideoSessionKHR, *const AllocationCallbacks<'_>) -> (),
>;

pub type PFN_vkDestroyVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        Device,
        VideoSessionParametersKHR,
        *const AllocationCallbacks<'_>,
    ) -> (),
>;

pub type PFN_vkDeviceMemoryReportCallbackEXT = Option<
    unsafe extern "system" fn(*const DeviceMemoryReportCallbackDataEXT<'_>, *mut c_void) -> (),
>;

pub type PFN_vkDeviceWaitIdle = Option<unsafe extern "system" fn(Device) -> VkResult>;

pub type PFN_vkDisplayPowerControlEXT = Option<
    unsafe extern "system" fn(Device, DisplayKHR, *const DisplayPowerInfoEXT<'_>) -> VkResult,
>;

pub type PFN_vkEndCommandBuffer = Option<unsafe extern "system" fn(CommandBuffer) -> VkResult>;

pub type PFN_vkEnumerateDeviceExtensionProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const c_char,
        *mut u32,
        *mut ExtensionProperties<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumerateDeviceLayerProperties = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut LayerProperties<'_>) -> VkResult,
>;

pub type PFN_vkEnumerateInstanceExtensionProperties = Option<
    unsafe extern "system" fn(*const c_char, *mut u32, *mut ExtensionProperties<'_>) -> VkResult,
>;

pub type PFN_vkEnumerateInstanceLayerProperties =
    Option<unsafe extern "system" fn(*mut u32, *mut LayerProperties<'_>) -> VkResult>;

pub type PFN_vkEnumerateInstanceVersion = Option<unsafe extern "system" fn(*mut u32) -> VkResult>;

pub type PFN_vkEnumeratePhysicalDeviceGroups = Option<
    unsafe extern "system" fn(
        Instance,
        *mut u32,
        *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR = Option<
    unsafe extern "system" fn(
        Instance,
        *mut u32,
        *mut PhysicalDeviceGroupProperties<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        u32,
        *mut u32,
        *mut PerformanceCounterARM<'_>,
        *mut PerformanceCounterDescriptionARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        u32,
        *mut u32,
        *mut PerformanceCounterKHR<'_>,
        *mut PerformanceCounterDescriptionKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut ShaderInstrumentationMetricDescriptionARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkEnumeratePhysicalDevices =
    Option<unsafe extern "system" fn(Instance, *mut u32, *mut PhysicalDevice) -> VkResult>;

#[cfg(feature = "metal")]
pub type PFN_vkExportMetalObjectsEXT =
    Option<unsafe extern "system" fn(Device, *mut ExportMetalObjectsInfoEXT<'_>) -> ()>;

pub type PFN_vkFlushMappedMemoryRanges =
    Option<unsafe extern "system" fn(Device, u32, *const MappedMemoryRange<'_>) -> VkResult>;

pub type PFN_vkFreeCommandBuffers =
    Option<unsafe extern "system" fn(Device, CommandPool, u32, *const CommandBuffer) -> ()>;

pub type PFN_vkFreeDescriptorSets = Option<
    unsafe extern "system" fn(Device, DescriptorPool, u32, *const DescriptorSet) -> VkResult,
>;

pub type PFN_vkFreeFunction = Option<unsafe extern "system" fn(*mut c_void, *mut c_void) -> ()>;

pub type PFN_vkFreeMemory =
    Option<unsafe extern "system" fn(Device, DeviceMemory, *const AllocationCallbacks<'_>) -> ()>;

pub type PFN_vkGetAccelerationStructureBuildSizesKHR = Option<
    unsafe extern "system" fn(
        Device,
        AccelerationStructureBuildTypeKHR,
        *const AccelerationStructureBuildGeometryInfoKHR<'_>,
        *const u32,
        *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) -> (),
>;

pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = Option<
    unsafe extern "system" fn(Device, *const AccelerationStructureDeviceAddressInfoKHR<'_>) -> u64,
>;

pub type PFN_vkGetAccelerationStructureHandleNV = Option<
    unsafe extern "system" fn(Device, AccelerationStructureNV, usize, *mut c_void) -> VkResult,
>;

pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureMemoryRequirementsInfoNV<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        *mut c_void,
    ) -> VkResult,
>;

#[cfg(feature = "android")]
pub type PFN_vkGetAndroidHardwareBufferPropertiesANDROID = Option<
    unsafe extern "system" fn(
        Device,
        *const AHardwareBuffer,
        *mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkGetBufferCollectionPropertiesFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        BufferCollectionFUCHSIA,
        *mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetBufferDeviceAddress =
    Option<unsafe extern "system" fn(Device, *const BufferDeviceAddressInfo<'_>) -> u64>;

pub type PFN_vkGetBufferDeviceAddressEXT =
    Option<unsafe extern "system" fn(Device, *const BufferDeviceAddressInfo<'_>) -> u64>;

pub type PFN_vkGetBufferDeviceAddressKHR =
    Option<unsafe extern "system" fn(Device, *const BufferDeviceAddressInfo<'_>) -> u64>;

pub type PFN_vkGetBufferMemoryRequirements =
    Option<unsafe extern "system" fn(Device, Buffer, *mut MemoryRequirements<'_>) -> ()>;

pub type PFN_vkGetBufferMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferMemoryRequirementsInfo2<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetBufferMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferMemoryRequirementsInfo2<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetBufferOpaqueCaptureAddress =
    Option<unsafe extern "system" fn(Device, *const BufferDeviceAddressInfo<'_>) -> u64>;

pub type PFN_vkGetBufferOpaqueCaptureAddressKHR =
    Option<unsafe extern "system" fn(Device, *const BufferDeviceAddressInfo<'_>) -> u64>;

pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const BufferCaptureDescriptorDataInfoEXT<'_>,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetCalibratedTimestampsEXT = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const CalibratedTimestampInfoKHR<'_>,
        *mut u64,
        *mut u64,
    ) -> VkResult,
>;

pub type PFN_vkGetCalibratedTimestampsKHR = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const CalibratedTimestampInfoKHR<'_>,
        *mut u64,
        *mut u64,
    ) -> VkResult,
>;

pub type PFN_vkGetClusterAccelerationStructureBuildSizesNV = Option<
    unsafe extern "system" fn(
        Device,
        *const ClusterAccelerationStructureInputInfoNV<'_>,
        *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) -> (),
>;

#[cfg(feature = "beta")]
pub type PFN_vkGetCudaModuleCacheNV =
    Option<unsafe extern "system" fn(Device, CudaModuleNV, *mut usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DataGraphPipelineInfoARM<'_>,
        *mut u32,
        *mut DataGraphPipelinePropertyARM,
    ) -> VkResult,
>;

pub type PFN_vkGetDataGraphPipelinePropertiesARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DataGraphPipelineInfoARM<'_>,
        u32,
        *mut DataGraphPipelinePropertyQueryResultARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
        *mut u32,
        *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR =
    Option<unsafe extern "system" fn(Device, DeferredOperationKHR) -> u32>;

pub type PFN_vkGetDeferredOperationResultKHR =
    Option<unsafe extern "system" fn(Device, DeferredOperationKHR) -> VkResult>;

pub type PFN_vkGetDescriptorEXT = Option<
    unsafe extern "system" fn(Device, *const DescriptorGetInfoEXT<'_>, usize, *mut c_void) -> (),
>;

pub type PFN_vkGetDescriptorSetHostMappingVALVE =
    Option<unsafe extern "system" fn(Device, DescriptorSet, *mut *mut c_void) -> ()>;

pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT =
    Option<unsafe extern "system" fn(Device, DescriptorSetLayout, u32, *mut u64) -> ()>;

pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorSetBindingReferenceVALVE<'_>,
        *mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ) -> (),
>;

pub type PFN_vkGetDescriptorSetLayoutSizeEXT =
    Option<unsafe extern "system" fn(Device, DescriptorSetLayout, *mut u64) -> ()>;

pub type PFN_vkGetDescriptorSetLayoutSupport = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorSetLayoutCreateInfo<'_>,
        *mut DescriptorSetLayoutSupport<'_>,
    ) -> (),
>;

pub type PFN_vkGetDescriptorSetLayoutSupportKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DescriptorSetLayoutCreateInfo<'_>,
        *mut DescriptorSetLayoutSupport<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const AccelerationStructureVersionInfoKHR<'_>,
        *mut AccelerationStructureCompatibilityKHR,
    ) -> (),
>;

pub type PFN_vkGetDeviceBufferMemoryRequirements = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceBufferMemoryRequirements<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceBufferMemoryRequirements<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceCombinedImageSamplerIndexNVX =
    Option<unsafe extern "system" fn(Device, u64, u64) -> u64>;

pub type PFN_vkGetDeviceFaultDebugInfoKHR =
    Option<unsafe extern "system" fn(Device, *mut DeviceFaultDebugInfoKHR<'_>) -> VkResult>;

pub type PFN_vkGetDeviceFaultInfoEXT = Option<
    unsafe extern "system" fn(
        Device,
        *mut DeviceFaultCountsEXT<'_>,
        *mut DeviceFaultInfoEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDeviceFaultReportsKHR = Option<
    unsafe extern "system" fn(Device, u64, *mut u32, *mut DeviceFaultInfoKHR<'_>) -> VkResult,
>;

pub type PFN_vkGetDeviceGroupPeerMemoryFeatures =
    Option<unsafe extern "system" fn(Device, u32, u32, u32, *mut PeerMemoryFeatureFlags) -> ()>;

pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR =
    Option<unsafe extern "system" fn(Device, u32, u32, u32, *mut PeerMemoryFeatureFlags) -> ()>;

pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = Option<
    unsafe extern "system" fn(Device, *mut DeviceGroupPresentCapabilitiesKHR<'_>) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetDeviceGroupSurfacePresentModes2EXT = Option<
    unsafe extern "system" fn(
        Device,
        *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        *mut DeviceGroupPresentModeFlagsKHR,
    ) -> VkResult,
>;

pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = Option<
    unsafe extern "system" fn(Device, SurfaceKHR, *mut DeviceGroupPresentModeFlagsKHR) -> VkResult,
>;

pub type PFN_vkGetDeviceImageMemoryRequirements = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageMemoryRequirements<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageMemoryRequirements<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceImageSparseMemoryRequirements = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageMemoryRequirements<'_>,
        *mut u32,
        *mut SparseImageMemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageMemoryRequirements<'_>,
        *mut u32,
        *mut SparseImageMemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceImageSubresourceLayout = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageSubresourceInfo<'_>,
        *mut SubresourceLayout2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceImageSubresourceLayoutKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceImageSubresourceInfo<'_>,
        *mut SubresourceLayout2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDeviceMemoryCommitment =
    Option<unsafe extern "system" fn(Device, DeviceMemory, *mut u64) -> ()>;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = Option<
    unsafe extern "system" fn(Device, *const DeviceMemoryOpaqueCaptureAddressInfo<'_>) -> u64,
>;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = Option<
    unsafe extern "system" fn(Device, *const DeviceMemoryOpaqueCaptureAddressInfo<'_>) -> u64,
>;

pub type PFN_vkGetDeviceMicromapCompatibilityEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const MicromapVersionInfoEXT<'_>,
        *mut AccelerationStructureCompatibilityKHR,
    ) -> (),
>;

pub type PFN_vkGetDeviceProcAddr =
    Option<unsafe extern "system" fn(Device, *const c_char) -> PFN_vkVoidFunction>;

pub type PFN_vkGetDeviceQueue =
    Option<unsafe extern "system" fn(Device, u32, u32, *mut Queue) -> ()>;

pub type PFN_vkGetDeviceQueue2 =
    Option<unsafe extern "system" fn(Device, *const DeviceQueueInfo2<'_>, *mut Queue) -> ()>;

pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI =
    Option<unsafe extern "system" fn(Device, RenderPass, *mut Extent2D<'_>) -> VkResult>;

pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceTensorMemoryRequirementsARM<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetDisplayModeProperties2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        DisplayKHR,
        *mut u32,
        *mut DisplayModeProperties2KHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDisplayModePropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        DisplayKHR,
        *mut u32,
        *mut DisplayModePropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDisplayPlaneCapabilities2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const DisplayPlaneInfo2KHR<'_>,
        *mut DisplayPlaneCapabilities2KHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDisplayPlaneCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        DisplayModeKHR,
        u32,
        *mut DisplayPlaneCapabilitiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut u32, *mut DisplayKHR) -> VkResult>;

pub type PFN_vkGetDrmDisplayEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, i32, u32, *mut DisplayKHR) -> VkResult>;

pub type PFN_vkGetDynamicRenderingTilePropertiesQCOM = Option<
    unsafe extern "system" fn(
        Device,
        *const RenderingInfo<'_>,
        *mut TilePropertiesQCOM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetEncodedVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const VideoEncodeSessionParametersGetInfoKHR<'_>,
        *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
        *mut usize,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetEventStatus = Option<unsafe extern "system" fn(Device, Event) -> VkResult>;

#[cfg(feature = "beta")]
pub type PFN_vkGetExecutionGraphPipelineNodeIndexAMDX = Option<
    unsafe extern "system" fn(
        Device,
        Pipeline,
        *const PipelineShaderStageNodeCreateInfoAMDX<'_>,
        *mut u32,
    ) -> VkResult,
>;

#[cfg(feature = "beta")]
pub type PFN_vkGetExecutionGraphPipelineScratchSizeAMDX = Option<
    unsafe extern "system" fn(
        Device,
        Pipeline,
        *mut ExecutionGraphPipelineScratchSizeAMDX<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetExternalComputeQueueDataNV = Option<
    unsafe extern "system" fn(
        ExternalComputeQueueNV,
        *mut ExternalComputeQueueDataParamsNV<'_>,
        *mut c_void,
    ) -> (),
>;

pub type PFN_vkGetFenceFdKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const FenceGetFdInfoKHR<'_>,
        *mut core::ffi::c_int,
    ) -> VkResult,
>;

pub type PFN_vkGetFenceStatus = Option<unsafe extern "system" fn(Device, Fence) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkGetFenceWin32HandleKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const FenceGetWin32HandleInfoKHR<'_>,
        *mut HANDLE,
    ) -> VkResult,
>;

pub type PFN_vkGetFramebufferTilePropertiesQCOM = Option<
    unsafe extern "system" fn(
        Device,
        Framebuffer,
        *mut u32,
        *mut TilePropertiesQCOM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        Device,
        *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetImageMemoryRequirements =
    Option<unsafe extern "system" fn(Device, Image, *mut MemoryRequirements<'_>) -> ()>;

pub type PFN_vkGetImageMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageMemoryRequirementsInfo2<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageMemoryRequirementsInfo2<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageOpaqueCaptureDataEXT = Option<
    unsafe extern "system" fn(Device, u32, *const Image, *mut HostAddressRangeEXT<'_>) -> VkResult,
>;

pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageCaptureDescriptorDataInfoEXT<'_>,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetImageSparseMemoryRequirements = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *mut u32,
        *mut SparseImageMemoryRequirements<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSparseMemoryRequirements2 = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageSparseMemoryRequirementsInfo2<'_>,
        *mut u32,
        *mut SparseImageMemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSparseMemoryRequirements2KHR = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageSparseMemoryRequirementsInfo2<'_>,
        *mut u32,
        *mut SparseImageMemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *const ImageSubresource<'_>,
        *mut SubresourceLayout<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout2 = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *const ImageSubresource2<'_>,
        *mut SubresourceLayout2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout2EXT = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *const ImageSubresource2<'_>,
        *mut SubresourceLayout2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageSubresourceLayout2KHR = Option<
    unsafe extern "system" fn(
        Device,
        Image,
        *const ImageSubresource2<'_>,
        *mut SubresourceLayout2<'_>,
    ) -> (),
>;

pub type PFN_vkGetImageViewAddressNVX = Option<
    unsafe extern "system" fn(
        Device,
        ImageView,
        *mut ImageViewAddressPropertiesNVX<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetImageViewHandle64NVX =
    Option<unsafe extern "system" fn(Device, *const ImageViewHandleInfoNVX<'_>) -> u64>;

pub type PFN_vkGetImageViewHandleNVX =
    Option<unsafe extern "system" fn(Device, *const ImageViewHandleInfoNVX<'_>) -> u32>;

pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetInstanceProcAddr =
    Option<unsafe extern "system" fn(Instance, *const c_char) -> PFN_vkVoidFunction>;

pub type PFN_vkGetInstanceProcAddrLUNARG =
    Option<unsafe extern "system" fn(Instance, *const c_char) -> PFN_vkVoidFunction>;

pub type PFN_vkGetLatencyTimingsNV =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, *mut GetLatencyMarkerInfoNV<'_>) -> ()>;

#[cfg(feature = "android")]
pub type PFN_vkGetMemoryAndroidHardwareBufferANDROID = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetAndroidHardwareBufferInfoANDROID<'_>,
        *mut *mut AHardwareBuffer,
    ) -> VkResult,
>;

pub type PFN_vkGetMemoryFdKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetFdInfoKHR<'_>,
        *mut core::ffi::c_int,
    ) -> VkResult,
>;

pub type PFN_vkGetMemoryFdPropertiesKHR = Option<
    unsafe extern "system" fn(
        Device,
        ExternalMemoryHandleTypeFlagBits,
        core::ffi::c_int,
        *mut MemoryFdPropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetMemoryHostPointerPropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        ExternalMemoryHandleTypeFlagBits,
        *const c_void,
        *mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "metal")]
pub type PFN_vkGetMemoryMetalHandleEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetMetalHandleInfoEXT<'_>,
        *mut *mut c_void,
    ) -> VkResult,
>;

#[cfg(feature = "metal")]
pub type PFN_vkGetMemoryMetalHandlePropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        ExternalMemoryHandleTypeFlagBits,
        *const c_void,
        *mut MemoryMetalHandlePropertiesEXT<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "ohos")]
pub type PFN_vkGetMemoryNativeBufferOHOS = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetNativeBufferInfoOHOS<'_>,
        *mut *mut OH_NativeBuffer,
    ) -> VkResult,
>;

pub type PFN_vkGetMemoryRemoteAddressNV = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetRemoteAddressInfoNV<'_>,
        *mut RemoteAddressNV,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandleKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetWin32HandleInfoKHR<'_>,
        *mut HANDLE,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandleNV = Option<
    unsafe extern "system" fn(
        Device,
        DeviceMemory,
        ExternalMemoryHandleTypeFlagsNV,
        *mut HANDLE,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = Option<
    unsafe extern "system" fn(
        Device,
        ExternalMemoryHandleTypeFlagBits,
        HANDLE,
        *mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkGetMemoryZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        *const MemoryGetZirconHandleInfoFUCHSIA<'_>,
        *mut zx_handle_t,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        ExternalMemoryHandleTypeFlagBits,
        zx_handle_t,
        *mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetMicromapBuildSizesEXT = Option<
    unsafe extern "system" fn(
        Device,
        AccelerationStructureBuildTypeKHR,
        *const MicromapBuildInfoEXT<'_>,
        *mut MicromapBuildSizesInfoEXT<'_>,
    ) -> (),
>;

#[cfg(feature = "ohos")]
pub type PFN_vkGetNativeBufferPropertiesOHOS = Option<
    unsafe extern "system" fn(
        Device,
        *const OH_NativeBuffer,
        *mut NativeBufferPropertiesOHOS<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV = Option<
    unsafe extern "system" fn(
        Device,
        *const PartitionedAccelerationStructureInstancesInputNV<'_>,
        *mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) -> (),
>;

pub type PFN_vkGetPastPresentationTimingEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const PastPresentationTimingInfoEXT<'_>,
        *mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPastPresentationTimingGOOGLE = Option<
    unsafe extern "system" fn(
        Device,
        SwapchainKHR,
        *mut u32,
        *mut PastPresentationTimingGOOGLE<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPerformanceParameterINTEL = Option<
    unsafe extern "system" fn(
        Device,
        PerformanceParameterTypeINTEL,
        *mut PerformanceValueINTEL<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut TimeDomainKHR) -> VkResult>;

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut TimeDomainKHR) -> VkResult>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut CooperativeMatrixFlexibleDimensionsPropertiesNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut CooperativeMatrixPropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut CooperativeMatrixPropertiesNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceCooperativeVectorPropertiesNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut CooperativeVectorPropertiesNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, DescriptorType) -> u64>;

#[cfg(feature = "directfb")]
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut IDirectFB) -> Bool32>;

pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut DisplayPlaneProperties2KHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut DisplayPlanePropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut DisplayProperties2KHR<'_>) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut DisplayPropertiesKHR<'_>) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalBufferInfo<'_>,
        *mut ExternalBufferProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalBufferInfo<'_>,
        *mut ExternalBufferProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalFenceInfo<'_>,
        *mut ExternalFenceProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalFenceInfo<'_>,
        *mut ExternalFenceProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        Format,
        ImageType,
        ImageTiling,
        ImageUsageFlags,
        ImageCreateFlags,
        ExternalMemoryHandleTypeFlagsNV,
        *mut ExternalImageFormatPropertiesNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        *mut ExternalSemaphoreProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalSemaphoreInfo<'_>,
        *mut ExternalSemaphoreProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceExternalTensorInfoARM<'_>,
        *mut ExternalTensorPropertiesARM<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceFeatures =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceFeatures<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFeatures2 =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceFeatures2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFeatures2KHR =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceFeatures2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFormatProperties =
    Option<unsafe extern "system" fn(PhysicalDevice, Format, *mut FormatProperties<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFormatProperties2 =
    Option<unsafe extern "system" fn(PhysicalDevice, Format, *mut FormatProperties2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR =
    Option<unsafe extern "system" fn(PhysicalDevice, Format, *mut FormatProperties2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut PhysicalDeviceFragmentShadingRateKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        Format,
        ImageType,
        ImageTiling,
        ImageUsageFlags,
        ImageCreateFlags,
        *mut ImageFormatProperties<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceImageFormatInfo2<'_>,
        *mut ImageFormatProperties2<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceImageFormatInfo2<'_>,
        *mut ImageFormatProperties2<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceMemoryProperties = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties<'_>) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties2<'_>) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceMemoryProperties2<'_>) -> (),
>;

pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        SampleCountFlagBits,
        *mut MultisamplePropertiesEXT<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const OpticalFlowImageFormatInfoNV<'_>,
        *mut u32,
        *mut OpticalFlowImageFormatPropertiesNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = Option<
    unsafe extern "system" fn(PhysicalDevice, SurfaceKHR, *mut u32, *mut Rect2D<'_>) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceProperties =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceProperties<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceProperties2 =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceProperties2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceProperties2KHR =
    Option<unsafe extern "system" fn(PhysicalDevice, *mut PhysicalDeviceProperties2<'_>) -> ()>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        u32,
        *const QueueFamilyDataGraphPropertiesARM<'_>,
        *mut BaseOutStructure<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        u32,
        *const QueueFamilyDataGraphPropertiesARM<'_>,
        *const DataGraphOpticalFlowImageFormatInfoARM<'_>,
        *mut u32,
        *mut DataGraphOpticalFlowImageFormatPropertiesARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
        *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        u32,
        *mut u32,
        *mut QueueFamilyDataGraphPropertiesARM<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const QueryPoolPerformanceCreateInfoKHR<'_>,
        *mut u32,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties<'_>) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties2<'_>) -> (),
>;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut u32, *mut QueueFamilyProperties2<'_>) -> (),
>;

#[cfg(feature = "screen")]
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut _screen_window) -> Bool32>;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        Format,
        ImageType,
        SampleCountFlagBits,
        ImageUsageFlags,
        ImageTiling,
        *mut u32,
        *mut SparseImageFormatProperties<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        *mut u32,
        *mut SparseImageFormatProperties2<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceSparseImageFormatInfo2<'_>,
        *mut u32,
        *mut SparseImageFormatProperties2<'_>,
    ) -> (),
>;

pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut FramebufferMixedSamplesCombinationNV<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        SurfaceKHR,
        *mut SurfaceCapabilities2EXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        *mut SurfaceCapabilities2KHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        SurfaceKHR,
        *mut SurfaceCapabilitiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        *mut u32,
        *mut SurfaceFormat2KHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        SurfaceKHR,
        *mut u32,
        *mut SurfaceFormatKHR<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceSurfaceInfo2KHR<'_>,
        *mut u32,
        *mut PresentModeKHR,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        SurfaceKHR,
        *mut u32,
        *mut PresentModeKHR,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, SurfaceKHR, *mut Bool32) -> VkResult>;

pub type PFN_vkGetPhysicalDeviceToolProperties = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *mut u32,
        *mut PhysicalDeviceToolProperties<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "ubm")]
pub type PFN_vkGetPhysicalDeviceUbmPresentationSupportSEC =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut ubm_device) -> Bool32>;

pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const VideoProfileInfoKHR<'_>,
        *mut VideoCapabilitiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = Option<
    unsafe extern "system" fn(
        PhysicalDevice,
        *const PhysicalDeviceVideoFormatInfoKHR<'_>,
        *mut u32,
        *mut VideoFormatPropertiesKHR<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "wayland")]
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut wl_display) -> Bool32>;

#[cfg(feature = "win32")]
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, u32) -> Bool32>;

#[cfg(feature = "xcb")]
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = Option<
    unsafe extern "system" fn(PhysicalDevice, u32, *mut xcb_connection_t, xcb_visualid_t) -> Bool32,
>;

#[cfg(feature = "xlib")]
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut Display, VisualID) -> Bool32>;

pub type PFN_vkGetPipelineBinaryDataKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineBinaryDataInfoKHR<'_>,
        *mut PipelineBinaryKeyKHR<'_>,
        *mut usize,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetPipelineCacheData =
    Option<unsafe extern "system" fn(Device, PipelineCache, *mut usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineExecutableInfoKHR<'_>,
        *mut u32,
        *mut PipelineExecutableInternalRepresentationKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPipelineExecutablePropertiesKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineInfoKHR<'_>,
        *mut u32,
        *mut PipelineExecutablePropertiesKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPipelineExecutableStatisticsKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineExecutableInfoKHR<'_>,
        *mut u32,
        *mut PipelineExecutableStatisticKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPipelineIndirectDeviceAddressNV = Option<
    unsafe extern "system" fn(Device, *const PipelineIndirectDeviceAddressInfoNV<'_>) -> u64,
>;

pub type PFN_vkGetPipelineIndirectMemoryRequirementsNV = Option<
    unsafe extern "system" fn(
        Device,
        *const ComputePipelineCreateInfo<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetPipelineKeyKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineCreateInfoKHR<'_>,
        *mut PipelineBinaryKeyKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPipelinePropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const PipelineInfoKHR<'_>,
        *mut BaseOutStructure<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetPrivateData =
    Option<unsafe extern "system" fn(Device, ObjectType, u64, PrivateDataSlot, *mut u64) -> ()>;

pub type PFN_vkGetPrivateDataEXT =
    Option<unsafe extern "system" fn(Device, ObjectType, u64, PrivateDataSlot, *mut u64) -> ()>;

pub type PFN_vkGetQueryPoolResults = Option<
    unsafe extern "system" fn(
        Device,
        QueryPool,
        u32,
        u32,
        usize,
        *mut c_void,
        u64,
        QueryResultFlags,
    ) -> VkResult,
>;

pub type PFN_vkGetQueueCheckpointData2NV =
    Option<unsafe extern "system" fn(Queue, *mut u32, *mut CheckpointData2NV<'_>) -> ()>;

pub type PFN_vkGetQueueCheckpointDataNV =
    Option<unsafe extern "system" fn(Queue, *mut u32, *mut CheckpointDataNV<'_>) -> ()>;

#[cfg(feature = "xlib-xrandr")]
pub type PFN_vkGetRandROutputDisplayEXT = Option<
    unsafe extern "system" fn(PhysicalDevice, *mut Display, RROutput, *mut DisplayKHR) -> VkResult,
>;

pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
    Option<unsafe extern "system" fn(Device, Pipeline, u32, u32, usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetRayTracingShaderGroupHandlesKHR =
    Option<unsafe extern "system" fn(Device, Pipeline, u32, u32, usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetRayTracingShaderGroupHandlesNV =
    Option<unsafe extern "system" fn(Device, Pipeline, u32, u32, usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR =
    Option<unsafe extern "system" fn(Device, Pipeline, u32, ShaderGroupShaderKHR) -> u64>;

pub type PFN_vkGetRefreshCycleDurationGOOGLE = Option<
    unsafe extern "system" fn(
        Device,
        SwapchainKHR,
        *mut RefreshCycleDurationGOOGLE<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetRenderAreaGranularity =
    Option<unsafe extern "system" fn(Device, RenderPass, *mut Extent2D<'_>) -> ()>;

pub type PFN_vkGetRenderingAreaGranularity = Option<
    unsafe extern "system" fn(Device, *const RenderingAreaInfo<'_>, *mut Extent2D<'_>) -> (),
>;

pub type PFN_vkGetRenderingAreaGranularityKHR = Option<
    unsafe extern "system" fn(Device, *const RenderingAreaInfo<'_>, *mut Extent2D<'_>) -> (),
>;

pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const SamplerCaptureDescriptorDataInfoEXT<'_>,
        *mut c_void,
    ) -> VkResult,
>;

#[cfg(feature = "screen")]
pub type PFN_vkGetScreenBufferPropertiesQNX = Option<
    unsafe extern "system" fn(
        Device,
        *const _screen_buffer,
        *mut ScreenBufferPropertiesQNX<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetSemaphoreCounterValue =
    Option<unsafe extern "system" fn(Device, Semaphore, *mut u64) -> VkResult>;

pub type PFN_vkGetSemaphoreCounterValueKHR =
    Option<unsafe extern "system" fn(Device, Semaphore, *mut u64) -> VkResult>;

pub type PFN_vkGetSemaphoreFdKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const SemaphoreGetFdInfoKHR<'_>,
        *mut core::ffi::c_int,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetSemaphoreWin32HandleKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const SemaphoreGetWin32HandleInfoKHR<'_>,
        *mut HANDLE,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
        *mut zx_handle_t,
    ) -> VkResult,
>;

pub type PFN_vkGetShaderBinaryDataEXT =
    Option<unsafe extern "system" fn(Device, ShaderEXT, *mut usize, *mut c_void) -> VkResult>;

pub type PFN_vkGetShaderInfoAMD = Option<
    unsafe extern "system" fn(
        Device,
        Pipeline,
        ShaderStageFlagBits,
        ShaderInfoTypeAMD,
        *mut usize,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetShaderInstrumentationValuesARM = Option<
    unsafe extern "system" fn(
        Device,
        ShaderInstrumentationARM,
        *mut u32,
        *mut c_void,
        ShaderInstrumentationValuesFlagsARM,
    ) -> VkResult,
>;

pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const ShaderModuleCreateInfo<'_>,
        *mut ShaderModuleIdentifierEXT<'_>,
    ) -> (),
>;

pub type PFN_vkGetShaderModuleIdentifierEXT = Option<
    unsafe extern "system" fn(Device, ShaderModule, *mut ShaderModuleIdentifierEXT<'_>) -> (),
>;

pub type PFN_vkGetSwapchainCounterEXT = Option<
    unsafe extern "system" fn(
        Device,
        SwapchainKHR,
        SurfaceCounterFlagBitsEXT,
        *mut u64,
    ) -> VkResult,
>;

pub type PFN_vkGetSwapchainImagesKHR =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, *mut u32, *mut Image) -> VkResult>;

pub type PFN_vkGetSwapchainStatusKHR =
    Option<unsafe extern "system" fn(Device, SwapchainKHR) -> VkResult>;

pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        SwapchainKHR,
        *mut SwapchainTimeDomainPropertiesEXT<'_>,
        *mut u64,
    ) -> VkResult,
>;

pub type PFN_vkGetSwapchainTimingPropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        SwapchainKHR,
        *mut SwapchainTimingPropertiesEXT<'_>,
        *mut u64,
    ) -> VkResult,
>;

pub type PFN_vkGetTensorMemoryRequirementsARM = Option<
    unsafe extern "system" fn(
        Device,
        *const TensorMemoryRequirementsInfoARM<'_>,
        *mut MemoryRequirements2<'_>,
    ) -> (),
>;

pub type PFN_vkGetTensorOpaqueCaptureDataARM = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const TensorARM,
        *mut HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = Option<
    unsafe extern "system" fn(
        Device,
        *const TensorCaptureDescriptorDataInfoARM<'_>,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = Option<
    unsafe extern "system" fn(
        Device,
        *const TensorViewCaptureDescriptorDataInfoARM<'_>,
        *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkGetValidationCacheDataEXT = Option<
    unsafe extern "system" fn(Device, ValidationCacheEXT, *mut usize, *mut c_void) -> VkResult,
>;

pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = Option<
    unsafe extern "system" fn(
        Device,
        VideoSessionKHR,
        *mut u32,
        *mut VideoSessionMemoryRequirementsKHR<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "win32")]
pub type PFN_vkGetWinrtDisplayNV =
    Option<unsafe extern "system" fn(PhysicalDevice, u32, *mut DisplayKHR) -> VkResult>;

pub type PFN_vkImportFenceFdKHR =
    Option<unsafe extern "system" fn(Device, *const ImportFenceFdInfoKHR<'_>) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkImportFenceWin32HandleKHR =
    Option<unsafe extern "system" fn(Device, *const ImportFenceWin32HandleInfoKHR<'_>) -> VkResult>;

pub type PFN_vkImportSemaphoreFdKHR =
    Option<unsafe extern "system" fn(Device, *const ImportSemaphoreFdInfoKHR<'_>) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkImportSemaphoreWin32HandleKHR = Option<
    unsafe extern "system" fn(Device, *const ImportSemaphoreWin32HandleInfoKHR<'_>) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

pub type PFN_vkInitializePerformanceApiINTEL = Option<
    unsafe extern "system" fn(Device, *const InitializePerformanceApiInfoINTEL<'_>) -> VkResult,
>;

pub type PFN_vkInternalAllocationNotification = Option<
    unsafe extern "system" fn(
        *mut c_void,
        usize,
        InternalAllocationType,
        SystemAllocationScope,
    ) -> (),
>;

pub type PFN_vkInternalFreeNotification = Option<
    unsafe extern "system" fn(
        *mut c_void,
        usize,
        InternalAllocationType,
        SystemAllocationScope,
    ) -> (),
>;

pub type PFN_vkInvalidateMappedMemoryRanges =
    Option<unsafe extern "system" fn(Device, u32, *const MappedMemoryRange<'_>) -> VkResult>;

pub type PFN_vkLatencySleepNV = Option<
    unsafe extern "system" fn(Device, SwapchainKHR, *const LatencySleepInfoNV<'_>) -> VkResult,
>;

pub type PFN_vkMapMemory = Option<
    unsafe extern "system" fn(
        Device,
        DeviceMemory,
        u64,
        u64,
        MemoryMapFlags,
        *mut *mut c_void,
    ) -> VkResult,
>;

pub type PFN_vkMapMemory2 = Option<
    unsafe extern "system" fn(Device, *const MemoryMapInfo<'_>, *mut *mut c_void) -> VkResult,
>;

pub type PFN_vkMapMemory2KHR = Option<
    unsafe extern "system" fn(Device, *const MemoryMapInfo<'_>, *mut *mut c_void) -> VkResult,
>;

pub type PFN_vkMergePipelineCaches =
    Option<unsafe extern "system" fn(Device, PipelineCache, u32, *const PipelineCache) -> VkResult>;

pub type PFN_vkMergeValidationCachesEXT = Option<
    unsafe extern "system" fn(
        Device,
        ValidationCacheEXT,
        u32,
        *const ValidationCacheEXT,
    ) -> VkResult,
>;

pub type PFN_vkQueueBeginDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(Queue, *const DebugUtilsLabelEXT<'_>) -> ()>;

pub type PFN_vkQueueBindSparse =
    Option<unsafe extern "system" fn(Queue, u32, *const BindSparseInfo<'_>, Fence) -> VkResult>;

pub type PFN_vkQueueEndDebugUtilsLabelEXT = Option<unsafe extern "system" fn(Queue) -> ()>;

pub type PFN_vkQueueInsertDebugUtilsLabelEXT =
    Option<unsafe extern "system" fn(Queue, *const DebugUtilsLabelEXT<'_>) -> ()>;

pub type PFN_vkQueueNotifyOutOfBandNV =
    Option<unsafe extern "system" fn(Queue, *const OutOfBandQueueTypeInfoNV<'_>) -> ()>;

pub type PFN_vkQueuePresentKHR =
    Option<unsafe extern "system" fn(Queue, *const PresentInfoKHR<'_>) -> VkResult>;

pub type PFN_vkQueueSetPerfHintQCOM =
    Option<unsafe extern "system" fn(Queue, *const PerfHintInfoQCOM<'_>) -> VkResult>;

pub type PFN_vkQueueSetPerformanceConfigurationINTEL =
    Option<unsafe extern "system" fn(Queue, PerformanceConfigurationINTEL) -> VkResult>;

pub type PFN_vkQueueSubmit =
    Option<unsafe extern "system" fn(Queue, u32, *const SubmitInfo<'_>, Fence) -> VkResult>;

pub type PFN_vkQueueSubmit2 =
    Option<unsafe extern "system" fn(Queue, u32, *const SubmitInfo2<'_>, Fence) -> VkResult>;

pub type PFN_vkQueueSubmit2KHR =
    Option<unsafe extern "system" fn(Queue, u32, *const SubmitInfo2<'_>, Fence) -> VkResult>;

pub type PFN_vkQueueWaitIdle = Option<unsafe extern "system" fn(Queue) -> VkResult>;

pub type PFN_vkReallocationFunction = Option<
    unsafe extern "system" fn(
        *mut c_void,
        *mut c_void,
        usize,
        usize,
        SystemAllocationScope,
    ) -> *mut c_void,
>;

pub type PFN_vkRegisterCustomBorderColorEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const SamplerCustomBorderColorCreateInfoEXT<'_>,
        Bool32,
        *mut u32,
    ) -> VkResult,
>;

pub type PFN_vkRegisterDeviceEventEXT = Option<
    unsafe extern "system" fn(
        Device,
        *const DeviceEventInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut Fence,
    ) -> VkResult,
>;

pub type PFN_vkRegisterDisplayEventEXT = Option<
    unsafe extern "system" fn(
        Device,
        DisplayKHR,
        *const DisplayEventInfoEXT<'_>,
        *const AllocationCallbacks<'_>,
        *mut Fence,
    ) -> VkResult,
>;

pub type PFN_vkReleaseCapturedPipelineDataKHR = Option<
    unsafe extern "system" fn(
        Device,
        *const ReleaseCapturedPipelineDataInfoKHR<'_>,
        *const AllocationCallbacks<'_>,
    ) -> VkResult,
>;

pub type PFN_vkReleaseDisplayEXT =
    Option<unsafe extern "system" fn(PhysicalDevice, DisplayKHR) -> VkResult>;

#[cfg(feature = "win32")]
pub type PFN_vkReleaseFullScreenExclusiveModeEXT =
    Option<unsafe extern "system" fn(Device, SwapchainKHR) -> VkResult>;

pub type PFN_vkReleasePerformanceConfigurationINTEL =
    Option<unsafe extern "system" fn(Device, PerformanceConfigurationINTEL) -> VkResult>;

pub type PFN_vkReleaseProfilingLockKHR = Option<unsafe extern "system" fn(Device) -> ()>;

pub type PFN_vkReleaseSwapchainImagesEXT =
    Option<unsafe extern "system" fn(Device, *const ReleaseSwapchainImagesInfoKHR<'_>) -> VkResult>;

pub type PFN_vkReleaseSwapchainImagesKHR =
    Option<unsafe extern "system" fn(Device, *const ReleaseSwapchainImagesInfoKHR<'_>) -> VkResult>;

pub type PFN_vkResetCommandBuffer =
    Option<unsafe extern "system" fn(CommandBuffer, CommandBufferResetFlags) -> VkResult>;

pub type PFN_vkResetCommandPool =
    Option<unsafe extern "system" fn(Device, CommandPool, CommandPoolResetFlags) -> VkResult>;

pub type PFN_vkResetDescriptorPool =
    Option<unsafe extern "system" fn(Device, DescriptorPool, DescriptorPoolResetFlags) -> VkResult>;

pub type PFN_vkResetEvent = Option<unsafe extern "system" fn(Device, Event) -> VkResult>;

pub type PFN_vkResetFences =
    Option<unsafe extern "system" fn(Device, u32, *const Fence) -> VkResult>;

pub type PFN_vkResetQueryPool =
    Option<unsafe extern "system" fn(Device, QueryPool, u32, u32) -> ()>;

pub type PFN_vkResetQueryPoolEXT =
    Option<unsafe extern "system" fn(Device, QueryPool, u32, u32) -> ()>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkSetBufferCollectionBufferConstraintsFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        BufferCollectionFUCHSIA,
        *const BufferConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

#[cfg(feature = "fuchsia")]
pub type PFN_vkSetBufferCollectionImageConstraintsFUCHSIA = Option<
    unsafe extern "system" fn(
        Device,
        BufferCollectionFUCHSIA,
        *const ImageConstraintsInfoFUCHSIA<'_>,
    ) -> VkResult,
>;

pub type PFN_vkSetDebugUtilsObjectNameEXT =
    Option<unsafe extern "system" fn(Device, *const DebugUtilsObjectNameInfoEXT<'_>) -> VkResult>;

pub type PFN_vkSetDebugUtilsObjectTagEXT =
    Option<unsafe extern "system" fn(Device, *const DebugUtilsObjectTagInfoEXT<'_>) -> VkResult>;

pub type PFN_vkSetDeviceMemoryPriorityEXT =
    Option<unsafe extern "system" fn(Device, DeviceMemory, f32) -> ()>;

pub type PFN_vkSetEvent = Option<unsafe extern "system" fn(Device, Event) -> VkResult>;

pub type PFN_vkSetHdrMetadataEXT = Option<
    unsafe extern "system" fn(Device, u32, *const SwapchainKHR, *const HdrMetadataEXT<'_>) -> (),
>;

pub type PFN_vkSetLatencyMarkerNV = Option<
    unsafe extern "system" fn(Device, SwapchainKHR, *const SetLatencyMarkerInfoNV<'_>) -> (),
>;

pub type PFN_vkSetLatencySleepModeNV = Option<
    unsafe extern "system" fn(Device, SwapchainKHR, *const LatencySleepModeInfoNV<'_>) -> VkResult,
>;

pub type PFN_vkSetLocalDimmingAMD =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, Bool32) -> ()>;

pub type PFN_vkSetPrivateData =
    Option<unsafe extern "system" fn(Device, ObjectType, u64, PrivateDataSlot, u64) -> VkResult>;

pub type PFN_vkSetPrivateDataEXT =
    Option<unsafe extern "system" fn(Device, ObjectType, u64, PrivateDataSlot, u64) -> VkResult>;

pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, u32) -> VkResult>;

pub type PFN_vkSignalSemaphore =
    Option<unsafe extern "system" fn(Device, *const SemaphoreSignalInfo<'_>) -> VkResult>;

pub type PFN_vkSignalSemaphoreKHR =
    Option<unsafe extern "system" fn(Device, *const SemaphoreSignalInfo<'_>) -> VkResult>;

pub type PFN_vkSubmitDebugUtilsMessageEXT = Option<
    unsafe extern "system" fn(
        Instance,
        DebugUtilsMessageSeverityFlagBitsEXT,
        DebugUtilsMessageTypeFlagsEXT,
        *const DebugUtilsMessengerCallbackDataEXT<'_>,
    ) -> (),
>;

pub type PFN_vkTransitionImageLayout = Option<
    unsafe extern "system" fn(Device, u32, *const HostImageLayoutTransitionInfo<'_>) -> VkResult,
>;

pub type PFN_vkTransitionImageLayoutEXT = Option<
    unsafe extern "system" fn(Device, u32, *const HostImageLayoutTransitionInfo<'_>) -> VkResult,
>;

pub type PFN_vkTrimCommandPool =
    Option<unsafe extern "system" fn(Device, CommandPool, CommandPoolTrimFlags) -> ()>;

pub type PFN_vkTrimCommandPoolKHR =
    Option<unsafe extern "system" fn(Device, CommandPool, CommandPoolTrimFlags) -> ()>;

pub type PFN_vkUninitializePerformanceApiINTEL = Option<unsafe extern "system" fn(Device) -> ()>;

pub type PFN_vkUnmapMemory = Option<unsafe extern "system" fn(Device, DeviceMemory) -> ()>;

pub type PFN_vkUnmapMemory2 =
    Option<unsafe extern "system" fn(Device, *const MemoryUnmapInfo<'_>) -> VkResult>;

pub type PFN_vkUnmapMemory2KHR =
    Option<unsafe extern "system" fn(Device, *const MemoryUnmapInfo<'_>) -> VkResult>;

pub type PFN_vkUnregisterCustomBorderColorEXT =
    Option<unsafe extern "system" fn(Device, u32) -> ()>;

pub type PFN_vkUpdateDescriptorSetWithTemplate = Option<
    unsafe extern "system" fn(Device, DescriptorSet, DescriptorUpdateTemplate, *const c_void) -> (),
>;

pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = Option<
    unsafe extern "system" fn(Device, DescriptorSet, DescriptorUpdateTemplate, *const c_void) -> (),
>;

pub type PFN_vkUpdateDescriptorSets = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const WriteDescriptorSet<'_>,
        u32,
        *const CopyDescriptorSet<'_>,
    ) -> (),
>;

pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = Option<
    unsafe extern "system" fn(
        Device,
        IndirectExecutionSetEXT,
        u32,
        *const WriteIndirectExecutionSetPipelineEXT<'_>,
    ) -> (),
>;

pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = Option<
    unsafe extern "system" fn(
        Device,
        IndirectExecutionSetEXT,
        u32,
        *const WriteIndirectExecutionSetShaderEXT<'_>,
    ) -> (),
>;

pub type PFN_vkUpdateVideoSessionParametersKHR = Option<
    unsafe extern "system" fn(
        Device,
        VideoSessionParametersKHR,
        *const VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> VkResult,
>;

pub type PFN_vkWaitForFences =
    Option<unsafe extern "system" fn(Device, u32, *const Fence, Bool32, u64) -> VkResult>;

pub type PFN_vkWaitForPresent2KHR = Option<
    unsafe extern "system" fn(Device, SwapchainKHR, *const PresentWait2InfoKHR<'_>) -> VkResult,
>;

pub type PFN_vkWaitForPresentKHR =
    Option<unsafe extern "system" fn(Device, SwapchainKHR, u64, u64) -> VkResult>;

pub type PFN_vkWaitSemaphores =
    Option<unsafe extern "system" fn(Device, *const SemaphoreWaitInfo<'_>, u64) -> VkResult>;

pub type PFN_vkWaitSemaphoresKHR =
    Option<unsafe extern "system" fn(Device, *const SemaphoreWaitInfo<'_>, u64) -> VkResult>;

pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const AccelerationStructureKHR,
        QueryType,
        usize,
        *mut c_void,
        usize,
    ) -> VkResult,
>;

pub type PFN_vkWriteMicromapsPropertiesEXT = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const MicromapEXT,
        QueryType,
        usize,
        *mut c_void,
        usize,
    ) -> VkResult,
>;

pub type PFN_vkWriteResourceDescriptorsEXT = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const ResourceDescriptorInfoEXT<'_>,
        *const HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;

pub type PFN_vkWriteSamplerDescriptorsEXT = Option<
    unsafe extern "system" fn(
        Device,
        u32,
        *const SamplerCreateInfo<'_>,
        *const HostAddressRangeEXT<'_>,
    ) -> VkResult,
>;
