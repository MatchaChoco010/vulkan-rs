#![allow(non_camel_case_types)]

use core::ffi::{CStr, c_char, c_ulong, c_void};
use core::fmt;
use core::marker::PhantomData;

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
pub type NvSciBufAttrList = *mut c_void;
pub type NvSciBufObj = *mut c_void;
pub type NvSciSyncAttrList = *mut c_void;
pub type NvSciSyncObj = *mut c_void;
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NvSciSyncFence {
    pub payload: [u64; 6],
}
pub enum OH_NativeBuffer {}
pub enum OHBufferHandle {}
pub enum OHNativeWindow {}
pub type RROutput = c_ulong;
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
pub type Flags = u32;
pub type Flags64 = u64;
pub type DeviceSize = u64;
pub type DeviceAddress = u64;
pub type SampleMask = u32;
pub type RemoteAddressNV = *mut c_void;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CStrPtr<'a> {
    ptr: *const c_char,
    _marker: PhantomData<&'a CStr>,
}

impl<'a> CStrPtr<'a> {
    pub const fn new(value: &'a CStr) -> Self {
        Self {
            ptr: value.as_ptr(),
            _marker: PhantomData,
        }
    }

    pub const unsafe fn from_ptr(ptr: *const c_char) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    pub const fn as_ptr(self) -> *const c_char {
        self.ptr
    }

    pub fn as_c_str(self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl<'a> From<&'a CStr> for CStrPtr<'a> {
    fn from(value: &'a CStr) -> Self {
        Self::new(value)
    }
}

impl fmt::Debug for CStrPtr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_c_str(), f)
    }
}

unsafe impl Send for CStrPtr<'_> {}
unsafe impl Sync for CStrPtr<'_> {}

pub trait Handle: Copy {
    const TYPE: ObjectType;
    fn as_raw_u64(self) -> u64;
    fn from_raw_u64(value: u64) -> Self;
}

pub unsafe trait Extends<Root> {}

pub unsafe trait TaggedStructure {
    const STRUCTURE_TYPE: StructureType;
}

#[repr(C)]
struct ChainHeader {
    s_type: StructureType,
    p_next: *mut c_void,
}

pub(crate) fn push_chain<T: TaggedStructure>(next: &mut T, head: *const c_void) -> *mut c_void {
    let next = core::ptr::from_mut(next).cast::<ChainHeader>();
    unsafe { (*next).p_next = head.cast_mut() };
    next.cast()
}

pub(crate) fn c_str_from_chars(
    chars: &[c_char],
) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    CStr::from_bytes_until_nul(bytes)
}

pub(crate) fn debug_enum(
    f: &mut fmt::Formatter<'_>,
    type_name: &str,
    name: Option<&str>,
    value: i32,
) -> fmt::Result {
    match name {
        Some(name) => f.write_str(name),
        None => write!(f, "{type_name}({value})"),
    }
}

pub(crate) fn name_of<T: Copy + Ord>(
    variants: &[(T, &'static str)],
    value: T,
) -> Option<&'static str> {
    variants
        .binary_search_by(|(candidate, _)| candidate.cmp(&value))
        .ok()
        .map(|index| variants[index].1)
}

pub(crate) fn value_of<T: Copy>(
    variants: &[(T, &'static str)],
    aliases: &[(T, &'static str)],
    name: &str,
) -> Option<T> {
    variants
        .iter()
        .chain(aliases)
        .find(|(_, candidate)| *candidate == name)
        .map(|(value, _)| *value)
}

pub(crate) fn debug_flags(
    f: &mut fmt::Formatter<'_>,
    value: u64,
    names: impl Iterator<Item = (u64, &'static str)>,
) -> fmt::Result {
    if value == 0 {
        return f.write_str("empty()");
    }
    let mut rest = value;
    let mut first = true;
    for (bit, name) in names {
        if bit.count_ones() == 1 && rest & bit != 0 {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            rest &= !bit;
        }
    }
    if rest != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{rest:#x}")?;
    }
    Ok(())
}

pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}

pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}

pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}

mod command_meta;
mod extension;
mod format;
pub mod generated;

pub use command_meta::*;
pub use extension::*;
pub use format::*;
pub use generated::*;

pub type Result<T> = core::result::Result<T, VkResult>;

impl VkResult {
    pub const fn into_result(self) -> Result<()> {
        if self.0 >= 0 { Ok(()) } else { Err(self) }
    }

    pub const fn is_error(self) -> bool {
        self.0 < 0
    }
}

impl fmt::Display for VkResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl core::error::Error for VkResult {}
