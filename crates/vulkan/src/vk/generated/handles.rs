#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::vk::*;

use core::ffi::c_void;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccelerationStructureKHR(pub u64);

impl AccelerationStructureKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AccelerationStructureNV(pub u64);

impl AccelerationStructureNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Buffer(pub u64);

impl Buffer {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BufferCollectionFUCHSIA(pub u64);

#[cfg(feature = "fuchsia")]
impl BufferCollectionFUCHSIA {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BufferView(pub u64);

impl BufferView {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CommandBuffer(pub *mut c_void);

impl CommandBuffer {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CommandPool(pub u64);

impl CommandPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CuFunctionNVX(pub u64);

impl CuFunctionNVX {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CuModuleNVX(pub u64);

impl CuModuleNVX {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CudaFunctionNV(pub u64);

#[cfg(feature = "beta")]
impl CudaFunctionNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[cfg(feature = "beta")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CudaModuleNV(pub u64);

#[cfg(feature = "beta")]
impl CudaModuleNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DataGraphPipelineSessionARM(pub u64);

impl DataGraphPipelineSessionARM {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DebugReportCallbackEXT(pub u64);

impl DebugReportCallbackEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DebugUtilsMessengerEXT(pub u64);

impl DebugUtilsMessengerEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DeferredOperationKHR(pub u64);

impl DeferredOperationKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorPool(pub u64);

impl DescriptorPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorSet(pub u64);

impl DescriptorSet {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorSetLayout(pub u64);

impl DescriptorSetLayout {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DescriptorUpdateTemplate(pub u64);

impl DescriptorUpdateTemplate {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

pub type DescriptorUpdateTemplateKHR = DescriptorUpdateTemplate;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Device(pub *mut c_void);

impl Device {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DeviceMemory(pub u64);

impl DeviceMemory {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DisplayKHR(pub u64);

impl DisplayKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct DisplayModeKHR(pub u64);

impl DisplayModeKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Event(pub u64);

impl Event {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternalComputeQueueNV(pub *mut c_void);

impl ExternalComputeQueueNV {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Fence(pub u64);

impl Fence {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Framebuffer(pub u64);

impl Framebuffer {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Image(pub u64);

impl Image {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImageView(pub u64);

impl ImageView {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IndirectCommandsLayoutEXT(pub u64);

impl IndirectCommandsLayoutEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IndirectCommandsLayoutNV(pub u64);

impl IndirectCommandsLayoutNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IndirectExecutionSetEXT(pub u64);

impl IndirectExecutionSetEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Instance(pub *mut c_void);

impl Instance {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MicromapEXT(pub u64);

impl MicromapEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OpticalFlowSessionNV(pub u64);

impl OpticalFlowSessionNV {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PerformanceConfigurationINTEL(pub u64);

impl PerformanceConfigurationINTEL {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PhysicalDevice(pub *mut c_void);

impl PhysicalDevice {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Pipeline(pub u64);

impl Pipeline {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PipelineBinaryKHR(pub u64);

impl PipelineBinaryKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PipelineCache(pub u64);

impl PipelineCache {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PipelineLayout(pub u64);

impl PipelineLayout {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrivateDataSlot(pub u64);

impl PrivateDataSlot {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

pub type PrivateDataSlotEXT = PrivateDataSlot;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct QueryPool(pub u64);

impl QueryPool {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Queue(pub *mut c_void);

impl Queue {
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }
    pub const fn is_null(self) -> bool {
        self.0.is_null()
    }
    pub const fn from_raw(value: *mut c_void) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> *mut c_void {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RenderPass(pub u64);

impl RenderPass {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Sampler(pub u64);

impl Sampler {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SamplerYcbcrConversion(pub u64);

impl SamplerYcbcrConversion {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

pub type SamplerYcbcrConversionKHR = SamplerYcbcrConversion;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Semaphore(pub u64);

impl Semaphore {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaderEXT(pub u64);

impl ShaderEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaderInstrumentationARM(pub u64);

impl ShaderInstrumentationARM {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShaderModule(pub u64);

impl ShaderModule {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SurfaceKHR(pub u64);

impl SurfaceKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SwapchainKHR(pub u64);

impl SwapchainKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TensorARM(pub u64);

impl TensorARM {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TensorViewARM(pub u64);

impl TensorViewARM {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ValidationCacheEXT(pub u64);

impl ValidationCacheEXT {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VideoSessionKHR(pub u64);

impl VideoSessionKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VideoSessionParametersKHR(pub u64);

impl VideoSessionParametersKHR {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}
