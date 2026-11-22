#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use crate::vk::*;

pub const HEADER_VERSION: u32 = 363;
pub const HEADER_VERSION_COMPLETE: u32 = make_api_version(0, 1, 4, 363);
pub const HEADERS_TAG: &str = "vulkan-sdk-1.4.363.0";
pub const HEADERS_COMMIT: &str = "6802bb4733b6";
pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
pub const API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);
pub const API_VERSION_1_4: u32 = make_api_version(0, 1, 4, 0);
