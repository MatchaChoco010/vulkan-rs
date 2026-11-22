use super::StructureType;
use core::ffi::c_void;

pub unsafe trait Extends<Root> {}

pub unsafe trait TaggedStructure {
    const STRUCTURE_TYPE: StructureType;
}

#[repr(C)]
struct ChainHeader {
    s_type: StructureType,
    p_next: *mut c_void,
}

pub(crate) fn push_chain<T: TaggedStructure>(next: &mut T, head: *const c_void) -> *mut c_void {
    let next = core::ptr::from_mut(next).cast::<ChainHeader>();
    unsafe { (*next).p_next = head.cast_mut() };
    next.cast()
}
