use core::ffi::CStr;
use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExtensionLevel {
    Instance,
    Device,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dependency {
    Version(u32),
    Extension(&'static CStr),
    All(&'static [Dependency]),
    Any(&'static [Dependency]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionInfo {
    pub name: &'static CStr,
    pub spec_version: u32,
    pub level: ExtensionLevel,
    pub depends: Option<Dependency>,
    pub promoted_to_version: Option<u32>,
    pub promoted_to_extension: Option<&'static CStr>,
}

impl ExtensionInfo {
    pub fn get(name: &CStr) -> Option<&'static ExtensionInfo> {
        super::EXTENSIONS
            .binary_search_by(|info| info.name.cmp(name))
            .ok()
            .map(|index| &super::EXTENSIONS[index])
    }

    pub fn dependencies_met(&self, api_version: u32, enabled: impl Fn(&CStr) -> bool) -> bool {
        self.depends
            .is_none_or(|depends| depends.is_met(api_version, &enabled))
    }
}

impl Dependency {
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
