pub(crate) mod bitmasks;
pub(crate) mod command_info;
pub(crate) mod commands;
pub(crate) mod constants;
pub(crate) mod enums;
pub(crate) mod extensions;
pub(crate) mod formats;
pub(crate) mod func_pointers;
pub(crate) mod handles;
mod indentation;
pub(crate) mod layout;
pub(crate) mod structs;
mod variants;
pub(crate) mod version;

pub(crate) use indentation::indent_by_nesting;

use crate::model::Cfg;
use crate::naming::type_name;

macro_rules! w {
    ($out:expr, $($arg:tt)*) => {{
        use std::fmt::Write as _;
        writeln!($out, $($arg)*).expect("write to String")
    }};
}
pub(crate) use w;

pub(crate) fn header(clippy_allows: &[&str]) -> String {
    let mut out = String::new();
    for lint in [
        "non_camel_case_types",
        "non_snake_case",
        "non_upper_case_globals",
        "unused_imports",
    ] {
        w!(out, "#![allow({lint})]");
    }
    for lint in clippy_allows {
        w!(out, "#![allow(clippy::{lint})]");
    }
    out.push_str("\nuse crate::vk::*;\n\n");
    out
}

pub(crate) fn arrow(return_ty: &str) -> String {
    if return_ty == "()" {
        String::new()
    } else {
        format!(" -> {return_ty}")
    }
}

pub(crate) fn type_aliases(cfg: &Cfg, aliases: &[String], target: &str, generics: &str) -> String {
    aliases
        .iter()
        .map(|alias| {
            let alias = type_name(alias);
            format!("{cfg}pub type {alias}{generics} = {target}{generics};\n\n")
        })
        .collect()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Borrow {
    Shared,
    Exclusive,
}

impl Borrow {
    pub(crate) fn of_pointer(points_to_const: bool) -> Self {
        if points_to_const {
            Borrow::Shared
        } else {
            Borrow::Exclusive
        }
    }

    pub(crate) fn reference(self, lifetime: &str, referent: &str) -> String {
        let lifetime = if lifetime.is_empty() {
            String::new()
        } else {
            format!("{lifetime} ")
        };
        match self {
            Borrow::Shared => format!("&{lifetime}{referent}"),
            Borrow::Exclusive => format!("&{lifetime}mut {referent}"),
        }
    }

    pub(crate) fn slice_as_ptr(self) -> &'static str {
        match self {
            Borrow::Shared => "as_ptr",
            Borrow::Exclusive => "as_mut_ptr",
        }
    }

    pub(crate) fn ptr_from_reference(self) -> &'static str {
        match self {
            Borrow::Shared => "core::ptr::from_ref",
            Borrow::Exclusive => "core::ptr::from_mut",
        }
    }
}

#[cfg(test)]
pub(crate) fn squash(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}
