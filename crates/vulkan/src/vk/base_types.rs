use core::ffi::c_void;

/// `VkBool32`: [`TRUE`](super::TRUE) or [`FALSE`](super::FALSE).
///
/// Struct fields hold this type; setters and command wrappers take and return `bool`.
pub type Bool32 = u32;
/// `VkFlags`: the 32-bit integer underlying the bitmask types.
pub type Flags = u32;
/// `VkFlags64`: the 64-bit integer underlying the 64-bit bitmask types.
pub type Flags64 = u64;
/// `VkDeviceSize`: a size or offset in device memory, in bytes.
pub type DeviceSize = u64;
/// `VkDeviceAddress`: an address in device memory.
pub type DeviceAddress = u64;
/// `VkSampleMask`: one 32-bit word of a sample mask.
pub type SampleMask = u32;
/// `VkRemoteAddressNV`: a pointer to memory of a remote device (`void*`).
pub type RemoteAddressNV = *mut c_void;
