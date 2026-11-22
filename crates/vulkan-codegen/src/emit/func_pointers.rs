use super::{arrow, header, w};
use crate::docs::{Block, Doc};
use crate::model::{CType, Cfg, Model};
use crate::naming::{field_name, func_pointer_name};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    out.push_str("use core::ffi::{c_char, c_void};\n\n");
    for fp in &model.ir.func_pointers {
        let params = fp
            .params
            .iter()
            .map(|p| {
                let ty = model.pointer_ty(&CType::parse(&p.full_ty, &[]), "'_");
                (field_name(&p.name), ty)
            })
            .collect::<Vec<_>>();
        let cfg = model.cfg(fp.protect.as_deref());
        let described = fp
            .params
            .iter()
            .filter_map(|p| model.described(&cfg, &fp.name, &p.name))
            .flat_map(parameter_item)
            .collect::<Vec<_>>();
        let mut doc = model.type_doc(&cfg, &fp.name, "C function pointer type", "");
        if !described.is_empty() {
            doc = doc.blocks([Block::Heading("Parameters"), Block::List(described)]);
        }
        out.push_str(&doc.render());
        out.push_str(&alias(
            &cfg,
            &func_pointer_name(&fp.name),
            &params,
            &model.return_ty(&fp.return_ty),
        ));
    }
    for c in &model.ir.commands {
        let params = c
            .params
            .iter()
            .map(|p| (field_name(&p.name), model.param_ty(p, "'_")))
            .collect::<Vec<_>>();
        let cfg = model.cfg(c.protect.as_deref());
        let doc = Doc::new().text(format!(
            "The type of a pointer to the C command {}, which {} calls.",
            model.reference(&c.name),
            model.link(&cfg, &c.name)
        ));
        out.push_str(&doc.render());
        out.push_str(&alias(
            &cfg,
            &func_pointer_name(&format!("PFN_{}", c.name)),
            &params,
            &model.return_ty(&c.return_ty),
        ));
    }
    out
}

pub(crate) fn parameter_item(described: Vec<Block>) -> Vec<(usize, String)> {
    described
        .into_iter()
        .flat_map(|block| match block {
            Block::Text(text) => vec![(0, text)],
            Block::List(items) => items
                .into_iter()
                .map(|(level, text)| (level + 1, text))
                .collect(),
            Block::Heading(_) => Vec::new(),
        })
        .collect()
}

fn alias(cfg: &Cfg, name: &str, params: &[(String, String)], return_ty: &str) -> String {
    let params = params
        .iter()
        .map(|(name, ty)| format!("{name}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ");
    let mut out = String::new();
    w!(
        out,
        "{cfg}pub type {name} = Option<unsafe extern \"system\" fn({params}){}>;\n",
        arrow(return_ty)
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{command, field, registry};
    use vulkan_registry::Handle;

    #[test]
    fn command_function_pointers_pass_arrays_by_pointer() {
        let mut ir = registry();
        ir.handles.push(Handle {
            name: "VkCommandBuffer".to_string(),
            dispatchable: true,
            ..Default::default()
        });
        let mut blend = field("blendConstants", "const float");
        blend.fixed_size_array = vec!["4".to_string()];
        let mut set_blend_constants = command(
            "vkCmdSetBlendConstants",
            vec![field("commandBuffer", "VkCommandBuffer"), blend],
        );
        set_blend_constants.device = true;
        ir.commands.push(set_blend_constants);
        let out = emit(&Model::new(&ir));
        assert!(out.contains(
            "pub type PFN_vkCmdSetBlendConstants = Option<unsafe extern \"system\" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4])>;"
        ));
    }
}
