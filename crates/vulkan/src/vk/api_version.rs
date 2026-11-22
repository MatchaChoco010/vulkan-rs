/// Packs a Vulkan version number, like the C macro `VK_MAKE_API_VERSION`.
///
/// The fields are not masked, as in C: `variant` must fit in 3 bits, `major` in 7 bits,
/// `minor` in 10 bits and `patch` in 12 bits. The variant of the Vulkan API is 0.
///
/// ```
/// use vulkan::vk;
///
/// assert_eq!(vk::make_api_version(0, 1, 3, 0), vk::API_VERSION_1_3);
/// assert_eq!(vk::api_version_minor(vk::API_VERSION_1_3), 3);
/// ```
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

/// The variant field of a packed version number, like `VK_API_VERSION_VARIANT`.
pub const fn api_version_variant(version: u32) -> u32 {
    version >> 29
}

/// The major version of a packed version number, like `VK_API_VERSION_MAJOR`.
pub const fn api_version_major(version: u32) -> u32 {
    (version >> 22) & 0x7f
}

/// The minor version of a packed version number, like `VK_API_VERSION_MINOR`.
pub const fn api_version_minor(version: u32) -> u32 {
    (version >> 12) & 0x3ff
}

/// The patch version of a packed version number, like `VK_API_VERSION_PATCH`.
pub const fn api_version_patch(version: u32) -> u32 {
    version & 0xfff
}
