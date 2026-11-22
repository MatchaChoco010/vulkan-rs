use super::commands;
use crate::emit::w;
use crate::model::{Dispatch, Model};
use crate::naming::command_name;

struct Loader {
    parameters: &'static str,
    get_proc_addr: &'static str,
    handle: &'static str,
}

fn loader(dispatch: Dispatch) -> Loader {
    match dispatch {
        Dispatch::Entry => Loader {
            parameters: "get_instance_proc_addr: PFN_vkGetInstanceProcAddr",
            get_proc_addr: "get_instance_proc_addr",
            handle: "Instance::null()",
        },
        Dispatch::Instance => Loader {
            parameters: "get_instance_proc_addr: PFN_vkGetInstanceProcAddr, instance: Instance",
            get_proc_addr: "get_instance_proc_addr",
            handle: "instance",
        },
        Dispatch::Device => Loader {
            parameters: "get_device_proc_addr: PFN_vkGetDeviceProcAddr, device: Device",
            get_proc_addr: "get_device_proc_addr",
            handle: "device",
        },
    }
}

pub(super) fn table(model: &Model<'_>, dispatch: Dispatch) -> String {
    let Loader {
        parameters,
        get_proc_addr,
        handle,
    } = loader(dispatch);
    let mut fields = String::new();
    let mut loads = String::new();
    for c in commands(model, dispatch) {
        let field = command_name(&c.name);
        let cfg = model.cfg(c.protect.as_deref());
        w!(fields, "{cfg}pub {field}: PFN_{},", c.name);
        if field == get_proc_addr && dispatch == Dispatch::Entry {
            w!(loads, "{field},");
        } else {
            w!(
                loads,
                "{cfg}{field}: crate::load_pfn(get({handle}, c\"{}\".as_ptr())),",
                c.name
            );
        }
    }
    format!(
        "#[derive(Copy, Clone)]
        pub struct {name} {{
            {fields}}}

        impl {name} {{
            pub unsafe fn load({parameters}) -> Self {{
                let get = {get_proc_addr}.expect(\"{get_proc_addr} must not be null\");
                unsafe {{
                    Self {{
                        {loads}}}
                }}
            }}
        }}\n\n",
        name = dispatch.table()
    )
}
