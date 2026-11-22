use vulkan_registry::Field;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct CType {
    pub(crate) base: String,
    pub(crate) base_is_const: bool,
    pub(crate) pointers: Vec<bool>,
    pub(crate) arrays: Vec<String>,
}

impl CType {
    pub(crate) fn parse(full_ty: &str, fixed_size_array: &[String]) -> Self {
        let is_const = |part: &str| part.split_whitespace().any(|token| token == "const");
        let parts = full_ty.split('*').collect::<Vec<_>>();
        let (levels, _after_last_star) = parts.split_at(parts.len() - 1);
        let base = parts[0]
            .split_whitespace()
            .filter(|token| *token != "const" && *token != "struct")
            .collect::<Vec<_>>()
            .join(" ");
        Self {
            base,
            base_is_const: is_const(parts[0]),
            pointers: levels.iter().map(|part| is_const(part)).collect(),
            arrays: fixed_size_array.to_vec(),
        }
    }

    pub(crate) fn of(field: &Field) -> Self {
        Self::parse(&field.full_ty, &field.fixed_size_array)
    }

    pub(crate) fn depth(&self) -> usize {
        self.pointers.len()
    }

    pub(crate) fn is_array(&self) -> bool {
        !self.arrays.is_empty()
    }

    pub(crate) fn outer_const(&self) -> bool {
        self.pointers.last().copied().unwrap_or(false)
    }

    pub(crate) fn pointee(&self) -> Self {
        let (_outermost, inner) = self
            .pointers
            .split_last()
            .expect("pointee of a type that is not a pointer");
        Self {
            base: self.base.clone(),
            base_is_const: self.base_is_const,
            pointers: inner.to_vec(),
            arrays: Vec::new(),
        }
    }
}

pub(crate) fn null_ptr(is_const: bool) -> &'static str {
    if is_const {
        "core::ptr::null()"
    } else {
        "core::ptr::null_mut()"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pointer_levels_and_constness() {
        let ty = CType::parse("const char* const*", &[]);
        assert_eq!(ty.base, "char");
        assert_eq!(ty.pointers, vec![true, true]);
        assert!(ty.base_is_const);

        let ty = CType::parse("void**", &[]);
        assert_eq!(ty.base, "void");
        assert_eq!(ty.pointers, vec![false, false]);
        assert!(!ty.base_is_const);

        let ty = CType::parse("struct wl_display*", &[]);
        assert_eq!(ty.base, "wl_display");
        assert_eq!(ty.pointers, vec![false]);

        let ty = CType::parse("const VkAccelerationStructureGeometryKHR* const *", &[]);
        assert_eq!(ty.pointers, vec![true, true]);

        let ty = CType::parse("uint32_t", &["3".to_string(), "4".to_string()]);
        assert_eq!(ty.depth(), 0);
        assert_eq!(ty.arrays, vec!["3", "4"]);
    }

    #[test]
    fn pointee_drops_the_outermost_pointer() {
        let ty = CType::parse("const char* const*", &[]).pointee();
        assert_eq!(ty.pointers, vec![true]);
        assert_eq!(CType::parse("void**", &[]).pointee().pointers, vec![false]);
    }
}
