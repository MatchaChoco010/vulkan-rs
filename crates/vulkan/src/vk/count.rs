#[track_caller]
pub(crate) fn len_as_count<T: TryFrom<usize>>(len: usize, owner: &str, count: &str) -> T {
    match T::try_from(len) {
        Ok(count) => count,
        Err(_) => panic!(
            "{owner}: the slice has {len} elements, more than `{count}`, a {}, can count",
            core::any::type_name::<T>()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::len_as_count;

    fn count<T: TryFrom<usize>>(len: usize) -> T {
        len_as_count(len, "VkCountsInfo::pValues", "valueCount")
    }

    #[test]
    fn lengths_up_to_u32_max_become_u32_counts_unchanged() {
        let max = usize::try_from(u32::MAX).unwrap();
        for len in [0, 1, max - 1, max] {
            assert_eq!(usize::try_from(count::<u32>(len)).unwrap(), len);
        }
    }

    #[cfg(target_pointer_width = "64")]
    #[test]
    #[should_panic(
        expected = "VkCountsInfo::pValues: the slice has 4294967296 elements, more than `valueCount`, a u32, can count"
    )]
    fn a_length_past_u32_max_panics_instead_of_becoming_a_u32_count() {
        count::<u32>(usize::try_from(u32::MAX).unwrap() + 1);
    }

    #[test]
    fn lengths_up_to_u8_max_become_u8_counts_unchanged() {
        for len in 0..=255 {
            assert_eq!(usize::from(count::<u8>(len)), len);
        }
    }

    #[test]
    #[should_panic(
        expected = "VkCountsInfo::pValues: the slice has 256 elements, more than `valueCount`, a u8, can count"
    )]
    fn a_length_past_u8_max_panics_instead_of_becoming_a_u8_count() {
        count::<u8>(256);
    }

    #[test]
    fn every_length_becomes_a_size_count_unchanged() {
        for len in [0, usize::try_from(u32::MAX).unwrap(), usize::MAX] {
            assert_eq!(count::<usize>(len), len);
        }
    }
}
