use super::StructureType;
use core::ffi::c_void;

/// Marks a struct that may appear in the `pNext` chain of `Root`.
///
/// The `push_next` method of a struct accepts only structs that implement `Extends` for
/// it, so a chain built with `push_next` holds only structs that the Vulkan registry lists
/// as extending its root (the `structextends` attribute of `vk.xml`). The crate implements it
/// for every such pair whose structs exist with the enabled features, for all lifetimes of
/// the two structs.
///
/// ```
/// use vulkan::vk;
///
/// let mut timeline = vk::SemaphoreTypeCreateInfo::default()
///     .semaphore_type(vk::SemaphoreType::TIMELINE)
///     .initial_value(1);
/// let info = vk::SemaphoreCreateInfo::default().push_next(&mut timeline);
/// assert!(!info.p_next.is_null());
/// ```
///
/// # Safety
///
/// A Vulkan implementation reads every struct of a `pNext` chain as the struct its `sType`
/// names, and must recognize it in the chain of `Root`. Implement `Extends<Root>` only for
/// a struct that the Vulkan specification allows in the `pNext` chain of `Root`.
pub unsafe trait Extends<Root> {}

/// A struct that starts with the `sType` and `pNext` members, so it can be linked into a
/// `pNext` chain.
///
/// `push_next` writes the `pNext` member of the struct it links in through this layout.
///
/// ```
/// use vulkan::vk::{self, TaggedStructure};
///
/// assert_eq!(
///     vk::DeviceCreateInfo::STRUCTURE_TYPE,
///     vk::StructureType::DEVICE_CREATE_INFO
/// );
/// assert_eq!(
///     vk::DeviceCreateInfo::default().s_type,
///     vk::DeviceCreateInfo::STRUCTURE_TYPE
/// );
/// ```
///
/// # Safety
///
/// The implementing type must be `#[repr(C)]`, its first field must be a
/// [`StructureType`] and its second field a pointer-sized, writable `pNext` pointer (a
/// `*const c_void` or `*mut c_void`), as in the C struct. `STRUCTURE_TYPE` must be the
/// `sType` value that identifies the struct.
pub unsafe trait TaggedStructure {
    /// The `VkStructureType` value of this struct, the value its `sType` member holds.
    const STRUCTURE_TYPE: StructureType;
}

#[repr(C)]
struct ChainHeader {
    s_type: StructureType,
    p_next: *mut c_void,
}

pub(crate) fn push_chain<T: TaggedStructure>(next: &mut T, head: *const c_void) -> *mut c_void {
    let next = core::ptr::from_mut(next).cast::<ChainHeader>();
    unsafe { (*next).p_next = head.cast_mut() };
    next.cast()
}
