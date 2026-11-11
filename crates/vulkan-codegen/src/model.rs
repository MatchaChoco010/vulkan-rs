//! Registry analysis shared by all emitters: type lookup, alias resolution, C to Rust
//! type mapping, lifetime analysis and command dispatch levels.

use crate::naming::{const_name, type_name};
use std::collections::{BTreeMap, BTreeSet};
use vulkan_registry::{Command, Field, Registry, Struct};

/// Types that are declared by hand in `crates/vulkan/src/vk/mod.rs` because they come from
/// platform headers, not from the registry. A registry type outside of this list and
/// outside of the registry's own types aborts generation, so a new platform type cannot
/// slip in unnoticed.
const EXTERNAL_TYPES: &[&str] = &[
    "AHardwareBuffer",
    "ANativeWindow",
    "CAMetalLayer",
    "Display",
    "DWORD",
    "GgpFrameToken",
    "GgpStreamDescriptor",
    "HANDLE",
    "HINSTANCE",
    "HMONITOR",
    "HWND",
    "IDirectFB",
    "IDirectFBSurface",
    "IOSurfaceRef",
    "LPCWSTR",
    "MTLBuffer_id",
    "MTLCommandQueue_id",
    "MTLDevice_id",
    "MTLSharedEvent_id",
    "MTLTexture_id",
    "NvSciBufAttrList",
    "NvSciBufObj",
    "NvSciSyncAttrList",
    "NvSciSyncFence",
    "NvSciSyncObj",
    "OH_NativeBuffer",
    "OHBufferHandle",
    "OHNativeWindow",
    "RROutput",
    "SECURITY_ATTRIBUTES",
    "VisualID",
    "Window",
    "_screen_buffer",
    "_screen_context",
    "_screen_window",
    "ubm_device",
    "ubm_surface",
    "wl_display",
    "wl_surface",
    "xcb_connection_t",
    "xcb_visualid_t",
    "xcb_window_t",
    "zx_handle_t",
];

/// Where a command's function pointer is loaded from.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Dispatch {
    /// Global commands, loaded with a null instance.
    Entry,
    /// Loaded with `vkGetInstanceProcAddr(instance, ..)`.
    Instance,
    /// Loaded with `vkGetDeviceProcAddr(device, ..)`.
    Device,
}

impl Dispatch {
    pub const ALL: [Dispatch; 3] = [Dispatch::Entry, Dispatch::Instance, Dispatch::Device];

    pub fn table(self) -> &'static str {
        match self {
            Dispatch::Entry => "EntryFn",
            Dispatch::Instance => "InstanceFn",
            Dispatch::Device => "DeviceFn",
        }
    }

    pub fn wrapper(self) -> &'static str {
        match self {
            Dispatch::Entry => "crate::Entry",
            Dispatch::Instance => "crate::Instance",
            Dispatch::Device => "crate::Device",
        }
    }

    /// The handle that the wrapper type owns and passes implicitly.
    pub fn self_handle(self) -> Option<&'static str> {
        match self {
            Dispatch::Entry => None,
            Dispatch::Instance => Some("VkInstance"),
            Dispatch::Device => Some("VkDevice"),
        }
    }
}

/// A C type as it appears in a member or parameter declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CType {
    /// Base type name, e.g. `VkBuffer`, `char`, `wl_display`.
    pub base: String,
    /// One entry per pointer level, innermost first. `true` when the pointee is `const`.
    pub pointers: Vec<bool>,
    /// Fixed-size array dimensions, outermost first.
    pub arrays: Vec<String>,
}

impl CType {
    pub fn parse(full_ty: &str, fixed_size_array: &[String]) -> Self {
        let parts = full_ty.split('*').collect::<Vec<_>>();
        let base = parts[0]
            .split_whitespace()
            .filter(|token| *token != "const" && *token != "struct")
            .collect::<Vec<_>>()
            .join(" ");
        let pointers = parts[..parts.len() - 1]
            .iter()
            .map(|part| part.split_whitespace().any(|token| token == "const"))
            .collect();
        Self {
            base,
            pointers,
            arrays: fixed_size_array.to_vec(),
        }
    }

    pub fn of(field: &Field) -> Self {
        Self::parse(&field.full_ty, &field.fixed_size_array)
    }

    pub fn depth(&self) -> usize {
        self.pointers.len()
    }

    /// Constness of the outermost pointee, i.e. what `*const` / `*mut` of the whole type is.
    pub fn outer_const(&self) -> bool {
        self.pointers.last().copied().unwrap_or(false)
    }

    /// `const` qualifier of the base type (meaningful for arrays passed as parameters).
    pub fn base_const(full_ty: &str) -> bool {
        full_ty
            .split('*')
            .next()
            .is_some_and(|part| part.split_whitespace().any(|token| token == "const"))
    }
}

pub struct Model<'a> {
    pub ir: &'a Registry,
    pub structs: BTreeMap<&'a str, &'a Struct>,
    handles: BTreeSet<&'a str>,
    enums: BTreeSet<&'a str>,
    bitmasks: BTreeSet<&'a str>,
    flags: BTreeSet<&'a str>,
    func_pointers: BTreeSet<&'a str>,
    constants: BTreeSet<&'a str>,
    aliases: BTreeMap<&'a str, &'a str>,
    lifetimes: BTreeSet<&'a str>,
    plain_eq: BTreeSet<&'a str>,
    plain_hash: BTreeSet<&'a str>,
}

impl<'a> Model<'a> {
    pub fn new(ir: &'a Registry) -> Self {
        let mut aliases = BTreeMap::new();
        macro_rules! index {
            ($items:expr) => {{
                let mut set = BTreeSet::new();
                for item in $items {
                    set.insert(item.name.as_str());
                    for alias in &item.aliases {
                        aliases.insert(alias.as_str(), item.name.as_str());
                    }
                }
                set
            }};
        }
        let handles = index!(&ir.handles);
        let enums = index!(&ir.enums);
        let bitmasks = index!(&ir.bitmasks);
        let flags = index!(&ir.flags);
        let _ = index!(&ir.structs);
        let structs = ir.structs.iter().map(|s| (s.name.as_str(), s)).collect();
        let func_pointers = ir.func_pointers.iter().map(|f| f.name.as_str()).collect();
        let constants = ir.constants.iter().map(|c| c.name.as_str()).collect();

        let mut model = Self {
            ir,
            structs,
            handles,
            enums,
            bitmasks,
            flags,
            func_pointers,
            constants,
            aliases,
            lifetimes: BTreeSet::new(),
            plain_eq: BTreeSet::new(),
            plain_hash: BTreeSet::new(),
        };
        model.lifetimes = model.compute_lifetimes();
        (model.plain_eq, model.plain_hash) = model.compute_plain();
        model
    }

    pub fn canonical<'b>(&'b self, name: &'b str) -> &'b str {
        self.aliases.get(name).copied().unwrap_or(name)
    }

    pub fn get_struct(&self, name: &str) -> Option<&'a Struct> {
        self.structs.get(self.canonical(name)).copied()
    }

    pub fn is_handle(&self, name: &str) -> bool {
        self.handles.contains(self.canonical(name))
    }

    pub fn is_enum(&self, name: &str) -> bool {
        self.enums.contains(self.canonical(name))
    }

    pub fn is_bitmask(&self, name: &str) -> bool {
        self.bitmasks.contains(self.canonical(name))
    }

    pub fn is_flags(&self, name: &str) -> bool {
        self.flags.contains(self.canonical(name))
    }

    pub fn is_func_pointer(&self, name: &str) -> bool {
        self.func_pointers.contains(name) || name.starts_with("PFN_")
    }

    /// Opaque platform types that Rust must never form a reference to.
    pub fn is_external(&self, name: &str) -> bool {
        EXTERNAL_TYPES.contains(&name)
    }

    /// A struct needs a lifetime parameter when it can hold a borrowed pointer, directly or
    /// through a nested struct or union.
    pub fn has_lifetime(&self, name: &str) -> bool {
        self.lifetimes.contains(self.canonical(name))
    }

    /// Structs made only of plain values: integers, floats, enums, flags and handles.
    pub fn derives_partial_eq(&self, name: &str) -> bool {
        self.plain_eq.contains(self.canonical(name))
    }

    /// Like [`Self::derives_partial_eq`], and without floats.
    pub fn derives_eq_hash(&self, name: &str) -> bool {
        self.plain_hash.contains(self.canonical(name))
    }

    pub fn has_p_next(&self, s: &Struct) -> bool {
        s.members.iter().any(|m| m.name == "pNext")
    }

    fn compute_lifetimes(&self) -> BTreeSet<&'a str> {
        let mut set = BTreeSet::new();
        loop {
            let mut changed = false;
            for s in &self.ir.structs {
                if set.contains(s.name.as_str()) {
                    continue;
                }
                let needs = s.members.iter().any(|m| {
                    let ty = CType::of(m);
                    (ty.depth() > 0 && !self.is_func_pointer(&m.ty))
                        || set.contains(self.canonical(&m.ty))
                });
                if needs {
                    set.insert(s.name.as_str());
                    changed = true;
                }
            }
            if !changed {
                return set;
            }
        }
    }

    fn compute_plain(&self) -> (BTreeSet<&'a str>, BTreeSet<&'a str>) {
        // Start from "everything is plain" and remove offenders until stable.
        let candidates = || {
            self.ir
                .structs
                .iter()
                .filter(|s| !s.union)
                .map(|s| s.name.as_str())
                .collect::<BTreeSet<_>>()
        };
        let shrink = |allow_float: bool| {
            let mut set = candidates();
            loop {
                let keep = set
                    .iter()
                    .copied()
                    .filter(|name| {
                        self.structs[name].members.iter().all(|m| {
                            let ty = CType::of(m);
                            if ty.depth() > 0 || self.is_func_pointer(&m.ty) {
                                return false;
                            }
                            let base = self.canonical(&m.ty);
                            if let Some(nested) = self.structs.get(base) {
                                return !nested.union && set.contains(base);
                            }
                            // Platform typedefs may be raw pointers.
                            if self.is_external(base) || base == "VkRemoteAddressNV" {
                                return false;
                            }
                            allow_float || !matches!(base, "float" | "double")
                        })
                    })
                    .collect::<BTreeSet<_>>();
                if keep.len() == set.len() {
                    return set;
                }
                set = keep;
            }
        };
        (shrink(true), shrink(false))
    }

    /// Rust type for a base C type. `lifetime` is what to write for struct lifetimes
    /// (`'a`, `'_` or `'static`).
    pub fn base_ty(&self, name: &str, lifetime: &str) -> String {
        let name = self.canonical(name);
        match name {
            "void" => "c_void".to_string(),
            "char" => "c_char".to_string(),
            "float" => "f32".to_string(),
            "double" => "f64".to_string(),
            "int" => "core::ffi::c_int".to_string(),
            "uint8_t" => "u8".to_string(),
            "int8_t" => "i8".to_string(),
            "uint16_t" => "u16".to_string(),
            "int16_t" => "i16".to_string(),
            "uint32_t" => "u32".to_string(),
            "int32_t" => "i32".to_string(),
            "uint64_t" => "u64".to_string(),
            "int64_t" => "i64".to_string(),
            "size_t" => "usize".to_string(),
            "VkBool32" | "VkFlags" | "VkFlags64" | "VkDeviceSize" | "VkDeviceAddress"
            | "VkSampleMask" | "VkRemoteAddressNV" => type_name(name),
            _ if self.is_func_pointer(name) => name.to_string(),
            _ if self.structs.contains_key(name) => {
                if self.has_lifetime(name) {
                    format!("{}<{lifetime}>", type_name(name))
                } else {
                    type_name(name)
                }
            }
            _ if self.handles.contains(name)
                || self.enums.contains(name)
                || self.bitmasks.contains(name)
                || self.flags.contains(name) =>
            {
                type_name(name)
            }
            _ if self.is_external(name) => name.to_string(),
            _ => panic!(
                "unknown C type `{name}`: declare it in crates/vulkan/src/vk/mod.rs and add \
                 it to EXTERNAL_TYPES in vulkan-codegen"
            ),
        }
    }

    /// Rust type of a struct member as stored in the `#[repr(C)]` struct.
    pub fn member_ty(&self, m: &Field, lifetime: &str) -> String {
        let ty = CType::of(m);
        let mut out = self.pointer_ty(&ty, lifetime);
        for len in ty.arrays.iter().rev() {
            out = format!("[{out}; {}]", self.array_len(len));
        }
        out
    }

    /// Rust type of a command parameter in the C ABI. Arrays decay to pointers.
    pub fn param_ty(&self, p: &Field, lifetime: &str) -> String {
        let ty = CType::of(p);
        let mut out = self.pointer_ty(&ty, lifetime);
        if !ty.arrays.is_empty() {
            for len in ty.arrays.iter().rev() {
                out = format!("[{out}; {}]", self.array_len(len));
            }
            let ptr = if CType::base_const(&p.full_ty) {
                "*const"
            } else {
                "*mut"
            };
            out = format!("{ptr} {out}");
        }
        out
    }

    /// Rust type for `full_ty` without arrays, e.g. `*const *const c_char`.
    pub fn pointer_ty(&self, ty: &CType, lifetime: &str) -> String {
        let mut out = self.base_ty(&ty.base, lifetime);
        for is_const in &ty.pointers {
            out = format!("{} {out}", if *is_const { "*const" } else { "*mut" });
        }
        out
    }

    /// Rust type for a C return type such as `void`, `VkResult` or `void*`.
    pub fn return_ty(&self, name: &str) -> String {
        if name == "void" {
            return "()".to_string();
        }
        let ty = CType::parse(name, &[]);
        self.pointer_ty(&ty, "'_")
    }

    pub fn array_len(&self, value: &str) -> String {
        if value.chars().all(|c| c.is_ascii_digit()) {
            value.to_string()
        } else {
            assert!(
                self.constants.contains(value),
                "array length `{value}` is not a registry constant"
            );
            const_name(value)
        }
    }

    /// Constants that are used as array lengths are emitted as `usize`.
    pub fn array_len_constants(&self) -> BTreeSet<&'a str> {
        let mut names = BTreeSet::new();
        for s in &self.ir.structs {
            for m in &s.members {
                for len in &m.fixed_size_array {
                    if let Some(name) = self.constants.get(len.as_str()) {
                        names.insert(*name);
                    }
                }
            }
        }
        for c in &self.ir.commands {
            for p in &c.params {
                for len in &p.fixed_size_array {
                    if let Some(name) = self.constants.get(len.as_str()) {
                        names.insert(*name);
                    }
                }
            }
        }
        names
    }

    pub fn dispatch(&self, c: &Command) -> Dispatch {
        // vkGetDeviceProcAddr takes a VkDevice but is the bootstrap for the device table,
        // so it has to come from vkGetInstanceProcAddr.
        if c.name == "vkGetDeviceProcAddr" {
            return Dispatch::Instance;
        }
        if c.name == "vkGetInstanceProcAddr" {
            return Dispatch::Entry;
        }
        let first_is_handle = c
            .params
            .first()
            .is_some_and(|p| CType::of(p).depth() == 0 && self.is_handle(&p.ty));
        if !first_is_handle {
            return Dispatch::Entry;
        }
        if c.device {
            Dispatch::Device
        } else {
            assert!(
                c.instance,
                "{} is neither instance nor device level",
                c.name
            );
            Dispatch::Instance
        }
    }

    /// Cargo feature for a platform protect macro such as `VK_USE_PLATFORM_WAYLAND_KHR`.
    pub fn feature(&self, protect: &str) -> String {
        if protect == "VK_ENABLE_BETA_EXTENSIONS" {
            return "beta".to_string();
        }
        self.ir
            .platforms
            .iter()
            .find(|(_, macro_name)| macro_name.as_str() == protect)
            .map(|(platform, _)| platform.replace('_', "-"))
            .unwrap_or_else(|| panic!("unhandled Vulkan protect macro: {protect}"))
    }

    /// All Cargo features that gate generated items, sorted.
    pub fn features(&self) -> Vec<String> {
        let mut features = self
            .ir
            .platforms
            .values()
            .map(|protect| self.feature(protect))
            .collect::<Vec<_>>();
        features.sort();
        features.dedup();
        features
    }

    pub fn cfg(&self, protect: Option<&str>) -> String {
        match protect {
            Some(protect) => format!("#[cfg(feature = \"{}\")]\n", self.feature(protect)),
            None => String::new(),
        }
    }
}

/// Short reference line for doc comments.
pub fn provided_by(extensions: &[String], version: Option<&str>) -> String {
    let mut parts = Vec::new();
    if let Some(version) = version {
        parts.push(format!("`{version}`"));
    }
    parts.extend(extensions.iter().map(|e| format!("`{e}`")));
    if parts.is_empty() {
        String::new()
    } else {
        format!("Provided by {}.", parts.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{command, field, registry, structure};

    #[test]
    fn parses_pointer_levels_and_constness() {
        let ty = CType::parse("const char* const*", &[]);
        assert_eq!(ty.base, "char");
        assert_eq!(ty.pointers, vec![true, true]);

        let ty = CType::parse("void**", &[]);
        assert_eq!(ty.base, "void");
        assert_eq!(ty.pointers, vec![false, false]);

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
    fn dispatch_follows_the_registry_level_not_the_first_parameter_name() {
        let mut ir = registry();
        ir.handles.push(vulkan_registry::Handle {
            name: "VkExternalComputeQueueNV".to_string(),
            ..Default::default()
        });
        ir.handles.push(vulkan_registry::Handle {
            name: "VkDevice".to_string(),
            ..Default::default()
        });
        let mut queue_data = command(
            "vkGetExternalComputeQueueDataNV",
            vec![field("externalQueue", "VkExternalComputeQueueNV")],
        );
        queue_data.device = true;
        let mut gdpa = command("vkGetDeviceProcAddr", vec![field("device", "VkDevice")]);
        gdpa.device = true;
        let mut create_instance = command(
            "vkCreateInstance",
            vec![field("pCreateInfo", "const VkInstanceCreateInfo*")],
        );
        create_instance.instance = true;
        let model = Model::new(&ir);
        assert_eq!(model.dispatch(&queue_data), Dispatch::Device);
        assert_eq!(model.dispatch(&gdpa), Dispatch::Instance);
        assert_eq!(model.dispatch(&create_instance), Dispatch::Entry);
    }

    #[test]
    fn features_come_from_registry_platforms() {
        let mut ir = registry();
        ir.platforms.insert(
            "xlib_xrandr".to_string(),
            "VK_USE_PLATFORM_XLIB_XRANDR_EXT".to_string(),
        );
        let model = Model::new(&ir);
        assert_eq!(
            model.feature("VK_USE_PLATFORM_XLIB_XRANDR_EXT"),
            "xlib-xrandr"
        );
        assert_eq!(model.feature("VK_ENABLE_BETA_EXTENSIONS"), "beta");
        assert_eq!(
            model.cfg(Some("VK_ENABLE_BETA_EXTENSIONS")),
            "#[cfg(feature = \"beta\")]\n"
        );
    }
}
