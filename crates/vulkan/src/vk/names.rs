use core::fmt;

pub(crate) fn debug_enum(
    f: &mut fmt::Formatter<'_>,
    type_name: &str,
    name: Option<&str>,
    value: i32,
) -> fmt::Result {
    match name {
        Some(name) => f.write_str(name),
        None => write!(f, "{type_name}({value})"),
    }
}

pub(crate) fn name_of<T: Copy + Ord>(
    variants: &[(T, &'static str)],
    value: T,
) -> Option<&'static str> {
    variants
        .binary_search_by(|(candidate, _)| candidate.cmp(&value))
        .ok()
        .map(|index| variants[index].1)
}

pub(crate) fn value_of<T: Copy>(
    variants: &[(T, &'static str)],
    aliases: &[(T, &'static str)],
    name: &str,
) -> Option<T> {
    variants
        .iter()
        .chain(aliases)
        .find(|(_, candidate)| *candidate == name)
        .map(|(value, _)| *value)
}

pub(crate) fn debug_flags(
    f: &mut fmt::Formatter<'_>,
    value: u64,
    names: impl Iterator<Item = (u64, &'static str)>,
) -> fmt::Result {
    if value == 0 {
        return f.write_str("empty()");
    }
    let mut rest = value;
    let mut first = true;
    for (bit, name) in names {
        if bit.count_ones() == 1 && rest & bit != 0 {
            if !first {
                f.write_str(" | ")?;
            }
            f.write_str(name)?;
            first = false;
            rest &= !bit;
        }
    }
    if rest != 0 {
        if !first {
            f.write_str(" | ")?;
        }
        write!(f, "{rest:#x}")?;
    }
    Ok(())
}
