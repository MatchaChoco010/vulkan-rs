use core::ffi::{c_char, c_void};
use vulkan::vk;

fn assert_counts_values<T: vk::LayerSettingValue>(ty: vk::LayerSettingTypeEXT, values: &[T]) {
    for len in 0..=values.len() {
        let values = &values[..len];
        let setting = vk::LayerSettingEXT::default().ty(ty).values(values);
        assert_eq!(
            setting.value_count as usize,
            len,
            "{ty:?}: valueCount is the number of values, not the {} bytes that they take",
            size_of_val(values)
        );
        assert_eq!(
            setting.p_values,
            values.as_ptr().cast::<c_void>(),
            "{ty:?}: pValues is the start of the slice"
        );
        assert_eq!(setting.ty, ty, "{ty:?}: values leaves the type as it was");
    }
}

#[test]
fn a_layer_setting_counts_its_values_in_values_of_every_setting_type() {
    assert_counts_values::<vk::Bool32>(
        vk::LayerSettingTypeEXT::BOOL32,
        &[vk::FALSE, vk::TRUE, vk::FALSE],
    );
    assert_counts_values::<i32>(vk::LayerSettingTypeEXT::INT32, &[-1, 0, 1]);
    assert_counts_values::<i64>(vk::LayerSettingTypeEXT::INT64, &[i64::MIN, 0, i64::MAX]);
    assert_counts_values::<u32>(vk::LayerSettingTypeEXT::UINT32, &[0, 1, u32::MAX]);
    assert_counts_values::<u64>(vk::LayerSettingTypeEXT::UINT64, &[0, 1, u64::MAX]);
    assert_counts_values::<f32>(vk::LayerSettingTypeEXT::FLOAT32, &[0.0, 0.5, 1.0]);
    assert_counts_values::<f64>(vk::LayerSettingTypeEXT::FLOAT64, &[0.0, 0.5, 1.0]);
    assert_counts_values::<vk::CStrPtr<'_>>(
        vk::LayerSettingTypeEXT::STRING,
        &[c"error".into(), c"warn".into(), c"info".into()],
    );
    assert_counts_values::<*const c_char>(
        vk::LayerSettingTypeEXT::STRING,
        &[c"error".as_ptr(), c"warn".as_ptr()],
    );
}

#[test]
fn a_bool32_layer_setting_holds_one_value_for_one_bool32() {
    let enabled = [vk::TRUE];
    let setting = vk::LayerSettingEXT::default()
        .layer_name(c"VK_LAYER_KHRONOS_validation")
        .setting_name(c"validate_sync")
        .ty(vk::LayerSettingTypeEXT::BOOL32)
        .values(&enabled);
    assert_eq!(
        setting.value_count, 1,
        "a layer reads valueCount values of VkBool32, so one value is one, not its four bytes"
    );
    let read = unsafe { *setting.p_values.cast::<vk::Bool32>() };
    assert_eq!(read, vk::TRUE);
}

#[test]
fn the_string_pointer_type_has_the_layout_of_a_c_string_pointer() {
    assert_eq!(size_of::<vk::CStrPtr<'_>>(), size_of::<*const c_char>());
    assert_eq!(align_of::<vk::CStrPtr<'_>>(), align_of::<*const c_char>());
    let (error, warn) = (c"error", c"warn");
    let names: [vk::CStrPtr<'_>; 2] = [error.into(), warn.into()];
    let setting = vk::LayerSettingEXT::default()
        .ty(vk::LayerSettingTypeEXT::STRING)
        .values(&names);
    let pointers =
        unsafe { core::slice::from_raw_parts(setting.p_values.cast::<*const c_char>(), 2) };
    assert_eq!(pointers, [error.as_ptr(), warn.as_ptr()]);
    assert_eq!(pointers, [names[0].as_ptr(), names[1].as_ptr()]);
}

trait AmbiguousIfALayerSettingValue<Marker> {
    fn is_not_a_layer_setting_value() {}
}

impl<T: ?Sized> AmbiguousIfALayerSettingValue<()> for T {}

struct IsALayerSettingValue;

impl<T: ?Sized + vk::LayerSettingValue> AmbiguousIfALayerSettingValue<IsALayerSettingValue> for T {}

macro_rules! assert_not_layer_setting_values {
    ($($ty:ty),+ $(,)?) => {
        $(<$ty as AmbiguousIfALayerSettingValue<_>>::is_not_a_layer_setting_value();)+
    };
}

#[test]
fn bytes_and_fat_string_pointers_are_not_values_of_a_layer_setting() {
    assert_not_layer_setting_values!(
        u8,
        i8,
        u16,
        i16,
        bool,
        usize,
        isize,
        [u8; 4],
        &[u8],
        &core::ffi::CStr,
        &str,
        *const c_void,
        (),
    );
    fn is_a_layer_setting_value<T: vk::LayerSettingValue>() {}
    is_a_layer_setting_value::<vk::Bool32>();
    is_a_layer_setting_value::<u32>();
    is_a_layer_setting_value::<i32>();
    is_a_layer_setting_value::<i64>();
    is_a_layer_setting_value::<u64>();
    is_a_layer_setting_value::<f32>();
    is_a_layer_setting_value::<f64>();
    is_a_layer_setting_value::<vk::CStrPtr<'_>>();
    is_a_layer_setting_value::<*const c_char>();
}
