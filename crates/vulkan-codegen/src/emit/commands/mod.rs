mod dispatch_tables;
mod inputs;
mod shape;
mod wrapper;

use super::func_pointers::parameter_item;
use super::{arrow, header, w};
use crate::docs::{Block, Doc};
use crate::model::{CType, Cfg, Dispatch, Model};
use crate::naming::{command_name, field_name, pointer_setter_name};
use std::collections::BTreeSet;
use vulkan_registry::{Command, Field};

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut out = header(&["too_many_arguments"]);
    out.push_str("use alloc::vec::Vec;\nuse core::ffi::{c_char, c_void, CStr};\n\n");
    for dispatch in Dispatch::ALL {
        out.push_str(&dispatch_tables::table(model, dispatch));
    }
    for dispatch in Dispatch::ALL {
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
    let parameters = params
        .iter()
        .filter_map(|p| model.described(cfg, &c.name, &p.name))
        .flat_map(parameter_item)
        .collect::<Vec<_>>();
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
    let signature = params
        .iter()
        .map(|p| format!(", {}: {}", field_name(&p.name), model.param_ty(p, "'_")))
        .collect::<String>();
    let self_handle = dispatch
        .passes_self_handle(c)
        .then(|| "self.handle()".to_string());
    let arguments = self_handle
        .into_iter()
        .chain(params.iter().map(|p| field_name(&p.name)))
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

const DEFERRED_SAFETY: &str = "If the command is deferred, that is, it returns `VK_OPERATION_DEFERRED_KHR`, the implementation can read and write the memory that the arguments point to until the deferred operation is complete: that memory, including the slices and the structures and arrays that they point to, has to stay allocated and unchanged until then, and what the command writes must not be read before.";

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
    use crate::emit::squash;
    use crate::test_support::{
        command, counted, field, optional, registry, result_command, structure, tagged,
    };
    use std::collections::BTreeMap;
    use vulkan_registry::{DocItem, DocPage, Docs, EnumField, Handle, Registry};

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
            "self.cmd_bind_vertex_buffers2_raw(command_buffer, first_binding, binding_count_len as u32, buffers.as_ptr(), offsets.as_ptr(), sizes.map_or(core::ptr::null(), |x| x.as_ptr()))"
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

    #[test]
    fn keeping_partial_output_follows_the_result_codes_not_the_command_name() {
        for code in [
            "VK_PIPELINE_COMPILE_REQUIRED",
            "VK_PIPELINE_COMPILE_REQUIRED_EXT",
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
            &["VK_SUCCESS", "VK_PIPELINE_COMPILE_REQUIRED"],
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
            "pub unsafe fn cmd_assemble_widgets_nv(&self, command_buffer: CommandBuffer, batches: &[WidgetBatchNV], first_indices: &[u32], parts: &[&[WidgetPartNV]], limits: &[&[u32]]) {"
        ));
        assert!(out.contains("let batch_count_len: usize = batches.len();"));
        assert!(out.contains("assert_eq!(parts.len(), batch_count_len,"));
        assert!(out.contains("assert_eq!(limits.len(), batch_count_len,"));
        assert!(out.contains(
            "for (element, entry) in batches.iter().zip(parts.iter()) { assert_eq!(entry.len(), element.part_count as usize, \"vkCmdAssembleWidgetsNV: each `parts` entry must have `part_count` elements\"); }"
        ));
        assert!(out.contains(
            "let parts_pointers = parts.iter().map(|entry| entry.as_ptr()).collect::<Vec<_>>();"
        ));
        assert!(out.contains(
            "self.cmd_assemble_widgets_nv_raw(command_buffer, batch_count_len as u32, batches.as_ptr(), first_indices.as_ptr(), parts_pointers.as_ptr(), limits_pointers.as_ptr())"
        ));
        assert_generated_alike_under_other_names(
            &ir,
            &docs,
            "vkCmdAssembleWidgetsNV",
            &[
                "vkCmdBuildAccelerationStructuresKHR",
                "vkCmdBuildAccelerationStructuresIndirectKHR",
            ],
            "parts: &[&[WidgetPartNV]]",
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
        assert!(generate_with_docs(&ir, &docs).contains("parts: &[&[WidgetPartNV]]"));
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
            ("takes slices of slices", ": &[&["),
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
        let renamed = ir
            .commands
            .iter()
            .enumerate()
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

    #[test]
    fn deferrable_commands_take_arrays_of_arrays_as_caller_owned_pointers() {
        let (ir, docs) = widgets();
        let out = generate_with_docs(&ir, &docs);
        assert!(out.contains(
            "pub unsafe fn join_widgets_nv(&self, deferred_operation: DeferredOperationKHR, batches: &[WidgetBatchNV], parts: &[*const WidgetPartNV]) -> crate::vk::Result<VkResult> {"
        ));
        assert!(out.contains("assert_eq!(parts.len(), batch_count_len,"));
        assert!(out.contains(
            "let result = self.join_widgets_nv_raw(deferred_operation, batch_count_len as u32, batches.as_ptr(), parts.as_ptr()); result.into_result()?; Ok(result)"
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
            "let result = self.create_ray_tracing_pipelines_khr_raw(deferred_operation, pipeline_cache, create_info_count_len as u32, create_infos.as_ptr(), pipelines.as_mut_ptr()); result.into_result()?; Ok(result)"
        ));
    }

    #[test]
    fn deferrable_commands_document_how_long_the_memory_has_to_live() {
        let out = emit(&Model::new(&deferrable_creation()));
        let documented = out
            .lines()
            .filter(|line| line.contains("VK_OPERATION_DEFERRED_KHR"))
            .count();
        assert!(
            out.contains("until the deferred operation is complete"),
            "{out}"
        );
        assert!(
            documented >= 2,
            "the raw command and the wrapper document deferral"
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
            assert!(
                !out.contains("for ("),
                "{}: the wrapper builds arrays of its own",
                c.name
            );
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
