use super::{CType, Model};
use vulkan_registry::{Command, Field};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Dispatch {
    Entry,
    Instance,
    Device,
}

impl Dispatch {
    pub(crate) const ALL: [Dispatch; 3] = [Dispatch::Entry, Dispatch::Instance, Dispatch::Device];

    pub(crate) fn level(self) -> &'static str {
        match self {
            Dispatch::Entry => "Entry",
            Dispatch::Instance => "Instance",
            Dispatch::Device => "Device",
        }
    }

    pub(crate) fn table(self) -> String {
        format!("{}Fn", self.level())
    }

    pub(crate) fn wrapper(self) -> String {
        format!("crate::{}", self.level())
    }

    pub(crate) fn self_handle(self) -> Option<&'static str> {
        match self {
            Dispatch::Entry => None,
            Dispatch::Instance => Some("VkInstance"),
            Dispatch::Device => Some("VkDevice"),
        }
    }

    pub(crate) fn passes_self_handle(self, c: &Command) -> bool {
        self.self_handle()
            .zip(c.params.first())
            .is_some_and(|(handle, first)| first.ty == handle && CType::of(first).depth() == 0)
    }

    pub(crate) fn explicit_params(self, c: &Command) -> &[Field] {
        if self.passes_self_handle(c) {
            &c.params[1..]
        } else {
            &c.params
        }
    }
}

impl Model<'_> {
    pub(crate) fn dispatch(&self, c: &Command) -> Dispatch {
        let level = self.level_of_first_parameter(c);
        if c.return_ty != "PFN_vkVoidFunction" {
            return level;
        }
        match level {
            Dispatch::Device => Dispatch::Instance,
            Dispatch::Instance => Dispatch::Entry,
            Dispatch::Entry => panic!(
                "{} returns command pointers but takes no handle to load them for",
                c.name
            ),
        }
    }

    fn level_of_first_parameter(&self, c: &Command) -> Dispatch {
        let first_is_handle = c
            .params
            .first()
            .is_some_and(|p| CType::of(p).depth() == 0 && self.is_handle(&p.ty));
        if !first_is_handle {
            return Dispatch::Entry;
        }
        if c.device {
            return Dispatch::Device;
        }
        assert!(
            c.instance,
            "{} is neither instance nor device level",
            c.name
        );
        Dispatch::Instance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{command, field, optional, registry};
    use vulkan_registry::{Handle, Registry};

    #[test]
    fn dispatch_follows_the_registry_level_not_the_first_parameter_name() {
        let mut ir = registry();
        for name in ["VkExternalComputeQueueNV", "VkDevice"] {
            ir.handles.push(Handle {
                name: name.to_string(),
                ..Default::default()
            });
        }
        let mut queue_data = command(
            "vkGetExternalComputeQueueDataNV",
            vec![field("externalQueue", "VkExternalComputeQueueNV")],
        );
        queue_data.device = true;
        let mut create_instance = command(
            "vkCreateInstance",
            vec![field("pCreateInfo", "const VkInstanceCreateInfo*")],
        );
        create_instance.instance = true;
        let model = Model::new(&ir);
        assert_eq!(model.dispatch(&queue_data), Dispatch::Device);
        assert_eq!(model.dispatch(&create_instance), Dispatch::Entry);
    }

    #[test]
    fn commands_that_return_command_pointers_are_loaded_one_level_up_whatever_their_name() {
        let mut ir = registry();
        for name in ["VkInstance", "VkDevice"] {
            ir.handles.push(Handle {
                name: name.to_string(),
                dispatchable: true,
                ..Default::default()
            });
        }
        let model = Model::new(&ir);
        for name in ["vkGetDeviceProcAddr", "vkLookUpCommandNV"] {
            let mut c = command(
                name,
                vec![field("device", "VkDevice"), field("pName", "const char*")],
            );
            c.return_ty = "PFN_vkVoidFunction".to_string();
            c.device = true;
            assert_eq!(model.dispatch(&c), Dispatch::Instance, "{name}");
            c.return_ty = "void".to_string();
            assert_eq!(model.dispatch(&c), Dispatch::Device, "{name}");
        }
        for name in ["vkGetInstanceProcAddr", "vkFindEntryPointEXT"] {
            let mut c = command(
                name,
                vec![
                    optional(field("instance", "VkInstance")),
                    field("pName", "const char*"),
                ],
            );
            c.return_ty = "PFN_vkVoidFunction".to_string();
            c.instance = true;
            assert_eq!(model.dispatch(&c), Dispatch::Entry, "{name}");
            c.return_ty = "void".to_string();
            assert_eq!(model.dispatch(&c), Dispatch::Instance, "{name}");
        }
    }

    #[test]
    fn the_embedded_registry_loads_exactly_the_proc_addr_commands_one_level_up() {
        let ir = Registry::get();
        let model = Model::new(ir);
        let loaded_one_level_up = ir
            .commands
            .iter()
            .filter(|c| model.dispatch(c) != model.level_of_first_parameter(c))
            .map(|c| (c.name.as_str(), model.dispatch(c)))
            .collect::<Vec<_>>();
        assert_eq!(
            loaded_one_level_up,
            [
                ("vkGetDeviceProcAddr", Dispatch::Instance),
                ("vkGetInstanceProcAddr", Dispatch::Entry),
            ]
        );
    }

    #[test]
    fn only_the_dispatching_handle_is_implicit() {
        let mut wait = command("vkDeviceWaitIdle", vec![field("device", "VkDevice")]);
        wait.device = true;
        assert!(Dispatch::Device.passes_self_handle(&wait));
        assert!(Dispatch::Device.explicit_params(&wait).is_empty());
        assert!(!Dispatch::Instance.passes_self_handle(&wait));
        assert_eq!(Dispatch::Entry.explicit_params(&wait).len(), 1);
    }
}
