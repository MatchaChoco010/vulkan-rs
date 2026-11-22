use crate::loader::LoaderLibrary;
use crate::{Instance, vk};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct Entry {
    static_fn: StaticFn,
    fp: Arc<vk::EntryFn>,
    library: LoaderLibrary,
}

#[derive(Copy, Clone)]
pub struct StaticFn {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

impl Entry {
    #[cfg(feature = "std")]
    pub unsafe fn load() -> core::result::Result<Self, crate::LoadError> {
        let (library, get_instance_proc_addr) = unsafe { LoaderLibrary::open(None) }?;
        Ok(unsafe { Self::with_library(get_instance_proc_addr, library) })
    }

    #[cfg(feature = "std")]
    pub unsafe fn load_from(
        path: impl AsRef<std::ffi::OsStr>,
    ) -> core::result::Result<Self, crate::LoadError> {
        let (library, get_instance_proc_addr) =
            unsafe { LoaderLibrary::open(Some(path.as_ref())) }?;
        Ok(unsafe { Self::with_library(get_instance_proc_addr, library) })
    }

    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        unsafe { Self::from_get_instance_proc_addr(Some(crate::loader::vkGetInstanceProcAddr)) }
    }

    pub unsafe fn from_get_instance_proc_addr(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr,
            })
        }
    }

    pub unsafe fn from_static_fn(static_fn: StaticFn) -> Self {
        let fp = unsafe { vk::EntryFn::load(static_fn.get_instance_proc_addr) };
        Self {
            static_fn,
            fp: Arc::new(fp),
            library: LoaderLibrary::default(),
        }
    }

    #[cfg(feature = "std")]
    unsafe fn with_library(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
        library: LoaderLibrary,
    ) -> Self {
        Self {
            library,
            ..unsafe { Self::from_get_instance_proc_addr(get_instance_proc_addr) }
        }
    }

    pub fn static_fn(&self) -> &StaticFn {
        &self.static_fn
    }

    pub fn fp(&self) -> &vk::EntryFn {
        &self.fp
    }

    pub(crate) fn library(&self) -> &LoaderLibrary {
        &self.library
    }

    pub unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> vk::Result<Instance> {
        let mut handle = vk::Instance::null();
        unsafe {
            self.create_instance_raw(
                create_info,
                allocator.map_or(core::ptr::null(), core::ptr::from_ref),
                &mut handle,
            )
        }
        .into_result()?;
        Ok(unsafe { Instance::load(self, handle) })
    }
}
