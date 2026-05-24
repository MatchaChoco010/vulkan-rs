#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod vk;

use core::ffi::c_char;

pub struct Entry {
    static_fn: StaticFn,
    fp: vk::EntryFn,
    #[cfg(feature = "std")]
    _library: Option<libloading::Library>,
}

#[derive(Copy, Clone)]
pub struct StaticFn {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

pub struct Instance {
    handle: vk::Instance,
    fp: vk::InstanceFn,
}

pub struct Device {
    handle: vk::Device,
    fp: vk::DeviceFn,
}

impl Entry {
    #[cfg(feature = "std")]
    /// Loads the Vulkan loader from the platform default dynamic library.
    ///
    /// # Safety
    ///
    /// The loaded Vulkan loader must be ABI-compatible with the generated bindings.
    /// Calling through function pointers from an incompatible loader is undefined behavior.
    pub unsafe fn load() -> core::result::Result<Self, LoadError> {
        unsafe { Self::load_from(default_vulkan_loader_name()) }
    }

    #[cfg(feature = "std")]
    /// Loads the Vulkan loader from a dynamic library path.
    ///
    /// # Safety
    ///
    /// `path` must identify a Vulkan loader whose exported functions are ABI-compatible
    /// with these bindings. The library is kept alive by the returned `Entry`.
    pub unsafe fn load_from(
        path: impl AsRef<std::ffi::OsStr>,
    ) -> core::result::Result<Self, LoadError> {
        let library = unsafe { libloading::Library::new(path) }.map_err(LoadError)?;
        let static_fn = {
            let symbol: libloading::Symbol<
                unsafe extern "system" fn(vk::Instance, *const c_char) -> vk::PFN_vkVoidFunction,
            > = unsafe { library.get(b"vkGetInstanceProcAddr\0") }.map_err(LoadError)?;
            StaticFn {
                get_instance_proc_addr: Some(*symbol),
            }
        };
        let fp = unsafe { vk::EntryFn::load(static_fn.get_instance_proc_addr) };
        Ok(Self {
            static_fn,
            fp,
            _library: Some(library),
        })
    }

    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr: Some(vkGetInstanceProcAddr),
            })
        }
    }

    /// Builds an entry dispatch table from a user supplied `vkGetInstanceProcAddr`.
    ///
    /// Applications can use this when the Vulkan loader is supplied by an embedding
    /// environment instead of the platform dynamic library.
    ///
    /// # Safety
    ///
    /// `get_instance_proc_addr` must be a valid Vulkan loader function pointer and
    /// must remain callable for the lifetime of the returned `Entry`.
    pub unsafe fn from_get_instance_proc_addr(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr,
            })
        }
    }

    /// Builds an entry dispatch table from statically supplied loader functions.
    ///
    /// # Safety
    ///
    /// Every function pointer in `static_fn` must be valid, ABI-compatible, and
    /// remain callable for the lifetime of the returned `Entry`.
    pub unsafe fn from_static_fn(static_fn: StaticFn) -> Self {
        let fp = unsafe { vk::EntryFn::load(static_fn.get_instance_proc_addr) };
        Self {
            static_fn,
            fp,
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

    /// Creates a Vulkan instance and loads its instance-level dispatch table.
    ///
    /// # Safety
    ///
    /// The caller must satisfy all Vulkan validity requirements for
    /// `vkCreateInstance`, including the validity of pointers reachable from
    /// `create_info` and `allocator`.
    pub unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> vk::Result<Instance> {
        let mut handle = vk::Instance::null();
        let result = unsafe {
            (self
                .fp
                .create_instance
                .expect("vkCreateInstance is not loaded"))(
                create_info,
                allocator.map_or(core::ptr::null(), |x| x),
                &mut handle,
            )
        };
        result.into_result()?;

        let fp = unsafe { vk::InstanceFn::load(self.fp.get_instance_proc_addr, handle) };
        Ok(Instance { handle, fp })
    }
}

impl Instance {
    pub fn from_raw(handle: vk::Instance, fp: vk::InstanceFn) -> Self {
        Self { handle, fp }
    }

    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    pub fn fp(&self) -> &vk::InstanceFn {
        &self.fp
    }

    /// Destroys the wrapped Vulkan instance.
    ///
    /// # Safety
    ///
    /// The instance must not be used after this call, and all Vulkan objects that
    /// must be destroyed before the instance must already have been destroyed.
    pub unsafe fn destroy_instance(&self, allocator: Option<&vk::AllocationCallbacks<'_>>) {
        unsafe {
            (self
                .fp
                .destroy_instance
                .expect("vkDestroyInstance is not loaded"))(
                self.handle,
                allocator.map_or(core::ptr::null(), |x| x),
            );
        }
    }

    /// Creates a Vulkan device and loads its device-level dispatch table.
    ///
    /// # Safety
    ///
    /// The caller must satisfy all Vulkan validity requirements for
    /// `vkCreateDevice`, including valid queue create infos, feature chains, and
    /// allocator pointers.
    pub unsafe fn create_device(
        &self,
        physical_device: vk::PhysicalDevice,
        create_info: &vk::DeviceCreateInfo<'_>,
        allocator: Option<&vk::AllocationCallbacks<'_>>,
    ) -> vk::Result<Device> {
        let mut handle = vk::Device::null();
        let result = unsafe {
            (self.fp.create_device.expect("vkCreateDevice is not loaded"))(
                physical_device,
                create_info,
                allocator.map_or(core::ptr::null(), |x| x),
                &mut handle,
            )
        };
        result.into_result()?;

        let fp = unsafe { vk::DeviceFn::load(self.fp.get_device_proc_addr, handle) };
        Ok(Device { handle, fp })
    }
}

impl Device {
    pub fn from_raw(handle: vk::Device, fp: vk::DeviceFn) -> Self {
        Self { handle, fp }
    }

    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    pub fn fp(&self) -> &vk::DeviceFn {
        &self.fp
    }

    /// Destroys the wrapped Vulkan device.
    ///
    /// # Safety
    ///
    /// The device must not be used after this call, and all Vulkan objects that
    /// must be destroyed before the device must already have been destroyed.
    pub unsafe fn destroy_device(&self, allocator: Option<&vk::AllocationCallbacks<'_>>) {
        unsafe {
            (self
                .fp
                .destroy_device
                .expect("vkDestroyDevice is not loaded"))(
                self.handle,
                allocator.map_or(core::ptr::null(), |x| x),
            );
        }
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct LoadError(libloading::Error);

#[cfg(feature = "std")]
impl core::fmt::Display for LoadError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to load Vulkan loader: {}", self.0)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LoadError {}

#[cfg(feature = "linked")]
unsafe extern "system" {
    fn vkGetInstanceProcAddr(instance: vk::Instance, name: *const c_char)
    -> vk::PFN_vkVoidFunction;
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
    unix,
    not(any(target_os = "macos", target_os = "ios"))
))]
fn default_vulkan_loader_name() -> &'static str {
    "libvulkan.so.1"
}

#[cfg(all(
    feature = "std",
    not(any(target_os = "windows", unix, target_os = "macos", target_os = "ios"))
))]
fn default_vulkan_loader_name() -> &'static str {
    "vulkan"
}

pub(crate) unsafe fn load_pfn<T>(pfn: vk::PFN_vkVoidFunction) -> T {
    debug_assert_eq!(
        core::mem::size_of::<T>(),
        core::mem::size_of::<vk::PFN_vkVoidFunction>()
    );
    unsafe { core::mem::transmute_copy(&pfn) }
}

pub(crate) fn c_name_ptr(name: &'static [u8]) -> *const c_char {
    name.as_ptr().cast()
}
