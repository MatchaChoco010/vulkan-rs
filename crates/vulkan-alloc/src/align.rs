pub(crate) fn align_up(value: u64, alignment: u64) -> Option<u64> {
    if !alignment.is_power_of_two() {
        let rem = value % alignment;
        return if rem == 0 {
            Some(value)
        } else {
            value.checked_add(alignment - rem)
        };
    }
    value
        .checked_add(alignment - 1)
        .map(|x| x & !(alignment - 1))
}

pub(crate) fn align_down(value: u64, alignment: u64) -> u64 {
    let alignment = alignment.max(1);
    value - (value % alignment)
}
