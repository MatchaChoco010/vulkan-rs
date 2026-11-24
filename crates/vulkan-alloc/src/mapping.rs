use std::ffi::c_void;
use std::ptr::NonNull;

use vulkan::vk;

use crate::align::{align_down, align_up};
use crate::allocation::Allocation;
use crate::allocator::Allocator;
use crate::error::AllocationError;

/// A mapped allocation, returned by [`Allocator::map`].
///
/// The guard holds one mapping reference of the allocation and releases it when dropped,
/// unmapping the block if no other reference remains. It borrows the allocator mutably,
/// so nothing can be allocated, freed or mapped until it is dropped. Every byte accessor
/// covers the [`Allocation::size`] bytes of the allocation.
#[must_use = "dropping the guard unmaps the allocation"]
pub struct MappedAllocation<'a> {
    allocator: &'a mut Allocator,
    allocation: &'a Allocation,
    ptr: NonNull<u8>,
    len: usize,
}

impl<'a> MappedAllocation<'a> {
    pub(crate) fn new(
        allocator: &'a mut Allocator,
        allocation: &'a Allocation,
        ptr: NonNull<u8>,
        len: usize,
    ) -> Self {
        Self {
            allocator,
            allocation,
            ptr,
            len,
        }
    }

    /// A pointer to the first byte of the allocation. It is valid while the guard is
    /// alive.
    ///
    /// Accesses through it fall under the safety conditions of [`Allocator::map`]: it
    /// must not write the bytes of a slice from [`MappedAllocation::as_bytes`] or
    /// [`MappedAllocation::byte_range`] while that slice is in use, and must not access
    /// the bytes of a slice from [`MappedAllocation::as_bytes_mut`] or
    /// [`MappedAllocation::byte_range_mut`] while that slice is in use.
    pub fn as_ptr(&self) -> *mut c_void {
        self.ptr.as_ptr().cast()
    }

    /// All bytes of the allocation.
    ///
    /// # Safety
    ///
    /// While the slice is in use, neither the device nor any other host pointer may
    /// write these bytes.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// All bytes of the allocation, writable.
    ///
    /// # Safety
    ///
    /// While the slice is in use, neither the device nor any other host pointer may
    /// read or write these bytes.
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }

    /// The bytes `offset..offset + size` of the allocation, or `None` if the range does
    /// not lie inside the [`Allocation::size`] bytes of the allocation.
    ///
    /// The safety conditions of [`Allocator::map`] cover the returned slice.
    pub fn byte_range(&self, offset: usize, size: usize) -> Option<&[u8]> {
        let end = offset.checked_add(size)?;
        unsafe { self.as_bytes() }.get(offset..end)
    }

    /// The bytes `offset..offset + size` of the allocation, writable, or `None` if the
    /// range does not lie inside the [`Allocation::size`] bytes of the allocation.
    ///
    /// The safety conditions of [`Allocator::map`] cover the returned slice.
    pub fn byte_range_mut(&mut self, offset: usize, size: usize) -> Option<&mut [u8]> {
        let end = offset.checked_add(size)?;
        unsafe { self.as_bytes_mut() }.get_mut(offset..end)
    }

    /// Flushes the host writes to the allocation, as [`Allocator::flush_allocation`]
    /// does. Only memory types without `HOST_COHERENT` need it. Memory of
    /// [`MemoryLocation::CpuToGpu`](crate::MemoryLocation::CpuToGpu) and
    /// [`MemoryLocation::GpuToCpu`](crate::MemoryLocation::GpuToCpu) always has `HOST_COHERENT`;
    /// [`MemoryLocation::GpuOnly`](crate::MemoryLocation::GpuOnly) and a
    /// [`MemoryLocation::Custom`](crate::MemoryLocation::Custom) location that does not require
    /// it can give memory without it.
    ///
    /// For a memory type with `HOST_COHERENT`, this makes no Vulkan call and returns
    /// `Ok(())`, so it can be called after every host write without looking at the memory
    /// type.
    ///
    /// # Errors
    ///
    /// [`AllocationError::Vulkan`] if `vkFlushMappedMemoryRanges` fails.
    ///
    /// # Safety
    ///
    /// The device the allocator was created with must not have been destroyed.
    pub unsafe fn flush(&mut self) -> Result<(), AllocationError> {
        unsafe { self.allocator.flush_allocation(self.allocation) }
    }

    /// Invalidates the host's view of the allocation, as
    /// [`Allocator::invalidate_allocation`] does, so that device writes made available
    /// to the host become visible. Only memory types without `HOST_COHERENT` need it.
    /// Memory of [`MemoryLocation::CpuToGpu`](crate::MemoryLocation::CpuToGpu) and
    /// [`MemoryLocation::GpuToCpu`](crate::MemoryLocation::GpuToCpu) always has `HOST_COHERENT`;
    /// [`MemoryLocation::GpuOnly`](crate::MemoryLocation::GpuOnly) and a
    /// [`MemoryLocation::Custom`](crate::MemoryLocation::Custom) location that does not require
    /// it can give memory without it.
    ///
    /// For a memory type with `HOST_COHERENT`, this makes no Vulkan call and returns
    /// `Ok(())`, so it can be called before every host read without looking at the memory
    /// type.
    ///
    /// # Errors
    ///
    /// [`AllocationError::Vulkan`] if `vkInvalidateMappedMemoryRanges` fails.
    ///
    /// # Safety
    ///
    /// The device the allocator was created with must not have been destroyed.
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
