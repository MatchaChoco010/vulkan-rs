use crate::model::{CType, Model, Scalar, null_ptr};
use crate::naming::const_name;
use vulkan_registry::{Field, Struct};

pub(super) fn default_expr(model: &Model<'_>, s: &Struct, m: &Field) -> String {
    let ty = CType::of(m);
    ty.arrays
        .iter()
        .rev()
        .fold(element_default(model, s, m, &ty), |element, len| {
            format!("[{element}; {}]", model.array_len(len))
        })
}

fn element_default(model: &Model<'_>, s: &Struct, m: &Field, ty: &CType) -> String {
    if m.name == "sType"
        && let Some(s_type) = &s.s_type
    {
        return model.structure_type(s_type);
    }
    if ty.depth() > 0 {
        return null_ptr(ty.outer_const()).to_string();
    }
    if model.is_func_pointer(&m.ty) {
        return "None".to_string();
    }
    let base = model.canonical(&m.ty);
    if let Some(neutral) = vulkan_neutral_value(s, m, base) {
        return neutral;
    }
    match model.scalar(base) {
        Some(Scalar::Float) => "0.0".to_string(),
        Some(Scalar::Integer) => "0".to_string(),
        None if model.has_opaque_representation(base) => {
            "unsafe { core::mem::zeroed() }".to_string()
        }
        None => "Default::default()".to_string(),
    }
}

fn vulkan_neutral_value(s: &Struct, m: &Field, base: &str) -> Option<String> {
    match (s.name.as_str(), m.name.as_str(), base) {
        (_, "srcQueueFamilyIndex" | "dstQueueFamilyIndex", _) => {
            Some(const_name("VK_QUEUE_FAMILY_IGNORED"))
        }
        ("VkImageCreateInfo", "mipLevels" | "arrayLayers", _) => Some("1".to_string()),
        (_, _, "VkSampleCountFlagBits") => Some("SampleCountFlagBits::_1".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{field, registry, structure, tagged};
    use vulkan_registry::{Bitmask, Constant};

    #[test]
    fn defaults_are_the_values_vulkan_treats_as_unset() {
        let mut ir = registry();
        ir.constants.push(Constant {
            name: "VK_UUID_SIZE".to_string(),
            ty: "uint32_t".to_string(),
            value: "16".to_string(),
            ..Default::default()
        });
        ir.bitmasks.push(Bitmask {
            name: "VkSampleCountFlagBits".to_string(),
            flags_name: "VkSampleCountFlags".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.structs
            .push(structure("VkExtent2D", vec![field("width", "uint32_t")]));
        let mut uuid = field("uuid", "uint8_t");
        uuid.fixed_size_array = vec!["VK_UUID_SIZE".to_string()];
        ir.structs.push(tagged(
            "VkImageCreateInfo",
            "VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO",
            vec![
                field("mipLevels", "uint32_t"),
                field("samples", "VkSampleCountFlagBits"),
                field("srcQueueFamilyIndex", "uint32_t"),
                field("scale", "float"),
                field("extent", "VkExtent2D"),
                field("window", "HWND"),
                field("pData", "const uint32_t*"),
                uuid,
            ],
        ));
        let model = Model::new(&ir);
        let s = model.get_struct("VkImageCreateInfo").unwrap();
        let defaults = s
            .members
            .iter()
            .map(|m| default_expr(&model, s, m))
            .collect::<Vec<_>>();
        assert_eq!(
            defaults,
            [
                "StructureType::IMAGE_CREATE_INFO",
                "core::ptr::null()",
                "1",
                "SampleCountFlagBits::_1",
                "QUEUE_FAMILY_IGNORED",
                "0.0",
                "Default::default()",
                "unsafe { core::mem::zeroed() }",
                "core::ptr::null()",
                "[0; UUID_SIZE]",
            ]
        );
    }
}
