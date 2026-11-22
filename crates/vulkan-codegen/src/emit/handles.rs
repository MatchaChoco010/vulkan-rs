use super::{header, type_aliases, w};
use crate::model::Model;
use crate::naming::type_name;

struct Representation {
    ty: &'static str,
    null: &'static str,
    is_null: &'static str,
    as_u64: &'static str,
    from_u64: &'static str,
    is_thread_safe_pointer: bool,
}

const DISPATCHABLE: Representation = Representation {
    ty: "*mut c_void",
    null: "core::ptr::null_mut()",
    is_null: "self.0.is_null()",
    as_u64: "self.0 as usize as u64",
    from_u64: "value as usize as *mut c_void",
    is_thread_safe_pointer: true,
};

const NON_DISPATCHABLE: Representation = Representation {
    ty: "u64",
    null: "0",
    is_null: "self.0 == 0",
    as_u64: "self.0",
    from_u64: "value",
    is_thread_safe_pointer: false,
};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&[]);
    out.push_str("use core::ffi::c_void;\n\n");
    for h in &model.ir.handles {
        let cfg = model.cfg(h.protect.as_deref());
        let name = type_name(&h.name);
        let object_type = model.enum_variant("VkObjectType", &h.object_type);
        let Representation {
            ty,
            null,
            is_null,
            as_u64,
            from_u64,
            is_thread_safe_pointer,
        } = if h.dispatchable {
            DISPATCHABLE
        } else {
            NON_DISPATCHABLE
        };
        let kind = if h.dispatchable {
            "C dispatchable handle"
        } else {
            "C non-dispatchable handle"
        };
        let provenance = if h.extensions.is_empty() {
            String::new()
        } else {
            model.provenance(&cfg, None, &h.extensions, None)
        };
        let parent = match &h.parent {
            Some(parent) => format!(
                "Objects of this type are created from a {}.",
                model.link(&cfg, parent)
            ),
            None => String::new(),
        };
        let doc = model
            .type_doc(&cfg, &h.name, kind, &provenance)
            .text(format!(
                "{parent} The value is the handle itself: copying it does not copy the object, and dropping it does not destroy the object."
            ))
            .render();
        w!(
            out,
            r#"{doc}{cfg}#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct {name}(pub {ty});

{cfg}impl {name} {{
    /// The null handle, `VK_NULL_HANDLE`.
    pub const fn null() -> Self {{ Self({null}) }}
    /// Whether this is the null handle.
    pub const fn is_null(self) -> bool {{ {is_null} }}
    /// Wraps a handle value of the C API.
    pub const fn from_raw(value: {ty}) -> Self {{ Self(value) }}
    /// The handle value as the C API passes it.
    pub const fn as_raw(self) -> {ty} {{ self.0 }}
}}

{cfg}impl Default for {name} {{
    fn default() -> Self {{ Self::null() }}
}}

{cfg}impl Handle for {name} {{
    const TYPE: ObjectType = ObjectType::{object_type};
    fn as_raw_u64(self) -> u64 {{ {as_u64} }}
    fn from_raw_u64(value: u64) -> Self {{ Self({from_u64}) }}
}}

{cfg}impl core::fmt::Debug for {name} {{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {{
        write!(f, "{name}({{:#x}})", self.as_raw_u64())
    }}
}}
"#
        );
        if is_thread_safe_pointer {
            w!(
                out,
                "{cfg}unsafe impl Send for {name} {{}}\n{cfg}unsafe impl Sync for {name} {{}}\n"
            );
        }
        out.push_str(&type_aliases(&cfg, &h.aliases, &name, ""));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::registry;
    use vulkan_registry::Handle;

    #[test]
    fn dispatchable_handles_are_send_and_sync() {
        let mut ir = registry();
        ir.handles.push(Handle {
            name: "VkDevice".to_string(),
            object_type: "VK_OBJECT_TYPE_DEVICE".to_string(),
            dispatchable: true,
            ..Default::default()
        });
        ir.handles.push(Handle {
            name: "VkBuffer".to_string(),
            object_type: "VK_OBJECT_TYPE_BUFFER".to_string(),
            dispatchable: false,
            ..Default::default()
        });
        let out = emit(&Model::new(&ir));
        assert!(out.contains("pub struct Device(pub *mut c_void);"));
        assert!(out.contains("unsafe impl Send for Device {}"));
        assert!(out.contains("unsafe impl Sync for Device {}"));
        assert!(out.contains("pub struct Buffer(pub u64);"));
        assert!(!out.contains("unsafe impl Send for Buffer {}"));
        assert!(out.contains("const TYPE: ObjectType = ObjectType::BUFFER;"));
    }
}
