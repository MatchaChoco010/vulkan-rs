use super::extensions::api_version_expr;
use super::{header, w};
use crate::model::Model;
use crate::naming::command_const_name;
use vulkan_registry::Command;

pub(crate) fn emit(model: &Model<'_>) -> String {
    let mut commands = model.ir.commands.iter().collect::<Vec<_>>();
    commands.sort_by(|a, b| a.name.as_bytes().cmp(b.name.as_bytes()));

    let mut out = header(&[]);
    w!(
        out,
        "/// One [`CommandInfo`] constant for each command of the registry, named after the method that calls the command.\npub mod command {{\n    use crate::vk::*;\n"
    );
    for c in &commands {
        out.push_str(&info(model, c));
    }
    w!(out, "}}\n");
    w!(
        out,
        "/// Every command of the registry, sorted by C name as bytes. [`CommandInfo::get`] searches it."
    );
    w!(out, "pub const COMMANDS: &[CommandInfo] = &[");
    for c in &commands {
        w!(out, "    command::{},", const_name(c));
    }
    w!(out, "];");
    out
}

fn const_name(c: &Command) -> String {
    command_const_name(&c.name)
}

fn info(model: &Model<'_>, c: &Command) -> String {
    let version = match c.version.as_deref() {
        None => "None".to_string(),
        Some(name) => format!(
            "Some({})",
            api_version_expr(name).unwrap_or_else(|| panic!("{}: version {name}", c.name))
        ),
    };
    let extensions = c
        .extensions
        .iter()
        .map(|name| format!("c\"{name}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "/// What the registry states about the C command {reference}.
        pub const {const_name}: CommandInfo = CommandInfo {{
            name: \"{name}\",
            level: CommandLevel::{level},
            success_codes: &[{success_codes}],
            error_codes: &[{error_codes}],
            version: {version},
            extensions: &[{extensions}],
        }};\n",
        reference = model.reference(&c.name),
        const_name = const_name(c),
        name = c.name,
        level = model.dispatch(c).level(),
        success_codes = result_codes(model, &c.success_codes),
        error_codes = result_codes(model, &c.error_codes),
    )
}

fn result_codes(model: &Model<'_>, codes: &[String]) -> String {
    codes
        .iter()
        .map(|code| result_code(model, code))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn result_code(model: &Model<'_>, code: &str) -> String {
    let field = model
        .get_enum("VkResult")
        .fields
        .iter()
        .find(|f| f.name == code || f.aliases.iter().any(|alias| alias == code))
        .unwrap_or_else(|| panic!("unknown result code {code}"));
    match field.protect {
        None => format!("VkResult::{}", model.enum_variant("VkResult", &field.name)),
        Some(_) => format!("VkResult({})", field.value),
    }
}

#[cfg(test)]
mod tests {
    use super::super::squash;
    use super::*;
    use crate::test_support::{command, field, registry, result_command};
    use vulkan_registry::{EnumField, Handle};

    #[test]
    fn command_table_lists_result_codes_and_providers() {
        let mut ir = registry();
        ir.handles.push(Handle {
            name: "VkQueue".to_string(),
            dispatchable: true,
            ..Default::default()
        });
        let code = |name: &str, value: &str| EnumField {
            name: name.to_string(),
            value: value.to_string(),
            ..Default::default()
        };
        let results = ir.enums.iter_mut().find(|e| e.name == "VkResult").unwrap();
        results.fields = vec![
            code("VK_SUCCESS", "0"),
            code("VK_SUBOPTIMAL_KHR", "1000001003"),
            code("VK_ERROR_DEVICE_LOST", "-4"),
        ];
        let mut present = result_command(
            "vkQueuePresentKHR",
            &["VK_SUCCESS", "VK_SUBOPTIMAL_KHR"],
            vec![field("queue", "VkQueue")],
        );
        present.device = true;
        present.error_codes = vec!["VK_ERROR_DEVICE_LOST".to_string()];
        present.extensions = vec!["VK_KHR_swapchain".to_string()];
        let mut wait = command("vkQueueWaitIdle2", vec![field("queue", "VkQueue")]);
        wait.device = true;
        wait.version = Some("VK_VERSION_1_3".to_string());
        ir.commands = vec![wait, present];
        let out = squash(&emit(&Model::new(&ir)));
        assert!(out.contains(
            "pub const QUEUE_PRESENT_KHR: CommandInfo = CommandInfo { name: \"vkQueuePresentKHR\", level: CommandLevel::Device, success_codes: &[VkResult::SUCCESS, VkResult::SUBOPTIMAL_KHR], error_codes: &[VkResult::ERROR_DEVICE_LOST], version: None, extensions: &[c\"VK_KHR_swapchain\"], };"
        ));
        assert!(out.contains(
            "name: \"vkQueueWaitIdle2\", level: CommandLevel::Device, success_codes: &[], error_codes: &[], version: Some(make_api_version(0, 1, 3, 0)), extensions: &[],"
        ));
        assert!(out.contains(
            "pub const COMMANDS: &[CommandInfo] = &[ command::QUEUE_PRESENT_KHR, command::QUEUE_WAIT_IDLE2, ];"
        ));
    }
}
