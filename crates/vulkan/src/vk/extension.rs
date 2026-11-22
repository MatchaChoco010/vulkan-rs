use core::ffi::CStr;
use core::fmt;

/// Where an extension is enabled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionLevel {
    /// An instance extension, enabled in `VkInstanceCreateInfo::ppEnabledExtensionNames`.
    Instance,
    /// A device extension, enabled in `VkDeviceCreateInfo::ppEnabledExtensionNames`.
    Device,
}

/// The `depends` expression of an extension in the registry.
///
/// Its `Display` form is the notation of the registry: `VK_VERSION_1_x` for a version, the
/// extension name for an extension, and parenthesized lists joined with `+` for
/// [`All`](Dependency::All) and with `,` for [`Any`](Dependency::Any).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dependency {
    /// The Vulkan version in use is at least this version (`vk::API_VERSION_1_x`).
    Version(u32),
    /// This extension is enabled, or was promoted to something that is in use. See
    /// [`Dependency::is_met`].
    Extension(&'static CStr),
    /// Every one of these holds.
    All(&'static [Dependency]),
    /// At least one of these holds.
    Any(&'static [Dependency]),
}

/// What the registry says about one extension: its level, dependencies and promotion.
///
/// [`EXTENSIONS`](super::EXTENSIONS) lists every extension, sorted by name, and
/// [`ExtensionInfo::get`] looks one up. Both cover the extensions of every platform and
/// the provisional extensions, whatever Cargo features are enabled. This answers what else
/// must be enabled for an extension before `vkCreateInstance` or `vkCreateDevice`.
///
/// ```
/// use core::ffi::CStr;
/// use vulkan::vk::{self, Dependency, ExtensionInfo, ExtensionLevel};
///
/// let swapchain = ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME).unwrap();
/// assert_eq!(swapchain.level, ExtensionLevel::Device);
/// assert_eq!(swapchain.spec_version, vk::KHR_SWAPCHAIN_SPEC_VERSION);
/// assert_eq!(swapchain.depends, Some(Dependency::Extension(vk::KHR_SURFACE_NAME)));
///
/// let enabled = [vk::KHR_SURFACE_NAME];
/// let is_enabled = |name: &CStr| enabled.contains(&name);
/// assert!(swapchain.dependencies_met(vk::API_VERSION_1_0, is_enabled));
/// assert!(!swapchain.dependencies_met(vk::API_VERSION_1_4, |_: &CStr| false));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionInfo {
    /// The name, for example `c"VK_KHR_swapchain"`, the value of `vk::KHR_SWAPCHAIN_NAME`.
    pub name: &'static CStr,
    /// The specification version, the value of the matching `*_SPEC_VERSION` constant.
    pub spec_version: u32,
    /// Whether this is an instance or a device extension.
    pub level: ExtensionLevel,
    /// What has to be in use for this extension to be enabled. `None` if nothing.
    pub depends: Option<Dependency>,
    /// The core version (`vk::API_VERSION_1_x`) that includes this extension, if any.
    pub promoted_to_version: Option<u32>,
    /// The extension this extension was promoted to, if any.
    pub promoted_to_extension: Option<&'static CStr>,
}

impl ExtensionInfo {
    /// Looks an extension up by name, for example
    /// `ExtensionInfo::get(vk::KHR_SWAPCHAIN_NAME)`. Returns `None` for a name that is not
    /// an extension of the registry.
    pub fn get(name: &CStr) -> Option<&'static ExtensionInfo> {
        super::EXTENSIONS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::EXTENSIONS[index])
    }

    /// Whether the dependencies of this extension are satisfied.
    ///
    /// `api_version` is the Vulkan version in use: the lower of the version requested in
    /// `VkApplicationInfo::apiVersion` and the version the instance or physical device
    /// supports. `enabled` tells whether an extension is
    /// enabled, or about to be. A device extension can depend on instance extensions, so
    /// `enabled` has to answer for both the instance and the device extension list.
    ///
    /// Returns `true` if [`depends`](ExtensionInfo::depends) is `None`, otherwise the
    /// result of [`Dependency::is_met`].
    pub fn dependencies_met(&self, api_version: u32, enabled: impl Fn(&CStr) -> bool) -> bool {
        self.depends
            .is_none_or(|depends| depends.is_met(api_version, &enabled))
    }
}

impl Dependency {
    /// Evaluates the expression. See [`ExtensionInfo::dependencies_met`] for `api_version`
    /// and `enabled`.
    ///
    /// A dependency on an extension holds when `enabled` returns `true` for it, when the
    /// extension was promoted to a core version not above `api_version`, or when it was
    /// promoted to another extension for which `enabled` returns `true`. A dependency on
    /// an extension that is not in [`EXTENSIONS`](super::EXTENSIONS) holds only when
    /// `enabled` returns `true` for it.
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
