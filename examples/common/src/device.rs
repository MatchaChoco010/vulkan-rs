use anyhow::{Result, bail};
use std::cell::RefCell;
use std::ffi::CStr;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use vulkan::{Device, Instance, vk};
use vulkan_alloc::{Allocation, Allocator, AllocatorCreateDesc, FreeError};

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

/// Whether a device created with `info` may take the device address of a buffer: the
/// feature `bufferDeviceAddress` in `VkPhysicalDeviceBufferDeviceAddressFeatures` or in
/// `VkPhysicalDeviceVulkan12Features` in the `pNext` chain.
///
/// # Safety
///
/// The `pNext` chain of `info` must be valid.
pub unsafe fn enables_buffer_device_address(info: &vk::DeviceCreateInfo<'_>) -> bool {
    let mut next = info.p_next.cast::<vk::BaseInStructure<'_>>();
    // SAFETY: every structure in a valid chain starts with `sType` and `pNext`, and
    // `sType` tells which structure it is.
    while let Some(base) = unsafe { next.as_ref() } {
        let enabled = match base.s_type {
            vk::StructureType::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceBufferDeviceAddressFeatures<'_>>())
                    .buffer_device_address
            },
            vk::StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES => unsafe {
                (*next.cast::<vk::PhysicalDeviceVulkan12Features<'_>>()).buffer_device_address
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
/// created from it. When the last clone goes, it waits until the device is idle, frees
/// whatever memory the allocator still holds, and destroys the device. The instance
/// goes once nothing else holds it.
#[derive(Clone)]
pub struct Gpu(Rc<GpuOwner>);

/// What a [`Gpu`] shares between its clones.
pub struct GpuOwner {
    pub instance: VulkanInstance,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub queue_family: u32,
    pub queue: vk::Queue,
    /// Whether the device was created with `bufferDeviceAddress`. The allocator then
    /// allocates all memory with `DEVICE_ADDRESS`, so any buffer may take its address.
    pub buffer_device_address: bool,
    /// Dropped after the device is destroyed, which is harmless: the `Drop` of
    /// [`GpuOwner`] calls `free_all` before it destroys the device, and an allocator that
    /// holds no memory makes no Vulkan call when dropped.
    allocator: RefCell<Allocator>,
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
            let buffer_device_address = enables_buffer_device_address(info);
            // Memory with DEVICE_ADDRESS may back any buffer or image, so with the feature
            // every allocation gets it and no allocation has to ask for it.
            let memory_allocate_flags = if buffer_device_address {
                vk::MemoryAllocateFlags::DEVICE_ADDRESS
            } else {
                vk::MemoryAllocateFlags::empty()
            };
            let allocator = Allocator::new(AllocatorCreateDesc {
                memory_allocate_flags,
                ..AllocatorCreateDesc::new(
                    &device,
                    instance.get_physical_device_memory_properties(physical_device),
                    instance.get_physical_device_properties(physical_device),
                )
            });
            Ok(Self(Rc::new(GpuOwner {
                instance,
                physical_device,
                device,
                queue_family,
                queue,
                buffer_device_address,
                allocator: RefCell::new(allocator),
            })))
        }
    }

    /// Wraps `value`, so that it is destroyed when the result goes out of scope.
    ///
    /// # Safety
    ///
    /// `value` must have been created from this device (or its allocator), and nothing
    /// else may destroy it. When the result is dropped, the device must be done with the
    /// object: work that used it was waited for.
    pub unsafe fn own<T: Destroy>(&self, value: T) -> Owned<T> {
        Owned {
            value: Some(value),
            gpu: self.clone(),
        }
    }
}

impl GpuOwner {
    /// Calls `f` with the allocator for the memory of buffers and images. The allocator
    /// is borrowed only for the call, since the [`Owned`] buffers and images borrow it
    /// again to free their memory.
    ///
    /// # Panics
    ///
    /// When called from within `f` of another call, which already borrows the allocator.
    pub fn with_allocator<T>(&self, f: impl FnOnce(&mut Allocator) -> T) -> T {
        f(&mut self.allocator.borrow_mut())
    }

    /// Destroys `buffer` and frees `allocation`, as the [`Destroy`] of a buffer does. A
    /// failure is printed: destruction has no caller to return it to. The buffer is
    /// destroyed all the same, and the memory of an allocation that could not be freed
    /// goes with the device.
    pub(crate) unsafe fn destroy_buffer(&self, buffer: vk::Buffer, allocation: Allocation) {
        let destroyed = self.destroy_with_memory(allocation, |allocator, allocation| unsafe {
            allocator.destroy_buffer(buffer, allocation)
        });
        if !destroyed {
            unsafe { self.device.destroy_buffer(buffer, None) };
        }
    }

    /// Destroys `image` and frees `allocation`, as the [`Destroy`] of an image does,
    /// with failures handled as for [`GpuOwner::destroy_buffer`].
    pub(crate) unsafe fn destroy_image(&self, image: vk::Image, allocation: Allocation) {
        let destroyed = self.destroy_with_memory(allocation, |allocator, allocation| unsafe {
            allocator.destroy_image(image, allocation)
        });
        if !destroyed {
            unsafe { self.device.destroy_image(image, None) };
        }
    }

    /// Calls `destroy` with the allocator and `allocation`, and tells whether it
    /// destroyed the resource: not when the allocator is in use, nor when it did not
    /// free the allocation, which leaves the resource alive.
    fn destroy_with_memory(
        &self,
        allocation: Allocation,
        destroy: impl FnOnce(&mut Allocator, Allocation) -> Result<(), FreeError>,
    ) -> bool {
        let Ok(mut allocator) = self.allocator.try_borrow_mut() else {
            cleanup_failed("an object was dropped while the allocator was in use");
            return false;
        };
        match destroy(&mut allocator, allocation) {
            Ok(()) => true,
            Err(error) => {
                cleanup_failed(format_args!("freeing memory: {}", error.error));
                false
            }
        }
    }
}

impl Drop for GpuOwner {
    fn drop(&mut self) {
        // SAFETY: every object created from the device holds a clone of the `Gpu`, so
        // all of them are gone; waiting makes sure that no submitted work is left.
        unsafe {
            let _ = self.device.device_wait_idle();
            // Each buffer and image freed its memory when it was destroyed. This frees
            // the empty blocks that the allocator keeps for later allocations and what a
            // failed free left behind, so that dropping the allocator after the device
            // makes no Vulkan call.
            let live = self.allocator.get_mut().free_all();
            if live != 0 {
                cleanup_failed(format_args!(
                    "{live} allocation(s) were still live when the device was destroyed"
                ));
            }
            self.device.destroy_device(None);
        }
    }
}

static CLEANUP_FAILED: AtomicBool = AtomicBool::new(false);

/// Prints a failure to destroy or free something, which has no caller to return an
/// error to, and remembers it for [`finish`].
pub fn cleanup_failed(message: impl Display) {
    eprintln!("{message}");
    CLEANUP_FAILED.store(true, Ordering::Relaxed);
}

/// The result of an example's run: `result`, or an error when destroying an object
/// failed on the way, so that the process exits with a failing status either way.
pub fn finish(result: Result<()>) -> Result<()> {
    finish_with(result, CLEANUP_FAILED.load(Ordering::Relaxed))
}

/// [`finish`], given whether a cleanup failed.
fn finish_with(result: Result<()>, cleanup_failed: bool) -> Result<()> {
    result?;
    if cleanup_failed {
        bail!("destroying objects failed, as printed above");
    }
    Ok(())
}

/// An object that a [`Gpu`] can destroy: a handle created from its device, or a buffer or
/// image together with its memory from the allocator.
pub trait Destroy {
    /// The device must not be using the object any more.
    unsafe fn destroy(self, gpu: &GpuOwner);
}

/// A `T` that is destroyed when this is dropped. It dereferences to `T`, so a handle is
/// used as `*owned` and the fields of a buffer as `owned.buffer`. It gives no `&mut T`
/// outside this crate, so that the owned object cannot be replaced by another one.
///
/// Locals are dropped in the reverse order of their declaration, and the fields of a
/// struct in the order of their declaration. An example that creates its objects in the
/// order they depend on each other therefore destroys them in the reverse order, on the
/// way out of a successful run as on an early return with `?`.
pub struct Owned<T: Destroy> {
    /// `None` only once [`Owned::into_inner`] took it.
    value: Option<T>,
    gpu: Gpu,
}

impl<T: Destroy> Owned<T> {
    /// Gives up the ownership: `value` is no longer destroyed by this.
    pub fn into_inner(mut self) -> T {
        self.value
            .take()
            .expect("an Owned holds its value until it is dropped")
    }

    /// The value, for the helpers of this crate that need `&mut` access to a part of it,
    /// such as the mapped bytes of an allocation. They must not replace the value.
    pub(crate) fn value_mut(&mut self) -> &mut T {
        self.value
            .as_mut()
            .expect("an Owned holds its value until it is dropped")
    }
}

impl<T: Destroy> Deref for Owned<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.value
            .as_ref()
            .expect("an Owned holds its value until it is dropped")
    }
}

impl<T: Destroy> Drop for Owned<T> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            // SAFETY: `Gpu::own` requires that the value came from this device, that
            // nothing else destroys it, and that the device is done with it.
            unsafe { value.destroy(&self.gpu) };
        }
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

/// Whether a physical device that reports `api_version` and the device extensions
/// `available` has at least `required_version` and every extension in `required`.
pub fn meets_requirements(
    api_version: u32,
    available: &[vk::ExtensionProperties],
    required_version: u32,
    required: &[&CStr],
) -> bool {
    api_version >= required_version && required.iter().all(|name| has_extension(available, name))
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

    #[test]
    fn a_run_fails_when_it_failed_or_a_cleanup_failed() {
        for run_fails in [false, true] {
            for cleanup_fails in [false, true] {
                let result = if run_fails {
                    Err(anyhow::anyhow!("the run failed"))
                } else {
                    Ok(())
                };
                let finished = finish_with(result, cleanup_fails);
                assert_eq!(
                    finished.is_err(),
                    run_fails || cleanup_fails,
                    "run fails: {run_fails}, cleanup fails: {cleanup_fails}"
                );
                if run_fails {
                    assert_eq!(finished.unwrap_err().to_string(), "the run failed");
                }
            }
        }
    }

    #[test]
    fn a_reported_cleanup_failure_is_remembered() {
        cleanup_failed("a test reports a cleanup failure");
        assert!(CLEANUP_FAILED.load(Ordering::Relaxed));
    }
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
