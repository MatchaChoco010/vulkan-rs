//! Pure Rust Vulkan bindings generated from Khronos Vulkan-Headers.
//!
//! - [`vk`] holds the types, constants and function pointer tables, one to one with the C API.
//! - [`Entry`], [`Instance`] and [`Device`] own a function pointer table each and expose
//!   every command as a method.
//!
//! # Borrowing
//!
//! Structs that can hold pointers carry a lifetime, and their setters take references
//! and slices with that lifetime. A struct therefore cannot outlive the data it points to:
//!
//! ```compile_fail,E0597
//! use vulkan::vk;
//! let info;
//! {
//!     let priorities = [1.0];
//!     info = vk::DeviceQueueCreateInfo::default().queue_priorities(&priorities);
//! }
//! let _ = info; // error: `priorities` does not live long enough
//! ```
//!
//! `pNext` chains follow the same rule, and only accept structs that the registry lists as
//! extending the root:
//!
//! ```
//! use vulkan::vk;
//! let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true);
//! let info = vk::DeviceCreateInfo::default().push_next(&mut vulkan12);
//! assert!(!info.p_next.is_null());
//! ```
//!
//! ```compile_fail,E0277
//! use vulkan::vk;
//! let mut present = vk::PresentInfoKHR::default();
//! // error: `PresentInfoKHR: Extends<DeviceCreateInfo>` is not satisfied
//! let info = vk::DeviceCreateInfo::default().push_next(&mut present);
//! ```
//!
//! There is no `build()` step that would erase the lifetime. The fields are public for
//! reading results and for the rare shape that has no setter; writing a pointer field
//! directly bypasses the lifetime.
//!
//! # Commands
//!
//! Every command is an `unsafe fn`: the compiler cannot check Vulkan's valid usage rules.
//! Each command comes in two forms.
//!
//! - `device.create_buffer(&info, None)`: slices, references, `vk::Result` and returned
//!   values.
//! - `device.create_buffer_raw(&info, ptr::null(), &mut buffer)`: the C signature.
//!
//! All commands of a level are loaded when the [`Instance`] or [`Device`] is created.
//! Calling a command that the loader did not provide, such as a command of an extension
//! that was not enabled, panics.
//!
//! # Features
//!
//! - `std` (default): [`Entry::load`] loads the Vulkan loader at runtime.
//! - `linked`: [`Entry::linked`] links the Vulkan loader at build time.
//! - `debug` (default): `Debug` for structs.
//! - `wayland`, `xcb`, `xlib`, `win32`, ...: platform types and commands.
//! - `beta`: provisional extensions.

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod vk;

use alloc::sync::Arc;

/// Keeps the dynamically loaded Vulkan loader alive.
///
/// [`Instance`] and [`Device`] hold a clone, so their function pointers stay valid even
/// if the [`Entry`] is dropped first.
#[cfg(feature = "std")]
type Library = Option<Arc<libloading::Library>>;

/// The Vulkan loader and its global commands.
///
/// Cloning is cheap: the function pointer table is shared.
#[derive(Clone)]
pub struct Entry {
    static_fn: StaticFn,
    fp: Arc<vk::EntryFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

/// The one function everything else is loaded from.
#[derive(Copy, Clone)]
pub struct StaticFn {
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

/// A `VkInstance` with its instance-level commands.
///
/// This is a handle plus a function pointer table. It does not destroy the instance on
/// drop; call [`Instance::destroy_instance`].
///
/// Cloning is cheap: the function pointer table is shared, so a clone costs one
/// reference count increment (two with a dynamically loaded Vulkan library).
#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: Arc<vk::InstanceFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

/// A `VkDevice` with its device-level commands.
///
/// This is a handle plus a function pointer table. It does not destroy the device on
/// drop; call [`Device::destroy_device`].
///
/// Cloning is cheap: the function pointer table (several kilobytes) is shared, so a
/// clone costs one reference count increment (two with a dynamically loaded Vulkan
/// library). Clone freely where borrowing a `&Device` is inconvenient.
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: Arc<vk::DeviceFn>,
    #[cfg(feature = "std")]
    _library: Library,
}

impl Entry {
    /// Loads the Vulkan loader from the platform default dynamic library.
    ///
    /// # Safety
    ///
    /// Loading a library runs its initializers, and the library found under the default
    /// name must be a Vulkan loader that is ABI-compatible with these bindings.
    #[cfg(feature = "std")]
    pub unsafe fn load() -> core::result::Result<Self, LoadError> {
        unsafe { Self::load_from(default_vulkan_loader_name()) }
    }

    /// Loads the Vulkan loader from a dynamic library path.
    ///
    /// # Safety
    ///
    /// Loading a library runs its initializers, and `path` must name a Vulkan loader that
    /// is ABI-compatible with these bindings.
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

    /// Uses the Vulkan loader that the program is linked against.
    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        // SAFETY: the linker resolved `vkGetInstanceProcAddr` from the Vulkan loader.
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr: Some(vkGetInstanceProcAddr),
            })
        }
    }

    /// Builds an entry from a `vkGetInstanceProcAddr` supplied by the embedding
    /// environment.
    ///
    /// # Safety
    ///
    /// `get_instance_proc_addr` must be a valid Vulkan loader function and must stay
    /// callable for as long as the returned `Entry`, and anything created from it, is used.
    pub unsafe fn from_get_instance_proc_addr(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr,
            })
        }
    }

    /// Builds an entry from statically supplied loader functions.
    ///
    /// # Safety
    ///
    /// Every function pointer in `static_fn` must be a valid Vulkan loader function and
    /// must stay callable for as long as the returned `Entry`, and anything created from
    /// it, is used.
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

    /// [`vkCreateInstance`](https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateInstance.html),
    /// then loads the instance-level commands.
    ///
    /// # Safety
    ///
    /// All valid usage rules of `vkCreateInstance` must be upheld.
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
    /// Loads the instance-level commands of an existing `VkInstance`.
    ///
    /// # Safety
    ///
    /// `handle` must be a live instance created through the loader behind `entry`.
    pub unsafe fn load(entry: &Entry, handle: vk::Instance) -> Self {
        let fp = unsafe { vk::InstanceFn::load(entry.static_fn.get_instance_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: entry._library.clone(),
        }
    }

    /// Wraps a handle with an already loaded table.
    ///
    /// Nothing keeps the Vulkan loader alive on behalf of the returned value; prefer
    /// [`Instance::load`].
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

    /// [`vkCreateDevice`](https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateDevice.html),
    /// then loads the device-level commands.
    ///
    /// # Safety
    ///
    /// All valid usage rules of `vkCreateDevice` must be upheld.
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
    /// Loads the device-level commands of an existing `VkDevice`.
    ///
    /// # Safety
    ///
    /// `handle` must be a live device created from `instance`.
    pub unsafe fn load(instance: &Instance, handle: vk::Device) -> Self {
        let fp = unsafe { vk::DeviceFn::load(instance.fp.get_device_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            #[cfg(feature = "std")]
            _library: instance._library.clone(),
        }
    }

    /// Wraps a handle with an already loaded table.
    ///
    /// Nothing keeps the Vulkan loader alive on behalf of the returned value; prefer
    /// [`Device::load`].
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

/// The Vulkan loader could not be loaded.
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

/// Reinterprets the untyped pointer returned by `vkGet*ProcAddr` as a typed one.
///
/// # Safety
///
/// `T` must be the `Option<unsafe extern "system" fn(..)>` type of the command that `pfn`
/// was looked up for.
pub(crate) unsafe fn load_pfn<T>(pfn: vk::PFN_vkVoidFunction) -> T {
    const {
        assert!(core::mem::size_of::<T>() == core::mem::size_of::<vk::PFN_vkVoidFunction>());
    }
    unsafe { core::mem::transmute_copy(&pfn) }
}
