use super::commands;
use crate::emit::w;
use crate::model::{Dispatch, Model};
use crate::naming::command_name;

struct Loader {
    scope: &'static str,
    parameters: &'static str,
    get_proc_addr: &'static str,
    handle: &'static str,
    safety: &'static str,
}

fn loader(dispatch: Dispatch) -> Loader {
    match dispatch {
        Dispatch::Entry => Loader {
            scope: "the commands that are called without an instance",
            safety: "`get_instance_proc_addr` has to be the `vkGetInstanceProcAddr` of a Vulkan loader or implementation that stays loaded for as long as the table is used.",
            parameters: "get_instance_proc_addr: PFN_vkGetInstanceProcAddr",
            get_proc_addr: "get_instance_proc_addr",
            handle: "Instance::null()",
        },
        Dispatch::Instance => Loader {
            scope: "the commands of an instance and of its physical devices",
            safety: "`get_instance_proc_addr` has to be the `vkGetInstanceProcAddr` of the Vulkan loader or implementation that created `instance`, which has to stay loaded for as long as the table is used, and `instance` has to be a valid instance.",
            parameters: "get_instance_proc_addr: PFN_vkGetInstanceProcAddr, instance: Instance",
            get_proc_addr: "get_instance_proc_addr",
            handle: "instance",
        },
        Dispatch::Device => Loader {
            scope: "the commands of a device and of its child objects",
            safety: "`get_device_proc_addr` has to be the `vkGetDeviceProcAddr` of the instance that created `device`, whose loader or implementation has to stay loaded for as long as the table is used, and `device` has to be a valid device.",
            parameters: "get_device_proc_addr: PFN_vkGetDeviceProcAddr, device: Device",
            get_proc_addr: "get_device_proc_addr",
            handle: "device",
        },
    }
}

pub(super) fn table(model: &Model<'_>, dispatch: Dispatch) -> String {
    let Loader {
        scope,
        parameters,
        get_proc_addr,
        handle,
        safety,
    } = loader(dispatch);
    let mut fields = String::new();
    let mut loads = String::new();
    for c in commands(model, dispatch) {
        let field = command_name(&c.name);
        let cfg = model.cfg(c.protect.as_deref());
        w!(
            fields,
            "/// The pointer to {}, or `None` if it is not loaded.\n{cfg}pub {field}: PFN_{},",
            model.reference(&c.name),
            c.name
        );
        if dispatch == Dispatch::Entry && c.return_ty == "PFN_vkVoidFunction" {
            w!(loads, "{field}: Some(get),");
        } else {
            w!(
                loads,
                "{cfg}{field}: crate::load_pfn(get({handle}, c\"{}\".as_ptr())),",
                c.name
            );
        }
    }
    format!(
        "/// The function pointers of {scope}.
        ///
        /// [`Self::load`] fills the table. [`{owner}`] holds one and calls through it.
        #[derive(Copy, Clone)]
        pub struct {name} {{
            {fields}}}

        impl {name} {{
            /// Asks `{get_proc_addr}` for every command of the table. A command that it
            /// returns no pointer for is left `None`.
            ///
            /// # Safety
            ///
            /// {safety}
            ///
            /// # Panics
            ///
            /// Panics if `{get_proc_addr}` is `None`.
            pub unsafe fn load({parameters}) -> Self {{
                let get = {get_proc_addr}.expect(\"{get_proc_addr} must not be null\");
                unsafe {{
                    Self {{
                        {loads}}}
                }}
            }}
        }}\n\n",
        name = dispatch.table(),
        owner = dispatch.wrapper()
    )
}
