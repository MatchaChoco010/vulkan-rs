use core::ffi::{CStr, c_char};
use core::fmt;
use core::marker::PhantomData;

/// A borrowed, NUL-terminated C string with the layout of `const char*`.
///
/// String arrays such as `ppEnabledExtensionNames` are set from `&'a [CStrPtr<'a>]`. A
/// `CStrPtr` has the size, alignment and ABI of `*const c_char` (it is
/// `#[repr(transparent)]`), so the slice can be passed where C expects
/// `const char* const*`, and its lifetime keeps the struct from outliving the strings.
///
/// A `CStrPtr` is never null. It is `Copy`, `Send` and `Sync`, converts from `&CStr` with
/// `From`, and its `Debug` form is that of the string.
///
/// ```
/// use vulkan::vk;
///
/// let names = [
///     vk::CStrPtr::new(vk::KHR_SWAPCHAIN_NAME),
///     vk::EXT_HDR_METADATA_NAME.into(),
/// ];
/// let info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
/// assert_eq!(info.enabled_extension_count, 2);
/// assert_eq!(names[1].as_c_str(), c"VK_EXT_hdr_metadata");
/// ```
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CStrPtr<'a> {
    ptr: *const c_char,
    _marker: PhantomData<&'a CStr>,
}

impl<'a> CStrPtr<'a> {
    /// Borrows `value`.
    pub const fn new(value: &'a CStr) -> Self {
        Self {
            ptr: value.as_ptr(),
            _marker: PhantomData,
        }
    }

    /// Wraps a raw C string pointer, for example one read from a struct that Vulkan filled.
    ///
    /// # Safety
    ///
    /// `ptr` must be non-null and point to a NUL-terminated string that is valid for reads
    /// up to and including the NUL, and that is not modified or freed during `'a`. The
    /// returned value can be sent to and read from other threads, so this must also hold
    /// for reads from any thread.
    pub const unsafe fn from_ptr(ptr: *const c_char) -> Self {
        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// The pointer to the first character, valid for `'a`.
    pub const fn as_ptr(self) -> *const c_char {
        self.ptr
    }

    /// The string as a [`CStr`]. This scans the string for its terminating NUL.
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

pub(crate) fn c_str_from_chars(
    chars: &[c_char],
) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    CStr::from_bytes_until_nul(bytes)
}
