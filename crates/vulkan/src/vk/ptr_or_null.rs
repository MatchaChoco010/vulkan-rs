pub(crate) fn ptr_or_null<T>(slice: &[T]) -> *const T {
    if slice.is_empty() {
        core::ptr::null()
    } else {
        slice.as_ptr()
    }
}

pub(crate) fn mut_ptr_or_null<T>(slice: &mut [T]) -> *mut T {
    if slice.is_empty() {
        core::ptr::null_mut()
    } else {
        slice.as_mut_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::{mut_ptr_or_null, ptr_or_null};
    use alloc::vec::Vec;

    #[test]
    fn an_empty_slice_gives_a_null_pointer_wherever_it_comes_from() {
        let values = [1_u32, 2, 3];
        let nothing: Vec<u32> = Vec::new();
        for empty in [
            &[][..],
            &values[..0],
            &values[1..1],
            &values[3..],
            &nothing[..],
        ] {
            assert!(ptr_or_null(empty).is_null());
        }
        let mut values = [1_u32, 2, 3];
        let mut nothing: Vec<u32> = Vec::new();
        assert!(mut_ptr_or_null::<u32>(&mut []).is_null());
        assert!(mut_ptr_or_null(&mut values[..0]).is_null());
        assert!(mut_ptr_or_null(&mut values[3..]).is_null());
        assert!(mut_ptr_or_null(&mut nothing[..]).is_null());
    }

    #[test]
    fn a_slice_with_elements_gives_the_pointer_to_its_first_element() {
        let mut values = [1_u32, 2, 3];
        for start in 0..values.len() {
            for end in start + 1..=values.len() {
                let first = core::ptr::from_ref(&values[start]);
                assert_eq!(ptr_or_null(&values[start..end]), first);
                assert_eq!(mut_ptr_or_null(&mut values[start..end]).cast_const(), first);
            }
        }
        let pointer = mut_ptr_or_null(&mut values[1..]);
        unsafe { pointer.write(7) };
        assert_eq!(values, [1, 7, 3]);
    }

    #[test]
    fn a_slice_of_values_without_a_size_is_null_only_when_it_has_no_elements() {
        let units = [(); 2];
        assert!(ptr_or_null(&units[..0]).is_null());
        assert_eq!(ptr_or_null(&units), units.as_ptr());
        assert!(!ptr_or_null(&units).is_null());
    }
}
