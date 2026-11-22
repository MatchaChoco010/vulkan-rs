use crate::loader::LoaderLibrary;
use crate::{Instance, vk};
use alloc::sync::Arc;

/// A `VkDevice` with its device-level commands.
///
/// A `Device` is the handle ([`vk::Device`]) plus the table of device-level function
/// pointers ([`vk::DeviceFn`]), looked up with `vkGetDeviceProcAddr` for this device when
/// the `Device` is created. The device-level commands are the commands on `VkDevice`,
/// `VkQueue` and `VkCommandBuffer`, such as `create_buffer`, `queue_submit` and
/// `cmd_draw`. Each of them is a method in the two forms described in the
/// [crate documentation](crate#commands); the `VkDevice` parameter is taken from `self`,
/// and queues and command buffers are passed as the first parameter. The pointers are
/// specific to this device, so only pass queues and command buffers of this device.
///
/// The device is not destroyed on drop; call [`Device::destroy_device`] after destroying
/// everything created from it. A `Device` created by [`Instance::create_device`] or
/// [`Device::load`] keeps the Vulkan loader library of its [`Instance`] loaded.
///
/// Cloning is cheap: the function pointer table (several kilobytes) is shared, so a clone
/// costs a reference count increment or two. Clone it where borrowing a `&Device` is
/// inconvenient.
#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: Arc<vk::DeviceFn>,
    _library: LoaderLibrary,
}

impl Device {
    /// Looks up the device-level commands of an existing `VkDevice` with the
    /// `vkGetDeviceProcAddr` of `instance`.
    ///
    /// Use it for a device created with `Instance::create_device_raw` or by other code.
    /// The returned value keeps the loader library of `instance` loaded.
    ///
    /// # Panics
    ///
    /// Panics if the table of `instance` has no `vkGetDeviceProcAddr`.
    ///
    /// # Safety
    ///
    /// `handle` must be a valid `VkDevice` created from a physical device of `instance`.
    pub unsafe fn load(instance: &Instance, handle: vk::Device) -> Self {
        let fp = unsafe { vk::DeviceFn::load(instance.fp().get_device_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            _library: instance.library().clone(),
        }
    }

    /// Wraps a handle and a table that was loaded for it.
    ///
    /// The command methods call the function pointers of `fp` with `handle`, so `fp` must
    /// have been loaded for `handle`, and the library the pointers come from must stay
    /// loaded while they are called: nothing keeps a library loaded on behalf of the
    /// returned value. Prefer [`Device::load`].
    pub fn from_raw(handle: vk::Device, fp: vk::DeviceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            _library: LoaderLibrary::default(),
        }
    }

    /// The `VkDevice` handle.
    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    /// The function pointers of the device-level commands.
    pub fn fp(&self) -> &vk::DeviceFn {
        &self.fp
    }
}
