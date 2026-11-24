#[cfg(test)]
mod absent_arrays;
mod bit_fields;
mod defaults;
mod read_only_members;
mod setters;
mod unions;
mod unmarked_arrays;

pub(crate) use bit_fields::{Slot, slots};
pub(crate) use read_only_members::assert_listed_read_only_members_exist;
pub(crate) use unmarked_arrays::assert_listed_structures_exist;

use super::{assert_distinct_names, header, type_aliases, w};
use crate::docs::{Doc, join_and};
use crate::model::{Cfg, Model, has_p_next};
use crate::naming::{field_name, type_name};
use std::collections::BTreeMap;
use vulkan_registry::{Field, Struct};

const MARKER_DOC: &str =
    "/// Ties the value to the lifetime `'a` of the data that its pointers borrow.\n";

pub(super) fn field_link(c_name: &str) -> String {
    let field = field_name(c_name);
    format!("[`{}`](Self::{field})", field.trim_start_matches("r#"))
}

fn c_declaration(m: &Field) -> String {
    let arrays = m
        .fixed_size_array
        .iter()
        .map(|len| format!("[{len}]"))
        .collect::<String>();
    let bits = m
        .bit_field_width
        .map(|width| format!(" : {width}"))
        .unwrap_or_default();
    format!("{} {}{arrays}{bits}", m.full_ty, m.name)
}

pub(super) fn member_doc(model: &Model<'_>, cfg: &Cfg, s: &Struct, m: &Field) -> Doc {
    match model.described(cfg, &s.name, &m.name) {
        Some(described) => Doc::new().blocks(described),
        None => Doc::new().text(format!("The C member `{}`.", c_declaration(m))),
    }
}

pub(super) fn type_doc(model: &Model<'_>, cfg: &Cfg, s: &Struct) -> Doc {
    let kind = if s.union { "C union" } else { "C structure" };
    let provenance = model.provenance(
        cfg,
        s.version.as_deref(),
        &s.extensions,
        s.video_std_header.as_deref(),
    );
    let roots = s
        .extends
        .iter()
        .filter(|root| model.get_struct(root).is_some())
        .map(|root| model.link(cfg, root))
        .collect::<Vec<_>>();
    let extends = if roots.is_empty() {
        String::new()
    } else {
        format!(
            "Extends {}: their `push_next` accepts this structure.",
            join_and(roots)
        )
    };
    model
        .type_doc(cfg, &s.name, kind, &provenance)
        .text(extends)
}

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&["derivable_impls"]);
    out.push_str("use core::ffi::{c_char, c_void, CStr};\nuse core::marker::PhantomData;\n\n");
    for s in &model.ir.structs {
        unmarked_arrays::assert_listed_members_exist(s);
        out.push_str(&if s.union {
            unions::definition(model, s)
        } else {
            definition(model, s)
        });
        let generics = if model.has_lifetime(&s.name) {
            "<'a>"
        } else {
            ""
        };
        out.push_str(&type_aliases(
            &model.cfg(s.protect.as_deref()),
            &s.aliases,
            &type_name(&s.name),
            generics,
        ));
    }
    out.push_str(&extends_impls(model));
    out
}

struct MethodNames<'s> {
    owner: &'s str,
    taken: BTreeMap<String, String>,
}

fn member_origin(m: &Field) -> String {
    format!("the C member `{}`", m.name)
}

impl<'s> MethodNames<'s> {
    fn new(owner: &'s str) -> Self {
        Self {
            owner,
            taken: BTreeMap::new(),
        }
    }

    fn is_taken(&self, name: &str) -> bool {
        self.taken.contains_key(name.trim_start_matches("r#"))
    }

    fn claim(&mut self, name: &str, origin: &str) -> String {
        let key = name.trim_start_matches("r#").to_string();
        if let Some(earlier) = self.taken.get(&key) {
            panic!(
                "{}: the methods of {earlier} and of {origin} are both named `{name}`",
                self.owner
            );
        }
        self.taken.insert(key, origin.to_string());
        name.to_string()
    }
}

fn marker_field(borrows: bool) -> Option<(String, String)> {
    borrows.then(|| ("the lifetime marker".to_string(), "_marker".to_string()))
}

fn thread_safety_impls(cfg: &Cfg, name: &str, borrows: bool) -> String {
    if !borrows {
        return String::new();
    }
    format!(
        "{cfg}unsafe impl Send for {name}<'_> {{}}\n{cfg}unsafe impl Sync for {name}<'_> {{}}\n\n"
    )
}

fn definition(model: &Model<'_>, s: &Struct) -> String {
    let cfg = model.cfg(s.protect.as_deref());
    let name = type_name(&s.name);
    let borrows = model.has_lifetime(&s.name);
    let (generics, anonymous) = if borrows { ("<'a>", "<'_>") } else { ("", "") };

    let mut derives = vec!["Copy", "Clone"];
    if model.derives_partial_eq(&s.name) {
        derives.push("PartialEq");
    }
    if model.derives_eq_hash(&s.name) {
        derives.extend(["Eq", "Hash"]);
    }

    let mut fields = String::new();
    let mut defaults = String::new();
    let mut methods = String::new();
    let mut names = MethodNames::new(&s.name);
    let counted = setters::CountedArrays::new(s);
    let slots = slots(model, s);
    assert_distinct_names(
        &s.name,
        slots
            .iter()
            .map(|slot| match slot {
                Slot::Plain(m) => (format!("the C member `{}`", m.name), field_name(&m.name)),
                Slot::Bits { storage, bits } => (
                    format!(
                        "the storage unit of the C bit-field `{}`",
                        bits[0].member.name
                    ),
                    storage.clone(),
                ),
            })
            .chain(marker_field(borrows)),
    );
    for slot in slots {
        match slot {
            Slot::Plain(m) => {
                let field = field_name(&m.name);
                w!(
                    fields,
                    "{}pub {field}: {},",
                    member_doc(model, &cfg, s, m).render(),
                    model.member_ty(m, "'a")
                );
                w!(
                    defaults,
                    "{field}: {},",
                    defaults::default_expr(model, s, m)
                );
                if m.name != "sType" && m.name != "pNext" {
                    methods.push_str(&setters::member_methods(model, s, m, &counted, &mut names));
                }
            }
            Slot::Bits { storage, bits } => {
                let members = bits
                    .iter()
                    .map(|bit| format!("`{}`", bit.member.name))
                    .collect();
                let doc = Doc::new().text(format!(
                    "The storage unit of the C bit-fields {}. The methods named after the bit-fields read and write them.",
                    join_and(members)
                ));
                w!(fields, "{}pub {storage}: u32,", doc.render());
                w!(defaults, "{storage}: 0,");
                for bit in &bits {
                    methods.push_str(&bit_fields::accessors(model, &storage, bit, &mut names));
                }
            }
        }
    }
    if borrows {
        w!(fields, "{MARKER_DOC}pub _marker: PhantomData<&'a ()>,");
        w!(defaults, "_marker: PhantomData,");
    }
    if let Some(p_next) = s.members.iter().find(|m| m.name == "pNext") {
        names.claim("push_next", "the `pNext` chain");
        methods.push_str(&setters::push_next(p_next));
    }

    let mut out = String::new();
    w!(
        out,
        r#"{doc}{cfg}#[repr(C)]
#[derive({derives})]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct {name}{generics} {{
    {fields}}}

{thread_safety}{cfg}impl Default for {name}{anonymous} {{
    fn default() -> Self {{
        Self {{
            {defaults}}}
    }}
}}

{cfg}impl{generics} {name}{generics} {{
    {methods}}}
"#,
        doc = type_doc(model, &cfg, s).render(),
        derives = derives.join(", "),
        thread_safety = thread_safety_impls(&cfg, &name, borrows),
    );
    if has_p_next(s)
        && let Some(s_type) = &s.s_type
    {
        w!(
            out,
            "{cfg}unsafe impl TaggedStructure for {name}<'_> {{
                const STRUCTURE_TYPE: StructureType = {};
            }}\n",
            model.structure_type(s_type)
        );
    }
    out
}

fn extends_impls(model: &Model<'_>) -> String {
    let mut out = String::new();
    for s in &model.ir.structs {
        for root in &s.extends {
            let Some(root) = model.get_struct(root) else {
                continue;
            };
            assert!(
                has_p_next(s) && has_p_next(root),
                "{} extends {} but one of them has no pNext",
                s.name,
                root.name
            );
            let cfg = model
                .cfg(s.protect.as_deref())
                .and(model.cfg(root.protect.as_deref()));
            w!(
                out,
                "{cfg}unsafe impl Extends<{}<'_>> for {}<'_> {{}}",
                type_name(&root.name),
                type_name(&s.name)
            );
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::squash;
    use crate::test_support::{counted, field, optional, registry, structure, tagged};
    use vulkan_registry::{Bitmask, Flags, Handle};

    #[test]
    fn plain_struct_has_no_lifetime_and_derives_comparisons() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkExtent2D",
            vec![field("width", "uint32_t"), field("height", "uint32_t")],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("#[derive(Copy, Clone, PartialEq, Eq, Hash)]"));
        assert!(out.contains("pub struct Extent2D { pub width: u32, pub height: u32, }"));
        assert!(out.contains("impl Extent2D {"));
        assert!(!out.contains("_marker"));
        assert!(!out.contains("unsafe impl Send for Extent2D"));
    }

    #[test]
    fn slice_setters_borrow_for_the_struct_lifetime() {
        let mut ir = registry();
        ir.structs.push(tagged(
            "VkDeviceQueueCreateInfo",
            "VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO",
            vec![
                field("queueCount", "uint32_t"),
                counted("pQueuePriorities", "const float*", "queueCount"),
            ],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub struct DeviceQueueCreateInfo<'a> {"));
        assert!(out.contains("pub _marker: PhantomData<&'a ()>,"));
        assert!(out.contains(
            "pub fn queue_priorities(mut self, value: &'a [f32]) -> Self { self.queue_count = len_as_count(value.len(), \"VkDeviceQueueCreateInfo::pQueuePriorities\", \"queueCount\"); self.p_queue_priorities = value.as_ptr(); self }"
        ));
        assert!(out.contains("s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,"));
        assert!(out.contains("unsafe impl TaggedStructure for DeviceQueueCreateInfo<'_> {"));
        assert!(out.contains("unsafe impl Send for DeviceQueueCreateInfo<'_> {}"));
        assert!(!out.contains("value: *const f32"));
    }

    #[test]
    fn arrays_sharing_a_count_check_each_other() {
        let mut ir = registry();
        ir.handles.push(Handle {
            name: "VkSemaphore".to_string(),
            ..Default::default()
        });
        ir.structs.push(tagged(
            "VkSubmitInfo",
            "VK_STRUCTURE_TYPE_SUBMIT_INFO",
            vec![
                field("waitSemaphoreCount", "uint32_t"),
                counted(
                    "pWaitSemaphores",
                    "const VkSemaphore*",
                    "waitSemaphoreCount",
                ),
                counted("pWaitDstStageMask", "const uint32_t*", "waitSemaphoreCount"),
            ],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains(
            "pub fn wait_semaphores(mut self, value: &'a [Semaphore]) -> Self { if !self.p_wait_dst_stage_mask.is_null() { assert_eq!(self.wait_semaphore_count as usize, value.len(),"
        ));
        assert!(out.contains(
            "pub fn wait_dst_stage_mask(mut self, value: &'a [u32]) -> Self { if !self.p_wait_semaphores.is_null() { assert_eq!("
        ));
    }

    #[test]
    fn string_arrays_use_lifetime_carrying_pointers() {
        let mut ir = registry();
        let mut names = counted(
            "ppEnabledExtensionNames",
            "const char* const*",
            "enabledExtensionCount",
        );
        names.null_terminated = true;
        let mut name = field("pApplicationName", "const char*");
        name.null_terminated = true;
        ir.structs.push(tagged(
            "VkInstanceCreateInfo",
            "VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO",
            vec![field("enabledExtensionCount", "uint32_t"), names, name],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub pp_enabled_extension_names: *const *const c_char,"));
        assert!(out.contains(
            "pub fn enabled_extension_names(mut self, value: &'a [CStrPtr<'a>]) -> Self { self.enabled_extension_count = len_as_count(value.len(), \"VkInstanceCreateInfo::ppEnabledExtensionNames\", \"enabledExtensionCount\"); self.pp_enabled_extension_names = value.as_ptr().cast(); self }"
        ));
        assert!(out.contains(
            "pub fn application_name(mut self, value: &'a CStr) -> Self { self.p_application_name = value.as_ptr(); self }"
        ));
    }

    #[test]
    fn slice_setters_check_only_the_counts_narrower_than_usize() {
        let mut ir = registry();
        ir.structs.push(structure(
            "VkCountsInfo",
            vec![
                field("byteCount", "uint8_t"),
                counted("pBytes", "const uint8_t*", "byteCount"),
                field("wordCount", "uint32_t"),
                counted("pWords", "const uint32_t*", "wordCount"),
                field("dataSize", "size_t"),
                counted("pData", "const void*", "dataSize"),
                field("messageSize", "uint64_t"),
                counted("pMessage", "const void*", "messageSize"),
            ],
        ));
        let model = Model::new(&ir);
        let out = squash(&emit(&model));
        for (setter, element, member, count, ty) in [
            ("bytes", "u8", "pBytes", "byteCount", "uint8_t"),
            ("words", "u32", "pWords", "wordCount", "uint32_t"),
        ] {
            assert!(
                out.contains(&format!(
                    "pub fn {setter}(mut self, value: &'a [{element}]) -> Self {{ self.{} = len_as_count(value.len(), \"VkCountsInfo::{member}\", \"{count}\");",
                    field_name(count),
                )),
                "{setter}: {out}"
            );
            let doc = crate::emit::doc_of(&emit(&model), &format!("pub fn {setter}("));
            assert!(
                doc.join(" ").contains(&format!(
                    "# Panics Panics if the slice has more elements than `{count}`, a `{ty}`, can count."
                )),
                "{setter}: {doc:?}"
            );
        }
        for (setter, count) in [("data", "data_size"), ("message", "message_size")] {
            assert!(
                out.contains(&format!(
                    "pub fn {setter}(mut self, value: &'a [u8]) -> Self {{ self.{count} = value.len() as _;"
                )),
                "{setter}: {out}"
            );
            let doc = crate::emit::doc_of(&emit(&model), &format!("pub fn {setter}("));
            assert!(!doc.join(" ").contains("# Panics"), "{setter}: {doc:?}");
        }
    }

    #[test]
    fn the_slice_setters_of_the_registry_check_every_count_narrower_than_usize() {
        let model = Model::new(vulkan_registry::Registry::get());
        let mut checked = 0;
        for s in &model.ir.structs {
            let out = squash(&definition(&model, s));
            let unchecked = out.matches("value.len() as _").count();
            let mut wide = 0;
            for m in &s.members {
                let ty = crate::model::CType::of(m);
                let count = m.len.as_deref().and_then(|len| {
                    let name = len.strip_suffix(",1").unwrap_or(len);
                    s.members.iter().find(|c| {
                        c.name == name
                            && crate::model::CType::of(c).depth() == 0
                            && !crate::model::CType::of(c).is_array()
                    })
                });
                let Some(count) = count else { continue };
                if ty.depth() == 0 || ty.is_array() {
                    continue;
                }
                if model.holds_every_slice_len(&count.ty) {
                    wide += 1;
                    continue;
                }
                checked += 1;
                assert!(
                    out.contains(&format!(
                        "= len_as_count(value.len(), \"{}::{}\", \"{}\");",
                        s.name, m.name, count.name
                    )),
                    "{}::{} sets its {} count without a check: {out}",
                    s.name,
                    m.name,
                    count.ty
                );
            }
            assert!(
                unchecked <= wide,
                "{} sets a count with `as _` that is not a size: {out}",
                s.name
            );
        }
        assert!(checked > 0, "the registry has counts narrower than usize");
    }

    #[test]
    fn pointer_arrays_borrow_their_elements() {
        let mut ir = registry();
        ir.structs
            .push(structure("VkGeometry", vec![field("pData", "const void*")]));
        ir.structs.push(tagged(
            "VkBuildInfo",
            "VK_STRUCTURE_TYPE_BUILD_INFO",
            vec![
                field("geometryCount", "uint32_t"),
                optional(counted("pGeometries", "const VkGeometry*", "geometryCount")),
                optional(counted(
                    "ppGeometries",
                    "const VkGeometry* const*",
                    "geometryCount,1",
                )),
            ],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub fn geometries(mut self, value: &'a [Geometry<'a>]) -> Self {"));
        assert!(
            out.contains(
                "pub fn geometries_ptrs(mut self, value: &'a [&'a Geometry<'a>]) -> Self {"
            )
        );
        assert!(out.contains("pub fn data(mut self, value: *const c_void) -> Self {"));
    }

    #[test]
    fn opaque_platform_pointers_stay_raw() {
        let mut ir = registry();
        ir.structs.push(tagged(
            "VkWaylandSurfaceCreateInfoKHR",
            "VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR",
            vec![field("display", "struct wl_display*")],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub display: *mut wl_display,"));
        assert!(out.contains("pub fn display(mut self, value: *mut wl_display) -> Self {"));
        assert!(!out.contains("&'a mut wl_display"));
    }

    #[test]
    fn push_next_overwrites_instead_of_walking_the_chain() {
        let mut ir = registry();
        ir.structs
            .push(tagged("VkRoot", "VK_STRUCTURE_TYPE_ROOT", vec![]));
        let mut ext = tagged("VkExt", "VK_STRUCTURE_TYPE_EXT", vec![]);
        ext.extends = vec!["VkRoot".to_string()];
        ir.structs.push(ext);
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains(
            "pub fn push_next<T: Extends<Self> + TaggedStructure>(mut self, next: &'a mut T) -> Self { self.p_next = push_chain(next, self.p_next); self }"
        ));
        assert!(out.contains("unsafe impl Extends<Root<'_>> for Ext<'_> {}"));
        assert!(!out.contains("while"));
    }

    #[test]
    fn bit_fields_are_packed_into_storage_units() {
        let mut ir = registry();
        ir.bitmasks.push(Bitmask {
            name: "VkGeometryInstanceFlagBitsKHR".to_string(),
            flags_name: "VkGeometryInstanceFlagsKHR".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        ir.flags.push(Flags {
            name: "VkGeometryInstanceFlagsKHR".to_string(),
            bit_width: 32,
            ..Default::default()
        });
        let bits = |name: &str, ty: &str, width: u32| {
            let mut f = field(name, ty);
            f.bit_field_width = Some(width);
            f
        };
        ir.structs.push(structure(
            "VkAccelerationStructureInstanceKHR",
            vec![
                bits("instanceCustomIndex", "uint32_t", 24),
                bits("mask", "uint32_t", 8),
                bits("instanceShaderBindingTableRecordOffset", "uint32_t", 24),
                bits("flags", "VkGeometryInstanceFlagsKHR", 8),
                field("accelerationStructureReference", "uint64_t"),
            ],
        ));
        let model = Model::new(&ir);
        let s = model
            .get_struct("VkAccelerationStructureInstanceKHR")
            .unwrap();
        let slots = slots(&model, s);
        assert_eq!(slots.len(), 3);
        let out = squash(&emit(&model));
        assert!(out.contains("pub bitfield0: u32,"));
        assert!(out.contains("pub bitfield1: u32,"));
        assert!(out.contains(
            "pub fn mask(mut self, value: u32) -> Self { self.bitfield0 = (self.bitfield0 & !(0xff << 24)) | ((value & 0xff) << 24); self }"
        ));
        assert!(out.contains(
            "pub fn get_instance_custom_index(&self) -> u32 { (self.bitfield0) & 0xffffff }"
        ));
        assert!(out.contains(
            "pub fn flags(mut self, value: GeometryInstanceFlagsKHR) -> Self { self.bitfield1 = (self.bitfield1 & !(0xff << 24)) | ((value.as_raw() & 0xff) << 24); self }"
        ));
    }

    #[test]
    fn fixed_arrays_get_typed_readers() {
        let mut ir = registry();
        let mut name = field("deviceName", "char");
        name.fixed_size_array = vec!["256".to_string()];
        let mut types = counted("memoryTypes", "uint32_t", "memoryTypeCount");
        types.fixed_size_array = vec!["32".to_string()];
        ir.structs.push(structure(
            "VkProps",
            vec![name, field("memoryTypeCount", "uint32_t"), types],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub fn device_name_as_c_str(&self)"));
        assert!(out.contains("pub fn memory_types_as_slice(&self) -> &[u32] {"));
    }

    #[test]
    fn only_a_single_array_of_characters_is_read_as_a_c_string() {
        let mut ir = registry();
        let mut lines = field("lines", "char");
        lines.fixed_size_array = vec!["4".to_string(), "16".to_string()];
        let mut names = field("names", "const char*");
        names.fixed_size_array = vec!["4".to_string()];
        let mut name = field("name", "char");
        name.fixed_size_array = vec!["16".to_string()];
        ir.structs
            .push(structure("VkLabels", vec![lines, names, name]));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub lines: [[c_char; 16]; 4],"), "{out}");
        assert!(out.contains("pub names: [*const c_char; 4],"), "{out}");
        assert!(out.contains("pub fn name_as_c_str(&self)"), "{out}");
        assert_eq!(out.matches("_as_c_str").count(), 1, "{out}");
        assert_eq!(out.matches("_as_slice").count(), 0, "{out}");
    }

    #[test]
    fn only_a_const_pointer_to_characters_that_end_with_nul_is_set_from_a_c_string() {
        let mut ir = registry();
        let mut name = field("pName", "const char*");
        name.null_terminated = true;
        let mut list = field("pList", "const uint32_t*");
        list.null_terminated = true;
        let mut buffer = field("pBuffer", "char*");
        buffer.null_terminated = true;
        ir.structs.push(structure(
            "VkNames",
            vec![name, field("pLetter", "const char*"), list, buffer],
        ));
        let out = squash(&emit(&Model::new(&ir)));
        assert!(
            out.contains(
                "pub fn name(mut self, value: &'a CStr) -> Self { self.p_name = value.as_ptr(); self }"
            ),
            "{out}"
        );
        assert_eq!(out.matches("&'a CStr").count(), 1, "{out}");
        for setter in ["pub fn letter(", "pub fn list(", "pub fn buffer("] {
            assert!(out.contains(setter), "no `{setter}` in {out}");
        }
    }

    #[test]
    fn unions_with_pointers_borrow_in_constructors() {
        let mut ir = registry();
        let mut u = structure(
            "VkDeviceOrHostAddressConstKHR",
            vec![
                field("deviceAddress", "VkDeviceAddress"),
                field("hostAddress", "const void*"),
            ],
        );
        u.union = true;
        ir.structs.push(u);
        ir.structs
            .push(structure("VkImageInfo", vec![field("layout", "uint32_t")]));
        let mut d = structure(
            "VkDescriptorDataEXT",
            vec![field("pSampledImage", "const VkImageInfo*")],
        );
        d.union = true;
        ir.structs.push(d);
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains("pub union DeviceOrHostAddressConstKHR<'a> {"));
        assert!(out.contains(
            "pub fn host_address(value: *const c_void) -> Self { let mut union = Self::default(); union.host_address = value; union }"
        ));
        assert!(out.contains(
            "pub fn sampled_image(value: &'a ImageInfo) -> Self { let mut union = Self::default(); union.p_sampled_image = core::ptr::from_ref(value); union }"
        ));
    }
}
