use crate::loader::LoaderLibrary;
use crate::{Instance, vk};
use alloc::sync::Arc;

#[derive(Clone)]
pub struct Device {
    handle: vk::Device,
    fp: Arc<vk::DeviceFn>,
    _library: LoaderLibrary,
}

impl Device {
    pub unsafe fn load(instance: &Instance, handle: vk::Device) -> Self {
        let fp = unsafe { vk::DeviceFn::load(instance.fp().get_device_proc_addr, handle) };
        Self {
            handle,
            fp: Arc::new(fp),
            _library: instance.library().clone(),
        }
    }

    pub fn from_raw(handle: vk::Device, fp: vk::DeviceFn) -> Self {
        Self {
            handle,
            fp: Arc::new(fp),
            _library: LoaderLibrary::default(),
        }
    }

    pub fn handle(&self) -> vk::Device {
        self.handle
    }

    pub fn fp(&self) -> &vk::DeviceFn {
        &self.fp
    }
}
