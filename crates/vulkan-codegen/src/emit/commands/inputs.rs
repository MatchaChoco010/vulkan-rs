use super::{argument_name, can_defer, inner_len, struct_member_len, value_count_param};
use crate::docs::join_and;
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
    pub(super) names: Vec<(String, String)>,
    pub(super) checks: String,
    pub(super) notes: Vec<String>,
    pub(super) narrowed_counts: Vec<String>,
    pub(super) has_slices_of_slices: bool,
    type_parameters: Vec<String>,
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
            narrowed_counts: Vec::new(),
            locals: Vec::new(),
            arrays_of_pointed_count: BTreeMap::new(),
            type_parameters: Vec::new(),
        };
        let by_param = (0..params.len())
            .map(|index| (!outputs.contains(&index)).then(|| planner.input(index)))
            .collect::<Vec<_>>();
        planner.note_pointed_counts();
        let mut names = by_param
            .iter()
            .zip(params)
            .filter_map(|(input, p)| match input.as_ref()? {
                Input::Argument { name, .. } => {
                    Some((format!("the C parameter `{}`", p.name), name.clone()))
                }
                Input::SliceLen { local, .. } => Some((
                    format!(
                        "the slice length that replaces the C parameter `{}`",
                        p.name
                    ),
                    local.clone(),
                )),
            })
            .collect::<Vec<_>>();
        names.append(&mut planner.locals);
        Self {
            by_param,
            names,
            checks: planner.checks,
            notes: planner.notes,
            narrowed_counts: planner.narrowed_counts,
            has_slices_of_slices: !planner.inner_lens.is_empty()
                && !planner.caller_owns_every_pointer,
            type_parameters: planner.type_parameters,
        }
    }

    pub(super) fn generics(&self) -> String {
        if self.type_parameters.is_empty() {
            String::new()
        } else {
            format!("<{}>", self.type_parameters.join(", "))
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
    narrowed_counts: Vec<String>,
    locals: Vec<(String, String)>,
    arrays_of_pointed_count: BTreeMap<usize, Vec<usize>>,
    type_parameters: Vec<String>,
}

impl Planner<'_> {
    fn input(&mut self, index: usize) -> Input {
        let p = &self.params[index];
        let ty = CType::of(p);
        let name = argument_name(p);
        assert!(
            !is_base_structure(self.model, &ty.base)
                || (ty.depth() == 1 && !ty.is_array() && p.len.is_none()),
            "{}: {} is a `{}`, and the wrappers take a `{}` only through a pointer to one structure",
            self.c.name,
            p.name,
            p.full_ty,
            ty.base
        );
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
        let pass = if !self.model.holds_every_slice_len(&count.ty) {
            let names = slices
                .iter()
                .map(|slice| format!("`{}`", argument_name(slice)))
                .collect::<Vec<_>>();
            let has = if names.len() == 1 { "has" } else { "have" };
            self.narrowed_counts.push(format!(
                "Also panics if {} {has} more elements than `{}`, a `{}`, can count.",
                join_and(names),
                count.name,
                count.ty
            ));
            format!(
                "len_as_count({local}, \"{}\", \"{}\")",
                self.c.name, count.name
            )
        } else {
            match self.model.base_ty(&count.ty, "'_").as_str() {
                "usize" => local.clone(),
                count_ty => format!("{local} as {count_ty}"),
            }
        };
        Input::SliceLen { pass, local }
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
        self.locals.push((
            format!(
                "the pointers to the inner slices of the C parameter `{}`",
                p.name
            ),
            format!("{name}_pointers"),
        ));
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
        if is_base_structure(self.model, &ty.base) {
            return self.tagged_structure(p, ty);
        }
        reference(self.model, p, ty)
    }

    fn tagged_structure(&mut self, p: &Field, ty: &CType) -> Input {
        assert!(
            self.type_parameters.is_empty(),
            "{}: {} is a second parameter that points to a `{}`, and the wrappers have one type parameter for such a structure",
            self.c.name,
            p.name,
            ty.base
        );
        self.type_parameters
            .push(format!("{TAGGED_STRUCTURE}: TaggedStructure"));
        let cfg = self.model.cfg(self.c.protect.as_deref());
        self.notes.push(format!(
            "`{}` is passed as a pointer to a {}, which stands for any structure that begins with `sType` and `pNext`: the command takes it as the structure that its `sType` names, so `{TAGGED_STRUCTURE}` has to be a structure that the specification allows for `{}`.",
            argument_name(p),
            self.model.link(&cfg, &ty.base),
            p.name
        ));
        reference_to(p, ty, TAGGED_STRUCTURE, ".cast()")
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
        let Some(count_index) = self.params.iter().position(|q| {
            let ty = CType::of(q);
            q.name == len && ty.pointers == [false] && !ty.is_array()
        }) else {
            return;
        };
        let array_index = self
            .params
            .iter()
            .position(|q| q.name == p.name)
            .unwrap_or_else(|| panic!("{}: {} is not a parameter", self.c.name, p.name));
        self.arrays_of_pointed_count
            .entry(count_index)
            .or_default()
            .push(array_index);
        let count = &self.params[count_index];
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

    fn note_pointed_counts(&mut self) {
        for (count, arrays) in &self.arrays_of_pointed_count {
            let count = argument_name(&self.params[*count]);
            let arrays = arrays
                .iter()
                .map(|array| &self.params[*array])
                .collect::<Vec<_>>();
            let names = join_and(
                arrays
                    .iter()
                    .map(|array| format!("`{}`", argument_name(array)))
                    .collect(),
            );
            let every_array_can_be_absent = arrays.iter().all(|array| array.optional);
            let no_array_can_be_absent = arrays.iter().all(|array| !array.optional);
            let contract = match (every_array_can_be_absent, arrays.len()) {
                (true, 1) => format!(
                    "if {names} is `None`, the command writes the number of elements that it can return to it; otherwise it has to hold the number of elements of {names} when the wrapper is called"
                ),
                (true, _) => "if all of them are `None`, the command writes the number of elements that it can return to it; otherwise it has to hold the number of elements of each of them that is not `None` when the wrapper is called".to_string(),
                (false, 1) => format!(
                    "it has to hold the number of elements of {names} when the wrapper is called"
                ),
                (false, _) if no_array_can_be_absent => "it has to hold the number of elements of each of them when the wrapper is called".to_string(),
                (false, _) => "it has to hold the number of elements of each of them that is not `None` when the wrapper is called".to_string(),
            };
            self.notes.push(format!(
                "`{count}` is the element count of {names}, which the command reads and writes: {contract}, which the wrapper checks, and the command overwrites it with the number of elements that it wrote."
            ));
        }
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

const TAGGED_STRUCTURE: &str = "T";

fn is_base_structure(model: &Model<'_>, base: &str) -> bool {
    matches!(
        model.canonical(base),
        "VkBaseInStructure" | "VkBaseOutStructure"
    )
}

fn reference(model: &Model<'_>, p: &Field, ty: &CType) -> Input {
    reference_to(p, ty, &model.base_ty(&ty.base, "'_"), "")
}

fn reference_to(p: &Field, ty: &CType, referent: &str, cast: &str) -> Input {
    let name = argument_name(p);
    let borrow = Borrow::of_pointer(ty.outer_const());
    let reference_ty = borrow.reference("", referent);
    let null = null_ptr(ty.outer_const());
    let (pass, rebinds_mutably) = match (borrow, p.optional) {
        (Borrow::Shared, false) => (format!("core::ptr::from_ref({name}){cast}"), false),
        (Borrow::Exclusive, false) => (format!("core::ptr::from_mut(&mut *{name}){cast}"), false),
        (Borrow::Shared, true) if cast.is_empty() => {
            (format!("{name}.map_or({null}, core::ptr::from_ref)"), false)
        }
        (Borrow::Shared, true) => (
            format!("{name}.map_or({null}, |x| core::ptr::from_ref(x){cast})"),
            false,
        ),
        (Borrow::Exclusive, true) => (
            format!("{name}.as_mut().map_or({null}, |x| core::ptr::from_mut(&mut **x){cast})"),
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

#[cfg(test)]
mod tests {
    use super::super::{creates_a_dispatching_handle, wrapper};
    use super::*;
    use crate::emit::{doc_of, squash};
    use crate::model::Dispatch;
    use crate::test_support::{catch_quietly, counted, field, optional, registry, result_command};
    use std::collections::BTreeSet;
    use vulkan_registry::{Docs, EnumField, Handle, Registry, Struct};

    fn base_structures() -> Registry {
        let mut ir = registry();
        let results = ir.enums.iter_mut().find(|e| e.name == "VkResult").unwrap();
        results.fields.push(EnumField {
            name: "VK_SUCCESS".to_string(),
            value: "0".to_string(),
            ..Default::default()
        });
        ir.handles.push(Handle {
            name: "VkDevice".to_string(),
            object_type: "VK_OBJECT_TYPE_DEVICE".to_string(),
            dispatchable: true,
            ..Default::default()
        });
        ir.structs.push(Struct {
            name: "VkBaseOutStructure".to_string(),
            members: vec![
                field("sType", "VkStructureType"),
                field("pNext", "struct VkBaseOutStructure*"),
            ],
            ..Default::default()
        });
        ir.structs.push(Struct {
            name: "VkBaseInStructure".to_string(),
            members: vec![
                field("sType", "VkStructureType"),
                field("pNext", "const struct VkBaseInStructure*"),
            ],
            ..Default::default()
        });
        ir
    }

    fn wrapper_of(ir: &Registry, params: Vec<Field>) -> String {
        let mut ir = ir.clone();
        let mut c = result_command("vkGetWidgetPropertiesNV", &["VK_SUCCESS"], params);
        c.device = true;
        ir.commands.push(c);
        let model = Model::new(&ir);
        wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device)
    }

    #[test]
    fn a_pointer_to_a_base_structure_takes_any_tagged_structure() {
        let ir = base_structures();
        let out = wrapper_of(
            &ir,
            vec![
                field("device", "VkDevice"),
                field("index", "uint32_t"),
                field("pProperties", "VkBaseOutStructure*"),
            ],
        );
        assert!(
            squash(&out).contains(
                "pub unsafe fn get_widget_properties_nv<T: TaggedStructure>(&self, index: u32, properties: &mut T) -> crate::vk::Result<()> { unsafe { self.get_widget_properties_nv_raw(index, core::ptr::from_mut(&mut *properties).cast()).into_result() } }"
            ),
            "{out}"
        );
        let doc = doc_of(&out, "pub unsafe fn get_widget_properties_nv").join(" ");
        assert!(
            doc.contains(
                "`properties` is passed as a pointer to a [`VkBaseOutStructure`](BaseOutStructure), which stands for any structure that begins with `sType` and `pNext`: the command takes it as the structure that its `sType` names, so `T` has to be a structure that the specification allows for `pProperties`."
            ),
            "{doc}"
        );
    }

    #[test]
    fn every_way_of_passing_a_base_structure_casts_the_pointer_to_the_tagged_structure() {
        let ir = base_structures();
        for (param, argument, pass) in [
            (
                field("pInfo", "const VkBaseInStructure*"),
                "info: &T",
                "core::ptr::from_ref(info).cast()",
            ),
            (
                optional(field("pInfo", "const VkBaseInStructure*")),
                "info: Option<&T>",
                "info.map_or(core::ptr::null(), |x| core::ptr::from_ref(x).cast())",
            ),
            (
                optional(field("pInfo", "VkBaseOutStructure*")),
                "mut info: Option<&mut T>",
                "info.as_mut().map_or(core::ptr::null_mut(), |x| core::ptr::from_mut(&mut **x).cast())",
            ),
        ] {
            let out = squash(&wrapper_of(
                &ir,
                vec![
                    field("device", "VkDevice"),
                    param,
                    field("flags", "uint32_t"),
                ],
            ));
            assert!(
                out.contains(&format!(
                    "pub unsafe fn get_widget_properties_nv<T: TaggedStructure>(&self, {argument}, flags: u32) -> crate::vk::Result<()> {{ unsafe {{ self.get_widget_properties_nv_raw({pass}, flags).into_result() }} }}"
                )),
                "{argument}: {out}"
            );
        }
    }

    #[test]
    fn other_structures_are_taken_by_their_own_type() {
        let mut ir = base_structures();
        ir.structs.push(crate::test_support::tagged(
            "VkWidgetPropertiesNV",
            "VK_STRUCTURE_TYPE_WIDGET_PROPERTIES_NV",
            vec![],
        ));
        let out = squash(&wrapper_of(
            &ir,
            vec![
                field("device", "VkDevice"),
                field("pProperties", "VkWidgetPropertiesNV*"),
            ],
        ));
        assert!(
            out.contains(
                "pub unsafe fn get_widget_properties_nv(&self, properties: &mut WidgetPropertiesNV<'_>) -> crate::vk::Result<()> { unsafe { self.get_widget_properties_nv_raw(core::ptr::from_mut(&mut *properties)).into_result() } }"
            ),
            "{out}"
        );
    }

    #[test]
    fn base_structures_that_are_not_one_pointed_to_structure_stop_the_generator() {
        let ir = base_structures();
        for (params, message) in [
            (
                vec![
                    field("device", "VkDevice"),
                    field("pFirst", "const VkBaseInStructure*"),
                    field("pSecond", "VkBaseOutStructure*"),
                    field("flags", "uint32_t"),
                ],
                "vkGetWidgetPropertiesNV: pSecond is a second parameter that points to a `VkBaseOutStructure`, and the wrappers have one type parameter for such a structure",
            ),
            (
                vec![
                    field("device", "VkDevice"),
                    field("infoCount", "uint32_t"),
                    counted("pInfos", "const VkBaseInStructure*", "infoCount"),
                ],
                "vkGetWidgetPropertiesNV: pInfos is a `const VkBaseInStructure*`, and the wrappers take a `VkBaseInStructure` only through a pointer to one structure",
            ),
            (
                vec![
                    field("device", "VkDevice"),
                    field("ppInfo", "const VkBaseInStructure* const*"),
                    field("flags", "uint32_t"),
                ],
                "vkGetWidgetPropertiesNV: ppInfo is a `const VkBaseInStructure* const*`, and the wrappers take a `VkBaseInStructure` only through a pointer to one structure",
            ),
        ] {
            assert_eq!(
                catch_quietly(|| wrapper_of(&ir, params)).expect_err("the generator stops"),
                message
            );
        }
    }

    #[test]
    fn the_wrappers_of_the_registry_take_no_base_structure_by_its_own_type() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut generic = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            let signature = out
                .split_once(" { unsafe {")
                .map_or(out.as_str(), |(signature, _)| signature);
            for base in ["BaseInStructure", "BaseOutStructure"] {
                assert!(
                    !signature.contains(base),
                    "{}: the wrapper takes or returns a `{base}`, which has room for `sType` and `pNext` only: {signature}",
                    c.name
                );
            }
            let params = model.dispatch(c).explicit_params(c);
            let base_structures = params
                .iter()
                .filter(|p| is_base_structure(&model, &p.ty))
                .collect::<Vec<_>>();
            let type_parameter = signature.contains("<T: TaggedStructure>(&self");
            assert_eq!(
                type_parameter,
                !base_structures.is_empty(),
                "{}: {signature}",
                c.name
            );
            for p in base_structures {
                let name = argument_name(p);
                let mutability = if CType::of(p).outer_const() {
                    ""
                } else {
                    "mut "
                };
                assert!(
                    signature.contains(&format!(", {name}: &{mutability}T")),
                    "{}: `{name}` is not a reference to the type parameter: {signature}",
                    c.name
                );
                generic.insert(format!("{}::{}", c.name, p.name));
            }
        }
        assert_eq!(
            generic,
            [
                "vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM::pProperties",
                "vkGetPipelinePropertiesEXT::pPipelineProperties",
            ]
            .into_iter()
            .map(String::from)
            .collect::<BTreeSet<_>>(),
            "crates/vulkan/tests/base_structure_parameters.rs calls each of these wrappers"
        );
    }
}
