use crate::vk;

#[derive(Clone, Default)]
pub(crate) struct LoaderLibrary {
    #[cfg(feature = "std")]
    _kept_loaded: Option<alloc::sync::Arc<libloading::Library>>,
}

#[cfg(feature = "std")]
impl LoaderLibrary {
    const DEFAULT_PATH: &str = if cfg!(target_os = "windows") {
        "vulkan-1.dll"
    } else if cfg!(any(target_os = "macos", target_os = "ios")) {
        "libvulkan.dylib"
    } else {
        "libvulkan.so.1"
    };

    pub(crate) unsafe fn open(
        path: Option<&std::ffi::OsStr>,
    ) -> Result<(Self, vk::PFN_vkGetInstanceProcAddr), LoadError> {
        let path = path.unwrap_or(Self::DEFAULT_PATH.as_ref());
        let library = unsafe { libloading::Library::new(path) }.map_err(LoadError)?;
        let get_instance_proc_addr = {
            let symbol: libloading::Symbol<
                unsafe extern "system" fn(
                    vk::Instance,
                    *const core::ffi::c_char,
                ) -> vk::PFN_vkVoidFunction,
            > = unsafe { library.get(b"vkGetInstanceProcAddr\0") }.map_err(LoadError)?;
            Some(*symbol)
        };
        let library = Self {
            _kept_loaded: Some(alloc::sync::Arc::new(library)),
        };
        Ok((library, get_instance_proc_addr))
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct LoadError(libloading::Error);

#[cfg(feature = "std")]
impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to load the Vulkan loader: {}", self.0)
    }
}

#[cfg(feature = "std")]
impl core::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        Some(&self.0)
    }
}

#[cfg(feature = "linked")]
#[cfg_attr(target_os = "windows", link(name = "vulkan-1"))]
#[cfg_attr(not(target_os = "windows"), link(name = "vulkan"))]
unsafe extern "system" {
    pub(crate) fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        name: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

pub(crate) unsafe fn load_pfn<T>(pfn: vk::PFN_vkVoidFunction) -> T {
    const {
        assert!(core::mem::size_of::<T>() == core::mem::size_of::<vk::PFN_vkVoidFunction>());
    }
    unsafe { core::mem::transmute_copy(&pfn) }
}
