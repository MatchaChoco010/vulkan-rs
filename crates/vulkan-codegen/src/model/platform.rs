const OPAQUE_PLATFORM_TYPES: &[&str] = &[
    "AHardwareBuffer",
    "ANativeWindow",
    "CAMetalLayer",
    "Display",
    "IDirectFB",
    "IDirectFBSurface",
    "OH_NativeBuffer",
    "OHBufferHandle",
    "OHNativeWindow",
    "SECURITY_ATTRIBUTES",
    "_screen_buffer",
    "_screen_context",
    "_screen_window",
    "ubm_device",
    "ubm_surface",
    "wl_display",
    "wl_surface",
    "xcb_connection_t",
];

const PLATFORM_VALUE_TYPES: &[&str] = &[
    "DWORD",
    "GgpFrameToken",
    "GgpStreamDescriptor",
    "HANDLE",
    "HINSTANCE",
    "HMONITOR",
    "HWND",
    "IOSurfaceRef",
    "LPCWSTR",
    "MTLBuffer_id",
    "MTLCommandQueue_id",
    "MTLDevice_id",
    "MTLSharedEvent_id",
    "MTLTexture_id",
    "NvSciBufAttrList",
    "NvSciBufObj",
    "NvSciSyncAttrList",
    "NvSciSyncFence",
    "NvSciSyncObj",
    "RROutput",
    "VisualID",
    "Window",
    "xcb_visualid_t",
    "xcb_window_t",
    "zx_handle_t",
];

pub(crate) fn is_platform_type(name: &str) -> bool {
    OPAQUE_PLATFORM_TYPES.contains(&name) || PLATFORM_VALUE_TYPES.contains(&name)
}

pub(crate) fn is_opaque(name: &str) -> bool {
    name == "void" || OPAQUE_PLATFORM_TYPES.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_value_typedefs_are_not_opaque() {
        assert!(is_opaque("void"));
        assert!(is_opaque("wl_display"));
        assert!(is_opaque("Display"));
        assert!(is_platform_type("HANDLE") && !is_opaque("HANDLE"));
        assert!(!is_opaque("zx_handle_t"));
        assert!(!is_opaque("uint32_t"));
        assert!(!is_platform_type("uint32_t"));
    }
}
