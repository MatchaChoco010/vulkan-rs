use super::{CType, Model, POINTER_TYPEDEFS, is_platform_type};
use crate::naming::{const_name, func_pointer_name, type_name};
use std::collections::BTreeSet;
use vulkan_registry::Field;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Scalar {
    Integer,
    Float,
}

const C_SCALARS: &[(&str, &str, Scalar)] = &[
    ("char", "c_char", Scalar::Integer),
    ("float", "f32", Scalar::Float),
    ("double", "f64", Scalar::Float),
    ("int", "core::ffi::c_int", Scalar::Integer),
    ("uint8_t", "u8", Scalar::Integer),
    ("int8_t", "i8", Scalar::Integer),
    ("uint16_t", "u16", Scalar::Integer),
    ("int16_t", "i16", Scalar::Integer),
    ("uint32_t", "u32", Scalar::Integer),
    ("int32_t", "i32", Scalar::Integer),
    ("uint64_t", "u64", Scalar::Integer),
    ("int64_t", "i64", Scalar::Integer),
    ("size_t", "usize", Scalar::Integer),
];

const INTEGER_TYPEDEFS: &[&str] = &[
    "VkBool32",
    "VkFlags",
    "VkFlags64",
    "VkDeviceSize",
    "VkDeviceAddress",
    "VkSampleMask",
];

fn c_scalar(name: &str) -> Option<(&'static str, Scalar)> {
    C_SCALARS
        .iter()
        .find(|(c_name, _, _)| *c_name == name)
        .map(|(_, rust_name, scalar)| (*rust_name, *scalar))
}

impl<'a> Model<'a> {
    pub(crate) fn scalar(&self, name: &str) -> Option<Scalar> {
        if INTEGER_TYPEDEFS.contains(&name) {
            return Some(Scalar::Integer);
        }
        c_scalar(name).map(|(_, scalar)| scalar)
    }

    pub(crate) fn holds_every_slice_len(&self, name: &str) -> bool {
        matches!(
            self.canonical(name),
            "size_t" | "uint64_t" | "int64_t" | "VkDeviceSize" | "VkDeviceAddress" | "VkFlags64"
        )
    }

    pub(crate) fn base_ty(&self, name: &str, lifetime: &str) -> String {
        let name = self.canonical(name);
        if name == "void" {
            return "c_void".to_string();
        }
        if let Some((rust_name, _)) = c_scalar(name) {
            return rust_name.to_string();
        }
        if self.structs.contains_key(name) && self.has_lifetime(name) {
            return format!("{}<{lifetime}>", type_name(name));
        }
        let is_generated_or_hand_declared = INTEGER_TYPEDEFS.contains(&name)
            || POINTER_TYPEDEFS.contains(&name)
            || self.structs.contains_key(name)
            || self.handles.contains(name)
            || self.enums.contains(name)
            || self.bitmasks.contains(name)
            || self.flags.contains(name);
        if is_generated_or_hand_declared {
            return type_name(name);
        }
        if self.is_func_pointer(name) {
            return func_pointer_name(name);
        }
        assert!(
            is_platform_type(name),
            "unknown C type `{name}`: declare it in crates/vulkan/src/vk/platform.rs and add it \
             to the platform types in vulkan-codegen"
        );
        name.to_string()
    }

    pub(crate) fn pointer_ty(&self, ty: &CType, lifetime: &str) -> String {
        ty.pointers
            .iter()
            .fold(self.base_ty(&ty.base, lifetime), |pointee, is_const| {
                format!("{} {pointee}", if *is_const { "*const" } else { "*mut" })
            })
    }

    fn array_ty(&self, ty: &CType, lifetime: &str) -> String {
        ty.arrays
            .iter()
            .rev()
            .fold(self.pointer_ty(ty, lifetime), |element, len| {
                format!("[{element}; {}]", self.array_len(len))
            })
    }

    pub(crate) fn member_ty(&self, m: &Field, lifetime: &str) -> String {
        self.array_ty(&CType::of(m), lifetime)
    }

    pub(crate) fn param_ty(&self, p: &Field, lifetime: &str) -> String {
        let ty = CType::of(p);
        let spelled = self.array_ty(&ty, lifetime);
        if !ty.is_array() {
            return spelled;
        }
        let decayed = if ty.base_is_const { "*const" } else { "*mut" };
        format!("{decayed} {spelled}")
    }

    pub(crate) fn return_ty(&self, name: &str) -> String {
        if name == "void" {
            return "()".to_string();
        }
        self.pointer_ty(&CType::parse(name, &[]), "'_")
    }

    pub(crate) fn array_len(&self, value: &str) -> String {
        if value.chars().all(|c| c.is_ascii_digit()) {
            return value.to_string();
        }
        assert!(
            self.constants.contains(value),
            "array length `{value}` is not a registry constant"
        );
        const_name(value)
    }

    pub(crate) fn array_len_constants(&self) -> BTreeSet<&'a str> {
        let members = self.ir.structs.iter().flat_map(|s| &s.members);
        let params = self.ir.commands.iter().flat_map(|c| &c.params);
        members
            .chain(params)
            .flat_map(|field| &field.fixed_size_array)
            .filter_map(|len| self.constants.get(len.as_str()).copied())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{field, registry};

    #[test]
    fn array_parameters_decay_to_pointers() {
        let ir = registry();
        let model = Model::new(&ir);
        let mut p = field("blendConstants", "const float");
        p.fixed_size_array = vec!["4".to_string()];
        assert_eq!(model.param_ty(&p, "'_"), "*const [f32; 4]");
        assert_eq!(model.member_ty(&p, "'a"), "[f32; 4]");
    }

    #[test]
    fn pointers_are_spelled_from_the_pointee_outwards() {
        let ir = registry();
        let model = Model::new(&ir);
        assert_eq!(
            model.member_ty(&field("ppNames", "const char* const*"), "'a"),
            "*const *const c_char"
        );
        assert_eq!(model.return_ty("void"), "()");
        assert_eq!(model.return_ty("void*"), "*mut c_void");
        assert_eq!(model.return_ty("VkBool32"), "Bool32");
    }

    #[test]
    #[should_panic(expected = "unknown C type `HCURSOR`")]
    fn a_type_the_bindings_do_not_declare_aborts_generation() {
        let ir = registry();
        Model::new(&ir).base_ty("HCURSOR", "'a");
    }
}
