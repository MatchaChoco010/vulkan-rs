#![allow(non_camel_case_types)]

use core::ffi::{c_ulong, c_void};

/// Android's `AHardwareBuffer` (`<android/hardware_buffer.h>`), opaque, used behind pointers.
pub enum AHardwareBuffer {}
/// Android's `ANativeWindow` (`<android/native_window.h>`), opaque, used behind pointers.
pub enum ANativeWindow {}
/// Core Animation's `CAMetalLayer`, which the C headers declare as `void` outside Objective-C.
pub type CAMetalLayer = c_void;
/// Xlib's `Display` (`<X11/Xlib.h>`), opaque, used behind pointers.
pub enum Display {}
/// Win32 `DWORD`: a 32-bit unsigned integer.
pub type DWORD = u32;
/// `GgpFrameToken` of the Google Games Platform (`<ggp_c/vulkan_types.h>`).
pub type GgpFrameToken = u64;
/// `GgpStreamDescriptor` of the Google Games Platform (`<ggp_c/vulkan_types.h>`).
pub type GgpStreamDescriptor = u64;
/// Win32 `HANDLE`.
pub type HANDLE = *mut c_void;
/// Win32 `HINSTANCE`: a module instance handle.
pub type HINSTANCE = *mut c_void;
/// Win32 `HMONITOR`: a monitor handle.
pub type HMONITOR = *mut c_void;
/// Win32 `HWND`: a window handle.
pub type HWND = *mut c_void;
/// DirectFB's `IDirectFB` interface (`<directfb.h>`), opaque, used behind pointers.
pub enum IDirectFB {}
/// DirectFB's `IDirectFBSurface` interface (`<directfb.h>`), opaque, used behind pointers.
pub enum IDirectFBSurface {}
/// The IOSurface object that [`IOSurfaceRef`] points to, opaque.
pub enum IOSurface {}
/// Apple's `IOSurfaceRef` (`<IOSurface/IOSurfaceRef.h>`).
pub type IOSurfaceRef = *mut IOSurface;
/// Win32 `LPCWSTR`: a pointer to a NUL-terminated UTF-16 string.
pub type LPCWSTR = *const u16;
/// `MTLBuffer_id`: an `id<MTLBuffer>`, `void*` outside Objective-C.
pub type MTLBuffer_id = *mut c_void;
/// `MTLCommandQueue_id`: an `id<MTLCommandQueue>`, `void*` outside Objective-C.
pub type MTLCommandQueue_id = *mut c_void;
/// `MTLDevice_id`: an `id<MTLDevice>`, `void*` outside Objective-C.
pub type MTLDevice_id = *mut c_void;
/// `MTLSharedEvent_id`: an `id<MTLSharedEvent>`, `void*` outside Objective-C.
pub type MTLSharedEvent_id = *mut c_void;
/// `MTLTexture_id`: an `id<MTLTexture>`, `void*` outside Objective-C.
pub type MTLTexture_id = *mut c_void;
/// NvSciBuf's `NvSciBufAttrList` (`<nvscibuf.h>`).
pub type NvSciBufAttrList = *mut c_void;
/// NvSciBuf's `NvSciBufObj` (`<nvscibuf.h>`).
pub type NvSciBufObj = *mut c_void;
/// NvSciSync's `NvSciSyncAttrList` (`<nvscisync.h>`).
pub type NvSciSyncAttrList = *mut c_void;
/// NvSciSync's `NvSciSyncObj` (`<nvscisync.h>`).
pub type NvSciSyncObj = *mut c_void;
/// NvSciSync's `NvSciSyncFence` (`<nvscisync.h>`): six 64-bit payload words.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NvSciSyncFence {
    /// The words of the fence, as NvSciSync defines them.
    pub payload: [u64; 6],
}
/// OpenHarmony's `OH_NativeBuffer`, opaque, used behind pointers.
pub enum OH_NativeBuffer {}
/// OpenHarmony's `OHBufferHandle`, opaque, used behind pointers.
pub enum OHBufferHandle {}
/// OpenHarmony's `OHNativeWindow`, opaque, used behind pointers.
pub enum OHNativeWindow {}
/// XRandR's `RROutput` (`<X11/extensions/Xrandr.h>`): an X resource ID.
pub type RROutput = c_ulong;
/// Win32 `SECURITY_ATTRIBUTES`, opaque, used behind pointers.
pub enum SECURITY_ATTRIBUTES {}
/// Xlib's `VisualID`.
pub type VisualID = c_ulong;
/// Xlib's `Window`: an X resource ID.
pub type Window = c_ulong;
/// QNX Screen's `struct _screen_buffer`, opaque, used behind pointers.
pub enum _screen_buffer {}
/// QNX Screen's `struct _screen_context`, opaque, used behind pointers.
pub enum _screen_context {}
/// QNX Screen's `struct _screen_window`, opaque, used behind pointers.
pub enum _screen_window {}
/// UBM's `struct ubm_device`, opaque, used behind pointers.
pub enum ubm_device {}
/// UBM's `struct ubm_surface`, opaque, used behind pointers.
pub enum ubm_surface {}
/// Wayland's `struct wl_display`, as `c_void`.
pub type wl_display = c_void;
/// Wayland's `struct wl_surface`, as `c_void`.
pub type wl_surface = c_void;
/// XCB's `xcb_connection_t` (`<xcb/xcb.h>`), opaque, used behind pointers.
pub enum xcb_connection_t {}
/// XCB's `xcb_visualid_t`.
pub type xcb_visualid_t = u32;
/// XCB's `xcb_window_t`.
pub type xcb_window_t = u32;
/// Fuchsia's Zircon `zx_handle_t`.
pub type zx_handle_t = u32;
