mod dispatch_tables;
mod inputs;
mod shape;
mod wrapper;

use super::func_pointers::parameter_items;
use super::{arrow, assert_distinct_names, header, parameter_names, w};
use crate::docs::{Block, Doc};
use crate::model::{CType, Cfg, Dispatch, Model, VoidArrayLength};
use crate::model::{void_array_len, void_array_length};
use crate::naming::{command_name, field_name, pointer_setter_name};
use std::collections::BTreeSet;
use vulkan_registry::{Command, Field};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&["too_many_arguments", "manual_div_ceil"]);
    out.push_str("use alloc::vec::Vec;\nuse core::ffi::{c_char, c_void, CStr};\n\n");
    for dispatch in Dispatch::ALL {
        out.push_str(&dispatch_tables::table(model, dispatch));
    }
    for c in &model.ir.commands {
        assert_void_arrays_are_counted_in_bytes(c);
    }
    for dispatch in Dispatch::ALL {
        let raw_methods = commands(model, dispatch).map(|c| {
            (
                format!("the raw method of the C command `{}`", c.name),
                raw_name(c),
            )
        });
        let wrapper_methods = commands(model, dispatch)
            .filter(|c| !creates_a_dispatching_handle(c))
            .map(|c| {
                (
                    format!("the wrapper of the C command `{}`", c.name),
                    command_name(&c.name),
                )
            });
        assert_distinct_names(&dispatch.wrapper(), raw_methods.chain(wrapper_methods));
        let raw_commands = commands(model, dispatch)
            .map(|c| raw_command(model, c, dispatch))
            .collect::<String>();
        let wrappers = commands(model, dispatch)
            .filter(|c| !creates_a_dispatching_handle(c))
            .map(|c| wrapper::wrapper(model, c, dispatch))
            .collect::<String>();
        w!(
            out,
            "impl {owner} {{
                {raw_commands}}}

            impl {owner} {{
                {wrappers}}}\n",
            owner = dispatch.wrapper()
        );
    }
    out
}

fn assert_void_arrays_are_counted_in_bytes(c: &Command) {
    for p in c.params.iter().filter(|p| void_array_len(p).is_some()) {
        assert_eq!(
            void_array_length(&c.name, p),
            VoidArrayLength::Bytes,
            "{}: the wrappers pass the `void` array {} as bytes, and the specification counts it in values",
            c.name,
            p.name
        );
    }
}

fn commands<'a>(model: &'a Model<'_>, dispatch: Dispatch) -> impl Iterator<Item = &'a Command> {
    model
        .ir
        .commands
        .iter()
        .filter(move |c| model.dispatch(c) == dispatch)
}

fn raw_name(c: &Command) -> String {
    format!("{}_raw", command_name(&c.name).trim_start_matches("r#"))
}

fn called(model: &Model<'_>, cfg: &Cfg, c: &Command) -> String {
    let alias = match &c.alias {
        Some(alias) => format!(", an alias of {},", model.link(cfg, alias)),
        None => String::new(),
    };
    format!("the C command {}{alias}", model.reference(&c.name))
}

fn provided(model: &Model<'_>, cfg: &Cfg, c: &Command) -> String {
    format!(
        "The command is provided by {}.",
        model.provenance(cfg, c.version.as_deref(), &c.extensions, None)
    )
}

fn not_loaded(c: &Command) -> String {
    format!(
        "Panics if `{}` is not loaded: the loader returned no pointer for it, for example because the version or the extension that provides it is not enabled.",
        c.name
    )
}

fn raw_doc(model: &Model<'_>, cfg: &Cfg, c: &Command, params: &[Field]) -> Doc {
    let parameters = parameter_items(model, cfg, &c.name, params.iter().map(|p| p.name.as_str()));
    let mut doc = Doc::new()
        .text(model.summary(cfg, &c.name).unwrap_or_default())
        .text(format!(
            "Calls {} with the arguments as they are. {}",
            called(model, cfg, c),
            provided(model, cfg, c)
        ));
    if !parameters.is_empty() {
        doc = doc.blocks([Block::Heading("Parameters"), Block::List(parameters)]);
    }
    doc.section(
        "Safety",
        with_deferred_safety(
            model,
            c,
            format!(
                "The call has to satisfy the valid usage that the Vulkan specification states for {}.",
                model.reference(&c.name)
            ),
        ),
    )
    .section("Panics", not_loaded(c))
}

fn with_deferred_safety(model: &Model<'_>, c: &Command, safety: String) -> String {
    if can_defer(model, c) {
        format!("{safety} {DEFERRED_SAFETY}")
    } else {
        safety
    }
}

fn raw_command(model: &Model<'_>, c: &Command, dispatch: Dispatch) -> String {
    let params = dispatch.explicit_params(c);
    let names = parameter_names(&c.name, params.iter().map(|p| p.name.as_str()), field_name);
    let signature = params
        .iter()
        .zip(&names)
        .map(|(p, name)| format!(", {name}: {}", model.param_ty(p, "'_")))
        .collect::<String>();
    let self_handle = dispatch
        .passes_self_handle(c)
        .then(|| "self.handle()".to_string());
    let arguments = self_handle
        .into_iter()
        .chain(names)
        .collect::<Vec<_>>()
        .join(", ");
    let cfg = model.cfg(c.protect.as_deref());
    format!(
        "{doc}{cfg}pub unsafe fn {raw_name}(&self{signature}){arrow} {{
            unsafe {{ (self.fp().{pointer}.expect(\"{c_name} is not loaded\"))({arguments}) }}
        }}\n",
        doc = raw_doc(model, &cfg, c, params).render(),
        raw_name = raw_name(c),
        arrow = arrow(&model.return_ty(&c.return_ty)),
        pointer = command_name(&c.name),
        c_name = c.name,
    )
}

fn argument_name(p: &Field) -> String {
    if CType::of(p).depth() > 0 {
        pointer_setter_name(&p.name)
    } else {
        field_name(&p.name)
    }
}

fn struct_member_len(len: Option<&str>) -> Option<(&str, &str)> {
    let len = len?;
    len.split_once("->").or_else(|| len.split_once("::"))
}

fn value_count_param(params: &[Field], p: &Field) -> Option<usize> {
    let len = p.len.as_deref()?;
    params
        .iter()
        .position(|c| c.name == len && CType::of(c).depth() == 0 && !CType::of(c).is_array())
}

fn inner_len(
    model: &Model<'_>,
    c: &Command,
    params: &[Field],
    p: &Field,
) -> Option<(usize, String)> {
    let ty = CType::of(p);
    let count = value_count_param(params, p)?;
    if ty.pointers != [true, true] || ty.is_array() {
        return None;
    }
    assert!(
        !p.optional,
        "{}: {} is an optional array of arrays",
        c.name, p.name
    );
    let len = described_inner_len(model, c, params, p, count);
    Some(len.unwrap_or_else(|| {
        panic!(
            "{}: the specification describes no inner length for {}, the length of each array that it points to",
            c.name, p.name
        )
    }))
}

fn described_inner_len(
    model: &Model<'_>,
    c: &Command,
    params: &[Field],
    p: &Field,
    count: usize,
) -> Option<(usize, String)> {
    let description = &model
        .page(&c.name)?
        .items
        .iter()
        .find(|item| item.name == p.name)?
        .text;
    let mut lens = description
        .split("pname:")
        .zip(description.split("pname:").skip(1))
        .filter_map(|(before, member)| {
            let owner = before.strip_suffix("[i].")?;
            let member = member.split(|c: char| !c.is_ascii_alphanumeric()).next()?;
            let index = params.iter().position(|q| q.name == owner)?;
            let owner = &params[index];
            let owner_ty = CType::of(owner);
            let counted_alike = owner_ty.depth() == 1
                && !owner.optional
                && value_count_param(params, owner) == Some(count);
            let element = model.get_struct(&owner_ty.base)?;
            let counts = element.members.iter().any(|m| {
                let m_ty = CType::of(m);
                m.name == member
                    && m_ty.depth() == 0
                    && !m_ty.is_array()
                    && matches!(model.canonical(&m.ty), "uint32_t" | "size_t")
            });
            (counted_alike && counts).then(|| (index, member.to_string()))
        })
        .collect::<BTreeSet<_>>();
    if lens.len() == 1 {
        lens.pop_first()
    } else {
        None
    }
}

fn can_defer(model: &Model<'_>, c: &Command) -> bool {
    c.success_codes
        .iter()
        .any(|code| model.result_code(code) == "VK_OPERATION_DEFERRED_KHR")
}

const DEFERRED_SAFETY: &str = "If the command is deferred, that is, it returns `VK_OPERATION_DEFERRED_KHR`, the implementation can access the arguments until the deferred operation is complete. Until then, the memory that the arguments point to, including the slices and the structures and arrays that they point to, has to stay allocated and unchanged, and what the command writes must not be read; the arguments that the command requires to be externally synchronized must not be accessed; and the Vulkan objects passed to it must not be passed to any other command as externally synchronized parameters.";

fn creates_a_dispatching_handle(c: &Command) -> bool {
    c.params.last().is_some_and(|p| {
        CType::of(p).pointers == [false]
            && p.len.is_none()
            && !p.optional
            && Dispatch::ALL
                .iter()
                .any(|dispatch| dispatch.self_handle() == Some(p.ty.as_str()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::command_info::result_code;
    use crate::emit::squash;
    use crate::test_support::{
        command, counted, field, optional, registry, result_command, structure, tagged,
    };
    use std::collections::BTreeMap;
    use vulkan_registry::{DocItem, DocPage, Docs, EnumField, Handle, Registry};

    #[derive(Default)]
    struct Bindings(Vec<String>);

    impl<'ast> syn::visit::Visit<'ast> for Bindings {
        fn visit_pat_ident(&mut self, pattern: &'ast syn::PatIdent) {
            self.0.push(pattern.ident.to_string());
            syn::visit::visit_pat_ident(self, pattern);
        }
    }

    #[test]
    fn every_name_that_a_wrapper_of_the_registry_binds_is_checked_against_its_parameters() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut wrappers = 0;
        let mut binding_next_to_parameters = 0;
        for dispatch in Dispatch::ALL {
            for c in commands(&model, dispatch).filter(|c| !creates_a_dispatching_handle(c)) {
                let source = wrapper::wrapper(&model, c, dispatch);
                let function = syn::parse_str::<syn::ImplItemFn>(&source)
                    .unwrap_or_else(|error| panic!("{}: {error}", c.name));
                let mut bindings = Bindings::default();
                syn::visit::Visit::visit_block(&mut bindings, &function.block);
                let checked = wrapper::checked_names(&model, c, dispatch);
                let mut parameters = Bindings::default();
                for input in &function.sig.inputs {
                    syn::visit::Visit::visit_fn_arg(&mut parameters, input);
                }
                binding_next_to_parameters +=
                    usize::from(!bindings.0.is_empty() && !parameters.0.is_empty());
                for binding in bindings.0 {
                    assert!(
                        checked.contains(binding.trim_start_matches("r#")),
                        "{}: the wrapper binds `{binding}`, which the generator does not check \
                         against the parameters",
                        c.name
                    );
                    assert!(
                        !parameters.0.contains(&binding),
                        "{}: the wrapper binds `{binding}`, which hides its own parameter of \
                         that name",
                        c.name
                    );
                }
                wrappers += 1;
            }
        }
        assert!(wrappers > 500, "only {wrappers} wrappers were checked");
        assert!(
            binding_next_to_parameters > 100,
            "only {binding_next_to_parameters} wrappers have parameters and bind names"
        );
    }

    fn base() -> Registry {
        let mut ir = registry();
        let results = ir.enums.iter_mut().find(|e| e.name == "VkResult").unwrap();
        for (name, value, aliases) in [
            ("VK_SUCCESS", "0", &[][..]),
            ("VK_NOT_READY", "1", &[]),
            ("VK_TIMEOUT", "2", &[]),
            ("VK_INCOMPLETE", "5", &[]),
            ("VK_SUBOPTIMAL_KHR", "1000001003", &[]),
            ("VK_OPERATION_DEFERRED_KHR", "1000268002", &[]),
            ("VK_OPERATION_NOT_DEFERRED_KHR", "1000268003", &[]),
            (
                "VK_PIPELINE_COMPILE_REQUIRED",
                "1000297000",
                &[
                    "VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT",
                    "VK_PIPELINE_COMPILE_REQUIRED_EXT",
                ],
            ),
        ] {
            results.fields.push(EnumField {
                name: name.to_string(),
                aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
                value: value.to_string(),
                ..Default::default()
            });
        }
        for (name, dispatchable) in [
            ("VkInstance", true),
            ("VkPhysicalDevice", true),
            ("VkDevice", true),
            ("VkCommandBuffer", true),
            ("VkQueue", true),
            ("VkBuffer", false),
            ("VkFence", false),
            ("VkSemaphore", false),
            ("VkSwapchainKHR", false),
            ("VkPipeline", false),
            ("VkPipelineCache", false),
            ("VkSurfaceKHR", false),
            ("VkDeferredOperationKHR", false),
        ] {
            ir.handles.push(Handle {
                name: name.to_string(),
                object_type: format!("VK_OBJECT_TYPE_{}", name.to_uppercase()),
                dispatchable,
                ..Default::default()
            });
        }
        ir
    }

    fn device(mut c: Command) -> Command {
        c.device = true;
        c
    }

    fn generate(ir: &Registry) -> String {
        squash(&emit(&Model::new(ir)))
    }

    fn generate_with_docs(ir: &Registry, docs: &Docs) -> String {
        squash(&emit(&Model::with_docs(ir, docs)))
    }

    fn assert_generated_alike_under_other_names(
        ir: &Registry,
        docs: &Docs,
        real: &str,
        others: &[&str],
        expected: &str,
    ) {
        let out = generate_with_docs(ir, docs);
        assert!(
            out.contains(expected),
            "{real} is not generated as `{expected}`: {out}"
        );
        for other in others {
            let mut renamed_ir = ir.clone();
            for c in &mut renamed_ir.commands {
                if c.name == real {
                    c.name = other.to_string();
                }
            }
            let mut renamed_docs = docs.clone();
            for page in &mut renamed_docs.pages {
                if page.name == real {
                    page.name = other.to_string();
                }
            }
            assert_eq!(
                generate_with_docs(&renamed_ir, &renamed_docs),
                out.replace(real, other)
                    .replace(&command_name(real), &command_name(other)),
                "{other} carries the registry attributes of {real} but is generated differently"
            );
        }
    }

    #[test]
    fn tables_follow_dispatch_level_and_use_c_string_literals() {
        let mut ir = base();
        let mut global = result_command(
            "vkEnumerateInstanceVersion",
            &["VK_SUCCESS"],
            vec![field("pApiVersion", "uint32_t*")],
        );
        global.instance = true;
        ir.commands.push(global);
        ir.commands.push(device(command(
            "vkDeviceWaitIdle",
            vec![field("device", "VkDevice")],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub struct EntryFn { pub enumerate_instance_version: PFN_vkEnumerateInstanceVersion, }"
        ));
        assert!(
            out.contains("pub struct DeviceFn { pub device_wait_idle: PFN_vkDeviceWaitIdle, }")
        );
        assert!(out.contains(
            "device_wait_idle: crate::load_pfn(get(device, c\"vkDeviceWaitIdle\".as_ptr())),"
        ));
        assert!(out.contains(
            "pub unsafe fn enumerate_instance_version(&self) -> crate::vk::Result<u32> {"
        ));
    }

    #[test]
    fn slices_replace_pointer_and_count_and_shared_counts_are_checked() {
        let mut ir = base();
        ir.commands.push(device(command(
            "vkCmdBindVertexBuffers2",
            vec![
                field("commandBuffer", "VkCommandBuffer"),
                field("firstBinding", "uint32_t"),
                field("bindingCount", "uint32_t"),
                counted("pBuffers", "const VkBuffer*", "bindingCount"),
                counted("pOffsets", "const VkDeviceSize*", "bindingCount"),
                optional(counted("pSizes", "const VkDeviceSize*", "bindingCount")),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn cmd_bind_vertex_buffers2(&self, command_buffer: CommandBuffer, first_binding: u32, buffers: &[Buffer], offsets: &[DeviceSize], sizes: Option<&[DeviceSize]>) {"
        ));
        assert!(out.contains("let binding_count_len: usize = buffers.len();"));
        assert!(out.contains("assert_eq!(offsets.len(), binding_count_len,"));
        assert!(
            out.contains(
                "if let Some(x) = sizes.as_ref() { assert_eq!(x.len(), binding_count_len,"
            )
        );
        assert!(out.contains(
            "self.cmd_bind_vertex_buffers2_raw(command_buffer, first_binding, len_as_count(binding_count_len, \"vkCmdBindVertexBuffers2\", \"bindingCount\"), buffers.as_ptr(), offsets.as_ptr(), sizes.map_or(core::ptr::null(), |x| x.as_ptr()))"
        ));
    }

    #[test]
    fn enumerations_retry_on_incomplete() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkGetSwapchainImagesKHR",
            &["VK_SUCCESS", "VK_INCOMPLETE"],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                field("pSwapchainImageCount", "uint32_t*"),
                optional(counted(
                    "pSwapchainImages",
                    "VkBuffer*",
                    "pSwapchainImageCount",
                )),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_swapchain_images_khr(&self, swapchain: SwapchainKHR) -> crate::vk::Result<Vec<Buffer>> {"
        ));
        assert!(out.contains("if result == VkResult::INCOMPLETE { continue; }"));
        assert!(out.contains(
            "self.get_swapchain_images_khr_raw(swapchain, &mut count, core::ptr::null_mut()).into_result()?;"
        ));
    }

    #[test]
    fn acquire_next_image_never_returns_an_unwritten_index() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkAcquireNextImageKHR",
            &[
                "VK_SUCCESS",
                "VK_TIMEOUT",
                "VK_NOT_READY",
                "VK_SUBOPTIMAL_KHR",
            ],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                field("timeout", "uint64_t"),
                optional(field("semaphore", "VkSemaphore")),
                optional(field("fence", "VkFence")),
                field("pImageIndex", "uint32_t*"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains("-> crate::vk::Result<Option<(u32, bool)>> {"));
        assert!(out.contains("VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None),"));
    }

    #[test]
    fn extra_success_codes_are_returned_to_the_caller() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkWaitForFences",
            &["VK_SUCCESS", "VK_TIMEOUT"],
            vec![
                field("device", "VkDevice"),
                field("fenceCount", "uint32_t"),
                counted("pFences", "const VkFence*", "fenceCount"),
                field("waitAll", "VkBool32"),
                field("timeout", "uint64_t"),
            ],
        )));
        ir.commands.push(device(result_command(
            "vkQueuePresentKHR",
            &["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"],
            vec![field("queue", "VkQueue")],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn wait_for_fences(&self, fences: &[Fence], wait_all: bool, timeout: u64) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("wait_all as Bool32"));
        assert!(out.contains(
            "pub unsafe fn queue_present_khr(&self, queue: Queue) -> crate::vk::Result<bool> {"
        ));
    }

    #[test]
    fn pipeline_creation_keeps_partial_output_on_failure() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkGraphicsPipelineCreateInfo",
            "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO",
            vec![],
        ));
        ir.commands.push(device(result_command(
            "vkCreateGraphicsPipelines",
            &["VK_SUCCESS", "VK_PIPELINE_COMPILE_REQUIRED_EXT"],
            vec![
                field("device", "VkDevice"),
                optional(field("pipelineCache", "VkPipelineCache")),
                field("createInfoCount", "uint32_t"),
                counted(
                    "pCreateInfos",
                    "const VkGraphicsPipelineCreateInfo*",
                    "createInfoCount",
                ),
                counted("pPipelines", "VkPipeline*", "createInfoCount"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn create_graphics_pipelines(&self, pipeline_cache: PipelineCache, create_infos: &[GraphicsPipelineCreateInfo<'_>]) -> core::result::Result<Vec<Pipeline>, (Vec<Pipeline>, VkResult)> {"
        ));
        assert!(out.contains("alloc::vec![Default::default(); create_info_count_len];"));
        assert!(out.contains("Err((values, result))"));
    }

    #[test]
    fn structs_with_p_next_are_filled_in_place() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkPhysicalDeviceFeatures2",
            "VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2",
            vec![],
        ));
        let mut c = command(
            "vkGetPhysicalDeviceFeatures2",
            vec![
                field("physicalDevice", "VkPhysicalDevice"),
                field("pFeatures", "VkPhysicalDeviceFeatures2*"),
            ],
        );
        c.instance = true;
        ir.commands.push(c);
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_physical_device_features2(&self, physical_device: PhysicalDevice, features: &mut PhysicalDeviceFeatures2<'_>) {"
        ));
    }

    #[test]
    fn opaque_platform_pointers_are_never_references() {
        let mut ir = base();
        let mut c = command(
            "vkGetPhysicalDeviceWaylandPresentationSupportKHR",
            vec![
                field("physicalDevice", "VkPhysicalDevice"),
                field("queueFamilyIndex", "uint32_t"),
                field("display", "struct wl_display*"),
            ],
        );
        c.return_ty = "VkBool32".to_string();
        c.instance = true;
        ir.commands.push(c);
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_physical_device_wayland_presentation_support_khr(&self, physical_device: PhysicalDevice, queue_family_index: u32, display: *mut wl_display) -> bool {"
        ));
    }

    #[test]
    fn array_parameters_are_borrowed_arrays() {
        let mut ir = base();
        let mut blend = field("blendConstants", "const float");
        blend.fixed_size_array = vec!["4".to_string()];
        ir.commands.push(device(command(
            "vkCmdSetBlendConstants",
            vec![field("commandBuffer", "VkCommandBuffer"), blend],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn cmd_set_blend_constants_raw(&self, command_buffer: CommandBuffer, blend_constants: *const [f32; 4]) {"
        ));
        assert!(out.contains(
            "pub unsafe fn cmd_set_blend_constants(&self, command_buffer: CommandBuffer, blend_constants: &[f32; 4]) {"
        ));
    }

    #[test]
    fn output_arrays_sized_by_a_struct_member_become_vectors() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkCommandBufferAllocateInfo",
            "VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO",
            vec![field("commandBufferCount", "uint32_t")],
        ));
        ir.commands.push(device(result_command(
            "vkAllocateCommandBuffers",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("pAllocateInfo", "const VkCommandBufferAllocateInfo*"),
                counted(
                    "pCommandBuffers",
                    "VkCommandBuffer*",
                    "pAllocateInfo->commandBufferCount",
                ),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn allocate_command_buffers(&self, allocate_info: &CommandBufferAllocateInfo<'_>) -> crate::vk::Result<Vec<CommandBuffer>> {"
        ));
        assert!(out.contains(
            "alloc::vec![Default::default(); allocate_info.command_buffer_count as usize];"
        ));
    }

    #[test]
    fn arrays_of_structs_with_pointers_are_supplied_by_the_caller() {
        let mut ir = base();
        ir.structs.push(structure(
            "VkHostAddressRangeEXT",
            vec![counted("address", "void*", "size"), field("size", "size_t")],
        ));
        ir.commands.push(device(result_command(
            "vkGetImageOpaqueCaptureDataEXT",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("imageCount", "uint32_t"),
                counted("pImages", "const VkBuffer*", "imageCount"),
                counted("pDatas", "VkHostAddressRangeEXT*", "imageCount"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_image_opaque_capture_data_ext(&self, images: &[Buffer], datas: &mut [HostAddressRangeEXT<'_>]) -> crate::vk::Result<()> {"
        ));
        assert!(out.contains("assert_eq!(datas.len(), image_count_len,"));
    }

    #[test]
    fn byte_buffers_hide_their_size_parameter() {
        let mut ir = base();
        ir.commands.push(device(result_command(
            "vkGetQueryPoolResults",
            &["VK_SUCCESS", "VK_NOT_READY"],
            vec![
                field("device", "VkDevice"),
                field("dataSize", "size_t"),
                counted("pData", "void*", "dataSize"),
                field("stride", "VkDeviceSize"),
            ],
        )));
        let out = generate(&ir);
        assert!(out.contains(
            "pub unsafe fn get_query_pool_results(&self, data: &mut [u8], stride: DeviceSize) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("data_size_len, data.as_mut_ptr().cast(), stride"));
    }

    #[test]
    #[should_panic(
        expected = "vkGetBlobsNV::pData: the registry gives `blobCount` as the length of this `void` array, which is not the name of a size in bytes"
    )]
    fn void_arrays_whose_length_is_not_named_as_a_size_stop_the_generator() {
        let mut ir = base();
        ir.commands.push(device(command(
            "vkGetBlobsNV",
            vec![
                field("device", "VkDevice"),
                field("blobCount", "uint32_t"),
                counted("pData", "void*", "blobCount"),
            ],
        )));
        generate(&ir);
    }

    #[test]
    fn slices_without_a_length_parameter_are_passed_as_null_when_empty() {
        let mut ir = base();
        ir.commands.push(device(command(
            "vkCmdSetSampleMaskEXT",
            vec![
                field("commandBuffer", "VkCommandBuffer"),
                field("samples", "uint32_t"),
                optional(counted(
                    "pSampleMask",
                    "const uint32_t*",
                    "(samples + 31) / 32",
                )),
                counted("pVersionData", "const uint8_t*", "2*samples"),
                field("wordCount", "uint32_t"),
                optional(counted("pWords", "const uint32_t*", "wordCount")),
            ],
        )));
        assert_generated_alike_under_other_names(
            &ir,
            &Docs::default(),
            "vkCmdSetSampleMaskEXT",
            &["vkCmdSetMasksNV"],
            "if let Some(x) = sample_mask.as_ref() { assert!(x.is_empty() || (u128::from(samples) + 31) / 32 <= x.len() as u128, \"vkCmdSetSampleMaskEXT: `sample_mask` must be empty or have at least `(samples + 31) / 32` elements\"); } assert!(version_data.is_empty() || 2*u128::from(samples) <= version_data.len() as u128, \"vkCmdSetSampleMaskEXT: `version_data` must be empty or have at least `2*samples` elements\"); let word_count_len: usize = [words.as_ref().map(|x| x.len())].into_iter().flatten().next().unwrap_or(0); self.cmd_set_sample_mask_ext_raw(command_buffer, samples, sample_mask.map_or(core::ptr::null(), |x| if x.is_empty() { core::ptr::null() } else { x.as_ptr() }), if version_data.is_empty() { core::ptr::null() } else { version_data.as_ptr() }, len_as_count(word_count_len, \"vkCmdSetSampleMaskEXT\", \"wordCount\"), words.map_or(core::ptr::null(), |x| x.as_ptr()))",
        );
        let documented = emit(&Model::new(&ir))
            .split_whitespace()
            .filter(|word| *word != "///")
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            documented.contains("An empty `sample_mask` is passed as a null pointer. From a `sample_mask` that is not empty, the command reads `(samples + 31) / 32` elements, an expression of the registry in which each parameter stands for the number that the C API passes for it. An empty `version_data` is passed as a null pointer. From a `version_data` that is not empty, the command reads `2*samples` elements, an expression of the registry in which each parameter stands for the number that the C API passes for it."),
            "{documented}"
        );
        assert!(
            documented.contains("is not enabled. Also panics if `sample_mask` is not empty and has fewer than `(samples + 31) / 32` elements. Also panics if `version_data` is not empty and has fewer than `2*samples` elements. Also panics if `words` has more elements than `wordCount`, a `uint32_t`, can count."),
            "{documented}"
        );
        assert!(!documented.contains("An empty `words`"), "{documented}");
    }

    #[test]
    fn a_length_that_the_wrappers_cannot_evaluate_stops_the_generator() {
        for len in [
            "2*VK_UUID_SIZE",
            "samples - 1",
            "samples % 32",
            "(layers + 31) / 32",
            "pLevels + 1",
            "wordCount * 2",
            "ratio * 2",
            "latexmath:[\\lceil{samples \\over 32}\\rceil]",
        ] {
            let mut ir = base();
            ir.commands.push(device(command(
                "vkCmdSetSampleMaskEXT",
                vec![
                    field("commandBuffer", "VkCommandBuffer"),
                    field("samples", "uint32_t"),
                    field("ratio", "float"),
                    field("pLevels", "const uint32_t*"),
                    field("wordCount", "uint32_t"),
                    counted("pWords", "const uint32_t*", "wordCount"),
                    optional(counted("pSampleMask", "const uint32_t*", len)),
                ],
            )));
            let message = crate::test_support::catch_quietly(|| generate(&ir))
                .expect_err("the length cannot be evaluated");
            assert_eq!(
                message,
                format!(
                    "vkCmdSetSampleMaskEXT: the wrappers cannot evaluate `{len}`, the length that the registry gives for pSampleMask: they read integers, the names of parameters that are passed as numbers, `+`, `*`, `/` and parentheses"
                ),
                "{len}"
            );
        }
    }

    #[test]
    fn the_wrappers_of_the_registry_pass_null_only_for_empty_slices_without_a_length() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut null_when_empty = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            if out.contains(".is_empty() {") {
                null_when_empty.insert(c.name.as_str());
            }
        }
        assert_eq!(
            null_when_empty,
            BTreeSet::from(["vkCmdSetSampleMaskEXT"]),
            "crates/vulkan/tests/uncounted_slices.rs checks each of these wrappers"
        );
    }

    #[test]
    fn the_wrappers_of_the_registry_check_every_length_that_is_an_expression_of_parameters() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut checked = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            for p in model.dispatch(c).explicit_params(c) {
                let ty = CType::of(p);
                let Some(len) = p.len.as_deref() else {
                    continue;
                };
                let names_one_parameter_or_member =
                    len.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                        || struct_member_len(Some(len)).is_some();
                if ty.depth() != 1 || ty.is_array() || names_one_parameter_or_member {
                    continue;
                }
                let name = argument_name(p);
                assert!(
                    out.contains(&format!(
                        "\"{}: `{name}` must be empty or have at least `{len}` elements\"",
                        c.name
                    )),
                    "{}: the wrapper does not check that `{name}` has `{len}` elements: {out}",
                    c.name
                );
                checked.insert(format!("{}::{} [{len}]", c.name, p.name));
            }
        }
        assert_eq!(
            checked,
            BTreeSet::from(
                ["vkCmdSetSampleMaskEXT::pSampleMask [(samples + 31) / 32]".to_string()]
            ),
            "crates/vulkan/tests/computed_lengths.rs calls each of these wrappers"
        );
    }

    #[test]
    fn counts_narrower_than_usize_are_checked_and_sizes_are_not() {
        let mut ir = base();
        for (name, ty) in [
            ("vkCmdSetWordsNV", "uint32_t"),
            ("vkCmdSetBytesNV", "uint8_t"),
            ("vkCmdSetDataNV", "size_t"),
            ("vkCmdUpdateDataNV", "VkDeviceSize"),
        ] {
            ir.commands.push(device(command(
                name,
                vec![
                    field("commandBuffer", "VkCommandBuffer"),
                    field("size", ty),
                    counted("pValues", "const void*", "size"),
                ],
            )));
        }
        let out = generate(&ir);
        for (name, raw) in [
            ("vkCmdSetWordsNV", "cmd_set_words_nv_raw"),
            ("vkCmdSetBytesNV", "cmd_set_bytes_nv_raw"),
        ] {
            assert!(
                out.contains(&format!(
                    "self.{raw}(command_buffer, len_as_count(size_len, \"{name}\", \"size\"), values.as_ptr().cast())"
                )),
                "{name}: {out}"
            );
        }
        assert!(out.contains(
            "self.cmd_set_data_nv_raw(command_buffer, size_len, values.as_ptr().cast())"
        ));
        assert!(out.contains(
            "self.cmd_update_data_nv_raw(command_buffer, size_len as DeviceSize, values.as_ptr().cast())"
        ));
        let documented = emit(&Model::new(&ir))
            .split_whitespace()
            .filter(|word| *word != "///")
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(
            documented
                .matches("Also panics if `values` has more elements than `size`, a `")
                .count(),
            2,
            "{documented}"
        );
        assert!(documented.contains(
            "Also panics if `values` has more elements than `size`, a `uint32_t`, can count."
        ));
    }

    #[test]
    fn the_wrappers_of_the_registry_check_every_count_narrower_than_usize() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut checked = 0;
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            for narrow in ["u8", "u16", "u32", "i8", "i16", "i32"] {
                assert!(
                    !out.contains(&format!("_len as {narrow}")),
                    "{}: a slice length is cast to {narrow} without a check: {out}",
                    c.name
                );
            }
            checked += out.matches("len_as_count(").count();
        }
        assert!(checked > 0, "the registry has counts narrower than usize");
    }

    fn only(c: Command) -> Registry {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkPresentInfoKHR",
            "VK_STRUCTURE_TYPE_PRESENT_INFO_KHR",
            vec![],
        ));
        ir.structs.push(tagged(
            "VkGraphicsPipelineCreateInfo",
            "VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO",
            vec![],
        ));
        ir.commands.push(device(c));
        ir
    }

    #[test]
    fn reporting_suboptimal_follows_the_result_codes_not_the_command_name() {
        for codes in [
            ["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"],
            ["VK_SUBOPTIMAL_KHR", "VK_SUCCESS"],
        ] {
            let present = only(result_command(
                "vkQueuePresentKHR",
                &codes,
                vec![
                    field("queue", "VkQueue"),
                    field("pPresentInfo", "const VkPresentInfoKHR*"),
                ],
            ));
            assert_generated_alike_under_other_names(
                &present,
                &Docs::default(),
                "vkQueuePresentKHR",
                &["vkQueueShowFramesNV", "vkGetSwapchainStatusKHR"],
                "pub unsafe fn queue_present_khr(&self, queue: Queue, present_info: &PresentInfoKHR<'_>) -> crate::vk::Result<bool> { unsafe { let result = self.queue_present_khr_raw(queue, core::ptr::from_ref(present_info)); result.into_result()?; Ok(result == VkResult::SUBOPTIMAL_KHR) } }",
            );
        }
        let wait = only(result_command(
            "vkWaitForPresentKHR",
            &["VK_SUCCESS", "VK_TIMEOUT", "VK_SUBOPTIMAL_KHR"],
            vec![
                field("swapchain", "VkSwapchainKHR"),
                field("timeout", "uint64_t"),
            ],
        ));
        assert!(
            generate(&wait).contains(
                "pub unsafe fn wait_for_present_khr(&self, swapchain: SwapchainKHR, timeout: u64) -> crate::vk::Result<VkResult> {"
            ),
            "a command that can also time out reports every success code"
        );
    }

    #[test]
    fn acquiring_follows_the_result_codes_not_the_command_name() {
        let acquire = only(result_command(
            "vkAcquireNextImageKHR",
            &[
                "VK_SUCCESS",
                "VK_TIMEOUT",
                "VK_NOT_READY",
                "VK_SUBOPTIMAL_KHR",
            ],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                field("timeout", "uint64_t"),
                optional(field("semaphore", "VkSemaphore")),
                optional(field("fence", "VkFence")),
                field("pImageIndex", "uint32_t*"),
            ],
        ));
        assert_generated_alike_under_other_names(
            &acquire,
            &Docs::default(),
            "vkAcquireNextImageKHR",
            &["vkTakeNextFrameNV", "vkAcquireNextImage2KHR"],
            "-> crate::vk::Result<Option<(u32, bool)>> { unsafe { let mut value: u32 = core::mem::zeroed::<u32>(); match self.acquire_next_image_khr_raw(swapchain, timeout, semaphore, fence, &mut value) { VkResult::SUCCESS => Ok(Some((value, false))), VkResult::SUBOPTIMAL_KHR => Ok(Some((value, true))), VkResult::TIMEOUT | VkResult::NOT_READY => Ok(None), error => Err(error), } } }",
        );
    }

    fn acquiring(error_codes: &[&str]) -> Registry {
        let acquire = only(result_command(
            "vkTakeNextFrameNV",
            &[
                "VK_SUCCESS",
                "VK_TIMEOUT",
                "VK_NOT_READY",
                "VK_SUBOPTIMAL_KHR",
            ],
            vec![
                field("device", "VkDevice"),
                field("swapchain", "VkSwapchainKHR"),
                field("pImageIndex", "uint32_t*"),
            ],
        ));
        with_error_codes(acquire, error_codes)
    }

    fn section<'a>(doc: &'a [String], title: &str) -> &'a str {
        let heading = doc
            .iter()
            .position(|block| block == &format!("# {title}"))
            .unwrap_or_else(|| panic!("no section {title} in {doc:?}"));
        &doc[heading + 1]
    }

    const EVERY_OTHER_CODE_IS_AN_ERROR: &str = "Returns the result code of the command if it is any code other than [`VK_SUCCESS`](VkResult::SUCCESS), [`VK_TIMEOUT`](VkResult::TIMEOUT), [`VK_NOT_READY`](VkResult::NOT_READY) and [`VK_SUBOPTIMAL_KHR`](VkResult::SUBOPTIMAL_KHR), the success codes that the registry lists for the command, whether the code is negative or not.";

    #[test]
    fn a_wrapper_that_returns_every_unlisted_code_as_an_error_documents_it() {
        for (error_codes, listed) in [
            (&[][..], String::new()),
            (
                &["VK_ERROR_DEVICE_LOST"][..],
                " The registry lists [`VK_ERROR_DEVICE_LOST`](VkResult::ERROR_DEVICE_LOST) as the error code of the command."
                    .to_string(),
            ),
            (
                &["VK_ERROR_OUT_OF_HOST_MEMORY", "VK_ERROR_DEVICE_LOST"][..],
                " The registry lists [`VK_ERROR_OUT_OF_HOST_MEMORY`](VkResult::ERROR_OUT_OF_HOST_MEMORY) and [`VK_ERROR_DEVICE_LOST`](VkResult::ERROR_DEVICE_LOST) as the error codes of the command."
                    .to_string(),
            ),
        ] {
            let ir = acquiring(error_codes);
            let model = Model::new(&ir);
            let out = wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device);
            assert!(squash(&out).contains("error => Err(error),"), "{out}");
            let doc = crate::emit::doc_of(&out, "pub unsafe fn take_next_frame_nv(");
            assert_eq!(
                section(&doc, "Errors"),
                format!("{EVERY_OTHER_CODE_IS_AN_ERROR}{listed}"),
                "{error_codes:?}"
            );
            assert!(
                doc.iter().any(|block| block.contains(
                    "or `None` if the command returned `VK_TIMEOUT` or `VK_NOT_READY`. Any other result is returned as the error."
                )),
                "{doc:?}"
            );
        }
    }

    #[test]
    fn the_wrappers_of_the_registry_that_return_every_unlisted_code_as_an_error_document_it() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut acquiring = BTreeSet::new();
        let mut keeping_partial_output = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = wrapper::wrapper(&model, c, model.dispatch(c));
            let code = squash(&out);
            let words = out
                .replace("///", "")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");
            let errors = words
                .split_once("# Errors ")
                .map_or("", |(_, errors)| errors)
                .split(" # ")
                .next()
                .unwrap_or_default();
            let returns_every_unlisted_code = code.contains("error => Err(error)");
            let returns_every_code_but_success = code.contains(
                "if result == VkResult::SUCCESS { Ok(values) } else { Err((values, result)) }",
            );
            assert_eq!(
                returns_every_unlisted_code,
                errors.starts_with("Returns the result code of the command if it is any code other than ")
                    && errors.contains(", the success codes that the registry lists for the command, whether the code is negative or not."),
                "{}: {errors}",
                c.name
            );
            assert_eq!(
                returns_every_unlisted_code,
                words.contains("Any other result is returned as the error."),
                "{}: {words}",
                c.name
            );
            assert_eq!(
                returns_every_code_but_success,
                errors.starts_with("Returns the result code of the command, together with the elements, if it is not `VK_SUCCESS`: ")
                    && words.contains("Any result other than `VK_SUCCESS` is returned as the error, together with the same elements."),
                "{}: {words}",
                c.name
            );
            if returns_every_unlisted_code {
                for success in &c.success_codes {
                    assert!(
                        errors.contains(&format!("[`{success}`]")),
                        "{}: {success} is missing from {errors}",
                        c.name
                    );
                }
                let listed = match c.error_codes.as_slice() {
                    [] => String::new(),
                    [only] => format!(
                        " The registry lists {} as the error code of the command.",
                        model.link(&model.cfg(c.protect.as_deref()), only)
                    ),
                    [.., second_to_last, last] => format!(
                        "[`{second_to_last}`]({}) and [`{last}`]({}) as the error codes of the command.",
                        result_code(&model, second_to_last),
                        result_code(&model, last)
                    ),
                };
                assert!(
                    errors.ends_with(&listed),
                    "{}: {errors} does not end with {listed}",
                    c.name
                );
                assert!(c.error_codes.len() >= 3, "{}", c.name);
                acquiring.insert(c.name.as_str());
            }
            if returns_every_code_but_success {
                keeping_partial_output.insert(c.name.as_str());
            }
            let takes_other_codes_as_successes = !returns_every_unlisted_code
                && !returns_every_code_but_success
                && code.contains("crate::vk::Result<");
            assert_eq!(
                takes_other_codes_as_successes,
                errors.starts_with("Returns the result code of the command if it is an error"),
                "{}: {errors}",
                c.name
            );
        }
        assert_eq!(
            acquiring,
            BTreeSet::from(["vkAcquireNextImage2KHR", "vkAcquireNextImageKHR"])
        );
        assert!(keeping_partial_output.contains("vkCreateGraphicsPipelines"));
    }

    #[test]
    fn keeping_partial_output_follows_the_result_codes_not_the_command_name() {
        for code in [
            "VK_PIPELINE_COMPILE_REQUIRED",
            "VK_PIPELINE_COMPILE_REQUIRED_EXT",
            "VK_INCOMPATIBLE_SHADER_BINARY_EXT",
            "VK_TIMEOUT",
        ] {
            let create = only(result_command(
                "vkCreateGraphicsPipelines",
                &["VK_SUCCESS", code],
                vec![
                    field("device", "VkDevice"),
                    optional(field("pipelineCache", "VkPipelineCache")),
                    field("createInfoCount", "uint32_t"),
                    counted(
                        "pCreateInfos",
                        "const VkGraphicsPipelineCreateInfo*",
                        "createInfoCount",
                    ),
                    counted("pPipelines", "VkPipeline*", "createInfoCount"),
                ],
            ));
            assert_generated_alike_under_other_names(
                &create,
                &Docs::default(),
                "vkCreateGraphicsPipelines",
                &["vkCompileProgramsNV", "vkBuildShaderGroupsEXT"],
                "-> core::result::Result<Vec<Pipeline>, (Vec<Pipeline>, VkResult)> {",
            );
        }
        let other_handles = only(result_command(
            "vkCompileProgramsNV",
            &["VK_SUCCESS", "VK_INCOMPATIBLE_SHADER_BINARY_EXT"],
            vec![
                field("device", "VkDevice"),
                field("programCount", "uint32_t"),
                counted("pSources", "const VkBuffer*", "programCount"),
                counted("pPrograms", "VkFence*", "programCount"),
            ],
        ));
        assert!(generate(&other_handles).contains(
            "pub unsafe fn compile_programs_nv(&self, sources: &[Buffer]) -> core::result::Result<Vec<Fence>, (Vec<Fence>, VkResult)> {"
        ));
        let documented = emit(&Model::new(&other_handles))
            .split_whitespace()
            .filter(|word| *word != "///")
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            documented.contains("if it is not `VK_SUCCESS`: another success code, which the registry lists as `VK_INCOMPATIBLE_SHADER_BINARY_EXT`, or an error code."),
            "{documented}"
        );
        assert!(!documented.contains("if it is an error"), "{documented}");
        let without_the_code = only(result_command(
            "vkCreateComputePipelines",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("createInfoCount", "uint32_t"),
                counted("pSources", "const VkBuffer*", "createInfoCount"),
                counted("pPipelines", "VkPipeline*", "createInfoCount"),
            ],
        ));
        assert!(generate(&without_the_code).contains(
            "pub unsafe fn create_compute_pipelines(&self, sources: &[Buffer]) -> crate::vk::Result<Vec<Pipeline>> {"
        ));
    }

    #[test]
    fn commands_that_create_a_dispatching_handle_are_left_to_the_hand_written_api() {
        for (real, created) in [
            ("vkCreateInstance", "VkInstance"),
            ("vkCreateDevice", "VkDevice"),
        ] {
            let mut ir = base();
            let mut c = result_command(
                real,
                &["VK_SUCCESS"],
                vec![
                    field("physicalDevice", "VkPhysicalDevice"),
                    field("pCreated", &format!("{created}*")),
                ],
            );
            c.instance = true;
            ir.commands.push(c);
            let wrapper = format!("pub unsafe fn {}(&self", command_name(real));
            let out = generate(&ir);
            assert!(!out.contains(&wrapper), "{real} has a generated wrapper");
            assert_generated_alike_under_other_names(
                &ir,
                &Docs::default(),
                real,
                &["vkOpenSessionNV", "vkCreateInstance", "vkCreateDevice"],
                &format!("pub unsafe fn {}_raw(&self", command_name(real)),
            );
        }
    }

    fn widgets() -> (Registry, Docs) {
        let mut ir = base();
        ir.structs.push(structure(
            "VkWidgetBatchNV",
            vec![field("flags", "uint32_t"), field("partCount", "uint32_t")],
        ));
        ir.structs.push(structure(
            "VkWidgetPartNV",
            vec![field("first", "uint32_t")],
        ));
        let batches = || counted("pBatches", "const VkWidgetBatchNV*", "batchCount");
        let parts = || counted("ppParts", "const VkWidgetPartNV* const*", "batchCount");
        ir.commands.push(device(command(
            "vkCmdAssembleWidgetsNV",
            vec![
                field("commandBuffer", "VkCommandBuffer"),
                field("batchCount", "uint32_t"),
                batches(),
                counted("pFirstIndices", "const uint32_t*", "batchCount"),
                parts(),
                counted("ppLimits", "const uint32_t* const*", "batchCount"),
            ],
        )));
        ir.commands.push(device(result_command(
            "vkJoinWidgetsNV",
            &[
                "VK_SUCCESS",
                "VK_OPERATION_DEFERRED_KHR",
                "VK_OPERATION_NOT_DEFERRED_KHR",
            ],
            vec![
                field("device", "VkDevice"),
                optional(field("deferredOperation", "VkDeferredOperationKHR")),
                field("batchCount", "uint32_t"),
                batches(),
                parts(),
            ],
        )));
        let item = |name: &str, text: &str| DocItem {
            name: name.to_string(),
            text: text.to_string(),
        };
        let parts_text = "pname:ppParts is a pointer to an array of pname:batchCount\npointers to arrays of slink:VkWidgetPartNV structures.\nEach pname:ppParts[i] is a pointer to an array of\npname:pBatches[i].pname:partCount\nslink:VkWidgetPartNV structures, used with pname:pFirstIndices[i].";
        let limits_text = "pname:ppLimits is a pointer to an array of pname:batchCount\npointers to arrays of pname:pBatches[i].pname:partCount values.";
        let page = |name: &str, items: Vec<DocItem>| DocPage {
            name: name.to_string(),
            kind: "protos".to_string(),
            desc: String::new(),
            aliases: Vec::new(),
            file: String::new(),
            items,
        };
        let docs = Docs {
            pages: vec![
                page(
                    "vkCmdAssembleWidgetsNV",
                    vec![item("ppParts", parts_text), item("ppLimits", limits_text)],
                ),
                page("vkJoinWidgetsNV", vec![item("ppParts", parts_text)]),
            ],
            ..Default::default()
        };
        (ir, docs)
    }

    #[test]
    fn arrays_of_arrays_whose_inner_length_the_specification_describes_are_slices_of_slices() {
        let (ir, docs) = widgets();
        let out = generate_with_docs(&ir, &docs);
        assert!(out.contains(
            "pub unsafe fn cmd_assemble_widgets_nv(&self, command_buffer: CommandBuffer, batches: &[WidgetBatchNV], first_indices: &[u32], parts: &[Option<&[WidgetPartNV]>], limits: &[Option<&[u32]>]) {"
        ));
        assert!(out.contains("let batch_count_len: usize = batches.len();"));
        assert!(out.contains("assert_eq!(parts.len(), batch_count_len,"));
        assert!(out.contains("assert_eq!(limits.len(), batch_count_len,"));
        assert!(out.contains(
            "for (element, entry) in batches.iter().zip(parts.iter()) { if let Some(entry) = entry { assert_eq!(entry.len(), element.part_count as usize, \"vkCmdAssembleWidgetsNV: each `parts` entry that is not `None` must have `part_count` elements\"); } }"
        ));
        assert!(out.contains(
            "let parts_pointers = parts.iter().map(|entry| entry.map_or(core::ptr::null(), |entry| entry.as_ptr())).collect::<Vec<_>>();"
        ));
        assert!(out.contains(
            "self.cmd_assemble_widgets_nv_raw(command_buffer, len_as_count(batch_count_len, \"vkCmdAssembleWidgetsNV\", \"batchCount\"), batches.as_ptr(), first_indices.as_ptr(), parts_pointers.as_ptr(), limits_pointers.as_ptr())"
        ));
        assert_generated_alike_under_other_names(
            &ir,
            &docs,
            "vkCmdAssembleWidgetsNV",
            &[
                "vkCmdBuildAccelerationStructuresKHR",
                "vkCmdBuildAccelerationStructuresIndirectKHR",
            ],
            "parts: &[Option<&[WidgetPartNV]>]",
        );
    }

    const ABSENT_INNER_SLICES: &str = "Each slice of slices is passed as an array of pointers to the inner slices, with a null pointer for `None`. `None` is valid only for an element for which the valid usage of the command asks for a null pointer or allows one, which the wrapper does not check.";

    fn documented(out: &str) -> String {
        out.replace("///", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn a_wrapper_that_takes_slices_of_slices_says_when_an_inner_slice_may_be_absent() {
        let (ir, docs) = widgets();
        let model = Model::with_docs(&ir, &docs);
        let assemble = wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device);
        assert!(
            squash(&assemble).contains("parts: &[Option<&[WidgetPartNV]>]"),
            "{assemble}"
        );
        assert_eq!(
            documented(&assemble).matches(ABSENT_INNER_SLICES).count(),
            1,
            "{assemble}"
        );
        let join = wrapper::wrapper(&model, &ir.commands[1], Dispatch::Device);
        assert!(
            squash(&join).contains("parts: &[*const WidgetPartNV]"),
            "{join}"
        );
        assert!(
            !documented(&join).contains("slice of slices")
                && !documented(&join).contains("`None` is valid"),
            "a deferrable command takes the pointers as they are: {join}"
        );
    }

    #[test]
    fn the_wrappers_of_the_registry_say_when_an_inner_slice_may_be_absent() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut found = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = wrapper::wrapper(&model, c, model.dispatch(c));
            let takes_optional_inner_slices = squash(&out).contains(": &[Option<&[");
            let documented = documented(&out);
            assert_eq!(
                documented.matches(ABSENT_INNER_SLICES).count(),
                usize::from(takes_optional_inner_slices),
                "{}: {documented}",
                c.name
            );
            assert_eq!(
                documented.contains("with a null pointer for `None`"),
                takes_optional_inner_slices,
                "{}: {documented}",
                c.name
            );
            if takes_optional_inner_slices {
                found.insert(c.name.as_str());
            }
        }
        assert_eq!(
            found,
            BTreeSet::from([
                "vkCmdBuildAccelerationStructuresIndirectKHR",
                "vkCmdBuildAccelerationStructuresKHR",
            ])
        );
    }

    #[test]
    #[should_panic(
        expected = "vkCmdAssembleWidgetsNV: the specification describes no inner length for ppParts"
    )]
    fn arrays_of_arrays_without_a_described_inner_length_stop_the_generator() {
        let (ir, _) = widgets();
        generate(&ir);
    }

    #[test]
    fn inner_lengths_are_counts_not_other_integers() {
        for ty in ["VkBool32", "VkDeviceSize", "VkDeviceAddress", "uint64_t"] {
            let (mut ir, docs) = widgets();
            for s in &mut ir.structs {
                for m in &mut s.members {
                    if m.name == "partCount" {
                        m.ty = ty.to_string();
                        m.full_ty = ty.to_string();
                    }
                }
            }
            let generated = std::panic::catch_unwind(|| generate_with_docs(&ir, &docs));
            assert!(
                generated.is_err(),
                "a {ty} member is taken as an inner length"
            );
        }
        let (mut ir, docs) = widgets();
        for s in &mut ir.structs {
            for m in &mut s.members {
                if m.name == "partCount" {
                    m.ty = "size_t".to_string();
                    m.full_ty = "size_t".to_string();
                }
            }
        }
        assert!(generate_with_docs(&ir, &docs).contains("parts: &[Option<&[WidgetPartNV]>]"));
    }

    #[test]
    fn counted_arrays_of_dispatching_handles_get_wrappers() {
        let mut ir = base();
        ir.structs.push(tagged(
            "VkDeviceGroupInfoNV",
            "VK_STRUCTURE_TYPE_DEVICE_GROUP_INFO_NV",
            vec![field("deviceCount", "uint32_t")],
        ));
        let mut c = result_command(
            "vkCreateDeviceGroupNV",
            &["VK_SUCCESS"],
            vec![
                field("physicalDevice", "VkPhysicalDevice"),
                field("pInfo", "const VkDeviceGroupInfoNV*"),
                counted("pDevices", "VkDevice*", "pInfo->deviceCount"),
            ],
        );
        c.instance = true;
        ir.commands.push(c);
        assert!(generate(&ir).contains(
            "pub unsafe fn create_device_group_nv(&self, physical_device: PhysicalDevice, info: &DeviceGroupInfoNV<'_>) -> crate::vk::Result<Vec<Device>> {"
        ));
    }

    fn wrapper_shapes_of_the_embedded_registry() -> BTreeMap<&'static str, BTreeSet<String>> {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let shapes = [
            ("no generated wrapper", ""),
            (
                "reports suboptimal",
                "Ok(result == VkResult::SUBOPTIMAL_KHR)",
            ),
            ("may not write the value", "-> crate::vk::Result<Option<("),
            ("keeps partial output", "-> core::result::Result<Vec<"),
            ("takes slices of slices", ": &[Option<&["),
        ];
        let mut found = BTreeMap::<&'static str, BTreeSet<String>>::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                found
                    .entry("no generated wrapper")
                    .or_default()
                    .insert(c.name.clone());
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            for (shape, marker) in shapes {
                if !marker.is_empty() && out.contains(marker) {
                    found.entry(shape).or_default().insert(c.name.clone());
                }
            }
        }
        found
    }

    #[test]
    fn the_embedded_registry_gets_the_special_wrapper_shapes_where_expected() {
        let names = |names: &[&str]| names.iter().map(|name| name.to_string()).collect();
        let expected: BTreeMap<&'static str, BTreeSet<String>> = [
            (
                "no generated wrapper",
                names(&["vkCreateDevice", "vkCreateInstance"]),
            ),
            (
                "reports suboptimal",
                names(&["vkGetSwapchainStatusKHR", "vkQueuePresentKHR"]),
            ),
            (
                "may not write the value",
                names(&["vkAcquireNextImage2KHR", "vkAcquireNextImageKHR"]),
            ),
            (
                "keeps partial output",
                names(&[
                    "vkCreateComputePipelines",
                    "vkCreateDataGraphPipelinesARM",
                    "vkCreateExecutionGraphPipelinesAMDX",
                    "vkCreateGraphicsPipelines",
                    "vkCreateRayTracingPipelinesNV",
                    "vkCreateShadersEXT",
                ]),
            ),
            (
                "takes slices of slices",
                names(&[
                    "vkCmdBuildAccelerationStructuresIndirectKHR",
                    "vkCmdBuildAccelerationStructuresKHR",
                ]),
            ),
        ]
        .into_iter()
        .collect();
        assert_eq!(wrapper_shapes_of_the_embedded_registry(), expected);
    }

    #[test]
    fn every_counted_array_of_arrays_parameter_of_the_registry_has_an_inner_length() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut found = 0;
        for c in &model.ir.commands {
            let params = model.dispatch(c).explicit_params(c);
            for p in params {
                if p.len.is_some() && CType::of(p).depth() == 2 {
                    found += 1;
                    assert!(
                        inner_len(&model, c, params, p).is_some(),
                        "{}: {} is not an array of arrays with an inner length",
                        c.name,
                        p.name
                    );
                }
            }
        }
        assert!(found > 0, "the registry has arrays of arrays");
    }

    fn map_tokens(text: &str, map: &BTreeMap<String, String>) -> String {
        let mut out = String::with_capacity(text.len());
        let mut token = String::new();
        let flush = |token: &mut String, out: &mut String| {
            out.push_str(map.get(token.as_str()).unwrap_or(token));
            token.clear();
        };
        for c in text.chars() {
            if c.is_ascii_alphanumeric() || c == '_' {
                token.push(c);
            } else {
                flush(&mut token, &mut out);
                out.push(c);
            }
        }
        flush(&mut token, &mut out);
        out
    }

    #[test]
    fn the_embedded_registry_generates_the_same_commands_under_other_command_names() {
        let (ir, docs) = (Registry::get(), Docs::get());
        let model = Model::with_docs(ir, docs);
        let listed_by_name = |c: &Command| !model.aligned_outputs(c).is_empty();
        assert_eq!(
            ir.commands
                .iter()
                .filter(|c| listed_by_name(c))
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>(),
            ["vkGetShaderBinaryDataEXT"],
            "the commands that keep their names here, because the generator lists a requirement \
             of the specification under the name of the command"
        );
        let renamed = ir
            .commands
            .iter()
            .enumerate()
            .filter(|(_, c)| !listed_by_name(c))
            .map(|(index, c)| (c.name.clone(), format!("vkRenamedCommand{index}")))
            .collect::<BTreeMap<_, _>>();
        let rename = |name: &mut String| {
            if let Some(new) = renamed.get(name.as_str()) {
                *name = new.clone();
            }
        };
        let mut renamed_ir = ir.clone();
        for c in &mut renamed_ir.commands {
            rename(&mut c.name);
            if let Some(alias) = &mut c.alias {
                rename(alias);
            }
        }
        let mut renamed_docs = docs.clone();
        for page in &mut renamed_docs.pages {
            rename(&mut page.name);
            page.aliases.iter_mut().for_each(rename);
        }
        let mut back = BTreeMap::new();
        for (old, new) in &renamed {
            back.insert(new.clone(), old.clone());
            back.insert(format!("PFN_{new}"), format!("PFN_{old}"));
            back.insert(command_name(new), command_name(old));
            back.insert(raw_name_of(new), raw_name_of(old));
        }
        let original = squash(&emit(&Model::with_docs(ir, docs)));
        let restored = map_tokens(
            &squash(&emit(&Model::with_docs(&renamed_ir, &renamed_docs))),
            &back,
        );
        if let Some(at) = original
            .bytes()
            .zip(restored.bytes())
            .position(|(a, b)| a != b)
        {
            let context = |text: &str| {
                let start = text[..at].rfind("pub unsafe fn").unwrap_or(0);
                text[start..(at + 200).min(text.len())].to_string()
            };
            panic!(
                "renaming the commands changes the generated code:\n{}\n---\n{}",
                context(&original),
                context(&restored)
            );
        }
        assert_eq!(original.len(), restored.len());
    }

    fn raw_name_of(c_name: &str) -> String {
        raw_name(&Command {
            name: c_name.to_string(),
            ..Default::default()
        })
    }

    fn fault_reports(codes: &[&str]) -> Registry {
        let mut ir = only(result_command(
            "vkGetDeviceFaultReportsKHR",
            codes,
            vec![
                field("device", "VkDevice"),
                field("timeout", "uint64_t"),
                field("pFaultCounts", "uint32_t*"),
                optional(counted(
                    "pFaultInfo",
                    "VkDeviceFaultInfoKHR*",
                    "pFaultCounts",
                )),
            ],
        ));
        ir.structs.push(tagged(
            "VkDeviceFaultInfoKHR",
            "VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_KHR",
            vec![field("flags", "uint32_t")],
        ));
        ir
    }

    #[test]
    fn enumerations_with_other_success_codes_leave_the_array_to_the_caller() {
        for codes in [
            &["VK_SUCCESS", "VK_INCOMPLETE", "VK_TIMEOUT"][..],
            &["VK_SUCCESS", "VK_TIMEOUT"],
            &["VK_SUCCESS", "VK_INCOMPLETE", "VK_NOT_READY"],
        ] {
            assert_generated_alike_under_other_names(
                &fault_reports(codes),
                &Docs::default(),
                "vkGetDeviceFaultReportsKHR",
                &["vkWaitForEventsNV"],
                "pub unsafe fn get_device_fault_reports_khr(&self, timeout: u64, fault_counts: &mut u32, mut fault_info: Option<&mut [DeviceFaultInfoKHR<'_>]>) -> crate::vk::Result<VkResult> {",
            );
            assert!(!generate(&fault_reports(codes)).contains("loop {"));
            assert!(generate(&fault_reports(codes)).contains(
                "if let Some(x) = fault_info.as_ref() { assert_eq!(x.len(), *fault_counts as usize, \"vkGetDeviceFaultReportsKHR: `fault_info` must have `*fault_counts` elements\"); }"
            ));
        }
        assert!(
            generate(&fault_reports(&["VK_SUCCESS", "VK_INCOMPLETE"])).contains(
                "pub unsafe fn get_device_fault_reports_khr(&self, timeout: u64) -> crate::vk::Result<Vec<DeviceFaultInfoKHR<'static>>> { unsafe { loop {"
            ),
            "an enumeration that only reports completeness keeps the loop"
        );
    }

    fn counted_through_a_pointer(arrays: &[(&str, bool)]) -> Registry {
        let mut params = vec![
            field("device", "VkDevice"),
            field("pFaultCounts", "uint32_t*"),
        ];
        for (name, is_optional) in arrays {
            let array = counted(name, "VkDeviceFaultInfoKHR*", "pFaultCounts");
            params.push(if *is_optional { optional(array) } else { array });
        }
        let mut ir = only(result_command(
            "vkGetFaultsNV",
            &["VK_SUCCESS", "VK_INCOMPLETE", "VK_TIMEOUT"],
            params,
        ));
        ir.structs.push(tagged(
            "VkDeviceFaultInfoKHR",
            "VK_STRUCTURE_TYPE_DEVICE_FAULT_INFO_KHR",
            vec![field("flags", "uint32_t")],
        ));
        ir
    }

    fn words(text: &str) -> String {
        text.replace("///", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn a_count_passed_through_a_pointer_is_described_as_read_and_written() {
        for (arrays, note) in [
            (
                &[("pFaultInfo", true)][..],
                "`fault_counts` is the element count of `fault_info`, which the command reads and writes: if `fault_info` is `None`, the command writes the number of elements that it can return to it; otherwise it has to hold the number of elements of `fault_info` when the wrapper is called, which the wrapper checks, and the command overwrites it with the number of elements that it wrote.",
            ),
            (
                &[("pFaultInfo", true), ("pFaultNames", true)],
                "`fault_counts` is the element count of `fault_info` and `fault_names`, which the command reads and writes: if all of them are `None`, the command writes the number of elements that it can return to it; otherwise it has to hold the number of elements of each of them that is not `None` when the wrapper is called, which the wrapper checks, and the command overwrites it with the number of elements that it wrote.",
            ),
            (
                &[("pFaultInfo", false)],
                "`fault_counts` is the element count of `fault_info`, which the command reads and writes: it has to hold the number of elements of `fault_info` when the wrapper is called, which the wrapper checks, and the command overwrites it with the number of elements that it wrote.",
            ),
            (
                &[("pFaultInfo", false), ("pFaultNames", false)],
                "`fault_counts` is the element count of `fault_info` and `fault_names`, which the command reads and writes: it has to hold the number of elements of each of them when the wrapper is called, which the wrapper checks, and the command overwrites it with the number of elements that it wrote.",
            ),
            (
                &[("pFaultInfo", false), ("pFaultNames", true)],
                "`fault_counts` is the element count of `fault_info` and `fault_names`, which the command reads and writes: it has to hold the number of elements of each of them that is not `None` when the wrapper is called, which the wrapper checks, and the command overwrites it with the number of elements that it wrote.",
            ),
        ] {
            let ir = counted_through_a_pointer(arrays);
            let model = Model::new(&ir);
            let out = wrapper::wrapper(&model, &ir.commands[0], Dispatch::Device);
            let documented = words(&out);
            assert!(
                documented.contains(&format!(
                    "through [`Self::get_faults_nv_raw`], which describes the parameters. {note} Returns the result code of the command"
                )),
                "{arrays:?}: {documented}"
            );
            assert_eq!(
                squash(&out).matches("`*fault_counts` elements").count(),
                arrays.len(),
                "{out}"
            );
        }
    }

    #[test]
    fn the_wrappers_of_the_registry_describe_every_count_that_they_read_through_a_pointer() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut found = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = wrapper::wrapper(&model, c, model.dispatch(c));
            let code = squash(&out);
            let documented = words(&out);
            let mut arrays_of_count = BTreeMap::<&str, Vec<&str>>::new();
            for (at, _) in code.match_indices("` must have `*") {
                let array = code[..at].rsplit('`').next().expect("the array");
                let count = code[at + "` must have `*".len()..]
                    .split('`')
                    .next()
                    .expect("the count");
                arrays_of_count.entry(count).or_default().push(array);
            }
            assert_eq!(
                documented
                    .matches(", which the command reads and writes: ")
                    .count(),
                arrays_of_count.len(),
                "{}: {documented}",
                c.name
            );
            for (count, arrays) in arrays_of_count {
                let names = crate::docs::join_and(
                    arrays.iter().map(|array| format!("`{array}`")).collect(),
                );
                assert!(
                    documented.contains(&format!(
                        "`{count}` is the element count of {names}, which the command reads and writes: "
                    )),
                    "{}: {documented}",
                    c.name
                );
                assert!(
                    code.contains(&format!("{count}: &mut ")),
                    "{}: the command writes `{count}`: {code}",
                    c.name
                );
                found.insert(c.name.as_str());
            }
        }
        assert_eq!(
            found,
            BTreeSet::from([
                "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
                "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
                "vkGetDeviceFaultReportsKHR",
            ])
        );
    }

    #[test]
    fn the_enumerations_of_the_registry_loop_only_when_they_report_nothing_but_completeness() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut left_to_the_caller = BTreeSet::new();
        for c in &model.ir.commands {
            if creates_a_dispatching_handle(c) {
                continue;
            }
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            let params = model.dispatch(c).explicit_params(c);
            let enumerates = params
                .last()
                .is_some_and(|last| shape::is_enumeration(params, last));
            if !enumerates {
                continue;
            }
            let codes = c
                .success_codes
                .iter()
                .map(|code| model.result_code(code))
                .collect::<BTreeSet<_>>();
            if codes
                .iter()
                .all(|code| ["VK_SUCCESS", "VK_INCOMPLETE"].contains(code))
            {
                continue;
            }
            assert!(
                !out.contains("loop {") && !out.contains("-> crate::vk::Result<Vec<"),
                "{} enumerates although it reports {codes:?}",
                c.name
            );
            left_to_the_caller.insert(c.name.as_str());
        }
        assert_eq!(
            left_to_the_caller,
            BTreeSet::from(["vkGetDeviceFaultReportsKHR"])
        );
    }

    #[test]
    fn deferrable_commands_take_arrays_of_arrays_as_caller_owned_pointers() {
        let (ir, docs) = widgets();
        let out = generate_with_docs(&ir, &docs);
        assert!(out.contains(
            "pub unsafe fn join_widgets_nv(&self, deferred_operation: DeferredOperationKHR, batches: &[WidgetBatchNV], parts: &[*const WidgetPartNV]) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("assert_eq!(parts.len(), batch_count_len,"));
        assert!(out.contains(
            "let result = self.join_widgets_nv_raw(deferred_operation, len_as_count(batch_count_len, \"vkJoinWidgetsNV\", \"batchCount\"), batches.as_ptr(), parts.as_ptr()); result.into_result()?; Ok(result)"
        ));
        assert_generated_alike_under_other_names(
            &ir,
            &docs,
            "vkJoinWidgetsNV",
            &["vkBuildAccelerationStructuresKHR"],
            "parts: &[*const WidgetPartNV]",
        );
    }

    fn deferrable_creation() -> Registry {
        only(result_command(
            "vkCreateRayTracingPipelinesKHR",
            &[
                "VK_SUCCESS",
                "VK_OPERATION_DEFERRED_KHR",
                "VK_OPERATION_NOT_DEFERRED_KHR",
                "VK_PIPELINE_COMPILE_REQUIRED_EXT",
            ],
            vec![
                field("device", "VkDevice"),
                optional(field("deferredOperation", "VkDeferredOperationKHR")),
                optional(field("pipelineCache", "VkPipelineCache")),
                field("createInfoCount", "uint32_t"),
                counted(
                    "pCreateInfos",
                    "const VkGraphicsPipelineCreateInfo*",
                    "createInfoCount",
                ),
                counted("pPipelines", "VkPipeline*", "createInfoCount"),
            ],
        ))
    }

    #[test]
    fn deferrable_commands_write_into_caller_owned_outputs() {
        let create = deferrable_creation();
        assert_generated_alike_under_other_names(
            &create,
            &Docs::default(),
            "vkCreateRayTracingPipelinesKHR",
            &["vkCompileProgramsLaterNV"],
            "pub unsafe fn create_ray_tracing_pipelines_khr(&self, deferred_operation: DeferredOperationKHR, pipeline_cache: PipelineCache, create_infos: &[GraphicsPipelineCreateInfo<'_>], pipelines: &mut [Pipeline]) -> crate::vk::Result<VkResult> { unsafe { let create_info_count_len: usize = create_infos.len(); assert_eq!(pipelines.len(), create_info_count_len,",
        );
        assert!(generate(&create).contains(
            "let result = self.create_ray_tracing_pipelines_khr_raw(deferred_operation, pipeline_cache, len_as_count(create_info_count_len, \"vkCreateRayTracingPipelinesKHR\", \"createInfoCount\"), create_infos.as_ptr(), pipelines.as_mut_ptr()); result.into_result()?; Ok(result)"
        ));
    }

    #[test]
    fn deferrable_commands_document_how_long_the_memory_has_to_live() {
        let ir = deferrable_creation();
        let model = Model::new(&ir);
        let c = &ir.commands[0];
        let lifetime =
            "the implementation can access the arguments until the deferred operation is complete";
        let words = |text: String| text.split_whitespace().collect::<Vec<_>>().join(" ");
        let raw = words(raw_command(&model, c, Dispatch::Device).replace("///", ""));
        let wrapper = words(wrapper::wrapper(&model, c, Dispatch::Device).replace("///", ""));
        for (what, text) in [("raw command", &raw), ("wrapper", &wrapper)] {
            assert_eq!(text.matches(lifetime).count(), 1, "{what}: {text}");
            assert!(
                text.contains(
                    "must not be passed to any other command as externally synchronized parameters"
                ),
                "{what}: {text}"
            );
        }
        assert!(
            wrapper.contains("What `pipelines` holds depends on the result: after `VK_SUCCESS`, the created handles; after `VK_OPERATION_DEFERRED_KHR`, nothing that may be read before the deferred operation is complete"),
            "{wrapper}"
        );
        let not_deferrable = only(result_command(
            "vkCreateComputePipelines",
            &["VK_SUCCESS"],
            vec![
                field("device", "VkDevice"),
                field("createInfoCount", "uint32_t"),
                counted("pSources", "const VkBuffer*", "createInfoCount"),
                counted("pPipelines", "VkPipeline*", "createInfoCount"),
            ],
        ));
        assert!(!words(emit(&Model::new(&not_deferrable))).contains("deferred operation"));
    }

    fn raw_call_arguments<'a>(out: &'a str, raw: &str) -> Vec<&'a str> {
        let start = out
            .find(&format!("self.{raw}("))
            .unwrap_or_else(|| panic!("no call of {raw} in {out}"))
            + raw.len()
            + "self.(".len();
        let mut depth = 0;
        let mut arguments = Vec::new();
        let mut from = start;
        for (offset, c) in out[start..].char_indices() {
            match c {
                '(' | '[' | '{' | '<' => depth += 1,
                ')' if depth == 0 => {
                    arguments.push(out[from..start + offset].trim());
                    break;
                }
                ')' | ']' | '}' | '>' => depth -= 1,
                ',' if depth == 0 => {
                    arguments.push(out[from..start + offset].trim());
                    from = start + offset + 1;
                }
                _ => {}
            }
        }
        arguments.retain(|argument| !argument.is_empty());
        arguments
    }

    fn with_error_codes(mut ir: Registry, codes: &[&str]) -> Registry {
        let results = ir.enums.iter_mut().find(|e| e.name == "VkResult").unwrap();
        for (name, value) in [
            ("VK_ERROR_OUT_OF_HOST_MEMORY", "-1"),
            ("VK_ERROR_DEVICE_LOST", "-4"),
        ] {
            results.fields.push(EnumField {
                name: name.to_string(),
                value: value.to_string(),
                ..Default::default()
            });
        }
        for c in &mut ir.commands {
            c.error_codes = codes.iter().map(|code| code.to_string()).collect();
        }
        ir
    }

    fn commands_of_every_wrapper_shape() -> Vec<Registry> {
        let get_value = result_command(
            "vkGetFenceValueNV",
            &["VK_SUCCESS"],
            vec![field("fence", "VkFence"), field("pValue", "uint64_t*")],
        );
        let get_value_or_not_ready = result_command(
            "vkGetFenceValueNV",
            &["VK_SUCCESS", "VK_NOT_READY"],
            vec![field("fence", "VkFence"), field("pValue", "uint64_t*")],
        );
        let enumerate = result_command(
            "vkGetFencesNV",
            &["VK_SUCCESS", "VK_INCOMPLETE"],
            vec![
                field("pFenceCount", "uint32_t*"),
                optional(counted("pFences", "VkFence*", "pFenceCount")),
            ],
        );
        let fill = result_command(
            "vkCreateFencesNV",
            &["VK_SUCCESS"],
            vec![
                field("bufferCount", "uint32_t"),
                counted("pBuffers", "const VkBuffer*", "bufferCount"),
                counted("pFences", "VkFence*", "bufferCount"),
            ],
        );
        let pass = |codes: &[&str]| {
            result_command(
                "vkLockFenceNV",
                codes,
                vec![field("fence", "VkFence"), field("timeout", "uint64_t")],
            )
        };
        [
            pass(&["VK_SUCCESS"]),
            pass(&["VK_SUCCESS", "VK_NOT_READY"]),
            pass(&["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"]),
            get_value,
            get_value_or_not_ready,
            enumerate,
            fill,
        ]
        .into_iter()
        .map(only)
        .collect()
    }

    #[test]
    fn non_negative_error_codes_are_returned_as_errors_in_every_wrapper_shape() {
        let guarded =
            "if matches!(result, VkResult::TIMEOUT) { return Err(result); } result.into_result()";
        for ir in commands_of_every_wrapper_shape() {
            let name = ir.commands[0].name.clone();
            let out = generate(&with_error_codes(
                ir,
                &["VK_ERROR_OUT_OF_HOST_MEMORY", "VK_TIMEOUT"],
            ));
            let conversions = out.matches(".into_result()").count();
            assert!(conversions > 0, "{name}: {out}");
            assert_eq!(
                out.matches(guarded).count(),
                conversions,
                "{name} converts a result code without returning VK_TIMEOUT as an error: {out}"
            );
        }
    }

    #[test]
    fn negative_error_codes_leave_the_wrappers_as_they_are() {
        for ir in commands_of_every_wrapper_shape() {
            let name = ir.commands[0].name.clone();
            let without = generate(&ir);
            let with = generate(&with_error_codes(
                ir,
                &["VK_ERROR_OUT_OF_HOST_MEMORY", "VK_ERROR_DEVICE_LOST"],
            ));
            assert_eq!(
                with, without,
                "{name} is generated differently for negative error codes"
            );
            assert!(!with.contains("matches!(result"), "{name}: {with}");
        }
    }

    #[test]
    fn every_listed_non_negative_error_code_is_returned_as_an_error() {
        let ir = with_error_codes(
            only(result_command(
                "vkAcquireProfilingLockKHR",
                &["VK_SUCCESS"],
                vec![field("timeout", "uint64_t")],
            )),
            &["VK_ERROR_OUT_OF_HOST_MEMORY", "VK_NOT_READY", "VK_TIMEOUT"],
        );
        assert_generated_alike_under_other_names(
            &ir,
            &Docs::default(),
            "vkAcquireProfilingLockKHR",
            &["vkWaitForFenceNV"],
            "pub unsafe fn acquire_profiling_lock_khr(&self, timeout: u64) -> crate::vk::Result<()> { unsafe { let result = self.acquire_profiling_lock_khr_raw(timeout); if matches!(result, VkResult::NOT_READY | VkResult::TIMEOUT) { return Err(result); } result.into_result() } }",
        );
        let documented = emit(&Model::new(&ir))
            .split_whitespace()
            .filter(|word| *word != "///")
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            documented.contains("Returns the result code of the command if it is an error, or if it is [`VK_NOT_READY`](VkResult::NOT_READY) or [`VK_TIMEOUT`](VkResult::TIMEOUT), which the registry lists as error codes of the command although they are not negative."),
            "{documented}"
        );
    }

    #[test]
    fn the_wrappers_of_the_registry_return_every_non_negative_error_code_as_an_error() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut found = BTreeSet::new();
        for c in &model.ir.commands {
            let successes = c
                .success_codes
                .iter()
                .map(|code| model.result_code(code))
                .collect::<BTreeSet<_>>();
            for code in &c.error_codes {
                let code = model.result_code(code);
                assert!(
                    !successes.contains(code) && code != "VK_INCOMPLETE",
                    "{}: {code} is listed as an error code although the wrappers take it as a success",
                    c.name
                );
            }
            let non_negative = c
                .error_codes
                .iter()
                .filter(|code| model.result_value(code) >= 0)
                .collect::<Vec<_>>();
            if creates_a_dispatching_handle(c) {
                assert!(
                    non_negative.is_empty(),
                    "{}: the hand-written wrapper converts the result with into_result, which takes {non_negative:?} as a success",
                    c.name
                );
                continue;
            }
            if non_negative.is_empty() {
                continue;
            }
            found.insert(c.name.as_str());
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            let conversions = out.matches(".into_result()").count();
            let only_success_is_ok = out.contains("error => Err(error)")
                || out.contains(
                    "if result == VkResult::SUCCESS { Ok(values) } else { Err((values, result)) }",
                );
            assert!(conversions > 0 || only_success_is_ok, "{}: {out}", c.name);
            for (at, _) in out.match_indices(".into_result()") {
                let guarded = out[..at]
                    .strip_suffix(") { return Err(result); } result")
                    .and_then(|before| {
                        let start = before.rfind("if matches!(result, ")?;
                        Some(&before[start + "if matches!(result, ".len()..])
                    })
                    .unwrap_or_default()
                    .split(" | ")
                    .collect::<BTreeSet<_>>();
                for code in &non_negative {
                    assert!(
                        guarded.contains(result_code(&model, code).as_str()),
                        "{}: the wrapper converts a result code without returning {code} as an error: {out}",
                        c.name
                    );
                }
            }
        }
        assert!(
            !found.is_empty(),
            "the registry lists non-negative error codes"
        );
    }

    #[test]
    fn deferrable_commands_of_the_registry_pass_no_memory_that_the_wrapper_owns() {
        let model = Model::with_docs(Registry::get(), Docs::get());
        let mut deferrable = BTreeSet::new();
        for c in &model.ir.commands {
            if !can_defer(&model, c) {
                continue;
            }
            deferrable.insert(c.name.as_str());
            let out = squash(&wrapper::wrapper(&model, c, model.dispatch(c)));
            for (at, _) in out.match_indices("let ") {
                let statement = &out[at + "let ".len()..];
                let local = statement
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .next()
                    .unwrap_or_default();
                let a_count =
                    local.ends_with("_len") && statement[local.len()..].starts_with(": usize = ");
                let the_result = local == "result" && statement[local.len()..].starts_with(" = ");
                assert!(
                    a_count || the_result,
                    "{}: the wrapper passes memory of its own in `let {}`",
                    c.name,
                    &statement[..statement.len().min(80)]
                );
            }
            let params = model.dispatch(c).explicit_params(c);
            let arguments = params.iter().map(argument_name).collect::<BTreeSet<_>>();
            let vocabulary = [
                "as",
                "as_mut",
                "as_mut_ptr",
                "as_ptr",
                "core",
                "ptr",
                "null",
                "null_mut",
                "map_or",
                "from_ref",
                "from_mut",
                "x",
                "mut",
                "u32",
                "usize",
                "Bool32",
                "self",
                "handle",
                "len_as_count",
            ];
            let passed = raw_call_arguments(&out, &raw_name(c));
            assert_eq!(passed.len(), params.len(), "{}: {passed:?}", c.name);
            for argument in passed {
                let without_messages = argument
                    .split('"')
                    .step_by(2)
                    .collect::<Vec<_>>()
                    .join("\"\"");
                for word in without_messages
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                    .filter(|word| !word.is_empty())
                {
                    let caller_owned = arguments.contains(word)
                        || word.strip_suffix("_len").is_some_and(|count| {
                            arguments.contains(count)
                                || params.iter().any(|p| argument_name(p) == count)
                        })
                        || vocabulary.contains(&word);
                    assert!(
                        caller_owned,
                        "{}: the argument `{argument}` passes `{word}`, which is not an argument of the wrapper",
                        c.name
                    );
                }
            }
        }
        assert_eq!(
            deferrable,
            BTreeSet::from([
                "vkBuildAccelerationStructuresKHR",
                "vkBuildMicromapsEXT",
                "vkCopyAccelerationStructureKHR",
                "vkCopyAccelerationStructureToMemoryKHR",
                "vkCopyMemoryToAccelerationStructureKHR",
                "vkCopyMemoryToMicromapEXT",
                "vkCopyMicromapEXT",
                "vkCopyMicromapToMemoryEXT",
                "vkCreateRayTracingPipelinesKHR",
            ])
        );
    }
}
