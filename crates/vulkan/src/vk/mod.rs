//! Vulkan types, constants and function pointer tables.
//!
//! Everything in [`generated`] comes from the registry. This module holds the small
//! hand-written base: platform types, base typedefs and the traits the generated code
//! builds on.

#![allow(non_camel_case_types)]

use core::ffi::{CStr, c_char, c_ulong, c_void};
use core::fmt;
use core::marker::PhantomData;

// Platform types. They come from platform headers, not from the registry, so they are
// declared here as opaque types or with their documented underlying type. Opaque types are
// only ever used behind raw pointers.
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
/// `NvSciSyncFence` from `nvscisync.h`: six 64-bit payload words.
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
/// Aliases of `c_void` so that pointers from windowing crates need no cast.
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub enum xcb_connection_t {}
pub type xcb_visualid_t = u32;
pub type xcb_window_t = u32;
pub type zx_handle_t = u32;

/// `VkBool32`. Setters and command wrappers take and return `bool`; the raw field holds
/// [`TRUE`] or [`FALSE`].
pub type Bool32 = u32;
/// `VkFlags`
pub type Flags = u32;
/// `VkFlags64`
pub type Flags64 = u64;
/// `VkDeviceSize`
pub type DeviceSize = u64;
/// `VkDeviceAddress`
pub type DeviceAddress = u64;
/// `VkSampleMask`
pub type SampleMask = u32;
/// `VkRemoteAddressNV`
pub type RemoteAddressNV = *mut c_void;

/// A `const char*` that remembers how long the string it points to lives.
///
/// Arrays of strings such as `ppEnabledExtensionNames` are `&'a [CStrPtr<'a>]`, so the
/// struct holding them cannot outlive the strings. It has the layout of `*const c_char`.
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

    /// Wraps a raw C string pointer.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a NUL-terminated string that stays valid and unmodified for `'a`.
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
        // SAFETY: both constructors guarantee a NUL-terminated string valid for `'a`.
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

// SAFETY: it is a shared borrow of an immutable string.
unsafe impl Send for CStrPtr<'_> {}
unsafe impl Sync for CStrPtr<'_> {}

/// Implemented by every Vulkan handle type.
pub trait Handle: Copy {
    /// The `VkObjectType` of this handle, e.g. for `VK_EXT_debug_utils` object names.
    const TYPE: ObjectType;
    fn as_raw_u64(self) -> u64;
    fn from_raw_u64(value: u64) -> Self;
}

/// Marks a struct that may appear in the `pNext` chain of `Root`.
///
/// # Safety
///
/// The registry must list the implementing struct as extending `Root` (`structextends`).
pub unsafe trait Extends<Root> {}

/// A struct that starts with `sType` and `pNext`.
///
/// # Safety
///
/// The implementing type must be `#[repr(C)]` and start with a [`StructureType`] field
/// followed by a pointer-sized `pNext` field, and `STRUCTURE_TYPE` must be its `sType`.
pub unsafe trait TaggedStructure {
    const STRUCTURE_TYPE: StructureType;
}

/// Common prefix of every [`TaggedStructure`].
#[repr(C)]
struct ChainHeader {
    s_type: StructureType,
    p_next: *mut c_void,
}

/// Links `next` in front of the chain that starts at `head` and returns the new head.
///
/// The previous `next.p_next` is overwritten, never followed: following it would mean
/// dereferencing a pointer that safe code may have set to anything.
pub(crate) fn push_chain<T: TaggedStructure>(next: &mut T, head: *const c_void) -> *mut c_void {
    let next = core::ptr::from_mut(next).cast::<ChainHeader>();
    // SAFETY: `TaggedStructure` guarantees that `T` starts with `ChainHeader`'s fields,
    // and `next` comes from a unique reference.
    unsafe { (*next).p_next = head.cast_mut() };
    next.cast()
}

/// Reads a fixed-size `char` array up to its first NUL.
pub(crate) fn c_str_from_chars(
    chars: &[c_char],
) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {
    // SAFETY: `c_char` is `i8` or `u8`; both have the size, alignment and validity of `u8`.
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    CStr::from_bytes_until_nul(bytes)
}

pub(crate) fn debug_enum(
    f: &mut fmt::Formatter<'_>,
    type_name: &str,
    value: i32,
    names: &[(i32, &str)],
) -> fmt::Result {
    match names.iter().find(|(known, _)| *known == value) {
        Some((_, name)) => f.write_str(name),
        None => write!(f, "{type_name}({value})"),
    }
}

/// Prints set flag bits by name, joined with ` | `, and unknown bits as one hex number.
fn debug_flags(
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
        if rest & bit != 0 {
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

pub(crate) fn debug_flags64(
    f: &mut fmt::Formatter<'_>,
    value: u64,
    names: &[(u64, &'static str)],
) -> fmt::Result {
    debug_flags(f, value, names.iter().copied())
}

pub(crate) fn debug_flags32(
    f: &mut fmt::Formatter<'_>,
    value: u32,
    names: &[(u32, &'static str)],
) -> fmt::Result {
    debug_flags(
        f,
        u64::from(value),
        names.iter().map(|&(bit, name)| (u64::from(bit), name)),
    )
}

/// `VK_MAKE_API_VERSION`
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

/// `VK_API_VERSION_VARIANT`
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}

/// `VK_API_VERSION_MAJOR`
pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}

/// `VK_API_VERSION_MINOR`
pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

/// `VK_API_VERSION_PATCH`
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}

mod extension;
mod format;
pub mod generated;

pub use extension::*;
pub use format::*;
pub use generated::*;

/// Result of a Vulkan command: `Err` holds an error code (negative `VkResult`).
///
/// Commands with more than one success code return the code inside `Ok`.
pub type Result<T> = core::result::Result<T, VkResult>;

impl VkResult {
    /// `Ok(())` for every success code (non-negative), `Err(self)` for error codes.
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
