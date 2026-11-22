use crate::model::{CType, Dispatch, Model};
use crate::naming::{command_name, field_name, pointer_setter_name};
use crate::test_support::{command, counted, field, registry, structure};
use std::collections::BTreeMap;
use vulkan_registry::{Docs, Enum, Field, FuncPointer, FuncPointerParam, Handle, Registry, Struct};

fn base() -> Registry {
    let mut ir = registry();
    ir.enums.push(Enum {
        name: "VkFormat".to_string(),
        bit_width: 32,
        ..Default::default()
    });
    ir.handles.push(Handle {
        name: "VkDevice".to_string(),
        object_type: "VK_OBJECT_TYPE_DEVICE".to_string(),
        dispatchable: true,
        ..Default::default()
    });
    ir.structs
        .push(structure("VkItem", vec![field("value", "uint32_t")]));
    ir
}

fn with_command(params: Vec<Field>) -> Registry {
    let mut ir = base();
    let mut c = command(
        "vkCollide",
        std::iter::once(field("device", "VkDevice"))
            .chain(params)
            .collect(),
    );
    c.device = true;
    ir.commands.push(c);
    ir
}

fn with_struct(s: Struct) -> Registry {
    let mut ir = base();
    ir.structs.push(s);
    ir
}

fn generate(ir: &Registry) {
    crate::generate(ir, &Docs::default());
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `info` and the C parameter `pInfo` both become the Rust name `info`"
)]
fn a_value_and_a_pointer_parameter_with_the_same_short_name_stop_the_generator() {
    generate(&with_command(vec![
        field("info", "uint32_t"),
        field("pInfo", "const VkItem*"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `type` and the C parameter `ty` both become the Rust name `ty`"
)]
fn a_parameter_that_is_renamed_from_a_keyword_and_one_with_that_name_stop_the_generator() {
    generate(&with_command(vec![
        field("type", "uint32_t"),
        field("ty", "uint32_t"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `deviceMask` and the C parameter `DeviceMask` both become the Rust name `device_mask`"
)]
fn parameters_that_differ_only_in_case_stop_the_generator() {
    generate(&with_command(vec![
        field("deviceMask", "uint32_t"),
        field("DeviceMask", "uint32_t"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the slice length that replaces the C parameter `itemCount` and the C parameter `itemCountLen` both become the Rust name `item_count_len`"
)]
fn a_parameter_named_like_the_length_of_a_slice_stops_the_generator() {
    generate(&with_command(vec![
        field("itemCount", "uint32_t"),
        counted("pItems", "const VkItem*", "itemCount"),
        field("itemCountLen", "uint32_t"),
    ]));
}

#[test]
#[should_panic(
    expected = "PFN_vkCollideCallback: the C parameter `type` and the C parameter `ty` both become the Rust name `ty`"
)]
fn function_pointer_parameters_with_the_same_rust_name_stop_the_generator() {
    let mut ir = base();
    ir.func_pointers.push(FuncPointer {
        name: "PFN_vkCollideCallback".to_string(),
        return_ty: "void".to_string(),
        params: ["type", "ty"]
            .iter()
            .map(|name| FuncPointerParam {
                name: name.to_string(),
                full_ty: "uint32_t".to_string(),
            })
            .collect(),
        ..Default::default()
    });
    generate(&ir);
}

#[test]
#[should_panic(
    expected = "crate::Device: the C command `vkFooBar` and the C command `vkFoo_Bar` both become the Rust name `foo_bar`"
)]
fn commands_of_one_table_with_the_same_rust_name_stop_the_generator() {
    let mut ir = base();
    for name in ["vkFooBar", "vkFoo_Bar"] {
        let mut c = command(name, vec![field("device", "VkDevice")]);
        c.device = true;
        ir.commands.push(c);
    }
    generate(&ir);
}

#[test]
#[should_panic(
    expected = "VkCollide: the C member `type` and the C member `ty` both become the Rust name `ty`"
)]
fn struct_members_with_the_same_rust_name_stop_the_generator() {
    generate(&with_struct(structure(
        "VkCollide",
        vec![field("type", "uint32_t"), field("ty", "uint32_t")],
    )));
}

#[test]
#[should_panic(
    expected = "VkCollide: the C member `fooBar` and the C member `foo_bar` both become the Rust name `foo_bar`"
)]
fn union_members_with_the_same_rust_name_stop_the_generator() {
    generate(&with_struct(Struct {
        union: true,
        ..structure(
            "VkCollide",
            vec![field("fooBar", "uint32_t"), field("foo_bar", "uint32_t")],
        )
    }));
}

#[test]
#[should_panic(
    expected = "VkCollide: the C member `_marker` and the lifetime marker both become the Rust name `_marker`"
)]
fn a_member_named_like_the_lifetime_marker_stops_the_generator() {
    generate(&with_struct(structure(
        "VkCollide",
        vec![
            field("_marker", "uint32_t"),
            field("pItem", "const VkItem*"),
        ],
    )));
}

#[test]
fn parameters_and_members_with_distinct_rust_names_generate() {
    let pairs = [
        (field("type", "uint32_t"), field("kind", "uint32_t")),
        (field("info", "uint32_t"), field("pInfos", "const VkItem*")),
        (field("fooBar", "uint32_t"), field("fooBars", "uint32_t")),
        (
            field("deviceMask", "uint32_t"),
            field("pDeviceMasks", "const uint32_t*"),
        ),
        (field("self", "uint32_t"), field("value", "uint32_t")),
    ];
    for (a, b) in pairs {
        let mut ir = with_command(vec![a.clone(), b.clone()]);
        ir.structs
            .push(structure("VkDistinct", vec![a.clone(), b.clone()]));
        ir.func_pointers.push(FuncPointer {
            name: "PFN_vkDistinctCallback".to_string(),
            return_ty: "void".to_string(),
            params: [&a, &b]
                .iter()
                .map(|p| FuncPointerParam {
                    name: p.name.clone(),
                    full_ty: p.full_ty.clone(),
                })
                .collect(),
            ..Default::default()
        });
        generate(&ir);
    }
}

fn assert_distinct<'a>(owner: &str, names: impl IntoIterator<Item = (&'a str, String)>) {
    let mut seen = BTreeMap::new();
    for (c_name, rust_name) in names {
        let rust_name = rust_name.trim_start_matches("r#").to_string();
        if let Some(earlier) = seen.insert(rust_name.clone(), c_name) {
            panic!("{owner}: `{earlier}` and `{c_name}` both become `{rust_name}`");
        }
    }
}

fn wrapper_argument_name(p: &Field) -> String {
    if CType::of(p).depth() > 0 {
        pointer_setter_name(&p.name)
    } else {
        field_name(&p.name)
    }
}

#[test]
fn no_two_names_of_one_registry_item_collide() {
    let ir = Registry::get();
    let model = Model::new(ir);
    for c in &ir.commands {
        let params = || c.params.iter().map(|p| p.name.as_str());
        assert_distinct(&c.name, params().map(|name| (name, field_name(name))));
        let lengths = c
            .params
            .iter()
            .filter(|p| c.params.iter().any(|q| q.len.as_deref() == Some(&p.name)))
            .map(|p| (p.name.as_str(), format!("{}_len", wrapper_argument_name(p))));
        assert_distinct(
            &c.name,
            c.params
                .iter()
                .map(|p| (p.name.as_str(), wrapper_argument_name(p)))
                .chain(lengths),
        );
    }
    for fp in &ir.func_pointers {
        assert_distinct(
            &fp.name,
            fp.params
                .iter()
                .map(|p| (p.name.as_str(), field_name(&p.name))),
        );
    }
    for s in &ir.structs {
        assert_distinct(
            &s.name,
            s.members
                .iter()
                .map(|m| (m.name.as_str(), field_name(&m.name)))
                .chain([("the lifetime marker", "_marker".to_string())]),
        );
    }
    for dispatch in Dispatch::ALL {
        assert_distinct(
            &dispatch.wrapper(),
            ir.commands
                .iter()
                .filter(|c| model.dispatch(c) == dispatch)
                .map(|c| (c.name.as_str(), command_name(&c.name))),
        );
    }
}
