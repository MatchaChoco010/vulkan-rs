const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield", "_",
];

const NOT_RAW_IDENTIFIERS: &[&str] = &["self", "Self", "super", "crate", "_"];

fn is_identifier(name: &str) -> bool {
    let plain = match name.strip_prefix("r#") {
        Some(rest) if NOT_RAW_IDENTIFIERS.contains(&rest) => return false,
        Some(rest) => rest,
        None if KEYWORDS.contains(&name) => return false,
        None => name,
    };
    let mut chars = plain.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn identifier(c_name: &str, rust_name: String) -> String {
    assert!(
        is_identifier(&rust_name),
        "the C name `{c_name}` becomes the Rust name `{rust_name}`, which is not a valid Rust \
         identifier"
    );
    rust_name
}

pub(crate) fn type_name(name: &str) -> String {
    if name == "VkResult" {
        return name.to_string();
    }
    identifier(name, name.strip_prefix("Vk").unwrap_or(name).to_string())
}

pub(crate) fn func_pointer_name(name: &str) -> String {
    identifier(name, name.to_string())
}

pub(crate) fn command_name(name: &str) -> String {
    identifier(name, snake_ident(name.strip_prefix("vk").unwrap_or(name)))
}

pub(crate) fn command_const_name(name: &str) -> String {
    let method = snake_ident(name.strip_prefix("vk").unwrap_or(name));
    identifier(name, method.trim_start_matches("r#").to_ascii_uppercase())
}

pub(crate) fn field_name(name: &str) -> String {
    identifier(name, snake_ident(name))
}

pub(crate) fn const_name(name: &str) -> String {
    identifier(name, screaming_ident(name))
}

pub(crate) fn extension_name_const(name_macro: &str) -> String {
    identifier(
        name_macro,
        screaming_ident(name_macro).replace("_EXTENSION_NAME", "_NAME"),
    )
}

pub(crate) fn pointer_setter_name(name: &str) -> String {
    let bytes = name.as_bytes();
    let short = if name.starts_with("pp") && bytes.len() > 2 && bytes[2].is_ascii_uppercase() {
        &name[2..]
    } else if name.starts_with('p') && bytes.len() > 1 && bytes[1].is_ascii_uppercase() {
        &name[1..]
    } else {
        name
    };
    identifier(name, snake_ident(short))
}

fn snake_ident(name: &str) -> String {
    raw_ident(&to_snake(name))
}

fn screaming_ident(name: &str) -> String {
    raw_ident(
        &name
            .strip_prefix("VK_")
            .unwrap_or(name)
            .to_ascii_uppercase(),
    )
}

fn split_vendor_tag<'a>(name: &'a str, vendor_tags: &'a [String]) -> (&'a str, Option<&'a str>) {
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

pub(crate) fn enum_variant_name(enum_name: &str, value: &str, vendor_tags: &[String]) -> String {
    let rust_type = type_name(enum_name);
    let (base, tag) = split_vendor_tag(&rust_type, vendor_tags);
    let prefix = format!("{}_", to_screaming(base));
    let mut name = value.strip_prefix("VK_").unwrap_or(value);
    name = name.strip_prefix(prefix.as_str()).unwrap_or(name);
    name = strip_tag_suffix(name, tag);
    identifier(value, raw_ident(name))
}

pub(crate) fn flag_bit_name(bitmask_name: &str, value: &str, vendor_tags: &[String]) -> String {
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
    if let Some(rest) = name.strip_suffix("_BIT") {
        name = rest.to_string();
    } else if let Some(pos) = name.rfind("_BIT_") {
        let vendor = &name[pos + "_BIT_".len()..];
        if vendor_tags.iter().any(|t| t == vendor) {
            name = format!("{}_{}", &name[..pos], vendor);
        }
    }
    identifier(value, raw_ident(strip_tag_suffix(&name, tag)))
}

fn strip_tag_suffix<'a>(name: &'a str, tag: Option<&str>) -> &'a str {
    let Some(tag) = tag else { return name };
    match name.strip_suffix(tag) {
        Some(rest) if rest.ends_with('_') && rest.len() > 1 => &rest[..rest.len() - 1],
        _ => name,
    }
}

fn raw_ident(name: &str) -> String {
    if name.as_bytes().first().is_some_and(|b| b.is_ascii_digit()) {
        return format!("_{name}");
    }
    if name == "type" {
        return "ty".to_string();
    }
    if NOT_RAW_IDENTIFIERS.contains(&name) {
        return format!("{name}_");
    }
    if KEYWORDS.contains(&name) {
        return format!("r#{name}");
    }
    name.to_string()
}

fn to_snake(name: &str) -> String {
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

fn to_screaming(name: &str) -> String {
    to_snake(name).to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_support::{RUST_KEYWORDS, catch_quietly, is_rust_identifier, proptest_config};
    use proptest::prelude::*;
    use proptest::sample::select;
    use vulkan_registry::Registry;

    fn tags() -> Vec<String> {
        ["KHR", "EXT", "NV", "NVX", "AMD", "QCOM", "ARM"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    fn capitalized(word: &str) -> String {
        let mut chars = word.chars();
        chars
            .next()
            .map(|first| first.to_ascii_uppercase().to_string() + chars.as_str())
            .unwrap_or_default()
    }

    fn c_identifier() -> impl Strategy<Value = String> {
        let piece = prop_oneof![
            select(
                &[
                    "Vk",
                    "vk",
                    "VK_",
                    "p",
                    "pp",
                    "PFN_",
                    "_",
                    "_BIT",
                    "_EXTENSION_NAME"
                ][..]
            )
            .prop_map(str::to_string),
            select(RUST_KEYWORDS).prop_map(str::to_string),
            select(RUST_KEYWORDS).prop_map(capitalized),
            select(RUST_KEYWORDS).prop_map(str::to_ascii_uppercase),
            "[A-Za-z0-9_]{1,4}",
        ];
        proptest::collection::vec(piece, 1..5)
            .prop_map(|pieces| pieces.concat())
            .prop_filter("a C identifier does not start with a digit", |name| {
                !name.starts_with(|c: char| c.is_ascii_digit())
            })
    }

    fn assert_identifier_or_panic(function: &str, input: &str, convert: impl FnOnce() -> String) {
        if let Ok(name) = catch_quietly(convert) {
            assert!(
                is_rust_identifier(&name),
                "{function}({input:?}) returned `{name}`, which is not a Rust identifier"
            );
        }
    }

    #[test]
    fn raw_ident_turns_every_rust_keyword_into_an_identifier() {
        for keyword in RUST_KEYWORDS {
            let name = raw_ident(keyword);
            assert!(
                is_rust_identifier(&name),
                "raw_ident({keyword:?}) returned `{name}`, which is not a Rust identifier"
            );
        }
    }

    proptest! {
        #![proptest_config(proptest_config(512))]

        #[test]
        fn raw_ident_turns_every_c_identifier_into_a_rust_identifier(name in c_identifier()) {
            let rust = raw_ident(&name);
            prop_assert!(
                is_rust_identifier(&rust),
                "raw_ident({:?}) returned `{}`, which is not a Rust identifier", name, rust
            );
        }

        #[test]
        fn every_name_conversion_returns_a_rust_identifier_or_panics(
            a in c_identifier(),
            b in c_identifier(),
        ) {
            let tags = tags();
            assert_identifier_or_panic("type_name", &a, || type_name(&a));
            assert_identifier_or_panic("func_pointer_name", &a, || func_pointer_name(&a));
            assert_identifier_or_panic("command_name", &a, || command_name(&a));
            assert_identifier_or_panic("command_const_name", &a, || command_const_name(&a));
            assert_identifier_or_panic("field_name", &a, || field_name(&a));
            assert_identifier_or_panic("const_name", &a, || const_name(&a));
            assert_identifier_or_panic("extension_name_const", &a, || extension_name_const(&a));
            assert_identifier_or_panic("pointer_setter_name", &a, || pointer_setter_name(&a));
            let pair = format!("{a:?}, {b:?}");
            assert_identifier_or_panic("enum_variant_name", &pair, || {
                enum_variant_name(&a, &b, &tags)
            });
            assert_identifier_or_panic("flag_bit_name", &pair, || flag_bit_name(&a, &b, &tags));
        }
    }

    #[test]
    fn names_that_are_not_identifiers_panic_with_the_c_name() {
        type Convert = fn() -> String;
        let cases: [(&str, Convert); 6] = [
            ("`Vk2D`", || type_name("Vk2D")),
            ("`VkSelf`", || type_name("VkSelf")),
            ("`vk`", || command_name("vk")),
            ("`VK_`", || const_name("VK_")),
            ("`VK_FOO_`", || enum_variant_name("VkFoo", "VK_FOO_", &[])),
            ("`naïve`", || field_name("naïve")),
        ];
        for (c_name, convert) in cases {
            let message =
                catch_quietly(convert).expect_err(&format!("converting {c_name} does not panic"));
            assert!(
                message.contains(c_name) && message.contains("not a valid Rust identifier"),
                "the panic message `{message}` does not name {c_name}"
            );
        }
    }

    #[test]
    fn every_registry_name_converts_to_a_rust_identifier() {
        let ir = Registry::get();
        let mut converted = Vec::new();
        let types = ir
            .handles
            .iter()
            .flat_map(|h| std::iter::once(&h.name).chain(&h.aliases))
            .chain(
                ir.enums
                    .iter()
                    .flat_map(|e| std::iter::once(&e.name).chain(&e.aliases)),
            )
            .chain(
                ir.bitmasks
                    .iter()
                    .flat_map(|b| [&b.name, &b.flags_name].into_iter().chain(&b.aliases)),
            )
            .chain(
                ir.flags
                    .iter()
                    .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases)),
            )
            .chain(
                ir.structs
                    .iter()
                    .flat_map(|s| std::iter::once(&s.name).chain(&s.aliases)),
            );
        for name in types {
            converted.push((name.clone(), type_name(name)));
        }
        for fp in &ir.func_pointers {
            converted.push((fp.name.clone(), func_pointer_name(&fp.name)));
            for p in &fp.params {
                converted.push((p.name.clone(), field_name(&p.name)));
            }
        }
        for c in &ir.commands {
            converted.push((c.name.clone(), command_name(&c.name)));
            converted.push((c.name.clone(), command_const_name(&c.name)));
            let pfn = format!("PFN_{}", c.name);
            converted.push((pfn.clone(), func_pointer_name(&pfn)));
        }
        let fields = ir
            .structs
            .iter()
            .flat_map(|s| &s.members)
            .chain(ir.commands.iter().flat_map(|c| &c.params));
        for f in fields {
            converted.push((f.name.clone(), field_name(&f.name)));
            converted.push((f.name.clone(), pointer_setter_name(&f.name)));
            for len in f
                .fixed_size_array
                .iter()
                .filter(|len| len.starts_with("VK_"))
            {
                converted.push((len.clone(), const_name(len)));
            }
        }
        let constants = ir
            .constants
            .iter()
            .map(|c| &c.name)
            .chain(ir.versions.iter().map(|v| &v.name_api))
            .chain(ir.extensions.iter().map(|e| &e.spec_version_macro));
        for name in constants {
            converted.push((name.clone(), const_name(name)));
        }
        for e in &ir.extensions {
            converted.push((e.name_macro.clone(), extension_name_const(&e.name_macro)));
        }
        for e in &ir.enums {
            for value in e
                .fields
                .iter()
                .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases))
            {
                converted.push((
                    value.clone(),
                    enum_variant_name(&e.name, value, &ir.vendor_tags),
                ));
            }
        }
        for b in &ir.bitmasks {
            for value in b
                .fields
                .iter()
                .flat_map(|f| std::iter::once(&f.name).chain(&f.aliases))
            {
                converted.push((
                    value.clone(),
                    flag_bit_name(&b.name, value, &ir.vendor_tags),
                ));
            }
        }
        assert!(
            converted.len() > 10_000,
            "the registry has {} names",
            converted.len()
        );
        for (c_name, rust_name) in converted {
            assert!(
                is_rust_identifier(&rust_name),
                "the C name `{c_name}` becomes `{rust_name}`, which is not a Rust identifier"
            );
        }
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
