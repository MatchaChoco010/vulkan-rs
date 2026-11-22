use core::ffi::{CStr, c_char};
use core::fmt;
use core::marker::PhantomData;

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

pub(crate) fn c_str_from_chars(
    chars: &[c_char],
) -> core::result::Result<&CStr, core::ffi::FromBytesUntilNulError> {
    let bytes = unsafe { core::slice::from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
    CStr::from_bytes_until_nul(bytes)
}
