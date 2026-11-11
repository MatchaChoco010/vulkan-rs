//! An output array inside a query struct stays mutably borrowed while the struct is in use.
use vulkan::vk;

fn query(_: &mut vk::DrmFormatModifierPropertiesListEXT<'_>) {}

fn main() {
    let mut modifiers = [vk::DrmFormatModifierPropertiesEXT::default(); 4];
    let mut list =
        vk::DrmFormatModifierPropertiesListEXT::default().drm_format_modifier_properties(&mut modifiers);
    let first = modifiers[0];
    query(&mut list);
    let _ = first;
}
