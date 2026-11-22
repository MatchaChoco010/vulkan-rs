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
/// the provisional extensions, whatever Cargo features are enabled. This answers which
/// other extensions and which Vulkan version an extension needs before `vkCreateInstance`
/// or `vkCreateDevice`. It does not answer which features the extension needs.
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

    /// Whether the extensions and versions that this extension depends on, which its
    /// [`depends`](ExtensionInfo::depends) expression names, are in use.
    ///
    /// `api_version` is the Vulkan version in use: the lower of the version requested in
    /// `VkApplicationInfo::apiVersion` and the version the instance or physical device
    /// supports. `enabled` tells whether an extension is
    /// enabled, or about to be. A device extension can depend on instance extensions, so
    /// `enabled` has to answer for both the instance and the device extension list.
    ///
    /// Returns `true` if [`depends`](ExtensionInfo::depends) is `None`, otherwise the
    /// result of [`Dependency::is_met`].
    ///
    /// `true` does not say that the features which a dependency stands for are enabled: the
    /// `depends` expression does not state them, and this does not look at features. See
    /// [`Dependency::is_met`] for what the caller still has to check.
    pub fn dependencies_met(&self, api_version: u32, enabled: impl Fn(&CStr) -> bool) -> bool {
        self.depends
            .is_none_or(|depends| depends.is_met(api_version, &enabled))
    }
}

impl Dependency {
    /// Evaluates the expression. See [`ExtensionInfo::dependencies_met`] for `api_version`
    /// and `enabled`.
    ///
    /// A dependency on an extension holds when `enabled` returns `true` for it or for any
    /// extension in its promotion chain, or when the chain ends in a core version not above
    /// `api_version`. The promotion chain follows
    /// [`promoted_to_extension`](ExtensionInfo::promoted_to_extension) from extension to
    /// extension: `VK_EXT_index_type_uint8` was promoted to `VK_KHR_index_type_uint8`, which
    /// was promoted to Vulkan 1.4, so a dependency on `VK_EXT_index_type_uint8` holds with
    /// either extension enabled or on Vulkan 1.4. A dependency on an extension that is not
    /// in [`EXTENSIONS`](super::EXTENSIONS) holds only when `enabled` returns `true` for it.
    ///
    /// # What a dependency that holds does not cover
    ///
    /// This evaluates the `depends` expression, which names extensions and versions, and
    /// the promotions of the extensions in it. Neither states features, and this does not
    /// look at features. The specification, in "Extension Dependencies" of the chapter
    /// "Extending Vulkan", says that the extension or core version to which an extension was
    /// promoted satisfies a dependency on it only as long as the features that the extension
    /// required are required or enabled there; where the promotion made a feature optional,
    /// "the dependency may not be satisfied", and "the only way to be certain is to look at
    /// the descriptions" of the extension and of what it was promoted to. So where a
    /// dependency holds because of another extension or a core version, the caller has to
    /// enable the features that the promotion left optional.
    ///
    /// `VK_KHR_acceleration_structure` depends on `VK_KHR_deferred_host_operations` and,
    /// besides it, either on Vulkan 1.1 with `VK_EXT_descriptor_indexing` and
    /// `VK_KHR_buffer_device_address`, or on Vulkan 1.2, to which those two were promoted.
    /// Vulkan 1.2 made their features optional: an implementation of Vulkan 1.2 that does
    /// not support `VK_KHR_buffer_device_address` does not have to support the
    /// `bufferDeviceAddress` feature. With only `VK_KHR_deferred_host_operations` enabled,
    /// the dependency holds on Vulkan 1.2 all the same, whether or not that feature is
    /// enabled, and it does not hold on Vulkan 1.1, where the two extensions are missing:
    ///
    /// ```
    /// use core::ffi::CStr;
    /// use vulkan::vk::{self, ExtensionInfo};
    ///
    /// let acceleration = ExtensionInfo::get(vk::KHR_ACCELERATION_STRUCTURE_NAME).unwrap();
    /// let enabled = |name: &CStr| name == vk::KHR_DEFERRED_HOST_OPERATIONS_NAME;
    /// assert!(acceleration.dependencies_met(vk::API_VERSION_1_2, enabled));
    /// assert!(!acceleration.dependencies_met(vk::API_VERSION_1_1, enabled));
    /// ```
    pub fn is_met(&self, api_version: u32, enabled: &impl Fn(&CStr) -> bool) -> bool {
        match *self {
            Dependency::Version(version) => api_version >= version,
            Dependency::Extension(name) => {
                extension_is_met(super::EXTENSIONS, name, api_version, enabled)
            }
            Dependency::All(all) => all.iter().all(|d| d.is_met(api_version, enabled)),
            Dependency::Any(any) => any.iter().any(|d| d.is_met(api_version, enabled)),
        }
    }
}

fn extension_is_met(
    table: &[ExtensionInfo],
    name: &CStr,
    api_version: u32,
    enabled: &impl Fn(&CStr) -> bool,
) -> bool {
    if enabled(name) {
        return true;
    }
    let mut current = name;
    for _ in 0..table.len() {
        let Ok(index) = table.binary_search_by(|info| info.name.cmp(current)) else {
            return false;
        };
        let info = &table[index];
        if info
            .promoted_to_version
            .is_some_and(|version| api_version >= version)
        {
            return true;
        }
        let Some(next) = info.promoted_to_extension else {
            return false;
        };
        if enabled(next) {
            return true;
        }
        current = next;
    }
    false
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

#[cfg(test)]
mod tests {
    use super::super::{API_VERSION_1_2, API_VERSION_1_3};
    use super::*;

    const fn extension(
        name: &'static CStr,
        promoted_to_version: Option<u32>,
        promoted_to_extension: Option<&'static CStr>,
    ) -> ExtensionInfo {
        ExtensionInfo {
            name,
            spec_version: 1,
            level: ExtensionLevel::Device,
            depends: None,
            promoted_to_version,
            promoted_to_extension,
        }
    }

    const CHAIN: &[ExtensionInfo] = &[
        extension(c"VK_A", None, Some(c"VK_B")),
        extension(c"VK_B", None, Some(c"VK_C")),
        extension(c"VK_C", Some(API_VERSION_1_3), None),
    ];

    const CYCLE: &[ExtensionInfo] = &[
        extension(c"VK_A", None, Some(c"VK_B")),
        extension(c"VK_B", None, Some(c"VK_C")),
        extension(c"VK_C", None, Some(c"VK_A")),
    ];

    #[test]
    fn every_extension_of_a_chain_and_its_final_core_version_meet_the_first() {
        let nothing = |_: &CStr| false;
        assert!(extension_is_met(CHAIN, c"VK_A", API_VERSION_1_3, &nothing));
        assert!(!extension_is_met(CHAIN, c"VK_A", API_VERSION_1_2, &nothing));
        for successor in [c"VK_B", c"VK_C"] {
            assert!(extension_is_met(
                CHAIN,
                c"VK_A",
                API_VERSION_1_2,
                &|name: &CStr| name == successor
            ));
        }
        assert!(!extension_is_met(
            CHAIN,
            c"VK_B",
            API_VERSION_1_2,
            &|name: &CStr| name == c"VK_A"
        ));
    }

    #[test]
    fn a_promotion_cycle_ends_the_search_instead_of_looping() {
        let nothing = |_: &CStr| false;
        assert!(!extension_is_met(CYCLE, c"VK_A", API_VERSION_1_3, &nothing));
        assert!(extension_is_met(
            CYCLE,
            c"VK_B",
            API_VERSION_1_3,
            &|name: &CStr| name == c"VK_A"
        ));
    }
}
