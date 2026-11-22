use crate::loader::LoaderLibrary;
use crate::{Device, Entry, vk};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct Instance {
    handle: vk::Instance,
    fp: Arc<vk::InstanceFn>,
    library: LoaderLibrary,
}

impl Instance {
    pub unsafe fn load(entry: &Entry, handle: vk::Instance) -> Self {
        let fp = unsafe { vk::InstanceFn::load(entry.static_fn().get_instance_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            library: entry.library().clone(),
        }
    }

    pub fn from_raw(handle: vk::Instance, fp: vk::InstanceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            library: LoaderLibrary::default(),
        }
    }

    pub fn handle(&self) -> vk::Instance {
        self.handle
    }

    pub fn fp(&self) -> &vk::InstanceFn {
        &self.fp
    }

    pub(crate) fn library(&self) -> &LoaderLibrary {
        &self.library
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
