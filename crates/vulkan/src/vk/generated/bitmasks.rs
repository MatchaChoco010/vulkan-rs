#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unsafe_op_in_unsafe_fn)]

use crate::vk::*;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureCreateFlagBitsKHR(pub u32);

impl AccelerationStructureCreateFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1u32);
    pub const MOTION_NV: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(8u32);
}

impl From<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from(value: AccelerationStructureCreateFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AccelerationStructureCreateFlagBitsKHR {
    type Output = AccelerationStructureCreateFlagsKHR;
    fn bitor(self, rhs: Self) -> AccelerationStructureCreateFlagsKHR {
        AccelerationStructureCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccelerationStructureCreateFlagsKHR>
    for AccelerationStructureCreateFlagBitsKHR
{
    type Output = AccelerationStructureCreateFlagsKHR;
    fn bitor(
        self,
        rhs: AccelerationStructureCreateFlagsKHR,
    ) -> AccelerationStructureCreateFlagsKHR {
        AccelerationStructureCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccelerationStructureCreateFlagBitsKHR>
    for AccelerationStructureCreateFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: AccelerationStructureCreateFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AccelerationStructureCreateFlagBitsKHR>
    for AccelerationStructureCreateFlagsKHR
{
    fn bitor_assign(&mut self, rhs: AccelerationStructureCreateFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlagBits(pub u32);

impl AccessFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const INDIRECT_COMMAND_READ: Self = Self(1u32);
    pub const INDEX_READ: Self = Self(2u32);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(4u32);
    pub const UNIFORM_READ: Self = Self(8u32);
    pub const INPUT_ATTACHMENT_READ: Self = Self(16u32);
    pub const SHADER_READ: Self = Self(32u32);
    pub const SHADER_WRITE: Self = Self(64u32);
    pub const COLOR_ATTACHMENT_READ: Self = Self(128u32);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(256u32);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512u32);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024u32);
    pub const TRANSFER_READ: Self = Self(2048u32);
    pub const TRANSFER_WRITE: Self = Self(4096u32);
    pub const HOST_READ: Self = Self(8192u32);
    pub const HOST_WRITE: Self = Self(16384u32);
    pub const MEMORY_READ: Self = Self(32768u32);
    pub const MEMORY_WRITE: Self = Self(65536u32);
    pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(131072u32);
    pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(262144u32);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288u32);
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576u32);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152u32);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608u32);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216u32);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728u32);
}

impl From<AccessFlagBits> for AccessFlags {
    fn from(value: AccessFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AccessFlagBits {
    type Output = AccessFlags;
    fn bitor(self, rhs: Self) -> AccessFlags {
        AccessFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlags> for AccessFlagBits {
    type Output = AccessFlags;
    fn bitor(self, rhs: AccessFlags) -> AccessFlags {
        AccessFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlagBits> for AccessFlags {
    type Output = Self;
    fn bitor(self, rhs: AccessFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AccessFlagBits> for AccessFlags {
    fn bitor_assign(&mut self, rhs: AccessFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlagBits2(pub u64);

impl AccessFlagBits2 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const ACCESS_2_NONE: Self = Self(0u64);
    pub const ACCESS_2_INDIRECT_COMMAND_READ: Self = Self(1u64);
    pub const ACCESS_2_INDEX_READ: Self = Self(2u64);
    pub const ACCESS_2_VERTEX_ATTRIBUTE_READ: Self = Self(4u64);
    pub const ACCESS_2_UNIFORM_READ: Self = Self(8u64);
    pub const ACCESS_2_INPUT_ATTACHMENT_READ: Self = Self(16u64);
    pub const ACCESS_2_SHADER_READ: Self = Self(32u64);
    pub const ACCESS_2_SHADER_WRITE: Self = Self(64u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_READ: Self = Self(128u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_WRITE: Self = Self(256u64);
    pub const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512u64);
    pub const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024u64);
    pub const ACCESS_2_TRANSFER_READ: Self = Self(2048u64);
    pub const ACCESS_2_TRANSFER_WRITE: Self = Self(4096u64);
    pub const ACCESS_2_HOST_READ: Self = Self(8192u64);
    pub const ACCESS_2_HOST_WRITE: Self = Self(16384u64);
    pub const ACCESS_2_MEMORY_READ: Self = Self(32768u64);
    pub const ACCESS_2_MEMORY_WRITE: Self = Self(65536u64);
    pub const ACCESS_2_COMMAND_PREPROCESS_READ_EXT: Self = Self(131072u64);
    pub const ACCESS_2_COMMAND_PREPROCESS_WRITE_EXT: Self = Self(262144u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288u64);
    pub const ACCESS_2_CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576u64);
    pub const ACCESS_2_ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152u64);
    pub const ACCESS_2_ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304u64);
    pub const ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608u64);
    pub const ACCESS_2_FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728u64);
    pub const ACCESS_2_SHADER_SAMPLED_READ: Self = Self(4294967296u64);
    pub const ACCESS_2_SHADER_STORAGE_READ: Self = Self(8589934592u64);
    pub const ACCESS_2_SHADER_STORAGE_WRITE: Self = Self(17179869184u64);
    pub const ACCESS_2_VIDEO_DECODE_READ_KHR: Self = Self(34359738368u64);
    pub const ACCESS_2_VIDEO_DECODE_WRITE_KHR: Self = Self(68719476736u64);
    pub const ACCESS_2_VIDEO_ENCODE_READ_KHR: Self = Self(137438953472u64);
    pub const ACCESS_2_VIDEO_ENCODE_WRITE_KHR: Self = Self(274877906944u64);
    pub const ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI: Self = Self(549755813888u64);
    pub const ACCESS_2_SHADER_BINDING_TABLE_READ_KHR: Self = Self(1099511627776u64);
    pub const ACCESS_2_DESCRIPTOR_BUFFER_READ_EXT: Self = Self(2199023255552u64);
    pub const ACCESS_2_OPTICAL_FLOW_READ_NV: Self = Self(4398046511104u64);
    pub const ACCESS_2_OPTICAL_FLOW_WRITE_NV: Self = Self(8796093022208u64);
    pub const ACCESS_2_MICROMAP_READ_EXT: Self = Self(17592186044416u64);
    pub const ACCESS_2_MICROMAP_WRITE_EXT: Self = Self(35184372088832u64);
    pub const ACCESS_2_DATA_GRAPH_READ_ARM: Self = Self(140737488355328u64);
    pub const ACCESS_2_DATA_GRAPH_WRITE_ARM: Self = Self(281474976710656u64);
    pub const ACCESS_2_SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(2251799813685248u64);
    pub const ACCESS_2_SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(4503599627370496u64);
    pub const ACCESS_2_MEMORY_DECOMPRESSION_READ_EXT: Self = Self(36028797018963968u64);
    pub const ACCESS_2_MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(72057594037927936u64);
    pub const ACCESS_2_SAMPLER_HEAP_READ_EXT: Self = Self(144115188075855872u64);
    pub const ACCESS_2_RESOURCE_HEAP_READ_EXT: Self = Self(288230376151711744u64);
}

impl From<AccessFlagBits2> for AccessFlags2 {
    fn from(value: AccessFlagBits2) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AccessFlagBits2 {
    type Output = AccessFlags2;
    fn bitor(self, rhs: Self) -> AccessFlags2 {
        AccessFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlags2> for AccessFlagBits2 {
    type Output = AccessFlags2;
    fn bitor(self, rhs: AccessFlags2) -> AccessFlags2 {
        AccessFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlagBits2> for AccessFlags2 {
    type Output = Self;
    fn bitor(self, rhs: AccessFlagBits2) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AccessFlagBits2> for AccessFlags2 {
    fn bitor_assign(&mut self, rhs: AccessFlagBits2) {
        self.0 |= rhs.0;
    }
}

pub type AccessFlagBits2KHR = AccessFlagBits2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlagBits3KHR(pub u64);

impl AccessFlagBits3KHR {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const ACCESS_3_NONE_KHR: Self = Self(0u64);
}

impl From<AccessFlagBits3KHR> for AccessFlags3KHR {
    fn from(value: AccessFlagBits3KHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AccessFlagBits3KHR {
    type Output = AccessFlags3KHR;
    fn bitor(self, rhs: Self) -> AccessFlags3KHR {
        AccessFlags3KHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlags3KHR> for AccessFlagBits3KHR {
    type Output = AccessFlags3KHR;
    fn bitor(self, rhs: AccessFlags3KHR) -> AccessFlags3KHR {
        AccessFlags3KHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AccessFlagBits3KHR> for AccessFlags3KHR {
    type Output = Self;
    fn bitor(self, rhs: AccessFlagBits3KHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AccessFlagBits3KHR> for AccessFlags3KHR {
    fn bitor_assign(&mut self, rhs: AccessFlagBits3KHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AcquireProfilingLockFlagBitsKHR(pub u32);

impl AcquireProfilingLockFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn from(value: AcquireProfilingLockFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AcquireProfilingLockFlagBitsKHR {
    type Output = AcquireProfilingLockFlagsKHR;
    fn bitor(self, rhs: Self) -> AcquireProfilingLockFlagsKHR {
        AcquireProfilingLockFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AcquireProfilingLockFlagsKHR> for AcquireProfilingLockFlagBitsKHR {
    type Output = AcquireProfilingLockFlagsKHR;
    fn bitor(self, rhs: AcquireProfilingLockFlagsKHR) -> AcquireProfilingLockFlagsKHR {
        AcquireProfilingLockFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: AcquireProfilingLockFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AcquireProfilingLockFlagBitsKHR> for AcquireProfilingLockFlagsKHR {
    fn bitor_assign(&mut self, rhs: AcquireProfilingLockFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AddressCommandFlagBitsKHR(pub u32);

impl AddressCommandFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_KHR: Self = Self(1u32);
    pub const FULLY_BOUND_KHR: Self = Self(2u32);
    pub const STORAGE_BUFFER_USAGE_KHR: Self = Self(4u32);
    pub const UNKNOWN_STORAGE_BUFFER_USAGE_KHR: Self = Self(8u32);
    pub const TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(16u32);
    pub const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(32u32);
}

impl From<AddressCommandFlagBitsKHR> for AddressCommandFlagsKHR {
    fn from(value: AddressCommandFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AddressCommandFlagBitsKHR {
    type Output = AddressCommandFlagsKHR;
    fn bitor(self, rhs: Self) -> AddressCommandFlagsKHR {
        AddressCommandFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AddressCommandFlagsKHR> for AddressCommandFlagBitsKHR {
    type Output = AddressCommandFlagsKHR;
    fn bitor(self, rhs: AddressCommandFlagsKHR) -> AddressCommandFlagsKHR {
        AddressCommandFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AddressCommandFlagBitsKHR> for AddressCommandFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: AddressCommandFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AddressCommandFlagBitsKHR> for AddressCommandFlagsKHR {
    fn bitor_assign(&mut self, rhs: AddressCommandFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AddressCopyFlagBitsKHR(pub u32);

impl AddressCopyFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL_KHR: Self = Self(1u32);
    pub const SPARSE_KHR: Self = Self(2u32);
    pub const PROTECTED_KHR: Self = Self(4u32);
}

impl From<AddressCopyFlagBitsKHR> for AddressCopyFlagsKHR {
    fn from(value: AddressCopyFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AddressCopyFlagBitsKHR {
    type Output = AddressCopyFlagsKHR;
    fn bitor(self, rhs: Self) -> AddressCopyFlagsKHR {
        AddressCopyFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AddressCopyFlagsKHR> for AddressCopyFlagBitsKHR {
    type Output = AddressCopyFlagsKHR;
    fn bitor(self, rhs: AddressCopyFlagsKHR) -> AddressCopyFlagsKHR {
        AddressCopyFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AddressCopyFlagBitsKHR> for AddressCopyFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: AddressCopyFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AddressCopyFlagBitsKHR> for AddressCopyFlagsKHR {
    fn bitor_assign(&mut self, rhs: AddressCopyFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentDescriptionFlagBits(pub u32);

impl AttachmentDescriptionFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MAY_ALIAS: Self = Self(1u32);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(4u32);
}

impl From<AttachmentDescriptionFlagBits> for AttachmentDescriptionFlags {
    fn from(value: AttachmentDescriptionFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for AttachmentDescriptionFlagBits {
    type Output = AttachmentDescriptionFlags;
    fn bitor(self, rhs: Self) -> AttachmentDescriptionFlags {
        AttachmentDescriptionFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AttachmentDescriptionFlags> for AttachmentDescriptionFlagBits {
    type Output = AttachmentDescriptionFlags;
    fn bitor(self, rhs: AttachmentDescriptionFlags) -> AttachmentDescriptionFlags {
        AttachmentDescriptionFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<AttachmentDescriptionFlagBits> for AttachmentDescriptionFlags {
    type Output = Self;
    fn bitor(self, rhs: AttachmentDescriptionFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<AttachmentDescriptionFlagBits> for AttachmentDescriptionFlags {
    fn bitor_assign(&mut self, rhs: AttachmentDescriptionFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferCreateFlagBits(pub u32);

impl BufferCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPARSE_BINDING: Self = Self(1u32);
    pub const SPARSE_RESIDENCY: Self = Self(2u32);
    pub const SPARSE_ALIASED: Self = Self(4u32);
    pub const PROTECTED: Self = Self(8u32);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(16u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(32u32);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(64u32);
}

impl From<BufferCreateFlagBits> for BufferCreateFlags {
    fn from(value: BufferCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for BufferCreateFlagBits {
    type Output = BufferCreateFlags;
    fn bitor(self, rhs: Self) -> BufferCreateFlags {
        BufferCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferCreateFlags> for BufferCreateFlagBits {
    type Output = BufferCreateFlags;
    fn bitor(self, rhs: BufferCreateFlags) -> BufferCreateFlags {
        BufferCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferCreateFlagBits> for BufferCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: BufferCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BufferCreateFlagBits> for BufferCreateFlags {
    fn bitor_assign(&mut self, rhs: BufferCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferUsageFlagBits(pub u32);

impl BufferUsageFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFER_SRC: Self = Self(1u32);
    pub const TRANSFER_DST: Self = Self(2u32);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4u32);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(8u32);
    pub const UNIFORM_BUFFER: Self = Self(16u32);
    pub const STORAGE_BUFFER: Self = Self(32u32);
    pub const INDEX_BUFFER: Self = Self(64u32);
    pub const VERTEX_BUFFER: Self = Self(128u32);
    pub const INDIRECT_BUFFER: Self = Self(256u32);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(512u32);
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(1024u32);
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(2048u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(4096u32);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(8192u32);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(16384u32);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(32768u32);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(65536u32);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(131072u32);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(524288u32);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1048576u32);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(2097152u32);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(4194304u32);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(8388608u32);
    pub const MICROMAP_STORAGE_EXT: Self = Self(16777216u32);
    #[cfg(feature = "beta")]
    pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432u32);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(67108864u32);
    pub const TILE_MEMORY_QCOM: Self = Self(134217728u32);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(268435456u32);
}

impl From<BufferUsageFlagBits> for BufferUsageFlags {
    fn from(value: BufferUsageFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for BufferUsageFlagBits {
    type Output = BufferUsageFlags;
    fn bitor(self, rhs: Self) -> BufferUsageFlags {
        BufferUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferUsageFlags> for BufferUsageFlagBits {
    type Output = BufferUsageFlags;
    fn bitor(self, rhs: BufferUsageFlags) -> BufferUsageFlags {
        BufferUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferUsageFlagBits> for BufferUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: BufferUsageFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BufferUsageFlagBits> for BufferUsageFlags {
    fn bitor_assign(&mut self, rhs: BufferUsageFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferUsageFlagBits2(pub u64);

impl BufferUsageFlagBits2 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const BUFFER_USAGE_2_TRANSFER_SRC: Self = Self(1u64);
    pub const BUFFER_USAGE_2_TRANSFER_DST: Self = Self(2u64);
    pub const BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER: Self = Self(4u64);
    pub const BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER: Self = Self(8u64);
    pub const BUFFER_USAGE_2_UNIFORM_BUFFER: Self = Self(16u64);
    pub const BUFFER_USAGE_2_STORAGE_BUFFER: Self = Self(32u64);
    pub const BUFFER_USAGE_2_INDEX_BUFFER: Self = Self(64u64);
    pub const BUFFER_USAGE_2_VERTEX_BUFFER: Self = Self(128u64);
    pub const BUFFER_USAGE_2_INDIRECT_BUFFER: Self = Self(256u64);
    pub const BUFFER_USAGE_2_CONDITIONAL_RENDERING_EXT: Self = Self(512u64);
    pub const BUFFER_USAGE_2_SHADER_BINDING_TABLE_KHR: Self = Self(1024u64);
    pub const BUFFER_USAGE_2_TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(2048u64);
    pub const BUFFER_USAGE_2_TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(4096u64);
    pub const BUFFER_USAGE_2_VIDEO_DECODE_SRC_KHR: Self = Self(8192u64);
    pub const BUFFER_USAGE_2_VIDEO_DECODE_DST_KHR: Self = Self(16384u64);
    pub const BUFFER_USAGE_2_VIDEO_ENCODE_DST_KHR: Self = Self(32768u64);
    pub const BUFFER_USAGE_2_VIDEO_ENCODE_SRC_KHR: Self = Self(65536u64);
    pub const BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS: Self = Self(131072u64);
    pub const BUFFER_USAGE_2_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
        Self(524288u64);
    pub const BUFFER_USAGE_2_ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1048576u64);
    pub const BUFFER_USAGE_2_SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(2097152u64);
    pub const BUFFER_USAGE_2_RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(4194304u64);
    pub const BUFFER_USAGE_2_MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(8388608u64);
    pub const BUFFER_USAGE_2_MICROMAP_STORAGE_EXT: Self = Self(16777216u64);
    #[cfg(feature = "beta")]
    pub const BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432u64);
    pub const BUFFER_USAGE_2_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(67108864u64);
    pub const BUFFER_USAGE_2_TILE_MEMORY_QCOM: Self = Self(134217728u64);
    pub const BUFFER_USAGE_2_DESCRIPTOR_HEAP_EXT: Self = Self(268435456u64);
    pub const BUFFER_USAGE_2_DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self = Self(536870912u64);
    pub const BUFFER_USAGE_2_PREPROCESS_BUFFER_EXT: Self = Self(2147483648u64);
    pub const BUFFER_USAGE_2_MEMORY_DECOMPRESSION_EXT: Self = Self(4294967296u64);
    #[cfg(feature = "beta")]
    pub const BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX: Self = Self(8589934592u64);
}

impl From<BufferUsageFlagBits2> for BufferUsageFlags2 {
    fn from(value: BufferUsageFlagBits2) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for BufferUsageFlagBits2 {
    type Output = BufferUsageFlags2;
    fn bitor(self, rhs: Self) -> BufferUsageFlags2 {
        BufferUsageFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferUsageFlags2> for BufferUsageFlagBits2 {
    type Output = BufferUsageFlags2;
    fn bitor(self, rhs: BufferUsageFlags2) -> BufferUsageFlags2 {
        BufferUsageFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BufferUsageFlagBits2> for BufferUsageFlags2 {
    type Output = Self;
    fn bitor(self, rhs: BufferUsageFlagBits2) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BufferUsageFlagBits2> for BufferUsageFlags2 {
    fn bitor_assign(&mut self, rhs: BufferUsageFlagBits2) {
        self.0 |= rhs.0;
    }
}

pub type BufferUsageFlagBits2KHR = BufferUsageFlagBits2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildAccelerationStructureFlagBitsKHR(pub u32);

impl BuildAccelerationStructureFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_UPDATE_KHR: Self = Self(1u32);
    pub const ALLOW_COMPACTION_KHR: Self = Self(2u32);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(4u32);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(8u32);
    pub const LOW_MEMORY_KHR: Self = Self(16u32);
    pub const MOTION_NV: Self = Self(32u32);
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(64u32);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(128u32);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(256u32);
    #[cfg(feature = "beta")]
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(512u32);
    pub const ALLOW_DATA_ACCESS_KHR: Self = Self(2048u32);
    pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(4096u32);
}

impl From<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from(value: BuildAccelerationStructureFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for BuildAccelerationStructureFlagBitsKHR {
    type Output = BuildAccelerationStructureFlagsKHR;
    fn bitor(self, rhs: Self) -> BuildAccelerationStructureFlagsKHR {
        BuildAccelerationStructureFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BuildAccelerationStructureFlagsKHR>
    for BuildAccelerationStructureFlagBitsKHR
{
    type Output = BuildAccelerationStructureFlagsKHR;
    fn bitor(self, rhs: BuildAccelerationStructureFlagsKHR) -> BuildAccelerationStructureFlagsKHR {
        BuildAccelerationStructureFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BuildAccelerationStructureFlagBitsKHR>
    for BuildAccelerationStructureFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: BuildAccelerationStructureFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BuildAccelerationStructureFlagBitsKHR>
    for BuildAccelerationStructureFlagsKHR
{
    fn bitor_assign(&mut self, rhs: BuildAccelerationStructureFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

pub type BuildAccelerationStructureFlagBitsNV = BuildAccelerationStructureFlagBitsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildMicromapFlagBitsEXT(pub u32);

impl BuildMicromapFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PREFER_FAST_TRACE_EXT: Self = Self(1u32);
    pub const PREFER_FAST_BUILD_EXT: Self = Self(2u32);
    pub const ALLOW_COMPACTION_EXT: Self = Self(4u32);
}

impl From<BuildMicromapFlagBitsEXT> for BuildMicromapFlagsEXT {
    fn from(value: BuildMicromapFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for BuildMicromapFlagBitsEXT {
    type Output = BuildMicromapFlagsEXT;
    fn bitor(self, rhs: Self) -> BuildMicromapFlagsEXT {
        BuildMicromapFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BuildMicromapFlagsEXT> for BuildMicromapFlagBitsEXT {
    type Output = BuildMicromapFlagsEXT;
    fn bitor(self, rhs: BuildMicromapFlagsEXT) -> BuildMicromapFlagsEXT {
        BuildMicromapFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<BuildMicromapFlagBitsEXT> for BuildMicromapFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: BuildMicromapFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<BuildMicromapFlagBitsEXT> for BuildMicromapFlagsEXT {
    fn bitor_assign(&mut self, rhs: BuildMicromapFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureAddressResolutionFlagBitsNV(pub u32);

impl ClusterAccelerationStructureAddressResolutionFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_NV: Self = Self(0u32);
    pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(1u32);
    pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(2u32);
    pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(4u32);
    pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(8u32);
    pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(16u32);
    pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(32u32);
}

impl From<ClusterAccelerationStructureAddressResolutionFlagBitsNV>
    for ClusterAccelerationStructureAddressResolutionFlagsNV
{
    fn from(value: ClusterAccelerationStructureAddressResolutionFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureAddressResolutionFlagBitsNV {
    type Output = ClusterAccelerationStructureAddressResolutionFlagsNV;
    fn bitor(self, rhs: Self) -> ClusterAccelerationStructureAddressResolutionFlagsNV {
        ClusterAccelerationStructureAddressResolutionFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureAddressResolutionFlagsNV>
    for ClusterAccelerationStructureAddressResolutionFlagBitsNV
{
    type Output = ClusterAccelerationStructureAddressResolutionFlagsNV;
    fn bitor(
        self,
        rhs: ClusterAccelerationStructureAddressResolutionFlagsNV,
    ) -> ClusterAccelerationStructureAddressResolutionFlagsNV {
        ClusterAccelerationStructureAddressResolutionFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureAddressResolutionFlagBitsNV>
    for ClusterAccelerationStructureAddressResolutionFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: ClusterAccelerationStructureAddressResolutionFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ClusterAccelerationStructureAddressResolutionFlagBitsNV>
    for ClusterAccelerationStructureAddressResolutionFlagsNV
{
    fn bitor_assign(&mut self, rhs: ClusterAccelerationStructureAddressResolutionFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureClusterFlagBitsNV(pub u32);

impl ClusterAccelerationStructureClusterFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(1u32);
}

impl From<ClusterAccelerationStructureClusterFlagBitsNV>
    for ClusterAccelerationStructureClusterFlagsNV
{
    fn from(value: ClusterAccelerationStructureClusterFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureClusterFlagBitsNV {
    type Output = ClusterAccelerationStructureClusterFlagsNV;
    fn bitor(self, rhs: Self) -> ClusterAccelerationStructureClusterFlagsNV {
        ClusterAccelerationStructureClusterFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureClusterFlagsNV>
    for ClusterAccelerationStructureClusterFlagBitsNV
{
    type Output = ClusterAccelerationStructureClusterFlagsNV;
    fn bitor(
        self,
        rhs: ClusterAccelerationStructureClusterFlagsNV,
    ) -> ClusterAccelerationStructureClusterFlagsNV {
        ClusterAccelerationStructureClusterFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureClusterFlagBitsNV>
    for ClusterAccelerationStructureClusterFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: ClusterAccelerationStructureClusterFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ClusterAccelerationStructureClusterFlagBitsNV>
    for ClusterAccelerationStructureClusterFlagsNV
{
    fn bitor_assign(&mut self, rhs: ClusterAccelerationStructureClusterFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureGeometryFlagBitsNV(pub u32);

impl ClusterAccelerationStructureGeometryFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CULL_DISABLE_NV: Self = Self(1u32);
    pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(2u32);
    pub const OPAQUE_NV: Self = Self(4u32);
}

impl From<ClusterAccelerationStructureGeometryFlagBitsNV>
    for ClusterAccelerationStructureGeometryFlagsNV
{
    fn from(value: ClusterAccelerationStructureGeometryFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureGeometryFlagBitsNV {
    type Output = ClusterAccelerationStructureGeometryFlagsNV;
    fn bitor(self, rhs: Self) -> ClusterAccelerationStructureGeometryFlagsNV {
        ClusterAccelerationStructureGeometryFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureGeometryFlagsNV>
    for ClusterAccelerationStructureGeometryFlagBitsNV
{
    type Output = ClusterAccelerationStructureGeometryFlagsNV;
    fn bitor(
        self,
        rhs: ClusterAccelerationStructureGeometryFlagsNV,
    ) -> ClusterAccelerationStructureGeometryFlagsNV {
        ClusterAccelerationStructureGeometryFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureGeometryFlagBitsNV>
    for ClusterAccelerationStructureGeometryFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: ClusterAccelerationStructureGeometryFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ClusterAccelerationStructureGeometryFlagBitsNV>
    for ClusterAccelerationStructureGeometryFlagsNV
{
    fn bitor_assign(&mut self, rhs: ClusterAccelerationStructureGeometryFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureIndexFormatFlagBitsNV(pub u32);

impl ClusterAccelerationStructureIndexFormatFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _8BIT_NV: Self = Self(1u32);
    pub const _16BIT_NV: Self = Self(2u32);
    pub const _32BIT_NV: Self = Self(4u32);
}

impl From<ClusterAccelerationStructureIndexFormatFlagBitsNV>
    for ClusterAccelerationStructureIndexFormatFlagsNV
{
    fn from(value: ClusterAccelerationStructureIndexFormatFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureIndexFormatFlagBitsNV {
    type Output = ClusterAccelerationStructureIndexFormatFlagsNV;
    fn bitor(self, rhs: Self) -> ClusterAccelerationStructureIndexFormatFlagsNV {
        ClusterAccelerationStructureIndexFormatFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureIndexFormatFlagsNV>
    for ClusterAccelerationStructureIndexFormatFlagBitsNV
{
    type Output = ClusterAccelerationStructureIndexFormatFlagsNV;
    fn bitor(
        self,
        rhs: ClusterAccelerationStructureIndexFormatFlagsNV,
    ) -> ClusterAccelerationStructureIndexFormatFlagsNV {
        ClusterAccelerationStructureIndexFormatFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ClusterAccelerationStructureIndexFormatFlagBitsNV>
    for ClusterAccelerationStructureIndexFormatFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: ClusterAccelerationStructureIndexFormatFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ClusterAccelerationStructureIndexFormatFlagBitsNV>
    for ClusterAccelerationStructureIndexFormatFlagsNV
{
    fn bitor_assign(&mut self, rhs: ClusterAccelerationStructureIndexFormatFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ColorComponentFlagBits(pub u32);

impl ColorComponentFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const R: Self = Self(1u32);
    pub const G: Self = Self(2u32);
    pub const B: Self = Self(4u32);
    pub const A: Self = Self(8u32);
}

impl From<ColorComponentFlagBits> for ColorComponentFlags {
    fn from(value: ColorComponentFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ColorComponentFlagBits {
    type Output = ColorComponentFlags;
    fn bitor(self, rhs: Self) -> ColorComponentFlags {
        ColorComponentFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ColorComponentFlags> for ColorComponentFlagBits {
    type Output = ColorComponentFlags;
    fn bitor(self, rhs: ColorComponentFlags) -> ColorComponentFlags {
        ColorComponentFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ColorComponentFlagBits> for ColorComponentFlags {
    type Output = Self;
    fn bitor(self, rhs: ColorComponentFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ColorComponentFlagBits> for ColorComponentFlags {
    fn bitor_assign(&mut self, rhs: ColorComponentFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBufferResetFlagBits(pub u32);

impl CommandBufferResetFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RELEASE_RESOURCES: Self = Self(1u32);
}

impl From<CommandBufferResetFlagBits> for CommandBufferResetFlags {
    fn from(value: CommandBufferResetFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CommandBufferResetFlagBits {
    type Output = CommandBufferResetFlags;
    fn bitor(self, rhs: Self) -> CommandBufferResetFlags {
        CommandBufferResetFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandBufferResetFlags> for CommandBufferResetFlagBits {
    type Output = CommandBufferResetFlags;
    fn bitor(self, rhs: CommandBufferResetFlags) -> CommandBufferResetFlags {
        CommandBufferResetFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandBufferResetFlagBits> for CommandBufferResetFlags {
    type Output = Self;
    fn bitor(self, rhs: CommandBufferResetFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CommandBufferResetFlagBits> for CommandBufferResetFlags {
    fn bitor_assign(&mut self, rhs: CommandBufferResetFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBufferUsageFlagBits(pub u32);

impl CommandBufferUsageFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ONE_TIME_SUBMIT: Self = Self(1u32);
    pub const RENDER_PASS_CONTINUE: Self = Self(2u32);
    pub const SIMULTANEOUS_USE: Self = Self(4u32);
}

impl From<CommandBufferUsageFlagBits> for CommandBufferUsageFlags {
    fn from(value: CommandBufferUsageFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CommandBufferUsageFlagBits {
    type Output = CommandBufferUsageFlags;
    fn bitor(self, rhs: Self) -> CommandBufferUsageFlags {
        CommandBufferUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandBufferUsageFlags> for CommandBufferUsageFlagBits {
    type Output = CommandBufferUsageFlags;
    fn bitor(self, rhs: CommandBufferUsageFlags) -> CommandBufferUsageFlags {
        CommandBufferUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandBufferUsageFlagBits> for CommandBufferUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: CommandBufferUsageFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CommandBufferUsageFlagBits> for CommandBufferUsageFlags {
    fn bitor_assign(&mut self, rhs: CommandBufferUsageFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandPoolCreateFlagBits(pub u32);

impl CommandPoolCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSIENT: Self = Self(1u32);
    pub const RESET_COMMAND_BUFFER: Self = Self(2u32);
    pub const PROTECTED: Self = Self(4u32);
}

impl From<CommandPoolCreateFlagBits> for CommandPoolCreateFlags {
    fn from(value: CommandPoolCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CommandPoolCreateFlagBits {
    type Output = CommandPoolCreateFlags;
    fn bitor(self, rhs: Self) -> CommandPoolCreateFlags {
        CommandPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandPoolCreateFlags> for CommandPoolCreateFlagBits {
    type Output = CommandPoolCreateFlags;
    fn bitor(self, rhs: CommandPoolCreateFlags) -> CommandPoolCreateFlags {
        CommandPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandPoolCreateFlagBits> for CommandPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: CommandPoolCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CommandPoolCreateFlagBits> for CommandPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: CommandPoolCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandPoolResetFlagBits(pub u32);

impl CommandPoolResetFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RELEASE_RESOURCES: Self = Self(1u32);
}

impl From<CommandPoolResetFlagBits> for CommandPoolResetFlags {
    fn from(value: CommandPoolResetFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CommandPoolResetFlagBits {
    type Output = CommandPoolResetFlags;
    fn bitor(self, rhs: Self) -> CommandPoolResetFlags {
        CommandPoolResetFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandPoolResetFlags> for CommandPoolResetFlagBits {
    type Output = CommandPoolResetFlags;
    fn bitor(self, rhs: CommandPoolResetFlags) -> CommandPoolResetFlags {
        CommandPoolResetFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CommandPoolResetFlagBits> for CommandPoolResetFlags {
    type Output = Self;
    fn bitor(self, rhs: CommandPoolResetFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CommandPoolResetFlagBits> for CommandPoolResetFlags {
    fn bitor_assign(&mut self, rhs: CommandPoolResetFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CompositeAlphaFlagBitsKHR(pub u32);

impl CompositeAlphaFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const PRE_MULTIPLIED_KHR: Self = Self(2u32);
    pub const POST_MULTIPLIED_KHR: Self = Self(4u32);
    pub const INHERIT_KHR: Self = Self(8u32);
}

impl From<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn from(value: CompositeAlphaFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CompositeAlphaFlagBitsKHR {
    type Output = CompositeAlphaFlagsKHR;
    fn bitor(self, rhs: Self) -> CompositeAlphaFlagsKHR {
        CompositeAlphaFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CompositeAlphaFlagsKHR> for CompositeAlphaFlagBitsKHR {
    type Output = CompositeAlphaFlagsKHR;
    fn bitor(self, rhs: CompositeAlphaFlagsKHR) -> CompositeAlphaFlagsKHR {
        CompositeAlphaFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: CompositeAlphaFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CompositeAlphaFlagBitsKHR> for CompositeAlphaFlagsKHR {
    fn bitor_assign(&mut self, rhs: CompositeAlphaFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ConditionalRenderingFlagBitsEXT(pub u32);

impl ConditionalRenderingFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVERTED_EXT: Self = Self(1u32);
}

impl From<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn from(value: ConditionalRenderingFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ConditionalRenderingFlagBitsEXT {
    type Output = ConditionalRenderingFlagsEXT;
    fn bitor(self, rhs: Self) -> ConditionalRenderingFlagsEXT {
        ConditionalRenderingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ConditionalRenderingFlagsEXT> for ConditionalRenderingFlagBitsEXT {
    type Output = ConditionalRenderingFlagsEXT;
    fn bitor(self, rhs: ConditionalRenderingFlagsEXT) -> ConditionalRenderingFlagsEXT {
        ConditionalRenderingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: ConditionalRenderingFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ConditionalRenderingFlagBitsEXT> for ConditionalRenderingFlagsEXT {
    fn bitor_assign(&mut self, rhs: ConditionalRenderingFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CullModeFlagBits(pub u32);

impl CullModeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const FRONT: Self = Self(1u32);
    pub const BACK: Self = Self(2u32);
    pub const FRONT_AND_BACK: Self = Self(3u32);
}

impl From<CullModeFlagBits> for CullModeFlags {
    fn from(value: CullModeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for CullModeFlagBits {
    type Output = CullModeFlags;
    fn bitor(self, rhs: Self) -> CullModeFlags {
        CullModeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CullModeFlags> for CullModeFlagBits {
    type Output = CullModeFlags;
    fn bitor(self, rhs: CullModeFlags) -> CullModeFlags {
        CullModeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<CullModeFlagBits> for CullModeFlags {
    type Output = Self;
    fn bitor(self, rhs: CullModeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<CullModeFlagBits> for CullModeFlags {
    fn bitor_assign(&mut self, rhs: CullModeFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowCreateFlagBitsARM(pub u32);

impl DataGraphOpticalFlowCreateFlagBitsARM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_HINT_ARM: Self = Self(1u32);
    pub const ENABLE_COST_ARM: Self = Self(2u32);
    pub const RESERVED_30_ARM: Self = Self(1073741824u32);
}

impl From<DataGraphOpticalFlowCreateFlagBitsARM> for DataGraphOpticalFlowCreateFlagsARM {
    fn from(value: DataGraphOpticalFlowCreateFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowCreateFlagBitsARM {
    type Output = DataGraphOpticalFlowCreateFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphOpticalFlowCreateFlagsARM {
        DataGraphOpticalFlowCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowCreateFlagsARM>
    for DataGraphOpticalFlowCreateFlagBitsARM
{
    type Output = DataGraphOpticalFlowCreateFlagsARM;
    fn bitor(self, rhs: DataGraphOpticalFlowCreateFlagsARM) -> DataGraphOpticalFlowCreateFlagsARM {
        DataGraphOpticalFlowCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowCreateFlagBitsARM>
    for DataGraphOpticalFlowCreateFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: DataGraphOpticalFlowCreateFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphOpticalFlowCreateFlagBitsARM>
    for DataGraphOpticalFlowCreateFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphOpticalFlowCreateFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowExecuteFlagBitsARM(pub u32);

impl DataGraphOpticalFlowExecuteFlagBitsARM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_TEMPORAL_HINTS_ARM: Self = Self(1u32);
    pub const INPUT_UNCHANGED_ARM: Self = Self(2u32);
    pub const REFERENCE_UNCHANGED_ARM: Self = Self(4u32);
    pub const INPUT_IS_PREVIOUS_REFERENCE_ARM: Self = Self(8u32);
    pub const REFERENCE_IS_PREVIOUS_INPUT_ARM: Self = Self(16u32);
}

impl From<DataGraphOpticalFlowExecuteFlagBitsARM> for DataGraphOpticalFlowExecuteFlagsARM {
    fn from(value: DataGraphOpticalFlowExecuteFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowExecuteFlagBitsARM {
    type Output = DataGraphOpticalFlowExecuteFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphOpticalFlowExecuteFlagsARM {
        DataGraphOpticalFlowExecuteFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowExecuteFlagsARM>
    for DataGraphOpticalFlowExecuteFlagBitsARM
{
    type Output = DataGraphOpticalFlowExecuteFlagsARM;
    fn bitor(
        self,
        rhs: DataGraphOpticalFlowExecuteFlagsARM,
    ) -> DataGraphOpticalFlowExecuteFlagsARM {
        DataGraphOpticalFlowExecuteFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowExecuteFlagBitsARM>
    for DataGraphOpticalFlowExecuteFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: DataGraphOpticalFlowExecuteFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphOpticalFlowExecuteFlagBitsARM>
    for DataGraphOpticalFlowExecuteFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphOpticalFlowExecuteFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowGridSizeFlagBitsARM(pub u32);

impl DataGraphOpticalFlowGridSizeFlagBitsARM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_ARM: Self = Self(0u32);
    pub const _1X1_ARM: Self = Self(1u32);
    pub const _2X2_ARM: Self = Self(2u32);
    pub const _4X4_ARM: Self = Self(4u32);
    pub const _8X8_ARM: Self = Self(8u32);
}

impl From<DataGraphOpticalFlowGridSizeFlagBitsARM> for DataGraphOpticalFlowGridSizeFlagsARM {
    fn from(value: DataGraphOpticalFlowGridSizeFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowGridSizeFlagBitsARM {
    type Output = DataGraphOpticalFlowGridSizeFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphOpticalFlowGridSizeFlagsARM {
        DataGraphOpticalFlowGridSizeFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowGridSizeFlagsARM>
    for DataGraphOpticalFlowGridSizeFlagBitsARM
{
    type Output = DataGraphOpticalFlowGridSizeFlagsARM;
    fn bitor(
        self,
        rhs: DataGraphOpticalFlowGridSizeFlagsARM,
    ) -> DataGraphOpticalFlowGridSizeFlagsARM {
        DataGraphOpticalFlowGridSizeFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowGridSizeFlagBitsARM>
    for DataGraphOpticalFlowGridSizeFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: DataGraphOpticalFlowGridSizeFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphOpticalFlowGridSizeFlagBitsARM>
    for DataGraphOpticalFlowGridSizeFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphOpticalFlowGridSizeFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowImageUsageFlagBitsARM(pub u32);

impl DataGraphOpticalFlowImageUsageFlagBitsARM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_ARM: Self = Self(0u32);
    pub const INPUT_ARM: Self = Self(1u32);
    pub const OUTPUT_ARM: Self = Self(2u32);
    pub const HINT_ARM: Self = Self(4u32);
    pub const COST_ARM: Self = Self(8u32);
}

impl From<DataGraphOpticalFlowImageUsageFlagBitsARM> for DataGraphOpticalFlowImageUsageFlagsARM {
    fn from(value: DataGraphOpticalFlowImageUsageFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowImageUsageFlagBitsARM {
    type Output = DataGraphOpticalFlowImageUsageFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphOpticalFlowImageUsageFlagsARM {
        DataGraphOpticalFlowImageUsageFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowImageUsageFlagsARM>
    for DataGraphOpticalFlowImageUsageFlagBitsARM
{
    type Output = DataGraphOpticalFlowImageUsageFlagsARM;
    fn bitor(
        self,
        rhs: DataGraphOpticalFlowImageUsageFlagsARM,
    ) -> DataGraphOpticalFlowImageUsageFlagsARM {
        DataGraphOpticalFlowImageUsageFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphOpticalFlowImageUsageFlagBitsARM>
    for DataGraphOpticalFlowImageUsageFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: DataGraphOpticalFlowImageUsageFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphOpticalFlowImageUsageFlagBitsARM>
    for DataGraphOpticalFlowImageUsageFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphOpticalFlowImageUsageFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineDispatchFlagBitsARM(pub u64);

impl DataGraphPipelineDispatchFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
}

impl From<DataGraphPipelineDispatchFlagBitsARM> for DataGraphPipelineDispatchFlagsARM {
    fn from(value: DataGraphPipelineDispatchFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphPipelineDispatchFlagBitsARM {
    type Output = DataGraphPipelineDispatchFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphPipelineDispatchFlagsARM {
        DataGraphPipelineDispatchFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphPipelineDispatchFlagsARM> for DataGraphPipelineDispatchFlagBitsARM {
    type Output = DataGraphPipelineDispatchFlagsARM;
    fn bitor(self, rhs: DataGraphPipelineDispatchFlagsARM) -> DataGraphPipelineDispatchFlagsARM {
        DataGraphPipelineDispatchFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphPipelineDispatchFlagBitsARM> for DataGraphPipelineDispatchFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: DataGraphPipelineDispatchFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphPipelineDispatchFlagBitsARM>
    for DataGraphPipelineDispatchFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphPipelineDispatchFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineSessionCreateFlagBitsARM(pub u64);

impl DataGraphPipelineSessionCreateFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PROTECTED_ARM: Self = Self(1u64);
    pub const OPTICAL_FLOW_CACHE_ARM: Self = Self(2u64);
}

impl From<DataGraphPipelineSessionCreateFlagBitsARM> for DataGraphPipelineSessionCreateFlagsARM {
    fn from(value: DataGraphPipelineSessionCreateFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphPipelineSessionCreateFlagBitsARM {
    type Output = DataGraphPipelineSessionCreateFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphPipelineSessionCreateFlagsARM {
        DataGraphPipelineSessionCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphPipelineSessionCreateFlagsARM>
    for DataGraphPipelineSessionCreateFlagBitsARM
{
    type Output = DataGraphPipelineSessionCreateFlagsARM;
    fn bitor(
        self,
        rhs: DataGraphPipelineSessionCreateFlagsARM,
    ) -> DataGraphPipelineSessionCreateFlagsARM {
        DataGraphPipelineSessionCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphPipelineSessionCreateFlagBitsARM>
    for DataGraphPipelineSessionCreateFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: DataGraphPipelineSessionCreateFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphPipelineSessionCreateFlagBitsARM>
    for DataGraphPipelineSessionCreateFlagsARM
{
    fn bitor_assign(&mut self, rhs: DataGraphPipelineSessionCreateFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphTOSAQualityFlagBitsARM(pub u32);

impl DataGraphTOSAQualityFlagBitsARM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ACCELERATED_ARM: Self = Self(1u32);
    pub const CONFORMANT_ARM: Self = Self(2u32);
    pub const EXPERIMENTAL_ARM: Self = Self(4u32);
    pub const DEPRECATED_ARM: Self = Self(8u32);
}

impl From<DataGraphTOSAQualityFlagBitsARM> for DataGraphTOSAQualityFlagsARM {
    fn from(value: DataGraphTOSAQualityFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DataGraphTOSAQualityFlagBitsARM {
    type Output = DataGraphTOSAQualityFlagsARM;
    fn bitor(self, rhs: Self) -> DataGraphTOSAQualityFlagsARM {
        DataGraphTOSAQualityFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphTOSAQualityFlagsARM> for DataGraphTOSAQualityFlagBitsARM {
    type Output = DataGraphTOSAQualityFlagsARM;
    fn bitor(self, rhs: DataGraphTOSAQualityFlagsARM) -> DataGraphTOSAQualityFlagsARM {
        DataGraphTOSAQualityFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DataGraphTOSAQualityFlagBitsARM> for DataGraphTOSAQualityFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: DataGraphTOSAQualityFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DataGraphTOSAQualityFlagBitsARM> for DataGraphTOSAQualityFlagsARM {
    fn bitor_assign(&mut self, rhs: DataGraphTOSAQualityFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugReportFlagBitsEXT(pub u32);

impl DebugReportFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INFORMATION_EXT: Self = Self(1u32);
    pub const WARNING_EXT: Self = Self(2u32);
    pub const PERFORMANCE_WARNING_EXT: Self = Self(4u32);
    pub const ERROR_EXT: Self = Self(8u32);
    pub const DEBUG_EXT: Self = Self(16u32);
}

impl From<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn from(value: DebugReportFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DebugReportFlagBitsEXT {
    type Output = DebugReportFlagsEXT;
    fn bitor(self, rhs: Self) -> DebugReportFlagsEXT {
        DebugReportFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugReportFlagsEXT> for DebugReportFlagBitsEXT {
    type Output = DebugReportFlagsEXT;
    fn bitor(self, rhs: DebugReportFlagsEXT) -> DebugReportFlagsEXT {
        DebugReportFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: DebugReportFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DebugReportFlagBitsEXT> for DebugReportFlagsEXT {
    fn bitor_assign(&mut self, rhs: DebugReportFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessageSeverityFlagBitsEXT(pub u32);

impl DebugUtilsMessageSeverityFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERBOSE: Self = Self(1u32);
    pub const INFO: Self = Self(16u32);
    pub const WARNING: Self = Self(256u32);
    pub const ERROR: Self = Self(4096u32);
}

impl From<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    fn from(value: DebugUtilsMessageSeverityFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DebugUtilsMessageSeverityFlagBitsEXT {
    type Output = DebugUtilsMessageSeverityFlagsEXT;
    fn bitor(self, rhs: Self) -> DebugUtilsMessageSeverityFlagsEXT {
        DebugUtilsMessageSeverityFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugUtilsMessageSeverityFlagsEXT> for DebugUtilsMessageSeverityFlagBitsEXT {
    type Output = DebugUtilsMessageSeverityFlagsEXT;
    fn bitor(self, rhs: DebugUtilsMessageSeverityFlagsEXT) -> DebugUtilsMessageSeverityFlagsEXT {
        DebugUtilsMessageSeverityFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugUtilsMessageSeverityFlagBitsEXT> for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: DebugUtilsMessageSeverityFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DebugUtilsMessageSeverityFlagBitsEXT>
    for DebugUtilsMessageSeverityFlagsEXT
{
    fn bitor_assign(&mut self, rhs: DebugUtilsMessageSeverityFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessageTypeFlagBitsEXT(pub u32);

impl DebugUtilsMessageTypeFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const GENERAL_EXT: Self = Self(1u32);
    pub const VALIDATION_EXT: Self = Self(2u32);
    pub const PERFORMANCE_EXT: Self = Self(4u32);
    pub const DEVICE_ADDRESS_BINDING_EXT: Self = Self(8u32);
}

impl From<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn from(value: DebugUtilsMessageTypeFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DebugUtilsMessageTypeFlagBitsEXT {
    type Output = DebugUtilsMessageTypeFlagsEXT;
    fn bitor(self, rhs: Self) -> DebugUtilsMessageTypeFlagsEXT {
        DebugUtilsMessageTypeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugUtilsMessageTypeFlagsEXT> for DebugUtilsMessageTypeFlagBitsEXT {
    type Output = DebugUtilsMessageTypeFlagsEXT;
    fn bitor(self, rhs: DebugUtilsMessageTypeFlagsEXT) -> DebugUtilsMessageTypeFlagsEXT {
        DebugUtilsMessageTypeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: DebugUtilsMessageTypeFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DebugUtilsMessageTypeFlagBitsEXT> for DebugUtilsMessageTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: DebugUtilsMessageTypeFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DependencyFlagBits(pub u32);

impl DependencyFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BY_REGION: Self = Self(1u32);
    pub const VIEW_LOCAL: Self = Self(2u32);
    pub const DEVICE_GROUP: Self = Self(4u32);
    pub const FEEDBACK_LOOP_EXT: Self = Self(8u32);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self = Self(32u32);
    pub const ASYMMETRIC_EVENT_KHR: Self = Self(64u32);
}

impl From<DependencyFlagBits> for DependencyFlags {
    fn from(value: DependencyFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DependencyFlagBits {
    type Output = DependencyFlags;
    fn bitor(self, rhs: Self) -> DependencyFlags {
        DependencyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DependencyFlags> for DependencyFlagBits {
    type Output = DependencyFlags;
    fn bitor(self, rhs: DependencyFlags) -> DependencyFlags {
        DependencyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DependencyFlagBits> for DependencyFlags {
    type Output = Self;
    fn bitor(self, rhs: DependencyFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DependencyFlagBits> for DependencyFlags {
    fn bitor_assign(&mut self, rhs: DependencyFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorBindingFlagBits(pub u32);

impl DescriptorBindingFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UPDATE_AFTER_BIND: Self = Self(1u32);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2u32);
    pub const PARTIALLY_BOUND: Self = Self(4u32);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8u32);
}

impl From<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn from(value: DescriptorBindingFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DescriptorBindingFlagBits {
    type Output = DescriptorBindingFlags;
    fn bitor(self, rhs: Self) -> DescriptorBindingFlags {
        DescriptorBindingFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorBindingFlags> for DescriptorBindingFlagBits {
    type Output = DescriptorBindingFlags;
    fn bitor(self, rhs: DescriptorBindingFlags) -> DescriptorBindingFlags {
        DescriptorBindingFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    type Output = Self;
    fn bitor(self, rhs: DescriptorBindingFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DescriptorBindingFlagBits> for DescriptorBindingFlags {
    fn bitor_assign(&mut self, rhs: DescriptorBindingFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type DescriptorBindingFlagBitsEXT = DescriptorBindingFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorPoolCreateFlagBits(pub u32);

impl DescriptorPoolCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FREE_DESCRIPTOR_SET: Self = Self(1u32);
    pub const UPDATE_AFTER_BIND: Self = Self(2u32);
    pub const HOST_ONLY_EXT: Self = Self(4u32);
    pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(8u32);
    pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(16u32);
}

impl From<DescriptorPoolCreateFlagBits> for DescriptorPoolCreateFlags {
    fn from(value: DescriptorPoolCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DescriptorPoolCreateFlagBits {
    type Output = DescriptorPoolCreateFlags;
    fn bitor(self, rhs: Self) -> DescriptorPoolCreateFlags {
        DescriptorPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorPoolCreateFlags> for DescriptorPoolCreateFlagBits {
    type Output = DescriptorPoolCreateFlags;
    fn bitor(self, rhs: DescriptorPoolCreateFlags) -> DescriptorPoolCreateFlags {
        DescriptorPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorPoolCreateFlagBits> for DescriptorPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: DescriptorPoolCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DescriptorPoolCreateFlagBits> for DescriptorPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: DescriptorPoolCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorSetLayoutCreateFlagBits(pub u32);

impl DescriptorSetLayoutCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PUSH_DESCRIPTOR: Self = Self(1u32);
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(2u32);
    pub const HOST_ONLY_POOL_EXT: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(16u32);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(32u32);
    pub const PER_STAGE_NV: Self = Self(64u32);
    pub const INDIRECT_BINDABLE_NV: Self = Self(128u32);
}

impl From<DescriptorSetLayoutCreateFlagBits> for DescriptorSetLayoutCreateFlags {
    fn from(value: DescriptorSetLayoutCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DescriptorSetLayoutCreateFlagBits {
    type Output = DescriptorSetLayoutCreateFlags;
    fn bitor(self, rhs: Self) -> DescriptorSetLayoutCreateFlags {
        DescriptorSetLayoutCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorSetLayoutCreateFlags> for DescriptorSetLayoutCreateFlagBits {
    type Output = DescriptorSetLayoutCreateFlags;
    fn bitor(self, rhs: DescriptorSetLayoutCreateFlags) -> DescriptorSetLayoutCreateFlags {
        DescriptorSetLayoutCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DescriptorSetLayoutCreateFlagBits> for DescriptorSetLayoutCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: DescriptorSetLayoutCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DescriptorSetLayoutCreateFlagBits> for DescriptorSetLayoutCreateFlags {
    fn bitor_assign(&mut self, rhs: DescriptorSetLayoutCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceAddressBindingFlagBitsEXT(pub u32);

impl DeviceAddressBindingFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INTERNAL_OBJECT_EXT: Self = Self(1u32);
}

impl From<DeviceAddressBindingFlagBitsEXT> for DeviceAddressBindingFlagsEXT {
    fn from(value: DeviceAddressBindingFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DeviceAddressBindingFlagBitsEXT {
    type Output = DeviceAddressBindingFlagsEXT;
    fn bitor(self, rhs: Self) -> DeviceAddressBindingFlagsEXT {
        DeviceAddressBindingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceAddressBindingFlagsEXT> for DeviceAddressBindingFlagBitsEXT {
    type Output = DeviceAddressBindingFlagsEXT;
    fn bitor(self, rhs: DeviceAddressBindingFlagsEXT) -> DeviceAddressBindingFlagsEXT {
        DeviceAddressBindingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceAddressBindingFlagBitsEXT> for DeviceAddressBindingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: DeviceAddressBindingFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DeviceAddressBindingFlagBitsEXT> for DeviceAddressBindingFlagsEXT {
    fn bitor_assign(&mut self, rhs: DeviceAddressBindingFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceDiagnosticsConfigFlagBitsNV(pub u32);

impl DeviceDiagnosticsConfigFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1u32);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(2u32);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(4u32);
    pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(8u32);
}

impl From<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn from(value: DeviceDiagnosticsConfigFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DeviceDiagnosticsConfigFlagBitsNV {
    type Output = DeviceDiagnosticsConfigFlagsNV;
    fn bitor(self, rhs: Self) -> DeviceDiagnosticsConfigFlagsNV {
        DeviceDiagnosticsConfigFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceDiagnosticsConfigFlagsNV> for DeviceDiagnosticsConfigFlagBitsNV {
    type Output = DeviceDiagnosticsConfigFlagsNV;
    fn bitor(self, rhs: DeviceDiagnosticsConfigFlagsNV) -> DeviceDiagnosticsConfigFlagsNV {
        DeviceDiagnosticsConfigFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: DeviceDiagnosticsConfigFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DeviceDiagnosticsConfigFlagBitsNV> for DeviceDiagnosticsConfigFlagsNV {
    fn bitor_assign(&mut self, rhs: DeviceDiagnosticsConfigFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceFaultFlagBitsKHR(pub u32);

impl DeviceFaultFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_DEVICE_LOST_KHR: Self = Self(1u32);
    pub const FLAG_MEMORY_ADDRESS_KHR: Self = Self(2u32);
    pub const FLAG_INSTRUCTION_ADDRESS_KHR: Self = Self(4u32);
    pub const FLAG_VENDOR_KHR: Self = Self(8u32);
    pub const FLAG_WATCHDOG_TIMEOUT_KHR: Self = Self(16u32);
    pub const FLAG_OVERFLOW_KHR: Self = Self(32u32);
}

impl From<DeviceFaultFlagBitsKHR> for DeviceFaultFlagsKHR {
    fn from(value: DeviceFaultFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DeviceFaultFlagBitsKHR {
    type Output = DeviceFaultFlagsKHR;
    fn bitor(self, rhs: Self) -> DeviceFaultFlagsKHR {
        DeviceFaultFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceFaultFlagsKHR> for DeviceFaultFlagBitsKHR {
    type Output = DeviceFaultFlagsKHR;
    fn bitor(self, rhs: DeviceFaultFlagsKHR) -> DeviceFaultFlagsKHR {
        DeviceFaultFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceFaultFlagBitsKHR> for DeviceFaultFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: DeviceFaultFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DeviceFaultFlagBitsKHR> for DeviceFaultFlagsKHR {
    fn bitor_assign(&mut self, rhs: DeviceFaultFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceGroupPresentModeFlagBitsKHR(pub u32);

impl DeviceGroupPresentModeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const LOCAL_KHR: Self = Self(1u32);
    pub const REMOTE_KHR: Self = Self(2u32);
    pub const SUM_KHR: Self = Self(4u32);
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(8u32);
}

impl From<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn from(value: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DeviceGroupPresentModeFlagBitsKHR {
    type Output = DeviceGroupPresentModeFlagsKHR;
    fn bitor(self, rhs: Self) -> DeviceGroupPresentModeFlagsKHR {
        DeviceGroupPresentModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceGroupPresentModeFlagsKHR> for DeviceGroupPresentModeFlagBitsKHR {
    type Output = DeviceGroupPresentModeFlagsKHR;
    fn bitor(self, rhs: DeviceGroupPresentModeFlagsKHR) -> DeviceGroupPresentModeFlagsKHR {
        DeviceGroupPresentModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: DeviceGroupPresentModeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DeviceGroupPresentModeFlagBitsKHR> for DeviceGroupPresentModeFlagsKHR {
    fn bitor_assign(&mut self, rhs: DeviceGroupPresentModeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceQueueCreateFlagBits(pub u32);

impl DeviceQueueCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED: Self = Self(1u32);
    pub const INTERNALLY_SYNCHRONIZED_KHR: Self = Self(4u32);
}

impl From<DeviceQueueCreateFlagBits> for DeviceQueueCreateFlags {
    fn from(value: DeviceQueueCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DeviceQueueCreateFlagBits {
    type Output = DeviceQueueCreateFlags;
    fn bitor(self, rhs: Self) -> DeviceQueueCreateFlags {
        DeviceQueueCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceQueueCreateFlags> for DeviceQueueCreateFlagBits {
    type Output = DeviceQueueCreateFlags;
    fn bitor(self, rhs: DeviceQueueCreateFlags) -> DeviceQueueCreateFlags {
        DeviceQueueCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DeviceQueueCreateFlagBits> for DeviceQueueCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: DeviceQueueCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DeviceQueueCreateFlagBits> for DeviceQueueCreateFlags {
    fn bitor_assign(&mut self, rhs: DeviceQueueCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplayPlaneAlphaFlagBitsKHR(pub u32);

impl DisplayPlaneAlphaFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const GLOBAL_KHR: Self = Self(2u32);
    pub const PER_PIXEL_KHR: Self = Self(4u32);
    pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(8u32);
}

impl From<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn from(value: DisplayPlaneAlphaFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for DisplayPlaneAlphaFlagBitsKHR {
    type Output = DisplayPlaneAlphaFlagsKHR;
    fn bitor(self, rhs: Self) -> DisplayPlaneAlphaFlagsKHR {
        DisplayPlaneAlphaFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DisplayPlaneAlphaFlagsKHR> for DisplayPlaneAlphaFlagBitsKHR {
    type Output = DisplayPlaneAlphaFlagsKHR;
    fn bitor(self, rhs: DisplayPlaneAlphaFlagsKHR) -> DisplayPlaneAlphaFlagsKHR {
        DisplayPlaneAlphaFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: DisplayPlaneAlphaFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<DisplayPlaneAlphaFlagBitsKHR> for DisplayPlaneAlphaFlagsKHR {
    fn bitor_assign(&mut self, rhs: DisplayPlaneAlphaFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct EventCreateFlagBits(pub u32);

impl EventCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ONLY: Self = Self(1u32);
}

impl From<EventCreateFlagBits> for EventCreateFlags {
    fn from(value: EventCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for EventCreateFlagBits {
    type Output = EventCreateFlags;
    fn bitor(self, rhs: Self) -> EventCreateFlags {
        EventCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<EventCreateFlags> for EventCreateFlagBits {
    type Output = EventCreateFlags;
    fn bitor(self, rhs: EventCreateFlags) -> EventCreateFlags {
        EventCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<EventCreateFlagBits> for EventCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: EventCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<EventCreateFlagBits> for EventCreateFlags {
    fn bitor_assign(&mut self, rhs: EventCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "metal")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExportMetalObjectTypeFlagBitsEXT(pub u32);

#[cfg(feature = "metal")]
impl ExportMetalObjectTypeFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const METAL_DEVICE_EXT: Self = Self(1u32);
    pub const METAL_COMMAND_QUEUE_EXT: Self = Self(2u32);
    pub const METAL_BUFFER_EXT: Self = Self(4u32);
    pub const METAL_TEXTURE_EXT: Self = Self(8u32);
    pub const METAL_IOSURFACE_EXT: Self = Self(16u32);
    pub const METAL_SHARED_EVENT_EXT: Self = Self(32u32);
}

#[cfg(feature = "metal")]
impl From<ExportMetalObjectTypeFlagBitsEXT> for ExportMetalObjectTypeFlagsEXT {
    fn from(value: ExportMetalObjectTypeFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOr for ExportMetalObjectTypeFlagBitsEXT {
    type Output = ExportMetalObjectTypeFlagsEXT;
    fn bitor(self, rhs: Self) -> ExportMetalObjectTypeFlagsEXT {
        ExportMetalObjectTypeFlagsEXT(self.0 | rhs.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOr<ExportMetalObjectTypeFlagsEXT> for ExportMetalObjectTypeFlagBitsEXT {
    type Output = ExportMetalObjectTypeFlagsEXT;
    fn bitor(self, rhs: ExportMetalObjectTypeFlagsEXT) -> ExportMetalObjectTypeFlagsEXT {
        ExportMetalObjectTypeFlagsEXT(self.0 | rhs.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOr<ExportMetalObjectTypeFlagBitsEXT> for ExportMetalObjectTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: ExportMetalObjectTypeFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOrAssign<ExportMetalObjectTypeFlagBitsEXT> for ExportMetalObjectTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: ExportMetalObjectTypeFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalFenceFeatureFlagBits(pub u32);

impl ExternalFenceFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPORTABLE: Self = Self(1u32);
    pub const IMPORTABLE: Self = Self(2u32);
}

impl From<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn from(value: ExternalFenceFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalFenceFeatureFlagBits {
    type Output = ExternalFenceFeatureFlags;
    fn bitor(self, rhs: Self) -> ExternalFenceFeatureFlags {
        ExternalFenceFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalFenceFeatureFlags> for ExternalFenceFeatureFlagBits {
    type Output = ExternalFenceFeatureFlags;
    fn bitor(self, rhs: ExternalFenceFeatureFlags) -> ExternalFenceFeatureFlags {
        ExternalFenceFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalFenceFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalFenceFeatureFlagBits> for ExternalFenceFeatureFlags {
    fn bitor_assign(&mut self, rhs: ExternalFenceFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalFenceFeatureFlagBitsKHR = ExternalFenceFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalFenceHandleTypeFlagBits(pub u32);

impl ExternalFenceHandleTypeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const SYNC_FD: Self = Self(8u32);
}

impl From<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn from(value: ExternalFenceHandleTypeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalFenceHandleTypeFlagBits {
    type Output = ExternalFenceHandleTypeFlags;
    fn bitor(self, rhs: Self) -> ExternalFenceHandleTypeFlags {
        ExternalFenceHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalFenceHandleTypeFlags> for ExternalFenceHandleTypeFlagBits {
    type Output = ExternalFenceHandleTypeFlags;
    fn bitor(self, rhs: ExternalFenceHandleTypeFlags) -> ExternalFenceHandleTypeFlags {
        ExternalFenceHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalFenceHandleTypeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalFenceHandleTypeFlagBits> for ExternalFenceHandleTypeFlags {
    fn bitor_assign(&mut self, rhs: ExternalFenceHandleTypeFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalFenceHandleTypeFlagBitsKHR = ExternalFenceHandleTypeFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryFeatureFlagBits(pub u32);

impl ExternalMemoryFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEDICATED_ONLY: Self = Self(1u32);
    pub const EXPORTABLE: Self = Self(2u32);
    pub const IMPORTABLE: Self = Self(4u32);
}

impl From<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn from(value: ExternalMemoryFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalMemoryFeatureFlagBits {
    type Output = ExternalMemoryFeatureFlags;
    fn bitor(self, rhs: Self) -> ExternalMemoryFeatureFlags {
        ExternalMemoryFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryFeatureFlags> for ExternalMemoryFeatureFlagBits {
    type Output = ExternalMemoryFeatureFlags;
    fn bitor(self, rhs: ExternalMemoryFeatureFlags) -> ExternalMemoryFeatureFlags {
        ExternalMemoryFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalMemoryFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalMemoryFeatureFlagBits> for ExternalMemoryFeatureFlags {
    fn bitor_assign(&mut self, rhs: ExternalMemoryFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalMemoryFeatureFlagBitsKHR = ExternalMemoryFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryFeatureFlagBitsNV(pub u32);

impl ExternalMemoryFeatureFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEDICATED_ONLY_NV: Self = Self(1u32);
    pub const EXPORTABLE_NV: Self = Self(2u32);
    pub const IMPORTABLE_NV: Self = Self(4u32);
}

impl From<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn from(value: ExternalMemoryFeatureFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalMemoryFeatureFlagBitsNV {
    type Output = ExternalMemoryFeatureFlagsNV;
    fn bitor(self, rhs: Self) -> ExternalMemoryFeatureFlagsNV {
        ExternalMemoryFeatureFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryFeatureFlagsNV> for ExternalMemoryFeatureFlagBitsNV {
    type Output = ExternalMemoryFeatureFlagsNV;
    fn bitor(self, rhs: ExternalMemoryFeatureFlagsNV) -> ExternalMemoryFeatureFlagsNV {
        ExternalMemoryFeatureFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: ExternalMemoryFeatureFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalMemoryFeatureFlagBitsNV> for ExternalMemoryFeatureFlagsNV {
    fn bitor_assign(&mut self, rhs: ExternalMemoryFeatureFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryHandleTypeFlagBits(pub u32);

impl ExternalMemoryHandleTypeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const D3D11_TEXTURE: Self = Self(8u32);
    pub const D3D11_TEXTURE_KMT: Self = Self(16u32);
    pub const D3D12_HEAP: Self = Self(32u32);
    pub const D3D12_RESOURCE: Self = Self(64u32);
    pub const HOST_ALLOCATION_EXT: Self = Self(128u32);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256u32);
    pub const DMA_BUF_EXT: Self = Self(512u32);
    pub const ANDROID_HARDWARE_BUFFER_BIT_ANDROID: Self = Self(1024u32);
    pub const ZIRCON_VMO_BIT_FUCHSIA: Self = Self(2048u32);
    pub const RDMA_ADDRESS_NV: Self = Self(4096u32);
    pub const SCREEN_BUFFER_BIT_QNX: Self = Self(16384u32);
    pub const OH_NATIVE_BUFFER_BIT_OHOS: Self = Self(32768u32);
    pub const MTLBUFFER_EXT: Self = Self(65536u32);
    pub const MTLTEXTURE_EXT: Self = Self(131072u32);
    pub const MTLHEAP_EXT: Self = Self(262144u32);
}

impl From<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn from(value: ExternalMemoryHandleTypeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalMemoryHandleTypeFlagBits {
    type Output = ExternalMemoryHandleTypeFlags;
    fn bitor(self, rhs: Self) -> ExternalMemoryHandleTypeFlags {
        ExternalMemoryHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryHandleTypeFlags> for ExternalMemoryHandleTypeFlagBits {
    type Output = ExternalMemoryHandleTypeFlags;
    fn bitor(self, rhs: ExternalMemoryHandleTypeFlags) -> ExternalMemoryHandleTypeFlags {
        ExternalMemoryHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalMemoryHandleTypeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalMemoryHandleTypeFlagBits> for ExternalMemoryHandleTypeFlags {
    fn bitor_assign(&mut self, rhs: ExternalMemoryHandleTypeFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalMemoryHandleTypeFlagBitsKHR = ExternalMemoryHandleTypeFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryHandleTypeFlagBitsNV(pub u32);

impl ExternalMemoryHandleTypeFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_WIN32_NV: Self = Self(1u32);
    pub const OPAQUE_WIN32_KMT_NV: Self = Self(2u32);
    pub const D3D11_IMAGE_NV: Self = Self(4u32);
    pub const D3D11_IMAGE_KMT_NV: Self = Self(8u32);
}

impl From<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    fn from(value: ExternalMemoryHandleTypeFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalMemoryHandleTypeFlagBitsNV {
    type Output = ExternalMemoryHandleTypeFlagsNV;
    fn bitor(self, rhs: Self) -> ExternalMemoryHandleTypeFlagsNV {
        ExternalMemoryHandleTypeFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryHandleTypeFlagsNV> for ExternalMemoryHandleTypeFlagBitsNV {
    type Output = ExternalMemoryHandleTypeFlagsNV;
    fn bitor(self, rhs: ExternalMemoryHandleTypeFlagsNV) -> ExternalMemoryHandleTypeFlagsNV {
        ExternalMemoryHandleTypeFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalMemoryHandleTypeFlagBitsNV> for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: ExternalMemoryHandleTypeFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalMemoryHandleTypeFlagBitsNV>
    for ExternalMemoryHandleTypeFlagsNV
{
    fn bitor_assign(&mut self, rhs: ExternalMemoryHandleTypeFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalSemaphoreFeatureFlagBits(pub u32);

impl ExternalSemaphoreFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPORTABLE: Self = Self(1u32);
    pub const IMPORTABLE: Self = Self(2u32);
}

impl From<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn from(value: ExternalSemaphoreFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalSemaphoreFeatureFlagBits {
    type Output = ExternalSemaphoreFeatureFlags;
    fn bitor(self, rhs: Self) -> ExternalSemaphoreFeatureFlags {
        ExternalSemaphoreFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalSemaphoreFeatureFlags> for ExternalSemaphoreFeatureFlagBits {
    type Output = ExternalSemaphoreFeatureFlags;
    fn bitor(self, rhs: ExternalSemaphoreFeatureFlags) -> ExternalSemaphoreFeatureFlags {
        ExternalSemaphoreFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalSemaphoreFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalSemaphoreFeatureFlagBits> for ExternalSemaphoreFeatureFlags {
    fn bitor_assign(&mut self, rhs: ExternalSemaphoreFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalSemaphoreFeatureFlagBitsKHR = ExternalSemaphoreFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalSemaphoreHandleTypeFlagBits(pub u32);

impl ExternalSemaphoreHandleTypeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const D3D12_FENCE: Self = Self(8u32);
    pub const SYNC_FD: Self = Self(16u32);
    pub const ZIRCON_EVENT_BIT_FUCHSIA: Self = Self(128u32);
}

impl From<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    fn from(value: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ExternalSemaphoreHandleTypeFlagBits {
    type Output = ExternalSemaphoreHandleTypeFlags;
    fn bitor(self, rhs: Self) -> ExternalSemaphoreHandleTypeFlags {
        ExternalSemaphoreHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalSemaphoreHandleTypeFlags> for ExternalSemaphoreHandleTypeFlagBits {
    type Output = ExternalSemaphoreHandleTypeFlags;
    fn bitor(self, rhs: ExternalSemaphoreHandleTypeFlags) -> ExternalSemaphoreHandleTypeFlags {
        ExternalSemaphoreHandleTypeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ExternalSemaphoreHandleTypeFlagBits> for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: ExternalSemaphoreHandleTypeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ExternalSemaphoreHandleTypeFlagBits>
    for ExternalSemaphoreHandleTypeFlags
{
    fn bitor_assign(&mut self, rhs: ExternalSemaphoreHandleTypeFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ExternalSemaphoreHandleTypeFlagBitsKHR = ExternalSemaphoreHandleTypeFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FenceCreateFlagBits(pub u32);

impl FenceCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SIGNALED: Self = Self(1u32);
}

impl From<FenceCreateFlagBits> for FenceCreateFlags {
    fn from(value: FenceCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FenceCreateFlagBits {
    type Output = FenceCreateFlags;
    fn bitor(self, rhs: Self) -> FenceCreateFlags {
        FenceCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FenceCreateFlags> for FenceCreateFlagBits {
    type Output = FenceCreateFlags;
    fn bitor(self, rhs: FenceCreateFlags) -> FenceCreateFlags {
        FenceCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FenceCreateFlagBits> for FenceCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: FenceCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FenceCreateFlagBits> for FenceCreateFlags {
    fn bitor_assign(&mut self, rhs: FenceCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FenceImportFlagBits(pub u32);

impl FenceImportFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TEMPORARY: Self = Self(1u32);
}

impl From<FenceImportFlagBits> for FenceImportFlags {
    fn from(value: FenceImportFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FenceImportFlagBits {
    type Output = FenceImportFlags;
    fn bitor(self, rhs: Self) -> FenceImportFlags {
        FenceImportFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FenceImportFlags> for FenceImportFlagBits {
    type Output = FenceImportFlags;
    fn bitor(self, rhs: FenceImportFlags) -> FenceImportFlags {
        FenceImportFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FenceImportFlagBits> for FenceImportFlags {
    type Output = Self;
    fn bitor(self, rhs: FenceImportFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FenceImportFlagBits> for FenceImportFlags {
    fn bitor_assign(&mut self, rhs: FenceImportFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type FenceImportFlagBitsKHR = FenceImportFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FormatFeatureFlagBits(pub u32);

impl FormatFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SAMPLED_IMAGE: Self = Self(1u32);
    pub const STORAGE_IMAGE: Self = Self(2u32);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(4u32);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(8u32);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(16u32);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32u32);
    pub const VERTEX_BUFFER: Self = Self(64u32);
    pub const COLOR_ATTACHMENT: Self = Self(128u32);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(256u32);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(512u32);
    pub const BLIT_SRC: Self = Self(1024u32);
    pub const BLIT_DST: Self = Self(2048u32);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096u32);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(8192u32);
    pub const TRANSFER_SRC: Self = Self(16384u32);
    pub const TRANSFER_DST: Self = Self(32768u32);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536u32);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(131072u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(524288u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(1048576u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(2097152u32);
    pub const DISJOINT: Self = Self(4194304u32);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(8388608u32);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216u32);
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432u32);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(67108864u32);
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728u32);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(268435456u32);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824u32);
}

impl From<FormatFeatureFlagBits> for FormatFeatureFlags {
    fn from(value: FormatFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FormatFeatureFlagBits {
    type Output = FormatFeatureFlags;
    fn bitor(self, rhs: Self) -> FormatFeatureFlags {
        FormatFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FormatFeatureFlags> for FormatFeatureFlagBits {
    type Output = FormatFeatureFlags;
    fn bitor(self, rhs: FormatFeatureFlags) -> FormatFeatureFlags {
        FormatFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FormatFeatureFlagBits> for FormatFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: FormatFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FormatFeatureFlagBits> for FormatFeatureFlags {
    fn bitor_assign(&mut self, rhs: FormatFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FormatFeatureFlagBits2(pub u64);

impl FormatFeatureFlagBits2 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE: Self = Self(1u64);
    pub const FORMAT_FEATURE_2_STORAGE_IMAGE: Self = Self(2u64);
    pub const FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC: Self = Self(4u64);
    pub const FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER: Self = Self(8u64);
    pub const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER: Self = Self(16u64);
    pub const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32u64);
    pub const FORMAT_FEATURE_2_VERTEX_BUFFER: Self = Self(64u64);
    pub const FORMAT_FEATURE_2_COLOR_ATTACHMENT: Self = Self(128u64);
    pub const FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND: Self = Self(256u64);
    pub const FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT: Self = Self(512u64);
    pub const FORMAT_FEATURE_2_BLIT_SRC: Self = Self(1024u64);
    pub const FORMAT_FEATURE_2_BLIT_DST: Self = Self(2048u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(8192u64);
    pub const FORMAT_FEATURE_2_TRANSFER_SRC: Self = Self(16384u64);
    pub const FORMAT_FEATURE_2_TRANSFER_DST: Self = Self(32768u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536u64);
    pub const FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES: Self = Self(131072u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        Self(524288u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(1048576u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(2097152u64);
    pub const FORMAT_FEATURE_2_DISJOINT: Self = Self(4194304u64);
    pub const FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES: Self = Self(8388608u64);
    pub const FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216u64);
    pub const FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432u64);
    pub const FORMAT_FEATURE_2_VIDEO_DECODE_DPB_KHR: Self = Self(67108864u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_KHR: Self = Self(268435456u64);
    pub const FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912u64);
    pub const FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824u64);
    pub const FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT: Self = Self(2147483648u64);
    pub const FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(4294967296u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(8589934592u64);
    pub const FORMAT_FEATURE_2_WEIGHT_IMAGE_QCOM: Self = Self(17179869184u64);
    pub const FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(34359738368u64);
    pub const FORMAT_FEATURE_2_BLOCK_MATCHING_QCOM: Self = Self(68719476736u64);
    pub const FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_QCOM: Self = Self(137438953472u64);
    pub const FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_NV: Self = Self(274877906944u64);
    pub const FORMAT_FEATURE_2_TENSOR_SHADER_ARM: Self = Self(549755813888u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_NV: Self = Self(1099511627776u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_NV: Self = Self(2199023255552u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_COST_NV: Self = Self(4398046511104u64);
    pub const FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_ARM: Self = Self(8796093022208u64);
    pub const FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER: Self = Self(70368744177664u64);
    pub const FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_ARM: Self = Self(281474976710656u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self =
        Self(562949953421312u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1125899906842624u64);
    pub const FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self =
        Self(2251799813685248u64);
    pub const FORMAT_FEATURE_2_DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(4503599627370496u64);
    pub const FORMAT_FEATURE_2_DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(9007199254740992u64);
    pub const FORMAT_FEATURE_2_STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(18014398509481984u64);
    pub const FORMAT_FEATURE_2_STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self =
        Self(36028797018963968u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM: Self = Self(72057594037927936u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM: Self =
        Self(144115188075855872u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_COST_ARM: Self = Self(288230376151711744u64);
    pub const FORMAT_FEATURE_2_COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(576460752303423488u64);
}

impl From<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    fn from(value: FormatFeatureFlagBits2) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FormatFeatureFlagBits2 {
    type Output = FormatFeatureFlags2;
    fn bitor(self, rhs: Self) -> FormatFeatureFlags2 {
        FormatFeatureFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FormatFeatureFlags2> for FormatFeatureFlagBits2 {
    type Output = FormatFeatureFlags2;
    fn bitor(self, rhs: FormatFeatureFlags2) -> FormatFeatureFlags2 {
        FormatFeatureFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    type Output = Self;
    fn bitor(self, rhs: FormatFeatureFlagBits2) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FormatFeatureFlagBits2> for FormatFeatureFlags2 {
    fn bitor_assign(&mut self, rhs: FormatFeatureFlagBits2) {
        self.0 |= rhs.0;
    }
}

pub type FormatFeatureFlagBits2KHR = FormatFeatureFlagBits2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FrameBoundaryFlagBitsEXT(pub u32);

impl FrameBoundaryFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRAME_END_EXT: Self = Self(1u32);
}

impl From<FrameBoundaryFlagBitsEXT> for FrameBoundaryFlagsEXT {
    fn from(value: FrameBoundaryFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FrameBoundaryFlagBitsEXT {
    type Output = FrameBoundaryFlagsEXT;
    fn bitor(self, rhs: Self) -> FrameBoundaryFlagsEXT {
        FrameBoundaryFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FrameBoundaryFlagsEXT> for FrameBoundaryFlagBitsEXT {
    type Output = FrameBoundaryFlagsEXT;
    fn bitor(self, rhs: FrameBoundaryFlagsEXT) -> FrameBoundaryFlagsEXT {
        FrameBoundaryFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FrameBoundaryFlagBitsEXT> for FrameBoundaryFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: FrameBoundaryFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FrameBoundaryFlagBitsEXT> for FrameBoundaryFlagsEXT {
    fn bitor_assign(&mut self, rhs: FrameBoundaryFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FramebufferCreateFlagBits(pub u32);

impl FramebufferCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const IMAGELESS: Self = Self(1u32);
}

impl From<FramebufferCreateFlagBits> for FramebufferCreateFlags {
    fn from(value: FramebufferCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for FramebufferCreateFlagBits {
    type Output = FramebufferCreateFlags;
    fn bitor(self, rhs: Self) -> FramebufferCreateFlags {
        FramebufferCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FramebufferCreateFlags> for FramebufferCreateFlagBits {
    type Output = FramebufferCreateFlags;
    fn bitor(self, rhs: FramebufferCreateFlags) -> FramebufferCreateFlags {
        FramebufferCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<FramebufferCreateFlagBits> for FramebufferCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: FramebufferCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<FramebufferCreateFlagBits> for FramebufferCreateFlags {
    fn bitor_assign(&mut self, rhs: FramebufferCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GeometryFlagBitsKHR(pub u32);

impl GeometryFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(2u32);
}

impl From<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from(value: GeometryFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for GeometryFlagBitsKHR {
    type Output = GeometryFlagsKHR;
    fn bitor(self, rhs: Self) -> GeometryFlagsKHR {
        GeometryFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GeometryFlagsKHR> for GeometryFlagBitsKHR {
    type Output = GeometryFlagsKHR;
    fn bitor(self, rhs: GeometryFlagsKHR) -> GeometryFlagsKHR {
        GeometryFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: GeometryFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn bitor_assign(&mut self, rhs: GeometryFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

pub type GeometryFlagBitsNV = GeometryFlagBitsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GeometryInstanceFlagBitsKHR(pub u32);

impl GeometryInstanceFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1u32);
    pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(2u32);
    pub const FORCE_OPAQUE_KHR: Self = Self(4u32);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(8u32);
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(16u32);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(32u32);
}

impl From<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from(value: GeometryInstanceFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for GeometryInstanceFlagBitsKHR {
    type Output = GeometryInstanceFlagsKHR;
    fn bitor(self, rhs: Self) -> GeometryInstanceFlagsKHR {
        GeometryInstanceFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GeometryInstanceFlagsKHR> for GeometryInstanceFlagBitsKHR {
    type Output = GeometryInstanceFlagsKHR;
    fn bitor(self, rhs: GeometryInstanceFlagsKHR) -> GeometryInstanceFlagsKHR {
        GeometryInstanceFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: GeometryInstanceFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn bitor_assign(&mut self, rhs: GeometryInstanceFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

pub type GeometryInstanceFlagBitsNV = GeometryInstanceFlagBitsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GraphicsPipelineLibraryFlagBitsEXT(pub u32);

impl GraphicsPipelineLibraryFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1u32);
    pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(2u32);
    pub const FRAGMENT_SHADER_EXT: Self = Self(4u32);
    pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(8u32);
}

impl From<GraphicsPipelineLibraryFlagBitsEXT> for GraphicsPipelineLibraryFlagsEXT {
    fn from(value: GraphicsPipelineLibraryFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for GraphicsPipelineLibraryFlagBitsEXT {
    type Output = GraphicsPipelineLibraryFlagsEXT;
    fn bitor(self, rhs: Self) -> GraphicsPipelineLibraryFlagsEXT {
        GraphicsPipelineLibraryFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GraphicsPipelineLibraryFlagsEXT> for GraphicsPipelineLibraryFlagBitsEXT {
    type Output = GraphicsPipelineLibraryFlagsEXT;
    fn bitor(self, rhs: GraphicsPipelineLibraryFlagsEXT) -> GraphicsPipelineLibraryFlagsEXT {
        GraphicsPipelineLibraryFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<GraphicsPipelineLibraryFlagBitsEXT> for GraphicsPipelineLibraryFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: GraphicsPipelineLibraryFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<GraphicsPipelineLibraryFlagBitsEXT>
    for GraphicsPipelineLibraryFlagsEXT
{
    fn bitor_assign(&mut self, rhs: GraphicsPipelineLibraryFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct HostImageCopyFlagBits(pub u32);

impl HostImageCopyFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MEMCPY: Self = Self(1u32);
}

impl From<HostImageCopyFlagBits> for HostImageCopyFlags {
    fn from(value: HostImageCopyFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for HostImageCopyFlagBits {
    type Output = HostImageCopyFlags;
    fn bitor(self, rhs: Self) -> HostImageCopyFlags {
        HostImageCopyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<HostImageCopyFlags> for HostImageCopyFlagBits {
    type Output = HostImageCopyFlags;
    fn bitor(self, rhs: HostImageCopyFlags) -> HostImageCopyFlags {
        HostImageCopyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<HostImageCopyFlagBits> for HostImageCopyFlags {
    type Output = Self;
    fn bitor(self, rhs: HostImageCopyFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<HostImageCopyFlagBits> for HostImageCopyFlags {
    fn bitor_assign(&mut self, rhs: HostImageCopyFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type HostImageCopyFlagBitsEXT = HostImageCopyFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageAspectFlagBits(pub u32);

impl ImageAspectFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const COLOR: Self = Self(1u32);
    pub const DEPTH: Self = Self(2u32);
    pub const STENCIL: Self = Self(4u32);
    pub const METADATA: Self = Self(8u32);
    pub const PLANE_0: Self = Self(16u32);
    pub const PLANE_1: Self = Self(32u32);
    pub const PLANE_2: Self = Self(64u32);
    pub const MEMORY_PLANE_0_EXT: Self = Self(128u32);
    pub const MEMORY_PLANE_1_EXT: Self = Self(256u32);
    pub const MEMORY_PLANE_2_EXT: Self = Self(512u32);
    pub const MEMORY_PLANE_3_EXT: Self = Self(1024u32);
}

impl From<ImageAspectFlagBits> for ImageAspectFlags {
    fn from(value: ImageAspectFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageAspectFlagBits {
    type Output = ImageAspectFlags;
    fn bitor(self, rhs: Self) -> ImageAspectFlags {
        ImageAspectFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageAspectFlags> for ImageAspectFlagBits {
    type Output = ImageAspectFlags;
    fn bitor(self, rhs: ImageAspectFlags) -> ImageAspectFlags {
        ImageAspectFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageAspectFlagBits> for ImageAspectFlags {
    type Output = Self;
    fn bitor(self, rhs: ImageAspectFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageAspectFlagBits> for ImageAspectFlags {
    fn bitor_assign(&mut self, rhs: ImageAspectFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCompressionFixedRateFlagBitsEXT(pub u32);

impl ImageCompressionFixedRateFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_EXT: Self = Self(0u32);
    pub const _1BPC_EXT: Self = Self(1u32);
    pub const _2BPC_EXT: Self = Self(2u32);
    pub const _3BPC_EXT: Self = Self(4u32);
    pub const _4BPC_EXT: Self = Self(8u32);
    pub const _5BPC_EXT: Self = Self(16u32);
    pub const _6BPC_EXT: Self = Self(32u32);
    pub const _7BPC_EXT: Self = Self(64u32);
    pub const _8BPC_EXT: Self = Self(128u32);
    pub const _9BPC_EXT: Self = Self(256u32);
    pub const _10BPC_EXT: Self = Self(512u32);
    pub const _11BPC_EXT: Self = Self(1024u32);
    pub const _12BPC_EXT: Self = Self(2048u32);
    pub const _13BPC_EXT: Self = Self(4096u32);
    pub const _14BPC_EXT: Self = Self(8192u32);
    pub const _15BPC_EXT: Self = Self(16384u32);
    pub const _16BPC_EXT: Self = Self(32768u32);
    pub const _17BPC_EXT: Self = Self(65536u32);
    pub const _18BPC_EXT: Self = Self(131072u32);
    pub const _19BPC_EXT: Self = Self(262144u32);
    pub const _20BPC_EXT: Self = Self(524288u32);
    pub const _21BPC_EXT: Self = Self(1048576u32);
    pub const _22BPC_EXT: Self = Self(2097152u32);
    pub const _23BPC_EXT: Self = Self(4194304u32);
    pub const _24BPC_EXT: Self = Self(8388608u32);
}

impl From<ImageCompressionFixedRateFlagBitsEXT> for ImageCompressionFixedRateFlagsEXT {
    fn from(value: ImageCompressionFixedRateFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageCompressionFixedRateFlagBitsEXT {
    type Output = ImageCompressionFixedRateFlagsEXT;
    fn bitor(self, rhs: Self) -> ImageCompressionFixedRateFlagsEXT {
        ImageCompressionFixedRateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCompressionFixedRateFlagsEXT> for ImageCompressionFixedRateFlagBitsEXT {
    type Output = ImageCompressionFixedRateFlagsEXT;
    fn bitor(self, rhs: ImageCompressionFixedRateFlagsEXT) -> ImageCompressionFixedRateFlagsEXT {
        ImageCompressionFixedRateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCompressionFixedRateFlagBitsEXT> for ImageCompressionFixedRateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: ImageCompressionFixedRateFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageCompressionFixedRateFlagBitsEXT>
    for ImageCompressionFixedRateFlagsEXT
{
    fn bitor_assign(&mut self, rhs: ImageCompressionFixedRateFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCompressionFlagBitsEXT(pub u32);

impl ImageCompressionFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_EXT: Self = Self(0u32);
    pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1u32);
    pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(2u32);
    pub const DISABLED_EXT: Self = Self(4u32);
}

impl From<ImageCompressionFlagBitsEXT> for ImageCompressionFlagsEXT {
    fn from(value: ImageCompressionFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageCompressionFlagBitsEXT {
    type Output = ImageCompressionFlagsEXT;
    fn bitor(self, rhs: Self) -> ImageCompressionFlagsEXT {
        ImageCompressionFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCompressionFlagsEXT> for ImageCompressionFlagBitsEXT {
    type Output = ImageCompressionFlagsEXT;
    fn bitor(self, rhs: ImageCompressionFlagsEXT) -> ImageCompressionFlagsEXT {
        ImageCompressionFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCompressionFlagBitsEXT> for ImageCompressionFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: ImageCompressionFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageCompressionFlagBitsEXT> for ImageCompressionFlagsEXT {
    fn bitor_assign(&mut self, rhs: ImageCompressionFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageConstraintsInfoFlagBitsFUCHSIA(pub u32);

#[cfg(feature = "fuchsia")]
impl ImageConstraintsInfoFlagBitsFUCHSIA {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1u32);
    pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(2u32);
    pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(4u32);
    pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(8u32);
    pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(16u32);
}

#[cfg(feature = "fuchsia")]
impl From<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    fn from(value: ImageConstraintsInfoFlagBitsFUCHSIA) -> Self {
        Self(value.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr for ImageConstraintsInfoFlagBitsFUCHSIA {
    type Output = ImageConstraintsInfoFlagsFUCHSIA;
    fn bitor(self, rhs: Self) -> ImageConstraintsInfoFlagsFUCHSIA {
        ImageConstraintsInfoFlagsFUCHSIA(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr<ImageConstraintsInfoFlagsFUCHSIA> for ImageConstraintsInfoFlagBitsFUCHSIA {
    type Output = ImageConstraintsInfoFlagsFUCHSIA;
    fn bitor(self, rhs: ImageConstraintsInfoFlagsFUCHSIA) -> ImageConstraintsInfoFlagsFUCHSIA {
        ImageConstraintsInfoFlagsFUCHSIA(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr<ImageConstraintsInfoFlagBitsFUCHSIA> for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    fn bitor(self, rhs: ImageConstraintsInfoFlagBitsFUCHSIA) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOrAssign<ImageConstraintsInfoFlagBitsFUCHSIA>
    for ImageConstraintsInfoFlagsFUCHSIA
{
    fn bitor_assign(&mut self, rhs: ImageConstraintsInfoFlagBitsFUCHSIA) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCreateFlagBits(pub u32);

impl ImageCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPARSE_BINDING: Self = Self(1u32);
    pub const SPARSE_RESIDENCY: Self = Self(2u32);
    pub const SPARSE_ALIASED: Self = Self(4u32);
    pub const MUTABLE_FORMAT: Self = Self(8u32);
    pub const CUBE_COMPATIBLE: Self = Self(16u32);
    pub const _2D_ARRAY_COMPATIBLE: Self = Self(32u32);
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(64u32);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(128u32);
    pub const EXTENDED_USAGE: Self = Self(256u32);
    pub const DISJOINT: Self = Self(512u32);
    pub const ALIAS: Self = Self(1024u32);
    pub const PROTECTED: Self = Self(2048u32);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(4096u32);
    pub const CORNER_SAMPLED_NV: Self = Self(8192u32);
    pub const SUBSAMPLED_EXT: Self = Self(16384u32);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(32768u32);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self = Self(65536u32);
    pub const _2D_VIEW_COMPATIBLE_EXT: Self = Self(131072u32);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(262144u32);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1048576u32);
    pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR: Self = Self(4194304u32);
}

impl From<ImageCreateFlagBits> for ImageCreateFlags {
    fn from(value: ImageCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageCreateFlagBits {
    type Output = ImageCreateFlags;
    fn bitor(self, rhs: Self) -> ImageCreateFlags {
        ImageCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCreateFlags> for ImageCreateFlagBits {
    type Output = ImageCreateFlags;
    fn bitor(self, rhs: ImageCreateFlags) -> ImageCreateFlags {
        ImageCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageCreateFlagBits> for ImageCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: ImageCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageCreateFlagBits> for ImageCreateFlags {
    fn bitor_assign(&mut self, rhs: ImageCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageUsageFlagBits(pub u32);

impl ImageUsageFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFER_SRC: Self = Self(1u32);
    pub const TRANSFER_DST: Self = Self(2u32);
    pub const SAMPLED: Self = Self(4u32);
    pub const STORAGE: Self = Self(8u32);
    pub const COLOR_ATTACHMENT: Self = Self(16u32);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(32u32);
    pub const TRANSIENT_ATTACHMENT: Self = Self(64u32);
    pub const INPUT_ATTACHMENT: Self = Self(128u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(256u32);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(512u32);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1024u32);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(2048u32);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(4096u32);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(8192u32);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(16384u32);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(32768u32);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144u32);
    pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(524288u32);
    pub const SAMPLE_WEIGHT_QCOM: Self = Self(1048576u32);
    pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(2097152u32);
    pub const HOST_TRANSFER: Self = Self(4194304u32);
    pub const TENSOR_ALIASING_ARM: Self = Self(8388608u32);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(33554432u32);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(67108864u32);
    pub const TILE_MEMORY_QCOM: Self = Self(134217728u32);
}

impl From<ImageUsageFlagBits> for ImageUsageFlags {
    fn from(value: ImageUsageFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageUsageFlagBits {
    type Output = ImageUsageFlags;
    fn bitor(self, rhs: Self) -> ImageUsageFlags {
        ImageUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageUsageFlags> for ImageUsageFlagBits {
    type Output = ImageUsageFlags;
    fn bitor(self, rhs: ImageUsageFlags) -> ImageUsageFlags {
        ImageUsageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageUsageFlagBits> for ImageUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: ImageUsageFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageUsageFlagBits> for ImageUsageFlags {
    fn bitor_assign(&mut self, rhs: ImageUsageFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageViewCreateFlagBits(pub u32);

impl ImageViewCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(1u32);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(2u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(4u32);
}

impl From<ImageViewCreateFlagBits> for ImageViewCreateFlags {
    fn from(value: ImageViewCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ImageViewCreateFlagBits {
    type Output = ImageViewCreateFlags;
    fn bitor(self, rhs: Self) -> ImageViewCreateFlags {
        ImageViewCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageViewCreateFlags> for ImageViewCreateFlagBits {
    type Output = ImageViewCreateFlags;
    fn bitor(self, rhs: ImageViewCreateFlags) -> ImageViewCreateFlags {
        ImageViewCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ImageViewCreateFlagBits> for ImageViewCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: ImageViewCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ImageViewCreateFlagBits> for ImageViewCreateFlags {
    fn bitor_assign(&mut self, rhs: ImageViewCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsInputModeFlagBitsEXT(pub u32);

impl IndirectCommandsInputModeFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1u32);
    pub const DXGI_INDEX_BUFFER_EXT: Self = Self(2u32);
}

impl From<IndirectCommandsInputModeFlagBitsEXT> for IndirectCommandsInputModeFlagsEXT {
    fn from(value: IndirectCommandsInputModeFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for IndirectCommandsInputModeFlagBitsEXT {
    type Output = IndirectCommandsInputModeFlagsEXT;
    fn bitor(self, rhs: Self) -> IndirectCommandsInputModeFlagsEXT {
        IndirectCommandsInputModeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsInputModeFlagsEXT> for IndirectCommandsInputModeFlagBitsEXT {
    type Output = IndirectCommandsInputModeFlagsEXT;
    fn bitor(self, rhs: IndirectCommandsInputModeFlagsEXT) -> IndirectCommandsInputModeFlagsEXT {
        IndirectCommandsInputModeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsInputModeFlagBitsEXT> for IndirectCommandsInputModeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: IndirectCommandsInputModeFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<IndirectCommandsInputModeFlagBitsEXT>
    for IndirectCommandsInputModeFlagsEXT
{
    fn bitor_assign(&mut self, rhs: IndirectCommandsInputModeFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsLayoutUsageFlagBitsEXT(pub u32);

impl IndirectCommandsLayoutUsageFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPLICIT_PREPROCESS_EXT: Self = Self(1u32);
    pub const UNORDERED_SEQUENCES_EXT: Self = Self(2u32);
}

impl From<IndirectCommandsLayoutUsageFlagBitsEXT> for IndirectCommandsLayoutUsageFlagsEXT {
    fn from(value: IndirectCommandsLayoutUsageFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for IndirectCommandsLayoutUsageFlagBitsEXT {
    type Output = IndirectCommandsLayoutUsageFlagsEXT;
    fn bitor(self, rhs: Self) -> IndirectCommandsLayoutUsageFlagsEXT {
        IndirectCommandsLayoutUsageFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsLayoutUsageFlagsEXT>
    for IndirectCommandsLayoutUsageFlagBitsEXT
{
    type Output = IndirectCommandsLayoutUsageFlagsEXT;
    fn bitor(
        self,
        rhs: IndirectCommandsLayoutUsageFlagsEXT,
    ) -> IndirectCommandsLayoutUsageFlagsEXT {
        IndirectCommandsLayoutUsageFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsLayoutUsageFlagBitsEXT>
    for IndirectCommandsLayoutUsageFlagsEXT
{
    type Output = Self;
    fn bitor(self, rhs: IndirectCommandsLayoutUsageFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<IndirectCommandsLayoutUsageFlagBitsEXT>
    for IndirectCommandsLayoutUsageFlagsEXT
{
    fn bitor_assign(&mut self, rhs: IndirectCommandsLayoutUsageFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(pub u32);

impl IndirectCommandsLayoutUsageFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(1u32);
    pub const INDEXED_SEQUENCES_NV: Self = Self(2u32);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(4u32);
}

impl From<IndirectCommandsLayoutUsageFlagBitsNV> for IndirectCommandsLayoutUsageFlagsNV {
    fn from(value: IndirectCommandsLayoutUsageFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for IndirectCommandsLayoutUsageFlagBitsNV {
    type Output = IndirectCommandsLayoutUsageFlagsNV;
    fn bitor(self, rhs: Self) -> IndirectCommandsLayoutUsageFlagsNV {
        IndirectCommandsLayoutUsageFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsLayoutUsageFlagsNV>
    for IndirectCommandsLayoutUsageFlagBitsNV
{
    type Output = IndirectCommandsLayoutUsageFlagsNV;
    fn bitor(self, rhs: IndirectCommandsLayoutUsageFlagsNV) -> IndirectCommandsLayoutUsageFlagsNV {
        IndirectCommandsLayoutUsageFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectCommandsLayoutUsageFlagBitsNV>
    for IndirectCommandsLayoutUsageFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: IndirectCommandsLayoutUsageFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<IndirectCommandsLayoutUsageFlagBitsNV>
    for IndirectCommandsLayoutUsageFlagsNV
{
    fn bitor_assign(&mut self, rhs: IndirectCommandsLayoutUsageFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectStateFlagBitsNV(pub u32);

impl IndirectStateFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_FRONTFACE_NV: Self = Self(1u32);
}

impl From<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn from(value: IndirectStateFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for IndirectStateFlagBitsNV {
    type Output = IndirectStateFlagsNV;
    fn bitor(self, rhs: Self) -> IndirectStateFlagsNV {
        IndirectStateFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectStateFlagsNV> for IndirectStateFlagBitsNV {
    type Output = IndirectStateFlagsNV;
    fn bitor(self, rhs: IndirectStateFlagsNV) -> IndirectStateFlagsNV {
        IndirectStateFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: IndirectStateFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<IndirectStateFlagBitsNV> for IndirectStateFlagsNV {
    fn bitor_assign(&mut self, rhs: IndirectStateFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InstanceCreateFlagBits(pub u32);

impl InstanceCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(1u32);
}

impl From<InstanceCreateFlagBits> for InstanceCreateFlags {
    fn from(value: InstanceCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for InstanceCreateFlagBits {
    type Output = InstanceCreateFlags;
    fn bitor(self, rhs: Self) -> InstanceCreateFlags {
        InstanceCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<InstanceCreateFlags> for InstanceCreateFlagBits {
    type Output = InstanceCreateFlags;
    fn bitor(self, rhs: InstanceCreateFlags) -> InstanceCreateFlags {
        InstanceCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<InstanceCreateFlagBits> for InstanceCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: InstanceCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<InstanceCreateFlagBits> for InstanceCreateFlags {
    fn bitor_assign(&mut self, rhs: InstanceCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryAllocateFlagBits(pub u32);

impl MemoryAllocateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_MASK: Self = Self(1u32);
    pub const DEVICE_ADDRESS: Self = Self(2u32);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4u32);
    pub const ZERO_INITIALIZE_EXT: Self = Self(8u32);
}

impl From<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn from(value: MemoryAllocateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryAllocateFlagBits {
    type Output = MemoryAllocateFlags;
    fn bitor(self, rhs: Self) -> MemoryAllocateFlags {
        MemoryAllocateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryAllocateFlags> for MemoryAllocateFlagBits {
    type Output = MemoryAllocateFlags;
    fn bitor(self, rhs: MemoryAllocateFlags) -> MemoryAllocateFlags {
        MemoryAllocateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    type Output = Self;
    fn bitor(self, rhs: MemoryAllocateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryAllocateFlagBits> for MemoryAllocateFlags {
    fn bitor_assign(&mut self, rhs: MemoryAllocateFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type MemoryAllocateFlagBitsKHR = MemoryAllocateFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryDecompressionMethodFlagBitsEXT(pub u64);

impl MemoryDecompressionMethodFlagBitsEXT {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const GDEFLATE_1_0_EXT: Self = Self(1u64);
}

impl From<MemoryDecompressionMethodFlagBitsEXT> for MemoryDecompressionMethodFlagsEXT {
    fn from(value: MemoryDecompressionMethodFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryDecompressionMethodFlagBitsEXT {
    type Output = MemoryDecompressionMethodFlagsEXT;
    fn bitor(self, rhs: Self) -> MemoryDecompressionMethodFlagsEXT {
        MemoryDecompressionMethodFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryDecompressionMethodFlagsEXT> for MemoryDecompressionMethodFlagBitsEXT {
    type Output = MemoryDecompressionMethodFlagsEXT;
    fn bitor(self, rhs: MemoryDecompressionMethodFlagsEXT) -> MemoryDecompressionMethodFlagsEXT {
        MemoryDecompressionMethodFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryDecompressionMethodFlagBitsEXT> for MemoryDecompressionMethodFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: MemoryDecompressionMethodFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryDecompressionMethodFlagBitsEXT>
    for MemoryDecompressionMethodFlagsEXT
{
    fn bitor_assign(&mut self, rhs: MemoryDecompressionMethodFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

pub type MemoryDecompressionMethodFlagBitsNV = MemoryDecompressionMethodFlagBitsEXT;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryHeapFlagBits(pub u32);

impl MemoryHeapFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL: Self = Self(1u32);
    pub const MULTI_INSTANCE: Self = Self(2u32);
    pub const TILE_MEMORY_QCOM: Self = Self(8u32);
}

impl From<MemoryHeapFlagBits> for MemoryHeapFlags {
    fn from(value: MemoryHeapFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryHeapFlagBits {
    type Output = MemoryHeapFlags;
    fn bitor(self, rhs: Self) -> MemoryHeapFlags {
        MemoryHeapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryHeapFlags> for MemoryHeapFlagBits {
    type Output = MemoryHeapFlags;
    fn bitor(self, rhs: MemoryHeapFlags) -> MemoryHeapFlags {
        MemoryHeapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryHeapFlagBits> for MemoryHeapFlags {
    type Output = Self;
    fn bitor(self, rhs: MemoryHeapFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryHeapFlagBits> for MemoryHeapFlags {
    fn bitor_assign(&mut self, rhs: MemoryHeapFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryMapFlagBits(pub u32);

impl MemoryMapFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PLACED_EXT: Self = Self(1u32);
}

impl From<MemoryMapFlagBits> for MemoryMapFlags {
    fn from(value: MemoryMapFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryMapFlagBits {
    type Output = MemoryMapFlags;
    fn bitor(self, rhs: Self) -> MemoryMapFlags {
        MemoryMapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryMapFlags> for MemoryMapFlagBits {
    type Output = MemoryMapFlags;
    fn bitor(self, rhs: MemoryMapFlags) -> MemoryMapFlags {
        MemoryMapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryMapFlagBits> for MemoryMapFlags {
    type Output = Self;
    fn bitor(self, rhs: MemoryMapFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryMapFlagBits> for MemoryMapFlags {
    fn bitor_assign(&mut self, rhs: MemoryMapFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryPropertyFlagBits(pub u32);

impl MemoryPropertyFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL: Self = Self(1u32);
    pub const HOST_VISIBLE: Self = Self(2u32);
    pub const HOST_COHERENT: Self = Self(4u32);
    pub const HOST_CACHED: Self = Self(8u32);
    pub const LAZILY_ALLOCATED: Self = Self(16u32);
    pub const PROTECTED: Self = Self(32u32);
    pub const DEVICE_COHERENT_AMD: Self = Self(64u32);
    pub const DEVICE_UNCACHED_AMD: Self = Self(128u32);
    pub const RDMA_CAPABLE_NV: Self = Self(256u32);
}

impl From<MemoryPropertyFlagBits> for MemoryPropertyFlags {
    fn from(value: MemoryPropertyFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryPropertyFlagBits {
    type Output = MemoryPropertyFlags;
    fn bitor(self, rhs: Self) -> MemoryPropertyFlags {
        MemoryPropertyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryPropertyFlags> for MemoryPropertyFlagBits {
    type Output = MemoryPropertyFlags;
    fn bitor(self, rhs: MemoryPropertyFlags) -> MemoryPropertyFlags {
        MemoryPropertyFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryPropertyFlagBits> for MemoryPropertyFlags {
    type Output = Self;
    fn bitor(self, rhs: MemoryPropertyFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryPropertyFlagBits> for MemoryPropertyFlags {
    fn bitor_assign(&mut self, rhs: MemoryPropertyFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryUnmapFlagBits(pub u32);

impl MemoryUnmapFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESERVE_EXT: Self = Self(1u32);
}

impl From<MemoryUnmapFlagBits> for MemoryUnmapFlags {
    fn from(value: MemoryUnmapFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MemoryUnmapFlagBits {
    type Output = MemoryUnmapFlags;
    fn bitor(self, rhs: Self) -> MemoryUnmapFlags {
        MemoryUnmapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryUnmapFlags> for MemoryUnmapFlagBits {
    type Output = MemoryUnmapFlags;
    fn bitor(self, rhs: MemoryUnmapFlags) -> MemoryUnmapFlags {
        MemoryUnmapFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MemoryUnmapFlagBits> for MemoryUnmapFlags {
    type Output = Self;
    fn bitor(self, rhs: MemoryUnmapFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MemoryUnmapFlagBits> for MemoryUnmapFlags {
    fn bitor_assign(&mut self, rhs: MemoryUnmapFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type MemoryUnmapFlagBitsKHR = MemoryUnmapFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MicromapCreateFlagBitsEXT(pub u32);

impl MicromapCreateFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(1u32);
}

impl From<MicromapCreateFlagBitsEXT> for MicromapCreateFlagsEXT {
    fn from(value: MicromapCreateFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for MicromapCreateFlagBitsEXT {
    type Output = MicromapCreateFlagsEXT;
    fn bitor(self, rhs: Self) -> MicromapCreateFlagsEXT {
        MicromapCreateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MicromapCreateFlagsEXT> for MicromapCreateFlagBitsEXT {
    type Output = MicromapCreateFlagsEXT;
    fn bitor(self, rhs: MicromapCreateFlagsEXT) -> MicromapCreateFlagsEXT {
        MicromapCreateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<MicromapCreateFlagBitsEXT> for MicromapCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: MicromapCreateFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<MicromapCreateFlagBitsEXT> for MicromapCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: MicromapCreateFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowExecuteFlagBitsNV(pub u32);

impl OpticalFlowExecuteFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(1u32);
}

impl From<OpticalFlowExecuteFlagBitsNV> for OpticalFlowExecuteFlagsNV {
    fn from(value: OpticalFlowExecuteFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for OpticalFlowExecuteFlagBitsNV {
    type Output = OpticalFlowExecuteFlagsNV;
    fn bitor(self, rhs: Self) -> OpticalFlowExecuteFlagsNV {
        OpticalFlowExecuteFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowExecuteFlagsNV> for OpticalFlowExecuteFlagBitsNV {
    type Output = OpticalFlowExecuteFlagsNV;
    fn bitor(self, rhs: OpticalFlowExecuteFlagsNV) -> OpticalFlowExecuteFlagsNV {
        OpticalFlowExecuteFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowExecuteFlagBitsNV> for OpticalFlowExecuteFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: OpticalFlowExecuteFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<OpticalFlowExecuteFlagBitsNV> for OpticalFlowExecuteFlagsNV {
    fn bitor_assign(&mut self, rhs: OpticalFlowExecuteFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowGridSizeFlagBitsNV(pub u32);

impl OpticalFlowGridSizeFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_NV: Self = Self(0u32);
    pub const _1X1_NV: Self = Self(1u32);
    pub const _2X2_NV: Self = Self(2u32);
    pub const _4X4_NV: Self = Self(4u32);
    pub const _8X8_NV: Self = Self(8u32);
}

impl From<OpticalFlowGridSizeFlagBitsNV> for OpticalFlowGridSizeFlagsNV {
    fn from(value: OpticalFlowGridSizeFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for OpticalFlowGridSizeFlagBitsNV {
    type Output = OpticalFlowGridSizeFlagsNV;
    fn bitor(self, rhs: Self) -> OpticalFlowGridSizeFlagsNV {
        OpticalFlowGridSizeFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowGridSizeFlagsNV> for OpticalFlowGridSizeFlagBitsNV {
    type Output = OpticalFlowGridSizeFlagsNV;
    fn bitor(self, rhs: OpticalFlowGridSizeFlagsNV) -> OpticalFlowGridSizeFlagsNV {
        OpticalFlowGridSizeFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowGridSizeFlagBitsNV> for OpticalFlowGridSizeFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: OpticalFlowGridSizeFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<OpticalFlowGridSizeFlagBitsNV> for OpticalFlowGridSizeFlagsNV {
    fn bitor_assign(&mut self, rhs: OpticalFlowGridSizeFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowSessionCreateFlagBitsNV(pub u32);

impl OpticalFlowSessionCreateFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_HINT_NV: Self = Self(1u32);
    pub const ENABLE_COST_NV: Self = Self(2u32);
    pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(4u32);
    pub const ALLOW_REGIONS_NV: Self = Self(8u32);
    pub const BOTH_DIRECTIONS_NV: Self = Self(16u32);
}

impl From<OpticalFlowSessionCreateFlagBitsNV> for OpticalFlowSessionCreateFlagsNV {
    fn from(value: OpticalFlowSessionCreateFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for OpticalFlowSessionCreateFlagBitsNV {
    type Output = OpticalFlowSessionCreateFlagsNV;
    fn bitor(self, rhs: Self) -> OpticalFlowSessionCreateFlagsNV {
        OpticalFlowSessionCreateFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowSessionCreateFlagsNV> for OpticalFlowSessionCreateFlagBitsNV {
    type Output = OpticalFlowSessionCreateFlagsNV;
    fn bitor(self, rhs: OpticalFlowSessionCreateFlagsNV) -> OpticalFlowSessionCreateFlagsNV {
        OpticalFlowSessionCreateFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowSessionCreateFlagBitsNV> for OpticalFlowSessionCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: OpticalFlowSessionCreateFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<OpticalFlowSessionCreateFlagBitsNV>
    for OpticalFlowSessionCreateFlagsNV
{
    fn bitor_assign(&mut self, rhs: OpticalFlowSessionCreateFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowUsageFlagBitsNV(pub u32);

impl OpticalFlowUsageFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_NV: Self = Self(0u32);
    pub const INPUT_NV: Self = Self(1u32);
    pub const OUTPUT_NV: Self = Self(2u32);
    pub const HINT_NV: Self = Self(4u32);
    pub const COST_NV: Self = Self(8u32);
    pub const GLOBAL_FLOW_NV: Self = Self(16u32);
}

impl From<OpticalFlowUsageFlagBitsNV> for OpticalFlowUsageFlagsNV {
    fn from(value: OpticalFlowUsageFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for OpticalFlowUsageFlagBitsNV {
    type Output = OpticalFlowUsageFlagsNV;
    fn bitor(self, rhs: Self) -> OpticalFlowUsageFlagsNV {
        OpticalFlowUsageFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowUsageFlagsNV> for OpticalFlowUsageFlagBitsNV {
    type Output = OpticalFlowUsageFlagsNV;
    fn bitor(self, rhs: OpticalFlowUsageFlagsNV) -> OpticalFlowUsageFlagsNV {
        OpticalFlowUsageFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<OpticalFlowUsageFlagBitsNV> for OpticalFlowUsageFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: OpticalFlowUsageFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<OpticalFlowUsageFlagBitsNV> for OpticalFlowUsageFlagsNV {
    fn bitor_assign(&mut self, rhs: OpticalFlowUsageFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PartitionedAccelerationStructureInstanceFlagBitsNV(pub u32);

impl PartitionedAccelerationStructureInstanceFlagBitsNV {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(1u32);
    pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(2u32);
    pub const FLAG_FORCE_OPAQUE_NV: Self = Self(4u32);
    pub const FLAG_FORCE_NO_OPAQUE_NV: Self = Self(8u32);
    pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(16u32);
}

impl From<PartitionedAccelerationStructureInstanceFlagBitsNV>
    for PartitionedAccelerationStructureInstanceFlagsNV
{
    fn from(value: PartitionedAccelerationStructureInstanceFlagBitsNV) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PartitionedAccelerationStructureInstanceFlagBitsNV {
    type Output = PartitionedAccelerationStructureInstanceFlagsNV;
    fn bitor(self, rhs: Self) -> PartitionedAccelerationStructureInstanceFlagsNV {
        PartitionedAccelerationStructureInstanceFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PartitionedAccelerationStructureInstanceFlagsNV>
    for PartitionedAccelerationStructureInstanceFlagBitsNV
{
    type Output = PartitionedAccelerationStructureInstanceFlagsNV;
    fn bitor(
        self,
        rhs: PartitionedAccelerationStructureInstanceFlagsNV,
    ) -> PartitionedAccelerationStructureInstanceFlagsNV {
        PartitionedAccelerationStructureInstanceFlagsNV(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PartitionedAccelerationStructureInstanceFlagBitsNV>
    for PartitionedAccelerationStructureInstanceFlagsNV
{
    type Output = Self;
    fn bitor(self, rhs: PartitionedAccelerationStructureInstanceFlagBitsNV) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PartitionedAccelerationStructureInstanceFlagBitsNV>
    for PartitionedAccelerationStructureInstanceFlagsNV
{
    fn bitor_assign(&mut self, rhs: PartitionedAccelerationStructureInstanceFlagBitsNV) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PastPresentationTimingFlagBitsEXT(pub u32);

impl PastPresentationTimingFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(1u32);
    pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(2u32);
}

impl From<PastPresentationTimingFlagBitsEXT> for PastPresentationTimingFlagsEXT {
    fn from(value: PastPresentationTimingFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PastPresentationTimingFlagBitsEXT {
    type Output = PastPresentationTimingFlagsEXT;
    fn bitor(self, rhs: Self) -> PastPresentationTimingFlagsEXT {
        PastPresentationTimingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PastPresentationTimingFlagsEXT> for PastPresentationTimingFlagBitsEXT {
    type Output = PastPresentationTimingFlagsEXT;
    fn bitor(self, rhs: PastPresentationTimingFlagsEXT) -> PastPresentationTimingFlagsEXT {
        PastPresentationTimingFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PastPresentationTimingFlagBitsEXT> for PastPresentationTimingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: PastPresentationTimingFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PastPresentationTimingFlagBitsEXT> for PastPresentationTimingFlagsEXT {
    fn bitor_assign(&mut self, rhs: PastPresentationTimingFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PeerMemoryFeatureFlagBits(pub u32);

impl PeerMemoryFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const COPY_SRC: Self = Self(1u32);
    pub const COPY_DST: Self = Self(2u32);
    pub const GENERIC_SRC: Self = Self(4u32);
    pub const GENERIC_DST: Self = Self(8u32);
}

impl From<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn from(value: PeerMemoryFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PeerMemoryFeatureFlagBits {
    type Output = PeerMemoryFeatureFlags;
    fn bitor(self, rhs: Self) -> PeerMemoryFeatureFlags {
        PeerMemoryFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PeerMemoryFeatureFlags> for PeerMemoryFeatureFlagBits {
    type Output = PeerMemoryFeatureFlags;
    fn bitor(self, rhs: PeerMemoryFeatureFlags) -> PeerMemoryFeatureFlags {
        PeerMemoryFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: PeerMemoryFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PeerMemoryFeatureFlagBits> for PeerMemoryFeatureFlags {
    fn bitor_assign(&mut self, rhs: PeerMemoryFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type PeerMemoryFeatureFlagBitsKHR = PeerMemoryFeatureFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterDescriptionFlagBitsKHR(pub u32);

impl PerformanceCounterDescriptionFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1u32);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(2u32);
}

impl From<PerformanceCounterDescriptionFlagBitsKHR> for PerformanceCounterDescriptionFlagsKHR {
    fn from(value: PerformanceCounterDescriptionFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PerformanceCounterDescriptionFlagBitsKHR {
    type Output = PerformanceCounterDescriptionFlagsKHR;
    fn bitor(self, rhs: Self) -> PerformanceCounterDescriptionFlagsKHR {
        PerformanceCounterDescriptionFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PerformanceCounterDescriptionFlagsKHR>
    for PerformanceCounterDescriptionFlagBitsKHR
{
    type Output = PerformanceCounterDescriptionFlagsKHR;
    fn bitor(
        self,
        rhs: PerformanceCounterDescriptionFlagsKHR,
    ) -> PerformanceCounterDescriptionFlagsKHR {
        PerformanceCounterDescriptionFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PerformanceCounterDescriptionFlagBitsKHR>
    for PerformanceCounterDescriptionFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: PerformanceCounterDescriptionFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PerformanceCounterDescriptionFlagBitsKHR>
    for PerformanceCounterDescriptionFlagsKHR
{
    fn bitor_assign(&mut self, rhs: PerformanceCounterDescriptionFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceSchedulingControlsFlagBitsARM(pub u64);

impl PhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const SHADER_CORE_COUNT_ARM: Self = Self(1u64);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(2u64);
}

impl From<PhysicalDeviceSchedulingControlsFlagBitsARM>
    for PhysicalDeviceSchedulingControlsFlagsARM
{
    fn from(value: PhysicalDeviceSchedulingControlsFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PhysicalDeviceSchedulingControlsFlagBitsARM {
    type Output = PhysicalDeviceSchedulingControlsFlagsARM;
    fn bitor(self, rhs: Self) -> PhysicalDeviceSchedulingControlsFlagsARM {
        PhysicalDeviceSchedulingControlsFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PhysicalDeviceSchedulingControlsFlagsARM>
    for PhysicalDeviceSchedulingControlsFlagBitsARM
{
    type Output = PhysicalDeviceSchedulingControlsFlagsARM;
    fn bitor(
        self,
        rhs: PhysicalDeviceSchedulingControlsFlagsARM,
    ) -> PhysicalDeviceSchedulingControlsFlagsARM {
        PhysicalDeviceSchedulingControlsFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PhysicalDeviceSchedulingControlsFlagBitsARM>
    for PhysicalDeviceSchedulingControlsFlagsARM
{
    type Output = Self;
    fn bitor(self, rhs: PhysicalDeviceSchedulingControlsFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PhysicalDeviceSchedulingControlsFlagBitsARM>
    for PhysicalDeviceSchedulingControlsFlagsARM
{
    fn bitor_assign(&mut self, rhs: PhysicalDeviceSchedulingControlsFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCacheCreateFlagBits(pub u32);

impl PipelineCacheCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1u32);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self = Self(8u32);
}

impl From<PipelineCacheCreateFlagBits> for PipelineCacheCreateFlags {
    fn from(value: PipelineCacheCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineCacheCreateFlagBits {
    type Output = PipelineCacheCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineCacheCreateFlags {
        PipelineCacheCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCacheCreateFlags> for PipelineCacheCreateFlagBits {
    type Output = PipelineCacheCreateFlags;
    fn bitor(self, rhs: PipelineCacheCreateFlags) -> PipelineCacheCreateFlags {
        PipelineCacheCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCacheCreateFlagBits> for PipelineCacheCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineCacheCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineCacheCreateFlagBits> for PipelineCacheCreateFlags {
    fn bitor_assign(&mut self, rhs: PipelineCacheCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineColorBlendStateCreateFlagBits(pub u32);

impl PipelineColorBlendStateCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(1u32);
}

impl From<PipelineColorBlendStateCreateFlagBits> for PipelineColorBlendStateCreateFlags {
    fn from(value: PipelineColorBlendStateCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineColorBlendStateCreateFlagBits {
    type Output = PipelineColorBlendStateCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineColorBlendStateCreateFlags {
        PipelineColorBlendStateCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineColorBlendStateCreateFlags>
    for PipelineColorBlendStateCreateFlagBits
{
    type Output = PipelineColorBlendStateCreateFlags;
    fn bitor(self, rhs: PipelineColorBlendStateCreateFlags) -> PipelineColorBlendStateCreateFlags {
        PipelineColorBlendStateCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineColorBlendStateCreateFlagBits>
    for PipelineColorBlendStateCreateFlags
{
    type Output = Self;
    fn bitor(self, rhs: PipelineColorBlendStateCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineColorBlendStateCreateFlagBits>
    for PipelineColorBlendStateCreateFlags
{
    fn bitor_assign(&mut self, rhs: PipelineColorBlendStateCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCompilerControlFlagBitsAMD(pub u32);

impl PipelineCompilerControlFlagBitsAMD {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    fn from(value: PipelineCompilerControlFlagBitsAMD) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineCompilerControlFlagBitsAMD {
    type Output = PipelineCompilerControlFlagsAMD;
    fn bitor(self, rhs: Self) -> PipelineCompilerControlFlagsAMD {
        PipelineCompilerControlFlagsAMD(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCompilerControlFlagsAMD> for PipelineCompilerControlFlagBitsAMD {
    type Output = PipelineCompilerControlFlagsAMD;
    fn bitor(self, rhs: PipelineCompilerControlFlagsAMD) -> PipelineCompilerControlFlagsAMD {
        PipelineCompilerControlFlagsAMD(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCompilerControlFlagBitsAMD> for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    fn bitor(self, rhs: PipelineCompilerControlFlagBitsAMD) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineCompilerControlFlagBitsAMD>
    for PipelineCompilerControlFlagsAMD
{
    fn bitor_assign(&mut self, rhs: PipelineCompilerControlFlagBitsAMD) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreateFlagBits(pub u32);

impl PipelineCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_OPTIMIZATION: Self = Self(1u32);
    pub const ALLOW_DERIVATIVES: Self = Self(2u32);
    pub const DERIVATIVE: Self = Self(4u32);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(8u32);
    pub const DISPATCH_BASE: Self = Self(16u32);
    pub const DEFER_COMPILE_NV: Self = Self(32u32);
    pub const CAPTURE_STATISTICS_KHR: Self = Self(64u32);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(128u32);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(256u32);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(512u32);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1024u32);
    pub const LIBRARY_KHR: Self = Self(2048u32);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(4096u32);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(8192u32);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(16384u32);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(32768u32);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(65536u32);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(131072u32);
    pub const INDIRECT_BINDABLE_NV: Self = Self(262144u32);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(524288u32);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1048576u32);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(2097152u32);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(4194304u32);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(8388608u32);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(16777216u32);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(33554432u32);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(67108864u32);
    pub const NO_PROTECTED_ACCESS: Self = Self(134217728u32);
    #[cfg(feature = "beta")]
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(268435456u32);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(536870912u32);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(1073741824u32);
}

impl From<PipelineCreateFlagBits> for PipelineCreateFlags {
    fn from(value: PipelineCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineCreateFlagBits {
    type Output = PipelineCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineCreateFlags {
        PipelineCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreateFlags> for PipelineCreateFlagBits {
    type Output = PipelineCreateFlags;
    fn bitor(self, rhs: PipelineCreateFlags) -> PipelineCreateFlags {
        PipelineCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreateFlagBits> for PipelineCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineCreateFlagBits> for PipelineCreateFlags {
    fn bitor_assign(&mut self, rhs: PipelineCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreateFlagBits2(pub u64);

impl PipelineCreateFlagBits2 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PIPELINE_CREATE_2_DISABLE_OPTIMIZATION: Self = Self(1u64);
    pub const PIPELINE_CREATE_2_ALLOW_DERIVATIVES: Self = Self(2u64);
    pub const PIPELINE_CREATE_2_DERIVATIVE: Self = Self(4u64);
    pub const PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(8u64);
    pub const PIPELINE_CREATE_2_DISPATCH_BASE: Self = Self(16u64);
    pub const PIPELINE_CREATE_2_DEFER_COMPILE_NV: Self = Self(32u64);
    pub const PIPELINE_CREATE_2_CAPTURE_STATISTICS_KHR: Self = Self(64u64);
    pub const PIPELINE_CREATE_2_CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(128u64);
    pub const PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(256u64);
    pub const PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE: Self = Self(512u64);
    pub const PIPELINE_CREATE_2_LINK_TIME_OPTIMIZATION_EXT: Self = Self(1024u64);
    pub const PIPELINE_CREATE_2_LIBRARY_KHR: Self = Self(2048u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(4096u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SKIP_AABBS_KHR: Self = Self(8192u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(16384u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(32768u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(65536u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self =
        Self(131072u64);
    pub const PIPELINE_CREATE_2_INDIRECT_BINDABLE_NV: Self = Self(262144u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
        Self(524288u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1048576u64);
    pub const PIPELINE_CREATE_2_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(2097152u64);
    pub const PIPELINE_CREATE_2_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self(4194304u64);
    pub const PIPELINE_CREATE_2_RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(8388608u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(16777216u64);
    pub const PIPELINE_CREATE_2_COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(33554432u64);
    pub const PIPELINE_CREATE_2_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
        Self(67108864u64);
    pub const PIPELINE_CREATE_2_NO_PROTECTED_ACCESS: Self = Self(134217728u64);
    #[cfg(feature = "beta")]
    pub const PIPELINE_CREATE_2_RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(268435456u64);
    pub const PIPELINE_CREATE_2_DESCRIPTOR_BUFFER_EXT: Self = Self(536870912u64);
    pub const PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY: Self = Self(1073741824u64);
    pub const PIPELINE_CREATE_2_CAPTURE_DATA_KHR: Self = Self(2147483648u64);
    #[cfg(feature = "beta")]
    pub const PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX: Self = Self(4294967296u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self =
        Self(8589934592u64);
    pub const PIPELINE_CREATE_2_ENABLE_LEGACY_DITHERING_EXT: Self = Self(17179869184u64);
    pub const PIPELINE_CREATE_2_DESCRIPTOR_HEAP_EXT: Self = Self(68719476736u64);
    pub const PIPELINE_CREATE_2_DISALLOW_OPACITY_MICROMAP_ARM: Self = Self(137438953472u64);
    pub const PIPELINE_CREATE_2_INDIRECT_BINDABLE_EXT: Self = Self(274877906944u64);
    pub const PIPELINE_CREATE_2_INSTRUMENT_SHADERS_ARM: Self = Self(549755813888u64);
    pub const PIPELINE_CREATE_2_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(1099511627776u64);
    pub const PIPELINE_CREATE_2_64_BIT_INDEXING_EXT: Self = Self(8796093022208u64);
}

impl From<PipelineCreateFlagBits2> for PipelineCreateFlags2 {
    fn from(value: PipelineCreateFlagBits2) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineCreateFlagBits2 {
    type Output = PipelineCreateFlags2;
    fn bitor(self, rhs: Self) -> PipelineCreateFlags2 {
        PipelineCreateFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreateFlags2> for PipelineCreateFlagBits2 {
    type Output = PipelineCreateFlags2;
    fn bitor(self, rhs: PipelineCreateFlags2) -> PipelineCreateFlags2 {
        PipelineCreateFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreateFlagBits2> for PipelineCreateFlags2 {
    type Output = Self;
    fn bitor(self, rhs: PipelineCreateFlagBits2) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineCreateFlagBits2> for PipelineCreateFlags2 {
    fn bitor_assign(&mut self, rhs: PipelineCreateFlagBits2) {
        self.0 |= rhs.0;
    }
}

pub type PipelineCreateFlagBits2KHR = PipelineCreateFlagBits2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreationFeedbackFlagBits(pub u32);

impl PipelineCreationFeedbackFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VALID: Self = Self(1u32);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(2u32);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(4u32);
}

impl From<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    fn from(value: PipelineCreationFeedbackFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineCreationFeedbackFlagBits {
    type Output = PipelineCreationFeedbackFlags;
    fn bitor(self, rhs: Self) -> PipelineCreationFeedbackFlags {
        PipelineCreationFeedbackFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreationFeedbackFlags> for PipelineCreationFeedbackFlagBits {
    type Output = PipelineCreationFeedbackFlags;
    fn bitor(self, rhs: PipelineCreationFeedbackFlags) -> PipelineCreationFeedbackFlags {
        PipelineCreationFeedbackFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineCreationFeedbackFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineCreationFeedbackFlagBits> for PipelineCreationFeedbackFlags {
    fn bitor_assign(&mut self, rhs: PipelineCreationFeedbackFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type PipelineCreationFeedbackFlagBitsEXT = PipelineCreationFeedbackFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineDepthStencilStateCreateFlagBits(pub u32);

impl PipelineDepthStencilStateCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(2u32);
}

impl From<PipelineDepthStencilStateCreateFlagBits> for PipelineDepthStencilStateCreateFlags {
    fn from(value: PipelineDepthStencilStateCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineDepthStencilStateCreateFlagBits {
    type Output = PipelineDepthStencilStateCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineDepthStencilStateCreateFlags {
        PipelineDepthStencilStateCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineDepthStencilStateCreateFlags>
    for PipelineDepthStencilStateCreateFlagBits
{
    type Output = PipelineDepthStencilStateCreateFlags;
    fn bitor(
        self,
        rhs: PipelineDepthStencilStateCreateFlags,
    ) -> PipelineDepthStencilStateCreateFlags {
        PipelineDepthStencilStateCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineDepthStencilStateCreateFlagBits>
    for PipelineDepthStencilStateCreateFlags
{
    type Output = Self;
    fn bitor(self, rhs: PipelineDepthStencilStateCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineDepthStencilStateCreateFlagBits>
    for PipelineDepthStencilStateCreateFlags
{
    fn bitor_assign(&mut self, rhs: PipelineDepthStencilStateCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineLayoutCreateFlagBits(pub u32);

impl PipelineLayoutCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INDEPENDENT_SETS_EXT: Self = Self(2u32);
    pub const NO_TASK_SHADER_KHR: Self = Self(4u32);
}

impl From<PipelineLayoutCreateFlagBits> for PipelineLayoutCreateFlags {
    fn from(value: PipelineLayoutCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineLayoutCreateFlagBits {
    type Output = PipelineLayoutCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineLayoutCreateFlags {
        PipelineLayoutCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineLayoutCreateFlags> for PipelineLayoutCreateFlagBits {
    type Output = PipelineLayoutCreateFlags;
    fn bitor(self, rhs: PipelineLayoutCreateFlags) -> PipelineLayoutCreateFlags {
        PipelineLayoutCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineLayoutCreateFlagBits> for PipelineLayoutCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineLayoutCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineLayoutCreateFlagBits> for PipelineLayoutCreateFlags {
    fn bitor_assign(&mut self, rhs: PipelineLayoutCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineShaderStageCreateFlagBits(pub u32);

impl PipelineShaderStageCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(1u32);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(2u32);
}

impl From<PipelineShaderStageCreateFlagBits> for PipelineShaderStageCreateFlags {
    fn from(value: PipelineShaderStageCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineShaderStageCreateFlagBits {
    type Output = PipelineShaderStageCreateFlags;
    fn bitor(self, rhs: Self) -> PipelineShaderStageCreateFlags {
        PipelineShaderStageCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineShaderStageCreateFlags> for PipelineShaderStageCreateFlagBits {
    type Output = PipelineShaderStageCreateFlags;
    fn bitor(self, rhs: PipelineShaderStageCreateFlags) -> PipelineShaderStageCreateFlags {
        PipelineShaderStageCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineShaderStageCreateFlagBits> for PipelineShaderStageCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineShaderStageCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineShaderStageCreateFlagBits> for PipelineShaderStageCreateFlags {
    fn bitor_assign(&mut self, rhs: PipelineShaderStageCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineStageFlagBits(pub u32);

impl PipelineStageFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const TOP_OF_PIPE: Self = Self(1u32);
    pub const DRAW_INDIRECT: Self = Self(2u32);
    pub const VERTEX_INPUT: Self = Self(4u32);
    pub const VERTEX_SHADER: Self = Self(8u32);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(16u32);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(32u32);
    pub const GEOMETRY_SHADER: Self = Self(64u32);
    pub const FRAGMENT_SHADER: Self = Self(128u32);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(256u32);
    pub const LATE_FRAGMENT_TESTS: Self = Self(512u32);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1024u32);
    pub const COMPUTE_SHADER: Self = Self(2048u32);
    pub const TRANSFER: Self = Self(4096u32);
    pub const BOTTOM_OF_PIPE: Self = Self(8192u32);
    pub const HOST: Self = Self(16384u32);
    pub const ALL_GRAPHICS: Self = Self(32768u32);
    pub const ALL_COMMANDS: Self = Self(65536u32);
    pub const COMMAND_PREPROCESS_EXT: Self = Self(131072u32);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(262144u32);
    pub const TASK_SHADER_EXT: Self = Self(524288u32);
    pub const MESH_SHADER_EXT: Self = Self(1048576u32);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(2097152u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304u32);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608u32);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(16777216u32);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432u32);
}

impl From<PipelineStageFlagBits> for PipelineStageFlags {
    fn from(value: PipelineStageFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineStageFlagBits {
    type Output = PipelineStageFlags;
    fn bitor(self, rhs: Self) -> PipelineStageFlags {
        PipelineStageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineStageFlags> for PipelineStageFlagBits {
    type Output = PipelineStageFlags;
    fn bitor(self, rhs: PipelineStageFlags) -> PipelineStageFlags {
        PipelineStageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineStageFlagBits> for PipelineStageFlags {
    type Output = Self;
    fn bitor(self, rhs: PipelineStageFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineStageFlagBits> for PipelineStageFlags {
    fn bitor_assign(&mut self, rhs: PipelineStageFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineStageFlagBits2(pub u64);

impl PipelineStageFlagBits2 {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PIPELINE_STAGE_2_NONE: Self = Self(0u64);
    pub const PIPELINE_STAGE_2_TOP_OF_PIPE: Self = Self(1u64);
    pub const PIPELINE_STAGE_2_DRAW_INDIRECT: Self = Self(2u64);
    pub const PIPELINE_STAGE_2_VERTEX_INPUT: Self = Self(4u64);
    pub const PIPELINE_STAGE_2_VERTEX_SHADER: Self = Self(8u64);
    pub const PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER: Self = Self(16u64);
    pub const PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER: Self = Self(32u64);
    pub const PIPELINE_STAGE_2_GEOMETRY_SHADER: Self = Self(64u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_SHADER: Self = Self(128u64);
    pub const PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS: Self = Self(256u64);
    pub const PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS: Self = Self(512u64);
    pub const PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT: Self = Self(1024u64);
    pub const PIPELINE_STAGE_2_COMPUTE_SHADER: Self = Self(2048u64);
    pub const PIPELINE_STAGE_2_ALL_TRANSFER: Self = Self(4096u64);
    pub const PIPELINE_STAGE_2_BOTTOM_OF_PIPE: Self = Self(8192u64);
    pub const PIPELINE_STAGE_2_HOST: Self = Self(16384u64);
    pub const PIPELINE_STAGE_2_ALL_GRAPHICS: Self = Self(32768u64);
    pub const PIPELINE_STAGE_2_ALL_COMMANDS: Self = Self(65536u64);
    pub const PIPELINE_STAGE_2_COMMAND_PREPROCESS_EXT: Self = Self(131072u64);
    pub const PIPELINE_STAGE_2_CONDITIONAL_RENDERING_EXT: Self = Self(262144u64);
    pub const PIPELINE_STAGE_2_TASK_SHADER_EXT: Self = Self(524288u64);
    pub const PIPELINE_STAGE_2_MESH_SHADER_EXT: Self = Self(1048576u64);
    pub const PIPELINE_STAGE_2_RAY_TRACING_SHADER_KHR: Self = Self(2097152u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608u64);
    pub const PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_EXT: Self = Self(16777216u64);
    pub const PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432u64);
    pub const PIPELINE_STAGE_2_VIDEO_DECODE_KHR: Self = Self(67108864u64);
    pub const PIPELINE_STAGE_2_VIDEO_ENCODE_KHR: Self = Self(134217728u64);
    pub const PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(268435456u64);
    pub const PIPELINE_STAGE_2_OPTICAL_FLOW_NV: Self = Self(536870912u64);
    pub const PIPELINE_STAGE_2_MICROMAP_BUILD_EXT: Self = Self(1073741824u64);
    pub const PIPELINE_STAGE_2_COPY: Self = Self(4294967296u64);
    pub const PIPELINE_STAGE_2_RESOLVE: Self = Self(8589934592u64);
    pub const PIPELINE_STAGE_2_BLIT: Self = Self(17179869184u64);
    pub const PIPELINE_STAGE_2_CLEAR: Self = Self(34359738368u64);
    pub const PIPELINE_STAGE_2_INDEX_INPUT: Self = Self(68719476736u64);
    pub const PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT: Self = Self(137438953472u64);
    pub const PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS: Self = Self(274877906944u64);
    pub const PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI: Self = Self(549755813888u64);
    pub const PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI: Self = Self(1099511627776u64);
    pub const PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI: Self = Self(2199023255552u64);
    pub const PIPELINE_STAGE_2_DATA_GRAPH_ARM: Self = Self(4398046511104u64);
    pub const PIPELINE_STAGE_2_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(17592186044416u64);
    pub const PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_EXT: Self = Self(35184372088832u64);
    pub const PIPELINE_STAGE_2_COPY_INDIRECT_KHR: Self = Self(70368744177664u64);
}

impl From<PipelineStageFlagBits2> for PipelineStageFlags2 {
    fn from(value: PipelineStageFlagBits2) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PipelineStageFlagBits2 {
    type Output = PipelineStageFlags2;
    fn bitor(self, rhs: Self) -> PipelineStageFlags2 {
        PipelineStageFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineStageFlags2> for PipelineStageFlagBits2 {
    type Output = PipelineStageFlags2;
    fn bitor(self, rhs: PipelineStageFlags2) -> PipelineStageFlags2 {
        PipelineStageFlags2(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PipelineStageFlagBits2> for PipelineStageFlags2 {
    type Output = Self;
    fn bitor(self, rhs: PipelineStageFlagBits2) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PipelineStageFlagBits2> for PipelineStageFlags2 {
    fn bitor_assign(&mut self, rhs: PipelineStageFlagBits2) {
        self.0 |= rhs.0;
    }
}

pub type PipelineStageFlagBits2KHR = PipelineStageFlagBits2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentGravityFlagBitsKHR(pub u32);

impl PresentGravityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MIN_KHR: Self = Self(1u32);
    pub const MAX_KHR: Self = Self(2u32);
    pub const CENTERED_KHR: Self = Self(4u32);
}

impl From<PresentGravityFlagBitsKHR> for PresentGravityFlagsKHR {
    fn from(value: PresentGravityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PresentGravityFlagBitsKHR {
    type Output = PresentGravityFlagsKHR;
    fn bitor(self, rhs: Self) -> PresentGravityFlagsKHR {
        PresentGravityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentGravityFlagsKHR> for PresentGravityFlagBitsKHR {
    type Output = PresentGravityFlagsKHR;
    fn bitor(self, rhs: PresentGravityFlagsKHR) -> PresentGravityFlagsKHR {
        PresentGravityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentGravityFlagBitsKHR> for PresentGravityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: PresentGravityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PresentGravityFlagBitsKHR> for PresentGravityFlagsKHR {
    fn bitor_assign(&mut self, rhs: PresentGravityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

pub type PresentGravityFlagBitsEXT = PresentGravityFlagBitsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentScalingFlagBitsKHR(pub u32);

impl PresentScalingFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ONE_TO_ONE_KHR: Self = Self(1u32);
    pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(2u32);
    pub const STRETCH_KHR: Self = Self(4u32);
}

impl From<PresentScalingFlagBitsKHR> for PresentScalingFlagsKHR {
    fn from(value: PresentScalingFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PresentScalingFlagBitsKHR {
    type Output = PresentScalingFlagsKHR;
    fn bitor(self, rhs: Self) -> PresentScalingFlagsKHR {
        PresentScalingFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentScalingFlagsKHR> for PresentScalingFlagBitsKHR {
    type Output = PresentScalingFlagsKHR;
    fn bitor(self, rhs: PresentScalingFlagsKHR) -> PresentScalingFlagsKHR {
        PresentScalingFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentScalingFlagBitsKHR> for PresentScalingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: PresentScalingFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PresentScalingFlagBitsKHR> for PresentScalingFlagsKHR {
    fn bitor_assign(&mut self, rhs: PresentScalingFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

pub type PresentScalingFlagBitsEXT = PresentScalingFlagBitsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentStageFlagBitsEXT(pub u32);

impl PresentStageFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const QUEUE_OPERATIONS_END_EXT: Self = Self(1u32);
    pub const REQUEST_DEQUEUED_EXT: Self = Self(2u32);
    pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(4u32);
    pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(8u32);
}

impl From<PresentStageFlagBitsEXT> for PresentStageFlagsEXT {
    fn from(value: PresentStageFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PresentStageFlagBitsEXT {
    type Output = PresentStageFlagsEXT;
    fn bitor(self, rhs: Self) -> PresentStageFlagsEXT {
        PresentStageFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentStageFlagsEXT> for PresentStageFlagBitsEXT {
    type Output = PresentStageFlagsEXT;
    fn bitor(self, rhs: PresentStageFlagsEXT) -> PresentStageFlagsEXT {
        PresentStageFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentStageFlagBitsEXT> for PresentStageFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: PresentStageFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PresentStageFlagBitsEXT> for PresentStageFlagsEXT {
    fn bitor_assign(&mut self, rhs: PresentStageFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentTimingInfoFlagBitsEXT(pub u32);

impl PresentTimingInfoFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(1u32);
    pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(2u32);
}

impl From<PresentTimingInfoFlagBitsEXT> for PresentTimingInfoFlagsEXT {
    fn from(value: PresentTimingInfoFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for PresentTimingInfoFlagBitsEXT {
    type Output = PresentTimingInfoFlagsEXT;
    fn bitor(self, rhs: Self) -> PresentTimingInfoFlagsEXT {
        PresentTimingInfoFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentTimingInfoFlagsEXT> for PresentTimingInfoFlagBitsEXT {
    type Output = PresentTimingInfoFlagsEXT;
    fn bitor(self, rhs: PresentTimingInfoFlagsEXT) -> PresentTimingInfoFlagsEXT {
        PresentTimingInfoFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<PresentTimingInfoFlagBitsEXT> for PresentTimingInfoFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: PresentTimingInfoFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<PresentTimingInfoFlagBitsEXT> for PresentTimingInfoFlagsEXT {
    fn bitor_assign(&mut self, rhs: PresentTimingInfoFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryControlFlagBits(pub u32);

impl QueryControlFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRECISE: Self = Self(1u32);
}

impl From<QueryControlFlagBits> for QueryControlFlags {
    fn from(value: QueryControlFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for QueryControlFlagBits {
    type Output = QueryControlFlags;
    fn bitor(self, rhs: Self) -> QueryControlFlags {
        QueryControlFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryControlFlags> for QueryControlFlagBits {
    type Output = QueryControlFlags;
    fn bitor(self, rhs: QueryControlFlags) -> QueryControlFlags {
        QueryControlFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryControlFlagBits> for QueryControlFlags {
    type Output = Self;
    fn bitor(self, rhs: QueryControlFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<QueryControlFlagBits> for QueryControlFlags {
    fn bitor_assign(&mut self, rhs: QueryControlFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryPipelineStatisticFlagBits(pub u32);

impl QueryPipelineStatisticFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1u32);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(2u32);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(4u32);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(8u32);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(16u32);
    pub const CLIPPING_INVOCATIONS: Self = Self(32u32);
    pub const CLIPPING_PRIMITIVES: Self = Self(64u32);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(128u32);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(256u32);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(512u32);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1024u32);
    pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(2048u32);
    pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(4096u32);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: Self = Self(8192u32);
}

impl From<QueryPipelineStatisticFlagBits> for QueryPipelineStatisticFlags {
    fn from(value: QueryPipelineStatisticFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for QueryPipelineStatisticFlagBits {
    type Output = QueryPipelineStatisticFlags;
    fn bitor(self, rhs: Self) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryPipelineStatisticFlags> for QueryPipelineStatisticFlagBits {
    type Output = QueryPipelineStatisticFlags;
    fn bitor(self, rhs: QueryPipelineStatisticFlags) -> QueryPipelineStatisticFlags {
        QueryPipelineStatisticFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryPipelineStatisticFlagBits> for QueryPipelineStatisticFlags {
    type Output = Self;
    fn bitor(self, rhs: QueryPipelineStatisticFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<QueryPipelineStatisticFlagBits> for QueryPipelineStatisticFlags {
    fn bitor_assign(&mut self, rhs: QueryPipelineStatisticFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryPoolCreateFlagBits(pub u32);

impl QueryPoolCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESET_KHR: Self = Self(1u32);
}

impl From<QueryPoolCreateFlagBits> for QueryPoolCreateFlags {
    fn from(value: QueryPoolCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for QueryPoolCreateFlagBits {
    type Output = QueryPoolCreateFlags;
    fn bitor(self, rhs: Self) -> QueryPoolCreateFlags {
        QueryPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryPoolCreateFlags> for QueryPoolCreateFlagBits {
    type Output = QueryPoolCreateFlags;
    fn bitor(self, rhs: QueryPoolCreateFlags) -> QueryPoolCreateFlags {
        QueryPoolCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryPoolCreateFlagBits> for QueryPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: QueryPoolCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<QueryPoolCreateFlagBits> for QueryPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: QueryPoolCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryResultFlagBits(pub u32);

impl QueryResultFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _64: Self = Self(1u32);
    pub const WAIT: Self = Self(2u32);
    pub const WITH_AVAILABILITY: Self = Self(4u32);
    pub const PARTIAL: Self = Self(8u32);
    pub const WITH_STATUS_KHR: Self = Self(16u32);
}

impl From<QueryResultFlagBits> for QueryResultFlags {
    fn from(value: QueryResultFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for QueryResultFlagBits {
    type Output = QueryResultFlags;
    fn bitor(self, rhs: Self) -> QueryResultFlags {
        QueryResultFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryResultFlags> for QueryResultFlagBits {
    type Output = QueryResultFlags;
    fn bitor(self, rhs: QueryResultFlags) -> QueryResultFlags {
        QueryResultFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueryResultFlagBits> for QueryResultFlags {
    type Output = Self;
    fn bitor(self, rhs: QueryResultFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<QueryResultFlagBits> for QueryResultFlags {
    fn bitor_assign(&mut self, rhs: QueryResultFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueueFlagBits(pub u32);

impl QueueFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const GRAPHICS: Self = Self(1u32);
    pub const COMPUTE: Self = Self(2u32);
    pub const TRANSFER: Self = Self(4u32);
    pub const SPARSE_BINDING: Self = Self(8u32);
    pub const PROTECTED: Self = Self(16u32);
    pub const VIDEO_DECODE_KHR: Self = Self(32u32);
    pub const VIDEO_ENCODE_KHR: Self = Self(64u32);
    pub const OPTICAL_FLOW_NV: Self = Self(256u32);
    pub const DATA_GRAPH_ARM: Self = Self(1024u32);
}

impl From<QueueFlagBits> for QueueFlags {
    fn from(value: QueueFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for QueueFlagBits {
    type Output = QueueFlags;
    fn bitor(self, rhs: Self) -> QueueFlags {
        QueueFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueueFlags> for QueueFlagBits {
    type Output = QueueFlags;
    fn bitor(self, rhs: QueueFlags) -> QueueFlags {
        QueueFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<QueueFlagBits> for QueueFlags {
    type Output = Self;
    fn bitor(self, rhs: QueueFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<QueueFlagBits> for QueueFlags {
    fn bitor_assign(&mut self, rhs: QueueFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderPassCreateFlagBits(pub u32);

impl RenderPassCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFORM_QCOM: Self = Self(2u32);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(4u32);
}

impl From<RenderPassCreateFlagBits> for RenderPassCreateFlags {
    fn from(value: RenderPassCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for RenderPassCreateFlagBits {
    type Output = RenderPassCreateFlags;
    fn bitor(self, rhs: Self) -> RenderPassCreateFlags {
        RenderPassCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderPassCreateFlags> for RenderPassCreateFlagBits {
    type Output = RenderPassCreateFlags;
    fn bitor(self, rhs: RenderPassCreateFlags) -> RenderPassCreateFlags {
        RenderPassCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderPassCreateFlagBits> for RenderPassCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: RenderPassCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<RenderPassCreateFlagBits> for RenderPassCreateFlags {
    fn bitor_assign(&mut self, rhs: RenderPassCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderingAttachmentFlagBitsKHR(pub u32);

impl RenderingAttachmentFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self = Self(1u32);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(4u32);
}

impl From<RenderingAttachmentFlagBitsKHR> for RenderingAttachmentFlagsKHR {
    fn from(value: RenderingAttachmentFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for RenderingAttachmentFlagBitsKHR {
    type Output = RenderingAttachmentFlagsKHR;
    fn bitor(self, rhs: Self) -> RenderingAttachmentFlagsKHR {
        RenderingAttachmentFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderingAttachmentFlagsKHR> for RenderingAttachmentFlagBitsKHR {
    type Output = RenderingAttachmentFlagsKHR;
    fn bitor(self, rhs: RenderingAttachmentFlagsKHR) -> RenderingAttachmentFlagsKHR {
        RenderingAttachmentFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderingAttachmentFlagBitsKHR> for RenderingAttachmentFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: RenderingAttachmentFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<RenderingAttachmentFlagBitsKHR> for RenderingAttachmentFlagsKHR {
    fn bitor_assign(&mut self, rhs: RenderingAttachmentFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderingFlagBits(pub u32);

impl RenderingFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1u32);
    pub const SUSPENDING: Self = Self(2u32);
    pub const RESUMING: Self = Self(4u32);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(8u32);
    pub const CONTENTS_INLINE_KHR: Self = Self(16u32);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(32u32);
    pub const FRAGMENT_REGION_EXT: Self = Self(64u32);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(128u32);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(256u32);
}

impl From<RenderingFlagBits> for RenderingFlags {
    fn from(value: RenderingFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for RenderingFlagBits {
    type Output = RenderingFlags;
    fn bitor(self, rhs: Self) -> RenderingFlags {
        RenderingFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderingFlags> for RenderingFlagBits {
    type Output = RenderingFlags;
    fn bitor(self, rhs: RenderingFlags) -> RenderingFlags {
        RenderingFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<RenderingFlagBits> for RenderingFlags {
    type Output = Self;
    fn bitor(self, rhs: RenderingFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<RenderingFlagBits> for RenderingFlags {
    fn bitor_assign(&mut self, rhs: RenderingFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type RenderingFlagBitsKHR = RenderingFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ResolveImageFlagBitsKHR(pub u32);

impl ResolveImageFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1u32);
    pub const ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
}

impl From<ResolveImageFlagBitsKHR> for ResolveImageFlagsKHR {
    fn from(value: ResolveImageFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ResolveImageFlagBitsKHR {
    type Output = ResolveImageFlagsKHR;
    fn bitor(self, rhs: Self) -> ResolveImageFlagsKHR {
        ResolveImageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ResolveImageFlagsKHR> for ResolveImageFlagBitsKHR {
    type Output = ResolveImageFlagsKHR;
    fn bitor(self, rhs: ResolveImageFlagsKHR) -> ResolveImageFlagsKHR {
        ResolveImageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ResolveImageFlagBitsKHR> for ResolveImageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: ResolveImageFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ResolveImageFlagBitsKHR> for ResolveImageFlagsKHR {
    fn bitor_assign(&mut self, rhs: ResolveImageFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ResolveModeFlagBits(pub u32);

impl ResolveModeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const SAMPLE_ZERO: Self = Self(1u32);
    pub const AVERAGE: Self = Self(2u32);
    pub const MIN: Self = Self(4u32);
    pub const MAX: Self = Self(8u32);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: Self = Self(16u32);
    pub const CUSTOM_EXT: Self = Self(32u32);
}

impl From<ResolveModeFlagBits> for ResolveModeFlags {
    fn from(value: ResolveModeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ResolveModeFlagBits {
    type Output = ResolveModeFlags;
    fn bitor(self, rhs: Self) -> ResolveModeFlags {
        ResolveModeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ResolveModeFlags> for ResolveModeFlagBits {
    type Output = ResolveModeFlags;
    fn bitor(self, rhs: ResolveModeFlags) -> ResolveModeFlags {
        ResolveModeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ResolveModeFlagBits> for ResolveModeFlags {
    type Output = Self;
    fn bitor(self, rhs: ResolveModeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ResolveModeFlagBits> for ResolveModeFlags {
    fn bitor_assign(&mut self, rhs: ResolveModeFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ResolveModeFlagBitsKHR = ResolveModeFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SampleCountFlagBits(pub u32);

impl SampleCountFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _1: Self = Self(1u32);
    pub const _2: Self = Self(2u32);
    pub const _4: Self = Self(4u32);
    pub const _8: Self = Self(8u32);
    pub const _16: Self = Self(16u32);
    pub const _32: Self = Self(32u32);
    pub const _64: Self = Self(64u32);
}

impl From<SampleCountFlagBits> for SampleCountFlags {
    fn from(value: SampleCountFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SampleCountFlagBits {
    type Output = SampleCountFlags;
    fn bitor(self, rhs: Self) -> SampleCountFlags {
        SampleCountFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SampleCountFlags> for SampleCountFlagBits {
    type Output = SampleCountFlags;
    fn bitor(self, rhs: SampleCountFlags) -> SampleCountFlags {
        SampleCountFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SampleCountFlagBits> for SampleCountFlags {
    type Output = Self;
    fn bitor(self, rhs: SampleCountFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SampleCountFlagBits> for SampleCountFlags {
    fn bitor_assign(&mut self, rhs: SampleCountFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerCreateFlagBits(pub u32);

impl SamplerCreateFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SUBSAMPLED_EXT: Self = Self(1u32);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(2u32);
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(8u32);
    pub const IMAGE_PROCESSING_QCOM: Self = Self(16u32);
}

impl From<SamplerCreateFlagBits> for SamplerCreateFlags {
    fn from(value: SamplerCreateFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SamplerCreateFlagBits {
    type Output = SamplerCreateFlags;
    fn bitor(self, rhs: Self) -> SamplerCreateFlags {
        SamplerCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SamplerCreateFlags> for SamplerCreateFlagBits {
    type Output = SamplerCreateFlags;
    fn bitor(self, rhs: SamplerCreateFlags) -> SamplerCreateFlags {
        SamplerCreateFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SamplerCreateFlagBits> for SamplerCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: SamplerCreateFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SamplerCreateFlagBits> for SamplerCreateFlags {
    fn bitor_assign(&mut self, rhs: SamplerCreateFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreImportFlagBits(pub u32);

impl SemaphoreImportFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TEMPORARY: Self = Self(1u32);
}

impl From<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn from(value: SemaphoreImportFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SemaphoreImportFlagBits {
    type Output = SemaphoreImportFlags;
    fn bitor(self, rhs: Self) -> SemaphoreImportFlags {
        SemaphoreImportFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SemaphoreImportFlags> for SemaphoreImportFlagBits {
    type Output = SemaphoreImportFlags;
    fn bitor(self, rhs: SemaphoreImportFlags) -> SemaphoreImportFlags {
        SemaphoreImportFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    type Output = Self;
    fn bitor(self, rhs: SemaphoreImportFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SemaphoreImportFlagBits> for SemaphoreImportFlags {
    fn bitor_assign(&mut self, rhs: SemaphoreImportFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type SemaphoreImportFlagBitsKHR = SemaphoreImportFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreWaitFlagBits(pub u32);

impl SemaphoreWaitFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ANY: Self = Self(1u32);
}

impl From<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn from(value: SemaphoreWaitFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SemaphoreWaitFlagBits {
    type Output = SemaphoreWaitFlags;
    fn bitor(self, rhs: Self) -> SemaphoreWaitFlags {
        SemaphoreWaitFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SemaphoreWaitFlags> for SemaphoreWaitFlagBits {
    type Output = SemaphoreWaitFlags;
    fn bitor(self, rhs: SemaphoreWaitFlags) -> SemaphoreWaitFlags {
        SemaphoreWaitFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    type Output = Self;
    fn bitor(self, rhs: SemaphoreWaitFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SemaphoreWaitFlagBits> for SemaphoreWaitFlags {
    fn bitor_assign(&mut self, rhs: SemaphoreWaitFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type SemaphoreWaitFlagBitsKHR = SemaphoreWaitFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderCorePropertiesFlagBitsAMD(pub u32);

impl ShaderCorePropertiesFlagBitsAMD {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
}

impl From<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn from(value: ShaderCorePropertiesFlagBitsAMD) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ShaderCorePropertiesFlagBitsAMD {
    type Output = ShaderCorePropertiesFlagsAMD;
    fn bitor(self, rhs: Self) -> ShaderCorePropertiesFlagsAMD {
        ShaderCorePropertiesFlagsAMD(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderCorePropertiesFlagsAMD> for ShaderCorePropertiesFlagBitsAMD {
    type Output = ShaderCorePropertiesFlagsAMD;
    fn bitor(self, rhs: ShaderCorePropertiesFlagsAMD) -> ShaderCorePropertiesFlagsAMD {
        ShaderCorePropertiesFlagsAMD(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    fn bitor(self, rhs: ShaderCorePropertiesFlagBitsAMD) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ShaderCorePropertiesFlagBitsAMD> for ShaderCorePropertiesFlagsAMD {
    fn bitor_assign(&mut self, rhs: ShaderCorePropertiesFlagBitsAMD) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderCreateFlagBitsEXT(pub u32);

impl ShaderCreateFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const LINK_STAGE_EXT: Self = Self(1u32);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(2u32);
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(4u32);
    pub const NO_TASK_SHADER_EXT: Self = Self(8u32);
    pub const DISPATCH_BASE_EXT: Self = Self(16u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self = Self(32u32);
    pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(64u32);
    pub const INDIRECT_BINDABLE_EXT: Self = Self(128u32);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(1024u32);
    pub const INSTRUMENT_SHADER_ARM: Self = Self(2048u32);
    pub const _64_BIT_INDEXING_EXT: Self = Self(32768u32);
    pub const INDEPENDENT_SETS_KHR: Self = Self(262144u32);
}

impl From<ShaderCreateFlagBitsEXT> for ShaderCreateFlagsEXT {
    fn from(value: ShaderCreateFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ShaderCreateFlagBitsEXT {
    type Output = ShaderCreateFlagsEXT;
    fn bitor(self, rhs: Self) -> ShaderCreateFlagsEXT {
        ShaderCreateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderCreateFlagsEXT> for ShaderCreateFlagBitsEXT {
    type Output = ShaderCreateFlagsEXT;
    fn bitor(self, rhs: ShaderCreateFlagsEXT) -> ShaderCreateFlagsEXT {
        ShaderCreateFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderCreateFlagBitsEXT> for ShaderCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: ShaderCreateFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ShaderCreateFlagBitsEXT> for ShaderCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: ShaderCreateFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderStageFlagBits(pub u32);

impl ShaderStageFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERTEX: Self = Self(1u32);
    pub const TESSELLATION_CONTROL: Self = Self(2u32);
    pub const TESSELLATION_EVALUATION: Self = Self(4u32);
    pub const GEOMETRY: Self = Self(8u32);
    pub const FRAGMENT: Self = Self(16u32);
    pub const ALL_GRAPHICS: Self = Self(31u32);
    pub const COMPUTE: Self = Self(32u32);
    pub const TASK_EXT: Self = Self(64u32);
    pub const MESH_EXT: Self = Self(128u32);
    pub const RAYGEN_KHR: Self = Self(256u32);
    pub const ANY_HIT_KHR: Self = Self(512u32);
    pub const CLOSEST_HIT_KHR: Self = Self(1024u32);
    pub const MISS_KHR: Self = Self(2048u32);
    pub const INTERSECTION_KHR: Self = Self(4096u32);
    pub const CALLABLE_KHR: Self = Self(8192u32);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(16384u32);
    pub const CLUSTER_CULLING_BIT_HUAWEI: Self = Self(524288u32);
    pub const ALL: Self = Self(2147483647u32);
}

impl From<ShaderStageFlagBits> for ShaderStageFlags {
    fn from(value: ShaderStageFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ShaderStageFlagBits {
    type Output = ShaderStageFlags;
    fn bitor(self, rhs: Self) -> ShaderStageFlags {
        ShaderStageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderStageFlags> for ShaderStageFlagBits {
    type Output = ShaderStageFlags;
    fn bitor(self, rhs: ShaderStageFlags) -> ShaderStageFlags {
        ShaderStageFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ShaderStageFlagBits> for ShaderStageFlags {
    type Output = Self;
    fn bitor(self, rhs: ShaderStageFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ShaderStageFlagBits> for ShaderStageFlags {
    fn bitor_assign(&mut self, rhs: ShaderStageFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SparseImageFormatFlagBits(pub u32);

impl SparseImageFormatFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SINGLE_MIPTAIL: Self = Self(1u32);
    pub const ALIGNED_MIP_SIZE: Self = Self(2u32);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(4u32);
}

impl From<SparseImageFormatFlagBits> for SparseImageFormatFlags {
    fn from(value: SparseImageFormatFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SparseImageFormatFlagBits {
    type Output = SparseImageFormatFlags;
    fn bitor(self, rhs: Self) -> SparseImageFormatFlags {
        SparseImageFormatFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SparseImageFormatFlags> for SparseImageFormatFlagBits {
    type Output = SparseImageFormatFlags;
    fn bitor(self, rhs: SparseImageFormatFlags) -> SparseImageFormatFlags {
        SparseImageFormatFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SparseImageFormatFlagBits> for SparseImageFormatFlags {
    type Output = Self;
    fn bitor(self, rhs: SparseImageFormatFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SparseImageFormatFlagBits> for SparseImageFormatFlags {
    fn bitor_assign(&mut self, rhs: SparseImageFormatFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SparseMemoryBindFlagBits(pub u32);

impl SparseMemoryBindFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const METADATA: Self = Self(1u32);
}

impl From<SparseMemoryBindFlagBits> for SparseMemoryBindFlags {
    fn from(value: SparseMemoryBindFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SparseMemoryBindFlagBits {
    type Output = SparseMemoryBindFlags;
    fn bitor(self, rhs: Self) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SparseMemoryBindFlags> for SparseMemoryBindFlagBits {
    type Output = SparseMemoryBindFlags;
    fn bitor(self, rhs: SparseMemoryBindFlags) -> SparseMemoryBindFlags {
        SparseMemoryBindFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SparseMemoryBindFlagBits> for SparseMemoryBindFlags {
    type Output = Self;
    fn bitor(self, rhs: SparseMemoryBindFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SparseMemoryBindFlagBits> for SparseMemoryBindFlags {
    fn bitor_assign(&mut self, rhs: SparseMemoryBindFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SpirvResourceTypeFlagBitsEXT(pub u32);

impl SpirvResourceTypeFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SAMPLER_EXT: Self = Self(1u32);
    pub const SAMPLED_IMAGE_EXT: Self = Self(2u32);
    pub const READ_ONLY_IMAGE_EXT: Self = Self(4u32);
    pub const READ_WRITE_IMAGE_EXT: Self = Self(8u32);
    pub const COMBINED_SAMPLED_IMAGE_EXT: Self = Self(16u32);
    pub const UNIFORM_BUFFER_EXT: Self = Self(32u32);
    pub const READ_ONLY_STORAGE_BUFFER_EXT: Self = Self(64u32);
    pub const READ_WRITE_STORAGE_BUFFER_EXT: Self = Self(128u32);
    pub const ACCELERATION_STRUCTURE_EXT: Self = Self(256u32);
    pub const TENSOR_ARM: Self = Self(512u32);
    pub const ALL_EXT: Self = Self(2147483647u32);
}

impl From<SpirvResourceTypeFlagBitsEXT> for SpirvResourceTypeFlagsEXT {
    fn from(value: SpirvResourceTypeFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SpirvResourceTypeFlagBitsEXT {
    type Output = SpirvResourceTypeFlagsEXT;
    fn bitor(self, rhs: Self) -> SpirvResourceTypeFlagsEXT {
        SpirvResourceTypeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SpirvResourceTypeFlagsEXT> for SpirvResourceTypeFlagBitsEXT {
    type Output = SpirvResourceTypeFlagsEXT;
    fn bitor(self, rhs: SpirvResourceTypeFlagsEXT) -> SpirvResourceTypeFlagsEXT {
        SpirvResourceTypeFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SpirvResourceTypeFlagBitsEXT> for SpirvResourceTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: SpirvResourceTypeFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SpirvResourceTypeFlagBitsEXT> for SpirvResourceTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: SpirvResourceTypeFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StencilFaceFlagBits(pub u32);

impl StencilFaceFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRONT: Self = Self(1u32);
    pub const BACK: Self = Self(2u32);
    pub const FRONT_AND_BACK: Self = Self(3u32);
}

impl From<StencilFaceFlagBits> for StencilFaceFlags {
    fn from(value: StencilFaceFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for StencilFaceFlagBits {
    type Output = StencilFaceFlags;
    fn bitor(self, rhs: Self) -> StencilFaceFlags {
        StencilFaceFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<StencilFaceFlags> for StencilFaceFlagBits {
    type Output = StencilFaceFlags;
    fn bitor(self, rhs: StencilFaceFlags) -> StencilFaceFlags {
        StencilFaceFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<StencilFaceFlagBits> for StencilFaceFlags {
    type Output = Self;
    fn bitor(self, rhs: StencilFaceFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<StencilFaceFlagBits> for StencilFaceFlags {
    fn bitor_assign(&mut self, rhs: StencilFaceFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubgroupFeatureFlagBits(pub u32);

impl SubgroupFeatureFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BASIC: Self = Self(1u32);
    pub const VOTE: Self = Self(2u32);
    pub const ARITHMETIC: Self = Self(4u32);
    pub const BALLOT: Self = Self(8u32);
    pub const SHUFFLE: Self = Self(16u32);
    pub const SHUFFLE_RELATIVE: Self = Self(32u32);
    pub const CLUSTERED: Self = Self(64u32);
    pub const QUAD: Self = Self(128u32);
    pub const PARTITIONED_EXT: Self = Self(256u32);
    pub const ROTATE: Self = Self(512u32);
    pub const ROTATE_CLUSTERED: Self = Self(1024u32);
}

impl From<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn from(value: SubgroupFeatureFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SubgroupFeatureFlagBits {
    type Output = SubgroupFeatureFlags;
    fn bitor(self, rhs: Self) -> SubgroupFeatureFlags {
        SubgroupFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubgroupFeatureFlags> for SubgroupFeatureFlagBits {
    type Output = SubgroupFeatureFlags;
    fn bitor(self, rhs: SubgroupFeatureFlags) -> SubgroupFeatureFlags {
        SubgroupFeatureFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: SubgroupFeatureFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SubgroupFeatureFlagBits> for SubgroupFeatureFlags {
    fn bitor_assign(&mut self, rhs: SubgroupFeatureFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubmitFlagBits(pub u32);

impl SubmitFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED: Self = Self(1u32);
}

impl From<SubmitFlagBits> for SubmitFlags {
    fn from(value: SubmitFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SubmitFlagBits {
    type Output = SubmitFlags;
    fn bitor(self, rhs: Self) -> SubmitFlags {
        SubmitFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubmitFlags> for SubmitFlagBits {
    type Output = SubmitFlags;
    fn bitor(self, rhs: SubmitFlags) -> SubmitFlags {
        SubmitFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubmitFlagBits> for SubmitFlags {
    type Output = Self;
    fn bitor(self, rhs: SubmitFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SubmitFlagBits> for SubmitFlags {
    fn bitor_assign(&mut self, rhs: SubmitFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type SubmitFlagBitsKHR = SubmitFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubpassDescriptionFlagBits(pub u32);

impl SubpassDescriptionFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PER_VIEW_ATTRIBUTES_BIT_NVX: Self = Self(1u32);
    pub const PER_VIEW_POSITION_X_ONLY_BIT_NVX: Self = Self(2u32);
    pub const FRAGMENT_REGION_EXT: Self = Self(4u32);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(8u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(16u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(32u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(64u32);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(128u32);
    pub const TILE_SHADING_APRON_QCOM: Self = Self(256u32);
}

impl From<SubpassDescriptionFlagBits> for SubpassDescriptionFlags {
    fn from(value: SubpassDescriptionFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SubpassDescriptionFlagBits {
    type Output = SubpassDescriptionFlags;
    fn bitor(self, rhs: Self) -> SubpassDescriptionFlags {
        SubpassDescriptionFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubpassDescriptionFlags> for SubpassDescriptionFlagBits {
    type Output = SubpassDescriptionFlags;
    fn bitor(self, rhs: SubpassDescriptionFlags) -> SubpassDescriptionFlags {
        SubpassDescriptionFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SubpassDescriptionFlagBits> for SubpassDescriptionFlags {
    type Output = Self;
    fn bitor(self, rhs: SubpassDescriptionFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SubpassDescriptionFlagBits> for SubpassDescriptionFlags {
    fn bitor_assign(&mut self, rhs: SubpassDescriptionFlagBits) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SurfaceCounterFlagBitsEXT(pub u32);

impl SurfaceCounterFlagBitsEXT {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VBLANK_EXT: Self = Self(1u32);
}

impl From<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn from(value: SurfaceCounterFlagBitsEXT) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SurfaceCounterFlagBitsEXT {
    type Output = SurfaceCounterFlagsEXT;
    fn bitor(self, rhs: Self) -> SurfaceCounterFlagsEXT {
        SurfaceCounterFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SurfaceCounterFlagsEXT> for SurfaceCounterFlagBitsEXT {
    type Output = SurfaceCounterFlagsEXT;
    fn bitor(self, rhs: SurfaceCounterFlagsEXT) -> SurfaceCounterFlagsEXT {
        SurfaceCounterFlagsEXT(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: SurfaceCounterFlagBitsEXT) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SurfaceCounterFlagBitsEXT> for SurfaceCounterFlagsEXT {
    fn bitor_assign(&mut self, rhs: SurfaceCounterFlagBitsEXT) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SurfaceTransformFlagBitsKHR(pub u32);

impl SurfaceTransformFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const IDENTITY_KHR: Self = Self(1u32);
    pub const ROTATE_90_KHR: Self = Self(2u32);
    pub const ROTATE_180_KHR: Self = Self(4u32);
    pub const ROTATE_270_KHR: Self = Self(8u32);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(16u32);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(32u32);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(64u32);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(128u32);
    pub const INHERIT_KHR: Self = Self(256u32);
}

impl From<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn from(value: SurfaceTransformFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SurfaceTransformFlagBitsKHR {
    type Output = SurfaceTransformFlagsKHR;
    fn bitor(self, rhs: Self) -> SurfaceTransformFlagsKHR {
        SurfaceTransformFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SurfaceTransformFlagsKHR> for SurfaceTransformFlagBitsKHR {
    type Output = SurfaceTransformFlagsKHR;
    fn bitor(self, rhs: SurfaceTransformFlagsKHR) -> SurfaceTransformFlagsKHR {
        SurfaceTransformFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: SurfaceTransformFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SurfaceTransformFlagBitsKHR> for SurfaceTransformFlagsKHR {
    fn bitor_assign(&mut self, rhs: SurfaceTransformFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SwapchainCreateFlagBitsKHR(pub u32);

impl SwapchainCreateFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1u32);
    pub const PROTECTED_KHR: Self = Self(2u32);
    pub const MUTABLE_FORMAT_KHR: Self = Self(4u32);
    pub const DEFERRED_MEMORY_ALLOCATION_KHR: Self = Self(8u32);
    pub const PRESENT_ID_2_KHR: Self = Self(64u32);
    pub const PRESENT_WAIT_2_KHR: Self = Self(128u32);
    pub const PRESENT_TIMING_EXT: Self = Self(512u32);
}

impl From<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn from(value: SwapchainCreateFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for SwapchainCreateFlagBitsKHR {
    type Output = SwapchainCreateFlagsKHR;
    fn bitor(self, rhs: Self) -> SwapchainCreateFlagsKHR {
        SwapchainCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SwapchainCreateFlagsKHR> for SwapchainCreateFlagBitsKHR {
    type Output = SwapchainCreateFlagsKHR;
    fn bitor(self, rhs: SwapchainCreateFlagsKHR) -> SwapchainCreateFlagsKHR {
        SwapchainCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: SwapchainCreateFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<SwapchainCreateFlagBitsKHR> for SwapchainCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: SwapchainCreateFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorCreateFlagBitsARM(pub u64);

impl TensorCreateFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const MUTABLE_FORMAT_ARM: Self = Self(1u64);
    pub const PROTECTED_ARM: Self = Self(2u64);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(4u64);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(8u64);
}

impl From<TensorCreateFlagBitsARM> for TensorCreateFlagsARM {
    fn from(value: TensorCreateFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for TensorCreateFlagBitsARM {
    type Output = TensorCreateFlagsARM;
    fn bitor(self, rhs: Self) -> TensorCreateFlagsARM {
        TensorCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorCreateFlagsARM> for TensorCreateFlagBitsARM {
    type Output = TensorCreateFlagsARM;
    fn bitor(self, rhs: TensorCreateFlagsARM) -> TensorCreateFlagsARM {
        TensorCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorCreateFlagBitsARM> for TensorCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: TensorCreateFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<TensorCreateFlagBitsARM> for TensorCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: TensorCreateFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorUsageFlagBitsARM(pub u64);

impl TensorUsageFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const SHADER_ARM: Self = Self(2u64);
    pub const TRANSFER_SRC_ARM: Self = Self(4u64);
    pub const TRANSFER_DST_ARM: Self = Self(8u64);
    pub const IMAGE_ALIASING_ARM: Self = Self(16u64);
    pub const DATA_GRAPH_ARM: Self = Self(32u64);
}

impl From<TensorUsageFlagBitsARM> for TensorUsageFlagsARM {
    fn from(value: TensorUsageFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for TensorUsageFlagBitsARM {
    type Output = TensorUsageFlagsARM;
    fn bitor(self, rhs: Self) -> TensorUsageFlagsARM {
        TensorUsageFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorUsageFlagsARM> for TensorUsageFlagBitsARM {
    type Output = TensorUsageFlagsARM;
    fn bitor(self, rhs: TensorUsageFlagsARM) -> TensorUsageFlagsARM {
        TensorUsageFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorUsageFlagBitsARM> for TensorUsageFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: TensorUsageFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<TensorUsageFlagBitsARM> for TensorUsageFlagsARM {
    fn bitor_assign(&mut self, rhs: TensorUsageFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorViewCreateFlagBitsARM(pub u64);

impl TensorViewCreateFlagBitsARM {
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1u64);
}

impl From<TensorViewCreateFlagBitsARM> for TensorViewCreateFlagsARM {
    fn from(value: TensorViewCreateFlagBitsARM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for TensorViewCreateFlagBitsARM {
    type Output = TensorViewCreateFlagsARM;
    fn bitor(self, rhs: Self) -> TensorViewCreateFlagsARM {
        TensorViewCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorViewCreateFlagsARM> for TensorViewCreateFlagBitsARM {
    type Output = TensorViewCreateFlagsARM;
    fn bitor(self, rhs: TensorViewCreateFlagsARM) -> TensorViewCreateFlagsARM {
        TensorViewCreateFlagsARM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TensorViewCreateFlagBitsARM> for TensorViewCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: TensorViewCreateFlagBitsARM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<TensorViewCreateFlagBitsARM> for TensorViewCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: TensorViewCreateFlagBitsARM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TileShadingRenderPassFlagBitsQCOM(pub u32);

impl TileShadingRenderPassFlagBitsQCOM {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_QCOM: Self = Self(1u32);
    pub const PER_TILE_EXECUTION_QCOM: Self = Self(2u32);
}

impl From<TileShadingRenderPassFlagBitsQCOM> for TileShadingRenderPassFlagsQCOM {
    fn from(value: TileShadingRenderPassFlagBitsQCOM) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for TileShadingRenderPassFlagBitsQCOM {
    type Output = TileShadingRenderPassFlagsQCOM;
    fn bitor(self, rhs: Self) -> TileShadingRenderPassFlagsQCOM {
        TileShadingRenderPassFlagsQCOM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TileShadingRenderPassFlagsQCOM> for TileShadingRenderPassFlagBitsQCOM {
    type Output = TileShadingRenderPassFlagsQCOM;
    fn bitor(self, rhs: TileShadingRenderPassFlagsQCOM) -> TileShadingRenderPassFlagsQCOM {
        TileShadingRenderPassFlagsQCOM(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<TileShadingRenderPassFlagBitsQCOM> for TileShadingRenderPassFlagsQCOM {
    type Output = Self;
    fn bitor(self, rhs: TileShadingRenderPassFlagBitsQCOM) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<TileShadingRenderPassFlagBitsQCOM> for TileShadingRenderPassFlagsQCOM {
    fn bitor_assign(&mut self, rhs: TileShadingRenderPassFlagBitsQCOM) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ToolPurposeFlagBits(pub u32);

impl ToolPurposeFlagBits {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VALIDATION: Self = Self(1u32);
    pub const PROFILING: Self = Self(2u32);
    pub const TRACING: Self = Self(4u32);
    pub const ADDITIONAL_FEATURES: Self = Self(8u32);
    pub const MODIFYING_FEATURES: Self = Self(16u32);
    pub const DEBUG_REPORTING_EXT: Self = Self(32u32);
    pub const DEBUG_MARKERS_EXT: Self = Self(64u32);
}

impl From<ToolPurposeFlagBits> for ToolPurposeFlags {
    fn from(value: ToolPurposeFlagBits) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for ToolPurposeFlagBits {
    type Output = ToolPurposeFlags;
    fn bitor(self, rhs: Self) -> ToolPurposeFlags {
        ToolPurposeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ToolPurposeFlags> for ToolPurposeFlagBits {
    type Output = ToolPurposeFlags;
    fn bitor(self, rhs: ToolPurposeFlags) -> ToolPurposeFlags {
        ToolPurposeFlags(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<ToolPurposeFlagBits> for ToolPurposeFlags {
    type Output = Self;
    fn bitor(self, rhs: ToolPurposeFlagBits) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<ToolPurposeFlagBits> for ToolPurposeFlags {
    fn bitor_assign(&mut self, rhs: ToolPurposeFlagBits) {
        self.0 |= rhs.0;
    }
}

pub type ToolPurposeFlagBitsEXT = ToolPurposeFlagBits;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCapabilityFlagBitsKHR(pub u32);

impl VideoCapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_CONTENT_KHR: Self = Self(1u32);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(2u32);
}

impl From<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn from(value: VideoCapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoCapabilityFlagBitsKHR {
    type Output = VideoCapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoCapabilityFlagsKHR {
        VideoCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCapabilityFlagsKHR> for VideoCapabilityFlagBitsKHR {
    type Output = VideoCapabilityFlagsKHR;
    fn bitor(self, rhs: VideoCapabilityFlagsKHR) -> VideoCapabilityFlagsKHR {
        VideoCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoCapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoCapabilityFlagBitsKHR> for VideoCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoCapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoChromaSubsamplingFlagBitsKHR(pub u32);

impl VideoChromaSubsamplingFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVALID_KHR: Self = Self(0u32);
    pub const MONOCHROME_KHR: Self = Self(1u32);
    pub const _420_KHR: Self = Self(2u32);
    pub const _422_KHR: Self = Self(4u32);
    pub const _444_KHR: Self = Self(8u32);
}

impl From<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn from(value: VideoChromaSubsamplingFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoChromaSubsamplingFlagBitsKHR {
    type Output = VideoChromaSubsamplingFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoChromaSubsamplingFlagsKHR {
        VideoChromaSubsamplingFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoChromaSubsamplingFlagsKHR> for VideoChromaSubsamplingFlagBitsKHR {
    type Output = VideoChromaSubsamplingFlagsKHR;
    fn bitor(self, rhs: VideoChromaSubsamplingFlagsKHR) -> VideoChromaSubsamplingFlagsKHR {
        VideoChromaSubsamplingFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoChromaSubsamplingFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoChromaSubsamplingFlagBitsKHR> for VideoChromaSubsamplingFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoChromaSubsamplingFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCodecOperationFlagBitsKHR(pub u32);

impl VideoCodecOperationFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_KHR: Self = Self(0u32);
    pub const DECODE_H264_KHR: Self = Self(1u32);
    pub const DECODE_H265_KHR: Self = Self(2u32);
    pub const DECODE_AV1_KHR: Self = Self(4u32);
    pub const DECODE_VP9_KHR: Self = Self(8u32);
    pub const ENCODE_H264_KHR: Self = Self(65536u32);
    pub const ENCODE_H265_KHR: Self = Self(131072u32);
    pub const ENCODE_AV1_KHR: Self = Self(262144u32);
}

impl From<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn from(value: VideoCodecOperationFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoCodecOperationFlagBitsKHR {
    type Output = VideoCodecOperationFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoCodecOperationFlagsKHR {
        VideoCodecOperationFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCodecOperationFlagsKHR> for VideoCodecOperationFlagBitsKHR {
    type Output = VideoCodecOperationFlagsKHR;
    fn bitor(self, rhs: VideoCodecOperationFlagsKHR) -> VideoCodecOperationFlagsKHR {
        VideoCodecOperationFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoCodecOperationFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoCodecOperationFlagBitsKHR> for VideoCodecOperationFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoCodecOperationFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCodingControlFlagBitsKHR(pub u32);

impl VideoCodingControlFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESET_KHR: Self = Self(1u32);
    pub const ENCODE_RATE_CONTROL_KHR: Self = Self(2u32);
    pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(4u32);
}

impl From<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn from(value: VideoCodingControlFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoCodingControlFlagBitsKHR {
    type Output = VideoCodingControlFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoCodingControlFlagsKHR {
        VideoCodingControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCodingControlFlagsKHR> for VideoCodingControlFlagBitsKHR {
    type Output = VideoCodingControlFlagsKHR;
    fn bitor(self, rhs: VideoCodingControlFlagsKHR) -> VideoCodingControlFlagsKHR {
        VideoCodingControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoCodingControlFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoCodingControlFlagBitsKHR> for VideoCodingControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoCodingControlFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoComponentBitDepthFlagBitsKHR(pub u32);

impl VideoComponentBitDepthFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVALID_KHR: Self = Self(0u32);
    pub const _8_KHR: Self = Self(1u32);
    pub const _10_KHR: Self = Self(4u32);
    pub const _12_KHR: Self = Self(16u32);
}

impl From<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn from(value: VideoComponentBitDepthFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoComponentBitDepthFlagBitsKHR {
    type Output = VideoComponentBitDepthFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoComponentBitDepthFlagsKHR {
        VideoComponentBitDepthFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoComponentBitDepthFlagsKHR> for VideoComponentBitDepthFlagBitsKHR {
    type Output = VideoComponentBitDepthFlagsKHR;
    fn bitor(self, rhs: VideoComponentBitDepthFlagsKHR) -> VideoComponentBitDepthFlagsKHR {
        VideoComponentBitDepthFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoComponentBitDepthFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoComponentBitDepthFlagBitsKHR> for VideoComponentBitDepthFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoComponentBitDepthFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeCapabilityFlagBitsKHR(pub u32);

impl VideoDecodeCapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1u32);
    pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(2u32);
}

impl From<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn from(value: VideoDecodeCapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoDecodeCapabilityFlagBitsKHR {
    type Output = VideoDecodeCapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoDecodeCapabilityFlagsKHR {
        VideoDecodeCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeCapabilityFlagsKHR> for VideoDecodeCapabilityFlagBitsKHR {
    type Output = VideoDecodeCapabilityFlagsKHR;
    fn bitor(self, rhs: VideoDecodeCapabilityFlagsKHR) -> VideoDecodeCapabilityFlagsKHR {
        VideoDecodeCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoDecodeCapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoDecodeCapabilityFlagBitsKHR> for VideoDecodeCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoDecodeCapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeH264PictureLayoutFlagBitsKHR(pub u32);

impl VideoDecodeH264PictureLayoutFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROGRESSIVE_KHR: Self = Self(0u32);
    pub const INTERLACED_INTERLEAVED_LINES_KHR: Self = Self(1u32);
    pub const INTERLACED_SEPARATE_PLANES_KHR: Self = Self(2u32);
}

impl From<VideoDecodeH264PictureLayoutFlagBitsKHR> for VideoDecodeH264PictureLayoutFlagsKHR {
    fn from(value: VideoDecodeH264PictureLayoutFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoDecodeH264PictureLayoutFlagBitsKHR {
    type Output = VideoDecodeH264PictureLayoutFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoDecodeH264PictureLayoutFlagsKHR {
        VideoDecodeH264PictureLayoutFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeH264PictureLayoutFlagsKHR>
    for VideoDecodeH264PictureLayoutFlagBitsKHR
{
    type Output = VideoDecodeH264PictureLayoutFlagsKHR;
    fn bitor(
        self,
        rhs: VideoDecodeH264PictureLayoutFlagsKHR,
    ) -> VideoDecodeH264PictureLayoutFlagsKHR {
        VideoDecodeH264PictureLayoutFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeH264PictureLayoutFlagBitsKHR>
    for VideoDecodeH264PictureLayoutFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoDecodeH264PictureLayoutFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoDecodeH264PictureLayoutFlagBitsKHR>
    for VideoDecodeH264PictureLayoutFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoDecodeH264PictureLayoutFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeUsageFlagBitsKHR(pub u32);

impl VideoDecodeUsageFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const TRANSCODING_KHR: Self = Self(1u32);
    pub const OFFLINE_KHR: Self = Self(2u32);
    pub const STREAMING_KHR: Self = Self(4u32);
}

impl From<VideoDecodeUsageFlagBitsKHR> for VideoDecodeUsageFlagsKHR {
    fn from(value: VideoDecodeUsageFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoDecodeUsageFlagBitsKHR {
    type Output = VideoDecodeUsageFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoDecodeUsageFlagsKHR {
        VideoDecodeUsageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeUsageFlagsKHR> for VideoDecodeUsageFlagBitsKHR {
    type Output = VideoDecodeUsageFlagsKHR;
    fn bitor(self, rhs: VideoDecodeUsageFlagsKHR) -> VideoDecodeUsageFlagsKHR {
        VideoDecodeUsageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoDecodeUsageFlagBitsKHR> for VideoDecodeUsageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoDecodeUsageFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoDecodeUsageFlagBitsKHR> for VideoDecodeUsageFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoDecodeUsageFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1CapabilityFlagBitsKHR(pub u32);

impl VideoEncodeAV1CapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self = Self(1u32);
    pub const GENERATE_OBU_EXTENSION_HEADER_KHR: Self = Self(2u32);
    pub const PRIMARY_REFERENCE_CDF_ONLY_KHR: Self = Self(4u32);
    pub const FRAME_SIZE_OVERRIDE_KHR: Self = Self(8u32);
    pub const MOTION_VECTOR_SCALING_KHR: Self = Self(16u32);
    pub const COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self = Self(32u32);
}

impl From<VideoEncodeAV1CapabilityFlagBitsKHR> for VideoEncodeAV1CapabilityFlagsKHR {
    fn from(value: VideoEncodeAV1CapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeAV1CapabilityFlagBitsKHR {
    type Output = VideoEncodeAV1CapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeAV1CapabilityFlagsKHR {
        VideoEncodeAV1CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1CapabilityFlagsKHR> for VideoEncodeAV1CapabilityFlagBitsKHR {
    type Output = VideoEncodeAV1CapabilityFlagsKHR;
    fn bitor(self, rhs: VideoEncodeAV1CapabilityFlagsKHR) -> VideoEncodeAV1CapabilityFlagsKHR {
        VideoEncodeAV1CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1CapabilityFlagBitsKHR> for VideoEncodeAV1CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeAV1CapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeAV1CapabilityFlagBitsKHR>
    for VideoEncodeAV1CapabilityFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeAV1CapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1RateControlFlagBitsKHR(pub u32);

impl VideoEncodeAV1RateControlFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const REGULAR_GOP_KHR: Self = Self(1u32);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
}

impl From<VideoEncodeAV1RateControlFlagBitsKHR> for VideoEncodeAV1RateControlFlagsKHR {
    fn from(value: VideoEncodeAV1RateControlFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeAV1RateControlFlagBitsKHR {
    type Output = VideoEncodeAV1RateControlFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeAV1RateControlFlagsKHR {
        VideoEncodeAV1RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1RateControlFlagsKHR> for VideoEncodeAV1RateControlFlagBitsKHR {
    type Output = VideoEncodeAV1RateControlFlagsKHR;
    fn bitor(self, rhs: VideoEncodeAV1RateControlFlagsKHR) -> VideoEncodeAV1RateControlFlagsKHR {
        VideoEncodeAV1RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1RateControlFlagBitsKHR> for VideoEncodeAV1RateControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeAV1RateControlFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeAV1RateControlFlagBitsKHR>
    for VideoEncodeAV1RateControlFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeAV1RateControlFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1StdFlagBitsKHR(pub u32);

impl VideoEncodeAV1StdFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self = Self(1u32);
    pub const SKIP_MODE_PRESENT_UNSET_KHR: Self = Self(2u32);
    pub const PRIMARY_REF_FRAME_KHR: Self = Self(4u32);
    pub const DELTA_Q_KHR: Self = Self(8u32);
}

impl From<VideoEncodeAV1StdFlagBitsKHR> for VideoEncodeAV1StdFlagsKHR {
    fn from(value: VideoEncodeAV1StdFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeAV1StdFlagBitsKHR {
    type Output = VideoEncodeAV1StdFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeAV1StdFlagsKHR {
        VideoEncodeAV1StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1StdFlagsKHR> for VideoEncodeAV1StdFlagBitsKHR {
    type Output = VideoEncodeAV1StdFlagsKHR;
    fn bitor(self, rhs: VideoEncodeAV1StdFlagsKHR) -> VideoEncodeAV1StdFlagsKHR {
        VideoEncodeAV1StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1StdFlagBitsKHR> for VideoEncodeAV1StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeAV1StdFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeAV1StdFlagBitsKHR> for VideoEncodeAV1StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeAV1StdFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1SuperblockSizeFlagBitsKHR(pub u32);

impl VideoEncodeAV1SuperblockSizeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _64_KHR: Self = Self(1u32);
    pub const _128_KHR: Self = Self(2u32);
}

impl From<VideoEncodeAV1SuperblockSizeFlagBitsKHR> for VideoEncodeAV1SuperblockSizeFlagsKHR {
    fn from(value: VideoEncodeAV1SuperblockSizeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeAV1SuperblockSizeFlagBitsKHR {
    type Output = VideoEncodeAV1SuperblockSizeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeAV1SuperblockSizeFlagsKHR {
        VideoEncodeAV1SuperblockSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1SuperblockSizeFlagsKHR>
    for VideoEncodeAV1SuperblockSizeFlagBitsKHR
{
    type Output = VideoEncodeAV1SuperblockSizeFlagsKHR;
    fn bitor(
        self,
        rhs: VideoEncodeAV1SuperblockSizeFlagsKHR,
    ) -> VideoEncodeAV1SuperblockSizeFlagsKHR {
        VideoEncodeAV1SuperblockSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeAV1SuperblockSizeFlagBitsKHR>
    for VideoEncodeAV1SuperblockSizeFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeAV1SuperblockSizeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeAV1SuperblockSizeFlagBitsKHR>
    for VideoEncodeAV1SuperblockSizeFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeAV1SuperblockSizeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeCapabilityFlagBitsKHR(pub u32);

impl VideoEncodeCapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(1u32);
    pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(2u32);
    pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(4u32);
    pub const EMPHASIS_MAP_KHR: Self = Self(8u32);
}

impl From<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn from(value: VideoEncodeCapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeCapabilityFlagBitsKHR {
    type Output = VideoEncodeCapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeCapabilityFlagsKHR {
        VideoEncodeCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeCapabilityFlagsKHR> for VideoEncodeCapabilityFlagBitsKHR {
    type Output = VideoEncodeCapabilityFlagsKHR;
    fn bitor(self, rhs: VideoEncodeCapabilityFlagsKHR) -> VideoEncodeCapabilityFlagsKHR {
        VideoEncodeCapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeCapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeCapabilityFlagBitsKHR> for VideoEncodeCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeCapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeContentFlagBitsKHR(pub u32);

impl VideoEncodeContentFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const CAMERA_KHR: Self = Self(1u32);
    pub const DESKTOP_KHR: Self = Self(2u32);
    pub const RENDERED_KHR: Self = Self(4u32);
}

impl From<VideoEncodeContentFlagBitsKHR> for VideoEncodeContentFlagsKHR {
    fn from(value: VideoEncodeContentFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeContentFlagBitsKHR {
    type Output = VideoEncodeContentFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeContentFlagsKHR {
        VideoEncodeContentFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeContentFlagsKHR> for VideoEncodeContentFlagBitsKHR {
    type Output = VideoEncodeContentFlagsKHR;
    fn bitor(self, rhs: VideoEncodeContentFlagsKHR) -> VideoEncodeContentFlagsKHR {
        VideoEncodeContentFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeContentFlagBitsKHR> for VideoEncodeContentFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeContentFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeContentFlagBitsKHR> for VideoEncodeContentFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeContentFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeFeedbackFlagBitsKHR(pub u32);

impl VideoEncodeFeedbackFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1u32);
    pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(2u32);
    pub const BITSTREAM_HAS_OVERRIDES_KHR: Self = Self(4u32);
}

impl From<VideoEncodeFeedbackFlagBitsKHR> for VideoEncodeFeedbackFlagsKHR {
    fn from(value: VideoEncodeFeedbackFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeFeedbackFlagBitsKHR {
    type Output = VideoEncodeFeedbackFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeFeedbackFlagsKHR {
        VideoEncodeFeedbackFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeFeedbackFlagsKHR> for VideoEncodeFeedbackFlagBitsKHR {
    type Output = VideoEncodeFeedbackFlagsKHR;
    fn bitor(self, rhs: VideoEncodeFeedbackFlagsKHR) -> VideoEncodeFeedbackFlagsKHR {
        VideoEncodeFeedbackFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeFeedbackFlagBitsKHR> for VideoEncodeFeedbackFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeFeedbackFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeFeedbackFlagBitsKHR> for VideoEncodeFeedbackFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeFeedbackFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeFlagBitsKHR(pub u32);

impl VideoEncodeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1u32);
    pub const WITH_EMPHASIS_MAP_KHR: Self = Self(2u32);
    pub const INTRA_REFRESH_KHR: Self = Self(4u32);
}

impl From<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn from(value: VideoEncodeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeFlagBitsKHR {
    type Output = VideoEncodeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeFlagsKHR {
        VideoEncodeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeFlagsKHR> for VideoEncodeFlagBitsKHR {
    type Output = VideoEncodeFlagsKHR;
    fn bitor(self, rhs: VideoEncodeFlagsKHR) -> VideoEncodeFlagsKHR {
        VideoEncodeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeFlagBitsKHR> for VideoEncodeFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264CapabilityFlagBitsKHR(pub u32);

impl VideoEncodeH264CapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(2u32);
    pub const ROW_UNALIGNED_SLICE_KHR: Self = Self(4u32);
    pub const DIFFERENT_SLICE_TYPE_KHR: Self = Self(8u32);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(16u32);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(32u32);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(64u32);
    pub const PER_SLICE_CONSTANT_QP_KHR: Self = Self(128u32);
    pub const GENERATE_PREFIX_NALU_KHR: Self = Self(256u32);
    pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(512u32);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1024u32);
}

impl From<VideoEncodeH264CapabilityFlagBitsKHR> for VideoEncodeH264CapabilityFlagsKHR {
    fn from(value: VideoEncodeH264CapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH264CapabilityFlagBitsKHR {
    type Output = VideoEncodeH264CapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH264CapabilityFlagsKHR {
        VideoEncodeH264CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264CapabilityFlagsKHR> for VideoEncodeH264CapabilityFlagBitsKHR {
    type Output = VideoEncodeH264CapabilityFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH264CapabilityFlagsKHR) -> VideoEncodeH264CapabilityFlagsKHR {
        VideoEncodeH264CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264CapabilityFlagBitsKHR> for VideoEncodeH264CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH264CapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH264CapabilityFlagBitsKHR>
    for VideoEncodeH264CapabilityFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeH264CapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264RateControlFlagBitsKHR(pub u32);

impl VideoEncodeH264RateControlFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const REGULAR_GOP_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(16u32);
}

impl From<VideoEncodeH264RateControlFlagBitsKHR> for VideoEncodeH264RateControlFlagsKHR {
    fn from(value: VideoEncodeH264RateControlFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH264RateControlFlagBitsKHR {
    type Output = VideoEncodeH264RateControlFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH264RateControlFlagsKHR {
        VideoEncodeH264RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264RateControlFlagsKHR>
    for VideoEncodeH264RateControlFlagBitsKHR
{
    type Output = VideoEncodeH264RateControlFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH264RateControlFlagsKHR) -> VideoEncodeH264RateControlFlagsKHR {
        VideoEncodeH264RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264RateControlFlagBitsKHR>
    for VideoEncodeH264RateControlFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH264RateControlFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH264RateControlFlagBitsKHR>
    for VideoEncodeH264RateControlFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeH264RateControlFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264StdFlagBitsKHR(pub u32);

impl VideoEncodeH264StdFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1u32);
    pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR: Self = Self(2u32);
    pub const SCALING_MATRIX_PRESENT_FLAG_SET_KHR: Self = Self(4u32);
    pub const CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(8u32);
    pub const SECOND_CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(16u32);
    pub const PIC_INIT_QP_MINUS26_KHR: Self = Self(32u32);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(64u32);
    pub const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR: Self = Self(128u32);
    pub const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR: Self = Self(256u32);
    pub const TRANSFORM_8X8_MODE_FLAG_SET_KHR: Self = Self(512u32);
    pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR: Self = Self(1024u32);
    pub const ENTROPY_CODING_MODE_FLAG_UNSET_KHR: Self = Self(2048u32);
    pub const ENTROPY_CODING_MODE_FLAG_SET_KHR: Self = Self(4096u32);
    pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR: Self = Self(8192u32);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(16384u32);
    pub const DEBLOCKING_FILTER_DISABLED_KHR: Self = Self(32768u32);
    pub const DEBLOCKING_FILTER_ENABLED_KHR: Self = Self(65536u32);
    pub const DEBLOCKING_FILTER_PARTIAL_KHR: Self = Self(131072u32);
    pub const SLICE_QP_DELTA_KHR: Self = Self(524288u32);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1048576u32);
}

impl From<VideoEncodeH264StdFlagBitsKHR> for VideoEncodeH264StdFlagsKHR {
    fn from(value: VideoEncodeH264StdFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH264StdFlagBitsKHR {
    type Output = VideoEncodeH264StdFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH264StdFlagsKHR {
        VideoEncodeH264StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264StdFlagsKHR> for VideoEncodeH264StdFlagBitsKHR {
    type Output = VideoEncodeH264StdFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH264StdFlagsKHR) -> VideoEncodeH264StdFlagsKHR {
        VideoEncodeH264StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH264StdFlagBitsKHR> for VideoEncodeH264StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH264StdFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH264StdFlagBitsKHR> for VideoEncodeH264StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeH264StdFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265CapabilityFlagBitsKHR(pub u32);

impl VideoEncodeH265CapabilityFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(2u32);
    pub const ROW_UNALIGNED_SLICE_SEGMENT_KHR: Self = Self(4u32);
    pub const DIFFERENT_SLICE_SEGMENT_TYPE_KHR: Self = Self(8u32);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(16u32);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(32u32);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(64u32);
    pub const PER_SLICE_SEGMENT_CONSTANT_QP_KHR: Self = Self(128u32);
    pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR: Self = Self(256u32);
    pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR: Self = Self(512u32);
    pub const CU_QP_DIFF_WRAPAROUND_KHR: Self = Self(1024u32);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(2048u32);
}

impl From<VideoEncodeH265CapabilityFlagBitsKHR> for VideoEncodeH265CapabilityFlagsKHR {
    fn from(value: VideoEncodeH265CapabilityFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH265CapabilityFlagBitsKHR {
    type Output = VideoEncodeH265CapabilityFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH265CapabilityFlagsKHR {
        VideoEncodeH265CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265CapabilityFlagsKHR> for VideoEncodeH265CapabilityFlagBitsKHR {
    type Output = VideoEncodeH265CapabilityFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH265CapabilityFlagsKHR) -> VideoEncodeH265CapabilityFlagsKHR {
        VideoEncodeH265CapabilityFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265CapabilityFlagBitsKHR> for VideoEncodeH265CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH265CapabilityFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH265CapabilityFlagBitsKHR>
    for VideoEncodeH265CapabilityFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeH265CapabilityFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265CtbSizeFlagBitsKHR(pub u32);

impl VideoEncodeH265CtbSizeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _16_KHR: Self = Self(1u32);
    pub const _32_KHR: Self = Self(2u32);
    pub const _64_KHR: Self = Self(4u32);
}

impl From<VideoEncodeH265CtbSizeFlagBitsKHR> for VideoEncodeH265CtbSizeFlagsKHR {
    fn from(value: VideoEncodeH265CtbSizeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH265CtbSizeFlagBitsKHR {
    type Output = VideoEncodeH265CtbSizeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH265CtbSizeFlagsKHR {
        VideoEncodeH265CtbSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265CtbSizeFlagsKHR> for VideoEncodeH265CtbSizeFlagBitsKHR {
    type Output = VideoEncodeH265CtbSizeFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH265CtbSizeFlagsKHR) -> VideoEncodeH265CtbSizeFlagsKHR {
        VideoEncodeH265CtbSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265CtbSizeFlagBitsKHR> for VideoEncodeH265CtbSizeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH265CtbSizeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH265CtbSizeFlagBitsKHR> for VideoEncodeH265CtbSizeFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeH265CtbSizeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265RateControlFlagBitsKHR(pub u32);

impl VideoEncodeH265RateControlFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const REGULAR_GOP_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
    pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self = Self(16u32);
}

impl From<VideoEncodeH265RateControlFlagBitsKHR> for VideoEncodeH265RateControlFlagsKHR {
    fn from(value: VideoEncodeH265RateControlFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH265RateControlFlagBitsKHR {
    type Output = VideoEncodeH265RateControlFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH265RateControlFlagsKHR {
        VideoEncodeH265RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265RateControlFlagsKHR>
    for VideoEncodeH265RateControlFlagBitsKHR
{
    type Output = VideoEncodeH265RateControlFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH265RateControlFlagsKHR) -> VideoEncodeH265RateControlFlagsKHR {
        VideoEncodeH265RateControlFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265RateControlFlagBitsKHR>
    for VideoEncodeH265RateControlFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH265RateControlFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH265RateControlFlagBitsKHR>
    for VideoEncodeH265RateControlFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeH265RateControlFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265StdFlagBitsKHR(pub u32);

impl VideoEncodeH265StdFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1u32);
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR: Self = Self(2u32);
    pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR: Self = Self(4u32);
    pub const PCM_ENABLED_FLAG_SET_KHR: Self = Self(8u32);
    pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR: Self = Self(16u32);
    pub const INIT_QP_MINUS26_KHR: Self = Self(32u32);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(64u32);
    pub const WEIGHTED_BIPRED_FLAG_SET_KHR: Self = Self(128u32);
    pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR: Self = Self(256u32);
    pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR: Self = Self(512u32);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR: Self = Self(1024u32);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR: Self = Self(2048u32);
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR: Self = Self(4096u32);
    pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR: Self = Self(8192u32);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(16384u32);
    pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR: Self = Self(32768u32);
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR: Self = Self(65536u32);
    pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR: Self = Self(131072u32);
    pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR: Self = Self(262144u32);
    pub const SLICE_QP_DELTA_KHR: Self = Self(524288u32);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1048576u32);
}

impl From<VideoEncodeH265StdFlagBitsKHR> for VideoEncodeH265StdFlagsKHR {
    fn from(value: VideoEncodeH265StdFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH265StdFlagBitsKHR {
    type Output = VideoEncodeH265StdFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH265StdFlagsKHR {
        VideoEncodeH265StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265StdFlagsKHR> for VideoEncodeH265StdFlagBitsKHR {
    type Output = VideoEncodeH265StdFlagsKHR;
    fn bitor(self, rhs: VideoEncodeH265StdFlagsKHR) -> VideoEncodeH265StdFlagsKHR {
        VideoEncodeH265StdFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265StdFlagBitsKHR> for VideoEncodeH265StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH265StdFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH265StdFlagBitsKHR> for VideoEncodeH265StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeH265StdFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265TransformBlockSizeFlagBitsKHR(pub u32);

impl VideoEncodeH265TransformBlockSizeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _4_KHR: Self = Self(1u32);
    pub const _8_KHR: Self = Self(2u32);
    pub const _16_KHR: Self = Self(4u32);
    pub const _32_KHR: Self = Self(8u32);
}

impl From<VideoEncodeH265TransformBlockSizeFlagBitsKHR>
    for VideoEncodeH265TransformBlockSizeFlagsKHR
{
    fn from(value: VideoEncodeH265TransformBlockSizeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeH265TransformBlockSizeFlagBitsKHR {
    type Output = VideoEncodeH265TransformBlockSizeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeH265TransformBlockSizeFlagsKHR {
        VideoEncodeH265TransformBlockSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265TransformBlockSizeFlagsKHR>
    for VideoEncodeH265TransformBlockSizeFlagBitsKHR
{
    type Output = VideoEncodeH265TransformBlockSizeFlagsKHR;
    fn bitor(
        self,
        rhs: VideoEncodeH265TransformBlockSizeFlagsKHR,
    ) -> VideoEncodeH265TransformBlockSizeFlagsKHR {
        VideoEncodeH265TransformBlockSizeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeH265TransformBlockSizeFlagBitsKHR>
    for VideoEncodeH265TransformBlockSizeFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeH265TransformBlockSizeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeH265TransformBlockSizeFlagBitsKHR>
    for VideoEncodeH265TransformBlockSizeFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeH265TransformBlockSizeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(pub u32);

impl VideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_KHR: Self = Self(0u32);
    pub const PER_PICTURE_PARTITION_KHR: Self = Self(1u32);
    pub const BLOCK_BASED_KHR: Self = Self(2u32);
    pub const BLOCK_ROW_BASED_KHR: Self = Self(4u32);
    pub const BLOCK_COLUMN_BASED_KHR: Self = Self(8u32);
}

impl From<VideoEncodeIntraRefreshModeFlagBitsKHR> for VideoEncodeIntraRefreshModeFlagsKHR {
    fn from(value: VideoEncodeIntraRefreshModeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeIntraRefreshModeFlagBitsKHR {
    type Output = VideoEncodeIntraRefreshModeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeIntraRefreshModeFlagsKHR {
        VideoEncodeIntraRefreshModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeIntraRefreshModeFlagsKHR>
    for VideoEncodeIntraRefreshModeFlagBitsKHR
{
    type Output = VideoEncodeIntraRefreshModeFlagsKHR;
    fn bitor(
        self,
        rhs: VideoEncodeIntraRefreshModeFlagsKHR,
    ) -> VideoEncodeIntraRefreshModeFlagsKHR {
        VideoEncodeIntraRefreshModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeIntraRefreshModeFlagBitsKHR>
    for VideoEncodeIntraRefreshModeFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeIntraRefreshModeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeIntraRefreshModeFlagBitsKHR>
    for VideoEncodeIntraRefreshModeFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeIntraRefreshModeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(pub u32);

impl VideoEncodeRateControlModeFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const DISABLED_KHR: Self = Self(1u32);
    pub const CBR_KHR: Self = Self(2u32);
    pub const VBR_KHR: Self = Self(4u32);
}

impl From<VideoEncodeRateControlModeFlagBitsKHR> for VideoEncodeRateControlModeFlagsKHR {
    fn from(value: VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeRateControlModeFlagBitsKHR {
    type Output = VideoEncodeRateControlModeFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeRateControlModeFlagsKHR {
        VideoEncodeRateControlModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRateControlModeFlagsKHR>
    for VideoEncodeRateControlModeFlagBitsKHR
{
    type Output = VideoEncodeRateControlModeFlagsKHR;
    fn bitor(self, rhs: VideoEncodeRateControlModeFlagsKHR) -> VideoEncodeRateControlModeFlagsKHR {
        VideoEncodeRateControlModeFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRateControlModeFlagBitsKHR>
    for VideoEncodeRateControlModeFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeRateControlModeFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeRateControlModeFlagBitsKHR>
    for VideoEncodeRateControlModeFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoEncodeRateControlModeFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbChromaOffsetFlagBitsVALVE(pub u32);

impl VideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const COSITED_EVEN_BIT_VALVE: Self = Self(1u32);
    pub const MIDPOINT_BIT_VALVE: Self = Self(2u32);
}

impl From<VideoEncodeRgbChromaOffsetFlagBitsVALVE> for VideoEncodeRgbChromaOffsetFlagsVALVE {
    fn from(value: VideoEncodeRgbChromaOffsetFlagBitsVALVE) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeRgbChromaOffsetFlagBitsVALVE {
    type Output = VideoEncodeRgbChromaOffsetFlagsVALVE;
    fn bitor(self, rhs: Self) -> VideoEncodeRgbChromaOffsetFlagsVALVE {
        VideoEncodeRgbChromaOffsetFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbChromaOffsetFlagsVALVE>
    for VideoEncodeRgbChromaOffsetFlagBitsVALVE
{
    type Output = VideoEncodeRgbChromaOffsetFlagsVALVE;
    fn bitor(
        self,
        rhs: VideoEncodeRgbChromaOffsetFlagsVALVE,
    ) -> VideoEncodeRgbChromaOffsetFlagsVALVE {
        VideoEncodeRgbChromaOffsetFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbChromaOffsetFlagBitsVALVE>
    for VideoEncodeRgbChromaOffsetFlagsVALVE
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeRgbChromaOffsetFlagBitsVALVE) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeRgbChromaOffsetFlagBitsVALVE>
    for VideoEncodeRgbChromaOffsetFlagsVALVE
{
    fn bitor_assign(&mut self, rhs: VideoEncodeRgbChromaOffsetFlagBitsVALVE) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbModelConversionFlagBitsVALVE(pub u32);

impl VideoEncodeRgbModelConversionFlagBitsVALVE {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RGB_IDENTITY_BIT_VALVE: Self = Self(1u32);
    pub const YCBCR_IDENTITY_BIT_VALVE: Self = Self(2u32);
    pub const YCBCR_709_BIT_VALVE: Self = Self(4u32);
    pub const YCBCR_601_BIT_VALVE: Self = Self(8u32);
    pub const YCBCR_2020_BIT_VALVE: Self = Self(16u32);
}

impl From<VideoEncodeRgbModelConversionFlagBitsVALVE> for VideoEncodeRgbModelConversionFlagsVALVE {
    fn from(value: VideoEncodeRgbModelConversionFlagBitsVALVE) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeRgbModelConversionFlagBitsVALVE {
    type Output = VideoEncodeRgbModelConversionFlagsVALVE;
    fn bitor(self, rhs: Self) -> VideoEncodeRgbModelConversionFlagsVALVE {
        VideoEncodeRgbModelConversionFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbModelConversionFlagsVALVE>
    for VideoEncodeRgbModelConversionFlagBitsVALVE
{
    type Output = VideoEncodeRgbModelConversionFlagsVALVE;
    fn bitor(
        self,
        rhs: VideoEncodeRgbModelConversionFlagsVALVE,
    ) -> VideoEncodeRgbModelConversionFlagsVALVE {
        VideoEncodeRgbModelConversionFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbModelConversionFlagBitsVALVE>
    for VideoEncodeRgbModelConversionFlagsVALVE
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeRgbModelConversionFlagBitsVALVE) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeRgbModelConversionFlagBitsVALVE>
    for VideoEncodeRgbModelConversionFlagsVALVE
{
    fn bitor_assign(&mut self, rhs: VideoEncodeRgbModelConversionFlagBitsVALVE) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbRangeCompressionFlagBitsVALVE(pub u32);

impl VideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FULL_RANGE_BIT_VALVE: Self = Self(1u32);
    pub const NARROW_RANGE_BIT_VALVE: Self = Self(2u32);
}

impl From<VideoEncodeRgbRangeCompressionFlagBitsVALVE>
    for VideoEncodeRgbRangeCompressionFlagsVALVE
{
    fn from(value: VideoEncodeRgbRangeCompressionFlagBitsVALVE) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeRgbRangeCompressionFlagBitsVALVE {
    type Output = VideoEncodeRgbRangeCompressionFlagsVALVE;
    fn bitor(self, rhs: Self) -> VideoEncodeRgbRangeCompressionFlagsVALVE {
        VideoEncodeRgbRangeCompressionFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbRangeCompressionFlagsVALVE>
    for VideoEncodeRgbRangeCompressionFlagBitsVALVE
{
    type Output = VideoEncodeRgbRangeCompressionFlagsVALVE;
    fn bitor(
        self,
        rhs: VideoEncodeRgbRangeCompressionFlagsVALVE,
    ) -> VideoEncodeRgbRangeCompressionFlagsVALVE {
        VideoEncodeRgbRangeCompressionFlagsVALVE(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeRgbRangeCompressionFlagBitsVALVE>
    for VideoEncodeRgbRangeCompressionFlagsVALVE
{
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeRgbRangeCompressionFlagBitsVALVE) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeRgbRangeCompressionFlagBitsVALVE>
    for VideoEncodeRgbRangeCompressionFlagsVALVE
{
    fn bitor_assign(&mut self, rhs: VideoEncodeRgbRangeCompressionFlagBitsVALVE) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeUsageFlagBitsKHR(pub u32);

impl VideoEncodeUsageFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const TRANSCODING_KHR: Self = Self(1u32);
    pub const STREAMING_KHR: Self = Self(2u32);
    pub const RECORDING_KHR: Self = Self(4u32);
    pub const CONFERENCING_KHR: Self = Self(8u32);
}

impl From<VideoEncodeUsageFlagBitsKHR> for VideoEncodeUsageFlagsKHR {
    fn from(value: VideoEncodeUsageFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoEncodeUsageFlagBitsKHR {
    type Output = VideoEncodeUsageFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoEncodeUsageFlagsKHR {
        VideoEncodeUsageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeUsageFlagsKHR> for VideoEncodeUsageFlagBitsKHR {
    type Output = VideoEncodeUsageFlagsKHR;
    fn bitor(self, rhs: VideoEncodeUsageFlagsKHR) -> VideoEncodeUsageFlagsKHR {
        VideoEncodeUsageFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoEncodeUsageFlagBitsKHR> for VideoEncodeUsageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoEncodeUsageFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoEncodeUsageFlagBitsKHR> for VideoEncodeUsageFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoEncodeUsageFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoSessionCreateFlagBitsKHR(pub u32);

impl VideoSessionCreateFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_CONTENT_KHR: Self = Self(1u32);
    pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(2u32);
    pub const INLINE_QUERIES_KHR: Self = Self(4u32);
    pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(8u32);
    pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(16u32);
    pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(32u32);
}

impl From<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn from(value: VideoSessionCreateFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoSessionCreateFlagBitsKHR {
    type Output = VideoSessionCreateFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoSessionCreateFlagsKHR {
        VideoSessionCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoSessionCreateFlagsKHR> for VideoSessionCreateFlagBitsKHR {
    type Output = VideoSessionCreateFlagsKHR;
    fn bitor(self, rhs: VideoSessionCreateFlagsKHR) -> VideoSessionCreateFlagsKHR {
        VideoSessionCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: VideoSessionCreateFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoSessionCreateFlagBitsKHR> for VideoSessionCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: VideoSessionCreateFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoSessionParametersCreateFlagBitsKHR(pub u32);

impl VideoSessionParametersCreateFlagBitsKHR {
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1u32);
}

impl From<VideoSessionParametersCreateFlagBitsKHR> for VideoSessionParametersCreateFlagsKHR {
    fn from(value: VideoSessionParametersCreateFlagBitsKHR) -> Self {
        Self(value.0)
    }
}

impl core::ops::BitOr for VideoSessionParametersCreateFlagBitsKHR {
    type Output = VideoSessionParametersCreateFlagsKHR;
    fn bitor(self, rhs: Self) -> VideoSessionParametersCreateFlagsKHR {
        VideoSessionParametersCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoSessionParametersCreateFlagsKHR>
    for VideoSessionParametersCreateFlagBitsKHR
{
    type Output = VideoSessionParametersCreateFlagsKHR;
    fn bitor(
        self,
        rhs: VideoSessionParametersCreateFlagsKHR,
    ) -> VideoSessionParametersCreateFlagsKHR {
        VideoSessionParametersCreateFlagsKHR(self.0 | rhs.0)
    }
}

impl core::ops::BitOr<VideoSessionParametersCreateFlagBitsKHR>
    for VideoSessionParametersCreateFlagsKHR
{
    type Output = Self;
    fn bitor(self, rhs: VideoSessionParametersCreateFlagBitsKHR) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign<VideoSessionParametersCreateFlagBitsKHR>
    for VideoSessionParametersCreateFlagsKHR
{
    fn bitor_assign(&mut self, rhs: VideoSessionParametersCreateFlagBitsKHR) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureCreateFlagsKHR(pub u32);

impl AccelerationStructureCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1u32);
    pub const MOTION_NV: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccelerationStructureCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureMotionInfoFlagsNV(pub u32);

impl AccelerationStructureMotionInfoFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccelerationStructureMotionInfoFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccelerationStructureMotionInfoFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccelerationStructureMotionInfoFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccelerationStructureMotionInstanceFlagsNV(pub u32);

impl AccelerationStructureMotionInstanceFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccelerationStructureMotionInstanceFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccelerationStructureMotionInstanceFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccelerationStructureMotionInstanceFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlags(pub u32);

impl AccessFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const INDIRECT_COMMAND_READ: Self = Self(1u32);
    pub const INDEX_READ: Self = Self(2u32);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(4u32);
    pub const UNIFORM_READ: Self = Self(8u32);
    pub const INPUT_ATTACHMENT_READ: Self = Self(16u32);
    pub const SHADER_READ: Self = Self(32u32);
    pub const SHADER_WRITE: Self = Self(64u32);
    pub const COLOR_ATTACHMENT_READ: Self = Self(128u32);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(256u32);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512u32);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024u32);
    pub const TRANSFER_READ: Self = Self(2048u32);
    pub const TRANSFER_WRITE: Self = Self(4096u32);
    pub const HOST_READ: Self = Self(8192u32);
    pub const HOST_WRITE: Self = Self(16384u32);
    pub const MEMORY_READ: Self = Self(32768u32);
    pub const MEMORY_WRITE: Self = Self(65536u32);
    pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(131072u32);
    pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(262144u32);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288u32);
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576u32);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152u32);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608u32);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216u32);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccessFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccessFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccessFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlags2(pub u64);

impl AccessFlags2 {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const ACCESS_2_NONE: Self = Self(0u64);
    pub const ACCESS_2_INDIRECT_COMMAND_READ: Self = Self(1u64);
    pub const ACCESS_2_INDEX_READ: Self = Self(2u64);
    pub const ACCESS_2_VERTEX_ATTRIBUTE_READ: Self = Self(4u64);
    pub const ACCESS_2_UNIFORM_READ: Self = Self(8u64);
    pub const ACCESS_2_INPUT_ATTACHMENT_READ: Self = Self(16u64);
    pub const ACCESS_2_SHADER_READ: Self = Self(32u64);
    pub const ACCESS_2_SHADER_WRITE: Self = Self(64u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_READ: Self = Self(128u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_WRITE: Self = Self(256u64);
    pub const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(512u64);
    pub const ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1024u64);
    pub const ACCESS_2_TRANSFER_READ: Self = Self(2048u64);
    pub const ACCESS_2_TRANSFER_WRITE: Self = Self(4096u64);
    pub const ACCESS_2_HOST_READ: Self = Self(8192u64);
    pub const ACCESS_2_HOST_WRITE: Self = Self(16384u64);
    pub const ACCESS_2_MEMORY_READ: Self = Self(32768u64);
    pub const ACCESS_2_MEMORY_WRITE: Self = Self(65536u64);
    pub const ACCESS_2_COMMAND_PREPROCESS_READ_EXT: Self = Self(131072u64);
    pub const ACCESS_2_COMMAND_PREPROCESS_WRITE_EXT: Self = Self(262144u64);
    pub const ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(524288u64);
    pub const ACCESS_2_CONDITIONAL_RENDERING_READ_EXT: Self = Self(1048576u64);
    pub const ACCESS_2_ACCELERATION_STRUCTURE_READ_KHR: Self = Self(2097152u64);
    pub const ACCESS_2_ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(4194304u64);
    pub const ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(8388608u64);
    pub const ACCESS_2_FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(16777216u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(33554432u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(67108864u64);
    pub const ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(134217728u64);
    pub const ACCESS_2_SHADER_SAMPLED_READ: Self = Self(4294967296u64);
    pub const ACCESS_2_SHADER_STORAGE_READ: Self = Self(8589934592u64);
    pub const ACCESS_2_SHADER_STORAGE_WRITE: Self = Self(17179869184u64);
    pub const ACCESS_2_VIDEO_DECODE_READ_KHR: Self = Self(34359738368u64);
    pub const ACCESS_2_VIDEO_DECODE_WRITE_KHR: Self = Self(68719476736u64);
    pub const ACCESS_2_VIDEO_ENCODE_READ_KHR: Self = Self(137438953472u64);
    pub const ACCESS_2_VIDEO_ENCODE_WRITE_KHR: Self = Self(274877906944u64);
    pub const ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI: Self = Self(549755813888u64);
    pub const ACCESS_2_SHADER_BINDING_TABLE_READ_KHR: Self = Self(1099511627776u64);
    pub const ACCESS_2_DESCRIPTOR_BUFFER_READ_EXT: Self = Self(2199023255552u64);
    pub const ACCESS_2_OPTICAL_FLOW_READ_NV: Self = Self(4398046511104u64);
    pub const ACCESS_2_OPTICAL_FLOW_WRITE_NV: Self = Self(8796093022208u64);
    pub const ACCESS_2_MICROMAP_READ_EXT: Self = Self(17592186044416u64);
    pub const ACCESS_2_MICROMAP_WRITE_EXT: Self = Self(35184372088832u64);
    pub const ACCESS_2_DATA_GRAPH_READ_ARM: Self = Self(140737488355328u64);
    pub const ACCESS_2_DATA_GRAPH_WRITE_ARM: Self = Self(281474976710656u64);
    pub const ACCESS_2_SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(2251799813685248u64);
    pub const ACCESS_2_SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(4503599627370496u64);
    pub const ACCESS_2_MEMORY_DECOMPRESSION_READ_EXT: Self = Self(36028797018963968u64);
    pub const ACCESS_2_MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(72057594037927936u64);
    pub const ACCESS_2_SAMPLER_HEAP_READ_EXT: Self = Self(144115188075855872u64);
    pub const ACCESS_2_RESOURCE_HEAP_READ_EXT: Self = Self(288230376151711744u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccessFlags2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccessFlags2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccessFlags2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type AccessFlags2KHR = AccessFlags2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AccessFlags3KHR(pub u64);

impl AccessFlags3KHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const ACCESS_3_NONE_KHR: Self = Self(0u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AccessFlags3KHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AccessFlags3KHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AccessFlags3KHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AcquireProfilingLockFlagsKHR(pub u32);

impl AcquireProfilingLockFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AcquireProfilingLockFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AcquireProfilingLockFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AddressCommandFlagsKHR(pub u32);

impl AddressCommandFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_KHR: Self = Self(1u32);
    pub const FULLY_BOUND_KHR: Self = Self(2u32);
    pub const STORAGE_BUFFER_USAGE_KHR: Self = Self(4u32);
    pub const UNKNOWN_STORAGE_BUFFER_USAGE_KHR: Self = Self(8u32);
    pub const TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(16u32);
    pub const UNKNOWN_TRANSFORM_FEEDBACK_BUFFER_USAGE_KHR: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AddressCommandFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AddressCommandFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AddressCommandFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AddressCopyFlagsKHR(pub u32);

impl AddressCopyFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL_KHR: Self = Self(1u32);
    pub const SPARSE_KHR: Self = Self(2u32);
    pub const PROTECTED_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AddressCopyFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AddressCopyFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AddressCopyFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "android")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AndroidSurfaceCreateFlagsKHR(pub u32);

#[cfg(feature = "android")]
impl AndroidSurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "android")]
impl core::ops::BitOr for AndroidSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "android")]
impl core::ops::BitOrAssign for AndroidSurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "android")]
impl core::ops::BitAnd for AndroidSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct AttachmentDescriptionFlags(pub u32);

impl AttachmentDescriptionFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MAY_ALIAS: Self = Self(1u32);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for AttachmentDescriptionFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for AttachmentDescriptionFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for AttachmentDescriptionFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferCreateFlags(pub u32);

impl BufferCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPARSE_BINDING: Self = Self(1u32);
    pub const SPARSE_RESIDENCY: Self = Self(2u32);
    pub const SPARSE_ALIASED: Self = Self(4u32);
    pub const PROTECTED: Self = Self(8u32);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(16u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(32u32);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(64u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BufferCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BufferCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BufferCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferUsageFlags(pub u32);

impl BufferUsageFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFER_SRC: Self = Self(1u32);
    pub const TRANSFER_DST: Self = Self(2u32);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4u32);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(8u32);
    pub const UNIFORM_BUFFER: Self = Self(16u32);
    pub const STORAGE_BUFFER: Self = Self(32u32);
    pub const INDEX_BUFFER: Self = Self(64u32);
    pub const VERTEX_BUFFER: Self = Self(128u32);
    pub const INDIRECT_BUFFER: Self = Self(256u32);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(512u32);
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(1024u32);
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(2048u32);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(4096u32);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(8192u32);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(16384u32);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(32768u32);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(65536u32);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(131072u32);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(524288u32);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1048576u32);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(2097152u32);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(4194304u32);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(8388608u32);
    pub const MICROMAP_STORAGE_EXT: Self = Self(16777216u32);
    #[cfg(feature = "beta")]
    pub const EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432u32);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(67108864u32);
    pub const TILE_MEMORY_QCOM: Self = Self(134217728u32);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(268435456u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BufferUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BufferUsageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BufferUsageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferUsageFlags2(pub u64);

impl BufferUsageFlags2 {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const BUFFER_USAGE_2_TRANSFER_SRC: Self = Self(1u64);
    pub const BUFFER_USAGE_2_TRANSFER_DST: Self = Self(2u64);
    pub const BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER: Self = Self(4u64);
    pub const BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER: Self = Self(8u64);
    pub const BUFFER_USAGE_2_UNIFORM_BUFFER: Self = Self(16u64);
    pub const BUFFER_USAGE_2_STORAGE_BUFFER: Self = Self(32u64);
    pub const BUFFER_USAGE_2_INDEX_BUFFER: Self = Self(64u64);
    pub const BUFFER_USAGE_2_VERTEX_BUFFER: Self = Self(128u64);
    pub const BUFFER_USAGE_2_INDIRECT_BUFFER: Self = Self(256u64);
    pub const BUFFER_USAGE_2_CONDITIONAL_RENDERING_EXT: Self = Self(512u64);
    pub const BUFFER_USAGE_2_SHADER_BINDING_TABLE_KHR: Self = Self(1024u64);
    pub const BUFFER_USAGE_2_TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(2048u64);
    pub const BUFFER_USAGE_2_TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(4096u64);
    pub const BUFFER_USAGE_2_VIDEO_DECODE_SRC_KHR: Self = Self(8192u64);
    pub const BUFFER_USAGE_2_VIDEO_DECODE_DST_KHR: Self = Self(16384u64);
    pub const BUFFER_USAGE_2_VIDEO_ENCODE_DST_KHR: Self = Self(32768u64);
    pub const BUFFER_USAGE_2_VIDEO_ENCODE_SRC_KHR: Self = Self(65536u64);
    pub const BUFFER_USAGE_2_SHADER_DEVICE_ADDRESS: Self = Self(131072u64);
    pub const BUFFER_USAGE_2_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self =
        Self(524288u64);
    pub const BUFFER_USAGE_2_ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1048576u64);
    pub const BUFFER_USAGE_2_SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(2097152u64);
    pub const BUFFER_USAGE_2_RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(4194304u64);
    pub const BUFFER_USAGE_2_MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(8388608u64);
    pub const BUFFER_USAGE_2_MICROMAP_STORAGE_EXT: Self = Self(16777216u64);
    #[cfg(feature = "beta")]
    pub const BUFFER_USAGE_2_EXECUTION_GRAPH_SCRATCH_BIT_AMDX: Self = Self(33554432u64);
    pub const BUFFER_USAGE_2_PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(67108864u64);
    pub const BUFFER_USAGE_2_TILE_MEMORY_QCOM: Self = Self(134217728u64);
    pub const BUFFER_USAGE_2_DESCRIPTOR_HEAP_EXT: Self = Self(268435456u64);
    pub const BUFFER_USAGE_2_DATA_GRAPH_FOREIGN_DESCRIPTOR_ARM: Self = Self(536870912u64);
    pub const BUFFER_USAGE_2_PREPROCESS_BUFFER_EXT: Self = Self(2147483648u64);
    pub const BUFFER_USAGE_2_MEMORY_DECOMPRESSION_EXT: Self = Self(4294967296u64);
    #[cfg(feature = "beta")]
    pub const BUFFER_USAGE_2_COMPRESSED_DATA_DGF1_BIT_AMDX: Self = Self(8589934592u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BufferUsageFlags2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BufferUsageFlags2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BufferUsageFlags2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type BufferUsageFlags2KHR = BufferUsageFlags2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BufferViewCreateFlags(pub u32);

impl BufferViewCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BufferViewCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BufferViewCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BufferViewCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildAccelerationStructureFlagsKHR(pub u32);

impl BuildAccelerationStructureFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_UPDATE_KHR: Self = Self(1u32);
    pub const ALLOW_COMPACTION_KHR: Self = Self(2u32);
    pub const PREFER_FAST_TRACE_KHR: Self = Self(4u32);
    pub const PREFER_FAST_BUILD_KHR: Self = Self(8u32);
    pub const LOW_MEMORY_KHR: Self = Self(16u32);
    pub const MOTION_NV: Self = Self(32u32);
    pub const ALLOW_OPACITY_MICROMAP_UPDATE_EXT: Self = Self(64u32);
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(128u32);
    pub const ALLOW_OPACITY_MICROMAP_DATA_UPDATE_EXT: Self = Self(256u32);
    #[cfg(feature = "beta")]
    pub const ALLOW_DISPLACEMENT_MICROMAP_UPDATE_NV: Self = Self(512u32);
    pub const ALLOW_DATA_ACCESS_KHR: Self = Self(2048u32);
    pub const ALLOW_CLUSTER_OPACITY_MICROMAPS_NV: Self = Self(4096u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BuildAccelerationStructureFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type BuildAccelerationStructureFlagsNV = BuildAccelerationStructureFlagsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct BuildMicromapFlagsEXT(pub u32);

impl BuildMicromapFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PREFER_FAST_TRACE_EXT: Self = Self(1u32);
    pub const PREFER_FAST_BUILD_EXT: Self = Self(2u32);
    pub const ALLOW_COMPACTION_EXT: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for BuildMicromapFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for BuildMicromapFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for BuildMicromapFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureAddressResolutionFlagsNV(pub u32);

impl ClusterAccelerationStructureAddressResolutionFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_NV: Self = Self(0u32);
    pub const INDIRECTED_DST_IMPLICIT_DATA_NV: Self = Self(1u32);
    pub const INDIRECTED_SCRATCH_DATA_NV: Self = Self(2u32);
    pub const INDIRECTED_DST_ADDRESS_ARRAY_NV: Self = Self(4u32);
    pub const INDIRECTED_DST_SIZES_ARRAY_NV: Self = Self(8u32);
    pub const INDIRECTED_SRC_INFOS_ARRAY_NV: Self = Self(16u32);
    pub const INDIRECTED_SRC_INFOS_COUNT_NV: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureAddressResolutionFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ClusterAccelerationStructureAddressResolutionFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ClusterAccelerationStructureAddressResolutionFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureClusterFlagsNV(pub u32);

impl ClusterAccelerationStructureClusterFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_DISABLE_OPACITY_MICROMAPS_NV: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureClusterFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ClusterAccelerationStructureClusterFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ClusterAccelerationStructureClusterFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureGeometryFlagsNV(pub u32);

impl ClusterAccelerationStructureGeometryFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CULL_DISABLE_NV: Self = Self(1u32);
    pub const NO_DUPLICATE_ANYHIT_INVOCATION_NV: Self = Self(2u32);
    pub const OPAQUE_NV: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureGeometryFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ClusterAccelerationStructureGeometryFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ClusterAccelerationStructureGeometryFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ClusterAccelerationStructureIndexFormatFlagsNV(pub u32);

impl ClusterAccelerationStructureIndexFormatFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _8BIT_NV: Self = Self(1u32);
    pub const _16BIT_NV: Self = Self(2u32);
    pub const _32BIT_NV: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ClusterAccelerationStructureIndexFormatFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ClusterAccelerationStructureIndexFormatFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ClusterAccelerationStructureIndexFormatFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ColorComponentFlags(pub u32);

impl ColorComponentFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const R: Self = Self(1u32);
    pub const G: Self = Self(2u32);
    pub const B: Self = Self(4u32);
    pub const A: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ColorComponentFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ColorComponentFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ColorComponentFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBufferResetFlags(pub u32);

impl CommandBufferResetFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RELEASE_RESOURCES: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CommandBufferResetFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CommandBufferResetFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CommandBufferResetFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBufferUsageFlags(pub u32);

impl CommandBufferUsageFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ONE_TIME_SUBMIT: Self = Self(1u32);
    pub const RENDER_PASS_CONTINUE: Self = Self(2u32);
    pub const SIMULTANEOUS_USE: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CommandBufferUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CommandBufferUsageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CommandBufferUsageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandPoolCreateFlags(pub u32);

impl CommandPoolCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSIENT: Self = Self(1u32);
    pub const RESET_COMMAND_BUFFER: Self = Self(2u32);
    pub const PROTECTED: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CommandPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CommandPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CommandPoolCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandPoolResetFlags(pub u32);

impl CommandPoolResetFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RELEASE_RESOURCES: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CommandPoolResetFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CommandPoolResetFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CommandPoolResetFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandPoolTrimFlags(pub u32);

impl CommandPoolTrimFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CommandPoolTrimFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CommandPoolTrimFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CommandPoolTrimFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type CommandPoolTrimFlagsKHR = CommandPoolTrimFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CompositeAlphaFlagsKHR(pub u32);

impl CompositeAlphaFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const PRE_MULTIPLIED_KHR: Self = Self(2u32);
    pub const POST_MULTIPLIED_KHR: Self = Self(4u32);
    pub const INHERIT_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CompositeAlphaFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CompositeAlphaFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CompositeAlphaFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ConditionalRenderingFlagsEXT(pub u32);

impl ConditionalRenderingFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVERTED_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ConditionalRenderingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ConditionalRenderingFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ConditionalRenderingFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct CullModeFlags(pub u32);

impl CullModeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const FRONT: Self = Self(1u32);
    pub const BACK: Self = Self(2u32);
    pub const FRONT_AND_BACK: Self = Self(3u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for CullModeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for CullModeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for CullModeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowCreateFlagsARM(pub u32);

impl DataGraphOpticalFlowCreateFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_HINT_ARM: Self = Self(1u32);
    pub const ENABLE_COST_ARM: Self = Self(2u32);
    pub const RESERVED_30_ARM: Self = Self(1073741824u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphOpticalFlowCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphOpticalFlowCreateFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowExecuteFlagsARM(pub u32);

impl DataGraphOpticalFlowExecuteFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_TEMPORAL_HINTS_ARM: Self = Self(1u32);
    pub const INPUT_UNCHANGED_ARM: Self = Self(2u32);
    pub const REFERENCE_UNCHANGED_ARM: Self = Self(4u32);
    pub const INPUT_IS_PREVIOUS_REFERENCE_ARM: Self = Self(8u32);
    pub const REFERENCE_IS_PREVIOUS_INPUT_ARM: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowExecuteFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphOpticalFlowExecuteFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphOpticalFlowExecuteFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowGridSizeFlagsARM(pub u32);

impl DataGraphOpticalFlowGridSizeFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_ARM: Self = Self(0u32);
    pub const _1X1_ARM: Self = Self(1u32);
    pub const _2X2_ARM: Self = Self(2u32);
    pub const _4X4_ARM: Self = Self(4u32);
    pub const _8X8_ARM: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowGridSizeFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphOpticalFlowGridSizeFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphOpticalFlowGridSizeFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphOpticalFlowImageUsageFlagsARM(pub u32);

impl DataGraphOpticalFlowImageUsageFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_ARM: Self = Self(0u32);
    pub const INPUT_ARM: Self = Self(1u32);
    pub const OUTPUT_ARM: Self = Self(2u32);
    pub const HINT_ARM: Self = Self(4u32);
    pub const COST_ARM: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphOpticalFlowImageUsageFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphOpticalFlowImageUsageFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphOpticalFlowImageUsageFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineDispatchFlagsARM(pub u64);

impl DataGraphPipelineDispatchFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphPipelineDispatchFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphPipelineDispatchFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphPipelineDispatchFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphPipelineSessionCreateFlagsARM(pub u64);

impl DataGraphPipelineSessionCreateFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PROTECTED_ARM: Self = Self(1u64);
    pub const OPTICAL_FLOW_CACHE_ARM: Self = Self(2u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphPipelineSessionCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphPipelineSessionCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphPipelineSessionCreateFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DataGraphTOSAQualityFlagsARM(pub u32);

impl DataGraphTOSAQualityFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ACCELERATED_ARM: Self = Self(1u32);
    pub const CONFORMANT_ARM: Self = Self(2u32);
    pub const EXPERIMENTAL_ARM: Self = Self(4u32);
    pub const DEPRECATED_ARM: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DataGraphTOSAQualityFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DataGraphTOSAQualityFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DataGraphTOSAQualityFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugReportFlagsEXT(pub u32);

impl DebugReportFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INFORMATION_EXT: Self = Self(1u32);
    pub const WARNING_EXT: Self = Self(2u32);
    pub const PERFORMANCE_WARNING_EXT: Self = Self(4u32);
    pub const ERROR_EXT: Self = Self(8u32);
    pub const DEBUG_EXT: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DebugReportFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DebugReportFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DebugReportFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub u32);

impl DebugUtilsMessageSeverityFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERBOSE: Self = Self(1u32);
    pub const INFO: Self = Self(16u32);
    pub const WARNING: Self = Self(256u32);
    pub const ERROR: Self = Self(4096u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DebugUtilsMessageSeverityFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub u32);

impl DebugUtilsMessageTypeFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const GENERAL_EXT: Self = Self(1u32);
    pub const VALIDATION_EXT: Self = Self(2u32);
    pub const PERFORMANCE_EXT: Self = Self(4u32);
    pub const DEVICE_ADDRESS_BINDING_EXT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DebugUtilsMessageTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DebugUtilsMessageTypeFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessengerCallbackDataFlagsEXT(pub u32);

impl DebugUtilsMessengerCallbackDataFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DebugUtilsMessengerCallbackDataFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DebugUtilsMessengerCallbackDataFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DebugUtilsMessengerCallbackDataFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DebugUtilsMessengerCreateFlagsEXT(pub u32);

impl DebugUtilsMessengerCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DebugUtilsMessengerCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DebugUtilsMessengerCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DebugUtilsMessengerCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DependencyFlags(pub u32);

impl DependencyFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BY_REGION: Self = Self(1u32);
    pub const VIEW_LOCAL: Self = Self(2u32);
    pub const DEVICE_GROUP: Self = Self(4u32);
    pub const FEEDBACK_LOOP_EXT: Self = Self(8u32);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self = Self(32u32);
    pub const ASYMMETRIC_EVENT_KHR: Self = Self(64u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DependencyFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DependencyFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DependencyFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorBindingFlags(pub u32);

impl DescriptorBindingFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UPDATE_AFTER_BIND: Self = Self(1u32);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(2u32);
    pub const PARTIALLY_BOUND: Self = Self(4u32);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DescriptorBindingFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DescriptorBindingFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DescriptorBindingFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type DescriptorBindingFlagsEXT = DescriptorBindingFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorPoolCreateFlags(pub u32);

impl DescriptorPoolCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FREE_DESCRIPTOR_SET: Self = Self(1u32);
    pub const UPDATE_AFTER_BIND: Self = Self(2u32);
    pub const HOST_ONLY_EXT: Self = Self(4u32);
    pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(8u32);
    pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DescriptorPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DescriptorPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DescriptorPoolCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorPoolResetFlags(pub u32);

impl DescriptorPoolResetFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DescriptorPoolResetFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DescriptorPoolResetFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DescriptorPoolResetFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorSetLayoutCreateFlags(pub u32);

impl DescriptorSetLayoutCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PUSH_DESCRIPTOR: Self = Self(1u32);
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(2u32);
    pub const HOST_ONLY_POOL_EXT: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(16u32);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(32u32);
    pub const PER_STAGE_NV: Self = Self(64u32);
    pub const INDIRECT_BINDABLE_NV: Self = Self(128u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DescriptorSetLayoutCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DescriptorSetLayoutCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DescriptorSetLayoutCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DescriptorUpdateTemplateCreateFlags(pub u32);

impl DescriptorUpdateTemplateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DescriptorUpdateTemplateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DescriptorUpdateTemplateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DescriptorUpdateTemplateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type DescriptorUpdateTemplateCreateFlagsKHR = DescriptorUpdateTemplateCreateFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceAddressBindingFlagsEXT(pub u32);

impl DeviceAddressBindingFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INTERNAL_OBJECT_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceAddressBindingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceAddressBindingFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceAddressBindingFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceCreateFlags(pub u32);

impl DeviceCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceDiagnosticsConfigFlagsNV(pub u32);

impl DeviceDiagnosticsConfigFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_SHADER_DEBUG_INFO_NV: Self = Self(1u32);
    pub const ENABLE_RESOURCE_TRACKING_NV: Self = Self(2u32);
    pub const ENABLE_AUTOMATIC_CHECKPOINTS_NV: Self = Self(4u32);
    pub const ENABLE_SHADER_ERROR_REPORTING_NV: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceDiagnosticsConfigFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceDiagnosticsConfigFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceFaultFlagsKHR(pub u32);

impl DeviceFaultFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_DEVICE_LOST_KHR: Self = Self(1u32);
    pub const FLAG_MEMORY_ADDRESS_KHR: Self = Self(2u32);
    pub const FLAG_INSTRUCTION_ADDRESS_KHR: Self = Self(4u32);
    pub const FLAG_VENDOR_KHR: Self = Self(8u32);
    pub const FLAG_WATCHDOG_TIMEOUT_KHR: Self = Self(16u32);
    pub const FLAG_OVERFLOW_KHR: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceFaultFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceFaultFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceFaultFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceGroupPresentModeFlagsKHR(pub u32);

impl DeviceGroupPresentModeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const LOCAL_KHR: Self = Self(1u32);
    pub const REMOTE_KHR: Self = Self(2u32);
    pub const SUM_KHR: Self = Self(4u32);
    pub const LOCAL_MULTI_DEVICE_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceGroupPresentModeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceGroupPresentModeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceMemoryReportFlagsEXT(pub u32);

impl DeviceMemoryReportFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceMemoryReportFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceMemoryReportFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceMemoryReportFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DeviceQueueCreateFlags(pub u32);

impl DeviceQueueCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED: Self = Self(1u32);
    pub const INTERNALLY_SYNCHRONIZED_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DeviceQueueCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DeviceQueueCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DeviceQueueCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DirectDriverLoadingFlagsLUNARG(pub u32);

impl DirectDriverLoadingFlagsLUNARG {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DirectDriverLoadingFlagsLUNARG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DirectDriverLoadingFlagsLUNARG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DirectDriverLoadingFlagsLUNARG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "directfb")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DirectFBSurfaceCreateFlagsEXT(pub u32);

#[cfg(feature = "directfb")]
impl DirectFBSurfaceCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "directfb")]
impl core::ops::BitOr for DirectFBSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "directfb")]
impl core::ops::BitOrAssign for DirectFBSurfaceCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "directfb")]
impl core::ops::BitAnd for DirectFBSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplayModeCreateFlagsKHR(pub u32);

impl DisplayModeCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DisplayModeCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DisplayModeCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DisplayModeCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplayPlaneAlphaFlagsKHR(pub u32);

impl DisplayPlaneAlphaFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const GLOBAL_KHR: Self = Self(2u32);
    pub const PER_PIXEL_KHR: Self = Self(4u32);
    pub const PER_PIXEL_PREMULTIPLIED_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DisplayPlaneAlphaFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DisplayPlaneAlphaFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct DisplaySurfaceCreateFlagsKHR(pub u32);

impl DisplaySurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for DisplaySurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for DisplaySurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for DisplaySurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct EventCreateFlags(pub u32);

impl EventCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ONLY: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for EventCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for EventCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for EventCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "metal")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExportMetalObjectTypeFlagsEXT(pub u32);

#[cfg(feature = "metal")]
impl ExportMetalObjectTypeFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const METAL_DEVICE_EXT: Self = Self(1u32);
    pub const METAL_COMMAND_QUEUE_EXT: Self = Self(2u32);
    pub const METAL_BUFFER_EXT: Self = Self(4u32);
    pub const METAL_TEXTURE_EXT: Self = Self(8u32);
    pub const METAL_IOSURFACE_EXT: Self = Self(16u32);
    pub const METAL_SHARED_EVENT_EXT: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOr for ExportMetalObjectTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOrAssign for ExportMetalObjectTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitAnd for ExportMetalObjectTypeFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalFenceFeatureFlags(pub u32);

impl ExternalFenceFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPORTABLE: Self = Self(1u32);
    pub const IMPORTABLE: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalFenceFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalFenceFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalFenceFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalFenceFeatureFlagsKHR = ExternalFenceFeatureFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalFenceHandleTypeFlags(pub u32);

impl ExternalFenceHandleTypeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const SYNC_FD: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalFenceHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalFenceHandleTypeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalFenceHandleTypeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalFenceHandleTypeFlagsKHR = ExternalFenceHandleTypeFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryFeatureFlags(pub u32);

impl ExternalMemoryFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEDICATED_ONLY: Self = Self(1u32);
    pub const EXPORTABLE: Self = Self(2u32);
    pub const IMPORTABLE: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalMemoryFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalMemoryFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalMemoryFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalMemoryFeatureFlagsKHR = ExternalMemoryFeatureFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryFeatureFlagsNV(pub u32);

impl ExternalMemoryFeatureFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEDICATED_ONLY_NV: Self = Self(1u32);
    pub const EXPORTABLE_NV: Self = Self(2u32);
    pub const IMPORTABLE_NV: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalMemoryFeatureFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalMemoryFeatureFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryHandleTypeFlags(pub u32);

impl ExternalMemoryHandleTypeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const D3D11_TEXTURE: Self = Self(8u32);
    pub const D3D11_TEXTURE_KMT: Self = Self(16u32);
    pub const D3D12_HEAP: Self = Self(32u32);
    pub const D3D12_RESOURCE: Self = Self(64u32);
    pub const HOST_ALLOCATION_EXT: Self = Self(128u32);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(256u32);
    pub const DMA_BUF_EXT: Self = Self(512u32);
    pub const ANDROID_HARDWARE_BUFFER_BIT_ANDROID: Self = Self(1024u32);
    pub const ZIRCON_VMO_BIT_FUCHSIA: Self = Self(2048u32);
    pub const RDMA_ADDRESS_NV: Self = Self(4096u32);
    pub const SCREEN_BUFFER_BIT_QNX: Self = Self(16384u32);
    pub const OH_NATIVE_BUFFER_BIT_OHOS: Self = Self(32768u32);
    pub const MTLBUFFER_EXT: Self = Self(65536u32);
    pub const MTLTEXTURE_EXT: Self = Self(131072u32);
    pub const MTLHEAP_EXT: Self = Self(262144u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalMemoryHandleTypeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalMemoryHandleTypeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalMemoryHandleTypeFlagsKHR = ExternalMemoryHandleTypeFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub u32);

impl ExternalMemoryHandleTypeFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_WIN32_NV: Self = Self(1u32);
    pub const OPAQUE_WIN32_KMT_NV: Self = Self(2u32);
    pub const D3D11_IMAGE_NV: Self = Self(4u32);
    pub const D3D11_IMAGE_KMT_NV: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalMemoryHandleTypeFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalMemoryHandleTypeFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalSemaphoreFeatureFlags(pub u32);

impl ExternalSemaphoreFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPORTABLE: Self = Self(1u32);
    pub const IMPORTABLE: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalSemaphoreFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalSemaphoreFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalSemaphoreFeatureFlagsKHR = ExternalSemaphoreFeatureFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ExternalSemaphoreHandleTypeFlags(pub u32);

impl ExternalSemaphoreHandleTypeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_FD: Self = Self(1u32);
    pub const OPAQUE_WIN32: Self = Self(2u32);
    pub const OPAQUE_WIN32_KMT: Self = Self(4u32);
    pub const D3D12_FENCE: Self = Self(8u32);
    pub const SYNC_FD: Self = Self(16u32);
    pub const ZIRCON_EVENT_BIT_FUCHSIA: Self = Self(128u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ExternalSemaphoreHandleTypeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ExternalSemaphoreHandleTypeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ExternalSemaphoreHandleTypeFlagsKHR = ExternalSemaphoreHandleTypeFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FenceCreateFlags(pub u32);

impl FenceCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SIGNALED: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FenceCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FenceCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FenceCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FenceImportFlags(pub u32);

impl FenceImportFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TEMPORARY: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FenceImportFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FenceImportFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FenceImportFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type FenceImportFlagsKHR = FenceImportFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FormatFeatureFlags(pub u32);

impl FormatFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SAMPLED_IMAGE: Self = Self(1u32);
    pub const STORAGE_IMAGE: Self = Self(2u32);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(4u32);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(8u32);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(16u32);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32u32);
    pub const VERTEX_BUFFER: Self = Self(64u32);
    pub const COLOR_ATTACHMENT: Self = Self(128u32);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(256u32);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(512u32);
    pub const BLIT_SRC: Self = Self(1024u32);
    pub const BLIT_DST: Self = Self(2048u32);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096u32);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(8192u32);
    pub const TRANSFER_SRC: Self = Self(16384u32);
    pub const TRANSFER_DST: Self = Self(32768u32);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536u32);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(131072u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(524288u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(1048576u32);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(2097152u32);
    pub const DISJOINT: Self = Self(4194304u32);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(8388608u32);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216u32);
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432u32);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(67108864u32);
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728u32);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(268435456u32);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FormatFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FormatFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FormatFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FormatFeatureFlags2(pub u64);

impl FormatFeatureFlags2 {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE: Self = Self(1u64);
    pub const FORMAT_FEATURE_2_STORAGE_IMAGE: Self = Self(2u64);
    pub const FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC: Self = Self(4u64);
    pub const FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER: Self = Self(8u64);
    pub const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER: Self = Self(16u64);
    pub const FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(32u64);
    pub const FORMAT_FEATURE_2_VERTEX_BUFFER: Self = Self(64u64);
    pub const FORMAT_FEATURE_2_COLOR_ATTACHMENT: Self = Self(128u64);
    pub const FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND: Self = Self(256u64);
    pub const FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT: Self = Self(512u64);
    pub const FORMAT_FEATURE_2_BLIT_SRC: Self = Self(1024u64);
    pub const FORMAT_FEATURE_2_BLIT_DST: Self = Self(2048u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(4096u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(8192u64);
    pub const FORMAT_FEATURE_2_TRANSFER_SRC: Self = Self(16384u64);
    pub const FORMAT_FEATURE_2_TRANSFER_DST: Self = Self(32768u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(65536u64);
    pub const FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES: Self = Self(131072u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(262144u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self =
        Self(524288u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self =
        Self(1048576u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self = Self(2097152u64);
    pub const FORMAT_FEATURE_2_DISJOINT: Self = Self(4194304u64);
    pub const FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES: Self = Self(8388608u64);
    pub const FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_EXT: Self = Self(16777216u64);
    pub const FORMAT_FEATURE_2_VIDEO_DECODE_OUTPUT_KHR: Self = Self(33554432u64);
    pub const FORMAT_FEATURE_2_VIDEO_DECODE_DPB_KHR: Self = Self(67108864u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_INPUT_KHR: Self = Self(134217728u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_DPB_KHR: Self = Self(268435456u64);
    pub const FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(536870912u64);
    pub const FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1073741824u64);
    pub const FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT: Self = Self(2147483648u64);
    pub const FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(4294967296u64);
    pub const FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(8589934592u64);
    pub const FORMAT_FEATURE_2_WEIGHT_IMAGE_QCOM: Self = Self(17179869184u64);
    pub const FORMAT_FEATURE_2_WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(34359738368u64);
    pub const FORMAT_FEATURE_2_BLOCK_MATCHING_QCOM: Self = Self(68719476736u64);
    pub const FORMAT_FEATURE_2_BOX_FILTER_SAMPLED_QCOM: Self = Self(137438953472u64);
    pub const FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_NV: Self = Self(274877906944u64);
    pub const FORMAT_FEATURE_2_TENSOR_SHADER_ARM: Self = Self(549755813888u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_IMAGE_NV: Self = Self(1099511627776u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_VECTOR_NV: Self = Self(2199023255552u64);
    pub const FORMAT_FEATURE_2_OPTICAL_FLOW_COST_NV: Self = Self(4398046511104u64);
    pub const FORMAT_FEATURE_2_TENSOR_IMAGE_ALIASING_ARM: Self = Self(8796093022208u64);
    pub const FORMAT_FEATURE_2_HOST_IMAGE_TRANSFER: Self = Self(70368744177664u64);
    pub const FORMAT_FEATURE_2_TENSOR_DATA_GRAPH_ARM: Self = Self(281474976710656u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self =
        Self(562949953421312u64);
    pub const FORMAT_FEATURE_2_VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1125899906842624u64);
    pub const FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self =
        Self(2251799813685248u64);
    pub const FORMAT_FEATURE_2_DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(4503599627370496u64);
    pub const FORMAT_FEATURE_2_DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(9007199254740992u64);
    pub const FORMAT_FEATURE_2_STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(18014398509481984u64);
    pub const FORMAT_FEATURE_2_STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self =
        Self(36028797018963968u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_IMAGE_ARM: Self = Self(72057594037927936u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_VECTOR_ARM: Self =
        Self(144115188075855872u64);
    pub const FORMAT_FEATURE_2_DATA_GRAPH_OPTICAL_FLOW_COST_ARM: Self = Self(288230376151711744u64);
    pub const FORMAT_FEATURE_2_COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(576460752303423488u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FormatFeatureFlags2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FormatFeatureFlags2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FormatFeatureFlags2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FrameBoundaryFlagsEXT(pub u32);

impl FrameBoundaryFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRAME_END_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FrameBoundaryFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FrameBoundaryFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FrameBoundaryFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct FramebufferCreateFlags(pub u32);

impl FramebufferCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const IMAGELESS: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for FramebufferCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for FramebufferCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for FramebufferCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GeometryFlagsKHR(pub u32);

impl GeometryFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const OPAQUE_KHR: Self = Self(1u32);
    pub const NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for GeometryFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for GeometryFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for GeometryFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type GeometryFlagsNV = GeometryFlagsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GeometryInstanceFlagsKHR(pub u32);

impl GeometryInstanceFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1u32);
    pub const TRIANGLE_FLIP_FACING_KHR: Self = Self(2u32);
    pub const FORCE_OPAQUE_KHR: Self = Self(4u32);
    pub const FORCE_NO_OPAQUE_KHR: Self = Self(8u32);
    pub const FORCE_OPACITY_MICROMAP_2_STATE_EXT: Self = Self(16u32);
    pub const DISABLE_OPACITY_MICROMAPS_EXT: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for GeometryInstanceFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for GeometryInstanceFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for GeometryInstanceFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type GeometryInstanceFlagsNV = GeometryInstanceFlagsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct GraphicsPipelineLibraryFlagsEXT(pub u32);

impl GraphicsPipelineLibraryFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1u32);
    pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(2u32);
    pub const FRAGMENT_SHADER_EXT: Self = Self(4u32);
    pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for GraphicsPipelineLibraryFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for GraphicsPipelineLibraryFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for GraphicsPipelineLibraryFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct HeadlessSurfaceCreateFlagsEXT(pub u32);

impl HeadlessSurfaceCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for HeadlessSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for HeadlessSurfaceCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for HeadlessSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct HostImageCopyFlags(pub u32);

impl HostImageCopyFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MEMCPY: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for HostImageCopyFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for HostImageCopyFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for HostImageCopyFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type HostImageCopyFlagsEXT = HostImageCopyFlags;

#[cfg(feature = "ios")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IOSSurfaceCreateFlagsMVK(pub u32);

#[cfg(feature = "ios")]
impl IOSSurfaceCreateFlagsMVK {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "ios")]
impl core::ops::BitOr for IOSSurfaceCreateFlagsMVK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "ios")]
impl core::ops::BitOrAssign for IOSSurfaceCreateFlagsMVK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "ios")]
impl core::ops::BitAnd for IOSSurfaceCreateFlagsMVK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageAspectFlags(pub u32);

impl ImageAspectFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const COLOR: Self = Self(1u32);
    pub const DEPTH: Self = Self(2u32);
    pub const STENCIL: Self = Self(4u32);
    pub const METADATA: Self = Self(8u32);
    pub const PLANE_0: Self = Self(16u32);
    pub const PLANE_1: Self = Self(32u32);
    pub const PLANE_2: Self = Self(64u32);
    pub const MEMORY_PLANE_0_EXT: Self = Self(128u32);
    pub const MEMORY_PLANE_1_EXT: Self = Self(256u32);
    pub const MEMORY_PLANE_2_EXT: Self = Self(512u32);
    pub const MEMORY_PLANE_3_EXT: Self = Self(1024u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageAspectFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageAspectFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageAspectFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCompressionFixedRateFlagsEXT(pub u32);

impl ImageCompressionFixedRateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_EXT: Self = Self(0u32);
    pub const _1BPC_EXT: Self = Self(1u32);
    pub const _2BPC_EXT: Self = Self(2u32);
    pub const _3BPC_EXT: Self = Self(4u32);
    pub const _4BPC_EXT: Self = Self(8u32);
    pub const _5BPC_EXT: Self = Self(16u32);
    pub const _6BPC_EXT: Self = Self(32u32);
    pub const _7BPC_EXT: Self = Self(64u32);
    pub const _8BPC_EXT: Self = Self(128u32);
    pub const _9BPC_EXT: Self = Self(256u32);
    pub const _10BPC_EXT: Self = Self(512u32);
    pub const _11BPC_EXT: Self = Self(1024u32);
    pub const _12BPC_EXT: Self = Self(2048u32);
    pub const _13BPC_EXT: Self = Self(4096u32);
    pub const _14BPC_EXT: Self = Self(8192u32);
    pub const _15BPC_EXT: Self = Self(16384u32);
    pub const _16BPC_EXT: Self = Self(32768u32);
    pub const _17BPC_EXT: Self = Self(65536u32);
    pub const _18BPC_EXT: Self = Self(131072u32);
    pub const _19BPC_EXT: Self = Self(262144u32);
    pub const _20BPC_EXT: Self = Self(524288u32);
    pub const _21BPC_EXT: Self = Self(1048576u32);
    pub const _22BPC_EXT: Self = Self(2097152u32);
    pub const _23BPC_EXT: Self = Self(4194304u32);
    pub const _24BPC_EXT: Self = Self(8388608u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageCompressionFixedRateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageCompressionFixedRateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageCompressionFixedRateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCompressionFlagsEXT(pub u32);

impl ImageCompressionFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_EXT: Self = Self(0u32);
    pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1u32);
    pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(2u32);
    pub const DISABLED_EXT: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageCompressionFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageCompressionFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageCompressionFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageConstraintsInfoFlagsFUCHSIA(pub u32);

#[cfg(feature = "fuchsia")]
impl ImageConstraintsInfoFlagsFUCHSIA {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CPU_READ_RARELY_FUCHSIA: Self = Self(1u32);
    pub const CPU_READ_OFTEN_FUCHSIA: Self = Self(2u32);
    pub const CPU_WRITE_RARELY_FUCHSIA: Self = Self(4u32);
    pub const CPU_WRITE_OFTEN_FUCHSIA: Self = Self(8u32);
    pub const PROTECTED_OPTIONAL_FUCHSIA: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOrAssign for ImageConstraintsInfoFlagsFUCHSIA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitAnd for ImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageCreateFlags(pub u32);

impl ImageCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPARSE_BINDING: Self = Self(1u32);
    pub const SPARSE_RESIDENCY: Self = Self(2u32);
    pub const SPARSE_ALIASED: Self = Self(4u32);
    pub const MUTABLE_FORMAT: Self = Self(8u32);
    pub const CUBE_COMPATIBLE: Self = Self(16u32);
    pub const _2D_ARRAY_COMPATIBLE: Self = Self(32u32);
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(64u32);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(128u32);
    pub const EXTENDED_USAGE: Self = Self(256u32);
    pub const DISJOINT: Self = Self(512u32);
    pub const ALIAS: Self = Self(1024u32);
    pub const PROTECTED: Self = Self(2048u32);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(4096u32);
    pub const CORNER_SAMPLED_NV: Self = Self(8192u32);
    pub const SUBSAMPLED_EXT: Self = Self(16384u32);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(32768u32);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self = Self(65536u32);
    pub const _2D_VIEW_COMPATIBLE_EXT: Self = Self(131072u32);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(262144u32);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1048576u32);
    pub const ALIAS_SINGLE_LAYER_DESCRIPTOR_KHR: Self = Self(4194304u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageFormatConstraintsFlagsFUCHSIA(pub u32);

#[cfg(feature = "fuchsia")]
impl ImageFormatConstraintsFlagsFUCHSIA {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr for ImageFormatConstraintsFlagsFUCHSIA {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOrAssign for ImageFormatConstraintsFlagsFUCHSIA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitAnd for ImageFormatConstraintsFlagsFUCHSIA {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImagePipeSurfaceCreateFlagsFUCHSIA(pub u32);

#[cfg(feature = "fuchsia")]
impl ImagePipeSurfaceCreateFlagsFUCHSIA {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOr for ImagePipeSurfaceCreateFlagsFUCHSIA {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitOrAssign for ImagePipeSurfaceCreateFlagsFUCHSIA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "fuchsia")]
impl core::ops::BitAnd for ImagePipeSurfaceCreateFlagsFUCHSIA {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageUsageFlags(pub u32);

impl ImageUsageFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFER_SRC: Self = Self(1u32);
    pub const TRANSFER_DST: Self = Self(2u32);
    pub const SAMPLED: Self = Self(4u32);
    pub const STORAGE: Self = Self(8u32);
    pub const COLOR_ATTACHMENT: Self = Self(16u32);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(32u32);
    pub const TRANSIENT_ATTACHMENT: Self = Self(64u32);
    pub const INPUT_ATTACHMENT: Self = Self(128u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(256u32);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(512u32);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1024u32);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(2048u32);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(4096u32);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(8192u32);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(16384u32);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(32768u32);
    pub const INVOCATION_MASK_BIT_HUAWEI: Self = Self(262144u32);
    pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(524288u32);
    pub const SAMPLE_WEIGHT_QCOM: Self = Self(1048576u32);
    pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(2097152u32);
    pub const HOST_TRANSFER: Self = Self(4194304u32);
    pub const TENSOR_ALIASING_ARM: Self = Self(8388608u32);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(33554432u32);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(67108864u32);
    pub const TILE_MEMORY_QCOM: Self = Self(134217728u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageUsageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageUsageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageUsageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ImageViewCreateFlags(pub u32);

impl ImageViewCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(1u32);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(2u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ImageViewCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ImageViewCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ImageViewCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsInputModeFlagsEXT(pub u32);

impl IndirectCommandsInputModeFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1u32);
    pub const DXGI_INDEX_BUFFER_EXT: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for IndirectCommandsInputModeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for IndirectCommandsInputModeFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for IndirectCommandsInputModeFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsLayoutUsageFlagsEXT(pub u32);

impl IndirectCommandsLayoutUsageFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPLICIT_PREPROCESS_EXT: Self = Self(1u32);
    pub const UNORDERED_SEQUENCES_EXT: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for IndirectCommandsLayoutUsageFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for IndirectCommandsLayoutUsageFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for IndirectCommandsLayoutUsageFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub u32);

impl IndirectCommandsLayoutUsageFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(1u32);
    pub const INDEXED_SEQUENCES_NV: Self = Self(2u32);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for IndirectCommandsLayoutUsageFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for IndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct IndirectStateFlagsNV(pub u32);

impl IndirectStateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_FRONTFACE_NV: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for IndirectStateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for IndirectStateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for IndirectStateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct InstanceCreateFlags(pub u32);

impl InstanceCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for InstanceCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for InstanceCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for InstanceCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "macos")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MacOSSurfaceCreateFlagsMVK(pub u32);

#[cfg(feature = "macos")]
impl MacOSSurfaceCreateFlagsMVK {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "macos")]
impl core::ops::BitOr for MacOSSurfaceCreateFlagsMVK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "macos")]
impl core::ops::BitOrAssign for MacOSSurfaceCreateFlagsMVK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "macos")]
impl core::ops::BitAnd for MacOSSurfaceCreateFlagsMVK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryAllocateFlags(pub u32);

impl MemoryAllocateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_MASK: Self = Self(1u32);
    pub const DEVICE_ADDRESS: Self = Self(2u32);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(4u32);
    pub const ZERO_INITIALIZE_EXT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryAllocateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryAllocateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryAllocateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type MemoryAllocateFlagsKHR = MemoryAllocateFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryDecompressionMethodFlagsEXT(pub u64);

impl MemoryDecompressionMethodFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const GDEFLATE_1_0_EXT: Self = Self(1u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryDecompressionMethodFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryDecompressionMethodFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryDecompressionMethodFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type MemoryDecompressionMethodFlagsNV = MemoryDecompressionMethodFlagsEXT;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryHeapFlags(pub u32);

impl MemoryHeapFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL: Self = Self(1u32);
    pub const MULTI_INSTANCE: Self = Self(2u32);
    pub const TILE_MEMORY_QCOM: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryHeapFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryHeapFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryHeapFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryMapFlags(pub u32);

impl MemoryMapFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PLACED_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryMapFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryMapFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryMapFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryPropertyFlags(pub u32);

impl MemoryPropertyFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_LOCAL: Self = Self(1u32);
    pub const HOST_VISIBLE: Self = Self(2u32);
    pub const HOST_COHERENT: Self = Self(4u32);
    pub const HOST_CACHED: Self = Self(8u32);
    pub const LAZILY_ALLOCATED: Self = Self(16u32);
    pub const PROTECTED: Self = Self(32u32);
    pub const DEVICE_COHERENT_AMD: Self = Self(64u32);
    pub const DEVICE_UNCACHED_AMD: Self = Self(128u32);
    pub const RDMA_CAPABLE_NV: Self = Self(256u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryPropertyFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryPropertyFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryPropertyFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MemoryUnmapFlags(pub u32);

impl MemoryUnmapFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESERVE_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MemoryUnmapFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MemoryUnmapFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MemoryUnmapFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type MemoryUnmapFlagsKHR = MemoryUnmapFlags;

#[cfg(feature = "metal")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MetalSurfaceCreateFlagsEXT(pub u32);

#[cfg(feature = "metal")]
impl MetalSurfaceCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOr for MetalSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitOrAssign for MetalSurfaceCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "metal")]
impl core::ops::BitAnd for MetalSurfaceCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct MicromapCreateFlagsEXT(pub u32);

impl MicromapCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for MicromapCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for MicromapCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for MicromapCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowExecuteFlagsNV(pub u32);

impl OpticalFlowExecuteFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_TEMPORAL_HINTS_NV: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for OpticalFlowExecuteFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for OpticalFlowExecuteFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for OpticalFlowExecuteFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowGridSizeFlagsNV(pub u32);

impl OpticalFlowGridSizeFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_NV: Self = Self(0u32);
    pub const _1X1_NV: Self = Self(1u32);
    pub const _2X2_NV: Self = Self(2u32);
    pub const _4X4_NV: Self = Self(4u32);
    pub const _8X8_NV: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for OpticalFlowGridSizeFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for OpticalFlowGridSizeFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for OpticalFlowGridSizeFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowSessionCreateFlagsNV(pub u32);

impl OpticalFlowSessionCreateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_HINT_NV: Self = Self(1u32);
    pub const ENABLE_COST_NV: Self = Self(2u32);
    pub const ENABLE_GLOBAL_FLOW_NV: Self = Self(4u32);
    pub const ALLOW_REGIONS_NV: Self = Self(8u32);
    pub const BOTH_DIRECTIONS_NV: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for OpticalFlowSessionCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for OpticalFlowSessionCreateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for OpticalFlowSessionCreateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct OpticalFlowUsageFlagsNV(pub u32);

impl OpticalFlowUsageFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNKNOWN_NV: Self = Self(0u32);
    pub const INPUT_NV: Self = Self(1u32);
    pub const OUTPUT_NV: Self = Self(2u32);
    pub const HINT_NV: Self = Self(4u32);
    pub const COST_NV: Self = Self(8u32);
    pub const GLOBAL_FLOW_NV: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for OpticalFlowUsageFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for OpticalFlowUsageFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for OpticalFlowUsageFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PartitionedAccelerationStructureInstanceFlagsNV(pub u32);

impl PartitionedAccelerationStructureInstanceFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FLAG_TRIANGLE_FACING_CULL_DISABLE_NV: Self = Self(1u32);
    pub const FLAG_TRIANGLE_FLIP_FACING_NV: Self = Self(2u32);
    pub const FLAG_FORCE_OPAQUE_NV: Self = Self(4u32);
    pub const FLAG_FORCE_NO_OPAQUE_NV: Self = Self(8u32);
    pub const FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PartitionedAccelerationStructureInstanceFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PartitionedAccelerationStructureInstanceFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PartitionedAccelerationStructureInstanceFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PastPresentationTimingFlagsEXT(pub u32);

impl PastPresentationTimingFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(1u32);
    pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PastPresentationTimingFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PastPresentationTimingFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PastPresentationTimingFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PeerMemoryFeatureFlags(pub u32);

impl PeerMemoryFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const COPY_SRC: Self = Self(1u32);
    pub const COPY_DST: Self = Self(2u32);
    pub const GENERIC_SRC: Self = Self(4u32);
    pub const GENERIC_DST: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PeerMemoryFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PeerMemoryFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PeerMemoryFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PeerMemoryFeatureFlagsKHR = PeerMemoryFeatureFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterDescriptionFlagsARM(pub u32);

impl PerformanceCounterDescriptionFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PerformanceCounterDescriptionFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PerformanceCounterDescriptionFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PerformanceCounterDescriptionFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PerformanceCounterDescriptionFlagsKHR(pub u32);

impl PerformanceCounterDescriptionFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PERFORMANCE_IMPACTING_KHR: Self = Self(1u32);
    pub const CONCURRENTLY_IMPACTED_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PerformanceCounterDescriptionFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PerformanceCounterDescriptionFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PhysicalDeviceSchedulingControlsFlagsARM(pub u64);

impl PhysicalDeviceSchedulingControlsFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const SHADER_CORE_COUNT_ARM: Self = Self(1u64);
    pub const DISPATCH_PARAMETERS_ARM: Self = Self(2u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PhysicalDeviceSchedulingControlsFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PhysicalDeviceSchedulingControlsFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PhysicalDeviceSchedulingControlsFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCacheCreateFlags(pub u32);

impl PipelineCacheCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1u32);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCacheCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCacheCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCacheCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineColorBlendStateCreateFlags(pub u32);

impl PipelineColorBlendStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineColorBlendStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineColorBlendStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineColorBlendStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCompilerControlFlagsAMD(pub u32);

impl PipelineCompilerControlFlagsAMD {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCompilerControlFlagsAMD {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCompilerControlFlagsAMD {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCoverageModulationStateCreateFlagsNV(pub u32);

impl PipelineCoverageModulationStateCreateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCoverageModulationStateCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCoverageModulationStateCreateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCoverageModulationStateCreateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCoverageReductionStateCreateFlagsNV(pub u32);

impl PipelineCoverageReductionStateCreateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCoverageReductionStateCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCoverageReductionStateCreateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCoverageReductionStateCreateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCoverageToColorStateCreateFlagsNV(pub u32);

impl PipelineCoverageToColorStateCreateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCoverageToColorStateCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCoverageToColorStateCreateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCoverageToColorStateCreateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreateFlags(pub u32);

impl PipelineCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DISABLE_OPTIMIZATION: Self = Self(1u32);
    pub const ALLOW_DERIVATIVES: Self = Self(2u32);
    pub const DERIVATIVE: Self = Self(4u32);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(8u32);
    pub const DISPATCH_BASE: Self = Self(16u32);
    pub const DEFER_COMPILE_NV: Self = Self(32u32);
    pub const CAPTURE_STATISTICS_KHR: Self = Self(64u32);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(128u32);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(256u32);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(512u32);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1024u32);
    pub const LIBRARY_KHR: Self = Self(2048u32);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(4096u32);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(8192u32);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(16384u32);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(32768u32);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(65536u32);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(131072u32);
    pub const INDIRECT_BINDABLE_NV: Self = Self(262144u32);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(524288u32);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1048576u32);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(2097152u32);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(4194304u32);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(8388608u32);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(16777216u32);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(33554432u32);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(67108864u32);
    pub const NO_PROTECTED_ACCESS: Self = Self(134217728u32);
    #[cfg(feature = "beta")]
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(268435456u32);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(536870912u32);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(1073741824u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreateFlags2(pub u64);

impl PipelineCreateFlags2 {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PIPELINE_CREATE_2_DISABLE_OPTIMIZATION: Self = Self(1u64);
    pub const PIPELINE_CREATE_2_ALLOW_DERIVATIVES: Self = Self(2u64);
    pub const PIPELINE_CREATE_2_DERIVATIVE: Self = Self(4u64);
    pub const PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(8u64);
    pub const PIPELINE_CREATE_2_DISPATCH_BASE: Self = Self(16u64);
    pub const PIPELINE_CREATE_2_DEFER_COMPILE_NV: Self = Self(32u64);
    pub const PIPELINE_CREATE_2_CAPTURE_STATISTICS_KHR: Self = Self(64u64);
    pub const PIPELINE_CREATE_2_CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(128u64);
    pub const PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(256u64);
    pub const PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE: Self = Self(512u64);
    pub const PIPELINE_CREATE_2_LINK_TIME_OPTIMIZATION_EXT: Self = Self(1024u64);
    pub const PIPELINE_CREATE_2_LIBRARY_KHR: Self = Self(2048u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(4096u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SKIP_AABBS_KHR: Self = Self(8192u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(16384u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(32768u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(65536u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self =
        Self(131072u64);
    pub const PIPELINE_CREATE_2_INDIRECT_BINDABLE_NV: Self = Self(262144u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self =
        Self(524288u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1048576u64);
    pub const PIPELINE_CREATE_2_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self(2097152u64);
    pub const PIPELINE_CREATE_2_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self(4194304u64);
    pub const PIPELINE_CREATE_2_RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(8388608u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(16777216u64);
    pub const PIPELINE_CREATE_2_COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(33554432u64);
    pub const PIPELINE_CREATE_2_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self =
        Self(67108864u64);
    pub const PIPELINE_CREATE_2_NO_PROTECTED_ACCESS: Self = Self(134217728u64);
    #[cfg(feature = "beta")]
    pub const PIPELINE_CREATE_2_RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(268435456u64);
    pub const PIPELINE_CREATE_2_DESCRIPTOR_BUFFER_EXT: Self = Self(536870912u64);
    pub const PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY: Self = Self(1073741824u64);
    pub const PIPELINE_CREATE_2_CAPTURE_DATA_KHR: Self = Self(2147483648u64);
    #[cfg(feature = "beta")]
    pub const PIPELINE_CREATE_2_EXECUTION_GRAPH_BIT_AMDX: Self = Self(4294967296u64);
    pub const PIPELINE_CREATE_2_RAY_TRACING_ALLOW_SPHERES_AND_LINEAR_SWEPT_SPHERES_NV: Self =
        Self(8589934592u64);
    pub const PIPELINE_CREATE_2_ENABLE_LEGACY_DITHERING_EXT: Self = Self(17179869184u64);
    pub const PIPELINE_CREATE_2_DESCRIPTOR_HEAP_EXT: Self = Self(68719476736u64);
    pub const PIPELINE_CREATE_2_DISALLOW_OPACITY_MICROMAP_ARM: Self = Self(137438953472u64);
    pub const PIPELINE_CREATE_2_INDIRECT_BINDABLE_EXT: Self = Self(274877906944u64);
    pub const PIPELINE_CREATE_2_INSTRUMENT_SHADERS_ARM: Self = Self(549755813888u64);
    pub const PIPELINE_CREATE_2_PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(1099511627776u64);
    pub const PIPELINE_CREATE_2_64_BIT_INDEXING_EXT: Self = Self(8796093022208u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCreateFlags2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCreateFlags2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCreateFlags2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PipelineCreateFlags2KHR = PipelineCreateFlags2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineCreationFeedbackFlags(pub u32);

impl PipelineCreationFeedbackFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VALID: Self = Self(1u32);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(2u32);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineCreationFeedbackFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineCreationFeedbackFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineCreationFeedbackFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PipelineCreationFeedbackFlagsEXT = PipelineCreationFeedbackFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineDepthStencilStateCreateFlags(pub u32);

impl PipelineDepthStencilStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineDepthStencilStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineDepthStencilStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineDepthStencilStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineDiscardRectangleStateCreateFlagsEXT(pub u32);

impl PipelineDiscardRectangleStateCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineDiscardRectangleStateCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineDiscardRectangleStateCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineDiscardRectangleStateCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineDynamicStateCreateFlags(pub u32);

impl PipelineDynamicStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineDynamicStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineDynamicStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineDynamicStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineInputAssemblyStateCreateFlags(pub u32);

impl PipelineInputAssemblyStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineInputAssemblyStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineInputAssemblyStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineInputAssemblyStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineLayoutCreateFlags(pub u32);

impl PipelineLayoutCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INDEPENDENT_SETS_EXT: Self = Self(2u32);
    pub const NO_TASK_SHADER_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineLayoutCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineLayoutCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineLayoutCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineMultisampleStateCreateFlags(pub u32);

impl PipelineMultisampleStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineMultisampleStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineMultisampleStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineMultisampleStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRasterizationConservativeStateCreateFlagsEXT(pub u32);

impl PipelineRasterizationConservativeStateCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineRasterizationConservativeStateCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineRasterizationConservativeStateCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineRasterizationConservativeStateCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(pub u32);

impl PipelineRasterizationDepthClipStateCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineRasterizationDepthClipStateCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRasterizationStateCreateFlags(pub u32);

impl PipelineRasterizationStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineRasterizationStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineRasterizationStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineRasterizationStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineRasterizationStateStreamCreateFlagsEXT(pub u32);

impl PipelineRasterizationStateStreamCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineRasterizationStateStreamCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineRasterizationStateStreamCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineRasterizationStateStreamCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineShaderStageCreateFlags(pub u32);

impl PipelineShaderStageCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(1u32);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineShaderStageCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineShaderStageCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineShaderStageCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineStageFlags(pub u32);

impl PipelineStageFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const TOP_OF_PIPE: Self = Self(1u32);
    pub const DRAW_INDIRECT: Self = Self(2u32);
    pub const VERTEX_INPUT: Self = Self(4u32);
    pub const VERTEX_SHADER: Self = Self(8u32);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(16u32);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(32u32);
    pub const GEOMETRY_SHADER: Self = Self(64u32);
    pub const FRAGMENT_SHADER: Self = Self(128u32);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(256u32);
    pub const LATE_FRAGMENT_TESTS: Self = Self(512u32);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1024u32);
    pub const COMPUTE_SHADER: Self = Self(2048u32);
    pub const TRANSFER: Self = Self(4096u32);
    pub const BOTTOM_OF_PIPE: Self = Self(8192u32);
    pub const HOST: Self = Self(16384u32);
    pub const ALL_GRAPHICS: Self = Self(32768u32);
    pub const ALL_COMMANDS: Self = Self(65536u32);
    pub const COMMAND_PREPROCESS_EXT: Self = Self(131072u32);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(262144u32);
    pub const TASK_SHADER_EXT: Self = Self(524288u32);
    pub const MESH_SHADER_EXT: Self = Self(1048576u32);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(2097152u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304u32);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608u32);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(16777216u32);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineStageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineStageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineStageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineStageFlags2(pub u64);

impl PipelineStageFlags2 {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const PIPELINE_STAGE_2_NONE: Self = Self(0u64);
    pub const PIPELINE_STAGE_2_TOP_OF_PIPE: Self = Self(1u64);
    pub const PIPELINE_STAGE_2_DRAW_INDIRECT: Self = Self(2u64);
    pub const PIPELINE_STAGE_2_VERTEX_INPUT: Self = Self(4u64);
    pub const PIPELINE_STAGE_2_VERTEX_SHADER: Self = Self(8u64);
    pub const PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER: Self = Self(16u64);
    pub const PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER: Self = Self(32u64);
    pub const PIPELINE_STAGE_2_GEOMETRY_SHADER: Self = Self(64u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_SHADER: Self = Self(128u64);
    pub const PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS: Self = Self(256u64);
    pub const PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS: Self = Self(512u64);
    pub const PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT: Self = Self(1024u64);
    pub const PIPELINE_STAGE_2_COMPUTE_SHADER: Self = Self(2048u64);
    pub const PIPELINE_STAGE_2_ALL_TRANSFER: Self = Self(4096u64);
    pub const PIPELINE_STAGE_2_BOTTOM_OF_PIPE: Self = Self(8192u64);
    pub const PIPELINE_STAGE_2_HOST: Self = Self(16384u64);
    pub const PIPELINE_STAGE_2_ALL_GRAPHICS: Self = Self(32768u64);
    pub const PIPELINE_STAGE_2_ALL_COMMANDS: Self = Self(65536u64);
    pub const PIPELINE_STAGE_2_COMMAND_PREPROCESS_EXT: Self = Self(131072u64);
    pub const PIPELINE_STAGE_2_CONDITIONAL_RENDERING_EXT: Self = Self(262144u64);
    pub const PIPELINE_STAGE_2_TASK_SHADER_EXT: Self = Self(524288u64);
    pub const PIPELINE_STAGE_2_MESH_SHADER_EXT: Self = Self(1048576u64);
    pub const PIPELINE_STAGE_2_RAY_TRACING_SHADER_KHR: Self = Self(2097152u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(4194304u64);
    pub const PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(8388608u64);
    pub const PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_EXT: Self = Self(16777216u64);
    pub const PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(33554432u64);
    pub const PIPELINE_STAGE_2_VIDEO_DECODE_KHR: Self = Self(67108864u64);
    pub const PIPELINE_STAGE_2_VIDEO_ENCODE_KHR: Self = Self(134217728u64);
    pub const PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(268435456u64);
    pub const PIPELINE_STAGE_2_OPTICAL_FLOW_NV: Self = Self(536870912u64);
    pub const PIPELINE_STAGE_2_MICROMAP_BUILD_EXT: Self = Self(1073741824u64);
    pub const PIPELINE_STAGE_2_COPY: Self = Self(4294967296u64);
    pub const PIPELINE_STAGE_2_RESOLVE: Self = Self(8589934592u64);
    pub const PIPELINE_STAGE_2_BLIT: Self = Self(17179869184u64);
    pub const PIPELINE_STAGE_2_CLEAR: Self = Self(34359738368u64);
    pub const PIPELINE_STAGE_2_INDEX_INPUT: Self = Self(68719476736u64);
    pub const PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT: Self = Self(137438953472u64);
    pub const PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS: Self = Self(274877906944u64);
    pub const PIPELINE_STAGE_2_SUBPASS_SHADER_BIT_HUAWEI: Self = Self(549755813888u64);
    pub const PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI: Self = Self(1099511627776u64);
    pub const PIPELINE_STAGE_2_CLUSTER_CULLING_SHADER_BIT_HUAWEI: Self = Self(2199023255552u64);
    pub const PIPELINE_STAGE_2_DATA_GRAPH_ARM: Self = Self(4398046511104u64);
    pub const PIPELINE_STAGE_2_CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(17592186044416u64);
    pub const PIPELINE_STAGE_2_MEMORY_DECOMPRESSION_EXT: Self = Self(35184372088832u64);
    pub const PIPELINE_STAGE_2_COPY_INDIRECT_KHR: Self = Self(70368744177664u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineStageFlags2 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineStageFlags2 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineStageFlags2 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PipelineStageFlags2KHR = PipelineStageFlags2;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineTessellationStateCreateFlags(pub u32);

impl PipelineTessellationStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineTessellationStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineTessellationStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineTessellationStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineVertexInputStateCreateFlags(pub u32);

impl PipelineVertexInputStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineVertexInputStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineVertexInputStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineVertexInputStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineViewportStateCreateFlags(pub u32);

impl PipelineViewportStateCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineViewportStateCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineViewportStateCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineViewportStateCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(pub u32);

impl PipelineViewportSwizzleStateCreateFlagsNV {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PipelineViewportSwizzleStateCreateFlagsNV {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PipelineViewportSwizzleStateCreateFlagsNV {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PipelineViewportSwizzleStateCreateFlagsNV {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentGravityFlagsKHR(pub u32);

impl PresentGravityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const MIN_KHR: Self = Self(1u32);
    pub const MAX_KHR: Self = Self(2u32);
    pub const CENTERED_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PresentGravityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PresentGravityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PresentGravityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PresentGravityFlagsEXT = PresentGravityFlagsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentScalingFlagsKHR(pub u32);

impl PresentScalingFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ONE_TO_ONE_KHR: Self = Self(1u32);
    pub const ASPECT_RATIO_STRETCH_KHR: Self = Self(2u32);
    pub const STRETCH_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PresentScalingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PresentScalingFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PresentScalingFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PresentScalingFlagsEXT = PresentScalingFlagsKHR;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentStageFlagsEXT(pub u32);

impl PresentStageFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const QUEUE_OPERATIONS_END_EXT: Self = Self(1u32);
    pub const REQUEST_DEQUEUED_EXT: Self = Self(2u32);
    pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(4u32);
    pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PresentStageFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PresentStageFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PresentStageFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PresentTimingInfoFlagsEXT(pub u32);

impl PresentTimingInfoFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(1u32);
    pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PresentTimingInfoFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PresentTimingInfoFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PresentTimingInfoFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct PrivateDataSlotCreateFlags(pub u32);

impl PrivateDataSlotCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for PrivateDataSlotCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for PrivateDataSlotCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for PrivateDataSlotCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type PrivateDataSlotCreateFlagsEXT = PrivateDataSlotCreateFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryControlFlags(pub u32);

impl QueryControlFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRECISE: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for QueryControlFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for QueryControlFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for QueryControlFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryPipelineStatisticFlags(pub u32);

impl QueryPipelineStatisticFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1u32);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(2u32);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(4u32);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(8u32);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(16u32);
    pub const CLIPPING_INVOCATIONS: Self = Self(32u32);
    pub const CLIPPING_PRIMITIVES: Self = Self(64u32);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(128u32);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(256u32);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(512u32);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1024u32);
    pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(2048u32);
    pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(4096u32);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_BIT_HUAWEI: Self = Self(8192u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for QueryPipelineStatisticFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for QueryPipelineStatisticFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for QueryPipelineStatisticFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryPoolCreateFlags(pub u32);

impl QueryPoolCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESET_KHR: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for QueryPoolCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for QueryPoolCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for QueryPoolCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueryResultFlags(pub u32);

impl QueryResultFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _64: Self = Self(1u32);
    pub const WAIT: Self = Self(2u32);
    pub const WITH_AVAILABILITY: Self = Self(4u32);
    pub const PARTIAL: Self = Self(8u32);
    pub const WITH_STATUS_KHR: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for QueryResultFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for QueryResultFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for QueryResultFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct QueueFlags(pub u32);

impl QueueFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const GRAPHICS: Self = Self(1u32);
    pub const COMPUTE: Self = Self(2u32);
    pub const TRANSFER: Self = Self(4u32);
    pub const SPARSE_BINDING: Self = Self(8u32);
    pub const PROTECTED: Self = Self(16u32);
    pub const VIDEO_DECODE_KHR: Self = Self(32u32);
    pub const VIDEO_ENCODE_KHR: Self = Self(64u32);
    pub const OPTICAL_FLOW_NV: Self = Self(256u32);
    pub const DATA_GRAPH_ARM: Self = Self(1024u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for QueueFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for QueueFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for QueueFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderPassCreateFlags(pub u32);

impl RenderPassCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TRANSFORM_QCOM: Self = Self(2u32);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for RenderPassCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for RenderPassCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for RenderPassCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderingAttachmentFlagsKHR(pub u32);

impl RenderingAttachmentFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INPUT_ATTACHMENT_FEEDBACK_KHR: Self = Self(1u32);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for RenderingAttachmentFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for RenderingAttachmentFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for RenderingAttachmentFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct RenderingFlags(pub u32);

impl RenderingFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1u32);
    pub const SUSPENDING: Self = Self(2u32);
    pub const RESUMING: Self = Self(4u32);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(8u32);
    pub const CONTENTS_INLINE_KHR: Self = Self(16u32);
    pub const PER_LAYER_FRAGMENT_DENSITY_BIT_VALVE: Self = Self(32u32);
    pub const FRAGMENT_REGION_EXT: Self = Self(64u32);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(128u32);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(256u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for RenderingFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for RenderingFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for RenderingFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type RenderingFlagsKHR = RenderingFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ResolveImageFlagsKHR(pub u32);

impl ResolveImageFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1u32);
    pub const ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ResolveImageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ResolveImageFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ResolveImageFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ResolveModeFlags(pub u32);

impl ResolveModeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE: Self = Self(0u32);
    pub const SAMPLE_ZERO: Self = Self(1u32);
    pub const AVERAGE: Self = Self(2u32);
    pub const MIN: Self = Self(4u32);
    pub const MAX: Self = Self(8u32);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_BIT_ANDROID: Self = Self(16u32);
    pub const CUSTOM_EXT: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ResolveModeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ResolveModeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ResolveModeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ResolveModeFlagsKHR = ResolveModeFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SampleCountFlags(pub u32);

impl SampleCountFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _1: Self = Self(1u32);
    pub const _2: Self = Self(2u32);
    pub const _4: Self = Self(4u32);
    pub const _8: Self = Self(8u32);
    pub const _16: Self = Self(16u32);
    pub const _32: Self = Self(32u32);
    pub const _64: Self = Self(64u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SampleCountFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SampleCountFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SampleCountFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SamplerCreateFlags(pub u32);

impl SamplerCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SUBSAMPLED_EXT: Self = Self(1u32);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(2u32);
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(4u32);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(8u32);
    pub const IMAGE_PROCESSING_QCOM: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SamplerCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SamplerCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SamplerCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "screen")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ScreenSurfaceCreateFlagsQNX(pub u32);

#[cfg(feature = "screen")]
impl ScreenSurfaceCreateFlagsQNX {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "screen")]
impl core::ops::BitOr for ScreenSurfaceCreateFlagsQNX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "screen")]
impl core::ops::BitOrAssign for ScreenSurfaceCreateFlagsQNX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "screen")]
impl core::ops::BitAnd for ScreenSurfaceCreateFlagsQNX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreCreateFlags(pub u32);

impl SemaphoreCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SemaphoreCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SemaphoreCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SemaphoreCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreImportFlags(pub u32);

impl SemaphoreImportFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const TEMPORARY: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SemaphoreImportFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SemaphoreImportFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SemaphoreImportFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SemaphoreWaitFlags(pub u32);

impl SemaphoreWaitFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ANY: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SemaphoreWaitFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SemaphoreWaitFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SemaphoreWaitFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type SemaphoreWaitFlagsKHR = SemaphoreWaitFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderCorePropertiesFlagsAMD(pub u32);

impl ShaderCorePropertiesFlagsAMD {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ShaderCorePropertiesFlagsAMD {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ShaderCorePropertiesFlagsAMD {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderCreateFlagsEXT(pub u32);

impl ShaderCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const LINK_STAGE_EXT: Self = Self(1u32);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(2u32);
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(4u32);
    pub const NO_TASK_SHADER_EXT: Self = Self(8u32);
    pub const DISPATCH_BASE_EXT: Self = Self(16u32);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self = Self(32u32);
    pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(64u32);
    pub const INDIRECT_BINDABLE_EXT: Self = Self(128u32);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(1024u32);
    pub const INSTRUMENT_SHADER_ARM: Self = Self(2048u32);
    pub const _64_BIT_INDEXING_EXT: Self = Self(32768u32);
    pub const INDEPENDENT_SETS_KHR: Self = Self(262144u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ShaderCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ShaderCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ShaderCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderInstrumentationValuesFlagsARM(pub u32);

impl ShaderInstrumentationValuesFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ShaderInstrumentationValuesFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ShaderInstrumentationValuesFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ShaderInstrumentationValuesFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderModuleCreateFlags(pub u32);

impl ShaderModuleCreateFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ShaderModuleCreateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ShaderModuleCreateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ShaderModuleCreateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ShaderStageFlags(pub u32);

impl ShaderStageFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VERTEX: Self = Self(1u32);
    pub const TESSELLATION_CONTROL: Self = Self(2u32);
    pub const TESSELLATION_EVALUATION: Self = Self(4u32);
    pub const GEOMETRY: Self = Self(8u32);
    pub const FRAGMENT: Self = Self(16u32);
    pub const ALL_GRAPHICS: Self = Self(31u32);
    pub const COMPUTE: Self = Self(32u32);
    pub const TASK_EXT: Self = Self(64u32);
    pub const MESH_EXT: Self = Self(128u32);
    pub const RAYGEN_KHR: Self = Self(256u32);
    pub const ANY_HIT_KHR: Self = Self(512u32);
    pub const CLOSEST_HIT_KHR: Self = Self(1024u32);
    pub const MISS_KHR: Self = Self(2048u32);
    pub const INTERSECTION_KHR: Self = Self(4096u32);
    pub const CALLABLE_KHR: Self = Self(8192u32);
    pub const SUBPASS_SHADING_BIT_HUAWEI: Self = Self(16384u32);
    pub const CLUSTER_CULLING_BIT_HUAWEI: Self = Self(524288u32);
    pub const ALL: Self = Self(2147483647u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ShaderStageFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ShaderStageFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ShaderStageFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SparseImageFormatFlags(pub u32);

impl SparseImageFormatFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SINGLE_MIPTAIL: Self = Self(1u32);
    pub const ALIGNED_MIP_SIZE: Self = Self(2u32);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SparseImageFormatFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SparseImageFormatFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SparseImageFormatFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SparseMemoryBindFlags(pub u32);

impl SparseMemoryBindFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const METADATA: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SparseMemoryBindFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SparseMemoryBindFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SparseMemoryBindFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SpirvResourceTypeFlagsEXT(pub u32);

impl SpirvResourceTypeFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SAMPLER_EXT: Self = Self(1u32);
    pub const SAMPLED_IMAGE_EXT: Self = Self(2u32);
    pub const READ_ONLY_IMAGE_EXT: Self = Self(4u32);
    pub const READ_WRITE_IMAGE_EXT: Self = Self(8u32);
    pub const COMBINED_SAMPLED_IMAGE_EXT: Self = Self(16u32);
    pub const UNIFORM_BUFFER_EXT: Self = Self(32u32);
    pub const READ_ONLY_STORAGE_BUFFER_EXT: Self = Self(64u32);
    pub const READ_WRITE_STORAGE_BUFFER_EXT: Self = Self(128u32);
    pub const ACCELERATION_STRUCTURE_EXT: Self = Self(256u32);
    pub const TENSOR_ARM: Self = Self(512u32);
    pub const ALL_EXT: Self = Self(2147483647u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SpirvResourceTypeFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SpirvResourceTypeFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SpirvResourceTypeFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StencilFaceFlags(pub u32);

impl StencilFaceFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FRONT: Self = Self(1u32);
    pub const BACK: Self = Self(2u32);
    pub const FRONT_AND_BACK: Self = Self(3u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for StencilFaceFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for StencilFaceFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for StencilFaceFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "ggp")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct StreamDescriptorSurfaceCreateFlagsGGP(pub u32);

#[cfg(feature = "ggp")]
impl StreamDescriptorSurfaceCreateFlagsGGP {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "ggp")]
impl core::ops::BitOr for StreamDescriptorSurfaceCreateFlagsGGP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "ggp")]
impl core::ops::BitOrAssign for StreamDescriptorSurfaceCreateFlagsGGP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "ggp")]
impl core::ops::BitAnd for StreamDescriptorSurfaceCreateFlagsGGP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubgroupFeatureFlags(pub u32);

impl SubgroupFeatureFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BASIC: Self = Self(1u32);
    pub const VOTE: Self = Self(2u32);
    pub const ARITHMETIC: Self = Self(4u32);
    pub const BALLOT: Self = Self(8u32);
    pub const SHUFFLE: Self = Self(16u32);
    pub const SHUFFLE_RELATIVE: Self = Self(32u32);
    pub const CLUSTERED: Self = Self(64u32);
    pub const QUAD: Self = Self(128u32);
    pub const PARTITIONED_EXT: Self = Self(256u32);
    pub const ROTATE: Self = Self(512u32);
    pub const ROTATE_CLUSTERED: Self = Self(1024u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SubgroupFeatureFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SubgroupFeatureFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SubgroupFeatureFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubmitFlags(pub u32);

impl SubmitFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SubmitFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SubmitFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SubmitFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type SubmitFlagsKHR = SubmitFlags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SubpassDescriptionFlags(pub u32);

impl SubpassDescriptionFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PER_VIEW_ATTRIBUTES_BIT_NVX: Self = Self(1u32);
    pub const PER_VIEW_POSITION_X_ONLY_BIT_NVX: Self = Self(2u32);
    pub const FRAGMENT_REGION_EXT: Self = Self(4u32);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(8u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(16u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(32u32);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(64u32);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(128u32);
    pub const TILE_SHADING_APRON_QCOM: Self = Self(256u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SubpassDescriptionFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SubpassDescriptionFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SubpassDescriptionFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SurfaceCounterFlagsEXT(pub u32);

impl SurfaceCounterFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VBLANK_EXT: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SurfaceCounterFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SurfaceCounterFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SurfaceCounterFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "ohos")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SurfaceCreateFlagsOHOS(pub u32);

#[cfg(feature = "ohos")]
impl SurfaceCreateFlagsOHOS {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "ohos")]
impl core::ops::BitOr for SurfaceCreateFlagsOHOS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "ohos")]
impl core::ops::BitOrAssign for SurfaceCreateFlagsOHOS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "ohos")]
impl core::ops::BitAnd for SurfaceCreateFlagsOHOS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SurfaceTransformFlagsKHR(pub u32);

impl SurfaceTransformFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const IDENTITY_KHR: Self = Self(1u32);
    pub const ROTATE_90_KHR: Self = Self(2u32);
    pub const ROTATE_180_KHR: Self = Self(4u32);
    pub const ROTATE_270_KHR: Self = Self(8u32);
    pub const HORIZONTAL_MIRROR_KHR: Self = Self(16u32);
    pub const HORIZONTAL_MIRROR_ROTATE_90_KHR: Self = Self(32u32);
    pub const HORIZONTAL_MIRROR_ROTATE_180_KHR: Self = Self(64u32);
    pub const HORIZONTAL_MIRROR_ROTATE_270_KHR: Self = Self(128u32);
    pub const INHERIT_KHR: Self = Self(256u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SurfaceTransformFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SurfaceTransformFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SurfaceTransformFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct SwapchainCreateFlagsKHR(pub u32);

impl SwapchainCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self(1u32);
    pub const PROTECTED_KHR: Self = Self(2u32);
    pub const MUTABLE_FORMAT_KHR: Self = Self(4u32);
    pub const DEFERRED_MEMORY_ALLOCATION_KHR: Self = Self(8u32);
    pub const PRESENT_ID_2_KHR: Self = Self(64u32);
    pub const PRESENT_WAIT_2_KHR: Self = Self(128u32);
    pub const PRESENT_TIMING_EXT: Self = Self(512u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for SwapchainCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for SwapchainCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for SwapchainCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorCreateFlagsARM(pub u64);

impl TensorCreateFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const MUTABLE_FORMAT_ARM: Self = Self(1u64);
    pub const PROTECTED_ARM: Self = Self(2u64);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(4u64);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(8u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for TensorCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for TensorCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for TensorCreateFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorUsageFlagsARM(pub u64);

impl TensorUsageFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const SHADER_ARM: Self = Self(2u64);
    pub const TRANSFER_SRC_ARM: Self = Self(4u64);
    pub const TRANSFER_DST_ARM: Self = Self(8u64);
    pub const IMAGE_ALIASING_ARM: Self = Self(16u64);
    pub const DATA_GRAPH_ARM: Self = Self(32u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for TensorUsageFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for TensorUsageFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for TensorUsageFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TensorViewCreateFlagsARM(pub u64);

impl TensorViewCreateFlagsARM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u64) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u64 {
        self.0
    }
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1u64);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for TensorViewCreateFlagsARM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for TensorViewCreateFlagsARM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for TensorViewCreateFlagsARM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct TileShadingRenderPassFlagsQCOM(pub u32);

impl TileShadingRenderPassFlagsQCOM {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ENABLE_QCOM: Self = Self(1u32);
    pub const PER_TILE_EXECUTION_QCOM: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for TileShadingRenderPassFlagsQCOM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for TileShadingRenderPassFlagsQCOM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for TileShadingRenderPassFlagsQCOM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ToolPurposeFlags(pub u32);

impl ToolPurposeFlags {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const VALIDATION: Self = Self(1u32);
    pub const PROFILING: Self = Self(2u32);
    pub const TRACING: Self = Self(4u32);
    pub const ADDITIONAL_FEATURES: Self = Self(8u32);
    pub const MODIFYING_FEATURES: Self = Self(16u32);
    pub const DEBUG_REPORTING_EXT: Self = Self(32u32);
    pub const DEBUG_MARKERS_EXT: Self = Self(64u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ToolPurposeFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ToolPurposeFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ToolPurposeFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

pub type ToolPurposeFlagsEXT = ToolPurposeFlags;

#[cfg(feature = "ubm")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct UbmSurfaceCreateFlagsSEC(pub u32);

#[cfg(feature = "ubm")]
impl UbmSurfaceCreateFlagsSEC {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "ubm")]
impl core::ops::BitOr for UbmSurfaceCreateFlagsSEC {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "ubm")]
impl core::ops::BitOrAssign for UbmSurfaceCreateFlagsSEC {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "ubm")]
impl core::ops::BitAnd for UbmSurfaceCreateFlagsSEC {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ValidationCacheCreateFlagsEXT(pub u32);

impl ValidationCacheCreateFlagsEXT {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for ValidationCacheCreateFlagsEXT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for ValidationCacheCreateFlagsEXT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for ValidationCacheCreateFlagsEXT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "vi")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct ViSurfaceCreateFlagsNN(pub u32);

#[cfg(feature = "vi")]
impl ViSurfaceCreateFlagsNN {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "vi")]
impl core::ops::BitOr for ViSurfaceCreateFlagsNN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "vi")]
impl core::ops::BitOrAssign for ViSurfaceCreateFlagsNN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "vi")]
impl core::ops::BitAnd for ViSurfaceCreateFlagsNN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoBeginCodingFlagsKHR(pub u32);

impl VideoBeginCodingFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoBeginCodingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoBeginCodingFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoBeginCodingFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCapabilityFlagsKHR(pub u32);

impl VideoCapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_CONTENT_KHR: Self = Self(1u32);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoCapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoChromaSubsamplingFlagsKHR(pub u32);

impl VideoChromaSubsamplingFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVALID_KHR: Self = Self(0u32);
    pub const MONOCHROME_KHR: Self = Self(1u32);
    pub const _420_KHR: Self = Self(2u32);
    pub const _422_KHR: Self = Self(4u32);
    pub const _444_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoChromaSubsamplingFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoChromaSubsamplingFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCodecOperationFlagsKHR(pub u32);

impl VideoCodecOperationFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_KHR: Self = Self(0u32);
    pub const DECODE_H264_KHR: Self = Self(1u32);
    pub const DECODE_H265_KHR: Self = Self(2u32);
    pub const DECODE_AV1_KHR: Self = Self(4u32);
    pub const DECODE_VP9_KHR: Self = Self(8u32);
    pub const ENCODE_H264_KHR: Self = Self(65536u32);
    pub const ENCODE_H265_KHR: Self = Self(131072u32);
    pub const ENCODE_AV1_KHR: Self = Self(262144u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoCodecOperationFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoCodecOperationFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoCodecOperationFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoCodingControlFlagsKHR(pub u32);

impl VideoCodingControlFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RESET_KHR: Self = Self(1u32);
    pub const ENCODE_RATE_CONTROL_KHR: Self = Self(2u32);
    pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoCodingControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoCodingControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoCodingControlFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoComponentBitDepthFlagsKHR(pub u32);

impl VideoComponentBitDepthFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const INVALID_KHR: Self = Self(0u32);
    pub const _8_KHR: Self = Self(1u32);
    pub const _10_KHR: Self = Self(4u32);
    pub const _12_KHR: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoComponentBitDepthFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoComponentBitDepthFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeCapabilityFlagsKHR(pub u32);

impl VideoDecodeCapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1u32);
    pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoDecodeCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoDecodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeFlagsKHR(pub u32);

impl VideoDecodeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoDecodeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoDecodeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoDecodeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeH264PictureLayoutFlagsKHR(pub u32);

impl VideoDecodeH264PictureLayoutFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROGRESSIVE_KHR: Self = Self(0u32);
    pub const INTERLACED_INTERLEAVED_LINES_KHR: Self = Self(1u32);
    pub const INTERLACED_SEPARATE_PLANES_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoDecodeH264PictureLayoutFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoDecodeH264PictureLayoutFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoDecodeH264PictureLayoutFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoDecodeUsageFlagsKHR(pub u32);

impl VideoDecodeUsageFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const TRANSCODING_KHR: Self = Self(1u32);
    pub const OFFLINE_KHR: Self = Self(2u32);
    pub const STREAMING_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoDecodeUsageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoDecodeUsageFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoDecodeUsageFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1CapabilityFlagsKHR(pub u32);

impl VideoEncodeAV1CapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self = Self(1u32);
    pub const GENERATE_OBU_EXTENSION_HEADER_KHR: Self = Self(2u32);
    pub const PRIMARY_REFERENCE_CDF_ONLY_KHR: Self = Self(4u32);
    pub const FRAME_SIZE_OVERRIDE_KHR: Self = Self(8u32);
    pub const MOTION_VECTOR_SCALING_KHR: Self = Self(16u32);
    pub const COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeAV1CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeAV1CapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeAV1CapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1RateControlFlagsKHR(pub u32);

impl VideoEncodeAV1RateControlFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const REGULAR_GOP_KHR: Self = Self(1u32);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeAV1RateControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeAV1RateControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeAV1RateControlFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1StdFlagsKHR(pub u32);

impl VideoEncodeAV1StdFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self = Self(1u32);
    pub const SKIP_MODE_PRESENT_UNSET_KHR: Self = Self(2u32);
    pub const PRIMARY_REF_FRAME_KHR: Self = Self(4u32);
    pub const DELTA_Q_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeAV1StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeAV1StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeAV1StdFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeAV1SuperblockSizeFlagsKHR(pub u32);

impl VideoEncodeAV1SuperblockSizeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _64_KHR: Self = Self(1u32);
    pub const _128_KHR: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeAV1SuperblockSizeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeAV1SuperblockSizeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeAV1SuperblockSizeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeCapabilityFlagsKHR(pub u32);

impl VideoEncodeCapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(1u32);
    pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(2u32);
    pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(4u32);
    pub const EMPHASIS_MAP_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeCapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeCapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeContentFlagsKHR(pub u32);

impl VideoEncodeContentFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const CAMERA_KHR: Self = Self(1u32);
    pub const DESKTOP_KHR: Self = Self(2u32);
    pub const RENDERED_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeContentFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeContentFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeContentFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeFeedbackFlagsKHR(pub u32);

impl VideoEncodeFeedbackFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1u32);
    pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(2u32);
    pub const BITSTREAM_HAS_OVERRIDES_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeFeedbackFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeFeedbackFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeFeedbackFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeFlagsKHR(pub u32);

impl VideoEncodeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1u32);
    pub const WITH_EMPHASIS_MAP_KHR: Self = Self(2u32);
    pub const INTRA_REFRESH_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264CapabilityFlagsKHR(pub u32);

impl VideoEncodeH264CapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(2u32);
    pub const ROW_UNALIGNED_SLICE_KHR: Self = Self(4u32);
    pub const DIFFERENT_SLICE_TYPE_KHR: Self = Self(8u32);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(16u32);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(32u32);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(64u32);
    pub const PER_SLICE_CONSTANT_QP_KHR: Self = Self(128u32);
    pub const GENERATE_PREFIX_NALU_KHR: Self = Self(256u32);
    pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(512u32);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1024u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH264CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH264CapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH264CapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264RateControlFlagsKHR(pub u32);

impl VideoEncodeH264RateControlFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const REGULAR_GOP_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH264RateControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH264RateControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH264RateControlFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH264StdFlagsKHR(pub u32);

impl VideoEncodeH264StdFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1u32);
    pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR: Self = Self(2u32);
    pub const SCALING_MATRIX_PRESENT_FLAG_SET_KHR: Self = Self(4u32);
    pub const CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(8u32);
    pub const SECOND_CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(16u32);
    pub const PIC_INIT_QP_MINUS26_KHR: Self = Self(32u32);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(64u32);
    pub const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR: Self = Self(128u32);
    pub const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR: Self = Self(256u32);
    pub const TRANSFORM_8X8_MODE_FLAG_SET_KHR: Self = Self(512u32);
    pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR: Self = Self(1024u32);
    pub const ENTROPY_CODING_MODE_FLAG_UNSET_KHR: Self = Self(2048u32);
    pub const ENTROPY_CODING_MODE_FLAG_SET_KHR: Self = Self(4096u32);
    pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR: Self = Self(8192u32);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(16384u32);
    pub const DEBLOCKING_FILTER_DISABLED_KHR: Self = Self(32768u32);
    pub const DEBLOCKING_FILTER_ENABLED_KHR: Self = Self(65536u32);
    pub const DEBLOCKING_FILTER_PARTIAL_KHR: Self = Self(131072u32);
    pub const SLICE_QP_DELTA_KHR: Self = Self(524288u32);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1048576u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH264StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH264StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH264StdFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265CapabilityFlagsKHR(pub u32);

impl VideoEncodeH265CapabilityFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(2u32);
    pub const ROW_UNALIGNED_SLICE_SEGMENT_KHR: Self = Self(4u32);
    pub const DIFFERENT_SLICE_SEGMENT_TYPE_KHR: Self = Self(8u32);
    pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(16u32);
    pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(32u32);
    pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(64u32);
    pub const PER_SLICE_SEGMENT_CONSTANT_QP_KHR: Self = Self(128u32);
    pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR: Self = Self(256u32);
    pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR: Self = Self(512u32);
    pub const CU_QP_DIFF_WRAPAROUND_KHR: Self = Self(1024u32);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(2048u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH265CapabilityFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH265CapabilityFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH265CapabilityFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265CtbSizeFlagsKHR(pub u32);

impl VideoEncodeH265CtbSizeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _16_KHR: Self = Self(1u32);
    pub const _32_KHR: Self = Self(2u32);
    pub const _64_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH265CtbSizeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH265CtbSizeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH265CtbSizeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265RateControlFlagsKHR(pub u32);

impl VideoEncodeH265RateControlFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1u32);
    pub const REGULAR_GOP_KHR: Self = Self(2u32);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(4u32);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(8u32);
    pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH265RateControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH265RateControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH265RateControlFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265StdFlagsKHR(pub u32);

impl VideoEncodeH265StdFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1u32);
    pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR: Self = Self(2u32);
    pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR: Self = Self(4u32);
    pub const PCM_ENABLED_FLAG_SET_KHR: Self = Self(8u32);
    pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR: Self = Self(16u32);
    pub const INIT_QP_MINUS26_KHR: Self = Self(32u32);
    pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(64u32);
    pub const WEIGHTED_BIPRED_FLAG_SET_KHR: Self = Self(128u32);
    pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR: Self = Self(256u32);
    pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR: Self = Self(512u32);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR: Self = Self(1024u32);
    pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR: Self = Self(2048u32);
    pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR: Self = Self(4096u32);
    pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR: Self = Self(8192u32);
    pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(16384u32);
    pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR: Self = Self(32768u32);
    pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR: Self = Self(65536u32);
    pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR: Self = Self(131072u32);
    pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR: Self = Self(262144u32);
    pub const SLICE_QP_DELTA_KHR: Self = Self(524288u32);
    pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1048576u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH265StdFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH265StdFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH265StdFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeH265TransformBlockSizeFlagsKHR(pub u32);

impl VideoEncodeH265TransformBlockSizeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const _4_KHR: Self = Self(1u32);
    pub const _8_KHR: Self = Self(2u32);
    pub const _16_KHR: Self = Self(4u32);
    pub const _32_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeH265TransformBlockSizeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeH265TransformBlockSizeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeH265TransformBlockSizeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeIntraRefreshModeFlagsKHR(pub u32);

impl VideoEncodeIntraRefreshModeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const NONE_KHR: Self = Self(0u32);
    pub const PER_PICTURE_PARTITION_KHR: Self = Self(1u32);
    pub const BLOCK_BASED_KHR: Self = Self(2u32);
    pub const BLOCK_ROW_BASED_KHR: Self = Self(4u32);
    pub const BLOCK_COLUMN_BASED_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeIntraRefreshModeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeIntraRefreshModeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeIntraRefreshModeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRateControlFlagsKHR(pub u32);

impl VideoEncodeRateControlFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeRateControlFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeRateControlFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRateControlModeFlagsKHR(pub u32);

impl VideoEncodeRateControlModeFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const DISABLED_KHR: Self = Self(1u32);
    pub const CBR_KHR: Self = Self(2u32);
    pub const VBR_KHR: Self = Self(4u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeRateControlModeFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeRateControlModeFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbChromaOffsetFlagsVALVE(pub u32);

impl VideoEncodeRgbChromaOffsetFlagsVALVE {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const COSITED_EVEN_BIT_VALVE: Self = Self(1u32);
    pub const MIDPOINT_BIT_VALVE: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeRgbChromaOffsetFlagsVALVE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeRgbChromaOffsetFlagsVALVE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeRgbChromaOffsetFlagsVALVE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbModelConversionFlagsVALVE(pub u32);

impl VideoEncodeRgbModelConversionFlagsVALVE {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const RGB_IDENTITY_BIT_VALVE: Self = Self(1u32);
    pub const YCBCR_IDENTITY_BIT_VALVE: Self = Self(2u32);
    pub const YCBCR_709_BIT_VALVE: Self = Self(4u32);
    pub const YCBCR_601_BIT_VALVE: Self = Self(8u32);
    pub const YCBCR_2020_BIT_VALVE: Self = Self(16u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeRgbModelConversionFlagsVALVE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeRgbModelConversionFlagsVALVE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeRgbModelConversionFlagsVALVE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeRgbRangeCompressionFlagsVALVE(pub u32);

impl VideoEncodeRgbRangeCompressionFlagsVALVE {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const FULL_RANGE_BIT_VALVE: Self = Self(1u32);
    pub const NARROW_RANGE_BIT_VALVE: Self = Self(2u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeRgbRangeCompressionFlagsVALVE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeRgbRangeCompressionFlagsVALVE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeRgbRangeCompressionFlagsVALVE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEncodeUsageFlagsKHR(pub u32);

impl VideoEncodeUsageFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const DEFAULT_KHR: Self = Self(0u32);
    pub const TRANSCODING_KHR: Self = Self(1u32);
    pub const STREAMING_KHR: Self = Self(2u32);
    pub const RECORDING_KHR: Self = Self(4u32);
    pub const CONFERENCING_KHR: Self = Self(8u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEncodeUsageFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEncodeUsageFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEncodeUsageFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoEndCodingFlagsKHR(pub u32);

impl VideoEndCodingFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoEndCodingFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoEndCodingFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoEndCodingFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoSessionCreateFlagsKHR(pub u32);

impl VideoSessionCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const PROTECTED_CONTENT_KHR: Self = Self(1u32);
    pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(2u32);
    pub const INLINE_QUERIES_KHR: Self = Self(4u32);
    pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(8u32);
    pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(16u32);
    pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(32u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoSessionCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoSessionCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoSessionCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct VideoSessionParametersCreateFlagsKHR(pub u32);

impl VideoSessionParametersCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1u32);
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

impl core::ops::BitOr for VideoSessionParametersCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for VideoSessionParametersCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl core::ops::BitAnd for VideoSessionParametersCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "wayland")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct WaylandSurfaceCreateFlagsKHR(pub u32);

#[cfg(feature = "wayland")]
impl WaylandSurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "wayland")]
impl core::ops::BitOr for WaylandSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "wayland")]
impl core::ops::BitOrAssign for WaylandSurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "wayland")]
impl core::ops::BitAnd for WaylandSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "win32")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct Win32SurfaceCreateFlagsKHR(pub u32);

#[cfg(feature = "win32")]
impl Win32SurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "win32")]
impl core::ops::BitOr for Win32SurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "win32")]
impl core::ops::BitOrAssign for Win32SurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "win32")]
impl core::ops::BitAnd for Win32SurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "xcb")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct XcbSurfaceCreateFlagsKHR(pub u32);

#[cfg(feature = "xcb")]
impl XcbSurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "xcb")]
impl core::ops::BitOr for XcbSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "xcb")]
impl core::ops::BitOrAssign for XcbSurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "xcb")]
impl core::ops::BitAnd for XcbSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

#[cfg(feature = "xlib")]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, Default)]
pub struct XlibSurfaceCreateFlagsKHR(pub u32);

#[cfg(feature = "xlib")]
impl XlibSurfaceCreateFlagsKHR {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn from_raw(value: u32) -> Self {
        Self(value)
    }
    pub const fn as_raw(self) -> u32 {
        self.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub const fn contains(self, rhs: Self) -> bool {
        (self.0 & rhs.0) == rhs.0
    }
}

#[cfg(feature = "xlib")]
impl core::ops::BitOr for XlibSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

#[cfg(feature = "xlib")]
impl core::ops::BitOrAssign for XlibSurfaceCreateFlagsKHR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[cfg(feature = "xlib")]
impl core::ops::BitAnd for XlibSurfaceCreateFlagsKHR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
