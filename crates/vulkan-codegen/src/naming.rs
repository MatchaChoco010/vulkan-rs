//! C identifier to Rust identifier mapping.

/// `VkBufferCreateInfo` to `BufferCreateInfo`. `VkResult` keeps its prefix because
/// `vk::Result<T>` is the result alias.
pub fn type_name(name: &str) -> String {
    if name == "VkResult" {
        return name.to_string();
    }
    name.strip_prefix("Vk").unwrap_or(name).to_string()
}

/// `vkCreateBuffer` to `create_buffer`.
pub fn command_name(name: &str) -> String {
    field_name(name.strip_prefix("vk").unwrap_or(name))
}

/// `pQueueCreateInfos` to `p_queue_create_infos`.
pub fn field_name(name: &str) -> String {
    raw_ident(&to_snake(name))
}

/// `VK_MAX_MEMORY_TYPES` to `MAX_MEMORY_TYPES`.
pub fn const_name(name: &str) -> String {
    raw_ident(
        &name
            .strip_prefix("VK_")
            .unwrap_or(name)
            .to_ascii_uppercase(),
    )
}

/// `VK_KHR_SWAPCHAIN_EXTENSION_NAME` to `KHR_SWAPCHAIN_NAME`.
pub fn extension_name_const(name_macro: &str) -> String {
    const_name(name_macro).replace("_EXTENSION_NAME", "_NAME")
}

/// Name of the setter that takes a reference or slice for a pointer member:
/// `pQueueCreateInfos` to `queue_create_infos`, `ppEnabledLayerNames` to `enabled_layer_names`.
pub fn pointer_setter_name(name: &str) -> String {
    let bytes = name.as_bytes();
    if name.starts_with("pp") && bytes.len() > 2 && bytes[2].is_ascii_uppercase() {
        return field_name(&name[2..]);
    }
    if name.starts_with('p') && bytes.len() > 1 && bytes[1].is_ascii_uppercase() {
        return field_name(&name[1..]);
    }
    field_name(name)
}

/// Splits a trailing vendor tag off a type name: `PresentModeKHR` to (`PresentMode`, `KHR`).
pub fn split_vendor_tag<'a>(
    name: &'a str,
    vendor_tags: &'a [String],
) -> (&'a str, Option<&'a str>) {
    // Prefer the longest tag so that `NVX` is not read as `X` after `NV`.
    let mut best: Option<&str> = None;
    for tag in vendor_tags {
        if name.ends_with(tag.as_str())
            && name.len() > tag.len()
            && best.is_none_or(|b| tag.len() > b.len())
        {
            best = Some(tag.as_str());
        }
    }
    match best {
        Some(tag) => (&name[..name.len() - tag.len()], Some(tag)),
        None => (name, None),
    }
}

/// `VK_PRESENT_MODE_IMMEDIATE_KHR` in `VkPresentModeKHR` to `IMMEDIATE`.
///
/// The enum type prefix is removed, and so is the vendor tag when it repeats the tag of
/// the enum type. A foreign tag stays: `VK_COLOR_SPACE_HDR10_ST2084_EXT` in
/// `VkColorSpaceKHR` becomes `HDR10_ST2084_EXT`.
pub fn enum_variant_name(enum_name: &str, value: &str, vendor_tags: &[String]) -> String {
    let rust_type = type_name(enum_name);
    let (base, tag) = split_vendor_tag(&rust_type, vendor_tags);
    let prefix = format!("{}_", to_screaming(base));
    let mut name = value.strip_prefix("VK_").unwrap_or(value);
    name = name.strip_prefix(prefix.as_str()).unwrap_or(name);
    name = strip_tag_suffix(name, tag);
    raw_ident(name)
}

/// `VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT` in `VkImageUsageFlagBits` to `COLOR_ATTACHMENT`.
///
/// `_BIT` is dropped. `VK_ACCESS_2_SHADER_READ_BIT` in `VkAccessFlagBits2` becomes
/// `SHADER_READ`.
pub fn flag_bit_name(bitmask_name: &str, value: &str, vendor_tags: &[String]) -> String {
    let rust_type = type_name(bitmask_name);
    let (base, tag) = split_vendor_tag(&rust_type, vendor_tags);
    let screaming = to_screaming(base);
    let prefix = match screaming.split_once("_FLAG_BITS") {
        Some((head, "")) => format!("{head}_"),
        Some((head, digits)) => format!("{head}_{digits}_"),
        None => format!("{screaming}_"),
    };
    let mut name = value.strip_prefix("VK_").unwrap_or(value).to_string();
    if let Some(rest) = name.strip_prefix(prefix.as_str()) {
        name = rest.to_string();
    }
    // `FOO_BIT` or `FOO_BIT_KHR`
    if let Some(rest) = name.strip_suffix("_BIT") {
        name = rest.to_string();
    } else if let Some(pos) = name.rfind("_BIT_") {
        let vendor = &name[pos + "_BIT_".len()..];
        if vendor_tags.iter().any(|t| t == vendor) {
            name = format!("{}_{}", &name[..pos], vendor);
        }
    }
    raw_ident(strip_tag_suffix(&name, tag))
}

fn strip_tag_suffix<'a>(name: &'a str, tag: Option<&str>) -> &'a str {
    let Some(tag) = tag else { return name };
    match name.strip_suffix(tag) {
        Some(rest) if rest.ends_with('_') && rest.len() > 1 => &rest[..rest.len() - 1],
        _ => name,
    }
}

/// Makes a name a valid Rust identifier: keywords are renamed or escaped, and a name that
/// starts with a digit gets a leading underscore.
pub fn raw_ident(name: &str) -> String {
    if name.as_bytes().first().is_some_and(|b| b.is_ascii_digit()) {
        return format!("_{name}");
    }
    match name {
        // `type` is common enough in Vulkan (`VkDescriptorPoolSize::type`) that `r#type`
        // at every use is a nuisance. `ty` is the conventional Rust spelling.
        "type" => "ty".to_string(),
        "match" | "ref" | "box" | "loop" | "move" | "where" | "use" | "fn" | "mod" | "in"
        | "as" | "pub" | "const" | "static" | "struct" | "enum" | "union" | "trait" | "impl"
        | "for" | "while" | "async" | "await" | "dyn" | "final" | "override" | "gen" => {
            format!("r#{name}")
        }
        // These cannot be raw identifiers.
        "self" | "Self" | "super" | "crate" => format!("{name}_"),
        _ => name.to_string(),
    }
}

pub fn to_snake(name: &str) -> String {
    let mut out = String::new();
    let chars = name.chars().collect::<Vec<_>>();
    for (i, ch) in chars.iter().copied().enumerate() {
        if ch.is_ascii_uppercase() {
            let prev = i.checked_sub(1).and_then(|i| chars.get(i)).copied();
            let next = chars.get(i + 1).copied();
            if i != 0
                && prev != Some('_')
                && (prev.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                    || next.is_some_and(|c| c.is_ascii_lowercase()))
            {
                out.push('_');
            }
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push(ch);
        }
    }
    out
}

pub fn to_screaming(name: &str) -> String {
    to_snake(name).to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tags() -> Vec<String> {
        ["KHR", "EXT", "NV", "NVX", "AMD", "QCOM", "ARM"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn type_and_command_names() {
        assert_eq!(type_name("VkBufferCreateInfo"), "BufferCreateInfo");
        assert_eq!(type_name("VkResult"), "VkResult");
        assert_eq!(type_name("StdVideoH264SpsFlags"), "StdVideoH264SpsFlags");
        assert_eq!(command_name("vkCreateBuffer"), "create_buffer");
        assert_eq!(
            command_name("vkGetPhysicalDeviceSurfaceFormats2KHR"),
            "get_physical_device_surface_formats2_khr"
        );
        assert_eq!(command_name("vkCmdDrawIndexed"), "cmd_draw_indexed");
    }

    #[test]
    fn field_names_escape_keywords() {
        assert_eq!(field_name("sType"), "s_type");
        assert_eq!(field_name("pNext"), "p_next");
        assert_eq!(field_name("type"), "ty");
        assert_eq!(field_name("match"), "r#match");
        assert_eq!(field_name("ppEnabledLayerNames"), "pp_enabled_layer_names");
        assert_eq!(field_name("ScalingList4x4"), "scaling_list4x4");
        assert_eq!(field_name("self"), "self_");
    }

    #[test]
    fn pointer_setter_names_drop_hungarian_prefix() {
        assert_eq!(
            pointer_setter_name("pQueueCreateInfos"),
            "queue_create_infos"
        );
        assert_eq!(
            pointer_setter_name("ppEnabledLayerNames"),
            "enabled_layer_names"
        );
        assert_eq!(pointer_setter_name("pipeline"), "pipeline");
        assert_eq!(pointer_setter_name("display"), "display");
    }

    #[test]
    fn enum_variants_drop_type_prefix_and_own_vendor_tag() {
        let tags = tags();
        assert_eq!(
            enum_variant_name("VkPresentModeKHR", "VK_PRESENT_MODE_IMMEDIATE_KHR", &tags),
            "IMMEDIATE"
        );
        assert_eq!(
            enum_variant_name(
                "VkPresentModeKHR",
                "VK_PRESENT_MODE_FIFO_LATEST_READY_EXT",
                &tags
            ),
            "FIFO_LATEST_READY_EXT"
        );
        assert_eq!(
            enum_variant_name("VkColorSpaceKHR", "VK_COLOR_SPACE_HDR10_ST2084_EXT", &tags),
            "HDR10_ST2084_EXT"
        );
        assert_eq!(
            enum_variant_name("VkFormat", "VK_FORMAT_R8G8B8A8_UNORM", &tags),
            "R8G8B8A8_UNORM"
        );
        assert_eq!(
            enum_variant_name("VkResult", "VK_ERROR_OUT_OF_DATE_KHR", &tags),
            "ERROR_OUT_OF_DATE_KHR"
        );
        assert_eq!(
            enum_variant_name("VkImageViewType", "VK_IMAGE_VIEW_TYPE_2D", &tags),
            "_2D"
        );
        assert_eq!(
            enum_variant_name(
                "StdVideoH264ChromaFormatIdc",
                "STD_VIDEO_H264_CHROMA_FORMAT_IDC_420",
                &tags
            ),
            "_420"
        );
    }

    #[test]
    fn flag_bits_drop_bit_and_own_vendor_tag() {
        let tags = tags();
        assert_eq!(
            flag_bit_name(
                "VkImageUsageFlagBits",
                "VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT",
                &tags
            ),
            "COLOR_ATTACHMENT"
        );
        assert_eq!(
            flag_bit_name(
                "VkBufferUsageFlagBits",
                "VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR",
                &tags
            ),
            "SHADER_DEVICE_ADDRESS_KHR"
        );
        assert_eq!(
            flag_bit_name("VkAccessFlagBits2", "VK_ACCESS_2_SHADER_READ_BIT", &tags),
            "SHADER_READ"
        );
        assert_eq!(
            flag_bit_name(
                "VkCompositeAlphaFlagBitsKHR",
                "VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR",
                &tags
            ),
            "OPAQUE"
        );
        assert_eq!(
            flag_bit_name(
                "VkDebugUtilsMessageSeverityFlagBitsEXT",
                "VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT",
                &tags
            ),
            "ERROR"
        );
        assert_eq!(
            flag_bit_name("VkSampleCountFlagBits", "VK_SAMPLE_COUNT_1_BIT", &tags),
            "_1"
        );
        assert_eq!(
            flag_bit_name("VkCullModeFlagBits", "VK_CULL_MODE_FRONT_AND_BACK", &tags),
            "FRONT_AND_BACK"
        );
        assert_eq!(
            flag_bit_name(
                "VkExternalMemoryHandleTypeFlagBits",
                "VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT",
                &tags
            ),
            "DMA_BUF_EXT"
        );
    }

    #[test]
    fn vendor_tag_split_prefers_longest_tag() {
        let tags = tags();
        assert_eq!(
            split_vendor_tag("PresentModeKHR", &tags),
            ("PresentMode", Some("KHR"))
        );
        assert_eq!(split_vendor_tag("FooNVX", &tags), ("Foo", Some("NVX")));
        assert_eq!(split_vendor_tag("Format", &tags), ("Format", None));
    }
}
