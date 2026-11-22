use std::collections::BTreeSet;
use vulkan::vk;

fn words_of(name: &str) -> Option<(u32, u32)> {
    name.split('_').find_map(|part| {
        let (count, bits) = part.split_once("PACK")?;
        let count = if count.is_empty() {
            1
        } else {
            count.parse::<u32>().ok()?
        };
        Some((count, bits.parse::<u32>().ok()?))
    })
}

fn fields_of(name: &str) -> Vec<(char, u32)> {
    let mut fields = Vec::new();
    for part in name.split('_') {
        let mut tokens = Vec::new();
        let mut rest = part;
        while let Some(letter) = rest.chars().next().filter(char::is_ascii_uppercase) {
            let digits = rest[1..]
                .find(|c: char| !c.is_ascii_digit())
                .map_or(rest.len(), |end| end + 1);
            let Ok(bits) = rest[1..digits].parse::<u32>() else {
                break;
            };
            tokens.push((letter, bits));
            rest = &rest[digits..];
        }
        if rest.is_empty() {
            fields.extend(tokens);
        }
    }
    fields
}

fn is_a_component(letter: char) -> bool {
    !matches!(letter, 'X' | 'E')
}

#[test]
fn the_bits_of_a_packed_format_that_are_no_component_are_its_padding_and_shared_exponent() {
    let mut packed = 0;
    let mut with_other_bits = 0;
    for &(format, name) in vk::Format::variants() {
        let Some(info) = format.info() else { continue };
        let Some(word_bits) = info.packed else {
            continue;
        };
        packed += 1;
        let (count, bits) = words_of(name).unwrap_or_else(|| panic!("{name}: no PACK suffix"));
        assert_eq!(word_bits, bits, "{name}");
        let component_bits = info
            .components
            .iter()
            .map(|component| {
                component
                    .bits
                    .unwrap_or_else(|| panic!("{name}: a component without a width"))
            })
            .sum::<u32>();
        let other_bits_of_the_name = fields_of(name)
            .iter()
            .filter(|(letter, _)| !is_a_component(*letter))
            .map(|(_, bits)| bits)
            .sum::<u32>();
        assert!(
            component_bits <= count * word_bits,
            "{name}: {component_bits} bits of components in {count} words of {word_bits} bits"
        );
        assert_eq!(
            count * word_bits - component_bits,
            other_bits_of_the_name,
            "{name}: the bits of the words that no component holds are the `X` and `E` fields of the name"
        );
        if other_bits_of_the_name > 0 {
            with_other_bits += 1;
        }
    }
    assert!(packed >= 30, "only {packed} packed formats");
    assert!(
        with_other_bits >= 2,
        "only {with_other_bits} packed formats with bits that are no component"
    );
}

fn positions(fields: &[(char, u32)], word_bits: u32) -> Vec<(char, u32, u32)> {
    let mut high = word_bits;
    let mut placed = Vec::new();
    for &(letter, bits) in fields {
        let low = high - bits;
        if is_a_component(letter) {
            placed.push((letter, low, high - 1));
        }
        high = low;
    }
    placed
}

#[test]
fn adding_up_component_widths_misplaces_the_components_behind_padding_or_an_exponent() {
    let mut misplaced = BTreeSet::new();
    for &(format, name) in vk::Format::variants() {
        let Some(info) = format.info() else { continue };
        let Some(word_bits) = info.packed else {
            continue;
        };
        if words_of(name).is_none_or(|(count, _)| count != 1) {
            continue;
        }
        let fields = fields_of(name);
        let components = fields
            .iter()
            .copied()
            .filter(|(letter, _)| is_a_component(*letter))
            .collect::<Vec<_>>();
        if positions(&fields, word_bits) != positions(&components, word_bits) {
            misplaced.insert(name);
        }
    }
    assert_eq!(
        misplaced,
        BTreeSet::from(["E5B9G9R9_UFLOAT_PACK32", "X8_D24_UNORM_PACK32"]),
        "the one-word packed formats in which bits that are no component come before a component"
    );

    let shared_exponent = vk::Format::E5B9G9R9_UFLOAT_PACK32.info().unwrap();
    assert_eq!(
        shared_exponent
            .components
            .iter()
            .map(|component| (component.name, component.bits))
            .collect::<Vec<_>>(),
        [
            (vk::ComponentName::B, Some(9)),
            (vk::ComponentName::G, Some(9)),
            (vk::ComponentName::R, Some(9)),
        ]
    );
    assert_eq!(
        positions(&fields_of("E5B9G9R9_UFLOAT_PACK32"), 32),
        [('B', 18, 26), ('G', 9, 17), ('R', 0, 8)],
        "the bits that the specification gives for the mantissas"
    );
}
