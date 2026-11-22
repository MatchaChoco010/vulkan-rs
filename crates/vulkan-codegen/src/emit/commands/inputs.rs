use super::{argument_name, can_defer, inner_len, struct_member_len, value_count_param};
use crate::emit::{Borrow, w};
use crate::model::{CType, Model, is_opaque, null_ptr};
use crate::naming::field_name;
use std::collections::BTreeMap;
use vulkan_registry::{Command, Field};

pub(super) enum Input {
    Argument {
        name: String,
        ty: String,
        pass: String,
        rebinds_mutably: bool,
    },
    SliceLen {
        local: String,
        pass: String,
    },
}

impl Input {
    fn argument(name: &str, ty: impl Into<String>, pass: impl Into<String>) -> Self {
        Input::Argument {
            name: name.to_string(),
            ty: ty.into(),
            pass: pass.into(),
            rebinds_mutably: false,
        }
    }

    fn passed_through(name: &str, ty: String) -> Self {
        Self::argument(name, ty, name)
    }
}

pub(super) struct Inputs {
    by_param: Vec<Option<Input>>,
    pub(super) checks: String,
    pub(super) notes: Vec<String>,
    pub(super) has_slices_of_slices: bool,
}

impl Inputs {
    pub(super) fn plan(
        model: &Model<'_>,
        c: &Command,
        params: &[Field],
        outputs: &[usize],
    ) -> Self {
        let inner_lens = params
            .iter()
            .enumerate()
            .filter(|(index, _)| !outputs.contains(index))
            .filter_map(|(index, p)| Some((index, inner_len(model, c, params, p)?)))
            .collect::<BTreeMap<_, _>>();
        let mut planner = Planner {
            model,
            c,
            params,
            slices_of_count: slices_of_count(params, outputs, &inner_lens),
            inner_lens,
            caller_owns_every_pointer: can_defer(model, c),
            checks: String::new(),
            notes: Vec::new(),
        };
        let by_param = (0..params.len())
            .map(|index| (!outputs.contains(&index)).then(|| planner.input(index)))
            .collect();
        Self {
            by_param,
            checks: planner.checks,
            notes: planner.notes,
            has_slices_of_slices: !planner.inner_lens.is_empty()
                && !planner.caller_owns_every_pointer,
        }
    }

    pub(super) fn signature(&self) -> String {
        self.by_param
            .iter()
            .flatten()
            .filter_map(|input| match input {
                Input::Argument {
                    name,
                    ty,
                    rebinds_mutably,
                    ..
                } => {
                    let mutable = if *rebinds_mutably { "mut " } else { "" };
                    Some(format!(", {mutable}{name}: {ty}"))
                }
                Input::SliceLen { .. } => None,
            })
            .collect()
    }

    pub(super) fn pass(&self, param: usize) -> Option<&str> {
        match self.by_param[param].as_ref()? {
            Input::Argument { pass, .. } | Input::SliceLen { pass, .. } => Some(pass),
        }
    }

    pub(super) fn slice_len_local(&self, count_param: usize) -> Option<&str> {
        match self.by_param[count_param].as_ref()? {
            Input::SliceLen { local, .. } => Some(local),
            Input::Argument { .. } => None,
        }
    }
}

fn slices_of_count(
    params: &[Field],
    outputs: &[usize],
    inner_lens: &BTreeMap<usize, (usize, String)>,
) -> BTreeMap<usize, Vec<usize>> {
    let mut slices_of_count = BTreeMap::<usize, Vec<usize>>::new();
    for (index, p) in params.iter().enumerate() {
        let ty = CType::of(p);
        let is_slice = ty.depth() == 1 || inner_lens.contains_key(&index);
        if outputs.contains(&index) || !is_slice || ty.is_array() {
            continue;
        }
        if let Some(count) = value_count_param(params, p)
            && !outputs.contains(&count)
        {
            slices_of_count.entry(count).or_default().push(index);
        }
    }
    slices_of_count
}

struct Planner<'a> {
    model: &'a Model<'a>,
    c: &'a Command,
    params: &'a [Field],
    slices_of_count: BTreeMap<usize, Vec<usize>>,
    inner_lens: BTreeMap<usize, (usize, String)>,
    caller_owns_every_pointer: bool,
    checks: String,
    notes: Vec<String>,
}

impl Planner<'_> {
    fn input(&mut self, index: usize) -> Input {
        let p = &self.params[index];
        let ty = CType::of(p);
        let name = argument_name(p);
        if let Some(slices) = self.slices_of_count.get(&index).cloned() {
            return self.slice_len(p, &slices);
        }
        if let Some((owner, member)) = self.inner_lens.get(&index).cloned() {
            if self.caller_owns_every_pointer {
                let element = self.model.base_ty(&ty.base, "'_");
                return Input::argument(
                    &name,
                    format!("&[*const {element}]"),
                    format!("{name}.as_ptr()"),
                );
            }
            return self.slice_of_slices(p, &ty, owner, &member);
        }
        if ty.is_array() {
            assert_eq!(ty.depth(), 0, "{}: pointer array parameter", self.c.name);
            let borrow = Borrow::of_pointer(ty.base_is_const);
            let array = self.model.member_ty(p, "'_");
            return Input::passed_through(&name, borrow.reference("", &array));
        }
        match ty.depth() {
            0 if self.model.canonical(&p.ty) == "VkBool32" => {
                Input::argument(&name, "bool", format!("{name} as Bool32"))
            }
            0 => Input::passed_through(&name, self.model.base_ty(&p.ty, "'_")),
            1 => self.pointer(p, &ty),
            _ => Input::passed_through(&name, self.model.param_ty(p, "'_")),
        }
    }

    fn slice_len(&mut self, count: &Field, slices: &[usize]) -> Input {
        let local = format!("{}_len", argument_name(count));
        let slices = slices.iter().map(|&s| &self.params[s]).collect::<Vec<_>>();
        let required = slices.iter().copied().find(|slice| !slice.optional);
        match required {
            Some(required) => w!(
                self.checks,
                "let {local}: usize = {}.len();",
                argument_name(required)
            ),
            None => {
                let lens = slices
                    .iter()
                    .map(|slice| format!("{}.as_ref().map(|x| x.len())", argument_name(slice)))
                    .collect::<Vec<_>>();
                w!(
                    self.checks,
                    "let {local}: usize = [{}].into_iter().flatten().next().unwrap_or(0);",
                    lens.join(", ")
                );
            }
        }
        let defines_len = required.unwrap_or(slices[0]);
        for slice in slices.iter().filter(|slice| slice.name != defines_len.name) {
            let message = format!(
                "{}: `{}` must have the same length as the other arrays sharing `{}`",
                self.c.name,
                argument_name(slice),
                count.name
            );
            self.assert_slice_len(slice, &local, &message);
        }
        let cast = match self.model.base_ty(&count.ty, "'_").as_str() {
            "usize" => String::new(),
            count_ty => format!(" as {count_ty}"),
        };
        Input::SliceLen {
            pass: format!("{local}{cast}"),
            local,
        }
    }

    fn slice_of_slices(&mut self, p: &Field, ty: &CType, owner: usize, member: &str) -> Input {
        let name = argument_name(p);
        let owner = argument_name(&self.params[owner]);
        let member = field_name(member);
        let element = self.model.base_ty(&ty.base, "'_");
        w!(
            self.checks,
            "for (element, entry) in {owner}.iter().zip({name}.iter()) {{
                if let Some(entry) = entry {{
                    assert_eq!(entry.len(), element.{member} as usize, \"{}: each `{name}` entry that is not `None` must have `{member}` elements\");
                }}
            }}
            let {name}_pointers = {name}.iter().map(|entry| entry.map_or(core::ptr::null(), |entry| entry.as_ptr())).collect::<Vec<_>>();",
            self.c.name
        );
        Input::argument(
            &name,
            format!("&[Option<&[{element}]>]"),
            format!("{name}_pointers.as_ptr()"),
        )
    }

    fn assert_slice_len(&mut self, slice: &Field, expected: &str, message: &str) {
        let name = argument_name(slice);
        if slice.optional {
            w!(
                self.checks,
                "if let Some(x) = {name}.as_ref() {{ assert_eq!(x.len(), {expected}, \"{message}\"); }}"
            );
        } else {
            w!(
                self.checks,
                "assert_eq!({name}.len(), {expected}, \"{message}\");"
            );
        }
    }

    fn pointer(&mut self, p: &Field, ty: &CType) -> Input {
        let name = argument_name(p);
        let opaque = is_opaque(&ty.base);
        let shares_a_count_param = self
            .slices_of_count
            .values()
            .any(|slices| slices.iter().any(|&s| self.params[s].name == p.name));
        if shares_a_count_param || (p.len.is_some() && !opaque) {
            self.assert_struct_member_len(p);
            self.assert_pointed_count_len(p);
            let uncounted = !shares_a_count_param && !self.names_its_length(p);
            if uncounted {
                self.notes.push(format!(
                    "An empty `{name}` is passed as a null pointer, because no parameter passes its length."
                ));
            }
            return slice(self.model, p, ty, uncounted);
        }
        if p.null_terminated && ty.base == "char" && ty.outer_const() {
            return if p.optional {
                Input::argument(
                    &name,
                    "Option<&CStr>",
                    format!("{name}.map_or(core::ptr::null(), |x| x.as_ptr())"),
                )
            } else {
                Input::argument(&name, "&CStr", format!("{name}.as_ptr()"))
            };
        }
        if opaque {
            return Input::passed_through(&name, self.model.param_ty(p, "'_"));
        }
        reference(self.model, p, ty)
    }

    fn names_its_length(&self, p: &Field) -> bool {
        let Some(len) = p.len.as_deref() else {
            return false;
        };
        let owner = struct_member_len(Some(len)).map_or(len, |(owner, _)| owner);
        self.params.iter().any(|q| q.name == owner)
    }

    fn assert_pointed_count_len(&mut self, p: &Field) {
        let Some(len) = p.len.as_deref() else {
            return;
        };
        let Some(count) = self.params.iter().find(|q| {
            let ty = CType::of(q);
            q.name == len && ty.pointers == [false] && !ty.is_array()
        }) else {
            return;
        };
        let count_name = argument_name(count);
        let cast = match self.model.base_ty(&count.ty, "'_").as_str() {
            "usize" => "",
            _ => " as usize",
        };
        let message = format!(
            "{}: `{}` must have `*{count_name}` elements",
            self.c.name,
            argument_name(p)
        );
        self.assert_slice_len(p, &format!("*{count_name}{cast}"), &message);
    }

    fn assert_struct_member_len(&mut self, p: &Field) {
        let Some((owner, member)) = struct_member_len(p.len.as_deref()) else {
            return;
        };
        let Some(owner) = self.params.iter().find(|q| q.name == owner) else {
            return;
        };
        let (owner, member) = (argument_name(owner), field_name(member));
        let message = format!(
            "{}: `{}` must have `{owner}.{member}` elements",
            self.c.name,
            argument_name(p)
        );
        self.assert_slice_len(p, &format!("{owner}.{member} as usize"), &message);
    }
}

fn slice(model: &Model<'_>, p: &Field, ty: &CType, null_if_empty: bool) -> Input {
    let name = argument_name(p);
    let (element, cast) = match ty.base.as_str() {
        "void" => ("u8".to_string(), ".cast()"),
        base => (model.base_ty(base, "'_"), ""),
    };
    let borrow = Borrow::of_pointer(ty.outer_const());
    let as_ptr = borrow.slice_as_ptr();
    let slice_ty = borrow.reference("", &format!("[{element}]"));
    let null = null_ptr(ty.outer_const());
    let pointer = |slice: &str| {
        if null_if_empty {
            format!("if {slice}.is_empty() {{ {null} }} else {{ {slice}.{as_ptr}(){cast} }}")
        } else {
            format!("{slice}.{as_ptr}(){cast}")
        }
    };
    if !p.optional {
        return Input::argument(&name, slice_ty, pointer(&name));
    }
    let reborrowed = match borrow {
        Borrow::Shared => name.clone(),
        Borrow::Exclusive => format!("{name}.as_mut()"),
    };
    Input::Argument {
        pass: format!("{reborrowed}.map_or({null}, |x| {})", pointer("x")),
        ty: format!("Option<{slice_ty}>"),
        rebinds_mutably: borrow == Borrow::Exclusive,
        name,
    }
}

fn reference(model: &Model<'_>, p: &Field, ty: &CType) -> Input {
    let name = argument_name(p);
    let borrow = Borrow::of_pointer(ty.outer_const());
    let reference_ty = borrow.reference("", &model.base_ty(&ty.base, "'_"));
    let null = null_ptr(ty.outer_const());
    let (pass, rebinds_mutably) = match (borrow, p.optional) {
        (Borrow::Shared, false) => (format!("core::ptr::from_ref({name})"), false),
        (Borrow::Exclusive, false) => (format!("core::ptr::from_mut(&mut *{name})"), false),
        (Borrow::Shared, true) => (format!("{name}.map_or({null}, core::ptr::from_ref)"), false),
        (Borrow::Exclusive, true) => (
            format!("{name}.as_mut().map_or({null}, |x| core::ptr::from_mut(&mut **x))"),
            true,
        ),
    };
    let ty = if p.optional {
        format!("Option<{reference_ty}>")
    } else {
        reference_ty
    };
    Input::Argument {
        name,
        ty,
        pass,
        rebinds_mutably,
    }
}
