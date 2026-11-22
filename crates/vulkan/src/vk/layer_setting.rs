use super::CStrPtr;
use core::ffi::c_char;

mod sealed {
    pub trait Sealed {}
}

/// A type that the values of a layer setting can have: the element type of the slice that
/// [`LayerSettingEXT::values`](super::LayerSettingEXT::values) takes.
///
/// `VkLayerSettingEXT::pValues` points to `valueCount` values of the type that
/// `VkLayerSettingEXT::type` names. The trait is implemented for the types that
/// [`LayerSettingTypeEXT`](super::LayerSettingTypeEXT) can name, and it is sealed, so no
/// other type implements it:
///
/// - `BOOL32`: [`Bool32`](super::Bool32), which is `u32`
/// - `INT32`: `i32`
/// - `INT64`: `i64`
/// - `UINT32`: `u32`
/// - `UINT64`: `u64`
/// - `FLOAT32`: `f32`
/// - `FLOAT64`: `f64`
/// - `STRING`: [`CStrPtr`] or `*const c_char`, a pointer to a NUL-terminated string
///
/// The trait does not tie the type to the value of `type`: a setting has to be given the
/// type that its `type` names.
///
/// The bytes of a value are not values of a setting. A slice of bytes does not compile:
///
/// ```compile_fail,E0277
/// use vulkan::vk;
///
/// let enabled = vk::TRUE.to_ne_bytes();
/// let setting = vk::LayerSettingEXT::default()
///     .setting_name(c"validate_sync")
///     .ty(vk::LayerSettingTypeEXT::BOOL32)
///     .values(&enabled);
/// ```
///
/// Pass a slice of the values themselves, and `valueCount` is their number:
///
/// ```
/// use vulkan::vk;
///
/// let enabled = [vk::TRUE];
/// let setting = vk::LayerSettingEXT::default()
///     .setting_name(c"validate_sync")
///     .ty(vk::LayerSettingTypeEXT::BOOL32)
///     .values(&enabled);
/// assert_eq!(setting.value_count, 1);
/// ```
///
/// A `&CStr` is a pointer and a length, not the pointer that C reads, so a slice of them
/// does not compile either:
///
/// ```compile_fail,E0277
/// use vulkan::vk;
///
/// let levels = [c"error", c"warn"];
/// let setting = vk::LayerSettingEXT::default()
///     .setting_name(c"report_flags")
///     .ty(vk::LayerSettingTypeEXT::STRING)
///     .values(&levels);
/// ```
///
/// Pass the strings as [`CStrPtr`]:
///
/// ```
/// use vulkan::vk;
///
/// let levels = [vk::CStrPtr::new(c"error"), vk::CStrPtr::new(c"warn")];
/// let setting = vk::LayerSettingEXT::default()
///     .setting_name(c"report_flags")
///     .ty(vk::LayerSettingTypeEXT::STRING)
///     .values(&levels);
/// assert_eq!(setting.value_count, 2);
/// ```
pub trait LayerSettingValue: sealed::Sealed {}

macro_rules! layer_setting_values {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl sealed::Sealed for $ty {}
            impl LayerSettingValue for $ty {}
        )+
    };
}

layer_setting_values!(u32, i32, i64, u64, f32, f64, *const c_char, CStrPtr<'_>);
