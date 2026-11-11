//! Extension metadata from the registry: level, dependencies and promotion.
//!
//! This is what a checking layer needs to answer "what else must be enabled for this
//! extension?" before `vkCreateInstance` or `vkCreateDevice`, instead of learning it
//! from a validation error.

use core::ffi::CStr;
use core::fmt;

/// Where an extension is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionLevel {
    /// `VkInstanceCreateInfo::ppEnabledExtensionNames`
    Instance,
    /// `VkDeviceCreateInfo::ppEnabledExtensionNames`
    Device,
}

/// The `depends` expression of an extension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dependency {
    /// The Vulkan version in use is at least this (`vk::API_VERSION_1_x`).
    Version(u32),
    /// This extension is enabled, or was promoted to something that is in use.
    Extension(&'static CStr),
    /// Every one of these holds.
    All(&'static [Dependency]),
    /// At least one of these holds.
    Any(&'static [Dependency]),
}

/// What the registry says about one extension. See [`ExtensionInfo::get`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionInfo {
    pub name: &'static CStr,
    pub spec_version: u32,
    pub level: ExtensionLevel,
    /// What has to be in use for this extension to be enabled. `None`: nothing.
    pub depends: Option<Dependency>,
    /// The core version (`vk::API_VERSION_1_x`) that includes this extension.
    pub promoted_to_version: Option<u32>,
    /// The extension that replaced this one.
    pub promoted_to_extension: Option<&'static CStr>,
}

impl ExtensionInfo {
    /// Looks an extension up by name, e.g. `ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME)`.
    /// Extensions of every platform are included, whatever Cargo features are enabled.
    pub fn get(name: &CStr) -> Option<&'static ExtensionInfo> {
        // `EXTENSIONS` is sorted by name.
        super::EXTENSIONS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::EXTENSIONS[index])
    }

    /// Whether the dependencies of this extension are satisfied.
    ///
    /// `api_version` is the Vulkan version in use: the lower of the version requested in
    /// `VkApplicationInfo::apiVersion` and the version the instance or device supports.
    /// `enabled` says whether an extension is in the instance or device extension list
    /// that is about to be, or was, enabled. A device extension can depend on instance
    /// extensions, so `enabled` has to answer for both lists.
    pub fn dependencies_met(&self, api_version: u32, enabled: impl Fn(&CStr) -> bool) -> bool {
        self.depends
            .is_none_or(|depends| depends.is_met(api_version, &enabled))
    }
}

impl Dependency {
    /// Evaluates the expression. See [`ExtensionInfo::dependencies_met`].
    ///
    /// A dependency on an extension is also satisfied when that extension was promoted
    /// to a core version that is in use, or to another extension that is enabled. This is
    /// the general rule of the specification; it assumes that the promoted functionality
    /// did not become optional in the promotion.
    pub fn is_met(&self, api_version: u32, enabled: &impl Fn(&CStr) -> bool) -> bool {
        match *self {
            Dependency::Version(version) => api_version >= version,
            Dependency::Extension(name) => {
                if enabled(name) {
                    return true;
                }
                let Some(info) = ExtensionInfo::get(name) else {
                    return false;
                };
                info.promoted_to_version
                    .is_some_and(|version| api_version >= version)
                    || info.promoted_to_extension.is_some_and(enabled)
            }
            Dependency::All(all) => all.iter().all(|d| d.is_met(api_version, enabled)),
            Dependency::Any(any) => any.iter().any(|d| d.is_met(api_version, enabled)),
        }
    }
}

/// Prints the expression in the notation of the registry: `+` is "and", `,` is "or".
impl fmt::Display for Dependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let list = |f: &mut fmt::Formatter<'_>, items: &[Dependency], separator: &str| {
            f.write_str("(")?;
            for (index, item) in items.iter().enumerate() {
                if index != 0 {
                    f.write_str(separator)?;
                }
                fmt::Display::fmt(item, f)?;
            }
            f.write_str(")")
        };
        match *self {
            Dependency::Version(version) => write!(
                f,
                "VK_VERSION_{}_{}",
                super::api_version_major(version),
                super::api_version_minor(version)
            ),
            Dependency::Extension(name) => f.write_str(name.to_str().unwrap_or("?")),
            Dependency::All(all) => list(f, all, "+"),
            Dependency::Any(any) => list(f, any, ","),
        }
    }
}
