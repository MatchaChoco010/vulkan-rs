#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use core::ffi::{c_ulong, c_void};

pub enum AHardwareBuffer {}
pub enum ANativeWindow {}
pub type CAMetalLayer = c_void;
pub enum Display {}
pub type DWORD = u32;
pub type GgpFrameToken = u64;
pub type GgpStreamDescriptor = u64;
pub type HANDLE = *mut c_void;
pub type HINSTANCE = *mut c_void;
pub type HMONITOR = *mut c_void;
pub type HWND = *mut c_void;
pub enum IDirectFB {}
pub enum IDirectFBSurface {}
pub enum IOSurface {}
pub type IOSurfaceRef = *mut IOSurface;
pub type LPCWSTR = *const u16;
pub type MTLBuffer_id = *mut c_void;
pub type MTLCommandQueue_id = *mut c_void;
pub type MTLDevice_id = *mut c_void;
pub type MTLSharedEvent_id = *mut c_void;
pub type MTLTexture_id = *mut c_void;
pub enum OH_NativeBuffer {}
pub enum OHNativeWindow {}
pub type RROutput = c_ulong;
pub type RemoteAddressNV = *mut c_void;
pub enum SECURITY_ATTRIBUTES {}
pub type VisualID = c_ulong;
pub type Window = c_ulong;
pub enum _screen_buffer {}
pub enum _screen_context {}
pub enum _screen_window {}
pub enum ubm_device {}
pub enum ubm_surface {}
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub enum xcb_connection_t {}
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type zx_handle_t = u32;
pub type Bool32 = u32;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Packed24_8(pub u32);

impl Packed24_8 {
    pub const fn new(low_24: u32, high_8: u8) -> Self {
        Self((low_24 & 0x00ff_ffff) | ((high_8 as u32) << 24))
    }

    pub const fn low_24(self) -> u32 {
        self.0 & 0x00ff_ffff
    }

    pub const fn high_8(self) -> u8 {
        (self.0 >> 24) as u8
    }
}

/// # Safety
///
/// Implementors must be valid Vulkan structures for the `Root` pNext chain according to
/// Vulkan's `structextends` metadata.
pub unsafe trait Extends<Root> {}

/// # Safety
///
/// Implementors must be Vulkan structures whose first two fields are compatible with
/// `VkBaseOutStructure`/`VkBaseInStructure` and whose `sType` value matches `STRUCTURE_TYPE`.
pub unsafe trait TaggedStructure {
    const STRUCTURE_TYPE: StructureType;

    /// # Safety
    ///
    /// `next` must either be null or point to a valid pNext chain node with a lifetime that
    /// outlives the structure being modified.
    unsafe fn set_p_next(&mut self, next: *mut c_void);
}

#[repr(C)]
pub struct PNextOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}

pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

pub mod generated;

pub use generated::*;

pub type Result<T> = core::result::Result<T, VkResult>;

impl core::fmt::Display for VkResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Vulkan error {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for VkResult {}
