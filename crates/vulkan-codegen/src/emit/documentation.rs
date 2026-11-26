use super::{bitmasks, commands, constants, doc_of, enums, extensions, handles, structs};
use crate::model::Model;
use crate::test_support::{counted, field, registry, result_command, tagged};
use vulkan_registry::{
    Bitmask, Constant, DocItem, DocPage, DocTable, Docs, Enum, EnumField, Extension, Flags, Handle,
    Registry,
};

fn page(name: &str, desc: &str, items: &[(&str, &str)]) -> DocPage {
    DocPage {
        name: name.to_string(),
        kind: String::new(),
        desc: desc.to_string(),
        aliases: Vec::new(),
        file: "chapters/queues.adoc".to_string(),
        items: items
            .iter()
            .map(|(name, text)| DocItem {
                name: name.to_string(),
                text: text.to_string(),
            })
            .collect(),
    }
}

fn value(name: &str, value: &str, aliases: &[&str]) -> EnumField {
    EnumField {
        name: name.to_string(),
        value: value.to_string(),
        aliases: aliases.iter().map(|alias| alias.to_string()).collect(),
        ..Default::default()
    }
}

fn enumeration(name: &str, fields: Vec<EnumField>) -> Enum {
    Enum {
        name: name.to_string(),
        bit_width: 32,
        fields,
        ..Default::default()
    }
}

fn ir() -> Registry {
    let mut ir = registry();
    ir.enums = vec![
        enumeration(
            "VkStructureType",
            vec![value("VK_STRUCTURE_TYPE_QUEUE_INFO", "1", &[])],
        ),
        enumeration(
            "VkResult",
            vec![
                value("VK_SUCCESS", "0", &[]),
                value("VK_ERROR_DEVICE_LOST", "-4", &[]),
            ],
        ),
        enumeration(
            "VkObjectType",
            vec![value("VK_OBJECT_TYPE_QUEUE", "4", &[])],
        ),
        enumeration(
            "VkColor",
            vec![
                value("VK_COLOR_RED", "0", &["VK_COLOR_CRIMSON"]),
                value("VK_COLOR_BLUE", "1", &[]),
                value("VK_COLOR_GREEN", "2", &[]),
            ],
        ),
    ];
    ir.bitmasks.push(Bitmask {
        name: "VkQueueFlagBits".to_string(),
        flags_name: "VkQueueFlags".to_string(),
        bit_width: 32,
        fields: vec![value("VK_QUEUE_GRAPHICS_BIT", "1", &[])],
        ..Default::default()
    });
    ir.flags.push(Flags {
        name: "VkQueueFlags".to_string(),
        bitmask: Some("VkQueueFlagBits".to_string()),
        bit_width: 32,
        ..Default::default()
    });
    for (name, object_type, parent) in [
        ("VkDevice", "VK_OBJECT_TYPE_QUEUE", None),
        ("VkQueue", "VK_OBJECT_TYPE_QUEUE", Some("VkDevice")),
    ] {
        ir.handles.push(Handle {
            name: name.to_string(),
            object_type: object_type.to_string(),
            parent: parent.map(str::to_string),
            dispatchable: true,
            ..Default::default()
        });
    }
    ir.structs.push(tagged(
        "VkDeviceInfo",
        "VK_STRUCTURE_TYPE_QUEUE_INFO",
        vec![],
    ));
    let mut queue_info = tagged(
        "VkQueueInfo",
        "VK_STRUCTURE_TYPE_QUEUE_INFO",
        vec![
            field("queueCount", "uint32_t"),
            counted("pPriorities", "const float*", "queueCount"),
            field("reserved", "uint32_t"),
        ],
    );
    queue_info.version = Some("VK_VERSION_1_1".to_string());
    queue_info.extensions = vec!["VK_KHR_queue".to_string()];
    queue_info.extends = vec!["VkDeviceInfo".to_string()];
    ir.structs.push(queue_info);
    let mut get_queue = result_command(
        "vkGetQueue",
        &["VK_SUCCESS"],
        vec![
            field("device", "VkDevice"),
            field("queueIndex", "uint32_t"),
            field("pQueue", "VkQueue*"),
        ],
    );
    get_queue.device = true;
    get_queue.error_codes = vec!["VK_ERROR_DEVICE_LOST".to_string()];
    ir.commands.push(get_queue);
    ir.extensions.push(Extension {
        name: "VK_KHR_queue".to_string(),
        name_macro: "VK_KHR_QUEUE_EXTENSION_NAME".to_string(),
        spec_version_macro: "VK_KHR_QUEUE_SPEC_VERSION".to_string(),
        spec_version: 3,
        device: true,
        depends: Some("VK_VERSION_1_1,VK_KHR_other".to_string()),
        promoted_to: Some("VK_VERSION_1_2".to_string()),
        ..Default::default()
    });
    ir.extensions.push(Extension {
        name: "VK_KHR_other".to_string(),
        name_macro: "VK_KHR_OTHER_EXTENSION_NAME".to_string(),
        spec_version_macro: "VK_KHR_OTHER_SPEC_VERSION".to_string(),
        spec_version: 1,
        device: true,
        ..Default::default()
    });
    ir.constants.push(Constant {
        name: "VK_MAX_QUEUES".to_string(),
        ty: "uint32_t".to_string(),
        value: "8".to_string(),
        video_std_header: None,
    });
    ir
}

fn docs() -> Docs {
    Docs {
        commit: "000000000000".to_string(),
        pages: vec![
            page(
                "VkQueueInfo",
                "Structure describing queues",
                &[
                    ("sType", "pname:sType is a elink:VkStructureType value."),
                    (
                        "queueCount",
                        "pname:queueCount is the number of queues, at most dlink:VK_MAX_QUEUES.",
                    ),
                ],
            ),
            page(
                "VkColor",
                "Colors of a queue",
                &[(
                    "VK_COLOR_RED",
                    "ename:VK_COLOR_RED specifies red, not ename:VK_COLOR_BLUE.",
                )],
            ),
            page(
                "VkQueueFlagBits",
                "Bitmask specifying capabilities of queues",
                &[(
                    "VK_QUEUE_GRAPHICS_BIT",
                    "ename:VK_QUEUE_GRAPHICS_BIT specifies graphics.",
                )],
            ),
            page("VkQueueFlags", "Bitmask of VkQueueFlagBits", &[]),
            page("VkQueue", "Opaque handle to a queue object", &[]),
            page("VK_MAX_QUEUES", "Number of queues", &[]),
            page(
                "vkGetQueue",
                "Get a queue handle from a device",
                &[
                    (
                        "device",
                        "pname:device is the logical device that owns the queue.",
                    ),
                    ("queueIndex", "pname:queueIndex is the index of the queue."),
                    (
                        "pQueue",
                        "pname:pQueue is a pointer to a slink:VkQueue object.",
                    ),
                ],
            ),
        ],
        tables: vec![
            DocTable {
                file: "chapters/queues.adoc".to_string(),
                page: Some("VkColor".to_string()),
                text: [
                    ".Wavelengths",
                    "[options=\"header\"]",
                    "|====",
                    "|elink:VkColor |Wavelength",
                    "|ename:VK_COLOR_RED |long",
                    "|ename:VK_COLOR_GREEN |medium, between ename:VK_COLOR_RED and ename:VK_COLOR_BLUE",
                    "|====",
                ]
                .join("\n"),
            },
            DocTable {
                file: "chapters/queues.adoc".to_string(),
                page: None,
                text: "[options=\"header\"]\n|====\n|Object type |Handle\n|ename:VK_OBJECT_TYPE_QUEUE |slink:VkQueue\n|===="
                    .to_string(),
            },
        ],
        ..Default::default()
    }
}

const PAGES: &str = "https://docs.vulkan.org/refpages/latest/refpages/source";

fn alias(c_name: &str) -> String {
    format!("#[doc(alias = \"{c_name}\")]")
}

#[test]
fn structures_carry_their_summary_origin_and_the_described_members() {
    let (ir, docs) = (ir(), docs());
    let out = structs::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub struct QueueInfo<'a>"),
        [
            "Structure describing queues.".to_string(),
            format!(
                "C structure [`VkQueueInfo`]({PAGES}/VkQueueInfo.html), provided by Vulkan 1.1 or [`VK_KHR_queue`](KHR_QUEUE_NAME)."
            ),
            "Extends [`VkDeviceInfo`](DeviceInfo): their `push_next` accepts this structure."
                .to_string(),
            alias("VkQueueInfo"),
        ]
    );
    let queue_info = &out[out.find("pub struct QueueInfo<'a>").expect("the struct")..];
    assert_eq!(
        doc_of(queue_info, "pub s_type:"),
        ["`sType` is a [`VkStructureType`](StructureType) value."]
    );
    assert_eq!(
        doc_of(&out, "pub queue_count:"),
        ["`queueCount` is the number of queues, at most [`VK_MAX_QUEUES`](MAX_QUEUES)."]
    );
    assert_eq!(
        doc_of(&out, "pub reserved:"),
        ["The C member `uint32_t reserved`."]
    );
    assert_eq!(
        doc_of(&out, "pub struct DeviceInfo<'a>"),
        [
            "C structure `VkDeviceInfo`, provided by Vulkan 1.0.".to_string(),
            alias("VkDeviceInfo")
        ]
    );
}

fn with_item(mut docs: Docs, page: &str, item: &str, lines: &[&str]) -> Docs {
    let described = docs
        .pages
        .iter_mut()
        .find(|described| described.name == page)
        .and_then(|page| {
            page.items
                .iter_mut()
                .find(|described| described.name == item)
        })
        .unwrap_or_else(|| panic!("no item {item} on the page {page}"));
    described.text = lines.join("\n");
    docs
}

#[test]
fn members_and_parameters_carry_everything_that_the_specification_attaches_to_their_item() {
    let ir = ir();
    let docs = with_item(
        docs(),
        "VkQueueInfo",
        "queueCount",
        &[
            "pname:queueCount is the number of queues, in the range",
            "[eq]#[1,dlink:VK_MAX_QUEUES]#",
            ".",
            "+",
            "The queues are:",
            "+",
            "** created first",
            "** destroyed last",
            "+",
            "[NOTE]",
            "====",
            "Queues are cheap.",
            "====",
        ],
    );
    let docs = with_item(
        docs,
        "vkGetQueue",
        "queueIndex",
        &[
            "pname:queueIndex is the index of the queue.",
            "+",
            "It is less than the number of queues.",
        ],
    );
    let model = Model::with_docs(&ir, &docs);
    assert_eq!(
        doc_of(&structs::emit(&model), "pub queue_count:"),
        [
            "`queueCount` is the number of queues, in the range \\[1,[`VK_MAX_QUEUES`](MAX_QUEUES)\\].",
            "The queues are:",
            "- created first - destroyed last",
            "> **Note** > > Queues are cheap.",
        ]
    );
    let commands = commands::emit(&model);
    let parameters = commands
        .lines()
        .map(str::trim)
        .skip_while(|line| *line != "/// # Parameters")
        .skip(1)
        .take_while(|line| *line != "/// # Safety")
        .collect::<Vec<_>>();
    assert_eq!(
        parameters,
        [
            "///",
            "/// - `queueIndex` is the index of the queue.",
            "///",
            "///   It is less than the number of queues.",
            "/// - `pQueue` is a pointer to a [`VkQueue`](Queue) object.",
            "///",
        ]
    );
}

fn with_items(mut docs: Docs, name: &str, items: &[(&str, &str)]) -> Docs {
    let described = docs
        .pages
        .iter_mut()
        .find(|described| described.name == name)
        .unwrap_or_else(|| panic!("no page {name}"));
    described.items = page(name, "", items).items;
    docs
}

#[test]
fn an_item_that_starts_with_several_names_documents_each_of_them() {
    let ir = ir();
    let docs = with_items(
        docs(),
        "VkQueueInfo",
        &[(
            "queueCount",
            "pname:queueCount, pname:pPriorities and pname:reserved are the number of queues,\ntheir priorities and nothing.",
        )],
    );
    let docs = with_items(
        docs,
        "VkColor",
        &[(
            "VK_COLOR_RED",
            "ename:VK_COLOR_RED and ename:VK_COLOR_BLUE specify a primary color.",
        )],
    );
    let docs = with_items(
        docs,
        "vkGetQueue",
        &[(
            "queueIndex",
            "pname:queueIndex and pname:pQueue are the index of the queue and where it is returned.",
        )],
    );
    let model = Model::with_docs(&ir, &docs);
    let structs = structs::emit(&model);
    for member in ["queue_count", "p_priorities", "reserved"] {
        assert_eq!(
            doc_of(&structs, &format!("pub {member}:")),
            [
                "`queueCount`, `pPriorities` and `reserved` are the number of queues, their priorities and nothing."
            ],
            "{member}"
        );
    }
    let enums = enums::emit(&model);
    assert_eq!(
        doc_of(&enums, "pub const RED:"),
        [
            "`VK_COLOR_RED` and [`VK_COLOR_BLUE`](Color::BLUE) specify a primary color."
                .to_string(),
            alias("VK_COLOR_RED"),
        ]
    );
    assert_eq!(
        doc_of(&enums, "pub const BLUE:"),
        [
            "[`VK_COLOR_RED`](Color::RED) and `VK_COLOR_BLUE` specify a primary color.".to_string(),
            alias("VK_COLOR_BLUE"),
        ]
    );
    let commands = commands::emit(&model);
    let parameters = commands
        .lines()
        .map(str::trim)
        .skip_while(|line| *line != "/// # Parameters")
        .skip(1)
        .take_while(|line| *line != "/// # Safety")
        .collect::<Vec<_>>();
    assert_eq!(
        parameters,
        [
            "///",
            "/// - `queueIndex` and `pQueue` are the index of the queue and where it is returned.",
            "///",
        ],
        "an item that describes two parameters is listed once"
    );
}

#[test]
fn setters_name_the_members_they_set() {
    let (ir, docs) = (ir(), docs());
    let out = structs::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub fn queue_count("),
        ["Sets [`queue_count`](Self::queue_count)."]
    );
    assert_eq!(
        doc_of(&out, "pub fn priorities("),
        [
            "Sets [`p_priorities`](Self::p_priorities) to the start of a slice of elements and [`queue_count`](Self::queue_count) to its length.",
            "An empty slice sets [`p_priorities`](Self::p_priorities) to a null pointer.",
            "# Panics",
            "Panics if the slice has more elements than `queueCount`, a `uint32_t`, can count."
        ]
    );
    assert!(doc_of(&out, "pub fn push_next<")[0].starts_with("Puts `next` at the head"));
}

#[test]
fn enumerants_are_described_aliased_or_named() {
    let (ir, docs) = (ir(), docs());
    let out = enums::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub struct Color("),
        [
            "Colors of a queue.".to_string(),
            format!("C enumeration [`VkColor`]({PAGES}/VkColor.html)."),
            alias("VkColor"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const RED:"),
        [
            "`VK_COLOR_RED` specifies red, not [`VK_COLOR_BLUE`](Color::BLUE).".to_string(),
            alias("VK_COLOR_RED"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const CRIMSON:"),
        [
            "`VK_COLOR_CRIMSON` is an alias of [`VK_COLOR_RED`](Self::RED).".to_string(),
            alias("VK_COLOR_CRIMSON"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const BLUE:"),
        [
            format!("`VK_COLOR_BLUE`, a value of [`VkColor`]({PAGES}/VkColor.html)."),
            alias("VK_COLOR_BLUE"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const QUEUE_INFO:"),
        [
            "`VK_STRUCTURE_TYPE_QUEUE_INFO`, the structure type of [`VkDeviceInfo`](DeviceInfo)."
                .to_string(),
            alias("VK_STRUCTURE_TYPE_QUEUE_INFO"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const QUEUE: Self = Self(4)"),
        [
            "`VK_OBJECT_TYPE_QUEUE`, the object type of [`VkDevice`](Device).".to_string(),
            alias("VK_OBJECT_TYPE_QUEUE"),
        ]
    );
}

#[test]
fn values_without_a_described_list_item_take_the_cells_of_their_table_row() {
    let (ir, docs) = (ir(), docs());
    let out = enums::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub const GREEN:"),
        [
            "`VK_COLOR_GREEN` in the table “Wavelengths” of the specification:".to_string(),
            "- Wavelength: medium, between [`VK_COLOR_RED`](Color::RED) and [`VK_COLOR_BLUE`](Color::BLUE)"
                .to_string(),
            alias("VK_COLOR_GREEN"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const RED:")[0],
        "`VK_COLOR_RED` specifies red, not [`VK_COLOR_BLUE`](Color::BLUE).",
        "a described list item comes before a table row"
    );
    assert_eq!(
        doc_of(&out, "pub const QUEUE: Self = Self(4)")[0],
        "`VK_OBJECT_TYPE_QUEUE`, the object type of [`VkDevice`](Device).",
        "the object type that the registry gives comes before a table row"
    );
}

#[test]
fn flags_take_the_descriptions_of_their_bits() {
    let (ir, docs) = (ir(), docs());
    let out = bitmasks::emit(&Model::with_docs(&ir, &docs));
    let flags = out
        .split("pub struct QueueFlags(")
        .nth(1)
        .expect("the flags type");
    assert_eq!(
        doc_of(flags, "pub const GRAPHICS:"),
        [
            "`VK_QUEUE_GRAPHICS_BIT` specifies graphics.".to_string(),
            alias("VK_QUEUE_GRAPHICS_BIT"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub struct QueueFlags("),
        [
            "Bitmask of VkQueueFlagBits.".to_string(),
            format!("C bitmask type [`VkQueueFlags`]({PAGES}/VkQueueFlags.html)."),
            "The bits are the constants of [`VkQueueFlagBits`](QueueFlagBits), which this type has as well. Values combine with `|`, `&`, `^` and `!`."
                .to_string(),
            alias("VkQueueFlags"),
        ]
    );
}

#[test]
fn handles_name_their_parent() {
    let (ir, docs) = (ir(), docs());
    let out = handles::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub struct Queue("),
        [
            "Opaque handle to a queue object.".to_string(),
            format!("C dispatchable handle [`VkQueue`]({PAGES}/VkQueue.html)."),
            "Objects of this type are created from a [`VkDevice`](Device). The value is the handle itself: copying it does not copy the object, and dropping it does not destroy the object."
                .to_string(),
            alias("VkQueue"),
        ]
    );
}

#[test]
fn raw_commands_list_the_parameters_of_their_signature() {
    let (ir, docs) = (ir(), docs());
    let out = commands::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub unsafe fn get_queue_raw("),
        [
            "Get a queue handle from a device.".to_string(),
            format!(
                "Calls the C command [`vkGetQueue`]({PAGES}/vkGetQueue.html) with the arguments as they are. The command is provided by Vulkan 1.0."
            ),
            "# Parameters".to_string(),
            "- `queueIndex` is the index of the queue. - `pQueue` is a pointer to a [`VkQueue`](Queue) object."
                .to_string(),
            "# Safety".to_string(),
            format!(
                "The call has to satisfy the valid usage that the Vulkan specification states for [`vkGetQueue`]({PAGES}/vkGetQueue.html)."
            ),
            "# Panics".to_string(),
            "Panics if `vkGetQueue` is not loaded: the loader returned no pointer for it, for example because the version or the extension that provides it is not enabled."
                .to_string(),
        ]
    );
}

#[test]
fn wrappers_say_what_they_return_and_which_errors_the_registry_lists() {
    let (ir, docs) = (ir(), docs());
    let out = commands::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub unsafe fn get_queue("),
        [
            "Get a queue handle from a device.".to_string(),
            format!(
                "Calls the C command [`vkGetQueue`]({PAGES}/vkGetQueue.html) through [`Self::get_queue_raw`], which describes the parameters. Returns what the command writes to `pQueue`. The command is provided by Vulkan 1.0."
            ),
            "# Errors".to_string(),
            "Returns the result code of the command if it is an error. The registry lists [`VK_ERROR_DEVICE_LOST`](VkResult::ERROR_DEVICE_LOST) for it."
                .to_string(),
            "# Safety".to_string(),
            "The same as for [`Self::get_queue_raw`].".to_string(),
            "# Panics".to_string(),
            "Panics if `vkGetQueue` is not loaded: the loader returned no pointer for it, for example because the version or the extension that provides it is not enabled."
                .to_string(),
            alias("vkGetQueue"),
        ]
    );
}

#[test]
fn extension_names_state_level_requirements_and_promotion() {
    let (ir, docs) = (ir(), docs());
    let out = extensions::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub const KHR_QUEUE_NAME:"),
        [
            format!(
                "The name of the device extension [`VK_KHR_queue`]({PAGES}/VK_KHR_queue.html), the C macro `VK_KHR_QUEUE_EXTENSION_NAME`."
            ),
            "Requires Vulkan 1.1 or [`VK_KHR_other`](KHR_OTHER_NAME). Promoted to Vulkan 1.2."
                .to_string(),
            alias("VK_KHR_queue"),
            alias("VK_KHR_QUEUE_EXTENSION_NAME"),
        ]
    );
}

#[test]
fn constants_carry_their_summary() {
    let (ir, docs) = (ir(), docs());
    let out = constants::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub const MAX_QUEUES:"),
        [
            "Number of queues.".to_string(),
            format!("C constant [`VK_MAX_QUEUES`]({PAGES}/VK_MAX_QUEUES.html)."),
            alias("VK_MAX_QUEUES"),
        ]
    );
}
