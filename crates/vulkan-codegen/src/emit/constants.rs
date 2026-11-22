use super::{header, w};
use crate::docs::Doc;
use crate::model::{Cfg, Model};
use crate::naming::const_name;

fn rust_ty(c_ty: &str, constant: &str) -> &'static str {
    match c_ty {
        "float" => "f32",
        "VkBool32" => "Bool32",
        "uint8_t" => "u8",
        "uint32_t" => "u32",
        "uint64_t" => "u64",
        other => panic!("unsupported Vulkan constant type {other} for {constant}"),
    }
}

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    let array_lens = model.array_len_constants();
    for c in &model.ir.constants {
        let ty = if array_lens.contains(c.name.as_str()) {
            "usize"
        } else {
            rust_ty(&c.ty, &c.name)
        };
        let from = Cfg::default();
        let origin = match c.video_std_header.as_deref() {
            Some(header) => format!(
                "C constant `{}`, provided by {}.",
                c.name,
                model.provenance(&from, None, &[], Some(header))
            ),
            None => format!("C constant {}.", model.reference(&c.name)),
        };
        let doc = Doc::new()
            .text(model.summary(&from, &c.name).unwrap_or_default())
            .text(origin)
            .alias(&c.name)
            .of_item(&const_name(&c.name));
        w!(
            out,
            "{}pub const {}: {ty} = {};",
            doc.render(),
            const_name(&c.name),
            c.value
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{field, registry, structure};
    use vulkan_registry::Constant;

    #[test]
    fn constants_used_as_array_lengths_are_usize() {
        let mut ir = registry();
        for (name, value) in [
            ("VK_UUID_SIZE", "16"),
            ("VK_ATTACHMENT_UNUSED", "4294967295"),
        ] {
            ir.constants.push(Constant {
                name: name.to_string(),
                ty: "uint32_t".to_string(),
                value: value.to_string(),
                ..Default::default()
            });
        }
        let mut uuid = field("pipelineCacheUUID", "uint8_t");
        uuid.fixed_size_array = vec!["VK_UUID_SIZE".to_string()];
        ir.structs.push(structure("VkProps", vec![uuid]));
        let out = emit(&Model::new(&ir));
        assert!(out.contains("pub const UUID_SIZE: usize = 16;"));
        assert!(out.contains("pub const ATTACHMENT_UNUSED: u32 = 4294967295;"));
    }
}
