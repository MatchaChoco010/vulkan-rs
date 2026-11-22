#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod vk;

use alloc::sync::Arc;

#[cfg(feature = "std")]
type Library = Option<Arc<libloading::Library>>;

#[derive(Clone)]
pub struct Entry {
    static_fn: StaticFn,
    fp: Arc<vk::EntryFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

#[derive(Copy, Clone)]
pub struct StaticFn {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: Arc<vk::InstanceFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: Arc<vk::DeviceFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

impl Entry {
    #[cfg(feature = "std")]
    pub unsafe fn load() -> core::result::Result<Self, LoadError> {
        unsafe { Self::load_from(default_vulkan_loader_name()) }
    }

    #[cfg(feature = "std")]
    pub unsafe fn load_from(
        path: impl AsRef<std::ffi::OsStr>,
    ) -> core::result::Result<Self, LoadError> {
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
        let mut entry = unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr,
            })
        };
        entry._library = Some(Arc::new(library));
        Ok(entry)
    }

    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr: Some(vkGetInstanceProcAddr),
            })
        }
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
            #[cfg(feature = "std")]
            _library: None,
        }
    }

    pub fn static_fn(&self) -> &StaticFn {
        &self.static_fn
    }

    pub fn fp(&self) -> &vk::EntryFn {
        &self.fp
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

impl Instance {
    pub unsafe fn load(entry: &Entry, handle: vk::Instance) -> Self {
        let fp = unsafe { vk::InstanceFn::load(entry.static_fn.get_instance_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: entry._library.clone(),
        }
    }

    pub fn from_raw(handle: vk::Instance, fp: vk::InstanceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: None,
        }
    }

    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    pub fn fp(&self) -> &vk::InstanceFn {
        &self.fp
    }

    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> vk::Result<Device> {
        let mut handle = vk::Device::null();
        unsafe {
            self.create_device_raw(
                physical_device,
                create_info,
                allocator.map_or(core::ptr::null(), core::ptr::from_ref),
                &mut handle,
            )
        }
        .into_result()?;
        Ok(unsafe { Device::load(self, handle) })
    }
}

impl Device {
    pub unsafe fn load(instance: &Instance, handle: vk::Device) -> Self {
        let fp = unsafe { vk::DeviceFn::load(instance.fp.get_device_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: instance._library.clone(),
        }
    }

    pub fn from_raw(handle: vk::Device, fp: vk::DeviceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: None,
        }
    }

    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    pub fn fp(&self) -> &vk::DeviceFn {
        &self.fp
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
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        name: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

#[cfg(all(feature = "std", target_os = "windows"))]
fn default_vulkan_loader_name() -> &'static str {
    "vulkan-1.dll"
}

#[cfg(all(feature = "std", any(target_os = "macos", target_os = "ios")))]
fn default_vulkan_loader_name() -> &'static str {
    "libvulkan.dylib"
}

#[cfg(all(
    feature = "std",
    not(any(target_os = "windows", target_os = "macos", target_os = "ios"))
))]
fn default_vulkan_loader_name() -> &'static str {
    "libvulkan.so.1"
}

pub(crate) unsafe fn load_pfn<T>(pfn: vk::PFN_vkVoidFunction) -> T {
    const {
        assert!(core::mem::size_of::<T>() == core::mem::size_of::<vk::PFN_vkVoidFunction>());
    }
    unsafe { core::mem::transmute_copy(&pfn) }
}
