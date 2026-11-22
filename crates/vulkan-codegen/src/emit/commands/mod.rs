mod dispatch_tables;
mod inputs;
mod shape;
mod special;
mod wrapper;

use super::{arrow, header, w};
use crate::model::{CType, Dispatch, Model};
use crate::naming::{command_name, field_name, pointer_setter_name};
use vulkan_registry::{Command, Field};

const HAND_WRITTEN_WRAPPERS: &[&str] = &["vkCreateInstance", "vkCreateDevice"];

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
            .filter(|c| !HAND_WRITTEN_WRAPPERS.contains(&c.name.as_str()))
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
    format!(
        "{cfg}pub unsafe fn {raw_name}(&self{signature}){arrow} {{
            unsafe {{ (self.fp().{pointer}.expect(\"{c_name} is not loaded\"))({arguments}) }}
        }}\n",
        cfg = model.cfg(c.protect.as_deref()),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emit::squash;
    use crate::test_support::{
        command, counted, field, optional, registry, result_command, structure, tagged,
    };
    use vulkan_registry::{Handle, Registry};

    fn base() -> Registry {
        let mut ir = registry();
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
}
