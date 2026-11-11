//! Function pointer tables and command wrappers.
//!
//! Every command gets two methods on `Entry`, `Instance` or `Device`:
//!
//! - `foo_raw`: the C signature, minus the handle the wrapper owns.
//! - `foo`: slices instead of pointer + count, references instead of pointers,
//!   `vk::Result` instead of `VkResult`, and returned values instead of out-pointers.
//!
//! The shape of `foo` is derived from registry attributes (`len`, `optional`, constness,
//! success codes). The few commands whose shape cannot be derived are listed explicitly in
//! this file.

use super::basic::arrow;
use super::{header, w};
use crate::model::{CType, Dispatch, Model, provided_by};
use crate::naming::{command_name, field_name, pointer_setter_name};
use std::collections::BTreeMap;
use vulkan_registry::{Command, Field};

/// Wrappers written by hand in `crates/vulkan/src/lib.rs` because they construct the
/// `Instance` / `Device` wrapper types. They still get a `_raw` method.
const HAND_WRITTEN: &[&str] = &["vkCreateInstance", "vkCreateDevice"];

pub fn emit_commands(model: &Model<'_>) -> String {
    let mut out = header(&["too_many_arguments", "missing_safety_doc"]);
    out.push_str("use alloc::vec::Vec;\nuse core::ffi::{c_char, c_void, CStr};\n\n");
    for dispatch in Dispatch::ALL {
        emit_table(&mut out, model, dispatch);
    }
    for dispatch in Dispatch::ALL {
        w!(out, "impl {} {{", dispatch.wrapper());
        for c in commands(model, dispatch) {
            emit_raw(&mut out, model, c, dispatch);
        }
        w!(out, "}}\n");
        w!(out, "impl {} {{", dispatch.wrapper());
        for c in commands(model, dispatch) {
            if !HAND_WRITTEN.contains(&c.name.as_str()) {
                emit_wrapper(&mut out, model, c, dispatch);
            }
        }
        w!(out, "}}\n");
    }
    out
}

fn commands<'a>(model: &'a Model<'_>, dispatch: Dispatch) -> impl Iterator<Item = &'a Command> {
    model
        .ir
        .commands
        .iter()
        .filter(move |c| model.dispatch(c) == dispatch)
}

fn emit_table(out: &mut String, model: &Model<'_>, dispatch: Dispatch) {
    let name = dispatch.table();
    w!(
        out,
        "/// Function pointers loaded for {}.",
        dispatch.wrapper()
    );
    w!(out, "///");
    w!(
        out,
        "/// A pointer is `None` when the loader does not provide the command, which is the"
    );
    w!(
        out,
        "/// normal state for commands of extensions or versions that were not enabled."
    );
    w!(out, "#[derive(Copy, Clone)]");
    w!(out, "pub struct {name} {{");
    for c in commands(model, dispatch) {
        w!(
            out,
            "    {}pub {}: PFN_{},",
            model.cfg(c.protect.as_deref()),
            command_name(&c.name),
            c.name
        );
    }
    w!(out, "}}\n");

    let (params, handle) = match dispatch {
        Dispatch::Entry => (
            "get_instance_proc_addr: PFN_vkGetInstanceProcAddr",
            "Instance::null()",
        ),
        Dispatch::Instance => (
            "get_instance_proc_addr: PFN_vkGetInstanceProcAddr, instance: Instance",
            "instance",
        ),
        Dispatch::Device => (
            "get_device_proc_addr: PFN_vkGetDeviceProcAddr, device: Device",
            "device",
        ),
    };
    let getter = if dispatch == Dispatch::Device {
        "get_device_proc_addr"
    } else {
        "get_instance_proc_addr"
    };
    w!(out, "impl {name} {{");
    w!(out, "    /// Loads every command of this level.");
    w!(out, "    ///");
    w!(out, "    /// # Safety");
    w!(out, "    ///");
    w!(
        out,
        "    /// `{getter}` must be a valid Vulkan loader function and the handle, if any, must"
    );
    w!(
        out,
        "    /// be a live handle created through the same loader."
    );
    w!(out, "    pub unsafe fn load({params}) -> Self {{");
    w!(
        out,
        "        let get = {getter}.expect(\"{getter} must not be null\");"
    );
    w!(out, "        unsafe {{");
    w!(out, "            Self {{");
    for c in commands(model, dispatch) {
        let field = command_name(&c.name);
        let cfg = model.cfg(c.protect.as_deref());
        if dispatch == Dispatch::Entry && c.name == "vkGetInstanceProcAddr" {
            w!(out, "                {field},");
        } else {
            w!(
                out,
                "                {cfg}{field}: crate::load_pfn(get({handle}, c\"{}\".as_ptr())),",
                c.name
            );
        }
    }
    w!(out, "            }}\n        }}\n    }}\n}}\n");
}

/// Parameters after the handle that the wrapper type passes itself.
fn explicit_params(c: &Command, dispatch: Dispatch) -> (bool, &[Field]) {
    let passes_self = dispatch
        .self_handle()
        .zip(c.params.first())
        .is_some_and(|(handle, first)| first.ty == handle && CType::of(first).depth() == 0);
    if passes_self {
        (true, &c.params[1..])
    } else {
        (false, &c.params[..])
    }
}

fn man_url(name: &str) -> String {
    format!("https://registry.khronos.org/vulkan/specs/latest/man/html/{name}.html")
}

fn emit_raw(out: &mut String, model: &Model<'_>, c: &Command, dispatch: Dispatch) {
    let (passes_self, params) = explicit_params(c, dispatch);
    let name = command_name(&c.name);
    let plain = name.trim_start_matches("r#");
    out.push_str(&model.cfg(c.protect.as_deref()));
    w!(out, "    /// Raw [`{}`]({}).", c.name, man_url(&c.name));
    w!(out, "    ///");
    w!(out, "    /// # Safety");
    w!(out, "    ///");
    w!(
        out,
        "    /// All valid usage rules of `{}` must be upheld. Panics if the command is not loaded.",
        c.name
    );
    let args = params
        .iter()
        .map(|p| format!(", {}: {}", field_name(&p.name), model.param_ty(p, "'_")))
        .collect::<String>();
    w!(
        out,
        "    pub unsafe fn {plain}_raw(&self{args}){} {{",
        arrow(&model.return_ty(&c.return_ty))
    );
    let mut call = Vec::new();
    if passes_self {
        call.push("self.handle()".to_string());
    }
    call.extend(params.iter().map(|p| field_name(&p.name)));
    w!(
        out,
        "        unsafe {{ (self.fp().{name}.expect(\"{} is not loaded\"))({}) }}",
        c.name,
        call.join(", ")
    );
    w!(out, "    }}");
}

/// How a C parameter appears in the high-level wrapper.
#[derive(Clone, Debug)]
enum Plan {
    /// A wrapper argument. `raw` is the expression passed to the `_raw` call.
    Arg {
        name: String,
        ty: String,
        raw: String,
        mutable: bool,
    },
    /// Not a wrapper argument; `raw` is computed from other arguments or locals.
    Hidden { raw: String },
}

/// Where the wrapper's return value comes from.
#[derive(Clone, Debug, PartialEq)]
enum Shape {
    /// Nothing is returned through pointers.
    Plain,
    /// The last parameter is a single out-pointer.
    Single { out: usize },
    /// `(uint32_t* pCount, T* pData)`: the two-call enumeration idiom.
    Enumerate { count: usize, data: usize },
    /// The last parameter is an output array as long as an input array.
    CountedOut { count: usize, out: usize },
    /// The last parameter is an output array whose length is a member of an input struct.
    StructLenOut {
        info: usize,
        member: String,
        out: usize,
    },
}

fn arg_name(p: &Field) -> String {
    if CType::of(p).depth() > 0 {
        pointer_setter_name(&p.name)
    } else {
        field_name(&p.name)
    }
}

/// `pAllocateInfo->commandBufferCount` to (`pAllocateInfo`, `commandBufferCount`).
fn struct_len(len: Option<&str>) -> Option<(&str, &str)> {
    let len = len?;
    len.split_once("->").or_else(|| len.split_once("::"))
}

/// Index of the by-value count parameter named by `p.len`.
fn value_count(params: &[Field], p: &Field) -> Option<usize> {
    let len = p.len.as_deref()?;
    params
        .iter()
        .position(|c| c.name == len && CType::of(c).depth() == 0 && c.fixed_size_array.is_empty())
}

fn is_opaque(model: &Model<'_>, ty: &CType) -> bool {
    model.is_opaque(&ty.base)
}

fn has_p_next(model: &Model<'_>, name: &str) -> bool {
    model.get_struct(name).is_some_and(|s| model.has_p_next(s))
}

fn detect_shape(model: &Model<'_>, c: &Command, params: &[Field]) -> Shape {
    if c.return_ty != "void" && c.return_ty != "VkResult" {
        return Shape::Plain;
    }
    let Some(out) = params.len().checked_sub(1) else {
        return Shape::Plain;
    };
    let last = &params[out];
    let ty = CType::of(last);
    if ty.depth() == 0 || ty.outer_const() || !ty.arrays.is_empty() {
        return Shape::Plain;
    }

    if ty.depth() == 1
        && out >= 1
        && let Some(len) = last.len.as_deref()
    {
        let count = &params[out - 1];
        let count_ty = CType::of(count);
        if count.name == len
            && count_ty.pointers == [false]
            && matches!(count_ty.base.as_str(), "uint32_t" | "size_t")
            && params
                .iter()
                .filter(|p| p.len.as_deref() == Some(len))
                .count()
                == 1
        {
            return Shape::Enumerate {
                count: out - 1,
                data: out,
            };
        }
    }

    if ty.depth() == 1 && ty.base != "void" {
        if let Some(count) = value_count(params, last)
            && params.iter().enumerate().any(|(i, p)| {
                i != out && CType::of(p).pointers == [true] && value_count(params, p) == Some(count)
            })
        {
            return Shape::CountedOut { count, out };
        }
        if let Some((owner, member)) = struct_len(last.len.as_deref())
            && let Some(info) = params.iter().position(|p| p.name == owner)
        {
            return Shape::StructLenOut {
                info,
                member: field_name(member),
                out,
            };
        }
    }

    let single = last.len.is_none()
        && match ty.depth() {
            1 => !is_opaque(model, &ty) && !has_p_next(model, &ty.base),
            // `void**`, `AHardwareBuffer**`: the pointer value itself is the output.
            2 => true,
            _ => false,
        };
    if single {
        Shape::Single { out }
    } else {
        Shape::Plain
    }
}

/// Plans every parameter that is not consumed by the output shape.
fn plan_inputs(
    model: &Model<'_>,
    c: &Command,
    params: &[Field],
    consumed: &[usize],
    prelude: &mut String,
) -> Vec<Option<Plan>> {
    // By-value counts and the slice arguments that determine them.
    let mut groups = BTreeMap::<usize, Vec<usize>>::new();
    for (i, p) in params.iter().enumerate() {
        let ty = CType::of(p);
        if consumed.contains(&i) || ty.depth() != 1 || !ty.arrays.is_empty() {
            continue;
        }
        if let Some(count) = value_count(params, p)
            && !consumed.contains(&count)
        {
            groups.entry(count).or_default().push(i);
        }
    }

    let mut plans = vec![None; params.len()];
    for (i, p) in params.iter().enumerate() {
        if consumed.contains(&i) {
            continue;
        }
        let ty = CType::of(p);
        let name = arg_name(p);
        let arg = |ty: String, raw: String, mutable: bool| {
            Some(Plan::Arg {
                name: name.clone(),
                ty,
                raw,
                mutable,
            })
        };

        if let Some(slices) = groups.get(&i) {
            // The count is the length of the first required slice, or of the first
            // provided optional one.
            let len_of = |index: usize| {
                let slice = &params[index];
                let slice_name = arg_name(slice);
                if slice.optional {
                    format!("{slice_name}.as_ref().map(|x| x.len())")
                } else {
                    format!("Some({slice_name}.len())")
                }
            };
            let primary = slices
                .iter()
                .copied()
                .find(|&s| !params[s].optional)
                .unwrap_or(slices[0]);
            let count_ty = model.base_ty(&p.ty, "'_");
            let as_count = if count_ty == "usize" {
                String::new()
            } else {
                format!(" as {count_ty}")
            };
            if params[primary].optional {
                let all = slices.iter().map(|&s| len_of(s)).collect::<Vec<_>>();
                w!(
                    prelude,
                    "let {name}_len: usize = [{}].into_iter().flatten().next().unwrap_or(0);",
                    all.join(", ")
                );
            } else {
                w!(
                    prelude,
                    "let {name}_len: usize = {}.len();",
                    arg_name(&params[primary])
                );
            }
            for &s in slices.iter().filter(|&&s| s != primary) {
                let other = arg_name(&params[s]);
                let message = format!(
                    "{}: `{other}` must have the same length as the other arrays sharing `{}`",
                    c.name, p.name
                );
                if params[s].optional {
                    w!(
                        prelude,
                        "if let Some(x) = {other}.as_ref() {{ assert_eq!(x.len(), {name}_len, \"{message}\"); }}"
                    );
                } else {
                    w!(
                        prelude,
                        "assert_eq!({other}.len(), {name}_len, \"{message}\");"
                    );
                }
            }
            plans[i] = Some(Plan::Hidden {
                raw: format!("{name}_len{as_count}"),
            });
            continue;
        }

        if !ty.arrays.is_empty() {
            assert_eq!(ty.depth(), 0, "{}: pointer array parameter", c.name);
            let array = model.member_ty(p, "'_");
            plans[i] = if CType::base_const(&p.full_ty) {
                arg(format!("&{array}"), name.clone(), false)
            } else {
                arg(format!("&mut {array}"), name.clone(), false)
            };
            continue;
        }

        if ty.depth() == 0 {
            plans[i] = if model.canonical(&p.ty) == "VkBool32" {
                arg("bool".to_string(), format!("{name} as Bool32"), false)
            } else {
                arg(model.base_ty(&p.ty, "'_"), name.clone(), false)
            };
            continue;
        }

        let opaque = is_opaque(model, &ty);
        let is_const = ty.outer_const();
        let null = if is_const {
            "core::ptr::null()"
        } else {
            "core::ptr::null_mut()"
        };

        if ty.depth() == 1 {
            let counted = value_count(params, p).is_some_and(|count| groups.contains_key(&count));
            let struct_counted = struct_len(p.len.as_deref())
                .and_then(|(owner, member)| {
                    params
                        .iter()
                        .find(|q| q.name == owner)
                        .map(|q| (arg_name(q), field_name(member)))
                })
                .filter(|_| !opaque);
            let complex_len = p.len.is_some() && !opaque;
            if counted || struct_counted.is_some() || complex_len {
                let (elem, cast) = if ty.base == "void" {
                    ("u8".to_string(), ".cast()")
                } else {
                    (model.base_ty(&ty.base, "'_"), "")
                };
                if let Some((owner, member)) = &struct_counted {
                    let message =
                        format!("{}: `{name}` must have `{owner}.{member}` elements", c.name);
                    if p.optional {
                        w!(
                            prelude,
                            "if let Some(x) = {name}.as_ref() {{ assert_eq!(x.len(), {owner}.{member} as usize, \"{message}\"); }}"
                        );
                    } else {
                        w!(
                            prelude,
                            "assert_eq!({name}.len(), {owner}.{member} as usize, \"{message}\");"
                        );
                    }
                }
                plans[i] = match (is_const, p.optional) {
                    (true, false) => arg(
                        format!("&[{elem}]"),
                        format!("{name}.as_ptr(){cast}"),
                        false,
                    ),
                    (false, false) => arg(
                        format!("&mut [{elem}]"),
                        format!("{name}.as_mut_ptr(){cast}"),
                        false,
                    ),
                    (true, true) => arg(
                        format!("Option<&[{elem}]>"),
                        format!("{name}.map_or({null}, |x| x.as_ptr(){cast})"),
                        false,
                    ),
                    (false, true) => arg(
                        format!("Option<&mut [{elem}]>"),
                        format!("{name}.as_mut().map_or({null}, |x| x.as_mut_ptr(){cast})"),
                        true,
                    ),
                };
            } else if p.null_terminated && ty.base == "char" && is_const {
                plans[i] = if p.optional {
                    arg(
                        "Option<&CStr>".to_string(),
                        format!("{name}.map_or({null}, |x| x.as_ptr())"),
                        false,
                    )
                } else {
                    arg("&CStr".to_string(), format!("{name}.as_ptr()"), false)
                };
            } else if opaque {
                plans[i] = arg(model.param_ty(p, "'_"), name.clone(), false);
            } else {
                let elem = model.base_ty(&ty.base, "'_");
                plans[i] = match (is_const, p.optional) {
                    (true, false) => arg(
                        format!("&{elem}"),
                        format!("core::ptr::from_ref({name})"),
                        false,
                    ),
                    (false, false) => arg(
                        format!("&mut {elem}"),
                        format!("core::ptr::from_mut(&mut *{name})"),
                        false,
                    ),
                    (true, true) => arg(
                        format!("Option<&{elem}>"),
                        format!("{name}.map_or({null}, core::ptr::from_ref)"),
                        false,
                    ),
                    (false, true) => arg(
                        format!("Option<&mut {elem}>"),
                        format!(
                            "{name}.as_mut().map_or({null}, |x| core::ptr::from_mut(&mut **x))"
                        ),
                        true,
                    ),
                };
            }
            continue;
        }

        // Deeper pointer shapes have no borrowed form.
        plans[i] = arg(model.param_ty(p, "'_"), name.clone(), false);
    }
    plans
}

fn doc(out: &mut String, c: &Command) {
    w!(out, "    /// [`{}`]({})", c.name, man_url(&c.name));
    let provided = provided_by(&c.extensions, c.version.as_deref());
    if !provided.is_empty() {
        w!(out, "    ///\n    /// {provided}");
    }
    w!(out, "    ///");
    w!(out, "    /// # Safety");
    w!(out, "    ///");
    w!(
        out,
        "    /// All valid usage rules of `{}` must be upheld. Panics if the command is not loaded.",
        c.name
    );
}

fn signature(name: &str, plans: &[Option<Plan>], ret: &str) -> String {
    let mut out = format!("    pub unsafe fn {name}(&self");
    for plan in plans.iter().flatten() {
        if let Plan::Arg {
            name, ty, mutable, ..
        } = plan
        {
            let mutable = if *mutable { "mut " } else { "" };
            out.push_str(&format!(", {mutable}{name}: {ty}"));
        }
    }
    out.push_str(&format!("){} {{", arrow(ret)));
    out
}

/// The `_raw` call expression with `extra` filling the parameters consumed by the shape.
fn raw_call(c: &Command, plans: &[Option<Plan>], extra: &[(usize, &str)]) -> String {
    let name = command_name(&c.name);
    let args = plans
        .iter()
        .enumerate()
        .map(|(i, plan)| {
            if let Some((_, raw)) = extra.iter().find(|(index, _)| *index == i) {
                return raw.to_string();
            }
            match plan {
                Some(Plan::Arg { raw, .. }) | Some(Plan::Hidden { raw }) => raw.clone(),
                None => panic!("{}: parameter {i} has no plan", c.name),
            }
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("self.{}_raw({args})", name.trim_start_matches("r#"))
}

fn extra_success_codes(c: &Command) -> bool {
    c.success_codes.iter().any(|code| code != "VK_SUCCESS")
}

/// Type of a value written through an out-pointer, and how to initialize its storage.
fn out_ty(model: &Model<'_>, p: &Field) -> (String, String) {
    let ty = CType::of(p);
    if ty.depth() == 2 {
        let inner = CType {
            base: ty.base.clone(),
            pointers: ty.pointers[..1].to_vec(),
            arrays: Vec::new(),
        };
        let init = if inner.outer_const() {
            "core::ptr::null()"
        } else {
            "core::ptr::null_mut()"
        };
        return (model.pointer_ty(&inner, "'static"), init.to_string());
    }
    let rust = model.base_ty(&ty.base, "'static");
    let init = if model.get_struct(&ty.base).is_some() || model.is_handle(&ty.base) {
        // Sets `sType` for structs that have one.
        "Default::default()".to_string()
    } else {
        // Integers, enums and flags: all-zero is a valid value. The wrapper body is one
        // `unsafe` block.
        format!("core::mem::zeroed::<{rust}>()")
    };
    (rust, init)
}

fn emit_wrapper(out: &mut String, model: &Model<'_>, c: &Command, dispatch: Dispatch) {
    let (_, params) = explicit_params(c, dispatch);
    let name = command_name(&c.name);
    out.push_str(&model.cfg(c.protect.as_deref()));
    doc(out, c);
    if let Some(body) = special_wrapper(c) {
        out.push_str(&body);
        return;
    }

    let shape = detect_shape(model, c, params);
    let consumed = match &shape {
        Shape::Plain => vec![],
        Shape::Single { out } => vec![*out],
        Shape::Enumerate { count, data } => vec![*count, *data],
        Shape::CountedOut { out, .. } | Shape::StructLenOut { out, .. } => vec![*out],
    };
    let mut prelude = String::new();
    let plans = plan_inputs(model, c, params, &consumed, &mut prelude);
    let is_result = c.return_ty == "VkResult";
    let mut body = prelude;

    let ret = match &shape {
        Shape::Plain => {
            let call = raw_call(c, &plans, &[]);
            if !is_result {
                let ret = model.return_ty(&c.return_ty);
                if ret == "Bool32" {
                    w!(body, "{call} != 0");
                    "bool".to_string()
                } else {
                    w!(body, "{call}");
                    ret
                }
            } else if c.name == "vkQueuePresentKHR" {
                // `Ok(true)` means VK_SUBOPTIMAL_KHR.
                w!(body, "let result = {call};");
                w!(body, "result.into_result()?;");
                w!(body, "Ok(result == VkResult::SUBOPTIMAL_KHR)");
                "crate::vk::Result<bool>".to_string()
            } else if extra_success_codes(c) {
                w!(body, "let result = {call};");
                w!(body, "result.into_result()?;");
                w!(body, "Ok(result)");
                "crate::vk::Result<VkResult>".to_string()
            } else {
                w!(body, "{call}.into_result()");
                "crate::vk::Result<()>".to_string()
            }
        }
        Shape::Single { out } => {
            let p = &params[*out];
            let (ty, init) = out_ty(model, p);
            let is_bool = model.canonical(&p.ty) == "VkBool32" && CType::of(p).depth() == 1;
            let (ret_ty, value) = if is_bool {
                ("bool".to_string(), "value != 0")
            } else {
                (ty.clone(), "value")
            };
            w!(body, "let mut value: {ty} = {init};");
            let call = raw_call(c, &plans, &[(*out, "&mut value")]);
            if !is_result {
                w!(body, "{call};");
                w!(body, "{value}");
                ret_ty
            } else if matches!(
                c.name.as_str(),
                "vkAcquireNextImageKHR" | "vkAcquireNextImage2KHR"
            ) {
                // The image index is only written for VK_SUCCESS and VK_SUBOPTIMAL_KHR.
                w!(body, "match {call} {{");
                w!(body, "    VkResult::SUCCESS => Ok(Some(({value}, false))),");
                w!(
                    body,
                    "    VkResult::SUBOPTIMAL_KHR => Ok(Some(({value}, true))),"
                );
                w!(
                    body,
                    "    VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),"
                );
                w!(body, "    error => Err(error),");
                w!(body, "}}");
                format!("crate::vk::Result<Option<({ret_ty}, bool)>>")
            } else if extra_success_codes(c) {
                w!(body, "let result = {call};");
                w!(body, "result.into_result()?;");
                w!(body, "Ok(({value}, result))");
                format!("crate::vk::Result<({ret_ty}, VkResult)>")
            } else {
                w!(body, "{call}.into_result()?;");
                w!(body, "Ok({value})");
                format!("crate::vk::Result<{ret_ty}>")
            }
        }
        Shape::Enumerate { count, data } => {
            let p = &params[*data];
            let (elem, init, cast) = if p.ty == "void" {
                ("u8".to_string(), "0u8".to_string(), ".cast()")
            } else {
                let (ty, init) = out_ty(model, p);
                (ty, init, "")
            };
            let count_ty = model.base_ty(&params[*count].ty, "'_");
            let len = if count_ty == "usize" {
                "count"
            } else {
                "count as usize"
            };
            let query = raw_call(
                c,
                &plans,
                &[(*count, "&mut count"), (*data, "core::ptr::null_mut()")],
            );
            let fill_arg = format!("values.as_mut_ptr(){cast}");
            let fill = raw_call(c, &plans, &[(*count, "&mut count"), (*data, &fill_arg)]);
            if is_result {
                // The set may change between the two calls, so retry on VK_INCOMPLETE.
                w!(body, "loop {{");
                w!(body, "    let mut count: {count_ty} = 0;");
                w!(body, "    {query}.into_result()?;");
                w!(
                    body,
                    "    let mut values: Vec<{elem}> = alloc::vec![{init}; {len}];"
                );
                w!(body, "    let result = {fill};");
                w!(
                    body,
                    "    if result == VkResult::INCOMPLETE {{ continue; }}"
                );
                w!(body, "    result.into_result()?;");
                w!(body, "    values.truncate({len});");
                w!(body, "    return Ok(values);");
                w!(body, "}}");
                format!("crate::vk::Result<Vec<{elem}>>")
            } else {
                w!(body, "let mut count: {count_ty} = 0;");
                w!(body, "{query};");
                w!(
                    body,
                    "let mut values: Vec<{elem}> = alloc::vec![{init}; {len}];"
                );
                w!(body, "{fill};");
                w!(body, "values.truncate({len});");
                w!(body, "values");
                format!("Vec<{elem}>")
            }
        }
        Shape::CountedOut { count, out } => {
            let (elem, init) = out_ty(model, &params[*out]);
            let Some(Plan::Hidden { raw: len }) = &plans[*count] else {
                panic!(
                    "{}: count of the output array is not derived from a slice",
                    c.name
                );
            };
            let len = len.split(" as ").next().expect("length local");
            w!(
                body,
                "let mut values: Vec<{elem}> = alloc::vec![{init}; {len}];"
            );
            let call = raw_call(c, &plans, &[(*out, "values.as_mut_ptr()")]);
            if !is_result {
                w!(body, "{call};");
                w!(body, "values");
                format!("Vec<{elem}>")
            } else if is_pipeline_creation(c) {
                // Pipeline creation can fail, or return VK_PIPELINE_COMPILE_REQUIRED, after
                // writing some handles. Hand them back so the caller can destroy them.
                w!(body, "let result = {call};");
                w!(
                    body,
                    "if result == VkResult::SUCCESS {{ Ok(values) }} else {{ Err((values, result)) }}"
                );
                format!("core::result::Result<Vec<{elem}>, (Vec<{elem}>, VkResult)>")
            } else {
                w!(body, "{call}.into_result()?;");
                w!(body, "Ok(values)");
                format!("crate::vk::Result<Vec<{elem}>>")
            }
        }
        Shape::StructLenOut { info, member, out } => {
            let (elem, init) = out_ty(model, &params[*out]);
            let owner = arg_name(&params[*info]);
            w!(
                body,
                "let mut values: Vec<{elem}> = alloc::vec![{init}; {owner}.{member} as usize];"
            );
            let call = raw_call(c, &plans, &[(*out, "values.as_mut_ptr()")]);
            if is_result {
                w!(body, "{call}.into_result()?;");
                w!(body, "Ok(values)");
                format!("crate::vk::Result<Vec<{elem}>>")
            } else {
                w!(body, "{call};");
                w!(body, "values");
                format!("Vec<{elem}>")
            }
        }
    };

    w!(out, "{}", signature(&name, &plans, &ret));
    w!(out, "        unsafe {{\n{body}        }}\n    }}");
}

fn is_pipeline_creation(c: &Command) -> bool {
    c.name.starts_with("vkCreate")
        && c.name.contains("Pipelines")
        && c.params.last().is_some_and(|p| p.ty == "VkPipeline")
}

/// Commands whose shape the registry attributes cannot express.
///
/// Acceleration structure builds take `ppBuildRangeInfos`: one pointer per build info,
/// each to `geometryCount` elements. A slice of slices keeps both lengths checked.
fn special_wrapper(c: &Command) -> Option<String> {
    let checks = |nested: &str| {
        format!(
            "        assert_eq!(infos.len(), {nested}.len(), \"{name}: `{nested}` must have one entry per info\");\n\
             \x20       for (info, entry) in infos.iter().zip({nested}.iter()) {{\n\
             \x20           assert_eq!(entry.len(), info.geometry_count as usize, \"{name}: each `{nested}` entry must have `geometry_count` elements\");\n\
             \x20       }}\n\
             \x20       let pointers = {nested}.iter().map(|entry| entry.as_ptr()).collect::<Vec<_>>();\n",
            name = c.name
        )
    };
    let body = match c.name.as_str() {
        "vkBuildAccelerationStructuresKHR" => format!(
            "    pub unsafe fn build_acceleration_structures_khr(&self, deferred_operation: DeferredOperationKHR, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR]]) -> crate::vk::Result<VkResult> {{\n\
             {}\
             \x20       let result = unsafe {{ self.build_acceleration_structures_khr_raw(deferred_operation, infos.len() as u32, infos.as_ptr(), pointers.as_ptr()) }};\n\
             \x20       result.into_result()?;\n\
             \x20       Ok(result)\n\
             \x20   }}\n",
            checks("build_range_infos")
        ),
        "vkCmdBuildAccelerationStructuresKHR" => format!(
            "    pub unsafe fn cmd_build_acceleration_structures_khr(&self, command_buffer: CommandBuffer, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], build_range_infos: &[&[AccelerationStructureBuildRangeInfoKHR]]) {{\n\
             {}\
             \x20       unsafe {{ self.cmd_build_acceleration_structures_khr_raw(command_buffer, infos.len() as u32, infos.as_ptr(), pointers.as_ptr()) }}\n\
             \x20   }}\n",
            checks("build_range_infos")
        ),
        "vkCmdBuildAccelerationStructuresIndirectKHR" => format!(
            "    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(&self, command_buffer: CommandBuffer, infos: &[AccelerationStructureBuildGeometryInfoKHR<'_>], indirect_device_addresses: &[DeviceAddress], indirect_strides: &[u32], max_primitive_counts: &[&[u32]]) {{\n\
             \x20       assert_eq!(infos.len(), indirect_device_addresses.len(), \"{name}: `indirect_device_addresses` must have one entry per info\");\n\
             \x20       assert_eq!(infos.len(), indirect_strides.len(), \"{name}: `indirect_strides` must have one entry per info\");\n\
             {}\
             \x20       unsafe {{ self.cmd_build_acceleration_structures_indirect_khr_raw(command_buffer, infos.len() as u32, infos.as_ptr(), indirect_device_addresses.as_ptr(), indirect_strides.as_ptr(), pointers.as_ptr()) }}\n\
             \x20   }}\n",
            checks("max_primitive_counts"),
            name = c.name
        ),
        _ => return None,
    };
    Some(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{
        command, counted, field, optional, registry, result_command, tagged,
    };
    use vulkan_registry::{Handle, Registry};

    fn squash(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn base() -> Registry {
        let mut ir = registry();
        for (name, dispatchable) in [
            ("VkInstance", true),
            ("VkPhysicalDevice", true),
            ("VkDevice", true),
            ("VkCommandBuffer", true),
            ("VkQueue", true),
            ("VkBuffer", false),
            ("VkFence", false),
            ("VkSemaphore", false),
            ("VkSwapchainKHR", false),
            ("VkPipeline", false),
            ("VkPipelineCache", false),
            ("VkSurfaceKHR", false),
        ] {
            ir.handles.push(Handle {
                name: name.to_string(),
                object_type: format!("VK_OBJECT_TYPE_{}", name.to_uppercase()),
                dispatchable,
                ..Default::default()
            });
        }
        ir
    }

    fn device(mut c: Command) -> Command {
        c.device = true;
        c
    }

    fn generate(ir: &Registry) -> String {
        squash(&emit_commands(&Model::new(ir)))
    }

    #[test]
    fn tables_follow_dispatch_level_and_use_c_string_literals() {
        let mut ir = base();
        let mut global = result_command(
            "vkEnumerateInstanceVersion",
            &["VK_SUCCESS"],
            vec![field("pApiVersion", "uint32_t*")],
        );
        global.instance = true;
        ir.commands.push(global);
        ir.commands.push(device(command(
            "vkDeviceWaitIdle",
            vec![field("device", "VkDevice")],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub struct EntryFn { pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion, }"
        ));
        assert!(
            out.contains("pub struct DeviceFn { pub device_wait_idle: PFN_vkDeviceWaitIdle, }")
        );
        assert!(out.contains(
            "device_wait_idle: crate::load_pfn(get(device, c\"vkDeviceWaitIdle\".as_ptr())),"
        ));
        assert!(out.contains(
            "pub unsafe fn enumerate_instance_version(&self) -> crate::vk::Result<u32> {"
        ));
    }

    #[test]
    fn slices_replace_pointer_and_count_and_shared_counts_are_checked() {
        let mut ir = base();
        let mut buffers = counted("pBuffers", "const VkBuffer*", "bindingCount");
        buffers.optional_pointer = true;
        ir.commands.push(device(command(
            "vkCmdBindVertexBuffers2",
            vec![
                field("commandBuffer", "VkCommandBuffer"),
                field("firstBinding", "uint32_t"),
                field("bindingCount", "uint32_t"),
                buffers,
                counted("pOffsets", "const VkDeviceSize*", "bindingCount"),
                optional(counted("pSizes", "const VkDeviceSize*", "bindingCount")),
            ],
        )));
        let out = generate(&ir);
        // `optional_pointer` (null handles allowed inside the array) does not make the
        // array itself optional.
        assert!(out.contains(
            "pub unsafe fn cmd_bind_vertex_buffers2(&self, command_buffer: CommandBuffer, first_binding: u32, buffers: &[Buffer], offsets: &[DeviceSize], sizes: Option<&[DeviceSize]>) {"
        ));
        assert!(out.contains("let binding_count_len: usize = buffers.len();"));
        assert!(out.contains("assert_eq!(offsets.len(), binding_count_len,"));
        assert!(
            out.contains(
                "if let Some(x) = sizes.as_ref() { assert_eq!(x.len(), binding_count_len,"
            )
        );
        assert!(out.contains(
            "self.cmd_bind_vertex_buffers2_raw(command_buffer, first_binding, binding_count_len as u32, buffers.as_ptr(), offsets.as_ptr(), sizes.map_or(core::ptr::null(), |x| x.as_ptr()))"
        ));
    }

    #[test]
    fn enumerations_retry_on_incomplete() {
        let mut ir = base();
        let mut count = field("pSwapchainImageCount", "uint32_t*");
        count.optional_pointer = true;
        ir.commands.push(device(result_command(
            "vkGetSwapchainImagesKHR",
            &["VK_SUCCESS", "VK_INCOMPLETE"],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                count,
                optional(counted(
                    "pSwapchainImages",
                    "VkBuffer*",
                    "pSwapchainImageCount",
                )),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_swapchain_images_khr(&self, swapchain: SwapchainKHR) -> crate::vk::Result<Vec<Buffer>> {"
        ));
        assert!(out.contains("if result == VkResult::INCOMPLETE { continue; }"));
        assert!(out.contains(
            "self.get_swapchain_images_khr_raw(swapchain, &mut count, core::ptr::null_mut()).into_result()?;"
        ));
    }

    #[test]
    fn acquire_next_image_never_returns_an_unwritten_index() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkAcquireNextImageKHR",
            &[
                "VK_SUCCESS",
                "VK_TIMEOUT",
                "VK_NOT_READY",
                "VK_SUBOPTIMAL_KHR",
            ],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                field("timeout", "uint64_t"),
                optional(field("semaphore", "VkSemaphore")),
                optional(field("fence", "VkFence")),
                field("pImageIndex", "uint32_t*"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains("-> crate::vk::Result<Option<(u32, bool)>> {"));
        assert!(out.contains("VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),"));
    }

    #[test]
    fn extra_success_codes_are_returned_to_the_caller() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkWaitForFences",
            &["VK_SUCCESS", "VK_TIMEOUT"],
            vec![
                field("device", "VkDevice"),
                field("fenceCount", "uint32_t"),
                counted("pFences", "const VkFence*", "fenceCount"),
                field("waitAll", "VkBool32"),
                field("timeout", "uint64_t"),
            ],
        )));
        ir.commands.push(device(result_command(
            "vkQueuePresentKHR",
            &["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"],
            vec![field("queue", "VkQueue")],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn wait_for_fences(&self, fences: &[Fence], wait_all: bool, timeout: u64) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("wait_all as Bool32"));
        assert!(out.contains(
            "pub unsafe fn queue_present_khr(&self, queue: Queue) -> crate::vk::Result<bool> {"
        ));
    }

    #[test]
    fn pipeline_creation_keeps_partial_output_on_failure() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkGraphicsPipelineCreateInfo",
            "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO",
            vec![],
        ));
        ir.commands.push(device(result_command(
            "vkCreateGraphicsPipelines",
            &["VK_SUCCESS", "VK_PIPELINE_COMPILE_REQUIRED_EXT"],
            vec![
                field("device", "VkDevice"),
                optional(field("pipelineCache", "VkPipelineCache")),
                field("createInfoCount", "uint32_t"),
                counted(
                    "pCreateInfos",
                    "const VkGraphicsPipelineCreateInfo*",
                    "createInfoCount",
                ),
                counted("pPipelines", "VkPipeline*", "createInfoCount"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn create_graphics_pipelines(&self, pipeline_cache: PipelineCache, create_infos: &[GraphicsPipelineCreateInfo<'_>]) -> core::result::Result<Vec<Pipeline>, (Vec<Pipeline>, VkResult)> {"
        ));
        assert!(out.contains("alloc::vec![Default::default(); create_info_count_len];"));
        assert!(out.contains("Err((values, result))"));
    }

    #[test]
    fn structs_with_p_next_are_filled_in_place() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkPhysicalDeviceFeatures2",
            "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2",
            vec![],
        ));
        let mut c = command(
            "vkGetPhysicalDeviceFeatures2",
            vec![
                field("physicalDevice", "VkPhysicalDevice"),
                field("pFeatures", "VkPhysicalDeviceFeatures2*"),
            ],
        );
        c.instance = true;
        ir.commands.push(c);
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_physical_device_features2(&self, physical_device: PhysicalDevice, features: &mut PhysicalDeviceFeatures2<'_>) {"
        ));
    }

    #[test]
    fn opaque_platform_pointers_are_never_references() {
        let mut ir = base();
        let mut c = command(
            "vkGetPhysicalDeviceWaylandPresentationSupportKHR",
            vec![
                field("physicalDevice", "VkPhysicalDevice"),
                field("queueFamilyIndex", "uint32_t"),
                field("display", "struct wl_display*"),
            ],
        );
        c.return_ty = "VkBool32".to_string();
        c.instance = true;
        ir.commands.push(c);
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_physical_device_wayland_presentation_support_khr(&self, physical_device: PhysicalDevice, queue_family_index: u32, display: *mut wl_display) -> bool {"
        ));
    }

    #[test]
    fn array_parameters_are_borrowed_arrays() {
        let mut ir = base();
        let mut blend = field("blendConstants", "const float");
        blend.fixed_size_array = vec!["4".to_string()];
        ir.commands.push(device(command(
            "vkCmdSetBlendConstants",
            vec![field("commandBuffer", "VkCommandBuffer"), blend],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn cmd_set_blend_constants_raw(&self, command_buffer: CommandBuffer, blend_constants: *const [f32; 4]) {"
        ));
        assert!(out.contains(
            "pub unsafe fn cmd_set_blend_constants(&self, command_buffer: CommandBuffer, blend_constants: &[f32; 4]) {"
        ));
    }

    #[test]
    fn output_arrays_sized_by_a_struct_member_become_vectors() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkCommandBufferAllocateInfo",
            "VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO",
            vec![field("commandBufferCount", "uint32_t")],
        ));
        ir.commands.push(device(result_command(
            "vkAllocateCommandBuffers",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("pAllocateInfo", "const VkCommandBufferAllocateInfo*"),
                counted(
                    "pCommandBuffers",
                    "VkCommandBuffer*",
                    "pAllocateInfo->commandBufferCount",
                ),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn allocate_command_buffers(&self, allocate_info: &CommandBufferAllocateInfo<'_>) -> crate::vk::Result<Vec<CommandBuffer>> {"
        ));
        assert!(out.contains(
            "alloc::vec![Default::default(); allocate_info.command_buffer_count as usize];"
        ));
    }

    #[test]
    fn byte_buffers_hide_their_size_parameter() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkGetQueryPoolResults",
            &["VK_SUCCESS", "VK_NOT_READY"],
            vec![
                field("device", "VkDevice"),
                field("dataSize", "size_t"),
                counted("pData", "void*", "dataSize"),
                field("stride", "VkDeviceSize"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_query_pool_results(&self, data: &mut [u8], stride: DeviceSize) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("data_size_len, data.as_mut_ptr().cast(), stride"));
    }
}
