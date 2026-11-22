use super::{CType, Model, Scalar};
use std::collections::BTreeSet;
use vulkan_registry::{Field, Struct};

#[derive(Copy, Clone, PartialEq, Eq)]
pub(super) enum Floats {
    Allowed,
    Forbidden,
}

impl<'a> Model<'a> {
    pub(super) fn structs_that_borrow(&self) -> BTreeSet<&'a str> {
        let mut borrowing = BTreeSet::new();
        loop {
            let found = self
                .ir
                .structs
                .iter()
                .filter(|s| !borrowing.contains(s.name.as_str()))
                .filter(|s| {
                    s.members.iter().any(|m| {
                        self.is_data_pointer(m) || borrowing.contains(self.canonical(&m.ty))
                    })
                })
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>();
            if found.is_empty() {
                return borrowing;
            }
            borrowing.extend(found);
        }
    }

    fn is_data_pointer(&self, m: &Field) -> bool {
        CType::of(m).depth() > 0 && !self.is_func_pointer(&m.ty)
    }

    pub(super) fn structs_of_plain_values(&self, floats: Floats) -> BTreeSet<&'a str> {
        let mut plain = self
            .ir
            .structs
            .iter()
            .filter(|s| !s.union)
            .map(|s| s.name.as_str())
            .collect::<BTreeSet<_>>();
        loop {
            let still_plain = plain
                .iter()
                .copied()
                .filter(|name| {
                    self.structs[name]
                        .members
                        .iter()
                        .all(|m| self.is_plain_value(m, floats, &plain))
                })
                .collect::<BTreeSet<_>>();
            if still_plain.len() == plain.len() {
                return plain;
            }
            plain = still_plain;
        }
    }

    fn is_plain_value(&self, m: &Field, floats: Floats, plain_structs: &BTreeSet<&str>) -> bool {
        if CType::of(m).depth() > 0 || self.is_func_pointer(&m.ty) {
            return false;
        }
        let base = self.canonical(&m.ty);
        if let Some(nested) = self.structs.get(base) {
            return is_struct(nested) && plain_structs.contains(base);
        }
        if self.has_opaque_representation(base) {
            return false;
        }
        floats == Floats::Allowed || self.scalar(base) != Some(Scalar::Float)
    }
}

fn is_struct(s: &Struct) -> bool {
    !s.union
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{field, registry, structure};

    #[test]
    fn lifetime_is_only_for_structs_that_can_borrow() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkExtent2D",
            vec![field("width", "uint32_t"), field("height", "uint32_t")],
        ));
        ir.structs.push(structure(
            "VkInner",
            vec![field("pData", "const uint32_t*")],
        ));
        ir.structs.push(structure(
            "VkOuter",
            vec![field("inner", "VkInner"), field("extent", "VkExtent2D")],
        ));
        ir.func_pointers.push(vulkan_registry::FuncPointer {
            name: "PFN_vkAllocationFunction".to_string(),
            ..Default::default()
        });
        ir.structs.push(structure(
            "VkCallbacks",
            vec![field("pfnAlloc", "PFN_vkAllocationFunction")],
        ));
        let model = Model::new(&ir);
        assert!(!model.has_lifetime("VkExtent2D"));
        assert!(model.has_lifetime("VkInner"));
        assert!(model.has_lifetime("VkOuter"));
        assert!(!model.has_lifetime("VkCallbacks"));
        assert_eq!(model.base_ty("VkOuter", "'a"), "Outer<'a>");
        assert_eq!(model.base_ty("VkExtent2D", "'a"), "Extent2D");
    }

    #[test]
    fn plain_structs_derive_comparison_traits() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkExtent2D",
            vec![field("width", "uint32_t"), field("height", "uint32_t")],
        ));
        ir.structs.push(structure(
            "VkViewport",
            vec![field("x", "float"), field("extent", "VkExtent2D")],
        ));
        ir.structs.push(structure(
            "VkInner",
            vec![field("pData", "const uint32_t*")],
        ));
        let model = Model::new(&ir);
        assert!(model.derives_partial_eq("VkExtent2D"));
        assert!(model.derives_eq_hash("VkExtent2D"));
        assert!(model.derives_partial_eq("VkViewport"));
        assert!(!model.derives_eq_hash("VkViewport"));
        assert!(!model.derives_partial_eq("VkInner"));
    }
}
