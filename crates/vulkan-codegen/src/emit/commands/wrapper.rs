use super::inputs::Inputs;
use super::shape::Shape;
use super::{
    argument_name, called, can_defer, not_loaded, provided, raw_name, with_deferred_safety,
};
use crate::docs::{Doc, join_and, join_or};
use crate::emit::arrow;
use crate::model::{CType, Cfg, Dispatch, Model, null_ptr};
use crate::naming::command_name;
use std::collections::BTreeSet;
use vulkan_registry::{Command, Field};

pub(super) fn wrapper(model: &Model<'_>, c: &Command, dispatch: Dispatch) -> String {
    let cfg = model.cfg(c.protect.as_deref());
    let params = dispatch.explicit_params(c);
    let shape = Shape::detect(model, c, params);
    let inputs = Inputs::plan(model, c, params, &shape.outputs());
    let call = Call {
        model,
        c,
        params,
        inputs: &inputs,
    };
    let Returned {
        ty,
        statements,
        note,
        errors,
    } = match &shape {
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
    let note = match deferred_outputs(model, c, params) {
        Some(outputs) => format!("{note} {outputs}"),
        None => note,
    };
    let note = if inputs.has_slices_of_slices {
        format!(
            "Each slice of slices is passed as an array of pointers to the inner slices. {note}"
        )
    } else {
        note
    };
    let doc = doc(
        model,
        &cfg,
        c,
        note.trim_end(),
        errors.or_else(|| ty.contains("Result<").then(|| error_codes(model, &cfg, c))),
        inputs.checks.contains("assert_eq!"),
    );
    format!(
        "{doc}{cfg}pub unsafe fn {name}(&self{signature}){arrow} {{
            unsafe {{
                {checks}{statements}}}
        }}\n",
        doc = doc.render(),
        name = command_name(&c.name),
        signature = inputs.signature(),
        arrow = arrow(&ty),
        checks = inputs.checks,
    )
}

fn doc(
    model: &Model<'_>,
    cfg: &Cfg,
    c: &Command,
    note: &str,
    errors: Option<String>,
    checks_lengths: bool,
) -> Doc {
    let raw = raw_name(c);
    let lengths = if checks_lengths {
        " Also panics if the slices do not have the lengths that the command needs: slices that it counts with one parameter have to be equally long, and a slice whose length it reads from another argument has to have that length."
    } else {
        ""
    };
    Doc::new()
        .text(model.summary(cfg, &c.name).unwrap_or_default())
        .text(format!(
            "Calls {} through [`Self::{raw}`], which describes the parameters. {note} {}",
            called(model, cfg, c),
            provided(model, cfg, c)
        ))
        .section("Errors", errors.unwrap_or_default())
        .section(
            "Safety",
            with_deferred_safety(model, c, format!("The same as for [`Self::{raw}`].")),
        )
        .section("Panics", format!("{}{lengths}", not_loaded(c)))
        .alias(&c.name)
}

fn error_codes(model: &Model<'_>, cfg: &Cfg, c: &Command) -> String {
    if c.error_codes.is_empty() {
        "Returns the result code of the command if it is an error.".to_string()
    } else {
        format!(
            "Returns the result code of the command if it is an error. The registry lists {} for it.",
            codes(model, cfg, &c.error_codes)
        )
    }
}

fn deferred_outputs(model: &Model<'_>, c: &Command, params: &[Field]) -> Option<String> {
    if !can_defer(model, c) {
        return None;
    }
    let outputs = params
        .iter()
        .filter(|p| CType::of(p).pointers == [false] && p.len.is_some() && model.is_handle(&p.ty))
        .map(|p| format!("`{}`", argument_name(p)))
        .collect::<Vec<_>>();
    if outputs.is_empty() {
        return None;
    }
    Some(format!(
        "What {} holds depends on the result: after `VK_SUCCESS`, the created handles; after `VK_OPERATION_DEFERRED_KHR`, nothing that may be read before the deferred operation is complete, and then what the result of the deferred operation says; after any other result, null handles for the elements that the command could not create, and valid handles, which have to be destroyed, for the others.",
        join_and(outputs)
    ))
}

fn codes(model: &Model<'_>, cfg: &Cfg, codes: &[String]) -> String {
    join_or(codes.iter().map(|code| model.link(cfg, code)).collect())
}

struct Returned {
    ty: String,
    statements: String,
    note: String,
    errors: Option<String>,
}

impl Returned {
    fn new(ty: impl Into<String>, statements: impl Into<String>) -> Self {
        Self {
            ty: ty.into(),
            statements: statements.into() + "\n",
            note: String::new(),
            errors: None,
        }
    }

    fn errors(mut self, errors: impl Into<String>) -> Self {
        self.errors = Some(errors.into());
        self
    }

    fn note(mut self, note: impl Into<String>) -> Self {
        self.note = note.into();
        self
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

    fn success_code(&self) -> String {
        format!(
            "the result code of the command when it is not an error, which the registry lists as {}",
            codes(
                self.model,
                &self.model.cfg(self.c.protect.as_deref()),
                &self.c.success_codes
            )
        )
    }

    fn has_success_codes_besides_success(&self) -> bool {
        self.c.success_codes.iter().any(|code| code != "VK_SUCCESS")
    }

    fn success_codes(&self) -> BTreeSet<&str> {
        self.c
            .success_codes
            .iter()
            .map(|code| self.model.result_code(code))
            .collect()
    }

    fn succeeds_with_exactly(&self, codes: &[&str]) -> bool {
        self.success_codes() == codes.iter().copied().collect()
    }

    fn reports_only_suboptimal(&self) -> bool {
        self.succeeds_with_exactly(&["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"])
    }

    fn may_not_write_the_value(&self) -> bool {
        self.succeeds_with_exactly(&[
            "VK_SUCCESS",
            "VK_TIMEOUT",
            "VK_NOT_READY",
            "VK_SUBOPTIMAL_KHR",
        ])
    }

    fn other_success_codes(&self) -> Vec<String> {
        self.c
            .success_codes
            .iter()
            .filter(|code| self.model.result_code(code) != "VK_SUCCESS")
            .cloned()
            .collect()
    }

    fn passes_through(&self) -> Returned {
        let call = self.raw(&[]);
        if !self.returns_result() {
            return match self.model.return_ty(&self.c.return_ty).as_str() {
                "Bool32" => Returned::new("bool", format!("{call} != 0"))
                    .note("Returns whether the command returned `VK_TRUE`."),
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
            )
            .note("Returns whether the command reported `VK_SUBOPTIMAL_KHR`.");
        }
        if self.has_success_codes_besides_success() {
            return Returned::new(
                "crate::vk::Result<VkResult>",
                format!(
                    "let result = {call};
                    result.into_result()?;
                    Ok(result)"
                ),
            )
            .note(format!("Returns {}.", self.success_code()));
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
        let written = format!("Returns what the command writes to `{}`", p.name);
        if !self.returns_result() {
            return Returned::new(
                ty,
                format!(
                    "{declaration}
                    {call};
                    {value}"
                ),
            )
            .note(format!("{written}."));
        }
        if self.may_not_write_the_value() {
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
            )
            .note(format!(
                "{written} and whether the command reported `VK_SUBOPTIMAL_KHR`, or `None` if the command returned `VK_TIMEOUT` or `VK_NOT_READY`."
            ));
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
            )
            .note(format!("{written}, and {}.", self.success_code()));
        }
        Returned::new(
            format!("crate::vk::Result<{ty}>"),
            format!(
                "{declaration}
                {call}.into_result()?;
                Ok({value})"
            ),
        )
        .note(format!("{written}."))
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
            )
            .note(format!(
                "Asks the command for the number of elements and then for the elements, again for as long as it reports `VK_INCOMPLETE`, and returns what it writes to `{}`.",
                p.name
            ));
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
        .note(format!(
            "Asks the command for the number of elements and then for the elements, and returns what it writes to `{}`.",
            p.name
        ))
    }

    fn fills_vector(&self, out: usize, len: &str) -> Returned {
        let (element, init) = self.output_element(&self.params[out]);
        let declaration = format!("let mut values: Vec<{element}> = alloc::vec![{init}; {len}];");
        let call = self.raw(&[(out, "values.as_mut_ptr()")]);
        let written = format!(
            "Returns the elements that the command writes to `{}`",
            self.params[out].name
        );
        if !self.returns_result() {
            return Returned::new(
                format!("Vec<{element}>"),
                format!(
                    "{declaration}
                    {call};
                    values"
                ),
            )
            .note(format!("{written}."));
        }
        if self.has_success_codes_besides_success() {
            let cfg = self.model.cfg(self.c.protect.as_deref());
            let errors = if self.c.error_codes.is_empty() {
                String::new()
            } else {
                format!(
                    ", which the registry lists as {}",
                    codes(self.model, &cfg, &self.c.error_codes)
                )
            };
            let handles = if self.model.is_handle(&self.params[out].ty) {
                " The elements that the command could not create are null handles; the others are valid and have to be destroyed."
            } else {
                ""
            };
            let errors = format!(
                "Returns the result code of the command, together with the elements, if it is not `VK_SUCCESS`: another success code, which the registry lists as {}, or an error code{errors}.{handles}",
                codes(self.model, &cfg, &self.other_success_codes())
            );
            return Returned::new(
                format!("core::result::Result<Vec<{element}>, (Vec<{element}>, VkResult)>"),
                format!(
                    "{declaration}
                    let result = {call};
                    if result == VkResult::SUCCESS {{ Ok(values) }} else {{ Err((values, result)) }}"
                ),
            )
            .note(format!(
                "{written}. Any result other than `VK_SUCCESS` is returned as the error, together with the same elements."
            ))
            .errors(errors);
        }
        Returned::new(
            format!("crate::vk::Result<Vec<{element}>>"),
            format!(
                "{declaration}
                {call}.into_result()?;
                Ok(values)"
            ),
        )
        .note(format!("{written}."))
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
