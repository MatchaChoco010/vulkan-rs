use crate::loader::LoaderLibrary;
use crate::{Device, Entry, vk};
use alloc::sync::Arc;

/// A `VkInstance` with its instance-level commands.
///
/// An `Instance` is the handle ([`vk::Instance`]) plus the table of instance-level
/// function pointers ([`vk::InstanceFn`]), looked up with `vkGetInstanceProcAddr` for this
/// instance when the `Instance` is created. The instance-level commands are the commands
/// on `VkInstance` and `VkPhysicalDevice`, such as `enumerate_physical_devices`,
/// `get_physical_device_properties`, `create_device` and the surface commands, and also
/// `vkGetDeviceProcAddr`. Each of them is a method in the two forms
/// described in the [crate documentation](crate#commands); the `VkInstance` parameter is
/// taken from `self`. [`Instance::create_device`] creates a [`Device`] with its commands
/// loaded.
///
/// The instance is not destroyed on drop; call [`Instance::destroy_instance`] after
/// destroying everything created from it. An `Instance` created by
/// [`Entry::create_instance`] or [`Instance::load`] keeps the Vulkan loader library of its
/// [`Entry`] loaded.
///
/// Cloning is cheap: the function pointer table is shared, so a clone costs a reference
/// count increment or two.
#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: Arc<vk::InstanceFn>,
    library: LoaderLibrary,
}

impl Instance {
    /// Looks up the instance-level commands of an existing `VkInstance` with the
    /// `vkGetInstanceProcAddr` of `entry`.
    ///
    /// Use it for an instance created with `Entry::create_instance_raw` or by other code
    /// that uses the same Vulkan loader. The returned value keeps the loader library of
    /// `entry` loaded.
    ///
    /// # Safety
    ///
    /// `handle` must be a valid `VkInstance` created through the loader behind `entry`.
    pub unsafe fn load(entry: &Entry, handle: vk::Instance) -> Self {
        let fp = unsafe { vk::InstanceFn::load(entry.static_fn().get_instance_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            library: entry.library().clone(),
        }
    }

    /// Wraps a handle and a table that was loaded for it.
    ///
    /// The command methods call the function pointers of `fp` with `handle`, so `fp` must
    /// have been loaded for `handle`, and the library the pointers come from must stay
    /// loaded while they are called: nothing keeps a library loaded on behalf of the
    /// returned value. Prefer [`Instance::load`].
    pub fn from_raw(handle: vk::Instance, fp: vk::InstanceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            library: LoaderLibrary::default(),
        }
    }

    /// The `VkInstance` handle.
    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    /// The function pointers of the instance-level commands.
    pub fn fp(&self) -> &vk::InstanceFn {
        &self.fp
    }

    pub(crate) fn library(&self) -> &LoaderLibrary {
        &self.library
    }

    /// Calls `vkCreateDevice`, then loads the device-level commands of the new device with
    /// [`Device::load`].
    ///
    /// The returned [`Device`] keeps the Vulkan loader library of this instance loaded.
    /// Destroy the device with [`Device::destroy_device`]; dropping it does not.
    ///
    /// # Errors
    ///
    /// Returns the error code of `vkCreateDevice`, for example
    /// `ERROR_EXTENSION_NOT_PRESENT`, `ERROR_FEATURE_NOT_PRESENT` or `ERROR_DEVICE_LOST`.
    ///
    /// # Panics
    ///
    /// Panics if the loader provided no `vkCreateDevice` or no `vkGetDeviceProcAddr`.
    ///
    /// # Safety
    ///
    /// The call must satisfy the valid usage rules of `vkCreateDevice`: `physical_device`
    /// must have been enumerated from this instance, and `create_info`, everything it
    /// points to (including its `pNext` chain) and `allocator` must be valid.
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
