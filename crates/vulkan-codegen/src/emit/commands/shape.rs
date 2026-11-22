use super::{can_defer, struct_member_len, value_count_param};
use crate::model::{CType, Model, has_p_next, is_opaque};
use crate::naming::field_name;
use vulkan_registry::{Command, Field};

#[derive(Clone, Debug, PartialEq)]
pub(super) enum Shape {
    PassThrough,
    ReturnsValue {
        out: usize,
    },
    Enumerates {
        count: usize,
        data: usize,
    },
    FillsOnePerInput {
        count: usize,
        out: usize,
    },
    FillsStructMemberCount {
        info: usize,
        member: String,
        out: usize,
    },
}

impl Shape {
    pub(super) fn detect(model: &Model<'_>, c: &Command, params: &[Field]) -> Self {
        if (c.return_ty != "void" && c.return_ty != "VkResult") || can_defer(model, c) {
            return Shape::PassThrough;
        }
        let Some((last, inputs)) = params.split_last() else {
            return Shape::PassThrough;
        };
        let out = inputs.len();
        let ty = CType::of(last);
        if ty.depth() == 0 || ty.outer_const() || ty.is_array() {
            return Shape::PassThrough;
        }
        if ty.depth() == 1 {
            if is_enumeration(params, last) {
                if !reports_only_completeness(model, c) {
                    return Shape::PassThrough;
                }
                return Shape::Enumerates {
                    count: out - 1,
                    data: out,
                };
            }
            if let Some(shape) = caller_sized_output(model, params, out, &ty) {
                return shape;
            }
        }
        let returns_value = last.len.is_none()
            && match ty.depth() {
                1 => !is_opaque(&ty.base) && !is_extensible_struct(model, &ty.base),
                2 => true,
                _ => false,
            };
        if returns_value {
            Shape::ReturnsValue { out }
        } else {
            Shape::PassThrough
        }
    }

    pub(super) fn outputs(&self) -> Vec<usize> {
        match self {
            Shape::PassThrough => vec![],
            Shape::Enumerates { count, data } => vec![*count, *data],
            Shape::ReturnsValue { out }
            | Shape::FillsOnePerInput { out, .. }
            | Shape::FillsStructMemberCount { out, .. } => vec![*out],
        }
    }
}

fn reports_only_completeness(model: &Model<'_>, c: &Command) -> bool {
    c.success_codes
        .iter()
        .all(|code| matches!(model.result_code(code), "VK_SUCCESS" | "VK_INCOMPLETE"))
}

pub(super) fn is_enumeration(params: &[Field], data: &Field) -> bool {
    let [.., count, _data] = params else {
        return false;
    };
    let Some(len) = data.len.as_deref() else {
        return false;
    };
    let count_ty = CType::of(count);
    count.name == len
        && count_ty.pointers == [false]
        && matches!(count_ty.base.as_str(), "uint32_t" | "size_t")
        && params
            .iter()
            .filter(|p| p.len.as_deref() == Some(len))
            .count()
            == 1
}

fn caller_sized_output(
    model: &Model<'_>,
    params: &[Field],
    out: usize,
    ty: &CType,
) -> Option<Shape> {
    if ty.base == "void" || model.has_lifetime(&ty.base) {
        return None;
    }
    let last = &params[out];
    if let Some(count) = value_count_param(params, last) {
        let an_input_slice_shares_the_count = params.iter().enumerate().any(|(index, p)| {
            index != out
                && CType::of(p).pointers == [true]
                && value_count_param(params, p) == Some(count)
        });
        if an_input_slice_shares_the_count {
            return Some(Shape::FillsOnePerInput { count, out });
        }
    }
    let (owner, member) = struct_member_len(last.len.as_deref())?;
    let info = params.iter().position(|p| p.name == owner)?;
    Some(Shape::FillsStructMemberCount {
        info,
        member: field_name(member),
        out,
    })
}

fn is_extensible_struct(model: &Model<'_>, name: &str) -> bool {
    model.get_struct(name).is_some_and(has_p_next)
}
