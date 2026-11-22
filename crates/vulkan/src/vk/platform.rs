#![allow(non_camel_case_types)]

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
