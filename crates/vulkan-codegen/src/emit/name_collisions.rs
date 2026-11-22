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
    crate::emit::commands::emit(&Model::new(&ir));
}

#[test]
#[should_panic(
    expected = "the CommandInfo constants: the C command `vkFooBar` and the C command `vkFoo_Bar` both become the Rust name `FOO_BAR`"
)]
fn commands_of_different_tables_with_the_same_constant_name_stop_the_generator() {
    let mut ir = base();
    let mut device_command = command("vkFooBar", vec![field("device", "VkDevice")]);
    device_command.device = true;
    ir.commands.push(device_command);
    ir.commands.push(command("vkFoo_Bar", Vec::new()));
    generate(&ir);
}

#[test]
#[should_panic(
    expected = "crate::Device: the raw method of the C command `vkFoo` and the wrapper of the C command `vkFooRaw` both become the Rust name `foo_raw`"
)]
fn a_wrapper_named_like_the_raw_method_of_another_command_stops_the_generator() {
    let mut ir = base();
    for name in ["vkFoo", "vkFooRaw"] {
        let mut c = command(name, vec![field("device", "VkDevice")]);
        c.device = true;
        ir.commands.push(c);
    }
    generate(&ir);
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `value` and the local `value` that receives the C parameter `pValue` both become the Rust name `value`"
)]
fn an_input_named_like_the_local_that_receives_the_returned_value_stops_the_generator() {
    generate(&with_command(vec![
        field("value", "uint32_t"),
        field("pValue", "uint32_t*"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `count` and the local `count` that receives the C parameter `pItemCount` both become the Rust name `count`"
)]
fn an_input_named_like_the_count_local_of_an_enumeration_stops_the_generator() {
    generate(&with_command(vec![
        field("count", "uint32_t"),
        field("pItemCount", "uint32_t*"),
        counted("pItems", "VkItem*", "pItemCount"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `values` and the local `values` that receives the C parameter `pItems` both become the Rust name `values`"
)]
fn an_input_named_like_the_values_local_of_an_enumeration_stops_the_generator() {
    generate(&with_command(vec![
        field("values", "uint32_t"),
        field("pItemCount", "uint32_t*"),
        counted("pItems", "VkItem*", "pItemCount"),
    ]));
}

#[test]
#[should_panic(
    expected = "vkCollide: the C parameter `values` and the local `values` that receives the C parameter `pItems` both become the Rust name `values`"
)]
fn an_input_named_like_the_values_local_of_a_filled_vector_stops_the_generator() {
    generate(&with_command(vec![
        field("values", "uint32_t"),
        field("itemCount", "uint32_t"),
        counted("pInfos", "const VkItem*", "itemCount"),
        counted("pItems", "VkItem*", "itemCount"),
    ]));
}

#[test]
fn inputs_named_like_locals_that_a_wrapper_does_not_declare_generate() {
    generate(&with_command(vec![field("value", "uint32_t")]));
    generate(&with_command(vec![
        field("values", "uint32_t"),
        field("count", "uint32_t"),
    ]));
    generate(&with_command(vec![
        field("value", "uint32_t"),
        field("pItemCount", "uint32_t*"),
        counted("pItems", "VkItem*", "pItemCount"),
    ]));
}

#[test]
#[should_panic(
    expected = "VkCollide: the methods of the C member `getFlags` and of the C member `flags` are both named `get_flags`"
)]
fn a_member_named_like_the_getter_of_a_bit_field_stops_the_generator() {
    let mut flags = field("flags", "uint32_t");
    flags.bit_field_width = Some(8);
    generate(&with_struct(structure(
        "VkCollide",
        vec![field("getFlags", "uint32_t"), flags],
    )));
}

#[test]
#[should_panic(
    expected = "VkCollide: the methods of the C member `pushNext` and of the `pNext` chain are both named `push_next`"
)]
fn a_member_named_like_push_next_stops_the_generator() {
    let mut ir = with_struct(crate::test_support::tagged(
        "VkCollide",
        "VK_STRUCTURE_TYPE_COLLIDE",
        vec![field("pushNext", "uint32_t")],
    ));
    ir.enums
        .iter_mut()
        .find(|e| e.name == "VkStructureType")
        .expect("VkStructureType")
        .fields
        .push(vulkan_registry::EnumField {
            name: "VK_STRUCTURE_TYPE_COLLIDE".to_string(),
            value: "1".to_string(),
            ..Default::default()
        });
    generate(&ir);
}

#[test]
#[should_panic(
    expected = "VkCollide: the methods of the C member `pInfo` and of the C member `infoPtrs` are both named `info_ptrs`"
)]
fn members_whose_setters_share_a_name_are_both_named_when_generation_stops() {
    generate(&with_struct(structure(
        "VkCollide",
        vec![
            field("info", "uint32_t"),
            field("pInfo", "const VkItem*"),
            field("infoPtrs", "uint32_t"),
        ],
    )));
}

#[derive(Default)]
struct LetBindings(Vec<String>);

impl<'ast> syn::visit::Visit<'ast> for LetBindings {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        let mut pattern = &local.pat;
        if let syn::Pat::Type(typed) = pattern {
            pattern = &typed.pat;
        }
        if let syn::Pat::Ident(ident) = pattern {
            self.0.push(ident.ident.to_string());
        }
        syn::visit::visit_local(self, local);
    }
}

#[test]
fn no_wrapper_of_the_registry_binds_a_local_with_the_name_of_one_of_its_parameters() {
    let source = crate::emit::commands::emit(&Model::with_docs(Registry::get(), Docs::get()));
    let file = syn::parse_file(&source).expect("the generated commands parse");
    let mut functions = 0;
    for item in &file.items {
        let syn::Item::Impl(block) = item else {
            continue;
        };
        for member in &block.items {
            let syn::ImplItem::Fn(function) = member else {
                continue;
            };
            functions += 1;
            let parameters = function
                .sig
                .inputs
                .iter()
                .filter_map(|input| match input {
                    syn::FnArg::Typed(typed) => match &*typed.pat {
                        syn::Pat::Ident(ident) => Some(ident.ident.to_string()),
                        _ => None,
                    },
                    syn::FnArg::Receiver(_) => None,
                })
                .collect::<Vec<_>>();
            let mut bindings = LetBindings::default();
            syn::visit::Visit::visit_block(&mut bindings, &function.block);
            for binding in bindings.0 {
                assert!(
                    !parameters.contains(&binding),
                    "{}: the local `{binding}` hides the parameter of the same name",
                    function.sig.ident
                );
            }
        }
    }
    assert!(functions > 1000, "only {functions} methods were checked");
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
