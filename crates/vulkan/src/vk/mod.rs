//! The Vulkan API as Rust items, one to one with the C API.
//!
//! This module holds the structs, unions, enums, bitmasks, handles, function pointer types,
//! constants and extension names of the Vulkan registry and of the video codec headers,
//! the types of the platform headers that Vulkan refers to (such as [`Display`] and
//! [`HWND`]), and the function pointer tables [`EntryFn`], [`InstanceFn`] and [`DeviceFn`]
//! that [`Entry`](crate::Entry), [`Instance`](crate::Instance) and
//! [`Device`](crate::Device) hold. It also holds facts from the registry that the C headers
//! do not carry: [`CommandInfo`] for each command, [`ExtensionInfo`] for each extension and
//! [`FormatInfo`] for each format.
//!
//! # Names
//!
//! The Rust name of a C item follows from its C name:
//!
//! | C | Rust |
//! |---|---|
//! | `VkBufferCreateInfo`, `VkSwapchainKHR`, `VkImageUsageFlags` | [`BufferCreateInfo`], [`SwapchainKHR`], [`ImageUsageFlags`] |
//! | `VkInstance`, `VkDevice` | the handles [`Instance`], [`Device`] (the crate root's [`crate::Instance`] and [`crate::Device`] wrap them with their commands) |
//! | `VkResult` | [`VkResult`]; [`Result<T>`](Result) is `Result<T, VkResult>` |
//! | `VkBool32`, `VkDeviceSize` | [`Bool32`], [`DeviceSize`] |
//! | `StdVideoH264SpsFlags` | [`StdVideoH264SpsFlags`] |
//! | `PFN_vkCreateBuffer`, `PFN_vkDebugUtilsMessengerCallbackEXT` | [`PFN_vkCreateBuffer`], [`PFN_vkDebugUtilsMessengerCallbackEXT`] |
//! | member `queueFamilyIndex`, `pNext` | field `queue_family_index`, `p_next` |
//! | member `type` | field `ty` |
//! | `VK_FORMAT_R8G8B8A8_UNORM` | `Format::R8G8B8A8_UNORM` |
//! | `VK_SUCCESS`, `VK_ERROR_DEVICE_LOST` | `VkResult::SUCCESS`, `VkResult::ERROR_DEVICE_LOST` |
//! | `VK_PRESENT_MODE_FIFO_KHR`, `VK_COLOR_SPACE_HDR10_ST2084_EXT` | `PresentModeKHR::FIFO`, `ColorSpaceKHR::HDR10_ST2084_EXT` |
//! | `VK_IMAGE_VIEW_TYPE_2D`, `VK_SAMPLE_COUNT_4_BIT` | `ImageViewType::_2D`, `SampleCountFlags::_4` |
//! | `VK_IMAGE_USAGE_SAMPLED_BIT` | `ImageUsageFlags::SAMPLED` and `ImageUsageFlagBits::SAMPLED` |
//! | `VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT` | `PipelineStageFlags2::TOP_OF_PIPE` |
//! | `VK_WHOLE_SIZE`, `VK_TRUE`, `VK_API_VERSION_1_3` | [`WHOLE_SIZE`], [`TRUE`], [`API_VERSION_1_3`] |
//! | `VK_MAKE_API_VERSION`, `VK_API_VERSION_MAJOR` | [`make_api_version`], [`api_version_major`] |
//! | `VK_KHR_SWAPCHAIN_EXTENSION_NAME`, `VK_KHR_SWAPCHAIN_SPEC_VERSION` | [`KHR_SWAPCHAIN_NAME`], [`KHR_SWAPCHAIN_SPEC_VERSION`] |
//! | `vkCreateBuffer` | the methods `Device::create_buffer` and `Device::create_buffer_raw`, the field [`DeviceFn::create_buffer`], and [`command::CREATE_BUFFER`] |
//!
//! In words:
//!
//! - Types drop the `Vk` prefix and keep their vendor suffix, except `VkResult`, which
//!   keeps its name next to the result type [`Result`] of the command wrappers. Names
//!   without the `Vk` prefix, such as the video codec types and the `PFN_vk*` types, are
//!   unchanged.
//! - Struct members become snake case fields. Setters have the same names, without the
//!   `p` or `pp` of pointer members (`pQueuePriorities` is set with `queue_priorities`).
//! - Enum and bitmask values are associated constants of their type. The constant is the
//!   C name without the prefix of the type, without the `_BIT` of bitmask bits, and without
//!   a vendor suffix equal to that of the type; a name that would start with a digit gets
//!   a leading `_`. The bits are constants of both the `*Flags` and the `*FlagBits` type.
//! - Other constants drop the `VK_` prefix. Extension names and versions drop `VK_` and
//!   `_EXTENSION`.
//! - Commands are methods of the [`Entry`](crate::Entry), [`Instance`](crate::Instance)
//!   or [`Device`](crate::Device) that [`CommandInfo::level`] names, in snake case without
//!   the `vk` prefix, with a `_raw` twin; the table field has the same name, and the
//!   [`command`] constant is the name in upper snake case.
//! - Registry aliases are kept: aliased types are type aliases (such as
//!   [`PipelineStageFlagBits2KHR`]) and aliased values are constants with the same value
//!   (such as `PresentModeKHR::FIFO_LATEST_READY_EXT`).
//!
//! Enums and bitmasks are `#[repr(transparent)]` newtypes over the C integer, so values
//! this crate does not know are representable. Their `name` and `from_name` methods convert
//! to and from the constant names (`from_name` also accepts alias names), `variants` lists
//! the known values with their names, and `Debug` prints the names:
//!
//! ```
//! use vulkan::vk;
//!
//! assert_eq!(vk::Format::R16G16B16A16_SFLOAT.name(), Some("R16G16B16A16_SFLOAT"));
//! assert_eq!(vk::ImageViewType::from_name("_2D"), Some(vk::ImageViewType::_2D));
//! assert_eq!(
//!     vk::PresentModeKHR::from_name("FIFO_LATEST_READY_EXT"),
//!     Some(vk::PresentModeKHR::FIFO_LATEST_READY)
//! );
//! assert_eq!(format!("{:?}", vk::Format::from_raw(-5)), "Format(-5)");
//! let stages = vk::ShaderStageFlags::VERTEX | vk::ShaderStageFlags::FRAGMENT;
//! assert_eq!(format!("{stages:?}"), "VERTEX | FRAGMENT");
//! ```

mod api_version;
mod base_types;
mod c_str;
mod chain;
mod command_meta;
mod count;
mod extension;
mod format;
/// The items generated from the Vulkan registry, re-exported at the root of
/// [`vk`](crate::vk). Refer to them through `vk`, for example as `vk::BufferCreateInfo`.
pub mod generated;
mod handle;
mod layer_setting;
mod names;
mod platform;
mod result;

pub use api_version::*;
pub use base_types::*;
pub use c_str::CStrPtr;
pub(crate) use c_str::c_str_from_chars;
pub(crate) use chain::push_chain;
pub use chain::{Extends, TaggedStructure};
pub use command_meta::*;
pub(crate) use count::len_as_count;
pub use extension::*;
pub use format::*;
pub use generated::*;
pub use handle::Handle;
pub use layer_setting::LayerSettingValue;
pub(crate) use names::{debug_enum, debug_flags, name_of, value_of};
pub use platform::*;
pub use result::Result;
