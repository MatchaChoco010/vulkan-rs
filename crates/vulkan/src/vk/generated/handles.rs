#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

use core::ffi::c_void;

/// Opaque handle to an acceleration structure object.
///
/// C non-dispatchable handle
/// [`VkAccelerationStructureKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureKHR.html),
/// provided by [`VK_KHR_device_address_commands`](KHR_DEVICE_ADDRESS_COMMANDS_NAME) or
/// [`VK_KHR_acceleration_structure`](KHR_ACCELERATION_STRUCTURE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkAccelerationStructureKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureKHR(pub u64);

impl AccelerationStructureKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for AccelerationStructureKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for AccelerationStructureKHR {
    const TYPE: ObjectType = ObjectType::ACCELERATION_STRUCTURE_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for AccelerationStructureKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccelerationStructureKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an acceleration structure object.
///
/// C non-dispatchable handle
/// [`VkAccelerationStructureNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureNV.html),
/// provided by [`VK_NV_ray_tracing`](NV_RAY_TRACING_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkAccelerationStructureNV")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccelerationStructureNV(pub u64);

impl AccelerationStructureNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for AccelerationStructureNV {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for AccelerationStructureNV {
    const TYPE: ObjectType = ObjectType::ACCELERATION_STRUCTURE_NV;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for AccelerationStructureNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AccelerationStructureNV({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a buffer object.
///
/// C non-dispatchable handle
/// [`VkBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBuffer.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkBuffer")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Buffer(pub u64);

impl Buffer {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Buffer {
    const TYPE: ObjectType = ObjectType::BUFFER;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a buffer collection object.
///
/// C non-dispatchable handle
/// [`VkBufferCollectionFUCHSIA`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCollectionFUCHSIA.html),
/// provided by [`VK_FUCHSIA_buffer_collection`](FUCHSIA_BUFFER_COLLECTION_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkBufferCollectionFUCHSIA")]
#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCollectionFUCHSIA(pub u64);

#[cfg(feature = "fuchsia")]
impl BufferCollectionFUCHSIA {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "fuchsia")]
impl Default for BufferCollectionFUCHSIA {
    fn default() -> Self {
        Self::null()
    }
}

#[cfg(feature = "fuchsia")]
impl Handle for BufferCollectionFUCHSIA {
    const TYPE: ObjectType = ObjectType::BUFFER_COLLECTION_FUCHSIA;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(feature = "fuchsia")]
impl core::fmt::Debug for BufferCollectionFUCHSIA {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BufferCollectionFUCHSIA({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a buffer view object.
///
/// C non-dispatchable handle
/// [`VkBufferView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferView.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkBufferView")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferView(pub u64);

impl BufferView {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for BufferView {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for BufferView {
    const TYPE: ObjectType = ObjectType::BUFFER_VIEW;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for BufferView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "BufferView({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a command buffer object.
///
/// C dispatchable handle
/// [`VkCommandBuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBuffer.html).
///
/// Objects of this type are created from a [`VkCommandPool`](CommandPool). The value is the
/// handle itself: copying it does not copy the object, and dropping it does not destroy the
/// object.
#[doc(alias = "VkCommandBuffer")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBuffer(pub *mut c_void);

impl CommandBuffer {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for CommandBuffer {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for CommandBuffer {
    const TYPE: ObjectType = ObjectType::COMMAND_BUFFER;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CommandBuffer({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for CommandBuffer {}
unsafe impl Sync for CommandBuffer {}

/// Opaque handle to a command pool object.
///
/// C non-dispatchable handle
/// [`VkCommandPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPool.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkCommandPool")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPool(pub u64);

impl CommandPool {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for CommandPool {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for CommandPool {
    const TYPE: ObjectType = ObjectType::COMMAND_POOL;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for CommandPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CommandPool({:#x})", self.as_raw_u64())
    }
}

/// Stub description of VkCuFunctionNVX.
///
/// C non-dispatchable handle
/// [`VkCuFunctionNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuFunctionNVX.html),
/// provided by [`VK_NVX_binary_import`](NVX_BINARY_IMPORT_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkCuFunctionNVX")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CuFunctionNVX(pub u64);

impl CuFunctionNVX {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for CuFunctionNVX {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for CuFunctionNVX {
    const TYPE: ObjectType = ObjectType::CU_FUNCTION_NVX;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for CuFunctionNVX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CuFunctionNVX({:#x})", self.as_raw_u64())
    }
}

/// Stub description of VkCuModuleNVX.
///
/// C non-dispatchable handle
/// [`VkCuModuleNVX`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCuModuleNVX.html),
/// provided by [`VK_NVX_binary_import`](NVX_BINARY_IMPORT_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkCuModuleNVX")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CuModuleNVX(pub u64);

impl CuModuleNVX {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for CuModuleNVX {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for CuModuleNVX {
    const TYPE: ObjectType = ObjectType::CU_MODULE_NVX;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for CuModuleNVX {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CuModuleNVX({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a CUDA function object.
///
/// C non-dispatchable handle
/// [`VkCudaFunctionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaFunctionNV.html),
/// provided by [`VK_NV_cuda_kernel_launch`](NV_CUDA_KERNEL_LAUNCH_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkCudaFunctionNV")]
#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CudaFunctionNV(pub u64);

#[cfg(feature = "beta")]
impl CudaFunctionNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "beta")]
impl Default for CudaFunctionNV {
    fn default() -> Self {
        Self::null()
    }
}

#[cfg(feature = "beta")]
impl Handle for CudaFunctionNV {
    const TYPE: ObjectType = ObjectType::CUDA_FUNCTION_NV;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(feature = "beta")]
impl core::fmt::Debug for CudaFunctionNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CudaFunctionNV({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a CUDA module object.
///
/// C non-dispatchable handle
/// [`VkCudaModuleNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkCudaModuleNV.html),
/// provided by [`VK_NV_cuda_kernel_launch`](NV_CUDA_KERNEL_LAUNCH_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkCudaModuleNV")]
#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CudaModuleNV(pub u64);

#[cfg(feature = "beta")]
impl CudaModuleNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "beta")]
impl Default for CudaModuleNV {
    fn default() -> Self {
        Self::null()
    }
}

#[cfg(feature = "beta")]
impl Handle for CudaModuleNV {
    const TYPE: ObjectType = ObjectType::CUDA_MODULE_NV;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(feature = "beta")]
impl core::fmt::Debug for CudaModuleNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CudaModuleNV({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a data graph pipeline session object.
///
/// C non-dispatchable handle
/// [`VkDataGraphPipelineSessionARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionARM.html),
/// provided by [`VK_ARM_data_graph`](ARM_DATA_GRAPH_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDataGraphPipelineSessionARM")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionARM(pub u64);

impl DataGraphPipelineSessionARM {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DataGraphPipelineSessionARM {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DataGraphPipelineSessionARM {
    const TYPE: ObjectType = ObjectType::DATA_GRAPH_PIPELINE_SESSION_ARM;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DataGraphPipelineSessionARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DataGraphPipelineSessionARM({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a debug report callback object.
///
/// C non-dispatchable handle
/// [`VkDebugReportCallbackEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportCallbackEXT.html),
/// provided by [`VK_EXT_debug_report`](EXT_DEBUG_REPORT_NAME).
///
/// Objects of this type are created from a [`VkInstance`](Instance). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDebugReportCallbackEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugReportCallbackEXT(pub u64);

impl DebugReportCallbackEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DebugReportCallbackEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DebugReportCallbackEXT {
    const TYPE: ObjectType = ObjectType::DEBUG_REPORT_CALLBACK_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DebugReportCallbackEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DebugReportCallbackEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a debug messenger object.
///
/// C non-dispatchable handle
/// [`VkDebugUtilsMessengerEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessengerEXT.html),
/// provided by [`VK_EXT_debug_utils`](EXT_DEBUG_UTILS_NAME).
///
/// Objects of this type are created from a [`VkInstance`](Instance). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDebugUtilsMessengerEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DebugUtilsMessengerEXT(pub u64);

impl DebugUtilsMessengerEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DebugUtilsMessengerEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DebugUtilsMessengerEXT {
    const TYPE: ObjectType = ObjectType::DEBUG_UTILS_MESSENGER_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DebugUtilsMessengerEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DebugUtilsMessengerEXT({:#x})", self.as_raw_u64())
    }
}

/// A deferred operation.
///
/// C non-dispatchable handle
/// [`VkDeferredOperationKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeferredOperationKHR.html),
/// provided by [`VK_KHR_deferred_host_operations`](KHR_DEFERRED_HOST_OPERATIONS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDeferredOperationKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeferredOperationKHR(pub u64);

impl DeferredOperationKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DeferredOperationKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DeferredOperationKHR {
    const TYPE: ObjectType = ObjectType::DEFERRED_OPERATION_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DeferredOperationKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DeferredOperationKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a descriptor pool object.
///
/// C non-dispatchable handle
/// [`VkDescriptorPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPool.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDescriptorPool")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPool(pub u64);

impl DescriptorPool {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DescriptorPool {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DescriptorPool {
    const TYPE: ObjectType = ObjectType::DESCRIPTOR_POOL;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DescriptorPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DescriptorPool({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a descriptor set object.
///
/// C non-dispatchable handle
/// [`VkDescriptorSet`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSet.html).
///
/// Objects of this type are created from a [`VkDescriptorPool`](DescriptorPool). The value is
/// the handle itself: copying it does not copy the object, and dropping it does not destroy the
/// object.
#[doc(alias = "VkDescriptorSet")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSet(pub u64);

impl DescriptorSet {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DescriptorSet {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DescriptorSet {
    const TYPE: ObjectType = ObjectType::DESCRIPTOR_SET;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DescriptorSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DescriptorSet({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a descriptor set layout object.
///
/// C non-dispatchable handle
/// [`VkDescriptorSetLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayout.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDescriptorSetLayout")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayout(pub u64);

impl DescriptorSetLayout {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DescriptorSetLayout {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DescriptorSetLayout {
    const TYPE: ObjectType = ObjectType::DESCRIPTOR_SET_LAYOUT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DescriptorSetLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DescriptorSetLayout({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a descriptor update template.
///
/// C non-dispatchable handle
/// [`VkDescriptorUpdateTemplate`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplate.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDescriptorUpdateTemplate")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplate(pub u64);

impl DescriptorUpdateTemplate {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DescriptorUpdateTemplate {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DescriptorUpdateTemplate {
    const TYPE: ObjectType = ObjectType::DESCRIPTOR_UPDATE_TEMPLATE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DescriptorUpdateTemplate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DescriptorUpdateTemplate({:#x})", self.as_raw_u64())
    }
}

/// Alias of [`DescriptorUpdateTemplate`], the C name `VkDescriptorUpdateTemplateKHR`.
#[doc(alias = "VkDescriptorUpdateTemplateKHR")]
pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

/// Opaque handle to a device object.
///
/// C dispatchable handle
/// [`VkDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDevice.html).
///
/// Objects of this type are created from a [`VkPhysicalDevice`](PhysicalDevice). The value is
/// the handle itself: copying it does not copy the object, and dropping it does not destroy the
/// object.
#[doc(alias = "VkDevice")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Device(pub *mut c_void);

impl Device {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for Device {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Device {
    const TYPE: ObjectType = ObjectType::DEVICE;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Device({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for Device {}
unsafe impl Sync for Device {}

/// Opaque handle to a device memory object.
///
/// C non-dispatchable handle
/// [`VkDeviceMemory`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemory.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkDeviceMemory")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceMemory(pub u64);

impl DeviceMemory {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DeviceMemory {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DeviceMemory {
    const TYPE: ObjectType = ObjectType::DEVICE_MEMORY;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DeviceMemory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DeviceMemory({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a display object.
///
/// C non-dispatchable handle
/// [`VkDisplayKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayKHR.html),
/// provided by [`VK_KHR_display`](KHR_DISPLAY_NAME).
///
/// Objects of this type are created from a [`VkPhysicalDevice`](PhysicalDevice). The value is
/// the handle itself: copying it does not copy the object, and dropping it does not destroy the
/// object.
#[doc(alias = "VkDisplayKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayKHR(pub u64);

impl DisplayKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DisplayKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DisplayKHR {
    const TYPE: ObjectType = ObjectType::DISPLAY_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DisplayKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DisplayKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a display mode object.
///
/// C non-dispatchable handle
/// [`VkDisplayModeKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayModeKHR.html),
/// provided by [`VK_KHR_display`](KHR_DISPLAY_NAME).
///
/// Objects of this type are created from a [`VkDisplayKHR`](DisplayKHR). The value is the
/// handle itself: copying it does not copy the object, and dropping it does not destroy the
/// object.
#[doc(alias = "VkDisplayModeKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayModeKHR(pub u64);

impl DisplayModeKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for DisplayModeKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for DisplayModeKHR {
    const TYPE: ObjectType = ObjectType::DISPLAY_MODE_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for DisplayModeKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "DisplayModeKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an event object.
///
/// C non-dispatchable handle
/// [`VkEvent`](https://docs.vulkan.org/refpages/latest/refpages/source/VkEvent.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkEvent")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Event(pub u64);

impl Event {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Event {
    const TYPE: ObjectType = ObjectType::EVENT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Event {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Event({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an external compute queue.
///
/// C dispatchable handle
/// [`VkExternalComputeQueueNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalComputeQueueNV.html),
/// provided by [`VK_NV_external_compute_queue`](NV_EXTERNAL_COMPUTE_QUEUE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkExternalComputeQueueNV")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalComputeQueueNV(pub *mut c_void);

impl ExternalComputeQueueNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for ExternalComputeQueueNV {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ExternalComputeQueueNV {
    const TYPE: ObjectType = ObjectType::EXTERNAL_COMPUTE_QUEUE_NV;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for ExternalComputeQueueNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ExternalComputeQueueNV({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for ExternalComputeQueueNV {}
unsafe impl Sync for ExternalComputeQueueNV {}

/// Opaque handle to a fence object.
///
/// C non-dispatchable handle
/// [`VkFence`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFence.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkFence")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Fence(pub u64);

impl Fence {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Fence {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Fence {
    const TYPE: ObjectType = ObjectType::FENCE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Fence {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Fence({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a framebuffer object.
///
/// C non-dispatchable handle
/// [`VkFramebuffer`](https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebuffer.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkFramebuffer")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Framebuffer(pub u64);

impl Framebuffer {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Framebuffer {
    const TYPE: ObjectType = ObjectType::FRAMEBUFFER;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Framebuffer({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a GPU Performance API object.
///
/// C non-dispatchable handle
/// [`VkGpaSessionAMD`](https://docs.vulkan.org/refpages/latest/refpages/source/VkGpaSessionAMD.html),
/// provided by [`VK_AMD_gpa_interface`](AMD_GPA_INTERFACE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkGpaSessionAMD")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GpaSessionAMD(pub u64);

impl GpaSessionAMD {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for GpaSessionAMD {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for GpaSessionAMD {
    const TYPE: ObjectType = ObjectType::GPA_SESSION_AMD;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for GpaSessionAMD {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GpaSessionAMD({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an image object.
///
/// C non-dispatchable handle
/// [`VkImage`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImage.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkImage")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Image(pub u64);

impl Image {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Image {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Image {
    const TYPE: ObjectType = ObjectType::IMAGE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Image {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Image({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an image view object.
///
/// C non-dispatchable handle
/// [`VkImageView`](https://docs.vulkan.org/refpages/latest/refpages/source/VkImageView.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkImageView")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageView(pub u64);

impl ImageView {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for ImageView {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ImageView {
    const TYPE: ObjectType = ObjectType::IMAGE_VIEW;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for ImageView {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ImageView({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an indirect commands layout object.
///
/// C non-dispatchable handle
/// [`VkIndirectCommandsLayoutEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutEXT.html),
/// provided by [`VK_EXT_device_generated_commands`](EXT_DEVICE_GENERATED_COMMANDS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkIndirectCommandsLayoutEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutEXT(pub u64);

impl IndirectCommandsLayoutEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for IndirectCommandsLayoutEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for IndirectCommandsLayoutEXT {
    const TYPE: ObjectType = ObjectType::INDIRECT_COMMANDS_LAYOUT_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for IndirectCommandsLayoutEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IndirectCommandsLayoutEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an indirect commands layout object.
///
/// C non-dispatchable handle
/// [`VkIndirectCommandsLayoutNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutNV.html),
/// provided by [`VK_NV_device_generated_commands`](NV_DEVICE_GENERATED_COMMANDS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkIndirectCommandsLayoutNV")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutNV(pub u64);

impl IndirectCommandsLayoutNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for IndirectCommandsLayoutNV {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for IndirectCommandsLayoutNV {
    const TYPE: ObjectType = ObjectType::INDIRECT_COMMANDS_LAYOUT_NV;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for IndirectCommandsLayoutNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IndirectCommandsLayoutNV({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an indirect execution set object.
///
/// C non-dispatchable handle
/// [`VkIndirectExecutionSetEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetEXT.html),
/// provided by [`VK_EXT_device_generated_commands`](EXT_DEVICE_GENERATED_COMMANDS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkIndirectExecutionSetEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectExecutionSetEXT(pub u64);

impl IndirectExecutionSetEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for IndirectExecutionSetEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for IndirectExecutionSetEXT {
    const TYPE: ObjectType = ObjectType::INDIRECT_EXECUTION_SET_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for IndirectExecutionSetEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "IndirectExecutionSetEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an instance object.
///
/// C dispatchable handle
/// [`VkInstance`](https://docs.vulkan.org/refpages/latest/refpages/source/VkInstance.html).
///
/// The value is the handle itself: copying it does not copy the object, and dropping it does
/// not destroy the object.
#[doc(alias = "VkInstance")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Instance(pub *mut c_void);

impl Instance {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for Instance {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Instance {
    const TYPE: ObjectType = ObjectType::INSTANCE;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for Instance {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Instance({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

/// Opaque handle to a micromap object.
///
/// C non-dispatchable handle
/// [`VkMicromapEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapEXT.html),
/// provided by [`VK_EXT_opacity_micromap`](EXT_OPACITY_MICROMAP_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkMicromapEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MicromapEXT(pub u64);

impl MicromapEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for MicromapEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for MicromapEXT {
    const TYPE: ObjectType = ObjectType::MICROMAP_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for MicromapEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MicromapEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an optical flow session object.
///
/// C non-dispatchable handle
/// [`VkOpticalFlowSessionNV`](https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionNV.html),
/// provided by [`VK_NV_optical_flow`](NV_OPTICAL_FLOW_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkOpticalFlowSessionNV")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpticalFlowSessionNV(pub u64);

impl OpticalFlowSessionNV {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for OpticalFlowSessionNV {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for OpticalFlowSessionNV {
    const TYPE: ObjectType = ObjectType::OPTICAL_FLOW_SESSION_NV;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for OpticalFlowSessionNV {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "OpticalFlowSessionNV({:#x})", self.as_raw_u64())
    }
}

/// Device configuration for performance queries.
///
/// C non-dispatchable handle
/// [`VkPerformanceConfigurationINTEL`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationINTEL.html),
/// provided by [`VK_INTEL_performance_query`](INTEL_PERFORMANCE_QUERY_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PerformanceConfigurationINTEL(pub u64);

impl PerformanceConfigurationINTEL {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for PerformanceConfigurationINTEL {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PerformanceConfigurationINTEL {
    const TYPE: ObjectType = ObjectType::PERFORMANCE_CONFIGURATION_INTEL;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for PerformanceConfigurationINTEL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PerformanceConfigurationINTEL({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a physical device object.
///
/// C dispatchable handle
/// [`VkPhysicalDevice`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDevice.html).
///
/// Objects of this type are created from a [`VkInstance`](Instance). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPhysicalDevice")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDevice(pub *mut c_void);

impl PhysicalDevice {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for PhysicalDevice {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PhysicalDevice {
    const TYPE: ObjectType = ObjectType::PHYSICAL_DEVICE;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalDevice({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for PhysicalDevice {}
unsafe impl Sync for PhysicalDevice {}

/// Opaque handle to a pipeline object.
///
/// C non-dispatchable handle
/// [`VkPipeline`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipeline.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPipeline")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pipeline(pub u64);

impl Pipeline {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Pipeline {
    const TYPE: ObjectType = ObjectType::PIPELINE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Pipeline {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Pipeline({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a pipeline binary object.
///
/// C non-dispatchable handle
/// [`VkPipelineBinaryKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBinaryKHR.html),
/// provided by [`VK_KHR_pipeline_binary`](KHR_PIPELINE_BINARY_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPipelineBinaryKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineBinaryKHR(pub u64);

impl PipelineBinaryKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for PipelineBinaryKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PipelineBinaryKHR {
    const TYPE: ObjectType = ObjectType::PIPELINE_BINARY_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for PipelineBinaryKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PipelineBinaryKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a pipeline cache object.
///
/// C non-dispatchable handle
/// [`VkPipelineCache`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCache.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPipelineCache")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCache(pub u64);

impl PipelineCache {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for PipelineCache {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PipelineCache {
    const TYPE: ObjectType = ObjectType::PIPELINE_CACHE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for PipelineCache {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PipelineCache({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a pipeline layout object.
///
/// C non-dispatchable handle
/// [`VkPipelineLayout`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayout.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPipelineLayout")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayout(pub u64);

impl PipelineLayout {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for PipelineLayout {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PipelineLayout {
    const TYPE: ObjectType = ObjectType::PIPELINE_LAYOUT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for PipelineLayout {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PipelineLayout({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a private data slot object.
///
/// C non-dispatchable handle
/// [`VkPrivateDataSlot`](https://docs.vulkan.org/refpages/latest/refpages/source/VkPrivateDataSlot.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkPrivateDataSlot")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrivateDataSlot(pub u64);

impl PrivateDataSlot {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for PrivateDataSlot {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for PrivateDataSlot {
    const TYPE: ObjectType = ObjectType::PRIVATE_DATA_SLOT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for PrivateDataSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PrivateDataSlot({:#x})", self.as_raw_u64())
    }
}

/// Alias of [`PrivateDataSlot`], the C name `VkPrivateDataSlotEXT`.
#[doc(alias = "VkPrivateDataSlotEXT")]
pub type PrivateDataSlotEXT = PrivateDataSlot;

/// Opaque handle to a query pool object.
///
/// C non-dispatchable handle
/// [`VkQueryPool`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPool.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkQueryPool")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPool(pub u64);

impl QueryPool {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for QueryPool {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for QueryPool {
    const TYPE: ObjectType = ObjectType::QUERY_POOL;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for QueryPool {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "QueryPool({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a queue object.
///
/// C dispatchable handle
/// [`VkQueue`](https://docs.vulkan.org/refpages/latest/refpages/source/VkQueue.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkQueue")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Queue(pub *mut c_void);

impl Queue {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Queue {
    const TYPE: ObjectType = ObjectType::QUEUE;
    fn as_raw_u64(self) -> u64 {
        self.0 as usize as u64
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value as usize as *mut c_void)
    }
}

impl core::fmt::Debug for Queue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Queue({:#x})", self.as_raw_u64())
    }
}

unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

/// Opaque handle to a render pass object.
///
/// C non-dispatchable handle
/// [`VkRenderPass`](https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPass.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkRenderPass")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPass(pub u64);

impl RenderPass {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for RenderPass {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for RenderPass {
    const TYPE: ObjectType = ObjectType::RENDER_PASS;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for RenderPass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RenderPass({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a sampler object.
///
/// C non-dispatchable handle
/// [`VkSampler`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSampler.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkSampler")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sampler(pub u64);

impl Sampler {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Sampler {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Sampler {
    const TYPE: ObjectType = ObjectType::SAMPLER;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Sampler {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Sampler({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a device-specific sampler Y′C<sub>B</sub>C<sub>R</sub> conversion
/// description.
///
/// C non-dispatchable handle
/// [`VkSamplerYcbcrConversion`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrConversion.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkSamplerYcbcrConversion")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrConversion(pub u64);

impl SamplerYcbcrConversion {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for SamplerYcbcrConversion {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for SamplerYcbcrConversion {
    const TYPE: ObjectType = ObjectType::SAMPLER_YCBCR_CONVERSION;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for SamplerYcbcrConversion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SamplerYcbcrConversion({:#x})", self.as_raw_u64())
    }
}

/// Alias of [`SamplerYcbcrConversion`], the C name `VkSamplerYcbcrConversionKHR`.
#[doc(alias = "VkSamplerYcbcrConversionKHR")]
pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

/// Opaque handle to a semaphore object.
///
/// C non-dispatchable handle
/// [`VkSemaphore`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphore.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkSemaphore")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Semaphore(pub u64);

impl Semaphore {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for Semaphore {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for Semaphore {
    const TYPE: ObjectType = ObjectType::SEMAPHORE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for Semaphore {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Semaphore({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a shader object.
///
/// C non-dispatchable handle
/// [`VkShaderEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderEXT.html),
/// provided by [`VK_EXT_shader_object`](EXT_SHADER_OBJECT_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkShaderEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderEXT(pub u64);

impl ShaderEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for ShaderEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ShaderEXT {
    const TYPE: ObjectType = ObjectType::SHADER_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for ShaderEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ShaderEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a shader instrumentation object.
///
/// C non-dispatchable handle
/// [`VkShaderInstrumentationARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInstrumentationARM.html),
/// provided by [`VK_ARM_shader_instrumentation`](ARM_SHADER_INSTRUMENTATION_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkShaderInstrumentationARM")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderInstrumentationARM(pub u64);

impl ShaderInstrumentationARM {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for ShaderInstrumentationARM {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ShaderInstrumentationARM {
    const TYPE: ObjectType = ObjectType::SHADER_INSTRUMENTATION_ARM;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for ShaderInstrumentationARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ShaderInstrumentationARM({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a shader module object.
///
/// C non-dispatchable handle
/// [`VkShaderModule`](https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderModule.html).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkShaderModule")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderModule(pub u64);

impl ShaderModule {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for ShaderModule {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ShaderModule {
    const TYPE: ObjectType = ObjectType::SHADER_MODULE;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for ShaderModule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ShaderModule({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a surface object.
///
/// C non-dispatchable handle
/// [`VkSurfaceKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceKHR.html),
/// provided by [`VK_KHR_surface`](KHR_SURFACE_NAME).
///
/// Objects of this type are created from a [`VkInstance`](Instance). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkSurfaceKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SurfaceKHR(pub u64);

impl SurfaceKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for SurfaceKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for SurfaceKHR {
    const TYPE: ObjectType = ObjectType::SURFACE_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for SurfaceKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SurfaceKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a swapchain object.
///
/// C non-dispatchable handle
/// [`VkSwapchainKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainKHR.html),
/// provided by [`VK_KHR_swapchain`](KHR_SWAPCHAIN_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkSwapchainKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SwapchainKHR(pub u64);

impl SwapchainKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for SwapchainKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for SwapchainKHR {
    const TYPE: ObjectType = ObjectType::SWAPCHAIN_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for SwapchainKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SwapchainKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a tensor object.
///
/// C non-dispatchable handle
/// [`VkTensorARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorARM.html),
/// provided by [`VK_EXT_descriptor_heap`](EXT_DESCRIPTOR_HEAP_NAME) or
/// [`VK_ARM_tensors`](ARM_TENSORS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkTensorARM")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorARM(pub u64);

impl TensorARM {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for TensorARM {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for TensorARM {
    const TYPE: ObjectType = ObjectType::TENSOR_ARM;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for TensorARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TensorARM({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to an tensor view object.
///
/// C non-dispatchable handle
/// [`VkTensorViewARM`](https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewARM.html),
/// provided by [`VK_ARM_tensors`](ARM_TENSORS_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkTensorViewARM")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorViewARM(pub u64);

impl TensorViewARM {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for TensorViewARM {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for TensorViewARM {
    const TYPE: ObjectType = ObjectType::TENSOR_VIEW_ARM;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for TensorViewARM {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TensorViewARM({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a validation cache object.
///
/// C non-dispatchable handle
/// [`VkValidationCacheEXT`](https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheEXT.html),
/// provided by [`VK_EXT_validation_cache`](EXT_VALIDATION_CACHE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkValidationCacheEXT")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValidationCacheEXT(pub u64);

impl ValidationCacheEXT {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for ValidationCacheEXT {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for ValidationCacheEXT {
    const TYPE: ObjectType = ObjectType::VALIDATION_CACHE_EXT;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for ValidationCacheEXT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ValidationCacheEXT({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a video session object.
///
/// C non-dispatchable handle
/// [`VkVideoSessionKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionKHR.html),
/// provided by [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkVideoSessionKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionKHR(pub u64);

impl VideoSessionKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for VideoSessionKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for VideoSessionKHR {
    const TYPE: ObjectType = ObjectType::VIDEO_SESSION_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for VideoSessionKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VideoSessionKHR({:#x})", self.as_raw_u64())
    }
}

/// Opaque handle to a video session parameters object.
///
/// C non-dispatchable handle
/// [`VkVideoSessionParametersKHR`](https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersKHR.html),
/// provided by [`VK_KHR_video_queue`](KHR_VIDEO_QUEUE_NAME).
///
/// Objects of this type are created from a [`VkDevice`](Device). The value is the handle
/// itself: copying it does not copy the object, and dropping it does not destroy the object.
#[doc(alias = "VkVideoSessionParametersKHR")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionParametersKHR(pub u64);

impl VideoSessionParametersKHR {
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {
        Self(0)
    }
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl Default for VideoSessionParametersKHR {
    fn default() -> Self {
        Self::null()
    }
}

impl Handle for VideoSessionParametersKHR {
    const TYPE: ObjectType = ObjectType::VIDEO_SESSION_PARAMETERS_KHR;
    fn as_raw_u64(self) -> u64 {
        self.0
    }
    fn from_raw_u64(value: u64) -> Self {
        Self(value)
    }
}

impl core::fmt::Debug for VideoSessionParametersKHR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VideoSessionParametersKHR({:#x})", self.as_raw_u64())
    }
}
