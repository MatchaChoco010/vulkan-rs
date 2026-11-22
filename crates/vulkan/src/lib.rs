//! Rust bindings to the Vulkan API, generated from the Khronos Vulkan-Headers registry.
//!
//! The crate has two layers:
//!
//! - [`vk`] holds the types, constants and function pointer types of the C API, one to one:
//!   structs, unions, enums, bitmasks, handles, `PFN_vk*` types, API constants, extension
//!   names, and the function pointer tables [`vk::EntryFn`], [`vk::InstanceFn`] and
//!   [`vk::DeviceFn`]. Its documentation explains how to find the Rust name of a C name.
//! - [`Entry`], [`Instance`] and [`Device`] each hold a function pointer table, loaded for
//!   their handle ([`Entry`] has none), and expose every command of their level as methods.
//!
//! # Getting started
//!
//! Load the Vulkan loader, create an instance and list the physical devices (this needs the
//! `std` feature, which is on by default):
//!
//! ```no_run
//! # #[cfg(not(feature = "std"))]
//! # fn main() {}
//! # #[cfg(feature = "std")]
//! # use vulkan::{Entry, vk};
//! # #[cfg(any())]
//! use vulkan::{Entry, vk};
//!
//! # #[cfg(feature = "std")]
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     unsafe {
//!         let entry = Entry::load()?;
//!         let application = vk::ApplicationInfo::default()
//!             .application_name(c"example")
//!             .api_version(vk::API_VERSION_1_3);
//!         let instance = entry.create_instance(
//!             &vk::InstanceCreateInfo::default().application_info(&application),
//!             None,
//!         )?;
//!         for physical_device in instance.enumerate_physical_devices()? {
//!             let properties = instance.get_physical_device_properties(physical_device);
//!             println!("{:?}", properties.device_name_as_c_str()?);
//!         }
//!         instance.destroy_instance(None);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! # Structs and borrowing
//!
//! Every struct and union implements `Default`. The default sets `sType` to the value of the
//! struct, pointers to null and the other members to zero, except where zero is not a
//! usable value: `srcQueueFamilyIndex` and `dstQueueFamilyIndex` are
//! `QUEUE_FAMILY_IGNORED`, `VkSampleCountFlagBits` members are `_1`, and the `mipLevels`
//! and `arrayLayers` of `VkImageCreateInfo` are 1. Members are set with builder-style
//! setters named after them, without the `p` or `pp` of pointer members
//! (`pQueuePriorities` is set with `queue_priorities`); `pNext` is set with `push_next`.
//!
//! Structs that can hold pointers carry a lifetime `'a`, and their setters take references
//! and slices with that lifetime, so the struct cannot outlive the data it points to. A
//! setter for an array also sets the member that holds its length; when several arrays
//! share one length member, the setters panic if the lengths differ. For an array whose
//! pointer the registry marks as optional, an empty slice stands for an absent array: the
//! pointer is set to null, the length member is left as it is, and the slice is not
//! compared with the arrays that share the member. Only when the pointer was not null
//! before, and no other array that shares the length member is set, such an empty slice
//! sets the length member to 0. A setter also panics
//! if the slice has more elements than the type of the length member can count, such as
//! more than 255 for a `uint8_t` member. `VkBool32` members
//! take `bool`, and strings take [`&CStr`](core::ffi::CStr) or, in arrays,
//! [`vk::CStrPtr`].
//!
//! ```
//! use vulkan::vk;
//!
//! let priorities = [1.0, 0.5];
//! let queues = [vk::DeviceQueueCreateInfo::default()
//!     .queue_family_index(0)
//!     .queue_priorities(&priorities)];
//! let extensions = [vk::CStrPtr::new(vk::KHR_SWAPCHAIN_NAME)];
//! let features = vk::PhysicalDeviceFeatures::default().sampler_anisotropy(true);
//! let info = vk::DeviceCreateInfo::default()
//!     .queue_create_infos(&queues)
//!     .enabled_extension_names(&extensions)
//!     .enabled_features(&features);
//! assert_eq!(queues[0].queue_count, 2);
//! assert_eq!(info.queue_create_info_count, 1);
//! assert_eq!(info.enabled_extension_count, 1);
//! assert_eq!(features.sampler_anisotropy, vk::TRUE);
//! ```
//!
//! `push_next` adds a struct to the `pNext` chain. It only accepts structs that the
//! registry lists as extending the root struct (see [`vk::Extends`]) and borrows them
//! mutably for the lifetime of the root. Each call puts the new struct at the front of the
//! chain, and overwrites the `pNext` of the pushed struct, so build each struct's own chain
//! through the root and not beforehand:
//!
//! ```
//! use vulkan::vk;
//!
//! let mut vulkan11 = vk::PhysicalDeviceVulkan11Features::default();
//! let mut vulkan12 = vk::PhysicalDeviceVulkan12Features::default().timeline_semaphore(true);
//! let vulkan12_ptr: *mut vk::PhysicalDeviceVulkan12Features<'_> = &mut vulkan12;
//! let features = vk::PhysicalDeviceFeatures2::default()
//!     .push_next(&mut vulkan11)
//!     .push_next(&mut vulkan12);
//! assert_eq!(features.p_next, vulkan12_ptr.cast());
//! ```
//!
//! There is no `build()` step that would erase the lifetime. The fields are public, for
//! reading what a command wrote and for the rare member that has no setter; writing a
//! pointer field directly bypasses the lifetime check.
//!
//! # Commands
//!
//! Every command is an `unsafe fn` method of [`Entry`], [`Instance`] or [`Device`]: the
//! compiler cannot check the valid usage rules of the specification, and the caller must
//! uphold them. Each command comes in two forms:
//!
//! - `device.create_buffer(&info, None)`: references, slices and `Option<&T>` for the
//!   parameters, `bool` for `VkBool32`, and the outputs returned. A command that returns
//!   `VkResult` returns a [`vk::Result`]; commands with more than one success code return
//!   the code, or a value that tells them apart, inside `Ok`, with one exception: commands
//!   that create several objects at once and return them, such as
//!   `create_graphics_pipelines` and `create_shaders_ext`, return
//!   `Err((handles, code))` for every code other than `SUCCESS`, with the handles that the
//!   command wrote. Commands that can be deferred, such as
//!   `create_ray_tracing_pipelines_khr`, instead write their outputs into slices that the
//!   caller passes and keeps, and return the code. Enumerations such as
//!   `enumerate_physical_devices` return a `Vec`.
//! - `device.create_buffer_raw(&info, ptr::null(), &mut buffer)`: the C signature, with raw
//!   pointers and [`vk::VkResult`]. The leading `VkInstance` or `VkDevice` parameter is
//!   taken from `self`.
//!
//! The command names are the C names in snake case without the `vk` prefix:
//! `vkCmdDraw` is `Device::cmd_draw` and `Device::cmd_draw_raw`. Commands on queues and
//! command buffers belong to [`Device`] and take the queue or command buffer as their first
//! parameter.
//!
//! All commands of a level are looked up when the [`Entry`], [`Instance`] or [`Device`] is
//! created. Calling a command whose function pointer the Vulkan loader returned as null
//! panics. Whether a command of an extension or version that is not enabled has a null
//! pointer depends on the loader and the driver; calling it is invalid either way. The
//! wrappers also panic when two slices that the C signature passes with one shared length
//! have different lengths, and when a slice has more elements than the type of its count
//! parameter can count, such as more than `u32::MAX` for a `uint32_t` count.
//!
//! Nothing is destroyed on drop: [`Entry`], [`Instance`] and [`Device`] only hold a handle
//! and a shared function pointer table, and are cheap to clone, `Send` and `Sync`. Destroy
//! objects with their `destroy_*` commands, for example [`Instance::destroy_instance`] and
//! [`Device::destroy_device`].
//!
//! # Registry metadata
//!
//! [`vk`] also carries facts from the registry that the C headers do not: the result codes,
//! level and providers of each command ([`vk::CommandInfo`]), the dependencies and
//! promotions of each extension ([`vk::ExtensionInfo`]), the texel layout of each format
//! ([`vk::FormatInfo`]), and the names of enum and bitmask values.
//!
//! ```
//! use vulkan::vk;
//!
//! let submit = vk::CommandInfo::get("vkQueueSubmit").unwrap();
//! assert_eq!(submit.level, vk::CommandLevel::Device);
//! assert!(submit.can_return(vk::VkResult::ERROR_DEVICE_LOST));
//!
//! let swapchain = vk::ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME).unwrap();
//! let enabled = |name: &core::ffi::CStr| name == vk::KHR_SURFACE_NAME;
//! assert!(swapchain.dependencies_met(vk::API_VERSION_1_0, enabled));
//!
//! let rgba = vk::Format::R8G8B8A8_SRGB.info().unwrap();
//! assert_eq!(rgba.block_size, 4);
//! assert!(rgba.is_srgb());
//!
//! assert_eq!(vk::Format::R8G8B8A8_SRGB.name(), Some("R8G8B8A8_SRGB"));
//! assert_eq!(vk::PresentModeKHR::from_name("MAILBOX"), Some(vk::PresentModeKHR::MAILBOX));
//! let usage = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::STORAGE;
//! assert_eq!(format!("{usage:?}"), "SAMPLED | STORAGE");
//! ```
//!
//! # Cargo features
//!
//! - `std` (default): `Entry::load` and `Entry::load_from` open the Vulkan loader as a
//!   dynamic library at run time, and report failures as `LoadError`. Without this
//!   feature the crate is `no_std` and only needs `alloc`.
//! - `linked`: `Entry::linked` uses the Vulkan loader the program is linked against
//!   (`vulkan-1` on Windows, `vulkan` elsewhere).
//! - `debug` (default): `Debug` for structs. Enums, bitmasks and handles always implement
//!   `Debug`.
//! - `beta`: provisional extensions (`VK_ENABLE_BETA_EXTENSIONS`).
//! - `android`, `directfb`, `fuchsia`, `ggp`, `ios`, `macos`, `metal`, `ohos`, `sci`,
//!   `screen`, `ubm`, `vi`, `wayland`, `win32`, `xcb`, `xlib`, `xlib-xrandr`: the commands
//!   and other items of the extensions that the C headers guard with the matching
//!   `VK_USE_PLATFORM_*` macro. `xlib-xrandr` enables `xlib`.
//!
//! The registry metadata ([`vk::COMMANDS`], [`vk::EXTENSIONS`], [`vk::Format::info`])
//! covers every platform and provisional extension, whatever features are enabled.
//!
//! # Documentation sources
//!
//! The documentation of the generated items in [`vk`] describes the Vulkan API in the words of
//! the [Vulkan Specification](https://github.com/KhronosGroup/Vulkan-Docs): the summary of
//! each type, command and constant, and the descriptions of structure members, command
//! parameters and enumerants, are excerpts from the sources of the Specification at the
//! commit [`vk::DOCS_COMMIT`]. The excerpts are Copyright The Khronos Group Inc. and the
//! other copyright holders named in those sources, and are licensed under the
//! [Creative Commons Attribution 4.0 International License](https://creativecommons.org/licenses/by/4.0/).
//!
//! The excerpts were changed in form: text that depends on versions and extensions is
//! resolved for all of them, the AsciiDoc markup is converted to Markdown, references to the
//! API link to the Rust items, and references to other parts of the Specification are reduced
//! to their text. Names stay the C names. The excerpts are not the Specification: each type,
//! command and constant links to its reference page, and the valid usage of a command is
//! stated only there.
//!
//! Everything else in this documentation, such as what a setter or a wrapper does, describes
//! this crate.

#![no_std]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod device;
mod entry;
mod instance;
mod loader;
pub mod vk;

pub use device::Device;
pub use entry::{Entry, StaticFn};
pub use instance::Instance;
#[cfg(feature = "std")]
pub use loader::LoadError;
pub(crate) use loader::load_pfn;
