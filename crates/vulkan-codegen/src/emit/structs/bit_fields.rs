use super::{MethodNames, member_origin};
use crate::docs::Doc;
use crate::model::{CType, Model};
use crate::naming::{field_name, type_name};
use vulkan_registry::{Field, Struct};

const STORAGE_UNIT_BITS: u32 = 32;

pub(crate) enum Slot<'a> {
    Plain(&'a Field),
    Bits { storage: String, bits: Vec<Bit<'a>> },
}

pub(crate) struct Bit<'a> {
    pub(crate) member: &'a Field,
    pub(crate) shift: u32,
    pub(crate) width: u32,
}

pub(crate) fn slots<'a>(model: &Model<'_>, s: &'a Struct) -> Vec<Slot<'a>> {
    let mut slots: Vec<Slot<'a>> = Vec::new();
    let mut storage_units = 0;
    for m in &s.members {
        let Some(width) = m.bit_field_width else {
            slots.push(Slot::Plain(m));
            continue;
        };
        assert_is_supported_bit_field(model, s, m, width);
        let used = match slots.last() {
            Some(Slot::Bits { bits, .. }) => bits.iter().map(|bit| bit.width).sum(),
            _ => STORAGE_UNIT_BITS,
        };
        if used + width > STORAGE_UNIT_BITS {
            slots.push(Slot::Bits {
                storage: format!("bitfield{storage_units}"),
                bits: Vec::new(),
            });
            storage_units += 1;
        }
        let Some(Slot::Bits { bits, .. }) = slots.last_mut() else {
            unreachable!("a storage unit was pushed above")
        };
        bits.push(Bit {
            member: m,
            shift: bits.iter().map(|bit| bit.width).sum(),
            width,
        });
    }
    slots
}

fn assert_is_supported_bit_field(model: &Model<'_>, s: &Struct, m: &Field, width: u32) {
    let base = model.canonical(&m.ty);
    assert!(
        base == "uint32_t" || model.is_flags(base),
        "{}::{}: bit-field of unsupported type {}",
        s.name,
        m.name,
        m.ty
    );
    assert!(
        (1..=STORAGE_UNIT_BITS).contains(&width) && CType::of(m).depth() == 0,
        "{}::{}: unsupported bit-field",
        s.name,
        m.name
    );
}

pub(super) fn accessors(
    model: &Model<'_>,
    storage: &str,
    bit: &Bit<'_>,
    names: &mut MethodNames<'_>,
) -> String {
    let origin = member_origin(bit.member);
    let setter = names.claim(&field_name(&bit.member.name), &origin);
    let getter = names.claim(&format!("get_{}", setter.trim_start_matches("r#")), &origin);
    let mask = u32::MAX >> (STORAGE_UNIT_BITS - bit.width);
    let (shl, shr) = match bit.shift {
        0 => (String::new(), String::new()),
        shift => (format!(" << {shift}"), format!(" >> {shift}")),
    };
    let raw = format!("(self.{storage}{shr}) & {mask:#x}");
    let (ty, value_as_raw, read) = if model.is_flags(&bit.member.ty) {
        let ty = type_name(model.canonical(&bit.member.ty));
        let read = format!("{ty}::from_raw({raw})");
        (ty, "value.as_raw()", read)
    } else {
        ("u32".to_string(), "value", raw)
    };
    let c_name = &bit.member.name;
    let width = match bit.width {
        1 => "1 bit".to_string(),
        width => format!("{width} bits"),
    };
    let setter_doc = Doc::new().text(format!(
        "Sets the C bit-field `{c_name}`, which is {width} wide. The higher bits of `value` are dropped."
    ));
    let getter_doc = Doc::new().text(format!("The value of the C bit-field `{c_name}`."));
    format!(
        "{}pub fn {setter}(mut self, value: {ty}) -> Self {{ self.{storage} = (self.{storage} & !({mask:#x}{shl})) | (({value_as_raw} & {mask:#x}){shl}); self }}
        {}pub fn {getter}(&self) -> {ty} {{ {read} }}\n",
        setter_doc.render(),
        getter_doc.render()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{field, registry, structure};

    #[test]
    fn bit_fields_that_do_not_fit_start_a_new_unit() {
        let mut ir = registry();
        let bits = |name: &str, width: u32| {
            let mut f = field(name, "uint32_t");
            f.bit_field_width = Some(width);
            f
        };
        ir.structs.push(structure(
            "StdVideoFlags",
            vec![
                bits("a", 20),
                bits("b", 20),
                bits("c", 12),
                field("plain", "uint32_t"),
                bits("d", 1),
            ],
        ));
        let model = Model::new(&ir);
        let slots = slots(&model, model.get_struct("StdVideoFlags").unwrap());
        let layout = slots
            .iter()
            .map(|slot| match slot {
                Slot::Bits { storage, bits } => (
                    storage.as_str(),
                    bits.iter().map(|b| (b.shift, b.width)).collect(),
                ),
                Slot::Plain(_) => ("", Vec::new()),
            })
            .collect::<Vec<(&str, Vec<_>)>>();
        assert_eq!(
            layout,
            vec![
                ("bitfield0", vec![(0, 20)]),
                ("bitfield1", vec![(0, 20), (20, 12)]),
                ("", vec![]),
                ("bitfield2", vec![(0, 1)]),
            ]
        );
    }
}
