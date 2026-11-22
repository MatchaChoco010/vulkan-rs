use crate::loader::LoaderLibrary;
use crate::{Instance, vk};
use alloc::sync::Arc;

/// The Vulkan loader and its global commands.
///
/// An `Entry` holds the loader's `vkGetInstanceProcAddr` ([`StaticFn`]) and the table of
/// the commands that need no instance ([`vk::EntryFn`]): `vkCreateInstance`,
/// `vkEnumerateInstanceExtensionProperties`, `vkEnumerateInstanceLayerProperties`,
/// `vkEnumerateInstanceVersion` and `vkGetInstanceProcAddr`. Each of them is a method in the
/// two forms described in the [crate documentation](crate#commands), for example
/// `enumerate_instance_extension_properties` and
/// `enumerate_instance_extension_properties_raw`. [`Entry::create_instance`] creates an
/// [`Instance`] with its commands loaded.
///
/// The constructors are `Entry::load` and `Entry::load_from` (feature `std`), which open
/// the loader as a dynamic library, `Entry::linked` (feature `linked`), and
/// [`Entry::from_get_instance_proc_addr`] and [`Entry::from_static_fn`] for a loader
/// function obtained elsewhere. A library opened by `Entry::load` or `Entry::load_from`
/// stays loaded as long as the `Entry`, a clone of it, or an [`Instance`] or
/// [`Device`](crate::Device) loaded from it exists.
///
/// Cloning is cheap: the function pointer table is shared.
#[derive(Clone)]
pub struct Entry {
    static_fn: StaticFn,
    fp: Arc<vk::EntryFn>,
    library: LoaderLibrary,
}

/// The loader function every other Vulkan command is looked up with.
#[derive(Copy, Clone)]
pub struct StaticFn {
    /// The `vkGetInstanceProcAddr` of the Vulkan loader.
    pub get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
}

impl Entry {
    /// Opens the Vulkan loader under the platform's default library name and looks up its
    /// global commands.
    ///
    /// The name is `vulkan-1.dll` on Windows, `libvulkan.dylib` on macOS and iOS, and
    /// `libvulkan.so.1` elsewhere; the platform's dynamic library search finds the file.
    /// The library stays loaded while the returned `Entry` or anything loaded from it
    /// exists.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError`](crate::LoadError) if the library cannot be opened or has no
    /// `vkGetInstanceProcAddr` symbol.
    ///
    /// # Safety
    ///
    /// Opening a library runs its initialization code. The library found under the default
    /// name must be a Vulkan loader whose exported `vkGetInstanceProcAddr` behaves as the
    /// Vulkan specification defines.
    #[cfg(feature = "std")]
    pub unsafe fn load() -> core::result::Result<Self, crate::LoadError> {
        let (library, get_instance_proc_addr) = unsafe { LoaderLibrary::open(None) }?;
        Ok(unsafe { Self::with_library(get_instance_proc_addr, library) })
    }

    /// Opens the Vulkan loader from `path` and looks up its global commands.
    ///
    /// `path` is passed to the platform's dynamic library loader, so a bare file name is
    /// searched for the way the platform searches for libraries. The library stays loaded
    /// while the returned `Entry` or anything loaded from it exists.
    ///
    /// # Errors
    ///
    /// Returns [`LoadError`](crate::LoadError) if the library cannot be opened or has no
    /// `vkGetInstanceProcAddr` symbol.
    ///
    /// # Safety
    ///
    /// Opening a library runs its initialization code. `path` must name a Vulkan loader (or
    /// another library exporting `vkGetInstanceProcAddr`) whose `vkGetInstanceProcAddr`
    /// behaves as the Vulkan specification defines.
    #[cfg(feature = "std")]
    pub unsafe fn load_from(
        path: impl AsRef<std::ffi::OsStr>,
    ) -> core::result::Result<Self, crate::LoadError> {
        let (library, get_instance_proc_addr) =
            unsafe { LoaderLibrary::open(Some(path.as_ref())) }?;
        Ok(unsafe { Self::with_library(get_instance_proc_addr, library) })
    }

    /// Uses the `vkGetInstanceProcAddr` of the Vulkan loader the program is linked against,
    /// and looks up the global commands.
    ///
    /// The crate links `vulkan-1` on Windows and `vulkan` elsewhere at build time.
    #[cfg(feature = "linked")]
    pub fn linked() -> Self {
        unsafe { Self::from_get_instance_proc_addr(Some(crate::loader::vkGetInstanceProcAddr)) }
    }

    /// Builds an entry from a `vkGetInstanceProcAddr` obtained elsewhere, such as from a
    /// windowing library, and looks up the global commands with it.
    ///
    /// The returned `Entry` does not keep any library loaded.
    ///
    /// # Panics
    ///
    /// Panics if `get_instance_proc_addr` is `None`.
    ///
    /// # Safety
    ///
    /// `get_instance_proc_addr` must behave as the Vulkan specification defines
    /// `vkGetInstanceProcAddr`. It, and every function pointer it returns, must stay
    /// callable for as long as the returned `Entry`, its clones, and every [`Instance`] and
    /// [`Device`](crate::Device) loaded from them are used.
    pub unsafe fn from_get_instance_proc_addr(
        get_instance_proc_addr: vk::PFN_vkGetInstanceProcAddr,
    ) -> Self {
        unsafe {
            Self::from_static_fn(StaticFn {
                get_instance_proc_addr,
            })
        }
    }

    /// Builds an entry from a [`StaticFn`] and looks up the global commands with it.
    ///
    /// The returned `Entry` does not keep any library loaded.
    ///
    /// # Panics
    ///
    /// Panics if `static_fn.get_instance_proc_addr` is `None`.
    ///
    /// # Safety
    ///
    /// `static_fn.get_instance_proc_addr` must behave as the Vulkan specification defines
    /// `vkGetInstanceProcAddr`. It, and every function pointer it returns, must stay
    /// callable for as long as the returned `Entry`, its clones, and every [`Instance`] and
    /// [`Device`](crate::Device) loaded from them are used.
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

    /// The `vkGetInstanceProcAddr` this entry was built from.
    pub fn static_fn(&self) -> &StaticFn {
        &self.static_fn
    }

    /// The function pointers of the global commands.
    pub fn fp(&self) -> &vk::EntryFn {
        &self.fp
    }

    pub(crate) fn library(&self) -> &LoaderLibrary {
        &self.library
    }

    /// Calls `vkCreateInstance`, then loads the instance-level commands of the new instance
    /// with [`Instance::load`].
    ///
    /// The returned [`Instance`] keeps the library of this entry loaded. Destroy the
    /// instance with [`Instance::destroy_instance`]; dropping it does not.
    ///
    /// # Errors
    ///
    /// Returns the error code of `vkCreateInstance`, for example
    /// `ERROR_EXTENSION_NOT_PRESENT` or `ERROR_INCOMPATIBLE_DRIVER`.
    ///
    /// # Panics
    ///
    /// Panics if the loader provided no `vkCreateInstance`.
    ///
    /// # Safety
    ///
    /// The call must satisfy the valid usage rules of `vkCreateInstance`: `create_info`,
    /// everything it points to (including its `pNext` chain) and `allocator` must be valid.
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
