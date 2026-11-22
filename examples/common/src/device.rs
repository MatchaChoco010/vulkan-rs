use anyhow::Result;
use std::cell::{RefCell, RefMut};
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use vulkan::{Device, Instance, vk};
use vulkan_alloc::{Allocator, AllocatorCreateDesc};

use crate::extensions::has_extension;
use crate::instance::VulkanInstance;

/// Whether a device created with `info` may run shaders that declare the SPIR-V
/// capability `DrawParameters`. The registry lists three ways to allow it: the feature
/// `shaderDrawParameters` in `VkPhysicalDeviceShaderDrawParametersFeatures` or in
/// `VkPhysicalDeviceVulkan11Features` in the `pNext` chain, or the device extension
/// `VK_KHR_shader_draw_parameters`.
///
/// # Safety
///
/// The `pNext` chain and the extension names of `info` must be valid.
pub unsafe fn enables_draw_parameters(info: &vk::DeviceCreateInfo<'_>) -> bool {
    unsafe { enables_draw_parameters_extension(info) || enables_shader_draw_parameters(info) }
}

unsafe fn enables_draw_parameters_extension(info: &vk::DeviceCreateInfo<'_>) -> bool {
    (0..info.enabled_extension_count as usize).any(|index| {
        // SAFETY: the caller guarantees an array of `enabledExtensionCount` valid
        // null-terminated strings.
        let name = unsafe { CStr::from_ptr(*info.pp_enabled_extension_names.add(index)) };
        name == vk::KHR_SHADER_DRAW_PARAMETERS_NAME
    })
}

unsafe fn enables_shader_draw_parameters(info: &vk::DeviceCreateInfo<'_>) -> bool {
    let mut next = info.p_next.cast::<vk::BaseInStructure<'_>>();
    // SAFETY: every structure in a valid chain starts with `sType` and `pNext`, and
    // `sType` tells which structure it is.
    while let Some(base) = unsafe { next.as_ref() } {
        let enabled = match base.s_type {
            vk::StructureType::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceShaderDrawParametersFeatures<'_>>())
                    .shader_draw_parameters
            },
            vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_1_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceVulkan11Features<'_>>()).shader_draw_parameters
            },
            _ => vk::FALSE,
        };
        if enabled != vk::FALSE {
            return true;
        }
        next = base.p_next;
    }
    false
}

/// The device with the one queue that the example submits to, the instance it was
/// created from, and the allocator for its memory.
///
/// The clones share all of it. Every object that the examples create from the device
/// is wrapped in an [`Owned`], which holds a clone, so the device outlives everything
/// created from it. When the last clone goes, it waits until the device is idle, drops
/// the allocator, which needs the device to free its memory, and destroys the device.
/// The instance goes once nothing else holds it.
#[derive(Clone)]
pub struct Gpu(Rc<GpuOwner>);

/// What a [`Gpu`] shares between its clones.
pub struct GpuOwner {
    pub instance: VulkanInstance,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub queue_family: u32,
    pub queue: vk::Queue,
    allocator: ManuallyDrop<RefCell<Allocator>>,
}

impl Deref for Gpu {
    type Target = GpuOwner;

    fn deref(&self) -> &GpuOwner {
        &self.0
    }
}

impl Gpu {
    /// Creates the device from `info` and takes queue 0 of `queue_family`, which `info`
    /// must ask for.
    pub unsafe fn create(
        instance: VulkanInstance,
        physical_device: vk::PhysicalDevice,
        queue_family: u32,
        info: &vk::DeviceCreateInfo<'_>,
    ) -> Result<Self> {
        unsafe {
            let device = instance.create_device(physical_device, info, None)?;
            let queue = device.get_device_queue(queue_family, 0);
            let allocator = Allocator::new(AllocatorCreateDesc::new(
                &device,
                instance.get_physical_device_memory_properties(physical_device),
                instance.get_physical_device_properties(physical_device),
            ));
            Ok(Self(Rc::new(GpuOwner {
                instance,
                physical_device,
                device,
                queue_family,
                queue,
                allocator: ManuallyDrop::new(RefCell::new(allocator)),
            })))
        }
    }

    /// Wraps `value`, so that it is destroyed when the result goes out of scope.
    ///
    /// # Safety
    ///
    /// `value` must have been created from this device (or its allocator), and nothing
    /// else may destroy it.
    pub unsafe fn own<T: Destroy>(&self, value: T) -> Owned<T> {
        Owned {
            value: ManuallyDrop::new(value),
            gpu: self.clone(),
        }
    }
}

impl GpuOwner {
    /// The allocator for the memory of buffers and images. Hold it only as long as one
    /// call needs it: the [`Owned`] buffers and images borrow it again to free their
    /// memory.
    pub fn allocator(&self) -> RefMut<'_, Allocator> {
        self.allocator.borrow_mut()
    }
}

impl Drop for GpuOwner {
    fn drop(&mut self) {
        // SAFETY: every object created from the device holds a clone of the `Gpu`, so
        // all of them are gone; waiting makes sure that no submitted work is left.
        unsafe {
            let _ = self.device.device_wait_idle();
            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
        }
    }
}

/// An object that a [`Gpu`] can destroy: a handle created from its device, or a buffer or
/// image together with its memory from the allocator.
pub trait Destroy {
    /// The device must not be using the object any more.
    unsafe fn destroy(self, gpu: &GpuOwner);
}

/// A `T` that is destroyed when this is dropped. It dereferences to `T`, so a handle is
/// used as `*owned` and the fields of a buffer as `owned.buffer`.
///
/// Locals are dropped in the reverse order of their declaration, and the fields of a
/// struct in the order of their declaration. An example that creates its objects in the
/// order they depend on each other therefore destroys them in the reverse order, on the
/// way out of a successful run as on an early return with `?`.
pub struct Owned<T: Destroy> {
    value: ManuallyDrop<T>,
    gpu: Gpu,
}

impl<T: Destroy> Owned<T> {
    /// Gives up the ownership: `value` is no longer destroyed by this.
    pub fn into_inner(self) -> T {
        let mut this = ManuallyDrop::new(self);
        // SAFETY: `this` is never dropped, so `value` is taken once, and the `Gpu` is
        // dropped here instead.
        unsafe {
            std::ptr::drop_in_place(&mut this.gpu);
            ManuallyDrop::take(&mut this.value)
        }
    }
}

impl<T: Destroy> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T: Destroy> DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: Destroy> Drop for Owned<T> {
    fn drop(&mut self) {
        // SAFETY: `Gpu::own` requires that the value came from this device and that
        // nothing else destroys it; `value` is not used after this.
        unsafe { ManuallyDrop::take(&mut self.value).destroy(&self.gpu) };
    }
}

macro_rules! destroy_handles {
    ($($handle:ty => $destroy:ident,)*) => {
        $(
            impl Destroy for $handle {
                unsafe fn destroy(self, gpu: &GpuOwner) {
                    unsafe { gpu.device.$destroy(self, None) };
                }
            }
        )*
    };
}

destroy_handles! {
    vk::AccelerationStructureKHR => destroy_acceleration_structure_khr,
    vk::Buffer => destroy_buffer,
    vk::CommandPool => destroy_command_pool,
    vk::DeviceMemory => free_memory,
    vk::Fence => destroy_fence,
    vk::Framebuffer => destroy_framebuffer,
    vk::Image => destroy_image,
    vk::ImageView => destroy_image_view,
    vk::Pipeline => destroy_pipeline,
    vk::PipelineLayout => destroy_pipeline_layout,
    vk::RenderPass => destroy_render_pass,
    vk::Sampler => destroy_sampler,
    vk::Semaphore => destroy_semaphore,
    vk::ShaderModule => destroy_shader_module,
}

/// Asks `suitable` about every physical device and picks one of those it accepts:
/// a discrete GPU over an integrated one, that over a virtual one, that over the rest.
/// Among equals the first wins. `suitable` returns what it found out about the device,
/// such as the queue family to use, and `None` to reject it.
pub unsafe fn pick_physical_device<T>(
    instance: &Instance,
    mut suitable: impl FnMut(vk::PhysicalDevice, &vk::PhysicalDeviceProperties) -> Result<Option<T>>,
) -> Result<Option<(vk::PhysicalDevice, T)>> {
    unsafe {
        let mut best = None;
        for physical_device in instance.enumerate_physical_devices()? {
            let properties = instance.get_physical_device_properties(physical_device);
            let Some(found) = suitable(physical_device, &properties)? else {
                continue;
            };
            let rank = match properties.device_type {
                vk::PhysicalDeviceType::DISCRETE_GPU => 3,
                vk::PhysicalDeviceType::INTEGRATED_GPU => 2,
                vk::PhysicalDeviceType::VIRTUAL_GPU => 1,
                _ => 0,
            };
            if best
                .as_ref()
                .is_none_or(|(best_rank, _, _)| rank > *best_rank)
            {
                best = Some((rank, physical_device, found));
            }
        }
        Ok(best.map(|(_, physical_device, found)| (physical_device, found)))
    }
}

/// Whether the physical device has every extension in `names`.
pub unsafe fn supports_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    names: &[&CStr],
) -> Result<bool> {
    let available =
        unsafe { instance.enumerate_device_extension_properties(physical_device, None) }?;
    Ok(names.iter().all(|name| has_extension(&available, name)))
}

/// The first queue family that supports all of `flags`.
pub unsafe fn find_queue_family(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    flags: vk::QueueFlags,
) -> Option<u32> {
    unsafe { instance.get_physical_device_queue_family_properties(physical_device) }
        .iter()
        .position(|family| family.queue_flags.contains(flags))
        .map(|index| index as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::c_void;
    use std::ptr;

    const ORDERS: [[usize; 3]; 6] = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];

    /// Whether a create info allows `DrawParameters` when its `pNext` chain holds the
    /// draw parameters features (index 0), the Vulkan 1.1 features (1) and the unrelated
    /// Vulkan 1.2 features (2) in `order`, and it enables `extensions`.
    fn allows(
        draw_parameters: bool,
        vulkan11: bool,
        order: [usize; 3],
        extensions: &[&CStr],
    ) -> bool {
        let mut draw_parameters_features =
            vk::PhysicalDeviceShaderDrawParametersFeatures::default()
                .shader_draw_parameters(draw_parameters);
        let mut vulkan11_features =
            vk::PhysicalDeviceVulkan11Features::default().shader_draw_parameters(vulkan11);
        let mut vulkan12_features = vk::PhysicalDeviceVulkan12Features::default();
        let structures: [*mut vk::BaseOutStructure<'_>; 3] = [
            ptr::from_mut(&mut draw_parameters_features).cast(),
            ptr::from_mut(&mut vulkan11_features).cast(),
            ptr::from_mut(&mut vulkan12_features).cast(),
        ];
        for pair in order.windows(2) {
            unsafe { (*structures[pair[0]]).p_next = structures[pair[1]] };
        }
        unsafe { (*structures[order[2]]).p_next = ptr::null_mut() };
        let names = extensions
            .iter()
            .map(|name| vk::CStrPtr::new(name))
            .collect::<Vec<_>>();
        let mut info = vk::DeviceCreateInfo::default().enabled_extension_names(&names);
        info.p_next = structures[order[0]].cast::<c_void>().cast_const();
        unsafe { enables_draw_parameters(&info) }
    }

    #[test]
    fn an_empty_create_info_does_not_allow_draw_parameters() {
        assert!(!unsafe { enables_draw_parameters(&vk::DeviceCreateInfo::default()) });
    }

    #[test]
    fn finds_every_enabler_of_draw_parameters_at_any_position() {
        let without = [vk::KHR_SWAPCHAIN_NAME];
        let with = [vk::KHR_SWAPCHAIN_NAME, vk::KHR_SHADER_DRAW_PARAMETERS_NAME];
        for order in ORDERS {
            for draw_parameters in [false, true] {
                for vulkan11 in [false, true] {
                    for extension in [false, true] {
                        let extensions: &[&CStr] = if extension { &with } else { &without };
                        assert_eq!(
                            allows(draw_parameters, vulkan11, order, extensions),
                            draw_parameters || vulkan11 || extension,
                            "order {order:?}, draw parameters features: {draw_parameters}, \
                             Vulkan 1.1 features: {vulkan11}, extension: {extension}"
                        );
                    }
                }
            }
        }
    }
}
