use super::{arrow, header, w};
use crate::model::{CType, Cfg, Model};
use crate::naming::field_name;

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
        out.push_str(&alias(
            &model.cfg(fp.protect.as_deref()),
            &fp.name,
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
        out.push_str(&alias(
            &model.cfg(c.protect.as_deref()),
            &format!("PFN_{}", c.name),
            &params,
            &model.return_ty(&c.return_ty),
        ));
    }
    out
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
        ir.commands.push(command(
            "vkCmdSetBlendConstants",
            vec![field("commandBuffer", "VkCommandBuffer"), blend],
        ));
        let out = emit(&Model::new(&ir));
        assert!(out.contains(
            "pub type PFN_vkCmdSetBlendConstants = Option<unsafe extern \"system\" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4])>;"
        ));
    }
}
