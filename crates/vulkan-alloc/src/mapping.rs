use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::vk;

use crate::align::{align_down, align_up};
use crate::allocation::Allocation;
use crate::allocator::Allocator;
use crate::error::AllocationError;

#[must_use = "dropping the guard unmaps the allocation"]
pub struct MappedAllocation<'a> {
    allocator: &'a mut Allocator,
    allocation: &'a Allocation,
    ptr: NonNull<u8>,
}

impl<'a> MappedAllocation<'a> {
    pub(crate) fn new(
        allocator: &'a mut Allocator,
        allocation: &'a Allocation,
        ptr: NonNull<u8>,
    ) -> Self {
        Self {
            allocator,
            allocation,
            ptr,
        }
    }

    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr().cast()
    }

    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.allocation.size() as usize) }
    }

    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.allocation.size() as usize)
        }
    }

    pub fn byte_range(&self, offset: usize, size: usize) -> &[u8] {
        self.try_byte_range(offset, size)
            .expect("mapped byte range exceeds allocation size")
    }

    pub fn byte_range_mut(&mut self, offset: usize, size: usize) -> &mut [u8] {
        self.try_byte_range_mut(offset, size)
            .expect("mapped byte range exceeds allocation size")
    }

    pub fn try_byte_range(&self, offset: usize, size: usize) -> Option<&[u8]> {
        let end = offset.checked_add(size)?;
        unsafe { self.as_bytes() }.get(offset..end)
    }

    pub fn try_byte_range_mut(&mut self, offset: usize, size: usize) -> Option<&mut [u8]> {
        let end = offset.checked_add(size)?;
        unsafe { self.as_bytes_mut() }.get_mut(offset..end)
    }

    pub unsafe fn flush(&mut self) -> Result<(), AllocationError> {
        unsafe { self.allocator.flush_allocation(self.allocation) }
    }

    pub unsafe fn invalidate(&mut self) -> Result<(), AllocationError> {
        unsafe { self.allocator.invalidate_allocation(self.allocation) }
    }
}

impl Drop for MappedAllocation<'_> {
    fn drop(&mut self) {
        unsafe {
            let _ = self.allocator.unmap_memory(self.allocation);
        }
    }
}

pub(crate) fn mapped_range_bounds(
    offset: u64,
    size: u64,
    block_size: u64,
    atom_size: u64,
) -> Option<(u64, u64)> {
    let end = offset.checked_add(size)?;
    if end > block_size {
        return None;
    }
    let atom = atom_size.max(1);
    let aligned_offset = align_down(offset, atom);
    let aligned_end = align_up(end, atom).unwrap_or(block_size).min(block_size);
    let range_size = if aligned_end == block_size {
        vk::WHOLE_SIZE
    } else {
        aligned_end.checked_sub(aligned_offset)?
    };
    Some((aligned_offset, range_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapped_ranges_are_limited_to_the_allocation_and_atom_aligned() {
        let (offset, size) = mapped_range_bounds(300, 100, 1024, 128).unwrap();
        assert_eq!(offset, 256);
        assert_eq!(size, 256);

        let (offset, size) = mapped_range_bounds(1000, 10, 1024, 128).unwrap();
        assert_eq!(offset, 896);
        assert_eq!(size, vk::WHOLE_SIZE);

        assert!(mapped_range_bounds(900, 200, 1024, 128).is_none());
    }
}
