use super::{bitmasks, commands, constants, doc_of, enums, extensions, handles, structs, version};
use crate::model::Model;
use crate::test_support::{catch_quietly, counted, field, registry, result_command, tagged};
use vulkan_registry::{
    Bitmask, Constant, DocItem, DocItemOutsidePages, DocPage, DocTable, Docs, Enum, EnumField,
    Extension, Flags, Handle, Legacy, LegacyAttribute, LegacyElement, LegacyKind, LegacyReason,
    Registry, Version,
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
        unnamed_items: Vec::new(),
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
fn a_list_item_outside_the_pages_describes_only_a_value_that_nothing_else_describes() {
    let ir = ir();
    let mut docs = docs();
    docs.pages
        .push(page("VkStructureType", "Structure types", &[]));
    docs.pages.push(page("VkObjectType", "Object types", &[]));
    for page in &mut docs.pages {
        page.items.retain(|item| item.name != "queueIndex");
    }
    let outside_pages = |name: &str, text: &str| DocItemOutsidePages {
        file: "chapters/queues.adoc".to_string(),
        name: name.to_string(),
        text: format!("ename:{name} specifies {text}."),
    };
    docs.items_outside_pages = [
        "VK_COLOR_RED",
        "VK_COLOR_CRIMSON",
        "VK_COLOR_GREEN",
        "VK_STRUCTURE_TYPE_QUEUE_INFO",
        "VK_OBJECT_TYPE_QUEUE",
    ]
    .iter()
    .flat_map(|name| {
        [
            outside_pages(name, "one thing"),
            outside_pages(name, "another"),
        ]
    })
    .chain(["VK_COLOR_BLUE", "reserved", "queueIndex"].map(|name| outside_pages(name, "a thing")))
    .collect();
    let model = Model::with_docs(&ir, &docs);
    let values = enums::emit(&model);
    for (constant, doc, source) in [
        (
            "pub const RED:",
            "`VK_COLOR_RED` specifies red, not [`VK_COLOR_BLUE`](Color::BLUE).",
            "a list item of the page",
        ),
        (
            "pub const CRIMSON:",
            "`VK_COLOR_CRIMSON` is an alias of [`VK_COLOR_RED`](Self::RED).",
            "the name that it is an alias of",
        ),
        (
            "pub const GREEN:",
            "`VK_COLOR_GREEN` in the table “Wavelengths” of the specification:",
            "a table row",
        ),
        (
            "pub const QUEUE_INFO:",
            "`VK_STRUCTURE_TYPE_QUEUE_INFO`, the structure type of [`VkDeviceInfo`](DeviceInfo).",
            "the structure of the registry",
        ),
        (
            "pub const QUEUE: Self = Self(4)",
            "`VK_OBJECT_TYPE_QUEUE`, the object type of [`VkDevice`](Device).",
            "the handle of the registry",
        ),
        (
            "pub const BLUE:",
            "`VK_COLOR_BLUE` specifies a thing.",
            "the list item outside the pages",
        ),
    ] {
        assert_eq!(
            doc_of(&values, constant)[0],
            doc,
            "{source} describes the value"
        );
    }
    assert_eq!(
        doc_of(&structs::emit(&model), "pub reserved:"),
        ["The C member `uint32_t reserved`."],
        "a member takes no list item from outside the pages"
    );
    let parameters = doc_of(&commands::emit(&model), "pub unsafe fn get_queue_raw(");
    assert_eq!(
        parameters[3], "- `pQueue` is a pointer to a [`VkQueue`](Queue) object.",
        "a parameter takes no list item from outside the pages"
    );
}

#[test]
fn a_description_under_an_alias_name_comes_after_everything_under_the_name_of_the_value() {
    let mut ir = ir();
    let structure_types = ir
        .enums
        .iter_mut()
        .find(|e| e.name == "VkStructureType")
        .expect("the structure types");
    for field in &mut structure_types.fields {
        field.aliases = vec![format!("{}_KHR", field.name)];
    }
    let object_types = ir
        .enums
        .iter_mut()
        .find(|e| e.name == "VkObjectType")
        .expect("the object types");
    for field in &mut object_types.fields {
        field.aliases = vec![format!("{}_KHR", field.name)];
    }
    let mut docs = docs();
    for page in &mut docs.pages {
        page.items.retain(|item| item.name != "VK_COLOR_RED");
    }
    docs.pages.push(page(
        "VkStructureType",
        "Structure types",
        &[(
            "VK_STRUCTURE_TYPE_QUEUE_INFO_KHR",
            "ename:VK_STRUCTURE_TYPE_QUEUE_INFO_KHR specifies a structure.",
        )],
    ));
    docs.pages.push(page(
        "VkObjectType",
        "Object types",
        &[(
            "VK_OBJECT_TYPE_QUEUE_KHR",
            "ename:VK_OBJECT_TYPE_QUEUE_KHR specifies a handle.",
        )],
    ));
    let colors = docs
        .pages
        .iter_mut()
        .find(|page| page.name == "VkColor")
        .expect("the page of the colors");
    colors.items.push(DocItem {
        name: "VK_COLOR_CRIMSON".to_string(),
        text: "ename:VK_COLOR_CRIMSON specifies crimson.".to_string(),
    });
    let red = |docs: &Docs| doc_of(&enums::emit(&Model::with_docs(&ir, docs)), "pub const RED:");
    assert_eq!(
        red(&docs)[0],
        "`VK_COLOR_RED` in the table “Wavelengths” of the specification:",
        "a table row under the name of the value comes first"
    );
    assert_eq!(
        doc_of(
            &enums::emit(&Model::with_docs(&ir, &docs)),
            "pub const QUEUE_INFO:"
        )[0],
        "`VK_STRUCTURE_TYPE_QUEUE_INFO`, the structure type of [`VkDeviceInfo`](DeviceInfo).",
        "the structure of the registry comes first"
    );
    assert_eq!(
        doc_of(
            &enums::emit(&Model::with_docs(&ir, &docs)),
            "pub const QUEUE: Self = Self(4)"
        )[0],
        "`VK_OBJECT_TYPE_QUEUE`, the object type of [`VkDevice`](Device).",
        "the handle of the registry comes first"
    );
    docs.tables.clear();
    assert_eq!(
        red(&docs)[0],
        "`VK_COLOR_RED`, which the specification describes under the name `VK_COLOR_CRIMSON`: [`VK_COLOR_CRIMSON`](Color::CRIMSON) specifies crimson."
    );
    docs.items_outside_pages = vec![DocItemOutsidePages {
        file: "chapters/queues.adoc".to_string(),
        name: "VK_COLOR_RED".to_string(),
        text: "ename:VK_COLOR_RED specifies red in the running text.".to_string(),
    }];
    assert_eq!(
        red(&docs)[0],
        "`VK_COLOR_RED` specifies red in the running text.",
        "a list item under the name of the value comes first, wherever it is"
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

const LEGACY: &str = "https://docs.vulkan.org/spec/latest/appendices/legacy.html#legacy-queues";

fn legacy(successor: &str) -> String {
    format!(
        "Legacy functionality, superseded by {successor}: see [Legacy Functionality]({LEGACY}) in the Vulkan specification."
    )
}

fn legacy_as_of(marked_by: &str) -> String {
    format!(
        "Legacy functionality as of {marked_by}: see [Legacy Functionality]({LEGACY}) in the Vulkan specification."
    )
}

fn legacy_ir() -> Registry {
    let mark = |kind, name: &str, marked_by: &[&str], superseded_by: Option<&str>| Legacy {
        kind,
        name: name.to_string(),
        structure: None,
        link: "legacy-queues".to_string(),
        marked_by: marked_by.iter().map(|by| by.to_string()).collect(),
        superseded_by: superseded_by.map(str::to_string),
    };
    let feature = |name: &str| Legacy {
        structure: Some("VkQueueInfo".to_string()),
        ..mark(
            LegacyKind::Feature,
            name,
            &["VK_VERSION_1_2", "VK_KHR_queue"],
            None,
        )
    };
    let mut ir = ir();
    let device_info = &mut ir.structs[0];
    assert_eq!(device_info.name, "VkDeviceInfo");
    device_info.aliases = vec!["VkDeviceInfoKHR".to_string()];
    let queue_info = &mut ir.structs[1];
    assert_eq!(queue_info.name, "VkQueueInfo");
    let mut label = field("label", "char");
    label.fixed_size_array = vec!["8".to_string()];
    queue_info.members.push(label);
    ir.structs.push(tagged(
        "VkOtherInfoKHR",
        "VK_STRUCTURE_TYPE_QUEUE_INFO",
        vec![],
    ));
    ir.versions.push(Version {
        name: "VK_VERSION_1_1".to_string(),
        name_api: "VK_API_VERSION_1_1".to_string(),
    });
    let v1_1 = ["VK_VERSION_1_1"];
    ir.legacy = vec![
        mark(LegacyKind::Type, "VkQueueInfo", &v1_1, Some("VkDeviceInfo")),
        mark(
            LegacyKind::Type,
            "VkOtherInfoKHR",
            &["VK_VERSION_1_1", "VK_KHR_queue", "VK_KHR_other"],
            None,
        ),
        mark(
            LegacyKind::Type,
            "VkDeviceInfoKHR",
            &["VK_VERSION_1_1", "VK_KHR_queue"],
            Some("VkQueueInfo"),
        ),
        mark(LegacyKind::Type, "VkDevice", &["VK_KHR_queue"], None),
        mark(
            LegacyKind::Type,
            "VkDeviceInfo",
            &["VK_KHR_queue", "VK_KHR_other"],
            Some("VkQueueInfo"),
        ),
        mark(
            LegacyKind::Type,
            "VkColor",
            &["VK_KHR_queue", "VK_VERSION_1_3"],
            None,
        ),
        mark(
            LegacyKind::Type,
            "VkQueueFlagBits",
            &["VK_VERSION_1_1", "VK_KHR_queue"],
            None,
        ),
        mark(
            LegacyKind::Type,
            "VkQueueFlags",
            &["VK_VERSION_1_0", "VK_KHR_queue", "VK_KHR_other"],
            None,
        ),
        mark(LegacyKind::Type, "VkQueue", &v1_1, Some("VkDevice")),
        mark(LegacyKind::Type, "VK_HEADER_VERSION", &v1_1, None),
        mark(
            LegacyKind::Type,
            "VK_HEADER_VERSION_COMPLETE",
            &v1_1,
            Some("VK_HEADER_VERSION"),
        ),
        mark(LegacyKind::Type, "VK_API_VERSION_1_0", &v1_1, None),
        mark(
            LegacyKind::Type,
            "VK_API_VERSION_1_1",
            &v1_1,
            Some("VK_API_VERSION_1_0"),
        ),
        mark(LegacyKind::Command, "vkGetQueue", &v1_1, Some("VkQueue")),
        feature("queueCount"),
        feature("pPriorities"),
        feature("reserved"),
        feature("label"),
    ];
    ir
}

fn legacy_docs() -> Docs {
    let mut docs = docs();
    docs.anchors = [(
        "legacy-queues".to_string(),
        "appendices/legacy.adoc".to_string(),
    )]
    .into();
    docs
}

#[test]
fn types_and_their_aliases_state_that_the_registry_marks_them_as_legacy() {
    let (ir, docs) = (legacy_ir(), legacy_docs());
    let model = Model::with_docs(&ir, &docs);
    let structs = structs::emit(&model);
    assert_eq!(
        doc_of(&structs, "pub struct QueueInfo<'a>"),
        [
            "Structure describing queues.".to_string(),
            format!(
                "C structure [`VkQueueInfo`]({PAGES}/VkQueueInfo.html), provided by Vulkan 1.1 or [`VK_KHR_queue`](KHR_QUEUE_NAME)."
            ),
            legacy("[`VkDeviceInfo`](DeviceInfo)"),
            "Extends [`VkDeviceInfo`](DeviceInfo): their `push_next` accepts this structure."
                .to_string(),
            alias("VkQueueInfo"),
        ]
    );
    assert_eq!(
        doc_of(&structs, "pub struct DeviceInfo<'a>"),
        [
            "C structure `VkDeviceInfo`, provided by Vulkan 1.0.".to_string(),
            alias("VkDeviceInfo")
        ],
        "only extensions mark the structure, which is core"
    );
    assert_eq!(
        doc_of(&structs, "pub type DeviceInfoKHR<'a>"),
        [
            "Alias of [`DeviceInfo`], the C name `VkDeviceInfoKHR`.".to_string(),
            alias("VkDeviceInfoKHR"),
        ],
        "the specification shows no mark of a type alias"
    );
    assert_eq!(
        doc_of(&structs, "pub struct OtherInfoKHR<'a>"),
        [
            "C structure `VkOtherInfoKHR`, provided by Vulkan 1.0.".to_string(),
            legacy_as_of(
                "Vulkan 1.1, [`VK_KHR_queue`](KHR_QUEUE_NAME) and [`VK_KHR_other`](KHR_OTHER_NAME)"
            ),
            alias("VkOtherInfoKHR"),
        ]
    );
    assert_eq!(
        doc_of(&enums::emit(&model), "pub struct Color("),
        [
            "Colors of a queue.".to_string(),
            format!("C enumeration [`VkColor`]({PAGES}/VkColor.html)."),
            legacy_as_of("Vulkan 1.3"),
            alias("VkColor"),
        ]
    );
    let bitmasks = bitmasks::emit(&model);
    assert_eq!(
        doc_of(&bitmasks, "pub struct QueueFlagBits(")[..3],
        [
            "Bitmask specifying capabilities of queues.".to_string(),
            format!(
                "C enumeration of flag bits [`VkQueueFlagBits`]({PAGES}/VkQueueFlagBits.html)."
            ),
            legacy_as_of("Vulkan 1.1"),
        ]
    );
    assert_eq!(
        doc_of(&bitmasks, "pub struct QueueFlags(")[..3],
        [
            "Bitmask of VkQueueFlagBits.".to_string(),
            format!("C bitmask type [`VkQueueFlags`]({PAGES}/VkQueueFlags.html)."),
            legacy_as_of("Vulkan 1.0"),
        ]
    );
    let handles = handles::emit(&model);
    assert_eq!(
        doc_of(&handles, "pub struct Queue(")[..3],
        [
            "Opaque handle to a queue object.".to_string(),
            format!("C dispatchable handle [`VkQueue`]({PAGES}/VkQueue.html)."),
            legacy("[`VkDevice`](Device)"),
        ]
    );
    assert_eq!(
        doc_of(&handles, "pub struct Device(").len(),
        3,
        "only an extension marks the handle, which is core"
    );
}

#[test]
fn commands_state_that_the_registry_marks_them_as_legacy_before_their_sections() {
    let (ir, docs) = (legacy_ir(), legacy_docs());
    let out = commands::emit(&Model::with_docs(&ir, &docs));
    let raw = doc_of(&out, "pub unsafe fn get_queue_raw(");
    assert_eq!(
        raw[1..4],
        [
            format!(
                "Calls the C command [`vkGetQueue`]({PAGES}/vkGetQueue.html) with the arguments as they are. The command is provided by Vulkan 1.0."
            ),
            legacy("[`VkQueue`](Queue)"),
            "# Parameters".to_string(),
        ]
    );
    let wrapper = doc_of(&out, "pub unsafe fn get_queue(");
    assert_eq!(
        wrapper[1..4],
        [
            format!(
                "Calls the C command [`vkGetQueue`]({PAGES}/vkGetQueue.html) through [`Self::get_queue_raw`], which describes the parameters. Returns what the command writes to `pQueue`. The command is provided by Vulkan 1.0."
            ),
            legacy("[`VkQueue`](Queue)"),
            "# Errors".to_string(),
        ]
    );
    assert_eq!(out.matches("Legacy functionality").count(), 2);
}

#[test]
fn members_and_their_setters_state_that_the_registry_marks_them_as_legacy() {
    let (ir, docs) = (legacy_ir(), legacy_docs());
    let out = structs::emit(&Model::with_docs(&ir, &docs));
    let marked_by = legacy_as_of("Vulkan 1.2");
    assert_eq!(
        doc_of(&out, "pub queue_count:"),
        [
            "`queueCount` is the number of queues, at most [`VK_MAX_QUEUES`](MAX_QUEUES).",
            marked_by.as_str()
        ]
    );
    assert_eq!(
        doc_of(&out, "pub reserved:"),
        ["The C member `uint32_t reserved`.", marked_by.as_str()]
    );
    assert_eq!(
        doc_of(&out, "pub label:"),
        ["The C member `char label[8]`.", marked_by.as_str()]
    );
    assert_eq!(
        doc_of(&out, "pub fn queue_count("),
        [
            "Sets [`queue_count`](Self::queue_count).",
            marked_by.as_str()
        ]
    );
    assert_eq!(
        doc_of(&out, "pub fn label("),
        ["Sets [`label`](Self::label).", marked_by.as_str()]
    );
    assert_eq!(
        doc_of(&out, "pub fn priorities("),
        [
            "Sets [`p_priorities`](Self::p_priorities) to the start of a slice of elements and [`queue_count`](Self::queue_count) to its length.",
            "An empty slice sets [`p_priorities`](Self::p_priorities) to a null pointer.",
            marked_by.as_str(),
            "# Panics",
            "Panics if the slice has more elements than `queueCount`, a `uint32_t`, can count."
        ]
    );
    assert_eq!(
        doc_of(&out, "pub fn label_as_c_str(")[0],
        "Reads [`label`](Self::label) as a C string that ends at its first NUL."
    );
    assert_eq!(
        out.matches("Legacy functionality").count(),
        10,
        "two structures, and the member and the setter of four features"
    );
}

#[test]
fn version_macros_state_that_the_registry_marks_them_as_legacy() {
    let (ir, docs) = (legacy_ir(), legacy_docs());
    let out = version::emit(&Model::with_docs(&ir, &docs));
    assert_eq!(
        doc_of(&out, "pub const HEADER_VERSION:"),
        [
            "C macro `VK_HEADER_VERSION`.".to_string(),
            legacy_as_of("Vulkan 1.1"),
            alias("VK_HEADER_VERSION"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const HEADER_VERSION_COMPLETE:"),
        [
            "C macro `VK_HEADER_VERSION_COMPLETE`.".to_string(),
            legacy("[`VK_HEADER_VERSION`](HEADER_VERSION)"),
            alias("VK_HEADER_VERSION_COMPLETE"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const API_VERSION_1_0:"),
        [
            "C macro `VK_API_VERSION_1_0`.".to_string(),
            legacy_as_of("Vulkan 1.1"),
            alias("VK_API_VERSION_1_0"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const API_VERSION_1_1:"),
        [
            "C macro `VK_API_VERSION_1_1`.".to_string(),
            legacy("[`VK_API_VERSION_1_0`](API_VERSION_1_0)"),
            alias("VK_API_VERSION_1_1"),
        ]
    );
    crate::docs::assert_every_legacy_mark_has_an_item(&Model::with_docs(&ir, &docs));
}

#[test]
fn documentation_without_legacy_marks_has_no_legacy_statement() {
    let (ir, docs) = (ir(), legacy_docs());
    let model = Model::with_docs(&ir, &docs);
    for emit in [
        bitmasks::emit,
        commands::emit,
        constants::emit,
        enums::emit,
        extensions::emit,
        handles::emit,
        structs::emit,
        version::emit,
    ] {
        assert!(!emit(&model).contains("Legacy"));
    }
}

const NO_REASON: &str = "Legacy functionality, for which the registry gives no reason.";
const NOT_USED: &str = "Legacy functionality that is not used.";

fn legacy_attribute(
    element: LegacyElement,
    owner: &str,
    name: &str,
    reason: LegacyReason,
    alias: Option<&str>,
) -> LegacyAttribute {
    LegacyAttribute {
        element,
        name: name.to_string(),
        owner: Some(owner.to_string()),
        reason,
        alias: alias.map(str::to_string),
    }
}

#[test]
fn values_say_that_the_registry_calls_them_legacy() {
    let mut ir = ir();
    let color = ir.enums.iter_mut().find(|e| e.name == "VkColor");
    color.expect("the enumeration").fields[0].aliases = vec![
        "VK_COLOR_CRIMSON".to_string(),
        "VK_COLOR_SCARLET".to_string(),
    ];
    ir.bitmasks[0].fields[0].aliases = vec![
        "VK_QUEUE_DRAWING_BIT".to_string(),
        "VK_QUEUE_GRAPHICS".to_string(),
        "VK_QUEUE_PAINTING_BIT".to_string(),
    ];
    let value = |owner: &str, name: &str, reason, alias| {
        legacy_attribute(LegacyElement::Enum, owner, name, reason, alias)
    };
    ir.legacy_attributes = vec![
        value(
            "VkColor",
            "VK_COLOR_SCARLET",
            LegacyReason::Aliased,
            Some("VK_COLOR_CRIMSON"),
        ),
        value("VkColor", "VK_COLOR_RED", LegacyReason::True, None),
        value("VkColor", "VK_COLOR_BLUE", LegacyReason::True, None),
        value("VkColor", "VK_COLOR_GREEN", LegacyReason::True, None),
        value(
            "VkQueueFlagBits",
            "VK_QUEUE_DRAWING_BIT",
            LegacyReason::Aliased,
            Some("VK_QUEUE_GRAPHICS_BIT"),
        ),
        value(
            "VkQueueFlagBits",
            "VK_QUEUE_GRAPHICS",
            LegacyReason::Aliased,
            Some("VK_QUEUE_GRAPHICS_BIT"),
        ),
    ];
    let docs = docs();
    let model = Model::with_docs(&ir, &docs);
    crate::docs::assert_every_legacy_attribute_is_stated(&model);
    let out = enums::emit(&model);
    assert_eq!(
        doc_of(&out, "pub const SCARLET:"),
        [
            "`VK_COLOR_SCARLET` is a legacy alias of [`VK_COLOR_RED`](Self::RED).".to_string(),
            alias("VK_COLOR_SCARLET"),
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
        doc_of(&out, "pub const RED:"),
        [
            "`VK_COLOR_RED` specifies red, not [`VK_COLOR_BLUE`](Color::BLUE).".to_string(),
            NO_REASON.to_string(),
            alias("VK_COLOR_RED"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const BLUE:"),
        [
            format!("`VK_COLOR_BLUE`, a value of [`VkColor`]({PAGES}/VkColor.html)."),
            NO_REASON.to_string(),
            alias("VK_COLOR_BLUE"),
        ]
    );
    assert_eq!(
        doc_of(&out, "pub const GREEN:")[2..],
        [NO_REASON.to_string(), alias("VK_COLOR_GREEN")]
    );
    assert_eq!(out.matches("Legacy functionality").count(), 3);
    assert_eq!(out.matches("legacy alias").count(), 1);

    let out = bitmasks::emit(&model);
    let (bits, flags) = out
        .split_once("pub struct QueueFlags(")
        .expect("the flags type after the bits");
    for constants in [bits, flags] {
        assert_eq!(
            doc_of(constants, "pub const DRAWING:"),
            [
                "`VK_QUEUE_DRAWING_BIT` is a legacy alias of [`VK_QUEUE_GRAPHICS_BIT`](Self::GRAPHICS)."
                    .to_string(),
                alias("VK_QUEUE_DRAWING_BIT"),
            ]
        );
        assert_eq!(
            doc_of(constants, "pub const PAINTING:"),
            [
                "`VK_QUEUE_PAINTING_BIT` is an alias of [`VK_QUEUE_GRAPHICS_BIT`](Self::GRAPHICS)."
                    .to_string(),
                alias("VK_QUEUE_PAINTING_BIT"),
            ]
        );
        assert_eq!(
            doc_of(constants, "pub const GRAPHICS:"),
            [
                "`VK_QUEUE_GRAPHICS_BIT` specifies graphics.".to_string(),
                "The C header also calls this value `VK_QUEUE_GRAPHICS`, which is a legacy alias of `VK_QUEUE_GRAPHICS_BIT`."
                    .to_string(),
                alias("VK_QUEUE_GRAPHICS_BIT"),
                alias("VK_QUEUE_GRAPHICS"),
            ],
            "the legacy alias `VK_QUEUE_GRAPHICS` has the Rust name of the value, which is not legacy"
        );
    }
    assert_eq!(out.matches("legacy").count(), 4);
}

fn queue_flags_with_aliases(aliases: &[&str], legacy: &[&str]) -> Registry {
    let mut ir = ir();
    ir.bitmasks[0].fields[0].aliases = aliases.iter().map(|alias| alias.to_string()).collect();
    let legacy = legacy.iter().map(|name| {
        legacy_attribute(
            LegacyElement::Enum,
            "VkQueueFlagBits",
            name,
            LegacyReason::Aliased,
            Some("VK_QUEUE_GRAPHICS_BIT"),
        )
    });
    ir.legacy_attributes = legacy.collect();
    ir
}

#[test]
fn a_constant_that_a_legacy_alias_shares_is_documented_as_the_name_that_is_not_legacy() {
    let painting = "VK_QUEUE_PAINTING_BIT";
    let legacy = ["VK_QUEUE_PAINTING", "VK_PAINTING"];
    let graphics = "[`VK_QUEUE_GRAPHICS_BIT`](Self::GRAPHICS)";
    for aliases in [
        [painting, legacy[0], legacy[1]],
        [legacy[0], painting, legacy[1]],
        [legacy[1], legacy[0], painting],
    ] {
        let ir = queue_flags_with_aliases(&aliases, &legacy);
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        let out = bitmasks::emit(&model);
        let (bits, flags) = out
            .split_once("pub struct QueueFlags(")
            .expect("the flags type after the bits");
        for constants in [bits, flags] {
            assert_eq!(
                doc_of(constants, "pub const PAINTING:"),
                [
                    format!("`VK_QUEUE_PAINTING_BIT` is an alias of {graphics}."),
                    format!(
                        "The C header also calls this value `VK_PAINTING` and `VK_QUEUE_PAINTING`, which are legacy aliases of {graphics}."
                    ),
                    alias("VK_QUEUE_PAINTING_BIT"),
                    alias("VK_PAINTING"),
                    alias("VK_QUEUE_PAINTING"),
                ],
                "{aliases:?}"
            );
            assert_eq!(constants.matches("pub const PAINTING:").count(), 1);
            assert_eq!(
                constants.matches("(Self(1), \"PAINTING\"),").count(),
                1,
                "{aliases:?}: the constant is one of the other names of its value"
            );
            assert_eq!(constants.matches("(Self(1), \"GRAPHICS\"),").count(), 1);
        }
        assert_eq!(out.matches("legacy").count(), 2, "{aliases:?}");
    }
}

#[test]
fn a_constant_that_only_legacy_aliases_share_is_documented_as_the_first_of_them() {
    let graphics = "[`VK_QUEUE_GRAPHICS_BIT`](Self::GRAPHICS)";
    for (aliases, own, other) in [
        (
            ["VK_QUEUE_PAINTING", "VK_PAINTING"],
            "VK_QUEUE_PAINTING",
            "VK_PAINTING",
        ),
        (
            ["VK_PAINTING", "VK_QUEUE_PAINTING"],
            "VK_PAINTING",
            "VK_QUEUE_PAINTING",
        ),
    ] {
        let ir = queue_flags_with_aliases(&aliases, &aliases);
        let docs = docs();
        let model = Model::with_docs(&ir, &docs);
        let out = bitmasks::emit(&model);
        assert_eq!(
            doc_of(&out, "pub const PAINTING:"),
            [
                format!("`{own}` is a legacy alias of {graphics}."),
                format!(
                    "The C header also calls this value `{other}`, which is a legacy alias of {graphics}."
                ),
                alias(own),
                alias(other),
            ]
        );
    }
}

#[test]
fn two_names_of_a_value_that_are_not_legacy_and_share_a_constant_abort_generation() {
    let ir = queue_flags_with_aliases(&["VK_QUEUE_PAINTING_BIT", "VK_PAINTING"], &[]);
    let docs = docs();
    assert_eq!(
        catch_quietly(|| bitmasks::emit(&Model::with_docs(&ir, &docs))).map(|_| ()),
        Err("VkQueueFlagBits: `VK_QUEUE_PAINTING_BIT` and `VK_PAINTING` both map to `PAINTING`, and neither is a legacy alias".to_string())
    );
    let mut ir = queue_flags_with_aliases(&[], &[]);
    let mut other = ir.bitmasks[0].fields[0].clone();
    other.name = "VK_GRAPHICS".to_string();
    other.value = "2".to_string();
    ir.bitmasks[0].fields.push(other);
    assert_eq!(
        catch_quietly(|| bitmasks::emit(&Model::with_docs(&ir, &docs))).map(|_| ()),
        Err(
            "VkQueueFlagBits: `VK_GRAPHICS` maps to `GRAPHICS` which already has value 1, not 2"
                .to_string()
        )
    );
}

#[test]
fn a_name_that_shares_the_constant_of_another_value_aborts_generation() {
    let docs = docs();
    for legacy in [&["VK_QUEUE_PAINTING"][..], &[]] {
        let mut ir = queue_flags_with_aliases(&["VK_QUEUE_PAINTING"], legacy);
        let mut other = ir.bitmasks[0].fields[0].clone();
        other.name = "VK_QUEUE_PAINTING_BIT".to_string();
        other.aliases.clear();
        ir.bitmasks[0].fields.push(other);
        assert_eq!(
            catch_quietly(|| bitmasks::emit(&Model::with_docs(&ir, &docs))).map(|_| ()),
            Err("VkQueueFlagBits: `VK_QUEUE_PAINTING`, a name of `VK_QUEUE_GRAPHICS_BIT`, maps to `PAINTING`, which stands for `VK_QUEUE_PAINTING_BIT`".to_string()),
            "{legacy:?}"
        );
    }
}

#[test]
fn members_that_are_not_used_say_so_unless_their_description_does_and_their_setters_say_so() {
    let mut ir = ir();
    let member = |name: &str| {
        legacy_attribute(
            LegacyElement::Member,
            "VkQueueInfo",
            name,
            LegacyReason::Unused,
            None,
        )
    };
    ir.legacy_attributes = vec![
        member("queueCount"),
        member("pPriorities"),
        member("reserved"),
    ];
    let docs = docs();
    let model = Model::with_docs(&ir, &docs);
    crate::docs::assert_every_legacy_attribute_is_stated(&model);
    let out = structs::emit(&model);
    assert_eq!(
        doc_of(&out, "pub queue_count:"),
        [
            "`queueCount` is the number of queues, at most [`VK_MAX_QUEUES`](MAX_QUEUES).",
            NOT_USED
        ]
    );
    assert_eq!(
        doc_of(&out, "pub p_priorities:"),
        ["The C member `const float* pPriorities`.", NOT_USED]
    );
    assert_eq!(
        doc_of(&out, "pub reserved:"),
        ["The C member `uint32_t reserved`.", NOT_USED]
    );
    assert_eq!(
        doc_of(&out, "pub fn queue_count("),
        ["Sets [`queue_count`](Self::queue_count).", NOT_USED]
    );
    assert_eq!(
        doc_of(&out, "pub fn reserved("),
        ["Sets [`reserved`](Self::reserved).", NOT_USED]
    );
    let priorities = doc_of(&out, "pub fn priorities(");
    assert_eq!(
        priorities[0],
        "Sets [`p_priorities`](Self::p_priorities) to the start of a slice of elements and [`queue_count`](Self::queue_count) to its length."
    );
    assert_eq!(
        priorities
            .iter()
            .position(|block| block == NOT_USED)
            .zip(priorities.iter().position(|block| block == "# Panics")),
        Some((priorities.len() - 3, priorities.len() - 2)),
        "the last paragraph before the sections: {priorities:?}"
    );
    assert_eq!(out.matches(NOT_USED).count(), 6);

    let described = with_item(
        docs,
        "VkQueueInfo",
        "queueCount",
        &[
            "pname:queueCount is legacy and",
            "not used.",
            "Implementations must: not access it.",
        ],
    );
    let out = structs::emit(&Model::with_docs(&ir, &described));
    assert_eq!(
        doc_of(&out, "pub queue_count:"),
        ["`queueCount` is legacy and not used. Implementations must not access it."]
    );
    assert_eq!(
        doc_of(&out, "pub fn queue_count("),
        ["Sets [`queue_count`](Self::queue_count).", NOT_USED]
    );
    assert_eq!(out.matches(NOT_USED).count(), 5);

    ir.legacy_attributes.clear();
    let out = structs::emit(&Model::with_docs(&ir, &described));
    assert_eq!(out.matches("Legacy").count(), 0);
}
