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

    fn self_handle(self) -> Option<&'static str> {
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
        match c.name.as_str() {
            "vkGetInstanceProcAddr" => return Dispatch::Entry,
            "vkGetDeviceProcAddr" => return Dispatch::Instance,
            _ => {}
        }
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
    use crate::test_support::{command, field, registry};
    use vulkan_registry::Handle;

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
    fn only_the_dispatching_handle_is_implicit() {
        let mut wait = command("vkDeviceWaitIdle", vec![field("device", "VkDevice")]);
        wait.device = true;
        assert!(Dispatch::Device.passes_self_handle(&wait));
        assert!(Dispatch::Device.explicit_params(&wait).is_empty());
        assert!(!Dispatch::Instance.passes_self_handle(&wait));
        assert_eq!(Dispatch::Entry.explicit_params(&wait).len(), 1);
    }
}
