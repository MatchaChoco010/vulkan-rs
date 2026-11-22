#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

/// Vulkan header file version number.
///
/// C macro
/// [`VK_HEADER_VERSION`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION.html).
#[doc(alias = "VK_HEADER_VERSION")]
pub const HEADER_VERSION: u32 = 363;
/// Vulkan header file complete version number.
///
/// C macro
/// [`VK_HEADER_VERSION_COMPLETE`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_HEADER_VERSION_COMPLETE.html).
#[doc(alias = "VK_HEADER_VERSION_COMPLETE")]
pub const HEADER_VERSION_COMPLETE: u32 = make_api_version(0, 1, 4, 363);
/// The tag of Khronos Vulkan-Headers that these bindings were generated from.
pub const HEADERS_TAG: &str = "vulkan-sdk-1.4.363.0";
/// The commit of Khronos Vulkan-Headers that these bindings were generated from.
pub const HEADERS_COMMIT: &str = "6802bb4733b6";
/// The commit of Khronos Vulkan-Docs, the source of the Vulkan Specification, that the
/// descriptions in this documentation were taken from.
pub const DOCS_COMMIT: &str = "560d4305e8a6";
/// Return API version number for Vulkan 1.0.
///
/// C macro
/// [`VK_API_VERSION_1_0`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_0.html).
#[doc(alias = "VK_API_VERSION_1_0")]
pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
/// Return API version number for Vulkan 1.1.
///
/// C macro
/// [`VK_API_VERSION_1_1`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_1.html).
#[doc(alias = "VK_API_VERSION_1_1")]
pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
/// Return API version number for Vulkan 1.2.
///
/// C macro
/// [`VK_API_VERSION_1_2`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_2.html).
#[doc(alias = "VK_API_VERSION_1_2")]
pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
/// Return API version number for Vulkan 1.3.
///
/// C macro
/// [`VK_API_VERSION_1_3`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_3.html).
#[doc(alias = "VK_API_VERSION_1_3")]
pub const API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);
/// Return API version number for Vulkan 1.4.
///
/// C macro
/// [`VK_API_VERSION_1_4`](https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_4.html).
#[doc(alias = "VK_API_VERSION_1_4")]
pub const API_VERSION_1_4: u32 = make_api_version(0, 1, 4, 0);
