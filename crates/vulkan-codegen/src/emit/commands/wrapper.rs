use super::inputs::Inputs;
use super::shape::Shape;
use super::{argument_name, raw_name, special};
use crate::emit::arrow;
use crate::model::{CType, Dispatch, Model, null_ptr};
use crate::naming::command_name;
use vulkan_registry::{Command, Field};

pub(super) fn wrapper(model: &Model<'_>, c: &Command, dispatch: Dispatch) -> String {
    let cfg = model.cfg(c.protect.as_deref());
    if let Some(wrapper) = special::wrapper(c) {
        return format!("{cfg}{wrapper}");
    }
    let params = dispatch.explicit_params(c);
    let shape = Shape::detect(model, c, params);
    let inputs = Inputs::plan(model, c, params, &shape.outputs());
    let call = Call {
        model,
        c,
        params,
        inputs: &inputs,
    };
    let Returned { ty, statements } = match &shape {
        Shape::PassThrough => call.passes_through(),
        Shape::ReturnsValue { out } => call.returns_value(*out),
        Shape::Enumerates { count, data } => call.enumerates(*count, *data),
        Shape::FillsOnePerInput { count, out } => {
            let len = inputs.slice_len_local(*count).unwrap_or_else(|| {
                panic!(
                    "{}: count of the output array is not derived from a slice",
                    c.name
                )
            });
            call.fills_vector(*out, len)
        }
        Shape::FillsStructMemberCount { info, member, out } => {
            let owner = argument_name(&params[*info]);
            call.fills_vector(*out, &format!("{owner}.{member} as usize"))
        }
    };
    format!(
        "{cfg}pub unsafe fn {name}(&self{signature}){arrow} {{
            unsafe {{
                {checks}{statements}}}
        }}\n",
        name = command_name(&c.name),
        signature = inputs.signature(),
        arrow = arrow(&ty),
        checks = inputs.checks,
    )
}

struct Returned {
    ty: String,
    statements: String,
}

impl Returned {
    fn new(ty: impl Into<String>, statements: impl Into<String>) -> Self {
        Self {
            ty: ty.into(),
            statements: statements.into() + "\n",
        }
    }
}

struct Call<'a> {
    model: &'a Model<'a>,
    c: &'a Command,
    params: &'a [Field],
    inputs: &'a Inputs,
}

impl Call<'_> {
    fn raw(&self, outputs: &[(usize, &str)]) -> String {
        let arguments = (0..self.params.len())
            .map(|param| {
                let output = outputs
                    .iter()
                    .find(|(index, _)| *index == param)
                    .map(|(_, pass)| *pass);
                output
                    .or_else(|| self.inputs.pass(param))
                    .unwrap_or_else(|| panic!("{}: parameter {param} is not passed", self.c.name))
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("self.{}({arguments})", raw_name(self.c))
    }

    fn returns_result(&self) -> bool {
        self.c.return_ty == "VkResult"
    }

    fn has_success_codes_besides_success(&self) -> bool {
        self.c.success_codes.iter().any(|code| code != "VK_SUCCESS")
    }

    fn reports_only_suboptimal(&self) -> bool {
        self.c.name == "vkQueuePresentKHR"
    }

    fn acquires_a_swapchain_image(&self) -> bool {
        matches!(
            self.c.name.as_str(),
            "vkAcquireNextImageKHR" | "vkAcquireNextImage2KHR"
        )
    }

    fn creates_pipelines(&self) -> bool {
        self.c.name.starts_with("vkCreate")
            && self.c.name.contains("Pipelines")
            && self.c.params.last().is_some_and(|p| p.ty == "VkPipeline")
    }

    fn passes_through(&self) -> Returned {
        let call = self.raw(&[]);
        if !self.returns_result() {
            return match self.model.return_ty(&self.c.return_ty).as_str() {
                "Bool32" => Returned::new("bool", format!("{call} != 0")),
                ty => Returned::new(ty, call),
            };
        }
        if self.reports_only_suboptimal() {
            return Returned::new(
                "crate::vk::Result<bool>",
                format!(
                    "let result = {call};
                    result.into_result()?;
                    Ok(result == VkResult::SUBOPTIMAL_KHR)"
                ),
            );
        }
        if self.has_success_codes_besides_success() {
            return Returned::new(
                "crate::vk::Result<VkResult>",
                format!(
                    "let result = {call};
                    result.into_result()?;
                    Ok(result)"
                ),
            );
        }
        Returned::new("crate::vk::Result<()>", format!("{call}.into_result()"))
    }

    fn returns_value(&self, out: usize) -> Returned {
        let p = &self.params[out];
        let (storage_ty, init) = self.output_element(p);
        let is_bool = self.model.canonical(&p.ty) == "VkBool32" && CType::of(p).depth() == 1;
        let (ty, value) = if is_bool {
            ("bool", "value != 0")
        } else {
            (storage_ty.as_str(), "value")
        };
        let declaration = format!("let mut value: {storage_ty} = {init};");
        let call = self.raw(&[(out, "&mut value")]);
        if !self.returns_result() {
            return Returned::new(
                ty,
                format!(
                    "{declaration}
                    {call};
                    {value}"
                ),
            );
        }
        if self.acquires_a_swapchain_image() {
            return Returned::new(
                format!("crate::vk::Result<Option<({ty}, bool)>>"),
                format!(
                    "{declaration}
                    match {call} {{
                        VkResult::SUCCESS => Ok(Some(({value}, false))),
                        VkResult::SUBOPTIMAL_KHR => Ok(Some(({value}, true))),
                        VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),
                        error => Err(error),
                    }}"
                ),
            );
        }
        if self.has_success_codes_besides_success() {
            return Returned::new(
                format!("crate::vk::Result<({ty}, VkResult)>"),
                format!(
                    "{declaration}
                    let result = {call};
                    result.into_result()?;
                    Ok(({value}, result))"
                ),
            );
        }
        Returned::new(
            format!("crate::vk::Result<{ty}>"),
            format!(
                "{declaration}
                {call}.into_result()?;
                Ok({value})"
            ),
        )
    }

    fn enumerates(&self, count: usize, data: usize) -> Returned {
        let p = &self.params[data];
        let (element, init, cast) = if p.ty == "void" {
            ("u8".to_string(), "0u8".to_string(), ".cast()")
        } else {
            let (element, init) = self.output_element(p);
            (element, init, "")
        };
        let count_ty = self.model.base_ty(&self.params[count].ty, "'_");
        let len = match count_ty.as_str() {
            "usize" => "count",
            _ => "count as usize",
        };
        let query = self.raw(&[(count, "&mut count"), (data, "core::ptr::null_mut()")]);
        let fill = self.raw(&[
            (count, "&mut count"),
            (data, &format!("values.as_mut_ptr(){cast}")),
        ]);
        if self.returns_result() {
            return Returned::new(
                format!("crate::vk::Result<Vec<{element}>>"),
                format!(
                    "loop {{
                        let mut count: {count_ty} = 0;
                        {query}.into_result()?;
                        let mut values: Vec<{element}> = alloc::vec![{init}; {len}];
                        let result = {fill};
                        if result == VkResult::INCOMPLETE {{ continue; }}
                        result.into_result()?;
                        values.truncate({len});
                        return Ok(values);
                    }}"
                ),
            );
        }
        Returned::new(
            format!("Vec<{element}>"),
            format!(
                "let mut count: {count_ty} = 0;
                {query};
                let mut values: Vec<{element}> = alloc::vec![{init}; {len}];
                {fill};
                values.truncate({len});
                values"
            ),
        )
    }

    fn fills_vector(&self, out: usize, len: &str) -> Returned {
        let (element, init) = self.output_element(&self.params[out]);
        let declaration = format!("let mut values: Vec<{element}> = alloc::vec![{init}; {len}];");
        let call = self.raw(&[(out, "values.as_mut_ptr()")]);
        if !self.returns_result() {
            return Returned::new(
                format!("Vec<{element}>"),
                format!(
                    "{declaration}
                    {call};
                    values"
                ),
            );
        }
        if self.creates_pipelines() {
            return Returned::new(
                format!("core::result::Result<Vec<{element}>, (Vec<{element}>, VkResult)>"),
                format!(
                    "{declaration}
                    let result = {call};
                    if result == VkResult::SUCCESS {{ Ok(values) }} else {{ Err((values, result)) }}"
                ),
            );
        }
        Returned::new(
            format!("crate::vk::Result<Vec<{element}>>"),
            format!(
                "{declaration}
                {call}.into_result()?;
                Ok(values)"
            ),
        )
    }

    fn output_element(&self, p: &Field) -> (String, String) {
        let ty = CType::of(p);
        if ty.depth() == 2 {
            let pointer = ty.pointee();
            return (
                self.model.pointer_ty(&pointer, "'static"),
                null_ptr(pointer.outer_const()).to_string(),
            );
        }
        let element = self.model.base_ty(&ty.base, "'static");
        let has_default =
            self.model.get_struct(&ty.base).is_some() || self.model.is_handle(&ty.base);
        let init = if has_default {
            "Default::default()".to_string()
        } else {
            format!("core::mem::zeroed::<{element}>()")
        };
        (element, init)
    }
}
