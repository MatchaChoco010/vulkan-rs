use super::Model;
use std::collections::BTreeSet;
use std::fmt;

pub(crate) const BETA_PROTECT: &str = "VK_ENABLE_BETA_EXTENSIONS";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Cfg {
    features: BTreeSet<String>,
}

impl Cfg {
    pub(crate) fn is_unconditional(&self) -> bool {
        self.features.is_empty()
    }

    pub(crate) fn is_implied_by(&self, other: &Cfg) -> bool {
        self.features.is_subset(&other.features)
    }

    pub(crate) fn and(mut self, other: Cfg) -> Cfg {
        self.features.extend(other.features);
        self
    }

    pub(crate) fn or_else(self, fallback: impl FnOnce() -> Cfg) -> Cfg {
        if self.is_unconditional() {
            fallback()
        } else {
            self
        }
    }
}

impl fmt::Display for Cfg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let features = self
            .features
            .iter()
            .map(|feature| format!("feature = \"{feature}\""))
            .collect::<Vec<_>>();
        match features.as_slice() {
            [] => Ok(()),
            [feature] => writeln!(f, "#[cfg({feature})]"),
            features => writeln!(f, "#[cfg(all({}))]", features.join(", ")),
        }
    }
}

impl Model<'_> {
    pub(crate) fn feature(&self, protect: &str) -> String {
        if protect == BETA_PROTECT {
            return "beta".to_string();
        }
        self.ir
            .platforms
            .iter()
            .find(|(_, macro_name)| macro_name.as_str() == protect)
            .map(|(platform, _)| platform.replace('_', "-"))
            .unwrap_or_else(|| panic!("unhandled Vulkan protect macro: {protect}"))
    }

    pub(crate) fn features(&self) -> Vec<String> {
        self.ir
            .platforms
            .values()
            .map(|protect| self.feature(protect))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub(crate) fn cfg(&self, protect: Option<&str>) -> Cfg {
        Cfg {
            features: protect
                .iter()
                .map(|protect| self.feature(protect))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::registry;

    #[test]
    fn features_come_from_registry_platforms() {
        let mut ir = registry();
        ir.platforms.insert(
            "xlib_xrandr".to_string(),
            "VK_USE_PLATFORM_XLIB_XRANDR_EXT".to_string(),
        );
        ir.platforms.insert(
            "wayland".to_string(),
            "VK_USE_PLATFORM_WAYLAND_KHR".to_string(),
        );
        let model = Model::new(&ir);
        assert_eq!(
            model.feature("VK_USE_PLATFORM_XLIB_XRANDR_EXT"),
            "xlib-xrandr"
        );
        assert_eq!(model.feature(BETA_PROTECT), "beta");
        assert_eq!(model.features(), vec!["wayland", "xlib-xrandr"]);
    }

    #[test]
    fn cfg_renders_as_an_attribute_line_or_nothing() {
        let mut ir = registry();
        ir.platforms.insert(
            "wayland".to_string(),
            "VK_USE_PLATFORM_WAYLAND_KHR".to_string(),
        );
        let model = Model::new(&ir);
        let beta = model.cfg(Some(BETA_PROTECT));
        let wayland = model.cfg(Some("VK_USE_PLATFORM_WAYLAND_KHR"));
        assert_eq!(model.cfg(None).to_string(), "");
        assert_eq!(beta.to_string(), "#[cfg(feature = \"beta\")]\n");
        assert_eq!(beta.clone().and(beta.clone()), beta);
        assert_eq!(
            beta.clone().and(wayland.clone()).to_string(),
            "#[cfg(all(feature = \"beta\", feature = \"wayland\"))]\n"
        );
        assert_eq!(model.cfg(None).or_else(|| wayland.clone()), wayland);
        assert_eq!(beta.clone().or_else(|| wayland.clone()), beta);
    }
}
